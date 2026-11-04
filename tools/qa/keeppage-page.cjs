// THE KEPT PAGE ON THE REAL PAGE (src/gpu/keep.ts, src/host/keeppage.ts): headless Chromium on the software WebGPU adapter (tools/qa/page-look.cjs's recipe).
//   PORT=5199 node tools/qa/keeppage-page.cjs      (a dev server on that port: PORT=5199 bun run tools/serve.ts; JOBS=a,b = only those jobs)
//   kept     a mission's address opened with ?keeppage=1: the tab's page is a SHELL (the keeper on it, the game's hooks not; its own __ready resolved) and the game
//            boots in its frame on lease 1, compiling its pipelines. A second mission picked in the frame (__scenarios.pick, the mission list's own page load):
//            the tab's document is the one that was opened (a mark left on it is still there, one device opened), the frame is on lease 2, and of the pipelines
//            it asked for the browser compiled at most a few that mission alone needs (named in the line), never the set again; the address bar is the
//            frame's. The frame loaded again (a restart's load): lease 3, NONE compiled, nor a shader module. The second mission's game after 120 frames is, by digest, the game a WHOLE load of that address plays (`fresh`, below)
//            …and the first mission's Game is gone from the browser's memory once the collector has run (a mark on it, held weakly by the shell).
//   stay     the ways back in that were never a page load stay none (a restart, a checkpoint loaded: the same document, the same lease), and the ones that
//            are a load are the frame's: Quit to the mission list, then the same mission picked again — the tab's document kept, nothing compiled
//   record   the same visit pointed at a STAND-IN backend (cloud/test/standin.ts: `npm ci` in cloud/ first): the first load's record says `kept` 1 with what
//            it compiled and the shell's ms, the second mission's `kept` 2 with at most a few compiled of what it asked and a shader wait a fraction of
//            the first's — each sent as its page went, from a frame being loaded over
//   fresh    the second mission's address under ?keeppage=0 in a tab of its own: a page as it always was (no frame, the hooks on it), the digest `kept` is held to
//   leave    a frame sent to an address that says ?keeppage=0: the TAB goes there and loads whole, and the load after it (the word gone from the address) is
//            whole too: the tab keeps the word
//   visitor  no word, in a browser that does not say a script drives it: the shell, by itself; the frame makes no key sink; the replay viewer, asked for in
//            the frame, gets the TAB
//            (HANDOFF=seconds, 900: how long that may take while the browser is still compiling. 30 s on one software-adapter box and 195 s on
//            another; a page that is not kept, sent the same way at the same moment, took 29 s where the kept one took 30)
//   queue    a visitor's page that plays, loaded again in its frame while the GPU is still behind on the frames it had drawn: the new load waits for the queue
//            before it takes the device (its `waited`, inside its device time) and does not sit in the game's build step instead. On the software adapter
//            the queue is tens of seconds behind there; where it is not 8 s behind the job FAILS as untested, it does not pass
//            (what it proves is the WAIT. This mission's start does not freeze without it: under ?keepdrain=0 it built the game in 1.1 s with the queue
//            still behind. The freeze was measured by hand on a tab's fourth and fifth start, docs/agents/render.md)
//   stepped  the same browser with ?paused (a tool's page): a page as it always was
//   driven   no word, in a driven browser: a page as it always was (what every other page tool loads)
// exit 0 = every job, with no page error · 1 = otherwise. KEEPPAGE_BREAK=1 opens `kept` and `visitor` under ?keeppage=0 and `queue` under ?keepdrain=0: they
// must FAIL.
// THE FRAMES ARE NOT DRAWN unless REAL=1 (carlamps-page.cjs's STEP: the software adapter takes minutes over a frame of the level). REAL=1 adds to `kept` one
// drawn frame of the second mission read back, held equal pixel for pixel to `fresh`'s: the kept pipelines draw the picture a whole load's draw. Game logic,
// the DOM and (REAL=1) the software adapter's picture: nothing here has been seen on a GPU.
const { chromium } = require('/opt/node-tools/node_modules/playwright');
const path = require('node:path'), cp = require('node:child_process'), fs = require('node:fs');
const PORT = process.env.PORT || 5199, QUALITY = process.env.QUALITY || 'verylow', BREAK = process.env.KEEPPAGE_BREAK === '1', REAL = process.env.REAL === '1';
const CPORT = Number(process.env.CLOUD_PORT) || 3217, CLOUD = `http://127.0.0.1:${CPORT}`, KEY = 'page-check-key', ROOT = path.resolve(__dirname, '../..');
const ONLY = (process.env.JOBS || '').split(',').map(s => s.trim()).filter(Boolean);
const FLAGS = ['--enable-unsafe-webgpu', '--use-gl=angle', '--use-angle=swiftshader', '--enable-features=Vulkan', '--use-vulkan=swiftshader', '--ignore-gpu-blocklist', '--disable-gpu-watchdog'];
const T0 = Date.now(), log = (...a) => console.log(`[${((Date.now() - T0) / 1000).toFixed(0)}s]`, ...a);
const BASE = `http://localhost:${PORT}/`, REST = `paused&mute&quality=${QUALITY}&seed=7`, PLAYED = `mute&quality=${QUALITY}&seed=7`;   // (PLAYED: no `paused`, a page a person opened)
const VISITOR = [...FLAGS, '--disable-blink-features=AutomationControlled'];   // a browser that does not say a script drives it
const A = `levelFile=depot.json&roomclear&squad4&${REST}`, SECOND = 'rescue-registry', FRAMES = 120;
const HANDOFF = Number(process.env.HANDOFF) || 900;   // seconds the tab is given to leave for another page of the site (`visitor`)
const want = (name) => !ONLY.length || ONLY.includes(name);

