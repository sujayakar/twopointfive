// THE SHADER ROWS ON THE REAL PAGE (src/ui/shaderrows.ts: Options > Graphics > Shaders — "Compact shaders", render/kernels.ts, and "Start on first frame shaders",
// gpu/device.ts PIPE_QUEUE.later): headless Chromium on the software WebGPU adapter, the page's own boot. What each row switches is held headless by
// tools/qa/shaderrows-check.ts on both stands; this is that the page itself boots with them, that the compact texts COMPILE there, and that the rows are on the screen.
//   PORT=5262 bun run tools/serve.ts &      then      node tools/qa/shaderrows-page.cjs [starts] [kept] [first]        (no word = all three)
//   starts WHERE THE ROWS START since 2026-10-27: a PERSON'S browser (this tool's Chromium says a script drives it, and the first-frame row starts off there:
//          the `starts`, `plain` and `kept` pages are opened with navigator.webdriver put to false) that has kept nothing opens the page with no word (medium: a tier that marches the froxels).
//          From the moment the start card is live (`__started`; a script's `__ready` waits for the later set too): the later set out or already in, frames drawn
//          meanwhile with sharp direct standing in and the froxel flag held down, every later pipeline arrives, frames after it with the froxel flag up. Then the
//          load is read: built with Compact shaders, Start on first frame shaders and Separate shadow rays (the split gather's layout is there, no megakernel
//          gather is made), nothing failed, nothing kept, Options shows the three Enabled. Then the three words at 0 on the address: a load as it was (today's
//          gather, no split, the start waited for every pipeline), Options shows the three Disabled (that its row says the address is setting it is
//          shaderrows-check's `plain`: the page paints a row's words only while the row is picked). Then `driven`: the page as every page tool opens it, built
//          without the first-frame row, every pipeline in when the start card is live, Options showing that row Disabled.
//   kept   a browser that has kept nothing opens the page with no word and, WHILE THE RENDERER IS STILL COMPILING, opens Options > Graphics: the section Shaders is
//          there with the three rows that start on Enabled; the first-frame row is turned off and kept off on this machine. The page is then loaded again with no
//          word: it is built without the row (its start waited for every pipeline), nothing failed and nothing on the console; Options shows the row Disabled,
//          Reset to defaults puts each back where it starts and takes the kept line away.
//   first  ?firstframe=1 alone at high (today's kernels: the two other rows that start on are put off on its address), from `__started`: the later set out or in, frames meanwhile, every later pipeline arrives, frames after, no error.
//   exit 0 = all of it with no page or console error · 1 = otherwise.
// THE COMPILE QUEUE (gpu/device.ts) is put OFF for these walks (`pipequeue=0` on every address): that is the page a Mac and every page tool get, each creation straight to the
// browser, so the later set is not held back by anything and is only not WAITED for. QUEUE=1 in front leaves the queue on (the page a Windows browser gets: the later set then
// waits in the queue's background lane, two at a time and shared with the wall-cut twins, which on this adapter is a long wait); the lane's own rules are pipequeue-check's.
// The software adapter compiles nothing like a GPU's driver does: no time printed here is a price on a real machine, and nothing here has been seen on a GPU.
const { chromium } = require('/opt/node-tools/node_modules/playwright');
const PORT = process.env.PORT || 5262, QUEUE_ON = process.env.QUEUE === '1', QUEUE = QUEUE_ON ? '&pipequeue=1' : '&pipequeue=0', WALKS = process.argv.slice(2).length ? process.argv.slice(2) : ['starts', 'kept', 'first'];
const FLAGS = ['--enable-unsafe-webgpu', '--use-gl=angle', '--use-angle=swiftshader', '--enable-features=Vulkan', '--use-vulkan=swiftshader', '--ignore-gpu-blocklist', '--disable-gpu-watchdog'];
// what the software adapter always says at boot (draw-equal.cjs): the smoke solver's pipelines do not compile on it
const NOISE = /""?smoke[A-Za-z0-9_]*""?|\[pipeline\] smoke_|number of storage textures \(5\) in the Compute stage/, NOISE_MAX = 80;
const BANNER = /WebGPU is experimental|SwiftShader|powerPreference|fallback adapter|Failed to load resource/i;
const T0 = Date.now(), log = (...a) => console.log(`[${((Date.now() - T0) / 1000).toFixed(0)}s]`, ...a);
const LS = 'twopointsix.shaderrows', FROXEL_VOL = 536870912, ON3 = ['Compact shaders', 'Start on first frame shaders', 'Separate shadow rays'];
// in the page: the start card is live (main.ts `__started`): what a player has. `__ready` is later now: it waits for the later set of shaders too
/** the start card is live (true), or the boot died (its card's words): a dead boot is not polled for forty minutes */
const STARTED = () => { if (window.__started === true) return true; const f = document.getElementById('fatal'); return f && f.style.display === 'flex' ? `the boot died: ${String(f.textContent).trim().slice(0, 240)}` : false; };
const started = async page => { const v = await (await page.waitForFunction(STARTED, null, { timeout: 2400000, polling: 500 })).jsonValue(); if (v !== true) throw new Error(String(v)); };
/** a PERSON'S browser (navigator.webdriver false): where the first-frame row starts on. A page a script drives says true and starts with that row off (ui/shaderrows.ts).
 *  The word moves the page's other reads of it too (main.ts: a lost device reloads a person's page, so these pages carry `gpuretry=0` and show the card as a
 *  driven page does; the Tab key's first hint and the kit guide stand as a person's, and nothing here looks at either) */
