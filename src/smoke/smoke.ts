// Multi-domain GPU smoke simulation (see smoke.wgsl). Exposes a SmokeBinding for the renderer.
import { makeBuffer, makeShader } from '../gpu/device';
import { Vec3, v3 } from '../math/vec';
import { Engine, SmokeBinding } from '../engine';
import { wgslWorldConsts, WORLD } from '../scene/world';
import { SmokeEmitter, SmokePush, SmokeSystemLike } from './types';
import smokeSrc from './smoke.wgsl' with { type: 'text' };
import halfSrc from './pressure_half.wgsl' with { type: 'text' };

export interface SmokeParams {
  buoyancy: number; weight: number; vortConf: number; velDamp: number; densDecay: number; tempDecay: number; turbulence: number;
  windX: number; windZ: number; densityScale: number; enabled: boolean; voxelFine: number; voxelCoarse: number;
  /** pressure solve: red-black Gauss-Seidel + SOR — rbIters red/black sweep pairs per step, relaxed by omega */
  rbIters: number; omega: number;
  /** LOSSY, default off: solve the pressure on a 32³ grid per domain instead of 64³ (see pressure_half.wgsl) — the fine flux balances are summed
   *  over 2×2×2 blocks (face by face, so the coarse system is sealed exactly where the fine grid is and thin fluid layers keep their equations),
   *  the same rbIters red/black SOR pairs run on the blocks (warm-started, their own textures), the result is prolongated trilinearly along open
   *  connections only into pA, and the unchanged `project` subtracts its gradient. Divergence under two cells wide is only partly removed: the
   *  plume keeps its large-scale motion and loses some fine curl. Off = the untouched full-resolution path (no extra dispatch, texture or module). */
  pressureHalf: boolean;
  /** with pressureHalf: ordinary fine red/black pairs (the default module's rbgs, the panel's omega) run between the prolongation and project.
   *  Default 1, and it matters: in a CPU soak of the whole step (canister preset, 90 s) the bare coarse solve (0) let the grid-scale divergence
   *  the plume skin makes every step pile up — rms divergence ≈ 5× the fine path's, +40 % smoke mass — because a 2×2×2 block cannot even see
   *  it, while ONE fine pair after the coarse solve removes it (rms divergence at or under the fine path's own, mass within a few %, calmer
   *  after the emitter dies). 2–3 measured slightly WORSE than 1 (a second over-relaxed pair overshoots back), so they are only there to try. 0…3. */
  pressureHalfSmooth: number;
  canisterDensity: number; canisterDuration: number; canisterRadius: number; canisterTemp: number; canisterSpeed: number;
  expansion: boolean;   // honour emitters' `expand` (divergence source + volume correction); off = they inject like plain jets
  /** empty-region skipping: the scalar passes (advectScalars, injectDecay) return at once in 8³-cell bricks that provably come out zero this step
   *  (see `occupancy` in smoke.wgsl). Lossless — on/off give bit-identical fields — so this is an A/B switch, not a quality knob. */
  brickSkip: boolean;
  /** density / temperature at or below this are stored as exactly 0 by injectDecay. Default 2^-14 ≈ 6.1e-5 = the smallest normal half float
   *  (below it f16 rounding stalls the decay, see injectDecay) ≈ an extinction of 3.7e-4 per metre: invisible. The one deliberate epsilon of the
   *  brick scheme — lets cells the plume left drop out of the occupancy; 0 = never flush. Applies identically with brickSkip on or off. */
  flushEps: number;
  /** debug / A-B: hold the field exactly as it is while it stays visible — no step, no emitter ageing, no domain retirement, new emits dropped.
   *  (`enabled = false` is not that: it also hides every domain from the renderer.) For pixel-exact captures of smoke shading changes. */
  frozen: boolean;
}
export const defaultSmokeParams = (): SmokeParams => ({ buoyancy: 1.6, weight: 0.03, vortConf: 4.0, velDamp: 0.3, densDecay: 0.14, tempDecay: 1.2, turbulence: 2.5, windX: 0, windZ: 0, densityScale: 1.0, enabled: true, voxelFine: 0.022, voxelCoarse: 0.055, canisterDensity: 95, canisterDuration: 32, canisterRadius: 0.2, canisterTemp: 1.6, canisterSpeed: 1.9, expansion: true, rbIters: 4, omega: 1.7, pressureHalf: false, pressureHalfSmooth: 1, brickSkip: true, flushEps: 2 ** -14, frozen: false });   // canister: about three of the old ones' worth of smoke (wider mouth, longer, denser) now that a coarse domain is a 3.5 m cube

const DIMS: [number, number, number] = [64, 64, 64];   // 262 k cells per domain: fine (2.2 cm) = a 1.4 m cube for gun smoke, coarse (5.5 cm) = a 3.5 m, ceiling-high cube for canisters — 1.6× the old 48×72×48 cells, paid for by the cheaper pressure solve
const HALF_DIMS: [number, number, number] = [DIMS[0] / 2, DIMS[1] / 2, DIMS[2] / 2];   // the pressureHalf option's coarse grid: 2×2×2 blocks (pressure_half.wgsl derives it as sim.dims / 2, slabs at zOff / 2)
const BRICK = 8;                                       // occupancy brick edge in cells: 2×2×2 workgroups of the 4³ kernels (handed to smoke.wgsl as consts, see the constructor)
const BRICK_GRID: [number, number, number] = [DIMS[0] / BRICK, DIMS[1] / BRICK, DIMS[2] / BRICK];
const BRICKS_PER_DOMAIN = BRICK_GRID[0] * BRICK_GRID[1] * BRICK_GRID[2];   // 512; also sizes the dilate kernel's workgroup arrays
const OCC_WORDS = BRICKS_PER_DOMAIN / 32;              // render occupancy: one bit per brick → 16 u32 per domain (renderOccPack in smoke.wgsl → `smokeOcc` in common.wgsl)
const BRICK_STAT_WORDS = 8;                            // per-domain counters read back for the HUD: active, scalar, vel, vmax bits, render-live, 3 spare
if (DIMS.some(n => n % BRICK)) throw new Error('smoke: DIMS must be multiples of the occupancy brick');
if (BRICK !== 8 || OCC_WORDS !== 16) throw new Error('smoke: common.wgsl (SMOKE_BRICK, smokeOcc: 16 words per domain) assumes 64³ domains cut into 8³ bricks');
export const MAX_DOMAINS = 8;
// 32, not 16: a flashbang alone is 2 end jets + 5 body vents + a wisp + a dozen spark trails for its first second. Nearly free — the
// per-cell loops early-out on the domain index and the uniform is 2 KB; what it bounds is how many sources one event may own at once.
const MAX_EMITTERS = 32;
const EMITTER_WORDS = 16;                      // struct Emitter in smoke.wgsl: 4 × vec4
const SIM_HEADER_WORDS = 20;                   // SimParams fields before the array (80 B, keeps the array 16-aligned)
const SIM_PARAMS_BYTES = SIM_HEADER_WORDS * 4 + MAX_EMITTERS * EMITTER_WORDS * 4;
const LEGACY_PUSH = 25;                        // the blend rate every emitter had before `push` was per-emitter

