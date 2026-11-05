// THE LOOK PROTOTYPE — WHICH SIDE OF A WALL (the look page's row `walls`; frame.sides, common.wgsl sideAt, lighting_common.wgsl diceIrradiance).
// The maintainer on his GPU, 11-01, the pass's store at midday with every door shut: "lots of light leak when doors are closed", and "weird that i went
// through all the buffers in the debug view and couldn't see what was causing the light leak really". Before it, 10-31: "i worry that we're finding these
// cases one by one and not fixing them systematically".
// THE FAULT, one fault with many faces. The radiance cache is a lattice of probes a metre apart and every read of it blends the probes round the point
// read. A wall is a hand thick, so a read within a metre of a wall blends in the probe on its far side, and a read made a metre off a face (as every
// surface read is) can stand on the far side outright. At night the far side is another lamp-lit room and the fault is a little warmth in the wrong
// place, and one rule after another was written for the cases that showed (a carton's back, a man by a wall, a boot print's lip). By day the far side of
// an outer wall is the open air, thousands of times brighter than a room shown that many times up: the floor behind a row of drums glowed white.
// THE RULE. The level says where its walls are, so every read asks: the place the read point is in is the only place whose probes it may blend. A
// PLACE is what walls and shut doors enclose (scene/placePlan.ts placeMask, the map the exposure is by). This file turns that map into one small word a
// cell that the frame carries to every shader: a free cell's place; a wall cell's two places and the line between them (the wall's own centre line, to
// 2.5 cm), so a point a finger off a wall's face is known to be on its side though the map's cell is 0.4 m. A doorway's cell is every place's (light
// does cross a threshold), and it knows its door's line too: which room a point in a doorway is of is which side of that line it is on (the
// exposure's question, composite.wgsl lookPlaceLevels; the cache's reads take a doorway's cell as every place's, as before). Where two walls meet in
// one cell, or a wall is turned, the cell says nothing and the read is as it was. NO LEAF IS IN THE PLAN (scene/placePlan.ts placeMask: doors open by
// degrees): a shut door's cells are a wall on the door's own line whatever angle its leaf has reached, so a door that has begun to swing says what a
// shut one says until its line opens.
// Pure: the check's too (tools/qa/wallsides-check.ts). No GPU thing is made here.
import { Box } from '../scene/boxes';
import { WORLD } from '../scene/world';
import { PLACE, PLACE_W, PLACE_H, PLACE_RES, PlaceGap, PlaceMap, placeWall, stampCells } from '../scene/placePlan';
import { type Pace, Steps, WHOLE as AT_ONCE, whole, paced as isPaced } from './steps';

/** a place's number is 0‥13 (two places a read could blend never share one; places far apart may); 14 = nothing known here; 15 = a doorway's cell */
export const SIDE_UNKNOWN = 14, SIDE_DOOR = 15, SIDE_WORDS = PLACE_W * PLACE_H;
/** in what sideAt returns for a free cell, bit 4: every cell within a probe spacing of it (a metre and the cell's own half, three cells) is its own place */
export const SIDE_CLEAR = 16;
/** a box whose |cos × sin| of yaw is this or more is turned: it has no line along an axis of the plan (a wall built askew) */
export const SIDE_TURNED = 0.02;
const WHOLE = 255, REACH = 3, Q_STEP = 0.025, D4: readonly (readonly [number, number])[] = [[1, 0], [-1, 0], [0, 1], [0, -1]];

/** the word of every cell of the map (row-major, as the map), a u32. Bits 0‥1 = 0 a free cell (bits 2‥5 its place, or 15 in a doorway; bit 6 = clear, above) ·
 *  1 a wall cell that knows its lines (bit 2 = it has a line across x, bits 3‥7 where: the cell's centre + (q − 15.5) × 2.5 cm; bit 8 and bits 9‥13 the same for a
 *  line across z; then four places of four bits from bit 14: low x low z, high x low z, low x high z, high x high z, each side of each line the cell has) ·
 *  2 a wall that says nothing (two walls side by side in one cell, a turned wall between two places) ·
 *  3 A DOORWAY'S CELL THAT KNOWS ITS LINE: the lines as a wall cell's (the same bits; no places), read by sideLine; to sideAt it is a doorway's cell, every
 *  place's. Its line is its door's (`gaps`), or for a doorway by its shape alone its run's one line (step 2: the jambs' own, else the run's middle).
 *  A wall cell with no line that stands inside one place (a square pier in a room) is that place's, written as a free cell. `mask` = placeMask's of the
 *  same boxes and doors. */
