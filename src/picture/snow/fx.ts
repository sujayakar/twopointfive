// SNOW IN THE AIR of a level built for snow. What falls is drawn by the particle rain's machinery in another dress: the same ring of GPU slots, the same emit record, the same shadow-tested
// lights and wind field. A flake falls at a walk, so it rides the air almost whole; what is missing of the population is laid through the slab's whole height at once (a fresh disc is full
// now) and a flake that leaves the disc is carried round to its far side. WISPS: fine snow the wind runs low over the open ground in streamers, one more particle of the ring, told by the
// aspect it is filed with. ONE SNOW IN ONE AIR: the flakes and the wisps ride the simulated field among the walls, and a wisp is snow that air lifts where it is fast enough. The picture's
// own; its dice are its own; nothing reads it.
import type { SweptSnow } from '../../ui/sweepSnow';
import { clamp, type Vec3 } from '../../math/vec';
import type { FollowCamera } from '../../host/camera';
import { snowConfig, snowLookConfig, lookConfig } from '../../render/frame';
import type { Particles } from '../../render/particles';
import { RAIN_MAX_BIRTHS, RAIN_CAP } from '../../render/particles';
import { WORLD } from '../../scene/world';
import type { Engine } from '../../engine';
import type { Drawn } from '../../draw/drawn';
import type { DrawWinds } from '../../draw/winds';
import { Air, Section } from '../../draw/sections';
import type { AirPicture } from '../air';
import type { Roofs } from '../rain';
import { SNOWTRACKS, type SnowKick, type SnowTracks } from './tracks';
import type { SnowBlasts } from './blast';
import { SNOW_LOOK } from './look';

/** The numbers (judged on software-rendered stills only — the maintainer's to move; `__snow.fx` on the console, live). Authored data, not player switches. */
export const SNOWFX = {
  perM2: 34.6,        // (the maintainer's, tuned on his GPU 10-15 with the weather's snowfall at 1.48: together far past the cap below — the look he passed is the cap's worth of flakes in view; was 12)
                      // flakes in the air over a square metre of open ground at snowfall 1 (the slab is the weather's rain top: 7 m)
  fallV: 1.25,        // m/s: a flake's terminal fall (× 0.8‥1.2 per flake on the GPU, off its seed)
  size: 0.028,        // m across at size jitter 1 (drawn soft: it reads smaller) — the maintainer's, 10-15 (was 0.05)
  sizeJit: 0.6,
  flutter: 0.41,      // m/s: the sway a flake adds of its own to the air it rides — the maintainer's, 10-15 (was 0.3)
  flutterRate: 2.2,   // rad/s of that sway (× 0.6‥1.6 per flake)
  // how a flake scatters the lights that reach it (the rain's phase function with a snow crystal's numbers: broad, a mild forward lobe, no glint)
  iso: 0.26, fwdK: 0.25, fwdG: 0.45, gain: 9,   // (iso: the maintainer's, 10-15; was 0.75)
  skyK: 1.5,            // × the sky dome's light on a flake (1 = a white scatterer's honest share; over it = the licence that lets snow be seen falling through a yard no lamp reaches)
  brightJit: 0.5,     // a few big bright ones among many fine
  minPx: 1, maxPx: 4, boostCap: 1.5, nearFadeM: 2.6,
  fadeIn: 0.4,        // s: a flake born in mid air comes up, it does not pop
  shutter: 1 / 120,   // s of its motion one flake shows: a dot, not a streak
  windK: 1.0,         // × the air's velocity on the flakes
  sheetAmp: 0.3, sheetLen: 9, sheetDrift: 1.0,   // denser and thinner curtains crossing the yard, carried downwind
  radiusK: 0.75, radiusMin: 10, radiusMax: 30,   // the disc = this × the camera distance (wider than the rain's: a flake drifts metres on its way down)
  past: 12,           // m past the level's rim the snow goes on falling, inside the view's disc (SNOW_LOOK.past; 0 = no flake past the rim) — live
  // ---- ONE SNOW IN ONE AIR (SNOW_LOOK.oneAir; particles.wgsl snowAir): the air a flake falls through — all LIVE
  airLow: 2.0,        // m: under this the air is the simulated field's (the walls' own: slack in a lee, fast through a gap) …
  airHigh: 3.4,       // … over this the open wind; between, blended (a wall's top is at 3 m)
  airLift: 1,         // × the field's lift (air going up and over what it meets)
  skid: 0.75,         // the chance a flake landing where the air is fast enough to lift snow (SNOWWISPS.liftLo ‥ liftHi) skips on along the ground instead of lying (0 = every flake lies where it lands)
};