const PERSON = () => { Object.defineProperty(Navigator.prototype, 'webdriver', { get: () => false, configurable: true }); };

// in the page: Options > Graphics by the screen's own buttons, its rows read off it
const OPEN = () => { const b = [...document.querySelectorAll('#menu button')].find(x => x.textContent.trim() === 'Options'); if (!b) return 'no Options button'; b.click();
  const t = document.querySelector('#options .op-tab[data-tab="graphics"]'); if (!t) return 'no Graphics tab'; t.click(); return ''; };
const ROWS = (key) => ({ secs: [...document.querySelectorAll('#options .op-sec')].map(s => s.textContent), waiting: !!document.querySelector('#options .op-wait'),
  rows: Object.fromEntries([...document.querySelectorAll('#options .op-row')].filter(r => r.querySelector('.op-switch')).map(r => [r.querySelector('.op-label').textContent, r.querySelector('.op-switch').textContent])), kept: localStorage.getItem(key) });
const FLIP = (label) => { const r = [...document.querySelectorAll('#options .op-row')].find(x => x.querySelector('.op-label').textContent === label); if (!r) return `no row ${label}`; r.querySelector('.op-switch').click(); return ''; };
const RESET = () => { const b = document.querySelector('#options [data-act="reset"]'); if (!b) return 'no Reset'; b.click(); return ''; };
// in the page: what is in, and n frames drawn one at a time (each waited for)
const STATE = () => { const E = window.__engine, P = E.passes, Q = window.__pipeQueue, B = window.__boot;
  return { frag: P.fragCompReady, frox: P.froxels.ready, volF: P.volFReady, sharp: !!P.sharpOn, queue: Q ? Q.waiting.length + Q.inflight : -1, pipes: B.pipelines, flags: new Uint32Array(E.frameData)[62] >>> 0, gpuInit: (B.gpuInitErrors || []).length, fatal: (document.querySelector('#fatal') || {}).textContent || '' }; };
const DRAW = async (n) => { const dev = window.__engine.device; const yieldMacro = () => new Promise(res => { const ch = new MessageChannel(); ch.port1.onmessage = () => res(); ch.port2.postMessage(0); });
  for (let i = 0; i < n; i++) { window.__runFrames(1, 1 / 60); await dev.queue.onSubmittedWorkDone(); await yieldMacro(); } return true; };

