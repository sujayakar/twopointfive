// The level-design devices (STATUS G4, docs/level-format.md 'Devices'): wall CAMERAS that see Sam through the guards' own eye test and light meter and call the
// floor's radio, KEYPADS on doors whose code Sam has to learn before the pad takes it, LASER tripwires that call the radio when something breaks them. All three
// are authored in LevelData.devices (scene/levelData.ts DeviceData → Level.devices); everything they show — a camera's body, lens and LED, a pad's plate and
// LED, a beam's emitter, receiver and the beam itself — is drawn HERE from the record as dynamic boxes (nothing of them is in `boxes`), and cameras / emitters
// are light TARGETS (game.ts LightTarget kind 'device') so the pistol's OCP blinds them for its spell, a round kills them for the encounter, and the mains-fed
// ones die in a blackout and come back with the restrike — lighting.ts updateTargets works their `factor` out with everybody else's; this file only reads it.
//
// ONE MODEL. A camera's picture is the guards' eye (guards.ts eyeContact: cone, reach, the line to chest and head through statics and leaves, the smoke marched
// between lens and chest by the same probe segment a guard's eye gets) and its exposure is Sam's light meter (Player.visibility — dark to you is dark to it):
// lensGain below is sightGain's light × distance × crouch × smoke, without the human parts (motion acuity, an alarmed floor's keenness, a hunter's doubling —
// a lens does not startle; the man at the monitor gets the integrator's seconds instead). A code overheard is heard through the propagation the mixer and the
// guards' ears use (col.sound): out of earshot, nothing learned. Guards never trip their own beams and never mind the cameras.
//
// What a device DOES to the floor goes through what the floor already has: control's radio line (g.msg 'radio: CONTROL — …', the HUD's radio strip), the
// nearest calm man sent to look (a GuardTask 'device' — the rack errand's mechanism: it survives the suspicion decay, he walks there, looks round, talks himself
// out of it), the floor going up a step (guards.ts raiseEscalation cause 'device'), the debrief's chronicle. Live play only (g.aiEnabled, not the tour's
// quietUtility, tune.devices): under the tour they pan and glow and report nothing.
import './seats';
import { Vec3, v3, clamp, lerp } from '../math/vec';
import type { DeviceDef } from '../scene/level';
import { DEVICE_DEFAULTS } from '../scene/levelData';
import type { RtLight } from '../render/lights';
import type { Game, Guard, LightTarget } from './game';
import type { Character } from './character';
import type { Door } from './doors';
import './doors';
import './handwork';
import './guards';
import './kitload';
import './guards';
import './combat';
import { chronicle, chronicleN, roomOf } from './chronicle';
import './civilians';
import { CameraMachine, LaserMachine } from '../sim/lightshome';
import type { SimState } from '../sim/state';
import { RUST_PASS, RUST_SURFACE, goneWithTick } from './crossing';

type CameraDef = Extract<DeviceDef, { kind: 'camera' }>;
type KeypadDef = Extract<DeviceDef, { kind: 'keypad' }>;
type LaserDef = Extract<DeviceDef, { kind: 'laser' }>;

/** the cone thrown into the haze by a live camera (tune.deviceCones): a faint red spot down its axis — a light slot apiece, no more */
const CONE = { intensity: 1.1, color: [1, 0.12, 0.07] as Vec3, rangeK: 0.7, volumetric: 1.0, outerMaxDeg: 24, radius: 0.12 };   // (radius: a sized emitter — clamps 1/d² so the haze shows a beam leaving the lens, not a hot bead in front of it. Where it starts: rust/sim/src/lights.rs CONE_AHEAD)   // (a tell, not a floodlight: at 3+ it painted the whole end of the corridor orange — 08-09 page check)
/** the keypad: seconds to key the code in (four keys, evenly), how close he must stand to the door to be at its pad, the deny beep's worth to the hearing model */
export const CODE_SECS = 1.2;
/** overhearing a code said aloud (dialogue.ts ambient 'pin' pair → overhear): within this many metres of PATH through the air as col.sound propagates it (round the
 *  corner, through the doorway) and at least this much of it carried (a shut door between them eats it) — the whistle's reach, the other way round */
