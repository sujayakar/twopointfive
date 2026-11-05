// WHAT IS PAINTED ON A WALL GOES WITH THE WALL, ON THE REAL PAGE (src/game/wallskin.ts WALLSKIN): headless Chromium on the software WebGPU adapter
// (tools/qa/page-look.cjs's recipe), a dressed level's room clear as the page deals it, and the heart of tools/qa/wallskin-check.ts done with the page's own
// Game and its own box world: the partition with the most painted band from the floor found, a blast set against it, and the band's face sampled against
// what the world rasterises (BoxWorld.visible after the page's own upload).
//   PORT=5199 node tools/qa/wallskin-page.cjs          (a dev server on that port: PORT=5199 bun run tools/serve.ts)
//   on    the page as it opens: the switch reads on, the wall holes, no point of its bands is drawn over an open cell and none over standing wall is lost
//   off   the same page under ?wallskin=0: the switch reads off, nothing is cut, and the band is wrong the way it was: on a level in the look's paints the
//         page's own scene/lookTidy.ts takes the wall's whole band off its standing length; on any other level it hangs across the hole
//   STILLS=<dir>  keeps frames read back from the page, both runs: before the blast and two seconds after it (software adapter, verylow; about five minutes a
//                 frame there) · FAR=1  each from the wall's far side too (the blast's side is often the dark one)
//   LEVEL=<file>  the level (default block-gwot.json) · EXTRA=<query> more of the query string for both runs (EXTRA='hour=midday')
//   PLAYWRIGHT=<dir>  where playwright (or playwright-core) is, if not /opt/node-tools/node_modules/playwright · CHROMIUM=<path> a browser to run
// exit 0 = both, with no page error · 1 = otherwise. The verdict is the box lists only: nothing here judges a picture, and the software adapter is not a GPU.
const { chromium } = require(process.env.PLAYWRIGHT || '/opt/node-tools/node_modules/playwright');
const fs = require('fs'), path = require('path');
const PORT = process.env.PORT || 5199, QUALITY = process.env.QUALITY || 'verylow', STILLS = process.env.STILLS || '', EXTRA = process.env.EXTRA ? `${process.env.EXTRA}&` : '', LEVEL = process.env.LEVEL || 'block-gwot.json', FAR = process.env.FAR === '1';
const FLAGS = ['--enable-unsafe-webgpu', '--use-gl=angle', '--use-angle=swiftshader', '--enable-features=Vulkan', '--use-vulkan=swiftshader', '--ignore-gpu-blocklist', '--disable-gpu-watchdog'];
const T0 = Date.now(), log = (...a) => console.log(`[${((Date.now() - T0) / 1000).toFixed(0)}s]`, ...a);

