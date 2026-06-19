<script lang="ts">
  import type { PermissionOption } from "$lib/api";

  let {
    toolTitle,
    options,
    onresolve,
  }: {
    toolTitle: string | null;
    options: PermissionOption[];
    /** optionId === null cancels the request. */
    onresolve: (optionId: string | null) => void;
  } = $props();

  function optionClass(kind: string | null): string {
    if (!kind) return "neutral";
    if (kind.includes("allow")) return "allow";
    if (kind.includes("reject") || kind.includes("deny")) return "deny";
    return "neutral";
  }
</script>

<div class="overlay">
  <div class="dialog" role="dialog" aria-modal="true">
    <h2>Approval required</h2>
    <p class="tool">{toolTitle ?? "The agent wants to perform an action."}</p>
    <div class="options">
      {#each options as opt (opt.option_id)}
        <button class={optionClass(opt.kind)} onclick={() => onresolve(opt.option_id)}>
          {opt.name}
        </button>
      {/each}
      <button class="cancel" onclick={() => onresolve(null)}>Cancel</button>
    </div>
  </div>
</div>

<style>
  .overlay {
    position: fixed;
    inset: 0;
    background: rgba(0, 0, 0, 0.6);
    display: flex;
    align-items: center;
    justify-content: center;
    z-index: 50;
  }
  .dialog {
    background: #15191f;
    border: 1px solid #2b333d;
    border-radius: 12px;
    padding: 1.25rem 1.5rem;
    max-width: 30rem;
    width: 90%;
    box-shadow: 0 20px 60px rgba(0, 0, 0, 0.5);
  }
  h2 { margin: 0 0 0.5rem; font-size: 1.05rem; }
  .tool {
    font-family: ui-monospace, monospace;
    background: #0c0f13;
    padding: 0.5rem 0.7rem;
    border-radius: 6px;
    word-break: break-word;
  }
  .options {
    display: flex;
    flex-wrap: wrap;
    gap: 0.5rem;
    margin-top: 1rem;
  }
  button {
    border: 1px solid #3a444f;
    background: #232b34;
    color: inherit;
    padding: 0.45rem 0.9rem;
    border-radius: 8px;
    cursor: pointer;
    font-size: 0.9rem;
  }
  button:hover { border-color: #5b6776; }
  .allow { background: #14532d; border-color: #1c6b3a; }
  .deny { background: #4c1d1d; border-color: #6b2a2a; }
  .cancel { margin-left: auto; opacity: 0.8; }
</style>