export function placeSides(statics: readonly Box[], mask: Uint8Array, gaps: readonly PlaceGap[] = [], out = new Uint32Array(PLACE_W * PLACE_H)): Uint32Array { return whole(placeSidesSteps(statics, mask, gaps, out)); }
const WALLS_A_STOP = 32, CELLS_A_STOP = 350;   // (a stop every so many wall boxes laid, and every so many cells of a pass over the map)
/** … MADE IN STEPS (render/steps.ts: in a browser this runs cold each time a wall changes, 6 to 18 ms of the office's blast frame in one go). EVERYTHING THAT
 *  CAN CHANGE UNDER IT IS TAKEN BEFORE THE FIRST STOP: the map's cells, the doors' lines, and of each box that is part of a wall the six numbers asked of
 *  it. From there it reads nothing but what it took, so the words it returns are those of the level as it stood when it began, however many frames it is
 *  given. `out` is written in its last steps and is nobody's until it is returned (WallSides hands it scratch of its own). */
export function* placeSidesSteps(statics: readonly Box[], mask0: Uint8Array, gaps0: readonly PlaceGap[] = [], out = new Uint32Array(PLACE_W * PLACE_H)): Generator<void, Uint32Array, void> {
  const W = PLACE_W, H = PLACE_H, N = W * H, cell = 1 / PLACE_RES, ox = WORLD.rcMin[0], oz = WORLD.rcMin[2];
  const mask = mask0.slice(), gaps = gaps0.map((g): PlaceGap => [g[0], g[1], g[2], g[3]]), wall = new Float64Array(statics.length * 6); let nw = 0;
  for (const b of statics) { const bits = placeWall(b); if (!bits) continue; wall[nw++] = bits; wall[nw++] = b.c[0]; wall[nw++] = b.c[2]; wall[nw++] = b.h[0]; wall[nw++] = b.h[2]; wall[nw++] = b.yaw; }
  yield;
  const free = (k: number) => mask[k] !== 255 && (mask[k] & 3) !== 1;
  // 1. the places: free cells that are no doorway's, joined side to side
  const comp = new Int16Array(N).fill(-1), stack: number[] = []; let nc = 0, popped = 0;
  for (let k0 = 0; k0 < N; k0++) { if (!free(k0) || comp[k0] >= 0) continue; comp[k0] = nc; stack.push(k0);
    while (stack.length) { if (++popped % CELLS_A_STOP === 0) yield; const c = stack.pop()!, i = c % W, j = (c - i) / W;
      for (const [di, dj] of D4) { const x = i + di, z = j + dj; if (x < 0 || z < 0 || x >= W || z >= H) continue; const t = z * W + x; if (!free(t) || comp[t] >= 0) continue; comp[t] = nc; stack.push(t); } }
    nc++; }
  yield;
  // 2. each wall cell's lines: the centre line of the wall that fills it, one across x and one across z. A WALL here is what makes the cell a wall on the
  //    map: one box that fills the whole height, or the boxes of one line that fill it between them (a sill, its pane and the lintel over it). A box that
  //    fills part of the height on a line of its own (a counter against the wall, a bench, a board hung on it) is furniture: it lays no line and stops none
  //    (it did, and a counter a hand off a wall's line left the wall's cells saying nothing: half of the silent cells of five levels). Two whole walls of
  //    one axis on different lines, a whole wall that is turned, or a whole square pier = nothing known here (step 6 may still know the place)
  const lineX = new Float32Array(N).fill(NaN), lineZ = new Float32Array(N).fill(NaN), thickX = new Float32Array(N), thickZ = new Float32Array(N), dead = new Uint8Array(N);
  const isDoor = (k: number) => mask[k] !== 255 && (mask[k] & 3) === 1;
  const lay = (k: number, a: number, x: number, t: number) => { if (mask[k] !== 255) return; const line = a === 0 ? lineX : lineZ, thick = a === 0 ? thickX : thickZ;
    if (Number.isNaN(line[k])) { line[k] = x; thick[k] = t; return; }
    if (Math.abs(line[k] - x) > 0.1) { dead[k] = 1; return; }
    if (t > thick[k]) { line[k] = x; thick[k] = t; } };
  const kill = (k: number) => { if (mask[k] === 255) dead[k] = 1; };
  //    (the parts of a made-up wall, by cell and axis: up to two lines, each with the slices of the height its boxes fill and its thickest box's line)
  const part = [0, 1].map(() => ({ line: new Float32Array(N * 2).fill(NaN), thick: new Float32Array(N * 2), bits: new Uint8Array(N * 2) }));
  const layPart = (k: number, a: number, x: number, t: number, bits: number) => { if (mask[k] !== 255) return; const p = part[a];
    for (let n = 0; n < 2; n++) { const m = k * 2 + n;
      if (Number.isNaN(p.line[m])) { p.line[m] = x; p.thick[m] = t; p.bits[m] = bits; return; }
      if (Math.abs(p.line[m] - x) <= 0.1) { p.bits[m] |= bits; if (t > p.thick[m]) { p.line[m] = x; p.thick[m] = t; } return; } } };
  const box = (o: number) => { const bits = wall[o], cx = wall[o + 1], cz = wall[o + 2], h0 = wall[o + 3], h2 = wall[o + 4], yaw = wall[o + 5], full = bits === WHOLE, c = Math.cos(yaw), s = Math.sin(yaw);
    if (Math.abs(c * s) >= SIDE_TURNED) { if (full) stampCells(cx, cz, h0, h2, yaw, kill); return; }   // (a turned wall)
    const turned = Math.abs(s) > 0.5, hx = turned ? h2 : h0, hz = turned ? h0 : h2;
    if (Math.abs(hx - hz) < 0.02) { if (full) stampCells(cx, cz, h0, h2, yaw, kill); return; }   // (a square pier has no line)
    const a = hx < hz ? 0 : 1, x = a === 0 ? cx : cz, t = Math.min(hx, hz);
    stampCells(cx, cz, h0, h2, yaw, full ? k => lay(k, a, x, t) : k => layPart(k, a, x, t, bits)); };
  for (let o = 0, n = 0; o < nw; o += 6) { box(o); if (++n % WALLS_A_STOP === 0) yield; }
  yield;
  for (let a = 0; a < 2; a++) { const p = part[a], line = a === 0 ? lineX : lineZ, thick = a === 0 ? thickX : thickZ;
    for (let k = 0; k < N; k++) { if (mask[k] !== 255 || !Number.isNaN(line[k])) continue; const m = k * 2, w0 = p.bits[m] === WHOLE, w1 = p.bits[m + 1] === WHOLE;
      if (w0 && w1) dead[k] = 1; else if (w0 || w1) { line[k] = p.line[w0 ? m : m + 1]; thick[k] = p.thick[w0 ? m : m + 1]; } } }
  //    … and a door's own line: for the cells of a shut door, which the map made a wall from jamb to jamb with no box in it, and for the cells of an open
  //    one (a doorway's cells: which side of the line a point is on is which room it is of)
  for (const [ax, az, bx, bz] of gaps) { const dx = bx - ax, dz = bz - az, len = Math.hypot(dx, dz); if (!(len > 1e-3)) continue; const ux = dx / len, uz = dz / len;
    if (Math.abs(ux * uz) >= 0.02) continue; const a = Math.abs(ux) > 0.5 ? 1 : 0, x = a === 0 ? ax : az, pad = PLACE.gapNear, line = a === 0 ? lineX : lineZ;
    const i0 = Math.max(0, Math.floor((Math.min(ax, bx) - pad - ox) * PLACE_RES)), i1 = Math.min(W - 1, Math.floor((Math.max(ax, bx) + pad - ox) * PLACE_RES));
    const j0 = Math.max(0, Math.floor((Math.min(az, bz) - pad - oz) * PLACE_RES)), j1 = Math.min(H - 1, Math.floor((Math.max(az, bz) + pad - oz) * PLACE_RES));
    for (let j = j0; j <= j1; j++) for (let i = i0; i <= i1; i++) { const k = j * W + i; if ((mask[k] !== 255 && !isDoor(k)) || !Number.isNaN(line[k])) continue;
      const px = ox + (i + 0.5) * cell - ax, pz = oz + (j + 0.5) * cell - az, t = px * ux + pz * uz; if (t >= -cell * 0.5 && t <= len + cell * 0.5 && Math.abs(px * uz - pz * ux) <= pad) line[k] = x; } }
  //    … and a doorway no door's line named (a doorway by its shape: an arch, a gap in a yard wall, a short passage), or the cells of a door's doorway
  //    its own line did not reach (a wall thicker than the line's reach). ONE LINE A RUN: its cells that touch are one doorway, and every cell of it
  //    holds the same line. (A line through the middle of EACH cell made a doorway two cells deep four bands of its two rooms, turn about: 158 runs on 29
  //    of 60 levels.) The line is, in this order: the line of a door whose own cells the run touches (one doorway, one line); else, across the way
  //    through, the centre line of the walls that are its jambs; else the run's middle along the way through. THE WAY THROUGH runs along the axis whose
  //    two hands are open: the jambs are the walls that TOUCH the run on both hands of the other. (Looking further than the touching cell took the back
  //    walls of two small rooms for jambs, and their doorway had walls on both axes and no line.) Jambs on both axes, or on neither: the plan cannot
  //    read a way through, and the cells say nothing.
  { const run = new Uint8Array(N), bare = (k: number) => isDoor(k) && Number.isNaN(lineX[k]) && Number.isNaN(lineZ[k]);
    for (let k0 = 0; k0 < N; k0++) { if (!bare(k0) || run[k0]) continue; const cells = [k0]; run[k0] = 1;
      let doorX = NaN, doorZ = NaN, jambX = NaN, jambZ = NaN, xm = false, xp = false, zm = false, zp = false, i0 = W, i1 = -1, j0 = H, j1 = -1;
      for (let n = 0; n < cells.length; n++) { const c = cells[n], i = c % W, j = (c - i) / W; i0 = Math.min(i0, i); i1 = Math.max(i1, i); j0 = Math.min(j0, j); j1 = Math.max(j1, j);
        for (const [di, dj] of D4) { const x = i + di, z = j + dj, off = x < 0 || z < 0 || x >= W || z >= H, t = z * W + x;
          if (off || mask[t] === 255) { if (di < 0) xm = true; else if (di > 0) xp = true; else if (dj < 0) zm = true; else zp = true;
            if (!off && !dead[t]) { if (di !== 0 && !Number.isNaN(lineZ[t]) && Number.isNaN(jambZ)) jambZ = lineZ[t]; if (dj !== 0 && !Number.isNaN(lineX[t]) && Number.isNaN(jambX)) jambX = lineX[t]; }
            continue; }
          if (bare(t)) { if (!run[t]) { run[t] = 1; cells.push(t); } continue; }
          if (isDoor(t)) { if (!Number.isNaN(lineX[t]) && Number.isNaN(doorX)) doorX = lineX[t]; if (!Number.isNaN(lineZ[t]) && Number.isNaN(doorZ)) doorZ = lineZ[t]; } } }
      const acrossX = xm && xp, acrossZ = zm && zp;   // (jambs on both x hands: the way through runs along z, and the line is one across z)
      if (!Number.isNaN(doorZ) && Number.isNaN(doorX)) { for (const c of cells) lineZ[c] = doorZ; continue; }
      if (!Number.isNaN(doorX) && Number.isNaN(doorZ)) { for (const c of cells) lineX[c] = doorX; continue; }
      if (acrossX === acrossZ) continue;
      if (acrossX) { const z = Number.isNaN(jambZ) ? oz + (j0 + j1 + 1) * 0.5 * cell : jambZ; for (const c of cells) lineZ[c] = z; } else { const x = Number.isNaN(jambX) ? ox + (i0 + i1 + 1) * 0.5 * cell : jambX; for (const c of cells) lineX[c] = x; } } }
  //    … and where a wall stops at another's face the cell at the foot of the first has only the second's line, and the corner's air is in it: it takes the first
  //    wall's line from the cell beside it, and is a corner with a place in each quarter
  { const addX: [number, number][] = [], addZ: [number, number][] = [], lim = (PLACE.gapNear + 15.5 * Q_STEP) - 0.2;
    for (let k = 0; k < N; k++) { if (mask[k] !== 255 || dead[k]) continue; const i = k % W, j = (k - i) / W, hasX = !Number.isNaN(lineX[k]), hasZ = !Number.isNaN(lineZ[k]); if (hasX === hasZ) continue;
      for (const d of [-1, 1]) { const x = hasZ ? i : i + d, z = hasZ ? j + d : j; if (x < 0 || z < 0 || x >= W || z >= H) continue; const t = z * W + x; if (mask[t] !== 255 || dead[t]) continue;
        if (hasZ && !Number.isNaN(lineX[t]) && Math.abs(lineX[t] - (ox + (i + 0.5) * cell)) <= lim) { addX.push([k, lineX[t]]); break; }
        if (hasX && !Number.isNaN(lineZ[t]) && Math.abs(lineZ[t] - (oz + (j + 0.5) * cell)) <= lim) { addZ.push([k, lineZ[t]]); break; } } }
    for (const [k, v] of addX) lineX[k] = v; for (const [k, v] of addZ) lineZ[k] = v; }
  yield;
  // 3. a number for each place: two places with cells within a probe's reach of each other never share one (greedy; a level has few places round any one)
  const near: Set<number>[] = Array.from({ length: nc }, () => new Set<number>()), clear = new Uint8Array(N);
  for (let k = 0; k < N; k++) { if (k > 0 && k % CELLS_A_STOP === 0) yield; const c = comp[k]; if (c < 0) continue; const i = k % W, j = (k - i) / W; let alone = 1;
    for (let dj = -REACH; dj <= REACH; dj++) for (let di = -REACH; di <= REACH; di++) { const x = i + di, z = j + dj; if (x < 0 || z < 0 || x >= W || z >= H) continue; const o = comp[z * W + x]; if (o !== c) { alone = 0; if (o >= 0) { near[c].add(o); near[o].add(c); } } }
    clear[k] = alone; }
  const tint = new Int8Array(nc).fill(-1);
  for (let c = 0; c < nc; c++) { let used = 0; for (const n of near[c]) if (tint[n] >= 0) used |= 1 << tint[n]; let t = 0; while (t < SIDE_UNKNOWN - 1 && (used >> t) & 1) t++; tint[c] = t; }
  // 4. what stands in each quarter of a wall cell: the first cell that is no wall, walking out from the cell away from its lines (across the one line it has, or
  //    cornerwise where it has two)
  const walk = (i: number, j: number, di: number, dj: number): number => {
    for (let n = 1; n <= 4; n++) { const x = i + di * n, z = j + dj * n; if (x < 0 || z < 0 || x >= W || z >= H) return SIDE_UNKNOWN; const t = z * W + x; if (mask[t] === 255) continue; if ((mask[t] & 3) === 1) return SIDE_DOOR; return tint[comp[t]]; }
    return SIDE_UNKNOWN; };
  // a line's place in its cell's word: steps of 2.5 cm about the cell's middle, 0‥31. A line ON THE CELL'S EDGE (a wall or a door on a cell boundary, as
  // most are) lies exactly between two steps, and rounding put it 1.25 cm inside one cell or the other by the float's noise: a hairline of the wrong room
  // along the line. It is written a step OUTSIDE the cell: the whole cell is then on one side, which is what an edge says. A doorway's line may lie outside
  // its cell altogether (its run's middle, cells away): the furthest step that way says the same.
  const q = (line: number, centre: number, far: boolean) => { const v = (line - centre) / Q_STEP + 15.5, edge = Math.abs(Math.abs(line - centre) - cell * 0.5) < 1e-4;
    const s = edge ? (line > centre ? Math.ceil(v - 1e-3) : Math.floor(v + 1e-3)) : Math.round(v); return far ? Math.min(31, Math.max(0, s)) : s; };
  //    … and a wall cell with no line that stands INSIDE one place (a square pier in a room, a turned crate stack as tall as the room): the four ways out of it
  //    all come to that place (a doorway's cell and the map's edge say nothing either way), so every point of the cell is in it. Two places = a wall between
  //    them that the plan cannot draw (a turned wall): the cell says nothing
  const within = (i: number, j: number): number => { let p = -1;
    for (const [di, dj] of D4) { const t = walk(i, j, di, dj); if (t >= SIDE_UNKNOWN) continue; if (p >= 0 && t !== p) return 2; p = t; }
    return p < 0 ? 2 : p << 2; };
  // 5. the words
  yield;
  for (let k = 0; k < N; k++) { if (k > 0 && k % CELLS_A_STOP === 0) yield; const i = k % W, j = (k - i) / W;
    const hasX = !Number.isNaN(lineX[k]), hasZ = !Number.isNaN(lineZ[k]), far = isDoor(k), qx = hasX ? q(lineX[k], ox + (i + 0.5) * cell, far) : 0, qz = hasZ ? q(lineZ[k], oz + (j + 0.5) * cell, far) : 0, lined = (hasX || hasZ) && qx >= 0 && qx <= 31 && qz >= 0 && qz <= 31;
    if (mask[k] !== 255 && !(isDoor(k) && lined)) { out[k] = ((mask[k] & 3) === 1 ? SIDE_DOOR : tint[comp[k]] | (clear[k] << 4)) << 2; continue; }
    if (mask[k] === 255 && (dead[k] || !lined)) { out[k] = within(i, j); continue; }
    let w = (mask[k] === 255 ? 1 : 3) | (hasX ? 1 << 2 : 0) | (qx << 3) | (hasZ ? 1 << 8 : 0) | (qz << 9);
    if (mask[k] === 255) for (let n = 0; n < 4; n++) w |= walk(i, j, hasX ? (n & 1 ? 1 : -1) : 0, hasZ ? (n & 2 ? 1 : -1) : 0) << (14 + 4 * n);   // (a doorway's cell is every place's to a read of the cache: it carries its lines and no places)
    out[k] = w >>> 0;
  }
  return out;
}
/** the place of point (x, z) by those words: 0‥13, SIDE_UNKNOWN, SIDE_DOOR. The shader's sideAt (common.wgsl) less its clear bit, line for line. */
export function sideAt(words: Uint32Array, x: number, z: number): number { return sideWordAt(words, x, z) & 15; }
/** … with the clear bit (+ SIDE_CLEAR), as the shader's returns it */
export function sideWordAt(words: Uint32Array, x: number, z: number): number {
  const qx = (x - WORLD.rcMin[0]) * PLACE_RES, qz = (z - WORLD.rcMin[2]) * PLACE_RES, i = Math.floor(qx), j = Math.floor(qz);
  if (i < 0 || j < 0 || i >= PLACE_W || j >= PLACE_H) return SIDE_UNKNOWN;
  const w = words[j * PLACE_W + i], kind = w & 3;
  if (kind === 0) return (w >>> 2) & 31; if (kind === 3) return SIDE_DOOR; if (kind !== 1) return SIDE_UNKNOWN;
  const hx = (w >>> 2) & 1 && qx / PLACE_RES >= (i + 0.5) / PLACE_RES + (((w >>> 3) & 31) - 15.5) * Q_STEP ? 1 : 0, hz = (w >>> 8) & 1 && qz / PLACE_RES >= (j + 0.5) / PLACE_RES + (((w >>> 9) & 31) - 15.5) * Q_STEP ? 2 : 0;
  return (w >>> (14 + 4 * (hx + hz))) & 15;
}
/** is (x, z) in a cell with nothing but its own place in a probe's reach (the shader's early way out) */
export function sideClear(words: Uint32Array, x: number, z: number): boolean { const w = sideWordAt(words, x, z); return w !== SIDE_UNKNOWN && (w & SIDE_CLEAR) !== 0 && w < 32 && (w & 15) < SIDE_UNKNOWN; }

