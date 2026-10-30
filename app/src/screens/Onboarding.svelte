<script lang="ts">
  // First run: experience, languages, the Rust toolchain check, Cargo warm-up and a daily goal.
  import { api, errorText } from '../lib/api'
  import type { ToolchainStatus } from '../lib/bindings/ToolchainStatus'
  import { app } from '../lib/state.svelte'

  let step = $state(0)
  let experience = $state<'beginner' | 'experienced'>(app.info?.profile.experience === 'experienced' ? 'experienced' : 'beginner')
  let languages = $state<string[]>(app.info?.profile.languages ?? [])
  let comingFrom = $state(app.info?.profile.coming_from ?? 'python')
  let goal = $state(Number(app.info?.profile.daily_goal ?? 30))
  let tc = $state<ToolchainStatus | null>(app.info?.toolchain ?? null)
  let checking = $state(false)
  let warm = $state<'idle' | 'running' | 'ok' | 'failed'>('idle')

  const langs = [
    { id: 'python', name: 'Python' },
    { id: 'js', name: 'JavaScript / TypeScript' },
    { id: 'java', name: 'Java / C#' },
    { id: 'c', name: 'C / C++' },
    { id: 'go', name: 'Go' },
  ]
  const steps = ['Welcome', 'Experience', 'Rust', 'Goal']

  async function recheck() {
    checking = true
    tc = await api.checkToolchain()
    checking = false
  }

  async function warmUp() {
    warm = 'running'
    try {
      warm = (await api.warmUp()) ? 'ok' : 'failed'
    } catch {
      warm = 'failed'
    }
  }

  async function finish() {
    try {
      await api.saveProfile({ experience, languages, coming_from: experience === 'beginner' && languages.length === 0 ? 'python' : comingFrom, daily_goal: goal, onboarded: true })
      await app.load()
      app.go({ name: 'home' })
    } catch (e) {
      app.toast({ kind: 'error', title: 'Could not save', body: errorText(e) })
    }
  }

  function toggleLang(id: string) {
    languages = languages.includes(id) ? languages.filter(l => l !== id) : [...languages, id]
    if (languages.length && !languages.includes(comingFrom)) comingFrom = languages[0]
  }
</script>

