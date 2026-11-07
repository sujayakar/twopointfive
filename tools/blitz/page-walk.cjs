// THE PAGE WALKED IN HEADLESS CHROMIUM, the road a player takes: the title, Play, a squad mission's row, Deploy (a page load), the first seconds of
// the mission by the keys (an order, the squad's ring held up on a door, a wedge by its digit, a frag), then the solo night at its own address to its
// first checkpoint, a save by hand and its load. Software WebGPU adapter (tools/qa/page-look.cjs's recipe): this holds that the page boots, that the
// menus lead to a mission and that the game answers the keys. It does not look at the picture.
//   PORT=5173 node tools/blitz/page-walk.cjs [title] [squad] [solo]        (a dev server on that port: PORT=5173 bun run tools/serve.ts; no part named = all)
//   env: WORD     what the address carries to choose the game: '' (the page's own default) · 'rust=1' · 'rust=0'
//        QUALITY  default verylow · VIEW default 640x360
// Every line is `- PASS — …` or `- FAIL — …` with what was read under it. exit 0 = no FAIL, no page error.
// A tutorial mission by a hand's own keys is the tools beside tools/qa/tut-page.cjs (tut-team-page.cjs ends its mission in about two minutes of play).
// On the software adapter a page takes minutes to come ready (every kernel is compiled on the CPU); on a machine with a GPU, seconds.
let chromium; try { ({ chromium } = require('playwright')); } catch { ({ chromium } = require('/opt/node-tools/node_modules/playwright')); }
const PORT = process.env.PORT || 5173, WORD = process.env.WORD || '', QUALITY = process.env.QUALITY || 'verylow';
const FLAGS = ['--enable-unsafe-webgpu', '--use-gl=angle', '--use-angle=swiftshader', '--enable-features=Vulkan', '--use-vulkan=swiftshader', '--ignore-gpu-blocklist', '--disable-gpu-watchdog'];
const parts = process.argv.slice(2).length ? process.argv.slice(2) : ['title', 'squad', 'solo'];
const T0 = Date.now(), log = (...a) => console.log(`[${((Date.now() - T0) / 1000).toFixed(0)}s]`, ...a);
let failed = false;
const check = (ok, label, detail) => { if (!ok) failed = true; console.log(`- ${ok ? 'PASS' : 'FAIL'} — ${label}\n    ${detail}`); };
const url = q => `http://localhost:${PORT}/?${[q, WORD, `quality=${QUALITY}`, 'mute'].filter(Boolean).join('&')}`;
const ready = async page => { await page.waitForFunction(() => !!window.__ready && typeof window.__ready.then === 'function', null, { timeout: 0 }); return page.evaluate(async () => { try { await window.__ready; return ''; } catch (e) { return String(e && e.message || e).slice(0, 400); } }); };
// what every part reads at its end: which game steps the page, and whether anything fell over
const state = page => page.evaluate(() => { const R = window.__rust, f = document.getElementById('fatal'), p = document.getElementById('rustpanic');
  return { game: R && R.on ? 'the Rust game steps this page' : 'no Rust game on this page (a title page has none until a mission is opened)', fatal: f && f.style.display === 'flex' ? (f.textContent || '').slice(0, 300) : '', stopped: p && p.style.display !== 'none' ? (p.textContent || '').slice(0, 300) : '',
    rust: R && R.on ? { ticks: R.ticks, outThrew: Object.keys(R.outThrew || {}), dressThrew: Object.keys((R.dress && R.dress.threw) || {}), asksRefused: R.asksRefused || 0, panics: R.panic ? R.panic.n : 0, applyThrew: R.panic ? R.panic.applyThrew : 0 } : null }; });
const sound = (S, tag) => check(!S.fatal && !S.stopped && (!S.rust || (!S.rust.outThrew.length && !S.rust.dressThrew.length && !S.rust.asksRefused && !S.rust.panics && !S.rust.applyThrew)), `${tag}: nothing fell over (no red card, no stopped game, no arm of the host threw, no question refused)`, `${S.game} · ${JSON.stringify(S.rust)}${S.fatal ? ' · RED CARD: ' + S.fatal : ''}${S.stopped ? ' · STOPPED: ' + S.stopped : ''}`);
// THE HANDS, in the page: key codes and the mouse written into the page's own Input a frame at a time, frames stepped undrawn
const HANDS = `(() => { const g = () => window.__game, I = window.__input, cam = window.__cam, cv = document.getElementById('view');
  const step = (n = 1) => window.__runFrames(n, 1 / 60, [], 'none');
  const aim = p => { const [x, y, front] = cam.project([p[0], p[1], p[2]], cv.clientWidth, cv.clientHeight); if (front) { I.mouseX = x; I.mouseY = y; } return !!front; };
  const press = (code, after = 4) => { I.pressed.add(code); step(1 + after); };
  const hold = (codes, frames, stop) => { for (let i = 0; i < frames; i++) { for (const c of codes) I.keys.add(c); step(1); if (stop && stop()) break; } for (const c of codes) I.keys.delete(c); step(1); };
  const flat = (a, b) => Math.hypot(a[0] - b[0], a[2] - b[2]);
  return { g, I, step, aim, press, hold, flat, pos: () => g().player.char.pos }; })()`;

