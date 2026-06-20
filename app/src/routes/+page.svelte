<script lang="ts">
  import { onMount, mount, unmount } from "svelte";
  import type { UnlistenFn } from "@tauri-apps/api/event";
  import WinBox from "winbox/src/js/winbox.js";
  import "winbox/dist/css/winbox.min.css";

  import { onAcpEvent, type AgentKind } from "$lib/api";
  import { sessions, type SessionState } from "$lib/sessions.svelte";
  import SessionWindow from "$lib/components/SessionWindow.svelte";

  // --- new-session launcher state ---
  let agent = $state<AgentKind>("opencode");
  let cwd = $state("/home/user");
  let starting = $state(false);

  const AGENT_LABELS: Record<AgentKind, string> = {
    opencode: "opencode",
    claude_code: "Claude Code",
    codex: "Codex CLI",
  };

  // --- windowing layer (the only place WinBox is touched; swap here later) ---
  type OpenWindow = { win: WinBox; comp: Record<string, unknown> };
  const windows = new Map<string, OpenWindow>();
  let cascade = 0;

  function openWindow(session: SessionState) {
    const offset = (cascade++ % 8) * 28;
    const win = new WinBox({
      title: `${AGENT_LABELS[session.agent]} — ${session.cwd}`,
      width: 760,
      height: 540,
      minwidth: 380,
      minheight: 280,
      x: 80 + offset,
      y: 60 + offset,
      onclose: () => {
        const open = windows.get(session.id);
        if (open) unmount(open.comp);
        windows.delete(session.id);
        sessions.remove(session.id);
        return false;
      },
    });
    const comp = mount(SessionWindow, { target: win.body, props: { session } });
    windows.set(session.id, { win, comp });
  }

  async function newSession() {
    if (starting || !cwd.trim()) return;
    starting = true;
    try {
      const session = await sessions.start(agent, cwd.trim());
      openWindow(session);
    } catch (e) {
      sessions.notices.push(`Failed to start: ${e}`);
    } finally {
      starting = false;
    }
  }

  function focusWindow(id: string) {
    windows.get(id)?.win.focus();
  }

  onMount(() => {
    let unlisten: UnlistenFn | undefined;
    onAcpEvent((e) => sessions.handle(e)).then((fn) => (unlisten = fn));
    return () => unlisten?.();
  });
</script>

<div class="desktop">
  <div class="hint">
    {#if sessions.list.length === 0}
      <div class="empty">
        <h1>local-fabric</h1>
        <p>Launch an agent session from the bar below. Each session opens in its own window.</p>
      </div>
    {/if}
  </div>

  {#if sessions.notices.length}
    <div class="notices">
      {#each sessions.notices as n, i (i)}
        <button class="notice" onclick={() => sessions.dismissNotice(i)} title="dismiss">{n} ✕</button>
      {/each}
    </div>
  {/if}

  <footer class="taskbar">
    <strong class="brand">local-fabric</strong>
    <div class="open-windows">
      {#each sessions.list as s (s.id)}
        <button class="tab" onclick={() => focusWindow(s.id)} title={s.cwd}>
          {AGENT_LABELS[s.agent]}
        </button>
      {/each}
    </div>
    <div class="launcher">
      <select bind:value={agent}>
        <option value="opencode">opencode</option>
        <option value="claude_code">Claude Code</option>
        <option value="codex">Codex CLI</option>
      </select>
      <input class="cwd" bind:value={cwd} placeholder="working directory" />
      <button class="start" onclick={newSession} disabled={starting || !cwd.trim()}>
        {starting ? "Starting…" : "+ New session"}
      </button>
    </div>
  </footer>
</div>

<style>
  :global(html, body) {
    margin: 0;
    height: 100%;
    background: #0b0e12;
    color: #e7ecf1;
    font-family: Inter, system-ui, Avenir, Helvetica, Arial, sans-serif;
  }
  .desktop {
    position: fixed;
    inset: 0;
    background:
      radial-gradient(1200px 600px at 70% -10%, #15212f 0%, transparent 60%),
      radial-gradient(900px 500px at -10% 110%, #1a1726 0%, transparent 55%),
      #0b0e12;
    overflow: hidden;
  }
  .hint {
    position: absolute;
    inset: 0;
    display: flex;
    align-items: center;
    justify-content: center;
    pointer-events: none;
  }
  .empty {
    text-align: center;
    opacity: 0.55;
  }
  .empty h1 {
    margin: 0 0 0.3rem;
    font-weight: 600;
    letter-spacing: 0.02em;
  }
  .notices {
    position: absolute;
    top: 0.75rem;
    right: 0.75rem;
    display: flex;
    flex-direction: column;
    gap: 0.3rem;
    max-width: 26rem;
    z-index: 10;
  }
  .notice {
    text-align: left;
    font-size: 0.78rem;
    color: #fca5a5;
    background: #1f1416;
    border: 1px solid #3a1f22;
    border-radius: 6px;
    padding: 0.35rem 0.55rem;
    cursor: pointer;
  }
  .taskbar {
    position: absolute;
    left: 0;
    right: 0;
    bottom: 0;
    display: flex;
    align-items: center;
    gap: 0.75rem;
    padding: 0.45rem 0.8rem;
    background: rgba(15, 20, 26, 0.92);
    border-top: 1px solid #1c232b;
    backdrop-filter: blur(6px);
    z-index: 20;
  }
  .brand {
    font-size: 0.9rem;
  }
  .open-windows {
    display: flex;
    gap: 0.35rem;
    flex: 1;
    overflow-x: auto;
  }
  .tab {
    font-size: 0.78rem;
    background: #1a212a;
    color: inherit;
    border: 1px solid #2a333d;
    border-radius: 6px;
    padding: 0.25rem 0.6rem;
    cursor: pointer;
    white-space: nowrap;
  }
  .tab:hover {
    border-color: #4a5763;
  }
  .launcher {
    display: flex;
    gap: 0.4rem;
    align-items: center;
  }
  select,
  input,
  .start {
    background: #1a212a;
    color: inherit;
    border: 1px solid #2a333d;
    border-radius: 8px;
    padding: 0.35rem 0.6rem;
    font-size: 0.82rem;
  }
  .cwd {
    width: 15rem;
    font-family: ui-monospace, monospace;
  }
  .start {
    cursor: pointer;
  }
  .start:hover:not(:disabled) {
    border-color: #4a5763;
  }
  .start:disabled {
    opacity: 0.5;
    cursor: default;
  }
</style>
