// Loose furniture physics: office chairs, cardboard cartons, bins and potted plants as 3-DOF rigid bodies (x, z, yaw — they stay on
// the floor and never topple). Characters shove them out of the way (two-way: a prop that cannot yield because it is pinned against
// the static scene holds the character back instead), bullets and thrown canisters knock them, swinging door leaves sweep them aside,
// they collide with the static boxes of the collision grid (2D oriented-rectangle SAT against each box's footprint, height-filtered),
// with each other (SAT, positional correction + a small impulse) and coast to rest under per-kind ground friction, then sleep.
// Anything abrupt — a shove that sets a prop going, a knock into a wall or another prop, a bullet strike — is reported as a PropEvent
// so the game can play a scrape / thud and feed the guards' hearing. Props are deliberately NOT collision occluders: they never block
// sight, sound or the nav grid (guards path straight through and shove them, which is the point).
// Conventions: yaw as Box.yaw / level frameAt (local +X → world (cos, −sin), local +Z → (sin, cos)); angular velocity w = d(yaw)/dt, so a
// point at offset r from the centre moves at (w·rz, −w·rx) and a planar impulse J at r changes w by (rz·Jx − rx·Jz)/I ("cross2" below).
import { wallClock } from '../math/side';
import { Vec3 } from '../math/vec';
import { Box, BoxFlag, makeBox, rayBox, MKEY } from '../scene/boxes';
import { BOTH, StaticCollision } from '../scene/collision';
import { thingWrote, type BoxMirror } from '../scene/boxMirror';
import { PropDef, PropKind } from '../scene/level';
import { EXACT, mathsEpoch } from '../math/exact';   // (the kept cos / sin of a yaw is remade when the game's own maths goes on or off)
import { RUST, ruleFell, ruleRed } from '../sim/sim';
import { PropStore, PT, PD, PK, PY, PE, PP, PF, PROP_CAP, PROP_EV, PROP_SOUNDS, PROPS_STEPPED, propIo, propMem, propDiff } from '../sim/propshome';
import { SimState } from '../sim/state';
import { simRandom } from '../math/dice';   // the simulation's die (math/dice.ts): the Game's own when it has one, Math.random otherwise

/** A character as the prop solver sees it: a kinematic-ish circle whose position may be corrected (held back) in place, like doors do. */
export interface PropContact { pos: Vec3; vel: Vec3; radius: number; who: number; mass: number; quiet: boolean; }
export interface PropEvent { prop: Prop; sound: 'scrape' | 'thud' | 'roll'; pos: Vec3; who: number; level: number; speed: number;
  /** was the thing rolling on its castors AS THE NOISE WAS MADE (whoever hears it is told: the thing itself may have rolled on, or stopped, by the time he does) */
  rolling: boolean;
  /** only on sound 'roll' (a castor chair coasting, CASTOR below): 'start' as its speed passes CASTOR.startV, 'stop' as it falls under CASTOR.stopV. level is 0: nobody's ears. */
  roll?: 'start' | 'stop'; }

export interface KindTune { decel: number; visc: number; rest: number; scrapeRate: number; thudRate: number; }
/** decel: Coulomb ground friction as a deceleration (m/s²) — casters roll on, a carton on carpet stops almost at once; visc: extra viscous damping (1/s);
 *  rest: restitution against walls / other props; scrapeRate / thudRate: playback rate of the shared scrape / knock one-shots (a steel bin rings
 *  bright, a carton knocks dull). Angular friction is derived from decel and the footprint size. */
/** AN OFFICE CHAIR ROLLS ON ITS CASTORS (the maintainer: "i'd also expect the chair to have wheels and roll"). on: off in the module and on the frozen stand
 *  (TPS_CHAIRROLL=1), on in the page (?chairroll=0 = the chair as it was). With it on a prop of kind 'chair' coasts on rolling resistance `decel` (m/s²) and
 *  `visc` (1/s) in place of KIND.chair's 1.6 / 0.5 — pushed at a walk it rolls on about a metre and a half over two seconds where it stopped in half a metre —
 *  its spin dies at `spin` / radius (rad/s²), and while it coasts the system raises PropEvent sound 'roll' (start above `startV`, stop under `stopV` m/s).
 *  Every other kind, and every contact, wall and door rule, is untouched. First-pass numbers, never played. */
export const CASTOR = { on: false, decel: 0.4, visc: 0.45, spin: 1.2, startV: 0.3, stopV: 0.1 };
const KIND: Record<PropKind, KindTune> = {
  chair: { decel: 1.6, visc: 0.5, rest: 0.25, scrapeRate: 1.35, thudRate: 1.15 },
  cardboard: { decel: 3.2, visc: 0.8, rest: 0.12, scrapeRate: 1.0, thudRate: 0.8 },
  bin: { decel: 3.0, visc: 0.8, rest: 0.3, scrapeRate: 1.2, thudRate: 1.5 },
  plant: { decel: 4.5, visc: 1.0, rest: 0.1, scrapeRate: 0.8, thudRate: 0.95 },
};
const MAX_V = 5, MAX_W = 9;              // speed caps: with 1/60 s substeps a prop moves ≤ 8 cm per step, far below footprint + thinnest static panel → no tunnelling
const SLEEP_V = 0.04, SLEEP_W = 0.06, SLEEP_T = 0.3;
const DOOR_GUIDE = 1.2;                    // m/s a shoved prop is slid along the wall toward the middle of a door frame it fits through (PropSystem.doorways)
const Y_MIN = 0.04;                       // static boxes entirely below this are floor finishes, not obstacles
/** statics one gather round a thing can list (rust/sim/src/props.rs NEAR: tools/qa/rust-check.ts holds the two together) */
const NEAR = 256;

/** the cos and sin kept beside every yaw are the maths' in Math's place NOW (math/exact.ts): worked out again, all of them, when the game's own maths went
 *  on or off since they were written */
function freshTrig(S: PropStore): void {
  if (S.epoch === mathsEpoch) return; S.epoch = mathsEpoch;
  const d = S.d, O = S.O, B = S.b8; for (let r = 0; r < S.total; r++) { const y = d[B + O.yaw + r]; d[B + O.cs + r] = Math.cos(y); d[B + O.sn + r] = Math.sin(y); }
}

/** the speed caps, on a row of the columns */
function clampRow(d: Float64Array, VX: number, VZ: number, WW: number, r: number): void {
  const s = Math.hypot(d[VX + r], d[VZ + r]); if (s > MAX_V) { d[VX + r] *= MAX_V / s; d[VZ + r] *= MAX_V / s; }
  if (d[WW + r] > MAX_W) d[WW + r] = MAX_W; else if (d[WW + r] < -MAX_W) d[WW + r] = -MAX_W;
}
/** A ROW'S YAW WRITTEN — the one place it is, for the step and for `Prop.yaw` alike: its cos and sin with it, whenever it is another value than it was */
function turnRow(d: Float64Array, YAW: number, CS: number, SN: number, r: number, y: number): void {
  if (y !== d[YAW + r]) { d[CS + r] = Math.cos(y); d[SN + r] = Math.sin(y); }
  d[YAW + r] = y;
}

