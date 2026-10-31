<script lang="ts">
  // Settings: editor assists (rust-analyzer), appearance, language comparisons, toolchain, reset.
  import { api, errorText } from '../lib/api'
  import { app, type Settings } from '../lib/state.svelte'

  let checking = $state(false)
  const tc = $derived(app.info?.toolchain)
  const ra = $derived(tc?.rust_analyzer ?? false)

  const assists: { key: keyof Settings; name: string; desc: string; needsRa: boolean }[] = [
    { key: 'assistCompletion', name: 'Autocomplete', desc: 'Suggestions while you type. Uses rust-analyzer when installed, otherwise keywords and words from your file.', needsRa: false },
    { key: 'assistHover', name: 'Hover information', desc: 'Types and documentation when you hover over code.', needsRa: true },
    { key: 'assistDiagnostics', name: 'Inline errors', desc: 'Red squiggles under mistakes as you type, before you run.', needsRa: true },
    { key: 'assistSignature', name: 'Signature help', desc: 'Shows a function\'s parameters while you type a call.', needsRa: true },
    { key: 'assistDefinition', name: 'Go to definition', desc: 'Press F12 on a name to jump to where it is defined.', needsRa: true },
  ]

  async function recheck() {
    checking = true
    try {
      await api.checkToolchain()
      await app.refresh()
    } finally {
      checking = false
    }
  }

  async function setLang(v: string) {
    if (!app.info) return
    await api.saveProfile({ ...app.info.profile, coming_from: v })
    await app.refresh()
  }

  async function setGoal(v: number) {
    if (!app.info) return
    await api.saveProfile({ ...app.info.profile, daily_goal: v })
    await app.refresh()
  }

  async function reset() {
    if (!(await app.ask({ title: 'Reset all progress?', body: 'This deletes your XP, streaks, answers, solved problems, drafts and badges. Settings are kept. This cannot be undone.', yes: 'Reset all progress', danger: true }))) return
    try {
      await api.resetProgress()
      await app.load()
      app.go({ name: 'onboarding' })
    } catch (e) {
      app.toast({ kind: 'error', title: 'Reset failed', body: errorText(e) })
    }
  }
</script>

<div class="page scroll">
  <h1>Settings</h1>

  <section class="card">
    <h2>Editor help</h2>
    <p class="muted">The course is designed to be done with these off: you learn more by reading the compiler's messages yourself. Turn on whatever you find useful. Changes apply when you open a problem.</p>
    {#each assists as a}
      <label class="toggle" class:disabled={a.needsRa && !ra}>
        <input type="checkbox" checked={app.settings[a.key] as boolean} disabled={a.needsRa && !ra} onchange={e => app.setSetting(a.key, (e.target as HTMLInputElement).checked as never)} />
        <span><strong>{a.name}</strong><br /><span class="muted">{a.desc}</span></span>
      </label>
    {/each}
    <div class="ra-status">
      {#if ra}<span class="ok">✓ rust-analyzer is installed.</span>
      {:else}<span>rust-analyzer isn't installed, so hover, inline errors, signature help and go-to-definition are unavailable. Install it with:</span><pre class="cmd">rustup component add rust-analyzer</pre>{/if}
    </div>
  </section>

  <section class="card">
    <h2>Appearance</h2>
    <div class="row">
      <span>Theme</span>
      <div class="seg">
        {#each [['light', 'Light'], ['dark', 'Dark'], ['system', 'System']] as [v, name]}
          <button aria-pressed={app.settings.theme === v} onclick={() => app.setSetting('theme', v as Settings['theme'])}>{name}</button>
        {/each}
      </div>
    </div>
    <div class="row">
      <span>Code font size</span>
      <input type="range" min="12" max="22" value={app.settings.fontSize} oninput={e => { app.setSetting('fontSize', Number((e.target as HTMLInputElement).value)); app.applyTheme() }} />
      <span class="muted">{app.settings.fontSize}px</span>
    </div>
  </section>

  <section class="card">
    <h2>Learning</h2>
    <div class="row">
      <span>“Coming from” comparisons</span>
      <select value={app.info?.profile.coming_from} onchange={e => setLang((e.target as HTMLSelectElement).value)}>
        <option value="python">Python</option><option value="js">JavaScript / TypeScript</option><option value="java">Java / C#</option><option value="c">C / C++</option><option value="go">Go</option>
      </select>
    </div>
    <div class="row">
      <span>Daily goal</span>
      <select value={String(app.info?.profile.daily_goal)} onchange={e => setGoal(Number((e.target as HTMLSelectElement).value))}>
        <option value="10">10 XP</option><option value="30">30 XP</option><option value="50">50 XP</option><option value="100">100 XP</option>
      </select>
    </div>
  </section>

  <section class="card">
    <h2>Rust toolchain</h2>
    {#if tc?.found}
      <p>Rust {tc.rustc_version} · course written for Rust {tc.target_rustc}{tc.new_enough ? '' : ` · needs ${tc.min_rustc} or newer`}</p>
    {:else}
      <p>Rust isn't installed. Install it from <strong>rustup.rs</strong>, then check again.</p>
      <pre class="cmd">curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh</pre>
    {/if}
    <button class="btn ghost small" onclick={recheck} disabled={checking}>{checking ? 'Checking…' : 'Check again'}</button>
  </section>

  <section class="card danger">
    <h2>Reset progress</h2>
    <p class="muted">Deletes XP, streaks, answers, solved problems, drafts and badges.</p>
    <button class="btn stop small" onclick={reset}>Reset all progress</button>
  </section>
</div>

<style>
  .page { height: 100%; padding: 28px 36px 60px; max-width: 820px; margin: 0 auto; display: flex; flex-direction: column; gap: 16px; }
  h1 { margin: 0; }
  h2 { margin: 0 0 8px; font-size: 18px; }
  .toggle { display: flex; gap: 12px; align-items: flex-start; padding: 10px 0; border-top: 1px solid var(--line); cursor: pointer; }
  .toggle.disabled { opacity: .5; cursor: default; }
  .toggle input { width: 20px; height: 20px; margin-top: 2px; accent-color: var(--rust); }
  .ra-status { margin-top: 10px; font-size: 14px; }
  .ok { color: var(--green); font-weight: 700; }
  .cmd { background: var(--code-bg); color: var(--code-ink); padding: 10px 12px; border-radius: 10px; font-family: var(--mono); font-size: 13px; user-select: all; overflow-x: auto; }
  .row { display: flex; align-items: center; gap: 14px; padding: 8px 0; }
  .row > span:first-child { flex: 0 0 220px; font-weight: 700; }
  .seg { display: flex; gap: 4px; background: var(--surface-2); padding: 4px; border-radius: 10px; }
  .seg button { border: 0; background: transparent; padding: 6px 14px; border-radius: 8px; font-weight: 700; cursor: pointer; }
  .seg button[aria-pressed="true"] { background: var(--surface); box-shadow: 0 1px 3px rgba(0, 0, 0, .12); }
  select { font: inherit; padding: 6px 10px; border-radius: 8px; border: 1px solid var(--line-2); background: var(--surface); color: var(--ink); }
  .danger { border-color: var(--red-soft); }
</style>
