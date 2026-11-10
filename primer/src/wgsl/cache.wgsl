// THE RADIANCE CACHE. A table of entries on surfaces. An entry's key is the cell its point is in, the way
// the surface faces, and which of two it is (a cell with a wall through it has two sides). It keeps the light
// that lands there, in two sums: straight from lamps, and everything else.
//
//   cache_keys    the key in each slot, 0 for none (a place in the table is found by walking on from key % size)
//   cache_points  where each entry is and which way it faces
//   cache         what it has gathered

const CACHE_SLOTS = 32768u;
const CACHE_REACH = 24u;

fn cache_facing(n: vec3f) -> u32 {
  if (knobs.break_no_facing != 0u) { return 0u; }
  let a = abs(n);
  if (a.x >= a.y && a.x >= a.z) { return select(1u, 0u, n.x > 0.0); }
  if (a.y >= a.z) { return select(3u, 2u, n.y > 0.0); }
  return select(5u, 4u, n.z > 0.0);
}

fn cache_key(position: vec3f, normal: vec3f, side: u32) -> u32 {
  // pushed off the surface first: floors and walls lie on cell borders, and rounding would toss them about
  let cell = vec3i(floor((position + normal * knobs.cache_push) / knobs.cache_cell));
  let h = hash2(bitcast<u32>(cell.x), hash2(bitcast<u32>(cell.y), hash2(bitcast<u32>(cell.z), cache_facing(normal) + 8u * side)));
  return max(h, 1u);
}

/// The slot that holds a key, or NOTHING.
fn cache_find(key: u32) -> u32 {
  var slot = key % CACHE_SLOTS;
  for (var i = 0u; i < CACHE_REACH; i++) {
    let k = atomicLoad(&cache_keys[slot]);
    if (k == key) { return slot; }
    if (k == 0u) { return NOTHING; }
    slot = (slot + 1u) % CACHE_SLOTS;
  }
  return NOTHING;
}

struct Founding { slot: u32, is_new: bool }

/// Takes a free slot for a key. Of several that try at once, one gets `is_new`.
fn cache_take(key: u32) -> Founding {
  var slot = key % CACHE_SLOTS;
  for (var i = 0u; i < CACHE_REACH * 2u; i++) {
    let k = atomicLoad(&cache_keys[slot]);
    if (k == key) { return Founding(slot, false); }
    if (k == 0u) {
      let r = atomicCompareExchangeWeak(&cache_keys[slot], 0u, key);
      if (r.exchanged) { return Founding(slot, true); }
      continue; // someone else was first, or it failed for nothing: look at the same slot again
    }
    slot = (slot + 1u) % CACHE_SLOTS;
  }
  return Founding(NOTHING, false);
}

struct CacheSpot {
  slot: u32,  // NOTHING: no entry speaks for this place
  side: u32,  // which of the two is free to be founded; 2: neither
}

/// The entry that speaks for a place: of the two that its cell may hold, the one that can be seen from it.
/// `nudge` (0 to 1 each way) looks up a little to one side first, so that cells blend into each other as noise
/// and not as blocks; if no entry speaks for that, the place itself is asked.
fn cache_spot_near(position: vec3f, normal: vec3f, nudge: vec2f) -> CacheSpot {
  let axes = frame_around(normal);
  let d = (nudge - 0.5) * knobs.cache_cell;
  let is_nudged = knobs.cache_nudge != 0u && any(nudge != vec2f(0.5));
  var last = CacheSpot(NOTHING, 2u);
  for (var k = select(2u, 0u, is_nudged); k < 4u; k++) { // nudged: its two sides; then itself: its two sides
    let place = position + (axes[0] * d.x + axes[1] * d.y) * select(0.0, 1.0, k < 2u);
    let side = k & 1u;
    let slot = cache_find(cache_key(place, normal, side));
    if (slot == NOTHING) {
      last = CacheSpot(NOTHING, side);
      if (k < 2u) { k = 1u; continue; } // on to the place itself
      return last;
    }
    let there = cache_points[slot];
    if (knobs.cache_wall_test == 0u || dot(there.normal, there.normal) == 0.0) { return CacheSpot(slot, side); }
    let start = position + normal * 0.02;
    let to = there.position + there.normal * 0.02 - start;
    let far = length(to);
    if (far < 0.02 || nearest(Ray(start, to / far, 0.0, far)).piece == NOTHING) { return CacheSpot(slot, side); }
  }
  return CacheSpot(NOTHING, 2u);
}
fn cache_spot(position: vec3f, normal: vec3f) -> CacheSpot {
  return cache_spot_near(position, normal, vec2f(0.5));
}
