// Full loop: seen, shot at, hit, COMPROMISED — then R-restart puts the level
// back to its opening state, including a lamp shot out during the run and the
// OCP charge (both are level state; a second attempt in a darker map is a
// different level, not a restart).
(async () => {
  window.__pause(true);
  const R = window.__renderer;
  R.resize(384, 240);
  const G = window.__guards, D = window.__detection, P = window.__player;
  const S = window.__scene, E = window.__equipment, C = window.__camera, V = window.__visibility;
  const DT = 0.05;
  const step = async (n) => { for (let i = 0; i < n; i++) await window.__renderStill(1, DT * 1000); };
  const failures = [];
  // Feet pinned throughout: the assert wants one variable at a time, and no
  // patrolling guard should wander into the lamp test.
  G.frozen = true;

  // ---- shoot a lamp out and prove it went dark ----------------------------
  // Under a south-storage fluorescent: bright, and off every guard's cone.
  const lamp = { x: 0, z: 9.5 };
  P.pos.x = lamp.x; P.pos.z = lamp.z + 1.2; P.velX = 0; P.velZ = 0; P.flashlightOn = false;
  await step(12);
  const litBefore = +V.level.toFixed(3);
  // Nearest static light overhead, aimed at along the camera ray like a click.
  let best = -1, bestD = Infinity;
  for (let i = 1; i < S.lights.length; i++) {
    const lg = S.lights[i];
    if (lg.kind === 1 || lg.intensity <= 0) continue;
    const d = Math.hypot(lg.pos.x - lamp.x, lg.pos.z - lamp.z);
    if (d < bestD && lg.pos.y > 2) { bestD = d; best = i; }
  }
  const target = S.lights[best];
  const m = P.muzzle();
  const dv = { x: target.pos.x - C.pos.x, y: target.pos.y - C.pos.y, z: target.pos.z - C.pos.z };
  const dl = Math.hypot(dv.x, dv.y, dv.z);
  const shot = E.shootOut(S.lights, S.lights.length, m.pos, C.pos, { x: dv.x / dl, y: dv.y / dl, z: dv.z / dl });
  if (!shot) failures.push("shootOut found no lamp on the aimed ray");
  else {
    // The same two renderer calls main.ts makes on a fixture kill.
    R.setStaticLightIntensity(shot.index, 0);
    if (shot.mat >= 0) R.setMaterialEmissive(shot.mat, 0, 0, 0);
    if (shot.index !== best) failures.push(`shot light ${shot.index}, aimed at ${best}`);
  }
  E.ocpCharge = 0.3;
  await step(12);
  const litAfterShot = +V.level.toFixed(3);
  if (!(litAfterShot < litBefore - 0.1)) failures.push(`lamp ${best} shot but meter ${litBefore} -> ${litAfterShot}`);

  // ---- stand in guard 0's beam until dead --------------------------------
  for (const gg of G.all) { gg.suspicion = 0; gg.state = "patrol"; gg.stimulus = null; gg.mode = "route"; }
  const g = G.all[0];
  const dir = g.light.dir; const l = Math.hypot(dir.x, dir.z);
  P.pos.x = g.pos.x + (dir.x / l) * 5.0;
  P.pos.z = g.pos.z + (dir.z / l) * 5.0;
  P.velX = 0; P.velZ = 0; P.crouching = false;
  let deadAt = null, firstShotAt = null;
  for (let i = 0; i < 200; i++) {
    await step(1);
    if (g.justFired && firstShotAt === null) firstShotAt = (i + 1) * DT;
    if (P.dead) { deadAt = (i + 1) * DT; break; }
  }
  await step(6);
  const card = document.getElementById("endcard");
  const deadState = {
    playerDead: P.dead,
    secondsToFirstShot: firstShotAt,
    secondsToDeath: deadAt,
    guard: D.snapshot()[0],
    cardVisible: card ? card.classList.contains("show") : false,
    cardText: card ? card.innerText.replace(/\n/g, " | ") : null,
    litInBeam: +V.level.toFixed(3),
  };
  if (!P.dead) failures.push("player never died in the beam");
  if (deadAt !== null && deadAt < D.tuning.fireReaction) failures.push(`shot dead in ${deadAt}s, before the ${D.tuning.fireReaction}s reaction beat`);
  if (!deadState.cardVisible || !/COMPROMISED/.test(deadState.cardText ?? "")) failures.push(`fail card wrong: ${deadState.cardText}`);

  // ---- restart through the hook R calls, then verify the reset took --------
  window.__restart();
  G.frozen = false;
  await step(4);
  const s0 = D.snapshot();
  const afterRestart = {
    playerDead: P.dead,
    playerPos: [+P.pos.x.toFixed(2), +P.pos.z.toFixed(2)],
    rounds: P.rounds,
    spares: P.spares,
    ocpCharge: E.ocpCharge,
    lampCpuIntensity: S.lights[best].intensity,
    guards: s0.map((s) => ({ state: s.state, susp: s.suspicion, dead: s.dead, mode: s.mode })),
    cardVisible: card ? card.classList.contains("show") : false,
  };
  if (afterRestart.playerDead) failures.push("player still dead after restart");
  if (Math.hypot(afterRestart.playerPos[0] + 13, afterRestart.playerPos[1] - 0.5) > 0.6) failures.push(`respawned at ${afterRestart.playerPos}, not the spawn`);
  if (afterRestart.cardVisible) failures.push("fail card still up after restart");
  if (afterRestart.ocpCharge !== 1) failures.push(`OCP charge ${afterRestart.ocpCharge} after restart, expected 1`);
  if (!(afterRestart.lampCpuIntensity > 0)) failures.push("shot lamp not restored on the CPU light list");
  s0.forEach((s, i) => {
    if (s.dead || s.state !== "patrol" || s.suspicion !== 0) failures.push(`guard ${i} not reset: ${JSON.stringify(s)}`);
  });
  // The restored lamp lights the meter again: back under it, read the probe.
  P.pos.x = lamp.x; P.pos.z = lamp.z + 1.2;
  await step(12);
  afterRestart.litUnderLamp = +V.level.toFixed(3);
  if (!(afterRestart.litUnderLamp > litAfterShot + 0.1)) {
    failures.push(`restored lamp does not light the probe: ${litAfterShot} -> ${afterRestart.litUnderLamp} (was ${litBefore})`);
  }
  return { ok: failures.length === 0, failures, lamp: { index: best, litBefore, litAfterShot }, deadState, afterRestart };
})()
