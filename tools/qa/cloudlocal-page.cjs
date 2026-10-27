// A LOCAL BUILD WITH A LOCAL BACKEND, AND A NOTE TYPED ON A REPLAY (src/cloud/cloud.ts cloudFind · cloudNotes; tools/serve.ts /qa/cloud; tools/cloud/local.ts),
// on the real page in headless Chromium, software WebGPU adapter. This check starts its own dev server with TPS_CLOUD_URL naming a STAND-IN backend on this
// machine (cloud/test/standin.ts: `npm ci` in cloud/ first) — the way `bun run dev` hands the page the deployment it started — and walks:
//   1. the dev server tells a page its backend (GET /qa/cloud), and a game page with NO word on its address has one: the developer's own, so it sends with
//      no notice to wait for; some seconds of play are on the backend as a kept tape
//   2. the replay viewer opens that kept visit; a note typed there is on the backend at once (no game page open), under the visit's key
//   3. the note edited replaces the line; a second note joins it; the first one emptied leaves only the second
//   4. read back (tools/cloud/pull.ts, then tools/qa/replay-dump.ts) the note is in the visit's moments, where whoever reads the replay finds it
//   PORT=5331 [CLOUD_PORT=3231] [NAV_MS=480000] node tools/qa/cloudlocal-page.cjs        (one boot of the game: minutes on this adapter)
// NOT SEEN HERE: Convex's own local backend (`convex dev`; this machine cannot fetch it), a real GPU, the hosted build. Exit 0 = every check passed.
//   CLOUDLOCAL_BREAK=1: the dev server is started with no backend named — must FAIL.
const { chromium } = require('/opt/node-tools/node_modules/playwright');
const path = require('node:path'), cp = require('node:child_process'), os = require('node:os'), fs = require('node:fs');
const PORT = process.env.PORT || 5331, CPORT = Number(process.env.CLOUD_PORT) || 3231, ROOT = path.resolve(__dirname, '../..'), BREAK = process.env.CLOUDLOCAL_BREAK === '1';
const FLAGS = ['--enable-unsafe-webgpu', '--use-gl=angle', '--use-angle=swiftshader', '--enable-features=Vulkan', '--use-vulkan=swiftshader', '--ignore-gpu-blocklist', '--disable-gpu-watchdog'];
const NOISE = /""?smoke[A-Za-z0-9_]*""?|\[pipeline\] smoke_|number of storage textures \(5\) in the Compute stage/;
const CLOUD = `http://127.0.0.1:${CPORT}`, KEY = 'page-check-key', NAV = Math.max(60000, Number(process.env.NAV_MS) || 480000), FR = Math.max(30, Number(process.env.FRAMES) || 240);
const T0 = Date.now(), log = (...a) => console.log(`[${((Date.now() - T0) / 1000).toFixed(0)}s]`, ...a);
let red = 0; const check = (ok, what) => { console.log(`${ok ? '- PASS' : '- FAIL'} — ${what}`); if (!ok) red++; };
const sleep = ms => new Promise(r => setTimeout(r, ms));
const admin = async (fn, args) => { const r = await fetch(`${CLOUD}/api/query`, { method: 'POST', headers: { 'Content-Type': 'application/json', Authorization: `Convex ${KEY}` }, body: JSON.stringify({ path: fn, format: 'convex_encoded_json', args: [args] }) }); const j = await r.json(); if (j.status !== 'success') throw new Error(`${fn}: ${JSON.stringify(j).slice(0, 200)}`); return j.value; };
const until = async (f, ms = 20000) => { const t = Date.now(); for (;;) { const v = await f().catch(() => null); if (v || Date.now() - t > ms) return v; await sleep(250); } };
(async () => {
  const kids = [];
  kids.push(cp.spawn('bun', ['run', 'test/standin.ts', String(CPORT)], { cwd: path.join(ROOT, 'cloud'), env: { ...process.env, STANDIN_KEY: KEY }, stdio: 'ignore' }));
  const env = { ...process.env, PORT: String(PORT) }; delete env.TPS_LOCAL_CLOUD; if (BREAK) delete env.TPS_CLOUD_URL; else env.TPS_CLOUD_URL = CLOUD;
  kids.push(cp.spawn('bun', ['run', 'tools/serve.ts'], { cwd: ROOT, env, stdio: 'ignore' }));
  const done = code => { for (const k of kids) try { k.kill(); } catch { /* */ } process.exit(code); };
  await until(async () => (await fetch(`${CLOUD}/__stats`)).ok); await until(async () => (await fetch(`http://localhost:${PORT}/qa/commit`)).ok, 60000);
  const browser = await chromium.launch({ headless: true, channel: 'chromium', args: FLAGS });
  const ctx = await browser.newContext({ viewport: { width: 1100, height: 700 }, deviceScaleFactor: 1 }), page = await ctx.newPage(), errors = [];
  page.on('console', m => { if (m.type() === 'error' && !NOISE.test(m.text()) && !/Failed to load resource/.test(m.text())) errors.push(m.text().slice(0, 300)); });
  page.on('pageerror', x => errors.push('pageerror: ' + String(x && x.stack || x).slice(0, 400)));
  const go = async (to) => {   // (leaving a page: it stops drawing first — tools/qa/replaykeep-page.cjs)
    await page.evaluate(async () => { const e = window.__engine; if (!e) return; e.render = () => {}; try { if (window.__smoke) window.__smoke.params.enabled = false; await Promise.race([e.device.queue.onSubmittedWorkDone(), new Promise(r => setTimeout(r, 20000))]); } catch { /* */ } }).catch(() => {});
    await page.goto(`http://localhost:${PORT}/${to}`, { timeout: NAV });
  };
  try {
    // ---- 1. the server names the backend; a page with no word on its address has it, and sends with no notice
    const said = (await (await fetch(`http://localhost:${PORT}/qa/cloud`)).text()).trim();
    log('booting the game…'); await go('?paused&quality=verylow&mute&roomclear&squad4');
    const a = await page.evaluate(async () => { await window.__ready; const c = window.__cloud; return { fatal: document.getElementById('fatal')?.textContent || '', cloud: c ? { url: c.url, local: c.local, share: c.share, sid: c.sid, pid: c.pid } : null, told: localStorage.getItem('twopointsix.cloud.told'), search: location.search }; });
    check(said === CLOUD && !a.fatal && !!a.cloud && a.cloud.url === CLOUD && a.cloud.local === true && a.cloud.share === true && !/cloud=/.test(a.search),
      `the dev server names its backend (${said || 'nothing'}) and a game page with no word on its address has it as its own (${a.cloud ? `${a.cloud.url}, local ${a.cloud.local}` : 'no backend'})${a.fatal ? ' · FATAL ' + a.fatal.slice(0, 120) : ''}`);
    const play = () => page.evaluate(async (n) => { window.__menu.close(); window.__smoke.params.enabled = false; const e = window.__engine, real = e.render; e.render = () => {}; try { await window.__runFrames(n, 1 / 60, ['KeyW']); } finally { e.render = real; } }, FR);
    await play(); await sleep(6500); await play(); await sleep(6500);
    const up = await page.evaluate(async () => { const K = await window.__replay.kept(); await window.__cloud.sync(); await window.__cloud.sync(); const me = K.rows.find(r => r.sid === window.__cloud.sid); return { pieces: me?.pieces ?? 0, seconds: me?.seconds ?? 0, told: localStorage.getItem('twopointsix.cloud.told') }; });
    const row = a.cloud ? (await admin('tapes:list', { pid: a.cloud.pid })).find(r => r.sid === a.cloud.sid) : null;
    check(up.pieces >= 2 && !!row && row.have === up.pieces && up.told !== '1', `with no notice shown (a local backend is the developer's own) the ${up.pieces} pieces kept of the visit (${up.seconds} s) are on the backend (${row?.have ?? 0} there)`);
    // ---- 2. the replay viewer: a note typed on the kept visit goes up at once
    const sid = a.cloud?.sid ?? 'none', keyOf = `visit:${sid}`;
    const line = async () => { const r = (await admin('notes:list', { pid: a.cloud.pid })).find(x => x.key === keyOf); return r ? JSON.parse(r.text) : null; };
    await go(`replay.html#kept=${sid}`);
    const open = await until(() => page.evaluate(() => { const f = window.__viewer?.file; return f ? { session: f.session, samples: f.samples.length } : null; }), 30000);
    const type = async (words) => { await page.keyboard.type(words, { delay: 5 }); await page.keyboard.press('Enter'); };
    await page.evaluate(() => { window.__viewer.seek(2); window.__viewer.noteAt([1.5, -2]); }); await sleep(300); await type('he stops short of the door');
    const n1 = await until(async () => { const L = await line(); return L && L.notes.length === 1 && L.notes[0].text === 'he stops short of the door' ? L : null; });
    check(!!open && open.session === sid && !!n1 && n1.k === 'notes' && n1.sid === sid && Array.isArray(n1.notes[0].pos), `the viewer opens the kept visit and a note typed there is on the backend with no game page open ("${n1?.notes?.[0]?.text ?? 'nothing arrived'}")`);
    // ---- 3. edited, joined by a second, the first emptied
    await page.locator('#notes textarea').first().click(); await page.keyboard.press('End'); await type(', twice');
    const n2 = await until(async () => { const L = await line(); return L && L.notes.length === 1 && /, twice$/.test(L.notes[0].text) ? L : null; });
    await page.evaluate(() => { window.__viewer.seek(4); window.__viewer.noteAt(null); }); await sleep(300); await type('the stack breaks up here');
    const n3 = await until(async () => { const L = await line(); return L && L.notes.length === 2 ? L : null; });
    await page.locator('#notes textarea').first().fill(''); await page.locator('#notes textarea').first().blur();
    const n4 = await until(async () => { const L = await line(); return L && L.notes.length === 1 && L.notes[0].text === 'the stack breaks up here' ? L : null; });
    check(!!n2 && !!n3 && !!n4, `an edit replaces the line ("${n2?.notes?.[0]?.text ?? '—'}"), a second note joins it (${n3?.notes?.length ?? 0} notes) and the first one emptied leaves the other (${n4?.notes?.length ?? '—'})`);
    // ---- 4. read back: the note is in the visit's moments
    const out = fs.mkdtempSync(path.join(os.tmpdir(), 'cloud-local-')), pull = cp.spawnSync('bun', ['run', 'tools/cloud/pull.ts', '--url', CLOUD, '--key', KEY, '--out', out, '--quiet'], { cwd: ROOT, encoding: 'utf8' });
    const dump = fs.readdirSync(out).find(f => /\.ndjson\.gz$/.test(f)), outDir = path.join(out, 'unpacked');
    const read = dump ? cp.spawnSync('bun', ['run', 'tools/qa/replay-dump.ts', path.join(out, dump), '--out', outDir, '--quiet'], { cwd: ROOT, encoding: 'utf8' }) : null;
    const moments = read && fs.existsSync(outDir) ? fs.readdirSync(outDir).filter(f => /moments\.md$/.test(f)).map(f => fs.readFileSync(path.join(outDir, f), 'utf8')).join('\n') : '';
    check(pull.status === 0 && !!read && read.status === 0 && /NOTE[^\n]*the stack breaks up here/.test(moments) && !/he stops short/.test(moments), `pulled and unpacked, the note stands in the visit's moments and the emptied one does not (${dump ?? String(pull.stderr).slice(0, 160)})`);
    check(errors.length === 0, errors.length ? `page errors: ${errors.slice(0, 3).join(' | ')}` : 'no page errors');
  } catch (e) { check(false, `the check itself threw: ${String(e && e.stack || e).slice(0, 400)}`); }
  await browser.close().catch(() => {});
  console.log(red ? `\n${red} FAILED` : '\nall pass'); done(red ? 1 : 0);
})();
