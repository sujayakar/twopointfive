// Per-frame uniform block (must match `struct Frame` in common.wgsl) + shared scene bind group.
import { Mat4, Vec3 } from '../math/vec';
import commonSrc from './shaders/common.wgsl' with { type: 'text' };
import sdfRigSrc from './shaders/sdfrig.wgsl' with { type: 'text' };
import { sdfRigWgslConsts } from './sdfrig';
import { wgslWorldConsts } from '../scene/world';
import { AIR_CELL, AIR_W, AIR_H, AIR_ROWS } from '../scene/airGrid';
import { REACH_VEC4S } from './lights';
import { LOOK_MIRRORS, LOOK_MIRROR_PLAIN, LOOK_WORDS } from './lookTex';
import { MESH_MAX_CHARS } from './meshbvh';
import { MARK_TABLE_ROWS } from '../scene/markings';
import { cutConfig, CUT_ROWS, CUT_STRIDE } from '../scene/cut';
import { GB_ROWS } from '../scene/boxes';
import { smokeGrid, smokeReadConsts } from '../smoke/smokeGrid';
import smokeReadSrc from './shaders/smoke_read.wgsl' with { type: 'text' };

/** Byte size of the per-frame uniform block = struct Frame in common.wgsl. Layout past the 416 bytes of scalars/vec4s the renderer always had (offsets in BYTES; f[]/u[] index = byte ÷ 4):
 *    416  cardCfg    vec4u              (u[104‥107])  torch bounce cards: x = count this frame; y, z = weather kinds' 3rd / 4th roof footprint packed (1 = a kind with no third roof), w = their fog packed (0 0 0 = no kind)
 *    432  cardSlots  array<vec4u, 2>    (u[108‥115])  the 8 light slots the cards are built from
 *    464  weatherA   vec4f              (f[116‥119])  rain rate, shear x, shear z, wetness
 *    480  weatherB   vec4f              (f[120‥123])  flash rgb, rain top
 *    496  weatherC   vec4f              (f[124‥127])  interior footprint x0 z0 x1 z1 = the first roof (the level's own roofs in force: + two packed roofs in cardCfg.yz and, under a kind, the fog in .w @420, u[105‥107])
 *    512  glassCfg   vec4f              (f[128‥131])  glass: reflection ×, tint depth, dust glow ×, refraction's index (0 = off)
 *    528  weatherD   vec4f              (f[132‥135])  a second no-rain footprint (the office's west stair tower; else the level's second roof) x0 z0 x1 z1
 *    544  volCfg     vec4u              (u[136‥139])  volumetric caps: x = lights per march step / per haze ray (1 or more), y = shadow-ray refresh period (steps), z = march step (tenths of a voxel), w = bits 0‥3 the SHADOW rays' smoke sample count (smokeShadowSteps; 0 = 8) | bits 8‥11 free
 *    560  prevCam    vec4f              (f[140‥143])  previous frame's camera position (w free)
 *    576  rtCfg      vec4f              (f[144‥147])  RT reflections: max roughness traced, rays per pixel, wet-ground anisotropy (+ 2 × the wet ground's break-up in tenths), the reflection target's divisor (0 = the pass is OFF this frame)
 *    592  matCfg     vec4f              (f[148‥151])  materials: analytic-highlight gain, reflection strength, per-ray luminance bound, detail-layer amount (0 = the g-buffer's detail layer off)
 *    608  p5Cfg      vec4u              (u[152‥155])  renderer v3 phase 5 (ultra extras): x, y free (the resampling gather's mode and numbers until 2026-11-07), z = the reflection pass's mesh / hit-reuse bits (all 0 = the passes as they were); w = the lowest light slot holding a TRANSIENT light this frame (gather_common's subtraction walk starts there; 0 when none is alive — review R2.2)
 *    624  jitter     vec4f              (f[156‥159])  this frame's TAAU jitter (internal px) xy, the previous render's zw
 *    640  markCfg    vec4f              (f[160‥163])  markings: base texel m (0 = off), contrast, emission ×, evaluate-at-reflection-hits
 *    656  sharpCfg   vec4f              (f[164‥167])  SHARP DIRECT (decoupled resolution): internal size xy, OUTPUT (g-buffer / composite) size zw — z 0 = off: every pass exactly as it was
 *    672  sharpCfgB  vec4f              (f[168‥171])  sharp direct's knob: w = the same-plane weight's k (xyz free)
 *    688  sharpCfgC  vec4f              (f[172‥175])  DECOUPLED JITTER under sharp direct (settings.sharpDecouple): xy = the jitter the INTERNAL raster carries relative to the (unjittered) OUTPUT raster, internal px — every internal ↔ output texel map adds it; 0 = the two rasters share one jitter (the sharp path as it was); zw spare
 *    704  rigCfg     vec4f              (f[176‥179])  SDF CHARACTER RIG (render/sdfrig.ts, shaders/sdfrig.wgsl): x = bits (64 character SELF-shadow in the direct estimators' shadow rays (common.wgsl selfShadowT) · 128 mesh shadows for NEAR receivers only (common.wgsl meshShadowT, MESH_NEAR_M); the bits under 64 are free — the round-cone proxy test is unconditional), yzw free
 *    720  rigCfgB    vec4f              (f[180‥183])  free (the smooth-union and soft-query knobs went 2026-11-07)
 *    736  giCfg      vec4f              (f[184‥187])  four FREE words (the young-history gather rays read them until 2026-11-07; no shader reads them now, the writer below still copies what it is given)
 *    752  weatherE   vec4f              (f[188‥191])  PARTICLE RAIN (rust/game_rules/src/rain_fx, EXPERIMENT): the near-field particle layer's disc — centre x, z, radius (m), and the share 0‥1 of the lattice rain it replaces inside it (on a snow level, where there is no lattice rain: 0, or NEGATIVE the metres past the level's rim the snow goes on falling — particles_common.wgsl snowPast) (volumetrics.wgsl rainNearKeep); w 0 = no particle layer: the lattice exactly as it was
 *    768  weatherF   vec4f              (f[192‥195])  PARTICLE RAIN v2: the drop's PHASE the lattice rain shares with the particles (volumetrics.wgsl rainLight) — x = isotropic floor, y = forward-lobe weight, z = its Henyey-Greenstein g, w = × gain; w 0 = the lattice's isotropic sum exactly as it was
 *    784  shadeCfg   vec4f              (f[196‥199])  THE LOOK PROTOTYPE, eyes adjust to shade (engine.ts lookShade; the one exposure model's, place_common.wgsl lookExpoFor, and the word composite.wgsl asks before it reads a pixel's place): x = the share of its dark a place darker than the open air is brought up by (0 = off), y = open air's mean radiance from the sky alone (the model's open air until the place field is live), z = spare (the most the rows before the model brought a pixel up), w = open ground's irradiance from the sky alone (luminances; the shoulder's white where the Contrast row says none). Zeros = the picture as it was
 *    800  veilA      vec4f              (f[200‥203])  THE LOOK PROTOTYPE, loose snow in the air (composite.wgsl lookPowder; render/lookveil.ts lookVeilFrame): xy = 1 ∕ the floor plan the powder's field covers (its width, its depth, m), z = how much of the powder is drawn 0‥1 (0 = none, every statement of it skipped), w = what the sun (or the moon) puts on level ground, as luminance
 *    816  veilB      vec4f              (f[204‥207])  xy = that plan's corner (m, x z), zw = the way to the sun along the ground per metre of height (x z; 0 = no sun: no lit side, no shade under it)
 *    832  eyeCfg     vec4f              (f[208‥211])  THE LOOK PROTOTYPE, one exposure (render/lookexpo.ts lookExpoFrame; place_common.wgsl lookExpoFor): x = the most a place is brought up, the night's exposure over the hour's (1 = none said, or a night), y = 1 (it was the day's share: the model asks no hour since 11-04), z = the power of the eyes' darkness the open is overexposed by (the Glare outside row), w = −1 − the Eyes follow Sam row: UNDER 0 = THE MODEL IS ON (composite.wgsl lookShadeGain, volumetrics.wgsl airShow), zeros = no pixel takes a gain by its place
 *    848  gradeCfg   vec4f              (f[212‥215])  THE LOOK PROTOTYPE, the day's contrast is the open air's (render/lookgrade.ts; place_common.wgsl lookExpoFor and lookRoll): x = 1 ∕ the contrast the tone map turns the day's stops by (0 = none is undone), y = the radiance the turn is about (sunlit ground, before the exposure). A place is toned as the open air is and then shown as much darker as it is to be; z = spare (the read of the cache on this side of a wall, gone 11-04 with the guess it served: composite.wgsl lookPlaceLevels), w = spare (the Room contrast row's, gone with the rows the model replaced)
 *    864  lampCfg    vec4f              (f[216‥219])  THE LOOK PROTOTYPE, light keeps to its room by day (the look panel's row `rooms`; zeros = every floor and glow as it was): x = × the night's lamp strength a fixture is drawn at this hour (render/photometry.ts: the hour's exposure), which the direct estimators' fixed floor is held at the same share of, y = × a glowing face's drawn strength as a RAY takes it (the gather, the cache: its night light at the day's lamp strength, not the glass as the eye sees it)
 *    880  sideCfg    vec4u              (u[220‥223])  THE LOOK PROTOTYPE, which side of a wall (the look panel's row `walls`; render/wallSides.ts; zeros = every cache read as it
 *                                                was): x = 1 the words below are this frame's level · y, z = the smoke lab's drawing rows (FrameParams.smokeDraw; zeros = as it was) · w = the look panel's Snow wisps drawing rows (FrameParams.powderDraw; 0 = as it was)
 *    896  placeCfg   vec4f              (f[224‥227])  THE LOOK PROTOTYPE, light keeps its order across places (the look panel's row `order`; composite.wgsl lookExpoGain): x = how far a pixel's own light caps its place's gain (0 = every pixel of a place by the place's gain, as it was; 1 = never by more than a place holding the pixel's light would get); y = the slot of the light list's first portal + 1, 0 with none (common.wgsl skyKeep: the bounce's rays ask which sky is theirs only then, and walk the list from there); z, w = the eyes' widest (render/lookexpo.ts lookExpoBound: the dim level in the night's units, never 0 while the model is on, and the most a place under it is shown by over the night's exposure).
 *    912  levelCfg   vec4u              (u[228‥231])  WHAT THE LEVEL RAISES (LevelSw below; common.wgsl lv): x = its switches, a bit each — the weather kinds' reads, the snow, the snow's look rows. They were constants of the scene prelude, so a level with another set was other shader text and every program that read one compiled again at that level's start; as data, every level asks for the one text
 *    928  sides      array<vec4u, 1750> (u[232‥7231]) its words, a u32 a cell of the floor plan (render/wallSides.ts placeSides; common.wgsl sideAt)
 *  28928  sideCols   array<vec4u, 140>  (u[7232‥7791]) the place of each probe column of the cache's first cascade, a nibble a column, room for the finest cascade the debugger offers (wallSides.ts WallSides.cols; lighting_common.wgsl diceTapsBy)
 *  31168  = FRAME_BYTES. Every member is 16-byte aligned; adding one = a row here + the struct member in common.wgsl (same order) + its writer in writeFrame + FRAME_BYTES, and IN FRONT OF `sides` with FRAME_HEAD_BYTES raised: only the head is sent every frame (a member put after the tables reaches the GPU when a door comes to rest). tools/qa/frame-layout.ts checks all of it. */
