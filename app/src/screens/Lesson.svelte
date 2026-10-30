<script lang="ts">
  // The lesson player: one idea per screen, steppers advance step by step, quick
  // checks must be answered correctly to move on, then a summary and a done screen.
  import { api, errorText } from '../lib/api'
  import type { Award } from '../lib/bindings/Award'
  import type { Checked } from '../lib/bindings/Checked'
  import type { Lesson } from '../lib/bindings/Lesson'
  import { app } from '../lib/state.svelte'
  import { panelStates } from '../lib/viz'
  import BookPanel from '../components/BookPanel.svelte'
  import CodeView from '../components/CodeView.svelte'
  import CodeFiles from '../components/CodeFiles.svelte'
  import ComingFrom from '../components/ComingFrom.svelte'
  import ErrorExplainer from '../components/ErrorExplainer.svelte'
  import Md from '../components/Md.svelte'
  import Panels from '../components/Panels.svelte'
  import QuestionCard from '../components/QuestionCard.svelte'

  let { chapter, id }: { chapter: string; id: string } = $props()

  let lesson = $state<Lesson | null>(null)
  let error = $state<string | null>(null)
  let pos = $state(0)
  let tries = $state(0)
  let award = $state<Award | null>(null)
  let readMore = $state(false)
  let card: QuestionCard | undefined = $state()
  let canCheck = $state(false)
  let result = $state<Checked | null>(null)

  $effect(() => {
    void id
    app.chapter(chapter).then(c => {
      lesson = c.lessons.find(l => l.id === id) ?? null
      if (!lesson) error = `Lesson ${id} wasn't found.`
      pos = 0
      tries = 0
      award = null
    }).catch(e => (error = errorText(e)))
  })

  type Pos = { kind: 'screen'; screen: number; step: number } | { kind: 'summary' } | { kind: 'done' }
  const positions = $derived.by((): Pos[] => {
    if (!lesson) return []
    const out: Pos[] = []
    lesson.screens.forEach((s, i) => {
      if (s.kind === 'stepper' && s.steps.length) s.steps.forEach((_, k) => out.push({ kind: 'screen', screen: i, step: k + 1 }))
      else out.push({ kind: 'screen', screen: i, step: 0 })
    })
    out.push({ kind: 'summary' }, { kind: 'done' })
    return out
  })
  const cur = $derived(positions[pos])
  const screen = $derived(cur?.kind === 'screen' && lesson ? lesson.screens[cur.screen] : null)
  const step = $derived(cur?.kind === 'screen' ? cur.step : 0)
  const stepData = $derived(screen && step ? screen.steps[step - 1] : null)
  const panels = $derived(screen ? panelStates(screen, screen.kind === 'stepper' ? step : 0) : [])
  const progress = $derived(positions.length ? (pos / (positions.length - 1)) * 100 : 0)
  const isCheck = $derived(screen?.kind === 'check')
  const teachingNo = $derived(lesson && cur?.kind === 'screen' ? lesson.screens.slice(0, cur.screen + 1).filter(s => s.kind !== 'check').length : 0)
  const teachingTotal = $derived(lesson ? lesson.screens.filter(s => s.kind !== 'check').length : 0)
  const nextSet = $derived(`Q${id.slice(1)}`)
  // Error screens mark the lines rustc points at (` --> src/main.rs:3:28`).
  // The file a compiler message points at (` --> src/lib.rs:2:5`): an error screen's
  // package opens on it.
  function diagnosticFile(d: string | null | undefined): string | null {
    return d?.match(/-->\s*([^:\s]+):\d+/)?.[1] ?? null
  }
  const errLines = $derived(screen?.kind === 'error' && screen.diagnostic ? [...screen.diagnostic.matchAll(/-->\s*[^:\s]+:(\d+):\d+/g)].map(m => Number(m[1])) : [])

  async function next() {
    if (!lesson) return
    if (pos < positions.length - 1) {
      pos++
      result = null
      if (positions[pos].kind === 'done' && !award) {
        try {
          award = await api.completeLesson(id, tries)
          app.celebrate(award)
          await app.refresh()
        } catch (e) {
          app.toast({ kind: 'error', title: 'Could not save progress', body: errorText(e) })
        }
      }
    }
  }

  function back() {
    if (pos > 0) {
      pos--
      result = null
    }
  }

  async function check() {
    if (!card) return
    const r = await card.check()
    if (!r.correct) tries++
  }

  function retry() {
    card?.reset()
  }

  function replay() {
    pos = 0
    result = null
  }

  function onkey(e: KeyboardEvent) {
    if (readMore || (e.target as HTMLElement)?.closest('input, textarea')) return
    if (e.key === 'ArrowRight' && !isCheck && cur?.kind !== 'done') next()
    if (e.key === 'ArrowLeft') back()
  }
</script>

<svelte:window onkeydown={onkey} />