export class Prop {
  /** THIS THING'S OWN NUMBERS ARE COLUMNS (sim/propshome.ts PropStore: a column a number, a part of the game's block from the moment the
   *  furniture is made, where its step runs whichever side takes it): every field below that changes as the game runs is an accessor over them, by the row its
   *  id stands in, so whoever reads or writes a chair reads and writes the same values wherever they stand. (The step itself reads the columns by row,
   *  not through these: PropSystem.step.) */
  private f(o: number): number { const S = this.S; return S.d[S.b8 + o + S.rowOf(this.id)]; }
  private pf(o: number, v: number): void { const S = this.S; S.d[S.b8 + o + S.rowOf(this.id)] = v; }
  private bit(m: number): boolean { const S = this.S; return (S.u[S.b4 + S.O.flags + S.rowOf(this.id)] & m) !== 0; }
  private setBit(m: number, v: boolean): void { const S = this.S, i = S.b4 + S.O.flags + S.rowOf(this.id); if (v) S.u[i] |= m; else S.u[i] &= ~m; }
  get x(): number { return this.f(this.S.O.x); } set x(v: number) { this.pf(this.S.O.x, v); }
  get z(): number { return this.f(this.S.O.z); } set z(v: number) { this.pf(this.S.O.z, v); }
  /** its yaw — and, written with it whenever it is written another value, the cos and sin of it (`cs`, `sn`: columns beside it, for the step) */
  get yaw(): number { return this.f(this.S.O.yaw); }
  set yaw(v: number) { const S = this.S, O = S.O, B = S.b8; freshTrig(S); turnRow(S.d, B + O.yaw, B + O.cs, B + O.sn, S.rowOf(this.id), v); }
  get vx(): number { return this.f(this.S.O.vx); } set vx(v: number) { this.pf(this.S.O.vx, v); }
  get vz(): number { return this.f(this.S.O.vz); } set vz(v: number) { this.pf(this.S.O.vz, v); }
  get w(): number { return this.f(this.S.O.w); } set w(v: number) { this.pf(this.S.O.w, v); }
  readonly hx: number; readonly hz: number; readonly radius: number; readonly height: number;
  readonly mass: number; readonly invMass: number; readonly invI: number; readonly tune: KindTune;
  home: [number, number, number];         // settled rest pose for reset()
  get asleep(): boolean { return this.bit(PT.asleep); } set asleep(v: boolean) { this.setBit(PT.asleep, v); }
  get idle(): number { return this.f(this.S.O.idle); } set idle(v: number) { this.pf(this.S.O.idle, v); }
  get dirty(): boolean { return this.bit(PT.dirty); } set dirty(v: boolean) { this.setBit(PT.dirty, v); }
  /** coasting on its castors (CASTOR): between the 'roll' start and stop events; not saved — a load finds it again inside a step */
  get rolling(): boolean { return this.bit(PT.rolling); } set rolling(v: boolean) { this.setBit(PT.rolling, v); }
  // who moved it last (blamed for its noise: a man's id, a whole number), last event time (rate limit), last time a character / leaf was pushing it
  get lastWho(): number { const S = this.S; return S.u[S.b4 + S.O.who + S.rowOf(this.id)] | 0; } set lastWho(v: number) { const S = this.S; S.u[S.b4 + S.O.who + S.rowOf(this.id)] = v; }
  get noiseT(): number { return this.f(this.S.O.noiseT); } set noiseT(v: number) { this.pf(this.S.O.noiseT, v); }
  get pushT(): number { return this.f(this.S.O.pushT); } set pushT(v: number) { this.pf(this.S.O.pushT, v); }
  /** world-space part boxes, mutated in place by place() — these are what gets rendered / traced / ray tested */
  readonly boxes: Box[];
  constructor(readonly def: PropDef, readonly id: number, readonly S: PropStore) {
    this.x = def.x; this.z = def.z; this.yaw = def.yaw; this.home = [def.x, def.z, def.yaw];
    { const O = S.O, d = S.d, r = S.b8 + S.rowOf(id); d[O.cs + r] = Math.cos(def.yaw); d[O.sn + r] = Math.sin(def.yaw); }   // (of the yaw it is made at, whatever that is: a yaw of −0 is `=== 0` and would keep the cos and sin of +0)
    this.hx = def.half[0]; this.hz = def.half[1]; this.radius = Math.hypot(this.hx, this.hz); this.height = def.height;
    this.mass = def.mass; this.invMass = 1 / def.mass;
    this.invI = 1 / (def.mass * (this.hx * this.hx + this.hz * this.hz) / 3 * 1.5);   // slab inertia ×1.5: a little sluggish in yaw reads as weight and keeps corner contacts calm
    this.tune = KIND[def.kind];
    this.boxes = def.parts.map((p, j) => makeBox({ c: [0, 0, 0], h: [p.h[0], p.h[1], p.h[2]], yaw: 0, albedo: p.albedo, flags: BoxFlag.Dynamic, name: def.name, mkey: MKEY.prop + id * 16 + j, ...(p.marks?.length ? { marks: p.marks } : {}), ...(p.stuff !== undefined ? { stuff: p.stuff } : {}) }));   // (mkey: motion identity per part — scene/boxes.ts)   // (the part's markings ride on its box: the tape and glyphs on a carton turn with it)
    this.place();
  }
  /** the cos and sin of its yaw (columns beside it: written where the yaw is) */
  get cs() { freshTrig(this.S); return this.f(this.S.O.cs); } get sn() { freshTrig(this.S); return this.f(this.S.O.sn); }
  /** Refresh the world boxes from the pose. */
  place() {
    const S = this.S; freshTrig(S);
    const O = S.O, d = S.d, r = S.b8 + S.rowOf(this.id), x = d[O.x + r], z = d[O.z + r], yaw = d[O.yaw + r], cs = d[O.cs + r], sn = d[O.sn + r], parts = this.def.parts;   // (its row read once: every part is placed from the same five numbers)
    for (let i = 0; i < parts.length; i++) {
      const p = parts[i], b = this.boxes[i];
      b.c[0] = x + cs * p.off[0] + sn * p.off[2]; b.c[1] = p.off[1]; b.c[2] = z - sn * p.off[0] + cs * p.off[2]; b.yaw = yaw + p.yaw;
    }
    this.dirty = false; thingWrote();   // (the box world's copy of the furniture's parts in the Rust module is owed these: scene/boxMirror.ts things)
  }
  wake() { this.asleep = false; this.idle = 0; }
  get speed() { return Math.hypot(this.vx, this.vz); }
}

