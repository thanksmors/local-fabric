// Reactive, multi-session store. Owns every agent session's state and routes
// backend events (`acp://event`) to the right session by id. UI components (the
// desktop taskbar and each SessionWindow) read and mutate sessions through here,
// so the windowing layer stays a thin view over this state.

import {
  startSession,
  sendPrompt,
  resolvePermission,
  listDir,
  type AgentKind,
  type SessionEvent,
  type UiEvent,
  type DirEntry,
  type PermissionOption,
} from "./api";

export interface Pending {
  token: number;
  toolTitle: string | null;
  options: PermissionOption[];
}

export interface SessionState {
  id: string;
  agent: AgentKind;
  cwd: string;
  events: SessionEvent[];
  pending: Pending | null;
  entries: DirEntry[];
}

class Sessions {
  list = $state<SessionState[]>([]);
  notices = $state<string[]>([]);

  get(id: string): SessionState | undefined {
    return this.list.find((s) => s.id === id);
  }

  /** Launch a new agent session and track it. */
  async start(agent: AgentKind, cwd: string): Promise<SessionState> {
    const id = await startSession(agent, cwd);
    const session: SessionState = {
      id,
      agent,
      cwd,
      events: [],
      pending: null,
      entries: [],
    };
    this.list.push(session);
    listDir(cwd)
      .then((e) => (session.entries = e))
      .catch((e) => this.notices.push(String(e)));
    return session;
  }

  async prompt(session: SessionState, text: string) {
    this.append(session, { type: "user_message", text });
    try {
      await sendPrompt(session.id, text);
    } catch (e) {
      this.notices.push(`Prompt failed: ${e}`);
    }
  }

  /** Resolve a pending permission request; `optionId === null` cancels. */
  async resolve(session: SessionState, optionId: string | null) {
    if (!session.pending) return;
    const token = session.pending.token;
    session.pending = null;
    try {
      await resolvePermission(session.id, token, optionId);
    } catch (e) {
      this.notices.push(`Approval failed: ${e}`);
    }
  }

  async navigate(session: SessionState, path: string) {
    try {
      session.entries = await listDir(path);
      session.cwd = path;
    } catch (e) {
      this.notices.push(String(e));
    }
  }

  remove(id: string) {
    this.list = this.list.filter((s) => s.id !== id);
  }

  dismissNotice(i: number) {
    this.notices = this.notices.filter((_, idx) => idx !== i);
  }

  /** Route one backend event to its session. */
  handle(e: UiEvent) {
    if (e.kind === "session") {
      const s = this.get(e.session_id);
      if (s) this.append(s, e.event);
    } else if (e.kind === "permission") {
      const s = this.get(e.session_id);
      if (s) {
        s.pending = {
          token: e.token,
          toolTitle: e.tool_title,
          options: e.options,
        };
      }
    } else if (e.kind === "notice") {
      this.notices.push(e.message);
    } else if (e.kind === "closed") {
      this.notices.push("An agent session closed.");
    }
  }

  // Coalesce consecutive streamed chunks of the same kind into one bubble.
  private append(session: SessionState, ev: SessionEvent) {
    const last = session.events[session.events.length - 1];
    if (
      last &&
      (ev.type === "agent_message" || ev.type === "agent_thought") &&
      last.type === ev.type
    ) {
      last.text += ev.text;
      session.events = [...session.events];
    } else {
      session.events = [...session.events, ev];
    }
  }
}

export const sessions = new Sessions();
