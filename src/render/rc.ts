// Radiance cascades (world-space, 2.5D) — GPU resources, per-cascade params and dispatch.
import { makeBuffer, makeShader } from '../gpu/device';
import { GpuTimer } from '../gpu/timer';
import { WORLD } from '../scene/world';
import { scenePrelude } from './frame';
import lightingCommon from './shaders/lighting_common.wgsl' with { type: 'text' };
import rcSrc from './shaders/rc.wgsl' with { type: 'text' };

export interface RcConfig {
  nx: number; ny: number; nz: number;   // cascade-0 probe grid
  d0: number;                           // directions per octahedral side at c0 (4 → 16 dirs)
  cascades: number;                     // number of cascades
  interval0: number;                    // base interval length (m); cascade n covers [L0*(4^n-1)/3, L0*(4^(n+1)-1)/3]
}

export const defaultRcConfig: RcConfig = { nx: 40, ny: 4, nz: 28, d0: 4, cascades: 4, interval0: 0.6 };   // 1 m probes: a radiance cache for the final gather, not something the eye sees directly

/** Solid angle of each texel of a D×D octahedral map (numerical), normalized to sum to 4π. */
export function octSolidAngles(D: number): Float32Array {
  const out = new Float32Array(D * D);
  const K = 24; // sub-samples per texel side
  const dec = (u: number, v: number): [number, number, number] => {
    const fx = u * 2 - 1, fy = v * 2 - 1;
    let nx = fx, nz = fy, ny = 1 - Math.abs(fx) - Math.abs(fy);
    const t = Math.max(-ny, 0); nx += nx >= 0 ? -t : t; nz += nz >= 0 ? -t : t;
    const l = Math.hypot(nx, ny, nz); return [nx / l, ny / l, nz / l];
  };
  let total = 0;
  for (let ty = 0; ty < D; ty++) for (let tx = 0; tx < D; tx++) {
    let om = 0; const du = 1 / (D * K);
    for (let sy = 0; sy < K; sy++) for (let sx = 0; sx < K; sx++) {
      const u = (tx * K + sx) * du, v = (ty * K + sy) * du;
      const a = dec(u, v), b = dec(u + du, v), c = dec(u, v + du);
      const e1 = [b[0] - a[0], b[1] - a[1], b[2] - a[2]], e2 = [c[0] - a[0], c[1] - a[1], c[2] - a[2]];
      const cx = e1[1] * e2[2] - e1[2] * e2[1], cy = e1[2] * e2[0] - e1[0] * e2[2], cz = e1[0] * e2[1] - e1[1] * e2[0];
      om += Math.hypot(cx, cy, cz);
    }
    out[ty * D + tx] = om; total += om;
  }
  const s = (4 * Math.PI) / total; for (let i = 0; i < out.length; i++) out[i] *= s;
  return out;
}

export class RadianceCascades {
  cfg: RcConfig;
  bufs: GPUBuffer[] = [];          // [0] = c0 full merged radiance; [n>=1] = pre-averaged merged radiance
  dice: GPUTexture; diceView: GPUTextureView; dicePrev!: GPUTexture; dicePrevView!: GPUTextureView;
  /** blend factor toward this frame's bake (1 = no history) */
  diceBlend = 0.3;
  dirOmegaBuf: GPUBuffer;
  private paramsBuf: GPUBuffer; private dummyParent: GPUBuffer | null = null;
  private pipeUpper: GPUComputePipeline; private pipeC0: GPUComputePipeline; private pipeDice: GPUComputePipeline;
  private bgTrace: GPUBindGroup[] = []; private bgDice: GPUBindGroup;
  dims: [number, number, number][] = [];
  stats = { intervals: 0, bytes: 0 };
  /** update upper cascades every other frame (halves their cost, adds ≤1 frame of latency to far-field light) */
  staggerUpper = false;
  /** LOSSY option (engine.settings.rcHalfRate): the whole cache at half rate — upper cascades on even frames, c0 + the dice bake on odd ones
   *  (the engine gives such a pair one direction jitter, so c0 merges parents traced with its own directions and the bake decodes c0 with
   *  them too). Everything downstream just reads a dice volume that is at most a frame older; bounce / light meter / smoke ambient lag a
   *  moving light by that much. staggerUpper then counts in pairs (c2+ every fourth frame). */
  halfRate = false;

