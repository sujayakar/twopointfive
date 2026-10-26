// SHIFT + THE KEY ADDS A GOAL, ON THE REAL PAGE (src/game/planmode.ts planGoal, CMDRING.goals): headless Chromium on the software WebGPU
// adapter, the page as it opens on the room clear with its squad of four, the keys pressed as a hand presses them.
//   PORT=5247 node tools/qa/goals-page.cjs     (a dev server on that port: PORT=5247 bun run tools/serve.ts)
//   first    Shift + Q on a door, the squad behind him: all four are sent to stack on it
//   second   Shift + Q on another door: two men are sent to stack there and two are still on the first
//   held     Shift + Q kept down on the first door again: the wheel comes up for the two men on that door, and the back key puts it away with no order
//   word     F, the second pair still on its way: the door tools start an entry on each of the two doors
//   off      the same page with ?goals=0: Shift + Q is Q, and on a second door all four go there
//   exit 0 = all of it with no page error · 1 = otherwise. The verdict is the order book's own state in the page: nothing here judges a picture, and
//   the software adapter is not a GPU.
const { chromium } = require('/opt/node-tools/node_modules/playwright');
const PORT = process.env.PORT || 5247, QUALITY = process.env.QUALITY || 'verylow';
const FLAGS = ['--enable-unsafe-webgpu', '--use-gl=angle', '--use-angle=swiftshader', '--enable-features=Vulkan', '--use-vulkan=swiftshader', '--ignore-gpu-blocklist', '--disable-gpu-watchdog'];
const T0 = Date.now(), log = (...a) => console.log(`[${((Date.now() - T0) / 1000).toFixed(0)}s]`, ...a);
const URL = `http://localhost:${PORT}/?roomclear&squad4&paused&quality=${QUALITY}&mute`;
let red = 0;
const check = (ok, what, got) => { if (!ok) red++; console.log(`- ${ok ? 'PASS' : 'FAIL'} — ${what}: ${got}`); };

// in the page: the cursor put on a door the squad's key reads as that door, the nearest first, never one of `skip` (its name; null: none)
const ON_DOOR = (skip) => { const g = window.__game, O = window.__orders, c = document.getElementById('view'), w = c.clientWidth, h = c.clientHeight, s = g.player.char.pos;
  const ds = [...g.doors.list].filter(d => !skip.includes(d.def.name)).sort((a, b) => Math.hypot(a.frameCentre[0] - s[0], a.frameCentre[2] - s[2]) - Math.hypot(b.frameCentre[0] - s[0], b.frameCentre[2] - s[2]));
  for (const d of ds.slice(0, 6)) { const th = d.def.closedDir, nx = -Math.sin(th), nz = Math.cos(th);
    for (const y of [d.frameCentre[1], 0.05, 1.0, 1.8]) for (const k of [0, 0.35, -0.35]) { const [x, yy, front] = window.__cam.project([d.frameCentre[0] + nx * k, y, d.frameCentre[2] + nz * k], w, h); if (!front) continue;
      window.__mouse(x, yy); window.__runFrames(2, 1 / 60, [], false); const t = O.targetUnder(window.__input, window.__cam, c); if (t.kind === 'door' && t.door === d) return d.def.name; } }
  return null; };
// in the page: who is on a plain stack by door, the wheel and whose it is, the last order and the door tools' entries
const READ = () => { const g = window.__game, O = window.__orders, by = {}; for (const m of O.men('all')) { const A = O.of(m); if (A && A.order === 'stack' && A.door) (by[A.door.def.name] ??= []).push(m.callsign); }
  const row = O.log[O.log.length - 1]; return { ring: !!O.cmdRing, part: O.menu && O.menu.part ? O.menu.part.map(m => m.callsign) : null, men: O.men('all').length, by, n: O.log.length, last: row ? `${row.order} to ${(row.who ?? []).length}` : 'none', ops: g.doorTools.ops.map(o => `${o.kind} ${o.door.def.name}`) }; };

