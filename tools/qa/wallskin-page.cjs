// WHAT IS PAINTED ON A WALL GOES WITH THE WALL, ON THE REAL PAGE (src/game/wallskin.ts WALLSKIN): headless Chromium on the software WebGPU adapter
// (tools/qa/page-look.cjs's recipe), a dressed level's room clear as the page deals it, and the heart of tools/qa/wallskin-check.ts done with the page's own
// Game and its own box world: the partition with the most painted band from the floor found, a blast set against it, and the band's face sampled against
// what the world rasterises (BoxWorld.visible after the page's own upload) and against the level's own collision (what each point lay on as the page
// opened, by a short ray straight in, and whether a solid surface still stands there).
//   PORT=5199 node tools/qa/wallskin-page.cjs          (a dev server on that port: PORT=5199 bun run tools/serve.ts)
//   on    the page as it opens: the switch reads on, the wall holes, no point of its bands that has lost its wall is drawn and none that has not is lost
//   off   the same page under ?wallskin=0: the switch reads off, nothing is cut, and the band is wrong the way it was: on a level in the look's paints the
//         page's own scene/lookTidy.ts takes the wall's whole band off its standing length; on any other level it hangs across the hole
//   STILLS=<dir>  keeps frames read back from the page, both runs: before the blast and two seconds after it (software adapter, verylow; about five minutes a
//                 frame there) · FAR=1  each from the wall's far side too (the blast's side is often the dark one)
//   CORNER=1      at a corner where two partitions overlap instead: one wall's two end columns opened, every thin sheet at the corner asked (the band on
//                 the wall that stands is whole, and so is the opened wall's band over the standing one's end)
//   PITCH=<rad> DIST=<m>  the stills' camera (0.5 and 6.5 at a wall; 0.8 and 5.5 at a corner, steeper: a corner's rooms are small). At a corner the camera stands on the side most of its paint faces
//   LEVEL=<file>  the level (default block-gwot.json) · EXTRA=<query> more of the query string for both runs (EXTRA='hour=midday')
//   PLAYWRIGHT=<dir>  where playwright (or playwright-core) is, if not /opt/node-tools/node_modules/playwright · CHROMIUM=<path> a browser to run
// exit 0 = both, with no page error · 1 = otherwise. The verdict is the box lists only: nothing here judges a picture, and the software adapter is not a GPU.
const { chromium } = require(process.env.PLAYWRIGHT || '/opt/node-tools/node_modules/playwright');
const fs = require('fs'), path = require('path');
const PORT = process.env.PORT || 5199, QUALITY = process.env.QUALITY || 'verylow', STILLS = process.env.STILLS || '', EXTRA = process.env.EXTRA ? `${process.env.EXTRA}&` : '', LEVEL = process.env.LEVEL || 'block-gwot.json', FAR = process.env.FAR === '1', CORNER = process.env.CORNER === '1', PITCH = +(process.env.PITCH || (CORNER ? 0.8 : 0.5)), DIST = +(process.env.DIST || (CORNER ? 5.5 : 6.5));
const FLAGS = ['--enable-unsafe-webgpu', '--use-gl=angle', '--use-angle=swiftshader', '--enable-features=Vulkan', '--use-vulkan=swiftshader', '--ignore-gpu-blocklist', '--disable-gpu-watchdog'];
const T0 = Date.now(), log = (...a) => console.log(`[${((Date.now() - T0) / 1000).toFixed(0)}s]`, ...a);

