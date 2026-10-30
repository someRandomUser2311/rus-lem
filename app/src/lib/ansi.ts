// Renders rustc/Cargo ANSI colour output as HTML, so compiler messages look the way
// they do in a terminal.

const COLORS: Record<number, string> = {
  30: '#6E7585', 31: '#FF6B5E', 32: '#5BD08A', 33: '#E0AF68', 34: '#7AA2F7', 35: '#BB9AF7', 36: '#7DCFFF', 37: '#E6E8EE',
  90: '#8C92A0', 91: '#FF8A80', 92: '#7EE2A8', 93: '#F2C760', 94: '#9AB8FF', 95: '#D0B4FF', 96: '#A5E4FF', 97: '#FFFFFF',
}

const esc = (s: string) => s.replace(/&/g, '&amp;').replace(/</g, '&lt;').replace(/>/g, '&gt;')

export function ansiToHtml(input: string): string {
  let out = ''
  let open = false
  let bold = false
  let color: string | null = null
  const re = /\x1b\[([0-9;]*)m/g
  let last = 0
  let m: RegExpExecArray | null
  const flush = (text: string) => {
    if (!text) return
    if (bold || color) out += `<span style="${color ? `color:${color};` : ''}${bold ? 'font-weight:700;' : ''}">${esc(text)}</span>`
    else out += esc(text)
  }
  while ((m = re.exec(input))) {
    flush(input.slice(last, m.index))
    last = re.lastIndex
    const codes = m[1] === '' ? [0] : m[1].split(';').map(Number)
    for (let i = 0; i < codes.length; i++) {
      const c = codes[i]
      if (c === 0) { bold = false; color = null }
      else if (c === 1) bold = true
      else if (c === 22) bold = false
      else if (c === 39) color = null
      else if (COLORS[c]) color = COLORS[c]
      else if (c === 38 && codes[i + 1] === 5) { color = xterm(codes[i + 2]); i += 2 }
    }
  }
  flush(input.slice(last))
  void open
  return out
}

function xterm(n: number): string {
  const base = [30, 31, 32, 33, 34, 35, 36, 37, 90, 91, 92, 93, 94, 95, 96, 97]
  return n < 16 ? COLORS[base[n]] : '#E6E8EE'
}
