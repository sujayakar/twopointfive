// THE HOSTED PAGE'S BACKEND, ON THE REAL PAGE (src/cloud/; tools/qa/cloud-check.ts holds the same headless): headless Chromium on the software WebGPU adapter
// (tools/qa/page-look.cjs's recipe), a STAND-IN backend on this machine (cloud/test/standin.ts: the real functions in the service's test library — `npm ci` in
// cloud/ first), five page loads in three browser profiles —
//   1. a mission page pointed at the stand-in (`?cloud=http://127.0.0.1:<port>`) is played with no key pressed: its kept tape, the visit's record and the
//      browser's settings are on the backend afterwards, and the tape reads back out (tools/cloud/pull.ts) as a replay with as many seconds as were kept
//   2. a FRESH browser profile opens the first one's link (`__cloud.link()`): it is asked (the browser's own dialog), and on yes takes the id and the settings
//      before the game reads a key, and its address no longer carries the word
//   3. the first profile again with NO word on the address: `__cloud` is null and the stand-in hears nothing (the build as it was)
//   4. a THIRD fresh profile on a page that has the address BUILT IN, as a hosted build has (`__TPS_CLOUD__` set before the page's script; its requests are
//      handed to the stand-in): a mission page that never shows the title plays with not one request made (the browser has not been shown the notice);
//      the title page then has the notice's line, the browser is marked told, and it sends
//   PORT=5311 [CLOUD_PORT=3210] [NAV_MS=480000] node tools/qa/cloud-page.cjs      (a dev server on PORT: PORT=5311 bun run tools/serve.ts; five boots of some
//   minutes each on the software adapter)
// NOT SEEN HERE: a real deployment, a real GPU, the hosted build itself (step 4 sets the address the build would carry on the dev server's page).
// exit 0 = all of it · 1 = otherwise / a page error
const { chromium } = require('/opt/node-tools/node_modules/playwright');
const path = require('node:path'), cp = require('node:child_process'), os = require('node:os'), fs = require('node:fs');
const PORT = process.env.PORT || 5199, CPORT = Number(process.env.CLOUD_PORT) || 3210, QUALITY = process.env.QUALITY || 'verylow', ROOT = path.resolve(__dirname, '../..');
const FLAGS = ['--enable-unsafe-webgpu', '--use-gl=angle', '--use-angle=swiftshader', '--enable-features=Vulkan', '--use-vulkan=swiftshader', '--ignore-gpu-blocklist', '--disable-gpu-watchdog'];
const NOISE = /""?smoke[A-Za-z0-9_]*""?|\[pipeline\] smoke_|number of storage textures \(5\) in the Compute stage/;
const CLOUD = `http://127.0.0.1:${CPORT}`, KEY = 'page-check-key';
let failed = false; const check = (ok, text) => { console.log(`- ${ok ? 'PASS' : 'FAIL'} — ${text}`); if (!ok) failed = true; };
const sleep = ms => new Promise(r => setTimeout(r, ms)), FR = Math.max(30, Number(process.env.FRAMES) || 240), NAV = Math.max(60000, Number(process.env.NAV_MS) || 480000);
const admin = async (kind, fn, args) => { const r = await fetch(`${CLOUD}/api/${kind}`, { method: 'POST', headers: { 'Content-Type': 'application/json', Authorization: `Convex ${KEY}` }, body: JSON.stringify({ path: fn, format: 'convex_encoded_json', args: [args] }) }); const j = await r.json(); if (j.status !== 'success') throw new Error(`${fn}: ${j.errorMessage}`); return j.value; };
const heard = async () => (await (await fetch(`${CLOUD}/__stats`)).json()).calls;
(async () => {
  const standin = cp.spawn('bun', ['run', 'test/standin.ts', String(CPORT)], { cwd: path.join(ROOT, 'cloud'), env: { ...process.env, STANDIN_KEY: KEY }, stdio: 'ignore' });
  for (let i = 0; i < 50; i++) { try { await heard(); break; } catch { await sleep(200); } }
  const browser = await chromium.launch({ headless: true, channel: 'chromium', args: FLAGS });
  const errors = [];
  const profile = async () => {
    const ctx = await browser.newContext({ viewport: { width: 960, height: 540 }, deviceScaleFactor: 1 }), page = await ctx.newPage();
    page.on('console', m => { if (m.type() === 'error' && !NOISE.test(m.text()) && !/Failed to load resource/.test(m.text())) errors.push(m.text().slice(0, 300)); });
    page.on('pageerror', x => errors.push('pageerror: ' + String(x && x.stack || x).slice(0, 400)));
    const asked = []; page.on('dialog', d => { asked.push(d.message()); d.accept().catch(() => {}); });   // (the one question a page asks: a browser handed another's link)
    // (leaving a page: it stops drawing first — on this adapter a page with drawn frames queued does not let a navigation commit for minutes; tools/qa/replaykeep-page.cjs)
    const go = async (url) => { await page.evaluate(async () => { const e = window.__engine; if (!e) return; e.render = () => {}; try { if (window.__smoke) window.__smoke.params.enabled = false; await Promise.race([e.device.queue.onSubmittedWorkDone(), new Promise(r => setTimeout(r, 20000))]); } catch { /* */ } }).catch(() => {}); await page.goto(url, { timeout: NAV }); };
    const ready = () => page.evaluate(async () => { await window.__ready; return { fatal: document.getElementById('fatal')?.textContent || '', search: location.search, cloud: window.__cloud ? { url: window.__cloud.url, pid: window.__cloud.pid, sid: window.__cloud.sid, share: window.__cloud.share } : null }; });
    const play = (frames) => page.evaluate(async (n) => { window.__menu.close(); window.__smoke.params.enabled = false; const e = window.__engine, real = e.render; e.render = () => {};
      try { await window.__runFrames(n, 1 / 60, ['KeyW']); } finally { e.render = real; } return { n: window.__replay.buf.n }; }, frames);
    return { ctx, page, go, ready, play, asked };
  };
  try {
    // ---- 1. a mission page pointed at the stand-in, played
    const A = await profile();
    await A.page.addInitScript(() => { try { if (!localStorage.getItem('twopointsix.options')) localStorage.setItem('twopointsix.options', JSON.stringify({ vol: { master: 0.31, sfx: 1, ambience: 1, music: 0.5 }, hints: false })); } catch { /* */ } });
    await A.go(`http://localhost:${PORT}/?paused&quality=${QUALITY}&mute&roomclear&squad4&cloud=${encodeURIComponent(CLOUD)}`);
    const a = await A.ready();
    check(!a.fatal && !!a.cloud && a.cloud.url === CLOUD && a.cloud.share === true && /^[a-z0-9]{24,64}$/.test(a.cloud.pid), `a page pointed at this machine's backend booted with one (${a.cloud?.url}, this browser's id ${a.cloud?.pid.length} characters)${a.fatal ? ' · FATAL ' + a.fatal : ''}`);
    await A.play(FR); await sleep(6500); await A.play(FR); await sleep(6500);
    const up = await A.page.evaluate(async () => { const K = await window.__replay.kept(); await window.__cloud.sync(); await window.__cloud.sync(); const me = K.rows.find(r => r.sid === window.__cloud.sid); return { kept: me ? { pieces: me.pieces, seconds: me.seconds } : null, stats: window.__cloud.stats(), summary: window.__cloud.summary(), link: window.__cloud.link() }; });
    const rows = await admin('query', 'tapes:list', { pid: a.cloud.pid }), row = rows.find(r => r.sid === a.cloud.sid);
    check(!!up.kept && up.kept.pieces >= 2 && !!row && row.have === up.kept.pieces && up.stats.tapes.failed === 0 && up.stats.wire.failed === 0, `nothing pressed: the ${up.kept?.pieces} pieces this browser kept of the visit (${up.kept?.seconds} s) are on the backend (${row?.have} there; ${up.stats.tapes.calls} calls, ${(up.stats.tapes.bytes / 1024).toFixed(0)} KB)`);
    // the tape reads back out as a replay (the reading tool under bun, against the stand-in)
    const out = fs.mkdtempSync(path.join(os.tmpdir(), 'cloud-page-')), pull = cp.spawnSync('bun', ['run', 'tools/cloud/pull.ts', '--url', CLOUD, '--key', KEY, '--out', out, '--quiet'], { cwd: ROOT, encoding: 'utf8' });
    const dump = fs.readdirSync(out).find(f => /\.ndjson\.gz$/.test(f)), dumpOut = path.join(out, 'unpacked');
    const read = dump ? cp.spawnSync('bun', ['run', 'tools/qa/replay-dump.ts', path.join(out, dump), '--out', dumpOut, '--quiet'], { cwd: ROOT, encoding: 'utf8' }) : null;
    const index = read && fs.existsSync(path.join(dumpOut, 'index.json')) ? JSON.parse(fs.readFileSync(path.join(dumpOut, 'index.json'), 'utf8')) : null, text = JSON.stringify(index ?? {});
    check(pull.status === 0 && !!dump && !!read && read.status === 0 && text.includes(a.cloud.sid), `the tape reads back out of the backend as a replay the dump reader unpacks (${dump ?? pull.stderr?.slice(0, 200) ?? 'no file'}${read && read.status !== 0 ? ' · ' + String(read.stderr || read.stdout).slice(0, 200) : ''})`);
    const tel = await admin('query', 'telemetry:list', { sid: a.cloud.sid }), visit = tel.find(r => r.kind === 'visit');
    check(!!visit && typeof visit.data.machine?.webgpu === 'string' && /roomclear/.test(visit.data.query) && !/player=/.test(visit.data.query), `the visit's record is there: the machine as the page reads it (${String(visit?.data.machine?.webgpu).slice(0, 60)}), the page's query`);
    const prefs = await admin('query', 'prefs:get', { pid: a.cloud.pid }), map = prefs ? JSON.parse(prefs.data) : {};
    check(!!prefs && /0\.31/.test(map['twopointsix.options'] ?? '') && !Object.keys(map).some(k => /^twopointsix\.(cloud|save|quality)/.test(k)), `the browser's settings are there (${Object.keys(map).length} keys, rev ${prefs?.rev}), and none of the keys that stay`);

    // ---- 2. a fresh profile opens the link
    const B = await profile();
    const link = up.link.replace(/\?player=/, `?paused&quality=${QUALITY}&mute&cloud=${encodeURIComponent(CLOUD)}&player=`);
    await B.go(link); await B.page.waitForURL(u => !/player=/.test(String(u)), { timeout: NAV }).catch(() => {});
    const b = await B.ready(), took = await B.page.evaluate(() => ({ options: localStorage.getItem('twopointsix.options'), pid: localStorage.getItem('twopointsix.cloud.pid'), master: window.__audio?.volumes?.master ?? null }));
    check(/player=/.test(up.link) && !b.fatal && !/player=/.test(b.search) && took.pid === a.cloud.pid && b.cloud?.pid === a.cloud.pid && /0\.31/.test(took.options ?? '') && B.asked.length === 1 && /settings from this link/.test(B.asked[0]) && A.asked.length === 0, `a fresh browser that opens the first one's link is asked once, and on yes is that browser to the backend, has its settings before the game read a key, and its address has dropped the word (${b.search.slice(0, 60)})`);

    // ---- 3. no word on the address: no backend, and the stand-in hears nothing
    // (the second profile's page is closed first — it sends on its own timers — and the count starts once the first profile's old page, which says its
    // goodbye as it goes, is gone and the new one is up)
    await B.ctx.close().catch(() => {});
    await A.go(`http://localhost:${PORT}/?paused&quality=${QUALITY}&mute&roomclear&squad4`);
    const c = await A.ready(); await sleep(1500);
    const before = await heard();
    await A.play(FR); await sleep(4000);
    const after = await heard();
    check(!c.fatal && c.cloud === null && after === before, `the same browser with no word on the address has no backend (__cloud ${c.cloud === null ? 'null' : 'made'}) and the stand-in heard ${after - before} calls while it played`);

    // ---- 4. the address built in, a browser that has made no choice: nothing before the notice, the notice on the title, then it sends
    await A.ctx.close().catch(() => {});
    const C = await profile(), BUILT = 'https://built-in.example'; let built = 0;
    await C.ctx.addInitScript(() => { window.__TPS_CLOUD__ = 'https://built-in.example'; });
    await C.ctx.route(BUILT + '/**', async route => {
      const req = route.request(), cors = { 'Access-Control-Allow-Origin': '*', 'Access-Control-Allow-Headers': '*', 'Access-Control-Allow-Methods': 'POST, OPTIONS' };
      if (req.method() === 'OPTIONS') return route.fulfill({ status: 204, headers: cors });
      built++;
      try { const r = await fetch(CLOUD + new URL(req.url()).pathname, { method: 'POST', headers: { 'Content-Type': 'application/json' }, body: req.postData() ?? '{}' }); await route.fulfill({ status: r.status, headers: { ...cors, 'Content-Type': 'application/json' }, body: await r.text() }); }
      catch { await route.abort().catch(() => {}); }
    });
    const state = () => C.page.evaluate(() => ({ url: window.__cloud?.url ?? null, share: window.__cloud?.share ?? null, waits: window.__cloud?.waits ?? null, told: localStorage.getItem('twopointsix.cloud.told'), note: document.querySelector('#menu .cloudnote')?.textContent ?? null, title: !!document.getElementById('menu')?.classList.contains('title') }));
    await C.go(`http://localhost:${PORT}/?paused&quality=${QUALITY}&mute&solo`);
    const d = await C.ready(); await C.play(FR); await sleep(5000);
    const d1 = await state(), heard1 = built;
    check(!d.fatal && d1.url === BUILT && d1.share === true && d1.waits === true && d1.told === null && d1.note === null && heard1 === 0, `a hosted build's page in a browser that has made no choice, on a page with no title: it would send (${d1.share}), waits for the notice (${d1.waits}) and made ${heard1} requests while it played${d.fatal ? ' · FATAL ' + d.fatal : ''}`);
    // (leaving that page: its own goodbye — the tab hidden, the page going — sends nothing either; the next page cannot have shown the notice by its load)
    await C.go(`http://localhost:${PORT}/?paused&quality=${QUALITY}&mute`); const left = built;
    const e = await C.ready(), e0 = await state(); await sleep(6000);
    const e1 = await state(); await C.page.evaluate(() => window.__cloud.sync()).catch(() => {});
    const tel4 = await admin('query', 'telemetry:list', { sid: e.cloud?.sid ?? 'none' }), visit4 = tel4.find(r => r.kind === 'visit');
    check(!e.fatal && e1.title && /sends your replays, stats and settings/.test(e1.note ?? '') && /Options/.test(e1.note ?? '') && left === 0 && e1.told === '1' && e1.waits === false && built > 0 && !!visit4 && typeof visit4.data.gpu?.timestamps === 'boolean' && visit4.data.bootMs > 0,
      `the page left with ${left} requests made; the title page of that build has the notice's line ("${String(e1.note).slice(0, 48)}…"), after it has been up a while (told at the page's ready: ${e0.told}) the browser is marked told, and it sends from there (${built} requests; the visit's record has the GPU as the page got it: timestamps ${visit4?.data.gpu?.timestamps}, ${visit4?.data.gpu?.features?.length} features, up in ${visit4?.data.bootMs} ms)${e.fatal ? ' · FATAL ' + e.fatal : ''}`);
    check(errors.length === 0, `no page errors${errors.length ? ': ' + errors.slice(0, 3).join(' | ') : ''}`);
  } catch (e) { check(false, `the walk threw: ${String(e && e.stack || e).slice(0, 500)}`); }
  await browser.close().catch(() => {}); standin.kill();
  console.log(failed ? '\nFAILED' : '\nall pass');
  process.exit(failed ? 1 : 0);
})();
