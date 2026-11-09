// THE PAGE OF A TAPE (`?playtape=<kept visit | file>`): where the tape comes from, and the address it plays at: the run's own words, for a page reads its picture off its address.
import { HAND } from '../draw/hand';
import { PLAYTAPE, recipeSearch } from '../tape/address';
import { HandWalk, TapeRefused } from '../tape/handtape';
import { handsOf, keptHands } from '../tape/keep';
import type { ReplayNote } from '../tape/format';
import { parseLines } from '../tape/kept';
import { keptStore } from '../tape/keptui';

async function opened(b: Uint8Array): Promise<Uint8Array> {
  if (b[0] !== 0x1f || b[1] !== 0x8b) return b;
  return new Uint8Array(await new Response(new Blob([b as BlobPart]).stream().pipeThrough(new DecompressionStream('gzip'))).arrayBuffer());
}
/** the tape this address names, or null: it names none. A visit's name is a run this browser keeps; anything else is a file: the tape itself, or a kept visit's lines */
export async function tapeAsked(q: URLSearchParams): Promise<Uint8Array | null> {
  const named = q.get(PLAYTAPE); if (!named) return null;
  if (keptName(named)) { const b = await keptHands(keptStore(), named); if (!b) throw new TapeRefused('That replay isn\u2019t saved in this browser anymore.'); return b; }
  if (/^\d+$/.test(named)) throw new TapeRefused('This replay is from an older version of the game, so it can\u2019t be played here.');
  const r = await fetch(new URL(named, document.baseURI), { cache: 'no-store' }); if (!r.ok) throw new TapeRefused(`The page couldn\u2019t find a replay at ${named}.`);
  const d = await opened(new Uint8Array(await r.arrayBuffer())); if (d[0] !== 0x7b) return d;
  const b = handsOf(parseLines(new TextDecoder().decode(d)).lines); if (!b) throw new TapeRefused('That file isn\u2019t a replay of this game.');
  return b;
}
/** is this a kept visit's name (host/session.ts) */
export const keptName = (s: string): boolean => /^\d{8}T\d{6}-\w+$/.test(s);
/** the notes on a kept visit */
export async function keptNotes(sid: string): Promise<ReplayNote[]> { return (await keptStore().rows()).find(r => r.sid === sid)?.notes?.slice() ?? []; }
/** the address this tape plays at, or null: this page is at it */
export function tapeAddress(bytes: Uint8Array, q: URLSearchParams): string | null {
  const S = new HandWalk(bytes, HAND.count).start, own: Record<string, string> = { [PLAYTAPE]: q.get(PLAYTAPE) ?? '' }; { const at = q.get('at'); if (at !== null) own.at = at; }
  const to = recipeSearch({ search: S.words, seed: S.seed }, q, own);
  return new URLSearchParams(to).toString() === q.toString() ? null : to;
}
