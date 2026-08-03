import { v3 } from "../core/math";
import { GroupSpec } from "../ui/panel";
import { buildSmokeLab } from "../scene/demo";
import { FLASHBANG } from "../game/flashes";
import { bootDemo, sl } from "./common";

// ---------------------------------------------------------------------------
// /demo/smoke — one hall, a countable set of obstacles, and every emitter on a
// key so the same shot can be fired repeatedly while a slider moves.
//
// The point is the parameters, not the scene: the source presets in
// game/smoke.ts are the thing being tuned, so they are mirrored here as live
// values rather than called through their fixed wrappers. When a set of
// numbers is right, it moves back into the preset.
// ---------------------------------------------------------------------------

/** Live copies of the presets under test. */
const jet = {
  radius: 0.7,
  density: 120,
  temp: 0,
  push: 45,
  speed: 10,
  expand: 0,
  seconds: 30,
};

const burst = {
  radius: 0.6,
  density: 450,
  temp: 30,
  expand: 70,
  /** Peak of the transient light, in the same units as every other light. */
  bang: FLASHBANG.intensity,
};

const muzzle = {
  radius: 0.6,
  density: 70,
  temp: 4,
  push: 60,
  speed: 4.5,
  /** Seconds the barrel keeps trailing after the shot itself. */
  wispSeconds: 2.5,
};

function groups(): GroupSpec[] {
  return [
    {
      title: "canister jet  (1)",
      items: [
        sl("seconds", 1, 60, 1, () => jet.seconds, (v) => (jet.seconds = v)),
        sl("density /s", 0, 400, 5, () => jet.density, (v) => (jet.density = v)),
        sl("radius", 0.1, 2, 0.05, () => jet.radius, (v) => (jet.radius = v)),
        // Cold on purpose: the reference footage is a cold jet that hugs the
        // ground, so `weight` should beat buoyancy outright.
        sl("temp /s", 0, 20, 0.2, () => jet.temp, (v) => (jet.temp = v)),
        sl("jet speed m/s", 0, 25, 0.5, () => jet.speed, (v) => (jet.speed = v)),
        sl("push 1/s", 0, 120, 1, () => jet.push, (v) => (jet.push = v)),
        sl("expand 1/s", 0, 120, 1, () => jet.expand, (v) => (jet.expand = v)),
      ],
    },
    {
      title: "flashbang  (2)",
      items: [
        sl("density /s", 0, 1200, 10, () => burst.density, (v) => (burst.density = v)),
        sl("radius", 0.1, 2, 0.05, () => burst.radius, (v) => (burst.radius = v)),
        sl("temp /s", 0, 80, 1, () => burst.temp, (v) => (burst.temp = v)),
        // div(v) ~ 3v/r, so 0.6 m pushing out at 10 m/s is about 50.
        sl("expand 1/s", 0, 300, 5, () => burst.expand, (v) => (burst.expand = v)),
        sl("bang intensity", 0, 400000, 5000,
          () => burst.bang, (v) => (burst.bang = v)),
      ],
    },
    {
      title: "muzzle  (3)",
      items: [
        sl("density /s", 0, 300, 5, () => muzzle.density, (v) => (muzzle.density = v)),
        sl("radius", 0.1, 2, 0.05, () => muzzle.radius, (v) => (muzzle.radius = v)),
        sl("temp /s", 0, 40, 0.5, () => muzzle.temp, (v) => (muzzle.temp = v)),
        sl("speed m/s", 0, 20, 0.5, () => muzzle.speed, (v) => (muzzle.speed = v)),
        sl("wisp seconds", 0, 8, 0.25,
          () => muzzle.wispSeconds, (v) => (muzzle.wispSeconds = v)),
      ],
    },
    {
      title: "walker",
      items: [
        sl("push 1/s", 0, 60, 1, () => walker.push, (v) => (walker.push = v)),
        sl("radius", 0.1, 1.5, 0.05, () => walker.radius, (v) => (walker.radius = v)),
      ],
    },
  ];
}

/**
 * The observer as a velocity source: no density, just drag.
 *
 * This is the cheapest form of "smoke reacts to the player" — the Source
 * struct's push/vel already do exactly this, so a character stirring the
 * medium costs one source slot and no new machinery. Off at push 0.
 */
const walker = { push: 18, radius: 0.5 };

