// THE KIT STEP'S BRIEF ON THE REAL PAGE (ui/kitBrief.ts `?kitbrief=0`, ui/kitGuide.ts `?kitguide=0`). The maintainer, 10-22: "we should teach how to pick squad layouts! so maybe a tutorial on the
// gear page for a mission (that's skippable). honestly it doesn't make a ton of sense right now since you don't know anything about the objective or map
// when picking kit" — and, 10-23, of the walkthrough that came up on every mission: "this tutorial is good but we should attach it to a tutorial mission
// (and make it bespoke and explain how to plan), not for the regular ones".
// Headless Chromium with no WebGPU adapter (the start page is plain DOM, live from the first tick), driven with REAL keys and clicks:
//   · the Team & kit step shows the mission's job, its facts, what is set and its map, and what the place asks of a kit (a rescue: hostages, locked doors);
//   · M makes the map large and puts it back;
//   · A REGULAR MISSION plays no walkthrough and has no word for one;
//   · THE SQUAD TUTORIAL MISSION shows its cards, and a browser that has never been there is walked through planning its kit, one card beside one lit
//     row at a time, each card gone only when its own thing is DONE (the map opened and put back, with the street's rooms named on it · Stun heavy put
//     on · a frag back on your card · one of the four looked at) — and not on anything else; no card writes a digit;
//   · the card never covers the row it is about and never leaves the window;
//   · finished, it is remembered under its own key and not shown on the next visit; the word on the step's head plays it again; Esc skips it and the
//     next Esc is the step's own; Enter under a card is its Next and never Deploy;
//   · `?kitpack=0` = that mission as issued again (no cards, no plan) · `?kitguide=0` = no walkthrough anywhere · `?kitguide=1` = the step's general
//     walkthrough on a regular mission, kept for comparing.
//   node tools/qa/kitguide-page.cjs [outDir]      env: PORT (the dev server's, default 5199) · SIZES · KITGUIDE_BREAK=1 (the walkthrough and the brief off, the step as it was: must FAIL)
// PASS / FAIL lines, exit 1 on a FAIL or a page error. Pictures of every beat go to outDir.
const { chromium } = require('/opt/node-tools/node_modules/playwright');
const fs = require('fs');
const path = require('path');
const outDir = process.argv[2] || '';
const PORT = process.env.PORT || 5199;
const SIZES = (process.env.SIZES || '1440x900,1353x770,1000x673').split(',').map(s => s.split('x').map(Number));
const BREAK = process.env.KITGUIDE_BREAK === '1';
const OFF = BREAK ? '&kitguide=0&kitbrief=0' : '';
let fails = 0; const errors = [];
const check = (ok, label, detail) => { if (!ok) fails++; console.log(`- ${ok ? 'PASS' : 'FAIL'} — ${label}\n    ${detail}`); };
if (outDir) fs.mkdirSync(outDir, { recursive: true });

