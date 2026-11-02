// Computes what each visualizer panel shows at a step, by applying the steps' ops in order.

import type { NodeState } from './bindings/NodeState'
import type { Panel } from './bindings/Panel'
import type { SceneNode } from './bindings/SceneNode'
import type { Screen } from './bindings/Screen'

export type TerminalState = { kind: 'terminal'; id: string; title: string | null; cursor: boolean; text: string; fresh: string; badge: string | null }
export type SceneState = {
  kind: 'scene'
  id: string
  layout: 'row' | 'tree' | 'grid'
  nodes: (SceneNode & { state: NodeState; depth: number; changed: boolean })[]
  edges: { id: string; from: string; to: string; label: string | null; hidden: boolean; style: string | null }[]
}
export type PanelState = TerminalState | SceneState

/** Panel states after steps 1..=step (step 0 = before the first step). */
export function panelStates(screen: Screen, step: number): PanelState[] {
  const states: PanelState[] = screen.panels.map((p: Panel) => initial(p))
  screen.steps.slice(0, step).forEach((s, i) => {
    const last = i === step - 1
    for (const st of states) {
      if (st.kind === 'terminal') st.fresh = last ? '' : st.fresh
      if (st.kind === 'scene') for (const n of st.nodes) if (last) n.changed = false
    }
    for (const op of s.ops) {
      const st = states.find(x => x.id === op.panel)
      if (!st) continue
      if (st.kind === 'terminal') {
        if (op.op === 'print') {
          st.text += op.text
          if (last) st.fresh += op.text
        } else if (op.op === 'badge') st.badge = op.text
        else if (op.op === 'reset') st.badge = null
      } else {
        if (op.op === 'node') {
          const n = st.nodes.find(x => x.id === op.id)
          if (!n) continue
          const ns = { ...n.state }
          if (op.hidden != null) ns.hidden = op.hidden
          if (op.style != null) ns.style = op.style
          if (op.lit != null) ns.lit = op.lit
          if (op.ghost != null) ns.ghost = op.ghost
          if (op.badge != null) ns.badge = op.badge === '' ? null : op.badge
          if (op.note != null) ns.note = op.note === '' ? null : op.note
          if (op.value != null) ns.value = op.value === '' ? null : op.value
          n.state = ns
          if (last) n.changed = true
        } else if (op.op === 'edge') {
          const e = st.edges.find(x => x.id === op.id)
          if (e) e.hidden = op.hidden
        } else if (op.op === 'reset') {
          for (const n of st.nodes) n.state = { ...n.state, lit: false, badge: null }
        }
      }
    }
  })
  return states
}

function initial(p: Panel): PanelState {
  if (p.kind === 'terminal') return { kind: 'terminal', id: p.id, title: p.title, cursor: p.cursor, text: p.text ?? '', fresh: '', badge: null }
  const depth = (n: SceneNode): number => {
    let d = 0
    let cur = n
    while (cur.parent) {
      const parent = p.nodes.find(x => x.id === cur.parent)
      if (!parent) break
      d++
      cur = parent
    }
    return d
  }
  return {
    kind: 'scene',
    id: p.id,
    layout: p.layout,
    nodes: p.nodes.map(n => ({ ...n, state: { ...n.state }, depth: depth(n), changed: false })),
    edges: p.edges.map(e => ({ ...e })),
  }
}

/** "5:15-16" → { line: 5, from: 15, to: 16 } */
export function parseSpan(s: string): { line: number; from: number; to: number } | null {
  const m = s.match(/^(\d+):(\d+)-(\d+)$/)
  return m ? { line: +m[1], from: +m[2], to: +m[3] } : null
}
