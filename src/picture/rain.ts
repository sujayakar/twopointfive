// RAIN, NEAR THE VIEW: real GPU drops over the ground the view is looking at (render/particles.ts kind 'rain'), with the lattice of columns standing back by as much under them
// (the frame's nearFade). The rate is the weather's (the sky's section); the disc, the dice and the drips off the edges of what gets rained on are the picture's own. Every frame it
// aims at N drops alive = the lattice's own column density at this rate × the OUTDOOR ground inside a disc round the view's look-at point (capped by the tier), and files one emit
// record for the births. Drips: fat slow drops off the edges of whatever outdoors gets rained on and stands over a drop, found once from the statics, and again when a wall goes.
// The drops ride the wind's records on the GPU; the lattice far off is handed the same air. Nothing here is read by a rule. Its dice are its own.
import { clamp, type Vec3 } from '../math/vec';
import type { FollowCamera } from '../game/camera';
import type { Particles } from '../render/particles';
import { RAIN_MAX_BIRTHS, RAIN_CAP } from '../render/particles';
import { BoxFlag, type Box } from '../scene/boxes';
import { WORLD } from '../scene/world';
import type { Engine } from '../engine';
import type { Drawn } from '../draw/drawn';
import { Air, Rect, Section } from '../draw/sections';
import { PlanRays } from './planRays';

