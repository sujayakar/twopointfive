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
// PLACE is what walls and shut doors enclose (render/placeField.ts placeMask, the map the exposure is by). This file turns that map into one small word a
// cell that the frame carries to every shader: a free cell's place; a wall cell's two places and the line between them (the wall's own centre line, to
// 2.5 cm), so a point a finger off a wall's face is known to be on its side though the map's cell is 0.4 m. A doorway's cell is every place's (light
// does cross a threshold). Where two walls meet in one cell, or a wall is turned, the cell says nothing and the read is as it was.
// Pure: the check's too (tools/qa/wallsides-check.ts). No GPU thing is made here.
import { Box } from '../scene/boxes';
import { WORLD } from '../scene/world';
import { PLACE, PLACE_W, PLACE_H, PLACE_RES, PlaceGap, placeMask, placeWall, leafShuts, stampCells, placeKey } from './placeField';

/** a place's number is 0‥13 (two places a read could blend never share one; places far apart may); 14 = nothing known here; 15 = a doorway's cell */
export const SIDE_UNKNOWN = 14, SIDE_DOOR = 15, SIDE_WORDS = PLACE_W * PLACE_H;
/** in what sideAt returns for a free cell, bit 4: every cell within a probe spacing of it (a metre and the cell's own half, three cells) is its own place */
export const SIDE_CLEAR = 16;
/** a box whose |cos × sin| of yaw is this or more is turned: it has no line along an axis of the plan (a leaf ajar; a wall built askew) */
export const SIDE_TURNED = 0.02;
const WHOLE = 255, REACH = 3, Q_STEP = 0.025, D4: readonly (readonly [number, number])[] = [[1, 0], [-1, 0], [0, 1], [0, -1]];

/** the word of every cell of the map (row-major, as the map), a u32. Bits 0‥1 = 0 a free cell (bits 2‥5 its place, or 15 in a doorway; bit 6 = clear, above) ·
 *  1 a wall cell that knows its lines (bit 2 = it has a line across x, bits 3‥7 where: the cell's centre + (q − 15.5) × 2.5 cm; bit 8 and bits 9‥13 the same for a
 *  line across z; then four places of four bits from bit 14: low x low z, high x low z, low x high z, high x high z, each side of each line the cell has) ·
 *  2 a wall that says nothing (two walls side by side in one cell, a turned wall between two places). A wall cell with no line that stands inside one place
 *  (a square pier in a room) is that place's, written as a free cell. `mask` = placeMask's of the same boxes and doors. */