// in the page: the game stepped with no frame drawn (the box world still packs: its list of what it draws is the page's own)
const STEP = (n) => { const g = window.__game, e = window.__engine, real = e.render; e.render = () => { e.world.upload(); }; try { for (let i = 0; i < n; i++) { g.player.visibility = 0.05; window.__runFrames(1, 1 / 60); } } finally { e.render = real; } };
// in the page: the partition with the most band from the floor on its faces (drawn-only, thin, on the face or within the rule's reach of it), and where a blast goes against it
const FIND = async () => {
  await window.__ready;
  const g = window.__game, boxes = g.col.boxes, walls = g.destr.walls, off = window.__knobs.registry().WALLSKIN.off; g.godMode = true; g.tune.chaos = 0.5;
  let pick = null;
  for (const B of walls.all()) { if (B.kind !== 'wall') continue; const b = B.box, ax = b.h[0] >= b.h[2], u = ax ? 0 : 2, n = 2 - u; if (2 * b.h[u] < 4) continue;
    const bands = [];
    for (let j = 0; j < walls.authored; j++) { const s = boxes[j]; if ((s.flags & 11) !== 2 || s.c[1] - s.h[1] > 0.06 || 2 * s.h[n] > 0.02 || s.h[u] < 0.5 || (s.marks && s.marks.length)) continue;   // (plain bands: a marked one stands as the whole box under a cut record, which this oracle does not read; wallskin-check.ts's does)
      const gap = Math.max(s.c[n] - s.h[n] - (b.c[n] + b.h[n]), b.c[n] - b.h[n] - (s.c[n] + s.h[n])); if (gap > off || gap < -0.02) continue;   // (on the face, or as far off it as the rule reaches)
      if (Math.min(s.c[u] + s.h[u], b.c[u] + b.h[u]) - Math.max(s.c[u] - s.h[u], b.c[u] - b.h[u]) < 1.5) continue; bands.push(j); }
    const top = Math.max(0, ...bands.map(j => boxes[j].c[1] + boxes[j].h[1])), score = bands.length + top;
    if (bands.length && (!pick || score > pick.score)) pick = { wall: B.id, bands: bands.map(j => ({ j, u, sg: boxes[j].c[n] >= b.c[n] ? 1 : -1 })), score, top, ax }; }
  if (!pick) return { err: 'no partition of this level carries a band from the floor' };
  const b = boxes[pick.wall], u = pick.ax ? 0 : 2, n = 2 - u, free = (p) => !g.col.nav.blocked[g.col.nav.idx(Math.floor(p[0] / g.col.nav.cell), Math.floor(p[2] / g.col.nav.cell))];
  const pt = (off) => { const p = [b.c[0], 0, b.c[2]]; p[n] += off; return p; }, side = [1, -1].find(s => free(pt(s * (b.h[n] + 1.2))) && free(pt(s * (b.h[n] + 2.4)))) || 1;
  pick.side = side; pick.at = pt(side * (b.h[n] + 0.3)); pick.at[1] = 1.0; pick.from = pt(side * (b.h[n] + 2.6)); pick.mid = [b.c[0], 0.9, b.c[2]];
  window.__skin = pick;
  return { ...pick, rule: window.__knobs.registry().WALLSKIN.on, len: 2 * b.h[u] };
};
// in the page: a corner where two partitions overlap, with thin drawn-only sheets on it, and somewhere free to stand and look at it
const FINDC = async () => {
  await window.__ready;
  const g = window.__game, boxes = g.col.boxes, walls = g.destr.walls, list = walls.list, off = window.__knobs.registry().WALLSKIN.off; g.godMode = true; g.tune.chaos = 0.5;
  const free = (p) => !g.col.nav.blocked[g.col.nav.idx(Math.floor(p[0] / g.col.nav.cell), Math.floor(p[2] / g.col.nav.cell))];
  const near = (at, r) => g.col.window(() => { const out = [];
    for (let j = 0; j < walls.authored; j++) { const s = boxes[j]; if ((s.flags & 131083) !== 2 || s.yaw || s.rot || s.round === true) continue;
      const t = s.h[0] <= s.h[2] ? 0 : 2, u = 2 - t; if (2 * s.h[t] > 0.04 + 1e-6 || s.h[t] > s.h[1] || Math.max(Math.abs(s.c[0] - at[0]) - s.h[0], Math.abs(s.c[2] - at[2]) - s.h[2]) > r) continue;
      const met = (sg) => { let k = 0; const ro = [s.c[0], s.c[1], s.c[2]], rd = [0, 0, 0]; rd[t] = -sg; ro[t] += sg * (s.h[t] + 0.02); for (let q = 0; q < 9; q++) { ro[u] = s.c[u] + s.h[u] * (q - 4) / 4.5; if (g.col.raycast(ro, rd, 0.02 + 2 * s.h[t] + off)) k++; } return k; };
      const a = met(1), c = met(-1); if (a + c) out.push({ j, u, sg: a >= c ? 1 : -1 }); }
    return out; });
  let pick = null;
  for (const A of list) { if (pick || A.kind !== 'wall' || A.hard || A.box.yaw) continue; const a = A.box, u = a.h[0] >= a.h[2] ? 0 : 2, n = 2 - u, N = u === 0 ? A.nx : A.nz; if (N < 4) continue;
    for (const B of list) { const b = B.box; if (pick || B === A || B.kind !== 'wall' || B.hard || b.yaw || (b.h[0] >= b.h[2] ? 0 : 2) !== n) continue;
      if (Math.min(a.c[0] + a.h[0], b.c[0] + b.h[0]) - Math.max(a.c[0] - a.h[0], b.c[0] - b.h[0]) < 0.02 || Math.min(a.c[2] + a.h[2], b.c[2] + b.h[2]) - Math.max(a.c[2] - a.h[2], b.c[2] - b.h[2]) < 0.02) continue;
      const end = b.c[u] >= a.c[u] ? 1 : -1, at = [a.c[0], 0, a.c[2]]; at[u] = a.c[u] + end * a.h[u]; const bands = near(at, 0.8); if (bands.length < 2) continue;
      const k = (c) => end > 0 ? N - 1 - c : c, cells = []; for (const c of [0, 1]) for (let iy = 0; iy < A.ny; iy++) cells.push((iy * A.nz + (u === 0 ? 0 : k(c))) * A.nx + (u === 0 ? k(c) : 0));
      const face = [0, 0]; for (const x of bands) { const s = boxes[x.j]; face[x.u === 0 ? 1 : 0] += x.sg * s.h[x.u] * s.h[1]; }   // (the way the corner's paint faces, by area: its sheets' fronts)
      const want = [Math.sign(face[0]) || 1, Math.sign(face[1]) || 1]; let from = null;
      for (const d of [want, [want[0], -want[1]], [-want[0], want[1]], [-want[0], -want[1]]]) { const p = [at[0] + 1.9 * d[0], 0, at[2] + 1.9 * d[1]], q = [at[0] + 1.0 * d[0], 0, at[2] + 1.0 * d[1]]; if (!from && free(p) && free(q)) from = p; }
      if (!from) continue;
      pick = { corner: true, wall: A.id, other: B.id, cells, bands, at, from, mid: [at[0], 0.8, at[2]], top: Math.max(...bands.map(x => boxes[x.j].c[1] + boxes[x.j].h[1])), len: 2 * a.h[u] }; } }
  if (!pick) return { err: 'no corner of this level where two partitions overlap carries two sheets' };
  window.__skin = pick;
  return { ...pick, rule: window.__knobs.registry().WALLSKIN.on };
};
// in the page: the bands' faces sampled against what the world rasterises and against the level's own collision (wallskin-check.ts's oracle). The first
// call is the baseline: the first solid surface a short ray straight in meets is what that point lay on; a point has lost its wall when no solid
// surface stands at that depth now. A point with something solid or something that moves in front of it is not asked.
const READ = () => {
  const g = window.__game, w = window.__engine.world, walls = g.destr.walls, S = walls.skins, P = window.__skin, boxes = g.col.boxes, off = window.__knobs.registry().WALLSKIN.off, SKIN = 10 << 24;
  const nS = w.statics.length, st = new Set(), dyn = [], cutBy = new Set(); for (let k = 0; k < w.visibleCount + w.visibleCutCount; k++) { const i = w.visible[k] & 0xFFFF; if (i < nS) st.add(i); else { dyn.push(w.all[i]); if (k >= w.visibleCount) cutBy.add(w.all[i]); } }
  const first = !P.base, base = P.base || (P.base = P.bands.map(() => [])); let over = 0, lost = 0, open = 0, asked = 0;
  g.col.window(() => P.bands.forEach(({ j, u, sg }, q) => { const s = boxes[j], n = 2 - u, mine = dyn.filter(b => b.mkey !== undefined && b.mkey >>> 24 === SKIN >>> 24 && ((b.mkey - SKIN) >>> 4) === j);
    const ro = [0, 0, 0], rd = [0, 0, 0], reach = 0.02 + 2 * s.h[n] + off; rd[n] = -sg; ro[n] = s.c[n] + sg * (s.h[n] + 0.02); let cnt = 0;
    for (let x = s.c[u] - s.h[u] + 0.0118; x < s.c[u] + s.h[u]; x += 0.0237) for (let y = s.c[1] - s.h[1] + 0.0118; y < s.c[1] + s.h[1]; y += 0.0237) {
      ro[u] = x; ro[1] = y; const hit = g.col.raycast(ro, rd, reach), met = hit ? boxes[hit.index] : undefined, moves = !!hit && !met, t = met && !(met.flags & (1 << 18)) ? hit.t : Infinity;   // (1 << 18: a bare stud left in an opened cell holds no paint)
      if (first) base[q].push(moves || t < 0.02 - 1e-6 ? -1 : t);
      const b0 = base[q][cnt++]; if (b0 < 0 || moves || t < 0.02 - 1e-6) continue;
      const gone = b0 < Infinity && !(t <= b0 + 1e-4);
      const drawn = st.has(j) || mine.some(b => Math.abs(x - b.c[u]) <= b.h[u] + 1e-9 && Math.abs(y - b.c[1]) <= b.h[1] + 1e-9 && (!cutBy.has(b) || b.cut.every(c => c.n[u] * (x - b.c[u]) + c.n[1] * (y - b.c[1]) <= c.d + 1e-9)));
      asked++; if (gone) open++; if (drawn && gone) over++; if (!drawn && !gone) lost++; } }));
  const row = ((walls.snapshot() || { b: [] }).b.find(r => r[0] === P.wall) || [])[1] || '';
  return { cells: row, stats: { ...S.stats }, same: S.out.size ? w.undrawn === S.out : w.undrawn === null, inList: w.dynamics.filter(b => b.name === 'skin').length, keys: new Set(w.dynamics.filter(b => b.name === 'skin').map(b => b.mkey)).size, over, lost, open, asked, fatal: (document.getElementById('fatal') || {}).textContent || '' };
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
    let r = {}; const still = async (tag) => { if (!STILLS) return; const u = await page.evaluate(GRAB, 3); if (u) { fs.writeFileSync(path.join(STILLS, `skin-${CORNER ? 'corner-' : ''}${job.name}-${tag}.png`), Buffer.from(u.split(',')[1], 'base64')); log(`  still ${job.name} ${tag}`); } };
    try {
      await page.goto(url); await page.evaluate(async () => { await window.__ready; }); await page.evaluate(STEP, 60); const s = await page.evaluate(CORNER ? FINDC : FIND); if (s.err) throw new Error(s.err); Object.assign(r, s); await page.evaluate(READ);
      const cam = (far) => page.evaluate(({ s: { mid, from, side, ax, corner }, far, PITCH, DIST }) => { const sd = far ? -side : side;   // the camera on the blast's side (or the far one), off to one side of Sam, who stands in the room on the blast's side (the picture is his cone's); at a corner, over Sam's shoulder (or from the other side of the corner)
        const yaw = corner ? Math.atan2(from[0] - mid[0], from[2] - mid[2]) + (far ? Math.PI : 0) : Math.atan2(ax ? 0 : sd, ax ? sd : 0) + 0.5;
        window.__stage({ look: mid, yaw, pitch: PITCH, dist: DIST }, { player: { pos: from, yaw: Math.atan2(mid[0] - from[0], mid[2] - from[2]) } }); }, { s, far, PITCH, DIST });
      await cam(false); await page.evaluate(STEP, 20); await still('0-before'); if (STILLS && FAR) { await cam(true); await still('0-before-far'); await cam(false); }
      Object.assign(r, { blast: await page.evaluate(() => { const g = window.__game, P = window.__skin, walls = g.destr.walls; if (!P.corner) return g.destr.blast(P.at, { who: g.player.id || 0, radius: 2.2 });
        const A = walls.list.find(B => B.id === P.wall); return g.col.window(() => walls.openCells(A, P.cells, 'blast', g.player.id || 0, null, false)) ? { walls: 1 } : null; }) });
      await page.evaluate(STEP, 120); Object.assign(r, await page.evaluate(READ)); await still('1-after'); if (STILLS && FAR) { await cam(true); await still('1-after-far'); }
    } catch (e) { r = { err: String(e).slice(0, 300) }; }
    await page.close();
    if (r.err) { red++; console.log(`- FAIL — ${job.name}: ${r.err}${errs.length ? ` · ${errs[0]}` : ''}`); continue; }
    const detail = `rule ${r.rule} · ${r.corner ? `the corner of walls #${r.wall} and #${r.other}, #${r.wall}'s two end columns opened, ${r.bands.length} sheets at it` : `wall #${r.wall} (${r.len.toFixed(1)} m, ${r.bands.length} bands from the floor to ${r.top.toFixed(2)} m)`} · cells ${r.cells} · ${r.asked} points asked, ${r.open} have lost their wall: drawn there ${r.over} · lost with a wall behind ${r.lost} · sheets cut ${r.stats.cut} into ${r.stats.pieces} pieces, ${r.inList} in the frame's list under ${r.keys} keys · the world's list is the rule's ${r.same} · page errors ${errs.length}${errs.length ? `: ${errs[0]}` : ''}${r.fatal ? ` · #fatal: ${r.fatal.slice(0, 120)}` : ''}`;
    const clean = !errs.length && !r.fatal, holed = r.blast && r.blast.walls >= 1 && r.open > 0;
    const ok = job.name === 'on' ? clean && r.rule === true && holed && r.over === 0 && r.lost === 0 && r.stats.cut >= 1 && r.inList >= 1 && r.keys === r.inList && r.same : clean && r.rule === false && holed && (r.over > 0 || r.lost > 0) && r.stats.cut === 0 && r.inList === 0;
    if (!ok) red++;
    console.log(`- ${ok ? 'PASS' : 'FAIL'} — ${job.name === 'on' ? 'the page as it opens: the rule is on, the wall holes, and its band is drawn only where the surface it lay on still stands' : '?wallskin=0: the rule is off, nothing is cut, and the band hangs across the hole or is gone from the whole wall (the build as it was)'}\n    ${detail}`);
  }
  await browser.close();
  console.log(`=== wallskin-page: ${red ? 'FAILURES above' : 'ALL PASS'} ===`);
  process.exit(red ? 1 : 0);
})();
