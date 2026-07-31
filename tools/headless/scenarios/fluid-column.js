// Track B2b (fluid) — a plume splits around a support column.
//
//   npm run build
//   python3 tools/headless/run.py --scenario tools/headless/scenarios/fluid-column.js \
//     --shot out.png --json out.json
//
// The corridor's support columns are at x = -18, -9, 0, 9, 18 and z = +/-3.9,
// 0.28 m half-extent, full height. A sustained jet is aimed down +x at the
// z = -3.9 column at x = 0 from 3 m upwind, and after 4 s the field is probed
// on the column's axis: inside it, immediately either side of it, and in its
// lee.
//
// The claim "smoke flows around the column, not through it" needs three
// numbers, not a picture: the column's cells must be the ones the occupancy
// bake calls solid (solidAtWorld), density inside them must be exactly zero,
// and density beside them must not be. The lee value says whether the two
// halves close up again behind it, which is what a real wake does.
(async () => {
  const DT_MS = 50;
  const RUN_S = 4.0;
  const CX = 0, CZ = -3.9, CY = 1.5;

  window.__pause(true);
  window.__guards.frozen = true;
  window.__renderer.resize(384, 240);
  const F = window.__fluid, SM = window.__smoke;
  F.reset();
  SM.reset(true);
  window.__canisters.reset();
  SM.silenced = false;
  // Held-on jet: the muzzle-burst regime, aimed at the column from upwind.
  SM.spawn({
    pos: { x: CX - 3, y: CY, z: CZ }, radius: 0.5,
    vel: { x: 3.5, y: 0, z: 0 }, push: 40,
    density: 40, temp: 3, life: Infinity,
  });
  await window.__renderStill(Math.round(RUN_S * 1000 / DT_MS), DT_MS);

  const P = (x, y, z) => ({
    at: [x, y, z], solid: F.solidAtWorld(x, y, z),
    density: +window.__sampleSmokeDensity(x, y, z).toFixed(4),
  });
  const probes = {
    upwind: P(CX - 1.0, CY, CZ),
    insideColumn: P(CX, CY, CZ),
    sideNear: P(CX, CY, CZ + 0.5),
    sideFar: P(CX, CY, CZ - 0.5),
    lee: P(CX + 0.6, CY, CZ),
    leeFar: P(CX + 1.5, CY, CZ),
  };
  const d = await F.densityStats();
  const failures = [];
  if (!probes.insideColumn.solid) {
    failures.push("the column's cell is not solid in the occupancy bake");
  }
  if (probes.insideColumn.density !== 0) {
    failures.push(`density ${probes.insideColumn.density} inside the column`);
  }
  if (probes.upwind.density <= 0) failures.push("the jet never reached the column");
  if (probes.sideNear.density <= 0 && probes.sideFar.density <= 0) {
    failures.push("no smoke either side of the column: it did not go around");
  }
  return {
    ok: failures.length === 0, failures,
    res: `${window.__renderer.renderWidth}x${window.__renderer.renderHeight}`,
    tSeconds: RUN_S, steps: F.steps, jacobi: F.lastJacobi,
    solidCells: F.solidCells, solidRow: F.solidRow,
    probes,
    cloud: {
      mass: +d.mass.toFixed(4), peak: +d.maxDensity.toFixed(4),
      visibleCells: d.visibleCells,
      bbox: d.bbox && {
        min: d.bbox.min.map((v) => +v.toFixed(2)),
        max: d.bbox.max.map((v) => +v.toFixed(2)),
      },
    },
  };
})()
