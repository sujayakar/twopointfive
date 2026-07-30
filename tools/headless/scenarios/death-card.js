// Stand in guard 0's beam until shot; leave the COMPROMISED card up for the shot.
(async () => {
  window.__pause(true);
  const R = window.__renderer;
  R.resize(384, 240);
  const G = window.__guards, P = window.__player;
  const DT = 0.05;
  G.frozen = true;
  const g = G.all[0];
  const dir = g.light.dir; const l = Math.hypot(dir.x, dir.z);
  P.pos.x = g.pos.x + (dir.x / l) * 5.0;
  P.pos.z = g.pos.z + (dir.z / l) * 5.0;
  P.velX = 0; P.velZ = 0; P.crouching = false;
  for (let i = 0; i < 200 && !P.dead; i++) await window.__renderStill(1, DT * 1000);
  // Let the card fade in.
  await window.__renderStill(24, DT * 1000);
  const card = document.getElementById("endcard");
  const text = card ? card.innerText.replace(/\n/g, " | ") : null;
  const failures = [];
  if (!P.dead) failures.push("player did not die in the beam");
  if (!card?.classList.contains("show") || !/COMPROMISED/.test(text ?? "")) failures.push(`card: ${text}`);
  return { ok: failures.length === 0, failures, dead: P.dead, card: text };
})()
