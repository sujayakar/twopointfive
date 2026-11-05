// GLASS PENETRATION — what a pane is to a ROUND, and to the question 'have I a shot from here' (rebuild/glass/penetration.md is the record; the maintainer, 10-08:
// "we should support bullets penetrating"). Until this a round that met a pane stopped in it (glass.ts: 'laminated: rounds stop, glass stays') and the line of
// FIRE (collision.ts shotBlocked) treated intact glass as a wall — a man who SAW his enemy through a window had no shot. Behind ONE switch, `GLASSPEN.on`
// (shipped ON; the harness stands at OFF like the other post-digest defaults — src/host/standup.ts; `?glasspen=0|1`, env GLASSPEN=on|off, Tab ▸ Renderer fx ▸
// Glass): off — or a showcase running (Game.touring: their choreography was laid with glass stopping rounds) — = every entry point answers on its first line
// and the game is the base's, digest for digest.
//
//   THE RULE (one for Sam, the squad and the defenders — fireWeapon asks nobody whose round it is):
//   • a round carries ENERGY, 1 = a rifle round at the muzzle (GLASSPEN.energy by weapon row, else by class): 5.56 1.0 · 5.45 carbine 0.9 · 5.7 0.6 · 9 mm 0.5;
//   • a SHEET of glass (glass.ts isSheet: ≤ 10 cm through — the cooler bottle is solid glass and stops everything, as ever) takes
//       toll = cost[type] × (thickness ∕ 12 mm) × min(obliqueMax, 1 ∕ cos θ) × (holeK if the round goes through a hole a round already made)
//     of it — type from the fracture model's own typing (GlassBreak.typeOf: annealed 0.10, tempered 0.16, laminated 0.42), θ off the pane's normal;
//   • what is left < minEnergy, or θ past the graze angle (grazeCos), or the (maxPanes + 1)th pane: the round is SPENT IN THE PANE — exactly the old stop
//     (the star, the crack heard, nothing beyond). So a 9 mm goes through three plain panes and stops in the fourth, and dies in ONE laminated sheet; a rifle
//     round goes through one laminated sheet and (just) a second;
//   • otherwise it goes ON from the far face, turned a little: toward the pane's normal by bendK × A × sin θ (rounds through raked glass break toward the
//     perpendicular) plus a scatter inside a cone of half-angle A = deflectDeg × (toll ∕ energy in) — 0.5° for a rifle through a plain pane, 1° for a 9 mm,
//     2° for a rifle through laminated — its bearing and size HASHED off where it hit (glass.ts hitSeed: the same shot bends the same way; never Math.random);
//   • every pane crossed gets the fracture part's impact call ONCE, at the entry point with the incoming direction (paneImpact below — today glass.ts glassHit:
//     the star, the crack, the 'glass' hearing event, GlassBreak.hit) and GLASSPEN_HOOKS.crossed the crossing whole (entry, EXIT point, direction out, energy
//     taken) for whoever throws the spall off the far face;
//   • a crossing within holeR of an earlier one on that pane goes through the HOLE: a quarter of the toll, a quarter of the turn (the second round through the
//     same spot flies true — the doctrine: the first round is for the glass). The list is per pane, ≤ `holes`, gone with the encounter (resetGlassPen).
//   The holes ride in checkpoints (savestate.ts `glassPen`, written only when there are any: a file saved with the switch off is the base's) and the switch rides
//   in session journals (journal.ts knobTables) — a load, and a replay, pay what the live game paid.
//   • A LOAD OF SHOT (shotgun.ts: a shell is nine rounds through fireWeapon) MARKS A PANE ONCE PER PLACE — the one rule for a pellet that goes through and for
//     one that stops in the sheet (rebuild/gear-and-glass/pellets.md): a pellet that meets the glass within shellR of a mark ITS OWN SHELL has already made on that
//     pane makes no mark of its own — no star, no crack heard, no event, no hole remembered, no exit jet. What it leaves in the glass is by what it did: one
//     that WENT THROUGH took its energy on with it, and only if a piece of the broken glass still stood where it passed does that piece go (GlassBreak.widen:
//     the hole is ragged, and it is ONE hole); one that STOPPED there left its energy in the sheet, on the mark it landed on (the fracture model's tally —
//     GlassBreak.impact without glass.ts's star and noise: toughened glass still dices at its break energy, a web's star grows heavier, no new star is cut).
//     A pellet anywhere else on the pane is its own mark, exactly as a round is. A pellet that goes through joins only a mark that went through (it never
//     passes 'by the hole' of a pellet that stopped); one that stops joins either kind. So a muzzle pressed to the glass files ONE crossing at the arm point,
//     not nine; a shell from a metre is a few marks in one ragged hole; from across a room it is nine pocks a hand apart, and those are nine blows. Each pellet
//     still pays the pane's toll by its own line and flies on (or not) as its own round. shellR 0 = every pellet its own mark (NOT the old behaviour — that
//     is the glass parts off: every third pellet stars a pane nothing passes).
//   A round that is through is a round: one-shot both ways as ever (no hit points anywhere — what the pane took decides only whether it gets through the NEXT
//   one, and how hard it shoves the furniture it meets: propImpulse × what is left).
//
//   ONE MODEL: StaticCollision.shotBlocked (guards.ts clearShot — the alert branch's `blocked`, the room fight's every round, the standoff; tactics.ts losOpen;
//   the squads' / pressure's / the rescue crew's stations; breach.ts) asks THE SAME TOLL through StaticCollision.glassPen with the energy of the gun in that
//   man's hands (roundEnergyOf; a caller with no man in hand gets defaultEnergy = the sidearm's: a station is never counted on for a line a pistol would not
//   have) — so a man fires through a pane exactly when his round would get through it, and one whose 9 mm dies in the manager's laminated sidelight still goes
//   round by the door. Eyes always passed glass (segmentBlocked); feet and sound still do not (solidBlocked).
import { manOf } from './seats';
import { GameSideMap } from '../math/side';
import { Vec3, v3, clamp } from '../math/vec';
import { Box } from '../scene/boxes';
import { lineBox } from '../scene/collision';
import { isSheet, glassImpact, glassPanes, GLASSFX } from './glass';
import { f64, sim as simX } from '../sim/sim';
import { glassBreakLive } from './glassbreak';
import { isHostile, teamOfChar } from './teams';
import type { Team } from './teams';
import type { Game } from './game';
import type { Character } from './character';
import type { WeaponDef } from './weapons';

