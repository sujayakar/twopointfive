// Screen-space lighting passes: direct (full res), indirect gather (half — a third with the lossy option), volumetrics (half — a quarter likewise), composite + FXAA.
import { makeBuffer, makeShader } from '../gpu/device';
import { FrameFlags, scenePrelude } from './frame';
import { GBuffer } from './gbuffer';
import { RadianceCascades } from './rc';
import lightingCommon from './shaders/lighting_common.wgsl' with { type: 'text' };
import directSrc from './shaders/direct.wgsl' with { type: 'text' };
import volSrc from './shaders/volumetrics.wgsl' with { type: 'text' };
import compositeSrc from './shaders/composite.wgsl' with { type: 'text' };
import fxaaSrc from './shaders/fxaa.wgsl' with { type: 'text' };
import softblurSrc from './shaders/softblur.wgsl' with { type: 'text' };
import temporalSrc from './shaders/temporal.wgsl' with { type: 'text' };
import fgatherSrc from './shaders/fgather.wgsl' with { type: 'text' };
import bloomSrc from './shaders/bloom.wgsl' with { type: 'text' };
import postSrc from './shaders/post.wgsl' with { type: 'text' };
import giblurSrc from './shaders/giblur.wgsl' with { type: 'text' };
import dtemporalSrc from './shaders/dtemporal.wgsl' with { type: 'text' };
import softtileSrc from './shaders/softtile.wgsl' with { type: 'text' };

const CS = GPUShaderStage.COMPUTE, FS = GPUShaderStage.FRAGMENT;

// Direct-light denoise chain per emitter class. Textures: S (raw estimate; also where the RESULT must end up, consumers
// bind it), P (shared scratch), H0/H1 (this class's temporal history, ping-pong). Spatial pass i uses stride
// STRIDES[i]. With history on the chain is  T: S→H[new]  then  H→P, P→S, S→P, …; without it  S→P, P→S, …; if the
// requested pass count would leave the result in P, one extra stride-1 pass brings it home to S (see buildChain). Under
// FrameFlags.SoftTileSkip every step also retires whole 8x8 tiles that are provably out of reach of any penumbra hint (skipThreshold).
const STRIDES = [1, 2, 4, 8, 16, 1, 1];
// Tile-level pass-through (FrameFlags.SoftTileSkip; softtile.wgsl builds the mask, the exactness argument is at the top of softblur.wgsl's
// main). A hint moves at most STRIDE px per pass, so after the chain step whose cumulative stride sum is `travelPx` only pixels within that
// Chebyshev distance of a hint-carrying pixel can differ from their input; pixels of tiles t apart are >= 8(t-1)+1 px apart, hence a whole
// 8x8 tile passes the step through once its tile distance to the nearest hint-holding tile reaches
const skipThreshold = (travelPx: number) => Math.floor((travelPx - 1) / 8) + 2;
// Per step: pipeline (= stride), which way it ping-pongs, and the threshold that step alone would need. The mask applies ONE threshold to
// the whole chain — the last step's, which is the largest (travel only accumulates) and therefore safe for every step — and a skipped tile
// copies its input texel to its output at every step, exactly what the per-pixel path stores there, so both ping-pong textures evolve
// step by step as they always did: nothing leans on texels surviving from an earlier step or frame.
type ChainStep = { pipe: number; bg: 'S2P' | 'P2S' | 'H2P'; skipT: number };
function buildChain(passes: number, history: boolean): ChainStep[] {
  const steps: ChainStep[] = [];
  let at: 'S' | 'P' | 'H' = history ? 'H' : 'S'; let travel = 0;
  const step = (pipe: number, bg: ChainStep['bg']) => { travel += STRIDES[pipe]; steps.push({ pipe, bg, skipT: skipThreshold(travel) }); };
  const n = Math.max(history ? 1 : 0, Math.min(passes, 6));       // with history at least one pass must move H → … → S
  for (let i = 0; i < n; i++) {
    if (at === 'H') { step(i, 'H2P'); at = 'P'; }
    else if (at === 'S') { step(i, 'S2P'); at = 'P'; }
    else { step(i, 'P2S'); at = 'S'; }
  }
  if (at === 'P') step(6, 'P2S');                                   // settle home (stride 1)
  return steps;
}
/** the chain-wide threshold: the last step's (1+2+4+1 = 8 px → 2 for the default 3 passes + history; 1+2+4+8+16+1 = 32 px → 5 at 6 passes) */
const chainSkipT = (chain: ChainStep[]) => chain.length ? chain[chain.length - 1].skipT : 2;
/** every threshold a (passes, history) setting can produce ({2, 3, 5} today) — one tileDistances pipeline each (threshold and scan radius are consts in the module) */
const ALL_SKIP_T = [...new Set([false, true].flatMap(h => [0, 1, 2, 3, 4, 5, 6].map(n => chainSkipT(buildChain(n, h)))))].sort((a, b) => a - b);

