import { Vec3, v3 } from "../core/math";
import { Smoke } from "./smoke";

// ---------------------------------------------------------------------------
// The effects, as numbers and as the code that spawns them.
//
// Split out of /demo/dynamics so that more than one renderer can show the same
// event. The tuning page draws the density field directly with a raymarch —
// milliseconds a frame, no denoiser, and therefore the right instrument for
// judging MOTION. The path tracer draws it as a participating medium inside a
// full GI solve, which is the only thing that can answer whether the numbers
// are graded right, because the two transfer functions disagree by a factor of
// about sixteen: the raymarch uses sigma_t = look.absorption * rho at 0.035,
// the tracer uses settings.volumetric * rho at 0.55.
//
// A density tuned through one and read through the other is meaningless, and
// two copies of these numbers would drift apart inside a session. So they live
// here, once, and both pages import them.
//
// Nothing in this file touches the GPU. It spawns Smoke sources, advances
// ballistic sparks, and reports what light the event throws; the caller
// decides how any of that is drawn.
// ---------------------------------------------------------------------------

/** 0 plume, 1 burst, 2 vent, 3 muzzle. */
export type EmitterKind = 0 | 1 | 2 | 3;

/**
 * The light an emitter throws, in the form a renderer needs it.
 *
 * Position and colour are per emitter, not global: a flashbang goes off on the
 * floor and is white, a muzzle flash is at the muzzle, warm, and an order of
 * magnitude shorter. `age` counts up; `age >= duration` means dark.
 */
export interface FlashLight {
  x: number; y: number; z: number;
  power: number; duration: number;
  r: number; g: number; b: number;
  age: number;
}

/**
 * The screen. It is meant to OBSCURE, which is a transmittance target rather
 * than a density: exp(-absorption * density * path) along the sightline, at
 * the height a person's eyes are.
 *
 * These values are a look decision — a fast narrow jet with everything wound
 * up — and `stack` is what makes them work. What follows is the measured
 * behaviour of the two controls that fight each other here, because both of
 * them saturate and neither says so.
 *
 * EXPAND IS A DENSITY DIVIDER. The advection pays back exp(-expand * dt) every
 * step, because that is what expanding means, so it caps the density a source
 * can reach and `density` alone cannot climb past the cap. Measured at 3 s,
 * peak density against expand, everything else held:
 *
 *     expand    0     5    10    20    90   245
 *     peak    528   108    63    39    30    30
 *
 * The last two are identical because the volume factor clamps at
 * SRC_DIV_DT/dt = 32/s: above that, the slider changes the RADIUS over which
 * the clamp binds and nothing about the magnitude. Radius (0.22 -> 1.0) and
 * dissipation (0.22 -> 0.05) do not move the ceiling either.
 *
 * But expand cannot be zero: at 0 the column holds peak 528 and never leaves
 * the floor — top at 1.56 m, T = 0.985 at eye height, invisible where it
 * matters. Expansion buys reach and costs density, and that trade is the whole
 * tuning problem for this emitter. `temp` is the other way to buy reach:
 * buoyancy is the only force that carries smoke up once expand is low, and at
 * temp 0 nothing does.
 *
 * For reference, the set that measured best on transmittance alone was
 * radius 0.45, temp 25, expand 5 — T = 0.019 at 1.2 m and 0.021 at 1.7 m,
 * uniform between the two heights. Worth trying against whatever is here now
 * if the screen ever stops reading as opaque.
 */
export const plume = {
  radius: 0.21, density: 600, temp: 0, speed: 20, push: 120,
  expand: 300, seconds: 16.2,
  /**
   * How many identical sources one press spawns.
   *
   * Exists because pressing space three times looked better than any single
   * setting could, and that is not a coincidence: density is ADDITIVE across
   * sources (`forces` sums `density * fall * dt` over all of them) while
   * `expand` is summed and then clamped at SRC_DIV_DT/dt = 32/s, which one
   * source at 300 already saturates on its own. So the second and third press
   * tripled the injection rate and changed the expansion not at all — which is
   * exactly the combination that gets past the density ceiling documented
   * below, and it is unreachable from a `density` slider that stops at 600.
   *
   * Spawning N sources rather than multiplying `density` by N, because `push`
   * does not compose linearly — it is a `mix` toward the source velocity
   * applied once per source per step, so N sources converge on the jet
   * velocity faster than one source with N times the push. Reproducing the
   * thing that was liked exactly is worth three of ninety-six emitter slots.
   */
  stack: 3,
};

/**
 * The flashbang. A detonation is expansion, not momentum: it has no jet
 * velocity and no push, it redistributes what it creates outward and is over
 * in a quarter second. The wisp is a second, much weaker source that survives
 * it, because one source cannot be both the bang and what is left afterwards.
 */
