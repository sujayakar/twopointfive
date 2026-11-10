// THE WORLD'S BLOCK: what a ray meets of things smaller than it is wide. Sparse cells of 0.32 and 0.64 m (wider ones held two sides of a wall, and leaked); a cell holds how thick its contents stand along x, y and z, a tight box
// round them, and their color from six sides. Level h holds what is smaller than h (and every leaf). Found once, by rays through the cells' own contents (blockmap.ts).
import { type V3, add, apply, mul, cross } from '../math';
import { type World, type Piece, boundsOf, F } from './world';

export const FINEST = 0.32, LEVELS = 2, BRICK = FINEST * 8, BRICK_WORDS = 588, OFFSET = [0, 512, 576, 584], CELL_ROWS = 2, LIGHT_ROWS = 3, /** a brick of smoke each */ SMOKE_ROWS = 3072;
/** how big a thing is to a ray: its middle extent (a wire is thin, a wall is not). 0, 1: gone once a ray is 0.32, 0.64 m wide; 3: always there, for it may part one room from another. (What moves leaves nothing behind: a person seen by a wide ray is too little to tell, and soft boxes in their place gained nothing) */
/** for a study: do solids go into cells too, or leaves alone? */
export const STUDY = { solids: true };
export function classOf(p: Piece): number {
  if (!STUDY.solids || p.flags & (F.GLASS | F.WATER | F.GLOW | F.NOSHADOW | F.FAR | F.BEND | F.MESH) || (p.lamp ?? -1) >= 0 || p.stays) return 3;
  const e = [...p.half].sort((a, b) => a - b), size = e[1]! * 2; if (size >= FINEST / 2 && e[2]! * 2 >= FINEST * 2) return 3;       /* (a strip of wall over a door, a shelf's board: a sheet, however narrow) */
  return size < FINEST ? 0 : size < FINEST * 2 ? 1 : 3;
}
/** a limb is bent pieces, a wall is slabs: what is thick enough and joined to its like is as long as the whole run of them, and stays when that is long. (A thin thing may go however long it is) */
export function joinRuns(ps: Piece[]): number {
  const mid = (p: Piece) => [...p.half].sort((a, b) => a - b)[1]! * 2, could = ps.filter(p => !(p.flags & (F.GLASS | F.WATER | F.GLOW | F.NOSHADOW | F.FAR | F.BEND | F.MESH)) && (p.lamp ?? -1) < 0 && mid(p) >= FINEST / 2), box = could.map(boundsOf), up = could.map((_, i) => i), top = (i: number): number => up[i] === i ? i : (up[i] = top(up[i]!));
  const at = new Map<string, number[]>(), G = 1.28; could.forEach((p, i) => { const b = box[i]!; for (let x = Math.floor(b.mn[0] / G); x <= Math.floor(b.mx[0] / G); x++) for (let y = Math.floor(b.mn[1] / G); y <= Math.floor(b.mx[1] / G); y++) for (let z = Math.floor(b.mn[2] / G); z <= Math.floor(b.mx[2] / G); z++) { const k = `${p.mat} ${x} ${y} ${z}`, l = at.get(k); if (!l) { at.set(k, [i]); continue; } for (const j of l) { const q = box[j]!; if (top(i) !== top(j) && [0, 1, 2].every(a => b.mn[a]! <= q.mx[a]! + 0.005 && q.mn[a]! <= b.mx[a]! + 0.005)) up[top(i)] = top(j); } l.push(i); } });
  const whole = new Map<number, { mn: number[]; mx: number[] }>(); could.forEach((_, i) => { const t = top(i), b = box[i]!, w = whole.get(t) ?? { mn: [9e9, 9e9, 9e9], mx: [-9e9, -9e9, -9e9] }; for (let a = 0; a < 3; a++) { w.mn[a] = Math.min(w.mn[a]!, b.mn[a]!); w.mx[a] = Math.max(w.mx[a]!, b.mx[a]!); } whole.set(t, w); });
  let n = 0; could.forEach((p, i) => { const w = whole.get(top(i))!; if (classOf(p) !== 3 && Math.max(w.mx[0]! - w.mn[0]!, w.mx[1]! - w.mn[1]!, w.mx[2]! - w.mn[2]!) >= FINEST * 2) { p.stays = true; n++; } }); return n;
}
export interface Blocks { lo: V3; n: V3; /** a brick's number + 1, or 0 */ page: Uint32Array; bricks: Uint32Array; /** each cell's corner and level */ cells: Float32Array; /** where each is told of in bricks */ said: Uint32Array; many: number }

