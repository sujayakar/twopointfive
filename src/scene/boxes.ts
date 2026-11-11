// Box scene representation, GPU packing and the 2D (XZ) uniform grid used by all ray queries.
import { Vec3, Quat, quat } from '../math/vec';
import { WORLD } from './world';
import { makeBuffer } from '../gpu/device';
import { CutPlane, CutGlass, cutConfig, cutClip, CUT_CLIP } from './cut';
import type { BoxShape } from './shapes';
import { BoxMat, Capsule } from '../draw/gpu';
import { Cap, Geo, Info } from '../draw/sections';
import type { MarkingDef } from './markings';

/** does a box of these flags stop a light of the sky (a mask of where such a light reaches is of those that do: render/reachKeep.ts) */
export const stopsSky = (flags: number): boolean => (flags & (BoxFlag.NoShadow | BoxFlag.Glass | BoxFlag.Dynamic)) === 0;
export const BoxFlag = {
  NoPrimary: 1,      // not rasterized into the G-buffer (ceilings, light-only geometry)
  NoShadow: 2,       // ignored by all rays (decals, glow cards)
  Cutaway: 4,        // rasterized only below mat.cutHeight (the viewer stage drops its back wall to a lip when the orbit camera goes behind it); rays see the full box
  Dynamic: 8,        // rebuilt every frame (character proxies, props on bones)
  Emissive: 16,      // has non-zero emission (lets shaders early-out)
  AreaLit: 32,       // its emission is represented by an analytic area light: rays ignore the emissive term (no double count, no small-emitter aliasing in the cascades)
  Rot: 64,           // carries a full 3D rotation (Box.rot; set automatically on upload): raster AND rays use the quaternion in boxMat instead of the packed yaw
  Gloss: 128,        // polished floor finish (lobby stone, the lino): under FrameFlags.Gloss the final gather traces one Fresnel-weighted reflection ray for on-screen pixels of its top face (fgather.wgsl glossReflection) — the last free bit of the GPU flag byte
  SoundSeal: 1 << 16,   // CPU-only (bits 8-15 of the GPU word are the owner byte, so CPU flags start at 1<<16): a dynamic occluder that currently seals sound (a door leaf near closed)
  // GLASS (experiment E3). The GPU flag word is a u32 whose low byte holds the flags above and whose second byte is the owner; bits 16-31 were simply never sent
  // (upload masked Box.flags to 0xff because the only higher bit, SoundSeal, is CPU-only). Rather than repurpose Cutaway (the viewer's stage wall needs it) the
  // word is WIDENED: Glass keeps the same bit on both sides (1<<17 here = BOX_GLASS 131072u in common.wgsl) and upload() passes exactly that bit through.
  // A glass box is authored geometry (levels/*.json flag name 'Glass', albedo = the pane's transmission tint) that no traced ray and no raster triangle ever
  // sees: upload() keeps it out of the trace grid AND the visible list and instead lists it in SceneInfo.glass (≤ MAX_GLASS panes, 3 vec4f each), where
  // composite / fgather / direct test view and shadow segments against it analytically (Fresnel reflection + tint + dust glow; common.wgsl glassTrace).
  // CPU: movement collision and sound treat it as a wall, sight passes through it (rust/game_rules/src/world/scene_collision.rs segment_blocked's `through`), a round / kick / bang breaks it.
  Glass: 1 << 17,
  // A THIN THING THAT STANDS FREE (as a level is stood up: a stovepipe, a window's bar, a lone post). CPU-only, like SoundSeal (bits
  // 18-31 of the GPU word are the marking row): the fog of war's own sight passes it while FOG.thin is on (rust/game_rules/src/stealth/fogofwar: segmentBlocked's `through`; a teammate's own eye too under FOG.mates — rust/game_rules/src/people/guards eye_through), and
  // the picture's fog eye learns which boxes carry it from the sight table's own bit list (render/fogofwar.ts FogFrame.thinBits), not from the box's word.
  Thin: 1 << 18,
  // A WINDOW'S BAR (as a level is stood up: the kit's upright between a window's bays). CPU-only, like Thin: a round that meets one
  // breaks it and a line of fire passes one while WINBARS.on is up (rust/game_rules/src/breakage/winbars.rs; rust/game_rules/src/world/scene_collision.rs shot_blocked's `firePass`); read by nothing else.
  Bar: 1 << 19,
} as const;

import { coatRough, SOLIDGLASS } from './stuff';   // (stuff.ts reaches only the look's table of names, a module with no imports of its own: no cycle)
export type BoxStuff = import('./stuff').BoxStuff;
export type DoesWord = import('./materials').DoesWord;
/** WHAT A PLANNER OF THE PICTURE ASKS OF A BOX: where it is, its size and turn, and what kind of thing it is (of its cuts, only whether it has any). A
 *  planner that is given more than one frame (render/steps.ts) works on copies of these, taken in the call that begins it (planBoxes), so the level it
 *  plans is one level, as it stood then. The type is the list of what may be asked: a planner that reads another field of a box does not compile until
 *  the copy carries it. Nothing of a copy is shared with the box it was taken from. */
