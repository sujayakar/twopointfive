// A FRAME CAPTURE TAKEN IN THE REAL PAGE (src/render/framecap.ts; F6 on the page): the look page on the pass, a few frames drawn, then a window of 3 frames
// asked through `__framecap.start` and the whole picture once, each sent to this checkout's dev server and read back with tools/qa/capture-read.ts.
//   PORT=5199 node tools/qa/framecap-page.cjs      (the dev server of THIS checkout: `PORT=5199 bun run tools/serve.ts`; its captures/ folder is read here)
//   env: PORT (default 5199) · ROOT (the checkout the server runs in, default the working directory) · URL (the page's query, default the look page at midday)
// Headless Chromium on the SOFTWARE adapter: about six minutes to boot, most of a minute a frame. What it can say: the page boots with every texture watched and
// no GPU error, a capture copies every target it lists without a validation error, the file reaches the server and the reader unpacks it, and which textures
// were not taken and why. What it cannot: a GPU's picture, or what a capture costs a frame there.
// Exit 0 = both captures landed and read back, the window's has the g-buffer, the lit picture and a history among at least 20 targets, each the window's size
// on its own grid, the picture's target is not empty, and no GPU error was raised; 1 otherwise.
const { chromium } = require('/opt/node-tools/node_modules/playwright');
const fs = require('fs'), path = require('path'), { spawnSync } = require('child_process');
const PORT = process.env.PORT || 5199, ROOT = process.env.ROOT || process.cwd(), Q = process.env.URL || 'look&roomclear&squad4&levelFile=pass.json&hud&hour=midday&weather=clear';
const FLAGS = ['--enable-unsafe-webgpu', '--use-gl=angle', '--use-angle=swiftshader', '--enable-features=Vulkan', '--use-vulkan=swiftshader', '--ignore-gpu-blocklist', '--disable-gpu-watchdog'];
const T0 = Date.now(), log = (...a) => console.log(`[${((Date.now() - T0) / 1000).toFixed(0)}s]`, ...a);
let red = 0; const check = (ok, w) => { console.log(`${ok ? '- PASS' : '- FAIL'} — ${w}`); if (!ok) red++; };
(async () => {
  const browser = await chromium.launch({ args: FLAGS }), page = await browser.newPage({ viewport: { width: 640, height: 400 } });
  page.on('pageerror', e => log('PAGEERROR', String(e).slice(0, 300))); const info = []; page.on('console', m => { if (/^\[framecap\]/.test(m.text())) { info.push(m.text()); log(m.text().slice(0, 400)); } });
  try {
    await page.goto(`http://localhost:${PORT}/?${Q}&seed=1&paused&mute&quality=high`, { waitUntil: 'commit' });
    await page.waitForFunction(() => !!window.__ready, null, { timeout: 240000 }); page.setDefaultTimeout(0);
    await page.evaluate(async () => { window.__testVisible = true; await window.__ready; try { window.__menu.close(); } catch { } const E = window.__engine;
      window.__gpuErrs = []; E.device.addEventListener('uncapturederror', e => { window.__gpuErrs.push(String(e.error && e.error.message).slice(0, 300)); });
      window.__done = async () => { await E.device.queue.onSubmittedWorkDone(); await new Promise(res => { const ch = new MessageChannel(); ch.port1.onmessage = () => res(); ch.port2.postMessage(0); }); };
      { const real = E.render.bind(E); E.render = (c, t, dt) => real(c, t, dt, false); } });
    log('booted');
    const step = async n => { for (let i = 0; i < n; i++) await page.evaluate(async () => { window.__runFrames(1, 1 / 60); await window.__done(); }); };
    const take = async (o, n) => { const said = await page.evaluate(o => window.__framecap.start(o), o); log(said); await step(n);
      for (let i = 0; i < 600; i++) { if (!(await page.evaluate(() => window.__framecap.active))) break; await new Promise(r => setTimeout(r, 500)); }
      return page.evaluate(() => window.__framecap.status); };
    await step(3);
    const s1 = await take({ frames: 3, px: 96, at: [0.5, 0.5] }, 3), s2 = await take({ whole: true }, 1);
    const errs = await page.evaluate(() => window.__gpuErrs), trace = await page.evaluate(() => window.__framecap.trace());
    check(errs.length === 0, `no GPU error with every texture watched and two captures taken${errs.length ? ': ' + errs[0] : ''}`);
    const names = [s1, s2].map(s => (/capture (cap-[\w-]+):/.exec(s) || [])[1]), files = names.map(n => n && path.join(ROOT, 'captures', n + '.tpscap.gz'));
    check(files.every(f => f && fs.existsSync(f)) && [s1, s2].every(s => /→ captures\//.test(s)), `both captures reached the dev server: ${s1} | ${s2}`);
    const reads = files.map(f => f && fs.existsSync(f) ? spawnSync('bun', ['run', path.join(ROOT, 'tools/qa/capture-read.ts'), f], { cwd: ROOT, encoding: 'utf8' }) : null);
    const rep = files.map((f, i) => reads[i] && reads[i].status === 0 ? fs.readFileSync(path.join(f.replace(/\.tpscap\.gz$/, ''), 'report.txt'), 'utf8') : '');
    check(reads.every(r => r && r.status === 0), 'the reader unpacks both');
    const targets = r => (/· (\d+) targets/.exec(r) || [])[1] | 0, has = (r, l) => new RegExp(`^  ${l} · `, 'm').test(r);
    check(targets(rep[0]) >= 20 && ['gAlbedo', 'gNormal', 'gDepth', 'gId', 'hdr', 'ldr'].every(l => has(rep[0], l)) && /^  \w*[Hh]ist\w* · /m.test(rep[0]), `the window's capture has ${targets(rep[0])} targets, the g-buffer, the lit picture and a history among them`);
    { const m = /^  hdr · \S+ · (\d+)×(\d+), window (\d+)×(\d+)/m.exec(rep[0]), g = /^  ldr · \S+ · (\d+)×(\d+), window (\d+)×(\d+)/m.exec(rep[0]);
      check(!!m && !!g && +g[3] === 96 && Math.abs(+m[3] - 96 * +m[1] / +g[1]) <= 1, `a 96 px window is 96 texels of the output-size picture and its own share of a target of another size (hdr ${m ? `${m[3]} of ${m[1]}` : '?'}, ldr ${g ? `${g[3]} of ${g[1]}` : '?'})`); }
    { const blk = (/^  hdr · [^\n]*\n((?:     [^\n]*\n)+)/m.exec(rep[0]) || [])[1] || '', zeros = [...blk.matchAll(/zero (\S+)/g)].map(x => +x[1]);
      check(zeros.length === 3 && zeros.every(z => z < 0.5), `the lit picture's window is not empty on any of the 3 frames (share of zero texels ${zeros.join(', ')})`); }
    check(/the whole picture/.test(rep[1]) && targets(rep[1]) >= 20 && !/, window /.test(rep[1]), `the whole-picture capture has ${targets(rep[1])} targets, each whole`);
    check(Array.isArray(trace.dt) && trace.dt.length >= 7 && /WHAT THE RENDERER CHANGED/.test(rep[0]), `the trace holds every drawn frame's length (${trace.dt.length}) and ${trace.events.length} events`);
    const not = (/NOT TAKEN\n([\s\S]*)$/.exec(rep[0]) || [])[1] || ''; console.log('not taken:\n' + (not || '  nothing'));
    console.log(`reports: ${files.map(f => f && f.replace(/\.tpscap\.gz$/, '/report.txt')).join('  ')}`);
  } catch (e) { check(false, `the page run failed: ${String(e).slice(0, 300)}`); }
  await browser.close().catch(() => { });
  console.log(red ? `=== framecap-page: FAILED (${red}) ===` : '=== framecap-page: ALL PASS ==='); process.exit(red ? 1 : 0);
})();