type DomainClass = 'fine' | 'coarse';
interface Domain {
  index: number; live: boolean; origin: Vec3; floorClosed: boolean; voxel: number; cls: DomainClass;
  lastEmit: number; born: number; parity: number; mass: number; needsBake: boolean; needsClear: boolean;
  /** pressureHalf: this domain's coarse pressure slabs must be zeroed before their next use (set on placement and, for every domain, when the option goes on) */
  needsClearCoarse: boolean;
  /** last brick occupancy read back: bricks the scalar passes ran in, bricks holding density/temperature, bricks holding velocity, max |v component|,
   *  bricks the renderer must still fetch in (density in the brick or its one-cell shell) */
  bricks: { active: number; scalar: number; vel: number; vmax: number; render: number };
}

const CS = GPUShaderStage.COMPUTE;

/** The pressureHalf option's GPU side, built the first time the option is switched on: a second shader module (smoke.wgsl + pressure_half.wgsl),
 *  its pipelines (compiled async — the fine path keeps running until they resolve), and the coarse textures: pressure ping / pong (warm start,
 *  persistent per domain slab) and, rewritten every step by restrictDiv, the block divergence and the packed per-block face-count word. */
interface HalfRes {
  ready: boolean; failed: string | null;
  pipes: Record<string, GPUComputePipeline>;
  pA: GPUTexture; pB: GPUTexture; div: GPUTexture; blocks: GPUTexture;
}

export class SmokeSystem implements SmokeSystemLike {
  params: SmokeParams = defaultSmokeParams();
  domains: Domain[] = [];
  emitters: (SmokeEmitter & { domain: number })[] = [];
  pushes: (SmokePush & { domain: number })[] = [];
  time = 0;
  binding: SmokeBinding;
  // textures
  private velA: GPUTexture; private velB: GPUTexture; private densA: GPUTexture; private densB: GPUTexture; private curlT: GPUTexture;
  private pA: GPUTexture; private pB: GPUTexture; private divT: GPUTexture; private obst: GPUTexture;
  private dummyF16: GPUTexture; private dummyR32: GPUTexture; private dummyW: GPUTexture[];
  private simParamBuf: GPUBuffer; private domParamBuf: GPUBuffer; private massBuf: GPUBuffer; private massRead: GPUBuffer; private massPending = false; private massFrame = 0;
  // brick occupancy: raw per-brick words, the dilated active set the scalar kernels test, and per-domain counters read back for stats
  private brickRaw: GPUBuffer; private brickAct: GPUBuffer; private brickStatBuf: GPUBuffer; private brickStatRead: GPUBuffer; private brickPending = false; private brickFrame = 0;
  // render occupancy: per-brick region masks (scratch) and the packed bit per brick the scene shaders test before fetching (renderer-facing, see renderOccPack)
  private brickRgn: GPUBuffer; private occBuf: GPUBuffer;
  private domainUbo: GPUBuffer;   // renderer-facing SmokeDomain[8]
  private g0Layout: GPUBindGroupLayout; private g1Layout: GPUBindGroupLayout; private bakeLayout: GPUBindGroupLayout;
  private g0: GPUBindGroup;
  private pipes: Record<string, GPUComputePipeline> = {};
  private bakePipe: GPUComputePipeline;
  private bgs = new Map<string, GPUBindGroup>();
  private bakeBG: GPUBindGroup | null = null; private bakeBGItems: GPUBuffer | null = null;
  private sampler: GPUSampler;
  private prelude: string;                        // consts prepended to smoke.wgsl (and to smoke.wgsl + pressure_half.wgsl for the option's module)
  private half: HalfRes | null = null;            // pressureHalf GPU objects, built on first use
  private halfArmed = false;                      // pressureHalf as encode() last saw it: an off→on edge zeroes every domain's coarse pressure
  private simData = new ArrayBuffer(SIM_PARAMS_BYTES); private simF = new Float32Array(this.simData); private simU = new Uint32Array(this.simData);
  /** bricks*: occupancy summed over live domains as last read back (a frame or two behind) — bricksActive / bricksTotal is the fraction of the
   *  scalar passes that actually ran; bricksScalar = bricks holding density or temperature, bricksVel = bricks holding any velocity;
   *  bricksRender = bricks the renderer still fetches in (density in the brick or its one-cell shell): the rest of every live domain costs it no fetch.
   *  steps = dispatches encoded this frame (all live domains); perStep = dispatches of one domain step (the last one encoded); pressure = which
   *  pressure solve that step ran ('fine' = the 64³ red/black SOR, 'half' = the pressureHalf option's 32³ solve); halfStatus = the option's GPU
   *  side: 'off' (never requested), 'compiling', 'ready', or 'failed: …' (the fine path keeps running). */
  stats = { live: 0, emitters: 0, steps: 0, perStep: 0, pressure: 'fine' as 'fine' | 'half', halfStatus: 'off', bricksActive: 0, bricksScalar: 0, bricksVel: 0, bricksRender: 0, bricksTotal: 0, vmax: 0 };