// ---- 2D oriented-rectangle SAT (scratch result, no allocation) ----
// rect = centre (x,z), yaw as cos/sin, half extents (hx along local X, hz along local Z). On overlap `sat` holds the minimum translation to apply to B
// (unit normal n from A to B, depth) and one contact point: the deepest vertex of the box whose face is NOT the separating axis (edge midpoint when an
// edge lies flat against the face, so a carton pushed square into a wall takes no spurious torque).
const sat = { nx: 0, nz: 0, depth: 0, px: 0, pz: 0 };
function satRects(ax: number, az: number, acs: number, asn: number, ahx: number, ahz: number, bx: number, bz: number, bcs: number, bsn: number, bhx: number, bhz: number): boolean {
  const dx = bx - ax, dz = bz - az;
  // axes: A.u=(acs,−asn) A.v=(asn,acs) B.u=(bcs,−bsn) B.v=(bsn,bcs)
  const cuu = Math.abs(acs * bcs + asn * bsn), cuv = Math.abs(acs * bsn - asn * bcs);   // |A.u·B.u| = |A.v·B.v|, |A.u·B.v| = |A.v·B.u|
  let best = 1e9, bnx = 0, bnz = 0, fromA = true;
  // A.u
  let d = dx * acs - dz * asn; let ov = ahx + bhx * cuu + bhz * cuv - Math.abs(d); if (ov <= 0) return false;
  if (ov < best) { best = ov; const s = d < 0 ? -1 : 1; bnx = acs * s; bnz = -asn * s; fromA = true; }
  // A.v
  d = dx * asn + dz * acs; ov = ahz + bhx * cuv + bhz * cuu - Math.abs(d); if (ov <= 0) return false;
  if (ov < best) { best = ov; const s = d < 0 ? -1 : 1; bnx = asn * s; bnz = acs * s; fromA = true; }
  // B.u
  d = dx * bcs - dz * bsn; ov = bhx + ahx * cuu + ahz * cuv - Math.abs(d); if (ov <= 0) return false;
  if (ov < best) { best = ov; const s = d < 0 ? -1 : 1; bnx = bcs * s; bnz = -bsn * s; fromA = false; }
  // B.v
  d = dx * bsn + dz * bcs; ov = bhz + ahx * cuv + ahz * cuu - Math.abs(d); if (ov <= 0) return false;
  if (ov < best) { best = ov; const s = d < 0 ? -1 : 1; bnx = bsn * s; bnz = bcs * s; fromA = false; }
  sat.nx = bnx; sat.nz = bnz; sat.depth = best;
  if (fromA) support(bx, bz, bcs, bsn, bhx, bhz, -bnx, -bnz); else support(ax, az, acs, asn, ahx, ahz, bnx, bnz);
  return true;
}
/** Farthest point of a rect along (dx,dz) into sat.px/pz; near-parallel edges resolve to their midpoint. */
function support(cx: number, cz: number, cs: number, sn: number, hx: number, hz: number, dx: number, dz: number) {
  const du = dx * cs - dz * sn, dv = dx * sn + dz * cs;
  const su = du > 0.06 ? hx : du < -0.06 ? -hx : 0, sv = dv > 0.06 ? hz : dv < -0.06 ? -hz : 0;
  sat.px = cx + cs * su + sn * sv; sat.pz = cz - sn * su + cs * sv;
}

export class PropSystem {
  /** the things in the level now, in the order they are stepped. ROW k OF THE STORE IS props[k] (sim/propshome.ts): `remove` and `reset` are the two
   *  writes of this list, and each moves the rows with it. */
  props: Prop[];
  /** every prop the level was built with, in construction order (index = Prop.id): `props` is this list minus what a blast broke apart (remove()) until reset() */
  readonly all: Prop[];
  /** every thing's own numbers and the system's clock, as columns (sim/propshome.ts): in an array of this system's, or in the game's block of the Rust module */
  readonly store: PropStore;
  enabled = true;            // off: props freeze where they are (still solid to characters)
  frictionScale = 1;
  get time(): number { return this.store.time; } set time(v: number) { this.store.time = v; }
  stats = wallClock({ awake: 0, ms: 0 }, 'ms');
  /** the statics one gather can list (rust/sim/src/props.rs NEAR is this length) */
  private near = new Int32Array(NEAR);
  private byBox = new Map<Box, Prop>();
  private muted = true;      // no events while settling / resetting
  /** THE NOISES OF THE STEP BEING TAKEN, in the order they were made: whoever hears one hears it AFTER the step (`updateHere`), as the module's step says
   *  its own after it is back (`takeBack`). One hearing order for both, and nothing a listener does — ask the Rust module and move its memory, shove a
   *  thing — can reach into a step half taken. Empty between steps. */
  private said: PropEvent[] = []; private stepping = false;
  private say(e: PropEvent): void { if (this.stepping) this.said.push(e); else this.emit(e); }
  /** THE DOOR FRAMES (Game hands them over once its doors stand: the frame's middle, the unit vector along the closed leaf, half the air between its stops).
   *  A REAL DOOR GUIDES FURNITURE THROUGH: between the stops of a real door (scene/doorSize.ts) there are 0.83 m, a chair is half a metre across and a man 0.6 —
   *  a chair shoved at the frame a hand's width off its middle caught a corner on the jamb's face and stood there, and nobody gets past a chair in a real
   *  doorway. So a prop somebody is shoving, standing at a frame it fits through but off the middle by more than the frame leaves it, is slid along the wall
   *  toward the middle (step 4b): it glances off the jamb and goes through, as a shoved chair does. A prop wider than the door, one nobody is pushing, and one
   *  whose middle is not before the opening are left alone. Level data only: no state of its own. */
  doorways: { cx: number; cz: number; ux: number; uz: number; half: number }[] = [];

  constructor(defs: PropDef[], private col: StaticCollision, private emit: (e: PropEvent) => void, st: SimState = new SimState()) {
    this.store = new PropStore(st, defs.length);
    this.all = defs.map((d, i) => new Prop(d, i, this.store)); this.props = this.all.slice();
    for (const p of this.props) for (const b of p.boxes) this.byBox.set(b, p);
    this.settle();
  }

  /** The prop that owns a rendered/traced box (e.g. from a BoxWorld.raycast hit on the dynamic list), or null. */
  propOf(b: Box): Prop | null { return this.byBox.get(b) ?? null; }
  boxes(out: Box[]) { for (const p of this.props) for (const b of p.boxes) out.push(b); }

  /** A prop leaves the level (game/furnblast.ts: it came apart beside a charge — its parts fly on as debris): no longer drawn, solid, pushed or stepped until reset(). */
  remove(p: Prop) { const i = this.props.indexOf(p); if (i >= 0) { this.props.splice(i, 1); this.store.leave(p.id); thingWrote(); } }
  /** Everything back to its rest pose, asleep. */
  reset() {
    if (this.props.length !== this.all.length) { this.props.length = 0; for (const p of this.all) this.props.push(p); this.store.homeRows(); thingWrote(); }
    for (const p of this.props) { p.x = p.home[0]; p.z = p.home[1]; p.yaw = p.home[2]; p.vx = p.vz = p.w = 0; p.asleep = true; p.idle = 0; p.noiseT = p.pushT = -10; p.lastWho = 0; p.rolling = false; p.dirty = true; p.place(); }
  }
  /** Debug: give every prop a random shove (panel button — lets you watch the solver without walking into things). */
  scatter(strength = 1.5) {
    for (const p of this.props) { const a = simRandom() * Math.PI * 2, s = strength * (0.5 + simRandom()); p.vx += Math.cos(a) * s; p.vz += Math.sin(a) * s; p.w += (simRandom() - 0.5) * 6 * strength; p.wake(); }
  }
  /** Authoring overlaps (a chair tucked 5 cm under a table edge, a bin grazing a desk corner) are resolved once, silently; the settled pose becomes home. */
  private settle() {
    this.muted = true; for (const p of this.props) p.dirty = true;   // dirty = "consider me" for the pair pass (everything starts asleep and freshly placed)
    // (twelve passes of every thing against the statics, which nothing here writes: one proof of the Rust module's copy for the lot — StaticCollision.window)
    const passes = () => { for (let it = 0; it < 12; it++) { for (let r = 0; r < this.props.length; r++) this.solveStatics(r, false); this.solvePairs(false); } };
    if (this.props.length) this.col.window(passes);
    for (const p of this.props) { p.vx = p.vz = p.w = 0; p.asleep = true; p.home = [p.x, p.z, p.yaw]; p.place(); }
    this.muted = false;
  }

