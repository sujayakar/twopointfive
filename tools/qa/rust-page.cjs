// THE SIMULATION IN RUST ON THE REAL PAGE (src/sim/sim.ts): headless Chromium on the software WebGPU adapter (tools/qa/page-look.cjs's recipe).
//   PORT=5199 node tools/qa/rust-page.cjs      (a dev server on that port: PORT=5199 bun run tools/serve.ts; JOBS=a,b = only those jobs)
//   loads    a mission's page with ?rust=0 (the rows the page has on by default put off: the game in TypeScript): the module compiled and started in the page's
//            own thread, in one step, and said so on the console; no row is on. The fixed
//            stream of tools/qa/rust-fold.js through the page's maths (the TypeScript functions, in Chrome's V8) and through the module: every answer the same
//            bits, and the module's word is the one bun folds the same stream to (asked of `bun run tools/qa/rust-check.ts --hash` here)
//   rows     the same address with ?rust=0&rustmaths: Math's functions and the lengths are the module's, and the game after 240 frames is, by digest and by the
//            chained raw bits of every frame, the game `loads` played on the TypeScript maths
//   plain    the address as a player opens it, no word about the rows: the rows of src/sim/sim.ts RUST_ON are on (and no other), every tick opens the module's
//            copy of the box world with each of them up, the fog's pass asks men of it, and the game after 240 frames is the game `loads` played in
//            TypeScript, by digest and by every frame's bits
//   systems  the same with every row but the maths raised by name (?rustlines&rustcircles&rustfog …: whatever src/sim/sim.ts RUST lists): the box world's
//            questions are asked of the module's copy inside every tick (src/scene/boxMirror.ts), and the game is the same game, as `plain`
//            …and with ?fell the page counts the questions its TypeScript answered while a row was up: none may be of a row that is one way in (sim.ts SEALED)
//   words    ?rust<row>=0 for the first default row: that row, and it alone, is off (the page is not played)
//   replay   `plain`'s frames as a replay file, opened as ?playtape=<file>: with no word the page stands on the recipe's rows (the three), with ?rust=0 the
//            word is still on the address the page is sent on to, every row is off, and the tape plays to its end on its trace either way
//   nojit    the same address in a page with no WebAssembly (a browser whose policy turns its JIT off has none): the page boots, says the module is not
//            running, leaves every row off (the default ones, asked for by saying nothing, and the rest, asked for by name), and plays the same game by digest
//            ALONE_WHO=1: …and says who asked the box world a question alone, outside a tick and every window (the address carries ?fell=who)
// exit 0 = every job, with no page error · 1 = otherwise. RUST_BREAK=1: `loads` hands the module's set the ENGINE's cosine, `rows` plays on it, and `systems`
// and `plain` are told every line is clear: all four must FAIL.
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
const SEEN = () => ({ why: window.__sim.whyNot(), say: window.__sim.say(), def: [...window.__sim.RUST_ON], maths: window.__sim.RUST.maths, on: Object.keys(window.__sim.RUST).filter(k => window.__sim.RUST[k]), copy: window.__game.col.mirror.slot, rows: Object.keys(window.__sim.RUST), wasm: typeof WebAssembly,
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
  let played = null, rowNames = null, defNames = null, tapeText = '';   // (the game every other job is held to, the rows the page's table lists, those the page has on by default, and the replay file of `plain`'s 240 frames)

  if (want('loads')) { const name = 'loads', { browser, page, errs, said } = await open(); try {
    await page.goto(`${BASE}?${ADDR}&rust=0`); const r = await ready(page); if (r !== 'ok') throw new Error(r);
    const s = await page.evaluate(SEEN); log(` ${name}: ${s.say}`);
    if (s.why || s.on.length || s.inMath || !s.exact || !/^rust simulation: \d+ bytes, started in/.test(s.say)) throw new Error(`the module is not started with every row off: ${JSON.stringify(s)}`);
    if (!said.some(l => /^rust simulation: \d+ bytes, started in/.test(l))) throw new Error(`the boot did not say the module started: ${JSON.stringify(said)}`);
    rowNames = s.rows; defNames = s.def; played = await page.evaluate(PLAY, FRAMES);   // (first: the game `rows` and `nojit` are held to stands whatever the stream below says)
    const f = await page.evaluate(FOLD, BREAK); log(` ${name}: the stream in the page — ${JSON.stringify({ ...f, first: f.first.slice(0, 2) })}`);
    const bun = cp.spawnSync('bun', ['run', 'tools/qa/rust-check.ts', '--hash'], { cwd: ROOT, encoding: 'utf8' }), line = (bun.stdout || '').split('\n').find(l => l.startsWith('RUST-HASH '));
    const there = line ? JSON.parse(line.slice(10)) : null; if (!there) throw new Error(`bun did not fold the stream: ${(bun.stderr || '').slice(-200)}`);
    if (f.differ !== 0 || f.own !== f.rust) throw new Error(`${f.differ} of ${f.asked} answers differ between the page's TypeScript maths and the module (${f.own} · ${f.rust}): ${f.first.slice(0, 3).join(' | ')}`);
    if (f.rust !== there.word) throw new Error(`the module folds the stream to ${f.rust} in the page and ${there.word} under ${there.engine}`);
    if (errs.length) throw new Error(`the page complained: ${errs.slice(0, 3).join(' | ')}`);
    pass(name, `${s.say} · ${(f.asked / 1e6).toFixed(1)} million answers in the page, 0 differ, word ${f.rust} = ${there.engine}'s (${f.ms} ms) · ${FRAMES} frames on the TypeScript maths: digest ${played.digest}, bits ${played.bits}`);
  } catch (e) { fail(name, String(e.message || e)); } await browser.close(); }

  if (want('rows')) { const name = 'rows', { browser, page, errs } = await open(); try {
    await page.goto(`${BASE}?${ADDR}&rust=0&rustmaths`); const r = await ready(page); if (r !== 'ok') throw new Error(r);
    const s = await page.evaluate(SEEN); log(` ${name}: ${s.say}`);
    if (s.why || !s.maths || !s.inMath || !/in Rust: maths/.test(s.say)) throw new Error(`?rustmaths did not put the module's maths in Math's place: ${JSON.stringify(s)}`);
    if (BREAK) await page.evaluate(() => { const x = new Float64Array(1), u = new Uint32Array(x.buffer), cos = Math.cos; Math.cos = (a) => { x[0] = cos(a); u[0] ^= 1; return x[0]; }; });
    const d = await page.evaluate(PLAY, FRAMES);
    if (errs.length) throw new Error(`the page complained: ${errs.slice(0, 3).join(' | ')}`);
    if (!played) throw new Error('`loads` did not play: nothing to hold this game to (run both jobs)');
    if (d.digest !== played.digest || d.bits !== played.bits) throw new Error(`the game on the module's maths is not the game on the TypeScript maths: digest ${d.digest} · ${played.digest}, bits ${d.bits} · ${played.bits}`);
    pass(name, `${s.say} · ${FRAMES} frames: digest ${d.digest}, bits ${d.bits} — the game on the TypeScript maths, to the bit`);
  } catch (e) { fail(name, String(e.message || e)); } await browser.close(); }

  // A PAGE WHOSE BOX WORLD IS ASKED OF THE MODULE (`plain`, `systems`): the rows it must have on and no other, the ticks that opened the module's copy with each
  // of those rows up and the men the fog's pass asked of it, and the game held to the one `loads` played in TypeScript
  const boxJob = async (name, rowsOf, addrOf, what) => { const { browser, page, errs } = await open(); try {
    if (!rowNames || !defNames) throw new Error('`loads` did not say which rows there are (run both jobs)');
    const up = rowsOf(rowNames, defNames);
    await page.goto(`${BASE}?${ADDR}${addrOf(up)}`); const r = await ready(page); if (r !== 'ok') throw new Error(r);
    const s = await page.evaluate(SEEN); log(` ${name}: ${s.say}`);
    if (s.why || s.maths || !up.length || s.on.join() !== up.join() || !/in Rust: lines/.test(s.say)) throw new Error(`${what} does not have exactly the rows ${up.join(', ')} on: ${JSON.stringify(s)}`);
    await page.evaluate(() => { const P = Object.getPrototypeOf(window.__game.col), open = P.tickOpen, see = P.eyesSee, k = window.__asked = { lines: 0, circles: 0, fog: 0, sees: 0 };
      P.tickOpen = function () { open.call(this); if (this.mirror.lines) k.lines++; if (this.mirror.circles) k.circles++; if (this.mirror.sight) k.fog++; };
      P.eyesSee = function (...a) { const r = see.apply(this, a); if (r >= 0) k.sees++; return r; }; });
    // (the red path: the module's answer to every line through the statics replaced by "clear" — a wrong walk that still plays)
    if (BREAK) await page.evaluate(() => { const M = window.__game.col.mirror, P = Object.getPrototypeOf(M), open = P.open; P.open = function (...a) { open.apply(this, a); if (!this.broke) { this.broke = true; this.x = Object.assign({}, this.x, { lines_blocked: () => 0 }); } }; });
    const d = await page.evaluate(PLAY, FRAMES), t = await page.evaluate(SEEN), k = await page.evaluate(() => window.__asked);
    if (errs.length) throw new Error(`the page complained: ${errs.slice(0, 3).join(' | ')}`);
    if (!(t.copy >= 0)) throw new Error(`no tick opened the module's copy of the box world (its world: ${t.copy})`);
    for (const row of ['lines', 'circles', 'fog']) if (up.includes(row) !== (k[row] > 0)) throw new Error(`the row ${row} is ${up.includes(row) ? 'on' : 'off'} and ${k[row]} ticks opened the module's copy with it (${JSON.stringify(k)})`);
    if (up.includes('fog') && !(k.sees > 0)) throw new Error(`the fog's pass asked no man of the module in ${FRAMES} frames (${JSON.stringify(k)})`);
    if (!played) throw new Error('`loads` did not play: nothing to hold this game to (run both jobs)');
    if (d.digest !== played.digest || d.bits !== played.bits) throw new Error(`${what} (${up.join(', ')} in the module) is not the game in TypeScript: digest ${d.digest} · ${played.digest}, bits ${d.bits} · ${played.bits}`);
    // (ONE WAY IN: with ?fell on the address the page counts every question its TypeScript answered while the row was up, from the level's building on — src/sim/sim.ts FELL.
    //  A count on a row of SEALED is a way back into TypeScript that a boot or a tick of the real page still takes)
    const fellSay = name !== 'systems' ? '' : await page.evaluate(() => { const L = window.__sim.fell(), sealed = window.__sim.fellSealed(); return (sealed.length ? 'SEALED ' + sealed.join(', ') + ' · ' : '') + (L.length ? L.join(' | ') : 'none'); });
    const alone = name !== 'systems' ? null : await page.evaluate(() => window.__sim.alone?.() ?? null);   // (the copy's proofs outside a tick over these frames: questions that proved it for themselves, and windows a question proved — scene/collision.ts ANYTIME)
    if (fellSay.startsWith('SEALED')) throw new Error(`a sealed row's question was answered by the TypeScript in the page: ${fellSay.slice(0, 600)}`);
    if (name === 'plain') tapeText = await page.evaluate(() => JSON.stringify(window.__replay?.file?.('rust-page') ?? null)).catch(() => '');   // (the replay of these frames, recorded with the rows on: `replay` plays it)
    pass(name, `${s.say} · ${FRAMES} frames, ${k.lines} ticks on the module's lines, ${k.circles} on its circles, ${k.fog} with its sight pass (${k.sees} men asked): digest ${d.digest}, bits ${d.bits} — the game in TypeScript, to the bit${fellSay ? ` · the TypeScript answered: ${fellSay.slice(0, 400)}` : ''}${alone ? ` · outside a tick the copy was proven ${alone[0]} times for a question alone and ${alone[1]} times for a window${alone[2] ? ' · alone: ' + Object.entries(alone[2]).sort((a, b) => b[1] - a[1]).slice(0, 8).map(([k, v]) => `${v} × ${k}`).join(' | ') : ''}` : ''}`);
  } catch (e) { fail(name, String(e.message || e)); } await browser.close(); };
  if (want('plain')) await boxJob('plain', (rows, def) => rows.filter(r => def.includes(r)), () => '', 'the page as a player opens it');
  if (want('systems')) await boxJob('systems', (rows) => rows.filter(r => r !== 'maths'), (up) => up.map(r => '&rust' + r).join('') + (process.env.ALONE_WHO === '1' ? '&fell=who' : '&fell'), 'the address that names every row but the maths');

  if (want('words')) { const name = 'words', { browser, page, errs } = await open(); try {
    if (!rowNames || !defNames || !defNames.length) throw new Error('`loads` did not say which rows there are (run both jobs)');
    const off = defNames[0], left = rowNames.filter(r => defNames.includes(r) && r !== off);
    await page.goto(`${BASE}?${ADDR}&rust${off}=0`); const r = await ready(page); if (r !== 'ok') throw new Error(r);
    const s = await page.evaluate(SEEN); log(` ${name}: ${s.say}`);
    if (s.why || s.on.join() !== left.join()) throw new Error(`?rust${off}=0 did not put that row, and it alone, off: ${JSON.stringify(s)}`);
    if (errs.length) throw new Error(`the page complained: ${errs.slice(0, 3).join(' | ')}`);
    pass(name, `?rust${off}=0: ${s.say}`);
  } catch (e) { fail(name, String(e.message || e)); } await browser.close(); }

  // A REPLAY'S PAGE AND THE ADDRESS'S WORD: `plain`'s frames as a replay file (recorded with the default rows on), opened as ?playtape=<file>. Such a page is
  // first sent on to its recipe's address and then lays the recipe's knob tables, its rows among them, over its own. With no word about the rows it stands
  // on the recipe's rows; with ?rust=0 the word is still on the address after the move, every row is off, and the tape plays to its end with no man off
  // his trace: the game recorded in the module, played back in TypeScript.
  if (want('replay')) { const name = 'replay'; try {
    if (!rowNames || !defNames) throw new Error('`loads` did not say which rows there are (run both jobs)');
    const T = tapeText && tapeText !== 'null' ? JSON.parse(tapeText) : null;
    if (!T || !T.rerun || 'dead' in T.rerun) throw new Error(`\`plain\` left no replay with a run tape (${T ? (T.rerun ? 'its tape ended early: ' + T.rerun.dead : 'no tape in the file') : 'no file'}): run both jobs`);
    const seen = [];
    for (const word of ['', '&rust=0']) { const { browser, page, errs } = await open(); try {
      await page.route('**/qa-rust-tape.json', r => r.fulfill({ contentType: 'application/json', body: tapeText }));
      await page.goto(`${BASE}?paused&mute&quality=${QUALITY}&playtape=qa-rust-tape.json${word}`);
      let r = 'no boot in time'; for (const end = Date.now() + 900000; Date.now() < end;) {   // (the page moves once, to the tape's own address: wait for THAT document)
        try { r = await page.evaluate(() => /[?&]roomclear/.test(location.search) && window.__ready ? Promise.race([window.__ready.then(() => window.__game && window.__sim && window.__tape ? 'ok' : 'wait', e => 'dead: ' + e), new Promise(res => setTimeout(() => res('wait'), 2000))]) : 'wait'); if (r !== 'wait') break; } catch { /* between documents */ }
        await page.waitForTimeout(400); }
      if (r !== 'ok') throw new Error(`the replay's page ${word || '(no word)'}: ${r}`);
      const s = await page.evaluate(SEEN), q = await page.evaluate(() => location.search), want = word ? [] : rowNames.filter(k => defNames.includes(k));
      if (s.why || s.on.join() !== want.join() || (word ? !/[?&]rust=0(&|$)/.test(q) : /[?&]rust(=|&|$)/.test(q))) throw new Error(`the replay's page ${word || 'with no word'} stands on the rows [${s.on}], not [${want}] (its address: ${q})`);
      const st = await page.evaluate(() => { const e = window.__engine, real = e.render; e.render = () => { e.world.upload(); }; try { let t = window.__tape.state, n = 0; while (!t.ended && n++ < 400) t = window.__tape.run(60, false); return t; } finally { e.render = real; } });
      if (errs.length) throw new Error(`the page complained: ${errs.slice(0, 3).join(' | ')}`);
      if (!st.ended || st.drift) throw new Error(`the tape ${word || 'with no word'} did not play to its end on the trace: ${JSON.stringify({ f: st.f, ticks: st.ticks, ended: st.ended, drift: st.drift })}`);
      seen.push(`${word ? '?rust=0' : 'no word'}: rows [${s.on.join(' ') || 'none'}], ${st.f} of ${st.ticks} steps, never off the trace`);
    } finally { await browser.close(); } }
    pass(name, `a replay recorded with ${rowNames.filter(k => defNames.includes(k)).join(', ')} in the module · ${seen.join(' · ')}`);
  } catch (e) { fail(name, String(e.message || e)); } }

  if (want('nojit')) { const name = 'nojit', { browser, page, errs, said } = await open(() => { delete globalThis.WebAssembly; }); try {
    // (the rows the page has on by default are asked for by saying nothing, every other by its word: a page with no WebAssembly must leave both kinds off)
    await page.goto(`${BASE}?${ADDR}${(rowNames || ['maths']).filter(r => !(defNames || []).includes(r)).map(r => '&rust' + r).join('')}`); const r = await ready(page); if (r !== 'ok') throw new Error(r);
    const s = await page.evaluate(SEEN); log(` ${name}: ${s.say}`);
    if (s.wasm !== 'undefined' || !s.why || s.on.length || !/NOT RUNNING/.test(s.say) || !said.some(l => /NOT RUNNING/.test(l))) throw new Error(`a page with no WebAssembly: ${JSON.stringify(s)} said ${JSON.stringify(said)}`);
    const d = await page.evaluate(PLAY, FRAMES);
    if (errs.length) throw new Error(`the page complained: ${errs.slice(0, 3).join(' | ')}`);
    if (played && (d.digest !== played.digest || d.bits !== played.bits)) throw new Error(`a page with no WebAssembly plays another game: digest ${d.digest} · ${played.digest}, bits ${d.bits} · ${played.bits}`);
    pass(name, `${s.say} · the rows stayed off · ${FRAMES} frames: digest ${d.digest}${played ? ', the same game' : ''}`);
  } catch (e) { fail(name, String(e.message || e)); } await browser.close(); }

  console.log(red ? `\n${red} FAILED` : '\nall pass'); process.exit(red ? 1 : 0);
})();