export interface PlanBox { c: Vec3; h: Vec3; yaw: number; flags: number; rot?: Quat; cut?: { readonly length: number }; round?: boolean }
export function planBoxes(list: readonly Box[]): PlanBox[] {
  const out: PlanBox[] = new Array(list.length);
  for (let i = 0; i < list.length; i++) { const b = list[i], r = b.rot;
    out[i] = { c: [b.c[0], b.c[1], b.c[2]], h: [b.h[0], b.h[1], b.h[2]], yaw: b.yaw, flags: b.flags, rot: r ? [r[0], r[1], r[2], r[3]] : undefined, cut: b.cut !== undefined && b.cut.length > 0 ? { length: b.cut.length } : undefined, round: b.round }; }
  return out;
}
export interface Box {
  c: Vec3;            // center
  h: Vec3;            // half extents (local axes)
  yaw: number;        // rotation about +Y (radians)
  albedo: Vec3;       // linear diffuse reflectance
  emissive: Vec3;     // emitted radiance (linear, HDR)
  flags: number;
  owner: number;      // 0 = world, 1..255 = character id (rays from a character skip its own proxies)
  cutHeight: number;  // world Y above which the box is hidden from primary visibility when Cutaway
  /** RENDER-ONLY albedo factor (absent = 1): what the renderer packs is albedo × wetK (sentAlbedo below; for a thing of solid glass, glass's dark × wetK), while `albedo` itself stays as authored (no rule reads a material from it: what a
   *  box does is its word, scene/materials.ts, in any weather). weather.ts darkens rained-on ground with it. */
  wetK?: number;
  /** optional full 3D orientation (unit quaternion, world) for the few things that tilt: hand props, goggles, spark streaks, tumbling
   *  canisters. Raster and rays both honour it (BoxFlag.Rot is set on upload; the grid footprint and CPU picking use it too); `yaw`
   *  should still hold the heading for CPU code that only cares about that. Absent = yaw about +Y as always. */
  rot?: Quat;
  /** DRAWN ELSEWHERE (rust/game_rules/src/outfit/stature.rs — a man drawn a few per cent taller or shorter takes the kit bolted to him along): the centre and half extents the GPU is handed for this box —
   *  its geo words and its trace-grid registration — in place of c / h. The simulation's box (CPU ray casts, picking, everything that reads c / h) stays where it is. Dynamics only;
   *  absent = c / h, as ever. */
  dc?: Vec3; dh?: Vec3;
  name?: string;      // optional tag (light fixtures etc.)
  /** The surface FINISH (renderer v3 phase 4 — the one BRDF: base colour = albedo, GGX roughness 0‥1, metalness): absent rough = ROUGH_DEFAULT, matte — no reflection ray is
   *  ever spent on the box — and `metal` = a bare conductor (F0 = its albedo, most of its diffuse gone) rather than a dielectric coat (F0 0.04 over the paint). Authored per
   *  material / per box in the level file (levelData.ts MaterialData.rough / metal, BoxData.rough / metal); the three below reach the GPU as ONE float in boxMat row 1's w
   *  (matWord, decoded by common.wgsl boxFinish) — the g-buffer writes roughness / metalness per pixel from it (+ the detail layer), reflect.wgsl reads the anisotropy. */
  rough?: number; metal?: boolean;
  /** WHAT THIS BOX IS MADE OF (scene/stuff.ts): a material of the look, 'waits', 'glass' or 'rubber'; absent = plain. It is SAID, never read off the
   *  colour: by the level file (the box's own `stuff`, else its material row's: levelLoad.ts puts it here when the file is read) or by the code that makes
   *  the box (a kit's add() from the palette row it was handed, a prop's part, a door's leaf). The look draws exactly this (render/gbuffer.ts lookWordOf),
   *  and the PICTURE of a round's hit reads it (rust/game_rules/src/breakage/particle_fx.rs HITSTUFF: the sparks and chips it knocks out, the crack in solid glass); nothing in the
   *  simulation does. It is part of the box's PAINT: whoever repaints the box says the new paint's word or drops this one, and whoever copies a box's
   *  paint copies it. */
  stuff?: BoxStuff;
  /** WHAT THIS BOX DOES (scene/materials.ts): the row of the table of materials whose `does` it takes: what it is to a round, a charge and a blast.
   *  Absent = the default row. GAMEPLAY, where `stuff` is dressing, and never worked out from it or from the colour: written here once by whoever makes
   *  the box (the loader from the level file, a kit from its paint, the game from the box a piece came off: wordOf below), and read by the rules through
   *  materials.ts doesOf and nothing else. Whoever REPAINTS a box leaves it alone: a thing in other clothes is the same thing. */
  does?: DoesWord;
  /** anisotropic finish — the lobe stretched along the view direction so a lamp mirrors as a long streak, not a disc: the wet lot's standing water
   *  (rust/game_rules/src/air/weather.rs sets it with the rain; runtime only, never authored) */
  aniso?: boolean;
  /** MARKINGS (scene/markings.ts): up to MAX_MARKS_PER_BOX pixel-art signifiers ON this box's faces — key caps, packing tape, drawer seams, the vending
   *  window, text bars, LEDs, glyphs — authored in the level file (BoxData.markings / a prop part's) or by the code that makes the box (door leaves). The
   *  upload registers the list in the marking table (the tail of matBuf) and names its first record in the top 14 bits of the GPU flags word; the g-buffer,
   *  composite and the reflection pass evaluate them; the secondary rays get their area average folded into `boxAlbedoSec` (the 4th mat row). */
  marks?: MarkingDef[];
  /** × the emission of this box's emissive markings (runtime: rust/game_rules/src/air/lighting.rs writes a switchable's supply factor here — a blackout, the OCP, a flicker —
   *  next to the box's own emissive; absent = 1) */
  markGain?: number;
  /** THE PICTURE'S EMISSION, where it is not the box's own (render-only: rust/game_rules/src/stealth/fogofwar FOG.kept — a lamp's face under the fog of war as the team last saw
   *  it, lit or dark): the upload takes these in place of `emissive` / `markGain`; nothing in the game reads them, and `emissive` itself stays the lamp's real
   *  state. Absent = the box's own. Read only: several boxes may be handed one array */
  fogEmit?: Readonly<Vec3>; fogMark?: number;
  /** MOTION IDENTITY (renderer v3, image-quality program E1): a key that names THIS part of THIS object across frames — a door's leaf, a chair's seat, a flying
   *  can's band, the torch in a man's hand (MKEY ranges below; the code that emits the box sets it). With it the g-buffer's motion vector for the box comes from
   *  the transform the SAME part had a frame ago wherever it sat in the dynamic list; without it (sparks, tracers, one-frame effects) the old rule stands: the
   *  box at the same INDEX a frame ago, accepted only if its half-extents and owner match. Statics never need one. */
  mkey?: number;
  /** THE CUT RECORD (scene/cut.ts — read its head): up to MAX_CUT_PLANES planes in this box's LOCAL frame, each keeping the side n · p ≤ d, that clip the box into a
   *  wedge, a shard, a snapped end. Under the switch (cutConfig.on) the raster draws the cut solid and every ray — GPU and CPU — meets it; what pushes against the box
   *  (collision, nav, debris contact) still meets the uncut box. Off, or absent / empty: the plain box. Build it with cut.ts cutRecord / cutPrism (normals unit, no
   *  redundant planes) and REPLACE the array to change a static's cut (the pack shadow compares its identity, as it does `marks`). */
  cut?: CutPlane[];
  /** the CORE colour: what this box's CUT faces show in the picture (white gypsum under paint, pale wood under varnish), matte. Absent = the box's own colour and
   *  finish. Read only with a record under the switch, and packed WITH the record: on a static, replace `cut` when you change it. The box's own six faces, and every
   *  ray that lands on a cut face (bounce, reflections), keep the box's own colour. */
  cutCore?: Vec3;
  /** GLASS (cut.ts CutGlass): this cut box's two z faces are drawn see-through inside a rim in the core colour. Picture only; packed with the record, like cutCore. */
  cutGlass?: CutGlass;
  /** ROUND (scene/shapes.ts): this cut box is a prism standing for a ROUND thing about its own x (1), y (2) or z (3). Picture only, packed with the
   *  record like cutCore: the g-buffer's cut pass shades the faces round that axis with one turning normal and lets marks and the look's paint run over them. Every
   *  ray still meets the prism's flat faces. Not with cutGlass. */
  cutRound?: 1 | 2 | 3;
  /** AN ARC (scene/shapes.ts): with cutRound about x or z, the round's axis lies this many metres UNDER the box's centre (its own −y), so a box cut to a
   *  few facets of a wide circle is shaded as that circle: a bowed roof, a fender. Picture only, packed with the record (which must leave its last row unused: seven
   *  planes at most). Absent or 0 = the round's axis is the box's own middle. */
  cutArc?: number;
  /** AN OLD BREAK (the same trial): the cut faces were not made a moment ago. Picture only, packed with the record: on the look page they take the look's paint
   *  as a face of the core's stuff, and on a snow level the snow lies on the ones that face up. A blast's fresh cut never sets it. Not with cutGlass. */
  cutOld?: boolean;
  /** WHAT AN OLD BREAK'S CORE IS MADE OF (with cutOld; as `stuff`): where the cut faces show another thing than the box is (a wall broken under its coping
   *  shows the coping's stuff). Absent = the box's own `stuff`. Picture only, packed with the record on the look page. */
  cutCoreStuff?: BoxStuff;
  /** ROUND (scene/round.ts — read its head): this box is a ROUND THING, the cylinder about its own y of radius h[0] (= h[2]) and half height h[1]. Every CPU ray
   *  and segment (rayBox, rust/game_rules/src/world/scene_collision.rs line_box) and what walks against it (collideCircle) meet the cylinder; where cut pieces are on so do the picture (the cut pass)
   *  and every GPU ray. Said by the level file (BoxData.round); never with a cut record. */
  round?: boolean;
  /** A SHAPE (scene/shapes.ts — read its head): how this box of a level stands when shapes are on, as its level file says it (BoxData.shape) or as the kit
   *  that made it said it. Authored data: nothing but shapes.ts shapeLevel reads it, as the level is stood up. */
  shape?: BoxShape;
  /** shapes.ts shapeLevel has stood this box as its shape (its record, if its shape has a form, is the shape's and no break's): taken back when the level is
   *  stood up with the switch down */
  shaped?: boolean;
  /** this static is a PIECE of a shape (BoxShape.with): added after the level's own boxes by shapeLevel, taken away by it; never in a level file's list.
   *  TRIM: drawn and nothing else (it stands NoShadow, so no ray, eye or body meets it); `of` = the number of the box that says it, which it follows out of
   *  the picture */
  piece?: boolean;
  of?: number;
  /** DRESSING (CPU only): drawn and lit like any box, and PICTURE ONLY for the CPU's world ray, exactly as a look piece is (BoxWorld.raycast) — a cursor, an
   *  order, a laser and a tripwire pass it. The debris pool's chunks carry it while the pool is dressing (rust/game_rules/src/breakage/debris, DEBRIS.dress). */
  dress?: boolean;
}

