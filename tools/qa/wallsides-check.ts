// WHICH SIDE OF A WALL (the look page's row `walls`; src/render/wallSides.ts, lighting_common.wgsl diceTaps and diceFacePointBy). The level's floor plan as the
// words every cache read asks, held to the pass: the map itself, and the read the shader makes of it, through the TS copies of the shader's own lines
// (wallSides.ts sideAt, diceTapsOf, diceFacePointOf). No GPU: that the WGSL says the same is tools/qa/wgsl-compile.cjs (it compiles) and the page's stills.
//   bun run tools/qa/wallsides-check.ts                    exit 0 = every line passed, 1 = one failed
//   WALLSIDES_BREAK=1 bun run tools/qa/wallsides-check.ts  the store's outer wall is left out of the map (the store and the yard are one place): must FAIL
import { readFileSync } from 'node:fs';
import { standUp, ROOT } from './headless.ts';
const { parseLevelData } = await import(`${ROOT}/src/scene/levelData.ts`);
const { loadLevel } = await import(`${ROOT}/src/scene/levelLoad.ts`);
const { placeMask, PLACE_W, PLACE_H, PLACE_RES } = await import(`${ROOT}/src/scene/placePlan.ts`);
const { placeSides, sideAt, sideClear, diceTapsOf, diceFacePointOf, WallSides, SIDE_UNKNOWN, SIDE_DOOR } = await import(`${ROOT}/src/render/wallSides.ts`);
const { WORLD } = await import(`${ROOT}/src/scene/world.ts`);
const BRK = process.env.WALLSIDES_BREAK === '1'; let red = false;
const check = (ok: boolean, what: string, detail = '') => { red ||= !ok; console.log(`- ${ok ? 'PASS' : 'FAIL'} — ${what}${detail ? `\n    ${detail}` : ''}`); };
console.log(`\n=== wallsides: the pass's floor plan as every cache read asks it${BRK ? ' (the store\'s outer wall left out: must fail)' : ''} ===`);
const l = console.log, w = console.warn; console.log = console.warn = () => { };
const W: any = await standUp({ level: loadLevel(parseLevelData(readFileSync(`${ROOT}/levels/pass.json`, 'utf8'))), seed: 1 }); for (let i = 0; i < 10; i++) W.step(1 / 60);
console.log = l; console.warn = w;
const g = W.game, world = g.engine.world;
const GAPS = g.doors.list.map((d: any) => { const [hx, hz] = d.def.hinge, a = d.def.closedDir, wd = d.def.width; return [hx, hz, hx + Math.cos(a) * wd, hz + Math.sin(a) * wd]; });
const statics = () => BRK ? world.statics.filter((b: any) => !(Math.abs(b.c[0] - 34) < 0.2 && b.h[0] < 0.3 && b.h[2] > 1)) : world.statics;   // (the break: no wall on the line x = 34)
const build = () => { const m = placeMask(statics(), world.dynamics, undefined, GAPS); return { m, s: placeSides(statics(), world.dynamics, m, GAPS) as Uint32Array }; };
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
  const ws = new WallSides(); const first = ws.update(statics(), world.dynamics, GAPS), again = ws.update(statics(), world.dynamics, GAPS);
  hang('store', 0); settle(); const moved = ws.update(statics(), world.dynamics, GAPS); let differ = 0; for (let z = 0.2; z < 28; z += 0.37) for (let x = 0.2; x < 40; x += 0.37) if (sideAt(ws.words, x, z) !== sideAt(s, x, z)) differ++;
  check(first && !again && moved && differ === 0 && ws.words.length === N, 'the frame\'s words are made again only when a door or a wall moves, and say what the map says', `${ws.words.length} words, ${differ} points that differ`);
  // a leaf that comes to rest: closed in 400 small steps from every angle, the words at rest are a fresh build's (the map's own key steps every 0.04 rad, and a
  // leaf that stopped inside the step it was last keyed in kept the words of a leaf still ajar); and a swing makes the words a handful of times, not once a frame
  let stale = 0, closings = 0, made = 0, frames = 0;
  for (const d of g.doors.list) for (const from of [1.4, 0.9, 0.31, 0.11]) { const name = d.def.name, live = new WallSides(); closings++;
    for (let n = 400; n >= 0; n--) { hang(name, from * n / 400 * (n % 7 === 3 ? 0.999 : 1)); W.step(1 / 60); if (BRK && n < 3) continue; live.update(statics(), world.dynamics, GAPS); frames++; }
    made += live.made; const fresh = build().s; for (let k = 0; k < N; k++) if (live.words[k] !== fresh[k]) { stale++; break; } }
  for (const d of g.doors.list) hang(d.def.name, 0); settle();
  check(stale === 0 && made < frames * 0.1, 'a door closed slowly from any angle leaves the words a fresh build would make, and a swing makes them a few times, not every frame', `${closings} closings, ${stale} left stale; the words made ${made} times in ${frames} frames`); }

