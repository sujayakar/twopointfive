// THE TUTORIAL'S KIT MISSION ON THE REAL PAGE, WITH REAL KEYS: headless Chromium on the software WebGPU adapter (tools/qa/tut-page.cjs's recipe — SwiftShader,
// no GPU; the page is ?paused and stepped by __runFrames with nothing drawn, every input a DOM event from Playwright's keyboard and mouse).
// What the headless lesson run (tools/qa/lesson-run.ts tut-kit) cannot see: that the bundle boots the level and its script on the frame (game/rails.ts),
// that each line is up with the page's own key on a key cap (E for his hands, Q for the squad, "Hold Q" for the ring), that the four stay at their truck
// while a hand walks in, that his own ring comes up under a real key kept down and Tab puts it away, that the squad's ring comes up under theirs and the
// digit the line names gives the order, that every card comes up once and closes, and that a hand at the keyboard and the mouse gets through all 32 steps.
// The hand reads where things are from the page (his place, a man's chest, a way round a wall) and acts only with keys, the mouse and its buttons.
// IT PLAYS WITH THE SANDBOX'S GOD MODE ON (GOD=0: off): this hand reacts in tenths of a second and its job is the keys, not the fights. That a wrong
// way puts him down is tools/qa/tut-kit-check.ts's, headless.
//   PORT=5231 node tools/qa/tut-kit-page.cjs        (a dev server on that port: PORT=5231 bun run tools/serve.ts)
//   env: QUALITY (default verylow) · OUT (a folder for the pictures; default a temp folder) · SHOTS=0 (no pictures: each is about a minute here) ·
//        STEPS (stop after this many steps) · CDP=<port> (a browser already up on the mission's page: a boot is minutes on the software adapter)
// exit 0 = every line passes · 1 = a FAIL or a page error. Software adapter: nothing here was seen on a GPU.
const { chromium } = require('/opt/node-tools/node_modules/playwright');
const fs = require('fs'), os = require('os'), path = require('path'), { execSync } = require('child_process');
const PORT = process.env.PORT || 5199, QUALITY = process.env.QUALITY || 'verylow', ROOT = path.resolve(__dirname, '../..');
const OUT = process.env.OUT || fs.mkdtempSync(path.join(os.tmpdir(), 'tut-kit-page-')), STEPS = +(process.env.STEPS || 99), GOD = process.env.GOD !== '0';
const FLAGS = ['--enable-unsafe-webgpu', '--use-gl=angle', '--use-angle=swiftshader', '--enable-features=Vulkan', '--use-vulkan=swiftshader', '--ignore-gpu-blocklist', '--disable-gpu-watchdog'];
const NOISE = /""?smoke[A-Za-z0-9_]*""?|\[pipeline\] smoke_|number of storage textures \(5\) in the Compute stage/;
// (the level's own marks, from its generator: the same numbers the scripted player walks by)
const K = JSON.parse(execSync(`bun -e "import * as K from './tools/levels/tut-kit.ts'; console.log(JSON.stringify(Object.fromEntries(Object.entries(K).filter(([, v]) => typeof v !== 'function'))))"`, { cwd: ROOT, encoding: 'utf8' }));
const T0 = Date.now(), log = (...a) => console.log(`[${((Date.now() - T0) / 1000).toFixed(0)}s]`, ...a);
let failed = false;
const check = (ok, label, detail) => { if (!ok) failed = true; console.log(`- ${ok ? 'PASS' : 'FAIL'} — ${label}\n    ${detail}`); };