(async () => {
  const browser = await chromium.launch({ headless: true, channel: 'chromium', args: FLAGS });
  let fails = 0; const line = (ok, what, detail = '') => { if (!ok) fails++; console.log(`- ${ok ? 'PASS' : 'FAIL'} — ${what}${detail ? `\n    ${detail}` : ''}`); };
  const fresh = async (person = false) => { const ctx = await browser.newContext({ viewport: { width: 1024, height: 576 }, deviceScaleFactor: 1 }); if (person) await ctx.addInitScript(PERSON); const page = await ctx.newPage(); page.setDefaultNavigationTimeout(900000); page.setDefaultTimeout(2400000);
    const errs = []; let noise = 0;
    page.on('pageerror', e => errs.push(String(e).slice(0, 300)));
    page.on('console', m => { const t = m.text(); if (m.type() !== 'error' || BANNER.test(t)) return; if (NOISE.test(t) && ++noise <= NOISE_MAX) return; errs.push(t.slice(0, 300)); });
    return { ctx, page, errs }; };
  /** the later set's walk on a page that has come ready: out at ready, frames meanwhile, all of it in, frames after */
  const laterWalk = async (page, errs, name, o) => {
    const at = await page.evaluate(STATE); log(`${name}: ready`, JSON.stringify(at));
    line(!at.fatal && at.pipes.failed === 0 && at.gpuInit === 0, `${name}: the page comes ready with no pipeline failed and no GPU error in its boot`, `${at.pipes.done} of ${at.pipes.total} pipelines, ${at.pipes.failed} failed, ${at.gpuInit} GPU errors${at.fatal ? ', fatal: ' + at.fatal : ''}`);
    const outAtReady = at.frag === false && at.frox === false && (!o.compact || at.volF === false);
    // (with the queue off nothing holds the later set back: this adapter compiles in the order it is asked and has had it in before the start resolved, every time so far —
    //  the start then shows only that it COUNTED and waited for fewer pipelines. With the queue on the set must still be out)
    if (QUEUE_ON || outAtReady) line(outAtReady, `${name}: it is ready with the later set still out`, `fragment composite ${at.frag}, froxel kernels ${at.frox}, volumetric twin ${at.volF}, ${at.queue} in the queue`);
    else console.log(`  (${name}: the later set was already in when the start resolved — the queue is off, nothing held it back; the start card counted ${at.pipes.total} pipelines, the later set not among them)`);
    await page.evaluate(DRAW, 3); const mid = await page.evaluate(STATE);
    const stillOut = mid.frag === false || mid.frox === false;
    line(!stillOut || (mid.sharp === true && (mid.flags & FROXEL_VOL) === 0), `${name}: frames drawn meanwhile take the stand-ins (sharp direct for the fragment composite, the froxel flag down)`, stillOut ? `sharp ${mid.sharp}, flags 0x${mid.flags.toString(16)}` : 'the later set was in before a frame could be drawn: not shown by this run');
    const t0 = Date.now(); let st = mid;
    while (Date.now() - t0 < 1500000 && !(st.frag && st.frox && st.volF && st.queue <= 0)) { await new Promise(r => setTimeout(r, 5000)); await page.evaluate(DRAW, 1); st = await page.evaluate(STATE); }
    line(st.frag && st.frox && st.volF, `${name}: every later pipeline arrives`, `after ${((Date.now() - t0) / 1000).toFixed(0)} s on this adapter: fragment composite ${st.frag}, froxel kernels ${st.frox}, volumetric twin ${st.volF}, ${st.queue} left in the queue`);
    await page.evaluate(DRAW, 3); const end = await page.evaluate(STATE);
    line(o.froxel ? (end.flags & FROXEL_VOL) !== 0 : true, `${name}: frames drawn after it${o.froxel ? ' march the froxels (the flag is up)' : ''}`, `flags 0x${end.flags.toString(16)}, sharp ${end.sharp}`);
    line(end.pipes.failed === 0 && !end.fatal && errs.length === 0, `${name}: nothing failed and nothing is on the console`, errs.length ? errs.slice(0, 4).join(' | ') : `${end.pipes.done} boot pipelines`);
  };

  if (WALKS.includes('kept')) {
    const { ctx, page, errs } = await fresh(true);
    const url = `http://localhost:${PORT}/?paused&quality=medium&mute&gpuretry=0${QUEUE}`; log(url);
    await page.goto(url);
    // while the renderer compiles: the rows are there without it
    let opened = 'never looked';
    for (let i = 0; i < 600; i++) { opened = await page.evaluate(OPEN).catch(e => String(e).slice(0, 80)); if (!opened) break; await new Promise(r => setTimeout(r, 500)); }
    const early = opened ? null : await page.evaluate(ROWS, LS);
    line(!!early && early.waiting && early.secs.includes('Shaders') && ON3.every(l => early.rows[l] === 'Enabled') && early.kept === null,
      'kept: while the renderer is still compiling, Options > Graphics has the section Shaders with the three rows that start on Enabled and nothing kept', early ? JSON.stringify(early) : opened);
    if (early) {
      const f = await page.evaluate(FLIP, 'Start on first frame shaders'); const off = await page.evaluate(ROWS, LS);
      line(!f && off.rows['Compact shaders'] === 'Enabled' && off.rows['Start on first frame shaders'] === 'Disabled' && off.kept === '{"firstFrame":false}', 'kept: the first-frame row turned off reads Disabled and is kept off on this machine; a row left where it starts is not written', f || JSON.stringify(off));
    }
    await page.goto(url); await page.evaluate(async () => { await window.__ready; });
    await page.evaluate(DRAW, 2); const at = await page.evaluate(STATE), rows = await page.evaluate(() => window.__bench.deps.rows()); log('kept: ready', JSON.stringify(at), rows.join());
    line(rows.join() === 'compactshaders,splitrays' && !at.fatal && at.pipes.failed === 0 && at.gpuInit === 0 && at.frag && at.frox && at.volF && errs.length === 0,
      'kept: the next load is built without the row (its start waited for every pipeline: the fragment composite, the froxel kernels and the volumetric twin are in), with nothing failed and nothing on the console',
      `rows ${rows.join() || 'none'} · ${at.pipes.done} of ${at.pipes.total} pipelines, ${at.pipes.failed} failed · ${errs.slice(0, 3).join(' | ')}`);
    await page.evaluate(() => { window.__menu.open('pause'); });
    const o2 = await page.evaluate(OPEN), shown = o2 ? null : await page.evaluate(ROWS, LS);
    line(!!shown && !shown.waiting && shown.secs.includes('Shaders') && shown.rows['Compact shaders'] === 'Enabled' && shown.rows['Start on first frame shaders'] === 'Disabled', 'kept: with the renderer up, Options > Graphics shows the row still Disabled', shown ? JSON.stringify(shown) : o2);
    const r = await page.evaluate(RESET), after = await page.evaluate(ROWS, LS);
    line(!r && ON3.every(l => after.rows[l] === 'Enabled') && after.kept === null, 'kept: Reset to defaults puts each back where it starts and takes the kept line away', r || JSON.stringify(after));
    await ctx.close();
  }
  if (WALKS.includes('starts')) {   // where the rows start: three of them are a browser's with nothing kept and no word
    const BUILT = () => { const P = window.__engine.passes, B = window.__boot, labels = (window.__shaderCount?.pipes ?? []).map(p => p.label);
      return { rows: window.__bench.deps.rows(), split: !!P.lFGX, splitPipes: !!(P.pFGXs || P.pFGX), mega: !!(P.pFG || P.pFGs || P.pFGR || P.pFGRs), volF: P.volFReady, labels: labels.filter(l => /gather/i.test(l)), pipes: B.pipelines, kept: localStorage.getItem('twopointsix.shaderrows'), fatal: (document.querySelector('#fatal') || {}).textContent || '' }; };
    for (const [name, words, want] of [['starts', '', true], ['plain', '&compactshaders=0&splitrays=0&firstframe=0', false]]) {
      const { ctx, page, errs } = await fresh(true);
      const url = `http://localhost:${PORT}/?paused&quality=medium&mute&gpuretry=0${words}${QUEUE}`; log(url);
      await page.goto(url);
      // (the default load starts on the first frame's shaders: looked at from the moment a player has it, the later set perhaps still out; then until all of it is in)
      if (want) { await started(page); await laterWalk(page, errs, name, { compact: true, froxel: true }); }
      await page.evaluate(async () => { await window.__ready; });
      await page.evaluate(DRAW, 2); const at = await page.evaluate(BUILT), marks = await page.evaluate(() => window.__boot.marks); log(`${name}: ready`, JSON.stringify(at));
      line(!at.fatal && at.pipes.failed === 0 && errs.length === 0, `${name}: the page comes ready and draws two frames with no pipeline failed and nothing on the console`, `${at.pipes.done} of ${at.pipes.total} pipelines · ${errs.slice(0, 3).join(' | ')}`);
      line(want ? marks.laterReady >= marks.interactive : marks.laterReady === undefined, want ? 'starts: the boot\'s marks say when the last shader built behind the start was in (laterReady, at or after the start card)' : 'plain: with the row off there is no such mark', `interactive ${marks.interactive}, laterReady ${marks.laterReady}`);
      line(want ? at.rows.join() === 'compactshaders,firstframe,splitrays' && at.split && at.splitPipes && !at.mega && at.kept === null : at.rows.length === 0 && !at.split && !at.splitPipes && at.mega && at.kept === null,
        want ? 'starts: with nothing kept and no word the load is built with Compact shaders, Start on first frame shaders and Separate shadow rays (the split gather is there, no megakernel gather), and nothing is kept'
          : 'plain: the three words at 0 are a load as it was (today\'s gather, nothing of the split, the start waited for every pipeline), and nothing is kept', JSON.stringify({ rows: at.rows, split: at.split, splitPipes: at.splitPipes, mega: at.mega, kept: at.kept, pipes: at.pipes.total }));
      await page.evaluate(() => { window.__menu.open('pause'); });
      const o = await page.evaluate(OPEN), shown = o ? null : await page.evaluate(ROWS, LS), state = want ? 'Enabled' : 'Disabled';
      line(!!shown && ON3.every(l => shown.rows[l] === state), `${name}: Options > Graphics shows the three ${state}`, shown ? JSON.stringify(shown.rows) : o);
      await ctx.close();
    }
    // … and the page as every page tool opens it (a script drives the browser): the first-frame row starts off, so `__started` and `__ready` are one moment
    { const { ctx, page, errs } = await fresh();
      const url = `http://localhost:${PORT}/?paused&quality=medium&mute${QUEUE}`; log(url);
      await page.goto(url); await started(page);
      const at = await page.evaluate(STATE), rows = await page.evaluate(() => window.__bench.deps.rows());
      await page.evaluate(async () => { await window.__ready; }); await page.evaluate(DRAW, 2); const later = await page.evaluate(() => window.__boot.marks.laterReady);   // (read once `__ready` has resolved: with the row on the mark is there by then)
      line(rows.join() === 'compactshaders,splitrays' && at.frag && at.frox && at.volF && later === undefined && !at.fatal && at.pipes.failed === 0 && errs.length === 0,
        'driven: a browser a script drives is built without the first-frame row: when the start card is live every pipeline is in (the fragment composite, the froxel kernels, the volumetric twin), with nothing failed and nothing on the console',
        `rows ${rows.join() || 'none'} · ${at.pipes.done} of ${at.pipes.total} pipelines · fragment composite ${at.frag}, froxel kernels ${at.frox}, volumetric twin ${at.volF} · ${errs.slice(0, 3).join(' | ')}`);
      await page.evaluate(() => { window.__menu.open('pause'); });
      const o = await page.evaluate(OPEN), shown = o ? null : await page.evaluate(ROWS, LS);
      line(!!shown && shown.rows['Start on first frame shaders'] === 'Disabled' && shown.rows['Compact shaders'] === 'Enabled' && shown.rows['Separate shadow rays'] === 'Enabled' && shown.kept === null, 'driven: Options > Graphics shows the first-frame row Disabled, the other two Enabled, nothing kept', shown ? JSON.stringify(shown) : o);
      await ctx.close(); }
  }
  if (WALKS.includes('first')) {
    const { ctx, page, errs } = await fresh();
    const url = `http://localhost:${PORT}/?paused&quality=high&mute&firstframe=1&compactshaders=0&splitrays=0${QUEUE}`; log(url);
    await page.goto(url); await started(page);
    await laterWalk(page, errs, 'first', { compact: false, froxel: false });
    const kept = await page.evaluate(k => localStorage.getItem(k), LS);
    line(kept === null, 'first: the address\'s word is this visit\'s and is not kept', String(kept));
    await ctx.close();
  }
  await browser.close();
  console.log(fails ? `\nshaderrows-page: ${fails} FAILED` : '\nshaderrows-page: all passed');
  process.exit(fails ? 1 : 0);
})().catch(e => { console.error('SCRIPT ERROR', String(e && e.stack || e).slice(0, 1200)); process.exit(2); });
