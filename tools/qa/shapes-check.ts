// A BOX SAYS ITS SHAPE (src/scene/shapes.ts, SHAPES).
// The lint: what it takes and what it refuses, and a shape to a level and back to data unchanged (line 1). Stood up: a yard whose pickup is given every kind
// of shape here (a body with rounded corners and two pieces, a wheel that is a round, a bonnet that is an arc with a plane beside it): with the switch each
// box has its record, the pieces follow the level's last box and no box has changed its number, its centre or its size; put back, the list is the file's
// again; and a box moved and turned takes its pieces along (line 2). The shape, by references written here (eight tangents of a circle, a quarter circle's
// three, a round's radius): boxes.ts rayBox and collision.ts lineBox over seeded rays, with the cut-pieces switch up and with it down (line 3). What walks
// meets the plain level: the walk grid and a circle's push are the same with shapes up as down, on that yard and on every level file that says a shape
// (line 4), and every shape a level file says stands. Down is the run it was (line 5). A level stood on its shapes is another level: the fingerprint, the stands and a recording from before the
// table (line 6); a checkpoint is refused across the switch, both ways, and opens in its own game (line 7); the game's own module, asked inside a tick of
// its copy of the shaped yard (line 8); and a blast takes a crate that says a shape and the plate that hangs on it together (line 9).
//   bun run tools/qa/shapes-check.ts          (probes.ts shapes; SHAPES_BREAK=1 must FAIL — the stand's word is left down where the check raises it)
// Both stands: the check gives the stand its word (TPS_SHAPES) itself. Nothing here is a picture: no shaped box has been seen on a GPU by this check.
import { readFileSync, readdirSync } from 'node:fs';
import { standUp, ROOT } from './headless.ts';
const { parseLevelData, validateLevelData, canonicalLevelData } = await import(`${ROOT}/src/scene/levelData.ts`);
const { loadLevel, levelToData } = await import(`${ROOT}/src/scene/levelLoad.ts`);
const { SHAPES, SHAPES_STATS, shapeLevel, shapesTape, shapeStamp, withPieces } = await import(`${ROOT}/src/scene/shapes.ts`);
const { digestGame, hashDigest } = await import(`${ROOT}/src/game/digest.ts`);
const { levelFingerprint, canLoad } = await import(`${ROOT}/src/game/savestate.ts`);
const { PLAYER_ID } = await import(`${ROOT}/src/game/consts.ts`);
const { rayBox, BoxFlag } = await import(`${ROOT}/src/scene/boxes.ts`);
const { StaticCollision, NavGrid, lineBox, FEET_Y } = await import(`${ROOT}/src/scene/collision.ts`);
const { cutConfig } = await import(`${ROOT}/src/scene/cut.ts`);

type Vec3 = [number, number, number];
type Plane = [number, number, number, number];
const DT = 1 / 60, BREAK = process.env.SHAPES_BREAK === '1', VERBOSE = !!process.env.SHAPES_VERBOSE, D = Math.PI / 180;
function seeded(seed: number) { let a = seed >>> 0; return () => { a |= 0; a = a + 0x6D2B79F5 | 0; let t = Math.imul(a ^ a >>> 15, 1 | a); t = t + Math.imul(t ^ t >>> 7, 61 | t) ^ t; return ((t ^ t >>> 14) >>> 0) / 4294967296; }; }
async function hushed<T>(f: () => T | Promise<T>): Promise<T> { const log = console.log, warn = console.warn; if (!VERBOSE) console.log = console.warn = () => { }; try { return await f(); } finally { console.log = log; console.warn = warn; } }
const LEVEL_FILES = readdirSync(`${ROOT}/levels`).filter(f => f.endsWith('.json') && !f.includes('.lint')).sort();
const dataOf = (file: string): any => JSON.parse(readFileSync(`${ROOT}/levels/${file}`, 'utf8'));
const levelOf = (d: any) => loadLevel(parseLevelData(JSON.stringify(d)));
const norm = (a: Vec3): Vec3 => { const l = Math.hypot(a[0], a[1], a[2]) || 1; return [a[0] / l, a[1] / l, a[2] / l]; };
const at = (o: Vec3, d: Vec3, t: number): Vec3 => [o[0] + d[0] * t, o[1] + d[1] * t, o[2] + d[2] * t];
/** a point in a box's own frame, and a point of that frame in the world (yaw about +y: local +x → (cos, 0, −sin), +z → (sin, 0, cos)) — written out here, not
 *  taken from the code under test */
