// Track B2b (fluid) — solver determinism check.
//
//   npm run build
//   python3 tools/headless/run.py --scenario tools/headless/scenarios/fluid-determinism.js --json out.json
//
// Everything that could vary run-to-run is pinned before the first sim step:
// the render loop is paused (only __renderStill steps the sim, at a fixed
// dt), the solver state and every emitter are reset (permanent ambient wisps
// included), and the only input is one instant density blob at a fixed
// point. The FNV-1a checksum of the raw fp16 density field after N steps must
// then be byte-identical across separate harness invocations. Run this
// twice and compare `checksum`.
(async () => {
  const DT_MS = 50;
  const STEPS = 40;

  window.__pause(true);
  window.__guards.frozen = true;
  window.__pinResolution(320, 200);
  const F = window.__fluid, SM = window.__smoke;

  F.reset();
  SM.reset(true);          // drops the permanent wisps as well
  window.__canisters.reset();
  SM.silenced = false;
  SM.puff(5, 1.4, 0, 0.9, 25);
  await window.__renderStill(1, DT_MS);   // the injection step
  SM.silenced = true;                     // no further inflow of any kind
  await window.__renderStill(STEPS - 1, DT_MS);

  const s = await F.densityStats();
  const failures = [];
  // The protocol, not the hash: a run that silently emitted nothing, or kept
  // an emitter alive, would produce a stable checksum that means nothing.
  if (F.steps !== STEPS) failures.push(`${F.steps} steps, expected ${STEPS}`);
  if (SM.count !== 0) failures.push(`${SM.count} emitters still packing`);
  if (!(s.mass > 0)) failures.push("no smoke in the field");
  if (!Number.isFinite(s.mass) || !Number.isFinite(s.maxDensity)) {
    failures.push("non-finite density field");
  }
  return {
    ok: failures.length === 0, failures,
    steps: F.steps,
    dtMs: DT_MS,
    sources: SM.count,
    checksum: s.checksum,
    mass: +s.mass.toFixed(6),
    maxDensity: +s.maxDensity.toFixed(6),
    nonzeroCells: s.nonzeroCells,
    centroid: s.centroid.map((v) => +v.toFixed(4)),
  };
})()
