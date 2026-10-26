// THE TUTORIAL'S FIRST MISSION ON THE REAL PAGE, WITH REAL KEYS: levels/tut-road.json played from its first line to the debrief in headless Chromium on the
// software WebGPU adapter (tools/qa/tut-page.cjs's recipe: the page is ?paused and stepped by __runFrames with nothing drawn, every input a DOM event
// from Playwright's keyboard and mouse, read by the page's own listeners).
// What the headless lesson run (tools/qa/lessons/tut-road.ts) cannot see: that the bundle boots the level and its script, that each line is up as it is
// written with the player's own key on a key cap, that the view and the trigger are locked against real events until their lines, that the two cards come
// up and close, and that a hand at the keyboard and the mouse gets through every step: walking by the keys the picture gives (W is up the SCREEN, whatever
// way the view was turned), the cursor put on a man where the page draws him.
//   PORT=5247 node tools/qa/tut-road-page.cjs        (a dev server on that port: PORT=5247 bun run tools/serve.ts)
//   env: QUALITY (default verylow) · OUT (a folder for the pictures; default a temp folder) · SHOTS=0 (no pictures: each is about a minute here) ·
//        PICS=boot,sprint,… (pictures at these steps only) · STEPS (stop after this many)
// exit 0 = every line passes · 1 = a FAIL or a page error. No GPU: the pictures are the software adapter's, at the lowest quality.
const { chromium } = require('/opt/node-tools/node_modules/playwright');
const fs = require('fs'), os = require('os'), path = require('path');
const PORT = process.env.PORT || 5199, QUALITY = process.env.QUALITY || 'verylow', ROOT = path.resolve(__dirname, '../..');
const OUT = process.env.OUT || fs.mkdtempSync(path.join(os.tmpdir(), 'tut-road-page-')), STEPS = +(process.env.STEPS || 99), PICS = process.env.PICS ? process.env.PICS.split(',') : null;
const FLAGS = ['--enable-unsafe-webgpu', '--use-gl=angle', '--use-angle=swiftshader', '--enable-features=Vulkan', '--use-vulkan=swiftshader', '--ignore-gpu-blocklist', '--disable-gpu-watchdog'];
const NOISE = /""?smoke[A-Za-z0-9_]*""?|\[pipeline\] smoke_|number of storage textures \(5\) in the Compute stage/;
const T0 = Date.now(), log = (...a) => console.log(`[${((Date.now() - T0) / 1000).toFixed(0)}s]`, ...a);
let failed = false;
const same = (drawn, written) => drawn.replace(/\s+/g, ' ').trim() === written.replace(/[\[\]]/g, '').replace(/\s+/g, ' ').trim();   // (the line as drawn carries its key on a cap, not in brackets)
const check = (ok, label, detail) => { if (!ok) failed = true; console.log(`- ${ok ? 'PASS' : 'FAIL'} — ${label}\n    ${detail}`); };
/** a key cap's words → the key's code */
const codeOf = cap => cap === 'Space' ? 'Space' : cap === 'Ctrl' ? 'ControlLeft' : cap === 'Shift' ? 'ShiftLeft' : cap === 'Tab' ? 'Tab' : /^[0-9]$/.test(cap) ? 'Digit' + cap : /^[A-Z]$/.test(cap) ? 'Key' + cap : cap;

