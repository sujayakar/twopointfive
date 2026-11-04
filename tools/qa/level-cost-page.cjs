// WHAT THE LOOK COSTS ON ONE LEVEL, IN ONE BOOT OF THE REAL PAGE (the levels goal, 11-03: "we'll want to dress up all the levels and check their budget").
// The look page of a level's mission in headless Chromium on the SOFTWARE WebGPU adapter, once, and from that one boot:
//   · what the page compiled: shader modules and their text, pipelines before the page came ready, and whether switching the look's layers off and on again
//     made it compile anything more (the budget's "shaders compile once": it must be nothing)
//   · a frame's time with the look's layers OFF against ON, at the same camera, quality, size and scale, the two taken in turn (off, on, off, on) so that a
//     drift of the machine falls on both: the wall time of a drawn frame, and where the adapter stamps them, each pass's own time
//   · a still, read back from the frame before anything is switched (the play camera at the mission's start, at the hour its row opens at).
// "Off" is every row the look panel lists as a layer with a cost of its own (ui/lookpanel.ts hands them to the sweep), each at its own off value, while the
// shaders, the level's shapes and the camera stay as they are. It is NOT the plain build: that page has other shader text (see --plain-page).
//
// EVERY NUMBER HERE IS A RELATIVE READ. A software adapter runs each pass on the CPU, one after another: a frame that takes seconds here takes milliseconds on a
// GPU, and what is dear here (a wide filter, many texture taps) is not what is dear there. What carries over is the DIRECTION and roughly the SHARE: on over off
// on this level against on over off on the pass, which is the one level the look was timed on a GPU. Nothing drawn here has been seen on a GPU.
//
//   node tools/qa/level-cost-page.cjs pass                     one level; its files land in dist/level-cost/pass/ (cost.json, look.png)
//   node tools/qa/level-cost-page.cjs pass block compound      several, one page after another (never two at once: the adapter loses its device)
//   --out <dir>        where the folders go (default dist/level-cost)
//   --port <n>         the dev server's port (default 5199). None listening there: one is started from this checkout and stopped again by its pid
//   --quality <q>      the preset the page stands at (default medium: at High a frame is half a minute on this adapter, and the read is relative either way)
//   --size <WxH>       the window (default 480x270)
//   --frames <n>       timed frames a turn (default 3) · --rounds <n>  turns of each state (default 2)
//   --plain-page       ALSO boot the same mission WITHOUT the look page (the build's own picture as this branch draws it) and say what that page compiled and how
//                      long its frame takes: the look's price in shader text and compile time, which every player pays whichever way the layers stand.
//                      Its still, plain-page.png, beside look.png is the before and after of the look itself
//   env PLAYWRIGHT_BROWSERS_PATH: the browser. The page loses its device at the end of its boot on Chromium 153; the checks were written on 141 (playwright 1.56.1)
// Exit 0 = every page came ready, nothing more was compiled by the switch, no GPU error and no page error; 1 = one of those; 2 = a level the list does not have.
const { chromium } = require('/opt/node-tools/node_modules/playwright');
const fs = require('fs'), path = require('path'), http = require('http');
const { execFileSync, spawn } = require('child_process');

const ROOT = path.resolve(__dirname, '../..');
const argv = process.argv.slice(2); const opt = (k, d) => { const i = argv.indexOf('--' + k); if (i < 0) return d; const v = argv[i + 1]; argv.splice(i, 2); return v; };
const has = k => { const i = argv.indexOf('--' + k); if (i < 0) return false; argv.splice(i, 1); return true; };
const PORT = Number(opt('port', process.env.PORT || 5199)), OUT = path.resolve(opt('out', path.join(ROOT, 'dist/level-cost'))), QUALITY = opt('quality', 'medium');
const [W, H] = String(opt('size', '480x270')).split('x').map(Number), FRAMES = Number(opt('frames', 3)), ROUNDS = Number(opt('rounds', 2)), PLAIN_PAGE = has('plain-page');
const FLAGS = ['--enable-unsafe-webgpu', '--use-gl=angle', '--use-angle=swiftshader', '--enable-features=Vulkan', '--use-vulkan=swiftshader', '--ignore-gpu-blocklist', '--disable-gpu-watchdog'];
const T0 = Date.now(), log = (...a) => console.log(`[${((Date.now() - T0) / 1000).toFixed(0)}s]`, ...a);
const RELATIVE = 'software adapter: a relative read, never seen on a GPU';