export const FRAME_BYTES = 31168;
/** the bytes before the look prototype's two tables (levelCfg and all before it): what a frame sends when the tables have not changed (engine.ts) */
export const FRAME_HEAD_BYTES = 928;
/** at most this many torch bounce cards a frame (cards.wgsl workgroups; fgather.wgsl stages 4 rows each in workgroup memory and keeps the exclusion set in 8 bits) */
export const MAX_CARDS = 8;

/** What the weather (rust/game_rules/src/weather) hands the frame: everything the volumetric pass needs to draw rain / splashes / the sky flash in the OUTDOOR air. All zero =
 *  no weather work anywhere (the shader's first test), which is what ?noweather and every Engine nobody attaches a Weather to get. */
export interface WeatherFrame {
  /** rain rate 0‥1: the fraction of the drop-column lattice that is falling (0 = dry: the pass skips the whole block) */
  rain: number;
  /** SNOWFALL 0‥1 on a level built for snow (rust/game_rules/src/weather → rust/game_rules/src/snow_fx: how many flakes the particle layer keeps in the air). Read on the CPU only — it is
   *  not in the Frame; with it > 0 `rain` is 0, so no streak, ring or wet ground is drawn anywhere. Absent / 0 = none. */
  snow?: number;
  /** wind shear of the falling streaks: metres sideways per metre of height, x and z (wind speed ÷ fall speed) */
  shearX: number; shearZ: number;
  /** ground wetness 0‥1 (splash rings on outdoor ground scale with it; the darkened / glossed asphalt is done on the boxes themselves) */
  wet: number;
  /** radiance added to outdoor air and rain by a lightning stroke's lit sky this frame (linear rgb; 0 outside a flash) */
  flash: Vec3;
  /** streaks live below this height (m) and fade in over the metre under it */
  rainTop: number;
  /** the building's interior footprint x0, z0, x1, z1: rays that end inside it draw no rain, and no rain is drawn inside its column (the roof is invisible, the rooms stay dry) */
  interior: [number, number, number, number];
  /** a second, small roofed footprint outside the main one — the west stair tower the QRF comes up (rays ending in it draw no rain; all zero = none) */
  annex: [number, number, number, number];
  /** WEATHER KINDS (rust/game_rules/src/weather_kinds; absent / 0 = none — the frame as it was): a third and a fourth roof footprint packed x0 z0 x1 z1 in 8 bits each of
   *  0.2 m (packRect), and the fog in the outdoor air in 32 bits (packFog). They ride in the three spare words of cardCfg (.y .z .w): the Frame keeps its size. */
  roofC?: number; roofD?: number; fog?: number;
  /** PARTICLE RAIN (experiment, rust/game_rules/src/rain_fx): the near-field particle layer's disc — centre x, z, radius (m), and 0‥1 = the share of the LATTICE rain the shader drops
   *  inside it (volumetrics.wgsl rainNearKeep: the streaks / splash rings there are the particles'); all zero = no particle layer, the lattice exactly as it was */
  nearFade: [number, number, number, number];
  /** PARTICLE RAIN v2: the phase function the lattice's streaks share with the particles' (iso, forward weight, HG g, gain — rust/game_rules/src/rain_fx PRAIN); all zero (w 0) = the isotropic sum as it was */
  phase?: [number, number, number, number];
}
export const noWeather = (): WeatherFrame => ({ rain: 0, shearX: 0, shearZ: 0, wet: 0, flash: [0, 0, 0], rainTop: 4.5, interior: [0, 0, 0, 0], annex: [0, 0, 0, 0], nearFade: [0, 0, 0, 0] });

