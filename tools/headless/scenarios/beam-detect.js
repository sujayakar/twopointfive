// Player standing 4 m down guard 0's beam: the accumulator must climb every
// perception tick and the guard must reach alert well inside a second of game
// time. Guards' feet are frozen so the geometry the assert measures does not
// drift; the clock is the fixed scenario step, not this machine.
(async () => {
  window.__pause(true);
  const R = window.__renderer;
  R.resize(384, 240);
  const G = window.__guards, D = window.__detection, P = window.__player;
  const DT = 0.05;
  const idx = 0;
  const g = G.all[idx];
  G.frozen = true;
  // This run measures the ramp to alert; the fail state has its own scenario.
  D.tuning.fireReaction = 999;
  const dir = g.light.dir;
  const l = Math.hypot(dir.x, dir.z);
  P.pos.x = g.pos.x + (dir.x / l) * 4.0;
  P.pos.z = g.pos.z + (dir.z / l) * 4.0;
  P.velX = 0; P.velZ = 0; P.crouching = false; P.flashlightOn = false;
  await window.__renderStill(4, DT * 1000);
  g.suspicion = 0; g.state = "patrol";
  const trace = [];
  let alertAt = null;
  for (let i = 0; i < 60; i++) {
    await window.__renderStill(1, DT * 1000);
    const s = D.snapshot()[idx];
    trace.push({ t: +((i + 1) * DT).toFixed(2), state: s.state, susp: s.suspicion, los: s.hasLOS, sees: s.sees, beam: s.inBeam, signal: s.signal });
    if (s.state === "alert" && alertAt === null) alertAt = (i + 1) * DT;
    if (alertAt !== null && (i + 1) * DT > alertAt + 0.6) break;
  }
  // Hold long enough for the beam tint and the edge pulse to settle — this is
  // also the frame the SEEN screenshot is taken of.
  await window.__renderStill(24, DT * 1000);
  const final = D.snapshot()[idx];
  const summary = D.summary();
  const failures = [];
  if (!final.hasLOS) failures.push("no LOS to a player standing in the beam");
  if (!final.inBeam) failures.push("player 4 m down the beam axis is not inBeam");
  if (!(final.signal >= D.tuning.beamRate * 0.4)) failures.push(`signal ${final.signal} too small in the beam`);
  if (alertAt === null) failures.push("guard never reached alert");
  else if (alertAt > 1.2) failures.push(`alert took ${alertAt}s of game time (>1.2)`);
  // Monotone rise until it pins at 1.
  for (let i = 1; i < trace.length; i++) {
    if (trace[i].susp < trace[i - 1].susp) { failures.push(`suspicion fell at t=${trace[i].t}`); break; }
  }
  if (summary.label !== "SEEN") failures.push(`HUD label ${summary.label}, expected SEEN`);
  return {
    ok: failures.length === 0, failures,
    playerPos: [+P.pos.x.toFixed(2), +P.pos.z.toFixed(2)],
    guardPos: [+g.pos.x.toFixed(2), +g.pos.z.toFixed(2)],
    lightLevel: +window.__visibility.level.toFixed(3),
    lux: +window.__visibility.illuminance.toFixed(4),
    alertAtSeconds: alertAt,
    trace,
    final,
    summary,
    hud: {
      detect: document.getElementById("detect")?.innerText.replace(/\n/g, " | "),
      gauge: document.getElementById("gauge")?.innerText.replace(/\n/g, " | "),
    },
  };
})()
