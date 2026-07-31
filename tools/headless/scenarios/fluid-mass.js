// Track B2b (fluid) — density-mass conservation with all sources off.
//
//   npm run build
//   python3 tools/headless/scenarios/../run.py --scenario tools/headless/scenarios/fluid-mass.js --json out.json
//
// One instant blob (a single injection step) in open corridor air, then no
// emitter of any kind: the always-on wisps are dropped by __smoke.reset(true),
// canisters cleared, guards frozen (no muzzle bursts). The room is closed, so
// with dissipation zeroed any change in the density integral ("mass") is the
// solver's own numerical error; with defaults it is that plus the modeled
// exp(-dissipation t). Phases (edit PHASES):
//   defaults    — the shipped tuning: the lifetime the game actually sees.
//   noDiss      — dissipation 0, everything else default: the error under motion.
//   stillAir    — dissipation 0 and buoyancy/weight/vorticity 0: nothing moves,
//                 so this isolates fp16 storage and boundary zeroing. Must be
//                 exactly 1.000 retained; anything else is a storage bug.
//   sinkOnly    — noDiss with vorticity 0: the pair brackets what the
//                 confinement costs in conservation.
//   noDissJ8    — noDiss at 8 Jacobi iterations.
//   noDissJ200  — noDiss at 200. If residual divergence were the sink these
//                 two would differ; the run asserts both counts reached the
//                 dispatch loop, so "they agree" cannot mean "the knob is dead".
// Reports mass at t = 0,1,2,3,5,8 s (edit TIMES), the peak density, occupied
// cells, the mass centroid and the per-y-row mass at each checkpoint.
(async () => {
  const PHASES = ["defaults", "noDiss", "stillAir", "sinkOnly", "noDissJ8", "noDissJ200"];
  const TIMES = [0, 1, 2, 3, 5, 8];
  const DT_MS = 50;

  window.__pause(true);
  window.__guards.frozen = true;
  window.__renderer.resize(320, 200);
  const F = window.__fluid, SM = window.__smoke;
  const base = { ...F.tune };
  const tunings = {
    defaults: {},
    noDiss: { dissipation: 0 },
    stillAir: { dissipation: 0, buoyancy: 0, weight: 0, vorticity: 0 },
    // Motion isolator: noDiss minus the confinement, so the pair brackets
    // what vorticity confinement costs in conservation. (A cold blob with
    // weight 0 has no force on it at all, so it is not a third isolator —
    // it is stillAir under another name.)
    sinkOnly: { dissipation: 0, vorticity: 0 },
    // Projection-quality probes: if residual divergence is the mass sink,
    // the leak must track the Jacobi iteration count.
    noDissJ8: { dissipation: 0, jacobi: 8 },
    noDissJ200: { dissipation: 0, jacobi: 200 },
  };

  const snap = async () => {
    const s = await F.densityStats();
    return {
      step: F.steps, t: +(F.steps * DT_MS / 1000 - DT_MS / 1000).toFixed(3),
      mass: s.mass, peak: s.maxDensity, cells: s.nonzeroCells,
      centroid: s.centroid.map((v) => +v.toFixed(3)),
      rowMass: s.rowMass.map((v) => +v.toFixed(4)),
      checksum: s.checksum,
    };
  };

  const phase = async (name) => {
    F.reset();
    SM.reset(true);
    window.__canisters.reset();
    Object.assign(F.tune, base, tunings[name]);
    SM.silenced = false;
    // Peak 25 at 1.4 m up, radius 0.9, at (5, 1.4, 0): open corridor air,
    // clear of the floor, columns and crates.
    SM.puff(5, 1.4, 0, 0.9, 25);
    await window.__renderStill(1, DT_MS);   // the injection step: t = 0
    SM.silenced = true;
    const series = [await snap()];
    for (let i = 1; i < TIMES.length; i++) {
      const stepsTo = Math.round(TIMES[i] * 1000 / DT_MS);
      const need = stepsTo - Math.round(TIMES[i - 1] * 1000 / DT_MS);
      await window.__renderStill(need, DT_MS);
      series.push(await snap());
    }
    const m0 = series[0].mass, m1 = series[series.length - 1].mass;
    const t = TIMES[TIMES.length - 1] - TIMES[0];
    return {
      name, tune: { ...F.tune }, dispatchedJacobi: F.lastJacobi,
      sources: SM.count, series,
      retained: m1 / m0,
      efoldSeconds: m1 > 0 && m0 > m1 ? t / Math.log(m0 / m1) : Infinity,
    };
  };

  const out = {};
  const failures = [];
  for (const name of PHASES) {
    const r = await phase(name);
    // Every phase's tuning must have reached the dispatch loop, or two phases
    // that differ only in `jacobi` report the same numbers and read as proof
    // that projection quality is irrelevant.
    const want = Math.max(2, Math.round(r.tune.jacobi / 2) * 2);
    if (r.dispatchedJacobi !== want) {
      failures.push(`${name}: jacobi ${want} requested, ${r.dispatchedJacobi} dispatched`);
    }
    out[name] = r;
  }
  Object.assign(F.tune, base);
  SM.silenced = false;
  return { ok: failures.length === 0, failures, phases: out };
})()
