// THE FIRST GOAL SENDS HALF THE SQUAD AND THE KEY TAPPED TWICE IS 'FALL IN', ON THE REAL PAGE (src/game/cmdring.ts CMDRING.goalHalf and
// CMDRING.twice): headless Chromium on the software WebGPU adapter, the page as it opens on the room clear with its squad of four, the keys
// pressed as a hand presses them.
//   PORT=5247 node tools/qa/halftwice-page.cjs     (a dev server on that port: PORT=5247 bun run tools/serve.ts)
//   half     Shift + Q on a door, the squad behind him: two men are sent to stack on it and two are still with him
//   rest     Shift + Q on another door: the other two are sent to stack there
//   once     Q on the first door: all four are sent to stack on it, at once
//   twice    Q again inside the window: all four fall in, and nobody is on a door
//   off      the same page with ?goalhalf=0&qtwice=0: Shift + Q on a door sends all four, and Q tapped twice leaves them on the door
//   exit 0 = all of it with no page error · 1 = otherwise. The verdict is the order book's own state in the page: nothing here judges a picture, and
//   the software adapter is not a GPU. Every frame is stepped and none is drawn (a drawn frame is half a minute here).
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
      window.__mouse(x, yy); window.__runFrames(2, 1 / 60, [], 'none'); const t = O.targetUnder(window.__input, window.__cam, c); if (t.kind === 'door' && t.door === d) return d.def.name; } }
  return null; };
// in the page: who is on a plain stack by door, the wheel and whose it is, the last order and the door tools' entries
const READ = () => { const g = window.__game, O = window.__orders, by = {}; for (const m of O.men('all')) { const A = O.of(m); if (A && A.order === 'stack' && A.door) (by[A.door.def.name] ??= []).push(m.callsign); }
  const row = O.log[O.log.length - 1]; return { ring: !!O.cmdRing, part: O.menu && O.menu.part ? O.menu.part.map(m => m.callsign) : null, men: O.men('all').length, by, n: O.log.length, last: row ? `${row.order} to ${(row.who ?? []).length}` : 'none', ops: g.doorTools.ops.map(o => `${o.kind} ${o.door.def.name}`) }; };

// … and who is falling in, and who is still with him (no order, or in the file behind him)
const WHO = () => { const O = window.__orders, men = O.men('all'); return { fall: men.filter(m => O.of(m)?.order === 'fallIn').length, his: men.filter(m => { const A = O.of(m); return !A || A.post === null; }).length }; };

(async () => {
  const browser = await chromium.launch({ headless: true, channel: 'chromium', args: FLAGS });
  const page = await (await browser.newContext({ viewport: { width: 1280, height: 720 }, deviceScaleFactor: 1 })).newPage();
  const errs = []; page.on('pageerror', e => errs.push('pageerror ' + String(e).slice(0, 300))); page.on('console', m => { if ((m.type() === 'error' && !/Failed to load resource/.test(m.text())) || /Invalid|error:/.test(m.text())) errs.push('console ' + m.text().slice(0, 300)); });
  const run = n => page.evaluate(n => { window.__runFrames(n, 1 / 60, [], 'none'); }, n);
  page.setDefaultNavigationTimeout(900000);   // (a boot on the software adapter is minutes)
  const boot = async (url) => { await page.goto(url); await page.evaluate(async () => { await window.__ready; });
    await page.evaluate(() => { window.__pauseLoop = true; window.__testVisible = true; window.__game.godMode = true; window.__runFrames(90, 1 / 60, [], 'none'); for (const m of window.__orders.squad()) m.god = true; }); };
  /** Shift held, the squad's key tapped, where the cursor is */
  const shiftQ = async () => { await page.keyboard.down('ShiftLeft'); await run(2); await page.keyboard.down('KeyQ'); await run(2); await page.keyboard.up('KeyQ'); await run(4); await page.keyboard.up('ShiftLeft'); await run(6); };

  const tapQ = async () => { await page.keyboard.down('KeyQ'); await run(2); await page.keyboard.up('KeyQ'); await run(2); };

  await boot(URL); log('ready');
  const d1 = await page.evaluate(ON_DOOR, []); await shiftQ();
  const a = await page.evaluate(READ), aw = await page.evaluate(WHO);
  check(!!d1 && a.men === 4 && (a.by[d1] ?? []).length === 2 && aw.his === 2, 'half — Shift + Q on a door: two men are sent to stack on it and two are still with him',
    `the door ${d1} · sent to stack: ${JSON.stringify(a.by)} · still with him: ${aw.his} · the last order: ${a.last}`);

  const d2 = await page.evaluate(ON_DOOR, [d1]); await shiftQ();
  const b = await page.evaluate(READ);
  check(!!d2 && (b.by[d1] ?? []).length === 2 && (b.by[d2] ?? []).length === 2 && (b.by[d1] ?? []).join() === (a.by[d1] ?? []).join(), 'rest — Shift + Q on another door: the other two are sent to stack there',
    `the door ${d2} · on a stack: ${JSON.stringify(b.by)} · the last order: ${b.last}`);

  const back = await page.evaluate(ON_DOOR, [d2]); await tapQ();
  const c = await page.evaluate(READ);
  check(back === d1 && (c.by[d1] ?? []).length === 4, 'once — Q on the first door: all four are sent to stack on it, at once', `the cursor back on ${back} · on a stack: ${JSON.stringify(c.by)}`);
  await tapQ(); await run(4);
  const d = await page.evaluate(READ), dw = await page.evaluate(WHO);
  check(dw.fall === 4 && !Object.keys(d.by).length && d.last === 'fallIn to 4', 'twice — Q again inside the window: all four fall in, and nobody is on a door', `falling in ${dw.fall} of 4 · on a stack: ${JSON.stringify(d.by)} · the last order: ${d.last}`);

  await boot(URL + '&goalhalf=0&qtwice=0'); log('ready, ?goalhalf=0&qtwice=0');
  const d3 = await page.evaluate(ON_DOOR, []); await shiftQ();
  const e = await page.evaluate(READ);
  await page.evaluate(ON_DOOR, []); await tapQ(); await tapQ(); await run(4);
  const f = await page.evaluate(READ), fw = await page.evaluate(WHO);
  check(!!d3 && (e.by[d3] ?? []).length === 4 && fw.fall === 0 && (f.by[d3] ?? []).length === 4, 'off — the same page with ?goalhalf=0&qtwice=0: Shift + Q on a door sends all four, and Q tapped twice leaves them on the door',
    `Shift + Q on ${d3}: ${JSON.stringify(e.by)} · after Q twice: on a stack ${JSON.stringify(f.by)}, falling in ${fw.fall}`);

  check(!errs.length, 'no page error', errs.length ? errs.join(' | ') : 'none');
  await browser.close();
  console.log(red ? `=== halftwice-page: ${red} FAIL ===` : '=== halftwice-page: ALL PASS ===');
  process.exit(red ? 1 : 0);
})().catch(e => { console.log('- FAIL — the drive stopped: ' + String(e).slice(0, 600)); process.exit(1); });
