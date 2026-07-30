// Track B2b (fluid) — the smoke grenade sequence.
//
//   npm run build
//   python3 tools/headless/run.py --scenario tools/headless/scenarios/fluid-grenade.js \
//     --shot out.png --json out.json
//
// One still per run at T seconds of game time after the throw (edit T below;
// a JS scenario cannot take arguments). Deterministic game time: __pause +
// __renderStill(n, 50) steps exactly 50 ms per frame, so the sim advances
// T/0.05 steps whatever SwiftShader's wall clock says. Also returns the
// solver's numbers at that instant: mass (density integral over the room),
// peak density, occupied cells, the density checksum (the determinism
// check compares it across two runs) and the projection's divergence residual.
(async () => {
  const T = 2.0;

  const R = window.__renderer, P = window.__player, cam = window.__camera;
  window.__pause(true);
  R.resize(384, 240);
  // Cubicle-farm floor south of the moon pools, a corridor column ahead of
  // it: the player throws from the pool row toward the column at (-9, -3.9).
  P.pos.x = -3; P.pos.z = -12; P.yaw = 0.9;
  P.velX = 0; P.velZ = 0; P.flashlightOn = false; P.carrying = false;
  window.__equipment.select(3);
  cam.yaw = Math.PI * 0.25; cam.pitch = Math.PI * 0.33; cam.distance = 20;
  const cv = document.querySelector("canvas");
  for (let i = 0; i < 4; i++) cam.update(0.5, { x: P.pos.x, y: 1, z: P.pos.z + 4 }, cv.width / cv.height);
  // Latch the aim/muzzle transforms, then throw at the column base.
  await window.__renderStill(2, 50);
  window.__throwGrenade(-8, -6);
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