// ---- in the page, before its script: every shader module and pipeline it makes (tools/qa/shader-count-page.cjs counts the same calls)
const COUNT = () => {
  const P = typeof GPUDevice === 'function' ? GPUDevice.prototype : null; if (!P) return;
  const hash = s => { let a = 0x811c9dc5, b = 0x1b873593; for (let i = 0; i < s.length; i++) { const c = s.charCodeAt(i); a = Math.imul(a ^ c, 16777619); b = Math.imul(b ^ c, 0x85ebca6b) + (b >>> 13); } return (a >>> 0).toString(36) + (b >>> 0).toString(36); };
  const C = window.__levelCost = { mods: [], pipes: [] };
  const sm = P.createShaderModule; P.createShaderModule = function (d) { C.mods.push({ label: d.label ?? '', h: hash(d.code), chars: d.code.length, at: performance.now() }); return sm.call(this, d); };
  for (const [name, sync] of [['createComputePipeline', true], ['createRenderPipeline', true], ['createComputePipelineAsync', false], ['createRenderPipelineAsync', false]]) {
    const f = P[name]; P[name] = function (d) { C.pipes.push({ label: d.label ?? '', sync, at: performance.now() }); return f.call(this, d); }; }
};
// ---- in the page: the look's layers off (true) or as they were (false). Off = every row the look panel lists as a layer with a cost of its own (it hands them
// to the sweep as `look.<row>`, each with its own off value), all at once: NOT its "plain boxes" (`look.all`), which leaves the lighting's, the smoke's and the
// eye's layers as they stand. Says how many rows; null = the page has no such rows
const LOOK_OFF = (off) => {
  const w = window;
  if (!off) { for (const f of (w.__levelCostBack ?? []).reverse()) f(); w.__levelCostBack = null; return 'as they were'; }
  if (w.__levelCostBack) return 'already off';
  const rows = (w.__perfsweep?.extra ?? []).filter(x => x.id.startsWith('look.') && x.id !== 'look.all'), back = [], names = [];
  for (const row of rows) { const r = row.flip(); if (r && typeof r === 'object') { back.push(r.apply()); names.push(row.id.slice(5)); } }
  if (!back.length) return null;
  w.__levelCostBack = back; return `${back.length} of the look's ${rows.length} layers put off: ${names.join(' ')}`;
};
// ---- in the page: n frames drawn one at a time, each waited for; the wall time of each and the passes' own times of the last
const TIME = async (n) => {
  const e = window.__engine, dev = e.device, ms = [];
  const yieldMacro = () => new Promise(res => { const ch = new MessageChannel(); ch.port1.onmessage = () => res(); ch.port2.postMessage(0); });
  for (let i = 0; i < n; i++) { const t = performance.now(); window.__runFrames(1, 1 / 600); await dev.queue.onSubmittedWorkDone(); ms.push(performance.now() - t); await yieldMacro(); }
  const T = e.timer, passes = T?.enabled ? Object.fromEntries([...T.lastFrame].map(([k, v]) => [k, +v.toFixed(2)])) : null;
  // (the union of every pass but the present: frames are serialized here and this adapter has no display to wait for, so the render passes stay in. The page's
  //  own GPU span on a real GPU leaves them out, ui/pricerun.ts DRAWN: the two are not the same quantity, and this one is in cost.json only as `passMs`)
  return { ms, passes, union: T?.enabled ? +T.unionMs(['present']).toFixed(2) : null };
};
// ---- in the page: the frame as it stands, read back as a PNG
const STILL = async () => {
  const e = window.__engine; e.render(window.__cam, window.__game.time, 1 / 60, false); const fr = await e.readbackFrame(); if (!fr) return { err: 'no frame' };
  if (fr.bgra) for (let i = 0; i < fr.data.length; i += 4) { const b = fr.data[i]; fr.data[i] = fr.data[i + 2]; fr.data[i + 2] = b; }
  let sum = 0; for (let i = 0; i < fr.data.length; i += 4) { sum += fr.data[i] + fr.data[i + 1] + fr.data[i + 2]; fr.data[i + 3] = 255; }
  const c2 = document.createElement('canvas'); c2.width = fr.w; c2.height = fr.h; c2.getContext('2d').putImageData(new ImageData(new Uint8ClampedArray(fr.data.buffer), fr.w, fr.h), 0, 0);
  return { w: fr.w, h: fr.h, mean: +(sum / (fr.w * fr.h * 3)).toFixed(2), url: c2.toDataURL('image/png') };
};
// ---- in the page: the page's own loop for `secs` of game time with the renderer stubbed out (the mission staged, nothing drawn)
const PLAY = (secs) => { const e = window.__engine, real = e.render; let threw = null; e.render = () => { };
  try { for (let i = 0; i < Math.round(secs * 60) && !threw; i++) { try { window.__runFrames(1, 1 / 60); } catch (x) { threw = `frame ${i}: ${x && x.stack || x}`; } } } finally { e.render = real; } return threw; };

