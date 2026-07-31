// Track B2b (fluid) — how long a cloud actually lasts, out to 20 s.
//
//   npm run build
//   python3 tools/headless/run.py --scenario tools/headless/scenarios/fluid-lifetime.js --json out.json
//
// fluid-mass measures 8 s, which is where the advection error lives; the
// lifetime claim in DEFAULT_FLUID_TUNING is about 20-30 s, so it needs its
// own horizon. Two phases from the same blob:
//   noDiss   — dissipation 0. Separates the one-off loss while the cloud is
//              in fast motion from the steady rate once it has settled: the
//              report quotes both, because a single e-folding number fitted
//              across the two is a fiction.
//   defaults — the shipped tuning, so the modeled exp(-0.13 t) can be
//              compared against what the field actually does.
(async () => {
  const TIMES = [0, 1, 2, 3, 5, 8, 12, 16, 20];
  const DT_MS = 50;

  window.__pause(true);
  window.__guards.frozen = true;
  window.__renderer.resize(320, 200);
  const F = window.__fluid, SM = window.__smoke;
  const base = { ...F.tune };

  const phase = async (tune) => {
    F.reset();
    SM.reset(true);
    window.__canisters.reset();
    Object.assign(F.tune, base, tune);
    SM.silenced = false;
    SM.puff(5, 1.4, 0, 0.9, 25);
    await window.__renderStill(1, DT_MS);
    SM.silenced = true;
    const series = [];
    let at = 1;
    for (const t of TIMES) {
      const need = Math.round(t * 1000 / DT_MS) + 1 - at;
      if (need > 0) { await window.__renderStill(need, DT_MS); at += need; }
      const s = await F.densityStats();
      series.push({
        t, mass: +s.mass.toFixed(5), peak: +s.maxDensity.toFixed(4),
        cells: s.nonzeroCells, visibleCells: s.visibleCells,
        centroidY: +s.centroid[1].toFixed(3),
      });
    }
    const m = (t) => series.find((s) => s.t === t).mass;
    return {
      tune: { ...F.tune }, dispatchedJacobi: F.lastJacobi, series,
      // The two regimes, reported separately.
      retainedTo3s: +(m(3) / m(0)).toFixed(4),
      retainedTo20s: +(m(20) / m(0)).toFixed(4),
      // Rate over the settled window only, 1/s.
      settledRatePerS: +(Math.log(m(8) / m(20)) / 12).toFixed(5),
      modeledRatePerS: F.tune.dissipation,
    };
  };

  const noDiss = await phase({ dissipation: 0 });
  const defaults = await phase({});
  Object.assign(F.tune, base);
  SM.silenced = false;
  const failures = [];
  for (const [name, p] of Object.entries({ noDiss, defaults })) {
    if (!p) continue;
    for (const s of p.series) {
      if (!Number.isFinite(s.mass) || s.mass < 0) {
        failures.push(`${name} t=${s.t}: mass ${s.mass}`);
      }
    }
    // With no modeled decay the field may drift either way but must not run
    // away: anything outside [0.5, 1.5] of the injected mass is a bug.
    if (name === "noDiss" && !(p.retainedTo20s > 0.5 && p.retainedTo20s < 1.5)) {
      failures.push(`noDiss retained ${p.retainedTo20s} at 20 s`);
    }
  }
  return { ok: failures.length === 0, failures, noDiss, defaults };
})()
