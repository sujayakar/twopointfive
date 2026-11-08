// THE PAGE OF A TAPE (`?playtape=<shelf id | file>`): where the tape comes from, and the address it plays at: the run's own words, for a page reads its picture off its address.
import { HAND } from '../draw/hand';
import { PLAYTAPE, recipeSearch } from '../replay/address';
import { HandWalk, TapeRefused } from '../replay/handtape';
import { shelfGet } from '../replay/shelf';

async function opened(b: Uint8Array): Promise<Uint8Array> {
  if (b[0] !== 0x1f || b[1] !== 0x8b) return b;
  return new Uint8Array(await new Response(new Blob([b as BlobPart]).stream().pipeThrough(new DecompressionStream('gzip'))).arrayBuffer());
}
const unpacked = (text: string): Uint8Array => Uint8Array.from(atob(text), c => c.charCodeAt(0));
const inFile = (f: { hands?: unknown }): Promise<Uint8Array> => { if (typeof f.hands !== 'string') throw new TapeRefused('This replay is from an older version of the game, so it can\u2019t be played here.'); return opened(unpacked(f.hands)); };
/** the tape this address names, or null: it names none. A replay file carries its tape as `hands` (gzip, base64); any other file is the tape itself */
export async function tapeAsked(q: URLSearchParams): Promise<Uint8Array | null> {
  const named = q.get(PLAYTAPE); if (!named) return null;
  if (/^\d+$/.test(named)) { const f = await shelfGet(Number(named)); if (!f) throw new TapeRefused('That replay isn\u2019t saved in this browser anymore.'); return inFile(f as { hands?: unknown }); }
  const r = await fetch(new URL(named, document.baseURI), { cache: 'no-store' }); if (!r.ok) throw new TapeRefused(`The page couldn\u2019t find a replay at ${named}.`);
  const d = await opened(new Uint8Array(await r.arrayBuffer()));
  return d[0] === 0x7b ? inFile(JSON.parse(new TextDecoder().decode(d)) as { hands?: unknown }) : d;
}
/** the address this tape plays at, or null: this page is at it */
export function tapeAddress(bytes: Uint8Array, q: URLSearchParams): string | null {
  const S = new HandWalk(bytes, HAND.count).start, own: Record<string, string> = { [PLAYTAPE]: q.get(PLAYTAPE) ?? '' }; { const at = q.get('at'); if (at !== null) own.at = at; }
  const to = recipeSearch({ search: S.words, seed: S.seed }, q, own);
  return new URLSearchParams(to).toString() === q.toString() ? null : to;
}
