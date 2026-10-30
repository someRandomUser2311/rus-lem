<script lang="ts">
  // "Coming from X" card. Starts on the learner's language; other tabs stay one click away.
  import { app } from '../lib/state.svelte'
  import Md from './Md.svelte'

  let { cards }: { cards: Record<string, string | undefined> } = $props()
  const names: Record<string, string> = { python: 'Python', js: 'JS / TS', java: 'Java / C#', c: 'C / C++', go: 'Go' }
  const order = ['python', 'js', 'java', 'c', 'go']
  let lang = $state(app.comingFrom)
  const available = $derived(order.filter(k => cards[k]))
  $effect(() => {
    if (!cards[lang] && available.length) lang = available[0]
  })
</script>

{#if available.length}
  <div class="coming-from">
    <div class="cf-tabs">
      <span class="label">Coming from</span>
      {#each available as k}
        <button aria-pressed={lang === k} onclick={() => (lang = k)}>{names[k]}</button>
      {/each}
    </div>
    <div class="cf-body"><Md src={cards[lang]} inline /></div>
  </div>
{/if}
