<script lang="ts">
  // Progress: stats, the activity calendar, the skill tree (concept mastery by chapter) and badges.
  import { api, errorText } from '../lib/api'
  import type { ProfilePage } from '../lib/bindings/ProfilePage'

  let page = $state<ProfilePage | null>(null)
  let error = $state<string | null>(null)
  let focus = $state<string | null>(null)

  $effect(() => {
    api.profile().then(p => (page = p)).catch(e => (error = errorText(e)))
  })

  const byChapter = $derived.by(() => {
    if (!page) return []
    const groups = new Map<number, ProfilePage['concepts']>()
    for (const c of page.concepts) groups.set(c.chapter, [...(groups.get(c.chapter) ?? []), c])
    return [...groups.entries()].map(([num, concepts]) => ({ num, concepts, ch: page!.chapters.find(c => c.num === num) }))
  })

  const shade = (m: number) => (m <= 0 ? 'm0' : m < 25 ? 'm1' : m < 50 ? 'm2' : m < 75 ? 'm3' : 'm4')
  const icons: Record<string, string> = { footprints: '👣', check: '✅', stack: '📚', mountain: '⛰️', spark: '✨', flame: '🔥', star: '⭐', hammer: '🔨', diamond: '💎', rocket: '🚀', flag: '🏁', trophy: '🏆' }
  const focused = $derived(page?.concepts.find(c => c.id === focus))
</script>

<div class="page scroll">
  {#if error}<p>{error}</p>{:else if page}
    <h1>Your progress</h1>
    <div class="cards">
      <div class="card big"><div class="k">Total XP</div><div class="v">{page.stats.xp}</div></div>
      <div class="card big"><div class="k">Level</div><div class="v">{page.stats.level}</div></div>
      <div class="card big"><div class="k">Streak</div><div class="v">🔥 {page.stats.streak}</div><div class="s">best {page.stats.best_streak}</div></div>
      <div class="card big"><div class="k">Lessons</div><div class="v">{page.stats.lessons_done}</div></div>
      <div class="card big"><div class="k">Problems solved</div><div class="v">{page.stats.problems_accepted}</div></div>
      <div class="card big"><div class="k">Questions right</div><div class="v">{page.stats.questions_correct}</div></div>
    </div>

    <h2>Activity</h2>
    <div class="card calendar">
      {#each page.stats.calendar as [day, xp]}
        <span class="day {xp >= 20 ? 'good' : xp > 0 ? 'some' : ''}" title="{day}: {xp} XP"></span>
      {/each}
    </div>
    <p class="muted small">Last 10 weeks. Dark squares are days with 20 XP or more.</p>

    <h2>Skill tree</h2>
    <p class="muted small">Every concept in the course. Mastery grows when you solve problems and answer questions about it, and fades slowly without practice. Review picks your weakest concepts.</p>
    <div class="tree">
      {#each byChapter as g}
        <div class="branch" class:locked={g.ch?.state.state === 'locked'}>
          <div class="branch-head">Ch {g.num} · {g.ch?.title ?? ''}</div>
          <div class="leaves">
            {#each g.concepts as c}
              {@const m = page.mastery[c.id] ?? 0}
              <button class="leaf {shade(m)}" class:quiz={c.tested_by === 'quiz'} onclick={() => (focus = c.id)} title="{c.name}: {Math.round(m)}%">{c.id}</button>
            {/each}
          </div>
        </div>
      {/each}
    </div>
    {#if focused}
      <div class="card focus">
        <div class="k">{focused.id} · introduced in {focused.lesson}</div>
        <div class="fname">{focused.name}</div>
        <p>{focused.definition}</p>
        <div class="progress"><div style="width:{page.mastery[focused.id] ?? 0}%"></div></div>
        <div class="muted small">Mastery {Math.round(page.mastery[focused.id] ?? 0)}%</div>
      </div>
    {/if}

    <h2>Badges</h2>
    <div class="badges">
      {#each page.badges as b}
        <div class="badge card" class:earned={b.earned_at != null}>
          <div class="bi">{icons[b.badge.icon] ?? '⭐'}</div>
          <div class="bt">{b.badge.title}</div>
          <div class="bd">{b.badge.description}</div>
        </div>
      {/each}
    </div>
  {:else}<p class="muted">Loading…</p>{/if}
</div>

<style>
  .page { height: 100%; padding: 28px 36px 60px; }
  h1 { margin: 0 0 18px; }
  h2 { margin: 28px 0 10px; font-size: 20px; }
  .cards { display: grid; grid-template-columns: repeat(auto-fill, minmax(160px, 1fr)); gap: 12px; }
  .big .k { font-size: 12px; font-weight: 800; color: var(--muted); text-transform: uppercase; letter-spacing: .06em; }
  .big .v { font-size: 28px; font-weight: 800; margin-top: 4px; }
  .big .s { font-size: 12px; color: var(--muted); }
  .calendar { display: grid; grid-template-rows: repeat(7, 14px); grid-auto-flow: column; grid-auto-columns: 14px; gap: 4px; width: max-content; }
  .day { width: 14px; height: 14px; border-radius: 3px; background: var(--track); }
  .day.some { background: #F5C9B5; }
  .day.good { background: var(--rust); }
  .small { font-size: 13px; }
  .tree { display: flex; flex-direction: column; gap: 10px; }
  .branch { display: grid; grid-template-columns: 220px 1fr; gap: 14px; align-items: start; }
  .branch.locked { opacity: .5; }
  .branch-head { font-weight: 800; font-size: 14px; padding-top: 6px; }
  .leaves { display: flex; flex-wrap: wrap; gap: 6px; }
  .leaf { border: 2px solid var(--line-2); border-radius: 999px; padding: 4px 10px; font-family: var(--mono); font-size: 12px; cursor: pointer; background: var(--surface); }
  .leaf.quiz { border-style: dashed; }
  .leaf.m1 { background: #FBEEE8; border-color: #F5C9B5; }
  .leaf.m2 { background: #F7D5C6; border-color: #EDA584; }
  .leaf.m3 { background: #EDA584; border-color: var(--rust); }
  .leaf.m4 { background: var(--rust); border-color: var(--rust-dark); color: #fff; }
  .focus { margin-top: 12px; max-width: 560px; }
  .focus .k { font-size: 12px; color: var(--muted); font-weight: 700; }
  .fname { font-size: 18px; font-weight: 800; margin-top: 4px; }
  .badges { display: grid; grid-template-columns: repeat(auto-fill, minmax(180px, 1fr)); gap: 12px; }
  .badge { text-align: center; opacity: .45; filter: grayscale(1); }
  .badge.earned { opacity: 1; filter: none; border-color: var(--gold); }
  .bi { font-size: 32px; }
  .bt { font-weight: 800; margin-top: 6px; }
  .bd { font-size: 13px; color: var(--muted); margin-top: 4px; }
</style>