export type PaneType = 'annealed' | 'tempered' | 'laminated';
/** the numbers (Tab ▸ Renderer fx ▸ Glass has the switch; the rest are constants until somebody has played it) */
export const GLASSPEN = {
  on: true,
  /** a round's energy at the muzzle by weapon row (1 = a 5.56 rifle round), and by class for a row not listed */
  energy: { fn2000: 1.0, mk18bare: 1.0, g19bare: 0.6, ak74m: 1.0, akms: 1.1, pkm: 1.3, aks74u: 0.9, fiveseven: 0.6, guard9mm: 0.5,
    /** ONE PELLET of 00 buck (weapons.ts m870, nine a shell): a 3.5 g soft lead ball at about 370 m/s — half a 9 mm round's energy on half its weight behind
     *  the same width, and round where the 9 mm is jacketed: it gets through ONE sheet of plain or toughened glass met near square-on (0.20 / 0.14 left of it:
     *  still a pellet beyond — past 56° off a plain pane's normal, 27° off a toughened one's, it is spent in the sheet), never a second standing one, and
     *  never a laminated sheet. The maintainer's call (rebuild/gear-and-glass/pellets.md): 0.22‥0.27 = plain glass only, 0.35 = two plain panes */
    m870: 0.3, m1014: 0.3,   // (the same shell out of the gas gun)
    mp7: 0.6 /* (4.6 mm: a small fast round — the 5.7's) */ } as Record<string, number>,
  byClass: { rifle: 1.0, carbine: 0.9, pistol: 0.5 } as Record<string, number>,
  /** the energy a line-of-fire question assumes when it has no man's gun to ask (a station picked for 'whoever stands here'): the sidearm's */
  defaultEnergy: 0.5,
  /** energy a sheet takes from a round crossing it square-on, per refThick of glass */
  cost: { annealed: 0.10, tempered: 0.16, laminated: 0.42 } as Record<PaneType, number>,
  refThick: 0.012,
  /** the slant's cap on 1 ∕ cos θ, and the cosine under which a round skids along the sheet and is spent in it (≈ 80° off the normal) */
  obliqueMax: 3, grazeCos: 0.17,
  /** under this much left on the far side the round is spent in the pane; panes one round can cross */
  minEnergy: 0.12, maxPanes: 4,
  /** the scatter cone's half-angle in degrees at toll = all the round had (× toll ∕ energy in), and the share of it the round breaks toward the normal on a slant */
  deflectDeg: 5, bendK: 0.5,
  /** a crossing within holeR (m) of an earlier one on the pane goes through the hole: × holeK on the toll and on the turn; crossings remembered per pane */
  holeR: 0.04, holeK: 0.25, holes: 24,
  /** A LOAD OF SHOT: a pellet meeting a pane within shellR (m) of a mark its own shell already made there adds no mark of its own (the shell rule, above);
   *  0 = every pellet its own impact, hole and jet (the probes' red path — not the old behaviour, which lives behind the glass parts' own switches) */
  shellR: 0.04,
  /** GLASS IS NOT COVER (shotThroughGlass below; orders.ts stackPosts): a place by a door is no place to stack when at least this many of the floor lattice's
   *  points beyond that door have a line of FIRE to a man stood there that crosses a pane, for a round of stackEnergy (a rifle's: the worst gun he may meet); 0 = the rule off */
  stackEyes: 3, stackEnergy: 1.0,
  /** … and a door with such lines onto BOTH jambs (a sidelight a rifle reaches either through): the whole stack on the hand fewer of them reach, in file, the
   *  hand with the pane left empty (orders.ts stackLeastGlass) — where the places so dealt are in at most `stackLeastK` of the lines the walls' own marks
   *  are in (a glazed partition running both ways off a door has no better hand: left as it was). false = the walls' own marks at every such door, a man on
   *  each jamb — as it was */
  stackLeast: true, stackLeastK: 0.75,
  /** our energy → the fracture model's impact unit (its 1 = one pistol round square on: rebuild/glass/fracture.md) */
  fractureK: 2,
  /** DEFENDERS SHOOT THROUGH GLASS: off = a man of a side hostile to Sam judges his line of FIRE as on the base — a standing pane is a wall to it, he comes round
   *  by the door instead of firing through the window (shotEnergy below hands his question no energy). Only the question: a round that does meet glass, his
   *  included, still crosses it by the toll, and Sam's side still shoots through. Tab ▸ Renderer fx ▸ Glass, `?glassfoes=0`, kept with the switch. */
  foesThrough: true,
};
/** whoever wants every crossing (the shard part's spall off the far face, a probe): called once per pane a round goes THROUGH, after the impact call */
export const GLASSPEN_HOOKS: { crossed: ((g: Game, x: PaneCross, who: number) => void) | null } = { crossed: null };
export const GLASSPEN_STATS = { crossed: 0, spent: 0, viaHole: 0, viaArm: 0, shotAsked: 0, shotThrough: 0, shotRefused: 0, /** pellets that met a pane on a mark their own shell had made (they are in crossed / spent too) */ shellJoined: 0 };

