// Box scene representation, GPU packing and the 2D (XZ) uniform grid used by all ray queries.
import { Vec3, Quat, v3, quat } from '../math/vec';
import { WORLD } from './world';
import { makeBuffer } from '../gpu/device';

export const BoxFlag = {
  NoPrimary: 1,      // not rasterized into the G-buffer (ceilings, light-only geometry)
  NoShadow: 2,       // ignored by all rays (decals, glow cards)
  Cutaway: 4,        // rasterized only below mat.cutHeight (the viewer stage drops its back wall to a lip when the orbit camera goes behind it); rays see the full box
  Dynamic: 8,        // rebuilt every frame (character proxies, props on bones)
  Emissive: 16,      // has non-zero emission (lets shaders early-out)
  AreaLit: 32,       // its emission is represented by an analytic area light: rays ignore the emissive term (no double count, no small-emitter aliasing in the cascades)
  Rot: 64,           // carries a full 3D rotation (Box.rot; set automatically on upload): raster AND rays use the quaternion in boxMat instead of the packed yaw
  SoundSeal: 1 << 16,   // CPU-only (bits 8-15 of the GPU word are the owner byte, so CPU flags start at 1<<16): a dynamic occluder that currently seals sound (a door leaf near closed)
} as const;

export interface Box {
  c: Vec3;            // center
  h: Vec3;            // half extents (local axes)
  yaw: number;        // rotation about +Y (radians)
  albedo: Vec3;       // linear diffuse reflectance
  emissive: Vec3;     // emitted radiance (linear, HDR)
  flags: number;
  owner: number;      // 0 = world, 1..255 = character id (rays from a character skip its own proxies)
  cutHeight: number;  // world Y above which the box is hidden from primary visibility when Cutaway
  /** optional full 3D orientation (unit quaternion, world) for the few things that tilt: hand props, goggles, spark streaks, tumbling
   *  canisters. Raster and rays both honour it (BoxFlag.Rot is set on upload; the grid footprint and CPU picking use it too); `yaw`
   *  should still hold the heading for CPU code that only cares about that. Absent = yaw about +Y as always. */
  rot?: Quat;
  name?: string;      // optional tag (light fixtures etc.)
}

export function makeBox(p: Partial<Box> & { c: Vec3; h: Vec3 }): Box {
  return { yaw: 0, albedo: [0.5, 0.5, 0.5], emissive: [0, 0, 0], flags: 0, owner: 0, cutHeight: 1e6, ...p };
}

const GEO_STRIDE = 8; // words (32 B): cx cy cz rot | hx hy hz flags
const MAT_STRIDE = 12; // words (48 B): albedo.rgb cutHeight | emissive.rgb owner | rotation quaternion xyzw (= yaw for ordinary boxes, the full 3D rotation for boxes with Box.rot; raster and rays both use it)

// f32 -> f16 bits (round-to-nearest), for pack2x16float on the CPU.
const f16Scratch = new Float32Array(1); const u32Scratch = new Uint32Array(f16Scratch.buffer);
function f16bits(v: number): number {
  f16Scratch[0] = v; const x = u32Scratch[0];
  const sign = (x >>> 16) & 0x8000; let exp = ((x >>> 23) & 0xff) - 127 + 15; let mant = x & 0x7fffff;
  if (exp <= 0) { if (exp < -10) return sign; mant |= 0x800000; const shift = 14 - exp; let r = mant >>> shift; if ((mant >>> (shift - 1)) & 1) r++; return sign | r; }
  if (exp >= 31) return sign | 0x7c00;
  let r = sign | (exp << 10) | (mant >>> 13); if (mant & 0x1000) r++; return r;
}
export const pack2x16f = (a: number, b: number) => (f16bits(a) | (f16bits(b) << 16)) >>> 0;
/** the value an f16 bit pattern denotes (what unpack2x16float hands the shader) */
function f16value(bits: number): number {
  const s = bits & 0x8000 ? -1 : 1, e = (bits >> 10) & 31, m = bits & 1023;
  return e === 0 ? s * m * 2 ** -24 : e === 31 ? (m ? NaN : s * Infinity) : s * (1 + m / 1024) * 2 ** (e - 15);
}

