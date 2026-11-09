// THE WORLD'S BLOCK: what a ray meets of things smaller than it is wide. Sparse cells of 0.32 and 0.64 m (wider ones held two sides of a wall, and leaked); a cell holds how thick its contents stand along x, y and z, a tight box
// round them, and their color from six sides. Level h holds what is smaller than h (and every leaf). Found once, by rays through the cells' own contents (measure.ts).
import { type V3, add, apply, mul } from '../math';
import { type World, type Piece, boundsOf, F } from '../scene/world';
import { PEOPLE, LIMBS, CARRY_ROWS } from './carry';

export const FINEST = 0.32, LEVELS = 2, BRICK = FINEST * 8, BRICK_WORDS = 588, OFFSET = [0, 512, 576, 584], CELL_ROWS = 2, LIGHT_ROWS = 3, /** a brick of smoke each */ SMOKE_ROWS = 3072;
/** how big a thing is to a ray: its middle extent (a wire is thin, a wall is not). 0, 1: gone once a ray is 0.32, 0.64 m wide; 3: always there, for it may part one room from another */
export function classOf(p: Piece): number {
  if (p.cls !== undefined) return p.cls;
  if (p.flags & (F.GLASS | F.WATER | F.GLOW | F.NOSHADOW | F.FAR | F.MOVES | F.BEND | F.MESH) || (p.lamp ?? -1) >= 0) return 3;
  const size = [...p.half].sort((a, b) => a - b)[1]! * 2;
  return size < FINEST ? 0 : size < FINEST * 2 ? 1 : 3;
}
export interface Blocks { lo: V3; n: V3; /** a brick's number + 1, or 0 */ page: Uint32Array; bricks: Uint32Array; /** each cell's corner and level */ cells: Float32Array; /** where each is told of in bricks */ said: Uint32Array; many: number; /** the most that may keep light */ lit: number }

/** which cells have anything in them */
export function layBlocks(w: World): Blocks {
  const at = new Map<string, { b: V3; has: Set<number> }>();
  const touch = (lo: V3, hi: V3, level: number) => {
    const h = FINEST * 2 ** level, per = 8 >> level, a = lo.map(v => Math.floor(v / h)), b = hi.map(v => Math.floor(v / h));
    for (let z = a[2]!; z <= b[2]!; z++) for (let y = a[1]!; y <= b[1]!; y++) for (let x = a[0]!; x <= b[0]!; x++) {
      const c = [Math.floor(x / per), Math.floor(y / per), Math.floor(z / per)] as V3, key = c.join(); let e = at.get(key); if (!e) at.set(key, e = { b: c, has: new Set() });
      e.has.add(OFFSET[level]! + (x - c[0] * per) + per * ((y - c[1] * per) + per * (z - c[2] * per)));
    }
  };
  for (const p of w.pieces) { const c = classOf(p); if (c === 3) continue; const b = boundsOf(p); for (let l = c; l < LEVELS; l++) touch(b.mn, b.mx, l); }
  for (const i of w.instances) for (const l of w.clusters[i.cluster]!.leaves) {
    const p = add(i.at, apply(i.rot, mul(l.at, i.scale))), t = mul(apply(i.rot, l.t), l.size * i.scale * 0.8);
    for (let k = 0; k < LEVELS; k++) { touch(p, p, k); const a = add(p, t), b = add(p, mul(t, -1)); touch(a, a, k); touch(b, b, k); }
  }
  if (!at.size) return { lo: [0, -90, 0], n: [1, 1, 1], page: new Uint32Array(4), bricks: new Uint32Array(BRICK_WORDS), cells: new Float32Array(4), said: new Uint32Array(1), many: 0, lit: 0 };
  const lo: V3 = [1e9, 1e9, 1e9], hi: V3 = [-1e9, -1e9, -1e9]; for (const { b } of at.values()) for (let k = 0; k < 3; k++) { lo[k] = Math.min(lo[k]!, b[k]!); hi[k] = Math.max(hi[k]!, b[k]! + 1); }
  const n = [0, 1, 2].map(k => hi[k]! - lo[k]!) as V3, page = new Uint32Array(Math.ceil(n[0] * n[1] * n[2] / 4) * 4), bricks = new Uint32Array(at.size * BRICK_WORDS), cells: number[] = [], said: number[] = []; let brick = 0, lit = 0;
  for (const { b, has } of at.values()) {
    page[b[0] - lo[0] + n[0] * (b[1] - lo[1] + n[1] * (b[2] - lo[2]))] = brick + 1;
    for (const s of has) {
      const level = s >= OFFSET[3]! ? 3 : s >= OFFSET[2]! ? 2 : s >= OFFSET[1]! ? 1 : 0, per = 8 >> level, i = s - OFFSET[level]!, h = FINEST * 2 ** level;
      cells.push(b[0] * BRICK + (i % per) * h, b[1] * BRICK + (Math.floor(i / per) % per) * h, b[2] * BRICK + Math.floor(i / per / per) * h, level); bricks[brick * BRICK_WORDS + s] = cells.length / 4; said.push(brick * BRICK_WORDS + s); lit++;
    }
    brick++;
  }
  return { lo: mul(lo, BRICK), n, page, bricks, cells: Float32Array.from(cells), said: Uint32Array.from(said), many: cells.length / 4, lit };
}
export const blockRows = (b: Blocks) => b.page.length / 4 + b.bricks.length / 4 + Math.max(1, b.many) * CELL_ROWS + (b.lit + PEOPLE * LIMBS) * LIGHT_ROWS + SMOKE_ROWS + CARRY_ROWS;
/** where they lie in extra, for the shaders */
export function blockCode(b: Blocks, row: number): string {
  const page = row, bricks = page + b.page.length / 4, cells = bricks + b.bricks.length / 4, f = (v: number) => v.toFixed(4);
  return `const BLK_LO = vec3f(${b.lo.map(f).join(', ')}); const BLK_N = vec3i(${b.n.join(', ')}); const BLK_PAGE = ${page}u; const BLK_BRICKS = ${bricks}u; const BLK_CELLS = ${cells}u; const BLK_MANY = ${b.many}u; const BLK_LIGHT = ${lightRow(b, row)}u; const BLK_SMOKE = ${smokeRow(b, row)}u; const BLK_CARRY = ${carryRow(b, row)}u;\n`;
}
export const cellRow = (b: Blocks, row: number) => row + b.page.length / 4 + b.bricks.length / 4;
export const lightRow = (b: Blocks, row: number) => cellRow(b, row) + Math.max(1, b.many) * CELL_ROWS;
export const smokeRow = (b: Blocks, row: number) => lightRow(b, row) + (b.lit + PEOPLE * LIMBS) * LIGHT_ROWS;
export const carryRow = (b: Blocks, row: number) => smokeRow(b, row) + SMOKE_ROWS;