// the switch from outside the panel: ?glasspen / ?glasspen=0 in the page, GLASSPEN=on|off under bun (the scripted-run tools) — either marks it FORCED, which
// the harness's stand-up respects (src/host/standup.ts puts an unforced switch at off: every frozen digest was recorded with rounds stopping in glass)
let forced = false;
const OFF_WORD = /^(0|off|false|no)$/i;
let forcedFoes = false;
try { const q = new URLSearchParams(location.search); if (q.has('glasspen')) { GLASSPEN.on = !OFF_WORD.test(q.get('glasspen') ?? ''); forced = true; } if (q.has('glassfoes')) { GLASSPEN.foesThrough = !OFF_WORD.test(q.get('glassfoes') ?? ''); forcedFoes = true; } } catch { /* no page */ }
try { const e = (globalThis as { process?: { env?: Record<string, string | undefined> } }).process?.env?.GLASSPEN; if (e) { GLASSPEN.on = !OFF_WORD.test(e); forced = true; } } catch { /* */ }
const LS_KEY = 'twopointsix.glasspen';
/** Game constructor: the switch kept per machine (the panel's row saves it); a URL / env word wins for the visit */
export function loadGlassPen() { try { const raw = localStorage.getItem(LS_KEY); if (!raw) return; const o = JSON.parse(raw); if (!forced && typeof o.on === 'boolean') GLASSPEN.on = o.on; if (!forcedFoes && typeof o.foesThrough === 'boolean') GLASSPEN.foesThrough = o.foesThrough; } catch { /* no storage / bad JSON: the default */ } }
export function saveGlassPen() { try { localStorage.setItem(LS_KEY, JSON.stringify({ on: GLASSPEN.on, foesThrough: GLASSPEN.foesThrough })); } catch { /* */ } }
/** src/host/standup.ts, the harness's stand-up: off unless the run asked for it by name */
export function glassPenHarness() { if (!forced) GLASSPEN.on = false; }

/** the one gate: the switch, and no showcase running (the tour / the breach were choreographed with rounds stopping in the glazing) */
export function glassPenLive(g: Game): boolean { return GLASSFX.on && GLASSPEN.on && !g.touring; }   // (GLASSFX: glass.ts's one switch over fracture, rounds and shards)