/** THE LINES OF ONE CELL, asked of any point: how far past each line of cell (i, j) the point (mx, mz), metres from the map's corner, is, toward +x and +z
 *  (0 on an axis the cell has no line across, never 0 on one it has); [0, 0] for a cell with no lines and off the map. Its sign is the step, in cells,
 *  toward the point's own side. The shader's sideLine (common.wgsl), line for line. */
export function sideLine(words: Uint32Array, i: number, j: number, mx: number, mz: number): [number, number] {
  if (i < 0 || j < 0 || i >= PLACE_W || j >= PLACE_H) return [0, 0];
  const w = words[j * PLACE_W + i]; if ((w & 1) === 0) return [0, 0];
  const dx = mx - ((i + 0.5) / PLACE_RES + (((w >>> 3) & 31) - 15.5) * Q_STEP), dz = mz - ((j + 0.5) / PLACE_RES + (((w >>> 9) & 31) - 15.5) * Q_STEP), off = (d: number) => (d >= 0 ? Math.max(d, 1e-6) : Math.min(d, -1e-6));
  return [(w >>> 2) & 1 ? off(dx) : 0, (w >>> 8) & 1 ? off(dz) : 0];
}
/** is cell (i, j) a doorway's that knows its line (the shader's sideIsDoor) */
export function sideIsDoor(words: Uint32Array, i: number, j: number): boolean { return i >= 0 && j >= 0 && i < PLACE_W && j < PLACE_H && (words[j * PLACE_W + i] & 3) === 3; }
/** … of the cell the point (x, z) is in (a tool's: the shader asks sideLine of the cell it has in hand) */
export function sideOff(words: Uint32Array, x: number, z: number): [number, number] { const mx = x - WORLD.rcMin[0], mz = z - WORLD.rcMin[2]; return sideLine(words, Math.floor(mx * PLACE_RES), Math.floor(mz * PLACE_RES), mx, mz); }
/** the words as the frame carries them, made again only when the map they are read off changes its walls or its doorways (or the cascade's size moves).
 *  WHERE THE MAP ONLY OPENED (a wall holed, a door's line opened) THEY ARE MADE OVER A FEW FRAMES (render/steps.ts; `pace` = the frame's allowance), the
 *  words and columns last made serving until the new ones are whole: for those frames a read by a hole just blown keeps to its own side as it did before
 *  the blast, and the cache it reads takes longer than that to hear of the hole. One begun is finished, on what it took as it began, a step at least
 *  every frame, and if the map has moved on since, the next begins the frame after: walls that change every frame cannot keep the words from being made.
 *  AT ONCE, as ever: the first time; WHERE THE MAP CLOSED A CELL (a door's line shut, a wall put back, a box set down: `PlaceMap.closed`; the old words
 *  know nothing of a cell that has become a wall, and the map and the field are fresh that same frame); when the cascade's size moves (the columns are
 *  numbered by it: old ones would name the wrong columns); on a new level; and with no allowance given (a tool, a check, the engine's frames after the
 *  picture has started over). */
