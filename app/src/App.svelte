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

<svelte:window onclick={guardLinks} onkeydown={e => { if (app.asking && e.key === 'Escape') app.asking.answer(false) }} />

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
        <div class="brand">
          <svg width="30" height="30" viewBox="100 100 824 824" aria-hidden="true"><defs><linearGradient id="lg" x1="0" y1="0" x2="1" y2="1"><stop offset="0" stop-color="#FF8A3D" /><stop offset="1" stop-color="#D2461A" /></linearGradient></defs><rect x="100" y="100" width="824" height="824" rx="185" fill="#1E1814" /><circle cx="512" cy="512" r="292" fill="none" stroke="url(#lg)" stroke-width="64" stroke-dasharray="45.86 45.86" /><circle cx="512" cy="512" r="252" fill="none" stroke="url(#lg)" stroke-width="44" /><g fill="none" stroke="#FFF4E8" stroke-width="48" stroke-linecap="round" stroke-linejoin="round"><polyline points="420,412 324,512 420,612" /><polyline points="604,412 700,512 604,612" /><line x1="548" y1="392" x2="476" y2="632" stroke="#FF8A3D" /></g></svg>
          <span>Rusticoder</span>
        </div>
        {#snippet ico(d: string)}<svg width="20" height="20" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" aria-hidden="true">{@html d}</svg>{/snippet}
        <div class="nav-label">Menu</div>
        <a href="#/" class:on={r.name === 'home'}>{@render ico('<path d="M3 10.5 12 3l9 7.5V20a1 1 0 0 1-1 1h-5v-6h-6v6H4a1 1 0 0 1-1-1z"/>')}Learn</a>
        {#if reviewChapter}<a href="#/review/{reviewChapter}" class:on={r.name === 'review'}>{@render ico('<path d="M3 12a9 9 0 0 1 15-6.7L21 8"/><path d="M21 3v5h-5"/><path d="M21 12a9 9 0 0 1-15 6.7L3 16"/><path d="M3 21v-5h5"/>')}Review</a>{/if}
        <a href="#/profile" class:on={r.name === 'profile'}>{@render ico('<path d="M4 20V10M10 20V4M16 20v-7M22 20H2"/>')}Progress</a>
        <a href="#/settings" class:on={r.name === 'settings'}>{@render ico('<circle cx="12" cy="12" r="3"/><path d="M19.4 15a1.7 1.7 0 0 0 .3 1.8l.1.1a2 2 0 1 1-2.8 2.8l-.1-.1a1.7 1.7 0 0 0-1.8-.3 1.7 1.7 0 0 0-1 1.5V21a2 2 0 1 1-4 0v-.1a1.7 1.7 0 0 0-1.1-1.5 1.7 1.7 0 0 0-1.8.3l-.1.1a2 2 0 1 1-2.8-2.8l.1-.1a1.7 1.7 0 0 0 .3-1.8 1.7 1.7 0 0 0-1.5-1H3a2 2 0 1 1 0-4h.1a1.7 1.7 0 0 0 1.5-1.1 1.7 1.7 0 0 0-.3-1.8l-.1-.1a2 2 0 1 1 2.8-2.8l.1.1a1.7 1.7 0 0 0 1.8.3H9a1.7 1.7 0 0 0 1-1.5V3a2 2 0 1 1 4 0v.1a1.7 1.7 0 0 0 1 1.5 1.7 1.7 0 0 0 1.8-.3l.1-.1a2 2 0 1 1 2.8 2.8l-.1.1a1.7 1.7 0 0 0-.3 1.8V9a1.7 1.7 0 0 0 1.5 1H21a2 2 0 1 1 0 4h-.1a1.7 1.7 0 0 0-1.5 1z"/>')}Settings</a>
        <div class="course">
          <div class="course-k">Course</div>
          <div class="course-v">{app.info.course_title}</div>
        </div>
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

{#if app.asking}
  {@const q = app.asking}
  <div class="ask-backdrop" role="presentation" onclick={() => q.answer(false)}></div>
  <div class="ask" role="alertdialog" aria-modal="true" aria-labelledby="ask-title">
    <h2 id="ask-title">{q.title}</h2>
    {#if q.body}<p>{q.body}</p>{/if}
    <div class="ask-actions">
      <!-- Focus Cancel for destructive questions, so Enter never deletes anything. -->
      <!-- svelte-ignore a11y_autofocus -->
      <button class="btn ghost" autofocus={q.danger} onclick={() => q.answer(false)}>Cancel</button>
      <!-- svelte-ignore a11y_autofocus -->
      <button class="btn {q.danger ? 'stop' : 'primary'}" autofocus={!q.danger} onclick={() => q.answer(true)}>{q.yes}</button>
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
  .side { border-right: 1px solid var(--line); background: var(--surface); padding: 22px 14px; display: flex; flex-direction: column; gap: 2px; }
  .brand { display: flex; align-items: center; gap: 10px; font-size: 19px; font-weight: 800; padding: 0 8px 22px; color: var(--ink); letter-spacing: -.02em; }
  .nav-label { font-size: 11px; font-weight: 700; letter-spacing: .08em; text-transform: uppercase; color: var(--muted); padding: 0 12px 8px; }
  .side a { position: relative; display: flex; align-items: center; gap: 12px; padding: 10px 12px; border-radius: 10px; font-weight: 600; font-size: 14.5px; color: var(--ink-3); text-decoration: none; }
  .side a :global(svg) { color: var(--muted); }
  .side a:hover { background: var(--surface-2); }
  .side a.on { background: var(--rust-soft); color: var(--rust); font-weight: 700; }
  .side a.on :global(svg) { color: var(--rust); }
  .side a.on::before { content: ''; position: absolute; left: -14px; top: 8px; bottom: 8px; width: 3px; border-radius: 0 3px 3px 0; background: var(--rust); }
  .course { margin-top: auto; padding: 12px; border-radius: 12px; background: var(--surface-2); }
  .course-k { font-size: 11px; font-weight: 700; letter-spacing: .08em; text-transform: uppercase; color: var(--muted); }
  .course-v { font-size: 13px; font-weight: 700; margin-top: 2px; }
  .fatal { padding: 40px; }
  .fatal pre { white-space: pre-wrap; }
  .toasts { position: fixed; right: 20px; bottom: 20px; display: flex; flex-direction: column; gap: 10px; z-index: 100; }
  .toast { min-width: 220px; max-width: 360px; background: var(--surface); border: 2px solid var(--line); border-radius: 14px; padding: 12px 16px; box-shadow: 0 10px 30px rgba(0, 0, 0, .15); animation: pop .3s ease; }
  .toast.xp { border-color: var(--gold); }
  .toast.badge { border-color: var(--purple); }
  .toast.error { border-color: var(--red); }
  .ask-backdrop { position: fixed; inset: 0; background: rgba(0, 0, 0, .35); z-index: 90; }
  .ask { position: fixed; top: 50%; left: 50%; transform: translate(-50%, -50%); width: min(460px, calc(100vw - 32px)); background: var(--surface); border: 2px solid var(--line); border-radius: 18px; padding: 24px; z-index: 91; box-shadow: 0 20px 50px rgba(0, 0, 0, .25); animation: pop .2s ease; }
  .ask h2 { margin: 0 0 8px; font-size: 20px; font-weight: 800; }
  .ask p { margin: 0; color: var(--ink-3); line-height: 1.5; }
  .ask-actions { display: flex; justify-content: flex-end; gap: 12px; margin-top: 22px; }
  .tt { font-weight: 800; }
  .tb { font-size: 14px; color: var(--ink-3); margin-top: 2px; white-space: pre-wrap; }
</style>
