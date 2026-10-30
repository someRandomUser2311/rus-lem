<script lang="ts">
  // Line icons for path nodes. They take the text colour, so they stay legible on every
  // node colour and in the locked state (emoji kept their own colours and didn't).
  let { kind, size = 30 }: { kind: string; size?: number } = $props()
</script>

<svg width={size} height={size} viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2.4" stroke-linecap="round" stroke-linejoin="round" aria-hidden="true">
  {#if kind === 'lesson'}
    <path d="M2 4h6a4 4 0 0 1 4 4v13a3 3 0 0 0-3-3H2z" /><path d="M22 4h-6a4 4 0 0 0-4 4v13a3 3 0 0 1 3-3h7z" />
  {:else if kind === 'questions'}
    <path d="M8.5 8.5a3.5 3.5 0 1 1 5 3.2c-1 .5-1.5 1.3-1.5 2.3v.5" /><path d="M12 19h.01" />
  {:else if kind === 'practice'}
    <circle cx="12" cy="12" r="9" /><circle cx="12" cy="12" r="5" /><circle cx="12" cy="12" r="1" />
  {:else if kind === 'problem'}
    <path d="M16 18l6-6-6-6" /><path d="M8 6l-6 6 6 6" />
  {:else if kind === 'review'}
    <path d="M3 12a9 9 0 0 1 15-6.7L21 8" /><path d="M21 3v5h-5" /><path d="M21 12a9 9 0 0 1-15 6.7L3 16" /><path d="M3 21v-5h5" />
  {:else if kind === 'project'}
    <path d="M21 8l-9-5-9 5v8l9 5 9-5z" /><path d="M3 8l9 5 9-5" /><path d="M12 13v8" />
  {:else if kind === 'milestone'}
    <path d="M5 22V3" /><path d="M5 4h13l-3 4.5L18 13H5" />
  {:else if kind === 'done'}
    <path d="M4 12.5l5 5L20 6.5" />
  {/if}
</svg>
