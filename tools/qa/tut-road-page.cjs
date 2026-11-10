// THE TUTORIAL'S FIRST MISSION ON THE REAL PAGE, WITH REAL KEYS: levels/tut-road.json played from its first line to the debrief in headless Chromium on the
// software WebGPU adapter (the page is ?paused and stepped by __draw.run with nothing drawn, every input a DOM event from Playwright's keyboard and mouse,
// read by the page's own listeners).
// What no test of the game alone can see: that the bundle boots the level and its script, that each line is up as it is
// written with the player's own key on a key cap, that the view and the trigger are locked against real events until their lines, that the two cards come
// up and close, and that a hand at the keyboard and the mouse gets through every step: walking by the keys the picture gives (W is up the SCREEN, whatever
// way the view was turned), the cursor put on a man where the page draws him.
//   PORT=5247 node tools/qa/tut-road-page.cjs        (a dev server on that port: PORT=5247 bun run tools/serve.ts)
//   env: QUALITY (default verylow) · STEPS (stop after this many)
// exit 0 = every line passes · 1 = a FAIL or a page error.
const { page: pageRun, NOISE } = require('./page.cjs'), PAGE = pageRun({ name: 'tut-road-page' }), { check, note: log } = PAGE;
const fs = require('fs'), path = require('path');
const QUALITY = process.env.QUALITY || 'verylow', ROOT = path.resolve(__dirname, '../..');
const STEPS = +(process.env.STEPS || 99);
const same = (drawn, written) => drawn.replace(/\s+/g, ' ').trim() === written.replace(/[\[\]]/g, '').replace(/\s+/g, ' ').trim();   // (the line as drawn carries its key on a cap, not in brackets)
/** a key cap's words → the key's code */
const codeOf = cap => cap === 'Space' ? 'Space' : cap === 'Ctrl' ? 'ControlLeft' : cap === 'Shift' ? 'ShiftLeft' : cap === 'Tab' ? 'Tab' : /^[0-9]$/.test(cap) ? 'Digit' + cap : /^[A-Z]$/.test(cap) ? 'Key' + cap : cap;

