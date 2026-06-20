<script lang="ts">
  import EventStream from "./EventStream.svelte";
  import ApprovalDialog from "./ApprovalDialog.svelte";
  import FileTree from "./FileTree.svelte";
  import { sessions, type SessionState } from "$lib/sessions.svelte";

  let { session }: { session: SessionState } = $props();
  let promptText = $state("");

  function send() {
    const text = promptText.trim();
    if (!text) return;
    promptText = "";
    sessions.prompt(session, text);
  }

  function onKey(e: KeyboardEvent) {
    if (e.key === "Enter" && (e.metaKey || e.ctrlKey)) {
      e.preventDefault();
      send();
    }
  }
</script>

<div class="win">
  <aside class="tree-col">
    <FileTree
      cwd={session.cwd}
      entries={session.entries}
      onnavigate={(p) => sessions.navigate(session, p)}
    />
  </aside>

  <div class="main">
    <EventStream events={session.events} />
    <div class="composer">
      <textarea
        bind:value={promptText}
        onkeydown={onKey}
        placeholder="Prompt the agent…  (Ctrl/Cmd+Enter to send)"
      ></textarea>
      <button onclick={send} disabled={!promptText.trim()}>Send</button>
    </div>
  </div>

  {#if session.pending}
    <ApprovalDialog
      toolTitle={session.pending.toolTitle}
      options={session.pending.options}
      onresolve={(opt) => sessions.resolve(session, opt)}
    />
  {/if}
</div>

<style>
  .win {
    position: relative;
    display: flex;
    height: 100%;
    background: #0b0e12;
    color: #e7ecf1;
    font-family: Inter, system-ui, Avenir, Helvetica, Arial, sans-serif;
  }
  .tree-col {
    width: 12rem;
    flex: 0 0 12rem;
    border-right: 1px solid #1c232b;
    min-height: 0;
  }
  .main {
    flex: 1;
    display: flex;
    flex-direction: column;
    min-width: 0;
  }
  .composer {
    display: flex;
    gap: 0.5rem;
    padding: 0.5rem;
    border-top: 1px solid #1c232b;
  }
  textarea {
    flex: 1;
    resize: none;
    height: 3rem;
    background: #141a21;
    color: inherit;
    border: 1px solid #28313b;
    border-radius: 8px;
    padding: 0.5rem 0.6rem;
    font-family: inherit;
    font-size: 0.9rem;
  }
  button {
    background: #1a212a;
    color: inherit;
    border: 1px solid #2a333d;
    border-radius: 8px;
    padding: 0.4rem 0.8rem;
    cursor: pointer;
    font-size: 0.85rem;
  }
  button:hover:not(:disabled) {
    border-color: #4a5763;
  }
  button:disabled {
    opacity: 0.5;
    cursor: default;
  }
</style>
