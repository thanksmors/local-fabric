// Typed bridge to the Rust backend.
//
// Mirrors the serde shapes in `src-tauri/src/acp` and `store.rs`. Tauri converts
// camelCase JS argument keys to the snake_case Rust parameter names, so commands
// are called with camelCase below.

import { invoke } from "@tauri-apps/api/core";
import { listen, type UnlistenFn } from "@tauri-apps/api/event";

export type AgentKind = "claude_code" | "codex" | "opencode";

export interface PlanEntry {
  content: string;
  priority: string | null;
  status: string | null;
}

/** Normalized, UI-facing streaming event (mirrors Rust `SessionEvent`). */
export type SessionEvent =
  | { type: "agent_message"; text: string }
  | { type: "agent_thought"; text: string }
  | { type: "user_message"; text: string }
  | {
      type: "tool_call";
      id: string;
      title: string | null;
      kind: string | null;
      status: string | null;
    }
  | { type: "tool_call_update"; id: string; status: string | null; content: unknown }
  | { type: "plan"; entries: PlanEntry[] }
  | { type: "unknown"; raw: unknown };

export interface PermissionOption {
  option_id: string;
  name: string;
  kind: string | null;
}

/** Events emitted by the backend over the `acp://event` channel. */
export type UiEvent =
  | { kind: "session"; session_id: string; event: SessionEvent }
  | {
      kind: "permission";
      session_id: string;
      token: number;
      tool_title: string | null;
      options: PermissionOption[];
    }
  | { kind: "notice"; message: string }
  | { kind: "closed" };

export interface DirEntry {
  name: string;
  path: string;
  is_dir: boolean;
}

export interface SessionRecord {
  id: string;
  agent: string;
  cwd: string;
  title: string;
  created_at: number;
}

export const startSession = (agent: AgentKind, cwd: string) =>
  invoke<string>("start_session", { agent, cwd });

export const sendPrompt = (sessionId: string, text: string) =>
  invoke<unknown>("send_prompt", { sessionId, text });

export const resolvePermission = (
  sessionId: string,
  token: number,
  optionId: string | null,
) => invoke<void>("resolve_permission", { sessionId, token, optionId });

export const listDir = (path: string) => invoke<DirEntry[]>("list_dir", { path });

export const listSessions = () => invoke<SessionRecord[]>("list_sessions");

/** Subscribe to backend events. Returns an unlisten function. */
export const onAcpEvent = (cb: (e: UiEvent) => void): Promise<UnlistenFn> =>
  listen<UiEvent>("acp://event", (e) => cb(e.payload));