export class BoxWorld {
  statics: Box[] = [];
  dynamics: Box[] = [];
  /** Combined list uploaded this frame (statics first, then dynamics). */
  all: Box[] = [];
  maxBoxes: number;
  geoData: ArrayBuffer; geoF32: Float32Array; geoU32: Uint32Array;
  matData: Float32Array; matU32: Uint32Array;
  geoBuf: GPUBuffer; matBuf: GPUBuffer;
  // grid — cellStart holds two regions in ONE buffer: [0, W*H] the plain per-cell start offsets into `items` (what the flag-off tracer and the
  // smoke obstacle bake read, unchanged), then from GRID_CB = W*H+1 two words per cell for the fast traversal (common.wgsl): heightMask24 | cheby<<24
  // and start24 | min(count,255)<<24. `items` likewise: [0, itemCount) plain box indices, [itemCount, 2*itemCount) index | qmin<<16 | qmax<<24.
  cellStart: Uint32Array; // (gridW*gridH + 1) + 2*gridW*gridH
  items: Uint32Array; itemCount = 0;
  cellBuf: GPUBuffer; itemBuf: GPUBuffer;
  globals: number[] = []; // indices of huge boxes tested once per ray
  /** occupancy of the last grid build: empty cells, mean look-ahead (Chebyshev distance to the nearest occupied cell) over the empty ones, busiest cell,
   *  and the registration slack handed to the tracer (metres a traced box may reach past the cells it is registered in; see buildGrid) */
  gridStats = { cells: WORLD.gridW * WORLD.gridH, empty: 0, meanSkip: 0, maxPerCell: 0, slack: 0 };
  private gridRanges = new Int32Array(0);                                   // per box this build: x0, x1, z0, z1 (cells; x0 < 0 = not in the grid), qmin, qmax (quantised padded height extent)
  private gridFill = new Uint32Array(WORLD.gridW * WORLD.gridH);            // running offsets while scattering items
  private gridMask = new Uint32Array(WORLD.gridW * WORLD.gridH);            // per cell: 24-slice height occupancy
  private gridDist = new Uint8Array(WORLD.gridW * WORLD.gridH);             // per cell: Chebyshev distance to the nearest occupied cell (0 = occupied), clamped to 255
  infoBuf: GPUBuffer; infoData = new Uint32Array(4 + 16 + 4 + 32 + 32 + 4 + 128); infoF32 = new Float32Array(this.infoData.buffer);   // struct SceneInfo in common.wgsl: header, 16 global indices, capsule header, 8 character bounds, 16 global (ymin, ymax) pairs, grid vec4 (x = registration slack), 16 global bounds (min.xyz _, max.xyz _)
  /** How far (m) each global's uploaded bounds (SceneInfo.globalsB, the FLAG_GRID_SLABS reject) stand outside the box the GPU traces, on top of the
   *  registration slack: ~100× every float difference between this f64 build and the tracer's f32 slab test (argument above traceClosest in
   *  common.wgsl), and still under the 1–2 cm every ray origin is lifted off its surface — which is what lets a ray leaving the floor skip the floor. */
  static readonly GLOBAL_PAD = 0.005;
  /** …and the same 5 mm is all FLAG_GRID_SLABS keeps of WORLD.gridYPad when it judges a cell's height span: the packed items carry the full 4 cm against
   *  float differences that total < 0.4 mm (tallied in the same place), so the shader hands gridYPad − ITEM_PAD_KEEP of it back at lookup time
   *  (SceneInfo.grid.y) without the packed data changing — enough that a ray lifted 1–2 cm off a desk, a floor finish or a wall top stops re-testing the
   *  very surface it left in its first cell. */
  static readonly ITEM_PAD_KEEP = 0.005;
  // character capsule proxies (see struct Capsule in common.wgsl): 8 floats each
  static readonly MAX_CAPSULES = 128; static readonly CAPS_PER_CHAR = 14;
  capsuleData = new Float32Array(BoxWorld.MAX_CAPSULES * 8); capsuleU32 = new Uint32Array(this.capsuleData.buffer); capsuleCount = 0; charCount = 0;
  capsuleBuf: GPUBuffer;
  visible: Uint32Array; visibleCount = 0; visibleBuf: GPUBuffer;
  staticsDirty = true;