export const burst = {
  // 450/s was tuned against a 25 cm lattice. At 3.125 cm the same rate packs
  // 512x more into a cell, and the measured result was peak density 1135 —
  // an optical depth of ~40/m, so the cloud rendered as an opaque surface with
  // every bit of its structure hidden behind it.
  //
  // Halved again (140 -> 70, and the wisp and trails with it) when the scalar
  // advection stopped exponentiating the pressure solve's residual. That bug
  // was a density amplifier: an identical burst measured peak 473 with it and
  // 17 without, and the mass it invented was concentrated into a few cells, so
  // the cloud read as translucent filaments around hot spots. Corrected, the
  // same mass is spread evenly and 140 renders as one opaque lump. This is the
  // density at which the internal structure reads again — it is a look
  // decision, not a consequence of the fix, and the slider is right there.
  radius: 0.38, density: 70, temp: 32, expand: 90, life: 0.12,
  wispRadius: 0.4, wispDensity: 13, wispTemp: 7, wispRise: 1.2, wispLife: 6,
  /**
   * The canister, not a ball of gas.
   *
   * A flashbang is a cylinder that vents through ports at its ends, so what
   * leaves it is two opposed axial jets and a weaker skirt around the body —
   * never an isotropic sphere. Spawning one spherical source could only ever
   * make a balloon, and no amount of expand or vorticity fixes the shape of
   * something whose shape was decided at the emitter.
   *
   * `seed` picks the can's resting orientation and the per-detonation
   * asymmetry. Deterministic rather than random because this page exists to
   * compare one burst against another, and a burst that differs every run
   * cannot be compared with the one before it — reroll by moving the slider.
   */
  seed: 3,
  /** Half the can's length: how far apart the two end jets sit. */
  halfLength: 0.17,
  /** Speed the end ports vent at. */
  ventSpeed: 14,
  /** 0 both ends equal, 1 one end does everything. Real cans are uneven. */
  asymmetry: 0.45,
  /** Weaker vents around the cylinder's waist. */
  bodyVents: 5,
  bodyFraction: 0.25,
  /** Tilt of the axis away from horizontal, radians. A can lies down. */
  tilt: 0.25,
  /**
   * How much every port is allowed to differ from the ideal.
   *
   * With the ports exactly at the ends, exactly opposed, exactly equal in
   * radius and speed, the result was still too regular: two clean lobes and an
   * evenly spaced skirt read as a machined object venting, not as a casing
   * coming apart. This perturbs each port's direction, its position along the
   * body, its radius and its speed independently — the shape stops being a
   * construction and starts being a sample.
   */
  jitter: 0.6,
  // On the ground, not in mid-air.
  //
  // Reference footage of live flashbangs is unanimous and this was the biggest
  // single error: the charge goes off on a surface. The fireball spreads
  // sideways because it cannot go down, the first second is a low wide bank of
  // smoke rather than a ball, and the column that rises does so FROM the
  // ground. Detonating at 1 m gives a free sphere, which is why it read as a
  // balloon however it was tuned.
  height: 0.12,
};

/** The light the bang throws. See flashAt() in smokeview.wgsl. */
export const flash = {
  power: 156,
  duration: 0.14,
  shadowSteps: 5,
};

/**
 * The light currently in the room, whatever lit it.
 *
 * Separate from the emitter blocks because a flash has a POSITION and a
 * COLOUR as well as a power, and those are properties of the event, not of
 * the page: a flashbang goes off on the floor and is near-white, a muzzle
 * flash is at the muzzle, warm, and roughly a third as long. The shader takes
 * exactly one point light, and this is what `fire()` loads into it.
 *
 * `shadowSteps` stays on `flash` rather than moving here: it is a cost knob
 * for the march, not a property of the thing that went off.
 */
export const flashLive = {
  x: 0, y: 0, z: 0,
  power: 0,
  duration: 0.14,
  /** Seconds since it went off; >= duration means dark. */
  age: 1e9,
  r: 1.0, g: 0.98, b: 0.95,
};

/**
 * The burning fragments.
 *
 * Ballistic, not fluid: they have their own gravity and drag and do not touch
 * the density field at all. In the reference footage they are what makes the
 * first fifth of a second read as an explosion — several hundred fine bright
 * filaments arcing out and falling — and the smoke that follows is a separate,
 * slower event.
 */
export const sparks = {
  count: 260,
  speed: 14,
  spread: 0.55,
  gravity: 16,
  drag: 1.6,
  life: 0.9,
  /** Multiplies the emitted colour; sparks are meant to clip. */
  brightness: 2.6,
  /** Fraction of `life` spent at full brightness before fading out. */
  hold: 0.15,
  /**
   * Tilts the throw from outward (0) toward straight up (1).
   *
   * `spread` already narrows the cone, but narrowing and aiming are different
   * things: a low spread still throws sideways, it just throws sideways in a
   * tighter band. This rotates the whole distribution toward vertical, which
   * is what a can venting against the ground actually does.
   */
  lift: 0.45,
};