export class LightingPasses {
  width = 0; height = 0;
  /** low-res target sizes: the final gather (indirect, its history and à-trous ping-pong) at 1/gatherDiv, the volumetrics at 1/volDiv */
  gw = 0; gh = 0; vw = 0; vh = 0;
  /** resolution divisors: gather 2, or 3 with the lossy `gatherThird` option; volumetrics 2, or 4 with `volQuarter` (setDivisors; the shaders read them from frame.lossyCfg.xy) */
  gatherDiv = 2; volDiv = 2;
  // direct lighting, two emitter classes (0 sharp, 1 broad): S = shadowed estimate + blur hint (and the final result), U = exact
  // unshadowed; one shared ping-pong scratch P (classes are denoised one after the other); H = per-class temporal history
  // (ping-pong) + one shared view-depth history for validating it
  dS: GPUTexture[] = []; dU: GPUTexture[] = []; dP: GPUTexture[] = []; dSView: GPUTextureView[] = []; dUView: GPUTextureView[] = []; dPView: GPUTextureView[] = [];
  dH: GPUTexture[][] = [[], []]; dHView: GPUTextureView[][] = [[], []]; dDepthHist: GPUTexture[] = []; dDepthHistView: GPUTextureView[] = []; dParity = 0;
  /** live knobs mirrored from engine settings each frame (passes 0-6, history weight) */
  directPasses = 6; directHistory = 0.7;
  /** tile pass-through mask on the chain's 8x8 grid (tw x th): per-tile hint flag, then (tile distance to the nearest flagged tile | skip bit 256) under the chain-wide threshold */
  tileFlag: GPUTexture | null = null; tileDist: GPUTexture | null = null; tileFlagView!: GPUTextureView; tileDistView!: GPUTextureView; tw = 0; th = 0;
  /** threshold the mask was last built with (softTileStats reports against it) */
  private lastSkipT = 0;
  indirect!: GPUTexture; vol!: GPUTexture; ldr!: GPUTexture; indHist: GPUTexture[] = []; indHistView: GPUTextureView[] = []; parity = 0;
  /** half-res ping-pong for the indirect à-trous (history stays unblurred in indHist) */
  gi: GPUTexture[] = []; giView: GPUTextureView[] = [];
  /** HDR scene colour (composite output) + bloom pyramid: down[0..4] (W/2 … W/32), up[0..3] */
  hdr!: GPUTexture; hdrView!: GPUTextureView; bloomDown: GPUTexture[] = []; bloomUp: GPUTexture[] = []; bloomDownView: GPUTextureView[] = []; bloomUpView: GPUTextureView[] = [];
  /** post look: bloom strength (fraction of the blurred frame added back) and AgX look saturation */
  bloom = 0.06; saturation = 1.15; grain = 0.022; nvPhosphor = 0.09;
  indirectView!: GPUTextureView; volView!: GPUTextureView; ldrView!: GPUTextureView;
  /** what consumers bind as 'direct' (sharp class, denoised) and 'soft' (broad class, denoised): the chain always ends in S */
  get directView(): GPUTextureView { return this.dSView[0]; }
  get softView(): GPUTextureView { return this.dSView[1]; }
  private lDirect: GPUBindGroupLayout; private lVol: GPUBindGroupLayout; private lComp: GPUBindGroupLayout; private lFxaa: GPUBindGroupLayout; private lSoft: GPUBindGroupLayout; private lTileFlags: GPUBindGroupLayout; private lTileDist: GPUBindGroupLayout; private lTemp: GPUBindGroupLayout; private lDTemp: GPUBindGroupLayout; private lFG: GPUBindGroupLayout; private lGiBlur: GPUBindGroupLayout; private lBloom: GPUBindGroupLayout; private lPost: GPUBindGroupLayout;
  private pDirect: GPUComputePipeline; private pVol: GPUComputePipeline; private pComp: GPURenderPipeline; private pFxaa: GPURenderPipeline; private pDen: GPUComputePipeline[] = []; private pTileFlags: GPUComputePipeline; private pTileDist = new Map<number, GPUComputePipeline>(); private pTemp: GPUComputePipeline; private pDTemp: GPUComputePipeline[] = []; private pFG: GPUComputePipeline; private pGiBlur: GPUComputePipeline[] = []; private pBloomDown0: GPUComputePipeline; private pBloomDown: GPUComputePipeline; private pBloomUp: GPUComputePipeline; private pPost: GPURenderPipeline; private postParams: GPUBuffer;
  private bgDirect: GPUBindGroup | null = null; private bgVol: GPUBindGroup | null = null; private bgFxaa: GPUBindGroup | null = null; private bgDen: GPUBindGroup[] = []; private bgDenH: GPUBindGroup[] = []; private bgTileFlags: GPUBindGroup | null = null; private bgTileDist: GPUBindGroup | null = null; private bgDTemp: GPUBindGroup[] = []; private bgTemp: (GPUBindGroup | null)[] = [null, null];
  private bgFG: GPUBindGroup | null = null; private bgGiBlur: (GPUBindGroup | null)[][] = [[], []]; private bgCompFG: GPUBindGroup | null = null;
  private bgBloomDown: GPUBindGroup[] = []; private bgBloomUp: GPUBindGroup[] = []; private bgPost: GPUBindGroup | null = null;
  private fxaaParams: GPUBuffer;
  fxaaEnabled = true;

