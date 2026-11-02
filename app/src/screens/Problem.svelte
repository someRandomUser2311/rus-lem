<script lang="ts">
  // The coding screen: problem on the left, editor in the middle, tests and compiler
  // output on the right. Cargo problems get file tabs (the project screen).
  import { onDestroy } from 'svelte'
  import { api, errorText } from '../lib/api'
  import { ansiToHtml } from '../lib/ansi'
  import type { GradeOutcome } from '../lib/bindings/GradeOutcome'
  import type { ProblemView } from '../lib/bindings/ProblemView'
  import type { SourceFile } from '../lib/bindings/SourceFile'
  import type { TestResult } from '../lib/bindings/TestResult'
  import { anyAssist, startLsp, type LspSession } from '../lib/lsp'
  import { app } from '../lib/state.svelte'
  import Editor from '../components/Editor.svelte'
  import Md from '../components/Md.svelte'

  let { chapter, id, review }: { chapter: string; id: string; review?: number } = $props()

  let p = $state<ProblemView | null>(null)
  let error = $state<string | null>(null)
  let files = $state<SourceFile[]>([])
  let active = $state('')
  let leftTab = $state<'problem' | 'hints' | 'solution'>('problem')
  let rightTab = $state<'tests' | 'output'>('tests')
  let running = $state<null | 'run' | 'submit'>(null)
  let phase = $state('')
  let compileOut = $state('')
  let liveTests = $state<TestResult[]>([])
  let outcome = $state<GradeOutcome | null>(null)
  let lsp = $state<LspSession | null>(null)
  let lspError = $state<string | null>(null)
  let showAccepted = $state(false)
  // Checkpoint problems: the part being worked on and graded.
  let part = $state<number | null>(null)
  let now = $state(Date.now())
  let editor: Editor | undefined = $state()
  let saveTimer: ReturnType<typeof setTimeout> | undefined

  const tick = setInterval(() => (now = Date.now()), 1000)
  onDestroy(() => {
    clearInterval(tick)
    clearTimeout(saveTimer)
    lsp?.stop()
  })

  $effect(() => {
    void id
    load()
  })

  async function load() {
    try {
      const v = await api.problem(id)
      p = v
      // A draft is kept while it still has every starter file (it may add module files).
      const draft = v.draft && v.starter.every(s => v.draft!.some(d => d.path === s.path)) ? v.draft : null
      files = (draft ?? v.starter).map(f => ({ ...f }))
      part = v.checkpoints ? firstOpenPart(v) : null
      active = files[0]?.path ?? ''
      outcome = null
      liveTests = []
      compileOut = ''
      hintLoadedAt = Date.now()
      editor?.load(files)
      await connectLsp()
    } catch (e) {
      error = errorText(e)
    }
  }

  // rust-analyzer runs only when an assist is on and it's installed.
  const assists = $derived({
    assistCompletion: app.settings.assistCompletion,
    assistHover: app.settings.assistHover,
    assistDiagnostics: app.settings.assistDiagnostics,
    assistSignature: app.settings.assistSignature,
    assistDefinition: app.settings.assistDefinition,
  })
  const raInstalled = $derived(app.info?.toolchain.rust_analyzer ?? false)

  async function connectLsp() {
    lsp?.stop()
    lsp = null
    lspError = null
    if (!p || !raInstalled) return
    const needsServer = assists.assistHover || assists.assistDiagnostics || assists.assistSignature || assists.assistDefinition || assists.assistCompletion
    if (!needsServer || !anyAssist(assists)) {
      void api.lspStop()
      return
    }
    try {
      lsp = await startLsp(p.id, files, assists)
    } catch (e) {
      lspError = errorText(e)
    }
  }

  function onchange(next: SourceFile[]) {
    files = next
    clearTimeout(saveTimer)
    saveTimer = setTimeout(() => void api.saveDraft(id, files), 800)
  }

  /** The first part not passed yet (the last part once all are). */
  function firstOpenPart(v: ProblemView): number {
    for (let n = 1; n <= v.parts.length; n++) if (!v.state.checkpoints.includes(n)) return n
    return v.parts.length
  }

  const partTests = $derived(p ? p.visible_tests.filter(t => !p!.checkpoints || t.part === part) : [])
  const partHidden = $derived(p ? (p.checkpoints && part ? (p.hidden_per_part[part - 1] ?? 0) : p.hidden_count) : 0)
  const lastPart = $derived(!p?.checkpoints || part === p.parts.length)

  async function grade(submit: boolean) {
    if (!p || running || (!submit && partTests.length === 0)) return
    running = submit ? 'submit' : 'run'
    phase = 'Starting'
    compileOut = ''
    liveTests = []
    outcome = null
    rightTab = 'tests'
    try {
      const o = await api.grade(p.id, files, submit, review ?? null, part, ev => {
        if (ev.event === 'phase') phase = ev.text
        else if (ev.event === 'compiled') {
          compileOut = ev.compile.rendered + ev.compile.log
          if (!ev.compile.ok) rightTab = 'output'
        } else if (ev.event === 'test') liveTests = [...liveTests, ev.result]
      })
      outcome = o
      p = o.problem
      compileOut = o.result.compile.rendered + o.result.compile.log
      if (o.result.verdict === 'compile_error') rightTab = 'output'
      app.celebrate({ xp: o.award.xp + o.review_xp, badges: o.award.badges })
      if (o.result.verdict === 'accepted') {
        showAccepted = true
        await app.refresh()
      }
    } catch (e) {
      app.toast({ kind: 'error', title: 'Could not run your code', body: errorText(e) })
    } finally {
      running = null
    }
  }

  // Multi-file Cargo problems: the learner creates module files (`src/text.rs`) and may
  // delete the ones they created; starter files stay. Test-writing problems: test files
  // under `tests/` instead.
  const testWriting = $derived(!!p && p.subject.length > 0)
  const canAddFiles = $derived(!!p && p.cargo && (p.kind === 'multi-file' || testWriting))
  let newPath = $state<string | null>(null)
  let newPathError = $state('')
  const MODULE_PATH = /^src\/([a-z_][a-z0-9_]*\/){0,3}[a-z_][a-z0-9_]*\.rs$/
  const TEST_PATH = /^tests\/([a-z_][a-z0-9_]*\/)?[a-z_][a-z0-9_]*\.rs$/
  // Test-writing problems: the supplied code at the top of a file is read-only.
  const locked = $derived.by(() => {
    const out: Record<string, { len: number; all: boolean }> = {}
    for (const s of p?.subject ?? []) {
      const starter = p?.starter.find(f => f.path === s.path)
      out[s.path] = { len: s.text.length, all: !starter || starter.text === s.text }
    }
    return out
  })

  function addFile() {
    if (newPath === null) return
    const path = newPath.trim()
    if (testWriting ? !TEST_PATH.test(path) : !MODULE_PATH.test(path)) {
      newPathError = testWriting
        ? 'Use a path like tests/shelf.rs or tests/common/mod.rs (lowercase letters, digits and _).'
        : 'Use a path like src/text.rs or src/fees/rate.rs (lowercase letters, digits and _).'
      return
    }
    if (files.some(f => f.path === path)) {
      newPathError = `${path} already exists.`
      return
    }
    files = [...files, { path, text: '' }].sort((a, b) => a.path.localeCompare(b.path))
    newPath = null
    newPathError = ''
    editor?.load(files)
    active = path
    void api.saveDraft(id, files)
    if (lsp) void connectLsp()
  }

  async function removeFile(path: string) {
    if (!(await app.ask({ title: `Delete ${path}?`, body: 'Its code will be lost.', yes: 'Delete file', danger: true }))) return
    files = files.filter(f => f.path !== path)
    if (active === path) active = files[0]?.path ?? ''
    editor?.load(files)
    void api.saveDraft(id, files)
    if (lsp) void connectLsp()
  }

  async function resetCode() {
    if (!p || !(await app.ask({ title: 'Start over?', body: 'Your code will be replaced with the starter code. Your current code will be lost.', yes: 'Replace my code', danger: true }))) return
    files = p.starter.map(f => ({ ...f }))
    editor?.load(files)
    void api.saveDraft(id, files)
  }

  let hintLoadedAt = Date.now()
  async function openHint(n: number) {
    if (!p) return
    p = await api.openHint(p.id, n)
    hintLoadedAt = Date.now()
  }
  const hintWait = $derived(p?.state.next_hint_in ? Math.max(0, p.state.next_hint_in - Math.floor((now - hintLoadedAt) / 1000)) : 0)

  const tests = $derived(outcome ? outcome.result.tests : liveTests)
  const passed = $derived(tests.filter(t => t.status === 'passed').length)
  const verdictText: Record<string, string> = {
    accepted: 'Accepted', passed: 'Visible tests passed', compile_error: 'Compile error', wrong_answer: 'Wrong answer',
    rule_broken: 'A rule was broken', time_limit: 'Time limit exceeded', runtime_error: 'Runtime error', cancelled: 'Cancelled',
  }
  const verdictGood = $derived(outcome && (outcome.result.verdict === 'accepted' || outcome.result.verdict === 'passed'))
  const show = (s: string) => s.replace(/\t/g, '→').replace(/ /g, '·')

  const next = $derived(app.nextAfter(id))
  const nextNode = $derived(next?.n ?? null)
  const nodeHref = (n: { kind: string; id: string }) => {
    const c = next?.ch.id ?? chapter
    return n.kind === 'lesson' ? `#/lesson/${c}/${n.id}` : n.kind === 'questions' || n.kind === 'practice' ? `#/questions/${c}/${n.id}` : n.kind === 'review' ? `#/review/${c}` : `#/problem/${c}/${n.id}`
  }

  function onkey(e: KeyboardEvent) {
    if ((e.metaKey || e.ctrlKey) && e.key === "'") {
      e.preventDefault()
      void grade(false)
    }
    if ((e.metaKey || e.ctrlKey) && e.key === 'Enter') {
      e.preventDefault()
      void grade(true)
    }
  }
