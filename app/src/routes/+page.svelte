<script lang="ts">
  import { onMount } from "svelte";
  import type { UnlistenFn } from "@tauri-apps/api/event";
  import {
    onAcpEvent,
    startSession,
    sendPrompt,
    resolvePermission,
    listDir,
    listSessions,
    type AgentKind,
    type SessionEvent,
    type UiEvent,
    type DirEntry,
    type SessionRecord,
    type PermissionOption,
  } from "$lib/api";
  import EventStream from "$lib/components/EventStream.svelte";
  import ApprovalDialog from "$lib/components/ApprovalDialog.svelte";
  import FileTree from "$lib/components/FileTree.svelte";

  type Pending = {
    token: number;
    toolTitle: string | null;
    options: PermissionOption[];
  };

  let agent = $state<AgentKind>("claude_code");
  let cwd = $state("/home/user");
  let sessionId = $state<string | null>(null);
  let starting = $state(false);

  let events = $state<SessionEvent[]>([]);
  let pending = $state<Pending | null>(null);
  let notices = $state<string[]>([]);
  let promptText = $state("");

  let entries = $state<DirEntry[]>([]);
  let pastSessions = $state<SessionRecord[]>([]);

  onMount(() => {
    let unlisten: UnlistenFn | undefined;
    onAcpEvent(handleEvent).then((fn) => (unlisten = fn));
    listSessions()
      .then((s) => (pastSessions = s))
      .catch((e) => (notices = [...notices, String(e)]));
    return () => unlisten?.();
  });

  function handleEvent(e: UiEvent) {
    if (e.kind === "session") {
      appendStreamEvent(e.event);
    } else if (e.kind === "permission") {
      pending = { token: e.token, toolTitle: e.tool_title, options: e.options };
    } else if (e.kind === "notice") {
      notices = [...notices, e.message];
    } else if (e.kind === "closed") {
      notices = [...notices, "Agent session closed."];
    }
  }

  // Coalesce consecutive streamed chunks of the same kind into one bubble.
  function appendStreamEvent(ev: SessionEvent) {
    const last = events[events.length - 1];
    if (
      last &&
      (ev.type === "agent_message" || ev.type === "agent_thought") &&
      last.type === ev.type
    ) {
      last.text += ev.text;
      events = [...events];
    } else {
      events = [...events, ev];
    }
  }

  async function navigate(path: string) {
    try {
      entries = await listDir(path);
      cwd = path;
    } catch (e) {
      notices = [...notices, String(e)];
    }
  }

  async function start() {
    starting = true;
    try {
      sessionId = await startSession(agent, cwd);
      events = [];
      await navigate(cwd);
      pastSessions = await listSessions();
    } catch (e) {
      notices = [...notices, `Failed to start: ${e}`];
    } finally {
      starting = false;
    }
  }

  async function send() {
    const text = promptText.trim();
    if (!sessionId || !text) return;
    promptText = "";
    appendStreamEvent({ type: "user_message", text });
    try {
      await sendPrompt(sessionId, text);
    } catch (e) {
      notices = [...notices, `Prompt failed: ${e}`];
    }
  }

  async function resolve(optionId: string | null) {
    if (!sessionId || !pending) return;
    const token = pending.token;
    pending = null;
    try {
      await resolvePermission(sessionId, token, optionId);
    } catch (e) {
      notices = [...notices, `Approval failed: ${e}`];
    }
  }

  function onComposerKey(e: KeyboardEvent) {
    if (e.key === "Enter" && (e.metaKey || e.ctrlKey)) {
      e.preventDefault();
      send();
    }
  }
</script>