(async () => {
  const browser = await chromium.launch({ headless: true, channel: 'chromium', args: FLAGS });
  const page = await (await browser.newContext({ viewport: (v => ({ width: +v[0], height: +v[1] }))((process.env.VIEW || '640x360').split('x')), deviceScaleFactor: 1 })).newPage();
  const errors = [];
  page.on('pageerror', e => errors.push('page error: ' + String(e && e.stack || e).slice(0, 400)));
  page.on('console', m => { if (m.type() === 'error' && !/Failed to load resource/.test(m.text())) errors.push('console error: ' + m.text().slice(0, 300)); });
  let missionHref = '';

  if (parts.includes('title')) {
    log(url(''));
    await page.goto(url(''), { timeout: 0 }); const why = await ready(page);
    const T = await page.evaluate(() => ({ mode: window.__menu ? window.__menu.mode : null, play: [...document.querySelectorAll('button')].filter(b => b.getClientRects().length && /^Play/.test((b.textContent || '').trim())).length }));
    check(!why && T.mode === 'start' && T.play >= 1, 'the title comes ready with its Play button', `ready ${why || 'yes'} · menu '${T.mode}' · Play buttons ${T.play}`);
    await page.evaluate(() => { const b = [...document.querySelectorAll('button')].find(b => b.getClientRects().length && /^Play/.test((b.textContent || '').trim())); if (b) b.click(); }); await page.waitForTimeout(800);
    const rows = await page.evaluate(() => [...document.querySelectorAll('#missions a.mrow, #missions a[href]')].filter(a => a.getClientRects().length).map(a => ({ text: (a.textContent || '').trim().replace(/\s+/g, ' ').slice(0, 40), href: a.getAttribute('href') || '' })));
    const row = rows.find(r => /compound/.test(r.href)) || rows.find(r => /squad4/.test(r.href));
    check(rows.length >= 3 && !!row, 'Play opens the mission list, and a squad mission has its row', `${rows.length} rows · ${row ? `'${row.text}' -> ${row.href}` : 'NO SQUAD ROW among ' + rows.slice(0, 6).map(r => r.text).join(' | ')}`);
    if (row) { await page.evaluate(h => { const a = [...document.querySelectorAll('#missions a.mrow, #missions a[href]')].find(a => a.getAttribute('href') === h); if (a) a.click(); }, row.href); await page.waitForTimeout(1200);
      const dep = await page.evaluate(() => { const b = document.querySelector('[data-act="deploy"]') || [...document.querySelectorAll('button')].find(b => b.getClientRects().length && /^Deploy/.test((b.textContent || '').trim())); return b ? (b.textContent || '').trim().slice(0, 40) : ''; });
      check(!!dep, 'the row\'s sheet has its Deploy button', dep ? `[${dep}]` : 'NO DEPLOY BUTTON'); missionHref = row.href;
      if (dep && parts.includes('squad')) { const nav = page.waitForNavigation({ timeout: 0 }); await page.evaluate(() => { const b = document.querySelector('[data-act="deploy"]') || [...document.querySelectorAll('button')].find(b => b.getClientRects().length && /^Deploy/.test((b.textContent || '').trim())); b.click(); }); await nav; missionHref = 'deployed'; } }
    sound(await state(page), 'the title');
  }

  if (parts.includes('squad')) {
    if (missionHref !== 'deployed') { const u = url('levelFile=compound.json&roomclear&squad4'); log(u); await page.goto(u, { timeout: 0 }); }
    const why = await ready(page);
    const F = await page.evaluate(async () => { const b = [...document.querySelectorAll('#menu button')].find(b => b.getClientRects().length && /^(Start|Go)/.test((b.textContent || '').trim())); if (b) b.click(); else if (window.__menu.mode !== 'closed') window.__menu.close(); await new Promise(r => setTimeout(r, 300));
      const g = window.__game; g.godMode = true; return { url: location.search, menu: window.__menu.mode, men: g.guards.length, ours: g.guards.filter(x => x.unit && x.team === 'operative').length, doors: g.doors.list.length }; });
    check(!why && F.menu === 'closed' && F.ours >= 1 && F.doors >= 1, 'the mission\'s page comes ready and its start card closes on a squad', `ready ${why || 'yes'} · ${JSON.stringify(F)}`);
    const R = await page.evaluate(`(() => { const H = ${HANDS}, g = H.g(), O = g.orders, out = {};
      H.step(60); H.hold(['KeyW'], 60);
      const door = () => g.doors.list.filter(d => d.isClosed()).sort((a, b) => H.flat(a.frameCentre, H.pos()) - H.flat(b.frameCentre, H.pos()))[0];
      const d = door(), c = d.frameCentre; out.door = d.def.name; out.seen = H.aim([c[0], 1.0, c[2]]); H.step(2);
      H.press('KeyB', 6); out.afterTap = O.log.map(l => l.order); H.step(180);
      // the command key kept down on the door: the ring, as the overlay reads it
      H.aim([c[0], 1.0, c[2]]); H.step(2); H.I.pressed.add('KeyB'); H.hold(['KeyB'], 30); H.step(2);
      const v = O.ringView(); out.ring = v ? { caption: v.caption, wedges: v.wedges.map(w => w.key + ':' + w.text + (w.off ? '(off)' : '')) } : null; out.targetKind = O.menu && O.menu.target ? String(O.menu.target.kind) : 'no menu';
      // a wedge by its digit: the order logged is the wedge's own
      const w = v && v.wedges.find(x => /^cover/.test(x.text) && !x.off); if (w) { H.press('Digit' + w.key, 6); out.digit = w.key + ' ' + w.text; out.afterDigit = O.log.map(l => l.order); }
      const away = () => { for (let i = 0; i < 3 && O.cmdRing; i++) { H.I.clicked |= 4; H.step(4); } }; away();
      const ringUp = p => { H.aim(p); H.step(2); H.I.pressed.add('KeyB'); H.hold(['KeyB'], 30); H.step(2); return O.ringView(); };
      // the door's second page (the wedge that opens it), and a wedge there by its digit: 'bang and clear' must give the bang, whatever its place
      { let v2 = ringUp([c[0], 1.0, c[2]]); const more = v2 && v2.wedges.find(x => x.opens); if (more) { H.press('Digit' + more.key, 4); v2 = O.ringView(); }
        out.page2 = v2 ? v2.wedges.map(w => w.key + ':' + w.text + (w.off ? '(off)' : '')) : null; const NAMES = [[/^bang and clear/, 'bangClear'], [/^open and clear/, 'openClear'], [/^breach with a charge/, 'charge'], [/^clear from outside/, 'thresholdHold'], [/^shotgun and clear/, 'bigClear'], [/^frag and clear/, 'fragClear']];
        const pick = v2 && NAMES.map(([re, id]) => ({ w: v2.wedges.find(x => re.test(x.text) && !x.off), id })).find(x => x.w); if (pick) { H.press('Digit' + pick.w.key, 6); out.tool = pick.w.key + ' ' + pick.w.text; out.toolWants = pick.id; out.afterTool = O.log.map(l => l.order); } away(); }
      // the ring on a floor spot beside him
      { const p = H.pos(), d2 = [c[0] - p[0], c[2] - p[2]], L = Math.hypot(d2[0], d2[1]) || 1; const v3 = ringUp([p[0] - d2[0] / L * 2.5, 0.05, p[2] - d2[1] / L * 2.5]); out.floor = v3 ? { caption: v3.caption, wedges: v3.wedges.map(w => w.key + ':' + w.text), kind: String(O.menu && O.menu.target && O.menu.target.kind) } : null; away(); }
      // a frag at the nearest man of the other side, the rifle back in hand, five seconds on
      const foe = g.guards.filter(x => x.team !== 'operative' && !x.civ && x.state !== 'dead').sort((a, b) => H.flat(a.char.pos, H.pos()) - H.flat(b.char.pos, H.pos()))[0];
      H.press('Digit5', 30); if (foe) { H.aim([foe.char.pos[0], 0.05, foe.char.pos[2]]); H.step(1); H.I.clicked |= 1; H.step(20); } out.inTheAir = g.items.items.length; H.press('Digit2', 30); H.step(300);
      out.end = { t: +g.time.toFixed(1), log: O.log.length, down: !!g.player.down, lines: g.messages.slice(-2).map(m => m.text) }; return out; })()`);
    check(R.seen && R.afterTap.length >= 1, 'the command key on a door gives the squad an order', `the door ${R.door}, on the view ${R.seen} · the log after the tap [${R.afterTap}]`);
    check(!!R.ring && R.ring.wedges.some(w => w.startsWith('1:')) && !/the squad itself/.test(R.ring.caption) && R.targetKind === 'door', 'the key kept down puts the ring up ON THE DOOR: its caption names the door and its first wedge is the door\'s own order', R.ring ? `caption '${R.ring.caption}' · wedges ${R.ring.wedges.join(' | ')} · the menu's target is a '${R.targetKind}'` : 'NO RING');
    check(!!R.digit && R.afterDigit[R.afterDigit.length - 1] === 'cover', 'a wedge\'s digit gives that wedge\'s order', R.digit ? `digit ${R.digit} · the log [${R.afterDigit}]` : 'no cover wedge on the ring');
    check(!!R.tool && R.afterTool[R.afterTool.length - 1] === R.toolWants, 'a door tool on the ring\'s second page gives that tool, whatever digit it sits on', R.tool ? `page two ${R.page2.join(' | ')} · digit ${R.tool} · the log [${R.afterTool}] (wanted '${R.toolWants}')` : `no door tool to pick on page two: ${R.page2 ? R.page2.join(' | ') : 'NO RING'}`);
    check(!!R.floor && R.floor.kind !== 'door' && R.floor.kind !== 'undefined' && R.floor.wedges.some(w => w.startsWith('1:')) && (R.floor.kind !== 'floor' || /that (spot|wall)/.test(R.floor.caption)), 'the ring on the floor beside him is the floor\'s (or his own): a first wedge, and a floor spot named as one', R.floor ? `a '${R.floor.kind}' · caption '${R.floor.caption}' · wedges ${R.floor.wedges.join(' | ')}` : 'NO RING');
    check(R.inTheAir >= 1 && R.end.t > 8, 'a frag leaves the hand and the mission runs on', `${R.inTheAir} thing(s) in the air · ${JSON.stringify(R.end)}`);
    sound(await state(page), 'the squad mission');
  }

  if (parts.includes('solo')) {
    const u = url('solo&seed=7&paused'); log(u); await page.goto(u, { timeout: 0 }); const why = await ready(page);
    const R = await page.evaluate(`(() => { const H = ${HANDS}; if (window.__menu.mode !== 'closed') window.__menu.close(); const g = H.g(), sv = g.saves, out = {};
      const slots = () => sv.slots().map(s => s.label + '@' + (+s.missionT).toFixed(1));
      H.step(60); H.hold(['KeyW'], 240); H.hold(['KeyD'], 30); for (let i = 0; i < 1800 && sv.stats.autosaves < 1; i++) H.step(1);
      out.checkpoint = slots(); out.tCheckpoint = +g.time.toFixed(1);
      H.press('F5', 30); out.byHand = slots(); const at = [...H.pos()]; H.hold(['KeyS'], 120); H.step(120); const t0 = g.time;
      const said = window.__save.load(); H.step(30);
      out.load = { said: said || 'loaded', clock: [+t0.toFixed(1), +g.time.toFixed(1)], back: H.flat(at, H.pos()) < 0.5, line: (g.messages[g.messages.length - 1] || {}).text || '', loads: g.chronicle.counts.loads };
      H.hold(['KeyW'], 120); out.end = { t: +g.time.toFixed(1), down: !!g.player.down }; return out; })()`);
    check(!why && R.checkpoint.length >= 1, 'the solo night comes ready and takes its first checkpoint by itself', `ready ${why || 'yes'} · [${R.checkpoint}] by ${R.tCheckpoint} s`);
    check(R.byHand.length > R.checkpoint.length, 'F5 keeps a save by hand', `[${R.byHand}]`);
    check(R.load.clock[1] < R.load.clock[0] && R.load.back && /checkpoint loaded/.test(R.load.line), 'its load puts him back where he saved, with the clock', JSON.stringify(R.load));
    sound(await state(page), 'the solo night');
  }

  check(errors.length === 0, 'no page error and no console error', errors.slice(0, 5).join(' | ') || 'none');
  try { const bp = browser.process && browser.process(); if (bp) bp.kill('SIGKILL'); } catch {}   // (a close of this browser can hang on the software adapter)
  process.exit(failed ? 1 : 0);
})().catch(e => { console.log('- FAIL — the walk itself threw\n    ' + String(e && e.stack || e).slice(0, 900)); process.exit(1); });
