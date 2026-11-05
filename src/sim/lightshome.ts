// THE LEVEL'S LIGHTS, FROM TYPESCRIPT — the thin caller of rust/sim/src/lights.rs. Every fixture's state (on, broken, blinded, what a hand left, the factor it
// burns at), the mains, the lamps fixtures feed and a camera's and a tripwire's own machine are the `lights` part of the game's block (sim/state.ts). This file
// is how the game reads them, and the one way it writes them: `LightHome.in`, a kind and a fixture. What the game makes of them (words, men sent, the marks on
// the debrief) is game/lighting.ts's and game/devices.ts's, on what the calls here say.
//
// A READ IS A READ OF THE BLOCK (`Fixture.on`, `.factor`; `Mains.active`; `CameraMachine.seen`): no call into the module, and read-only to the compiler. Once a
// tick `paint` hands what the module worked out to what the picture reads, the engine's RtLight list: the module's rows are the truth, RtLight its copy.
//
// (Imports sim.ts and state.ts alone at run time: like the rest of src/sim, this is below the game.)
import { sim, type SimExports } from './sim';
import { simPart, simTenant, P_LIGHTS, type SimState } from './state';
import type { Vec3 } from '../math/vec';
import type { RtLight } from '../render/lights';

/** THE COLUMNS OF THE PART in rust/sim/src/lights.rs COLS' order. Where each stands is the module's word, read once. */
export const LIGHT_COLS = ['n', 'rows', 'beacons', 'cameras', 'lasers', 'mains', 'since', 'until', 'restored_at', 'fix_t', 'blind_until', 'factor', 'stagger', 'base', 'hand', 'bits',
  'x', 'y', 'z', 'range', 'radius', 'kind', 'dx', 'dy', 'dz', 'outer', 'inner', 'r', 'g', 'b', 'intensity', 'sx', 'sy', 'sz', 's_radius', 's_intensity', 'row_base', 'row_bits', 'row_target',
  'beacon_phase', 'beacon_row', 'beacon_target', 'cam_yaw', 'cam_phase', 'cam_seen', 'cam_reported', 'cam_lx', 'cam_ly', 'cam_lz', 'cam_rest', 'cam_a', 'cam_b', 'cam_period', 'cam_pitch', 'cam_px', 'cam_py', 'cam_pz',
  'cam_state', 'cam_bits', 'cam_target', 'cam_row', 'las_last', 'las_cut', 'las_sense', 'las_trips', 'las_bits', 'las_target'] as const;
type Col = (typeof LIGHT_COLS)[number];
/** how a column is reached in the block: `st.d[C.factor + i]` for a column of doubles, `st.w[C.bits + i]` for one of words (the part's own place included) */
let C: Record<Col, number> = null as unknown as Record<Col, number>;
let X: SimExports = null as unknown as SimExports;
export const WORD_COLS: ReadonlySet<string> = new Set<string>(['n', 'rows', 'beacons', 'cameras', 'lasers', 'mains', 'bits', 'row_bits', 'row_target', 'beacon_row', 'beacon_target', 'cam_state', 'cam_bits', 'cam_target', 'cam_row', 'las_trips', 'las_bits', 'las_target']);
/** what the part holds (lights.rs TARGETS, ROWS, BEACONS, CAMERAS, LASERS: game/levelfit.ts refuses a level over one of these, by its name) */
export const LIGHT_CAPS = { targets: 256, rows: 256, beacons: 32, cameras: 32, lasers: 32 } as const;
function columns(): void {
  if (C) return;
  const x = sim(), base = simPart(P_LIGHTS), o = {} as Record<Col, number>;
  if (x.lights_cap(14) !== LIGHT_COLS.length || x.lights_cap(0) !== LIGHT_CAPS.targets || x.lights_cap(1) !== LIGHT_CAPS.rows || x.lights_cap(2) !== LIGHT_CAPS.beacons || x.lights_cap(3) !== LIGHT_CAPS.cameras || x.lights_cap(4) !== LIGHT_CAPS.lasers)
    throw new Error(`the Rust module lays the lights out as ${x.lights_cap(14)} columns and src/sim/lightshome.ts as ${LIGHT_COLS.length}: bun run tools/sim/build.ts`);
  LIGHT_COLS.forEach((k, i) => { const at = base + x.lights_cap(100 + i); o[k] = WORD_COLS.has(k) ? at >>> 2 : at >>> 3; });
  C = o; X = x;
}
/** a fixture's truths (lights.rs T_*), a row's (R_*), a camera's (C_*), a tripwire's (L_*), the mains' (M_*) */
const T = { on: 1, broken: 2, flicker: 4, mains: 8, hand: 16, authored: 32, emissive: 64 } as const;
export const ROW_BIT = { on: 1, passing: 2, screen: 4, role: 24, own: 0, area: 8, beacon: 16, cone: 24, steady: 32 } as const;
const CB = { sweep: 1, last: 2, saw: 4, dead: 8 } as const;
const LB = { dead: 1, held: 2 } as const;
export const MAINS = { active: 1, permanent: 2 } as const;
/** every event from outside, by kind (lights.rs IN_*) */
export const LIGHT_IN = { on: 0, break: 1, mend: 2, toggle: 3, blind: 4, hand: 5, authored: 6, base: 7, put: 8, trip: 9, back: 10, mains: 11, fix: 12, fresh: 13, stagger: 14 } as const;
/** what a tick of the lights says happened (lights.rs EV_*): the mains' timed trip ran out · a man's hand is at the panel · he has reset it */
export const LIGHT_EV = { ranOut: 1, hand: 2, fixed: 4 } as const;
/** what a camera's two halves say (CAM_*) and a tripwire's (LASER_*) */
export const CAM = { ok: 1, down: 2, suspect: 4, alarm: 8 } as const;
export const LASER = { ok: 1, down: 2, trip: 4 } as const;
/** a camera's state by its number in the module */
export const CAMERA_STATES = ['idle', 'suspect', 'alarm'] as const;
export type CameraState = (typeof CAMERA_STATES)[number];

