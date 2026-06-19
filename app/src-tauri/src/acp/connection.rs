//! Async ACP connection: a JSON-RPC peer over line-delimited stdio.
//!
//! The connection is generic over any [`AsyncRead`]/[`AsyncWrite`] pair, so it
//! drives a real agent subprocess in production and a mock agent (via
//! `tokio::io::duplex`) in tests. Inbound messages are demultiplexed by a single
//! background read loop:
//!
//! * responses complete the matching pending request,
//! * `session/update` notifications are normalized and forwarded as [`UiEvent`]s,
//! * `session/request_permission` requests are surfaced as [`UiEvent::Permission`]
//!   with a token the UI later uses to resolve them,
//! * any other agent request is answered with `method not found` so the agent
//!   never blocks waiting on a capability we do not yet serve.

use std::collections::HashMap;
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::Arc;

use serde::Serialize;
use serde_json::Value;
use tokio::io::{AsyncBufReadExt, AsyncRead, AsyncWrite, AsyncWriteExt, BufReader};
use tokio::sync::{mpsc, oneshot, Mutex};

use super::protocol::{self, Inbound, PermissionOption, SessionEvent};

/// Errors surfaced by connection operations.
#[derive(Debug, thiserror::Error)]
pub enum AcpError {
    #[error("io error: {0}")]
    Io(String),
    #[error("agent error {code}: {message}")]
    Agent { code: i64, message: String },
    #[error("connection closed before a response arrived")]
    Closed,
    #[error("unexpected response shape: {0}")]
    BadResponse(String),
    #[error("unknown permission token: {0}")]
    UnknownToken(u64),
}

/// An event destined for the UI. Serialized and emitted over a Tauri channel.
#[derive(Debug, Clone, Serialize)]
#[serde(tag = "kind", rename_all = "snake_case")]
pub enum UiEvent {
    /// A normalized streaming update for a session.
    Session {
        session_id: String,
        event: SessionEvent,
    },
    /// The agent is asking the user to approve an action. Resolve via the token.
    Permission {
        session_id: String,
        token: u64,
        tool_title: Option<String>,
        options: Vec<PermissionOption>,
    },
    /// A non-fatal protocol problem worth surfacing.
    Notice { message: String },
    /// The agent's stdout closed; the session is over.
    Closed,
}

type Pending = Arc<Mutex<HashMap<u64, oneshot::Sender<Result<Value, protocol::JsonRpcError>>>>>;
type PermTokens = Arc<Mutex<HashMap<u64, Value>>>;

/// A live ACP connection to one agent subprocess.
pub struct AcpConnection {
    writer: Mutex<Box<dyn AsyncWrite + Unpin + Send>>,
    next_id: AtomicU64,
    next_token: AtomicU64,
    pending: Pending,
    perm_tokens: PermTokens,
}

impl AcpConnection {
    /// Build a connection from an async read/write pair and start the background
    /// read loop. Normalized [`UiEvent`]s flow out through `events`.
    pub fn start<R, W>(reader: R, writer: W, events: mpsc::UnboundedSender<UiEvent>) -> Arc<Self>
    where
        R: AsyncRead + Unpin + Send + 'static,
        W: AsyncWrite + Unpin + Send + 'static,
    {
        let conn = Arc::new(Self {
            writer: Mutex::new(Box::new(writer)),
            next_id: AtomicU64::new(1),
            next_token: AtomicU64::new(1),
            pending: Arc::new(Mutex::new(HashMap::new())),
            perm_tokens: Arc::new(Mutex::new(HashMap::new())),
        });
        let loop_conn = conn.clone();
        tokio::spawn(async move {
            loop_conn.read_loop(reader, events).await;
        });
        conn
    }

    async fn read_loop<R>(self: Arc<Self>, reader: R, events: mpsc::UnboundedSender<UiEvent>)
    where
        R: AsyncRead + Unpin + Send + 'static,
    {
        let mut lines = BufReader::new(reader).lines();
        loop {
            match lines.next_line().await {
                Ok(Some(line)) => {
                    let trimmed = line.trim();
                    if trimmed.is_empty() {
                        continue;
                    }
                    if let Err(msg) = self.dispatch(trimmed, &events).await {
                        let _ = events.send(UiEvent::Notice { message: msg });
                    }
                }
                Ok(None) => break, // EOF
                Err(e) => {
                    let _ = events.send(UiEvent::Notice {
                        message: format!("read error: {e}"),
                    });
                    break;
                }
            }
        }
        // Fail any still-pending requests so callers stop waiting.
        let mut pending = self.pending.lock().await;
        for (_, tx) in pending.drain() {
            let _ = tx.send(Err(protocol::JsonRpcError {
                code: 0,
                message: "connection closed".into(),
                data: None,
            }));
        }
        let _ = events.send(UiEvent::Closed);
    }