/** what the step shows right now, read off the page */
const read = page => page.evaluate(() => {
  const root = document.querySelector('#missions'), vis = e => !!e && e.offsetParent !== null && getComputedStyle(e).visibility !== 'hidden';
  const box = e => { if (!e) return null; const r = e.getBoundingClientRect(); return { l: r.left, t: r.top, r: r.right, b: r.bottom, w: r.width, h: r.height }; };
  const card = document.querySelector('.kg-card'), lit = document.querySelector('.kg-lit'), map = document.querySelector('.msheet > .mmap');
  const txt = sel => { const e = document.querySelector(sel); return vis(e) ? e.textContent.trim() : ''; };
  return { cls: root.className, pane: root.classList.contains('pane-loadout') ? 'kit' : 'mission',
    card: vis(card) ? { title: card.querySelector('h3').firstChild.textContent, text: [...card.querySelectorAll('p')].map(p => p.textContent).join(' '), box: box(card), pips: card.querySelectorAll('.kg-pips i').length, tick: card.classList.contains('tick') } : null,
    lit: lit ? (lit.getAttribute('data-row') || 'map') : '', litBox: box(lit), sel: document.querySelector('#loadout [data-row].sel')?.getAttribute('data-row') ?? '',
    map: vis(map) ? box(map) : null, big: root.classList.contains('kb-big'), blurb: txt('.mblurb'), csum: txt('.mcsum'), cap: txt('.kb-cap'), kitTop: document.querySelector('.onehost')?.getBoundingClientRect().top ?? 0,
    facts: [...document.querySelectorAll('.mfacts .mfact')].filter(vis).map(f => `${f.querySelector('i').textContent} ${f.querySelector('b').textContent}`),
    again: vis(document.querySelector('.kg-again')), squad: document.querySelector('#loadout .lo-size.on')?.textContent.trim() ?? '', kit: document.querySelector('#loadout .lo-set.on')?.getAttribute('data-kit') ?? '',
    man: document.querySelector('#loadout .lo-slot.pick .lo-name')?.textContent ?? '', seen: localStorage.getItem('twopointsix.kitguide'), planSeen: localStorage.getItem('twopointsix.kitplan.tut-squad'), win: { w: innerWidth, h: innerHeight },
    beat: vis(card) ? card.getAttribute('data-beat') : '', word: vis(document.querySelector('.kg-again')) ? document.querySelector('.kg-again').textContent : '', cards: vis(document.querySelector('#loadout .lo-kit .kc')),
    pins: [...document.querySelectorAll('.kb-pin')].filter(e => vis(e.parentElement)).map(e => ({ label: e.getAttribute('data-pin'), box: box(e.querySelector('b')), at: box(e) })),
    pool: vis(document.querySelector('#loadout .lo-pool')) ? { text: document.querySelector('#loadout .lo-pool').textContent, box: box(document.querySelector('#loadout .lo-pool')) } : null,
    frags: document.querySelector('#loadout .lo-kit .kc-row[data-field="frags"] .kc-val')?.textContent ?? '', smoke: document.querySelector('#loadout .lo-kit .kc-row[data-field="smoke"] .kc-val')?.textContent ?? '' };
});
const apart = (a, b) => !a || !b || a.r <= b.l + 0.5 || b.r <= a.l + 0.5 || a.b <= b.t + 0.5 || b.b <= a.t + 0.5;
const inside = (a, w) => !!a && a.l >= -0.5 && a.t >= -0.5 && a.r <= w.w + 0.5 && a.b <= w.h + 0.5;

