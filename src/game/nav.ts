import { Box, boxBounds } from "../scene/scene";

// ---------------------------------------------------------------------------
// Coarse XZ navigation grid + A*.
//
// Only for excursions off the authored patrol polylines (chasing a last-known
// position, walking a search pattern back to the route). Patrols never touch
// it, so a coarse 0.5 m raster is enough: a guard is a 0.32 m circle and the
// tightest gap it ever has to thread on purpose is a 1.3 m doorway.
// ---------------------------------------------------------------------------

/** Guards are the same rig as the player: 0.28 m radius plus a little slack. */
const AGENT_RADIUS = 0.34;
/** A box blocks a walking body if any part of it sits inside this height band. */
const BLOCK_MIN_Y = 0.32;
const BLOCK_MAX_Y = 1.5;

const SQRT2 = Math.SQRT2;

export class NavGrid {
  readonly cell: number;
  readonly w: number;
  readonly h: number;
  readonly minX: number;
  readonly minZ: number;
  /** 1 = blocked. Row-major, x fastest. */
  readonly blocked: Uint8Array;

  // A* scratch, allocated once. Excursions are rare but they must not stall.
  private gScore: Float32Array;
  private fScore: Float32Array;
  private parent: Int32Array;
  private state: Uint8Array; // 0 unvisited, 1 open, 2 closed
  private heap: Int32Array;
  private stamp = 0;
  private stamps: Uint32Array;

  constructor(
    bounds: { minX: number; minZ: number; maxX: number; maxZ: number },
    colliders: Float32Array,
    boxes: readonly Box[],
    cell = 0.5,
  ) {
    this.cell = cell;
    this.minX = bounds.minX;
    this.minZ = bounds.minZ;
    this.w = Math.ceil((bounds.maxX - bounds.minX) / cell);
    this.h = Math.ceil((bounds.maxZ - bounds.minZ) / cell);
    const n = this.w * this.h;
    this.blocked = new Uint8Array(n);
    this.gScore = new Float32Array(n);
    this.fScore = new Float32Array(n);
    this.parent = new Int32Array(n);
    this.state = new Uint8Array(n);
    this.heap = new Int32Array(n + 1);
    this.stamps = new Uint32Array(n);

    // Movement colliders are the authoritative footprints for walls, desks,
    // crates and partitions; the box scan below adds the clutter they omit
    // (chairs, racks' housings) so a path never routes a guard through a chair.
    for (let i = 0; i < colliders.length; i += 4) {
      this.fillRect(colliders[i], colliders[i + 1], colliders[i + 2], colliders[i + 3]);
    }
    for (const b of boxes) {
      const bb = boxBounds(b);
      // Height band a walking torso occupies. Floor slabs, carpets, ceilings
      // and wall fixtures all fall outside it.
      if (bb.max.y < BLOCK_MIN_Y || bb.min.y > BLOCK_MAX_Y) continue;
      this.fillRect(bb.min.x, bb.min.z, bb.max.x, bb.max.z);
    }
    // The border cells are the exterior walls anyway; sealing them means the
    // neighbour loop never has to bounds-check.
    for (let x = 0; x < this.w; x++) {
      this.blocked[x] = 1;
      this.blocked[(this.h - 1) * this.w + x] = 1;
    }
    for (let z = 0; z < this.h; z++) {
      this.blocked[z * this.w] = 1;
      this.blocked[z * this.w + this.w - 1] = 1;
    }
  }

  private fillRect(x0: number, z0: number, x1: number, z1: number): void {
    const ix0 = Math.max(0, Math.floor((x0 - AGENT_RADIUS - this.minX) / this.cell));
    const ix1 = Math.min(this.w - 1, Math.floor((x1 + AGENT_RADIUS - this.minX) / this.cell));
    const iz0 = Math.max(0, Math.floor((z0 - AGENT_RADIUS - this.minZ) / this.cell));
    const iz1 = Math.min(this.h - 1, Math.floor((z1 + AGENT_RADIUS - this.minZ) / this.cell));
    for (let iz = iz0; iz <= iz1; iz++) {
      for (let ix = ix0; ix <= ix1; ix++) this.blocked[iz * this.w + ix] = 1;
    }
  }

