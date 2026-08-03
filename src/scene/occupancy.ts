import { Box, boxBounds } from "./scene";

/**
 * Occupancy field for the fluid solver: which cells of a world-space grid
 * are solid (walls, columns, crates, partitions) so smoke flows around them.
 *
 * Baked once from the same static boxes the tracer draws, through the CPU
 * BVH query the throwable physics already carries — one geometry, one
 * source of truth, and no second acceleration structure.
 *
 * A cell is solid when enough of a 3x3x3 sub-lattice of it lies inside kept
 * boxes. Two classes of box are skipped, both for reasons of resolution:
 *
 *   - shorter than a cell (desk tops, carpets, light housings): they cannot
 *     deflect a plume coarser than they are, and keeping them would only
 *     stipple the field with false one-cell obstacles;
 *   - caps of the lattice — a box whose overlap with the grid is confined to
 *     the topmost row and which continues above the grid. The room's ceiling
 *     slab is the one such box: it spans y 3.20-3.50 against a grid top at
 *     3.25, so its only overlap is the top 5 cm of row 12. Baking it would
 *     make that entire row solid (the row's top sub-lattice plane sits at
 *     3.208, contributing 9 hits against SOLID_MIN_HITS = 6), deleting 25 cm
 *     of real air exactly where ceiling-hugging smoke lives. The grid's own
 *     top face is already a wall, so the slab is redundant there.
 *
 * Note the cap rule is geometric on purpose. `FLAG_NO_CAMERA` would select the
 * ceiling too, and used to, but that flag means "invisible to camera rays",
 * not "not an obstacle" — see the matching note on `Physics.raycast`. An
 * invisible-but-solid box inside the room must reach the fluid.
 */
export interface OccupancyGrid {
  dims: [number, number, number];
  /** 1 = solid, x-fastest layout: (z * ny + y) * nx + x. */
  data: Uint8Array;
  solidCells: number;
}

/** AABB overlap query into the static BVH: pushes original box indices. */
export type BoxQuery = (
  minx: number, miny: number, minz: number,
  maxx: number, maxy: number, maxz: number,
  out: number[],
) => number[];

/** Sub-lattice hits (of 27) that make a cell solid. A thin partition through
 *  a cell catches a full plane of 9, so 6 keeps thin walls and drops grazes. */
const SOLID_MIN_HITS = 6;

/** Point-in-oriented-box: three dot products into the box frame. */
function inside(b: Box, px: number, py: number, pz: number): boolean {
  const dx = px - b.center.x, dy = py - b.center.y, dz = pz - b.center.z;
  const r0 = b.rot[0], r1 = b.rot[1], r2 = b.rot[2];
  const lx = dx * r0.x + dy * r0.y + dz * r0.z;
  if (Math.abs(lx) > b.half.x) return false;
  const ly = dx * r1.x + dy * r1.y + dz * r1.z;
  if (Math.abs(ly) > b.half.y) return false;
  const lz = dx * r2.x + dy * r2.y + dz * r2.z;
  return Math.abs(lz) <= b.half.z;
}

/**
 * Is this box axis-aligned in world space? Then its world AABB IS the box,
 * containment is three interval tests, and the 27-bit mask separates into the
 * outer product of three 3-bit per-axis masks.
 *
 * Worth the special case because the expensive boxes are exactly these: the
 * floor and the walls span most of the lattice, so they dominate the scatter's
 * cost, and they are never rotated. A rotated box is small (a chair, a crate)
 * and its cell range is small with it.
 */
function axisAligned(b: Box): boolean {
  const r = b.rot;
  for (let i = 0; i < 3; i++) {
    const v = i === 0 ? r[0] : i === 1 ? r[1] : r[2];
    const ax = Math.abs(v.x), ay = Math.abs(v.y), az = Math.abs(v.z);
    const on = (ax > 0.999 ? 1 : 0) + (ay > 0.999 ? 1 : 0) + (az > 0.999 ? 1 : 0);
    if (on !== 1) return false;
  }
  return true;
}

/** 3-bit mask of which sub-samples along one axis fall inside [lo, hi]. */
function axisMask(c0: number, cw: number, lo: number, hi: number): number {
  let m = 0;
  const a = c0 + cw / 6;
  if (a >= lo && a <= hi) { m |= 1; }
  const b = c0 + cw / 2;
  if (b >= lo && b <= hi) { m |= 2; }
  const c = c0 + (5 * cw) / 6;
  if (c >= lo && c <= hi) { m |= 4; }
  return m;
}

