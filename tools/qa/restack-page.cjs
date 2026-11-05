// THE KEY ON A STACK IS 'STACK UP' AGAIN, ON THE REAL PAGE (src/game/autopilot.ts autoPick, AUTOPILOT.restack; orders.ts again): headless Chromium on
// the software WebGPU adapter, the page as it opens on the room clear with its squad of four, the keys pressed as a hand presses them.
//   PORT=5247 node tools/qa/restack-page.cjs     (a dev server on that port: PORT=5247 bun run tools/serve.ts)
//   stacked  Q on a door: all four are sent to stack on it, and they get there (the frames stepped with no picture: the software adapter is slow)
//   again    Q on that door: one man answers with a line of RESTACK_LINES, no order is written, no door is worked, and they are still stacked
//   word     F: the door tools start an entry on that door
//   off      the same page with ?restack=0: Q on the door they are stacked on starts the entry, as it did
//   exit 0 = all of it with no page error · 1 = otherwise. The verdict is the order book's own state in the page: nothing here judges a picture or
//   hears a voice, and the software adapter is not a GPU. Every frame is stepped and none is drawn (a drawn frame is half a minute here).
const { chromium } = require('/opt/node-tools/node_modules/playwright');
const PORT = process.env.PORT || 5247, QUALITY = process.env.QUALITY || 'verylow';
const FLAGS = ['--enable-unsafe-webgpu', '--use-gl=angle', '--use-angle=swiftshader', '--enable-features=Vulkan', '--use-vulkan=swiftshader', '--ignore-gpu-blocklist', '--disable-gpu-watchdog'];
const T0 = Date.now(), log = (...a) => console.log(`[${((Date.now() - T0) / 1000).toFixed(0)}s]`, ...a);
const URL = `http://localhost:${PORT}/?roomclear&squad4&paused&quality=${QUALITY}&mute`;
let red = 0;
const check = (ok, what, got) => { if (!ok) red++; console.log(`- ${ok ? 'PASS' : 'FAIL'} — ${what}: ${got}`); };

// in the page: the cursor put on the door of that name (null: the nearest door the squad's key reads as a door); its name, or null
const ON_DOOR = (name) => { const g = window.__game, O = window.__orders, c = document.getElementById('view'), w = c.clientWidth, h = c.clientHeight, s = g.player.char.pos;
  const ds = [...g.doors.list].filter(d => !name || d.def.name === name).sort((a, b) => Math.hypot(a.frameCentre[0] - s[0], a.frameCentre[2] - s[2]) - Math.hypot(b.frameCentre[0] - s[0], b.frameCentre[2] - s[2]));
  for (const d of ds.slice(0, 6)) { const th = d.def.closedDir, nx = -Math.sin(th), nz = Math.cos(th);
    for (const y of [d.frameCentre[1], 0.05, 1.0, 1.8]) for (const k of [0, 0.35, -0.35]) { const [x, yy, front] = window.__cam.project([d.frameCentre[0] + nx * k, y, d.frameCentre[2] + nz * k], w, h); if (!front) continue;
      window.__mouse(x, yy); window.__runFrames(2, 1 / 60, [], 'none'); const t = O.targetUnder(window.__input, window.__cam, c); if (t.kind === 'door' && t.door === d) return d.def.name; } }
  return null; };
// in the page: who is stacked and arrived on that door, the orders written, the door tools' entries, what the men have said, the rule's own count
const READ = (name) => { const g = window.__game, O = window.__orders, men = O.men('all'), on = men.filter(m => { const A = O.of(m); return A && A.order === 'stack' && A.door && A.door.def.name === name; });
  return { men: men.length, on: on.length, set: on.filter(m => O.of(m).arrived).length, n: O.log.length, ops: g.doorTools.ops.map(o => `${o.kind} ${o.door.def.name}`), said: (window.__said ?? []).slice(), again: window.__autopilot.stats.again,
    restack: window.__autopilot.knobs.restack, at: men.map(m => [m.char.pos[0], m.char.pos[2]]) }; };