/** ONE SCENE on every boot of a level: the die pinned (the page draws it from the clock otherwise, and a mission with no seed of its own is dealt anew each
 *  boot), and the look page as the mission plays on it, not as it opens for a visitor (`lookkit=0`: the fog of war and the kit as dealt, where a visitor's look
 *  page puts the fog down). So the look page and the page without the look are the same picture but for the look, and the pricing run times the same */
const SCENE = '&seed=1&lookkit=0';
const median = a => { const s = [...a].sort((x, y) => x - y), n = s.length; return n ? (n % 2 ? s[(n - 1) / 2] : (s[n / 2 - 1] + s[n / 2]) / 2) : NaN; };
const up = () => new Promise(res => { const r = http.get({ host: 'localhost', port: PORT, path: '/', timeout: 2000 }, x => { x.resume(); res(x.statusCode === 200); }); r.on('error', () => res(false)); r.on('timeout', () => { r.destroy(); res(false); }); });

/** one page: boot, count, and (on the look page) the off / on turns and the stills */
async function boot(browser, level, query, look, dir) {
  const R = { page: look ? 'look' : 'plain', url: `http://localhost:${PORT}/?${look ? 'look&' : ''}${query}${SCENE}&paused&mute&quality=${QUALITY}`, errors: [], gpuErrors: [] };
  const page = await browser.newPage({ viewport: { width: W, height: H }, deviceScaleFactor: 1 });
  await page.addInitScript(() => { window.__testVisible = true; }); await page.addInitScript(COUNT);
  page.on('pageerror', x => R.errors.push('pageerror: ' + String(x && x.stack || x).slice(0, 500)));
  page.on('console', m => { const x = m.text(); if (m.type() === 'error' && !/Failed to load resource/.test(x)) (/WebGPU|GPU device|pipeline|shader/i.test(x) ? R.gpuErrors : R.errors).push(x.slice(0, 400)); });
  page.on('response', r => { if (r.status() >= 400 && !/favicon/.test(r.url())) R.errors.push(`http ${r.status()}: ${r.url()}`); });
  try {
    log(level, R.page, 'goto', R.url); const t = Date.now();
    await page.goto(R.url, { waitUntil: 'commit' });
    await page.waitForFunction(() => !!window.__ready, null, { timeout: 1500000 });
    const ready = await page.evaluate(async () => { await window.__ready; const e = window.__engine, c = window.__levelCost; c.readyAt = performance.now();
      return { fatal: document.getElementById('fatal')?.textContent || '', boot: window.__boot?.line ?? '', pipelines: window.__boot?.pipelines ?? null, stamps: !!e.timer?.enabled, look: !!window.__look }; });
    R.readySecs = +((Date.now() - t) / 1000).toFixed(1); Object.assign(R, ready); if (ready.fatal) R.errors.push('fatal: ' + ready.fatal);
    log(level, R.page, `ready in ${R.readySecs} s`);
    // the second set of ray programs (cut boxes that swap in) compiles after ready where the page has one: waited for, so that it is not counted against the switch
    R.cutSet = await page.evaluate(async (secs) => { const P = window.__cut?.programs, r = P?.swap ? P.ready : null; if (!r) return 'none';
      const got = await Promise.race([Promise.resolve(r).then(ok => ok ? 'ready' : 'failed', () => 'failed'), new Promise(done => setTimeout(() => done('timed out'), secs * 1000))]); return got; }, 600).catch(e => 'threw: ' + e.message);
    // the start card away, the smoke solver off (a step of it is seconds on this adapter), the mission staged
    await page.evaluate(() => { window.__menu?.close(); if (window.__smoke?.params) window.__smoke.params.enabled = false; });
    const threw = await page.evaluate(PLAY, 2); if (threw) R.errors.push('play threw: ' + threw);
    await page.evaluate(TIME, 2);   // (the first frames after a boot fill the histories: not timed)
    Object.assign(R, await page.evaluate(() => { const e = window.__engine; return { internal: [e.width, e.height], out: [e.outWidth, e.outHeight], scale: +e.settings.renderScale.toFixed(3), quality: window.__quality?.label ?? null }; }));   // (the sizes as the first drawn frames left them: at ready the canvas has none yet)
    // THE STILL, before anything is switched: the picture is an accumulation over frames, and for a dozen frames after a switch it is still mostly the state before it
    // (a still taken in a turn with the layers off was the layers on, to a hundredth). So one still, of the page as it opens, on a history that has seen nothing else
    { const st = await page.evaluate(STILL), name = look ? 'look.png' : 'plain-page.png'; if (st.url) { fs.writeFileSync(path.join(dir, name), Buffer.from(st.url.split(',')[1], 'base64')); R.still = { file: name, size: [st.w, st.h], mean: st.mean }; } else R.errors.push(`still: ${st.err}`); }
    const made = () => page.evaluate(() => { const c = window.__levelCost; return { mods: c.mods.length, pipes: c.pipes.length }; });
    const before = await made();
    const turns = { on: [], off: [] }; let passes = { on: null, off: null }, union = { on: [], off: [] }, how = null;
    if (look) turns_: for (let r = 0; r < ROUNDS; r++) for (const off of [true, false]) {
      const did = await page.evaluate(LOOK_OFF, off); if (off) how = did; if (off && did === null) { R.errors.push('the page has no way to put the look\'s layers off (its panel\'s rows are not on the sweep)'); break turns_; }
      await page.evaluate(TIME, 1);   // (the frame the switch lands on)
      const got = await page.evaluate(TIME, FRAMES), k = off ? 'off' : 'on'; turns[k].push(...got.ms); passes[k] = got.passes; if (got.union !== null) union[k].push(got.union);
      log(level, `layers ${off ? 'off' : 'on '}: ${got.ms.map(v => (v / 1000).toFixed(2)).join(' ')} s a frame`);
    }
    else { const got = await page.evaluate(TIME, FRAMES * ROUNDS); turns.on.push(...got.ms); passes.on = got.passes; if (got.union !== null) union.on.push(got.union); }
    const after = await made(), c = await page.evaluate(() => window.__levelCost);
    const atBoot = x => x.at <= c.readyAt, texts = new Set(c.mods.map(m => m.h));
    R.compiled = { modules: c.mods.length, texts: texts.size, textKB: Math.round([...new Map(c.mods.map(m => [m.h, m.chars])).values()].reduce((s, n) => s + n, 0) / 1024), pipelines: c.pipes.length,
      pipelinesBeforeReady: c.pipes.filter(atBoot).length, pipelinesSync: c.pipes.filter(p => p.sync).length, afterReady: c.pipes.filter(p => !atBoot(p)).map(p => p.label),
      bySwitch: { modules: after.mods - before.mods, pipelines: after.pipes - before.pipes, labels: c.pipes.slice(before.pipes, after.pipes).map(p => p.label) } };
    R.pipelineLabels = c.pipes.map(p => p.label); R.textHashes = [...texts];
    R.offBy = how; R.frameMs = { on: turns.on.map(v => +v.toFixed(0)), off: turns.off.map(v => +v.toFixed(0)), onMedian: +median(turns.on).toFixed(0), offMedian: turns.off.length ? +median(turns.off).toFixed(0) : null };
    if (turns.off.length) R.frameMs.onOverOff = +(median(turns.on) / median(turns.off)).toFixed(3);
    if (passes.on) { R.passMs = { on: passes.on, off: passes.off, unionOn: union.on.length ? +median(union.on).toFixed(1) : null, unionOff: union.off.length ? +median(union.off).toFixed(1) : null };
      if (passes.off) R.passMs.dearest = Object.keys(passes.on).map(k => ({ pass: k, on: passes.on[k], off: passes.off[k] ?? 0, more: +(passes.on[k] - (passes.off[k] ?? 0)).toFixed(2) })).sort((a, b) => b.more - a.more).slice(0, 6); }
  } catch (e) { R.errors.push('did not finish: ' + String(e && e.message || e).slice(0, 400)); }
  await page.close().catch(() => { });
  return R;
}

