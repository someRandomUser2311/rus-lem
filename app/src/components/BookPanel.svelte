<script lang="ts">
  // "Read more": the matching Rust Book chapter, bundled for offline reading.
  import { api, errorText } from '../lib/api'
  import { bookMd } from '../lib/md'

  let { files, onclose }: { files: string[]; onclose: () => void } = $props()
  // svelte-ignore state_referenced_locally
  let current = $state(files[0])
  let html = $state('')
  let error = $state<string | null>(null)

  // Links between book chapters open in the panel; nothing navigates the app away.
  function onclick(e: MouseEvent) {
    const a = (e.target as HTMLElement).closest('a')
    if (!a) return
    const href = a.getAttribute('href') ?? ''
    e.preventDefault()
    const m = href.match(/^(?:\.\/)?(ch[\w-]+)\.html/)
    if (m) current = `${m[1]}.md`
    else if (href.startsWith('#')) document.getElementById(href.slice(1))?.scrollIntoView()
  }

  $effect(() => {
    const f = current
    error = null
    html = ''
    api.readBook(f).then(t => (html = bookMd(t))).catch(e => (error = errorText(e)))
  })
</script>

<div class="backdrop" role="presentation" onclick={onclose}></div>
<aside class="drawer" aria-label="The Rust Programming Language">
  <header>
    <div>
      <div class="eyebrow">Read more</div>
      <div class="title">The Rust Programming Language</div>
    </div>
    <button class="btn ghost small" onclick={onclose}>Close</button>
  </header>
  {#if files.length > 1}
    <nav>
      {#each files as f}
        <button class:on={f === current} onclick={() => (current = f)}>{f.replace(/\.md$/, '').replace(/^ch\d+-\d+-/, '').replace(/-/g, ' ')}</button>
      {/each}
    </nav>
  {/if}
  <!-- svelte-ignore a11y_click_events_have_key_events, a11y_no_static_element_interactions -->
  <div class="body prose scroll" {onclick}>
    {#if error}<p class="muted">{error}</p>{:else if !html}<p class="muted">Loading…</p>{:else}{@html html}{/if}
    <p class="attrib">From <em>The Rust Programming Language</em> by Steve Klabnik, Carol Nichols and Chris Krycho, with contributions from the Rust Community. MIT / Apache-2.0.</p>
  </div>
</aside>

<style>
  .backdrop { position: fixed; inset: 0; background: rgba(0, 0, 0, .25); z-index: 40; }
  .drawer { position: fixed; top: 0; right: 0; bottom: 0; width: min(720px, 92vw); background: var(--surface); z-index: 41; display: flex; flex-direction: column; box-shadow: -12px 0 40px rgba(0, 0, 0, .18); animation: slide .25s ease; }
  @keyframes slide { from { transform: translateX(40px); opacity: 0; } to { transform: none; opacity: 1; } }
  header { display: flex; justify-content: space-between; align-items: center; padding: 18px 24px; border-bottom: 1px solid var(--line); }
  .title { font-weight: 800; font-size: 18px; }
  nav { display: flex; gap: 6px; padding: 10px 24px; border-bottom: 1px solid var(--line); flex-wrap: wrap; }
  nav button { border: 0; background: var(--surface-2); border-radius: 8px; padding: 6px 10px; font-size: 13px; font-weight: 700; cursor: pointer; text-transform: capitalize; }
  nav button.on { background: var(--rust); color: #fff; }
  .body { padding: 20px 28px 40px; flex: 1; font-size: 16px; }
  .body :global(pre.block) { background: var(--code-bg); color: var(--code-ink); padding: 14px 18px; border-radius: 12px; overflow-x: auto; font-size: 14px; line-height: 1.6; }
  .body :global(pre.block code) { background: none; padding: 0; color: inherit; }
  .attrib { margin-top: 28px !important; font-size: 13px; color: var(--muted); }
</style>