  toCell(x: number, z: number): number {
    const ix = Math.floor((x - this.minX) / this.cell);
    const iz = Math.floor((z - this.minZ) / this.cell);
    if (ix < 0 || iz < 0 || ix >= this.w || iz >= this.h) return -1;
    return iz * this.w + ix;
  }

  cellCenter(idx: number): [number, number] {
    const ix = idx % this.w;
    const iz = (idx - ix) / this.w;
    return [this.minX + (ix + 0.5) * this.cell, this.minZ + (iz + 0.5) * this.cell];
  }

  isOpenAt(x: number, z: number): boolean {
    const c = this.toCell(x, z);
    return c >= 0 && this.blocked[c] === 0;
  }

  /**
   * Nearest open cell to a world point, or -1.
   *
   * A last-known position is where the *player* stood, and the player fits in
   * gaps the guard raster calls solid, so goals routinely land in blocked
   * cells. Spiral out until something walkable turns up.
   */
  nearestOpen(x: number, z: number, maxRing = 8): number {
    let ix = Math.floor((x - this.minX) / this.cell);
    let iz = Math.floor((z - this.minZ) / this.cell);
    ix = Math.min(this.w - 1, Math.max(0, ix));
    iz = Math.min(this.h - 1, Math.max(0, iz));
    if (this.blocked[iz * this.w + ix] === 0) return iz * this.w + ix;
    let best = -1;
    let bestD2 = Infinity;
    for (let r = 1; r <= maxRing && best < 0; r++) {
      for (let dz = -r; dz <= r; dz++) {
        for (let dx = -r; dx <= r; dx++) {
          if (Math.max(Math.abs(dx), Math.abs(dz)) !== r) continue;
          const cx = ix + dx, cz = iz + dz;
          if (cx < 0 || cz < 0 || cx >= this.w || cz >= this.h) continue;
          const idx = cz * this.w + cx;
          if (this.blocked[idx]) continue;
          const d2 = dx * dx + dz * dz;
          if (d2 < bestD2) { bestD2 = d2; best = idx; }
        }
      }
    }
    return best;
  }

