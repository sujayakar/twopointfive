// Renderer orchestration: owns GPU resources and encodes the frame.
import { Gpu, makeBuffer } from './gpu/device';
import { GpuTimer } from './gpu/timer';
import { BoxWorld } from './scene/boxes';
import { FRAME_BYTES, FrameFlags, FrameParams, createSceneBindGroup, createSceneLayout, writeFrame } from './render/frame';
import { GBuffer, SkinnedDrawable } from './render/gbuffer';
import { RadianceCascades, RcConfig, defaultRcConfig } from './render/rc';
import { LightingPasses } from './render/passes';
import { LightSet } from './render/lights';
import { FollowCamera } from './game/camera';
import { Vec3 } from './math/vec';

export interface RenderSettings {
  renderScale: number;      // internal resolution relative to CSS pixels * devicePixelRatio
  maxInternalHeight: number; // cap on internal render height in pixels (keeps cost bounded on big windows)
  exposure: number;
  flags: number;
  debugView: number;
  hazeDensity: number;
  indirectScale: number;
  emissiveScale: number;
  volSteps: number;
  skyZenith: Vec3; skyHorizon: Vec3;
  capColor: [number, number, number, number]; // section cap tint (rgb) + albedo mix
  nvAmount: number; nvGain: number; hitFlash: number;
  // direct light sampling / reuse knobs (see direct.wgsl, dtemporal.wgsl, softblur.wgsl)
  directSamplesTop: number;    // stratified emitter samples for each class's strongest light (1-8)
  directSamplesOther: number;  // samples for the other leaders (1-4)
  directHistory: number;       // temporal reuse weight for direct light (0 = history-free; 0.7 ≈ 3-frame EMA), depth-validated + variance-clipped, transient lights bypass it
  directPasses: number;        // spatial à-trous passes per class (0-6)
  directBlurCap: number;       // max penumbra filter radius in px
  directAdaptiveMin: number;   // adaptive sampling: stop after this many samples if they all agree (2-4)
  secMinE: number;             // shadow-ray threshold (W/m² unshadowed) for lights at cascade interval hits
  flashlightRadius: number;    // emitter radius used for torches / the rail light (m): a real bezel is ~0.015-0.02; larger = softer torch shadows
  tileCull: boolean;           // direct pass: each 8x8 tile scores only the lights that can reach its bounds (lossless; FrameFlags.TileCull) — off = every pixel scores every light, for A/B
  softTileSkip: boolean;       // penumbra filter: 8x8 tiles provably out of reach of every blur hint skip each à-trous step outright (lossless; FrameFlags.SoftTileSkip) — off = every pixel decides for itself, for A/B
  smokeRenderSkip: boolean;    // smoke samplers (shadow rays, beams, the volumetric march, probe sight lines) test the solver's per-brick render occupancy and skip the atlas fetch where the trilinear footprint is provably all zero (lossless; FrameFlags.SmokeRenderSkip) — off = always fetch, for A/B
  gridSkip: boolean;           // every traced ray (traceClosest / occluded / occludedT): the DDA runs past cells the grid build proved empty without loading them — one Chebyshev-distance byte per cell (lossless; FrameFlags.GridSkip) — off = a load pair per cell as before, for A/B
  gridYCull: boolean;          // every traced ray: cells and boxes (and the once-per-ray globals) whose stored height band the remaining stretch of the ray cannot reach are not slab-tested (per-cell 24-slice occupancy + per-item quantised extent + per-global extent; the argument is above traceClosest in common.wgsl) (lossless; FrameFlags.GridYCull) — off = every registered box is tested, for A/B
  axisBoxFast: boolean;        // slab test: boxes with yaw exactly 0 (two thirds of the grid items, every global) skip the rotation into the local frame and reuse the ray's inverse direction instead of three divisions — the arithmetic is provably the same bits (isectBox in common.wgsl) (lossless; FrameFlags.AxisBox) — off = always rotate, for A/B
  gridSlabs: boolean;          // every traced ray: the huge once-per-ray "global" slabs (ground, carpet base, floor finishes, ceiling) are skipped unless the ray segment's bounds meet theirs on all three axes (5 mm proud) instead of on height alone (4 cm) — a ray lifted off the floor stops testing four floors — and each cell's height span is judged 3.5 cm tighter at both ends, so a ray no longer re-tests the surface it just left (argument above traceClosest in common.wgsl; tools/qa/grid-globals.ts counts it) (lossless; FrameFlags.GridSlabs) — off = the height cull as it was, for A/B
  // ---- LOSSY trade-offs (all default off; each independently switchable at runtime for A/B — off is exactly the untouched path) ----
  checkerDirect: boolean;      // direct pass: shadow rays on a checkerboard half of the pixels per frame (parity alternates), the other half rebuilds S = U × its traced neighbours' S/U; hard edges and unsupported pixels still trace (FrameFlags.CheckerDirect)
  gatherThird: boolean;        // final gather (rays, temporal, à-trous) at 1/3 resolution instead of 1/2; the composite's joint-bilateral upsample follows the divisor (frame.lossyCfg.x; targets are recreated on change)
  rcHalfRate: boolean;         // radiance-cascade cache at half rate: upper cascades on even frames, c0 + dice bake on odd ones (rc.halfRate); the dice everyone reads is at most a frame older
  volQuarter: boolean;         // volumetrics (haze beams + smoke march) at 1/4 resolution instead of 1/2; the composite's depth-aware upsample follows (frame.lossyCfg.y)
  dimRays: boolean;            // direct pass: pixels whose total unshadowed luminance (both classes) is under dimRaysBelow spend one emitter sample per leader instead of directSamplesTop/Other (frame.lossyCfg.z = the threshold, 0 when off)
  dimRaysBelow: number;        // that threshold, in the estimator's irradiance units: 0.01 W/m² unshadowed on a mid-grey surface at exposure 2 lands at ≈2 % of display white after AgX (0.02 ≈ 4 %)
}

