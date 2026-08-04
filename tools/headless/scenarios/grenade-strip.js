// Renders one detonation on /demo/grenades at fixed simulation beats and hands
// the frames back as PNG data URLs.
//
// This exists because the interactive page cannot be graded by an agent: a
// browser tab throttles animation frames when it is not focused, so driving it
// from outside yields a few frames per screenshot and never a whole effect.
// Here the rAF loop is stood down entirely and time is advanced by hand, so
// every beat lands on an exact simulation second regardless of how long
// SwiftShader spends tracing it.
//
// One run, one filmstrip. Parameters come in through __scenarioArg so a sweep
// is a shell loop rather than an edit.
//
//   {
//     "beats":  [0.05, 0.25, 0.6, 1.5, 3, 6],   // simulation seconds
//     "kind":   "bang" | "can",
//     "yaw":    0.6,                             // parked camera
//     "pitch":  0.34,
//     "width":  480, "height": 270,
//     "params": { "look": { "detail": 2.0 }, ... }   // deep-merged, optional
//   }
(async () => {
  const A = window.__scenarioArg || {};
  const beats = A.beats || [0.05, 0.25, 0.6, 1.5, 3.0, 6.0];
  const kind = A.kind || "bang";
  const W = A.width || 480;
  const H = A.height || 270;
  const failures = [];

  const g = window.__grenade;
  if (!g) return { ok: false, failures: ["__grenade missing — wrong page?"] };

  // Small frames: software tracing is ~seconds per frame almost regardless of
  // size, but the PNGs come back inside the result blob and full-size ones
  // would dwarf it.
  //
  // The canvas ELEMENT has to shrink too, not just the render targets.
  // toDataURL captures the backing store, so resizing only the targets leaves
  // the frame sitting in the corner of a 1920x1080 canvas surrounded by
  // whatever was in the rest of it — which is exactly as useless as it sounds.
  const canvas = document.querySelector("canvas");
  canvas.style.width = W + "px";
  canvas.style.height = H + "px";
  window.__demo.resize();
  window.__renderer.resize(W, H);

  // Live parameter overrides, deep-merged one level into each block.
  if (A.params) {
    for (const [block, vals] of Object.entries(A.params)) {
      const target = g.params[block];
      if (!target) { failures.push(`unknown param block ${block}`); continue; }
      Object.assign(target, vals);
    }
    g.apply();
  }

  // Stand the rAF loop down, park the camera, and start from an empty room.
  window.__pause(true);
  g.park(A.yaw ?? 0.6, A.pitch ?? 0.34, A.distance ?? 4.6);
  g.reset();

  // A few frames before firing: the denoiser needs history for the room, or
  // the first beat is graded against a half-converged image and every later
  // one looks better than it is for reasons that have nothing to do with smoke.
  await window.__renderStill(8, 50);

  if (kind === "bang") g.bang(); else g.can();

  // 50 ms is the game's own dt cap, so a beat of N seconds is 20N frames. Beats
  // are cumulative from the detonation.
  const DT = 0.05;
  const shots = [];
  let simTime = 0;
  for (const b of beats) {
    const need = Math.max(0, Math.round((b - simTime) / DT));
    if (need > 0) await window.__renderStill(need, DT * 1000);
    simTime += need * DT;
    shots.push({ t: +simTime.toFixed(2), png: canvas.toDataURL("image/png") });
  }

  // Numbers alongside the pictures. A filmstrip says whether it looks right;
  // these say whether two runs differ at all, and by how much — which is the
  // difference between "I think that one is better" and a result.
  //
  // visibleCells is the one that tracks apparent size, mass the total medium,
  // and maxDensity whether a core survived or the whole thing thinned out.
  const ds = await window.__fluid.densityStats();
  const stats = {
    mass: +ds.mass.toFixed(3),
    maxDensity: +ds.maxDensity.toFixed(3),
    visibleCells: ds.visibleCells,
    nonzeroCells: ds.nonzeroCells,
    centroid: ds.centroid.map((v) => +v.toFixed(2)),
    bbox: ds.bbox,
  };

  return {
    ok: failures.length === 0,
    failures,
    kind,
    res: [W, H],
    beats: shots.map((s) => s.t),
    frames: shots.map((s) => s.png),
    smokeCells: window.__fluid.dims,
    smokeCell: window.__fluid.cell,
    stats,
  };
})()
