// A TEAMMATE'S 40 mm GRENADE, FIRED ON ORDER — "40 mm there".
//
// The maintainer, on the gear screen (10-19): "should be symmetric between self and teammates (e.g. 40mm should work for squad)". Until here the launcher's
// high-explosive round was Sam's alone (he40.ts: "nobody else is handed one"). Here a teammate's card may carry rounds (kit.ts MemberKit.shells, staged by
// kitStage) and the squad has one counted order on a spot of FLOOR, on the throws' own page of the floor's menu (squadthrow.ts):
//
//   X on a spot of floor · G ('throw ▸') · 3 = 40 mm there.      Nothing held, no digit above 3 on the page.
//
// THE ROUND is he40.ts's and nothing else: fireShell off the teammate's own launcher, the same 76 m/s, the same nose fuze that arms HE40.armM out, the same
// burst (frag.ts blast) — on whoever stands in it, his own side and the player too. Nothing here judges anybody or draws a die.
// THE COUNT is each man's own: what his card carries rides in his pouches and goes down his tube, so a man with none fires none whoever else has one, and a
// man down takes his with him. The row and the squad block show what the men on their feet have between them.
// WHO FIRES: of the men the menu goes to, one with a round, a long gun in his hands that has the launcher under it (weapons.ts `launcher`), a hand free
// (squadsmoke.ts squadHandFree, squadthrow.ts handsAtWork), the tube closed again after his last (LAUNCHER40.cd), and A SHOT: the arc he40.ts would fly from
// his tube to the spot (shellSolve — statics, leaves, furniture, the bodies a round stops on: the player, the unarmed, the other side — and, to the plan, a
// teammate: nobody fires one through a friend's back) goes off within
// `nearM` of the spot and is ARMED there, a margin over (`armSlack`). A man of the other side standing in the line is where it goes off instead — if it is
// armed there. A man whose gun is on the spot already is asked first, then the nearest.
// HE POINTS IT FIRST. A man whose gun is UP and on the spot (within `aimRad`: in a fight, or under an order that has it at his shoulder) fires at once and
// keeps whatever he was doing — the fight included. Another is turned
// to it: lent from his order as a walking thrower is (squadthrow.ts squadLend — only a man who may be: no drill, door tool or fight has him), gun up on the
// spot, and the round goes when his gun is on it and the floor still says yes; then he is handed back (squadHandBack). NOBODY WALKS for it and nobody runs.
// SENSE, asked when the menu is read and again as the round goes (nothing is spent on a refusal): none left · none with the men addressed · nobody of them
// ready (no launcher in a free hand, the tube still open, not ours to turn) · too close to arm · no shot for anybody addressed · NEVER ONTO ITS OWN: the
// player, a teammate still breathing (the man firing too), a hostage, the man to take alive or a civilian inside the burst's reach (FRAG.woundM and
// `reachSlack`, by squadthrow.ts squadFragCatches with nobody counted on to duck: it goes off as it lands) of where it goes off or of the spot = refused,
// naming who. A man whose round would have one of ours in it is passed over for the next man with a shot.
// A SILENT squad: the row is one of silent.ts LOUD_ROWS (marked, and giving it takes the squad out of silent) and of holdfire.ts LIFT_ORDERS.
//
// SQUAD40.on: off in the module and on the frozen stand; on in the page (host/profile.ts; `?nosquad40`). The row is on the menu only while the switch is on,
// the throws' page is there (squadthrow.ts: no page, no row) AND this staging's cards brought a round: with no card carrying one nothing is kept for the Game, nothing is registered, nothing runs but one map read a
// frame, and a run digests as it did before this file — on either stand. `SQUAD40_BREAK=1` (the check's): the switch reads off whatever it is set to.
//
// State is per Game in a side table: each man's rounds, his last shot, who is turning to one, what the squad block shows. None of it is in a checkpoint —
// none is taken with a unit on the floor. Check: tools/qa/squad40-check.ts (`probes.ts squad40`).
// (No top-level statement here uses another game module's export: src/game is one import cycle.)
import { SideMap } from '../math/side';
import type { Vec3 } from '../math/vec';

import type { Game, Guard } from './game';
import type { ThrowSolution } from './throwables';

import { type Lent } from './squadthrow';

import { RUST_PASS, RUST_SURFACE } from './crossing';

