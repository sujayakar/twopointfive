// THE KEPT RUNS' FORMAT: every run is kept in the browser with no key pressed, and a set of them travels as one file (tools/cloud/pull.ts writes it from a hosted build's backend,
// tools/qa/replay-dump.ts reads it).
// (the maintainer, 10-11: "maybe a good model would be that we save all replays as we go (in browser storage?) and then i can just periodically dump all of them to
// you (and you dedup?) … without me needing to do a bunch of explicit curation")
// A page visit's tape is kept as PIECES: every few seconds the page hands over what the tape has gained since the last piece, and the piece is written to this browser's own storage
// (keepstore.ts). Pieces laid end to end are the whole visit, every run made on that page load. A DUMP is those pieces as lines of JSON, one line a record (`KeptLine`), gzipped in members
// that simply follow one another:
//   {k:'dump'}   once, first: what the file holds
//   {k:'head'}   a visit's head (build, module, page query, level) — written with its first piece and again whenever it changed; the LAST one stands
//   {k:'hands'}  a piece of the visit's tape of hands: its bytes, and where in it they go (keep.ts)
//   {k:'notes'}  notes put on a kept visit
// and, from builds before the page that draws, recordings of the old kind, of which only the notes are read:
//   {k:'piece'}  a piece of a visit's trace · {k:'f7'} a file its key saved, by its notes · {k:'file'} a whole one
// No import but a type: the reading tools load this file alone.
import type { ReplayNote } from './format';

export const KEPT_VERSION = 1;

/** what a visit's row says of it, and whatever else its head line has */
export interface KeptHead { url?: string; commit?: string; userAgent?: string; seed?: number; level: { file?: string; scenario?: string }; /** the tape of hands' version; none: a recording of the old kind */ hands?: number; /** the module it was played on */ module?: string }
export type KeptLine =
  | { k: 'dump'; v: number; at: string; userAgent?: string; visits: { sid: string; pieces: number; bytes: number; startedAt: string; lastAt: string; sent?: string }[]; shelf: number }
  | ({ k: 'head'; sid: string; at: string } & KeptHead)
  | { k: 'hands'; sid: string; seq: number; at: string; from: number; b: string }
  | { k: 'notes'; sid: string; notes: ReplayNote[] }
  | { k: 'piece'; sid: string }
  | { k: 'f7'; session?: string; savedAt: string; seconds: number; scenario?: string; notes: ReplayNote[] }
  | { k: 'file'; file: OldFile };
/** a whole file of the old kind, as far as it is read */
export interface OldFile { session?: string; savedAt?: string; seconds?: number; url?: string; commit?: string; seed?: number | null; level?: { scenario?: string; file?: string }; notes?: ReplayNote[] }

export const headLine = (sid: string, at: string, h: object): string => JSON.stringify({ k: 'head', sid, at, ...h }) + '\n';

/** lines of JSON as records; a line that does not parse (a write cut short as the page went away) is counted, not thrown */
export function parseLines(text: string, into: KeptLine[] = []): { lines: KeptLine[]; bad: number } {
  let bad = 0;
  for (const row of text.split('\n')) { if (!row.trim()) continue; try { const o = JSON.parse(row) as KeptLine; if (o && typeof o === 'object' && typeof (o as { k?: unknown }).k === 'string') into.push(o); else bad++; } catch { bad++; } }
  return { lines: into, bad };
}

/** a visit's head that stands, and its notes */
export interface Visit { sid: string; head: (KeptHead & { at: string }) | null; notes: ReplayNote[]; }
export function visitsOf(lines: KeptLine[], into = new Map<string, Visit>()): Map<string, Visit> {
  const of = (sid: string) => { let v = into.get(sid); if (!v) into.set(sid, v = { sid, head: null, notes: [] }); return v; };
  for (const L of lines) {
    if (L.k === 'head') { const v = of(L.sid); if (!v.head || L.at >= v.head.at) { const { k: _k, sid: _s, ...h } = L; v.head = h; } }
    else if (L.k === 'notes') { const v = of(L.sid); for (const n of L.notes) if (!v.notes.some(m => m.id === n.id && m.r === n.r && m.text === n.text)) v.notes.push(n); }
  }
  return into;
}
