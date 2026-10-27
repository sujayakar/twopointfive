// WHAT THE PLAYER WROTE ON A REPLAY, SENT UP (cloud/convex/notes.ts): the notes put on a kept visit in the viewer, and the F7 shelf's saved windows with
// theirs. The kept tapes' hand-over file used to carry these (keepstore.ts makeDump); with that file gone from the page this is their road, and
// tools/cloud/pull.ts writes them back into the hand-over file it makes, as the same lines (kept.ts: `{k:'notes'}`, `{k:'f7'}`), so the reader reads them
// as it always did. A SYNC OF WHAT THE BROWSER HOLDS, like the tapes': a round reads the kept rows and the shelf and sends a line only where its text
// differs from the one that last went (`marks`, kept in the browser). A note taken off again goes as a line with none. A saved window whose visit is not
// kept here (the trace is in no kept tape) goes up whole, once, through the saved files' road (files.ts) as kind 'f7'.
// No game code and no DOM: the check drives it under bun with a store and a shelf of its own.
import type { Wire } from './wire';

/** `secs`: a round this often. `max`: a line past this many characters stays in the browser. `batch`: lines in one call. `slackMs`: a visit kept up to
 *  this long before a window was saved still stands in for it (the keeper's beats: keepstore.ts makeDump read it the same) */
export const NOTESYNC = { secs: 30, max: 200 * 1024, batch: 8, slackMs: 20000 };

/** (the check's red path: CLOUD_BREAK=1 in front of tools/qa/cloud-check.ts — a line is marked as gone and was never sent; a page has no process) */
const BREAK = (() => { try { return String((globalThis as { process?: { env?: Record<string, string | undefined> } }).process?.env?.CLOUD_BREAK ?? '') === '1'; } catch { return false; } })();

export interface NotedRow { sid: string; lastAt: string; pieces: number; notes?: unknown[] }
export interface ShelfNoted { id: number; savedAt: string; seconds: number; why: string; scenario: string; session?: string; n1?: number; notes: unknown[] }
export interface NoteSource {
  rows(): Promise<NotedRow[]>;
  /** the F7 shelf's rows with their notes (replay/shelf.ts shelfNoted), and one saved window whole (shelfGet) */
  shelf?(): Promise<ShelfNoted[]>;
  shelfFile?(id: number): Promise<unknown | null>;
}
export interface NoteMarks { get(): Record<string, string>; set(all: Record<string, string>): void }
export interface NoteOpts {
  pid: string; marks: NoteMarks;
  may?: () => boolean;
  /** a visit whose tape may go (the tapes' own list): the notes of any other stay in the browser with it */
  only?: (sid: string) => boolean;
  /** the saved files' way in (files.ts): true = taken */
  file?: (name: string, blob: Blob, kind: string, meta?: Record<string, string>) => boolean;
  now?: () => number;
}
const mark = (t: string): string => { let h = 2166136261; for (let i = 0; i < t.length; i++) h = Math.imul(h ^ t.charCodeAt(i), 16777619); return `${t.length}:${(h >>> 0).toString(36)}`; };
const safe = (s: string): string => s.replace(/[^A-Za-z0-9_.+-]/g, '-').slice(0, 48);

export class NoteSync {
  private going: Promise<void> | null = null; private skipped = new Set<string>();
  readonly stats = { rounds: 0, sent: 0, files: 0, dropped: 0, failed: 0, lastError: '' };
  constructor(private w: Wire, private src: NoteSource, private o: NoteOpts) {}

  /** send the lines that changed since they last went. One round at a time; never rejects */
  round(): Promise<void> { return this.going ??= this.run().catch(() => { /* the next round */ }).finally(() => { this.going = null; }); }

  private async run(): Promise<void> {
    if (this.o.may && !this.o.may()) return;
    this.stats.rounds++;
    const rows = await this.src.rows(), shelf = (await this.src.shelf?.()) ?? [], marks = { ...this.o.marks.get() }, live = new Set<string>();
    const want: { key: string; text: string }[] = [];
    for (const r of rows) {
      if (this.o.only && !this.o.only(r.sid)) continue;
      const key = `visit:${safe(r.sid)}`; live.add(key);
      // (a row with no notes says so only where it once had some: the backend's line is emptied, not left standing)
      if (r.notes?.length || marks[key]) want.push({ key, text: JSON.stringify({ k: 'notes', sid: r.sid, notes: r.notes ?? [] }) });
    }
    const keptAt = new Map(rows.map(r => [r.sid, Date.parse(r.lastAt)]));
    for (const s of shelf) {
      // (a saved window is a stretch of a visit: where that visit's tape may not go — played before the notice, or with the word off — the window stays too)
      if (this.o.only && !(s.session && this.o.only(s.session))) continue;
      const at = Date.parse(s.savedAt), till = s.session ? keptAt.get(s.session) : undefined, here = till !== undefined && !(till < at - NOTESYNC.slackMs);
      const fkey = `file:${safe(s.savedAt)}:${s.id}`, key = `f7:${safe(s.session ?? '')}:${safe(s.savedAt)}`; live.add(fkey); live.add(key);
      if (!here && !marks[fkey] && !this.skipped.has(fkey) && this.o.file && this.src.shelfFile) {
        const f = await this.src.shelfFile(s.id); let took = false;
        if (f) try { took = this.o.file(`f7-${safe(s.savedAt)}.json`, new Blob([JSON.stringify(f)], { type: 'application/json' }), 'f7', { ...(s.session ? { session: s.session } : {}), savedAt: s.savedAt, why: String(s.why ?? '').slice(0, 200), scenario: String(s.scenario ?? '').slice(0, 120) }); } catch { /* not taken */ }
        if (took) { marks[fkey] = '1'; this.stats.files++; } else this.skipped.add(fkey);   // (not taken — the files' word is off, it is too big: not read again this visit)
      }
      want.push({ key, text: JSON.stringify({ k: 'f7', ...(s.session ? { session: s.session } : {}), savedAt: s.savedAt, ...(s.n1 !== undefined ? { n: s.n1 } : {}), seconds: s.seconds, why: s.why, scenario: s.scenario, notes: s.notes ?? [] }) });
    }
    const changed = want.filter(x => { if (x.text.length > NOTESYNC.max) { if (marks[x.key] !== 'big') { marks[x.key] = 'big'; this.stats.dropped++; } return false; } return marks[x.key] !== mark(x.text); });
    for (let i = 0; i < changed.length; i += NOTESYNC.batch) {
      if (this.o.may && !this.o.may()) break;
      const part = changed.slice(i, i + NOTESYNC.batch), at = (this.o.now ?? Date.now)();
      try { if (!BREAK) await this.w.mutation('notes:put', { pid: this.o.pid, items: part.map(x => ({ key: x.key, at, text: x.text })) }); for (const x of part) marks[x.key] = mark(x.text); this.stats.sent += part.length; }
      catch (e) {
        this.stats.failed++; this.stats.lastError = String((e as Error | undefined)?.message ?? e).slice(0, 200);
        if (!(e as { refused?: boolean })?.refused) break;
        for (const x of part) marks[x.key] = mark(x.text);   // (refused: sending the same again fails the same)
        this.stats.dropped += part.length;
      }
    }
    // (a visit the store let go, a window the shelf dropped: their marks go with them; the backend keeps what it was sent)
    for (const k of Object.keys(marks)) if (!live.has(k)) delete marks[k];
    this.o.marks.set(marks);
  }

  summary(): string { const s = this.stats; return `notes: ${s.sent} lines sent, ${s.files} saved windows${s.failed ? ` · ${s.failed} failed (${s.lastError})` : ''}`; }
}
