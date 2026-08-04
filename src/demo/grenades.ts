import { v3 } from "../core/math";
import { GroupSpec } from "../ui/panel";
import { buildGrenadeBox } from "../scene/demo";
import { FLASHBANG } from "../game/flashes";
import { DemoDeps, bootDemo, sl, tg } from "./common";

// ---------------------------------------------------------------------------
// /demo/grenades — a flashbang and a smoke grenade, at the middle of a small
// room, on a camera that goes all the way round them.
//
// Written to be GRADED, not just driven. Every earlier attempt at tuning this
// failed the same way: a browser tab throttles animation frames when it is not
// focused, so an agent driving it sees a handful of frames per screenshot and
// never watches an effect play. That is tuning by argument instead of by
// looking, and it does not converge.
//
// So the page exposes `__grenade`, and everything that matters is reachable
// from it: reset to a known state, fire either device, park the camera, step
// exact simulation time. tools/headless/scenarios/grenade-strip.js drives that
// to render one detonation at fixed beats and hand the frames back as PNGs —
// one headless run, a filmstrip, no throttling and nothing to guess at.
//
// Consequences for how this file is written:
//
//   - No wall clock anywhere. Beats are simulation seconds.
//   - Nothing reads the cursor. Both devices fire at the middle of the room,
//     so a scenario and a person see the same event.
//   - Firing is reproducible from a reset: same reset, same inputs, same
//     frames.
// ---------------------------------------------------------------------------

/** Everything is thrown here. Named because three separate things need it. */
const CENTRE = v3(0, 0, 0);

/**
 * Interior half-extent of buildGrenadeBox; the orbit clamps against it.
 *
 * A circle puts the eye at its radius on every axis in turn, so a distance
 * chosen to clear the corners walks through the middle of a wall a quarter
 * turn later. That renders black, and a black frame reads as a broken renderer
 * rather than as a camera standing outside the building.
 */
const ROOM_HALF = 5;

// --- the two devices, as live numbers ---------------------------------------
//
// Mirrors of the shipped presets rather than calls into them, so a slider can
// move without an edit-rebuild cycle. `P` prints them back in source shape.

/**
 * Smoke.flashbang().
 *
 * Two sources, because one cannot be both the detonation and what is left
 * afterwards: a dense core that expands and is gone in a quarter second, and a
 * thin warm column that rises for seconds after it.
 */
const bang = {
  intensity: FLASHBANG.intensity,
  duration: FLASHBANG.duration,
  lightRadius: FLASHBANG.radius,
  coreRadius: 0.6,
  coreDensity: 450,
  coreTemp: 30,
  coreExpand: 70,
  coreLife: 0.25,
  wispRadius: 0.4,
  wispDensity: 26,
  wispTemp: 7,
  wispRise: 1.2,
  wispLife: 6,
};

/**
 * Smoke.canisterCloud().
 *
 * temp 0 is load-bearing. The solver's force term is
 * `v.y += (buoyancy*temp - weight*dens)*dt`, and buoyancy beats weight per
 * unit by ~30x, so any warmth at all lifts a jet that is meant to hug the
 * floor. No expand either: a vent is momentum, and expansion thins a cloud,
 * which is the opposite of what a concealment device wants.
 */
const can = {
  radius: 0.55,
  density: 200,
  temp: 0,
  push: 45,
  speed: 9,
  rise: 0.4,
  seconds: 30,
  yaw: 0,
  height: 0.3,
};

/** How the medium is drawn — the biggest lever on whether it reads as smoke. */
const look = {
  detail: 1.35,
  detailFreq: 9.0,
  steps: 24,
  exposure: 0.35,
};

/** Solver tuning, mirrored so it is gradeable alongside everything else. */
const solver = {
  vorticity: 6.0,
  buoyancy: 6.0,
  weight: 0.3,
  dissipation: 0.22,
  cooling: 3.0,
};

const orbit = { on: true, speed: 0.22, distance: 4.6, pitch: 0.34 };
const replay = { on: true, every: 9 };

let nextFire = 0.6;
let whichNext = 0;