export const PRAIN = {
  // ---- how a drop scatters the lights that reach it (particles.wgsl rainPhase; volumetrics.wgsl rainLight takes iso / fwdK / fwdG too so near and far agree)
  iso: 0.25,          // the isotropic floor of the phase function (1 = v1's grey scatterer exactly; the forward lobe below is 4π-normalised, so iso + fwdK = 1 keeps the all-angle mean at v1's calibration)
  fwdK: 0.75,         // weight of the FORWARD lobe: refraction through the drop throws most light on within a few tens of degrees — a streak between the eye and a lamp flares, one lit from behind the eye is dim
  fwdG: 0.55,         // the lobe's Henyey-Greenstein g (0.55: ×7.8 looking straight into the light through the drop, ×1.3 at 55° off it, ×0.19 with the light behind the eye — a water sphere's transmitted fan reaches out to ~80°, so broad rather than a spike)
  glintK: 1.0,        // the strand GLINT: the water cylinder along the streak mirrors a light where the half vector stands square to the streak (Fresnel × a lobe widened by the emitter's angular size)
  glintRough: 0.08,   // that lobe's own width (+ the emitter's angular half-size per light: Karis' bound — a mirror shows a lamp at the lamp's radiance, never more)
  glintCap: 12,       // and a hard cap per light, × its irradiance × the gain (belt and braces against a firefly)
  gain: 1.9,          // × everything the drop scatters (v1's calibration = 1 at iso 1; the follow camera sees drops lit from its own side, under the lobe: × 1.9 puts them at ~0.8 of v1 there, side-lit 1.1×, 55° off a light 2.3×, straight into it 11×)
  tailKeep: 1.0,      // the streak's TAIL shows last frame's shade × this (1 = the point up the streak lit as it was a frame ago; 0 = head colour all along, v1)
  // ---- what the streak does to what is BEHIND it (particles_draw.wgsl rainFragment: the composite's copy sampled off the streak's line)
  refract: 1.0,       // 0 = off (v1: additive scatter only); 1 = the streak's own coverage REPLACES the background with what the water cylinder refracts of it (refract_common.wgsl — the module shared with the solid-glass lane: Snell in and out of the circular section, the exit ray followed to the scene's depth behind and projected back for the fetch, the two Fresnel transmissions folded in)
  refractReachM: 6,   // m: how far behind the drop the refracted ray is followed at most (the scene depth behind the pixel less the drop's own, capped) — a drop images what lies well off its line, minified with the sides swapped; smaller = subtler
  refractTaps: 3,     // taps ACROSS the cylinder (2 = ±0.55 of its radius · 3 = −0.7 / 0 / +0.7 · 4 = ±0.3, ±0.8), averaged: the pixel spans the whole width
  refractClamp: 6,    // each tap's display-referred luminance is held under this (× display white) — an emitter's face behind a streak must not become a white needle
  refractDepth: 0.3,  // m: a tap landing on something this much NEARER the eye than the drop (Sam's head beside the streak) is not behind it — the centre sample stands in
  refractPaint: 0,    // 0‥1: how much of the PICTURE a refraction read taken BESIDE the streak keeps. A tap is one fixed offset per streak, so with all of it (1 = as it was) a streak whose side tap landed on a red stripe was red from end to end and its neighbour cream: the rain beside The block's striped tent was a displaced picture of the tent (the maintainer, 10-12). 0 = that read hands over its light only (rainRefractTap: no colour of its own — the colour of what is straight behind the streak, else of the light on the drop —, its luminance half way to the pixel's own, its offset's length scattered per pixel and frame); a tap that lands straight BEHIND the streak (the 3-tap set's middle one) is untouched either way
  // ---- the population's spread (particles_draw.wgsl rainVertex, hashed off each drop's seed)
  lenJit: 0.45,       // streak length × (1 ± this) per drop (shutter spread: they do not all fall at one speed to the eye)
  brightJit: 0.7,     // 0 = every drop alike; 1 = a few fat bright ones and many fine (0.5 + 1.5 h², mean 1 — the lattice's `big`)
  minPx: 0.75,        // half-width floor in px: thinner streaks are drawn this wide and dimmed by as much (v1 0.75 = a 1.5 px line)
  maxPx: 1.1,         // half-width CEILING in px: a drop a couple of metres off the lens is a thin bright line, not an 8-px bar — what is wider is drawn this wide and brightened by as much, up to boostCap (v1: no ceiling)
  boostCap: 2.5,      // … the most that narrowing may brighten it
  nearFadeM: 2.6,     // m: a drop nearer the lens than 0.35 × this shows nothing, fully itself from here out (v1: 0.7 → 2 m) — no depth of field, so the ones under the camera's nose go instead of smearing the frame
  beadAmp: 0.4,       // BEADS: a big drop oscillates as it falls (a hundred-odd Hz), so a real streak photographs brightness-modulated along its length — × (1 ± this), 3‥6 beats per streak, phase and count per drop (0 = an even line, v1)
  // ---- WIND (particles.wgsl emit / rainStep ride windAir; the lattice's lean and the sheets are set here on the CPU)
  windK: 1.0,         // × the air's velocity on the drops (0 = they fall plumb whatever blows; 1 = the wind field as the paper and the smoke feel it)
  latticeLean: 0.92,  // the LATTICE far field leans by the storm's air × this ÷ 9 m/s (0.92 = the mean of the height profile a drop falls through; 0 = v1's raw weather shear, opposite in sign to the wind field)
  sheetAmp: 0.5,      // drifting SHEETS: births thinned by (1 + amp·noise)/(1 + amp) on a value noise carried downwind (0 = uniform rain)
  sheetLen: 7,        // m: the sheets' scale
  sheetDrift: 1.0,    // × the mean air speed: how fast they cross the lot
};
export const RAIN_FX = {
  latticeCell: 0.45,        // volumetrics.wgsl RAIN_CELL: two columns a cell …
  latticePeriod: 2.2,       // … RAIN_PERIOD: a drop every 2.2 m down each falling column → columns/m² × top ÷ this = drops alive per m² of ground
  fallV: 9.0,               // terminal fall (m/s) = the lattice's RAIN_V (the particles' drag is set so gravity ÷ drag = this)
  radiusK: 0.6,             // auto radius = this × the camera distance (0.6 × 21 m ≈ the follow view's corners), clamped radiusMin‥Max
  radiusMin: 8, radiusMax: 30,
  size: 0.008,              // streak width (m) at size jitter 1 (× 0.5‥1.5 per drop: they are not all alike — the lattice's `big`)
  sizeJit: 0.5,
  topJit: 0.9,              // born up to this far under the slab's top (no layers falling in step)
  speedJit: 0.06,           // ± on the terminal speed per drop
  splashSecs: 0.2,          // a landing's crown + droplets last this long (particles.ts params.rain.sleepV)
  dripPerPointSec: 0.35,    // drips per second per active edge point at rain rate 1 (× rate)
  dripSize: 2.6,            // × the streak width: fat
  dripSpeed: 0.35,          // m/s as it lets go (the drag / gravity bring it up to the fall speed within a second)
  dripMaxPerFrame: 4,       // records are the scarce thing (128 a frame for everybody): a trickle, never a burst
  edgeStep: 0.3,            // sample spacing along an edge (m)
  edgeMinTop: 0.9, edgeMaxTop: 7.6, edgeMinArea: 0.12, edgeMinFall: 0.35,
};


