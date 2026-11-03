// THE SIMULATION IN RUST ON THE REAL PAGE (src/sim/sim.ts): headless Chromium on the software WebGPU adapter (tools/qa/page-look.cjs's recipe).
//   PORT=5199 node tools/qa/rust-page.cjs      (a dev server on that port: PORT=5199 bun run tools/serve.ts; JOBS=a,b = only those jobs)
//   loads    a mission's page: the module compiled and started in the page's own thread, in one step, and said so on the console; no row is on. The fixed
//            stream of tools/qa/rust-fold.js through the page's maths (the TypeScript functions, in Chrome's V8) and through the module: every answer the same
//            bits, and the module's word is the one bun folds the same stream to (asked of `bun run tools/qa/rust-check.ts --hash` here)
//   rows     the same address with ?rustmaths: Math's functions and the lengths are the module's, and the game after 240 frames is, by digest and by the
//            chained raw bits of every frame, the game `loads` played on the TypeScript maths
//   systems  the same address with every other row raised (?rustlines&rustcircles&rustfog …: whatever src/sim/sim.ts RUST lists besides the maths): the box
//            world's questions are asked of the module's copy inside every tick (src/scene/boxMirror.ts), and the game after 240 frames is, by digest and
//            by the chained raw bits of every frame, the game `loads` played in TypeScript
//   nojit    the same address in a page with no WebAssembly (a browser whose policy turns its JIT off has none): the page boots, says the module is not
//            running, leaves the row off though the address asks for it, and plays the same game by digest
// exit 0 = every job, with no page error · 1 = otherwise. RUST_BREAK=1: `loads` hands the module's set the ENGINE's cosine, `rows` plays on it, and `systems`
// is told every line is clear: all three must FAIL.
// THE FRAMES ARE NOT DRAWN (the software adapter takes minutes over a frame of a level): this is the page's bundle, its boot and its game logic. Nothing
// here has been seen on a GPU.
const { chromium } = require('/opt/node-tools/node_modules/playwright');
const path = require('node:path'), cp = require('node:child_process');
const PORT = process.env.PORT || 5199, QUALITY = process.env.QUALITY || 'verylow', BREAK = process.env.RUST_BREAK === '1', ROOT = path.resolve(__dirname, '../..');
const ONLY = (process.env.JOBS || '').split(',').map(s => s.trim()).filter(Boolean);
const FLAGS = ['--enable-unsafe-webgpu', '--use-gl=angle', '--use-angle=swiftshader', '--enable-features=Vulkan', '--use-vulkan=swiftshader', '--ignore-gpu-blocklist', '--disable-gpu-watchdog'];
const T0 = Date.now(), log = (...a) => console.log(`[${((Date.now() - T0) / 1000).toFixed(0)}s]`, ...a);
const BASE = `http://localhost:${PORT}/`, ADDR = `levelFile=depot.json&roomclear&squad4&paused&mute&quality=${QUALITY}&seed=7`, FRAMES = 240;
const want = (name) => !ONLY.length || ONLY.includes(name);

// in the page: the game after n frames, the renderer's draw put aside; its digest and the chained raw bits of every frame
const PLAY = (n) => { const e = window.__engine, real = e.render; e.render = () => { e.world.upload(); }; let h; try { for (let i = 0; i < n; i++) { window.__runFrames(1, 1 / 60); h = window.__digest.bits(h); } } finally { e.render = real; } return { digest: window.__digest.game(), bits: (h >>> 0).toString(36) }; };
// in the page: what the module says of itself
const SEEN = () => ({ why: window.__sim.whyNot(), say: window.__sim.say(), maths: window.__sim.RUST.maths, on: Object.keys(window.__sim.RUST).filter(k => window.__sim.RUST[k]), copy: window.__game.col.mirror.slot, rows: Object.keys(window.__sim.RUST), wasm: typeof WebAssembly,
  inMath: !window.__sim.whyNot() && Math.sin === window.__sim.sim().maths_sin && Math.atan2 === window.__sim.sim().maths_atan2, exact: window.__knobs.registry().EXACT.on });
// in the page: the fixed stream through the page's maths and the module's
const FOLD = async (brk) => {
  const F = await import('/qa/rust-fold.js'), S = window.__sim;
  const own = {}; for (const n of ['sin', 'cos', 'tan', 'asin', 'acos', 'atan', 'atan2', 'exp', 'log', 'log2', 'log10', 'pow', 'tanh', 'hypot']) own[n] = Math[n];
  const rust = { ...S.maths() };   // (the set the row installs: src/sim/sim.ts mathsOfRust)
  if (brk) { const x = new Float64Array(1), u = new Uint32Array(x.buffer), cos = rust.cos; rust.cos = (a) => { x[0] = cos(a); u[0] ^= 1; return x[0]; }; }   // (the red path: the last bit of every cosine)
  const t0 = performance.now(), a = F.fold(own, F.FOLD_N, rust), b = F.fold(rust, F.FOLD_N);
  return { asked: a.asked, differ: a.differ, first: a.first, own: a.word, rust: b.word, ms: Math.round(performance.now() - t0) };
};

