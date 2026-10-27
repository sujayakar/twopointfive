// THE SQUAD'S TUTORIAL MISSION ON THE GROUND, ON THE REAL PAGE, WITH REAL KEYS: headless Chromium on the software WebGPU adapter (tools/qa/tut-page.cjs's
// recipe — SwiftShader, no GPU; the page is ?paused and stepped by __runFrames with nothing drawn, every input a DOM event from Playwright's keyboard and mouse).
// What the headless lesson run cannot see: that the bundle boots the level and its script on the frame (game/rails.ts) with him on the floor and no seat
// above it, that the one big line is up with the page's own keys on key caps, that the squad's key and the go key are locked against a real key until their
// steps, that a hand gets through all twelve steps (the wheel on the view, the squad's key on the ground, on three doors and on himself, the go key, his
// own feet through the gate, into the tea room, across the yard and back to the trucks), that the two cards come up and close, and what the road does on a
// page that has smoke: how many of the roadblock are still up when the gate's step comes, and whether a round finds him.
//   PORT=5231 node tools/qa/tut-team-page.cjs        (a dev server on that port: PORT=5231 bun run tools/serve.ts)
//   env: QUALITY (default verylow) · OUT (a folder for the pictures; default a temp folder) · SHOTS=0 (no pictures: each is about a minute here)
// exit 0 = every line passes · 1 = a FAIL or a page error
const { chromium } = require('/opt/node-tools/node_modules/playwright');
const fs = require('fs'), os = require('os'), path = require('path');
const PORT = process.env.PORT || 5199, QUALITY = process.env.QUALITY || 'verylow', ROOT = path.resolve(__dirname, '../..');
const OUT = process.env.OUT || fs.mkdtempSync(path.join(os.tmpdir(), 'tut-team-page-'));
const FLAGS = ['--enable-unsafe-webgpu', '--use-gl=angle', '--use-angle=swiftshader', '--enable-features=Vulkan', '--use-vulkan=swiftshader', '--ignore-gpu-blocklist', '--disable-gpu-watchdog'];
const NOISE = /""?smoke[A-Za-z0-9_]*""?|\[pipeline\] smoke_|number of storage textures \(5\) in the Compute stage/;
// (his walks, corner by corner, as the walk grid has them: along the wall and through the gate · in by the tea room's door · across the yard south of the
//  plough lorry · back out by the gate to the trucks)
const TO_GATE = [[6.25, 0, 21.25], [8.75, 0, 20.75], [13.75, 0, 20.75], [13.75, 0, 17.75], [12.25, 0, 17.75], [10.4, 0, 18.6]];
const TO_TEA = [[11.75, 0, 17.75], [11.75, 0, 16.25], [10.75, 0, 15.25], [9.25, 0, 12.25], [9.25, 0, 10.25], [10.25, 0, 8.75]];
// (out of the tea room by the leg he came in on: its door's leaf stands open into the room, and a straight line at the doorway from the east ends behind it)
const TO_BUNK = [[9.25, 0, 10.25], [9.25, 0, 12.25], [19.75, 0, 17.75], [26.25, 0, 17.5]];
const TO_TRUCKS = [[19.75, 0, 17.75], [15.25, 0, 17.75], [13.75, 0, 20.75], [6.75, 0, 20.75], [5.25, 0, 22.75]];
const T0 = Date.now(), log = (...a) => console.log(`[${((Date.now() - T0) / 1000).toFixed(0)}s]`, ...a);
let failed = false;
const check = (ok, label, detail) => { if (!ok) failed = true; console.log(`- ${ok ? 'PASS' : 'FAIL'} — ${label}\n    ${detail}`); };