  constructor(private engine: Engine) {
    const device = engine.device;
    const [W, H, D] = DIMS; const AD = D * MAX_DOMAINS;
    const mk3d = (format: GPUTextureFormat, label: string, extra: GPUTextureUsageFlags = 0, w = W, h = H, d = AD) => device.createTexture({ label, size: [w, h, d], dimension: '3d', format, usage: GPUTextureUsage.STORAGE_BINDING | GPUTextureUsage.TEXTURE_BINDING | GPUTextureUsage.COPY_SRC | extra });
    this.velA = mk3d('rgba16float', 'smokeVelA'); this.velB = mk3d('rgba16float', 'smokeVelB');
    this.densA = mk3d('rgba16float', 'smokeDensA'); this.densB = mk3d('rgba16float', 'smokeDensB');
    this.curlT = mk3d('rgba16float', 'smokeCurl');
    this.pA = mk3d('r32float', 'smokePA'); this.pB = mk3d('r32float', 'smokePB'); this.divT = mk3d('r32float', 'smokeDiv'); this.obst = mk3d('r32float', 'smokeObst');
    this.dummyF16 = mk3d('rgba16float', 'dummyF16', 0, 4, 4, 4); this.dummyR32 = mk3d('r32float', 'dummyR32', 0, 4, 4, 4);
    // one distinct dummy per writable slot (writable storage bindings may not alias within a dispatch)
    this.dummyW = [mk3d('rgba16float', 'dummyW20', 0, 4, 4, 4), mk3d('rgba16float', 'dummyW21', 0, 4, 4, 4), mk3d('rgba16float', 'dummyW22', 0, 4, 4, 4), mk3d('r32float', 'dummyW23', 0, 4, 4, 4), mk3d('r32float', 'dummyW24', 0, 4, 4, 4)];
    this.sampler = device.createSampler({ magFilter: 'linear', minFilter: 'linear', addressModeU: 'clamp-to-edge', addressModeV: 'clamp-to-edge', addressModeW: 'clamp-to-edge' });
    this.simParamBuf = makeBuffer(device, SIM_PARAMS_BYTES, GPUBufferUsage.UNIFORM | GPUBufferUsage.COPY_DST, 'smokeSimParams');
    this.domParamBuf = makeBuffer(device, 256 * MAX_DOMAINS, GPUBufferUsage.UNIFORM | GPUBufferUsage.COPY_DST, 'smokeDomParams');
    this.massBuf = makeBuffer(device, 4 * 8, GPUBufferUsage.STORAGE | GPUBufferUsage.COPY_SRC | GPUBufferUsage.COPY_DST, 'smokeMass');
    this.massRead = device.createBuffer({ size: 32, usage: GPUBufferUsage.COPY_DST | GPUBufferUsage.MAP_READ });
    this.domainUbo = makeBuffer(device, 64 * 8, GPUBufferUsage.UNIFORM | GPUBufferUsage.COPY_DST, 'smokeDomainsUbo');
    this.brickRaw = makeBuffer(device, 8 * BRICKS_PER_DOMAIN * MAX_DOMAINS, GPUBufferUsage.STORAGE, 'smokeBrickRaw');           // vec2u per brick
    this.brickAct = makeBuffer(device, 4 * BRICKS_PER_DOMAIN * MAX_DOMAINS, GPUBufferUsage.STORAGE, 'smokeBrickAct');           // u32 per brick
    this.brickStatBuf = makeBuffer(device, 4 * BRICK_STAT_WORDS * MAX_DOMAINS, GPUBufferUsage.STORAGE | GPUBufferUsage.COPY_SRC, 'smokeBrickStats');   // BRICK_STAT_WORDS u32 per domain
    this.brickStatRead = device.createBuffer({ size: 4 * BRICK_STAT_WORDS * MAX_DOMAINS, usage: GPUBufferUsage.COPY_DST | GPUBufferUsage.MAP_READ });
    this.brickRgn = makeBuffer(device, 4 * BRICKS_PER_DOMAIN * MAX_DOMAINS, GPUBufferUsage.STORAGE, 'smokeBrickRgn');           // u32 region mask per brick (scratch between the two render-occupancy kernels)
    // one bit per brick, 16 words per domain: written (storage) by renderOccPack at the end of every domain step, read as a 512-B uniform block by the scene
    // shaders (binding 11) — a uniform rather than a seventh scene storage buffer keeps every pipeline inside the base storage-buffers-per-stage limit.
    // Zero-initialised = "nothing to fetch anywhere", which is right for slots that never stepped (they are never live either).
    this.occBuf = makeBuffer(device, 4 * OCC_WORDS * MAX_DOMAINS, GPUBufferUsage.STORAGE | GPUBufferUsage.UNIFORM, 'smokeRenderOcc');
    for (let i = 0; i < MAX_DOMAINS; i++) this.domains.push({ index: i, live: false, origin: [0, -100, 0], floorClosed: false, voxel: 0.03, cls: 'fine', lastEmit: -1e9, born: 0, parity: 0, mass: 0, needsBake: false, needsClear: false, needsClearCoarse: true, bricks: { active: 0, scalar: 0, vel: 0, vmax: 0, render: 0 } });

    // layouts
    this.g0Layout = device.createBindGroupLayout({ label: 'smokeG0', entries: [
      { binding: 0, visibility: CS, buffer: { type: 'uniform' } },
      { binding: 1, visibility: CS, buffer: { type: 'uniform', hasDynamicOffset: true } },
    ] });
    const t3 = (sampleType: GPUTextureSampleType): GPUBindGroupLayoutEntry['texture'] => ({ sampleType, viewDimension: '3d' });
    const st = (format: GPUTextureFormat): GPUBindGroupLayoutEntry['storageTexture'] => ({ access: 'write-only', format, viewDimension: '3d' });
    this.g1Layout = device.createBindGroupLayout({ label: 'smokeG1', entries: [
      { binding: 10, visibility: CS, sampler: { type: 'filtering' } },
      { binding: 11, visibility: CS, texture: t3('float') }, { binding: 12, visibility: CS, texture: t3('float') }, { binding: 13, visibility: CS, texture: t3('float') },
      { binding: 14, visibility: CS, texture: t3('unfilterable-float') }, { binding: 15, visibility: CS, texture: t3('unfilterable-float') }, { binding: 16, visibility: CS, texture: t3('unfilterable-float') },
      { binding: 20, visibility: CS, storageTexture: st('rgba16float') }, { binding: 21, visibility: CS, storageTexture: st('rgba16float') }, { binding: 22, visibility: CS, storageTexture: st('rgba16float') },
      { binding: 23, visibility: CS, storageTexture: st('r32float') }, { binding: 24, visibility: CS, storageTexture: st('r32float') },
      { binding: 25, visibility: CS, buffer: { type: 'storage' } },
      { binding: 26, visibility: CS, buffer: { type: 'storage' } }, { binding: 27, visibility: CS, buffer: { type: 'storage' } }, { binding: 28, visibility: CS, buffer: { type: 'storage' } },
      { binding: 29, visibility: CS, buffer: { type: 'storage' } }, { binding: 30, visibility: CS, buffer: { type: 'storage' } },
    ] });
    this.bakeLayout = device.createBindGroupLayout({ label: 'smokeBake', entries: [
      { binding: 0, visibility: CS, storageTexture: st('r32float') },
      { binding: 1, visibility: CS, buffer: { type: 'read-only-storage' } }, { binding: 2, visibility: CS, buffer: { type: 'read-only-storage' } }, { binding: 3, visibility: CS, buffer: { type: 'read-only-storage' } },
      { binding: 4, visibility: CS, buffer: { type: 'uniform' } },
    ] });
    this.g0 = device.createBindGroup({ layout: this.g0Layout, entries: [{ binding: 0, resource: { buffer: this.simParamBuf } }, { binding: 1, resource: { buffer: this.domParamBuf, offset: 0, size: 32 } }] });
    this.prelude = wgslWorldConsts() + `const BRICK: u32 = ${BRICK}u;\nconst BRICKS_PER_DOMAIN: u32 = ${BRICKS_PER_DOMAIN}u;\nconst OCC_WORDS: u32 = ${OCC_WORDS}u;\nconst BRICK_STAT_WORDS: u32 = ${BRICK_STAT_WORDS}u;\n`;
    const mod = makeShader(device, this.prelude + smokeSrc, 'smoke');
    const plSim = device.createPipelineLayout({ bindGroupLayouts: [this.g0Layout, this.g1Layout] });
    for (const ep of ['advectVel', 'curl', 'forces', 'divergence', 'project', 'occupancy', 'dilate', 'advectScalars', 'injectDecay', 'renderOccRegions', 'renderOccPack', 'clearAll', 'reduceMass'])
      this.pipes[ep] = device.createComputePipeline({ label: 'smoke_' + ep, layout: plSim, compute: { module: mod, entryPoint: ep } });
    for (const [name, col] of [['rbgsR', 0], ['rbgsB', 1]] as const) this.pipes[name] = device.createComputePipeline({ label: 'smoke_' + name, layout: plSim, compute: { module: mod, entryPoint: 'rbgs', constants: { RB_COLOR: col } } });   // red / black half sweeps of the SOR pressure solve
    this.bakePipe = device.createComputePipeline({ label: 'smoke_bake', layout: device.createPipelineLayout({ bindGroupLayouts: [this.g0Layout, this.bakeLayout] }), compute: { module: mod, entryPoint: 'bakeObstacles' } });

    this.binding = { uniform: this.domainUbo, atlasView: this.densA.createView(), occ: this.occBuf, count: 0, encode: (enc, ts) => this.encode(enc, ts) };
    this.writeDomainUbo();
  }

