// THE PRICING RUN, DRIVEN END TO END (src/ui/pricerun.ts, `?look&pricelevels`): the real page in headless Chromium on the software adapter, the run asked for
// on the address for a level or two, and nothing touched until its last page: the chain of page loads (a level's look page, then its page without the look),
// the turns with the layers off and on, the card, the one file. IT PROVES THE CHAIN, NOT A NUMBER: on this adapter a frame takes seconds, so the run is handed
// short turns of a frame or two (window.__priceOpts) and what it times says nothing of a GPU.
//   node tools/qa/pricerun-page.cjs [--port 5199] [--out dist/pricerun] [--quality verylow] [level …]      (default: vault)
//   node tools/qa/pricerun-page.cjs --real [--size 1920x1080] [--quality high] [--look "<the look page's words>"] [--look-only] [level …]
//       THE RUN AS THE PAGE RUNS IT, for a machine with a GPU: no short turns, the run's own quality unless one is named, every level on the
//       mission list unless some are named. The same checks; what it times is that machine's, named in the file. Files go to --out, or $OUT.
//       --look "look=walls:0&geo=0" = the look page's own words for the whole run (default `look`); --look-only = no page without the look (pricepage=0).
//       --play N = the mission is played N seconds of its own clock before its picture is held (the run's own: 4), to see what a longer stand-up changes.
//       --live N = THE WORLD RUNS TO SECOND N of the game's clock before each page's running read (the run's own: PRICE.runS, 20; 0 = held pictures only, the walk as
//       it was until 11-05: `pricelive` on the address). The running read about doubles a level's time on the walk.
//       --each N = A FRESH BROWSER EVERY N LEVELS (default 5; 0 = one browser for all). A walk of 29 levels in one browser tires: on a rented card, 11-05, the
//       GPU's answer came later and later from about the 15th level and the last seven rows read nonsense with every check green; the same seven alone read
//       sane. So a long walk is run in even parts on one dev server, each this tool again on its own levels, its files in <out>/part-<k> (cleared first): every
//       part's price file is also copied to <out> (the table tool reads the files of one kind together), and the walk passes when every part does.
//       (Ten until the running read came: that walk tired after about a quarter of an hour, which is ten levels of held pictures and five with the world run.)
// No dev server on the port: one is started from this checkout and stopped again by its pid. One software page at a time on a machine.
// Exit 0 = the last page came up with a row for every level, each with its three states and its running reads, no program made by the switch, the file handed over, no page error.
const { chromium } = require('/opt/node-tools/node_modules/playwright');
const fs = require('fs'), path = require('path'), http = require('http');
const { spawn, execFileSync } = require('child_process');
const ROOT = path.resolve(__dirname, '../..');
const argv = process.argv.slice(2); const opt = (k, d) => { const i = argv.indexOf('--' + k); if (i < 0) return d; const v = argv[i + 1]; argv.splice(i, 2); return v; };
const flag = k => { const i = argv.indexOf('--' + k); if (i < 0) return false; argv.splice(i, 1); return true; };
const REAL = flag('real'), ONLY = flag('look-only'), SERVED = flag('served') /* (this tool's own word to a part: the walk holds the dev server) */, LOOK = opt('look', 'look'), PLAY = Number(opt('play', 0)) || 0, LIVE = opt('live', ''), EACH = Math.max(0, Number(opt('each', 5)) || 0);
const PORT = Number(opt('port', process.env.PORT || 5199)), OUT = path.resolve(opt('out', process.env.OUT || path.join(ROOT, 'dist/pricerun'))), QUALITY = opt('quality', REAL ? '' : 'verylow');
const [VW, VH] = opt('size', REAL ? '1920x1080' : '320x180').split('x').map(Number), LEVELS = argv.length ? argv : REAL ? [] : ['vault'];   // (none named on a real run = every level on the list)
// (…and the page is not told a script drives it: navigator.webdriver false, so it boots as it does for a person — the load brief, the attract reel, the device's retry)
const FLAGS = ['--disable-blink-features=AutomationControlled', '--enable-unsafe-webgpu', '--use-gl=angle', '--use-angle=swiftshader', '--enable-features=Vulkan', '--use-vulkan=swiftshader', '--ignore-gpu-blocklist', '--disable-gpu-watchdog'];
const T0 = Date.now(), log = (...a) => console.log(`[${((Date.now() - T0) / 1000).toFixed(0)}s]`, ...a);
const up = () => new Promise(res => { const r = http.get({ host: 'localhost', port: PORT, path: '/', timeout: 2000 }, x => { x.resume(); res(x.statusCode === 200); }); r.on('error', () => res(false)); r.on('timeout', () => { r.destroy(); res(false); }); });
let red = false; const check = (ok, what, detail = '') => { red = red || !ok; console.log(`- ${ok ? 'PASS' : 'FAIL'} — ${what}${detail ? `\n    ${detail}` : ''}`); };

