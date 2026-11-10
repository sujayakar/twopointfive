/** who wins a cell this frame: the smallest of its callers' numbers, shuffled by the cell, so that a run repeats and no corner of the screen is favored. A pixel's ray calls by its number (under HITS), an entry's
 *  own ray by the entry's key and the top bit (its slot would not do: that is where a race put it) */
fn callers(id: u32, key: u32) -> u32 { return id ^ (pcg(key) & select(HITS - 1u, 0x7fffffffu, id >= 0x80000000u)); }
fn cacheFound(key: u32, id: u32) {
  let w = callers(id, key); var s = key % SLOTS; var i = 0u; if (w == NOTHING) { return; }
  for (var tries = 0u; tries < 2u * REACH && i < REACH; tries++) {
    let k = atomicLoad(&keys[s]); if (k == key) { atomicMin(&cand[s], w); return; }
    if (k == 0u) { let r = atomicCompareExchangeWeak(&keys[s], 0u, key); if (r.exchanged || r.old_value == key) { atomicMin(&cand[s], w); return; } if (r.old_value == 0u) { continue; } }   // (a weak exchange may fail for nothing: the same slot again)
    s = (s + 1u) % SLOTS; i++;
  }
}
/** a ray landed at x: the entry for that place is founded, or asked to move there. `at`: where in hits it may leave word; `id`: who calls */
fn tellAs(at: u32, id: u32, x: vec3f, n: vec3f) { let e = cacheEntry(x, n); if (e.x == NOTHING && e.y >= TRIES) { return; } hits[at] = vec4u(bitcast<vec3u>(x), (octPack(n) & 0xfffffffcu) | select(1u, 3u, askMoves)); cacheFound(cacheKey(x, n, e.y), id); }
fn cacheTell(who: u32, x: vec3f, n: vec3f) { if (who < HITS) { tellAs(who, who, x, n); } }
