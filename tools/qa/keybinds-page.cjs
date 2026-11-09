// THE PLAYER'S OWN KEYS ON THE REAL PAGE (src/host/keymap.ts, src/ui/keybinds.ts, Options ▸ Controls): headless Chromium against a dev server, the keys
// pressed for real.
//   PORT=5199 node tools/qa/keybinds-page.cjs [outDir]        (a dev server on that port: PORT=5199 bun run tools/serve.ts)
//   env: GPU=1 adds the game itself (the software WebGPU adapter under the page: two boots, about four minutes each) · ONLY=game skips the screen
// THE SCREEN (no adapter: the boot stops at 'No WebGPU adapter', and the options are plain DOM that is live from the first tick):
//   every key that can move is a cap you can click, the page's own keys are plain caps · a click and a key: the cap is that key, and the machine keeps it ·
//   by the keys alone (W S to a cap, Enter, the key): a key another row has swaps the two and shows the other · one of the page's own keys is shaken off and
//   the cap goes on waiting · Esc lets it go and leaves the screen up · Backspace leaves a row with no key, marked · a digit pressed for a waiting cap is its
//   key, never the screen's tab · the words beside a cap name the keys as bound · a page load later the caps are as they were left ·
//   Reset is the build's keys and nothing kept · ?keybinds=0: the plain list, the kept table untouched.
//   THE PAGE'S OWN BOARD (host/keymap.ts qFree, the last three lines; all the above run under ?qfree=0): the wall hold's cap is a dashed —, unmarked, its words
//   say walk into it · a click and Q: the cap is Q, marked, kept; Reset is — again · ?wallpress=0: Q again · ?keybinds=0: a plain —.
//   Q and F only: the orders menu's cap and the who cap are a dashed —, unmarked · a click and X: the menu's cap is X, marked, kept; Reset is — again.
//   NO ROW HOLDS A KEY UNSEEN (its own line, the page's own board): every row with a key has its cap on the list; G and H give nothing.
//   EMPTY HANDS ARE A SLOT (rust/game_types/src/handslot, host/keymap.ts SLOT; the last line): 'Empty hands' heads the kit's rows on a cap that reads backtick, unmarked;
//   there is no row for E; the debugger's cap reads backslash, unmarked · a click and K: the slot's cap is K, marked, kept; Reset is backtick again.
// GPU=1, THE GAME: crouch put on K on the screen, over the start card: K crouches him at once, C does nothing; the kit column names the light's new key; a
//   door's Space row names Space's new key; a page load later K still crouches; no page error, no GPU error.
// exit 0 = every line PASS with no page error · 1 = otherwise. Nothing here has been seen on a GPU, and nobody has played it.
const fs = require('fs'), path = require('path');
const PAGE = require('./page.cjs').page({ name: 'keybinds-page' }), { NO_GPU, late } = require('./page.cjs'), { check, note: log } = PAGE;
const GPU = process.env.GPU === '1', QUALITY = process.env.QUALITY || 'verylow', outDir = process.argv[2] || '';
const NOISE = /""?smoke[A-Za-z0-9_]*""?|\[pipeline\] smoke_|number of storage textures \(5\) in the Compute stage|No WebGPU adapter|WebGPU is not available|Failed to load resource/;
if (outDir) fs.mkdirSync(outDir, { recursive: true });
const errors = [];
// (the screen's browser has no GPU)
const watching = (name, gpu) => ({ into: errors, as: `${name}: `, thrown: gpu ? undefined : /WebGPU adapter/, skip: gpu ? NOISE : [NOISE, /GPU|adapter|WebGPU/i] });
const still = async (page, name) => { if (!outDir) return; const ok = await page.screenshot({ path: path.join(outDir, name), timeout: 180000 }).then(() => true, () => false); if (!ok) log(`no picture ${name}: the screenshot did not come`); };   // (a picture for a person: one that will not come is said, never a red)
// (`board`: every line but the last four is asked on the board as it was, the wall hold on Q, E the holster, Space the action and V the whistle —
// ?qfree=0&handslot=0&handring=0&qonly=0; the page's own board gives the wall's row no key, has empty hands as a slot on backtick with the debugger on
// backslash, and gives the orders menu's row and the who row no key (Q and F are the squad's only keys: the ring is the menu).
// &slotsetup=0 on every line: the selector and the light on Z and L and the kit column as it was — the rows a gun's setup frees and its box are not looked at here)
const url = (more = '', board = '&qfree=0&qonly=0') => PAGE.url(`paused&quality=${QUALITY}&mute&slotsetup=0${board}${more ? '&' + more : ''}`);
const openControls = async page => { await page.waitForSelector('#menu .actions button', { timeout: 60000 });
  await page.evaluate(() => [...document.querySelectorAll('#menu .actions button')].find(b => /^Options/.test(b.textContent))?.click()); await page.waitForSelector('#options.on', { timeout: 10000 });
  await page.evaluate(() => document.querySelector('#options .op-tab[data-tab="controls"]').click()); await page.waitForTimeout(150); };   // (a click by the page's own hand: under the software adapter the frame is seconds late and a pointer waits for it)
