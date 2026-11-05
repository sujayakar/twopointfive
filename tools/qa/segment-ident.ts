// SEGMENT TESTS, OLD AGAINST NEW — collision.ts segmentBlocked / segmentBlockedDynamic / shotBlocked's pane list were rewritten for speed (scalars, a cell scanned
// once, a far box passed before its slab test) on the promise that no answer moves. This holds the code as it was (below, verbatim but for reading the collision
// object's private fields) beside the code as it is, over random segments on every level: endpoints anywhere, at eye / chest / shin heights, on box faces and a
// hair inside them, straight up, a few centimetres long, through door leaves swung to random angles — the answer, every pane noted and its distance (and, when the panes
// are listed, every glassOpen question in order) must be the same. Exit 1 on the first level with a difference.
//   bun run tools/qa/segment-ident.ts [--n 200000] [--levels office.json,warehouse.json] [--seed 1] [--bench]
// --rust: THE RUST MODULE'S WALK AGAINST BOTH (src/sim/sim.ts RUST.lines; rust/sim/src/lines.rs over the copy src/scene/boxMirror.ts keeps). No tick runs here,
// so the tool opens the copy as a tick would (StaticCollision.tickOpen) and says so when it re-shapes the broken walls (wrote), as the game's rules do; "now"
// is then the module's answer, held to the code as it was AND to this file's own walk as it is. A pane's open question is the module's own there
// (rust/sim/src/glass.rs over the copy's panes), so with --rust the holes are REAL ones: the tool breaks the level's glass with the game's own rule
// (rounds, blasts, the glass's step) before the segments and again half way, and the made-up set of holes is left to the run without --rust; the
// order the TypeScript's walks ask the question in is compared there too, for the module asks nobody. RUST_BREAK=1 with --rust is the red path: the
// re-shape goes unsaid (and the panes walked through, before the circles), the module answers from a copy that is no longer the boxes, and the run must
// FAIL — for the lines AND for the circles: the differing answers are counted by row, and exit 3 names a row the red path left green (exit 1 = red as it must).
// With --rust the CIRCLES are held too (RUST.circles; rust/sim/src/collide.rs): a quarter as many circles as segments — a body's footprint anywhere, on a box's
// face, dead on a box's centre, of every size a man or a prop has — pushed out, asked what they touch and gathered, by the module and by this file's own code:
// where the circle ends (both coordinates' bits), whether it touched, and every list the same.
// With --rust the RAYS are held too (RUST.rays; lines.rs ray against collision.ts raycastHere): along every segment, and short of and past its end, the
// nearest thing met, how far (its bits) and the face's normal (its bits) — over leaves, tilted statics, and pieces this tool makes ROUND and CUT (a cut
// record of one or two planes), with a leaf taken out of the list for some (the list a rule swaps in for itself), and through panes of glass that are
// open where the ray meets them. A run is fair only if rays hit each kind of box and nearly all were the module's.
import { readdirSync } from 'fs';
import { resolve } from 'path';
import { standUp } from './headless.ts';
const ROOT = resolve(import.meta.dir, '../..');
const { v3, quat } = await import(`${ROOT}/src/math/vec.ts`);
const { BoxFlag, rayBox, makeBox } = await import(`${ROOT}/src/scene/boxes.ts`);
const { lineBox, RAYS } = await import(`${ROOT}/src/scene/collision.ts`);
const CUT: any = await import(`${ROOT}/src/scene/cut.ts`);
const { loadLevel } = await import(`${ROOT}/src/scene/levelLoad.ts`);
type Vec3 = [number, number, number];

