// WHICH SIDE OF A WALL (the look page's row `walls`; src/render/wallSides.ts, lighting_common.wgsl diceTaps and diceFacePointBy). The level's floor plan as the
// words every cache read asks, held to the pass: the map itself, and the read the shader makes of it, through the TS copies of the shader's own lines
// (wallSides.ts sideAt, diceTapsOf, diceFacePointOf). No GPU: that the WGSL says the same is tools/qa/wgsl-compile.cjs (it compiles) and the page's stills.
//   bun run tools/qa/wallsides-check.ts                    exit 0 = every line passed, 1 = one failed
//   WALLSIDES_BREAK=1 bun run tools/qa/wallsides-check.ts  the store's outer wall is left out of the map (the store and the yard are one place): must FAIL
//   SIDEDOOR_BREAK=1  …                                     the words are made with no door told (a door's cells know no line of their own): must FAIL
import { readFileSync } from 'node:fs';
import { standUp, ROOT } from './headless.ts';
const { parseLevelData } = await import(`${ROOT}/src/scene/levelData.ts`);
const { loadLevel } = await import(`${ROOT}/src/scene/levelLoad.ts`);
const { placeMask, PlaceMap, PLACE_W, PLACE_H, PLACE_RES } = await import(`${ROOT}/src/scene/placePlan.ts`);
const { placeSides, sideAt, sideOff, sideLine, sideIsDoor, sideClear, diceTapsOf, diceFacePointOf, WallSides, SIDE_UNKNOWN, SIDE_DOOR } = await import(`${ROOT}/src/render/wallSides.ts`);
const { WORLD } = await import(`${ROOT}/src/scene/world.ts`);
const BRK = process.env.WALLSIDES_BREAK === '1', BRK_DOOR = process.env.SIDEDOOR_BREAK === '1'; let red = false;
const check = (ok: boolean, what: string, detail = '') => { red ||= !ok; console.log(`- ${ok ? 'PASS' : 'FAIL'} — ${what}${detail ? `\n    ${detail}` : ''}`); };
console.log(`\n=== wallsides: the pass's floor plan as every cache read asks it${BRK ? ' (the store\'s outer wall left out: must fail)' : ''} ===`);
const l = console.log, w = console.warn; console.log = console.warn = () => { };
const W: any = await standUp({ level: loadLevel(parseLevelData(readFileSync(`${ROOT}/levels/pass.json`, 'utf8'))), seed: 1 }); for (let i = 0; i < 10; i++) W.step(1 / 60);
console.log = l; console.warn = w;
const g = W.game, world = g.engine.world;
const GAPS = g.doors.list.map((d: any) => { const [hx, hz] = d.def.hinge, a = d.def.closedDir, wd = d.def.width; return [hx, hz, hx + Math.cos(a) * wd, hz + Math.sin(a) * wd]; });
const statics = () => BRK ? world.statics.filter((b: any) => !(Math.abs(b.c[0] - 34) < 0.2 && b.h[0] < 0.3 && b.h[2] > 1)) : world.statics;   // (the break: no wall on the line x = 34)
const build = () => { const m = placeMask(statics(), world.dynamics, undefined, GAPS); return { m, s: placeSides(statics(), m, BRK_DOOR ? [] : GAPS) as Uint32Array }; };
/** the frame's words as the engine keeps them: the one map of the level updated, then the words off it (engine.ts) */
const PM = new PlaceMap(), upd = (ws: any, nx = 40, nz = 28): boolean => { PM.update(statics(), world.dynamics, GAPS); return ws.update(statics(), BRK_DOOR ? [] : GAPS, nx, nz, PM); };
const hang = (name: string, a: number) => { const d = g.doors.byName(name); d.latched = a === 0; d.vel = 0; d.angle = a; };
const settle = () => { for (let i = 0; i < 2; i++) W.step(1 / 60); };
for (const d of g.doors.list) hang(d.def.name, 0); settle();
const { m, s } = build();
const N = PLACE_W * PLACE_H, ox = WORLD.rcMin[0], oz = WORLD.rcMin[2];
const ROOMS: Record<string, [number, number]> = { tea: [17, 8], kitchen: [25, 8], store: [31, 8], bunk: [31, 13.5], yard: [36, 8] };   // a point well inside each (levels/pass.json)

// 1. the places, and a finger off each face
{ const at = Object.fromEntries(Object.entries(ROOMS).map(([n, p]) => [n, sideAt(s, p[0], p[1])]));
  const pairs: [string, string][] = [['store', 'yard'], ['store', 'bunk'], ['store', 'kitchen'], ['kitchen', 'tea'], ['bunk', 'yard'], ['kitchen', 'yard'], ['tea', 'yard']];
  const same = pairs.filter(([a, b]) => at[a] === at[b] || at[a] >= SIDE_UNKNOWN || at[b] >= SIDE_UNKNOWN);
  check(same.length === 0, 'with every door shut, rooms that share a wall and the open air are places of different numbers', `${Object.entries(at).map(([n, v]) => `${n} ${v}`).join(' · ')}${same.length ? ` — the same: ${same.map(p => p.join(' and ')).join(', ')}` : ''}`);
  // the store's four walls (x 28 and 34, z 5 and 11; each 0.24 m thick): 3 cm off each face, every 10 cm along it, is the place of the room that face looks into
  const faces: [string, string, (t: number) => [number, number]][] = [
    ['the outer wall, inside', 'store', t => [34 - 0.15, 5.4 + t * 5.2]], ['the outer wall, outside', 'yard', t => [34 + 0.15, 5.4 + t * 5.2]],
    ['the bunk wall, the store\'s face', 'store', t => [28.4 + t * 5.2, 11 - 0.15]], ['the bunk wall, the bunk\'s face', 'bunk', t => [28.4 + t * 5.2, 11 + 0.15]],
    ['the kitchen wall and its shut door, the store\'s face', 'store', t => [28 + 0.15, 5.4 + t * 5.2]], ['the kitchen wall and its shut door, the kitchen\'s face', 'kitchen', t => [28 - 0.15, 5.4 + t * 5.2]] ];
  const bad: string[] = []; let looked = 0;
  for (const [name, room, p] of faces) for (let k = 0; k <= 52; k++) { const [x, z] = p(k / 52); looked++; const v = sideAt(s, x, z); if (v !== at[room]) bad.push(`${name} at (${x.toFixed(2)}, ${z.toFixed(2)}) is ${v}, ${room} is ${at[room]}`); }
  check(bad.length === 0, 'a point 3 cm off either face of a wall, or of a shut door, is on its own side, all along the wall', `${looked} points on six faces of the store's walls, ${bad.length} on the wrong side${bad.length ? `: ${bad.slice(0, 3).join('; ')}` : ''}`); }

