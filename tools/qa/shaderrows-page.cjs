// THE SHADER ROWS ON THE REAL PAGE (src/ui/shaderrows.ts: Options > Graphics > Shaders — "Compact shaders", render/kernels.ts, and "Start on first frame shaders",
// gpu/device.ts PIPE_QUEUE.later): headless Chromium on the software WebGPU adapter, the page's own boot. What each row switches is held headless by
// tools/qa/shaderrows-check.ts on both stands; this is that the page itself boots with them, that the compact texts COMPILE there, and that the rows are on the screen.
//   PORT=5262 bun run tools/serve.ts &      then      node tools/qa/shaderrows-page.cjs [kept] [first]        (no word = both walks)
//   kept   a browser that has kept nothing opens the page with no word and, WHILE THE RENDERER IS STILL COMPILING, opens Options > Graphics: the section Shaders is
//          there with both rows Disabled; both are put on and the line is kept on this machine. The page is then loaded again with no word (medium: a tier that
//          marches the froxels): it comes ready with the later set still out (no fragment composite, no froxel kernels, no froxel twin of the volumetric pass, which
//          only the compact set has), frames are drawn meanwhile with sharp direct standing in and the froxel flag held down, every later pipeline arrives, frames
//          are drawn after it with the froxel flag up, no pipeline failed and nothing on the console; Options shows both rows Enabled, Reset to defaults puts them off
//          and takes the kept line away.
//   first  ?firstframe=1 alone at high (today's kernels): ready with the later set out, frames meanwhile, every later pipeline arrives, frames after, no error.
//   exit 0 = all of it with no page or console error · 1 = otherwise.
// THE COMPILE QUEUE (gpu/device.ts) is put OFF for these walks (`pipequeue=0` on every address): that is the page a Mac and every page tool get, each creation straight to the
// browser, so the later set is not held back by anything and is only not WAITED for. QUEUE=1 in front leaves the queue on (the page a Windows browser gets: the later set then
// waits in the queue's background lane, two at a time and shared with the wall-cut twins, which on this adapter is a long wait); the lane's own rules are pipequeue-check's.
// The software adapter compiles nothing like a GPU's driver does: no time printed here is a price on a real machine, and nothing here has been seen on a GPU.
const { chromium } = require('/opt/node-tools/node_modules/playwright');
const PORT = process.env.PORT || 5262, QUEUE_ON = process.env.QUEUE === '1', QUEUE = QUEUE_ON ? '&pipequeue=1' : '&pipequeue=0', WALKS = process.argv.slice(2).length ? process.argv.slice(2) : ['kept', 'first'];
const FLAGS = ['--enable-unsafe-webgpu', '--use-gl=angle', '--use-angle=swiftshader', '--enable-features=Vulkan', '--use-vulkan=swiftshader', '--ignore-gpu-blocklist', '--disable-gpu-watchdog'];
// what the software adapter always says at boot (draw-equal.cjs): the smoke solver's pipelines do not compile on it
const NOISE = /""?smoke[A-Za-z0-9_]*""?|\[pipeline\] smoke_|number of storage textures \(5\) in the Compute stage/, NOISE_MAX = 80;
const BANNER = /WebGPU is experimental|SwiftShader|powerPreference|fallback adapter|Failed to load resource/i;
const T0 = Date.now(), log = (...a) => console.log(`[${((Date.now() - T0) / 1000).toFixed(0)}s]`, ...a);
const LS = 'twopointsix.shaderrows', FROXEL_VOL = 536870912;

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
  const fresh = async () => { const ctx = await browser.newContext({ viewport: { width: 1024, height: 576 }, deviceScaleFactor: 1 }); const page = await ctx.newPage(); page.setDefaultNavigationTimeout(900000); page.setDefaultTimeout(2400000);
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
    const { ctx, page, errs } = await fresh();
    const url = `http://localhost:${PORT}/?paused&quality=medium&mute${QUEUE}`; log(url);
    await page.goto(url);
    // while the renderer compiles: the rows are there without it
    let opened = 'never looked';
    for (let i = 0; i < 600; i++) { opened = await page.evaluate(OPEN).catch(e => String(e).slice(0, 80)); if (!opened) break; await new Promise(r => setTimeout(r, 500)); }
    const early = opened ? null : await page.evaluate(ROWS, LS);
    line(!!early && early.waiting && early.secs.includes('Shaders') && early.rows['Compact shaders'] === 'Disabled' && early.rows['Start on first frame shaders'] === 'Disabled' && early.kept === null,
      'kept: while the renderer is still compiling, Options > Graphics has the section Shaders with both rows Disabled and nothing kept', early ? JSON.stringify(early) : opened);
    if (early) {
      const f = (await page.evaluate(FLIP, 'Compact shaders')) || (await page.evaluate(FLIP, 'Start on first frame shaders')); const on = await page.evaluate(ROWS, LS);
      let k = {}; try { k = JSON.parse(on.kept || '{}'); } catch { /* shown below */ }
      line(!f && on.rows['Compact shaders'] === 'Enabled' && on.rows['Start on first frame shaders'] === 'Enabled' && k.compact === true && k.firstFrame === true, 'kept: both put on read Enabled and are kept on this machine', f || JSON.stringify(on));
    }
    await page.goto(url); await page.evaluate(async () => { await window.__ready; });
    await laterWalk(page, errs, 'kept', { compact: true, froxel: true });
    await page.evaluate(() => { window.__menu.open('pause'); });
    const o2 = await page.evaluate(OPEN), shown = o2 ? null : await page.evaluate(ROWS, LS);
    line(!!shown && !shown.waiting && shown.secs.includes('Shaders') && shown.rows['Compact shaders'] === 'Enabled' && shown.rows['Start on first frame shaders'] === 'Enabled', 'kept: with the renderer up, Options > Graphics shows both rows Enabled under Shaders', shown ? JSON.stringify(shown) : o2);
    const r = await page.evaluate(RESET), after = await page.evaluate(ROWS, LS);
    line(!r && after.rows['Compact shaders'] === 'Disabled' && after.rows['Start on first frame shaders'] === 'Disabled' && after.kept === null, 'kept: Reset to defaults puts both off and takes the kept line away', r || JSON.stringify(after));
    await ctx.close();
  }
  if (WALKS.includes('first')) {
    const { ctx, page, errs } = await fresh();
    const url = `http://localhost:${PORT}/?paused&quality=high&mute&firstframe=1${QUEUE}`; log(url);
    await page.goto(url); await page.evaluate(async () => { await window.__ready; });
    await laterWalk(page, errs, 'first', { compact: false, froxel: false });
    const kept = await page.evaluate(k => localStorage.getItem(k), LS);
    line(kept === null, 'first: the address\'s word is this visit\'s and is not kept', String(kept));
    await ctx.close();
  }
  await browser.close();
  console.log(fails ? `\nshaderrows-page: ${fails} FAILED` : '\nshaderrows-page: all passed');
  process.exit(fails ? 1 : 0);
})().catch(e => { console.error('SCRIPT ERROR', String(e && e.stack || e).slice(0, 1200)); process.exit(2); });
