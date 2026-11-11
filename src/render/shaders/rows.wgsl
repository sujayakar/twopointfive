// Written by the program `half` (rust/game_start/src/bin/half) from rust/game_draw/src/gpu.rs: THE ROWS A SHADER READS THAT RUST LAYS OUT. To change one, change it there.

// a box's place
struct BoxGeo {
  c: vec3f,   // its middle
  rot: u32,   // its yaw's cosine and sine (a tilted box is turned by `Mat.rot`)
  h: vec3f,   // its half sizes
  flags: u32,   // its bits; whose it is in the second byte; from bit 18 the first row (from 1) of its marks
};
fn boxGeoRot(r: BoxGeo) -> vec2f { return unpack2x16float(r.rot); }

// the scene's block: how many boxes, the grid, the men's bounds, the panes
struct SceneInfo {
  numBoxes: u32,
  numGlobals: u32,
  gridW: u32,
  gridH: u32,
  globals: array<vec4u, 4>,   // the boxes too large for the grid, by their places
  numCapsules: u32,
  numChars: u32,
  capsPerChar: u32,
  itemsB: u32,   // where the packed copies of the grid's items begin
  charBounds: array<vec4f, 16>,   // a man's proxies' sphere
  globalsY: array<vec4f, 8>,   // a global's padded lowest and highest, two to a row
  grid: vec4f,   // x: how far a traced box can reach outside the cells it is registered in (m); y: how much tighter a cell's height span is judged than it is packed
  globalsB: array<vec4f, 32>,   // a global's bounds in the world: its least corner, then its greatest, a little proud of it
  glassInfo: vec4u,   // x, y: how many panes
  glass: array<vec4f, 96>,   // a pane: its middle and its yaw's cosine, its half sizes and the sine, its tint and its place among the statics
};

// one of a man's proxies, as a shader traces it: a round cone
struct Capsule {
  a: vec3f,
  r: f32,
  b: vec3f,
  misc: u32,   // whose it is in the low byte, its colour (5, 6, 5 bits) in the next two, and in the top one the radius at `b` in millimetres (0: `r` again)
};

// a light, as a shader reads it
struct Light {
  pos: vec3f,
  kind: u32,   // 0 a point, 1 a spot, 2 a directional one, 3 an area (a one-sided disk: its normal `dir`, its radius `radius`, its colour radiance by area); flags above the low byte
  dir: vec3f,
  range: f32,
  color: vec3f,
  cosOuter: f32,
  cosInner: f32,
  volumetric: f32,
  owner: u32,   // the low byte a man's, the panes it shines through above it
  radius: f32,
};

// a man's instance, as the skinning reads it
struct CharInst {
  tint: vec3f,
  jointOffset: u32,   // where his bones begin
  tint2: vec3f,
  owner: u32,
  wet: vec4f,   // the wet coat: how wet, its beads, how much of it is rain, its phase
  wetK: vec4f,   // its numbers: beads' density and size, a trail's length, the roughness wet · dry, how it darkens and by what, its shine · the waterline (bind-pose m above the sole the coat reaches) and the wicking band above it
  wetK2: vec4f,
  wetK3: vec2f,
  parts: u32,   // the cast's parts he wears, a bit a part
  pad: u32,
};
