// Track B2b (fluid) — does the warm-started pressure solve drift?
//
//   npm run build
//   python3 tools/headless/run.py --scenario tools/headless/scenarios/fluid-pressure.js --json out.json
//
// The room is closed and every wall is Neumann, so the pressure Poisson system
// is singular: its solution is defined only up to an additive constant. `step`
// warm-starts each frame from the previous frame's pressure, which is free
// convergence when the right-hand side is compatible and an accumulator when it
// is not — and with fp16 velocities the discrete divergence does not sum to
// exactly zero. A null-space mode growing in the warm start is invisible in the
// gradient right up to the point where `p[c] - p[c-e]` loses its significant
// digits to a large common offset.
//
// So: hold a strong jet for a minute of game time and watch the pressure's mean
// against its spread, together with the residual it is supposed to be killing.
// A drift that matters shows up as |mean| / spread climbing while the residual
// gets worse. A drift that does not matter stays flat, and then this bundle
// retires the hypothesis instead of leaving it in the report as a worry.
(async () => {
  const DT_MS = 50;
  const SECONDS = 60;
  const EVERY = 10;              // seconds between checkpoints

  window.__pause(true);
  window.__guards.frozen = true;
  window.__pinResolution(320, 200);
  const F = window.__fluid, SM = window.__smoke;

  F.reset();
  SM.reset(true);
  window.__canisters.reset();
  window.__smoke.silenced = false;
  // A sustained 4 m/s jet down the corridor: the largest divergence the solver
  // is asked to remove in normal play, held for the whole run.
  const jet = () => SM.spawn({
    pos: { x: -6, y: 1.5, z: 0 }, radius: 0.6,
    vel: { x: 4, y: 0, z: 0 }, push: 40,
    density: 30, temp: 1.0, life: Infinity, permanent: true,
  });
  jet();

  const out = [];
  let stepped = 0;
  for (let t = 0; t <= SECONDS; t += EVERY) {
    const need = Math.round(t * 1000 / DT_MS) - stepped;
    if (need > 0) { await window.__renderStill(need, DT_MS); stepped += need; }
    const p = await F.pressureStats();
    const v = await F.divergenceStats();
    out.push({
      t, steps: F.steps,
      pMean: +p.mean.toFixed(6), pMin: +p.min.toFixed(6), pMax: +p.max.toFixed(6),
      pSpread: +p.spread.toFixed(6), pAbsMax: +p.absMax.toFixed(6),
      meanOverSpread: +p.meanOverSpread.toFixed(6),
      activeCells: v.activeCells,
      activePostMean: v.activePostMean === null ? null : +v.activePostMean.toFixed(6),
      activeRelResidual: v.activeRelResidual === null ? null : +v.activeRelResidual.toFixed(6),
      meanReduction: +v.meanReduction.toFixed(2),
    });
  }
  SM.reset(true);
  SM.silenced = false;

  const first = out[1], last = out[out.length - 1];   // t = 10 s vs t = 60 s
  const failures = [];
  for (const r of out) {
    if (!Number.isFinite(r.pMean) || !Number.isFinite(r.pSpread)) {
      failures.push(`t=${r.t}: non-finite pressure`);
    }
  }
  // The substantive assert: over 50 s of sustained forcing the residual the
  // projection leaves must not get worse. Anything else is the warm start
  // poisoning the solve.
  if (last.activeRelResidual !== null && first.activeRelResidual !== null
      && !(last.activeRelResidual < first.activeRelResidual * 3)) {
    failures.push(
      `residual degraded ${first.activeRelResidual} -> ${last.activeRelResidual}`);
  }
  return {
    ok: failures.length === 0, failures,
    dtMs: DT_MS, seconds: SECONDS, jacobi: F.lastJacobi, tune: { ...F.tune },
    fluidCells: (await F.pressureStats()).cells,
    drift: {
      meanAt10s: first.pMean, meanAt60s: last.pMean,
      spreadAt10s: first.pSpread, spreadAt60s: last.pSpread,
      meanOverSpreadAt10s: first.meanOverSpread, meanOverSpreadAt60s: last.meanOverSpread,
      residualAt10s: first.activeRelResidual, residualAt60s: last.activeRelResidual,
    },
    series: out,
  };
})()
