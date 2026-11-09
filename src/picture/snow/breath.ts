// BREATH: steam off the men's breath on a level built for snow. WHO: every living man on his feet or his knees under the open sky, never a man the fog of war hides. RHYTHM: each man's own, a
// phase and a pace off a hash of his id, and his effort is his ground speed. A BREATH is one source handed to the smoke solver that rides his mouth for as long as he breathes out, within
// reach of the view and while a solver draws; else a few small dust puffs that rise. Who breathes and when is worked out here from the men's rows, by a clock of its own. Nobody reads any of it. Its dice are its own.
import { clamp, type Vec3 } from '../../math/vec';
import type { Particles } from '../../render/particles';
import type { SmokeSystemLike } from '../../smoke/types';
import type { Man } from './tracks';
import { SNOW_LOOK } from './look';

/** how a man's hard breathing shows (rust/game_rules/src/character HARDBREATH, as the page opens) */
const HB = { every: 0.95, puffs: 4, count: 9, jet: 3.2, size: 2.0, carry: 0.85, up: 2.5, down: 14 };

/** The numbers (judged on software-rendered stills only, never seen moving — the maintainer's to move; `__snow.breath` on the console, live with the next breath).
 *  Authored data, not player switches. */
export const SNOWBREATH = {
  every: 3.6,          // s between breaths at rest (× 1 ± everyJit per man, and ± a tenth breath to breath)
  everyJit: 0.18,
  everyHard: 1.3,      // s between breaths when blown
  out: 0.5,            // s one breath takes to leave him at rest (shorter as the breaths come faster)
  puffs: 3,            // records a breath is filed as, across `out`
  count: 5,            // puffs in each record
  speed: 0.7,          // m/s out of the mouth (the puff's drag stops it inside a hand's length; the last of a breath leaves slower)
  spread: 0.24,        // rad: the cone's half angle
  down: 0.12,          // how far below the head's facing it leaves (a share of straight down)
  pitch: [-0.45, 0.3] as [number, number],   // the head's own pitch it follows, held to this (the facing's y: −1 straight down)
  carry: 0.6,          // × his own velocity it leaves with
  life: 1.6,           // s (× 1 ± lifeJit)
  lifeJit: 0.25,
  size: 0.085,         // m: a puff's radius at birth (× 1 ± sizeJit; the kind grows it by three fifths over its life)
  sizeJit: 0.3,
  opacity: 0.66,       // × the dust puff's own peak opacity (rust/game_rules/src/particle_fx DUST_LOOK.opacity, 0.3)
  hardK: 1.5,          // × size and opacity when blown
  color: [0.9, 0.93, 1.0] as Vec3,   // what it scatters: near white, a touch cold
  mouthAhead: 0.1,     // m from the head bone (eye level) forward to the lips
  mouthDown: 0.075,    // m below it
  minHead: 0.75,       // m: a head lower than this over his feet is a man lying down — no breath is shown off him
  runLo: 1.9,          // m/s: under this his pace costs him nothing
  runHi: 4.0,          // m/s: at this he is working as hard as he can
  effortUp: 4,         // s of running until he is blown
  effortDown: 12,      // s to get his wind back
};

/** THE STEAM'S NUMBERS (SNOW_LOOK.steam: the breath through the smoke solver). Set from the solver's arithmetic and from software-rendered stills, NEVER SEEN ON A
 *  GPU and never timed — the maintainer's to move (`__snow.steam` on the console, live with the next breath). Authored data, not player switches. The rhythm, the
 *  mouth, the pitch and how much of his walk it leaves with are SNOWBREATH's (and the hard breath rule's) as for the puffs: only what the gas is made of is here.
 *  HOW THICK: a guard's sight line starts at his eye, a hand above his own mouth, and nothing in the game reads smoke until a line has lost a fifth of its light
 *  (rust/game_rules/src/guards: (T − 0.3) / 0.5, held to 0‥1), so `density` is kept where a line through the thick of a breath loses less than that. */
