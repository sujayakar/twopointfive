// A RUN'S PAGE IS NOBODY'S TRY (src/main.ts `offBooks`, src/host/runpage.ts ownPage): headless Chromium on the software WebGPU adapter, a fight played on the page's
// own loop with the renderer stubbed, one page at a time, on these addresses:
//   plain          the kill house won on the mission's own address: the page raises the end card by itself, the try goes on twopointsix.verdicts, the time on twopointsix.parbest
//   plain-down     Sam alone, shot down by a defender he is stood beside: the card by itself (the game's own EndCard event, not the verdict's beat), the try on the books
//   benchrun       a benchmark scene's word (no run under way, so the page is a plain mission page on a scene's address): the same win, no card, both stores as they were
//   benchrun-down  … and Sam down there: no card, the books as they were
//   pricerun       a pricing run's word: the win, as benchrun
//   bench          the word that asks for a benchmark, on a tab with no session store (the refused run plays the mission): the win, as benchrun
// The two plain arms are what make the others mean something: if one raises no card or moves no store, the stage reached no end, so an arm asked for by name brings its plain arm first.
//   PORT=<a dev server's port> node tools/qa/runbooks-page.cjs [arm …]
// exit 0 = all of it with no page error · 1 = otherwise. The stealth game's own way out is not staged here (a long walk); it ends by the same event as plain-down.
const { chromium } = require(process.env.PLAYWRIGHT_MODULE || 'playwright');
const PORT = process.env.PORT || 5199;
const FLAGS = ['--enable-unsafe-webgpu', '--use-gl=angle', '--use-angle=swiftshader', '--enable-features=Vulkan', '--use-vulkan=swiftshader', '--ignore-gpu-blocklist', '--disable-gpu-watchdog'];
const MISSION = 'paused&quality=verylow&mute&nocutboxes&cloud=0&levelFile=killhouse.json&roomclear&seed=1', WIN = '&squad4&difficulty=standard', DOWN = '&squad=0&difficulty=hard';
const ARMS = { plain: { q: WIN, end: 'win', control: true }, 'plain-down': { q: DOWN, end: 'down', control: true }, benchrun: { q: WIN + '&benchrun=0', end: 'win' }, 'benchrun-down': { q: DOWN + '&benchrun=0', end: 'down' }, pricerun: { q: WIN + '&pricerun=0', end: 'win' }, bench: { q: WIN + '&bench', end: 'win', noSession: true } };
const VERDICTS = 'twopointsix.verdicts', PARBEST = 'twopointsix.parbest';
// a kept record for each store, so "as they were" is a string that was there and not a store nobody opened
const KEPT = { [VERDICTS]: JSON.stringify({ 'killhouse-four|standard': { n: 2, s: 2, w: { quiet: { tries: 1, clean: true, secs: 80, wins: 1 } } } }), [PARBEST]: JSON.stringify({ 'killhouse.json|way1|squad4|standard': 5999 }) };
let red = false;
const check = (ok, what, detail) => { if (!ok) red = true; console.log(`- ${ok ? 'PASS' : 'FAIL'} — ${what}\n    ${detail}`); };
const T0 = Date.now(), log = (...a) => console.log(`[${((Date.now() - T0) / 1000).toFixed(0)}s]`, ...a);

// in the page: the walk that starts a try, then the end asked for, then the frames until its card's beat is past
const PLAY = (end) => { const e = window.__engine, real = e.render, g = window.__game; e.render = () => {}; const step = (n, keys) => window.__runFrames(n, 1 / 60, keys || []);
  const foes = () => g.guards.map((x, i) => [x, i]).filter(([x]) => x.team !== 'operative' && x.state !== 'dead').map(([, i]) => i);
  try { window.__smoke && (window.__smoke.params.enabled = false); step(30); step(150, ['KeyD']); step(60, ['KeyA']);
    const started = g.verdict.started;
    if (end === 'win') { for (let k = 0; k < 4; k++) { const f = foes(); if (f.length) window.__killGuard(f[0]); step(70, ['KeyW']); } for (const i of foes()) window.__killGuard(i); }
    else for (let k = 0; k < 120 && !g.player.down; k++) { const f = g.guards[foes()[0]], p = f.char.pos; g.teleportPlayer([p[0] + 1.2, 0, p[2]]); step(30); }
    step(600);
    return started; } finally { e.render = real; } };
