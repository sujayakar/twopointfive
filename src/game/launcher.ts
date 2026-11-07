// The FN2000's ring-airfoil launcher (docs/internal/sc20k-design.md §5, lane R5): a rubber ring out of the tube under the fore-end at 30 m/s along a previewed,
// nearly flat arc — gravity and a little drag on it — that puts a man OUT COLD if it takes him in the head, the neck or high in the chest (combat.ts killGuard's
// quiet path: 'struck', breathing, what he held laid down, a thud the room hears — the takedown at fifteen metres), STAGGERS him anywhere lower (doubled over a
// second and a half, no round from him, a yelp — and then he is alert, and knows which side it came from), and clatters off whatever else it meets as a noise the
// room goes to look at. Never lethal, never suppression (a ring is not a round: nothing here calls suppression.ts), never retrievable: a spent ring is a 'ring'
// item on the floor (throwables.ts) and the count only ever falls. Any Character may fire one (fireRing takes the shooter): Sam today off the rifle in his hands
// (player.ts: the RING attachment selected with the rifle's slot digit, let go on the right button), an operative handed the same rifle later. Rings in flight live on Game.rings; game.ts steps them right after the player and
// draws them with the items.
import { manById } from './seats';
import { Vec3, v3 } from '../math/vec';
import { gearText } from './gear';
import { Box } from '../scene/boxes';
import type { Character } from './character';
import type { Game, Guard } from './game';
import type { ThrowSolution } from './throwables';
import './combat';
import './civilians';
import { endGuardScript } from './squad';
import './particleFx';
import { w } from './words';   // (a leaf: the dialogue pass's one switch)
import { loadKeys } from './slotsetup';
import { simRandom } from '../math/dice';   // the simulation's die (math/dice.ts): the Game's own when it has one, Math.random otherwise
import { word } from './belief';
import { RUST_PASS, RUST_SURFACE, crossToRust, goneWithTick } from './crossing';

/** Every number of the ring's flight and of what it does (panel-tunable; the QA probes read them rather than copy them). */
export const RING = {
  /** muzzle speed (m/s), gravity (m/s²), air drag (/s: v ×= e^(−drag·t)), the ring's contact radius (m), seconds of flight before it is spent regardless, substeps a frame */
  speed: 30, gravity: 9.81, drag: 0.4, radius: 0.04, life: 1.5, substeps: 4,
  /** where on a man is 'out cold': the hit point no lower than this far under his neck joint (the throat, the jaw, the skull, the top of the sternum — CT-generous);
   *  anything lower staggers. The body is Character.rayHit's cylinder (0.28 about the hips, soles to crown), the same body a round finds. */
  koBelowNeck: 0.07,
  /** the stagger: seconds doubled over (Guard.staggerUntil — rooted, gun down, no round; guards.ts), his trigger dead at least this long, the awareness it leaves him
   *  with (0.92: alert — somebody just shot him with something), and how far back along the ring's line his fix on 'where from' is put when he has no better one */
  stagger: 1.5, staggerFireCd: 1.5, staggerAware: 0.92, staggerBack: 4,
  /** hearing ('prop' events, the room's business — guards.ts hearingCheck's door / furniture branch: ~9 m of path at 0.45, ~6 at 0.3): the launcher going off AT
   *  THE SHOOTER, the ring clacking off a wall / a desk at the impact (who 0: a noise the room cannot explain — men go and look at IT, which is half of what the thing
   *  is for), a man folding under a head hit (the takedown's 0.3, at him), the thump and yelp of a body hit */
  launchNoise: 0.45, clackNoise: 0.3, koNoise: 0.3, hitNoise: 0.35,
  /** N·s into a chair or a carton it strikes (it hops); how much of its normal speed a ring keeps coming back off a wall and how much of its slide along it (rubber
   *  at thirty metres a second: most of it goes in the slap), and the most it may carry into throwables' hands after — so it drops and skitters by the mark it
   *  made its noise at, where the men who heard it will look, rather than skating off down the corridor */
  propJ: 3, restitution: 0.2, keepTangent: 0.25, spentSpeedMax: 5,
  /** aim: lead the man under the reticle by his ground velocity (one iteration), and solve the launch elevation so the arc passes through the aim point (two) */
  lead: true, solveIters: 2,
};
/** the yelp of a man a ring has just doubled over (placeholder lines — the writing pass is the maintainer's) */
const STAGGER_LINES = ['agh — what the—', 'AH! son of a—', 'ow! what— who\'s there?!', 'nngh— something hit me!', 'jesus— my ribs—'];

