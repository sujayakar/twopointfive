// THE LIST'S CAP, asked of what the page draws (missions-probes.ts, unplayed-check.ts, picks-probes.ts): no group draws more than eight rows.
// A row cut from the list (scene/scenarios.ts `offList`) stays a row of the table and is on no screen, so it is not counted. What the page draws is asked
// of ui/missions.ts listedMissions with its switch on, never of a row's own flag (the harness leaves OFFLIST_UI off: the list probes pin the whole table).
// TWO WAYS A NINTH CARD CAN STILL STAND, neither held by this cap: `?offlist=0` (every row of the table drawn, a word for looking at the cut rows) and a
// visit that names a cut row (`?loadout=<id>`: listedMissions' `keep` draws that one row that once).
type Row = { id: string; group: string };
type Lists<T> = { OFFLIST_UI: { on: boolean }; listedMissions: (list: T[]) => T[] };
export const GROUP_CAP = 8;
/** the rows of `list` the page draws: listedMissions with the page's switch on, put back after */
export function drawnOf<T extends Row>(M: Lists<T>, list: T[]): T[] {
  const was = M.OFFLIST_UI.on; M.OFFLIST_UI.on = true;
  try { return M.listedMissions(list); } finally { M.OFFLIST_UI.on = was; }
}
/** every group of `list` draws at least one row and no more than the cap */
export function groupsFit<T extends Row>(M: Lists<T>, list: T[], groups: string[] = [...new Set(list.map(m => m.group))]): boolean {
  const D = drawnOf(M, list);
  return groups.every(g => { const n = D.filter(m => m.group === g).length; return n >= 1 && n <= GROUP_CAP; });
}