    /// Handle one inbound line. Returns Err(message) for a recoverable problem.
    async fn dispatch(
        &self,
        line: &str,
        events: &mpsc::UnboundedSender<UiEvent>,
    ) -> Result<(), String> {
        let value: Value =
            serde_json::from_str(line).map_err(|e| format!("invalid JSON from agent: {e}"))?;
        match protocol::classify(value)? {
            Inbound::Response { id, result } => {
                if let Some(tx) = self.pending.lock().await.remove(&id) {
                    let _ = tx.send(result);
                }
                Ok(())
            }
            Inbound::Notification { method, params } => {
                if method == "session/update" {
                    if let (Some(session_id), Some(update)) =
                        (protocol::session_id_of(&params), protocol::update_of(&params))
                    {
                        let event = protocol::normalize_update(update);
                        let _ = events.send(UiEvent::Session { session_id, event });
                    }
                }
                Ok(())
            }
            Inbound::Request { id, method, params } => {
                self.handle_agent_request(id, &method, params, events).await
            }
        }
    }

    async fn handle_agent_request(
        &self,
        id: Value,
        method: &str,
        params: Value,
        events: &mpsc::UnboundedSender<UiEvent>,
    ) -> Result<(), String> {
        if method == "session/request_permission" {
            let parsed: protocol::RequestPermissionParams = serde_json::from_value(params)
                .map_err(|e| format!("bad request_permission params: {e}"))?;
            let token = self.next_token.fetch_add(1, Ordering::Relaxed);
            self.perm_tokens.lock().await.insert(token, id);
            let _ = events.send(UiEvent::Permission {
                session_id: parsed.session_id,
                token,
                tool_title: parsed.tool_call.and_then(|t| t.title),
                options: parsed.options,
            });
            Ok(())
        } else {
            // We do not serve this capability yet; answer so the agent proceeds.
            let frame =
                protocol::error_response_frame(id, protocol::METHOD_NOT_FOUND, "method not found");
            self.write_value(&frame).await.map_err(|e| e.to_string())
        }
    }

    async fn write_value(&self, value: &Value) -> Result<(), AcpError> {
        let mut buf = serde_json::to_vec(value).map_err(|e| AcpError::Io(e.to_string()))?;
        buf.push(b'\n');
        let mut w = self.writer.lock().await;
        w.write_all(&buf).await.map_err(|e| AcpError::Io(e.to_string()))?;
        w.flush().await.map_err(|e| AcpError::Io(e.to_string()))
    }

    /// Send a request and await its result.
    pub async fn request(&self, method: &str, params: Value) -> Result<Value, AcpError> {
        let id = self.next_id.fetch_add(1, Ordering::Relaxed);
        let (tx, rx) = oneshot::channel();
        self.pending.lock().await.insert(id, tx);
        let frame = protocol::request_frame(id, method, params);
        if let Err(e) = self.write_value(&frame).await {
            self.pending.lock().await.remove(&id);
            return Err(e);
        }
        match rx.await {
            Ok(Ok(v)) => Ok(v),
            Ok(Err(e)) => Err(AcpError::Agent {
                code: e.code,
                message: e.message,
            }),
            Err(_) => Err(AcpError::Closed),
        }
    }

    // ----- High-level ACP operations -----

    /// Perform the ACP `initialize` handshake.
    pub async fn initialize(&self) -> Result<Value, AcpError> {
        self.request("initialize", protocol::initialize_params()).await
    }

    /// Create a new session rooted at `cwd`; returns the agent's session id.
    pub async fn new_session(&self, cwd: &str) -> Result<String, AcpError> {
        let result = self
            .request("session/new", protocol::new_session_params(cwd))
            .await?;
        result
            .get("sessionId")
            .and_then(|v| v.as_str())
            .map(|s| s.to_string())
            .ok_or_else(|| AcpError::BadResponse("session/new lacked sessionId".into()))
    }

    /// Send a prompt turn. Resolves when the turn ends (streaming arrives via
    /// [`UiEvent::Session`] in the meantime).
    pub async fn prompt(&self, session_id: &str, text: &str) -> Result<Value, AcpError> {
        self.request("session/prompt", protocol::prompt_params(session_id, text))
            .await
    }

    /// Resolve a pending permission request. `option_id == None` cancels it.
    pub async fn resolve_permission(
        &self,
        token: u64,
        option_id: Option<&str>,
    ) -> Result<(), AcpError> {
        let id = self
            .perm_tokens
            .lock()
            .await
            .remove(&token)
            .ok_or(AcpError::UnknownToken(token))?;
        let result = match option_id {
            Some(opt) => protocol::permission_selected_result(opt),
            None => protocol::permission_cancelled_result(),
        };
        let frame = protocol::response_frame(id, result);
        self.write_value(&frame).await
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader};

