// WHAT EACH PROGRAM COSTS TO COMPILE, ALONE (gpu/device.ts, ui/boot.ts). The page's boot asks for its hundred-odd pipelines side by side, so the time a
// program took "since it was asked" (window.__boot.slowest, shader-count-page.cjs) is mostly the time it stood in line behind the others: it says which
// came in last, not which is dear. Here the real page boots in headless Chromium with every create{Compute,Render}PipelineAsync call held in ONE line:
// a program is handed to the browser only when the one before it has come back, and its time is its own. The page only sees slower promises. Printed:
//   · every program in the order of its cost: its label, its own milliseconds, the size of its module's text, and the second of the page's life it was
//     asked in (the boot's set, the sets behind the start and a pass's first use are all asked before the page calls itself ready, so the tool cannot name
//     a program's set: the label and that second are what it has)
//   · the sum (what the page costs one thread of the compiler) beside the page's own boot line, and the sum by module text (two modules of one label, a
//     program and its twin, are two rows)
// A program whose module an earlier program of the line already compiled may come back sooner for it (the browser and the driver keep what they made in
// the process): that is what the page pays too, in this order.
//   node tools/qa/shader-serial-page.cjs [--port 5199] [--settle 6] [--json out.json] [address]
// The address is a query string ('?levelFile=compound.json&roomclear&squad4&quality=high'); none = the title at medium. Pipelines made with the calls
// that do not wait (createComputePipeline, createRenderPipeline) cannot be held or timed and are only counted.
// ON THE SOFTWARE ADAPTER THE MILLISECONDS ARE THAT ADAPTER'S: the order may be nothing like a driver's. The table is worth reading from a machine with a
// GPU. Exit 0 = the page came ready and every program of the line came back, 1 = not.
const { chromium } = require('/opt/node-tools/node_modules/playwright');
const fs = require('fs');
const argv = process.argv.slice(2); const opt = (k, d) => { const i = argv.indexOf('--' + k); if (i < 0) return d; const v = argv[i + 1]; argv.splice(i, 2); return v; };
const PORT = opt('port', process.env.PORT || 5199), SETTLE = Number(opt('settle', 6)), JSON_OUT = opt('json', null);
const FLAGS = ['--enable-unsafe-webgpu', '--use-gl=angle', '--use-angle=swiftshader', '--enable-features=Vulkan', '--use-vulkan=swiftshader', '--ignore-gpu-blocklist', '--disable-gpu-watchdog'];
const T0 = Date.now(), log = (...a) => console.log(`[${((Date.now() - T0) / 1000).toFixed(0)}s]`, ...a);

// in the page, before its script: the one line
const LINE = () => {
  const P = typeof GPUDevice === 'function' ? GPUDevice.prototype : null; if (!P) return;
  const S = window.__serial = { rows: [], sync: 0, waiting: 0, ready: -1, idleAt: performance.now(), mods: 0 };
  const ofMod = new WeakMap();
  const sm = P.createShaderModule; P.createShaderModule = function (d) { const m = sm.call(this, d); ofMod.set(m, { label: d.label ?? '', chars: d.code.length, n: ++S.mods }); return m; };
  let tail = Promise.resolve();
  for (const name of ['createComputePipelineAsync', 'createRenderPipelineAsync']) {
    const f = P[name]; P[name] = function (d) {
      const mods = [d.compute, d.vertex, d.fragment].filter(Boolean).map(s => ofMod.get(s.module) ?? { label: '?', chars: 0, n: 0 });
      const row = { label: d.label ?? '', mod: [...new Set(mods.map(m => m.label))].join('+'), modKey: [...new Set(mods.map(m => `${m.label}#${m.n}`))].join('+'), chars: mods.reduce((n, m, i) => mods.findIndex(x => x === m) === i ? n + m.chars : n, 0), asked: performance.now(), ms: -1, ok: false, beforeReady: S.ready < 0 };
      S.rows.push(row); S.waiting++;
      const run = tail.then(() => { const t = performance.now(); return f.call(this, d).then(p => { row.ms = performance.now() - t; row.ok = true; return p; }, e => { row.ms = performance.now() - t; throw e; }); });
      tail = run.then(() => { S.waiting--; S.idleAt = performance.now(); }, () => { S.waiting--; S.idleAt = performance.now(); });
      return run; }; }
  for (const name of ['createComputePipeline', 'createRenderPipeline']) { const f = P[name]; P[name] = function (d) { S.sync++; return f.call(this, d); }; }
};

