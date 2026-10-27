// THE KEPT TAPES' STORE: where the pieces of every page visit's tape live in this browser (kept.ts has the format and why), what is let go when the room runs
// out, and how a visit is read back out of it. IndexedDB, a database of its own (the F7 shelf's is not touched), three stores:
//   `visits`  one small row a page visit: when, what, how many pieces, how many bytes, notes put on it
//   `heads`   the visit's head line as it stands (rewritten only when the switches or the floor changed)
//   `pieces`  [visit, number] → the piece's line, gzipped where the browser can (a Blob), the plain text where it was written as the page was going away
// Every call resolves: a browser without IndexedDB, a private window or a full disk is a store that says it did not keep it — the game is never held up.
// NOTHING HERE TALKS TO A NETWORK. (The page's own hand-over — everything kept as one downloaded file, with a mark on each row of what a file had carried —
// was taken out on 2026-10-27: a hosted build's backend reads this store instead, src/cloud/tapes.ts. The file's format and its reader stay: kept.ts,
// tools/qa/replay-dump.ts. A row written by an older page may still carry `sent`, `sentAt` or `noted`: nobody reads them.)
// A memory twin (`memoryStore`) stands in under bun, where the headless check lays the same pieces through the same code.
import type { ReplayNote } from './format';
import type { KeptLine, Visit } from './kept';
import { parseLines, visitsOf } from './kept';

/** `secs` between pieces · `maxMB` of kept tapes at most, and never more than `share` of what the browser says this site may use · `openMs` an open of the
 *  database is waited on (shelf.ts SHELF.openMs: a browser can leave the request with no event at all) */
export const KEEP = { secs: 5, maxMB: 600, share: 0.5, openMs: 5000 };

export interface KeptRow {
  sid: string; startedAt: string; lastAt: string; scenario: string; url: string; commit?: string;
  /** seconds of play kept, pieces written, bytes they take */
  seconds: number; pieces: number; bytes: number;
  notes?: ReplayNote[];
  /** its page said it was going (pagehide): nobody is playing it. NEVER STORED ON THE ROW — `rows()` reads it off the store's own mark (`left` below), which a
   *  going page can write without reading the row back, and so without laying an old row over a note another tab wrote since */
  left?: boolean;
}
export type Body = Blob | string;
export interface KeptStore {
  /** one piece and the visit's row, together (the row's `notes` are whoever wrote them last: never overwritten from here) */
  put(row: KeptRow, seq: number, body: Body, head?: string): Promise<boolean>;
  /** `put` for a page that is GOING AWAY: every request is asked in this very turn and the transaction told to commit, with nothing read back first (a
   *  transaction left waiting on a read when its page dies is thrown away whole, piece and row). The row is laid over the row this page last saw stored, so a
   *  note another tab wrote since this page's last put is the one thing it can miss. Absent = the store has no such hurry (`put` serves). */
  putNow?(row: KeptRow, seq: number, body: Body, head?: string): Promise<boolean>;
  /** the visit's page says it is going (`gone`), or that it is back and being played after all. One key of its own, asked in this very turn and told to
   *  commit: no row is read, written or made by it (a mark on a visit that was dropped lists nothing), and `drop` takes it with the visit. */
  left(sid: string, gone: boolean): Promise<boolean>;
  rows(): Promise<KeptRow[]>;
  head(sid: string): Promise<string | null>;
  bodies(sid: string): Promise<Body[]>;
  /** the visit's pieces numbered `from` and up, each with its number, in order — a read only, and only of those (the backend's sync, src/cloud/tapes.ts, asks
   *  for what it has not sent: a long visit's whole tape is not read again every round) */
  piecesFrom(sid: string, from: number): Promise<{ seq: number; body: Body }[]>;
  drop(sid: string): Promise<void>;
  patch(sid: string, fields: Partial<KeptRow>): Promise<boolean>;
}