/** ONE LAYOUT FOR EVERY BOX: a box is made with every field of the interface in place, in one order (the optional ones `undefined`), so the engines keep ONE hidden class
 *  for all of them and every read of a box in a hot loop (the line tests, the rays, the pack, the fog's list) is a plain field load. A field set later (`mkey`, `dc`,
 *  `marks` …) is then a write to a slot that is there, never a new layout. Make a box through this or through makeBox; a literal elsewhere forks the layout again. */
export function boxOf(c: Vec3, h: Vec3, yaw: number, albedo: Vec3, emissive: Vec3, flags: number, owner: number, rot?: Quat, name?: string): Box {
  return { yaw, albedo, emissive, flags, owner, cutHeight: 1e6, c, h, rot, name, mkey: undefined, dc: undefined, dh: undefined, rough: undefined, metal: undefined, aniso: undefined,
    marks: undefined, markGain: undefined, fogEmit: undefined, fogMark: undefined, wetK: undefined, cut: undefined, cutCore: undefined, cutGlass: undefined, cutRound: undefined, cutArc: undefined, cutOld: undefined, round: undefined, shape: undefined, shaped: undefined, piece: undefined, of: undefined, dress: undefined, stuff: undefined, does: undefined, cutCoreStuff: undefined,
  } satisfies { [K in keyof Required<Box>]: Box[K] };   // (a field added to Box and not here does not typecheck)
}
export function makeBox(p: Partial<Box> & { c: Vec3; h: Vec3 }): Box {
  const b = boxOf(p.c, p.h, 0, [0.5, 0.5, 0.5], [0, 0, 0], 0, 0);
  for (const k in p) (b as unknown as Record<string, unknown>)[k] = (p as unknown as Record<string, unknown>)[k];   // (the spread's own rule: every key the caller gave, as given)
  return b;
}

