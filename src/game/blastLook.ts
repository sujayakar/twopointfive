// THE LOOK OF A SET CHARGE AND OF ITS BLAST — picture only (the destruction look goal: 'a charge is seen set and its blast fades out').
//
// What it draws, and nothing else:
//   · A SET WALL CHARGE you can see from the game's camera. wallcharge.ts draws the door charge's slab and a 2 cm lit tell (one red pixel from 16 m); this adds, on the
//     face it was set from: the frame of explosive strip round the slab (the outline of the hole it is there to cut), a lamp that BLINKS, and the firing line down the
//     wall and out across the floor to its reel, which carries a small steady lamp of its own. Raster-only boxes (Dynamic | NoShadow: no ray, no query sees them),
//     the line widened to the frame's pixel like the lasers. In a dark room what shows is the two lamps, what they throw on the strip and the line beside
//     them, and the faint marker tape along both (all painted on as emission: no light is added); the rest is there for any light that falls on it.
//   · THE BLAST OF A WALL CHARGE. The report, the flash and the smoke stay the stun charge's, to the number (combat.ts detonateStun, called by wallcharge.ts blow()) —
//     the flash's COLOUR turned from the stun charge's magnesium white to an explosive's orange-white at the same luminance (WALL_FLASH), so that as it falls from
//     its white peak it is seen to cool. In place of the stun charge's all-round fountain of fragments: OUT OF THE WALL'S TWO FACES a fast narrow jet of sparks, a
//     glowing core that sits in the hole and cools (the afterglow, as particles), a slower shower, embers that skitter and die, a tongue of the wall's dust and fragments of the wall itself — each its own speed / size / life, more of
//     it into the room than back at the squad, and through a wall that held all of it back off the face.
//   · A STUN CAN'S FRAGMENTS (a can that was thrown — not a door charge borrowing the can's report): thrown the way the can lies (the axis its two smoke jets take,
//     effects.ts spawnBurst), unequal ends, a low all-round fan, a small fountain and embers, in place of 160 alike in every direction — and none of these fast
//     enough upward to clear a partition. (The dozen CPU fragments that carry the smoke trails are spawnBurst's and go as they always did.)
//
// The numbers below are SHARES AND RATIOS of the authored ones: every spark count is a share of particleFx.ts PFX.bangSparks, every spark speed / life / size a
// ratio of effects.ts sparks.speed / .life and PFX.bangSparkSize — the Tab row and a viewer retune of the stun charge's sparks move this look with them.
//
// LOOK ONLY. Nothing here is read by play: no collision, sight line, sound, hearing event, damage or dice. It takes nothing from the destruction lane's hash counters
// or rate caps and nothing from Math.random (its own counter, hashed). The particle system's own serial moves exactly as it did — ONE step a burst: the first record
// of a burst takes its seed there as the fountain's one record did, every further record brings its own. Not saved, not journaled: the set charges are re-read off
// g.wallCharge.set every frame, the few queued emits drop when the clock goes back. IT ADDS NO LIGHT: a light is read by the light meter (the renderer's probe, and
// guards.ts lightEstimateAt in its place headless), which is play — the afterglow light it could add (BLAST.glow) is written, was tried in stills, moved one
// play-through digest of twelve, and is OFF. THE ONE THING OF PLAY'S IT TOUCHES is the colour of the wall charge's flash, and of that the meter reads the luminance
// (0.2126 r + 0.7152 g + 0.0722 b), which is kept to the bit (warmOf); what is left is the meter's floor for a light too dim to count, taken on the brightest
// channel: a warm light clears it a few centimetres further out, at the very edge of its 14 m reach — under 0.002 of irradiance, for the flash's 0.14 s.
//
// WHAT IT GOES BY, since detonateStun and bangFx carry no word of who is blowing: a burst is a WALL CHARGE's when it stands at the stun point (wallcharge.ts blow():
// 0.15 m off the face, 0.5 m up, + effects.ts bang.height) of a set charge this module drew that has just left the list; its flash is the 'flash' light at that
// point + bang.lightLift with bang.lightDuration to live; a burst is a THROWN CAN's when a 'flash' item older than this frame lies bang.height under it; a
// burst is a DOOR CHARGE's (doortools.ts blow()) when that item was born this very frame and it stands where blow() puts it: 0.2 m off a door's frame line, 0.7 of the
// door's width along it. If any of those numbers moves without this file, the burst is taken for nobody's and keeps the fountain it had. One argument through detonateStun →
// bangFx would replace all of it (wallcharge.ts, doortools.ts and combat.ts are not this goal's to edit).
//
// The switch: it rides the destruction goal's (destructionSwitch.ts DESTRUCTION.on; `?nodestruction` = everything as it was) and its knob (chaos 0, the tour and the
// showcases: nothing of this). BLAST.on / .stun / .door / .glow are constants for an A/B, not rows on a panel (a door charge's throw also rides the particles' own
// 'sparks bounce off surfaces' row: with that off it is the fountain it was).
//
// Hooks (one line each): game.ts update → blastLookTick · particleFx.ts bangFx → blastSparks. wallcharge.ts, combat.ts and destruction.ts are untouched.
import { SideMap } from '../math/side';
import type { Game } from './game';
import type { Particles, EmitOpts, PKind } from '../render/particles';
import type { SetWallCharge, WallSpot } from './wallcharge';
import { Vec3, v3 } from '../math/vec';
import { DESTRUCTION } from './destructionSwitch';
import { chaosOf } from './destruction';
import { WALLCHARGE, wallPt } from './wallcharge';
import { alongOf, offOf } from './doortools';   // (called inside doorChargeAt only: nothing of doortools.ts is touched at load)
import { gpuFx, emitDust, PFX } from './particleFx';
import * as fx from './effects';