export const defaultSettings = (): RenderSettings => ({
  renderScale: 1.0, maxInternalHeight: 1000, exposure: 2.0,
  flags: FrameFlags.Direct | FrameFlags.Indirect | FrameFlags.Volumetrics | FrameFlags.Smoke | FrameFlags.RcVisWeight | FrameFlags.SmokeShadows | FrameFlags.Bounce | FrameFlags.Dither | FrameFlags.SoftShadow | FrameFlags.Temporal,
  debugView: 0, hazeDensity: 0.035, indirectScale: 0.35, emissiveScale: 1.0, volSteps: 20,
  skyZenith: [0.015, 0.025, 0.05], skyHorizon: [0.04, 0.05, 0.08],
  capColor: [0.010, 0.012, 0.016, 0.3],
  nvAmount: 0, nvGain: 475, hitFlash: 0,   // nvGain = intensifier tube gain (log curve), not a linear multiplier
  directSamplesTop: 8, directSamplesOther: 4, directHistory: 0.5, directPasses: 3, directBlurCap: 10, directAdaptiveMin: 3, secMinE: 0.006, flashlightRadius: 0.07,   // adaptive: 8/4 is the budget in penumbrae, `directAdaptiveMin` rays where samples agree
  tileCull: true,              // proven exact: 4 views × final/direct/indirect bit-identical off vs on under the deterministic capture protocol (08-06); toggle stays in the Direct light panel
  softTileSkip: true,          // exact by construction (argument in softblur.wgsl); A/B via the Direct light panel toggle / frame flag 8192
  smokeRenderSkip: true,       // exact by construction (argument above renderOccRegions in smoke.wgsl); A/B via the Smoke panel toggle / frame flag 32768
  gridSkip: true,              // exact by construction (one DDA iteration per cell as before, only the loads of provably empty cells go); A/B via the Direct light panel toggle / frame flag 131072
  gridYCull: true,             // exact under the grid's own registration invariant (argument above traceClosest in common.wgsl); A/B via the Direct light panel toggle / frame flag 262144
  axisBoxFast: true,           // exact by IEEE identities (1·x − 0·z = x); A/B via the Direct light panel toggle / frame flag 524288
  gridSlabs: true,             // exact with a 5 mm margin over < 0.4 mm of float differences (argument above traceClosest); −40 % slab tests on rays leaving the floor per tools/qa/grid-globals.ts; A/B via the Direct light panel toggle / frame flag 1048576
  checkerDirect: false, gatherThird: false, rcHalfRate: false, volQuarter: false, dimRays: false, dimRaysBelow: 0.01,
});

/** What the renderer needs from the smoke sim: the domain table (uniform), the density atlas, the per-brick render occupancy (`occ`, 16 words per domain,
 *  bound as a uniform block — see smokeDensityAt in common.wgsl), how many domain slots to walk, and the sim's encoder. */