// --- firing -----------------------------------------------------------------

function fireBang(d: DemoDeps): void {
  const at = v3(CENTRE.x, 1.0, CENTRE.z);
  if (bang.intensity > 0) {
    d.flashes.spawn(at, {
      ...FLASHBANG,
      intensity: bang.intensity,
      duration: bang.duration,
      radius: bang.lightRadius,
    });
  }
  d.smoke.spawn({
    pos: at, radius: bang.coreRadius, density: bang.coreDensity,
    temp: bang.coreTemp, expand: bang.coreExpand,
    life: bang.coreLife, attack: 0.02,
  });
  d.smoke.spawn({
    pos: at, radius: bang.wispRadius, vel: v3(0, bang.wispRise, 0), push: 5,
    density: bang.wispDensity, temp: bang.wispTemp,
    life: bang.wispLife, attack: 0.3,
  });
}

function fireCan(d: DemoDeps): void {
  const ax = Math.sin(can.yaw), az = Math.cos(can.yaw);
  d.smoke.spawn({
    pos: v3(CENTRE.x, can.height, CENTRE.z),
    radius: can.radius,
    vel: v3(ax * can.speed, can.rise, az * can.speed),
    push: can.push, density: can.density, temp: can.temp,
    life: can.seconds, attack: 0.15,
  });
}

/** Back to the state the page booted in: empty room, lattice anchored. */
function resetWorld(d: DemoDeps): void {
  d.smoke.reset(true);
  d.renderer.fluid.reset();
  d.renderer.activateFine(CENTRE.x, CENTRE.z);
  nextFire = replay.every;
}

function applyLook(d: DemoDeps): void {
  d.settings.smokeDetail = look.detail;
  d.settings.smokeDetailFreq = look.detailFreq;
  d.settings.volumetricSteps = look.steps;
  d.settings.exposure = look.exposure;
  Object.assign(d.renderer.fluid.tune, solver);
  Object.assign(d.renderer.fluidFine.tune, solver);
}

/** Clamped orbit distance: the horizontal reach is what has to fit the room. */
function fitDistance(distance: number, pitch: number): number {
  return Math.min(distance, (ROOM_HALF - 0.7) / Math.max(Math.cos(pitch), 0.2));
}

// --- panel ------------------------------------------------------------------

function fmt(n: number): string {
  return Number.isInteger(n) ? String(n) : String(+n.toFixed(3));
}

/** Prints the live numbers in the shape the source files want them. */
function dump(): void {
  console.log(
    "// game/smoke.ts  flashbang()\n"
    + `this.spawn({ pos: at, radius: ${fmt(bang.coreRadius)}, density: ${fmt(bang.coreDensity)},`
    + ` temp: ${fmt(bang.coreTemp)}, expand: ${fmt(bang.coreExpand)},`
    + ` push: 0, life: ${fmt(bang.coreLife)}, attack: 0.02 });\n`
    + `this.spawn({ pos: at, radius: ${fmt(bang.wispRadius)},`
    + ` vel: { x: 0, y: ${fmt(bang.wispRise)}, z: 0 }, push: 5,`
    + ` density: ${fmt(bang.wispDensity)}, temp: ${fmt(bang.wispTemp)},`
    + ` life: ${fmt(bang.wispLife)}, attack: 0.3 });\n\n`
    + "// game/flashes.ts  FLASHBANG\n"
    + `{ duration: ${fmt(bang.duration)}, intensity: ${fmt(bang.intensity)},`
    + ` radius: ${fmt(bang.lightRadius)} }\n\n`
    + "// game/smoke.ts  canisterCloud()\n"
    + `radius: ${fmt(can.radius)}, density: ${fmt(can.density)}, temp: ${fmt(can.temp)},`
    + ` push: ${fmt(can.push)}, speed: ${fmt(can.speed)}, rise: ${fmt(can.rise)}\n\n`
    + "// look / solver\n"
    + `detail ${fmt(look.detail)} freq ${fmt(look.detailFreq)} steps ${fmt(look.steps)}`
    + ` | vorticity ${fmt(solver.vorticity)} buoyancy ${fmt(solver.buoyancy)}`
    + ` weight ${fmt(solver.weight)} dissipation ${fmt(solver.dissipation)}`,
  );
}