export const BLAST = {
  on: true,
  /** a thrown stun can's fragments restyled too (false: its fountain as it was; the wall charge's blast keeps its own look either way) */
  stun: true,
  /** a door charge's fragments thrown the way the blast goes (false: its all-round fountain as it was) — and only while sparks bounce off surfaces is on */
  door: true,
  /** the wall charge's AFTERGLOW LIGHT — OFF: a light is read by the light meter (the renderer's, and the CPU estimate the headless runs use — guards.ts
   *  lightEstimateAt), so it is play: with it on, one play-through digest of twelve moved. true = half a second of warm light at the hole after the flash. */
  glow: false,
};


/** what the shares and ratios below are OF, as authored when they were set (effects.ts sparks · particleFx.ts PFX): at these values the tables read in sparks,
 *  m/s, seconds and metres as written */
const REF = { sparks: 160, speed: 20, life: 0.55, size: 0.0025 };

/** THE WALL CHARGE'S BLAST. Per face: [far side (the room), near side (the squad's)]. Speeds m/s, cones rad, lives s, sizes m (a spark's size is its streak's
 *  thickness — under a pixel it is its brightness; a record's speed is its FASTEST, the jitter only slows). The cones are tilted down and narrow enough that the
 *  steepest spark of each climbs under a metre from the slab's 1.05 m (speed × sin(cone + tilt), squared, over 2 × the sparks' gravity 16): one that cleared a
 *  2.2 m partition would land in the next room and be seen beyond the wall. */
export const WALL_BLAST = {
  jet:    { count: [30, 14], speed: [21, 15], jit: 0.65, cone: [0.36, 0.5], tilt: -0.2, life: 0.24, lifeJit: 0.5, size: 0.003, sizeJit: 0.5 },
  /** THE CORE: a few big slow sparks that sit in the hole and cool on the sparks' own ramp, white to orange to red — the blast's afterglow, as particles: a LIGHT
   *  there would be read by the light meter (BLAST.glow) */
  core:   { count: [14, 7], speed: [1.6, 1.1], jit: 0.6, cone: [1.3, 1.3], tilt: 0.1, life: 0.95, lifeJit: 0.5, size: 0.04, sizeJit: 0.5 },
  shower: { count: [18, 10], speed: [10, 8], jit: 0.7, cone: [0.85, 0.95], tilt: -0.3, life: 0.55, lifeJit: 0.5, size: 0.004, sizeJit: 0.5, after: 1 },
  embers: { count: [18, 9], speed: [6, 4.5], jit: 0.85, cone: [1.1, 1.2], tilt: -0.1, life: 1.2, lifeJit: 0.8, size: 0.0035, sizeJit: 0.8, after: 2 },
  /** fragments of the wall (lit chips in its own paint, paled like a round's): they lie where they land, as a round's chips do */
  chips:  { count: [48, 22], speed: [7.5, 4.5], jit: 0.75, cone: [0.7, 0.85], tilt: 0.12, life: 20, lifeJit: 0.3, size: 0.04, sizeJit: 0.7, aspect: 1.7, after: 1 },
  /** a tongue of the wall's own dust out of each face (particle dust — none of the solver's, nothing a sight line reads) */
  dust:   { count: [44, 24], speed: [5.5, 3.6], jit: 0.6, cone: [0.55, 0.7], tilt: 0.05, life: 1.7, lifeJit: 0.3, size: 0.3, sizeJit: 0.35 },
};