export interface SmokeBinding { uniform: GPUBuffer; atlasView: GPUTextureView; occ: GPUBuffer; count: number; encode?: (enc: GPUCommandEncoder, ts: (n: string) => GPUComputePassTimestampWrites | undefined) => void; }

export class Engine {
  device: GPUDevice;
  world: BoxWorld;
  lights: LightSet;
  rc: RadianceCascades;
  gbuffer: GBuffer;
  passes: LightingPasses;
  timer: GpuTimer;
  sceneLayout: GPUBindGroupLayout;
  sceneBG: GPUBindGroup | null = null;
  frameBuf: GPUBuffer; frameData = new ArrayBuffer(FRAME_BYTES);
  dummyStorage: GPUBuffer;
  smoke: SmokeBinding;
  settings: RenderSettings = defaultSettings();
  frameIdx = 0;
  /** internal-resolution scale actually in effect after the maxInternalHeight cap (<= settings.renderScale) */
  effectiveScale = 1; capScale = 1; lastDt = 1 / 60;
  width = 0; height = 0;
  skin: SkinnedDrawable | null = null;
  cpuMs = { boxes: 0, encode: 0 };
  /** extra GPU work encoded after lighting (irradiance probes etc.): (enc, sceneBG, diceView, ts) */
  extraPasses: ((enc: GPUCommandEncoder, sceneBG: GPUBindGroup, diceView: GPUTextureView, ts: (n: string) => GPUComputePassTimestampWrites | undefined) => void)[] = [];
  afterSubmit: (() => void)[] = [];

  constructor(public gpu: Gpu, rcCfg: RcConfig = defaultRcConfig) {
    const device = this.device = gpu.device;
    this.timer = new GpuTimer(device, gpu.hasTimestamps);
    this.sceneLayout = createSceneLayout(device);
    this.world = new BoxWorld(device);
    this.lights = new LightSet(device);
    this.frameBuf = makeBuffer(device, FRAME_BYTES, GPUBufferUsage.UNIFORM | GPUBufferUsage.COPY_DST, 'frame');
    this.dummyStorage = makeBuffer(device, 256, GPUBufferUsage.STORAGE, 'dummy');
    // smoke placeholder until the sim is attached
    const dummyTex = device.createTexture({ size: [1, 1, 1], dimension: '3d', format: 'rgba16float', usage: GPUTextureUsage.TEXTURE_BINDING });
    const smokeUbo = makeBuffer(device, 64 * 8, GPUBufferUsage.UNIFORM | GPUBufferUsage.COPY_DST, 'smokeDomainsDummy');
    const smokeOcc = makeBuffer(device, 64 * 8, GPUBufferUsage.UNIFORM | GPUBufferUsage.COPY_DST, 'smokeOccDummy');   // 16 words × 8 domains, all zero (count is 0 anyway)
    this.smoke = { uniform: smokeUbo, atlasView: dummyTex.createView(), occ: smokeOcc, count: 0 };
    this.rc = new RadianceCascades(device, this.sceneLayout, { ...rcCfg });
    this.gbuffer = new GBuffer(device, this.sceneLayout);
    this.passes = new LightingPasses(device, this.sceneLayout, gpu.format, gpu.linearSampler);
  }

  setSmoke(s: SmokeBinding) { this.smoke = s; this.sceneBG = null; }

  rebuildRc(cfg: RcConfig) {
    this.rc.destroy(); this.rc = new RadianceCascades(this.device, this.sceneLayout, { ...cfg }); this.passes.invalidate(); this.sceneBG = null;
  }

  private ensureSceneBG() {
    if (this.sceneBG && this.sceneBGItems === this.world.itemBuf) return;
    this.sceneBG = createSceneBindGroup(this.device, this.sceneLayout, {
      frame: this.frameBuf, boxGeo: this.world.geoBuf, boxMat: this.world.matBuf, gridCells: this.world.cellBuf, gridItems: this.world.itemBuf,
      sceneInfo: this.world.infoBuf, lights: this.lights.buf, smoke: this.smoke.uniform, smokeAtlasView: this.smoke.atlasView, smokeOcc: this.smoke.occ, linSamp: this.gpu.linearSampler, capsules: this.world.capsuleBuf,
    });
    this.sceneBGItems = this.world.itemBuf;
  }
  private sceneBGItems: GPUBuffer | null = null;

