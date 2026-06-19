//! Agent Client Protocol (ACP) message types and normalization.
//!
//! ACP is a JSON-RPC 2.0 protocol spoken over stdio between a client (this app)
//! and an agent subprocess. See <https://agentclientprotocol.com>. This module
//! models the subset the harness needs for the v1 vertical slice: initialize,
//! session lifecycle, streaming session updates, and permission requests.
//!
//! Everything here is pure data and pure functions so it can be unit-tested
//! without a running agent. The async I/O lives in [`super::connection`].

use serde::{Deserialize, Serialize};
use serde_json::{json, Value};

/// ACP protocol version this client speaks.
pub const PROTOCOL_VERSION: u32 = 1;

/// JSON-RPC 2.0 version tag.
pub const JSONRPC_VERSION: &str = "2.0";

// ---------------------------------------------------------------------------
// JSON-RPC framing
// ---------------------------------------------------------------------------

/// A JSON-RPC error object.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct JsonRpcError {
    pub code: i64,
    pub message: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<Value>,
}

/// JSON-RPC error code for an unknown method (per spec).
pub const METHOD_NOT_FOUND: i64 = -32601;

/// A parsed inbound JSON-RPC message from the agent.
///
/// Classification is intentionally tolerant: agents differ slightly, so we key
/// off structural presence of `method`/`id` rather than a rigid schema.
#[derive(Debug, Clone, PartialEq)]
pub enum Inbound {
    /// Response to a request we sent (has our numeric `id`, plus result/error).
    Response {
        id: u64,
        result: Result<Value, JsonRpcError>,
    },
    /// Notification from the agent (has `method`, no `id`), e.g. `session/update`.
    Notification { method: String, params: Value },
    /// Request from the agent (has `method` and `id`), e.g.
    /// `session/request_permission`. The `id` is preserved verbatim because the
    /// agent chooses its own id type (number or string).
    Request {
        id: Value,
        method: String,
        params: Value,
    },
}

/// Classify a raw inbound JSON value into a [`Inbound`] variant.
pub fn classify(value: Value) -> Result<Inbound, String> {
    let obj = value
        .as_object()
        .ok_or_else(|| "message is not a JSON object".to_string())?;
    let has_method = obj.contains_key("method");
    let has_id = obj.get("id").map(|v| !v.is_null()).unwrap_or(false);

    match (has_method, has_id) {
        (true, true) => Ok(Inbound::Request {
            id: obj["id"].clone(),
            method: obj["method"].as_str().unwrap_or_default().to_string(),
            params: obj.get("params").cloned().unwrap_or(Value::Null),
        }),
        (true, false) => Ok(Inbound::Notification {
            method: obj["method"].as_str().unwrap_or_default().to_string(),
            params: obj.get("params").cloned().unwrap_or(Value::Null),
        }),
        (false, true) => {
            let id = obj["id"]
                .as_u64()
                .ok_or_else(|| "response id is not a u64".to_string())?;
            if let Some(err) = obj.get("error").filter(|e| !e.is_null()) {
                let err: JsonRpcError = serde_json::from_value(err.clone())
                    .map_err(|e| format!("bad error object: {e}"))?;
                Ok(Inbound::Response {
                    id,
                    result: Err(err),
                })
            } else {
                Ok(Inbound::Response {
                    id,
                    result: Ok(obj.get("result").cloned().unwrap_or(Value::Null)),
                })
            }
        }
        (false, false) => Err("message has neither method nor id".to_string()),
    }
}

/// Build an outbound request frame.
pub fn request_frame(id: u64, method: &str, params: Value) -> Value {
    json!({ "jsonrpc": JSONRPC_VERSION, "id": id, "method": method, "params": params })
}

/// Build an outbound success response frame for an agent-originated request.
pub fn response_frame(id: Value, result: Value) -> Value {
    json!({ "jsonrpc": JSONRPC_VERSION, "id": id, "result": result })
}

/// Build an outbound error response frame for an agent-originated request.
pub fn error_response_frame(id: Value, code: i64, message: &str) -> Value {
    json!({ "jsonrpc": JSONRPC_VERSION, "id": id, "error": { "code": code, "message": message } })
}

// ---------------------------------------------------------------------------
// ACP method params / results
// ---------------------------------------------------------------------------

/// Params for `initialize`.
///
/// For the v1 slice the client does not yet serve the agent's file-system
/// requests, so both fs capabilities are false and the agent uses its own file
/// access.
pub fn initialize_params() -> Value {
    json!({
        "protocolVersion": PROTOCOL_VERSION,
        "clientCapabilities": {
            "fs": { "readTextFile": false, "writeTextFile": false }
        }
    })
}

