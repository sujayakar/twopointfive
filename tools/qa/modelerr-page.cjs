// THE GAME'S OWN SENSES AGAINST THE PICTURE, ON THE REAL PAGE (src/host/modelerr.ts, `?modelerr`): headless Chromium on the software WebGPU adapter
// (tools/qa/page-look.cjs's recipe) walks the smallest plan with the picture ASKED, and keeps the report the page makes.
//   PORT=5199 node tools/qa/modelerr-page.cjs [out.json]     (a dev server on that port: PORT=5199 bun run tools/serve.ts)
//   env: PLAN (default tiny) · SIZE (default 480x270) · QUALITY (default verylow) · MINUTES (the most it may take, default 50)
// What it holds: the page boots with the word and no page error, the plan ends, the picture answered light points AND sight lines with smoke on them, and
// the report says so. Its numbers are the software adapter's: the page's own shaders, worked out on a CPU. Nothing here has been seen on a GPU.
// Slow by nature: a drawn frame is about half a minute here, so only the frames the picture is asked on are drawn.
// exit 0 = all of that · 1 = otherwise.
const { chromium } = require('/opt/node-tools/node_modules/playwright');
const fs = require('node:fs');
const PORT = process.env.PORT || 5199, PLAN = process.env.PLAN || 'tiny', QUALITY = process.env.QUALITY || 'verylow', MINUTES = Number(process.env.MINUTES || 50);
const [W, H] = (process.env.SIZE || '480x270').split('x').map(Number), OUT = process.argv[2] || '';
const FLAGS = ['--enable-unsafe-webgpu', '--use-gl=angle', '--use-angle=swiftshader', '--enable-features=Vulkan', '--use-vulkan=swiftshader', '--ignore-gpu-blocklist', '--disable-gpu-watchdog'];
const T0 = Date.now(), log = (...a) => console.log(`[${((Date.now() - T0) / 1000).toFixed(0)}s]`, ...a);
// in the page: up to n frames of the plan. A frame is DRAWN only when the picture owes an answer or is about to be asked (asks waiting, the last frames a
// station settles on). While smoke is in the air and nothing is asked, the game steps undrawn and the solver alone is stepped beside it (smoke.ts
// stepAlone): the smoke ages as on a drawn page at a fraction of a drawn frame's cost
const STEP = async (n) => {
  const dev = window.__engine.device, dt = 1 / 20; let drawn = 0, alone = 0;
  for (let i = 0; i < n; i++) {
    const R = window.__modelerrRun; if (window.__modelerr) break;
    const draw = !R || R.asks.length > 0 || (R.phase === 'settle' && R.left <= 6) || (R.phase === 'gap' && R.left > 24);
    window.__runFrames(1, dt, [], draw ? true : 'none');
    if (draw) drawn++; else if (R && R.phase !== 'settle') { window.__smoke.stepAlone(dt, window.__game.followPos()); alone++; }
    if (draw || alone) { await dev.queue.onSubmittedWorkDone(); await new Promise(res => setTimeout(res, 0)); }
  }
  const R = window.__modelerrRun, S = R ? R.report.samples : [];
  return { done: !!window.__modelerr, station: R ? R.at : -1, phase: R ? R.phase : 'none', waiting: R ? R.asks.length : 0, samples: S.length, answered: S.filter(s => s.pic !== null).length, drawn, alone, time: +window.__game.time.toFixed(2) };
};
(async () => {
  const browser = await chromium.launch({ headless: true, channel: 'chromium', args: FLAGS });
  const page = await (await browser.newContext({ viewport: { width: W, height: H }, deviceScaleFactor: 1 })).newPage();
  const errs = []; page.on('pageerror', e => errs.push(String(e).slice(0, 300)));
  page.on('console', m => { if ((m.type() === 'error' || m.type() === 'warning') && !/Failed to load resource/.test(m.text())) errs.push(m.text().slice(0, 300)); });
  let red = 0; const fail = (why) => { red++; console.log(`- FAIL — ${why}`); };
  try {
    await page.goto(`http://localhost:${PORT}/?modelerr=${PLAN}&paused&mute&quality=${QUALITY}&seed=1`);
    let boot = 'no boot in time';
    for (const end = Date.now() + 900000; Date.now() < end;) {
      try { const r = await page.evaluate(() => Promise.race([window.__ready.then(() => window.__game ? 'ok' : 'wait', e => 'dead: ' + e), new Promise(res => setTimeout(() => res('wait'), 2000))])); if (r !== 'wait') { boot = r; break; } } catch { /* between documents */ }
      await page.waitForTimeout(400); }
    if (boot !== 'ok') throw new Error(boot);
    log('booted');
    await page.evaluate(() => { window.__testVisible = true; window.__menu.close(); });
    let last = null;
    for (const end = Date.now() + MINUTES * 60000; Date.now() < end;) {
      last = await page.evaluate(STEP, 4); log(JSON.stringify(last)); if (last.done) break; }
    if (!last || !last.done) throw new Error(`the plan did not end in ${MINUTES} minutes: ${JSON.stringify(last)}`);
    const R = await page.evaluate(() => window.__modelerr), card = await page.evaluate(() => [...document.querySelectorAll('pre')].map(p => p.textContent).find(t => /senses against the picture/.test(t)) || '');
    if (OUT) { fs.writeFileSync(OUT, JSON.stringify(R)); log('written', OUT); }
    console.log(card);
    const light = R.samples.filter(s => s.k === 'light'), smoke = R.samples.filter(s => s.k === 'smoke'), got = (L) => L.filter(s => s.pic !== null).length;
    const smoked = smoke.filter(s => s.pic !== null && s.pic < 0.99).length;
    if (!R.picture) fail('the report says no picture was asked');
    if (!got(light)) fail(`no light point was answered by the picture (${light.length} asked)`);
    if (!got(smoke)) fail(`no sight line was answered by the picture (${smoke.length} asked)`);
    if (!smoked) fail('the picture has no smoke on any sight line: its solver did not run, or the canister was not let out');
    if (!card) fail('the page showed no card');
    console.log(`- the picture answered ${got(light)} of ${light.length} light points and ${got(smoke)} of ${smoke.length} sight lines (${smoked} with smoke on them) · ${R.unanswered} never answered · world ${R.world} · ${JSON.stringify(R.where || {})}`);
  } catch (e) { fail(String(e && e.message || e)); }
  if (errs.length) { red++; console.log(`- FAIL — the page said:\n    ${[...new Set(errs)].slice(0, 12).join('\n    ')}`); }
  await browser.close().catch(() => {});
  console.log(red ? `\n=== modelerr-page: ${red} FAILED ===` : '\n=== modelerr-page: ok (the software adapter: never seen on a GPU) ===');
  process.exit(red ? 1 : 0);
})();