    /// Drive a full handshake + prompt + permission round-trip against an
    /// in-process mock agent connected over a duplex pipe. This exercises the
    /// real request/response demux, notification forwarding, and permission
    /// token plumbing without the actual Claude Code binary.
    #[tokio::test]
    async fn full_session_flow_against_mock_agent() {
        // client_rx/tx is the client side; agent_rx/tx is the mock agent side.
        let (client_io, agent_io) = tokio::io::duplex(8192);
        let (client_read, client_write) = tokio::io::split(client_io);
        let (agent_read, mut agent_write) = tokio::io::split(agent_io);

        let (ev_tx, mut ev_rx) = mpsc::unbounded_channel();
        let conn = AcpConnection::start(client_read, client_write, ev_tx);

        // Mock agent: read client requests line by line and reply.
        tokio::spawn(async move {
            let mut lines = BufReader::new(agent_read).lines();

            // 1. initialize
            let line = lines.next_line().await.unwrap().unwrap();
            let msg: Value = serde_json::from_str(&line).unwrap();
            assert_eq!(msg["method"], "initialize");
            let reply = serde_json::json!({
                "jsonrpc": "2.0", "id": msg["id"],
                "result": { "protocolVersion": 1 }
            });
            send(&mut agent_write, &reply).await;

            // 2. session/new
            let line = lines.next_line().await.unwrap().unwrap();
            let msg: Value = serde_json::from_str(&line).unwrap();
            assert_eq!(msg["method"], "session/new");
            let reply = serde_json::json!({
                "jsonrpc": "2.0", "id": msg["id"],
                "result": { "sessionId": "sess-42" }
            });
            send(&mut agent_write, &reply).await;

            // 3. session/prompt -> stream a chunk, request permission, then finish.
            let line = lines.next_line().await.unwrap().unwrap();
            let prompt_msg: Value = serde_json::from_str(&line).unwrap();
            assert_eq!(prompt_msg["method"], "session/prompt");

            let chunk = serde_json::json!({
                "jsonrpc": "2.0", "method": "session/update",
                "params": {
                    "sessionId": "sess-42",
                    "update": { "sessionUpdate": "agent_message_chunk",
                                "content": { "type": "text", "text": "working" } }
                }
            });
            send(&mut agent_write, &chunk).await;

            let perm = serde_json::json!({
                "jsonrpc": "2.0", "id": "perm-1",
                "method": "session/request_permission",
                "params": {
                    "sessionId": "sess-42",
                    "toolCall": { "toolCallId": "tc-1", "title": "rm file", "status": "pending" },
                    "options": [ { "optionId": "allow", "name": "Allow" } ]
                }
            });
            send(&mut agent_write, &perm).await;

            // Wait for the client's permission response before ending the turn.
            let line = lines.next_line().await.unwrap().unwrap();
            let perm_resp: Value = serde_json::from_str(&line).unwrap();
            assert_eq!(perm_resp["id"], "perm-1");
            assert_eq!(perm_resp["result"]["outcome"]["optionId"], "allow");

            let done = serde_json::json!({
                "jsonrpc": "2.0", "id": prompt_msg["id"],
                "result": { "stopReason": "end_turn" }
            });
            send(&mut agent_write, &done).await;
        });

        assert!(conn.initialize().await.is_ok());
        assert_eq!(conn.new_session("/tmp").await.unwrap(), "sess-42");

        // Issue the prompt concurrently with consuming its streamed events.
        let conn2 = conn.clone();
        let prompt_task =
            tokio::spawn(async move { conn2.prompt("sess-42", "do a thing").await });

        // First event: the streamed agent message chunk.
        match ev_rx.recv().await.unwrap() {
            UiEvent::Session { session_id, event } => {
                assert_eq!(session_id, "sess-42");
                assert_eq!(event, SessionEvent::AgentMessage { text: "working".into() });
            }
            other => panic!("expected session event, got {other:?}"),
        }

        // Second event: the permission request. Resolve it with "allow".
        match ev_rx.recv().await.unwrap() {
            UiEvent::Permission { token, tool_title, options, .. } => {
                assert_eq!(tool_title.as_deref(), Some("rm file"));
                assert_eq!(options[0].option_id, "allow");
                conn.resolve_permission(token, Some("allow")).await.unwrap();
            }
            other => panic!("expected permission event, got {other:?}"),
        }

        let result = prompt_task.await.unwrap().unwrap();
        assert_eq!(result["stopReason"], "end_turn");
    }

    #[tokio::test]
    async fn resolve_unknown_token_errors() {
        let (client_io, _agent_io) = tokio::io::duplex(1024);
        let (r, w) = tokio::io::split(client_io);
        let (ev_tx, _ev_rx) = mpsc::unbounded_channel();
        let conn = AcpConnection::start(r, w, ev_tx);
        assert!(matches!(
            conn.resolve_permission(999, Some("allow")).await,
            Err(AcpError::UnknownToken(999))
        ));
    }

    async fn send<W: AsyncWrite + Unpin>(w: &mut W, value: &Value) {
        let mut buf = serde_json::to_vec(value).unwrap();
        buf.push(b'\n');
        w.write_all(&buf).await.unwrap();
        w.flush().await.unwrap();
    }
}
