//! Tauri backend for the local-first agent harness (v1 vertical slice).
//!
//! Responsibilities:
//! * launch an agent as an ACP subprocess and own its lifetime,
//! * forward normalized [`UiEvent`]s to the frontend over a Tauri event,
//! * expose commands the Svelte UI calls: start a session, send a prompt,
//!   resolve a permission request, browse the working directory, list sessions.
//!
//! All cross-process work goes through [`acp`]; persistence through [`store`].

pub mod acp;
mod files;
mod store;

use std::collections::HashMap;
use std::sync::{Arc, Mutex};

use rusqlite::Connection;
use serde_json::Value;
use tauri::{AppHandle, Emitter, Manager, State};
use tokio::sync::Mutex as AsyncMutex;

use acp::{AcpConnection, AgentKind, UiEvent};
use store::SessionRecord;

/// The Tauri event channel name carrying [`UiEvent`]s to the frontend.
const EVENT_NAME: &str = "acp://event";

/// A running agent session: the ACP connection plus the child process it owns.
/// Keeping the child here keeps it alive; dropping it (via `kill_on_drop`) tears
/// the agent down when the session is removed.
struct LiveSession {
    conn: Arc<AcpConnection>,
    #[allow(dead_code)]
    child: tokio::process::Child,
}

/// Shared application state.
struct AppState {
    sessions: AsyncMutex<HashMap<String, LiveSession>>,
    db: Mutex<Connection>,
}

/// Launch an agent, complete the ACP handshake, open a session, and begin
/// streaming its events to the frontend. Returns the new session id.
#[tauri::command]
async fn start_session(
    app: AppHandle,
    state: State<'_, AppState>,
    agent: AgentKind,
    cwd: String,
) -> Result<String, String> {
    let spec = agent.launch_spec();
    let mut cmd = tokio::process::Command::new(&spec.program);
    cmd.args(&spec.args)
        .current_dir(&cwd)
        .stdin(std::process::Stdio::piped())
        .stdout(std::process::Stdio::piped())
        .stderr(std::process::Stdio::inherit())
        .kill_on_drop(true);

    let mut child = cmd
        .spawn()
        .map_err(|e| format!("failed to launch {}: {e}", agent.label()))?;
    let stdin = child.stdin.take().ok_or("agent stdin unavailable")?;
    let stdout = child.stdout.take().ok_or("agent stdout unavailable")?;

    let (ev_tx, mut ev_rx) = tokio::sync::mpsc::unbounded_channel::<UiEvent>();
    let conn = AcpConnection::start(stdout, stdin, ev_tx);

    // Forward connection events to the frontend.
    let app_handle = app.clone();
    tokio::spawn(async move {
        while let Some(event) = ev_rx.recv().await {
            let _ = app_handle.emit(EVENT_NAME, event);
        }
    });

    conn.initialize().await.map_err(|e| e.to_string())?;
    let session_id = conn.new_session(&cwd).await.map_err(|e| e.to_string())?;

    // Persist the session (brief synchronous DB scope, no await held).
    let record = SessionRecord {
        id: session_id.clone(),
        agent: serde_json::to_value(agent)
            .ok()
            .and_then(|v| v.as_str().map(String::from))
            .unwrap_or_default(),
        cwd: cwd.clone(),
        title: format!("{} — {}", agent.label(), cwd),
        created_at: now_secs(),
    };
    {
        let db = state.db.lock().map_err(|e| e.to_string())?;
        store::upsert_session(&db, &record).map_err(|e| e.to_string())?;
    }

    state
        .sessions
        .lock()
        .await
        .insert(session_id.clone(), LiveSession { conn, child });

    Ok(session_id)
}

/// Send a prompt turn to an existing session. Resolves when the turn ends;
/// streamed output arrives via the event channel in the meantime.
#[tauri::command]
async fn send_prompt(
    state: State<'_, AppState>,
    session_id: String,
    text: String,
) -> Result<Value, String> {
    let conn = lookup(&state, &session_id).await?;
    conn.prompt(&session_id, &text)
        .await
        .map_err(|e| e.to_string())
}

/// Resolve a pending permission request. `option_id == None` cancels it.
#[tauri::command]
async fn resolve_permission(
    state: State<'_, AppState>,
    session_id: String,
    token: u64,
    option_id: Option<String>,
) -> Result<(), String> {
    let conn = lookup(&state, &session_id).await?;
    conn.resolve_permission(token, option_id.as_deref())
        .await
        .map_err(|e| e.to_string())
}

/// List the immediate children of a directory for the file tree.
#[tauri::command]
fn list_dir(path: String) -> Result<Vec<files::DirEntry>, String> {
    files::list_dir(std::path::Path::new(&path)).map_err(|e| e.to_string())
}

/// List persisted sessions, most recent first.
#[tauri::command]
fn list_sessions(state: State<'_, AppState>) -> Result<Vec<SessionRecord>, String> {
    let db = state.db.lock().map_err(|e| e.to_string())?;
    store::list_sessions(&db).map_err(|e| e.to_string())
}

async fn lookup(
    state: &State<'_, AppState>,
    session_id: &str,
) -> Result<Arc<AcpConnection>, String> {
    state
        .sessions
        .lock()
        .await
        .get(session_id)
        .map(|s| s.conn.clone())
        .ok_or_else(|| format!("no live session: {session_id}"))
}

fn now_secs() -> i64 {
    std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map(|d| d.as_secs() as i64)
        .unwrap_or(0)
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .setup(|app| {
            let data_dir = app.path().app_data_dir()?;
            std::fs::create_dir_all(&data_dir)?;
            let conn = store::open(&data_dir.join("harness.db"))?;
            app.manage(AppState {
                sessions: AsyncMutex::new(HashMap::new()),
                db: Mutex::new(conn),
            });
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            start_session,
            send_prompt,
            resolve_permission,
            list_dir,
            list_sessions
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