// ── the code as it was (a7ce1d6) ──
let refStamp = new Int32Array(0), refId = 0;
function refDynamic(col: any, a: Vec3, b: Vec3, needFlags = 0): boolean {
  const d = v3.sub(b, a); const len = v3.len(d); if (len < 1e-4) return false; const rd = v3.scale(d, 1 / len);
  for (const db of col.dynamicBoxes) { if ((db.flags & needFlags) !== needFlags) continue; const hit = rayBox(a, rd, db, len); if (hit && hit.t >= 0) return true; }
  return false;
}
function refBlocked(col: any, a: Vec3, b: Vec3, shrink = 0.02, through: number = BoxFlag.Glass, panes: { bi: number; t: number }[] | null = null): boolean {
  const d = v3.sub(b, a); const len = v3.len(d); if (len < 1e-4) return false;
  const rd = v3.scale(d, 1 / len);
  if (refDynamic(col, a, b)) return true;
  if (refStamp.length < col.boxes.length) refStamp = new Int32Array(col.boxes.length);
  const id = ++refId; const steps = Math.ceil(len / 0.5) + 1;
  for (let i = 0; i <= steps; i++) {
    const t = (i / steps) * len; const x = Math.floor((a[0] + rd[0] * t) / col.cs), z = Math.floor((a[2] + rd[2] * t) / col.cs);
    for (let dz = -1; dz <= 1; dz++) for (let dx = -1; dx <= 1; dx++) {
      const cx = x + dx, cz = z + dz; if (cx < 0 || cz < 0 || cx >= col.W || cz >= col.H) continue;
      const cell = col.cells[cz * col.W + cx];
      for (let k = 0; k < cell.length; k++) {
        const bi = cell[k]; if (refStamp[bi] === id) continue; refStamp[bi] = id;
        if (col.boxes[bi].flags & (through | BoxFlag.NoShadow)) continue;
        const s = lineBox(a, rd, col.boxes[bi]); if (!s) continue;
        if (s.depth > shrink) continue;
        if (s.t1 > shrink && s.t0 < len - shrink) {
          if (col.glassOpen && (col.boxes[bi].flags & BoxFlag.Glass) && col.glassOpen(bi, v3.mad(a, rd, Math.max(0, s.t0)))) continue;
          if (panes && (col.boxes[bi].flags & BoxFlag.Glass)) { panes.push({ bi, t: Math.max(0, s.t0) }); continue; }
          return true;
        }
      }
    }
  }
  return false;
}

