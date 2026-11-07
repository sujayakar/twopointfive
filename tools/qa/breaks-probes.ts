// The 'breaks' probe family (src/game/breakTable.ts — walls, cover and what the destruction lane knocks loose break BY MATERIAL into cut pieces), run through probes.ts:
//   bun run tools/qa/probes.ts breaks | breaks-shapes | breaks-wall | breaks-again | breaks-overflow | breaks-caps | breaks-determinism [--level block-squads]
// CUT PIECES ARE A SETTLED RULE (scene/cut.ts CUT.on; 2026-11-07): nothing here asks the wreck with the cut switch off or writes it. Until then a probe `off` did, with
// an off run beside the on run in `wall`, `studs`, `furniture` and `look`; those runs, their lines and the furniture rule's pin in `wall` are out.
// Headless = the Proxy device: nothing is drawn. What is asserted is the TABLE and the BUDGET:
//   (shapes)       every shape of every row, a few hundred hashed draws each, is a valid fitted cut box (a solid, ≤ 8 planes, inside its box, the box tight to it) with the
//                  outline its row asks for (a slab 5 to 7 sides, a wedge 3, a strip's sides within 8° of its length …); a ragged end bites no deeper than its row
//                  allows and leaves the uncut box as the piece's own bounds; the same seeds give the same planes;
//   (wall)         a charge against a partition: the thrown pieces are cut slabs inside the blast's share (7 at pool 64, 4 at pool 32), a heap is dealt
//                  at rest on the far side at the wall's foot, the pieces either side of the hole carry a ragged end (≤ 4 planes, bite ≤ 12 cm), chips by the row's
//                  count;
//   (again)        level B, one strong event only: a plasterboard slab landing fast snaps in two, slowly it does not, a kick over the row's line snaps it, a second
//                  generation never breaks; wood never on landing but by a kick; half the tiles; a binder never; a mug landing hard is gone;
//   (overflow)     the ladder: an evicted piece crumbles into chips of its row, a refused one becomes chips, with more than half the pool moving nothing breaks a
//                  second time (it goes to particles), the pool never exceeds its cap;
//   (caps)         every breakable of the level blown end to end at pool 64 and at pool 32: pool ≤ cap on every frame, standing pieces ≤ the stand-in cap and each
//                  ≤ 4 planes, every chunk's record valid, and once it has settled the LOAD NUMBER (pieces moving + particle records waiting) is under the pool size;
//   (determinism)  the same seeded wreck twice → the same pieces, planes and counts.
import { standUp, ROOT, fake } from './headless.ts';
import { readFileSync } from 'node:fs';
const { v3, quat } = await import(`${ROOT}/src/math/vec.ts`);
const { Particles } = await import(`${ROOT}/src/render/particles.ts`);
const { WALLS, WALL_STATS, WALL_LOG } = await import(`${ROOT}/src/game/wallbreak.ts`);
const { BoxFlag, makeBox } = await import(`${ROOT}/src/scene/boxes.ts`);
const { PLAYER_ID } = await import(`${ROOT}/src/game/consts.ts`);
const { loadLevel } = await import(`${ROOT}/src/scene/levelLoad.ts`);
const { parseLevelData } = await import(`${ROOT}/src/scene/levelData.ts`);
const CUT = await import(`${ROOT}/src/scene/cut.ts`);
const { cutVerts, cutBounds, cutRecord, MAX_CUT_PLANES } = CUT;
const BT = await import(`${ROOT}/src/game/breakTable.ts`);
const { BREAK, BREAKS, BREAK_STATS, shapeBox, raggedCut, slabOutline, wedgeOutline, snappedOutline, stripOutline } = BT;
const { hash01 } = await import(`${ROOT}/src/game/debris.ts`);

type Vec3 = [number, number, number];
const DT = 1 / 60;
const LEVEL = (() => { const i = process.argv.indexOf('--level'); return i >= 0 ? process.argv[i + 1] : 'block-squads'; })();
function seeded(a: number) { return () => { a |= 0; a = (a + 0x6D2B79F5) | 0; let t = Math.imul(a ^ (a >>> 15), 1 | a); t = (t + Math.imul(t ^ (t >>> 7), 61 | t)) ^ t; return ((t ^ (t >>> 14)) >>> 0) / 4294967296; }; }
const f2 = (x: number) => x.toFixed(2), f3 = (x: number) => x.toFixed(3);
type Verdict = (ok: boolean, label: string, detail: string) => void;
function family(title: string): { verdict: Verdict; done: () => boolean } {
  console.log(`\n=== ${title} ===`); let all = true;
  return { verdict: (ok, label, detail) => { all &&= ok; console.log(`- ${ok ? 'PASS' : 'FAIL'} — ${label}\n    ${detail}`); }, done: () => { const ok = all; console.log(`=== ${title.split(':')[0]}: ${ok ? 'ALL PASS' : 'FAILURES above'} ===`); return ok; } };
}
const zero = (o: Record<string, number>) => { for (const k of Object.keys(o)) o[k] = 0; };
const WALLS0 = JSON.parse(JSON.stringify(WALLS));
const restoreKnobs = () => { for (const k of Object.keys(WALLS0)) (WALLS as any)[k] = JSON.parse(JSON.stringify(WALLS0[k])); };