<div class="wrap">
  <div class="steps">
    {#each steps as s, i}<span class:on={i <= step}>{s}</span>{/each}
  </div>
  <div class="card panel">
    {#if step === 0}
      <div class="logo">🦀</div>
      <h1>Welcome to Rusticoder</h1>
      <p class="lead">Learn Rust by writing it: short visual lessons from <em>The Rust Programming Language</em>, quick questions, and hundreds of coding problems checked by real tests on your own computer.</p>
      <ul class="points">
        <li>The compiler is your teacher. You'll see its messages in full.</li>
        <li>Failing costs nothing. No hearts, no lost XP. Try as often as you like.</li>
        <li>Everything works offline.</li>
      </ul>
      <button class="btn primary" onclick={() => (step = 1)}>Get started</button>
    {:else if step === 1}
      <h1>How much programming have you done?</h1>
      <div class="options">
        <button class="opt" role="radio" aria-checked={experience === 'beginner'} onclick={() => (experience = 'beginner')}><span class="key">1</span><span><strong>I'm new to programming.</strong> Show me the foundation lessons too.</span></button>
        <button class="opt" role="radio" aria-checked={experience === 'experienced'} onclick={() => (experience = 'experienced')}><span class="key">2</span><span><strong>I already program in another language.</strong> Let me test out of what I know.</span></button>
      </div>
      <h2>Which languages do you know?</h2>
      <p class="muted">Lessons compare Rust with the language you pick. You can change it later.</p>
      <div class="langs">
        {#each langs as l}
          <button class="chip" class:on={languages.includes(l.id)} onclick={() => toggleLang(l.id)}>{l.name}</button>
        {/each}
      </div>
      {#if languages.length > 1}
        <label class="row">Compare with: <select bind:value={comingFrom}>{#each langs.filter(l => languages.includes(l.id)) as l}<option value={l.id}>{l.name}</option>{/each}</select></label>
      {/if}
      <div class="nav"><button class="btn ghost" onclick={() => (step = 0)}>Back</button><button class="btn primary" onclick={() => (step = 2)}>Continue</button></div>
    {:else if step === 2}
      <h1>Rust on your computer</h1>
      <p class="muted">Rusticoder compiles and tests your code with your own Rust installation.</p>
      {#if tc?.found}
        <div class="status ok">✓ Found Rust {tc.rustc_version}{tc.new_enough ? '' : ` (the course needs ${tc.min_rustc} or newer; update with rustup)`}</div>
        <div class="status {tc.rust_analyzer ? 'ok' : 'info'}">
          {tc.rust_analyzer ? '✓ rust-analyzer is installed (optional editor help, off by default).' : 'rust-analyzer isn\'t installed. It is optional: it powers editor help you can turn on in Settings.'}
        </div>
        {#if !tc.rust_analyzer}<pre class="cmd">rustup component add rust-analyzer</pre>{/if}
        <div class="warm">
          {#if warm === 'idle'}<button class="btn ghost small" onclick={warmUp}>Prepare Cargo (first build)</button><span class="muted small">Builds a tiny project once so later Cargo exercises start faster.</span>
          {:else if warm === 'running'}<span class="spinner"></span> Building…
          {:else if warm === 'ok'}<span class="okc">✓ Cargo is ready.</span>
          {:else}<span class="bad">The warm-up build failed. You can still continue.</span>{/if}
        </div>
      {:else}
        <div class="status bad">Rust isn't installed yet.</div>
        <p>Install it with rustup, the official installer. On macOS or Linux, run this in a terminal:</p>
        <pre class="cmd">curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh</pre>
        <p class="muted small">On Windows, download rustup-init.exe from rustup.rs. Then come back and check again. You can also skip this and start with the lessons.</p>
      {/if}
      <div class="nav">
        <button class="btn ghost" onclick={() => (step = 1)}>Back</button>
        <button class="btn ghost" onclick={recheck} disabled={checking}>{checking ? 'Checking…' : 'Check again'}</button>
        <button class="btn primary" onclick={() => (step = 3)}>{tc?.found ? 'Continue' : 'Skip for now'}</button>
      </div>
    {:else}
      <h1>Pick a daily goal</h1>
      <p class="muted">XP you'd like to earn each day. A lesson is 20 XP; problems are 5 to 40.</p>
      <div class="options">
        {#each [[10, 'Casual'], [30, 'Regular'], [50, 'Serious'], [100, 'Intense']] as [xp, name]}
          <button class="opt" role="radio" aria-checked={goal === xp} onclick={() => (goal = Number(xp))}><span class="key">⚡</span><span><strong>{name}</strong> · {xp} XP a day</span></button>
        {/each}
      </div>
      <div class="nav"><button class="btn ghost" onclick={() => (step = 2)}>Back</button><button class="btn primary" onclick={finish}>Start learning</button></div>
    {/if}
  </div>
</div>

<style>
  .wrap { height: 100%; overflow-y: auto; display: flex; flex-direction: column; align-items: center; padding: 40px 20px; gap: 18px; }
  .steps { display: flex; gap: 8px; }
  .steps span { font-size: 12px; font-weight: 800; color: var(--muted); padding: 4px 10px; border-radius: 999px; background: var(--surface-2); }
  .steps span.on { background: var(--rust); color: #fff; }
  .panel { width: min(680px, 100%); padding: 32px; display: flex; flex-direction: column; gap: 16px; }
  h1 { margin: 0; font-size: 28px; }
  h2 { margin: 10px 0 0; font-size: 18px; }
  .logo { font-size: 56px; }
  .lead { font-size: 18px; line-height: 1.6; color: var(--ink-2); margin: 0; }
  .points { margin: 0; padding-left: 20px; line-height: 1.8; color: var(--ink-2); }
  .langs { display: flex; flex-wrap: wrap; gap: 8px; }
  .chip { border: 2px solid var(--line-2); background: var(--surface); border-radius: 12px; padding: 8px 14px; font-weight: 700; cursor: pointer; }
  .chip.on { border-color: var(--blue); background: var(--blue-soft); color: var(--blue-dark); }
  .row { display: flex; gap: 10px; align-items: center; font-weight: 700; }
  select { font: inherit; padding: 6px 10px; border-radius: 8px; border: 1px solid var(--line-2); background: var(--surface); color: var(--ink); }
  .nav { display: flex; gap: 10px; justify-content: flex-end; margin-top: 8px; }
  .status { padding: 12px 14px; border-radius: 12px; font-weight: 700; }
  .status.ok { background: var(--green-soft); color: var(--green-dark); }
  .status.bad { background: var(--red-soft); color: var(--red-dark); }
  .status.info { background: var(--blue-soft); color: var(--blue-dark); font-weight: 600; }
  .cmd { background: var(--code-bg); color: var(--code-ink); padding: 12px 14px; border-radius: 10px; font-family: var(--mono); font-size: 13px; overflow-x: auto; margin: 0; user-select: all; }
  .warm { display: flex; align-items: center; gap: 12px; }
  .okc { color: var(--green); font-weight: 700; }
  .bad { color: var(--red); }
  .small { font-size: 13px; }
</style>
