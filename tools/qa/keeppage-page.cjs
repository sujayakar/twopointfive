// THE KEPT PAGE ON THE REAL PAGE (src/gpu/keep.ts, src/host/keeppage.ts): headless Chromium on the software WebGPU adapter (tools/qa/page-look.cjs's recipe).
//   PORT=5199 node tools/qa/keeppage-page.cjs      (a dev server on that port: PORT=5199 bun run tools/serve.ts; JOBS=kept or JOBS=fresh = that job alone)
//   kept     a mission's address opened with ?keeppage=1: the tab's page is a SHELL (the keeper on it, the game's hooks not; its own __ready resolved) and the game
//            boots in its frame on lease 1, compiling its pipelines. A second mission's address opened in the frame, as a page load:
//            the tab's document is the one that was opened (a mark left on it is still there, one device opened), the frame is on lease 2, and of the pipelines
//            it asked for the browser compiled at most a few that mission alone needs (named in the line), never the set again; the address bar is the
//            frame's. The frame loaded again (a restart's load): lease 3, NONE compiled, nor a shader module. The second mission's game after 120 frames is, by digest, the game a WHOLE load of that address plays (`fresh`, below)
//            …and the first mission's Game is gone from the browser's memory once the collector has run (a mark on it, held weakly by the shell).
//   fresh    the second mission's address under ?keeppage=0 in a tab of its own: a page as it always was (no frame, the hooks on it), the digest `kept` is held to
// exit 0 = every job, with no page error · 1 = otherwise. KEEPPAGE_BREAK=1 opens `kept` under ?keeppage=0: it must FAIL.
// THE FRAMES ARE NOT DRAWN unless REAL=1 (carlamps-page.cjs's STEP: the software adapter takes minutes over a frame of the level). REAL=1 adds to `kept` one
// drawn frame of the second mission read back, held equal pixel for pixel to `fresh`'s: the kept pipelines draw the picture a whole load's draw. Game logic,
// the DOM and (REAL=1) the software adapter's picture: nothing here has been seen on a GPU.
const { chromium } = require('/opt/node-tools/node_modules/playwright');
const PORT = process.env.PORT || 5199, QUALITY = process.env.QUALITY || 'verylow', BREAK = process.env.KEEPPAGE_BREAK === '1', REAL = process.env.REAL === '1';
const ONLY = (process.env.JOBS === 'all' ? '' : process.env.JOBS || 'kept,fresh').split(',').map(s => s.trim()).filter(Boolean);
const FLAGS = ['--enable-unsafe-webgpu', '--use-gl=angle', '--use-angle=swiftshader', '--enable-features=Vulkan', '--use-vulkan=swiftshader', '--ignore-gpu-blocklist', '--disable-gpu-watchdog'];
const T0 = Date.now(), log = (...a) => console.log(`[${((Date.now() - T0) / 1000).toFixed(0)}s]`, ...a);
const BASE = `http://localhost:${PORT}/`, REST = `paused&mute&quality=${QUALITY}&seed=7`;
const A = `levelFile=depot.json&roomclear&squad4&${REST}`, SECOND = `?levelFile=registry.json&rescue&keeppage=1&${REST}`, FRAMES = 120;
const want = (name) => !ONLY.length || ONLY.includes(name);

// in the page: the game after n frames, the renderer's draw put aside
const PLAY = (n) => { window.__draw.run(n, [], 'none'); return JSON.stringify(window.__draw.game.mark(true)); };
// in the page: one drawn frame, read back (page-look.cjs DRAW), as a short hash of its bytes
const DRAW = async () => {
  const dev = window.__engine.device; window.__draw.run(1, [], 'last'); await dev.queue.onSubmittedWorkDone();
  window.__engine.render(window.__draw.cam, window.__game.time, 1 / 60, false); const fr = await window.__engine.readbackFrame(); if (!fr) return null;
  let h = 2166136261 >>> 0, lit = 0; for (let i = 0; i < fr.data.length; i++) { h = Math.imul(h ^ fr.data[i], 16777619) >>> 0; if ((i & 3) !== 3 && fr.data[i] > 8) lit++; }
  return { hash: h.toString(16), w: fr.w, h: fr.h, lit };
};
const SEEN = () => ({ role: window.__keptPage ? window.__keptPage.role : null, n: window.__keptPage && window.__keptPage.lease ? window.__keptPage.lease.n : null, seat: window.__keptPage && window.__keptPage.lease ? window.__keptPage.lease.seat : null, alive: window.__keptPage && window.__keptPage.lease ? window.__keptPage.lease.alive : null, stats: window.__keptPage && window.__keptPage.lease ? { ...window.__keptPage.lease.stats, labels: [...window.__keptPage.lease.stats.labels] } : null,
  boot: window.__boot ? { ...window.__boot.pipelines } : null, href: location.href, game: !!window.__game, framed: window.top !== window.self,
  // (the load's clock on its GPU calls, gpu/keep.ts startCallClock: its tally, the play total that follows it, and what is over the queue's own calls)
  calls: window.__keptPage && window.__keptPage.calls ? JSON.parse(JSON.stringify(window.__keptPage.calls)) : null, play: window.__keptPage && window.__keptPage.play ? { ...window.__keptPage.play } : null,
  clockOver: window.__engine ? ['writeBuffer', 'writeTexture', 'submit'].filter(m => Object.prototype.hasOwnProperty.call(window.__engine.device.queue, m)) : null });
const SHELL = () => ({ keeper: !!window.__pageKeeper, seats: window.__pageKeeper ? window.__pageKeeper.seats.join() : null, leases: window.__pageKeeper ? window.__pageKeeper.leases : 0, opened: window.__pageKeeper ? window.__pageKeeper.opened : 0, refused: window.__pageKeeper ? window.__pageKeeper.refused : null,
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
    await frameOf(page).evaluate(q => { location.search = q; }, SECOND);
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

  console.log(red ? `keeppage-page: ${red} FAILED` : 'keeppage-page: all ok'); process.exit(red ? 1 : 0);
})().catch(e => { console.error(e); process.exit(1); });
