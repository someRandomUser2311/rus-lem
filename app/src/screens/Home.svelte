<script lang="ts">
  // The course path: chapters as units, every lesson, question set, problem, review,
  // project and milestone as a node, with streak, XP and the daily goal alongside.
  import type { ChapterState } from '../lib/bindings/ChapterState'
  import type { PathNode } from '../lib/bindings/PathNode'
  import PathIcon from '../components/PathIcon.svelte'
  import { app } from '../lib/state.svelte'
  import Md from '../components/Md.svelte'
  import { plain } from '../lib/md'

  const kindName: Record<string, string> = { lesson: 'Lesson', questions: 'Questions', practice: 'Practice', problem: 'Problem', review: 'Review', project: 'Chapter project', milestone: 'Library milestone' }

  function href(ch: ChapterState, n: PathNode): string {
    switch (n.kind) {
      case 'lesson': return `#/lesson/${ch.id}/${n.id}`
      case 'questions':
      case 'practice': return `#/questions/${ch.id}/${n.id}`
      case 'review': return `#/review/${ch.id}`
      default: return `#/problem/${ch.id}/${n.id}`
    }
  }

  const stats = $derived(app.info?.stats)
  const firstOpen = $derived.by(() => {
    for (const ch of app.path) for (const n of ch.nodes) if (n.state.state === 'open') return { ch, n }
    return null
  })
  const current = $derived(app.path.find(c => c.state.state === 'open') ?? app.path[0])
  const offsets = [0, 45, 70, 45, 0, -45, -70, -45]
  // Labels sit in a column to the right of the widest zigzag, so they never cover a node.
  const labelGap = (off: number) => Math.max(...offsets) - off + 26
  let hovered = $state<string | null>(null)

  $effect(() => {
    // Scroll to the learner's next node on arrival.
    if (!firstOpen) return
    queueMicrotask(() => document.getElementById(`node-${firstOpen.n.id}`)?.scrollIntoView({ block: 'center' }))
  })
</script>

