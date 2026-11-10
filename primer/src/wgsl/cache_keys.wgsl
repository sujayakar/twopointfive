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
