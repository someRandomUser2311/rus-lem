// Typed wrappers for the backend commands (app/src-tauri/src/commands.rs).

import { Channel, invoke } from '@tauri-apps/api/core'
import type { AppInfo } from './bindings/AppInfo'
import type { Award } from './bindings/Award'
import type { ChapterState } from './bindings/ChapterState'
import type { ChapterView } from './bindings/ChapterView'
import type { Checked } from './bindings/Checked'
import type { GradeEvent } from './bindings/GradeEvent'
import type { GradeOutcome } from './bindings/GradeOutcome'
import type { LspStarted } from './bindings/LspStarted'
import type { ProblemView } from './bindings/ProblemView'
import type { Profile } from './bindings/Profile'
import type { ProfilePage } from './bindings/ProfilePage'
import type { Response } from './bindings/Response'
import type { ReviewPick } from './bindings/ReviewPick'
import type { SourceFile } from './bindings/SourceFile'
import type { ToolchainStatus } from './bindings/ToolchainStatus'

// Inside the desktop app commands go through Tauri. In a plain browser (the
// development preview, `rusticoder-web`) they go over HTTP to the same backend code.
const inTauri = typeof window !== 'undefined' && '__TAURI_INTERNALS__' in window
// The Vite dev server proxies /invoke to rusticoder-web (vite.config.ts).
const WEB = '/invoke/'

async function call<T>(cmd: string, args: Record<string, unknown> = {}): Promise<T> {
  if (inTauri) return invoke<T>(cmd, args)
  const res = await fetch(WEB + cmd, { method: 'POST', headers: { 'content-type': 'application/json' }, body: JSON.stringify(args) })
  const body = await res.json()
  if (!res.ok) throw body
  return body as T
}

let lspPoll: ReturnType<typeof setInterval> | undefined

export const api = {
  appInfo: () => call<AppInfo>('app_info'),
  checkToolchain: () => call<ToolchainStatus>('check_toolchain'),
  warmUp: () => call<boolean>('warm_up'),
  saveProfile: (profile: Profile) => call<void>('save_profile', { profile }),
  setSetting: (key: string, value: unknown) => call<void>('set_setting', { key, value }),
  resetProgress: () => call<void>('reset_progress'),
  path: () => call<ChapterState[]>('get_path'),
  chapter: (id: string) => call<ChapterView>('get_chapter', { id }),
  completeLesson: (lesson: string, tries: number) => call<Award>('complete_lesson', { lesson, tries }),
  checkAnswer: (question: string, response: Response, record: boolean) => call<Checked>('check_answer', { question, response, record }),
  completeSet: (set: string, score: number) => call<Award>('complete_set', { set, score }),
  problem: (id: string) => call<ProblemView>('get_problem', { id }),
  saveDraft: (id: string, files: SourceFile[]) => call<void>('save_draft', { id, files }),
  openHint: (id: string, n: number) => call<ProblemView>('open_hint', { id, n }),
  grade: async (id: string, files: SourceFile[], submit: boolean, reviewChapter: number | null, part: number | null, onEvent: (e: GradeEvent) => void) => {
    if (!inTauri) {
      const r = await call<{ events: GradeEvent[]; result: GradeOutcome }>('grade', { id, files, submit, reviewChapter, part })
      r.events.forEach(onEvent)
      return r.result
    }
    const ch = new Channel<GradeEvent>()
    ch.onmessage = onEvent
    return invoke<GradeOutcome>('grade', { id, files, submit, reviewChapter, part, onEvent: ch })
  },
  cancelGrade: () => call<void>('cancel_grade'),
  profile: () => call<ProfilePage>('get_profile'),
  review: (chapter: number) => call<ReviewPick[]>('get_review', { chapter }),
  readBook: (file: string) => call<string>('read_book', { file }),
  lspStart: async (id: string, files: SourceFile[], onMessage: (m: string) => void) => {
    if (!inTauri) {
      // Browser preview: poll the backend for rust-analyzer's messages.
      clearInterval(lspPoll)
      const started = await call<LspStarted>('lsp_start', { id, files })
      lspPoll = setInterval(async () => {
        const msgs = await call<string[]>('lsp_poll').catch(() => [])
        msgs.forEach(onMessage)
      }, 50)
      return started
    }
    const ch = new Channel<string>()
    ch.onmessage = onMessage
    return invoke<LspStarted>('lsp_start', { id, files, onMessage: ch })
  },
  lspSend: (message: string) => call<void>('lsp_send', { message }),
  lspStop: () => {
    clearInterval(lspPoll)
    return call<void>('lsp_stop')
  },
}

export function errorText(e: unknown): string {
  return typeof e === 'string' ? e : e instanceof Error ? e.message : JSON.stringify(e)
}
