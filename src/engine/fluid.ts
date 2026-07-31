import { halfToFloat } from "../core/half";
import { SHADERS, createShaderModule } from "./shaders";

/**
 * GPU stable-fluids solver for the smoke medium — see fluid.wgsl for the
 * numerics. Owns its own lattice and every simulation resource, and writes the
 * density interface texture (the volumetric channel's contract) as its last
 * pass each step. The renderer only hands it a command encoder, the frame's
 * game dt and the game's source list.
 */

/** Tunables. Defaults chosen by eye against the headless verification shots. */
export interface FluidTuning {
  /** Pressure-projection Jacobi iterations per step (rounded up to even). */
  jacobi: number;
  /** Vorticity-confinement strength — the knob that makes it read as smoke. */
  vorticity: number;
  /** Upward acceleration per unit temperature, m/s^2. */
  buoyancy: number;
  /** Downward acceleration per unit density, m/s^2. Dense smoke pools. */
  weight: number;
  /** Density decay rate 1/s (smoke thins on its own). */
  dissipation: number;
  /** Temperature decay rate 1/s (hot air cools, so a plume stops rising). */
  cooling: number;
}

export const DEFAULT_FLUID_TUNING: FluidTuning = {
  // 40 measured to ~2e-3 relative divergence residual on the room grid; the
  // curl of a pooling canister cloud reads the same at 20, so this is headroom.
  jacobi: 40,
  vorticity: 1.6,
  buoyancy: 1.4,
  weight: 0.045,
  // ~8 s e-folding: a canister cloud that stopped emitting at 8 s is still a
  // haze at 20 s and gone by ~30 s.
  dissipation: 0.13,
  cooling: 0.6,
};

/** Sources per step and the float stride of one (matches the WGSL Source struct). */
export const FLUID_MAX_SOURCES = 32;
export const FLUID_SOURCE_STRIDE = 12;

const HEADER_F32 = 28;
const PARAM_BYTES = (HEADER_F32 + FLUID_MAX_SOURCES * FLUID_SOURCE_STRIDE) * 4;
const WG = 4;

/** Builds the static-solid field for a given lattice; 1 = solid, x-fastest. */
export type OccupancyBaker = (
  dims: [number, number, number],
  origin: [number, number, number],
  cell: [number, number, number],
) => Uint8Array;

export interface FluidInterface {
  /** The density interface texture (rgba16float, storage-writable). */
  volume: GPUTexture;
  dims: [number, number, number];
  origin: [number, number, number];
  /** Cubic cell size of the interface grid, metres. */
  cell: number;
}

type Profile = (label: string) => GPUComputePassTimestampWrites | undefined;

export class FluidSim {
  readonly tune: FluidTuning = { ...DEFAULT_FLUID_TUNING };
  /** Sim lattice = interface grid divided by `scale` in x/z (y is kept). */
  scale = 1;
  dims: [number, number, number] = [1, 1, 1];
  cell: [number, number, number] = [1, 1, 1];
  /** Solid cell count from the last occupancy bake. */
  solidCells = 0;

  private layout!: GPUBindGroupLayout;
  private pipes!: Record<string, GPUComputePipeline>;
  private sampler!: GPUSampler;
  private params!: GPUBuffer;
  private paramData = new ArrayBuffer(PARAM_BYTES);
  private pf = new Float32Array(this.paramData);
  private pu = new Uint32Array(this.paramData);
  private vel: GPUTexture[] = [];
  private scl: GPUTexture[] = [];
  private prs: GPUTexture[] = [];
  private div!: GPUTexture;
  private curl!: GPUTexture;
  private occBuffer!: GPUBuffer;
  private dummies: GPUTexture[] = [];
  /** Bind groups per pass; velocity-parity-indexed where a pair is needed. */
  private bg: Record<string, GPUBindGroup[]> = {};
  private velParity = 0;
  /** Sim steps taken since the last reset — the mass-drift/determinism protocols count these. */
  steps = 0;
  /**
   * Jacobi iterations the last step actually dispatched. Read back by the
   * iteration sweep: a tuning knob that does not reach the dispatch loop is
   * indistinguishable from a knob that does nothing, and this tells them apart.
   */
  lastJacobi = 0;

