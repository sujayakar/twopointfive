// THE LIGHTS' STATE, THE MODULE AGAINST THE TYPESCRIPT IT REPLACED — rust/sim/src/lights.rs on every level's fixtures, and on made-up ones, with the arithmetic of the
// TypeScript's own lighting.ts, lightswitch.ts and devices.ts kept here statement for statement (`Twin`, which goes with the deletion). Every call of the one
// way in (sim/lightshome.ts LIGHTS_WATCH) is put to the twin too, and after each call the two must agree to the bit: every column of the part, and what the call
// answered. So the whole of it: each fixture's on, broken, blind, hand and factor, the mains' clocks, each lamp's row, a camera's and a tripwire's machine, and every event out.
//   bun run tools/qa/lights-ident.ts [--levels office.json,lights-out.json] [--ticks 2400] [--seed 1]
// Three parts:
//   made-up  fixtures and machines of the tool's own, every event kind on a clock and made-up men in front of the machines, no Game's tick: what it folds to is a word
//            per seed kept in tools/qa/fixtures/lights-ident.json (`--write` writes it), which outlives the twin and which no level's edit moves.
//   levels   a whole game on each level, men and all, every event from the game's own functions on a clock (a round's and a blast's break, the OCP, the mains tripped
//            and thrown and shot, groups, the panel's toggle and repair, a wall switch both ways, the show's day, the hour, a save and a load, a restart, Sam walked
//            through a camera's picture and a beam). A kind of call the levels together never met is a FAIL.
//   rows     after every tick of those games, each lamp with a row: its record in the part is the record the renderer's list holds, to the bit. A row left a tick
//            stale is red here before any question reads it.
// LIGHTS_BREAK=1 is the red path: the twin's restrike threshold one part in a thousand off, and a row written a millimetre out once. The run must FAIL.
import { resolve } from 'path';
import { standUp } from './headless.ts';
const ROOT = resolve(import.meta.dir, '../..');
const H: any = await import(`${ROOT}/src/sim/lightshome.ts`), ST: any = await import(`${ROOT}/src/sim/state.ts`), SIM: any = await import(`${ROOT}/src/sim/sim.ts`);
const { loadLevel } = await import(`${ROOT}/src/scene/levelLoad.ts`);
const LG: any = await import(`${ROOT}/src/game/lighting.ts`), LS: any = await import(`${ROOT}/src/game/lightswitch.ts`), SV: any = await import(`${ROOT}/src/game/savestate.ts`), HR: any = await import(`${ROOT}/src/game/hours.ts`);
const LIGHT_IN = H.LIGHT_IN, LIGHT_COLS: readonly string[] = H.LIGHT_COLS, WORDS: ReadonlySet<string> = H.WORD_COLS, CAPS = H.LIGHT_CAPS;
const argv = process.argv.slice(2), flag = (n: string) => { const i = argv.indexOf(n); return i >= 0 ? argv[i + 1] : undefined; }, has = (n: string) => argv.includes(n);
const TICKS = Number(flag('--ticks') ?? 2400), SEED = Number(flag('--seed') ?? 1), BREAK = process.env.LIGHTS_BREAK === '1', DT = 1 / 60;
const LEVELS = (flag('--levels') ?? 'office.json,lights-out.json,vault.json,tut-sky.json,bazaar.json,pass.json,depot.json').split(',');
const FIXTURE = resolve(ROOT, 'tools/qa/fixtures/lights-ident.json');
const log = console.log, err = console.error; const hush = (on: boolean) => { console.log = on ? () => {} : log; console.error = on ? () => {} : err; };
let bad = 0; const fail = (why: string) => { if (bad++ < 12) log(`- FAIL — ${why}`); };
const rng = (seed: number) => { let s = seed >>> 0; return () => { s = (s + 0x6D2B79F5) >>> 0; let t = s; t = Math.imul(t ^ (t >>> 15), t | 1); t ^= t + Math.imul(t ^ (t >>> 7), t | 61); return ((t ^ (t >>> 14)) >>> 0) / 4294967296; }; };

// ---------------------------------------------------------------- how long each column is
const WIDTH: Record<string, number> = {};
{ const groups: [string, number][] = [['blind_until bits factor stagger base hand', CAPS.targets], ['x y z range radius kind dx dy dz outer inner r g b intensity sx sy sz s_radius s_intensity row_base row_bits row_target', CAPS.rows], ['beacon_phase beacon_row beacon_target', CAPS.beacons],
    ['cam_yaw cam_phase cam_seen cam_reported cam_lx cam_ly cam_lz cam_rest cam_a cam_b cam_period cam_pitch cam_px cam_py cam_pz cam_state cam_bits cam_target cam_row', CAPS.cameras], ['las_last las_cut las_sense las_trips las_bits las_target', CAPS.lasers]];
  for (const k of LIGHT_COLS) WIDTH[k] = 1;
  for (const [names, n] of groups) for (const k of names.split(' ')) WIDTH[k] = n; }