  constructor(private device: GPUDevice, maxBoxes = 4096) {
    this.maxBoxes = maxBoxes;
    this.geoData = new ArrayBuffer(maxBoxes * GEO_STRIDE * 4); this.geoF32 = new Float32Array(this.geoData); this.geoU32 = new Uint32Array(this.geoData);
    this.matData = new Float32Array(maxBoxes * MAT_STRIDE); this.matU32 = new Uint32Array(this.matData.buffer);
    const S = GPUBufferUsage.STORAGE | GPUBufferUsage.COPY_DST;
    this.geoBuf = makeBuffer(device, maxBoxes * GEO_STRIDE * 4, S, 'boxGeo');
    this.matBuf = makeBuffer(device, maxBoxes * MAT_STRIDE * 4, S, 'boxMat');
    if (WORLD.gridW + WORLD.gridH > 96) throw new Error('trace grid: the DDA in common.wgsl caps a ray at 96 cell visits; W + H must stay ≤ 96 for that cap to be unreachable');
    if (maxBoxes > 65536 || maxBoxes * WORLD.gridW * WORLD.gridH >= 1 << 24) throw new Error('trace grid: packed items keep the box index in 16 bits and cell starts in 24');
    this.cellStart = new Uint32Array(WORLD.gridW * WORLD.gridH + 1 + 2 * WORLD.gridW * WORLD.gridH);
    this.items = new Uint32Array(65536);
    this.cellBuf = makeBuffer(device, this.cellStart.byteLength, S, 'gridCells');
    this.itemBuf = makeBuffer(device, this.items.byteLength, S, 'gridItems');
    this.infoBuf = makeBuffer(device, this.infoData.byteLength, GPUBufferUsage.UNIFORM | GPUBufferUsage.COPY_DST, 'sceneInfo');
    this.capsuleBuf = makeBuffer(device, this.capsuleData.byteLength, S, 'capsules');
    this.visible = new Uint32Array(maxBoxes);
    this.visibleBuf = makeBuffer(device, this.visible.byteLength, S, 'visibleBoxes');
  }

  addStatic(b: Box): number { this.statics.push(b); this.staticsDirty = true; return this.statics.length - 1; }

  /** Begin a frame's capsule list. Characters append CAPS_PER_CHAR capsules each via addCharacterCapsules(). */
  resetCapsules() { this.capsuleCount = 0; this.charCount = 0; }
  /** caps: array of [ax,ay,az, r, bx,by,bz] (exactly CAPS_PER_CHAR entries; pad with zero-radius), bound = [cx,cy,cz,radius]. */
  addCharacterCapsules(owner: number, albedo: Vec3, caps: number[][], bound: [number, number, number, number]) {
    if (this.charCount >= 8) return;
    const rgb565 = ((Math.round(albedo[0] * 31) & 31) << 11) | ((Math.round(albedo[1] * 63) & 63) << 5) | (Math.round(albedo[2] * 31) & 31);
    const misc = ((owner & 0xff) | (rgb565 << 8)) >>> 0;
    const base = this.charCount * BoxWorld.CAPS_PER_CHAR;
    for (let i = 0; i < BoxWorld.CAPS_PER_CHAR; i++) {
      const c = caps[i] ?? [0, -100, 0, 0, 0, -100, 0]; const o = (base + i) * 8;
      this.capsuleData[o] = c[0]; this.capsuleData[o + 1] = c[1]; this.capsuleData[o + 2] = c[2]; this.capsuleData[o + 3] = c[3];
      this.capsuleData[o + 4] = c[4]; this.capsuleData[o + 5] = c[5]; this.capsuleData[o + 6] = c[6]; this.capsuleU32[o + 7] = misc;
    }
    const bo = 24 + this.charCount * 4; this.infoF32[bo] = bound[0]; this.infoF32[bo + 1] = bound[1]; this.infoF32[bo + 2] = bound[2]; this.infoF32[bo + 3] = bound[3];
    this.charCount++; this.capsuleCount = this.charCount * BoxWorld.CAPS_PER_CHAR;
  }