<div class="app">
  <header>
    <strong>local-fabric</strong>
    <span class="tag">agent harness</span>
    <div class="spacer"></div>
    <select bind:value={agent} disabled={!!sessionId}>
      <option value="claude_code">Claude Code</option>
      <option value="codex">Codex CLI</option>
      <option value="opencode">opencode</option>
    </select>
    <input class="cwd" bind:value={cwd} placeholder="working directory" disabled={!!sessionId} />
    <button onclick={start} disabled={starting || !!sessionId}>
      {starting ? "Starting…" : sessionId ? "Running" : "Start session"}
    </button>
  </header>

  <div class="body">
    <aside class="sidebar">
      <FileTree {cwd} {entries} onnavigate={navigate} />
      {#if pastSessions.length}
        <div class="past">
          <div class="past-h">Past sessions</div>
          {#each pastSessions as s (s.id)}
            <div class="past-item" title={s.cwd}>{s.title}</div>
          {/each}
        </div>
      {/if}
    </aside>

    <main>
      <EventStream {events} />
      {#if notices.length}
        <div class="notices">
          {#each notices as n, i (i)}<div class="notice">{n}</div>{/each}
        </div>
      {/if}
      <div class="composer">
        <textarea
          bind:value={promptText}
          onkeydown={onComposerKey}
          placeholder={sessionId
            ? "Prompt the agent…  (Ctrl/Cmd+Enter to send)"
            : "Start a session first"}
          disabled={!sessionId}
        ></textarea>
        <button onclick={send} disabled={!sessionId || !promptText.trim()}>Send</button>
      </div>
    </main>
  </div>

  {#if pending}
    <ApprovalDialog toolTitle={pending.toolTitle} options={pending.options} onresolve={resolve} />
  {/if}
</div>

<style>
  :global(html, body) {
    margin: 0;
    height: 100%;
    background: #0b0e12;
    color: #e7ecf1;
    font-family: Inter, system-ui, Avenir, Helvetica, Arial, sans-serif;
  }
  .app {
    display: flex;
    flex-direction: column;
    height: 100vh;
  }
  header {
    display: flex;
    align-items: center;
    gap: 0.6rem;
    padding: 0.5rem 0.9rem;
    border-bottom: 1px solid #1c232b;
    background: #0f141a;
  }
  .tag {
    font-size: 0.7rem;
    text-transform: uppercase;
    letter-spacing: 0.08em;
    opacity: 0.5;
  }
  .spacer { flex: 1; }
  .body { display: flex; flex: 1; min-height: 0; }
  .sidebar {
    width: 16rem;
    border-right: 1px solid #1c232b;
    display: flex;
    flex-direction: column;
    min-height: 0;
  }
  .past { border-top: 1px solid #1c232b; padding: 0.5rem 0; overflow-y: auto; max-height: 12rem; }
  .past-h {
    font-size: 0.7rem;
    text-transform: uppercase;
    opacity: 0.5;
    padding: 0 0.6rem 0.3rem;
  }
  .past-item {
    font-size: 0.78rem;
    padding: 0.2rem 0.6rem;
    white-space: nowrap;
    overflow: hidden;
    text-overflow: ellipsis;
    opacity: 0.85;
  }
  main { flex: 1; display: flex; flex-direction: column; min-width: 0; }
  .notices { padding: 0 0.75rem; }
  .notice {
    font-size: 0.78rem;
    color: #fca5a5;
    background: #1f1416;
    border-radius: 4px;
    padding: 0.3rem 0.5rem;
    margin-bottom: 0.3rem;
  }
  .composer {
    display: flex;
    gap: 0.5rem;
    padding: 0.6rem 0.75rem;
    border-top: 1px solid #1c232b;
  }
  textarea {
    flex: 1;
    resize: none;
    height: 3.2rem;
    background: #141a21;
    color: inherit;
    border: 1px solid #28313b;
    border-radius: 8px;
    padding: 0.5rem 0.6rem;
    font-family: inherit;
    font-size: 0.9rem;
  }
  select,
  input,
  button {
    background: #1a212a;
    color: inherit;
    border: 1px solid #2a333d;
    border-radius: 8px;
    padding: 0.4rem 0.7rem;
    font-size: 0.85rem;
  }
  .cwd { width: 16rem; font-family: ui-monospace, monospace; }
  button { cursor: pointer; }
  button:hover:not(:disabled) { border-color: #4a5763; }
  button:disabled,
  select:disabled,
  input:disabled { opacity: 0.5; cursor: default; }
</style>
