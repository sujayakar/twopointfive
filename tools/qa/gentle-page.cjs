// A GENTLE SECOND TRY (src/gpu/device.ts GENTLE), on the real page in headless Chromium, software WebGPU adapter: the browser's GPU process is killed while the
// page compiles its shaders (the debugging protocol's Browser.crashGpuProcess, the one way a real loss can be made here), and the page must note the loss, load again by itself and
// compile ONE program at a time on that load, saying so. The browser is launched so that the page does not see a driven one (navigator.webdriver false): a page a
// script drives leaves no note, by the rule.
//   node tools/qa/gentle-page.cjs        env: PORT (the dev server's, default 5199) · GENTLE_WAIT_S (how long the gentle load may take to come ready, default 0 = not waited for)
// What this is NOT: a machine that loses its device by itself. Whether one at a time gets such a machine through is its own records' to say (docs/agents/qa.md).
// Exit 0 = every check passed, 1 = one failed or a page did not get that far.   GENTLE_BREAK=1: the page runs under ?gentle=0 — must FAIL.
const { chromium } = require('/opt/node-tools/node_modules/playwright');
const PORT = process.env.PORT || 5199, BREAK = process.env.GENTLE_BREAK === '1', WAIT_S = Number(process.env.GENTLE_WAIT_S || 0);
const FLAGS = ['--enable-unsafe-webgpu', '--use-gl=angle', '--use-angle=swiftshader', '--enable-features=Vulkan', '--use-vulkan=swiftshader', '--ignore-gpu-blocklist', '--disable-gpu-watchdog', '--disable-blink-features=AutomationControlled'];
const T0 = Date.now(), log = (...a) => console.log(`[${((Date.now() - T0) / 1000).toFixed(0)}s]`, ...a);
let red = 0; const check = (ok, what) => { console.log(`${ok ? '- PASS' : '- FAIL'} — ${what}`); if (!ok) red++; };
// (its own device and no frame around it, no backend, START not held: the page as plain as a person's can be)
const URL = `http://localhost:${PORT}/?quality=verylow&levelFile=train-move.json&quiet&keeppage=0&settle=0&cloud=0${BREAK ? '&gentle=0' : ''}`;
const sleep = ms => new Promise(r => setTimeout(r, ms));