  /** Footprint half extents in world XZ for a yawed box. */
  static footprint(b: Box): [number, number] {
    if (b.rot) {   // XZ half extents of the rotated box's world AABB: |R| · h
      const ax = quat.rotate(b.rot, [1, 0, 0]), ay = quat.rotate(b.rot, [0, 1, 0]), az = quat.rotate(b.rot, [0, 0, 1]);
      return [Math.abs(ax[0]) * b.h[0] + Math.abs(ay[0]) * b.h[1] + Math.abs(az[0]) * b.h[2], Math.abs(ax[2]) * b.h[0] + Math.abs(ay[2]) * b.h[1] + Math.abs(az[2]) * b.h[2]];
    }
    const cs = Math.abs(Math.cos(b.yaw)), sn = Math.abs(Math.sin(b.yaw));
    return [cs * b.h[0] + sn * b.h[2], sn * b.h[0] + cs * b.h[2]];
  }
  /** World Y half extent (yawed boxes: h.y; fully rotated ones: the |R|·h row, like footprint()). */
  static heightExtent(b: Box): number {
    if (!b.rot) return b.h[1];
    const ax = quat.rotate(b.rot, [1, 0, 0]), ay = quat.rotate(b.rot, [0, 1, 0]), az = quat.rotate(b.rot, [0, 0, 1]);
    return Math.abs(ax[1]) * b.h[0] + Math.abs(ay[1]) * b.h[1] + Math.abs(az[1]) * b.h[2];
  }
  /** The height quantiser the traversal's y-cull uses (GRID_YQ* in world.ts / common.wgsl): floor((y − y0)·s) clamped to [0, levels−1]. */
  static yLevel(y: number): number {
    const q = Math.floor((y - WORLD.gridY0) * (WORLD.gridYLevels / (WORLD.gridY1 - WORLD.gridY0)));
    return q < 0 ? 0 : q > WORLD.gridYLevels - 1 ? WORLD.gridYLevels - 1 : q;
  }