export const OVERHEAR = { pathM: 9, carry: 0.5 };

// ---------------------------------------------------------------- runtime records
/** A CAMERA OF THE LEVEL: its own machine is in the module (sim/lightshome.ts CameraMachine, rust/sim/src/lights.rs camera_pan, camera_see: the pan, the integrator, the state, the last sighting), and these are the level's facts of it */
export class CameraRt extends CameraMachine {
  /** the probe's smoke transmittance lens → Sam's chest (1 headless / until the renderer answers: a sensor's last answer, an input), and the key it is asked under */
  smokeTrans = 1;
  constructor(slot: number, st: SimState, readonly def: CameraDef, readonly index: number, readonly target: LightTarget, readonly cone: RtLight, readonly smokeKey: string) { super(slot, st); }
}
/** A TRIPWIRE OF THE LEVEL: its own machine is in the module (sim/lightshome.ts LaserMachine, rust/sim/src/lights.rs laser_open, laser_close) */
export class LaserRt extends LaserMachine {
  constructor(slot: number, st: SimState, readonly def: LaserDef, readonly index: number, readonly target: LightTarget, readonly dir: Vec3, readonly len: number) { super(slot, st); }
}
export interface KeypadRt { def: KeypadDef; door: Door; /** the pad's outward normal (plate faces it) */ n: Vec3; }
/** what the QA probes and the panel read: every report a device has made this encounter, newest last */
export interface DeviceLogRow { seq: number; t: number; device: string; what: 'suspicion' | 'alarm' | 'trip' | 'dead' | 'code' | 'overheard' | 'deny'; where: string; sent: string | null; }
/** the devices in a checkpoint (Devices.snapshot / restore — game/savestate.ts) */
export interface DevicesSnap { cameras: { yaw: number; phase: number; seen: number; state: 'idle' | 'suspect' | 'alarm'; reportedAt: number; lastSeen: Vec3 | null; deadCalled: boolean }[]; lasers: { lastTripAt: number; trips: number; deadCalled: boolean; held: boolean }[]; log: DeviceLogRow[]; radioCount: number; seq: number; }

