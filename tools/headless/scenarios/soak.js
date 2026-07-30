// The full excursion cycle for one guard: seen -> alert -> player vanishes ->
// pursue last-known -> arrive -> search -> timeout -> path back -> patrol on
// its own route. searchTime is shortened so the loop fits a headless run.
(async () => {
  const R = window.__renderer;
  R.resize(384, 240);
  const G = window.__guards, D = window.__detection, P = window.__player;
  D.tuning.fireReaction = 999;
  D.tuning.searchTime = 2.0;
  const idx = 2;
  const g = G.all[idx];
  // Put the player in the server-room guard's beam.
  const dir = g.light.dir; const l = Math.hypot(dir.x, dir.z);
  P.pos.x = g.pos.x + (dir.x / l) * 4.5;
  P.pos.z = g.pos.z + (dir.z / l) * 4.5;
  P.velX = 0; P.velZ = 0; P.crouching = false;
  const seenAt = [P.pos.x, P.pos.z];
  let alerted = false;
  for (let i = 0; i < 60; i++) {
    await window.__renderStill(1);
    if (D.snapshot()[idx].state === "alert") { alerted = true; break; }
  }
  // Vanish: back to spawn on the far side of the building.
  P.pos.x = -13; P.pos.z = 0.5;
  const timeline = [];
  const t0 = window.__stats.frames;
  let final = null;
  for (let i = 0; i < 90; i++) {
    await window.__renderStill(5);
    const s = D.snapshot()[idx];
    timeline.push([window.__stats.frames - t0, s.state, s.mode, s.pos, s.suspicion, s.hasLOS]);
    if (s.state === "patrol" && s.mode === "route" && i > 3) { final = s; break; }
  }
  const routeStart = [9.2, 6.1];
  return {
    seenAt, alerted, timeline, final,
    others: D.snapshot().map((s) => ({ i: s.index, state: s.state, mode: s.mode })),
  };
})()
