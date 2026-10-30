<script lang="ts">
  // A real rustc diagnostic with numbered annotations; each mark is highlighted in the message.
  import type { Annotation } from '../lib/bindings/Annotation'
  import Md from './Md.svelte'

  let { diagnostic, annotations }: { diagnostic: string; annotations: Annotation[] } = $props()
  const esc = (s: string) => s.replace(/&/g, '&amp;').replace(/</g, '&lt;').replace(/>/g, '&gt;')

  const html = $derived.by(() => {
    let out = esc(diagnostic)
    annotations.forEach((a, i) => {
      const m = esc(a.mark)
      const at = out.indexOf(m)
      if (m && at >= 0) out = out.slice(0, at) + `<mark>${m}</mark><span class="n">${i + 1}</span>` + out.slice(at + m.length)
    })
    return out
  })
  const title = $derived(diagnostic.split('\n')[0])
</script>

<div class="rustc-error">
  <div class="head"><span class="dot">!</span>{title.startsWith('error') ? 'Compiler error' : 'Compiler message'}</div>
  <pre>{@html html}</pre>
  {#if annotations.length}
    <div class="annots">
      {#each annotations as a, i}
        <div class="annot"><span class="num">{i + 1}</span><Md src={a.note_md} inline /></div>
      {/each}
    </div>
  {/if}
</div>

<style>
  .rustc-error { background: var(--surface); border: 2px solid var(--red); border-radius: var(--radius); overflow: hidden; }
  .head { display: flex; align-items: center; gap: 10px; padding: 12px 18px; background: var(--red-soft); color: var(--red-dark); font-weight: 800; font-size: 15px; }
  .dot { width: 22px; height: 22px; border-radius: 11px; background: var(--red); color: #fff; display: inline-flex; align-items: center; justify-content: center; font-size: 13px; }
  pre { margin: 0; padding: 16px 18px; font-family: var(--mono); font-size: 13px; line-height: 1.6; color: var(--ink); overflow-x: auto; white-space: pre; }
  /* The highlight is a shadow, not padding, so marked text keeps its column. The number
     follows the mark in the text flow instead of floating over the next characters. */
  pre :global(mark) { background: #FFE2A8; color: #16181D; border-radius: 3px; box-shadow: 0 0 0 2px #FFE2A8; }
  pre :global(.n) { display: inline-block; margin-left: 6px; width: 17px; height: 17px; border-radius: 9px; background: var(--rust); color: #fff; font: 800 11px/17px var(--sans); text-align: center; vertical-align: 1px; }
  .annots { display: flex; flex-direction: column; gap: 10px; padding: 0 18px 16px; }
  .annot { display: flex; gap: 10px; align-items: flex-start; font-size: 15px; line-height: 1.5; color: var(--ink-2); }
  .num { flex: 0 0 26px; height: 26px; border-radius: 13px; background: var(--rust); color: #fff; font-size: 13px; font-weight: 800; display: flex; align-items: center; justify-content: center; }
</style>
