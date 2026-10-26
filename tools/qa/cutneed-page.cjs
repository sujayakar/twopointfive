// WALL-CUT SHADERS WHEN NEEDED, ON THE PAGE (src/render/cutSwap.ts WHEN NEEDED; ?cutwhenneeded=1): the real page in headless Chromium on the software WebGPU adapter,
// with every shader module and pipeline counted before the page's own script runs (as tools/qa/shader-count-page.cjs counts them). A CUT TWIN is a pipeline whose
// module was compiled behind the cut prelude (`const CUT_BOXES: bool = true;`), other than the g-buffer's own cut pipeline, which is the picture's and always made.
// With the row on it asks that
//   1  the page comes ready and draws frames with NO cut twin made and the set unasked (state 'none')
//   2  a test piece stood in the world (window.__cut.test) asks for the set on the next frames ('piece'), and is drawn whole meanwhile
//   3  the set arrives (state 'ready'), the frame takes it (live) and draws the piece cut, with no GPU error and no page error on the console
// With --compare it loads the page a second time with the row off and asks that the start builds the set by itself, as it was, to the same number of twins. With
// --reader it loads it once more with the row on and does what the page tools do before they stage a piece (await window.__cut.programs.ready): the set must come,
// asked for by the reading. --only-reader runs that page alone.
//   node tools/qa/cutneed-page.cjs [--port 5199] [--frames 12] [--compare] [--reader | --only-reader] [query]      (a server on the port: PORT=5199 bun run tools/serve.ts)
// A frame is seconds on this adapter and the set a couple of minutes, a boot five: the whole walk is about half an hour.
// The query is the page's ('levelFile=compound.json&roomclear&squad4'); none = the title. The software adapter compiles nothing like a GPU's driver does: what
// is checked is WHAT is compiled WHEN, never how long it takes. Exit 0 = every line passed, 1 = one failed.
const { chromium } = require('/opt/node-tools/node_modules/playwright');
const argv = process.argv.slice(2); const opt = (k, d) => { const i = argv.indexOf('--' + k); if (i < 0) return d; const v = argv[i + 1]; argv.splice(i, 2); return v; };
const flag = k => { const i = argv.indexOf('--' + k); if (i < 0) return false; argv.splice(i, 1); return true; };
const PORT = opt('port', process.env.PORT || 5199), FRAMES = Number(opt('frames', 12)), COMPARE = flag('compare'), ONLY_READER = flag('only-reader'), READER = flag('reader') || ONLY_READER, QUERY = (argv[0] || '').replace(/^\?/, '');
const FLAGS = ['--enable-unsafe-webgpu', '--use-gl=angle', '--use-angle=swiftshader', '--enable-features=Vulkan', '--use-vulkan=swiftshader', '--ignore-gpu-blocklist', '--disable-gpu-watchdog'];
const T0 = Date.now(), log = (...a) => console.log(`[${((Date.now() - T0) / 1000).toFixed(0)}s]`, ...a);
let red = 0; const line = (ok, what, detail) => { console.log(`- ${ok ? 'PASS' : 'FAIL'} — ${what}\n    ${detail}`); if (!ok) red++; };

// in the page, before its script: the counters
const COUNT = () => {
  const P = typeof GPUDevice === 'function' ? GPUDevice.prototype : null; if (!P) return;
  const C = window.__cutCount = { pipes: [] }; const cutMod = new WeakSet();
  const sm = P.createShaderModule; P.createShaderModule = function (d) { const m = sm.call(this, d); if (d.code.includes('const CUT_BOXES: bool = true;')) cutMod.add(m); return m; };
  for (const name of ['createComputePipeline', 'createRenderPipeline', 'createComputePipelineAsync', 'createRenderPipelineAsync']) {
    const f = P[name]; P[name] = function (d) { const cut = [d.compute, d.vertex, d.fragment].some(s => s && cutMod.has(s.module)); C.pipes.push({ label: d.label ?? '', cut, at: performance.now() }); return f.call(this, d); }; }
};
const READ = () => { const C = window.__cutCount, I = window.__cut.programs.info, S = window.__cut.stats; const twins = C.pipes.filter(p => p.cut && p.label !== 'gbufBoxesCut');
  return { pipelines: C.pipes.length, twins: twins.length, twinLabels: twins.map(p => p.label), raster: C.pipes.filter(p => p.cut && p.label === 'gbufBoxesCut').length, state: I.state, live: I.live, asked: I.asked, whenNeeded: I.whenNeeded, traced: I.traced, drawnWhole: S.drawnWhole, error: I.error }; };

async function open(browser, row) {
  const ctx = await browser.newContext({ viewport: { width: 320, height: 180 }, deviceScaleFactor: 1 });
  await ctx.addInitScript(() => { window.__testVisible = true; }); await ctx.addInitScript(COUNT);
  const q = new URLSearchParams(QUERY); if (!q.has('mute')) q.set('mute', ''); if (!q.has('quality')) q.set('quality', 'medium'); if (row) q.set('cutwhenneeded', '1');
  const url = `http://localhost:${PORT}/?${q.toString().replace(/=(?=&|$)/g, '')}`;
  const page = await ctx.newPage(); const errs = [], gpuErrs = [];
  page.on('pageerror', e => errs.push(String(e).slice(0, 300)));
  page.on('console', m => { if (m.type() === 'error' && /WebGPU|GPU device|pipeline|cut programs|WGSL/i.test(m.text())) gpuErrs.push(m.text().slice(0, 300)); });
  log(url);
  await page.goto(url, { waitUntil: 'commit' });
  await page.waitForFunction(() => !!window.__ready, null, { timeout: 900000 }); await page.evaluate(async () => { await window.__ready; });
  log('ready');
  return { ctx, page, errs, gpuErrs };
}
const frames = (page, n) => page.evaluate(async n => { for (let k = 0; k < n; k += 20) { window.__runFrames(Math.min(20, n - k)); await new Promise(r => { const ch = new MessageChannel(); ch.port1.onmessage = () => r(); ch.port2.postMessage(0); }); } await window.__engine.device.queue.onSubmittedWorkDone(); }, n);
const until = async (page, what, secs) => { const t = Date.now(); for (;;) { const s = await page.evaluate(() => window.__cut.programs.state); if (what(s)) return s; if (Date.now() - t > secs * 1000) return s; await page.waitForTimeout(2000); } };

