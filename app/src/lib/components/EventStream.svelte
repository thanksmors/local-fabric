<script lang="ts">
  import type { SessionEvent } from "$lib/api";

  let { events }: { events: SessionEvent[] } = $props();
</script>

<div class="stream">
  {#each events as ev, i (i)}
    {#if ev.type === "user_message"}
      <div class="msg user"><span class="who">you</span>{ev.text}</div>
    {:else if ev.type === "agent_message"}
      <div class="msg agent"><span class="who">agent</span>{ev.text}</div>
    {:else if ev.type === "agent_thought"}
      <div class="msg thought"><span class="who">thinking</span>{ev.text}</div>
    {:else if ev.type === "tool_call"}
      <div class="tool">
        <span class="badge">tool</span>
        <span class="tool-title">{ev.title ?? ev.id}</span>
        {#if ev.kind}<span class="dim">· {ev.kind}</span>{/if}
        {#if ev.status}<span class="status status-{ev.status}">{ev.status}</span>{/if}
      </div>
    {:else if ev.type === "tool_call_update"}
      <div class="tool update">
        <span class="badge">tool ·</span>
        <span class="dim">{ev.id}</span>
        {#if ev.status}<span class="status status-{ev.status}">{ev.status}</span>{/if}
        {#if ev.content}<pre class="content">{JSON.stringify(ev.content, null, 2)}</pre>{/if}
      </div>
    {:else if ev.type === "plan"}
      <div class="plan">
        <span class="badge">plan</span>
        <ul>
          {#each ev.entries as entry, j (j)}
            <li>
              {#if entry.status}<span class="status status-{entry.status}">{entry.status}</span>{/if}
              {entry.content}
            </li>
          {/each}
        </ul>
      </div>
    {:else}
      <pre class="unknown">{JSON.stringify(ev.raw, null, 2)}</pre>
    {/if}
  {/each}
  {#if events.length === 0}
    <p class="empty">No activity yet. Send a prompt to begin.</p>
  {/if}
</div>

<style>
  .stream {
    display: flex;
    flex-direction: column;
    gap: 0.5rem;
    padding: 0.75rem;
    overflow-y: auto;
    flex: 1;
  }
  .msg {
    white-space: pre-wrap;
    line-height: 1.45;
    padding: 0.4rem 0.6rem;
    border-radius: 8px;
  }
  .who {
    display: inline-block;
    font-size: 0.7rem;
    text-transform: uppercase;
    letter-spacing: 0.05em;
    opacity: 0.6;
    margin-right: 0.5rem;
  }
  .user { background: #1e293b; }
  .agent { background: #15212f; }
  .thought { opacity: 0.65; font-style: italic; }
  .tool {
    font-family: ui-monospace, monospace;
    font-size: 0.85rem;
    padding: 0.4rem 0.6rem;
    background: #0f1b15;
    border-left: 3px solid #2f7d52;
    border-radius: 4px;
  }
  .badge {
    color: #5fd99a;
    font-weight: 600;
    margin-right: 0.4rem;
  }
  .tool-title { font-weight: 600; }
  .dim { opacity: 0.6; }
  .status {
    margin-left: 0.4rem;
    font-size: 0.72rem;
    padding: 0.05rem 0.4rem;
    border-radius: 999px;
    background: #334155;
  }
  .status-completed { background: #14532d; color: #86efac; }
  .status-failed { background: #4c1d1d; color: #fca5a5; }
  .status-in_progress { background: #1e3a5f; color: #93c5fd; }
  .content {
    margin: 0.4rem 0 0;
    max-height: 12rem;
    overflow: auto;
    background: #0b0f0d;
    padding: 0.4rem;
    border-radius: 4px;
  }
  .plan { padding: 0.4rem 0.6rem; background: #1a1726; border-radius: 4px; }
  .plan ul { margin: 0.3rem 0 0; padding-left: 1.2rem; }
  .unknown {
    font-size: 0.75rem;
    opacity: 0.7;
    background: #1a1a1a;
    padding: 0.4rem;
    border-radius: 4px;
    overflow: auto;
  }
  .empty { opacity: 0.45; text-align: center; margin-top: 2rem; }
</style>