// 2. two places a read could blend never share a number; a clear cell has nothing but its own place in reach
{ const free = (k: number) => m[k] !== 255 && (m[k] & 3) !== 1; const comp = new Int32Array(N).fill(-1); let nc = 0;
  for (let k0 = 0; k0 < N; k0++) { if (!free(k0) || comp[k0] >= 0) continue; const st = [k0]; comp[k0] = nc; while (st.length) { const c = st.pop()!, i = c % PLACE_W, j = (c - i) / PLACE_W; for (const [di, dj] of [[1, 0], [-1, 0], [0, 1], [0, -1]]) { const x = i + di, z = j + dj; if (x < 0 || z < 0 || x >= PLACE_W || z >= PLACE_H) continue; const t = z * PLACE_W + x; if (free(t) && comp[t] < 0) { comp[t] = nc; st.push(t); } } } nc++; }
  let shared = 0, wrongClear = 0, clear = 0, pairs = 0;
  for (let k = 0; k < N; k++) { if (!free(k)) continue; const i = k % PLACE_W, j = (k - i) / PLACE_W, isClear = ((s[k] >>> 6) & 1) !== 0; if (isClear) clear++;
    for (let dj = -3; dj <= 3; dj++) for (let di = -3; di <= 3; di++) { const x = i + di, z = j + dj; if (x < 0 || z < 0 || x >= PLACE_W || z >= PLACE_H) continue; const t = z * PLACE_W + x;
      if (isClear && comp[t] !== comp[k]) wrongClear++;
      if (free(t) && comp[t] !== comp[k]) { pairs++; if (((s[t] >>> 2) & 15) === ((s[k] >>> 2) & 15)) shared++; } } }
  check(nc >= 5 && pairs > 1000 && shared === 0, 'no two places within a probe\'s reach of each other share a number', `${nc} places, ${pairs} pairs of cells in different places within three cells, ${shared} with one number`);
  const frees = [...m].filter((_v, k) => free(k)).length;
  check(wrongClear === 0 && clear > frees * 0.5, 'a cell marked clear has nothing but its own place within three cells, and most of the floor is clear', `${clear} of ${frees} free cells clear (a read there costs one word), ${wrongClear} wrongly`);
  let walls = 0, lines = 0, inside = 0, silent = 0; for (let k = 0; k < N; k++) if (m[k] === 255) { walls++; const kind = s[k] & 3; if (kind === 1) lines++; else if (kind === 0) inside++; else silent++; }
  // (58 said nothing while a counter against a wall laid a line of its own in the wall's cells and a pier in a room had no place; what is left is the lorry, parked askew in the yard)
  let silentIndoors = 0; for (let k = 0; k < N; k++) if (m[k] === 255 && (s[k] & 3) === 2) { const x = ox + ((k % PLACE_W) + 0.5) / PLACE_RES, z = oz + (Math.floor(k / PLACE_W) + 0.5) / PLACE_RES; if (!(x > 16 && x < 20 && z > 16 && z < 20)) silentIndoors++; }
  check(lines > walls * 0.9 && silent <= 30 && silentIndoors === 0, 'a wall cell knows its lines, or the one place it stands in; the only cells that say nothing are the lorry\'s, askew in the yard', `${walls} wall cells: ${lines} with lines, ${inside} inside one place, ${silent} that say nothing (${silentIndoors} of them not the lorry's)`); }

// 3. the read: no share for a probe column in another place, everywhere on the floor
{ let looked = 0, masked = 0, leak = 0, sum = 0; const worst: string[] = [];
  for (let z = oz + 0.065; z < oz + WORLD.rcSize[2]; z += 0.13) for (let x = ox + 0.065; x < ox + WORLD.rcSize[0]; x += 0.13) {
    const own = sideAt(s, x, z); if (own >= SIDE_UNKNOWN) continue; looked++;
    const taps = diceTapsOf(s, x, z); const all = taps.map(([ix, iz]: [number, number, number]) => sideAt(s, ox + ix + 0.5, oz + iz + 0.5)); const foreign = all.filter((c: number) => c !== own && c < SIDE_UNKNOWN).length;
    if (Math.abs(taps.reduce((a: number, t: [number, number, number]) => a + t[2], 0) - 1) > 1e-6) sum++;
    if (foreign && foreign < 4 && taps.length === 4 && all.some((c: number) => c === own || c >= SIDE_UNKNOWN)) { leak++; if (worst.length < 3) worst.push(`(${x.toFixed(2)}, ${z.toFixed(2)}) in ${own} blends ${all.join(',')}`); }
    else if (foreign && taps.length < 4) { leak++; if (worst.length < 3) worst.push(`(${x.toFixed(2)}, ${z.toFixed(2)}) in ${own} keeps a column of ${all.join(',')}`); }
    if (taps.length < 4 || foreign === 0 && !sideClear(s, x, z)) masked++; }
  check(looked > 40000 && leak === 0 && sum === 0, 'a read gives no share to a probe column in another place, anywhere on the floor, and its shares sum to 1', `${looked} points 13 cm apart, ${masked} near a wall, ${leak} with a column across a wall, ${sum} whose shares do not sum to 1${worst.length ? `: ${worst.join('; ')}` : ''}`);
  // the case the maintainer saw: the store's floor between the drums and the outer wall, and the yard's ground at the wall's foot
  const inS = diceTapsOf(s, 33.7, 7.4), outS = diceTapsOf(s, 34.3, 7.4);
  check(inS.every(([ix]: [number, number, number]) => ix <= 33) && outS.every(([ix]: [number, number, number]) => ix >= 34), 'the store\'s floor 30 cm inside its outer wall reads no probe of the yard, and the ground 30 cm outside reads none of the store', `inside: columns x ${inS.map((t: [number, number, number]) => t[0]).join(',')} · outside: ${outS.map((t: [number, number, number]) => t[0]).join(',')}`); }