/** A STUN CAN'S FRAGMENTS (149 where there were 160): two unequal lobes along the can's axis, a low fan all round, a small fountain, embers. */
export const STUN_SPARKS = {
  /** each lobe is three narrow cones fanned `fan` rad apart in the floor's plane (one wide cone would throw a third of it at the ceiling) */
  lobes:    { count: 34, speed: 17, jit: 0.6, cone: 0.17, fan: 0.3, rise: 0.05, life: 0.3, lifeJit: 0.5, size: 0.0025, sizeJit: 0.4 },
  fan:      { count: 40, speed: 10, jit: 0.7, lift: 0.12, spread: 0.5, life: 0.5, lifeJit: 0.5, size: 0.003, sizeJit: 0.4 },
  fountain: { count: 22, speed: 6, jit: 0.5, spread: 0.55, lift: 0.5, life: 0.6, lifeJit: 0.4, size: 0.003, sizeJit: 0.4 },
  embers:   { count: 18, speed: 3, jit: 0.6, spread: 0.8, lift: 0.2, life: 1.5, lifeJit: 0.5, size: 0.0045, sizeJit: 0.4 },
};

/** A DOOR CHARGE'S FRAGMENTS (156 where the all-round fountain threw 160), only while sparks come back off surfaces (render/particles.ts sparkBounce): the
 *  fountain went mostly UP, met only the ceiling and burnt out in the air, so a bounce had nothing to show. These go the two ways the blast goes, as the wall
 *  charge's do: INTO the room along the door's normal, from just ahead of the leaf the charge has thrown in (the leaf is between the burst and the room from the
 *  first frame: from the burst's own point they would all meet its back) — a flat jet whose axis meets the floor about 3.6 m in at 0.19 s (its life has to
 *  outlast that) and a wider low skitter that meets it inside two metres and bounces most — and BACK at the wall and floor the squad stands off from; then what
 *  is left of the fountain, and the stun's own embers. Those three start where the charge sat, on the face it was set on: the burst's own point is in the leaf's
 *  path, and at a fine frame step the leaf ran over a third of the slow ones. Speeds m/s (a record's FASTEST), cones rad, `tilt` the axis's rise per metre
 *  of run, lives s, sizes m. Before any bounce the steepest spark climbs about a metre over the burst. */
export const DOOR_SPARKS = {
  jet:      { count: 56, speed: 19, jit: 0.65, cone: 0.4, tilt: -0.08, life: 0.45, lifeJit: 0.5, size: 0.003, sizeJit: 0.5 },
  skitter:  { count: 44, speed: 10, jit: 0.7, cone: 0.9, tilt: -0.3, life: 0.55, lifeJit: 0.5, size: 0.004, sizeJit: 0.5 },
  back:     { count: 22, speed: 12, jit: 0.65, cone: 0.5, tilt: -0.2, life: 0.3, lifeJit: 0.5, size: 0.003, sizeJit: 0.5 },
  /** the stun's fountain, fewer (its embers are thrown as they are) */
  fountain: 16,
  /** how far ahead of the thrown leaf's face the room's share starts (m), and how many FRAMES of the leaf's own flight are added to that: the sparks are born
   *  against the leaf where this frame's step leaves it (doorSlabs runs after the blast), and a film steps a twelfth of a second at a time — with a fixed lead a
   *  film's leaf had already run over the place and took most of the jet on its back (and at 0.12 m the tipping leaf still swallowed 9 of the 156 at birth) */
  ahead: 0.2, lead: 1.5,
};