  /**
   * A* over the raster, 8-connected, no corner cutting. Returns world-space
   * waypoints ending exactly at the goal cell centre, or null when there is no
   * route. Deterministic: ties break by cell index.
   */
  findPath(fromX: number, fromZ: number, toX: number, toZ: number): Array<[number, number]> | null {
    const start = this.nearestOpen(fromX, fromZ);
    const goal = this.nearestOpen(toX, toZ);
    if (start < 0 || goal < 0) return null;
    if (start === goal) return [this.cellCenter(goal)];

    // A frame stamp instead of clearing three arrays of ~7.5k entries.
    this.stamp++;
    if (this.stamp === 0xffffffff) { this.stamps.fill(0); this.stamp = 1; }
    const stamp = this.stamp;
    const w = this.w;
    const gx = goal % w, gz = (goal - gx) / w;
    const heur = (idx: number) => {
      const ix = idx % w, iz = (idx - ix) / w;
      const dx = Math.abs(ix - gx), dz = Math.abs(iz - gz);
      return (dx + dz) + (SQRT2 - 2) * Math.min(dx, dz);
    };
    const touch = (idx: number) => {
      if (this.stamps[idx] !== stamp) {
        this.stamps[idx] = stamp;
        this.gScore[idx] = Infinity;
        this.state[idx] = 0;
        this.parent[idx] = -1;
      }
    };

    let heapSize = 0;
    const f = this.fScore, g = this.gScore, heap = this.heap;
    const push = (idx: number) => {
      let i = heapSize++;
      heap[i] = idx;
      while (i > 0) {
        const p = (i - 1) >> 1;
        if (f[heap[p]] < f[heap[i]] || (f[heap[p]] === f[heap[i]] && heap[p] <= heap[i])) break;
        const t = heap[p]; heap[p] = heap[i]; heap[i] = t;
        i = p;
      }
    };
    const pop = (): number => {
      const top = heap[0];
      heap[0] = heap[--heapSize];
      let i = 0;
      for (;;) {
        const l = i * 2 + 1, r = l + 1;
        let m = i;
        if (l < heapSize && (f[heap[l]] < f[heap[m]] || (f[heap[l]] === f[heap[m]] && heap[l] < heap[m]))) m = l;
        if (r < heapSize && (f[heap[r]] < f[heap[m]] || (f[heap[r]] === f[heap[m]] && heap[r] < heap[m]))) m = r;
        if (m === i) break;
        const t = heap[m]; heap[m] = heap[i]; heap[i] = t;
        i = m;
      }
      return top;
    };

    touch(start);
    g[start] = 0;
    f[start] = heur(start);
    this.state[start] = 1;
    push(start);

    const DX = [1, -1, 0, 0, 1, 1, -1, -1];
    const DZ = [0, 0, 1, -1, 1, -1, 1, -1];
    let found = false;
    while (heapSize > 0) {
      const cur = pop();
      if (this.state[cur] === 2) continue; // stale duplicate
      if (cur === goal) { found = true; break; }
      this.state[cur] = 2;
      const cx = cur % w;
      for (let k = 0; k < 8; k++) {
        const nx = cx + DX[k];
        const nb = cur + DZ[k] * w + DX[k];
        if (nx < 0 || nx >= w) continue;
        if (nb < 0 || nb >= this.blocked.length || this.blocked[nb]) continue;
        // Diagonals may not cut a blocked corner: the guard has a radius.
        if (k >= 4 && (this.blocked[cur + DX[k]] || this.blocked[cur + DZ[k] * w])) continue;
        touch(nb);
        if (this.state[nb] === 2) continue;
        const cost = g[cur] + (k >= 4 ? SQRT2 : 1);
        if (cost >= g[nb]) continue;
        g[nb] = cost;
        f[nb] = cost + heur(nb);
        this.parent[nb] = cur;
        this.state[nb] = 1;
        push(nb);
      }
    }
    if (!found) return null;

    const rev: Array<[number, number]> = [];
    for (let c = goal; c !== -1; c = this.parent[c]) {
      rev.push(this.cellCenter(c));
      if (c === start) break;
    }
    rev.reverse();
    return this.smooth(rev);
  }

  /**
   * Greedy string-pull: drop every waypoint the guard can walk straight past.
   * The raster path zigzags on the diagonal; this is what makes an excursion
   * read as a walk rather than a staircase.
   */
  private smooth(pts: Array<[number, number]>): Array<[number, number]> {
    if (pts.length <= 2) return pts;
    const out: Array<[number, number]> = [pts[0]];
    let anchor = 0;
    for (let i = 2; i < pts.length; i++) {
      if (!this.walkable(pts[anchor], pts[i])) {
        out.push(pts[i - 1]);
        anchor = i - 1;
      }
    }
    out.push(pts[pts.length - 1]);
    return out;
  }

  /** True when the straight segment stays on open cells (sampled at a quarter cell). */
  walkable(a: [number, number], b: [number, number]): boolean {
    const d = Math.hypot(b[0] - a[0], b[1] - a[1]);
    const steps = Math.max(1, Math.ceil(d / (this.cell * 0.25)));
    for (let i = 0; i <= steps; i++) {
      const t = i / steps;
      if (!this.isOpenAt(a[0] + (b[0] - a[0]) * t, a[1] + (b[1] - a[1]) * t)) return false;
    }
    return true;
  }
}
