// Track B2b (fluid) — where the advection step loses mass.
//
//   npm run build
//   python3 tools/headless/run.py --scenario tools/headless/scenarios/fluid-leak.js --json out.json
//
// The mass sweep says a sinking cloud keeps ~0.43 of its mass over 8 s and
// that the Jacobi iteration count does not move that number, so an
// under-converged pressure solve is not the sink. This localises the sink
// instead: `advectionBalance` weighs the scalar field immediately before and
// after one semi-Lagrangian step, per y row, so a defect that lives in a
// single cell layer against the floor is visible as such. Sampled at several
// times as the blob descends: in free air first, then as it lands.
//
// Row 0 of this grid is the baked floor (always solid, always zero mass);
// row 1 is the lowest fluid layer.
(async () => {
  const DT_MS = 50;
  const AT = [0.5, 1.5, 2.5, 3.5, 5.0];

  window.__pause(true);
  window.__guards.frozen = true;
  window.__pinResolution(320, 200);
  const F = window.__fluid, SM = window.__smoke;
  const base = { ...F.tune };
  Object.assign(F.tune, base, { dissipation: 0, vorticity: 0 });

  F.reset();
  SM.reset(true);
  window.__canisters.reset();
  SM.silenced = false;
  SM.puff(5, 1.4, 0, 0.9, 25);
  await window.__renderStill(1, DT_MS);
  SM.silenced = true;

  const out = [];
  let at = 1;
  for (const t of AT) {
    const need = Math.round(t * 1000 / DT_MS) - at;
    if (need > 0) { await window.__renderStill(need, DT_MS); at += need; }
    const b = await F.advectionBalance();
    const d = await F.divergenceStats();
    out.push({
      t, steps: F.steps, jacobi: F.lastJacobi,
      before: +b.before.toFixed(5), after: +b.after.toFixed(5),
      relDefect: +b.relDefect.toFixed(6),
      rowBefore: b.rowBefore.map((v) => +v.toFixed(4)),
      rowDefect: b.rowDefect.map((v) => +v.toFixed(5)),
      // The projection's own residual, for the record: if the leak tracked it
      // the iteration sweep would have moved the retained fraction.
      activePostMean: d.activePostMean === null ? null : +d.activePostMean.toFixed(5),
      activeRelResidual: d.activeRelResidual === null ? null : +d.activeRelResidual.toFixed(5),
      activeCells: d.activeCells,
      activeVelRms: d.activeVelRms === null ? null : +d.activeVelRms.toFixed(4),
    });
  }
  Object.assign(F.tune, base);
  SM.silenced = false;
  // The advection step is not conservative, but it must stay bounded: a defect
  // over 2% per step is the regime the operator mismatch used to produce.
  const failures = out
    .filter((s) => !(Math.abs(s.relDefect) < 0.02))
    .map((s) => `t=${s.t}: advection defect ${s.relDefect} per step`);
  return { ok: failures.length === 0, failures, samples: out };
})()