  // ---------------------------------------------------------------- bind groups
  private bg(name: string, spec: Partial<Record<'srcVel' | 'srcDens' | 'srcCurl' | 'srcP' | 'srcDiv' | 'srcObst' | 'dstVel' | 'dstDens' | 'dstCurl' | 'dstP' | 'dstDiv', GPUTexture>>): GPUBindGroup {
    let g = this.bgs.get(name); if (g) return g;
    const v = (t: GPUTexture | undefined, d: GPUTexture) => (t ?? d).createView();
    g = this.engine.device.createBindGroup({ label: name, layout: this.g1Layout, entries: [
      { binding: 10, resource: this.sampler },
      { binding: 11, resource: v(spec.srcVel, this.dummyF16) }, { binding: 12, resource: v(spec.srcDens, this.dummyF16) }, { binding: 13, resource: v(spec.srcCurl, this.dummyF16) },
      { binding: 14, resource: v(spec.srcP, this.dummyR32) }, { binding: 15, resource: v(spec.srcDiv, this.dummyR32) }, { binding: 16, resource: v(spec.srcObst, this.dummyR32) },
      { binding: 20, resource: v(spec.dstVel, this.dummyW[0]) }, { binding: 21, resource: v(spec.dstDens, this.dummyW[1]) }, { binding: 22, resource: v(spec.dstCurl, this.dummyW[2]) },
      { binding: 23, resource: v(spec.dstP, this.dummyW[3]) }, { binding: 24, resource: v(spec.dstDiv, this.dummyW[4]) },
      { binding: 25, resource: { buffer: this.massBuf } },
      { binding: 26, resource: { buffer: this.brickRaw } }, { binding: 27, resource: { buffer: this.brickAct } }, { binding: 28, resource: { buffer: this.brickStatBuf } },
      { binding: 29, resource: { buffer: this.brickRgn } }, { binding: 30, resource: { buffer: this.occBuf } },
    ] });
    this.bgs.set(name, g); return g;
  }

  // ---------------------------------------------------------------- domains
  private size(d: { voxel: number }): Vec3 { return [DIMS[0] * d.voxel, DIMS[1] * d.voxel, DIMS[2] * d.voxel]; }
  private domainContains(d: Domain, p: Vec3, margin: number): boolean {
    const [sx, sy, sz] = this.size(d);
    return p[0] > d.origin[0] + margin && p[0] < d.origin[0] + sx - margin && p[2] > d.origin[2] + margin && p[2] < d.origin[2] + sz - margin && p[1] > d.origin[1] + 0.05 && p[1] < d.origin[1] + sy - Math.min(0.45, sy * 0.25);
  }
  private placeDomain(p: Vec3, cls: DomainClass): Domain {
    let d = this.domains.find(x => !x.live);
    if (!d) {
      // pool exhausted: recycle the domain that matters least — no live emitters first, then least mass, then oldest emission
      const busy = new Set(this.emitters.map(e => e.domain));
      const score = (x: Domain) => (busy.has(x.index) ? 1e6 : 0) + x.mass * 10 + Math.max(0, 30 - (this.time - x.lastEmit));
      d = this.domains.reduce((a, b) => (score(a) <= score(b) ? a : b));
    }
    d.voxel = cls === 'fine' ? this.params.voxelFine : this.params.voxelCoarse; d.cls = cls;
    const [sx, sy, sz] = this.size(d);
    let oy = p[1] - (cls === 'fine' ? 0.55 : 0.75); let floorClosed = false;
    if (oy < 0.3) { oy = 0; floorClosed = true; }
    oy = Math.min(oy, Math.max(0, WORLD.ceilingY + 0.15 - sy));
    if (oy <= 0.001) floorClosed = true;
    const snap = (x: number) => Math.round(x / d!.voxel) * d!.voxel;
    d.origin = [snap(p[0] - sx / 2), snap(Math.max(0, oy)), snap(p[2] - sz / 2)];
    d.floorClosed = floorClosed; d.live = true; d.born = this.time; d.lastEmit = this.time; d.mass = 0; d.needsBake = true; d.needsClear = true; d.needsClearCoarse = true; d.parity = 0; d.bricks = { active: 0, scalar: 0, vel: 0, vmax: 0, render: 0 };
    this.writeDomainUbo();
    return d;
  }
  /** A live domain of the class that holds `p` with slack (and, when given, the emitter position `ep` inside the acceptance margin update() enforces —
   *  else a reused domain could cull the very emitter that asked for it, e.g. a second shot's bore jet 0.16 m behind its anchor), or a fresh one at p. */
  private domainFor(p: Vec3, cls: DomainClass, ep?: Vec3): Domain {
    for (const d of this.domains) if (d.live && d.cls === cls && this.domainContains(d, p, cls === 'fine' ? 0.2 : 0.35) && (!ep || this.domainContains(d, ep, 0.12))) return d;
    return this.placeDomain(p, cls);
  }
  private writeDomainUbo() {
    // renderer-facing: struct SmokeDomain { origin: vec3f, voxel: f32, atlasOffset: vec3u, live: u32, dims: vec3u, pad: u32, densityScale, pad×3 }
    const data = new ArrayBuffer(64 * 8); const f = new Float32Array(data), u = new Uint32Array(data);
    let n = 0;
    this.domains.forEach((d, i) => {
      const o = i * 16;
      f[o] = d.origin[0]; f[o + 1] = d.origin[1]; f[o + 2] = d.origin[2]; f[o + 3] = d.voxel;
      u[o + 4] = 0; u[o + 5] = 0; u[o + 6] = DIMS[2] * i; u[o + 7] = d.live ? 1 : 0;
      u[o + 8] = DIMS[0]; u[o + 9] = DIMS[1]; u[o + 10] = DIMS[2]; u[o + 11] = 0;
      f[o + 12] = this.params.densityScale; f[o + 13] = 0; f[o + 14] = 0; f[o + 15] = 0;
      if (d.live) n = i + 1;
    });
    this.engine.device.queue.writeBuffer(this.domainUbo, 0, data);
    this.binding.count = this.params.enabled ? n : 0;
    // sim-facing per-domain params (256-B stride): zOff, index, floorClosed, pad, origin
    const dp = new ArrayBuffer(256 * MAX_DOMAINS);
    this.domains.forEach((d, i) => { const uu = new Uint32Array(dp, 256 * i, 8), ff = new Float32Array(dp, 256 * i, 8); uu[0] = DIMS[2] * i; uu[1] = i; uu[2] = d.floorClosed ? 1 : 0; uu[3] = 0; ff[4] = d.origin[0]; ff[5] = d.origin[1]; ff[6] = d.origin[2]; ff[7] = d.voxel; });
    this.engine.device.queue.writeBuffer(this.domParamBuf, 0, dp);
  }