const DB = 'twopointsix-kept', VISITS = 'visits', HEADS = 'heads', PIECES = 'pieces';
function open(): Promise<IDBDatabase | null> {
  return new Promise(res => {
    let over = false;
    const end = (db: IDBDatabase | null) => { if (over) { try { db?.close(); } catch { /* */ } return; } over = true; clearTimeout(t); res(db); };
    const t = setTimeout(() => end(null), KEEP.openMs);
    try {
      const rq = indexedDB.open(DB, 1);
      rq.onupgradeneeded = () => { const db = rq.result, has = (n: string) => db.objectStoreNames.contains(n);
        if (!has(VISITS)) db.createObjectStore(VISITS, { keyPath: 'sid' }); if (!has(HEADS)) db.createObjectStore(HEADS); if (!has(PIECES)) db.createObjectStore(PIECES); };
      rq.onsuccess = () => end(rq.result); rq.onerror = () => end(null); rq.onblocked = () => end(null);
    } catch { end(null); }
  });
}
const done = <T>(rq: IDBRequest<T>): Promise<T | null> => new Promise(res => { rq.onsuccess = () => res(rq.result); rq.onerror = () => res(null); });
const settled = (tx: IDBTransaction): Promise<boolean> => new Promise(res => { tx.oncomplete = () => res(true); tx.onabort = () => res(false); tx.onerror = () => res(false); });
const span = (sid: string) => IDBKeyRange.bound([sid, 0], [sid, Number.MAX_SAFE_INTEGER]);
/** where a visit's 'its page has gone' mark lives: the heads' store, under a key no visit id can be (an array; a head's key is the id, a string). No store of its
 *  own: that would be a new version of the database, and a page of an older build on this origin could then open none of it. */
const leftKey = (sid: string): [string, string] => ['left', sid];
const isLeftKey = (k: unknown): k is [string, string] => Array.isArray(k) && k.length === 2 && k[0] === 'left' && typeof k[1] === 'string';
/** the row as the keeper knows it laid over the row as stored: the notes the viewer wrote on it stay */
const merged = (old: KeptRow | undefined, row: KeptRow): KeptRow => ({ ...row, ...(old?.notes ? { notes: old.notes } : {}) });

