// Written by tools/draw/gen.ts from rust/game_draw. To change a section, change it there.

/** the stamp of the sections this was made from: a module of another stamp is refused (drawn.ts) */
export const LAYOUT = 1179116878;
/** each section's place: in the table the module answers with, and its bit in what a host asks for */
export const enum Section { fog = 0, count = 1 }

/** the fog: one row */
export const enum Fog {
  /** u32: the fog is drawn; 0 = every other field is 0 */
  on = 0,
  /** u32: live eyes */
  count = 1,
  /** f32 × 28: an eye: x y z, then the man's id (0 = none) */
  eyes = 2,
  /** f32 × 28: where it looks in plan (x, z; unit), the cosine of its half field (-2 = all round), its all-round near field in metres */
  looks = 30,
  /** f32 */
  dim = 58,
  /** f32 */
  desat = 59,
  /** f32 */
  topPad = 60,
  /** f32: the band the fog comes in over across a sight edge, metres */
  edge = 61,
  /** f32: the wedge's soft edge: the cosines of the half field less and plus half the soft band, and the metres the near field's rim comes in over */
  softIn = 62,
  /** f32 */
  softOut = 63,
  /** f32 */
  nearSoft = 64,
  /** f32 */
  flat = 65,
  /** f32: the lamps' strength in the fog (0 = none), the fill, the level under which a lamp is not traced, and the fill's corner shade and its reach */
  lit = 66,
  /** f32 */
  litFill = 67,
  /** f32 */
  litCut = 68,
  /** f32 */
  litAo = 69,
  /** f32 */
  litAoM = 70,
  /** u32 */
  lightCount = 71,
  /** u32 × 768: the lamps, as the scene's own light record is packed: 16 words a lamp */
  lights = 72,
  /** u32: the full picture under the fog, and its fill */
  full = 840,
  /** f32 */
  fullFlat = 841,
  /** u32: smoke and dust are drawn over floor nobody sees */
  air = 842,
  /** u32: thin things do not cut a sight line */
  thin = 843,
  /** u32: what lies still stays as last seen */
  kept = 844,
  /** u32 × 128: which statics are thin, a bit a box, and the list they were read off: its geometry serial and its length */
  thinBits = 845,
  /** u32 */
  thinSerial = 973,
  /** u32 */
  thinCount = 974,
  /** words a row */
  row = 975,
}