/** WHOEVER COPIES A BOX'S PAINT COPIES ITS WORDS (Box.stuff and Box.does: a piece broken off a thing is made of what the thing is, to the eye and to the
 *  next round). Spread beside the albedo of the box being made: `makeBox({ …, albedo: v3.copy(b.albedo), ...wordOf(b) })`. */
export const wordOf = (b: { stuff?: BoxStuff; does?: DoesWord }): { stuff?: BoxStuff; does?: DoesWord } =>
  b.stuff !== undefined ? (b.does !== undefined ? { stuff: b.stuff, does: b.does } : { stuff: b.stuff }) : b.does !== undefined ? { does: b.does } : {};

/** roughness a box without a finish is taken to have: fully matte — over every preset's rtMaxRough, so the reflection pass skips it outright */
export const ROUGH_DEFAULT = 1.0;
/** a thing of solid glass: it says 'glass' and is no pane (a pane, BoxFlag.Glass, is the glass engine's whatever it says). The one test: what cracks
 *  as solid glass (rust/game_rules/src/breakage/particle_fx.rs solid_glass) and what is drawn as it (below) ask it. */
export const solidGlassBox = (b: { stuff?: BoxStuff; flags: number }): boolean => b.stuff === 'glass' && (b.flags & BoxFlag.Glass) === 0;
/** the neutral dark a thing of solid glass is sent as in place of its own colour (scene/stuff.ts SOLIDGLASS), or −1 where it is sent as it is: the rule
 *  down, no solid glass, a face that carries markings, a colour that dark already */
export function glassDark(b: Box): number {
  if (!SOLIDGLASS.on || b.marks !== undefined || !solidGlassBox(b)) return -1;
  const a = b.albedo; return 0.2126 * a[0] + 0.7152 * a[1] + 0.0722 * a[2] > SOLIDGLASS.lum ? SOLIDGLASS.lum : -1;
}
/** The finish as the one f32 the GPU gets (boxMat[3i+1].w): roughness clamped to 0‥1, + 2 if metal, + 4 if anisotropic — three plain floats' worth in
 *  disjoint ranges, so common.wgsl boxFinish() peels them off with two compares (no bit tricks on a float the driver may renormalise). */
export function matWord(b: Box): number {
  const own = b.rough ?? ROUGH_DEFAULT, coat = coatRough(b.stuff) ?? (SOLIDGLASS.on && solidGlassBox(b) ? SOLIDGLASS.rough : undefined), r = coat !== undefined && coat < own ? coat : own;   // (a box that says a named coat is no rougher than the coat, and a thing of solid glass no rougher than glass: scene/stuff.ts COATFINISH, SOLIDGLASS)
  return (r < 0 ? 0 : r > 1 ? 1 : r) + (b.metal ? 2 : 0) + (b.aniso ? 4 : 0);
}


/** THE BOX WORLD'S TABLES ON THE DEVICE, and what the picture's own plans ask of a static. The rows are the module's (rust/game_draw boxes): draw/world.ts hands them
 *  over as they lie and packs nothing. What is the page's own here is the men's proxies, which are fitted to the rig. */
export class BoxWorld {
  /** the statics and the doors' leaves, as a plan reads them (PlanBox) */
  statics: Box[] = [];
  dynamics: Box[] = [];
  /** how many boxes were uploaded this frame (none here: a tool's world is empty, the page's is draw/world.ts) */
  get count(): number { return 0; }
  readonly maxBoxes = WORLD.maxBoxes;
  geoBuf: GPUBuffer; matBuf: GPUBuffer;
  // grid: cells hold two regions in ONE buffer: [0, W*H] the plain per-cell start offsets into `items` (what the flag-off tracer and the smoke obstacle bake read), then from
  // GRID_CB = W*H+1 two words per cell for the fast traversal (common.wgsl): heightMask24 | cheby<<24 and start24 | min(count,255)<<24. `items` likewise: [0, itemCount) plain
  // box indices, [itemCount, 2*itemCount) index | qmin<<16 | qmax<<24.
  cellBuf: GPUBuffer; itemBuf: GPUBuffer;
  /** the registration slack handed to the tracer (metres a traced box may reach past the cells it is registered in: rust/game_draw boxes/grid.rs) */
  slack = 0;
  /** EVERY MAN DRAWN HAS HIS PROXIES: the list holds as many characters as the skinning draws (anim/characters.ts MAX_CHARS). The mesh trees still hold MESH_MAX_CHARS
   *  (render/meshbvh.ts): past them a man is his round cones to every ray (common.wgsl meshShadowT / meshClosest). */
  static readonly MAX_CAST = 16;
  /** …and how many of them a frame's list takes: all of them, or (?allproxies=0 in the page, host/drawpage.ts) the eight it took before. The list's own room (MAX_CAST, the
   *  info block's layout) does not move with it. */
  static castCap = BoxWorld.MAX_CAST;
  static readonly CAPS_PER_CHAR = 15; static readonly MAX_CAPSULES = BoxWorld.MAX_CAST * BoxWorld.CAPS_PER_CHAR;
  /** the scene's block (struct SceneInfo): the module's but for the men's proxies */
  infoBuf: GPUBuffer; infoData = new Uint32Array(Info.row); infoF32 = new Float32Array(this.infoData.buffer);
  /** the panes uploaded this frame, in SceneInfo order (indices into `statics`) */
  glassList: number[] = [];
  /** static box indices carrying BoxFlag.Glass (intact or broken) */
  glassStatics: number[] = [];
  // character proxies (see struct Capsule in common.wgsl — traced as ROUND CONES; the record and these members keep their old name): 8 floats each
  capsuleData = new Float32Array(BoxWorld.MAX_CAPSULES * Capsule.row); capsuleU32 = new Uint32Array(this.capsuleData.buffer); capsuleCount = 0; charCount = 0;
  capsuleBuf: GPUBuffer;
  /** The SDF CHARACTER RIG's fit (render/sdfrig.ts; the engine writes it from its attached fit each frame before upload): with it set, upload() turns the frame's list into the rig —
   *  slot i's radius becomes the fitted r0[i], the fitted r1[i] rides in the top byte of the record's misc word (millimetres; RB_QUANT), and each character's cull sphere is fitted to
   *  his cones (+ kPad for the smooth union's bulge). null = no fit attached: the list exactly as addCapsuleRows wrote it (no r1 byte — sdfrig.wgsl coneRb then
   *  reads r again, an untapered cone). */
  rig: { r0: Float32Array; r1: Float32Array; kPad: number; byOwner?: Map<number, { r0: Float32Array; r1: Float32Array; pad?: number }> } | null = null;
  static readonly RB_QUANT = 0.001;   // = render/sdfrig.ts RB_QUANT (shaders/sdfrig.wgsl coneRb decodes it)
  /** the boxes drawn this frame, the plain ones first and then the cut; of the cut, the first `visibleCutTraced` a ray can meet (the picture-only ones behind them are glass plates) */
  visibleCount = 0; visibleCutCount = 0; visibleCutTraced = 0; visibleBuf: GPUBuffer;
  /** how many boxes of the last upload carry a cut record AND can be met by a ray (not NoShadow: a glass plate is picture only) */
  cutTraced = 0;
  /** a count of the statics' changes: what was read off them stands while it does. Every row that is told, whatever changed it (a load, a flag, a lamp's face), where
   *  `geomSerial` counts what a blast does to the level's shape alone */
  staticsTold = 0;
  geomSerial = 0;
  /** CUT BOXES (scene/cut.ts): latched at construction (the shaders were compiled from the same value) */
  readonly cutOn = cutConfig.on;

