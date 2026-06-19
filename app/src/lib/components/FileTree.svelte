<script lang="ts">
  import type { DirEntry } from "$lib/api";

  let {
    cwd,
    entries,
    onnavigate,
  }: {
    cwd: string;
    entries: DirEntry[];
    onnavigate: (path: string) => void;
  } = $props();

  function parentOf(path: string): string {
    const trimmed = path.replace(/\/+$/, "");
    const idx = trimmed.lastIndexOf("/");
    return idx <= 0 ? "/" : trimmed.slice(0, idx);
  }
</script>

<div class="tree">
  <div class="path" title={cwd}>{cwd}</div>
  <button class="entry up" onclick={() => onnavigate(parentOf(cwd))}>../</button>
  {#each entries as entry (entry.path)}
    {#if entry.is_dir}
      <button class="entry dir" onclick={() => onnavigate(entry.path)}>📁 {entry.name}</button>
    {:else}
      <div class="entry file">📄 {entry.name}</div>
    {/if}
  {/each}
  {#if entries.length === 0}
    <p class="empty">empty</p>
  {/if}
</div>

<style>
  .tree {
    display: flex;
    flex-direction: column;
    font-size: 0.82rem;
    overflow-y: auto;
    height: 100%;
  }
  .path {
    padding: 0.5rem 0.6rem;
    font-family: ui-monospace, monospace;
    opacity: 0.6;
    border-bottom: 1px solid #222a33;
    overflow: hidden;
    text-overflow: ellipsis;
    white-space: nowrap;
  }
  .entry {
    text-align: left;
    background: none;
    border: none;
    color: inherit;
    padding: 0.25rem 0.6rem;
    font: inherit;
  }
  button.entry { cursor: pointer; }
  button.entry:hover { background: #1c242d; }
  .dir { color: #93c5fd; }
  .file { opacity: 0.85; }
  .up { opacity: 0.6; }
  .empty { opacity: 0.4; padding: 0.6rem; }
</style>