  // ---------------------------------------------------------------- API
  emit(e: SmokeEmitter) {
    if (!this.params.enabled || this.params.frozen) return;   // frozen: a fresh domain would go live over a slab (and occupancy words) nothing clears until the next step
    const d = this.domainFor(e.anchor ?? e.pos, e.lattice ?? (e.kind === 'canister' ? 'coarse' : 'fine'), e.pos);
    d.lastEmit = this.time;
    // a splat narrower than a cell falls between samples: deposits next to nothing and flickers as it moves (spark trails on the 5.5 cm lattice)
    const radius = e.minVoxels ? Math.max(e.radius, e.minVoxels * d.voxel) : e.radius;
    this.emitters.push({ ...e, radius, pos: v3.copy(e.pos), dir: v3.normalize(e.dir), domain: d.index });
    if (this.emitters.length > 64) { const drop = this.emitters.findIndex(x => (x.prio ?? 1) === 0); this.emitters.splice(drop >= 0 ? drop : 0, 1); }   // shed a spark trail before a screen
  }
  push(p: SmokePush) {
    if (!this.params.enabled || this.params.frozen || this.pushes.length >= 10) return;
    for (const d of this.domains) if (d.live && this.domainContains(d, p.pos, 0.02)) { this.pushes.push({ ...p, pos: v3.copy(p.pos), domain: d.index }); return; }
  }
  inSmokeDomain(p: Vec3): boolean { for (const d of this.domains) if (d.live && d.mass > 2 && this.domainContains(d, p, 0)) return true; return false; }
  /** emitters an effect may still add and have packed this step (pushes take up to 6 of the MAX_EMITTERS slots) */
  budget(): number { return Math.max(0, MAX_EMITTERS - 6 - this.emitters.length); }
  spawnCanister(pos: Vec3, duration?: number) {
    const P = this.params;
    this.emit({ pos: v3.add(pos, [0, 0.12, 0]), dir: [0, 1, 0], speed: P.canisterSpeed, radius: P.canisterRadius, density: P.canisterDensity, temperature: P.canisterTemp, ttl: duration ?? P.canisterDuration, age: 0, kind: 'canister' });
  }
  clearAll() { for (const d of this.domains) { d.live = false; d.lastEmit = -1e9; } this.emitters.length = 0; this.writeDomainUbo(); }

  update(dt: number) {
    if (this.params.frozen) return;   // debug freeze: no ageing, tracking or retirement either — the field, its domains and its occupancy stay exactly put
    this.time += dt;
    // age emitters, follow tracked sources, migrate to a containing domain if the source moved out
    for (let i = this.emitters.length - 1; i >= 0; i--) {
      const e = this.emitters[i]; e.age += dt;
      if (e.age >= e.ttl) { this.emitters.splice(i, 1); continue; }
      if (e.track) {
        const t = e.track();
        if (t) { e.pos = v3.copy(t.pos); if (e.kind === 'wisp') e.dir = v3.normalize(v3.add([0, 1, 0], v3.scale(t.dir, 0.3))); }
        else if (e.confined) { this.emitters.splice(i, 1); continue; }   // the spark it was riding went out
      }
      const d = this.domains[e.domain];
      if (!d.live || !this.domainContains(d, e.pos, 0.1)) {
        // a barrel wisp that wanders out of its domain (shooter running) or a spark trail arcing out of the box is not worth a fresh 64³ domain — let it go
        if (e.kind === 'wisp' || e.confined) { this.emitters.splice(i, 1); continue; }
        const nd = this.domainFor(e.pos, d.cls); e.domain = nd.index;
      }
      this.domains[e.domain].lastEmit = this.time;
    }
    // retire idle domains
    for (const d of this.domains) {
      if (!d.live) continue;
      const idle = this.time - d.lastEmit;
      if ((idle > 4 && d.mass < 0.5 && this.time - d.born > 2) || idle > 40) { d.live = false; this.writeDomainUbo(); }
    }
    this.stats.live = this.domains.filter(d => d.live).length; this.stats.emitters = this.emitters.length;
    if (this.stats.live === 0) {
      this.pushes.length = 0;   // nothing will consume them; don't let them leak into a recycled domain index later
      this.stats.bricksActive = this.stats.bricksScalar = this.stats.bricksVel = this.stats.bricksRender = this.stats.bricksTotal = this.stats.vmax = 0;   // no step runs, so no readback refreshes these
    }
    if (this.binding.count !== (this.params.enabled ? this.liveCount() : 0)) this.writeDomainUbo();
  }
  private liveCount() { let n = 0; this.domains.forEach((d, i) => { if (d.live) n = i + 1; }); return n; }

