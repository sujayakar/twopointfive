import { v3 } from "../core/math";
import { GroupSpec } from "../ui/panel";
import { buildGrenadeBox } from "../scene/demo";
import { FLASHBANG } from "../game/flashes";
import { DemoDeps, bootDemo, sl, tg } from "./common";

// ---------------------------------------------------------------------------
// /demo/grenades — the two thrown devices, on a crate, in a room you can get
// all the way around.
//
// Split out from /demo/smoke rather than added to it. That page is a solver
// lab: a 24 m hall with columns to split a plume, a barrier to pool against
// and an alcove to fill, because those are what tell you the *fluid* is right.
// A grenade is a two-second event judged from the outside, so every one of
// those obstacles is something to orbit past instead of evidence.
//
// The values below are the shipped presets, mirrored as live numbers rather
// than called through Smoke.flashbang()/canisterCloud(). When a set is right
// it moves back into game/smoke.ts and game/flashes.ts — the panel prints
// them on demand so that transcription is copy rather than retyping.
// ---------------------------------------------------------------------------

/**
 * Smoke.flashbang(), verbatim as of writing.
 *
 * Two sources, because one cannot be both the detonation and what is left
 * afterwards: a dense core that expands and is gone in a quarter second, and a
 * thin warm column that rises for six.
 */
const bang = {
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
  /** The light. FLASHBANG.intensity is ~35x a muzzle flash on purpose. */
  intensity: FLASHBANG.intensity,
  duration: FLASHBANG.duration,
  lightRadius: FLASHBANG.radius,
};

/**
 * Smoke.canisterCloud(), verbatim as of writing.
 *
 * temp 0 is load-bearing: buoyancy (1.4/unit) beats weight (0.045/unit) unless
 * density is ~30x temperature, so any warmth at all lifts the jet off the
 * floor it is supposed to hug. No expand either — a vent is momentum, and
 * expansion thins the cloud, which is the opposite of what concealment wants.
 */
const can = {
  radius: 0.55,
  density: 200,
  temp: 0,
  push: 45,
  speed: 9,
  rise: 0.4,
  seconds: 30,
};

/** Where the can lies and which way it vents; the demo has no thrown body. */
const vent = { yaw: 0, height: 0.3 };

/**
 * The showcase camera: a slow continuous orbit around the middle of the room.
 *
 * A grenade is a three-dimensional event and a fixed camera shows one side of
 * it. Orbiting is also the honest test — a cloud that looks right from one
 * angle and like a flat card from ninety degrees away is a cloud that is not
 * right yet.
 */
const orbit = { on: true, speed: 0.22, distance: 4.6, pitch: 0.34, height: 1.0 };

/**
 * Interior half-extent of buildGrenadeBox, and the reason the orbit is clamped.
 *
 * An orbit is a circle, and a circle of radius r puts the eye at r on every
 * axis in turn — so a distance that clears the corners still walks through the
 * middle of a wall a quarter turn later. The first cut sat at 5.73 m
 * horizontally against a wall at 5.16 m and rendered a black screen for most
 * of the sweep, which looked like the renderer was broken rather than like the
 * camera was outside the building.
 */
const ROOM_HALF = 5;

/** Fires the effect again on a timer, so it can be watched without touching a key. */
const replay = { on: true, every: 9 };

/** Everything the page throws, at the middle of the room. */
const CENTRE = { x: 0, z: 0 };

/** Replay bookkeeping. */
let nextFire = 1.0;
let whichNext = 0;

function fmt(n: number): string {
  return Number.isInteger(n) ? String(n) : String(+n.toFixed(3));
}

/**
 * Prints the current numbers in the shape the source files want them.
 *
 * Tuning ends with somebody copying values into a preset, and copying them out
 * of eleven separate sliders by eye is how a good set gets landed slightly
 * wrong.
 */
function dump(): void {
  console.log(
    `// game/smoke.ts  flashbang()\n`
    + `this.spawn({ pos: at, radius: ${fmt(bang.coreRadius)}, density: ${fmt(bang.coreDensity)},`
    + ` temp: ${fmt(bang.coreTemp)}, expand: ${fmt(bang.coreExpand)},`
    + ` push: 0, life: ${fmt(bang.coreLife)}, attack: 0.02 });\n`
    + `this.spawn({ pos: at, radius: ${fmt(bang.wispRadius)},`
    + ` vel: { x: 0, y: ${fmt(bang.wispRise)}, z: 0 }, push: 5,`
    + ` density: ${fmt(bang.wispDensity)}, temp: ${fmt(bang.wispTemp)},`
    + ` life: ${fmt(bang.wispLife)}, attack: 0.3 });\n\n`
    + `// game/flashes.ts  FLASHBANG\n`
    + `{ duration: ${fmt(bang.duration)}, intensity: ${fmt(bang.intensity)},`
    + ` radius: ${fmt(bang.lightRadius)} }\n\n`
    + `// game/smoke.ts  canisterCloud()\n`
    + `radius: ${fmt(can.radius)}, density: ${fmt(can.density)}, temp: ${fmt(can.temp)},`
    + ` push: ${fmt(can.push)}, speed: ${fmt(can.speed)}, rise: ${fmt(can.rise)}`,
  );
}

