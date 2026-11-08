// WHAT THE PAGE WRITES OF THE GAME (tools/qa/writetrap.cjs says how it is seen). The Rust game's state is written by the Rust game: a page that writes it plays
// another game than the one the module steps, and a game of two machines parts (a door's kept centre, written by the HUD's asking for it, was one such).
//   node tools/qa/writetrap-page.cjs [address]     --seats2: the page in the second seat of a two-player game on a real server (tools/mp), the first seat a bun client's · --say: the rows with a reason too · env: PORT (the dev server's, default 5199) · PLAYWRIGHT_MODULE, CHROMIUM · WRITETRAP_BREAK=block|row plants a write of each kind: it must be named
// A word of the block, or a row, written outside a call of the module is a FAULT unless tools/wire/fields.json `pageWrites` says why that is right (`wire.ts page-writes`).
// Exit 0 pass · 1 a fault. On the software adapter a drawn frame is seconds: a few frames at each drawn stretch of the walk, and its last.
const { chromium } = require(process.env.PLAYWRIGHT_MODULE || 'playwright'), fs = require('fs'), path = require('path'), { install } = require('./writetrap.cjs');
const PORT = process.env.PORT || 5199, Q = process.argv.slice(2).find(a => !a.startsWith('--')) || 'levelFile=pass.json&roomclear&squad4', BREAK = process.env.WRITETRAP_BREAK || '', SAY = process.argv.includes('--say'), TWO = process.argv.includes('--seats2');
const { spawn } = require('node:child_process'), ROOT = path.resolve(__dirname, '../..'), MP = 5960 + process.pid % 30, sleep = ms => new Promise(r => setTimeout(r, ms)), kids = [];
const kid = (args, env) => { const k = spawn('bun', ['run', ...args], { cwd: ROOT, stdio: 'ignore', env: { ...process.env, ...env } }); kids.push(k); return k; }, done = code => { for (const k of kids) k.kill(); process.exit(code); };
const WHY = JSON.parse(fs.readFileSync(path.join(__dirname, '../wire/fields.json'), 'utf8')).pageWrites ?? {}, why = q => WHY[q] ?? WHY[q.slice(0, q.lastIndexOf('.'))];
const FLAGS = ['--enable-unsafe-webgpu', '--use-gl=angle', '--use-angle=swiftshader', '--enable-features=Vulkan', '--use-vulkan=swiftshader', '--ignore-gpu-blocklist', '--disable-gpu-watchdog', '--no-sandbox'];
(async () => { const b = await chromium.launch({ headless: true, ...(process.env.CHROMIUM ? { executablePath: process.env.CHROMIUM } : {}), args: FLAGS }), p = await b.newPage({ viewport: { width: 640, height: 360 } });
  p.on('pageerror', e => console.log('PAGEERROR', String(e).slice(0, 300))); const T0 = Date.now(); p.on('console', m => { if (m.text().startsWith('[walk]')) console.log(`${((Date.now() - T0) / 1000).toFixed(0)} s ${m.text()}`); });
  if (TWO) { kid(['tools/mp/server.ts'], { MP_PORT: String(MP), MP_BUILD: '', MP_IDLE_S: '600', MP_GRACE_S: '900', MP_SETUP: JSON.stringify({ game: 'check', scenario: `?${Q}&seats=2`, seed: 7, seatKeys: ['seat-one', 'seat-two'] }) });
    for (let i = 0; i < 300; i++) { try { await (await fetch(`http://127.0.0.1:${MP}/health`)).json(); break; } catch { await sleep(100); } }
    kid(['tools/mp/bot.ts', '--url', `ws://127.0.0.1:${MP}`, '--game', 'check', '--build', '', '--ticks', '3000', '--timeout', '1200', '--key', 'seat-one']); }
  await p.goto(`http://localhost:${PORT}/?${TWO ? `net=${encodeURIComponent(`ws://127.0.0.1:${MP}`)}&netgame=check&netkey=seat-two` : Q}&paused&keeppage=0&quality=verylow&mute`, { timeout: 180000 });
  if (TWO) await p.waitForFunction(() => new URLSearchParams(location.search).has('seed'), null, { timeout: 120000 });
  await p.waitForFunction(() => window.__ready !== undefined, null, { timeout: 600000 }); await p.evaluate(() => window.__ready);
  await p.evaluate(install);
  await p.evaluate(async TWO => { window.__testVisible = true; const t = performance.now(), S = () => window.__net.state; do { await window.__runFrames(4, 1 / 60, [], 'none'); await new Promise(r => setTimeout(r, 0)); } while (TWO && (S().state !== 'playing' || !S().sent || S().far) && performance.now() - t < 240000); }, TWO);
  const R = await p.evaluate(async ({ BREAK, TWO }) => { const g = window.__game, T = window.__writetrap, said = [];
    let taken = -1e9; const run = async (n, keys = [], draw = 'none') => { for (let i = 0; i < n; i += 30) { if (window.__rust.ticks - taken >= (TWO ? 240 : 30)) { taken = window.__rust.ticks; T.take(); } await window.__runFrames(Math.min(30, n - i), 1 / 60, keys, draw); for (const k of keys) window.__input.keys.delete(k); } };
    const tap = async k => { window.__input.pressed.add(k); await run(4); };
    const doing = async (what, fn) => { console.log(`[walk] ${what}`); try { await fn(); } catch (e) { said.push(`${what}: ${String(e).slice(0, 160)}`); } };
    await run(60);
    // (a game of two: its state is the server's, so nothing is called on it by hand; the walk is what a player's hands do, drawn)
    if (TWO) { const S = () => window.__net.state, tick = () => new Promise(r => setTimeout(r, 0)), t = performance.now(), from = S().confirmed; let n = 0;
      for (const k of ['KeyE', 'KeyQ', 'Digit1', 'Digit3', 'KeyR', 'KeyN', 'KeyC', 'KeyF', 'Tab', 'KeyX', 'Escape']) { await tap(k); for (let i = 0; i < 4; i++) { await run(8); await tick(); } }
      while (S().confirmed < from + 900 && performance.now() - t < 600000 && !S().bad && !S().broken && !S().bye) { n++; await run(6, [['KeyW', 'KeyD', 'KeyS', 'KeyA'][(n >> 3) & 3]], n % 5 ? 'none' : true); if (n % 40 === 0) { window.__input.buttons = 1; await run(6); window.__input.buttons = 0; } await tick(); }
      said.push(`two seats: ${S().confirmed - from} ticks confirmed, words ${S().wordsOk} the same${S().bad ? `, PARTED ${JSON.stringify(S().bad).slice(0, 200)}` : ''}${S().broken ? `, broken ${S().broken}` : ''}`); return { ...T.report(), said, ticks: window.__rust.ticks }; }
    if (BREAK === 'block') await doing('planted', async () => { const d = g.doors.list[0], at = g.hostSeat; g.hostSeat = function () { d.angle = d.angle + 0.01; return at.call(g); }; await run(6); delete g.hostSeat; });
    if (BREAK === 'row') await doing('planted', async () => { const at = g.hostSeat; g.hostSeat = function plantedWrite() { T.plant(); return at.call(g); }; await run(6); delete g.hostSeat; });
    if (BREAK) return { ...T.report(), said, ticks: window.__rust.ticks };
    await doing('walk', async () => { for (const k of ['KeyW', 'KeyD', 'KeyS', 'KeyA']) await run(60, [k]); });
    await doing('keys', async () => { for (const k of ['KeyE', 'KeyQ', 'Digit1', 'Digit2', 'Digit3', 'KeyR', 'KeyN', 'KeyC', 'KeyV', 'KeyG', 'KeyF', 'Tab', 'Tab', 'KeyX', 'KeyZ', 'Space', 'KeyB', 'KeyL', 'KeyH', 'KeyM', 'Escape', 'Escape']) { await tap(k); await run(12); } });
    await doing('doors', async () => { for (const d of (g.doors?.list ?? []).slice(0, 2)) { window.__rust.call('Game.teleportPlayer', g, [[d.def.hinge[0] + 0.9, 0, d.def.hinge[1] + 0.9]]); await run(4, [], true); await tap('KeyE'); await run(36); await run(4, [], true); } });
    await doing('rings', async () => { for (const k of ['KeyQ', 'KeyE']) { await run(36, [k]); await run(4, [k], true); await run(10); } });
    await doing('fire', async () => { window.__input.buttons = 1; await run(60); window.__input.buttons = 2; await run(40); window.__input.buttons = 0; });
    await doing('fight', async () => { for (const h of g.guards.filter(x => x.team !== 'operative').slice(0, 3)) { window.__rust.call('Game.teleportPlayer', g, [[h.char.pos[0] + 3, 0, h.char.pos[2]]]); await run(60); window.__input.buttons = 1; await run(30); window.__input.buttons = 0; await run(60); } });
    await doing('save', async () => { await tap('F5'); await run(30); const s = g.saves.timeline()[0]; said.push(`saves ${g.saves.timeline().length}`); await run(90, ['KeyW']); if (s) { const no = g.saves.load(g, s.key); if (no) said.push(`load: ${no}`); } await run(60); });
    await doing('restart', async () => { g.restart('restarted'); await run(60, ['KeyW']); });
    await doing('draw', async () => { await run(20, [], true); });
    return { ...T.report(), said, ticks: window.__rust.ticks }; }, { BREAK, TWO });
  for (const s of R.said) console.log(`note ${s}`);
  let bad = 0; for (const [kind, L] of [['block', R.block], ['row', R.rows]]) for (const r of L.sort((a, b) => a.q < b.q ? -1 : 1)) { const w = why(kind === 'block' ? `block.${r.q}` : r.q); if (!w) bad++; if (!w || SAY) { console.log(`${w ? 'said ' : 'FAULT'} ${kind} ${r.q} x${r.n}${w ? `: ${w}` : ''}`); for (const [by, n] of r.by) console.log(`        ${n} ${by.slice(0, 260)}`); } }
  if (SAY) for (const s of R.sample) console.log(`  ${s}`);
  console.log(`=== writetrap-page: ${R.trapped} fields trapped, the block ${R.on ? 'held at every call' : 'NOT SEEN'}, ${R.ticks} ticks, ${R.block.length} parts and ${R.rows.length} fields written by the page, ${bad} with no reason${bad ? '' : ', all pass'} ===`);
  await b.close(); done(bad || !R.trapped || !R.on ? 1 : 0); })().catch(e => { console.error(String(e).slice(0, 1500)); done(1); });
