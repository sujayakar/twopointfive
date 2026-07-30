// Stand in guard 0's beam until shot; leave the COMPROMISED card up for the shot.
(async () => {
  const R = window.__renderer;
  R.resize(384, 240);
  const G = window.__guards, P = window.__player;
  G.frozen = true;
  const g = G.all[0];
  const dir = g.light.dir; const l = Math.hypot(dir.x, dir.z);
  P.pos.x = g.pos.x + (dir.x / l) * 5.0;
  P.pos.z = g.pos.z + (dir.z / l) * 5.0;
  P.velX = 0; P.velZ = 0; P.crouching = false;
  for (let i = 0; i < 90 && !P.dead; i++) await window.__renderStill(1);
  await window.__renderStill(6);
  const card = document.getElementById("endcard");
  return { dead: P.dead, card: card ? card.innerText.replace(/\n/g, " | ") : null };
})()