export const SQUAD40 = {
  on: false,
  /** a shot is one whose round goes off within this of the spot (m) … */
  nearM: 1.2,
  /** … having flown this much more than it arms at (m: HE40.armM is counted along its path, and he is not a range table) */
  armSlack: 0.5,
  /** nobody of ours inside the burst's own outer radius and this (m: FRAG.woundM — it goes off where it lands, so there is no run to allow for as a thrown frag's) */
  reachSlack: 0.5,
  /** the tube: this far ahead of him toward the spot (m), this high (m) — or `headDrop` under his head, if that is lower (a man down on a knee) */
  muzzleAhead: 0.55, muzzleUp: 1.35, headDrop: 0.3,
  /** his gun is ON the spot within this (rad): such a man fires at once; another is turned to it first … */
  aimRad: 0.2,
  /** … for this long at most (s), his gun up at least this long before the round goes (s), and stays on it this long after it has gone (s) */
  aimSecs: 2, raiseSecs: 0.35, afterSecs: 0.4,
  /** the squad block names the shot this long (s) */
  shownSecs: 2.5,
};
/** `SQUAD40_BREAK=1` (the check's): the order is not there — no row, no round — whatever the switch says */
const BREAK = (() => { try { return String((globalThis as { process?: { env?: Record<string, string | undefined> } }).process?.env?.SQUAD40_BREAK ?? '') === '1'; } catch { return false; } })();
/** A TEAMMATE'S 40 mm WORKS IN THIS BUILD (the card's question — kit.ts, as wallHere is the wall charge's): the switch is on and the order is not broken */
export function squad40Here(): boolean { return SQUAD40.on && !BREAK; }

export const SHELL_THERE = 'shellThere';
/** one man's shot: the tube, the point sent at, the arc, where it goes off (`burst`: the spot, or the man of the other side in its line) */
interface Shot { gd: Guard; from: Vec3; to: Vec3; sol: ThrowSolution; burst: Vec3; aimed: boolean }
/** a man turning to his shot: lent from his order (squadthrow.ts), the spot, since when, when the round went (−1: not yet), the arc as planned (the picture's) */
interface Turn extends Lent { at: Vec3; t0: number; firedT: number; shot: Shot }
interface State { rounds: Map<Guard, number>; brought: number; last: Map<Guard, number>; turns: Map<Guard, Turn>; noted: unknown;
  shown: { t: number; text: string } | null; kept: Map<Asker, { key: string; t: number; a: Shot | string; who: string | null }>; }
/** who re-reads a plan inside a third of a second, each with an answer of his own: the row's two questions (the picture keeps none — its reads are a drawn
 *  frame's, and nothing a drawn frame does may be in the Game's state) */
type Asker = 'only' | 'valid';
const states = new SideMap<Game, State>('squad40.states');
export const SQUAD40_STATS = { staged: 0, fired: 0, turned: 0, refused: 0, gaveUp: 0 };
/** every round fired on the order, newest last (QA; capped) */
export const SQUAD40_LOG: { t: number; who: string; from: Vec3; at: Vec3; burst: Vec3; left: number; turned: boolean }[] = [];

/** the launcher is under the gun in his hands: a long gun whose row has one (not his sidearm, not a shotgun — the big slot's or his own) */
const tubeUp = (gd: Guard): boolean => { const d = gd.weapon.def; return d.cls !== 'pistol' && !d.pellets && d.launcher !== false; };
/** the rounds the squad's men on their feet have between them (0: none brought, or all fired) · what this staging brought */
export function squad40Left(g: Game): number { const s = states.get(g); let n = 0; if (s) for (const [gd, k] of s.rounds) if (gd.state !== 'dead') n += k; return n; }
export function squad40Brought(g: Game): number { return states.get(g)?.brought ?? 0; }
/** one man's rounds left */
export function squad40Of(g: Game, gd: Guard): number { return states.get(g)?.rounds.get(gd) ?? 0; }

/** A SCENARIO HAS JUST STAGED ITS TEAM (kit.ts kitStage, every staging): each man's rounds for this run are what his card carries — none for a man whose
 *  gun has no launcher under it. With nobody carrying one, and never one on this Game: nothing kept. */
export function squad40Stage(g: Game, carried: readonly (readonly [Guard, number])[]) { { const __s = RUST_SURFACE; if (__s) { const __r = __s.call('squad40Stage', undefined, [g, carried]); if (__r !== RUST_PASS) return __r as any; } }
  const rounds = new Map<Guard, number>(); let n = 0;
  for (const [gd, c] of carried) { const k = tubeUp(gd) ? Math.max(0, Math.floor(c) || 0) : 0; if (k) { rounds.set(gd, k); n += k; } }
  let s = states.get(g);
  if (!s) { if (!n) return; states.set(g, s = { rounds, brought: 0, last: new Map(), turns: new Map(), noted: null, shown: null, kept: new Map() }); }
  s.rounds = rounds; s.brought = n; s.last = new Map(); s.turns = new Map(); s.shown = null; s.kept.clear(); SQUAD40_STATS.staged++;
}

export function squad40Summary(g: Game): string {
  const s = states.get(g); if (!squad40Here()) return 'off';
  return s && s.brought ? `${squad40Left(g)} of ${s.brought} left · X on a spot of floor, G, then 3 · fired ${SQUAD40_STATS.fired}` : 'none brought (no teammate\'s card carries a 40 mm grenade)';
}