  /** Closest part box of any prop along the ray (t < tmax), for bullets / thrown items; `box` = the part that was hit (a mark left on it rides that part: marks.ts). */
  raycast(ro: Vec3, rd: Vec3, tmax: number): { t: number; n: Vec3; prop: Prop; box: Box } | null {
    let best = tmax; let bp: Prop | null = null; let bn: Vec3 = [0, 1, 0]; let bb: Box | null = null;
    const fl = Math.hypot(rd[0], rd[2]), S = this.store, d = S.d, X = S.b8 + S.O.x, Z = S.b8 + S.O.z, L = this.props;
    for (let r = 0; r < L.length; r++) { const p = L[r];
      // reject on the bounding cylinder: perpendicular distance from the prop axis to the ray in XZ, and not behind / beyond the best hit
      const ox = d[X + r] - ro[0], oz = d[Z + r] - ro[2]; const along = fl > 1e-6 ? (ox * rd[0] + oz * rd[2]) / fl : 0;
      if (fl > 1e-6 && (along < -p.radius || along > best * fl + p.radius)) continue;
      const perp2 = ox * ox + oz * oz - along * along; if (perp2 > (p.radius + 0.05) * (p.radius + 0.05)) continue;
      for (const b of p.boxes) { const h = rayBox(ro, rd, b, best); if (h && h.t < best) { best = h.t; bp = p; bn = h.n; bb = b; } }
    }
    const hit = bp && bb ? { t: best, n: bn, prop: bp, box: bb } : null;
    if (BOTH.on) this.col.thingsBoth(this, ro, rd, tmax, hit);   // (the check's: the Rust module's ray over its copy of these parts, asked the same and compared)
    return hit;
  }

  /** Planar impulse J (N·s, y ignored) applied at world point `point`; `who` is blamed for the noise. thud=true reports it as an impact (bullet strike). */
  applyImpulse(p: Prop, point: Vec3, J: Vec3, who: number, thud = false) {
    const rx = point[0] - p.x, rz = point[2] - p.z;
    p.vx += J[0] * p.invMass; p.vz += J[2] * p.invMass; p.w += (rz * J[0] - rx * J[2]) * p.invI;
    this.clampVel(p); p.wake(); if (who) p.lastWho = who;
    if (thud) this.noise(p, 'thud', Math.hypot(J[0], J[2]) * p.invMass + 1, who || p.lastWho, 0.55);   // a round smacking into a carton across the room is a decent lure
  }


  private clampVel(p: Prop) { const S = this.store, O = S.O, B = S.b8; clampRow(S.d, B + O.vx, B + O.vz, B + O.w, S.rowOf(p.id)); }
  /** A castor chair's coast, as something a sound can follow (CASTOR): one 'roll' event with roll 'start' when its speed passes startV, one with 'stop' when it
   *  falls under stopV (or the switch goes off under it). Nothing is filed for anybody's ears: level 0, and Game.onPropEvent passes the kind over. */
  private rollNote(p: Prop, speed: number) {
    if (!p.rolling && speed > CASTOR.startV && !this.muted) { p.rolling = true; this.say({ prop: p, sound: 'roll', roll: 'start', pos: [p.x, 0.05, p.z], who: p.lastWho, level: 0, speed, rolling: true }); }
    else if (p.rolling && speed < CASTOR.stopV) { p.rolling = false; this.say({ prop: p, sound: 'roll', roll: 'stop', pos: [p.x, 0.05, p.z], who: p.lastWho, level: 0, speed, rolling: false }); }
  }
  private noise(p: Prop, sound: 'scrape' | 'thud', speed: number, who: number, level: number) {
    if (this.muted || this.time - p.noiseT < (sound === 'thud' ? 0.35 : 0.8)) return;   // per-prop rate limit: a chair rolled along a wall is one scrape and the odd knock, not a machine gun
    p.noiseT = this.time;
    this.say({ prop: p, sound, pos: [p.x, Math.min(0.6, p.height * 0.5), p.z], who, level: Math.min(1, level), speed, rolling: p.rolling });
  }

  /** Advance the simulation: characters as circles (positions corrected in place when a prop cannot yield), `leaves` = kinematic boxes (door leaves).
   *  THE STEP RUNS IN THE RUST MODULE where it can (`there`), on the same columns, and here otherwise; what it leaves is the same to the bit either way
   *  (tools/qa/prop-ident.ts, and whole missions in tools/qa/rust-check.ts). The boxes the picture draws are placed here, from the columns, both ways. */
  update(dt: number, contacts: PropContact[], leaves: Box[]) {
    const t0 = performance.now();
    const S = this.store, L = this.props, n = L.length;
    { const u = S.u, ID = S.b4 + S.O.id; let ok = n === S.n; for (let r = 0; ok && r < n; r++) ok = u[ID + r] === L[r].id;   // (ROW r IS props[r]: the step pairs a row's numbers with that thing's size and weight)
      if (!ok) throw new Error(`the furniture's list (${n} things) is not its rows (${S.n}) in their order: something wrote PropSystem.props that is not remove() or reset()`); }
    freshTrig(S);
    const T = this.there(contacts, leaves);
    if (T === null) { if (this.all.length) PROPS_STEPPED.here++; this.updateHere(dt, contacts, leaves); }   // (a level with no furniture has no step to count)
    else if (BOTH.on) { const bad = this.updateBoth(dt, contacts, leaves, T); BOTH.asked++; PROPS_STEPPED.asked++; if (bad) { BOTH.differ++; PROPS_STEPPED.differ++; if (!BOTH.first) BOTH.first = bad; if (!PROPS_STEPPED.first) PROPS_STEPPED.first = bad; } }
    else this.takeBack(contacts, this.handOver(dt, contacts, T));
    let awake = 0;
    { const u = S.u, FL = S.b4 + S.O.flags; for (let r = 0; r < n; r++) { const f = u[FL + r]; if (f & PT.dirty) L[r].place(); if (!(f & PT.asleep)) awake++; } }
    this.stats.awake = awake; this.stats.ms = performance.now() - t0;
  }
  /** the step as this file takes it */
  updateHere(dt: number, contacts: PropContact[], leaves: Box[]) {
    const n = Math.min(4, Math.max(1, Math.ceil(dt * 60 - 1e-6))); const h = dt / n;   // ≤ 1/60 s per substep (the frame dt is already clamped to 1/20)
    this.stepping = true;
    try { for (let s = 0; s < n; s++) { this.time += h; this.step(h, contacts, leaves); } } finally { this.stepping = false; }
    if (this.said.length) { const said = this.said; this.said = []; for (const e of said) this.emit(e); }
  }