/**
 * The flashbang: a light, a core that expands and is gone, and a wisp that
 * rises for seconds. Thrown at the middle of the room rather than at the
 * cursor — this is a showcase, and the camera is already pointed there.
 */
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

/** The can: a cold directional vent along the panel's yaw. */
function fireCan(d: DemoDeps): void {
  const ax = Math.sin(vent.yaw), az = Math.cos(vent.yaw);
  d.smoke.spawn({
    pos: v3(CENTRE.x, vent.height, CENTRE.z),
    radius: can.radius,
    vel: v3(ax * can.speed, can.rise, az * can.speed),
    push: can.push, density: can.density, temp: can.temp,
    life: can.seconds, attack: 0.15,
  });
}

function groups(): GroupSpec[] {
  return [
    {
      title: "flashbang — light  (1)",
      items: [
        sl("intensity", 0, 400000, 5000,
          () => bang.intensity, (v) => (bang.intensity = v)),
        sl("duration s", 0.02, 0.6, 0.01,
          () => bang.duration, (v) => (bang.duration = v)),
        // Drives how soft the shadows it throws are, which is most of what
        // sells it as a light in the room rather than a flash on the lens.
        sl("light radius", 0.05, 1.5, 0.05,
          () => bang.lightRadius, (v) => (bang.lightRadius = v)),
      ],
    },
    {
      title: "flashbang — core",
      items: [
        sl("density /s", 0, 1200, 10,
          () => bang.coreDensity, (v) => (bang.coreDensity = v)),
        sl("radius", 0.1, 2, 0.05,
          () => bang.coreRadius, (v) => (bang.coreRadius = v)),
        sl("temp /s", 0, 80, 1, () => bang.coreTemp, (v) => (bang.coreTemp = v)),
        // div(v) ~ 3v/r, so 0.6 m pushing out at 10 m/s is about 50.
        sl("expand 1/s", 0, 300, 5,
          () => bang.coreExpand, (v) => (bang.coreExpand = v)),
        sl("life s", 0.05, 2, 0.05,
          () => bang.coreLife, (v) => (bang.coreLife = v)),
      ],
    },
    {
      title: "flashbang — wisp",
      items: [
        sl("density /s", 0, 200, 2,
          () => bang.wispDensity, (v) => (bang.wispDensity = v)),
        sl("radius", 0.1, 1.5, 0.05,
          () => bang.wispRadius, (v) => (bang.wispRadius = v)),
        sl("temp /s", 0, 40, 0.5, () => bang.wispTemp, (v) => (bang.wispTemp = v)),
        sl("rise m/s", 0, 6, 0.1, () => bang.wispRise, (v) => (bang.wispRise = v)),
        sl("life s", 0, 20, 0.5, () => bang.wispLife, (v) => (bang.wispLife = v)),
      ],
    },
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
      title: "smoke grenade  (2)",
      items: [
        sl("seconds", 1, 60, 1, () => can.seconds, (v) => (can.seconds = v)),
        sl("density /s", 0, 500, 5, () => can.density, (v) => (can.density = v)),
        sl("radius", 0.1, 2, 0.05, () => can.radius, (v) => (can.radius = v)),
        // Cold on purpose — see the note on `can`.
        sl("temp /s", 0, 20, 0.2, () => can.temp, (v) => (can.temp = v)),
        sl("jet speed m/s", 0, 25, 0.5, () => can.speed, (v) => (can.speed = v)),
        sl("push 1/s", 0, 120, 1, () => can.push, (v) => (can.push = v)),
        sl("upward bias m/s", 0, 3, 0.1, () => can.rise, (v) => (can.rise = v)),
        sl("vent yaw", -3.15, 3.15, 0.05, () => vent.yaw, (v) => (vent.yaw = v)),
      ],
    },
  ];
}