(async () => {
  const rows = JSON.parse(execFileSync('bun', ['run', path.join(__dirname, 'level-audit.ts'), '--rows'], { cwd: ROOT, encoding: 'utf8' }).trim().split('\n').pop());
  const want = argv.filter(a => !a.startsWith('--')), unknown = want.filter(n => !rows.some(r => r.level === n));
  if (!want.length || unknown.length) { console.error(unknown.length ? `level-cost-page: not on the mission list: ${unknown.join(', ')}` : 'level-cost-page: name a level', `\nthe list has: ${rows.map(r => r.level).join(' ')}`); process.exit(2); }
  let server = null;
  if (!(await up())) { log(`no dev server on ${PORT}: starting one from this checkout`); server = spawn('bun', ['run', 'tools/serve.ts'], { cwd: ROOT, env: { ...process.env, PORT: String(PORT) }, stdio: 'ignore' });
    for (let i = 0; i < 60 && !(await up()); i++) await new Promise(r => setTimeout(r, 500)); if (!(await up())) { console.error('level-cost-page: the dev server did not come up'); server.kill(); process.exit(1); } }
  const browser = await chromium.launch({ headless: true, channel: 'chromium', args: FLAGS }); let red = 0;
  for (const level of want) {
    const row = rows.find(r => r.level === level), dir = path.join(OUT, level); fs.mkdirSync(dir, { recursive: true });
    const query = row.address.replace(/^\?/, '');
    const L = await boot(browser, level, query, true, dir), P = PLAIN_PAGE ? await boot(browser, level, query, false, dir) : null;
    const out = { level, mission: row.missions[0], hour: row.hour, address: `?look&${query}${SCENE}`, adapter: 'software (SwiftShader)', read: RELATIVE, quality: QUALITY, window: [W, H], frames: FRAMES, rounds: ROUNDS, at: new Date().toISOString(), look: L, plainPage: P };
    if (P && L.pipelineLabels && P.pipelineLabels) { const had = new Set(P.textHashes); const count = a => a.reduce((m, l) => (m.set(l, (m.get(l) ?? 0) + 1), m), new Map()), a = count(L.pipelineLabels), b = count(P.pipelineLabels);
      out.againstPlainPage = { textsOnlyOnLook: L.textHashes.filter(h => !had.has(h)).length, textsShared: L.textHashes.filter(h => had.has(h)).length, textKB: [P.compiled?.textKB, L.compiled?.textKB], pipelines: [P.compiled?.pipelines, L.compiled?.pipelines],
        pipelinesOnlyOnLook: [...a].filter(([l, n]) => n > (b.get(l) ?? 0)).map(([l, n]) => n - (b.get(l) ?? 0) > 1 ? `${l} ×${n - (b.get(l) ?? 0)}` : l), readySecs: [P.readySecs, L.readySecs], frameMs: [P.frameMs?.onMedian, L.frameMs?.onMedian] }; }
    for (const R of [L, P]) if (R) { delete R.pipelineLabels; delete R.textHashes; }
    fs.writeFileSync(path.join(dir, 'cost.json'), JSON.stringify(out, null, 1) + '\n');
    const bad = [...L.errors, ...L.gpuErrors, ...(P ? [...P.errors, ...P.gpuErrors] : [])], sw = L.compiled?.bySwitch;
    const ok = bad.length === 0 && !!L.frameMs?.onOverOff && sw && sw.modules === 0 && sw.pipelines === 0; if (!ok) red++;
    console.log(`\n${ok ? 'PASS' : 'FAIL'} — ${level} (${row.hour}, ${QUALITY}, ${L.internal?.join('×') ?? '?'} inside ${W}×${H}; ${RELATIVE})`);
    if (L.compiled) console.log(`  compiled: ${L.compiled.modules} shader modules (${L.compiled.texts} texts, ${L.compiled.textKB} KB), ${L.compiled.pipelines} pipelines, ${L.compiled.pipelinesBeforeReady} before the page came ready in ${L.readySecs} s; switching the layers off and on made ${sw.modules} modules and ${sw.pipelines} pipelines${sw.labels.length ? ` (${sw.labels.join(', ')})` : ''}`);
    if (L.frameMs?.onOverOff) console.log(`  a frame: layers off ${(L.frameMs.offMedian / 1000).toFixed(2)} s, on ${(L.frameMs.onMedian / 1000).toFixed(2)} s — on is ${L.frameMs.onOverOff} × off (${L.offBy}; medians of ${L.frameMs.on.length} frames each)`);
    if (L.passMs?.dearest) console.log(`  the passes that grew most with the layers on (ms on this adapter): ${L.passMs.dearest.filter(d => d.more > 0).map(d => `${d.pass} ${d.off} → ${d.on}`).join(', ') || 'none grew'}`);
    if (out.againstPlainPage) { const A = out.againstPlainPage; console.log(`  against the page without the look: shader text ${A.textKB[0]} → ${A.textKB[1]} KB, pipelines ${A.pipelines[0]} → ${A.pipelines[1]}${A.pipelinesOnlyOnLook.length ? ` (more on the look page: ${A.pipelinesOnlyOnLook.join(', ')})` : ''}, ready in ${A.readySecs[0]} → ${A.readySecs[1]} s, a frame ${A.frameMs[0]} → ${A.frameMs[1]} ms`); }
    for (const b of bad) console.log(`  ! ${b}`);
    console.log(`  written: ${dir}/cost.json${L.still ? ', look.png' : ''}${P?.still ? ', plain-page.png (the same mission without the look)' : ''}`);
  }
  await browser.close(); if (server) server.kill();
  process.exit(red ? 1 : 0);
})().catch(e => { console.error(e); process.exit(1); });