/** Population count of a 27-bit sub-sample mask. */
function popcount27(v: number): number {
  let x = v - ((v >> 1) & 0x55555555);
  x = (x & 0x33333333) + ((x >> 2) & 0x33333333);
  x = (x + (x >> 4)) & 0x0f0f0f0f;
  return (x * 0x01010101) >> 24;
}

/**
 * Reference implementation: one BVH query per cell, gathering boxes.
 *
 * Superseded by the scatter bake below and kept only as the oracle it is
 * checked against — `bakeOccupancy` must agree with this CELL FOR CELL, not
 * merely on the solid count, because the obvious scatter bug (accumulating a
 * hit count instead of a sub-sample mask) double-counts sub-samples covered by
 * two boxes and still produces a plausible total.
 */
export function bakeOccupancyGather(
  boxes: Box[],
  query: BoxQuery,
  dims: [number, number, number],
  origin: [number, number, number],
  cell: [number, number, number],
): OccupancyGrid {

  const [nx, ny, nz] = dims;
  const data = new Uint8Array(nx * ny * nz);
  // Anything shorter than a cell height is below the field's resolution.
  const minHeight = cell[1];
  // A box confined to the top row and continuing above the grid caps the
  // lattice, which its own top face already does — see the note above.
  const latticeTop = origin[1] + ny * cell[1];
  const topRowFloor = latticeTop - cell[1];
  const cand: number[] = [];
  const kept: Box[] = [];
  let solidCells = 0;
  for (let k = 0; k < nz; k++) {
    const z0 = origin[2] + k * cell[2];
    for (let j = 0; j < ny; j++) {
      const y0 = origin[1] + j * cell[1];
      for (let i = 0; i < nx; i++) {
        const x0 = origin[0] + i * cell[0];
        cand.length = 0;
        query(x0, y0, z0, x0 + cell[0], y0 + cell[1], z0 + cell[2], cand);
        if (cand.length === 0) continue;
        kept.length = 0;
        for (let c = 0; c < cand.length; c++) {
          const b = boxes[cand[c]];
          const bb = boxBounds(b);
          if (bb.max.y - bb.min.y < minHeight) continue;
          if (bb.min.y >= topRowFloor && bb.max.y > latticeTop) continue;
          kept.push(b);
        }
        if (kept.length === 0) continue;
        let hits = 0;
        for (let sz = 0; sz < 3 && hits < SOLID_MIN_HITS; sz++) {
          const pz = z0 + ((2 * sz + 1) / 6) * cell[2];
          for (let sy = 0; sy < 3; sy++) {
            const py = y0 + ((2 * sy + 1) / 6) * cell[1];
            for (let sx = 0; sx < 3; sx++) {
              const px = x0 + ((2 * sx + 1) / 6) * cell[0];
              for (let bIdx = 0; bIdx < kept.length; bIdx++) {
                if (inside(kept[bIdx], px, py, pz)) { hits++; break; }
              }
            }
          }
        }
        if (hits >= SOLID_MIN_HITS) {
          data[(k * ny + j) * nx + i] = 1;
          solidCells++;
        }
      }
    }
  }
  return { dims, data, solidCells };
}

/**
 * Scatter bake: iterate boxes, not cells.
 *
 * The gather form above runs one BVH query per cell — 389,376 of them for the
 * shipped lattice, and 24.9 million if a 6.25 cm level-wide field is ever
 * wanted — when the scene holds only a few hundred boxes. Inverting the loop
 * makes it one query for the whole lattice plus a rasterisation of each kept
 * box over its own cell range, which is proportional to the geometry rather
 * than to the volume, and is what makes a finer field affordable at all.
 *
 * The correctness trap: a sub-sample covered by TWO boxes must count once.
 * The gather form gets this free by breaking out of its box loop on the first
 * hit; a scatter form accumulating a per-cell counter does not, and
 * over-counts exactly where geometry overlaps — which in this scene is every
 * wall junction. So each cell carries a 27-bit mask of which sub-samples are
 * covered, and the threshold is applied to its popcount.
 */
