import { Vec3, v3 } from "../core/math";
import { LevelInfo } from "./level";
import { FLAG_NO_CAMERA, SceneBuilder } from "./scene";

// ---------------------------------------------------------------------------
// Minimal scenes for the /demo routes.
//
// These exist to make one subsystem legible, not to look like a level. The
// office is 532 boxes of overlapping evidence: when smoke pools oddly or a
// wall bounces light in patches, you cannot tell which of thirty nearby
// surfaces caused it. Each scene below is built so that exactly one thing is
// varying and the geometry it interacts with is countable.
//
// Both keep the same `buildOffice` shape (SceneBuilder in, LevelInfo out), so
// nothing downstream needs to know which scene it got.
// ---------------------------------------------------------------------------

const WALL_H = 3.2;

/**
 * Collider for an axis-aligned box. Every piece in these scenes is unrotated,
 * so the footprint is the box, and going through boxBounds would only add an
 * import to say so.
 */
function footprint(out: number[], c: Vec3, h: Vec3): void {
  out.push(c.x - h.x, c.z - h.z, c.x + h.x, c.z + h.z);
}

/** Shared: floor slab, four walls, and the collider/bounds bookkeeping. */
function room(
  s: SceneBuilder, hx: number, hz: number, floorMat: number, wallMat: number,
): { colliders: number[]; bounds: LevelInfo["bounds"] } {
  const colliders: number[] = [];
  const t = 0.16;
  s.box(v3(0, -0.05, 0), v3(hx, 0.05, hz), floorMat);
  // Walls sit just outside the interior so the nominal half-extent is the
  // usable floor, which is what the fluid grid and the camera framing assume.
  const wall = (cx: number, cz: number, wx: number, wz: number) => {
    s.box(v3(cx, WALL_H / 2, cz), v3(wx, WALL_H / 2, wz), wallMat);
    colliders.push(cx - wx, cz - wz, cx + wx, cz + wz);
  };
  wall(0, -hz - t, hx + t * 2, t);
  wall(0, hz + t, hx + t * 2, t);
  wall(-hx - t, 0, t, hz);
  wall(hx + t, 0, t, hz);
  return { colliders, bounds: { minX: -hx, minZ: -hz, maxX: hx, maxZ: hz } };
}

/**
 * `/demo/smoke` — an empty hall with a countable set of obstacles.
 *
 * Sized to sit well inside the fluid lattice (52 x 3.25 x 36 from
 * MEDIUM_ORIGIN) so no plume ever reaches a grid wall and gets a boundary
 * condition confused for a result. The obstacles are the three cases worth
 * watching: a free column to split a plume around, a waist-high barrier to
 * pool against, and an alcove to fill.
 */
