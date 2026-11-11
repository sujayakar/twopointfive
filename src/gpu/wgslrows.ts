// Written by tools/gpu/wgsl-rows.ts from the shaders' own structs. To change one, change it in its shader and run that.

/** src/render/shaders/common.wgsl struct Frame, in words of four bytes */
export const enum FrameAt {
  /** mat4x4f */
  viewProj = 0,
  /** mat4x4f */
  invViewProj = 16,
  /** mat4x4f */
  prevViewProj = 32,
  /** vec3f */
  camPos = 48,
  /** f32 */
  time = 51,
  /** vec3f */
  camDir = 52,
  /** f32 */
  dt = 55,
  /** vec2f */
  screen = 56,
  /** vec2f */
  invScreen = 58,
  /** u32 */
  frameIdx = 60,
  /** u32 */
  numLights = 61,
  /** u32 */
  flags = 62,
  /** f32 */
  exposure = 63,
  /** vec3f */
  skyZenith = 64,
  /** f32 */
  hazeDensity = 67,
  /** vec3f */
  skyHorizon = 68,
  /** f32 */
  rcInterval0 = 71,
  /** vec3u */
  rcC0Dims = 72,
  /** u32 */
  rcNumCascades = 75,
  /** u32 */
  rcD0 = 76,
  /** u32 */
  rcFrameParity = 77,
  /** u32 */
  numSmoke = 78,
  /** u32 */
  debugView = 79,
  /** f32 */
  indirectScale = 80,
  /** f32 */
  emissiveScale = 81,
  /** u32 */
  volSteps = 82,
  /** f32 */
  secMinE = 83,
  /** vec4f */
  capColor = 84,
  /** vec4f */
  post = 88,
  /** vec4f */
  rcJitter = 92,
  /** vec4f */
  directCfg = 96,
  /** vec4f */
  lossyCfg = 100,
  /** vec4u */
  cardCfg = 104,
  /** array<vec4u, 2> */
  cardSlots = 108,
  /** vec4f */
  weatherA = 116,
  /** vec4f */
  weatherB = 120,
  /** vec4f */
  weatherC = 124,
  /** vec4f */
  glassCfg = 128,
  /** vec4f */
  weatherD = 132,
  /** vec4u */
  volCfg = 136,
  /** vec4f */
  prevCam = 140,
  /** vec4f */
  rtCfg = 144,
  /** vec4f */
  matCfg = 148,
  /** vec4u */
  p5Cfg = 152,
  /** vec4f */
  jitter = 156,
  /** vec4f */
  markCfg = 160,
  /** vec4f */
  sharpCfg = 164,
  /** vec4f */
  sharpCfgB = 168,
  /** vec4f */
  sharpCfgC = 172,
  /** vec4f */
  rigCfg = 176,
  /** vec4f */
  rigCfgB = 180,
  /** vec4f */
  giCfg = 184,
  /** vec4f */
  weatherE = 188,
  /** vec4f */
  weatherF = 192,
  /** vec4f */
  shadeCfg = 196,
  /** vec4f */
  veilA = 200,
  /** vec4f */
  veilB = 204,
  /** vec4f */
  eyeCfg = 208,
  /** vec4f */
  gradeCfg = 212,
  /** vec4f */
  lampCfg = 216,
  /** vec4u */
  sideCfg = 220,
  /** vec4f */
  placeCfg = 224,
  /** vec4u */
  levelCfg = 228,
  /** array<vec4u, 1750> */
  sides = 232,
  /** array<vec4u, 140> */
  sideCols = 7232,
  /** words it takes */
  words = 7792,
}