  // ---------------------------------------------------------------- GPU
  private writeSimParams(dt: number) {
    const f = this.simF, u = this.simU; const P = this.params;
    u[0] = DIMS[0]; u[1] = DIMS[1]; u[2] = DIMS[2];
    type Packed = (SmokeEmitter | (SmokePush & { density?: number; temperature?: number; kind?: undefined; age?: number; ttl?: number; mode?: undefined; push?: number; expand?: number; attack?: number; jitter?: number; prio?: number })) & { domain: number };
    const room = MAX_EMITTERS - Math.min(this.pushes.length, 6);
    let ems: Packed[] = this.emitters;
    // more live emitters than slots: keep by priority (screens > jets / wisps > spark trails), newest first within a priority — never starve a
    // 20 s canister for the second half of a flashbang's trails
    const prioOf = (e: Packed) => e.prio ?? (e.kind === 'canister' ? 2 : 1);
    if (ems.length > room) ems = ems.map((e, i) => ({ e, i })).sort((a, b) => (prioOf(b.e) - prioOf(a.e)) || (b.i - a.i)).slice(0, room).map(x => x.e);
    const all: Packed[] = [...ems, ...this.pushes.slice(0, MAX_EMITTERS - ems.length)];
    // expanding sources first: the solver's two per-cell expansion loops only run over [0, numExpand)
    const expands = (e: Packed) => P.expansion && (e.expand ?? 0) > 0;
    const em = [...all.filter(expands), ...all.filter(e => !expands(e))];
    u[3] = em.length;
    f[4] = P.turbulence; f[5] = dt; f[6] = this.time; f[7] = P.buoyancy;
    f[8] = P.weight; f[9] = P.vortConf; f[10] = P.velDamp; f[11] = P.densDecay;
    f[12] = P.tempDecay; f[13] = DIMS[2] * MAX_DOMAINS; f[14] = P.windX; f[15] = P.windZ;
    u[16] = em.filter(expands).length; f[17] = P.omega; u[18] = P.brickSkip ? 1 : 0; f[19] = Math.max(0, P.flushEps);
    em.forEach((e, i) => {
      const o = SIM_HEADER_WORDS + i * EMITTER_WORDS; const d = this.domains[e.domain];
      const kind: string = e.kind ?? 'push';
      let dens = e.density ?? 0, temp = e.temperature ?? 0, expand = P.expansion ? (e.expand ?? 0) : 0;
      const age = e.age ?? 0, ttl = e.ttl ?? 1; const x = age / ttl;
      // emission envelope: the ported effects ramp in over `attack` and out over the last quarter (density, heat AND expansion, so a burst does not
      // shove the air before it has any smoke to shove); legacy kinds keep their hand-made fades
      let env = 1;
      if (e.attack !== undefined) env = Math.min(1, age / Math.max(e.attack, 1e-3)) * (x > 0.75 ? Math.max(0, (1 - x) / 0.25) : 1);
      else if (kind === 'wisp') env = Math.pow(1 - x, 1.5);
      else if (kind === 'canister') env = Math.min(1, age / 0.6) * (x > 0.85 ? (1 - x) / 0.15 : 1);
      dens *= env; temp *= env; expand *= env;
      // per-frame jitter of the jet direction/speed seeds asymmetry that confinement/turbulence then amplify (seeded effects bring their own, smaller)
      const j = e.jitter ?? (kind === 'flash' || kind === 'push' || kind === 'burst' ? 0.08 : 0.35); const jd = v3.normalize([e.dir[0] + (Math.random() - 0.5) * j, e.dir[1] + (Math.random() - 0.5) * j, e.dir[2] + (Math.random() - 0.5) * j]);
      const sp = e.speed * (kind === 'push' ? 1 : e.jitter !== undefined ? 1 + (Math.random() - 0.5) * e.jitter : 0.75 + Math.random() * 0.5);
      f[o] = e.pos[0] - d.origin[0]; f[o + 1] = e.pos[1] - d.origin[1]; f[o + 2] = e.pos[2] - d.origin[2]; f[o + 3] = e.radius;
      f[o + 4] = jd[0]; f[o + 5] = jd[1]; f[o + 6] = jd[2]; f[o + 7] = sp;
      f[o + 8] = dens; f[o + 9] = temp; u[o + 10] = e.domain; f[o + 11] = e.mode === 'burst' ? 1 : 0;
      f[o + 12] = e.push ?? LEGACY_PUSH; f[o + 13] = expand; f[o + 14] = 0; f[o + 15] = 0;
    });
    this.pushes.length = 0;
    this.engine.device.queue.writeBuffer(this.simParamBuf, 0, this.simData);
  }

  /** First use of params.pressureHalf: build the option's module, pipelines and coarse textures. The pipelines compile asynchronously — encode()
   *  stays on the full-resolution solve until `ready`, and for good if any of them fails (logged; stats.halfStatus says which). Nothing here exists,
   *  or is compiled, on a run that never switches the option on. */
  private ensureHalf() {
    if (this.half) return;
    const device = this.engine.device;
    const [w, h, d] = HALF_DIMS;
    const tex = (label: string) => device.createTexture({ label, size: [w, h, d * MAX_DOMAINS], dimension: '3d', format: 'r32float', usage: GPUTextureUsage.STORAGE_BINDING | GPUTextureUsage.TEXTURE_BINDING | GPUTextureUsage.COPY_SRC });
    const H: HalfRes = { ready: false, failed: null, pipes: {}, pA: tex('smokeHalfPA'), pB: tex('smokeHalfPB'), div: tex('smokeHalfDiv'), blocks: tex('smokeHalfBlocks') };   // 4 × 1 MB
    this.half = H; this.stats.halfStatus = 'compiling';
    const mod = makeShader(device, this.prelude + smokeSrc + halfSrc, 'smokeHalf');   // (a compile message's line number counts the prelude and all of smoke.wgsl first)
    const layout = device.createPipelineLayout({ bindGroupLayouts: [this.g0Layout, this.g1Layout] });
    const specs: [string, string, Record<string, number>?][] = [
      ['restrictDiv', 'restrictDiv'], ['prolongate', 'prolongate'], ['clearCoarse', 'clearCoarse'],
      ['sorCR', 'rbgsC', { RB_COLOR: 0 }], ['sorCB', 'rbgsC', { RB_COLOR: 1 }],   // coarse red / black half sweeps (the fine smoothing pairs reuse the default module's rbgsR / rbgsB)
    ];
    Promise.all(specs.map(([name, entryPoint, constants]) => device.createComputePipelineAsync({ label: 'smoke_' + name, layout, compute: { module: mod, entryPoint, ...(constants ? { constants } : {}) } }).then(p => { H.pipes[name] = p; })))
      .then(() => { H.ready = true; this.stats.halfStatus = 'ready'; })
      .catch(e => { H.failed = String(e?.message ?? e); this.stats.halfStatus = 'failed: ' + H.failed; console.error('smoke: pressureHalf pipelines failed to build — staying on the full-resolution pressure solve', e); });
  }