  private constructor(
    private device: GPUDevice,
    private iface: FluidInterface,
    private bake: OccupancyBaker | null,
  ) {}

  static async create(
    device: GPUDevice, iface: FluidInterface, bake: OccupancyBaker | null,
  ): Promise<FluidSim> {
    const sim = new FluidSim(device, iface, bake);
    await sim.init();
    return sim;
  }

  private async init(): Promise<void> {
    const d = this.device;
    const C = GPUShaderStage.COMPUTE;
    const tex3 = (sampleType: GPUTextureSampleType) =>
      ({ sampleType, viewDimension: "3d" as const });
    const st3 = (format: GPUTextureFormat) =>
      ({ access: "write-only" as const, format, viewDimension: "3d" as const });
    this.layout = d.createBindGroupLayout({
      label: "fluid",
      entries: [
        { binding: 0, visibility: C, buffer: { type: "uniform" } },
        { binding: 1, visibility: C, sampler: { type: "filtering" } },
        { binding: 2, visibility: C, texture: tex3("float") },
        { binding: 3, visibility: C, texture: tex3("float") },
        { binding: 4, visibility: C, texture: tex3("unfilterable-float") },
        { binding: 5, visibility: C, texture: tex3("unfilterable-float") },
        { binding: 6, visibility: C, buffer: { type: "read-only-storage" } },
        { binding: 7, visibility: C, storageTexture: st3("rgba16float") },
        { binding: 8, visibility: C, storageTexture: st3("rgba16float") },
        { binding: 9, visibility: C, storageTexture: st3("r32float") },
      ],
    });
    const mod = await createShaderModule(d, "fluid", SHADERS.fluid);
    const pl = d.createPipelineLayout({ label: "fluid", bindGroupLayouts: [this.layout] });
    d.pushErrorScope("validation");
    this.pipes = {};
    for (const ep of [
      "curl", "forces", "advectVel", "divergence", "jacobi", "project",
      "advectScl", "writeVolume",
    ]) {
      this.pipes[ep] = d.createComputePipeline({
        label: `fluid-${ep}`, layout: pl, compute: { module: mod, entryPoint: ep },
      });
    }
    const err = await d.popErrorScope();
    if (err) throw new Error(`fluid pipeline creation failed: ${err.message}`);

    this.sampler = d.createSampler({
      label: "fluid-linear",
      magFilter: "linear", minFilter: "linear",
      addressModeU: "clamp-to-edge", addressModeV: "clamp-to-edge", addressModeW: "clamp-to-edge",
    });
    this.params = d.createBuffer({
      label: "fluid-params", size: PARAM_BYTES,
      usage: GPUBufferUsage.UNIFORM | GPUBufferUsage.COPY_DST,
    });
    const f16 = "rgba16float" as const;
    const dummy = (label: string, format: GPUTextureFormat, usage: number) =>
      d.createTexture({ label, dimension: "3d", size: [1, 1, 1], format, usage });
    // Placeholders for a pass's unused slots. Distinct textures per slot, so
    // no bind group ever names one texture as both an input and an output.
    this.dummies = [
      dummy("fluid-dummy-in-f", f16, GPUTextureUsage.TEXTURE_BINDING),
      dummy("fluid-dummy-in-r", "r32float", GPUTextureUsage.TEXTURE_BINDING),
      dummy("fluid-dummy-out-f0", f16, GPUTextureUsage.STORAGE_BINDING),
      dummy("fluid-dummy-out-f1", f16, GPUTextureUsage.STORAGE_BINDING),
      dummy("fluid-dummy-out-r", "r32float", GPUTextureUsage.STORAGE_BINDING),
    ];
    this.buildGrid(this.scale);
  }

  /**
   * (Re)builds the sim lattice at `scale` (1 = the interface grid, 2 = half
   * in x/z), re-baking occupancy and reallocating every field zeroed. The
   * interface grid never changes; a coarser lattice is resampled into it.
   */
  setScale(scale: number): void {
    this.buildGrid(Math.max(1, Math.floor(scale)));
  }

  /** Zeroes the simulation state (fields reallocated; occupancy kept). */
  reset(): void {
    this.buildGrid(this.scale, true);
  }