const ROW_COLS = 'x y z range radius kind dx dy dz outer inner r g b intensity sx sy sz s_radius s_intensity row_bits'.split(' ');
const CAM_COLS = 'cam_yaw cam_phase cam_seen cam_reported cam_lx cam_ly cam_lz cam_state cam_bits'.split(' ');
const LAS_COLS = 'las_last las_trips las_bits'.split(' ');

/** one column of a home's part, as numbers */
function read(home: any, k: string): number[] { const st = home.block, at = home.col(k), src = WORDS.has(k) ? st.w : st.d, out = new Array(WIDTH[k]); for (let i = 0; i < WIDTH[k]; i++) out[i] = src[at + i]; return out; }
function readInto(home: any, T: Record<string, number[]>, k: string, i: number) { const st = home.block; T[k][i] = (WORDS.has(k) ? st.w : st.d)[home.col(k) + i]; }

// ---------------------------------------------------------------- THE TWIN: the TypeScript's own statements, on plain numbers
const T_ON = 1, T_BROKEN = 2, T_FLICKER = 4, T_MAINS = 8, T_HAND = 16, T_AUTHORED = 32, T_EMISSIVE = 64, R_ON = 1, R_ROLE = 24, R_OWN = 0, R_AREA = 8, C_SWEEP = 1, C_LAST = 2, C_SAW = 4, C_DEAD = 8, L_DEAD = 1, L_HELD = 2;
const RESTRIKE = BREAK ? 0.6505 : 0.65;
function supply(time: number, darkUntil: number): number {
  if (time < darkUntil) return 0;
  if (time < darkUntil + 1.2) { const x = (time - darkUntil) / 1.2; const s = Math.sin(time * 45) * Math.sin(time * 13.7 + 1.3); return s > RESTRIKE - x * 0.9 ? 1 : 0.05; }
  return 1;
}
class Twin {
  T: Record<string, number[]> = {};
  calls = 0; kinds = new Set<string>();
  constructor(readonly home: any) { for (const k of LIGHT_COLS) this.T[k] = read(home, k); }
  private back(time: number) { const T = this.T; T.mains[0] = 0; T.until[0] = -1; T.restored_at[0] = time; T.fix_t[0] = 0; }
  private burn(r: number, on: boolean, I: number) { const T = this.T; T.intensity[r] = I; T.s_intensity[r] = I; T.row_bits[r] = (T.row_bits[r] & ~R_ON) | (on ? R_ON : 0); }
  private hand(t: number, a: number) { const T = this.T; if (a === a) { T.bits[t] |= T_HAND; T.hand[t] = a; } else { T.bits[t] &= ~T_HAND; T.hand[t] = 0; } }
  in(kind: number, t: number, a: number, b: number, c: number, d: number): number {
    const T = this.T, I = LIGHT_IN, bits = T.bits;
    switch (kind) {
      case I.on: if (a !== 0) bits[t] |= T_ON; else bits[t] &= ~T_ON; break;
      case I.break: bits[t] |= T_BROKEN; break;
      case I.mend: bits[t] &= ~T_BROKEN; break;
      case I.toggle: bits[t] = (bits[t] ^ T_ON) & ~T_BROKEN; break;
      case I.blind: T.blind_until[t] = a; break;
      case I.hand: this.hand(t, a); break;
      case I.authored: if (a !== 0) bits[t] |= T_AUTHORED; else bits[t] &= ~T_AUTHORED; break;
      case I.base: T.base[t] = a; break;
      case I.stagger: T.stagger[t] = a; break;
      case I.put: bits[t] = (bits[t] & ~(T_ON | T_BROKEN)) | (a !== 0 ? T_ON : 0) | (b !== 0 ? T_BROKEN : 0); T.blind_until[t] = c; this.hand(t, d); break;
      case I.trip: { const time = a, seconds = b, on = (T.mains[0] & 1) !== 0, perm = (T.mains[0] & 2) !== 0, permanent = !isFinite(seconds);
        if (on && perm) return 0;
        const fresh = !on; if (!on) T.since[0] = time;
        T.mains[0] = 1 | (permanent ? 2 : 0); T.until[0] = permanent ? Infinity : Math.max(T.until[0], time + seconds);
        return fresh ? 3 : 1; }
      case I.back: if ((T.mains[0] & 1) === 0) return 0; this.back(a); return 1;
      case I.mains: T.mains[0] = a; T.until[0] = b; T.since[0] = c; T.restored_at[0] = d; break;
      case I.fix: T.fix_t[0] = a; break;
      case I.fresh: T.mains[0] = 0; T.until[0] = -1; T.since[0] = -1; T.restored_at[0] = -100; T.fix_t[0] = 0; break;
      default: throw new Error(`an event of a kind the twin has no line for: ${kind}`);
    }
    return 0;
  }
  flip(on: boolean, time: number, strike: boolean, list: number[]) {
    const T = this.T, bits = T.bits;
    if (on) {
      const hand = list.some(t => (bits[t] & T_HAND) !== 0 && (bits[t] & T_ON) === 0), authored = list.some(t => (bits[t] & T_AUTHORED) !== 0);
      for (const t of list) {
        const mine = hand ? (bits[t] & T_HAND) !== 0 : !authored || (bits[t] & T_AUTHORED) !== 0;
        if (mine && (bits[t] & T_ON) === 0) { if (strike && (bits[t] & T_EMISSIVE) !== 0 && (bits[t] & T_BROKEN) === 0 && T.blind_until[t] < 0) T.blind_until[t] = time; bits[t] |= T_ON; }
        this.hand(t, NaN);
      }
    } else for (const t of list) { if ((bits[t] & T_ON) === 0) continue; bits[t] &= ~T_ON; this.hand(t, time); }
  }
  step(time: number, dt: number, near: boolean, fumble: number, es: number): number {
    const T = this.T; let ev = 0;
    const on = () => (T.mains[0] & 1) !== 0, perm = () => (T.mains[0] & 2) !== 0;
    if (on() && !perm() && time >= T.until[0]) { this.back(time); ev |= 1; }
    if (on() && !perm() && time - T.since[0] > 3) {
      T.fix_t[0] = near ? T.fix_t[0] + dt : 0;
      if (near) { ev |= 2; if (T.fix_t[0] > fumble) { this.back(time); ev |= 4; } }
    }
    const beaconsOn = on() ? time - T.since[0] > 0.8 : time - T.restored_at[0] < 1.2;
    for (let i = 0; i < T.beacons[0]; i++) {
      const t = T.beacon_target[i], r = T.beacon_row[i];
      if (beaconsOn) T.bits[t] |= T_ON; else T.bits[t] &= ~T_ON;
      const a = time * 3.3 + T.beacon_phase[i], v = [Math.cos(a), -0.5, Math.sin(a)], l = Math.hypot(v[0], v[1], v[2]) || 1;
      T.dx[r] = v[0] / l; T.dy[r] = v[1] / l; T.dz[r] = v[2] / l;
      this.burn(r, beaconsOn && T.factor[t] > 0, T.row_base[r]);
    }
    for (let t = 0; t < T.n[0]; t++) {
      const b = T.bits[t]; let f = (b & T_ON) !== 0 && (b & T_BROKEN) === 0 ? 1 : 0;
      if (f > 0 && T.blind_until[t] > 0) { f *= supply(time, T.blind_until[t]); if (time > T.blind_until[t] + 1.2) T.blind_until[t] = -1; }
      if (f > 0 && (b & T_MAINS) !== 0) f *= on() ? 0 : supply(time, T.restored_at[0] + T.stagger[t]);
      if (f > 0 && (b & T_FLICKER) !== 0) { const s = Math.sin(time * 3.1) + Math.sin(time * 7.3 + 2) * 0.6; if (s > 1.15) f *= (Math.sin(time * 90) > 0 ? 0.15 : 1); }
      T.factor[t] = f;
    }
    for (let r = 0; r < T.rows[0]; r++) {
      const role = T.row_bits[r] & R_ROLE;
      if (role === R_OWN) { const t = T.row_target[r], f = T.factor[t]; this.burn(r, f > 0.001, T.base[t] * f); }
      else if (role === R_AREA) { const f = T.factor[T.row_target[r]]; this.burn(r, f > 0.001, T.row_base[r] * f * es); }
    }
    return ev;
  }
  private alive(t: number) { const T = this.T; return T.factor[t] > 0.05 && (T.bits[t] & T_BROKEN) === 0; }
  pan(i: number, dt: number, live: boolean, cones: boolean): number {
    const T = this.T, t = T.cam_target[i], ok = this.alive(t), sweep = (T.cam_bits[i] & C_SWEEP) !== 0;
    if (ok && sweep) { T.cam_phase[i] = (T.cam_phase[i] + dt / Math.max(0.5, T.cam_period[i])) % 1; const s = 0.5 - 0.5 * Math.cos(T.cam_phase[i] * Math.PI * 2); T.cam_yaw[i] = T.cam_a[i] + (T.cam_b[i] - T.cam_a[i]) * s; }
    else if (!sweep) T.cam_yaw[i] = T.cam_rest[i];
    const pitch = T.cam_pitch[i], yaw = T.cam_yaw[i], cp = Math.cos(pitch), look = [Math.sin(yaw) * cp, Math.sin(pitch), Math.cos(yaw) * cp], r = T.cam_row[i];
    T.dx[r] = look[0]; T.dy[r] = look[1]; T.dz[r] = look[2];
    T.x[r] = T.cam_px[i] + look[0] * 0.1; T.y[r] = T.cam_py[i] + look[1] * 0.1; T.z[r] = T.cam_pz[i] + look[2] * 0.1; T.sx[r] = T.x[r]; T.sy[r] = T.y[r]; T.sz[r] = T.z[r];
    this.burn(r, ok && cones, T.intensity[r]);
    let out = ok ? 1 : 0; const broken = (T.bits[t] & T_BROKEN) !== 0;
    if (broken && (T.cam_bits[i] & C_DEAD) === 0) { T.cam_bits[i] |= C_DEAD; T.cam_seen[i] = 0; T.cam_state[i] = 0; out |= 2; }
    else if (!broken) T.cam_bits[i] &= ~C_DEAD;
    T.cam_bits[i] &= ~C_SAW;
    if (!ok || !live) { T.cam_seen[i] = ok ? Math.max(0, T.cam_seen[i] - 0.5 * dt) : 0; if (!ok) T.cam_state[i] = 0; }
    return out;
  }
  see(i: number, time: number, dt: number, visible: boolean, gain: number, x: number, y: number, z: number, sus: number, alarm: number): number {
    const T = this.T; let out = 0;
    if (visible && gain > 0.02) { T.cam_seen[i] = Math.min(alarm + 1, T.cam_seen[i] + gain * dt); T.cam_lx[i] = x; T.cam_ly[i] = y; T.cam_lz[i] = z; T.cam_bits[i] |= C_LAST | C_SAW; }
    else T.cam_seen[i] = Math.max(0, T.cam_seen[i] - 0.5 * dt);
    const last = (T.cam_bits[i] & C_LAST) !== 0;
    if (T.cam_state[i] === 0 && T.cam_seen[i] >= sus && last) { T.cam_state[i] = 1; T.cam_reported[i] = time; out |= 4; }
    if (T.cam_state[i] !== 2 && T.cam_seen[i] >= alarm && last && (T.cam_bits[i] & C_SAW) !== 0) { T.cam_state[i] = 2; T.cam_reported[i] = time; out |= 8; }
    if (T.cam_state[i] !== 0 && T.cam_seen[i] <= 0 && time - T.cam_reported[i] > 20) T.cam_state[i] = 0;
    return out;
  }
  open(i: number): number {
    const T = this.T, t = T.las_target[i], ok = this.alive(t); let out = ok ? 1 : 0; const broken = (T.bits[t] & T_BROKEN) !== 0;
    if (broken && (T.las_bits[i] & L_DEAD) === 0) { T.las_bits[i] |= L_DEAD; out |= 2; } else if (!broken) T.las_bits[i] &= ~L_DEAD;
    if (!ok) { T.las_cut[i] = 0; T.las_sense[i] = 0; T.las_bits[i] &= ~L_HELD; }
    return out;
  }
  close(i: number, time: number, live: boolean, cut: number, sense: number, held: boolean): number {
    const T = this.T; T.las_cut[i] = cut; T.las_sense[i] = sense;
    const edge = held && (T.las_bits[i] & L_HELD) === 0; if (held) T.las_bits[i] |= L_HELD; else T.las_bits[i] &= ~L_HELD;
    if (!live || !edge || time - T.las_last[i] < 4) return 0;
    T.las_last[i] = time; T.las_trips[i]++; return 4;
  }
  /** what the host wrote itself (`feed`, a machine's `put`): taken as written, since nothing of the module's step is in it */
  rowFed(r: number) { for (const k of ROW_COLS) readInto(this.home, this.T, k, r); }
  camPut(s: number) { for (const k of CAM_COLS) readInto(this.home, this.T, k, s); }
  lasPut(s: number) { for (const k of LAS_COLS) readInto(this.home, this.T, k, s); }
  /** the part against this, every column of what is in use */
  diff(where: string) {
    const T = this.T, st = this.home.block;
    for (const k of LIGHT_COLS) {
      const n = k.startsWith('cam_') ? T.cameras[0] : k.startsWith('las_') ? T.lasers[0] : k.startsWith('beacon_') ? T.beacons[0] : WIDTH[k] === 1 ? 1 : ROW_COLS.includes(k) || k === 'row_base' || k === 'row_target' ? T.rows[0] : T.n[0];
      const at = this.home.col(k), src = WORDS.has(k) ? st.w : st.d, tw = T[k];
      for (let i = 0; i < n; i++) if (!Object.is(src[at + i], tw[i])) { fail(`${where}: ${k}[${i}] is ${src[at + i]} in the module and ${tw[i]} by the TypeScript's arithmetic`); tw[i] = src[at + i]; }
    }
  }
}
/** put every call of the way in to a twin too, and compare after it */
function watch(twins: Map<any, Twin>, where: () => string) {
  H.LIGHTS_WATCH.fn = (home: any, what: string, a: readonly number[], out: number) => {
    if (what === 'camera-put' || what === 'laser-put') { for (const T of twins.values()) { const ok = what === 'camera-put' ? a[0] < T.T.cameras[0] : a[0] < T.T.lasers[0]; if (!ok) continue; if (what === 'camera-put') T.camPut(a[0]); else T.lasPut(a[0]); } return; }
    let T = twins.get(home); if (!T) return;
    T.calls++; T.kinds.add(what === 'in' ? `in:${Object.keys(LIGHT_IN).find(k => LIGHT_IN[k] === a[0])}` : what);
    let got = 0;
    switch (what) {
      case 'in': got = T.in(a[0], a[1], a[2], a[3], a[4], a[5]); break;
      case 'flip': T.flip(a[0] !== 0, a[1], a[2] !== 0, a.slice(3) as number[]); break;
      case 'step': got = T.step(a[0], a[1], a[2] !== 0, a[3], a[4]); break;
      case 'pan': got = T.pan(a[0], a[1], a[2] !== 0, a[3] !== 0); break;
      case 'see': got = T.see(a[0], a[1], a[2], a[3] !== 0, a[4], a[5], a[6], a[7], a[8], a[9]); break;
      case 'open': got = T.open(a[0]); break;
      case 'close': got = T.close(a[0], a[1], a[2] !== 0, a[3], a[4], a[5] !== 0); break;
      case 'feed': T.rowFed(a[0]); return;
      default: throw new Error(`a call the twin has no line for: ${what}`);
    }
    if (got !== out) fail(`${where()}: ${what}(${a.join(', ')}) answered ${out} in the module and ${got} by the TypeScript's arithmetic`);
    T.diff(`${where()} after ${what}${what === 'in' ? ':' + a[0] : ''}`);
  };
}

