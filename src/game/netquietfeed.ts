// THE NET GOES QUIET, the squads' half of its feeds (netquiet.ts has the rule and the switch). enemysquads.ts sits early in src/game's one import cycle: a
// runtime import of netquiet.ts from it pulls morale.ts — and what that pulls — forward of where the cycle has always evaluated them (measured: the headless
// boot throws in wind.ts). So that file's two seams file what happened HERE, a leaf with type-only imports, and netquiet.ts's tick reads it the same frame
// (the squads tick inside the room fight's, before it); (what that file asks back, is this man the last at his act, is the record's: src/game/owner.ts).
// `live` is netquiet.ts's to write (its tick: NETQUIET.on && MORALE.on); with it off nothing is filed.
import { SideMap } from '../math/side';
import type { Game, Guard } from './game';

/** `live` = netquiet.ts's two switches are both on, as of its last tick (not a switch of its own: nothing sets it but that tick) */
export const NETFEED = { live: false };
/** a man asked for by name who did not answer · a body found (and by whom); `men` = the net it was said on */
export interface NetFeed { what: 'asked' | 'body'; by: Guard | null; who: Guard | null; men: readonly Guard[] }
const filed = new SideMap<Game, NetFeed[]>('netquietfeed.filed');
/** enemysquads.ts radioCheck: `asker` asks for `who` by name — a return on its first line with the switch off */
export function netFeedAsked(g: Game, asker: Guard | null, who: Guard, men: readonly Guard[]) {
  if (!NETFEED.live) return;
  let F = filed.get(g); if (!F) filed.set(g, F = []); F.push({ what: 'asked', by: asker, who, men });
}
/** enemysquads.ts event 'found': `finder` has a man of the squad down — a return on its first line with the switch off */
export function netFeedBody(g: Game, finder: Guard | null, men: readonly Guard[]) {
  if (!NETFEED.live) return;
  let F = filed.get(g); if (!F) filed.set(g, F = []); F.push({ what: 'body', by: finder, who: null, men });
}
/** netquiet.ts's tick: what was filed since it last asked (and the book is empty again) */
export function netFeedTake(g: Game): NetFeed[] { const F = filed.get(g); if (!F || !F.length) return []; filed.set(g, []); return F; }
