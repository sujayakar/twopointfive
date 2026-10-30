// THE BROWSER'S SETTINGS, KEPT ON THE BACKEND TOO. Every setting of the game lives in this browser's localStorage under a `twopointsix.*` key, each written
// by its own module (ui/options.ts, host/keymap.ts, ui/findme.ts, game/loadout.ts …). This file does not know those modules: it takes the keys AS A MAP —
// every `twopointsix.*` key bar the ones below — and keeps that map on the backend under the browser's id (`prefs:put`), sent again a little after it changed.
// A browser that is handed another's id (a link: cloud.ts `?player=`) takes that map as its own before the game reads a single key.
// LEFT OUT (`skipped`): this module family's own keys; what belongs to the MACHINE, not the player (the quality tier and what auto settled on, the text
// size — a laptop must not open on a desktop's pick); saved games and the editor's document (large, and not settings); marks of one visit or one store.
// No game code is imported; the store is handed in (the page's is localStorage, the check's a map).
import type { Wire } from './wire';

export interface KeyStore { length: number; key(i: number): string | null; getItem(k: string): string | null; setItem(k: string, v: string): void; removeItem(k: string): void }

/** `secs` between looks at the keys · `valueMax` a key's value is sent up to (longer = not a setting) · `mapMax` the whole map (the backend's bound is 512 KB) */
export const PREFSYNC = { secs: 15, valueMax: 64 * 1024, mapMax: 400 * 1024 };
const PREFIX = 'twopointsix.';
const SKIP: (string | RegExp)[] = [
  /^twopointsix\.cloud\./,
  'twopointsix.quality', 'twopointsix.autoPreset', 'twopointsix.autoScaleRel', 'twopointsix.textsize', 'twopointsix.gpuLost', 'twopointsix.gentle', 'twopointsix.bench', 'twopointsix.shaderrows',
  /^twopointsix\.save\./, 'twopointsix.editorLevel', /^twopointsix\.editor\./, /^twopointsix\.viewer\./, /^twopointsix\.workbench\./,
  /^twopointsix\.kept\./, /^twopointsix\.onefile/, 'twopointsix.replay.from', 'twopointsix.testMute', /\.visit$/,
];
export const skipped = (k: string): boolean => !k.startsWith(PREFIX) || SKIP.some(s => typeof s === 'string' ? s === k : s.test(k));

/** the settings as they stand in the store */
export function prefsNow(store: KeyStore): Record<string, string> {
  const out: Record<string, string> = {}; let size = 0;
  const keys: string[] = []; for (let i = 0; i < store.length; i++) { const k = store.key(i); if (k !== null && !skipped(k)) keys.push(k); }
  for (const k of keys.sort()) { const v = store.getItem(k); if (v === null || v.length > PREFSYNC.valueMax || size + k.length + v.length > PREFSYNC.mapMax) continue; out[k] = v; size += k.length + v.length; }
  return out;
}
/** lay a map over the store: every key of it written, every setting the store has that it has not removed. True if anything changed */
export function prefsApply(store: KeyStore, map: Record<string, string>): boolean {
  let changed = false;
  const had = prefsNow(store);
  for (const k of Object.keys(had)) if (!(k in map)) { store.removeItem(k); changed = true; }
  for (const [k, v] of Object.entries(map)) if (!skipped(k) && typeof v === 'string' && had[k] !== v) { store.setItem(k, v); changed = true; }
  return changed;
}

export class PrefSync {
  readonly stats = { puts: 0, failed: 0, rev: 0, keys: 0, lastError: '' };
  /** the map as last sent (its JSON): a look that reads the same sends nothing */
  private sent: string | null = null;
  private busy: Promise<void> | null = null;
  constructor(private w: Wire, private store: KeyStore, private o: { pid: string; may?: () => boolean }) {}
  /** what the backend holds for this id (null: nothing yet, or it could not be asked) */
  async get(): Promise<{ rev: number; map: Record<string, string> } | null> {
    try {
      const r = await this.w.query<{ rev: number; data: string } | null>('prefs:get', { pid: this.o.pid }); if (!r) return null;
      const map = JSON.parse(r.data) as Record<string, string>; return map && typeof map === 'object' ? { rev: r.rev, map } : null;
    } catch (e) { this.stats.lastError = String((e as Error | undefined)?.message ?? e).slice(0, 200); return null; }
  }
  /** the backend's map becomes this browser's (a browser handed an id); true if a key changed */
  async take(got?: { rev: number; map: Record<string, string> } | null): Promise<boolean> {
    const r = got ?? await this.get(); if (!r) return false;
    const changed = prefsApply(this.store, r.map); this.sent = JSON.stringify(prefsNow(this.store)); this.stats.rev = r.rev;
    return changed;
  }
  /** what was sent last is what the backend has: the next look sends only a change (said once `hello` has answered that the backend's copy is this browser's own) */
  settled(json: string | null): void { this.sent = json; }
  /** look at the keys and send the map if it is not the one last sent. `going`: the page is leaving. Never rejects */
  look(going = false): Promise<void> {
    if (this.busy && !going) return this.busy;
    const run = (async () => {
      if (this.o.may && !this.o.may()) return;
      const map = prefsNow(this.store), json = JSON.stringify(map); if (json === this.sent) return;
      try { const r = await this.w.mutation<{ rev: number }>('prefs:put', { pid: this.o.pid, data: json }, { keepalive: going && json.length < 48000 }); this.sent = json; this.stats.puts++; this.stats.rev = r.rev; this.stats.keys = Object.keys(map).length; }
      catch (e) { this.stats.failed++; this.stats.lastError = String((e as Error | undefined)?.message ?? e).slice(0, 200); }
    })();
    const p: Promise<void> = run.finally(() => { if (this.busy === p) this.busy = null; }); this.busy = p;
    return p;
  }
  summary(): string { const S = this.stats; return `prefs: ${S.puts} saves (rev ${S.rev}, ${S.keys} keys)${S.failed ? ` · ${S.failed} failed (${S.lastError})` : ''}`; }
}
