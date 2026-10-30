<script lang="ts">
  // The code editor: CodeMirror 6 with one state per file tab. Syntax highlighting
  // only, unless the learner turned on editor assists in Settings.
  import { closeBrackets } from '@codemirror/autocomplete'
  import { defaultKeymap, history, historyKeymap, indentWithTab } from '@codemirror/commands'
  import { rust } from '@codemirror/lang-rust'
  import { bracketMatching, HighlightStyle, indentOnInput, syntaxHighlighting } from '@codemirror/language'
  import { EditorState, type Extension } from '@codemirror/state'
  import { drawSelection, EditorView, highlightActiveLine, highlightActiveLineGutter, keymap, lineNumbers } from '@codemirror/view'
  import { tags as t } from '@lezer/highlight'
  import { onDestroy } from 'svelte'
  import type { SourceFile } from '../lib/bindings/SourceFile'
  import { basicCompletion, type LspSession } from '../lib/lsp'

  let {
    files,
    active = $bindable(),
    completion = false,
    lsp = null,
    onchange,
    readonly = false,
  }: {
    files: SourceFile[]
    active: string
    completion?: boolean
    lsp?: LspSession | null
    onchange: (files: SourceFile[]) => void
    readonly?: boolean
  } = $props()

  let host: HTMLDivElement | undefined = $state()
  let view: EditorView | null = null
  let states = new Map<string, EditorState>()
  // The editor owns the text after creation; `load()` replaces it.
  // svelte-ignore state_referenced_locally
  let current: SourceFile[] = files.map(f => ({ ...f }))

  const style = HighlightStyle.define([
    { tag: [t.keyword, t.controlKeyword, t.definitionKeyword, t.modifier, t.self, t.bool], color: 'var(--tok-kw)' },
    { tag: [t.string, t.character], color: 'var(--tok-str)' },
    { tag: [t.number, t.integer, t.float], color: 'var(--tok-num)' },
    { tag: [t.lineComment, t.blockComment], color: 'var(--tok-com)', fontStyle: 'italic' },
    { tag: [t.typeName, t.className, t.namespace], color: 'var(--tok-type)' },
    { tag: [t.macroName], color: 'var(--tok-mac)' },
    { tag: [t.function(t.variableName), t.function(t.definition(t.variableName))], color: 'var(--tok-fn)' },
    { tag: [t.heading, t.propertyName], color: 'var(--tok-fn)' },
  ])

  const theme = EditorView.theme(
    {
      '&': { height: '100%', backgroundColor: 'var(--code-bg)', color: 'var(--code-ink)', fontSize: 'var(--editor-font)' },
      '.cm-content': { fontFamily: 'var(--mono)', caretColor: '#fff', padding: '12px 0' },
      '.cm-gutters': { backgroundColor: 'var(--code-bg)', color: 'var(--code-dim)', border: 'none', fontFamily: 'var(--mono)' },
      '.cm-activeLine': { backgroundColor: 'rgba(255,255,255,.04)' },
      '.cm-activeLineGutter': { backgroundColor: 'rgba(255,255,255,.06)', color: 'var(--code-ink)' },
      '.cm-cursor': { borderLeftColor: '#fff', borderLeftWidth: '2px' },
      '&.cm-focused .cm-selectionBackground, .cm-selectionBackground, ::selection': { backgroundColor: 'rgba(122,162,247,.35) !important' },
      '.cm-scroller': { lineHeight: '1.7' },
      '.cm-tooltip': { backgroundColor: 'var(--surface)', color: 'var(--ink)', border: '1px solid var(--line)', borderRadius: '10px', fontFamily: 'var(--sans)', overflow: 'hidden' },
      '.cm-tooltip-autocomplete > ul': { fontFamily: 'var(--mono)', fontSize: '13px' },
      '.cm-tooltip-autocomplete > ul > li': { padding: '3px 10px', color: 'var(--ink-2)' },
      '.cm-tooltip-autocomplete > ul > li[aria-selected]': { backgroundColor: 'var(--blue)', color: '#fff' },
      '.cm-completionDetail': { color: 'var(--muted)', marginLeft: '8px' },
      '.cm-tooltip-autocomplete > ul > li[aria-selected] .cm-completionDetail': { color: 'rgba(255,255,255,.8)' },
      '.cm-tooltip.cm-completionInfo, .cm-tooltip-hover': { padding: '8px 12px', maxWidth: '520px', fontSize: '13px', lineHeight: '1.5' },
      '.cm-diagnostic': { fontFamily: 'var(--sans)', fontSize: '13px' },
    },
    { dark: true },
  )

  function extensionsFor(path: string): Extension[] {
    const isRust = path.endsWith('.rs')
    const ext: Extension[] = [
      lineNumbers(),
      highlightActiveLine(),
      highlightActiveLineGutter(),
      history(),
      drawSelection(),
      indentOnInput(),
      bracketMatching(),
      EditorState.tabSize.of(4),
      keymap.of([...defaultKeymap, ...historyKeymap, indentWithTab]),
      syntaxHighlighting(style),
      theme,
      EditorView.updateListener.of(u => {
        if (u.docChanged) {
          const text = u.state.doc.toString()
          current = current.map(f => (f.path === path ? { ...f, text } : f))
          onchange(current)
        }
      }),
      EditorState.readOnly.of(readonly),
      EditorView.contentAttributes.of({ spellcheck: 'false', autocorrect: 'off', autocapitalize: 'off' }),
    ]
    if (isRust) ext.push(rust())
    if (completion) {
      ext.push(closeBrackets())
      if (!lsp) ext.push(basicCompletion())
    }
    const uri = lsp?.uris[path]
    if (lsp && uri) ext.push(lsp.client.plugin(uri, isRust ? 'rust' : 'toml'))
    return ext
  }

  function stateFor(path: string): EditorState {
    let s = states.get(path)
    if (!s) {
      const f = current.find(f => f.path === path)
      s = EditorState.create({ doc: f?.text ?? '', extensions: extensionsFor(path) })
      states.set(path, s)
    }
    return s
  }

  /** Rebuilds every file's state (new assists, a reset to starter code, a new LSP session). */
  export function load(newFiles: SourceFile[]) {
    current = newFiles.map(f => ({ ...f }))
    states = new Map()
    view?.setState(stateFor(active))
  }

  $effect(() => {
    if (!host) return
    view = new EditorView({ state: stateFor(active), parent: host })
    return () => view?.destroy()
  })

  // Switching tabs keeps each file's undo history.
  let shown = active
  $effect(() => {
    const a = active
    if (!view || a === shown) return
    states.set(shown, view.state)
    shown = a
    view.setState(stateFor(a))
  })

  // New assists or LSP session: rebuild states with the current text.
  let firstRun = true
  $effect(() => {
    void completion
    void lsp
    if (firstRun) {
      firstRun = false
      return
    }
    if (view) states.set(shown, view.state)
    const texts = current
    states = new Map()
    current = texts
    view?.setState(stateFor(active))
  })

  onDestroy(() => view?.destroy())
</script>

<div class="editor" bind:this={host}></div>

<style>
  .editor { height: 100%; min-height: 0; overflow: hidden; }
  .editor :global(.cm-editor) { height: 100%; }
  .editor :global(.cm-editor.cm-focused) { outline: none; }
</style>
