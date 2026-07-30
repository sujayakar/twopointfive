// Crouched behind a cubicle back panel (1.35 m) in the guard's beam: LOS must
// break; standing at the same spot the head clears the partition and it must
// not. Guard 1 is pinned in the aisle facing north into the cubicle.
(async () => {
  const R = window.__renderer;
  R.resize(384, 240);
  const G = window.__guards, D = window.__detection, P = window.__player, RC = window.__raycaster;
  G.frozen = true;
  D.tuning.fireReaction = 999;
  const T = D.tuning;
  const g = G.all[1];
  g.pos.x = -18; g.pos.z = -5.5; g.yaw = Math.PI;
  P.pos.x = -18; P.pos.z = -8.4; P.velX = 0; P.velZ = 0; P.flashlightOn = false;
  await window.__renderStill(2);
  const eye = { x: g.pos.x, y: T.eyeHeight, z: g.pos.z };
  const raw = {
    standingBlocked: RC.blocked(eye, { x: P.pos.x, y: T.targetStand, z: P.pos.z }, T.losMargin),
    crouchedBlocked: RC.blocked(eye, { x: P.pos.x, y: T.targetCrouch, z: P.pos.z }, T.losMargin),
  };
  const run = async (crouch, n) => {
    P.crouching = crouch;
    // Start each phase from a blank mind so the accumulator is measurable.
    g.suspicion = 0; g.state = "patrol"; g.mode = "route"; g.stimulus = null;
    const rows = [];
    for (let i = 0; i < n; i++) {
      await window.__renderStill(1);
      const s = D.snapshot()[1];
      rows.push({ los: s.hasLOS, beam: s.inBeam, signal: s.signal, susp: s.suspicion, state: s.state });
    }
    return rows;
  };
  const crouched = await run(true, 8);
  const standing = await run(false, 8);
  P.crouching = false;
  return {
    guard: { pos: [g.pos.x, g.pos.z], yaw: g.yaw, beam: [+g.light.dir.x.toFixed(3), +g.light.dir.y.toFixed(3), +g.light.dir.z.toFixed(3)] },
    playerPos: [P.pos.x, P.pos.z],
    partition: { z: -7.05, top: 1.35, note: "cubicle back panel between guard (z=-5.5) and player (z=-8.4)" },
    raycast: raw,
    crouched,
    standing,
  };
})()