(async () => {
  const browser = await chromium.launch({ headless: true, channel: 'chromium', args: FLAGS });
  const page = await (await browser.newContext({ viewport: { width: 1280, height: 720 }, deviceScaleFactor: 1 })).newPage();
  const errs = []; page.on('pageerror', e => errs.push('pageerror ' + String(e).slice(0, 300))); page.on('console', m => { if ((m.type() === 'error' && !/Failed to load resource/.test(m.text())) || /Invalid|error:/.test(m.text())) errs.push('console ' + m.text().slice(0, 300)); });
  const run = n => page.evaluate(n => { window.__runFrames(n, 1 / 60, [], false); }, n);
  page.setDefaultNavigationTimeout(900000);   // (a boot on the software adapter is minutes)
  const boot = async (url) => { await page.goto(url); await page.evaluate(async () => { await window.__ready; });
    await page.evaluate(() => { window.__pauseLoop = true; window.__testVisible = true; window.__game.godMode = true; window.__runFrames(90, 1 / 60, [], false); for (const m of window.__orders.squad()) m.god = true; }); };
  /** Shift held, the squad's key tapped, where the cursor is */
  const shiftQ = async () => { await page.keyboard.down('ShiftLeft'); await run(2); await page.keyboard.down('KeyQ'); await run(2); await page.keyboard.up('KeyQ'); await run(4); await page.keyboard.up('ShiftLeft'); await run(6); };

  await boot(URL); log('ready');
  const d1 = await page.evaluate(ON_DOOR, []); await shiftQ();
  const a = await page.evaluate(READ);
  check(!!d1 && a.men === 4 && (a.by[d1] ?? []).length === 4, 'first — Shift + Q on a door: all four are sent to stack on it',
    `the door ${d1} · sent to stack: ${JSON.stringify(a.by)} · the last order: ${a.last}`);

  const d2 = await page.evaluate(ON_DOOR, [d1]); await shiftQ();
  const b = await page.evaluate(READ);
  check(!!d2 && (b.by[d1] ?? []).length === 2 && (b.by[d2] ?? []).length === 2, 'second — Shift + Q on another door: two men are sent to stack there and two are still on the first',
    `the door ${d2} · on a stack: ${JSON.stringify(b.by)} · the last order: ${b.last}`);

  // (kept down on the door two of them are on: the wheel is theirs — CMDRING.goalRing — and the back key gives nothing: Tab on the page, where Esc is the pause — game/backkey.ts)
  const back = await page.evaluate(ON_DOOR, [d2]), nH = b.n;
  await page.keyboard.down('ShiftLeft'); await run(2); await page.keyboard.down('KeyQ'); await run(45); const h = await page.evaluate(READ);
  await page.keyboard.up('KeyQ'); await run(2); await page.keyboard.up('ShiftLeft'); await run(2); await page.keyboard.press('Tab'); await run(6); const h2 = await page.evaluate(READ);
  check(back === d1 && h.ring && !!h.part && h.part.length === 2 && h.part.every(n => (b.by[d1] ?? []).includes(n)) && !h2.ring && h2.n === nH, 'held — Shift + Q kept down on the first door: the wheel is up for the two men on it, and the back key (Tab on the page) puts it away with no order',
    `the cursor back on ${back} · the wheel ${h.ring ? 'up' : 'NOT up'} for ${h.part ? h.part.join(', ') : 'everyone'} · after the back key: the wheel ${h2.ring ? 'STILL up' : 'gone'}, orders given ${h2.n - nH}`);

  // (the word at once, the second pair still walking: the door tools make an entry of each stack as the word is given and wait for the stacks themselves —
  //  a frame on the software adapter is too slow to walk the men to their marks here; both stacks stood and men through both doors is goals-check's line 3)
  await run(20);
  await page.keyboard.down('KeyF'); await run(2); await page.keyboard.up('KeyF'); await run(30);
  const c = await page.evaluate(READ), doors = new Set(c.ops.map(o => o.split(' ')[1]));
  check(doors.has(d1) && doors.has(d2), 'word — F: the door tools start an entry on each of the two doors', `entries: ${c.ops.join(' + ') || 'none'}`);

  await boot(URL + '&goals=0'); log('ready, ?goals=0');
  const n0 = (await page.evaluate(READ)).n; const d3 = await page.evaluate(ON_DOOR, []); await shiftQ();
  const d = await page.evaluate(READ);
  const d4 = await page.evaluate(ON_DOOR, [d3]); await shiftQ(); const e = await page.evaluate(READ);
  check(!!d3 && !!d4 && d.n > n0 && (d.by[d3] ?? []).length === 4 && (e.by[d4] ?? []).length === 4 && !(e.by[d3] ?? []).length, 'off — the same page with ?goals=0: Shift + Q is Q, and on a second door all four go there',
    `on ${d3}: ${JSON.stringify(d.by)} · then on ${d4}: ${JSON.stringify(e.by)}`);

  check(!errs.length, 'no page error', errs.length ? errs.join(' | ') : 'none');
  await browser.close();
  console.log(red ? `=== goals-page: ${red} FAIL ===` : '=== goals-page: ALL PASS ===');
  process.exit(red ? 1 : 0);
})().catch(e => { console.log('- FAIL — the drive stopped: ' + String(e).slice(0, 600)); process.exit(1); });
