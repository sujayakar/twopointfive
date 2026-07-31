// Track B2b (fluid) — the canister sequence, measured rather than eyeballed.
//
//   npm run build
//   python3 tools/headless/run.py --scenario tools/headless/scenarios/fluid-sequence.js \
//     --shot out.png --json out.json
//
// One throw, then the density field weighed at t = 0.5 / 2 / 5 / 10 s of game
// time: total mass, peak, the cloud's world bounding box and the occupied
// cells per y row (row 0 is the floor layer, row 12 the ceiling). Those four
// numbers per checkpoint are what "grows, curls, spreads along the floor,
// pools against the wall" has to mean if it is to be checkable — a still
// shows one camera's opinion, the box and the row histogram show the field.
// The screenshot is taken at the last checkpoint.
//
// Same deterministic protocol as fluid-canister.js: paused first, guards
// frozen, solver and every emitter reset, a fixed release point, fixed dt.
(async () => {
  // 1.0 brackets the emitter's onset: a canister only starts smoking once its
  // body sleeps, so t = 0.5 legitimately reads zero mass while it is bouncing.
  const AT = [0.5, 1, 2, 5, 10];
  const DT_MS = 50;

  const R = window.__renderer, P = window.__player, cam = window.__camera;
  window.__pause(true);
  window.__guards.frozen = true;
  R.resize(384, 240);
  const F = window.__fluid, SM = window.__smoke;
  F.reset();
  SM.reset(true);
  window.__canisters.reset();
  P.pos.x = -3; P.pos.z = -12; P.yaw = 0.9;
  P.velX = 0; P.velZ = 0; P.flashlightOn = false; P.carrying = false;
  window.__equipment.select(3);
  cam.yaw = Math.PI * 0.25; cam.pitch = Math.PI * 0.33; cam.distance = 20;
  const cv = document.querySelector("canvas");
  for (let i = 0; i < 4; i++) {
    cam.update(0.5, { x: P.pos.x, y: 1, z: P.pos.z + 4 }, cv.width / cv.height);
  }
  window.__throwCanister(-8, -6, [-2.8, 1.2, -11.6]);

  const out = [];
  let stepped = 0;
  for (const t of AT) {
    const need = Math.round(t * 1000 / DT_MS) - stepped;
    if (need > 0) { await window.__renderStill(need, DT_MS); stepped += need; }
    const d = await F.densityStats();
    const v = await F.divergenceStats();
    out.push({
      t, steps: F.steps, jacobi: F.lastJacobi,
      canisterEmitting: SM.count > 0,
      mass: +d.mass.toFixed(4), peak: +d.maxDensity.toFixed(4),
      nonzeroCells: d.nonzeroCells, visibleCells: d.visibleCells,
      centroid: d.centroid.map((x) => +x.toFixed(3)),
      bbox: d.bbox && {
        min: d.bbox.min.map((x) => +x.toFixed(2)),
        max: d.bbox.max.map((x) => +x.toFixed(2)),
        size: d.bbox.max.map((x, i) => +(x - d.bbox.min[i]).toFixed(2)),
      },
      rowCells: d.rowCells,
      rowMass: d.rowMass.map((x) => +x.toFixed(4)),
      // null when nothing in the room is moving faster than activeSpeed —
      // a settled cloud spreads by numerical diffusion, not by flow.
      activePostMean: v.activePostMean === null ? null : +v.activePostMean.toFixed(6),
      activeRelResidual: v.activeRelResidual === null ? null : +v.activeRelResidual.toFixed(6),
      activeSpeed: v.activeSpeed, activeCells: v.activeCells,
      meanReduction: +v.meanReduction.toFixed(2),
      checksum: d.checksum,
    });
  }
  const last = out[out.length - 1];
  const failures = [];
  if (!(last.mass > 0)) failures.push("no smoke at the last checkpoint");
  // The shape claim the bundle rests on: a heavy cloud ends up wider than it
  // is tall, and thickest at the floor.
  if (last.bbox && !(last.bbox.size[0] > last.bbox.size[1])) {
    failures.push(`cloud is not wider than tall: ${last.bbox.size}`);
  }
  if (last.rowCells[0] < last.rowCells[4]) {
    failures.push(`floor row thinner than row 4: ${last.rowCells.slice(0, 5)}`);
  }
  if (out.some((q) => !Number.isFinite(q.mass) || !Number.isFinite(q.peak))) {
    failures.push("non-finite density field");
  }
  return {
    ok: failures.length === 0, failures,
    res: `${R.renderWidth}x${R.renderHeight}`,
    dims: F.dims, cell: F.cell, solidCells: F.solidCells,
    canisters: window.__canisters.count, sources: SM.count,
    tune: { ...F.tune },
    resources: F.resources(),
    sequence: out,
  };
})()
