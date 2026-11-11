// Decals (EXPERIMENT E1): the marks an encounter leaves on the level — bullet holes, the scorch of a stun charge, boot scuffs on a kicked leaf,
// the pale chip a ring airfoil knocks out of the plaster. NOT geometry and not boxes in the trace grid: the game's own list (rust/game_rules marks),
// bucketed into the level's 1 m XZ cells, sent (when it changed) as ONE read-only storage buffer that the g-buffer's FRAGMENT stage reads
// (render/shaders/decals.wgsl): for the shaded pixel's world position + face normal + box index it walks its cell's few entries, rejects by
// bounding sphere / facing / plane distance / owning box, and modulates the albedo it writes (darken + tint, a paler chipped rim), tilts the
// normal a little across a crater's bevel, and lowers the albedo target's alpha where soot should kill the polished floors' gloss. Because it is
// the g-buffer albedo / normal that change, everything downstream — the traced direct light, the final gather's bounce AT the pixel, the gloss
// term — responds without knowing decals exist. (What does NOT respond: the bounce OFF a marked surface onto its neighbours — secondary hits
// shade with the box's own material.)
//
// Marks on a DYNAMIC box (a door leaf, a piece of loose furniture) follow where the box now stands,
// restricted on the GPU to pixels of that very box; marks on statics are world-fixed and paint any surface that passes
// the facing + plane tests (a scorch runs across the floor slab AND the rug lying on it). Lifetime: the encounter, the oldest going past CAP.
//
// One buffer, four regions (word offsets exported to the shader by decalWgslConsts, so both sides read one layout):
//   [0]                       header vec4u: count, list length, serial, 1 while marks SIT IN THE SURFACE (decals.wgsl dkSit; 0 = every mark drawn as it was)
//   [CELL0, +CELLS_V4)        the cell table: one u32 per 1 m cell = start16 | count16 into the index list (4 cells a vec4)
//   [LIST0, +LIST_V4)         the index list: decal indices per cell, oldest first (4 a vec4)
//   [REC0, +3·CAP)            the records, 3 vec4 each: (pos.xyz, radius) (normal.xyz, param) (tangent.xyz, bits(kind3 | attached1 | seed12 | box16))
//                             (box16 = the box an attached mark rides, 0xffff for a world-fixed one, or — kind glassHit — the static index of ITS pane)
//
// Kind 'wetprint' (rust/game_rules/src/air/wet.rs): a wet shoe print, param = how wet (rust/game_rules/src/air/wet.rs lowers it as it dries).
// Kind 'damage' with param ≥ WEAR_MARK.look (rust/game_rules/src/breakage/wallwear.rs): a WALL-WEAR mark — a round's own 'bullet' record EDITED in place
// into what a breakable shows before it gives. See WEAR_MARK below; decals.wgsl wearMark draws it.
// Kind 'glassHit' (E3 × E1): a round's hole + crack star on a glass pane. Panes are in no raster, so the g-buffer's decalSurface skips the kind outright; it is
// composite.wgsl that evaluates it (glassMarks in decals.wgsl) at the point the view ray crosses the pane, from the same cell lists, and modulates the pane's
// transmission / reflection there. Same ring, same lifetime, same buffer — the composite pass binds it too (passes.ts group 1 binding 10).
import { WORLD } from '../scene/world';
import { makeBuffer } from '../gpu/device';


// ---- buffer layout (words of 16 bytes) — decalWgslConsts() hands the same numbers to the shader
const CELLS = WORLD.gridW * WORLD.gridH;
const CELLS_V4 = Math.ceil(CELLS / 4);
export const DECAL_CAP = 512;          // live decals (ring: the oldest goes)
export const DECAL_LIST_CAP = 4096;    // cell-list entries in all (most decals sit in ≤ 4 cells; a floor film of settled dust — rust/game_rules/src/breakage/floormess.rs, up to ~1.8 m — in up to 25)
const LIST_V4 = DECAL_LIST_CAP / 4;
const CELL0 = 1, LIST0 = CELL0 + CELLS_V4, REC0 = LIST0 + LIST_V4;
const TOTAL_V4 = REC0 + DECAL_CAP * 3;
export const DECAL_BYTES = TOTAL_V4 * 16;