  // ---------------------------------------------------------------- the step in the Rust module (rust/sim/src/props.rs)
  /** CAN THIS STEP RUN IN THE MODULE: the row on (the things' columns are in the game's block always); the game's own maths in Math's place (the step works
   *  its angles and lengths out there); noises being made (a system still settling its level makes none, and is not the game's yet); this file's own step
   *  not replaced on this instance (a check that puts its own over one gets ITS step); the box world's copy standing with the game's own leaves in it (or
   *  none in the level), and every leaf handed in one of them; and no more men, leaves or frames than the hand-over holds. → the copy to step over and
   *  which of its leaves sweep, or null: this file steps, on the same columns. */
  private there(contacts: PropContact[], leaves: Box[]): { M: BoxMirror; leaves: number[] | null; n: number } | null {
    if (!RUST.props || this.all.length === 0) return null;   // (a level with no loose furniture has no step, here or there: its columns are no tenant of the block)
    // (each way back to this file's step is said, for the count of questions the TypeScript took: sim.ts FELL)
    if (!EXACT.on) return null;   // (idle: a row that works lengths out answers under the game's own maths alone — sim.ts OWN_MATHS)
    if (ruleRed('props')) return null;
    if (this.muted) return ruleFell('props', 'a step while the level is still being settled');
    if (this.step !== PropSystem.prototype.step || this.updateHere !== PropSystem.prototype.updateHere) return ruleFell('props', 'a check\'s own step in the rule\'s place');
    const io = propIo(), dyn = this.col.dynamicBoxes;
    if (contacts.length > io.maxContacts || leaves.length > io.maxLeaves || this.doorways.length > io.maxWays) return ruleFell('props', 'more men, leaves or door frames than a step is handed');
    let idx: number[] | null = null;
    if (leaves !== dyn) { idx = []; for (const b of leaves) { const i = dyn.indexOf(b); if (i < 0) return ruleFell('props', 'a leaf handed in that is not in the box world'); idx.push(i); } }
    const M = this.col.rulesWorld(dyn); return M ? { M, leaves: idx, n: leaves.length } : null;
  }
  /** the step's hand-over written (the level's facts of each thing by row, the men, the leaves, the door frames, the rules' numbers) and the step taken:
   *  how many noises it listed. Nothing of the game outside the things' columns is written until `takeBack`. */
  private handOver(dt: number, contacts: PropContact[], T: { M: BoxMirror; leaves: number[] | null; n: number }): number {
    const io = propIo(), m = propMem(), L = this.props, n = L.length, nc = contacts.length, C = PROP_CAP, K = io.maxContacts, W = io.maxWays, F = this.doorways;
    for (let r = 0, o = io.def; r < n; r++, o++) { const p = L[r], t = p.tune;
      m[o + PD.hx * C] = p.hx; m[o + PD.hz * C] = p.hz; m[o + PD.radius * C] = p.radius; m[o + PD.height * C] = p.height; m[o + PD.mass * C] = p.mass; m[o + PD.invMass * C] = p.invMass; m[o + PD.invI * C] = p.invI;
      m[o + PD.decel * C] = t.decel; m[o + PD.visc * C] = t.visc; m[o + PD.rest * C] = t.rest; m[o + PD.chair * C] = p.def.kind === 'chair' ? 1 : 0; }
    for (let c = 0, o = io.con; c < nc; c++, o++) { const k = contacts[c];
      m[o + PK.x * K] = k.pos[0]; m[o + PK.z * K] = k.pos[2]; m[o + PK.vx * K] = k.vel[0]; m[o + PK.vz * K] = k.vel[2]; m[o + PK.radius * K] = k.radius; m[o + PK.who * K] = k.who; m[o + PK.mass * K] = k.mass; m[o + PK.quiet * K] = k.quiet ? 1 : 0; }
    for (let l = 0; l < T.n; l++) m[io.leaf + l] = T.leaves ? T.leaves[l] : l;
    for (let f = 0, o = io.way; f < F.length; f++, o++) { const y = F[f]; m[o + PY.cx * W] = y.cx; m[o + PY.cz * W] = y.cz; m[o + PY.ux * W] = y.ux; m[o + PY.uz * W] = y.uz; m[o + PY.half * W] = y.half; }
    const P = io.par;
    m[P + PP.dt] = dt; m[P + PP.props] = n; m[P + PP.contacts] = nc; m[P + PP.leaves] = T.n; m[P + PP.ways] = F.length;
    m[P + PP.flags] = (this.enabled ? PF.enabled : 0) | (CASTOR.on ? PF.castor : 0); m[P + PP.friction] = this.frictionScale;
    m[P + PP.casDecel] = CASTOR.decel; m[P + PP.casVisc] = CASTOR.visc; m[P + PP.casSpin] = CASTOR.spin; m[P + PP.casStart] = CASTOR.startV; m[P + PP.casStop] = CASTOR.stopV;
    const ev = io.x.props_update(this.store.home.bind(), T.M.slot);
    { const q = propMem(), S = PROPS_STEPPED; S.swept += q[P + PP.swept]; S.guided += q[P + PP.guided]; S.pairs += q[P + PP.pairs]; S.walls += q[P + PP.walls]; S.held += q[P + PP.held]; S.substeps += q[P + PP.substeps]; }
    const wrong = propMem()[P + PP.wrong]; if (wrong) throw new Error(`the furniture's step in the Rust module could not be what it was handed (${wrong === 1 ? 'more noises than its list holds' : 'more things, men, leaves or frames than its tables hold, or another count of things than its rows'})`);
    return ev;
  }
  /** …and what it did taken back into the game: where the furniture leaves each man, and the noises in the order they were made. WHOEVER HEARS ONE MAY ASK
   *  THE MODULE (and so grow its memory under a view of it), so everything the step left is read out first. */
  private takeBack(contacts: PropContact[], ev: number): void {
    const io = propIo(), m = propMem(), nc = contacts.length, K = io.maxContacts;
    for (let c = 0, o = io.con; c < nc; c++, o++) { const p = contacts[c].pos; p[0] = m[o + PK.x * K]; p[2] = m[o + PK.z * K]; }
    PROPS_STEPPED.there++; PROPS_STEPPED.noises += ev;
    if (ev === 0) return;
    const E = Array.from(m.subarray(io.ev, io.ev + ev * PROP_EV));
    for (let e = 0; e < ev; e++) this.emit(this.heard(E, e * PROP_EV));
  }
  /** a noise of the module's list as the event this file makes of one */
  private heard(E: ArrayLike<number>, o: number): PropEvent {
    const sound = PROP_SOUNDS[E[o + PE.sound]], prop = this.all[E[o + PE.prop]], pos: Vec3 = [E[o + PE.x], E[o + PE.y], E[o + PE.z]], who = E[o + PE.who], level = E[o + PE.level], speed = E[o + PE.speed], rolling = E[o + PE.rolling] !== 0;
    return sound === 'roll' ? { prop, sound, roll: E[o + PE.roll] !== 0 ? 'start' : 'stop', pos, who, level, speed, rolling } : { prop, sound, pos, who, level, speed, rolling };
  }
  /** THE STEP BOTH WAYS, COMPARED (collision.ts BOTH; tools/qa/prop-ident.ts): the module's step with nothing of it kept, then this file's from the same
   *  columns — which is the step the game goes on with — and every column, every man's place and every noise held against the other. → what differed, in
   *  words, or '' */
  updateBoth(dt: number, contacts: PropContact[], leaves: Box[], T: { M: BoxMirror; leaves: number[] | null; n: number }): string {
    const io = propIo(), S = this.store, nc = contacts.length, K = io.maxContacts;
    const was = S.bytes();
    const ev = this.handOver(dt, contacts, T);
    const m = propMem(), there = S.bytes(), con = m.slice(io.con, io.con + Object.keys(PK).length * K), E = Array.from(m.subarray(io.ev, io.ev + ev * PROP_EV));
    S.put(was);   // (the columns as they were: this file's step starts where the module's did)
    const sent: PropEvent[] = [], emit = this.emit;
    this.emit = e => { sent.push({ ...e, pos: [e.pos[0], e.pos[1], e.pos[2]] }); emit(e); };
    try { this.updateHere(dt, contacts, leaves); } finally { this.emit = emit; }
    const is = Object.is, bad: string[] = propDiff(S, S.bytes(), there, 4).map(t => `${t} (here | there)`), say = (t: string) => { if (bad.length < 4) bad.push(t); };
    for (let c = 0; c < nc; c++) { const p = contacts[c].pos; if (!is(p[0], con[PK.x * K + c]) || !is(p[2], con[PK.z * K + c])) say(`man ${c} (who ${contacts[c].who}) ends at ${p[0]}, ${p[2]} here, ${con[PK.x * K + c]}, ${con[PK.z * K + c]} there`); }
    if (sent.length !== ev) say(`${sent.length} noises here, ${ev} there`);
    for (let e = 0; e < sent.length && e < ev; e++) { const a = sent[e], b = this.heard(E, e * PROP_EV);
      if (a.prop !== b.prop || a.sound !== b.sound || a.roll !== b.roll || !is(a.pos[0], b.pos[0]) || !is(a.pos[1], b.pos[1]) || !is(a.pos[2], b.pos[2]) || !is(a.who, b.who) || !is(a.level, b.level) || !is(a.speed, b.speed) || a.rolling !== b.rolling)
        say(`noise ${e}: thing ${a.prop.id} ${a.sound} by ${a.who} at ${a.level} (${a.speed}) here, thing ${b.prop.id} ${b.sound} by ${b.who} at ${b.level} (${b.speed}) there`); }
    PROPS_STEPPED.noises += ev;
    return bad.length ? `the furniture's step: ${bad.join(' · ')}` : '';
  }