/** One ring in the air: where, how fast, how long, whose (the Character that fired it: never hit by it, blamed for the noise, credited with the knockout), its roll about the line of flight (for the picture) */
export interface Ring { pos: Vec3; vel: Vec3; age: number; shooter: Character; spin: number; }

/** What a thing out of the launcher's tube flies by (the ring's RING; the 40 mm round's table in he40.ts): launchVel and arcSolve read only these. */
export interface Ballistics { speed: number; gravity: number; drag: number; radius: number; life: number; substeps: number; solveIters: number; }
/** the launch velocity for a ring from `from` to pass through `target`: dead at it, raised by however far a simulated ring falls short over that distance (twice) */
export function launchVel(from: Vec3, target: Vec3, R: Ballistics = RING): Vec3 {
  const d = v3.sub(target, from); const L = v3.len(d); if (L < 1e-3) return [0, 0, R.speed];
  let aim = v3.copy(target);
  let v0: Vec3 = v3.scale(v3.normalize(d), R.speed);
  for (let it = 0; it < R.solveIters; it++) {
    v0 = v3.scale(v3.normalize(v3.sub(aim, from)), R.speed);
    // fly it (no collisions) to the target's horizontal distance and see how far under the target it passes
    const hd = Math.hypot(d[0], d[2]); let p = v3.copy(from), v = v3.copy(v0); const h = 1 / 240; let yAt = NaN;
    for (let i = 0; i < R.life / h; i++) {
      const k = Math.exp(-R.drag * h); v = [v[0] * k, v[1] * k - R.gravity * h, v[2] * k];
      const np: Vec3 = v3.mad(p, v, h);
      const a = Math.hypot(p[0] - from[0], p[2] - from[2]), b = Math.hypot(np[0] - from[0], np[2] - from[2]);
      if (b >= hd) { const u = b > a ? (hd - a) / (b - a) : 1; yAt = p[1] + (np[1] - p[1]) * u; break; }
      p = np;
    }
    if (!Number.isFinite(yAt)) break;
    aim = [aim[0], aim[1] + (target[1] - yAt), aim[2]];   // aim higher by the shortfall (converges in two: the arc is nearly flat)
  }
  return v0;
}

/** Where a ring fired now from `from` at `target` would go: the launch velocity, and the polyline of its flight until it first meets a static, a leaf, a prop or the
 *  floor (or its life runs out) — the shape the canisters' preview draws (ThrowSolution: player.ts hands it to the overlay as throwPreview). Men are not in it: the
 *  line shows the arc, the reticle shows the man. */
export function ringSolve(g: Game, from: Vec3, target: Vec3): ThrowSolution { return crossToRust('ringSolve', undefined, [g, from, target]) as any; }
/** ringSolve for anything the tube throws (the ring; he40.ts's round): the same stepping as the live flight, by that thing's own table */
export function arcSolve(g: Game, from: Vec3, target: Vec3, R: Ballistics): ThrowSolution { { const __s = RUST_SURFACE; if (__s) { const __r = __s.call('arcSolve', undefined, [g, from, target, R]); if (__r !== RUST_PASS) return __r as any; } }
  const v0 = launchVel(from, target, R);
  // stepped exactly as updateRings steps a live ring at 60 Hz (dt / substeps), so the polyline is the flight to the centimetre; a point every substeps-th step
  const points: Vec3[] = []; let p = v3.copy(from), v = v3.copy(v0); let land = v3.copy(target); const h = 1 / 60 / R.substeps; let T = 0;
  for (let i = 0; i * h < R.life; i++) {
    if (i % R.substeps === 0) points.push(v3.copy(p));
    const k = Math.exp(-R.drag * h); v = [v[0] * k, v[1] * k - R.gravity * h, v[2] * k];
    const np = v3.mad(p, v, h); const seg = v3.sub(np, p); const len = v3.len(seg); T = i * h;
    if (len > 1e-6) { const dir = v3.scale(seg, 1 / len); const reach = len + R.radius; const st = g.col.raycast(p, dir, reach); const ph = g.props.raycast(p, dir, st ? st.t : reach); const t = ph ? ph.t : st ? st.t : -1; if (t >= 0) { land = v3.mad(p, dir, Math.max(0, t - R.radius)); points.push(land); break; } }
    if (np[1] < R.radius) { land = [np[0], R.radius, np[2]]; points.push(land); break; }
    p = np; land = v3.copy(np);
  }
  return { v0, T, points, land };
}


/** Let one ring go from `from` at `target` off `shooter`'s launcher: it is in the air from this frame (Game.rings), the launcher's whoosh-thunk plays at the tube, and
 *  the room inside nine metres or so hears it go (a 'prop' event at the shooter, RING.launchNoise — not gunfire: nobody eighty metres off learns a thing). */
