<script lang="ts">
  // One question of any of the nine types. The parent drives it: `canCheck`, then
  // `check()`, then `reset()` for another try.
  import { api } from '../lib/api'
  import type { Checked } from '../lib/bindings/Checked'
  import type { Question } from '../lib/bindings/Question'
  import type { Response } from '../lib/bindings/Response'
  import CodeView from './CodeView.svelte'
  import Md from './Md.svelte'

  let {
    q,
    record = false,
    retry = false,
    canCheck = $bindable(false),
    result = $bindable(null),
  }: { q: Question; record?: boolean; retry?: boolean; canCheck?: boolean; result?: Checked | null } = $props()

  let keys = $state<string[]>([])
  let text = $state('')
  let blanks = $state<string[]>([])
  let order = $state<string[]>([])
  let lines = $state<number[]>([])
  let pairs = $state<[string, string][]>([])
  let leftPick = $state<string | null>(null)

  const blankCount = $derived(q.answer.kind === 'blanks' ? q.answer.blanks.length : 0)
  const choice = $derived(['single', 'multi', 'true_false', 'compiles'].includes(q.kind))
  const checked = $derived(result !== null)
  // With retries (lesson quick checks), a wrong answer doesn't give the right one away.
  const reveal = $derived(result !== null && (!retry || result.correct))

  $effect(() => {
    if (blanks.length !== blankCount) blanks = Array.from({ length: blankCount }, () => '')
  })

  $effect(() => {
    canCheck =
      !checked &&
      (choice ? keys.length > 0
        : q.kind === 'predict_output' ? text.trim().length > 0
        : q.kind === 'fill_blank' ? blanks.length > 0 && blanks.every(b => b.trim())
        : q.kind === 'arrange' ? order.length === q.options.length
        : q.kind === 'spot_bug' ? lines.length > 0
        : q.kind === 'match_pairs' ? pairs.length === q.left.length
        : false)
  })

  function response(): Response {
    switch (q.kind) {
      case 'predict_output': return { kind: 'text', text }
      case 'fill_blank': return { kind: 'blanks', blanks }
      case 'arrange': return { kind: 'order', order }
      case 'spot_bug': return { kind: 'lines', lines }
      case 'match_pairs': return { kind: 'pairs', pairs }
      default: return { kind: 'keys', keys }
    }
  }

  export async function check(): Promise<Checked> {
    result = await api.checkAnswer(q.id, response(), record)
    return result
  }

  export function reset() {
    result = null
    keys = []
    text = ''
    blanks = Array.from({ length: blankCount }, () => '')
    order = []
    lines = []
    pairs = []
    leftPick = null
  }

  function pickKey(k: string) {
    if (checked) return
    if (q.kind === 'multi') keys = keys.includes(k) ? keys.filter(x => x !== k) : [...keys, k]
    else keys = [k]
  }

  function pickLine(n: number) {
    if (checked) return
    lines = lines.includes(n) ? lines.filter(x => x !== n) : [...lines, n]
  }

  function pickLeft(k: string) {
    if (checked) return
    pairs = pairs.filter(p => p[0] !== k)
    leftPick = k
  }

  function pickRight(k: string) {
    if (checked || !leftPick) return
    pairs = [...pairs.filter(p => p[1] !== k), [leftPick, k]]
    leftPick = null
  }

  const correctKeys = $derived(q.answer.kind === 'keys' ? q.answer.keys : [])
  const correctLines = $derived(q.answer.kind === 'lines' ? q.answer.lines : [])
  const correctPairs = $derived(q.answer.kind === 'pairs' ? q.answer.pairs : [])
  const pairColor = (left: string) => ['#2457C5', '#C2461B', '#1B7A43', '#6D4BC3', '#9A6A00', '#C8372D'][Math.max(0, q.left.findIndex(l => l.key === left)) % 6]

  /** What the learner should read after checking. */
  const explanation = $derived.by(() => {
    if (!result) return ''
    if (choice) {
      const picked = keys.map(k => q.explanations.find(e => e.key === k)?.text_md).filter(Boolean)
      return picked.join('\n\n')
    }
    if (result.matched_wrong != null && q.wrong_answers[result.matched_wrong]) return q.wrong_answers[result.matched_wrong].why_md
    return q.general_md ?? ''
  })

  export function feedback(): string {
    return explanation
  }

  const answerText = $derived.by(() => {
    const a = q.answer
    if (a.kind === 'text') return a.text
    if (a.kind === 'blanks') return a.blanks.map((b, i) => (a.blanks.length > 1 ? `(${i + 1}) ` : '') + b[0]).join('   ')
    if (a.kind === 'orders') return a.orders[0].map(k => q.options.find(o => o.key === k)?.text_md ?? k).join('\n')
    if (a.kind === 'lines') return a.lines.map(l => `line ${l}`).join(', ')
    return ''
  })