/** The WISPS' numbers (judged on software-rendered stills only, never seen moving — the maintainer's to move; `__snow.wisps` on the console, live: speedK / opacity /
 *  gain at once, the rest with the next births — a wisp lives a few seconds). Authored data, not player switches. */
export const SNOWWISPS = {
  perM2: 3,           // wisps over a square metre of open ground inside the view's disc at a steady blow (they gather into the gust patches: several times this there, none between)
  max: 1000,          // … and no more than this alive (the ring is the flakes' first: at this many the flakes keep their own cap of 4000)
  life: 4.5,          // s (× 1 ± lifeJit per wisp)
  lifeJit: 0.25,
  y0: 0.04, y1: 0.42, // m: born between these heights over the ground
  size: 0.6,          // m across at birth (× 1 ± sizeJit; it widens by three fifths over its life)
  sizeJit: 0.5,
  speedK: 5,          // × the wind field's air at the ground (WX_OWN.wind: the field's where the wisp IS — nothing in a roof's lee; off: the storm's own vector everywhere open): the snow's weather is a quiet 0.85 m/s (0.5 at the ground) and spindrift at that speed does not read as blowing — LIVE
  stretch: 0.12,      // s of its own motion a wisp is drawn along (× 0.6‥1.4 per wisp): its length = its speed × this (0.3 when it was a combed streak: a puff only leans)
  motes: 0.55,        // 0‥1: A PUFF OF FINE SNOW — how many motes a puff holds (his 10-15 clip of a ground blizzard: "gentle, shimmering puffs of very small, a little
                      // sparkly particles"); 0 = the combed streak of before ("stretched out particle billboards"; with stretch 0.3). Sixteenths; reaches the wisps filed next
  sparkle: 0.5,       // 0‥1: how hard a mote glints now and then (0 = none). Sixteenths too
  opacity: 0.3,       // the streak's peak weight at its core (× 0.5‥1.5 per wisp) — LIVE
  gain: 2,            // × the light a flake would show there — LIVE
  meander: 0.8,       // m/s of crosswind sway on a slow noise the wisps run through (they snake together)
  gustLen: 5,         // m across a gust patch (three times as long down the wind)
  gustPow: 3,         // births × (the storm's gust this frame)^this: they come and go with the wind's own gusting (rust/game_rules/src/air/wind.rs storm_gust_amp)
  minAir: 0.15,       // m/s: under this much storm air nothing blows
  // ---- ONE SNOW IN ONE AIR (SNOW_LOOK.oneAir): a wisp is snow the wind LIFTS off the ground — all LIVE. The speeds are the air's at the ground, in m/s as the
  // wind sim's row shows them (the pass: 0.47 with no gust, 0.3 ‥ 0.63 through one, more where the walls squeeze it). speedK, gustLen, gustPow are not read then.
  liftLo: 0.42,       // m/s: air slower than this lifts nothing …
  liftHi: 0.7,        // … at this every wisp that tries is lifted (between: as many as the air is fast)
  dropK: 0.7,         // × liftLo: in air slower than this a wisp sinks back and is gone
};
/** The GRAINS' numbers (never seen on a GPU, never seen moving — the maintainer's to move; `__snow.grains` on the console: all LIVE but `max`, `size` and
 *  `sizeJit`, which reach the grains filed next — every slot is refiled inside nine seconds). Authored data, not player switches. */