/** a set charge as this module knows it: when it was set (the lamp's clock) and how much of the firing line's run the floor in front of it has room for */
interface Seen { c: SetWallCharge; t0: number; run: number }
interface State { seen: Seen[]; queue: { at: number; run: () => void }[]; serial: number; first: boolean; last: number }
const STATE = new SideMap<Game, State>('blastLook.STATE');
const stateOf = (g: Game): State => { let s = STATE.get(g); if (!s) { s = { seen: [], queue: [], serial: 1, first: false, last: -1 }; STATE.set(g, s); } return s; };
/** this module's own dice: a counter of its own through a hash (never the lane's, never Math.random) */
const hashU = (n: number) => { let x = Math.imul(n ^ 0x9e3779b9, 0x85ebca6b) >>> 0; x ^= x >>> 15; x = Math.imul(x, 0xc2b2ae35) >>> 0; x ^= x >>> 13; return x >>> 0; };
const FRAME = 1 / 60;
const live = (g: Game) => DESTRUCTION.on && BLAST.on && chaosOf(g) > 0;
const thick = (W: WallSpot) => 2 * (W.alongX ? W.box.h[2] : W.box.h[0]);
const near = (a: Vec3, b: Vec3, m: number) => Math.abs(a[0] - b[0]) + Math.abs(a[1] - b[1]) + Math.abs(a[2] - b[2]) < m;
/** where a wall charge's borrowed stun burst stands (wallcharge.ts blow() + combat.ts detonateStun) */
const burstAt = (W: WallSpot): Vec3 => wallPt(W, 0, 0.15, 0.5 + fx.bang.height);




/** is the wall open at the charge (the blast made its hole)? one segment through it at the slab's height */
const holed = (g: Game, W: WallSpot) => !g.col.solidBlocked(wallPt(W, 0, 0.12, WALLCHARGE.height), wallPt(W, 0, -(thick(W) + 0.12), WALLCHARGE.height));



/** particleFx.ts bangFx, in place of its one spark record: true = the fragments were thrown here (a wall charge's two ways out of the wall, or a thrown can's
 *  along its axis) — false = not this module's (the switch off, or a burst that is neither): the caller throws its own as it always did */
export function blastSparks(g: Game, P: Particles, at: Vec3): boolean {
  if (!live(g)) return false;
  const S = stateOf(g), set = g.wallCharge?.set;
  const s = set ? S.seen.find(x => !set.includes(x.c) && near(burstAt(x.c.wall), at, 0.06)) : undefined;
  const door = !s && BLAST.door && P.sparkBounce && P.collide ? doorChargeAt(g, at, Math.min(0.1, (g.engine as { lastDt?: number }).lastDt ?? 1 / 30)) : null;
  if (!s && !door && !(BLAST.stun && canAt(g, at))) return false;
  S.first = true;
  if (s) wallBlast(g, S, P, s.c.wall); else if (door) doorBlast(S, P, door); else stunSparks(g, S, P, at);
  S.first = false;
  return true;
}
/** the 'flash' item a burst stands over (combat.ts detonateStun: fx.bang.height under it) — `born`: the one born this very frame, else an older one */
const flashUnder = (g: Game, at: Vec3, born: boolean) => g.items.items.some(it => it.kind === 'flash' && (it.age === 0) === born && near([it.pos[0], it.pos[1] + fx.bang.height, it.pos[2]], at, 0.02));
/** is this burst a THROWN stun can's (its item is older than this frame)? A door charge borrows the can's report through an item born the frame it blows: no
 *  can, no axis — doorChargeAt below knows it by that and by the doorway it stands in */
const canAt = (g: Game, at: Vec3) => flashUnder(g, at, false);

