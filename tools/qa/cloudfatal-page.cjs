// A RED CARD TELLS THE BACKEND (src/main.ts FATAL.tell → src/cloud/cloud.ts cloudFatal; the card: src/gpu/device.ts showFatal), on the real page in headless
// Chromium, software WebGPU adapter, against a STAND-IN backend on this machine (cloud/test/standin.ts: `npm ci` in cloud/ first). `requestAdapter` is stubbed
// before the page's own script runs (?stub=none, as tools/qa/nogpu-page.cjs does), so the page dies before the game is up:
//   1. a fresh browser on a page with the backend's address BUILT IN, as a hosted build has (`__TPS_CLOUD__` set before the page's script; its requests are
//      handed to the stand-in — an address on the address bar would count as the browser's own choice and wait for no notice): the red card carries the line on what the game sends, on top; not one request before the line has
//      been in view; then the visit's record (not started) and the `fatal` (stage adapter, the machine's facts, the build's commit) are on the backend
//   2. the same browser again: no line (it has been told), and the record goes at once
//   3. a fresh browser under ?cloudstats=0, and one with no backend on the address: the card as it was and nothing heard
//   4. a page that BOOTED and then threw (CLOUDFATAL_ONLY=card leaves this out: it boots the game, minutes on this adapter): the `fatal` is on the visit that
//      was up, with its stage and how long it had played
//   PORT=5321 [CLOUD_PORT=3217] node tools/qa/cloudfatal-page.cjs        (a dev server on PORT: PORT=5321 bun run tools/serve.ts)
// Nothing here is a machine that really has no adapter, a real GPU or the real service. Exit 0 = every check passed, 1 = one failed.
//   CLOUDFATAL_BREAK=1: every page runs under ?cloudstats=0 — must FAIL.
const { chromium } = require('/opt/node-tools/node_modules/playwright');
const path = require('node:path'), cp = require('node:child_process');
const PORT = process.env.PORT || 5199, CPORT = Number(process.env.CLOUD_PORT) || 3217, ROOT = path.resolve(__dirname, '../..'), ONLY = process.env.CLOUDFATAL_ONLY || '';
const BREAK = process.env.CLOUDFATAL_BREAK === '1' ? '&cloudstats=0' : '', NAV = Math.max(60000, Number(process.env.NAV_MS) || 480000);
const FLAGS = ['--enable-unsafe-webgpu', '--use-gl=angle', '--use-angle=swiftshader', '--enable-features=Vulkan', '--use-vulkan=swiftshader', '--ignore-gpu-blocklist', '--disable-gpu-watchdog'];
const CLOUD = `http://127.0.0.1:${CPORT}`, KEY = 'page-check-key', BUILT = 'https://built-in.example';
const T0 = Date.now(), log = (...a) => console.log(`[${((Date.now() - T0) / 1000).toFixed(0)}s]`, ...a);
let red = 0; const check = (ok, what) => { console.log(`${ok ? '- PASS' : '- FAIL'} — ${what}`); if (!ok) red++; };
const sleep = ms => new Promise(r => setTimeout(r, ms));
const admin = async (fn, args) => { const r = await fetch(`${CLOUD}/api/query`, { method: 'POST', headers: { 'Content-Type': 'application/json', Authorization: `Convex ${KEY}` }, body: JSON.stringify({ path: fn, format: 'convex_encoded_json', args: [args] }) }); const j = await r.json(); if (j.status !== 'success') throw new Error(`${fn}: ${JSON.stringify(j).slice(0, 200)}`); return j.value; };
const heard = async () => JSON.stringify((await (await fetch(`${CLOUD}/__stats`)).json()).calls);
const fatals = () => admin('telemetry:list', { kind: 'fatal' });
const until = async (f, ms = 15000) => { const t = Date.now(); for (;;) { const v = await f(); if (v || Date.now() - t > ms) return v; await sleep(250); } };
(async () => {
  const standin = cp.spawn('bun', ['run', 'test/standin.ts', String(CPORT)], { cwd: path.join(ROOT, 'cloud'), env: { ...process.env, STANDIN_KEY: KEY }, stdio: 'ignore' });
  for (let i = 0; i < 50; i++) { try { await heard(); break; } catch { await sleep(200); } }
  const browser = await chromium.launch({ headless: true, channel: 'chromium', args: FLAGS });
  const profile = async (built = true) => {
    const ctx = await browser.newContext({ viewport: { width: 1000, height: 640 }, deviceScaleFactor: 1 });
    if (built) {
      await ctx.addInitScript(() => { window.__TPS_CLOUD__ = 'https://built-in.example'; });
      await ctx.route(BUILT + '/**', async route => {
        const req = route.request(), cors = { 'Access-Control-Allow-Origin': '*', 'Access-Control-Allow-Headers': '*', 'Access-Control-Allow-Methods': 'POST, OPTIONS' };
        if (req.method() === 'OPTIONS') return route.fulfill({ status: 204, headers: cors });
        try { const r = await fetch(CLOUD + new URL(req.url()).pathname, { method: 'POST', headers: { 'Content-Type': 'application/json' }, body: req.postData() ?? '{}' }); await route.fulfill({ status: r.status, headers: { ...cors, 'Content-Type': 'application/json' }, body: await r.text() }); }
        catch { await route.abort().catch(() => {}); }
      });
    }
    await ctx.addInitScript(() => {
      window.__testVisible = true;
      if (new URLSearchParams(location.search).get('stub') !== 'none' || !navigator.gpu) return;
      const real = navigator.gpu.requestAdapter.bind(navigator.gpu);
      navigator.gpu.requestAdapter = o => Promise.resolve(o?.forceFallbackAdapter ? real(o) : null);
    });
    const page = await ctx.newPage();
    const card = () => page.evaluate(() => { const el = document.getElementById('fatal'), n = el?.querySelector('.cloudnote'), r = n?.getBoundingClientRect();
      return { up: el?.style.display === 'flex', text: (el?.innerText ?? '').slice(0, 200), line: n?.textContent ?? '', first: !!n && el.firstElementChild === n,
        top: !!n && !!r && document.elementFromPoint(r.left + r.width / 2, r.top + r.height / 2) === n, told: localStorage.getItem('twopointsix.cloud.told'), pid: localStorage.getItem('twopointsix.cloud.pid') }; });
    const dead = async (words) => { await page.goto(`http://localhost:${PORT}/?paused&quality=low&mute&stub=none${words}${BREAK}`, { waitUntil: 'load' }); await page.waitForFunction(() => document.getElementById('fatal')?.style.display === 'flex', null, { timeout: 60000 }); };
    return { ctx, page, card, dead };
  };
  try {
    const commit = (await (await fetch(`http://localhost:${PORT}/qa/commit`)).text()).trim();
    // ---- 1. a fresh browser: the line on the card, nothing before it has been read, then the two records
    const A = await profile(), before = await heard();
    await A.dead(''); await sleep(1700);   // (the commit's wait is a second and a half at most; the line needs two seconds in view after it)
    const c1 = await A.card(), early = await heard();
    check(c1.up && /sends your replays, stats, settings and saved files/.test(c1.line) && c1.first && c1.top && c1.told !== '1' && early === before,
      `a page that never got a GPU shows what the game sends on its red card, first and on top, and has made no request yet ("${c1.line.slice(0, 40)}…"; heard ${early})`);
    const f1 = await until(async () => { const F = await fatals(); return F.length ? F : null; }), r1 = f1?.[0];
    const v1 = r1 ? (await admin('telemetry:list', { sid: r1.sid })).find(r => r.kind === 'visit') : null, c1b = await A.card();
    check(!!r1 && f1.length === 1 && /^fatal-/.test(r1.sid) && r1.data.stage === 'adapter' && typeof r1.data.message === 'string' && r1.data.message.length > 0 && Array.isArray(r1.data.stack) && !!r1.data.data?.facts && typeof r1.data.data.facts === 'object'
      && !!v1 && v1.data.started === false && c1b.told === '1' && r1.pid === c1b.pid,
      `once the line has been read the visit's record (not started) and the fatal are on the backend (stage ${r1?.data?.stage}, "${String(r1?.data?.message).slice(0, 60)}", ${Object.keys(r1?.data?.data?.facts ?? {}).length} facts of the machine)`);
    check(!commit || (!!r1 && r1.build === commit && v1?.build === commit), `both carry the build's commit, asked of the page's own server (${r1?.build ?? 'none'}; the server says ${commit || 'none'})`);
    // ---- 2. the same browser again: told, so no line and no wait
    await A.dead('');
    const f2 = await until(async () => { const F = await fatals(); return F.length >= 2 ? F : null; }), c2 = await A.card();
    check(!!f2 && f2.length === 2 && c2.up && c2.line === '' && f2[0].pid === c1b.pid && f2[0].sid !== f2[1].sid, `the same browser again: no line on the card, and its record goes at once under a visit of its own (${f2?.length} fatal records)`);
    await A.ctx.close();
    // ---- 3. the stats' word off, and no backend at all: the card as it was, nothing heard
    const B = await profile(), quiet0 = await heard();
    await B.dead('&cloudstats=0'); await sleep(4500); const c3 = await B.card(); await B.ctx.close();
    const B2 = await profile(false); await B2.dead(''); await sleep(4500); const c4 = await B2.card(), quiet1 = await heard();
    check(c3.up && c3.line === '' && c4.up && c4.line === '' && c4.pid === null && quiet1 === quiet0 && (await fatals()).length === (f2?.length ?? 0), `under ?cloudstats=0, and on a page with no backend, the card is as it was and nothing is heard (${quiet1})`);
    await B2.ctx.close();
    // ---- 4. a page that booted, then threw
    if (ONLY !== 'card') {
      const C = await profile(); log('booting the game…');
      await C.page.goto(`http://localhost:${PORT}/?paused&quality=verylow&mute&roomclear${BREAK}`, { waitUntil: 'load', timeout: NAV });
      const up = await C.page.evaluate(async () => { await window.__ready; const e = window.__engine; e.render = () => {}; try { window.__smoke.params.enabled = false; } catch { /* */ }
        window.__menu.close(); await window.__runFrames(120, 1 / 60, []); return { sid: window.__cloud?.sid ?? null, fatal: document.getElementById('fatal')?.style.display === 'flex' }; });
      await C.page.evaluate(() => { setTimeout(() => { throw new Error('page check: thrown on purpose'); }, 0); });
      await C.page.waitForFunction(() => document.getElementById('fatal')?.style.display === 'flex', null, { timeout: 30000 });
      const c5 = await C.card();
      const r5 = await until(async () => (await fatals()).find(r => r.sid === up.sid) ?? null, 20000);
      check(!up.fatal && !!up.sid && c5.up && /sends your replays/.test(c5.line) && !!r5 && r5.data.stage === 'run' && /thrown on purpose/.test(r5.data.message + ' ' + (r5.data.stack ?? []).join(' ')) && typeof r5.data.frames === 'number',
        `a page that was up and then threw puts its fatal on the visit that was up (stage ${r5?.data?.stage}, ${r5?.data?.frames} frames, ${r5?.data?.played} s played; the line on the card: ${c5.line ? 'yes' : 'no'})`);
      await C.ctx.close();
    }
  } catch (e) { check(false, `the check itself threw: ${String(e && e.stack || e).slice(0, 400)}`); }
  await browser.close(); try { standin.kill(); } catch { /* */ }
  console.log(red ? `\n${red} FAILED` : '\nall pass'); process.exit(red ? 1 : 0);
})();