export const BREATHSTEAM = {
  radius: 0.034,       // m: the gaussian the mouth lets it out through (× √size when blown): a wisp like the barrel's. TRIED AND PUT BACK: 0.09 with `density` 1.4
                       // and `temp` 2.5, a wide thin cloud the puffs' own size and no thicker along a sight line — it held the pool three to five times
                       // the bricks (963 for five men, still rising, where this holds about 170) and showed no better in a software still
  cells: 1.5,          // … never under this many cells of the fine lattice (a splat under a cell lays down next to nothing, and flickers)
  density: 9,          // what the mouth lets out a second at its core (× size when blown; the solver's unit: 6 /m of extinction each)
  temp: 4,             // heat a second at its core: what lifts it (the solver: +1.6 m/s² a unit, gone in under a second on the fine lattice)
  speed: 0.8,          // m/s out of the mouth at rest (× the jet when blown). The solver's air has next to no drag: this is how far the plume reaches
  push: 12,            // 1/s: how hard the air at the mouth is taken up to that speed
  expand: 2,           // 1/s: how fast the breath opens as it leaves
  attack: 0.08,        // s for a breath to come up (it tails off over its last quarter)
  jitter: 0.3,         // how much its heading and speed wander frame to frame: what keeps two breaths from being one shape
  carryK: 0.5,         // × the share of his own velocity the puffs leave with (`carry`): in air that does not stop it, a runner's breath is left behind him, not thrown ahead
  wake: 0.16,          // m: a walking man's head shoves the smoke it passes through, this wide (the solver's push; 0 = his hips and chest alone, as any smoke)
  reach: 12,           // m from where the view looks: a man further off breathes puffs (the frame's far corners are 11 m from it at the game's own zoom, and a breath
                       // holds some ninety bricks of the fine lattice for two seconds: the men nobody can see should not; 0 = every man steams)
};

interface Row { ph: number; pace: number; effort: number; n: number; left: number; of: number; gap: number; step: number; shows: boolean; at: number; steam: boolean; asked: number; src: number }
const mix = (x: number): number => { x = (x ^ 61) ^ (x >>> 16); x = (x + (x << 3)) >>> 0; x ^= x >>> 4; x = Math.imul(x, 0x27d4eb2d) >>> 0; x ^= x >>> 15; return x >>> 0; };
const SEED = 0x0b7ea7b5;
/** this module's own die: a number in 0‥1 off (a man, a count), the same every run */
const h01 = (a: number, b: number): number => mix((Math.imul(a + 1, 2654435761) ^ Math.imul(b + 1, 2246822519) ^ SEED) >>> 0) / 4294967296;
const recSeed = (id: number, breath: number, k: number): number => mix((Math.imul(id + 1, 3266489917) ^ Math.imul(breath * 16 + k + 1, 668265263) ^ SEED ^ 0xb4ea7) >>> 0);

/** his mouth, and where his breath leaves it (unit): the head's heading, its pitch held inside `pitch`, a share of straight down */
function mouthOf(c: Man): { pos: Vec3; dir: Vec3 } {
  const K = SNOWBREATH, hd = c.facing, head = c.head, hl = Math.hypot(hd[0], hd[2]);
  const fx = hl > 0.2 ? hd[0] / hl : Math.sin(c.bodyYaw), fz = hl > 0.2 ? hd[2] / hl : Math.cos(c.bodyYaw);
  const py = clamp(hd[1], K.pitch[0], K.pitch[1]), ph = Math.sqrt(Math.max(0, 1 - py * py)), f: Vec3 = [fx * ph, py, fz * ph], dl = Math.hypot(f[0], f[1] - K.down, f[2]) || 1;
  return { pos: [head[0] + fx * K.mouthAhead, head[1] - K.mouthDown, head[2] + fz * K.mouthAhead], dir: [f[0] / dl, (f[1] - K.down) / dl, f[2] / dl] };
}