/** Measurement only (read when the g-buffer shader is built): evaluate the decal lookup N times per fragment so its cost rises above the
 *  timer's noise floor — the reported delta ÷ N is the honest per-fragment figure. 1 = production. */
export const decalDebug = { repeat: 1 };
/** WGSL consts for render/shaders/decals.wgsl: the layout above + the caps (prepended by gbuffer.ts next to the scene prelude) */
/** WALL-WEAR marks (rust/game_rules/src/breakage/wallwear.rs writes them, decals.wgsl decalSurface / wearMark reads them — these numbers are the contract, emitted into the shader by
 *  decalWgslConsts): kind 'damage', param = look + material (0 plaster · 1 wood · 2 cardboard / fabric) + strength 0‥0.999. A mark's reach on a face (m) =
 *  reach0 + reach1 × strength. A THROUGH mark (a partition of half thickness minHalf‥maxHalf) sits on the wall's mid-plane with radius hypot(half, reach): the
 *  shader recovers the half thickness from the two and paints ONLY surfaces lying on the wall's two face planes (± faceTol), entry look on the one toward the
 *  record's normal, exit look on the other — never a door leaf or a prop that merely stands in the wall's plane. A FACE mark (radius = reach) is an ordinary decal. */
export const WEAR_MARK = { look: 3, materials: 3, reach0: 0.09, reach1: 0.21, minHalf: 0.035, maxHalf: 0.09, faceTol: 0.015 };
export function decalWgslConsts(binding = 3): string {
  return `@group(1) @binding(${binding | 0}) var<storage, read> decalBuf: array<vec4u>;   // render/decals.ts — the g-buffer binds it at 3, the composite pass (glass cracks) at 10. UNSIGNED words: the cell / list regions are small integers (subnormal bit patterns as f32) and a record's bits word can be a NaN pattern — a float-typed view may flush or canonicalise those on load on some hardware; the float rows are bitcast<vec4f> at the read (review R2.1 / R10.3)\nconst DECAL_CELL0: u32 = ${CELL0}u;\nconst DECAL_LIST0: u32 = ${LIST0}u;\nconst DECAL_REC0: u32 = ${REC0}u;\nconst DECAL_CAP: u32 = ${DECAL_CAP}u;\nconst DECAL_LIST_CAP: u32 = ${DECAL_LIST_CAP}u;\nconst DECAL_REPEAT: u32 = ${Math.max(1, decalDebug.repeat | 0)}u;   // measurement only (decalDebug.repeat): 1 in production\n` +
    `const WEAR_LOOK: f32 = ${WEAR_MARK.look.toFixed(1)}; const WEAR_LOOKS: f32 = ${WEAR_MARK.materials.toFixed(1)}; const WEAR_REACH0: f32 = ${WEAR_MARK.reach0}; const WEAR_REACH1: f32 = ${WEAR_MARK.reach1}; const WEAR_MIN_HALF: f32 = ${(WEAR_MARK.minHalf - 0.005).toFixed(4)}; const WEAR_FACE_TOL: f32 = ${WEAR_MARK.faceTol};   // render/decals.ts WEAR_MARK\n`;
}

/** the renderer's end of the marks: the buffer the two shaders read, and what is told of it. The marks are the game's (rust/game_draw marks.rs lays the buffer out), and the page's own
 *  class (draw/marks.ts) sends it */
export abstract class Decals {
  /** off (the picture's settings.decals, written here by the engine): frame() sends nothing and the engine keeps FrameFlags.Decals down, so the buffer stays exactly as it was */
  enabled = true;
  readonly buf: GPUBuffer;
  /** what the last frame was told: marks drawn, list entries, entries the caps left out, uploads so far */
  stats = { count: 0, listLen: 0, dropped: 0, uploads: 0 };
  constructor(device: GPUDevice) { this.buf = makeBuffer(device, DECAL_BYTES, GPUBufferUsage.STORAGE | GPUBufferUsage.COPY_DST, 'decals'); }
  abstract get count(): number;
  abstract frame(): void;
}