  constructor(private device: GPUDevice, sceneLayout: GPUBindGroupLayout, cfg: RcConfig) {
    this.cfg = cfg;
    const mod = makeShader(device, scenePrelude() + lightingCommon + rcSrc, 'rc');
    const layoutTrace = device.createBindGroupLayout({ label: 'rcTrace', entries: [
      { binding: 0, visibility: GPUShaderStage.COMPUTE, buffer: { type: 'uniform' } },
      { binding: 1, visibility: GPUShaderStage.COMPUTE, buffer: { type: 'storage' } },
      { binding: 2, visibility: GPUShaderStage.COMPUTE, buffer: { type: 'read-only-storage' } },
      { binding: 3, visibility: GPUShaderStage.COMPUTE, texture: { sampleType: 'float', viewDimension: '3d' } },
      { binding: 4, visibility: GPUShaderStage.COMPUTE, buffer: { type: 'uniform' } },
    ] });
    const layoutDice = device.createBindGroupLayout({ label: 'rcDice', entries: [
      { binding: 0, visibility: GPUShaderStage.COMPUTE, buffer: { type: 'uniform' } },
      { binding: 3, visibility: GPUShaderStage.COMPUTE, texture: { sampleType: 'float', viewDimension: '3d' } },   // last frame's dice (EMA source)
      { binding: 4, visibility: GPUShaderStage.COMPUTE, buffer: { type: 'uniform' } },
      { binding: 5, visibility: GPUShaderStage.COMPUTE, storageTexture: { access: 'write-only', format: 'rgba16float', viewDimension: '3d' } },
      { binding: 6, visibility: GPUShaderStage.COMPUTE, buffer: { type: 'read-only-storage' } },
    ] });
    const plTrace = device.createPipelineLayout({ bindGroupLayouts: [sceneLayout, layoutTrace] });
    const plDice = device.createPipelineLayout({ bindGroupLayouts: [sceneLayout, layoutDice] });
    this.pipeUpper = device.createComputePipeline({ label: 'rcUpper', layout: plTrace, compute: { module: mod, entryPoint: 'rcUpper' } });
    this.pipeC0 = device.createComputePipeline({ label: 'rcC0', layout: plTrace, compute: { module: mod, entryPoint: 'rcC0' } });
    this.pipeDice = device.createComputePipeline({ label: 'rcDice', layout: plDice, compute: { module: mod, entryPoint: 'rcDice' } });

    // buffers
    const N = cfg.cascades; let bytes = 0; let intervals = 0;
    for (let n = 0; n < N; n++) {
      const d: [number, number, number] = [Math.ceil(cfg.nx / (1 << n)), cfg.ny, Math.ceil(cfg.nz / (1 << n))];
      this.dims.push(d);
      const probes = d[0] * d[1] * d[2]; const Dn = cfg.d0 << n;
      const texels = n === 0 ? probes * Dn * Dn : probes * (Dn / 2) * (Dn / 2);
      intervals += probes * Dn * Dn;
      const size = texels * 8; bytes += size;
      this.bufs.push(makeBuffer(device, size, GPUBufferUsage.STORAGE, `rcCascade${n}`));
    }
    this.stats = { intervals, bytes };
    this.dice = device.createTexture({ label: 'rcDice', size: [cfg.nx, cfg.ny * 7, cfg.nz], dimension: '3d', format: 'rgba16float', usage: GPUTextureUsage.STORAGE_BINDING | GPUTextureUsage.TEXTURE_BINDING | GPUTextureUsage.COPY_SRC });
    this.diceView = this.dice.createView();
    // previous frame's dice: the bake blends toward it (short EMA) so per-frame direction jitter does not shimmer the bounce / light meter / smoke ambient
    this.dicePrev = device.createTexture({ label: 'rcDicePrev', size: [cfg.nx, cfg.ny * 7, cfg.nz], dimension: '3d', format: 'rgba16float', usage: GPUTextureUsage.TEXTURE_BINDING | GPUTextureUsage.COPY_DST });
    this.dicePrevView = this.dicePrev.createView();
    // solid angles
    const om = octSolidAngles(cfg.d0); const omData = new Float32Array(64); for (let i = 0; i < Math.min(16, om.length); i++) omData[i * 4] = om[i];
    this.dirOmegaBuf = makeBuffer(device, 256, GPUBufferUsage.UNIFORM | GPUBufferUsage.COPY_DST, 'dirOmega');
    device.queue.writeBuffer(this.dirOmegaBuf, 0, omData);
    // params (256-B slots)
    this.paramsBuf = makeBuffer(device, 256 * N, GPUBufferUsage.UNIFORM | GPUBufferUsage.COPY_DST, 'rcParams');
    this.writeParams();
    const dummy = this.dummyParent = makeBuffer(device, 16, GPUBufferUsage.STORAGE, 'rcDummyParent');
    for (let n = 0; n < N; n++) {
      this.bgTrace.push(device.createBindGroup({ layout: layoutTrace, entries: [
        { binding: 0, resource: { buffer: this.paramsBuf, offset: 256 * n, size: 80 } },
        { binding: 1, resource: { buffer: this.bufs[n] } },
        { binding: 2, resource: { buffer: n + 1 < N ? this.bufs[n + 1] : dummy } },
        { binding: 3, resource: this.diceView },
        { binding: 4, resource: { buffer: this.dirOmegaBuf } },
      ] }));
    }
    this.bgDice = device.createBindGroup({ layout: layoutDice, entries: [
      { binding: 0, resource: { buffer: this.paramsBuf, offset: 0, size: 80 } },
      { binding: 3, resource: this.dicePrevView },
      { binding: 4, resource: { buffer: this.dirOmegaBuf } },
      { binding: 5, resource: this.diceView },
      { binding: 6, resource: { buffer: this.bufs[0] } },
    ] });
  }