export class SnowBreath {
  private men = new Map<number, Row>(); private tick = 0; private byId = new Map<number, Man>();
  constructor(private P: Particles | null, private smoke: SmokeSystemLike, private underRoof: (x: number, z: number) => boolean) {}
  /** a load, a restart */
  clear(): void { this.men.clear(); }
  private row(c: Man): Row {
    let m = this.men.get(c.id);
    if (!m) { m = { ph: h01(c.id, 1), pace: 1 + SNOWBREATH.everyJit * (2 * h01(c.id, 2) - 1), effort: 0, n: 0, left: 0, of: Math.max(1, Math.round(SNOWBREATH.puffs)), gap: 0, step: 0, shows: false, at: 0, steam: false, asked: -1, src: -1 }; this.men.set(c.id, m); }
    return m;
  }
  /** Once a tick. Picture only. */
  update(men: readonly Man[], dt: number, focus: ArrayLike<number>): void {
    if (!SNOW_LOOK.breath || !(dt > 0)) return;
    const K = SNOWBREATH, B = BREATHSTEAM, P = this.P, steam = SNOW_LOOK.steam && B.density > 0 && this.smoke.drawing?.() === true, tick = ++this.tick;
    this.byId.clear();
    for (const c of men) { this.byId.set(c.id, c); if (c.dead) continue; this.one(c, c.hidden ? false : true, dt, tick, steam, focus, P); }
    for (const [id, m] of this.men) if (m.at !== tick) this.men.delete(id);
  }
  private one(c: Man, seen: boolean, dt: number, tick: number, steam: boolean, focus: ArrayLike<number>, P: Particles | null): void {
    const K = SNOWBREATH, B = BREATHSTEAM, m = this.row(c); m.at = tick;
    // his wind: up while he runs, back slowly once he stops
    const sp = Math.hypot(c.vel[0], c.vel[2]), want = clamp((sp - K.runLo) / Math.max(0.01, K.runHi - K.runLo), 0, 1);
    m.effort = want > m.effort ? Math.min(want, m.effort + dt / Math.max(0.1, HB.up)) : Math.max(want, m.effort - dt / Math.max(0.1, HB.down));
    const every = Math.max(0.3, (K.every + (HB.every - K.every) * m.effort) * m.pace * (0.9 + 0.2 * h01(c.id, 16 + m.n)));
    m.shows = seen && c.alive && c.head[1] - c.pos[1] >= K.minHead && !this.underRoof(c.head[0], c.head[2]);
    const inReach = steam && (!(B.reach > 0) || Math.hypot(c.pos[0] - focus[0], c.pos[2] - focus[2]) <= B.reach), near = inReach && m.shows;
    if (near && B.wake > 0 && sp >= 0.25) this.smoke.push({ pos: c.head as Vec3, dir: [c.vel[0] / sp, 0.05, c.vel[2] / sp], speed: sp * 1.1, radius: B.wake, lattice: 'fine' });
    // the rhythm runs whether or not it shows (a man who steps out from under a roof is in mid breath, not at the start of one)
    m.ph += dt / every;
    if (m.ph >= 1) { m.ph = m.ph >= 2 ? 0 : m.ph - 1; m.n++; m.of = m.left = Math.max(1, Math.round(K.puffs + (HB.puffs - K.puffs) * m.effort)); m.gap = 0; m.step = Math.min(K.out, every * 0.4) / m.left; }
    if (m.left <= 0) return;
    m.gap -= dt; if (m.gap > 0) return;
    m.gap += m.step; m.left--;
    if (!m.shows) return;
    // is THIS breath steam through the solver: said when its first record falls due and kept for the breath
    if (m.asked !== m.n) { m.asked = m.n; m.steam = inReach; m.src = -1; }
    const hard = 1 + (HB.size - 1) * m.effort, carry = K.carry + (HB.carry - K.carry) * m.effort, jet = 1 + (HB.jet - 1) * m.effort;
    if (m.steam) {
      // the whole breath is one source on the solver's fine lattice, let go when a record of it would be filed and none of this breath's rides his mouth. It rides his mouth for what is left of the breath and is dropped the tick he dies, lies down, or goes under a roof or into the fog
      if (!steam || m.src === m.n) return;
      const born = m.n, v = B.speed * jet, cv = carry * B.carryK, id = c.id;
      const at = (): { pos: Vec3; dir: Vec3 } | null => {
        const now = this.byId.get(id), r = this.men.get(id);
        if (!now || !r || !now.alive || !r.shows || r.n !== born || this.tick - r.at > 1 || !SNOW_LOOK.steam) { if (r && r.src === born) r.src = -1; return null; }
        const a = mouthOf(now); return { pos: a.pos, dir: [a.dir[0] * v + now.vel[0] * cv, a.dir[1] * v, a.dir[2] * v + now.vel[2] * cv] };
      };
      const a0 = at(); if (!a0) return;
      this.smoke.emit({ pos: a0.pos, dir: a0.dir, speed: Math.hypot(a0.dir[0], a0.dir[1], a0.dir[2]), radius: B.radius * Math.sqrt(hard), minVoxels: B.cells, density: B.density * hard, temperature: B.temp * hard,
        ttl: m.step * (m.left + 1), age: 0, kind: 'breath', push: B.push, expand: B.expand, attack: B.attack, jitter: B.jitter, lattice: 'fine', confined: true, look: true, trackVel: true, track: at });
      m.src = born; return;
    }
    if (!P) return;
    const a = mouthOf(c), tail = 1 - 0.3 * (1 - (m.left + 1) / m.of);   // (the last of a breath leaves slower than the first)
    P.emit('dust', a.pos, a.dir, Math.max(1, Math.round(K.count + (HB.count - K.count) * m.effort)), { shape: 'cone', speed: K.speed * tail * jet, speedJit: 0.35, spread: K.spread, life: K.life, lifeJit: K.lifeJit, size: K.size * hard, sizeJit: K.sizeJit, posJit: 0.012,
      vadd: [c.vel[0] * carry, 0, c.vel[2] * carry], owner: c.id, color: K.color, steam: clamp(K.opacity * hard, 0, 1), seed: recSeed(c.id, m.n, m.left) });
  }
}