(async () => {
  const data = JSON.parse(fs.readFileSync(path.join(ROOT, 'levels', 'tut-road.json'), 'utf8')), steps = data.scenarios.lesson.steps, step = id => steps.find(s => s.id === id);
  const { page, errors, step: frames } = await PAGE.open({ timeout: 0, errors: { skip: [NOISE, /Failed to load resource/] } });
  const run = n => frames(n);   // (the last of them laid out and never drawn: the renderer is stubbed, a drawn frame is half a minute here)
  const read = () => page.evaluate(() => { const D = window.__draw, G = D.game, V = G.view(), L = V.lesson, M = G.map(), T = G.tally(), el = document.getElementById('railprompt'), card = document.getElementById('railcard'), me = M.men.find(m => m.kind === 'player'), men = M.men.filter(m => m.kind === 'man'), gun = V.kit.gun;
    const hut = M.doors.find(d => d.name === 'hut') ?? M.doors[0];
    return { i: L?.step ?? -1, id: L?.id ?? null, done: !!L?.finished, line: el?.querySelector('.line')?.textContent ?? '', keys: [...(el?.querySelectorAll('.line kbd') ?? [])].map(k => k.textContent), up: !!el && getComputedStyle(el).display !== 'none' && !!el.querySelector('.line'),
      fatal: document.getElementById('fatal')?.textContent || '', rails: !!L?.rails, sum: (L?.rails?.open ?? []).join(' '), t: +G.time.toFixed(1), pos: me ? [+me.x.toFixed(2), +me.z.toFixed(2)] : [0, 0], down: !!V.kit.down, away: !V.kit.slots.some(s => s.up && s.thing !== 'hands'), rounds: gun ? gun.mag + (gun.chamber ? 1 : 0) : -1,
      yaw: +D.cam.yaw.toFixed(3), stage: T.stage, men: men.map(x => ({ up: !x.down, p: [x.x, 1.25, x.z] })), foes: men.filter(x => !x.down).length, doorOff: !!hut?.off || !!hut?.broken,
      card: { on: !!card && card.classList.contains('on'), h: card?.querySelector('h2')?.textContent ?? '', p: [...(card?.querySelectorAll('p') ?? [])].map(x => x.textContent).join(' / ') } }; });
  const at = p => page.evaluate(p => { const c = document.getElementById('view'), r = c.getBoundingClientRect(), [x, y, vis] = window.__draw.cam.project(p, r.width, r.height, 1); return { x: r.left + x, y: r.top + y, vis: !!vis && x > 8 && y > 8 && x < r.width - 8 && y < r.height - 8 }; }, p);
  const tap = async k => { await page.keyboard.down(k); await run(2); await page.keyboard.up(k); await run(2); };
  /** the next point of the way there that is more than a pace off: a search over the floor the page's own map has, so that a hand walks round a truck and does not push at it */
  const next = p => page.evaluate(p => { const W = window, G = W.__draw.game, me = G.map().men.find(m => m.kind === 'player'), C = 0.2, NX = 200, NZ = 140, R = 0.36;
    if (!W.__grid) { const g = W.__grid = new Uint8Array(NX * NZ), B = G.floor().boxes; for (let b = 0; b + 5 < B.length; b += 6) { const [x, z, hx, hz, deg] = B.slice(b, b + 5), a = deg * Math.PI / 180, c = Math.cos(a), s = Math.sin(a), e = Math.hypot(hx, hz) + R; if (hx > 15 || hz > 15) continue;
        for (let j = Math.max(0, Math.floor((z - e) / C)); j <= Math.min(NZ - 1, (z + e) / C); j++) for (let i = Math.max(0, Math.floor((x - e) / C)); i <= Math.min(NX - 1, (x + e) / C); i++) { const dx = (i + 0.5) * C - x, dz = (j + 0.5) * C - z, u = dx * c - dz * s, v = dx * s + dz * c; if (Math.abs(u) < hx + R && Math.abs(v) < hz + R) g[j * NX + i] = 1; } } }
    const g = W.__grid, cell = (x, z) => Math.floor(z / C) * NX + Math.floor(x / C), free = k => { if (!g[k]) return k; let best = -1, bd = 1e9; for (let q = 0; q < g.length; q++) if (!g[q]) { const d = Math.hypot(q % NX - k % NX, Math.floor(q / NX) - Math.floor(k / NX)); if (d < bd) { bd = d; best = q; } } return best; };
    const from = free(cell(me.x, me.z)), to = free(cell(p[0], p[2])), prev = new Int32Array(g.length).fill(-1), q = [to]; prev[to] = to;
    for (let h = 0; h < q.length && prev[from] < 0; h++) { const k = q[h]; for (const d of [1, -1, NX, -NX]) { const n = k + d; if (n >= 0 && n < g.length && !g[n] && prev[n] < 0) { prev[n] = k; q.push(n); } } }
    if (prev[from] < 0) return p; let k = from, n = 0; while (k !== to && n++ < 6) k = prev[k]; return k === to ? p : [(k % NX + 0.5) * C, 0, (Math.floor(k / NX) + 0.5) * C]; }, p);
  /** walk to a place by the keys the PICTURE gives: W is up the screen. Eight frames a look; `keys` held with them (Shift); `raw`: straight at it (into a wall) */
  const walk = async (p, o = {}) => { const near = o.near ?? 0.35, held = new Set(); let R = null;
    const set = async want => { for (const k of [...held]) if (!want.includes(k)) { await page.keyboard.up(k); held.delete(k); } for (const k of want) if (!held.has(k)) { await page.keyboard.down(k); held.add(k); } };
    try { for (let n = 0; n < (o.max ?? 15) * 7.5; n++) { R = await read(); if (R.down || Math.hypot(R.pos[0] - p[0], R.pos[1] - p[2]) < near || (o.stop && o.stop(R))) break;
        const a = await at([R.pos[0], 0, R.pos[1]]), b = await at(o.raw ? p : await next(p)), dx = b.x - a.x, dy = b.y - a.y, m = Math.max(Math.abs(dx), Math.abs(dy)), want = [...(o.keys ?? [])];
        if (dx > 0.4 * m) want.push('KeyD'); if (dx < -0.4 * m) want.push('KeyA'); if (dy < -0.4 * m) want.push('KeyW'); if (dy > 0.4 * m) want.push('KeyS');
        await set(want); await run(8); } } finally { await set([]); await run(2); }
    return await read(); };
  const route = async (pts, o) => { let R = null; for (const p of pts) R = await walk(p, o); return R; };
  /** the cursor on a man (his place on the list) where the page draws him */
  const onMan = async k => { const m = (await read()).men[k]; if (!m || !m.up) return false; const s = await at(m.p); await page.mouse.move(s.x, s.y); await run(2); return true; };
  /** the cursor on him and the left button held, until `stop` */
  const shoot = async (k, secs, stop) => { if (!await onMan(k)) return; await page.mouse.down(); try { for (let n = 0; n < secs * 10; n++) { await onMan(k); await run(6); if (stop(await read())) break; } } finally { await page.mouse.up(); await run(3); } };
  const until = async (id, n) => { for (let k = 0; k < n; k++) { await run(60); if ((await read()).id !== id) break; } };

  const url = PAGE.url(`levelFile=tut-road.json&quiet&paused&quality=${QUALITY}&mute`);
  log(url);
  await page.goto(url, { timeout: 0 });
  await page.evaluate(async () => { await window.__ready; window.__engine.render = () => {}; window.__draw.run(3, [], 'last'); });
  let R = await read();
  log('booted', JSON.stringify({ ...R, men: R.men.length }));
  check(!R.fatal && R.rails && R.up && R.i === 0 && same(R.line, steps[0].line) && R.foes === 6, 'the page boots the mission on rails: its first line is up as written, six men on the floor',
    `fatal "${R.fatal}" · rails ${R.rails} (${R.sum}) · line "${R.line}" keys [${R.keys}] · men ${R.foes}`);
  // ---- locked: before their lines, a hand on the view, the rifle's key, the trigger, the crouch and a throw does nothing
  { const y0 = R.yaw; await page.keyboard.down('ControlLeft'); for (let x = 400; x <= 800; x += 100) { await page.mouse.move(x, 320); await run(3); } await page.keyboard.up('ControlLeft');
    await page.keyboard.down('Space'); for (let x = 800; x >= 400; x -= 100) { await page.mouse.move(x, 320); await run(3); } await page.keyboard.up('Space');
    for (const k of ['Digit2', 'KeyC', 'Digit3', 'KeyR']) await tap(k);
    await page.mouse.move(700, 300); await page.mouse.down(); await run(4); await page.mouse.up(); await run(4); R = await read();
    check(Math.abs(R.yaw - y0) < 1e-3 && R.away && R.i === 0, 'before anything is taught the view does not turn, and the rifle\'s key and a click do nothing',
      `the view's yaw ${y0} → ${R.yaw} · rifle ${R.away ? 'slung' : 'in his hands'} · step ${R.i} · ${R.sum}`); }

  const hand = {
    move: async () => { await route([[3.4, 0, 17.2], [3.1, 0, 12.6]], { near: 0.5 }); },
    // (the view's key is the one the line's own cap names)
    turn: async () => { const R0 = await read(), k = codeOf(R0.keys[0] || 'Ctrl'); await page.keyboard.down(k); for (let x = 400; x <= 900 && (await read()).id === 'turn'; x += 25) { await page.mouse.move(x, 320); await run(3); } await page.keyboard.up(k);
      const R1 = await read(); check(Math.abs(R1.yaw - R0.yaw) > 0.5, 'the view turns with the key on the line\'s cap held and the mouse moved', `cap [${R0.keys}] → ${k} · the view's yaw ${R0.yaw} → ${R1.yaw}`);
      // (and back as it opened, by the script and not by a hand: the pictures after this are the level as a player first sees it)
      for (let n = 0; n < 6; n++) { const y = (await read()).yaw; if (Math.abs(y - R0.yaw) < 0.01) break; await page.evaluate(d => window.__draw.cam.rotate(d), R0.yaw - y); await run(6); }
      const back = (await read()).yaw; if (Math.abs(back - R0.yaw) > 0.05) log(`  (the view is at ${back}, it opened at ${R0.yaw})`); },
    ready: async () => { await tap('Digit2'); await run(40); },
    sprint: async () => { await route([[3.2, 0, 11.1]], { near: 0.5 }); await route([[10.35, 0, 11.9]], { keys: ['ShiftLeft'], near: 0.7 }); },
    wall: async () => { await walk([11.6, 0, 11.9], { raw: true, max: 3, stop: R => R.id !== 'wall' }); },
    peek: async () => { for (let n = 0; n < 6 && (await read()).id === 'peek'; n++) await walk([10.35, 0, 14.05], { raw: true, max: 1, near: 0.05, stop: R => R.id !== 'peek' }); for (let n = 0; n < 30 && (await read()).id === 'peek'; n++) { await onMan(0); await run(6); } },
    fire: async () => { for (let n = 0; n < 8 && (await read()).men[0].up; n++) { await onMan(0); await page.mouse.down(); await run(3); await page.mouse.up(); await run(20); } check(!(await read()).men[0].up, 'from the gateway\'s corner the cursor on the sentry and a click put him down', ''); },
    auto: async () => { await tap('Digit2'); await run(10); await tap('Digit1'); await run(10); },
    off: async () => { const R0 = await read(); await walk([R0.pos[0] - 2, 0, R0.pos[1]], { raw: true, max: 2, stop: R => R.id !== 'off' }); },
    inner: async () => { await route([[10.0, 0, 14.6], [12.2, 0, 15.2], [15.2, 0, 12.75]], { near: 0.5 }); },
    door: async () => { const s = await at([17, 1.1, 12.75]); await page.mouse.move(s.x, s.y); await run(3); const r0 = (await read()).rounds;
      await page.mouse.down(); for (let n = 0; n < 60; n++) { await run(6); const q = await read(); if (q.doorOff || q.id !== 'door') break; } await page.mouse.up(); await run(3);
      const q = await read(); check(q.id !== 'door' && q.foes === 5, 'the trigger held on the hut\'s door takes it, and the man behind the boards is not hit', `now '${q.id}' after ${r0 - q.rounds} round(s) · men up ${q.foes}`); },
    reload: async () => { await tap('KeyR'); await until('reload', 8); },
    thin: async () => { await walk([16.3, 0, 12.9], { near: 0.3 }); await run(150); const q0 = await read(); await shoot(1, 8, q => !q.men[1].up);   // (he comes out of his nook to the boards as the line comes up)
      const q = await read(); check(!q.men[1].up && !q.down, 'from the doorway, the cursor on the man behind the boards and the trigger held puts him down through them', `after ${q0.rounds - q.rounds} round(s)`); },
    arch: async () => { await route([[18.0, 0, 12.8], [20.8, 0, 12.9], [21.25, 0, 14.3], [21.5, 0, 14.75]], { near: 0.4 }); await until('arch', 3); },
    crouch: async () => { await run(60); await tap('KeyC'); },
    low: async () => { await route([[21.6, 0, 16.7], [26.7, 0, 16.7]], { near: 0.5 }); },
    cover: async () => { await route([[27.9, 0, 16.4]], { near: 0.4 }); await tap('KeyC'); await route([[28.6, 0, 16.2]], { near: 0.3 }); await walk([28.6, 0, 15.0], { raw: true, max: 3, stop: R => R.id !== 'cover' }); },
    gunner: async () => { await walk([30.2, 0, 15.9], { raw: true, max: 2, near: 0.05, stop: R => R.id !== 'gunner' }); for (let n = 0; n < 20; n++) { await onMan(2); await run(6); } await shoot(2, 8, q => !q.men[2].up); await until('gunner', 4); },
    watch: async () => { await until('watch', 12); },
    last: async () => { for (let n = 0; n < 12; n++) { const q = await read(), k = q.men.findIndex((x, i) => i >= 3 && x.up); if (k < 0 || q.down) break; await shoot(k, 4, z => !z.men[k].up || z.down); } await until('last', 4); },
    radio: async () => { const R0 = await read(); await walk([R0.pos[0], 0, R0.pos[1] + 1.5], { raw: true, max: 2 }); await route([[30.4, 0, 16.5], [33.9, 0, 14.0], [34.0, 0, 11.6], [32.7, 0, 11.4]], { near: 0.4 }); },
    // (the interaction key is the one the line's own cap names)
    take: async () => { const q = await read(), k = codeOf(q.keys[0] || 'Space'); await tap(k); for (let n = 0; n < 12 && (await read()).stage !== 'exfil'; n++) await run(60);
      check((await read()).stage === 'exfil', 'the key on the line\'s cap takes the radio', `cap [${q.keys}] → ${k} · the mission's stage '${(await read()).stage}'`); },
    out: async () => { await route([[34.0, 0, 11.7], [34.3, 0, 13.6], [36.2, 0, 14.25], [39.2, 0, 14.25]], { near: 0.4, stop: R => R.stage === 'done' }); },
  };

  const cardsSeen = [];
  for (let n = 0; n < steps.length && n < STEPS; n++) {
    const s = steps[n]; R = await read();
    if (R.id !== s.id) { check(false, `step ${n + 1} '${s.id}' is the one up`, `up: ${R.id} (${R.i}) · line "${R.line}"`); break; }
    if (R.down) { check(false, `he is on his feet at step ${n + 1} '${s.id}'`, `down at (${R.pos})`); break; }
    // A CARD on the step (its `explain`): up with its words, and Enter closes it
    if (s.explain) { await run(30); const C = (await read()).card; await page.waitForTimeout(450); await page.keyboard.press('Enter'); await run(10); const D = (await read()).card; cardsSeen.push(s.id);
      check(C.on && C.h === s.explain.title && !!C.p && !D.on, `step ${n + 1} '${s.id}' opens on its card, and Enter closes it`, `"${C.h}": ${C.p} · after Enter the card is ${D.on ? 'UP' : 'gone'}`); R = await read(); }
    const written = s.setupLine ?? s.line;
    check(R.up && (same(R.line, written) || same(R.line.replace(/\b(Space|Ctrl|E)\b/g, '_'), written.replace(/\[(Space|Ctrl)\]/g, '_'))), `step ${n + 1} '${s.id}': its line is up as written, each key on a cap`, `"${R.line}" keys [${R.keys}]`);
    log(`step ${n + 1} '${s.id}': "${R.line}" keys [${R.keys}] t ${R.t} at (${R.pos}) · men up ${R.foes}`);
    await hand[s.id](s);
    for (let k = 0; k < 6; k++) { R = await read(); if (R.id !== s.id) break; await run(60); }
    R = await read();
    check(R.id !== s.id && !R.down, `step ${n + 1} '${s.id}' is done by a hand's own keys`, `now ${R.done ? 'finished' : `'${R.id}'`} at t ${R.t} · at (${R.pos}) · men up ${R.foes} · ${R.down ? 'HE IS DOWN' : 'on his feet'} · ${R.sum}`);
    if (R.id === s.id || R.down) { break; }
  }
  await run(120); R = await read();
  log('end', JSON.stringify({ ...R, men: R.foes }));
  check(R.done && R.stage === 'done', 'every line done, he is out through the gate and the mission is over', `lesson ${R.done ? 'finished' : `on '${R.id}'`} · stage '${R.stage}' · ${R.t} s of play`);
  check(!errors.length, 'no page error and no console error', errors.length ? errors.slice(0, 6).join('\n    ') : 'none');
  await PAGE.end('');
})().catch(e => { console.error(e); process.exit(1); });