/** src/smoke/smoke.wgsl struct SP, in words of four bytes */
export const enum SpAt {
  /** u32 */
  numEmitters = 0,
  /** u32 */
  numExpand = 1,
  /** u32 */
  stepNo = 2,
  /** u32 */
  keep = 3,
  /** f32 */
  dt = 4,
  /** f32 */
  time = 5,
  /** f32 */
  buoyancy = 6,
  /** f32 */
  weight = 7,
  /** f32 */
  vortConf = 8,
  /** f32 */
  velDamp = 9,
  /** f32 */
  densDecay = 10,
  /** f32 */
  tempDecay = 11,
  /** f32 */
  turbulence = 12,
  /** f32 */
  omega = 13,
  /** f32 */
  flushEps = 14,
  /** u32 */
  ring = 15,
  /** f32 */
  stirConf = 16,
  /** f32 */
  pad17 = 17,
  /** u32 */
  numDirty = 18,
  /** u32 */
  bakeAll = 19,
  /** f32 */
  softLo = 20,
  /** f32 */
  softHi = 21,
  /** f32 */
  softEps = 22,
  /** f32 */
  softDecay = 23,
  /** f32 */
  thinDecay = 24,
  /** f32 */
  thinBelow = 25,
  /** f32 */
  readSkip = 26,
  /** f32 */
  fresh = 27,
  /** f32 */
  viewX = 28,
  /** f32 */
  viewZ = 29,
  /** f32 */
  viewR = 30,
  /** f32 */
  viewBias = 31,
  /** f32 */
  calmCells = 32,
  /** f32 */
  farCells = 33,
  /** u32 */
  calmMax = 34,
  /** f32 */
  dtCap = 35,
  /** f32 */
  bend = 36,
  /** f32 */
  whole = 37,
  /** f32 */
  fineDecay = 38,
  /** f32 */
  fineShare = 39,
  /** f32 */
  fineEps = 40,
  /** f32 */
  fineHeat = 41,
  /** u32 */
  fineRing = 42,
  /** f32 */
  pull = 43,
  /** array<Level, 2> */
  levels = 44,
  /** array<vec4f, 16> */
  dirty = 60,
  /** array<vec4f, 8> */
  windBound = 124,
  /** array<Emitter, 64> */
  emitters = 156,
  /** words it takes */
  words = 1180,
}

/** src/smoke/smoke.wgsl struct Level, in words of four bytes */
export const enum LevelAt {
  /** vec3u */
  dims = 0,
  /** u32 */
  pageBase = 3,
  /** f32 */
  voxel = 4,
  /** u32 */
  texY = 5,
  /** u32 */
  tile = 6,
  /** u32 */
  tileBase = 7,
  /** words it takes */
  words = 8,
}

/** src/smoke/smoke.wgsl struct Emitter, in words of four bytes */
export const enum EmitterAt {
  /** vec3f */
  pos = 0,
  /** f32 */
  radius = 3,
  /** vec3f */
  dir = 4,
  /** f32 */
  speed = 7,
  /** f32 */
  density = 8,
  /** f32 */
  temperature = 9,
  /** u32 */
  level = 10,
  /** f32 */
  mode = 11,
  /** f32 */
  push = 12,
  /** f32 */
  expand = 13,
  /** u32 */
  kind = 14,
  /** f32 */
  tall = 15,
  /** words it takes */
  words = 16,
}

/** src/smoke/wind_force.wgsl struct WindBlock, in words of four bytes */
export const enum WindAt {
  /** u32 */
  count = 0,
  /** f32 */
  relax = 1,
  /** f32 */
  minW = 2,
  /** f32 */
  pad0 = 3,
  /** array<vec4u, 2> */
  spare = 4,
  /** array<WindSrc, WIND_MAX_SRC> */
  src = 12,
  /** words it takes */
  words = 140,
}

/** src/smoke/wind_force.wgsl struct WindSrc, in words of four bytes */
export const enum WindSrcAt {
  /** vec3f */
  pos = 0,
  /** f32 */
  reach = 3,
  /** vec3f */
  dir = 4,
  /** f32 */
  speed = 7,
  /** vec4f */
  aux = 8,
  /** u32 */
  kind = 12,
  /** u32 */
  spare0 = 13,
  /** u32 */
  spare1 = 14,
  /** u32 */
  spare2 = 15,
  /** words it takes */
  words = 16,
}

/** src/render/shaders/air_field.wgsl struct AirU, in words of four bytes */
export const enum AirAt {
  /** vec2f */
  wind = 0,
  /** f32 */
  dt = 2,
  /** f32 */
  time = 3,
  /** vec2u */
  size = 4,
  /** u32 */
  stride = 6,
  /** f32 */
  res = 7,
  /** f32 */
  relax = 8,
  /** f32 */
  relaxRoof = 9,
  /** f32 */
  confine = 10,
  /** f32 */
  turb = 11,
  /** f32 */
  turbSize = 12,
  /** f32 */
  turbRate = 13,
  /** f32 */
  sigma = 14,
  /** f32 */
  sor = 15,
  /** f32 */
  cap = 16,
  /** f32 */
  liftGain = 17,
  /** f32 */
  liftMax = 18,
  /** f32 */
  invDt = 19,
  /** f32 */
  windLen = 20,
  /** f32 */
  seed = 21,
  /** vec2f */
  dWind = 22,
  /** f32 */
  drive = 24,
  /** u32 */
  srcCount = 25,
  /** f32 */
  sigmaRoof = 26,
  /** f32 */
  outScale = 27,
  /** array<AirSrc, 16> */
  src = 28,
  /** words it takes */
  words = 156,
}