// 4. a face that looks at a wall reads on its own side of it
{ const store = sideAt(s, 31, 8);
  const cases: [string, number, number, number, number][] = [['a drum\'s side 0.3 m from the outer wall, looking at it', 33.7, 7.4, 1, 0], ['a crate\'s side 0.4 m from the bunk wall, looking at it', 33.3, 10.6, 0, 1], ['the outer wall\'s own inside face', 33.88, 8, -1, 0], ['a crate\'s side in the middle of the room', 31, 8, 1, 0]];
  const bad = cases.filter(([, x, z, nx, nz]) => { const [qx, qz] = diceFacePointOf(s, x, z, nx, nz); return sideAt(s, qx, qz) !== store; });
  const far = diceFacePointOf(s, 31, 8, 1, 0), wall = diceFacePointOf(s, 33.88, 8, -1, 0);
  check(bad.length === 0 && Math.abs(far[0] - 32) < 1e-6 && Math.abs(wall[0] - 32.88) < 1e-6, 'a face that looks at a wall a hand away reads the cache on its own side of the wall; a face with a metre of air reads a metre out, as ever', bad.map(c => c[0]).join('; ')); }

// 5. a door: open, its cells are every place's; shut again, the words are as they were; the frame's packing says the same
{ hang('store', 1.4); settle(); const o = build(); let doorCells = 0; for (let z = 7.8; z < 8.8; z += 0.1) for (let x = 27.8; x <= 28.2; x += 0.1) if (sideAt(o.s, x, z) === SIDE_DOOR) doorCells++;
  const stillTwo = sideAt(o.s, 31, 8) !== sideAt(o.s, 25, 8) && sideAt(o.s, 31, 8) < SIDE_UNKNOWN;
  check(doorCells > 0 && stillTwo, 'with the store\'s door open its doorway is every place\'s, and the store and the kitchen are still two places', `${doorCells} points of the doorway, store ${sideAt(o.s, 31, 8)}, kitchen ${sideAt(o.s, 25, 8)}`);
  const ws = new WallSides(); const first = upd(ws), again = upd(ws);
  hang('store', 0); settle(); const moved = upd(ws); let differ = 0; for (let z = 0.2; z < 28; z += 0.37) for (let x = 0.2; x < 40; x += 0.37) if (sideAt(ws.words, x, z) !== sideAt(s, x, z)) differ++;
  check(first && !again && moved && differ === 0 && ws.words.length === N, 'the frame\'s words are made again only when a door or a wall moves, and say what the map says', `${ws.words.length} words, ${differ} points that differ`);
  // a leaf that comes to rest: closed in 400 small steps from every angle, the words at rest are a fresh build's (a key that stepped every 0.04 rad once left a
  // leaf that stopped inside the step it was last keyed in kept the words of a leaf still ajar); and a swing makes the words a handful of times, not once a frame
  let stale = 0, closings = 0, made = 0, frames = 0;
  for (const d of g.doors.list) for (const from of [1.4, 0.9, 0.31, 0.11]) { const name = d.def.name, live = new WallSides(); closings++;
    for (let n = 400; n >= 0; n--) { hang(name, from * n / 400 * (n % 7 === 3 ? 0.999 : 1)); W.step(1 / 60); if (BRK && n < 3) continue; upd(live); frames++; }
    made += live.made; const fresh = build().s; for (let k = 0; k < N; k++) if (live.words[k] !== fresh[k]) { stale++; break; } }
  for (const d of g.doors.list) hang(d.def.name, 0); settle();
  check(stale === 0 && made < frames * 0.1, 'a door closed slowly from any angle leaves the words a fresh build would make, and a swing makes them a few times, not every frame', `${closings} closings, ${stale} left stale; the words made ${made} times in ${frames} frames`); }

// 6. the column table: the place of each probe column, a nibble a column, is what the plan says at that column (a read's four columns come from it, not from four asks of the plan)
{ const ws = new WallSides(); upd(ws, 40, 28); let wrong = 0, sided = 0;
  for (let j = 0; j < 28; j++) for (let i = 0; i < 40; i++) { const k = j * 40 + i, c = (ws.cols[k >> 3] >>> ((k & 7) * 4)) & 15, want = sideAt(ws.words, ox + i + 0.5, oz + j + 0.5); if (c !== want) wrong++; if (c < SIDE_UNKNOWN) sided++; }
  const big = new WallSides(); upd(big, 80, 56); let wrongBig = 0;
  for (let j = 0; j < 56; j++) for (let i = 0; i < 80; i++) { const k = j * 80 + i, c = (big.cols[k >> 3] >>> ((k & 7) * 4)) & 15; if (c !== sideAt(big.words, ox + (i + 0.5) * 0.5, oz + (j + 0.5) * 0.5)) wrongBig++; }
  const huge = new WallSides(); upd(huge, 120, 84);
  check(ws.fits && wrong === 0 && sided > 500 && big.fits && wrongBig === 0 && !huge.fits && huge.cols.every((v: number) => v === 0), 'the column table says what the plan says at each probe column, at a metre a probe and at the finest cascade the debugger offers; one it cannot hold leaves the flag down', `40 × 28: ${wrong} columns that differ, ${sided} in a known place; 80 × 56: ${wrongBig} that differ; 120 × 84: fits ${huge.fits}`); }

// 7. the shader's own numbers for the plan are the map's (common.wgsl says them as constants: a map of another size would have every word read from the wrong cell)
{ const c = readFileSync(`${ROOT}/src/render/shaders/common.wgsl`, 'utf8'), num = (n: string) => +(c.match(new RegExp(`const ${n}: \\w+ = ([0-9.]+)u?;`))?.[1] ?? NaN);
  const frameTs = readFileSync(`${ROOT}/src/render/frame.ts`, 'utf8'), words = +(c.match(/sides: array<vec4u, (\d+)>/)?.[1] ?? NaN) * 4, cols = +(c.match(/sideCols: array<vec4u, (\d+)>/)?.[1] ?? NaN) * 4;
  const { SIDE_COL_WORDS, SIDE_WORDS } = await import(`${ROOT}/src/render/wallSides.ts`);
  check(num('SIDE_W') === PLACE_W && num('SIDE_H') === PLACE_H && num('SIDE_RES') === PLACE_RES && num('SIDE_UNKNOWN') === SIDE_UNKNOWN && num('SIDE_DOOR') === SIDE_DOOR && words === SIDE_WORDS && cols === SIDE_COL_WORDS && frameTs.includes('u.set(sc, 7240)'),
    'the shader reads the plan at the map\'s own size and the frame holds as many words as the tables have', `common.wgsl ${num('SIDE_W')} × ${num('SIDE_H')} at ${num('SIDE_RES')} a metre, ${words} + ${cols} words · the map ${PLACE_W} × ${PLACE_H} at ${PLACE_RES}, ${SIDE_WORDS} + ${SIDE_COL_WORDS}`); }