const localOf = (b: any, p: readonly number[]): Vec3 => { const c = Math.cos(b.yaw), s = Math.sin(b.yaw), q = [p[0] - b.c[0], p[1] - b.c[1], p[2] - b.c[2]]; return [c * q[0] - s * q[2], q[1], s * q[0] + c * q[2]]; };
const worldOf = (c0: readonly number[], yaw: number, l: readonly number[]): Vec3 => { const c = Math.cos(yaw), s = Math.sin(yaw); return [c0[0] + c * l[0] + s * l[2], c0[1] + l[1], c0[2] - s * l[0] + c * l[2]]; };
/** the planes of a nose: each upright corner at +x rounded to radius r by three tangents of a quarter circle about (h.x − r, ±(h.z − r)) */
const NOSE_R = 0.22, NOSE_A = [22.5, 45, 67.5];
const nose = (h: Vec3): Plane[] => [1, -1].flatMap(s => NOSE_A.map((a): Plane => [Math.cos(a * D), 0, s * Math.sin(a * D), Math.cos(a * D) * (h[0] - NOSE_R) + Math.sin(a * D) * (h[2] - NOSE_R) + NOSE_R]));
/** a bonnet's fall: the plane from the top face 0.2 m in from the end at +x down to 0.15 m under the top at that end */
const fall = (h: Vec3): Plane => { const L = Math.hypot(0.15, 0.2), n = [0.15 / L, 0.2 / L]; return [n[0], n[1], 0, n[0] * h[0] + n[1] * (h[1] - 0.15)]; };

/** THE YARD WITH A SHAPED PICKUP: levels/gwot-yard.json, its first pickup's body, one wheel and bonnet given a shape of each kind, and a wooden crate
 *  beside it that says a shape and has a plate hung on its side (the level's own data is not touched: a copy; the bonnet's copy gives up its markings, which
 *  a box that says a form may not carry). Returns the data and the boxes' places in the list. */
function shapedYard(): { d: any; tub: number; wheel: number; bonnet: number; crate: number } {
  const d = dataOf('gwot-yard.json'), B: any[] = d.boxes, near = (a: number, b: number) => Math.abs(a - b) < 1e-6;
  const tub = B.findIndex(b => near(b.h[0], 2.45) && near(b.h[1], 0.25) && near(b.h[2], 0.86)); if (tub < 0) throw new Error('shapes-check: the yard has no pickup body (2.45 × 0.25 × 0.86)');
  const T = B[tub], yaw = T.yaw ?? 0;
  const by = (l: Vec3, h: Vec3) => { const c = worldOf(T.c, yaw, [l[0], l[1] - T.c[1], l[2]]); return B.findIndex(b => Math.hypot(b.c[0] - c[0], b.c[1] - c[1], b.c[2] - c[2]) < 1e-3 && near(b.h[0], h[0]) && near(b.h[1], h[1]) && near(b.h[2], h[2])); };
  const wheel = by([1.52, 0.37, 0.78], [0.37, 0.37, 0.13]), bonnet = by([1.75, 0.9, 0], [0.7, 0.1, 0.84]); if (wheel < 0 || bonnet < 0) throw new Error('shapes-check: the yard\'s pickup has no front wheel or bonnet where the kit writes them');
  T.shape = { cut: nose(T.h), old: true, with: [
    { off: [1.52, -0.18, 0.914], h: [0.2, 0.2, 0.004], albedo: [0.3, 0.31, 0.32], rough: 0.45, metal: true, round: 3 },   // (a hub, proud of the tyre and of the body's side)
    { off: [0.38, 0.74, 0.78], h: [0.5, 0.21, 0.03], material: T.material, arc: { axis: 3, rise: 0.3 } }] };                           // (an arch over the wheel)
  B[wheel].shape = { round: 3 };
  delete B[bonnet].markings; B[bonnet].shape = { arc: { axis: 1, rise: 0.2 }, cut: [fall(B[bonnet].h)] };
  // the crate: wood, a metre long, on the yard's ground three metres off the pickup's side; two corners cut, and a plate hung on its long face
  const cc = worldOf(T.c, yaw, [0, 0, 3]); const CR = 0.12, ch: Vec3 = [0.5, 0.4, 0.3];
  B.push({ id: 'shapes_check_crate', floor: T.floor, c: [cc[0], 0.4, cc[2]], h: ch, yaw, material: 'woodDark',
    shape: { cut: [1, -1].map((sg): Plane => [Math.SQRT1_2, 0, sg * Math.SQRT1_2, Math.SQRT1_2 * (ch[0] + ch[2] - CR)]), with: [{ off: [0, 0.1, 0.305], h: [0.2, 0.12, 0.005], albedo: [0.8, 0.8, 0.75] }] } });
  return { d, tub, wheel, bonnet, crate: B.length - 1 };
}
const flat = (B: any[]) => JSON.stringify(B.map(b => [b.c, b.h, b.yaw, b.flags, b.marks ?? null, b.cut ?? null, b.shaped ?? null, b.piece ?? null]));
/** a level stood as the switch says (the break word leaves the stand's own word down). No word is given for cut pieces: the shape is the game's either way. */
async function stand(d: any, on: boolean): Promise<any> {
  Math.random = seeded(21);
  const word = process.env.TPS_SHAPES;
  try { process.env.TPS_SHAPES = on && !BREAK ? '1' : '0'; return await hushed(() => standUp({ level: levelOf(d), seed: 1 })); }
  finally { if (word === undefined) delete process.env.TPS_SHAPES; else process.env.TPS_SHAPES = word; }
}

