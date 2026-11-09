// What every way of keeping light shares with the others: what a reader says and gets, and the light that is asked for rather than kept.
struct Cached { E: vec3f, sun: f32, n: f32 };
const EVERY_WAY = 0x80010003u;                                      // a normal that is none: foliage, which faces every way
var<private> askWide: f32 = 1.0; var<private> askPiece: u32 = NOTHING;
fn facing(n: vec3f) -> u32 { if (all(n == vec3f(0.0))) { return 6u; } let a = abs(n); if (a.x >= a.y && a.x >= a.z) { return select(1u, 0u, n.x > 0.0); } if (a.y >= a.z) { return select(3u, 2u, n.y > 0.0); } return select(5u, 4u, n.z > 0.0); }
/** the level itself is kept finely; the city round it only coarsely */
fn cacheNear(x: vec3f) -> bool { return abs(x.x) < 20.0 && abs(x.z) < 15.0 && x.y > -0.6 && x.y < 12.0; }
fn unOct(p: u32) -> vec3f { return octUnpack(p & 0xfffffffeu); }