/// Params for `session/new`.
pub fn new_session_params(cwd: &str) -> Value {
    json!({ "cwd": cwd, "mcpServers": [] })
}

/// Params for `session/prompt` with a single text block.
pub fn prompt_params(session_id: &str, text: &str) -> Value {
    json!({
        "sessionId": session_id,
        "prompt": [ { "type": "text", "text": text } ]
    })
}

/// Outcome payload for a `session/request_permission` response.
pub fn permission_selected_result(option_id: &str) -> Value {
    json!({ "outcome": { "outcome": "selected", "optionId": option_id } })
}

/// Outcome payload for a cancelled permission request.
pub fn permission_cancelled_result() -> Value {
    json!({ "outcome": { "outcome": "cancelled" } })
}

// ---------------------------------------------------------------------------
// Session update modeling + normalization
// ---------------------------------------------------------------------------

/// A content block. Only text is rendered in the v1 slice; richer blocks are
/// captured as [`ContentBlock::Other`] so deserialization never fails.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum ContentBlock {
    Text { text: String },
    #[serde(other)]
    Other,
}

impl ContentBlock {
    /// Best-effort text extraction; empty for non-text blocks.
    pub fn text(&self) -> String {
        match self {
            ContentBlock::Text { text } => text.clone(),
            ContentBlock::Other => String::new(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ToolCall {
    pub tool_call_id: String,
    #[serde(default)]
    pub title: Option<String>,
    #[serde(default)]
    pub kind: Option<String>,
    #[serde(default)]
    pub status: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ToolCallUpdate {
    pub tool_call_id: String,
    #[serde(default)]
    pub status: Option<String>,
    #[serde(default)]
    pub title: Option<String>,
    #[serde(default)]
    pub content: Option<Value>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PlanEntry {
    pub content: String,
    #[serde(default)]
    pub priority: Option<String>,
    #[serde(default)]
    pub status: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PermissionOption {
    pub option_id: String,
    pub name: String,
    #[serde(default)]
    pub kind: Option<String>,
}

/// Params of a `session/request_permission` request from the agent.
#[derive(Debug, Clone, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RequestPermissionParams {
    pub session_id: String,
    #[serde(default)]
    pub tool_call: Option<ToolCall>,
    pub options: Vec<PermissionOption>,
}

/// The discriminated `update` object inside a `session/update` notification.
#[derive(Debug, Clone, PartialEq, Deserialize)]
#[serde(tag = "sessionUpdate", rename_all = "snake_case")]
pub enum SessionUpdate {
    AgentMessageChunk { content: ContentBlock },
    AgentThoughtChunk { content: ContentBlock },
    UserMessageChunk { content: ContentBlock },
    ToolCall(ToolCall),
    ToolCallUpdate(ToolCallUpdate),
    Plan { entries: Vec<PlanEntry> },
    #[serde(other)]
    Unknown,
}

/// A normalized, UI-facing session event. This is the single shape the Svelte
/// frontend consumes regardless of which agent produced it.
#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum SessionEvent {
    AgentMessage {
        text: String,
    },
    AgentThought {
        text: String,
    },
    UserMessage {
        text: String,
    },
    ToolCall {
        id: String,
        title: Option<String>,
        kind: Option<String>,
        status: Option<String>,
    },
    ToolCallUpdate {
        id: String,
        status: Option<String>,
        content: Option<Value>,
    },
    Plan {
        entries: Vec<PlanEntry>,
    },
    /// An update we did not model; the raw payload is preserved for the UI/debug.
    Unknown {
        raw: Value,
    },
}

/// Normalize a raw `update` value (the inner object of a `session/update`
/// notification) into a [`SessionEvent`]. Unknown or unparseable shapes are
/// preserved verbatim rather than dropped.
pub fn normalize_update(update: &Value) -> SessionEvent {
    match serde_json::from_value::<SessionUpdate>(update.clone()) {
        Ok(SessionUpdate::AgentMessageChunk { content }) => SessionEvent::AgentMessage {
            text: content.text(),
        },
        Ok(SessionUpdate::AgentThoughtChunk { content }) => SessionEvent::AgentThought {
            text: content.text(),
        },
        Ok(SessionUpdate::UserMessageChunk { content }) => SessionEvent::UserMessage {
            text: content.text(),
        },
        Ok(SessionUpdate::ToolCall(tc)) => SessionEvent::ToolCall {
            id: tc.tool_call_id,
            title: tc.title,
            kind: tc.kind,
            status: tc.status,
        },
        Ok(SessionUpdate::ToolCallUpdate(tc)) => SessionEvent::ToolCallUpdate {
            id: tc.tool_call_id,
            status: tc.status,
            content: tc.content,
        },
        Ok(SessionUpdate::Plan { entries }) => SessionEvent::Plan { entries },
        Ok(SessionUpdate::Unknown) | Err(_) => SessionEvent::Unknown {
            raw: update.clone(),
        },
    }
}

/// Extract `sessionId` from a `session/update` notification's params.
pub fn session_id_of(params: &Value) -> Option<String> {
    params.get("sessionId")?.as_str().map(|s| s.to_string())
}

/// Extract the inner `update` object from a `session/update` notification.
pub fn update_of(params: &Value) -> Option<&Value> {
    params.get("update")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn classifies_response() {
        let v = json!({ "jsonrpc": "2.0", "id": 7, "result": { "ok": true } });
        match classify(v).unwrap() {
            Inbound::Response { id, result } => {
                assert_eq!(id, 7);
                assert_eq!(result.unwrap(), json!({ "ok": true }));
            }
            other => panic!("expected response, got {other:?}"),
        }
    }

    #[test]
    fn classifies_error_response() {
        let v = json!({ "jsonrpc": "2.0", "id": 3, "error": { "code": -32601, "message": "nope" } });
        match classify(v).unwrap() {
            Inbound::Response {
                id,
                result: Err(e),
            } => {
                assert_eq!(id, 3);
                assert_eq!(e.code, METHOD_NOT_FOUND);
            }
            other => panic!("expected error response, got {other:?}"),
        }
    }

    #[test]
    fn classifies_notification() {
        let v = json!({ "jsonrpc": "2.0", "method": "session/update", "params": { "sessionId": "s1" } });
        match classify(v).unwrap() {
            Inbound::Notification { method, params } => {
                assert_eq!(method, "session/update");
                assert_eq!(session_id_of(&params).as_deref(), Some("s1"));
            }
            other => panic!("expected notification, got {other:?}"),
        }
    }

    #[test]
    fn classifies_agent_request() {
        let v = json!({
            "jsonrpc": "2.0",
            "id": "perm-1",
            "method": "session/request_permission",
            "params": { "sessionId": "s1", "options": [] }
        });
        match classify(v).unwrap() {
            Inbound::Request { id, method, .. } => {
                assert_eq!(id, json!("perm-1"));
                assert_eq!(method, "session/request_permission");
            }
            other => panic!("expected request, got {other:?}"),
        }
    }

    #[test]
    fn normalizes_agent_message_chunk() {
        let update = json!({
            "sessionUpdate": "agent_message_chunk",
            "content": { "type": "text", "text": "hello" }
        });
        assert_eq!(
            normalize_update(&update),
            SessionEvent::AgentMessage {
                text: "hello".into()
            }
        );
    }

    #[test]
    fn normalizes_tool_call() {
        let update = json!({
            "sessionUpdate": "tool_call",
            "toolCallId": "tc-1",
            "title": "Read file",
            "kind": "read",
            "status": "pending"
        });
        assert_eq!(
            normalize_update(&update),
            SessionEvent::ToolCall {
                id: "tc-1".into(),
                title: Some("Read file".into()),
                kind: Some("read".into()),
                status: Some("pending".into()),
            }
        );
    }

    #[test]
    fn unknown_update_preserves_raw() {
        let update = json!({ "sessionUpdate": "future_thing", "weird": 1 });
        match normalize_update(&update) {
            SessionEvent::Unknown { raw } => assert_eq!(raw, update),
            other => panic!("expected unknown, got {other:?}"),
        }
    }

    #[test]
    fn parses_permission_request_params() {
        let params = json!({
            "sessionId": "s1",
            "toolCall": { "toolCallId": "tc-9", "title": "rm -rf", "status": "pending" },
            "options": [
                { "optionId": "allow", "name": "Allow", "kind": "allow_once" },
                { "optionId": "deny", "name": "Deny", "kind": "reject_once" }
            ]
        });
        let parsed: RequestPermissionParams = serde_json::from_value(params).unwrap();
        assert_eq!(parsed.session_id, "s1");
        assert_eq!(parsed.options.len(), 2);
        assert_eq!(parsed.tool_call.unwrap().title.as_deref(), Some("rm -rf"));
    }

    #[test]
    fn non_object_message_is_error() {
        assert!(classify(json!([1, 2, 3])).is_err());
        assert!(classify(json!({ "jsonrpc": "2.0" })).is_err());
    }
}
