// The opening frame's promise: a player who touches nothing at the spawn is
// not compromised. Runs 75 s of game time — two laps of the corridor guard,
// whose westward leg looks straight down the corridor at the spawn — with
// the world live and the opening loadout, and records the worst detection
// reading and the meter along the way. Then a coda with the torch switched
// on at the spawn (a metre off the west wall): not an assert, a measurement
// of what your own beam costs there.
(async () => {
  window.__pause(true);
  const R = window.__renderer;
  R.resize(256, 160);
  const D = window.__detection, P = window.__player, V = window.__visibility;
  const DT = 0.05;
  window.__restart();
  P.velX = 0; P.velZ = 0;
  const SECONDS = 75;
  const trace = [];
  let worst = { level: 0, t: 0, guard: -1, dist: 0 };
  let worstLit = 0;
  for (let f = 1; f <= SECONDS / DT; f++) {
    await window.__renderStill(1, DT * 1000);
    const snap = D.snapshot();
    snap.forEach((s) => {
      if (s.suspicion > worst.level) worst = { level: s.suspicion, t: +(f * DT).toFixed(2), guard: s.index, dist: s.dist, state: s.state, sees: s.sees, signal: s.signal };
    });
    worstLit = Math.max(worstLit, V.level);
    if (f % (5 / DT) === 0) {
      const sum = D.summary();
      trace.push({
        t: +(f * DT).toFixed(1), level: +sum.level.toFixed(3), label: sum.label,
        lit: +V.level.toFixed(3),
        guards: snap.map((s) => ({ i: s.index, pos: s.pos, state: s.state, dist: s.dist, susp: s.suspicion, sees: s.sees })),
      });
    }
    if (P.dead) break;
  }
  const failures = [];
  if (P.dead) failures.push("an idle player at spawn was shot dead");
  if (worst.level >= D.tuning.suspiciousAt) {
    failures.push(`guard ${worst.guard} reached suspicion ${worst.level} at t=${worst.t}s (>= ${D.tuning.suspiciousAt}) on an idle player`);
  }
  if (D.snapshot().some((s) => s.state !== "patrol")) failures.push("a guard left patrol against an idle player");

  // ---- coda: the same spot with your own torch on --------------------------
  P.flashlightOn = true;
  let torchLit = 0, torchWorst = 0;
  for (let f = 1; f <= 20 / DT; f++) {
    await window.__renderStill(1, DT * 1000);
    torchLit = Math.max(torchLit, V.level);
    for (const s of D.snapshot()) torchWorst = Math.max(torchWorst, s.suspicion);
    if (P.dead) break;
  }
  return {
    ok: failures.length === 0, failures,
    seconds: SECONDS,
    spawn: [+P.pos.x.toFixed(2), +P.pos.z.toFixed(2)],
    worst: { ...worst, level: +worst.level.toFixed(4) },
    worstLightLevel: +worstLit.toFixed(3),
    everAlerted: D.everAlerted,
    torchOnCoda: { seconds: 20, worstLightLevel: +torchLit.toFixed(3), worstSuspicion: +torchWorst.toFixed(3), summary: D.summary() },
    trace,
  };
})()