export function bakeOccupancy(
  boxes: Box[],
  query: BoxQuery,
  dims: [number, number, number],
  origin: [number, number, number],
  cell: [number, number, number],
): OccupancyGrid {
  const [nx, ny, nz] = dims;
  const data = new Uint8Array(nx * ny * nz);
  const minHeight = cell[1];
  const latticeTop = origin[1] + ny * cell[1];
  const topRowFloor = latticeTop - cell[1];

  // One query for the entire lattice. The per-sub-sample containment test
  // below is exact, so a superset of candidates costs time and not accuracy.
  const cand: number[] = [];
  query(
    origin[0], origin[1], origin[2],
    origin[0] + nx * cell[0], origin[1] + ny * cell[1], origin[2] + nz * cell[2],
    cand,
  );

  const mask = new Int32Array(nx * ny * nz);
  for (let c = 0; c < cand.length; c++) {
    const b = boxes[cand[c]];
    const bb = boxBounds(b);
    if (bb.max.y - bb.min.y < minHeight) continue;
    if (bb.min.y >= topRowFloor && bb.max.y > latticeTop) continue;

    // Cell range this box's AABB can touch. An oriented box is smaller than
    // its AABB, so this over-covers; `inside` rejects the difference.
    const i0 = Math.max(0, Math.floor((bb.min.x - origin[0]) / cell[0]));
    const i1 = Math.min(nx - 1, Math.floor((bb.max.x - origin[0]) / cell[0]));
    const j0 = Math.max(0, Math.floor((bb.min.y - origin[1]) / cell[1]));
    const j1 = Math.min(ny - 1, Math.floor((bb.max.y - origin[1]) / cell[1]));
    const k0 = Math.max(0, Math.floor((bb.min.z - origin[2]) / cell[2]));
    const k1 = Math.min(nz - 1, Math.floor((bb.max.z - origin[2]) / cell[2]));
    if (i0 > i1 || j0 > j1 || k0 > k1) continue;

    const aa = axisAligned(b);
    for (let k = k0; k <= k1; k++) {
      const z0 = origin[2] + k * cell[2];
      const mz = aa ? axisMask(z0, cell[2], bb.min.z, bb.max.z) : 0;
      if (aa && mz === 0) { continue; }
      for (let j = j0; j <= j1; j++) {
        const y0 = origin[1] + j * cell[1];
        const rowBase = (k * ny + j) * nx;
        if (aa) {
          const my = axisMask(y0, cell[1], bb.min.y, bb.max.y);
          if (my === 0) { continue; }
          for (let i = i0; i <= i1; i++) {
            const mx = axisMask(origin[0] + i * cell[0], cell[0], bb.min.x, bb.max.x);
            if (mx === 0) { continue; }
            // Outer product: bit (sz*9 + sy*3 + sx) is set when all three
            // axis masks carry their component.
            let m = 0;
            for (let sz = 0; sz < 3; sz++) {
              if ((mz & (1 << sz)) === 0) { continue; }
              for (let sy = 0; sy < 3; sy++) {
                if ((my & (1 << sy)) === 0) { continue; }
                m |= mx << (sz * 9 + sy * 3);
              }
            }
            mask[rowBase + i] |= m;
          }
          continue;
        }
        for (let i = i0; i <= i1; i++) {
          const x0 = origin[0] + i * cell[0];
          let m = mask[rowBase + i];
          let bit = 1;
          for (let sz = 0; sz < 3; sz++) {
            const pz = z0 + ((2 * sz + 1) / 6) * cell[2];
            for (let sy = 0; sy < 3; sy++) {
              const py = y0 + ((2 * sy + 1) / 6) * cell[1];
              for (let sx = 0; sx < 3; sx++) {
                const px = x0 + ((2 * sx + 1) / 6) * cell[0];
                if ((m & bit) === 0 && inside(b, px, py, pz)) { m = m | bit; }
                bit = bit << 1;
              }
            }
          }
          mask[rowBase + i] = m;
        }
      }
    }
  }

  let solidCells = 0;
  for (let n = 0; n < mask.length; n++) {
    if (popcount27(mask[n]) >= SOLID_MIN_HITS) { data[n] = 1; solidCells++; }
  }
  return { dims, data, solidCells };
}

/**
 * The same bake, packed one bit per cell instead of one byte.
 *
 * This is the field a fine local lattice reads through a window: it has to
 * span the whole level (the lattice moves), so at 12.5 cm it is 3.1M cells and
 * at 6.25 cm it is 24.9M. A byte each would be 3 MB and 25 MB; a bit each is
 * 390 KB and 3.1 MB. The 12.5 cm bit field is 389,376 bytes — byte for byte
 * the size of the coarse lattice's existing byte-per-cell field.
 *
 * x-fastest, matching the byte layout, so the index arithmetic is shared.
 */
export function packOccupancyBits(grid: OccupancyGrid): Uint32Array {
  const n = grid.data.length;
  const words = new Uint32Array((n + 31) >> 5);
  for (let i = 0; i < n; i++) {
    if (grid.data[i]) { words[i >> 5] |= 1 << (i & 31); }
  }
  return words;
}