(async () => {
  let red = 0; const fail = (name, why) => { red++; console.log(`- FAIL — ${name}: ${why}`); }, pass = (name, what) => console.log(`- ok — ${name}: ${what}`);
  const open = async (init) => { const browser = await chromium.launch({ headless: true, channel: 'chromium', args: FLAGS }); const page = await (await browser.newContext({ viewport: { width: 1024, height: 576 }, deviceScaleFactor: 1 })).newPage();
    const errs = [], said = []; page.on('pageerror', e => errs.push(String(e).slice(0, 300)));
    page.on('console', m => { if (/^rust simulation:/.test(m.text())) said.push(m.text()); if ((m.type() === 'error' || m.type() === 'warning') && !/Failed to load resource/.test(m.text())) errs.push(m.text().slice(0, 300)); });
    if (init) await page.addInitScript(init);
    return { browser, page, errs, said }; };
  const ready = async (page, secs = 900) => { for (const end = Date.now() + secs * 1000; Date.now() < end;) {
      try { const r = await page.evaluate(() => Promise.race([window.__ready.then(() => window.__game && window.__sim ? 'ok' : 'wait', e => 'dead: ' + e), new Promise(res => setTimeout(() => res('wait'), 2000))])); if (r !== 'wait') return r; } catch { /* between documents */ }
      await page.waitForTimeout(400); } return 'no boot in time'; };
  let played = null, rowNames = null;   // (the game every other job is held to, and the rows the page's table lists)

  if (want('loads')) { const name = 'loads', { browser, page, errs, said } = await open(); try {
    await page.goto(`${BASE}?${ADDR}`); const r = await ready(page); if (r !== 'ok') throw new Error(r);
    const s = await page.evaluate(SEEN); log(` ${name}: ${s.say}`);
    if (s.why || s.on.length || s.inMath || !s.exact || !/^rust simulation: \d+ bytes, started in/.test(s.say)) throw new Error(`the module is not started with every row off: ${JSON.stringify(s)}`);
    if (!said.some(l => /^rust simulation: \d+ bytes, started in/.test(l))) throw new Error(`the boot did not say the module started: ${JSON.stringify(said)}`);
    rowNames = s.rows; played = await page.evaluate(PLAY, FRAMES);   // (first: the game `rows` and `nojit` are held to stands whatever the stream below says)
    const f = await page.evaluate(FOLD, BREAK); log(` ${name}: the stream in the page — ${JSON.stringify({ ...f, first: f.first.slice(0, 2) })}`);
    const bun = cp.spawnSync('bun', ['run', 'tools/qa/rust-check.ts', '--hash'], { cwd: ROOT, encoding: 'utf8' }), line = (bun.stdout || '').split('\n').find(l => l.startsWith('RUST-HASH '));
    const there = line ? JSON.parse(line.slice(10)) : null; if (!there) throw new Error(`bun did not fold the stream: ${(bun.stderr || '').slice(-200)}`);
    if (f.differ !== 0 || f.own !== f.rust) throw new Error(`${f.differ} of ${f.asked} answers differ between the page's TypeScript maths and the module (${f.own} · ${f.rust}): ${f.first.slice(0, 3).join(' | ')}`);
    if (f.rust !== there.word) throw new Error(`the module folds the stream to ${f.rust} in the page and ${there.word} under ${there.engine}`);
    if (errs.length) throw new Error(`the page complained: ${errs.slice(0, 3).join(' | ')}`);
    pass(name, `${s.say} · ${(f.asked / 1e6).toFixed(1)} million answers in the page, 0 differ, word ${f.rust} = ${there.engine}'s (${f.ms} ms) · ${FRAMES} frames on the TypeScript maths: digest ${played.digest}, bits ${played.bits}`);
  } catch (e) { fail(name, String(e.message || e)); } await browser.close(); }

  if (want('rows')) { const name = 'rows', { browser, page, errs } = await open(); try {
    await page.goto(`${BASE}?${ADDR}&rustmaths`); const r = await ready(page); if (r !== 'ok') throw new Error(r);
    const s = await page.evaluate(SEEN); log(` ${name}: ${s.say}`);
    if (s.why || !s.maths || !s.inMath || !/in Rust: maths/.test(s.say)) throw new Error(`?rustmaths did not put the module's maths in Math's place: ${JSON.stringify(s)}`);
    if (BREAK) await page.evaluate(() => { const x = new Float64Array(1), u = new Uint32Array(x.buffer), cos = Math.cos; Math.cos = (a) => { x[0] = cos(a); u[0] ^= 1; return x[0]; }; });
    const d = await page.evaluate(PLAY, FRAMES);
    if (errs.length) throw new Error(`the page complained: ${errs.slice(0, 3).join(' | ')}`);
    if (!played) throw new Error('`loads` did not play: nothing to hold this game to (run both jobs)');
    if (d.digest !== played.digest || d.bits !== played.bits) throw new Error(`the game on the module's maths is not the game on the TypeScript maths: digest ${d.digest} · ${played.digest}, bits ${d.bits} · ${played.bits}`);
    pass(name, `${s.say} · ${FRAMES} frames: digest ${d.digest}, bits ${d.bits} — the game on the TypeScript maths, to the bit`);
  } catch (e) { fail(name, String(e.message || e)); } await browser.close(); }

  if (want('systems')) { const name = 'systems', { browser, page, errs } = await open(); try {
    if (!rowNames) throw new Error('`loads` did not say which rows there are (run both jobs)');
    const up = rowNames.filter(r => r !== 'maths');
    await page.goto(`${BASE}?${ADDR}${up.map(r => '&rust' + r).join('')}`); const r = await ready(page); if (r !== 'ok') throw new Error(r);
    const s = await page.evaluate(SEEN); log(` ${name}: ${s.say}`);
    if (s.why || s.maths || s.on.join() !== up.join() || !/in Rust: lines/.test(s.say)) throw new Error(`the address did not raise the rows ${up.join(', ')}: ${JSON.stringify(s)}`);
    // (the red path: the module's answer to every line through the statics replaced by "clear" — a wrong walk that still plays)
    if (BREAK) await page.evaluate(() => { const M = window.__game.col.mirror, P = Object.getPrototypeOf(M), open = P.open; P.open = function (...a) { open.apply(this, a); if (!this.broke) { this.broke = true; this.x = Object.assign({}, this.x, { lines_blocked: () => 0 }); } }; });
    const d = await page.evaluate(PLAY, FRAMES), t = await page.evaluate(SEEN);
    if (errs.length) throw new Error(`the page complained: ${errs.slice(0, 3).join(' | ')}`);
    if (!(t.copy >= 0)) throw new Error(`no tick opened the module's copy of the box world (its world: ${t.copy})`);
    if (!played) throw new Error('`loads` did not play: nothing to hold this game to (run both jobs)');
    if (d.digest !== played.digest || d.bits !== played.bits) throw new Error(`the game with ${up.join(', ')} asked of the module is not the game in TypeScript: digest ${d.digest} · ${played.digest}, bits ${d.bits} · ${played.bits}`);
    pass(name, `${s.say} · ${FRAMES} frames: digest ${d.digest}, bits ${d.bits} — the game in TypeScript, to the bit`);
  } catch (e) { fail(name, String(e.message || e)); } await browser.close(); }

  if (want('nojit')) { const name = 'nojit', { browser, page, errs, said } = await open(() => { delete globalThis.WebAssembly; }); try {
    await page.goto(`${BASE}?${ADDR}${(rowNames || ['maths', 'lines']).map(r => '&rust' + r).join('')}`); const r = await ready(page); if (r !== 'ok') throw new Error(r);
    const s = await page.evaluate(SEEN); log(` ${name}: ${s.say}`);
    if (s.wasm !== 'undefined' || !s.why || s.on.length || !/NOT RUNNING/.test(s.say) || !said.some(l => /NOT RUNNING/.test(l))) throw new Error(`a page with no WebAssembly: ${JSON.stringify(s)} said ${JSON.stringify(said)}`);
    const d = await page.evaluate(PLAY, FRAMES);
    if (errs.length) throw new Error(`the page complained: ${errs.slice(0, 3).join(' | ')}`);
    if (played && (d.digest !== played.digest || d.bits !== played.bits)) throw new Error(`a page with no WebAssembly plays another game: digest ${d.digest} · ${played.digest}, bits ${d.bits} · ${played.bits}`);
    pass(name, `${s.say} · the rows stayed off · ${FRAMES} frames: digest ${d.digest}${played ? ', the same game' : ''}`);
  } catch (e) { fail(name, String(e.message || e)); } await browser.close(); }

  console.log(red ? `\n${red} FAILED` : '\nall pass'); process.exit(red ? 1 : 0);
})();