/**
 * Smoke trailing the sparks.
 *
 * This replaced a separate "fragment" system that laid its jets out on a
 * Fibonacci sphere. Even coverage was the point of that lattice and it was
 * also its problem: evenly spaced arms read as a symmetrical starburst, and
 * real debris is clumped and uneven. The sparks already have hashed
 * directions, varied speeds, gravity and a floor bounce — they are irregular
 * for free — so hanging the smoke off them costs nothing and removes the
 * symmetry at the source.
 *
 * Only the first `count` sparks smoke: the solver takes FLUID_MAX_SOURCES
 * emitters and the can's ports have already spent some of that budget. What
 * is left is charged against `smoke.free` at the moment the trails spawn, not
 * against a constant — the old `FLUID_MAX_SOURCES - 2` dated from when a burst
 * was one sphere plus a wisp, and with 2 end jets, 5 body vents and the wisp
 * it let `count` promise 90 trails and deliver 88, with Smoke quietly evicting
 * the overflow.
 */
export const trail = {
  count: 90,
  /**
   * The radius, and nothing else touches it.
   *
   * It used to be a floor under a speed-derived term — `max(radius, speed *
   * dt * 0.6)` — meant to stop fast trails beading. At 14 m/s that computes
   * 0.42 m, so a slider set to 0.03 silently produced a 0.84 m blob and the
   * trails rendered as giant sheets an order of magnitude bigger than the
   * sparks drawing them. A control that quietly overrides itself is worse than
   * one that is simply too coarse.
   *
   * The beading it was papering over is real: a moving source deposits once
   * per frame, so a trail is continuous only while `speed * dt` stays under
   * about `2 * radius`. At 50 ms frames that is 0.05 * speed — a 0.06 m radius
   * holds together to ~2.4 m/s and dashes above it. That is now a trade the
   * sliders expose rather than one made silently, and dashes are not
   * necessarily wrong: debris trails are broken in the reference footage too.
   */
  radius: 0.06,
  density: 95,
  temp: 30,
  /**
   * Matched to the sparks' life, so the trail draws the whole arc.
   *
   * The sparks are ballistic — gravity 16, drag, floor bounce — and measured,
   * their mean vertical velocity crosses zero between 0.2 s and 0.4 s and
   * their peak height falls from 2.62 m back to the floor by 1.0 s. At the old
   * 0.5 s the trails stopped emitting almost exactly at the apex, so the smoke
   * recorded the throw and none of the fall, and the arc that is right there
   * in the motion never reached the image.
   */
  life: 0.9,
};
export const MAX_SPARKS = 512;

export const vent = {
  radius: 0.55, density: 200, temp: 0, speed: 9, push: 45,
  expand: 0, seconds: 30, yaw: 0, height: 0.3,
};

/**
 * A shot: the flash, the gas, and what is left hanging in the air.
 *
 * Three separate things on three separate timescales, which is the whole
 * reason this is its own emitter rather than a small burst. The light is over
 * in about 40 ms. The gas jet is over in about 60 ms and is DIRECTIONAL —
 * propellant leaving a barrel at speed, not a sphere expanding — so it should
 * read as a cone thrown forward, not a ball centred on the weapon. The wisp
 * outlives both by a factor of fifty and is the only part a player actually
 * has time to look at, which is why it gets its own radius and rise rather
 * than inheriting the jet's.
 *
 * Values are a starting point picked to be physically sane, not tuned: the
 * jet is short and hot, the wisp is cool and slow, and the flash is a third
 * the length of the flashbang's and much warmer. Expect to move them.
 */