export const FrameFlags = {
  Direct: 1, Indirect: 2, Volumetrics: 4, Smoke: 8, RcVisWeight: 16, ReachMask: 32 /* direct pass + secondary vertices: skip lights whose CPU reach mask (reach.ts) excludes the shading point's cell — set from settings.lightReach */, SmokeShadows: 64, Bounce: 128, Dither: 256, SoftShadow: 512, NightVision: 1024, Temporal: 2048, RcRoom: 32768 /* THE LOOK PROTOTYPE, light keeps to its room (engine.ts, the look panel's row `rooms`): a cache ray's hit takes its bounced light from the cell the ray itself came through (lighting_common.wgsl hitRadiance), beside the cascades' hand-over overlap (render/rc.ts overlap) */,
  Gloss: 2097152 /* EXPERIMENT: traced glossy reflections on floors flagged BoxFlag.Gloss (one reflection ray per gloss texel in the final gather, its own half-res specular texture + history + one blur pass, added un-multiplied by albedo in composite) — default off; off stores / filters / reads nothing (set from settings.gloss) */,
  Decals: 4194304 /* decals (E1): the g-buffer's fragment stage applies the marks in render/decals.ts (bullet holes, scorches, scuffs, chips) to the albedo / normal / gloss-keep it stores — set from settings.decals while any decal exists; off = fsBox exactly as it was */,
  MeshShadows: 8388608 /* EXPERIMENT: every shadow / occlusion ray (direct pass, gather hits, cascades, beams, probes) tests characters against their skinned triangle mesh — a per-character refit BVH written by render/meshShadows.ts each frame, walked in common.wgsl meshShadowT — instead of the 15 analytic capsules; primary visibility and closest-hit shading (gather / gloss hits) stay as they are. Default off (settings.meshShadows); needs meshBinding.enabled; off is the untouched capsule path. */,
  Particles: 16777216 /* GPU particles (E4) (render/particles.ts — sparks, debris chips, dust, casings simulated in a compute pass and splatted into the HDR target after the composite). Set from settings.gpuParticles ∧ a Particles attached; no scene shader reads it (the pass graph does) — it rides in the flags word so the HUD / captures say which path drew the sparks. Off = the CPU box sparks exactly as they were. */,
  BounceCards: 33554432 /* torch bounce cards (settings.torchBounceCards): each carried spot light's footprint on the surface it hits is rebuilt every frame as a one-sided analytic emitter (cards.wgsl) that the final gather samples with one shadow ray instead of discovering the hotspot with its cosine rays — which exclude that light's direct term where they land inside the footprint, so nothing is counted twice (fgather.wgsl). Off = no card pass, nothing sampled or excluded: bit-identical. */,
  Glass: 268435456 /* glass panes (E3) — boxes flagged BoxFlag.Glass are kept out of the raster and the trace grid and listed in SceneInfo.glass; composite lays Fresnel reflection (traced in the final gather like the gloss floors, sharing their specular texture + history + à-trous pass), the pane's tint and a dust glow over what the G-buffer shows behind them, and the direct pass tints light that crossed a pane (set from settings.glass; off = no pane list at all: empty openings, the level as it was) */,
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
  directCfg: [number, number, number, number];   // free, free, blurCapPx, free (the old estimator's ray budgets and history weight until 2026-11-07)
  numTransient: number;        // transient (ttl) lights alive this frame — lets the gather skip its subtraction loop
  secMinE: number;             // shadow-ray threshold for lights at cascade interval hits (W/m², unshadowed)
  glassCfg: [number, number, number, number];   // glass experiment: x = reflection strength (× the physical Fresnel term), y = tint depth (exponent on the pane's albedo per crossing), z = dust glow gain, w = refraction: the index of refraction while settings.glassRefract (≥ 1), 0 = off (composite.wgsl glassRefractPath)
  lossyCfg: [number, number, number, number];   // lossy options: gather divisor (2|3), volumetric divisor (2|4), free (the old estimator's dim-ray threshold until 2026-11-07), the gather's relative firefly bound K (0 = off) — see RenderSettings
  /** torch bounce cards this frame (FrameFlags.BounceCards): the light SLOTS (indices into this frame's uploaded light list) whose footprint cards.wgsl rebuilds, ≤ MAX_CARDS; empty = the card pass is not dispatched and the gather touches nothing */
  cardSlots: number[];
  weather: WeatherFrame;       // rain / splashes / sky flash in the outdoor air (volumetrics.wgsl (c)); noWeather() = nothing drawn
  /** volumetric worst-case caps (volumetrics.wgsl): [K = lights traced per smoke-march step / per haze ray (0 = off, the pass as it was), shadow-ray refresh period in march steps (1‥3), march step in tenths of a voxel (10‥30), 0] */
  volCfg: [number, number, number, number];
  /** the weather air march's step cap, packed into volCfg.w bits 24‥28 (0 = half the tier's beam steps; volumetrics.wgsl wxMaxSteps) */
  wxAirSteps?: number;
  /** the camera position the PREVIOUS frame was rendered from (prevViewProj's eye — the engine snapshots it per render) */
  prevCamPos: Vec3;
  /** RT reflections (renderer v3 phase 4, reflect.wgsl): [max roughness that gets a ray, rays per pixel, wet-ground anisotropy 0‥1, the reflection target's divisor — 0 = the pass is off this
   *  frame: nothing dispatched, composite reads nothing] */
  rtCfg: [number, number, number, number];
  /** materials: [analytic-highlight gain (the GGX image of point / spot / panel lights on a finish — 1 = physical), reflection strength ×, per-ray luminance bound, detail-layer amount 0‥1
   *  (the g-buffer's wear / dust / edge terms; 0 = fsBox exactly as it was)] */
  matCfg: [number, number, number, number];
  /** renderer v3 phase 5, the ultra-only extras (engine.ts settings rtMeshHits / rtHitReuse; all zero = every pass exactly as it was): [free, free (the resampling gather's mode and
   *  numbers until 2026-11-07), the reflection pass's bits (1 mesh closest-hit for characters, 2 second-ray decorrelation + hit reuse), 0] */
  p5Cfg: [number, number, number, number];
  /** TAAU jitter in internal pixels: [this frame's x, y, the previous render's x, y] (all 0 without TAAU) */
  jitter: [number, number, number, number];
  /** MARKINGS (scene/markings.ts, shaders/markings.wgsl): [base texel in metres — 0 = markings off everywhere (the frame as it was), contrast 0‥1, × the emission of emissive markings,
   *  1 = markings are evaluated at reflection hits too] */
  markCfg: [number, number, number, number];
  /** SHARP DIRECT (renderer v3 follow-up, docs/internal/sharp-direct.md): [internal width, height, OUTPUT width, height] while the decoupled-resolution path is live this frame —
   *  the g-buffer, the direct light's unshadowed term, the composite and the TAA run at the output size, everything stochastic stays internal and is joint-bilateral upsampled. z = 0 = off:
   *  every pass exactly as it was (composite / taau / particles_draw read the same statements they always did). Both frame uniforms (internal-screen and output-screen) carry the same four. */
  sharpCfg: [number, number, number, number];
  /** sharp direct's knob: [free, free, free, same-plane k (sharp_direct.wgsl)] */
  sharpCfgB: [number, number, number, number];
  /** DECOUPLED JITTER (settings.sharpDecouple, sharp direct only): [the internal raster's jitter relative to the output raster in internal px x, y, 0, 0] — 0 = one jitter for both (as it was) */
  sharpCfgC: [number, number, number, number];
  /** SDF CHARACTER RIG (render/sdfrig.ts + shaders/sdfrig.wgsl): [bits (64 character self-shadow · 128 mesh shadows near only; the bits under 64 free), free, free, free] */
  rigCfg: [number, number, number, number];
  /** free (the smooth-union and soft-query knobs went 2026-11-07) */
  rigCfgB: [number, number, number, number];
  /** GI-IN-MOTION (fgather.wgsl `main`): [young-history ray multiplier k (≤ 1 = off — the gather byte for byte as it was), the per-frame cap on texels that may take it (count), the history length (frames)
   *  under which a texel is 'young', 0] */
  giCfg?: [number, number, number, number];   // (free words: see the layout above)
  shadeCfg?: [number, number, number, number];   // (THE LOOK PROTOTYPE's shade row, engine.ts lookShade: optional — absent = zeros = off)
  gradeCfg?: [number, number, number, number];   // (THE LOOK PROTOTYPE's Contrast row as the one exposure model tones a place by it, render/lookgrade.ts: optional — absent = zeros = nothing undone)
  placeCfg?: [number, number, number, number];   // (THE LOOK PROTOTYPE, light keeps its order across places: optional — absent = zeros = every pixel by its place's gain)
  lampCfg?: [number, number, number, number];   // (THE LOOK PROTOTYPE, light keeps to its room by day: optional — absent = zeros = every floor and glow as it was)
  sides?: Uint32Array | null;   // (THE LOOK PROTOTYPE, which side of a wall: render/wallSides.ts WallSides.words, 7,000 of them. They are the exposure's too (composite.wgsl lookPlaceLevels reads a cell's lines), so they ride whenever the place model or the Walls have two sides row is on; the FLAG, sideCfg.x, is the cache reads' alone and is up only with `sideCols`)
  /** THE SMOKE LAB's drawing rows as the two words the shaders read (vol_common.wgsl HOW SMOKE IS DRAWN, AS ROWS; Engine.smokeDrawWords): absent or zeros = the picture as it was */
  smokeDraw?: readonly [number, number] | null;
  /** THE LOOK PANEL's Snow wisps drawing rows as the word the composite reads (render/powderField.ts powderDrawWord): absent or 0 = the shader's own numbers */
  powderDraw?: number;
  sideCols?: Uint32Array | null;   // (… and the place of each probe column, 560 words: WallSides.cols; absent = the flag down: every cache read as it was)
  eyeCfg?: [number, number, number, number];   // (THE LOOK PROTOTYPE's one exposure model, render/lookexpo.ts lookExpoFrame: optional — absent = zeros = off)
  veilA?: [number, number, number, number]; veilB?: [number, number, number, number];   // (THE LOOK PROTOTYPE's loose snow in the air, render/lookveil.ts lookVeilFrame: optional — absent = zeros = none)
}

