// /demo/dynamics — one burst, parked camera, frames back as PNG data URLs.
//
//   npm run build
//   python3 tools/headless/run.py --page /demo/dynamics.html \
//     --scenario tools/headless/scenarios/dynamics-shot.js --json out.json \
//     --arg '{"beats":[0.15,0.4,0.8,1.5]}'
//
// The numeric scenarios say whether the field is conserved. This says whether
// it still looks like smoke — which is a separate question, and one that a
// change to the density scale can fail while every conservation figure
// improves. Orbit is switched off and the camera parked so two runs differ
// only by what is being tested.
//
//   { "beats": [...], "params": { "burst": { "density": 900 } },
//     "width": 480, "height": 300 }
(async () => {
  const A = window.__scenarioArg || {};
  const D = window.__dyn;
  if (!D) return { ok: false, failures: ["__dyn missing — wrong page?"] };

  const BEATS = A.beats || [0.15, 0.4, 0.8, 1.5];
  // 50 ms is the game's dt cap, but the page interactively runs at rAF — ~16 ms
  // — and short-lived things are quantised by it. A 40 ms muzzle flash covers
  // two or three real frames and none of a 50 ms one, so anything under about
  // a tenth of a second has to be graded at the step it will actually run at.
  const DT_MS = A.dt ?? 50;
  const W = A.width || 420;
  const H = A.height || 260;
  const failures = [];

  const canvas = document.querySelector("canvas");
  D.resize(W, H);
  D.pause(true);
  D.setKind(A.kind ?? 1);
  D.params.dbg.openFaces = A.openFaces ?? 15;
  Object.assign(D.params.cam, { orbit: false, yaw: 0.6, pitch: 0.22, distance: 4.2 });
  if (A.tune) Object.assign(D.params.solver, A.tune);
  for (const [block, vals] of Object.entries(A.params || {})) {
    if (!D.params[block]) { failures.push(`unknown param block ${block}`); continue; }
    Object.assign(D.params[block], vals);
  }

  D.reset();
  // The canvas resize above only reaches the backing store once layout has
  // flushed and the ResizeObserver has run. Without this wait the FIRST beat
  // comes back at the window's size and every later one at the requested size
  // — two different framings in one filmstrip, which is worse than either.
  await new Promise((r) => requestAnimationFrame(() => requestAnimationFrame(r)));
  if (canvas.width !== Math.round(W * (devicePixelRatio || 1) * D.params.look.renderScale)) {
    // Not fatal — the frames are still comparable to each other — but it must
    // not pass silently, because the numbers are quoted against a resolution.
    failures.push(`canvas ${canvas.width}x${canvas.height}, asked for ${W}x${H}`);
  }
  D.fire();
  const shots = [];
  let t = 0;
  for (const b of BEATS) {
    const need = Math.round((b - t) / (DT_MS / 1000));
    if (need > 0) { await D.step(need, DT_MS); t += need * DT_MS / 1000; }
    const s = await D.fluid.densityStats();
    shots.push({
      t: +t.toFixed(2),
      mass: +s.mass.toFixed(3),
      peak: +s.maxDensity.toFixed(1),
      visibleCells: s.visibleCells,
      // Optical depth through a half-metre of the core at the page's
      // absorption — the figure that decides whether it reads as smoke at all.
      tauCore: +(s.maxDensity * D.params.look.absorption * 0.5).toFixed(2),
      png: canvas.toDataURL("image/png"),
    });
  }

  return {
    ok: failures.length === 0,
    failures,
    res: [W, H],
    burstDensity: D.params.burst.density,
    beats: shots.map((s) => s.t),
    stats: shots.map(({ png, ...rest }) => rest),
    frames: shots.map((s) => s.png),
  };
})()