/** src/render/shaders/air_field.wgsl struct AirSrc, in words of four bytes */
export const enum AirSrcAt {
  /** vec4f */
  a = 0,
  /** vec4f */
  b = 4,
  /** words it takes */
  words = 8,
}

/** src/render/shaders/place_field.wgsl struct PlaceU, in words of four bytes */
export const enum PlaceAt {
  /** u32 */
  parity = 0,
  /** u32 */
  snap = 1,
  /** f32 */
  kappa = 2,
  /** f32 */
  omega = 3,
  /** vec2f */
  sam = 4,
  /** f32 */
  eyeOpen = 6,
  /** f32 */
  eyeClose = 7,
  /** f32 */
  y = 8,
  /** f32 */
  dark = 9,
  /** f32 */
  spare = 10,
  /** f32 */
  skyEase = 11,
  /** f32 */
  skyShare = 12,
  /** f32 */
  near = 13,
  /** f32 */
  nearDoor = 14,
  /** f32 */
  pad2 = 15,
  /** words it takes */
  words = 16,
}

/** src/render/shaders/powder_field.wgsl struct PowU, in words of four bytes */
export const enum PowAt {
  /** vec4f */
  a = 0,
  /** vec4f */
  b = 4,
  /** vec4f */
  c = 8,
  /** vec4f */
  d = 12,
  /** vec4f */
  e = 16,
  /** vec4u */
  size = 20,
  /** vec4u */
  n = 24,
  /** vec4f */
  air = 28,
  /** array<PowKick, 16> */
  kicks = 32,
  /** words it takes */
  words = 160,
}

/** src/render/shaders/powder_field.wgsl struct PowKick, in words of four bytes */
export const enum PowKickAt {
  /** vec4f */
  a = 0,
  /** vec4f */
  b = 4,
  /** words it takes */
  words = 8,
}

/** src/render/shaders/outline.wgsl struct OutlineU, in words of four bytes */
export const enum OutlineAt {
  /** vec4f */
  ink = 0,
  /** u32 */
  owner = 4,
  /** f32 */
  radius = 5,
  /** f32 */
  pad0 = 6,
  /** f32 */
  pad1 = 7,
  /** vec4f */
  fade = 8,
  /** words it takes */
  words = 12,
}

/** src/render/shaders/composite.wgsl struct Beams, in words of four bytes */
export const enum BeamsAt {
  /** u32 */
  n = 0,
  /** u32 */
  p0 = 1,
  /** u32 */
  p1 = 2,
  /** u32 */
  p2 = 3,
  /** vec4f */
  k = 4,
  /** array<BeamSeg, 8> */
  seg = 8,
  /** vec4u */
  fireN = 104,
  /** array<FireRec, 4> */
  fire = 108,
  /** FireLook */
  fireLook = 172,
  /** words it takes */
  words = 188,
}

/** src/render/shaders/composite.wgsl struct BeamSeg, in words of four bytes */
export const enum BeamSegAt {
  /** vec4f */
  a = 0,
  /** vec4f */
  b = 4,
  /** vec4f */
  e = 8,
  /** words it takes */
  words = 12,
}

/** src/render/shaders/composite.wgsl struct FireRec, in words of four bytes */
export const enum FireRecAt {
  /** vec4f */
  a = 0,
  /** vec4f */
  b = 4,
  /** vec4f */
  c = 8,
  /** vec4f */
  d = 12,
  /** words it takes */
  words = 16,
}

/** src/render/shaders/composite.wgsl struct FireLook, in words of four bytes */
export const enum FireLookAt {
  /** vec4f */
  a = 0,
  /** vec4f */
  b = 4,
  /** vec4f */
  c = 8,
  /** vec4f */
  d = 12,
  /** words it takes */
  words = 16,
}