/** which cells have anything in them */
export function layBlocks(w: World): Blocks {
  const at = new Map<number, { b: V3; has: Uint8Array }>(), OFF = 1 << 15;
  const mark = (x: number, y: number, z: number, level: number) => {
    const s = level ? 2 : 3, bx = x >> s, by = y >> s, bz = z >> s, key = ((bx + OFF) * 65536 + (by + OFF)) * 65536 + (bz + OFF); let e = at.get(key); if (!e) at.set(key, e = { b: [bx, by, bz], has: new Uint8Array(BRICK_WORDS) });
    e.has[OFFSET[level]! + (x - (bx << s)) + (1 << s) * ((y - (by << s)) + (1 << s) * (z - (bz << s)))] = 1;
  };
  const touch = (lo: V3, hi: V3, level: number) => { const k = 1 / (FINEST * 2 ** level), a = lo.map(v => Math.floor(v * k)), b = hi.map(v => Math.floor(v * k)); for (let z = a[2]!; z <= b[2]!; z++) for (let y = a[1]!; y <= b[1]!; y++) for (let x = a[0]!; x <= b[0]!; x++) mark(x, y, z, level); };
  for (const p of w.pieces) { const c = classOf(p); if (c === 3) continue; const b = boundsOf(p); for (let l = c; l < LEVELS; l++) touch(b.mn, b.mx, l); }
  for (const i of w.instances) for (const l of w.clusters[i.cluster]!.leaves) {
    const p = add(i.at, apply(i.rot, mul(l.at, i.scale))), t = mul(apply(i.rot, l.t), l.size * i.scale * 0.8), s = mul(apply(i.rot, cross(l.n, l.t)), l.size * i.scale * 0.4);
    for (const [x, y, z] of [p, add(p, t), add(p, mul(t, -1)), add(p, s), add(p, mul(s, -1))]) for (let k = 0; k < LEVELS; k++) { const q = 1 / (FINEST * 2 ** k); mark(Math.floor(x * q), Math.floor(y * q), Math.floor(z * q), k); }
  }
  if (!at.size) return { lo: [0, -90, 0], n: [1, 1, 1], page: new Uint32Array(4), bricks: new Uint32Array(BRICK_WORDS), cells: new Float32Array(4), said: new Uint32Array(1), many: 0 };
  const lo: V3 = [1e9, 1e9, 1e9], hi: V3 = [-1e9, -1e9, -1e9]; for (const { b } of at.values()) for (let k = 0; k < 3; k++) { lo[k] = Math.min(lo[k]!, b[k]!); hi[k] = Math.max(hi[k]!, b[k]! + 1); }
  const n = [0, 1, 2].map(k => hi[k]! - lo[k]!) as V3, page = new Uint32Array(Math.ceil(n[0] * n[1] * n[2] / 4) * 4), bricks = new Uint32Array(at.size * BRICK_WORDS), cells: number[] = [], said: number[] = []; let brick = 0;
  for (const { b, has } of at.values()) {
    page[b[0] - lo[0] + n[0] * (b[1] - lo[1] + n[1] * (b[2] - lo[2]))] = brick + 1;
    for (let s = 0; s < BRICK_WORDS; s++) if (has[s]) {
      const level = s >= OFFSET[1]! ? 1 : 0, per = 8 >> level, i = s - OFFSET[level]!, h = FINEST * 2 ** level;
      cells.push(b[0] * BRICK + (i % per) * h, b[1] * BRICK + (Math.floor(i / per) % per) * h, b[2] * BRICK + Math.floor(i / per / per) * h, level); bricks[brick * BRICK_WORDS + s] = cells.length / 4; said.push(brick * BRICK_WORDS + s);
    }
    brick++;
  }
  return { lo: mul(lo, BRICK), n, page, bricks, cells: Float32Array.from(cells), said: Uint32Array.from(said), many: cells.length / 4 };
}
export const blockRows = (b: Blocks) => b.page.length / 4 + b.bricks.length / 4 + Math.max(1, b.many) * CELL_ROWS + b.many * LIGHT_ROWS + SMOKE_ROWS;
/** what torches light, after them: five torches, 256 places each, two rows a place (stages/spots.wgsl) */
export const SPOT_TORCHES = 5, SPOT_ROWS = SPOT_TORCHES * 256 * 2, spotRow = (b: Blocks, row: number) => row + blockRows(b);
/** where they lie in extra, for the shaders */
export function blockCode(b: Blocks, row: number): string {
  const page = row, bricks = page + b.page.length / 4, cells = bricks + b.bricks.length / 4, f = (v: number) => v.toFixed(4);
  return `const BLK_LO = vec3f(${b.lo.map(f).join(', ')}); const BLK_N = vec3i(${b.n.join(', ')}); const BLK_PAGE = ${page}u; const BLK_BRICKS = ${bricks}u; const BLK_CELLS = ${cells}u; const BLK_MANY = ${b.many}u; const BLK_LIGHT = ${lightRow(b, row)}u; const BLK_SMOKE = ${smokeRow(b, row)}u; const SPOT_ROW = ${spotRow(b, row)}u;\n`;
}
export const cellRow = (b: Blocks, row: number) => row + b.page.length / 4 + b.bricks.length / 4;
export const lightRow = (b: Blocks, row: number) => cellRow(b, row) + Math.max(1, b.many) * CELL_ROWS;
export const smokeRow = (b: Blocks, row: number) => lightRow(b, row) + b.many * LIGHT_ROWS;