/** a round's energy as it leaves this row's muzzle */
export function roundEnergy(def: WeaponDef): number { return GLASSPEN.energy[def.id] ?? GLASSPEN.byClass[def.cls] ?? GLASSPEN.defaultEnergy; }
/** …and of the gun in this man's hands (Sam's gun in hand, else his pistol; a guard's / teammate's weapon; anybody else: the default) */
export function roundEnergyOf(g: Game, c: Character): number {
  const pl = manOf(g, c); if (pl) return roundEnergy((pl.gunInHand() ?? pl.pistol).def);
  for (const gd of g.guards) if (gd.char === c) return roundEnergy(gd.weapon.def);
  return GLASSPEN.defaultEnergy;
}

/** The energy a line-of-fire question about a SIDE is asked with when no man is to hand (a corner to hold, a spot to stand off at): undefined = the model's
 *  cautious default, as ever; 0 = no standing pane is crossed, for a side hostile to Sam while `foesThrough` is off (shotEnergy's rule, for the places picked
 *  ahead of the shot). */
export function sideShotEnergy(g: Game, team: Team): number | undefined {
  return glassPenLive(g) && !GLASSPEN.foesThrough && isHostile(team, 'player') ? 0 : undefined;
}

/** the energy a line-of-fire question about THIS man's gun is asked with (undefined while the switch is off: nothing is looked up) */
export function shotEnergy(g: Game, c: Character, def?: WeaponDef): number | undefined {
  if (!glassPenLive(g)) return undefined;
  if (!GLASSPEN.foesThrough && isHostile(teamOfChar(g, c), 'player')) return 0;   // (the row off: nothing to give a pane, so every standing pane on his line refuses it — the base's answer for him; a piece already out of the glass is still a hole)
  return def ? roundEnergy(def) : roundEnergyOf(g, c);
}

/** the pane's type — the FRACTURE model's own word for it (glassbreak.ts typeOf), so what a round pays and how the pane fails are one typing */
export function paneType(g: Game, b: Box, bi = g.level.boxes.indexOf(b)): PaneType { const gb = g.glassBreak; return (gb ? gb.typeOfPane(bi) ?? gb.typeOf(b) : null) ?? 'annealed'; }   // (bi: the caller's index of b when it has one — the toll does — so nothing is searched per pane per question)

// ---------------------------------------------------------------- holes: where rounds have already gone through each pane (per Game; gone with the encounter)
interface PenState { log: string[]; /** counts every change to the holes (one made, all cleared, a file's put back) */ rev: number }
const states = new GameSideMap<Game, PenState>('glasspen.states');
function stateOf(g: Game): PenState { let s = states.get(g); if (!s) { s = { log: [], rev: 0 }; states.set(g, s); } return s; }
/** Game.resetLevelState, beside resetGlass: the holes go with the stars */
export function resetGlassPen(g: Game) { const s = states.get(g); if (s) { s.log.length = 0; s.rev++; } if (g.sim.open) simX().pen_clear_holes(g.sim.bind()); }
/** A number that moves whenever what a round has to pay at ANY pane may have moved: a through-hole made or cleared here, a piece out of a pane / a pane gone /
 *  every pane whole again in the fracture model. For whoever keeps an answer that was worked out through the toll (orders.ts behindGlass). */
export function glassTollSig(g: Game): number { return (states.get(g)?.rev ?? 0) * 65537 + (g.glassBreak ? g.glassBreak.openSig() : 0); }
function addHole(g: Game, bi: number, p: Vec3) { stateOf(g).rev++; g.glassBreak.asking((x, s) => x.pen_add_hole(s, bi, p[0], p[1], p[2])); }
/** savestate.ts: the holes (pane → its crossings' points), or undefined when there are none — a file written with the switch off carries no such key, as before */
export type GlassPenSnap = [number, number[]][];
export function glassPenSnapshot(g: Game): GlassPenSnap | undefined {
  if (!g.sim.open) return undefined; const x = simX(), s = g.sim.bind(), n = x.pen_hole_rows(s); if (!n) return undefined;
  const out: GlassPenSnap = []; for (let r = 0; r < n; r++) { const pts: number[] = []; for (let i = 0, m = x.pen_hole_n(s, r) * 3; i < m; i++) pts.push(x.pen_hole_pt(s, r, i)); out.push([x.pen_hole_bi(s, r), pts]); }
  return out;
}
/** …and back (after restartEncounter cleared them): the stars come back with the decals, the holes with this — a round by an old hole pays the same after a load.
 *  NOT while the fracture model is live: a load leaves every pane WHOLE there (glassbreak.ts keeps nothing in a checkpoint, and on the page a cracking round files
 *  no star), so a hole put back would be a cheap way through glass that shows no mark. Both come back or neither: with the fracture live, neither. */
