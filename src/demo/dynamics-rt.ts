import { v3 } from "../core/math";
import { GroupSpec } from "../ui/panel";
import { buildDynamicsBox } from "../scene/demo";
import {
  EmitterKind, SparkField, burst, fire as fireEffect,
  flash, muzzle, plume, sparks, trail, vent,
} from "../game/effects";
import { DemoDeps, bootDemo, sl, tg } from "./common";

// ---------------------------------------------------------------------------
// /demo/dynamics-rt — the same event as /demo/dynamics, through the tracer.
//
// The sibling page draws the density field with one raymarch: milliseconds a
// frame, no denoiser, and therefore the right instrument for judging MOTION.
// It cannot answer whether the numbers are GRADED right, because the two
// renderers disagree about what a density means by a factor of about sixteen —
// the raymarch uses sigma_t = absorption * rho at 0.035, the tracer uses
// settings.volumetric * rho at 0.55 — and on top of that the tracer's output
// goes through ReSTIR, a three-signal SVGF and AgX, none of which the raymarch
// has. A density tuned through one and read through the other is meaningless.
//
// So: same lattice, same emitters, same numbers, different renderer. Both
// pages import game/effects.ts, so there is one copy of the tuning and no way
// for the two to drift apart inside a session. Open them side by side.
//
// The fine lattice is deliberately never activated. `smoke` below repoints the
// PRIMARY sim at 128 x 96 x 128 @ 3.125 cm, which is finer than the fine
// lattice's 6.25 cm, so restricting one into the other would be upsampling
// backwards. peerMode stays 0 unless activateFine() is called, and nothing
// here calls it — one lattice, exactly the sibling page's.
// ---------------------------------------------------------------------------

/** Matches /demo/dynamics: 4 x 3 x 4 m centred on the origin at 3.125 cm. */
const DIMS: [number, number, number] = [128, 96, 128];
const CELL = 0.03125;

/** Half the room's interior, for clamping the orbit off the walls. */
const ROOM_HALF = 4.5;

let kind: EmitterKind = 1;
const orbit = { on: true, speed: 0.22, distance: 4.2, pitch: 0.24 };

/**
 * Solver tuning, mirrored so it is gradeable here too.
 *
 * Deliberately NOT imported from the sibling page: `solver` there is a page
 * object, and the two pages having independent solver knobs is what lets one
 * ask "is this jacobi count enough" without disturbing the other. The emitter
 * numbers are shared; how hard the solver is run is a per-page question.
 */
const solver = {
  vorticity: 6.0,
  buoyancy: 6.0,
  weight: 0.3,
  dissipation: 0.22,
  cooling: 3.0,
  jacobi: 20,
};

/**
 * How the tracer draws the medium — the half of the grade the sibling page
 * cannot show. `volumetric` is the extinction per unit density, and it is the
 * single number that decides whether a density tuned at absorption 0.035 reads
 * as a wisp or as a wall here.
 */
const look = {
  volumetric: 0.55,
  detail: 1.35,
  detailFreq: 9.0,
  steps: 24,
  exposure: 0.6,
  sky: 0.04,
};

/**
 * Sparks are simulated but not drawn.
 *
 * The sibling page draws them as additive line segments, which is a forward
 * pass with no equivalent here — an ember is emissive geometry a millimetre
 * across, and the tracer's BVH is built from boxes.
 *
 * Stepping them anyway is not optional. The burst hangs up to 88 smoke sources
 * off the sparks with `follow`, which reads each spark's live position every
 * frame; a field that never advances leaves all of them stacked at the origin,
 * and the burst loses the entire trail structure that its tuning is built on.
 */
const sparkField = new SparkField();

let lastFire = 0;

function fire(d: DemoDeps): void {
  const lit = fireEffect(kind, d.smoke, sparkField);
  if (!lit) return;
  // The flash goes through the game's transient-light path, which is the
  // point: on the sibling page it is a point light inside a raymarch with a
  // hand-rolled shadow march, and here it is a real sphere light that casts
  // real shadows, bounces off the walls and lands in the transient denoiser
  // signal. Whether the power is right is a question only this page can ask.
  d.flashes.spawn(v3(lit.x, lit.y, lit.z), {
    duration: lit.duration,
    // `power` is the raymarch's units — radiance at one metre in a shader that
    // divides by d^2 and nothing else. The tracer wants candela-ish figures
    // against its own exposure, and the ratio between the two is exactly the
    // kind of thing that has to be found by looking rather than derived.
    intensity: lit.power * flashGain.value,
    color: v3(lit.r, lit.g, lit.b),
    radius: 0.12,
  });
}

/** The one number that cannot be shared with the raymarch page. See fire(). */
const flashGain = { value: 900 };

function resetWorld(d: DemoDeps): void {
  sparkField.clear();
  d.smoke.reset(true);
  d.renderer.fluid.reset();
}

function applyLook(d: DemoDeps): void {
  d.settings.volumetric = look.volumetric;
  d.settings.smokeDetail = look.detail;
  d.settings.smokeDetailFreq = look.detailFreq;
  d.settings.volumetricSteps = look.steps;
  d.settings.exposure = look.exposure;
  d.settings.skyIntensity = look.sky;
  Object.assign(d.renderer.fluid.tune, solver);
}

function fitDistance(distance: number, pitch: number): number {
  return Math.min(distance, (ROOM_HALF - 0.4) / Math.max(Math.cos(pitch), 0.2));
}

