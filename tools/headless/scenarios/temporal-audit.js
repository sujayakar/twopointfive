// Temporal-reprojection audit for animated geometry (Track B4).
//
//   npm run build
//   python3 tools/headless/run.py --scenario tools/headless/scenarios/temporal-audit.js \
//       --arg '{"modes":[0,1,2,3]}' --json /tmp/temporal.json
//
// Runs, per debug tap mode (RenderSettings.debugTapMode: 0 shipped, 1
// fork-point behaviour, 2 loose absolute band, 3 accept-all), the same
// scenarios at a fixed internal resolution with the rAF loop paused:
//
//   stand      45 frames standing still — character history vs the static
//              window around it (control).
//   walk       60 frames walking a circle at the game's own walk gait
//              (Player.scriptedMove drives player.update), sampled every 15.
//   crouchwalk the same crouched — the stealth game's primary stance.
//   pop_*      the character is camera-invisible for 40 frames while the scene
//              settles, then appears; a character pixel whose history length
//              exceeds its age k has taken history from the geometry behind
//              it (fracOver). Variants: standing in the open, crouched in the
//              open, crouched pressed against the nearest collider.
//
// Every number is a history length (effective sample count) read from the
// direct-signal moments, masked by the G-buffer surface class so "character"
// means the animated geometry's own texels, not a screen box around it.
(async () => {
  const arg = window.__scenarioArg || {};
  const W = arg.width || 384, H = arg.height || 240;
  const modes = arg.modes || [0, 1, 2];
  const cases = arg.cases || ["stand", "walk", "crouchwalk", "pop_stand_open", "pop_crouch_open", "pop_crouch_contact"];
  const S = window.__settings, R = window.__renderer, P = window.__player, G = window.__guards;
  const bg = window.__benchGuard;
  bg.active = true; bg.aborted = false; bg.abortAt = performance.now() + 3.6e6;

  // Reference reprojection params: no colour clamp, no history cap — history
  // length is then purely the geometric validation being audited.
  S.reference = arg.reference !== undefined ? arg.reference : true;
  window.__pinBenchPose();
  const home = { x: P.pos.x, z: P.pos.z };
  for (const g of G.guards) { g.pos.x = 120; g.pos.z = 120; }

  const resetHistory = () => { R.resize(W, H); R.resize(W, H + 1); R.resize(W, H); };
  // The follow camera damps toward the player over ~10 frames at the harness
  // dt; probes must start from a camera that has stopped moving.
  const settleCamera = async (n = 20) => { for (let i = 0; i < n; i++) await window.__renderStill(1); };
  await settleCamera(20);
  const grow = (rects, px) => rects.map((r) => ({ x0: r.x0 - px, y0: r.y0 - px, x1: r.x1 + px, y1: r.y1 + px }));
  const charStats = (age) => window.__historyStats({ cls: "dynamic", age });
  const staticStats = () => window.__historyStats({ cls: "static", rects: grow(window.__playerScreenRects(), 24) });
  const charBox = () => {
    const rs = window.__playerScreenRects();
    let x0 = 1e9, y0 = 1e9, x1 = -1e9, y1 = -1e9;
    for (const r of rs) { x0 = Math.min(x0, r.x0); y0 = Math.min(y0, r.y0); x1 = Math.max(x1, r.x1); y1 = Math.max(y1, r.y1); }
    return { x0: Math.max(0, x0 - 2), y0: Math.max(0, y0 - 2), x1: Math.min(W, x1 + 2), y1: Math.min(H, y1 + 2) };
  };
  const asciiMap = async () => {
    // The character's bounding box: history length as base-36-ish glyphs, '.'
    // for non-character pixels, so the shape of any hole or theft is visible.
    const b = charBox();
    const m = await window.__historyRect(b);
    // 0-9 a-z A-Z = 0..61, '#' beyond, '*' fresh, '.' not a character pixel.
    const DIGITS = "0123456789abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ";
    const glyph = (z, c) => {
      if (c !== 2) return ".";
      if (z < 1.5) return "*";
      const v = Math.round(z);
      return v >= DIGITS.length ? "#" : DIGITS[v];
    };
    const rows = [];
    for (let y = 0; y < m.h; y++) {
      let row = "";
      for (let x = 0; x < m.w; x++) row += glyph(m.z[y * m.w + x], m.cls[y * m.w + x]);
      rows.push(row);
    }
    return rows;
  };

  // Nearest collider from `from` over 16 directions: where a player pressing
  // into cover ends up (the collision circle stops the march), plus what the
  // camera-visible geometry actually is there, sampled by ray-casting at
  // three heights along the approach.
  const findContact = (from) => {
    let best = null;
    for (let i = 0; i < 16; i++) {
      const a = (i / 16) * Math.PI * 2;
      const dx = Math.sin(a), dz = Math.cos(a);
      for (let t = 0.05; t <= 6.0; t += 0.05) {
        if (P.blocked(from.x + dx * t, from.z + dz * t)) {
          if (!best || t < best.t) best = { t, dx, dz };
          break;
        }
      }
    }
    if (!best) return null;
    const pos = { x: from.x + best.dx * (best.t - 0.05), z: from.z + best.dz * (best.t - 0.05) };
    const heights = [0.15, 0.45, 0.75, 1.05];
    const faces = heights.map((h) => {
      const hit = window.__raycaster.raycast({ x: pos.x, y: h, z: pos.z }, { x: best.dx, y: 0, z: best.dz }, 2.0);
      return { h, t: hit ? +hit.t.toFixed(3) : null };
    });
    return { pos, dir: { x: best.dx, z: best.dz }, faces };
  };

  const runStand = async () => {
    P.crouching = false; P.pos.x = home.x; P.pos.z = home.z; P.velX = 0; P.velZ = 0;
    await settleCamera();
    resetHistory();
    for (let i = 0; i < 45; i++) await window.__renderStill(1);
    return { frames: 45, character: await charStats(45), control: await staticStats(), map: await asciiMap() };
  };

  const runWalk = async (crouch) => {
    P.crouching = crouch; P.pos.x = home.x; P.pos.z = home.z; P.velX = 0; P.velZ = 0;
    await settleCamera();
    resetHistory();
    for (let i = 0; i < 45; i++) await window.__renderStill(1);
    const samples = [];
    for (let f = 15; f <= 60; f += 15) {
      await window.__renderMotion(15);
      samples.push({ f, character: await charStats(45 + f), control: await staticStats() });
      if (f === 30) samples[samples.length - 1].map = await asciiMap();
    }
    P.crouching = false;
    return { warm: 45, samples };
  };

  const runPop = async (stance, where) => {
    P.crouching = stance === "crouch";
    let contact = null;
    if (where === "contact") {
      contact = findContact(home);
      if (!contact) return { error: "no collider within 6 m of the bench pose" };
      P.pos.x = contact.pos.x; P.pos.z = contact.pos.z;
    } else {
      P.pos.x = home.x; P.pos.z = home.z;
    }
    P.velX = 0; P.velZ = 0;
    await settleCamera();
    resetHistory();
    window.__hidePlayer(true);
    for (let i = 0; i < 40; i++) await window.__renderStill(1);
    window.__hidePlayer(false);
    const frames = [];
    let map = null;
    for (let k = 1; k <= 12; k++) {
      await window.__renderStill(1);
      frames.push({ k, ...(await charStats(k)) });
      if (k === 4) map = await asciiMap();
    }
    return { stance, where, contact, warm: 40, frames, map };
  };

  const out = { W, H, reference: S.reference, home, modes: {} };
  const t0 = performance.now();
  for (const mode of modes) {
    S.debugTapMode = mode;
    const res = {};
    for (const c of cases) {
      if (c === "stand") res.stand = await runStand();
      else if (c === "walk") res.walk = await runWalk(false);
      else if (c === "crouchwalk") res.crouchwalk = await runWalk(true);
      else if (c === "pop_stand_open") res.pop_stand_open = await runPop("stand", "open");
      else if (c === "pop_crouch_open") res.pop_crouch_open = await runPop("crouch", "open");
      else if (c === "pop_crouch_contact") res.pop_crouch_contact = await runPop("crouch", "contact");
    }
    out.modes[mode] = res;
  }
  S.debugTapMode = 0;
  window.__hidePlayer(false);
  out.wallSeconds = Math.round((performance.now() - t0) / 1000);
  return out;
})()