</script>

<div class="q">
  <Md src={q.prompt_md} cls="prompt" />
  {#if q.code && q.kind !== 'spot_bug'}
    <CodeView code={q.code} />
  {/if}

  {#if choice}
    {#if q.kind === 'multi'}<div class="multi-note">Select all that apply.</div>{/if}
    <div class="options" role={q.kind === 'multi' ? 'group' : 'radiogroup'}>
      {#each q.options as o}
        <button
          class="opt"
          class:right={reveal && correctKeys.includes(o.key)}
          class:wrong={checked && keys.includes(o.key) && !correctKeys.includes(o.key)}
          role={q.kind === 'multi' ? 'checkbox' : 'radio'}
          aria-checked={keys.includes(o.key)}
          disabled={checked}
          onclick={() => pickKey(o.key)}
        ><span class="key">{o.key}</span><Md src={o.text_md} inline /></button>
      {/each}
    </div>
  {:else if q.kind === 'predict_output'}
    <textarea class="answer" placeholder="Type exactly what the program prints" bind:value={text} disabled={checked} spellcheck="false"></textarea>
  {:else if q.kind === 'fill_blank'}
    <div class="blanks">
      {#each blanks as _, i}
        <label class="blank">{#if blankCount > 1}<span>({i + 1})</span>{/if}<input class="answer" bind:value={blanks[i]} disabled={checked} spellcheck="false" autocomplete="off" placeholder="Your answer" /></label>
      {/each}
    </div>
  {:else if q.kind === 'arrange'}
    <div class="arrange">
      <div class="zone">
        <div class="zone-label">Your order</div>
        {#if order.length === 0}<div class="zone-empty">Click the lines below in the right order.</div>{/if}
        {#each order as k, i}
          <button class="chip placed" disabled={checked} onclick={() => (order = order.filter(x => x !== k))}><span class="n">{i + 1}</span><Md src={q.options.find(o => o.key === k)?.text_md} inline /></button>
        {/each}
      </div>
      <div class="pool">
        {#each q.options.filter(o => !order.includes(o.key)) as o}
          <button class="chip" disabled={checked} onclick={() => (order = [...order, o.key])}><Md src={o.text_md} inline /></button>
        {/each}
      </div>
    </div>
  {:else if q.kind === 'spot_bug' && q.code}
    <CodeView code={q.code} onpick={checked ? undefined : pickLine} picked={lines} right={reveal ? correctLines : []} />
  {:else if q.kind === 'match_pairs'}
    <div class="match">
      <div class="col">
        {#each q.left as l}
          {@const p = pairs.find(x => x[0] === l.key)}
          <button class="chip" class:sel={leftPick === l.key} style={p ? `border-color:${pairColor(l.key)}` : ''} disabled={checked} onclick={() => pickLeft(l.key)}>
            <span class="n" style={p ? `background:${pairColor(l.key)}` : ''}>{l.key}</span><Md src={l.text_md} inline />
          </button>
        {/each}
      </div>
      <div class="col">
        {#each q.options as r}
          {@const p = pairs.find(x => x[1] === r.key)}
          <button class="chip" style={p ? `border-color:${pairColor(p[0])}` : ''} disabled={checked || !leftPick} onclick={() => pickRight(r.key)}>
            <span class="n" style={p ? `background:${pairColor(p[0])}` : ''}>{p ? p[0] : r.key}</span><Md src={r.text_md} inline />
          </button>
        {/each}
      </div>
    </div>
    <div class="multi-note">{leftPick ? `Now pick the match for ${leftPick}.` : 'Pick an item on the left, then its match on the right.'}</div>
  {/if}

  {#if result && reveal}
    <div class="after card">
      {#if !result.correct && answerText}
        <div class="right-answer"><strong>Answer:</strong> {#if q.kind === 'predict_output' || q.kind === 'arrange'}<pre>{answerText}</pre>{:else}<code>{answerText}</code>{/if}</div>
      {/if}
      {#if q.kind === 'compiles' && q.verify && !q.verify.compiles && q.verify.error_message}
        <div class="rustc"><span class="muted">rustc says:</span> <code>{q.verify.error_message}</code>{#if q.verify.error_line} <span class="muted">(line {q.verify.error_line})</span>{/if}</div>
      {/if}
      {#if q.kind === 'compiles' && q.verify?.compiles && q.verify.stdout}
        <div class="rustc"><span class="muted">It prints:</span> <code>{q.verify.stdout}</code></div>
      {/if}
      {#if choice}
        <ul class="all-expl">
          {#each q.explanations as e}
            <li class:ok={e.correct}><span class="k">{e.key}</span><Md src={e.text_md} inline /></li>
          {/each}
        </ul>
      {:else if q.kind === 'match_pairs'}
        <ul class="all-expl">
          {#each correctPairs as [l, r]}
            {@const ex = q.explanations.find(e => e.key === `${l}-${r}`)}
            <li class="ok"><span class="k">{l}-{r}</span><Md src={ex?.text_md ?? ''} inline /></li>
          {/each}
        </ul>
      {:else}
        {#if q.general_md}<Md src={q.general_md} />{/if}
        {#if q.fix_md}<div class="fix"><strong>Fix:</strong> <Md src={q.fix_md} inline /></div>{/if}
        {#if q.wrong_answers.length}
          <div class="wrongs">
            <div class="muted small">Common wrong answers</div>
            <ul class="all-expl">
              {#each q.wrong_answers as w, i}
                <li class:hit={result.matched_wrong === i}><Md src={w.answer_md} inline />: <Md src={w.why_md} inline /></li>
              {/each}
            </ul>
          </div>
        {/if}
      {/if}
    </div>
  {/if}
</div>

<style>
  .q { display: flex; flex-direction: column; gap: 18px; }
  .q :global(.prompt) { font-size: 20px; font-weight: 600; color: var(--ink); line-height: 1.5; }
  .multi-note { font-size: 14px; font-weight: 700; color: var(--muted); }
  .blanks { display: flex; flex-direction: column; gap: 10px; }
  .blank { display: flex; align-items: center; gap: 10px; font-weight: 800; color: var(--muted); }
  .arrange { display: flex; flex-direction: column; gap: 14px; }
  .zone { min-height: 80px; border: 2px dashed var(--line-2); border-radius: 14px; padding: 12px; display: flex; flex-direction: column; gap: 8px; }
  .zone-label { font-size: 12px; font-weight: 800; color: var(--muted); text-transform: uppercase; letter-spacing: .06em; }
  .zone-empty { color: var(--muted); font-size: 14px; }
  .pool { display: flex; flex-direction: column; gap: 8px; }
  .chip { display: flex; align-items: center; gap: 10px; text-align: left; padding: 10px 14px; border-radius: 12px; border: 2px solid var(--line-2); background: var(--surface); box-shadow: 0 3px 0 var(--line-2); font-size: 15px; cursor: pointer; font-family: var(--mono); }
  .chip:disabled { cursor: default; }
  .chip.sel { border-color: var(--blue); background: var(--blue-soft); }
  .chip .n { flex: 0 0 24px; height: 24px; border-radius: 12px; background: var(--ink-3); color: var(--surface); font: 800 12px/24px var(--sans); text-align: center; }
  .match { display: grid; grid-template-columns: 1fr 1fr; gap: 14px; }
  .match .col { display: flex; flex-direction: column; gap: 8px; }
  .match .chip { font-family: var(--sans); }
  .after { display: flex; flex-direction: column; gap: 12px; }
  .right-answer pre { margin: 6px 0 0; font-family: var(--mono); background: var(--surface-2); padding: 10px 14px; border-radius: 10px; }
  .all-expl { list-style: none; padding: 0; margin: 0; display: flex; flex-direction: column; gap: 8px; font-size: 15px; line-height: 1.5; color: var(--ink-2); }
  .all-expl li { display: flex; gap: 10px; align-items: baseline; }
  .all-expl .k { flex: 0 0 auto; font-weight: 800; color: var(--red-dark); min-width: 28px; }
  .all-expl li.ok .k { color: var(--green-dark); }
  .all-expl li.hit { background: var(--red-soft); border-radius: 8px; padding: 4px 8px; }
  .small { font-size: 13px; margin-bottom: 6px; }
  .rustc { font-size: 14px; }
</style>
