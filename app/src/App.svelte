<script lang="ts">
  // The shell: left navigation, the current screen, and toasts for XP and badges.
  import { app } from './lib/state.svelte'
  import Home from './screens/Home.svelte'
  import Lesson from './screens/Lesson.svelte'
  import Onboarding from './screens/Onboarding.svelte'
  import Problem from './screens/Problem.svelte'
  import Profile from './screens/Profile.svelte'
  import Questions from './screens/Questions.svelte'
  import Review from './screens/Review.svelte'
  import Settings from './screens/Settings.svelte'

  app.load()

  // External links in course text would replace the whole app; keep the learner here.
  function guardLinks(e: MouseEvent) {
    const a = (e.target as HTMLElement).closest?.('a')
    const href = a?.getAttribute('href') ?? ''
    if (a && /^(https?:|mailto:)/.test(href)) {
      e.preventDefault()
      app.toast({ kind: 'info', title: 'Link', body: href })
    }
  }

  const r = $derived(app.route)
  const focus = $derived(r.name === 'lesson' || r.name === 'questions' || r.name === 'problem' || r.name === 'onboarding')
  const reviewChapter = $derived(app.path.find(c => c.state.state === 'open' && c.num > 1)?.id)
</script>

<svelte:window onclick={guardLinks} />

{#if app.loadError}
  <div class="fatal">
    <h1>Rusticoder couldn't start</h1>
    <pre>{app.loadError}</pre>
  </div>
{:else if !app.info}
  <div class="fatal muted">Loading…</div>
{:else}
  <div class="shell" class:focus>
    {#if !focus}
      <nav class="side">
        <div class="brand">🦀 <span>Rusticoder</span></div>
        <a href="#/" class:on={r.name === 'home'}><span>🗺️</span>Learn</a>
        {#if reviewChapter}<a href="#/review/{reviewChapter}" class:on={r.name === 'review'}><span>🔁</span>Review</a>{/if}
        <a href="#/profile" class:on={r.name === 'profile'}><span>📈</span>Progress</a>
        <a href="#/settings" class:on={r.name === 'settings'}><span>⚙️</span>Settings</a>
        <div class="course">{app.info.course_title}</div>
      </nav>
    {/if}
    <div class="main">
      {#if r.name === 'onboarding'}<Onboarding />
      {:else if r.name === 'lesson'}{#key r.id}<Lesson chapter={r.chapter} id={r.id} />{/key}
      {:else if r.name === 'questions'}{#key r.id}<Questions chapter={r.chapter} id={r.id} />{/key}
      {:else if r.name === 'problem'}{#key r.id}<Problem chapter={r.chapter} id={r.id} review={r.review} />{/key}
      {:else if r.name === 'review'}<Review chapter={r.chapter} />
      {:else if r.name === 'profile'}<Profile />
      {:else if r.name === 'settings'}<Settings />
      {:else}<Home />{/if}
    </div>
  </div>
{/if}

<div class="toasts" aria-live="polite">
  {#each app.toasts as t (t.id)}
    <div class="toast {t.kind}">
      <div class="tt">{t.kind === 'xp' ? '⚡ ' : t.kind === 'badge' ? '🏅 ' : ''}{t.title}</div>
      {#if t.body}<div class="tb">{t.body}</div>{/if}
    </div>
  {/each}
</div>

<style>
  .shell { height: 100%; display: grid; grid-template-columns: 220px 1fr; }
  .shell.focus { grid-template-columns: 1fr; }
  .main { min-width: 0; min-height: 0; height: 100%; overflow: hidden; }
  .side { border-right: 1px solid var(--line); background: var(--surface); padding: 20px 14px; display: flex; flex-direction: column; gap: 6px; }
  .brand { font-size: 22px; font-weight: 900; padding: 4px 10px 18px; color: var(--rust); letter-spacing: -.02em; }
  .side a { display: flex; align-items: center; gap: 12px; padding: 12px 14px; border-radius: 12px; font-weight: 800; color: var(--ink-3); text-decoration: none; border: 2px solid transparent; }
  .side a:hover { background: var(--surface-2); }
  .side a.on { background: var(--rust-soft); color: var(--rust); border-color: var(--rust); }
  .course { margin-top: auto; font-size: 12px; color: var(--muted); padding: 10px; }
  .fatal { padding: 40px; }
  .fatal pre { white-space: pre-wrap; }
  .toasts { position: fixed; right: 20px; bottom: 20px; display: flex; flex-direction: column; gap: 10px; z-index: 100; }
  .toast { min-width: 220px; max-width: 360px; background: var(--surface); border: 2px solid var(--line); border-radius: 14px; padding: 12px 16px; box-shadow: 0 10px 30px rgba(0, 0, 0, .15); animation: pop .3s ease; }
  .toast.xp { border-color: var(--gold); }
  .toast.badge { border-color: var(--purple); }
  .toast.error { border-color: var(--red); }
  .tt { font-weight: 800; }
  .tb { font-size: 14px; color: var(--ink-3); margin-top: 2px; white-space: pre-wrap; }
</style>
