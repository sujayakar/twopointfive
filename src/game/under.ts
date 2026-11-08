// A TARGET OF THE VIEW'S, AS THE PAGE'S OBJECTS: the Rust game says what a point or the cursor is on (src/view/read.ts Under: a man by his character's id, a door by its
// name, a wall by its place), and the menus and the HUD that still work with the page's objects are handed those. Nothing is worked out here.
import type { Game } from './game';
import type { OrderTarget } from './orders';
import type { Under } from '../view/read';
import { RAW } from '../math/side';

/** the target an answer was last made into: one object an answer, however many ask (a record the host keeps till the game or the cursor moves) */
type Made = Under & { made?: OrderTarget | null };
/** null: nothing to say (no Rust game stands), or a man or a door this page has not got */
export function targetOf(g: Game, u: Under | null): OrderTarget | null { if (!u) return null; const m = u as Made; return m.made !== undefined ? m.made : (m.made = make(g, u)); }
function make(g: Game, u: Under): OrderTarget | null {
  const man = (id: number) => g.guards.find(x => x.char.id === id);
  switch (u.kind) {
    case 'floor': { const w = u.wall, box = w ? g.col.boxes[w.index] : undefined, t = u.thing;
      return { kind: 'floor', at: u.at, ...(w && box ? { wall: { index: w.index, box, alongX: w.alongX, u: w.u, side: w.side as 1 | -1, ok: w.ok, why: w.why, ...(w.outer ? { outer: true } : {}) } } : {}),
        ...(t ? { thing: { kind: t.kind, face: t.face, n: t.n, stand: t.stand, name: t.name } } : {}) }; }
    case 'door': { const door = u.door === null ? null : g.doors.byName(u.door); return door ? { kind: 'door', at: u.at, door, side: u.side as 1 | -1 } : null; }
    case 'foe': { const foe = man(u.man); return foe ? { kind: 'foe', at: u.at, foe } : null; }
    case 'none': { if (u.man < 0) return { kind: 'none', at: u.at }; const m = man(u.man); return m ? { kind: 'none', at: u.at, man: m } : null; }
  }
}
/** whose the question is: the seat whose book asks, by its place in the game's list (a seat's own view of the game), or −1: the one this machine shows */
export function seatOf(g: Game): number { const v = g as unknown as { [RAW]?: unknown; seat?: { index: number } }; return v[RAW] && v.seat ? v.seat.index : -1; }