export class WallSides {
  readonly words = new Uint32Array(SIDE_WORDS);
  /** the place of each probe column of the cache's first cascade, a nibble a column (index z × nx + x): what a read's four columns are, without asking the plan four times */
  readonly cols = new Uint32Array(SIDE_COL_WORDS);
  /** false = the cascade has more columns than the table holds: the frame's flag stays down and every read is the plain one */
  fits = true;
  /** times the words were made (the engine sends the tables to the GPU again only when this has moved) */
  made = 0;
  private key = NaN; private from: PlaceMap | null = null; private nx = 0; private nz = 0; private level: Box | null = null; private closed = -1;
  private readonly job = new Steps<boolean>(); private jobKey = NaN; private readonly nextWords = new Uint32Array(SIDE_WORDS); private readonly nextCols = new Uint32Array(SIDE_COL_WORDS);
  /** is a making in hand */
  get busy(): boolean { return this.job.busy; }
  /** what is in hand is given up (the rows that read the words are off, or the picture is starting over): the next asking begins again */
  drop(): void { this.job.drop(); }
  private *make(statics: readonly Box[], gaps: readonly PlaceGap[], nx: number, nz: number, mask: Uint8Array): Generator<void, boolean, void> {
    yield* placeSidesSteps(statics, mask, gaps, this.nextWords); yield;
    return sideCols(this.nextWords, nx, nz, this.nextCols);
  }
  /** `map` = the map of these very boxes and doors as they stand (scene/placePlan.ts PlaceMap, updated by the caller this frame): the one map of the level */
  update(statics: readonly Box[], gaps: readonly PlaceGap[], nx: number, nz: number, map: PlaceMap, pace: Pace = AT_ONCE): boolean {
    const key = map.shape * 65536 + nx * 131 + nz;   // (`shape`: which cells are a wall's and which a doorway's. How far a door stands open is not in the words, so a swing makes them when its line opens or shuts and at no step between)
    if (!this.job.busy && key === this.key && map === this.from) return false;
    const level = statics[0] ?? null;   // (a new level's statics are new boxes)
    const paced = isPaced(pace) && this.made > 0 && map === this.from && nx === this.nx && nz === this.nz && level === this.level && map.closed === this.closed;
    let fits: boolean;
    try {
      if (!paced) { this.job.drop(); this.jobKey = key; fits = whole(this.make(statics, gaps, nx, nz, map.mask)); }
      else {
        if (!this.job.busy) { this.job.begin(this.make(statics, gaps, nx, nz, map.mask)); this.jobKey = key; }   // (what it takes of the level, it takes in this call: the job's first step)
        const got = this.job.go(pace); if (got === undefined) return false; fits = got;
      }
    } catch (e) { this.key = this.jobKey; this.from = map; this.nx = nx; this.nz = nz; this.level = level; this.closed = map.closed; throw e; }   // (a planner that throws is not asked again for the same map: the words last made serve, as they did when the key was kept before the work)
    this.fits = fits; this.words.set(this.nextWords); this.cols.set(this.nextCols); this.key = this.jobKey; this.from = map; this.nx = nx; this.nz = nz; this.level = level; this.closed = map.closed; this.made++; return true;
  }
}
/** the column table for a cascade of nx × nz columns over the cache's plan; false (and the table cleared) when it does not hold that many. It holds the
 *  finest cascade the debugger offers (80 × 56, half-metre probes): at a size it did not hold, the rule was off for the whole level with the older guards
 *  compiled out. */
