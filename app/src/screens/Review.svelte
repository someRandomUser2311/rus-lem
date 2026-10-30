<script lang="ts">
  // Spaced-repetition review: problems from earlier chapters for your weakest concepts.
  import { api, errorText } from '../lib/api'
  import type { ReviewPick } from '../lib/bindings/ReviewPick'
  import { app } from '../lib/state.svelte'
  import Md from '../components/Md.svelte'

  let { chapter }: { chapter: string } = $props()
  let picks = $state<ReviewPick[] | null>(null)
  let error = $state<string | null>(null)
  const num = $derived(Number(chapter.replace(/^ch/, '')))
  const concept = (id: string) => app.info?.concepts.find(c => c.id === id)

  $effect(() => {
    api.review(num).then(p => (picks = p)).catch(e => (error = errorText(e)))
  })
</script>

<div class="page scroll">
  <a class="back" href="#/">← Path</a>
  <h1>Review · before Chapter {num}</h1>
  <p class="muted">Problems from earlier chapters, picked for the concepts you've practised least. Each Accepted review earns its level's XP.</p>
  {#if error}<p>{error}</p>
  {:else if picks && picks.length === 0}
    <div class="card">There's nothing to review yet: this is the first chapter.</div>
  {:else if picks}
    <div class="list">
      {#each picks as r}
        {@const c = concept(r.concept)}
        <a class="card pick" href="#/problem/ch{String(r.chapter).padStart(2, '0')}/{r.problem.id}/{num}">
          <div class="top"><span class="tag level-{r.problem.level}">{r.problem.level}</span><span class="muted">Chapter {r.chapter}</span><span class="xp">{r.problem.xp} XP</span></div>
          <div class="title">{r.problem.id} · <Md inline src={r.problem.title} /></div>
          <div class="why">Revisits <code>{r.concept}</code>{c ? ` (${c.name})` : ''} · mastery {Math.round(r.mastery)}%</div>
        </a>
      {/each}
    </div>
  {:else}<p class="muted">Loading…</p>{/if}
</div>

<style>
  .page { height: 100%; padding: 28px 36px; max-width: 820px; margin: 0 auto; }
  .back { text-decoration: none; font-weight: 700; color: var(--muted); }
  h1 { margin: 12px 0 6px; }
  .list { display: flex; flex-direction: column; gap: 12px; margin-top: 18px; }
  .pick { text-decoration: none; color: inherit; display: flex; flex-direction: column; gap: 6px; transition: transform .1s ease; }
  .pick:hover { transform: translateY(-2px); border-color: var(--rust); }
  .top { display: flex; gap: 10px; align-items: center; font-size: 13px; }
  .xp { margin-left: auto; font-weight: 800; color: var(--gold-ink); }
  .title { font-size: 18px; font-weight: 800; }
  .why { font-size: 14px; color: var(--ink-3); }
</style>