// in the page: the game stepped with no frame drawn (the box world still packs: its list of what it draws is the page's own)
const STEP = (n) => { const g = window.__game, e = window.__engine, real = e.render; e.render = () => { e.world.upload(); }; try { for (let i = 0; i < n; i++) { g.player.visibility = 0.05; window.__runFrames(1, 1 / 60); } } finally { e.render = real; } };
// in the page: the partition with the most band from the floor on its faces (drawn-only, thin, on the face), and where a blast goes against it
const FIND = async () => {
  await window.__ready;
  const g = window.__game, boxes = g.col.boxes, walls = g.destr.walls; g.godMode = true; g.tune.chaos = 0.5;
  let pick = null;
  for (const B of walls.all()) { if (B.kind !== 'wall') continue; const b = B.box, ax = b.h[0] >= b.h[2], u = ax ? 0 : 2, n = 2 - u; if (2 * b.h[u] < 4) continue;
    const bands = [];
    for (let j = 0; j < walls.authored; j++) { const s = boxes[j]; if ((s.flags & 11) !== 2 || s.c[1] - s.h[1] > 0.06 || 2 * s.h[n] > 0.02 || s.h[u] < 0.5 || (s.marks && s.marks.length)) continue;   // (plain bands: a marked one stands as the whole box under a cut record, which this oracle does not read; wallskin-check.ts's does)
      const gap = Math.max(s.c[n] - s.h[n] - (b.c[n] + b.h[n]), b.c[n] - b.h[n] - (s.c[n] + s.h[n])); if (gap > 0.01 || gap < -0.02) continue;
      if (Math.min(s.c[u] + s.h[u], b.c[u] + b.h[u]) - Math.max(s.c[u] - s.h[u], b.c[u] - b.h[u]) < 1.5) continue; bands.push(j); }
    const top = Math.max(0, ...bands.map(j => boxes[j].c[1] + boxes[j].h[1])), score = bands.length + top;
    if (bands.length && (!pick || score > pick.score)) pick = { wall: B.id, bands, score, top, ax }; }
  if (!pick) return { err: 'no partition of this level carries a band from the floor' };
  const b = boxes[pick.wall], u = pick.ax ? 0 : 2, n = 2 - u, free = (p) => !g.col.nav.blocked[g.col.nav.idx(Math.floor(p[0] / g.col.nav.cell), Math.floor(p[2] / g.col.nav.cell))];
  const pt = (off) => { const p = [b.c[0], 0, b.c[2]]; p[n] += off; return p; }, side = [1, -1].find(s => free(pt(s * (b.h[n] + 1.2))) && free(pt(s * (b.h[n] + 2.4)))) || 1;
  pick.side = side; pick.at = pt(side * (b.h[n] + 0.3)); pick.at[1] = 1.0; pick.from = pt(side * (b.h[n] + 2.6)); pick.mid = [b.c[0], 0.9, b.c[2]];
  window.__skin = pick;
  return { ...pick, rule: window.__knobs.registry().WALLSKIN.on, len: 2 * b.h[u] };
};
// in the page: the band's faces sampled against what the world rasterises and against the wall's cells (wallskin-check.ts's oracle)
const READ = () => {
  const g = window.__game, w = window.__engine.world, walls = g.destr.walls, S = walls.skins, P = window.__skin, boxes = g.col.boxes, wb = boxes[P.wall], u = P.ax ? 0 : 2;
  const nS = w.statics.length, st = new Set(), dyn = []; for (let k = 0; k < w.visibleCount + w.visibleCutCount; k++) { const i = w.visible[k] & 0xFFFF; if (i < nS) st.add(i); else dyn.push(w.all[i]); }
  const row = ((walls.snapshot() || { b: [] }).b.find(r => r[0] === P.wall) || [])[1] || '', N = row.length / 2, w0 = wb.c[u] - wb.h[u], len = 2 * wb.h[u], n = 2 - u;
  let over = 0, lost = 0, open = 0;
  for (const j of P.bands) { const s = boxes[j]; const mine = dyn.filter(b => b.name === 'skin' && b.albedo.every((v, k) => Math.abs(v - s.albedo[k]) < 1e-9) && Math.abs(b.c[n] - s.c[n]) <= b.h[n] + 1e-6);
    for (let x = s.c[u] - s.h[u] + 0.0118; x < s.c[u] + s.h[u]; x += 0.0237) for (let y = s.c[1] - s.h[1] + 0.0118; y < s.c[1] + s.h[1]; y += 0.0237) {
      const k = Math.floor((x - w0) / len * N), backed = !N || k < 0 || k >= N || row[(y >= 0.7 ? 1 : 0) * N + k] !== '1';
      const drawn = st.has(j) || mine.some(b => Math.abs(x - b.c[u]) <= b.h[u] + 1e-9 && Math.abs(y - b.c[1]) <= b.h[1] + 1e-9);
      if (!backed) open++; if (drawn && !backed) over++; if (!drawn && backed) lost++; } }
  return { cells: row, stats: { ...S.stats }, same: w.undrawn === S.cut, inList: w.dynamics.filter(b => b.name === 'skin').length, over, lost, open, fatal: (document.getElementById('fatal') || {}).textContent || '' };
};
const GRAB = async (n) => {
  const dev = window.__engine.device;
  const yieldMacro = () => new Promise(res => { const ch = new MessageChannel(); ch.port1.onmessage = () => res(); ch.port2.postMessage(0); });
  for (let i = 0; i < n; i++) { window.__runFrames(1, 1 / 6000); await dev.queue.onSubmittedWorkDone(); await yieldMacro(); }
  window.__engine.render(window.__cam, window.__game.time, 1 / 60, false); const fr = await window.__engine.readbackFrame(); if (!fr) return null;
  if (fr.bgra) for (let i = 0; i < fr.data.length; i += 4) { const b = fr.data[i]; fr.data[i] = fr.data[i + 2]; fr.data[i + 2] = b; }
  for (let i = 3; i < fr.data.length; i += 4) fr.data[i] = 255;
  const c2 = document.createElement('canvas'); c2.width = fr.w; c2.height = fr.h; c2.getContext('2d').putImageData(new ImageData(new Uint8ClampedArray(fr.data.buffer), fr.w, fr.h), 0, 0);
  return c2.toDataURL('image/png');
};