// in the page: the game after n frames, the renderer's draw put aside
const PLAY = (n) => { const e = window.__engine, real = e.render; e.render = () => { e.world.upload(); }; try { window.__runFrames(n, 1 / 60); } finally { e.render = real; } return window.__digest.game(); };
// in the page: one drawn frame, read back (page-look.cjs DRAW), as a short hash of its bytes
const DRAW = async () => {
  const dev = window.__engine.device; window.__runFrames(1, 1 / 600); await dev.queue.onSubmittedWorkDone();
  window.__engine.render(window.__cam, window.__game.time, 1 / 60, false); const fr = await window.__engine.readbackFrame(); if (!fr) return null;
  let h = 2166136261 >>> 0, lit = 0; for (let i = 0; i < fr.data.length; i++) { h = Math.imul(h ^ fr.data[i], 16777619) >>> 0; if ((i & 3) !== 3 && fr.data[i] > 8) lit++; }
  return { hash: h.toString(16), w: fr.w, h: fr.h, lit };
};
const SEEN = () => ({ role: window.__keptPage ? window.__keptPage.role : null, n: window.__keptPage && window.__keptPage.lease ? window.__keptPage.lease.n : null, stats: window.__keptPage && window.__keptPage.lease ? { ...window.__keptPage.lease.stats, labels: [...window.__keptPage.lease.stats.labels] } : null,
  boot: window.__boot ? { ...window.__boot.pipelines } : null, href: location.href, game: !!window.__game, framed: window.top !== window.self,
  // (the load's clock on its GPU calls, gpu/keep.ts startCallClock: its tally, the play total that follows it, and what is over the queue's own calls)
  calls: window.__keptPage && window.__keptPage.calls ? JSON.parse(JSON.stringify(window.__keptPage.calls)) : null, play: window.__keptPage && window.__keptPage.play ? { ...window.__keptPage.play } : null,
  clockOver: window.__engine ? ['writeBuffer', 'writeTexture', 'submit'].filter(m => Object.prototype.hasOwnProperty.call(window.__engine.device.queue, m)) : null });
const SHELL = () => ({ keeper: !!window.__pageKeeper, leases: window.__pageKeeper ? window.__pageKeeper.leases : 0, opened: window.__pageKeeper ? window.__pageKeeper.opened : 0, refused: window.__pageKeeper ? window.__pageKeeper.refused : null,
  frame: !!document.getElementById('kept-page'), game: !!window.__game, mark: window.__mark || null, href: location.href, role: window.__keptPage ? window.__keptPage.role : null });