/** this browser's store. One connection for the page's life (a piece every few seconds: not an open each); a call made before it is open waits for it. */
export function browserStore(): KeptStore {
  let dbp: Promise<IDBDatabase | null> | null = null, open1: IDBDatabase | null = null;
  /** each visit's row as this page last saw it stored (what `putNow` lays its row over) */
  const seen = new Map<string, KeptRow>();
  const db = () => dbp ??= open().then(d => { open1 = d; if (d) d.onversionchange = () => { try { d.close(); } catch { /* */ } dbp = null; open1 = null; }; else dbp = null; return d; });
  const now = (d: IDBDatabase, row: KeptRow, seq: number, body: Body, head?: string): Promise<boolean> => {
    const tx = d.transaction(head === undefined ? [VISITS, PIECES] : [VISITS, PIECES, HEADS], 'readwrite'), ok = settled(tx), m = merged(seen.get(row.sid), row);
    tx.objectStore(VISITS).put(m); tx.objectStore(PIECES).put(body, [row.sid, seq]);
    if (head !== undefined) tx.objectStore(HEADS).put(head, row.sid);
    try { tx.commit?.(); } catch { /* a browser without it commits by itself, if the page lives that long */ }
    seen.set(row.sid, m);
    return ok;
  };
  const run = async <T>(miss: T, f: (d: IDBDatabase) => Promise<T>): Promise<T> => { const d = await db(); if (!d) return miss; try { return await f(d); } catch { return miss; } };
  return {
    put: (row, seq, body, head) => run(false, d => {
      const tx = d.transaction(head === undefined ? [VISITS, PIECES] : [VISITS, PIECES, HEADS], 'readwrite'), ok = settled(tx), V = tx.objectStore(VISITS), rq = V.get(row.sid);
      rq.onsuccess = () => { const m = merged(rq.result as KeptRow | undefined, row); seen.set(row.sid, m); V.put(m); };
      tx.objectStore(PIECES).put(body, [row.sid, seq]);
      if (head !== undefined) tx.objectStore(HEADS).put(head, row.sid);
      return ok;
    }),
    putNow: (row, seq, body, head) => { const d = open1; if (!d) return run(false, d2 => now(d2, row, seq, body, head)); try { return now(d, row, seq, body, head); } catch { return Promise.resolve(false); } },
    rows: () => run([] as KeptRow[], async d => { const tx = d.transaction([VISITS, HEADS], 'readonly'), rq = tx.objectStore(VISITS).getAll(), kq = tx.objectStore(HEADS).getAllKeys();
      const [got, keys] = await Promise.all([done(rq), done(kq)]);   // (both asked for before either is waited on: a request's answer is heard only by a handler already on it)
      const rows = (got as KeptRow[] | null) ?? [], gone = new Set(((keys as unknown[] | null) ?? []).filter(isLeftKey).map(k => k[1]));
      return rows.map(r => { const { left: _stored, ...rest } = r; return gone.has(r.sid) ? { ...rest, left: true } : rest; }); }),
    left: (sid, gone) => { const f = (d: IDBDatabase): Promise<boolean> => { const tx = d.transaction(HEADS, 'readwrite'), ok = settled(tx), H = tx.objectStore(HEADS); if (gone) H.put(1, leftKey(sid)); else H.delete(leftKey(sid)); try { tx.commit?.(); } catch { /* */ } return ok; };
      const d = open1; if (!d) return run(false, f); try { return f(d); } catch { return Promise.resolve(false); } },
    head: sid => run(null as string | null, async d => { const h = await done(d.transaction(HEADS, 'readonly').objectStore(HEADS).get(sid)); return typeof h === 'string' ? h : null; }),
    bodies: sid => run([] as Body[], async d => ((await done(d.transaction(PIECES, 'readonly').objectStore(PIECES).getAll(span(sid)))) as Body[] | null) ?? []),
    piecesFrom: (sid, from) => run([] as { seq: number; body: Body }[], async d => {   // (both asked of one transaction before either is waited on: the keys and the bodies are of one moment)
      const P = d.transaction(PIECES, 'readonly').objectStore(PIECES), range = IDBKeyRange.bound([sid, Math.max(0, from)], [sid, Number.MAX_SAFE_INTEGER]), bq = P.getAll(range), kq = P.getAllKeys(range);
      const [bodies, keys] = await Promise.all([done(bq), done(kq)]) as [Body[] | null, [string, number][] | null]; if (!bodies || !keys || bodies.length !== keys.length) return [];
      return bodies.map((body, i) => ({ seq: keys[i][1], body }));
    }),
    drop: sid => run(undefined, async d => { const tx = d.transaction([VISITS, HEADS, PIECES], 'readwrite'), ok = settled(tx); tx.objectStore(VISITS).delete(sid); tx.objectStore(HEADS).delete(sid); tx.objectStore(HEADS).delete(leftKey(sid)); tx.objectStore(PIECES).delete(span(sid)); await ok; }),
    patch: (sid, fields) => run(false, d => { const tx = d.transaction(VISITS, 'readwrite'), ok = settled(tx), V = tx.objectStore(VISITS), rq = V.get(sid);
      rq.onsuccess = () => { const row = rq.result as KeptRow | undefined; if (row) { const m = { ...row, ...fields }; if (seen.has(sid)) seen.set(sid, m); V.put(m); } }; return ok; }),
  };
}
/** the same store in memory (the headless check; `full` = bytes it will hold before a put is refused, as a full disk refuses one) */
export function memoryStore(full = Infinity): KeptStore & { size(): number } {
  const V = new Map<string, KeptRow>(), H = new Map<string, string>(), P = new Map<string, Map<number, Body>>(), L = new Set<string>();
  const len = (b: Body) => typeof b === 'string' ? b.length : b.size, size = () => { let n = 0; for (const m of P.values()) for (const b of m.values()) n += len(b); return n; };
  return {
    size,
    put: async (row, seq, body, head) => { if (size() + len(body) > full) return false; V.set(row.sid, merged(V.get(row.sid), row)); let m = P.get(row.sid); if (!m) P.set(row.sid, m = new Map()); m.set(seq, body); if (head !== undefined) H.set(row.sid, head); return true; },
    rows: async () => [...V.values()].map(r => { const { left: _stored, ...rest } = r; return L.has(r.sid) ? { ...rest, left: true } : rest; }),
    left: async (sid, gone) => { if (gone) L.add(sid); else L.delete(sid); return true; },
    head: async sid => H.get(sid) ?? null,
    bodies: async sid => [...(P.get(sid) ?? new Map<number, Body>()).entries()].sort((a, b) => a[0] - b[0]).map(e => e[1]),
    piecesFrom: async (sid, from) => [...(P.get(sid) ?? new Map<number, Body>()).entries()].filter(e => e[0] >= from).sort((a, b) => a[0] - b[0]).map(e => ({ seq: e[0], body: e[1] })),
    drop: async sid => { V.delete(sid); H.delete(sid); P.delete(sid); L.delete(sid); },
    patch: async (sid, fields) => { const r = V.get(sid); if (!r) return false; V.set(sid, { ...r, ...fields }); return true; },
  };
}

