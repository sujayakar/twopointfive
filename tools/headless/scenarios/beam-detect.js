// Player standing in guard 3's beam (conference room, isolated): assert the
// suspicion accumulator climbs and the guard transitions to alert. Guards'
// feet are frozen so the geometry the assert measures does not drift.
(async () => {
  const R = window.__renderer;
  R.resize(384, 240);
  const G = window.__guards, D = window.__detection, P = window.__player;
  const idx = 0;
  const g = G.all[idx];
  G.frozen = true;
  // Do not let the guard finish the job here: this run measures the ramp
  // to alert; the fail state has its own scenario.
  D.tuning.fireReaction = 999;
  const dir = g.light.dir;
  const l = Math.hypot(dir.x, dir.z);
  P.pos.x = g.pos.x + (dir.x / l) * 4.0;
  P.pos.z = g.pos.z + (dir.z / l) * 4.0;
  P.velX = 0; P.velZ = 0; P.crouching = false;
  await window.__renderStill(2);
  const start = D.snapshot()[idx];
  const trace = [];
  for (let i = 0; i < 40; i++) {
    await window.__renderStill(1);
    const s = D.snapshot()[idx];
    trace.push({ i, state: s.state, susp: s.suspicion, los: s.hasLOS, beam: s.inBeam, signal: s.signal, torch: s.torch });
    if (s.state === "alert" && trace.length > 4) break;
  }
  await window.__renderStill(24);
  const final = D.snapshot()[idx];
  return {
    guardStart: start,
    playerPos: [P.pos.x, P.pos.z],
    lightLevel: window.__visibility.level,
    lux: window.__visibility.illuminance,
    trace,
    final,
    summary: D.summary(),
    hud: {
      detect: document.getElementById("detect")?.innerText,
      gauge: document.getElementById("gauge")?.innerText,
    },
  };
})()
