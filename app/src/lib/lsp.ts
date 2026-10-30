// rust-analyzer in the editor: a CodeMirror LSP client whose transport goes through
// the backend (which runs rust-analyzer). Every feature is opt-in from Settings.

import { autocompletion, completeAnyWord, completeFromList, type Completion } from '@codemirror/autocomplete'
import { setDiagnostics } from '@codemirror/lint'
import { hoverTooltips, jumpToDefinitionKeymap, LSPClient, LSPPlugin, serverCompletion, serverDiagnostics, signatureHelp, type Transport } from '@codemirror/lsp-client'
import type { Extension } from '@codemirror/state'
import { EditorView, keymap, ViewPlugin, type ViewUpdate } from '@codemirror/view'
import { api } from './api'
import type { SourceFile } from './bindings/SourceFile'
import type { Settings } from './state.svelte'

export type Assists = Pick<Settings, 'assistCompletion' | 'assistHover' | 'assistDiagnostics' | 'assistSignature' | 'assistDefinition'>

export const anyAssist = (s: Assists) => s.assistCompletion || s.assistHover || s.assistDiagnostics || s.assistSignature || s.assistDefinition

export type LspSession = { client: LSPClient; uris: Record<string, string>; stop: () => void }

export async function startLsp(problemId: string, files: SourceFile[], s: Assists): Promise<LspSession> {
  let handlers: ((m: string) => void)[] = []
  const started = await api.lspStart(problemId, files, m => handlers.forEach(h => h(m)))
  const transport: Transport = {
    send: m => void api.lspSend(m).catch(() => {}),
    subscribe: h => void handlers.push(h),
    unsubscribe: h => void (handlers = handlers.filter(x => x !== h)),
  }
  const extensions: Extension[] = []
  if (s.assistCompletion) extensions.push(serverCompletion())
  if (s.assistHover) extensions.push(hoverTooltips())
  if (s.assistSignature) extensions.push(signatureHelp())
  if (s.assistDefinition) extensions.push(keymap.of(jumpToDefinitionKeymap))
  if (s.assistDiagnostics) extensions.push(pullDiagnostics)
  const client = new LSPClient({
    rootUri: started.root_uri,
    extensions: [...extensions, ...(s.assistDiagnostics ? [serverDiagnostics()] : [])],
  }).connect(transport)
  return {
    client,
    uris: started.uris as Record<string, string>,
    stop: () => {
      client.disconnect()
      void api.lspStop()
    },
  }
}

type PullItem = { range: { start: { line: number; character: number }; end: { line: number; character: number } }; severity?: number; message: string }

/** The client advertises pull diagnostics, so rust-analyzer stops pushing them after
 * edits; ask for them ourselves, shortly after the text changes. */
const pullDiagnostics = ViewPlugin.fromClass(
  class {
    timer: ReturnType<typeof setTimeout> | undefined
    constructor(readonly view: EditorView) {
      this.schedule(300)
    }
    update(u: ViewUpdate) {
      if (u.docChanged) this.schedule(600)
    }
    schedule(ms: number) {
      clearTimeout(this.timer)
      this.timer = setTimeout(() => void this.pull(), ms)
    }
    async pull() {
      const plugin = LSPPlugin.get(this.view)
      if (!plugin) return
      plugin.client.sync()
      try {
        const r = await plugin.client.request<object, { items?: PullItem[] }>('textDocument/diagnostic', { textDocument: { uri: plugin.uri } })
        if (!r?.items) return
        const doc = plugin.syncedDoc
        const diagnostics = r.items.map(d => ({
          from: plugin.unsyncedChanges.mapPos(plugin.fromPosition(d.range.start, doc)),
          to: plugin.unsyncedChanges.mapPos(plugin.fromPosition(d.range.end, doc)),
          severity: (d.severity === 2 ? 'warning' : d.severity === 3 || d.severity === 4 ? 'info' : 'error') as 'error' | 'warning' | 'info',
          message: d.message,
        }))
        this.view.dispatch(setDiagnostics(this.view.state, diagnostics))
      } catch {
        // rust-analyzer may still be starting; the next edit pulls again.
      }
    }
    destroy() {
      clearTimeout(this.timer)
    }
  },
)

const KEYWORDS = 'as async await break const continue crate dyn else enum extern false fn for if impl in let loop match mod move mut pub ref return self Self static struct super trait true type unsafe use where while'.split(' ')
const SNIPPETS: Completion[] = [
  { label: 'fn main', type: 'function', apply: 'fn main() {\n    \n}' },
  { label: 'println!', type: 'function', apply: 'println!("")' },
  { label: 'print!', type: 'function', apply: 'print!("")' },
  { label: 'format!', type: 'function', apply: 'format!("")' },
  { label: 'vec!', type: 'function', apply: 'vec![]' },
  { label: 'String::new', type: 'function', apply: 'String::new()' },
  { label: 'String::from', type: 'function', apply: 'String::from("")' },
]

/** Completion without rust-analyzer: keywords, a few snippets and words from the file. */
export function basicCompletion(): Extension {
  return autocompletion({
    override: [completeFromList([...KEYWORDS.map(k => ({ label: k, type: 'keyword' })), ...SNIPPETS]), completeAnyWord],
  })
}