(async () => {
  const browser = await chromium.launch({ headless: true, channel: 'chromium', args: FLAGS });
  const ctx = await browser.newContext({ viewport: { width: 320, height: 180 }, deviceScaleFactor: 1 });
  await ctx.addInitScript(() => { window.__testVisible = true; });
  const page = await ctx.newPage(); const said = [];
  page.on('console', m => said.push({ type: m.type(), text: m.text().slice(0, 400), at: Date.now() }));
  log(URL); await page.goto(URL, { waitUntil: 'commit' });
  check(await page.evaluate(() => navigator.webdriver !== true), 'the page does not see a driven browser');
  // the boot's compiles under way: some in, most not
  let seen = null;
  for (let i = 0; i < 600 && !seen; i++) { await sleep(500); seen = await page.evaluate(() => { const m = /compiling shaders (\d+) \/ (\d+)/.exec(document.body.innerText); return m && +m[2] > 20 && +m[1] >= 2 && +m[1] < +m[2] - 10 ? [+m[1], +m[2]] : null; }).catch(() => null); }
  if (!seen) { check(false, 'the page never showed its shader count'); await browser.close(); process.exit(1); }
  log(`compiling shaders ${seen[0]} / ${seen[1]}: the GPU process is killed now`);
  check(await page.evaluate(() => localStorage.getItem('twopointsix.gentle')) === null && await page.evaluate(() => window.__pipeQueue === undefined), 'before any loss: no note, and no queue in front of this browser');
  const before = await page.evaluate(() => performance.timeOrigin);
  const cdp = await browser.newBrowserCDPSession(); await cdp.send('Browser.crashGpuProcess').catch(e => log('crashGpuProcess:', String(e).slice(0, 160)));
  // the page tells the loss, notes it and loads itself again
  let again = false;
  for (let i = 0; i < 240 && !again; i++) { await sleep(500); again = await page.evaluate(t => performance.timeOrigin !== t, before).catch(() => false); }
  const lost = said.find(s => /WebGPU device lost/.test(s.text));
  check(!!lost, `the page was told the loss (${lost ? lost.text.slice(0, 120) : 'nothing printed'})`);
  check(again, 'and loaded itself again');
  if (!again) { const card = await page.evaluate(() => document.getElementById('fatal')?.textContent ?? '').catch(() => ''); log('card:', card.slice(0, 300)); await browser.close(); console.log(`${red} FAILED`); process.exit(1); }
  await page.waitForFunction(() => document.readyState !== 'loading' && /compiling shaders \d+ \/ \d+/.test(document.body.innerText) || !!window.__boot, null, { timeout: 120000 }).catch(() => { /* read what is there */ });
  const raw = await page.evaluate(() => localStorage.getItem('twopointsix.gentle')), note = Number((raw || '').split(':')[1]);
  check(note > 0 && Math.abs(Date.now() - note) < 300000 && /^1:/.test(raw), `the loss while compiling is noted in the browser, as its first (${raw || 'no note'})`);
  // the second load: the queue is on, one in flight, and it says so
  const samples = [];
  for (let i = 0; i < 40; i++) { await sleep(500); const q = await page.evaluate(() => { const Q = window.__pipeQueue; return Q ? { limit: Q.limit(), inflight: Q.inflight, waiting: Q.waiting.length, phase: Q.phase } : null; }).catch(() => null); if (q) samples.push(q); }
  const most = Math.max(-1, ...samples.map(s => s.inflight)), queuedUp = Math.max(-1, ...samples.map(s => s.waiting));
  check(samples.length > 0 && samples.every(s => s.limit === 1), `the second load has the queue on with one in flight allowed (${samples.length ? samples[0].limit : 'no queue'})`);
  check(most === 1 && queuedUp > 5, `and over twenty seconds never more than one program was with the browser while others waited (${most} at the most, ${queuedUp} waiting at the most)`);
  const warn = said.find(s => s.at > note && /shaders compile one at a time on this load/.test(s.text));
  check(!!warn && warn.type === 'warning', `it prints the warning the records keep (${warn ? warn.text.slice(0, 160) : 'none'})`);
  check(!(await page.evaluate(() => { const f = document.getElementById('fatal'); return !!f && f.style.display !== 'none' && !!f.textContent; })), 'and no red card is up');
  if (WAIT_S > 0) {
    log(`waiting up to ${WAIT_S} s for the gentle load to come ready`);
    const ok = await page.evaluate(ms => Promise.race([window.__ready.then(() => true), new Promise(r => setTimeout(() => r(false), ms))]), WAIT_S * 1000).catch(() => false);
    const b = ok ? await page.evaluate(() => ({ ms: window.__boot.marks.interactive, p: window.__boot.pipelines, q: { inflight: window.__pipeQueue.inflight, waiting: window.__pipeQueue.waiting.length } })) : null;
    // (ready = the boot's and the later set's programs are in; the background set goes on behind it, one at a time)
    check(ok && b.p.failed === 0 && b.p.done === b.p.total && b.q.inflight <= 1, `the gentle load came ready with every program of its start made (${b ? `${Math.round(b.ms / 1000)} s to the page, ${b.p.done} of ${b.p.total}, ${b.p.failed} failed, ${b.q.inflight} in flight and ${b.q.waiting} of the background set waiting` : 'not ready in time'})`);
  }
  await browser.close();
  console.log(red ? `${red} FAILED` : 'ALL PASS');
  process.exit(red ? 1 : 0);
})().catch(e => { console.error(e); process.exit(1); });
