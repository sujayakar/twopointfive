// What the light meter is made of, measured with the world frozen:
//   - crouch vs stand at a spot 4 m down guard 0's beam;
//   - the reading is stable frame to frame there (no bimodal carrier
//     self-occlusion of the torch it is standing in);
//   - the beam reddening on alert does not move the meter (tint is display,
//     not input);
//   - the player's own torch raises the reading when it has something close
//     to bounce off, and adds ~nothing pointed down an empty corridor — the
//     honest shape of "your own light gives you away".
(async () => {
  window.__pause(true);
  const R = window.__renderer;
  R.resize(384, 240);
  const G = window.__guards, D = window.__detection, P = window.__player, V = window.__visibility;
  const canvas = document.querySelector("canvas");
  const DT = 0.05;
  G.frozen = true;
  D.tuning.fireReaction = 999;
  const g = G.all[0];
  const dir = g.light.dir; const l = Math.hypot(dir.x, dir.z);
  P.pos.x = g.pos.x + (dir.x / l) * 4.0;
  P.pos.z = g.pos.z + (dir.z / l) * 4.0;
  P.velX = 0; P.velZ = 0; P.crouching = false; P.flashlightOn = false;
  const stats = (rows) => {
    const lux = rows.map((r) => r[0]);
    const mean = lux.reduce((a, b) => a + b, 0) / lux.length;
    const min = Math.min(...lux), max = Math.max(...lux);
    return { mean: +mean.toFixed(4), min: +min.toFixed(4), max: +max.toFixed(4), spread: +(max / Math.max(min, 1e-6)).toFixed(3) };
  };
  const sample = async (n, settle = 12) => {
    await window.__renderStill(settle, DT * 1000);
    const rows = [];
    for (let i = 0; i < n; i++) {
      await window.__renderStill(1, DT * 1000);
      rows.push([+V.illuminance.toFixed(4), +V.raw.toFixed(3), +V.level.toFixed(3), Array.from(R.probeDebug).map((x) => +x.toFixed(3))]);
    }
    return { rows, stats: stats(rows) };
  };
  const out = {};

  // ---- in the guard's beam: stance, stability, tint --------------------------
  out.stand = await sample(10, 20);
  P.crouching = true;
  out.crouch = await sample(8);
  P.crouching = false;
  // Pin the state (vision zeroed so the brain cannot decide otherwise) and let
  // the eased tint and the smoothed meter settle before each sample.
  D.tuning.seenRate = 0; D.tuning.beamRate = 0;
  g.state = "patrol"; g.suspicion = 0;
  out.tintCalm = await sample(10, 40);
  const calmTorch = D.snapshot()[0].torch;
  g.state = "alert"; g.suspicion = 0.9;
  out.tintAlert = await sample(10, 40);
  out.tintColor = { calm: calmTorch, alert: D.snapshot()[0].torch, alertIntensity: +g.light.intensity.toFixed(1) };

  // ---- own torch, corridor vs wall ------------------------------------------
  P.flashlightOn = false; out.corridorOff = await sample(8);
  P.flashlightOn = true;  out.corridorOn = await sample(8);
  P.flashlightOn = false;
  // Move into the empty conference room a metre off its east wall, then aim
  // at the wall: try the four screen-edge cursor positions and keep the yaw
  // closest to facing +x (yaw = pi/2).
  P.pos.x = -15.0; P.pos.z = 1.0;
  await window.__renderStill(6, DT * 1000);
  const rect = canvas.getBoundingClientRect();
  const cands = [[rect.right - 5, rect.top + rect.height / 2], [rect.left + 5, rect.top + rect.height / 2],
                 [rect.left + rect.width / 2, rect.top + 5], [rect.left + rect.width / 2, rect.bottom - 5]];
  let bestPt = cands[0], bestErr = Infinity, bestYaw = 0;
  for (const [cx, cy] of cands) {
    canvas.dispatchEvent(new MouseEvent("mousemove", { clientX: cx, clientY: cy, bubbles: true }));
    await window.__renderStill(20, DT * 1000);
    const err = Math.abs(Math.atan2(Math.sin(P.yaw - Math.PI / 2), Math.cos(P.yaw - Math.PI / 2)));
    if (err < bestErr) { bestErr = err; bestPt = [cx, cy]; bestYaw = P.yaw; }
  }
  canvas.dispatchEvent(new MouseEvent("mousemove", { clientX: bestPt[0], clientY: bestPt[1], bubbles: true }));
  await window.__renderStill(20, DT * 1000);
  out.wall = { playerPos: [+P.pos.x.toFixed(2), +P.pos.z.toFixed(2)], yaw: +P.yaw.toFixed(3), yawErrFromEast: +bestErr.toFixed(3), wallAtX: -14.08 };
  P.flashlightOn = false; out.wallOff = await sample(8);
  P.flashlightOn = true;  out.wallOn = await sample(8);
  P.flashlightOn = false;

  const failures = [];
  for (const k of ["stand", "crouch", "tintCalm", "tintAlert", "wallOff", "wallOn"]) {
    if (out[k].stats.spread > 1.5) failures.push(`${k}: bimodal probe, max/min = ${out[k].stats.spread}`);
  }
  const tintRatio = out.tintAlert.stats.mean / Math.max(out.tintCalm.stats.mean, 1e-6);
  out.tintRatio = +tintRatio.toFixed(3);
  if (Math.abs(tintRatio - 1) > 0.15) failures.push(`beam tint changed the meter by ${((tintRatio - 1) * 100).toFixed(1)}%`);
  out.torchDelta = {
    corridor: +(out.corridorOn.stats.mean - out.corridorOff.stats.mean).toFixed(4),
    wall: +(out.wallOn.stats.mean - out.wallOff.stats.mean).toFixed(4),
  };
  if (bestErr < 0.6) {
    if (!(out.wallOn.stats.mean > out.wallOff.stats.mean * 1.10)) {
      failures.push(`torch-on facing a wall did not raise the reading: ${out.wallOff.stats.mean} -> ${out.wallOn.stats.mean}`);
    }
  } else {
    failures.push(`could not aim the player at the wall (yaw ${P.yaw.toFixed(2)}) — torch-delta unmeasured`);
  }
  return {
    ok: failures.length === 0, failures,
    geometry: { guard: [+g.pos.x.toFixed(2), +g.pos.z.toFixed(2)], playerInBeam: [+(g.pos.x + (dir.x / l) * 4).toFixed(2), +(g.pos.z + (dir.z / l) * 4).toFixed(2)] },
    ...out,
  };
})()
