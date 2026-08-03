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
  /**
   * Pressure-projection Jacobi iterations per step. Rounded to the nearest
   * even count with a floor of 2, so the converged pressure lands back in
   * prs[0] — an odd or fractional request is *not* rounded up (4.4 -> 4,
   * 5 -> 4). `lastJacobi` reports what actually ran; compare against that
   * rounding, never against the raw request.
   */
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
  // From the measured residual curve (fluid-jacobi), not from taste. On a
  // sustained jet the active-cell relative residual falls as ~1/N with no
  // plateau: 4 -> 1.07e-2, 10 -> 3.92e-3, 20 -> 1.64e-3, 40 -> 7.97e-4,
  // 80 -> 3.87e-4. The knee is where projection error stops dominating, not
  // where the curve bends. What leftover divergence costs the density field per
  // step is dt times the ABSOLUTE active-cell mean |div v| (not the relative
  // figure above): 0.05 s * 0.0032/s = 1.6e-4 at 20 iterations, against the
  // advection scheme's own 1e-3..3e-3. So 20 buys an order of margin, and
  // everything above it is real in the residual but unmeasurable in mass —
  // 8 and 200 iterations retain within 0.24% of each other over 8 s.
  jacobi: 20,
  vorticity: 1.6,
  buoyancy: 1.4,
  weight: 0.045,
  // Models a 7.7 s e-folding, and that is the rate the settled field really
  // follows: measured 0.1227/s over 8-20 s against this 0.130. What the model
  // leaves out is a one-off ~24% deficit the advection scheme takes while the
  // cloud is still billowing in the first ~3 s, so mass tracks
  // 0.76 * exp(-0.13 t) and crosses 1/e of what was injected at 5.3 s rather
  // than 7.7 s — 5.3 s is log-linear interpolation of the measured mass table
  // (9.042 at 3 s, 4.289 at 8 s, target 6.380); the 0.76 * exp(-0.13 t) model
  // solves to 5.6 s. Left as it is deliberately: the deficit is front-loaded, so
  // lowering this rate to absorb it would leave the late haze hanging around
  // too long. See the track report's mass and lifetime tables.
  dissipation: 0.13,
  cooling: 0.6,
};

/** Sources per step and the float stride of one (matches the WGSL Source struct). */
export const FLUID_MAX_SOURCES = 32;
export const FLUID_SOURCE_STRIDE = 12;

