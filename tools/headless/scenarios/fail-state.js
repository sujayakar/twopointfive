// Full loop: seen, shot at, hit, COMPROMISED, then R-restart puts the level
// back to its opening state.
(async () => {
  const R = window.__renderer;
  R.resize(384, 240);
  const G = window.__guards, D = window.__detection, P = window.__player;
  G.frozen = true;
  const g = G.all[0];
  const dir = g.light.dir; const l = Math.hypot(dir.x, dir.z);
  P.pos.x = g.pos.x + (dir.x / l) * 5.0;
  P.pos.z = g.pos.z + (dir.z / l) * 5.0;
  P.velX = 0; P.velZ = 0; P.crouching = false;
  const t0 = window.__stats.frames;
  let died = -1;
  for (let i = 0; i < 90; i++) {
    await window.__renderStill(1);
    if (P.dead && died < 0) { died = window.__stats.frames - t0; break; }
  }
  await window.__renderStill(4);
  const card = document.getElementById("endcard");
  const deadState = {
    playerDead: P.dead,
    framesToDeath: died,
    guard: D.snapshot()[0],
    cardVisible: card ? card.classList.contains("show") : false,
    cardText: card ? card.innerText.replace(/\n/g, " | ") : null,
  };
  // Restart through the same hook R calls, then verify the reset took.
  window.__restart();
  await window.__renderStill(3);
  const s0 = D.snapshot();
  const afterRestart = {
    playerDead: P.dead,
    playerPos: [+P.pos.x.toFixed(2), +P.pos.z.toFixed(2)],
    rounds: P.rounds,
    spares: P.spares,
    guards: s0.map((s) => ({ state: s.state, susp: s.suspicion, dead: s.dead })),
    cardVisible: card ? card.classList.contains("show") : false,
  };
  return { deadState, afterRestart };
})()