(async () => {
  const browser = await chromium.launch({ headless: true, channel: 'chromium', args: FLAGS });
  let twinsOn = -1;
  if (!ONLY_READER) { console.log(`\n=== cutneed-page: the row ON (${QUERY || 'the title'}) ===`);
    const { ctx, page, errs, gpuErrs } = await open(browser, true);
    await frames(page, FRAMES); const a = await page.evaluate(READ);
    line(a.whenNeeded === true && a.twins === 0 && a.state === 'none' && a.asked === '' && a.raster === 1, `1 ready, ${FRAMES} frames drawn: no cut twin has been made and the set is unasked`,
      `pipelines ${a.pipelines} (the g-buffer's own cut pipeline ${a.raster}) · cut twins ${a.twins}${a.twins ? ` (${a.twinLabels.join(', ')})` : ''} · state ${a.state} · asked '${a.asked}' · row ${a.whenNeeded}`);
    const stood = await page.evaluate(() => { const g = window.__game, p = g.player.char.pos; const b = window.__cut.test([p[0] + 1.2, 0, p[2] + 0.6], 'wedge', { scale: 2 }); return !!b; });
    await frames(page, 3); const b = await page.evaluate(READ);
    line(stood && b.traced > 0 && b.asked === 'piece' && (b.state === 'building' || b.state === 'ready') && !b.live, '2 a test piece stood in the world: the next frames ask for the set, and until it is in the frame keeps the boot\'s programs',
      `stood ${stood} · cut boxes a ray can meet ${b.traced} · asked '${b.asked}' · state ${b.state} · live ${b.live} · drawn whole ${b.drawnWhole} · cut twins asked for so far ${b.twins}`);
    const st = await until(page, s => s === 'ready' || s === 'failed', 1500); log(`the set: ${st}`);
    await frames(page, 6); const c = await page.evaluate(READ); twinsOn = c.twins;
    line(c.state === 'ready' && c.live && c.twins > 0 && c.drawnWhole === 0 && !c.error, '3 the set arrives and the frame takes it: the piece is traced and drawn cut',
      `state ${c.state} · live ${c.live} · cut twins made ${c.twins} · pipelines in all ${c.pipelines} · drawn whole ${c.drawnWhole}${c.error ? ` · ${c.error}` : ''}`);
    line(!gpuErrs.length && !errs.length, 'no GPU error and no page error on the console', gpuErrs.length || errs.length ? [...gpuErrs, ...errs].slice(0, 3).join(' | ') : 'none');
    await ctx.close(); }
  if (COMPARE && !ONLY_READER) { console.log('\n=== cutneed-page: the row OFF (as it was) ===');
    const { ctx, page, errs, gpuErrs } = await open(browser, false);
    const st = await until(page, s => s === 'ready' || s === 'failed', 1500); await frames(page, 6); const a = await page.evaluate(READ);
    line(a.whenNeeded === false && st === 'ready' && a.asked === '' && a.twins === twinsOn && !a.live, 'the start builds the set by itself with nothing cut, and it is the same set', `state ${st} · asked '${a.asked}' · cut twins ${a.twins} (the row on, after its piece: ${twinsOn}) · pipelines in all ${a.pipelines} · live ${a.live}`);
    line(!gpuErrs.length && !errs.length, 'no GPU error and no page error on the console', gpuErrs.length || errs.length ? [...gpuErrs, ...errs].slice(0, 3).join(' | ') : 'none');
    await ctx.close(); }
  if (READER) { console.log('\n=== cutneed-page: the row ON, and a tool that awaits the set ===');
    const { ctx, page, errs, gpuErrs } = await open(browser, true);
    const a = await page.evaluate(READ);
    const got = await page.evaluate(() => Promise.race([window.__cut.programs.ready, new Promise(r => setTimeout(() => r('no answer in 25 minutes'), 1500000))]));
    await frames(page, 2); const b = await page.evaluate(READ);
    line(a.state === 'none' && a.twins === 0 && got === true && b.state === 'ready' && b.asked === 'ready' && b.twins > 0 && (twinsOn < 0 || b.twins === twinsOn) && !b.live,
      'awaiting window.__cut.programs.ready asks for the set and resolves true once it is in (nothing cut: the frame stays on the boot\'s programs)',
      `before: state ${a.state}, cut twins ${a.twins} · the promise: ${got} · after: state ${b.state}, asked '${b.asked}', cut twins ${b.twins}${twinsOn >= 0 ? ` (by a piece: ${twinsOn})` : ''}, live ${b.live}`);
    line(!gpuErrs.length && !errs.length, 'no GPU error and no page error on the console', gpuErrs.length || errs.length ? [...gpuErrs, ...errs].slice(0, 3).join(' | ') : 'none');
    await ctx.close(); }
  await browser.close();
  console.log(red ? `\n=== cutneed-page: ${red} FAILED ===` : '\n=== cutneed-page: ALL PASS ===');
  process.exit(red ? 1 : 0);
})().catch(e => { console.error(e); process.exit(1); });