const ZERO4: [number, number, number, number] = [0, 0, 0, 0];
/** typed views over a frame block, made once per ArrayBuffer (the engine writes the same two blocks every frame — review R1.4: no fresh views per call) */
const frameViews = new WeakMap<ArrayBuffer, { f: Float32Array; u: Uint32Array; b: Uint8Array }>();
function viewsOf(buf: ArrayBuffer) { let v = frameViews.get(buf); if (!v) { v = { f: new Float32Array(buf), u: new Uint32Array(buf), b: new Uint8Array(buf) }; frameViews.set(buf, v); } return v; }
export function writeFrame(dst: ArrayBuffer, p: FrameParams) {
  const { f, u } = viewsOf(dst);
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
  f[92] = p.rcJitter[0]; f[93] = p.rcJitter[1]; f[94] = 0;   /* (free: the old estimator's adaptive minimum until 2026-11-07) */ f[95] = p.numTransient;
  f[96] = p.directCfg[0]; f[97] = p.directCfg[1]; f[98] = p.directCfg[2]; f[99] = p.directCfg[3];
  f[100] = p.lossyCfg[0]; f[101] = p.lossyCfg[1]; f[102] = p.lossyCfg[2]; f[103] = p.lossyCfg[3];
  const nc = Math.min(p.cardSlots.length, MAX_CARDS);
  u[104] = nc; u[105] = 0; u[106] = 0; u[107] = 0;                     // cardCfg: x = card count this frame
  for (let k = 0; k < MAX_CARDS; k++) u[108 + k] = k < nc ? p.cardSlots[k] >>> 0 : 0;   // cardSlots: array<vec4u, 2> = 8 light slots
  const w = p.weather;
  f[116] = w.rain; f[117] = w.shearX; f[118] = w.shearZ; f[119] = w.wet;                 // weatherA @464
  f[120] = w.flash[0]; f[121] = w.flash[1]; f[122] = w.flash[2]; f[123] = w.rainTop;   // weatherB @480
  f[124] = w.interior[0]; f[125] = w.interior[1]; f[126] = w.interior[2]; f[127] = w.interior[3];   // weatherC @496
  f[128] = p.glassCfg[0]; f[129] = p.glassCfg[1]; f[130] = p.glassCfg[2]; f[131] = p.glassCfg[3];   // glassCfg @512
  f[132] = w.annex[0]; f[133] = w.annex[1]; f[134] = w.annex[2]; f[135] = w.annex[3];   // weatherD @528
  u[105] = (w.roofC ?? 0) >>> 0; u[106] = (w.roofD ?? 0) >>> 0; u[107] = (w.fog ?? 0) >>> 0;   // cardCfg.yzw @420: the weather kinds' extra roofs + fog (0 0 0 = none, as these words always were)
  u[136] = p.volCfg[0] >>> 0; u[137] = p.volCfg[1] >>> 0; u[138] = p.volCfg[2] >>> 0; u[139] = (p.volCfg[3] | (((p.wxAirSteps ?? 0) & 31) << 24)) >>> 0;   // volCfg @544
  f[140] = p.prevCamPos[0]; f[141] = p.prevCamPos[1]; f[142] = p.prevCamPos[2]; f[143] = 0;    // prevCam @560 (w free)
  f[144] = p.rtCfg[0]; f[145] = p.rtCfg[1]; f[146] = p.rtCfg[2]; f[147] = p.rtCfg[3];           // rtCfg @576
  f[148] = p.matCfg[0]; f[149] = p.matCfg[1]; f[150] = p.matCfg[2]; f[151] = p.matCfg[3];       // matCfg @592
  u[152] = p.p5Cfg[0] >>> 0; u[153] = p.p5Cfg[1] >>> 0; u[154] = p.p5Cfg[2] >>> 0; u[155] = p.p5Cfg[3] >>> 0;   // p5Cfg @608
  f[156] = p.jitter[0]; f[157] = p.jitter[1]; f[158] = p.jitter[2]; f[159] = p.jitter[3];       // jitter @624
  f[160] = p.markCfg[0]; f[161] = p.markCfg[1]; f[162] = p.markCfg[2]; f[163] = p.markCfg[3];   // markCfg @640
  f[164] = p.sharpCfg[0]; f[165] = p.sharpCfg[1]; f[166] = p.sharpCfg[2]; f[167] = p.sharpCfg[3];   // sharpCfg @656
  f[168] = p.sharpCfgB[0]; f[169] = p.sharpCfgB[1]; f[170] = p.sharpCfgB[2]; f[171] = p.sharpCfgB[3];   // sharpCfgB @672
  f[172] = p.sharpCfgC[0]; f[173] = p.sharpCfgC[1]; f[174] = p.sharpCfgC[2]; f[175] = p.sharpCfgC[3];   // sharpCfgC @688
  f[176] = p.rigCfg[0]; f[177] = p.rigCfg[1]; f[178] = p.rigCfg[2]; f[179] = p.rigCfg[3];               // rigCfg @704
  f[180] = p.rigCfgB[0]; f[181] = p.rigCfgB[1]; f[182] = p.rigCfgB[2]; f[183] = p.rigCfgB[3];           // rigCfgB @720
  const gc = p.giCfg ?? ZERO4; f[184] = gc[0]; f[185] = gc[1]; f[186] = gc[2]; f[187] = gc[3];   // giCfg @736 (p.giCfg; absent = zeros = the gather as it was)
  const nf = w.nearFade ?? ZERO4;
  f[188] = nf[0]; f[189] = nf[1]; f[190] = nf[2]; f[191] = nf[3];   // weatherE @752 (particle rain's near-field disc; a WeatherFrame written before the field existed reads as zeros)
  const pf = w.phase ?? ZERO4;
  f[192] = pf[0]; f[193] = pf[1]; f[194] = pf[2]; f[195] = pf[3];   // weatherF @768 (particle rain v2: the drop's phase for the lattice; zeros = isotropic as it was)
  const sh = p.shadeCfg ?? ZERO4; f[196] = sh[0]; f[197] = sh[1]; f[198] = sh[2]; f[199] = sh[3];   // shadeCfg @784 (the look prototype's shade row; absent = zeros = off)
  const va = p.veilA ?? ZERO4; f[200] = va[0]; f[201] = va[1]; f[202] = va[2]; f[203] = va[3];   // veilA @800 (the look prototype's loose snow in the air; absent = zeros = none)
  const vb = p.veilB ?? ZERO4; f[204] = vb[0]; f[205] = vb[1]; f[206] = vb[2]; f[207] = vb[3];   // veilB @816
  const ey = p.eyeCfg ?? ZERO4; f[208] = ey[0]; f[209] = ey[1]; f[210] = ey[2]; f[211] = ey[3];   // eyeCfg @832 (the look prototype's one exposure model; absent = zeros = off)
  const gr = p.gradeCfg ?? ZERO4; f[212] = gr[0]; f[213] = gr[1]; f[214] = gr[2]; f[215] = gr[3];   // gradeCfg @848 (the look prototype's Contrast row, which the one exposure model tones a place by; absent = zeros = nothing undone)
  const lc = p.lampCfg ?? ZERO4; f[216] = lc[0]; f[217] = lc[1]; f[218] = lc[2]; f[219] = lc[3];   // lampCfg @864 (the look prototype's Light stays in its room row by day; absent = zeros = off)
  const sc = p.sideCols ?? null, sd = p.sides ?? null; u[220] = sd && sc ? 1 : 0; u[221] = p.smokeDraw ? p.smokeDraw[0] >>> 0 : 0; u[222] = p.smokeDraw ? p.smokeDraw[1] >>> 0 : 0; u[223] = (p.powderDraw ?? 0) >>> 0; if (sd) u.set(sd, 232); if (sd && sc) u.set(sc, 7232);
  const pc = p.placeCfg ?? ZERO4; f[224] = pc[0]; f[225] = pc[1]; f[226] = pc[2]; f[227] = pc[3];   // placeCfg @896 (the look prototype's Light in order row; absent = zeros = off)
  u[228] = levelSwitches(); u[229] = 0; u[230] = 0; u[231] = 0;   // levelCfg @912: what the level raises, read here off the page's own switches so that no caller can leave it out. The last member of the head: the two tables above follow it   // sideCfg @880, sides @896 (the look prototype's Walls have two sides row; absent = the flag down, the words left as they lay)
}

/** The OUTPUT-screen twin of a frame block already written by writeFrame: the same bytes with screen / invScreen (f[56‥59]) replaced by the output size — what the passes that run at the OUTPUT
 *  resolution under sharp direct (the g-buffer raster, the unshadowed-direct + composite kernels, the TAA resolve, the particles' splat) bind as group 0, so `frame.screen`, worldFromDepth and every
 *  pixel-footprint term mean THEIR pixel there with no shader knowing the difference. */
export function writeFrameOut(dst: ArrayBuffer, src: ArrayBuffer, outW: number, outH: number, mats?: { viewProj: Mat4; invViewProj: Mat4; prevViewProj: Mat4 }) {
  viewsOf(dst).b.set(viewsOf(src).b);
  const f = viewsOf(dst).f; f[56] = outW; f[57] = outH; f[58] = 1 / outW; f[59] = 1 / outH;
  if (mats) { f.set(mats.viewProj, 0); f.set(mats.invViewProj, 16); f.set(mats.prevViewProj, 32); f[156] = 0; f[157] = 0; f[158] = 0; f[159] = 0; }   // (+ frame.jitter @624 zeroed: this raster has none) — DECOUPLED JITTER: the output raster's own (unjittered) matrices — the g-buffer, U, the composite, GBAA and the 1 : 1 resolve then run on a still raster while the internal terms keep their jitter
}

