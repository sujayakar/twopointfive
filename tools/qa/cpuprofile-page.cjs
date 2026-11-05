// WHERE A FRAME'S CPU GOES, ON THE REAL PAGE (a measuring tool, not a check: no PASS line, no probe runs it). Opens one address of the game in Chromium, runs a
// script in the page, and takes the browser's own sampling CPU profile of the page's thread over the stretches the script asks for:
//     await __prof.start();  …what is to be measured…  await __prof.stop('a name');
// For each stretch it prints the call tree from the top down (every branch over 2 % of the stretch, in ms) and the functions that cost most themselves, by the
// names the dev server's bundle keeps. With --raw <dir> the profiles themselves are kept as .cpuprofile files (they open in a browser's performance panel).
//   PORT=<a dev server's port> node tools/qa/cpuprofile-page.cjs --query '<the address's words>' [--do '<js>' | env PAGE_DO='<js>'] [--out <file.json>] [--raw <dir>] [--depth 14] [--least 2]
// The picture is real only on a machine with an adapter, but the CPU's side of a frame is what is asked here. Milliseconds are the machine's own: what
// compares is one branch against another, and one address against another on the same machine.
const { chromium } = require('/opt/node-tools/node_modules/playwright');
const fs = require('fs'), path = require('path');
const argv = process.argv.slice(2), flag = (n, d) => { const i = argv.indexOf(n); return i >= 0 ? argv[i + 1] : d; };
const PORT = process.env.PORT; if (!PORT) { console.error('PORT=<the dev server\'s port> is needed'); process.exit(2); }
const QUERY = flag('--query', '').replace(/^\?/, ''), DO = flag('--do', process.env.PAGE_DO || ''), OUT = flag('--out', ''), RAW = flag('--raw', ''), DEPTH = +flag('--depth', 14), LEAST = +flag('--least', 2) / 100, LIMIT = +flag('--limit', 300) * 1000;
const T0 = Date.now(), log = (...a) => console.log(`[${((Date.now() - T0) / 1000).toFixed(0)}s]`, ...a);
if (!DO) { console.error('nothing to run: --do \'<js>\' or PAGE_DO'); process.exit(2); }

/** a profile as a tree by function: ms of its own and with what it called */
function read(profile) {
  const byId = new Map(profile.nodes.map(n => [n.id, n])), parent = new Map(); for (const n of profile.nodes) for (const c of n.children || []) parent.set(c, n.id);
  const self = new Map(); let all = 0; profile.samples.forEach((id, i) => { const d = (profile.timeDeltas[i] || 0) / 1000; self.set(id, (self.get(id) || 0) + d); all += d; });
  const total = new Map(); const sum = id => { let t = self.get(id) || 0; for (const c of byId.get(id).children || []) t += sum(c); total.set(id, t); return t; }; sum(profile.nodes[0].id);
  const name = n => { const f = n.callFrame, at = f.url ? `${f.url.split('/').pop().split('?')[0]}:${f.lineNumber + 1}` : ''; return `${f.functionName || '(anonymous)'}${at ? ` ${at}` : ''}`; };
  const idle = profile.nodes.filter(n => /^\((idle|program|garbage collector)\)$/.test(n.callFrame.functionName));
  const busy = all - idle.filter(n => n.callFrame.functionName !== '(garbage collector)').reduce((s, n) => s + (self.get(n.id) || 0), 0);
  const lines = []; const walk = (id, depth) => { const n = byId.get(id), t = total.get(id); if (depth > 0 && (t < busy * LEAST || depth > DEPTH || /^\((idle|program)\)$/.test(n.callFrame.functionName))) return;
    if (depth > 0) lines.push(`${'  '.repeat(depth - 1)}${t.toFixed(1)} ms (${(self.get(id) || 0).toFixed(1)} its own)  ${name(n)}`);
    for (const c of [...(n.children || [])].sort((a, b) => total.get(b) - total.get(a))) walk(c, depth + 1); }; walk(profile.nodes[0].id, 0);
  const own = new Map(); for (const n of profile.nodes) { const k = name(n); own.set(k, (own.get(k) || 0) + (self.get(n.id) || 0)); }
  const top = [...own].filter(([k]) => !/^\((idle|program)\)/.test(k)).sort((a, b) => b[1] - a[1]).slice(0, 24).map(([k, v]) => [k, +v.toFixed(1)]);
  return { ms: +all.toFixed(1), busy: +busy.toFixed(1), gc: +idle.filter(n => n.callFrame.functionName === '(garbage collector)').reduce((s, n) => s + (self.get(n.id) || 0), 0).toFixed(1), tree: lines, own: top };
}

(async () => {
  const browser = await chromium.launch({ args: ['--enable-unsafe-webgpu'] }), ctx = await browser.newContext({ viewport: { width: 1920, height: 1080 } });
  await ctx.addInitScript(() => { window.__testVisible = true; });
  const page = await ctx.newPage(), cdp = await ctx.newCDPSession(page), errors = [], said = [], takes = [];
  page.on('pageerror', e => { errors.push(String(e).slice(0, 300)); log('PAGEERROR', String(e).slice(0, 200)); });
  page.on('console', m => { const x = m.text(); if (/^[A-Z]{4,} \{/.test(x)) said.push(x); if (m.type() === 'error') errors.push(x.slice(0, 300)); });
  await cdp.send('Profiler.enable'); await cdp.send('Profiler.setSamplingInterval', { interval: 100 });
  await page.exposeFunction('__profStart', async () => { await cdp.send('Profiler.start'); });
  await page.exposeFunction('__profStop', async tag => { const { profile } = await cdp.send('Profiler.stop'); const r = read(profile); takes.push({ tag: String(tag), ...r });
    if (RAW) { fs.mkdirSync(RAW, { recursive: true }); fs.writeFileSync(path.join(RAW, `${String(tag).replace(/[^\w.-]+/g, '-')}.cpuprofile`), JSON.stringify(profile)); }
    log(`== ${tag}: ${r.ms} ms profiled, ${r.busy} ms of them at work (${r.gc} ms collecting garbage)`); for (const l of r.tree) console.log('   ' + l); console.log('   — costing most themselves: ' + r.own.slice(0, 14).map(o => `${o[0]} ${o[1]}`).join(' · ')); });
  await page.addInitScript(() => { window.__prof = { start: () => window.__profStart(), stop: tag => window.__profStop(tag) }; });
  log('goto', QUERY); await page.goto(`http://localhost:${PORT}/?${QUERY}`, { waitUntil: 'commit' });
  await page.waitForFunction(() => !!window.__ready, null, { timeout: LIMIT }); await Promise.race([page.evaluate(() => window.__ready.then(() => true)), new Promise((_, rej) => setTimeout(() => rej(new Error('the page was not ready in time')), LIMIT))]);
  await page.evaluate(() => { try { window.__menu.close(); } catch { } });
  const adapter = await page.evaluate(async () => { const a = navigator.gpu && await navigator.gpu.requestAdapter(); return a ? `${a.info.vendor} ${a.info.architecture}` : 'no adapter'; });
  log('ready on', adapter, '· running the script');
  await Promise.race([page.evaluate(`(async () => { ${DO} })()`), new Promise((_, rej) => setTimeout(() => rej(new Error('the script did not end in time')), LIMIT))]);
  for (const s of said) console.log(s.slice(0, 4000));
  if (OUT) fs.writeFileSync(OUT, JSON.stringify({ query: QUERY, adapter, browser: browser.version(), errors, said, takes }, null, 1));
  await browser.close(); process.exit(errors.length ? 1 : 0);
})().catch(e => { console.error(e); process.exit(1); });