export function glassPenRestore(g: Game, s: GlassPenSnap | undefined | null) {
  if (!Array.isArray(s) || glassBreakLive(g)) return; stateOf(g).rev++;
  for (const row of s) { if (!Array.isArray(row) || !isSheet(g.level.boxes[row[0]]) || !Array.isArray(row[1])) continue; const p = row[1].filter(x => Number.isFinite(x)), kept = p.slice(0, Math.min(p.length - p.length % 3, GLASSPEN.holes * 3));
    g.glassBreak.asking((x, b) => { for (let i = 0; i < kept.length; i += 3) x.pen_add_hole(b, row[0], kept[i], kept[i + 1], kept[i + 2]); }); }
}
/** how many through-holes pane bi carries (QA) */
export function holeCount(g: Game, bi: number): number { if (!g.sim.open) return 0; const x = simX(), s = g.sim.bind(); for (let r = 0, n = x.pen_hole_rows(s); r < n; r++) if (x.pen_hole_bi(s, r) === bi) return x.pen_hole_n(s, r); return 0; }

// ---------------------------------------------------------------- the toll
export interface PaneToll { type: PaneType; n: Vec3; cos: number; take: number; eOut: number; hole: boolean }
/** What sheet `bi` takes from a round of energy `e` meeting it at `at` flying `d` (unit), the `nth` pane of its flight (0 = the first) — or null: the round is
 *  spent in it (not a sheet, a graze, too little left, one pane too many). Pure: fireWeapon's path and every line-of-fire question read this one function. */
export function paneToll(g: Game, bi: number, at: Vec3, d: Vec3, e: number, nth = 0): PaneToll | null {
  const r = g.glassBreak.asking((x): PaneToll | null => { if (!x.pen_toll(bi, at[0], at[1], at[2], d[0], d[1], d[2], e, nth)) return null; const m = f64(), o = x.sim_scratch() / 8; return { type: PANE_TYPES[m[o]], n: [m[o + 1], m[o + 2], m[o + 3]], cos: m[o + 4], take: m[o + 5], eOut: m[o + 6], hole: m[o + 7] !== 0 }; });
  return r;
}
const PANE_TYPES: readonly PaneType[] = ['annealed', 'tempered', 'laminated'];
/** a crossing as the module lists it (rust/sim/src/pen.rs write_cross) */
function crossAt(m: Float64Array, o: number, into: number): PaneCross {
  return { type: PANE_TYPES[m[o]], n: [m[o + 1], m[o + 2], m[o + 3]], cos: m[o + 4], take: m[o + 5], eOut: m[o + 6], hole: m[o + 7] !== 0, bi: m[o + 8], t: m[o + 9], at: [m[o + 10], m[o + 11], m[o + 12]], exit: [m[o + 13], m[o + 14], m[o + 15]], dirIn: [m[o + 16], m[o + 17], m[o + 18]], dirOut: [m[o + 19], m[o + 20], m[o + 21]], eIn: m[o + 22], turnDeg: m[o + 23], thru: m[o + 24], into };
}
const CROSS = 25, LEG = 9, MAX_CROSS = 16;

/** one pane a round went through */
export interface PaneCross extends PaneToll {
  bi: number;
  /** metres along the round's path (from the muzzle) to the entry point */
  t: number;
  /** where it went in, where it came out, the direction in and out (unit), energy in, degrees it was turned by */
  at: Vec3; exit: Vec3; dirIn: Vec3; dirOut: Vec3; eIn: number; turnDeg: number;
  /** metres of the path spent inside the glass */
  thru: number;
  /** the sheet the round is INSIDE as it leaves this one (a pane abutting it), or −1 */
  into: number;
}
/** one pane on a round's flight: what it takes and where the round leaves it — or null: spent in it (roundPath below; wallpen.ts thinPath, the flight over panes and thin walls alike) */
export function paneCrossing(g: Game, bi: number, o: Vec3, d: Vec3, t: number, e: number, nth: number): PaneCross | null {
  const r = g.glassBreak.asking((x): PaneCross | null => { if (!x.pen_cross(bi, o[0], o[1], o[2], d[0], d[1], d[2], t, e, nth)) return null; const m = f64(), a = x.sim_scratch() / 8; return crossAt(m, a, m[a + CROSS]); });
  return r;
}

