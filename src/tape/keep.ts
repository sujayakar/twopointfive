// EVERY RUN IS KEPT: the tape of the run being played (handtape.ts) goes to this browser's store of kept runs (tape/keepstore.ts) a piece at a time as it grows, nothing pressed, and is
// read back whole from there. A piece is bytes of the tape and where in it they go, so a run whose page was killed is kept up to its last piece.
import type { KeepSource } from './keeper';
import type { KeptHead } from './kept';
import { bodyText, type KeptStore } from './keepstore';
import { HAND } from '../draw/input';
import { HANDTAPE, type HandTape } from './handtape';

/** a visit's head, where its tape is one of hands */
export interface HandsHead extends KeptHead { game: 'twopointsix'; hands: number; /** which numbers a hand of it is of (draw/input.ts HAND) */ hand: number; seed: number }
interface Line { k?: string; sid?: string; from?: number; b?: string }

const CHUNK = 0x8000;
const packed = (b: Uint8Array): string => { let s = ''; for (let i = 0; i < b.length; i += CHUNK) s += String.fromCharCode(...b.subarray(i, i + CHUNK)); return btoa(s); };
const unpacked = (s: string): Uint8Array => Uint8Array.from(atob(s), c => c.charCodeAt(0));

/** the pieces of the tape `tape` answers (null: nobody's run is on) */
export function handSource(tape: () => HandTape | null): KeepSource {
  let sent = 0; const at0 = new Map<number, number>();   // (where each piece began)
  return {
    again: () => { sent = 0; }, missed: seq => { sent = Math.min(sent, at0.get(seq) ?? sent); },   // (nothing after a hole is of any use: from there again)
    since: () => {
      const T = tape(), b = T && T.length > sent ? T.whole(sent) : null; if (!T || !b) return null;
      const from = sent; sent += b.length;
      return { line: (sid, seq, at) => { at0.set(seq, from); return JSON.stringify({ k: 'hands', sid, seq, at, from, b: packed(b) }) + '\n'; }, seconds: Math.round(T.seconds * 10) / 10 };
    },
  };
}

/** a kept visit's tape out of its lines (a kept body's, a hand-over file's): what there is of it from its first byte up to the first that is missing (of two visits' lines, the first's); null: not its first */
export function handsOf(lines: Iterable<Line>): Uint8Array | null {
  const all0 = [...lines].filter(l => l.k === 'hands' && typeof l.from === 'number' && typeof l.b === 'string'), got = all0.filter(l => l.sid === all0[0].sid).sort((a, b) => a.from! - b.from!), out: Uint8Array[] = []; let n = 0;
  for (const l of got) { if (l.from! > n) break; const b = unpacked(l.b!); if (l.from! + b.length <= n) continue; const fresh = b.subarray(n - l.from!); out.push(fresh); n += fresh.length; }
  if (!n) return null;
  const all = new Uint8Array(n); let at = 0; for (const b of out) { all.set(b, at); at += b.length; }
  return all;
}
/** can a kept visit's tape be walked by this build, by what its head says of it: one of another version, or whose hands have other numbers, is refused */
export async function walkable(store: Pick<KeptStore, 'head'>, sid: string): Promise<boolean> {
  try { const h = JSON.parse(await store.head(sid) ?? '{}') as Partial<HandsHead>; return h.hands === HANDTAPE.version && (h.hand ?? HAND) === HAND; } catch { return false; }
}
/** a kept visit's tape, read out of the store; null: it is not kept, or has none */
export async function keptHands(store: Pick<KeptStore, 'bodies'>, sid: string): Promise<Uint8Array | null> {
  const lines: Line[] = [];
  for (const b of await store.bodies(sid)) { let text: string; try { text = await bodyText(b); } catch { continue; }   // (a body that does not unpack is a missing piece)
    for (const row of text.split('\n')) if (row) try { lines.push(JSON.parse(row) as Line); } catch { /* cut short as its page went */ } }
  return handsOf(lines);
}
