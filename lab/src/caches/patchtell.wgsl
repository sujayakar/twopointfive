fn tellOne(who: u32, x: vec3f, n: vec3f, level: u32) {
  let e = cacheEntry(x, n, level); if (e.x == NOTHING && e.y > 1u) { return; }
  if (e.x != NOTHING && !ROAM && any(cells[2u * e.x + 1u].xyz != vec3f(0.0))) { return; }
  hits[who] = vec4u(bitcast<vec3u>(x), octPack(n)); cacheFound(cacheKeyOf(x, n, level, e.y), who);
}
fn cacheTell(who: u32, x: vec3f, n: vec3f, moves: bool) {
  if (who >= HITS + SLOTS) { return; } if (!cacheNear(x)) { tellOne(who, x, n, 0u); return; }
  if (LEVELS > 1u) { tellOne(who, x, n, select(askLevel, min(askLevel + 2u, LEVELS - 1u), moves)); return; }
  for (var l = select(0u, levelMost() - 1u, moves); l < levelMost(); l++) { tellOne(who, x, n, l); }
}