/** the harness Game on the loud level at `chaos`, a Particles attached, everyone deaf, blind and out of the way */
async function stage(seed: number, chaos: number, cap = 64) {
  Math.random = seeded(seed); restoreKnobs();
  const lv = LEVEL === 'office' ? undefined : loadLevel(parseLevelData(readFileSync(`${ROOT}/levels/${LEVEL}.json`, 'utf8')));
  const W = await standUp(lv ? { level: lv, dress: true } : { dress: true }); const { game, engine } = W;   // (the rows up: the debris pool is a dressing row, and a host that draws nothing holds no pool)
  engine.particles = new Particles(fake('device'), fake('sceneLayout')); engine.settings.gpuParticles = true; engine.settings.debrisCap = cap;
  game.godMode = true; game.aiEnabled = false; game.player.visibility = 0.02; game.tune.chaos = chaos; zero(WALL_STATS); WALL_LOG.length = 0; zero(BREAK_STATS);
  game.destr.breaks.look = null;   // (the family judges the table ALONE: a look-only wall dressing a merged tree attaches is put aside — breaks-look sets its own)
  for (let i = 0; i < 3; i++) W.step(DT);
  for (const gd of game.guards) { gd.hold = true; gd.pinned = true; gd.heardUpTo = 1e12; gd.flashlight.enabled = false; }
  return { W, game, engine };
}
/** a partition column with open floor on both sides, away from doors and the wall's ends (wallbreak-probes.ts's pick) */
function pickWall(game: any, skip = 0): { id: number; box: any; alongX: boolean; mid: Vec3; n: Vec3; a: Vec3; b: Vec3 } | null {
  const nav = game.col.nav; const free = (p: Vec3) => !nav.blocked[nav.idx(Math.floor(p[0] / nav.cell), Math.floor(p[2] / nav.cell))];
  let seen = 0;
  for (const w of game.destr.walls.all()) {
    if (w.kind !== 'wall') continue; const b = w.box; const alongX = b.h[0] >= b.h[2]; const L = (alongX ? b.h[0] : b.h[2]);
    if (L < 1.6) continue;
    const nCol = Math.min(32, Math.max(1, Math.round(2 * L / WALLS.col))), wCol = 2 * L / nCol;
    for (let k = 0; k < nCol; k++) {
      const u = -L + (k + 0.5) * wCol; if (u < -L + 1.4 || u > L - 1.4) continue;
      const mid: Vec3 = alongX ? [b.c[0] + u, 0, b.c[2]] : [b.c[0], 0, b.c[2] + u]; const n: Vec3 = alongX ? [0, 0, 1] : [1, 0, 0];
      const a = v3.mad(mid, n, 1.2), bb = v3.mad(mid, n, -1.2);
      if (!free(a) || !free(bb) || !free(v3.mad(mid, n, 0.7)) || !free(v3.mad(mid, n, -0.7))) continue;
      if (game.col.solidBlocked([a[0], 1.3, a[2]], [mid[0] + n[0] * 0.2, 1.3, mid[2] + n[2] * 0.2]) || game.col.solidBlocked([bb[0], 1.3, bb[2]], [mid[0] - n[0] * 0.2, 1.3, mid[2] - n[2] * 0.2])) continue;
      if (game.doors.list.some((d: any) => Math.hypot(d.frameCentre[0] - mid[0], d.frameCentre[2] - mid[2]) < 2.2)) continue;
      if (seen++ < skip) break;
      return { id: w.id, box: b, alongX, mid, n, a, b: bb };
    }
  }
  return null;
}
const standing = (game: any): any[] => { const out: any[] = []; const a = game.destr.walls.authored; for (let i = a; i < game.col.boxes.length; i++) { const b = game.col.boxes[i]; if (!(b.flags & BoxFlag.NoShadow)) out.push(b); } return out; };
/** a cut record's faults (null = a valid record on a box it leaves tight): planes ≤ 8, unit normals, a solid left, the box fitted to it when `fitted` */
function recordFault(h: Vec3, cut: any[] | undefined, fitted: boolean): string | null {
  if (!cut) return null;
  if (!cut.length) return 'an empty array (should be undefined)';
  if (cut.length > MAX_CUT_PLANES) return `${cut.length} planes`;
  for (const p of cut) { const l = Math.hypot(p.n[0], p.n[1], p.n[2]); if (Math.abs(l - 1) > 1e-6 || !isFinite(p.d)) return `plane not unit / finite (${l})`; }
  const r = cutRecord(h, cut); if (r.empty) return 'nothing left of the box';
  if ((r.cut?.length ?? 0) !== cut.length) return `${cut.length - (r.cut?.length ?? 0)} redundant plane(s)`;
  if (cutVerts(h, cut).length < 4) return 'no solid';
  if (fitted) { const B = cutBounds(h, cut); for (let a = 0; a < 3; a++) if (Math.abs(B.max[a] - h[a]) > 1e-5 + 1e-4 * h[a] || Math.abs(B.min[a] + h[a]) > 1e-5 + 1e-4 * h[a]) return `box not fitted on axis ${a} (${f3(B.min[a])}‥${f3(B.max[a])} in ±${f3(h[a])})`; }
  return null;
}
/** how deep the planes bite into the face `axis`·`sign` of a box (the furthest any cut corner sits behind that face) */
function biteOf(h: Vec3, cut: any[]): number { let worst = 0; const B = cutBounds(h, cut); for (let a = 0; a < 3; a++) worst = Math.max(worst, h[a] - B.max[a], B.min[a] + h[a]); return worst; }

