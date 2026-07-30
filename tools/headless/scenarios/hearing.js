// Gunshot hearing, three measurements.
//
//   A) callouts silenced (calloutRange 0): a shot in the north-west corner
//      alerts exactly the guards inside gunshotRange and moves nobody beyond.
//   B) callouts on, natural layout: the alerted set is exactly (in gunshot
//      range) plus (within one callout of a guard that was). One relay hop.
//   C) callouts on, hand-placed line: g1 is reached only by g0's shout and
//      g2 only by g1's — so g2 must stay quiet. The shout does not chain.
//
// The trigger is the real one (mousedown), so the input -> noise chain is
// exercised, not just Detection.noise().
(async () => {
  window.__pause(true);
  const R = window.__renderer;
  R.resize(384, 240);
  const G = window.__guards, D = window.__detection, P = window.__player;
  const DT = 0.05;
  const T = D.tuning;
  const CALLOUT = T.calloutRange;
  D.tuning.fireReaction = 999;
  const canvas = document.querySelector("canvas");
  const step = async (n) => { for (let i = 0; i < n; i++) await window.__renderStill(1, DT * 1000); };

  const fireShot = async (shotAt, setup) => {
    window.__restart();
    G.frozen = false;
    if (setup) setup();
    P.pos.x = shotAt[0]; P.pos.z = shotAt[1]; P.velX = 0; P.velZ = 0;
    await step(4);
    const positions = G.all.map((g) => [+g.pos.x.toFixed(2), +g.pos.z.toFixed(2)]);
    const dists = G.all.map((g) => +Math.hypot(g.pos.x - shotAt[0], g.pos.z - shotAt[1]).toFixed(2));
    const rounds0 = P.rounds;
    canvas.dispatchEvent(new MouseEvent("mousedown", { button: 0, bubbles: true }));
    await step(1);
    window.dispatchEvent(new MouseEvent("mouseup", { button: 0, bubbles: true }));
    await step(8);
    const after = D.snapshot().map((s) => ({ state: s.state, susp: s.suspicion, stimulus: s.stimulus, mode: s.mode }));
    return { positions, dists, fired: P.rounds < rounds0, after };
  };

  // ---- A: no callouts ----------------------------------------------------
  const shotNW = [-25.2, -16.2];
  D.tuning.calloutRange = 0;
  const A = await fireShot(shotNW);
  // ---- B: callouts on, natural layout ---------------------------------------
  D.tuning.calloutRange = CALLOUT;
  const B = await fireShot(shotNW);
  // ---- C: hand-placed relay line -----------------------------------------
  const shotC = [-24, 1];
  const C = await fireShot(shotC, () => {
    G.frozen = true;
    const at = [[5, 0.8], [21, 0.8], [21, -14], [25, 15]];
    G.all.forEach((g, i) => { g.pos.x = at[i][0]; g.pos.z = at[i][1]; });
  });

  const failures = [];
  if (!A.fired) failures.push("A: the click did not fire the pistol");
  const inRangeA = A.dists.map((d) => d <= T.gunshotRange);
  if (!inRangeA.some((x) => x)) failures.push("A: setup has no guard inside gunshotRange");
  if (!inRangeA.some((x) => !x)) failures.push("A: setup has no guard beyond gunshotRange");
  A.after.forEach((s, i) => {
    if (inRangeA[i]) {
      if (s.state !== "alert") failures.push(`A: guard ${i} at ${A.dists[i]} m (in range) is ${s.state}, expected alert`);
      const st = s.stimulus;
      if (!st || Math.hypot(st[0] - shotNW[0], st[1] - shotNW[1]) > 1.5) {
        failures.push(`A: guard ${i} stimulus ${JSON.stringify(st)} not at the shot`);
      }
    } else if (s.state !== "patrol" || s.susp !== 0) {
      failures.push(`A: guard ${i} at ${A.dists[i]} m (beyond ${T.gunshotRange}) reacted: ${s.state} susp ${s.susp}`);
    }
  });
  const inRangeB = B.dists.map((d) => d <= T.gunshotRange);
  const oneHopB = B.positions.map((p, i) => inRangeB[i] || B.positions.some((q, j) =>
    j !== i && inRangeB[j] && Math.hypot(p[0] - q[0], p[1] - q[1]) <= CALLOUT));
  B.after.forEach((s, i) => {
    const alerted = s.state === "alert";
    if (oneHopB[i] && !alerted) failures.push(`B: guard ${i} should have been reached (shot or one shout) but is ${s.state}`);
    if (!oneHopB[i] && (alerted || s.susp !== 0)) {
      failures.push(`B: guard ${i} beyond the shot and every first shout reacted (${s.state}, susp ${s.susp})`);
    }
  });
  // C: expected exactly g0 (shot) and g1 (g0's shout) alert; g2, g3 untouched.
  const expectC = [true, true, false, false];
  C.after.forEach((s, i) => {
    const alerted = s.state === "alert";
    if (expectC[i] && !alerted) failures.push(`C: guard ${i} should be alert, is ${s.state}`);
    if (!expectC[i] && (alerted || s.susp !== 0)) {
      failures.push(`C: guard ${i} reachable only by a second shout still reacted (${s.state}, susp ${s.susp}) — the callout chained`);
    }
  });
  return {
    ok: failures.length === 0, failures,
    gunshotRange: T.gunshotRange, calloutRange: CALLOUT,
    A: { note: "callouts off", shot: shotNW, positions: A.positions, dists: A.dists, inRange: inRangeA, after: A.after },
    B: { note: "callouts on, one hop", shot: shotNW, positions: B.positions, dists: B.dists, inRange: inRangeB, oneHop: oneHopB, after: B.after },
    C: { note: "relay line: g1 hears g0 shout; g2/g3 would need g1 to shout again", shot: shotC, positions: C.positions, dists: C.dists, after: C.after },
  };
})()
