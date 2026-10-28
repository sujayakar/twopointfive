// ---- THE LOOK PROTOTYPE'S LOOSE SNOW (render/powderField.ts): the powder the wind carries over lying snow, as ONE LAYER OF STUFF IN THE GAME'S OWN AIR.
// PICTURE ONLY. The maintainer, 10-28, of the first try (a pattern slid along one wind vector, and sprites for a boot's puff): "the snow blowing in the wind
// doesn't look like it reacts to walls, and it seems to imply a wind velocity that doesn't match the wind velocity for the falling snow"; the puffs "look like
// particle effects". So nothing here has a wind of its own: every cell of powder is carried by the air field's velocity where it is (render/airField.ts, the
// field the falling flakes ride), which is what bends it round a wall's end, stalls it in a lee and draws it through a gap.
//
// ONE CELL = 1 / U.a.w metres (finer than the air's: a thread of powder is a hand wide). A texel holds   r = the powder over that spot as a COLUMN (the optical
// depth seen from straight above),   g = that column × its height in metres (so height = g / r, and the two are carried alike),   b, a = the air's velocity there.
// ONE STEP, one kernel, old → new:
//   CARRY     back along the air (a midpoint step) and read there with a cubic held inside what the four nearest cells hold: a plain bilinear read smears a
//             thread into a cloud in a second at these speeds (the step moves a fifteenth of a cell).
//   SETTLE    powder in moving air stays up for U.b.x's life; in still air it drops out in U.b.y's. That alone is why a lee is clear and its edge is drawn.
//             THICK powder falls out faster than thin (U.e.w per unit of column): a boot's puff is gone in a second or two but for the thin tail of it
//             the wind takes, and a streamer cannot pile up into a bank of fog.
//   PICK-UP   lying snow under the open sky gives powder where the air runs at the ambient wind or over it (not in a lee; most in a jet round a corner or
//             through a gap), from SMALL CRESTS that stand where they are and come and go slowly, inside gusts' patches: a crest's powder is drawn out
//             downwind into a thread by the carry, which is how a streamer is made. Where the air goes UP (the field's lift: a wall's windward foot) it is
//             lifted there too, and higher.
//   KICKS     a boot, a body, a blast: a round of powder put in where it landed, by how hard (PowderField.kick), swelling for a third of a second, in
//             LUMPS of its own (a smooth round reads as a stamp) that the air then pulls about.
// A solid cell holds none.

struct PowKick {
  a: vec4f,   // x, z (m), sigma (m), the column added at its middle this step
  b: vec4f,   // the height it is thrown to (m), its own seed (its lumps), spare × 2
};
struct PowU {
  a: vec4f,     // dt (s), the caller's clock (s), × the air's velocity (the height share the flakes take at the powder's height), cells a metre
  b: vec4f,     // kept a step in moving air, kept a step in still air, picked up a step at a full crest, lifted a step per m/s of the air's lift
  c: vec4f,     // the ambient wind's speed (m/s), the pick-up's gate from … to (× the ambient), the height's own keep a step
  d: vec4f,     // a gust patch's size (m), a crest's size (m), the crests' cut (0‥1: over it a crest gives), how fast both change (1/s)
  e: vec4f,     // the height wind-lifted powder is put in at (m), + this per m/s of lift, the seed, a step's extra fall-out per unit of column
  size: vec4u,  // w, h (this field's cells), the air field's w, h
  n: vec4u,     // kicks this step, spare × 3
  air: vec4f,   // the air field's cells a metre, spare × 3
  kicks: array<PowKick, 16>,
};
@group(0) @binding(0) var<uniform> U: PowU;
@group(0) @binding(1) var airTex: texture_2d<f32>;    // render/airField.ts view: r, g = velocity m/s (x, z), b = lift m/s, a = 0 in a solid
@group(0) @binding(2) var cellTex: texture_2d<u32>;   // its cell map: 255 = solid, else the share of the open wind × 254
@group(0) @binding(3) var prevTex: texture_2d<f32>;
@group(0) @binding(4) var nextTex: texture_storage_2d<rgba16float, write>;