(async () => {
  let red = 0; const browser = await chromium.launch({ headless: true, ...(process.env.CHROMIUM ? { executablePath: process.env.CHROMIUM } : { channel: 'chromium' }), args: FLAGS });
  if (STILLS) fs.mkdirSync(STILLS, { recursive: true });
  for (const job of [{ name: 'on', query: '' }, { name: 'off', query: 'wallskin=0&' }]) {
    const page = await (await browser.newContext({ viewport: { width: 1024, height: 576 }, deviceScaleFactor: 1 })).newPage();
    const errs = []; page.on('pageerror', e => errs.push(String(e).slice(0, 300))); page.on('console', m => { if (m.type() === 'error' && !/Failed to load resource/.test(m.text())) errs.push(m.text().slice(0, 300)); });
    page.on('response', r => { if (r.status() >= 400) log(`  (${r.status()} ${r.url()})`); });
    const url = `http://localhost:${PORT}/?levelFile=${LEVEL}&roomclear&squad4&${job.query}${EXTRA}paused&quality=${QUALITY}&mute`; log(`${job.name}: ${url}`);
    let r = {}; const still = async (tag) => { if (!STILLS) return; const u = await page.evaluate(GRAB, 3); if (u) { fs.writeFileSync(path.join(STILLS, `skin-${job.name}-${tag}.png`), Buffer.from(u.split(',')[1], 'base64')); log(`  still ${job.name} ${tag}`); } };
    try {
      await page.goto(url); await page.evaluate(async () => { await window.__ready; }); await page.evaluate(STEP, 60); const s = await page.evaluate(FIND); if (s.err) throw new Error(s.err); Object.assign(r, s);
      const cam = (far) => page.evaluate(({ s: { mid, from, side, ax }, far }) => { const sd = far ? -side : side;   // the camera on the blast's side (or the far one), off to one side of Sam, who stands in the room on the blast's side (the picture is his cone's)
        window.__stage({ look: mid, yaw: Math.atan2(ax ? 0 : sd, ax ? sd : 0) + 0.5, pitch: 0.5, dist: 6.5 }, { player: { pos: from, yaw: Math.atan2(mid[0] - from[0], mid[2] - from[2]) } }); }, { s, far });
      await cam(false); await page.evaluate(STEP, 20); await still('0-before'); if (STILLS && FAR) { await cam(true); await still('0-before-far'); await cam(false); }
      Object.assign(r, { blast: await page.evaluate(() => { const g = window.__game, P = window.__skin; return g.destr.blast(P.at, { who: g.player.id || 0, radius: 2.2 }); }) });
      await page.evaluate(STEP, 120); Object.assign(r, await page.evaluate(READ)); await still('1-after'); if (STILLS && FAR) { await cam(true); await still('1-after-far'); }
    } catch (e) { r = { err: String(e).slice(0, 300) }; }
    await page.close();
    if (r.err) { red++; console.log(`- FAIL — ${job.name}: ${r.err}${errs.length ? ` · ${errs[0]}` : ''}`); continue; }
    const detail = `rule ${r.rule} · wall #${r.wall} (${r.len.toFixed(1)} m, ${r.bands.length} bands from the floor to ${r.top.toFixed(2)} m) · cells ${r.cells} · ${r.open} points of band over open cells: drawn there ${r.over} · lost over standing wall ${r.lost} · skins cut ${r.stats.cut} into ${r.stats.pieces} pieces, ${r.inList} in the frame's list · the world's list is the rule's ${r.same} · page errors ${errs.length}${errs.length ? `: ${errs[0]}` : ''}${r.fatal ? ` · #fatal: ${r.fatal.slice(0, 120)}` : ''}`;
    const clean = !errs.length && !r.fatal, holed = r.blast && r.blast.walls >= 1 && r.open > 0;
    const ok = job.name === 'on' ? clean && r.rule === true && holed && r.over === 0 && r.lost === 0 && r.stats.cut >= 1 && r.inList >= 1 && r.same : clean && r.rule === false && holed && (r.over > 0 || r.lost > 0) && r.stats.cut === 0 && r.inList === 0;
    if (!ok) red++;
    console.log(`- ${ok ? 'PASS' : 'FAIL'} — ${job.name === 'on' ? 'the page as it opens: the rule is on, the wall holes, and its band is drawn only where the wall still stands behind it' : '?wallskin=0: the rule is off, nothing is cut, and the band hangs across the hole or is gone from the whole wall (the build as it was)'}\n    ${detail}`);
  }
  await browser.close();
  console.log(`=== wallskin-page: ${red ? 'FAILURES above' : 'ALL PASS'} ===`);
  process.exit(red ? 1 : 0);
})();