export const SIDE_COL_WORDS = 560;
export function sideCols(words: Uint32Array, nx: number, nz: number, out: Uint32Array): boolean {
  out.fill(0); if (nx * nz > SIDE_COL_WORDS * 8) return false;
  const sx = WORLD.rcSize[0] / nx, sz = WORLD.rcSize[2] / nz;
  for (let j = 0; j < nz; j++) for (let i = 0; i < nx; i++) { const k = j * nx + i; out[k >> 3] |= sideAt(words, WORLD.rcMin[0] + (i + 0.5) * sx, WORLD.rcMin[2] + (j + 0.5) * sz) << ((k & 7) * 4); }
  return true;
}

/** THE SHADER'S READ, IN TS, for the check alone (lighting_common.wgsl diceTaps, line for line; `dims` and `sp` = the cache's first cascade in plan: probes a
 *  cell, metres a probe). The lattice columns a read at (x, z) blends and each one's share: [column x index, column z index, share]. */
export function diceTapsOf(words: Uint32Array, x: number, z: number, dims: readonly [number, number] = [WORLD.rcSize[0], WORLD.rcSize[2]], sp: readonly [number, number] = [1, 1]): [number, number, number][] {
  const ox = WORLD.rcMin[0], oz = WORLD.rcMin[2], cl = (v: number, a: number, b: number) => Math.min(Math.max(v, a), b);
  const gx = cl((x - ox) / sp[0] - 0.5, 0, dims[0] - 1), gz = cl((z - oz) / sp[1] - 0.5, 0, dims[1] - 1), ix = Math.floor(gx), iz = Math.floor(gz), fx = gx - ix, fz = gz - iz;
  const jx = Math.min(ix + 1, dims[0] - 1), jz = Math.min(iz + 1, dims[1] - 1), plain: [number, number, number][] = [[ix, iz, (1 - fx) * (1 - fz)], [jx, iz, fx * (1 - fz)], [ix, jz, (1 - fx) * fz], [jx, jz, fx * fz]];
  const word = (px: number, pz: number) => sideWordAt(words, px, pz);
  const own = word(x, z); if (own >= SIDE_UNKNOWN) return plain;
  const cx0 = ox + (ix + 0.5) * sp[0], cx1 = ox + (jx + 0.5) * sp[0], cz0 = oz + (iz + 0.5) * sp[1], cz1 = oz + (jz + 0.5) * sp[1];
  const ok = (px: number, pz: number) => { const c = word(px, pz) & 15; return c === own || c >= SIDE_UNKNOWN ? 1 : 0; };
  const m00 = ok(cx0, cz0), m10 = ok(cx1, cz0), m01 = ok(cx0, cz1), m11 = ok(cx1, cz1), any = m00 + m10 + m01 + m11; if (any === 4 || any === 0) return plain;
  const wx0 = Math.max(1 - fx, 1e-3), wx1 = Math.max(fx, 1e-3), wz0 = Math.max(1 - fz, 1e-3), wz1 = Math.max(fz, 1e-3);
  const a0 = wx0 * m00, b0 = wx1 * m10, a1 = wx0 * m01, b1 = wx1 * m11, r0 = (a0 + b0) * wz0, r1 = (a1 + b1) * wz1, tot = r0 + r1;
  const row = (a: number, b: number, r: number, k: number): [number, number, number][] => (r <= 0 ? [] : [[ix, k, r / tot * a / (a + b)], [jx, k, r / tot * b / (a + b)]]);
  return [...row(a0, b0, r0, iz), ...row(a1, b1, r1, jz)].filter(t => t[2] > 0) as [number, number, number][];
}
/** … and where a face at (sx, sz) with plan normal (nx, nz) reads, `off` metres out (lighting_common.wgsl diceFacePointBy): [x, z] */
export function diceFacePointOf(words: Uint32Array, sx: number, sz: number, nx: number, nz: number, off = 1): [number, number] {
  const word = (px: number, pz: number) => sideWordAt(words, px, pz);
  const Q: [number, number] = [sx + nx * off, sz + nz * off], own = word(sx + nx * 0.04, sz + nz * 0.04); if (own >= SIDE_UNKNOWN) return Q;
  const q = word(Q[0], Q[1]) & 15; if (q === own || q === SIDE_DOOR) return Q;
  const H: [number, number] = [sx + nx * off * 0.5, sz + nz * off * 0.5], h = word(H[0], H[1]) & 15; if (h === own || h === SIDE_DOOR) return H;
  const d = Math.min(off, 0.12); return [sx + nx * d, sz + nz * d];
}