/** A FIXTURE AS IT IS MADE: game.ts, devices.ts, carlamps.ts say these; `LightHome.add` takes the state into the block. */
export interface FixtureDef {
  name: string; group: string; kind: 'analytic' | 'emissive' | 'breaker' | 'device';   // device: a camera or a tripwire's emitter (devices.ts) — no boxes or light of its own here, it draws itself off `factor`; the OCP, a round and the mains treat it as any other
  pos: Vec3;                       // fixture position (for targeting / AI)
  light?: RtLight; boxes: number[]; baseEmissive: Vec3; baseIntensity: number;
  on: boolean; broken: boolean; disabledUntil: number; fluorescentFlicker: boolean;
  factor: number;
  mains: boolean;                  // fed from the breaker (drops in a blackout); false for street lighting, exit signs, UPS-fed LEDs, beacons
  interactive: boolean;            // can be aimed at / OCP'd / shot as a fixture (server LEDs are scenery: too many, too small, pure UI noise)
  stagger: number;                 // restrike delay for this fixture's contactor group when the mains come back
  areaLights?: { light: RtLight; base: number }[];   // analytic area lights standing in for a flat emissive fixture's emission (smooth direct light + soft shadows instead of cascade rays finding a small bright box)
  /** a vehicle lamp's own paint as the level has it (carlamps.ts: a broken one is painted dark, a mended one painted this again) */
  lens?: Vec3;
}
/** A FIXTURE OF A GAME: what the level says of it, as plain fields, and its state, read from the block (never written here: `LightHome.in`). */
export class Fixture {
  name: string; group: string; kind: FixtureDef['kind']; pos: Vec3; light?: RtLight; boxes: number[]; baseEmissive: Vec3; fluorescentFlicker: boolean; mains: boolean; interactive: boolean; areaLights?: { light: RtLight; base: number }[]; lens?: Vec3;
  constructor(readonly i: number, private readonly st: SimState, def: FixtureDef) {
    this.name = def.name; this.group = def.group; this.kind = def.kind; this.pos = def.pos; this.boxes = def.boxes; this.baseEmissive = def.baseEmissive; this.fluorescentFlicker = def.fluorescentFlicker; this.mains = def.mains; this.interactive = def.interactive;
    if (def.light) this.light = def.light;
    if (def.areaLights) this.areaLights = def.areaLights;
    if (def.lens) this.lens = def.lens;
  }
  /** its switch */
  get on(): boolean { return (this.st.w[C.bits + this.i] & T.on) !== 0; }
  get broken(): boolean { return (this.st.w[C.bits + this.i] & T.broken) !== 0; }
  /** an OCP's blind runs to here, and a strike's restrike from here (−1: neither) */
  get disabledUntil(): number { return this.st.d[C.blind_until + this.i]; }
  /** what it burns at this tick, 0‥1: the one number on, broken, blinded, the mains and a flicker come to */
  get factor(): number { return this.st.d[C.factor + this.i]; }
  /** the game time a hand put it out at its wall switch (lightswitch.ts; undefined: no hand did) */
  get handOff(): number | undefined { return (this.st.w[C.bits + this.i] & T.hand) !== 0 ? this.st.d[C.hand + this.i] : undefined; }
  /** on in the level as authored (lightswitch.ts: what 'lights on' puts a room back to) */
  get authored(): boolean { return (this.st.w[C.bits + this.i] & T.authored) !== 0; }
  get baseIntensity(): number { return this.st.d[C.base + this.i]; }
  get stagger(): number { return this.st.d[C.stagger + this.i]; }
}
/** THE MAINS, read (`Game.blackout`) */
export class Mains {
  constructor(private readonly st: SimState) {}
  get active(): boolean { return (this.st.w[C.mains] & MAINS.active) !== 0; }
  get permanent(): boolean { return (this.st.w[C.mains] & MAINS.permanent) !== 0; }
  get until(): number { return this.st.d[C.until]; }
  get since(): number { return this.st.d[C.since]; }
  get restoredAt(): number { return this.st.d[C.restored_at]; }
  /** how long a man of theirs has been at the panel */
  get fixT(): number { return this.st.d[C.fix_t]; }
  /** its numbers as a plain record (a check's copy; JSON of it), never the block */
  toJSON(): { active: boolean; until: number; since: number; permanent: boolean; restoredAt: number } { return { active: this.active, until: this.until, since: this.since, permanent: this.permanent, restoredAt: this.restoredAt }; }
}
/** A CAMERA'S OWN MACHINE, read (devices.ts CameraRt is this and the level's facts of it) */
export class CameraMachine {
  constructor(readonly slot: number, protected readonly st: SimState) {}
  /** the lens's yaw this frame (pans between its sweep) and the pan's phase 0‥1 */
  get yaw(): number { return this.st.d[C.cam_yaw + this.slot]; }
  get phase(): number { return this.st.d[C.cam_phase + this.slot]; }
  /** the integrator: lit-seconds of Sam in the picture */
  get seen(): number { return this.st.d[C.cam_seen + this.slot]; }
  /** idle → suspect (control has radioed, a man is on his way) → alarm (the floor went up); back to idle once drained and re-armed */
  get state(): CameraState { return CAMERA_STATES[this.st.w[C.cam_state + this.slot]]; }
  get reportedAt(): number { return this.st.d[C.cam_reported + this.slot]; }
  /** where the picture last had him (the errand's goal, the alarm's fix) */
  get lastSeen(): Vec3 | null { const s = this.slot, d = this.st.d; return (this.st.w[C.cam_bits + s] & CB.last) !== 0 ? [d[C.cam_lx + s], d[C.cam_ly + s], d[C.cam_lz + s]] : null; }
  get sawThisFrame(): boolean { return (this.st.w[C.cam_bits + this.slot] & CB.saw) !== 0; }
  /** a dead camera has been called in (once per death) */
  get deadCalled(): boolean { return (this.st.w[C.cam_bits + this.slot] & CB.dead) !== 0; }
  /** …all of it as a file has it, or a fresh encounter (devices.ts) */
  put(yaw: number, phase: number, seen: number, state: CameraState, reportedAt: number, lastSeen: Vec3 | null, deadCalled: boolean): void {
    const s = this.slot, d = this.st.d, w = this.st.w;
    d[C.cam_yaw + s] = yaw; d[C.cam_phase + s] = phase; d[C.cam_seen + s] = seen; d[C.cam_reported + s] = reportedAt; w[C.cam_state + s] = CAMERA_STATES.indexOf(state);
    d[C.cam_lx + s] = lastSeen ? lastSeen[0] : 0; d[C.cam_ly + s] = lastSeen ? lastSeen[1] : 0; d[C.cam_lz + s] = lastSeen ? lastSeen[2] : 0;
    w[C.cam_bits + s] = (w[C.cam_bits + s] & CB.sweep) | (lastSeen ? CB.last : 0) | (deadCalled ? CB.dead : 0);
  }
}
/** A TRIPWIRE'S OWN MACHINE, read (devices.ts LaserRt) */
export class LaserMachine {
  constructor(readonly slot: number, protected readonly st: SimState) {}
  get lastTripAt(): number { return this.st.d[C.las_last + this.slot]; }
  get trips(): number { return this.st.w[C.las_trips + this.slot]; }
  /** drawn length this frame (a leaf, a body, a chair in the beam cuts the picture of it short) */
  get cut(): number { return this.st.d[C.las_cut + this.slot]; }
  /** how far the SENSOR's beam reaches this frame */
  get sense(): number { return this.st.d[C.las_sense + this.slot]; }
  get deadCalled(): boolean { return (this.st.w[C.las_bits + this.slot] & LB.dead) !== 0; }
  /** something is breaking it right now (the trip fires on the edge) */
  get held(): boolean { return (this.st.w[C.las_bits + this.slot] & LB.held) !== 0; }
  /** …the machine's own as a file has it, or a fresh encounter (devices.ts) */
  put(lastTripAt: number, trips: number, deadCalled: boolean, held: boolean): void {
    const s = this.slot, w = this.st.w; this.st.d[C.las_last + s] = lastTripAt; w[C.las_trips + s] = trips; w[C.las_bits + s] = (deadCalled ? LB.dead : 0) | (held ? LB.held : 0);
  }
}
/** a beacon as the rows take it */
interface BeaconFact { light: RtLight; phase: number; target: Fixture }