// 48, not 28: the tail carries the second-instance block (see FluidParams in
// fluid.wgsl). 48 f32 = 192 B, a multiple of 16, so the Source array's
// alignment and stride are unchanged and `f.set(sources, HEADER_F32)` still
// lands where the shader expects it.
const HEADER_F32 = 48;
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
  /**
   * Solid cells per y row of the last bake. Row 0 is the lattice's bottom row —
   * the air just above the floor plate, not the plate itself, which lies below
   * the lattice; a few thousand of its cells are solid where geometry stands.
   *
   * The top row is fluid by design and that is a documented deviation from the
   * density contract's "treat y >= 3.2 as your solid top wall": the ceiling
   * slab overlaps only the top 5 cm of that row, so the bake skips it rather
   * than solidify 25 cm of real air. See `bakeOccupancy` and the track report.
   */
  solidRow: number[] = [];

  private layout!: GPUBindGroupLayout;
  /** The layout's entry descriptors, kept so `resources` can count them. */
  private layoutEntries!: GPUBindGroupLayoutEntry[];
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
  /** MacCormack's forward-advected intermediate — see advectSclFwd/Mac. */
  private sclHat!: GPUTexture;
  private occBuffer!: GPUBuffer;
  /** The bake, kept CPU-side so a scenario can ask whether a point is solid. */
  private occCpu: Uint8Array<ArrayBufferLike> = new Uint8Array(0);
  private dummies: GPUTexture[] = [];
  /** Bind groups per pass; velocity-parity-indexed where a pair is needed. */
  private bg: Record<string, GPUBindGroup[]> = {};
  /**
   * World origin of the SIM lattice, which is no longer necessarily the
   * interface's. They coincide on the coarse global lattice and diverge for a
   * fine local one, which sits wherever its event put it and writes no
   * interface of its own.
   */
  simOrigin: [number, number, number] = [0, 0, 0];
  /**
   * Where the next buildGrid should place this lattice. buildGrid rewrites
   * simOrigin, so a lattice that moves has to declare its destination before
   * resetting rather than assigning simOrigin after — the bake happens inside
   * buildGrid and would otherwise be taken at the old position.
   */
  nextOrigin: [number, number, number] | null = null;
  /**
   * Level-wide bit-packed occupancy, owned by the caller and shared. Set on a
   * fine lattice; null on the coarse one, which bakes its own byte field.
   */
  sharedOcc: GPUBuffer | null = null;
  /**
   * Whether this lattice fills the shared interface texture. False on a fine
   * lattice: it has no interface of its own, the tracer samples its density
   * field directly, and dispatching writeVolume would have it stamp its small
   * box over the coarse lattice's level-wide interface.
   */
  writesInterface = true;
  /**
   * A finer lattice whose result this one restricts into itself. Set on the
   * coarse lattice; null on the fine one, which restricts into nothing.
   */
  peerDensity: GPUTexture | null = null;
  /** Second-instance state; the defaults are "coarse global lattice". */
  occMode = 0;
  openFaces = 0;
  peerMode = 0;
  occOffset: [number, number, number] = [0, 0, 0];
  occDims: [number, number, number] = [0, 0, 0];
  peerOrigin: [number, number, number] = [0, 0, 0];
  peerCell: [number, number, number] = [0, 0, 0];
  peerDims: [number, number, number] = [0, 0, 0];
  rimCells = 0;
  /**
   * Bumped whenever buildGrid reallocates the field textures. Anything holding
   * a bind group over them (the tracer does) must compare and rebuild — a
   * reset() otherwise leaves it pointing at destroyed textures.
   */
  generation = 0;

  /** The settled density field — .x is density. What the tracer samples on a fine lattice. */
  get densityTexture(): GPUTexture { return this.scl[0]; }

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
    this.layoutEntries = [
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
    ];
    this.layout = d.createBindGroupLayout({ label: "fluid", entries: this.layoutEntries });
    const mod = await createShaderModule(d, "fluid", SHADERS.fluid);
    const pl = d.createPipelineLayout({ label: "fluid", bindGroupLayouts: [this.layout] });
    d.pushErrorScope("validation");
    this.pipes = {};
    for (const ep of [
      "curl", "forces", "advectVel", "divergence", "jacobi", "project",
      "advectSclFwd", "advectSclMac", "writeVolume",
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

    for (const t of [...this.vel, ...this.scl, ...this.prs,
                     this.sclHat, this.div, this.curl]) t?.destroy();
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
    // Pressure is copyable so `pressureStats` can watch the warm start: pure
    // Neumann walls leave the solve singular, and a warm-started singular solve
    // is where a null-space mode would accumulate if one did.
    this.prs = [mk("fluid-prs-0", "r32float", true), mk("fluid-prs-1", "r32float", true)];
    // MacCormack's intermediate: the forward-advected field, which the
    // corrector must gather from, so it has to be materialised rather than
    // recomputed per cell.
    this.sclHat = mk("fluid-scl-hat", "rgba16float", true);
    this.div = mk("fluid-div", "r32float", true);
    this.curl = mk("fluid-curl", "rgba16float");
    this.simOrigin = this.nextOrigin
      ? [...this.nextOrigin]
      : [this.iface.origin[0], this.iface.origin[1], this.iface.origin[2]];
    this.generation++;
    this.velParity = 0;
    this.steps = 0;
    this.lastJacobi = 0;

    if (this.sharedOcc) {
      // A fine local lattice bakes nothing. It reads a window into a level-wide
      // bit field, which is baked once and shared, because the lattice moves
      // and re-baking per move is exactly the cost this design exists to avoid.
      // Do NOT destroy it here — this instance does not own it.
      this.occBuffer = this.sharedOcc;
      this.solidCells = 0;
      this.solidRow = new Array<number>(gy).fill(0);
      this.occCpu = new Uint8Array(0);
    } else if (!keepOccupancy || changedScale || !this.occBuffer || this.nextOrigin) {
      this.occBuffer?.destroy();
      const cells = gx * gy * gz;
      const occ = this.bake
        ? this.bake(this.dims, this.simOrigin, this.cell)
        : new Uint8Array(cells);
      this.solidCells = 0;
      this.occCpu = occ;
      const rows = new Array<number>(gy).fill(0);
      const words = new Uint32Array(Math.ceil(cells / 4));
      for (let i = 0; i < cells; i++) {
        if (occ[i]) {
          words[i >> 2] |= 1 << ((i & 3) * 8);
          this.solidCells++;
          rows[Math.floor(i / gx) % gy]++;
        }
      }
      this.solidRow = rows;
      this.occBuffer = d.createBuffer({
        label: "fluid-occupancy", size: Math.max(16, words.byteLength),
        usage: GPUBufferUsage.STORAGE | GPUBufferUsage.COPY_DST,
      });
      d.queue.writeBuffer(this.occBuffer, 0, words);
    }
    this.buildBindGroups();
  }

  /** Re-create the bind groups after a peer texture is attached. */
  rebuildBindGroups(): void { this.buildBindGroups(); }

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
      curl: [], forces: [], advectVel: [], divergence: [], divergenceScratch: [],
      project: [], advectSclFwd: [], advectSclMac: [],
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
      // `divergenceStats` re-runs the divergence kernel to see the projected
      // field, and must not overwrite the RHS the solve was handed. prs1 is
      // free between steps: the next step's first Jacobi iteration reads prs0
      // and writes prs1, so nothing ever reads what is left here.
      this.bg.divergenceScratch.push(
        mk(`fluid-divergence-scratch-${p}`, cur, null, null, null, null, null, prs1),
      );
      this.bg.project.push(mk(`fluid-project-${p}`, cur, null, prs0, null, v(oth), null, null));
      // Forward writes phi_hat; the corrector reads phi^n (texA), the velocity
      // (texB) and phi_hat (texC), and writes the final field.
      this.bg.advectSclFwd.push(
        mk(`fluid-advectscl-fwd-${p}`, scl1, oth, null, null, v(this.sclHat), null, null),
      );
      // texD carries the fine peer's density when one is set, which is what
      // the restriction in advectSclMac reads. Without a peer it is a dummy,
      // exactly as before.
      this.bg.advectSclMac.push(
        mk(`fluid-advectscl-mac-${p}`, scl1, oth, this.sclHat,
           this.peerDensity ?? null, v(scl0), null, null),
      );
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
    run(cp, "advectSclFwd", this.bg.advectSclFwd[p]);
    run(cp, "advectSclMac", this.bg.advectSclMac[p]);
    if (this.writesInterface) {
      const [ifx, ify, ifz] = this.iface.dims;
      run(cp, "writeVolume", this.bg.write[0],
        Math.ceil(ifx / WG), Math.ceil(ify / WG), Math.ceil(ifz / WG));
    }
    cp.end();

    this.velParity = 1 - p;
    this.steps++;
  }

  private writeParams(dt: number, sources: Float32Array, count: number): void {
    const f = this.pf, u = this.pu;
    u[0] = this.dims[0]; u[1] = this.dims[1]; u[2] = this.dims[2]; u[3] = count;
    f[4] = this.cell[0]; f[5] = this.cell[1]; f[6] = this.cell[2]; f[7] = dt;
    f[8] = this.simOrigin[0]; f[9] = this.simOrigin[1]; f[10] = this.simOrigin[2];
    f[11] = this.tune.dissipation;
    u[12] = this.iface.dims[0]; u[13] = this.iface.dims[1]; u[14] = this.iface.dims[2];
    f[15] = this.tune.cooling;
    f[16] = this.iface.cell; f[17] = this.iface.cell; f[18] = this.iface.cell;
    f[19] = this.tune.buoyancy;
    f[20] = this.iface.origin[0]; f[21] = this.iface.origin[1]; f[22] = this.iface.origin[2];
    f[23] = this.tune.weight;
    f[24] = this.tune.vorticity;
    // Second-instance block. Zero on the coarse lattice.
    u[25] = this.occMode;
    u[26] = this.openFaces;
    u[27] = this.peerMode;
    u[28] = this.occOffset[0] >>> 0; u[29] = this.occOffset[1] >>> 0;
    u[30] = this.occOffset[2] >>> 0;
    f[31] = this.rimCells;
    u[32] = this.occDims[0]; u[33] = this.occDims[1]; u[34] = this.occDims[2];
    f[36] = this.peerOrigin[0]; f[37] = this.peerOrigin[1]; f[38] = this.peerOrigin[2];
    f[40] = this.peerCell[0]; f[41] = this.peerCell[1]; f[42] = this.peerCell[2];
    u[44] = this.peerDims[0]; u[45] = this.peerDims[1]; u[46] = this.peerDims[2];
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
   * density, the mass-weighted centroid, the mass per y row (row 0 is the air
   * just above the floor, row ny-1 the row under the ceiling — the two rows
   * where wall contact happens), and two FNV-1a hashes the determinism check
   * compares across runs of the same script.
   *
   * Be exact about what each hash covers, because "the checksums matched" is
   * only as strong as the bytes hashed. `checksum` is the **density lane
   * alone** (R of scl[0]) — it is the long-lived figure the report's
   * determinism table quotes. `fieldChecksum` covers all four lanes of the
   * same texture, so it also sees **temperature**; B and A are written zero.
   * Neither sees velocity, pressure, divergence or curl, so a drift confined
   * to those and not yet propagated into the scalars passes both. Velocity is
   * read separately by `divergenceStats`.
   */
  async densityStats(threshold = 0.05): Promise<{
    mass: number; maxDensity: number; nonzeroCells: number;
    checksum: string; fieldChecksum: string;
    centroid: [number, number, number]; rowMass: number[];
    rowCells: number[]; visibleCells: number;
    bbox: { min: [number, number, number]; max: [number, number, number] } | null;
  }> {
    const { data, bytesPerRow, rows } = await this.readTexture(this.scl[0], 8);
    const u16 = new Uint16Array(data);
    const stride = bytesPerRow / 2;
    const [nx, ny, nz] = this.dims;
    let sum = 0, peak = 0, nonzero = 0, h = 0x811c9dc5, hAll = 0x811c9dc5;
    let mx = 0, my = 0, mz = 0;
    const rowSum = new Float64Array(ny);
    const rowCells = new Int32Array(ny);
    // Extent of the cells a viewer would actually see, so "the cloud spread
    // along the floor" is a measured box and not an impression of a still.
    let visible = 0;
    let bi = nx, bj = ny, bk = nz, ti = -1, tj = -1, tk = -1;
    for (let k = 0; k < nz; k++) {
      for (let j = 0; j < ny; j++) {
        const rowBase = (k * rows + j) * stride;
        for (let i = 0; i < nx; i++) {
          const o = rowBase + i * 4;
          const raw = u16[o];
          h ^= raw; h = Math.imul(h, 0x01000193) >>> 0;
          for (let lane = 0; lane < 4; lane++) {
            hAll ^= u16[o + lane]; hAll = Math.imul(hAll, 0x01000193) >>> 0;
          }
          if (raw === 0) continue;
          const dens = halfToFloat(raw);
          sum += dens;
          rowSum[j] += dens;
          mx += dens * i; my += dens * j; mz += dens * k;
          nonzero++;
          if (dens > peak) peak = dens;
          if (dens >= threshold) {
            visible++; rowCells[j]++;
            if (i < bi) bi = i; if (i > ti) ti = i;
            if (j < bj) bj = j; if (j > tj) tj = j;
            if (k < bk) bk = k; if (k > tk) tk = k;
          }
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
      fieldChecksum: hAll.toString(16).padStart(8, "0"),
      centroid: [
        o[0] + (mx / w + 0.5) * this.cell[0],
        o[1] + (my / w + 0.5) * this.cell[1],
        o[2] + (mz / w + 0.5) * this.cell[2],
      ],
      rowMass: Array.from(rowSum, (s) => s * cellVol),
      rowCells: Array.from(rowCells),
      visibleCells: visible,
      bbox: ti < 0 ? null : {
        min: [o[0] + bi * this.cell[0], o[1] + bj * this.cell[1], o[2] + bk * this.cell[2]],
        max: [
          o[0] + (ti + 1) * this.cell[0],
          o[1] + (tj + 1) * this.cell[1],
          o[2] + (tk + 1) * this.cell[2],
        ],
      },
    };
  }

  /**
   * Density along a ray through the simulation field: the samples, the column
   * integral (density-unit x m) and the optical depth for a given extinction
   * per unit density. Read from the solver's own lattice, trilinearly — the
   * same reconstruction the shader's sampler performs — rather than from the
   * quarter-resolution gameplay readback, whose box average smooths a compact
   * cloud by more than an order of magnitude.
   *
   * This is the SMOKE field only. The renderer's medium is
   * `mediumDensity = densityStatic + smokeDensity`, and `densityStatic` is the
   * drifting fog at mean `settings.fogAmount` (0.55 by default), so a tau from
   * here understates the rendered optical depth by sigma_t * fogAmount * path
   * — about 0.03/m at the default. Ratios along one ray are unaffected;
   * absolute transmittance quoted for the screen is not.
   */
  async columnDensity(
    from: [number, number, number], dir: [number, number, number],
    length: number, step = 0.1, sigmaPerUnitDensity = 0.05,
  ): Promise<{
    samples: { s: number; density: number; tau: number; transmittance: number }[];
    integral: number; tau: number; peak: number;
    halfAt: number | null; tenthAt: number | null;
  }> {
    const sample = await this.densitySampler();
    const n = Math.max(1, Math.round(length / step));
    const samples = [];
    let integral = 0, tau = 0, peak = 0;
    let halfAt: number | null = null, tenthAt: number | null = null;
    for (let q = 1; q <= n; q++) {
      const s = q * step;
      const d = sample(from[0] + dir[0] * s, from[1] + dir[1] * s, from[2] + dir[2] * s);
      if (d > peak) peak = d;
      integral += d * step;
      tau += sigmaPerUnitDensity * d * step;
      const T = Math.exp(-tau);
      if (halfAt === null && T <= 0.5) halfAt = s;
      if (tenthAt === null && T <= 0.1) tenthAt = s;
      samples.push({ s, density: d, tau, transmittance: T });
    }
    return { samples, integral, tau, peak, halfAt, tenthAt };
  }

  /** Density of the simulated field at a list of world points, one read. */
  async densitySamples(points: [number, number, number][]): Promise<number[]> {
    const sample = await this.densitySampler();
    return points.map((p) => sample(p[0], p[1], p[2]));
  }

  /**
   * Reads the density field once and returns a trilinear point sampler over
   * it. Cell-centre convention, no smoke outside the lattice — the same
   * reconstruction the shader's linear sampler performs.
   */
  private async densitySampler(): Promise<(x: number, y: number, z: number) => number> {
    const { data, bytesPerRow, rows } = await this.readTexture(this.scl[0], 8);
    const u16 = new Uint16Array(data);
    const stride = bytesPerRow / 2;
    const [nx, ny, nz] = this.dims;
    const o = this.iface.origin;
    const at = (i: number, j: number, k: number) =>
      halfToFloat(u16[(k * rows + j) * stride + i * 4]);
    // Cell-centre trilinear sample; outside the lattice reads as no smoke.
    const sample = (x: number, y: number, z: number): number => {
      const gx = (x - o[0]) / this.cell[0] - 0.5;
      const gy = (y - o[1]) / this.cell[1] - 0.5;
      const gz = (z - o[2]) / this.cell[2] - 0.5;
      if (gx < -0.5 || gy < -0.5 || gz < -0.5) return 0;
      if (gx > nx - 0.5 || gy > ny - 0.5 || gz > nz - 0.5) return 0;
      const i0 = Math.max(0, Math.min(nx - 2, Math.floor(gx)));
      const j0 = Math.max(0, Math.min(ny - 2, Math.floor(gy)));
      const k0 = Math.max(0, Math.min(nz - 2, Math.floor(gz)));
      const fx = Math.min(1, Math.max(0, gx - i0));
      const fy = Math.min(1, Math.max(0, gy - j0));
      const fz = Math.min(1, Math.max(0, gz - k0));
      const lerp = (a: number, b: number, t: number) => a + (b - a) * t;
      const y0 = lerp(lerp(at(i0, j0, k0), at(i0 + 1, j0, k0), fx),
                      lerp(at(i0, j0 + 1, k0), at(i0 + 1, j0 + 1, k0), fx), fy);
      const y1 = lerp(lerp(at(i0, j0, k0 + 1), at(i0 + 1, j0, k0 + 1), fx),
                      lerp(at(i0, j0 + 1, k0 + 1), at(i0 + 1, j0 + 1, k0 + 1), fx), fy);
      return lerp(y0, y1, fz);
    };
    return sample;
  }

  /**
   * Is the lattice cell containing a world point solid? The obstacle check
   * needs this: "the plume went around the column" is only a claim about the
   * simulation if the column's cells are the ones the bake calls solid.
   * Outside the lattice counts as solid, matching the shader's walls.
   */
  solidAtWorld(x: number, y: number, z: number): boolean {
    const o = this.iface.origin;
    const [nx, ny, nz] = this.dims;
    const i = Math.floor((x - o[0]) / this.cell[0]);
    const j = Math.floor((y - o[1]) / this.cell[1]);
    const k = Math.floor((z - o[2]) / this.cell[2]);
    if (i < 0 || j < 0 || k < 0 || i >= nx || j >= ny || k >= nz) return true;
    return this.occCpu[(k * ny + j) * nx + i] !== 0;
  }

  /**
   * What the solver costs, counted rather than estimated: the bind-group
   * layout's own entry descriptors tallied by kind (the storage-texture count
   * is the one that has to stay inside SwiftShader's default limit of 4 per
   * stage) and the bytes of every field it owns. The interface volume is B2a's
   * and is excluded.
   *
   * `storageTexturesPerStage` equals the total because every entry here is
   * compute-only; it is derived from the same entries, so adding a fourth
   * storage texture moves this number instead of leaving it stale.
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
      // MacCormack's forward field. It was missing here, so the report read
      // 52 B/cell against 60 B/cell actually allocated — an 8 B/cell blind
      // spot that grows with every lattice this thing is ever pointed at.
      { label: this.sclHat.label, bytes: cells * 8 },
      ...this.prs.map((t) => ({ label: t.label, bytes: cells * 4 })),
      { label: this.div.label, bytes: cells * 4 },
    ];
    const buffers = [
      { label: "fluid-occupancy", bytes: this.occBuffer.size },
      { label: "fluid-params", bytes: PARAM_BYTES },
    ];
    const totalBytes =
      textures.reduce((a, t) => a + t.bytes, 0) + buffers.reduce((a, b) => a + b.bytes, 0);
    const bindings: Record<string, number> = {};
    let storageTextures = 0;
    for (const e of this.layoutEntries) {
      let kind: string;
      if (e.buffer) {
        kind = e.buffer.type === "uniform" ? "uniformBuffer"
          : e.buffer.type === "read-only-storage" ? "readOnlyStorageBuffer"
            : "storageBuffer";
      } else if (e.sampler) {
        kind = "sampler";
      } else if (e.storageTexture) {
        kind = "storageTexture3d";
        storageTextures++;
      } else if (e.texture) {
        kind = "sampledTexture3d";
      } else {
        kind = "unknown";
      }
      bindings[kind] = (bindings[kind] ?? 0) + 1;
    }
    return {
      bindings,
      storageTexturesPerStage: storageTextures,
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
    hat: number; relGather: number; relCorrect: number;
    rowBefore: number[]; rowAfter: number[]; rowDefect: number[];
    rowHat: number[];
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
    // sclHat is the FORWARD-ONLY field: one plain semi-Lagrangian gather, with
    // no MacCormack correction, no limiter and no volume factor applied yet.
    // Weighing it splits the step's mass defect into the part the gather itself
    // commits and the part the correct/limit/scale stage adds on top, which is
    // the only way to tell a limiter that discards mass from a gather that
    // never conserved it. Both stages are otherwise invisible in `after`.
    const rowHat = await weigh(this.sclHat);
    const sum = (a: number[]) => a.reduce((x, y) => x + y, 0);
    const before = sum(rowBefore), after = sum(rowAfter), hat = sum(rowHat);
    return {
      before, after, defect: after - before,
      relDefect: before > 0 ? (after - before) / before : 0,
      hat,
      relGather: before > 0 ? (hat - before) / before : 0,
      relCorrect: hat > 0 ? (after - hat) / hat : 0,
      rowBefore, rowAfter, rowHat,
      rowDefect: rowAfter.map((v, j) => v - rowBefore[j]),
    };
  }

  /**
   * The pressure field the last solve produced, over fluid cells only.
   *
   * Closed room, every wall Neumann: the pressure Poisson system is singular,
   * so its solution is only defined up to an additive constant, and `step`
   * warm-starts each frame from the previous frame's pressure. That pairing is
   * the one worth watching — if the discrete right-hand side is not exactly
   * compatible (fp16 velocity means it is not), Jacobi feeds the mismatch into
   * the null space, where it shows up as a drifting mean with no effect on the
   * gradient until the drift is large enough to cost the neighbour difference
   * its significant digits. `mean` against `spread` is the ratio that answers
   * it; `steps` says how much warm starting has accumulated so far.
   */
  async pressureStats(): Promise<{
    steps: number; cells: number; mean: number; min: number; max: number;
    spread: number; absMax: number; meanOverSpread: number;
  }> {
    const { data, bytesPerRow, rows } = await this.readTexture(this.prs[0], 4);
    const f32 = new Float32Array(data);
    const stride = bytesPerRow / 4;
    const [nx, ny, nz] = this.dims;
    let sum = 0, n = 0, lo = Infinity, hi = -Infinity, absMax = 0;
    for (let k = 0; k < nz; k++) {
      for (let j = 0; j < ny; j++) {
        const base = (k * rows + j) * stride;
        for (let i = 0; i < nx; i++) {
          // Solid cells are stored zero by every writer; including them would
          // drag the mean toward zero and hide exactly the drift being measured.
          if (this.occCpu[(k * ny + j) * nx + i] !== 0) continue;
          const p = f32[base + i];
          sum += p; n++;
          if (p < lo) lo = p;
          if (p > hi) hi = p;
          if (Math.abs(p) > absMax) absMax = Math.abs(p);
        }
      }
    }
    const mean = n > 0 ? sum / n : 0;
    const spread = n > 0 ? hi - lo : 0;
    return {
      steps: this.steps, cells: n, mean, min: lo, max: hi, spread, absMax,
      meanOverSpread: spread > 0 ? Math.abs(mean) / spread : 0,
    };
  }

  /**
   * Projection quality on the current step. `pre*` = |div| of the field the
   * Jacobi solve was handed (the div texture still holds that RHS), `post*`
   * = |div| of the projected velocity now current, both in 1/s. Whole-room
   * means are dominated by still air, so the `active*` figures restrict to
   * cells moving faster than ACTIVE_SPEED and give the residual relative to
   * that flow (activeVelRms / cell is the natural divergence unit there).
   *
   * The recomputed post-projection divergence goes to the spare pressure
   * texture, not back into `div`: `div` still holds the solve's RHS, which is
   * what `pre*` reads, so calling this twice at one checkpoint reports the
   * same `pre` figures rather than silently reporting post values as pre.
   *
   * Every `active*` figure is null when no cell is moving faster than
   * `activeSpeed`, because a mean over an empty set reported as 0 reads as a
   * perfect residual when it means "nothing was measured" — a settled cloud
   * spreading by numerical diffusion alone has no active cells at all.
   */
  async divergenceStats(activeSpeed = 0.05): Promise<{
    preMaxAbsDiv: number; preMeanAbsDiv: number;
    maxAbsDiv: number; meanAbsDiv: number; meanReduction: number;
    activeSpeed: number; activeCells: number; activeVelRms: number | null;
    activePreMean: number | null; activePostMean: number | null;
    activePreMax: number | null; activePostMax: number | null;
    activeRelResidual: number | null; activeRelResidualMax: number | null;
  }> {
    const ACTIVE_SPEED = activeSpeed;
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
    cp.setBindGroup(0, this.bg.divergenceScratch[p]);
    cp.dispatchWorkgroups(Math.ceil(nx / WG), Math.ceil(ny / WG), Math.ceil(nz / WG));
    cp.end();
    this.device.queue.submit([enc.finish()]);
    const post = flatten(await this.readTexture(this.prs[1], 4));

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
    const none = aCells === 0;
    const activeVelRms = none ? null : Math.sqrt(aVsq / aCells);
    const activePost = none ? null : aPost / aCells;
    const scale = activeVelRms === null
      ? 0
      : activeVelRms / Math.min(this.cell[0], this.cell[1], this.cell[2]);
    const rel = (v: number) => (scale > 0 ? v / scale : null);
    return {
      preMaxAbsDiv: preMax,
      preMeanAbsDiv: preSum / n,
      maxAbsDiv: postMax,
      meanAbsDiv: postSum / n,
      meanReduction: postSum > 0 ? preSum / postSum : 0,
      activeSpeed: ACTIVE_SPEED,
      activeCells: aCells,
      activeVelRms,
      activePreMean: none ? null : aPre / aCells,
      activePostMean: activePost,
      activePreMax: none ? null : aPreMax,
      activePostMax: none ? null : aPostMax,
      activeRelResidual: activePost === null ? null : rel(activePost),
      activeRelResidualMax: none ? null : rel(aPostMax),
    };
  }
}
