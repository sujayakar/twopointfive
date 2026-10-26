// A LESSON DONE LEADS TO THE NEXT ONE: the end card of a tutorial mission that was won offers the next row of the tutorial as its first button (ui/menu.ts
// debrief) — that row's own page load, the visitor's other flags kept, as a pick of it on the mission list would be. Null on any other page, after the last
// lesson, and with no verdict (?noverdict: the card as it was). "Next" is the next lesson the lists draw (ui/missions.ts listedMissions; ?offlist=0: every row).
import { VERDICT } from '../game/verdict';
import { TUTORIAL_MISSIONS } from '../scene/scenarioList';
import { listedMissions, missionHref, missionList } from './missions';

/** `id` = the tutorial row that is up (the running scenario's own entry: what was staged, not what the query asked for) */
export function nextLessonOf(id: string | null | undefined, search: string = typeof location !== 'undefined' ? location.search ?? '' : ''): { id: string; title: string; href: string; start(): void } | null {
  if (!VERDICT.on || !id) return null;
  // (the next row THE LISTS DRAW, in the tutorial's own order: a lesson cut from the list — scenarios.ts `offList` — is nobody's next lesson, and one played by
  //  its own address leads to the listed lesson after it, as a visit that names it has it drawn that once)
  const all = missionList(), drawn = new Set(listedMissions(all, id).map(x => x.scenario?.id)), rows = TUTORIAL_MISSIONS.filter(e => drawn.has(e.id));
  const i = rows.findIndex(e => e.id === id), next = i >= 0 ? rows[i + 1] : undefined, m = next && all.find(x => x.scenario?.id === next.id);
  if (!m) return null;
  const href = missionHref(m, search);
  return { id: m.id, title: m.title, href, start: () => { const L = location as Location; L.assign((L.pathname ?? '') + href + (L.hash ?? '')); } };
}