let server = null, browser = null, part = null;   // (let go however the run ends: a throw below must not leave a dev server on the port for the next run to find)
for (const sig of ['SIGINT', 'SIGTERM']) process.on(sig, () => { try { part?.kill(sig); } catch { /* gone */ } try { server?.kill(); } catch { /* gone */ } Promise.resolve(browser?.close()).catch(() => { }).finally(() => process.exit(130)); });   // (… nor a signal from outside, which reaches the part that is walking too)
/** a dev server on the port: the one that is there, or one started from this checkout (true: this process started it and stops it) */
const serve = async () => { if (await up()) { log(SERVED ? `the walk's dev server on ${PORT}` : `A DEV SERVER WAS ALREADY ON ${PORT} and is used as it is: this tool cannot tell whose checkout it serves (the run's file names its build where the page knows it)`); return false; }
  log(`no dev server on ${PORT}: starting one from this checkout`); server = spawn('bun', ['run', 'tools/serve.ts'], { cwd: ROOT, env: { ...process.env, PORT: String(PORT) }, stdio: 'ignore' });
  for (let i = 0; i < 60 && !(await up()); i++) await new Promise(r => setTimeout(r, 500)); return true; };
// A LONG WALK IN PARTS (--each): this tool again for every part, a fresh browser and a fresh GPU process each, one after another, on ONE dev server. The
// parts are even (11 levels at ten each are 6 and 5, not 10 and 1: the run reads a late answer against its own least, and one level has little to be read
// against). The levels, where none are named, are THIS CHECKOUT'S list (the run's own order): a server from another checkout may list others.
const PART_LIMIT_MS = 150 * 60e3;   // (a part that hangs is stopped: its own loop gives a page two hours)
async function inParts(list) {
  const k = Math.ceil(list.length / EACH), size = Math.ceil(list.length / k), parts = []; for (let i = 0; i < list.length; i += size) parts.push(list.slice(i, i + size));
  console.log(`=== pricerun-page${REAL ? ' --real' : ''}: ${list.length} levels in ${parts.length} parts of ${parts.map(p => p.length).join(' + ')}, a fresh browser each (--each ${EACH}) ===`);
  fs.mkdirSync(OUT, { recursive: true }); await serve(); let bad = 0; const reports = [];
  for (let i = 0; i < parts.length; i++) { const levels = parts[i], out = path.join(OUT, `part-${i + 1}`);
    fs.rmSync(out, { recursive: true, force: true });   // (what an earlier walk left there is not this part's)
    console.log(`\n--- part ${i + 1} of ${parts.length}: ${levels.join(', ')} ---`);
    const code = await new Promise(res => { part = spawn(process.execPath, [__filename, ...(REAL ? ['--real'] : []), '--served', '--each', '0', '--port', String(PORT), '--out', out, '--size', `${VW}x${VH}`, '--look', LOOK, ...(QUALITY ? ['--quality', QUALITY] : []), ...(ONLY ? ['--look-only'] : []), ...(PLAY ? ['--play', String(PLAY)] : []), ...(LIVE !== '' ? ['--live', LIVE] : []), ...levels], { stdio: 'inherit' });
      const t = setTimeout(() => { console.log(`part ${i + 1} ran past ${PART_LIMIT_MS / 60e3} minutes and is stopped`); try { part.kill('SIGKILL'); } catch { /* gone */ } }, PART_LIMIT_MS);
      part.on('exit', c => { clearTimeout(t); part = null; res(c ?? 1); }); part.on('error', () => { clearTimeout(t); part = null; res(1); }); });
    if (code !== 0) bad++;
    for (const f of fs.existsSync(out) ? fs.readdirSync(out) : []) { if (/^price-levels-.*\.json$/.test(f)) fs.copyFileSync(path.join(out, f), path.join(OUT, f)); if (f === 'report.txt') reports.push(`part ${i + 1} of ${parts.length}: ${levels.join(', ')}\n\n` + fs.readFileSync(path.join(out, f), 'utf8')); } }
  if (server) { server.kill(); server = null; }
  if (reports.length) fs.writeFileSync(path.join(OUT, 'report.txt'), reports.join('\n\n'));
  console.log(bad ? `\n=== pricerun-page: FAILED (${bad} of ${parts.length} parts) ===` : `\n=== pricerun-page: ALL PASS (${parts.length} parts, ${list.length} levels) ===`);
  process.exit(bad ? 1 : 0);
}
(async () => {
  if (EACH > 0) { const list = LEVELS.length ? LEVELS : JSON.parse(execFileSync('bun', ['-e', "import { priceLevels } from './src/ui/pricerun.ts'; console.log(JSON.stringify(priceLevels().map(l => l.level)))"], { cwd: ROOT, encoding: 'utf8' }));
    if (list.length > EACH) return inParts(list); }
  fs.mkdirSync(OUT, { recursive: true });
  await serve();
  browser = await chromium.launch({ headless: true, channel: 'chromium', args: FLAGS });
  const ctx = await browser.newContext({ viewport: { width: VW, height: VH }, deviceScaleFactor: 1, acceptDownloads: true });
  // a frame is seconds long here: a turn is a frame dropped and two kept, and a long frame is not the tab having been away
  await ctx.addInitScript(([real, play]) => { window.__testVisible = true; if (!real) window.__priceOpts = { playS: 0.5, runS: +((play > 0 ? play : 0.5) + 0.2).toFixed(4) /* (twelve more steps before the running read) */, catchS: 0, settleS: 0, settleFrames: 1, measureFrames: 2, capS: 240, awayS: 600, lateMs: 0 /* (the late answer's rule is not asked here: a frame is seconds long and the timer drops a pass over a second, so there is nothing for it to read) */ }; if (play > 0) window.__priceOpts = { ...(window.__priceOpts || {}), playS: play }; }, [REAL, PLAY]);   // (half a second of the game's clock: thirty steps of the page's own scripted step, each drawn, after the page's own warm-up; no wait for the GPU to catch up before the turns, whose frame is seconds long here)
  const page = await ctx.newPage(), errors = [], pages = []; let saved = null, gaveUp = '';
  page.on('pageerror', x => errors.push('pageerror: ' + String(x && x.stack || x).slice(0, 400)));
  page.on('console', m => { const x = m.text(); if (/price every level/.test(x)) { log('page:', x.slice(0, 300)); gaveUp = gaveUp || x.slice(0, 300); } if (m.type() === 'error' && !/Failed to load resource/.test(x)) errors.push(x.slice(0, 300)); });   // (the run's own word on the console is always its end without a file: stopped, refused, or no session store — not waited on for the cap)
  page.on('framenavigated', f => { if (f === page.mainFrame()) { pages.push(f.url()); log('page', pages.length, f.url().replace(`http://localhost:${PORT}`, '')); } });
  page.on('download', async d => { saved = path.join(OUT, d.suggestedFilename()); await d.saveAs(saved).catch(e => { errors.push('the download was not kept: ' + e.message); saved = null; }); });
  const asked = `/?${LOOK}&${'pricelevels'}${LEVELS.length ? '=' + LEVELS.join(',') : ''}${QUALITY ? '&quality=' + QUALITY : ''}${REAL ? '' : '&pricesecs=1'}${ONLY ? '&pricepage=0' : ''}${LIVE !== '' ? '&pricelive=' + LIVE : ''}&downloads=1`;   // (downloads=1: the page's save door as it was, host/savefile.ts, so the one file comes down here to be read; the run carries the flag to its last page)
  log('goto', asked);
  await page.goto(`http://localhost:${PORT}${asked}`, { waitUntil: 'commit' });
  let file = null, said = '';
  for (const t = Date.now(); Date.now() - t < 2 * 3600 * 1000 && !file && !gaveUp;) {   // (the page changes under the question: an evaluate that lands on a navigation throws and is asked again)
    await new Promise(r => setTimeout(r, 5000));
    try { const got = await page.evaluate(() => ({ file: window.__priceLevels ?? null, veil: document.getElementById('bench-veil')?.textContent ?? '', fatal: document.getElementById('fatal')?.textContent ?? '' })); file = got.file;
      if (got.veil && got.veil !== said) { said = got.veil; log('says:', said.slice(0, 140)); } if (got.fatal) { errors.push('fatal: ' + got.fatal.slice(0, 300)); break; } } catch { /* between pages */ }
  }
  if (gaveUp && !file) errors.push('the run ended itself: ' + gaveUp);
  const driven = await page.evaluate(() => navigator.webdriver).catch(() => null);
  const report = file ? await page.evaluate(() => document.getElementById('bench-report')?.value ?? '').catch(() => '') : '';
  if (file) await new Promise(r => setTimeout(r, 1500));   // (the download lands a moment after the card)
  await page.screenshot({ path: path.join(OUT, 'last-page.png') }).catch(() => { });
  await browser.close(); browser = null; if (server) { server.kill(); server = null; }
  console.log(REAL ? `\n=== pricerun-page --real: ${LEVELS.join(', ') || 'every level on the list'} at ${QUALITY || 'the run\'s own quality'}, ${VW}x${VH}: the run as the page runs it; what it times is this machine's (${file?.gpu ?? 'no file'}) ===`
    : `\n=== pricerun-page: ${LEVELS.join(', ')} at ${QUALITY}, software adapter: the chain is what is checked, no number here is a price ===`);
  const rows = file?.rows ?? [], STEP = Math.ceil((PLAY > 0 ? PLAY : REAL ? 4 : 0.5) * 60 - 1e-6) / 60,   /* (the play ends on a whole sixtieth) */ PER = ONLY ? 1 : 2, want = (LEVELS.length || rows.length) * PER + 1;
  check(!!file, 'the run reached its last page and left its file on the page', file ? `${rows.length} rows; asked for quality ${file.quality}; ${file.gpu}` : `it stopped at page ${pages.length}: ${pages[pages.length - 1] ?? '?'}`);
  check(pages.length === want && pages.slice(1).every((u, i) => new RegExp(`[?&]pricerun=${i}(&|$)`).test(u) && /[?&]downloads=1(&|$)/.test(u) && /[?&]look(=|&|$)/.test(u) === (ONLY || i % 2 === 0)), `the chain is the address that asked, then for each level its look page and its page without the look, in order, each carrying the visitor's flag (${want} loads)`, pages.map(u => u.replace(`http://localhost:${PORT}`, '')).join('\n    '));
  check(LEVELS.length ? rows.length === LEVELS.length && rows.every((r, i) => r.level === LEVELS[i]) : rows.length > 1 && new Set(rows.map(r => r.level)).size === rows.length, LEVELS.length ? 'a row for every level asked for, in the order asked' : 'a row for each level of the list, none twice', rows.map(r => r.level).join(', '));
  check(rows.length > 0 && rows.every(r => r.off.frames > 0 && r.on.frames > 0 && (ONLY ? !r.page : r.page && r.page.win.frames > 0)), ONLY ? 'every row has its two states timed: the layers off, the layers on (no page without the look was asked for)' : 'every row has its three states timed: the page without the look, the layers off, the layers on',
    rows.map(r => `${r.level}: no look ${r.page?.win.ms ?? '—'} ms (GPU ${r.page?.win.gpu ?? '—'}), layers off ${r.off.ms} (${r.off.gpu ?? '—'}), on ${r.on.ms} (${r.on.gpu ?? '—'}); ${r.offBy}`).join('\n    '));
  // THE SECOND THE WORLD RAN TO before its running read is the file's own word (bar.runS; none: held pictures only). What was asked for here is held to it:
  // `--live N` as the page reads the word (a number, nought or more: anything else is no second, and the page says so and keeps its own)
  const RUNS = file?.bar?.runS ?? 0, liveAsked = LIVE !== '' && isFinite(parseFloat(LIVE)) && parseFloat(LIVE) >= 0 ? parseFloat(LIVE) : null;
  check(liveAsked !== null ? RUNS === liveAsked : RUNS > 0, liveAsked !== null ? (liveAsked > 0 ? `the world ran to the second asked for (--live ${liveAsked})` : 'held pictures only were asked for (--live 0), and the file says no running read') : 'the world ran to the run\'s own second before its running read', `the file says ${file?.bar?.runS ?? 'no running read'}${LIVE !== '' && liveAsked === null ? `; --live ${LIVE} is no second` : ''}`);
  { const wins = rows.flatMap(r => [...(ONLY ? [] : [['no look', r.page?.win]]), ['layers off', r.off], ['layers on', r.on], ...(RUNS > 0 ? [...(ONLY ? [] : [['no look, the world running', r.page?.run?.win]]), ['the look page, the world running', r.run?.win]] : [])].map(([k, x]) => ({ level: r.level, k, x })));
    // the compute passes with the g-buffer are added up beside the game's own number, and the GPU's "done" is told in frames (how far behind it is: not a cost)
    check(wins.length > 0 && wins.every(({ x }) => x && x.work > 0 && x.lag > 0 && Math.abs(x.lag - x.work / x.ms) <= 0.02 + 0.01 * x.lag && (x.gpu === null ? x.all === null : x.all !== null && x.all >= x.gpu)), 'each state has the compute passes with the g-buffer added up (no less than the GPU\'s own span), and how many frames after a frame began the GPU was done with it', wins.map(({ level, k, x }) => `${level} ${k}: GPU ${x?.gpu ?? '—'} ms, with the g-buffer ${x?.all ?? '—'} ms; done ${x?.lag ?? '—'} frames after (start to done ${x?.work ?? '—'} ms, between frames ${x?.ms ?? '—'})`).join('\n    ')); }
  { const R = g => g ? `${g.preset} ×${g.scale}${g.cap ? `, at most ${g.cap} rows` : ''}, gather at a ${g.gatherThird ? 'third' : 'half'}, volumetrics at a ${g.volQuarter ? 'quarter' : 'half'}, sharp ${g.sharp ? 'on' : 'off'}, ladder ${g.adaptive === false ? 'off' : 'ON'}` : 'no rung written', same = (a, b) => !!a && !!b && ['preset', 'scale', 'cap', 'gatherThird', 'volQuarter', 'sharp'].every(k => a[k] === b[k]);
    check(rows.length > 0 && rows.every(r => r.rung && r.rung.adaptive === false && (ONLY || (r.page && r.page.rung && r.page.rung.adaptive === false && same(r.rung, r.page.rung) && r.internal.join('x') === r.page.internal.join('x')))), ONLY ? 'every look page stood on its preset\'s own rows with the quality ladder off' : 'a level\'s two pages stood on ONE rung of the quality ladder, at one size, with the ladder off',
      rows.map(r => `${r.level}: look page ${r.internal.join('×')}, ${R(r.rung)}${ONLY ? '' : `\n      ${' '.repeat(r.level.length)}without it ${r.page?.internal.join('×') ?? '—'}, ${R(r.page?.rung)}`}`).join('\n    ')); }
  check(rows.length > 0 && rows.every(r => r.programs.bySwitch === 0 && r.programs.boot > 0), 'the look page counted its shader programs, and switching the layers made none', rows.map(r => `${r.level}: ${r.programs.boot} before ready + ${r.programs.after} after + ${r.programs.bySwitch} by the switch; without the look ${r.page?.programs.boot ?? '—'} + ${r.page?.programs.after ?? '—'}`).join('\n    '));
  check(rows.length > 0 && rows.every(r => r.heldAt !== null && Math.abs(r.heldAt - STEP) < 1e-3 && (ONLY || (r.page && r.page.heldAt === r.heldAt))), ONLY ? `every page was held at the step asked for (${STEP} s of the game's clock)` : `a level's two pages were held at the same step of the game's clock, the one asked for (${STEP} s)`, rows.map(r => `${r.level}: look page ${r.heldAt} s, without the look ${r.page?.heldAt ?? '—'} s`).join('\n    '));
  // (a running read the run set apart is a correct file: an alarm in a mission's first seconds is the mission's, and a walk does not fail for it. On this
  //  tool's own two levels at its own short second nothing should be apart, so there it is held to that)
  if (RUNS > 0) check(rows.length > 0 && rows.every(r => r.run && r.run.win.frames > 0 && r.run.at > RUNS && r.run.at > r.heldAt && Array.isArray(r.overRun) && Array.isArray(r.runApart) && (REAL || r.runApart.length === 0) && (ONLY || (r.page?.run && r.page.run.win.frames > 0 && r.page.run.at > RUNS))),
    `every page was read again WITH THE WORLD RUNNING: played on from its held step past ${RUNS} s of the game's clock, its clock still moving through the turn${REAL ? `, and that read judged or set apart by the run with its reason (${rows.filter(r => r.runApart?.length).length} set apart)` : ', and that read judged'}`,
    rows.map(r => `${r.level}: look page held at ${r.heldAt} s, running read ended at ${r.run?.at ?? '—'} s (${r.run?.win.frames ?? 0} frames, a frame every ${r.run?.win.ms ?? '—'} ms, with the g-buffer ${r.run?.win.all ?? '—'} ms against ${r.on.all ?? '—'} held)${ONLY ? '' : `; without the look ended at ${r.page?.run?.at ?? '—'} s (${r.page?.run?.win.frames ?? 0} frames, ${r.page?.run?.win.all ?? '—'} ms against ${r.page?.win.all ?? '—'} held)`}; ${r.runApart?.length ? 'NOT JUDGED: ' + r.runApart.join('; ') : r.overRun?.length ? r.overRun.join('; ') : 'under'}`).join('\n    '));
  else check(rows.length > 0 && rows.every(r => !r.run && !r.page?.run && r.overRun === undefined), 'the file says no running read, and no row has one', rows.map(r => `${r.level}: ${r.run ? 'HAS a running read' : 'held only'}`).join(', '));
  check(rows.length > 0 && rows.every(r => Array.isArray(r.over)) && file?.version === 6 && Array.isArray(file.look) && file.look.some(w => /^look(=|$)/.test(w)) && LOOK.split('&').filter(Boolean).every(w => file.look.includes(w)) && !!file.machine, 'every row carries the run\'s own verdict, and the file says which look was priced (every word asked for is among the words the run carried) and on what', file ? `the look page's words: ${file.look.join('&')}; build ${file.commit ?? 'not said'}; ${file.gpu}${file.machine?.hz ? `, ${file.machine.hz} Hz` : ''}; verdicts: ${rows.map(r => `${r.level} ${r.over.length ? r.over.join('; ') : 'under'}`).join(' | ')}` : '');
  check(!!file && file.notes.length === 0 && rows.every(r => Array.isArray(r.apart) && r.apart.length === 0), 'no page was left unpriced, and every row is one picture (nothing moved under the turns, its two pages at one size, rung and step), so every row was judged', [...(file?.notes ?? []), ...rows.flatMap(r => [...(r.apart ?? ['no `apart` on the row']), ...r.notes].map(n => `${r.level}: ${n}`))].join('\n    ') || 'no notes');
  check(!!saved && fs.existsSync(saved) && JSON.parse(fs.readFileSync(saved, 'utf8')).rows.length === rows.length, 'the one file was handed over as a download, with the same rows', saved ?? 'no download');
  check(/WHAT THE LOOK COSTS/.test(report) && rows.every(r => report.includes(r.level)) && /RUNNING, with the g-buffer ms/.test(report) === (RUNS > 0), `the last page shows the table as text, a line a level${RUNS > 0 ? ', with the running read\'s columns' : ''}`);
  check(driven === false, 'the page was not told a script drives it (navigator.webdriver false): what ran is what a person\'s page runs', `navigator.webdriver ${driven}`);
  check(errors.length === 0, 'no page error and no error on the console through the whole run', errors.slice(0, 6).join('\n    '));
  if (report) { fs.writeFileSync(path.join(OUT, 'report.txt'), report + '\n'); console.log('\n' + report); }
  console.log(red ? '\n=== pricerun-page: FAILED ===' : '\n=== pricerun-page: ALL PASS ===');
  process.exit(red ? 1 : 0);
})().catch(async e => { console.error(e); try { await browser?.close(); } catch { /* gone */ } try { server?.kill(); } catch { /* gone */ } process.exit(1); });
