// Leaves the scene at a chosen moment of the temporal audit so run.py --shot
// captures it. window.__scenarioArg: { case: "walk"|"pop", mode, k, crouch,
// where, reference, view, width, height, walkFrames }.
(async () => {
  const arg = window.__scenarioArg || {};
  const W = arg.width || 384, H = arg.height || 240;
  const S = window.__settings, R = window.__renderer, P = window.__player, G = window.__guards;
  const bg = window.__benchGuard;
  bg.active = true; bg.aborted = false; bg.abortAt = performance.now() + 3.6e6;
  S.reference = !!arg.reference;
  S.debugTapMode = arg.mode || 0;
  S.debugView = arg.view || 0;
  window.__pinBenchPose();
  const home = { x: P.pos.x, z: P.pos.z };
  for (const g of G.guards) { g.pos.x = 120; g.pos.z = 120; }
  const resetHistory = () => { R.resize(W, H); R.resize(W, H + 1); R.resize(W, H); };
  // Where the character is, for cropping the page screenshot: internal-res
  // box plus the canvas geometry that maps it to page pixels.
  const geom = () => {
    const rs = window.__playerScreenRects();
    let x0 = 1e9, y0 = 1e9, x1 = -1e9, y1 = -1e9;
    for (const r of rs) { x0 = Math.min(x0, r.x0); y0 = Math.min(y0, r.y0); x1 = Math.max(x1, r.x1); y1 = Math.max(y1, r.y1); }
    const cv = document.querySelector("canvas");
    const cr = cv.getBoundingClientRect();
    return {
      box: { x0, y0, x1, y1 }, render: { w: R.renderWidth, h: R.renderHeight },
      canvas: { w: cv.width, h: cv.height, cssX: cr.x, cssY: cr.y, cssW: cr.width, cssH: cr.height },
    };
  };
  for (let i = 0; i < 30; i++) await window.__renderStill(1);

  if (arg.case === "walk") {
    P.crouching = !!arg.crouch;
    resetHistory();
    for (let i = 0; i < 45; i++) await window.__renderStill(1);
    await window.__renderMotion(arg.walkFrames || 30);
    return { case: "walk", mode: S.debugTapMode, character: await window.__historyStats({ cls: "dynamic" }), ...geom() };
  }
  // pop
  P.crouching = !!arg.crouch;
  if (arg.where === "contact") {
    let best = null;
    for (let i = 0; i < 16; i++) {
      const a = (i / 16) * Math.PI * 2;
      const dx = Math.sin(a), dz = Math.cos(a);
      for (let t = 0.05; t <= 6.0; t += 0.05) {
        if (P.blocked(home.x + dx * t, home.z + dz * t)) { if (!best || t < best.t) best = { t, dx, dz }; break; }
      }
    }
    if (best) { P.pos.x = home.x + best.dx * (best.t - 0.05); P.pos.z = home.z + best.dz * (best.t - 0.05); }
    for (let i = 0; i < 30; i++) await window.__renderStill(1);
  }
  resetHistory();
  window.__hidePlayer(true);
  for (let i = 0; i < 40; i++) await window.__renderStill(1);
  window.__hidePlayer(false);
  const k = arg.k || 4;
  for (let i = 0; i < k; i++) await window.__renderStill(1);
  return {
    case: "pop", mode: S.debugTapMode, k, crouch: !!arg.crouch,
    character: await window.__historyStats({ cls: "dynamic", age: k }),
    ...geom(),
  };
})()