/** ONE GAME'S LIGHTS. `Game.lightHome`. */
export class LightHome {
  readonly mains: Mains;
  /** the lamp each row is, in the list's order: what `paint` copies to */
  private lamps: RtLight[] = [];
  constructor(private readonly st: SimState) {
    columns(); st.bind(); st.w[0 + 1] |= LIGHTS_BIT; this.mains = new Mains(st); this.in(LIGHT_IN.fresh, null);   // (word 1 of the block: the head's homes, state.rs Head)
  }
  /** A FIXTURE MORE (the level as it is built; the kit picture's own lamp) */
  add(def: FixtureDef): Fixture {
    const st = this.st, n = st.w[C.n];
    if (n >= LIGHT_CAPS.targets) throw new Error(`a level of more than ${LIGHT_CAPS.targets} light fixtures got past game/levelfit.ts`);
    st.w[C.n] = n + 1;
    st.w[C.bits + n] = (def.on ? T.on : 0) | (def.broken ? T.broken : 0) | (def.fluorescentFlicker ? T.flicker : 0) | (def.mains ? T.mains : 0) | (def.kind === 'emissive' ? T.emissive : 0);
    st.d[C.blind_until + n] = def.disabledUntil; st.d[C.factor + n] = def.factor; st.d[C.stagger + n] = def.stagger; st.d[C.base + n] = def.baseIntensity;
    return new Fixture(n, st, def);
  }
  /** A CAMERA MORE, at rest (`sweep`: its two yaws, none = it does not pan): its machine, and the level's facts of it (where, how it looks) */
  camera(target: Fixture, rest: number, sweep: readonly [number, number] | undefined, period: number, pitch: number, pos: Vec3): number {
    const st = this.st, s = st.w[C.cameras];
    if (s >= LIGHT_CAPS.cameras) throw new Error(`a level of more than ${LIGHT_CAPS.cameras} cameras got past game/levelfit.ts`);
    st.w[C.cameras] = s + 1; st.w[C.cam_target + s] = target.i; st.w[C.cam_bits + s] = sweep ? CB.sweep : 0;
    st.d[C.cam_rest + s] = rest; st.d[C.cam_a + s] = sweep ? sweep[0] : 0; st.d[C.cam_b + s] = sweep ? sweep[1] : 0; st.d[C.cam_period + s] = period; st.d[C.cam_pitch + s] = pitch;
    st.d[C.cam_px + s] = pos[0]; st.d[C.cam_py + s] = pos[1]; st.d[C.cam_pz + s] = pos[2]; st.d[C.cam_yaw + s] = rest; st.d[C.cam_reported + s] = -100;
    return s;
  }
  /** A TRIPWIRE MORE */
  laser(target: Fixture, len: number): number {
    const st = this.st, s = st.w[C.lasers];
    if (s >= LIGHT_CAPS.lasers) throw new Error(`a level of more than ${LIGHT_CAPS.lasers} tripwires got past game/levelfit.ts`);
    st.w[C.lasers] = s + 1; st.w[C.las_target + s] = target.i; st.d[C.las_last + s] = -100; st.d[C.las_cut + s] = len; st.d[C.las_sense + s] = len;
    return s;
  }
  /** THE ROWS, STOOD ONCE as the level is built, in the list's order: every lamp a fixture feeds gets one (`RtLight.row`), and no other lamp has a row. A row's index IS its place among the rows: a lamp of the question's own is written with how many rows stand before it in the list. */
  stand(list: readonly RtLight[], targets: readonly Fixture[], beacons: readonly BeaconFact[], cones: readonly { light: RtLight | null; target: Fixture; slot: number }[]): void {
    const st = this.st, d = st.d, w = st.w;
    const who = new Map<RtLight, { role: number; t: Fixture; base: number }>();
    for (const t of targets) { if (t.light) who.set(t.light, { role: ROW_BIT.own, t, base: 0 }); for (const a of t.areaLights ?? []) who.set(a.light, { role: ROW_BIT.area, t, base: a.base }); }
    for (const b of beacons) who.set(b.light, { role: ROW_BIT.beacon, t: b.target, base: b.light.peakIntensity });
    for (const c of cones) if (c.light) who.set(c.light, { role: ROW_BIT.cone, t: c.target, base: 0 });
    let r = 0; this.lamps = [];
    for (const l of list) {
      l.row = undefined; const W = who.get(l); if (!W || l.owner) continue;   // (a lamp with an owner gets no row: `skipOwner` is the question's, and only the question's own lamps have one)
      if (r >= LIGHT_CAPS.rows) throw new Error(`a level of more than ${LIGHT_CAPS.rows} lamps on its fixtures got past game/levelfit.ts`);
      l.row = r; this.lamps.push(l);
      d[C.row_base + r] = W.base; w[C.row_target + r] = W.t.i;
      w[C.row_bits + r] = W.role | (W.role === ROW_BIT.beacon ? ROW_BIT.passing : 0) | (W.role === ROW_BIT.area && W.t.group === 'monitors' ? ROW_BIT.screen : 0);
      this.feed(l); r++;
    }
    w[C.rows] = r;
    const rowOf = (l: RtLight): number => { if (l.row === undefined) throw new Error(`the lamp '${l.name}' has an owner or is not in the list, and the module writes it: a beacon's spot and a camera's cone must each have a row`); return l.row; };
    beacons.forEach((b, i) => { d[C.beacon_phase + i] = b.phase; w[C.beacon_row + i] = rowOf(b.light); w[C.beacon_target + i] = b.target.i; });
    w[C.beacons] = beacons.length;
    for (const c of cones) if (c.light) w[C.cam_row + c.slot] = rowOf(c.light);
  }
  /** A LAMP'S WHOLE RECORD INTO ITS ROW, as the lamp stands: as the rows are stood, and each tick for a lamp somebody else moves (a fire's, game/fire.ts). A steady self
   *  (`RtLight.steady`) has the row's bit up and its columns; none leaves them zeros, and a reader takes the first self. Nothing for a lamp with no row. */
  feed(l: RtLight): void {
    const r = l.row; if (r === undefined) return;
    const st = this.st, d = st.d, w = st.w, s = l.steady;
    d[C.x + r] = l.pos[0]; d[C.y + r] = l.pos[1]; d[C.z + r] = l.pos[2]; d[C.range + r] = l.range; d[C.radius + r] = l.radius; d[C.kind + r] = l.kind;
    d[C.dx + r] = l.dir[0]; d[C.dy + r] = l.dir[1]; d[C.dz + r] = l.dir[2]; d[C.outer + r] = l.outerDeg; d[C.inner + r] = l.innerDeg;
    d[C.r + r] = l.color[0]; d[C.g + r] = l.color[1]; d[C.b + r] = l.color[2]; d[C.intensity + r] = l.intensity;
    d[C.sx + r] = s ? s.pos[0] : 0; d[C.sy + r] = s ? s.pos[1] : 0; d[C.sz + r] = s ? s.pos[2] : 0; d[C.s_radius + r] = s ? s.radius : 0; d[C.s_intensity + r] = s ? s.intensity : 0;
    w[C.row_bits + r] = (w[C.row_bits + r] & ~(ROW_BIT.on | ROW_BIT.steady)) | (l.enabled ? ROW_BIT.on : 0) | (s ? ROW_BIT.steady : 0);
  }