export function fireRing(g: Game, shooter: Character, from: Vec3, target: Vec3): Ring { return crossToRust('fireRing', undefined, [g, shooter, from, target]) as any; }



/** A ring in the body (below the knockout line): he doubles over — rooted RING.stagger seconds the way the dazzle roots a man (Guard.staggerUntil → reactT; guards.ts
 *  lowers the gun and lets no round go meanwhile) with the flinch on him, any burst or choreography he was in over, a yelp — and it leaves him ALERT (awareness to
 *  gunfire's 0.92: the transition, its CONTACT and the floor's convergence follow next frame as for anything else that puts a man there) with his fix, if he had none
 *  better, RING.staggerBack metres back up the ring's line: the side it came from. The thump and the yelp are a noise at him. A man held in Sam's arm only flinches. */
export function staggerGuard(g: Game, gd: Guard, dir: Vec3, shooter: Character) { { const __s = RUST_SURFACE; if (__s) { const __r = __s.call('staggerGuard', undefined, [g, gd, dir, shooter]); if (__r !== RUST_PASS) return __r as any; } }
  if (gd.state === 'dead') return;
  const R = RING, c = gd.char;
  c.anim.hit(); g.audio.play('bodyHit', v3.add(c.pos, [0, 1.1, 0]), 0.55, { rate: 1.05 });
  if (gd.held) return;
  if (gd.script) endGuardScript(gd, gd.state === 'patrol' ? 'suspicious' : gd.state === 'search' ? 'search' : gd.state === 'alert' ? 'alert' : 'suspicious');   // (choreography ends where he stands, as a grab ends it)
  gd.staggerUntil = Math.max(gd.staggerUntil, g.time + R.stagger); gd.reactT = Math.max(gd.reactT, R.stagger); gd.fireCd = Math.max(gd.fireCd, R.staggerFireCd);
  gd.weapon.burst = null;
  gd.path = []; gd.pathGoal = null; gd.stallRef = null; gd.speed = 0; c.vel = [0, 0, 0];
  if (gd.state !== 'alert' || !gd.lastKnown) {   // where from: back up the line it flew, on the floor, somewhere a man can stand (else where he is)
    let back: Vec3 = [c.pos[0] - dir[0] * R.staggerBack, 0, c.pos[2] - dir[2] * R.staggerBack];
    if (g.col.nav.isBlocked(back[0], back[2])) back = g.col.nav.nearestFreePoint(back[0], back[2]) ?? v3.copy(c.pos);
    word(g, gd, back);
  }
  gd.awareness = Math.max(gd.awareness, R.staggerAware);
  if (g.aiEnabled) gd.witness.heard.small++;   // (his record: something got him — counted with the small things; the alert it tips him into books itself next frame)
  { const pool = w(gd.state === 'alert' ? STAGGER_LINES.filter(t => !/who's there/.test(t)) : STAGGER_LINES, STAGGER_LINES); g.say(gd, pool[Math.floor(simRandom() * pool.length) % pool.length]); }   // (words.ts: a man already fighting Sam knows who is there; one roll either way)
  g.events.push({ kind: 'prop', pos: v3.copy(c.pos), time: g.time, loud: false, level: R.hitNoise, who: shooter.id });
  { const pl = manById(g, shooter.id); if (pl) pl.noise = Math.max(pl.noise, R.hitNoise); }
}
/** the stagger lines (the QA soak counts them as their own smell, not a bark repeat) */
export function staggerLines(): string[] { return [...STAGGER_LINES]; }

/** Rings in flight as boxes: three thin slats crossed at 60° in the plane square to its flight, rolling about it — a small pale blur of a ring going away from you.
 *  Raster-only (NoShadow): nothing traces a 6 cm rubber ring. Under the 2010s issue kit (gear.ts GEAR.on) the same thing in the air is drawn as what it is
 *  worded there, a 40 mm sponge round: throwables.ts spongeBoxes, three boxes under the same flags — the picture alone; the flight above reads neither. */
export function ringBoxes(g: Game, out: Box[]): void { return goneWithTick('ringBoxes') as any; }

/** Panel / QA: 'rings ▸ 4 left · attachment RING · 1 in the air · 2 on the floor' */
export function ringSummary(g: Game): string {
  const rf = g.player.rifle;
  return `rings ▸ ${g.player.hasRifle ? `${rf.rings} left · attachment ${rf.attachment ? rf.attachment.toUpperCase() : 'none'} (${loadKeys()} with the rifle up; RMB fires it)` : gearText('no FN2000 in the kit')} · ${g.rings.length} in the air · ${g.items.items.filter(x => x.kind === 'ring').length} on the floor`;
}
