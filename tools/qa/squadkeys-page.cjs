// THE CONTROLS LIST SAYS THE SQUAD'S KEYS AS THE PAGE HAS THEM, AND NAMES NO PLAN MODE — ON THE REAL PAGE (src/ui/options.ts squadRows; game/cmdring.ts
// `only`, `goals`). The maintainer, 10-26: "let's remove plan mode and update the tutorials accordingly". As the page opens:
//   · Options ▸ Controls, Squad, is three rows: the command key's ("Order the squad", its tap and its hold), then the sprint key with that key ("Send
//     part of the squad to one more thing"), then the word's ("Send in everyone who's waiting"); no row for the orders menu, its two letters or who
//     takes an order, and no word of the tab names plan mode; a who row given a key by hand says the key does nothing now; a machine that moved
//     sprint to another key reads that key on the part's row, cap and words; a menu given a key by hand has its three rows back, with no middle button;
//   · `?goals=0`: no row for a part of the squad, and the hand-bound who row does not point at one;
//   · `?qonly=0`: the six rows as they were;
//   · the older card (`?cards`): its key list has the command key and the word's for a team, and no "who gets it".
//   node tools/qa/squadkeys-page.cjs      env: PORT (the dev server's, default 5199) · SQUADKEYS_BREAK=1 (goals off on every page: must FAIL)
// Options ▸ Game (the "Plan mode circles" row) needs the game stood up and is not drawn here: tools/qa/squadkeys-check.ts reads its guard.
// PASS / FAIL lines, exit 1 on a FAIL or a page error.
const { chromium } = require('/opt/node-tools/node_modules/playwright');
const PORT = process.env.PORT || 5199, BREAK = process.env.SQUADKEYS_BREAK === '1';
let fails = 0; const check = (ok, label, detail = '') => { if (!ok) fails++; console.log(`- ${ok ? 'PASS' : 'FAIL'} — ${label}${detail ? `\n    ${detail}` : ''}`); };
(async () => {
  const browser = await chromium.launch({ args: ['--disable-gpu'] }), errors = [];
  const fresh = async (store = {}) => { const ctx = await browser.newContext({ viewport: { width: 1440, height: 900 } });
    await ctx.addInitScript(s => { for (const [k, v] of Object.entries(s)) try { if (localStorage.getItem(k) === null) localStorage.setItem(k, v); } catch { /* no store */ } }, store);
    await ctx.addInitScript(() => addEventListener('DOMContentLoaded', () => { const s = document.createElement('style'); s.textContent = '#fatal { display: none !important; }'; document.head.appendChild(s); }));
    const page = await ctx.newPage(); page.on('pageerror', e => { if (/WebGPU|adapter/i.test(String(e))) return; errors.push(String(e).slice(0, 200)); }); return { ctx, page }; };
  const url = (more = '') => `http://localhost:${PORT}/?paused&quality=verylow&mute${BREAK ? '&goals=0' : ''}${more ? '&' + more : ''}`;
  const squad = async page => { await page.waitForSelector('#menu .actions button', { timeout: 60000 });
    await page.evaluate(() => [...document.querySelectorAll('#menu .actions button')].find(b => /options/i.test(b.textContent)).click());
    await page.waitForSelector('#options .op-tab[data-tab="controls"]'); await page.click('#options .op-tab[data-tab="controls"]');
    return page.evaluate(() => { const rows = []; for (const g of document.querySelectorAll('#options .op-kgroup')) { if (!/squad/i.test(g.querySelector('.op-sec')?.textContent ?? '')) continue;
      for (const e of g.querySelectorAll('.op-krow')) { const w = e.querySelector('.op-what'), sm = w.querySelector('small'); rows.push({ caps: [...e.querySelectorAll('.op-caps > *')].map(c => c.textContent).join(' '), what: w.firstChild.textContent, small: sm ? sm.textContent : '' }); } }
      return { rows, plan: /plan mode/i.test(document.querySelector('#options').textContent) }; }); };
  const say = S => S.rows.map(r => `[${r.caps}] ${r.what}`).join(' · ');

  // ── as the page opens ──
  { const { ctx, page } = await fresh(); await page.goto(url(), { waitUntil: 'domcontentloaded' }); const S = await squad(page), [a, b, c] = S.rows;
    check(S.rows.length === 3 && a.caps === 'Q' && a.what === 'Order the squad' && /tap it/.test(a.small) && /wheel/.test(a.small) && b.caps === 'Shift + Q' && /part of the squad/.test(b.what) && /hold Shift and tap Q\. keep Q down/.test(b.small) && c.caps === 'F' && /everyone who's waiting/.test(c.what),
      "Squad is three rows: the command key's, Shift with it for a part of the squad, then the word's", `${say(S)} · "${b ? b.small : ''}"`);
    check(!S.plan && !S.rows.some(r => /orders, on what|who takes/i.test(r.what) || /middle mouse/i.test(r.small)), 'no row for the orders menu or for who takes an order, and no word of the tab names plan mode', `plan mode named: ${S.plan}`);
    await ctx.close(); }
  // ── a who row given a key by hand ──
  { const { ctx, page } = await fresh({ 'twopointsix.keys': '{"KeyT":"KeyT"}' }); await page.goto(url(), { waitUntil: 'domcontentloaded' }); const S = await squad(page), who = S.rows.find(r => /who takes/i.test(r.what));
    check(!!who && who.caps === 'T' && /doesn't do anything now/.test(who.small) && /hold Shift and tap Q/.test(who.small) && !S.plan, 'a who row given a key by hand says the key does nothing now, and names no plan mode', who ? `[${who.caps}] "${who.small}"` : 'the row is NOT LISTED');
    await ctx.close(); }
  // ── sprint moved to another key: the part's row follows it ──
  { const { ctx, page } = await fresh({ 'twopointsix.keys': '{"ShiftLeft":"KeyK"}' }); await page.goto(url(), { waitUntil: 'domcontentloaded' }); const S = await squad(page), b = S.rows[1];
    check(!!b && b.caps === 'K + Q' && /hold K and tap Q/.test(b.small), "a machine that moved sprint to K reads K with the command key on the part's row, cap and words", b ? `[${b.caps}] "${b.small}"` : 'no second row');
    await ctx.close(); }
  // ── the orders menu given a key by hand ──
  { const { ctx, page } = await fresh({ 'twopointsix.keys': '{"KeyX":"KeyX"}' }); await page.goto(url(), { waitUntil: 'domcontentloaded' }); const S = await squad(page), m = S.rows.find(r => /orders, on what/i.test(r.what));
    check(!!m && m.caps === 'X' && !/middle/i.test(m.small) && S.rows.some(r => r.caps === 'H') && S.rows.some(r => r.caps === 'G'), 'a menu given a key by hand has its row and its two letters back, and its words leave the middle button out', `${say(S)} · "${m ? m.small : ''}"`);
    await ctx.close(); }
  // ── goals off: as it was since the whole-squad rule ──
  if (!BREAK) { const { ctx, page } = await fresh({ 'twopointsix.keys': '{"KeyT":"KeyT"}' }); await page.goto(url('goals=0'), { waitUntil: 'domcontentloaded' }); const S = await squad(page), who = S.rows.find(r => /who takes/i.test(r.what));
    check(!S.rows.some(r => /\+/.test(r.caps)) && !!who && /only works in plan mode|every order goes to the whole squad/.test(who.small) && !/Shift/.test(who.small), '?goals=0: no row for a part of the squad, and the hand-bound who row does not point at one (its words as they were with plan mode, or that every order is the whole squad\'s)', say(S));
    await ctx.close(); }
  // ── the older keys: as they were ──
  if (!BREAK) { const { ctx, page } = await fresh(); await page.goto(url('qonly=0'), { waitUntil: 'domcontentloaded' }); const S = await squad(page);
    check(S.rows.length === 6 && S.rows[0].what === 'Orders, on what the cursor is on' && S.rows[1].what === 'The usual order for that spot, at once' && S.rows[2].what === 'Who takes the next order' && S.rows[3].what === 'Go', '?qonly=0: the six rows as they were', say(S));
    await ctx.close(); }
  // ── the older card's key list ──
  { const { ctx, page } = await fresh(); await page.goto(url('cards'), { waitUntil: 'domcontentloaded' }); await page.waitForSelector('#menu .controls .row', { timeout: 60000 });
    const rows = await page.evaluate(() => [...document.querySelectorAll('#menu .controls .row')].map(r => [r.querySelector('.k').textContent, r.querySelector('.v').textContent])), team = rows.find(r => /with a team/.test(r[1]));
    check(!!team && team[0] === 'Q · F' && /tap Q for the usual order/.test(team[1]) && /wheel/.test(team[1]) && /press F to send in everyone/.test(team[1]) && !rows.some(r => /who gets it|with the orders up/i.test(r[1])), "the older card: a team's keys are the command key and the word's, and nothing says who gets it", team ? `[${team[0]}] ${team[1]}` : 'no team row');
    await ctx.close(); }
  await browser.close();
  check(errors.length === 0, 'no page error', errors.join(' | ') || 'none');
  console.log(fails ? `\n=== squadkeys-page: ${fails} FAILED ===` : '\n=== squadkeys-page: ALL PASS ===');
  process.exit(fails ? 1 : 0);
})().catch(e => { console.error(e); process.exit(1); });