export async function shapesCheck(): Promise<boolean> {
  let all = true;
  const line = (ok: boolean, s: string, detail = '') => { all &&= ok; console.log(`- ${ok ? 'PASS' : 'FAIL'} — ${s}${detail ? `\n    ${detail}` : ''}`); };
  console.log(`\n=== shapes: a box says its shape${BREAK ? ' (SHAPES_BREAK: the switch left off)' : ''} ===`);
  const was = { ...SHAPES }, cutWas = cutConfig.on, stats0 = { ...SHAPES_STATS }, rnd = Math.random;
  try {
    const Y = shapedYard(), opts = { materials: Y.d.materials, boxIds: Y.d.boxes.map((b: any) => b.id), boxGroups: Y.d.boxes.map((b: any) => b.group) } as any;
    // (1) the lint, and a shape to a level and back
    { const errs = (f: (d: any) => void): string[] => { const d = JSON.parse(JSON.stringify(Y.d)); f(d); return validateLevelData(d) as string[]; };
      const has = (e: string[], w: string) => e.length > 0 && e.every(x => x.includes('shape')) && e.some(x => x.includes(w));
      const cases: [string, string[], string][] = [
        ['nine planes', errs(d => { d.boxes[Y.tub].shape.cut.push([0.6, 0.8, 0, 2], [-0.6, 0.8, 0, 2], [0, 0.8, 0.6, 0.7]); }), 'at most 8'],
        ['a second plane beside an arc', errs(d => { d.boxes[Y.bonnet].shape.cut.push([0, 1, 0, 0.05]); }), 'room for 1'],
        ['a round on a box not square across its axis', errs(d => { d.boxes[Y.wheel].h = [0.37, 0.3, 0.13]; }), 'square'],
        ['an arc higher than its box', errs(d => { d.boxes[Y.bonnet].shape.arc.rise = 0.5; }), 'high'],
        ['an arc that rises more than its half width', errs(d => { d.boxes[Y.tub].shape.with[1].arc = { axis: 1, rise: 0.2 }; }), 'half round'],
        ['a round and an arc at once', errs(d => { d.boxes[Y.wheel].shape.arc = { axis: 3, rise: 0.1 }; }), 'one or the other'],
        ['a shape on a tilted box', errs(d => { d.boxes[Y.wheel].rot = [0, 0, 0, 1]; }), 'rot'],
        ['a piece with no paint', errs(d => { delete d.boxes[Y.tub].shape.with[0].albedo; }), 'material / albedo'],
        ['a piece with pieces', errs(d => { d.boxes[Y.tub].shape.with[1].with = []; }), 'unknown key'],
        ['a shape that says nothing', errs(d => { d.boxes[Y.wheel].shape = {}; }), 'says nothing'],
        ['a cut that leaves nothing', errs(d => { d.boxes[Y.tub].shape.cut = [[1, 0, 0, -5]]; }), 'leaves nothing'],
        ['a plane that cuts nothing', errs(d => { d.boxes[Y.tub].shape.cut.push([1, 0, 0, 99]); }), 'cuts nothing'],
        ['a centre or a size of its own on a shaped box', errs(d => { d.boxes[Y.bonnet].shape.h = [0.65, 0.1, 0.84]; }), 'unknown key'],
        ['a piece that says flags', errs(d => { d.boxes[Y.tub].shape.with[0].flags = ['Gloss']; }), 'trim'],
        ['a piece turned by no number', errs(d => { d.boxes[Y.tub].shape.with[0].yaw = 1e999; }), 'yaw'],
        ['a shape on a slab of a kit octagon', errs(d => { const T = d.boxes[Y.tub], r = 0.3, t8 = Math.tan(Math.PI / 8);   // (four slabs as the kit writes a drum: one paint, turned 45° each, their tops stepped down 2 mm)
          for (let k = 0; k < 4; k++) { const top = 1 - 0.002 * k; d.boxes.push({ id: `shapes_check_drum_${k}`, floor: T.floor, c: [T.c[0], top / 2, T.c[2] + 6], h: [r, top / 2, r * t8], yaw: k * Math.PI / 4, material: T.material }); }
          d.boxes[d.boxes.length - 2].shape = { with: [{ off: [0, 0.6, 0], h: [0.05, 0.05, 0.05], albedo: [1, 1, 1] }] }; }), 'octagon'],
        ['a form on a box with markings', errs(d => { d.boxes[Y.bonnet].markings = dataOf('gwot-yard.json').boxes[Y.bonnet].markings; }), 'markings'],
        ['a form on a pane of glass', errs(d => { d.boxes[Y.wheel].flags = ['Glass']; }), 'Glass'],
      ];
      const good = errs(() => { }), kept = cases.filter(c => has(c[1], c[2]));
      // the same shape written with its keys in another order is the same level's bytes (levelData.ts canonicalLevelData)
      const mixed = JSON.parse(JSON.stringify(Y.d)), ms = mixed.boxes[Y.tub].shape; mixed.boxes[Y.tub].shape = { with: ms.with.map((w: any) => Object.fromEntries(Object.entries(w).reverse())), old: ms.old, cut: ms.cut };
      const canon = JSON.stringify(canonicalLevelData(parseLevelData(JSON.stringify(mixed)))) === JSON.stringify(canonicalLevelData(parseLevelData(JSON.stringify(Y.d)))) && JSON.stringify(mixed) !== JSON.stringify(Y.d);
      const L = levelOf(Y.d), back = levelToData(L, opts); shapeLevel(L.boxes, true); const stoodBack = levelToData(L, opts); shapeLevel(L.boxes, false);
      const same = (o: any) => [Y.tub, Y.wheel, Y.bonnet, Y.crate].every(i => JSON.stringify(o.boxes[i].shape) === JSON.stringify(Y.d.boxes[i].shape) && JSON.stringify(o.boxes[i].c) === JSON.stringify(Y.d.boxes[i].c) && JSON.stringify(o.boxes[i].h) === JSON.stringify(Y.d.boxes[i].h)) && o.boxes.length === Y.d.boxes.length;
      line(good.length === 0 && kept.length === cases.length && same(back) && same(stoodBack) && canon,
        `1 the level lint takes a shape of each kind and refuses ${cases.map(c => c[0]).join(', ')}; and a shape goes to a level and back to data unchanged, from a level as loaded and from one stood up shaped (the pieces are written only inside the shape that says them), and a shape with its keys in another order is the same level's bytes`,
        `good ${good.length} errors · refused ${kept.length} of ${cases.length}${kept.length < cases.length ? ` (not: ${cases.filter(c => !has(c[1], c[2])).map(c => `${c[0]} → ${c[1][0] ?? 'no error'}`).join(' · ')})` : ''} · back the same ${same(back)} · from a stood level ${same(stoodBack)} · keys in another order ${canon}`); }

    // (2) stood up, and put back
    const L = levelOf(Y.d), n0 = L.boxes.length, file = flat(L.boxes), first = L.boxes.slice(), places = L.boxes.map((b: any) => [b.c, b.h]);
    const made = shapeLevel(L.boxes, !BREAK), up = flat(L.boxes), nUp = L.boxes.length;
    { const B = L.boxes, tub = B[Y.tub], wheel = B[Y.wheel], bonnet = B[Y.bonnet], crate = B[Y.crate], tail = B.slice(n0);
      const same = first.every((b: any, i: number) => B[i] === b && b.c === places[i][0] && b.h === places[i][1]), twice = (shapeLevel(B, !BREAK), flat(B) === up && B.length === nUp);
      const recs = tub.cut?.length === 6 && wheel.cut?.length === 8 && bonnet.cut?.length === 7 && crate.cut?.length === 2 && tail.length === 3 && tail.every((p: any) => p.piece === true && p.flags === BoxFlag.NoShadow) && tail[0].cut?.length > 0 && tail[1].cut?.length > 0;
      const mine = withPieces(B, [Y.tub, Y.wheel]) as number[], tied = JSON.stringify(mine) === JSON.stringify(BREAK ? [Y.tub, Y.wheel] : [Y.tub, Y.wheel, n0, n0 + 1]) && tail.every((p: any) => typeof p.of === 'number' && B[p.of].shaped === true);
      // the number the level's fingerprint takes in: these shapes', another for a plane moved a centimetre, the same for a piece repainted, none when none stands
      const stampUp = shapeStamp(B) as number, stampOf = (f: (d: any) => void) => { const d = JSON.parse(JSON.stringify(Y.d)); f(d); const X = levelOf(d); shapeLevel(X.boxes, true); return shapeStamp(X.boxes) as number; };
      const stamps = stampUp !== 0 && stampOf(() => { }) === stampUp && stampOf(d => { d.boxes[Y.tub].shape.cut[0][3] += 0.01; }) !== stampUp && stampOf(d => { d.boxes[Y.tub].shape.with[1].off[0] += 0.01; }) !== stampUp && stampOf(d => { d.boxes[Y.tub].shape.with[0].albedo = [0.9, 0.1, 0.1]; delete d.boxes[Y.tub].shape.old; }) === stampUp;
      shapeLevel(B, false); const down = flat(B) === file && B.length === n0 && !B.some((b: any) => b.shaped || b.cut || b.piece) && shapeStamp(B) === 0;
      // a box moved and turned takes its shape along: the pieces stand where the box's own frame puts them
      const M = levelOf(Y.d), mt = M.boxes[Y.tub]; mt.c = [mt.c[0] + 3, mt.c[1], mt.c[2] - 2]; mt.yaw += 0.7; shapeLevel(M.boxes, !BREAK);
      const hub = M.boxes[n0], want = worldOf(mt.c, mt.yaw, [1.52, -0.18, 0.914]), carried = !!hub && Math.hypot(hub.c[0] - want[0], hub.c[1] - want[1], hub.c[2] - want[2]) < 1e-9 && Math.abs(hub.yaw - mt.yaw) < 1e-12;
      // a window's bar (winbars.ts breaks it into lengths itself) is left plain; and a box's pieces are found by its number
      const Q = levelOf(Y.d); Q.boxes[Y.wheel].flags |= BoxFlag.Bar; const barOk = shapeLevel(Q.boxes, true) === 3 && !Q.boxes[Y.wheel].shaped && !Q.boxes[Y.wheel].cut;
      shapeLevel(B, !BREAK);
      line(made === 4 && recs && same && twice && stamps && down && carried && barOk && tied,
        '2 the yard stood up with the switch: the body has its six corner planes, the wheel is a round, the bonnet an arc with its plane, the crate its two corners, and the three pieces follow the level\'s last box, each trim (NoShadow and no other flag); every box of the level is the box it was, at the place and of the size it was, and a second stand-up changes nothing; the number of the shapes that stand is another for a plane or a piece moved a centimetre and the same for a piece repainted; put back, the list is the file\'s again, no box keeps a record or a piece and the number is none; a body moved three metres and turned takes its hub with it; a window\'s bar that says a form is left plain; and a box\'s pieces are found by its number',
        `${made} boxes shaped of 4 · ${n0} boxes, ${nUp} stood up · records ${tub.cut?.length ?? 0}/${wheel.cut?.length ?? 0}/${bonnet.cut?.length ?? 0}/${crate.cut?.length ?? 0} planes, ${tail.length} pieces · same boxes ${same} · twice the same ${twice} · the number ${stamps} · put back ${down} · carried ${carried} · a bar left plain ${barOk} · pieces by their box ${tied}`); }

    // (3) the shape, by references written here — with the cut-pieces switch up, and down (the shape is the game's: a picture setting does not move it)
    { const B = L.boxes, T = B[Y.tub], W = B[Y.wheel], N = B[Y.bonnet];
      const U8 = [30, 60, 120, 150, 210, 240, 300, 330].map(a => [Math.cos(a * D), Math.sin(a * D)]);
      const inBox = (b: any, l: Vec3, g = 0) => Math.abs(l[0]) < b.h[0] + g && Math.abs(l[1]) < b.h[1] + g && Math.abs(l[2]) < b.h[2] + g;
      // the wheel: the box, less what lies outside eight tangents of a circle of its smaller half across its own z (with the box's own four faces: twelve sides)
      const inWheel = (p: readonly number[], g = 0) => { const l = localOf(W, p), r = Math.min(W.h[0], W.h[1]); return inBox(W, l, g) && U8.every(u => u[0] * l[0] + u[1] * l[1] < r + g); };
      // the body: the box less the two nose corners, each three tangents of a quarter circle of radius 0.22 about (h.x − r, ±(h.z − r))
      const inTub = (p: readonly number[], g = 0) => { const l = localOf(T, p); if (!inBox(T, l, g)) return false;
        for (const s of [-1, 1]) for (const a of NOSE_A) if (Math.cos(a * D) * (l[0] - (T.h[0] - NOSE_R)) + Math.sin(a * D) * (s * l[2] - (T.h[2] - NOSE_R)) > NOSE_R + g) return false; return true; };
      const said: string[] = []; let ok = true;
      for (const cut of [true, false]) { cutConfig.on = cut; const R = seeded(5);
        let rays = 0, hits = 0, late = 0, missed = 0, off = 0, lineBad = 0;
        for (const [b, inside] of [[W, inWheel], [T, inTub]] as [any, (p: readonly number[], g?: number) => boolean][]) for (let k = 0; k < 1500; k++) {
          const tgt: Vec3 = [b.c[0] + (R() - 0.5) * 2.4 * Math.max(b.h[0], b.h[2]), b.c[1] + (R() - 0.5) * 2.4 * b.h[1], b.c[2] + (R() - 0.5) * 2.4 * Math.max(b.h[0], b.h[2])];
          const o: Vec3 = [tgt[0] + (R() - 0.5) * 4, tgt[1] + (R() - 0.5) * 4, tgt[2] + (R() - 0.5) * 4]; if (inside(o, 0.01)) continue; rays++;
          const d = norm([tgt[0] - o[0], tgt[1] - o[1], tgt[2] - o[2]]), tmax = 8, h = rayBox(o, d, b, tmax), seg = lineBox(o, d, b);
          let ref = -1; for (let s = 1; s < 8000; s++) { const t = tmax * s / 8000; if (inside(at(o, d, t), -1e-4)) { ref = t; break; } }
          if (h) { hits++; if (!inside(at(o, d, h.t + 2e-4), 1e-6) || inside(at(o, d, h.t - 2e-4), -1e-6)) off++; if (ref >= 0 && ref < h.t - tmax / 8000 - 1e-4) late++;
            if (!seg || Math.abs(seg.t0 - h.t) > 1e-9 || !inside(at(o, d, (seg.t0 + seg.t1) / 2), 1e-6)) lineBad++; }
          else if (ref >= 0) missed++;
        }
        // the bonnet: its top lies on the round of radius R that rises 0.2 over its half width, never under it and never more than the facets stand off it
        const w = N.h[2], rise = 0.2, RR = (w * w + rise * rise) / (2 * rise), yAxis = N.h[1] - RR; let tops = 0, under = 0, over = 0, sunk = 0;
        for (let k = 0; k < 1500; k++) { const l: Vec3 = [(R() - 0.5) * 2 * (N.h[0] - 0.3), 2, (R() - 0.5) * 1.98 * w], o = worldOf(N.c, N.yaw, l), h = rayBox(o, [0, -1, 0], N, 5); if (!h) { sunk++; continue; } tops++;
          const q = localOf(N, at(o, [0, -1, 0], h.t)), rr = Math.hypot(q[1] - yAxis, q[2]); if (rr < RR - 1e-7) under++; if (rr > RR * 1.03) over++; }
        ok &&= late === 0 && missed === 0 && off === 0 && lineBad === 0 && hits > 600 && tops > 1000 && sunk === 0 && under === 0 && over === 0;
        said.push(`cut pieces ${cut ? 'up' : 'down'}: ${rays} rays, ${hits} hits · ${off} off the surface · ${late} late · ${missed} missed · ${lineBad} intervals · the bonnet: ${tops} landed, ${sunk} fell through, ${under} under the round, ${over} over it`); }
      cutConfig.on = cutWas;
      line(ok, '3 one model, by references written here, with the cut-pieces switch up and with it down: over seeded rays rayBox lands on the wheel\'s twelve-sided round and on the body\'s rounded nose where the reference first finds a point inside, a miss crosses none of it, and lineBox\'s interval begins there; a ray dropped on the bonnet lands on the round that rises 0.2 m over its half width, never under it and never more than three hundredths of its radius over',
        said.join('\n    ')); }

    // (4) what walks: the same walk grid and the same push with shapes up as down — on the yard, and on every level file that says a shape
    { const R = seeded(9), scratch = new Int32Array(96); let files = 0, shaped = 0, cells = 0, navBad = 0, pushes = 0, pushBad = 0, touchBad = 0; const bad: string[] = [], unstood: string[] = [];
      const one = (name: string, d: any) => { const A = levelOf(d), Bv = levelOf(d), said = A.boxes.filter((b: any) => b.shape).length, n = shapeLevel(A.boxes, true); shapeLevel(Bv.boxes, false);   // (said: of the boxes this stand loads) if (n !== said) unstood.push(`${name}: ${said - n} of ${said}`); if (!n) return;
        files++; shaped += n; const ca = new StaticCollision(A.boxes), cb = new StaticCollision(Bv.boxes), na = new NavGrid(ca), nb = new NavGrid(cb); let nBad = 0, pBad = 0;
        for (let i = 0; i < na.blocked.length; i++) { cells++; if (na.blocked[i] !== nb.blocked[i]) nBad++; }
        const its = A.boxes.filter((b: any) => b.shaped || b.piece);
        for (let k = 0; k < 4000; k++) { const b = its[Math.floor(R() * its.length)], p: Vec3 = [b.c[0] + (R() - 0.5) * 3.2, 0, b.c[2] + (R() - 0.5) * 3.2], q: Vec3 = [p[0], p[1], p[2]], r = 0.2 + R() * 0.25;
          const ha = ca.collideCircle(p, r, FEET_Y, 1.5), hb = cb.collideCircle(q, r, FEET_Y, 1.5); pushes++;
          if (ha !== hb || Math.hypot(p[0] - q[0], p[2] - q[2]) > 1e-9) pBad++;
          if (ca.circleTouches(q[0], q[2], r + 0.01, FEET_Y, 1.5, scratch) === 0 !== (cb.circleTouches(q[0], q[2], r + 0.01, FEET_Y, 1.5, scratch) === 0)) touchBad++; }
        navBad += nBad; pushBad += pBad; if (nBad || pBad) bad.push(`${name}: ${nBad} walk-grid cells, ${pBad} pushes differ`); };
      one('the yard with a shaped pickup', Y.d);
      const own = files; for (const f of LEVEL_FILES) { const d = dataOf(f); if (d.boxes.some((b: any) => b.shape)) one(f, d); }
      line(own === 1 && navBad === 0 && pushBad === 0 && touchBad === 0 && pushes >= 4000 && unstood.length === 0,
        '4 what walks meets the plain level: the walk grid baked from the level stood up shaped is cell for cell the grid of the level stood up plain, and a walker\'s circle put down round every shaped box and piece is pushed to the same place, or left alone, by both — on the yard with a shaped pickup and on every level file that says a shape; and every shape a level says stands',
        `${files} level${files === 1 ? '' : 's'} (${files - own} file${files - own === 1 ? '' : 's'} under levels/ say a shape), ${shaped} shaped boxes · ${cells} walk-grid cells, ${navBad} differ · ${pushes} circles, ${pushBad} pushed otherwise, ${touchBad} touching otherwise · shapes said that do not stand: ${unstood.join(', ') || 'none'}${bad.length ? `\n    ${bad.slice(0, 8).join('\n    ')}` : ''}`); }

    // (5) down is the run it was: the yard that says shapes, stood with the switch down, against the yard's own file
    // (each game is stood and played at once: the stand seeds the tool's own die, and a game stood between another's stand-up and its run would move it on)
    const run = async (V: any) => { await hushed(() => { for (let i = 0; i < 300; i++) V.step(DT); }); return hashDigest(digestGame(V.game, true)); };
    const plainD = JSON.parse(JSON.stringify(Y.d)); for (const b of plainD.boxes) delete b.shape;   // (the same yard — the bonnet without its marks, the crate — saying no shape)
    const plain: any = await stand(plainD, false), dPlain = await run(plain), off: any = await stand(Y.d, false), dOff = await run(off), on: any = await stand(Y.d, true), dOn = await run(on);
    { const downPlain = !off.game.level.boxes.some((x: any) => x.shaped || x.cut || x.piece) && off.game.col.boxes.length === plain.game.col.boxes.length;
      const upShaped = on.game.col.boxes.length === off.game.col.boxes.length + 3 && on.game.level.boxes[Y.tub].cut?.length === 6;
      const marked = off.game.level.boxes.every((b: any, i: number) => on.game.level.boxes[i].flags === b.flags);   // (thin things, a window's bars: the stand-up's marks are read off the plain boxes)
      line(dPlain === dOff && downPlain && upShaped && marked,
        '5 five seconds of the yard that says shapes, stood with the switch down, is the same run by digest as the same yard saying none, and its list is the file\'s; stood with the switch up (and no word for cut pieces) the game\'s world has the shaped boxes and the three pieces, and every box of the level the flags it has with the switch down',
        `saying none ${dPlain} · saying shapes, down ${dOff} · up ${dOn}${dOn === dPlain ? ' (the same: nobody met the pickup in five seconds)' : ''} · down plain ${downPlain} · the same flags ${marked} · up shaped ${upShaped} (${on.game.col.boxes.length} statics, ${off.game.col.boxes.length} plain)`); }

    // (6) a level stood on its shapes is another level; the stands and a recording
    { const J: any = await import(`${ROOT}/src/game/journal.ts`), PR: any = await import(`${ROOT}/src/host/profile.ts`), SE: any = await import(`${ROOT}/src/host/standEnv.ts`);
      const snap = () => JSON.parse(JSON.stringify(J.knobsSnapshot(on.game)));
      SHAPES.on = true; const old = snap(); delete old.tables.SHAPES; shapesTape(old.tables); const tapeOff = SHAPES.on === false;
      SHAPES.on = true; const rec = snap(); shapesTape(rec.tables); const tapeOn = SHAPES.on === true && rec.tables.SHAPES?.on === true;
      const called = ['src/replay/recipestand.ts', 'tools/qa/rerun.ts'].filter(f => !/shapesTape\(/.test(readFileSync(`${ROOT}/${f}`, 'utf8')));
      if (!/TPS_SHAPES = .*SHAPES\?\.on/.test(readFileSync(`${ROOT}/tools/qa/replay.ts`, 'utf8'))) called.push('tools/qa/replay.ts (the stand\'s word)');
      const fpOn = levelFingerprint(on.game), fpOff = levelFingerprint(off.game), fpPlain = levelFingerprint(plain.game);
      SHAPES.on = true; J.applyKnobs(on.game, old); const before = SHAPES.on;
      SHAPES.on = false; J.applyKnobs(on.game, rec); const kept = SHAPES.on;
      line(fpOn.fp !== fpOff.fp && fpOff.fp === fpPlain.fp && fpOn.boxes === fpOff.boxes + 3 && tapeOff && tapeOn && called.length === 0 && before === false && kept === true && PR.FROZEN_HOLDS['SHAPES.on'] === 'TPS_SHAPES=1' && SE.STAND_ENV.includes('TPS_SHAPES'),
        '6 a level stood on its shapes is another level: its fingerprint is not the plain level\'s, and the plain level\'s is the one of the same level saying no shape; a recording from before the table has the switch put down before its level is stood up (the two tape players by the table, the session replay by the stand\'s word) and one made with it on keeps it; the stands\' list has the row and the word',
        `fingerprint ${fpOn.fp} over ${fpOn.boxes} boxes up, ${fpOff.fp} over ${fpOff.boxes} down, ${fpPlain.fp} saying none · a tape without the table: ${tapeOff ? 'down' : 'UP'} · with it: ${tapeOn ? 'up' : 'DOWN'} · not calling: ${called.join(', ') || 'none'} · applyKnobs without the table: ${before} · recorded on: ${kept} · FROZEN_HOLDS ${PR.FROZEN_HOLDS['SHAPES.on'] ?? 'no row'} · STAND_ENV ${SE.STAND_ENV.includes('TPS_SHAPES')}`); }

    // (7) a checkpoint across the switch is refused, both ways, by the refusal a checkpoint of another level gets; in its own game it opens and plays on
    { const sOff = JSON.parse(JSON.stringify(off.snapshot('shapes down', true))), sOn = JSON.parse(JSON.stringify(on.snapshot('shapes up', true)));
      const downUp = canLoad(on.game, sOff) as string | null, upDown = canLoad(off.game, sOn) as string | null, ownOff = canLoad(off.game, sOff) as string | null, ownOn = canLoad(on.game, sOn) as string | null; let played = '';
      try { off.restore(sOff); on.restore(sOn); await hushed(() => { for (let i = 0; i < 30; i++) { off.step(DT); on.step(DT); } }); } catch (e) { played = String((e as Error).message ?? e).slice(0, 160); }
      line(!!downUp && downUp.includes('another level') && !!upDown && upDown.includes('another level') && ownOff === null && ownOn === null && played === '',
        '7 a checkpoint saved with shapes down is refused by the game stood with them up, and one saved with them up by the game stood with them down, each as a checkpoint of another level is; each opens in its own game and plays on',
        `down into up: ${downUp ?? 'OPENED'} · up into down: ${upDown ?? 'OPENED'} · its own: ${ownOff ?? 'opens'} / ${ownOn ?? 'opens'}${played ? ` · ${played}` : ''}`); }

    // (8) the game's own module, asked of its copy of the shaped yard inside a tick (a line of sight and a circle are the two questions it answers itself)
    { const SIM: any = await import(`${ROOT}/src/sim/sim.ts`), C = on.game.col, B: any[] = C.boxes, R = seeded(13), its = B.filter(b => b.shaped || b.piece), rows = [SIM.RUST.lines, SIM.RUST.circles];
      let asked = 0, segBad = 0, pushBad = 0;
      const cases = Array.from({ length: 3000 }, () => { const b = its[Math.floor(R() * its.length)] ?? B[Y.tub]; const a: Vec3 = [b.c[0] + (R() - 0.5) * 6, 0.2 + R() * 1.6, b.c[2] + (R() - 0.5) * 6], z: Vec3 = [b.c[0] + (R() - 0.5) * 2, 0.2 + R() * 1.4, b.c[2] + (R() - 0.5) * 2]; return { a, z, r: 0.2 + R() * 0.25 }; });
      const here = cases.map(q => { const p: Vec3 = [q.z[0], 0, q.z[2]]; const hit = C.collideCircleHere(p, q.r, FEET_Y, 1.5, 3); return { seg: C.segmentBlockedHere(q.a, q.z, 0.02, BoxFlag.Glass, null), hit, p }; });
      try { SIM.RUST.lines = SIM.RUST.circles = true; C.tickOpen();
        if (C.mirror.live && C.mirror.lines && C.mirror.circles) cases.forEach((q, i) => { asked++; const p: Vec3 = [q.z[0], 0, q.z[2]], hit = C.collideCircle(p, q.r, FEET_Y, 1.5), H = here[i];
          if (C.segmentBlocked(q.a, q.z) !== H.seg) segBad++; if (hit !== H.hit || Math.hypot(p[0] - H.p[0], p[2] - H.p[2]) > 1e-9) pushBad++; });
      } finally { C.tickShut(); [SIM.RUST.lines, SIM.RUST.circles] = rows; }
      const why = SIM.simWhyNot();
      line(segBad === 0 && pushBad === 0 && (why ? asked === 0 : asked === cases.length) && here.some(h => h.seg) && here.some(h => !h.seg),
        `8 the game\'s own module, asked inside a tick of its copy of the shaped yard (its rows for lines and circles up; cut pieces as the stand has them): a line of sight and a walker\'s circle about the shaped boxes and their pieces are answered as this file\'s TypeScript answers them${why ? ' — NOT JUDGED HERE: the module cannot run on this machine' : ''}`,
        `${cases.length} lines and circles · cut pieces ${cutConfig.on ? 'up' : 'down'} on this stand · the module: ${why ? `cannot run here (${why}), not asked` : `${asked} asked, ${segBad} lines and ${pushBad} pushes answered otherwise`} · of the lines ${here.filter(h => h.seg).length} are stopped`); }

    // (9) a blast: the crate that says a shape goes whole, and the plate that hangs on it goes with it (the wall engine's own rule for what hangs on a thing)
    { const G = on.game, B: any[] = G.col.boxes, crate = B[Y.crate], plate = B.find(b => b.piece === true && Math.hypot(b.c[0] - crate.c[0], b.c[2] - crate.c[2]) < 0.5), gone = (b: any) => !!b && (b.flags & BoxFlag.NoPrimary) !== 0;
      const br = G.destr.walls.breakableAt(Y.crate), stoodBefore = !gone(crate) && !gone(plate), cells = br ? br.cells : -1;
      const DS: any = await import(`${ROOT}/src/game/destructionSwitch.ts`), ds0 = DS.DESTRUCTION.on, chaos0 = G.tune.chaos; let res: any = null;
      try { DS.DESTRUCTION.on = true; G.tune.chaos = 0.5; await hushed(() => { res = G.destr.blast([crate.c[0], 0.4, crate.c[2] + 0.5], { who: PLAYER_ID, radius: 2.4 }); for (let i = 0; i < 90; i++) on.step(DT); }); }
      finally { DS.DESTRUCTION.on = ds0; G.tune.chaos = chaos0; }
      let saved = ''; try { const s2 = JSON.parse(JSON.stringify(on.snapshot('after the blast', true))); const no = canLoad(on.game, s2); if (no) saved = no; else on.restore(s2); } catch (e) { saved = String((e as Error).message ?? e).slice(0, 160); }
      line(!!plate && stoodBefore && cells === 1 && gone(crate) && gone(plate) && saved === '',
        '9 a blast beside the wooden crate that says a shape: the crate is one cell to the wall engine and goes whole, the plate hung on its side goes with it, and a checkpoint taken afterwards opens in that game',
        `the crate: ${br ? `${cells} cell${cells === 1 ? '' : 's'}` : 'NOT the wall engine\'s'} · stood before ${stoodBefore} · the blast took ${res?.cover ?? '?'} of cover · after: the crate ${gone(crate) ? 'gone' : 'STANDS'}, its plate ${gone(plate) ? 'gone' : 'STANDS'} · the checkpoint: ${saved || 'opens'}`); }
  } finally { Object.assign(SHAPES, was); cutConfig.on = cutWas; Object.assign(SHAPES_STATS, stats0); Math.random = rnd; }
  console.log(`=== shapes: ${all ? 'ALL PASS' : 'FAILURES above'} ===`);
  return all;
}
if (import.meta.main) process.exit((await shapesCheck()) ? 0 : 1);
