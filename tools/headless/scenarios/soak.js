// The full excursion cycle for one guard: seen -> alert -> player vanishes ->
// pursue last-known -> arrive -> search -> timeout -> path back -> patrol on
// its own route. searchTime is shortened so the loop fits a headless run.
(async () => {
  window.__pause(true);
  const R = window.__renderer;
  R.resize(256, 160);
  const G = window.__guards, D = window.__detection, P = window.__player;
  const DT = 0.05;
  D.tuning.fireReaction = 999;
  D.tuning.searchTime = 2.0;
  const idx = 2;
  const g = G.all[idx];
  // Put the player in the server-room guard's beam.
  const dir = g.light.dir; const l = Math.hypot(dir.x, dir.z);
  P.pos.x = g.pos.x + (dir.x / l) * 4.5;
  P.pos.z = g.pos.z + (dir.z / l) * 4.5;
  P.velX = 0; P.velZ = 0; P.crouching = false;
  const seenAt = [+P.pos.x.toFixed(2), +P.pos.z.toFixed(2)];
  let alerted = false;
  for (let i = 0; i < 80; i++) {
    await window.__renderStill(1, DT * 1000);
    if (D.snapshot()[idx].state === "alert") { alerted = true; break; }
  }
  // Vanish: to spawn on the far side of the building, in the dark.
  P.pos.x = -13; P.pos.z = 0.5;
  const timeline = [];
  let final = null;
  let maxSearchSweepStates = 0;
  for (let i = 0; i < 120; i++) {
    await window.__renderStill(5, DT * 1000);
    const s = D.snapshot()[idx];
    timeline.push([+((i + 1) * 0.25).toFixed(2), s.state, s.mode, s.pos, s.suspicion, s.hasLOS, s.sees]);
    if (s.state === "search") maxSearchSweepStates++;
    if (s.state === "patrol" && s.mode === "route" && i > 3) { final = s; break; }
  }
  const failures = [];
  if (!alerted) failures.push("guard never went alert with the player in its beam");
  if (!timeline.some((r) => r[1] === "alert" && r[2] === "nav")) failures.push("alert guard never pursued (mode nav)");
  if (!timeline.some((r) => r[1] === "search")) failures.push("guard never searched the last-known spot");
  if (!final) failures.push("guard never resumed its patrol route within 30 s");
  if (timeline.some((r) => r[6])) failures.push("guard reported eyes on a player who was at spawn in the dark");
  return {
    ok: failures.length === 0, failures,
    seenAt, alerted, seconds: timeline.length * 0.25, timeline, final,
    others: D.snapshot().map((s) => ({ i: s.index, state: s.state, mode: s.mode })),
  };
})()
