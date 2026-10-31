// App-wide state: course info, the learner's path, settings, routing and toasts.

import { api, errorText } from './api'
import type { AppInfo } from './bindings/AppInfo'
import type { ChapterState } from './bindings/ChapterState'
import type { ChapterView } from './bindings/ChapterView'

export type Route =
  | { name: 'home' }
  | { name: 'onboarding' }
  | { name: 'lesson'; chapter: string; id: string }
  | { name: 'questions'; chapter: string; id: string }
  | { name: 'problem'; chapter: string; id: string; review?: number }
  | { name: 'review'; chapter: string }
  | { name: 'profile' }
  | { name: 'settings' }

export type Settings = {
  theme: 'light' | 'dark' | 'system'
  fontSize: number
  assistCompletion: boolean
  assistHover: boolean
  assistDiagnostics: boolean
  assistSignature: boolean
  assistDefinition: boolean
}

export const defaultSettings: Settings = {
  theme: 'light',
  fontSize: 15,
  assistCompletion: false,
  assistHover: false,
  assistDiagnostics: false,
  assistSignature: false,
  assistDefinition: false,
}

function parseHash(): Route {
  const parts = location.hash.replace(/^#\/?/, '').split('/').map(decodeURIComponent)
  switch (parts[0]) {
    case 'onboarding': return { name: 'onboarding' }
    case 'lesson': return { name: 'lesson', chapter: parts[1], id: parts[2] }
    case 'questions': return { name: 'questions', chapter: parts[1], id: parts[2] }
    case 'problem': return { name: 'problem', chapter: parts[1], id: parts[2], review: parts[3] ? Number(parts[3]) : undefined }
    case 'review': return { name: 'review', chapter: parts[1] }
    case 'profile': return { name: 'profile' }
    case 'settings': return { name: 'settings' }
    default: return { name: 'home' }
  }
}

export function href(r: Route): string {
  switch (r.name) {
    case 'home': return '#/'
    case 'onboarding': return '#/onboarding'
    case 'lesson': return `#/lesson/${r.chapter}/${encodeURIComponent(r.id)}`
    case 'questions': return `#/questions/${r.chapter}/${encodeURIComponent(r.id)}`
    case 'problem': return `#/problem/${r.chapter}/${encodeURIComponent(r.id)}${r.review ? `/${r.review}` : ''}`
    case 'review': return `#/review/${r.chapter}`
    case 'profile': return '#/profile'
    case 'settings': return '#/settings'
  }
}

export type Toast = { id: number; title: string; body?: string; kind: 'xp' | 'badge' | 'error' | 'info' }
/** An in-app yes/no question. The webview ignores `window.confirm`, so this replaces it. */
export type Ask = { title: string; body?: string; yes: string; danger?: boolean; answer: (yes: boolean) => void }

class App {
  info = $state<AppInfo | null>(null)
  path = $state<ChapterState[]>([])
  route = $state<Route>(parseHash())
  settings = $state<Settings>({ ...defaultSettings })
  toasts = $state<Toast[]>([])
  asking = $state<Ask | null>(null)
  chapters = new Map<string, ChapterView>()
  loadError = $state<string | null>(null)
  private toastId = 0

  constructor() {
    window.addEventListener('hashchange', () => (this.route = parseHash()))
  }

  go(r: Route) {
    location.hash = href(r)
  }

  async load() {
    try {
      this.info = await api.appInfo()
      this.settings = { ...defaultSettings, ...(this.info.settings as Partial<Settings>) }
      this.applyTheme()
      await this.refresh()
      if (!this.info.profile.onboarded && this.route.name !== 'onboarding') this.go({ name: 'onboarding' })
    } catch (e) {
      this.loadError = errorText(e)
    }
  }

  async refresh() {
    this.path = await api.path()
    if (this.info) this.info = { ...this.info, ...(await api.appInfo()) }
  }

  async chapter(id: string): Promise<ChapterView> {
    let c = this.chapters.get(id)
    if (!c) {
      c = await api.chapter(id)
      this.chapters.set(id, c)
    }
    return c
  }

  async setSetting<K extends keyof Settings>(key: K, value: Settings[K]) {
    this.settings[key] = value
    await api.setSetting(key, value)
    if (key === 'theme') this.applyTheme()
  }

  applyTheme() {
    const t = this.settings.theme
    const dark = t === 'dark' || (t === 'system' && matchMedia('(prefers-color-scheme: dark)').matches)
    document.documentElement.dataset.theme = dark ? 'dark' : 'light'
    document.documentElement.style.setProperty('--editor-font', `${this.settings.fontSize}px`)
  }

  toast(t: Omit<Toast, 'id'>) {
    const id = ++this.toastId
    this.toasts.push({ ...t, id })
    setTimeout(() => (this.toasts = this.toasts.filter(x => x.id !== id)), t.kind === 'error' ? 7000 : 3500)
  }

  /** Asks a yes/no question in a dialog; resolves to true when the learner confirms. */
  ask(q: Omit<Ask, 'answer'>): Promise<boolean> {
    this.asking?.answer(false)
    return new Promise(resolve => {
      this.asking = { ...q, answer: yes => { this.asking = null; resolve(yes) } }
    })
  }

  /** Shows XP and new badges from an award. */
  celebrate(award: { xp: number; badges: string[] }) {
    const xp = Number(award.xp)
    if (xp > 0) this.toast({ kind: 'xp', title: `+${xp} XP` })
    for (const b of award.badges) this.toast({ kind: 'badge', title: 'New badge', body: badgeTitle(b) })
  }

  get comingFrom(): string {
    return this.info?.profile.coming_from ?? 'python'
  }
}

export function badgeTitle(id: string): string {
  const m = id.match(/^chapter-(\d+)$/)
  if (m) return `Chapter ${m[1]} complete`
  const titles: Record<string, string> = {
    'first-lesson': 'First steps', 'first-accepted': 'Accepted!', 'ten-accepted': 'Ten down', 'fifty-accepted': 'Half a hundred',
    'hundred-questions': 'Quiz whiz', 'streak-3': 'On a roll', 'streak-7': 'Week of Rust', 'streak-30': 'Month of Rust',
    'xp-100': '100 XP', 'xp-1000': '1,000 XP', 'xp-5000': '5,000 XP', persistent: 'Persistent', 'first-hard': 'Hard mode', 'tested-out': 'Already knew it',
  }
  return titles[id] ?? id
}

export const app = new App()