// ================================================================ shapes
export async function breaksShapes(): Promise<boolean> {
  const F = family('breaks-shapes: every shape of every row is a valid cut box with the outline its row asks for');
  const stream = (seed: number) => { let k = seed * 7919; return () => hash01(k++ * 2246822519); };
  const sides = (o: [number, number][]) => o.length; const DEG = 180 / Math.PI;
  // outlines
  { let bad = ''; let minS = 99, maxS = 0, worstAng = 0;
    for (let s = 1; s <= 400 && !bad; s++) { const r = stream(s); const L = 0.08 + 0.2 * r(), S = L * (0.5 + 0.5 * r()); const tears = 1 + (s % 3);
      const o = slabOutline(L, S, tears, r); minS = Math.min(minS, sides(o)); maxS = Math.max(maxS, sides(o)); if (sides(o) !== 4 + tears) bad = `seed ${s}: ${sides(o)} sides for ${tears} tears`;
      let area = 0; for (let i = 0; i < o.length; i++) { const p = o[i], q = o[(i + 1) % o.length], t = o[(i + 2) % o.length]; area += p[0] * q[1] - q[0] * p[1]; const cr = (q[0] - p[0]) * (t[1] - q[1]) - (q[1] - p[1]) * (t[0] - q[0]); if (cr < -1e-9) bad = `seed ${s}: not convex at ${i}`; }
      if (!(area > 0.5 * 4 * L * S)) bad ||= `seed ${s}: less than half the rectangle left`;
      for (let i = 0; i < o.length; i++) { const p = o[i], q = o[(i + 1) % o.length]; const du = Math.abs(q[0] - p[0]), dv = Math.abs(q[1] - p[1]); if (du > 1e-9 && dv > 1e-9) worstAng = Math.max(worstAng, Math.abs(Math.atan2(dv, du) * DEG - 37.5)); } }
    F.verdict(!bad && minS === 5 && maxS === 7, 'slab: the rectangle with 1 to 3 corners off — 5 to 7 sides, convex, most of it left', bad || `400 draws: ${minS} to ${maxS} sides; torn edges within ${f2(worstAng)}° of 37.5° (15 to 60° off the long side, steeper only where the short side caps it)`); }
  { let bad = ''; let worst = 0, lo = 99, hi = 0; const areaOf = (o: [number, number][]) => { let a = 0; for (let i = 0; i < o.length; i++) { const p = o[i], q = o[(i + 1) % o.length]; a += p[0] * q[1] - q[0] * p[1]; } return Math.abs(a) / 2; };
    for (let s = 1; s <= 400 && !bad; s++) { const L = 0.1 + 0.2 * hash01(s * 3), S = L * (0.3 + 0.7 * hash01(s * 5)); const o = wedgeOutline(L, S, stream(s + 1000), false), o2 = wedgeOutline(L, S, stream(s + 1000), true);
      lo = Math.min(lo, o.length, o2.length); hi = Math.max(hi, o.length, o2.length); if (o.length < 3 || o.length > 5) bad = `seed ${s}: ${o.length} sides`;
      if (Math.abs(areaOf(o) + areaOf(o2) - 4 * L * S) > 1e-9) bad ||= `seed ${s}: the two halves are not the panel (${areaOf(o)} + ${areaOf(o2)} of ${4 * L * S})`;
      // the one edge that lies on no side of the rectangle is the cut: within 15° of a diagonal
      let cuts = 0; for (let i = 0; i < o.length; i++) { const p = o[i], q = o[(i + 1) % o.length]; const du = Math.abs(q[0] - p[0]), dv = Math.abs(q[1] - p[1]); if (du > 1e-9 && dv > 1e-9) { cuts++; worst = Math.max(worst, Math.abs(Math.atan2(dv, du) - Math.atan2(S, L)) * DEG); } }
      if (cuts !== 1) bad ||= `seed ${s}: ${cuts} cut edges`; }
    F.verdict(!bad && worst <= 15 + 1e-6, 'wedge: one cut near the diagonal, the two flips the two halves of the panel', bad || `400 draws: ${lo} to ${hi} sides, the cut at most ${f2(worst)}° off the diagonal (≤ 15)`); }
  { let bad = ''; let worst = 0, lo = 99, hi = 0;
    for (let s = 1; s <= 400 && !bad; s++) { const r = stream(s + 2000); const L = 0.15 + 0.15 * r(), S = L * (0.16 + 0.17 * r()); const o = stripOutline(L, S * 0.45, r); if (o.length !== 4) bad = `seed ${s}: ${o.length} sides`;
      for (const [i, j] of [[0, 1], [2, 3]]) worst = Math.max(worst, Math.abs(Math.atan2(Math.abs(o[j][1] - o[i][1]), Math.abs(o[j][0] - o[i][0]))) * DEG);
      const tip = (o[2][1] - o[1][1]) / (2 * S * 0.45); if (!(tip > 0)) bad = `seed ${s}: the tip closed (${tip})`;
      const e = snappedOutline(L, S, r); lo = Math.min(lo, e.length); hi = Math.max(hi, e.length); if (e.length < 4 || e.length > 6) bad ||= `seed ${s}: snapped end with ${e.length} sides`; }
    F.verdict(!bad && worst <= 8 + 1e-6, 'strip: both long sides within 8° of its length, closing to a tip; snapped end: 4 to 6 sides', bad || `400 draws: long sides ≤ ${f2(worst)}° off the length; snapped ends ${lo} to ${hi} sides`); }
  // every row's cell shapes as fitted cut boxes
  { let bad = ''; let n = 0, planes = 0, maxP = 0, plain = 0;
    for (const [name, row] of Object.entries(BREAK) as [string, any][]) for (const shape of [...new Set([...row.cell, 'slab', 'wedge', 'snapped', 'strip', 'lump', 'panel'])]) for (let s = 1; s <= 60 && !bad; s++) {
      const r = stream(s + n * 13); const L = (row.span[0] + (row.span[1] - row.span[0]) * r()) / 2, S = L * (row.slim[0] + (row.slim[1] - row.slim[0]) * r()) * (shape === 'strip' ? 0.45 : 1);
      for (const thin of [0, 1, 2]) { const sb = shapeBox(shape, thin, row.thick, L, S, s % 2 === 0, row, r, s % 3 === 0); n++;
        if (!sb) { bad = `${name} ${shape} seed ${s}: nothing left`; break; }
        if (Math.min(...sb.h) <= 0 || sb.h.some((x: number) => !isFinite(x))) { bad = `${name} ${shape} seed ${s}: box ${sb.h}`; break; }
        if (shape !== 'lump' && Math.abs(sb.h[thin] - row.thick) > 1e-9) { bad = `${name} ${shape}: thickness ${sb.h[thin]} for ${row.thick}`; break; }
        const f = recordFault(sb.h, sb.cut, true); if (f) { bad = `${name} ${shape} seed ${s} thin ${thin}: ${f}`; break; }
        if (shape === 'panel' && sb.cut) { bad = `${name}: a panel with a cut`; break; }
        if (sb.cut) { planes += sb.cut.length; maxP = Math.max(maxP, sb.cut.length); } else plain++; } }
    F.verdict(!bad, 'every row × shape × thin axis: a solid, ≤ 8 planes, none redundant, the box fitted to the cut', bad || `${n} boxes over ${Object.keys(BREAK).length} rows: mean ${f2(planes / Math.max(1, n - plain))} planes on a cut one, most ${maxP}; ${plain} uncut (panels); ${BREAK_STATS.dropped} planes dropped for the 8-plane limit`); }
  // ragged ends
  { let bad = ''; let n = 0, worst = 0, maxP = 0;
    for (const [name, row] of Object.entries(BREAK) as [string, any][]) { if (!row.ragged) continue;
      for (let s = 1; s <= 300 && !bad; s++) { const r = stream(s + 5000); const h: Vec3 = [0.25 + 1.5 * r(), 0.35 + 0.9 * r(), 0.05 + 0.03 * r()];
        const faces: any[] = [{ axis: 0, sign: s % 2 ? 1 : -1, lat: s % 7 === 0 ? [2, 1] : [1, 2], seed: s * 31 }]; if (s % 3 === 0) faces.push({ axis: 0, sign: s % 2 ? -1 : 1, lat: [1, 2], seed: s * 57 }); if (s % 4 === 0) faces.push({ axis: 1, sign: 1, lat: [0, 2], seed: s * 91 }); if (s % 5 === 0) faces[0].lo = -h[1] + 0.7, faces[0].lat = [1];
        const cut = raggedCut(row, h, faces); n++; if (!cut) continue;
        const f = recordFault(h, cut, false) ?? (biteOf(h, cut) > row.ragged.bite + 1e-6 ? `the box's bounds taken back ${f3(biteOf(h, cut))} m` : null); if (f) { bad = `${name} seed ${s}: ${f}`; break; }
        maxP = Math.max(maxP, cut.length); if (cut.length > BREAKS.maxStandingPlanes) { bad = `${name} seed ${s}: ${cut.length} planes on a standing piece`; break; }
        const again = raggedCut(row, h, faces); if (JSON.stringify(again) !== JSON.stringify(cut)) { bad = `${name} seed ${s}: not the same planes twice`; break; }
        // the deepest any corner of the piece is taken back from a face it used to reach
        let deep = 0; const V = cutVerts(h, cut); for (const p of cut) for (const v of V) if (Math.abs(p.n[0] * v[0] + p.n[1] * v[1] + p.n[2] * v[2] - p.d) < 1e-7) { const ax = [0, 1, 2].reduce((m, a) => Math.abs(p.n[a]) > Math.abs(p.n[m]) ? a : m, 0); deep = Math.max(deep, h[ax] - Math.abs(v[ax])); }
        worst = Math.max(worst, deep); if (deep > row.ragged.bite + 1e-6) { bad = `${name} seed ${s}: bites ${f3(deep)} m (row allows ${row.ragged.bite})`; break; } } }
    F.verdict(!bad && worst <= 0.12 + 1e-6, 'ragged end: ≤ 4 planes, bites no deeper than its row (≤ 12 cm) at any corner or on any side of the uncut box, stable for a seed', bad || `${n} ends over the rows that have one: deepest bite ${f3(worst)} m, most ${maxP} planes`); }
  void biteOf;
  return F.done();
}