  /** Rebuild combined list, pack, grid, and upload. Call once per frame after dynamics are set. */
  upload() {
    const all = this.all; all.length = 0;
    for (const b of this.statics) all.push(b); for (const b of this.dynamics) all.push(b);
    if (all.length > this.maxBoxes) throw new Error(`too many boxes: ${all.length}`);
    const gf = this.geoF32, gu = this.geoU32, mf = this.matData, mu = this.matU32;
    let nv = 0;
    for (let i = 0; i < all.length; i++) {
      const b = all[i]; const o = i * GEO_STRIDE;
      let flags = (b.flags & 0xff) | ((b.owner & 0xff) << 8) | (b.rot ? BoxFlag.Rot : 0);   // GPU word: flag bits 0-7, owner byte 8-15 (boxOwner() in common.wgsl); CPU-only flags live at >= 1<<16 and never reach the GPU
      if (b.emissive[0] + b.emissive[1] + b.emissive[2] > 0) flags |= BoxFlag.Emissive;
      gf[o] = b.c[0]; gf[o + 1] = b.c[1]; gf[o + 2] = b.c[2]; gu[o + 3] = pack2x16f(Math.cos(b.yaw), Math.sin(b.yaw));
      gf[o + 4] = b.h[0]; gf[o + 5] = b.h[1]; gf[o + 6] = b.h[2]; gu[o + 7] = flags >>> 0;
      const m = i * MAT_STRIDE;
      mf[m] = b.albedo[0]; mf[m + 1] = b.albedo[1]; mf[m + 2] = b.albedo[2]; mf[m + 3] = b.cutHeight;
      mf[m + 4] = b.emissive[0]; mf[m + 5] = b.emissive[1]; mf[m + 6] = b.emissive[2]; mu[m + 7] = b.owner;
      if (b.rot) { mf[m + 8] = b.rot[0]; mf[m + 9] = b.rot[1]; mf[m + 10] = b.rot[2]; mf[m + 11] = b.rot[3]; }
      else { const s2 = Math.sin(b.yaw / 2), c2 = Math.cos(b.yaw / 2); mf[m + 8] = 0; mf[m + 9] = s2; mf[m + 10] = 0; mf[m + 11] = c2; }   // yaw about +Y (local +X → (cos, 0, −sin), +Z → (sin, 0, cos): the same convention as the packed yaw the rays use)
      if (!(b.flags & (BoxFlag.NoPrimary)) ) this.visible[nv++] = i;
    }
    this.visibleCount = nv;
    const q = this.device.queue;
    q.writeBuffer(this.geoBuf, 0, this.geoData, 0, all.length * GEO_STRIDE * 4);
    q.writeBuffer(this.matBuf, 0, this.matData.buffer, 0, all.length * MAT_STRIDE * 4);
    q.writeBuffer(this.visibleBuf, 0, this.visible.buffer, 0, nv * 4);
    this.buildGrid();
  }

