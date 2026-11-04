// THE TUTORIAL'S FOURTH MISSION ON THE REAL PAGE, WITH REAL KEYS: headless Chromium on the software WebGPU adapter (tools/qa/tut-page.cjs's recipe — SwiftShader,
// no GPU; the page is ?paused and stepped by __runFrames with nothing drawn, every input a DOM event from Playwright's keyboard and mouse).
// What the headless lesson run cannot see: that the bundle boots the level and its script on the frame (game/rails.ts) from the Tutorial group's row, that the
// one big line is up with the player's own keys on key caps, that Shift with the squad's key is the plain key until its step and adds a goal from it with plan
// mode never up, that one press of the word's key starts an entry on each door, that a hand walks to each ring and finds the next mark on its picture, that on
// the last gate a Shift tap gives nothing and says to keep the key down, that the pair kept down opens the ring for a pair with the step's own line on it, and
// that the mission plays to its end with every card met and closed and no page error.
//   PORT=5233 node tools/qa/tut-queue-page.cjs        (a dev server on that port: PORT=5233 bun run tools/serve.ts)
//   env: QUALITY (default verylow) · GOD=0 (the player can be hit; by default he cannot, so that a slow hand on a slow adapter still ends the run: the
//        teammates are the frame's to spare either way)
// exit 0 = every line passes · 1 = a FAIL or a page error. The verdicts are the page's own state (the lesson, the order book, the door tools, the DOM of the
// line and the cards): nothing here judges a picture, and the software adapter is not a GPU.
const { chromium } = require('/opt/node-tools/node_modules/playwright');
const PORT = process.env.PORT || 5233, QUALITY = process.env.QUALITY || 'verylow', GOD = process.env.GOD !== '0';
const FLAGS = ['--enable-unsafe-webgpu', '--use-gl=angle', '--use-angle=swiftshader', '--enable-features=Vulkan', '--use-vulkan=swiftshader', '--ignore-gpu-blocklist', '--disable-gpu-watchdog'];
const NOISE = /""?smoke[A-Za-z0-9_]*""?|\[pipeline\] smoke_|number of storage textures \(5\) in the Compute stage/;
const T0 = Date.now(), log = (...a) => console.log(`[${((Date.now() - T0) / 1000).toFixed(0)}s]`, ...a);
let failed = false;
const check = (ok, label, detail) => { if (!ok) failed = true; console.log(`- ${ok ? 'PASS' : 'FAIL'} — ${label}\n    ${detail}`); };