// ================================================================ a blown wall
export async function breaksWall(): Promise<boolean> {
  const F = family(`breaks-wall: a charge against a partition with cut pieces on — slabs, a heap, ragged ends, the same boxes underfoot (${LEVEL})`);
  const run = async (cap: number) => {
    const S = await stage(11, 0.5, cap); const { W, game } = S;
    const pick = pickWall(game); if (!pick) return null;
    const { mid, n } = pick; const at: Vec3 = [mid[0] + n[0] * 0.3, 0.3, mid[2] + n[2] * 0.3];
    const open0 = new Map<number, number>(game.destr.walls.all().map((w: any) => [w.id, w.open]));
    // (until 2026-11-07 the furniture rule was written off around this charge; it is settled and the write was refused. Through the hole the charge razes what
    //  stands behind the wall — on the block a racking bay and its binders — and at the small pool those pieces can take the slot of the wall's own heap slab)
    const res = game.destr.blast(at, { who: PLAYER_ID });
    W.step(DT);   // (the blast's pieces reach the tools' copy of the game with the next step)
    const born = game.destr.debris.list.map((c: any) => ({ c, rest0: c.rest, side: (c.box.c[0] - mid[0]) * n[0] + (c.box.c[2] - mid[2]) * n[2], y0: c.box.c[1], vn: c.v[0] * n[0] + c.v[2] * n[2] }));
    const chips0 = BREAK_STATS.chips, cells0 = game.destr.walls.all().find((w: any) => w.id === pick.id).open;
    // (what ELSE of the wall engine's the same charge opened — breakable cover in the burst's own reach: its cells' chips are its own row's)
    const also: { surf: string; cells: number }[] = game.destr.walls.all().filter((w: any) => w.id !== pick.id && w.open > (open0.get(w.id) ?? 0)).map((w: any) => ({ surf: w.surf, cells: w.open - (open0.get(w.id) ?? 0) }));
    let peak = 0; for (let f = 0; f < 240; f++) { W.step(DT); peak = Math.max(peak, game.destr.debris.list.length); }
    const out = { res, born, chips0, cells0, also, peak, pick, game, stats: { ...BREAK_STATS }, wstats: { ...WALL_STATS }, standing: standing(game), share: game.destr.breaks.share(), surf: game.destr.walls.all().find((w: any) => w.id === pick.id).surf };
    return out;
  };
  const on = await run(64), on32 = await run(32);
  if (!on || !on32) { F.verdict(false, 'a partition with floor both sides', 'none found'); return F.done(); }
  const row = BREAK[BT.matOfSurf(on.surf)] as any;
  F.verdict(on.res.walls >= 1 && on.res.opened > 0, 'the charge holed the wall', `wall #${on.pick.id} (${on.surf}): ${on.wstats.holes} hole(s), nav cells opened ${on.res.opened}`);
  // thrown + heap inside the share
  for (const [tag, r] of [['pool 64', on], ['pool 32', on32]] as [string, any][]) {
    const heap = r.born.filter((b: any) => b.rest0), thrown = r.born.filter((b: any) => !b.rest0 && b.c.kind === 'chunk' && b.c.mat && Math.abs(b.side) < 0.2);   // born in the wall's own thickness: a piece of furniture the same charge broke is not the wall's
    if (tag === 'pool 64') F.verdict(thrown.length > 0 && thrown.length <= r.share.thrown && heap.length === r.stats.heap && heap.length <= r.share.heap && heap.length >= 1, `${tag}: thrown wall pieces and the dealt heap inside the blast's share`,
      `${thrown.length} thrown (share ${r.share.thrown}), ${heap.length} dealt at rest (share ${r.share.heap}); the pool peaked at ${r.peak} / ${r.game.destr.debris.cap()}; refused ${r.stats.refused}`);
    F.verdict(thrown.every((b: any) => b.vn < -0.2), `${tag}: every thrown piece leaves through the wall's face away from the charge`, `speeds along the wall's normal ${thrown.map((b: any) => f2(b.vn)).join(' ')} m/s (charge on +)`);
    const far = heap.every((b: any) => b.side < -0.02), low = heap.every((b: any) => b.y0 < 0.25), still = heap.every((b: any) => b.c.rest && Math.abs(b.c.box.c[1] - b.y0) < 1e-6 || !r.game.destr.debris.list.includes(b.c));
    F.verdict(far && low && still, `${tag}: the heap lies at the wall's foot on the far side and stays put`, `sides ${heap.map((b: any) => f2(b.side)).join(' ')} m from the wall line (charge on +), heights ${heap.map((b: any) => f2(b.y0)).join(' ')} m`);
    const wallBorn = r.born.filter((b: any) => b.rest0 || Math.abs(b.side) < 0.2);   /* the wall's own, as above */
    let bad = ''; let cutN = 0, planes = 0; for (const b of wallBorn) { if (!b.c.mat) { bad = 'a piece with no material row'; break; } const f = recordFault(b.c.box.h, b.c.box.cut, true); if (f) { bad = f; break; } if (b.c.box.cut) { cutN++; planes += b.c.box.cut.length; } }
    F.verdict(!bad && cutN >= Math.min(2, wallBorn.length), `${tag}: the pieces are cut boxes with valid, fitted records`, bad || `${cutN} of ${wallBorn.length} carry a cut (${planes} planes); row ${BT.matOfSurf(r.surf)}`);
  }
  // the tint toward the core
  { const paint = on.pick.box.albedo, core = BT.coreOf(row, paint); const want = v3.lerp(paint, core, BREAKS.coreTint); const c = on.born.find((b: any) => b.c.kind === 'chunk'); const got = c ? c.c.box.albedo : [0, 0, 0];
    F.verdict(!!c && v3.dist(got, want) < 1e-6 && v3.dist(c.c.mat.paint, paint) < 1e-9, 'a piece shows the stuff inside: its paint taken toward the core colour, the paint remembered for its chips', `paint ${paint.map(f2)} → piece ${got.map(f2)} (core ${core.map(f2)}, ${BREAKS.coreTint} of the way)`); }
  // cut faces show the inside of the material
  { const want = BT.faceCore(row, on.pick.box.albedo); const pcs = on.born.filter((b: any) => b.rest0 || Math.abs(b.side) < 0.2).map((b: any) => b.c.box).filter((b: any) => b.cut), ends = on.standing.filter((b: any) => b.cut);
    const ok = (b: any) => want ? !!b.cutCore && v3.dist(b.cutCore, want) < 1e-6 : !b.cutCore;
    F.verdict(pcs.length > 0 && pcs.every(ok) && ends.every(ok), 'every cut face shows the row\'s core colour (pieces and standing ends)',
      `${pcs.length} pieces + ${ends.length} standing ends, core ${want ? want.map(f2) : 'the box\'s own'}`); }
  // ragged ends on what stands
  { const pieces = on.standing.filter((b: any) => b.name !== 'stud'), studs = on.standing.length - pieces.length; const withCut = pieces.filter((b: any) => b.cut); let bad = ''; let worst = 0, maxP = 0;
    for (const b of withCut) { const f = recordFault(b.h, b.cut, false) ?? (biteOf(b.h, b.cut) > 0.12 + 1e-6 ? `bounds taken back ${f3(biteOf(b.h, b.cut))} m` : null); if (f) { bad = f; break; } maxP = Math.max(maxP, b.cut.length); worst = Math.max(worst, biteOf(b.h, b.cut)); }
    F.verdict(!bad && withCut.length >= 2 && maxP <= BREAKS.maxStandingPlanes, 'the pieces either side of the hole carry a ragged end (≤ 4 planes each, no side of the uncut box taken back more than 12 cm)', bad || `${withCut.length} of ${pieces.length} standing pieces cut, most ${maxP} planes (+ ${studs} studs left in the opened fringe)`); }
  return F.done();
}