// ---------------------------------------------------------------- THE WORD: every column in use folded
function fold(home: any, into: number): number {
  let h = into >>> 0; const st = home.block, B = new Float64Array(1), W = new Uint32Array(B.buffer);
  const mix = (x: number) => { h = Math.imul(h ^ x, 0x01000193) >>> 0; };
  const T = (k: string) => read(home, k)[0];
  for (const k of LIGHT_COLS) {
    const n = k.startsWith('cam_') ? T('cameras') : k.startsWith('las_') ? T('lasers') : k.startsWith('beacon_') ? T('beacons') : WIDTH[k] === 1 ? 1 : ROW_COLS.includes(k) || k === 'row_base' || k === 'row_target' ? T('rows') : T('n');
    const at = home.col(k); if (WORDS.has(k)) for (let i = 0; i < n; i++) mix(st.w[at + i]); else for (let i = 0; i < n; i++) { B[0] = st.d[at + i]; mix(W[0]); mix(W[1]); }
  }
  return h;
}

// ---------------------------------------------------------------- PART ONE: made-up fixtures, every event kind on a clock
function lampDef(i: number, kind: number, enabled: boolean, intensity: number): any { const k = i + 1; return { name: `lamp${i}`, group: 'made', kind, pos: [k * 0.5, 2.5, -k * 0.25], dir: [0, -1, 0], color: [0.9, 0.8, 0.7], intensity, range: 6 + i % 5, innerDeg: 20, outerDeg: 40, radius: 0.1, volumetric: 0, owner: 0, enabled, fixtureBox: -1, flicker: 0, ttl: -1, age: 0, peakIntensity: intensity }; }
function madeUp(seed: number, ticks: number, twinned: boolean): { word: number; kinds: Set<string> } {
  const rnd = rng(seed), st = new ST.SimState(), home = new H.LightHome(st), list: any[] = [], targets: any[] = [];
  const NF = 40, fix = (i: number, extra: any = {}) => ({ name: `f${i}`, group: i % 3 ? 'a' : 'b', kind: i % 4 === 0 ? 'analytic' : 'emissive', pos: [i, 2, 0], boxes: [], baseEmissive: [1, 1, 1], baseIntensity: 2 + i * 0.25, on: i % 5 !== 0, broken: false, disabledUntil: -1, fluorescentFlicker: i % 7 === 0, factor: i % 5 !== 0 ? 1 : 0, mains: i % 3 !== 2, interactive: true, stagger: i * 0.13, ...extra });
  for (let i = 0; i < NF; i++) { const def = fix(i); if (def.kind === 'analytic') { def.light = lampDef(i, 0, def.on, def.baseIntensity); list.push(def.light); } const t = home.add(def); targets.push(t); if (i % 4 === 1) { const al = lampDef(100 + i, 3, def.on, 1.5); list.push(al); t.areaLights = [{ light: al, base: 1.5 }]; } home.in(LIGHT_IN.authored, t, i % 2); }
  const beacons: any[] = []; for (let i = 0; i < 3; i++) { const l = lampDef(200 + i, 1, false, 55); list.push(l); beacons.push({ light: l, phase: i * 1.7, target: targets[i * 5] }); }
  const cams: any[] = [], lasers: number[] = [];
  for (let i = 0; i < 3; i++) { const t = home.add(fix(NF + i, { kind: 'device', mains: i !== 1, stagger: 1.4 + 0.3 * i })); targets.push(t); const cone = lampDef(300 + i, 1, false, 1.1); list.push(cone); const slot = home.camera(t, 0.5 * i, i === 2 ? undefined : [-0.8, 0.9], 6 + i, -0.4, [i, 3, 2]); cams.push({ light: cone, target: t, slot }); }
  for (let i = 0; i < 2; i++) { const t = home.add(fix(NF + 3 + i, { kind: 'device', stagger: 1.1 + 0.3 * i })); targets.push(t); lasers.push(home.laser(t, 5 + i)); }
  home.stand(list, targets, beacons, cams);
  const twins = new Map<any, Twin>(); if (twinned) { twins.set(home, new Twin(home)); watch(twins, () => `made-up seed ${seed} tick ${tick}`); }
  let time = 3, word = 0x811c9dc5, tick = 0;
  const someF = () => targets[Math.floor(rnd() * targets.length)];
  for (tick = 0; tick < ticks; tick++) {
    time += DT;
    if (tick % 7 === 0) {
      const t = someF(), k = Math.floor(rnd() * 15);
      switch (k) {
        case 0: home.in(LIGHT_IN.on, t, rnd() < 0.5 ? 1 : 0); break;
        case 1: home.in(LIGHT_IN.break, t); break;
        case 2: home.in(LIGHT_IN.mend, t); break;
        case 3: home.in(LIGHT_IN.toggle, t); break;
        case 4: home.in(LIGHT_IN.blind, t, rnd() < 0.2 ? -1 : time + rnd() * 3); break;
        case 5: home.in(LIGHT_IN.hand, t, rnd() < 0.5 ? NaN : time); break;
        case 6: home.in(LIGHT_IN.base, t, rnd() * 4); break;
        case 7: home.in(LIGHT_IN.put, t, rnd() < 0.5 ? 1 : 0, rnd() < 0.3 ? 1 : 0, rnd() < 0.5 ? -1 : time + 1, rnd() < 0.5 ? NaN : time - 1); break;
        case 8: home.in(LIGHT_IN.trip, null, time, rnd() < 0.15 ? Infinity : rnd() * 7); break;
        case 9: home.in(LIGHT_IN.back, null, time); break;
        case 10: home.in(LIGHT_IN.mains, null, rnd() < 0.5 ? 0 : 1, time + rnd() * 5, time - rnd() * 4, time - rnd() * 2); break;
        case 11: home.in(LIGHT_IN.fix, null, rnd()); break;
        case 12: if (rnd() < 0.2) home.in(LIGHT_IN.fresh, null); break;
        case 13: home.in(rnd() < 0.5 ? LIGHT_IN.stagger : LIGHT_IN.authored, t, rnd() < 0.5 ? rnd() * 2 : 0); break;
        case 14: { const a = targets.filter(() => rnd() < 0.2); home.flip(a, rnd() < 0.5, time, rnd() < 0.7); break; }
      }
      if (rnd() < 0.1) { const c = cams[Math.floor(rnd() * cams.length)]; const M = new H.CameraMachine(c.slot, st); M.put(rnd(), rnd(), rnd() * 2, H.CAMERA_STATES[Math.floor(rnd() * 3)], time - rnd() * 30, rnd() < 0.5 ? null : [rnd(), 0, rnd()], rnd() < 0.3); }
      if (rnd() < 0.1) { const L = new H.LaserMachine(lasers[Math.floor(rnd() * lasers.length)], st); L.put(time - rnd() * 8, Math.floor(rnd() * 4), rnd() < 0.3, rnd() < 0.3); }
      if (rnd() < 0.3) { const l = list[Math.floor(rnd() * list.length)]; l.intensity = rnd() * 3; l.pos = [rnd(), rnd(), rnd()]; l.enabled = rnd() < 0.5; if (rnd() < 0.5) l.steady = { intensity: rnd(), pos: [rnd(), rnd(), rnd()], radius: rnd() }; else delete l.steady; home.feed(l); }
    }
    const dt = rnd() < 0.02 ? 0 : DT, near = rnd() < 0.15;
    home.step(time, dt, near, 1 + rnd() * 2, 0.5 + rnd());
    for (const c of cams) { const live = rnd() < 0.9, m = home.cameraPan(c.slot, c.light, dt, live, rnd() < 0.8); if (live && (m & H.CAM.ok)) { const vis = rnd() < 0.6; home.cameraSee(c.slot, time, dt, vis, vis ? rnd() * 3 * (rnd() < 0.2 ? 0 : 1) : 0, [rnd(), 0, rnd()], 0.8, 2); } }
    for (const l of lasers) { const m = home.laserOpen(l); if (m & H.LASER.ok) home.laserClose(l, time, rnd() < 0.9, rnd() * 5, rnd() * 5, rnd() < 0.3); }
    home.paint();
    word = fold(home, word);
  }
  const T = twins.get(home); H.LIGHTS_WATCH.fn = null;
  return { word, kinds: T ? T.kinds : new Set() };
}