{#if error}
  <div class="center"><p>{error}</p><a class="btn ghost" href="#/">Back to the path</a></div>
{:else if lesson && cur}
  <div class="player">
    <header class="top">
      <a class="close" href="#/" aria-label="Close lesson">✕</a>
      <div class="progress" aria-label="Lesson progress"><div style="width:{progress}%"></div></div>
      <span class="label">{lesson.id} · <Md inline src={lesson.title} /></span>
      {#if lesson.read_more.length}<button class="btn ghost small" onclick={() => (readMore = true)}>Read more</button>{/if}
    </header>

    <main class="stage scroll">
      {#key pos}
        <section class="screen" class:narrow={isCheck || cur.kind !== 'screen'}>
          {#if cur.kind === 'screen' && screen}
            {#if screen.kind === 'check' && screen.question}
              <div class="eyebrow blue">Quick check</div>
              <QuestionCard bind:this={card} q={screen.question} retry bind:canCheck bind:result />
            {:else}
              <div class="eyebrow"><Md inline src={lesson.title} /> · {teachingNo} of {teachingTotal}</div>
              <h1>{screen.heading}</h1>
              {#if screen.kind === 'compare' && screen.compare}
                <div class="compare">
                  {#each [screen.compare.left, screen.compare.right] as side, i}
                    <div class="side">
                      <span class="side-label" class:alt={i === 1}>{side.label}</span>
                      <CodeView code={side.code} small />
                      <Md src={side.caption_md} cls="caption" />
                    </div>
                  {/each}
                </div>
                {#if screen.text_md}<Md src={screen.text_md} />{/if}
              {:else}
                <div class="split">
                  <div class="col">
                    {#if screen.kind === 'stepper' && stepData}
                      <div class="caption card" aria-live="polite"><span class="stepno">Step {step} of {screen.steps.length}</span><Md src={stepData.caption_md} /></div>
                    {/if}
                    {#if screen.text_md}<div class="card text"><Md src={screen.text_md} /></div>{/if}
                    {#if Object.keys(screen.coming_from).length}<ComingFrom cards={screen.coming_from} />{/if}
                  </div>
                  <div class="col wide">
                    {#if screen.code}
                      <CodeView code={screen.code} hl={stepData?.hl ?? []} err={stepData?.err.length ? stepData.err : errLines} notes={stepData?.notes ?? []} links={stepData?.links ?? []} />
                    {:else if screen.files?.length}
                      <CodeFiles files={screen.files} file={stepData?.file ?? diagnosticFile(screen.diagnostic)} hl={stepData?.hl ?? []} err={stepData?.err.length ? stepData.err : errLines} notes={stepData?.notes ?? []} />
                    {/if}
                    {#if screen.kind === 'error' && screen.diagnostic}
                      <ErrorExplainer diagnostic={screen.diagnostic} annotations={screen.annotations} />
                    {/if}
                    {#if panels.length}<Panels {panels} />{/if}
                  </div>
                </div>
              {/if}
            {/if}
          {:else if cur.kind === 'summary'}
            <div class="eyebrow">Summary</div>
            <h1>What you learned</h1>
            <ul class="summary-list">
              {#each lesson.summary as s}<li><Md src={s} inline /></li>{/each}
            </ul>
          {:else}
            <div class="done">
              <div class="medal">★</div>
              <h1>Lesson complete!</h1>
              <p class="muted">{tries === 0 ? lesson.completion.first_try : lesson.completion.many_tries.replace('{tries}', String(tries + 1))}</p>
              <div class="stats">
                <div class="stat"><div class="k">XP</div><div class="v">+{award?.xp ?? 0}</div></div>
                <div class="stat s2"><div class="k">Wrong tries</div><div class="v">{tries}</div></div>
                <div class="stat s3"><div class="k">Minutes</div><div class="v">~{lesson.minutes}</div></div>
              </div>
              <div class="done-actions">
                <a class="btn primary" href="#/questions/{chapter}/{nextSet}">{lesson.completion.button || 'Continue'}</a>
                <a class="btn ghost" href="#/">Back to the path</a>
                <button class="btn ghost" onclick={replay}>Replay lesson</button>
              </div>
            </div>
          {/if}
        </section>
      {/key}
    </main>

    {#if cur.kind !== 'done'}
      <footer class="footer-bar" class:right-state={result?.correct} class:wrong-state={result && !result.correct}>
        {#if result}
          <div class="feedback">
            <div class="icon">{result.correct ? '✓' : '✕'}</div>
            <div>
              <div class="title">{result.correct ? 'Correct!' : 'Not quite'}</div>
              <div class="body"><Md src={card?.feedback() ?? ''} inline /></div>
            </div>
          </div>
          {#if result.correct}
            <button class="btn go" onclick={next}>Continue</button>
          {:else}
            <button class="btn stop" onclick={retry}>Try again</button>
          {/if}
        {:else}
          <button class="btn ghost" onclick={back} disabled={pos === 0}>Back</button>
          {#if isCheck}
            <button class="btn blue" onclick={check} disabled={!canCheck}>Check</button>
          {:else}
            <button class="btn primary" onclick={next}>{screen?.kind === 'stepper' && step < (screen?.steps.length ?? 0) ? 'Next step' : cur.kind === 'summary' ? 'Finish' : 'Continue'}</button>
          {/if}
        {/if}
      </footer>
    {/if}
  </div>
  {#if readMore}<BookPanel files={lesson.read_more} onclose={() => (readMore = false)} />{/if}
{:else}
  <div class="center muted">Loading…</div>
{/if}

<style>
  .player { height: 100%; display: flex; flex-direction: column; }
  .top { display: flex; align-items: center; gap: 20px; padding: 16px 32px; background: var(--surface); border-bottom: 1px solid var(--line); }
  .close { width: 40px; height: 40px; border-radius: 12px; color: var(--muted); display: flex; align-items: center; justify-content: center; text-decoration: none; font-size: 18px; }
  .close:hover { background: var(--surface-2); color: var(--ink); }
  .top .progress { flex: 1; }
  .label { font-size: 14px; font-weight: 700; color: var(--muted); white-space: nowrap; }
  .stage { flex: 1; }
  .screen { max-width: 1280px; margin: 0 auto; padding: 32px; display: flex; flex-direction: column; gap: 22px; animation: screen-in .35s ease; }
  .screen.narrow { max-width: 780px; }
  @keyframes screen-in { from { opacity: 0; transform: translateY(6px); } to { opacity: 1; transform: none; } }
  h1 { margin: -12px 0 0; font-size: 30px; font-weight: 800; line-height: 1.2; }
  .eyebrow.blue { color: var(--blue); }
  .split { display: flex; flex-wrap: wrap; gap: 24px; align-items: flex-start; }
  .col { flex: 1 1 380px; min-width: 0; display: flex; flex-direction: column; gap: 16px; }
  .col.wide { flex-basis: 520px; }
  .card.text :global(.prose) { font-size: 17px; }
  .caption { border-left: 5px solid var(--rust); }
  .caption :global(.prose) { font-size: 18px; color: var(--ink); }
  .stepno { display: block; font-size: 12px; font-weight: 800; color: var(--muted); text-transform: uppercase; letter-spacing: .06em; margin-bottom: 6px; }
  .compare { display: grid; grid-template-columns: repeat(auto-fit, minmax(320px, 1fr)); gap: 18px; }
  .side { display: flex; flex-direction: column; gap: 10px; }
  .side-label { align-self: flex-start; font-size: 13px; font-weight: 800; padding: 4px 10px; border-radius: 8px; background: var(--surface-2); color: var(--ink-3); }
  .side-label.alt { background: var(--green-soft); color: var(--green-dark); }
  .side :global(.caption) { font-size: 15px; color: var(--ink-3); }
  .summary-list { margin: 0; padding: 0; list-style: none; display: flex; flex-direction: column; gap: 12px; }
  .summary-list li { display: flex; gap: 12px; align-items: flex-start; background: var(--surface); border: 1px solid var(--line); border-radius: 14px; padding: 14px 16px; font-size: 16px; line-height: 1.5; }
  .summary-list li::before { content: "✓"; flex: 0 0 26px; height: 26px; border-radius: 13px; background: var(--green-soft); color: var(--green); font-weight: 800; display: flex; align-items: center; justify-content: center; }
  .done { display: flex; flex-direction: column; align-items: center; text-align: center; gap: 18px; padding-top: 30px; max-width: 520px; margin: 0 auto; }
  .done h1 { margin: 0; }
  .medal { width: 96px; height: 96px; border-radius: 28px; background: var(--gold); box-shadow: 0 8px 0 var(--gold-dark); display: flex; align-items: center; justify-content: center; color: #fff; font-size: 48px; animation: pop .5s ease; }
  .stats { width: 100%; display: grid; grid-template-columns: repeat(3, minmax(0, 1fr)); gap: 12px; }
  .stat { background: var(--surface); border: 2px solid var(--gold); border-radius: 16px; padding: 14px 8px; }
  .stat .k { font-size: 12px; font-weight: 800; color: var(--gold-ink); text-transform: uppercase; letter-spacing: .06em; }
  .stat .v { font-size: 24px; font-weight: 800; }
  .stat.s2 { border-color: #E0701E; } .stat.s2 .k { color: var(--link); }
  .stat.s3 { border-color: var(--green); } .stat.s3 .k { color: var(--green); }
  .done-actions { width: 100%; display: flex; flex-direction: column; gap: 12px; }
  .done-actions .btn { width: 100%; }
  .center { height: 100%; display: flex; flex-direction: column; gap: 16px; align-items: center; justify-content: center; }
</style>
