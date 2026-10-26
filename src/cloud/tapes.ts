// THE KEPT TAPES GO UP: what this browser already keeps of every page visit (src/replay/keeper.ts files a piece every few seconds into keepstore.ts) is sent to
// the backend's `tapes:put`, a round every `secs`. It is a SYNC OF THE STORE, not a second recorder: a round reads the store's rows, and for every visit with
// pieces the backend has not been sent it sends them, newest visit first — so a visit whose page closed mid-way is finished by the next page that opens, and a
// call that failed is simply tried again next round. A piece goes up as the gzip member the store holds (one written plain as its page went away is packed
// here), cut into parts where it is past one document; the head goes with it whenever it changed.
// ONLY READS the store (rows · head · bodies): nothing of the keeper's, the dump's or the viewer's is written, and a store that keeps nothing sends nothing.
// How far each visit has gone is this module's own note (`TapeNotes`: the page's is localStorage `twopointsix.cloud.tapes`); with the note lost, the backend
// is asked what it has. No game code is imported.
import type { KeptStore, KeptRow, Body } from '../replay/keepstore';
import type { Wire } from './wire';
import { WireError } from './wire';

/** `secs` between rounds · `chunk` bytes in one part (the backend's own bound is 700 KB a document) · `call` bytes and `parts` in one call ·
 *  `round` bytes one round sends at most (a browser holding hours of tape from a day offline catches up over several) · `notes` visits remembered */
export const TAPESYNC = { secs: 20, chunk: 512 * 1024, call: 1536 * 1024, parts: 32, round: 12 * 1048576, notes: 400 };

/** how far a visit has gone up: store bodies sent, and the head that went (its length and moment: a head is the floor's whole drawing, not to be held twice) */
export interface TapeNote { n: number; head: string }
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
type Store = Pick<KeptStore, 'rows' | 'head' | 'bodies'>;
interface Part { seq: number; part: number; parts: number; at: string; body: Uint8Array }

/** (the check's red path: CLOUD_BREAK=1 in front of tools/qa/cloud-check.ts — every third piece is taken as sent and is not; a page has no process) */
const BREAK = (() => { try { return String((globalThis as { process?: { env?: Record<string, string | undefined> } }).process?.env?.CLOUD_BREAK ?? '') === '1'; } catch { return false; } })();
const PIECE = /^\{"k":"piece","sid":"[^"]*","seq":(\d+),"at":"([^"]*)"/, HEAD = /^\{"k":"head","sid":"[^"]*","at":"([^"]*)"/;
const bytesOf = async (b: Blob): Promise<Uint8Array> => new Uint8Array(await b.arrayBuffer());
const isGz = (u: Uint8Array): boolean => u[0] === 0x1f && u[1] === 0x8b;
async function gunzipText(u: Uint8Array): Promise<string> { return new Response(new Blob([u as BlobPart]).stream().pipeThrough(new DecompressionStream('gzip'))).text(); }

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
      if (note.n >= row.pieces && note.head !== '') {   // (nothing new by the row's own count: the bodies are not read — unless the head may have changed, which only a live visit's does)
        if (Date.parse(row.lastAt) < Date.now() - 120000) continue;
      }
      const sent = await this.visit(row, note, room);
      room -= sent; all[row.sid] = note; this.notes.set(all);
      if (sent < 0) break;   // (a call did not get there: the rest waits for the next round)
    }
  }

  /** send what one visit has that its note does not; the bytes sent, or -1 where a call did not get there (a call the backend REFUSED skips its pieces: sending
   *  the same again fails the same, and one bad piece must not hold the rest of the tape back for good) */
  private async visit(row: KeptRow, note: TapeNote, room: number): Promise<number> {
    const bodies = await this.store.bodies(row.sid), headText = await this.store.head(row.sid);
    if (bodies.length < note.n) note.n = 0;   // (the visit was removed and begun again under its name: keeper.ts back)
    const parts: Part[] = [];
    // the head, where it is not the one that went
    let headKey = note.head;
    if (headText) {
      const at = HEAD.exec(headText)?.[1] ?? row.lastAt, key = `${headText.length}@${at}`;
      if (key !== note.head) { headKey = key; for (const p of await this.cut(-1, at, headText)) parts.push(p); }
    }
    let n = note.n; const ends: number[] = [];   // (ends[i] = parts.length once body n+i is whole in the list)
    let size = parts.reduce((a, p) => a + p.body.length, 0);
    for (let i = note.n; i < bodies.length && size < room; i++) {
      let got: Part[];
      try { got = BREAK && i % 3 === 2 ? [] : await this.piece(bodies[i], i, row); } catch { got = []; }   // (a body that does not read is a missing piece, as it is to the dump)
      for (const p of got) { parts.push(p); size += p.body.length; }
      ends.push(parts.length);
    }
    if (!parts.length) { if (ends.length) { note.n += ends.length; } return 0; }
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
      // (the note moves over every body whose parts have all been asked, and takes the head once its parts have)
      while (ends.length && ends[0] <= from) { ends.shift(); n++; this.stats.pieces++; }
      note.n = n; if (parts[0].seq < 0 && from > 0 && (from >= parts.length || parts[from].seq >= 0)) note.head = headKey;
    }
    if (sent > 0 && !this.seen.has(row.sid)) { this.seen.add(row.sid); this.stats.visits++; }
    return sent;
  }

  /** one store body as its parts: its number and moment are read off its own line (a body's place in the store is not its number where a put was lost) */
  private async piece(b: Body, index: number, row: KeptRow): Promise<Part[]> {
    let text: string | null = typeof b === 'string' ? b : null, raw: Uint8Array | null = null;
    if (text === null) { raw = await bytesOf(b as Blob); text = isGz(raw) ? await gunzipText(raw) : new TextDecoder().decode(raw); if (!isGz(raw)) raw = null; }
    const m = PIECE.exec(text), seq = m ? Number(m[1]) : index, at = m?.[2] ?? row.lastAt;
    return this.cut(seq, at, text, raw);
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
