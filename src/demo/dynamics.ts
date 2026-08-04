import { mat4Invert, mat4LookAt, mat4Mul, mat4Perspective, v3 } from "../core/math";
import { GPUContext, initGPU } from "../engine/gpu";
import { FLUID_MAX_SOURCES, FLUID_SOURCE_STRIDE, FluidSim } from "../engine/fluid";
import { Smoke } from "../game/smoke";
import { ControlSpec, GroupSpec, TweakPanel } from "../ui/panel";
import viewSrc from "../shaders/smokeview.wgsl?raw";
import sparkSrc from "../shaders/sparks.wgsl?raw";

// ---------------------------------------------------------------------------
// /demo/dynamics — the fluid solver and nothing else.
//
// No path tracer, no BVH, no denoiser, no GI, no level. One fluid lattice and
// one fragment shader that marches it. That is the whole page.
//
// It exists because tuning smoke MOTION through the real renderer does not
// work. Measured: 2.9 s per frame under software rasterisation with bounces at
// zero and reservoir reuse off, because ~14 compute passes a frame cost what
// they cost regardless of resolution. And the tracer's output then goes through
// temporal accumulation and an a-trous filter whose purpose is to destroy
// high-frequency spatial detail — the exact thing that distinguishes smoke from
// porridge. So the dynamics were being judged through a machine designed to
// hide them, one look every few minutes.
//
// Here a frame is milliseconds and the density field is shown as it is. When
// the motion is right, the numbers move to game/smoke.ts and the look is the
// renderer's problem again.
// ---------------------------------------------------------------------------

/**
 * 4 x 3 x 4 m at 3.125 cm — 128 x 96 x 128.
 *
 * Twice the linear resolution of the grenade page's lattice over a quarter of
 * the volume. Resolution is the dominant lever on whether a cloud has internal
 * structure at all: below roughly a hundred cells across the source there is
 * nothing for vorticity to curl and every scheme produces the same lozenge.
 */
const DIMS: [number, number, number] = [128, 96, 128];
const CELL = 0.03125;
const ORIGIN: [number, number, number] = [
  -(DIMS[0] * CELL) / 2, 0, -(DIMS[2] * CELL) / 2,
];

const VIEW_BYTES = 208;

/**
 * Absorption is per unit density per metre, and the solver's densities are
 * ~100, not ~1.
 *
 * The first pass used 1.4, which is the right order for a normalised field and
 * gives an optical depth of ~155 per metre here: opaque within one step, fully
 * self-shadowed, and therefore a black silhouette against a dark sky. The
 * cloud was there — mass 5.1, peak 111 — and completely invisible.
 *
 * 0.035 puts a 0.5 m thick core at an optical depth around 2, which is where a
 * cloud reads as dense but still lets its lit rim through.
 */
const look = {
  steps: 96,
  shadowSteps: 6,
  absorption: 0.035,
  scatter: 1.0,
  // Chosen by looking, not by theory: at 0.12/1.1 the cloud was technically
  // present and visually absent. These are the values the first frame that
  // actually read as smoke was captured at.
  ambient: 0.28,
  exposure: 2.4,
  densityScale: 1.0,
  /**
   * Fraction of the backing store actually rendered.
   *
   * This page is pure raymarch and its cost is per PIXEL: `steps` samples a
   * ray, and every sample whose density clears the threshold pays
   * `shadowSteps` more toward the key light and `flash.shadowSteps` more
   * toward the bang. At devicePixelRatio 2 a 1280x720 window is a 2560x1440
   * backing store, so 3.7 M pixels x 96 steps is the whole frame budget and
   * nothing else on the page is close.
   *
   * Which also means the cost depends on what is ON SCREEN, not just on the
   * settings: the march breaks once transmittance drops below 0.003 and skips
   * the shadow taps in empty cells, so a dense compact cloud is CHEAPER than
   * the thin haze that fills the box a few seconds after a burst. The worst
   * frame is the one that looks like the least.
   */
  renderScale: 1.0,
};

const solver = {
  vorticity: 6.0,
  buoyancy: 6.0,
  weight: 0.3,
  dissipation: 0.22,
  cooling: 3.0,
  jacobi: 20,
};

/**
 * Knobs that are not `FluidSim.tune` fields, kept out of `solver` so the
 * blanket `Object.assign(fluid.tune, solver)` stays a clean copy.
 *
 * `openFaces` is here rather than hardcoded because the lateral boundary is
 * the one part of the solver this page exercises that the game's coarse
 * lattice does not, and "does the bug survive closing the walls" is the single
 * measurement that localises anything wrong with it.
 */
const dbg = { openFaces: 0xf };

/**
 * One parameter block per emitter, not one shared block.
 *
 * They were shared, and the shared set was really the plume's: `speed`, `push`
 * and `seconds` did nothing at all for a burst, whose life was hardcoded, and
 * `expand` fell back to 70 whenever it was set to zero — so half the panel was
 * inert and one control lied. A burst and a plume have different mechanisms;
 * giving them one set of sliders only hides which ones matter.
 */
