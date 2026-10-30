// Markdown to HTML for course text and the bundled Rust Book. Raw HTML in the
// source is escaped, except the Book's <Listing> wrappers, which become captions.

import { Marked } from 'marked'
import { highlightToHtml } from './highlight'

const escape = (s: string) => s.replace(/&/g, '&amp;').replace(/</g, '&lt;').replace(/>/g, '&gt;').replace(/"/g, '&quot;')

const marked = new Marked({
  gfm: true,
  breaks: false,
  renderer: {
    html({ text }) {
      return escape(text)
    },
    code({ text, lang }) {
      const l = (lang ?? '').split(/[ ,]/)[0]
      return `<pre class="block"><code>${highlightToHtml(text, l)}</code></pre>`
    },
  },
})

export function md(src: string | null | undefined): string {
  if (!src) return ''
  return marked.parse(src, { async: false }) as string
}

/** Inline Markdown (no wrapping paragraph). */
export function mdInline(src: string | null | undefined): string {
  if (!src) return ''
  return marked.parseInline(src, { async: false }) as string
}

/** Prepares a Rust Book chapter: drops comments and anchors, turns listings into captions. */
export function bookMd(src: string): string {
  const cleaned = src
    .replace(/<!--[\s\S]*?-->/g, '')
    .replace(/<a id="[^"]*"><\/a>/g, '')
    .replace(/<Listing([^>]*)>/g, (_m, attrs: string) => {
      const num = attrs.match(/number="([^"]*)"/)?.[1]
      const cap = attrs.match(/caption="([^"]*)"/)?.[1]
      const file = attrs.match(/file-name="([^"]*)"/)?.[1]
      const parts = [num ? `Listing ${num}` : '', file ? `\`${file}\`` : ''].filter(Boolean).join(' · ')
      return parts || cap ? `\n*${parts}${cap ? (parts ? ': ' : '') + cap : ''}*\n` : ''
    })
    .replace(/<\/Listing>/g, '')
    .replace(/<span class="filename">([^<]*)<\/span>/g, '*$1*')
    .replace(/<span class="caption">([^<]*)<\/span>/g, '*$1*')
  return md(cleaned)
}

/** A title as plain text, for attributes and labels: `match` → match. */
export function plain(src: string | null | undefined): string {
  return (src ?? '').replace(/`/g, '')
}