export const SNOWGRAINS = {
  max: 2600,          // grains' slots in the ring: each lies or flies (how many fly is the air's doing — a third to a half in a steady blow, nearly all in a gust)
  radius: 9,          // m about the view's middle they live in; they thin over its outer third, where the wisps take over
  size: 0.02,         // m across (× 1 ± sizeJit): fine snow — on screen a pixel or two
  sizeJit: 0.5,
  speedK: 5,          // × the air field's velocity = a flying grain's (the wisps' own speedK: at the snow's quiet 0.85 m/s of storm nothing reads as blowing; the field itself stays the game's air as it is)
  opacity: 0.5,       // a grain's weight (× 0.5‥1.5 per grain)
  gain: 1.5,          // × the light a flake would show there
  eddy: 0.45,         // the small eddies added on top of the field's air, as a share of that air's speed
  lift: 1,            // × the field's lift (air going up and over what it meets)
  wispsKeep: 0.6,     // × the wisps' cap while the grains run (the ring is shared: at 1 the flakes' cap falls by about a fifth)
  wispsBack: 0,       // 0‥1 how far the wisps stand back from the view's middle while the grains run (1 = thinned to nothing inside the grains' radius, as the
                      // grains first shipped; 0 = wisps everywhere, with the grains: the maintainer, 10-15, "i actually like the far wisps … maybe we could
                      // have both the near and far ones everywhere?") — LIVE
  minAir: 0.15,       // m/s: under this much storm air nothing blows
  // how a flying grain is DRAWN (the maintainer on his GPU, 10-15: "the long wisps do look like a stretched particle" — each was a hard line along 1∕45 s of its
  // flight, 10‥20 cm, all one way): a short dash with a fading tail. All three LIVE on the grains in the air (particles_draw.wgsl grainVertex)
  shutter: 0.008,     // s of its own motion a grain is drawn along (0 = a round dot)
  dash: 3,            // … but never longer than this many of its own widths (0‥15)
  tail: 0.8,          // how far the dash's tail end fades (0 = an even line, 1 = to nothing)
};

/** SNOW PAST THE LEVEL'S RIM (SNOW_LOOK.past) — particles_common.wgsl flakeOpen on the CPU: may a flake be in the air over ground point (x, z)? Under none of
 *  `roofed`'s roofs, and inside the world's rim grown by `past` m; with past 0 it is the rain's own rule (a quarter metre INSIDE the rim), as it was. A flake high in
 *  the air is drawn well off its own ground toward the camera's foot, so with the rim on the camera's side the near part of the picture is snowed on by flakes that
 *  are past the rim. Nothing lies out there: wisps, kicks and prints keep the world's rim. Picture only. */
export function flakeOpen(roofed: (x: number, z: number) => boolean, x: number, z: number, past: number): boolean {
  const x0 = WORLD.min[0], z0 = WORLD.min[2], x1 = WORLD.max[0], z1 = WORLD.max[2];
  if (past <= 0) return x > x0 + 0.25 && z > z0 + 0.25 && x < x1 - 0.25 && z < z1 - 0.25 && !roofed(x, z);
  return x > x0 - past && z > z0 - past && x < x1 + past && z < z1 + past && !roofed(clamp(x, x0 + 0.26, x1 - 0.26), clamp(z, z0 + 0.26, z1 - 0.26));   // (past the rim: the roof rule of the rim beside it)
}
/** is ground point (x, z) past the world's rectangle (particles_common.wgsl pastWorld) */
export const pastWorld = (x: number, z: number): boolean => x < WORLD.min[0] || z < WORLD.min[2] || x > WORLD.max[0] || z > WORLD.max[2];
/** Past the rim the fall thins sooner toward the disc's edge than inside it (shares of the disc's radius; particles_common.wgsl PAST_RIM_IN / PAST_RIM_OUT hold
 *  the same two). The ring's flakes are a fixed number spread over all the open air in the disc, and a flake past the rim shows only over the near part of the
 *  picture: with the level's own taper (0.7‥1) the fall by an edge was a quarter thinner than in the middle of the yard; with this one it is the yard's. */
