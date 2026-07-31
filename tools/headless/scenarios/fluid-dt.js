// Track B2b (fluid) — what order of error the residual mass defect is, and
// what a global mass renormalisation would and would not fix.
//
//   npm run build
//   python3 tools/headless/run.py --scenario tools/headless/scenarios/fluid-dt.js --json out.json
//
// After the operator fix, a sinking cloud still keeps only ~0.72 of its mass
// over 8 s with dissipation off. "Semi-Lagrangian gather advection is not
// conservative" explains the sign but not the size, and it does not say whether
// the residual is a truncation error (goes away as dt shrinks) or a leak at a
// boundary (does not). One measurement separates them: the same 8 s of game
// time at three step sizes.
//
// The gather maps each cell centre back along the flow, so its mass error per
// step is the departure of that map's Jacobian from 1. For a field the
// projection has already made divergence-free, the first-order term vanishes
// and what is left is O(dt^2) per step — which over a fixed physical time is
// O(dt) total, since the step count rises as 1/dt. So halving dt should halve
// the loss, and the fitted order should come out near 1. If instead the loss
// barely moves, the defect is not truncation and something is eating mass at a
// boundary.
//
// Second question, same data: a global renormalisation (scale the whole field
// each step so the integral matches its target) is the cheap fix the brief
// allowed as a last resort. What it would restore is computed here rather than
// argued: the factor it must apply, and what that factor does to the peak
// density — because the thing a viewer sees thinning is the peak, not the
// integral.
(async () => {
  const DT_MS = [50, 25, 12.5];
  const TIMES = [0, 1, 2, 3, 5, 8];
  const BALANCE_AT = 1.5;        // seconds: mid-descent, where the defect peaks

  window.__pause(true);
  window.__guards.frozen = true;
  window.__pinResolution(320, 200);
  const F = window.__fluid, SM = window.__smoke;
  const base = { ...F.tune };

  const phase = async (dtMs) => {
    F.reset();
    SM.reset(true);
    window.__canisters.reset();
    // The leaky configuration: weight and confinement on, dissipation off, so
    // every change in the integral is the scheme's own error.
    Object.assign(F.tune, base, { dissipation: 0 });
    SM.silenced = false;
    SM.puff(5, 1.4, 0, 0.9, 25);
    await window.__renderStill(1, dtMs);      // the injection step: t = 0
    SM.silenced = true;

    const series = [];
    let balance = null;
    let stepped = 0;
    const stepsTo = (t) => Math.round(t * 1000 / dtMs);
    const marks = [...TIMES];
    if (!marks.includes(BALANCE_AT)) marks.push(BALANCE_AT);
    marks.sort((a, b) => a - b);
    for (const t of marks) {
      const need = stepsTo(t) - stepped;
      if (need > 0) { await window.__renderStill(need, dtMs); stepped += need; }
      if (t === BALANCE_AT) {
        const b = await F.advectionBalance();
        balance = {
          t, before: +b.before.toFixed(5), after: +b.after.toFixed(5),
          relDefect: +b.relDefect.toFixed(7),
        };
      }
      if (!TIMES.includes(t)) continue;
      const s = await F.densityStats();
      series.push({
        t, steps: F.steps, mass: +s.mass.toFixed(5), peak: +s.maxDensity.toFixed(4),
        visibleCells: s.visibleCells, centroidY: +s.centroid[1].toFixed(3),
      });
    }
    const m0 = series[0].mass, mN = series[series.length - 1].mass;
    const p0 = series[0].peak, pN = series[series.length - 1].peak;
    // What a global renormaliser would have to do at the last checkpoint, and
    // what that does to the peak: it multiplies the field, so it restores the
    // integral exactly and the peak only in the same ratio.
    const k = mN > 0 ? m0 / mN : Infinity;
    return {
      dtMs, jacobi: F.lastJacobi, steps: F.steps, series, balance,
      retained: +(mN / m0).toFixed(5),
      peakRetained: +(pN / p0).toFixed(5),
      renorm: {
        factor: +k.toFixed(4),
        peakRetainedAfter: +(pN * k / p0).toFixed(5),
        peakGapBefore: +(1 - pN / p0).toFixed(5),
        peakGapAfter: +(1 - pN * k / p0).toFixed(5),
      },
    };
  };

  const out = [];
  for (const dt of DT_MS) out.push(await phase(dt));
  Object.assign(F.tune, base);
  SM.silenced = false;

  // Fitted order of the total loss in dt, and of the per-step defect in dt.
  const order = (a, b, va, vb) => +(Math.log(va / vb) / Math.log(a / b)).toFixed(3);
  const loss = out.map((r) => 1 - r.retained);
  const perStep = out.map((r) => Math.abs(r.balance.relDefect));
  const fits = {
    totalLossOrder: [
      order(out[0].dtMs, out[1].dtMs, loss[0], loss[1]),
      order(out[1].dtMs, out[2].dtMs, loss[1], loss[2]),
    ],
    perStepDefectOrder: [
      order(out[0].dtMs, out[1].dtMs, perStep[0], perStep[1]),
      order(out[1].dtMs, out[2].dtMs, perStep[1], perStep[2]),
    ],
  };

  const failures = [];
  // Every phase must have taken the step count its dt implies, or the sweep is
  // comparing different amounts of game time.
  for (const r of out) {
    const want = Math.round(TIMES[TIMES.length - 1] * 1000 / r.dtMs) + 1;
    if (r.steps !== want) failures.push(`dt ${r.dtMs}: ${r.steps} steps, expected ${want}`);
    if (!Number.isFinite(r.series[r.series.length - 1].mass)) {
      failures.push(`dt ${r.dtMs}: non-finite mass`);
    }
  }
  // The substantive assert: refining the step must recover mass. If it does not,
  // the residual is not a truncation error and this bundle's conclusion is void.
  if (!(out[out.length - 1].retained > out[0].retained + 0.02)) {
    failures.push(`refining dt did not recover mass: ${out.map((r) => r.retained)}`);
  }

  return { ok: failures.length === 0, failures, tune: { ...base }, fits, phases: out };
})()
