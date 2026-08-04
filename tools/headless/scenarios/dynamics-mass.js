// /demo/dynamics — where does the density integral go when nothing is emitting?
//
//   npm run build
//   python3 tools/headless/run.py --page /demo/dynamics.html \
//     --scenario tools/headless/scenarios/dynamics-mass.js --json out.json \
//     --arg '{"openFaces":15}'
//
// One burst, then silence: `smoke.silenced` stops every emitter packing, and
// dissipation is zeroed, so after `warm` the solver has no way to change the
// density integral except its own arithmetic.
//
// Three figures answer three different questions:
//   mass / peak    did it create smoke, and is it running away
//   shellMass      WHERE — banded by distance from the nearest lateral face,
//                  band 0 the outermost ring. A bulk error moves every band
//                  together; a boundary error moves band 0 alone.
//   divergence     the projection residual, which is what the volume
//                  correction in advectSclMac exponentiates every step.
//
//   {
//     "openFaces": 15,           // 0 closes the four side walls
//     "warm": 0.4,               // seconds of burst before silencing
//     "beats": [0.5, 1, 2, ...], // simulation seconds, absolute
//     "tune": { "vorticity": 0 } // merged into params.solver
//   }
(async () => {
  const A = window.__scenarioArg || {};
  const D = window.__dyn;
  if (!D) return { ok: false, failures: ["__dyn missing — wrong page?"] };

  const BEATS = A.beats || [0.5, 0.75, 1.0, 1.5, 2.0, 3.0, 4.0];
  const DT_MS = 50;
  const F = D.fluid;
  const failures = [];

  // The raymarch costs real time under SwiftShader and none of it is being
  // graded here, so the frame is postage-stamp sized. The fluid step is
  // resolution-independent, which is the whole reason this is measurable.
  D.resize(160, 100);
  D.pause(true);
  D.setKind(1);
  D.params.dbg.openFaces = A.openFaces ?? 15;
  // Dissipation models smoke thinning; it is not solver error and it would
  // mask solver error at exp(-0.22 t) — a third of the mass over five seconds.
  Object.assign(D.params.solver, { dissipation: 0, cooling: 0 }, A.tune || {});
  // Any other block, one level deep: `{"burst":{"height":1.5}}` lifts the
  // detonation off the floor, which is how "is the wall the mass source?"
  // gets asked without editing the page.
  for (const [block, vals] of Object.entries(A.params || {})) {
    if (!D.params[block]) { failures.push(`unknown param block ${block}`); continue; }
    Object.assign(D.params[block], vals);
  }

  const snap = async (t) => {
    const s = await F.densityStats();
    const dv = await F.divergenceStats();
    return {
      t: +t.toFixed(2),
      mass: +s.mass.toFixed(4),
      peak: +s.maxDensity.toFixed(1),
      cells: s.nonzeroCells,
      liveSources: D.smoke.count,
      // Bands, outermost first. Trailing bands are the interior.
      shell: s.shellMass.map((v) => +v.toFixed(4)),
      shellPeak: s.shellPeak.map((v) => +v.toFixed(1)),
      // What advectSclMac feeds to exp(-div*dt). dt is 0.05, and the
      // correction saturates at |div*dt| = MAX_DIV_DT = 2, i.e. |div| = 40.
      divMaxAbs: +dv.maxAbsDiv.toFixed(2),
      divMeanAbs: +dv.meanAbsDiv.toFixed(4),
      divPreMaxAbs: +dv.preMaxAbsDiv.toFixed(2),
      reduction: +dv.meanReduction.toFixed(2),
      velRms: dv.activeVelRms === null ? null : +dv.activeVelRms.toFixed(3),
      activeCells: dv.activeCells,
    };
  };

  D.reset();
  D.fire();
  const WARM = A.warm ?? 0.4;
  await D.step(Math.round(WARM * 1000 / DT_MS), DT_MS);
  // Every emitter goes quiet here. Sources keep ageing, they just pack
  // nothing — so from this point the solver is a closed system.
  D.smoke.silenced = true;
  const series = [await snap(WARM)];
  let t = WARM;
  for (const b of BEATS) {
    const need = Math.round((b - t) / (DT_MS / 1000));
    if (need <= 0) continue;
    await D.step(need, DT_MS);
    t += need * DT_MS / 1000;
    series.push(await snap(t));
  }
  D.smoke.silenced = false;

  const first = series[0], last = series[series.length - 1];
  const finite = series.filter((s) => Number.isFinite(s.mass) && s.mass > 0);
  const blewUp = series.find((s) => !Number.isFinite(s.mass) || s.peak > 60000);
  return {
    ok: failures.length === 0,
    failures,
    openFaces: D.params.dbg.openFaces,
    tune: { ...F.tune },
    series,
    // >1 means the solver made smoke out of nothing while nothing was emitting.
    coastGrowth: first.mass > 0 ? +(last.mass / first.mass).toFixed(3) : null,
    // Same ratio per band. If this is a boundary bug, band 0 diverges from the
    // rest; if it is a bulk bug, they move together.
    coastGrowthByShell: first.shell.map((m, i) =>
      (m > 1e-6 ? +(last.shell[i] / m).toFixed(3) : null)),
    // The last checkpoint that was still a number, and when it stopped being one.
    lastFinite: finite.length ? finite[finite.length - 1] : null,
    blewUpAt: blewUp ? blewUp.t : null,
  };
})()
