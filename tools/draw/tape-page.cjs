// A RUN ON THE PAGE THAT DRAWS IS KEPT AND PLAYED AGAIN: the page on the software adapter, stepped by hand. A run with a save, a load and a restart is played, F7 keeps it, the page of it walks it to the same state,
// is sought both ways, its camera taken off and given back, its pace turned, a note kept on it. Never a GPU's judgement.
//   PORT=<a dev server> node tools/draw/tape-page.cjs [--out dir]      exit 0 = every check passed, 1 = a FAIL, 2 = the script died
const PAGE = require('../qa/page.cjs').page({ name: 'tape-page', detail: 'json' }), { check } = PAGE;
const { mkdirSync } = require('node:fs');
const at = process.argv.indexOf('--out'), OUT = at > 0 ? process.argv[at + 1] : null;
// (a page that plays a run has the game keep what a viewer's own picture is made from: its whole state is every such walk's, the run's own in its die and its parts)
const same = (a, b) => a.die === b.die && a.parts === b.parts, whole = (a, b) => same(a, b) && a.state === b.state;
(async () => {
  const ctx = await PAGE.context();
  const open = async (words, ready = true) => {
    const { page, errors, go } = await PAGE.open({ context: ctx, timeout: 600000, errors: { also: /Invalid|error:/ } });
    await go(words, { timeout: 300000 });
    if (ready) { await page.waitForFunction(() => !!window.__ready && !!window.__draw, null, { timeout: 600000 }); await page.evaluate(() => window.__ready); }
    return { page, errors };
  };
  const still = async (page, name) => { if (OUT) { mkdirSync(OUT, { recursive: true }); await page.evaluate(() => window.__draw.engine.device.queue.onSubmittedWorkDone()); await page.screenshot({ path: `${OUT}/${name}.png` }); } };
  // ---- the run
  const R = await open('paused&quality=verylow&mute&seed=7&roomclear&squad4');
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
  const s0 = await W.page.evaluate(() => { const d = window.__draw, P = d.played; d.run(0); return { at: P.at, total: P.total, tries: P.tries, paused: P.paused, big: !!document.getElementById('tapebig'), bar: !!document.getElementById('tapebar'), said: document.querySelector('#tapebar .say').textContent, search: location.search, tape: !!d.game.tape }; });
  check(s0.at === 0 && s0.total === 664 && s0.tries.length === 1 && s0.paused && s0.big && s0.bar && /roomclear/.test(s0.search) && !s0.tape, 'the run\'s page stands on its first tick, under a play button, at the run\'s own address, and tapes nothing', s0);
  const s1 = await W.page.evaluate(() => { const d = window.__draw, P = d.played; d.run(30, [], 'last'); const c = d.cam.target.slice(); d.run(P.total, [], 'none'); return { at: P.at, parted: P.parted, lost: P.lost, m: d.game.mark(true), c }; });
  check(s1.at === 664 && s1.parted === null && s1.lost === null && same(s1.m, run.m), 'it walks to its end, and the die and the parts there are the run\'s', { walked: s1, played: run.m });
  await still(W.page, 'tape-1-end');
  const s2 = await W.page.evaluate(async () => {
    const d = window.__draw, P = d.played, line = document.querySelector('#tapebar .line'), r = line.getBoundingClientRect(), out = {};
    line.dispatchEvent(new MouseEvent('click', { clientX: r.left + r.width * 0.25, clientY: r.top + 5, bubbles: true })); for (let i = 0; i < 400 && P.seeking; i++) d.run(1, [], 'none'); out.quarter = P.at;
    window.dispatchEvent(new KeyboardEvent('keydown', { code: 'ArrowLeft', key: 'ArrowLeft', bubbles: true, cancelable: true })); for (let i = 0; i < 400 && P.seeking; i++) d.run(1, [], 'none'); out.back = P.at;
    const key = (code, k) => window.dispatchEvent(new KeyboardEvent('keydown', { code, key: k, bubbles: true, cancelable: true }));
    const b = [...document.querySelectorAll('#tapebar .seg button')]; b[0].click(); b[0].click(); out.slow = P.pace; key('Equal', '='); key('Equal', '='); key('Equal', '='); out.two = P.pace; b[1].click(); out.one = P.pace;
    d.run(P.total, [], 'none'); out.end = P.at; out.parted = P.parted; out.m = d.game.mark(true); return out; });
  check(Math.abs(s2.quarter - 166) < 4 && s2.back < 12 && s2.slow === 0.25 && s2.two === 2 && s2.one === 1, 'the line and the keys seek it, the bar turns its pace', s2);
  check(s2.end === 664 && s2.parted === null && whole(s2.m, s1.m), 'sought back and walked on, it ends as it did', s2.m);
  const s3 = await W.page.evaluate(() => {
    const d = window.__draw, P = d.played, cv = document.getElementById('view'), was = d.cam.target.slice(), ev = (t, x, y, b) => cv.dispatchEvent(new PointerEvent(t, { clientX: x, clientY: y, button: 0, buttons: b, pointerId: 1, bubbles: true }));
    P.seek(300); for (let i = 0; i < 400 && P.seeking; i++) d.run(1, [], 'none'); d.run(1, [], 'none'); const a = d.cam.target.slice();
    ev('pointerdown', 600, 360, 1); ev('pointermove', 700, 400, 1); ev('pointerup', 700, 400, 0); d.run(1, [], 'none'); const b = d.cam.target.slice(); d.run(60, [], 'none'); const c = d.cam.target.slice();
    window.dispatchEvent(new KeyboardEvent('keydown', { code: 'KeyC', key: 'c', bubbles: true })); d.run(1, [], 'none'); const e = d.cam.target.slice(); d.run(P.total, [], 'none'); return { a, b, c, e, at: P.at, m: d.game.mark(true) }; });
  const far = (p, q) => Math.hypot(p[0] - q[0], p[2] - q[2]);
  check(far(s3.a, s3.b) > 0.5 && far(s3.b, s3.c) < 0.01 && far(s3.c, s3.e) > 0.3 && whole(s3.m, s1.m), 'a drag takes the camera off the run\'s view and C gives it back, and the run is the same run', { a: s3.a, b: s3.b, c: s3.c, e: s3.e });
  // ---- how whoever watches sees it
  const s6 = await W.page.evaluate(async () => {
    const d = window.__draw, P = d.played, key = (code, k) => window.dispatchEvent(new KeyboardEvent('keydown', { code, key: k, bubbles: true, cancelable: true })), B = t => [...document.querySelectorAll('#tapebar button')].find(b => b.textContent === t), on = t => B(t).classList.contains('on'), seen = () => d.game.drawn.n[15], out = {};
    P.seek(400); for (let i = 0; i < 400 && P.seeking; i++) d.run(1, [], 'none'); d.run(1, [], 'none'); out.men = seen(); out.fog = [B('Fog').hidden, on('Fog')]; key('KeyF', 'f'); d.run(1, [], 'none'); out.bare = seen(); out.off = [B('Fog').hidden, on('Fog'), d.watched.fog, d.hud.page.bared];
    out.nv0 = [on('NV'), d.watched.tube, d.engine.settings.nvAmount]; key('KeyV', 'v'); d.run(30, [], 'none'); out.nv1 = [on('NV'), d.watched.tube, d.engine.settings.nvAmount]; key('KeyV', 'v'); d.run(30, [], 'none'); out.nv2 = [on('NV'), d.watched.tube, d.engine.settings.nvAmount];
    key('KeyV', 'v'); P.seek(100); for (let i = 0; i < 400 && P.seeking; i++) d.run(1, [], 'none'); d.run(1, [], 'none'); out.kept = [d.watched.fog, d.watched.tube, seen()]; d.run(P.total, [], 'none'); out.m = d.game.mark(true); key('KeyV', 'v'); key('KeyF', 'f'); d.run(1, [], 'none'); out.back = [d.watched.fog, d.watched.tube];
    const ink = cv => { const x = cv.getContext('2d').getImageData(0, 0, cv.width, cv.height).data; let n = 0; for (let i = 3; i < x.length; i += 4) if (x[i]) n++; return n; }, M = d.game.map(), small = document.getElementById('tapemap'); await new Promise(r => setTimeout(r, 150)); d.run(0, [], 'none'); d.run(1, [], 'none'); out.map = { ink: ink(small), w: small.clientWidth, men: M.men.length, doors: M.doors.length };
    key('KeyM', 'm'); await new Promise(r => setTimeout(r, 150)); d.run(1, [], 'none'); const big = document.querySelector('#tapemapbig canvas'); out.large = { up: !document.getElementById('tapemapbig').hidden, ink: ink(big) }; const was = d.cam.target.slice(), r = big.getBoundingClientRect();
    big.dispatchEvent(new MouseEvent('click', { clientX: r.left + r.width * 0.2, clientY: r.top + r.height * 0.2, bubbles: true })); d.run(1, [], 'none'); out.went = Math.hypot(d.cam.target[0] - was[0], d.cam.target[2] - was[2]); out.shut = document.getElementById('tapemapbig').hidden; key('KeyC', 'c'); d.run(1, [], 'none'); return out; });
  check(!s6.fog[0] && s6.fog[1] && s6.bare > s6.men && !s6.off[0] && !s6.off[1] && !s6.off[2] && s6.off[3], 'the fog comes off the picture, and everybody is drawn', s6);
  check(!s6.nv0[0] && s6.nv1[0] && s6.nv1[1] === true && s6.nv1[2] > 0.9 && !s6.nv2[0] && s6.nv2[1] === null && s6.nv2[2] < 0.1, 'the picture is put through a tube and taken out of it again');
  check(s6.kept[0] === false && s6.kept[1] === true && whole(s6.m, s1.m) && s6.back[0] === true && s6.back[1] === null, 'both are kept over a seek, and the run is the same run');
  check(s6.map.ink > 500 && s6.map.men > 4 && s6.large.up && s6.large.ink > s6.map.ink && s6.went > 2 && s6.shut, 'a map of it is in the corner, opens large on M, and a press of it sends the camera there');
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
  check(Math.abs(s5.at - run.began) < 3 && whole(s5.m, s1.m) && s5.notes === 1, 'opened at its last try it stands there with its note, and ends the same', s5);
  await V.page.close();
  // ---- lost to a panic on the way, it is gone back on
  { const L = await open(`paused&quality=verylow&mute&playtape=${sid}`);
    const s = await L.page.evaluate(() => { const d = window.__draw, P = d.played, G = d.game; d.run(400, [], 'none'); try { G.X.face_debug_panic(G.h, 0); } catch { /* lost */ } const lost = !!G.lost; d.run(5, [], 'none'); const stood = P.at;
      P.seek(200); for (let i = 0; i < 600 && P.seeking; i++) d.run(1, [], 'none'); const back = P.at, up = !G.lost; d.run(P.total, [], 'none'); return { lost, stood, back, up, card: !!document.querySelector('#fatal[style*="flex"]'), m: G.mark(true) }; });
    check(s.lost && s.stood === 400 && Math.abs(s.back - 200) < 3 && s.up && !s.card && whole(s.m, s1.m), 'lost to a panic, it stands, is sought back and ends the same', s); await L.page.close(); }
  // ---- live: a person's page, its own frames
  { const { context: small, page, errors, go } = await PAGE.open({ size: [320, 180], options: { storageState: await ctx.storageState({ indexedDB: true }) }, timeout: 600000, errors: { console: false } });
    await go(`quality=verylow&mute&playtape=${sid}`, { timeout: 300000 }); await page.waitForFunction(() => !!window.__ready && !!window.__draw && !!document.getElementById('tapebig'), null, { timeout: 600000 }); await page.evaluate(() => window.__ready);
    const still0 = await page.evaluate(() => window.__draw.played.at); await page.evaluate(() => document.getElementById('tapebig').click());
    const went = await page.waitForFunction(() => window.__draw.played.at >= 12, null, { timeout: 400000 }).then(() => true).catch(() => false), s = await page.evaluate(() => { const P = window.__draw.played; return { at: P.at, paused: P.paused, parted: P.parted, big: !!document.getElementById('tapebig'), frames: window.__draw.told.frames }; });
    check(still0 === 0 && went && !s.paused && s.parted === null && !s.big && errors.length === 0, 'on a person\'s page it waits under the play button, and plays by the page\'s own frames', { still0, ...s, errors: errors.slice(0, 3) }); await small.close(); }
  // ---- the list, down the title's column
  { const T = await open('paused&quality=verylow&mute');
    const s = await T.page.evaluate(async () => { const at = q => [...document.querySelectorAll('#menu .actions .btn')].find(b => b.textContent.startsWith(q)), wait = ms => new Promise(r => setTimeout(r, ms)), out = {};
      at('Extras').click(); out.extras = !!at('Replays'); at('Replays').click(); await wait(1200); const a = document.querySelector('#menu .xlist a.xrow'); out.row = a && { href: a.getAttribute('href'), name: a.querySelector('b').textContent, line: a.querySelector('i').textContent }; out.note = document.querySelector('#menu .xnote').textContent; return out; }); await still(T.page, 'tape-3-list');
    const gone = await T.page.evaluate(async () => { const at = q => [...document.querySelectorAll('#menu .actions .btn')].find(b => b.textContent.startsWith(q)); window.dispatchEvent(new KeyboardEvent('keydown', { code: 'Escape', key: 'Escape', bubbles: true, cancelable: true })); await new Promise(r => setTimeout(r, 100)); return !!at('Play'); });
    check(s.extras && s.row && s.row.href === `?playtape=${sid}` && /Room clear/.test(s.row.name) && /1 note/.test(s.row.line) && gone, 'the title lists the runs kept, and Esc goes back', s); check(T.errors.length === 0, 'the title: nothing threw', T.errors.slice(0, 3)); await T.page.close(); }
  // ---- played on another version of the game: the list and the bar say so
  { const T = await open('paused&quality=verylow&mute');
    const was = await T.page.evaluate(async sid => { const db = await new Promise((res, rej) => { const q = indexedDB.open('twopointsix-kept'); q.onsuccess = () => res(q.result); q.onerror = rej; }); const tx = db.transaction('visits', 'readwrite'), st = tx.objectStore('visits'), r = await new Promise(res => { const q = st.get(sid); q.onsuccess = () => res(q.result); }); const m = r.module; r.module = 'another'; st.put(r); await new Promise(res => { tx.oncomplete = res; }); return m; }, sid);
    const s = await T.page.evaluate(async () => { const at = q => [...document.querySelectorAll('#menu .actions .btn')].find(b => b.textContent.startsWith(q)); at('Extras').click(); at('Replays').click(); await new Promise(r => setTimeout(r, 1200)); const a = document.querySelector('#menu .xlist a.xrow'); return { old: a.classList.contains('old'), line: a.querySelector('i').textContent }; });
    await still(T.page, 'tape-4-older'); await T.page.close();
    const U = await open(`paused&quality=verylow&mute&playtape=${sid}`), [b, parted] = await U.page.evaluate(() => { const d = window.__draw; d.run(1, [], 'none'); const said = document.querySelector('#tapebar .say').textContent; d.run(240, [], 'none'); return [said, d.played.parted]; }); await U.page.close();
    check(!!was && s.old && /older version/.test(s.line) && /older version of the game/.test(b) && !/older/.test(s0.said) && parted === null, 'a run of an older version of the game is said to be one, on the list and on the bar, and plays as it was', { was, ...s, bar: b, parted, own: s0.said }); }
  // ---- refused in words
  const N = await open('paused&quality=verylow&mute&playtape=20200101T000000-none', false); await N.page.waitForFunction(() => /saved in this browser/.test(document.body.textContent), null, { timeout: 300000 }).then(async () => check(await N.page.evaluate(() => { const c = document.getElementById('taperefuse'); return !!c && document.getElementById('fatal').style.display !== 'flex' && !/\n\s+at |\.ts:/.test(c.textContent) && c.querySelector('a').textContent === '\u2039 Main menu'; }), 'a replay that is gone is refused in words, with a way back')).catch(() => check(false, 'a replay that is gone is refused in words'));
  await PAGE.end(PAGE.fails ? `=== tape page: ${PAGE.fails} FAILED ===` : '=== tape page: ALL PASS ===');
})().catch(e => { console.error(e); process.exit(2); });