// 8. THE EXPOSURE'S QUESTION: which place a pixel is of (place_common.wgsl lookPlaceLevels and placeSideCell, the same rule step for step: the shader asks its candidate cells in one loop, with
//    the field's cells given by hand: a doorway's cell and a wall's hold none). The maintainer on his GPU, 11-04: "weird flash on the door when opening",
//    "there's a dark band right by the entry of the door threshold and door", and of the depot at midday "lots of bleeding in that level".
{ const comp = readFileSync(`${ROOT}/src/render/shaders/composite.wgsl`, 'utf8'), compact = readFileSync(`${ROOT}/src/render/shaders/compact/composite.wgsl`, 'utf8'), pcm = readFileSync(`${ROOT}/src/render/shaders/place_common.wgsl`, 'utf8');
  const rule = pcm.slice(pcm.indexOf("/** THE PIXEL'S PLACE IS READ OFF THE FLOOR PLAN"), pcm.indexOf('/** THE THREE GAINS OF THE PLACE A POINT OF THE AIR IS IN')), num = (n: string) => +(pcm.match(new RegExp(`const ${n}: f32 = ([0-9.]+);`))?.[1] ?? NaN);   // (the rule is place_common.wgsl's, one text: the composite's two writings and the particles' splat are built with it)
  const TOP = num('PLACE_TOP'), WALL = num('PLACE_WALL'), PAST = num('PLACE_PAST'), SAME = num('PLACE_SAME'), PAST_SHARE = 0.0005 /* (1 in 2,000: the corners where masses stand three deep, 80 points on the day it was written, most in one warehouse) */, cl = (v: number, a: number, b: number) => Math.min(Math.max(v, a), b);
  const cellAt = (lv: Float32Array, i: number, j: number) => lv[cl(j, 0, PLACE_H - 1) * PLACE_W + cl(i, 0, PLACE_W - 1)];
  const blend = (lv: Float32Array, qx: number, qz: number, ci: number, cj: number, own: number): number => {
    const fx = cl(qx - ci, 0, 1) - 0.5, fz = cl(qz - cj, 0, 1) - 0.5, sx = fx >= 0 ? 1 : -1, sz = fz >= 0 ? 1 : -1, ax = Math.abs(fx), az = Math.abs(fz);
    const vx = cellAt(lv, ci + sx, cj), vz = cellAt(lv, ci, cj + sz), vd = cellAt(lv, ci + sx, cj + sz); let ws = (1 - ax) * (1 - az), sum = own * ws;
    if (vx > -500) { const w = ax * (1 - az); sum += w * vx; ws += w; } if (vz > -500) { const w = (1 - ax) * az; sum += w * vz; ws += w; }
    if (vd > -500 && (vx > -500 || vz > -500)) { const w = ax * az; sum += w * vd; ws += w; }
    return sum / ws; };
  /** placeSideCell: [the cell's i, its j (−1 = none)] */
  const sideCell = (words: Uint32Array, lv: Float32Array, mx: number, mz: number, ci: number, cj: number, oi: number, oj: number): [number, number] => { const [d0x, d0z] = sideLine(words, ci, cj, mx, mz);
    for (let r = 0; r < 6; r++) { ci += oi; cj += oj; if (cellAt(lv, ci, cj) > -500) return [ci, cj];
      const [dx, dz] = sideLine(words, ci, cj, mx, mz);
      if (sideIsDoor(words, ci, cj)) { if (dx === 0 && dz !== 0 && oi !== 0) { oi = 0; oj = Math.sign(dz); } else if (dz === 0 && dx !== 0 && oj !== 0) { oi = Math.sign(dx); oj = 0; } }
      else if ((dx * oi < -PAST && (d0x === 0 || Math.abs(dx - d0x) > SAME)) || (dz * oj < -PAST && (d0z === 0 || Math.abs(dz - d0z) > SAME))) return [-1, -1]; }
    return [-1, -1]; };
  /** lookPlaceLevels: [the level, which rule answered: 'cell', 'side', 'top', 'ring', 'own', the answering cell's i and j (−1 where it is not one cell's)] of
   *  the point (x, z); `up` = a face that looks up, `y` its height, `own` log2 of its own light */
  const levelAt = (words: Uint32Array, lv: Float32Array, x: number, z: number, up = false, y = 0, own = 0): [number, string, number, number] => {
    const mx = x - ox, mz = z - oz, qx = mx * PLACE_RES, qz = mz * PLACE_RES, ci = Math.floor(qx), cj = Math.floor(qz), mine = cellAt(lv, ci, cj), [dx, dz] = sideLine(words, ci, cj, mx, mz); let ai = -1, aj = -1;
    const across = (wi: number, wj: number, oi: number, oj: number): number => { const [a, b] = sideCell(words, lv, mx, mz, wi, wj, oi, oj); if (a < 0) return -1000; ai = a; aj = b; return blend(lv, qx, qz, a, b, cellAt(lv, a, b)); };
    if (up && y > TOP) { let wi = ci, wj = cj, tx = dx, tz = dz;
      if (mine > -500) { const sx = qx - Math.floor(qx) >= 0.5 ? 1 : -1, sz = qz - Math.floor(qz) >= 0.5 ? 1 : -1, nx = sideLine(words, ci + sx, cj, mx, mz), nz = sideLine(words, ci, cj + sz, mx, mz), near = (d: [number, number]) => (d[0] !== 0 || d[1] !== 0) && Math.max(Math.abs(d[0]), Math.abs(d[1])) <= WALL;
        if (near(nx)) { wi = ci + sx; [tx, tz] = nx; } else if (near(nz)) { wj = cj + sz; [tx, tz] = nz; } }
      const oi = tx !== 0 && Math.abs(tx) <= WALL ? Math.sign(tx) : 0, oj = tz !== 0 && Math.abs(tz) <= WALL ? Math.sign(tz) : 0;
      if (oi !== 0 || oj !== 0) { let best = -1000;
        if (oi !== 0 && oj !== 0) best = Math.max(across(wi, wj, oi, oj), across(wi, wj, -oi, -oj), across(wi, wj, oi, -oj), across(wi, wj, -oi, oj));
        else if (tx !== 0 && tz !== 0) { const pi = oi === 0 ? Math.sign(tx) : 0, pj = oj === 0 ? Math.sign(tz) : 0; best = Math.max(across(wi, wj, oi + pi, oj + pj), across(wi, wj, -oi + pi, -oj + pj)); }
        else best = Math.max(across(wi, wj, oi, oj), across(wi, wj, -oi, -oj));
        if (best > -500) return [best, 'top', -1, -1]; } }
    if (mine > -500) return [blend(lv, qx, qz, ci, cj, mine), 'cell', ci, cj];
    const oi = Math.sign(dx), oj = Math.sign(dz); if (oi !== 0 || oj !== 0) { const a = across(ci, cj, oi, oj); if (a > -500) return [a, 'side', ai, aj]; }
    let hi = -1000; ai = aj = -1;
    for (let pass = 0; pass < 2 && hi < -500; pass++) { if (pass === 0 && oi === 0 && oj === 0) continue;
      for (let r = 1; r <= 4 && hi < -500; r++) for (let dj = -r; dj <= r; dj++) for (let di = -r; di <= r; di++) { if (pass === 0 && ((oi !== 0 && di * oi < 0) || (oj !== 0 && dj * oj < 0))) continue; const v = cellAt(lv, ci + di, cj + dj); if (v > hi) { hi = v; ai = cl(ci + di, 0, PLACE_W - 1); aj = cl(cj + dj, 0, PLACE_H - 1); } } }
    return hi < -500 ? [own, 'own', -1, -1] : [hi, 'ring', ai, aj]; };
  /** the field's cells by hand over a map: the store's floor at −8, every other free cell at 0, walls and doorways none */
  const fieldOf = (mk: Uint8Array): Float32Array => { const lv = new Float32Array(N).fill(-1000), seen = new Uint8Array(N), q = [Math.floor((8 - oz) * PLACE_RES) * PLACE_W + Math.floor((31 - ox) * PLACE_RES)]; seen[q[0]] = 1;
    while (q.length) { const k = q.pop()!; for (const d of [1, -1, PLACE_W, -PLACE_W]) { const t2 = k + d; if (t2 < 0 || t2 >= N || seen[t2] || mk[t2] === 255 || (mk[t2] & 3) === 1) continue; seen[t2] = 1; q.push(t2); } }
    for (let k = 0; k < N; k++) if (mk[k] !== 255 && (mk[k] & 3) !== 1) lv[k] = seen[k] ? -8 : 0; return lv; };
  // (a) THE FLASH: a door mid swing. Every cell of every door's line knows its line at every angle of the leaf, so no point of a doorway or of a door
  //     that has begun to open is ever left to the rule for cells that say nothing
  { let looked = 0, silent = 0; const at: string[] = [];
    for (const d0 of g.doors.list) { for (const o of g.doors.list) hang(o.def.name, 0); settle(); const shut = build().m; hang(d0.def.name, d0.maxAuth); settle(); const wide = build().m, cells: number[] = []; for (let k = 0; k < N; k++) if (shut[k] === 255 && wide[k] !== 255) cells.push(k);
      for (let a = d0.minAuth; a <= d0.maxAuth + 1e-6; a += 0.05) { hang(d0.def.name, Math.abs(a) < 0.02 ? 0 : a); settle(); const b = build();
        for (const k of cells) { looked++; const x = ox + ((k % PLACE_W) + 0.5) / PLACE_RES, z = oz + (Math.floor(k / PLACE_W) + 0.5) / PLACE_RES, [dx, dz] = sideOff(b.s, x, z); if (dx === 0 && dz === 0) { silent++; if (at.length < 3) at.push(`${d0.def.name} at ${a.toFixed(2)} rad: (${x.toFixed(1)}, ${z.toFixed(1)})`); } } } }
    for (const o of g.doors.list) hang(o.def.name, 0); settle();
    check(looked > 1000 && silent === 0, 'every cell of a door\'s line knows the line at every angle of its leaf: a door that has begun to swing leaves no cell that says nothing', `${looked} cells of ${g.doors.list.length} doors' lines over their whole travel, ${silent} that said nothing${at.length ? `: ${at.join('; ')}` : ''}`); }
  // (b) THROUGH AN OPEN DOORWAY each side of the door's line is its own room to the line, and there is no third level between them: the store's door (the
  //     wall on x = 28) at three angles of its leaf, a walk from the kitchen to the store on three lines through it, a millimetre a step
  { let bad = 0, looked = 0, rules = new Set<string>(); const at: string[] = [];
    for (const ang of [0.3, 0.8, 1.4]) { hang('store', ang); settle(); const o = build(), lv = fieldOf(o.m);
      for (let x = 27.3; x <= 28.7001; x += 0.001) for (const z of [8.1, 8.3, 8.5]) { const [v, r] = levelAt(o.s, lv, x, z), want = x < 28 ? 0 : -8; looked++; rules.add(r); if (Math.abs(v - want) > 1e-6) { bad++; if (at.length < 3) at.push(`leaf at ${ang}: (${x.toFixed(3)}, ${z}) is ${v.toFixed(1)} by ${r}`); } } }
    check(looked > 12000 && bad === 0 && !rules.has('ring') && !rules.has('own') && rules.has('side'),
      'through an open doorway the kitchen\'s floor is the kitchen\'s to the door\'s line and the store\'s is the store\'s from it, at every angle of the leaf: no level between the two, no hairline of the wrong room at the line, and no point left to a guess', `${looked} points a millimetre apart on three lines through the store\'s door with its leaf at 0.3, 0.8 and 1.4 rad: ${bad} that are not their own room\'s${at.length ? `: ${at.join('; ')}` : ''}; answered by: ${[...rules].join(', ')}`);
    hang('store', 0); settle(); }
  // (c) AT A WALL'S FOOT AND ON ITS FACES a point is of the room it faces, to the number: the store's walls with every door shut
  { const lv = fieldOf(m), pts: [string, number, (t: number) => [number, number]][] = [['the outer wall, inside', -8, t => [34 - 0.15, 5.4 + t * 5.2]], ['the outer wall, outside', 0, t => [34 + 0.15, 5.4 + t * 5.2]], ['the bunk wall, the store\'s face', -8, t => [28.4 + t * 5.2, 11 - 0.15]], ['the bunk wall, the bunk\'s face', 0, t => [28.4 + t * 5.2, 11 + 0.15]], ['the kitchen wall and its shut door, the store\'s face', -8, t => [28 + 0.15, 5.4 + t * 5.2]], ['the kitchen wall and its shut door, the kitchen\'s face', 0, t => [28 - 0.15, 5.4 + t * 5.2]]];
    const bad: string[] = []; let looked = 0, guessed = 0; for (const [name, want, p] of pts) for (let k = 0; k <= 52; k++) { const [x, z] = p(k / 52), [v, r] = levelAt(s, lv, x, z); looked++; if (r === 'ring' || r === 'own') guessed++; if (Math.abs(v - want) > 1e-6) bad.push(`${name} at (${x.toFixed(2)}, ${z.toFixed(2)}): ${v.toFixed(1)} by ${r}`); }
    check(bad.length === 0 && guessed === 0, 'the floor at a wall\'s foot and a point a hand off its face, or off a shut door\'s, take the level of the room they face and no other', `${looked} points on six faces of the store\'s walls, ${bad.length} wrong${bad.length ? `: ${bad.slice(0, 3).join('; ')}` : ''}`); }
  // (d) A WALL'S TOP is of neither room: the brighter of the two its line parts, whatever light falls on it. The store's outer wall, the dark store under it
  { const lv = fieldOf(m), mid = levelAt(s, lv, 34, 8, true, 2.6, -12), inEdge = levelAt(s, lv, 33.89, 8, true, 2.6, 1.5), outEdge = levelAt(s, lv, 34.11, 8, true, 2.6, -12), foot = levelAt(s, lv, 33.95, 8, true, 0, 1.5), cab = levelAt(s, lv, 33.5, 8, true, 2.0, 1.5), cab2 = levelAt(s, lv, 33.7, 8, true, 2.0, 1.5);
    check(mid[0] === 0 && mid[1] === 'top' && inEdge[0] === 0 && inEdge[1] === 'top' && outEdge[0] === 0 && foot[0] === -8 && foot[1] === 'side' && cab[0] === -8 && cab[1] === 'cell' && cab2[0] === -8 && cab2[1] === 'side', 'a wall top between the yard and a dark room is shown as the yard is from edge to edge, the part that overhangs the room\'s own cell too; the floor at that wall\'s foot is the room\'s, and so is a tall thing\'s top half a metre inside it and one that stands against the wall, in the wall\'s own cell', `the top of the store\'s outer wall at its middle and 1 cm in from each edge: ${mid[0]}, ${inEdge[0]}, ${outEdge[0]} (the yard 0, the store −8); the floor inside its foot: ${foot[0]}; a cabinet top 0.5 m in: ${cab[0]} by ${cab[1]}, and one 0.3 m in, 18 cm clear of the wall\'s face: ${cab2[0]} by ${cab2[1]}`); }
  // (e) the kernel says the same in both of its writings, and the guess is gone from it: no read of the radiance cache and no box ray by the pixel for its place
  { const a = rule, body = a.slice(a.indexOf('fn lookPlaceLevels')), air = pcm.slice(pcm.indexOf('fn placeAir('), pcm.indexOf("/** THE PIXEL'S PLACE IS READ OFF THE FLOOR PLAN")), splat = readFileSync(`${ROOT}/src/render/shaders/particles_draw.wgsl`, 'utf8');
    check(a.length > 3000 && [comp, compact].every(t => !/lookPlaceHere|lookPlaceField|fn lookPlaceLevels|fn placeSideCell|fn placeBlend/.test(t) && (t.match(/lookPlaceLevels\(/g) ?? []).length === 1) && (splat.match(/lookPlaceLevels\(/g) ?? []).length === 1 && splat.includes('let lv = lookPlaceLevels(X, vec3f(0.0, 1.0, 0.0), lit);') && !/occludedTBoxes|diceMeanRadiance|c0Spacing|dicePos/.test(body) && body.includes('let d = sideLine(c, m);') && body.includes('for (var ask = select(1, 0, N.y > 0.7 && P.y > PLACE_TOP); ask < 2; ask++) {') && (body.match(/placeSideCell\(/g) ?? []).length === 1 && (body.match(/placeBlend\(/g) ?? []).length === 1 && body.includes('if (best.x > -500.0) { return exp2(best); }') && body.includes('if ((dx.x != 0.0 || dx.y != 0.0) && max(abs(dx.x), abs(dx.y)) <= PLACE_WALL) { cw = c + vec2i(s.x, 0); dt = dx; }') && body.includes('let t = vec2i(select(0, i32(sign(dt.x)), dt.x != 0.0 && abs(dt.x) <= PLACE_WALL), select(0, i32(sign(dt.y)), dt.y != 0.0 && abs(dt.y) <= PLACE_WALL));') && body.includes('if (t.x != 0 && t.y != 0) { n = 4; dirs[0] = t; dirs[1] = -t; dirs[2] = vec2i(t.x, -t.y); dirs[3] = vec2i(-t.x, t.y); }') && body.includes('n = 2; dirs[0] = t + p; dirs[1] = -t + p; }') && body.includes('if (ask == 0 && ((o.x != 0 && dx * o.x < 0) || (o.y != 0 && dy * o.y < 0))) { continue; }') && body.includes('return exp2(vec2f(hi, max(nh, hi)));') && Number.isFinite(TOP) && Number.isFinite(WALL) && Number.isFinite(PAST)
      && a.includes('    c += o; if (placeCell(c) > -500.0) { return c; }\n    let d = sideLine(c, m);\n    if (sideIsDoor(c)) { if (d.x == 0.0 && d.y != 0.0 && o.x != 0) { o = vec2i(0, i32(sign(d.y))); } else if (d.y == 0.0 && d.x != 0.0 && o.y != 0) { o = vec2i(i32(sign(d.x)), 0); } }\n    else if ((d.x * f32(o.x) < -PLACE_PAST && (d0.x == 0.0 || abs(d.x - d0.x) > PLACE_SAME)) || (d.y * f32(o.y) < -PLACE_PAST && (d0.y == 0.0 || abs(d.y - d0.y) > PLACE_SAME))) { return vec2i(-9999); }') && Number.isFinite(SAME) && a.includes('  for (var r = 0; r < 6; r++) {') && air.length > 200 && !air.includes('sideLine')
      && readFileSync(`${ROOT}/src/render/shaders/place_field.wgsl`, 'utf8').includes('if ((m & 3u) == 1u) { field[i] = PLACE_SOLID; field[PLACE_NEAR + i] = max(max(h0.z, h1.z), max(h2.z, h3.z)); return; }'),
      'the rule is one text, which both writings of the composite and the particles\' splat are built with and each asks in one place; it asks the plan and the field alone (no read of the cache and no box ray for a pixel\'s place) with one call of the walk and one of the blend, and the field gives a doorway\'s cell no level', `${a.length} characters of the rule; a top is a face that looks up above ${TOP} m within ${WALL} m of a wall\'s line`); }
  // (f) EVERY LEVEL, its doors wide and then shut, the rooms numbered as the plan parts them: (1) along the way through every doorway the answer changes
  //     ONCE, where its two rooms meet (a doorway two cells deep was four bands of them, turn about, when each cell had a line of its own); (2) a point
  //     in a wall's cell or a doorway's is answered by a cell IT CAN SEE: nothing as tall as a wall, and no shut door's line, lies between the point and
  //     the middle of the cell that answers for it (the foot of a wall that ends in a doorway was shown by the room behind the next wall; the pocket
  //     behind a shelf's end panel by the room across the partition); (3) the top of an upright wall is the top rule's from edge to edge, and the top of a
  //     tall thing that is no wall is its room's
  { const l0 = console.log, w0 = console.warn; let levels = 0, runs = 0, striped = 0, sides = 0, rings = 0, past = 0, tops = 0, topsOff = 0, things = 0, thingsOff = 0; const exS: string[] = [], exP: string[] = [], exT: string[] = [], exC: string[] = [];
    const { readdirSync } = await import('node:fs'); const { placeWall } = await import(`${ROOT}/src/scene/placePlan.ts`);
    for (const name of readdirSync(`${ROOT}/levels`).filter((f: string) => f.endsWith('.json') && !f.includes('.lint')).map((f: string) => f.replace(/\.json$/, ''))) {
      console.log = console.warn = () => { }; let X: any; try { X = await standUp({ level: loadLevel(parseLevelData(readFileSync(`${ROOT}/levels/${name}.json`, 'utf8'))), seed: 1 }); for (let i = 0; i < 4; i++) X.step(1 / 60); } catch { console.log = l0; console.warn = w0; continue; }
      console.log = l0; console.warn = w0; const gg = X.game, wd = gg.engine.world, list: any[] = gg.doors.list; gg.aiEnabled = false; levels++;
      const gaps = list.map((d: any) => { const [hx, hz] = d.def.hinge, a = d.def.closedDir, w = d.def.width; return [hx, hz, hx + Math.cos(a) * w, hz + Math.sin(a) * w]; });
      for (const open of [true, false]) { for (const d of list) { const a = open ? d.maxAuth : 0; d.latched = !open; d.vel = 0; d.angle = a; } X.step(1 / 60); X.step(1 / 60);
        const mk = placeMask(wd.statics, wd.dynamics, undefined, gaps) as Uint8Array, sw = placeSides(wd.statics, mk, BRK_DOOR ? [] : gaps) as Uint32Array, free = (k: number) => mk[k] !== 255 && (mk[k] & 3) !== 1, isD = (k: number) => k >= 0 && k < N && mk[k] !== 255 && (mk[k] & 3) === 1;
        const lv = new Float32Array(N).fill(-1000); let nc = 0;   // (each room its own number, as a level)
        for (let k0 = 0; k0 < N; k0++) { if (!free(k0) || lv[k0] > -500) continue; const st = [k0]; lv[k0] = nc; while (st.length) { const c = st.pop()!, i = c % PLACE_W, j = (c - i) / PLACE_W; for (const [di, dj] of [[1, 0], [-1, 0], [0, 1], [0, -1]]) { const x = i + di, z = j + dj; if (x < 0 || z < 0 || x >= PLACE_W || z >= PLACE_H) continue; const t2 = z * PLACE_W + x; if (free(t2) && lv[t2] < -500) { lv[t2] = nc; st.push(t2); } } } nc++; }
        const seen = new Set<string>();
        // (what stands between two points of the plan: the upright boxes a wall's height, as rectangles, and with the doors shut each door's line)
        const rects: [number, number, number, number][] = []; for (const b of wd.statics) { if (placeWall(b) !== 255 || Math.abs(Math.sin(b.yaw) * Math.cos(b.yaw)) > 0.02) continue; const turned = Math.abs(Math.sin(b.yaw)) > 0.5, hx = turned ? b.h[2] : b.h[0], hz = turned ? b.h[0] : b.h[2]; rects.push([b.c[0] - hx, b.c[2] - hz, b.c[0] + hx, b.c[2] + hz]); }
        if (!open) for (const [ax, az, bx, bz] of gaps) rects.push([Math.min(ax, bx) - 0.01, Math.min(az, bz) - 0.01, Math.max(ax, bx) + 0.01, Math.max(az, bz) + 0.01]);
        const inside = (x: number, z: number) => rects.some(r => x > r[0] - 0.01 && x < r[2] + 0.01 && z > r[1] - 0.01 && z < r[3] + 0.01);
        const blocked = (x0: number, z0: number, x1: number, z1: number) => rects.some(r => { let t0 = 0, t1 = 1; for (const [p, d, lo, hi] of [[x0, x1 - x0, r[0], r[2]], [z0, z1 - z0, r[1], r[3]]]) { if (Math.abs(d) < 1e-9) { if (p <= lo || p >= hi) return false; continue; } const a = (lo - p) / d, b2 = (hi - p) / d; t0 = Math.max(t0, Math.min(a, b2)); t1 = Math.min(t1, Math.max(a, b2)); if (t0 >= t1) return false; } return (t1 - t0) * Math.hypot(x1 - x0, z1 - z0) > 0.005; });   // (half a centimetre of it or more inside a thing)
        /** does the point see any cell of room `room` within two metres (a thing in the way of the one cell that answered is nothing if the room is in sight) */
        const sees = (x: number, z: number, room: number) => { const ci = Math.floor((x - ox) * PLACE_RES), cj = Math.floor((z - oz) * PLACE_RES); for (let dj = -5; dj <= 5; dj++) for (let di = -5; di <= 5; di++) { const a = ci + di, b = cj + dj; if (a < 0 || b < 0 || a >= PLACE_W || b >= PLACE_H || lv[b * PLACE_W + a] !== room) continue; if (!blocked(x, z, ox + (a + 0.5) / PLACE_RES, oz + (b + 0.5) / PLACE_RES)) return true; } return false; };
        for (let k = 0; k < N; k++) { const i = k % PLACE_W, j = (k - i) / PLACE_W, kind = sw[k] & 3;
          if (kind === 1 || kind === 3) { const x = ox + (i + 0.5) / PLACE_RES, z = oz + (j + 0.5) / PLACE_RES; for (const [ex, ez] of [[-0.15, -0.15], [0.15, -0.15], [-0.15, 0.15], [0.15, 0.15]]) { const px = x + ex, pz = z + ez; if (inside(px, pz)) continue; const r = levelAt(sw, lv, px, pz); if (r[1] === 'ring') rings++; if (r[1] !== 'side') continue; sides++;
            if (blocked(px, pz, ox + (r[2] + 0.5) / PLACE_RES, oz + (r[3] + 0.5) / PLACE_RES) && !sees(px, pz, lv[r[3] * PLACE_W + r[2]])) { past++; if (exP.length < 4) exP.push(`${name}${open ? '' : ' (doors shut)'} (${px.toFixed(2)}, ${pz.toFixed(2)}) by the cell at (${(ox + (r[2] + 0.5) / PLACE_RES).toFixed(1)}, ${(oz + (r[3] + 0.5) / PLACE_RES).toFixed(1)})`); } } }
          if (kind !== 3 || !isD(k)) continue; const hasX = (sw[k] >>> 2) & 1, hasZ = (sw[k] >>> 8) & 1;
          for (const axis of [0, 1]) { if (!(axis === 0 ? hasX : hasZ)) continue; const step = axis === 0 ? 1 : PLACE_W;
            let a = k; while (isD(a - step) && (axis === 1 || (a % PLACE_W) > 0)) a -= step; let b = k; while (isD(b + step) && (axis === 1 || (b % PLACE_W) < PLACE_W - 1)) b += step; const key = `${axis}:${a}:${b}:${axis === 0 ? j : i}`; if (seen.has(key)) continue; seen.add(key);
            const ai = a % PLACE_W, aj = (a - ai) / PLACE_W, n = (b - a) / step + 1; let prev = NaN, changes = 0; runs++;
            for (let t2 = 0; t2 < n * 200; t2++) { const u = (t2 + 0.5) / 200, x = axis === 0 ? ox + (ai + u) / PLACE_RES : ox + (i + 0.5) / PLACE_RES, z = axis === 0 ? oz + (j + 0.5) / PLACE_RES : oz + (aj + u) / PLACE_RES, v = levelAt(sw, lv, x, z)[0]; if (t2 > 0 && v !== prev) changes++; prev = v; }
            if (changes > 1) { striped++; if (exS.length < 3) exS.push(`${name}${open ? '' : ' (doors shut)'} (${(ox + (ai + 0.5) / PLACE_RES).toFixed(1)}, ${(oz + (aj + 0.5) / PLACE_RES).toFixed(1)}): ${changes} changes over ${n} cells`); } } }
        if (!open) continue;
        for (const b of wd.statics) { if (placeWall(b) !== 255 || Math.abs(Math.sin(b.yaw) * Math.cos(b.yaw)) > 0.02 || b.c[1] + b.h[1] <= TOP) continue; const turned = Math.abs(Math.sin(b.yaw)) > 0.5, hx = turned ? b.h[2] : b.h[0], hz = turned ? b.h[0] : b.h[2];
          if (Math.min(hx, hz) <= 0.125 && Math.max(hx, hz) >= 0.6) {   // (a wall of the two thicknesses a level builds with, a pace long or more: its top from edge to edge, a millimetre in)
            for (const u of [-0.99, -0.75, -0.5, -0.25, 0, 0.25, 0.5, 0.75, 0.99]) for (let v = -0.99; v <= 0.991; v += 0.0825) { const along = hx > hz, x = b.c[0] + (along ? v : u) * hx, z = b.c[2] + (along ? u : v) * hz, r = levelAt(sw, lv, x, z, true, b.c[1] + b.h[1], 0); tops++; if (r[1] === 'cell' || r[1] === 'side') { topsOff++; if (exT.length < 3) exT.push(`${name} (${x.toFixed(3)}, ${z.toFixed(3)}) by ${r[1]}`); } } }
          else if (Math.min(hx, hz) >= 0.2) {   // (a tall thing that is no wall: a cabinet, a locker, a stack. Its top further than a wall's half thickness from the thing's own line and from every other is never the top rule's)
            for (let u = -1 + 1 / 16; u < 1; u += 1 / 8) for (let v = -1 + 1 / 16; v < 1; v += 1 / 8) { const x = b.c[0] + u * hx, z = b.c[2] + v * hz, r = levelAt(sw, lv, x, z, true, b.c[1] + b.h[1], 0), far = (hx < hz ? Math.abs(u) * hx : Math.abs(v) * hz) > WALL + 0.02; if (!far) continue;
              // (… and 0.3 m or more from the middle line of every wall and every other upright thing a wall's height, measured on the boxes themselves)
              if (rects.some(q => { if (q[0] === b.c[0] - hx && q[1] === b.c[2] - hz) return false; const wx = q[2] - q[0], wz = q[3] - q[1], cx = (q[0] + q[2]) / 2, cz = (q[1] + q[3]) / 2; return wx < wz ? Math.abs(x - cx) < 0.3 && z > q[1] - 0.3 && z < q[3] + 0.3 : Math.abs(z - cz) < 0.3 && x > q[0] - 0.3 && x < q[2] + 0.3; })) continue;
              things++; if (r[1] === 'top') { thingsOff++; if (exC.length < 3) exC.push(`${name} (${x.toFixed(2)}, ${z.toFixed(2)})`); } } } } } }
    check(levels >= 50 && runs > 400 && striped === 0 && sides > 20000 && past <= sides * PAST_SHARE && tops > 20000 && topsOff <= tops * 0.005 && things > 2000 && thingsOff <= things * 0.005,
      'on every level, doors wide and doors shut: the way through a doorway changes rooms once, a point at a wall\'s foot or in a doorway is answered by a cell it can see, the top of a wall is shown by the top\'s rule from edge to edge, and the top of a tall thing that is no wall is not', `${levels} levels: ${runs} doorway runs, ${striped} with more than one change${exS.length ? ` (${exS.join('; ')})` : ''}; ${sides} points in the cells with lines answered by their side, ${past} (${(100 * past / Math.max(sides, 1)).toFixed(3)} %) by a cell behind something a wall\'s height${exP.length ? ` (${exP.join('; ')})` : ''}, and ${rings} more with nothing on their side take the brightest cell near them (pockets between tall things: not judged); ${tops} points on the tops of upright walls a millimetre in from each edge, ${topsOff} (${(100 * topsOff / Math.max(tops, 1)).toFixed(2)} %) shown as one room\'s (under 0.5: walls beside another tall thing whose line the cell holds)${exT.length ? ` (${exT.join('; ')})` : ''}; ${things} points on the tops of tall things 0.3 m from every wall box, ${thingsOff} (${(100 * thingsOff / Math.max(things, 1)).toFixed(2)} %, under 0.5: beside a window wall, which is no one box) taken for a wall\'s top${exC.length ? ` (${exC.join('; ')})` : ''}`); }
}

console.log(`\n=== wallsides: ${red ? 'FAILED' : 'ALL PASS'} ===`);
process.exit(red ? 1 : 0);
