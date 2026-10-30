// Static syntax highlighting for read-only code, using the same Rust grammar as the editor.

import { rustLanguage } from '@codemirror/lang-rust'
import { highlightTree, tagHighlighter, tags as t } from '@lezer/highlight'

const classes = tagHighlighter([
  { tag: [t.keyword, t.controlKeyword, t.definitionKeyword, t.modifier, t.operatorKeyword, t.self, t.bool], class: 't-kw' },
  { tag: [t.string, t.character, t.special(t.string)], class: 't-str' },
  { tag: [t.number, t.integer, t.float], class: 't-num' },
  { tag: [t.lineComment, t.blockComment, t.comment], class: 't-com' },
  { tag: [t.typeName, t.className, t.namespace], class: 't-type' },
  { tag: [t.macroName], class: 't-mac' },
  { tag: [t.function(t.variableName), t.function(t.definition(t.variableName))], class: 't-fn' },
  { tag: [t.labelName, t.special(t.variableName)], class: 't-life' },
])

const esc = (s: string) => s.replace(/&/g, '&amp;').replace(/</g, '&lt;').replace(/>/g, '&gt;')

/** Highlights code and returns one HTML string per line. */
export function highlightLines(code: string, lang: string): string[] {
  if (lang === 'rust' || lang === 'rs') {
    const tree = rustLanguage.parser.parse(code)
    let html = ''
    let pos = 0
    highlightTree(tree, classes, (from, to, cls) => {
      if (from > pos) html += esc(code.slice(pos, from))
      html += `<span class="${cls}">${esc(code.slice(from, to))}</span>`
      pos = to
    })
    html += esc(code.slice(pos))
    // Macro names: `println!` is parsed as an identifier followed by `!`.
    html = html.replace(/\b([a-z_][a-z0-9_]*)!/g, '<span class="t-mac">$1!</span>')
    return splitHtmlLines(html)
  }
  if (lang === 'console' || lang === 'shell' || lang === 'sh') {
    return code.split('\n').map(l => (l.startsWith('$ ') ? `<span class="t-prompt">$ </span><span class="t-kw">${esc(l.slice(2))}</span>` : esc(l)))
  }
  if (lang === 'toml') {
    return code.split('\n').map(l => {
      if (/^\s*\[.*\]\s*$/.test(l)) return `<span class="t-type">${esc(l)}</span>`
      const m = l.match(/^(\s*[\w.-]+)(\s*=\s*)(.*)$/)
      if (m) return `<span class="t-fn">${esc(m[1])}</span>${esc(m[2])}<span class="t-str">${esc(m[3])}</span>`
      return esc(l)
    })
  }
  return code.split('\n').map(esc)
}

/** Splits highlighted HTML at newlines, closing and reopening spans across lines. */
function splitHtmlLines(html: string): string[] {
  const lines: string[] = []
  let cur = ''
  const open: string[] = []
  const re = /(<span class="[^"]*">|<\/span>|\n|[^<\n]+)/g
  let m: RegExpExecArray | null
  while ((m = re.exec(html))) {
    const tok = m[1]
    if (tok === '\n') {
      cur += '</span>'.repeat(open.length)
      lines.push(cur)
      cur = open.join('')
    } else if (tok.startsWith('<span')) {
      open.push(tok)
      cur += tok
    } else if (tok === '</span>') {
      open.pop()
      cur += tok
    } else cur += tok
  }
  lines.push(cur)
  return lines
}

export function highlightToHtml(code: string, lang: string): string {
  return highlightLines(code, lang).join('\n')
}