export const PAST_TAPER = [0.5, 0.8] as const;
/** the share of throws kept at `r01` of the disc's radius, over ground at (x, z): the emit kernel's own acceptance, and the sooner one past the world's rim */
export function flakeTaper(r01: number, x: number, z: number, past: number, sooner: readonly [number, number] = PAST_TAPER): number {
  if (past <= 0 || !pastWorld(x, z)) return rimTaper(r01);
  const t = clamp((r01 - sooner[0]) / (sooner[1] - sooner[0]), 0, 1); return Math.min(rimTaper(r01), 1 - t * t * (3 - 2 * t));
}
/** how far past the rim the snow falls this frame (m): the switch and the number, 0 = not at all */
export const snowPastM = (): number => SNOW_LOOK.past ? clamp(SNOWFX.past, 0, 40) : 0;
export const SNOWPUFF = {
  least: 0.2,   // a kick under this force raises no puff (a thing set down)
  most: 2,      // × a kick's force the row can ask for at most
};
const DOWN: Vec3 = [0, -1, 0];
const num = (v: unknown, d: number) => typeof v === 'number' && Number.isFinite(v) ? v : d;
const rimTaper = (r01: number) => { const t = clamp((r01 - 0.7) / 0.3, 0, 1); return 1 - t * t * (3 - 2 * t); };   // (the emit kernel's own acceptance toward the disc's rim)
const rolled = (serial: number, a: number, salt: number, seed: number): number => { let x = (serial * a) >>> 0; x = (x ^ 61) ^ (x >>> 16); x = (x + (x << 3)) >>> 0; x ^= x >>> 4; x = Math.imul(x, 0x27d4eb2d) >>> 0; x ^= x >>> 15; return (x ^ seed ^ salt) >>> 0; };
const smooth = (a: number, b: number, x: number) => { const t = clamp((x - a) / (b - a), 0, 1); return t * t * (3 - 2 * t); };
/** how a wisp filed now is drawn, in the colour word's fourth byte: SNOWWISPS.motes × 16 + SNOWWISPS.sparkle, in sixteenths */
export const wispLookByte = () => Math.round(clamp(+SNOWWISPS.motes || 0, 0, 1) * 15) * 16 + Math.round(clamp(+SNOWWISPS.sparkle || 0, 0, 1) * 15);
const grainCap = () => clamp(Math.round(SNOWGRAINS.max), 0, 6000);