// Burst is the default: it is the effect under development, and opening on
// the plume meant every session started by changing a dropdown.
let kind = 1; // 0 plume, 1 burst, 2 vent, 3 muzzle

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
const plume = {
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
const burst = {
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
const flash = {
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
const flashLive = {
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
const sparks = {
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
const trail = {
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
const MAX_SPARKS = 512;

const vent = {
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
const muzzle = {
  /** Barrel height and bearing. `standoff` is how far in front the gas exits. */
  height: 1.35,
  yaw: 0,
  standoff: 0.3,

  // ---- the gas jet: fast, hot, and gone in three frames -------------------
  radius: 0.13,
  density: 260,
  temp: 26,
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
  wispRadius: 0.17,
  wispDensity: 46,
  wispTemp: 6,
  wispRise: 0.7,
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
  sparkCount: 44,
  sparkSpeed: 9,
  /** Cone half-angle around the bore, radians. Narrow: this is not a burst. */
  sparkCone: 0.35,
  sparkLife: 0.22,
  seed: 5,
};

const cam = { yaw: 0.6, pitch: 0.22, distance: 4.2, height: 1.2, orbit: true, speed: 0.25 };

function fatal(title: string, body: string): void {
  const el = document.getElementById("fatal");
  if (!el) return;
  document.getElementById("fatal-title")!.textContent = title;
  document.getElementById("fatal-body")!.textContent = body;
  el.classList.add("show");
}

const sl = (
  label: string, min: number, max: number, step: number,
  get: () => number, set: (v: number) => void,
): ControlSpec => ({ kind: "slider", label, min, max, step, get, set });

const tg = (
  label: string, get: () => boolean, set: (v: boolean) => void,
): ControlSpec => ({ kind: "toggle", label, get, set });

async function main(): Promise<void> {
  const app = document.getElementById("app")!;
  const canvas = document.createElement("canvas");
  app.appendChild(canvas);

  let ctx: GPUContext;
  try {
    ctx = await initGPU(canvas, {
      onDeviceLost: (i) => fatal("GPU device lost", i.message || String(i.reason)),
      onUncapturedError: (e) => console.error("[webgpu]", e.message),
    });
  } catch (e) {
    fatal("Startup failed", String(e));
    return;
  }
  const d = ctx.device;

  // The interface texture the solver writes its density into. Nothing samples
  // it but this page, so it is sized to the lattice exactly.
  const volume = d.createTexture({
    label: "dyn-volume",
    dimension: "3d",
    size: { width: DIMS[0], height: DIMS[1], depthOrArrayLayers: DIMS[2] },
    format: "rgba16float",
    usage: GPUTextureUsage.STORAGE_BINDING | GPUTextureUsage.TEXTURE_BINDING
      | GPUTextureUsage.COPY_DST | GPUTextureUsage.COPY_SRC,
  });

  // No occupancy baker: there is no level here, so the solver sees an empty
  // box with six walls, which is what a smoke box should be.
  const fluid = await FluidSim.create(d, {
    volume, dims: DIMS, origin: ORIGIN, cell: CELL,
  }, null);
  Object.assign(fluid.tune, solver);
  // Lateral outflow: the four side walls stop being walls.
  //
  // A closed box conserves everything, so a burst that reaches the sides piles
  // up against them and the pressure solve pushes it back in — energy the room
  // never had, arriving from a boundary that is an arbitrary cut through open
  // air. Bits 0-3 are -x/+x/-z/+z. Floor and ceiling stay solid on purpose:
  // those are real surfaces, the burst sits on one and the column should still
  // spread against the other.
  fluid.openFaces = 0xf;

  const smoke = new Smoke();

  // ---- sparks -------------------------------------------------------------
  // Flat arrays rather than objects: this is rebuilt into a vertex buffer every
  // frame, and a few hundred allocations per frame is exactly the shape of
  // garbage that shows up as a stutter rather than as a slow frame.
  const spPos = new Float32Array(MAX_SPARKS * 3);
  const spPrev = new Float32Array(MAX_SPARKS * 3);
  const spVel = new Float32Array(MAX_SPARKS * 3);
  const spAge = new Float32Array(MAX_SPARKS);
  const spLife = new Float32Array(MAX_SPARKS);
  let spCount = 0;
  /** Two vertices per spark, each xyz + rgb. */
  const spVerts = new Float32Array(MAX_SPARKS * 2 * 6);

  function emitSparks(at: { x: number; y: number; z: number }): void {
    spCount = Math.min(sparks.count, MAX_SPARKS);
    for (let i = 0; i < spCount; i++) {
      // Deterministic: this page is for comparing one burst against another,
      // and Math.random would make every run a different explosion.
      // Seeded from the burst, so rerolling the can rerolls the throw with it.
      // Without this the sparks were identical across every seed, which made
      // the bbox identical too and briefly looked like the seed did nothing.
      const h = (k: number): number => {
        const t = Math.sin((i + 1) * 12.9898 + k * 78.233 + burst.seed * 53.17)
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
      spPos[o] = at.x; spPos[o + 1] = at.y; spPos[o + 2] = at.z;
      spPrev[o] = at.x; spPrev[o + 1] = at.y; spPrev[o + 2] = at.z;
      spVel[o] = Math.cos(theta) * r * sp;
      spVel[o + 1] = cy * sp;
      spVel[o + 2] = Math.sin(theta) * r * sp;
      spAge[i] = 0;
      spLife[i] = sparks.life * (0.5 + 0.7 * h(4));
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
  function emitSparkCone(
    at: { x: number; y: number; z: number },
    dir: { x: number; y: number; z: number },
    o: { count: number; speed: number; cone: number; life: number; seed: number },
  ): void {
    spCount = Math.min(o.count, MAX_SPARKS);
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
    for (let i = 0; i < spCount; i++) {
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
      spPos[j] = at.x; spPos[j + 1] = at.y; spPos[j + 2] = at.z;
      spPrev[j] = at.x; spPrev[j + 1] = at.y; spPrev[j + 2] = at.z;
      spVel[j] = (dir.x * cy + ux * r * ct + wx * r * st) * sp;
      spVel[j + 1] = (dir.y * cy + uy * r * ct + wy * r * st) * sp;
      spVel[j + 2] = (dir.z * cy + uz * r * ct + wz * r * st) * sp;
      spAge[i] = 0;
      spLife[i] = o.life * (0.5 + 0.7 * h(4));
    }
  }

  function stepSparks(dt: number): number {
    let verts = 0;
    const decay = Math.exp(-sparks.drag * dt);
    for (let i = 0; i < spCount; i++) {
      if (spAge[i] >= spLife[i]) { continue; }
      const o = i * 3;
      spPrev[o] = spPos[o]; spPrev[o + 1] = spPos[o + 1]; spPrev[o + 2] = spPos[o + 2];
      spVel[o + 1] -= sparks.gravity * dt;
      spVel[o] *= decay; spVel[o + 1] *= decay; spVel[o + 2] *= decay;
      spPos[o] += spVel[o] * dt;
      spPos[o + 1] += spVel[o + 1] * dt;
      spPos[o + 2] += spVel[o + 2] * dt;
      // Bounce off the floor, losing most of the energy. In the indoor footage
      // the sparks skitter along the ground for as long as they fly.
      if (spPos[o + 1] < 0.01) { spPos[o + 1] = 0.01; spVel[o + 1] *= -0.25; }
      spAge[i] += dt;

      const u = spAge[i] / spLife[i];
      // Hold, then fall away fast. An ember does not fade linearly; it is
      // bright and then it is embers.
      const f = u < sparks.hold ? 1 : Math.max(0, 1 - (u - sparks.hold) / (1 - sparks.hold));
      const e = f * f * sparks.brightness;
      // White-hot to orange to red as it cools.
      const cr = e, cg = e * (0.35 + 0.5 * f), cb = e * (0.08 + 0.25 * f * f);
      const v = verts * 6;
      spVerts[v] = spPrev[o]; spVerts[v + 1] = spPrev[o + 1]; spVerts[v + 2] = spPrev[o + 2];
      spVerts[v + 3] = cr * 0.25; spVerts[v + 4] = cg * 0.25; spVerts[v + 5] = cb * 0.25;
      spVerts[v + 6] = spPos[o]; spVerts[v + 7] = spPos[o + 1]; spVerts[v + 8] = spPos[o + 2];
      spVerts[v + 9] = cr; spVerts[v + 10] = cg; spVerts[v + 11] = cb;
      verts += 2;
    }
    return verts;
  }

  // ---- the view pass ------------------------------------------------------
  const mod = d.createShaderModule({ label: "smokeview", code: viewSrc });
  // Surfaced explicitly. A shader that fails to compile makes the pipeline
  // invalid, which invalidates the whole command buffer at submit — including
  // the fluid step sharing it. The symptom is a solver that silently does
  // nothing, which is a long way from the cause.
  {
    const info = await mod.getCompilationInfo();
    const bad = info.messages.filter((m) => m.type === "error");
    for (const m of info.messages) {
      console[m.type === "error" ? "error" : "warn"](
        `[smokeview] ${m.type} ${m.lineNum}:${m.linePos} ${m.message}`);
    }
    if (bad.length) {
      fatal("smokeview failed to compile", bad.map((m) => `${m.lineNum}: ${m.message}`).join("\n"));
      return;
    }
  }
  const viewBuf = d.createBuffer({
    label: "smokeview-params",
    size: VIEW_BYTES,
    usage: GPUBufferUsage.UNIFORM | GPUBufferUsage.COPY_DST,
  });
  const sampler = d.createSampler({
    magFilter: "linear", minFilter: "linear",
    addressModeU: "clamp-to-edge", addressModeV: "clamp-to-edge",
    addressModeW: "clamp-to-edge",
  });
  const layout = d.createBindGroupLayout({
    label: "smokeview",
    entries: [
      { binding: 0, visibility: GPUShaderStage.FRAGMENT, buffer: { type: "uniform" } },
      {
        binding: 1, visibility: GPUShaderStage.FRAGMENT,
        texture: { sampleType: "float", viewDimension: "3d" },
      },
      { binding: 2, visibility: GPUShaderStage.FRAGMENT, sampler: { type: "filtering" } },
    ],
  });
  const pipeline = d.createRenderPipeline({
    label: "smokeview",
    layout: d.createPipelineLayout({ bindGroupLayouts: [layout] }),
    vertex: { module: mod, entryPoint: "vs" },
    fragment: { module: mod, entryPoint: "fs", targets: [{ format: ctx.format }] },
    primitive: { topology: "triangle-list" },
  });
  // The solver ping-pongs its scalar textures, so the density view can change
  // identity across a reset. Rebuilt whenever that happens rather than cached.
  let bindGroup = makeBindGroup();
  function makeBindGroup(): GPUBindGroup {
    return d.createBindGroup({
      label: "smokeview",
      layout,
      entries: [
        { binding: 0, resource: { buffer: viewBuf } },
        { binding: 1, resource: fluid.densityTexture.createView({ dimension: "3d" }) },
        { binding: 2, resource: sampler },
      ],
    });
  }

  // ---- spark pipeline -----------------------------------------------------
  const spMod = d.createShaderModule({ label: "sparks", code: sparkSrc });
  {
    const info = await spMod.getCompilationInfo();
    const bad = info.messages.filter((m) => m.type === "error");
    for (const m of bad) console.error(`[sparks] ${m.lineNum}: ${m.message}`);
    if (bad.length) { fatal("sparks failed to compile", bad[0].message); return; }
  }
  const spBuf = d.createBuffer({
    label: "spark-verts",
    size: spVerts.byteLength,
    usage: GPUBufferUsage.VERTEX | GPUBufferUsage.COPY_DST,
  });
  const spUniform = d.createBuffer({
    label: "spark-view", size: 64,
    usage: GPUBufferUsage.UNIFORM | GPUBufferUsage.COPY_DST,
  });
  const spLayout = d.createBindGroupLayout({
    label: "sparks",
    entries: [{ binding: 0, visibility: GPUShaderStage.VERTEX, buffer: { type: "uniform" } }],
  });
  const spBind = d.createBindGroup({
    label: "sparks", layout: spLayout,
    entries: [{ binding: 0, resource: { buffer: spUniform } }],
  });
  const spPipeline = d.createRenderPipeline({
    label: "sparks",
    layout: d.createPipelineLayout({ bindGroupLayouts: [spLayout] }),
    vertex: {
      module: spMod, entryPoint: "vs",
      buffers: [{
        arrayStride: 24,
        attributes: [
          { shaderLocation: 0, offset: 0, format: "float32x3" },
          { shaderLocation: 1, offset: 12, format: "float32x3" },
        ],
      }],
    },
    fragment: {
      module: spMod, entryPoint: "fs",
      targets: [{
        format: ctx.format,
        // Additive: an ember is emissive, so nothing it crosses should dim it.
        blend: {
          color: { srcFactor: "one", dstFactor: "one", operation: "add" },
          alpha: { srcFactor: "one", dstFactor: "one", operation: "add" },
        },
      }],
    },
    primitive: { topology: "line-list" },
  });

  const viewData = new ArrayBuffer(VIEW_BYTES);
  const vf = new Float32Array(viewData);

  function resize(): void {
    const dpr = Math.min(devicePixelRatio || 1, 2) * look.renderScale;
    canvas.width = Math.max(2, Math.floor(canvas.clientWidth * dpr));
    canvas.height = Math.max(2, Math.floor(canvas.clientHeight * dpr));
  }
  resize();
  new ResizeObserver(resize).observe(canvas);

  // ---- camera -------------------------------------------------------------
  // Built with the project's own matrix helpers rather than by hand.
  //
  // The first pass derived inv(proj) analytically and got it wrong in a way
  // that is easy to miss and hard to read: every pixel came out with the same
  // ray direction, so the frame was a single flat colour with no gradient and
  // no box hit. It looked like the volume was empty. Compose the forward
  // matrix from the same lookAt/perspective the game uses and invert it — the
  // one operation here that has already been verified elsewhere.
  function viewProjInverse(): { inv: Float32Array; fwd: Float32Array; eye: number[] } {
    const cp = Math.cos(cam.pitch), sp = Math.sin(cam.pitch);
    const eye = v3(
      Math.sin(cam.yaw) * cp * cam.distance,
      cam.height + sp * cam.distance,
      Math.cos(cam.yaw) * cp * cam.distance,
    );
    const view = mat4LookAt(eye, v3(0, cam.height, 0), v3(0, 1, 0));
    const proj = mat4Perspective(1.05, canvas.width / Math.max(canvas.height, 1), 0.05);
    const fwd = mat4Mul(proj, view);
    return { inv: mat4Invert(fwd), fwd, eye: [eye.x, eye.y, eye.z] };
  }

  // ---- emitters -----------------------------------------------------------
  /** Arms the one point light the shader has, from whatever just went off. */
  function litBy(
    x: number, y: number, z: number,
    power: number, duration: number,
    r: number, g: number, b: number,
  ): void {
    flashLive.x = x; flashLive.y = y; flashLive.z = z;
    flashLive.power = power; flashLive.duration = duration;
    flashLive.r = r; flashLive.g = g; flashLive.b = b;
    flashLive.age = 0;
  }

  function fire(): void {
    if (kind === 3) {
      // ---- muzzle ---------------------------------------------------------
      const dx = Math.sin(muzzle.yaw), dz = Math.cos(muzzle.yaw);
      const at = v3(dx * muzzle.standoff, muzzle.height, dz * muzzle.standoff);
      // Warm, short, and at the muzzle rather than the origin — the three
      // things that separate a shot from a flashbang as far as the light is
      // concerned. 2000 K propellant is strongly orange; a white flash here
      // reads as a camera going off.
      litBy(at.x, at.y, at.z,
        muzzle.flashPower, muzzle.flashDuration, 1.0, 0.62, 0.26);

      // The bore. `push` drags the medium along the barrel, which is what
      // makes it a jet rather than a ball that happens to be off-centre.
      smoke.spawn({
        pos: at,
        radius: muzzle.radius,
        vel: v3(dx * muzzle.speed, 0.4, dz * muzzle.speed),
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
          pos: v3(dx * muzzle.standoff * 0.7, muzzle.height, dz * muzzle.standoff * 0.7),
          radius: muzzle.radius * 0.7,
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
        radius: muzzle.wispRadius,
        vel: v3(dx * 0.5, muzzle.wispRise, dz * 0.5),
        push: 4,
        density: muzzle.wispDensity,
        temp: muzzle.wispTemp,
        life: muzzle.wispLife,
        attack: 0.15,
      });

      if (muzzle.sparkCount > 0) {
        emitSparkCone(at, { x: dx, y: 0.08, z: dz }, {
          count: muzzle.sparkCount, speed: muzzle.sparkSpeed,
          cone: muzzle.sparkCone, life: muzzle.sparkLife, seed: muzzle.seed,
        });
      } else {
        spCount = 0;
      }
      return;
    }
    if (kind === 1) {
      const at = v3(0, burst.height, 0);
      // The light and the smoke are the same event and start on the same
      // frame; a flash that leads or trails its own cloud reads as two things.
      litBy(at.x, at.y, at.z, flash.power, flash.duration, 1.0, 0.98, 0.95);

      // ---- the canister ---------------------------------------------------
      const rnd = (k: number): number => {
        const t = Math.sin(burst.seed * 37.31 + k * 91.7) * 43758.5453;
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
      emitSparks(at);
      // Smoke hung off the sparks themselves. `follow` reads the spark's live
      // position every frame, so each trail inherits that spark's direction,
      // speed, arc and floor bounce — none of which is symmetric.
      const n = Math.min(trail.count, smoke.free, spCount);
      for (let i = 0; i < n; i++) {
        const o = i * 3;
        smoke.spawn({
          pos: v3(spPos[o], spPos[o + 1], spPos[o + 2]),
          follow: () => v3(spPos[o], spPos[o + 1], spPos[o + 2]),
          radius: trail.radius,
          density: trail.density,
          temp: trail.temp,
          life: trail.life,
          attack: 0.01,
        });
      }
      return;
    }
    if (kind === 2) {
      const ax = Math.sin(vent.yaw), az = Math.cos(vent.yaw);
      smoke.spawn({
        pos: v3(0, vent.height, 0), radius: vent.radius,
        vel: v3(ax * vent.speed, 0.3, az * vent.speed), push: vent.push,
        density: vent.density, temp: vent.temp, expand: vent.expand,
        life: vent.seconds, attack: 0.12,
      });
      return;
    }
    // Plume: a small hot source near the floor, which is the case that shows
    // buoyancy, vorticity and dissipation all at once. `stack` presses are
    // co-located and identical, which is what pressing space N times does.
    const n = Math.max(1, Math.min(Math.round(plume.stack), smoke.free));
    for (let i = 0; i < n; i++) {
      smoke.spawn({
        pos: v3(0, 0.18, 0), radius: plume.radius,
        vel: v3(0, plume.speed, 0), push: plume.push,
        density: plume.density, temp: plume.temp, expand: plume.expand,
        life: plume.seconds, attack: 0.12,
      });
    }
  }

  function reset(): void {
    spCount = 0;
    smoke.reset(true);
    fluid.reset();
    bindGroup = makeBindGroup();
  }

  // ---- frame --------------------------------------------------------------
  let paused = false;
  let simTime = 0;
  const empty = new Float32Array(FLUID_MAX_SOURCES * FLUID_SOURCE_STRIDE);

  function frameBody(dt: number): void {
    if (cam.orbit) cam.yaw += dt * cam.speed;
    flashLive.age += dt;
    smoke.update(dt, true);
    Object.assign(fluid.tune, solver);
    // Lateral outflow: the four side walls stop being walls.
    //
    // A closed box conserves everything, so a burst that reaches the sides
    // piles up against them and the pressure solve pushes it back in — energy
    // the room never had, arriving from a boundary that is an arbitrary cut
    // through open air. Bits 0-3 are -x/+x/-z/+z. Floor and ceiling stay solid
    // on purpose: those are real surfaces, the burst sits on one and the
    // column should still spread against the other.
    fluid.openFaces = dbg.openFaces;

    const enc = d.createCommandEncoder({ label: "dyn" });
    fluid.step(enc, dt, smoke.count > 0 ? smoke.packed : empty, smoke.count, () => undefined);

    const { inv, fwd, eye } = viewProjInverse();
    const spVertCount = stepSparks(dt);
    vf.set(inv, 0);
    vf[16] = eye[0]; vf[17] = eye[1]; vf[18] = eye[2]; vf[19] = look.steps;
    vf[20] = ORIGIN[0]; vf[21] = ORIGIN[1]; vf[22] = ORIGIN[2];
    vf[23] = look.shadowSteps;
    vf[24] = ORIGIN[0] + DIMS[0] * CELL;
    vf[25] = ORIGIN[1] + DIMS[1] * CELL;
    vf[26] = ORIGIN[2] + DIMS[2] * CELL;
    vf[27] = look.absorption;
    // Key light, from above and to one side: a plume needs a lit face and a
    // shadowed one or it reads as a flat card whatever the solver is doing.
    const ll = Math.hypot(0.4, 0.85, 0.3);
    vf[28] = 0.4 / ll; vf[29] = 0.85 / ll; vf[30] = 0.3 / ll; vf[31] = look.scatter;
    vf[32] = 1.0; vf[33] = 0.97; vf[34] = 0.92; vf[35] = look.ambient;
    vf[36] = 0.09; vf[37] = 0.11; vf[38] = 0.14; vf[39] = look.exposure;
    vf[40] = 0.02; vf[41] = 0.02; vf[42] = 0.03; vf[43] = look.densityScale;
    // Flash. Squared falloff in time as well as distance: a linear fade reads
    // as a lamp being turned down, where a bang is almost all over in its
    // first third.
    const u = flashLive.age / Math.max(flashLive.duration, 1e-3);
    const env = u >= 1 ? 0 : (1 - u) * (1 - u);
    vf[44] = flashLive.x; vf[45] = flashLive.y; vf[46] = flashLive.z;
    vf[47] = flashLive.power * env;
    vf[48] = flashLive.r; vf[49] = flashLive.g; vf[50] = flashLive.b;
    vf[51] = flash.shadowSteps;
    d.queue.writeBuffer(viewBuf, 0, viewData);

    const pass = enc.beginRenderPass({
      label: "smokeview",
      colorAttachments: [{
        view: ctx.context.getCurrentTexture().createView(),
        loadOp: "clear", storeOp: "store",
        clearValue: { r: 0, g: 0, b: 0, a: 1 },
      }],
    });
    pass.setPipeline(pipeline);
    pass.setBindGroup(0, bindGroup);
    pass.draw(3);

    // Sparks last, into the same pass, so they sit over the volume. They are
    // not depth-tested against it — an ember bright enough to see through
    // smoke is the behaviour the footage shows, not a bug being papered over.
    if (spVertCount > 0) {
      d.queue.writeBuffer(spBuf, 0, spVerts, 0, spVertCount * 6);
      d.queue.writeBuffer(spUniform, 0, fwd as Float32Array<ArrayBuffer>);
      pass.setPipeline(spPipeline);
      pass.setBindGroup(0, spBind);
      pass.setVertexBuffer(0, spBuf);
      pass.draw(spVertCount);
    }
    pass.end();

    d.queue.submit([enc.finish()]);
    simTime += dt;
  }

  // ---- frame-time readout -------------------------------------------------
  //
  // A number, because "it feels faster when I do X" cannot be acted on and
  // this page has several plausible values of X: resolution, march steps,
  // shadow taps, and how much of the box currently has smoke in it. The last
  // one is not obvious and is often the answer — the march breaks early
  // against a dense cloud and runs its full length through a thin one, so the
  // frame gets slower as the smoke *fades*.
  //
  // Median over the window, not mean: one 200 ms hitch from a shader compile
  // or a GC would otherwise dominate the average and hide the steady state.
  const FRAME_WINDOW = 90;
  const frameMs: number[] = [];
  let hudDue = 0;
  const hudEl = document.getElementById("hud");

  function updateHud(now: number): void {
    if (!hudEl || now < hudDue) return;
    hudDue = now + 250;
    if (frameMs.length < 8) return;
    const s = [...frameMs].sort((a, b) => a - b);
    const med = s[s.length >> 1];
    const p90 = s[Math.min(s.length - 1, Math.floor(s.length * 0.9))];
    const mp = (canvas.width * canvas.height) / 1e6;
    hudEl.textContent =
      `${med.toFixed(1)} ms  ${(1000 / Math.max(med, 0.01)).toFixed(0)} fps`
      + `   p90 ${p90.toFixed(1)} ms\n`
      + `${canvas.width}x${canvas.height}  ${mp.toFixed(1)} Mpx`
      + `  x${look.renderScale.toFixed(2)}  ${look.steps}+${look.shadowSteps} steps`;
  }

  let prev = performance.now();
  function frame(now: number): void {
    const raw = now - prev;
    // The sim's dt is capped; the readout's is not. Capping it here would
    // report 50 ms as the ceiling and a page running at 8 fps would read as
    // if it were running at 20.
    const dt = Math.min(raw / 1000, 0.05);
    prev = now;
    if (!paused) {
      frameBody(dt);
      frameMs.push(raw);
      if (frameMs.length > FRAME_WINDOW) frameMs.shift();
    }
    updateHud(now);
    requestAnimationFrame(frame);
  }

  // ---- panel --------------------------------------------------------------
  /**
   * Every live value, in one blob.
   *
   * Tuning ends with somebody moving these numbers into game/smoke.ts, and the
   * panel now has upwards of fifty sliders across seven groups. Reading them
   * off the screen one at a time is how a good set gets landed slightly wrong;
   * this is the same act done by copy.
   */
  function settingsBlob(): string {
    const round = (o: Record<string, number>): Record<string, number> => {
      const out: Record<string, number> = {};
      for (const [k, v] of Object.entries(o)) {
        out[k] = typeof v === "number" ? +v.toFixed(4) : v;
      }
      return out;
    };
    return JSON.stringify({
      kind: ["plume", "burst", "vent", "muzzle"][kind],
      lattice: { dims: DIMS, cell: CELL },
      burst: round(burst as unknown as Record<string, number>),
      trail: round(trail as unknown as Record<string, number>),
      // `age` is live state, not a setting; it would be noise in a paste.
      sparks: round({ ...sparks } as unknown as Record<string, number>),
      flash: round({ power: flash.power, duration: flash.duration,
                     shadowSteps: flash.shadowSteps }),
      plume: round(plume as unknown as Record<string, number>),
      vent: round(vent as unknown as Record<string, number>),
      muzzle: round(muzzle as unknown as Record<string, number>),
      solver: round(solver as unknown as Record<string, number>),
      look: round(look as unknown as Record<string, number>),
      cam: round({ yaw: cam.yaw, pitch: cam.pitch, distance: cam.distance,
                   height: cam.height }),
    }, null, 2);
  }

  function copySettings(): void {
    const text = settingsBlob();
    // Always log as well: clipboard access needs a user gesture and a secure
    // context, and a button that silently fails is worse than no button.
    console.log(text);
    void navigator.clipboard?.writeText(text).then(
      () => console.log("[dynamics] settings copied to clipboard"),
      (e) => console.warn("[dynamics] clipboard refused, use the log above:", e),
    );
  }

  const groups: GroupSpec[] = [
    {
      title: "emitter  (space fires · R resets)",
      items: [
        { kind: "button", label: "copy all settings (C)", onClick: copySettings },
        {
          kind: "select", label: "kind",
          options: ["plume", "burst", "vent", "muzzle"],
          get: () => kind,
          set: (v) => { kind = v; queueMicrotask(() => panel.refresh()); },
        },
      ],
    },
    {
      title: "plume",
      show: () => kind === 0,
      items: [
        sl("radius", 0.05, 1, 0.01, () => plume.radius, (v) => (plume.radius = v)),
        sl("density /s", 0, 600, 5, () => plume.density, (v) => (plume.density = v)),
        sl("temp /s", 0, 40, 0.5, () => plume.temp, (v) => (plume.temp = v)),
        sl("speed m/s", 0, 20, 0.1, () => plume.speed, (v) => (plume.speed = v)),
        sl("push 1/s", 0, 120, 1, () => plume.push, (v) => (plume.push = v)),
        sl("expand 1/s", 0, 300, 5, () => plume.expand, (v) => (plume.expand = v)),
        sl("seconds", 0.1, 60, 0.1, () => plume.seconds, (v) => (plume.seconds = v)),
        // Directly under `density`, because it is the same axis: this is the
        // multiplier that gets past the slider's own ceiling.
        sl("stack (presses)", 1, 8, 1, () => plume.stack, (v) => (plume.stack = v)),
      ],
    },
    {
      title: "muzzle — flash",
      show: () => kind === 3,
      items: [
        sl("power", 0, 200, 1,
          () => muzzle.flashPower, (v) => (muzzle.flashPower = v)),
        sl("duration s", 0.01, 0.5, 0.005,
          () => muzzle.flashDuration, (v) => (muzzle.flashDuration = v)),
        sl("shadow steps", 0, 12, 1,
          () => flash.shadowSteps, (v) => (flash.shadowSteps = v)),
      ],
    },
    {
      title: "muzzle — gas",
      show: () => kind === 3,
      items: [
        sl("height", 0.2, 2.5, 0.05, () => muzzle.height, (v) => (muzzle.height = v)),
        sl("yaw", 0, 6.28, 0.02, () => muzzle.yaw, (v) => (muzzle.yaw = v)),
        sl("standoff", 0, 1, 0.01,
          () => muzzle.standoff, (v) => (muzzle.standoff = v)),
        sl("radius", 0.03, 0.6, 0.01, () => muzzle.radius, (v) => (muzzle.radius = v)),
        sl("density /s", 0, 1200, 10,
          () => muzzle.density, (v) => (muzzle.density = v)),
        sl("temp /s", 0, 80, 1, () => muzzle.temp, (v) => (muzzle.temp = v)),
        sl("speed m/s", 0, 40, 0.5, () => muzzle.speed, (v) => (muzzle.speed = v)),
        sl("push 1/s", 0, 200, 1, () => muzzle.push, (v) => (muzzle.push = v)),
        sl("expand 1/s", 0, 120, 1, () => muzzle.expand, (v) => (muzzle.expand = v)),
        sl("life s", 0.01, 0.4, 0.005, () => muzzle.life, (v) => (muzzle.life = v)),
        sl("ports", 0, 4, 1, () => muzzle.ports, (v) => (muzzle.ports = v)),
        sl("port fraction", 0, 1, 0.05,
          () => muzzle.portFraction, (v) => (muzzle.portFraction = v)),
        sl("port angle", 0, 1.6, 0.05,
          () => muzzle.portAngle, (v) => (muzzle.portAngle = v)),
      ],
    },
    {
      title: "muzzle — wisp",
      show: () => kind === 3,
      items: [
        sl("radius", 0.03, 0.8, 0.01,
          () => muzzle.wispRadius, (v) => (muzzle.wispRadius = v)),
        sl("density /s", 0, 300, 2,
          () => muzzle.wispDensity, (v) => (muzzle.wispDensity = v)),
        sl("temp /s", 0, 40, 0.5,
          () => muzzle.wispTemp, (v) => (muzzle.wispTemp = v)),
        sl("rise m/s", 0, 4, 0.05,
          () => muzzle.wispRise, (v) => (muzzle.wispRise = v)),
        sl("life s", 0.1, 12, 0.1,
          () => muzzle.wispLife, (v) => (muzzle.wispLife = v)),
      ],
    },
    {
      title: "muzzle — powder",
      show: () => kind === 3,
      items: [
        sl("count", 0, 200, 2,
          () => muzzle.sparkCount, (v) => (muzzle.sparkCount = v)),
        sl("speed m/s", 0, 30, 0.5,
          () => muzzle.sparkSpeed, (v) => (muzzle.sparkSpeed = v)),
        sl("cone rad", 0.02, 1.4, 0.02,
          () => muzzle.sparkCone, (v) => (muzzle.sparkCone = v)),
        sl("life s", 0.05, 1.5, 0.01,
          () => muzzle.sparkLife, (v) => (muzzle.sparkLife = v)),
        sl("seed (reroll)", 0, 64, 1, () => muzzle.seed, (v) => (muzzle.seed = v)),
      ],
    },
    {
      title: "burst — flash",
      show: () => kind === 1,
      items: [
        sl("power", 0, 200, 1, () => flash.power, (v) => (flash.power = v)),
        sl("duration s", 0.02, 1, 0.01, () => flash.duration, (v) => (flash.duration = v)),
        sl("shadow steps", 0, 12, 1,
          () => flash.shadowSteps, (v) => (flash.shadowSteps = v)),
      ],
    },
    {
      title: "burst — core",
      show: () => kind === 1,
      items: [
        sl("radius", 0.05, 2, 0.01, () => burst.radius, (v) => (burst.radius = v)),
        sl("density /s", 0, 1200, 10, () => burst.density, (v) => (burst.density = v)),
        sl("temp /s", 0, 80, 1, () => burst.temp, (v) => (burst.temp = v)),
        sl("expand 1/s", 0, 300, 5, () => burst.expand, (v) => (burst.expand = v)),
        sl("life s", 0.02, 2, 0.01, () => burst.life, (v) => (burst.life = v)),
        sl("height", 0.02, 2.5, 0.02, () => burst.height, (v) => (burst.height = v)),
        sl("seed (reroll)", 0, 64, 1, () => burst.seed, (v) => (burst.seed = v)),
        sl("can half-length", 0.01, 0.4, 0.01,
          () => burst.halfLength, (v) => (burst.halfLength = v)),
        sl("vent speed m/s", 0, 25, 0.5,
          () => burst.ventSpeed, (v) => (burst.ventSpeed = v)),
        sl("end asymmetry", 0, 1, 0.02,
          () => burst.asymmetry, (v) => (burst.asymmetry = v)),
        sl("body vents", 0, 10, 1, () => burst.bodyVents, (v) => (burst.bodyVents = v)),
        sl("body strength", 0, 1.5, 0.05,
          () => burst.bodyFraction, (v) => (burst.bodyFraction = v)),
        sl("axis tilt", 0, 1.4, 0.02, () => burst.tilt, (v) => (burst.tilt = v)),
        sl("port jitter", 0, 1.5, 0.05, () => burst.jitter, (v) => (burst.jitter = v)),
      ],
    },
    {
      title: "burst — spark trails",
      show: () => kind === 1,
      items: [
        sl("smoking sparks", 0, 90, 1, () => trail.count, (v) => (trail.count = v)),
        sl("radius", 0.02, 0.4, 0.005, () => trail.radius, (v) => (trail.radius = v)),
        sl("density /s", 0, 400, 5, () => trail.density, (v) => (trail.density = v)),
        sl("temp /s", 0, 40, 0.5, () => trail.temp, (v) => (trail.temp = v)),
        sl("life s", 0.05, 3, 0.05, () => trail.life, (v) => (trail.life = v)),
      ],
    },
    {
      title: "burst — sparks",
      show: () => kind === 1,
      items: [
        sl("count", 0, 512, 8, () => sparks.count, (v) => (sparks.count = v)),
        sl("speed m/s", 1, 40, 0.5, () => sparks.speed, (v) => (sparks.speed = v)),
        sl("spread", 0, 1, 0.02, () => sparks.spread, (v) => (sparks.spread = v)),
        sl("gravity", 0, 40, 0.5, () => sparks.gravity, (v) => (sparks.gravity = v)),
        sl("drag 1/s", 0, 8, 0.05, () => sparks.drag, (v) => (sparks.drag = v)),
        sl("life s", 0.1, 4, 0.05, () => sparks.life, (v) => (sparks.life = v)),
        sl("brightness", 0, 8, 0.1,
          () => sparks.brightness, (v) => (sparks.brightness = v)),
        sl("hold", 0, 0.8, 0.02, () => sparks.hold, (v) => (sparks.hold = v)),
        sl("lift", 0, 1, 0.02, () => sparks.lift, (v) => (sparks.lift = v)),
      ],
    },
    {
      title: "burst — wisp",
      show: () => kind === 1,
      items: [
        sl("radius", 0.05, 1.5, 0.05,
          () => burst.wispRadius, (v) => (burst.wispRadius = v)),
        sl("density /s", 0, 200, 2,
          () => burst.wispDensity, (v) => (burst.wispDensity = v)),
        sl("temp /s", 0, 40, 0.5, () => burst.wispTemp, (v) => (burst.wispTemp = v)),
        sl("rise m/s", 0, 6, 0.1, () => burst.wispRise, (v) => (burst.wispRise = v)),
        sl("life s", 0, 20, 0.5, () => burst.wispLife, (v) => (burst.wispLife = v)),
      ],
    },
    {
      title: "vent",
      show: () => kind === 2,
      items: [
        sl("radius", 0.05, 1.5, 0.01, () => vent.radius, (v) => (vent.radius = v)),
        sl("density /s", 0, 600, 5, () => vent.density, (v) => (vent.density = v)),
        sl("temp /s", 0, 20, 0.2, () => vent.temp, (v) => (vent.temp = v)),
        sl("speed m/s", 0, 25, 0.5, () => vent.speed, (v) => (vent.speed = v)),
        sl("push 1/s", 0, 120, 1, () => vent.push, (v) => (vent.push = v)),
        sl("yaw", -3.15, 3.15, 0.05, () => vent.yaw, (v) => (vent.yaw = v)),
        sl("seconds", 0.1, 60, 0.1, () => vent.seconds, (v) => (vent.seconds = v)),
      ],
    },
    {
      title: "solver",
      items: [
        sl("vorticity", 0, 30, 0.25, () => solver.vorticity, (v) => (solver.vorticity = v)),
        sl("buoyancy", 0, 30, 0.25, () => solver.buoyancy, (v) => (solver.buoyancy = v)),
        sl("weight", 0, 3, 0.01, () => solver.weight, (v) => (solver.weight = v)),
        sl("dissipation", 0, 2, 0.005,
          () => solver.dissipation, (v) => (solver.dissipation = v)),
        sl("cooling", 0, 12, 0.1, () => solver.cooling, (v) => (solver.cooling = v)),
        sl("jacobi", 4, 80, 2, () => solver.jacobi, (v) => (solver.jacobi = v)),
      ],
    },
    {
      title: "view",
      items: [
        // First, because it is the biggest lever on frame time by a wide
        // margin: cost is linear in pixels and 0.5 is a quarter of them.
        sl("render scale", 0.25, 1, 0.05,
          () => look.renderScale, (v) => { look.renderScale = v; resize(); }),
        sl("march steps", 16, 256, 8, () => look.steps, (v) => (look.steps = v)),
        sl("shadow steps", 0, 16, 1, () => look.shadowSteps, (v) => (look.shadowSteps = v)),
        sl("absorption", 0.002, 0.4, 0.002, () => look.absorption, (v) => (look.absorption = v)),
        sl("density scale", 0.1, 6, 0.05,
          () => look.densityScale, (v) => (look.densityScale = v)),
        sl("scatter", 0, 3, 0.05, () => look.scatter, (v) => (look.scatter = v)),
        sl("ambient", 0, 0.5, 0.005, () => look.ambient, (v) => (look.ambient = v)),
        sl("exposure", 0.1, 4, 0.05, () => look.exposure, (v) => (look.exposure = v)),
        tg("orbit", () => cam.orbit, (v) => (cam.orbit = v)),
        sl("orbit speed", 0, 1.5, 0.05, () => cam.speed, (v) => (cam.speed = v)),
        sl("distance", 1, 10, 0.1, () => cam.distance, (v) => (cam.distance = v)),
        sl("pitch", -0.4, 1.4, 0.02, () => cam.pitch, (v) => (cam.pitch = v)),
      ],
    },
  ];
  const panel = new TweakPanel(groups);
  // Apply the show predicates once, so the page opens with only the selected
  // emitter's controls rather than all three.
  panel.refresh();

  const help = document.getElementById("help");
  if (help) {
    help.textContent = "space fire · R reset · O orbit · ` panel\n"
      + `${DIMS[0]}x${DIMS[1]}x${DIMS[2]} @ ${CELL * 100} cm · C copies settings`;
  }

  addEventListener("keydown", (e) => {
    if (e.code === "Backquote") { panel.toggleVisible(); e.preventDefault(); return; }
    if (e.code === "Space") { fire(); e.preventDefault(); }
    if (e.code === "KeyR") reset();
    if (e.code === "KeyO") cam.orbit = !cam.orbit;
    if (e.code === "KeyC") copySettings();
  });

  /** Grading surface; see tools/headless/scenarios/smoke-strip.js. */
  Object.assign(window, {
    __dyn: {
      fluid, smoke,
      params: {
        plume, burst, trail, sparks, vent, muzzle, flash, solver, look, cam, dbg,
      },
      settings: settingsBlob,
      kind: () => kind,
      setKind: (k: number) => { kind = k; },
      fire, reset,
      pause: (on: boolean) => { paused = on; },
      /** Steps exactly `n` frames of `dtMs` each, however long each takes. */
      step: async (n: number, dtMs = 50): Promise<string> => {
        for (let i = 0; i < n; i++) frameBody(dtMs / 1000);
        await d.queue.onSubmittedWorkDone();
        return `stepped ${n}`;
      },
      resize: (w: number, h: number) => {
        canvas.style.width = `${w}px`;
        canvas.style.height = `${h}px`;
        resize();
      },
      simTime: () => simTime,
      /** Spark state, for checking the ballistic arc is doing what it claims. */
      sparkStats: () => {
        let alive = 0, maxY = 0, meanVy = 0, maxR = 0;
        for (let i = 0; i < spCount; i++) {
          if (spAge[i] >= spLife[i]) { continue; }
          const o = i * 3;
          alive++;
          maxY = Math.max(maxY, spPos[o + 1]);
          meanVy += spVel[o + 1];
          maxR = Math.max(maxR, Math.hypot(spPos[o], spPos[o + 2]));
        }
        return { alive, maxY, meanVy: alive ? meanVy / alive : 0, maxR };
      },
    },
    __renderer: { renderWidth: () => canvas.width },
    __renderStill: async (n = 30, dtMs = 50): Promise<string> => {
      for (let i = 0; i < n; i++) frameBody(dtMs / 1000);
      await d.queue.onSubmittedWorkDone();
      return `rendered ${n}`;
    },
  });

  requestAnimationFrame(frame);
}

void main();
