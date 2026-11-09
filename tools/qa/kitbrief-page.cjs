// THE KIT STEP'S BRIEF ON THE REAL PAGE (ui/kitBrief.ts `?kitbrief=0`). The maintainer, 10-22: "honestly it doesn't make a ton of sense right now since you don't know anything about the
// objective or map when picking kit".
// Headless Chromium with no WebGPU adapter (the start page is plain DOM, live from the first tick), driven with REAL keys and clicks:
//   · the Team & kit step shows the mission's job, its facts, what is set and its map, and what the place asks of a kit (a rescue: hostages, locked doors);
//   · M makes the map large and puts it back;
//   · no digit above 5 among the facts, and on a short window the brief takes no height from the kit.
//   node tools/qa/kitbrief-page.cjs [outDir]      env: PORT (the dev server's, default 5199) · SIZES · KITBRIEF_BREAK=1 (the brief off, the step as it was: must FAIL)
// PASS / FAIL lines, exit 1 on a FAIL or a page error. Pictures go to outDir.
const fs = require('fs');
const path = require('path');
const outDir = process.argv[2] || '';
const PAGE = require('./page.cjs').page({ name: 'kitbrief-page', gpu: false }), { check } = PAGE;
const SIZES = (process.env.SIZES || '1440x900,1353x770,1000x673').split(',').map(s => s.split('x').map(Number));
const BREAK = process.env.KITBRIEF_BREAK === '1';
const OFF = BREAK ? '&kitbrief=0' : '';
const errors = [];
if (outDir) fs.mkdirSync(outDir, { recursive: true });

/** what the step shows right now, read off the page */
const read = page => page.evaluate(() => {
  const root = document.querySelector('#missions'), vis = e => !!e && e.offsetParent !== null && getComputedStyle(e).visibility !== 'hidden';
  const box = e => { if (!e) return null; const r = e.getBoundingClientRect(); return { l: r.left, t: r.top, r: r.right, b: r.bottom, w: r.width, h: r.height }; };
  const map = document.querySelector('.msheet > .mmap');
  const txt = sel => { const e = document.querySelector(sel); return vis(e) ? e.textContent.trim() : ''; };
  return { cls: root.className, pane: root.classList.contains('pane-loadout') ? 'kit' : 'mission',
    map: vis(map) ? box(map) : null, big: root.classList.contains('kb-big'), blurb: txt('.mblurb'), csum: txt('.mcsum'), cap: txt('.kb-cap'), kitTop: document.querySelector('.onehost')?.getBoundingClientRect().top ?? 0,
    facts: [...document.querySelectorAll('.mfacts .mfact')].filter(vis).map(f => `${f.querySelector('i').textContent} ${f.querySelector('b').textContent}`),
    win: { w: innerWidth, h: innerHeight } };
});
const inside = (a, w) => !!a && a.l >= -0.5 && a.t >= -0.5 && a.r <= w.w + 0.5 && a.b <= w.h + 0.5;

(async () => {
  const browser = await PAGE.launch();
  const fresh = async (W, H) => { const { context: ctx, page } = await PAGE.open({ browser, size: [W, H], fatalOff: true, errors: { into: errors, console: false, thrown: /WebGPU|adapter/i } });
    return { ctx, page }; };
  const open = async (page, q) => { await page.goto(PAGE.url(`${q}&paused&mute${OFF}`), { waitUntil: 'domcontentloaded', timeout: 180000 });
    await page.waitForSelector('#missions.one.on #loadout .lo-slot.in', { timeout: 60000 }); await page.waitForTimeout(1200); };
  const shot = async (page, name) => { if (outDir) await page.screenshot({ path: path.join(outDir, `${name}${BREAK ? '-break' : ''}.png`) }).catch(() => {}); };

  for (const [W, H] of SIZES) {
    const at = `${W}x${H}`, { ctx, page } = await fresh(W, H);
    // ── the brief, on a rescue: the job, the map, what the place asks ──
    await open(page, 'loadout=rescue-registry');
    let s = await read(page);
    const short = H <= 800, kitTop = s.kitTop;   // (a short window: the band is no taller than the name alone was — the map a small mark, the job whole on the large sheet)
    check(!!s.map && s.map.w >= (short ? 40 : 120) && (short || s.blurb.length > 20) && s.facts.length >= 1, `${at}: the kit step shows the mission's map${short ? '' : ', its job'} and its facts`, `map ${s.map ? `${s.map.w.toFixed(0)} px wide` : 'not drawn'} · "${s.blurb.slice(0, 60)}…" · ${s.facts.join(' | ') || 'no facts'}`);
    check(s.facts.some(f => /^Hostages 2$/i.test(f)) && s.facts.some(f => /^Locked doors 4$/i.test(f)), `${at}: the registry's own counts are on it: two hostages, four locked doors`, s.facts.join(' | ') || 'none');
    if (!short) check(s.csum.length > 0, `${at}: what is set for the mission is on it`, s.csum || 'nothing');
    await page.keyboard.press('m'); await page.waitForTimeout(350); const big = await read(page); await shot(page, `map-large-${at}`);
    await page.keyboard.press('m'); await page.waitForTimeout(350); const small = await read(page);
    check(big.big && big.map && s.map && big.map.w > s.map.w * 2 && inside(big.map, big.win) && !small.big && big.cap.length > 30, `${at}: M makes the map large with the job under it, M again puts it back`, `${s.map ? s.map.w.toFixed(0) : '—'} px → ${big.map ? big.map.w.toFixed(0) : '—'} px → ${small.map ? small.map.w.toFixed(0) : '—'} px`);
    await shot(page, `brief-${at}`);
    let g;
    // ── the brief's counts on other missions ──
    await open(page, 'loadout=hostage-rescue'); g = await read(page);
    check(g.facts.some(f => /^Hostages 2$/i.test(f)) && g.facts.some(f => /^Locked doors 4$/i.test(f)), `${at}: the office's rescue: two hostages, and four locked doors (its staging unlocks the side door)`, g.facts.join(' | ') || 'none');
    await open(page, 'loadout=warehouse-four'); g = await read(page);
    check(!g.facts.some(f => /[6-9]|\d\d/.test(f)), `${at}: no digit above 5 among the facts (a count past five is written out; the Defenders cell is not drawn)`, g.facts.join(' | ') || 'none');
    if (short) { await open(page, 'loadout=rescue-registry&kitbrief=0'); const was = await read(page);
      check(kitTop <= was.kitTop + 0.5, `${at}: a short window: the brief takes no height from the kit (it starts where it did under the name alone)`, `the kit starts at ${kitTop.toFixed(0)} px with the brief, ${was.kitTop.toFixed(0)} px without`); }
    await ctx.close();
  }
  check(errors.length === 0, 'no page error', errors.join(' | ') || 'none');
  await PAGE.end();
})().catch(e => { console.error(e); process.exit(2); });