function groups(d: DemoDeps): GroupSpec[] {
  const relight = () => applyLook(d);
  return [
    {
      title: "emitter  (space fires · R resets)",
      items: [
        {
          kind: "select", label: "kind",
          options: ["plume", "burst", "vent", "muzzle"],
          get: () => kind,
          set: (v) => { kind = v as EmitterKind; },
        },
        tg("auto orbit", () => orbit.on, (v) => (orbit.on = v)),
        sl("orbit speed", 0, 1.2, 0.02, () => orbit.speed, (v) => (orbit.speed = v)),
        sl("distance", 1.2, 8, 0.1, () => orbit.distance, (v) => (orbit.distance = v)),
        sl("pitch", 0.05, 1.4, 0.02, () => orbit.pitch, (v) => (orbit.pitch = v)),
      ],
    },
    {
      // The grade. Everything here is the tracer's half of the transfer
      // function and has no counterpart on the raymarch page.
      title: "medium  (the re-grade)",
      items: [
        sl("volumetric (sigma/rho)", 0.02, 1.5, 0.01,
          () => look.volumetric, (v) => (look.volumetric = v), relight),
        sl("smoke detail", 0, 4, 0.05, () => look.detail, (v) => (look.detail = v), relight),
        sl("detail freq", 1, 24, 0.5,
          () => look.detailFreq, (v) => (look.detailFreq = v), relight),
        sl("march steps", 4, 64, 2, () => look.steps, (v) => (look.steps = v), relight),
        sl("exposure", 0.05, 1.2, 0.01,
          () => look.exposure, (v) => (look.exposure = v), relight),
        sl("flash gain", 0, 4000, 25,
          () => flashGain.value, (v) => (flashGain.value = v)),
      ],
    },
    {
      title: "solver",
      items: [
        sl("vorticity", 0, 20, 0.25,
          () => solver.vorticity, (v) => (solver.vorticity = v), relight),
        sl("buoyancy", 0, 20, 0.25,
          () => solver.buoyancy, (v) => (solver.buoyancy = v), relight),
        sl("weight", 0, 2, 0.02, () => solver.weight, (v) => (solver.weight = v), relight),
        sl("dissipation", 0, 2, 0.01,
          () => solver.dissipation, (v) => (solver.dissipation = v), relight),
        sl("cooling", 0, 10, 0.1, () => solver.cooling, (v) => (solver.cooling = v), relight),
        sl("jacobi", 4, 80, 2, () => solver.jacobi, (v) => (solver.jacobi = v), relight),
      ],
    },
  ];
}

void bootDemo({
  build: buildDynamicsBox,
  groups,

  /**
   * The sibling page's lattice, exactly.
   *
   * This replaces the PRIMARY sim, not just the rendered texture — see
   * Renderer.create, where smokeDims/smokeCellSize/smokeOrigin are what
   * `this.fluid` is constructed from. 1.57 M cells against the level
   * lattice's 389 k, over a domain 156x smaller.
   */
  smoke: {
    dims: DIMS,
    origin: [-(DIMS[0] * CELL) / 2, 0, -(DIMS[2] * CELL) / 2],
    cell: CELL,
  },

  help: "space fires · R clears · O orbit · ` panel\n"
    + "1 plume · 2 burst · 3 vent · 4 muzzle — same numbers as /demo/dynamics",

  init(d) {
    // Full internal resolution: nothing here is framerate-critical and a 2x
    // upscale is fatal while judging the edge of a cloud.
    d.settings.resolutionScale = 1.0;
    d.settings.fogAmount = 0.0;
    applyLook(d);

    d.camera.distance = fitDistance(orbit.distance, orbit.pitch);
    d.camera.pitch = orbit.pitch;
    d.torch.on = false;
    // The orbit is centred on `focus`, and `focus` starts at the level's
    // spawn point, not the origin. Left alone that puts the eye a room and a
    // half away from the box — far outside the wall, which renders as a flat
    // sheet of wall albedo and reads as a broken renderer rather than as a
    // camera standing in the wrong place.
    d.focus.x = 0;
    d.focus.z = 0;

    Object.assign(window, {
      __dynrt: {
        renderer: d.renderer,
        fluid: d.renderer.fluid,
        smoke: d.smoke,
        params: { plume, burst, trail, sparks, vent, muzzle, flash, solver, look },
        kind: () => kind,
        setKind: (k: number) => { kind = k as EmitterKind; },
        fire: () => fire(d),
        reset: () => resetWorld(d),
        apply: () => applyLook(d),
        park: (yaw: number, pitch = orbit.pitch, distance = orbit.distance) => {
          orbit.on = false;
          d.camera.yaw = yaw;
          d.camera.pitch = pitch;
          d.camera.distance = fitDistance(distance, pitch);
          d.focus.x = 0;
          d.focus.z = 0;
        },
        sparkStats: () => sparkField.stats(),
      },
    });
  },

  step(d, dt, elapsed) {
    // Advance the sparks even though nothing draws them: the burst's trail
    // sources follow their positions. See the sparkField comment.
    sparkField.step(dt);
    lastFire += dt;
    if (orbit.on) {
      d.camera.yaw = elapsed * orbit.speed;
      d.camera.pitch = orbit.pitch;
      d.camera.distance = fitDistance(orbit.distance, orbit.pitch);
    }
    // Re-pinned every frame, not just at init: WASD walks `focus` around the
    // floor, and one stray keypress would otherwise leave the orbit centred
    // somewhere that is not the thing being looked at.
    d.focus.x = 0;
    d.focus.z = 0;
  },

  key(d, code) {
    switch (code) {
      case "Space": fire(d); break;
      case "KeyR": resetWorld(d); break;
      case "KeyO": orbit.on = !orbit.on; break;
      case "Digit1": kind = 0; break;
      case "Digit2": kind = 1; break;
      case "Digit3": kind = 2; break;
      case "Digit4": kind = 3; break;
    }
  },
});