<div class="home">
  <main class="path scroll">
    {#each app.path as ch (ch.num)}
      <section class="unit" class:locked={ch.state.state === 'locked'}>
        <div class="unit-head" class:soon={!ch.available}>
          <div>
            <div class="unit-eyebrow">Part {ch.num <= 6 ? 'I' : ch.num <= 11 ? 'II' : ch.num <= 16 ? 'III' : 'IV'} · Chapter {ch.num}</div>
            <div class="unit-title"><Md inline src={ch.title} /></div>
          </div>
          {#if !ch.available}
            <span class="soon-tag">Coming soon</span>
          {:else if ch.state.state === 'locked'}
            <span class="soon-tag">🔒 {ch.state.state === 'locked' ? ch.state.reason : ''}</span>
          {:else}
            <div class="unit-goal">
              <div class="goal-text">{ch.accepted}/{ch.needed} problems{ch.needs_hard ? ' incl. a Hard' : ''} to open Chapter {ch.num + 1}</div>
              <div class="progress"><div style="width:{Math.min(100, (ch.accepted / Math.max(1, ch.needed)) * 100)}%"></div></div>
            </div>
          {/if}
        </div>

        {#if ch.available}
          <div class="nodes">
            {#each ch.nodes as n, i (n.id)}
              {@const st = n.state.state}
              {@const off = offsets[i % offsets.length]}
              <div class="node-row" class:hover={hovered === n.id} class:next={firstOpen?.n.id === n.id} style="transform: translateX({off}px)">
                {#if st === 'locked'}
                  <div class="node locked" id="node-{n.id}" title={n.state.state === 'locked' ? n.state.reason : ''} onmouseenter={() => (hovered = n.id)} onmouseleave={() => (hovered = null)} role="img" aria-label="{plain(n.title)} (locked)">
                    <PathIcon kind={n.kind} />
                  </div>
                {:else}
                  <a class="node {n.kind} {st}" class:next={firstOpen?.n.id === n.id} id="node-{n.id}" href={href(ch, n)} onmouseenter={() => (hovered = n.id)} onmouseleave={() => (hovered = null)} aria-label={plain(n.title)}>
                    <PathIcon kind={st === 'done' ? 'done' : n.kind} size={st === 'done' ? 34 : 30} />
                  </a>
                {/if}
                <div class="node-label" style="left: calc(100% + {labelGap(off)}px)" class:show={hovered === n.id || firstOpen?.n.id === n.id}>
                  <div class="nl-kind">{kindName[n.kind]}{n.level ? ` · ${n.level}` : ''}{n.xp ? ` · ${n.xp} XP` : ''}</div>
                  <div class="nl-title"><Md inline src={n.kind === 'lesson' || n.kind === 'problem' || n.kind === 'project' || n.kind === 'milestone' ? `${n.id} · ${n.title}` : n.title} /></div>
                  {#if n.state.state === 'locked'}<div class="nl-why">{n.state.reason}</div>{/if}
                  {#if firstOpen?.n.id === n.id}<a class="btn primary small start" href={href(ch, n)}>Start</a>{/if}
                </div>
              </div>
            {/each}
          </div>
        {/if}
      </section>
    {/each}
  </main>

  <aside class="rail scroll">
    {#if stats}
      <div class="card stat-row">
        <div class="pill streak" title="Days in a row with 20 XP or more">🔥 {stats.streak}</div>
        <div class="pill xp">⚡ {stats.xp} XP</div>
        <div class="pill lvl">Level {stats.level}</div>
      </div>
      <div class="card">
        <div class="card-title">Daily goal</div>
        <div class="goal-row"><span>{Math.min(stats.today_xp, stats.daily_goal)} / {stats.daily_goal} XP today</span>{#if stats.today_xp >= stats.daily_goal}<span class="done">Done ✓</span>{/if}</div>
        <div class="progress"><div style="width:{Math.min(100, (stats.today_xp / Math.max(1, stats.daily_goal)) * 100)}%; background: var(--gold)"></div></div>
        <div class="muted small">A day counts towards your streak at 20 XP.</div>
      </div>
      <div class="card">
        <div class="card-title">Level {stats.level}</div>
        <div class="progress"><div style="width:{((stats.xp - stats.level_floor) / Math.max(1, stats.level_next - stats.level_floor)) * 100}%; background: var(--rust)"></div></div>
        <div class="muted small">{stats.level_next - stats.xp} XP to level {stats.level + 1}</div>
      </div>
    {/if}
    {#if current && current.available}
      <div class="card">
        <div class="card-title">Chapter {current.num}: <Md inline src={current.title} /></div>
        <p class="small">To open the next chapter: finish every lesson and get <strong>{current.needed}</strong> problems Accepted{current.needs_hard ? ', including a Hard one' : ''}. Questions and projects give XP but aren't required.</p>
        {#if current.num > 1}<a class="btn ghost small" href="#/review/{current.id}">Review earlier chapters</a>{/if}
      </div>
    {/if}
    {#if app.info && !app.info.toolchain.found}
      <div class="card warn">
        <div class="card-title">Rust isn't installed</div>
        <p class="small">You can read lessons and answer questions, but running code needs Rust. <a href="#/settings">Set it up</a>.</p>
      </div>
    {/if}
  </aside>
</div>

<style>
  .home { height: 100%; display: grid; grid-template-columns: 1fr 340px; }
  .path { padding: 28px 24px 120px; container-type: inline-size; }
  .unit { max-width: 640px; margin: 0 auto 36px; }
  .unit-head { display: flex; justify-content: space-between; align-items: center; gap: 16px; padding: 18px 22px; border-radius: 18px; background: var(--rust); color: #fff; box-shadow: 0 5px 0 var(--rust-dark); }
  .unit.locked .unit-head, .unit-head.soon { background: var(--surface-2); color: var(--muted); box-shadow: 0 5px 0 var(--line-2); }
  .unit-eyebrow { font-size: 12px; font-weight: 800; letter-spacing: .08em; text-transform: uppercase; opacity: .85; }
  .unit-title { font-size: 22px; font-weight: 800; }
  .soon-tag { font-size: 13px; font-weight: 700; max-width: 260px; text-align: right; }
  .unit-goal { width: 220px; }
  .goal-text { font-size: 12px; font-weight: 700; margin-bottom: 6px; text-align: right; }
  .unit-goal .progress { background: rgba(255, 255, 255, .3); height: 10px; }
  .unit-goal .progress > div { background: #fff; }
  .nodes { display: flex; flex-direction: column; align-items: center; gap: 18px; padding: 30px 250px 10px 0; }
  .node-row { position: relative; display: flex; align-items: center; }
  .node-row.next { z-index: 2; }
  .node-row.hover { z-index: 3; }
  .node { width: 68px; height: 68px; border-radius: 50%; display: flex; align-items: center; justify-content: center; font-size: 26px; text-decoration: none; background: var(--rust); box-shadow: 0 6px 0 var(--rust-dark); color: #fff; transition: transform .1s ease; }
  .node:hover { transform: translateY(-2px); }
  .node.open.questions, .node.open.practice { background: var(--blue); box-shadow: 0 6px 0 var(--blue-dark); }
  .node.open.problem { background: var(--green); box-shadow: 0 6px 0 var(--green-dark); }
  .node.open.project, .node.open.milestone { background: var(--purple); box-shadow: 0 6px 0 #4B3290; width: 80px; height: 80px; }
  .node.open.review { background: var(--gold); box-shadow: 0 6px 0 var(--gold-dark); }
  .node.done { background: var(--gold); box-shadow: 0 6px 0 var(--gold-dark); font-weight: 900; }
  .node.locked { background: var(--surface-2); box-shadow: 0 6px 0 var(--line-2); color: var(--muted); cursor: not-allowed; }
  .node.next { outline: 5px solid rgba(194, 70, 27, .25); outline-offset: 6px; animation: bob 2s ease-in-out infinite; }
  @keyframes bob { 50% { transform: translateY(-4px); } }
  .node-label { position: absolute; top: 50%; transform: translateY(-50%); width: 230px; background: var(--surface); border: 1px solid var(--line); border-radius: 12px; padding: 10px 12px; box-shadow: 0 6px 24px rgba(0, 0, 0, .08); opacity: 0; pointer-events: none; transition: opacity .15s ease; z-index: 2; }
  .node-label.show { opacity: 1; pointer-events: auto; }
  .nl-kind { font-size: 11px; font-weight: 800; color: var(--muted); text-transform: uppercase; letter-spacing: .06em; }
  .nl-title { font-weight: 700; margin-top: 2px; }
  .nl-why { font-size: 12px; color: var(--muted); margin-top: 4px; }
  .start { margin-top: 8px; width: 100%; }
  /* Too narrow for a label column: labels drop below their node, on top of the path. */
  @container (max-width: 560px) {
    .nodes { padding-right: 0; }
    .node-label { left: 50% !important; top: calc(100% + 16px); transform: translateX(-50%); }
  }
  @media (max-width: 1100px) { .home { grid-template-columns: 1fr 280px; } }
  .rail { border-left: 1px solid var(--line); padding: 24px 20px; display: flex; flex-direction: column; gap: 16px; background: var(--bg); }
  .stat-row { display: flex; gap: 8px; justify-content: space-between; padding: 14px; }
  .pill { font-weight: 800; font-size: 15px; }
  .streak { color: #E0701E; }
  .xp { color: var(--gold-ink); }
  .lvl { color: var(--blue); }
  .card-title { font-weight: 800; margin-bottom: 10px; }
  .goal-row { display: flex; justify-content: space-between; font-size: 14px; font-weight: 700; margin-bottom: 8px; }
  .goal-row .done { color: var(--green); }
  .small { font-size: 13px; margin: 8px 0 0; line-height: 1.5; }
  .card .progress { height: 12px; }
  .warn { border-color: var(--red); }
</style>
