// The FN2000's 40 mm high-explosive round (gear: the launcher's second attachment, beside the ring airfoil — launcher.ts). Out of the same tube under the
// fore-end, on the same right button, down a previewed arc — but at a low-velocity grenade's 76 m/s, and with a nose fuze:
//   - it ARMS only once it has flown HE40.armM metres of its own path (the fuze counts the round's turns, which is to say its distance). The default is the
//     blast's own outer radius, so an armed round can wound the man who fired it only if he walked toward it;
//   - armed, it goes off on the first thing it meets — a wall, a door, a desk, the floor, a man — with THE SAME BLAST the hand grenade has: frag.ts blast(),
//     its radii, its cover rule, its bang, its one call into the destruction lane. Nothing here judges anybody, calls g.destr or touches a door;
//   - short of that it is a DUD: a quarter-kilo slug that slaps into whatever it met, clatters down and lies there inert (a 'shell' item — throwables.ts), a
//     noise the room goes to look at. Into a man it is the ring's body hit: he folds for a moment, then he is alert and knows the side it came from.
// Impartial like the blast: his own side, the unarmed and Sam himself are judged by where they stand when it bursts. Never hit points; no dice of its own (the
// blast's one hashed roll aside). Sam's, and a teammate's on order (squad40.ts: the same round by `fireShell`). Rounds in flight live on Game.shells; game.ts steps them beside the rings.
//
// (No top-level statement here uses another game module's export: src/game is one import cycle — see CLAUDE.md.)
import './seats';
import { Vec3, v3 } from '../math/vec';
import type { Character } from './character';
import type { Game, Guard } from './game';
import { type ThrowSolution } from './throwables';
import { bodyHit } from './combat';
import './civilians';
import './particleFx';
import { arcSolve, launchVel } from './launcher';
import { blast, FRAG, type BlastReport } from './frag';
import { RUST_PASS, RUST_SURFACE, goneWithTick } from './crossing';

/** Every number of the round's flight and of what a dud does (the blast's numbers are frag.ts FRAG's: one blast). */
export const HE40 = {
  /** muzzle speed (m/s: the 40×46 mm low-velocity round), gravity, air drag (/s), contact radius (m), seconds of flight before it is simply gone (a round sent
   *  up and out over the lot), substeps a frame, and the passes of the elevation solve (launcher.ts launchVel) */
  speed: 76, gravity: 9.81, drag: 0.12, radius: 0.03, life: 2.5, substeps: 4, solveIters: 2,
  /** metres of its own path before the fuze is armed: anything it meets sooner gets a dud */
  armM: 8,
  /** how far off the surface it struck the burst is judged from (m): the blast's point must sit in the room, not in the plaster */
  standoff: 0.15,
  /** hearing ('prop' events, as the ring's): the tube's thump AT THE SHOOTER (not gunfire: a hollow pop, but a bigger one than the ring's), a dud slapping
   *  into a wall (who 0: a noise nobody can put a man behind), a dud into a civilian or Sam (into a guard the one noise is his fold's: launcher.ts staggerGuard) */
  launchNoise: 0.7, dudNoise: 0.45, dudHitNoise: 0.45,
  /** a dud: N·s into the chair it strikes, the share of its normal / sliding speed it keeps coming off a surface, and the most it carries on as an item */
  propJ: 9, restitution: 0.15, keepTangent: 0.3, spentSpeedMax: 6,
};
/** Every round's end, newest last (QA; capped): where, how far it had flown, armed or not, what it met, and the blast's report when there was one */
export interface ShellEnd { at: Vec3; flown: number; armed: boolean; met: 'static' | 'prop' | 'floor' | 'man' | 'air'; report: BlastReport | null; }
export const HE40_LOG: ShellEnd[] = [];
export const HE40_STATS = { fired: 0, bursts: 0, duds: 0, gone: 0 };
export function he40StatsReset() { HE40_LOG.length = 0; const S = HE40_STATS; S.fired = S.bursts = S.duds = S.gone = 0; }

/** One round in the air: where, how fast, how long, how many metres of its own path it has flown (the fuze), whose, its roll (for the picture) */
export interface Shell { pos: Vec3; vel: Vec3; age: number; flown: number; shooter: Character; spin: number; }

