import { Vec3, v3 } from "../core/math";
import { Box } from "../scene/scene";
import { BVH, BVH_NODE_STRIDE_F32 } from "../scene/bvh";

// ---------------------------------------------------------------------------
// CPU ray casts against the static world.
//
// A direct port of `trace` in common.wgsl, so a bullet lands where the image
// actually shows geometry. That equivalence is the whole reason this exists
// rather than a simpler collision proxy: a shot that visibly passes through a
// wall, or stops at one that is not drawn, is worse than no ray at all.
//
// Extracted from a rigid-body module that was removed — moving geometry looks
// bad under a temporal denoiser and costs per-ray time. Ray casting has neither
// problem: nothing moves, and this runs on the CPU.
// ---------------------------------------------------------------------------

const STACK_DEPTH = 32;
const RAY_EPS = 1e-4;
/** Guards against a division by zero producing NaN in the slab test. */
const DIR_EPS = 1e-12;

export interface RaycastHit {
  /** Distance along `dir`, which must be unit length. */
  t: number;
  /** Surface normal, already flipped to face the incoming ray. */
  normal: Vec3;
  /** Index into the `boxes` array this was built from. */
  hit: number;
}

export class Raycaster {
  private readonly _boxes: Box[];
  private readonly _bvh: BVH;
  private readonly _bvhU32: Uint32Array;
  private readonly _stack = new Int32Array(STACK_DEPTH);

  constructor(boxes: Box[], bvh: BVH) {
    this._boxes = boxes;
    this._bvh = bvh;
    this._bvhU32 = new Uint32Array(bvh.nodes.buffer, bvh.nodes.byteOffset, bvh.nodes.length);
  }

  raycast(origin: Vec3, dir: Vec3, tmax: number): RaycastHit | null {
    const bvh = this._bvh;
    if (bvh.order.length === 0) return null;
    const nodes = bvh.nodes;
    const u32 = this._bvhU32;

    const dx = Math.abs(dir.x) < DIR_EPS ? (dir.x < 0 ? -DIR_EPS : DIR_EPS) : dir.x;
    const dy = Math.abs(dir.y) < DIR_EPS ? (dir.y < 0 ? -DIR_EPS : DIR_EPS) : dir.y;
    const dz = Math.abs(dir.z) < DIR_EPS ? (dir.z < 0 ? -DIR_EPS : DIR_EPS) : dir.z;
    const ix = 1 / dx, iy = 1 / dy, iz = 1 / dz;

    let bestT = tmax;
    let bestBox = -1;
    let bnx = 0, bny = 1, bnz = 0;

    const stack = this._stack;
    let sp = 0;
    let cur = 0;

    for (;;) {
      const o = cur * BVH_NODE_STRIDE_F32;
      const count = u32[o + 7];
      if (count > 0) {
        const first = u32[o + 3];
        for (let i = 0; i < count; i++) {
          const bi = bvh.order[first + i];
          if (hitBox(origin, dir, this._boxes[bi], RAY_EPS, bestT)) {
            bestT = hitT; bnx = hitNX; bny = hitNY; bnz = hitNZ; bestBox = bi;
          }
        }
        if (sp === 0) break;
        cur = stack[--sp];
      } else {
        const li = u32[o + 3];
        const ri = li + 1;
        const dl = slabAABB(origin, ix, iy, iz, nodes, li * BVH_NODE_STRIDE_F32, bestT);
        const dr = slabAABB(origin, ix, iy, iz, nodes, ri * BVH_NODE_STRIDE_F32, bestT);
        if (dl < 0 && dr < 0) {
          if (sp === 0) break;
          cur = stack[--sp];
        } else if (dl < 0) {
          cur = ri;
        } else if (dr < 0) {
          cur = li;
        } else if (dl < dr) {
          if (sp < STACK_DEPTH - 1) stack[sp++] = ri;
          cur = li;
        } else {
          if (sp < STACK_DEPTH - 1) stack[sp++] = li;
          cur = ri;
        }
      }
    }

    if (bestBox < 0) return null;
    // Two-sided shading convention, same as trace().
    if (bnx * dir.x + bny * dir.y + bnz * dir.z > 0) { bnx = -bnx; bny = -bny; bnz = -bnz; }
    return { t: bestT, normal: v3(bnx, bny, bnz), hit: bestBox };
  }

