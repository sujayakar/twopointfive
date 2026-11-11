// MARKINGS — object legibility from one or two SIGNIFIERS per object class, made of the same thing the world is made of: rectangles, in 2D, ON a box
// face (the maintainer's ask 08-10: "the monitor looks great but the keyboard doesn't … cardboard boxes are hard to recognize
// … vending machine"). REPRESENTATION (agreed with the maintainer: image-style authoring, analytic rendering): a marking is a tiny PALETTE-INDEXED
// BITMAP — W × H ≤ 32 × 32 cells, 2 bits a cell (0 = transparent, 1‥3 = palette entries), each palette entry a MATERIAL (albedo — absolute or × the
// box's own — roughness, emitted radiance), never a shaded colour, so baked lighting is impossible by construction — laid on a world-space texel
// lattice anchored at the face centre (MARKING_TEXEL 2.5 cm base; `fine` = half, `coarse` = double, all nesting). The g-buffer / composite / the
// reflection pass evaluate it ANALYTICALLY (render/shaders/markings.wgsl): cell = floor(face uv ÷ texel), 2 bits fetched from the per-box table, palette
// lookup, coverage against the cell edges for anti-aliasing ONLY at cell boundaries (crisp pixel art that never shimmers under the TAAU jitter), a fade to
// the marking's own mean under about a pixel a texel (a tint at verylow, never noise), cone-averaged at reflection hits. No image files, no texture
// bindings, no sampling in any ray pass. PROCEDURAL kinds (key grids, seams + pulls, text bars, LED rows, tape, can rows) are bitmap GENERATORS below;
// HAND-DRAWN kinds (this-side-up / fragile glyphs, a label, a scribble) are ASCII-art rows ('.' 'a' 'b' 'c'). The level file stores the cells (top row
// first) — canonical, deterministic, greppable. Readable text / logos would come later as an SDF-glyph atlas add-on to this same table, not now.
//
// This file also computes each marking's per-entry COVERAGE FRACTIONS (the mean the shader fades to; area-averaged over the box, the albedo the secondary
// rays see — boxes.ts upload: GI energy right at zero ray cost) and its MEAN EMISSION (an emissive marking is a LIGHT: rust/game_rules/src/tick/game registers a one-sided
// rectangular emitter for it exactly like an emissive fixture box, so ReSTIR shadows, the beams and the bounce all see it).
import type { Vec3 } from '../math/vec';

/** the base world-space texel (m): a key cap, a text bar, an LED are one of these (fine = half, coarse = double) */
export const MARKING_TEXEL = 0.025;
export const MAX_MARKS_PER_BOX = 4;
export const MAX_CELLS_W = 32; export const MAX_CELLS_H = 32; 
/** GPU table: vec4f rows in the tail of BoxWorld.matBuf; a record = MARK_HEAD_ROWS header rows + ceil(W·H / 48) cell rows (12 cells a float, exact small integers) */
export const MARK_HEAD_ROWS = 8;  export const CELLS_PER_ROW = 48;
export const MARK_TABLE_ROWS = 6144;   // 96 KB of table (the office packs ≈ 1.5k rows)
export const MARK_SLOT_MAX = (1 << 14) - 1;   // the 1-based first-record ROW in the flags word's top 14 bits

export type MarkFace = '+x' | '-x' | '+y' | '-y' | '+z' | '-z';
export const MARK_FACES: readonly MarkFace[] = ['+x', '-x', '+y', '-y', '+z', '-z'];   // index = the g-buffer's face order (gbuffer.wgsl cubeCorner)
export type MarkRes = 'fine' | 'base' | 'coarse';
export const RES_K: Record<MarkRes, number> = { fine: 0.5, base: 1, coarse: 2 };

/** One palette entry = a MATERIAL: `albedo` absolute linear rgb, else `mul` × the box's own albedo (absent both = the box's albedo untouched — a roughness-only
 *  or emission-only entry); `rough` (absent = the box's); `emit` = emitted radiance rgb (W/sr/m², absent = none — × the switchable's supply at runtime). */