(async () => {
  let red = 0; const fail = (name, why) => { red++; console.log(`- FAIL — ${name}: ${why}`); }, pass = (name, what) => console.log(`- ok — ${name}: ${what}`);
  const open = async (flags = FLAGS) => { const browser = await chromium.launch({ headless: true, channel: 'chromium', args: flags }); const page = await (await browser.newContext({ viewport: { width: 1024, height: 576 }, deviceScaleFactor: 1 })).newPage();
    const errs = []; page.on('pageerror', e => errs.push(String(e).slice(0, 300))); page.on('console', m => { if ((m.type() === 'error' || m.type() === 'warning') && !/Failed to load resource/.test(m.text())) errs.push(m.text().slice(0, 300)); });
    const docs = []; page.on('request', r => { if (r.isNavigationRequest() && r.frame() === page.mainFrame()) docs.push(r.url()); });
    return { browser, page, errs, docs }; };
  const frameOf = (page) => page.frames().find(f => f.parentFrame() === page.mainFrame() && f.name() !== 'x' && /^http/.test(f.url())) || null;
  // the frame's (or the page's) boot over: its lease number is `n` where one is asked for (a load that is still the last one's document has the last one's)
  const ready = async (page, inFrame, n = null, secs = 900) => { for (const end = Date.now() + secs * 1000; Date.now() < end;) { const f = inFrame ? frameOf(page) : page.mainFrame();
      if (f) try { const r = await f.evaluate((want) => Promise.race([window.__ready.then(() => window.__keptPage && (want === null || (window.__keptPage.lease && window.__keptPage.lease.n === want)) ? 'ok' : 'wait', e => 'dead: ' + e), new Promise(r => setTimeout(() => r('wait'), 400))]), n); if (r !== 'wait') return r; } catch (e) { /* the frame is between two documents */ }
      await page.waitForTimeout(400); } return 'no boot in time'; };
  let keptDigest = null, keptSecond = null, keptPicture = null;

  if (want('kept')) { const name = 'kept', { browser, page, errs, docs } = await open(); try {
    await page.goto(`${BASE}?${BREAK ? 'keeppage=0' : 'keeppage=1'}&${A}`); await page.evaluate(() => { window.__mark = 'opened'; });
    const r1 = await ready(page, true, 1); if (r1 !== 'ok') throw new Error(`the first mission's frame: ${r1}`);
    const s0 = await page.evaluate(SHELL), f1 = await frameOf(page).evaluate(SEEN), topReady = await page.evaluate(() => Promise.race([window.__ready.then(() => true), new Promise(r => setTimeout(() => r(false), 2000))]));
    log(` ${name}: first load — ${JSON.stringify(f1.stats)} of boot ${JSON.stringify(f1.boot)}`);
    if (!s0.keeper || !s0.frame || s0.game || s0.role !== 'shell' || !topReady) throw new Error(`the tab's page is not a shell: ${JSON.stringify(s0)} ready ${topReady}`);
    if (f1.role !== 'kept' || f1.n !== 1 || !f1.game || !(f1.stats.pipelines > 50) || f1.boot.failed) throw new Error(`the first frame: ${JSON.stringify(f1)}`);
    log(` ${name}: first load's clock on its GPU calls — ${JSON.stringify(f1.calls)}`);
    // (after the freeze the play total's three calls are over the queue's own)
    if (!(f1.calls && f1.calls.n > 20 && f1.calls.mb > 0) || !f1.play || !f1.clockOver || f1.clockOver.length !== 3) throw new Error(`the first load's clock on its GPU calls: ${JSON.stringify(f1.calls)}, play ${JSON.stringify(f1.play)}, over the queue ${JSON.stringify(f1.clockOver)}`);
    await page.evaluate(() => { window.__old = new WeakRef(document.getElementById('kept-page').contentWindow.__game); });
    await frameOf(page).evaluate((id) => { if (!window.__scenarios.pick(id)) throw new Error('no such mission'); }, SECOND);
    const r2 = await ready(page, true, 2); if (r2 !== 'ok') throw new Error(`the second mission's frame: ${r2}`);
    const s1 = await page.evaluate(SHELL), f2 = await frameOf(page).evaluate(SEEN); keptSecond = f2.href;
    log(` ${name}: second mission — ${JSON.stringify(f2.stats)} of boot ${JSON.stringify(f2.boot)} at ${f2.href}`);
    if (s1.mark !== 'opened' || s1.opened !== 1 || s1.leases !== 2 || docs.length !== 1) throw new Error(`the tab's page was not kept: ${JSON.stringify(s1)}, ${docs.length} loads of the tab`);
    // (another kind of mission may need a program the first never asked for: a few, named — never the renderer's set again)
    if (f2.n !== 2 || f2.stats.pipelines > 4 || f2.stats.modules > 4 || !(f2.stats.asked > 50) || f2.boot.failed || f2.boot.done !== f2.boot.total) throw new Error(`the second mission compiled again: ${JSON.stringify(f2)}`);
    if (!/registry\.json/.test(f2.href) || s1.href !== f2.href) throw new Error(`the address bar is not the frame's: ${s1.href} · ${f2.href}`);
    keptDigest = await frameOf(page).evaluate(PLAY, FRAMES); if (REAL) { keptPicture = await frameOf(page).evaluate(DRAW); log(` ${name}: a drawn frame ${JSON.stringify(keptPicture)}`); }
    await frameOf(page).evaluate(() => { location.reload(); });
    const r3 = await ready(page, true, 3); if (r3 !== 'ok') throw new Error(`the frame loaded again: ${r3}`);
    const s2 = await page.evaluate(SHELL), f3 = await frameOf(page).evaluate(SEEN);
    if (s2.mark !== 'opened' || s2.opened !== 1 || f3.n !== 3 || f3.stats.pipelines !== 0 || f3.stats.modules !== 0 || docs.length !== 1) throw new Error(`a load of the frame again: ${JSON.stringify(s2)} ${JSON.stringify(f3)}`);
    const again = await frameOf(page).evaluate(PLAY, FRAMES); if (again !== keptDigest) throw new Error(`the same mission loaded again plays another game: ${again} · ${keptDigest}`);
    const cdp = await page.context().newCDPSession(page); let gone = false;
    for (let i = 0; i < 6 && !gone; i++) { await cdp.send('HeapProfiler.collectGarbage'); await page.waitForTimeout(500); gone = await page.evaluate(() => window.__old.deref() === undefined); }
    if (!gone) throw new Error('the first mission\'s Game is still held after two loads and a collection: something of the shell\'s keeps a gone page alive');
    if (errs.length) throw new Error(`the page complained: ${errs.slice(0, 3).join(' | ')}`);
    pass(name, `one tab document over three loads; first load compiled ${f1.stats.pipelines} pipelines and ${f1.stats.modules} modules, the second mission ${f2.stats.pipelines} of ${f2.stats.asked} asked${f2.stats.labels.length ? ` (${f2.stats.labels.join(', ')})` : ''}, its reload 0 of ${f3.stats.asked}; digest ${keptDigest} twice; the first mission's Game collected`);
  } catch (e) { fail(name, String(e.message || e)); } await browser.close(); }

  if (want('fresh')) { const name = 'fresh', { browser, page, errs } = await open(); try {
    const url = (keptSecond || `${BASE}?levelFile=registry.json&rescue&keeppage=1&${REST}`).replace('keeppage=1', 'keeppage=0');
    await page.goto(url); const r = await ready(page, false); if (r !== 'ok') throw new Error(r);
    const s = await page.evaluate(SHELL); if (s.keeper || s.frame || !s.game || s.role !== 'plain') throw new Error(`not a plain page: ${JSON.stringify(s)}`);
    const d = await page.evaluate(PLAY, FRAMES); const pic = REAL ? await page.evaluate(DRAW) : null;
    if (errs.length) throw new Error(`the page complained: ${errs.slice(0, 3).join(' | ')}`);
    if (want('kept') && keptDigest !== d) throw new Error(`the second mission on a kept page is not the game a whole load plays: ${keptDigest} · ${d}`);
    if (REAL && want('kept') && (!pic || !keptPicture || pic.hash !== keptPicture.hash || !(pic.lit > 1000))) throw new Error(`the drawn frames differ, or drew nothing: kept ${JSON.stringify(keptPicture)} · whole ${JSON.stringify(pic)}`);
    pass(name, `a whole load of ${url.replace(BASE, '')} plays digest ${d}${want('kept') ? ', the kept page\'s' : ''}${REAL && pic ? `; one drawn frame ${pic.w}×${pic.h} equal byte for byte (${pic.lit} lit bytes)` : ''}`);
  } catch (e) { fail(name, String(e.message || e)); } await browser.close(); }

  // (in a browser that does not say it is driven: the last load, with no word on its address, would be a SHELL but for the word the tab kept)
  if (want('leave')) { const name = 'leave', { browser, page, errs, docs } = await open(VISITOR); try {
    await page.goto(`${BASE}?keeppage=1&solo&${REST}`); const r1 = await ready(page, true, 1); if (r1 !== 'ok') throw new Error(r1);
    await page.evaluate(() => { window.__mark = 'opened'; });
    await frameOf(page).evaluate((to) => { location.assign(to); }, `${BASE}?keeppage=0&solo&${REST}`);
    // (a whole page compiling on this adapter answers nothing for minutes: the first answer is the new document's)
    let r2 = 'the tab never left its shell'; for (const end = Date.now() + 900000; Date.now() < end;) { const m = await page.evaluate(() => window.__mark || null).catch(() => 'between'); if (m === null) { r2 = await ready(page, false); break; } await page.waitForTimeout(1000); }
    if (r2 !== 'ok') throw new Error(r2);
    const s = await page.evaluate(SHELL); if (s.keeper || s.frame || !s.game || docs.length !== 2) throw new Error(`the tab did not load whole: ${JSON.stringify(s)}, ${docs.length} loads`);
    await page.goto(`${BASE}?solo&${PLAYED}`, { waitUntil: 'commit', timeout: 900000 }); const r3 = await ready(page, false); if (r3 !== 'ok') throw new Error(r3);
    const s3 = await page.evaluate(SHELL); if (s3.keeper || s3.role !== 'plain') throw new Error(`the tab forgot the word: ${JSON.stringify(s3)}`);
    if (errs.length) throw new Error(`the page complained: ${errs.slice(0, 3).join(' | ')}`);
    pass(name, 'a frame sent to ?keeppage=0 took the tab there, whole; the next load of the tab, with no word, whole too');
  } catch (e) { fail(name, String(e.message || e)); } await browser.close(); }

  if (want('stay')) { const name = 'stay', { browser, page, errs, docs } = await open(); try {
    await page.goto(`${BASE}?keeppage=1&${A}`); await page.evaluate(() => { window.__mark = 'opened'; });
    const r1 = await ready(page, true, 1); if (r1 !== 'ok') throw new Error(r1);
    const f = frameOf(page); await f.evaluate(() => { window.__doc = 'first'; window.__menu.close(); }); await f.evaluate(PLAY, 90);
    const a = await f.evaluate((PLAYs) => { const play = new Function('n', `return (${PLAYs})(n)`); const g = window.__game, t0 = g.time; g.restart(); play(30);
      const saved = window.__save.save('check', true); play(60); const t1 = g.time, why = window.__save.load(); play(5);
      return { doc: window.__doc, n: window.__keptPage.lease.n, t0, t1, t2: g.time, saved: !!saved, why, current: (window.__scenarios.current() || {}).id || null }; }, PLAY.toString());
    log(` ${name}: restart and checkpoint — ${JSON.stringify(a)}`);
    if (a.doc !== 'first' || a.n !== 1 || !a.saved || a.why || !(a.t2 < a.t1) || docs.length !== 1) throw new Error(`a restart or a checkpoint's load left the page, or did not happen: ${JSON.stringify(a)}`);
    await f.evaluate(() => { window.__scenarios.quit(); });
    const r2 = await ready(page, true, 2); if (r2 !== 'ok') throw new Error(`the quit's load: ${r2}`);
    const q = await frameOf(page).evaluate(() => ({ doc: window.__doc || null, n: window.__keptPage.lease.n, stats: { ...window.__keptPage.lease.stats }, href: location.href, current: (window.__scenarios.current() || {}).id || null, list: !!document.querySelector('#missions.on, #startpage.on, #menu.on') }));
    log(` ${name}: quit — ${JSON.stringify(q)}`);
    const s2 = await page.evaluate(SHELL);
    if (q.doc !== null || q.n !== 2 || q.current || /levelFile/.test(q.href) || s2.mark !== 'opened' || s2.opened !== 1 || s2.href !== q.href || docs.length !== 1) throw new Error(`Quit to the list: ${JSON.stringify(q)} ${JSON.stringify(s2)}`);
    await frameOf(page).evaluate(() => { if (!window.__scenarios.pick('depot-four')) throw new Error('no such mission'); });
    const r3 = await ready(page, true, 3); if (r3 !== 'ok') throw new Error(`the pick's load: ${r3}`);
    const p = await frameOf(page).evaluate(SEEN), s3 = await page.evaluate(SHELL);
    if (p.n !== 3 || p.stats.pipelines !== 0 || p.stats.modules !== 0 || s3.mark !== 'opened' || s3.opened !== 1 || docs.length !== 1) throw new Error(`the same mission picked again compiled, or left the page: ${JSON.stringify(p)} ${JSON.stringify(s3)}`);
    if (errs.length) throw new Error(`the page complained: ${errs.slice(0, 3).join(' | ')}`);
    pass(name, `a restart and a checkpoint's load stayed in the document (lease 1); Quit to the list was a load of the frame (lease 2, ${q.stats.pipelines} of ${q.stats.asked} compiled${q.stats.labels && q.stats.labels.length ? ': ' + q.stats.labels.join(', ') : ''}); the mission picked again lease 3, 0 of ${p.stats.asked}; one tab document throughout`);
  } catch (e) { fail(name, String(e.message || e)); } await browser.close(); }

  if (want('record')) { const name = 'record'; let standin = null; const { browser, page, errs } = await open(); try {
    if (!fs.existsSync(path.join(ROOT, 'cloud/node_modules/convex-test'))) throw new Error('no stand-in backend: run `npm ci` in cloud/ first');
    standin = cp.spawn('bun', ['run', 'test/standin.ts', String(CPORT)], { cwd: path.join(ROOT, 'cloud'), env: { ...process.env, STANDIN_KEY: KEY }, stdio: 'ignore' });
    const admin = async (kind, fn, args) => { const r = await fetch(`${CLOUD}/api/${kind}`, { method: 'POST', headers: { 'Content-Type': 'application/json', Authorization: `Convex ${KEY}` }, body: JSON.stringify({ path: fn, format: 'convex_encoded_json', args: [args] }) }); const j = await r.json(); if (j.status !== 'success') throw new Error(`${fn}: ${JSON.stringify(j).slice(0, 200)}`); return j.value; };
    for (let i = 0; i < 60; i++) { try { await fetch(`${CLOUD}/__stats`); break; } catch { await new Promise(r => setTimeout(r, 250)); } }
    await page.goto(`${BASE}?keeppage=1&cloud=${encodeURIComponent(CLOUD)}&${A}`);
    const sent = async (n) => { const r = await ready(page, true, n); if (r !== 'ok') throw new Error(`load ${n}: ${r}`); return frameOf(page).evaluate(async () => { if (!window.__cloud) return null; await window.__cloud.sync(); return window.__cloud.sid; }); };
    const sid1 = await sent(1); if (!sid1) throw new Error('the page has no backend under ?cloud=');
    await frameOf(page).evaluate((id) => { window.__scenarios.pick(id); }, SECOND);
    const sid2 = await sent(2); if (!sid2 || sid2 === sid1) throw new Error(`the second load's visit: ${sid2} · ${sid1}`);
    await frameOf(page).evaluate(() => { location.reload(); }); await sent(3);
    let b1 = null, b2 = null, kinds = [];
    for (let i = 0; i < 40 && !(b1 && b2); i++) { b1 = ((await admin('query', 'telemetry:list', { sid: sid1, kind: 'boot' }))[0] || {}).data || null; b2 = ((await admin('query', 'telemetry:list', { sid: sid2, kind: 'boot' }))[0] || {}).data || null; if (!(b1 && b2)) await page.waitForTimeout(500); }
    kinds = [...new Set((await admin('query', 'telemetry:list', { sid: sid1 })).map(r => r.kind))].sort();
    log(` ${name}: first ${JSON.stringify(b1 && { kept: b1.kept, made: b1.made, shell: b1.shell, nth: b1.nth, pipelinesMs: b1.pipelinesMs, interactive: b1.interactive, mission: b1.mission })}`);
    log(` ${name}: second ${JSON.stringify(b2 && { kept: b2.kept, made: b2.made, shell: b2.shell, nth: b2.nth, pipelinesMs: b2.pipelinesMs, interactive: b2.interactive, mission: b2.mission })} · the first visit's kinds ${kinds.join(', ')}`);
    if (!b1 || b1.kept !== 1 || !(b1.made && b1.made.pipelines > 50 && b1.made.asked >= b1.made.pipelines) || typeof b1.shell !== 'number' || b1.nth !== 1) throw new Error(`the first load's record: ${JSON.stringify(b1).slice(0, 300)}`);
    if (!b2 || b2.kept !== 2 || !(b2.made && b2.made.pipelines <= 4 && b2.made.asked > 50) || 'shell' in b2 || b2.nth !== 2 || !(b2.pipelinesMs < b1.pipelinesMs / 4)) throw new Error(`the second mission's record: ${JSON.stringify(b2).slice(0, 300)}`);
    if (errs.length) throw new Error(`the page complained: ${errs.slice(0, 3).join(' | ')}`);
    pass(name, `the first load's record: kept 1, ${b1.made.pipelines} of ${b1.made.asked} compiled, ${Math.round(b1.pipelinesMs)} ms on shaders, shell ${b1.shell} ms; the second mission's: kept 2, ${b2.made.pipelines} of ${b2.made.asked}${b2.made.labels ? ' (' + b2.made.labels.join(', ') + ')' : ''}, ${Math.round(b2.pipelinesMs)} ms; both sent as their page went (the first visit's kinds: ${kinds.join(', ')})`);
  } catch (e) { fail(name, String(e.message || e)); } await browser.close(); if (standin) standin.kill(); }

  for (const [name, flags, role, rest] of [['visitor', VISITOR, 'shell', PLAYED], ['stepped', VISITOR, 'plain', REST], ['driven', FLAGS, 'plain', PLAYED]]) { if (!want(name)) continue; const { browser, page, errs } = await open(flags); try {
    await page.goto(`${BASE}?${BREAK && role === 'shell' ? 'keeppage=0&' : ''}solo&${rest}`, { waitUntil: 'commit', timeout: 900000 }); const driven = await page.evaluate(() => navigator.webdriver === true);
    if (driven !== (name === 'driven')) throw new Error(`the browser says driven = ${driven}`);
    const r = await ready(page, role === 'shell', role === 'shell' ? 1 : null); if (r !== 'ok') throw new Error(r);
    const s = await page.evaluate(SHELL); if (s.role !== role || s.keeper !== (role === 'shell') || s.game !== (role === 'plain')) throw new Error(JSON.stringify(s));
    let more = '';
    if (role === 'plain') {   // (a whole page's load is timed as a kept one is, and its record says so)
      const read = () => page.evaluate(() => ({ calls: window.__keptPage && window.__keptPage.calls ? JSON.parse(JSON.stringify(window.__keptPage.calls)) : null, play: window.__keptPage && window.__keptPage.play ? { ...window.__keptPage.play } : null, frames: window.__engine ? window.__engine.frameIdx || 0 : 0,
        over: ['writeBuffer', 'writeTexture', 'submit'].filter(m => Object.prototype.hasOwnProperty.call(window.__engine.device.queue, m)), dev: ['createBuffer', 'createTexture'].filter(m => Object.prototype.hasOwnProperty.call(window.__engine.device, m)) }));
      let c = await read();
      if (!(c.calls && c.calls.n > 20 && c.calls.mb > 0) || c.dev.length) throw new Error(`a whole page's clock on its GPU calls: ${JSON.stringify(c)}`);
      more = `; its load's GPU calls were timed (${c.calls.n} calls, ${Math.round(c.calls.ms)} ms, ${c.calls.mb.toFixed(1)} MB)`;
      // …and after the load the play total runs on a page that draws (a stepped page draws nothing by itself)
      if (!c.play || c.over.length !== 3) throw new Error(`a whole page's play total: ${JSON.stringify(c)}`);
      // (on the software adapter a page that plays answers nothing for minutes at a time: HANDOFF is how long that may take, and no answer is said as such)
      if (name === 'driven') { let answered = 1; for (const end = Date.now() + HANDOFF * 1000; Date.now() < end && !(c.play && c.play.n > 0);) { await new Promise(r => setTimeout(r, 2000)); const got = await Promise.race([read().catch(() => null), new Promise(r => setTimeout(() => r(null), 20000))]); if (got) { c = got; answered++; } }
        if (!(c.play && c.play.n > 0)) throw new Error(`a played page's play total counted nothing in ${HANDOFF} s (the page answered ${answered} times, the last with ${JSON.stringify(c)})`);
        more += `, then its frames' queue calls totalled (${c.play.n} calls, ${Math.round(c.play.ms)} ms)`; }
    }
    if (role === 'shell') {
      // the keys are the frame's own window's (no sink for an outer page's keys: a key pressed in a text box is still a key for the box)
      const sink = await frameOf(page).evaluate(() => document.querySelectorAll('textarea[aria-label="game input"]').length);
      if (sink) throw new Error('the kept frame made a key sink, as a page in somebody else\'s frame does');
      // another page of the site is the TAB's to load
      // (the hand-off is the TAB's navigation, and on the software adapter the tab's page answers nothing while the GPU process is still compiling what the
      // load asked for behind its first frame: so the wait is on the navigation itself, long, and never on an answer from the page that is leaving)
      const t0 = Date.now(), gone = page.waitForURL(u => new URL(String(u)).pathname === '/replay', { waitUntil: 'commit', timeout: HANDOFF * 1000 }).then(() => true, () => false);
      frameOf(page).evaluate(() => { location.assign('/replay'); }).catch(() => { /* the frame went with the tab's page before it could answer */ });
      if (!await gone) throw new Error(`the replay viewer did not get the tab in ${HANDOFF} s: the tab is at ${page.url()}`);
      const took = ((Date.now() - t0) / 1000).toFixed(0);
      let at = null; for (const end = Date.now() + 120000; Date.now() < end && !at;) { at = await Promise.race([page.evaluate(() => ({ path: location.pathname, keeper: !!window.__pageKeeper, frame: !!document.getElementById('kept-page') })).catch(() => null), new Promise(r => setTimeout(() => r(null), 5000))]); if (!at) await page.waitForTimeout(300); }
      if (!at || at.path !== '/replay' || at.keeper || at.frame) throw new Error(`the replay viewer's page is not the tab's own: ${JSON.stringify(at)}`);
      log(` ${name}: the tab was the replay viewer's ${took} s after the frame asked`);
      more = `; its keys are its own window's; the replay viewer, asked for in the frame, got the tab (${took} s)`;
    }
    if (errs.length) throw new Error(`the page complained: ${errs.slice(0, 3).join(' | ')}`);
    pass(name, (role === 'shell' ? 'no word, not driven: the tab\'s page is a shell and the game is in its frame' : name === 'stepped' ? 'no word, ?paused: a page as it always was' : 'no word, driven: a page as it always was') + more);
  } catch (e) { fail(name, String(e.message || e)); } await browser.close(); }

  if (want('queue')) { const name = 'queue', { browser, page, errs } = await open(VISITOR); try {
    await page.goto(`${BASE}?${BREAK ? 'keepdrain=0&' : ''}solo&${PLAYED}`, { waitUntil: 'commit', timeout: 900000 });
    const r = await ready(page, true, 1); if (r !== 'ok') throw new Error(r);
    // how far behind the GPU is, asked of the queue from the SHELL at the moment the frame loads again (the frame's own page is going); the wait's cap is
    // lifted, a software adapter being minutes behind at times
    await page.evaluate(() => { window.__pageKeeper.drainMs = 3600000; const d = document.getElementById('kept-page').contentWindow.__engine.device, t = performance.now(); window.__behind = null; d.queue.onSubmittedWorkDone().then(() => { window.__behind = Math.round(performance.now() - t); }); });
    frameOf(page).evaluate(() => { location.reload(); }).catch(() => { /* the frame's page went before it could answer */ });
    const rq = await ready(page, true, 2, 1800); if (rq !== 'ok') throw new Error(`the frame loaded again: ${rq}`);
    const q = await frameOf(page).evaluate(() => { const w = window.__keptPage.lease.waited; return { waited: w === null ? null : Math.round(w), ctor: Math.round(window.__boot.spans.gameCtor), gpu: Math.round(window.__boot.spans.gpu) }; }), behind = await page.evaluate(() => window.__behind);
    log(` ${name}: the queue was ${behind} ms behind when the frame loaded again; the load waited ${q.waited} ms for it (device ${q.gpu} ms), then built the game in ${q.ctor} ms`);
    if (behind === null) throw new Error(`the queue asked when the frame loaded again has not answered yet, after a load that says it waited ${q.waited} ms for it`);
    if (!(behind > 8000)) throw new Error(`the queue was only ${behind} ms behind when the frame loaded again: the wait is untested on this adapter`);
    if (q.waited === null) throw new Error(`with the queue ${behind} ms behind the load did not wait for it, and built the game in ${q.ctor} ms`);
    if (q.gpu < q.waited - 50) throw new Error(`the load's device time (${q.gpu} ms) does not hold its wait for the queue (${q.waited} ms)`);
    if (q.ctor > behind / 2) throw new Error(`with the queue ${behind} ms behind the load waited ${q.waited} ms for it and then sat ${q.ctor} ms in the game's build step`);
    if (errs.length) throw new Error(`the page complained: ${errs.slice(0, 3).join(' | ')}`);
    pass(name, `a load with the queue ${(behind / 1000).toFixed(0)} s behind waited ${(q.waited / 1000).toFixed(0)} s for it before it took the device, and built the game in ${(q.ctor / 1000).toFixed(1)} s`);
  } catch (e) { fail(name, String(e.message || e)); } await browser.close(); }

  console.log(red ? `keeppage-page: ${red} FAILED` : 'keeppage-page: all ok'); process.exit(red ? 1 : 0);
})().catch(e => { console.error(e); process.exit(1); });