function groups(d: DemoDeps): GroupSpec[] {
  const relight = () => applyLook(d);
  return [
    {
      title: "showcase",
      items: [
        tg("auto orbit", () => orbit.on, (v) => (orbit.on = v)),
        sl("orbit speed", 0, 1.2, 0.02, () => orbit.speed, (v) => (orbit.speed = v)),
        sl("distance", 2, 8, 0.1, () => orbit.distance, (v) => (orbit.distance = v)),
        sl("pitch", 0.05, 1.4, 0.02, () => orbit.pitch, (v) => (orbit.pitch = v)),
        tg("auto replay", () => replay.on, (v) => (replay.on = v)),
        sl("replay every s", 3, 30, 1, () => replay.every, (v) => (replay.every = v)),
      ],
    },
    {
      title: "look",
      items: [
        sl("smoke detail", 0, 4, 0.05, () => look.detail, (v) => (look.detail = v), relight),
        sl("detail freq", 1, 24, 0.5,
          () => look.detailFreq, (v) => (look.detailFreq = v), relight),
        sl("march steps", 4, 64, 2, () => look.steps, (v) => (look.steps = v), relight),
        sl("exposure", 0.05, 1.2, 0.01,
          () => look.exposure, (v) => (look.exposure = v), relight),
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
      ],
    },
    {
      title: "flashbang — light  (1)",
      items: [
        sl("intensity", 0, 400000, 5000, () => bang.intensity, (v) => (bang.intensity = v)),
        sl("duration s", 0.02, 0.6, 0.01, () => bang.duration, (v) => (bang.duration = v)),
        sl("light radius", 0.05, 1.5, 0.05,
          () => bang.lightRadius, (v) => (bang.lightRadius = v)),
      ],
    },
    {
      title: "flashbang — core",
      items: [
        sl("density /s", 0, 1200, 10, () => bang.coreDensity, (v) => (bang.coreDensity = v)),
        sl("radius", 0.1, 2, 0.05, () => bang.coreRadius, (v) => (bang.coreRadius = v)),
        sl("temp /s", 0, 80, 1, () => bang.coreTemp, (v) => (bang.coreTemp = v)),
        // div(v) ~ 3v/r, so 0.6 m pushing out at 10 m/s is about 50.
        sl("expand 1/s", 0, 300, 5, () => bang.coreExpand, (v) => (bang.coreExpand = v)),
        sl("life s", 0.05, 2, 0.05, () => bang.coreLife, (v) => (bang.coreLife = v)),
      ],
    },
    {
      title: "flashbang — wisp",
      items: [
        sl("density /s", 0, 200, 2, () => bang.wispDensity, (v) => (bang.wispDensity = v)),
        sl("radius", 0.1, 1.5, 0.05, () => bang.wispRadius, (v) => (bang.wispRadius = v)),
        sl("temp /s", 0, 40, 0.5, () => bang.wispTemp, (v) => (bang.wispTemp = v)),
        sl("rise m/s", 0, 6, 0.1, () => bang.wispRise, (v) => (bang.wispRise = v)),
        sl("life s", 0, 20, 0.5, () => bang.wispLife, (v) => (bang.wispLife = v)),
      ],
    },
    {
      title: "smoke grenade  (2)",
      items: [
        sl("seconds", 1, 60, 1, () => can.seconds, (v) => (can.seconds = v)),
        sl("density /s", 0, 500, 5, () => can.density, (v) => (can.density = v)),
        sl("radius", 0.1, 2, 0.05, () => can.radius, (v) => (can.radius = v)),
        sl("temp /s", 0, 20, 0.2, () => can.temp, (v) => (can.temp = v)),
        sl("jet speed m/s", 0, 25, 0.5, () => can.speed, (v) => (can.speed = v)),
        sl("push 1/s", 0, 120, 1, () => can.push, (v) => (can.push = v)),
        sl("upward bias m/s", 0, 3, 0.1, () => can.rise, (v) => (can.rise = v)),
        sl("vent yaw", -3.15, 3.15, 0.05, () => can.yaw, (v) => (can.yaw = v)),
      ],
    },
  ];
}

