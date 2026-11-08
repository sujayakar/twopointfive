// A RUN ON THE PAGE THAT DRAWS IS KEPT AND PLAYED AGAIN: draw.html on the software adapter, stepped by hand. A run with a save, a load and a restart is played, F7 keeps it, the page of it walks it to the same state,
// is sought both ways, its camera taken off and given back, its pace turned, a note kept on it. Never a GPU's judgement.
//   PORT=<a dev server> node tools/draw/tape-page.cjs [--out dir]      exit 0 = every check passed, 1 = a FAIL, 2 = the script died
const { chromium } = (() => { try { return require('playwright'); } catch { return require('/opt/node-tools/node_modules/playwright'); } })();
const { mkdirSync } = require('node:fs');
const PORT = process.env.PORT || 5199, at = process.argv.indexOf('--out'), OUT = at > 0 ? process.argv[at + 1] : null;
const FLAGS = ['--enable-unsafe-webgpu', '--use-gl=angle', '--use-angle=swiftshader', '--enable-features=Vulkan', '--use-vulkan=swiftshader', '--ignore-gpu-blocklist', '--disable-gpu-watchdog', '--no-sandbox'];
let fails = 0;
const check = (ok, what, got) => { console.log(`- ${ok ? 'PASS' : 'FAIL'} — ${what}${got === undefined ? '' : `   (${JSON.stringify(got)})`}`); if (!ok) fails++; };
(async () => {
  const browser = await chromium.launch({ headless: true, executablePath: process.env.CHROMIUM || '/usr/bin/chromium', args: FLAGS }), ctx = await browser.newContext({ viewport: { width: 1280, height: 720 }, deviceScaleFactor: 1 });
  const open = async (words, ready = true) => {
    const page = await ctx.newPage(), errors = []; page.setDefaultTimeout(600000);
    page.on('pageerror', x => errors.push(String(x && x.stack || x).slice(0, 500)));
    page.on('console', m => { const t = m.text(); if (m.type() === 'error' || /Invalid|error:/.test(t)) errors.push(t.slice(0, 300)); });
    await page.goto(`http://localhost:${PORT}/draw.html?${words}`, { timeout: 300000 });
    if (ready) { await page.waitForFunction(() => !!window.__ready && !!window.__draw, null, { timeout: 600000 }); await page.evaluate(() => window.__ready); }
    return { page, errors };
  };
  const still = async (page, name) => { if (OUT) { mkdirSync(OUT, { recursive: true }); await page.evaluate(() => window.__draw.engine.device.queue.onSubmittedWorkDone()); await page.screenshot({ path: `${OUT}/${name}.png` }); } };
  // ---- the run
  const R = await open('paused&quality=verylow&mute&seed=7&roomclear&squad4&freshstart=0');
  const run = await R.page.evaluate(async () => {
    const d = window.__draw, I = d.input, go = (n, keys) => d.run(n, keys, 'none'); go(60);
    go(90, ['KeyW']); I.pressed.add('F5'); go(2); I.mouseX = 900; I.mouseY = 300; I.buttons |= 1; I.clicked |= 1; go(30); I.buttons = 0; go(200, ['KeyD']);
    d.game.commands.load('manual.0'); go(60); d.game.commands.restart(''); go(120, ['KeyW']); I.pressed.add('KeyC'); go(100, ['KeyA']); d.run(2, [], 'last');
    window.dispatchEvent(new KeyboardEvent('keydown', { code: 'F7', key: 'F7', cancelable: true, bubbles: true })); await new Promise(r => setTimeout(r, 1500));
    const T = d.game.tape, m = d.game.mark(true), db = await new Promise((res, rej) => { const q = indexedDB.open('twopointsix-kept'); q.onsuccess = () => res(q.result); q.onerror = rej; });
    const rows = await new Promise(res => { const q = db.transaction('visits').objectStore('visits').getAll(); q.onsuccess = () => res(q.result); });
    return { ticks: T && T.ticks, began: T && T.began, kb: T && Math.round(T.length / 1024), m, rows: rows.map(r => ({ sid: r.sid, seconds: r.seconds, hands: r.hands, scenario: r.scenario, pieces: r.pieces })), said: document.getElementById('hud').textContent };
  });
  check(run.ticks === 664 && run.rows.length === 1 && run.rows[0].hands === true && Math.abs(run.rows[0].seconds - 664 / 60) < 0.2, 'a run is kept as it is played, and F7 says so', run);
  check(/replay is saved/.test(run.said), 'the player is told what F7 did', run.said);
  check(run.began > 400 && run.began < 450, 'the tape says where the last try began', run.began);
  check(R.errors.length === 0, 'the run: nothing threw', R.errors.slice(0, 4));
  await R.page.close();
  const sid = run.rows[0] && run.rows[0].sid;
  // ---- its page
  const W = await open(`paused&quality=verylow&mute&playtape=${sid}`);
  const s0 = await W.page.evaluate(() => { const d = window.__draw, P = d.played; d.run(0); return { at: P.at, total: P.total, tries: P.tries, paused: P.paused, big: !!document.getElementById('tapebig'), bar: !!document.getElementById('tapebar'), search: location.search, tape: !!d.game.tape }; });
  check(s0.at === 0 && s0.total === 664 && s0.tries.length === 1 && s0.paused && s0.big && s0.bar && /roomclear/.test(s0.search) && !s0.tape, 'the run\'s page stands on its first tick, under a play button, at the run\'s own address, and tapes nothing', s0);
  const s1 = await W.page.evaluate(() => { const d = window.__draw, P = d.played; d.run(30, [], 'last'); const c = d.cam.target.slice(); d.run(P.total, [], 'none'); return { at: P.at, parted: P.parted, lost: P.lost, m: d.game.mark(true), c }; });
  check(s1.at === 664 && s1.parted === null && s1.lost === null && s1.m.die === run.m.die && s1.m.parts === run.m.parts && s1.m.state === run.m.state, 'it walks to its end, and the whole state there is the run\'s', { walked: s1, played: run.m });
  await still(W.page, 'tape-1-end');
  const s2 = await W.page.evaluate(async () => {
    const d = window.__draw, P = d.played, line = document.querySelector('#tapebar .line'), r = line.getBoundingClientRect(), out = {};
    line.dispatchEvent(new MouseEvent('click', { clientX: r.left + r.width * 0.25, clientY: r.top + 5, bubbles: true })); for (let i = 0; i < 400 && P.seeking; i++) d.run(1, [], 'none'); out.quarter = P.at;
    window.dispatchEvent(new KeyboardEvent('keydown', { code: 'ArrowLeft', key: 'ArrowLeft', bubbles: true, cancelable: true })); for (let i = 0; i < 400 && P.seeking; i++) d.run(1, [], 'none'); out.back = P.at;
    const key = (code, k) => window.dispatchEvent(new KeyboardEvent('keydown', { code, key: k, bubbles: true, cancelable: true }));
    const b = [...document.querySelectorAll('#tapebar .seg button')]; b[0].click(); b[0].click(); out.slow = P.pace; key('Equal', '='); key('Equal', '='); key('Equal', '='); out.two = P.pace; b[1].click(); out.one = P.pace;
    d.run(P.total, [], 'none'); out.end = P.at; out.parted = P.parted; out.m = d.game.mark(true); return out; });
  check(Math.abs(s2.quarter - 166) < 4 && s2.back < 12 && s2.slow === 0.25 && s2.two === 2 && s2.one === 1, 'the line and the keys seek it, the bar turns its pace', s2);
  check(s2.end === 664 && s2.parted === null && s2.m.state === run.m.state, 'sought back and walked on, it ends as it did', s2.m);
  const s3 = await W.page.evaluate(() => {
    const d = window.__draw, P = d.played, cv = document.getElementById('view'), was = d.cam.target.slice(), ev = (t, x, y, b) => cv.dispatchEvent(new PointerEvent(t, { clientX: x, clientY: y, button: 0, buttons: b, pointerId: 1, bubbles: true }));
    P.seek(300); for (let i = 0; i < 400 && P.seeking; i++) d.run(1, [], 'none'); d.run(1, [], 'none'); const a = d.cam.target.slice();
    ev('pointerdown', 600, 360, 1); ev('pointermove', 700, 400, 1); ev('pointerup', 700, 400, 0); d.run(1, [], 'none'); const b = d.cam.target.slice(); d.run(60, [], 'none'); const c = d.cam.target.slice();
    window.dispatchEvent(new KeyboardEvent('keydown', { code: 'KeyC', key: 'c', bubbles: true })); d.run(1, [], 'none'); const e = d.cam.target.slice(); d.run(P.total, [], 'none'); return { a, b, c, e, at: P.at, m: d.game.mark(true) }; });
  const far = (p, q) => Math.hypot(p[0] - q[0], p[2] - q[2]);
  check(far(s3.a, s3.b) > 0.5 && far(s3.b, s3.c) < 0.01 && far(s3.c, s3.e) > 0.3 && s3.m.state === run.m.state, 'a drag takes the camera off the run\'s view and C gives it back, and the run is the same run', { a: s3.a, b: s3.b, c: s3.c, e: s3.e });
  const s4 = await W.page.evaluate(async () => {
    const key = (code, k, t = window) => t.dispatchEvent(new KeyboardEvent('keydown', { code, key: k, bubbles: true, cancelable: true })), P = window.__draw.played; P.seek(120); for (let i = 0; i < 400 && P.seeking; i++) window.__draw.run(1, [], 'none');
    key('KeyN', 'n'); const f = document.querySelector('#tapenote input'), up = !document.getElementById('tapenote').hidden; f.value = 'he stops here'; key('Enter', 'Enter', f); await new Promise(r => setTimeout(r, 800));
    const db = await new Promise((res, rej) => { const q = indexedDB.open('twopointsix-kept'); q.onsuccess = () => res(q.result); q.onerror = rej; }), rows = await new Promise(res => { const q = db.transaction('visits').objectStore('visits').getAll(); q.onsuccess = () => res(q.result); });
    return { up, notes: rows[0].notes, ticks: document.querySelectorAll('#tapebar .tk').length, rows: rows.length }; });
  check(s4.up && s4.notes && s4.notes.length === 1 && s4.notes[0].text === 'he stops here' && Math.abs(s4.notes[0].n - 120) < 3 && s4.ticks === 1 && s4.rows === 1, 'a note is kept on the run, and stands on the line', s4);
  check(W.errors.length === 0, 'the run\'s page: nothing threw and no GPU error was said', W.errors.slice(0, 4));
  await still(W.page, 'tape-2-note');
  await W.page.close();
  // ---- from the try that is on
  const V = await open(`paused&quality=verylow&mute&playtape=${sid}&at=${run.began}`);
  const s5 = await V.page.evaluate(() => { const d = window.__draw, P = d.played; for (let i = 0; i < 600 && P.seeking; i++) d.run(1, [], 'none'); const at = P.at; d.run(P.total, [], 'none'); return { at, m: d.game.mark(true), notes: document.querySelectorAll('#tapebar .tk').length }; });
  check(Math.abs(s5.at - run.began) < 3 && s5.m.state === run.m.state && s5.notes === 1, 'opened at its last try it stands there with its note, and ends the same', s5);
  await V.page.close();
  // ---- lost to a panic on the way, it is gone back on
  { const L = await open(`paused&quality=verylow&mute&playtape=${sid}`);
    const s = await L.page.evaluate(() => { const d = window.__draw, P = d.played, G = d.game; d.run(400, [], 'none'); try { G.X.face_debug_panic(G.h, 0); } catch { /* lost */ } const lost = !!G.lost; d.run(5, [], 'none'); const stood = P.at;
      P.seek(200); for (let i = 0; i < 600 && P.seeking; i++) d.run(1, [], 'none'); const back = P.at, up = !G.lost; d.run(P.total, [], 'none'); return { lost, stood, back, up, card: !!document.querySelector('#fatal[style*="flex"]'), m: G.mark(true) }; });
    check(s.lost && s.stood === 400 && Math.abs(s.back - 200) < 3 && s.up && !s.card && s.m.state === run.m.state, 'lost to a panic, it stands, is sought back and ends the same', s); await L.page.close(); }
  // ---- live: a person's page, its own frames
  { const small = await browser.newContext({ viewport: { width: 320, height: 180 }, deviceScaleFactor: 1, storageState: await ctx.storageState({ indexedDB: true }) }), page = await small.newPage(), errors = []; page.setDefaultTimeout(600000); page.on('pageerror', x => errors.push(String(x).slice(0, 300)));
    await page.goto(`http://localhost:${PORT}/draw.html?quality=verylow&mute&playtape=${sid}`, { timeout: 300000 }); await page.waitForFunction(() => !!window.__ready && !!window.__draw && !!document.getElementById('tapebig'), null, { timeout: 600000 }); await page.evaluate(() => window.__ready);
    const still0 = await page.evaluate(() => window.__draw.played.at); await page.evaluate(() => document.getElementById('tapebig').click());
    const went = await page.waitForFunction(() => window.__draw.played.at >= 12, null, { timeout: 400000 }).then(() => true).catch(() => false), s = await page.evaluate(() => { const P = window.__draw.played; return { at: P.at, paused: P.paused, parted: P.parted, big: !!document.getElementById('tapebig'), frames: window.__draw.told.frames }; });
    check(still0 === 0 && went && !s.paused && s.parted === null && !s.big && errors.length === 0, 'on a person\'s page it waits under the play button, and plays by the page\'s own frames', { still0, ...s, errors: errors.slice(0, 3) }); await small.close(); }
  // ---- refused in words
  const N = await open('paused&quality=verylow&mute&playtape=20200101T000000-none', false); await N.page.waitForFunction(() => /saved in this browser/.test(document.body.textContent), null, { timeout: 300000 }).then(() => check(true, 'a replay that is gone is refused in words')).catch(() => check(false, 'a replay that is gone is refused in words'));
  await browser.close();
  console.log(fails ? `=== tape page: ${fails} FAILED ===` : '=== tape page: ALL PASS ===');
  process.exit(fails ? 1 : 0);
})().catch(e => { console.error(e); process.exit(2); });