export class SnowFx {
  private seed = 0x5a0f1a4e; private serial = 1; private birthAcc = 0; private pop = 0; private lastEmitted = -1; private fillAt = -1; private wasActive = false; private bandOff: [number, number] = [0, 0];
  private wSerial = 1; private wAcc = 0; private flakeFiled = 0; private wispsOn = false; private wispBirths = 0;
  private gSerial = 1; private gAcc = 0; private gFill = -1; private gHold = 0; private gSeen = false; private grainsActive = false;
  private kickSeen = 0; private kickQ: SnowKick[] = [];
  /** the disc the flakes wrap in, for the frame's weather: x, z, radius, and the metres past the rim (negative) */
  readonly disc: [number, number, number, number] = [0, 0, 0, 0];
  constructor(private E: Engine, private D: Drawn, private P: Particles | null, private air: AirPicture, private roofs: Roofs, private winds: DrawWinds, private blasts: SnowBlasts) {}
  /** a load, a restart: the ring was emptied with it */
  cut(): void { this.P?.clearRain(); this.wasActive = false; this.birthAcc = 0; this.pop = 0; this.lastEmitted = -1; this.fillAt = -1; this.bandOff[0] = this.bandOff[1] = 0; this.wAcc = 0; this.flakeFiled = 0; this.gAcc = 0; this.gFill = -1; this.gHold = 0; this.gSeen = false; this.kickQ.length = 0; this.kickSeen = 0; }
  private grainsOn(): boolean { return SNOW_LOOK.grains && !SNOW_LOOK.oneAir && snowLookConfig.grains && this.air.live; }
  private oneAirOn(): boolean { return SNOW_LOOK.oneAir && snowLookConfig.grains && this.air.live; }
  private wispsWanted(): boolean { return SNOW_LOOK.wisps && snowLookConfig.wisps; }
  private snowing(): boolean { return (this.E.weather?.snow ?? 0) > 0.001; }
  /** this snow, as the F8 sweep's two rows read and flip it */
  readonly swept: SweptSnow = { look: SNOW_LOOK, wisps: SNOWWISPS, snowing: () => this.snowing(), oneAirOn: () => this.oneAirOn() && this.snowing(), wispsOn: () => this.wispsWanted() && this.snowing(), lookByte: wispLookByte, still: () => this.wispsOn ? '' : 'the air is too slow to lift snow' };
  /** the storm field's share at ground point (x, z), of the roofs in force: what the GPU's own throws then keep of a wisp record (particles.wgsl wispShare) */
  private stormShare(x: number, z: number): number {
    const D = this.D, s = this.winds.active.list.find(w => w.kind === 'storm'); if (!s) return 0;
    const lee = D.f[D.at[Section.air] + Air.stormLee];
    if (s.roofs) { let out = 1e6; for (let i = 0; i < s.roofs.length && i < 4; i++) { const r = s.roofs[i], d = !(r[2] > r[0]) || !(r[3] > r[1]) ? 1e6 : Math.max(Math.max(r[0] - x, x - r[2]), Math.max(r[1] - z, z - r[3])); if (d < out) out = d; } return out <= 0 ? 0 : smooth(0, lee, out); }
    const I = s.interior!, A = s.annex!; if ((x > I[0] && z > I[1] && x < I[2] && z < I[3]) || (x > A[0] && z > A[1] && x < A[2] && z < A[3])) return 0;
    return smooth(0, lee, Math.max(Math.max(I[0] - x, x - I[2]), Math.max(I[1] - z, z - I[3])));
  }
  /** Once a tick, in the rain's place */
  tick(dt: number, cam: FollowCamera, T: SnowTracks | null): void {
    const F = SNOWFX, B = this.blasts, D = this.D, a0 = D.at[Section.air], set = this.E.settings as unknown as Record<string, unknown>, w = this.E.weather, P = this.P;
    B.step(dt);
    const snow = Math.max(0, w?.snow ?? 0), why = !P || snow <= 0.001 || set.particleRain !== true || !snowConfig.on;
    if (why) {
      if (this.wasActive) { P?.clearRain(); this.wasActive = false; this.birthAcc = 0; this.pop = 0; this.fillAt = -1; }
      this.wispsOn = false; this.wAcc = 0; this.flakeFiled = 0; this.grainsOff(); this.disc[2] = 0; if (P) this.oneAirCfg(false);
      return;
    }
    const Pp = P!; this.wasActive = true;
    const kp = Pp.params.rain;
    kp.drag = kp.gravity / F.fallV; kp.sleepV = 0.02; kp.holdOrFade = F.fadeIn; kp.shutterOrGrowth = F.shutter; kp.brightOrOpacity = 1; kp.restitution = num(set.particleRainReact, 6);
    const L = Pp.rainLook;
    L.iso = F.iso; L.fwdK = F.fwdK; L.fwdG = F.fwdG; L.gain = F.gain; L.glintK = F.skyK; L.glintRough = F.flutter; L.glintCap = F.flutterRate; L.tailKeep = 0;
    L.refract = 0; L.refractPaint = 0; L.lenJit = 0; L.brightJit = F.brightJit; L.minPx = F.minPx; L.maxPx = F.maxPx; L.boostCap = F.boostCap; L.nearFadeM = F.nearFadeM; L.beadAmp = 0;
    L.windK = F.windK; L.sheetAmp = F.sheetAmp; L.sheetLen = F.sheetLen;
    // the curtains ride the air the flakes ride
    { const sp = D.f[a0 + Air.stormSpeed], ax = sp > 0 ? D.f[a0 + Air.stormDir] * sp * F.windK : 0, az = sp > 0 ? D.f[a0 + Air.stormDir + 2] * sp * F.windK : 0;
      this.bandOff[0] += ax * F.sheetDrift * dt; this.bandOff[1] += az * F.sheetDrift * dt; L.bandOffX = this.bandOff[0]; L.bandOffZ = this.bandOff[1]; }
    // the disc, and the open ground inside it (1 m cells by their centres, under the rim's taper: what the GPU's per-flake rejection integrates to)
    const rSet = num(set.particleRainRadius, 0), R = rSet > 0 ? clamp(rSet, 3, 40) : clamp(F.radiusK * cam.distance, F.radiusMin, F.radiusMax), cx = cam.target[0], cz = cam.target[2], roofed = this.roofs.under;
    const past = snowPastM(); let area = 0, inside = 0, blown = 0; const wOn = this.wispsWanted(), onField = wOn;
    { const x0 = Math.floor(cx - R), x1 = Math.ceil(cx + R), z0 = Math.floor(cz - R), z1 = Math.ceil(cz + R);
      for (let z = z0; z < z1; z++) for (let x = x0; x < x1; x++) { const px = x + 0.5, pz = z + 0.5, r = Math.hypot(px - cx, pz - cz) / R; if (r >= 1) continue;
        if (!flakeOpen(roofed, px, pz, past)) continue;
        const k = flakeTaper(r, px, pz, past); area += k;
        if (past > 0 && pastWorld(px, pz)) continue;   // (past the rim: the flakes' air only)
        inside += k; if (onField) blown += k * this.stormShare(px, pz); } }
    const top = w.rainTop > 0 ? w.rainTop : 7, want = F.perM2 * snow * area * num(set.particleRainDensity, 1), air = Math.max(0.3, top / F.fallV), life = (top + 0.3) / (F.fallV * 0.8) * 1.1 + 0.4, sheetK = 1 + clamp(F.sheetAmp, 0, 0.95);
    const gOn = this.grainsOn(), oa = this.oneAirOn(), wispMax = SNOWWISPS.max * (gOn && this.grainsActive ? clamp(SNOWGRAINS.wispsKeep, 0, 1) : 1);
    const grainSlots = gOn ? grainCap() * (this.gHold > 0 ? 2.06 : 1.06) + 64 : 0, wispSlots = wOn ? wispMax / Math.max(0.5, SNOWWISPS.life) * (Math.max(life, SNOWWISPS.life * (1 + SNOWWISPS.lifeJit)) + dt + 0.1) : 0;
    const ringMost = (RAIN_CAP * 0.92 - wispSlots - grainSlots) / (sheetK * (1 + (life + dt + 0.1) / air)), cap = Math.max(0, Math.min(num(set.particleRainCount, 4000), ringMost)), target = Math.min(want, cap);
    // the flakes in the air, reckoned: births are counted when the RING has taken them. A cleared ring = none
    { const em = Pp.rainStats.emitted; if (this.lastEmitted >= 0 && em > this.lastEmitted) { this.pop += (wOn || gOn ? Math.min(em - this.lastEmitted, this.flakeFiled) : em - this.lastEmitted) / sheetK; this.flakeFiled = 0; } this.lastEmitted = em; }
    if (Pp.rainStats.live === 0) this.pop = 0;
    this.pop = Math.max(0, this.pop - this.pop * Math.min(1, dt / air));
    const go = area > 0 && dt > 0;
    let room = go ? Math.max(0, Math.min(RAIN_MAX_BIRTHS, Math.floor(RAIN_CAP * 0.95 - Pp.rainStats.live))) : 0;
    this.birthAcc = Math.min(this.birthAcc + (target / air) * sheetK * dt, RAIN_MAX_BIRTHS * 2);
    const trickle = Math.min(Math.floor(this.birthAcc), room); this.birthAcc -= Math.floor(this.birthAcc); room -= trickle;
    // the make-up: one fill at a time, none filed while the last waits on the ring
    const fill = go && this.pop < 0.85 * target && this.fillAt !== Pp.rainStats.emitted ? Math.min(room, Math.ceil((target - this.pop) * sheetK)) : 0;
    this.disc[0] = cx; this.disc[1] = cz; this.disc[2] = area > 0 ? R : 0; this.disc[3] = past > 0 ? -past : 0;
    const rec = (lift: number) => ({ shape: 'rainDisc' as const, radius: R, speed: F.fallV, speedJit: 0.1, life, lifeJit: 0, size: F.size, sizeJit: F.sizeJit, lift, seed: rolled(this.serial++, 2654435761, 0, this.seed) });
    let filed = 0;
    if (trickle > 0) filed += Pp.emit('rain', [cx, top, cz], DOWN, trickle, rec(Math.min(0.6, top)));
    if (fill > 0) { const n = Pp.emit('rain', [cx, top, cz], DOWN, fill, rec(top - 0.25)); if (n > 0) this.fillAt = Pp.rainStats.emitted; filed += n; }
    const births = trickle + fill;
    if (wOn || gOn) this.flakeFiled = Math.min(this.flakeFiled + filed, RAIN_MAX_BIRTHS);
    let left = go ? RAIN_MAX_BIRTHS - births : 0;
    const slack = () => Math.max(0, Math.min(left, Math.floor(RAIN_CAP * 0.985 - Pp.rainStats.live)));
    if (wOn && SNOW_LOOK.blast && dt > 0) left -= B.puffs(Pp, slack());
    if (oa && SNOW_LOOK.blast && dt > 0) left -= B.thrown(Pp, slack(), F.size, F.sizeJit);
    if (SNOW_LOOK.marks && dt > 0) left -= this.markKicks(T, Pp, slack(), F.size, F.sizeJit, cx, cz, R);
    if (wOn) { this.updateWisps(Pp, dt, cx, cz, R, oa ? inside : onField ? blown : inside, left, wispMax, oa); left -= this.wispBirths; }
    else this.wispsOn = false;
    if (gOn) this.updateGrains(Pp, dt, cx, cz, left, life); else this.grainsOff();
    this.oneAirCfg(oa);
  }
  /** THE KICK: the loose snow each mark made since the last tick threw up, filed as FLAKE records, a little finer, born a hand over the mark and going up and on the way the thing was going. The strongest few a tick; the rest wait a tick. The prints' list is only read */
  private markKicks(T: SnowTracks | null, Pp: Particles, room: number, size: number, sizeJit: number, cx: number, cz: number, R: number): number {
    if (!T) return 0;
    if (!T.kicks.length) { this.kickQ.length = 0; this.kickSeen = T.kicked; return 0; }
    if (lookConfig.puffs > 0 && T.kicked >= this.kickSeen) this.bootKicks(T.kicks, this.kickSeen, lookConfig.puffs);
    const Q = this.kickQ.filter(k => T.kicks.includes(k)); for (const k of T.kicks) if (k.n > this.kickSeen) Q.push(k);
    this.kickQ = Q; this.kickSeen = T.kicked;
    if (!Q.length) return 0;
    const K = SNOWTRACKS, near = R * 0.7; let filed = 0, recs = 0, keep = 0;
    Q.sort((a, b) => b.force - a.force || a.serial - b.serial);
    for (const k of Q) {
      if (Math.hypot(k.x - cx, k.z - cz) > near) continue;
      const n = Math.min(Math.round(K.puff * Math.pow(k.force, 1.5)), Math.round(K.puffMost)); if (n <= 0) continue;
      if (n > room - filed) continue;
      if (recs >= K.puffRecs) { if (keep < K.puffWait) Q[keep++] = k; continue; }
      const l = Math.hypot(k.dx, k.dz), dir: [number, number, number] = l > 1e-3 ? [k.dx / l * 0.42, 0.91, k.dz / l * 0.42] : [0, 1, 0];
      filed += Math.max(0, Pp.emit('rain', [k.x, 0.16, k.z], dir, n, { shape: 'rainDisc', radius: Math.max(0.06, k.wide), speed: K.puffUp * (0.5 + 0.5 * k.force), speedJit: 0.7, life: K.puffLife, lifeJit: 0.3, size: size * 0.75, sizeJit, lift: 0.12, seed: (0x51ed27 + k.serial * 40503) >>> 0 })); recs++;
    }
    Q.length = keep;
    return filed;
  }
  /** each NEW kick goes into the look's powder field by its force, where it landed and the way the boot was going */
  private bootKicks(kicks: readonly SnowKick[], seen: number, amount: number): void {
    const field = this.E.powder; if (!field || !(amount > 0)) return;
    const a = Math.min(amount, SNOWPUFF.most);
    for (const k of kicks) { if (k.n <= seen || k.force < SNOWPUFF.least) continue; field.kick(k.x, k.z, k.force * a, k.dx, k.dz); }
  }
  /** WISPS, once a tick behind the flakes: this tick's share of births as ONE record of the flakes' own shape: the disc, the bottom half metre, aspect 2 (what tells the GPU it is a wisp) */
  private updateWisps(Pp: Particles, dt: number, cx: number, cz: number, R: number, area: number, room: number, max: number, oneAir: boolean): void {
    const K = SNOWWISPS, D = this.D, a0 = D.at[Section.air], L = Pp.rainLook; L.refractReachM = oneAir ? SNOWFX.windK : K.speedK; L.refractClamp = K.opacity; L.refractDepth = K.gain;
    const air = D.f[a0 + Air.master] > 0 ? D.f[a0 + Air.stormSpeed] * SNOWFX.windK : 0, peak = oneAir ? this.air.stats.speed * 1.5 : 0;
    this.wispBirths = 0;
    if (!(air >= K.minAir) || !(area > 0) || (oneAir && !(peak > K.liftLo))) { this.wispsOn = false; this.wAcc = 0; return; }
    const blow = oneAir ? 1 : Math.pow(clamp(D.f[a0 + Air.stormGust], 0, 2), K.gustPow), target = Math.min(max, K.perM2 * area * blow * num((this.E.settings as unknown as Record<string, unknown>).particleRainDensity, 1));
    this.wAcc = Math.min(this.wAcc + target / Math.max(0.5, K.life) * dt, 96);
    let n = Math.min(Math.floor(this.wAcc), Math.max(0, room)); this.wAcc -= Math.floor(this.wAcc);
    if (!(dt > 0)) n = 0;
    if (n > 0) n = Pp.emit('rain', [cx, K.y1, cz], DOWN, n, { shape: 'rainDisc', radius: R, speed: 0.01, speedJit: 0, life: K.life, lifeJit: K.lifeJit, size: K.size, sizeJit: K.sizeJit, lift: Math.max(0, K.y1 - K.y0), aspect: 2,
      color: [clamp(K.stretch / 2, 0, 1), clamp(K.meander / 4, 0, 1), clamp(K.gustLen / 32, 0, 1)], top: wispLookByte(), seed: rolled(this.wSerial++, 2246822519, 0x77159, this.seed) });
    this.wispsOn = true; this.wispBirths = n;
  }
  /** ONE SNOW IN ONE AIR, once a tick: ask for the air, and hand the particle step its numbers. Off: [20] = 0 and the step is what it was */
  private oneAirCfg(on: boolean): void {
    const c = this.P?.grainCfg; if (!c || c.length < 28) return;
    if (!on) { c[20] = 0; return; }
    this.air.want();
    const A = this.air.stats, F = SNOWFX, K = SNOWWISPS;
    c[0] = Math.max(A.calm, 1e-3); c[5] = F.airLift; c[7] = 1; c[8] = 0; c[9] = 0; c[10] = 0; c[11] = 0;
    c[20] = 1; c[21] = Math.max(0, K.liftLo); c[22] = Math.max(K.liftHi, K.liftLo + 1e-3); c[23] = clamp(F.skid, 0, 1);
    c[24] = Math.max(0, F.airLow); c[25] = Math.max(F.airHigh, F.airLow + 0.01); c[26] = F.airLift; c[27] = clamp(K.dropK, 0, 1);
  }
  private grainsOff(): void { const c = this.P?.grainCfg; if (c) { c[7] = 0; c[11] = 0; } this.grainsActive = false; this.gAcc = 0; this.gFill = -1; this.gHold = 0; this.gSeen = false; }
  /** GRAINS, once a tick behind the flakes and the wisps (only with one air off): keep `max` grain SLOTS in the ring and hand the GPU this tick's numbers */
  private updateGrains(Pp: Particles, dt: number, cx: number, cz: number, room: number, slotLife: number): void {
    const K = SNOWGRAINS, c = Pp.grainCfg; this.air.want();
    const u0 = this.air.stats.calm, none = !(u0 >= K.minAir);
    c[0] = Math.max(u0, K.minAir); c[1] = Math.max(1, K.radius); c[2] = Math.max(0, K.opacity); c[3] = Math.max(0, K.gain); c[4] = Math.max(0, K.eddy); c[5] = K.lift; c[6] = Math.max(0, K.speedK); c[7] = none ? 0 : 1;
    c[16] = clamp(K.shutter, 0, 0.2); c[17] = clamp(K.dash, 0, 15.9); c[18] = clamp(K.tail, 0, 1); c[19] = clamp(+K.wispsBack || 0, 0, 1);
    const target = none ? 0 : Math.round(grainCap() * clamp(num((this.E.settings as unknown as Record<string, unknown>).particleRainDensity, 1), 0, 1));
    if (Pp.rainStats.live > 0) this.gSeen = true;
    if (none || (this.gSeen && Pp.rainStats.live === 0)) { this.gFill = -1; this.gAcc = 0; this.gSeen = false; }
    this.gHold = Math.max(0, this.gHold - Math.max(0, dt));
    if (!none && dt > 0) {
      if (this.gFill < 0) { this.gFill = target; this.gHold = slotLife + 0.5; }
      const rec = (life: number, lifeJit: number) => ({ shape: 'rainDisc' as const, radius: K.radius, speed: 0.01, speedJit: 0, life, lifeJit, size: K.size, sizeJit: K.sizeJit, lift: 0, aspect: 3, seed: rolled(this.gSerial++, 3266489917, 0x6a41, this.seed) });
      this.gAcc = Math.min(this.gAcc + target / slotLife * dt, 96);
      const trickle = Math.min(Math.floor(this.gAcc), Math.max(0, room)); this.gAcc -= Math.floor(this.gAcc);
      if (trickle > 0) room -= Pp.emit('rain', [cx, 0.3, cz], DOWN, trickle, rec(slotLife, 0));
      const fill = Math.min(this.gFill, Math.max(0, room));
      if (fill > 0) this.gFill -= Pp.emit('rain', [cx, 0.3, cz], DOWN, fill, rec(slotLife * 0.5, 1));
    }
    this.grainsActive = !none;
  }
}