// ---------------------------------------------------------------- PART TWO AND THREE: whole games
/** each lamp with a row: its record in the part is the record the renderer's list holds */
function rowsTrue(g: any, where: string) {
  const home = g.lightHome, st = home.block, d = st.d, w = st.w;
  for (const l of g.engine.lights.lights) {
    const r = l.row; if (r === undefined) continue; const s = l.steady;
    const say = (name: string, a: number, b: number) => { if (!Object.is(a, b)) fail(`${where}: the row of ${l.name}: ${name} is ${a} and the lamp's ${b}`); };
    const c = (k: string) => d[home.col(k) + r];
    say('on', (w[home.col('row_bits') + r] & 1) !== 0 ? 1 : 0, l.enabled ? 1 : 0);
    if (!l.enabled) continue;   // (no question reads the rest of a row that is out: lightshome.ts feedBurning)
    say('x', c('x'), l.pos[0]); say('y', c('y'), l.pos[1]); say('z', c('z'), l.pos[2]); say('range', c('range'), l.range); say('radius', c('radius'), l.radius); say('kind', c('kind'), l.kind);
    say('dx', c('dx'), l.dir[0]); say('dy', c('dy'), l.dir[1]); say('dz', c('dz'), l.dir[2]); say('outer', c('outer'), l.outerDeg); say('inner', c('inner'), l.innerDeg);
    say('r', c('r'), l.color[0]); say('g', c('g'), l.color[1]); say('b', c('b'), l.color[2]); say('intensity', c('intensity'), l.intensity);
    say('steady x', c('sx'), s ? s.pos[0] : l.pos[0]); say('steady y', c('sy'), s ? s.pos[1] : l.pos[1]); say('steady z', c('sz'), s ? s.pos[2] : l.pos[2]); say('steady radius', c('s_radius'), s ? s.radius : l.radius); say('steady intensity', c('s_intensity'), s ? s.intensity : l.intensity);
  }
}
async function level(file: string): Promise<Set<string>> {
  hush(true); let W: any;
  try { const data = JSON.parse(await Bun.file(resolve(ROOT, 'levels', file)).text()), lv = file === 'office.json' ? null : loadLevel(data); W = await standUp({ uploadEvery: 1000, seed: SEED, ...(lv ? { level: lv } : {}) }); } finally { hush(false); }
  const g = W.game, rnd = rng(SEED * 7 + file.length), twins = new Map<any, Twin>(); let tick = 0;
  twins.set(g.lightHome, new Twin(g.lightHome)); watch(twins, () => `${file} tick ${tick}`);
  const someT = (f: (t: any) => boolean = () => true) => { const a = g.targets.filter(f); return a.length ? a[Math.floor(rnd() * a.length)] : null; };
  const events: (() => void)[] = [
    () => { const t = someT(x => x.kind !== 'breaker'); if (t) g.ocp(t); g.player.ocp.charge = 1; },
    () => { const t = someT(x => x.kind === 'breaker'); if (t) g.ocp(t); g.player.ocp.charge = 1; },
    () => { const t = someT(x => x.boxes.length > 0 && x.kind !== 'breaker'); if (t && g.destr?.breakFixtureBox) g.destr.breakFixtureBox(t.boxes[0]); },
    () => { g.setBlackout(4 + rnd() * 6, 'debug'); },
    () => { g.setBlackout(86400, 'hand'); },
    () => { g.endBlackout(); },
    () => { const t = someT(); if (t) LG.setGroup(g, t.group, rnd() < 0.5); },
    () => { const t = someT(); if (t) LG.toggleTarget(g, t); },
    () => { LG.repairLights(g); },
    () => { if (LS.LIGHTSWITCH.on) { const a = LS.wallSwitches(g); if (a.length) { const sw = a[Math.floor(rnd() * a.length)]; LS.flipSwitch(g, sw, !LS.switchLit(sw), g.player.char); } } },
    () => { if (g.level.daylight && !LG.isDaylight(g)) LG.setDaylight(g, true); else if (LG.isDaylight(g)) LG.setDaylight(g, false); },
    () => { if (!LG.isDaylight(g)) HR.applyHour(g, HR.hourOf(g) === 'night' ? 'midday' : 'night'); },
    () => { if (LG.isDaylight(g)) LG.setDaylight(g, false); const s = SV.saveState(g); W.step(DT); SV.loadState(g, JSON.parse(JSON.stringify(s))); },
    () => { if (LG.isDaylight(g)) LG.setDaylight(g, false); g.restartEncounter(); },
    () => { g.setBlackout(Infinity, 'shot'); },
    () => { const c = g.devices?.cameras[Math.floor(rnd() * g.devices.cameras.length)]; if (c) { const look = [Math.sin(c.yaw), Math.cos(c.yaw)], p = g.col.nav.nearestFreePoint(c.def.pos[0] + look[0] * 3, c.def.pos[2] + look[1] * 3); if (p) g.teleportPlayer([p[0], 0, p[2]]); } },
    () => { const l = g.devices?.lasers[Math.floor(rnd() * g.devices.lasers.length)]; if (l) { const a = l.def.from, b = l.def.to, p = g.col.nav.nearestFreePoint((a[0] + b[0]) / 2, (a[2] + b[2]) / 2); if (p) g.teleportPlayer([p[0], 0, p[2]]); } },
    () => { const t = someT(x => x.kind === 'device'); if (t) g.lightHome.stage(t, null, true); },
  ];
  g.godMode = true; let met = new Set<string>();
  hush(true);
  try {
    for (tick = 0; tick < TICKS; tick++) {
      if (tick % 45 === 44) events[Math.floor(rnd() * events.length)]();
      W.step(DT);
      if (BREAK && tick === 300 && g.lightHome.block.d.length) { const r = 0, at = g.lightHome.col('x'); g.lightHome.block.d[at + r] += 0.001; }
      rowsTrue(g, `${file} tick ${tick}`);
    }
  } finally { hush(false); }
  met = twins.get(g.lightHome)!.kinds; H.LIGHTS_WATCH.fn = null;
  return met;
}

