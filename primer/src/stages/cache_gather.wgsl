// One more sample into each entry whose turn it was. Its ray's hit is asked of this same table as it stood last
// frame: that is how one bounce a frame becomes all of them.

fn run(slot: u32) {
  var e = cache_before[slot];
  let p = cache_points[slot];
  if (dot(p.normal, p.normal) == 0.0) { cache[slot] = Entry(); return; }
  let asked = cache_direct[slot];
  if (asked.seen >= 0.0 && frame.cache_go != 0u) {
    let y = cache_hits[slot];
    var bounced = vec3f(0.0);
    if (y.distance >= 0.0 && y.lamp == NOTHING) {
      let spot = cache_spot(y.position, y.normal);
      if (spot.slot != NOTHING && (spot.slot != slot || knobs.break_reads_itself != 0u)) {
        let there = cache_before[spot.slot];
        bounced = y.sends * (there.direct + there.known) * y.weight;
      }
    } else if (y.distance < 0.0 && y.distance != NO_RAY) {
      bounced = sky(y.toward) * y.weight;
    }
    let n = min(e.samples + 1.0, f32(knobs.cache_memory));
    // later samples may count for more: the first ones read a table that was still dark
    var k = 1.0 / n;
    if (knobs.cache_late != 0u && knobs.cache_mode == CACHE_MODE_RUNNING) { k = min(1.0, 2.0 / (n + 1.0)); }
    if (!is_bad(bounced.x + bounced.y + bounced.z)) { e.bounced = mix(e.bounced, bounced, k); }
    if (!is_bad(asked.lands.x + asked.lands.y + asked.lands.z)) { e.direct = mix(e.direct, asked.lands, 1.0 / min(e.direct_samples + 1.0, f32(knobs.cache_memory))); }
    e.samples = n;
    e.direct_samples = min(e.direct_samples + 1.0, f32(knobs.cache_memory));
  }
  if (knobs.cache_mode == CACHE_MODE_RUNNING) {
    e.known = e.bounced;
    if (e.samples > 0.0) { e.ready = 1.0; }
  } else if (frame.cache_commit != 0u && e.samples > 0.0) {
    // one whole bounce is in: let it be read, and gather the next one from nothing
    e.known = e.bounced;
    e.ready = 1.0;
    e.bounced = vec3f(0.0);
    e.samples = 0.0;
  }
  cache[slot] = e;
}