/** is this burst a DOOR CHARGE's (doortools.ts blow(): a 'flash' item born THIS frame, 0.2 m off the door's frame line and 0.7 of its width along it)? →
 *  `into`: the way the blast goes, off the door into the room it was blown into · `from`: where the room's share starts — ahead of the leaf the charge threw in
 *  (doorblast.ts: a loose slab), the burst's own point when the leaf fell back or stands in its frame (steel) · `rest`: where the rest starts — the burst's
 *  point mirrored onto the charge's own face of the door when the leaf went in, else the burst's point. null = not one — or a leaf flung in on its
 *  hinges (the off-hinges switch off), which sweeps through the burst: that one keeps its fountain. `dt`: the frame step (the engine's last: frames come evenly). */
function doorChargeAt(g: Game, at: Vec3, dt: number): { into: Vec3; from: Vec3; rest: Vec3 } | null {
  if (!flashUnder(g, at, true)) return null;
  for (const d of g.doors.list) {
    const W = d.def.width, along = alongOf(d, at), off = offOf(d, at, 1);
    if (along < W * 0.55 || along > W * 0.85 || Math.abs(off) < 0.16 || Math.abs(off) > 0.26) continue;
    const sg = Math.sign(off), th = d.def.closedDir, into: Vec3 = [-Math.sin(th) * sg, 0, Math.cos(th) * sg], S = d.slab;
    if (!d.off || !S) return Math.abs(d.angle) < 0.05 && Math.abs(d.vel) < 0.01 ? { into, from: at, rest: at } : null;
    const gone = (S.p[0] - at[0]) * into[0] + (S.p[2] - at[2]) * into[2], fly = Math.max(0, S.v[0] * into[0] + S.v[2] * into[2]);
    return { into, from: v3.mad(at, into, Math.max(0, gone + d.thick / 2 + DOOR_SPARKS.ahead + fly * dt * DOOR_SPARKS.lead)), rest: fly > 0 ? v3.mad(at, into, -2 * Math.abs(off)) : at };
  }
  return null;
}
function doorBlast(S: State, P: Particles, D: { into: Vec3; from: Vec3; rest: Vec3 }) {
  const K = DOOR_SPARKS, k = scale(), n = D.into;
  for (const [F, way, from] of [[K.jet, 1, D.from], [K.skitter, 1, D.from], [K.back, -1, D.rest]] as const)
    emit(S, P, 'spark', from, v3.normalize([n[0] * way, F.tilt, n[2] * way]), Math.round(F.count * k.n), { shape: 'cone', speed: F.speed * k.v, speedJit: F.jit, spread: F.cone, life: F.life * k.l, lifeJit: F.lifeJit, size: F.size * k.s, sizeJit: F.sizeJit });
  burstOf(S, P, D.rest, { ...STUN_SPARKS.fountain, count: K.fountain }, k); burstOf(S, P, D.rest, STUN_SPARKS.embers, k);
}
/** one all-round record of the stun's (its fountain, its embers) */
function burstOf(S: State, P: Particles, at: Vec3, R: typeof STUN_SPARKS.fountain, k: ReturnType<typeof scale>) {
  emit(S, P, 'spark', at, [0, 1, 0], Math.round(R.count * k.n), { shape: 'burst', speed: R.speed * k.v, speedJit: R.jit, spread: R.spread, lift: R.lift, life: R.life * k.l, lifeJit: R.lifeJit, size: R.size * k.s, sizeJit: R.sizeJit });
}

/** one record. The FIRST of a burst takes its seed off the particle system's own serial — the one step the fountain's record took, so every other burst's seed
 *  is what it was — and every later one (this frame's and the queued stages') brings a seed off this module's counter. */
function emit(S: State, P: Particles, kind: PKind, pos: Vec3, dir: Vec3, count: number, o: EmitOpts): void {
  const oo = S.first ? o : { ...o, seed: hashU(S.serial++ * 2654435761) };
  const got = kind === 'dust' ? emitDust(P, pos, dir, count, oo) : P.emit(kind, pos, dir, count, oo);
  if (got > 0) S.first = false;
}
/** the authored stun sparks as they stand, over what they were when the tables were set: count · speed · life · size */
const scale = () => ({ n: PFX.bangSparks / REF.sparks, v: fx.sparks.speed / REF.speed, l: fx.sparks.life / REF.life, s: PFX.bangSparkSize / REF.size });