  /** One substep: integrate → characters push → prop↔prop → door leaves → statics → characters held back. Statics come last among the prop movers so
   *  nothing ends a substep inside a wall; the final character pass then concedes whatever the props could not.
   *  IT READS AND WRITES THE COLUMNS BY ROW (`d[X + r]`: sim/propshome.ts; row r is props[r], whose own fields are the level's facts of it), the same
   *  statements in the same order as when a thing's numbers were its fields. Nothing in a step asks the Rust module (its noises are said after it:
   *  `say`), so the columns' arrays asked for as it opens stand to its end. */
  private step(h: number, contacts: PropContact[], leaves: Box[]) {
    const S = this.store, O = S.O, B = S.b8, W = S.b4, props = this.props, n = props.length, time = S.time; const d = S.d, u = S.u;
    const X = B + O.x, Z = B + O.z, YAW = B + O.yaw, VX = B + O.vx, VZ = B + O.vz, WW = B + O.w, CS = B + O.cs, SN = B + O.sn, IDLE = B + O.idle, PUSH = B + O.pushT, FL = W + O.flags, WHO = W + O.who;
    if (this.enabled) {
      // 1. integrate + ground friction (Coulomb + a little viscous, linear and angular) + sleep
      for (let r = 0; r < n; r++) {
        if (u[FL + r] & PT.asleep) continue;
        const p = props[r];
        d[X + r] += d[VX + r] * h; d[Z + r] += d[VZ + r] * h; turnRow(d, YAW, CS, SN, r, d[YAW + r] + d[WW + r] * h); u[FL + r] |= PT.dirty;
        const fs = this.frictionScale; const cas = CASTOR.on && p.def.kind === 'chair';
        const dec = cas ? CASTOR.decel : p.tune.decel, vis = cas ? CASTOR.visc : p.tune.visc, spin = cas ? CASTOR.spin : 1.5 * p.tune.decel; const sp = Math.hypot(d[VX + r], d[VZ + r]);
        if (sp > 0) { const ns = Math.max(0, sp - dec * fs * h) * Math.exp(-vis * fs * h); d[VX + r] *= ns / sp; d[VZ + r] *= ns / sp; }
        const aw = Math.abs(d[WW + r]); if (aw > 0) { const na = Math.max(0, aw - spin * fs / p.radius * h) * Math.exp(-vis * fs * h); d[WW + r] *= na / aw; }
        if (cas || (u[FL + r] & PT.rolling)) { this.rollNote(p, cas ? Math.hypot(d[VX + r], d[VZ + r]) : 0); }
        if (sp < SLEEP_V && aw < SLEEP_W) { d[IDLE + r] += h; if (d[IDLE + r] > SLEEP_T) { u[FL + r] |= PT.asleep; d[VX + r] = d[VZ + r] = d[WW + r] = 0; } } else d[IDLE + r] = 0;
        if (d[YAW + r] > Math.PI * 4 || d[YAW + r] < -Math.PI * 4) turnRow(d, YAW, CS, SN, r, d[YAW + r] % (Math.PI * 2));
      }
    }
    // 2. characters push props (impulse toward the character's velocity, penetration split by mass — heavier props visibly slow you down)
    for (const c of contacts) {
      const invMc = 1 / c.mass;
      for (let r = 0; r < n; r++) {
        const p = props[r];
        const dx = c.pos[0] - d[X + r], dz = c.pos[2] - d[Z + r]; const rr = p.radius + c.radius; if (dx * dx + dz * dz > rr * rr) continue;
        if (!this.circleContact(r, c.pos[0], c.pos[2], c.radius)) continue;
        const nx = cc.nx, nz = cc.nz, pen = cc.pen;
        if (!this.enabled) { c.pos[0] += nx * pen; c.pos[2] += nz * pen; continue; }        // frozen props are just obstacles
        const fresh = time - d[PUSH + r] > 0.25; d[PUSH + r] = time;                         // a new shove (as opposed to leaning on it continuously) — only those are worth a scrape
        u[FL + r] &= ~PT.asleep; d[IDLE + r] = 0; u[WHO + r] = c.who; u[FL + r] |= PT.dirty;
        const rx = cc.px - d[X + r], rz = cc.pz - d[Z + r];
        // relative normal velocity of the prop's contact point w.r.t. the character (n points prop → character; > 0 = closing)
        const vpx = d[VX + r] + d[WW + r] * rz, vpz = d[VZ + r] - d[WW + r] * rx;
        const vn = (vpx - c.vel[0]) * nx + (vpz - c.vel[2]) * nz;
        if (vn > 0) {
          const cr = rz * nx - rx * nz; const k = p.invMass + p.invI * cr * cr + invMc; const jn = vn / k;
          d[VX + r] -= jn * nx * p.invMass; d[VZ + r] -= jn * nz * p.invMass; d[WW + r] -= jn * cr * p.invI; clampRow(d, VX, VZ, WW, r);
          const dv = jn * p.invMass;                                                          // how hard it was set going: a nudge is silent, walking pace scrapes, a sprint into a chair is loud
          if (fresh && dv > 0.7) this.noise(p, 'scrape', dv, c.who, (0.28 * dv) * (c.quiet ? 0.5 : 1));
        }
        const share = p.invMass / (p.invMass + invMc);
        d[X + r] -= nx * pen * share; d[Z + r] -= nz * pen * share; c.pos[0] += nx * pen * (1 - share); c.pos[2] += nz * pen * (1 - share);
      }
    }
    if (this.enabled) {
      // 3. prop ↔ prop
      this.solvePairs(true);
      // 4. door leaves sweep props aside (one-way and cheap: the door model never feels them; a prop shoved into a standing leaf finds it solid)
      for (const lb of leaves) {
        const lcs = Math.cos(lb.yaw), lsn = Math.sin(lb.yaw); const lr = Math.hypot(lb.h[0], lb.h[2]);
        for (let r = 0; r < n; r++) {
          const p = props[r];
          const dx = d[X + r] - lb.c[0], dz = d[Z + r] - lb.c[2]; if (dx * dx + dz * dz > (lr + p.radius) * (lr + p.radius)) continue;
          if (!satRects(lb.c[0], lb.c[2], lcs, lsn, lb.h[0], lb.h[2], d[X + r], d[Z + r], d[CS + r], d[SN + r], p.hx, p.hz)) continue;
          d[X + r] += sat.nx * sat.depth; d[Z + r] += sat.nz * sat.depth; u[FL + r] |= PT.dirty; u[FL + r] &= ~PT.asleep; d[IDLE + r] = 0; d[PUSH + r] = time; u[WHO + r] = 0;   // whoever swung the door is unknown here: any clatter that follows is 'somebody', which guards do check out
          const vn = d[VX + r] * sat.nx + d[VZ + r] * sat.nz; const want = Math.min(2, sat.depth / h * 0.5);   // carried along at about the leaf's speed (from how far it intruded this substep), never launched
          if (vn < want) { d[VX + r] += (want - vn) * sat.nx; d[VZ + r] += (want - vn) * sat.nz; }
        }
      }
      // 4b. a door frame guides a shoved prop through (doorways, above)
      for (const F of this.doorways) for (let r = 0; r < n; r++) {
        if ((u[FL + r] & PT.asleep) || time - d[PUSH + r] > 0.12 || (u[WHO + r] | 0) === 0) continue;   // (… by a body: a leaf's sweep stamps the same clock, and guiding that back at the leaf would be a motor)
        const p = props[r];
        const dx = d[X + r] - F.cx, dz = d[Z + r] - F.cz; if (dx * dx + dz * dz > (F.half + p.radius + 0.4) * (F.half + p.radius + 0.4)) continue;
        const cs = d[CS + r], sn = d[SN + r], along = dx * F.ux + dz * F.uz, off = -dx * F.uz + dz * F.ux;
        const eu = p.hx * Math.abs(cs * F.ux - sn * F.uz) + p.hz * Math.abs(sn * F.ux + cs * F.uz), en = p.hx * Math.abs(-cs * F.uz - sn * F.ux) + p.hz * Math.abs(-sn * F.uz + cs * F.ux);   // its footprint's reach along the wall and across it
        const lim = F.half - eu - 0.01; if (lim < 0 || Math.abs(along) > F.half + 0.05 || Math.abs(off) > en + 0.15) continue;   // (wider than the door · its middle not before the opening · not at the frame yet)
        const over = Math.abs(along) - lim; if (over <= 0) continue;
        if (Math.abs(-d[VX + r] * F.uz + d[VZ + r] * F.ux) < 0.5 * Math.abs(d[VX + r] * F.ux + d[VZ + r] * F.uz)) continue;   // (shoved ALONG the wall, past the door: not on its way through)
        const k = Math.min(over, DOOR_GUIDE * h) * (along > 0 ? -1 : 1); d[X + r] += F.ux * k; d[Z + r] += F.uz * k; u[FL + r] |= PT.dirty;
      }
      // 5. prop ↔ static scene (last of the prop movers, so nothing ends a substep inside a wall)
      for (let r = 0; r < n; r++) if (!(u[FL + r] & PT.asleep) || (u[FL + r] & PT.dirty)) this.solveStatics(r, true);
    }
    // 6. whatever a prop could not yield (pinned against a wall / another prop), the character gives back: pushed clear of the final prop poses.
    //    A prop found squeezed here also sheds the momentum step 2 keeps feeding it from the character's commanded velocity — otherwise a chair held
    //    against a partition by someone still walking into it would rattle between the two every substep (spin building at the corner, knock after knock)
    for (const c of contacts) for (let r = 0; r < n; r++) {
      const p = props[r];
      const dx = c.pos[0] - d[X + r], dz = c.pos[2] - d[Z + r]; const rr = p.radius + c.radius; if (dx * dx + dz * dz > rr * rr) continue;
      if (!this.circleContact(r, c.pos[0], c.pos[2], c.radius)) continue;
      c.pos[0] += cc.nx * cc.pen; c.pos[2] += cc.nz * cc.pen;
      if (cc.pen > 0.004) { d[VX + r] *= 0.3; d[VZ + r] *= 0.3; d[WW + r] *= 0.3; }
    }
  }

