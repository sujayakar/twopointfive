// Track B2b (fluid) — the smoke grenade sequence.
//
//   npm run build
//   python3 tools/headless/run.py --scenario tools/headless/scenarios/fluid-grenade.js \
//     --shot out.png --json out.json
//
// One still per run at T seconds of game time after the throw (edit T below;
// a JS scenario cannot take arguments). Deterministic protocol, so two runs
// with the same T return the same density checksum:
//   - __pause(true) FIRST: the rAF loop stands down, so the free-running
//     settle frames stop stepping the sim behind our back;
//   - guards frozen: no wandering, no muzzle bursts entering the medium;
//   - sim fields zeroed, EVERY source dropped (the level's always-on wisps
//     included) and canisters cleared — nothing the settle frames emitted
//     survives into the measurement;
//   - the throw releases from a fixed world point, not the animation-posed
//     weapon hand, and every step advances exactly 50 ms of game time.
// Returns the solver's numbers at that instant: mass (density integral over
// the room), peak density, occupied cells, the density checksum and the
// projection's divergence residual.
(async () => {
  const T = 2.0;

  const R = window.__renderer, P = window.__player, cam = window.__camera;
  window.__pause(true);
  window.__guards.frozen = true;
  R.resize(384, 240);
  window.__fluid.reset();
  window.__smoke.reset(true);
  window.__grenades.reset();
  // Cubicle-farm floor south of the moon pools, a corridor column ahead of
  // it: the player throws from the pool row toward the column at (-9, -3.9).
  P.pos.x = -3; P.pos.z = -12; P.yaw = 0.9;
  P.velX = 0; P.velZ = 0; P.flashlightOn = false; P.carrying = false;
  window.__equipment.select(3);
  cam.yaw = Math.PI * 0.25; cam.pitch = Math.PI * 0.33; cam.distance = 20;
  const cv = document.querySelector("canvas");
  for (let i = 0; i < 4; i++) cam.update(0.5, { x: P.pos.x, y: 1, z: P.pos.z + 4 }, cv.width / cv.height);
  // Fixed release point (chest height beside the player) at the column base.
  window.__throwGrenade(-8, -6, [-2.8, 1.2, -11.6]);
  const frames = Math.max(1, Math.round(T / 0.05));
  await window.__renderStill(frames, 50);
  const dens = await window.__fluid.densityStats();
  const div = await window.__fluid.divergenceStats();
  return {
    T, steps: window.__fluid.steps, res: `${R.renderWidth}x${R.renderHeight}`,
    scale: window.__fluid.scale, dims: window.__fluid.dims, cell: window.__fluid.cell,
    solidCells: window.__fluid.solidCells,
    grenades: window.__grenades.count, sources: window.__smoke.count,
    density: dens, divergence: div,
  };
})()
