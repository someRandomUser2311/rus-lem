<script lang="ts">
  // A follow-up or practice set: one question at a time, feedback after each,
  // a score at the end. The score counts each question's first answer.
  import { api, errorText } from '../lib/api'
  import type { Award } from '../lib/bindings/Award'
  import type { Checked } from '../lib/bindings/Checked'
  import type { ChapterView } from '../lib/bindings/ChapterView'
  import type { QuestionSet } from '../lib/bindings/QuestionSet'
  import { app } from '../lib/state.svelte'
  import Md from '../components/Md.svelte'
  import QuestionCard from '../components/QuestionCard.svelte'
  import { plain } from '../lib/md'

  let { chapter, id }: { chapter: string; id: string } = $props()

  let set = $state<QuestionSet | null>(null)
  let ch = $state<ChapterView | null>(null)
  let error = $state<string | null>(null)
  let i = $state(0)
  let correct = $state(0)
  let finished = $state(false)
  let award = $state<Award | null>(null)
  let card: QuestionCard | undefined = $state()
  let canCheck = $state(false)
  let result = $state<Checked | null>(null)

  $effect(() => {
    void id
    app.chapter(chapter).then(c => {
      ch = c
      set = c.question_sets.find(s => s.id === id) ?? null
      if (!set) error = `Question set ${id} wasn't found.`
      i = 0
      correct = 0
      finished = false
      award = null
      result = null
    }).catch(e => (error = errorText(e)))
  })

  const q = $derived(set?.questions[i])
  const lesson = $derived(set?.lesson ? ch?.lessons.find(l => l.id === set?.lesson) : null)
  const score = $derived(set ? correct / set.questions.length : 0)

  async function check() {
    if (!card) return
    const r = await card.check()
    if (r.correct) correct++
  }

  async function next() {
    if (!set) return
    result = null
    if (i < set.questions.length - 1) {
      i++
      return
    }
    finished = true
    try {
      award = await api.completeSet(set.id, score)
      app.celebrate(award)
      await app.refresh()
    } catch (e) {
      app.toast({ kind: 'error', title: 'Could not save progress', body: errorText(e) })
    }
  }

  // The next unlocked item on the path after this set, finished or not.
  const nextItem = $derived.by(() => {
    const next = app.nextAfter(id)
    if (!next) return null
    const { ch, n } = next
    const href = n.kind === 'lesson' ? `#/lesson/${ch.id}/${n.id}` : n.kind === 'questions' || n.kind === 'practice' ? `#/questions/${ch.id}/${n.id}` : n.kind === 'review' ? `#/review/${ch.id}` : `#/problem/${ch.id}/${n.id}`
    return { href, label: `Next: ${plain(n.title)}` }
  })
</script>

{#if error}
  <div class="center"><p>{error}</p><a class="btn ghost" href="#/">Back to the path</a></div>
{:else if set}
  <div class="player">
    <header class="top">
      <a class="close" href="#/" aria-label="Close">✕</a>
      <div class="progress"><div style="width:{((i + (finished ? 1 : 0)) / set.questions.length) * 100}%"></div></div>
      <span class="label">{set.practice ? 'Practice' : `Questions · ${plain(lesson?.title)}`}</span>
    </header>
    <main class="stage scroll">
      {#if !finished && q}
        {#key i}
          <section class="screen">
            <div class="eyebrow">Question {i + 1} of {set.questions.length} · {'●'.repeat(q.difficulty)}{'○'.repeat(3 - q.difficulty)}</div>
            <QuestionCard bind:this={card} {q} record bind:canCheck bind:result />
          </section>
        {/key}
      {:else}
        <section class="screen done">
          <div class="medal" class:great={score >= 0.8}>{Math.round(score * 100)}%</div>
          <h1>{set.practice ? 'Practice complete' : 'Questions complete'}</h1>
          <p class="muted">You got {correct} of {set.questions.length} right on the first try.{score < 0.8 ? ' Every explanation you read is part of learning; try the set again any time.' : ''}</p>
          {#if lesson?.foundation && score >= 0.8 && award && award.xp > 10}
            <div class="callout">You scored 80% or more, so the lesson “<Md inline src={lesson.title} />” counts as done.</div>
          {/if}
          <div class="actions">
            {#if nextItem}<a class="btn primary" href={nextItem.href}>{nextItem.label}</a>{/if}
            <a class="btn ghost" href="#/">Back to the path</a>
          </div>
        </section>
      {/if}
    </main>
    {#if !finished}
      <footer class="footer-bar" class:right-state={result?.correct} class:wrong-state={result && !result.correct}>
        {#if result}
          <div class="feedback">
            <div class="icon">{result.correct ? '✓' : '✕'}</div>
            <div>
              <div class="title">{result.correct ? 'Correct!' : 'Not quite'}</div>
              <div class="body"><Md src={card?.feedback() ?? ''} inline /></div>
            </div>
          </div>
          <button class="btn {result.correct ? 'go' : 'stop'}" onclick={next}>Continue</button>
        {:else}
          <span class="muted">{set.practice ? 'Mixed practice from the whole chapter' : 'Answer to see why'}</span>
          <button class="btn blue" onclick={check} disabled={!canCheck}>Check</button>
        {/if}
      </footer>
    {/if}
  </div>
{:else}
  <div class="center muted">Loading…</div>
{/if}

<style>
  .player { height: 100%; display: flex; flex-direction: column; }
  .top { display: flex; align-items: center; gap: 20px; padding: 16px 32px; background: var(--surface); border-bottom: 1px solid var(--line); }
  .close { width: 40px; height: 40px; border-radius: 12px; color: var(--muted); display: flex; align-items: center; justify-content: center; text-decoration: none; font-size: 18px; }
  .top .progress { flex: 1; }
  .label { font-size: 14px; font-weight: 700; color: var(--muted); white-space: nowrap; }
  .stage { flex: 1; }
  .screen { max-width: 780px; margin: 0 auto; padding: 36px 32px; display: flex; flex-direction: column; gap: 20px; animation: screen-in .3s ease; }
  @keyframes screen-in { from { opacity: 0; transform: translateY(6px); } to { opacity: 1; transform: none; } }
  .done { align-items: center; text-align: center; }
  .done h1 { margin: 0; }
  .medal { width: 110px; height: 110px; border-radius: 30px; background: var(--blue); box-shadow: 0 8px 0 var(--blue-dark); display: flex; align-items: center; justify-content: center; color: #fff; font-size: 30px; font-weight: 900; animation: pop .5s ease; }
  .medal.great { background: var(--gold); box-shadow: 0 8px 0 var(--gold-dark); }
  .callout { background: var(--green-soft); color: var(--green-dark); border-radius: 14px; padding: 12px 16px; }
  .actions { display: flex; flex-direction: column; gap: 12px; width: 100%; max-width: 420px; }
  .center { height: 100%; display: flex; flex-direction: column; gap: 16px; align-items: center; justify-content: center; }
</style>
