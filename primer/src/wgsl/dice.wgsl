// THE DICE. Every random number has a pixel, a frame and a name (`what`), so any of them can be
// repeated. Three kinds:
//   white    independent throws
//   filling  each pixel's throws fill the square evenly over time (Owen-scrambled Sobol, Burley 2020)
//   blue     the same, and neighboring pixels throw differently, so the noise is fine-grained

fn hash(x: u32) -> u32 {
  let s = x * 747796405u + 2891336453u;
  let w = ((s >> ((s >> 28u) + 4u)) ^ s) * 277803737u;
  return (w >> 22u) ^ w;
}
fn hash2(a: u32, b: u32) -> u32 { return hash(a + hash(b)); }

fn unit(x: u32) -> f32 { return f32(x >> 8u) / 16777216.0; }

fn laine_karras(x0: u32, seed: u32) -> u32 {
  var x = x0 + seed;
  x ^= x * 0x6c50b47cu;
  x ^= x * 0xb82f1e52u;
  x ^= x * 0xc7afe638u;
  x ^= x * 0x8d22f6e6u;
  return x;
}
fn scramble(x: u32, seed: u32) -> u32 {
  return reverseBits(laine_karras(reverseBits(x), seed));
}
fn sobol_second(i: u32) -> u32 {
  var v = 0x80000000u;
  var out = 0u;
  for (var k = i; k != 0u; k >>= 1u) {
    if ((k & 1u) != 0u) { out ^= v; }
    v ^= v >> 1u;
  }
  return out;
}
fn filling2(index: u32, seed: u32) -> vec2f {
  let i = scramble(index, seed);
  let x = scramble(reverseBits(i), hash2(seed, 1u));
  let y = scramble(sobol_second(i), hash2(seed, 2u));
  return vec2f(unit(x), unit(y));
}

fn blue_at(p: vec2u) -> f32 {
  let i = (p.y & 63u) * 64u + (p.x & 63u);
  return tables.blue[i >> 2u][i & 3u];
}

/// Two numbers from 0 to 1, for pixel `pixel`, throw number `index`, named `what`.
fn dice_of(kind: u32, pixel: vec2u, index: u32, what: u32) -> vec2f {
  let home = hash2(pixel.x, hash2(pixel.y, what));
  if (kind == DICE_KIND_WHITE) {
    let h = hash2(index, home);
    return vec2f(unit(h), unit(hash(h)));
  }
  if (kind == DICE_KIND_FILLING) { return filling2(index, home); }
  let step = filling2(index, hash(what));
  let shift = vec2u(hash(what + 11u), hash(what + 23u)) >> vec2u(16u);
  let offset = vec2f(blue_at(pixel + shift), blue_at(pixel + shift + vec2u(31u, 17u)));
  return fract(step + offset);
}

fn dice2(pixel: vec2u, what: u32) -> vec2f {
  let index = select(frame.number, 0u, knobs.dice_frozen != 0u);
  return dice_of(knobs.dice_kind, pixel, index, what);
}
fn dice1(pixel: vec2u, what: u32) -> f32 { return dice2(pixel, what).x; }

// the names
const DICE_LAMP_PICK = 1u;
const DICE_LAMP_POINT = 2u;
const DICE_WIDE = 3u;
const DICE_NARROW = 4u;
const DICE_Y_LAMP = 5u;
const DICE_Y_POINT = 6u;
const DICE_SHINE_PICK = 7u;
const DICE_SHINE_POINT = 8u;
const DICE_WIDE_WAY = 9u;
const DICE_NARROW_RAY = 10u;
const DICE_NUDGE = 11u;