type Fan = { count: number[]; speed: number[]; jit: number; cone: number[]; tilt: number; life: number; lifeJit: number; size: number; sizeJit: number; after?: number; aspect?: number };
function wallBlast(g: Game, S: State, P: Particles, W: WallSpot) {
  const B = WALL_BLAST, H = WALLCHARGE.height, open = holed(g, W), k = scale(), ONE = { n: 1, v: 1, l: 1, s: 1 };
  const n: Vec3 = W.alongX ? [0, 0, W.side] : [W.side, 0, 0];
  // the two ways out: [into the room — only through a wall that gave — , back off the face it was set on]
  const ways: { from: Vec3; d: Vec3; k: 0 | 1 }[] = [{ from: wallPt(W, 0, 0.1, H), d: n, k: 1 }];
  if (open) ways.unshift({ from: wallPt(W, 0, -(thick(W) + 0.1), H), d: v3.scale(n, -1), k: 0 });
  const pale = v3.lerp(W.box.albedo, [0.86, 0.85, 0.82], 0.45);
  const throwFan = (kind: 'spark' | 'debris' | 'dust', F: Fan, color?: Vec3) => {
    const K = kind === 'spark' ? k : ONE;
    const go = (Q: Particles) => { for (const w of ways) {
      const i = open ? w.k : 0;   // (a wall that held: everything it had comes back off the near face)
      emit(S, Q, kind, w.from, v3.normalize([w.d[0], F.tilt, w.d[2]]), Math.round(F.count[i] * K.n), { shape: 'cone', speed: F.speed[i] * K.v, speedJit: F.jit, spread: F.cone[i], life: F.life * K.l, lifeJit: F.lifeJit, size: F.size * K.s, sizeJit: F.sizeJit, posJit: 0.06, color, aspect: F.aspect });
    } };
    if (F.after) S.queue.push({ at: g.time + (F.after - 0.5) * FRAME, run: () => { const Q = gpuFx(g); if (Q) go(Q); } }); else go(P);
  };
  throwFan('spark', B.jet); throwFan('spark', B.core); throwFan('dust', B.dust, v3.scale(pale, 0.85)); throwFan('spark', B.shower); throwFan('spark', B.embers); throwFan('debris', B.chips, pale);
}

function stunSparks(g: Game, S: State, P: Particles, at: Vec3) {
  const K = STUN_SPARKS, B = fx.bang, k = scale();
  // the can's own axis and which end blows harder — effects.ts spawnBurst's draws for this burst's seed (the jets of smoke go the same two ways)
  const seed = g.fxSeed - 1; const rnd = (i: number) => { const t = Math.sin(seed * 37.31 + i * 91.7) * 43758.5453; return t - Math.floor(t); };
  const yaw = rnd(1) * Math.PI * 2, bias = (rnd(3) - 0.5) * 2 * B.asymmetry;
  const L = K.lobes;
  for (const side of [1, -1]) {
    const n = L.count * k.n * Math.max(0.25, 1 + side * bias);
    for (const [j, share, fast] of [[-1, 0.3, 0.85], [0, 0.4, 1], [1, 0.3, 0.9]]) { const a = yaw + j * L.fan;
      emit(S, P, 'spark', at, v3.normalize([Math.sin(a) * side, L.rise, Math.cos(a) * side]), Math.round(n * share), { shape: 'cone', speed: L.speed * fast * k.v, speedJit: L.jit, spread: L.cone, life: L.life * k.l, lifeJit: L.lifeJit, size: L.size * k.s, sizeJit: L.sizeJit }); }
  }
  const F = K.fan; emit(S, P, 'spark', at, [0, 1, 0], Math.round(F.count * k.n), { shape: 'ring', radius: 0.03, speed: F.speed * k.v, speedJit: F.jit, lift: F.lift, spread: F.spread, life: F.life * k.l, lifeJit: F.lifeJit, size: F.size * k.s, sizeJit: F.sizeJit });
  burstOf(S, P, at, K.fountain, k); burstOf(S, P, at, K.embers, k);
}
