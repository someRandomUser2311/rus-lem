<script lang="ts">
  // A package's files as tabs. A step that names a file switches to it; its highlights and
  // error marks apply to that file only. `stacked` shows every file at once instead.
  import type { CodeBlock } from '../lib/bindings/CodeBlock'
  import type { CodeNote } from '../lib/bindings/CodeNote'
  import CodeView from './CodeView.svelte'

  let {
    files,
    file = null,
    hl = [],
    err = [],
    notes = [],
    stacked = false,
  }: {
    files: CodeBlock[]
    file?: string | null
    hl?: number[]
    err?: number[]
    notes?: CodeNote[]
    stacked?: boolean
  } = $props()

  // The file the screen is about: the step's, else the first Rust file after Cargo.toml.
  const preferred = $derived(file ?? files.find(f => f.lang === 'rust')?.file ?? files[0]?.file ?? null)
  let picked: string | null = $state(null)
  $effect(() => {
    // A new step (or screen) moves the view to its file.
    void preferred
    picked = null
  })
  const active = $derived(picked ?? preferred)
  const shown = $derived(files.find(f => f.file === active) ?? files[0])
  const marks = $derived(file && shown?.file === file)
</script>

{#if stacked}
  <div class="stack">
    {#each files as f}<CodeView code={f} small />{/each}
  </div>
{:else if shown}
  <div class="files">
    <div class="tabs" role="tablist">
      {#each files as f}
        <button
          class="tab"
          class:on={f.file === shown.file}
          class:stepfile={!!file && f.file === file}
          role="tab"
          aria-selected={f.file === shown.file}
          onclick={() => (picked = f.file)}
        >{f.file}</button>
      {/each}
    </div>
    <CodeView code={shown} hl={marks || !file ? hl : []} err={marks || !file ? err : []} notes={marks || !file ? notes : []} />
  </div>
{/if}

<style>
  .stack { display: flex; flex-direction: column; gap: 10px; }
  .files { display: flex; flex-direction: column; }
  .tabs { display: flex; flex-wrap: wrap; gap: 4px; margin-bottom: -1px; }
  .tab {
    font: 12px/1 var(--mono, ui-monospace, monospace);
    padding: 7px 10px;
    border: 1px solid var(--line, #2a2f3a);
    border-bottom: none;
    border-radius: 8px 8px 0 0;
    background: transparent;
    color: var(--muted, #9aa3b2);
    cursor: pointer;
  }
  .tab.on { background: var(--code-bg, #1a1d24); color: var(--text, #e6e6e6); }
  .tab.stepfile:not(.on) { color: var(--accent, #E0AF68); }
</style>