export function buildSmokeLab(s: SceneBuilder): LevelInfo {
  // A LIT OFFICE ROOM, using the level's own materials and fixtures.
  //
  // This used to be a dark room with a bright softbox behind the smoke, which
  // reads a silhouette beautifully and calibrates nothing: a cloud tuned
  // against a 60-intensity panel at exposure 0.06 is tuned for a regime the
  // game never enters. The game is a lit interior at exposure 0.35 with
  // fluorescent strips, and any density, extinction or detail setting that is
  // going to ship has to be chosen there.
  //
  // Values below are copied from scene/level.ts, not approximated: same 3.2 m
  // ceiling, same wall albedo, same carpet, same fixture geometry and the same
  // 1:9 radiance-to-intensity convention. When something looks right here it
  // looks the same in the office.
  const mFloor = s.material(v3(0.11, 0.115, 0.125), 0.92, 0.0);   // mCarpetGrey
  const mWall = s.material(v3(0.62, 0.625, 0.63), 0.72, 0.0);     // mWall
  const mColumn = s.material(v3(0.60, 0.565, 0.51), 0.75, 0.0);   // mWallWarm
  const mBarrier = s.material(v3(0.55, 0.56, 0.58), 0.28, 1.0);   // mMetal

  const HX = 12, HZ = 8;
  const { colliders, bounds } = room(s, HX, HZ, mFloor, mWall);
  const collide = (c: Vec3, h: Vec3) => footprint(colliders, c, h);

  // 0.56 m columns: the same width the track report's column test used, and
  // just over two 0.25 m fluid cells, so a plume splitting around one is the
  // occupancy grid working rather than a single-cell artefact.
  for (const x of [-3, 3]) {
    const c = v3(x, WALL_H / 2, 0), h = v3(0.28, WALL_H / 2, 0.28);
    s.box(c, h, mColumn);
    collide(c, h);
  }

  // Waist-high barrier: smoke should pile against the upwind face and spill
  // over the top, which is the clearest read on whether weight is doing
  // anything.
  {
    const c = v3(0, 0.6, 4.5), h = v3(5, 0.6, 0.12);
    s.box(c, h, mBarrier);
    collide(c, h);
  }

  // Alcove in the far wall — a pocket that should fill and hold.
  for (const dx of [-3.2, 3.2]) {
    const c = v3(dx, WALL_H / 2, -5.5), h = v3(0.12, WALL_H / 2, 2.5);
    s.box(c, h, mBarrier);
    collide(c, h);
  }

  // Ceiling fluorescents, identical to level.ts's `fluorescent`: a metal
  // housing with the emitter recessed into it, radiance `power` and intensity
  // `power * 9`. Power 2.0 and the same cool tube colour the office uses.
  //
  // A grid rather than one strip, because a single source gives the cloud one
  // hard shadow and a flattering rim; an office is many weak overlapping
  // sources, which is a much less forgiving thing for smoke to sit in and is
  // the case worth tuning against.
  const tube = (x: number, z: number) => {
    s.box(v3(x, WALL_H - 0.06, z), v3(0.62, 0.05, 0.14), mBarrier);
    s.areaLight(
      v3(x, WALL_H - 0.14, z), v3(0.58, 0.02, 0.10),
      v3(0.72, 0.85, 1.0), 2.0, 18.0,
    );
  };
  for (const z of [-4.5, 0, 4.5]) for (const x of [-7, 0, 7]) tube(x, z);

  return { colliders: new Float32Array(colliders), spawn: v3(0, 0, 6.5), bounds };
}

/**
 * `/demo/grenades` — one crate in a small room, and nothing else.
 *
 * The smoke lab is a 24x16 hall with columns, a barrier and two alcoves,
 * because those are the cases that tell you whether the *solver* is right. A
 * grenade is a different question: it is over in a couple of seconds and what
 * matters is how it looks from the outside, so the obstacles stop being
 * evidence and start being clutter you have to orbit past.
 *
 * So: a crate to throw it at, four walls close enough to bounce off, and the
 * office's own lighting regime. Small enough that one orbit sees every side of
 * the cloud, which is the whole point of tuning here rather than in the level.
 *
 * Same materials, ceiling height, fixture geometry and 1:9 radiance-to-
 * intensity convention as scene/level.ts, for the same reason the smoke lab
 * copies them: a cloud tuned in a regime the game never enters is tuned
 * against a fiction.
 */
export function buildGrenadeBox(s: SceneBuilder): LevelInfo {
  const mFloor = s.material(v3(0.11, 0.115, 0.125), 0.92, 0.0);   // mCarpetGrey
  const mWall = s.material(v3(0.62, 0.625, 0.63), 0.72, 0.0);     // mWall
  const mMetal = s.material(v3(0.55, 0.56, 0.58), 0.28, 1.0);     // mMetal
  const mCrate = s.material(v3(0.52, 0.375, 0.21), 0.86, 0.0);    // mCrate

  // 5x5 of usable floor. Big enough that a 0.6 m burst is not touching a wall
  // on frame one, small enough to orbit in one gesture.
  const HX = 5, HZ = 5;
  const { colliders, bounds } = room(s, HX, HZ, mFloor, mWall);

  // The box. Waist-high and off-centre: a cloud has to both wrap it and be
  // occluded by it, and dead centre would make the orbit symmetric and hide
  // exactly that.
  {
    const c = v3(-0.9, 0.5, -0.4), h = v3(0.5, 0.5, 0.5);
    s.box(c, h, mCrate);
    footprint(colliders, c, h);
  }

  // Ceiling, invisible to camera rays so an overhead orbit is not looking at
  // the outside of a closed lid. It still occludes and still bounces.
  s.box(v3(0, WALL_H + 0.05, 0), v3(HX, 0.05, HZ), mWall, undefined, FLAG_NO_CAMERA);

  // Two fluorescents rather than one. A single source gives the cloud one hard
  // shadow and a flattering rim; overlapping weak sources are both what an
  // office is and the less forgiving case to tune against.
  const tube = (x: number, z: number) => {
    s.box(v3(x, WALL_H - 0.06, z), v3(0.62, 0.05, 0.14), mMetal);
    s.areaLight(
      v3(x, WALL_H - 0.14, z), v3(0.58, 0.02, 0.10),
      v3(0.72, 0.85, 1.0), 2.0, 18.0,
    );
  };
  tube(-2.2, 0);
  tube(2.2, 0);

  return { colliders: new Float32Array(colliders), spawn: v3(0, 0, 3), bounds };
}

