import { Vec3, rotY, v3 } from "../core/math";
import { FLAG_NO_CAMERA, LIGHT_SPHERE, SceneBuilder, boxBounds } from "./scene";

/** Deterministic PRNG so the level (and therefore any render comparison) is stable. */
function mulberry32(seed: number) {
  let a = seed >>> 0;
  return () => {
    a = (a + 0x6d2b79f5) | 0;
    let t = Math.imul(a ^ (a >>> 15), 1 | a);
    t = (t + Math.imul(t ^ (t >>> 7), 61 | t)) ^ t;
    return ((t ^ (t >>> 14)) >>> 0) / 4294967296;
  };
}

export interface LevelInfo {
  /** Axis-aligned colliders in the XZ plane: [minX, minZ, maxX, maxZ]. */
  colliders: Float32Array;
  spawn: Vec3;
  bounds: { minX: number; minZ: number; maxX: number; maxZ: number };
}

const FLOOR_Y = 0;
const WALL_H = 3.2;
const PARTITION_H = 1.35;

export function buildOffice(s: SceneBuilder): LevelInfo {
  const rng = mulberry32(0x5eed1234);
  const colliders: number[] = [];

  // -- materials ------------------------------------------------------------
  // Roughness values matter a lot here: the polished-concrete corridor picking
  // up a long specular streak from the flashlight is most of the "photoreal"
  // read, and carpet killing that streak is what sells the contrast.
  const mConcrete = s.material(v3(0.30, 0.31, 0.335), 0.34, 0.0);
  const mCarpetBlue = s.material(v3(0.055, 0.085, 0.20), 0.94, 0.0);
  const mCarpetRed = s.material(v3(0.19, 0.045, 0.055), 0.94, 0.0);
  const mCarpetGrey = s.material(v3(0.11, 0.115, 0.125), 0.92, 0.0);
  const mWall = s.material(v3(0.62, 0.625, 0.63), 0.72, 0.0);
  const mWallWarm = s.material(v3(0.60, 0.565, 0.51), 0.75, 0.0);
  const mPartition = s.material(v3(0.44, 0.455, 0.47), 0.85, 0.0);
  const mDesk = s.material(v3(0.42, 0.32, 0.20), 0.42, 0.0);
  const mDeskGrey = s.material(v3(0.26, 0.265, 0.275), 0.5, 0.0);
  const mChair = s.material(v3(0.12, 0.19, 0.34), 0.62, 0.0);
  const mMetal = s.material(v3(0.55, 0.56, 0.58), 0.28, 1.0);
  const mDarkMetal = s.material(v3(0.10, 0.105, 0.11), 0.35, 1.0);
  const mCrate = s.material(v3(0.52, 0.375, 0.21), 0.86, 0.0);
  const mCrateLight = s.material(v3(0.60, 0.45, 0.27), 0.86, 0.0);
  const mPlastic = s.material(v3(0.18, 0.19, 0.20), 0.30, 0.0);

  // -- helpers --------------------------------------------------------------
  const addCollider = (minX: number, minZ: number, maxX: number, maxZ: number) => {
    colliders.push(minX, minZ, maxX, maxZ);
  };

  /**
   * Collider matching a box's real world footprint.
   *
   * The hand-written rects this replaces went wrong two ways. They were sized
   * by eye — the cubicle's 0.09m-thick back panel was given a 1.9x1.9m square,
   * so 0.95m of empty floor on either side of it was solid — and they ignored
   * yaw entirely, so any rotated piece kept its unrotated footprint. Deriving
   * from boxBounds cannot drift from the geometry it is supposed to represent.
   */
  const addBoxCollider = (center: Vec3, half: Vec3, rot?: [Vec3, Vec3, Vec3]) => {
    const b = boxBounds({ center, half, rot: rot ?? rotY(0), material: 0 });
    addCollider(b.min.x, b.min.z, b.max.x, b.max.z);
  };

  /** Axis-aligned wall from (x0,z0) to (x1,z1). */
  const wall = (
    x0: number, z0: number, x1: number, z1: number,
    h = WALL_H, t = 0.16, mat = mWall, collide = true,
  ) => {
    const cx = (x0 + x1) / 2;
    const cz = (z0 + z1) / 2;
    const hx = Math.max(Math.abs(x1 - x0) / 2, t / 2);
    const hz = Math.max(Math.abs(z1 - z0) / 2, t / 2);
    s.box(v3(cx, FLOOR_Y + h / 2, cz), v3(hx, h / 2, hz), mat);
    if (collide) addCollider(cx - hx, cz - hz, cx + hx, cz + hz);
  };

  const carpet = (cx: number, cz: number, hx: number, hz: number, mat: number) => {
    s.box(v3(cx, FLOOR_Y + 0.012, cz), v3(hx, 0.012, hz), mat);
  };

  /** Desk: top slab plus two end panels. */
  const desk = (x: number, z: number, yaw: number, w = 1.6, d = 0.75, mat = mDesk) => {
    const r = rotY(yaw);
    const topY = 0.74;
    s.box(v3(x, topY, z), v3(w / 2, 0.035, d / 2), mat, r);
    const c = Math.cos(yaw), sn = Math.sin(yaw);
    for (const side of [-1, 1]) {
      const ox = c * (side * (w / 2 - 0.06));
      const oz = -sn * (side * (w / 2 - 0.06));
      s.box(v3(x + ox, topY / 2, z + oz), v3(0.03, topY / 2, d / 2 - 0.05), mDeskGrey, r);
    }
    addBoxCollider(v3(x, topY, z), v3(w / 2, 0.035, d / 2), r);
  };

  /** Office chair: seat, back, post, star base. */
  const chair = (x: number, z: number, yaw: number) => {
    const r = rotY(yaw);
    s.box(v3(x, 0.46, z), v3(0.24, 0.045, 0.23), mChair, r);
    const c = Math.cos(yaw), sn = Math.sin(yaw);
    // Backrest sits behind the seat along the chair's local -Z.
    s.box(v3(x - sn * 0.21, 0.72, z - c * 0.21), v3(0.235, 0.24, 0.04), mChair, r);
    s.box(v3(x, 0.28, z), v3(0.035, 0.18, 0.035), mDarkMetal);
    for (let i = 0; i < 5; i++) {
      const a = yaw + (i / 5) * Math.PI * 2;
      s.box(
        v3(x + Math.sin(a) * 0.14, 0.075, z + Math.cos(a) * 0.14),
        v3(0.025, 0.02, 0.16),
        mDarkMetal,
        rotY(a),
      );
    }
  };

  /** Monitor with a faint emissive screen — a practical light source. */
  const monitor = (x: number, z: number, yaw: number, on: boolean) => {
    const r = rotY(yaw);
    const c = Math.cos(yaw), sn = Math.sin(yaw);
    s.box(v3(x, 0.80, z), v3(0.10, 0.025, 0.10), mPlastic, r);
    s.box(v3(x, 0.92, z), v3(0.02, 0.10, 0.02), mPlastic, r);
    s.box(v3(x, 1.13, z), v3(0.28, 0.18, 0.02), mPlastic, r);
    if (on) {
      // Screen faces local +Z.
      const sx = x + sn * 0.025;
      const sz = z + c * 0.025;
      const glow = v3(0.35, 0.55, 1.0);
      const matScreen = s.material(v3(0, 0, 0), 1, 0, v3(glow.x * 1.6, glow.y * 1.6, glow.z * 1.6));
      s.box(v3(sx, 1.13, sz), v3(0.255, 0.155, 0.006), matScreen, r);
      s.light({
        pos: v3(sx + sn * 0.05, 1.13, sz + c * 0.05),
        kind: LIGHT_SPHERE,
        dir: v3(sn, 0, c),
        radius: 0.22,
        color: glow,
        intensity: 1.1,
        cosInner: -1,
        cosOuter: -1,
        emissiveMat: matScreen,
      });
    }
  };

  const crate = (x: number, z: number, size: number, yaw: number, mat: number) => {
    s.box(v3(x, size / 2, z), v3(size / 2, size / 2, size / 2), mat, rotY(yaw));
    addCollider(x - size / 2, z - size / 2, x + size / 2, z + size / 2);
  };

  /** One cubicle: an L of partition walls, a desk, a chair, a monitor. */
  const cubicle = (x: number, z: number, yaw: number, screenOn: boolean) => {
    const c = Math.cos(yaw), sn = Math.sin(yaw);
    const local = (lx: number, lz: number): [number, number] => [
      x + c * lx + sn * lz,
      z - sn * lx + c * lz,
    ];
    const r = rotY(yaw);
    // Back panel and one side panel, forming an L opening toward local +Z.
    const [bx, bz] = local(0, -0.95);
    const backHalf = v3(0.95, PARTITION_H / 2, 0.045);
    s.box(v3(bx, PARTITION_H / 2, bz), backHalf, mPartition, r);
    addBoxCollider(v3(bx, PARTITION_H / 2, bz), backHalf, r);
    const [sx, sz] = local(-0.95, 0);
    const sideHalf = v3(0.045, PARTITION_H / 2, 0.95);
    s.box(v3(sx, PARTITION_H / 2, sz), sideHalf, mPartition, r);
    // The side panel had no collider at all, so the player walked through it.
    addBoxCollider(v3(sx, PARTITION_H / 2, sz), sideHalf, r);

    const [dx, dz] = local(0, -0.6);
    desk(dx, dz, yaw, 1.7, 0.7, mDeskGrey);
    const [mx, mz] = local(0.1, -0.75);
    monitor(mx, mz, yaw, screenOn);
    const [cx2, cz2] = local(-0.05, 0.05);
    chair(cx2, cz2, yaw + Math.PI + (rng() - 0.5) * 0.9);
  };

  /** Emissive strip in a housing — the fluorescent tubes that light far rooms. */
  const fluorescent = (x: number, z: number, yaw: number, color: Vec3, power: number) => {
    const r = rotY(yaw);
    s.box(v3(x, WALL_H - 0.06, z), v3(0.62, 0.05, 0.14), mMetal, r);
    // Tubes are large and are meant to actually light a room, so intensity runs
    // well ahead of the surface radiance.
    s.areaLight(v3(x, WALL_H - 0.14, z), v3(0.58, 0.02, 0.10), color, power, power * 9.0, r);
  };

  /** Exit sign: dim, but it should visibly wash green onto whatever is near it. */
  const exitSign = (x: number, z: number, yaw: number) => {
    s.areaLight(
      v3(x, 2.35, z), v3(0.16, 0.075, 0.02),
      // Intensity dropped from 3.0 once ReSTIR started sampling it honestly:
      // the old estimator culled sub-threshold samples, which quietly hid most
      // of this light's reach. It is a sign, not a lamp.
      v3(0.15, 1.0, 0.35), 2.4, 1.1, rotY(yaw),
    );
  };

  // -- floor plate ----------------------------------------------------------
  const MINX = -26, MAXX = 26, MINZ = -18, MAXZ = 18;
  s.box(
    v3((MINX + MAXX) / 2, FLOOR_Y - 0.5, (MINZ + MAXZ) / 2),
    v3((MAXX - MINX) / 2 + 1, 0.5, (MAXZ - MINZ) / 2 + 1),
    mConcrete,
  );

  /**
   * A facade with genuine openings — sill below, header above, piers between.
   *
   * This matters more than it looks: an opaque box of "glass" sunk into a solid
   * wall blocks every shadow ray, so no exterior light can ever reach the
   * floor. Real holes let the moon throw the long slanted pools that carry the
   * whole mood.
   */
  const facade = (
    x0: number, x1: number, z: number,
    winW = 2.4, sill = 0.95, head = 2.45, spacing = 7.0,
  ) => {
    const t = 0.3;
    const centers: number[] = [];
    for (let x = x0 + spacing * 0.6; x < x1 - winW; x += spacing) centers.push(x);

    // Piers between openings (and the two end returns).
    let cursor = x0;
    for (const c of centers) {
      const left = c - winW / 2;
      if (left > cursor) wall(cursor, z, left, z, WALL_H, t, mWallWarm);
      cursor = c + winW / 2;
    }
    if (cursor < x1) wall(cursor, z, x1, z, WALL_H, t, mWallWarm);

    for (const c of centers) {
      const cx = c;
      // Sill and header close the opening top and bottom.
      s.box(v3(cx, sill / 2, z), v3(winW / 2, sill / 2, t / 2), mWallWarm);
      s.box(v3(cx, (head + WALL_H) / 2, z), v3(winW / 2, (WALL_H - head) / 2, t / 2), mWallWarm);
      // Frame reveal, inset so it does not seal the hole.
      const fy = (sill + head) / 2;
      const fh = (head - sill) / 2;
      for (const side of [-1, 1]) {
        s.box(v3(cx + side * (winW / 2 - 0.04), fy, z), v3(0.04, fh, t / 2 + 0.01), mDarkMetal);
      }
      s.box(v3(cx, fy, z), v3(winW / 2, 0.035, t / 2 + 0.01), mDarkMetal);
      // Blocks walking through the window.
      addCollider(cx - winW / 2, z - t / 2, cx + winW / 2, z + t / 2);
    }
  };

  facade(MINX, MAXX, MINZ);
  facade(MINX, MAXX, MAXZ);
  wall(MINX, MINZ, MINX, MAXZ, WALL_H, 0.3, mWallWarm);
  wall(MAXX, MINZ, MAXX, MAXZ, WALL_H, 0.3, mWallWarm);

  // Ceiling. Flagged invisible to camera rays so the cutaway view still works,
  // but fully present for shadow and bounce rays: it keeps the moon out except
  // through the windows, and gives the rooms real interior bounce light.
  // One slab, deliberately. Splitting it into an 8x6 grid so the BVH could cull
  // it looked like an obvious win and measured as a 12% loss (63.8 -> 56.3 fps
  // at 1152x720/2-bounce): a single huge box costs one cheap OBB test that
  // almost always resolves immediately, whereas 48 tiles add BVH levels that
  // every upward ray has to descend. Do not "optimise" this again without
  // measuring.
  const mCeiling = s.material(v3(0.55, 0.55, 0.56), 0.9, 0.0);
  s.box(
    v3((MINX + MAXX) / 2, WALL_H + 0.15, (MINZ + MAXZ) / 2),
    v3((MAXX - MINX) / 2 + 1, 0.15, (MAXZ - MINZ) / 2 + 1),
    mCeiling,
    undefined,
    FLAG_NO_CAMERA,
  );

  // The moon. Far away and physically large, so the pools it throws through the
  // windows have crisp edges with just a hint of penumbra.
  //
  // MUST be the first light registered: the shader treats lights[0] as the key
  // light and samples it on its own channel. main.ts asserts this at startup.
  s.light({
    pos: v3(14, 34, -74),
    kind: LIGHT_SPHERE,
    dir: v3(0, -1, 0),
    radius: 9.0,
    color: v3(0.62, 0.74, 1.0),
    // Inverse-square over ~80m, so this has to be large to arrive at ~2.
    intensity: 34000,
    cosInner: -1,
    cosOuter: -1,
  });

  // -- central corridor -----------------------------------------------------
  carpet(0, 0, 24, 2.6, mCarpetGrey);
  // Polished concrete strip down the middle of the corridor.
  s.box(v3(0, FLOOR_Y + 0.030, 0), v3(23, 0.006, 1.5), mConcrete);

  // -- north cubicle farm (blue carpet) ------------------------------------
  carpet(-9, -10, 12, 6.5, mCarpetBlue);
  for (let i = 0; i < 4; i++) {
    for (let j = 0; j < 2; j++) {
      cubicle(-18 + i * 4.6, -13.2 + j * 5.2, j === 0 ? 0 : Math.PI, rng() < 0.28);
    }
  }

  // -- north-east open area (red carpet) ------------------------------------
  carpet(11, -10, 10, 6.5, mCarpetRed);
  for (let i = 0; i < 3; i++) {
    for (let j = 0; j < 2; j++) {
      cubicle(3.5 + i * 4.6, -13.2 + j * 5.2, j === 0 ? 0 : Math.PI, rng() < 0.28);
    }
  }

  // -- conference room, west ------------------------------------------------
  wall(-26, -4.5, -14, -4.5, WALL_H, 0.16, mWall);
  wall(-14, -4.5, -14, 6.5, WALL_H, 0.16, mWall);
  wall(-26, 6.5, -18.5, 6.5, WALL_H, 0.16, mWall); // doorway gap at x >= -18.5
  carpet(-20, 1, 6, 5.5, mCarpetRed);
  desk(-20, 1, 0, 5.2, 1.5, mDesk);
  for (let i = 0; i < 3; i++) {
    chair(-22.2 + i * 2.2, -0.5, Math.PI + (rng() - 0.5) * 0.5);
    chair(-22.2 + i * 2.2, 2.5, (rng() - 0.5) * 0.5);
  }
  fluorescent(-20, 1, 0, v3(1.0, 0.92, 0.74), 3.4);
  exitSign(-18.3, 6.4, 0);

  // -- south-west storage: the crate pile from the reference -----------------
  carpet(-6, 10, 8, 6, mCarpetBlue);
  for (let i = 0; i < 22; i++) {
    const x = -13 + rng() * 14;
    const z = 5.5 + rng() * 9;
    const size = 0.55 + rng() * 0.5;
    crate(x, z, size, rng() * Math.PI, rng() < 0.4 ? mCrateLight : mCrate);
  }
  // A few stacked two high for vertical interest and shadow layering.
  for (let i = 0; i < 5; i++) {
    const x = -12 + rng() * 12;
    const z = 6 + rng() * 8;
    crate(x, z, 0.7, rng() * Math.PI, mCrate);
    s.box(v3(x, 0.7 + 0.28, z), v3(0.28, 0.28, 0.28), mCrateLight, rotY(rng() * Math.PI));
  }

  // -- south-east server / office block -------------------------------------
  wall(8, 4.5, 24, 4.5, WALL_H, 0.16, mWall);
  wall(8, 4.5, 8, 16, WALL_H, 0.16, mWall);
  carpet(16, 10, 8, 6, mCarpetRed);
  for (let i = 0; i < 4; i++) {
    s.box(v3(10.5 + i * 3.4, 1.0, 8.5), v3(0.45, 1.0, 0.9), mDarkMetal);
    // Rack status LEDs.
    s.areaLight(
      v3(10.5 + i * 3.4, 1.4, 7.58),
      v3(0.30, 0.015, 0.012),
      i % 2 === 0 ? v3(0.2, 1.0, 0.4) : v3(1.0, 0.35, 0.15),
      1.6, 0.9,
    );
    addCollider(10.5 + i * 3.4 - 0.45, 7.6, 10.5 + i * 3.4 + 0.45, 9.4);
  }
  for (let i = 0; i < 4; i++) cubicle(11 + i * 4.4, 13.5, Math.PI, rng() < 0.5);
  fluorescent(14, 12.5, 0, v3(0.72, 0.85, 1.0), 2.0);
  exitSign(8.2, 4.4, Math.PI / 2);

  // -- scattered detail through the corridor --------------------------------
  for (let i = 0; i < 7; i++) {
    const x = -20 + rng() * 40;
    const z = (rng() < 0.5 ? -1 : 1) * (1.8 + rng() * 0.7);
    crate(x, z, 0.45 + rng() * 0.25, rng() * Math.PI, mCrate);
  }
  // Support columns down the corridor: great for casting long shadows.
  for (let i = 0; i < 5; i++) {
    const x = -18 + i * 9;
    s.box(v3(x, WALL_H / 2, -3.9), v3(0.28, WALL_H / 2, 0.28), mWallWarm);
    addCollider(x - 0.28, -4.18, x + 0.28, -3.62);
    s.box(v3(x, WALL_H / 2, 3.9), v3(0.28, WALL_H / 2, 0.28), mWallWarm);
    addCollider(x - 0.28, 3.62, x + 0.28, 4.18);
  }

  // A single failing fluorescent over the corridor's east end.
  fluorescent(19, 0, Math.PI / 2, v3(0.80, 0.88, 1.0), 2.6);

  // Night-time practicals. A sealed ceiling makes the interior genuinely black,
  // which is correct but unplayable — a real office after hours keeps emergency
  // and security lighting on, so this is both the physical and the design fix.
  // The warm/cool split also gives the frame some colour to work with.
  for (let i = 0; i < 4; i++) {
    fluorescent(-16 + i * 11, -8.6, 0, v3(0.70, 0.82, 1.0), 0.55);
  }
  for (let i = 0; i < 3; i++) {
    fluorescent(-11 + i * 11, 9.5, 0, v3(1.0, 0.88, 0.68), 0.5);
  }
  exitSign(-25.6, -12, Math.PI / 2);
  exitSign(25.6, 6, -Math.PI / 2);
  exitSign(2.0, -4.4, 0);

  // Wall-mounted emergency units: small, dim, warm, and low down so they rake
  // light along the floor rather than flooding the room.
  for (const [ex, ez] of [[-21, -3.9], [1, 3.9], [13, -3.9], [22, 3.9]]) {
    s.areaLight(v3(ex, 2.1, ez), v3(0.11, 0.05, 0.05), v3(1.0, 0.72, 0.42), 1.6, 5.0);
  }

  return {
    colliders: new Float32Array(colliders),
    // Corridor, west end: cubicle farm to the north, crate store to the south,
    // and a clear run of floor ahead for the beam. Deliberately not adjacent to
    // a practical — standing next to one blows out the opening frame.
    spawn: v3(-13, 0, 0.5),
    bounds: { minX: MINX, minZ: MINZ, maxX: MAXX, maxZ: MAXZ },
  };
}