// ── in the page ──
const CAPS = () => Object.fromEntries([...document.querySelectorAll('#options button.op-key')].map(b => [b.dataset.key, { t: b.textContent, c: b.className.replace('op-key', '').trim() }]));
const KEPT = () => localStorage.getItem('twopointsix.keys');
const SCREEN = () => ({ up: !!document.querySelector('#options.on'), tab: document.querySelector('#options .op-tab.on')?.dataset.tab, desc: document.querySelector('#options .op-desc')?.textContent ?? '', plain: [...document.querySelectorAll('#options .op-caps kbd')].map(k => k.textContent) });

const marksOf = c => c.replace(/\b(sel|none)\b/g, '').trim();   // (a marked cap's classes: a row the build gives no key reads `none`, unmarked)
async function screen(browser) {
  const { context: ctx, page } = await PAGE.open({ browser, size: [1280, 800], errors: watching('screen', false) });
  await page.goto(url(), { waitUntil: 'domcontentloaded' }); await openControls(page);
  const key = async k => { await page.keyboard.press(k); await page.waitForTimeout(60); };
  { const caps = await page.evaluate(CAPS), S = await page.evaluate(SCREEN), want = { KeyW: 'W', KeyA: 'A', KeyS: 'S', KeyD: 'D', ShiftLeft: 'Shift', KeyC: 'C', KeyQ: 'Q', Space: 'E', KeyV: '—', KeyN: 'N', KeyR: 'R', KeyZ: 'Z', KeyL: 'L', Digit0: '`', Digit1: '1', Digit2: '2', Digit3: '3', Digit4: '4', Digit5: '5', KeyB: 'B', KeyT: 'T', KeyF: 'F', Backquote: '\\' };
    const off = Object.keys(want).filter(k => caps[k]?.t !== want[k] || marksOf(caps[k].c)), fixed = ['Mouse', 'Wheel', 'LMB', 'RMB', 'Esc', 'Enter', 'F7'].filter(k => !S.plain.includes(k));
    check(!off.length && Object.keys(caps).length === Object.keys(want).length && !fixed.length && await page.evaluate(KEPT) === null, 'Options ▸ Controls: every key that can move is a cap to click, reading the build\'s key and unmarked; the mouse and the page\'s own keys are plain caps',
      `${Object.keys(caps).length} caps to click${off.length ? `, wrong: ${off.map(k => `${k} '${caps[k]?.t}' [${caps[k]?.c}]`).join(' · ')}` : ''} · plain caps missing: ${fixed.join(' ') || 'none'}`); await still(page, 'controls-as-built.png'); }
  { await page.click('#options button.op-key[data-key="KeyC"]'); await page.waitForTimeout(80); const w = (await page.evaluate(CAPS)).KeyC, d = (await page.evaluate(SCREEN)).desc; await still(page, 'controls-waiting.png');
    await key('k'); const c = (await page.evaluate(CAPS)).KeyC, kept = await page.evaluate(KEPT), S = await page.evaluate(SCREEN);
    check(w.t === '…' && /wait/.test(w.c) && /Press its new key/.test(d) && c.t === 'K' && /moved/.test(c.c) && !/wait/.test(c.c) && kept === '{"KeyC":"KeyK"}' && S.up && S.tab === 'controls',
      'a click on Crouch\'s cap and it waits; K pressed and the cap is K, marked as moved, and the machine has kept it', `waiting: '${w.t}' [${w.c}] '${d.slice(0, 60)}' · after K: '${c.t}' [${c.c}] · kept ${kept}`); }
  { await key('s'); await key('Enter'); const sel = await page.evaluate(() => document.querySelector('#options button.op-key.wait')?.dataset.key); await key('k');   // (W S walk the caps: the next one down is Q's)
    const caps = await page.evaluate(CAPS), kept = JSON.parse(await page.evaluate(KEPT) || '{}');
    check(sel === 'KeyQ' && caps.KeyQ.t === 'K' && caps.KeyC.t === 'Q' && /swap/.test(caps.KeyC.c) && kept.KeyQ === 'KeyK' && kept.KeyC === 'KeyQ', 'by the keys alone (S to the next cap, Enter, K): the wall\'s key takes K, which Crouch had, and Crouch takes Q in exchange and shows it',
      `the cap that waited: ${sel} · wall '${caps.KeyQ.t}', crouch '${caps.KeyC.t}' [${caps.KeyC.c}] · kept ${JSON.stringify(kept)}`); await still(page, 'controls-swapped.png'); }
  { await key('Enter'); await key('Tab'); const a = (await page.evaluate(CAPS)).KeyQ, d = (await page.evaluate(SCREEN)).desc; await key('ArrowUp'); const a2 = (await page.evaluate(CAPS)).KeyQ; await key('Escape'); const b = (await page.evaluate(CAPS)).KeyQ, S = await page.evaluate(SCREEN);
    check(/wait/.test(a.c) && /no/.test(a.c) && /page needs that key/.test(d) && /wait/.test(a2.c) && b.t === 'K' && !/wait/.test(b.c) && S.up, 'a page\'s own key (Tab, an arrow) is shaken off and the cap goes on waiting; Esc lets it go with nothing changed and the screen stays up',
      `after Tab: [${a.c}] '${d.slice(0, 50)}' · after ↑: [${a2.c}] · after Esc: '${b.t}' [${b.c}], screen up ${S.up}`); }
  { await key('Enter'); await key('Backspace'); const q = (await page.evaluate(CAPS)).KeyQ, small = await page.evaluate(() => [...document.querySelectorAll('#options .op-krow')].find(r => r.querySelector('[data-key="KeyQ"]'))?.querySelector('small')?.textContent ?? '');
    await page.click('#options button.op-key[data-key="KeyR"]'); await key('2'); const caps = await page.evaluate(CAPS), S = await page.evaluate(SCREEN);
    check(q.t === '—' && /none/.test(q.c) && /, then step away to let go/.test(small) && !/Q|—/.test(small) && caps.KeyR.t === '2' && caps.Digit2.t === 'R' && S.tab === 'controls', 'Backspace leaves the wall with no key, marked, and the words beside it say so; a digit pressed for a waiting cap is its key (the rifle\'s row takes R in exchange), never the screen\'s tab',
      `wall '${q.t}' [${q.c}] · '${small}' · reload '${caps.KeyR.t}', rifle '${caps.Digit2.t}', tab ${S.tab}`); await still(page, 'controls-moved.png'); }
  { const before = await page.evaluate(CAPS); await page.reload({ waitUntil: 'domcontentloaded' });
    await openControls(page); const after = await page.evaluate(CAPS), diff = Object.keys(before).filter(k => before[k].t !== after[k]?.t);
    check(!diff.length && after.KeyQ.t === '—' && after.KeyR.t === '2' && after.KeyC.t === 'Q', 'a page load later every cap is as it was left', `caps that differ: ${diff.join(' ') || 'none'} · wall '${after.KeyQ.t}', reload '${after.KeyR.t}', crouch '${after.KeyC.t}'`); await still(page, 'controls-after-load.png'); }
  { await page.goto(url('keybinds=0'), { waitUntil: 'domcontentloaded' }); await openControls(page); const n = Object.keys(await page.evaluate(CAPS)).length, S = await page.evaluate(SCREEN), kept = await page.evaluate(KEPT), reset = await page.evaluate(() => !!document.querySelector('#options [data-act="reset"]'));
    check(n === 0 && S.plain.includes('C') && S.plain.includes('Q') && S.plain.includes('R') && !reset && /KeyQ/.test(kept || ''), '?keybinds=0: the plain list of the build\'s keys, nothing to click and no Reset, and the kept table left as it was', `caps to click ${n} · C Q R plain: ${['C', 'Q', 'R'].map(k => S.plain.includes(k))} · Reset ${reset} · kept ${kept}`); }
  { await page.goto(url(), { waitUntil: 'domcontentloaded' }); await openControls(page); await page.click('#options [data-act="reset"]'); await page.waitForTimeout(80);
    const caps = await page.evaluate(CAPS), off = Object.entries(caps).filter(([, v]) => marksOf(v.c)).map(([k]) => k), kept = await page.evaluate(KEPT);
    check(caps.KeyC.t === 'C' && caps.KeyQ.t === 'Q' && caps.KeyR.t === 'R' && caps.Digit2.t === '2' && !off.length && kept === null, 'Reset: every cap is the build\'s again, none marked, and nothing is kept', `C Q R 2: ${caps.KeyC.t} ${caps.KeyQ.t} ${caps.KeyR.t} ${caps.Digit2.t} · still marked: ${off.join(' ') || 'none'} · kept ${kept}`); }
  { await page.goto(url('bkey', ''), { waitUntil: 'domcontentloaded' }); await openControls(page);   // (the wall hold's rule by itself, the squad's command left on B: with it on Q, as the page opens, Q given to the wall is a swap)
    const row = () => page.evaluate(() => { const b = document.querySelector('#options button.op-key[data-key="KeyQ"]'), r = b?.closest('.op-krow'); return { t: b?.textContent ?? '', c: (b?.className ?? '').replace('op-key', '').replace('sel', '').trim(), what: r?.querySelector('.op-what')?.firstChild?.textContent ?? '', small: r?.querySelector('small')?.textContent ?? '' }; });
    const a = await row(), kept0 = await page.evaluate(KEPT);
    await page.evaluate(() => document.querySelector('#options button.op-key[data-key="KeyQ"]').click()); await page.waitForTimeout(80); await key('q'); const b = await row(), kept = await page.evaluate(KEPT);
    await page.click('#options [data-act="reset"]'); await page.waitForTimeout(80); const c = await row(), kept2 = await page.evaluate(KEPT);
    check(a.t === '—' && /none/.test(a.c) && !/moved/.test(a.c) && /Walk into a wall/.test(a.what) && !/Q/.test(a.small) && kept0 === null
      && b.t === 'Q' && /moved/.test(b.c) && /press Q or step away/.test(b.small) && kept === '{"KeyQ":"KeyQ"}' && c.t === '—' && !/moved/.test(c.c) && kept2 === null,
      'the page\'s own board: the wall hold\'s cap is a dashed —, not marked as moved, its words say walk into it; a click and Q puts it on Q (marked, kept); Reset is — again and nothing kept',
      `as it opens '${a.t}' [${a.c}] '${a.what}' '${a.small}' kept ${kept0} · after Q '${b.t}' [${b.c}] '${b.small}' kept ${kept} · after Reset '${c.t}' [${c.c}] kept ${kept2}`); await still(page, 'controls-no-wall-key.png'); }
  { await page.goto(url('', ''), { waitUntil: 'domcontentloaded' }); await openControls(page);   // (Q and F are the squad's only keys, KEYMAP.qOnly: the who row has no key and is not listed)
    const row = id => page.evaluate(id => { const b = document.querySelector(`#options button.op-key[data-key="${id}"]`); return { t: b?.textContent ?? '', c: (b?.className ?? '').replace('op-key', '').replace('sel', '').trim() }; }, id);
    const x0 = await row('KeyX'), t0 = await row('KeyT'), kept0 = await page.evaluate(KEPT);
    check(x0.t === '' && t0.t === '' && kept0 === null, 'the page\'s own board: no row for the orders menu (it is gone) and the who row not listed', `as it opens: orders \'${x0.t}\' [${x0.c}], who \'${t0.t}\' [${t0.c}] kept ${kept0}`); }
  { await page.goto(url('wallpress=0&bkey', ''), { waitUntil: 'domcontentloaded' }); await openControls(page);
    const r = await page.evaluate(() => { const b = document.querySelector('#options button.op-key[data-key="KeyQ"]'), row = b?.closest('.op-krow'); return { t: b?.textContent ?? '', c: (b?.className ?? '').replace('op-key', '').replace('sel', '').trim(), small: row?.querySelector('small')?.textContent ?? '' }; });
    check(r.t === 'Q' && !r.c && /press Q or step away/.test(r.small), 'the page\'s own board with the walk-in switched off (?wallpress=0): the wall hold is on Q again, its cap and its words as they were', `'${r.t}' [${r.c}] '${r.small}'`); }
  { await page.goto(url('keybinds=0&bkey', ''), { waitUntil: 'domcontentloaded' }); await openControls(page); const S = await page.evaluate(SCREEN), n = Object.keys(await page.evaluate(CAPS)).length;
    check(n === 0 && S.plain.includes('—') && !S.plain.includes('Q') && S.plain.includes('C'), 'the page\'s own board under ?keybinds=0: the plain list, the wall hold\'s cap a plain — and no Q in it', `caps to click ${n} · plain caps: ${S.plain.join(' ')}`); }
  { await page.goto(url('bkey', ''), { waitUntil: 'domcontentloaded' }); await openControls(page);
    const rows = () => page.evaluate(() => { const cap = id => { const b = document.querySelector(`#options button.op-key[data-key="${id}"]`), r = b?.closest('.op-krow'); return b ? { t: b.textContent, c: b.className.replace('op-key', '').replace('sel', '').trim(), what: r?.querySelector('.op-what')?.firstChild?.textContent ?? '', first: r === r?.parentElement?.querySelector('.op-krow') } : null; }; return { slot: cap('Digit0'), e: cap('KeyE'), dbg: cap('Backquote'), act: cap('Space'), wh: cap('KeyV'), cam: [...document.querySelectorAll('#options .op-krow')].find(r => /Orbit the camera/.test(r.textContent))?.querySelector('kbd')?.textContent, ring: [...document.querySelectorAll('#options .op-what small')].some(w => /for a ring of/.test(w.textContent)), holster: [...document.querySelectorAll('#options .op-what')].some(w => /Holster or draw/.test(w.textContent)) }; });
    const a = await rows(), kept0 = await page.evaluate(KEPT);
    await page.evaluate(() => document.querySelector('#options button.op-key[data-key="Digit0"]').click()); await page.waitForTimeout(80); await key('k'); const b = await rows(), kept = await page.evaluate(KEPT);
    await page.click('#options [data-act="reset"]'); await page.waitForTimeout(80); const c = await rows(), kept2 = await page.evaluate(KEPT);
    check(!!a.slot && a.slot.t === '`' && !a.slot.c && a.slot.what === 'Empty hands' && a.slot.first && !a.e && !a.holster && a.dbg?.t === '\\' && !a.dbg.c && kept0 === null && a.act?.t === 'E' && a.ring && !!a.wh && a.wh.t !== 'V' && a.cam === 'Space'
      && b.slot?.t === 'K' && /moved/.test(b.slot.c) && kept === '{"Digit0":"KeyK"}' && c.slot?.t === '`' && !c.slot.c && kept2 === null,
      'the page\'s own board: Empty hands heads the kit\'s rows on backtick, unmarked, E has no row, the debugger\'s cap is backslash, the action\'s cap is E with the ring\'s words and the whistle\'s row has no V, the camera\'s cap is Space; a click and K puts the slot on K (marked, kept); Reset is backtick again',
      `as it opens: slot '${a.slot?.t}' [${a.slot?.c}] '${a.slot?.what}' first of its group ${a.slot?.first} · E's row ${a.e ? 'THERE' : 'none'}, 'Holster or draw' ${a.holster} · debugger '${a.dbg?.t}' [${a.dbg?.c}] kept ${kept0} · the action '${a.act?.t}', its words name the ring ${a.ring} · the whistle '${a.wh?.t}' · the camera '${a.cam}' · after K '${b.slot?.t}' [${b.slot?.c}] kept ${kept} · after Reset '${c.slot?.t}' kept ${kept2}`); await still(page, 'controls-empty-hands.png'); }
  await ctx.close();
}

