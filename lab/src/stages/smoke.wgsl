// SMOKE IN BRICKS. The world is cells of H; eight cells a side make a brick, and only bricks that hold smoke, or may by the next step, are kept: a page says which
// slot of the pool holds which brick. So the cells can be small and the cost is the smoke's, not the building's.
//   M   page[brick] 0 or slot + 1 · per slot: its brick + 1, when last wanted, how dense, its 27 neighbours · the bricks asked for, the live slots
//   (which slot a brick gets is decided in the bricks' own order, so that a run repeats to the bit: the atlas is read at the slot's place, and filters round by it)
//   V   velocity xyz and density, as halves      Q   heat, twice over      P   pressure, then what is not yet balanced
//   W   which of a cell's six faces are shut: bits 0‥5 by what stands (+x +y +z −x −y −z), 6‥11 by that or a door
const H = 0.08; const AT = vec3f(-24.0, -0.16, -19.2); const PN = vec3i(75, 24, 60); const PAGES = 108000u; const SLOTS_S = 3072u; const NONE = 0xffffffffu;
const M_PAGE = 0u; const M_BRICK = 108000u; const M_STAMP = 111072u; const M_DENSE = 114144u; const M_NBR = 117216u; const M_FREE = 200160u; const M_LIVE = 203232u; const M_N = 206304u;   // (there: how many live, how many new)
// (M_FREE now lists the bricks asked for this step)
const M_BORN = 206320u; const M_MEAN = 209392u;                  // (there: each live brick's own mean speed)
fn brickIx(b: vec3i) -> u32 { return u32(b.x + PN.x * (b.y + PN.y * b.z)); }
fn brickOf(i: u32) -> vec3i { let x = i32(i) % PN.x; let y = (i32(i) / PN.x) % PN.y; return vec3i(x, y, i32(i) / (PN.x * PN.y)); }
fn inPage(b: vec3i) -> bool { return all(b >= vec3i(0)) && all(b < PN); }
fn local(c: vec3i) -> u32 { return u32(c.x + 8 * (c.y + 8 * c.z)); }
fn cellOf(i: u32) -> vec3i { return vec3i(i32(i & 7u), i32((i >> 3u) & 7u), i32(i >> 6u)); }
fn mid(b: vec3i, c: vec3i) -> vec3f { return AT + (vec3f(b * 8 + c) + 0.5) * H; }