(async () => {
  const data = JSON.parse(fs.readFileSync(path.join(ROOT, 'levels/tut-kit.json'), 'utf8')), steps = data.scenarios.lesson.steps;
  const held = process.env.CDP;
  const browser = held ? await chromium.connectOverCDP(`http://127.0.0.1:${held}`) : await chromium.launch({ headless: true, channel: 'chromium', args: FLAGS });
  const page = held ? browser.contexts()[0].pages()[0] : await (await browser.newContext({ viewport: { width: 1280, height: 720 }, deviceScaleFactor: 1 })).newPage(); const errors = [];
  page.setDefaultTimeout(0);
  page.on('console', m => { if (m.type() === 'error' && !NOISE.test(m.text()) && !/Failed to load resource/.test(m.text())) errors.push(m.text().slice(0, 300)); });
  page.on('pageerror', x => errors.push('pageerror: ' + String(x && x.stack || x).slice(0, 400)));
  const run = n => page.evaluate(n => window.__runFrames(n, 1 / 60, [], 'none'), n);   // (stepped, never drawn: a drawn frame is half a minute here)
  const read = () => page.evaluate(() => { const g = window.__game, P = g.player, el = document.getElementById('railprompt'), up = x => x.state !== 'dead', M = g.orders.menu, l = g.orders.log;
    const sq = g.guards.filter(x => x.unit && x.team === 'operative' && up(x)), foes = g.guards.filter(x => x.team !== 'operative');
    const room = id => { const r = g.roomClear.rooms.find(x => x.def.id === id); if (!r) return false; const b = r.def.bounds; return !!r.cleared || sq.filter(m => m.char.pos[0] > b.x0 && m.char.pos[0] < b.x1 && m.char.pos[2] > b.z0 && m.char.pos[2] < b.z1).length >= 3; };
    const lamp = g.targets.find(t => t.name === 'lamp_shop'), w = P.slot === 'rifle' ? P.rifle : P.pistol;
    return { i: g.lesson.i, id: g.lesson.step?.id ?? null, done: !!g.lesson.finished, line: (el?.querySelector('.line')?.textContent ?? '').replace(/\s+/g, ' ').trim(), keys: [...(el?.querySelectorAll('.line kbd') ?? [])].map(k => k.textContent), note: el?.querySelector('.note')?.textContent ?? '',
      up: !!el && getComputedStyle(el).display !== 'none', fatal: document.getElementById('fatal')?.textContent || '', rails: g.rails.live, sum: g.rails.summary(), script: !!g.director.running, t: +g.time.toFixed(1),
      at: [+P.char.pos[0].toFixed(2), +P.char.pos[2].toFixed(2)], crouch: !!P.crouch, down: !!P.down, slot: P.slot ?? null, equipT: P.equipT, dry: w ? w.roundsReady === 0 && !w.reloading : false, reloading: !!w?.reloading, charging: !!P.charging || !!P.kick,
      kit: `${P.canisters}/${P.flashbangs}/${P.frags}/${P.rifle?.shells} ${P.rifle?.attachment}`, ownRing: !!P.sel?.open && !!P.sel?.ring, nv: !!P.nv, dark: !!g.blackout.active, lampLit: !!lamp && lamp.on && !lamp.broken && lamp.factor > 0.05,
      men: foes.filter(up).length, downed: foes.filter(x => !up(x)).length, gunners: foes.filter(x => x.callsign === 'Novak').length, mates: sq.length, squad: sq.map(m => ({ c: m.callsign, x: +m.char.pos[0].toFixed(1), z: +m.char.pos[2].toFixed(1), o: g.orders.of(m)?.order ?? '-', low: !!g.orders.of(m)?.crouch, set: !!g.orders.of(m)?.arrived })),
      orders: l.length, last: l.length ? l[l.length - 1].order : null, menu: !!M, flat: !!M?.flat, ring: !!g.orders.cmdRing, kind: M?.target?.kind ?? null,
      doorSet: g.doorTools.set.length, wallSet: g.wallCharge?.set.length ?? 0, wallClear: (g.wallCharge?.set.length ?? 0) > 0 && !g.wallCharge.tooClose(g.wallCharge.set[0]).length,
      doors: Object.fromEntries(['inner', 'shop', 'store'].map(n => [n, +(g.doors.byName(n)?.angle ?? 0).toFixed(2)])), bunk: room('bunk'), fuel: room('fuel'), state: window.__roomclear?.state ?? null }; });
  // (a picture, as tut-page.cjs takes one: the frame is drawn and read back in the page and stood under the HUD as a still)
  const shot = async tag => { if (process.env.SHOTS === '0') return '(no picture asked)'; const f = path.join(OUT, `tut-kit-${tag}.png`);
    await page.evaluate(async () => { const dev = window.__engine.device, y = () => new Promise(res => { const ch = new MessageChannel(); ch.port1.onmessage = () => res(); ch.port2.postMessage(0); });
      for (let i = 0; i < 2; i++) { window.__runFrames(1, 1 / 600); await dev.queue.onSubmittedWorkDone(); await y(); }
      window.__engine.render(window.__cam, window.__game.time, 1 / 60, false); const fr = await window.__engine.readbackFrame(); if (!fr) return;
      if (fr.bgra) for (let i = 0; i < fr.data.length; i += 4) { const b = fr.data[i]; fr.data[i] = fr.data[i + 2]; fr.data[i + 2] = b; }
      for (let i = 3; i < fr.data.length; i += 4) fr.data[i] = 255;
      const c2 = document.createElement('canvas'); c2.width = fr.w; c2.height = fr.h; c2.getContext('2d').putImageData(new ImageData(new Uint8ClampedArray(fr.data.buffer), fr.w, fr.h), 0, 0);
      const v = document.getElementById('view'), r = v.getBoundingClientRect(); let im = document.getElementById('__still'); if (!im) { im = document.createElement('img'); im.id = '__still'; v.parentNode.insertBefore(im, v); }
      im.style.cssText = `position:fixed;left:${r.left}px;top:${r.top}px;width:${r.width}px;height:${r.height}px;z-index:0;`; im.src = c2.toDataURL('image/png'); await im.decode(); v.style.visibility = 'hidden'; });
    try { await page.screenshot({ path: f, timeout: 60000 }); } catch (e) { return `(no picture: ${String(e).slice(0, 60)})`; }
    finally { await page.evaluate(() => { document.getElementById('view').style.visibility = ''; document.getElementById('__still')?.remove(); }); }
    return f; };
  // ---- the hand
  const proj = p => page.evaluate(p => { const c = document.getElementById('view'), r = c.getBoundingClientRect(), [x, y] = window.__cam.project(p, r.width, r.height, 1); return { x: r.left + x, y: r.top + y, vis: x > 12 && y > 12 && x < r.width - 12 && y < r.height - 12 }; }, p);
  /** the cursor on a place in the world (false: it is off the picture) */
  const point = async w => { const s = await proj(w); await page.mouse.move(Math.max(4, Math.min(1276, s.x)), Math.max(4, Math.min(716, s.y))); await run(2); return s.vis; };
  const tap = async (k, n = 3) => { await page.keyboard.down(k); await run(n); await page.keyboard.up(k); await run(3); };
  const keep = async (k, n = 30) => { await page.keyboard.down(k); await run(n); await page.keyboard.up(k); await run(3); };
  const click = async w => { await point(w); await run(2); await point(w); await page.mouse.down(); await run(2); await page.mouse.up(); await run(3); };
  // A CARD (a step's `explain`, ui/railcard.ts): read and closed with Enter, as a hand does it (a key closes one only once it has been up a moment)
  const cards = [];
  const card = async () => { const c = await page.evaluate(() => { const e = document.getElementById('railcard'); return e && e.classList.contains('on') ? { h: e.querySelector('h2')?.textContent || '', p: [...e.querySelectorAll('p')].map(x => x.textContent).join(' / ') } : null; });
    if (!c) return null; await page.waitForTimeout(450); await page.keyboard.press('Enter'); await run(2);
    const still = await page.evaluate(() => !!document.getElementById('railcard')?.classList.contains('on')); cards.push({ ...c, closed: !still }); return c; };
  /** the page stepped until `want` (at most `secs` of the game's time), a card closed as it comes */
  const until = async (want, secs, n = 30) => { for (let k = 0; k < secs * 60 / n; k++) { await card(); const s = await read(); if (want(s)) return s; await run(n); } await card(); return read(); };
  // (walked with the walk keys held, steered by where the mark is ON THE PICTURE from him: D is right, S is down, whatever way the view is turned. A stride
  //  that got him nowhere — a teammate in a doorway, a jamb — is answered as a hand would: a short step to one side, the other side next time)
  const stride = async (keys, n) => { for (const k of keys) await page.keyboard.down(k); await run(n); for (const k of keys) await page.keyboard.up(k); await run(1); };
  const walk = async (to, secs, near = 0.6, stop) => { let was = Infinity, side = 1;
    for (let k = 0; k < secs * 5; k++) { const s = await read(); if (stop && stop(s)) return true; const d = Math.hypot(to[0] - s.at[0], to[2] - s.at[1]); if (d < near) return true;
      const a = await proj([s.at[0], 0, s.at[1]]), b = await proj([to[0], 0, to[2]]), l = Math.hypot(b.x - a.x, b.y - a.y) || 1, px = (b.x - a.x) / l, py = (b.y - a.y) / l, keys = [];
      if (was - d < 0.05) { await stride(Math.abs(px) > Math.abs(py) ? [side > 0 ? 'KeyS' : 'KeyW'] : [side > 0 ? 'KeyD' : 'KeyA'], 14); side = -side; was = Infinity; continue; }
      was = d;
      if (Math.abs(px) > 0.35) keys.push(px > 0 ? 'KeyD' : 'KeyA'); if (Math.abs(py) > 0.35) keys.push(py > 0 ? 'KeyS' : 'KeyW'); await stride(keys, Math.max(3, Math.min(30, Math.round(d * 10)))); }
    return false; };
  /** to a place by the floor's own way round the walls (the page's walk grid, read as a person reads the floor), each leg walked with the keys */
  const go = async (to, secs = 30, near = 0.6, stop) => { const way = await page.evaluate(to => { const g = window.__game, p = g.player.char.pos; return g.col.nav.findPath([p[0], 0, p[2]], to, 8000) ?? [to]; }, [to[0], 0, to[2]]);
    for (let k = 0; k < way.length - 1; k++) { if (!(await walk(way[k], secs, 0.45, stop))) break; if (stop && stop(await read())) return true; }
    return walk(to, secs, near, stop); };
  /** a slot's key, and the moment the thing takes to come into the hand */
  const take = async code => { await tap(code, 5); await until(s => s.equipT <= 0, 3, 6); await run(3); };
  /** a can thrown: its slot's key, the cursor on the place, a click — and, as a hand does when nothing left his hand, the click again (`n`: which count of
   *  smoke / stun / frags it is) */
  const toss = async (code, w, n) => { const left = async () => +(await read()).kit.split(' ')[0].split('/')[n], had = await left(); await take(code);
    for (let k = 0; k < 3 && (await left()) === had; k++) { await run(20); await click(w); await run(10); } return (await left()) < had; };
  /** the can in the air until it has gone off */
  const landed = async (kind, secs) => { let seen = false; for (let k = 0; k < secs * 10; k++) { const it = await page.evaluate(kind => window.__game.items.items.some(x => x.kind === kind && !x.fired), kind); if (seen && !it) break; seen = seen || it; await run(6); } await run(20); };
  /** the nearest man still up that he has a line to (of `names`, or anybody): his chest, and whether it is on the picture */
  const mark = names => page.evaluate(names => { const g = window.__game, P = g.player.char, a = P.bones?.head ?? [P.pos[0], 1.5, P.pos[2]], chest = x => x.char.bones?.chest ?? [x.char.pos[0], 1.25, x.char.pos[2]];
    const all = g.guards.filter(x => x.team !== 'operative' && x.state !== 'dead' && (!names || names.includes(x.callsign))), seen = all.filter(x => !g.col.shotBlocked(a, chest(x))).sort((p, q) => Math.hypot(p.char.pos[0] - P.pos[0], p.char.pos[2] - P.pos[2]) - Math.hypot(q.char.pos[0] - P.pos[0], q.char.pos[2] - P.pos[2]));
    const far = all.find(x => !x.script) ?? all[0]; return { n: all.length, c: seen.length ? [...chest(seen[0])] : null, who: seen[0]?.callsign ?? null, next: far ? [far.char.pos[0], 0, far.char.pos[2]] : null }; }, names || null);
  /** shoot whoever he has a line to until `want`, going after the rest if `chase` (the left button: held for the rifle, a press a round for the pistol) */
  const fight = async (want, secs, names, chase) => { const t0 = (await read()).t;
    for (;;) { await card(); const s = await read(); if (want(s) || s.t - t0 > secs) return want(s);
      if (s.dry) { await tap('KeyR'); await until(x => !x.reloading, 5, 10); continue; }
      const m = await mark(names); if (!m.n) { await run(20); continue; }
      if (!m.c) { if (chase && m.next) await go(m.next, 2, 1.5, () => false); else await run(10); continue; }
      await point(m.c); await run(10); const again = await mark(names); if (again.c) await point(again.c);
      await page.mouse.down(); await run(s.slot === 'pistol' ? 2 : 18); await page.mouse.up(); await run(6); } };
  /** the squad's key kept down on a place until their ring is up, then the digits the line now names */
  const ring = async w => { await point(w); await run(4); await point(w); await keep('KeyQ', 30); const a = await read(); if (!a.menu) return { ...a, digits: [] };
    const digits = a.keys.filter(k => /^[1-5]$/.test(k)); for (const d of digits) await tap('Digit' + d, 4); return { ...a, digits, after: await read() }; };

  const url = `http://localhost:${PORT}/?levelFile=tut-kit.json&roomclear=1&squad4&paused&quality=${QUALITY}&mute`;
  log(url);
  if (!held) { await page.goto(url, { waitUntil: 'commit' }); await page.waitForFunction(() => window.__ready !== undefined, null, { timeout: 0 }); }
  const row = await page.evaluate(async ([held, god]) => { await window.__ready; window.__testVisible = true; if (held) window.__game.restartEncounter(); window.__game.godMode = god; window.__runFrames(3, 1 / 60, [], 'none');
    const e = window.__scenarios.current()?.entry ?? window.__scenarios.current(); return e ? `${e.id} · ${e.group} ▸ ${e.title}` : null; }, [!!held, GOD]);
  await run(40); let R = await read(); log('booted', JSON.stringify(R));   // (the gun's man is stood up half a second in)
  check(/^tut-kit · tutorial/.test(row || ''), "the page knows this URL as the Tutorial group's row", String(row));
  check(!R.fatal && R.rails && R.up && R.i === 0 && R.id === 'gate' && R.mates === 4 && R.men === 7 && R.script, 'it boots on rails: one big line up, the four, the level\'s six defenders and the gunner, its script running',
    `fatal "${R.fatal}" · ${R.sum} · line "${R.line}" keys [${R.keys}] · ${R.mates} teammates · ${R.men} defenders · script ${R.script}`);
  check(R.kit === '1/1/1/1 he40', 'he carries one smoke, one stun grenade, one frag and one grenade on the launcher', `smoke/stun/frags/40 mm ${R.kit}`);
  log('picture', await shot('0-boot'));

  // ---- the hands, a step each
  const home = s => s.squad.every(m => m.x < 9.2), sq = s => s.squad.map(m => `${m.c} ${m.o}${m.low ? ' low' : ''} @${m.x},${m.z}`).join(' · ');
  const AX = K.AXIS, hand = {
    gate: async () => {
      // (the squad's key before its line: a tap and the key kept down give nothing, and the four are where the mission stood them, on a knee)
      const b = await read(); await point([b.at[0] + 2, 0.02, b.at[1] - 2]); await tap('KeyQ'); await keep('KeyQ', 30); const a = await read();
      check(a.orders === b.orders && !a.menu && !a.ring, "before its line the squad's key gives nothing, tapped or kept down", `orders ${b.orders} → ${a.orders} · ring up ${a.menu || a.ring} · ${a.sum}`);
      await go(K.INNER_FROM, 30); const c = await read();
      check(home(c) && c.squad.every(m => m.o === 'hold' && m.low), 'he is in by the gate and the four are still at their truck, each holding on a knee', sq(c)); },
    open: async () => { const b = await read(); check(b.keys.join(',') === 'E' && !/\[/.test(b.line), "the line's key is drawn as a key cap with the key the page has his hands on (E)", `"${b.line}" keys [${b.keys}]`);
      await point([K.INNER_X, 1.0, K.INNER_Z]); await run(20); await tap('KeyE'); await until(s => Math.abs(s.doors.inner) > 0.3, 5); },
    arch: async () => { await go([K.INNER_X + 1.0, 0, K.INNER_Z], 20); await go(K.JAMB, 20); },
    low: async () => { await run(70); await tap('KeyC'); const a = await read(); check(a.crouch, 'C puts him in a crouch', `crouched ${a.crouch}`); await go([16.9, 0, 13.3], 12); await go(K.BAGS_END, 15); },
    smoke: async () => { await toss('Digit3', K.GAP, 0); await landed('smoke', 4); await run(170); },
    cross: async () => { await go(K.BARRIER, 10, 0.5); },
    frag: async () => { if ((await read()).crouch) await tap('KeyC'); await toss('Digit5', K.NEST, 2); await landed('frag', 4.5); let a = await until(s => s.downed >= 1, 4);
      if (a.downed < 1) { await take('Digit2'); await go([24.9, 0, 11.6], 10); await fight(s => s.downed >= 1, 20, ['Novak']); } },
    pistol: async () => { await take('Digit1'); },
    lamp: async () => { for (let k = 0; k < 10 && (await read()).lampLit; k++) { await point(K.LAMP); await run(20); await click(K.LAMP); await run(12); if ((await read()).dry) { await tap('KeyR'); await until(s => !s.reloading, 5, 10); } }
      const a = await read(); check(!a.lampLit, 'the lamp by the shop is shot out with the pistol', `lit ${a.lampLit} · in his hands ${a.slot}`); },
    ring: async () => { const c = [K.SHOP_DOOR[0], 1.0, K.SHOP_DOOR[2]]; await go([24.2, 0, AX + 0.2], 10); await point(c); await run(30); await card(); await point(c);
      const b = await read(); await keep('KeyE', 30); const a = await read();
      check(a.ownRing && a.doorSet === 0 && /^Hold E/.test(b.line), 'E kept down on the locked door: his own ring is up and stays with the key let go, and nothing is done yet', `line "${b.line}" keys [${b.keys}] · ring up ${a.ownRing} · charges set ${a.doorSet}`);
      log('picture', await shot('10-ring')); },
    tab: async () => { await tap('Tab'); const a = await read(); check(!a.ownRing && a.doorSet === 0, 'Tab puts the ring away with nothing done', `ring up ${a.ownRing} · charges set ${a.doorSet}`); },
    charge: async () => { const c = [K.SHOP_DOOR[0], 1.0, K.SHOP_DOOR[2]]; await point(c); await run(10); await keep('KeyE', 30); const u = await read(); await tap('Digit1'); const a = await until(s => s.doorSet > 0 && !s.charging, 12);
      check(u.ownRing && a.doorSet === 1, "E kept down again and the ring's first wedge: his charge is on the door", `ring up ${u.ownRing} · charges set ${a.doorSet}`); await run(20); },
    back: async () => { await go(K.BACK, 10, 0.5); await run(20); },
    // (the cursor is left wherever the walk left it: the tap is his own charge's wherever he points)
    blow: async () => { const b = await read(); await run(20); await tap('KeyE', 6); const a = await until(s => s.id !== 'blow', 6);
      check(a.id !== 'blow' && Math.abs(a.doors.shop) > 0.3 && /^Press E/.test(b.line), 'stood clear, a tap of E blows his own charge and the door is open', `line "${b.line}" · the door's leaf ${a.doors.shop} · now '${a.id}'`); },
    shop: async () => { await go([23.5, 0, AX + 0.05], 6); await fight(s => s.id !== 'shop', 25, ['Tango 1']); },
    stun: async () => { await go([26.8, 0, AX], 10); await toss('Digit4', K.STUN_AT, 1);
      for (let k = 0; k < 12 && !(await page.evaluate(() => window.__game.items.items.some(x => x.kind === 'flash' && !x.fired))); k++) await run(10);
      await go([26.8, 0, AX + 1.5], 4); await landed('flash', 3.4); },
    store: async () => { await take('Digit1'); await go([K.STORE_DOORWAY[0], 0, AX], 6); await fight(s => s.id !== 'store', 30, null, true); },
    rifle: async () => { await take('Digit2'); },
    leave: async () => { await go(K.LAUNCH_FROM, 40, 0.15, s => s.id !== 'leave'); },   // (he stops as the line changes: the ring's near edge)
    behind: async () => { await until(s => s.id !== 'behind', 12); },
    launch: async () => {
      for (let k = 0; k < 24; k++) { const near = await page.evaluate(L => ['Brandt', 'Marsh'].map(n => window.__game.guards.find(x => x.callsign === n)).filter(Boolean).every((x, i) => x.state === 'dead' || Math.hypot(x.char.pos[0] - L[i][0], x.char.pos[2] - L[i][2]) < 0.8), K.LAST_MEN); if (near) break; await run(30); }
      await run(30); const at = K.LAST_COVER, before = await read();   // (the step's own mark, where its ring is drawn: the doorway they stand in) // (the cursor on the doorway first, as the line before had him look there: a press while the gun is still swinging past the arch's wall begins nothing,
      //  and a hand whose press gave no arc presses again)
      for (let n = 0; n < 3 && /\/1 he40$/.test((await read()).kit); n++) { await point(at); await run(40); await point(at); await page.mouse.down({ button: 'right' }); for (let k = 0; k < 6; k++) { await point(at); await run(5); }
        await page.mouse.up({ button: 'right' });
        await run(60); }
      await run(60);
      const a = await read(); check(/\/0 he40$/.test(a.kit), "the right button held and let go fires the launcher's one grenade", `smoke/stun/frags/40 mm ${a.kit} · ${a.downed} down`);
      const said = await page.evaluate(() => window.__game.messages.map(m => m.text).filter(t => /too close to arm|was a dud/.test(t)));
      check(!said.length && a.downed > before.downed, 'from where the line before stood him the grenade is armed when it gets to the doorway, and takes one of the two at least', `he is at ${a.at} · ${a.downed - before.downed} went down${said.length ? ` · the page said "${said[0]}"` : ''}`);
      if (a.id === 'launch') await fight(s => s.id !== 'launch', 30, ['Brandt', 'Marsh'], true); },   // (the grenade took neither: the rifle, as the next line says)
    last: async () => { await fight(s => s.id !== 'last', 60, ['Brandt', 'Marsh'], true); },
    call: async () => { const b = await read(); check(home(b), 'all through his own half the four have stayed at their truck', sq(b));
      check(b.keys.join(',') === 'Q', "the line's key is the squad's, on a key cap (Q)", `"${b.line}" keys [${b.keys}]`);
      await go([K.ARCH_MOUTH[0] + 1.6, 0, AX], 10); const me = (await read()).at; await point([me[0], 0.1, me[1]]); await tap('KeyQ');
      const a = await until(s => s.id !== 'call', 70); check(a.id !== 'call' && a.squad.every(m => m.o !== 'hold' || m.x > 9.2), 'a tap of Q on himself: the four come up from the truck', sq(a)); },
    breaker: async () => { for (let k = 0; k < 3 && !(await read()).dark; k++) { if (!(await point(K.BREAKER))) await go([K.BREAKER[0], 0, K.BREAKER[2] + 5], 20); await point(K.BREAKER); await tap('KeyQ'); await until(s => s.dark, 30); }
      const a = await read(); check(a.dark, 'a tap of Q on the breaker box: a teammate works it and the lights are out', `lights out ${a.dark} · ${sq(a)}`); },
    nv: async () => { await tap('KeyN', 6); await run(10); const a = await read(); check(a.nv || a.id !== 'nv', 'N turns on night vision', `night vision ${a.nv}`); },
    door: async () => { await go([27.2, 0, AX], 30); await card(); const b = await read();
      check(/^Hold Q/.test(b.line) && b.keys[0] === 'Q', 'the line reads Hold Q, the key on a key cap', `"${b.line}" keys [${b.keys}]`);
      await point(K.BUNK_DOOR); await run(4); await tap('KeyQ'); await run(6); const t = await read();
      check(t.orders === b.orders && !!t.note, 'a tap where the line says hold gives no order, and words come up under the line', `orders ${b.orders} → ${t.orders} · under the line "${t.note}"`);
      for (let k = 0; k < 3 && !(await read()).bunk; k++) { const r = await ring(K.BUNK_DOOR);
        if (k === 0) { check(r.menu && r.flat && r.kind === 'door' && r.digits.join(',') === '4', "Q kept down on the bunk room door: the squad's ring is up, one ring, and the line names its digit", `ring ${r.menu}, flat ${r.flat}, on a ${r.kind} · line "${r.line}" keys [${r.keys}]`);
          check(r.after?.last === 'bangClear' && !r.after.menu, 'that digit gives bang and clear and the ring is away', `the last order '${r.after?.last}' · ring up ${r.after?.menu}`); log('picture', await shot('26-door')); }
        let a = await until(s => s.bunk || s.squad.every(m => m.set), 25); await run(60); if (!a.bunk) await tap('KeyF'); await until(s => s.bunk, 30); await run(120); } },
    wall: async () => { await go([28.0, 0, 8.6], 30); await card();
      for (let k = 0; k < 3; k++) { const s = await read(); if (s.wallSet || s.fuel) break; const r = await ring(K.WALL_MARK);
        if (k === 0) check(r.menu && r.digits.join(',') === '5,3' && r.after?.last === 'wallCharge', 'Q kept down on the wall, more, then the wall charge: the order is given', `ring ${r.menu} on a ${r.kind} · line "${r.line}" keys [${r.keys}] · the last order '${r.after?.last}'`);
        await until(x => x.wallSet > 0, 20); }
      await go([27.4, 0, 12.4], 20); await until(s => s.wallClear, 20); await run(40); },
    breach: async () => { await run(60); await tap('KeyF'); await until(s => s.fuel, 40); await run(120); },
    // (the cursor on his feet: the ray through his chest lands a metre behind him, and with a door there the game reads the door)
    lead: async () => { const me = (await read()).at; await point([me[0], 0.1, me[1]]); await tap('KeyQ'); await go([K.YARD_FROM[0], 0, K.YARD_FROM[2]], 40); await until(s => s.id !== 'lead', 30); },
    lights: async () => { await until(s => s.id !== 'lights', 16); },
    throw: async () => { for (let k = 0; k < 2 && (await read()).id === 'throw'; k++) { const r = await ring(K.FRAG_AT);
        if (k === 0) check(r.menu && r.kind === 'floor' && r.digits.join(',') === '5,1' && r.after?.last === 'fragThere', 'Q kept down on the ground behind the sandbags, more, then the frag: the order is given', `ring ${r.menu} on a ${r.kind} · line "${r.line}" keys [${r.keys}] · the last order '${r.after?.last}'`);
        await landed('frag', 8); await until(s => s.id !== 'throw', 6); } },
    clear: async () => { await fight(s => s.done || s.id !== 'clear', 60, null, true); },
  };
  let n = 0;
  for (; n < steps.length && n < STEPS; n++) {
    await card(); R = await read(); if (R.done) break;
    const s = steps[R.i]; if (!s) break;
    if (R.i > n) { log(`  (step${R.i - n > 1 ? 's' : ''} ${steps.slice(n, R.i).map(x => `'${x.id}'`).join(', ')} went by with the one before)`); n = R.i; }
    log(`step ${n + 1} '${s.id}': "${R.line}" keys [${R.keys}] t ${R.t} · he is at ${R.at} · ${R.men} defenders`);
    if (!hand[s.id]) { check(false, `step '${s.id}' has a hand`, 'no entry'); break; }
    await hand[s.id](s);
    R = await until(x => x.done || x.id !== s.id, 8);
    check(R.done || R.id !== s.id, `step ${n + 1} '${s.id}' is done by a hand's own keys`, `now ${R.done ? 'finished' : `'${R.id}'`} at t ${R.t} · he is at ${R.at}${R.down ? ' DOWN' : ''} · ${R.men} defenders · ${R.sum}`);
    if (!R.done && R.id === s.id) { log('stuck picture', await shot(`stuck-${s.id}`)); log('stuck', JSON.stringify(R)); break; }
  }
  await run(240); await card(); R = await read();
  log('end', JSON.stringify(R)); log('picture', await shot('9-end'));
  if (STEPS >= steps.length) {
    check(R.done && R.men === 0, 'every step is done, nobody of theirs is left and the mission ends', `finished ${R.done} · ${R.men} defenders · the mission's state ${JSON.stringify(R.state)} · ${R.mates} teammates up`);
    const want = steps.filter(s => s.explain).map(s => s.explain.title);
    // (a page played before, CDP: the cards it has shown stay read, so this line is a fresh page's)
    if (!held) check(want.every(h => cards.some(c => c.h === h && c.closed && c.p.length > 40)), 'every card of the mission was up once with its words and closed with Enter', cards.map(c => `"${c.h}"${c.closed ? '' : ' (STILL UP)'}`).join(' · ') || 'none'); }
  // ---- A CHECKPOINT PUT BACK FROM HIS OWN HALF: its step is up again, the level's script runs again, the gun's man is not stood up a second time, and
  //      the four are back at their truck on a knee (nothing of the wait is kept: the steps say they wait again)
  if (STEPS >= steps.length) { const slots = await page.evaluate(() => window.__game.saves.slots().map(x => ({ key: x.key, label: x.label || '' })));
    const mine = slots.find(x => /the gun/.test(x.label)) ?? slots.find(x => /the shop|the store/.test(x.label));
    check(!!mine, 'a checkpoint of his own half was kept', `slots: ${slots.map(x => x.label).join(' · ') || 'none'}`);
    if (mine) { const why = await page.evaluate(k => window.__game.saves.load(window.__game, k), mine.key); await run(120); await card(); R = await read(); const call = steps.findIndex(x => x.id === 'call');
      check(!why && R.i < call && R.script && home(R) && R.squad.every(m => m.o === 'hold') && R.gunners === 1 && R.mates === 4, `'${mine.label}' put back: a step of his own half is up, the script runs again with one gunner and no second, and the four are at their truck holding`,
        `step ${R.i + 1} '${R.id}' · script ${R.script} · ${R.men} defenders up, ${R.downed} down, ${R.gunners} gunner · ${sq(R)} ${why || ''}`);
      await run(600); const L = await read(); check(home(L) && L.squad.every(m => m.o === 'hold' && m.low), 'ten seconds on, they are still there, each on a knee', sq(L)); } }
  check(!errors.length, 'no page error and no console error', errors.length ? errors.slice(0, 6).join('\n    ') : 'none');
  console.log(`pictures: ${OUT}`);
  if (!held) await browser.close();
  console.log(failed ? '=== tut-kit-page: FAILED ===' : `=== tut-kit-page: ALL PASS (software adapter: never seen on a GPU${GOD ? '; god mode on' : ''}) ===`);
  process.exit(failed ? 1 : 0);
})().catch(e => { console.error(e); process.exit(1); });
