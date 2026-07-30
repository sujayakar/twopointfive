// A gunshot in the north-west corner: guards inside the 34 m broadcast radius
// go alert toward the sound; the server-room guard beyond it does nothing.
// Fired through the real trigger (mousedown), so the whole chain is exercised.
(async () => {
  const R = window.__renderer;
  R.resize(384, 240);
  const G = window.__guards, D = window.__detection, P = window.__player;
  D.tuning.fireReaction = 999;
  P.pos.x = -25.2; P.pos.z = -16.2; P.velX = 0; P.velZ = 0;
  await window.__renderStill(2);
  const shot = [P.pos.x, P.pos.z];
  const dists = G.all.map((g) => +Math.hypot(g.pos.x - shot[0], g.pos.z - shot[1]).toFixed(2));
  const before = D.snapshot().map((s) => ({ state: s.state, susp: s.suspicion }));
  const canvas = document.querySelector("canvas");
  canvas.dispatchEvent(new MouseEvent("mousedown", { button: 0, bubbles: true }));
  await window.__renderStill(1);
  window.dispatchEvent(new MouseEvent("mouseup", { button: 0, bubbles: true }));
  await window.__renderStill(3);
  const after = D.snapshot().map((s) => ({ state: s.state, susp: s.suspicion, stimulus: s.stimulus, mode: s.mode, pos: s.pos }));
  await window.__renderStill(25);
  const later = D.snapshot().map((s) => ({ state: s.state, mode: s.mode, pos: s.pos, stimulus: s.stimulus, hasLOS: s.hasLOS, dist: s.dist, susp: s.suspicion }));
  return { shot, gunshotRange: D.tuning.gunshotRange, dists, roundsLeft: P.rounds, before, after, later };
})()
