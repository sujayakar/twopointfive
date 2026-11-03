// THE PRICING RUN, DRIVEN END TO END (src/ui/pricerun.ts, `?look&pricelevels`): the real page in headless Chromium on the software adapter, the run asked for
// on the address for a level or two, and nothing touched until its last page: the chain of page loads (a level's look page, then its page without the look),
// the turns with the layers off and on, the card, the one file. IT PROVES THE CHAIN, NOT A NUMBER: on this adapter a frame takes seconds, so the run is handed
// short turns of a frame or two (window.__priceOpts) and what it times says nothing of a GPU.
//   node tools/qa/pricerun-page.cjs [--port 5199] [--out dist/pricerun] [--quality verylow] [level …]      (default: vault)
// No dev server on the port: one is started from this checkout and stopped again by its pid. One software page at a time on a machine.
// Exit 0 = the last page came up with a row for every level, each with its three states, no program made by the switch, the file handed over, no page error.
const { chromium } = require('/opt/node-tools/node_modules/playwright');
const fs = require('fs'), path = require('path'), http = require('http');
const { spawn } = require('child_process');
const ROOT = path.resolve(__dirname, '../..');
const argv = process.argv.slice(2); const opt = (k, d) => { const i = argv.indexOf('--' + k); if (i < 0) return d; const v = argv[i + 1]; argv.splice(i, 2); return v; };
const PORT = Number(opt('port', process.env.PORT || 5199)), OUT = path.resolve(opt('out', path.join(ROOT, 'dist/pricerun'))), QUALITY = opt('quality', 'verylow'), LEVELS = argv.length ? argv : ['vault'];
// (…and the page is not told a script drives it: navigator.webdriver false, so it boots as it does for a person — the load brief, the attract reel, the device's retry)
const FLAGS = ['--disable-blink-features=AutomationControlled', '--enable-unsafe-webgpu', '--use-gl=angle', '--use-angle=swiftshader', '--enable-features=Vulkan', '--use-vulkan=swiftshader', '--ignore-gpu-blocklist', '--disable-gpu-watchdog'];
const T0 = Date.now(), log = (...a) => console.log(`[${((Date.now() - T0) / 1000).toFixed(0)}s]`, ...a);
const up = () => new Promise(res => { const r = http.get({ host: 'localhost', port: PORT, path: '/', timeout: 2000 }, x => { x.resume(); res(x.statusCode === 200); }); r.on('error', () => res(false)); r.on('timeout', () => { r.destroy(); res(false); }); });
let red = false; const check = (ok, what, detail = '') => { red = red || !ok; console.log(`- ${ok ? 'PASS' : 'FAIL'} — ${what}${detail ? `\n    ${detail}` : ''}`); };