  private buildGrid(scale: number, keepOccupancy = false): void {
    const d = this.device;
    const changedScale = scale !== this.scale;
    this.scale = scale;
    const [ix, iy, iz] = this.iface.dims;
    const nx = Math.max(1, Math.round(ix / scale));
    const nz = Math.max(1, Math.round(iz / scale));
    this.dims = [nx, iy, nz];
    const c = this.iface.cell;
    this.cell = [c * (ix / nx), c, c * (iz / nz)];

    for (const t of [...this.vel, ...this.scl, ...this.prs, this.div, this.curl]) t?.destroy();
    const [gx, gy, gz] = this.dims;
    const mk = (label: string, format: GPUTextureFormat, copySrc = false) =>
      d.createTexture({
        label, dimension: "3d", size: [gx, gy, gz], format,
        usage: GPUTextureUsage.STORAGE_BINDING | GPUTextureUsage.TEXTURE_BINDING |
          (copySrc ? GPUTextureUsage.COPY_SRC : 0),
      });
    this.vel = [mk("fluid-vel-0", "rgba16float", true), mk("fluid-vel-1", "rgba16float", true)];
    // scl-1 is copyable so a scenario can weigh the field immediately before
    // and after one advection step (see advectionBalance).
    this.scl = [mk("fluid-scl-0", "rgba16float", true), mk("fluid-scl-1", "rgba16float", true)];
    this.prs = [mk("fluid-prs-0", "r32float"), mk("fluid-prs-1", "r32float")];
    this.div = mk("fluid-div", "r32float", true);
    this.curl = mk("fluid-curl", "rgba16float");
    this.velParity = 0;
    this.steps = 0;
    this.lastJacobi = 0;

    if (!keepOccupancy || changedScale || !this.occBuffer) {
      this.occBuffer?.destroy();
      const cells = gx * gy * gz;
      const occ = this.bake
        ? this.bake(this.dims, this.iface.origin, this.cell)
        : new Uint8Array(cells);
      this.solidCells = 0;
      const words = new Uint32Array(Math.ceil(cells / 4));
      for (let i = 0; i < cells; i++) {
        if (occ[i]) {
          words[i >> 2] |= 1 << ((i & 3) * 8);
          this.solidCells++;
        }
      }
      this.occBuffer = d.createBuffer({
        label: "fluid-occupancy", size: Math.max(16, words.byteLength),
        usage: GPUBufferUsage.STORAGE | GPUBufferUsage.COPY_DST,
      });
      d.queue.writeBuffer(this.occBuffer, 0, words);
    }
    this.buildBindGroups();
  }

  private buildBindGroups(): void {
    const d = this.device;
    const v = (t: GPUTexture) => t.createView({ dimension: "3d" });
    const [dInF, dInR, dOutF0, dOutF1, dOutR] = this.dummies;
    const ifView = this.iface.volume.createView({ dimension: "3d" });
    const mk = (
      label: string,
      texA: GPUTexture | null, texB: GPUTexture | null,
      texC: GPUTexture | null, texD: GPUTexture | null,
      outF0: GPUTextureView | null, outF1: GPUTexture | null,
      outR: GPUTexture | null,
    ): GPUBindGroup =>
      d.createBindGroup({
        label, layout: this.layout,
        entries: [
          { binding: 0, resource: { buffer: this.params } },
          { binding: 1, resource: this.sampler },
          { binding: 2, resource: v(texA ?? dInF) },
          { binding: 3, resource: v(texB ?? dInF) },
          { binding: 4, resource: v(texC ?? dInR) },
          { binding: 5, resource: v(texD ?? dInR) },
          { binding: 6, resource: { buffer: this.occBuffer } },
          { binding: 7, resource: outF0 ?? v(dOutF0) },
          { binding: 8, resource: v(outF1 ?? dOutF1) },
          { binding: 9, resource: v(outR ?? dOutR) },
        ],
      });
    const [scl0, scl1] = this.scl;
    const [prs0, prs1] = this.prs;
    this.bg = {
      curl: [], forces: [], advectVel: [], divergence: [], project: [], advectScl: [],
      // Pressure ping-pong is independent of velocity parity.
      jacobi: [
        mk("fluid-jacobi-0", null, null, prs0, this.div, null, null, prs1),
        mk("fluid-jacobi-1", null, null, prs1, this.div, null, null, prs0),
      ],
      write: [mk("fluid-write", scl0, null, null, null, ifView, null, null)],
    };
    // p indexes which velocity copy is current at the start of a step; the
    // step's passes bounce it cur -> oth -> cur -> oth, so parity flips.
    for (let p = 0; p < 2; p++) {
      const cur = this.vel[p], oth = this.vel[1 - p];
      this.bg.curl.push(mk(`fluid-curl-${p}`, cur, null, null, null, v(this.curl), null, null));
      this.bg.forces.push(
        mk(`fluid-forces-${p}`, cur, scl0, this.curl, null, v(oth), scl1, null),
      );
      this.bg.advectVel.push(mk(`fluid-advectvel-${p}`, oth, null, null, null, v(cur), null, null));
      this.bg.divergence.push(
        mk(`fluid-divergence-${p}`, cur, null, null, null, null, null, this.div),
      );
      this.bg.project.push(mk(`fluid-project-${p}`, cur, null, prs0, null, v(oth), null, null));
      this.bg.advectScl.push(mk(`fluid-advectscl-${p}`, scl1, oth, null, null, v(scl0), null, null));
    }
  }