  constructor(private device: GPUDevice, sceneLayout: GPUBindGroupLayout, private canvasFormat: GPUTextureFormat, private linSamp: GPUSampler) {
    const pre = scenePrelude();
    const tex2d = (sampleType: GPUTextureSampleType = 'float'): GPUBindGroupLayoutEntry['texture'] => ({ sampleType, viewDimension: '2d' });
    this.lDirect = device.createBindGroupLayout({ label: 'direct', entries: [
      { binding: 0, visibility: CS, texture: tex2d('depth') },
      { binding: 1, visibility: CS, texture: tex2d('float') },
      { binding: 2, visibility: CS, texture: tex2d('uint') },
      { binding: 3, visibility: CS, storageTexture: { access: 'write-only', format: 'rgba16float', viewDimension: '2d' } },
      { binding: 4, visibility: CS, storageTexture: { access: 'write-only', format: 'rgba16float', viewDimension: '2d' } },
      { binding: 5, visibility: CS, storageTexture: { access: 'write-only', format: 'rgba16float', viewDimension: '2d' } },
      { binding: 6, visibility: CS, storageTexture: { access: 'write-only', format: 'rgba16float', viewDimension: '2d' } },
    ] });
    const stor16: GPUStorageTextureBindingLayout = { access: 'write-only', format: 'rgba16float', viewDimension: '2d' };
    this.lSoft = device.createBindGroupLayout({ label: 'softblur', entries: [   // both emitter classes in one dispatch
      { binding: 0, visibility: CS, texture: tex2d('depth') }, { binding: 1, visibility: CS, texture: tex2d('float') },
      { binding: 2, visibility: CS, texture: tex2d('unfilterable-float') }, { binding: 3, visibility: CS, texture: tex2d('unfilterable-float') },
      { binding: 4, visibility: CS, storageTexture: stor16 }, { binding: 5, visibility: CS, storageTexture: stor16 },
      { binding: 6, visibility: CS, texture: tex2d('unfilterable-float') }, { binding: 7, visibility: CS, texture: tex2d('unfilterable-float') },
      { binding: 8, visibility: CS, texture: tex2d('uint') },   // tile mask (softtile.wgsl): bit 256 = this tile passes every step of the chain through
    ] });
    const storU32: GPUStorageTextureBindingLayout = { access: 'write-only', format: 'r32uint', viewDimension: '2d' };
    this.lTileFlags = device.createBindGroupLayout({ label: 'softtileFlags', entries: [   // S0, S1 → per-tile hint flag
      { binding: 0, visibility: CS, texture: tex2d('unfilterable-float') }, { binding: 1, visibility: CS, texture: tex2d('unfilterable-float') }, { binding: 2, visibility: CS, storageTexture: storU32 } ] });
    this.lTileDist = device.createBindGroupLayout({ label: 'softtileDist', entries: [     // flags → per-tile distance (own group: the flag texture is sampled here, written there)
      { binding: 3, visibility: CS, texture: tex2d('uint') }, { binding: 4, visibility: CS, storageTexture: storU32 } ] });
    this.lDTemp = device.createBindGroupLayout({ label: 'dtemporal', entries: [   // both classes + the shared depth history in one dispatch
      { binding: 0, visibility: CS, texture: tex2d('depth') },
      { binding: 1, visibility: CS, texture: tex2d('unfilterable-float') }, { binding: 2, visibility: CS, texture: tex2d('unfilterable-float') },
      { binding: 3, visibility: CS, texture: tex2d('float') }, { binding: 4, visibility: CS, texture: tex2d('float') },
      { binding: 5, visibility: CS, texture: tex2d('unfilterable-float') },
      { binding: 6, visibility: CS, storageTexture: stor16 }, { binding: 7, visibility: CS, storageTexture: stor16 },
      { binding: 8, visibility: CS, storageTexture: { access: 'write-only', format: 'r32float', viewDimension: '2d' } },
    ] });
    this.lTemp = device.createBindGroupLayout({ label: 'temporal', entries: [
      { binding: 0, visibility: CS, texture: tex2d('depth') },
      { binding: 1, visibility: CS, texture: tex2d('unfilterable-float') },
      { binding: 2, visibility: CS, texture: tex2d('float') },
      { binding: 3, visibility: CS, storageTexture: { access: 'write-only', format: 'rgba16float', viewDimension: '2d' } },
    ] });
    this.lFG = device.createBindGroupLayout({ label: 'fgather', entries: [
      { binding: 0, visibility: CS, texture: tex2d('depth') },
      { binding: 1, visibility: CS, texture: tex2d('float') },
      { binding: 2, visibility: CS, texture: tex2d('uint') },
      { binding: 3, visibility: CS, texture: tex2d('unfilterable-float') },
      { binding: 4, visibility: CS, texture: tex2d('unfilterable-float') },
      { binding: 5, visibility: CS, texture: { sampleType: 'float', viewDimension: '3d' } },
      { binding: 6, visibility: CS, storageTexture: { access: 'write-only', format: 'rgba16float', viewDimension: '2d' } },
    ] });
    this.lGiBlur = device.createBindGroupLayout({ label: 'giblur', entries: [
      { binding: 0, visibility: CS, texture: tex2d('depth') },
      { binding: 1, visibility: CS, texture: tex2d('float') },
      { binding: 2, visibility: CS, texture: tex2d('unfilterable-float') },
      { binding: 3, visibility: CS, storageTexture: { access: 'write-only', format: 'rgba16float', viewDimension: '2d' } },
    ] });
    this.lBloom = device.createBindGroupLayout({ label: 'bloom', entries: [
      { binding: 0, visibility: CS, texture: tex2d('float') },
      { binding: 1, visibility: CS, storageTexture: { access: 'write-only', format: 'rgba16float', viewDimension: '2d' } },
      { binding: 2, visibility: CS, texture: tex2d('unfilterable-float') },
    ] });
    this.lPost = device.createBindGroupLayout({ label: 'post', entries: [
      { binding: 0, visibility: FS, texture: tex2d('unfilterable-float') },
      { binding: 1, visibility: FS, texture: tex2d('float') },
      { binding: 2, visibility: FS, buffer: { type: 'uniform' } },
    ] });
    this.lVol = device.createBindGroupLayout({ label: 'vol', entries: [
      { binding: 0, visibility: CS, texture: tex2d('depth') },
      { binding: 1, visibility: CS, texture: { sampleType: 'float', viewDimension: '3d' } },
      { binding: 2, visibility: CS, storageTexture: { access: 'write-only', format: 'rgba16float', viewDimension: '2d' } },
    ] });
    this.lComp = device.createBindGroupLayout({ label: 'composite', entries: [
      { binding: 0, visibility: FS, texture: tex2d('float') },
      { binding: 1, visibility: FS, texture: tex2d('float') },
      { binding: 2, visibility: FS, texture: tex2d('depth') },
      { binding: 3, visibility: FS, texture: tex2d('uint') },
      { binding: 4, visibility: FS, texture: tex2d('unfilterable-float') },
      { binding: 5, visibility: FS, texture: tex2d('unfilterable-float') },
      { binding: 6, visibility: FS, texture: tex2d('unfilterable-float') },
      { binding: 7, visibility: FS, texture: { sampleType: 'float', viewDimension: '3d' } },
      { binding: 8, visibility: FS, texture: tex2d('unfilterable-float') },
    ] });
    this.lFxaa = device.createBindGroupLayout({ label: 'fxaa', entries: [
      { binding: 0, visibility: FS, texture: tex2d('float') },
      { binding: 1, visibility: FS, sampler: { type: 'filtering' } },
      { binding: 2, visibility: FS, buffer: { type: 'uniform' } },
    ] });
    const mk = (src: string, label: string) => makeShader(device, pre + lightingCommon + src, label);
    this.pDirect = device.createComputePipeline({ label: 'direct', layout: device.createPipelineLayout({ bindGroupLayouts: [sceneLayout, this.lDirect] }), compute: { module: makeShader(device, pre + directSrc, 'direct'), entryPoint: 'main' } });
    this.pTemp = device.createComputePipeline({ label: 'temporal', layout: device.createPipelineLayout({ bindGroupLayouts: [sceneLayout, this.lTemp] }), compute: { module: makeShader(device, pre + temporalSrc, 'temporal'), entryPoint: 'main' } });
    { const mod = makeShader(device, pre + softblurSrc, 'softblur'); const lay = device.createPipelineLayout({ bindGroupLayouts: [sceneLayout, this.lSoft] });
      STRIDES.forEach((stride, pass) => this.pDen.push(device.createComputePipeline({ label: `denoise${pass}s${stride}`, layout: lay, compute: { module: mod, entryPoint: 'main', constants: { STRIDE: stride } } }))); }   // (only STRIDE is an override in the shader: an unused override constant fails pipeline creation on WebKit)
    { // the chain's tile pass-through mask: one tiny module per threshold the settings can produce (2, 3, 5 today) with SKIP_T and its scan
      // radius baked in as consts — the mask texel itself carries the decision, so softblur needs nothing but that texel and the frame flag.
      // No scene prelude: neither entry point touches group 0 (the layouts still list it so bind group 0 can stay put across the pass).
      const layF = device.createPipelineLayout({ bindGroupLayouts: [sceneLayout, this.lTileFlags] }), layD = device.createPipelineLayout({ bindGroupLayouts: [sceneLayout, this.lTileDist] });
      const mods = ALL_SKIP_T.map(T => makeShader(device, `const SKIP_T: i32 = ${T};\nconst SCAN_R: i32 = ${T - 1};\n` + softtileSrc, `softtile${T}`));
      this.pTileFlags = device.createComputePipeline({ label: 'softtileFlags', layout: layF, compute: { module: mods[0], entryPoint: 'markTiles' } });   // (markTiles is the same in every module)
      ALL_SKIP_T.forEach((T, i) => this.pTileDist.set(T, device.createComputePipeline({ label: `softtileDist${T}`, layout: layD, compute: { module: mods[i], entryPoint: 'tileDistances' } }))); }
    this.pDTemp.push(device.createComputePipeline({ label: 'dtemporal', layout: device.createPipelineLayout({ bindGroupLayouts: [sceneLayout, this.lDTemp] }), compute: { module: makeShader(device, pre + dtemporalSrc, 'dtemporal'), entryPoint: 'main' } }));
    this.pFG = device.createComputePipeline({ label: 'fgather', layout: device.createPipelineLayout({ bindGroupLayouts: [sceneLayout, this.lFG] }), compute: { module: mk(fgatherSrc, 'fgather'), entryPoint: 'main' } });
    { const mod = makeShader(device, pre + giblurSrc, 'giblur'); const lay = device.createPipelineLayout({ bindGroupLayouts: [sceneLayout, this.lGiBlur] });
      for (const stride of [1, 2, 4]) this.pGiBlur.push(device.createComputePipeline({ label: `giblur${stride}`, layout: lay, compute: { module: mod, entryPoint: 'main', constants: { STRIDE: stride } } })); }
    this.pVol = device.createComputePipeline({ label: 'vol', layout: device.createPipelineLayout({ bindGroupLayouts: [sceneLayout, this.lVol] }), compute: { module: mk(volSrc, 'volumetrics'), entryPoint: 'main' } });
    const compMod = mk(compositeSrc, 'composite');
    this.pComp = device.createRenderPipeline({ label: 'composite', layout: device.createPipelineLayout({ bindGroupLayouts: [sceneLayout, this.lComp] }),
      vertex: { module: compMod, entryPoint: 'vs' }, fragment: { module: compMod, entryPoint: 'fs', targets: [{ format: 'rgba16float' }] }, primitive: { topology: 'triangle-list' } });
    { const mod = makeShader(device, pre + bloomSrc, 'bloom'); const lay = device.createPipelineLayout({ bindGroupLayouts: [sceneLayout, this.lBloom] });
      this.pBloomDown0 = device.createComputePipeline({ label: 'bloomDown0', layout: lay, compute: { module: mod, entryPoint: 'down', constants: { FIRST: 1 } } });
      this.pBloomDown = device.createComputePipeline({ label: 'bloomDown', layout: lay, compute: { module: mod, entryPoint: 'down', constants: { FIRST: 0 } } });
      this.pBloomUp = device.createComputePipeline({ label: 'bloomUp', layout: lay, compute: { module: mod, entryPoint: 'up' } }); }
    { const mod = makeShader(device, pre + postSrc, 'post');
      this.pPost = device.createRenderPipeline({ label: 'post', layout: device.createPipelineLayout({ bindGroupLayouts: [sceneLayout, this.lPost] }),
        vertex: { module: mod, entryPoint: 'vs' }, fragment: { module: mod, entryPoint: 'fs', targets: [{ format: 'rgba8unorm' }] }, primitive: { topology: 'triangle-list' } }); }
    this.postParams = makeBuffer(device, 16, GPUBufferUsage.UNIFORM | GPUBufferUsage.COPY_DST, 'postParams');
    const fxMod = makeShader(device, fxaaSrc, 'fxaa');
    this.pFxaa = device.createRenderPipeline({ label: 'fxaa', layout: device.createPipelineLayout({ bindGroupLayouts: [this.lFxaa] }),
      vertex: { module: fxMod, entryPoint: 'vs' }, fragment: { module: fxMod, entryPoint: 'fs', targets: [{ format: canvasFormat }] }, primitive: { topology: 'triangle-list' } });
    this.fxaaParams = makeBuffer(device, 32, GPUBufferUsage.UNIFORM | GPUBufferUsage.COPY_DST, 'fxaaParams');
  }

