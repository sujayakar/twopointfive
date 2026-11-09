// A RUN'S PAGE IS NOBODY'S TRY (src/host/runpage.ts ownPage, asked in src/host/drawpage.ts): headless Chromium on the software WebGPU adapter, one fight stepped with nothing drawn,
// one page at a time. Sam alone throws a frag at his own feet, by real keys and a real click, so the run begins and ends as a player's would. On these addresses:
//   plain     the mission's own: the page raises the end card by itself and the try goes on twopointsix.verdicts
//   benchrun  a benchmark scene's word (no run under way, so a plain mission on a scene's address): the same end, no card, the books and the par best as they were
//   pricerun  a pricing run's word: the same
//   bench     the word that asks for a benchmark, on a tab with no session store (the refused run plays the mission): the same
// `plain` is what makes the others mean something: if it raises no card or moves no store, the stage reached no end, so it always runs first.
//   PORT=<a dev server's port> node tools/qa/runbooks-page.cjs [benchrun pricerun bench]     (no word: plain and benchrun, a minute and a half each)
// exit 0 = all of it with no page error · 1 = otherwise. A loss files no par time, so a par best moving is not held here: the page tells the books one thing, win or loss (RunBooks.tell), behind one gate.
const { chromium } = require(process.env.PLAYWRIGHT_MODULE || 'playwright');
const PORT = process.env.PORT || 5199;
const FLAGS = ['--enable-unsafe-webgpu', '--use-gl=angle', '--use-angle=swiftshader', '--enable-features=Vulkan', '--use-vulkan=swiftshader', '--ignore-gpu-blocklist', '--disable-gpu-watchdog'];
const MISSION = 'paused&quality=verylow&mute&nocutboxes&cloud=0&levelFile=killhouse.json&roomclear&seed=1&squad=0&difficulty=hard';
const ARMS = { plain: { q: '' }, benchrun: { q: '&benchrun=0' }, pricerun: { q: '&pricerun=0' }, bench: { q: '&bench', noSession: true } };
const VERDICTS = 'twopointsix.verdicts', PARBEST = 'twopointsix.parbest';
// a kept record for each store, so "as they were" is a string that was there and not a store nobody opened
const KEPT = { [VERDICTS]: JSON.stringify({ 'killhouse-four|standard': { n: 2, s: 2, w: { quiet: { tries: 1, clean: true, secs: 80, wins: 1 } } } }), [PARBEST]: JSON.stringify({ 'killhouse.json|way1|squad4|standard': 5999 }) };
let red = false;
const check = (ok, what, detail) => { if (!ok) red = true; console.log(`- ${ok ? 'PASS' : 'FAIL'} — ${what}\n    ${detail}`); };
const T0 = Date.now(), log = (...a) => console.log(`[${((Date.now() - T0) / 1000).toFixed(0)}s]`, ...a);
const READ = ([V, P]) => { const d = window.__draw, r = d.game.run();
  return { begun: r.begun, over: r.over, won: r.won, card: d.pause.up, shown: !!document.querySelector('#menu.on .db'), verdicts: localStorage.getItem(V), parbest: localStorage.getItem(P) }; };

(async () => {
  const browser = await chromium.launch({ headless: true, channel: 'chromium', args: FLAGS });
  const asked = process.argv.slice(2), bad = asked.filter(a => !(a in ARMS));
  if (bad.length) { console.log(`no arm '${bad[0]}' (${Object.keys(ARMS).join(' · ')})`); process.exit(1); }
  for (const arm of ['plain', ...(asked.length ? asked.filter(a => a !== 'plain') : ['benchrun'])]) {
    const A = ARMS[arm], ctx = await browser.newContext({ viewport: { width: 640, height: 360 }, deviceScaleFactor: 1 });
    await ctx.addInitScript(([kept, noSession]) => { window.__testVisible = true;
      for (const [k, v] of Object.entries(kept)) if (localStorage.getItem(k) === null) localStorage.setItem(k, v);
      if (noSession) Object.defineProperty(window, 'sessionStorage', { get() { throw new Error('no session store'); } }); }, [KEPT, !!A.noSession]);
    const page = await ctx.newPage(), errors = [];
    page.on('pageerror', x => errors.push(String(x && x.stack || x).slice(0, 300)));
    page.on('console', m => { if (m.type() === 'error' && !/Failed to load resource/.test(m.text())) errors.push(m.text().slice(0, 300)); });
    await page.goto(`http://localhost:${PORT}/?${MISSION}${A.q}`, { timeout: 600000 });
    await page.waitForFunction(() => !!window.__ready, null, { timeout: 600000 });
    await page.evaluate(async () => { await window.__ready; });
    log(arm, 'ready');
    const step = (n, keys = []) => page.evaluate(([n, keys]) => window.__runFrames(n, 1 / 60, keys, 'none'), [n, keys]);
    await step(60, ['KeyD']);   // his first stride begins the try
    let c = await page.evaluate(READ, [VERDICTS, PARBEST]); const begun = c.begun;
    await page.keyboard.down('Digit5'); await step(2); await page.keyboard.up('Digit5'); await step(4);
    for (let k = 0; k < 40 && !c.over; k++) {   // (the frag at his feet, again until he is down: where the cursor falls is the page's to say)
      await page.mouse.move(320, 180); await step(2); await page.mouse.down(); await step(2); await page.mouse.up(); await step(240);
      c = await page.evaluate(READ, [VERDICTS, PARBEST]); }
    await step(600); c = await page.evaluate(READ, [VERDICTS, PARBEST]);
    log(arm, JSON.stringify(c).slice(0, 420));
    check(begun && c.over && !c.won, `${arm}: the try began and the run ended`, `begun ${begun} · over ${c.over} · won ${c.won}`);
    if (arm === 'plain') {
      check(c.card && c.shown, 'plain: the page raised the end card by itself', `card ${c.card} · on screen ${c.shown}`);
      check(c.verdicts !== KEPT[VERDICTS], 'plain: the try is on the books', c.verdicts);
    } else {
      check(!c.card && !c.shown, `${arm}: no end card`, `card ${c.card} · on screen ${c.shown}`);
      check(c.verdicts === KEPT[VERDICTS], `${arm}: the books as they were`, c.verdicts);
      check(c.parbest === KEPT[PARBEST], `${arm}: the par best as it was`, c.parbest);
    }
    check(!errors.length, `${arm}: no page error`, errors.join(' | ') || 'none');
    await ctx.close();
  }
  await browser.close();
  console.log(red ? 'runbooks-page: FAIL' : 'runbooks-page: ALL PASS');
  process.exit(red ? 1 : 0);
})().catch(e => { console.error(e); process.exit(1); });