fn powHash(p: vec3i, salt: u32) -> f32 {
  var h = (u32(p.x) * 0x9E3779B1u) ^ (u32(p.y) * 0x85EBCA77u) ^ (u32(p.z) * 0xC2B2AE3Du) ^ (salt * 0x27D4EB2Fu);
  h ^= h >> 15u; h *= 0x2C1B3C6Du; h ^= h >> 12u; h *= 0x297A2D39u; h ^= h >> 15u;
  return f32(h >> 8u) * (1.0 / 16777216.0);
}
/** a smooth value noise 0‥1 over x, z and time */
fn powNoise(p: vec3f, salt: u32) -> f32 {
  let i = floor(p); let f = p - i; let u = f * f * (3.0 - 2.0 * f); let c = vec3i(i);
  var z: array<f32, 2>;
  for (var k = 0; k < 2; k++) {
    let a = mix(powHash(c + vec3i(0, 0, k), salt), powHash(c + vec3i(1, 0, k), salt), u.x);
    let b = mix(powHash(c + vec3i(0, 1, k), salt), powHash(c + vec3i(1, 1, k), salt), u.x);
    z[k] = mix(a, b, u.y);
  }
  return mix(z[0], z[1], u.z);
}
/** the air map at p (metres): 255 = solid; past the world's rim the open air */
fn powCell(p: vec2f) -> u32 {
  let c = vec2i(floor(p * U.air.x));
  if (c.x < 0 || c.y < 0 || c.x >= i32(U.size.z) || c.y >= i32(U.size.w)) { return 254u; }
  return textureLoad(cellTex, c, 0).r;
}
/** the air at p (metres), between its cells' centres */
fn powAir(p: vec2f) -> vec4f {
  let q = p * U.air.x - 0.5; let i = floor(q); let f = q - i; let m = vec2i(U.size.zw) - 1;
  let c0 = clamp(vec2i(i), vec2i(0), m); let c1 = clamp(vec2i(i) + 1, vec2i(0), m);
  let a = textureLoad(airTex, c0, 0); let b = textureLoad(airTex, vec2i(c1.x, c0.y), 0);
  let c = textureLoad(airTex, vec2i(c0.x, c1.y), 0); let d = textureLoad(airTex, c1, 0);
  return mix(mix(a, b, f.x), mix(c, d, f.x), f.y);
}
fn powCubic(t: f32) -> vec4f {
  let t2 = t * t; let t3 = t2 * t;
  return vec4f(-0.5 * t3 + t2 - 0.5 * t, 1.5 * t3 - 2.5 * t2 + 1.0, -1.5 * t3 + 2.0 * t2 + 0.5 * t, 0.5 * t3 - 0.5 * t2);
}
/** the old field at q (in cells), a cubic read held inside what the four nearest hold; nothing past the rim */
fn powPrev(q: vec2f) -> vec2f {
  let s = q - 0.5; let fl = floor(s); let i = vec2i(fl); let f = s - fl; let wx = powCubic(f.x); let wy = powCubic(f.y);
  var sum = vec2f(0.0); var lo = vec2f(1e9); var hi = vec2f(0.0);
  for (var b = 0; b < 4; b++) { for (var a = 0; a < 4; a++) {
    let c = i + vec2i(a - 1, b - 1); var v = vec2f(0.0);
    if (c.x >= 0 && c.y >= 0 && c.x < i32(U.size.x) && c.y < i32(U.size.y)) { v = textureLoad(prevTex, c, 0).rg; }
    sum += v * (wx[a] * wy[b]);
    if (a >= 1 && a <= 2 && b >= 1 && b <= 2) { lo = min(lo, v); hi = max(hi, v); }
  } }
  return clamp(sum, lo, hi);
}

@compute @workgroup_size(8, 8)
fn powderStep(@builtin(global_invocation_id) id: vec3u) {
  if (id.x >= U.size.x || id.y >= U.size.y) { return; }
  let ij = vec2i(id.xy); let p = (vec2f(id.xy) + 0.5) / U.a.w;
  let cell = powCell(p);
  if (cell == 255u) { textureStore(nextTex, ij, vec4f(0.0)); return; }
  let A = powAir(p); let v = A.xy * U.a.z; let dt = U.a.x;
  // CARRY
  let vm = powAir(p - 0.5 * dt * v).xy * U.a.z; var p0 = p - dt * vm;
  if (powCell(p0) == 255u) { p0 = p; }
  var q = powPrev(p0 * U.a.w);
  // SETTLE
  let amb = max(U.c.x, 1e-3); let rel = length(v) / amb;
  let keep = mix(U.b.y, U.b.x, smoothstep(0.25, 0.8, rel));
  let thick = keep / (1.0 + U.e.w * q.x);
  q = vec2f(q.x * thick, q.y * thick * U.c.w);
  // PICK-UP
  let share = f32(cell) / 254.0;
  if (share > 0.5) {
    let salt = u32(U.e.z); let t = U.a.y * U.d.w; let blowing = smoothstep(0.05, 0.2, amb);
    let gust = smoothstep(0.38, 0.62, powNoise(vec3f(p / U.d.x, t), salt));
    let crest = smoothstep(U.d.z, min(U.d.z + 0.1, 1.0), powNoise(vec3f(p / U.d.y, t * 1.7), salt + 1u));
    let lift = max(A.z, 0.0);
    let s = share * blowing * U.b.z * smoothstep(U.c.y, U.c.z, rel) * gust * crest;
    let sl = share * blowing * U.b.w * lift * (0.35 + 0.65 * powNoise(vec3f(p / (0.6 * U.d.y), t * 2.3), salt + 2u));
    q += vec2f(s + sl, s * U.e.x + sl * (U.e.x + U.e.y * lift));
  }
  // KICKS
  for (var k = 0u; k < min(U.n.x, 16u); k++) {
    let K = U.kicks[k]; let r = (p - K.a.xy) / max(K.a.z, 1e-3); let r2 = dot(r, r);
    if (r2 < 9.0) {
      let lump = 0.3 + 1.4 * powNoise(vec3f(p / 0.16, K.b.y), u32(U.e.z) + 3u);   // (about 1 on average; a lump is a boot long, over a cell and a half of the field)
      let add = K.a.w * exp(-0.5 * r2) * lump; q += vec2f(add, add * K.b.x * (0.6 + 0.4 * lump));
    }
  }
  textureStore(nextTex, ij, vec4f(max(q, vec2f(0.0)), v));
}