  /** Lossy-option divisors (engine.render mirrors the settings every frame): a change recreates the sized targets — same path as a window resize. */
  setDivisors(gatherDiv: number, volDiv: number) {
    if (gatherDiv === this.gatherDiv && volDiv === this.volDiv) return;
    this.gatherDiv = gatherDiv; this.volDiv = volDiv;
    if (this.width) { const w = this.width, h = this.height; this.width = 0; this.resize(w, h); }
  }

  resize(w: number, h: number) {
    if (w === this.width && h === this.height) return;
    this.width = w; this.height = h; this.gw = Math.ceil(w / this.gatherDiv); this.gh = Math.ceil(h / this.gatherDiv); this.vw = Math.ceil(w / this.volDiv); this.vh = Math.ceil(h / this.volDiv);
    for (const t of [this.indirect, this.vol, this.ldr, ...this.dP, ...this.dS, ...this.dU]) t?.destroy();
    const su = GPUTextureUsage.STORAGE_BINDING | GPUTextureUsage.TEXTURE_BINDING;
    this.dS = []; this.dU = []; this.dP = [];
    for (let c = 0; c < 2; c++) {
      this.dS.push(this.device.createTexture({ label: `directS${c}`, size: [w, h], format: 'rgba16float', usage: su }));
      this.dU.push(this.device.createTexture({ label: `directU${c}`, size: [w, h], format: 'rgba16float', usage: su }));
      this.dP.push(this.device.createTexture({ label: `directP${c}`, size: [w, h], format: 'rgba16float', usage: su }));   // ping-pong partner per class (both classes are filtered in the same dispatch)
    }
    this.dSView = this.dS.map(t => t.createView()); this.dUView = this.dU.map(t => t.createView()); this.dPView = this.dP.map(t => t.createView());
    this.tileFlag?.destroy(); this.tileDist?.destroy();
    this.tw = Math.ceil(w / 8); this.th = Math.ceil(h / 8);   // = the softblur dispatch grid: one texel per workgroup
    this.tileFlag = this.device.createTexture({ label: 'softTileFlag', size: [this.tw, this.th], format: 'r32uint', usage: su });
    this.tileDist = this.device.createTexture({ label: 'softTileDist', size: [this.tw, this.th], format: 'r32uint', usage: su | GPUTextureUsage.COPY_SRC });   // (COPY_SRC: softTileStats reads it back)
    this.tileFlagView = this.tileFlag.createView(); this.tileDistView = this.tileDist.createView();
    for (const t of [...this.dH[0], ...this.dH[1], ...this.dDepthHist]) t.destroy();
    this.dH = [0, 1].map(c => [0, 1].map(i => this.device.createTexture({ label: `directH${c}${i}`, size: [w, h], format: 'rgba16float', usage: su })));
    this.dHView = this.dH.map(a => a.map(t => t.createView()));
    this.dDepthHist = [0, 1].map(i => this.device.createTexture({ label: `directDepthHist${i}`, size: [w, h], format: 'r32float', usage: su }));
    this.dDepthHistView = this.dDepthHist.map(t => t.createView());
    for (const t of this.indHist) t.destroy();
    this.indHist = [0, 1].map(i => this.device.createTexture({ label: `indHist${i}`, size: [this.gw, this.gh], format: 'rgba16float', usage: su }));
    this.indHistView = this.indHist.map(t => t.createView());
    this.indirect = this.device.createTexture({ label: 'indirect', size: [this.gw, this.gh], format: 'rgba16float', usage: su });
    for (const t of this.gi) t.destroy();
    this.gi = [0, 1].map(i => this.device.createTexture({ label: `gi${i}`, size: [this.gw, this.gh], format: 'rgba16float', usage: su })); this.giView = this.gi.map(t => t.createView());
    this.vol = this.device.createTexture({ label: 'vol', size: [this.vw, this.vh], format: 'rgba16float', usage: su });
    this.ldr = this.device.createTexture({ label: 'ldr', size: [w, h], format: 'rgba8unorm', usage: GPUTextureUsage.RENDER_ATTACHMENT | GPUTextureUsage.TEXTURE_BINDING });
    this.hdr?.destroy(); for (const t of [...this.bloomDown, ...this.bloomUp]) t.destroy();
    this.hdr = this.device.createTexture({ label: 'hdr', size: [w, h], format: 'rgba16float', usage: GPUTextureUsage.RENDER_ATTACHMENT | GPUTextureUsage.TEXTURE_BINDING }); this.hdrView = this.hdr.createView();
    this.bloomDown = []; this.bloomUp = [];
    for (let i = 0; i < 5; i++) { const bw = Math.max(1, Math.ceil(w / (2 << i))), bh = Math.max(1, Math.ceil(h / (2 << i))); this.bloomDown.push(this.device.createTexture({ label: `bloomD${i}`, size: [bw, bh], format: 'rgba16float', usage: su })); if (i < 4) this.bloomUp.push(this.device.createTexture({ label: `bloomU${i}`, size: [bw, bh], format: 'rgba16float', usage: su })); }
    this.bloomDownView = this.bloomDown.map(t => t.createView()); this.bloomUpView = this.bloomUp.map(t => t.createView());
    this.indirectView = this.indirect.createView(); this.volView = this.vol.createView(); this.ldrView = this.ldr.createView();
    this.bgDirect = this.bgVol = this.bgFxaa = null; this.bgDen = []; this.bgDenH = []; this.bgTileFlags = this.bgTileDist = null; this.bgDTemp = []; this.bgTemp = [null, null]; this.bgFG = null; this.bgGiBlur = [[], []]; this.bgCompFG = null; this.bgBloomDown = []; this.bgBloomUp = []; this.bgPost = null;
    this.writeFxaaParams();
  }