/** the length of a previewed arc (m), and whether a round that flew it would be armed where it lands */
export function arcLength(sol: ThrowSolution): number { let L = 0; for (let i = 1; i < sol.points.length; i++) L += v3.dist(sol.points[i - 1], sol.points[i]); return L; }
export function arcArmed(sol: ThrowSolution): boolean { return arcLength(sol) >= HE40.armM; }
/** the crown test both the flight and the preview hand combat.ts bodyHit: a round that starts inside a man's column, below his crown, has met him */
const insideAt = (pos: Vec3) => (c: Character) => pos[1] < (c.bones.headTop?.[1] ?? c.pos[1] + 1.75) + HE40.radius;
/** The living man under the reticle that a round can MEET: the puppet's target or the snapped man — never one of Sam's own side, whom a round in flight
 *  passes as his bullets do (updateShells): with a teammate under the reticle the round is aimed at, and previewed to, whatever is beyond him. */
export function shellMan(g: Game): Guard | null { return goneWithTick('shellMan') as any; }
/** The living bodies a round in flight stops on, as far as Sam can SEE them: the flight's own list (updateShells: everybody but his own side) less the men the
 *  fog of war hides — an arc that stopped short on an unseen man would give him away. */
export function shellBodies(g: Game): Character[] { return goneWithTick('shellBodies') as any; }
/** Where a round fired now from `from` at `target` would go (launcher.ts arcSolve by this round's table: the polyline the overlay draws, to the first static,
 *  leaf, prop or floor — the round's centre where it stops, which is where the flight counts its path to). Men are not in that solve, as they are not in the
 *  ring's — so the line is then cut where it first enters one of `bodies` (shellBodies: the men he can see), by the flight's own test: the round stops there,
 *  and whether it would be ARMED there (arcArmed: the reticle's word, the arc's colour) is the path to that man, not to the wall behind him. A man who moves
 *  in the tenth of a second it flies is met where he then is: the word is exact for one standing. */
export function shellSolve(g: Game, from: Vec3, target: Vec3, bodies: Character[] = []): ThrowSolution { { const __s = RUST_SURFACE; if (__s) { const __r = __s.call('shellSolve', undefined, [g, from, target, bodies]); if (__r !== RUST_PASS) return __r as any; } }
  const sol = arcSolve(g, from, target, HE40); if (!bodies.length) return sol;
  const P = sol.points;
  for (let i = 1; i < P.length; i++) {
    const seg = v3.sub(P[i], P[i - 1]), len = v3.len(seg); if (len < 1e-6) continue;
    const dir = v3.scale(seg, 1 / len), inside = insideAt(P[i - 1]); let best = Infinity;
    for (const c of bodies) { const t = bodyHit(c, P[i - 1], dir, len, inside); if (t >= 0 && t < best) best = t; }
    if (best < Infinity) { const land = v3.mad(P[i - 1], dir, best); return { v0: sol.v0, T: sol.T, points: [...P.slice(0, i), land], land }; }
  }
  return sol;
}


/** Let one round go from `from` at `target` off `shooter`'s launcher: in the air from this frame (Game.shells), the tube's thump, and the room hears it go. */
export function fireShell(g: Game, shooter: Character, from: Vec3, target: Vec3): Shell { { const __s = RUST_SURFACE; if (__s) { const __r = __s.call('fireShell@src/game/he40.ts', undefined, [g, shooter, from, target]); if (__r !== RUST_PASS) return __r as any; } }
  const sh: Shell = { pos: v3.copy(from), vel: launchVel(from, target, HE40), age: 0, flown: 0, shooter, spin: 0 };
  g.shells.push(sh); HE40_STATS.fired++;
  g.audio.play('launcher', from, 1.0, { rate: 0.6 });   // (no sample of its own yet: the ring launcher's, pitched down to a thump)
  g.events.push({ kind: 'prop', pos: v3.copy(shooter.pos), time: g.time, loud: false, level: HE40.launchNoise, who: shooter.id });
  if (shooter === g.player.char) g.player.noise = Math.max(g.player.noise, HE40.launchNoise);
  return sh;
}

function logEnd(e: ShellEnd) { HE40_LOG.push(e); if (HE40_LOG.length > 32) HE40_LOG.shift(); }
/** ARMED and stopped: the burst, judged from `pt` (a point in the room, HE40.standoff off whatever it struck) — frag.ts's blast and nothing else. */
function burst(g: Game, sh: Shell, pt: Vec3, met: ShellEnd['met']) {
  const report = blast(g, [pt[0], pt[1] - FRAG.lift, pt[2]], { who: sh.shooter.id, name: '40 mm grenade' });   // (blast() judges from FRAG.lift over the point it is given: handed the point that much lower, it judges from `pt`)
  HE40_STATS.bursts++; logEnd({ at: v3.copy(pt), flown: sh.flown, armed: true, met, report });
}



