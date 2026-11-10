// A LESSON DONE LEADS TO THE NEXT ONE: the end card of a tutorial mission that was won offers the next row of the tutorial as its first button
// — that row's own page load, the visitor's other flags kept, as a pick of it on the mission list would be. Null on any other page and after the last
// lesson. "Next" is the next lesson the lists draw (ui/missions.ts listedMissions).
import { desk, deskStands } from '../view/desk';

/** `id` = the tutorial row that is up (the running scenario's own entry: what was staged, not what the query asked for) */
export function nextLessonOf(id: string | null | undefined, search: string = typeof location !== 'undefined' ? location.search ?? '' : ''): { id: string; title: string; href: string; start(): void } | null {
  if (!id || !deskStands()) return null;
  // (the next row THE LISTS DRAW, in the tutorial's own order: a lesson cut from the list is nobody's next lesson, and one played by its own address leads to the listed lesson after it, as a visit
  //  that names it has it drawn that once)
  const rows = desk().start(search).rows.filter(r => r.lesson), i = rows.findIndex(r => r.id === id), m = i >= 0 ? rows[i + 1] : undefined;
  if (!m) return null;
  return { id: m.id, title: m.title, href: m.link, start: () => { const L = location as Location; L.assign((L.pathname ?? '') + m.link + (L.hash ?? '')); } };
}