/** the drip lines are found again a share a tick, once the statics have stood still this long, so that a wall blown costs no one frame */
const DRIP_SAMPLES = 16, DRIP_HOLD = 8;

interface DripPoint { pos: Vec3; fall: number; box: number }
/** the walk where it stopped, the lines found so far, the statics it is over, the samples left of this tick's share */
interface DripJob { boxes: number; serial: number; left: number; hold: number; bi: number; ei: number; k: number; paid: boolean; out: DripPoint[]; rays: PlanRays }

const inRect = (r: ArrayLike<number>, x: number, z: number) => x >= r[0] && z >= r[1] && x <= r[2] && z <= r[3];
const rimTaper = (r01: number) => { const t = clamp((r01 - 0.7) / 0.3, 0, 1); return 1 - t * t * (3 - 2 * t); };
const DOWN: Vec3 = [0, -1, 0];
const num = (v: unknown, d: number) => typeof v === 'number' && Number.isFinite(v) ? v : d;
/** the life a drip off an edge `fall` m up is filed with: a drag-slowed free fall, generously (the landing cuts it to the splash anyway), + the splash */
const dripLife = (fall: number) => Math.sqrt(2 * fall / 9.8) * 1.25 + 0.15 + RAIN_FX.splashSecs;

/** WHAT IS UNDER A ROOF, for the weather's picture: the level's own roofs where a rain falls by them, else the two rectangles */
export class Roofs {
  private rects: number[][] = []; private level = false; private sig = 0; private interior = [0, 0, 0, 0]; private annex = [0, 0, 0, 0];
  constructor(private D: Drawn) {}
  /** the word that moves when the roofs do */
  get word(): number { return this.sig; }
  from(): void {
    const D = this.D, f = D.f, a = D.at[Section.air], n = D.n[Section.roofs]; this.level = D.u[a + Air.levelRoofs] !== 0;
    let s = n + (this.level ? 977 : 0); this.rects.length = n;
    for (let i = 0, o = D.at[Section.roofs]; i < n; i++, o += Rect.row) { const r = this.rects[i] ??= [0, 0, 0, 0]; for (let k = 0; k < 4; k++) { r[k] = f[o + Rect.r + k]; s = (Math.imul(s, 33) + Math.round(r[k] * 64)) | 0; } }
    this.sig = s;
    for (let k = 0; k < 4; k++) { this.interior[k] = f[a + Air.interior + k]; this.annex[k] = f[a + Air.annex + k]; }
  }
  /** under the roofs the weather is cut by now (weather.ts underRoof) */
  under = (x: number, z: number): boolean => { for (const r of this.rects) if (inRect(r, x, z)) return true; return false; };
  /** is this ground outdoors, for the rain */
  outdoors = (x: number, z: number): boolean => !(this.level ? this.under(x, z) : inRect(this.interior, x, z) || inRect(this.annex, x, z)) && x > WORLD.min[0] + 0.25 && z > WORLD.min[2] + 0.25 && x < WORLD.max[0] - 0.25 && z < WORLD.max[2] - 0.25;
}

