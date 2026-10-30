<script lang="ts">
  // Read-only code with line numbers, per-step highlights, notes and arrows between spans.
  import type { CodeBlock } from '../lib/bindings/CodeBlock'
  import type { CodeLink } from '../lib/bindings/CodeLink'
  import type { CodeNote } from '../lib/bindings/CodeNote'
  import { highlightLines } from '../lib/highlight'
  import { parseSpan } from '../lib/viz'

  let {
    code,
    hl = [],
    err = [],
    notes = [],
    links = [],
    picked = [],
    right = [],
    onpick,
    small = false,
  }: {
    code: CodeBlock
    hl?: number[]
    err?: number[]
    notes?: CodeNote[]
    links?: CodeLink[]
    picked?: number[]
    right?: number[]
    onpick?: (line: number) => void
    small?: boolean
  } = $props()

  const lines = $derived(highlightLines(code.text, code.lang))
  const label = $derived(code.file ?? (code.lang === 'console' ? 'terminal' : code.lang === 'toml' ? 'Cargo.toml' : ''))

  let pre: HTMLPreElement | undefined = $state()
  let charW = $state(9)
  let lineH = $state(28)
  let gutter = $state(48)
  let padTop = $state(14)

  $effect(() => {
    if (!pre) return
    void lines
    const probe = pre.querySelector('.probe') as HTMLElement | null
    const first = pre.querySelector('.line') as HTMLElement | null
    const ln = pre.querySelector('.ln') as HTMLElement | null
    if (probe) charW = probe.getBoundingClientRect().width / 10
    if (first) lineH = first.getBoundingClientRect().height
    if (ln) gutter = ln.getBoundingClientRect().width
    padTop = parseFloat(getComputedStyle(pre).paddingTop) || 14
  })

  const arrows = $derived(
    links
      .map(l => {
        const a = parseSpan(l.from)
        const b = parseSpan(l.to)
        if (!a || !b) return null
        const x1 = gutter + ((a.from + a.to) / 2 - 1) * charW
        const x2 = gutter + ((b.from + b.to) / 2 - 1) * charW
        const y1 = padTop + (a.line - 1) * lineH + 4
        const y2 = padTop + (b.line - 1) * lineH + 4
        return { x1, x2, y1, y2 }
      })
      .filter(Boolean) as { x1: number; x2: number; y1: number; y2: number }[],
  )
</script>

<div class="codebox">
  {#if label}
    <div class="codehead"><span class="file">{label}</span>{#if onpick}<span class="tag-r">Click a line</span>{/if}</div>
  {/if}
  <!-- svelte-ignore a11y_no_noninteractive_tabindex -->
  <pre class="code" class:small bind:this={pre}><span class="probe" aria-hidden="true">0000000000</span>{#each lines as html, i}<div
        class="line"
        class:hl={hl.includes(i + 1)}
        class:err={err.includes(i + 1)}
        class:pick={!!onpick}
        class:picked={picked.includes(i + 1)}
        class:right={right.includes(i + 1)}
        role={onpick ? 'button' : undefined}
        tabindex={onpick ? 0 : undefined}
        onclick={() => onpick?.(i + 1)}
        onkeydown={e => (e.key === 'Enter' || e.key === ' ') && onpick?.(i + 1)}><span class="ln">{i + 1}</span><span>{@html html || ' '}</span>{#each notes.filter(n => n.line === i + 1) as n}<span class="note">{n.text}</span>{/each}</div>{/each}{#if arrows.length}<svg class="links" aria-hidden="true">
        <defs><marker id="ah" viewBox="0 0 10 10" refX="8" refY="5" markerWidth="7" markerHeight="7" orient="auto-start-reverse"><path d="M0,0 L10,5 L0,10 z" fill="#E0AF68" /></marker></defs>
        {#each arrows as a}
          <path d="M{a.x1},{a.y1} C{a.x1},{a.y1 - 22} {a.x2},{a.y2 - 22} {a.x2},{a.y2}" fill="none" stroke="#E0AF68" stroke-width="2" marker-end="url(#ah)" />
        {/each}
      </svg>{/if}</pre>
</div>

<style>
  pre.code.small { font-size: 13px; line-height: 1.7; }
  pre.code { font-size: var(--editor-font); }
  .probe { position: absolute; visibility: hidden; white-space: pre; }
  .links { position: absolute; inset: 0; width: 100%; height: 100%; pointer-events: none; overflow: visible; }
</style>