  constructor(protected device: GPUDevice) {
    const n = this.maxBoxes, S = GPUBufferUsage.STORAGE | GPUBufferUsage.COPY_DST, cells = WORLD.gridW * WORLD.gridH;
    this.geoBuf = makeBuffer(device, n * Geo.row * 4, S | GPUBufferUsage.COPY_SRC, 'boxGeo');   // (COPY_SRC: the g-buffer snapshots both into its 'previous frame' buffer for the motion vectors — render/gbuffer.ts snapshotPrev)
    this.matBuf = makeBuffer(device, (this.cutOn ? BoxMat.rows : BoxMat.cut) * 16, S | GPUBufferUsage.COPY_SRC, 'boxMat');   // the material rows, and the tables behind them (a page that cuts nothing has no cut planes)
    if (WORLD.gridW + WORLD.gridH > 96) throw new Error('trace grid: the DDA in common.wgsl caps a ray at 96 cell visits; W + H must stay ≤ 96 for that cap to be unreachable');
    if (n > 65536 || n * cells >= 1 << 24) throw new Error('trace grid: packed items keep the box index in 16 bits and cell starts in 24');
    this.cellBuf = makeBuffer(device, (cells + 1 + 2 * cells) * 4, S, 'gridCells');
    this.itemBuf = makeBuffer(device, 65536 * 4, S, 'gridItems');
    this.infoBuf = makeBuffer(device, this.infoData.byteLength, GPUBufferUsage.UNIFORM | GPUBufferUsage.COPY_DST, 'sceneInfo');
    this.capsuleBuf = makeBuffer(device, this.capsuleData.byteLength, S, 'capsules');
    this.visibleBuf = makeBuffer(device, n * 4, S, 'visibleBoxes');
  }
  /** the frame's rows handed to the device */
  upload(): void {}

