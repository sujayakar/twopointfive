// THE SHORT LIST'S PLAY: remembering what this machine has started, and the row after one for the pause and end cards' 'Next', as the desk says it (view/read.ts ShortView).
// The start page shows the rows' missions first (ui/startpage.ts).
import { desk, deskStands } from '../view/desk';

const LS = 'twopointsix.picks';         // the ids started on this machine
const LS_MISSION = 'twopointsix.mission';   // (ui/missions.ts: the row its list opens on — a pick started here is that row)

/** the ids this machine has started */
function picksPlayed(): string[] { try { const v = JSON.parse(globalThis.localStorage?.getItem(LS) ?? '[]'); return Array.isArray(v) ? v.filter(x => typeof x === 'string') : []; } catch { return []; } }
/** the row `id` is being started: remembered as started on this machine, and its mission is the row the mission list opens on */
export function notePicked(id: string, mission: string) { try { const had = picksPlayed(); if (!had.includes(id)) globalThis.localStorage?.setItem(LS, JSON.stringify([...had, id])); globalThis.localStorage?.setItem(LS_MISSION, mission); } catch { /* no store */ } }
/** THE PICK AFTER THIS ONE, for the pause and debrief cards of a run a pick started (null: the page is not a pick's, or it was the last):
 *  the next row's title and the page load that starts it. A difficulty the player chose over this row's own goes on to the next (and is said in the title);
 *  otherwise the next row opens on its own. `scenario`: the row the page is starts a scenario (the solo night's starts none) */
export function nextPickOf(search: string = typeof location !== 'undefined' ? location.search ?? '' : ''): { scenario: boolean; title: string; href: string; start(): void } | null {
  if (!deskStands()) return null;
  const v = desk().next(search); if (!v.next) return null;
  return { scenario: v.nowScenario, title: v.title, href: v.link, start: () => {
    notePicked(v.next, v.nextMission);
    const L = location as Location, to = (L.pathname ?? '') + v.link + (L.hash ?? '');
    if (to === (L.pathname ?? '') + (L.search ?? '') + (L.hash ?? '')) L.reload(); else L.assign(to);
  } };
}