export const muzzle = {
  /** Barrel height and bearing. `standoff` is how far in front the gas exits. */
  height: 1.35,
  yaw: 0,
  standoff: 0.3,

  // ---- the gas jet: fast, hot, and gone in three frames -------------------
  radius: 0.13,
  density: 260,
  temp: 80,
  speed: 16,
  push: 90,
  /**
   * Modest, and deliberately far below the flashbang's 90.
   *
   * Expansion is a density divider — the advection pays back exp(-expand * dt)
   * every step — so a jet that expands hard cannot also be dense, and a muzzle
   * flash is a small DENSE puff, not a spreading cloud. See `plume` for the
   * measured curve.
   */
  expand: 24,
  life: 0.06,

  /**
   * Compensator ports, venting up and to the sides.
   *
   * What makes a muzzle flash read as a weapon rather than a jet of gas: the
   * gas does not all leave through the bore. 0 gives a bare barrel.
   */
  ports: 2,
  portFraction: 0.4,
  /** Port bearing off the bore axis, radians. */
  portAngle: 1.1,

  // ---- the smoke that stays ----------------------------------------------
  //
  // Tight and hot rather than the wide cool puff this started as: at radius
  // 0.07 it is two cells across, so it stays a thread instead of a ball, and
  // at temp 28.5 buoyancy carries it up on its own instead of needing `rise`
  // to throw it.
  wispRadius: 0.07,
  wispDensity: 46,
  wispTemp: 28.5,
  wispRise: 1.2,
  wispLife: 2.5,

  // ---- the light ---------------------------------------------------------
  /** Warm, because burning propellant is: this is roughly 2000 K. */
  flashPower: 46,
  /**
   * Longer than a real muzzle flash by two orders of magnitude, and it has to
   * be, for two reasons that compound.
   *
   * A source deposits NOTHING on its first packed frame: `Smoke.update`
   * computes its attack envelope as min(1, age/attack) and age is zero then.
   * So the gas first exists one frame after `fire()`. And this light is only
   * ever visible THROUGH the medium — `flashAt` is evaluated at density
   * samples, so a flash with nothing to illuminate is a flash that did not
   * happen.
   *
   * At 0.04 the arithmetic came out: at 16 ms frames the light is at 36% on
   * the frame with no smoke and 4% on the first frame with smoke, so it never
   * lit its own gas at all. 0.09 puts it at 41% on the first frame that has
   * something to light and fades over the two after it, which is the whole
   * length of the jet.
   */
  flashDuration: 0.09,

  // ---- unburnt powder ----------------------------------------------------
  //
  // Fast and brief, which is the pair that matters: the trail each spark draws
  // is the segment it covered this frame, so `speed` sets the streak's LENGTH
  // and `life` sets how far it gets before vanishing. At 9 m/s over 0.22 s the
  // powder read as a slow orange spray drifting off the barrel — the arc of a
  // thrown ember, not the snap of gas leaving a bore. 21.5 over 0.11 s covers
  // roughly the same 2.4 m of ground in half the time, so the streaks are
  // twice as long and gone before the gas has finished leaving.
  sparkCount: 44,
  sparkSpeed: 21.5,
  /** Cone half-angle around the bore, radians. Narrow: this is not a burst. */
  sparkCone: 0.35,
  sparkLife: 0.11,
  seed: 5,
};

/**
 * Ballistic sparks: incandescent fragments that do not touch the density field.
 *
 * Flat typed arrays rather than objects, because this is rebuilt into a vertex
 * buffer every frame and a few hundred allocations per frame is exactly the
 * shape of garbage that shows up as a stutter rather than as a slow frame.
 *
 * `verts` is a line list, two vertices per live spark, each xyz + rgb — from
 * where the spark was last frame to where it is now, so a trail's length is
 * its speed and no history buffer is needed.
 */
export class SparkField {
  readonly pos = new Float32Array(MAX_SPARKS * 3);
  readonly prev = new Float32Array(MAX_SPARKS * 3);
  readonly vel = new Float32Array(MAX_SPARKS * 3);
  readonly age = new Float32Array(MAX_SPARKS);
  readonly life = new Float32Array(MAX_SPARKS);
  /** Two vertices per spark, each xyz + rgb. */
  readonly verts = new Float32Array(MAX_SPARKS * 2 * 6);
  count = 0;

  clear(): void { this.count = 0; }

  emitBurst(at: { x: number; y: number; z: number }, seed = burst.seed): void {
    this.count = Math.min(sparks.count, MAX_SPARKS);
    for (let i = 0; i < this.count; i++) {
      // Deterministic: this page is for comparing one burst against another,
      // and Math.random would make every run a different explosion.
      // Seeded from the burst, so rerolling the can rerolls the throw with it.
      // Without this the sparks were identical across every seed, which made
      // the bbox identical too and briefly looked like the seed did nothing.
      const h = (k: number): number => {
        const t = Math.sin((i + 1) * 12.9898 + k * 78.233 + seed * 53.17)
          * 43758.5453;
        return t - Math.floor(t);
      };
      // Upward hemisphere, widened by `spread`. A ground burst throws out and
      // up; nothing goes down through the floor.
      const theta = h(1) * Math.PI * 2;
      let cy = Math.pow(h(2), 1 + sparks.spread * 3) * 0.9 + 0.05;
      let r = Math.sqrt(Math.max(0, 1 - cy * cy));
      // Rotate toward vertical rather than just narrowing: lift 1 puts every
      // spark straight up, 0 leaves the raw hemisphere.
      cy = cy + (1 - cy) * sparks.lift;
      r = Math.sqrt(Math.max(0, 1 - cy * cy));
      const sp = sparks.speed * (0.45 + 0.55 * h(3));
      const o = i * 3;
      this.pos[o] = at.x; this.pos[o + 1] = at.y; this.pos[o + 2] = at.z;
      this.prev[o] = at.x; this.prev[o + 1] = at.y; this.prev[o + 2] = at.z;
      this.vel[o] = Math.cos(theta) * r * sp;
      this.vel[o + 1] = cy * sp;
      this.vel[o + 2] = Math.sin(theta) * r * sp;
      this.age[i] = 0;
      this.life[i] = sparks.life * (0.5 + 0.7 * h(4));
    }
  }