  /** Circle (character) vs the footprint of the prop in row r: on overlap fills `cc` with the normal (prop → circle), penetration and the contact point on the prop. */
  private circleContact(r: number, cx: number, cz: number, rad: number): boolean {
    const S = this.store, O = S.O, d = S.d, B = S.b8, p = this.props[r], px0 = d[B + O.x + r], pz0 = d[B + O.z + r];
    const cs = d[B + O.cs + r], sn = d[B + O.sn + r]; const dx = cx - px0, dz = cz - pz0;
    const lx = cs * dx - sn * dz, lz = sn * dx + cs * dz;                                  // circle centre in prop space
    const qx = lx < -p.hx ? -p.hx : lx > p.hx ? p.hx : lx, qz = lz < -p.hz ? -p.hz : lz > p.hz ? p.hz : lz;
    const ddx = lx - qx, ddz = lz - qz; const d2 = ddx * ddx + ddz * ddz; if (d2 >= rad * rad) return false;
    let nlx: number, nlz: number, pen: number;
    if (d2 > 1e-10) { const dd = Math.sqrt(d2); nlx = ddx / dd; nlz = ddz / dd; pen = rad - dd; }
    else { const px = p.hx - Math.abs(lx), pz = p.hz - Math.abs(lz); if (px < pz) { nlx = lx < 0 ? -1 : 1; nlz = 0; pen = px + rad; } else { nlx = 0; nlz = lz < 0 ? -1 : 1; pen = pz + rad; } }
    cc.nx = cs * nlx + sn * nlz; cc.nz = -sn * nlx + cs * nlz; cc.pen = pen;
    cc.px = px0 + cs * qx + sn * qz; cc.pz = pz0 - sn * qx + cs * qz;
    return true;
  }