(async () => {
  const browser = await chromium.launch({ headless: true, channel: 'chromium', args: FLAGS });
  const page = await (await browser.newContext({ viewport: { width: 1280, height: 720 }, deviceScaleFactor: 1 })).newPage(); const errors = [];
  page.setDefaultTimeout(0);
  page.on('console', m => { if (m.type() === 'error' && !NOISE.test(m.text()) && !/Failed to load resource/.test(m.text())) errors.push(m.text().slice(0, 300)); });
  page.on('pageerror', x => errors.push('pageerror: ' + String(x && x.stack || x).slice(0, 400)));
  const run = n => page.evaluate(n => window.__runFrames(n, 1 / 60, [], 'none'), n);   // (stepped, never drawn: a drawn frame is half a minute here)
  const read = () => page.evaluate(() => { const g = window.__game, el = document.getElementById('railprompt'), men = g.guards.filter(x => x.team === 'operative' && x.state !== 'dead');
    return { i: g.lesson.i, id: g.lesson.step?.id ?? null, done: !!g.lesson.finished, line: el?.querySelector('.line')?.textContent ?? '', keys: [...(el?.querySelectorAll('kbd') ?? [])].map(k => k.textContent), up: !!el && getComputedStyle(el).display !== 'none',
      row: document.getElementById('objective')?.textContent ?? '', fatal: document.getElementById('fatal')?.textContent || '', rails: g.rails.live, sum: g.rails.summary(), script: !!g.director.running, sky: document.body.classList.contains('skyseat'),
      at: [+g.player.char.pos[0].toFixed(1), +g.player.char.pos[2].toFixed(1)], down: !!g.player.down, hits: g.player.hitsLeft, mates: men.length, men: g.guards.filter(x => x.team === 'hostile' && x.state !== 'dead').length,
      block: g.guards.filter(x => /^Tango (6|7|8)$/.test(x.callsign) && x.state !== 'dead').length, cans: g.items.items.filter(i => i.kind === 'smoke').length,
      set: men.length > 0 && men.every(x => g.orders.of(x)?.arrived), follow: men.length > 0 && men.every(x => g.orders.of(x)?.order === 'fallIn'), knees: men.filter(x => g.rails.isSpared(x)).length,
      orders: g.orders.log.length, last: g.orders.log.length ? `${g.orders.log[g.orders.log.length - 1].order} ${g.orders.log[g.orders.log.length - 1].target}` : '', menu: !!window.__orders?.menu, t: +g.time.toFixed(1),
      basis: (() => { try { const b = window.__cam.planarBasis(); return { f: [b.fwd[0], b.fwd[2]], r: [b.right[0], b.right[2]] }; } catch { return null; } })(), yaw: (() => { try { return +window.__cam.yaw.toFixed(2); } catch { return null; } })() }; });
  // (a picture, as tut-page.cjs takes one: the frame is drawn and read back in the page and stood under the HUD as a still)
  const shot = async tag => { if (process.env.SHOTS === '0') return '(no picture asked)'; const f = path.join(OUT, `tut-team-${tag}.png`);
    await page.evaluate(async () => { const dev = window.__engine.device, y = () => new Promise(res => { const ch = new MessageChannel(); ch.port1.onmessage = () => res(); ch.port2.postMessage(0); });
      for (let i = 0; i < 2; i++) { window.__runFrames(1, 1 / 600); await dev.queue.onSubmittedWorkDone(); await y(); }
      window.__engine.render(window.__cam, window.__game.time, 1 / 60, false); const fr = await window.__engine.readbackFrame(); if (!fr) return;
      if (fr.bgra) for (let i = 0; i < fr.data.length; i += 4) { const b = fr.data[i]; fr.data[i] = fr.data[i + 2]; fr.data[i + 2] = b; }
      for (let i = 3; i < fr.data.length; i += 4) fr.data[i] = 255;
      const c2 = document.createElement('canvas'); c2.width = fr.w; c2.height = fr.h; c2.getContext('2d').putImageData(new ImageData(new Uint8ClampedArray(fr.data.buffer), fr.w, fr.h), 0, 0);
      const v = document.getElementById('view'), r = v.getBoundingClientRect(); let im = document.getElementById('__still'); if (!im) { im = document.createElement('img'); im.id = '__still'; v.parentNode.insertBefore(im, v); }
      im.style.cssText = `position:fixed;left:${r.left}px;top:${r.top}px;width:${r.width}px;height:${r.height}px;z-index:0;`; im.src = c2.toDataURL('image/png'); await im.decode(); v.style.visibility = 'hidden'; });
    try { await page.screenshot({ path: f, timeout: 60000 }); } catch (e) { return `(no picture: ${String(e).slice(0, 60)})`; }
    finally { await page.evaluate(() => { document.getElementById('view').style.visibility = ''; document.getElementById('__still')?.remove(); }); }
    return f; };
  const point = async w => { const s = await page.evaluate(p => { const c = document.getElementById('view'), r = c.getBoundingClientRect(), [x, y] = window.__cam.project(p, r.width, r.height, 1); return { x: r.left + x, y: r.top + y }; }, w); await page.mouse.move(s.x, s.y); await run(2); };
  const tap = async k => { await page.keyboard.down(k); await run(3); await page.keyboard.up(k); await run(3); };
  // A CARD (a step's `explain`, ui/railcard.ts): read and closed with Enter, as a hand does it (a key closes one only once it has been up a moment). Every
  // card met is kept by its title: the last lines hold the list to the level's own
  const cards = [];
  const card = async () => { const c = await page.evaluate(() => { const e = document.getElementById('railcard'); return e && e.classList.contains('on') ? { h: e.querySelector('h2')?.textContent || '', p: [...e.querySelectorAll('p')].map(x => x.textContent).join(' / ') } : null; });
    if (!c) return null; await page.waitForTimeout(450); await page.keyboard.press('Enter'); await run(2);
    const still = await page.evaluate(() => !!document.getElementById('railcard')?.classList.contains('on')); cards.push({ ...c, closed: !still }); return c; };
  const until = async (want, secs) => { for (let k = 0; k < secs; k++) { await card(); const s = await read(); if (want(s)) return s; await run(60); } await card(); return read(); };
  // (walked with the walk keys held, steered off where the page says he is. The view may be turned, so which key is which way is MEASURED: a short stride on D
  //  and one on S give the two directions the keys move him in, and each stride after is the pair of keys whose sum points most nearly at the mark. Where
  //  the page hands out its camera the two directions are read from it afresh before every stride)
  let AX = null;
  const stride = async (keys, n) => { for (const k of keys) await page.keyboard.down(k); await run(n); for (const k of keys) await page.keyboard.up(k); await run(1); };
  const axes = async () => { const a = await read(); await stride(['KeyD'], 10); const b = await read(); await stride(['KeyS'], 10); const c = await read();
    const n = v => { const l = Math.hypot(v[0], v[1]) || 1; return [v[0] / l, v[1] / l]; }; AX = { D: n([b.at[0] - a.at[0], b.at[1] - a.at[1]]), S: n([c.at[0] - b.at[0], c.at[1] - b.at[1]]) }; };
  //  (a stride that got him nowhere — a teammate in a doorway, a jamb — is answered as a hand would: a short step to one side, the other side next time)
  const walk = async (to, secs) => { if (!AX) await axes(); let was = Infinity, side = 1;
    for (let k = 0; k < secs * 4; k++) { const s = await read(), dx = to[0] - s.at[0], dz = to[2] - s.at[1], d = Math.hypot(dx, dz); if (d < 0.7) return true;
      if (s.basis) AX = { D: s.basis.r, S: [-s.basis.f[0], -s.basis.f[1]] };   // (the view as it stands NOW: it can have turned since the strides were measured, and a stale pair of axes walks him into a wall)
      const pd = (dx * AX.D[0] + dz * AX.D[1]) / d, ps = (dx * AX.S[0] + dz * AX.S[1]) / d, keys = [];
      if (was - d < 0.08) { await stride(Math.abs(pd) > Math.abs(ps) ? [side > 0 ? 'KeyS' : 'KeyW'] : [side > 0 ? 'KeyD' : 'KeyA'], 14); side = -side; was = Infinity; continue; }
      was = d;
      if (Math.abs(pd) > 0.3) keys.push(pd > 0 ? 'KeyD' : 'KeyA'); if (Math.abs(ps) > 0.3) keys.push(ps > 0 ? 'KeyS' : 'KeyW'); await stride(keys, Math.max(4, Math.min(40, Math.round(d * 14)))); }
    return false; };

  const steps = JSON.parse(fs.readFileSync(path.join(ROOT, 'levels/tut-team.json'), 'utf8')).scenarios.lesson.steps, step = id => steps.find(s => s.id === id);
  const along = async (way, secs) => { for (const p of way) if (!(await walk(p, secs))) return false; return true; };
  const url = `http://localhost:${PORT}/?levelFile=tut-team.json&roomclear=1&squad4&paused&quality=${QUALITY}&mute&nocutboxes`;
  log(url);
  await page.goto(url, { waitUntil: 'commit' });
  await page.waitForFunction(() => window.__ready !== undefined, null, { timeout: 0 });   // (the bundle has run: the boot's own promise is there to wait on)
  const row = await page.evaluate(async () => { await window.__ready; window.__testVisible = true; window.__runFrames(3, 1 / 60, [], 'none'); const e = window.__scenarios.current()?.entry ?? window.__scenarios.current(); return e ? `${e.id} · ${e.group} ▸ ${e.title}` : null; });
  let R = await read(); log('booted', JSON.stringify(R));
  check(/^tut-team · tutorial/.test(row || ''), "the page knows this URL as the Tutorial group's row", String(row));
  check(!R.fatal && R.rails && R.up && R.i === 0 && R.id === 'zoom' && R.mates === 4 && R.script && !R.sky && Math.hypot(R.at[0] - 4.55, R.at[1] - 22.5) < 0.6, 'it boots on rails with him on the floor behind the north truck: one big line up, the four, its script running, no seat above',
    `fatal "${R.fatal}" · ${R.sum} · line "${R.line}" keys [${R.keys}] · small row "${R.row}" · he at ${R.at} · ${R.mates} teammates · ${R.men} defenders · script ${R.script} · sky seat ${R.sky}`);
  check(R.keys.length === 1 && /^(Space|Ctrl)$/.test(R.keys[0]) && !/\[/.test(R.line), "the line's key is drawn as a key cap with the key the page has the view on", `"${R.line}" keys [${R.keys}]`);
  log('picture', await shot('1-boot'));

  // ---- locked: the squad's key, the go key and the ring do nothing under a real key before their steps
  { const b = await read(); await page.mouse.move(640, 300);
    for (const k of ['KeyQ', 'KeyF', 'KeyX']) await tap(k);
    await page.keyboard.down('KeyQ'); await run(30); await page.keyboard.up('KeyQ'); await run(3);
    const a = await read(); check(a.orders === b.orders && !a.menu && a.i === 0, "before their steps the squad's key (tapped and kept down) and the go key do nothing", `orders ${b.orders} → ${a.orders} · ring up ${a.menu} · ${a.sum}`); }

  // ---- the road: the zoom, the smoke, the squad's key on the yard
  { const k = R.keys[0] === 'Space' ? 'Space' : 'ControlLeft'; await page.keyboard.down(k); for (let n = 0; n < 8; n++) { await page.mouse.wheel(0, 120); await run(4); } await page.keyboard.up(k); await run(4); }
  R = await until(x => x.i >= 1, 4); check(R.i >= 1, 'the view key held and the wheel: zoomed, and the smoke\'s line is up', `step ${R.i + 1} '${R.id}' · "${R.line}"`);
  R = await until(x => x.i >= 2, 10); check(R.i === 2 && R.id === 'move' && R.cans >= 2 && R.keys.join() === 'Q' && R.hits > 0 && !R.down, "the squad's two cans are out and the line asks for the squad's key on the yard, its cap the page's (Q); he is on his feet", `step ${R.i + 1} '${R.id}' at ${R.t} s · cans ${R.cans} · "${R.line}" keys [${R.keys}] · hits left ${R.hits}`);
  await run(120);   // (two seconds of reading, the smoke building)
  await point(step('move').at); await tap('KeyQ');
  { const a = await until(x => x.i >= 3 || /using the smoke/.test(x.line), 3); check(a.orders >= 1 && /^move /.test(a.last) && (a.i >= 3 || !a.keys.length), "the squad's key on the yard gives the move, and the line says what they are doing with no key on it", `last order "${a.last}" · step ${a.i + 1} · "${a.line}" keys [${a.keys}]`); }
  R = await until(x => x.i >= 3, 60); check(R.i === 3 && R.id === 'with', 'the four are in the yard and the line asks him to go through the gate with them', `step ${R.i + 1} '${R.id}' at ${R.t} s · "${R.line}" · roadblock still up ${R.block} · on a knee ${R.knees}`);
  const blockAtMove = R.block;
  const through = await along(TO_GATE, 30); R = await until(x => x.i >= 4, 6);
  check(through && R.i >= 4 && !R.down, 'walked along the wall and through the gate: he is in the yard on his feet and the gate\'s line is up (or already gone by)', `walked ${through} · he at ${R.at} · step ${R.i + 1} '${R.id}' · "${R.line}" · hits left ${R.hits}`);
  log(`THE ROAD ON A PAGE WITH SMOKE: roadblock up when the four were in the yard ${blockAtMove} of 3 · when he was through the gate ${R.block} of 3`);
  log('picture', await shot('2-gate'));

  // ---- the gate: the roadblock comes on, and the squad holds it. A round that puts him down is answered as a hand would: Enter, back to the gate's checkpoint
  let downs = 0;
  for (let k = 0; k < 90; k++) { R = await read(); await card(); if (R.i >= 5) break;
    if (R.down) { downs++; log(`  he is down at ${R.at} (step '${R.id}'): Enter`); await page.keyboard.press('Enter'); await run(30); R = await read(); log('  put back', JSON.stringify({ i: R.i, id: R.id, at: R.at, script: R.script, block: R.block, down: R.down })); if (downs >= 2) { await page.evaluate(() => { window.__game.godMode = true; }); log('  twice down: rounds no longer drop him for the rest of this run (said in the last line)'); } }
    await run(60); }
  check(R.i >= 5 && R.block === 0 && R.script, 'the roadblock is down and the first door\'s line is up, the script still running', `step ${R.i + 1} '${R.id}' at ${R.t} s · roadblock up ${R.block} · he went down ${downs}× · "${R.line}"`);
  check(downs < 2, 'he was not put down twice at the gate (he stands inside the wall beside it and never fires: a gate that drops such a hand twice is too much for a first mission)', `he went down ${downs}×${downs >= 2 ? ' · the rest of this run is played with rounds no longer dropping him, to read the later steps' : ''}`);

  // ---- indoors: the squad's key on a door, the go key; twice
  R = await until(x => x.id === 'stack' && x.up, 4);
  await point(step('stack').at); await tap('KeyQ');
  { const a = await until(x => x.i >= 6 || /stacking/.test(x.line), 3); check(/^stack door:tea/.test(a.last) && (a.i >= 6 || !a.keys.length), "the squad's key on the tea room's door: they stack, and the line says so with no key on it", `last order "${a.last}" · step ${a.i + 1} · "${a.line}" keys [${a.keys}]`); }
  R = await until(x => x.i >= 6, 60); check(R.i === 6 && R.keys.join() === 'F', 'they stand on the door and the line asks for the go key, its cap the page\'s (F)', `step ${R.i + 1} '${R.id}' at ${R.t} s · "${R.line}" keys [${R.keys}]`);
  // (no second tap of the squad's key on the door they stand on: the go key alone is what takes the room here, so this line is the go key's)
  R = await until(x => x.i >= 6 && x.set, 20); await tap('KeyF');
  R = await until(x => x.i >= 7, 60); check(R.i >= 7, 'the go key: the tea room is theirs', `step ${R.i + 1} '${R.id}' at ${R.t} s · "${R.line}" · defenders up ${R.men}`);
  const inTea = await along(TO_TEA, 30); R = await read();
  await point(step('kitchen').at); await tap('KeyQ'); R = await until(x => x.set || x.i >= 8, 30); await run(60); await tap('KeyF');
  R = await until(x => x.i >= 8, 60); check(inTea && R.i >= 8, 'walked into the tea room, the squad\'s key on the kitchen\'s door and the go key: the kitchen is theirs', `walked ${inTea} · step ${R.i + 1} '${R.id}' at ${R.t} s · "${R.line}" · defenders up ${R.men}`);

  // ---- follow me: the squad's key on himself, and across the yard
  R = await until(x => x.id === 'fallin' && x.up, 4);
  { const me = await read(); await point([me.at[0], 1.0, me.at[1]]); await tap('KeyQ'); const a = await until(x => x.follow || x.i >= 9, 3);
    check(/^fallIn/.test(a.last) && a.follow, "the squad's key on himself: all four fall in", `last order "${a.last}" · following ${a.follow} · step ${a.i + 1} '${a.id}' · "${a.line}"`); }
  R = await until(x => x.i >= 9, 10);
  log(`  (the view's yaw at the lead: ${(await read()).yaw})`);
  const toBunk = await along(TO_BUNK, 40); R = await until(x => x.i >= 10, 40);
  check(toBunk && R.i >= 10, 'led across the yard: he and three of them stand by the last building and the line asks for its door', `walked ${toBunk} · he at ${R.at} · step ${R.i + 1} '${R.id}' at ${R.t} s · "${R.line}"`);
  log('picture', await shot('3-bunk'));
  for (let n = 0; n < 3 && (await read()).id === 'bunk'; n++) { await point(step('bunk').at); await tap('KeyQ'); R = await until(x => x.set || x.i >= 11, 30); await run(60); await tap('KeyF'); R = await until(x => x.i >= 11, 40); }
  check(R.i >= 11, 'the two keys on the last door: the bunkhouse is theirs', `step ${R.i + 1} '${R.id}' at ${R.t} s · "${R.line}" · defenders up ${R.men}`);

  // ---- back to the trucks: the squad's key on himself once more, and his feet
  // (beside a doorway the cursor on himself can be on the door as drawn, and the tap is then that door's: what a hand does is step off it and tap again)
  { let me = await read(); await point([me.at[0], 1.0, me.at[1]]); await tap('KeyQ'); let a = await until(x => x.follow, 3);
    if (!a.follow) { log(`  (the squad's key on himself at ${me.at} gave "${a.last}": he steps off the door and taps again)`); await walk([me.at[0] - 2.2, 0, me.at[1] + 0.6], 8); me = await read(); await point([me.at[0], 1.0, me.at[1]]); await tap('KeyQ'); a = await until(x => x.follow, 3); }
    check(/^fallIn/.test(a.last) && a.follow, "the squad's key on himself once more: all four fall in", `last order "${a.last}" · following ${a.follow} · he at ${a.at}`); }
  const back = await along(TO_TRUCKS, 50); R = await until(x => x.done, 60);
  check(back && R.done && !R.fatal, 'called to him again and walked back to the trucks: every step is done and the mission ends', `walked ${back} · he at ${R.at} · finished ${R.done} at ${R.t} s · he went down ${downs}× in all`);
  { const end = await page.evaluate(() => { const e = document.querySelector('#verdict, .verdict, #debrief'); return e ? e.textContent.replace(/\s+/g, ' ').slice(0, 200) : ''; }); log('  the end card:', end || '(none found by this run\'s selectors)'); }
  check(['Doors', 'Follow me'].every(h => cards.some(c => c.h === h && c.closed && c.p.length > 40)) && cards.length === 2, 'the two cards were each up once with their words and closed', cards.map(c => `"${c.h}": ${c.p}${c.closed ? '' : ' (STILL UP)'}`).join(' · ') || 'none');
  check(!errors.length, 'no page error and no console error', errors.length ? errors.slice(0, 6).join('\n    ') : 'none');
  console.log(`pictures: ${OUT}`);
  await browser.close();
  console.log(failed ? '=== tut-team-page: FAILED ===' : '=== tut-team-page: ALL PASS (software adapter: never seen on a GPU) ===');
  process.exit(failed ? 1 : 0);
})().catch(e => { console.error(e); process.exit(1); });