  /**
   * Sparks thrown in a cone about `dir`, for emitters that point somewhere.
   *
   * Separate from `emitSparks` rather than a mode on it. That one models a
   * ground burst — an upward hemisphere shaped by `spread` and rotated toward
   * vertical by `lift` — and those two controls are tuned against reference
   * footage. Folding a direction into it would mean reinterpreting both, and
   * the burst is not the thing being changed here.
   *
   * Writes the same arrays, so `stepSparks` and the trail spawner do not care
   * which one filled them.
   */
  emitCone(
    at: { x: number; y: number; z: number },
    dir: { x: number; y: number; z: number },
    o: { count: number; speed: number; cone: number; life: number; seed: number },
  ): void {
    this.count = Math.min(o.count, MAX_SPARKS);
    // Any vector not parallel to dir will do; pick the axis dir leans on least
    // so the cross product never degenerates.
    const up = Math.abs(dir.y) > 0.9 ? { x: 1, y: 0, z: 0 } : { x: 0, y: 1, z: 0 };
    let ux = up.y * dir.z - up.z * dir.y;
    let uy = up.z * dir.x - up.x * dir.z;
    let uz = up.x * dir.y - up.y * dir.x;
    const ul = Math.hypot(ux, uy, uz) || 1;
    ux /= ul; uy /= ul; uz /= ul;
    const wx = dir.y * uz - dir.z * uy;
    const wy = dir.z * ux - dir.x * uz;
    const wz = dir.x * uy - dir.y * ux;
    const cosMax = Math.cos(o.cone);
    for (let i = 0; i < this.count; i++) {
      const h = (k: number): number => {
        const t = Math.sin((i + 1) * 12.9898 + k * 78.233 + o.seed * 53.17) * 43758.5453;
        return t - Math.floor(t);
      };
      // Uniform over the spherical cap, not uniform in angle: the latter piles
      // sparks on the axis and leaves the rim bare.
      const cy = 1 - h(1) * (1 - cosMax);
      const r = Math.sqrt(Math.max(0, 1 - cy * cy));
      const th = h(2) * Math.PI * 2;
      const ct = Math.cos(th), st = Math.sin(th);
      const sp = o.speed * (0.45 + 0.55 * h(3));
      const j = i * 3;
      this.pos[j] = at.x; this.pos[j + 1] = at.y; this.pos[j + 2] = at.z;
      this.prev[j] = at.x; this.prev[j + 1] = at.y; this.prev[j + 2] = at.z;
      this.vel[j] = (dir.x * cy + ux * r * ct + wx * r * st) * sp;
      this.vel[j + 1] = (dir.y * cy + uy * r * ct + wy * r * st) * sp;
      this.vel[j + 2] = (dir.z * cy + uz * r * ct + wz * r * st) * sp;
      this.age[i] = 0;
      this.life[i] = o.life * (0.5 + 0.7 * h(4));
    }
  }

  step(dt: number): number {
    let verts = 0;
    const decay = Math.exp(-sparks.drag * dt);
    for (let i = 0; i < this.count; i++) {
      if (this.age[i] >= this.life[i]) { continue; }
      const o = i * 3;
      this.prev[o] = this.pos[o]; this.prev[o + 1] = this.pos[o + 1]; this.prev[o + 2] = this.pos[o + 2];
      this.vel[o + 1] -= sparks.gravity * dt;
      this.vel[o] *= decay; this.vel[o + 1] *= decay; this.vel[o + 2] *= decay;
      this.pos[o] += this.vel[o] * dt;
      this.pos[o + 1] += this.vel[o + 1] * dt;
      this.pos[o + 2] += this.vel[o + 2] * dt;
      // Bounce off the floor, losing most of the energy. In the indoor footage
      // the sparks skitter along the ground for as long as they fly.
      if (this.pos[o + 1] < 0.01) { this.pos[o + 1] = 0.01; this.vel[o + 1] *= -0.25; }
      this.age[i] += dt;

      const u = this.age[i] / this.life[i];
      // Hold, then fall away fast. An ember does not fade linearly; it is
      // bright and then it is embers.
      const f = u < sparks.hold ? 1 : Math.max(0, 1 - (u - sparks.hold) / (1 - sparks.hold));
      const e = f * f * sparks.brightness;
      // White-hot to orange to red as it cools.
      const cr = e, cg = e * (0.35 + 0.5 * f), cb = e * (0.08 + 0.25 * f * f);
      const v = verts * 6;
      this.verts[v] = this.prev[o]; this.verts[v + 1] = this.prev[o + 1]; this.verts[v + 2] = this.prev[o + 2];
      this.verts[v + 3] = cr * 0.25; this.verts[v + 4] = cg * 0.25; this.verts[v + 5] = cb * 0.25;
      this.verts[v + 6] = this.pos[o]; this.verts[v + 7] = this.pos[o + 1]; this.verts[v + 8] = this.pos[o + 2];
      this.verts[v + 9] = cr; this.verts[v + 10] = cg; this.verts[v + 11] = cb;
      verts += 2;
    }
    return verts;
  }