export class Devices {
  cameras: CameraRt[] = []; lasers: LaserRt[] = []; keypads: KeypadRt[] = [];
  private padByDoor = new Map<Door, KeypadRt>();
  log: DeviceLogRow[] = [];
  /** reports ever made (monotonic across the log's cap and restarts: the soak reads rows with seq above what it has seen) and radio lines control has put out this encounter */
  seq = 0; radioCount = 0;
  /** append a report to the log (capped) with its sequence number */
  note(g: Game, r: Omit<DeviceLogRow, 'seq' | 't'>) { this.log.push({ seq: ++this.seq, t: g.time, ...r }); if (this.log.length > 48) this.log.splice(0, this.log.length - 48); }
  constructor(g: Game) {
    const defs = g.level.devices ?? [];
    let ci = 0, li = 0;
    for (const d of defs) {
      if (d.kind === 'camera') {
        const target = g.lightHome.add({ name: d.name, group: d.group ?? DEVICE_DEFAULTS.group, kind: 'device', pos: v3.copy(d.pos), boxes: [], baseEmissive: [0, 0, 0], baseIntensity: 0, on: true, broken: false, disabledUntil: -1, fluorescentFlicker: false, factor: 1, mains: d.mains !== false, interactive: true, stagger: 1.4 + 0.3 * ci });
        g.targets.push(target);
        const half = (d.fovDeg ?? DEVICE_DEFAULTS.fovDeg) / 2;
        const cone = g.engine.lights.add({ name: `${d.name}_cone`, group: 'devices', kind: 1, pos: v3.copy(d.pos), dir: [0, -1, 0], color: v3.copy(CONE.color), intensity: CONE.intensity, range: (d.range ?? DEVICE_DEFAULTS.range) * CONE.rangeK, innerDeg: Math.max(1, Math.min(half, CONE.outerMaxDeg) - 6), outerDeg: Math.min(half, CONE.outerMaxDeg), radius: CONE.radius, volumetric: CONE.volumetric, owner: 0, enabled: false, fixtureBox: -1, flicker: 0, ttl: -1, age: 0, peakIntensity: CONE.intensity });
        const slot = g.lightHome.camera(target, d.yaw, d.sweep, d.sweepPeriod ?? DEVICE_DEFAULTS.sweepPeriod, d.pitch ?? DEVICE_DEFAULTS.pitch, d.pos); ci++;
        this.cameras.push(new CameraRt(slot, g.sim, d, ci, target, cone, `smokeCam${ci}`));
      } else if (d.kind === 'laser') {
        const target = g.lightHome.add({ name: d.name, group: d.group ?? DEVICE_DEFAULTS.group, kind: 'device', pos: v3.copy(d.from), boxes: [], baseEmissive: [0, 0, 0], baseIntensity: 0, on: true, broken: false, disabledUntil: -1, fluorescentFlicker: false, factor: 1, mains: d.mains !== false, interactive: true, stagger: 1.1 + 0.3 * li });
        g.targets.push(target);
        const seg = v3.sub(d.to, d.from); const len = v3.len(seg);
        this.lasers.push(new LaserRt(g.lightHome.laser(target, len), g.sim, d, ++li, target, v3.scale(seg, 1 / Math.max(len, 1e-6)), len));
      } else {
        const door = g.doors.byName(d.door);
        if (!door) throw new Error(`keypad '${d.name}': no door '${d.door}'`);   // (the validator has already refused a file where this could happen)
        const K: KeypadRt = { def: d, door, n: [Math.sin(d.yaw), 0, Math.cos(d.yaw)] };
        this.keypads.push(K); this.padByDoor.set(door, K);
      }
    }
  }
  /** the keypad on this door, if it is a keypad door */
  padOf(d: Door): KeypadRt | null { return this.padByDoor.get(d) ?? null; }
  /** the keypad whose door stands nearest a floor point (dialogue.ts: the pad two men talking would mean; the one a questioned man gives up) — null on a level with none */
  padNear(p: Vec3): KeypadRt | null { let best: KeypadRt | null = null, bd = Infinity; for (const k of this.keypads) { const dd = v3.distXZ(k.door.frameCentre, p); if (dd < bd) { bd = dd; best = k; } } return best; }
  /** a fresh encounter: integrators, states, trip clocks, the log (the targets themselves are repaired by lighting.ts repairLights with every other fixture) */
  /** for a checkpoint (game/savestate.ts): each camera's pan phase, integrator and report state, each beam's counts and clocks, the log — the light targets they hang
   *  off (alive / OCP'd / shot) travel with the fixtures */
  snapshot(): DevicesSnap {
    return { cameras: this.cameras.map(c => ({ yaw: c.yaw, phase: c.phase, seen: c.seen, state: c.state, reportedAt: c.reportedAt, lastSeen: c.lastSeen ? v3.copy(c.lastSeen) : null, deadCalled: c.deadCalled })),
      lasers: this.lasers.map(l => ({ lastTripAt: l.lastTripAt, trips: l.trips, deadCalled: l.deadCalled, held: l.held })), log: this.log.map(r => ({ ...r })), radioCount: this.radioCount, seq: this.seq };
  }
  restore(g: Game, s: DevicesSnap) { { const __s = RUST_SURFACE; if (__s) { const __r = __s.call('Devices.restore', this, [g, s]); if (__r !== RUST_PASS) return __r as any; } }
    this.reset();
    const f = (x: number, d = 0) => Number.isFinite(x) ? x : d;
    s.cameras?.forEach((C, i) => { const c = this.cameras[i]; if (!c || !C) return; c.put(f(C.yaw, c.def.yaw), f(C.phase), Math.max(0, f(C.seen)), C.state === 'suspect' || C.state === 'alarm' ? C.state : 'idle', f(C.reportedAt, -100), C.lastSeen ? v3.copy(C.lastSeen) : null, !!C.deadCalled); });
    s.lasers?.forEach((L, i) => { const l = this.lasers[i]; if (!l || !L) return; l.put(f(L.lastTripAt, -100), L.trips | 0, !!L.deadCalled, !!L.held); });
    this.log = (s.log ?? []).slice(-48).map(r => ({ ...r })); this.radioCount = s.radioCount | 0; this.seq = Math.max(this.seq, s.seq | 0);
    if (g.player.coding) stopCoding(g);
  }
  reset() {
    for (const c of this.cameras) c.put(c.def.yaw, 0, 0, 'idle', -100, null, false);
    for (const l of this.lasers) l.put(-100, 0, false, false);
    this.log.length = 0; this.radioCount = 0;   // (seq runs on: it is what the soak counts by)
  }
  /** panel / console: one line — every camera's state and integrator, every beam's trips, the codes Sam knows */
  summary(g: Game): string {
    const cams = this.cameras.map(c => `${c.def.name}${alive(c.target) ? '' : ' (dark)'} ${c.state} ${c.seen.toFixed(2)}s${c.sawThisFrame ? ' ◉' : ''}`).join(' · ');
    const las = this.lasers.map(l => `${l.def.name}${alive(l.target) ? '' : ' (dark)'} ×${l.trips}`).join(' · ');
    return `devices ▸ ${g.tune.devices ? '' : 'OFF · '}${cams || 'no cameras'} | ${las || 'no beams'} | codes [${[...g.player.codes].join(' ') || '—'}]`;
  }
}
/** powered, unbroken, not knocked out: the device works this frame (lighting.ts updateTargets has folded on / broken / the OCP / the mains into factor) */
export function alive(t: LightTarget): boolean { return t.factor > 0.05 && !t.broken; }