// ---------------------------------------------------------------- the round's flight
/** a straight leg of a round's path: from `o` along `d` for `len` metres, beginning `t0` metres from the muzzle, carrying energy `e` */
export interface RoundSeg { o: Vec3; d: Vec3; t0: number; len: number; e: number }
export interface RoundPath {
  segs: RoundSeg[]; cross: PaneCross[];
  /** the static (or leaf: index ≥ 1e6) the last leg ends in — `t` metres of PATH from the muzzle — or null: it flew the whole range */
  st: { t: number; index: number; n: Vec3 } | null;
  /** metres of path to that stop (the range if none) */
  len: number;
}
type StaticHit = { t: number; index: number; n: Vec3 } | null;
/** The flight of a round of energy `energy` from `from` along `dir` (unit) for at most `range` metres over statics and leaves: every sheet it goes through and
 *  the legs between them. Null = it crosses no pane (it stops in the first thing it meets, or flies clear) — the caller's old straight line stands, untouched.
 *  `first` = the caller's own col.raycast(from, dir, range) if it has one. PURE: nothing is filed until commitCrossings — a body on the near side of the glass
 *  stops the round before the pane ever hears of it. */
export function roundPath(g: Game, from: Vec3, dir: Vec3, range: number, energy: number, first?: StaticHit, nth0 = 0): RoundPath | null {
  const hit: StaticHit = first === undefined ? g.col.raycast(from, dir, range) : first;
  if (!hit || hit.index >= 1e6 || !isSheet(g.level.boxes[hit.index])) return null;   // (the common round: no pane first — not one allocation more than before)
  const r = g.glassBreak.asking((x): RoundPath | null => {
    if (!x.pen_path(from[0], from[1], from[2], dir[0], dir[1], dir[2], range, energy, nth0, hit.t, hit.index, hit.n[0], hit.n[1], hit.n[2])) return null;
    const m = f64(), a = x.sim_scratch() / 8, h = x.pen_path_head() / 8, segs: RoundSeg[] = [], cross: PaneCross[] = [];
    for (let i = 0, n = m[h]; i < n; i++) { const o = a + i * LEG; segs.push({ o: [m[o], m[o + 1], m[o + 2]], d: [m[o + 3], m[o + 4], m[o + 5]], t0: m[o + 6], len: m[o + 7], e: m[o + 8] }); }
    for (let i = 0, n = m[h + 1], c0 = a + (MAX_CROSS + 1) * LEG; i < n; i++) cross.push(crossAt(m, c0 + i * CROSS, m[h + 8 + i]));
    return { segs, cross, st: m[h + 3] < 0 ? null : { t: m[h + 4], index: m[h + 3], n: [m[h + 5], m[h + 6], m[h + 7]] }, len: m[h + 2] };
  });
  return r;
}
/** the leg holding path distance T (a T inside a pane's thickness belongs to the leg before it) */
export function segAt(P: RoundPath, T: number): RoundSeg { let s = P.segs[0]; for (const k of P.segs) { if (k.t0 > T) break; s = k; } return s; }
/** the point T metres along the path */
export function pathAt(P: RoundPath, T: number): Vec3 { const s = segAt(P, T); return v3.mad(s.o, s.d, clamp(T - s.t0, 0, s.len)); }

/** THE IMPACT CALL — one per pane a round meets, entry side: pane, point, direction in, the round's energy as it met the glass, who — for a round that goes
 *  through and for one spent in the pane alike: glass.ts glassImpact (the fracture model's blow, the star, the crack heard, the 'glass' event) at energy ×
 *  GLASSPEN.fractureK — its unit is 1 = a pistol round square on, ours 1 = a rifle round, hence the 2 — WITH THIS RULE'S VERDICT: `through` true = the round went
 *  on (the fracture model leaves a hole where it passed), false = it stopped in the sheet (float glass keeps every piece; toughened still dices at its energy). */
export function paneImpact(g: Game, bi: number, at: Vec3, dir: Vec3, energy: number, who: number, through: boolean, quiet = false) { glassImpact(g, bi, at, dir, energy * GLASSPEN.fractureK, who, 'shot', quiet, { through }); }

/** ONE SHELL ON THE GLASS (shotgun.ts Shell carries it for the nine pellets' flight; nothing outlives the shell but the probes' copy): pane → five numbers per
 *  pellet that met it — x, y, z, went-through (1 | 0), filed-a-mark (1) or joined one (0) */
export interface ShellGlass { glass: Map<number, number[]> }
/** THE SHELL RULE's one question — a pellet of `shell` meets pane `bi` at `at`, going through or stopping in it: 'join' = it landed on a mark its own shell made
 *  (nothing more is filed for it), else the mark is booked and the answer says whether it is this shell's 'first' on the pane or one 'more' (heard as the first was). */
