// A SECOND VISIT'S BOOT, ON A GPU (a measuring tool, not a check: it has no PASS line and nothing here runs it by itself). The first load of a page compiles every
// shader; the browser keeps what it compiled, so a later visit should boot in the time of its slowest other wait. This opens one address again and again in ONE
// browser the way a player comes back to it (a new tab each visit, the old one closed) and prints, for every visit, the boot's own record: ms to interactive, when
// the script started, the three slowest pipelines by name, every pipeline group over a second, how long the second program set took behind the start.
//   PORT=<a dev server's port> node tools/qa/warmboot-page.cjs [--query '<the address's words>'] [--plan '<name>:<visits>:<stay>[:<cache KB>][:restart] …'] [--out <file.json>]
//     stay = `full` (the visit stays until every shader built behind the start is in, the second program set too) or a number of seconds after interactive
//     cache KB = Chrome's own size for its store of compiled shaders (--gpu-disk-cache-size-kb), left alone where it is not given
//     restart = the browser itself is closed between visits and started again on the same profile (what is kept is then only what reached the disk)
//   Each scenario of the plan starts on a fresh profile, so its first visit is cold. The default plan: full:5:full short:5:3 big:5:3:102400 restart:4:full::restart
// Needs a real adapter: on a machine without one the page never reaches its start and every visit reads `no interactive mark`. Seconds are the machine's own and
// say nothing of another's; what compares is one visit against the next.
const { chromium } = require('/opt/node-tools/node_modules/playwright');
const fs = require('fs'), os = require('os'), path = require('path');
const argv = process.argv.slice(2), flag = (n, d) => { const i = argv.indexOf(n); return i >= 0 ? argv[i + 1] : d; };
const PORT = process.env.PORT; if (!PORT) { console.error('PORT=<the dev server\'s port> is needed (bun run dev, or PORT=5199 bun run tools/serve.ts)'); process.exit(2); }
const QUERY = flag('--query', 'look&solo&levelFile=office.json&quality=high&seed=1&noverdict&nokeep&mute&adapt=0').replace(/^\?/, ''), OUT = flag('--out', '');
const PLAN = flag('--plan', 'full:5:full short:5:3 big:5:3:102400 restart:4:full::restart').split(/\s+/).filter(Boolean).map(s => { const [name, visits, stay, kb, how] = s.split(':'); return { name, visits: +visits || 3, stay: stay === 'full' || !stay ? 'full' : +stay, kb: +kb || 0, restart: how === 'restart' }; });
const LIMIT = +flag('--limit', 240) * 1000, T0 = Date.now(), log = (...a) => console.log(`[${((Date.now() - T0) / 1000).toFixed(0)}s]`, ...a);
const sleep = ms => new Promise(r => setTimeout(r, ms));
let ctx = null; const stop = async () => { try { ctx && await ctx.close(); } catch { } ctx = null; };
for (const s of ['SIGINT', 'SIGTERM', 'SIGHUP']) process.on(s, () => { stop().then(() => process.exit(130)); });

/** one visit: a new tab on the address, to interactive, the stay, the boot's record, the tab closed */
async function visit(sc, k) {
  const page = await ctx.newPage(), errors = []; page.on('pageerror', e => errors.push(String(e).slice(0, 200)));
  const t0 = Date.now(), r = { visit: k };
  try {
    await page.goto(`http://localhost:${PORT}/?${QUERY}`, { waitUntil: 'commit' });
    await page.waitForFunction(() => !!(window.__boot && window.__boot.marks && window.__boot.marks.interactive), null, { timeout: LIMIT, polling: 100 });
    r.wall = Date.now() - t0;
    await page.evaluate(() => { try { window.__menu.close(); } catch { } });
    if (sc.stay === 'full') {
      await Promise.race([page.evaluate(() => window.__ready.then(() => true)), sleep(LIMIT)]);
      for (let i = 0, none = 0; i < LIMIT / 250; i++) { const st = await page.evaluate(() => { try { return window.__cut.programs.state; } catch { return 'unread'; } }); if (st === 'ready' || st === 'failed' || st === 'unread') break; if (st === 'none' && ++none > 32) break; await sleep(250); }
      await sleep(2000);
    } else await sleep(sc.stay * 1000);
    Object.assign(r, await page.evaluate(() => {
      const B = window.__boot, sp = {}, big = {}; for (const k in B.spans) { sp[k] = Math.round(B.spans[k]); if (k.startsWith('pipelines_') && sp[k] > 1000) big[k.slice(10)] = sp[k]; }
      let cut = null; try { const i = window.__cut.programs.info; cut = { state: i.state, buildMs: i.buildMs, pipelines: i.pipelines }; } catch { }
      const nav = performance.getEntriesByType('navigation')[0], slowFetch = performance.getEntriesByType('resource').filter(e => e.responseEnd - e.startTime > 400).map(e => [e.name.split('/').pop().slice(0, 28), Math.round(e.startTime), Math.round(e.responseEnd)]).slice(0, 4);
      return { interactive: Math.round(B.marks.interactive), scriptAt: sp.beforeScript, pipelines: sp.pipelines, slowest: (B.slowest || []).slice(0, 3).map(t => [t.label, Math.round(t.ms)]), groups: big, laterReady: B.marks.laterReady ? Math.round(B.marks.laterReady) : null, cut,
        lookFillMs: window.__engine && window.__engine.gbuffer ? Math.round(window.__engine.gbuffer.lookFillMs) : null, navResponseEnd: nav ? Math.round(nav.responseEnd) : null, slowFetch, gpuErrors: B.gpuInitErrors, at: Math.round(performance.now()) };
    }));
  } catch (e) { r.error = String(e).slice(0, 240); }
  if (errors.length) r.pageErrors = errors.slice(0, 3);
  r.stayed = Date.now() - t0; await Promise.race([page.close().catch(() => { }), sleep(5000)]);   // (a tab that will not close in 5 s is left to the browser's own close)
  return r;
}