/**
 * `/demo/indirect` — a colour-bleed box, deliberately small.
 *
 * Saturated side walls make bounce light unmistakable: anything red on the
 * floor arrived there indirectly. Small on purpose — the radiosity patch grid
 * floors at 0.4 m, so on a 4 m wall a patch is a tenth of it and quantization
 * is visible by eye rather than by instrument. The lone crate gives a near-
 * field case (contact shadow plus bounce onto an adjacent face) at the scale
 * where the artefact was reported.
 */
export function buildIndirectBox(s: SceneBuilder): LevelInfo {
  const mFloor = s.material(v3(0.55, 0.55, 0.55), 0.80, 0.0);
  const mWhite = s.material(v3(0.70, 0.70, 0.70), 0.75, 0.0);
  const mRed = s.material(v3(0.62, 0.09, 0.08), 0.85, 0.0);
  const mGreen = s.material(v3(0.10, 0.52, 0.12), 0.85, 0.0);
  const mCrate = s.material(v3(0.52, 0.375, 0.21), 0.86, 0.0);

  const HX = 4, HZ = 4;
  const { colliders, bounds } = room(s, HX, HZ, mFloor, mWhite);
  const collide = (c: Vec3, h: Vec3) => footprint(colliders, c, h);

  // Coloured side walls, inset so they read as surfaces rather than as the
  // room's own shell.
  const t = 0.08;
  s.box(v3(-HX + t, WALL_H / 2, 0), v3(t, WALL_H / 2, HZ), mRed);
  s.box(v3(HX - t, WALL_H / 2, 0), v3(t, WALL_H / 2, HZ), mGreen);
  // Ceiling, so bounce has somewhere to come back from — but invisible to
  // camera rays, or a top-down view is looking at the outside of a closed box.
  // It still occludes and still bounces; only primary visibility skips it.
  s.box(v3(0, WALL_H + 0.05, 0), v3(HX, 0.05, HZ), mWhite, undefined, FLAG_NO_CAMERA);

  // One crate, off-centre: near enough to a wall that its shadow and the
  // wall's bounce onto it are the same event.
  {
    const c = v3(-1.8, 0.45, -0.6), h = v3(0.45, 0.45, 0.45);
    s.box(c, h, mCrate);
    collide(c, h);
  }

  // Deliberately small clutter. The radiosity patch grid floors at 0.4 m, so a
  // face under ~0.6 m across gets gridW = 1 and lights up as one flat value —
  // that population is where the reported blotching is worst, and a room of
  // large flat walls does not contain it. Sizes straddle the threshold on
  // purpose: 0.5 m and 0.3 m faces either side of 0.4 m.
  const clutter: Array<[number, number, number, number]> = [
    [1.4, -1.9, 0.25, 0.55],
    [2.1, -1.2, 0.15, 0.32],
    [0.6, -2.4, 0.18, 0.9],
    [-0.4, 1.7, 0.22, 0.45],
    [1.9, 1.4, 0.14, 0.7],
  ];
  for (const [x, z, half, height] of clutter) {
    const c = v3(x, height / 2, z), h = v3(half, height / 2, half);
    s.box(c, h, mCrate);
    collide(c, h);
  }

  // Overhead panel, off-centre.
  //
  // `intensity` is ~9x `radiance`, which is the level's own convention for a
  // fixture meant to light a room rather than just look bright (see
  // `fluorescent` in level.ts) — the two are deliberately independent, and
  // getting the ratio backwards gives a blazing emitter that casts almost
  // nothing. Off-centre because a light at the middle of a symmetric box
  // bounces symmetrically, and a flat wash is the one thing that cannot show
  // whether indirect is working.
  s.areaLight(
    v3(1.1, WALL_H - 0.08, -0.7), v3(0.5, 0.03, 0.5),
    v3(1.0, 0.97, 0.92), 3.0, 27,
  );

  return { colliders: new Float32Array(colliders), spawn: v3(0, 0, 2.6), bounds };
}