export interface MarkPal { albedo?: Vec3; mul?: number; rough?: number; emit?: Vec3; }
export interface MarkingDef {
  face: MarkFace;
  /** texel = MARKING_TEXEL × ½ / 1 / 2; absent = base */
  res?: MarkRes;
  /** lattice coords (integers, this marking's texel) of the bitmap's BOTTOM-LEFT cell, from the face centre along the face's (u, v) axes (FACE_AXES) */
  at: [number, number];
  /** W, H in cells (1‥32) */
  size: [number, number];
  pal: MarkPal[];
  /** H rows TOP FIRST, each W chars of '.' (transparent) 'a' 'b' 'c' (pal[0‥2]) */
  cells: string[];
}
export const CELL_CHARS = '.abc';

/** Box-local (u, v, n) axes per face — u to the right and v up as seen from OUTSIDE for the vertical faces (gbuffer.wgsl cubeCorner's basis); +y: u = +x, v = −z */
export const FACE_AXES: readonly { u: Vec3; v: Vec3; n: Vec3 }[] = [
  { u: [0, 0, -1], v: [0, 1, 0], n: [1, 0, 0] }, { u: [0, 0, 1], v: [0, 1, 0], n: [-1, 0, 0] },
  { u: [1, 0, 0], v: [0, 0, -1], n: [0, 1, 0] }, { u: [1, 0, 0], v: [0, 0, 1], n: [0, -1, 0] },
  { u: [1, 0, 0], v: [0, 1, 0], n: [0, 0, 1] }, { u: [-1, 0, 0], v: [0, 1, 0], n: [0, 0, -1] },
];
export function faceHalf(h: Vec3, f: number): { hu: number; hv: number; hn: number } {
  const ax = FACE_AXES[f]; const pick = (a: Vec3) => Math.abs(a[0]) * h[0] + Math.abs(a[1]) * h[1] + Math.abs(a[2]) * h[2];
  return { hu: pick(ax.u), hv: pick(ax.v), hn: pick(ax.n) };
}
export const faceIndex = (f: MarkFace) => MARK_FACES.indexOf(f);
export const texelOf = (m: { res?: MarkRes }, base = MARKING_TEXEL) => base * RES_K[m.res ?? 'base'];

/** One MarkingDef with only the keys it means, in canonical order (levelData KEYS.marking / KEYS.markpal) — every producer goes through here so two markings that
 *  mean the same thing are the same structure (the level identity probe compares them leaf for leaf). */
export function marking(m: MarkingDef): MarkingDef {
  const o: MarkingDef = { face: m.face, ...(m.res !== undefined && m.res !== 'base' ? { res: m.res } : {}), at: [m.at[0], m.at[1]], size: [m.size[0], m.size[1]], pal: m.pal.map(palEntry), cells: m.cells.slice() } as MarkingDef;
  return o;
}
export function palEntry(p: MarkPal): MarkPal {
  const o: MarkPal = {};
  if (p.albedo) o.albedo = [p.albedo[0], p.albedo[1], p.albedo[2]]; else if (p.mul !== undefined && p.mul !== 1) o.mul = p.mul;
  if (p.rough !== undefined) o.rough = p.rough;
  if (p.emit && (p.emit[0] > 0 || p.emit[1] > 0 || p.emit[2] > 0)) o.emit = [p.emit[0], p.emit[1], p.emit[2]];
  return o;
}
export const copyMarkings = (ms: readonly MarkingDef[] | undefined): MarkingDef[] | undefined => ms && ms.length ? ms.map(marking) : undefined;

