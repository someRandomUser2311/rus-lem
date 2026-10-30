<script lang="ts">
  // Renders course Markdown. Inline mode skips the wrapping paragraph.
  import { md, mdInline } from '../lib/md'
  let { src, inline = false, cls = '' }: { src: string | null | undefined; inline?: boolean; cls?: string } = $props()
</script>

{#if inline}<span class={cls}>{@html mdInline(src)}</span>{:else}<div class="prose {cls}">{@html md(src)}</div>{/if}
