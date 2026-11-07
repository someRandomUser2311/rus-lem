<script lang="ts">
  // Home: a "continue" hero for the learner's next item, the course as a list of chapter
  // cards (the current one expanded into a timeline of its items), and a rail with the
  // streak week, daily goal, level and the current chapter's unlock checklist.
  import type { ChapterState } from '../lib/bindings/ChapterState'
  import type { PathNode } from '../lib/bindings/PathNode'
  import PathIcon from '../components/PathIcon.svelte'
  import { app } from '../lib/state.svelte'
  import Md from '../components/Md.svelte'
  import { plain } from '../lib/md'

  const kindName: Record<string, string> = { lesson: 'Lesson', questions: 'Questions', practice: 'Practice set', problem: 'Problem', review: 'Review', project: 'Chapter project', milestone: 'Milestone' }
  const partOf = (n: number) => (n <= 6 ? 'I' : n <= 11 ? 'II' : n <= 16 ? 'III' : 'IV')

  function href(ch: ChapterState, n: PathNode): string {
    switch (n.kind) {
      case 'lesson': return `#/lesson/${ch.id}/${n.id}`
      case 'questions':
      case 'practice': return `#/questions/${ch.id}/${n.id}`
      case 'review': return `#/review/${ch.id}`
      default: return `#/problem/${ch.id}/${n.id}`
    }
  }
  const label = (n: PathNode) => (['lesson', 'problem', 'project', 'milestone'].includes(n.kind) ? `${n.id} · ${n.title}` : n.title)
  const doneCount = (ch: ChapterState) => ch.nodes.filter(n => n.state.state === 'done').length
  const pct = (a: number, b: number) => Math.min(100, Math.round((a / Math.max(1, b)) * 100))

  const stats = $derived(app.info?.stats)
  const next = $derived(app.resumeNode())
  const current = $derived(next?.ch ?? app.path.find(c => c.state.state === 'open') ?? app.path[0])
  const started = $derived((stats?.xp ?? 0) > 0)
  const bundled = $derived(app.path.filter(c => c.available))
  const passedCount = $derived(app.path.filter(c => c.passed).length)

  // Expanded chapters: the current one by default; the learner can open others.
  let open = $state<Set<number>>(new Set())
  $effect(() => { if (current && open.size === 0) open = new Set([current.num]) })
  function toggle(num: number) {
    const s = new Set(open)
    if (s.has(num)) s.delete(num); else s.add(num)
    open = s
  }

  // Last 7 days of the calendar, oldest first, for the streak week.
  const week = $derived((stats?.calendar ?? []).slice(-7).map(([d, xp]) => ({ day: new Date(d + 'T00:00').toLocaleDateString(undefined, { weekday: 'narrow' }), xp, hit: xp >= 20 })))

  const lessons = $derived(current ? current.nodes.filter(n => n.kind === 'lesson') : [])
  const lessonsDone = $derived(lessons.filter(n => n.state.state === 'done').length)

  // Daily goal ring geometry.
  const R = 34, C = 2 * Math.PI * R
  const goalPct = $derived(stats ? Math.min(1, stats.today_xp / Math.max(1, stats.daily_goal)) : 0)
</script>