(async () => {
  const all = { query: QUERY, browser: '', adapter: null, scenarios: [] };
  for (const sc of PLAN) {
    const dir = fs.mkdtempSync(path.join(os.tmpdir(), 'warmboot-')), args = ['--enable-unsafe-webgpu', ...(sc.kb ? [`--gpu-disk-cache-size-kb=${sc.kb}`] : [])];
    const open = async () => { ctx = await chromium.launchPersistentContext(dir, { args, viewport: { width: 1920, height: 1080 } }); await ctx.addInitScript(() => { window.__testVisible = true; }); all.browser = ctx.browser() ? ctx.browser().version() : all.browser; };
    await open(); const rows = [];
    if (!all.adapter) { const p = await ctx.newPage(); await p.goto(`http://localhost:${PORT}/?${QUERY}`, { waitUntil: 'commit' }).catch(() => { }); all.adapter = await p.evaluate(async () => { if (!navigator.gpu) return 'no navigator.gpu'; const a = await navigator.gpu.requestAdapter(); return a ? `${a.info.vendor} ${a.info.architecture} ${a.info.description || ''}`.trim() : 'no adapter'; }).catch(e => String(e).slice(0, 80)); await p.close(); log('adapter:', all.adapter); }
    log(`== ${sc.name}: ${sc.visits} visits, each stays ${sc.stay === 'full' ? 'until every shader behind the start is in' : sc.stay + ' s after interactive'}${sc.kb ? `, the shader store set to ${sc.kb} KB` : ''}${sc.restart ? ', the browser restarted between visits' : ''}`);
    for (let k = 0; k < sc.visits; k++) {
      if (sc.restart && k) { await stop(); await sleep(1500); await open(); }
      const r = await visit(sc, k); rows.push(r);
      log(r.error ? `  visit ${k}: ${r.error}` : `  visit ${k}${k ? '' : ' (cold)'}: interactive ${r.interactive} ms · script at ${r.scriptAt} · pipelines ${r.pipelines} · slowest ${r.slowest.map(s => s.join(' ')).join(', ')} · groups over 1 s ${JSON.stringify(r.groups)} · second set ${r.cut ? `${r.cut.state} ${r.cut.buildMs} ms` : '?'} · stayed ${(r.stayed / 1000).toFixed(1)} s${r.slowFetch && r.slowFetch.length ? ` · slow fetches ${JSON.stringify(r.slowFetch)}` : ''}${r.gpuErrors && r.gpuErrors.length ? ` · GPU ERRORS ${r.gpuErrors.length}` : ''}`);
    }
    await stop(); try { fs.rmSync(dir, { recursive: true, force: true }); } catch { }
    all.scenarios.push({ ...sc, rows });
    const warm = rows.slice(1).filter(r => !r.error).map(r => r.interactive);
    log(`   ${sc.name}: cold ${rows[0] && rows[0].interactive} ms, then ${warm.join(', ')} ms`);
  }
  if (OUT) fs.writeFileSync(OUT, JSON.stringify(all, null, 1));
  process.exit(0);
})().catch(async e => { console.error(e); await stop(); process.exit(1); });