(async () => {
  const data = JSON.parse(fs.readFileSync(path.join(ROOT, 'levels', 'tut-road.json'), 'utf8')), steps = data.scenarios.lesson.steps, step = id => steps.find(s => s.id === id);
  const browser = await chromium.launch({ headless: true, channel: 'chromium', args: FLAGS });
  const page = await (await browser.newContext({ viewport: { width: 1280, height: 720 }, deviceScaleFactor: 1 })).newPage(); const errors = [];
  page.setDefaultTimeout(0);
  page.on('console', m => { if (m.type() === 'error' && !NOISE.test(m.text()) && !/Failed to load resource/.test(m.text())) errors.push(m.text().slice(0, 300)); });
  page.on('pageerror', x => errors.push('pageerror: ' + String(x && x.stack || x).slice(0, 400)));
  const run = n => page.evaluate(n => window.__runFrames(n, 1 / 60, [], 'none'), n);   // (stepped, never drawn: a drawn frame is half a minute here)
  const read = () => page.evaluate(() => { const g = window.__game, el = document.getElementById('railprompt'), P = g.player, card = document.getElementById('railcard'), up = x => x.state !== 'dead';
    return { i: g.lesson.i, id: g.lesson.step?.id ?? null, done: g.lesson.finished, line: el?.querySelector('.line')?.textContent ?? '', keys: [...(el?.querySelectorAll('.line kbd') ?? [])].map(k => k.textContent), up: !!el && getComputedStyle(el).display !== 'none',
      fatal: document.getElementById('fatal')?.textContent || '', rails: g.rails.live, sum: g.rails.summary(), t: +g.time.toFixed(1), pos: [+P.char.pos[0].toFixed(2), +P.char.pos[2].toFixed(2)], down: !!P.down, slot: P.slot, away: !!P.holstered, mode: P.rifle.mode, rounds: P.rifle.roundsReady,
      wn: P.wall ? [P.wall.n[0], P.wall.n[2]] : null, wall: P.wall ? `${P.wall.kind} lean ${(P.wall.lean ?? 0).toFixed(2)}${P.wall.atEdge ? ' edge' : ''}` : '', crouch: !!P.crouch, yaw: +g.seat.cam.yaw.toFixed(3), stage: g.mission.stage, debrief: !!g.mission.debrief,
      men: g.guards.map(x => ({ r: x.route?.name ?? '', up: up(x), p: [x.char.pos[0], 1.25, x.char.pos[2]] })), foes: g.guards.filter(up).length,
      kit: `${P.canisters}/${P.flashbangs}/${P.frags}/${P.rifle.shells}/${P.rifle.rings}`, hut: (d => d ? { shut: d.isClosed(), locked: d.locked } : null)(g.doors.byName('hut')),
      card: { on: !!card && card.classList.contains('on'), h: card?.querySelector('h2')?.textContent ?? '', p: [...(card?.querySelectorAll('p') ?? [])].map(x => x.textContent).join(' / ') } }; });
  const at = p => page.evaluate(p => { const c = document.getElementById('view'), r = c.getBoundingClientRect(), [x, y, vis] = window.__cam.project(p, r.width, r.height, 1); return { x: r.left + x, y: r.top + y, vis: !!vis && x > 8 && y > 8 && x < r.width - 8 && y < r.height - 8 }; }, p);
  // A PICTURE (tut-page.cjs): the frame drawn and read back in the page, stood under the HUD as a still, and the page taken with that
  const shot = async tag => { if (process.env.SHOTS === '0' || (PICS && !/^stuck/.test(tag) && !PICS.includes(String(tag).replace(/^\d+-/, '')))) return '(no picture asked)'; const f = path.join(OUT, `tut-road-${tag}.png`);
    try { await page.evaluate(async () => { const dev = window.__engine.device, y = () => new Promise(res => { const ch = new MessageChannel(); ch.port1.onmessage = () => res(); ch.port2.postMessage(0); });
      for (let i = 0; i < 2; i++) { window.__runFrames(1, 1 / 600); await dev.queue.onSubmittedWorkDone(); await y(); }
      window.__engine.render(window.__cam, window.__game.time, 1 / 60, false); const fr = await window.__engine.readbackFrame(); if (!fr) return;
      if (fr.bgra) for (let i = 0; i < fr.data.length; i += 4) { const b = fr.data[i]; fr.data[i] = fr.data[i + 2]; fr.data[i + 2] = b; }
      for (let i = 3; i < fr.data.length; i += 4) fr.data[i] = 255;
      const c2 = document.createElement('canvas'); c2.width = fr.w; c2.height = fr.h; c2.getContext('2d').putImageData(new ImageData(new Uint8ClampedArray(fr.data.buffer), fr.w, fr.h), 0, 0);
      const v = document.getElementById('view'), r = v.getBoundingClientRect(); let im = document.getElementById('__still'); if (!im) { im = document.createElement('img'); im.id = '__still'; v.parentNode.insertBefore(im, v); }
      im.style.cssText = `position:fixed;left:${r.left}px;top:${r.top}px;width:${r.width}px;height:${r.height}px;z-index:0;`; im.src = c2.toDataURL('image/png'); await im.decode(); v.style.visibility = 'hidden'; }); }
    catch (e) { return `(no picture: the frame was not read back: ${String(e).slice(0, 80)})`; }   // (the software adapter loses the read now and then: a picture is never the check)
    try { await page.screenshot({ path: f, timeout: 60000 }); } catch (e) { return `(no picture: ${String(e).slice(0, 60)})`; }
    finally { await page.evaluate(() => { document.getElementById('view').style.visibility = ''; document.getElementById('__still')?.remove(); }); }
    return f; };
  const tap = async k => { await page.keyboard.down(k); await run(2); await page.keyboard.up(k); await run(2); };
  /** the next point of the way there that is more than a pace off (the guards' own path search: a hand walks round a truck, it does not push at it) */
  const next = p => page.evaluate(p => { const g = window.__game, P = g.player.char.pos, nav = g.col.nav, q = nav.isBlocked(p[0], p[2]) ? (nav.nearestFreePoint(p[0], p[2]) ?? p) : p;
    const way = nav.findPath([P[0], 0, P[2]], [q[0], 0, q[2]], 8000) ?? []; for (const w of way) if (Math.hypot(w[0] - P[0], w[2] - P[2]) > 0.45) return [w[0], 0, w[2]]; return [p[0], 0, p[2]]; }, p);
  /** walk to a place by the keys the PICTURE gives: W is up the screen. Eight frames a look; `keys` held with them (Shift); `raw`: straight at it (into a wall) */
  const walk = async (p, o = {}) => { const near = o.near ?? 0.35, held = new Set(); let R = null;
    const set = async want => { for (const k of [...held]) if (!want.includes(k)) { await page.keyboard.up(k); held.delete(k); } for (const k of want) if (!held.has(k)) { await page.keyboard.down(k); held.add(k); } };
    try { for (let n = 0; n < (o.max ?? 15) * 7.5; n++) { R = await read(); if (R.down || Math.hypot(R.pos[0] - p[0], R.pos[1] - p[2]) < near || (o.stop && o.stop(R))) break;
        const a = await at([R.pos[0], 0, R.pos[1]]), b = await at(o.raw ? p : await next(p)), dx = b.x - a.x, dy = b.y - a.y, m = Math.max(Math.abs(dx), Math.abs(dy)), want = [...(o.keys ?? [])];
        if (dx > 0.4 * m) want.push('KeyD'); if (dx < -0.4 * m) want.push('KeyA'); if (dy < -0.4 * m) want.push('KeyW'); if (dy > 0.4 * m) want.push('KeyS');
        await set(want); await run(8); } } finally { await set([]); await run(2); }
    return await read(); };
  const route = async (pts, o) => { let R = null; for (const p of pts) R = await walk(p, o); return R; };
  /** off a wall the way the lesson says: the move keys, square away from it */
  const offWall = async () => { const R = await read(); if (!R.wn) return R; return await walk([R.pos[0] + R.wn[0] * 2, 0, R.pos[1] + R.wn[1] * 2], { raw: true, max: 2, stop: q => !q.wall }); };
  /** the cursor on a man where the page draws him */
  const onMan = async r => { const R = await read(), m = R.men.find(x => x.r === r && x.up); if (!m) return false; const s = await at(m.p); await page.mouse.move(s.x, s.y); await run(2); return true; };
  /** the cursor on him and the left button held, until `stop` */
  const shoot = async (r, secs, stop) => { if (!await onMan(r)) return; await page.mouse.down(); try { for (let n = 0; n < secs * 10; n++) { await onMan(r); await run(6); if (stop(await read())) break; } } finally { await page.mouse.up(); await run(3); } };
  const until = async (id, n) => { for (let k = 0; k < n; k++) { await run(60); if ((await read()).id !== id) break; } };

  const url = `http://localhost:${PORT}/?levelFile=tut-road.json&quiet&paused&quality=${QUALITY}&mute`;
  log(url);
  await page.goto(url, { timeout: 0 });
  await page.evaluate(async () => { await window.__ready; window.__testVisible = true; window.__runFrames(3, 1 / 60, [], 'none'); });
  let R = await read();
  log('booted', JSON.stringify({ ...R, men: R.men.length }));
  check(!R.fatal && R.rails && R.up && R.i === 0 && same(R.line, steps[0].line) && R.foes === 6, 'the page boots the mission on rails: its first line is up as written, six men on the floor',
    `fatal "${R.fatal}" · rails ${R.rails} (${R.sum}) · line "${R.line}" keys [${R.keys}] · men ${R.foes}`);
  check(R.kit === '0/0/0/0/0', 'he carries the rifle and nothing to throw or fire from under it', `smoke/stun/frags/40 mm/sponge ${R.kit}`);
  log('picture', await shot('0-boot'));

  // ---- locked: before their lines, a hand on the view, the rifle's key, the trigger, the crouch and a throw does nothing
  { const y0 = R.yaw; await page.keyboard.down('ControlLeft'); for (let x = 400; x <= 800; x += 100) { await page.mouse.move(x, 320); await run(3); } await page.keyboard.up('ControlLeft');
    await page.keyboard.down('Space'); for (let x = 800; x >= 400; x -= 100) { await page.mouse.move(x, 320); await run(3); } await page.keyboard.up('Space');
    for (const k of ['Digit2', 'KeyC', 'Digit3', 'KeyR']) await tap(k);
    await page.mouse.move(700, 300); await page.mouse.down(); await run(4); await page.mouse.up(); await run(4); R = await read();
    check(Math.abs(R.yaw - y0) < 1e-3 && R.away && !R.crouch && R.rounds >= 30 && R.i === 0, 'before anything is taught the view does not turn, and the rifle\'s key, the crouch, a throw and a click do nothing',
      `the view's yaw ${y0} → ${R.yaw} · rifle ${R.away ? 'slung' : 'in his hands'} · crouched ${R.crouch} · rounds ${R.rounds} · step ${R.i} · ${R.sum}`); }

  const hand = {
    move: async () => { await route([[3.4, 0, 17.2], [3.1, 0, 12.6]], { near: 0.5 }); },
    // (the view's key is the one the line's own cap names)
    turn: async () => { const R0 = await read(), k = codeOf(R0.keys[0] || 'Ctrl'); await page.keyboard.down(k); for (let x = 400; x <= 900 && (await read()).id === 'turn'; x += 25) { await page.mouse.move(x, 320); await run(3); } await page.keyboard.up(k);
      const R1 = await read(); check(Math.abs(R1.yaw - R0.yaw) > 0.5, 'the view turns with the key on the line\'s cap held and the mouse moved', `cap [${R0.keys}] → ${k} · the view's yaw ${R0.yaw} → ${R1.yaw}`);
      // (and back as it opened, by the harness and not by a hand: the pictures after this are the level as a player first sees it)
      for (let n = 0; n < 6; n++) { const y = (await read()).yaw; if (Math.abs(y - R0.yaw) < 0.01) break; await page.evaluate(d => window.__cam.rotate(d), R0.yaw - y); await run(6); }
      const back = (await read()).yaw; if (Math.abs(back - R0.yaw) > 0.05) log(`  (the view is at ${back}, it opened at ${R0.yaw})`); },
    ready: async () => { await tap('Digit2'); await run(40); },
    sprint: async () => { await route([[3.2, 0, 11.1]], { near: 0.5 }); await route([[10.35, 0, 11.9]], { keys: ['ShiftLeft'], near: 0.7 }); },
    wall: async () => { await walk([11.6, 0, 11.9], { raw: true, max: 3, stop: R => !!R.wall }); },
    peek: async () => { await walk([10.35, 0, 14.05], { raw: true, max: 4, near: 0.05, stop: R => !R.wall || / edge/.test(R.wall) }); for (let n = 0; n < 30 && (await read()).id === 'peek'; n++) { await onMan('sentry'); await run(6); } },
    fire: async () => { const y = await read(); check(/lean [0-9.]+/.test(y.wall) && parseFloat(y.wall.split('lean ')[1]) > 0.3, 'at the gateway\'s corner with the cursor on the sentry he leans out', `wall '${y.wall}' at (${y.pos})`);
      for (let n = 0; n < 8 && (await read()).men[0].up; n++) { await onMan('sentry'); await page.mouse.down(); await run(3); await page.mouse.up(); await run(20); } },
    auto: async () => { await tap('Digit2'); await run(10); await tap('Digit1'); await run(10); },
    off: async () => { const R0 = await read(), R1 = await offWall(); check(!!R0.wall && !R1.wall, 'the move keys held square away from the wall take him off it', `'${R0.wall}' at (${R0.pos}) → '${R1.wall}' at (${R1.pos})`); },
    inner: async () => { await route([[10.0, 0, 14.6], [12.2, 0, 15.2], [15.2, 0, 12.75]], { near: 0.5 }); },
    door: async () => { const s = await at([17, 1.1, 12.75]); await page.mouse.move(s.x, s.y); await run(3); const r0 = (await read()).rounds;
      await page.mouse.down(); for (let n = 0; n < 60; n++) { await run(6); const q = await read(); if (!q.hut.shut || q.id !== 'door') break; } await page.mouse.up(); await run(3);
      const q = await read(); check(!q.hut.shut && q.foes === 5, 'the trigger held on the hut\'s door takes it off its hinges, and the man behind the boards is not hit', `the door ${q.hut.shut ? 'STANDS' : 'is out'} after ${r0 - q.rounds} round(s) on '${q.mode}' · men up ${q.foes}`); await until('door', 4); },
    reload: async () => { await tap('KeyR'); await until('reload', 8); },
    thin: async () => { await walk([16.3, 0, 12.9], { near: 0.3 }); await run(150);   // (he comes out of his nook to the boards as the line comes up)
      const q0 = await read(); await shoot('hut', 8, q => !q.men[1].up);
      const q = await read(); check(!q.men[1].up && !q.down, 'from the doorway, the cursor on the man behind the boards and the trigger held puts him down through them', `he is ${q.men[1].up ? 'UP' : 'down'} after ${q0.rounds - q.rounds} round(s) · the player ${q.down ? 'DOWN' : 'up'} at (${q.pos})`); await until('thin', 4); },
    arch: async () => { await route([[18.0, 0, 12.8], [20.8, 0, 12.9], [21.25, 0, 14.3], [21.5, 0, 14.75]], { near: 0.4 }); await until('arch', 3); },
    crouch: async () => { await run(60); await tap('KeyC'); },
    low: async () => { await route([[21.6, 0, 16.7], [26.7, 0, 16.7]], { near: 0.5 }); },
    cover: async () => { await route([[27.9, 0, 16.4]], { near: 0.4 }); await tap('KeyC'); await route([[28.6, 0, 16.2]], { near: 0.3 }); await walk([28.6, 0, 15.0], { raw: true, max: 3, stop: R => !!R.wall }); },
    gunner: async () => { await walk([30.2, 0, 15.9], { raw: true, max: 4, near: 0.05, stop: R => !R.wall || / edge/.test(R.wall) }); for (let n = 0; n < 20; n++) { await onMan('gunner'); await run(6); }
      const y = await read(); check(!y.down && /lean/.test(y.wall), 'at the end of the roadblock\'s wall with the cursor on the gunner he leans out, and the gun has not found him', `wall '${y.wall}' at (${y.pos}) · ${y.down ? 'DOWN' : 'up'}`);
      await shoot('gunner', 8, q => !q.men[2].up); await until('gunner', 4); },
    watch: async () => { await until('watch', 12); },
    last: async () => { for (let n = 0; n < 12; n++) { const q = await read(); const m = q.men.find((x, k) => k >= 3 && x.up); if (!m || q.down) break; await shoot(m.r, 4, z => !z.men.find(x => x.r === m.r).up || z.down); } await until('last', 4); },
    radio: async () => { await offWall(); await route([[30.4, 0, 16.5], [33.9, 0, 14.0], [34.0, 0, 11.6], [32.7, 0, 11.4]], { near: 0.4 }); },
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
    if (s.explain) { await run(30); const C = (await read()).card; log('picture', await shot(`60${n + 1}-card`)); await page.waitForTimeout(450); await page.keyboard.press('Enter'); await run(10); const D = (await read()).card; cardsSeen.push(s.id);
      check(C.on && C.h === s.explain.title && !!C.p && !D.on, `step ${n + 1} '${s.id}' opens on its card, and Enter closes it`, `"${C.h}": ${C.p} · after Enter the card is ${D.on ? 'UP' : 'gone'}`); R = await read(); }
    const written = s.setupLine ?? s.line;
    check(R.up && (same(R.line, written) || same(R.line.replace(/\b(Space|Ctrl|E)\b/g, '_'), written.replace(/\[(Space|Ctrl)\]/g, '_'))), `step ${n + 1} '${s.id}': its line is up as written, each key on a cap`, `"${R.line}" keys [${R.keys}]`);
    const f = await shot(`${n + 1}-${s.id}`);
    log(`step ${n + 1} '${s.id}': "${R.line}" keys [${R.keys}] t ${R.t} at (${R.pos}) · men up ${R.foes} · ${f}`);
    await hand[s.id](s);
    for (let k = 0; k < 6; k++) { R = await read(); if (R.id !== s.id) break; await run(60); }
    R = await read();
    check(R.id !== s.id && !R.down, `step ${n + 1} '${s.id}' is done by a hand's own keys`, `now ${R.done ? 'finished' : `'${R.id}'`} at t ${R.t} · at (${R.pos}) · men up ${R.foes} · ${R.down ? 'HE IS DOWN' : 'on his feet'} · ${R.sum}`);
    if (R.id === s.id || R.down) { log('stuck picture', await shot(`stuck-${s.id}`)); break; }
  }
  await run(120); R = await read();
  log('end', JSON.stringify({ ...R, men: R.foes }));
  check(R.done && R.stage === 'done', 'every line done, he is out through the gate and the mission is over', `lesson ${R.done ? 'finished' : `on '${R.id}'`} · stage '${R.stage}' · ${R.t} s of play`);
  log('picture', await shot('9-end'));
  check(!errors.length, 'no page error and no console error', errors.length ? errors.slice(0, 6).join('\n    ') : 'none');
  console.log(`pictures: ${OUT}`);
  await browser.close();
  process.exit(failed ? 1 : 0);
})().catch(e => { console.error(e); process.exit(1); });