// ---------------------------------------------------------------- the run
{ const no = SIM.simWhyNot(); if (no) { log(`=== lights-ident: the module does not run here (${no}) — FAIL ===`); process.exit(1); } }
const KEEP: Record<string, number> = await Bun.file(FIXTURE).json().catch(() => ({})), KEPT = TICKS === 2400;   // (the words are of 2400 ticks a seed)
if (!Object.keys(KEEP).length && !has('--write')) fail(`no kept words at ${FIXTURE}: --write makes them`);
const words: Record<string, number> = {};
{ const all = new Set<string>();
  for (const s of [1, 2, 3]) { const R = madeUp(s, TICKS, true); words[`seed${s}`] = R.word; for (const k of R.kinds) all.add(k);
    if (KEPT && KEEP[`seed${s}`] !== undefined && KEEP[`seed${s}`] !== R.word && !has('--write')) fail(`made-up seed ${s}: the word is ${R.word.toString(16)} and the kept one ${KEEP[`seed${s}`].toString(16)}`); }
  const want = [...Object.keys(LIGHT_IN).map(k => `in:${k}`), 'flip', 'step', 'pan', 'see', 'open', 'close'], missed = want.filter(k => !all.has(k));
  if (missed.length) fail(`the made-up run never met: ${missed.join(', ')}`);
  log(`made-up: 3 seeds, ${TICKS} ticks, ${all.size} kinds of call compared, words ${Object.values(words).map(w => w.toString(16)).join(' ')}`); }
{ const all = new Set<string>();
  for (const f of LEVELS) { const met = await level(f); for (const k of met) all.add(k); log(`${f.padEnd(22)} ${met.size} kinds of call compared: ${[...met].join(' ')}`); }
  if (!flag('--levels')) { const want = ['step', ...(LS.LIGHTSWITCH.on ? ['flip', 'in:hand'] : []), 'pan', 'see', 'open', 'close', 'in:on', 'in:break', 'in:mend', 'in:toggle', 'in:blind', 'in:put', 'in:trip', 'in:back', 'in:mains', 'in:fix'], missed = want.filter(k => !all.has(k));
    if (missed.length) fail(`the games never met: ${missed.join(', ')}`); } }
if (has('--write')) { await Bun.write(FIXTURE, JSON.stringify(words, null, 1) + '\n'); log('words written'); }
log(bad ? `=== lights-ident: FAIL, ${bad} difference(s)${BREAK ? ' (the red path: as it should)' : ''} ===` : `=== lights-ident: PASS — the module and the TypeScript's arithmetic are one, and every row is its lamp ===`);
process.exit(bad ? 1 : 0);