// 6. the column table: the place of each probe column, a nibble a column, is what the plan says at that column (a read's four columns come from it, not from four asks of the plan)
{ const ws = new WallSides(); ws.update(statics(), world.dynamics, GAPS, 40, 28); let wrong = 0, sided = 0;
  for (let j = 0; j < 28; j++) for (let i = 0; i < 40; i++) { const k = j * 40 + i, c = (ws.cols[k >> 3] >>> ((k & 7) * 4)) & 15, want = sideAt(ws.words, ox + i + 0.5, oz + j + 0.5); if (c !== want) wrong++; if (c < SIDE_UNKNOWN) sided++; }
  const big = new WallSides(); big.update(statics(), world.dynamics, GAPS, 80, 56); let wrongBig = 0;
  for (let j = 0; j < 56; j++) for (let i = 0; i < 80; i++) { const k = j * 80 + i, c = (big.cols[k >> 3] >>> ((k & 7) * 4)) & 15; if (c !== sideAt(big.words, ox + (i + 0.5) * 0.5, oz + (j + 0.5) * 0.5)) wrongBig++; }
  const huge = new WallSides(); huge.update(statics(), world.dynamics, GAPS, 120, 84);
  check(ws.fits && wrong === 0 && sided > 500 && big.fits && wrongBig === 0 && !huge.fits && huge.cols.every((v: number) => v === 0), 'the column table says what the plan says at each probe column, at a metre a probe and at the finest cascade the debugger offers; one it cannot hold leaves the flag down', `40 × 28: ${wrong} columns that differ, ${sided} in a known place; 80 × 56: ${wrongBig} that differ; 120 × 84: fits ${huge.fits}`); }

// 7. the shader's own numbers for the plan are the map's (common.wgsl says them as constants: a map of another size would have every word read from the wrong cell)
{ const c = readFileSync(`${ROOT}/src/render/shaders/common.wgsl`, 'utf8'), num = (n: string) => +(c.match(new RegExp(`const ${n}: \\w+ = ([0-9.]+)u?;`))?.[1] ?? NaN);
  const frameTs = readFileSync(`${ROOT}/src/render/frame.ts`, 'utf8'), words = +(c.match(/sides: array<vec4u, (\d+)>/)?.[1] ?? NaN) * 4, cols = +(c.match(/sideCols: array<vec4u, (\d+)>/)?.[1] ?? NaN) * 4;
  const { SIDE_COL_WORDS, SIDE_WORDS } = await import(`${ROOT}/src/render/wallSides.ts`);
  check(num('SIDE_W') === PLACE_W && num('SIDE_H') === PLACE_H && num('SIDE_RES') === PLACE_RES && num('SIDE_UNKNOWN') === SIDE_UNKNOWN && num('SIDE_DOOR') === SIDE_DOOR && words === SIDE_WORDS && cols === SIDE_COL_WORDS && frameTs.includes('u.set(sc, 7240)'),
    'the shader reads the plan at the map\'s own size and the frame holds as many words as the tables have', `common.wgsl ${num('SIDE_W')} × ${num('SIDE_H')} at ${num('SIDE_RES')} a metre, ${words} + ${cols} words · the map ${PLACE_W} × ${PLACE_H} at ${PLACE_RES}, ${SIDE_WORDS} + ${SIDE_COL_WORDS}`); }

console.log(`\n=== wallsides: ${red ? 'FAILED' : 'ALL PASS'} ===`);
process.exit(red ? 1 : 0);