  setFxaa(on: boolean) { this.fxaaEnabled = on; if (this.width) this.writeFxaaParams(); }
  /** Test hook (main.ts __resetClock): zero the per-frame counters that feed the grain seed and the two history parities, so a scripted capture renders
   *  the same frames whatever ran during boot. */
  resetClocks() { this.grainSeed = 0; this.dParity = 0; this.parity = 0; }
  private grainSeed = 0;
  private writeFxaaParams() { this.device.queue.writeBuffer(this.fxaaParams, 0, new Float32Array([1 / this.width, 1 / this.height, this.fxaaEnabled ? 1 : 0, this.grain, this.grainSeed, 0, 0, 0])); }

  private ensureBindGroups(g: GBuffer, rc: RadianceCascades) {
    if (this.bgDirect) return;
    const d = this.device;
    this.bgDirect = d.createBindGroup({ layout: this.lDirect, entries: [
      { binding: 0, resource: g.depthView }, { binding: 1, resource: g.normalView }, { binding: 2, resource: g.idView },
      { binding: 3, resource: this.dSView[0] }, { binding: 4, resource: this.dUView[0] }, { binding: 5, resource: this.dSView[1] }, { binding: 6, resource: this.dUView[1] },
    ] });
    // denoise bind groups (both classes per dispatch): bgDen[0] = S→P, bgDen[1] = P→S, bgDenH[hp] = H[hp]→P; every one also carries the
    // tile mask. Mask builders: bgTileFlags (S0, S1 → tileFlag), bgTileDist (tileFlag → tileDist).
    // temporal bgDTemp[hp]: (S0, S1, H0[1-hp], H1[1-hp], depthHist[1-hp]) → (H0[hp], H1[hp], depthHist[hp])
    const den = (s0: GPUTextureView, s1: GPUTextureView, d0: GPUTextureView, d1: GPUTextureView) => d.createBindGroup({ layout: this.lSoft, entries: [
      { binding: 0, resource: g.depthView }, { binding: 1, resource: g.normalView }, { binding: 2, resource: s0 }, { binding: 3, resource: s1 },
      { binding: 4, resource: d0 }, { binding: 5, resource: d1 }, { binding: 6, resource: this.dUView[0] }, { binding: 7, resource: this.dUView[1] },
      { binding: 8, resource: this.tileDistView } ] });
    this.bgDen = [den(this.dSView[0], this.dSView[1], this.dPView[0], this.dPView[1]), den(this.dPView[0], this.dPView[1], this.dSView[0], this.dSView[1])];
    this.bgDenH = [0, 1].map(hp => den(this.dHView[0][hp], this.dHView[1][hp], this.dPView[0], this.dPView[1]));
    this.bgTileFlags = d.createBindGroup({ layout: this.lTileFlags, entries: [ { binding: 0, resource: this.dSView[0] }, { binding: 1, resource: this.dSView[1] }, { binding: 2, resource: this.tileFlagView } ] });
    this.bgTileDist = d.createBindGroup({ layout: this.lTileDist, entries: [ { binding: 3, resource: this.tileFlagView }, { binding: 4, resource: this.tileDistView } ] });
    this.bgDTemp = [0, 1].map(hp => d.createBindGroup({ layout: this.lDTemp, entries: [
      { binding: 0, resource: g.depthView }, { binding: 1, resource: this.dSView[0] }, { binding: 2, resource: this.dSView[1] },
      { binding: 3, resource: this.dHView[0][1 - hp] }, { binding: 4, resource: this.dHView[1][1 - hp] }, { binding: 5, resource: this.dDepthHistView[1 - hp] },
      { binding: 6, resource: this.dHView[0][hp] }, { binding: 7, resource: this.dHView[1][hp] }, { binding: 8, resource: this.dDepthHistView[hp] } ] }));
    this.bgVol = d.createBindGroup({ layout: this.lVol, entries: [
      { binding: 0, resource: g.depthView }, { binding: 1, resource: rc.diceView }, { binding: 2, resource: this.volView },
    ] });
    for (let pz = 0; pz < 2; pz++) {
      // parity pz: temporal writes indHist[pz] from (raw indirect, indHist[1-pz]); the à-trous chain then reads indHist[pz]
      this.bgTemp[pz] = d.createBindGroup({ layout: this.lTemp, entries: [
        { binding: 0, resource: g.depthView }, { binding: 1, resource: this.indirectView }, { binding: 2, resource: this.indHistView[1 - pz] }, { binding: 3, resource: this.indHistView[pz] },
      ] });
    }
    // final gather: fgather → indirect; temporal → indHist[pz]; à-trous indHist[pz] → gi0 → gi1 → gi0; composite reads gi0
    this.bgFG = d.createBindGroup({ layout: this.lFG, entries: [
      { binding: 0, resource: g.depthView }, { binding: 1, resource: g.normalView }, { binding: 2, resource: g.idView }, { binding: 3, resource: this.directView }, { binding: 4, resource: this.softView },
      { binding: 5, resource: rc.diceView }, { binding: 6, resource: this.indirectView },
    ] });
    const gb = (src: GPUTextureView, dst: GPUTextureView) => d.createBindGroup({ layout: this.lGiBlur, entries: [ { binding: 0, resource: g.depthView }, { binding: 1, resource: g.normalView }, { binding: 2, resource: src }, { binding: 3, resource: dst } ] });
    for (let pz = 0; pz < 2; pz++) this.bgGiBlur[pz] = [gb(this.indHistView[pz], this.giView[0]), gb(this.giView[0], this.giView[1]), gb(this.giView[1], this.giView[0])];
    this.bgCompFG = d.createBindGroup({ layout: this.lComp, entries: [
      { binding: 0, resource: g.albedoView }, { binding: 1, resource: g.normalView }, { binding: 2, resource: g.depthView }, { binding: 3, resource: g.idView },
      { binding: 4, resource: this.directView }, { binding: 5, resource: this.giView[0] }, { binding: 6, resource: this.volView }, { binding: 7, resource: rc.diceView }, { binding: 8, resource: this.softView },
    ] });
    // bloom: down chain hdr→D0→D1→D2→D3→D4, up chain U3 = D3+tent(D4), U2 = D2+tent(U3), U1 = D1+tent(U2), U0 = D0+tent(U1)
    const bl = (src: GPUTextureView, dst: GPUTextureView, add: GPUTextureView) => d.createBindGroup({ layout: this.lBloom, entries: [ { binding: 0, resource: src }, { binding: 1, resource: dst }, { binding: 2, resource: add } ] });
    this.bgBloomDown = [bl(this.hdrView, this.bloomDownView[0], this.hdrView)];
    for (let i = 1; i < 5; i++) this.bgBloomDown.push(bl(this.bloomDownView[i - 1], this.bloomDownView[i], this.hdrView));
    this.bgBloomUp = [];
    for (let i = 3; i >= 0; i--) this.bgBloomUp.push(bl(i === 3 ? this.bloomDownView[4] : this.bloomUpView[i + 1], this.bloomUpView[i], this.bloomDownView[i]));
    this.bgPost = d.createBindGroup({ layout: this.lPost, entries: [ { binding: 0, resource: this.hdrView }, { binding: 1, resource: this.bloomUpView[0] }, { binding: 2, resource: { buffer: this.postParams } } ] });
    this.bgFxaa = d.createBindGroup({ layout: this.lFxaa, entries: [
      { binding: 0, resource: this.ldrView }, { binding: 1, resource: this.linSamp }, { binding: 2, resource: { buffer: this.fxaaParams } },
    ] });
  }

