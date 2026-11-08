// every standing face of the level founds its cells, before anything has been seen
@compute @workgroup_size(64) fn main(@builtin(global_invocation_id) id: vec3u) {
  let i = id.x + warmAt.x; if (i >= warmAt.y) { return; } let h = hits[i - warmAt.x]; let n = octUnpack(bitcast<u32>(h.w)); cacheFound(cacheKey(h.xyz, n, 0u), i - warmAt.x); cacheFound(cacheKey(h.xyz, n, 1u), i - warmAt.x);
}
