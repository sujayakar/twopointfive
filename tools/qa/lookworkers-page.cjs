// THE LOOK'S MAPS ON WORKERS, IN A BROWSER (src/render/lookWorkers.ts) — the page's half of tools/qa/lookworkers-check.ts. It does NOT boot the game:
// tools/qa/lookworkers-entry.ts is bundled with `bun build` (which runs the macro that hands the generator's text to the workers, as every build of the
// page does) and loaded in a blank page of headless Chromium. There every layer is made on workers started from that text and made again on the page's
// own thread, and the two are set against each other byte for byte: one PASS / FAIL line, exit 1 on a FAIL or any page error.
// What it shows: a module worker starts from the bundled text in a real browser, and the browser's engine makes the same maps on either thread. The
// seconds it prints are this machine's and say nothing of a player's.
//   node tools/qa/lookworkers-page.cjs [--most 8]
const { chromium } = require('/opt/node-tools/node_modules/playwright');
const { execFileSync } = require('child_process'); const http = require('http'), path = require('path'), fs = require('fs'), os = require('os');
const ROOT = path.resolve(__dirname, '../..'); const argv = process.argv.slice(2), flag = (n, d) => { const i = argv.indexOf(n); return i >= 0 ? argv[i + 1] : d; };
const MOST = +flag('--most', 8);
const T0 = Date.now(), log = (...a) => console.log(`[${((Date.now() - T0) / 1000).toFixed(0)}s]`, ...a);
let browser = null, srv = null; const stop = () => { try { browser && browser.close(); } catch { } try { srv && srv.close(); } catch { } browser = srv = null; };
process.on('exit', stop); for (const s of ['SIGINT', 'SIGTERM', 'SIGHUP']) process.on(s, () => { stop(); process.exit(130); });
(async () => {
  const tmp = fs.mkdtempSync(path.join(os.tmpdir(), 'lookworkers-'));
  execFileSync('bun', ['build', path.join(ROOT, 'tools/qa/lookworkers-entry.ts'), '--outfile', path.join(tmp, 'lw.js'), '--target', 'browser', '--minify'], { cwd: ROOT, stdio: ['ignore', 'ignore', 'inherit'] });
  srv = http.createServer((q, r) => { if (q.url === '/lw.js') { r.writeHead(200, { 'Content-Type': 'text/javascript' }); r.end(fs.readFileSync(path.join(tmp, 'lw.js'))); } else { r.writeHead(200, { 'Content-Type': 'text/html' }); r.end('<!doctype html><meta charset="utf-8"><title>look workers</title><script type="module" src="/lw.js"></script>'); } });
  await new Promise(res => srv.listen(0, '127.0.0.1', res)); const port = srv.address().port;
  browser = await chromium.launch({ headless: true, channel: 'chromium' });
  const page = await browser.newPage(); const errors = [];
  page.on('pageerror', e => { errors.push('pageerror: ' + String(e.stack || e).slice(0, 500)); log('PAGEERROR', String(e).slice(0, 300)); });
  page.on('console', m => { if (m.type() === 'error') { errors.push(m.text().slice(0, 500)); log('console', m.text().slice(0, 600)); } });
  await page.goto(`http://127.0.0.1:${port}/`); await page.waitForFunction(() => !!window.__lookWorkers, null, { timeout: 30000 });
  const cores = await page.evaluate(() => window.__lookWorkers.cores);
  let ok = errors.length === 0;
  for (const exact of [false, true]) {
    const r = await page.evaluate(([most, ex]) => window.__lookWorkers.run(most, ex), [MOST, exact]);
    const good = !r.error && r.bad.length === 0 && r.workers >= 1 && errors.length === 0; ok = ok && good;
    console.log(`${good ? '- PASS' : '- FAIL'} — in Chromium, on ${exact ? 'the game\'s own maths (the page\'s)' : 'the engine\'s maths'}, all ${r.layers ?? '?'} layers made on ${r.workers ?? 0} workers (the page says ${cores} cores) are the maps made on the page's own thread, byte for byte${r.error ? `: ${r.error}` : r.bad.length ? `: NOT ${r.bad.slice(0, 6).join(', ')}` : ''}${errors.length ? ` · ${errors.length} page errors: ${errors[0]}` : ''} · ${((r.onWorkers ?? 0) / 1000).toFixed(1)} s on the workers, ${((r.here ?? 0) / 1000).toFixed(1)} s here (this machine's)`);
  }
  stop(); process.exit(ok ? 0 : 1);
})().catch(e => { console.error(e); stop(); process.exit(1); });