function shellMark(shell: ShellGlass, bi: number, at: Vec3, through: boolean): 'first' | 'more' | 'join' {
  let m = shell.glass.get(bi); if (!m) shell.glass.set(bi, m = []);
  const r2 = GLASSPEN.shellR * GLASSPEN.shellR; let marks = 0, join = false;
  for (let i = 0; i < m.length; i += 5) {
    if (!m[i + 4]) continue; marks++;
    if (through && !m[i + 3]) continue;   // (a pellet that goes through does not pass by the hole of one that stopped)
    const dx = m[i] - at[0], dy = m[i + 1] - at[1], dz = m[i + 2] - at[2]; if (dx * dx + dy * dy + dz * dz < r2) { join = true; break; }
  }
  m.push(at[0], at[1], at[2], through ? 1 : 0, join ? 0 : 1);
  return join ? 'join' : marks === 0 ? 'first' : 'more';
}
/** The round got as far as `tHit` metres of path: every pane it crossed before that is filed — the impact call, the hole remembered, the hook. */
export function commitCrossings(g: Game, cross: PaneCross[], tHit: number, who: number, shell?: ShellGlass) {
  for (const x of cross) {
    if (x.t > tHit + 1e-6) break;   // (a man, a prop on the near side: the round never reached this pane)
    const mark = shell ? shellMark(shell, x.bi, x.at, true) : 'first';
    if (mark === 'join') {   // a pellet through the hole its own shell made: counted, and the piece it passed through (if one still stood) goes — nothing else
      GLASSPEN_STATS.crossed++; GLASSPEN_STATS.shellJoined++; if (x.hole) GLASSPEN_STATS.viaHole++;
      g.glassBreak?.widen(x.bi, x.at); continue;
    }
    paneImpact(g, x.bi, x.at, x.dirIn, x.eIn, who, true, mark === 'more');   // (a shell's later marks on a pane: the glass's one crack and one event were its first's)
    addHole(g, x.bi, x.at); GLASSPEN_STATS.crossed++; if (x.hole) GLASSPEN_STATS.viaHole++;
    const S = stateOf(g); S.log.push(`${g.time.toFixed(1)}s pane ${x.bi} (${x.type}) crossed${x.hole ? ' by its hole' : ''}: energy ${x.eIn.toFixed(2)} → ${x.eOut.toFixed(2)}, turned ${x.turnDeg.toFixed(2)}°${who ? ` (who ${who})` : ''}`); if (S.log.length > 24) S.log.shift();
    GLASSPEN_HOOKS.crossed?.(g, x, who);
  }
}
/** fireWeapon, the rule live: the round STOPPED in static `bi` — if that is glass it gets the impact call (the old stop: the star, the crack) with the energy the round arrived with */
export function spentInPane(g: Game, bi: number, at: Vec3, dir: Vec3, energy: number, who: number, shell?: ShellGlass) {
  const sheet = isSheet(g.level.boxes[bi]); if (sheet) GLASSPEN_STATS.spent++;
  const mark = shell && sheet ? shellMark(shell, bi, at, false) : 'first';   // (a pellet: the shell rule — one that stops on a mark its own shell made cuts no new one)
  if (mark === 'join') { GLASSPEN_STATS.shellJoined++; g.glassBreak?.impact(bi, at, dir, energy * GLASSPEN.fractureK, who, false); return; }   // (no star, no crack, no event — but what stops in the glass stays in the glass: its energy goes on the mark it landed on)
  paneImpact(g, bi, at, dir, energy, who, false, mark === 'more');
}

/** THE MUZZLE ALREADY PAST THE GLASS (combat.ts armThrough: gun shoulder and muzzle on opposite sides of a panel — a man pressed to the glazing): if every panel
 *  across the arm is a sheet a round of `energy` gets through, the round is NOT put into the panel at the muzzle — it leaves the muzzle with what the glass left
 *  it, and the pane is filed at the arm's crossing. Returns the energy left and the panes that took (they count toward maxPanes on the flight from the muzzle),
 *  or null: something a round stops in is across the arm (the old rule stands). `shell` = the pellet's shell (the shell rule: one crossing filed at the arm
 *  point for the nine of them). THE ARM THROUGH A HOLE — the glass across it already fallen where it crosses (the round before made the hole; combat.ts
 *  armThrough tests the pane's box and still says 'a panel', the arm's ray sees the open cell and meets nothing): nothing to pay and nothing to file, the round
 *  flies on from the muzzle whole. (It used to be put into 'the panel' at the muzzle: the second round through one's own hole, and a shell's pellets two to
 *  nine, went nowhere.) */