  /** …of a lamp somebody moves each tick (a fire's): only while it burns. Out, its record is the picture's alone (the fog keeps a fire's lamp lit where the team has not seen it go, fire.ts `held`), no question reads a row that is out, and what the picture does stays out of the block. */
  feedBurning(l: RtLight): void {
    if (l.enabled) { this.feed(l); return; }
    const r = l.row; if (r !== undefined) this.st.w[C.row_bits + r] &= ~ROW_BIT.on;
  }

  /** EVERY EVENT FROM OUTSIDE (lights.rs input): a kind, a fixture where it has one, its numbers. → the kind's own answer. */
  in(kind: number, t: Fixture | null, a = 0, b = 0, c = 0, d = 0): number {
    return X.lights_in(this.st.bind(), kind, t ? t.i : 0, a, b, c, d);
  }
  /** …as a check stages a fixture by hand: its switch, whether it is broken, blind until */
  stage(t: Fixture, on: boolean | null, broken: boolean | null, until: number | null = null): void {
    if (on !== null) this.in(LIGHT_IN.on, t, on ? 1 : 0);
    if (broken !== null) this.in(broken ? LIGHT_IN.break : LIGHT_IN.mend, t);
    if (until !== null) this.in(LIGHT_IN.blind, t, until);
  }
  /** A WALL SWITCH'S CIRCUIT flipped (lights.rs flip) */
  flip(targets: readonly Fixture[], on: boolean, time: number, strike: boolean): void {
    const slot = this.st.bind(), ids = new Uint32Array(X.memory.buffer, X.lights_ids(), targets.length);
    for (let i = 0; i < targets.length; i++) ids[i] = targets[i].i;
    X.lights_flip(slot, on ? 1 : 0, time, strike ? 1 : 0, targets.length);
  }
  /** ONE TICK (lights.rs step). → LIGHT_EV bits. */
  step(time: number, dt: number, near: boolean, fumble: number, emissiveScale: number): number {
    return X.lights_step(this.st.bind(), time, dt, near ? 1 : 0, fumble, emissiveScale);
  }
  /** WHAT THE MODULE WORKED OUT, TO WHAT THE PICTURE READS: a lamp's burning, and a beacon's turn */
  paint(): void {
    const st = this.st, d = st.d, w = st.w, L = this.lamps;
    for (let r = 0; r < L.length; r++) {
      const b = w[C.row_bits + r], role = b & ROW_BIT.role; if (role === ROW_BIT.cone) continue;
      const l = L[r], i = d[C.intensity + r]; l.enabled = (b & ROW_BIT.on) !== 0; l.intensity = i;
      if (role === ROW_BIT.area) l.peakIntensity = i;
      else if (role === ROW_BIT.beacon) l.dir = [d[C.dx + r], d[C.dy + r], d[C.dz + r]];
    }
  }
  /** a camera before its picture (lights.rs camera_pan), and its cone to what the picture reads. → CAM bits. */
  cameraPan(slot: number, cone: RtLight | null, dt: number, live: boolean, cones: boolean): number {
    const out = X.camera_pan(this.st.bind(), slot, dt, live ? 1 : 0, cones ? 1 : 0);
    if (cone && cone.row !== undefined) { const st = this.st, d = st.d, r = cone.row; cone.enabled = (st.w[C.row_bits + r] & ROW_BIT.on) !== 0; cone.pos = [d[C.x + r], d[C.y + r], d[C.z + r]]; cone.dir = [d[C.dx + r], d[C.dy + r], d[C.dz + r]]; }
    return out;
  }
  /** …and after it (lights.rs camera_see). → CAM bits. */
  cameraSee(slot: number, time: number, dt: number, visible: boolean, gain: number, p: Vec3, suspicion: number, alarm: number): number {
    return X.camera_see(this.st.bind(), slot, time, dt, visible ? 1 : 0, gain, p[0], p[1], p[2], suspicion, alarm);
  }
  /** a tripwire before what is in its beam (lights.rs laser_open). → LASER bits. */
  laserOpen(slot: number): number {
    return X.laser_open(this.st.bind(), slot);
  }
  /** …and after it (lights.rs laser_close). → LASER bits. */
  laserClose(slot: number, time: number, live: boolean, cut: number, sense: number, held: boolean): number {
    return X.laser_close(this.st.bind(), slot, time, live ? 1 : 0, cut, sense, held ? 1 : 0);
  }
  /** where a column stands in the block, for a check (`st.d` or `st.w` by WORD_COLS) */
  col(name: Col): number { return C[name]; }
  /** the block, for a check */
  get block(): SimState { return this.st; }
}
// A game's lights have no home but the block, from the game's first line to its last: the tenant's wish is always up, and so a block with a game's lights in it is never shut (state.ts settle)
const LIGHTS_BIT = 1024;
simTenant({ name: 'lights', bit: LIGHTS_BIT, wants: () => true, into: () => {}, outOf: () => {} });