  /** Live spark state, for checking the arc is doing what it claims. */
  stats(): { alive: number; maxY: number; meanVy: number; maxR: number } {
    let alive = 0, maxY = 0, meanVy = 0, maxR = 0;
    for (let i = 0; i < this.count; i++) {
      if (this.age[i] >= this.life[i]) { continue; }
      const o = i * 3;
      alive++;
      maxY = Math.max(maxY, this.pos[o + 1]);
      meanVy += this.vel[o + 1];
      maxR = Math.max(maxR, Math.hypot(this.pos[o], this.pos[o + 2]));
    }
    return { alive, maxY, meanVy: alive ? meanVy / alive : 0, maxR };
  }
}


function light(
  x: number, y: number, z: number, power: number, duration: number,
  r: number, g: number, b: number,
): FlashLight {
  return { x, y, z, power, duration, r, g, b, age: 0 };
}

/**
 * Spawns one event's sources and returns the light it throws.
 *
 * The caller owns the Smoke list and the SparkField; this only writes to them.
 * Returning the light rather than mutating a global is what lets two pages
 * with completely different renderers drive the same event.
 */
/**
 * A muzzle flash: bore jet, compensator ports, the wisp that stays, and the
 * light. `pos` is the weapon's muzzle and `dir` the unit vector down the bore;
 * everything is placed relative to those, so the same numbers serve a demo
 * firing along +z at the origin and a guard firing across a room.
 *
 * `radiusScale` exists because a muzzle plume is genuinely about 13 cm across
 * and not every lattice can say so. At the tuning page's 3.125 cm that is four
 * cells and the ports are visibly separate; on the game's 25 cm medium lattice
 * it is half a cell, which is not a small puff but NO puff — the source falls
 * between samples and the shot emits nothing at all. Scaling the radii up is
 * the honest lie: the shape and the proportions survive, the plume is drawn
 * bigger than life because the alternative is drawing it not at all. Pass 1 on
 * anything at 6.25 cm or finer.
 */
export function spawnMuzzle(
  smoke: Smoke, sp: SparkField, pos: Vec3, dir: Vec3, seed = muzzle.seed,
  radiusScale = 1,
): FlashLight {
  let lit: FlashLight;
  // ---- muzzle ---------------------------------------------------------
  const dx = Math.sin(muzzle.yaw), dz = Math.cos(muzzle.yaw);
  const at = v3(dx * muzzle.standoff, muzzle.height, dz * muzzle.standoff);
  // Warm, short, and at the muzzle rather than the origin — the three
  // things that separate a shot from a flashbang as far as the light is
  // concerned. 2000 K propellant is strongly orange; a white flash here
  // reads as a camera going off.
  lit = light(at.x, at.y, at.z,
    muzzle.flashPower, muzzle.flashDuration, 1.0, 0.62, 0.26);

  // The bore. `push` drags the medium along the barrel, which is what
  // makes it a jet rather than a ball that happens to be off-centre.
  smoke.spawn({
    pos: at,
    radius: muzzle.radius * radiusScale,
    vel: v3(dir.x * muzzle.speed, dir.y * muzzle.speed + 0.4, dir.z * muzzle.speed),
    push: muzzle.push,
    density: muzzle.density,
    temp: muzzle.temp,
    expand: muzzle.expand,
    life: muzzle.life,
    attack: 0.01,
  });

  // Compensator ports, symmetric about the bore and angled up: gas that
  // does not leave through the barrel is what stops this reading as a
  // single jet, and it is the part that is visibly a WEAPON.
  for (let i = 0; i < muzzle.ports; i++) {
    const side = i % 2 === 0 ? 1 : -1;
    const a = muzzle.yaw + side * muzzle.portAngle;
    const px = Math.sin(a), pz = Math.cos(a);
    const sp = muzzle.speed * 0.55;
    smoke.spawn({
      pos: v3(
    pos.x + dir.x * muzzle.standoff * 0.7,
    pos.y + dir.y * muzzle.standoff * 0.7,
    pos.z + dir.z * muzzle.standoff * 0.7,
  ),
      radius: muzzle.radius * 0.7 * radiusScale,
      vel: v3(px * sp, sp * 0.5, pz * sp),
      push: muzzle.push * 0.6,
      density: muzzle.density * muzzle.portFraction,
      temp: muzzle.temp * muzzle.portFraction,
      expand: muzzle.expand * 0.5,
      life: muzzle.life,
      attack: 0.01,
    });
  }

  // What is still there a second later, and the only part with time to be
  // looked at. Cool and slow, so buoyancy drifts it rather than throwing it.
  smoke.spawn({
    pos: at,
    radius: muzzle.wispRadius * radiusScale,
    vel: v3(dir.x * 0.5, dir.y * 0.5 + muzzle.wispRise, dir.z * 0.5),
    push: 4,
    density: muzzle.wispDensity,
    temp: muzzle.wispTemp,
    life: muzzle.wispLife,
    attack: 0.15,
  });

  if (muzzle.sparkCount > 0) {
    sp.emitCone(at, { x: dir.x, y: dir.y + 0.08, z: dir.z }, {
      count: muzzle.sparkCount, speed: muzzle.sparkSpeed,
      cone: muzzle.sparkCone, life: muzzle.sparkLife, seed,
    });
  } else {
    sp.count = 0;
  }
  return lit;
}

