// WHERE A FRAME'S CPU GOES, ON THE REAL PAGE (a measuring tool, not a check: no PASS line, no probe runs it). Opens one address of the game in Chromium, runs a
// script in the page, and takes the browser's own sampling CPU profile of the page's thread over the stretches the script asks for:
//     await __prof.start();  …what is to be measured…  await __prof.stop('a name');
// For each stretch it prints the call tree from the top down (every branch over 2 % of the stretch, in ms) and the functions that cost most themselves, by the
// names the dev server's bundle keeps. THEN ITS DEAREST TASKS, EACH AS ITS OWN TREE (--tasks, 3 unless said): a task is one unbroken run of work on the
// page's thread, which for a frame is the frame. A sum over a stretch cannot tell one 60 ms frame from four of 15; this can.
// With --raw <dir> the profiles themselves are kept as .cpuprofile files (they open in a browser's performance panel).
//   PORT=<a dev server's port> node tools/qa/cpuprofile-page.cjs --query '<the address's words>' [--do '<js>' | env PAGE_DO='<js>'] [--out <file.json>] [--raw <dir>] [--depth 14] [--least 2] [--tasks 3]
// The picture is real only on a machine with an adapter, but the CPU's side of a frame is what is asked here. Milliseconds are the machine's own: what
// compares is one branch against another, and one address against another on the same machine.
const { chromium } = require('/opt/node-tools/node_modules/playwright');
const fs = require('fs'), path = require('path');
const argv = process.argv.slice(2), flag = (n, d) => { const i = argv.indexOf(n); return i >= 0 ? argv[i + 1] : d; };
const PORT = process.env.PORT; if (!PORT) { console.error('PORT=<the dev server\'s port> is needed'); process.exit(2); }
const QUERY = flag('--query', '').replace(/^\?/, ''), DO = flag('--do', process.env.PAGE_DO || ''), OUT = flag('--out', ''), RAW = flag('--raw', ''), DEPTH = +flag('--depth', 14), LEAST = +flag('--least', 2) / 100, LIMIT = +flag('--limit', 300) * 1000, TASKS = +flag('--tasks', 3);
const T0 = Date.now(), log = (...a) => console.log(`[${((Date.now() - T0) / 1000).toFixed(0)}s]`, ...a);
if (!DO) { console.error('nothing to run: --do \'<js>\' or PAGE_DO'); process.exit(2); }

/** a profile as a tree by function: ms of its own and with what it called; and the same for each of its dearest tasks */
function read(profile) {
  const byId = new Map(profile.nodes.map(n => [n.id, n])), kids = id => byId.get(id).children || [], root = profile.nodes[0].id;
  const fn = id => byId.get(id).callFrame.functionName, resting = id => /^\((idle|program)\)$/.test(fn(id));
  const name = n => { const f = n.callFrame, at = f.url ? `${f.url.split('/').pop().split('?')[0]}:${f.lineNumber + 1}` : ''; return `${f.functionName || '(anonymous)'}${at ? ` ${at}` : ''}`; };
  /** the tree of the samples [from, to): lines for every branch over `least` of its work, and the functions dearest by themselves */
  const tree = (from, to, least) => {
    const self = new Map(); let all = 0, rest = 0, gc = 0;
    for (let i = from; i < to; i++) { const id = profile.samples[i], d = (profile.timeDeltas[i] || 0) / 1000; self.set(id, (self.get(id) || 0) + d); all += d; if (resting(id)) rest += d; else if (fn(id) === '(garbage collector)') gc += d; }
    const total = new Map(); const sum = id => { let s = self.get(id) || 0; for (const c of kids(id)) s += sum(c); total.set(id, s); return s; }; sum(root);
    const busy = all - rest, lines = [];
    const walk = (id, depth) => { const s = total.get(id); if (depth > 0 && (s < busy * least || s <= 0 || depth > DEPTH || resting(id))) return;
      if (depth > 0) lines.push(`${'  '.repeat(depth - 1)}${s.toFixed(1)} ms (${(self.get(id) || 0).toFixed(1)} its own)  ${name(byId.get(id))}`);
      for (const c of [...kids(id)].sort((x, y) => total.get(y) - total.get(x))) walk(c, depth + 1); }; walk(root, 0);
    const own = new Map(); for (const [id, v] of self) { if (resting(id)) continue; const k = name(byId.get(id)); own.set(k, (own.get(k) || 0) + v); }
    return { ms: +all.toFixed(1), busy: +busy.toFixed(1), gc: +gc.toFixed(1), tree: lines, own: [...own].sort((x, y) => y[1] - x[1]).slice(0, 24).map(([k, v]) => [k, +v.toFixed(1)]) };
  };
  const n = profile.samples.length, whole = tree(0, n, LEAST);
  // the tasks: runs of samples at work, a rest of under a third of a millisecond inside one not ending it
  const runs = []; let t = 0, start = -1, startAt = 0, lastBusy = 0, lastBusyAt = 0, work = 0;
  for (let i = 0; i < n; i++) { const d = (profile.timeDeltas[i] || 0) / 1000; t += d;
    if (!resting(profile.samples[i])) { if (start < 0) { start = i; startAt = t - d; work = 0; } work += d; lastBusy = i; lastBusyAt = t; }
    else if (start >= 0 && t - lastBusyAt > 0.33) { runs.push({ from: start, to: lastBusy + 1, at: startAt, work }); start = -1; } }
  if (start >= 0) runs.push({ from: start, to: lastBusy + 1, at: startAt, work });
  const dearest = runs.sort((x, y) => y.work - x.work).slice(0, Math.max(0, TASKS)).sort((x, y) => x.at - y.at).map(r => ({ at: +r.at.toFixed(1), ...tree(r.from, r.to, Math.max(LEAST, 0.04)) }));
  return { ...whole, tasksInAll: runs.length, tasks: dearest };
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
    log(`== ${tag}: ${r.ms} ms profiled, ${r.busy} ms of them at work (${r.gc} ms collecting garbage)`); for (const l of r.tree) console.log('   ' + l); console.log('   — costing most themselves: ' + r.own.slice(0, 14).map(o => `${o[0]} ${o[1]}`).join(' · '));
    for (const k of r.tasks) { console.log(`   -- a task ${k.at} ms into the stretch: ${k.busy} ms at work (${k.gc} ms collecting garbage)`); for (const l of k.tree) console.log('      ' + l); console.log('      — costing most themselves: ' + k.own.slice(0, 8).map(o => `${o[0]} ${o[1]}`).join(' · ')); } });
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