/** src/render/shaders/taau.wgsl struct TaauParams, in words of four bytes */
export const enum TaauAt {
  /** mat4x4f */
  prevViewProjU = 0,
  /** vec2f */
  jitter = 16,
  /** vec2f */
  inSize = 18,
  /** vec2f */
  outSize = 20,
  /** f32 */
  alpha = 22,
  /** f32 */
  clipGamma = 23,
  /** u32 */
  flags = 24,
  /** f32 */
  reactK = 25,
  /** f32 */
  kSharp = 26,
  /** f32 */
  velK = 27,
  /** f32 */
  camStill = 28,
  /** f32 */
  motionA = 29,
  /** f32 */
  histSharp = 30,
  /** words it takes */
  words = 32,
}

/** src/render/shaders/particles_common.wgsl struct EmitBlock, in words of four bytes */
export const enum EmitBlockAt {
  /** u32 */
  numRecs = 0,
  /** u32 */
  totalNew = 1,
  /** u32 */
  frameSeed = 2,
  /** u32 */
  flags = 3,
  /** array<Window, 2> */
  win = 4,
  /** array<vec4f, 14> */
  kp = 12,
  /** array<vec4f, 7> */
  ki = 68,
  /** array<EmitRec, MAX_EMITS> */
  recs = 96,
  /** array<ImpulseRec, MAX_IMPS> */
  imps = 3680,
  /** Window */
  rainWin = 3936,
  /** vec4f */
  rainCfg = 3940,
  /** vec4f */
  rainCfgB = 3944,
  /** vec4f */
  rainCfgC = 3948,
  /** vec4f */
  rainCfgD = 3952,
  /** vec4f */
  rainCfgE = 3956,
  /** vec4f */
  rainCfgF = 3960,
  /** words it takes */
  words = 3964,
}

/** src/render/shaders/particles_common.wgsl struct Window, in words of four bytes */
export const enum WindowAt {
  /** u32 */
  start = 0,
  /** u32 */
  size = 1,
  /** u32 */
  tail = 2,
  /** u32 */
  live = 3,
  /** words it takes */
  words = 4,
}

/** src/render/shaders/particles_common.wgsl struct EmitRec, in words of four bytes */
export const enum EmitRecAt {
  /** vec3f */
  pos = 0,
  /** u32 */
  count = 3,
  /** vec3f */
  dir = 4,
  /** f32 */
  speed = 7,
  /** vec3f */
  posB = 8,
  /** u32 */
  first = 11,
  /** vec3f */
  vadd = 12,
  /** u32 */
  shape = 15,
  /** vec4f */
  p = 16,
  /** vec4f */
  q = 20,
  /** vec4u */
  k = 24,
  /** words it takes */
  words = 28,
}

/** src/render/shaders/particles_common.wgsl struct ImpulseRec, in words of four bytes */
export const enum ImpulseRecAt {
  /** vec3f */
  pos = 0,
  /** f32 */
  radius = 3,
  /** vec3f */
  bias = 4,
  /** f32 */
  strength = 7,
  /** vec3f */
  eye = 8,
  /** f32 */
  up = 11,
  /** u32 */
  kinds = 12,
  /** u32 */
  seed = 13,
  /** u32 */
  iflags = 14,
  /** u32 */
  spare = 15,
  /** words it takes */
  words = 16,
}

/** what a frame's flags word raises (common.wgsl FLAG_*) */
export const FrameFlags = {
  Direct: 1,
  Indirect: 2,
  Volumetrics: 4,
  Smoke: 8,
  RcVisWeight: 16,
  SmokeShadows: 64,
  ReachMask: 32,
  Bounce: 128,
  Dither: 256,
  SoftShadow: 512,
  NightVision: 1024,
  Temporal: 2048,
  RcRoom: 32768,
  Gloss: 2097152,
  Decals: 4194304,
  MeshShadows: 8388608,
  Particles: 16777216,
  BounceCards: 33554432,
  Glass: 268435456,
} as const;

/** what a level raises, as bits of frame.levelCfg.x (common.wgsl lv) */
export const LevelSw = {
  WX_KINDS: 1,
  WX_SNOW: 2,
  SNOW_SURF: 4,
  SNOW_WISPS: 8,
  SNOW_TRACKS: 16,
  SNOW_GLOW: 32,
  SNOW_SPARKLE: 64,
  SNOW_GRAINS: 128,
  SNOW_UNDER: 256,
} as const;
