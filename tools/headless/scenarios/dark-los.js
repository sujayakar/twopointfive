// The thesis under fire: a guard with a clear ray to a player it has no light
// on must not know where that player is. Guard 0 is pinned facing the corridor
// with the player 16 m down its cone — past beamRange, torch off — and the
// eye's light floor forced above any reading so the visual signal is exactly
// zero while LOS holds. Then: a thud makes him suspicious, the player moves,
// and a maxed suspicion (what a gunshot or a colleague's shout does) makes him
// alert. He must chase the noise, never the invisible player, and never fire.
(async () => {
  window.__pause(true);
  const R = window.__renderer;
  R.resize(384, 240);
  const G = window.__guards, D = window.__detection, P = window.__player;
  const DT = 0.05;
  const g = G.all[0];
  G.frozen = true;
  D.tuning.fireReaction = 0.7;
  // Stand-in for "dark and out of the beam": no light level counts.
  D.tuning.litMin = 1.0;
  g.pos.x = 4; g.pos.z = 0.8; g.yaw = -Math.PI / 2; // facing -x, down the corridor
  P.pos.x = -12; P.pos.z = 0.8; P.velX = 0; P.velZ = 0;
  P.crouching = false; P.flashlightOn = false;
  await window.__renderStill(6, DT * 1000);
  g.suspicion = 0; g.state = "patrol"; g.mode = "route"; g.stimulus = null;
  const step = async (n) => { for (let i = 0; i < n; i++) await window.__renderStill(1, DT * 1000); };

  await step(10);
  const idle = D.snapshot()[0];

  // A noise from where the player stands (seeded straight into the guard —
  // the thud radius is 4.5 m and this measures the eye, not the ear):
  // suspicious, sweeping toward it. Suspicion must still decay from here even
  // though the ray to the player is clear the whole time.
  g.suspicion = 0.4;
  g.stimulus = { x: P.pos.x, y: 0, z: P.pos.z };
  await step(30);
  const afterThud = D.snapshot()[0];

  // The zero-signal player relocates; the guard must not follow.
  P.pos.z = 0.8 - 3.0;
  await step(20);
  const afterMove = D.snapshot()[0];

  // Full alert with a clear ray to the dark, moved player.
  g.suspicion = 0.9;
  const shots0 = window.__stats.frames;
  let dead = false, fired = 0;
  for (let i = 0; i < 60; i++) {
    await step(1);
    if (g.justFired) fired++;
    if (P.dead) { dead = true; break; }
  }
  const afterAlert = D.snapshot()[0];

  const failures = [];
  if (!idle.hasLOS) failures.push("no geometric LOS in the setup — test is not measuring the dark case");
  if (idle.signal !== 0) failures.push(`signal ${idle.signal}, expected 0 with litMin forced to 1`);
  if (idle.sees) failures.push("guard reports eyes on a zero-signal player");
  if (idle.suspicion > 0) failures.push(`suspicion ${idle.suspicion} accrued from a zero signal`);
  if (idle.stimulus !== null) failures.push("stimulus set from a zero-signal sighting");
  if (afterThud.state !== "suspicious") failures.push(`noise gave state ${afterThud.state}, expected suspicious`);
  if (!(afterThud.suspicion < 0.4)) failures.push(`suspicion ${afterThud.suspicion} did not decay after the noise (LOS must not freeze it)`);
  const st = afterMove.stimulus;
  if (!st || Math.abs(st[1] - 0.8) > 0.6) failures.push(`stimulus ${JSON.stringify(st)} tracked the moved player instead of the noise at z=0.8`);
  if (dead) failures.push("a zero-signal player was shot dead");
  if (fired > 0) failures.push(`guard fired ${fired} rounds at a target it cannot see`);
  if (afterAlert.mode !== "nav" && afterAlert.state === "alert") {
    failures.push(`alert guard is ${afterAlert.mode}, expected nav (chasing the last-known point)`);
  }
  return {
    ok: failures.length === 0, failures,
    setup: { guard: [g.pos.x, g.pos.z], guardYaw: +g.yaw.toFixed(3), player: [P.pos.x, +P.pos.z.toFixed(2)] },
    idle, afterThud, afterMove, afterAlert, fired, playerDead: dead,
  };
})()
