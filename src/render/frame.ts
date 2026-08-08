// Per-frame uniform block (must match `struct Frame` in common.wgsl) + shared scene bind group.
import { Mat4, Vec3 } from '../math/vec';
import commonSrc from './shaders/common.wgsl' with { type: 'text' };
import { wgslWorldConsts } from '../scene/world';

export const FRAME_BYTES = 416;

export const FrameFlags = {
  Direct: 1, Indirect: 2, Volumetrics: 4, Smoke: 8, RcVisWeight: 16, /* 32 was NearField (legacy gather) */ SmokeShadows: 64, Bounce: 128, Dither: 256, SoftShadow: 512, NightVision: 1024, Temporal: 2048, TileCull: 4096 /* direct pass: lossless per-tile light culling (set from settings.tileCull, not settings.flags) */, SoftTileSkip: 8192 /* penumbra filter: lossless tile-level pass-through (set from settings.softTileSkip) */, TailChroma: 16384,
  SmokeRenderSkip: 32768 /* smoke samplers skip atlas fetches in bricks the solver's render occupancy proves empty — lossless (set from settings.smokeRenderSkip) */,
  CheckerDirect: 65536 /* LOSSY: shadow rays on half the pixels per frame, checkerboard (set from settings.checkerDirect) */,
  GridSkip: 131072 /* trace grid: DDA runs past provably empty cells without loads — lossless (set from settings.gridSkip) */,
  GridYCull: 262144 /* trace grid: cells / boxes / globals outside the ray's remaining height span are not slab-tested — lossless (set from settings.gridYCull) */,
  AxisBox: 524288 /* slab test: unrotated boxes skip the yaw rotation and its three divisions — lossless (set from settings.axisBoxFast) */,
  GridSlabs: 1048576 /* trace grid: globals rejected by the ray segment's bounds on all three axes, cell height spans judged 3.5 cm tighter — lossless (set from settings.gridSlabs) */,
} as const;

export interface FrameParams {
  viewProj: Mat4; invViewProj: Mat4; prevViewProj: Mat4;
  camPos: Vec3; camDir: Vec3; time: number; dt: number;
  width: number; height: number;
  frameIdx: number; numLights: number; flags: number; exposure: number;
  skyZenith: Vec3; skyHorizon: Vec3; hazeDensity: number;
  rcInterval0: number; rcC0Dims: [number, number, number]; rcNumCascades: number; rcD0: number; rcFrameParity: number;
  numSmoke: number; debugView: number;
  indirectScale: number; emissiveScale: number; volSteps: number;
  capColor: [number, number, number, number];
  post: [number, number, number, number];
  rcJitter: [number, number];
  directCfg: [number, number, number, number];   // samplesTop, samplesOther, blurCapPx, historyWeight
  directAdaptiveMin: number;
  numTransient: number;        // transient (ttl) lights alive this frame — lets the gather skip its subtraction loop
  secMinE: number;             // shadow-ray threshold for lights at cascade interval hits (W/m², unshadowed)
  lossyCfg: [number, number, number, number];   // lossy options: gather divisor (2|3), volumetric divisor (2|4), dim-ray luminance threshold (0 = off), spare — see RenderSettings
}