  intervalStart(n: number) { return this.cfg.interval0 * (Math.pow(4, n) - 1) / 3; }

  writeParams() {
    const { cfg } = this; const N = cfg.cascades;
    const data = new ArrayBuffer(256 * N);
    for (let n = 0; n < N; n++) {
      const u = new Uint32Array(data, 256 * n, 20), f = new Float32Array(data, 256 * n, 20);
      const d = this.dims[n]; const Dn = cfg.d0 << n;
      const sp = [WORLD.rcSize[0] / d[0], WORLD.rcSize[1] / d[1], WORLD.rcSize[2] / d[2]];
      u[0] = n; u[1] = Dn; u[2] = n === 0 ? Dn : Dn / 2; u[3] = d[0] * d[1] * d[2];
      u[4] = d[0]; u[5] = d[1]; u[6] = d[2]; u[7] = n + 1 < N ? 1 : 0;
      f[8] = sp[0]; f[9] = sp[1]; f[10] = sp[2]; f[11] = this.intervalStart(n);
      const pd = n + 1 < N ? this.dims[n + 1] : d;
      u[12] = pd[0]; u[13] = pd[1]; u[14] = pd[2]; f[15] = this.intervalStart(n + 1) - this.intervalStart(n);
      f[16] = WORLD.rcSize[0] / pd[0]; f[17] = WORLD.rcSize[1] / pd[1]; f[18] = WORLD.rcSize[2] / pd[2]; f[19] = this.diceBlend;
    }
    this.device.queue.writeBuffer(this.paramsBuf, 0, data);
  }

  setInterval0(v: number) { if (v !== this.cfg.interval0) { this.cfg.interval0 = v; this.writeParams(); } }

  encode(enc: GPUCommandEncoder, sceneBG: GPUBindGroup, timer: GpuTimer, frameIdx: number) {
    const N = this.cfg.cascades;
    // half rate: uppers on even frames, c0 + bake on odd; a resting pass still opens an empty timed pass under its name so the HUD's smoothed
    // per-pass numbers (and the auto-quality calibration that sums them) average the cache at the rate it really runs at
    const doUpper = !this.halfRate || (frameIdx & 1) === 0, doC0 = !this.halfRate || (frameIdx & 1) === 1;
    const rest = (name: string) => { const tw = timer.pass(name); if (tw) enc.beginComputePass({ label: `${name} (resting)`, timestampWrites: tw }).end(); };
    const staggerClock = this.halfRate ? (frameIdx >> 1) : frameIdx;
    for (let n = N - 1; n >= 1; n--) {
      if (this.staggerUpper && n >= 2 && ((staggerClock + n) & 1) === 1) continue;
      if (!doUpper) { rest(`rc c${n}`); continue; }
      const pass = enc.beginComputePass({ label: `rc c${n}`, timestampWrites: timer.pass(`rc c${n}`) });
      pass.setPipeline(this.pipeUpper); pass.setBindGroup(0, sceneBG); pass.setBindGroup(1, this.bgTrace[n]);
      const d = this.dims[n]; pass.dispatchWorkgroups(d[0], d[1], d[2]);
      pass.end();
    }
    if (!doC0) { rest('rc c0'); rest('rc dice'); return; }
    {
      const pass = enc.beginComputePass({ label: 'rc c0', timestampWrites: timer.pass('rc c0') });
      pass.setPipeline(this.pipeC0); pass.setBindGroup(0, sceneBG); pass.setBindGroup(1, this.bgTrace[0]);
      const d = this.dims[0]; const perWg = Math.floor(64 / (this.cfg.d0 * this.cfg.d0));
      pass.dispatchWorkgroups(Math.ceil(d[0] / perWg), d[1], d[2]);
      pass.end();
    }
    if (this.diceBlend < 1) enc.copyTextureToTexture({ texture: this.dice }, { texture: this.dicePrev }, [this.cfg.nx, this.cfg.ny * 7, this.cfg.nz]);   // history source for the EMA (skipped when history is off: blend 1 ignores it)
    {
      const pass = enc.beginComputePass({ label: 'rc dice', timestampWrites: timer.pass('rc dice') });
      pass.setPipeline(this.pipeDice); pass.setBindGroup(0, sceneBG); pass.setBindGroup(1, this.bgDice);
      const d = this.dims[0]; pass.dispatchWorkgroups(Math.ceil((d[0] * d[1] * d[2]) / 64));
      pass.end();
    }
  }

  setDiceBlend(v: number) { if (v !== this.diceBlend) { this.diceBlend = v; this.writeParams(); } }
  destroy() { for (const b of this.bufs) b.destroy(); this.dice.destroy(); this.dicePrev.destroy(); this.paramsBuf.destroy(); this.dirOmegaBuf.destroy(); this.dummyParent?.destroy(); }
}
