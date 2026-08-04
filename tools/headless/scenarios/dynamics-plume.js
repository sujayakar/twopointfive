// /demo/dynamics — how much does the plume actually obscure?
//
//   npm run build
//   python3 tools/headless/run.py --page /demo/dynamics.html \
//     --scenario tools/headless/scenarios/dynamics-plume.js --json out.json \
//     --arg '{"sweep":{"block":"plume","key":"expand","values":[0,20,32,90,245]}}'
//
// "Thicker" is not a density, it is a TRANSMITTANCE: what fraction of the light
// from whatever is behind the smoke still reaches the eye. That is
// exp(-absorption * density * path), so it is set by the product of three
// numbers and reading any one of them alone tells you nothing. This fires the
// plume, waits, and shoots a horizontal ray straight through the column at eye
// height — the sightline a guard would have — and reports the optical depth
// and the transmittance along it.
//
//   T > 0.5   you can see straight through
//   T ~ 0.1   a silhouette
//   T < 0.02  obscured
//
// `sweep` re-runs the whole thing per value of one parameter, which is the only
// honest way to answer "does this slider help" for a control that appears in
// three different places in the solver.
(async () => {
  const A = window.__scenarioArg || {};
  const D = window.__dyn;
  if (!D) return { ok: false, failures: ["__dyn missing — wrong page?"] };

  const T = A.t ?? 3.0;
  const DT_MS = 50;
  const KIND = A.kind ?? 0;
  const EYE = A.eye ?? 1.2;
  const failures = [];

  D.resize(160, 100);
  D.pause(true);
  D.params.dbg.openFaces = A.openFaces ?? 15;
  if (A.tune) Object.assign(D.params.solver, A.tune);
  for (const [block, vals] of Object.entries(A.params || {})) {
    if (!D.params[block]) { failures.push(`unknown param block ${block}`); continue; }
    Object.assign(D.params[block], vals);
  }

  const sweep = A.sweep || { block: "plume", key: "expand", values: [null] };
  const base = D.params[sweep.block] ? D.params[sweep.block][sweep.key] : null;

  // Named whole-configuration runs, for when the question is not "what does
  // this one slider do" but "which of these combinations obscures". Each entry
  // is { name, plume: {...}, solver: {...}, look: {...} }, applied over the
  // page defaults and reverted after.
  const CONFIGS = A.configs || null;

  const runOne = async () => {
    D.setKind(KIND);
    D.reset();
    D.fire();
    await D.step(Math.round(T * 1000 / DT_MS), DT_MS);
    const s = await D.fluid.densityStats();
    // Straight through the middle of the box at eye height. sigma is the
    // page's own absorption, so tau here is the tau the raymarch sees.
    const col = await D.fluid.columnDensity(
      [-2.0, EYE, 0], [1, 0, 0], 4.0, 0.05, D.params.look.absorption);
    return {
      mass: +s.mass.toFixed(2),
      peakDensity: +s.maxDensity.toFixed(1),
      visibleCells: s.visibleCells,
      bbox: s.bbox && {
        min: s.bbox.min.map((v) => +v.toFixed(2)),
        max: s.bbox.max.map((v) => +v.toFixed(2)),
      },
      // The answer.
      columnPeak: +col.peak.toFixed(1),
      tau: +col.tau.toFixed(2),
      transmittance: +Math.exp(-col.tau).toFixed(4),
    };
  };

  const results = [];
  if (CONFIGS) {
    const snapshot = {};
    for (const c of CONFIGS) {
      for (const block of Object.keys(c)) {
        if (block === "name" || !D.params[block]) continue;
        snapshot[block] = snapshot[block] || { ...D.params[block] };
      }
    }
    for (const c of CONFIGS) {
      // Every run starts from the same defaults, so configs cannot inherit
      // each other's overrides through the order they happen to be listed.
      for (const [block, vals] of Object.entries(snapshot)) {
        Object.assign(D.params[block], vals);
      }
      for (const [block, vals] of Object.entries(c)) {
        if (block === "name") continue;
        if (!D.params[block]) { failures.push(`unknown block ${block}`); continue; }
        Object.assign(D.params[block], vals);
      }
      results.push({ name: c.name, ...(await runOne()) });
    }
    for (const [block, vals] of Object.entries(snapshot)) {
      Object.assign(D.params[block], vals);
    }
  } else {
    for (const v of sweep.values) {
      if (v !== null) D.params[sweep.block][sweep.key] = v;
      results.push({ [sweep.key]: v === null ? base : v, ...(await runOne()) });
    }
    if (base !== null && sweep.values[0] !== null) {
      D.params[sweep.block][sweep.key] = base;
    }
  }

  return {
    ok: failures.length === 0,
    failures,
    t: T, eye: EYE, kind: KIND,
    absorption: D.params.look.absorption,
    swept: CONFIGS ? "configs" : `${sweep.block}.${sweep.key}`,
    results,
  };
})()
