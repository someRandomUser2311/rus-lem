<script lang="ts">
  // Draws visualizer panels: program output (terminal) and boxes-and-arrows scenes.
  import type { PanelState, SceneState } from '../lib/viz'

  let { panels }: { panels: PanelState[] } = $props()

  function nodeClass(n: SceneState['nodes'][number]) {
    const s = n.state
    return [
      'node',
      `k-${n.kind}`,
      s.style ? `s-${s.style}` : '',
      s.lit ? 'lit' : '',
      s.ghost ? 'ghost' : '',
      n.changed ? 'changed' : '',
    ].join(' ')
  }

  /** Row layout: nodes in order with an arrow (and label) between consecutive connected nodes. */
  function rowItems(p: SceneState) {
    const visible = p.nodes.filter(n => !n.state.hidden)
    return visible.map((n, i) => {
      const next = visible[i + 1]
      const edge = next ? p.edges.find(e => e.from === n.id && e.to === next.id && !e.hidden) : undefined
      return { n, edge, nextGhost: next?.state.ghost ?? false }
    })
  }
</script>

<div class="panels">
  {#each panels as p (p.id)}
    {#if p.kind === 'terminal'}
      <div class="term">
        <div class="term-head">{p.title ?? 'Output'}</div>
        <pre class="term-body">{p.text.slice(0, p.text.length - p.fresh.length)}<span class="fresh">{p.fresh}</span>{#if p.cursor}<span class="cursor"></span>{/if}{#if !p.text && !p.cursor}<span class="empty">(nothing printed yet)</span>{/if}</pre>
        {#if p.badge}<div class="term-badge">{p.badge}</div>{/if}
      </div>
    {:else if p.layout === 'row'}
      <div class="scene row">
        {#each rowItems(p) as it (it.n.id)}
          <div class={nodeClass(it.n)}>
            <div class="label">{it.n.label}</div>
            {#if it.n.sub}<div class="sub">{it.n.sub}</div>{/if}
            {#if it.n.state.value != null}<code class="value">{it.n.state.value}</code>{/if}
            {#if it.n.state.badge}<div class="badge">{it.n.state.badge}</div>{/if}
          </div>
          {#if it.edge}
            <div class="edge" class:dim={it.nextGhost}>
              {#if it.edge.label}<span class="edge-label">{it.edge.label}</span>{/if}
              <span class="edge-line"></span>
            </div>
          {/if}
        {/each}
      </div>
    {:else}
      <div class="scene" class:tree={p.layout === 'tree'} class:grid={p.layout === 'grid'}>
        {#each p.nodes.filter(n => !n.state.hidden) as n (n.id)}
          {#if n.kind === 'code'}
            <pre class="node k-code" class:changed={n.changed}>{n.label}</pre>
          {:else}
            <div class={nodeClass(n)} style="margin-left: {p.layout === 'tree' ? n.depth * 26 : 0}px">
              <span class="ico">{n.kind === 'folder' ? '📁' : n.kind === 'file' ? '📄' : ''}</span>
              <span class="label">{n.label}</span>
              {#if n.state.value != null}<code class="value">{n.state.value}</code>{/if}
              {#if n.state.badge}<span class="badge">{n.state.badge}</span>{/if}
              {#if n.state.note}<span class="note">{n.state.note}</span>{/if}
            </div>
          {/if}
        {/each}
      </div>
    {/if}
  {/each}
</div>

<style>
  .panels { display: flex; flex-direction: column; gap: 16px; }
  .term { background: var(--code-bg); border-radius: var(--radius); overflow: hidden; }
  .term-head { padding: 10px 18px; font-size: 12px; font-weight: 700; color: var(--code-head); border-bottom: 1px solid var(--code-line); }
  .term-body { margin: 0; padding: 16px 18px; min-height: 120px; font-family: var(--mono); font-size: 16px; line-height: 1.7; color: var(--code-ink); white-space: pre-wrap; }
  .fresh { background: rgba(242, 182, 50, .35); border-radius: 3px; animation: flash 1s ease forwards; }
  @keyframes flash { from { background: rgba(242, 182, 50, .7); } to { background: rgba(242, 182, 50, .15); } }
  .cursor { display: inline-block; width: 9px; height: 1.1em; vertical-align: text-bottom; background: var(--code-ink); animation: blink 1s steps(1) infinite; }
  @keyframes blink { 50% { opacity: 0; } }
  .empty { color: var(--code-dim); font-style: italic; }
  .term-badge { margin: 0 18px 14px; display: inline-block; font-size: 12px; font-weight: 800; letter-spacing: .06em; text-transform: uppercase; color: var(--code-head); background: var(--code-line); padding: 4px 10px; border-radius: 6px; animation: pop .3s ease; }

  .scene { background: var(--surface); border: 1px solid var(--line); border-radius: var(--radius); padding: 22px; }
  .scene.row { display: flex; align-items: stretch; flex-wrap: wrap; gap: 0; row-gap: 16px; }
  .scene.tree { display: flex; flex-direction: column; gap: 6px; font-family: var(--mono); font-size: 15px; }
  .scene.grid { display: flex; flex-wrap: wrap; gap: 12px; }
  .node { border-radius: 14px; border: 2px solid var(--line-2); background: var(--surface); padding: 12px 16px; transition: all .45s ease; position: relative; }
  .scene.row .node { flex: 1 1 140px; display: flex; flex-direction: column; gap: 6px; min-width: 120px; }
  .scene.tree .node { display: inline-flex; align-items: center; gap: 8px; align-self: flex-start; padding: 5px 10px; border-radius: 10px; }
  .node .label { font-family: var(--mono); font-weight: 700; font-size: 17px; }
  .scene.tree .node .label { font-size: 15px; font-weight: 600; }
  .node .sub { font-size: 13px; color: var(--muted); line-height: 1.4; }
  .node.s-rust { border-color: var(--rust); }
  .node.s-blue { border-color: var(--blue); }
  .node.s-green { border-color: var(--green); }
  .node.s-red { border-color: var(--red); }
  .node.lit.s-rust, .node.lit:not([class*="s-"]) { background: var(--rust-soft); }
  .node.lit.s-blue { background: var(--blue-soft); }
  .node.lit.s-green { background: var(--green-soft); }
  .node.lit { box-shadow: 0 4px 0 var(--line-2); }
  .node.ghost { border-style: dashed; opacity: .45; }
  .node.changed { animation: pop .35s ease; }
  .badge { align-self: flex-start; font-size: 11px; font-weight: 800; letter-spacing: .06em; text-transform: uppercase; border-radius: 6px; padding: 3px 8px; background: var(--ink-2); color: var(--surface); animation: pop .3s ease; font-family: var(--sans); }
  .value { font-family: var(--mono); font-size: 14px; font-weight: 700; color: var(--ink); background: var(--surface-2); border-radius: 6px; padding: 2px 8px; white-space: pre; animation: pop .3s ease; }
  .note { font-family: var(--sans); font-size: 12px; color: var(--muted); }
  .k-code { margin: 6px 0 0; background: var(--code-bg); color: var(--code-ink); border: none; font-size: 13px; line-height: 1.6; padding: 12px 16px; border-radius: 10px; align-self: stretch; }
  .edge { flex: 0 0 90px; display: flex; flex-direction: column; align-items: center; justify-content: center; gap: 4px; transition: opacity .4s ease; }
  .edge.dim { opacity: .35; }
  .edge-label { font-size: 12px; font-weight: 700; color: var(--muted); }
  .edge-line { position: relative; width: 70px; height: 3px; background: var(--rust); }
  .edge-line::after { content: ""; position: absolute; right: -2px; top: -6px; border-top: 7.5px solid transparent; border-bottom: 7.5px solid transparent; border-left: 11px solid var(--rust); }
  .ico { font-size: 14px; }
</style>