// ---------------------------------------------------------------------------
// Synthetic scenes for narrowing down a transport bug.
//
// A ladder, each rung changing exactly one thing about how light enters the
// room. Cascades measures 84% of reference in the colour-bleed box above and
// 12% in the office with only its static practicals lit; these exist to find
// which difference between those two scenes is responsible.
// ---------------------------------------------------------------------------

/** Neutral shell with a chosen albedo, sized to sit inside the fluid lattice. */
function shell(
  s: SceneBuilder, half: number, albedo: number, emissive: number,
): { mat: number; colliders: number[]; bounds: LevelInfo["bounds"] } {
  const a = v3(albedo, albedo, albedo);
  const mat = emissive > 0
    ? s.material(a, 0.9, 0, v3(emissive, emissive, emissive))
    : s.material(a, 0.9, 0);
  const colliders: number[] = [];
  const t = 0.1;
  // Floor, ceiling and four walls, all the same material.
  s.box(v3(0, -t, 0), v3(half, t, half), mat);
  s.box(v3(0, WALL_H + t, 0), v3(half, t, half), mat, undefined, FLAG_NO_CAMERA);
  for (const [cx, cz, hx, hz] of [
    [0, -half - t, half + t * 2, t], [0, half + t, half + t * 2, t],
    [-half - t, 0, t, half], [half + t, 0, t, half],
  ] as const) {
    s.box(v3(cx, WALL_H / 2, cz), v3(hx, WALL_H / 2, hz), mat);
    colliders.push(cx - hx, cz - hz, cx + hx, cz + hz);
  }
  return { mat, colliders, bounds: { minX: -half, minZ: -half, maxX: half, maxZ: half } };
}

/**
 * Furnace test: every surface emits the same radiance and has the same albedo.
 *
 * The classic GI conservation check, and the reason it is first on the ladder:
 * a uniform environment has no small bright source to miss, so sampling
 * efficiency cannot be the variable. Equilibrium radiance is L/(1-rho)
 * exactly, so the indirect part is L*rho/(1-rho) — with L = 1 and rho = 0.5,
 * indirect radiance is exactly 1.0, everywhere, in every direction.
 *
 * Deliberately NO registered lights: emission is picked up only by rays
 * actually hitting a surface. That takes next-event estimation out of the
 * measurement entirely, so a failure here is transport (quadrature, merge,
 * multi-bounce) and a pass here moves all suspicion onto light sampling.
 */
export function buildFurnace(s: SceneBuilder): LevelInfo {
  const { colliders, bounds } = shell(s, 4, 0.5, 1.0);
  return { colliders: new Float32Array(colliders), spawn: v3(0, 0, 0), bounds };
}

/** One large registered emitter; same shell, so only the light changes. */
export function buildOnePanel(s: SceneBuilder): LevelInfo {
  const { colliders, bounds } = shell(s, 4, 0.5, 0);
  s.areaLight(v3(0, WALL_H - 0.08, 0), v3(2.0, 0.03, 2.0),
    v3(1, 1, 1), 3.0, 27);
  return { colliders: new Float32Array(colliders), spawn: v3(0, 0, 0), bounds };
}

/**
 * The same total intensity split across 16 small strips, the office's regime.
 * If cascades passes buildOnePanel and fails this, the variable is emitter
 * SIZE and COUNT rather than anything about the room.
 */
