// Written by the program `half` (rust/game_start/src/bin/half) from rust/game_draw/src/gpu.rs. To change a row a shader reads, change it there: the shader's own struct is written with this (src/render/shaders/rows.wgsl).

/** one of a man's proxies, as a shader traces it: a round cone */
export const enum Capsule {
  /** f32 × 3 */
  a = 0,
  /** f32 */
  r = 3,
  /** f32 × 3 */
  b = 4,
  /** u32: whose it is in the low byte, its colour (5, 6, 5 bits) in the next two, and in the top one the radius at `b` in millimetres (0: `r` again) */
  misc = 7,
  /** words a row */
  row = 8,
}

/** a light, as a shader reads it */
export const enum LightRow {
  /** f32 × 3 */
  pos = 0,
  /** u32: 0 a point, 1 a spot, 2 a directional one, 3 an area (a one-sided disk: its normal `dir`, its radius `radius`, its colour radiance by area); flags above the low byte */
  kind = 3,
  /** f32 × 3 */
  dir = 4,
  /** f32 */
  range = 7,
  /** f32 × 3 */
  color = 8,
  /** f32 */
  cosOuter = 11,
  /** f32 */
  cosInner = 12,
  /** f32 */
  volumetric = 13,
  /** u32: the low byte a man's, the panes it shines through above it */
  owner = 14,
  /** f32 */
  radius = 15,
  /** words a row */
  row = 16,
}

/** a mark's head; its cells follow, twelve to a float */
export const enum MarkHead {
  /** f32 × 4: where it lies on the face and how many cells it is across and up */
  place = 0,
  /** f32: its face in the low 3 bits, its cells' size in 2, then: the box's last mark, some of the list emits, its light is an area's, which entries emit (3 bits), which have a colour and which a roughness (3 and 3) */
  says = 4,
  /** f32 × 3: how much of it each of its three entries covers */
  shares = 5,
  /** f32 × 24: an entry's colour and roughness (under 0: the box's), then what it emits */
  pal = 8,
  /** words a row */
  row = 32,
}

/** THE BOXES' MATERIAL BUFFER: where each of its tables begins, in rows of four floats, and how long it is */
export const enum BoxMat { mat = 0, mark = 16384, pane = 22528, air = 23680, cut = 24800, rows = 61664 }