</script>

<svelte:window onkeydown={onkey} />

{#if error}
  <div class="center"><p>{error}</p><a class="btn ghost" href="#/">Back to the path</a></div>
{:else if p}
  <div class="coding">
    <header class="bar">
      <a class="back" href={review ? `#/review/${chapter}` : '#/'}>← {review ? 'Review' : 'Path'}</a>
      <span class="tag level-{p.level}">{p.level}</span>
      <h1>{p.id} · <Md inline src={p.title} /></h1>
      {#if p.state.accepted}<span class="solved">✓ Solved</span>{/if}
      <span class="spacer"></span>
      <span class="xp">{p.xp} XP</span>
      {#if p.checkpoints}
        <div class="parts" role="tablist" aria-label="Project parts">
          {#each p.parts as title, i}
            <button role="tab" class:on={part === i + 1} class:passed={p.state.checkpoints.includes(i + 1)} aria-selected={part === i + 1} onclick={() => { part = i + 1; outcome = null; liveTests = [] }} disabled={!!running} title={title}>
              {#if p.state.checkpoints.includes(i + 1)}✓ {/if}Part {i + 1}
            </button>
          {/each}
        </div>
      {/if}
      <button class="btn ghost small" onclick={resetCode} disabled={!!running}>Reset code</button>
      <button class="btn ghost small" onclick={() => grade(false)} disabled={!!running || partTests.length === 0} title={partTests.length ? "Run the visible tests (⌘')" : 'This part has no visible tests; Submit checks it'}>{#if running === 'run'}<span class="spinner"></span>{/if}Run</button>
      <button class="btn go small" onclick={() => grade(true)} disabled={!!running} title="Run every test (⌘↵)">{#if running === 'submit'}<span class="spinner"></span>{/if}Submit</button>
      {#if running}<button class="btn ghost small" onclick={() => api.cancelGrade()}>Stop</button>{/if}
    </header>

    <div class="cols">
      <!-- problem -->
      <section class="left">
        <nav class="tabs">
          <button class:on={leftTab === 'problem'} onclick={() => (leftTab = 'problem')}>Problem</button>
          <button class:on={leftTab === 'hints'} onclick={() => (leftTab = 'hints')}>Hints {p.state.hints_opened.length}/{p.hint_count}</button>
          <button class:on={leftTab === 'solution'} onclick={() => (leftTab = 'solution')}>Solution{#if !p.state.solution_available} 🔒{/if}</button>
        </nav>
        <div class="scroll pane">
          {#if leftTab === 'problem'}
            <Md src={p.description_md} />
            {#if p.examples_md}<h3>Examples</h3><Md src={p.examples_md} />{/if}
            {#if p.constraints_md}<h3>Constraints</h3><Md src={p.constraints_md} />{/if}
            <div class="meta muted">
              {p.starter_stage} starter · {p.time_limit_ms / 1000} s per test · {p.visible_tests.length} visible and {p.hidden_count} hidden tests
              {#if p.checkpoints}<br />Each part is a checkpoint: pick it above, then Run and Submit check that part's tests. The project is solved when Part {p.parts.length} is Accepted.{/if}
            </div>
          {:else if leftTab === 'hints'}
            <p class="muted">Hints open one at a time and cost nothing. The next one opens 30 seconds after the last, or after one more submit.</p>
            {#each p.hints as h, i}
              <div class="hint card">
                <div class="hint-head">Hint {i + 1}</div>
                {#if h}<Md src={h} />
                {:else if p.state.next_hint === i + 1}
                  {#if p.state.next_hint_in === 0 || hintWait === 0}
                    <button class="btn blue small" onclick={() => openHint(i + 1)}>Show hint {i + 1}</button>
                  {:else if p.state.next_hint_in == null}
                    <span class="muted">Submit once more to unlock it.</span>
                  {:else}
                    <span class="muted">Available in {hintWait} s, or after your next submit.</span>
                  {/if}
                {:else}<span class="muted">Locked</span>{/if}
              </div>
            {/each}
          {:else}
            {#if p.solution}
              {#if p.solution_approach_md}<h3>Approach</h3><Md src={p.solution_approach_md} />{/if}
              {#each p.solution as f}
                <h3><code>{f.path}</code></h3>
                <pre class="sol">{f.text}</pre>
              {/each}
              {#if p.idiomatic_md}<div class="callout"><strong>Idiomatic note:</strong> <Md src={p.idiomatic_md} inline /></div>{/if}
            {:else}
              <p>The reference solution unlocks when your code is <strong>Accepted</strong>, or after <strong>5 failed submits</strong> and all {p.hint_count} hints.</p>
              <p class="muted">So far: {p.state.failed_submits} failed submit{p.state.failed_submits === 1 ? '' : 's'}, {p.state.hints_opened.length} of {p.hint_count} hints.</p>
            {/if}
          {/if}
        </div>
      </section>

      <!-- editor -->
      <section class="mid">
        <nav class="tabs files">
          {#each files as f}
            <span class="filetab" class:on={active === f.path}>
              <button class:on={active === f.path} onclick={() => (active = f.path)}>{f.path}{#if locked[f.path]?.all}<span class="ro" title="Supplied, read-only"> · read-only</span>{/if}</button>
              {#if canAddFiles && !p?.starter.some(s => s.path === f.path)}
                <button class="del" title="Delete {f.path}" aria-label="Delete {f.path}" disabled={!!running} onclick={() => removeFile(f.path)}>×</button>
              {/if}
            </span>
          {/each}
          {#if canAddFiles}
            {#if newPath === null}
              <button class="add" title={testWriting ? 'Create a test file' : 'Create a module file'} disabled={!!running} onclick={() => { newPath = testWriting ? 'tests/' : 'src/'; newPathError = '' }}>+ New file</button>
            {:else}
              <!-- svelte-ignore a11y_autofocus -->
              <input
                class="newpath"
                bind:value={newPath}
                autofocus
                spellcheck="false"
                aria-label="New file path"
                placeholder={testWriting ? 'tests/shelf.rs' : 'src/text.rs'}
                onkeydown={e => { if (e.key === 'Enter') addFile(); else if (e.key === 'Escape') { newPath = null; newPathError = '' } }}
              />
              <button class="add" onclick={addFile}>Create</button>
              {#if newPathError}<span class="patherr" role="alert">{newPathError}</span>{/if}
            {/if}
          {/if}
          <span class="spacer"></span>
          {#if lsp}<span class="ra on" title="rust-analyzer is running">rust-analyzer</span>{:else if lspError}<span class="ra" title={lspError}>rust-analyzer unavailable</span>{/if}
        </nav>
        {#if testWriting}
          <p class="locknote">{locked[active]?.all ? 'This file is supplied and read-only.' : locked[active] ? 'The shaded code at the top is supplied and read-only. Write your tests below it.' : 'Write your tests in this file.'}</p>
        {/if}
        <div class="editor-wrap">
          {#key lsp}
            <Editor bind:this={editor} {files} bind:active completion={app.settings.assistCompletion} {lsp} {onchange} {locked} />
          {/key}
        </div>
      </section>

      <!-- results -->
      <section class="right">
        <nav class="tabs">
          <button class:on={rightTab === 'tests'} onclick={() => (rightTab = 'tests')}>Tests</button>
          <button class:on={rightTab === 'output'} onclick={() => (rightTab = 'output')}>Compiler output{#if compileOut} •{/if}</button>
        </nav>
        <div class="scroll pane">
          {#if running}
            <div class="status"><span class="spinner"></span> {phase}…</div>
          {/if}
          {#if outcome}
            <div class="verdict" class:good={verdictGood}>
              <div class="v-title">{verdictText[outcome.result.verdict]}</div>
              <div class="v-sub">{passed} of {tests.length} tests passed · {outcome.result.ms} ms</div>
            </div>
          {/if}
          {#if rightTab === 'output'}
            {#if compileOut}<pre class="ansi">{@html ansiToHtml(compileOut)}</pre>
            {:else}<p class="muted">Run your code to see the compiler's messages here, in full.</p>{/if}
          {:else if tests.length}
            <ul class="tests">
              {#each tests as t (t.key)}
                <li class="t {t.status}">
                  <div class="t-head">
                    <span class="mark">{t.status === 'passed' ? '✓' : t.status === 'not_run' ? '–' : '✕'}</span>
                    <span>Test {t.n}</span>
                    <span class="cat">{t.visible ? t.category : `hidden · ${t.category}`}</span>
                  </div>
                  {#if t.status !== 'passed'}
                    {#if t.message}<div class="msg"><Md src={t.message} inline /></div>{/if}
                    {#if t.input}<div class="io"><span>Input</span><Md src={t.input} inline /></div>{/if}
                    {#if t.expected != null}<div class="io"><span>Expected</span><pre>{show(t.expected)}</pre></div>{/if}
                    {#if t.actual != null}<div class="io"><span>Your output</span><pre>{show(t.actual)}</pre></div>{/if}
                  {/if}
                </li>
              {/each}
            </ul>
            {#if outcome && p.state.revealed.length}<p class="muted small">Hidden tests that failed twice now show their input.</p>{/if}
          {:else}
            <h3 class="sub">{partTests.length ? 'Visible tests' : 'Hidden tests'}{#if p.checkpoints && part} · Part {part}{#if p.parts[part - 1]}: {p.parts[part - 1]}{/if}{/if}</h3>
            <ul class="tests">
              {#each partTests as t}
                <li class="t">
                  <div class="t-head"><span class="mark">·</span><span>Test {t.n}</span><span class="cat">{t.category}</span></div>
                  <div class="io"><span>Input</span><Md src={t.input} inline /></div>
                  <div class="io"><span>Expected</span><Md src={t.expected} inline /></div>
                </li>
              {/each}
            </ul>
            <p class="muted small">{#if partTests.length === 0}This part has only hidden tests: Submit checks all {partHidden}.{:else if partHidden === 0}Run and Submit both check these {partTests.length}.{:else}Run checks these {partTests.length}. Submit also checks {partHidden} hidden {partHidden === 1 ? 'test' : 'tests'}.{/if} Failing costs nothing.</p>
          {/if}
        </div>
      </section>
    </div>
  </div>

  {#if showAccepted && outcome}
    <div class="modal-bg" role="presentation" onclick={() => (showAccepted = false)}></div>
    <div class="modal" role="dialog" aria-label="Accepted">
      <div class="medal">✓</div>
      {#if outcome.result.part != null && p.checkpoints && outcome.result.part < p.parts.length}
        {@const done = outcome.result.part}
        <h2>Part {done} passed!</h2>
        <p class="muted">All {tests.length} tests of Part {done} passed. Keep your code and build Part {done + 1} on top of it.</p>
        <div class="actions">
          <button class="btn primary" onclick={() => { showAccepted = false; part = done + 1; outcome = null; liveTests = []; leftTab = 'problem' }}>Continue to Part {done + 1}{#if p.parts[done]}: {p.parts[done]}{/if}</button>
          <a class="btn ghost" href="#/">Back to the path</a>
        </div>
      {:else}
      <h2>Accepted!</h2>
      <p class="muted">All {tests.length} tests passed.{outcome.award.first_time ? ` +${outcome.award.xp + outcome.review_xp} XP.` : ''}</p>
      <div class="actions">
        {#if nextNode}<a class="btn primary" href={nodeHref(nextNode)} onclick={() => (showAccepted = false)}>Next: <Md inline src={nextNode.title} /></a>{/if}
        <button class="btn ghost" onclick={() => { showAccepted = false; leftTab = 'solution' }}>Compare with the reference solution</button>
        <a class="btn ghost" href="#/">Back to the path</a>
      </div>
      {/if}
    </div>
  {/if}
{:else}
  <div class="center muted">Loading…</div>
{/if}

<style>
  .coding { height: 100%; display: flex; flex-direction: column; }
  .bar { display: flex; align-items: center; gap: 12px; padding: 10px 18px; background: var(--surface); border-bottom: 1px solid var(--line); }
  .bar h1 { font-size: 18px; margin: 0; white-space: nowrap; overflow: hidden; text-overflow: ellipsis; }
  .back { text-decoration: none; font-weight: 700; color: var(--muted); }
  .solved { color: var(--green); font-weight: 800; font-size: 14px; }
  .parts { display: flex; gap: 2px; padding: 3px; border-radius: 10px; background: var(--surface-2); }
  .parts button { border: 0; background: none; padding: 5px 10px; border-radius: 8px; font: inherit; font-size: 13px; font-weight: 700; color: var(--muted); cursor: pointer; white-space: nowrap; }
  .parts button.on { background: var(--surface); color: var(--ink); box-shadow: 0 1px 3px rgba(0, 0, 0, .12); }
  .parts button.passed { color: var(--green); }
  .spacer { flex: 1; }
  .xp { font-weight: 800; color: var(--gold-ink); font-size: 14px; }
  .cols { flex: 1; min-height: 0; display: grid; grid-template-columns: minmax(300px, 1fr) minmax(380px, 1.35fr) minmax(300px, 1fr); }
  section { min-height: 0; display: flex; flex-direction: column; border-right: 1px solid var(--line); background: var(--surface); }
  .mid { background: var(--code-bg); }
  .tabs { display: flex; gap: 4px; padding: 8px 10px; border-bottom: 1px solid var(--line); align-items: center; }
  .tabs button { border: 0; background: transparent; padding: 7px 12px; border-radius: 8px; font-weight: 700; font-size: 14px; color: var(--muted); cursor: pointer; }
  .tabs button.on { background: var(--surface-2); color: var(--ink); }
  .tabs.files { border-color: var(--code-line); }
  .tabs.files button { font-family: var(--mono); font-weight: 500; color: var(--code-head); }
  .tabs.files button.on { background: var(--code-line); color: var(--code-ink); }
  .tabs.files { flex-wrap: wrap; }
  .filetab { display: inline-flex; align-items: center; border-radius: 8px; }
  .filetab.on { background: var(--code-line); }
  .tabs.files .filetab button.del { padding: 4px 8px 4px 0; font-size: 15px; line-height: 1; }
  .tabs.files button.add { font-family: var(--sans); font-weight: 700; color: var(--code-head); border: 1px dashed var(--code-line); }
  .newpath { font: 13px var(--mono); background: var(--code-bg); color: var(--code-ink); border: 1px solid var(--code-line); border-radius: 8px; padding: 6px 8px; width: 180px; }
  .patherr { font-size: 12px; color: #F7768E; }
  .ra { font-size: 11px; color: var(--code-dim); padding-right: 6px; }
  .ra.on { color: #5BD08A; }
  .editor-wrap { flex: 1; min-height: 0; }
  .locknote { margin: 0; padding: 6px 14px; font-size: 12.5px; color: var(--code-dim); border-bottom: 1px solid var(--code-line); }
  .ro { font-family: var(--sans); font-size: 11px; color: var(--code-dim); }
  .pane { flex: 1; padding: 18px 20px 32px; }
  .pane :global(.prose) { font-size: 15.5px; }
  .pane h3 { font-size: 15px; margin: 20px 0 8px; }
  .meta { margin-top: 20px; font-size: 13px; }
  .hint { margin-top: 12px; }
  .hint-head { font-weight: 800; font-size: 13px; text-transform: uppercase; letter-spacing: .06em; color: var(--muted); margin-bottom: 8px; }
  .sol { background: var(--code-bg); color: var(--code-ink); padding: 14px 16px; border-radius: 12px; font-family: var(--mono); font-size: 13px; overflow-x: auto; line-height: 1.6; }
  .callout { background: var(--purple-soft); border-radius: 12px; padding: 12px 14px; margin-top: 12px; font-size: 14px; }
  .status { display: flex; align-items: center; gap: 10px; color: var(--muted); font-weight: 700; margin-bottom: 12px; }
  .verdict { border-radius: 14px; padding: 14px 16px; background: var(--red-soft); color: var(--red-dark); margin-bottom: 14px; animation: pop .3s ease; }
  .verdict.good { background: var(--green-soft); color: var(--green-dark); }
  .v-title { font-size: 20px; font-weight: 800; }
  .v-sub { font-size: 13px; margin-top: 2px; }
  .ansi { background: var(--code-bg); color: var(--code-ink); padding: 14px 16px; border-radius: 12px; font-family: var(--mono); font-size: 12.5px; line-height: 1.55; overflow-x: auto; white-space: pre; }
  .tests { list-style: none; margin: 0; padding: 0; display: flex; flex-direction: column; gap: 8px; }
  .t { border: 1px solid var(--line); border-radius: 12px; padding: 10px 12px; font-size: 14px; }
  .t.passed { border-color: var(--green-soft); }
  .t.failed, .t.time_limit, .t.runtime_error, .t.output_limit { border-color: var(--red); background: var(--red-soft); }
  .t-head { display: flex; gap: 8px; align-items: center; font-weight: 700; }
  .mark { width: 20px; text-align: center; }
  .t.passed .mark { color: var(--green); }
  .t.failed .mark, .t.time_limit .mark, .t.runtime_error .mark { color: var(--red); }
  .cat { margin-left: auto; font-size: 12px; color: var(--muted); font-weight: 600; }
  .msg { margin: 6px 0 0 28px; color: var(--ink-2); white-space: pre-wrap; }
  .io { margin: 6px 0 0 28px; display: flex; flex-direction: column; gap: 2px; }
  .io > span { font-size: 11px; font-weight: 800; text-transform: uppercase; letter-spacing: .06em; color: var(--muted); }
  .io pre { margin: 0; background: var(--surface); border: 1px solid var(--line); border-radius: 8px; padding: 6px 8px; font-family: var(--mono); font-size: 12.5px; white-space: pre-wrap; word-break: break-all; }
  .sub { font-size: 14px; margin: 0 0 10px; }
  .small { font-size: 13px; }
  .modal-bg { position: fixed; inset: 0; background: rgba(0, 0, 0, .35); z-index: 50; }
  .modal { position: fixed; z-index: 51; top: 50%; left: 50%; transform: translate(-50%, -50%); width: min(460px, 90vw); background: var(--surface); border-radius: 24px; padding: 32px; display: flex; flex-direction: column; align-items: center; text-align: center; gap: 10px; box-shadow: 0 20px 60px rgba(0, 0, 0, .3); animation: pop .3s ease; }
  .modal h2 { margin: 6px 0 0; font-size: 28px; }
  .modal .medal { width: 80px; height: 80px; border-radius: 24px; background: var(--green); box-shadow: 0 6px 0 var(--green-dark); color: #fff; font-size: 40px; display: flex; align-items: center; justify-content: center; }
  .actions { display: flex; flex-direction: column; gap: 10px; width: 100%; margin-top: 10px; }
  .center { height: 100%; display: flex; flex-direction: column; gap: 16px; align-items: center; justify-content: center; }
</style>
