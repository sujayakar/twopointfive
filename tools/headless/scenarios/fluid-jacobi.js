// Track B2b (fluid) — pressure-projection quality vs Jacobi iteration count.
//
//   npm run build
//   python3 tools/headless/run.py --scenario tools/headless/scenarios/fluid-jacobi.js --json out.json
//
// A still cloud has no divergence to remove, so the residual is only
// meaningful under real flow: each iteration count is measured on the SAME
// scripted flow — a sustained 4 m/s horizontal jet across the corridor (the
// muzzle-burst regime, held on) — from a zeroed field, stepped a fixed
// number of 50 ms game frames. Read at an early step (the transient, warm
// start barely primed) and a late one (quasi-steady). Per count: the solve's
// input (pre |div|) vs output (post |div|), the moving-cell mean residual
// relative to activeVelRms/cell, and how many cells are moving.
(async () => {
  const COUNTS = [4, 10, 20, 40, 80];
  const EARLY = 3, LATE = 24;

  window.__pause(true);
  window.__guards.frozen = true;
  window.__renderer.resize(320, 200);
  const S = window.__settings, F = window.__fluid, SM = window.__smoke;
  const pick = (d) => ({
    steps: F.steps,
    preMean: d.preMeanAbsDiv, postMean: d.meanAbsDiv,
    preMax: d.preMaxAbsDiv, postMax: d.maxAbsDiv,
    meanReduction: d.meanReduction,
    activeCells: d.activeCells, activeVelRms: d.activeVelRms,
    activePreMean: d.activePreMean, activePostMean: d.activePostMean,
    activeRelResidual: d.activeRelResidual,
  });
  const out = [];
  for (const jac of COUNTS) {
    F.reset();
    SM.reset(true);
    S.fluidJacobi = jac;
    SM.spawn({
      pos: { x: -12, y: 1.5, z: 0 }, radius: 0.5,
      vel: { x: 4, y: 0, z: 0 }, push: 40,
      density: 40, temp: 4, life: Infinity,
    });
    await window.__renderStill(EARLY, 50);
    const early = pick(await F.divergenceStats());
    await window.__renderStill(LATE - EARLY, 50);
    const late = pick(await F.divergenceStats());
    out.push({ jacobi: jac, early, late });
  }
  return { sweep: out };
})()
