// THE WORLD'S BLOCK: what a ray meets of leaves. Sparse cells of 0.32 and 0.64 m (wider ones held two sides of a wall, and leaked); a cell holds how thick its leaves stand each way (a form in the way:
// they hang in sheets), a tight box round them, and their color from six sides. Found once, by rays through the cells' own contents (blockmap.ts). Solids are not in it: what rays meet of those is the things
// themselves (small ones in cells were tried: no faster, and 0.1 to 0.5 points further from the truth).
import { type V3, add, apply, mul, cross } from '../math';
import { type World, type Piece, boundsOf, F } from './world';

export const FINEST = 0.32, LEVELS = 2, BRICK = FINEST * 8, BRICK_WORDS = 588, OFFSET = [0, 512, 576, 584], CELL_ROWS = 2, LIGHT_ROWS = 3, /** a brick of smoke each */ SMOKE_ROWS = 3072;
/** does it part nothing from anything? (A wire, a twig, a cup: a cell of leaves goes on behind it, and does not behind a wall) */
export function thin(p: Piece): boolean {
  if (p.flags & (F.GLASS | F.WATER | F.GLOW | F.NOSHADOW | F.FAR | F.BEND | F.MESH) || (p.lamp ?? -1) >= 0 || p.stays) return false;
  const e = [...p.half].sort((a, b) => a - b), size = e[1]! * 2; return size < FINEST * 2 && !(size >= FINEST / 2 && e[2]! * 2 >= FINEST * 2);
}
export interface Blocks { lo: V3; n: V3; /** a brick's number + 1, or 0 */ page: Uint32Array; bricks: Uint32Array; /** each cell's corner and level */ cells: Float32Array; /** where each is told of in bricks */ said: Uint32Array; many: number }

/** which cells have anything in them */
export function layBlocks(w: World): Blocks {
  const at = new Map<number, { b: V3; has: Uint8Array }>(), OFF = 1 << 15;
  const mark = (x: number, y: number, z: number, level: number) => {
    const s = level ? 2 : 3, bx = x >> s, by = y >> s, bz = z >> s, key = ((bx + OFF) * 65536 + (by + OFF)) * 65536 + (bz + OFF); let e = at.get(key); if (!e) at.set(key, e = { b: [bx, by, bz], has: new Uint8Array(BRICK_WORDS) });
    e.has[OFFSET[level]! + (x - (bx << s)) + (1 << s) * ((y - (by << s)) + (1 << s) * (z - (bz << s)))] = 1;
  };
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
/** what torches light, after them: five torches, 256 places each, two rows a place (stages/spots.wgsl); and after them what it was a frame ago */
export const SPOT_TORCHES = 5, SPOT_ROWS = SPOT_TORCHES * 256 * 2 * 2, spotRow = (b: Blocks, row: number) => row + blockRows(b);
/** where they lie in extra, for the shaders */
export function blockCode(b: Blocks, row: number): string {
  const page = row, bricks = page + b.page.length / 4, cells = bricks + b.bricks.length / 4, f = (v: number) => v.toFixed(4);
  return `const BLK_LO = vec3f(${b.lo.map(f).join(', ')}); const BLK_N = vec3i(${b.n.join(', ')}); const BLK_PAGE = ${page}u; const BLK_BRICKS = ${bricks}u; const BLK_CELLS = ${cells}u; const BLK_MANY = ${b.many}u; const BLK_LIGHT = ${lightRow(b, row)}u; const BLK_SMOKE = ${smokeRow(b, row)}u; const SPOT_ROW = ${spotRow(b, row)}u; const SPOT_WAS = ${SPOT_ROWS / 2}u; const SPOT_TORCHES = ${SPOT_TORCHES}u;\n`;
}
export const cellRow = (b: Blocks, row: number) => row + b.page.length / 4 + b.bricks.length / 4;
export const lightRow = (b: Blocks, row: number) => cellRow(b, row) + Math.max(1, b.many) * CELL_ROWS;
export const smokeRow = (b: Blocks, row: number) => lightRow(b, row) + b.many * LIGHT_ROWS;