  /** Begin a frame's proxy list. Characters append CAPS_PER_CHAR records each. */
  resetCapsules() { this.capsuleCount = 0; this.charCount = 0; }
  /** a character's records as rows of words (src/draw: an end and its radius, the other end, whose they are) from `at`, in one colour or one a record; his bound at `b` of `fb` */
  addCapsuleRows(f: Float32Array, u: Uint32Array, at: number, owner: number, colours: readonly Vec3[], fb: Float32Array, b: number): void {
    if (this.charCount >= BoxWorld.castCap) return;
    const N = BoxWorld.CAPS_PER_CHAR, base = this.charCount * N, D = this.capsuleData;
    for (let i = 0; i < N; i++) { const o = (base + i) * Capsule.row, s = at + i * Cap.row, c = colours[i] ?? colours[0];
      for (let k = 0; k < 3; k++) { D[o + Capsule.a + k] = f[s + Cap.a + k]; D[o + Capsule.b + k] = f[s + Cap.b + k]; } D[o + Capsule.r] = f[s + Cap.r];
      const r = Math.round(Math.min(1, Math.max(0, c[0])) * 31) & 31, g = Math.round(Math.min(1, Math.max(0, c[1])) * 63) & 63, l = Math.round(Math.min(1, Math.max(0, c[2])) * 31) & 31;
      this.capsuleU32[o + Capsule.misc] = ((owner & 255) | ((r << 11 | g << 5 | l) << 8)) >>> 0; }
    const bo = Info.charBounds + this.charCount * 4; for (let k = 0; k < 4; k++) this.infoF32[bo + k] = fb[b + k];
    this.charCount++; this.capsuleCount = this.charCount * N;
  }
  /** Turn the frame's proxy list into the SDF rig (see `rig`): per real record (radius > 0) the fitted r0 as its radius and r1 packed above the albedo; per character the cull sphere
   *  fitted to the cones' ends + radii (+ the smooth union's pad + 1 cm) about the middle of their box. Idempotent — a render with no game step re-applies it to the same records and
   *  gets the same words (the ends are never touched, r0 / r1 come from the fit, the sphere is rebuilt from both). */
  protected applyRig(base: { r0: Float32Array; r1: Float32Array; kPad: number; byOwner?: Map<number, { r0: Float32Array; r1: Float32Array; pad?: number }> }) {
    const D = this.capsuleData, U = this.capsuleU32, N = BoxWorld.CAPS_PER_CHAR;
    for (let c = 0; c < this.charCount; c++) {
      const rig = base.byOwner?.get(U[c * N * Capsule.row + Capsule.misc] & 0xff) ?? base;   // (the cast: a man's own fit by his id — the owner byte every one of his records carries — else the shared one)
      let x0 = 1e9, y0 = 1e9, z0 = 1e9, x1 = -1e9, y1 = -1e9, z1 = -1e9;
      for (let i = 0; i < N; i++) {
        const o = (c * N + i) * Capsule.row; if (!(D[o + Capsule.r] > 0)) continue;   // (a padding record: nothing to fit, never hit)
        const r0 = rig.r0[i], r1 = rig.r1[i];
        D[o + Capsule.r] = r0; U[o + Capsule.misc] = ((U[o + Capsule.misc] & 0x00ffffff) | (Math.max(1, Math.min(255, Math.round(r1 / BoxWorld.RB_QUANT))) << 24)) >>> 0;
        const r = Math.max(r0, r1);
        x0 = Math.min(x0, D[o + Capsule.a] - r, D[o + Capsule.b] - r); y0 = Math.min(y0, D[o + Capsule.a + 1] - r, D[o + Capsule.b + 1] - r); z0 = Math.min(z0, D[o + Capsule.a + 2] - r, D[o + Capsule.b + 2] - r);
        x1 = Math.max(x1, D[o + Capsule.a] + r, D[o + Capsule.b] + r); y1 = Math.max(y1, D[o + Capsule.a + 1] + r, D[o + Capsule.b + 1] + r); z1 = Math.max(z1, D[o + Capsule.a + 2] + r, D[o + Capsule.b + 2] + r);
      }
      if (x0 > x1) continue;
      const cx = (x0 + x1) / 2, cy = (y0 + y1) / 2, cz = (z0 + z1) / 2; let rad = 0;
      for (let i = 0; i < N; i++) {
        const o = (c * N + i) * Capsule.row; if (!(D[o + Capsule.r] > 0)) continue;
        rad = Math.max(rad, Math.hypot(D[o + Capsule.a] - cx, D[o + Capsule.a + 1] - cy, D[o + Capsule.a + 2] - cz) + D[o + Capsule.r], Math.hypot(D[o + Capsule.b] - cx, D[o + Capsule.b + 1] - cy, D[o + Capsule.b + 2] - cz) + rig.r1[i]);
      }
      const bo = Info.charBounds + c * 4; this.infoF32[bo] = cx; this.infoF32[bo + 1] = cy; this.infoF32[bo + 2] = cz; this.infoF32[bo + 3] = rad + base.kPad + 0.01 + ((rig as { pad?: number }).pad ?? 0);   // (pad: a cast outfit's reach outside its cones — sdfrig.ts RigFit.pad; nothing on the mannequin)
    }
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
}

/** IS THIS BOX'S CUT RECORD MET by the game's rays and segments? Under the cut-pieces switch every record is (scene/cut.ts); a shape's always: a shape is
 *  the simulation's, and a picture setting may not move it (as a round thing, scene/round.ts). Ask it only of a box that has a record. */
export const cutMet = (b: Box): boolean => (cutConfig.on || b.shaped === true) && b.cut!.length > 0;
export function rayBox(ro: Vec3, rd: Vec3, b: Box, tmax: number): { t: number; n: Vec3 } | null {
  return rayBoxOwn(ro, rd, b, tmax);
}
/** The test itself, with no crossing: for the box world's own raycast (BoxWorld.raycast is the host's and walks the host's list, a thousand boxes a ray:
 *  asked of the Rust game once a box, one ray from outside a tick cost most of a second). */
function rayBoxOwn(ro: Vec3, rd: Vec3, b: Box, tmax: number): { t: number; n: Vec3 } | null {
  if (b.round === true) return rayBoxRound(ro, rd, b, tmax);   // ROUND THINGS (scene/round.ts): the cylinder (below)
  if (b.cut !== undefined && cutMet(b)) return rayBoxCut(ro, rd, b, tmax);   // CUT BOXES: the cut solid (below) — every other box takes the lines it always took
  if (b.rot) return rayBoxRot(ro, rd, b, tmax);
  // (the CPU-frame lane: the slab loop written out over the three axes and scalars — the same statements in the same order as the array form it replaces, so t and n are the same bits; the two
  //  scratch arrays a test allocated are gone. rayBox is the game's hottest CPU function: ~1000 calls a frame from the raycasts alone.)
  const cs = Math.cos(b.yaw), sn = Math.sin(b.yaw);
  const px = ro[0] - b.c[0], py = ro[1] - b.c[1], pz = ro[2] - b.c[2];
  // world -> local (R^T)
  const lx = cs * px - sn * pz, lz = sn * px + cs * pz, ly = py;
  const dx = cs * rd[0] - sn * rd[2], dz = sn * rd[0] + cs * rd[2], dy = rd[1];
  const ix = 1 / (dx || 1e-20), iy = 1 / (dy || 1e-20), iz = 1 / (dz || 1e-20);
  const h = b.h; let t0 = 0, t1 = tmax; let axis = -1, sgn = 0;
  { let ta = (-h[0] - lx) * ix, tb = (h[0] - lx) * ix; let s = -1; if (ta > tb) { const tmp = ta; ta = tb; tb = tmp; s = 1; } if (ta > t0) { t0 = ta; axis = 0; sgn = s; } if (tb < t1) t1 = tb; if (t0 > t1) return null; }
  { let ta = (-h[1] - ly) * iy, tb = (h[1] - ly) * iy; let s = -1; if (ta > tb) { const tmp = ta; ta = tb; tb = tmp; s = 1; } if (ta > t0) { t0 = ta; axis = 1; sgn = s; } if (tb < t1) t1 = tb; if (t0 > t1) return null; }
  { let ta = (-h[2] - lz) * iz, tb = (h[2] - lz) * iz; let s = -1; if (ta > tb) { const tmp = ta; ta = tb; tb = tmp; s = 1; } if (ta > t0) { t0 = ta; axis = 2; sgn = s; } if (tb < t1) t1 = tb; if (t0 > t1) return null; }
  // Origin INSIDE the box (no face entered ahead of it — an origin exactly ON a face heading in counts as inside too): no hit, deliberately. Callers lean on it: the
  // cursor's pick and a round's static hit start rays at muzzles and lenses that may sit in their own housing, an item or a spark born inside a static leaves it
  // freely, an eye momentarily inside a swinging leaf still sees (rust/game_rules/src/world/scene_collision.rs segment_blocked_dynamic). What must NOT lean on it is a blocked-or-not question about a
  // segment born on a wall's face — rust/game_rules/src/world/scene_collision.rs segment_blocked judges statics by the full slab interval (lineBox) for exactly that reason. Same rule in rayBoxRot.
  if (axis < 0) return null;
  const ln0 = axis === 0 ? sgn : 0, ln1 = axis === 1 ? sgn : 0, ln2 = axis === 2 ? sgn : 0;   // the local face normal (the other two components the literal 0 they always were)
  const n: Vec3 = [cs * ln0 + sn * ln2, ln1, -sn * ln0 + cs * ln2];
  return { t: t0, n };
}
/** Same slab test in the frame of a fully rotated box (Box.rot). */
function rayBoxRot(ro: Vec3, rd: Vec3, b: Box, tmax: number): { t: number; n: Vec3 } | null {
  // qi = quat.conj(b.rot); o = quat.rotate(qi, v3.sub(ro, b.c)); d = quat.rotate(qi, rd) — over scalars, quat.rotate's expression each (v' = v + 2w(qv × v) + 2 qv × (qv × v))
  const R = b.rot!; const qx = -R[0], qy = -R[1], qz = -R[2], qw = R[3];
  const sx = ro[0] - b.c[0], sy = ro[1] - b.c[1], sz = ro[2] - b.c[2];
  let tx = 2 * (qy * sz - qz * sy), ty = 2 * (qz * sx - qx * sz), tz = 2 * (qx * sy - qy * sx);
  const o0 = sx + qw * tx + (qy * tz - qz * ty), o1 = sy + qw * ty + (qz * tx - qx * tz), o2 = sz + qw * tz + (qx * ty - qy * tx);
  const vx = rd[0], vy = rd[1], vz = rd[2];
  tx = 2 * (qy * vz - qz * vy); ty = 2 * (qz * vx - qx * vz); tz = 2 * (qx * vy - qy * vx);
  const d0 = vx + qw * tx + (qy * tz - qz * ty), d1 = vy + qw * ty + (qz * tx - qx * tz), d2 = vz + qw * tz + (qx * ty - qy * tx);
  const h = b.h; let t0 = 0, t1 = tmax; let axis = -1, sgn = 0;
  { const inv = 1 / (d0 || 1e-20); let ta = (-h[0] - o0) * inv, tb = (h[0] - o0) * inv; let s = -1; if (ta > tb) { const tmp = ta; ta = tb; tb = tmp; s = 1; } if (ta > t0) { t0 = ta; axis = 0; sgn = s; } if (tb < t1) t1 = tb; if (t0 > t1) return null; }
  { const inv = 1 / (d1 || 1e-20); let ta = (-h[1] - o1) * inv, tb = (h[1] - o1) * inv; let s = -1; if (ta > tb) { const tmp = ta; ta = tb; tb = tmp; s = 1; } if (ta > t0) { t0 = ta; axis = 1; sgn = s; } if (tb < t1) t1 = tb; if (t0 > t1) return null; }
  { const inv = 1 / (d2 || 1e-20); let ta = (-h[2] - o2) * inv, tb = (h[2] - o2) * inv; let s = -1; if (ta > tb) { const tmp = ta; ta = tb; tb = tmp; s = 1; } if (ta > t0) { t0 = ta; axis = 2; sgn = s; } if (tb < t1) t1 = tb; if (t0 > t1) return null; }
  if (axis < 0) return null;   // origin inside: no hit — the rule rayBox documents (kept on purpose; see there for who leans on it and who must not)
  const ln: Vec3 = [0, 0, 0]; ln[axis] = sgn;
  return { t: t0, n: quat.rotate(b.rot!, ln) };
}

/** CUT BOXES (scene/cut.ts): the same test against a box with a cut record — the slab interval in the box's frame (yawed or fully rotated), cut down by the
 *  record's planes (cutClip: common.wgsl isectCut's statements). The entry is a box face or a cut plane, whichever the ray meets LAST on its way in; the normal is
 *  that face's. Origin inside the cut solid = no hit, the rule rayBox documents; an origin inside the box but in the part cut AWAY is outside, and can hit. */
function rayBoxCut(ro: Vec3, rd: Vec3, b: Box, tmax: number): { t: number; n: Vec3 } | null {
  let ox: number, oy: number, oz: number, dx: number, dy: number, dz: number; let cs = 1, sn = 0;
  const px = ro[0] - b.c[0], py = ro[1] - b.c[1], pz = ro[2] - b.c[2];
  if (b.rot) {   // (over scalars, rayBoxRot's expressions: no arrays on the raycast's path)
    const R = b.rot; const qx = -R[0], qy = -R[1], qz = -R[2], qw = R[3];
    let tx = 2 * (qy * pz - qz * py), ty = 2 * (qz * px - qx * pz), tz = 2 * (qx * py - qy * px);
    ox = px + qw * tx + (qy * tz - qz * ty); oy = py + qw * ty + (qz * tx - qx * tz); oz = pz + qw * tz + (qx * ty - qy * tx);
    const vx = rd[0], vy = rd[1], vz = rd[2];
    tx = 2 * (qy * vz - qz * vy); ty = 2 * (qz * vx - qx * vz); tz = 2 * (qx * vy - qy * vx);
    dx = vx + qw * tx + (qy * tz - qz * ty); dy = vy + qw * ty + (qz * tx - qx * tz); dz = vz + qw * tz + (qx * ty - qy * tx);
  } else {
    cs = Math.cos(b.yaw); sn = Math.sin(b.yaw);
    ox = cs * px - sn * pz; oy = py; oz = sn * px + cs * pz; dx = cs * rd[0] - sn * rd[2]; dy = rd[1]; dz = sn * rd[0] + cs * rd[2];
  }
  const h = b.h; let t0 = 0, t1 = tmax; let axis = -1, sgn = 0;
  { const inv = 1 / (dx || 1e-20); let ta = (-h[0] - ox) * inv, tb = (h[0] - ox) * inv; let s = -1; if (ta > tb) { const tmp = ta; ta = tb; tb = tmp; s = 1; } if (ta > t0) { t0 = ta; axis = 0; sgn = s; } if (tb < t1) t1 = tb; if (t0 > t1) return null; }
  { const inv = 1 / (dy || 1e-20); let ta = (-h[1] - oy) * inv, tb = (h[1] - oy) * inv; let s = -1; if (ta > tb) { const tmp = ta; ta = tb; tb = tmp; s = 1; } if (ta > t0) { t0 = ta; axis = 1; sgn = s; } if (tb < t1) t1 = tb; if (t0 > t1) return null; }
  { const inv = 1 / (dz || 1e-20); let ta = (-h[2] - oz) * inv, tb = (h[2] - oz) * inv; let s = -1; if (ta > tb) { const tmp = ta; ta = tb; tb = tmp; s = 1; } if (ta > t0) { t0 = ta; axis = 2; sgn = s; } if (tb < t1) t1 = tb; if (t0 > t1) return null; }
  if (!cutClip(b.cut!, ox, oy, oz, dx, dy, dz, t0, t1)) return null;
  const k = CUT_CLIP.k;
  if (k < 0 && axis < 0) return null;   // origin inside the cut solid
  let ln: Vec3;
  if (k >= 0) { const n = b.cut![k].n; ln = [n[0], n[1], n[2]]; } else { ln = [0, 0, 0]; ln[axis] = sgn; }
  if (b.rot) return { t: CUT_CLIP.t0, n: quat.rotate(b.rot, ln) };
  return { t: CUT_CLIP.t0, n: [cs * ln[0] + sn * ln[2], ln[1], -sn * ln[0] + cs * ln[2]] };
}

/** ROUND THINGS (scene/round.ts): the same test against a round box — the cylinder about its own y, radius h[0], between its two end faces. The ray's stay inside
 *  is one interval: its stay between the ends, cut down to its stay inside the circle (common.wgsl isectRound's statements). The normal is the end's, or points
 *  straight out from the axis at the place the ray meets the side. Origin inside = no hit, the rule rayBox documents. */
function rayBoxRound(ro: Vec3, rd: Vec3, b: Box, tmax: number): { t: number; n: Vec3 } | null {
  let ox: number, oy: number, oz: number, dx: number, dy: number, dz: number; let cs = 1, sn = 0;
  const px = ro[0] - b.c[0], py = ro[1] - b.c[1], pz = ro[2] - b.c[2];
  if (b.rot) {   // (rayBoxRot's expressions over scalars)
    const R = b.rot; const qx = -R[0], qy = -R[1], qz = -R[2], qw = R[3];
    let tx = 2 * (qy * pz - qz * py), ty = 2 * (qz * px - qx * pz), tz = 2 * (qx * py - qy * px);
    ox = px + qw * tx + (qy * tz - qz * ty); oy = py + qw * ty + (qz * tx - qx * tz); oz = pz + qw * tz + (qx * ty - qy * tx);
    const vx = rd[0], vy = rd[1], vz = rd[2];
    tx = 2 * (qy * vz - qz * vy); ty = 2 * (qz * vx - qx * vz); tz = 2 * (qx * vy - qy * vx);
    dx = vx + qw * tx + (qy * tz - qz * ty); dy = vy + qw * ty + (qz * tx - qx * tz); dz = vz + qw * tz + (qx * ty - qy * tx);
  } else {
    cs = Math.cos(b.yaw); sn = Math.sin(b.yaw);
    ox = cs * px - sn * pz; oy = py; oz = sn * px + cs * pz; dx = cs * rd[0] - sn * rd[2]; dy = rd[1]; dz = sn * rd[0] + cs * rd[2];
  }
  const R = b.h[0], hy = b.h[1]; let t0 = 0, t1 = tmax; let face = 0, sgn = 0;   // face: 0 = none entered (the origin is inside), 1 = an end, 2 = the side
  { const inv = 1 / (dy || 1e-20); let ta = (-hy - oy) * inv, tb = (hy - oy) * inv; let s = -1; if (ta > tb) { const tmp = ta; ta = tb; tb = tmp; s = 1; } if (ta > t0) { t0 = ta; face = 1; sgn = s; } if (tb < t1) t1 = tb; if (t0 > t1) return null; }
  const a = dx * dx + dz * dz, hb = ox * dx + oz * dz, c = ox * ox + oz * oz - R * R;
  if (a < 1e-18) { if (c > 0) return null; }   // along the axis: inside the circle for every t, or never
  else { const disc = hb * hb - a * c; if (disc < 0) return null; const s = Math.sqrt(disc), ta = (-hb - s) / a, tb = (-hb + s) / a; if (ta > t0) { t0 = ta; face = 2; } if (tb < t1) t1 = tb; if (t0 > t1) return null; }
  if (face === 0) return null;   // origin inside
  let l0 = 0, l1 = sgn, l2 = 0;
  if (face === 2) { const hx = ox + dx * t0, hz = oz + dz * t0, il = 1 / (Math.sqrt(hx * hx + hz * hz) || 1); l0 = hx * il; l1 = 0; l2 = hz * il; }
  if (b.rot) return { t: t0, n: quat.rotate(b.rot, [l0, l1, l2]) };
  return { t: t0, n: [cs * l0 + sn * l2, l1, -sn * l0 + cs * l2] };
}
