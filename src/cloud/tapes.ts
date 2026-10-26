// THE KEPT TAPES GO UP: what this browser already keeps of every page visit (src/replay/keeper.ts files a piece every few seconds into keepstore.ts) is sent to
// the backend's `tapes:put`, a round every `secs`. It is a SYNC OF THE STORE, not a second recorder: a round reads the store's rows, and for every visit with
// pieces the backend has not been sent it sends them, newest visit first — so a visit whose page closed mid-way is finished by the next page that opens, and a
// call that failed is simply tried again next round. A piece goes up as the gzip member the store holds (one written plain as its page went away is packed
// here), cut into parts where it is past one document; the head goes with it whenever it changed.
// ONLY READS the store (rows · head · piecesFrom): nothing of the keeper's, the dump's or the viewer's is written, and a store that keeps nothing sends nothing.
// WHAT A ROUND COSTS THE PAGE: it reads only the pieces numbered past what went (never a visit's whole tape again), a packed piece goes as the bytes the
// store holds — it is NOT unpacked — and a visit whose row has not moved since a round sent all of it is not read at all. What is left on the page's own
// thread is the wire's: a call's bytes to base64 and to JSON (`call` bounds one call). Never measured on a real page beside a fight.
// How far each visit has gone is this module's own note (`TapeNotes`: the page's is localStorage `twopointsix.cloud.tapes`); with the note lost, the backend
// is asked what it has. No game code is imported.
import type { KeptStore, KeptRow, Body } from '../replay/keepstore';
import type { Wire } from './wire';
import { WireError } from './wire';

/** `secs` between rounds · `chunk` bytes in one part (the backend's own bound is 700 KB a document) · `call` bytes and `parts` in one call (small: what a call
 *  costs the page's own thread grows with it) · `round` bytes one round sends at most (a browser holding hours of tape from a day offline catches up over
 *  several) · `notes` visits remembered */
export const TAPESYNC = { secs: 20, chunk: 384 * 1024, call: 512 * 1024, parts: 32, round: 12 * 1048576, notes: 400 };

/** how far a visit has gone up: `n` the NEXT PIECE NUMBER to send (the store keys a piece by its number, and so does the backend: a count of bodies parts
 *  from both where the store refused a piece or a visit was begun again under its name), the head that went (its length and moment: a head is the floor's
 *  whole drawing, not to be held twice), and `at` the row's last moment when a round last sent all there was (a row that has not moved since is not read) */
export interface TapeNote { n: number; head: string; at?: string }
export interface TapeNotes { get(): Record<string, TapeNote>; set(all: Record<string, TapeNote>): void }
export const memoryNotes = (): TapeNotes => { let all: Record<string, TapeNote> = {}; return { get: () => all, set: a => { all = a; } }; };

export interface TapeSyncOpts {
  pid: string;
  notes?: TapeNotes;
  gz?: ((text: string) => Promise<Blob>) | null;
  /** the page may send right now (the switch is read every round) */
  may?: () => boolean;
  /** where given, ONLY the visits it answers true for go up (the page's list of visits that were played, start to end, while the browser sent its tapes):
   *  any other is never asked about, never read, never sent */
  only?: (sid: string) => boolean;
}
type Store = Pick<KeptStore, 'rows' | 'head' | 'piecesFrom'>;
interface Part { seq: number; part: number; parts: number; at: string; body: Uint8Array }

