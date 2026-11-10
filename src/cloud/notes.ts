// WHAT THE PLAYER WROTE ON A REPLAY, SENT UP (cloud/convex/notes.ts): the notes put on a kept visit. tools/cloud/pull.ts writes them back into the hand-over file it
// makes, as the same lines (kept.ts: `{k:'notes'}`), so the reader reads them as it always did. A SYNC OF WHAT THE BROWSER HOLDS, like the tapes': a round reads the
// kept rows and sends a line only where its text differs from the one that last went (`marks`, kept in the browser). A note taken off again goes as a line with none.
// No game code and no DOM: the check drives it under bun with a store of its own.
import type { Wire } from './wire';

/** `secs`: a round this often. `max`: a line past this many characters stays in the browser. `batch`: lines in one call */
export const NOTESYNC = { secs: 30, max: 200 * 1024, batch: 8 };

/** (the check's red path: CLOUD_BREAK=1 in front of tools/qa/cloud-check.ts — a line is marked as gone and was never sent; a page has no process) */
const BREAK = (() => { try { return String((globalThis as { process?: { env?: Record<string, string | undefined> } }).process?.env?.CLOUD_BREAK ?? '') === '1'; } catch { return false; } })();

export interface NotedRow { sid: string; lastAt: string; pieces: number; notes?: unknown[] }
export interface NoteSource { rows(): Promise<NotedRow[]> }
export interface NoteMarks { get(): Record<string, string>; set(all: Record<string, string>): void }
export interface NoteOpts {
  pid: string; marks: NoteMarks;
  may?: () => boolean;
  /** a visit whose tape may go (the tapes' own list): the notes of any other stay in the browser with it */
  only?: (sid: string) => boolean;
  now?: () => number;
}
const mark = (t: string): string => { let h = 2166136261; for (let i = 0; i < t.length; i++) h = Math.imul(h ^ t.charCodeAt(i), 16777619); return `${t.length}:${(h >>> 0).toString(36)}`; };
const safe = (s: string): string => s.replace(/[^A-Za-z0-9_.+-]/g, '-').slice(0, 48);

export class NoteSync {
  private going: Promise<void> | null = null;
  readonly stats = { rounds: 0, sent: 0, dropped: 0, failed: 0, lastError: '' };
  constructor(private w: Wire, private src: NoteSource, private o: NoteOpts) {}

  /** send the lines that changed since they last went. One round at a time; never rejects */
  round(): Promise<void> { return this.going ??= this.run().catch(() => { /* the next round */ }).finally(() => { this.going = null; }); }

  private async run(): Promise<void> {
    if (this.o.may && !this.o.may()) return;
    this.stats.rounds++;
    const rows = await this.src.rows(), marks = { ...this.o.marks.get() }, live = new Set<string>();
    const want: { key: string; text: string }[] = [];
    for (const r of rows) {
      if (this.o.only && !this.o.only(r.sid)) continue;
      const key = `visit:${safe(r.sid)}`; live.add(key);
      // (a row with no notes says so only where it once had some: the backend's line is emptied, not left standing)
      if (r.notes?.length || marks[key]) want.push({ key, text: JSON.stringify({ k: 'notes', sid: r.sid, notes: r.notes ?? [] }) });
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
    // (a visit the store let go: its mark goes with it; the backend keeps what it was sent)
    for (const k of Object.keys(marks)) if (!live.has(k)) delete marks[k];
    this.o.marks.set(marks);
  }

  summary(): string { const s = this.stats; return `notes: ${s.sent} lines sent${s.failed ? ` · ${s.failed} failed (${s.lastError})` : ''}`; }
}
