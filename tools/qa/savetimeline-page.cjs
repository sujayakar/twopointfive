// THE SAVE TIMELINE ON THE REAL PAGE, WITH REAL KEYS: headless Chromium on the software WebGPU adapter. F5 (a keydown kept from the browser) three times a few
// seconds apart; Escape's card shows three tiles in the mission clock's order; a click on the middle one closes the card and the game stands at that save's clock; the
// game then plays on. F5 with the card up writes nothing at once: the tile to come is shown, and the first tick after the card writes it, once.
//   PORT=5199 node tools/qa/savetimeline-page.cjs     (a dev server on that port)     exit 0 = every line · 1 = a FAIL or a page error
// No frame is drawn (the renderer is stubbed): the tiles' pictures and what is drawn after a load are not asked here.
const PAGE = require('./page.cjs').page({ name: 'savetimeline-page', detail: 'json' }), { check } = PAGE;
const QUALITY = process.env.QUALITY || 'verylow';

// in the page
const F5 = () => { const e = new KeyboardEvent('keydown', { code: 'F5', key: 'F5', bubbles: true, cancelable: true }); window.dispatchEvent(e); window.__draw.run(1, [], 'last'); window.dispatchEvent(new KeyboardEvent('keyup', { code: 'F5', key: 'F5', bubbles: true })); window.__draw.run(2, [], 'last'); return e.defaultPrevented; };
const SEEN = () => { const s = window.__draw.game.saves(); return { rows: s.rows.map(r => ({ key: r.key, slot: r.slot, t: +r.missionT.toFixed(2) })), elapsed: s.elapsed, up: window.__draw.pause.up, tiles: [...document.querySelectorAll('#menu .saves .tile')].map(t => ({ save: t.dataset.save ?? null, due: t.classList.contains('due'), age: t.querySelector('.age')?.textContent ?? '' })) }; };
const ESC = () => { window.dispatchEvent(new KeyboardEvent('keydown', { code: 'Escape', key: 'Escape', bubbles: true, cancelable: true })); };

(async () => {
  const { page, errors, go, step } = await PAGE.open({ timeout: 0, errors: { console: false } });
  await go(`levelFile=depot.json&roomclear&squad4&paused&quality=${QUALITY}&mute&seed=7`);
  await page.evaluate(async () => { await window.__ready; window.__engine.render = () => {}; window.__draw.run(60, [], 'last'); });
  const kept = []; for (let i = 0; i < 3; i++) { kept.push(await page.evaluate(F5)); await step(240); }
  const a = await page.evaluate(SEEN), hand = a.rows.filter(r => r.slot === 'manual');
  check(kept.every(x => x) && hand.length === 3 && hand[0].t < hand[1].t && hand[1].t < hand[2].t && a.elapsed > hand[2].t, 'F5 is kept from the browser and saves by hand, three times at three clocks', { kept, rows: a.rows, elapsed: a.elapsed });
  await page.evaluate(ESC); const b = await page.evaluate(SEEN), order = b.tiles.filter(t => t.save !== null);
  check(b.up && order.length === b.rows.length && order.every((t, i) => i === 0 || b.rows.findIndex(r => r.key === order[i - 1].save.split('@')[0]) < b.rows.findIndex(r => r.key === t.save.split('@')[0])), 'the pause card shows a tile a save, in the clock\'s order', { up: b.up, tiles: b.tiles });
  const mid = hand[1];
  await page.evaluate(key => { [...document.querySelectorAll('#menu .saves .tile')].find(t => (t.dataset.save ?? '').split('@')[0] === key)?.click(); }, mid ? mid.key : ''); await step(2);
  const c = await page.evaluate(SEEN);
  check(!!mid && !c.up && Math.abs(c.elapsed - mid.t) < 0.2, 'a click on the middle tile closes the card and the game stands at that save', { elapsed: c.elapsed, at: mid && mid.t });
  await step(120); const d = await page.evaluate(SEEN);
  check(d.elapsed > c.elapsed + 1.5, 'and it plays on', { from: c.elapsed, to: d.elapsed });
  await page.evaluate(ESC); const e0 = await page.evaluate(SEEN); await page.evaluate(() => { const e = new KeyboardEvent('keydown', { code: 'F5', key: 'F5', bubbles: true, cancelable: true }); window.dispatchEvent(e); window.dispatchEvent(new KeyboardEvent('keyup', { code: 'F5', key: 'F5', bubbles: true })); });
  const e1 = await page.evaluate(SEEN); await page.evaluate(ESC); await step(3); const e2 = await page.evaluate(SEEN);
  check(e0.up && e1.rows.length === e0.rows.length && e1.tiles.some(t => t.due) && !e2.up && e2.rows.filter(r => r.slot === 'manual').length === e0.rows.filter(r => r.slot === 'manual').length + 1, 'F5 with the card up shows the tile to come and writes it on the first tick after, once', { before: e0.rows.length, held: e1.rows.length, tiles: e1.tiles, after: e2.rows.length });
  check(!errors.length, 'no page error', errors.slice(0, 4));
  await PAGE.end();
})().catch(e => { console.error(e); process.exit(2); });