  private solvePairs(dynamic: boolean) {
    const S = this.store, O = S.O, B = S.b8, W = S.b4, props = this.props, n = props.length, time = S.time; const d = S.d, u = S.u;
    const X = B + O.x, Z = B + O.z, VX = B + O.vx, VZ = B + O.vz, WW = B + O.w, CS = B + O.cs, SN = B + O.sn, IDLE = B + O.idle, PUSH = B + O.pushT, FL = W + O.flags, WHO = W + O.who;
    for (let i = 0; i < n; i++) {
      const a = props[i];
      for (let j = i + 1; j < n; j++) {
        const b = props[j];
        if ((u[FL + i] & PT.asleep) && (u[FL + j] & PT.asleep) && !(u[FL + i] & PT.dirty) && !(u[FL + j] & PT.dirty)) continue;
        const dx = d[X + j] - d[X + i], dz = d[Z + j] - d[Z + i]; const rr = a.radius + b.radius; if (dx * dx + dz * dz > rr * rr) continue;
        if (!satRects(d[X + i], d[Z + i], d[CS + i], d[SN + i], a.hx, a.hz, d[X + j], d[Z + j], d[CS + j], d[SN + j], b.hx, b.hz)) continue;
        const nx = sat.nx, nz = sat.nz;
        // positional correction split by inverse mass (a little slop so resting neighbours do not chatter)
        const corr = Math.max(0, sat.depth - 0.003); const wa = a.invMass / (a.invMass + b.invMass);
        d[X + i] -= nx * corr * wa; d[Z + i] -= nz * corr * wa; d[X + j] += nx * corr * (1 - wa); d[Z + j] += nz * corr * (1 - wa); u[FL + i] |= PT.dirty; u[FL + j] |= PT.dirty;
        if (!dynamic) continue;
        const rax = sat.px - d[X + i], raz = sat.pz - d[Z + i], rbx = sat.px - d[X + j], rbz = sat.pz - d[Z + j];
        const vn = ((d[VX + j] + d[WW + j] * rbz) - (d[VX + i] + d[WW + i] * raz)) * nx + ((d[VZ + j] - d[WW + j] * rbx) - (d[VZ + i] - d[WW + i] * rax)) * nz;   // < 0: approaching
        if (vn > -0.05) continue;
        const vlin = (d[VX + j] - d[VX + i]) * nx + (d[VZ + j] - d[VZ + i]) * nz;                  // linear closing speed before the impulse, for the noise below
        const who = (u[FL + i] & PT.asleep) ? u[WHO + j] | 0 : (u[FL + j] & PT.asleep) ? u[WHO + i] | 0 : (Math.hypot(d[VX + i], d[VZ + i]) > Math.hypot(d[VX + j], d[VZ + j]) ? u[WHO + i] | 0 : u[WHO + j] | 0);   // momentum hand-over carries the blame with it
        const ca = raz * nx - rax * nz, cb = rbz * nx - rbx * nz;
        const e = Math.min(a.tune.rest, b.tune.rest);
        const jn = -(1 + e) * vn / (a.invMass + b.invMass + a.invI * ca * ca + b.invI * cb * cb);
        d[VX + i] -= jn * nx * a.invMass; d[VZ + i] -= jn * nz * a.invMass; d[WW + i] -= jn * ca * a.invI;
        d[VX + j] += jn * nx * b.invMass; d[VZ + j] += jn * nz * b.invMass; d[WW + j] += jn * cb * b.invI;
        clampRow(d, VX, VZ, WW, i); clampRow(d, VX, VZ, WW, j);
        u[WHO + i] = who; u[WHO + j] = who; u[FL + i] &= ~PT.asleep; d[IDLE + i] = 0; u[FL + j] &= ~PT.asleep; d[IDLE + j] = 0;
        // audible clatter: judged on the bodies' linear closing speed (spin at a corner is not loudness), and not while a character is squeezing them together
        if (-vlin > 0.9 && time - Math.max(d[PUSH + i], d[PUSH + j]) > 0.05) this.noise(a.mass < b.mass ? a : b, 'thud', -vlin, who, 0.12 + 0.14 * -vlin);
      }
    }
  }

  /** Push the prop in row r out of the static boxes around it (up to 3 passes); with `dynamic`, reflect/friction its velocity at the contact and report knocks. */
  private solveStatics(r: number, dynamic: boolean) {
    const boxes = this.col.boxes, S = this.store, O = S.O, B = S.b8, W = S.b4, p = this.props[r]; const d = S.d, u = S.u;
    const X = B + O.x, Z = B + O.z, VX = B + O.vx, VZ = B + O.vz, WW = B + O.w, CS = B + O.cs, SN = B + O.sn, PUSH = B + O.pushT, FL = W + O.flags, WHO = W + O.who;
    for (let it = 0; it < 3; it++) {
      const R = p.radius + 0.02;
      const n = this.col.gather(d[X + r] - R, d[Z + r] - R, d[X + r] + R, d[Z + r] + R, Y_MIN, p.height, this.near);
      let any = false;
      for (let k = 0; k < n; k++) {
        const b = boxes[this.near[k]];
        const bcs = Math.cos(b.yaw), bsn = Math.sin(b.yaw);
        // circle-vs-rect early out (most gathered boxes are metres of wall nowhere near the prop)
        const dx = d[X + r] - b.c[0], dz = d[Z + r] - b.c[2]; const lx = bcs * dx - bsn * dz, lz = bsn * dx + bcs * dz;
        const ex = Math.abs(lx) - b.h[0], ez = Math.abs(lz) - b.h[2]; if (ex > p.radius || ez > p.radius || (ex > 0 && ez > 0 && ex * ex + ez * ez > p.radius * p.radius)) continue;
        if (!satRects(b.c[0], b.c[2], bcs, bsn, b.h[0], b.h[2], d[X + r], d[Z + r], d[CS + r], d[SN + r], p.hx, p.hz)) continue;
        any = true; const nx = sat.nx, nz = sat.nz;
        d[X + r] += nx * sat.depth; d[Z + r] += nz * sat.depth; u[FL + r] |= PT.dirty;
        if (!dynamic) continue;
        const rx = sat.px - d[X + r], rz = sat.pz - d[Z + r];
        const vpx = d[VX + r] + d[WW + r] * rz, vpz = d[VZ + r] - d[WW + r] * rx; const vn = vpx * nx + vpz * nz;   // < 0: driving into the wall
        if (vn >= 0) continue;
        const vlin = d[VX + r] * nx + d[VZ + r] * nz;                                         // the body's own closing speed, for the noise below (before the impulse changes it)
        const cr = rz * nx - rx * nz; const kn = p.invMass + p.invI * cr * cr; const jn = -(1 + p.tune.rest) * vn / kn;
        d[VX + r] += jn * nx * p.invMass; d[VZ + r] += jn * nz * p.invMass; d[WW + r] += jn * cr * p.invI;
        // Coulomb friction along the wall (μ 0.4): a carton dragged along a partition slows, a chair glancing off a jamb spins away
        const tx = -nz, tz = nx; const vt = vpx * tx + vpz * tz;
        if (Math.abs(vt) > 1e-4) { const ct = rz * tx - rx * tz; const kt = p.invMass + p.invI * ct * ct; let jt = -vt / kt; const lim = 0.4 * jn; if (jt > lim) jt = lim; else if (jt < -lim) jt = -lim; d[VX + r] += jt * tx * p.invMass; d[VZ + r] += jt * tz * p.invMass; d[WW + r] += jt * ct * p.invI; }
        clampRow(d, VX, VZ, WW, r);
        // a knock worth hearing: something coasting into the wall on its own (linear speed; a pinned prop's corner spin is not an impact) — not a prop
        // somebody is walking into the wall right now (that shove already made its scrape, and a sprint makes it a loud one)
        if (-vlin > 0.8 && S.time - d[PUSH + r] > 0.05) this.noise(p, 'thud', -vlin, u[WHO + r] | 0, 0.1 + 0.15 * -vlin);
      }
      if (!any) break;
    }
  }
}
// scratch for circleContact
const cc = { nx: 0, nz: 0, pen: 0, px: 0, pz: 0 };
