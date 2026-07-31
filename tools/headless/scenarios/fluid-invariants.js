// Track B2b (fluid) — the cheap invariant re-check.
//
//   npm run build
//   python3 tools/headless/run.py --scenario tools/headless/scenarios/fluid-invariants.js --json out.json
//
// Everything here is a machine-independent assert that costs a handful of
// frames, so it is the bundle to run when a change is believed inert and there
// is no budget to re-derive the transport tables. It covers, in order:
//
//   1. cold start — the same three asserts as smoke.js (probe, nav, guards), so
//      a WGSL compile error, a pipeline/bind-group validation error or a page
//      exception fails the run;
//   2. the occupancy bake — total solid cells and the per-row counts, which pin
//      that the ceiling slab is still the only box skipped as a lattice cap;
//   3. resources() — the bind-group tally, which is counted from the layout's
//      own entries rather than written down, and the storage-texture count that
//      has to stay inside SwiftShader's limit of 4;
//   4. divergenceStats() called TWICE at one checkpoint — the `pre` figures must
//      not move. They used to: the instrument re-ran the divergence kernel into
//      the same texture it read the solve's RHS from, so a second call reported
//      post-projection values as pre;
//   5. an instant source on a frame that cannot deliver it. Under a frozen
//      clock the renderer skips the fluid step (dt = 0), and while
//      __smoke.silenced is set the packer emits nothing. In both cases the puff
//      must survive to be delivered on the next frame that can take it, rather
//      than be consumed for nothing.
//
// Note the frozen-clock arm renders a settling frame after __freezeClock(true)
// and before the puff, and it has to. dt is `now - prev` clamped to 50 ms, and
// the FIRST frozen frame is fed `frozenClock` against a `prev` left by the
// previous frame — real time has advanced ~1 s per traced frame while the
// scenario's synthetic clock advanced 50 ms, so that difference is large and
// positive and the frame takes a full 50 ms step. Only from the second frozen
// frame is `prev === frozenClock` and dt exactly 0. Measured, not assumed: the
// first version of this bundle asserted dt = 0 on the first frozen frame and
// the run reported the puff being delivered on it.
(async () => {
  const DT_MS = 50;
  const failures = [];
  const eq = (what, got, want) => {
    if (got !== want) failures.push(`${what}: ${got}, expected ${want}`);
  };

  window.__pause(true);
  window.__guards.frozen = true;
  window.__pinResolution(320, 200);
  const F = window.__fluid, SM = window.__smoke;

  // ---- 1. cold start ------------------------------------------------------
  await window.__renderStill(3, DT_MS);
  const nav = window.__nav;
  let blocked = 0;
  for (let i = 0; i < nav.blocked.length; i++) blocked += nav.blocked[i];
  const probe = Array.from(window.__renderer.probeDebug).map((x) => +x.toFixed(3));
  const guards = window.__detection.snapshot();
  if (!(probe[0] > 0)) failures.push(`probe sees ${probe[0]} lights`);
  if (!(blocked > 0 && blocked < nav.blocked.length)) {
    failures.push(`nav raster degenerate: ${blocked}/${nav.blocked.length} blocked`);
  }
  eq("guards", guards.length, 4);

  // ---- 2. occupancy bake --------------------------------------------------
  // 19,420 with the ceiling slab skipped as a lattice cap. Baking the slab
  // instead would make row 12 entirely solid (29,952 cells in that row), so
  // this number is the load-bearing one for the top-row deviation.
  const bake = { solidCells: F.solidCells, solidRow: F.solidRow.slice() };
  eq("solidCells", bake.solidCells, 19420);
  eq("solidRow length", bake.solidRow.length, 13);
  eq("solidRow[12] (top row, ceiling skipped)", bake.solidRow[12], 1218);
  eq("solidRow sum", bake.solidRow.reduce((a, b) => a + b, 0), 19420);

  // ---- 3. resources ------------------------------------------------------
  const res = F.resources();
  eq("storageTexturesPerStage", res.storageTexturesPerStage, 3);
  eq("sampledTexture3d", res.bindings.sampledTexture3d, 4);
  eq("uniformBuffer", res.bindings.uniformBuffer, 1);
  eq("sampler", res.bindings.sampler, 1);
  eq("readOnlyStorageBuffer", res.bindings.readOnlyStorageBuffer, 1);
  eq("storageTexture3d", res.bindings.storageTexture3d, 3);
  if (res.bindings.storageBuffer || res.bindings.unknown) {
    failures.push(`unexpected binding kinds: ${JSON.stringify(res.bindings)}`);
  }

  // ---- 4. divergenceStats is idempotent on `pre` --------------------------
  F.reset();
  SM.reset(true);
  window.__canisters.reset();
  SM.silenced = false;
  SM.puff(5, 1.4, 0, 0.9, 25);
  await window.__renderStill(1, DT_MS);   // injection
  SM.silenced = true;
  await window.__renderStill(2, DT_MS);   // something to project
  const d1 = await F.divergenceStats();
  const d2 = await F.divergenceStats();
  const pre = (d) => [d.preMeanAbsDiv, d.preMaxAbsDiv];
  if (JSON.stringify(pre(d1)) !== JSON.stringify(pre(d2))) {
    failures.push(
      `divergenceStats pre moved between calls: ${JSON.stringify(pre(d1))} then ${JSON.stringify(pre(d2))}`,
    );
  }

  // ---- 5. an instant source is not spent on a frame that cannot deliver ---
  const massNow = async () => (await F.densityStats()).mass;
  // `settle` frames run after arming and before the puff — see the header note
  // on why the first frozen frame is not a dt = 0 frame.
  const undeliverable = async (label, arm, disarm, settle) => {
    F.reset();
    SM.reset(true);
    SM.silenced = false;
    arm();
    if (settle > 0) await window.__renderStill(settle, DT_MS);
    const beforePuff = await massNow();
    if (beforePuff !== 0) failures.push(`${label}: field not empty before the puff (${beforePuff})`);
    window.__smokePuff(5, 1.4, 0, 0.9, 25);
    await window.__renderStill(1, DT_MS);
    const held = await massNow();
    if (held !== 0) failures.push(`${label}: injected ${held} on a frame that cannot deliver`);
    disarm();
    await window.__renderStill(1, DT_MS);
    const delivered = await massNow();
    if (!(delivered > 0)) failures.push(`${label}: puff lost, mass ${delivered} after release`);
    return { held, delivered };
  };
  const frozen = await undeliverable(
    "frozen clock",
    () => window.__freezeClock(true),
    () => window.__freezeClock(false),
    1,
  );
  const silenced = await undeliverable(
    "silenced",
    () => { SM.silenced = true; },
    () => { SM.silenced = false; },
    0,
  );

  const stats = await F.densityStats();
  SM.silenced = false;
  return {
    ok: failures.length === 0, failures,
    coldStart: {
      probeDebug: probe,
      nav: { w: nav.w, h: nav.h, cells: nav.blocked.length, blocked },
      guards: guards.length,
    },
    bake,
    resources: {
      bindings: res.bindings,
      storageTexturesPerStage: res.storageTexturesPerStage,
      totalMB: res.totalMB,
    },
    divergence: { first: pre(d1), second: pre(d2), activeCells: d1.activeCells },
    instantSource: { frozen, silenced },
    field: {
      steps: F.steps,
      mass: +stats.mass.toFixed(6),
      peak: +stats.maxDensity.toFixed(6),
      checksum: stats.checksum,
      fieldChecksum: stats.fieldChecksum,
    },
  };
})()