/** WGSL prelude for every shader that touches the scene: world consts (+ the size of the per-light reach uniform, which needs lights.ts's MAX_LIGHTS) + common declarations (group 0). */
/** How the per-light reach rows are bound (set before the Engine is built): a read-only STORAGE buffer by default — measured 0.12–0.17 ms cheaper on the
 *  direct pass than the uniform on Apple GPUs (divergent indexing of a uniform array), A/B'd both ways twice — which makes 9 storage buffers in the compute
 *  stage; an adapter that only offers the WebGPU default of 8 gets the UNIFORM binding instead (the Engine constructor decides). */
export const reachBinding = { storage: true };
/** Whether the mesh-shadow buffer (render/meshShadows.ts: per-character refit triangle BVHs + skinned corners, ONE read-only storage buffer at binding 13) is
 *  part of the scene group at all. It costs a storage-buffer slot in every scene pipeline whether or not settings.meshShadows is on, so the Engine
 *  constructor turns it off on adapters whose per-stage budget cannot take scene 6 + reach 1 + mesh 1 + the cascade trace's / probe's 2 = 10 (Chrome on
 *  Metal offers exactly 10); the prelude then declares a stub meshLoad() and the setting is inert (engine.meshShadowsAvailable says which). */
export const meshBinding = { enabled: true };
/** GLASS BREAKING (rust/game_rules/src/glassbreak, docs/internal/glassbreak.md): the compile-time switch for its shader reads — the scene prelude's `const GLASS_BREAK` (glassTrace /
 *  glassTransmit passing fallen cells in common.wgsl, the crack network in composite.wgsl). host/drawpage.ts raises it at BOOT from the persisted `tune.glassBreak` before the Engine
 *  builds a pipeline; false = every such statement is dead code the compiler drops — the modules as they were (flag-off identity is hash-proven against main). A page
 *  reload applies a change of the switch to the GPU side. */
export const glassBreakConfig = { on: false };
/** WHAT A LEVEL RAISES IS DATA, NOT SHADER TEXT (2026-11-07). The three tables below say what the page's level asks of the shaders: the weather kinds' reads,
 *  the snow, the rows of the snow's look. host/drawpage.ts writes them at boot, as it always did. They were constants of the scene prelude (`const WX_KINDS: bool`
 *  and eight more), so the office, a level with its own roofs and a snow level were three sets of shader text, and a visit that went from the title to a
 *  mission compiled every program that read one of them a second time (about a third of the programs between the title and a snow level). Now each is a bit of the frame
 *  block's `levelCfg.x` (`LevelSw`, written by writeFrame from these tables) and a shader asks `lv(WX_KINDS)`: the same answer at every use, on every
 *  level, from ONE text. A branch a level does not raise is not taken; it is no longer left out of the program. Nothing in the update reads the word. */
/** WEATHER KINDS (rust/game_rules/src/weather_kinds, rust/game_rules/src/weather): their shader reads (the level's own roofs and the fog in the outdoor air: common.wgsl wx*, read by
 *  composite / vol_common / volumetrics / fgather / particles_common). host/drawpage.ts raises it at BOOT when the page is opened with ?weather=<kind>, or
 *  on a level that is not the office's family, or once a game is stood under its own roofs (Weather.levelRoofs) — its own storm reads its own roofs; false = no such statement runs. */
export const weatherKindConfig = { on: false };
/** SNOW (rust/game_rules/src/weather_kinds SNOW_LEVELS, rust/game_rules/src/snow_fx): the particle shaders' flake half, `lv(WX_SNOW)` (particles.wgsl rainStep: a flake rides the air,
 *  flutters, wraps round the view's disc and lands without a splash; particles_draw.wgsl rainFragment: a soft round flake, nothing refracted). host/drawpage.ts raises
 *  it at BOOT on a page whose level is one built for snow; false = no such statement runs. A page is one level for its whole life, so the two never meet. */
export const snowConfig = { on: false };
/** THE SNOW'S LOOK (rust/game_rules/src/weather_kinds SNOW_LOOK; picture only): the rows of a page opened on a snow level, raised by host/drawpage.ts at BOOT with snowConfig.
 *  `surface` → the g-buffer module's second writing (render/gbuffer.ts `GBUF_SNOW`: its programs come in two texts and a level picks; no shader asks the level word's SNOW_SURF bit, which says the same to whoever reads the word): the g-buffer shades the snow paint as one surface off the level's snow field and draws its drifts as a mesh
 *  (shaders/snow.wgsl, render/snowField.ts). `wisps` → `lv(SNOW_WISPS)`: the particle shaders' half of the wisps the wind runs over the ground
 *  (rust/game_rules/src/snow_fx). false = no such statement runs. */
export const snowLookConfig = { surface: false, wisps: false, tracks: false, glow: false, sparkle: false, grains: false, under: false };   // (`grains` → `lv(SNOW_GRAINS)`: the particle shaders' half of the fine grains that ride the air field, render/airField.ts; `tracks` → `lv(SNOW_TRACKS)`: snow.wgsl reads the trail texture; `glow` → `lv(SNOW_GLOW)`: the lighting shaders' wrap on snow pixels; `sparkle` → `lv(SNOW_SPARKLE)`: composite.wgsl's facet flashes on lit fresh snow; `under` → `lv(SNOW_UNDER)`: a flake asks the roofs as one)
/** the bits of the frame block's `levelCfg.x`, and the names the shaders ask them by (`lv(NAME)`; the scene prelude writes these as its consts, so the two
 *  cannot come to differ) */
export const LevelSw = { WX_KINDS: 1, WX_SNOW: 2, SNOW_SURF: 4, SNOW_WISPS: 8, SNOW_TRACKS: 16, SNOW_GLOW: 32, SNOW_SPARKLE: 64, SNOW_GRAINS: 128, SNOW_UNDER: 256 } as const;
/** what the page's level raises, as that word */
export function levelSwitches(): number {
  const S = LevelSw;   // (each table named whole where it is read: the checks of a row ask which files read it)
  return ((weatherKindConfig.on ? S.WX_KINDS : 0) | (snowConfig.on ? S.WX_SNOW : 0) | (snowLookConfig.surface ? S.SNOW_SURF : 0) | (snowLookConfig.wisps ? S.SNOW_WISPS : 0) | (snowLookConfig.tracks ? S.SNOW_TRACKS : 0)
    | (snowLookConfig.glow ? S.SNOW_GLOW : 0) | (snowLookConfig.sparkle ? S.SNOW_SPARKLE : 0) | (snowLookConfig.grains ? S.SNOW_GRAINS : 0) | (snowLookConfig.under ? S.SNOW_UNDER : 0)) >>> 0;
}
/** THE VEIL AT THE BUILDING'S SHELL (common.wgsl wxFogApplySurf — a picked weather's fog on the outside faces of outside walls, a ramp across a roof's edge): the
 *  scene prelude's `const WX_EDGE`. On by default; host/pictureopens.ts lowers it at BOOT for ?nowallfog = the fog classed by the surface point alone, the picture as it was
 *  (composite.wgsl takes wxFogApply, the statement it always had). Read only under WX_KINDS: with no weather picked it is dead code either way. Picture only. */
export const weatherEdgeConfig = { on: true };
/** THE WEATHER'S AIR AS A VOLUME (common.wgsl wxAir / wxIndoor, volumetrics.wgsl wxAirMarch; rust/game_rules/src/air_dust): a picked weather's dust / fog is a MEDIUM the
 *  volumetric pass marches — thicker the more of it a pixel looks through, lit by the sun / moon with shadow rays and by the sky's light, carried into rooms at
 *  open doors and blown walls — in place of composite.wgsl's flat veil over what counts as outdoors (wxFogApply / wxFogApplySurf). The scene prelude's `const WX_VOL`.
 *  On by default; host/pictureopens.ts lowers it at BOOT for ?flatfog = the veil exactly as it was. Read only under WX_KINDS: with no weather picked it is dead code either way. */
export const weatherVolConfig = { on: true };
/** RAIN IS NOT LIT THROUGH A SHUT DOOR OR A WALL (volumetrics.wgsl rainLight): the rain's streaks and splashes were lit UNSHADOWED by every light whose reach
 *  mask (render/reach.ts) has their cell — and a mask counts every door as standing open and knows nothing of a light that is carried, so a bulb in a shut room
 *  lit the rain in a wedge outside its door and a weapon light pointed at a wall from inside lit the rain beyond it. The scene prelude's `const RAIN_SHADOW`:
 *  one shadow ray where the mask is not sure (a cell reached only through a doorway or by a sliver), for a carried light and for a flash. On by default;
 *  host/pictureopens.ts lowers it at BOOT for ?norainshadow = the rain lit as it was. Picture only. */