(async () => {
  const browser = await chromium.launch({ headless: true, channel: 'chromium', args: FLAGS });
  const page = await (await browser.newContext({ viewport: { width: 1280, height: 720 }, deviceScaleFactor: 1 })).newPage(); const errors = [];
  page.setDefaultTimeout(0);
  page.on('console', m => { const t = m.text(); if (NOISE.test(t) || /Failed to load resource/.test(t)) return; if (m.type() === 'error' || /Invalid|error:/.test(t)) errors.push(t.slice(0, 300)); });
  page.on('pageerror', x => errors.push('pageerror: ' + String(x && x.stack || x).slice(0, 400)));
  const run = n => page.evaluate(n => window.__runFrames(n, 1 / 60, [], 'none'), n);   // (stepped, never drawn: a drawn frame is half a minute here)
  const read = () => page.evaluate(() => { const g = window.__game, O = g.orders, el = document.getElementById('railprompt'), by = {}, watch = {};
    for (const m of O.men('all')) { const A = O.of(m); if (A?.order === 'stack' && A.door) (by[A.door.def.name] ??= []).push(m.callsign); if (A?.order === 'cover' && A.watch) (watch[`${A.watch[0].toFixed(1)},${A.watch[2].toFixed(1)}`] ??= []).push(m.callsign); }
    const M = O.menu;
    return { i: g.lesson.i, id: g.lesson.step?.id ?? null, at: g.lesson.step?.at ?? null, line: el?.querySelector('.line')?.textContent ?? '', keys: [...(el?.querySelectorAll('kbd') ?? [])].map(k => k.textContent), note: el?.querySelector('.note')?.textContent ?? '',
      up: !!el && getComputedStyle(el).display !== 'none', fatal: document.getElementById('fatal')?.textContent || '', rails: g.rails.live, sum: g.rails.summary(), script: !!g.director.running,
      pos: [+g.player.char.pos[0].toFixed(1), +g.player.char.pos[2].toFixed(1)], mates: g.guards.filter(x => x.team === 'operative' && x.state !== 'dead').length, men: g.guards.filter(x => x.team === 'hostile' && x.state !== 'dead').length,
      plan: !!O.plan?.up, by, watch, following: O.men('all').filter(m => { const A = O.of(m); return !A || A.post === null; }).length, ops: g.doorTools.ops.map(o => `${o.kind} ${o.door.def.name}`), orders: O.log.length,
      ring: M ? { part: !!M.part, flat: !!M.flat, men: (M.men ?? []).map(m => m.callsign), rows: M.rows.map(r => r.id) } : null,
      markOn: (() => { const p = g.lesson.step?.at; if (!p) return null; const v = document.getElementById('view').getBoundingClientRect(), sc = window.__cam.project(p, v.width, v.height, 1); return sc[0] > 20 && sc[0] < v.width - 20 && sc[1] > 20 && sc[1] < v.height - 20; })(),
      over: !!g.lesson.finished, end: g.roomClear?.end ?? null, t: +g.time.toFixed(1) }; });
  const point = async w => { const s = await page.evaluate(p => { const c = document.getElementById('view'), r = c.getBoundingClientRect(), [x, y] = window.__cam.project(p, r.width, r.height, 1); return { x: r.left + x, y: r.top + y }; }, w); await page.mouse.move(s.x, s.y); await run(2); };
  const self = async () => point(await page.evaluate(() => [...window.__game.player.char.pos]));
  const tap = async k => { await page.keyboard.down(k); await run(3); await page.keyboard.up(k); await run(3); };
  /** Shift held, the squad's key tapped, where the cursor is */
  const shiftTap = async () => { await page.keyboard.down('ShiftLeft'); await run(2); await page.keyboard.down('KeyQ'); await run(2); await page.keyboard.up('KeyQ'); await run(4); await page.keyboard.up('ShiftLeft'); await run(6); };
  // A CARD (a step's `explain`, ui/railcard.ts): read and closed with Enter, as a hand does it. Every card met is kept by its title: the last line holds the list to the level's own
  const cards = [];
  const card = async () => { const c = await page.evaluate(() => { const e = document.getElementById('railcard'); return e && e.classList.contains('on') ? { h: e.querySelector('h2')?.textContent || '', p: [...e.querySelectorAll('p')].map(x => x.textContent).join(' / ') } : null; });
    if (!c) return null; await page.waitForTimeout(450); await page.keyboard.press('Enter'); await run(2);
    const still = await page.evaluate(() => !!document.getElementById('railcard')?.classList.contains('on')); cards.push({ ...c, closed: !still }); return c; };
  const until = async (want, secs) => { for (let k = 0; k < secs; k++) { await card(); const s = await read(); if (want(s)) return s; await run(60); } await card(); return read(); };
  // (walked with the walk keys held along the level's own walk grid from where he is — a hand goes round a wall, not at it. The view may be turned, so which
  //  key is which way is MEASURED: a short stride on D and one on S give the two directions the keys move him in)
  let AX = null;
  const stride = async (keys, n) => { for (const k of keys) await page.keyboard.down(k); await run(n); for (const k of keys) await page.keyboard.up(k); await run(1); };
  const axes = async () => { const a = await read(); await stride(['KeyD'], 10); const b = await read(); await stride(['KeyS'], 10); const c = await read();
    const n = v => { const l = Math.hypot(v[0], v[1]) || 1; return [v[0] / l, v[1] / l]; }; AX = { D: n([b.pos[0] - a.pos[0], b.pos[1] - a.pos[1]]), S: n([c.pos[0] - b.pos[0], c.pos[1] - b.pos[1]]) }; };
  const leg = async (to, secs) => { let was = Infinity, side = 1;
    for (let k = 0; k < secs * 4; k++) { const s = await read(), dx = to[0] - s.pos[0], dz = to[2] - s.pos[1], d = Math.hypot(dx, dz); if (d < 0.45) return true;
      const pd = (dx * AX.D[0] + dz * AX.D[1]) / d, ps = (dx * AX.S[0] + dz * AX.S[1]) / d, keys = [];
      if (was - d < 0.08) { await stride(Math.abs(pd) > Math.abs(ps) ? [side > 0 ? 'KeyS' : 'KeyW'] : [side > 0 ? 'KeyD' : 'KeyA'], 14); side = -side; was = Infinity; continue; }
      was = d;
      if (Math.abs(pd) > 0.3) keys.push(pd > 0 ? 'KeyD' : 'KeyA'); if (Math.abs(ps) > 0.3) keys.push(ps > 0 ? 'KeyS' : 'KeyW'); await stride(keys, Math.max(4, Math.min(30, Math.round(d * 14)))); }
    return false; };
  const walk = async (to, via = []) => { if (!AX) await axes();
    for (const goal of [...via, to]) { const path = await page.evaluate(p => { const g = window.__game, w = g.col.nav.findPath([...g.player.char.pos], p, 20000); return w ? w.map(q => [q[0], 0, q[2]]) : null; }, goal);
      for (const w of [...(path ?? []), goal]) if (!(await leg(w, 12))) break; }
    const s = await read(); return Math.hypot(to[0] - s.pos[0], to[2] - s.pos[1]) < 1.6; };

  const url = `http://localhost:${PORT}/?levelFile=tut-queue.json&roomclear=1&squad4&paused&quality=${QUALITY}&mute&nocutboxes`;
  log(url);
  await page.goto(url, { waitUntil: 'commit' });
  await page.waitForFunction(() => window.__ready !== undefined, null, { timeout: 0 });   // (the bundle has run: the boot's own promise is there to wait on)
  const row = await page.evaluate(async god => { await window.__ready; window.__testVisible = true; if (god) window.__game.godMode = true; window.__runFrames(3, 1 / 60, [], 'none'); const e = window.__scenarios.current()?.entry ?? window.__scenarios.current(); return e ? `${e.id} · ${e.group} ▸ ${e.title}` : null; }, GOD);
  const L = await page.evaluate(() => { const S = window.__game.level.scenarios.lesson; return { steps: S.steps.map(T => ({ id: T.id, at: T.at ?? null, card: T.explain?.title ?? null, flat: T.flatLine ?? null })) }; });
  const at = id => L.steps.find(T => T.id === id).at;
  let R = await read(); log('booted', JSON.stringify({ id: R.id, line: R.line, keys: R.keys, mates: R.mates, men: R.men }));
  check(/^tut-queue · tutorial/.test(row || ''), "the page knows this URL as the Tutorial group's row", String(row));
  check(!R.fatal && R.rails && R.i === 0 && R.id === 'corner' && R.mates === 4 && R.men === 8 && R.script && !R.plan, 'it boots on rails: the first step up, the four, eight enemies, its script running, plan mode down',
    `fatal "${R.fatal}" · ${R.sum} · ${R.mates} teammates · ${R.men} enemies · script ${R.script} · plan mode ${R.plan ? 'UP' : 'down'}`);
  R = await until(x => x.up && !!x.line, 4);
  check(cards.length === 1 && cards[0].closed && R.up && !/\[/.test(R.line), 'the opening card comes up and closes on Enter, and the one big line is up behind it', `card "${cards[0]?.h}" closed ${cards[0]?.closed} · "${R.line}" keys [${R.keys}]`);

  // ---- 1 · two doors at once
  let ok = await walk(at('corner')); R = await until(x => x.i >= 1, 6);
  check(ok && R.id === 'door1' && R.markOn && R.following === 4, "walked to the marked corner: the next line is up, its door's mark is on the picture and the four came with him", `reached ${ok} at ${R.pos} · step '${R.id}' · "${R.line}" keys [${R.keys}] · mark on the picture ${R.markOn} · following ${R.following}`);
  // (Shift down with the key BEFORE its step: the plain key — all four, plan mode down)
  await point(at('door1')); await shiftTap(); R = await read();
  check((R.by.guard ?? []).length === 4 && !R.plan, 'before its step, Shift with the squad\'s key is the plain key: all four are sent to the guardroom door and plan mode stays down', `sent to stack: ${JSON.stringify(R.by)} · plan mode ${R.plan ? 'UP' : 'down'}`);
  R = await until(x => x.i >= 2, 40);
  check(R.id === 'door2' && cards.some(c => c.h === 'Add a second order' && c.closed) && R.keys.includes('Shift') && R.keys.includes('Q'), "they stack, the card for the second order comes and goes, and the line asks for Shift and the squad's key as key caps", `step '${R.id}' at ${R.t} s · "${R.line}" keys [${R.keys}] · cards: ${cards.map(c => c.h).join(' · ')}`);
  await point(at('door2')); await shiftTap(); R = await read();
  check((R.by.guard ?? []).length === 2 && (R.by.guard_side ?? []).length === 2 && !R.plan, 'from its step it adds a goal: two are sent to the side door, two are still on the first, plan mode down', `on a stack: ${JSON.stringify(R.by)} · plan mode ${R.plan ? 'UP' : 'down'}`);
  // (the word pressed while the second pair still walks: its key is not his yet)
  { const before = R.ops.length; await tap('KeyF'); const a = await read(); check(a.ops.length === before && a.id === 'door2', 'the word pressed before both stacks stand starts no entry', `entries ${before} → ${a.ops.length} · step '${a.id}'`); }
  R = await until(x => x.id === 'word', 60);
  check(R.id === 'word' && R.keys.length === 1 && R.keys[0] === 'F', 'both stacks stand: the line asks for the word\'s key', `step '${R.id}' at ${R.t} s · "${R.line}" keys [${R.keys}]`);
  await tap('KeyF'); R = await read(); { const doors = new Set(R.ops.map(o => o.split(' ')[1]));
    check(doors.has('guard') && doors.has('guard_side'), 'one press of the word starts an entry on each door', `entries: ${R.ops.join(' + ') || 'none'}`); }
  R = await until(x => x.i >= 4, 40);
  check(R.id === 'front' && R.men === 6 && R.mates === 4, 'the guardroom is taken from both doors and the next line is the hall\'s front door', `step '${R.id}' at ${R.t} s · ${R.men} enemies left · ${R.mates} teammates · "${R.line}"`);

  // ---- 2 · a gunner watching the door
  check(!!R.markOn, "the hall's front door is on the picture from where he stands", `mark on the picture ${R.markOn} · he is at ${R.pos}`);
  await point(at('front')); await tap('KeyQ'); R = await until(x => x.i >= 5, 60);
  check(R.id === 'around' && (R.by.hall ?? []).length === 4, 'the squad\'s key on the front door: all four stack on it and the next line walks him round behind the hall', `step '${R.id}' at ${R.t} s · on a stack: ${JSON.stringify(R.by)} · "${R.line}"`);
  ok = await walk(at('around'), [[23.3, 0, 13.4], [23.3, 0, 20.4]]); R = await until(x => x.i >= 6, 6);
  check(ok && R.id === 'back' && R.markOn && cards.some(c => c.h === 'A gunner watching the door' && c.closed), 'walked round the hall: the gunner\'s card comes and goes and the back door\'s mark is on the picture', `reached ${ok} at ${R.pos} · step '${R.id}' · mark on the picture ${R.markOn} · cards: ${cards.map(c => c.h).join(' · ')}`);
  await point(at('back')); await shiftTap(); R = await read();
  check((R.by.hall ?? []).length === 2 && (R.by.hall_back ?? []).length === 2 && !R.plan, 'Shift with the key on the back door: two are sent round to it and two stay on the front door', `on a stack: ${JSON.stringify(R.by)}`);
  R = await until(x => x.id === 'word2', 80);
  check(R.id === 'word2', 'the pair comes round and stands on the back door: the line asks for the word', `step '${R.id}' at ${R.t} s · "${R.line}" keys [${R.keys}]`);
  await tap('KeyF'); R = await until(x => x.i >= 8, 40);
  check(R.id === 'call' && R.men === 4 && R.mates === 4, 'the hall is taken through both doors with nobody of his lost', `step '${R.id}' at ${R.t} s · ${R.men} enemies left · ${R.mates} teammates`);

  // ---- 3 · two gates
  await until(x => cards.some(c => c.h === 'Cover two gates'), 4);
  await self(); await tap('KeyQ'); R = await until(x => x.i >= 9, 6);
  check(R.id === 'yard' && R.following === 4, 'the squad\'s key on himself: all four fall in and the next line walks him out to the yard', `step '${R.id}' · following ${R.following} · "${R.line}"`);
  ok = await walk(at('yard'), [[23.3, 0, 20.4], [23.3, 0, 13.0]]); R = await until(x => x.i >= 10, 8);   // (back round the hall's end, over open ground: the way through the hall is the scripted player's — tools/qa/lessons/tut-queue.ts)
  check(ok && R.id === 'gateA' && R.markOn, "walked out to the yard's mark: the road gate's spot is on the picture", `reached ${ok} at ${R.pos} · step '${R.id}' · mark on the picture ${R.markOn}`);
  await run(120);   // (the file closes up behind him, as a hand waits for it)
  await point(at('gateA')); await shiftTap(); R = await until(x => x.i >= 11, 6); { const e = at('gateA'), k = `${e[0].toFixed(1)},${e[2].toFixed(1)}`;
    check(R.id === 'gateB' && (R.watch[k] ?? []).length === 4 && cards.some(c => c.h === 'Pick their order'), 'Shift with the key by the road gate: all four turn to watch it from where they stand, and the wheel\'s card comes', `step '${R.id}' · watching: ${JSON.stringify(R.watch)} · cards: ${cards.map(c => c.h).join(' · ')}`); }
  // (the last gate is the wheel's: a Shift tap gives nothing and says to keep the key down)
  R = await until(x => x.up && !!x.line, 3);
  { const n0 = R.orders; await point(at('gateB')); await shiftTap(); const a = await read();
    check(a.orders === n0 && /held down/.test(a.note) && !a.ring, 'on the wheel\'s step a Shift tap gives no order and the line says to keep the key down', `orders ${n0} → ${a.orders} · the note "${a.note}" · "${a.line}" keys [${a.keys}]`); }
  await point(at('gateB')); await page.keyboard.down('ShiftLeft'); await run(2); await page.keyboard.down('KeyQ'); await run(50); await point(at('gateB')); R = await read();
  { const want = L.steps.find(T => T.id === 'gateB').flat.replace(/\[([^\]]+)\]/g, '$1');
    check(!!R.ring && R.ring.part && R.ring.flat && R.ring.men.length === 2 && R.ring.rows.indexOf('cover') === 1 && R.ring.rows.length <= 5 && R.line.replace(/\s+/g, ' ').trim() === want && R.keys.includes('2'),
      'Shift and the key kept down: the one ring is up for two of them, no more than five wedges, and the line is the step\'s own for the wedge to press, its digit a key cap', `ring ${JSON.stringify(R.ring)} · "${R.line}" keys [${R.keys}]`); }
  await page.keyboard.down('Digit2'); await run(2); await page.keyboard.up('Digit2'); await run(3); await page.keyboard.up('KeyQ'); await page.keyboard.up('ShiftLeft'); await run(6); R = await until(x => x.i >= 12, 6);
  { const e = at('gateA'), n = at('gateB'), ke = `${e[0].toFixed(1)},${e[2].toFixed(1)}`, kn = `${n[0].toFixed(1)},${n[2].toFixed(1)}`;
    check(R.id === 'hold' && (R.watch[ke] ?? []).length === 2 && (R.watch[kn] ?? []).length === 2 && !R.ring && !R.plan, 'its second wedge: that pair turns to the back gate, the other pair stays on the road gate, the ring is away', `step '${R.id}' · watching: ${JSON.stringify(R.watch)}`); }
  R = await until(x => x.over && x.men === 0, 90);
  check(R.over && R.men === 0 && R.mates === 4, 'both gates are held: every step is done, all eight enemies are down and nobody of his is lost', `over ${R.over} at ${R.t} s · ${R.men} enemies left · ${R.mates} teammates · end ${JSON.stringify(R.end)}`);
  await run(240); R = await read();
  { const want = L.steps.map(T => T.card).filter(Boolean), met = cards.map(c => c.h), open = cards.filter(c => !c.closed).map(c => c.h);
    check(want.every(h => met.includes(h)) && !open.length, 'every card of the level was met and closed', `the level's: ${want.join(' · ')} · met: ${met.join(' · ')}${open.length ? ` · LEFT OPEN: ${open.join(' · ')}` : ''}`); }
  check(!R.fatal && errors.length === 0, 'no page error from boot to the end', errors.length ? errors.slice(0, 4).join(' | ') : `none · fatal "${R.fatal}"`);
  await browser.close();
  console.log(failed ? '=== tut-queue-page: FAIL ===' : '=== tut-queue-page: ALL PASS ===');
  process.exit(failed ? 1 : 0);
})().catch(e => { console.error('tut-queue-page: ' + (e && e.stack || e)); process.exit(1); });