export class RainPicture {
  private seed = 0x5eed1e5; private birthAcc = 0; private dripAcc = 0; private wasActive = false; private edges: DripPoint[] | null = null; private edgeBoxes = 0; private roofWord = NaN; private job: DripJob | null = null; private serial = 1;
  private pop = 0; private popLog: { t: number; dt: number; n: number }[] = []; private clock = 0; private lastC: [number, number] | null = null; private dripLifeMax = 0;
  private shear: [number, number] = [0, 0]; private bandOff: [number, number] = [0, 0]; private shearInit = false;
  /** what the frame's weather is told, whenever it is next written from the sky's section */
  private nearFade: [number, number, number, number] = [0, 0, 0, 0]; private phase: [number, number, number, number] = [0, 0, 0, 0]; private drawn = false;
  private dripAct: DripPoint[] = [];
  constructor(private E: Engine, private D: Drawn, private P: Particles | null, private boxes: Box[], private roofs: Roofs) {}
  private rnd(): number { this.seed = (this.seed + 0x6D2B79F5) >>> 0; const a = this.seed; let t = Math.imul(a ^ (a >>> 15), 1 | a); t = (t + Math.imul(t ^ (t >>> 7), 61 | t)) ^ t; return ((t ^ (t >>> 14)) >>> 0) / 4294967296; }
  private nextSeed(): number { let x = (this.serial++ * 2654435761) >>> 0; x = (x ^ 61) ^ (x >>> 16); x = (x + (x << 3)) >>> 0; x ^= x >>> 4; x = Math.imul(x, 0x27d4eb2d) >>> 0; x ^= x >>> 15; return x >>> 0; }
  /** a load, a restart: the ring was emptied with it */
  cut(): void { this.P?.clearRain(); this.wasActive = false; this.birthAcc = 0; this.dripAcc = 0; this.pop = 0; this.popLog.length = 0; this.lastC = null; this.shearInit = false; this.bandOff[0] = this.bandOff[1] = 0; this.edges = null; this.job = null; }
  /** the frame's weather, the sky's section just written over it */
  dress(): void { const W = this.E.weather; if (!this.drawn) return; W.nearFade[0] = this.nearFade[0]; W.nearFade[1] = this.nearFade[1]; W.nearFade[2] = this.nearFade[2]; W.nearFade[3] = this.nearFade[3]; W.phase = this.phase; if (PRAIN.latticeLean > 0) { W.shearX = this.shear[0]; W.shearZ = this.shear[1]; } }
  /** Once a tick: the recycler, the drips, what the lattice is told */
  tick(dt: number, cam: FollowCamera): void {
    const set = this.E.settings as unknown as Record<string, unknown>, w = this.E.weather, Pp = this.P, on = set.particleRain === true, rain = Math.max(0, w?.rain ?? 0), D = this.D, a0 = D.at[Section.air];
    if (this.roofs.word !== this.roofWord) { this.roofWord = this.roofs.word; this.edges = null; this.job = null; }
    const why = !on || !Pp || rain <= 0.001;
    if (on && (this.job || !this.edges || this.edgeBoxes !== this.boxes.length)) this.pace(!why && !this.edges);
    if (why) { this.drawn = false; if (this.wasActive) { Pp?.clearRain(); this.wasActive = false; this.birthAcc = 0; this.dripAcc = 0; this.pop = 0; this.popLog.length = 0; this.lastC = null; this.shearInit = false; } return; }
    this.wasActive = true; this.drawn = true;
    const p = Pp!;
    p.params.rain.brightOrOpacity = num(set.particleRainBright, 1); p.params.rain.restitution = num(set.particleRainReact, 6); p.params.rain.sleepV = RAIN_FX.splashSecs; p.params.rain.drag = p.params.rain.gravity / RAIN_FX.fallV;
    const K = PRAIN, L = p.rainLook;
    L.iso = K.iso; L.fwdK = K.fwdK; L.fwdG = K.fwdG; L.glintK = K.glintK; L.glintRough = K.glintRough; L.glintCap = K.glintCap; L.gain = K.gain; L.tailKeep = K.tailKeep;
    L.refract = K.refract; L.refractReachM = K.refractReachM; L.refractTaps = K.refractTaps; L.refractClamp = K.refractClamp; L.refractDepth = K.refractDepth; L.refractPaint = K.refractPaint; L.lenJit = K.lenJit; L.brightJit = K.brightJit; L.minPx = K.minPx; L.maxPx = K.maxPx; L.boostCap = K.boostCap; L.nearFadeM = K.nearFadeM; L.beadAmp = K.beadAmp;
    L.windK = K.windK; L.sheetAmp = K.sheetAmp; L.sheetLen = K.sheetLen;
    { const ph = this.phase; ph[0] = K.iso; ph[1] = K.fwdK; ph[2] = K.fwdG; ph[3] = K.gain; }   // (the lattice takes the same phase, so the two layers flare and dim alike)
    // WIND. The drops ride the wind's records on the GPU; here the lattice far off is handed the same air, as a shear of the opposite sign, eased by the drops' own drag, and the sheets' noise is carried downwind
    { const sp = D.f[a0 + Air.stormSpeed], ax = sp > 0 ? D.f[a0 + Air.stormDir] * sp * K.windK : 0, az = sp > 0 ? D.f[a0 + Air.stormDir + 2] * sp * K.windK : 0;
      if (K.latticeLean > 0) { const tx = -ax * K.latticeLean / RAIN_FX.fallV, tz = -az * K.latticeLean / RAIN_FX.fallV, a = this.shearInit ? 1 - Math.exp(-dt * p.params.rain.drag) : 1; this.shearInit = true; this.shear[0] += (tx - this.shear[0]) * a; this.shear[1] += (tz - this.shear[1]) * a; } else this.shearInit = false;
      this.bandOff[0] += ax * K.sheetDrift * dt; this.bandOff[1] += az * K.sheetDrift * dt; L.bandOffX = this.bandOff[0]; L.bandOffZ = this.bandOff[1]; }
    // the disc round the look-at point, sized to the view
    const rSet = num(set.particleRainRadius, 0), R = rSet > 0 ? clamp(rSet, 3, 40) : clamp(RAIN_FX.radiusK * cam.distance, RAIN_FX.radiusMin, RAIN_FX.radiusMax), cx = cam.target[0], cz = cam.target[2];
    const area = this.area(cx, cz, R);
    const top = w.rainTop > 0 ? w.rainTop : 7, latticePerM2 = rain * (2 / (RAIN_FX.latticeCell * RAIN_FX.latticeCell)) * (top / RAIN_FX.latticePeriod), want = latticePerM2 * area * num(set.particleRainDensity, 1);
    const meanFall = Math.max(0.2, (top - 0.5 * RAIN_FX.topJit) / RAIN_FX.fallV), life = (top + 0.3) / (RAIN_FX.fallV * 0.94) + RAIN_FX.splashSecs + 0.05, lifeMax = Math.max(life, set.particleRainDrips !== false ? this.dripLifeMax : 0);
    const sheetK = 1 + clamp(K.sheetAmp, 0, 0.95), ringMost = RAIN_CAP * meanFall / (lifeMax + dt + 0.1) * 0.92 / sheetK, cap = Math.max(0, Math.min(num(set.particleRainCount, 4000), ringMost)), target = Math.min(want, cap), perSec = target / meanFall;
    // the drops in the air under THIS disc: filled by the births, drained over the mean fall, cut by the overlap when the disc jumps: the lattice stands back only by as much
    this.clock += dt;
    const log = this.popLog;
    if (this.lastC) { const jump = Math.hypot(cx - this.lastC[0], cz - this.lastC[1]); if (jump > 0.05 * R) { const k = Math.max(0, 1 - jump / (1.4 * R)); for (const e of log) e.n *= k; } this.lastC[0] = cx; this.lastC[1] = cz; } else this.lastC = [cx, cz];
    if (p.rainStats.live === 0) log.length = 0;
    while (log.length && this.clock - log[0].t >= meanFall) log.shift();
    this.birthAcc = Math.min(this.birthAcc + perSec * sheetK * dt, RAIN_MAX_BIRTHS * 2);
    let births = Math.floor(this.birthAcc); this.birthAcc -= births;
    births = Math.min(births, RAIN_MAX_BIRTHS - RAIN_FX.dripMaxPerFrame);
    if (!(births > 0 && area > 0 && dt > 0)) births = 0;
    if (births > 0) log.push({ t: this.clock, dt, n: births / sheetK });
    this.pop = 0; for (const e of log) { const older = this.clock - meanFall - (e.t - e.dt); this.pop += e.n * (older > 0 && e.dt > 0 ? clamp(1 - older / e.dt, 0, 1) : 1); }
    let fill = target > 1 ? clamp(this.pop / target, 0, 1) : 0; if (fill > 0.97) fill = 1;
    const kSet = num(set.particleRainLattice, -1), keep = kSet >= 0 ? clamp(kSet, 0, 1) : (want > 1e-3 ? clamp(1 - target / want, 0, 1) : 1), nf = this.nearFade;
    if (area > 0) { nf[0] = cx; nf[1] = cz; nf[2] = R; nf[3] = (1 - keep) * fill; } else nf[3] = 0;
    if (births > 0) p.emit('rain', [cx, top, cz], DOWN, births, { shape: 'rainDisc', radius: R, speed: RAIN_FX.fallV, speedJit: RAIN_FX.speedJit, life, lifeJit: 0, size: RAIN_FX.size, sizeJit: RAIN_FX.sizeJit, lift: RAIN_FX.topJit, seed: this.nextSeed() });
    // drips off the edges inside the disc
    if (set.particleRainDrips !== false && dt > 0 && this.edges) {
      const act = this.dripAct; act.length = 0; const R2 = (R * 0.92) * (R * 0.92);
      for (const d of this.edges) { const dx = d.pos[0] - cx, dz = d.pos[2] - cz; if (dx * dx + dz * dz < R2) act.push(d); }
      if (act.length) {
        this.dripAcc = Math.min(this.dripAcc + RAIN_FX.dripPerPointSec * rain * act.length * dt, RAIN_FX.dripMaxPerFrame * 3);
        let n = Math.min(Math.floor(this.dripAcc), RAIN_FX.dripMaxPerFrame); this.dripAcc -= n;
        while (n-- > 0) { const d = act[Math.min(act.length - 1, Math.floor(this.rnd() * act.length))]; p.emit('rain', d.pos, DOWN, 1, { shape: 'cone', spread: 0.05, speed: RAIN_FX.dripSpeed, speedJit: 0.5, life: dripLife(d.fall), lifeJit: 0, size: RAIN_FX.size * RAIN_FX.dripSize, sizeJit: 0.2, posJit: 0.012, seed: this.nextSeed() }); }
      } else this.dripAcc = 0;
    }
  }
  /** the OUTDOOR ground in the disc, weighted by the rim taper the births are thinned by (1 m cells by their centres) */
  area(cx: number, cz: number, R: number): number {
    let area = 0; const x0 = Math.floor(cx - R), x1 = Math.ceil(cx + R), z0 = Math.floor(cz - R), z1 = Math.ceil(cz + R), out = this.roofs.outdoors;
    for (let z = z0; z < z1; z++) for (let x = x0; x < x1; x++) { const px = x + 0.5, pz = z + 0.5, r = Math.hypot(px - cx, pz - cz) / R; if (r >= 1 || !out(px, pz)) continue; area += rimTaper(r); }
    return area;
  }
  /** the drip lines a share at a time (`now` = all of it: rain is drawn and there are no lines to drip off yet), begun once the statics have stood still; the lines in use are swapped whole when the walk ends */
  private pace(now: boolean): void {
    const n = this.boxes.length, serial = this.E.world.geomSerial;
    if (!this.job || this.job.boxes !== n || this.job.serial !== serial) this.job = { boxes: n, serial, left: 0, hold: this.edges ? DRIP_HOLD : 0, bi: 0, ei: 0, k: 0, paid: false, out: [], rays: new PlanRays(this.boxes) };
    const job = this.job; if (!now && job.hold-- > 0) return;
    job.left = now ? Infinity : DRIP_SAMPLES;
    if (dripWalk(this.boxes, job, this.roofs.outdoors)) { this.edges = job.out; this.edgeBoxes = n; this.job = null; this.dripLifeMax = job.out.reduce((m, d) => Math.max(m, dripLife(d.fall)), 0); }
  }
  /** the drip points as found so far (a check's) */
  get dripPoints(): readonly DripPoint[] | null { return this.edges; }
}