  private ensureBakeBG() {
    const w = this.engine.world;
    if (this.bakeBG && this.bakeBGItems === w.itemBuf) return;
    this.bakeBG = this.engine.device.createBindGroup({ layout: this.bakeLayout, entries: [
      { binding: 0, resource: this.obst.createView() },
      { binding: 1, resource: { buffer: w.geoBuf } }, { binding: 2, resource: { buffer: w.cellBuf } }, { binding: 3, resource: { buffer: w.itemBuf } }, { binding: 4, resource: { buffer: w.infoBuf } },
    ] });
    this.bakeBGItems = w.itemBuf;
  }

  encode(enc: GPUCommandEncoder, ts: (n: string) => GPUComputePassTimestampWrites | undefined) {
    // pressureHalf edge: the first frame it is seen on builds the option's GPU side (once) and marks every domain's coarse pressure for zeroing, so a
    // runtime toggle never warm-starts from a slab left over from the last time it was on. Checked before the early-outs so an edge while nothing
    // is live still counts. (An on→off→on between two frames is not an edge: the coarse field is then simply still current.)
    const wantHalf = !!this.params.pressureHalf;
    if (wantHalf && !this.halfArmed) { this.ensureHalf(); for (const d of this.domains) d.needsClearCoarse = true; }
    this.halfArmed = wantHalf;
    const live = this.domains.filter(d => d.live);
    this.stats.steps = 0;
    if (!this.params.enabled || this.params.frozen || live.length === 0) return;   // (frozen: densA and the render occupancy both keep their last step — still a matched pair)
    if (this.engine.lastDt <= 0) return;                        // paused (menu open): the field holds still — stepping with a fallback dt kept injecting behind the pause card
    const dt = Math.min(1 / 30, Math.max(1 / 2000, this.engine.lastDt));
    this.writeSimParams(dt);
    this.ensureBakeBG();
    const [W, H, D] = DIMS; const wg: [number, number, number] = [Math.ceil(W / 4), Math.ceil(H / 4), Math.ceil(D / 4)];
    const cwg: [number, number, number] = [Math.ceil(HALF_DIMS[0] / 4), Math.ceil(HALF_DIMS[1] / 4), Math.ceil(HALF_DIMS[2] / 4)];   // one invocation per 2×2×2 block
    const HR = wantHalf && this.half?.ready ? this.half : null;   // the option runs this frame (else: off, still compiling, or failed → the fine solve, untouched)
    this.stats.pressure = HR ? 'half' : 'fine';
    const pass = enc.beginComputePass({ label: 'smoke', timestampWrites: ts('smoke') });
    const A = { velA: this.velA, velB: this.velB };
    const doMass = (++this.massFrame % 15) === 0 && !this.massPending;   // every 15th step: reduceMass per domain, then (after the pass) copy out + clear the accumulators
    for (const d of live) {
      const dyn = [256 * d.index]; const steps0 = this.stats.steps;
      pass.setBindGroup(0, this.g0, dyn);
      const run = (pipe: string, bg: GPUBindGroup, groups: readonly [number, number, number] = wg, pipes = this.pipes) => { pass.setPipeline(pipes[pipe]); pass.setBindGroup(1, bg); pass.dispatchWorkgroups(...groups); this.stats.steps++; };
      if (d.needsClear) {
        run('clearAll', this.bg('clear0', { dstVel: this.velA, dstDens: this.densA, dstP: this.pA, dstCurl: this.curlT }));
        run('clearAll', this.bg('clear1', { dstVel: this.velB, dstDens: this.densB, dstP: this.pB, dstCurl: this.curlT }));
        d.needsClear = false; d.needsClearCoarse = true; d.parity = 0;
      }
      if (d.needsBake) { pass.setPipeline(this.bakePipe); pass.setBindGroup(1, this.bakeBG!); pass.dispatchWorkgroups(...wg); d.needsBake = false; }
      // velocity roles alternate each step: "cur" holds the latest projected field
      const cur = d.parity === 0 ? A.velA : A.velB, oth = d.parity === 0 ? A.velB : A.velA; const pn = d.parity;
      run('advectVel', this.bg(`advV${pn}`, { srcVel: cur, srcObst: this.obst, dstVel: oth }));
      run('curl', this.bg(`curl${pn}`, { srcVel: oth, dstCurl: this.curlT }));
      run('forces', this.bg(`forces${pn}`, { srcVel: oth, srcDens: this.densA, srcCurl: this.curlT, srcObst: this.obst, dstVel: cur }));
      run('divergence', this.bg(`div${pn}`, { srcVel: cur, srcObst: this.obst, dstDiv: this.divT }));
      // pressure: red-black SOR — red half sweep pA→pB, black half sweep pB→pA, so every pair leaves the result in pA where project reads it
      const red = this.bg('sorR', { srcP: this.pA, srcDiv: this.divT, srcObst: this.obst, dstP: this.pB });
      const black = this.bg('sorB', { srcP: this.pB, srcDiv: this.divT, srcObst: this.obst, dstP: this.pA });
      const pairs = Math.max(1, Math.round(this.params.rbIters));
      if (HR) {
        // LOSSY pressureHalf (pressure_half.wgsl): restrict divT + the obstacle bake to 32³ blocks (block divergence + packed face counts), run the same
        // pairs there (HR.pA → HR.pB → HR.pA, warm-started), prolongate along open connections into pA, optionally smooth with ordinary fine pairs,
        // then the unchanged project below reads pA as always
        if (d.needsClearCoarse) { run('clearCoarse', this.bg('halfClear', { dstP: HR.pA, dstDiv: HR.pB }), cwg, HR.pipes); d.needsClearCoarse = false; }
        run('restrictDiv', this.bg('halfRestrict', { srcDiv: this.divT, srcObst: this.obst, dstDiv: HR.div, dstP: HR.blocks }), cwg, HR.pipes);   // (dstP carries the block words)
        const redC = this.bg('halfSorR', { srcP: HR.pA, srcDiv: HR.div, srcObst: HR.blocks, dstP: HR.pB });
        const blackC = this.bg('halfSorB', { srcP: HR.pB, srcDiv: HR.div, srcObst: HR.blocks, dstP: HR.pA });
        for (let i = 0; i < pairs; i++) { run('sorCR', redC, cwg, HR.pipes); run('sorCB', blackC, cwg, HR.pipes); }
        run('prolongate', this.bg('halfProlong', { srcP: HR.pA, srcObst: HR.blocks, dstP: this.pA }), cwg, HR.pipes);
        const smooth = Math.max(0, Math.min(3, Math.round(this.params.pressureHalfSmooth) || 0));
        for (let i = 0; i < smooth; i++) { run('rbgsR', red); run('rbgsB', black); }   // the fine sweeps and their sorR / sorB bind groups as they are: pA → pB → pA
      } else {
        for (let i = 0; i < pairs; i++) { pass.setPipeline(this.pipes.rbgsR); pass.setBindGroup(1, red); pass.dispatchWorkgroups(...wg); pass.setPipeline(this.pipes.rbgsB); pass.setBindGroup(1, black); pass.dispatchWorkgroups(...wg); this.stats.steps += 2; }
      }
      run('project', this.bg(`proj${pn}`, { srcVel: cur, srcP: this.pA, srcObst: this.obst, dstVel: oth }));   // projected field now in `oth`
      // brick occupancy of exactly what advectScalars will read (densA + the projected field), then the dilated active set both scalar
      // kernels test; always run (two small dispatches) so the stats show the occupancy even while params.brickSkip is off
      const occ = this.bg(`occ${pn}`, { srcVel: oth, srcDens: this.densA });
      run('occupancy', occ, BRICK_GRID);   // one 8×8 workgroup per brick
      run('dilate', occ, [1, 1, 1]);       // one workgroup per domain (textures unused)
      run('advectScalars', this.bg(`advS${pn}`, { srcVel: oth, srcDens: this.densA, srcObst: this.obst, dstDens: this.densB }));
      run('injectDecay', this.bg('inj', { srcDens: this.densB, srcObst: this.obst, dstDens: this.densA }));
      // render occupancy LAST, on densA exactly as every render pass later in this command buffer will sample it (see renderOccRegions in smoke.wgsl):
      // per-brick region masks, then the packed "brick or its one-cell shell holds density" bits the scene shaders test before each atlas fetch.
      // Always run (~one extra read of densA + a 64-thread pack) so the words never go stale, whether or not the renderer's skip flag is on.
      const rocc = this.bg('rocc', { srcDens: this.densA });
      run('renderOccRegions', rocc, BRICK_GRID);   // one 8×8 workgroup per brick
      run('renderOccPack', rocc, [1, 1, 1]);       // one workgroup per domain
      if (doMass) run('reduceMass', this.bg('mass', { srcDens: this.densA }));
      d.parity ^= 1;
      this.stats.perStep = this.stats.steps - steps0;
    }
    pass.end();
    if (doMass) {
      enc.copyBufferToBuffer(this.massBuf, 0, this.massRead, 0, 32);
      enc.clearBuffer(this.massBuf, 0, 32);
      this.massPending = true;
    }
    if (!this.brickPending && (++this.brickFrame & 3) === 0) { enc.copyBufferToBuffer(this.brickStatBuf, 0, this.brickStatRead, 0, 4 * BRICK_STAT_WORDS * MAX_DOMAINS); this.brickPending = true; }   // stats for the HUD / console: every 4th frame is plenty
  }