  resize() {
    const canvas = this.gpu.canvas;
    const dpr = window.devicePixelRatio || 1;
    const cw = Math.max(1, Math.floor(canvas.clientWidth * dpr)), ch = Math.max(1, Math.floor(canvas.clientHeight * dpr));
    // canvas backing store at internal res; browser upscales
    let scale = this.settings.renderScale;
    if (ch * scale > this.settings.maxInternalHeight) scale = this.settings.maxInternalHeight / ch;
    this.effectiveScale = scale;   // what actually got rendered (the height cap can undercut renderScale on big / HiDPI windows) — adaptive quality steps from this
    this.capScale = Math.min(1, this.settings.maxInternalHeight / ch);   // the most this window can get at the current cap, whatever renderScale says
    const w = Math.max(16, Math.floor(cw * scale)), h = Math.max(16, Math.floor(ch * scale));
    if (canvas.width !== w || canvas.height !== h) { canvas.width = w; canvas.height = h; }
    if (w !== this.width || h !== this.height) {
      this.width = w; this.height = h;
      this.gbuffer.resize(w, h); this.passes.resize(w, h); this.passes.invalidate();
    }
  }

  private offscreenTarget: GPUTexture | null = null;
  /** Read back the last offscreen frame (render with present=false first) as RGBA8 pixels — for automated captures. */
  async readbackFrame(): Promise<{ w: number; h: number; data: Uint8Array; bgra: boolean } | null> {
    const tex = this.offscreenTarget; if (!tex) return null;
    const w = tex.width, h = tex.height; const bpr = Math.ceil(w * 4 / 256) * 256;
    const buf = this.device.createBuffer({ size: bpr * h, usage: GPUBufferUsage.COPY_DST | GPUBufferUsage.MAP_READ });
    const enc = this.device.createCommandEncoder(); enc.copyTextureToBuffer({ texture: tex }, { buffer: buf, bytesPerRow: bpr }, [w, h]); this.device.queue.submit([enc.finish()]);
    await buf.mapAsync(GPUMapMode.READ);
    const src = new Uint8Array(buf.getMappedRange()); const data = new Uint8Array(w * h * 4);
    for (let y = 0; y < h; y++) data.set(src.subarray(y * bpr, y * bpr + w * 4), y * w * 4);
    buf.unmap(); buf.destroy();
    return { w, h, data, bgra: this.gpu.format.startsWith('bgra') };
  }
  /** present=false renders into an internal target instead of the swap chain (scripted runs while the tab is hidden, where getCurrentTexture() stalls). */
  render(cam: FollowCamera, time: number, dt: number, present = true) {
    this.lastDt = dt;
    this.passes.setDivisors(this.settings.gatherThird ? 3 : 2, this.settings.volQuarter ? 4 : 2);   // (lossy options; recreates the low-res targets when one changes)
    this.resize();
    cam.aspect = this.width / this.height;
    cam.rebuild();
    const t0 = performance.now();
    this.world.upload();
    const numLights = this.lights.upload(time);
    this.cpuMs.boxes = performance.now() - t0;
    this.ensureSceneBG();

    const s = this.settings;
    this.rc.halfRate = s.rcHalfRate;
    const jf = (this.rc.staggerUpper || this.rc.halfRate) ? (this.frameIdx >> 1) : this.frameIdx;   // same direction jitter for the pair of frames that share cascade work (stagger: c2+ every other frame; half rate: uppers even / c0 + bake odd)
    this.rc.setDiceBlend((s.flags & FrameFlags.Temporal) ? 0.3 : 1.0);
    const fp: FrameParams = {
      viewProj: cam.viewProj, invViewProj: cam.invViewProj, prevViewProj: cam.prevViewProj,
      camPos: cam.pos, camDir: cam.forward, time, dt, width: this.width, height: this.height,
      frameIdx: this.frameIdx, numLights, flags: (s.flags & ~(FrameFlags.TileCull | FrameFlags.SoftTileSkip | FrameFlags.SmokeRenderSkip | FrameFlags.CheckerDirect | FrameFlags.GridSkip | FrameFlags.GridYCull | FrameFlags.AxisBox | FrameFlags.GridSlabs)) | (s.nvAmount > 0.001 ? FrameFlags.NightVision : 0) | (s.tileCull ? FrameFlags.TileCull : 0) | (s.softTileSkip ? FrameFlags.SoftTileSkip : 0) | (s.smokeRenderSkip ? FrameFlags.SmokeRenderSkip : 0) | (s.checkerDirect ? FrameFlags.CheckerDirect : 0) | (s.gridSkip ? FrameFlags.GridSkip : 0) | (s.gridYCull ? FrameFlags.GridYCull : 0) | (s.axisBoxFast ? FrameFlags.AxisBox : 0) | (s.gridSlabs ? FrameFlags.GridSlabs : 0), exposure: s.exposure,   // tileCull / softTileSkip / smokeRenderSkip / gridSkip / gridYCull / axisBoxFast / gridSlabs: the boolean settings are the single source of truth for bits 4096 / 8192 / 32768 / 131072 / 262144 / 524288 / 1048576
      skyZenith: s.skyZenith, skyHorizon: s.skyHorizon, hazeDensity: s.hazeDensity,
      rcInterval0: this.rc.cfg.interval0, rcC0Dims: [this.rc.cfg.nx, this.rc.cfg.ny, this.rc.cfg.nz], rcNumCascades: this.rc.cfg.cascades, rcD0: this.rc.cfg.d0, rcFrameParity: this.frameIdx & 1,
      numSmoke: this.smoke.count, debugView: s.debugView, indirectScale: s.indirectScale, emissiveScale: s.emissiveScale, volSteps: s.volSteps,
      capColor: s.capColor, post: [s.nvAmount, s.nvGain, s.hitFlash, Math.tan(cam.fov / 2)],
      directCfg: [s.directSamplesTop, s.directSamplesOther, s.directBlurCap, s.directHistory],
      rcJitter: (s.flags & FrameFlags.Temporal) ? [0.1 + 0.8 * ((jf * 0.7548776662466927) % 1), 0.1 + 0.8 * ((jf * 0.5698402909980532) % 1)] : [0.5, 0.5],
      directAdaptiveMin: s.directAdaptiveMin,
      numTransient: this.lights.lights.reduce((n, l) => n + (l.ttl >= 0 && l.enabled ? 1 : 0), 0),
      secMinE: s.secMinE,   // R2 sequence over the octahedral texel: cascade direction aliasing becomes noise for the indirect temporal filter
      lossyCfg: [this.passes.gatherDiv, this.passes.volDiv, s.dimRays ? Math.max(0, s.dimRaysBelow) : 0, 0],
    };
    writeFrame(this.frameData, fp);
    this.device.queue.writeBuffer(this.frameBuf, 0, this.frameData);

    const t1 = performance.now();
    const timer = this.timer; timer.beginFrame();
    const enc = this.device.createCommandEncoder({ label: 'frame' });
    const ts = (n: string) => timer.pass(n);
    if (this.smoke.encode) this.smoke.encode(enc, ts);
    this.gbuffer.encode(enc, this.sceneBG!, this.world.visibleBuf, this.world.visibleCount, this.skin, this.dummyStorage, ts('gbuffer') as GPURenderPassTimestampWrites | undefined);
    this.rc.encode(enc, this.sceneBG!, timer, this.frameIdx);
    let canvasView: GPUTextureView;
    if (present) canvasView = this.gpu.ctx.getCurrentTexture().createView();
    else {
      if (!this.offscreenTarget || this.offscreenTarget.width !== this.width || this.offscreenTarget.height !== this.height) { this.offscreenTarget?.destroy(); this.offscreenTarget = this.device.createTexture({ size: [this.width, this.height], format: this.gpu.format, usage: GPUTextureUsage.RENDER_ATTACHMENT | GPUTextureUsage.COPY_SRC }); }
      canvasView = this.offscreenTarget.createView();
    }
    this.passes.directPasses = s.directPasses; this.passes.directHistory = s.directHistory;
    this.passes.encode(enc, this.sceneBG!, this.gbuffer, this.rc, canvasView, ts, fp.flags);
    for (const f of this.extraPasses) f(enc, this.sceneBG!, this.rc.diceView, ts);
    timer.resolve(enc);
    this.device.queue.submit([enc.finish()]);
    timer.afterSubmit();
    for (const f of this.afterSubmit) f();
    this.cpuMs.encode = performance.now() - t1;
    this.frameIdx++;
  }
}

export { FrameFlags };
