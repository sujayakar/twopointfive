// THE SIMULATION IN RUST ON THE REAL PAGE (src/sim/sim.ts): headless Chromium on the software WebGPU adapter (tools/qa/page-look.cjs's recipe).
//   PORT=5199 node tools/qa/rust-page.cjs      (a dev server on that port: PORT=5199 bun run tools/serve.ts; JOBS=a,b = only those jobs)
// The module is the only way the game is played: its rows are not switches, and the page's check has no game in TypeScript to hold the page to. It holds:
//   loads    a mission's page as a player opens it: the module compiled and started in the page's own thread, in one step, and said so on the console;
//            every row of src/sim/sim.ts RUST_ON is up and no other. The fixed stream of tools/qa/rust-fold.js through the page's maths (the game's own
//            functions, in Chrome's V8) and through the module: every answer the same bits, and the module's word is the one bun folds the same stream to
//            (asked of `bun run tools/qa/rust-check.ts --hash` here). Then 240 frames: the game every other job is held to
//   plain    the same address again, with ?fell: the same game by digest and by the chained raw bits of every frame (two loads of the page are one
//            game), every tick opens the module's copy of the box world, the fog's pass asks men of it, and the page's count of questions its
//            TypeScript answered while a row was up reads none of a row that is one way in (sim.ts SEALED); how often the copy was proven outside a
//            tick is printed (ALONE_WHO=1: and who asked, the address then carries ?fell=who)
//   words    the same address with every retired word on it (?rust=0, ?rust<row>=0 for the first row, ?noexact): the rows are the same rows, the game
//            the same game, and the page says of each word that it does nothing
//   replay   `plain`'s frames as a replay file, opened as ?playtape=<file>: the page stands on the rows the game is played on and the tape plays to its end
//            and the tape plays to its end on its trace
//   nojit    the same address in a page with no WebAssembly (a browser whose policy turns its JIT off has none): the card says the maintainer's words,
//            no game is built, and the page's promise of a boot is refused
// exit 0 = every job, with no page error · 1 = otherwise. RUST_BREAK=1: `loads` hands the module's set a cosine one bit off, and `plain` is told every
// line is clear: both must FAIL.
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
// the card a page with no WebAssembly shows, word for word (src/sim/sim.ts NO_WASM_WORDS)
const WORDS = "This game needs WebAssembly, and it's turned off in this browser. Try another browser.";