/** the lens's look direction at yaw / pitch (Character convention: forward = [sin yaw, 0, cos yaw]; pitch < 0 looks down) */
export function lensDir(yaw: number, pitch: number): Vec3 { return goneWithTick('lensDir') as any; }
/** where a panning camera points at phase u (0‥1 round trip): eased there and back between the sweep's two yaws */
export function sweepYaw(d: CameraDef, phase: number): number { if (!d.sweep) return d.yaw; const s = 0.5 - 0.5 * Math.cos(phase * Math.PI * 2); return lerp(d.sweep[0], d.sweep[1], s); }



/** does any capsule of `c` cross the beam a→b: the point on the beam where it does, else null */
export function bodyBreaks(c: Character, a: Vec3, b: Vec3): Vec3 | null { return goneWithTick('bodyBreaks') as any; }

/** the debrief's rows (chronicle.ts words them): through the mission's own live gate like every hook */
function chronDevice(g: Game, kind: 'cameraSpotted' | 'cameraAlarm' | 'laserTripped' | 'deviceDown' | 'codeLearned' | 'codeEntered', data: Record<string, string | number | boolean | undefined>, pos?: Vec3) {
  return kind === 'cameraSpotted' || kind === 'laserTripped' ? chronicleN(g, kind, 'devices', data, pos) : chronicle(g, kind, data, pos);   // (the score's counter moves with the run live, whether or not the capped timeline still takes the row — chronicle.ts chronicleN, review U9.2)
}

// ---------------------------------------------------------------- keypads: learning a code, keying it in
/** Sam LEARNS the code for lock `lockId` (a keypad's name): off a questioned man (`how` 'told', game.ts interrogate on intel kind 'code') or overheard ('overheard').
 *  Idempotent; says so in the log once; one for the debrief. Returns whether it was news. */
export function learnCode(g: Game, lockId: string, how: 'told' | 'overheard', from?: Guard | null): boolean { { const __s = RUST_SURFACE; if (__s) { const __r = __s.call('learnCode', undefined, [g, lockId, how, from]); if (__r !== RUST_PASS) return __r as any; } }
  const pl = g.player; if (pl.codes.has(lockId)) return false;
  const K = g.devices?.keypads.find(k => k.def.name === lockId); if (!K) return false;
  pl.codes.add(lockId);
  g.msg(`${how === 'told' ? `${from?.callsign ?? 'he'} gave up` : 'overheard'} the ${padDoorName(K)} code — ${spell(K.def.code)}`);
  chronDevice(g, 'codeLearned', { door: K.door.def.name, how, from: from?.callsign }, from?.char.pos);
  g.devices.note(g, { device: lockId, what: how === 'told' ? 'code' : 'overheard', where: roomOf(pl.char.pos, g.level), sent: from?.callsign ?? null });
  return true;
}
/** the digits as said / shown ('7-3-9-1') */
export function spell(code: string): string { return code.split('').join('-'); }
/** what a pad's door is called on the HUD ('side door', 'server door') */
export function padDoorName(K: KeypadRt): string { return K.door.def.name === 'server_manager' ? 'side door' : `${K.door.def.name.replace(/_/g, ' ')} door`; }
export function stopCoding(g: Game) { const pl = g.player; if (!pl.coding) return; pl.coding = null; pl.codeT = 0; }