(async () => {
  const addr = argv[0] ?? '?';
  const q = new URLSearchParams(addr.replace(/^\?/, '')); if (!q.has('mute')) q.set('mute', ''); if (!q.has('quality')) q.set('quality', 'medium');
  const url = `http://localhost:${PORT}/?${q.toString().replace(/=(?=&|$)/g, '')}`;
  const browser = await chromium.launch({ headless: true, channel: 'chromium', args: FLAGS });
  let red = 0;
  try {
    const ctx = await browser.newContext({ viewport: { width: 320, height: 180 }, deviceScaleFactor: 1 });
    await ctx.addInitScript(() => { window.__testVisible = true; });
    await ctx.addInitScript(LINE);
    const page = await ctx.newPage(); const gpuErrs = [];
    page.on('console', m => { if (m.type() === 'error' && /WebGPU|GPU device|pipeline/i.test(m.text())) gpuErrs.push(m.text().slice(0, 300)); });
    log(url);
    await page.goto(url, { waitUntil: 'commit' });
    try { await page.waitForFunction(() => !!window.__ready, null, { timeout: 1500000 }); await page.evaluate(() => { window.__serialReady = false; window.__ready.then(() => { window.__serial.ready = performance.now(); window.__serialReady = true; }, () => { window.__serialReady = 'failed'; }); });
      await page.waitForFunction(() => window.__serialReady !== false, null, { timeout: 1500000, polling: 500 }); if (await page.evaluate(() => window.__serialReady) !== true) throw new Error('the page\'s boot failed'); }
    catch (e) { red++; console.log(`- FAIL — the page did not come ready — ${String(e).slice(0, 200)}`); }
    // the sets behind the start: until the line has stood empty for the settle
    if (!red) await page.waitForFunction(s => window.__serial.waiting === 0 && performance.now() - window.__serial.idleAt > s * 1000, SETTLE, { timeout: 1500000, polling: 500 }).catch(e => { red++; console.log(`- FAIL — the line did not empty — ${String(e).slice(0, 200)}`); });
    const R = await page.evaluate(() => ({ ...window.__serial, line: (window.__boot && window.__boot.line) || '' }));
    const env = await page.evaluate(async () => { try { const a = await navigator.gpu.requestAdapter(); const i = a && (a.info || (a.requestAdapterInfo && await a.requestAdapterInfo())); return i ? `${i.vendor} ${i.architecture} ${i.device} ${i.description}`.trim() : 'no adapter'; } catch (e) { return String(e).slice(0, 80); } });   // (an adapter asked for here, after the page's own: the same machine's answer, not a read of the page's device)
    const rows = R.rows.slice().sort((a, b) => b.ms - a.ms), bad = rows.filter(r => !r.ok);
    if (bad.length) { red++; console.log(`- FAIL — ${bad.length} programs of the line did not come back: ${bad.map(r => r.label).slice(0, 8).join(', ')}`); }
    if (gpuErrs.length) { red++; console.log(`- FAIL — GPU errors on the console: ${gpuErrs.slice(0, 3).join(' · ')}`); }
    const sum = l => l.reduce((n, r) => n + Math.max(0, r.ms), 0), before = rows.filter(r => r.beforeReady);
    console.log(`adapter: ${env}`);
    console.log(`${rows.length} programs in one line (${before.length} asked before the page called itself ready, ${rows.length - before.length} after) · ${R.sync} more made with the calls that do not wait (not timed)`);
    console.log(`the line's sum: ${(sum(rows) / 1000).toFixed(1)} s`);
    console.log('      ms    text KB  asked s  program (module)');
    for (const r of rows) console.log(`${r.ms.toFixed(0).padStart(8)}  ${(r.chars / 1024).toFixed(0).padStart(8)}  ${(r.asked / 1000).toFixed(1).padStart(7)}  ${r.label}${r.mod && r.mod !== r.label ? ` (${r.mod})` : ''}`);
    const byMod = new Map(); for (const r of rows) { const m = byMod.get(r.modKey) ?? { ms: 0, n: 0, label: r.mod }; m.ms += Math.max(0, r.ms); m.n++; byMod.set(r.modKey, m); }
    console.log('by module text, the dearest twelve:');
    for (const [, v] of [...byMod].sort((a, b) => b[1].ms - a[1].ms).slice(0, 12)) console.log(`${v.ms.toFixed(0).padStart(8)}  ${String(v.n).padStart(3)} programs  ${v.label}`);
    if (R.line) console.log(`the page's own line (its times are the held line's, not a boot's): ${R.line}`);
    if (JSON_OUT) fs.writeFileSync(JSON_OUT, JSON.stringify({ url, adapter: env, sync: R.sync, rows: R.rows }, null, 1));
  } catch (e) { red++; console.log(`- FAIL — ${String(e && e.stack || e).slice(0, 600)}`); }
  await browser.close().catch(() => {});
  console.log(red ? `shader-serial-page: ${red} FAILED` : 'shader-serial-page: ok');
  process.exit(red ? 1 : 0);
})();
