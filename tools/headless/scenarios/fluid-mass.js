// Track B2b (fluid) — density-mass conservation with all sources off.
//
//   npm run build
//   python3 tools/headless/scenarios/../run.py --scenario tools/headless/scenarios/fluid-mass.js --json out.json
//
// One instant blob (a single injection step) in open corridor air, then no
// emitter of any kind: the always-on wisps are dropped by __smoke.reset(true),
// canisters cleared, guards frozen (no muzzle bursts). The room is closed, so
// with dissipation zeroed any decline of the density integral ("mass") is the
// solver's own numerical leak; with defaults it is that leak plus the modeled
// exp(-dissipation t). Phases (edit PHASES):
//   defaults  — the shipped tuning: the honest lifetime the game sees.
//   noDiss    — dissipation 0, everything else default: the leak under motion.
//   liftOnly  — dissipation 0, weight 0, vorticity 0: buoyant rise only,
//               no pooling against the floor.
//   stillAir  — dissipation 0 and buoyancy/weight/vorticity 0: nothing moves;
//               isolates fp16 storage and boundary zeroing.
// Reports mass at t = 0,1,2,3,5,8 s (edit TIMES), the peak density, occupied
// cells, the mass centroid and the per-y-row mass at each checkpoint.
(async () => {
  const PHASES = ["sinkOnly", "swirlOnly", "noDissJ8", "noDissJ200"];
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
    liftOnly: { dissipation: 0, weight: 0, vorticity: 0 },
    stillAir: { dissipation: 0, buoyancy: 0, weight: 0, vorticity: 0 },
    // Motion isolators: which force channel carries the numerical leak.
    sinkOnly: { dissipation: 0, vorticity: 0 },
    swirlOnly: { dissipation: 0, weight: 0 },
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
      name, tune: { ...F.tune }, sources: SM.count, series,
      retained: m1 / m0,
      efoldSeconds: m1 > 0 && m0 > m1 ? t / Math.log(m0 / m1) : Infinity,
    };
  };

  const out = {};
  for (const name of PHASES) out[name] = await phase(name);
  Object.assign(F.tune, base);
  SM.silenced = false;
  return { phases: out };
})()
