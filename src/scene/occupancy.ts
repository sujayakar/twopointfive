import { Box, FLAG_NO_CAMERA, boxBounds } from "./scene";

/**
 * Occupancy field for the fluid solver: which cells of a world-space grid
 * are solid (walls, columns, crates, partitions) so smoke flows around them.
 *
 * Baked once from the same static boxes the tracer draws, through the CPU
 * BVH query the throwable physics already carries — one geometry, one
 * source of truth, and no second acceleration structure.
 *
 * A cell is solid when enough of a 3x3x3 sub-lattice of it lies inside kept
 * boxes. Boxes shorter than a cell (desk tops, carpets, light housings) are
 * skipped: they cannot deflect a plume that is coarser than they are, and
 * keeping them would only stipple the field with false one-cell obstacles.
 * The camera-invisible ceiling slab is skipped too — the room's top is the
 * grid's own boundary, not geometry.
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

function boxFlagged(b: Box, flag: number): boolean {
  return ((b.flags ?? 0) & flag) !== 0;
}

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

export function bakeOccupancy(
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
          if (boxFlagged(b, FLAG_NO_CAMERA)) continue;
          const bb = boxBounds(b);
          if (bb.max.y - bb.min.y < minHeight) continue;
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