  /** True when anything blocks the segment from `a` to `b`. */
  blocked(a: Vec3, b: Vec3): boolean {
    const dx = b.x - a.x, dy = b.y - a.y, dz = b.z - a.z;
    const d = Math.hypot(dx, dy, dz);
    if (d < 1e-5) return false;
    const inv = 1 / d;
    const hit = this.raycast(a, v3(dx * inv, dy * inv, dz * inv), d - RAY_EPS * 8);
    return hit !== null;
  }
}

// --- module scratch, to keep the hot path allocation-free --------------------
let hitT = 0;
let hitNX = 0, hitNY = 0, hitNZ = 0;

function slabAABB(
  ro: Vec3, ix: number, iy: number, iz: number,
  nodes: Float32Array, o: number, tmax: number,
): number {
  const ax = (nodes[o] - ro.x) * ix, bx = (nodes[o + 4] - ro.x) * ix;
  const ay = (nodes[o + 1] - ro.y) * iy, by = (nodes[o + 5] - ro.y) * iy;
  const az = (nodes[o + 2] - ro.z) * iz, bz = (nodes[o + 6] - ro.z) * iz;
  const tNear = Math.max(Math.min(ax, bx), Math.min(ay, by), Math.min(az, bz));
  const tFar = Math.min(Math.max(ax, bx), Math.max(ay, by), Math.max(az, bz));
  if (tFar < Math.max(tNear, 0) || tNear > tmax) return -1;
  return Math.max(tNear, 0);
}

function hitBox(ro: Vec3, rd: Vec3, box: Box, tmin: number, tmax: number): boolean {
  const dx = ro.x - box.center.x, dy = ro.y - box.center.y, dz = ro.z - box.center.z;
  const r0 = box.rot[0], r1 = box.rot[1], r2 = box.rot[2];
  // World -> local is three dot products: the rotation is orthonormal and its
  // columns are the local axes in world space.
  const lox = dx * r0.x + dy * r0.y + dz * r0.z;
  const loy = dx * r1.x + dy * r1.y + dz * r1.z;
  const loz = dx * r2.x + dy * r2.y + dz * r2.z;
  let ldx = rd.x * r0.x + rd.y * r0.y + rd.z * r0.z;
  let ldy = rd.x * r1.x + rd.y * r1.y + rd.z * r1.z;
  let ldz = rd.x * r2.x + rd.y * r2.y + rd.z * r2.z;
  if (Math.abs(ldx) < DIR_EPS) ldx = ldx < 0 ? -DIR_EPS : DIR_EPS;
  if (Math.abs(ldy) < DIR_EPS) ldy = ldy < 0 ? -DIR_EPS : DIR_EPS;
  if (Math.abs(ldz) < DIR_EPS) ldz = ldz < 0 ? -DIR_EPS : DIR_EPS;

  const ix = 1 / ldx, iy = 1 / ldy, iz = 1 / ldz;
  const h = box.half;
  const ax = (-h.x - lox) * ix, bx = (h.x - lox) * ix;
  const ay = (-h.y - loy) * iy, by = (h.y - loy) * iy;
  const az = (-h.z - loz) * iz, bz = (h.z - loz) * iz;
  const tnx = Math.min(ax, bx), tfx = Math.max(ax, bx);
  const tny = Math.min(ay, by), tfy = Math.max(ay, by);
  const tnz = Math.min(az, bz), tfz = Math.max(az, bz);
  const tNear = Math.max(tnx, tny, tnz);
  const tFar = Math.min(tfx, tfy, tfz);
  if (tFar < tNear || tFar < tmin) return false;

  const inside = tNear < tmin;
  const t = inside ? tFar : tNear;
  if (t < tmin || t > tmax) return false;

  // Whichever slab produced the extremum owns the face.
  let nlx = 0, nly = 0, nlz = 0;
  if (inside) {
    if (tfx <= tfy && tfx <= tfz) nlx = ldx > 0 ? -1 : 1;
    else if (tfy <= tfz) nly = ldy > 0 ? -1 : 1;
    else nlz = ldz > 0 ? -1 : 1;
  } else {
    if (tnx >= tny && tnx >= tnz) nlx = ldx > 0 ? -1 : 1;
    else if (tny >= tnz) nly = ldy > 0 ? -1 : 1;
    else nlz = ldz > 0 ? -1 : 1;
  }
  hitT = t;
  hitNX = nlx * r0.x + nly * r1.x + nlz * r2.x;
  hitNY = nlx * r0.y + nly * r1.y + nlz * r2.y;
  hitNZ = nlx * r0.z + nly * r1.z + nlz * r2.z;
  return true;
}