// --- boot -------------------------------------------------------------------

void bootDemo({
  build: buildGrenadeBox,
  groups,

  /**
   * A lattice for this room instead of for the level.
   *
   * This is the resolution that matters, and it is not the one anybody reaches
   * for first. The tracer samples ONE 3D texture and takes its dimensions from
   * that texture, so the *drawn* smoke is only ever as fine as this — however
   * detailed the simulation runs, it is restricted back down into this before
   * a ray is cast. At the level's 25 cm a 0.6 m burst is about two voxels
   * across, which is a blob by construction.
   *
   * 8 x 3.5 x 8 m at 6.25 cm: 128 x 56 x 128 = 917k cells against the level
   * lattice's 389k, over a domain 29x smaller. Four times the linear detail in
   * the image. The room is 5 x 5, so the box clears it on every side.
   */
  smoke: {
    dims: [128, 56, 128],
    origin: [-4, 0, -4],
    cell: 0.0625,
  },

  help: "orbits on its own · O toggle · right-drag look · wheel zoom · ` panel\n"
    + "1 flashbang · 2 smoke grenade · R clear · P print values",

  init(d) {
    // Full internal resolution. The game runs 0.5 for vblank headroom, which
    // on a 1920x1080 canvas is a hard 2x upscale — fine while moving, fatal
    // while judging the edge of a cloud. Nothing here is framerate-critical.
    d.settings.resolutionScale = 1.0;
    d.settings.fogAmount = 0.0;
    d.settings.skyIntensity = 0.04;
    applyLook(d);

    d.camera.distance = fitDistance(orbit.distance, orbit.pitch);
    d.camera.pitch = orbit.pitch;
    d.focus.x = CENTRE.x;
    d.focus.z = CENTRE.z;

    // Anchored once and never moved. /demo/smoke re-anchors per event because
    // a plume there can cross a hall; here nothing leaves the room, and
    // re-anchoring calls reset(), which would wipe a cloud mid-effect.
    d.renderer.activateFine(CENTRE.x, CENTRE.z);
    d.resize();

    /**
     * The grading surface.
     *
     * A headless scenario drives exactly this, so what gets measured is what a
     * person sees on the same page rather than a second code path that could
     * drift away from it.
     */
    Object.assign(window, {
      __grenade: {
        reset: () => resetWorld(d),
        bang: () => fireBang(d),
        can: () => fireCan(d),
        /** Stops the orbit and places the camera, for a repeatable frame. */
        park: (yaw: number, pitch = orbit.pitch, distance = orbit.distance) => {
          orbit.on = false;
          replay.on = false;
          d.camera.yaw = yaw;
          d.camera.pitch = pitch;
          d.camera.distance = fitDistance(distance, pitch);
          d.focus.x = CENTRE.x;
          d.focus.z = CENTRE.z;
        },
        /** Live parameter blocks, so a scenario can sweep them. */
        params: { bang, can, look, solver, orbit, replay },
        apply: () => applyLook(d),
      },
    });
  },

  step(d, dt, elapsed) {
    if (orbit.on) {
      d.camera.yaw = elapsed * orbit.speed;
      d.camera.pitch = orbit.pitch;
      d.camera.distance = fitDistance(orbit.distance, orbit.pitch);
      d.focus.x = CENTRE.x;
      d.focus.z = CENTRE.z;
    }
    if (!replay.on) return;
    nextFire -= dt;
    if (nextFire > 0) return;
    resetWorld(d);
    if (whichNext === 0) fireBang(d); else fireCan(d);
    whichNext ^= 1;
  },

  key(d, code) {
    switch (code) {
      case "Digit1": resetWorld(d); fireBang(d); break;
      case "Digit2": resetWorld(d); fireCan(d); break;
      case "KeyR": resetWorld(d); break;
      case "KeyP": dump(); break;
      case "KeyO": orbit.on = !orbit.on; break;
    }
  },
});