export const rainShadowConfig = { on: true };
/** A SPLASH IS THE SAME SPLASH AT EVERY ZOOM (volumetrics.wgsl splashAt; the maintainer, 10-19, three zooms of one yard: "the low res rain isn't scale
 *  invariant"). The ring a landing drop throws up is drawn in the volumetric pass, whose texel is 2 or 4 pixels: the ring's line is never drawn thinner than
 *  a texel AT ITS DISTANCE, and it was drawn at full brightness however wide that made it — so with the camera pulled back a 2 cm ring became a blob a
 *  texel or two across (30 cm of yard) as bright as the ring, and a third of the lit ground was blobs where, close in, a hundredth of it is rings. The scene
 *  prelude's `const RAIN_SCALE`: the ring keeps its LIGHT, not its brightness — drawn wider than its 6 mm line it is dimmer by the footprint it gained (what
 *  the streaks beside it always did: RAIN_W / halfW). Close in (a texel under 6 mm at the ground) nothing changes. On by default; host/pictureopens.ts lowers it at BOOT
 *  for ?norainscale = the rings as they were. Picture only. */
export const rainScaleConfig = { on: true };
/** RAIN OVER A WALL'S FACE (shaders/common.wgsl wxUnderRoof; the maintainer, 10-19, a depot's yard in the rain: "rain isn't showing up on top of walls
 *  properly"). The rain is hidden from a pixel whose surface is indoors, and "indoors" was the roof's bare rectangle — the building's outer walls grown by
 *  centimetres — so the streaks stopped dead at every outer wall's face, at the door in it and at the wall's top. The scene prelude's `const RAIN_WALLS`:
 *  the lattice rain and the particle rain ask whether the surface is UNDER the roof (the rectangle drawn in by 0.2 m, below the ceiling), the rule the snow
 *  has had since 10-15. On by default; host/pictureopens.ts lowers it at BOOT for ?norainwalls = the rain cut at the rectangle, as it was. Picture only. */
export const rainWallsConfig = { on: true };
/** THE BOUNCE NEVER READ THROUGH A WALL OFF A MAN (shaders/lighting_common.wgsl diceWalledPoint; the maintainer, 10-20, the pass at night, the squad in the snow
 *  against a lit room's outer wall: "i think there's some leak on direct and specular and indirect with the wall here"). The radiance cache is read a probe
 *  spacing along a face's normal, which is sound for a wall and wrong for a man standing by one: the read point is the room behind the wall. The scene prelude's
 *  `const BOUNCE_WALLS`: where a man (or a box he carries) is the face read for, or the surface a gather ray left is no wall, the read point is found by short box
 *  rays (stopped short of the nearest box along the normal, its footprint held to the cells it can see, no read where it sees none) — the gather, the gloss /
 *  pane / ray-traced reflections' hit shading, the cache's own probes and a coat's sheen. On by default; host/pictureopens.ts lowers it at BOOT for ?nobouncewalls = every
 *  read as it was. No game table: nothing in the update reads it. NOT mended by it: the light meter (shaders/probe.wgsl) reads the cache AT the man, blended
 *  over the cells around him, and by a lit room's wall some of those cells are the room's. Its cost was never timed on a GPU. */
export const bounceWallsConfig = { on: true };
/** WALLS HAVE TWO SIDES, AS ONE RULE FOR EVERY READ OF THE CACHE (render/wallSides.ts; shaders/common.wgsl sideAt, lighting_common.wgsl diceTaps and
 *  diceFacePointBy; the look page's row `walls`). The level's floor plan says which place a point is in, and a read of the radiance cache keeps to the place it is
 *  made for. With the prelude's `const WALL_SIDES` up that one rule stands IN PLACE OF the four older guards, each of which met one case of the same fault with
 *  means of its own: the plane of the surface a gather ray left (diceIrradianceFrom, 08-12), an occlusion ray for a point in smoke (diceMeanRadianceWalled, 08-09),
 *  five to eight box rays off a man (BOUNCE WALLS: diceWalledPoint, 10-20) and the cache's own nearest-cell read (LIGHT KEEPS TO ITS ROOM: hitRadiance under
 *  FLAG_RC_ROOM, 10-31). Their text is still in the modules and is dead code under the const.
 *  UP on every page: `?look=walls:0` lowers it at boot, and the four guards are back; a const of the shaders, so the row is not live. */
export const wallSidesConfig = { on: true };
/** THE LOOK PROTOTYPE (shaders/look.wgsl; the page /look, or ?look on the game's): tiling material maps made at boot (render/lookTex.ts) laid on the boxes by paint
 *  class, their relief lit by the screen-size direct light, weather that follows the building, small boxes' edges rounded. The g-buffer keeps the tilted normal with
 *  alpha 0 and gdown hands the working-size passes the box's own face back (common.wgsl lookGeoN). It is every page's picture, with no switch.
 *  `bounce` = how far the relief leans the bounced light (composite.wgsl); `pressed` = the panel's Pressed panels row, for the one reader of
 *  it outside the g-buffer (the reflection pass: how far a stuff's own `mirrors` stands in for a plain coat's, passes.ts setBeams); `air` = × the haze the frame block carries (engine.ts: the panel's Air row —
 *  the setting itself is the hour's to write). Never seen on a GPU. */
export const lookConfig = { kept: 0, pressed: 1, bounce: 0.7, air: 1, sky: 1, shade: 0.7, lamps: 1, exposure: null as (() => number | null) | null, puffs: 1, veil: 1, snow: null as (() => boolean) | null, eyes: 1, eyeAt: null as (() => readonly [number, number] | null) | null, doorGaps: null as (() => readonly (readonly [number, number, number, number])[]) | null, nightSky: null as (() => { zenith: ArrayLike<number>; horizon: ArrayLike<number> } | null) | null, fireSmoke: 1, longFoot: 1, walked: 1, fireSnow: 1, rooms: 1, steady: 1, roomBright: 1, order: 1, dark: 1, windows: 1, glare: 0.1, stir: 1, gradeExp: 1, gradeC: 1.35, gradeH: 0, gradeS: 0, gradeSat: 1 };   // (nightSky: the level's own sky as its file has it, a night's on every level but one that is its own day: the page's word, host/drawpage.ts, for render/lookexpo.ts lookNightSkyE while an hour is up · dark: the Eyes open in the dark row, how far past the night's exposure a place under the dim level is shown, × LOOK_EXPO.dark stops · stir: the Stirred smoke row, Game.smokeWake: a man moving through smoke is a body the smoke goes round, with a kick behind him at every stride, and the solver keeps the eddies he sheds turning, smoke.wgsl emBody; 0 = the two soft shoves as the build has them; glare: the Glare outside row, render/lookexpo.ts LOOK_EXPO.out; grade*: the grading rows as the tone map takes them, render/gbuffer.ts lookSync: × the exposure, the contrast, + the highlights' and the shadows' stops, × the saturation)
//   // (lamps: the panel's Real light values row, how far every source that is not the hour's own goes toward its real strength at that hour, render/photometry.ts; exposure: the page's word for the exposure of the hour that is up against the night's, host/drawpage.ts from rust/game_rules/src/hours, null = a night or nobody says; puffs: the Snow puffs row, × what a boot kicks into the powder's field, rust/game_rules/src/snow_fx bootKicks; veil: the Blowing snow row, × what the wind picks up into it, render/powderField.ts; snow: the page's word for whether snow lies on this level, host/drawpage.ts: no snow, no powder; fireSmoke: the Fire smoke row, rust/game_rules/src/fire FIRE_LOOK_SMOKE; eyes: the Eyes follow Sam row, how far the one exposure model's own eye goes to the place he stands in, render/lookexpo.ts; sky: × the sky's radiance, what every ray that leaves the level brings back: the panel's Sky light row)
/** A METAL NEVER MIRRORS THE ROOM BEHIND A WALL (the maintainer, 10-23, the solo night's manager's office: "some weird light leakage on bookshelf? seems like
 *  irradiance cache leak through wall? and showing up in specular?"). A ray-traced reflection shades its hit with the radiance cache read a probe spacing along
 *  the hit's normal. For a hit on a wall that is sound. For a hit on a shelf board, a desk top or the floor within half a cell of a partition the spacing runs
 *  ALONG the partition, the read's four plan cells straddle it, and up to half of what comes back is the lit room next door. A dielectric shows a few per cent of
 *  its mirror; a metal shows all of it (F0 = its paint), so the steel shelving on the office's wall glowed with the storage room's light in the dark. The scene
 *  prelude's `const REFL_WALLS`: a metal's reflection rays take lighting_common.wgsl's walled read (diceWalledPoint: stopped short of the nearest box along the
 *  normal, the footprint held to the cells the point can see, no read where it sees none) at every hit; everything else keeps the read it had. On by default;
 *  host/pictureopens.ts lowers it at BOOT for ?noreflwalls = every read as it was. No game table: nothing in the update reads it, and the light meter (shaders/probe.wgsl)
 *  does not read the reflections. NOT mended by it: the same blend in the DIFFUSE gather's hits (a third as strong by the look's indirect scale, and a metal
 *  keeps under a third of its diffuse), the glossy floors' and the panes' mirror on the presets with no traced reflections, and the meter's own read. Five to
 *  eight short box rays more for each ray of a metal pixel: never timed on a GPU. */