/**
 * A flashbang canister coming apart at `at`: two opposed end jets, a skirt of
 * body vents, the wisp, and smoke hung off the sparks. `seed` picks the can's
 * resting orientation and its per-detonation asymmetry — the game should vary
 * it per throw, the tuning page holds it fixed so two bursts can be compared.
 */
export function spawnBurst(
  smoke: Smoke, sp: SparkField, at: Vec3, seed = burst.seed,
  maxTrails = trail.count,
): FlashLight {
  let lit: FlashLight;
  
  // The light and the smoke are the same event and start on the same
  // frame; a flash that leads or trails its own cloud reads as two things.
  lit = light(at.x, at.y, at.z, flash.power, flash.duration, 1.0, 0.98, 0.95);

  // ---- the canister ---------------------------------------------------
  const rnd = (k: number): number => {
    const t = Math.sin(seed * 37.31 + k * 91.7) * 43758.5453;
    return t - Math.floor(t);
  };
  // A can lying on the ground: axis mostly horizontal, yaw wherever it
  // came to rest, tilted a little because floors are not billiard tables.
  const yaw = rnd(1) * Math.PI * 2;
  const tilt = (rnd(2) - 0.5) * 2 * burst.tilt;
  const ax = Math.cos(tilt) * Math.sin(yaw);
  const ay = Math.sin(tilt);
  const az = Math.cos(tilt) * Math.cos(yaw);

  // Two opposed end jets, deliberately unequal. In footage one port
  // almost always dominates — the can is against something, or its ends
  // did not open evenly.
  const bias = (rnd(3) - 0.5) * 2 * burst.asymmetry;
  const J = burst.jitter;
  /** Unit vector, perturbed off `d` by up to `amount` radians. */
  const wobble = (
    dx: number, dy: number, dz: number, amount: number, k: number,
  ): [number, number, number] => {
    const nx = dx + (rnd(k) - 0.5) * amount;
    const ny = dy + (rnd(k + 1) - 0.5) * amount;
    const nz = dz + (rnd(k + 2) - 0.5) * amount;
    const l = Math.hypot(nx, ny, nz) || 1;
    return [nx / l, ny / l, nz / l];
  };

  let si = 0;
  for (const side of [1, -1]) {
    si++;
    const w = (1 + side * bias) * (1 + (rnd(40 + si) - 0.5) * J);
    if (w <= 0.02) { continue; }
    // Ports are not exactly at the ends, not exactly opposed, and not the
    // same size as each other.
    const along = burst.halfLength * side * (0.6 + rnd(50 + si) * 0.8);
    const [dx, dy, dz] = wobble(ax * side, ay * side, az * side, J * 0.9, 60 + si * 3);
    const sp = burst.ventSpeed * (1 + (rnd(70 + si) - 0.5) * J * 0.8);
    smoke.spawn({
      pos: v3(at.x + ax * along, at.y + ay * along, at.z + az * along),
      radius: burst.radius * (1 + (rnd(80 + si) - 0.5) * J * 0.7),
      vel: v3(dx * sp, dy * sp, dz * sp),
      push: 40,
      density: burst.density * w,
      temp: burst.temp * w,
      expand: burst.expand,
      life: burst.life * (1 + (rnd(90 + si) - 0.5) * J * 0.5),
      attack: 0.02,
    });
  }

  // Skirt: the body ports, weaker and perpendicular to the axis. This is
  // what stops the two jets reading as a dumbbell.
  const up = Math.abs(ay) > 0.9 ? v3(1, 0, 0) : v3(0, 1, 0);
  const px = up.y * az - up.z * ay, py = up.z * ax - up.x * az;
  const pz = up.x * ay - up.y * ax;
  const pl = Math.hypot(px, py, pz) || 1;
  const qx = ay * (pz / pl) - az * (py / pl);
  const qy = az * (px / pl) - ax * (pz / pl);
  const qz = ax * (py / pl) - ay * (px / pl);
  for (let i = 0; i < burst.bodyVents; i++) {
    const a2 = (i / Math.max(1, burst.bodyVents)) * Math.PI * 2 + rnd(10 + i) * 1.2;
    const c = Math.cos(a2), sn = Math.sin(a2);
    const dx = (px / pl) * c + qx * sn;
    const dy = (py / pl) * c + qy * sn;
    const dz = (pz / pl) * c + qz * sn;
    const w = burst.bodyFraction * (0.5 + rnd(30 + i)) * (1 + (rnd(120 + i) - 0.5) * J);
    // Along the body, not all at the waist.
    const along = (rnd(130 + i) - 0.5) * 2 * burst.halfLength;
    const sp = burst.ventSpeed * 0.6 * (1 + (rnd(140 + i) - 0.5) * J);
    const [wx, wy, wz] = wobble(dx, dy, dz, J * 0.7, 150 + i * 3);
    smoke.spawn({
      pos: v3(at.x + ax * along, at.y + ay * along, at.z + az * along),
      radius: burst.radius * 0.7 * (1 + (rnd(170 + i) - 0.5) * J * 0.8),
      vel: v3(wx * sp, wy * sp, wz * sp),
      push: 30,
      density: burst.density * w,
      temp: burst.temp * w,
      expand: burst.expand * 0.5,
      life: burst.life * (1 + (rnd(180 + i) - 0.5) * J * 0.5),
      attack: 0.02,
    });
  }
  smoke.spawn({
    pos: at, radius: burst.wispRadius, vel: v3(0, burst.wispRise, 0), push: 5,
    density: burst.wispDensity, temp: burst.wispTemp,
    life: burst.wispLife, attack: 0.3,
  });
  sp.emitBurst(at, seed);
  // Smoke hung off the sparks themselves. `follow` reads the spark's live
  // position every frame, so each trail inherits that spark's direction,
  // speed, arc and floor bounce — none of which is symmetric.
  const n = Math.min(maxTrails, trail.count, smoke.free, sp.count);
  for (let i = 0; i < n; i++) {
    const o = i * 3;
    smoke.spawn({
      pos: v3(sp.pos[o], sp.pos[o + 1], sp.pos[o + 2]),
      follow: () => v3(sp.pos[o], sp.pos[o + 1], sp.pos[o + 2]),
      radius: trail.radius,
      density: trail.density,
      temp: trail.temp,
      life: trail.life,
      attack: 0.01,
    });
  }
  return lit;
}