// ------------------------------------------------------------------------------------------------------------------------------------ authoring: the cell canvas + generators
/** A W × H cell canvas (j = 0 is the BOTTOM row; toRows() prints top first). The procedural kinds draw into one; several shapes on one face region share one. */
export class CellCanvas {
  readonly cells: Uint8Array;
  constructor(readonly W: number, readonly H: number) { if (W < 1 || H < 1 || W > MAX_CELLS_W || H > MAX_CELLS_H) throw new Error(`CellCanvas ${W}×${H}: 1‥32 each`); this.cells = new Uint8Array(W * H); }
  set(i: number, j: number, e: number) { if (i >= 0 && j >= 0 && i < this.W && j < this.H) this.cells[j * this.W + i] = e; return this; }
  get(i: number, j: number) { return i >= 0 && j >= 0 && i < this.W && j < this.H ? this.cells[j * this.W + i] : 0; }
  /** fill the cell rect [i0, i1) × [j0, j1) with entry e */
  fill(e: number, i0 = 0, j0 = 0, i1 = this.W, j1 = this.H) { for (let j = Math.max(j0, 0); j < Math.min(j1, this.H); j++) for (let i = Math.max(i0, 0); i < Math.min(i1, this.W); i++) this.cells[j * this.W + i] = e; return this; }
  toRows(): string[] { const rows: string[] = []; for (let j = this.H - 1; j >= 0; j--) { let s = ''; for (let i = 0; i < this.W; i++) s += CELL_CHARS[this.cells[j * this.W + i]]; rows.push(s); } return rows; }
}
/** the one integer hash the generators use for ragged text / can layouts (deterministic: the level file must not depend on Math.random) */
export function mkHash(s: number, a: number, b: number): number {
  let h = (Math.imul(s >>> 0, 0x9E3779B1) ^ Math.imul((a + 0x7F4A) >>> 0, 0x85EBCA77) ^ Math.imul((b + 0x94D0) >>> 0, 0xC2B2AE3D)) >>> 0;
  h = (h ^ (h >>> 15)) >>> 0; h = Math.imul(h, 0x2C1B3C6D) >>> 0; h = (h ^ (h >>> 12)) >>> 0; h = Math.imul(h, 0x297A2D39) >>> 0; h = (h ^ (h >>> 15)) >>> 0;
  return h >>> 0;
}
const idiv = (a: number, b: number) => Math.floor(a / b);
export const GEN = {
  /** caps (entry cap) on a deck (entry deck): cap × cap cells with `gap` between and around; keyboard = the bottom row's middle is one long bar */
  grid(c: CellCanvas, capW = 1, capH = 1, gap = 1, keyboard = false, deck = 1, cap = 2) {
    const Pu = capW + gap, Pv = capH + gap; const W = c.W, H = c.H;
    for (let j = 0; j < H; j++) for (let i = 0; i < W; i++) {
      const r = idiv(j, Pv); const inV = (j - r * Pv) >= gap; let inU = (i % Pu) >= gap;
      if (keyboard && r === 0) { const sL = idiv(W * 5, 16), sR = W - sL; if (i >= sL && i < sR) inU = i > sL && i < sR - 1; }
      c.set(i, j, inU && inV ? cap : deck);
    }
    return c;
  },
  /** 1-cell seams (entry seam) between `rows` × `cols` cells of the canvas (never on its edge) + a pullW × pullH pull (entry pull) in each cell's upper third */
  seams(c: CellCanvas, rows = 3, cols = 1, pullW = 3, pullH = 1, seam = 1, pull = 2) {
    const W = c.W, H = c.H;
    const jb = (k: number) => idiv(k * H + idiv(rows, 2), rows), ib = (k: number) => idiv(k * W + idiv(cols, 2), cols);
    for (let k = 1; k < rows; k++) c.fill(seam, 0, jb(k), W, jb(k) + 1);
    for (let k = 1; k < cols; k++) c.fill(seam, ib(k), 0, ib(k) + 1, H);
    if (pullW > 0 && pullH > 0) for (let r = 0; r < rows; r++) for (let q = 0; q < cols; q++) {
      const j0 = r === 0 ? 0 : jb(r) + 1, j1 = r === rows - 1 ? H : jb(r + 1); const i0 = q === 0 ? 0 : ib(q) + 1, i1 = q === cols - 1 ? W : ib(q + 1);
      const cw = i1 - i0, chh = j1 - j0; const pw = Math.min(pullW, cw); const pi0 = i0 + idiv(cw - pw, 2), pj0 = j0 + idiv(chh * 2, 3);
      c.fill(pull, pi0, pj0, pi0 + pw, Math.min(pj0 + pullH, j1));
    }
    return c;
  },
  /** ragged text bars from the top: `thick` cells tall, `gap` between, a cell of margin either side, the odd paragraph break; entry ink */
  lines(c: CellCanvas, thick = 1, gap = 1, seed = 0, ink = 1) {
    const W = c.W, H = c.H; const P = thick + gap; const nBars = idiv(H + gap, P); const inner = Math.max(W - 2, 0);
    for (let r = 0; r < nBars; r++) {
      const hsh = mkHash(seed, r, 7); if (r > 0 && r < nBars - 1 && hsh % 5 === 4) continue;
      let len = 1 + idiv(inner * (5 + (hsh >>> 4) % 6), 10); if (r === nBars - 1 && nBars > 2) len = 1 + idiv(inner * 3, 10) + (hsh >>> 8) % Math.max(idiv(W, 4), 1);
      const lim = Math.min(1 + len, W - 1) + (W <= 3 ? 1 : 0); const top = H - 1 - r * P;
      c.fill(ink, 1, top - thick + 1, lim, top + 1);
    }
    return c;
  },
  /** a signature-like zigzag two thirds up + a short underline, one cell wide (entry ink) */
  scribble(c: CellCanvas, seed = 0, ink = 1) {
    const W = c.W, H = c.H; const yb = idiv(H * 3, 5), amp = 1 + (seed % 2);
    let prev = -99;
    for (let i = 1; i < W - 1; i++) {
      const t = i % 4; const y = yb + (t === 0 ? 0 : t === 2 ? 2 : 1) * amp - amp;
      if (prev > -99 && Math.abs(y - prev) > 1) c.fill(ink, i, Math.min(y, prev) + 1, i + 1, Math.max(y, prev));   // join the stroke: no gaps between diagonal neighbours two rows apart
      c.set(i, y, ink); prev = y;
    }
    c.fill(ink, 1, idiv(H, 4), 1 + idiv((W - 2) * 3, 5), idiv(H, 4) + 1);
    return c;
  },
  /** a row of `count` 1-cell dots at `pitch` along u at row j (entry dot; every dot k with hash % amberEvery === 0 → entry alt) */
  dots(c: CellCanvas, count = 3, pitch = 2, j = idiv(c.H, 2), seed = 0, amberEvery = 0, dot = 1, alt = 2) {
    for (let k = 0; k < count; k++) c.set(k * pitch, j, amberEvery > 0 && mkHash(seed, k, 3) % amberEvery === 0 ? alt : dot);
    return c;
  },
  /** the vending PRODUCT WINDOW: `cols` cans a shelf × `rows` shelves — each cell's bottom row = the rail (transparent = the machine's own dark front) with a
   *  2-cell white tag (entry back), a 1-cell backlit gap left of each can and over the top shelf (entry back), cans in entries canA / canB (2 : 1), the odd coil sold out */
  window(c: CellCanvas, cols = 4, rows = 4, seed = 0, canA = 1, canB = 2, back = 3) {
    const W = c.W, H = c.H;
    for (let j = 0; j < H; j++) for (let i = 0; i < W; i++) {
      const col = Math.min(idiv(i * cols, W), cols - 1), r = Math.min(idiv(j * rows, H), rows - 1);
      const i0 = idiv(col * W, cols), i1 = idiv((col + 1) * W, cols), j0 = idiv(r * H, rows), j1 = idiv((r + 1) * H, rows);
      const ci = i - i0, cj = j - j0, cw = i1 - i0, chh = j1 - j0;
      let e: number;
      if (cj === 0) e = ci >= 1 && ci <= 2 && ci < cw ? back : 0;
      else if (cj === chh - 1 && chh >= 4) e = back;
      else if (ci === 0) e = back;
      else { const hsh = mkHash(seed, col, r); e = hsh % 7 === 0 ? back : (hsh >>> 3) % 3 === 2 ? canB : canA; }
      c.set(i, j, e);
    }
    return c;
  },
  /** a 5×7 glyph (GLYPHS row masks, top first) at cell (i0, j0) = its bottom-left, × scale, entry ink */
  glyph(c: CellCanvas, g: number, i0 = 0, j0 = 0, scale = 1, ink = 1) { const rows = GLYPHS[g]; if (rows) blit57(c, rows, i0, j0, scale, ink); return c; },
  /** STENCILLED LETTERS (the STENCIL face below: 5 × 7 a letter, a column between two): `text` from cell (i0, j0) = its bottom-left, × scale, entry ink. A
   *  character the face does not have is a gap a letter wide. A FIGURE at scale 2 and over is DRAWN in its box (NUMERAL: strokes, not blown-up cells: a 7 a
   *  metre tall made of 20 cm squares is a staircase). What does not fit the canvas is cut (stencilWidth says how wide a text is) */
  text(c: CellCanvas, text: string, i0 = 0, j0 = 0, scale = 1, ink = 1) {
    let i = i0;
    for (const ch of text.toUpperCase()) {
      const pen = scale >= 2 ? NUMERAL[ch] : undefined, rows = STENCIL[ch];
      if (pen) strokes(c, pen, i, j0, 5 * scale, 7 * scale, 0.5 + 0.75 * scale, ink); else if (rows) blit57(c, rows, i, j0, scale, ink);
      i += 6 * scale;
    }
    return c;
  },
  /** WARNING STRIPES at 45°: bands `w` cells wide of entries a and b in turn, leaning to the right going up. `ph` = where the pattern starts (the canvas's own
   *  lattice place, so two bitmaps side by side carry one run of stripes across the join) */
  stripes(c: CellCanvas, w = 2, a = 1, b = 2, ph = 0) {
    for (let j = 0; j < c.H; j++) for (let i = 0; i < c.W; i++) c.set(i, j, (((i - j + ph) % (2 * w)) + 2 * w) % (2 * w) < w ? a : b);
    return c;
  },
  /** a painted OUTLINE `w` cells wide round the canvas (a hatch's edge, a bay on a floor), entry ink; `gap` > 0 leaves that many cells out of the middle of
   *  each of its four sides, so only the corners are drawn. Only the outline's own cells are written: what the canvas holds inside it stays */
  frame(c: CellCanvas, w = 1, ink = 1, gap = 0) {
    const W = c.W, H = c.H, gu = Math.min(gap, Math.max(W - 2 * w, 0)), gv = Math.min(gap, Math.max(H - 2 * w, 0)), u0 = idiv(W - gu, 2), v0 = idiv(H - gv, 2);
    for (let j = 0; j < H; j++) for (let i = 0; i < W; i++) {
      const onU = j < w || j >= H - w, onV = i < w || i >= W - w;
      if ((onU && !(i >= u0 && i < u0 + gu)) || (onV && !(j >= v0 && j < v0 + gv))) c.set(i, j, ink);
    }
    return c;
  },
};
/** a 5 × 7 letter's rows (top first, bit 4 = its left column) onto the canvas at (i0, j0) = its bottom-left, × scale */
function blit57(c: CellCanvas, rows: readonly number[], i0: number, j0: number, scale: number, ink: number) {
  for (let gr = 0; gr < 7; gr++) for (let gc = 0; gc < 5; gc++) if ((rows[gr] >> (4 - gc)) & 1) c.fill(ink, i0 + gc * scale, j0 + (6 - gr) * scale, i0 + (gc + 1) * scale, j0 + (7 - gr) * scale);
}
/** strokes drawn in a w × h cell box at (i0, j0): each a run of points in the box's own 0‥1 (y up), a cell painted where its middle is within half a pen of one */
function strokes(c: CellCanvas, runs: readonly (readonly (readonly [number, number])[])[], i0: number, j0: number, w: number, h: number, pen: number, ink: number) {
  const r = pen / 2;
  for (let j = 0; j < h; j++) for (let i = 0; i < w; i++) {
    const px = i + 0.5, py = j + 0.5; let hit = false;
    for (const run of runs) { for (let k = 0; k + 1 < run.length && !hit; k++) {
      const ax = r + run[k][0] * (w - pen), ay = r + run[k][1] * (h - pen), bx = r + run[k + 1][0] * (w - pen), by = r + run[k + 1][1] * (h - pen);   // (the pen's middle stays half a pen inside the box, so a stroke along its edge is whole)
      const dx = bx - ax, dy = by - ay, t = Math.min(1, Math.max(0, ((px - ax) * dx + (py - ay) * dy) / Math.max(dx * dx + dy * dy, 1e-9)));
      hit = Math.hypot(px - ax - t * dx, py - ay - t * dy) <= r;
    } if (hit) break; }
    if (hit) c.set(i0 + i, j0 + j, ink);
  }
}
const arc = (cx: number, cy: number, rx: number, ry: number, a0: number, a1: number, n = 12): [number, number][] => Array.from({ length: n + 1 }, (_, k) => { const a = (a0 + (a1 - a0) * k / n) * Math.PI / 180; return [cx + rx * Math.cos(a), cy + ry * Math.sin(a)] as [number, number]; });
/** THE FIGURES AS A SIGN PAINTER DRAWS THEM: each a few strokes in a box of 0‥1 (y up), drawn at whatever size the stencil is cut (GEN.text, scale 2 and over) */
const numeral: Record<string, readonly (readonly (readonly [number, number])[])[]> = {};
export const NUMERAL: Readonly<Record<string, readonly (readonly (readonly [number, number])[])[]>> = numeral;
Object.assign(numeral, {
  0: [arc(0.5, 0.5, 0.5, 0.5, 0, 360, 20)],
  1: [[[0.12, 0.72], [0.5, 1], [0.5, 0]], [[0.12, 0], [0.88, 0]]],
  2: [[...arc(0.5, 0.7, 0.5, 0.3, 175, -35, 10), [0, 0], [1, 0]]],
  3: [arc(0.48, 0.74, 0.46, 0.26, 150, -90, 10), arc(0.48, 0.26, 0.52, 0.26, 90, -150, 10)],
  4: [[[0.74, 0], [0.74, 1], [0, 0.32], [1, 0.32]]],
  5: [[[0.92, 1], [0.12, 1], [0.06, 0.56], ...arc(0.46, 0.32, 0.54, 0.32, 105, -150, 10)]],
  6: [[...arc(0.56, 0.5, 0.56, 0.5, 55, 195, 10), ...arc(0.5, 0.3, 0.5, 0.3, 180, -180, 16)]],
  7: [[[0, 1], [1, 1], [0.36, 0]]],
  8: [arc(0.5, 0.75, 0.42, 0.25, 0, 360, 16), arc(0.5, 0.26, 0.5, 0.26, 0, 360, 16)],
  9: [[...arc(0.5, 0.7, 0.5, 0.3, 0, 360, 16), ...arc(0.44, 0.5, 0.56, 0.5, 15, -125, 10)]],
});
/** cells a stencilled text is wide at a scale (its last letter's spare column left off) */
export const stencilWidth = (text: string, scale = 1) => Math.max(text.length * 6 - 1, 0) * scale;
const face5 = (rows: string): readonly number[] => rows.split(' ').map(r => parseInt(r.replace(/#/g, '1').replace(/\./g, '0'), 2));
/** THE STENCIL FACE: capitals, figures, a few signs and four arrows, each 5 × 7 cells drawn by hand (rows top first, '#' = paint). What a painter sprays on a
 *  wall through a cut sheet: a bay's number, a level, the way to a stair. Plain block shapes with no bridges: at one cell a stroke a bridge is a hole, and the
 *  look's old-paint read (markings.wgsl markSoft) is what breaks them up */
export const STENCIL: Readonly<Record<string, readonly number[]>> = {
  A: face5('.###. #...# #...# ##### #...# #...# #...#'), B: face5('####. #...# #...# ####. #...# #...# ####.'), C: face5('.#### #.... #.... #.... #.... #.... .####'),
  D: face5('####. #...# #...# #...# #...# #...# ####.'), E: face5('##### #.... #.... ####. #.... #.... #####'), F: face5('##### #.... #.... ####. #.... #.... #....'),
  G: face5('.#### #.... #.... #.### #...# #...# .####'), H: face5('#...# #...# #...# ##### #...# #...# #...#'), I: face5('.###. ..#.. ..#.. ..#.. ..#.. ..#.. .###.'),
  J: face5('..### ...#. ...#. ...#. ...#. #..#. .##..'), K: face5('#...# #..#. #.#.. ##... #.#.. #..#. #...#'), L: face5('#.... #.... #.... #.... #.... #.... #####'),
  M: face5('#...# ##.## #.#.# #.#.# #...# #...# #...#'), N: face5('#...# ##..# #.#.# #.#.# #..## #...# #...#'), O: face5('.###. #...# #...# #...# #...# #...# .###.'),
  P: face5('####. #...# #...# ####. #.... #.... #....'), Q: face5('.###. #...# #...# #...# #.#.# #..#. .##.#'), R: face5('####. #...# #...# ####. #.#.. #..#. #...#'),
  S: face5('.#### #.... #.... .###. ....# ....# ####.'), T: face5('##### ..#.. ..#.. ..#.. ..#.. ..#.. ..#..'), U: face5('#...# #...# #...# #...# #...# #...# .###.'),
  V: face5('#...# #...# #...# #...# #...# .#.#. ..#..'), W: face5('#...# #...# #...# #.#.# #.#.# ##.## #...#'), X: face5('#...# #...# .#.#. ..#.. .#.#. #...# #...#'),
  Y: face5('#...# #...# .#.#. ..#.. ..#.. ..#.. ..#..'), Z: face5('##### ....# ...#. ..#.. .#... #.... #####'),
  0: face5('.###. #...# #...# #...# #...# #...# .###.'), 1: face5('..#.. .##.. ..#.. ..#.. ..#.. ..#.. .###.'), 2: face5('.###. #...# ....# ...#. ..#.. .#... #####'),
  3: face5('####. ....# ....# .###. ....# ....# ####.'), 4: face5('...#. ..##. .#.#. #..#. ##### ...#. ...#.'), 5: face5('##### #.... ####. ....# ....# #...# .###.'),
  6: face5('.###. #.... #.... ####. #...# #...# .###.'), 7: face5('##### ....# ...#. ..#.. ..#.. ..#.. ..#..'), 8: face5('.###. #...# #...# .###. #...# #...# .###.'),
  9: face5('.###. #...# #...# .#### ....# ....# .###.'),
  '-': face5('..... ..... ..... ##### ..... ..... .....'), '/': face5('....# ....# ...#. ..#.. .#... #.... #....'), '.': face5('..... ..... ..... ..... ..... .##.. .##..'),
  '→': face5('..... ..#.. ...#. ##### ...#. ..#.. .....'), '←': face5('..... ..#.. .#... ##### .#... ..#.. .....'),
  '↑': face5('..#.. .###. #.#.# ..#.. ..#.. ..#.. ..#..'), '↓': face5('..#.. ..#.. ..#.. ..#.. #.#.# .###. ..#..'),
};
/** 5×7 pictograms, row 0 = TOP, bit 4 = leftmost column */
export const GLYPHS: readonly (readonly number[])[] = [
  [0b00100, 0b01110, 0b10101, 0b00100, 0b00100, 0b00100, 0b11111],   // arrow up over a base line: this side up
  [0b10001, 0b10001, 0b11111, 0b01110, 0b00100, 0b00100, 0b01110],   // stemmed glass: fragile
  [0b00100, 0b01110, 0b11111, 0b00100, 0b00100, 0b10100, 0b01100],   // umbrella: keep dry
  [0b00000, 0b00100, 0b00100, 0b11111, 0b00100, 0b00100, 0b00000],   // cross (first aid)
  [0b11111, 0b10001, 0b10001, 0b10001, 0b10001, 0b10001, 0b11111],   // hollow square (a maker's mark / the company logo's frame)
];
/** Build a marking from a rect in METRES on the face (snapped INWARD onto the marking's lattice → at / size) and a painter that fills the canvas; null if the snapped rect is empty. */
export function markFromRect(face: MarkFace, res: MarkRes, rect: [number, number, number, number], pal: MarkPal[], paint: (c: CellCanvas) => void, base = MARKING_TEXEL): MarkingDef | null {
  const t = base * RES_K[res]; const eps = 1e-7;
  const U0 = Math.ceil(rect[0] / t - eps), V0 = Math.ceil(rect[1] / t - eps), U1 = Math.floor(rect[2] / t + eps), V1 = Math.floor(rect[3] / t + eps);
  const W = Math.min(U1 - U0, MAX_CELLS_W), H = Math.min(V1 - V0, MAX_CELLS_H); if (W < 1 || H < 1) return null;
  const c = new CellCanvas(W, H); paint(c);
  return marking({ face, res, at: [U0, V0], size: [W, H], pal, cells: c.toRows() });
}
/** a marking from ASCII-art rows (top first) placed with its bottom-left cell at lattice (U0, V0) */
export function markFromArt(face: MarkFace, res: MarkRes, U0: number, V0: number, rows: string[], pal: MarkPal[]): MarkingDef {
  const W = rows[0]?.length ?? 0; if (!W || rows.some(r => r.length !== W)) throw new Error('markFromArt: rows must be equal length');
  return marking({ face, res, at: [U0, V0], size: [W, rows.length], pal, cells: rows.slice() });
}

// ------------------------------------------------------------------------------------------------------------------------------------ stats + packing
export interface MarkStats {
  texel: number; W: number; H: number;
  /** coverage fraction of palette entries 1‥3 over the bitmap */
  f: [number, number, number];
  /** the entries' resolved albedos (absolute; `mul` × the base) */
  alb: [Vec3, Vec3, Vec3];
  /** mean radiance over the bitmap at gain 1 (rgb) and the bitmap's area (m²) */
  meanL: Vec3; area: number;
  /** mean albedo change over the bitmap: Σ_k f_k (albedo_k − base) */
  dAlbedo: Vec3;
  /** bit k−1 set = entry k emits */
  emask: number;
}
export function resolveAlbedo(p: MarkPal | undefined, base: Vec3): Vec3 {
  if (!p) return [base[0], base[1], base[2]];
  if (p.albedo) return [p.albedo[0], p.albedo[1], p.albedo[2]];
  const k = p.mul ?? 1; return [Math.min(base[0] * k, 1), Math.min(base[1] * k, 1), Math.min(base[2] * k, 1)];
}
export function markStats(m: MarkingDef, base: Vec3, texelBase = MARKING_TEXEL): MarkStats {
  const t = texelOf(m, texelBase); const [W, H] = m.size;
  const cnt = [0, 0, 0, 0];
  for (const r of m.cells) for (let i = 0; i < r.length; i++) cnt[CELL_CHARS.indexOf(r[i])]++;
  const n = W * H; const f: [number, number, number] = [cnt[1] / n, cnt[2] / n, cnt[3] / n];
  const alb: [Vec3, Vec3, Vec3] = [resolveAlbedo(m.pal[0], base), resolveAlbedo(m.pal[1], base), resolveAlbedo(m.pal[2], base)];
  const meanL: Vec3 = [0, 0, 0]; const dAlbedo: Vec3 = [0, 0, 0]; let emask = 0;
  for (let k = 0; k < 3; k++) {
    const p = m.pal[k]; if (!p) continue;
    if (p.emit) { emask |= 1 << k; for (let ch = 0; ch < 3; ch++) meanL[ch] += f[k] * p.emit[ch]; }
    if (p.albedo || (p.mul !== undefined && p.mul !== 1)) for (let ch = 0; ch < 3; ch++) dAlbedo[ch] += f[k] * (alb[k][ch] - base[ch]);
  }
  return { texel: t, W, H, f, alb, meanL, area: (W * t) * (H * t), dAlbedo, emask };
}
/** rows the packed record takes */
export const markRecordRows = (m: MarkingDef) => MARK_HEAD_ROWS + Math.ceil(m.size[0] * m.size[1] / CELLS_PER_ROW);