(async () => {
  const browser = await chromium.launch({ headless: true, args: ['--disable-gpu'] });
  const fresh = async (W, H) => { const ctx = await browser.newContext({ viewport: { width: W, height: H } });
    await ctx.addInitScript(() => { try { Object.defineProperty(Navigator.prototype, 'webdriver', { get: () => false }); } catch { /* as it is */ } });   // (this browser stands for a player's: the walkthrough is not played unasked in one a script drives — ui/kitGuide.ts)
    await ctx.addInitScript(() => addEventListener('DOMContentLoaded', () => { const s = document.createElement('style'); s.textContent = '#fatal { display: none !important; }'; document.head.appendChild(s); }));
    const page = await ctx.newPage(); page.on('pageerror', e => { if (/WebGPU|adapter/i.test(String(e))) return; errors.push(String(e).slice(0, 200)); });
    return { ctx, page }; };
  const open = async (page, q) => { await page.goto(`http://localhost:${PORT}/?${q}&paused&mute${OFF}`, { waitUntil: 'domcontentloaded', timeout: 180000 });
    await page.waitForSelector('#missions.one.on #loadout .lo-slot.in', { timeout: 60000 }); await page.waitForTimeout(1200); };
  const shot = async (page, name) => { if (outDir) await page.screenshot({ path: path.join(outDir, `${name}${BREAK ? '-break' : ''}.png`) }).catch(() => {}); };

  for (const [W, H] of SIZES) {
    const at = `${W}x${H}`, { ctx, page } = await fresh(W, H);
    // ── the brief, on a rescue: the job, the map, what the place asks ──
    await open(page, 'loadout=rescue-registry&kitguide=0');
    let s = await read(page);
    const short = H <= 800, kitTop = s.kitTop;   // (a short window: the band is no taller than the name alone was — the map a small mark, the job whole on the large sheet)
    check(!!s.map && s.map.w >= (short ? 40 : 120) && (short || s.blurb.length > 20) && s.facts.length >= 1, `${at}: the kit step shows the mission's map${short ? '' : ', its job'} and its facts`, `map ${s.map ? `${s.map.w.toFixed(0)} px wide` : 'not drawn'} · "${s.blurb.slice(0, 60)}…" · ${s.facts.join(' | ') || 'no facts'}`);
    check(s.facts.some(f => /^Hostages 2$/i.test(f)) && s.facts.some(f => /^Locked doors 4$/i.test(f)), `${at}: the registry's own counts are on it: two hostages, four locked doors`, s.facts.join(' | ') || 'none');
    if (!short) check(s.csum.length > 0, `${at}: what is set for the mission is on it`, s.csum || 'nothing');
    check(!s.card && !s.again, `${at}: ?kitguide=0 = no card and no word for the walkthrough`, `card ${s.card ? s.card.title : 'none'} · word ${s.again}`);
    await page.keyboard.press('m'); await page.waitForTimeout(350); const big = await read(page); await shot(page, `map-large-${at}`);
    await page.keyboard.press('m'); await page.waitForTimeout(350); const small = await read(page);
    check(big.big && big.map && s.map && big.map.w > s.map.w * 2 && inside(big.map, big.win) && !small.big && big.cap.length > 30, `${at}: M makes the map large with the job under it, M again puts it back`, `${s.map ? s.map.w.toFixed(0) : '—'} px → ${big.map ? big.map.w.toFixed(0) : '—'} px → ${small.map ? small.map.w.toFixed(0) : '—'} px`);
    await shot(page, `brief-${at}`);
    // ── A REGULAR MISSION, in a browser that has never been here: no walkthrough, no word for one ──
    await open(page, 'loadout=compound-four');
    s = await read(page);
    check(!s.card && !s.again && s.cards && s.seen === null, `${at}: a regular mission plays no walkthrough and has no word for one`, `card ${s.card ? s.card.title : 'none'} · word ${s.again} · its kit cards shown ${s.cards}`);
    let g;
    // …the step's general walkthrough, kept for comparing: asked for on the address of a regular mission
    await open(page, 'loadout=compound-four&kitguide=1&squad=2'); g = await read(page);
    check(!!g.card && g.card.title === 'The job' && /2/.test(g.squad), `${at}: ?kitguide=1 on a regular mission plays the step's general walkthrough (kept for comparing), at the team size the address names`, `card "${g.card ? g.card.title : 'none'}" · the step opens at '${g.squad}'`);
    await page.keyboard.press('Enter'); await page.waitForTimeout(450); g = await read(page);
    check(g.pane === 'kit' && !!g.card && g.card.title === 'Who comes', `${at}: …its Enter is its Next too`, `on ${g.pane} · card "${g.card ? g.card.title : 'none'}"`);
    await page.keyboard.press('a'); await page.waitForTimeout(1100); g = await read(page);
    check(!!g.card && g.card.title === 'A kit for the squad', `${at}: …and another team size picked is its second card done`, `card "${g.card ? g.card.title : 'none'}"`);
    await page.keyboard.press('Escape'); await page.waitForTimeout(300); g = await read(page);
    check(!g.card && g.word === 'How to pick' && g.seen === 'done', `${at}: …skipped, it leaves its own word on the head`, `word '${g.word}' · store ${g.seen}`);
    // ── the brief's counts on other missions ──
    await open(page, 'loadout=hostage-rescue&kitguide=0'); g = await read(page);
    check(g.facts.some(f => /^Hostages 2$/i.test(f)) && g.facts.some(f => /^Locked doors 4$/i.test(f)), `${at}: the office's rescue: two hostages, and four locked doors (its staging unlocks the side door)`, g.facts.join(' | ') || 'none');
    await open(page, 'loadout=warehouse-four&kitguide=0'); g = await read(page);
    check(!g.facts.some(f => /[6-9]|\d\d/.test(f)), `${at}: no digit above 5 among the facts (a count past five is written out; the Defenders cell is not drawn: ui/missions.ts MYSTERY)`, g.facts.join(' | ') || 'none');
    if (short) { await open(page, 'loadout=rescue-registry&kitbrief=0&kitguide=0'); const was = await read(page);
      check(kitTop <= was.kitTop + 0.5, `${at}: a short window: the brief takes no height from the kit (it starts where it did under the name alone)`, `the kit starts at ${kitTop.toFixed(0)} px with the brief, ${was.kitTop.toFixed(0)} px without`); }
    await ctx.close();
  }
  await browser.close();
  check(errors.length === 0, 'no page error', errors.join(' | ') || 'none');
  console.log(fails ? `\n=== kitguide-page: ${fails} FAILED ===` : '\n=== kitguide-page: ALL PASS ===');
  process.exit(fails ? 1 : 0);
})().catch(e => { console.error(e); process.exit(2); });