<div class="home">
  <main class="main scroll">
    <div class="wrap">
      <header class="top">
        <div>
          <div class="eyebrow">{app.info?.course_title}</div>
          <h1>{started ? 'Welcome back' : 'Let’s start learning Rust'}</h1>
        </div>
        <div class="course-meter" title="Chapters passed">
          <span><strong>{passedCount}</strong> of {app.path.length} chapters</span>
          <div class="bar"><div style="width:{pct(passedCount, app.path.length)}%"></div></div>
        </div>
      </header>

      {#if next}
        {@const ch = next.ch}
        {@const n = next.n}
        <section class="hero">
          <div class="hero-mark kind-{n.kind}"><PathIcon kind={n.kind} size={34} /></div>
          <div class="hero-body">
            <div class="hero-eyebrow">{started ? 'Continue' : 'Start here'} · Chapter {ch.num}, <Md inline src={ch.title} /></div>
            <h2><Md inline src={label(n)} /></h2>
            <div class="hero-meta">
              <span class="chip">{kindName[n.kind]}</span>
              {#if n.level && n.level !== kindName[n.kind]}<span class="chip lvl-{n.level.toLowerCase()}">{n.level}</span>{/if}
              {#if n.xp}<span class="chip xp">+{n.xp} XP</span>{/if}
            </div>
            <div class="hero-progress">
              <div class="bar"><div style="width:{pct(doneCount(ch), ch.nodes.length)}%"></div></div>
              <span>{doneCount(ch)}/{ch.nodes.length} in this chapter</span>
            </div>
          </div>
          <a class="btn primary hero-btn" href={href(ch, n)}>{started ? 'Resume' : 'Start'}<svg width="18" height="18" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2.6" stroke-linecap="round" stroke-linejoin="round"><path d="M5 12h14M13 6l6 6-6 6" /></svg></a>
        </section>
      {/if}

      <div class="section-head">
        <h3>Course path</h3>
        <span class="muted">{bundled.length} chapters{app.path.length > bundled.length ? ` · ${app.path.length - bundled.length} coming soon` : ''}</span>
      </div>

      <div class="chapters">
        {#each app.path as ch (ch.num)}
          {@const locked = ch.state.state === 'locked'}
          {@const isOpen = ch.available && open.has(ch.num)}
          {@const d = doneCount(ch)}
          <section class="chapter" class:locked class:soon={!ch.available} class:open={isOpen} class:current={ch.num === current?.num}>
            <button class="ch-head" onclick={() => ch.available && toggle(ch.num)} disabled={!ch.available} aria-expanded={isOpen}>
              <div class="ch-num" class:passed={ch.passed}>
                {#if ch.passed}<PathIcon kind="done" size={20} />{:else if locked || !ch.available}<svg width="18" height="18" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2.4" stroke-linecap="round"><rect x="5" y="11" width="14" height="10" rx="2" /><path d="M8 11V8a4 4 0 0 1 8 0v3" /></svg>{:else}{ch.num}{/if}
              </div>
              <div class="ch-text">
                <div class="ch-eyebrow">Part {partOf(ch.num)} · Chapter {ch.num}</div>
                <div class="ch-title"><Md inline src={ch.title} /></div>
                {#if locked && ch.available && ch.state.state === 'locked'}<div class="ch-why">{ch.state.reason}</div>{/if}
              </div>
              <div class="ch-side">
                {#if !ch.available}
                  <span class="tag">Coming soon</span>
                {:else}
                  <div class="ch-prog">
                    <span>{d}/{ch.nodes.length}</span>
                    <div class="bar small"><div style="width:{pct(d, ch.nodes.length)}%"></div></div>
                  </div>
                  <svg class="chev" width="18" height="18" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2.4" stroke-linecap="round" stroke-linejoin="round"><path d="M6 9l6 6 6-6" /></svg>
                {/if}
              </div>
            </button>

            {#if isOpen}
              <ol class="items">
                {#each ch.nodes as n (n.id)}
                  {@const st = n.state.state}
                  {@const isNext = next?.n.id === n.id}
                  <li class="item {st} kind-{n.kind}" class:next={isNext}>
                    <span class="dot"><PathIcon kind={st === 'done' ? 'done' : n.kind} size={16} /></span>
                    {#if st === 'locked'}
                      <div class="row" title={n.state.state === 'locked' ? n.state.reason : ''}>
                        <span class="it-title"><Md inline src={label(n)} /></span>
                        <span class="it-meta">{kindName[n.kind]}{n.level && n.level !== kindName[n.kind] ? ` · ${n.level}` : ''}</span>
                        <span class="it-end lock">Locked</span>
                      </div>
                    {:else}
                      <a class="row" href={href(ch, n)} aria-label={plain(n.title)}>
                        <span class="it-title"><Md inline src={label(n)} /></span>
                        <span class="it-meta">{kindName[n.kind]}{n.level && n.level !== kindName[n.kind] ? ` · ${n.level}` : ''}</span>
                        {#if isNext}<span class="it-end go">{started ? 'Up next' : 'Start'}</span>
                        {:else if st === 'done'}<span class="it-end done">Done</span>
                        {:else if n.xp}<span class="it-end">+{n.xp} XP</span>{/if}
                      </a>
                    {/if}
                  </li>
                {/each}
              </ol>
            {/if}
          </section>
        {/each}
      </div>
    </div>
  </main>

  <aside class="rail scroll">
    {#if stats}
      <div class="card streak-card">
        <div class="streak-top">
          <div>
            <div class="big">{stats.streak} <span>day{stats.streak === 1 ? '' : 's'}</span></div>
            <div class="muted small">Streak · best {stats.best_streak}</div>
          </div>
          <svg class="flame" class:lit={stats.streak > 0} width="34" height="34" viewBox="0 0 24 24" fill="currentColor"><path d="M12 2c.6 3.2 2.4 4.9 4 6.6 1.6 1.7 3 3.5 3 6.4A7 7 0 0 1 5 15c0-2.3 1-4 2.3-5.3.3 1.7 1 2.8 2.2 3.4C9 9 10.4 5.4 12 2z" /></svg>
        </div>
        <div class="week">
          {#each week as w}
            <div class="wd" class:hit={w.hit} class:some={w.xp > 0 && !w.hit} title="{w.xp} XP">
              <span class="wd-dot">{#if w.hit}<PathIcon kind="done" size={12} />{/if}</span>
              <span class="wd-l">{w.day}</span>
            </div>
          {/each}
        </div>
      </div>

      <div class="card goal-card">
        <svg class="ring" width="84" height="84" viewBox="0 0 84 84">
          <circle cx="42" cy="42" r={R} class="ring-track" />
          <circle cx="42" cy="42" r={R} class="ring-fill" stroke-dasharray={C} stroke-dashoffset={C * (1 - goalPct)} transform="rotate(-90 42 42)" />
          <text x="42" y="47" text-anchor="middle">{Math.round(goalPct * 100)}%</text>
        </svg>
        <div>
          <div class="card-title">Daily goal</div>
          <div class="goal-num"><strong>{Math.min(stats.today_xp, stats.daily_goal)}</strong> / {stats.daily_goal} XP</div>
          <div class="muted small">{stats.today_xp >= stats.daily_goal ? 'Goal met for today.' : '20 XP keeps your streak going.'}</div>
        </div>
      </div>

      <div class="card">
        <div class="lvl-row">
          <span class="lvl-badge">{stats.level}</span>
          <div class="lvl-text">
            <div class="card-title">Level {stats.level}</div>
            <div class="muted small">{stats.xp} XP total · {stats.level_next - stats.xp} to level {stats.level + 1}</div>
          </div>
        </div>
        <div class="bar gold"><div style="width:{pct(stats.xp - stats.level_floor, stats.level_next - stats.level_floor)}%"></div></div>
      </div>
    {/if}

    {#if current && current.available && current.num < app.path.length}
      <div class="card">
        <div class="card-title">Unlock Chapter {current.num + 1}</div>
        <ul class="checks">
          <li class:ok={lessonsDone === lessons.length}><span class="ck"></span>Finish every lesson <em>{lessonsDone}/{lessons.length}</em></li>
          <li class:ok={current.accepted >= current.needed}><span class="ck"></span>Get problems Accepted <em>{Math.min(current.accepted, current.needed)}/{current.needed}</em></li>
          {#if current.needs_hard}<li class:ok={current.has_hard}><span class="ck"></span>Include one Hard problem</li>{/if}
        </ul>
        <p class="muted small">Questions and projects give XP but aren’t required.</p>
        {#if current.num > 1}<a class="btn ghost small full" href="#/review/{current.id}">Review earlier chapters</a>{/if}
      </div>
    {/if}

    {#if app.info && !app.info.toolchain.found}
      <div class="card warn">
        <div class="card-title">Rust isn’t installed</div>
        <p class="small">Lessons and questions work, but running code needs Rust. <a href="#/settings">Set it up</a>.</p>
      </div>
    {/if}
  </aside>
</div>

<style>
  .home { height: 100%; display: grid; grid-template-columns: minmax(0, 1fr) 320px; }
  .main { padding: 36px 40px 80px; }
  .wrap { max-width: 780px; margin: 0 auto; }

  .top { display: flex; align-items: flex-end; justify-content: space-between; gap: 24px; margin-bottom: 24px; }
  .eyebrow { font-size: 12px; font-weight: 700; letter-spacing: .08em; text-transform: uppercase; color: var(--muted); }
  h1 { margin: 4px 0 0; font-size: 28px; font-weight: 800; letter-spacing: -.02em; }
  .course-meter { width: 200px; font-size: 13px; color: var(--muted); text-align: right; }
  .course-meter strong { color: var(--ink); }
  .course-meter .bar { margin-top: 6px; }

  .bar { height: 8px; border-radius: 99px; background: var(--track); overflow: hidden; }
  .bar.small { height: 6px; }
  .bar > div { height: 100%; border-radius: 99px; background: var(--rust); transition: width .5s ease; }
  .bar.gold > div { background: var(--gold); }

  /* Continue hero: the logo palette, dark in both themes. */
  .hero { display: grid; grid-template-columns: auto 1fr auto; align-items: center; gap: 22px; padding: 24px 26px; border-radius: 20px; color: #FFF4E8;
    background: radial-gradient(120% 140% at 0% 0%, #3A2C24 0%, #1E1814 55%, #16120F 100%); box-shadow: 0 12px 32px rgba(22, 18, 15, .18); position: relative; overflow: hidden; }
  .hero::after { content: ''; position: absolute; right: -60px; top: -60px; width: 220px; height: 220px; border-radius: 50%; border: 22px dashed rgba(255, 138, 61, .10); pointer-events: none; }
  .hero-mark { width: 68px; height: 68px; border-radius: 18px; display: grid; place-items: center; color: #fff; background: linear-gradient(135deg, #FF8A3D, #D2461A); box-shadow: 0 6px 18px rgba(210, 70, 26, .4); }
  .hero-eyebrow { font-size: 13px; color: rgba(255, 244, 232, .7); font-weight: 600; }
  .hero h2 { margin: 4px 0 10px; font-size: 22px; font-weight: 800; letter-spacing: -.01em; line-height: 1.25; }
  .hero-meta { display: flex; flex-wrap: wrap; gap: 6px; }
  .chip { font-size: 12px; font-weight: 700; padding: 3px 10px; border-radius: 99px; background: rgba(255, 255, 255, .1); }
  .chip.xp { background: rgba(242, 182, 50, .18); color: #F8CF6F; }
  .hero-progress { display: flex; align-items: center; gap: 12px; margin-top: 14px; font-size: 12px; color: rgba(255, 244, 232, .65); }
  .hero-progress .bar { flex: 1; max-width: 260px; background: rgba(255, 255, 255, .12); }
  .hero-progress .bar > div { background: linear-gradient(90deg, #FF8A3D, #D2461A); }
  .hero-btn { z-index: 1; }

  .section-head { display: flex; align-items: baseline; justify-content: space-between; margin: 36px 0 14px; }
  .section-head h3 { margin: 0; font-size: 18px; font-weight: 800; }
  .section-head span { font-size: 13px; }

  .chapters { display: flex; flex-direction: column; gap: 10px; }
  .chapter { background: var(--surface); border: 1px solid var(--line); border-radius: 16px; overflow: hidden; transition: border-color .15s ease, box-shadow .15s ease; }
  .chapter.current { border-color: color-mix(in srgb, var(--rust) 45%, var(--line)); }
  .chapter.open { box-shadow: 0 6px 20px rgba(0, 0, 0, .05); }
  .chapter.soon { background: transparent; border-style: dashed; }
  .ch-head { width: 100%; display: flex; align-items: center; gap: 16px; padding: 14px 18px; background: none; border: 0; text-align: left; font: inherit; color: inherit; cursor: pointer; }
  .ch-head:disabled { cursor: default; }
  .ch-head:not(:disabled):hover { background: var(--surface-2); }
  .ch-num { flex: none; width: 40px; height: 40px; border-radius: 12px; display: grid; place-items: center; font-weight: 800; font-size: 16px; background: var(--rust-soft); color: var(--rust); }
  .ch-num.passed { background: var(--green-soft); color: var(--green); }
  .locked .ch-num, .soon .ch-num { background: var(--surface-2); color: var(--muted); }
  .ch-text { flex: 1; min-width: 0; }
  .ch-eyebrow { font-size: 11px; font-weight: 700; letter-spacing: .06em; text-transform: uppercase; color: var(--muted); }
  .ch-title { font-size: 16px; font-weight: 700; margin-top: 1px; }
  .soon .ch-title, .locked .ch-title { color: var(--ink-3); }
  .ch-why { font-size: 12px; color: var(--muted); margin-top: 2px; }
  .ch-side { display: flex; align-items: center; gap: 14px; flex: none; }
  .ch-prog { display: flex; align-items: center; gap: 10px; font-size: 12px; font-weight: 700; color: var(--muted); font-variant-numeric: tabular-nums; }
  .ch-prog .bar { width: 90px; }
  .chev { color: var(--muted); transition: transform .2s ease; }
  .open .chev { transform: rotate(180deg); }
  .tag { font-size: 12px; font-weight: 700; color: var(--muted); padding: 4px 10px; border-radius: 99px; background: var(--surface-2); }

  /* Items: a vertical timeline. */
  .items { list-style: none; margin: 0; padding: 4px 18px 14px; border-top: 1px solid var(--line); }
  .item { position: relative; display: flex; align-items: center; gap: 14px; }
  .item::before { content: ''; position: absolute; left: 19px; top: 0; bottom: 0; width: 2px; background: var(--line); }
  .item:first-child::before { top: 50%; }
  .item:last-child::before { bottom: 50%; }
  .item.done::before { background: color-mix(in srgb, var(--green) 40%, var(--line)); }
  .dot { position: relative; z-index: 1; flex: none; width: 40px; height: 32px; display: grid; place-items: center; }
  .dot :global(svg) { box-sizing: content-box; padding: 7px; border-radius: 50%; background: var(--surface-2); color: var(--muted); border: 2px solid var(--surface); }
  .item.open .dot :global(svg) { background: var(--rust-soft); color: var(--rust); }
  .item.open.kind-questions .dot :global(svg), .item.open.kind-practice .dot :global(svg) { background: var(--blue-soft); color: var(--blue); }
  .item.open.kind-problem .dot :global(svg) { background: var(--green-soft); color: var(--green); }
  .item.open.kind-project .dot :global(svg), .item.open.kind-milestone .dot :global(svg) { background: var(--purple-soft); color: var(--purple); }
  .item.open.kind-review .dot :global(svg) { background: var(--gold-soft); color: var(--gold-ink); }
  .item.done .dot :global(svg) { background: var(--green); color: #fff; }
  .item.next .dot :global(svg) { background: var(--rust); color: #fff; box-shadow: 0 0 0 4px color-mix(in srgb, var(--rust) 22%, transparent); }
  .row { flex: 1; min-width: 0; display: grid; grid-template-columns: minmax(0, 1fr) auto auto; align-items: center; gap: 14px; padding: 10px 12px; margin: 2px 0; border-radius: 10px; color: inherit; text-decoration: none; }
  a.row:hover { background: var(--surface-2); }
  .item.next .row { background: var(--rust-soft); }
  .it-title { font-size: 14px; font-weight: 600; white-space: nowrap; overflow: hidden; text-overflow: ellipsis; }
  .item.locked .it-title { color: var(--muted); font-weight: 500; }
  .it-meta { font-size: 12px; color: var(--muted); white-space: nowrap; }
  .it-end { font-size: 12px; font-weight: 700; color: var(--muted); min-width: 64px; text-align: right; white-space: nowrap; }
  .it-end.done { color: var(--green); }
  .it-end.go { color: var(--rust); }
  .it-end.lock { font-weight: 500; }

  /* Rail */
  .rail { border-left: 1px solid var(--line); padding: 36px 20px; display: flex; flex-direction: column; gap: 14px; background: var(--bg); }
  .rail .card { padding: 18px; border-radius: 16px; }
  .card-title { font-weight: 800; font-size: 15px; }
  .small { font-size: 12.5px; line-height: 1.5; margin: 0; }
  .streak-top { display: flex; justify-content: space-between; align-items: flex-start; }
  .big { font-size: 28px; font-weight: 800; letter-spacing: -.02em; line-height: 1; }
  .big span { font-size: 15px; font-weight: 700; color: var(--muted); }
  .flame { color: var(--line-2); }
  .flame.lit { color: #F07A2B; }
  .week { display: grid; grid-template-columns: repeat(7, 1fr); gap: 4px; margin-top: 16px; }
  .wd { display: flex; flex-direction: column; align-items: center; gap: 6px; }
  .wd-dot { width: 26px; height: 26px; border-radius: 50%; display: grid; place-items: center; background: var(--surface-2); border: 2px solid var(--line); color: #fff; }
  .wd.some .wd-dot { border-color: #F07A2B; }
  .wd.hit .wd-dot { background: #F07A2B; border-color: #F07A2B; }
  .wd-l { font-size: 11px; font-weight: 700; color: var(--muted); }
  .goal-card { display: flex; align-items: center; gap: 16px; }
  .ring-track { fill: none; stroke: var(--track); stroke-width: 8; }
  .ring-fill { fill: none; stroke: var(--gold); stroke-width: 8; stroke-linecap: round; transition: stroke-dashoffset .6s ease; }
  .ring text { font-size: 15px; font-weight: 800; fill: var(--ink); font-family: var(--sans); }
  .goal-num { font-size: 14px; color: var(--muted); margin: 2px 0 4px; }
  .goal-num strong { color: var(--ink); font-size: 18px; }
  .lvl-row { display: flex; align-items: center; gap: 12px; margin-bottom: 12px; }
  .lvl-badge { width: 40px; height: 40px; border-radius: 12px; display: grid; place-items: center; font-weight: 800; color: var(--gold-ink); background: var(--gold-soft); }
  .checks { list-style: none; padding: 0; margin: 12px 0; display: flex; flex-direction: column; gap: 10px; }
  .checks li { display: flex; align-items: center; gap: 10px; font-size: 13.5px; }
  .checks em { margin-left: auto; font-style: normal; font-weight: 700; color: var(--muted); font-variant-numeric: tabular-nums; }
  .ck { width: 18px; height: 18px; border-radius: 50%; border: 2px solid var(--line-2); flex: none; }
  .checks li.ok .ck { border-color: var(--green); background: var(--green) url("data:image/svg+xml,%3Csvg xmlns='http://www.w3.org/2000/svg' viewBox='0 0 24 24' fill='none' stroke='white' stroke-width='4' stroke-linecap='round' stroke-linejoin='round'%3E%3Cpath d='M5 12.5l4.5 4.5L19 7.5'/%3E%3C/svg%3E") center/12px no-repeat; }
  .full { width: 100%; margin-top: 14px; }
  .warn { border-color: var(--red); }

  @media (max-width: 1150px) {
    .home { grid-template-columns: minmax(0, 1fr) 280px; }
    .main { padding: 28px 24px 60px; }
    .it-meta { display: none; }
  }
  @media (max-width: 860px) {
    .home { grid-template-columns: 1fr; }
    .rail { display: none; }
    .hero { grid-template-columns: auto 1fr; }
    .hero-btn { grid-column: 1 / -1; }
  }
</style>