  /** Measurement aid (console: `await __engine.passes.softTileStats()`): reads back the tile mask of the last frame rendered with
   *  settings.softTileSkip on and reports the threshold it was built with, the fraction of 8x8 tiles the whole chain passes through (≈ the
   *  fraction of the chain's per-pixel work saved; a retired tile costs one broadcast texel load + a 2-texel copy per step) and the tile
   *  count per distance 0 .. skipT (the scan saturates at skipT: "at least that far"). */
  async softTileStats(): Promise<{ tiles: number; skipT: number; skipped: number; byDistance: number[]; steps: { stride: number; dir: string; ownT: number }[] } | null> {
    if (!this.tileDist || !this.lastSkipT) return null;
    const tw = this.tw, th = this.th, bpr = Math.ceil(tw * 4 / 256) * 256, T = this.lastSkipT;
    const buf = this.device.createBuffer({ size: bpr * th, usage: GPUBufferUsage.COPY_DST | GPUBufferUsage.MAP_READ });
    const enc = this.device.createCommandEncoder(); enc.copyTextureToBuffer({ texture: this.tileDist }, { buffer: buf, bytesPerRow: bpr }, [tw, th]); this.device.queue.submit([enc.finish()]);
    await buf.mapAsync(GPUMapMode.READ);
    const u = new Uint32Array(buf.getMappedRange().slice(0)); buf.unmap(); buf.destroy();
    const byDistance = new Array<number>(T + 1).fill(0); let skipped = 0;
    for (let y = 0; y < th; y++) for (let x = 0; x < tw; x++) { const v = u[y * (bpr / 4) + x]; byDistance[Math.min(v & 255, T)]++; if (v & 256) skipped++; }
    const steps = buildChain(this.directPasses, this.directHistory > 0).map(st => ({ stride: STRIDES[st.pipe], dir: st.bg, ownT: st.skipT }));
    return { tiles: tw * th, skipT: T, skipped: skipped / (tw * th), byDistance, steps };
  }