// ================================================================ studs
export async function breaksStuds(): Promise<boolean> {
  const F = family(`breaks-studs: a partition shot through over a standing sill keeps its metal uprights; a hole does not (${LEVEL})`);
  const run = async () => {
    const S = await stage(14, 0.5, 64); const { game, engine } = S; const pick = pickWall(game); if (!pick) return null;
    const walls = game.destr.walls as any; const B = walls.list.find((b: any) => b.id === pick.id); const N = pick.alongX ? B.nx : B.nz; const ci = (k: number, iy: number) => pick.alongX ? (iy * B.nz) * B.nx + k : (iy * B.nz + k) * B.nx;
    const u = pick.alongX ? pick.mid[0] - pick.box.c[0] : pick.mid[2] - pick.box.c[2]; const L = pick.alongX ? pick.box.h[0] : pick.box.h[2]; const k0 = Math.max(1, Math.min(N - 4, Math.floor((u + L) / (2 * L) * N) - 1));
    const studs = () => standing(game).filter((b: any) => b.name === 'stud'); const n0 = game.col.boxes.length;
    walls.openCells(B, [ci(k0, 1), ci(k0 + 1, 1), ci(k0 + 2, 1)], 'fire', PLAYER_ID, [pick.n[0], 0, pick.n[2]]);
    const port = studs().map((b: any) => ({ c: [...b.c], h: [...b.h], metal: b.metal })); const statics1 = game.col.boxes.length;
    // a round's ray across the port at chest height: passes between the studs, stops on one
    const mid = (k: number): Vec3 => pick.alongX ? [pick.box.c[0] - L + 2 * L * (k + 0.5) / N, 1.4, pick.box.c[2]] : [pick.box.c[0], 1.4, pick.box.c[2] - L + 2 * L * (k + 0.5) / N];
    const line = (k: number): Vec3 => pick.alongX ? [pick.box.c[0] - L + 2 * L * k / N, 1.4, pick.box.c[2]] : [pick.box.c[0], 1.4, pick.box.c[2] - L + 2 * L * k / N];
    const through = (p: Vec3) => !game.col.raycast(v3.mad(p, pick.n, 1), [-pick.n[0], 0, -pick.n[2]], 2);
    const passMid = through(mid(k0 + 1)), stopLine = !through(line(k0 + 1));
    walls.openCells(B, [ci(k0 + 1, 0)], 'fire', PLAYER_ID, [pick.n[0], 0, pick.n[2]]);
    const holed = studs().length;
    const same = game.col.boxes.length === engine.world.statics.length && game.col.boxes.every((b: any, i: number) => b === engine.world.statics[i]);
    game.restartEncounter(); const after = studs().length, n2 = game.col.boxes.length;
    return { port, statics1, n0, passMid, stopLine, holed, same, after, n2, sillTop: pick.box.c[1] + B.yb[1], top: pick.box.c[1] + B.yb[2] };
  };
  const on = await run();
  if (!on) { F.verdict(false, 'a partition with floor both sides', 'none found'); return F.done(); }
  F.verdict(on.port.length === 2 && on.port.every((b: any) => b.metal === true && Math.min(b.h[0], b.h[2]) <= 0.021 && Math.abs(b.c[1] + b.h[1] - on.top) < 1e-6 && b.c[1] - b.h[1] < on.sillTop), 'three columns opened above the sill: two studs stand on the column lines between them, sill to top',
    `${on.port.length} studs, ${on.port.map((b: any) => `${f2(b.c[1] - b.h[1])}‥${f2(b.c[1] + b.h[1])} m`).join(', ')} (sill top ${f2(on.sillTop)}, wall top ${f2(on.top)}); ${on.statics1 - on.n0} stand-ins added`);
  F.verdict(on.passMid && on.stopLine, 'a round passes between the studs and stops on one, as on a real frame', `mid-column ray ${on.passMid ? 'passes' : 'stops'}, column-line ray ${on.stopLine ? 'stops' : 'passes'}`);
  F.verdict(on.holed === 0 && on.same, 'the sill between them shot out: the studs either side of the gap go with it; the picture\'s list is still the collision\'s', `${on.holed} studs after the sill went`);
  F.verdict(on.after === 0 && on.n2 === on.n0, 'a restart puts the wall back with no stand-in left', `${on.after} studs, ${on.n2} statics (authored ${on.n0})`);
  return F.done();
}