void bootDemo({
  build: buildGrenadeBox,
  groups,

  /**
   * A lattice for this room instead of for the office.
   *
   * This is the resolution that matters. The tracer samples one 3D texture, so
   * the *rendered* smoke is only ever as fine as this — the level's 25 cm makes
   * a 0.6 m burst about two voxels across, which is a blob however good the
   * solver underneath is.
   *
   * 8 x 3.5 x 8 m at 6.25 cm: 128 x 56 x 128 = 917k cells against the level
   * lattice's 389k, for a domain 29x smaller. Four times the linear detail in
   * the image. The room is 5 x 5 so the box clears it on every side, and the
   * origin is placed so the room sits in the middle of it.
   */
  smoke: {
    dims: [128, 56, 128],
    origin: [-4, 0, -4],
    cell: 0.0625,
  },
  help: "orbits on its own · O toggle orbit · right-drag look · wheel zoom · ` panel\n"
    + "1 flashbang · 2 smoke grenade · R clear · P print values to console",

  init(d) {
    // Framed on the crate from standing height, close enough that a 0.6 m
    // burst fills a useful part of the frame.
    d.camera.distance = 6.0;
    d.camera.pitch = 0.34;
    d.camera.yaw = 0.6;
    d.focus.x = 0;
    d.focus.z = 0;

    // Full internal resolution. The game runs at 0.5 for vblank headroom on a
    // 1920x1080 canvas, which is a hard 2x upscale — fine when you are moving
    // and fatal when you are staring at the edge of a cloud deciding whether
    // it has the structure you want. Nothing here is framerate-critical.
    d.settings.resolutionScale = 1.0;

    // THE GAME'S SETTINGS, VERBATIM. Same reason /demo/smoke does this: a
    // cloud that looks wonderful at exposure 0.06 against a softbox tells you
    // nothing about how it reads in a lit office at 0.35.
    d.settings.exposure = 0.35;
    d.settings.volumetric = 0.10;
    d.settings.fogAmount = 0.0;
    d.settings.volumetricSteps = 24;
    d.settings.smokeDetail = 1.35;
    d.settings.smokeDetailFreq = 9.0;
    d.settings.skyIntensity = 0.04;

    Object.assign(d.renderer.fluid.tune, {
      vorticity: 6.0, buoyancy: 6.0, weight: 0.3,
      dissipation: 0.22, cooling: 3.0, jacobi: 20,
    });

    // The fine lattice, on from the start and never moved.
    //
    // This is the difference between a blob and a cloud. The medium lattice is
    // 25 cm cells sized for a 52 m office, so a 0.6 m burst is about two and a
    // half cells across and there is nothing for vorticity to make wisps out
    // of. The fine lattice is 6.25 cm over 8 x 3.25 x 8 m — 128 x 52 x 128 —
    // and the room was built at 5 x 5 precisely so it fits inside that box
    // with room to spare.
    //
    // /demo/smoke anchors it per-event because a plume there can travel the
    // length of a hall. Here nothing leaves the room, so anchoring it once at
    // the origin means it never resets mid-effect.
    d.renderer.activateFine(CENTRE.x, CENTRE.z);
    d.resize();
  },

  step(d, dt, elapsed) {
    if (orbit.on) {
      // Clamped so no slider setting can put the eye through a wall: the
      // horizontal reach is distance * cos(pitch), and that is what has to fit.
      const maxR = ROOM_HALF - 0.7;
      const cp = Math.max(Math.cos(orbit.pitch), 0.2);
      d.camera.yaw = elapsed * orbit.speed;
      d.camera.pitch = orbit.pitch;
      d.camera.distance = Math.min(orbit.distance, maxR / cp);
      d.focus.x = CENTRE.x;
      d.focus.z = CENTRE.z;
    }
    if (!replay.on) return;
    // Re-fire on a timer so the effect can be watched, and judged, without a
    // hand on the keyboard. Both devices, staggered: the bang reads against a
    // clean room, the can against what the bang left behind.
    nextFire -= dt;
    if (nextFire > 0) return;
    nextFire = replay.every;
    d.smoke.reset(true);
    d.renderer.fluid.reset();
    d.renderer.activateFine(CENTRE.x, CENTRE.z);
    if (whichNext === 0) fireBang(d); else fireCan(d);
    whichNext ^= 1;
  },

  key(d, code) {
    switch (code) {
      case "Digit1":
        fireBang(d);
        break;
      case "Digit2":
        fireCan(d);
        break;
      case "KeyR":
        d.smoke.reset(true);
        d.renderer.fluid.reset();
        d.renderer.activateFine(CENTRE.x, CENTRE.z);
        break;
      case "KeyP":
        dump();
        break;
      case "KeyO":
        orbit.on = !orbit.on;
        break;
    }
  },
});