  /** RC/gbuffer resources were recreated → drop cached bind groups. */
  invalidate() { this.bgDirect = this.bgVol = this.bgFxaa = null; this.bgDen = []; this.bgDenH = []; this.bgTileFlags = this.bgTileDist = null; this.bgDTemp = []; this.bgTemp = [null, null]; this.bgFG = null; this.bgGiBlur = [[], []]; this.bgCompFG = null; this.bgBloomDown = []; this.bgBloomUp = []; this.bgPost = null; }

  encode(enc: GPUCommandEncoder, sceneBG: GPUBindGroup, g: GBuffer, rc: RadianceCascades, canvasView: GPUTextureView, ts: (name: string) => GPUComputePassTimestampWrites | undefined, frameFlags = 0xffffffff) {
    this.ensureBindGroups(g, rc);
    {
      const p = enc.beginComputePass({ label: 'direct', timestampWrites: ts('direct') });
      p.setPipeline(this.pDirect); p.setBindGroup(0, sceneBG); p.setBindGroup(1, this.bgDirect!);
      p.dispatchWorkgroups(Math.ceil(this.width / 8), Math.ceil(this.height / 8)); p.end();
    }
    // per class: optional temporal reuse (S → H) then the spatial chain, always ending in S; skipped entirely when direct light
    // or the denoiser is switched off — the raw S is then exactly what consumers should see
    if ((frameFlags & (FrameFlags.Direct | FrameFlags.SoftShadow)) === (FrameFlags.Direct | FrameFlags.SoftShadow)) {
      const useHist = this.directHistory > 0;
      const chain = buildChain(this.directPasses, useHist);
      const p = enc.beginComputePass({ label: 'softblur', timestampWrites: ts('softblur') });   // (the tile mask below is built inside this pass so its cost shows up in the same timing)
      const wx = Math.ceil(this.width / 8), wy = Math.ceil(this.height / 8);
      p.setBindGroup(0, sceneBG);
      if ((frameFlags & FrameFlags.SoftTileSkip) && chain.length > 0) {   // the mask, from THIS frame's S (the direct pass above has ended): per-tile hint flags, then distances + skip bit under the chain-wide threshold. Flag off: not built, and softblur never looks at it
        const T = this.pTileDist.has(chainSkipT(chain)) ? chainSkipT(chain) : ALL_SKIP_T[ALL_SKIP_T.length - 1]; this.lastSkipT = T;   // (every reachable chain's threshold has a pipeline; the widest is the safe fallback)
        p.setPipeline(this.pTileFlags); p.setBindGroup(1, this.bgTileFlags!); p.dispatchWorkgroups(wx, wy);
        p.setPipeline(this.pTileDist.get(T)!); p.setBindGroup(1, this.bgTileDist!); p.dispatchWorkgroups(Math.ceil(this.tw / 8), Math.ceil(this.th / 8));
      }
      if (useHist) this.dParity ^= 1;
      const hp = this.dParity;
      if (useHist) { p.setPipeline(this.pDTemp[0]); p.setBindGroup(1, this.bgDTemp[hp]); p.dispatchWorkgroups(wx, wy); }
      for (const st of chain) { p.setPipeline(this.pDen[st.pipe]); p.setBindGroup(1, st.bg === 'S2P' ? this.bgDen[0] : st.bg === 'P2S' ? this.bgDen[1] : this.bgDenH[hp]); p.dispatchWorkgroups(wx, wy); }
      p.end();
    }
    const hx = Math.ceil(this.gw / 8), hy = Math.ceil(this.gh / 8);   // gather-resolution dispatch (1/2, or 1/3 with gatherThird)
    {
      const p = enc.beginComputePass({ label: 'gather', timestampWrites: ts('gather') });
      p.setPipeline(this.pFG); p.setBindGroup(0, sceneBG); p.setBindGroup(1, this.bgFG!);
      p.dispatchWorkgroups(hx, hy); p.end();
    }
    this.parity ^= 1;
    {
      const p = enc.beginComputePass({ label: 'temporal', timestampWrites: ts('temporal') });
      p.setPipeline(this.pTemp); p.setBindGroup(0, sceneBG); p.setBindGroup(1, this.bgTemp[this.parity]!);
      p.dispatchWorkgroups(hx, hy);
      this.bgGiBlur[this.parity].forEach((bg, i) => { p.setPipeline(this.pGiBlur[i]); p.setBindGroup(1, bg!); p.dispatchWorkgroups(hx, hy); });
      p.end();
    }
    {
      const p = enc.beginComputePass({ label: 'volumetrics', timestampWrites: ts('volumetrics') });
      p.setPipeline(this.pVol); p.setBindGroup(0, sceneBG); p.setBindGroup(1, this.bgVol!);
      p.dispatchWorkgroups(Math.ceil(this.vw / 8), Math.ceil(this.vh / 8)); p.end();
    }
    {
      const p = enc.beginRenderPass({ label: 'composite', colorAttachments: [{ view: this.hdrView, loadOp: 'clear', storeOp: 'store', clearValue: { r: 0, g: 0, b: 0, a: 1 } }], timestampWrites: ts('composite') as GPURenderPassTimestampWrites | undefined });
      p.setPipeline(this.pComp); p.setBindGroup(0, sceneBG); p.setBindGroup(1, this.bgCompFG!); p.draw(3); p.end();
    }
    {
      this.device.queue.writeBuffer(this.postParams, 0, new Float32Array([this.bloom, this.saturation, 0, this.nvPhosphor]));
      this.grainSeed = (this.grainSeed + 1) % 4096; this.writeFxaaParams();   // grain animates per frame and is applied after FXAA
      const p = enc.beginComputePass({ label: 'bloom', timestampWrites: ts('bloom') });
      p.setBindGroup(0, sceneBG);
      if (this.bloom > 0) {
        for (let i = 0; i < 5; i++) { const t = this.bloomDown[i]; p.setPipeline(i === 0 ? this.pBloomDown0 : this.pBloomDown); p.setBindGroup(1, this.bgBloomDown[i]); p.dispatchWorkgroups(Math.ceil(t.width / 8), Math.ceil(t.height / 8)); }
        for (let k = 0; k < 4; k++) { const t = this.bloomUp[3 - k]; p.setPipeline(this.pBloomUp); p.setBindGroup(1, this.bgBloomUp[k]); p.dispatchWorkgroups(Math.ceil(t.width / 8), Math.ceil(t.height / 8)); }
      }
      p.end();
    }
    {
      const p = enc.beginRenderPass({ label: 'post', colorAttachments: [{ view: this.ldrView, loadOp: 'clear', storeOp: 'store', clearValue: { r: 0, g: 0, b: 0, a: 1 } }], timestampWrites: ts('post') as GPURenderPassTimestampWrites | undefined });
      p.setPipeline(this.pPost); p.setBindGroup(0, sceneBG); p.setBindGroup(1, this.bgPost!); p.draw(3); p.end();
    }
    {
      const p = enc.beginRenderPass({ label: 'fxaa', colorAttachments: [{ view: canvasView, loadOp: 'clear', storeOp: 'store', clearValue: { r: 0, g: 0, b: 0, a: 1 } }], timestampWrites: ts('fxaa') as GPURenderPassTimestampWrites | undefined });
      p.setPipeline(this.pFxaa); p.setBindGroup(0, this.bgFxaa!); p.draw(3); p.end();
    }
  }
}