/** every static's TOP EDGES that are outdoors, get rained on (open sky over the surface just inside the edge) and stand over a drop, sampled every edgeStep along each edge, 3.5 cm
 *  proud of the face and a hair under the top. It stops (false) whenever `job.left` samples are used up, and the next call goes on with that same sample */
export function dripWalk(statics: readonly Box[], job: DripJob, outdoors: (x: number, z: number) => boolean): boolean {
  const F = RAIN_FX, out = job.out, rays = job.rays;
  for (; job.bi < statics.length; job.bi++, job.ei = 0, job.k = 0, job.paid = false) { const bi = job.bi, b = statics[bi];
    if (b.flags & (BoxFlag.NoShadow | BoxFlag.Glass | BoxFlag.Dynamic)) continue;
    if (b.rot) continue;
    const top = b.c[1] + b.h[1];
    if (top < F.edgeMinTop || top > F.edgeMaxTop || 4 * b.h[0] * b.h[2] < F.edgeMinArea) continue;
    const c = Math.cos(b.yaw), s = Math.sin(b.yaw), toWorld = (lx: number, lz: number): [number, number] => [b.c[0] + c * lx + s * lz, b.c[2] - s * lx + c * lz];
    const edges: [number, number, number, number, number][] = [[1, b.h[2], 0, 1, b.h[0]], [1, -b.h[2], 0, -1, b.h[0]], [0, b.h[0], 1, 0, b.h[2]], [0, -b.h[0], -1, 0, b.h[2]]];
    for (; job.ei < edges.length; job.ei++, job.k = 0, job.paid = false) { const [alongX, fixed, nx, nz, half] = edges[job.ei], len = 2 * half, n = Math.max(1, Math.round(len / F.edgeStep));
      for (; job.k < n; job.k++) { const u = -half + (job.k + 0.5) * len / n, lx = alongX ? u : fixed, lz = alongX ? fixed : u;
        const [qx, qz] = toWorld(lx + nx * 0.035, lz + nz * 0.035), [ix, iz] = toWorld(lx - nx * 0.06, lz - nz * 0.06), out0 = outdoors(qx, qz);
        if (job.paid) job.paid = false; else if ((job.left -= out0 ? 1 : 1 / 16) < 0) { job.paid = true; return false; }
        if (!out0) continue;
        if (rays.inside(qx, top - 0.02, qz, bi)) continue;          // (a ledge tucked into a bigger box)
        if (rays.covered(ix, top + 0.05, iz, 9.5)) continue;        // (a canopy keeps it dry: no water to come off it)
        const fall = rays.down(qx, top - 0.02, qz, 9);              // (the first solid straight down, at least edgeMinFall below)
        if (!Number.isFinite(fall) || fall < F.edgeMinFall) continue;
        out.push({ pos: [qx, top - 0.012, qz], fall, box: bi });
      }
    }
  }
  return true;
}