/** (the check's red path: CLOUD_BREAK=1 in front of tools/qa/cloud-check.ts — every third piece is taken as sent and is not; a page has no process) */
const BREAK = (() => { try { return String((globalThis as { process?: { env?: Record<string, string | undefined> } }).process?.env?.CLOUD_BREAK ?? '') === '1'; } catch { return false; } })();
const HEAD = /^\{"k":"head","sid":"[^"]*","at":"([^"]*)"/;
const bytesOf = async (b: Blob): Promise<Uint8Array> => new Uint8Array(await b.arrayBuffer());
const isGz = (u: Uint8Array): boolean => u[0] === 0x1f && u[1] === 0x8b;

export class TapeSync {
  readonly stats = { rounds: 0, calls: 0, parts: 0, pieces: 0, bytes: 0, failed: 0, visits: 0, lastError: '' };
  private busy: Promise<void> | null = null;
  private seen = new Set<string>();
  private notes: TapeNotes;
  private gz: ((text: string) => Promise<Blob>) | null;
  constructor(private w: Wire, private store: Store, private o: TapeSyncOpts) {
    this.notes = o.notes ?? memoryNotes();
    this.gz = o.gz !== undefined ? o.gz : typeof CompressionStream === 'undefined' ? null : (text: string) => new Response(new Blob([text]).stream().pipeThrough(new CompressionStream('gzip'))).blob();
  }
  /** one round; a round asked for while one runs waits for that one instead. Never rejects */
  round(): Promise<void> { return this.busy ??= this.round1().catch(e => { this.stats.failed++; this.stats.lastError = String((e as Error | undefined)?.message ?? e).slice(0, 200); }).finally(() => { this.busy = null; }); }

  private async round1(): Promise<void> {
    if (this.o.may && !this.o.may()) return;
    this.stats.rounds++;
    const rows = (await this.store.rows()).filter(r => r.pieces > 0 && (!this.o.only || this.o.only(r.sid))).sort((a, b) => a.lastAt < b.lastAt ? 1 : -1);
    const all = { ...this.notes.get() };
    // (the note of a visit the store has let go is let go too, oldest first, once there are many)
    { const kept = new Set(rows.map(r => r.sid)), gone = Object.keys(all).filter(s => !kept.has(s)); if (Object.keys(all).length > TAPESYNC.notes) for (const s of gone) delete all[s]; }
    // a visit with no note: the backend says what it has of it (a browser whose notes were cleared, or the same store read by a page of another address)
    const unknown = rows.filter(r => !all[r.sid]).map(r => r.sid).slice(0, 32);
    if (unknown.length) {
      const have = await this.w.query<Record<string, number>>('tapes:have', { pid: this.o.pid, sids: unknown });
      for (const sid of unknown) all[sid] = { n: have[sid] ?? 0, head: '' };
      this.notes.set(all);
    }
    let room = TAPESYNC.round;
    for (const row of rows) {
      if (room <= 0) break;
      const note = all[row.sid]; if (!note) continue;
      if (note.at !== undefined && note.at === row.lastAt) continue;   // (nothing was filed since a round sent all there was: the visit is not read)
      const sent = await this.visit(row, note, room);
      if (sent > 0) room -= sent;
      all[row.sid] = note; this.notes.set(all);
      if (sent < 0) break;   // (a call did not get there: the rest waits for the next round)
    }
  }

  /** send what one visit has past its note; the bytes sent, or -1 where a call did not get there (a call the backend REFUSED skips its pieces: sending
   *  the same again fails the same, and one bad piece must not hold the rest of the tape back for good) */
  private async visit(row: KeptRow, note: TapeNote, room: number): Promise<number> {
    const list = await this.store.piecesFrom(row.sid, note.n), headText = await this.store.head(row.sid);
    const parts: Part[] = [];
    // the head, where it is not the one that went
    let headKey = note.head;
    if (headText) {
      const at = HEAD.exec(headText)?.[1] ?? row.lastAt, key = `${headText.length}@${at}`;
      if (key !== note.head) { headKey = key; for (const p of await this.cut(-1, at, headText)) parts.push(p); }
    }
    const ends: { upto: number; next: number }[] = [];   // (a piece is whole in the list once parts.length is `upto`; `next` = the number after it)
    let size = parts.reduce((a, p) => a + p.body.length, 0), took = 0;
    for (const e of list) {
      if (size >= room) break;
      let got: Part[];
      try { got = BREAK && e.seq % 3 === 2 ? [] : await this.piece(e.body, e.seq, row); } catch { got = []; }   // (a body that does not read is a missing piece, as it is to the dump)
      for (const p of got) { parts.push(p); size += p.body.length; }
      ends.push({ upto: parts.length, next: e.seq + 1 }); took++;
    }
    const whole = took === list.length;   // (all the store had of the visit is in this round's list: once it has gone, the row is not read again until it moves)
    if (!parts.length) { if (ends.length) note.n = ends[ends.length - 1].next; if (whole) note.at = row.lastAt; return 0; }
    const meta = { startedAt: row.startedAt, lastAt: row.lastAt, scenario: row.scenario, url: row.url.slice(0, 2000), ...(row.commit ? { commit: row.commit } : {}), seconds: row.seconds, pieces: row.pieces, bytes: row.bytes };
    let sent = 0, from = 0;
    while (from < parts.length) {
      let to = from, bytes = 0;
      while (to < parts.length && to - from < TAPESYNC.parts && (to === from || bytes + parts[to].body.length <= TAPESYNC.call)) { bytes += parts[to].body.length; to++; }
      try { await this.w.mutation('tapes:put', { pid: this.o.pid, sid: row.sid, row: meta, chunks: parts.slice(from, to) }); this.stats.calls++; this.stats.parts += to - from; this.stats.bytes += bytes; sent += bytes; }
      catch (e) {
        this.stats.failed++; this.stats.lastError = String((e as Error | undefined)?.message ?? e).slice(0, 200);
        if (!(e instanceof WireError && e.refused)) return -1;
      }
      from = to;
      // (the note moves over every piece whose parts have all been asked, and takes the head once its parts have)
      while (ends.length && ends[0].upto <= from) { note.n = ends.shift()!.next; this.stats.pieces++; }
      if (parts[0].seq < 0 && (from >= parts.length || parts[from].seq >= 0)) note.head = headKey;
    }
    if (whole) note.at = row.lastAt;
    if (sent > 0 && !this.seen.has(row.sid)) { this.seen.add(row.sid); this.stats.visits++; }
    return sent;
  }

  /** one store body as its parts, under the number the store keeps it by. A packed body goes as the bytes it is — it is not unpacked to be read — and its
   *  moment on the wire is the visit's start: the same at every sending of the piece, which is all the backend asks of it */
  private async piece(b: Body, seq: number, row: KeptRow): Promise<Part[]> {
    if (typeof b === 'string') return this.cut(seq, row.startedAt, b);
    const raw = await bytesOf(b);
    return isGz(raw) ? this.cut(seq, row.startedAt, '', raw) : this.cut(seq, row.startedAt, new TextDecoder().decode(raw));
  }
  /** a line as parts of at most `chunk` bytes: the gzip member it already is (`packed`), or packed here where the browser can, or as its plain text */
  private async cut(seq: number, at: string, text: string, packed: Uint8Array | null = null): Promise<Part[]> {
    let u = packed;
    if (!u) { try { u = this.gz ? await bytesOf(await this.gz(text)) : new TextEncoder().encode(text); } catch { u = new TextEncoder().encode(text); } }
    const parts = Math.max(1, Math.ceil(u.length / TAPESYNC.chunk)), out: Part[] = [];
    for (let p = 0; p < parts; p++) out.push({ seq, part: p, parts, at, body: u.subarray(p * TAPESYNC.chunk, (p + 1) * TAPESYNC.chunk) });
    return out;
  }
  summary(): string { const S = this.stats; return `tapes: ${S.pieces} pieces of ${S.visits} visit${S.visits === 1 ? '' : 's'} sent (${(S.bytes / 1024).toFixed(0)} KB, ${S.calls} calls)${S.failed ? ` · ${S.failed} failed (${S.lastError})` : ''}`; }
}