  /**
   * Encodes one simulation step and the interface write.
   *
   * @param sources packed Source structs (FLUID_SOURCE_STRIDE floats each).
   * @param profile per-pass timestamp writes, so the three passes show up by
   *                name in the GPU profiler.
   */
  step(
    enc: GPUCommandEncoder, dt: number,
    sources: Float32Array, sourceCount: number,
    profile: Profile,
  ): void {
    const n = Math.min(sourceCount, FLUID_MAX_SOURCES);
    this.writeParams(dt, sources, n);
    const p = this.velParity;
    const [nx, ny, nz] = this.dims;
    const gx = Math.ceil(nx / WG), gy = Math.ceil(ny / WG), gz = Math.ceil(nz / WG);
    const run = (pass: GPUComputePassEncoder, pipe: string, bg: GPUBindGroup, x = gx, y = gy, z = gz) => {
      pass.setPipeline(this.pipes[pipe]);
      pass.setBindGroup(0, bg);
      pass.dispatchWorkgroups(x, y, z);
    };

    // Advection group: vorticity, forces + sources, velocity self-advection.
    let cp = enc.beginComputePass({ label: "fluidAdvect", timestampWrites: profile("fluidAdvect") });
    run(cp, "curl", this.bg.curl[p]);
    run(cp, "forces", this.bg.forces[p]);
    run(cp, "advectVel", this.bg.advectVel[p]);
    cp.end();

    // Pressure projection. Jacobi count is even so the converged pressure
    // lands back in prs[0], where project reads it and the next frame warm
    // starts from it.
    cp = enc.beginComputePass({ label: "fluidPressure", timestampWrites: profile("fluidPressure") });
    run(cp, "divergence", this.bg.divergence[p]);
    const iters = Math.max(2, Math.round(this.tune.jacobi / 2) * 2);
    for (let i = 0; i < iters; i++) run(cp, "jacobi", this.bg.jacobi[i & 1]);
    this.lastJacobi = iters;
    run(cp, "project", this.bg.project[p]);
    cp.end();

    // Scalar advection by the projected field, then the interface fill.
    cp = enc.beginComputePass({ label: "fluidScalars", timestampWrites: profile("fluidScalars") });
    run(cp, "advectScl", this.bg.advectScl[p]);
    const [ifx, ify, ifz] = this.iface.dims;
    run(cp, "writeVolume", this.bg.write[0],
      Math.ceil(ifx / WG), Math.ceil(ify / WG), Math.ceil(ifz / WG));
    cp.end();

    this.velParity = 1 - p;
    this.steps++;
  }