(async () => {
  fs.mkdirSync(OUT, { recursive: true });
  let server = null;
  if (!(await up())) { log(`no dev server on ${PORT}: starting one from this checkout`); server = spawn('bun', ['run', 'tools/serve.ts'], { cwd: ROOT, env: { ...process.env, PORT: String(PORT) }, stdio: 'ignore' });
    for (let i = 0; i < 60 && !(await up()); i++) await new Promise(r => setTimeout(r, 500)); }
  const browser = await chromium.launch({ headless: true, channel: 'chromium', args: FLAGS });
  const ctx = await browser.newContext({ viewport: { width: 320, height: 180 }, deviceScaleFactor: 1, acceptDownloads: true });
  // a frame is seconds long here: a turn is a frame dropped and two kept, and a long frame is not the tab having been away
  await ctx.addInitScript(() => { window.__testVisible = true; window.__priceOpts = { playS: 1, settleS: 0, settleFrames: 1, measureFrames: 2, capS: 240, awayS: 600 }; });
  const page = await ctx.newPage(), errors = [], pages = []; let saved = null;
  page.on('pageerror', x => errors.push('pageerror: ' + String(x && x.stack || x).slice(0, 400)));
  page.on('console', m => { const x = m.text(); if (m.type() === 'error' && !/Failed to load resource/.test(x)) errors.push(x.slice(0, 300)); else if (/price every level/.test(x)) log('page:', x.slice(0, 300)); });
  page.on('framenavigated', f => { if (f === page.mainFrame()) { pages.push(f.url()); log('page', pages.length, f.url().replace(`http://localhost:${PORT}`, '')); } });
  page.on('download', async d => { saved = path.join(OUT, d.suggestedFilename()); await d.saveAs(saved).catch(e => { errors.push('the download was not kept: ' + e.message); saved = null; }); });
  const asked = `/?look&${'pricelevels'}=${LEVELS.join(',')}&quality=${QUALITY}&pricesecs=1`;
  log('goto', asked);
  await page.goto(`http://localhost:${PORT}${asked}`, { waitUntil: 'commit' });
  let file = null, said = '';
  for (const t = Date.now(); Date.now() - t < 2 * 3600 * 1000 && !file;) {   // (the page changes under the question: an evaluate that lands on a navigation throws and is asked again)
    await new Promise(r => setTimeout(r, 5000));
    try { const got = await page.evaluate(() => ({ file: window.__priceLevels ?? null, veil: document.getElementById('bench-veil')?.textContent ?? '', fatal: document.getElementById('fatal')?.textContent ?? '' })); file = got.file;
      if (got.veil && got.veil !== said) { said = got.veil; log('says:', said.slice(0, 140)); } if (got.fatal) { errors.push('fatal: ' + got.fatal.slice(0, 300)); break; } } catch { /* between pages */ }
  }
  const driven = await page.evaluate(() => navigator.webdriver).catch(() => null);
  const report = file ? await page.evaluate(() => document.getElementById('bench-report')?.value ?? '').catch(() => '') : '';
  if (file) await new Promise(r => setTimeout(r, 1500));   // (the download lands a moment after the card)
  await page.screenshot({ path: path.join(OUT, 'last-page.png') }).catch(() => { });
  await browser.close(); if (server) server.kill();
  console.log(`\n=== pricerun-page: ${LEVELS.join(', ')} at ${QUALITY}, software adapter: the chain is what is checked, no number here is a price ===`);
  const rows = file?.rows ?? [], want = LEVELS.length * 2 + 1;
  check(!!file, 'the run reached its last page and left its file on the page', file ? `${rows.length} rows; asked for quality ${file.quality}; ${file.gpu}` : `it stopped at page ${pages.length}: ${pages[pages.length - 1] ?? '?'}`);
  check(pages.length === want && pages.slice(1).every((u, i) => new RegExp(`[?&]pricerun=${i}(&|$)`).test(u) && /[?&]look(&|$)/.test(u) === (i % 2 === 0)), `the chain is the address that asked, then for each level its look page and its page without the look, in order (${want} loads)`, pages.map(u => u.replace(`http://localhost:${PORT}`, '')).join('\n    '));
  check(rows.length === LEVELS.length && rows.every((r, i) => r.level === LEVELS[i]), 'a row for every level asked for, in the order asked', rows.map(r => r.level).join(', '));
  check(rows.length > 0 && rows.every(r => r.off.frames > 0 && r.on.frames > 0 && r.page && r.page.win.frames > 0), 'every row has its three states timed: the page without the look, the layers off, the layers on',
    rows.map(r => `${r.level}: no look ${r.page?.win.ms ?? '—'} ms (GPU ${r.page?.win.gpu ?? '—'}), layers off ${r.off.ms} (${r.off.gpu ?? '—'}), on ${r.on.ms} (${r.on.gpu ?? '—'}); ${r.offBy}`).join('\n    '));
  check(rows.length > 0 && rows.every(r => r.programs.bySwitch === 0 && r.programs.boot > 0), 'the look page counted its shader programs, and switching the layers made none', rows.map(r => `${r.level}: ${r.programs.boot} before ready + ${r.programs.after} after + ${r.programs.bySwitch} by the switch; without the look ${r.page?.programs.boot ?? '—'} + ${r.page?.programs.after ?? '—'}`).join('\n    '));
  check(!!saved && fs.existsSync(saved) && JSON.parse(fs.readFileSync(saved, 'utf8')).rows.length === rows.length, 'the one file was handed over as a download, with the same rows', saved ?? 'no download');
  check(/WHAT THE LOOK COSTS/.test(report) && rows.every(r => report.includes(r.level)), 'the last page shows the table as text, a line a level');
  check(driven === false, 'the page was not told a script drives it (navigator.webdriver false): what ran is what a person\'s page runs', `navigator.webdriver ${driven}`);
  check(errors.length === 0, 'no page error and no error on the console through the whole run', errors.slice(0, 6).join('\n    '));
  if (report) { fs.writeFileSync(path.join(OUT, 'report.txt'), report + '\n'); console.log('\n' + report); }
  console.log(red ? '\n=== pricerun-page: FAILED ===' : '\n=== pricerun-page: ALL PASS ===');
  process.exit(red ? 1 : 0);
})().catch(e => { console.error(e); process.exit(1); });