export const reflWallsConfig = { on: true };
/** SHARP DIRECT'S CAP (shaders/sharp_direct.wgsl, "never more than the lights that reach this cell send unshadowed"): the scene prelude's `const SHARP_CAP`.
 *  On by default; host/pictureopens.ts lowers it at BOOT for ?sharpcap=0 = a class takes the ratio its taps carry across with no bound, as it was (the white beads on a
 *  round edge near a walled-off strong light come back). No game table: nothing in the update reads it. */
export const sharpCapConfig = { on: true };
/** THE SUN'S GLINT (11-04; the maintainer, of a car's gloss paint beside two pictures of the real thing: "mm doesn't look good. we don't need to go full
 *  hyperrealistic but we should sell it more"). Every lamp's highlight on a smooth finish is added analytically (shaders/reflect.wgsl rtGlints), and the one
 *  light it left out was the brightest: a directional light. The sky a reflection ray comes back with (common.wgsl skyRadiance) is a gradient with no sun in
 *  it, so by day NOTHING smooth showed the sun: no streak along a car's wing, no glint on the lumps of a gloss-painted wall. The scene prelude's `const
 *  SUN_GLINT`: a directional light (the sun, and by night the moon) goes through rtGlints' loop as a fifth candidate with rules of its own, which that
 *  function's head says (outside the lamps' contest, never unshadowed, a ray to the world's top, faded in, bounded by its own light, a floor under its lobe). It
 *  follows the shading normal, so a material's relief catches it. Only where the reflection pass makes highlights at all: a finish up to the highlights' own fade
 *  (shaders/common.wgsl RT_HI_FADE, the same on ultra, high and medium; none on low). ON THE LOOK'S PAGE ONLY, on by default there; host/pictureopens.ts lowers it at BOOT for ?nosunglint = every
 *  picture as it was. Without the word the shader's flag is a constant false and its lines are the lines they were, with one empty turn of the shadow
 *  loop; no kernel gains a call site (a call is pasted whole where Windows compiles). No game table: nothing in the update reads a reflection and the light
 *  meter (shaders/probe.wgsl) does not either. COST on the look's page: one shadow ray to the world's top (and a smoke march, in smoke) on the pixels
 *  inside the sun's highlight. On a flat smooth floor that is all of it or none of it, by where the camera looks; a metal's highlight is wide (a steel
 *  floor passes from most bearings). Never timed, never seen on a GPU. */
export const sunGlintConfig = { on: true };
/** NO LAMP'S HIGHLIGHT IS CREDITED UNSEEN (the address's `&glintpick=1`, OFF unless asked; shaders/reflect.wgsl rtGlints' head says the
 *  rule). The reflection pass gave the four strongest lamps' highlights a shadow ray and added every other lamp's at half strength with no ray: the fifth
 *  lamp behind a wall glinted, and five in plain sight came out short. The scene prelude's `const GLINT_PICK`: the three strongest keep their rays and the
 *  fourth goes to one lamp picked from all the rest by its weight, drawn for the rest's whole weight. No ray more, no call site more; a pixel four lamps
 *  or fewer reach is the pixel it was. A const of the shaders, so host/drawpage.ts reads the word before the first pipeline. Picture only: nothing in the update
 *  reads a reflection. ITS END: on the maintainer's yes the word goes, `GLINT_PICK` with it, and the four slots, `rest` and its half are deleted from the
 *  three texts in the same commit. Never seen on a GPU. */
export const glintPickConfig = { on: false };
/** HOW FAR THE HIGHLIGHTS REACH (every page that traces reflections; shaders/common.wgsl rtHiFade, the scene prelude's `const RT_HI_REACH`). 1 = the lamps',
 *  the panels' and the sun's highlights are drawn on every finish up to 0.5 of roughness and fade out to 0.75, on every preset (RT_HI_FADE). 0 = they fade
 *  where the mirror does, at the preset's cut-off: the reach they had before 11-05, kept as DATA of the one path (the address's `&hireach=0`) so that the two
 *  pictures can be held side by side on one card. A const of the shaders, so host/drawpage.ts reads the word before the first pipeline. Picture only. */
export const highlightConfig = { reach: 1 };

/** the prelude's consts for the weather's volume: the switch, and where the air grid (scene/airGrid.ts) sits in boxMat's tail — after the marking and glass tables and BEFORE the cut
 *  table, so its place is the same with the cut switch on or off */
function weatherVolWgsl(): string {
  return `const WX_VOL: bool = ${weatherVolConfig.on ? 'true' : 'false'}; const WXD_BASE: u32 = MAX_BOXES * 4u + ${MARK_TABLE_ROWS + GB_ROWS}u; const WXD_W: i32 = ${AIR_W}; const WXD_H: i32 = ${AIR_H}; const WXD_CELL: f32 = ${AIR_CELL};   // the weather's air as a volume (render/frame.ts weatherVolConfig; scene/airGrid.ts)\n`;
}
/** ROUND THINGS IN THE CUT SET (scene/round.ts; common.wgsl ROUND_AS_CUT): a module compiled with the cut rule traces a round as the prism its rows of the cut table
 *  already hold, through the cut test. False in a module without the cut rule: there a round is the cylinder. */
function roundRayWgsl(cut: boolean): string {
  return `const ROUND_AS_CUT: bool = ${cut && cutConfig.on ? 'true' : 'false'};   // round things in a module with the cut rule: the picture's prism by isectCut (render/frame.ts roundRayWgsl) — false = the cylinder by isectRound wherever ROUND_BOXES\n`;
}
/** The text in front of every scene shader. `cut`: the CUT BOXES rule is compiled in (scene/cut.ts): wherever the switch is on, in EVERY module — one set of ray
 *  programs since 2026-11-07 (there was a second set with the rule, swapped in while a cut piece was alive). A tool may ask for the other text to size the rule. */