/** A smoke canister venting sideways at `at`, along `yaw`. */
export function spawnVent(smoke: Smoke, at: Vec3, yaw = vent.yaw): void {
  const ax = Math.sin(yaw), az = Math.cos(yaw);
  smoke.spawn({
    pos: at, radius: vent.radius,
    vel: v3(ax * vent.speed, 0.3, az * vent.speed), push: vent.push,
    density: vent.density, temp: vent.temp, expand: vent.expand,
    life: vent.seconds, attack: 0.12,
  });
}

/** A sustained column at `at`. `stack` co-located sources, see plume.stack. */
export function spawnPlume(smoke: Smoke, at: Vec3): void {
// Plume: a small hot source near the floor, which is the case that shows
// buoyancy, vorticity and dissipation all at once. `stack` presses are
// co-located and identical, which is what pressing space N times does.
const n = Math.max(1, Math.min(Math.round(plume.stack), smoke.free));
for (let i = 0; i < n; i++) {
  smoke.spawn({
    pos: at, radius: plume.radius,
    vel: v3(0, plume.speed, 0), push: plume.push,
    density: plume.density, temp: plume.temp, expand: plume.expand,
    life: plume.seconds, attack: 0.12,
  });
}
}

/**
 * The tuning page's dispatcher: places each emitter where /demo/dynamics puts
 * it and returns whatever light it threw.
 *
 * The placement lives here rather than in the spawn functions because it is a
 * property of the PAGE — everything fires at the middle of the box so a
 * scenario and a person see the same event — and not of the effect.
 */
export function fire(
  kind: EmitterKind, smoke: Smoke, sp: SparkField,
): FlashLight | null {
  if (kind === 3) {
    const d = v3(Math.sin(muzzle.yaw), 0, Math.cos(muzzle.yaw));
    return spawnMuzzle(smoke, sp, v3(0, muzzle.height, 0), d);
  }
  if (kind === 1) return spawnBurst(smoke, sp, v3(0, burst.height, 0));
  if (kind === 2) { spawnVent(smoke, v3(0, vent.height, 0)); return null; }
  spawnPlume(smoke, v3(0, 0.18, 0));
  return null;
}