export function buildManyStrips(s: SceneBuilder): LevelInfo {
  const { colliders, bounds } = shell(s, 4, 0.5, 0);
  for (let i = 0; i < 16; i++) {
    const x = -3 + (i % 4) * 2;
    const z = -3 + Math.floor(i / 4) * 2;
    s.areaLight(v3(x, WALL_H - 0.14, z), v3(0.58, 0.02, 0.10),
      v3(1, 1, 1), 3.0, 27 / 16);
  }
  return { colliders: new Float32Array(colliders), spawn: v3(0, 0, 0), bounds };
}

export const SYNTHETIC: Record<string, (s: SceneBuilder) => LevelInfo> = {
  furnace: buildFurnace,
  onePanel: buildOnePanel,
  manyStrips: buildManyStrips,
  bleed: buildIndirectBox,
};

/**
 * `/demo/dynamics-rt` — the tuning page's lattice, lit for the path tracer.
 *
 * Sized so the CAMERA can get outside the lattice, which is not the same as
 * sized to contain it. /demo/dynamics simulates 4 x 3 x 4 m at 3.125 cm, and
 * the first cut of this room was 5.2 x 5.2 — the smallest that holds the box
 * with a margin. That was wrong in a way only a render shows: the orbit has to
 * clear the walls, so it clamped to 2.2 m of horizontal reach, and the lattice
 * half-diagonal is 2.83. The eye was INSIDE the cloud at every angle and the
 * frame was a flat brown murk.
 *
 * 9 x 9 costs nothing to fix it. The lattice is 4 x 3 x 4 whatever the room
 * is — room size is geometry, not cells — so the only price is a few more
 * square metres of wall for the GI to bounce off. Smoke that leaves the
 * lattice still vanishes at its open lateral faces, exactly as on the sibling
 * page.
 *
 * One warm key, not the grenade page's pair of fluorescents. Two overlapping
 * cool sources are the honest office case and they are deliberately
 * unflattering; that is the right call when tuning a room and the wrong one
 * when tuning a MEDIUM, because flat fill is exactly what hides the difference
 * between a cloud with internal structure and a lozenge. A single warm source
 * off to one side gives every puff a lit face, a shadowed face and a rim, and
 * those three are what the density grade is being judged on.
 */
export function buildDynamicsBox(s: SceneBuilder): LevelInfo {
  const mFloor = s.material(v3(0.13, 0.13, 0.14), 0.92, 0.0);
  const mWall = s.material(v3(0.56, 0.55, 0.53), 0.75, 0.0);
  const mMetal = s.material(v3(0.55, 0.56, 0.58), 0.28, 1.0);
  const mCrate = s.material(v3(0.5, 0.36, 0.2), 0.86, 0.0);

  const HX = 4.5, HZ = 4.5;
  const { colliders, bounds } = room(s, HX, HZ, mFloor, mWall);

  // Low and off to one side: smoke has to both wrap it and be occluded by it,
  // and a knee-high box is what tells you whether the cloud is pooling on the
  // floor or floating above it — which no amount of looking at a free-standing
  // plume will.
  {
    const c = v3(-1.6, 0.35, -1.2), h = v3(0.45, 0.35, 0.45);
    s.box(c, h, mCrate);
    footprint(colliders, c, h);
  }

  // Invisible to camera rays, so an overhead angle is not looking at the
  // outside of a lid. It still occludes and still bounces.
  s.box(v3(0, WALL_H + 0.05, 0), v3(HX, 0.05, HZ), mWall, undefined, FLAG_NO_CAMERA);

  // The key. Warm and high on one wall, angled across the box rather than
  // straight down: a source directly overhead lights the top of a cloud and
  // leaves its silhouette flat, which is the one view that cannot show shape.
  s.box(v3(2.9, WALL_H - 0.5, -1.4), v3(0.06, 0.26, 0.6), mMetal);
  s.areaLight(
    v3(2.76, WALL_H - 0.5, -1.4), v3(0.02, 0.24, 0.56),
    v3(1.0, 0.82, 0.62), 3.0, 44.0,
  );

  return { colliders: new Float32Array(colliders), spawn: v3(0, 0, 2), bounds };
}