export function writeFrame(dst: ArrayBuffer, p: FrameParams) {
  const f = new Float32Array(dst), u = new Uint32Array(dst);
  f.set(p.viewProj, 0); f.set(p.invViewProj, 16); f.set(p.prevViewProj, 32);
  f.set(p.camPos, 48); f[51] = p.time;
  f.set(p.camDir, 52); f[55] = p.dt;
  f[56] = p.width; f[57] = p.height; f[58] = 1 / p.width; f[59] = 1 / p.height;
  u[60] = p.frameIdx; u[61] = p.numLights; u[62] = p.flags; f[63] = p.exposure;
  f.set(p.skyZenith, 64); f[67] = p.hazeDensity;
  f.set(p.skyHorizon, 68); f[71] = p.rcInterval0;
  u[72] = p.rcC0Dims[0]; u[73] = p.rcC0Dims[1]; u[74] = p.rcC0Dims[2]; u[75] = p.rcNumCascades;
  u[76] = p.rcD0; u[77] = p.rcFrameParity; u[78] = p.numSmoke; u[79] = p.debugView;
  f[80] = p.indirectScale; f[81] = p.emissiveScale; u[82] = p.volSteps; f[83] = p.secMinE;
  f[84] = p.capColor[0]; f[85] = p.capColor[1]; f[86] = p.capColor[2]; f[87] = p.capColor[3];
  f[88] = p.post[0]; f[89] = p.post[1]; f[90] = p.post[2]; f[91] = p.post[3];
  f[92] = p.rcJitter[0]; f[93] = p.rcJitter[1]; f[94] = p.directAdaptiveMin; f[95] = p.numTransient;
  f[96] = p.directCfg[0]; f[97] = p.directCfg[1]; f[98] = p.directCfg[2]; f[99] = p.directCfg[3];
  f[100] = p.lossyCfg[0]; f[101] = p.lossyCfg[1]; f[102] = p.lossyCfg[2]; f[103] = p.lossyCfg[3];
}

/** WGSL prelude for every shader that touches the scene: world consts + common declarations (group 0). */
export function scenePrelude(): string { return wgslWorldConsts() + '\n' + commonSrc + '\n'; }

export function createSceneLayout(device: GPUDevice): GPUBindGroupLayout {
  const all = GPUShaderStage.VERTEX | GPUShaderStage.FRAGMENT | GPUShaderStage.COMPUTE;
  const cf = GPUShaderStage.FRAGMENT | GPUShaderStage.COMPUTE;
  return device.createBindGroupLayout({
    label: 'sceneLayout',
    entries: [
      { binding: 0, visibility: all, buffer: { type: 'uniform' } },
      { binding: 1, visibility: all, buffer: { type: 'read-only-storage' } },
      { binding: 2, visibility: all, buffer: { type: 'read-only-storage' } },
      { binding: 3, visibility: cf, buffer: { type: 'read-only-storage' } },
      { binding: 4, visibility: cf, buffer: { type: 'read-only-storage' } },
      { binding: 5, visibility: all, buffer: { type: 'uniform' } },
      { binding: 6, visibility: cf, buffer: { type: 'read-only-storage' } },
      { binding: 7, visibility: cf, buffer: { type: 'uniform' } },
      { binding: 8, visibility: cf, texture: { sampleType: 'float', viewDimension: '3d' } },
      { binding: 9, visibility: cf, sampler: { type: 'filtering' } },
      { binding: 10, visibility: cf, buffer: { type: 'read-only-storage' } },
      { binding: 11, visibility: cf, buffer: { type: 'uniform' } },   // smoke render occupancy (512 B written by the smoke solver each step; uniform, not a 7th storage buffer, to stay inside the base per-stage limit)
    ],
  });
}

export interface SceneBuffers {
  frame: GPUBuffer; boxGeo: GPUBuffer; boxMat: GPUBuffer; gridCells: GPUBuffer; gridItems: GPUBuffer; sceneInfo: GPUBuffer; lights: GPUBuffer; smoke: GPUBuffer; smokeAtlasView: GPUTextureView; smokeOcc: GPUBuffer; linSamp: GPUSampler; capsules: GPUBuffer;
}
export function createSceneBindGroup(device: GPUDevice, layout: GPUBindGroupLayout, b: SceneBuffers): GPUBindGroup {
  return device.createBindGroup({
    label: 'sceneBG', layout,
    entries: [
      { binding: 0, resource: { buffer: b.frame } },
      { binding: 1, resource: { buffer: b.boxGeo } },
      { binding: 2, resource: { buffer: b.boxMat } },
      { binding: 3, resource: { buffer: b.gridCells } },
      { binding: 4, resource: { buffer: b.gridItems } },
      { binding: 5, resource: { buffer: b.sceneInfo } },
      { binding: 6, resource: { buffer: b.lights } },
      { binding: 7, resource: { buffer: b.smoke } },
      { binding: 8, resource: b.smokeAtlasView },
      { binding: 9, resource: b.linSamp },
      { binding: 10, resource: { buffer: b.capsules } },
      { binding: 11, resource: { buffer: b.smokeOcc } },
    ],
  });
}