  private writeParams(dt: number, sources: Float32Array, count: number): void {
    const f = this.pf, u = this.pu;
    u[0] = this.dims[0]; u[1] = this.dims[1]; u[2] = this.dims[2]; u[3] = count;
    f[4] = this.cell[0]; f[5] = this.cell[1]; f[6] = this.cell[2]; f[7] = dt;
    f[8] = this.iface.origin[0]; f[9] = this.iface.origin[1]; f[10] = this.iface.origin[2];
    f[11] = this.tune.dissipation;
    u[12] = this.iface.dims[0]; u[13] = this.iface.dims[1]; u[14] = this.iface.dims[2];
    f[15] = this.tune.cooling;
    f[16] = this.iface.cell; f[17] = this.iface.cell; f[18] = this.iface.cell;
    f[19] = this.tune.buoyancy;
    f[20] = this.iface.origin[0]; f[21] = this.iface.origin[1]; f[22] = this.iface.origin[2];
    f[23] = this.tune.weight;
    f[24] = this.tune.vorticity;
    f.set(sources.subarray(0, count * FLUID_SOURCE_STRIDE), HEADER_F32);
    this.device.queue.writeBuffer(
      this.params, 0, this.paramData, 0, (HEADER_F32 + count * FLUID_SOURCE_STRIDE) * 4,
    );
  }

  // -------------------------------------------------------------------------
  // Verification readbacks. Diagnostics for the headless harness; each maps
  // a staging buffer, so never call one per frame from the game loop.
  // -------------------------------------------------------------------------

  /** Reads a whole 3D texture back; `bpp` bytes per texel. Rows are padded. */
  private async readTexture(
    tex: GPUTexture, bpp: number,
  ): Promise<{ data: ArrayBuffer; bytesPerRow: number; rows: number }> {
    const [nx, ny, nz] = [tex.width, tex.height, tex.depthOrArrayLayers];
    const bytesPerRow = Math.ceil((nx * bpp) / 256) * 256;
    const size = bytesPerRow * ny * nz;
    const staging = this.device.createBuffer({
      label: "fluid-readback", size,
      usage: GPUBufferUsage.COPY_DST | GPUBufferUsage.MAP_READ,
    });
    const enc = this.device.createCommandEncoder({ label: "fluid-readback" });
    enc.copyTextureToBuffer(
      { texture: tex },
      { buffer: staging, bytesPerRow, rowsPerImage: ny },
      { width: nx, height: ny, depthOrArrayLayers: nz },
    );
    this.device.queue.submit([enc.finish()]);
    try {
      await staging.mapAsync(GPUMapMode.READ);
      const data = staging.getMappedRange().slice(0);
      staging.unmap();
      return { data, bytesPerRow, rows: ny };
    } finally {
      // A rejected map (device loss, teardown) must not leak the staging buffer.
      staging.destroy();
    }
  }

  /**
   * Density integral over the room ("mass", in density-unit x m^3), the peak
   * density, the mass-weighted centroid, the mass per y row (floor row 0 to
   * ceiling row ny-1 — where wall contact happens), and an FNV-1a hash of
   * the raw field — the determinism check compares this hash across two
   * runs of the same script.
   */
  async densityStats(): Promise<{
    mass: number; maxDensity: number; nonzeroCells: number; checksum: string;
    centroid: [number, number, number]; rowMass: number[];
  }> {
    const { data, bytesPerRow, rows } = await this.readTexture(this.scl[0], 8);
    const u16 = new Uint16Array(data);
    const stride = bytesPerRow / 2;
    const [nx, ny, nz] = this.dims;
    let sum = 0, peak = 0, nonzero = 0, h = 0x811c9dc5;
    let mx = 0, my = 0, mz = 0;
    const rowSum = new Float64Array(ny);
    for (let k = 0; k < nz; k++) {
      for (let j = 0; j < ny; j++) {
        const rowBase = (k * rows + j) * stride;
        for (let i = 0; i < nx; i++) {
          const raw = u16[rowBase + i * 4];
          h ^= raw; h = Math.imul(h, 0x01000193) >>> 0;
          if (raw === 0) continue;
          const dens = halfToFloat(raw);
          sum += dens;
          rowSum[j] += dens;
          mx += dens * i; my += dens * j; mz += dens * k;
          nonzero++;
          if (dens > peak) peak = dens;
        }
      }
    }
    const cellVol = this.cell[0] * this.cell[1] * this.cell[2];
    const o = this.iface.origin;
    const w = sum > 0 ? sum : 1;
    return {
      mass: sum * cellVol,
      maxDensity: peak,
      nonzeroCells: nonzero,
      checksum: h.toString(16).padStart(8, "0"),
      centroid: [
        o[0] + (mx / w + 0.5) * this.cell[0],
        o[1] + (my / w + 0.5) * this.cell[1],
        o[2] + (mz / w + 0.5) * this.cell[2],
      ],
      rowMass: Array.from(rowSum, (s) => s * cellVol),
    };
  }