// ---------------------------------------------------------------- persistence of the panel's knobs (like qrf.ts / whistle.ts: per machine)
const LS_KEY = 'twopointsix.devices';
export const DEVICE_TUNE_DEFAULTS = { devices: true, lasers: true, cameraSuspicionSecs: 0.8, cameraAlarmSecs: 2.0, deviceCones: true };   // (lit-seconds to a report / to the alarm; Tab ▸ Sandbox ▸ Devices carries both — the maintainer, 08-12: 'keep cameras as they are' — structured hiding places for bodies come later as a system)
export function loadDeviceTune(g: Game) {
  try {
    const raw = localStorage.getItem(LS_KEY); if (!raw) return; const o = JSON.parse(raw); const T = g.tune;
    if (typeof o.devices === 'boolean') T.devices = o.devices; if (typeof o.lasers === 'boolean') T.lasers = o.lasers; if (typeof o.deviceCones === 'boolean') T.deviceCones = o.deviceCones;
    if (Number.isFinite(o.cameraSuspicionSecs) && o.cameraSuspicionSecs > 0) T.cameraSuspicionSecs = o.cameraSuspicionSecs; if (Number.isFinite(o.cameraAlarmSecs) && o.cameraAlarmSecs > 0) T.cameraAlarmSecs = o.cameraAlarmSecs;
  } catch { /* none / unavailable */ }
}
export function saveDeviceTune(g: Game) { const T = g.tune; try { localStorage.setItem(LS_KEY, JSON.stringify({ devices: T.devices, lasers: T.lasers, cameraSuspicionSecs: T.cameraSuspicionSecs, cameraAlarmSecs: T.cameraAlarmSecs, deviceCones: T.deviceCones })); } catch { /* */ } }
export function resetDeviceTune(g: Game) { Object.assign(g.tune, DEVICE_TUNE_DEFAULTS); try { localStorage.removeItem(LS_KEY); } catch { /* */ } }

// ---------------------------------------------------------------- geometry
/** Closest points between segments p (p0→p1) and b (b0→b1): the distance and the point on b. (Ericson, Real-Time Collision Detection §5.1.9.) */
export function segSegClosest(p0: Vec3, p1: Vec3, b0: Vec3, b1: Vec3): { dist: number; onB: Vec3 } {
  const d1 = v3.sub(p1, p0), d2 = v3.sub(b1, b0), r = v3.sub(p0, b0);
  const a = v3.dot(d1, d1), e = v3.dot(d2, d2), f = v3.dot(d2, r);
  let s: number, t: number;
  if (a <= 1e-9 && e <= 1e-9) { s = 0; t = 0; }
  else if (a <= 1e-9) { s = 0; t = clamp(f / e, 0, 1); }
  else {
    const c = v3.dot(d1, r);
    if (e <= 1e-9) { t = 0; s = clamp(-c / a, 0, 1); }
    else {
      const bb = v3.dot(d1, d2), denom = a * e - bb * bb;
      s = denom > 1e-9 ? clamp((bb * f - c * e) / denom, 0, 1) : 0;
      t = (bb * s + f) / e;
      if (t < 0) { t = 0; s = clamp(-c / a, 0, 1); } else if (t > 1) { t = 1; s = clamp((bb - c) / a, 0, 1); }
    }
  }
  const onP = v3.mad(p0, d1, s), onB = v3.mad(b0, d2, t);
  return { dist: v3.dist(onP, onB), onB };
}