export function scenePrelude(cut: boolean = cutConfig.on): string {
  const commonText = `${Object.entries(LevelSw).map(([k, v]) => `const ${k}: u32 = ${v}u;`).join(' ')}   // what a level raises, as bits of frame.levelCfg.x: a shader asks lv(NAME) (render/frame.ts LevelSw; common.wgsl lv) — data, so every level is one text\nconst RAIN_SCALE: bool = ${rainScaleConfig.on ? 'true' : 'false'};   // a splash ring drawn wider than its line keeps its light, not its brightness (render/frame.ts rainScaleConfig): false = as bright however wide, as it was\nconst RAIN_WALLS: bool = ${rainWallsConfig.on ? 'true' : 'false'};   // rain shows in front of a wall's outer face and its top (render/frame.ts rainWallsConfig): false = hidden wherever the surface is inside a roof's rectangle, as it was\nconst LOOK_MIRROR_PLAIN: f32 = ${LOOK_MIRROR_PLAIN.toFixed(4)}; const LOOK_MIRRORS = array<f32, ${LOOK_WORDS}>(${LOOK_MIRRORS.map(v => v.toFixed(4)).join(', ')});   // what a coat mirrors head-on: a plain one, and each word a box may say (lookTex.ts STUFFS, a stuff's mirrors; common.wgsl boxStuff): the reflection pass's alone\nconst LOOK_BOUNCE: f32 = ${lookConfig.bounce.toFixed(3)};\nconst WALL_SIDES: bool = ${wallSidesConfig.on ? 'true' : 'false'};   // every read of the radiance cache keeps to its side of a wall by the floor plan, in place of the four older guards (render/frame.ts wallSidesConfig; shaders/lighting_common.wgsl diceTaps): false = each guard as it was\nconst BOUNCE_WALLS: bool = ${bounceWallsConfig.on ? 'true' : 'false'};   // the radiance cache is never read through a wall off a man (render/frame.ts bounceWallsConfig; shaders/lighting_common.wgsl diceWalledPoint): false = a probe spacing along his normal, as it was\nconst SHARP_CAP: bool = ${sharpCapConfig.on ? 'true' : 'false'};   // sharp direct bounds a class by its live lights' unshadowed sum (render/frame.ts sharpCapConfig; shaders/sharp_direct.wgsl): false = no bound, as it was\nconst REFL_WALLS: bool = ${reflWallsConfig.on ? 'true' : 'false'};   // a metal never mirrors the room behind a wall (render/frame.ts reflWallsConfig; shaders/reflect.wgsl): false = the cache a probe spacing along the hit's normal, as it was\nconst SUN_GLINT: bool = ${sunGlintConfig.on ? 'true' : 'false'};   // the sun's (and the moon's) highlight on a smooth finish (render/frame.ts sunGlintConfig; shaders/reflect.wgsl rtGlints): false = a directional light gives none, as it was\nconst GLINT_PICK: bool = ${glintPickConfig.on ? 'true' : 'false'};   // the lamps' highlights past the three strongest: one picked by its weight stands for them (render/frame.ts glintPickConfig; shaders/reflect.wgsl rtGlints): false = the fourth strongest and the rest at half, unshadowed, as it was\nconst RT_HI_REACH: f32 = ${highlightConfig.reach.toFixed(3)};   // how far the highlights reach: 1 = their own fade on every preset (common.wgsl RT_HI_FADE), 0 = where the mirror fades (render/frame.ts highlightConfig; common.wgsl rtHiFade)\nconst RAIN_SHADOW: bool = ${rainShadowConfig.on ? 'true' : 'false'};   // rain asks a shadow ray where a light's reach mask is not sure (render/frame.ts rainShadowConfig): false = lit by the mask alone, as it was\nconst WX_EDGE: bool = ${weatherEdgeConfig.on ? 'true' : 'false'};   // the weather's veil at the building's shell (render/frame.ts weatherEdgeConfig): false = composite.wgsl fogs by the surface point alone, as it always did\nconst ROUND_BOXES: bool = ${cutConfig.on ? 'true' : 'false'};   // round things (scene/round.ts): a box flagged GEO_ROUND is a cylinder to isectBox — false = that branch and roundNormalAt are dead code\nconst GLASS_BREAK: bool = ${glassBreakConfig.on ? 'true' : 'false'}; const GB_BASE: u32 = MAX_BOXES * 4u + ${MARK_TABLE_ROWS}u;   // glass breaking (render/frame.ts glassBreakConfig; game/glassbreak.ts): false = the cell-table reads in glassTrace / glassTransmit / composite are dead code; GB_BASE = the cell table's first vec4 row in boxMat (after the marking table: scene/markings.ts MARK_TABLE_ROWS)\n` + (cut ? `const CUT_BOXES: bool = true; const CUT_ROWS: u32 = ${CUT_ROWS}u; const CUT_STRIDE: u32 = ${CUT_STRIDE}u; const CUT_BASE: u32 = MAX_BOXES * 4u + ${MARK_TABLE_ROWS + GB_ROWS + AIR_ROWS}u;   // cut boxes (scene/cut.ts cutConfig): a box flagged GEO_CUT is clipped by the planes in its CUT_ROWS rows of boxMat from CUT_BASE (after the marking and glass tables and the weather's air grid) — isectBox, boxNormalFromCode, the g-buffer's cut pass\n` : `const CUT_BOXES: bool = false; const CUT_ROWS: u32 = ${CUT_ROWS}u; const CUT_STRIDE: u32 = ${CUT_STRIDE}u; const CUT_BASE: u32 = 0u;   // cut boxes off (scene/cut.ts cutConfig): the cut reads in isectBox / boxNormalFromCode are dead code\n`) + commonSrc + '\n' + smokeReadConsts(smokeGrid()) + smokeReadSrc;   // (+ every smoke read: the lattices' numbers as consts — smoke/smokeGrid.ts — then shaders/smoke_read.wgsl)
  const decl = reachBinding.storage
    ? '@group(0) @binding(12) var<storage, read> lightReachS: array<u32>;\nfn reachWord(w: u32) -> u32 { return lightReachS[w]; }\n'
    : '@group(0) @binding(12) var<uniform> lightReachU: array<vec4u, REACH_VEC4S>;\nfn reachWord(w: u32) -> u32 { return lightReachU[w >> 2u][w & 3u]; }\n';
  const mesh = meshBinding.enabled
    ? '@group(0) @binding(13) var<storage, read> meshData: array<vec4u>;   // mesh-shadow trees + skinned corners (render/meshShadows.ts layout; walked by meshShadowT under FLAG_MESH_SHADOWS)\nfn meshLoad(i: u32) -> vec4u { return meshData[i]; }\n'
    : 'fn meshLoad(i: u32) -> vec4u { return vec4u(0u); }   // no mesh-shadow binding on this adapter (frame.ts meshBinding): a zero header = an empty tree, and the engine never raises the flag\n';
  return wgslWorldConsts() + weatherVolWgsl() + roundRayWgsl(cut) + `const REACH_VEC4S: u32 = ${REACH_VEC4S}u;   // lightReach uniform: MAX_LIGHTS × REACH_ROW_WORDS u32 as vec4u (render/lights.ts)\nconst MESH_MAX_CHARS: u32 = ${MESH_MAX_CHARS}u;   // characters with a mesh-shadow tree (render/meshbvh.ts)\n` + commonText + '\n' + decl + mesh
    + sdfRigWgslConsts() + sdfRigSrc + '\n';   // the SDF character rig (render/sdfrig.ts's quantum as a const, then shaders/sdfrig.wgsl): the round-cone primitive common.wgsl's proxy loops call
}

export function createSceneLayout(device: GPUDevice): GPUBindGroupLayout {
  const all = GPUShaderStage.VERTEX | GPUShaderStage.FRAGMENT | GPUShaderStage.COMPUTE;
  const cf = GPUShaderStage.FRAGMENT | GPUShaderStage.COMPUTE;
  const entries: GPUBindGroupLayoutEntry[] = [
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
      { binding: 11, visibility: all, buffer: { type: 'uniform' } },   // the fog of war's sight table (render/fogofwar.ts FogSight; common.wgsl fogSeenAt reads it for a light with a fog bit, the picture's smoke — while FOG.air is off — and its particles) — a uniform: no sampled texture is left to a fragment stage
      { binding: 12, visibility: cf, buffer: { type: reachBinding.storage ? 'read-only-storage' : 'uniform' } },   // per-light reach masks (lights.ts reachBuf: REACH_BYTES = 17.5 KB as array<vec4u, REACH_VEC4S>). Uniform for the storage-buffer budget: the group binds 6 storage buffers (1 2 3 4 6 10); the cascade trace and the light probe add 2 each in group 1 = 8 in the compute stage = the WebGPU DEFAULT maxStorageBuffersPerShaderStage, so any core adapter runs it (engine.ts still checks ≥ 8 at boot with a readable message)
  ];
  if (meshBinding.enabled) entries.push({ binding: 13, visibility: cf, buffer: { type: 'read-only-storage' } });   // mesh-shadow trees (meshShadows.ts): the group's 8th storage buffer where the adapter affords 10 per stage (see meshBinding)
  entries.push({ binding: 14, visibility: cf, texture: { sampleType: 'uint', viewDimension: '3d' } });   // the smoke's page texture (a TEXTURE: the storage-buffer slots are all spoken for)
  return device.createBindGroupLayout({ label: 'sceneLayout', entries });
}

export interface SceneBuffers {
  frame: GPUBuffer; boxGeo: GPUBuffer; boxMat: GPUBuffer; gridCells: GPUBuffer; gridItems: GPUBuffer; sceneInfo: GPUBuffer; lights: GPUBuffer; smoke: GPUBuffer; smokeAtlasView: GPUTextureView; lightReach: GPUBuffer; linSamp: GPUSampler; capsules: GPUBuffer;
  /** the mesh-shadow buffer (MeshShadows.buf), or any ≥ 32-byte storage buffer of zeros while none is attached (a zero header is an empty tree); ignored when !meshBinding.enabled */
  mesh: GPUBuffer;
  /** the smoke's page texture (smoke/smoke.ts; engine.ts has a 1³ stand-in until a solver is attached) */
  smokePage: GPUTextureView;
  /** the fog of war's sight table (render/fogofwar.ts FogSight.buf) — or, for the group the table's own pass is bound with, a stand-in as large, all zeros */
  fogSight: GPUBuffer;
}
export function createSceneBindGroup(device: GPUDevice, layout: GPUBindGroupLayout, b: SceneBuffers): GPUBindGroup {
  const entries: GPUBindGroupEntry[] = [
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
      { binding: 11, resource: { buffer: b.fogSight } },
      { binding: 12, resource: { buffer: b.lightReach } },
  ];
  if (meshBinding.enabled) entries.push({ binding: 13, resource: { buffer: b.mesh } });
  entries.push({ binding: 14, resource: b.smokePage });
  return device.createBindGroup({ label: 'sceneBG', layout, entries });
}
