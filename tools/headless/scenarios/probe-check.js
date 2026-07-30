// What the light meter is made of: reads the probe with the player's torch
// on, then off, at the same spot with everything else frozen. Also a check
// that crouching lowers the probe (the reading changes with stance).
(async () => {
  const R = window.__renderer;
  R.resize(384, 240);
  const G = window.__guards, D = window.__detection, P = window.__player, V = window.__visibility;
  G.frozen = true;
  D.tuning.fireReaction = 999;
  const g = G.all[0];
  const dir = g.light.dir; const l = Math.hypot(dir.x, dir.z);
  P.pos.x = g.pos.x + (dir.x / l) * 4.0;
  P.pos.z = g.pos.z + (dir.z / l) * 4.0;
  P.velX = 0; P.velZ = 0; P.crouching = false;
  const sample = async (n) => {
    const rows = [];
    for (let i = 0; i < n; i++) {
      await window.__renderStill(1);
      rows.push([+V.illuminance.toFixed(4), +V.raw.toFixed(3), +V.level.toFixed(3), Array.from(R.probeDebug).map(x => +x.toFixed(3))]);
    }
    return rows;
  };
  const out = {};
  P.flashlightOn = true;  out.torchOn = await sample(8);
  P.flashlightOn = false; out.torchOff = await sample(8);
  P.crouching = true;     out.crouchOff = await sample(6);
  P.flashlightOn = true;  out.crouchOn = await sample(6);
  P.crouching = false;
  return out;
})()
