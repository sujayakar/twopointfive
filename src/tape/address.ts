// THE ADDRESS OF A RUN BEING PLAYED AGAIN (a replay's page, a networked game's): which words of an address are the run's, and which the visit's that plays it. A leaf: no game.
import { SHADER_ROW_WORDS } from '../ui/shaderrows';

export const PLAYTAPE = 'playtape';
/** the address words a replay page keeps from the VISIT, not from the run it plays (replay/engineplay.ts tapeSearch): how the page is looked at and listened to is the
 *  viewer's — which sound table the mixer plays is the listener's, a replay holds no sound. The end card's Replay hands the game page's own on to its tab, bar the
 *  harness's `paused`: the replay of a run played at a chosen quality, or muted, opens the same way. */
export const VISIT_FLAGS = ['paused', 'mute', 'quality', 'adapt', 'hud', 'oldsounds', 'newsounds', 'soundlanes', 'fogedge', 'fogdesat', 'fogair', 'fogkept', 'nobouncewalls', 'noreflwalls', 'sharpcap', 'allproxies', 'framecap', 'pace', 'giveway', 'flight', 'look', 'nosunglint', 'glintpick', 'holedepth', 'kept', 'hireach',
  ...SHADER_ROW_WORDS, 'pipequeue', 'shadertrim', 'shaderfold', 'fell', 'smokeindirect'];   // (the second line: how THIS machine builds its shaders — ui/shaderrows.ts, the compile queue and the trim, gpu/ — a word of the visit's address read once as the page loads; without them here a replay opened with one was sent on to an address without it and built the plain way. nobouncewalls: the picture's walled bounce read off — render/frame.ts bounceWallsConfig — so a page tool can play one replay both ways)
// (…and `fell`: the count of questions the TypeScript answered while its row was up, sim.ts FELL — a page sent on to a recipe's address keeps the word. The Rust module's rows have no word: they are not switches)
/** a word of an address is the visit's: one of VISIT_FLAGS */
export const visitWord = (k: string): boolean => VISIT_FLAGS.includes(k);
/** the visit's words on an address, pair by pair in the list's order (a word can stand more than once: the look page's rows) */
export const visitPairs = (q: URLSearchParams): [string, string][] => VISIT_FLAGS.flatMap(k => q.getAll(k).map(v => [k, v] as [string, string]));
/** keys of the tape's query that are this visit's and not the run's (the tape's seed is said by its recipe) */
const NOT_THE_RUNS = ['paused', 'mute', 'quality', 'seed', 'pick', 'loadout', 'noreplay', 'hud', 'oldsounds', 'newsounds', 'onetheme', 'shuffle', 'soundlanes', PLAYTAPE, 'at', 'player', 'net', 'netgame', 'netkey'];
/** THE SENDING WORDS (cloud/cloud.ts: whether this browser sends, and what) are the viewer's, not the run's — a tape recorded against one backend does not
 *  point the page that replays it there, and a viewer's `?cloud=0` stays on the replay's address — with ONE thing kept from the tape: its OFF, where the
 *  viewer's address says nothing of that word (the map viewer's road here carries no words, and a visit that did not send must not start sending on its
 *  replay's page). `player` (whose settings a link hands over) is no word of the run either */
export const SENDING_WORDS = ['cloud', 'cloudtapes', 'cloudstats', 'cloudprefs', 'cloudfiles'];
export const SAYS_OFF = /^(0|off|false|no)$/i;
/** a recipe's own query with this visit's keys and the words that say what the page is doing with it (`own`: a tape's `playtape` and `at`, a networked game's
 *  `net` words — src/net/door.ts): what the page must be booted on to be that run's page */
export function recipeSearch(recipe: { search?: string; seed?: number | null }, now: URLSearchParams, own: Record<string, string>): string {
  const out = new URLSearchParams();
  for (const [k, val] of new URLSearchParams(recipe.search ?? '')) if (SENDING_WORDS.includes(k) ? SAYS_OFF.test(val) : !NOT_THE_RUNS.includes(k) && !visitWord(k)) out.append(k, val);   // (a visit's word is never the run's, whichever list a goal put it on: a tape recorded under one does not put it on the page that replays it)
  out.set('seed', recipe.seed === null || recipe.seed === undefined ? 'off' : String(recipe.seed));
  out.set('noreplay', '');   // (no tape of a tape: the page's own buffer stays off)
  for (const [k, v] of visitPairs(now)) out.append(k, v);   // (pair by pair)
  for (const k of SENDING_WORDS) if (now.has(k)) out.set(k, now.get(k) ?? '');
  for (const [k, val] of Object.entries(own)) out.set(k, val);
  return '?' + out.toString();
}
