// Track B2b (fluid) — mass drift with no sources + the physics self-test.
//
//   npm run build
//   python3 tools/headless/run.py --scenario tools/headless/scenarios/fluid-stats.js --json out.json
//
// Phase A: an instant blob in open corridor air, dissipation zeroed, every
// emitter silenced, all other tuning at defaults — the density integral is
// tracked over the run, so any decline is the solver's own numerical mass
// leak (semi-Lagrangian resample + fp16 + solid-cell zeroing), not the
// modeled dissipation. Phase B repeats it with buoyancy/weight/vorticity
// also zeroed (no self-driven motion) to separate the advection leak from
// the pooling-against-solids leak. Then physics.selfTest.
(async () => {
  const CHECK_EVERY = 40;
  const CHECKS = 3;              // 120 steps = 6.0 s of game time per phase

  window.__pause(true);
  window.__guards.frozen = true;
  window.__renderer.resize(320, 200);
  const F = window.__fluid, SM = window.__smoke;
  const base = { ...F.tune };

  const phase = async (tune) => {
    F.reset();
    SM.reset(true);
    Object.assign(F.tune, base, tune);
    SM.silenced = false;
    // Peak 25 at 1.4 m up, radius 0.9: clear of the floor, columns, crates.
    SM.puff(5, 1.4, 0, 0.9, 25);
    await window.__renderStill(1, 50);
    SM.silenced = true;
    const series = [];
    let s = await F.densityStats();
    series.push({ step: F.steps, mass: s.mass, peak: s.maxDensity, cells: s.nonzeroCells });
    for (let i = 0; i < CHECKS; i++) {
      await window.__renderStill(CHECK_EVERY, 50);
      s = await F.densityStats();
      series.push({ step: F.steps, mass: s.mass, peak: s.maxDensity, cells: s.nonzeroCells });
    }
    const m0 = series[0].mass, m1 = series[series.length - 1].mass;
    const t = (series[series.length - 1].step - series[0].step) * 0.05;
    return {
      tune: { ...F.tune }, series,
      efoldSeconds: m1 > 0 && m0 > m1 ? t / Math.log(m0 / m1) : Infinity,
      retained: m1 / m0,
    };
  };

  const still = await phase({ dissipation: 0, buoyancy: 0, weight: 0, vorticity: 0 });
  const moving = await phase({ dissipation: 0 });
  Object.assign(F.tune, base);
  SM.silenced = false;
  const st = window.__physicsSelfTest();
  return {
    stillAir: still, defaultsNoDissipation: moving,
    selfTest: { pass: st.pass, lines: st.lines },
  };
})()