  /** Call after queue.submit. */
  afterSubmit() {
    if (this.massPending && this.massRead.mapState === 'unmapped') {
      this.massRead.mapAsync(GPUMapMode.READ).then(() => {
        const u = new Uint32Array(this.massRead.getMappedRange().slice(0)); this.massRead.unmap();
        this.domains.forEach((d, i) => { d.mass = u[i] / 100; });
        this.massPending = false;
      }).catch(() => { this.massPending = false; });
    }
    if (this.brickPending && this.brickStatRead.mapState === 'unmapped') {
      this.brickStatRead.mapAsync(GPUMapMode.READ).then(() => {
        const m = this.brickStatRead.getMappedRange(); const u = new Uint32Array(m), f = new Float32Array(m);
        const s = this.stats; s.bricksActive = 0; s.bricksScalar = 0; s.bricksVel = 0; s.bricksRender = 0; s.bricksTotal = 0; s.vmax = 0;
        this.domains.forEach((d, i) => {
          const b = d.bricks; const o = i * BRICK_STAT_WORDS; b.active = u[o]; b.scalar = u[o + 1]; b.vel = u[o + 2]; b.vmax = f[o + 3]; b.render = u[o + 4];
          if (!d.live) return;   // a retired domain's slot just holds its last step
          s.bricksActive += b.active; s.bricksScalar += b.scalar; s.bricksVel += b.vel; s.bricksRender += b.render; s.bricksTotal += BRICKS_PER_DOMAIN; s.vmax = Math.max(s.vmax, b.vmax);
        });
        this.brickStatRead.unmap();
        this.brickPending = false;
      }).catch(() => { this.brickPending = false; });
    }
  }

  /** Debug / A-B: hash the textures that define the simulation state (densA = what the renderer samples, plus both velocity textures) over the
   *  live domains. Equal strings ⇒ bit-identical fields, e.g. the same seeded, fixed-step run replayed with params.brickSkip on and off. densB is
   *  deliberately left out: with skipping on it is scratch that is only defined inside the step's active bricks. ~6 MB of readback per live
   *  domain — a console tool, not for the frame loop. */
  async checksum(): Promise<string> {
    const device = this.engine.device; const [W, H, D] = DIMS;
    const live = this.domains.filter(d => d.live);
    if (live.length === 0) return 'no live domains';
    const texs: [string, GPUTexture][] = [['densA', this.densA], ['velA', this.velA], ['velB', this.velB]];
    const bpr = W * 8, slab = bpr * H * D;   // rgba16float: 8 B per texel, 512-B rows (256-aligned as copies require)
    const buf = device.createBuffer({ size: slab * live.length * texs.length, usage: GPUBufferUsage.COPY_DST | GPUBufferUsage.MAP_READ });
    const enc = device.createCommandEncoder();
    texs.forEach(([, t], ti) => live.forEach((d, di) => enc.copyTextureToBuffer({ texture: t, origin: [0, 0, D * d.index] }, { buffer: buf, offset: (ti * live.length + di) * slab, bytesPerRow: bpr, rowsPerImage: H }, [W, H, D])));
    device.queue.submit([enc.finish()]);
    await buf.mapAsync(GPUMapMode.READ);
    const u = new Uint32Array(buf.getMappedRange());
    const words = slab / 4 * live.length;
    const parts = texs.map(([name], ti) => { let h = 0x811c9dc5; for (let i = ti * words, e = i + words; i < e; i++) { h = Math.imul(h ^ u[i], 0x01000193) >>> 0; } return `${name}:${h.toString(16).padStart(8, '0')}`; });   // FNV-1a over 32-bit words
    buf.unmap(); buf.destroy();
    return `${parts.join(' ')} · domains ${live.map(d => d.index).join(',')} · t=${this.time.toFixed(3)}`;
  }
}