async function game(browser) {
  const opened = await PAGE.open({ browser, errors: watching('game', true) }), ctx = opened.context; let page = opened.page;
  const boot = async () => { const fatal = await page.evaluate(async () => { await window.__ready; return document.getElementById('fatal')?.textContent || ''; }).catch(e => String(e)); log(`booted${fatal ? `: ${fatal}` : ''}`); return fatal; };
  const key = async k => { await page.keyboard.press(k); await page.waitForTimeout(60); };
  /** a real key down, some frames, the key up: what he is doing after */
  const tap = async (k, n = 6) => { await page.keyboard.down(k); const r = await page.evaluate(n => { const filed = [...window.__draw.input.keys].join('+'); window.__draw.run(n, [], 'none'); return { filed }; }, n); await page.keyboard.up(k); await page.evaluate(() => window.__draw.run(2, [], 'none')); return r; };
  await page.goto(url(), { waitUntil: 'domcontentloaded', timeout: 180000 }); const fatal = await boot();
  await openControls(page);
  const cap = id => page.evaluate(id => document.querySelector(`#options button.op-key[data-key="${id}"]`).click(), id);
  await cap('KeyC'); await key('k'); await cap('KeyL'); await key('o'); await cap('Space'); await key('m'); await key('Escape');
  await page.evaluate(() => { window.__draw.run(20, [], 'none'); });
  { const k = await tap('k'), c = await tap('c');
    check(!fatal && k.filed === 'KeyC' && c.filed === '', 'in play, at once: K is filed as the game\'s crouch key; C, which crouch left, files nothing', `K: filed '${k.filed}' · C: filed '${c.filed}'`); }
  await tap('2', 120); await tap('o');   // (the rifle up and its light on: the box names the light's key only while it is on)
  { const hud = await page.evaluate(() => { window.__draw.run(12, [], 'last'); return { gad: [...document.querySelectorAll('#equip .hb-st .o.on')].map(e => e.textContent), keys: [...document.querySelectorAll('#equip .slot .key')].map(e => e.textContent) }; });
    check(hud.gad.includes('O LIGHT') && !hud.gad.includes('L LIGHT') && hud.keys[1] === '1', 'the HUD names the keys as bound: the kit column\'s light is on O, a key not moved reads as ever', `kit column: ${hud.gad.join(' · ')} · slots: ${hud.keys.join(' ')}`); }
  // the page load: a fresh page of the same machine (the same store). The one that played is shut without waiting for it: under the software adapter it answers a reload late
  { const was = page; ({ page } = await PAGE.open({ context: ctx, errors: watching('game, loaded again', true) })); was.close().catch(() => { }); await page.goto(url(), { waitUntil: 'domcontentloaded', timeout: 180000 }); }
  const fatal2 = await boot();
  { await page.evaluate(() => { window.__draw.run(20, [], 'none'); }); const k = await tap('k'), c = await tap('c');
    check(!fatal2 && k.filed === 'KeyC' && c.filed === '', 'a page load later, nothing touched: K is still the crouch key and C files nothing', `K: filed '${k.filed}' · C: filed '${c.filed}'`); }
  await late(ctx.close());   // (the context is this run's own: its kept table goes with it)
}

(async () => {
  if (process.env.ONLY !== 'game') { const browser = await PAGE.launch({ flags: NO_GPU }); try { await screen(browser); } finally { await PAGE.close(); } }
  if (GPU) { const browser = await PAGE.launch(); try { await game(browser); } finally { await PAGE.close(); } }
  check(!errors.length, 'no page error', errors.slice(0, 6).join(' · ') || 'none');
  await PAGE.end(`=== keybinds-page: ${PAGE.fails ? 'FAILURES above' : 'ALL PASS'}${GPU ? '' : ' (the screen only: GPU=1 adds the game)'} ===`);
})().catch(e => { console.error(e); process.exit(1); });
