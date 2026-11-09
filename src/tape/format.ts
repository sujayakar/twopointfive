// WHAT A REPLAY'S PAGE AND THE TOOLS THAT READ ONE SHARE: a note on a kept run, the floor as a drawing, a clock's words, and the way back to the game. (The tape itself is handtape.ts, a kept
// visit's lines are kept.ts.) No import: a tool reads this alone.

/** a NOTE the maintainer put on the replay in the viewer: a moment (`r`, with the game clock and step count of the nearest sample), a spot on the floor (or none), words */
export interface ReplayNote { id: string; r: number; t: number; n: number; pos: [number, number] | null; text: string; at: string; }

/** the floor as a drawing: every solid thing between knee and head height as a turned rectangle — [x, z, half-width, half-depth, yaw°, kind] with kind 0 = a wall
 *  (floor to above head height), 1 = low (furniture, cover), 2 = glass — and the doors' hinges */
export interface ReplayMap {
  bounds: [number, number, number, number];
  boxes: number[];
  doors: { name: string; hinge: [number, number]; dir: number; width: number }[];
}

/** m:ss.s */
export function clock(sec: number): string { const s = Math.round(Math.max(0, sec) * 10) / 10; const m = Math.floor(s / 60);   // (to the tenth FIRST: 299.96 is 5:00.0, not 4:60.0)
  return `${m}:${(s - m * 60).toFixed(1).padStart(4, '0')}`; }

// ── ONE WINDOW (the maintainer, 10-15: "replays should just play in the current window, not open a new tab"): the game page leaves a mark when it goes to a
//    replay, and each replay page's '‹ Game' reads it.
/** where this window was when it left for a replay (sessionStorage: this tab's own): the address, the history SLOT it was in (the Navigation API's entry key —
 *  the count of entries cannot tell where a window stands once it has gone Back) and the shelf entry or kept visit it left for. ONE TRIP'S: a game page that is up again drops it
 *  (`gameUp`), so a replay reached later by another road finds none and goes to the
 *  game's own page. */
const CAME_FROM = 'twopointsix.replay.from';
type Slots = { currentEntry?: { key?: string } | null; entries?: () => { key: string }[]; traverseTo?: (key: string) => { committed?: Promise<unknown>; finished: Promise<unknown> } };
const slots = (): Slots | null => { try { return (globalThis as { navigation?: Slots }).navigation ?? null; } catch { return null; } };
/** a game's page is up, booted or handed back by the browser as it was left: the last trip to a replay is over */
export function gameUp(): void { const drop = (): void => { try { sessionStorage.removeItem(CAME_FROM); } catch { /* a private window */ } }; drop(); try { window.addEventListener('pageshow', e => { if (e.persisted) drop(); }); } catch { /* no page */ } }
/** this window goes to a replay page — never a new tab — leaving the mark its way back reads */
export function leaveFor(url: string, id: number | string): void {
  try { sessionStorage.setItem(CAME_FROM, JSON.stringify({ href: location.href, key: slots()?.currentEntry?.key ?? null, id })); } catch { /* a private window: Back is the browser's */ }
  location.assign(url);
}
/** the way back from a replay page on shelf entry `id` to the game it was left for: THAT entry of this window's history when it is still there (however many
 *  hops between the replay and the map lie between — the browser may hand the game page back as it was left), else the address left, else `home`. A mark left
 *  for another shelf entry, or by no game page, is not trusted. No Navigation API (an older browser): the address. */
export function backToGame(id: number | string | null, home: string): void {
  // (ONE PRESS, ONE GOING: a second inside the wait is not a second road — and the mark is READ, not taken: a press after the wait, on a page still here, goes the
  // same way again and not to `home`. The game page drops the mark when it is up again: ReplayKey's constructor, and its pageshow)
  const now = performance.now(); if (now - wentAt < BACK_MS) return; wentAt = now;
  try { window.addEventListener('pageshow', () => { wentAt = -1e9; }, { once: true }); } catch { /* */ }   // (this page handed back as it was left takes the next press at once)
  let from: { href?: string; key?: string | null; id?: number | string } | null = null;
  try { from = JSON.parse(sessionStorage.getItem(CAME_FROM) ?? 'null'); } catch { /* */ }
  if (from && (id === null || from.id !== id || typeof from.href !== 'string' || /[?&]playtape=/.test(from.href))) from = null;
  const to = from?.href ?? home, N = slots(), key = from?.key;
  let there = false; try { there = !!key && !!N?.traverseTo && !!N.entries?.().some(e => e.key === key); } catch { /* */ }
  if (!there) { location.assign(to); return; }
  // (a traverse that fails rejects BOTH its promises: each is caught — the page's own net for a rejection nobody caught is a fatal card — and the address makes it good)
  try { const T = N!.traverseTo!(key!); T.committed?.catch(() => { /* finished's catch goes by the address */ }); T.finished.catch(() => { if (document.visibilityState === 'visible') location.assign(to); }); } catch { location.assign(to); }
}
let wentAt = -1e9; const BACK_MS = 8000;