// ── the run ──
const argv = process.argv.slice(2); const flag = (n: string) => { const i = argv.indexOf(n); return i >= 0 ? argv[i + 1] : undefined; };
const N = Number(flag('--n') ?? 200000), SEED = Number(flag('--seed') ?? 1);
const files = (flag('--levels') ?? readdirSync(resolve(ROOT, 'levels')).filter(f => f.endsWith('.json') && !f.endsWith('.lint.json')).join(',')).split(',');
let s32 = SEED >>> 0; const rnd = () => { s32 = (s32 + 0x6D2B79F5) >>> 0; let t = s32; t = Math.imul(t ^ (t >>> 15), t | 1); t ^= t + Math.imul(t ^ (t >>> 7), t | 61); return ((t ^ (t >>> 14)) >>> 0) / 4294967296; };
const RUSTED = argv.includes('--rust'), UNSAID = RUSTED && process.env.RUST_BREAK === '1';
const SIM: any = await import(`${ROOT}/src/sim/sim.ts`), GLASS: any = await import(`${ROOT}/src/game/glass.ts`);
if (RUSTED) { const no = SIM.simWhyNot(); if (no) { console.log(`=== segment-ident --rust: the module does not run here (${no}) — FAIL ===`); process.exit(1); } }
const RAYED = { n: 0, there: 0, hit: 0, leaf: 0, tilted: 0, round: 0, cut: 0, roundT: 0, cutT: 0, swapped: 0, turned: 0, long: 0 };   // (the rays over every level: asked, the module's, and what they hit)
let bad = 0; const ever = { lines: 0, rays: 0, circles: 0 };   // (answers that differed over every level, by the row that gave them: the red path must turn BOTH red)
const log = console.log, err = console.error; const hush = (on: boolean) => { console.log = on ? () => {} : log; console.error = on ? () => {} : err; };
for (const file of files) {
  let W: any; hush(true);
  try { const data = JSON.parse(await Bun.file(resolve(ROOT, 'levels', file)).text()); W = await standUp({ uploadEvery: 1000, ...(file === 'office.json' ? {} : { level: loadLevel(data) }) }); }
  catch (e) { hush(false); log(`${file.padEnd(20)} not stood up (${String((e as Error).message).split('\n')[0].slice(0, 90)}) — skipped`); continue; }
  hush(false);
  if (!RUSTED) SIM.RUST.lines = SIM.RUST.rays = SIM.RUST.circles = false;   // (without --rust this file holds the TypeScript's walk against the walk as it was: the module's rows, where a stand has them up, are put down)
  const g = W.game, col = g.col as any; const boxes = col.boxes as any[];
  for (const d of g.doors.list) { if (rnd() < 0.6) { d.angle = (rnd() * 2 - 1) * 1.9; d.latched = false; } } hush(true); try { for (let i = 0; i < 3; i++) W.step(1 / 60); } catch { /* a level that needs more to run a frame still has its statics and leaves */ } hush(false);
  // BROKEN WALLS (a build whose statics change after the bake — StaticCollision.addStatic; skipped where there is no such thing): a share of the upright statics is
  // hidden and stood back up as pieces appended to the list, filed in the authored box's cells; half way through the segments every piece is re-shaped in place.
  const HIDE = BoxFlag.NoPrimary | BoxFlag.NoShadow; const broken: { box: any; pieces: any[] }[] = [];
  const reshape = (B: { box: any; pieces: any[] }) => { const b = B.box; for (const p of B.pieces) {
    const h: Vec3 = [b.h[0] * (0.15 + rnd() * 0.85), b.h[1] * (0.15 + rnd() * 0.85), b.h[2] * (0.15 + rnd() * 0.85)]; const l: Vec3 = [(rnd() * 2 - 1) * (b.h[0] - h[0]), (rnd() * 2 - 1) * (b.h[1] - h[1]), (rnd() * 2 - 1) * (b.h[2] - h[2])];
    p.c = [b.c[0] + Math.cos(b.yaw) * l[0] + Math.sin(b.yaw) * l[2], b.c[1] + l[1], b.c[2] - Math.sin(b.yaw) * l[0] + Math.cos(b.yaw) * l[2]]; p.h = h; p.flags = rnd() < 0.15 ? b.flags0 | HIDE : b.flags0;
    if (RUSTED) {   // (the shapes a break leaves: a share of the pieces round, a share cut by a plane or two)
      delete p.cut; delete p.round; delete p.rot; const q = rnd();
      if (rnd() < 0.3) {   // (…and a share of them TUMBLED, a round or a cut piece lying as it fell: made small enough, about the middle of what broke, to lie inside its plan however it is turned — a static is filed in the grid by its plan alone)
        const R = Math.hypot(h[0], h[1], h[2]), fit = 0.9 * Math.min(b.h[0], b.h[2]) / R; if (fit < 1) { h[0] *= fit; h[1] *= fit; h[2] *= fit; }
        p.c = [b.c[0], b.c[1] + l[1], b.c[2]];
        const u = rnd() * 2 - 1, th = rnd() * 6.283, r = Math.sqrt(1 - u * u), a = (rnd() - 0.5) * 2.4, sa = Math.sin(a / 2); p.rot = [r * Math.cos(th) * sa, u * sa, r * Math.sin(th) * sa, Math.cos(a / 2)]; RAYED.turned++; }
      if (q < 0.12) { h[0] = h[2] = Math.min(h[0], h[2]); p.round = true; }
      else if (q < 0.35) { const pl = () => { const u = rnd() * 2 - 1, th = rnd() * 6.283, r = Math.sqrt(1 - u * u), n: Vec3 = [r * Math.cos(th), u, r * Math.sin(th)]; return { n, d: (rnd() * 1.4 - 0.7) * (Math.abs(n[0]) * h[0] + Math.abs(n[1]) * h[1] + Math.abs(n[2]) * h[2]) }; };
        const cut = CUT.cutRecord(h, rnd() < 0.5 ? [pl()] : [pl(), pl()]).cut; if (cut) p.cut = cut; } } } };
  if (typeof col.addStatic === 'function') {
    const authored = boxes.length;
    for (let i = 0; i < authored; i++) { const b = boxes[i]; if (b.rot || (b.flags & (HIDE | BoxFlag.Glass)) || rnd() > 0.12) continue;
      const B = { box: b, pieces: [] as any[] }; b.flags0 = b.flags; const n = 1 + Math.floor(rnd() * 3);
      for (let k = 0; k < n; k++) { const pc = makeBox({ c: [b.c[0], b.c[1], b.c[2]], h: [b.h[0], b.h[1], b.h[2]], yaw: b.yaw, albedo: [0.5, 0.5, 0.5], flags: b.flags, name: 'piece' }); col.addStatic(pc, b); B.pieces.push(pc); }
      reshape(B); b.flags = b.flags0 | HIDE; broken.push(B); }
  }
  const x0 = -2, x1 = col.W * col.cs + 2, z0 = -2, z1 = col.H * col.cs + 2;
  const anywhere = (): Vec3 => [x0 + rnd() * (x1 - x0), rnd() < 0.7 ? [0.15, 0.45, 0.9, 1.2, 1.5, 2.0, 2.6][Math.floor(rnd() * 7)] : -0.5 + rnd() * 4.5, z0 + rnd() * (z1 - z0)];
  const onBox = (): Vec3 => { const b = boxes[Math.floor(rnd() * boxes.length)]; const l: Vec3 = [(rnd() * 2 - 1) * b.h[0], (rnd() * 2 - 1) * b.h[1], (rnd() * 2 - 1) * b.h[2]];
    const ax = Math.floor(rnd() * 3); l[ax] = (rnd() < 0.5 ? -1 : 1) * (b.h[ax] + [0, 0, 1e-9, -1e-9, 0.019, 0.021, -0.019, -0.021, 0.001, -0.3][Math.floor(rnd() * 10)]);
    const w = b.rot ? quat.rotate(b.rot, l) : [Math.cos(b.yaw) * l[0] + Math.sin(b.yaw) * l[2], l[1], -Math.sin(b.yaw) * l[0] + Math.cos(b.yaw) * l[2]]; return [b.c[0] + w[0], b.c[1] + w[1], b.c[2] + w[2]]; };
  const onLeaf = (): Vec3 => { const dyn = col.dynamicBoxes; if (!dyn.length) return anywhere(); const b = dyn[Math.floor(rnd() * dyn.length)]; return [b.c[0] + (rnd() * 2 - 1) * 1.5, b.c[1] + (rnd() * 2 - 1) * 1.2, b.c[2] + (rnd() * 2 - 1) * 1.5]; };
  const tilt = boxes.filter(b => b.rot);   // (tilted statics are filed by their yawed footprint alone: a share of the segments start or end on and round them)
  const onTilt = (): Vec3 => { const b = tilt[Math.floor(rnd() * tilt.length)]; if (!b.rot) return anywhere();   // (a piece re-shaped upright since)
    if (rnd() < 0.5) return [b.c[0] + (rnd() * 2 - 1) * 2.5, b.c[1] + (rnd() * 2 - 1) * 0.8, b.c[2] + (rnd() * 2 - 1) * 2.5];
    const l: Vec3 = [(rnd() * 2 - 1) * b.h[0] * 1.2, (rnd() * 2 - 1) * b.h[1] * 1.2, (rnd() * 2 - 1) * b.h[2] * 1.2]; const w = quat.rotate(b.rot, l); return [b.c[0] + w[0], b.c[1] + w[1], b.c[2] + w[2]]; };
  const pick = (): Vec3 => { const r = rnd(); return tilt.length && r > 0.88 ? onTilt() : r < 0.45 ? anywhere() : r < 0.8 ? onBox() : onLeaf(); };
  const asked: string[] = [], askedRef: string[] = []; let sink = asked;
  const realOpen = col.glassOpen, realPanes = col.glassPanes;
  const hang = (f: typeof realOpen) => { if (col.glassOpen !== f) col.glassRule(f, f === realOpen && realPanes ? () => realPanes : null); };   // (the game's own rule with its own rows; a made-up one with none, and the TypeScript answers by it)
  const holes = (bi: number, p: Vec3) => ((bi * 7919 + Math.floor(p[0] * 3) * 31 + Math.floor(p[1] * 3) * 17 + Math.floor(p[2] * 3)) & 3) === 0;   // (a made-up set of fallen cells, the same for both)
  // --rust: REAL holes, made by the game's own rule in the level's own sheets of glass (the module judges a pane by the copy of that rule's state)
  const sheets: number[] = []; boxes.forEach((b: any, i: number) => { if (GLASS.isSheet(b) && !(b.flags & BoxFlag.NoShadow)) sheets.push(i); });
  const smash = (share: number) => { const gb = g.glassBreak; if (!gb || !sheets.length) return;
    GLASS.GLASSFX.on = true; g.tune.glassBreak = true; if (!(g.tune.chaos > 0)) g.tune.chaos = 1;   // (glass breaking live, whatever the stand holds: the holes are this check's footing)
    for (const bi of sheets) { if (rnd() > share) continue; const b = boxes[bi], wide = b.h[0] >= b.h[2], hu = Math.max(b.h[0], b.h[2]), cs = Math.cos(b.yaw), sn = Math.sin(b.yaw);
      const at = (u: number, v: number, w: number): Vec3 => { const lx = wide ? u : w, lz = wide ? w : u; return [b.c[0] + cs * lx + sn * lz, b.c[1] + v, b.c[2] - sn * lx + cs * lz]; };
      if (rnd() < 0.3) gb.blast(at((rnd() * 2 - 1) * hu, 0, (rnd() < 0.5 ? 1 : -1) * (0.3 + rnd())), { radius: 2 + rnd() * 2, power: 0.5 + rnd(), who: 0 });
      else for (let k = 1 + Math.floor(rnd() * 4); k > 0; k--) gb.impact(bi, at((rnd() * 2 - 1) * hu * 0.95, (rnd() * 2 - 1) * b.h[1] * 0.95, 0), [wide ? sn : cs, 0, wide ? cs : -sn], 1 + rnd() * 2, 0, rnd() < 0.5 ? true : undefined); }
    for (let k = Math.floor(rnd() * 12); k > 0; k--) { g.time += 1 / 60; gb.update(1 / 60); } };   // (some of it still on its way out: pieces queued, a toughened pane part way down)
  const PANED = { live: 0 };
  let diff = 0, diffL = 0, diffC = 0, diffR = 0, blockedN = 0, paneN = 0, first = '', inRust = 0;   // (of `diff`: diffL the module's line answers, diffC its circles'; the rest is the walk as it was, or a copy that fell)
  if (RUSTED) { CUT.cutConfig.on = true; SIM.RUST.lines = SIM.RUST.rays = SIM.RUST.circles = true; col.tickOpen(); hush(true); try { smash(0.6); } finally { hush(false); } PANED.live = g.glassBreak?.panes.size ?? 0; }   // (the row raised here: standing a level up puts every row down, as the stand has them — and the copy made the boxes as they stand, the questions below sent to the module)
  for (let i = 0; i < N; i++) {
    if (i === (N >> 1)) { for (const B of broken) reshape(B); if (RUSTED && !UNSAID) col.wrote(); if (RUSTED && !UNSAID) { hush(true); try { smash(0.4); } finally { hush(false); } PANED.live = g.glassBreak?.panes.size ?? 0; } }   // (not on the red path: a pane that goes says the boxes again, and would pay the re-shape's debt for it)
    const a = pick(); let b = pick(); const r = rnd();
    if (r < 0.08) b = [a[0] + (rnd() - 0.5) * 0.05, a[1] + (rnd() - 0.5) * 0.05, a[2] + (rnd() - 0.5) * 0.05]; else if (r < 0.14) b = [a[0], a[1] + (rnd() * 2 - 1) * 9, a[2]]; else if (r < 0.3) { const L = 0.3 + rnd() * 4, th = rnd() * 6.283; b = [a[0] + Math.sin(th) * L, a[1] + (rnd() - 0.5) * 0.6, a[2] + Math.cos(th) * L]; } else if (r < 0.34) b = [a[0] + (rnd() < 0.5 ? 0 : (rnd() - 0.5) * 30), a[1], a[2] + (rnd() - 0.5) * 30];
    const shrink = [0.02, 0.02, 0.02, 0, 0.05, 0.3, -0.05][Math.floor(rnd() * 7)], through = rnd() < 0.5 ? BoxFlag.Glass : 0, wantPanes = through === 0 && rnd() < 0.5, withHoles = rnd() < 0.4;
    hang(RUSTED ? (withHoles ? realOpen : null)   // (the game's own rule over the glass as it was broken above, or no rule at all: every pane shut)
      : withHoles ? (bi: number, p: Vec3) => { sink.push(`${bi}:${p[0]},${p[1]},${p[2]}`); return holes(bi, p); } : realOpen);
    const pn: { bi: number; t: number }[] = [], pr: { bi: number; t: number }[] = [];
    let here = false; const ph: { bi: number; t: number }[] = [];
    if (RUSTED) { sink = []; here = col.segmentBlockedHere(a, b, shrink, through, wantPanes ? ph : null); if (col.mirror.owes) col.mirror.again(col.boxes, col.glassWalkable); if (col.mirror.lines) inRust++; }
    asked.length = 0; askedRef.length = 0; sink = asked; const now = col.segmentBlocked(a, b, shrink, through, wantPanes ? pn : null); const nowD = col.segmentBlockedDynamic(a, b), nowS = col.segmentBlockedDynamic(a, b, BoxFlag.SoundSeal);
    sink = askedRef; const was = refBlocked(col, a, b, shrink, through, wantPanes ? pr : null); const wasD = refDynamic(col, a, b), wasS = refDynamic(col, a, b, BoxFlag.SoundSeal);
    if (now) blockedN++; paneN += pn.length;
    if (RUSTED && (here !== now || (!now && (ph.length !== pn.length || ph.some((p, k) => p.bi !== pn[k].bi || !Object.is(p.t, pn[k].t)))))) { diff++; diffL++; first ||= `a ${a.map(n => n.toFixed(4))} b ${b.map(n => n.toFixed(4))} shrink ${shrink} through ${through}: this file's walk ${here} with ${ph.length} pane(s), the module's ${now} with ${pn.length}`; }
    if (RUSTED && now) { pn.length = pr.length = 0; asked.length = askedRef.length = 0; }   // (blocked: the list and the questions asked on the way to the wall are nobody's answer)
    if (RUSTED) {   // THE RAY along the same segment — short of its end, to it, or past it, and now and then (`long`) for two thousand or a million kilometres, walked as far as the level reaches (one straight up or down among them: a single sample) — by the module and by this file's own walk
      const d = v3.sub(b, a), L = v3.len(d);
      if (L > 1e-4) { const rd = v3.scale(d, 1 / L), plan = Math.hypot(rd[0], rd[2]), long = (plan > 0.05 || plan === 0) && rnd() < 0.04, tm = long ? [2e6, 1e9][Math.floor(rnd() * 2)] : L * [1, 1, 0.4, 2.5, 6][Math.floor(rnd() * 5)], all = col.dynamicBoxes, out = all.length > 1 && rnd() < 0.12 ? Math.floor(rnd() * all.length) : -1;
        sink = [];
        if (out >= 0) { col.dynamicBoxes = all.filter((_: unknown, j: number) => j !== out); RAYED.swapped++; }
        const hereR = col.raycastHere(a, rd, tm), n0 = RAYS.n, nowR = col.raycast(a, rd, tm); col.dynamicBoxes = all;
        RAYED.n++; if (RAYS.n > n0) RAYED.there++; if (long) RAYED.long++;
        if (nowR) { RAYED.hit++; if (nowR.index >= 1e6) RAYED.leaf++; else { const hb = boxes[nowR.index]; if (hb.round === true) { RAYED.round++; if (hb.rot) RAYED.roundT++; } else if (hb.cut && hb.cut.length) { RAYED.cut++; if (hb.rot) RAYED.cutT++; } else if (hb.rot) RAYED.tilted++; } }
        if ((hereR === null) !== (nowR === null) || (hereR && nowR && (hereR.index !== nowR.index || !Object.is(hereR.t, nowR.t) || !Object.is(hereR.n[0], nowR.n[0]) || !Object.is(hereR.n[1], nowR.n[1]) || !Object.is(hereR.n[2], nowR.n[2])))) {
          diff++; diffR++; first ||= `a ray from ${a.map(n => n.toFixed(4))} along ${rd.map((n: number) => n.toFixed(5))} for ${tm.toFixed(4)}${out >= 0 ? ` (leaf ${out} out of the list)` : ''}: this file's walk ${hereR ? `box ${hereR.index} at ${hereR.t} by ${hereR.n}` : 'nothing'}, the module ${nowR ? `box ${nowR.index} at ${nowR.t} by ${nowR.n}` : 'nothing'}`; } } }
    if (now !== was || nowD !== wasD || nowS !== wasS || pn.length !== pr.length || pn.some((p, k) => p.bi !== pr[k].bi || !Object.is(p.t, pr[k].t)) || (wantPanes && !RUSTED && (asked.length !== askedRef.length || asked.some((q, k) => q !== askedRef[k])))) {
      diff++; first ||= `a ${a.map(n => n.toFixed(4))} b ${b.map(n => n.toFixed(4))} shrink ${shrink} through ${through}: blocked ${was}→${now}, leaves ${wasD}→${nowD}, sealed ${wasS}→${nowS}, panes ${pr.length}→${pn.length}, glassOpen asked ${askedRef.length}→${asked.length}`; }
  }
  hang(realOpen);
  let circN = 0, circHit = 0;
  if (RUSTED) {
    const walkWas = col.glassWalkable; col.glassWalkable = (bi: number) => (bi * 2654435761 >>> 28) < 5; if (!UNSAID) col.wrote();   // (a made-up set of panes walked through, the same for both: said, as the rule says a pane's change — and on the red path left unsaid like the re-shape, or the circles would be asked of a copy made good again and their row would have no red path)
    const oa = new Int32Array(64), ob = new Int32Array(64), small = new Int32Array(3), smallB = new Int32Array(3);
    for (let i = 0; i < (N >> 2); i++) {
      const at = rnd() < 0.5 ? anywhere() : rnd() < 0.1 ? (() => { const b = boxes[Math.floor(rnd() * boxes.length)]; return [b.c[0], b.c[1], b.c[2]] as Vec3; })() : onBox();
      const r = [0.05, 0.18, 0.3, 0.32, 0.45, 0.6, 1.4][Math.floor(rnd() * 7)], y0 = [0, 0.05, 0.2, 0.22, 0.6, 1.2][Math.floor(rnd() * 6)], y1 = y0 + [0.1, 0.9, 1.7, 3][Math.floor(rnd() * 4)], its = [1, 3, 3, 3, 6][Math.floor(rnd() * 5)];
      const p: Vec3 = [at[0], at[1], at[2]], q: Vec3 = [at[0], at[1], at[2]];
      const hit = col.collideCircle(p, r, y0, y1, its), hitH = col.collideCircleHere(q, r, y0, y1, its);
      const few = rnd() < 0.1, A = few ? small : oa, B = few ? smallB : ob; A.fill(-7); B.fill(-7);   // (a list of three: the gather that does not fit, and what it leaves behind)
      const t = col.circleTouches(at[0], at[2], r, y0, y1, A), tH = col.circleTouchesHere(at[0], at[2], r, y0, y1, B); let same = t === tH && A.every((v, k) => v === B[k]);
      A.fill(-7); B.fill(-7); const n = col.gather(at[0] - r, at[2] - r, at[0] + r, at[2] + r, y0, y1, A), nH = col.gatherHere(at[0] - r, at[2] - r, at[0] + r, at[2] + r, y0, y1, B); same &&= n === nH && A.every((v, k) => v === B[k]);
      circN++; if (hit) circHit++;
      if (hit !== hitH || !Object.is(p[0], q[0]) || !Object.is(p[2], q[2]) || !same) { diff++; diffC++; first ||= `a circle of ${r} at ${at.map(v => v.toFixed(4))} over ${y0} to ${y1}, ${its} passes: this file's ends ${q[0]}, ${q[2]} (touched ${hitH}; ${tH} touching, ${nH} gathered), the module's ${p[0]}, ${p[2]} (touched ${hit}; ${t} touching, ${n} gathered)`; }
    }
    if (!col.mirror.circles && !UNSAID) { diff++; first ||= 'the circles were not asked of the module (its copy fell)'; }
    col.glassWalkable = walkWas;
  }
  if (RUSTED && inRust < N * 0.9 && !UNSAID) { diff++; first ||= `only ${inRust} of ${N} questions could go to the module (a level too big for its tables, or the copy fell)`; }
  if (argv.includes('--bench')) {   // the same sight-line-like segments through both, interleaved in blocks (eye height, 2 to 25 m, inside the level)
    const segs: [Vec3, Vec3][] = []; for (let i = 0; i < 20000; i++) { const a: Vec3 = [1 + rnd() * (col.W * col.cs - 2), 1.5, 1 + rnd() * (col.H * col.cs - 2)], L = 2 + rnd() * 23, th = rnd() * 6.283; segs.push([a, [Math.min(col.W * col.cs - 1, Math.max(1, a[0] + Math.sin(th) * L)), 0.9 + rnd() * 0.8, Math.min(col.H * col.cs - 1, Math.max(1, a[2] + Math.cos(th) * L))]]); }
    if (RUSTED) col.tickOpen();
    let tNew = 0, tOld = 0, n = 0; for (let rep = 0; rep < 10; rep++) { let t = performance.now(); for (const [a, b] of segs) if (col.segmentBlocked(a, b)) n++; tNew += performance.now() - t; t = performance.now(); for (const [a, b] of segs) if (refBlocked(col, a, b)) n--; tOld += performance.now() - t; }
    log(`${''.padEnd(20)} bench: ${(tOld / 200).toFixed(2)} µs a segment as it was → ${(tNew / 200).toFixed(2)} µs now (×${(tOld / tNew).toFixed(1)}; answers ${n === 0 ? 'equal' : 'DIFFER'})`);
  }
  if (RUSTED) col.tickShut();
  log(`${file.padEnd(20)} ${N} segments${RUSTED ? ` (${inRust} to the Rust module)` : ''} · ${boxes.length} statics · ${col.dynamicBoxes.length} leaves · ${broken.length ? `${broken.length} broken into ${broken.reduce((n, B) => n + B.pieces.length, 0)} pieces · ` : ''}${blockedN} blocked · ${paneN} panes noted · ${RUSTED ? `${PANED.live} broken panes in the copy · ${circN} circles, ${circHit} pushed out · ` : ''}${diff ? `FAIL — ${diff} differ${RUSTED ? ` (the module's: ${diffL} lines, ${diffR} rays, ${diffC} circles)` : ''}, first: ${first}` : 'PASS — every answer and every pane the same'}`);
  if (diff) bad++; ever.lines += diffL; ever.rays += diffR; ever.circles += diffC;
}
const blind = UNSAID ? (['lines', 'rays', 'circles'] as const).filter(k => !ever[k]) : [];   // (the red path must turn each row's answers red on some level, as sense-ident's must each sense)
if (RUSTED) log('SEGMENT-IDENT ' + JSON.stringify(ever));
if (RUSTED) { const R = RAYED, thin = (['leaf', 'tilted', 'round', 'cut', 'roundT', 'cutT', 'swapped'] as const).filter(k => R[k] === 0);
  log(`rays: ${R.n} asked, ${R.there} the module's (the rest the host's: a ray of no length) · ${R.hit} hit: ${R.leaf} a leaf, ${R.tilted} a tilted static, ${R.round} a round piece (${R.roundT} of them on a tumbled one), ${R.cut} a cut one (${R.cutT}) · ${R.swapped} with a leaf out of the list · ${R.long} of two thousand kilometres or longer${RUSTED && R.long === 0 ? ' — NOT A FAIR RUN: no long ray was asked' : ''}`);
  if (RUSTED && R.long === 0) bad++;
  if (!UNSAID && files.length >= 4 && (thin.length || R.there < R.n * 0.9)) { log(`    NOT A FAIR RUN OF RAYS: ${thin.length ? `never met ${thin.join(', ')}` : 'under nine in ten were the module\'s'}`); bad++; } }
log(bad ? `=== segment-ident${RUSTED ? ' --rust' : ''}: ${bad} level(s) FAIL${UNSAID ? blind.length ? ` — AND THE RED PATH IS NOT RED FOR ${blind.join(', ')}` : ' — lines, rays and circles each differed somewhere, as the red path must' : ''} ===` : UNSAID ? `=== segment-ident --rust: ALL PASS — AND THE RED PATH IS NOT RED FOR ${blind.join(', ')} ===` : `=== segment-ident${RUSTED ? ' --rust' : ''}: ALL PASS ===`);
process.exit(blind.length ? 3 : bad ? 1 : 0);