export function placeSides(statics: readonly Box[], dynamics: readonly Box[], mask: Uint8Array, gaps: readonly PlaceGap[] = [], out = new Uint32Array(PLACE_W * PLACE_H)): Uint32Array {
  const W = PLACE_W, H = PLACE_H, N = W * H, cell = 1 / PLACE_RES, ox = WORLD.rcMin[0], oz = WORLD.rcMin[2];
  const free = (k: number) => mask[k] !== 255 && (mask[k] & 3) !== 1;
  // 1. the places: free cells that are no doorway's, joined side to side
  const comp = new Int16Array(N).fill(-1), stack: number[] = []; let nc = 0;
  for (let k0 = 0; k0 < N; k0++) { if (!free(k0) || comp[k0] >= 0) continue; comp[k0] = nc; stack.push(k0);
    while (stack.length) { const c = stack.pop()!, i = c % W, j = (c - i) / W;
      for (const [di, dj] of D4) { const x = i + di, z = j + dj; if (x < 0 || z < 0 || x >= W || z >= H) continue; const t = z * W + x; if (!free(t) || comp[t] >= 0) continue; comp[t] = nc; stack.push(t); } }
    nc++; }
  // 2. each wall cell's lines: the centre line of the wall that fills it, one across x and one across z. A WALL here is what makes the cell a wall on the
  //    map: one box that fills the whole height, or the boxes of one line that fill it between them (a sill, its pane and the lintel over it). A box that
  //    fills part of the height on a line of its own (a counter against the wall, a bench, a board hung on it) is furniture: it lays no line and stops none
  //    (it did, and a counter a hand off a wall's line left the wall's cells saying nothing: half of the silent cells of five levels). Two whole walls of
  //    one axis on different lines, a whole wall that is turned, or a whole square pier = nothing known here (step 6 may still know the place)
  const lineX = new Float32Array(N).fill(NaN), lineZ = new Float32Array(N).fill(NaN), thickX = new Float32Array(N), thickZ = new Float32Array(N), dead = new Uint8Array(N);
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
  const box = (b: Box) => { const bits = placeWall(b); if (!bits) return; const whole = bits === WHOLE, c = Math.cos(b.yaw), s = Math.sin(b.yaw);
    if (Math.abs(c * s) >= SIDE_TURNED) { if (whole) stampCells(b.c[0], b.c[2], b.h[0], b.h[2], b.yaw, kill); return; }   // (a turned wall, a leaf ajar)
    const turned = Math.abs(s) > 0.5, hx = turned ? b.h[2] : b.h[0], hz = turned ? b.h[0] : b.h[2];
    if (Math.abs(hx - hz) < 0.02) { if (whole) stampCells(b.c[0], b.c[2], b.h[0], b.h[2], b.yaw, kill); return; }   // (a square pier has no line)
    const a = hx < hz ? 0 : 1, x = a === 0 ? b.c[0] : b.c[2], t = Math.min(hx, hz);
    stampCells(b.c[0], b.c[2], b.h[0], b.h[2], b.yaw, whole ? k => lay(k, a, x, t) : k => layPart(k, a, x, t, bits)); };
  for (const b of statics) box(b); for (const b of dynamics) if (gaps.some(g => leafShuts(b, g))) box(b);   // (a leaf lays a line only where it shuts its door: open, it is no wall and must not silence the jamb's cell its hinge end lies in, render/placeField.ts placeMask)
  for (let a = 0; a < 2; a++) { const p = part[a], line = a === 0 ? lineX : lineZ, thick = a === 0 ? thickX : thickZ;
    for (let k = 0; k < N; k++) { if (mask[k] !== 255 || !Number.isNaN(line[k])) continue; const m = k * 2, w0 = p.bits[m] === WHOLE, w1 = p.bits[m + 1] === WHOLE;
      if (w0 && w1) dead[k] = 1; else if (w0 || w1) { line[k] = p.line[w0 ? m : m + 1]; thick[k] = p.thick[w0 ? m : m + 1]; } } }
  //    … and the crack beside a shut leaf, which the map shut from jamb to jamb: the door's own line
  for (const [ax, az, bx, bz] of gaps) { const dx = bx - ax, dz = bz - az, len = Math.hypot(dx, dz); if (!(len > 1e-3)) continue; const ux = dx / len, uz = dz / len;
    if (Math.abs(ux * uz) >= 0.02) continue; const a = Math.abs(ux) > 0.5 ? 1 : 0, x = a === 0 ? ax : az, pad = PLACE.gapNear, line = a === 0 ? lineX : lineZ;
    const i0 = Math.max(0, Math.floor((Math.min(ax, bx) - pad - ox) * PLACE_RES)), i1 = Math.min(W - 1, Math.floor((Math.max(ax, bx) + pad - ox) * PLACE_RES));
    const j0 = Math.max(0, Math.floor((Math.min(az, bz) - pad - oz) * PLACE_RES)), j1 = Math.min(H - 1, Math.floor((Math.max(az, bz) + pad - oz) * PLACE_RES));
    for (let j = j0; j <= j1; j++) for (let i = i0; i <= i1; i++) { const k = j * W + i; if (mask[k] !== 255 || !Number.isNaN(line[k])) continue;
      const px = ox + (i + 0.5) * cell - ax, pz = oz + (j + 0.5) * cell - az, t = px * ux + pz * uz; if (t >= -cell * 0.5 && t <= len + cell * 0.5 && Math.abs(px * uz - pz * ux) <= pad) line[k] = x; } }
  //    … and where a wall stops at another's face the cell at the foot of the first has only the second's line, and the corner's air is in it: it takes the first
  //    wall's line from the cell beside it, and is a corner with a place in each quarter
  { const addX: [number, number][] = [], addZ: [number, number][] = [], lim = (PLACE.gapNear + 15.5 * Q_STEP) - 0.2;
    for (let k = 0; k < N; k++) { if (mask[k] !== 255 || dead[k]) continue; const i = k % W, j = (k - i) / W, hasX = !Number.isNaN(lineX[k]), hasZ = !Number.isNaN(lineZ[k]); if (hasX === hasZ) continue;
      for (const d of [-1, 1]) { const x = hasZ ? i : i + d, z = hasZ ? j + d : j; if (x < 0 || z < 0 || x >= W || z >= H) continue; const t = z * W + x; if (mask[t] !== 255 || dead[t]) continue;
        if (hasZ && !Number.isNaN(lineX[t]) && Math.abs(lineX[t] - (ox + (i + 0.5) * cell)) <= lim) { addX.push([k, lineX[t]]); break; }
        if (hasX && !Number.isNaN(lineZ[t]) && Math.abs(lineZ[t] - (oz + (j + 0.5) * cell)) <= lim) { addZ.push([k, lineZ[t]]); break; } } }
    for (const [k, v] of addX) lineX[k] = v; for (const [k, v] of addZ) lineZ[k] = v; }
  // 3. a number for each place: two places with cells within a probe's reach of each other never share one (greedy; a level has few places round any one)
  const near: Set<number>[] = Array.from({ length: nc }, () => new Set<number>()), clear = new Uint8Array(N);
  for (let k = 0; k < N; k++) { const c = comp[k]; if (c < 0) continue; const i = k % W, j = (k - i) / W; let alone = 1;
    for (let dj = -REACH; dj <= REACH; dj++) for (let di = -REACH; di <= REACH; di++) { const x = i + di, z = j + dj; if (x < 0 || z < 0 || x >= W || z >= H) continue; const o = comp[z * W + x]; if (o !== c) { alone = 0; if (o >= 0) { near[c].add(o); near[o].add(c); } } }
    clear[k] = alone; }
  const tint = new Int8Array(nc).fill(-1);
  for (let c = 0; c < nc; c++) { let used = 0; for (const n of near[c]) if (tint[n] >= 0) used |= 1 << tint[n]; let t = 0; while (t < SIDE_UNKNOWN - 1 && (used >> t) & 1) t++; tint[c] = t; }
  // 4. what stands in each quarter of a wall cell: the first cell that is no wall, walking out from the cell away from its lines (across the one line it has, or
  //    cornerwise where it has two)
  const walk = (i: number, j: number, di: number, dj: number): number => {
    for (let n = 1; n <= 4; n++) { const x = i + di * n, z = j + dj * n; if (x < 0 || z < 0 || x >= W || z >= H) return SIDE_UNKNOWN; const t = z * W + x; if (mask[t] === 255) continue; return (mask[t] & 3) === 1 ? SIDE_DOOR : tint[comp[t]]; }
    return SIDE_UNKNOWN; };
  const q = (line: number, centre: number) => Math.round((line - centre) / Q_STEP + 15.5);
  //    … and a wall cell with no line that stands INSIDE one place (a square pier in a room, a turned crate stack as tall as the room): the four ways out of it
  //    all come to that place (a doorway's cell and the map's edge say nothing either way), so every point of the cell is in it. Two places = a wall between
  //    them that the plan cannot draw (a turned wall): the cell says nothing
  const within = (i: number, j: number): number => { let p = -1;
    for (const [di, dj] of D4) { const t = walk(i, j, di, dj); if (t >= SIDE_UNKNOWN) continue; if (p >= 0 && t !== p) return 2; p = t; }
    return p < 0 ? 2 : p << 2; };
  // 5. the words
  for (let k = 0; k < N; k++) { const i = k % W, j = (k - i) / W;
    if (mask[k] !== 255) { out[k] = ((mask[k] & 3) === 1 ? SIDE_DOOR : tint[comp[k]] | (clear[k] << 4)) << 2; continue; }
    const hasX = !Number.isNaN(lineX[k]), hasZ = !Number.isNaN(lineZ[k]), qx = hasX ? q(lineX[k], ox + (i + 0.5) * cell) : 0, qz = hasZ ? q(lineZ[k], oz + (j + 0.5) * cell) : 0;
    if (dead[k] || (!hasX && !hasZ) || qx < 0 || qx > 31 || qz < 0 || qz > 31) { out[k] = within(i, j); continue; }
    let w = 1 | (hasX ? 1 << 2 : 0) | (qx << 3) | (hasZ ? 1 << 8 : 0) | (qz << 9);
    for (let n = 0; n < 4; n++) w |= walk(i, j, hasX ? (n & 1 ? 1 : -1) : 0, hasZ ? (n & 2 ? 1 : -1) : 0) << (14 + 4 * n);
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
  if (kind === 0) return (w >>> 2) & 31; if (kind !== 1) return SIDE_UNKNOWN;
  const hx = (w >>> 2) & 1 && qx / PLACE_RES >= (i + 0.5) / PLACE_RES + (((w >>> 3) & 31) - 15.5) * Q_STEP ? 1 : 0, hz = (w >>> 8) & 1 && qz / PLACE_RES >= (j + 0.5) / PLACE_RES + (((w >>> 9) & 31) - 15.5) * Q_STEP ? 2 : 0;
  return (w >>> (14 + 4 * (hx + hz))) & 15;
}
/** is (x, z) in a cell with nothing but its own place in a probe's reach (the shader's early way out) */
export function sideClear(words: Uint32Array, x: number, z: number): boolean { const w = sideWordAt(words, x, z); return w !== SIDE_UNKNOWN && (w & SIDE_CLEAR) !== 0 && w < 32 && (w & 15) < SIDE_UNKNOWN; }

/** a number that changes when the words would. Not the map's own key (placeField.ts placeKey), on two counts. A leaf in mid-swing says nothing where it
 *  hangs whatever its angle, so it is keyed by a sixth of a quarter turn: the map's step made the words again on every frame of a swing, 2 ms each. And a
 *  leaf is keyed by whether it LIES ALONG AN AXIS, the very test placeSides makes: the map's steps fall at other angles, and a leaf that came to rest a
 *  hair inside the step it was last keyed in kept the words made while it was still turned (a shut door that said nothing; 6 closings of 168). */
export function sidesKey(statics: readonly Box[], dynamics: readonly Box[], gaps: readonly PlaceGap[], nx: number, nz: number): number {
  let key = placeKey(statics, [], gaps);
  for (const b of dynamics) { if (!placeWall(b)) continue; const turned = Math.abs(Math.cos(b.yaw) * Math.sin(b.yaw)) >= SIDE_TURNED;
    key = (Math.imul(key, 33) + (turned ? 7919 + Math.round(b.yaw / 0.26) : Math.round(b.yaw / (Math.PI / 2)) * 5 + Math.round((b.c[0] * 3 + b.c[2] * 7) * 10))) | 0; }
  return (Math.imul(key, 33) + nx * 131 + nz) | 0;
}
/** the words as the frame carries them, made again only when sidesKey says: a wall, a pane's box, a door's line, a leaf come to rest or gone from it */
export class WallSides {
  readonly words = new Uint32Array(SIDE_WORDS);
  /** the place of each probe column of the cache's first cascade, a nibble a column (index z × nx + x): what a read's four columns are, without asking the plan four times */
  readonly cols = new Uint32Array(SIDE_COL_WORDS);
  /** false = the cascade has more columns than the table holds: the frame's flag stays down and every read is the plain one */
  fits = true;
  /** times the words were made (the engine sends the tables to the GPU again only when this has moved) */
  made = 0;
  private key = NaN; private readonly mask = new Uint8Array(PLACE_W * PLACE_H);
  /** `map` = the map of these very boxes and doors as they stand, where the caller keeps one (placeField.ts PlaceMap): it is not drawn a second time */
  update(statics: readonly Box[], dynamics: readonly Box[], gaps: readonly PlaceGap[] = [], nx: number = WORLD.rcSize[0], nz: number = WORLD.rcSize[2], map: Uint8Array | null = null): boolean {
    const key = sidesKey(statics, dynamics, gaps, nx, nz); if (key === this.key) return false;
    this.key = key; placeSides(statics, dynamics, map ?? placeMask(statics, dynamics, this.mask, gaps), gaps, this.words); this.fits = sideCols(this.words, nx, nz, this.cols); this.made++; return true;
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