(async () => {
  const browser = await chromium.launch({ headless: true, channel: 'chromium', args: FLAGS });
  const page = await (await browser.newContext({ viewport: { width: 1280, height: 720 }, deviceScaleFactor: 1 })).newPage();
  const errs = []; page.on('pageerror', e => errs.push('pageerror ' + String(e).slice(0, 300))); page.on('console', m => { if ((m.type() === 'error' && !/Failed to load resource/.test(m.text())) || /Invalid|error:/.test(m.text())) errs.push('console ' + m.text().slice(0, 300)); });
  const run = n => page.evaluate(n => { window.__runFrames(n, 1 / 60, [], 'none'); }, n);
  page.setDefaultNavigationTimeout(900000);   // (a boot on the software adapter is minutes)
  const boot = async (url) => { await page.goto(url); await page.evaluate(async () => { await window.__ready; });
    await page.evaluate(() => { window.__pauseLoop = true; window.__testVisible = true; const g = window.__game; g.godMode = true; window.__runFrames(90, 1 / 60, [], 'none'); for (const m of window.__orders.squad()) m.god = true;
      window.__said = []; const say0 = g.say.bind(g); g.say = (who, text, ...r) => { window.__said.push(`${who && who.callsign} '${text}'`); return say0(who, text, ...r); }; }); };
  const tapQ = async () => { await page.keyboard.down('KeyQ'); await run(2); await page.keyboard.up('KeyQ'); await run(6); };
  /** the game stepped with no picture drawn until the four are stacked and arrived on that door, or `secs` of the game have gone */
  const walk = (name, secs) => page.evaluate(([name, secs]) => { const O = window.__orders, set = () => O.men('all').every(m => { const A = O.of(m); return A && A.order === 'stack' && A.door && A.door.def.name === name && A.arrived; });
    let s = 0; for (; s < secs && !set(); s++) window.__runFrames(60, 1 / 60, [], 'none'); if (set()) window.__runFrames(90, 1 / 60, [], 'none'); return { set: set(), secs: s }; }, [name, secs]);
  const far = (a, b) => Math.max(...a.map((p, i) => Math.hypot(p[0] - b[i][0], p[1] - b[i][1])));

  await boot(URL); log('ready');
  const d1 = await page.evaluate(ON_DOOR, null); await tapQ(); const w1 = await walk(d1, 60); const a = await page.evaluate(READ, d1);
  check(!!d1 && a.restack === true && a.men === 4 && a.on === 4 && a.set === 4 && !a.ops.length, 'stacked — Q on a door: all four are sent to stack on it and they get there',
    `the rule on this page: ${a.restack} · the door ${d1} · sent ${a.on} of ${a.men}, arrived ${a.set} after ${w1.secs} s of the game · door tools: ${a.ops.join(' + ') || 'none'}`);

  const on2 = await page.evaluate(ON_DOOR, d1); await tapQ(); await run(60); const b = await page.evaluate(READ, d1), lines = b.said.slice(a.said.length);
  check(on2 === d1 && b.n === a.n && !b.ops.length && lines.length === 1 && /stacked up\.|ready to go\./.test(lines[0]) && b.again === a.again + 1 && b.set === 4 && far(a.at, b.at) < 0.05,
    'again — Q on that door: one man answers that they are stacked, no order is written, no door is worked and nobody leaves his mark',
    `said: ${lines.join(', ') || 'nothing'} · orders written ${b.n - a.n} · door tools: ${b.ops.join(' + ') || 'none'} · still arrived ${b.set} · the furthest any man went ${far(a.at, b.at).toFixed(2)} m`);

  await page.keyboard.down('KeyF'); await run(2); await page.keyboard.up('KeyF'); await run(30); const c = await page.evaluate(READ, d1);
  check(c.ops.some(o => o.split(' ')[1] === d1), 'word — F: the door tools start an entry on that door', `entries: ${c.ops.join(' + ') || 'none'}`);

  await boot(URL + '&restack=0'); log('ready, ?restack=0');
  const d3 = await page.evaluate(ON_DOOR, null); await tapQ(); const w3 = await walk(d3, 60); const d = await page.evaluate(READ, d3);
  await page.evaluate(ON_DOOR, d3); await tapQ(); await run(30); const e = await page.evaluate(READ, d3);
  check(d.restack === false && d.set === 4 && e.ops.some(o => o.split(' ')[1] === d3) && e.again === d.again, 'off — the same page with ?restack=0: Q on the door they are stacked on starts the entry',
    `the rule on this page: ${d.restack} · the door ${d3}, arrived ${d.set} after ${w3.secs} s · entries after the second press: ${e.ops.join(' + ') || 'none'}`);

  check(!errs.length, 'no page error', errs.length ? errs.join(' | ') : 'none');
  await browser.close();
  console.log(red ? `=== restack-page: ${red} FAIL ===` : '=== restack-page: ALL PASS ===');
  process.exit(red ? 1 : 0);
})().catch(e => { console.log('- FAIL — the drive stopped: ' + String(e).slice(0, 600)); process.exit(1); });