  /**
   * What the solver costs, counted rather than estimated: the bind-group
   * layout's entries by kind (the storage-texture count is the one that has
   * to stay inside SwiftShader's default limit of 4 per stage) and the bytes
   * of every field it owns. The interface volume is B2a's and is excluded.
   */
  resources(): {
    bindings: Record<string, number>; storageTexturesPerStage: number;
    textures: { label: string; bytes: number }[];
    buffers: { label: string; bytes: number }[];
    totalBytes: number; totalMB: number;
  } {
    const [nx, ny, nz] = this.dims;
    const cells = nx * ny * nz;
    const textures = [
      ...this.vel.map((t) => ({ label: t.label, bytes: cells * 8 })),
      ...this.scl.map((t) => ({ label: t.label, bytes: cells * 8 })),
      { label: this.curl.label, bytes: cells * 8 },
      ...this.prs.map((t) => ({ label: t.label, bytes: cells * 4 })),
      { label: this.div.label, bytes: cells * 4 },
    ];
    const buffers = [
      { label: "fluid-occupancy", bytes: this.occBuffer.size },
      { label: "fluid-params", bytes: PARAM_BYTES },
    ];
    const totalBytes =
      textures.reduce((a, t) => a + t.bytes, 0) + buffers.reduce((a, b) => a + b.bytes, 0);
    return {
      bindings: {
        uniformBuffer: 1, sampler: 1, sampledTexture3d: 4,
        readOnlyStorageBuffer: 1, storageTexture3d: 3,
      },
      storageTexturesPerStage: 3,
      textures, buffers, totalBytes,
      totalMB: +(totalBytes / (1024 * 1024)).toFixed(3),
    };
  }

  /**
   * Mass balance across the single advection step the last `step()` ran.
   *
   * The scalar fields are not ping-ponged by parity: every step goes
   * scl0 -(forces: sources, buoyancy bookkeeping, dissipation)-> scl1
   * -(advectScl)-> scl0. So after a step scl1 still holds exactly what
   * advection was handed and scl0 what it produced, and the difference of
   * their integrals is the advection scheme's own mass defect for that step —
   * no source, dissipation or fp16 write-back mixed in. Per y row as well as
   * total, because a leak that lives in one cell layer against a solid looks
   * like a diffuse global leak in the total alone.
   */
  async advectionBalance(): Promise<{
    before: number; after: number; defect: number; relDefect: number;
    rowBefore: number[]; rowAfter: number[]; rowDefect: number[];
  }> {
    const [nx, ny, nz] = this.dims;
    const cellVol = this.cell[0] * this.cell[1] * this.cell[2];
    const weigh = async (tex: GPUTexture) => {
      const { data, bytesPerRow, rows } = await this.readTexture(tex, 8);
      const u16 = new Uint16Array(data);
      const stride = bytesPerRow / 2;
      const row = new Float64Array(ny);
      for (let k = 0; k < nz; k++) {
        for (let j = 0; j < ny; j++) {
          const base = (k * rows + j) * stride;
          for (let i = 0; i < nx; i++) {
            const raw = u16[base + i * 4];
            if (raw !== 0) row[j] += halfToFloat(raw);
          }
        }
      }
      return Array.from(row, (v) => v * cellVol);
    };
    const rowBefore = await weigh(this.scl[1]);
    const rowAfter = await weigh(this.scl[0]);
    const sum = (a: number[]) => a.reduce((x, y) => x + y, 0);
    const before = sum(rowBefore), after = sum(rowAfter);
    return {
      before, after, defect: after - before,
      relDefect: before > 0 ? (after - before) / before : 0,
      rowBefore, rowAfter,
      rowDefect: rowAfter.map((v, j) => v - rowBefore[j]),
    };
  }