void bootDemo({
  build: buildSmokeLab,
  groups,
  help: "WASD walk · right-drag orbit · wheel zoom · F torch · ` panel\n"
    + "1 canister jet · 2 flashbang · 3 muzzle · 4 smoulder · 5 cold puff · R clear",

  init(d) {
    // Framed onto the FLOOR, where a cold canister cloud actually lives, and
    // from a standing eye height rather than lying in the cloud with it.
    d.camera.distance = 8.5;
    d.camera.pitch = 0.42;
    d.camera.yaw = 0.35;
    d.focus.x = 0;
    d.focus.z = 0.5;

    // THE GAME'S SETTINGS, VERBATIM from DEFAULT_SETTINGS in engine/renderer.ts.
    //
    // This demo used to open at exposure 0.06, extinction 1.35 and fog 0, which
    // is a regime the game never enters — it made smoke look wonderful here and
    // told you nothing about how it would look in the office. Anything tuned
    // against those numbers was tuned against a fiction.
    //
    // The room haze is off, which is what let extinction be set for the cloud
    // instead of splitting the difference with a 52 m fog. Whatever gets tuned
    // here transfers, because these ARE the game's numbers.
    d.settings.exposure = 0.35;
    d.settings.volumetric = 0.55;
    d.settings.fogAmount = 0.0;
    d.settings.volumetricSteps = 24;
    d.settings.smokeDetail = 1.35;
    d.settings.smokeDetailFreq = 9.0;
    d.settings.skyIntensity = 0.04;

    // Vorticity 6 would ball the plume up on its own — my sweep found the knee
    // at ~3 with buoyancy at its 1.4 default. Pairing it with buoyancy 6 is a
    // different operating point, not a violation of that result: the extra lift
    // outruns the confinement instead of losing to it.
    //
    // jacobi 20, not 200. Measured across 8/20/60/200: the divergence residual
    // falls 10x (1.2e-3 -> 1.2e-4) and nothing else moves — mass 41.08 vs
    // 41.13, peak 13.2 vs 13.2, ~1880 cells for all four.
    Object.assign(d.renderer.fluid.tune, {
      vorticity: 6.0, buoyancy: 6.0, weight: 0.3,
      dissipation: 0.22, cooling: 3.0, jacobi: 20,
    });
  },

  key(d, code) {
    const at = d.cursor;
    switch (code) {
      case "Digit1": {
        // Anchor the fine lattice on the vent. The canister is the one event
        // that both lasts long enough to be worth 26 rows and stays put.
        d.renderer.activateFine(at.x, at.z);
        // Jetting away from the observer, along the ground: the reference is a
        // canister lying on tarmac venting sideways, not a rising plume.
        const dx = at.x - d.focus.x, dz = at.z - d.focus.z;
        const l = Math.hypot(dx, dz) || 1;
        d.smoke.spawn({
          pos: v3(at.x, 0.3, at.z),
          radius: jet.radius,
          vel: v3((dx / l) * jet.speed, 0.4, (dz / l) * jet.speed),
          push: jet.push,
          density: jet.density,
          temp: jet.temp,
          expand: jet.expand,
          life: jet.seconds,
          attack: 0.15,
        });
        break;
      }
      case "Digit2":
        d.renderer.activateFine(at.x, at.z);
        // The bang is a real light, so it lights the room through the same
        // path as everything else — the smoke it just created included. An
        // additive sprite could not throw shadows off the columns or pick out
        // the plume it is sitting inside.
        if (burst.bang > 0) {
          d.flashes.spawn(v3(at.x, 1.0, at.z), { ...FLASHBANG, intensity: burst.bang });
        }
        d.smoke.spawn({
          pos: v3(at.x, 1.0, at.z),
          radius: burst.radius,
          density: burst.density,
          temp: burst.temp,
          expand: burst.expand,
          life: 0.25,
          attack: 0.02,
        });
        d.smoke.spawn({
          pos: v3(at.x, 1.0, at.z),
          radius: 0.4, vel: v3(0, 1.2, 0), push: 5,
          density: 14, temp: 7, life: 6, attack: 0.3,
        });
        break;
      case "Digit3": {
        const dx = at.x - d.focus.x, dz = at.z - d.focus.z;
        const l = Math.hypot(dx, dz) || 1;
        const bx = d.focus.x + (dx / l) * 0.5, bz = d.focus.z + (dz / l) * 0.5;
        d.smoke.spawn({
          pos: v3(bx, 1.2, bz),
          radius: muzzle.radius,
          vel: v3((dx / l) * muzzle.speed, 0.6, (dz / l) * muzzle.speed),
          push: muzzle.push,
          density: muzzle.density,
          temp: muzzle.temp,
          life: 0.14,
        });
        // The trailing wisp is a separate, much weaker source that stays on the
        // barrel. One source cannot be both the shot and the smoulder after it.
        if (muzzle.wispSeconds > 0) {
          d.smoke.spawn({
            pos: v3(bx, 1.2, bz),
            radius: 0.18, vel: v3(0, 0.4, 0), push: 2,
            density: 5, temp: 3, life: muzzle.wispSeconds, attack: 0.05,
          });
        }
        break;
      }
      case "Digit4":
        d.smoke.smolder(v3(at.x, 2.6, at.z));
        break;
      case "Digit5":
        d.smoke.puff(at.x, 1.0, at.z, 0.6, 40);
        break;
      case "KeyR":
        d.smoke.reset(true);
        d.renderer.fluid.reset();
        d.renderer.deactivateFine();
        break;
    }
  },

  step(d) {
    if (walker.push <= 0) return;
    // Re-spawned each frame rather than kept alive with a `follow`, because a
    // one-frame source is what a moving obstacle actually is: it should drag
    // the air it is in now, not the air it was in when it was created.
    const v = d.walkVel;
    if (Math.hypot(v.x, v.z) < 0.05) return;
    d.smoke.spawn({
      pos: v3(d.focus.x, 1.0, d.focus.z),
      radius: walker.radius,
      vel: v3(v.x, 0, v.z),
      push: walker.push,
      density: 0,
      life: 0.02,
    });
  },
});