  private buildGrid() {
    const W = WORLD.gridW, H = WORLD.gridH, cs = WORLD.gridCell, N = W * H, CB = N + 1;
    const all = this.all; const counts = this.cellStart; counts.fill(0, 0, N + 1);
    const mask = this.gridMask; mask.fill(0);
    this.globals.length = 0;
    if (this.gridRanges.length < all.length * 6) this.gridRanges = new Int32Array(Math.max(all.length, this.maxBoxes) * 6);
    const ranges = this.gridRanges; // per box: x0,x1,z0,z1 (cell coords; x0 = -1 → not in the grid), qmin, qmax
    const pad = WORLD.gridYPad;
    // Registration slack: how far (m) the box the GPU actually traces can reach outside the footprint it is registered with. Yawed boxes are
    // intersected with the f16 cos/sin packed above — not quite orthonormal — so their traced outline is the footprint scaled by up to
    // ~1 + 2⁻¹⁰ (millimetres on a car); measured here per build from the very bits the shader unpacks. Plus WORLD.gridJitter for the f32
    // rounding of centres, half sizes and ray-relative offsets. The height cull looks this far back across the cell line it just crossed
    // (common.wgsl gridCell callers), which is what keeps it exact for a ray grazing such a sliver.
    let slack = 0; const gu = this.geoU32;
    for (let i = 0; i < all.length; i++) {
      const b = all[i]; const r = i * 6;
      if (b.flags & BoxFlag.NoShadow) { ranges[r] = -1; continue; }
      const [ex, ez] = BoxWorld.footprint(b);
      if (!b.rot) {
        const rot = gu[i * GEO_STRIDE + 3]; const c = Math.abs(f16value(rot & 0xffff)), s = Math.abs(f16value(rot >>> 16));
        if (c !== 0 && s !== 0) { const det = c * c + s * s; slack = Math.max(slack, (c * b.h[0] + s * b.h[2]) / det - ex, (s * b.h[0] + c * b.h[2]) / det - ez); }   // (axis-aligned: c, s ∈ {0, 1} exactly, no slack)
      }
      let x0 = Math.floor((b.c[0] - ex) / cs), x1 = Math.floor((b.c[0] + ex) / cs);
      let z0 = Math.floor((b.c[2] - ez) / cs), z1 = Math.floor((b.c[2] + ez) / cs);
      x0 = Math.max(0, x0); z0 = Math.max(0, z0); x1 = Math.min(W - 1, x1); z1 = Math.min(H - 1, z1);
      if (x1 < x0 || z1 < z0) { ranges[r] = -1; continue; }
      const area = (x1 - x0 + 1) * (z1 - z0 + 1);
      if (area > 96 && this.globals.length < 16) { this.globals.push(i); ranges[r] = -1; continue; }
      ranges[r] = x0; ranges[r + 1] = x1; ranges[r + 2] = z0; ranges[r + 3] = z1;
      // padded, quantised height extent (the y-cull compares these against the ray's remaining height span; the pad covers every rounding
      // difference between this f64 build and the tracer's f32 arithmetic many times over) and the 24 coarse slices it touches
      const ey = BoxWorld.heightExtent(b);
      const qmin = BoxWorld.yLevel(b.c[1] - ey - pad), qmax = BoxWorld.yLevel(b.c[1] + ey + pad);
      ranges[r + 4] = qmin; ranges[r + 5] = qmax;
      const s0 = Math.floor(qmin / 10), s1 = Math.floor(qmax / 10);
      const bits = ((0xffffff >>> (23 - (s1 - s0))) << s0) >>> 0;
      for (let z = z0; z <= z1; z++) for (let x = x0; x <= x1; x++) { counts[z * W + x]++; mask[z * W + x] |= bits; }
    }
    // exclusive prefix sum -> starts (region A: plain, read by the flag-off tracer and the smoke obstacle bake)
    let sum = 0; let maxPer = 0; for (let c = 0; c < N; c++) { const n = counts[c]; if (n > maxPer) maxPer = n; counts[c] = sum; sum += n; } counts[N] = sum;
    if (2 * sum > this.items.length) { this.items = new Uint32Array(Math.max(2 * sum, this.items.length * 2)); this.itemBuf.destroy(); this.itemBuf = makeBuffer(this.device, this.items.byteLength, GPUBufferUsage.STORAGE | GPUBufferUsage.COPY_DST, 'gridItems'); }
    const fill = this.gridFill; fill.fill(0); // running offsets
    const items = this.items;
    for (let i = 0; i < all.length; i++) {
      const r = i * 6; const x0 = ranges[r]; if (x0 < 0) continue; const x1 = ranges[r + 1], z0 = ranges[r + 2], z1 = ranges[r + 3];
      const packed = (i | (ranges[r + 4] << 16) | (ranges[r + 5] << 24)) >>> 0;   // region B item: index16 | qmin8 | qmax8 (levels ≤ 239 fit a byte)
      for (let z = z0; z <= z1; z++) for (let x = x0; x <= x1; x++) { const c = z * W + x; const k = counts[c] + fill[c]++; items[k] = i; items[sum + k] = packed; }
    }
    this.itemCount = sum;
    // Chebyshev distance of every cell to the nearest occupied one (two-pass 8-neighbour chamfer: exact for the chessboard metric). An empty cell at
    // distance d lets the DDA take its next d−1 steps without touching memory: any cell within d−1 king moves is empty as well.
    const dist = this.gridDist;
    for (let c = 0; c < N; c++) dist[c] = counts[c + 1] > counts[c] ? 0 : 255;
    for (let z = 0; z < H; z++) for (let x = 0; x < W; x++) {
      const c = z * W + x; let d = dist[c]; if (d === 0) continue;
      if (x > 0) d = Math.min(d, dist[c - 1] + 1);
      if (z > 0) { d = Math.min(d, dist[c - W] + 1); if (x > 0) d = Math.min(d, dist[c - W - 1] + 1); if (x < W - 1) d = Math.min(d, dist[c - W + 1] + 1); }
      dist[c] = d;
    }
    for (let z = H - 1; z >= 0; z--) for (let x = W - 1; x >= 0; x--) {
      const c = z * W + x; let d = dist[c]; if (d === 0) continue;
      if (x < W - 1) d = Math.min(d, dist[c + 1] + 1);
      if (z < H - 1) { d = Math.min(d, dist[c + W] + 1); if (x < W - 1) d = Math.min(d, dist[c + W + 1] + 1); if (x > 0) d = Math.min(d, dist[c + W - 1] + 1); }
      dist[c] = d;
    }
    // region B: two words per cell for the fast traversal
    let empty = 0, skipSum = 0;
    for (let c = 0; c < N; c++) {
      const n = counts[c + 1] - counts[c]; const d = dist[c];
      if (d > 0) { empty++; skipSum += d; }
      counts[CB + 2 * c] = (mask[c] | (d << 24)) >>> 0;
      counts[CB + 2 * c + 1] = (counts[c] | (Math.min(n, 255) << 24)) >>> 0;   // count 255 = "long cell": the shader reads the end from region A instead
    }
    this.gridStats.empty = empty; this.gridStats.meanSkip = empty ? skipSum / empty : 0; this.gridStats.maxPerCell = maxPer; this.gridStats.slack = slack + WORLD.gridJitter;
    const info = this.infoData; info[0] = all.length; info[1] = this.globals.length; info[2] = W; info[3] = H;
    const infoF = this.infoF32;
    infoF[88] = this.gridStats.slack; infoF[89] = pad - BoxWorld.ITEM_PAD_KEEP; infoF[90] = 0; infoF[91] = 0;   // SceneInfo.grid: x = registration slack (m), y = how much tighter than the packed extents FLAG_GRID_SLABS judges a cell's height span at each end (m)
    const gpad = BoxWorld.GLOBAL_PAD + this.gridStats.slack;
    for (let i = 0; i < 16; i++) {
      const gi = this.globals[i]; info[4 + i] = gi ?? 0;
      const yo = 56 + 2 * i, bo = 92 + 8 * i;   // globalsY pair, globalsB min/max vec4s
      if (gi === undefined) { infoF[yo] = 0; infoF[yo + 1] = 0; infoF.fill(0, bo, bo + 8); continue; }
      const g = all[gi]; const ey = BoxWorld.heightExtent(g); const [ex, ez] = BoxWorld.footprint(g);
      infoF[yo] = g.c[1] - ey - pad; infoF[yo + 1] = g.c[1] + ey + pad;   // globalsY: the same padded height extent per global, in metres (the tracer skips a global the ray's heights never reach)
      // globalsB: the global's world bounds on all three axes, GLOBAL_PAD + slack proud of the traced box — the FLAG_GRID_SLABS reject skips a global the
      // ray's segment [ro, ro + rd·tmax] cannot touch in x, y or z (floors under a ray that leaves the floor, the corridor / break-room finishes for rays
      // that never cross them, the ceiling for exterior rays), where globalsY's 4 cm still had every ray off the floor test all four floor slabs
      infoF[bo] = g.c[0] - ex - gpad; infoF[bo + 1] = g.c[1] - ey - gpad; infoF[bo + 2] = g.c[2] - ez - gpad; infoF[bo + 3] = 0;
      infoF[bo + 4] = g.c[0] + ex + gpad; infoF[bo + 5] = g.c[1] + ey + gpad; infoF[bo + 6] = g.c[2] + ez + gpad; infoF[bo + 7] = 0;
    }
    info[20] = this.capsuleCount; info[21] = this.charCount; info[22] = BoxWorld.CAPS_PER_CHAR; info[23] = sum;   // [23] = itemsB: where the packed copies of the items start
    this.device.queue.writeBuffer(this.capsuleBuf, 0, this.capsuleData.buffer, 0, Math.max(32, this.capsuleCount * 32));
    const q = this.device.queue;
    q.writeBuffer(this.cellBuf, 0, this.cellStart);
    q.writeBuffer(this.itemBuf, 0, this.items.buffer, 0, Math.max(4, 2 * sum * 4));
    q.writeBuffer(this.infoBuf, 0, info);
  }