  /**
   * Projection quality on the current step. `pre*` = |div| of the field the
   * Jacobi solve was handed (the div texture still holds that RHS), `post*`
   * = |div| of the projected velocity now current, both in 1/s. Whole-room
   * means are dominated by still air, so the `active*` figures restrict to
   * cells moving faster than ACTIVE_SPEED and give the residual relative to
   * that flow (activeVelRms / cell is the natural divergence unit there).
   * Recomputes divergence into the div texture (harmless: the next step
   * overwrites it).
   */
  async divergenceStats(): Promise<{
    preMaxAbsDiv: number; preMeanAbsDiv: number;
    maxAbsDiv: number; meanAbsDiv: number; meanReduction: number;
    activeCells: number; activeVelRms: number;
    activePreMean: number; activePostMean: number;
    activePreMax: number; activePostMax: number;
    activeRelResidual: number; activeRelResidualMax: number;
  }> {
    const ACTIVE_SPEED = 0.05;
    const p = this.velParity;
    const [nx, ny, nz] = this.dims;
    const n = nx * ny * nz;
    const flatten = (r: { data: ArrayBuffer; bytesPerRow: number; rows: number }) => {
      const f32 = new Float32Array(r.data);
      const stride = r.bytesPerRow / 4;
      const out = new Float32Array(n);
      let t = 0;
      for (let k = 0; k < nz; k++) {
        for (let j = 0; j < ny; j++) {
          const base = (k * r.rows + j) * stride;
          for (let i = 0; i < nx; i++) out[t++] = f32[base + i];
        }
      }
      return out;
    };

    // Pre-projection divergence: the RHS the last step's solve left behind.
    const pre = flatten(await this.readTexture(this.div, 4));

    const enc = this.device.createCommandEncoder({ label: "fluid-div-stats" });
    const cp = enc.beginComputePass({ label: "fluid-div-stats" });
    cp.setPipeline(this.pipes.divergence);
    cp.setBindGroup(0, this.bg.divergence[p]);
    cp.dispatchWorkgroups(Math.ceil(nx / WG), Math.ceil(ny / WG), Math.ceil(nz / WG));
    cp.end();
    this.device.queue.submit([enc.finish()]);
    const post = flatten(await this.readTexture(this.div, 4));

    const vv = await this.readTexture(this.vel[p], 8);
    const vu16 = new Uint16Array(vv.data);
    const vstride = vv.bytesPerRow / 2;
    let preMax = 0, preSum = 0, postMax = 0, postSum = 0;
    let aCells = 0, aVsq = 0, aPre = 0, aPost = 0, aPreMax = 0, aPostMax = 0;
    let t = 0;
    for (let k = 0; k < nz; k++) {
      for (let j = 0; j < ny; j++) {
        const base = (k * vv.rows + j) * vstride;
        for (let i = 0; i < nx; i++, t++) {
          const o = base + i * 4;
          const vx = halfToFloat(vu16[o]), vy = halfToFloat(vu16[o + 1]), vz = halfToFloat(vu16[o + 2]);
          const s2 = vx * vx + vy * vy + vz * vz;
          const a = Math.abs(pre[t]), b = Math.abs(post[t]);
          if (a > preMax) preMax = a;
          if (b > postMax) postMax = b;
          preSum += a; postSum += b;
          if (s2 > ACTIVE_SPEED * ACTIVE_SPEED) {
            aCells++; aVsq += s2; aPre += a; aPost += b;
            if (a > aPreMax) aPreMax = a;
            if (b > aPostMax) aPostMax = b;
          }
        }
      }
    }
    const activeVelRms = Math.sqrt(aVsq / Math.max(aCells, 1));
    const activePost = aPost / Math.max(aCells, 1);
    const scale = activeVelRms / Math.min(this.cell[0], this.cell[1], this.cell[2]);
    return {
      preMaxAbsDiv: preMax,
      preMeanAbsDiv: preSum / n,
      maxAbsDiv: postMax,
      meanAbsDiv: postSum / n,
      meanReduction: postSum > 0 ? preSum / postSum : 0,
      activeCells: aCells,
      activeVelRms,
      activePreMean: aPre / Math.max(aCells, 1),
      activePostMean: activePost,
      activePreMax: aPreMax,
      activePostMax: aPostMax,
      activeRelResidual: scale > 0 ? activePost / scale : 0,
      activeRelResidualMax: scale > 0 ? aPostMax / scale : 0,
    };
  }
}
