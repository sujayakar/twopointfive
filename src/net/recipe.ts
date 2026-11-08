// THE RECIPE: how a networked game is told of where the game that steps is stood through the TypeScript game (a page with one, tools/mp/stand.ts): the die, the
// page's address, every knob table, the kit, as a tape carries them (replay/format.ts RunTape.recipe). It goes with that page (start.ts is what a game in
// the module is started from).
import type { PlayerCmd } from '../game/cmd';
import type { RunTape } from '../replay/format';
import type { WireCmd } from './wire';

export type Recipe = RunTape['recipe'];
/** the knob tables a networked game stands on, whatever its maker says (the game's own maths, the same bits on every engine, are no table: every game
 *  plays on them): every seat's record checked (the view worked out again from its pose: fromWire sends none), and the run's verdict kept (how a server knows the
 *  mission is over). Whoever makes a recipe forces them; whoever stands one refuses a recipe without them (`recipeWhy`) */
export const NET_TABLES: Record<string, Record<string, unknown>> = { CMDCHECK: { on: true }, VERDICT: { on: true } };
/** why a networked game cannot be stood from `r` ('' = it can) */
export function recipeWhy(r: Recipe): string {
  if (!r || typeof r !== 'object' || !r.tables) return 'the recipe carries no knob tables';
  const T = { ...((r.knobs as { tables?: object } | null)?.tables ?? {}), ...r.tables } as Record<string, Record<string, unknown> | undefined>;   // (a recipe keeps its tables in two places: the journal's own and every other, game/runtape.ts recipeOf)
  if (r.knobs !== null && typeof r.knobs !== 'object') return 'the recipe\'s knobs are not a record';
  if (!r.scenario) return 'the recipe has no mission up';
  for (const [name, want] of Object.entries(NET_TABLES)) for (const [k, v] of Object.entries(want)) if (T[name]?.[k] !== v) return `the recipe has ${name}.${k} at ${JSON.stringify(T[name]?.[k])}: a networked game needs ${JSON.stringify(v)}`;
  return '';
}

export function toWire(c: PlayerCmd): WireCmd {
  const V = c.view, out: WireCmd = { k: c.keys.slice(), b: c.buttons, m: [c.mx, c.my], s: [c.w, c.h, c.pw, c.ph], v: [V.target[0], V.target[1], V.target[2], V.yaw, V.pitch, V.distance, V.fov, V.aspect, V.aimLead] };
  if (c.pressed.length) out.p = c.pressed.slice(); if (c.clicked) out.c = c.clicked; if (c.wheel) out.w = c.wheel;
  return out;
}
/** the record a tick runs on, from the wire's: the matrices and the eye are left for the game to work out (NaN: cmd.ts checkCmd builds them for a remote seat,
 *  and for any seat whose matrices are not numbers) */
export function fromWire(w: WireCmd): PlayerCmd {
  const v = w.v;
  return { keys: w.k.slice(), pressed: w.p ? w.p.slice() : [], buttons: w.b, clicked: w.c ?? 0, wheel: w.w ?? 0, mx: w.m[0], my: w.m[1], w: w.s[0], h: w.s[1], pw: w.s[2], ph: w.s[3],
    view: { vp: new Array<number>(16).fill(NaN), ivp: new Array<number>(16).fill(NaN), pos: [0, 0, 0], forward: [0, 0, 0], target: [v[0], v[1], v[2]], yaw: v[3], pitch: v[4], distance: v[5], fov: v[6], aspect: v[7], aimLead: v[8] } };
}
