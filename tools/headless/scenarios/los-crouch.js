// Crouched behind a cubicle back panel (1.35 m) in the guard's beam: LOS must
// break and the guard must score nothing; standing at the same spot the head
// clears the partition, LOS returns and the beam finds the player. Guard 1
// is pinned in the aisle facing south into the cubicle.
(async () => {
  window.__pause(true);
  const R = window.__renderer;
  R.resize(384, 240);
  const G = window.__guards, D = window.__detection, P = window.__player, RC = window.__raycaster;
  const DT = 0.05;
  G.frozen = true;
  D.tuning.fireReaction = 999;
  const T = D.tuning;
  const g = G.all[1];
  g.pos.x = -18; g.pos.z = -5.5; g.yaw = Math.PI;
  P.pos.x = -18; P.pos.z = -8.4; P.velX = 0; P.velZ = 0; P.flashlightOn = false;
  await window.__renderStill(4, DT * 1000);
  const eye = { x: g.pos.x, y: T.eyeHeight, z: g.pos.z };
  const raw = {
    standingBlocked: RC.blocked(eye, { x: P.pos.x, y: T.targetStand, z: P.pos.z }, T.losMargin),
    crouchedBlocked: RC.blocked(eye, { x: P.pos.x, y: T.targetCrouch, z: P.pos.z }, T.losMargin),
  };
  const run = async (crouch, n) => {
    P.crouching = crouch;
    // Let the stance-dependent probe settle, then start from a blank mind.
    await window.__renderStill(6, DT * 1000);
    g.suspicion = 0; g.state = "patrol"; g.mode = "route"; g.stimulus = null;
    const rows = [];
    for (let i = 0; i < n; i++) {
      await window.__renderStill(1, DT * 1000);
      const s = D.snapshot()[1];
      rows.push({ los: s.hasLOS, sees: s.sees, beam: s.inBeam, signal: s.signal, susp: s.suspicion, state: s.state, lit: +window.__visibility.level.toFixed(3) });
    }
    return rows;
  };
  const crouched = await run(true, 16);
  const standing = await run(false, 16);
  P.crouching = false;
  const failures = [];
  if (raw.standingBlocked) failures.push("raw raycast: standing target blocked (head should clear the 1.35 m panel)");
  if (!raw.crouchedBlocked) failures.push("raw raycast: crouched target clear (panel should block it)");
  if (crouched.some((r) => r.los)) failures.push("guard reports LOS to the crouched player");
  if (crouched.some((r) => r.sees)) failures.push("guard has eyes on the crouched player");
  const cLast = crouched[crouched.length - 1];
  if (cLast.susp !== 0) failures.push(`crouched: suspicion reached ${cLast.susp}, expected 0`);
  if (!standing.every((r) => r.los)) failures.push("guard lost LOS to the standing player");
  const sLast = standing[standing.length - 1];
  if (!(sLast.susp > D.tuning.suspiciousAt)) failures.push(`standing: suspicion only ${sLast.susp} after ${standing.length} frames in the beam`);
  return {
    ok: failures.length === 0, failures,
    guard: { pos: [g.pos.x, g.pos.z], yaw: g.yaw, beam: [+g.light.dir.x.toFixed(3), +g.light.dir.y.toFixed(3), +g.light.dir.z.toFixed(3)] },
    playerPos: [P.pos.x, P.pos.z],
    partition: { z: -7.05, top: 1.35, note: "cubicle back panel between guard (z=-5.5) and player (z=-8.4)" },
    heights: { eye: T.eyeHeight, targetStand: T.targetStand, targetCrouch: T.targetCrouch, probeCrouch: T.probeCrouch },
    raycast: raw,
    crouched,
    standing,
  };
})()