export function armThroughGlass(g: Game, c: Character, dir: Vec3, energy: number, shell?: ShellGlass): { e: number; panes: number } | null {
  if (!glassPenLive(g)) return null;
  const from = c.bones.upperArmR ?? v3.add(c.pos, [0, 1.4, 0]), mz = c.muzzle, L = v3.len(v3.sub(mz, from));
  const w = g.glassBreak.asking((x): { e: number; cross: PaneCross[] } | null => { const n = x.pen_arm(from[0], from[1], from[2], mz[0], mz[1], mz[2], dir[0], dir[1], dir[2], energy); if (n < 0) return null; const m = f64(), a = x.sim_scratch() / 8, cross: PaneCross[] = []; for (let i = 0; i < n; i++) cross.push(crossAt(m, a + i * CROSS, -1)); return { e: m[a + x.sim_scratch_len() - 1], cross }; });
  if (!w) return null;
  if (!w.cross.length) return { e: energy, panes: 0 };
  commitCrossings(g, w.cross, L, c.id, shell); GLASSPEN_STATS.viaArm++;   // (a shell: its nine pellets cross at the one arm point — the first files it, the rest join it)
  return { e: w.e, panes: w.cross.length };
}

/** StaticCollision.glassPen.passes — the line of FIRE across the panes a segment meets (in order, nothing else in the way): does a round of `energy` get
 *  through every one of them? The toll fireWeapon pays, on the straight line (the turn is the round's business, not the question's). */
export function shotPasses(g: Game, a: Vec3, rd: Vec3, panes: { bi: number; t: number }[], energy = GLASSPEN.defaultEnergy): boolean {
  GLASSPEN_STATS.shotAsked++;
  const ok = g.glassBreak.asking((x) => { const m = f64(), o = x.sim_scratch() / 8; for (let k = 0; k < panes.length; k++) { m[o + 2 * k] = panes[k].bi; m[o + 2 * k + 1] = panes[k].t; } return x.pen_passes(a[0], a[1], a[2], rd[0], rd[1], rd[2], panes.length, energy) === 1; });
  if (ok) GLASSPEN_STATS.shotThrough++; else GLASSPEN_STATS.shotRefused++;
  return ok;
}

/** GLASS IS NOT COVER — how many of `eyes` have a line of fire to `p` that goes THROUGH GLASS: nothing a round stops in on the line (the toll above, for a round
 *  of `energy`), and a pane's box across it (whole, starred or with a piece out: a window is a window). The level's statics alone — the leaves are taken out
 *  for the asking, so the answer is the place's, not this second's. 0 while the rule is not live (a pane is then a wall to a round, as it was). Pure. */
export function shotThroughGlass(g: Game, eyes: Vec3[], p: Vec3, energy = GLASSPEN.stackEnergy): number {
  if (!glassPenLive(g)) return 0;
  const panes = glassPanes(g), leaves = g.col.dynamicBoxes; let n = 0; g.col.dynamicBoxes = [];
  try {
    for (const e of eyes) {
      const d = v3.sub(p, e), L = v3.len(d); if (L < 1e-3) continue; const rd = v3.scale(d, 1 / L);
      let glass = false; for (const bi of panes) { const s = lineBox(e, rd, g.level.boxes[bi]); if (s && s.t1 > 0 && s.t0 < L) { glass = true; break; } }
      if (glass && !g.col.shotBlocked(e, p, 0.02, energy)) n++;
    }
  } finally { g.col.dynamicBoxes = leaves; }
  return n;
}

const linePanes: { bi: number; t: number }[] = [];
/** IS THERE STANDING GLASS ON THIS LINE OF FIRE — a pane a round from `a` to `b` would have to go through (a piece already out of it where the line crosses is
 *  open air, as it is to the round). Asked by a rule that cares WHAT the round would break on its way, not whether it gets there (silent.ts: glass going is
 *  heard across the floor whatever the gun) — and asked of a line the shot test has ALREADY PASSED (silent.ts: clearShot's own line, taken on to the point on the man), so
 *  nothing solid ends the walk before a pane on it is met. False while rounds stop in glass (the switch off, a showcase: a pane is then a wall and there is no
 *  such shot). Pure. */
export function glassOnLine(g: Game, a: Vec3, b: Vec3): boolean {
  if (!glassPenLive(g)) return false;
  linePanes.length = 0; g.col.segmentBlocked(a, b, 0.02, 0, linePanes);
  return linePanes.length > 0;
}

/** panel / console line */
export function glassPenSummary(g: Game): string {
  const S = states.get(g); const st = GLASSPEN_STATS;
  return `rounds through glass ${GLASSPEN.on ? (glassPenLive(g) ? 'ON' : 'on — inert while a showcase runs') : 'off (rounds stop in the pane)'} ▸ ${st.crossed} panes crossed (${st.viaHole} by a hole, ${st.viaArm} muzzle-past) · ${st.spent} rounds spent in a pane${S?.log.length ? ` · last: ${S.log[S.log.length - 1]}` : ''}`;
}