/** text → one gzip member, where the browser can (null where it cannot: pieces are then kept, and dumped, as plain text) */
export const gzip: ((text: string) => Promise<Blob>) | null = typeof CompressionStream === 'undefined' ? null
  : (text: string) => new Response(new Blob([text]).stream().pipeThrough(new CompressionStream('gzip'))).blob();

/** how many bytes of tapes this browser keeps: `maxMB`, or `share` of what it says the site may use if that is less */
export async function keepBudget(): Promise<number> {
  let max = KEEP.maxMB * 1048576;
  try { const e = await navigator.storage?.estimate?.(); if (e?.quota) max = Math.min(max, e.quota * KEEP.share); } catch { /* */ }
  return max;
}
/** a visit some page wrote to this recently is taken to be still being played (a page files every KEEP.secs) — unless its page said it was going (`left`):
 *  a game just left for the list of replays is not being played, and its row must not say so for a minute */
export const WRITING_MS = 60000;
/** (the check's red path: REPLAYLEFT_BREAK=1 in front of tools/qa/replaykeep-probes.ts — a page has no process) */
const LEFT_BREAK = (() => { try { return String((globalThis as { process?: { env?: Record<string, string | undefined> } }).process?.env?.REPLAYLEFT_BREAK ?? '') === '1'; } catch { return false; } })();
export const beingWritten = (r: KeptRow, nowMs = Date.now()): boolean => { if (r.left && !LEFT_BREAK) return false; const t = Date.parse(r.lastAt); return Number.isFinite(t) && Math.abs(nowMs - t) < WRITING_MS; };
/** LET GO OF THE OLDEST until the kept tapes fit `max` bytes, oldest first — never `live`
 *  (the visit being written), nor a visit written inside the last WRITING_MS whose page has not said it left (still being played). Whole visits only: half a tape is not a tape.
 *  Returns what was dropped. */
export async function pruneKept(store: KeptStore, max: number, live: string | null = null, nowMs = Date.now()): Promise<KeptRow[]> {
  const rows = await store.rows(); let total = 0; for (const r of rows) total += r.bytes;
  if (total <= max) return [];
  const order = rows.filter(r => r.sid !== live && !beingWritten(r, nowMs)).sort((a, b) => a.lastAt < b.lastAt ? -1 : a.lastAt > b.lastAt ? 1 : 0);
  const out: KeptRow[] = [];
  for (const r of order) { if (total <= max) break; await store.drop(r.sid); total -= r.bytes; out.push(r); }
  return out;
}

export interface KeptSummary { visits: number; seconds: number; bytes: number; rows: KeptRow[]; }
/** what is kept, newest first */
export async function keptSummary(store: KeptStore): Promise<KeptSummary> {
  const rows = (await store.rows()).sort((a, b) => a.lastAt < b.lastAt ? 1 : -1), S: KeptSummary = { visits: rows.length, seconds: 0, bytes: 0, rows };
  for (const r of rows) { S.seconds += r.seconds; S.bytes += r.bytes; }
  return S;
}

/** a piece's body as its text (a gzip member where the browser packed it) */
async function textOf(b: Body): Promise<string> {
  if (typeof b === 'string') return b;
  const u = new Uint8Array(await b.arrayBuffer());
  if (u[0] === 0x1f && u[1] === 0x8b) return new Response(new Blob([u]).stream().pipeThrough(new DecompressionStream('gzip'))).text();
  return new TextDecoder().decode(u);
}
/** one kept visit as its records, read out of the store (kept.ts assemble makes a replay file of it); null: not kept, or nothing of it reads */
export async function readVisit(store: KeptStore, sid: string): Promise<Visit | null> {
  const head = await store.head(sid), bodies = await store.bodies(sid); if (!head || !bodies.length) return null;
  const lines: KeptLine[] = []; parseLines(head, lines);
  for (const b of bodies) { try { parseLines(await textOf(b), lines); } catch { /* a piece that does not unpack is a missing piece */ } }
  const V = visitsOf(lines).get(sid) ?? null;
  if (V) { const row = (await store.rows()).find(r => r.sid === sid); if (row?.notes) V.notes = row.notes.slice(); }
  return V;
}

