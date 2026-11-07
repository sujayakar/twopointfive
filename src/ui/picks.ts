// THE SHORT LIST'S PLAY (scene/picks.ts PICKS is the table): remembering what this machine has started, the page load that starts a row, its title, and the row
// after one for the pause and end cards' 'Next'. The start page shows the rows' missions first (ui/startpage.ts). `?nopicks` (PICKS_UI.on false): no 'next'.
import { PICKS_UI, pickHref, pickEntry, pickOfQuery, pickDifficulty, pickOtherDifficulty, pickChosenDifficulty, nextPick, type Pick } from '../scene/picks';
import { leversHref } from '../game/difficulty';
import { quietFogAside } from '../game/quietfog';

const LS = 'twopointsix.picks';         // the ids started on this machine
const LS_MISSION = 'twopointsix.mission';   // (ui/missions.ts: the row its list opens on — a pick started here is that row)

/** the ids this machine has started */
function picksPlayed(): string[] { try { const v = JSON.parse(globalThis.localStorage?.getItem(LS) ?? '[]'); return Array.isArray(v) ? v.filter(x => typeof x === 'string') : []; } catch { return []; } }
/** what the card calls a pick: its own title, else its mission's */
export function pickTitle(p: Pick): string { return p.title ?? pickEntry(p)?.title ?? 'Solo night'; }
/** `p` is being started: remembered as started on this machine, and its mission is the row the mission list opens on */
export function notePicked(p: Pick) { try { const had = picksPlayed(); if (!had.includes(p.id)) globalThis.localStorage?.setItem(LS, JSON.stringify([...had, p.id])); globalThis.localStorage?.setItem(LS_MISSION, p.mission); } catch { /* no store */ } }
/** start `p`: remembered as started, then the page load of its URL (the same URL = a reload) */
export function startPick(p: Pick, href: string) {
  notePicked(p);
  href = quietFogAside(() => leversHref(href));   // (a difficulty lever moved by hand rides on the link only where this machine could not keep it — game/difficulty.ts; none moved = the link as it is)
  const L = location as Location, to = (L.pathname ?? '') + href + (L.hash ?? '');
  if (to === (L.pathname ?? '') + (L.search ?? '') + (L.hash ?? '')) L.reload(); else L.assign(to);
}
/** THE PICK AFTER THIS ONE, for the pause and debrief cards of a run a pick started (null: the switch is off, the page is not a pick's, or it was the last):
 *  the next row's title and the page load that starts it. A difficulty the player chose over this row's own goes on to the next (and is said in the title);
 *  otherwise the next row opens on its own */
export function nextPickOf(search: string = typeof location !== 'undefined' ? location.search ?? '' : ''): { from: Pick; pick: Pick; title: string; href: string; start(): void } | null {
  if (!PICKS_UI.on) return null;
  const now = pickOfQuery(search), next = now && nextPick(now);
  const chosen = next && pickOtherDifficulty(next) ? pickChosenDifficulty(search) : undefined, carried = chosen && next && chosen !== pickDifficulty(next) ? chosen : undefined;   // (a row with one difficulty opens on it)
  const href = next ? pickHref(next, search, undefined, undefined, carried) : null;
  return now && next && href !== null ? { from: now, pick: next, title: pickTitle(next) + (carried ? ` · ${carried}` : ''), href, start: () => startPick(next, href) } : null;
}
