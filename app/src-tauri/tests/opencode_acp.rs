//! End-to-end ACP integration test against a real `opencode acp` subprocess.
//!
//! Ignored by default because it needs external setup:
//!   * `opencode` installed and on PATH,
//!   * a provider authenticated (e.g. MiniMax via `opencode auth login`) with a
//!     default model set,
//!   * network access to the model API.
//!
//! Run it explicitly:
//!   cargo test --test opencode_acp -- --ignored --nocapture
//!
//! It drives the harness's own [`AcpConnection`] through the full path —
//! initialize → session/new → session/prompt — and asserts that at least one
//! streamed agent message comes back, proving opencode + the configured model
//! work through our client (no GUI required).

use std::time::Duration;

use app_lib::acp::protocol::SessionEvent;
use app_lib::acp::{AcpConnection, AgentKind, UiEvent};
use tokio::sync::mpsc;

#[tokio::test(flavor = "multi_thread")]
#[ignore = "requires opencode installed and an authenticated provider (e.g. MiniMax)"]
async fn opencode_acp_streams_an_agent_message() {
    let cwd = std::env::temp_dir()
        .join(format!("lf-opencode-acp-{}", std::process::id()));
    std::fs::create_dir_all(&cwd).unwrap();

    let spec = AgentKind::Opencode.launch_spec();
    let mut child = tokio::process::Command::new(&spec.program)
        .args(&spec.args)
        .current_dir(&cwd)
        .stdin(std::process::Stdio::piped())
        .stdout(std::process::Stdio::piped())
        .stderr(std::process::Stdio::inherit())
        .kill_on_drop(true)
        .spawn()
        .expect("failed to spawn `opencode acp` — is opencode installed and on PATH?");

    let stdin = child.stdin.take().unwrap();
    let stdout = child.stdout.take().unwrap();
    let (tx, mut rx) = mpsc::unbounded_channel::<UiEvent>();
    let conn = AcpConnection::start(stdout, stdin, tx);

    conn.initialize().await.expect("ACP initialize failed");
    let session_id = conn
        .new_session(&cwd.to_string_lossy())
        .await
        .expect("session/new failed");
    assert!(!session_id.is_empty(), "expected a non-empty session id");

    // Fire the prompt; streamed updates arrive on `rx` concurrently.
    let conn2 = conn.clone();
    let sid = session_id.clone();
    let prompt_task = tokio::spawn(async move {
        conn2.prompt(&sid, "Reply with the single word: pong").await
    });

    // Collect events until we see an agent message, or time out.
    let saw_message = tokio::time::timeout(Duration::from_secs(120), async {
        while let Some(ev) = rx.recv().await {
            match ev {
                UiEvent::Session { event: SessionEvent::AgentMessage { text }, .. } => {
                    eprintln!("agent said: {text}");
                    return true;
                }
                UiEvent::Notice { message } => eprintln!("notice: {message}"),
                UiEvent::Closed => return false,
                _ => {}
            }
        }
        false
    })
    .await
    .expect("timed out waiting for an agent message");

    assert!(saw_message, "no streamed agent message was received");

    let _ = prompt_task.await;
}