// ================================================================ the overflow ladder
/** FURNITURE: a plain piece handed to dress() takes its row's outline inside the box it had; sheet metal stays whole but for one kinked panel a blast. */
export async function breaksFurniture(): Promise<boolean> {
  const F = family(`breaks-furniture: a plain furniture piece takes its row's outline where it is; one metal panel a blast comes off kinked (${LEVEL})`);
  const run = async () => {
    const S = await stage(14, 0.5, 64); const { game } = S; const D = game.destr.debris, Bk = game.destr.breaks;
    const pick = pickWall(game); if (!pick) return null;
    D.clear(); zero(BREAK_STATS);
    const mk = (h: Vec3, k: number, mat: string) => { const c0: Vec3 = [pick.a[0] + (pick.alongX ? (k - 3) * 0.7 : 0), 1.2, pick.a[2] + (pick.alongX ? 0 : (k - 3) * 0.7)];
      const c = D.spawn('chunk', makeBox({ c: [...c0] as Vec3, h: [...h] as Vec3, albedo: [0.5, 0.4, 0.3] }), [0, 0, 0], [0, 0, 0], PLAYER_ID); if (c.mat) c.mat.mat = mat; Bk.dress(c); return { c, c0, h0: h }; };
    const out = { slat: mk([0.4, 0.012, 0.05], 0, 'wood'), slim: mk([0.3, 0.01, 0.02], 1, 'wood'), board: mk([0.22, 0.015, 0.2], 2, 'plasterboard'), lump: mk([0.1, 0.08, 0.09], 3, 'plasterboard'),
      cushion: mk([0.2, 0.08, 0.2], 4, 'fabric'), plate: mk([0.3, 0.008, 0.2], 5, 'metal'), plate2: mk([0.3, 0.008, 0.2], 6, 'metal') };
    const r = { out, stats: { ...BREAK_STATS } }; return r;
  };
  const on = await run(), on2 = await run();
  if (!on || !on2) { F.verdict(false, 'open floor to stand pieces on', 'none'); return F.done(); }
  const inside = (p: any) => { const b = p.c.box; let worst = 0; for (const v of cutVerts(b.h, b.cut)) for (let a = 0; a < 3; a++) worst = Math.max(worst, Math.abs(b.c[a] + v[a] - p.c0[a]) - p.h0[a]); return worst; };
  const cutN = (p: any) => p.c.box.cut?.length ?? 0;
  { const ps = [on.out.slat, on.out.slim, on.out.board, on.out.lump]; const bad = ps.map(p => recordFault(p.c.box.h, p.c.box.cut, true)).filter(Boolean); const worst = Math.max(...ps.map(inside));
    F.verdict(ps.every(p => cutN(p) >= 1) && !bad.length && worst < 1e-6 && cutN(on.out.lump) >= 3, 'a wooden slat, a slim one, a board slab and a blocky piece each take a cut outline, fitted, inside the box they were given',
      `planes: slat ${cutN(on.out.slat)}, slim ${cutN(on.out.slim)}, board ${cutN(on.out.board)}, lump ${cutN(on.out.lump)}; furthest outside the given box ${f3(worst * 1000)} mm; record faults ${bad.length ? bad.join('; ') : 'none'}`); }
  { const b = on.out.slim.c.box; const B = cutBounds(b.h, b.cut); const long = 0; const endW = (sign: number) => { let lo = 1e9, hi = -1e9; for (const v of cutVerts(b.h, b.cut)) if (Math.abs(v[long] - sign * b.h[long]) < 1e-4) { lo = Math.min(lo, v[2]); hi = Math.max(hi, v[2]); } return hi - lo; };
    const w0 = endW(-1), w1 = endW(1);
    F.verdict(Math.min(w0, w1) < 0.75 * Math.max(w0, w1), 'the slim one is a splinter: it tapers toward one end', `end widths ${f3(w0 * 1000)} and ${f3(w1 * 1000)} mm (bounds ${f3((B.max[0] - B.min[0]))} m long)`); }
  F.verdict(!on.out.cushion.c.box.cut && !on.out.cushion.c.mate, 'a cushion stays whole', `planes ${cutN(on.out.cushion)}`);
  { const c = on.out.plate.c, b = c.box, m = c.mate; const ok0 = !!m && cutN(on.out.plate) === 3 && m.cut?.length === 3 && !recordFault(b.h, b.cut, false) && !recordFault(m.h, m.cut, false);
    let both = 0, either = 0, n = 0; const inC = (cut: any[], p: number[]) => cut.every(q => q.n[0] * p[0] + q.n[1] * p[1] + q.n[2] * p[2] <= q.d + 1e-9);
    if (ok0) for (let i = 0; i < 60; i++) for (let j = 0; j < 60; j++) for (let k = 0; k < 40; k++) { const p = [(-1 + (2 * i + 1) / 60) * b.h[0], (-1 + (2 * j + 1) / 60) * b.h[1], (-1 + (2 * k + 1) / 40) * b.h[2]]; const a0 = inC(b.cut, p), a1 = inC(m.cut, p); n++; if (a0 || a1) either++; if (a0 && a1) both++; }
    const vol = either / Math.max(1, n) * 8 * b.h[0] * b.h[1] * b.h[2], want = 8 * 0.3 * 0.008 * 0.2;
    const top = (cut: any[]) => cut.find(q => q.n[1] > 0.5), t0 = ok0 ? top(b.cut) : null, t1 = ok0 ? top(m.cut) : null; const ang = t0 && t1 ? Math.acos(Math.min(1, t0.n[0] * t1.n[0] + t0.n[1] * t1.n[1] + t0.n[2] * t1.n[2])) * 180 / Math.PI : -1;
    F.verdict(ok0 && Math.abs(vol / want - 1) < 0.08 && both / Math.max(1, either) < 0.03 && ang >= BREAKS.kink.deg[0] - 0.01 && ang <= BREAKS.kink.deg[1] + 0.01 && Math.abs(b.h[0] - 0.3) < 1e-9 && Math.abs(b.h[2] - 0.2) < 1e-9,
      'the first metal plate comes off kinked: two halves of one plate in one box, meeting on the fold, the angle inside the row\'s range, no metal gained or lost',
      ok0 ? `halves ${f3(ang)}° apart (range ${BREAKS.kink.deg.join('‥')}); metal ${f3(vol * 1e6)} cm³ of ${f3(want * 1e6)}; overlap ${f3(100 * both / Math.max(1, either))} %; the box ${f3(b.h[1] * 2000)} mm thick over a ${f3(16)} mm plate` : `mate ${!!m}, planes ${cutN(on.out.plate)} / ${m?.cut?.length}`); }
  F.verdict(!on.out.plate2.c.box.cut && !on.out.plate2.c.mate && on.stats.kinked === 1, 'the next plate of the same blast stays a whole flat panel: metal bends once, it never breaks', `planes ${cutN(on.out.plate2)}, kinked ${on.stats.kinked}`);
  { const sig = (r: any) => JSON.stringify(Object.values(r.out).map((p: any) => [p.c.box.c, p.c.box.h, p.c.box.cut, p.c.mate?.cut]));
    F.verdict(sig(on) === sig(on2), 'two fresh games dress the same pieces the same way', `${sig(on).length} bytes of pose and planes compared`); }
  return F.done();
}