const READ = ([V, P]) => { const m = document.getElementById('menu'), g = window.__game, R = g.verdict.result;
  return { word: R ? R.word : null, won: !!R && R.won, down: !!g.player.down, card: !!m.querySelector('.db'), mode: window.__menu.mode, verdicts: localStorage.getItem(V), parbest: localStorage.getItem(P), left: g.guards.filter(x => x.team !== 'operative' && x.state !== 'dead').length }; };

(async () => {
  const browser = await chromium.launch({ headless: true, channel: 'chromium', args: FLAGS });
  const asked = process.argv.slice(2), bad = asked.filter(a => !(a in ARMS));
  if (bad.length) { console.log(`no arm '${bad[0]}' (${Object.keys(ARMS).join(' · ')})`); process.exit(1); }
  const named = asked.length ? asked : Object.keys(ARMS), want = Object.keys(ARMS).filter(a => named.includes(a) || (ARMS[a].control && named.some(n => ARMS[n].end === ARMS[a].end)));
  for (const arm of want) {
    const A = ARMS[arm];
    const ctx = await browser.newContext({ viewport: { width: 640, height: 360 }, deviceScaleFactor: 1 });
    await ctx.addInitScript(([kept, noSession]) => { window.__testVisible = true;
      for (const [k, v] of Object.entries(kept)) if (localStorage.getItem(k) === null) localStorage.setItem(k, v);
      if (noSession) Object.defineProperty(window, 'sessionStorage', { get() { throw new Error('no session store'); } }); }, [KEPT, !!A.noSession]);
    const page = await ctx.newPage(), errors = [];
    page.on('pageerror', x => errors.push(String(x && x.stack || x).slice(0, 300)));
    // (__killGuard is a TypeScript body on the puppet: the items a dead man drops are refused by the scan and said here. That is the stage's own noise, on every address alike, so it is counted and not a page error)
    let stageNoise = 0; page.on('console', m => { if (m.type() !== 'error' || /Failed to load resource/.test(m.text())) return; if (/^the scan: a write on the puppet was NOT SENT/.test(m.text())) stageNoise++; else errors.push(m.text().slice(0, 300)); });
    await page.goto(`http://localhost:${PORT}/?${MISSION}${A.q}`, { timeout: 1200000 });
    await page.waitForFunction(() => !!window.__ready, null, { timeout: 1200000 });
    await page.evaluate(async () => { await window.__ready; window.__menu.close(); });
    log(arm, 'ready');
    const started = await page.evaluate(PLAY, A.end), c = await page.evaluate(READ, [VERDICTS, PARBEST]);
    log(arm, JSON.stringify(c).slice(0, 500));
    if (A.end === 'win') check(started && c.word && c.won && c.left === 0, `${arm}: the try started and the fight ended won`, `started ${started} · ${c.word} · defenders up ${c.left}`);
    else check(started && c.down && c.word && !c.won, `${arm}: the try started and Sam went down`, `started ${started} · down ${c.down} · ${c.word}`);
    if (A.control) {
      check(c.card && c.mode === 'debrief', `${arm}: the page raised the end card by itself`, `card ${c.card} · menu ${c.mode}`);
      check(c.verdicts !== KEPT[VERDICTS], `${arm}: the try is on the books`, c.verdicts);
      if (A.end === 'win') check(c.parbest !== KEPT[PARBEST], `${arm}: the time is on the par best`, c.parbest);
    } else {
      check(!c.card && c.mode !== 'debrief', `${arm}: no end card`, `card ${c.card} · menu ${c.mode}`);
      check(c.verdicts === KEPT[VERDICTS], `${arm}: the books as they were`, c.verdicts);
      check(c.parbest === KEPT[PARBEST], `${arm}: the par best as it was`, c.parbest);
    }
    check(!errors.length, `${arm}: no page error`, `${errors.join(' | ') || 'none'} · the stage's refused writes ${stageNoise}`);
    await ctx.close();
  }
  await browser.close();
  console.log(red ? 'runbooks-page: FAIL' : 'runbooks-page: ALL PASS');
  process.exit(red ? 1 : 0);
})().catch(e => { console.error(e); process.exit(1); });
