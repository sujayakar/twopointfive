// EVERY RUN KEPT, ON THE REAL PAGE, AND ONE OF THEM HANDED OVER (src/replay/keeper.ts, keepstore.ts, keptui.ts; tools/qa/replaykeep-probes.ts holds the same
// headless): headless Chromium on the software WebGPU adapter (tools/qa/page-look.cjs's recipe), one browser profile, three page loads and the replay page —
//   1. a mission page is played with no key pressed: the page's own timer files its tape in this browser's storage, piece by piece
//   2. the page is LEFT for another mission (no beat due): what was played up to the last step is there afterwards, and its row says its page has gone
//      (keeper.ts leave: the list of replays does not call it 'being played'); the second visit is kept beside it, and is marked in its turn when its page
//      goes with nothing new to file. NOT SEEN HERE: the list's own words inside the minute — on this adapter a navigation takes minutes to commit
//   3. the start page, nothing played: keeps nothing. Its Extras has NO row that hands every kept replay over as one file, and the page no `__replay.dump`
//      (both taken out on 2026-10-27: a hosted build's backend reads the store by itself)
//   4. /replay lists both visits with no button that downloads them all, and opens each; no button downloads the open one either ('Download with notes'
//      went on 2026-10-27), and the visit as the page put it together is one whole tape with its run tape to tools/qa/replay-dump.ts (both visits: one's
//      last piece was written as its page went away)
//   PORT=5311 [NAV_MS=480000] node tools/qa/replaykeep-page.cjs [outDir]      (a dev server on that port: PORT=5311 bun run tools/serve.ts; FRAMES=240 = frames a stretch of play;
//   three boots of some minutes each on the software adapter)
// exit 0 = all of it · 1 = otherwise / a page error
const { chromium } = require('/opt/node-tools/node_modules/playwright');
const fs = require('node:fs'), os = require('node:os'), path = require('node:path'), cp = require('node:child_process');
const PORT = process.env.PORT || 5199, QUALITY = process.env.QUALITY || 'verylow', ROOT = path.resolve(__dirname, '../..');
const OUT = process.argv[2] || fs.mkdtempSync(path.join(os.tmpdir(), 'replaykeep-page-'));
const FLAGS = ['--enable-unsafe-webgpu', '--use-gl=angle', '--use-angle=swiftshader', '--enable-features=Vulkan', '--use-vulkan=swiftshader', '--ignore-gpu-blocklist', '--disable-gpu-watchdog'];
const NOISE = /""?smoke[A-Za-z0-9_]*""?|\[pipeline\] smoke_|number of storage textures \(5\) in the Compute stage/;
let failed = false; const check = (ok, text) => { console.log(`- ${ok ? 'PASS' : 'FAIL'} — ${text}`); if (!ok) failed = true; };
const sleep = ms => new Promise(r => setTimeout(r, ms)), FR = Math.max(30, Number(process.env.FRAMES) || 240);
(async () => {
  fs.mkdirSync(OUT, { recursive: true });
  const browser = await chromium.launch({ headless: true, channel: 'chromium', args: FLAGS });
  const ctx = await browser.newContext({ viewport: { width: 960, height: 540 }, deviceScaleFactor: 1, acceptDownloads: true });
  const page = await ctx.newPage(); const errors = [];
  page.on('console', m => { if (m.type() === 'error' && !NOISE.test(m.text()) && !/Failed to load resource/.test(m.text())) errors.push(m.text().slice(0, 300)); });
  page.on('pageerror', x => errors.push('pageerror: ' + String(x && x.stack || x).slice(0, 400)));
  // LEAVING A PAGE: the game draws a frame on every animation frame, stepped or not, and on this adapter a drawn frame is seconds of main thread — a page
  // with frames queued does not let a navigation commit for minutes (the walk died on a 240 s page load on a loaded box, a different load each time). So the
  // page being left stops drawing first and waits for what it had queued; the load itself is given NAV_MS (default 480 s) and says how long it took.
  const NAV = Math.max(60000, Number(process.env.NAV_MS) || 480000), went = [];
  const go = async (path) => {
    await page.evaluate(async () => { const e = window.__engine; if (!e) return; e.render = () => {}; try { if (window.__smoke) window.__smoke.params.enabled = false; await Promise.race([e.device.queue.onSubmittedWorkDone(), new Promise(r => setTimeout(r, 20000))]); } catch { /* */ } }).catch(() => {});
    const t0 = Date.now(); await page.goto(`http://localhost:${PORT}/${path}`, { timeout: NAV }); went.push(`${path.slice(0, 28)} ${((Date.now() - t0) / 1000).toFixed(0)} s`);
  };
  const boot = async (q) => { await go(`?${q}`); return page.evaluate(async () => { await window.__ready; return { fatal: document.getElementById('fatal')?.textContent || '', sid: window.__replay?.session, keeping: !!window.__replay?.keeper }; }); };
  /** play: frames stepped with W held (the page is ?paused, so nothing else steps it); → the buffer's step count and seconds of play */
  // (tools/qa/page-look.cjs's way: the renderer stubbed out and the smoke solver off while it plays — on this adapter a drawn squad frame takes seconds, and a
  //  page with drawn frames still queued does not let a navigation commit for minutes; `draw` then draws ONE frame for real and waits for it)
  const play = (frames) => page.evaluate(async (n) => { window.__menu.close(); window.__smoke.params.enabled = false; const e = window.__engine, real = e.render; e.render = () => {};
    try { await window.__runFrames(n, 1 / 60, ['KeyW']); } finally { e.render = real; } return { n: window.__replay.buf.n, secs: window.__game.time }; }, frames);
  const draw = () => page.evaluate(async () => { await window.__runFrames(1, 1 / 60); await window.__engine.device.queue.onSubmittedWorkDone(); });
  const kept = () => page.evaluate(async () => { const K = await window.__replay.kept(); return { visits: K.visits, words: K.words, rows: K.rows.map(r => ({ sid: r.sid, pieces: r.pieces, seconds: r.seconds, bytes: r.bytes, scenario: r.scenario, left: r.left === true })), broke: window.__replay.keeper?.broke ?? '', stats: window.__replay.keeper ? { ...window.__replay.keeper.stats } : null }; });

  // ---- 1. a mission page, played; the page's own timer does the filing
  const A = await boot(`paused&quality=${QUALITY}&mute&roomclear&squad4`);
  check(!A.fatal && A.keeping && !!A.sid, `visit one booted and is keeping its tape (${A.sid})${A.fatal ? ' · FATAL ' + A.fatal : ''}`);
  await play(FR); await draw(); await sleep(6500); await play(FR); await sleep(6500);
  const timer = await page.evaluate(() => ({ ...window.__replay.keeper.stats }));
  check(timer.beats >= 2 && timer.pieces >= 1 && timer.failed === 0, `nothing pressed: the page's timer has filed ${timer.pieces} piece(s) in ${timer.beats} beats (${(timer.bytes / 1024).toFixed(0)} KB; a beat ${(timer.ms / Math.max(1, timer.beats)).toFixed(1)} ms, worst ${timer.worstMs.toFixed(1)})`);
  // (a run's end is filed at once. The page is paused, so nothing moves but what is done here: a little play, then the run made over by the stealth mission's
  // stage word, put into an end by hand and put back — this line is the page's own wiring, timer to idle moment to store; a squad mission's real end, which
  // moves no stage word, is the probes' (replaykeep-probes.ts 'end, on a real mission'). The timer's own beat may land first and take the stretch: the end is
  // then filed with no new piece, which is why the store's row is read and not the count of beats)
  const before = await page.evaluate(() => window.__replay.keeper.filed.seconds);
  await play(30);
  const ended = await page.evaluate(async () => { const g = window.__game, K = window.__replay.keeper, was = g.mission.stage, e0 = K.stats.ends;
    await new Promise(r => setTimeout(r, 600));   // (the keeper has been told 'not over')
    const t0 = performance.now(); g.mission.stage = 'done'; while (K.stats.ends === e0 && performance.now() - t0 < 4000) await new Promise(r => setTimeout(r, 50));
    const ms = performance.now() - t0; g.mission.stage = was; await K.idle(); await new Promise(r => setTimeout(r, 600)); return { ends: K.stats.ends - e0, ms, seconds: K.filed.seconds, was }; });
  check(ended.ends === 1 && ended.ms < 2500 && ended.seconds > before + 0.3, `a run that is over is filed ${(ended.ms / 1000).toFixed(1)} s later, not at the next beat (${ended.ends} for the end; the store's row went from ${before} s to ${ended.seconds} s; the word was '${ended.was}')`);
  const pA = await play(Math.round(FR / 2));   // (a stretch no beat has seen: the page is left with it)

  // ---- 2. left for another mission
  const B = await boot(`paused&quality=${QUALITY}&mute&solo`);
  check(!B.fatal && B.keeping && B.sid !== A.sid, `visit two booted with a visit id of its own (${B.sid})`);
  const pB = await play(FR * 2); const K2 = await kept(), rA = K2.rows.find(r => r.sid === A.sid), rB = K2.rows.find(r => r.sid === B.sid);
  check(!!rA && rA.seconds >= pA.n / 60 - 0.4, `leaving the page lost nothing: visit one holds ${rA?.seconds} s of the ${(pA.n / 60).toFixed(1)} s stepped, in ${rA?.pieces} pieces`);
  check(!!rA && rA.left === true && !!rB && rB.left === false, `the page that was left said so as it went (visit one: left ${rA?.left}), the one being played has not (left ${rB?.left}): the list will not call visit one 'being played'`);
  check(K2.visits === 2 && !!rB && rB.pieces >= 1 && !K2.broke, `visit two is kept beside it (${K2.visits} visits: ${K2.words}${K2.broke ? ' · STOPPED: ' + K2.broke : ''})`);

  // ---- 3. the start page: keeps nothing by itself, and hands nothing over
  await go(`?paused&quality=${QUALITY}&mute`);   // (paused: an idle start page starts its attract reel, and on this adapter a drawn frame is seconds of main thread)
  const T = await page.evaluate(async () => { await window.__ready; return { sid: window.__replay.session, fatal: document.getElementById('fatal')?.textContent || '' }; });
  // (visit two had filed everything when its page went — kept() above — so its mark is ALL that page wrote as it went: one small write asked at pagehide)
  const K3 = await kept(), gA = K3.rows.find(r => r.sid === A.sid), gB = K3.rows.find(r => r.sid === B.sid);
  check(!!gA && gA.left === true && !!gB && gB.left === true && gB.pieces === rB?.pieces, `visit two, left with nothing new to file, is marked as left all the same (left ${gB?.left}, ${gB?.pieces} pieces as before), and visit one still is`);
  await sleep(3000);
  const ex = await page.evaluate(() => { const b = document.querySelector('#menu .xopen'); if (!b) return { err: 'no Extras button', rows: [], dump: '' }; b.click(); return { err: '', rows: [...document.querySelectorAll('#menu a.xrow')].map(a => a.textContent.trim()), dump: typeof window.__replay.dump }; });
  check(!T.fatal && !ex.err && ex.rows.some(t => /^Replays/.test(t)) && !ex.rows.some(t => /hand over/i.test(t)) && ex.dump === 'undefined', `the start page's Extras lists its tools (${ex.rows.join(' · ')}) with no row that hands replays over, and the page has no __replay.dump (${ex.dump})${ex.err ? ' — ' + ex.err : ''}`);

  // ---- 4. the replay page: both visits listed, nothing that downloads them all, one opens and is handed over by itself
  await go('replay.html'); await sleep(2500);
  const V = await page.evaluate(() => ({ rows: [...document.querySelectorAll('#keptList .row')].map(r => r.textContent), line: document.getElementById('keptLine')?.textContent ?? '', all: !!document.getElementById('dumpNew') || !!document.getElementById('dumpAll') || !!document.querySelector('.keptBtns') }));
  check(V.rows.length === 2 && !V.all && /^2 kept \(/.test(V.line) && !V.rows.some(t => /handed over|grown since|being played/.test(t)), `/replay lists both visits, with no button that downloads them all and no word of a hand-over (${JSON.stringify(V.line.slice(0, 90))})`);
  // each visit in turn: it opens, the page has NO button that downloads it ('Download with notes' was taken out on 2026-10-27: no file leaves a page), and
  // the reader takes the visit as the page put it together, end to end (visit one's last piece was written as its page went away, visit two's by an ordinary
  // beat). The file the reader is fed is the viewer's own open replay, read out of the page here: no download is asked for.
  const opened = [], asked = []; page.on('download', d => asked.push(d.suggestedFilename()));
  for (let i = 0; i < 2; i++) {
    if (i) { await go('replay.html'); await sleep(2500); }
    await page.locator('#keptList .row').nth(i).click(); await sleep(2500);
    const O = await page.evaluate(() => { const f = window.__viewer?.file; return f ? { secs: f.seconds, why: f.why, session: f.session, samples: f.samples.length, text: JSON.stringify(f), button: !!document.getElementById('export') || [...document.querySelectorAll('button, a')].some(b => /download/i.test(b.textContent || '') || b.hasAttribute('download')) } : null; });
    check(!!O && O.samples > 20 && (O.session === A.sid || O.session === B.sid) && !opened.includes(O.session), `row ${i + 1}: a kept visit opens in the viewer (${O ? `${O.session === A.sid ? 'visit one' : 'visit two'}, ${O.secs} s, ${O.samples} samples` : 'nothing opened'})`);
    check(!!O && !O.button, `row ${i + 1}: with a visit open the page has no button or link that downloads it`);
    if (!O) continue; opened.push(O.session);
    const file = path.join(OUT, `visit-${i + 1}.json`); fs.writeFileSync(file, O.text);
    const un = path.join(OUT, `unpacked-${i + 1}`), rd = cp.spawnSync('bun', ['run', path.join(ROOT, 'tools/qa/replay-dump.ts'), file, '--out', un, '--quiet'], { encoding: 'utf8' });
    const IX = fs.existsSync(path.join(un, 'index.json')) ? JSON.parse(fs.readFileSync(path.join(un, 'index.json'), 'utf8')) : { entries: {} }, es = Object.values(IX.entries), e0 = es[0], want = O.session === A.sid ? pA.n : pB.n;
    check(rd.status === 0 && es.length === 1 && !e0.missing.length && /^\d+ steps$/.test(e0.tape), `row ${i + 1}: tools/qa/replay-dump.ts takes the visit the page put together as one whole tape with its run tape (${e0 ? `${e0.scenario} ${e0.seconds} s, ${e0.tape}` : 'nothing'})${rd.status ? '\n' + (rd.stdout + rd.stderr).slice(-600) : ''}`);
    check(!!e0 && Math.abs(e0.n1 - want) <= 3 && e0.f7.length === 0 && /-visit$/.test(e0.name), `row ${i + 1}: the tape reaches the visit's last step, and the reader takes it for a whole visit, not a press of the key (${e0?.n1} of ${want}; ${e0?.name})`);
  }
  check(asked.length === 0, `nothing on /replay asked the browser for a download (${asked.length ? asked.join(', ') : 'none'})`);

  check(!errors.length, `no page or console error${errors.length ? ': ' + errors.slice(0, 4).join(' | ') : ''}`);
  await browser.close();
  console.log(`(page loads: ${went.join(' · ')})`);
  console.log(failed ? '=== replaykeep-page: FAILED ===' : '=== replaykeep-page: ALL PASS ===');
  console.log(`(files: ${OUT})`);
  process.exit(failed ? 1 : 0);
})().catch(e => { console.error(e); process.exit(1); });