// ================================================================ the worst case
export async function breaksCaps(): Promise<boolean> {
  const F = family(`breaks-caps: every breakable of the level blown end to end stays inside the pools (${LEVEL})`);
  for (const cap of [64, 32]) {
    const S = await stage(15, 1, cap); const { W, game, engine } = S; const D = game.destr.debris;
    let blasts = 0, peak = 0, peakLoad = 0, bad = ''; const P = engine.particles as any; const births = cap >= 64 ? 384 : 192;   // (the page's frame deals the queue its births every frame: high 384, low 192)
    const step0 = W.step; W.step = (dt: number) => { step0(dt); P.prepare(dt, true, births); };
    const look = () => { peak = Math.max(peak, D.list.length); peakLoad = Math.max(peakLoad, game.destr.breaks.load()); if (!bad) for (const c of D.list) { /* a kinked plate is two halves in one box: neither half fills it */ const f = recordFault(c.box.h, c.box.cut, !c.mate) ?? (c.mate ? recordFault(c.mate.h, c.mate.cut, false) : null); if (f) { bad = `chunk ${c.serial}: ${f}`; break; } if (c.box.c.some((x: number) => !isFinite(x))) { bad = `chunk ${c.serial} not finite`; break; } } };
    for (const w of game.destr.walls.all()) { const b = w.box; const alongX = b.h[0] >= b.h[2]; const L = alongX ? b.h[0] : b.h[2];
      for (let u = -L; u <= L; u += 1.2) { const at: Vec3 = alongX ? [b.c[0] + u, 0.4, b.c[2] + ((blasts & 1) ? 0.3 : -0.3)] : [b.c[0] + ((blasts & 1) ? 0.3 : -0.3), 0.4, b.c[2] + u]; game.destr.blast(at, { who: PLAYER_ID, radius: 2.6 }); blasts++; W.step(DT); look(); } }
    for (let f = 0; f < 600; f++) { W.step(DT); look(); }
    const inv = game.destr.walls.inventory(); const st = standing(game); let sbad = ''; let cutN = 0, maxP = 0;
    for (const b of st) { const f = recordFault(b.h, b.cut, false); if (f) { sbad = f; break; } if (b.cut) { cutN++; maxP = Math.max(maxP, b.cut.length); } }
    F.verdict(peak <= cap && !bad, `pool ${cap}: the rigid pool never passes its cap, every chunk's record valid on every frame`, bad || `${blasts} blasts: peak ${peak}/${cap}; shaped ${BREAK_STATS.shaped}, heap ${BREAK_STATS.heap}, again ${BREAK_STATS.again}, crumbled ${BREAK_STATS.crumbled}, refused ${BREAK_STATS.refused}, mayhem ${BREAK_STATS.mayhem}; ${BREAK_STATS.dropped} planes dropped`);
    F.verdict(inv.pieces <= WALLS.maxPieces && !sbad && maxP <= BREAKS.maxStandingPlanes, `pool ${cap}: standing pieces inside the stand-in cap, each ≤ 4 planes`, sbad || `${inv.pieces}/${WALLS.maxPieces} pieces standing, ${cutN} with a ragged end, most ${maxP} planes`);
    const load = game.destr.breaks.load();
    F.verdict(load < cap, `pool ${cap}: once it has settled the load number (pieces moving + particle records waiting) is under the pool size`, `load ${load} after 10 s (moving ${D.moving}); worst while it went on ${peakLoad}`);
    
  }
  return F.done();
}

// ================================================================ determinism
function wreck(W: any, game: any) {
  let k = 0;
  for (const w of game.destr.walls.all()) { if (k >= 6) break; const b = w.box; const alongX = b.h[0] >= b.h[2];
    const at: Vec3 = alongX ? [b.c[0], 0.4, b.c[2] + 0.3] : [b.c[0] + 0.3, 0.4, b.c[2]]; game.destr.blast(at, { who: PLAYER_ID, radius: 2.4 }); for (let f = 0; f < 40; f++) W.step(DT); k++; }
  for (let f = 0; f < 200; f++) W.step(DT);
}