  /** CPU ray cast against all boxes (closest hit). Used for mouse picking / aim point. */
  raycast(ro: Vec3, rd: Vec3, tmax = 1e9, skipFlags: number = BoxFlag.NoShadow | BoxFlag.NoPrimary, skipOwner = 0): { t: number; index: number; n: Vec3 } | null {
    let best = tmax, bi = -1; let bn: Vec3 = [0, 1, 0];
    for (let i = 0; i < this.all.length; i++) {
      const b = this.all[i]; if (b.flags & skipFlags) continue; if (skipOwner && b.owner === skipOwner) continue;   // (e.g. the cursor's aim ray must not land on the player's own gun / goggles)
      const r = rayBox(ro, rd, b, best); if (r && r.t < best) { best = r.t; bi = i; bn = r.n; }
    }
    return bi >= 0 ? { t: best, index: bi, n: bn } : null;
  }
}

export function rayBox(ro: Vec3, rd: Vec3, b: Box, tmax: number): { t: number; n: Vec3 } | null {
  if (b.rot) return rayBoxRot(ro, rd, b, tmax);
  const cs = Math.cos(b.yaw), sn = Math.sin(b.yaw);
  const px = ro[0] - b.c[0], py = ro[1] - b.c[1], pz = ro[2] - b.c[2];
  // world -> local (R^T)
  const lx = cs * px - sn * pz, lz = sn * px + cs * pz, ly = py;
  const dx = cs * rd[0] - sn * rd[2], dz = sn * rd[0] + cs * rd[2], dy = rd[1];
  const inv = [1 / (dx || 1e-20), 1 / (dy || 1e-20), 1 / (dz || 1e-20)];
  const o = [lx, ly, lz]; let t0 = 0, t1 = tmax; let axis = -1, sgn = 0;
  for (let a = 0; a < 3; a++) {
    let ta = (-b.h[a] - o[a]) * inv[a], tb = (b.h[a] - o[a]) * inv[a]; let s = -1;
    if (ta > tb) { const tmp = ta; ta = tb; tb = tmp; s = 1; }
    if (ta > t0) { t0 = ta; axis = a; sgn = s; }
    if (tb < t1) t1 = tb;
    if (t0 > t1) return null;
  }
  if (axis < 0) return null; // origin inside
  const ln: Vec3 = [0, 0, 0]; ln[axis] = sgn;
  const n: Vec3 = [cs * ln[0] + sn * ln[2], ln[1], -sn * ln[0] + cs * ln[2]];
  return { t: t0, n };
}
/** Same slab test in the frame of a fully rotated box (Box.rot). */
function rayBoxRot(ro: Vec3, rd: Vec3, b: Box, tmax: number): { t: number; n: Vec3 } | null {
  const qi = quat.conj(b.rot!); const o = quat.rotate(qi, v3.sub(ro, b.c)); const d = quat.rotate(qi, rd);
  let t0 = 0, t1 = tmax; let axis = -1, sgn = 0;
  for (let a = 0; a < 3; a++) {
    const inv = 1 / (d[a] || 1e-20);
    let ta = (-b.h[a] - o[a]) * inv, tb = (b.h[a] - o[a]) * inv; let s = -1;
    if (ta > tb) { const tmp = ta; ta = tb; tb = tmp; s = 1; }
    if (ta > t0) { t0 = ta; axis = a; sgn = s; }
    if (tb < t1) t1 = tb;
    if (t0 > t1) return null;
  }
  if (axis < 0) return null;
  const ln: Vec3 = [0, 0, 0]; ln[axis] = sgn;
  return { t: t0, n: quat.rotate(b.rot!, ln) };
}