// in the page: the game after n frames, the renderer's draw put aside; its digest and the chained raw bits of every frame
const PLAY = (n) => { const e = window.__engine, real = e.render; e.render = () => { e.world.upload(); }; let h; try { for (let i = 0; i < n; i++) { window.__runFrames(1, 1 / 60); h = window.__digest.bits(h); } } finally { e.render = real; } return { digest: window.__digest.game(), bits: (h >>> 0).toString(36) }; };
// in the page: what the module says of itself
const SEEN = () => ({ why: window.__sim.whyNot(), say: window.__sim.say(), def: [...window.__sim.RUST_ON], maths: window.__sim.RUST.maths, on: Object.keys(window.__sim.RUST).filter(k => window.__sim.RUST[k]), copy: window.__game.col.mirror.slot, rows: Object.keys(window.__sim.RUST), wasm: typeof WebAssembly,
  inMath: !window.__sim.whyNot() && Math.sin === window.__sim.sim().maths_sin && Math.atan2 === window.__sim.sim().maths_atan2, card: (() => { const c = document.getElementById('fatal'); return c && c.style.display === 'flex' ? c.textContent : ''; })() });
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
    const errs = [], said = [], warned = []; page.on('pageerror', e => errs.push(String(e).slice(0, 300)));
    page.on('console', m => { const t = m.text(); if (/^rust simulation:/.test(t)) said.push(t);
      if (/are not switches any more/.test(t)) { warned.push(`${m.type()} ${t}`); return; }   // (a retired word: the page says so, as a note — a warning there would be filed as a fault of the page's, cloud/report.ts tapConsole)
      if ((m.type() === 'error' || m.type() === 'warning') && !/Failed to load resource/.test(t)) errs.push(t.slice(0, 300)); });
    if (init) await page.addInitScript(init);
    return { browser, page, errs, said, warned }; };
  const ready = async (page, secs = 900) => { for (const end = Date.now() + secs * 1000; Date.now() < end;) {
      try { const r = await page.evaluate(() => Promise.race([window.__ready.then(() => window.__game && window.__sim ? 'ok' : 'wait', e => 'dead: ' + e), new Promise(res => setTimeout(() => res('wait'), 2000))])); if (r !== 'wait') return r; } catch { /* between documents */ }
      await page.waitForTimeout(400); } return 'no boot in time'; };
  const rowsAsPlayed = (s) => s.on.join() === s.def.join() && !s.maths;
  let played = null, defNames = null, tapeText = '';   // (the game every other job is held to, the rows the game is played on, and the replay file of `plain`'s 240 frames)

  if (want('loads')) { const name = 'loads', { browser, page, errs, said } = await open(); try {
    await page.goto(`${BASE}?${ADDR}`); const r = await ready(page); if (r !== 'ok') throw new Error(r);
    const s = await page.evaluate(SEEN); log(` ${name}: ${s.say}`);
    if (s.why || !rowsAsPlayed(s) || s.inMath || !/^rust simulation: \d+ bytes, started in/.test(s.say)) throw new Error(`the module is not started with the rows the game is played on (${s.def.join(' ')}): ${JSON.stringify(s)}`);
    if (!said.some(l => /^rust simulation: \d+ bytes, started in/.test(l))) throw new Error(`the boot did not say the module started: ${JSON.stringify(said)}`);
    defNames = s.def;
    const f = await page.evaluate(FOLD, BREAK); log(` ${name}: the stream in the page — ${JSON.stringify({ ...f, first: f.first.slice(0, 2) })}`);
    played = await page.evaluate(PLAY, FRAMES);
    const bun = cp.spawnSync('bun', ['run', 'tools/qa/rust-check.ts', '--hash'], { cwd: ROOT, encoding: 'utf8' }), line = (bun.stdout || '').split('\n').find(l => l.startsWith('RUST-HASH '));
    const there = line ? JSON.parse(line.slice(10)) : null; if (!there) throw new Error(`bun did not fold the stream: ${(bun.stderr || '').slice(-200)}`);
    if (f.differ !== 0 || f.own !== f.rust) throw new Error(`${f.differ} of ${f.asked} answers differ between the page's own maths and the module (${f.own} · ${f.rust}): ${f.first.slice(0, 3).join(' | ')}`);
    if (f.rust !== there.word) throw new Error(`the module folds the stream to ${f.rust} in the page and ${there.word} under ${there.engine}`);
    if (errs.length) throw new Error(`the page complained: ${errs.slice(0, 3).join(' | ')}`);
    pass(name, `${s.say} · ${(f.asked / 1e6).toFixed(1)} million answers in the page, 0 differ, word ${f.rust} = ${there.engine}'s (${f.ms} ms) · ${FRAMES} frames: digest ${played.digest}, bits ${played.bits}`);
  } catch (e) { fail(name, String(e.message || e)); } await browser.close(); }

  // THE PAGE AGAIN, COUNTED: the same game, the module's copy opened by every tick, the fog's pass asking men of it, and nothing of a sealed row answered in TypeScript
  if (want('plain')) { const name = 'plain', { browser, page, errs } = await open(); try {
    if (!played || !defNames) throw new Error('`loads` did not play: nothing to hold this game to (run both jobs)');
    await page.goto(`${BASE}?${ADDR}${process.env.ALONE_WHO === '1' ? '&fell=who' : '&fell'}`); const r = await ready(page); if (r !== 'ok') throw new Error(r);
    const s = await page.evaluate(SEEN); log(` ${name}: ${s.say}`);
    if (s.why || !rowsAsPlayed(s)) throw new Error(`the page does not stand on the rows the game is played on: ${JSON.stringify(s)}`);
    await page.evaluate(() => { const P = Object.getPrototypeOf(window.__game.col), open = P.tickOpen, see = P.eyesSee, k = window.__asked = { lines: 0, circles: 0, fog: 0, sees: 0 }; let asked = true;   // (`fog`: the ticks in which the fog's pass asked the module a man — the pass is no row, and is the module's whenever it runs)
      P.tickOpen = function () { open.call(this); if (this.mirror.lines) k.lines++; if (this.mirror.circles) k.circles++; asked = false; };
      P.eyesSee = function (...a) { const r = see.apply(this, a); k.sees++; if (!asked) { asked = true; k.fog++; } return r; }; });
    // (the red path: the module's answer to every line through the statics replaced by "clear" — a wrong walk that still plays)
    if (BREAK) await page.evaluate(() => { const M = window.__game.col.mirror, P = Object.getPrototypeOf(M), open = P.open; P.open = function (...a) { open.apply(this, a); if (!this.broke) { this.broke = true; this.x = Object.assign({}, this.x, { lines_blocked: () => 0 }); } }; });
    const d = await page.evaluate(PLAY, FRAMES), t = await page.evaluate(SEEN), k = await page.evaluate(() => window.__asked);
    if (errs.length) throw new Error(`the page complained: ${errs.slice(0, 3).join(' | ')}`);
    if (!(t.copy >= 0)) throw new Error(`no tick opened the module's copy of the box world (its world: ${t.copy})`);
    for (const row of ['lines', 'circles']) if (!(k[row] > 0)) throw new Error(`no tick opened the module's copy with the row ${row} (${JSON.stringify(k)})`);
    if (!(k.sees > 0 && k.fog > 0)) throw new Error(`the fog's pass asked no man of the module in ${FRAMES} frames (${JSON.stringify(k)})`);
    if (d.digest !== played.digest || d.bits !== played.bits) throw new Error(`two loads of the page are two games: digest ${d.digest} · ${played.digest}, bits ${d.bits} · ${played.bits}`);
    // (ONE WAY IN: with ?fell on the address the page counts every question its TypeScript answered while the row was up, from the level's building on — src/sim/sim.ts FELL.
    //  A count on a row of SEALED is a way back into TypeScript that a boot or a tick of the real page still takes)
    const fellSay = await page.evaluate(() => { const L = window.__sim.fell(), sealed = window.__sim.fellSealed(); return (sealed.length ? 'SEALED ' + sealed.join(', ') + ' · ' : '') + (L.length ? L.join(' | ') : 'none'); });
    const alone = await page.evaluate(() => window.__sim.alone?.() ?? null);   // (the copy's proofs outside a tick over these frames: questions that proved it for themselves, and windows a question proved — scene/collision.ts ANYTIME)
    if (fellSay.startsWith('SEALED')) throw new Error(`a sealed row's question was answered by the TypeScript in the page: ${fellSay.slice(0, 600)}`);
    tapeText = await page.evaluate(() => JSON.stringify(window.__replay?.file?.('rust-page') ?? null)).catch(() => '');   // (the replay of these frames: `replay` plays it)
    pass(name, `${s.say} · ${FRAMES} frames, ${k.lines} ticks on the module's lines, ${k.circles} on its circles, ${k.fog} with its sight pass (${k.sees} men asked): digest ${d.digest}, bits ${d.bits} — the game \`loads\` played, to the bit · the TypeScript answered: ${fellSay.slice(0, 400)}${alone ? ` · outside a tick the copy was proven ${alone[0]} times for a question alone and ${alone[1]} times for a window${alone[2] ? ' · alone: ' + Object.entries(alone[2]).sort((a, b) => b[1] - a[1]).slice(0, 8).map(([k, v]) => `${v} × ${k}`).join(' | ') : ''}` : ''}`);
  } catch (e) { fail(name, String(e.message || e)); } await browser.close(); }

  // THE RETIRED WORDS DO NOTHING, AND THE PAGE SAYS SO
  if (want('words')) { const name = 'words', { browser, page, errs, warned } = await open(); try {
    if (!played || !defNames || !defNames.length) throw new Error('`loads` did not play: nothing to hold this game to (run both jobs)');
    const words = ['rust=0', `rust${defNames[0]}=0`, 'noexact'];
    await page.goto(`${BASE}?${ADDR}&${words.join('&')}`); const r = await ready(page); if (r !== 'ok') throw new Error(r);
    const s = await page.evaluate(SEEN); log(` ${name}: ${s.say}`);
    if (s.why || !rowsAsPlayed(s)) throw new Error(`a retired word moved a row: ${JSON.stringify(s)}`);
    if (warned.some(l => !/^(info|log) /.test(l))) throw new Error(`the page said a retired word as a warning, which is reported as its fault: ${JSON.stringify(warned)}`);
    if (warned.length !== words.length) throw new Error(`the page said ${warned.length} things of ${words.length} old words (once each is wanted): ${JSON.stringify(warned)}`);
    for (const w of words) if (!warned.some(l => l.replace(/^\w+ /, '').startsWith('?' + w.split('=')[0] + ':'))) throw new Error(`the page did not say that ?${w.split('=')[0]} does nothing (it said: ${JSON.stringify(warned)})`);
    const d = await page.evaluate(PLAY, FRAMES);
    if (errs.length) throw new Error(`the page complained: ${errs.slice(0, 3).join(' | ')}`);
    if (d.digest !== played.digest || d.bits !== played.bits) throw new Error(`the page under the retired words plays another game: digest ${d.digest} · ${played.digest}, bits ${d.bits} · ${played.bits}`);
    pass(name, `?${words.join('&')}: the same rows, the same game (digest ${d.digest}), and the page said of each word that it does nothing`);
  } catch (e) { fail(name, String(e.message || e)); } await browser.close(); }

  // A REPLAY'S PAGE: `plain`'s frames as a replay file, opened as ?playtape=<file>. Such a page is first sent on to its recipe's address and then lays the
  // recipe's knob tables over its own; the rows are no knobs, so it stands on the rows the game is played on, and the tape plays to its end with no man
  // off his trace.
  if (want('replay')) { const name = 'replay'; try {
    if (!defNames) throw new Error('`loads` did not say which rows the game is played on (run both jobs)');
    const T = tapeText && tapeText !== 'null' ? JSON.parse(tapeText) : null;
    if (!T || !T.rerun || 'dead' in T.rerun) throw new Error(`\`plain\` left no replay with a run tape (${T ? (T.rerun ? 'its tape ended early: ' + T.rerun.dead : 'no tape in the file') : 'no file'}): run both jobs`);
    const seen = [];
    for (const word of ['']) { const { browser, page, errs } = await open(); try {   // (one page: an old word typed on a replay's address is not a visit word, so the page it is sent on to never has it — nothing to open a second time)
      await page.route('**/qa-rust-tape.json', r => r.fulfill({ contentType: 'application/json', body: tapeText }));
      await page.goto(`${BASE}?paused&mute&quality=${QUALITY}&playtape=qa-rust-tape.json${word}`);
      let r = 'no boot in time'; for (const end = Date.now() + 900000; Date.now() < end;) {   // (the page moves once, to the tape's own address: wait for THAT document)
        try { r = await page.evaluate(() => /[?&]roomclear/.test(location.search) && window.__ready ? Promise.race([window.__ready.then(() => window.__game && window.__sim && window.__tape ? 'ok' : 'wait', e => 'dead: ' + e), new Promise(res => setTimeout(() => res('wait'), 2000))]) : 'wait'); if (r !== 'wait') break; } catch { /* between documents */ }
        await page.waitForTimeout(400); }
      if (r !== 'ok') throw new Error(`the replay's page ${word || '(no word)'}: ${r}`);
      const s = await page.evaluate(SEEN);
      if (s.why || !rowsAsPlayed(s)) throw new Error(`the replay's page ${word || 'with no word'} stands on the rows [${s.on}], not [${s.def}]`);
      const st = await page.evaluate(() => { const e = window.__engine, real = e.render; e.render = () => { e.world.upload(); }; try { let t = window.__tape.state, n = 0; while (!t.ended && n++ < 400) t = window.__tape.run(60, false); return t; } finally { e.render = real; } });
      if (errs.length) throw new Error(`the page complained: ${errs.slice(0, 3).join(' | ')}`);
      if (!st.ended || st.drift) throw new Error(`the tape ${word || 'with no word'} did not play to its end on the trace: ${JSON.stringify({ f: st.f, ticks: st.ticks, ended: st.ended, drift: st.drift })}`);
      seen.push(`${word ? '?rust=0' : 'no word'}: ${st.f} of ${st.ticks} steps, never off the trace`);
    } finally { await browser.close(); } }
    pass(name, `a replay of the page's own frames, on the rows the game is played on · ${seen.join(' · ')}`);
  } catch (e) { fail(name, String(e.message || e)); } }

  // NO WEBASSEMBLY: THE CARD, AND NO GAME
  if (want('nojit')) { const name = 'nojit', { browser, page, errs, said } = await open(() => { delete globalThis.WebAssembly; }); try {
    await page.goto(`${BASE}?${ADDR}`);
    let r = 'no boot in time'; for (const end = Date.now() + 300000; Date.now() < end;) {
      try { r = await page.evaluate(() => window.__ready ? Promise.race([window.__ready.then(() => 'booted', e => 'refused: ' + (e && e.name)), new Promise(res => setTimeout(() => res('wait'), 2000))]) : 'wait'); if (r !== 'wait') break; } catch { /* between documents */ }
      await page.waitForTimeout(400); }
    const s = await page.evaluate(() => ({ wasm: typeof WebAssembly, game: !!window.__game, card: (() => { const c = document.getElementById('fatal'); return c && c.style.display === 'flex' ? c.textContent : ''; })() }));
    log(` ${name}: ${r} · the card: ${JSON.stringify(s.card)}`);
    if (s.wasm !== 'undefined') throw new Error('the page still has WebAssembly: nothing was tried');
    if (r !== 'refused: NoWasmError') throw new Error(`a page with no WebAssembly: its boot ${r}, not refused for want of the module`);
    if (s.game) throw new Error('a page with no WebAssembly built a game');
    if (s.card !== WORDS) throw new Error(`the card does not say the maintainer's words: ${JSON.stringify(s.card)} · wanted ${JSON.stringify(WORDS)}`);
    if (!said.some(l => /NOT RUNNING/.test(l))) throw new Error(`the console did not say the module is not running: ${JSON.stringify(said)}`);
    const other = errs.filter(e => !/NoWasmError|no WebAssembly/.test(e)); if (other.length) throw new Error(`the page complained of something else: ${other.slice(0, 3).join(' | ')}`);
    pass(name, `no WebAssembly: no game is built and the card reads ${JSON.stringify(s.card)}`);
  } catch (e) { fail(name, String(e.message || e)); } await browser.close(); }

  console.log(red ? `\n${red} FAILED` : '\nall pass'); process.exit(red ? 1 : 0);
})();
