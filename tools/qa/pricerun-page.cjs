// THE PRICING RUN, DRIVEN END TO END (src/ui/pricerun.ts, `?look&pricelevels`): the real page in headless Chromium on the software adapter, the run asked for
// on the address for a level or two, and nothing touched until its last page: the chain of page loads (a level's look page, then its page without the look),
// the turns with the layers off and on, the card, the one file. IT PROVES THE CHAIN, NOT A NUMBER: on this adapter a frame takes seconds, so the run is handed
// short turns of a frame or two (window.__priceOpts) and what it times says nothing of a GPU.
//   node tools/qa/pricerun-page.cjs [--port 5199] [--out dist/pricerun] [--quality verylow] [level …]      (default: vault)
//   node tools/qa/pricerun-page.cjs --real [--size 1920x1080] [--quality high] [--look "<the look page's words>"] [--look-only] [level …]
//       THE RUN AS THE PAGE RUNS IT, for a machine with a GPU: no short turns, the run's own quality unless one is named, every level on the
//       mission list unless some are named. The same checks; what it times is that machine's, named in the file. Files go to --out, or $OUT.
//       --look "look=walls:0&geo=0" = the look page's own words for the whole run (default `look`); --look-only = no page without the look (pricepage=0).
// No dev server on the port: one is started from this checkout and stopped again by its pid. One software page at a time on a machine.
// Exit 0 = the last page came up with a row for every level, each with its three states, no program made by the switch, the file handed over, no page error.
const { chromium } = require('/opt/node-tools/node_modules/playwright');
const fs = require('fs'), path = require('path'), http = require('http');
const { spawn } = require('child_process');
const ROOT = path.resolve(__dirname, '../..');
const argv = process.argv.slice(2); const opt = (k, d) => { const i = argv.indexOf('--' + k); if (i < 0) return d; const v = argv[i + 1]; argv.splice(i, 2); return v; };
const flag = k => { const i = argv.indexOf('--' + k); if (i < 0) return false; argv.splice(i, 1); return true; };
const REAL = flag('real'), ONLY = flag('look-only'), LOOK = opt('look', 'look');
const PORT = Number(opt('port', process.env.PORT || 5199)), OUT = path.resolve(opt('out', process.env.OUT || path.join(ROOT, 'dist/pricerun'))), QUALITY = opt('quality', REAL ? '' : 'verylow');
const [VW, VH] = opt('size', REAL ? '1920x1080' : '320x180').split('x').map(Number), LEVELS = argv.length ? argv : REAL ? [] : ['vault'];   // (none named on a real run = every level on the list)
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
  const ctx = await browser.newContext({ viewport: { width: VW, height: VH }, deviceScaleFactor: 1, acceptDownloads: true });
  // a frame is seconds long here: a turn is a frame dropped and two kept, and a long frame is not the tab having been away
  await ctx.addInitScript(real => { window.__testVisible = true; if (!real) window.__priceOpts = { playS: 0.5, catchS: 0, settleS: 0, settleFrames: 1, measureFrames: 2, capS: 240, awayS: 600 }; }, REAL);   // (half a second of the game's clock: thirty steps of the page's own scripted step, two a frame; no wait for the GPU to catch up, whose frame is seconds long here)
  const page = await ctx.newPage(), errors = [], pages = []; let saved = null;
  page.on('pageerror', x => errors.push('pageerror: ' + String(x && x.stack || x).slice(0, 400)));
  page.on('console', m => { const x = m.text(); if (m.type() === 'error' && !/Failed to load resource/.test(x)) errors.push(x.slice(0, 300)); else if (/price every level/.test(x)) log('page:', x.slice(0, 300)); });
  page.on('framenavigated', f => { if (f === page.mainFrame()) { pages.push(f.url()); log('page', pages.length, f.url().replace(`http://localhost:${PORT}`, '')); } });
  page.on('download', async d => { saved = path.join(OUT, d.suggestedFilename()); await d.saveAs(saved).catch(e => { errors.push('the download was not kept: ' + e.message); saved = null; }); });
  const asked = `/?${LOOK}&${'pricelevels'}${LEVELS.length ? '=' + LEVELS.join(',') : ''}${QUALITY ? '&quality=' + QUALITY : ''}${REAL ? '' : '&pricesecs=1'}${ONLY ? '&pricepage=0' : ''}&downloads=1`;   // (downloads=1: the page's save door as it was, host/savefile.ts, so the one file comes down here to be read; the run carries the flag to its last page)
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
  console.log(REAL ? `\n=== pricerun-page --real: ${LEVELS.join(', ') || 'every level on the list'} at ${QUALITY || 'the run\'s own quality'}, ${VW}x${VH}: the run as the page runs it; what it times is this machine's (${file?.gpu ?? 'no file'}) ===`
    : `\n=== pricerun-page: ${LEVELS.join(', ')} at ${QUALITY}, software adapter: the chain is what is checked, no number here is a price ===`);
  const rows = file?.rows ?? [], PER = ONLY ? 1 : 2, want = (LEVELS.length || rows.length) * PER + 1;
  check(!!file, 'the run reached its last page and left its file on the page', file ? `${rows.length} rows; asked for quality ${file.quality}; ${file.gpu}` : `it stopped at page ${pages.length}: ${pages[pages.length - 1] ?? '?'}`);
  check(pages.length === want && pages.slice(1).every((u, i) => new RegExp(`[?&]pricerun=${i}(&|$)`).test(u) && /[?&]downloads=1(&|$)/.test(u) && /[?&]look(=|&|$)/.test(u) === (ONLY || i % 2 === 0)), `the chain is the address that asked, then for each level its look page and its page without the look, in order, each carrying the visitor's flag (${want} loads)`, pages.map(u => u.replace(`http://localhost:${PORT}`, '')).join('\n    '));
  check(LEVELS.length ? rows.length === LEVELS.length && rows.every((r, i) => r.level === LEVELS[i]) : rows.length > 1 && new Set(rows.map(r => r.level)).size === rows.length, LEVELS.length ? 'a row for every level asked for, in the order asked' : 'a row for each level of the list, none twice', rows.map(r => r.level).join(', '));
  check(rows.length > 0 && rows.every(r => r.off.frames > 0 && r.on.frames > 0 && (ONLY ? !r.page : r.page && r.page.win.frames > 0)), ONLY ? 'every row has its two states timed: the layers off, the layers on (no page without the look was asked for)' : 'every row has its three states timed: the page without the look, the layers off, the layers on',
    rows.map(r => `${r.level}: no look ${r.page?.win.ms ?? '—'} ms (GPU ${r.page?.win.gpu ?? '—'}), layers off ${r.off.ms} (${r.off.gpu ?? '—'}), on ${r.on.ms} (${r.on.gpu ?? '—'}); ${r.offBy}`).join('\n    '));
  { const wins = rows.flatMap(r => [...(ONLY ? [] : [['no look', r.page?.win]]), ['layers off', r.off], ['layers on', r.on]].map(([k, x]) => ({ level: r.level, k, x })));
    // the compute passes with the g-buffer are added up beside the game's own number, and the GPU's "done" is told in frames (how far behind it is: not a cost)
    check(wins.length > 0 && wins.every(({ x }) => x && x.work > 0 && x.lag > 0 && Math.abs(x.lag - x.work / x.ms) <= 0.02 + 0.01 * x.lag && (x.gpu === null ? x.all === null : x.all !== null && x.all >= x.gpu)), 'each state has the compute passes with the g-buffer added up (no less than the GPU\'s own span), and how many frames after a frame began the GPU was done with it', wins.map(({ level, k, x }) => `${level} ${k}: GPU ${x?.gpu ?? '—'} ms, with the g-buffer ${x?.all ?? '—'} ms; done ${x?.lag ?? '—'} frames after (start to done ${x?.work ?? '—'} ms, between frames ${x?.ms ?? '—'})`).join('\n    ')); }
  { const R = g => g ? `${g.preset} ×${g.scale}${g.cap ? `, at most ${g.cap} rows` : ''}, gather at a ${g.gatherThird ? 'third' : 'half'}, volumetrics at a ${g.volQuarter ? 'quarter' : 'half'}, sharp ${g.sharp ? 'on' : 'off'}, ladder ${g.adaptive === false ? 'off' : 'ON'}` : 'no rung written', same = (a, b) => !!a && !!b && ['preset', 'scale', 'cap', 'gatherThird', 'volQuarter', 'sharp'].every(k => a[k] === b[k]);
    check(rows.length > 0 && rows.every(r => r.rung && r.rung.adaptive === false && (ONLY || (r.page && r.page.rung && r.page.rung.adaptive === false && same(r.rung, r.page.rung) && r.internal.join('x') === r.page.internal.join('x')))), ONLY ? 'every look page stood on its preset\'s own rows with the quality ladder off' : 'a level\'s two pages stood on ONE rung of the quality ladder, at one size, with the ladder off',
      rows.map(r => `${r.level}: look page ${r.internal.join('×')}, ${R(r.rung)}${ONLY ? '' : `\n      ${' '.repeat(r.level.length)}without it ${r.page?.internal.join('×') ?? '—'}, ${R(r.page?.rung)}`}`).join('\n    ')); }
  check(rows.length > 0 && rows.every(r => r.programs.bySwitch === 0 && r.programs.boot > 0), 'the look page counted its shader programs, and switching the layers made none', rows.map(r => `${r.level}: ${r.programs.boot} before ready + ${r.programs.after} after + ${r.programs.bySwitch} by the switch; without the look ${r.page?.programs.boot ?? '—'} + ${r.page?.programs.after ?? '—'}`).join('\n    '));
  check(rows.length > 0 && rows.every(r => r.heldAt !== null && (ONLY || (r.page && r.page.heldAt === r.heldAt))), ONLY ? 'every page was held at a step of the game\'s clock' : 'a level\'s two pages were held at the same step of the game\'s clock', rows.map(r => `${r.level}: look page ${r.heldAt} s, without the look ${r.page?.heldAt ?? '—'} s`).join('\n    '));
  check(rows.length > 0 && rows.every(r => Array.isArray(r.over)) && file?.version === 4 && Array.isArray(file.look) && file.look.some(w => /^look(=|$)/.test(w)) && LOOK.split('&').filter(Boolean).every(w => file.look.includes(w)) && !!file.machine, 'every row carries the run\'s own verdict, and the file says which look was priced (every word asked for is among the words the run carried) and on what', file ? `the look page's words: ${file.look.join('&')}; ${file.gpu}${file.machine?.hz ? `, ${file.machine.hz} Hz` : ''}; verdicts: ${rows.map(r => `${r.level} ${r.over.length ? r.over.join('; ') : 'under'}`).join(' | ')}` : '');
  check(!!file && file.notes.length === 0 && rows.every(r => !r.notes.some(n => /moved during the turns|not the same/.test(n))), 'no page was left unpriced, and nothing moved under the turns', [...(file?.notes ?? []), ...rows.flatMap(r => r.notes.map(n => `${r.level}: ${n}`))].join('\n    ') || 'no notes');
  check(!!saved && fs.existsSync(saved) && JSON.parse(fs.readFileSync(saved, 'utf8')).rows.length === rows.length, 'the one file was handed over as a download, with the same rows', saved ?? 'no download');
  check(/WHAT THE LOOK COSTS/.test(report) && rows.every(r => report.includes(r.level)), 'the last page shows the table as text, a line a level');
  check(driven === false, 'the page was not told a script drives it (navigator.webdriver false): what ran is what a person\'s page runs', `navigator.webdriver ${driven}`);
  check(errors.length === 0, 'no page error and no error on the console through the whole run', errors.slice(0, 6).join('\n    '));
  if (report) { fs.writeFileSync(path.join(OUT, 'report.txt'), report + '\n'); console.log('\n' + report); }
  console.log(red ? '\n=== pricerun-page: FAILED ===' : '\n=== pricerun-page: ALL PASS ===');
  process.exit(red ? 1 : 0);
})().catch(e => { console.error(e); process.exit(1); });
