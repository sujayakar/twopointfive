// what was called for last frame: a new key takes the place of the caller who won it; an old entry moves there, if that is joined to where it began
@compute @workgroup_size(64) fn main(@builtin(global_invocation_id) id: vec3u) {
  let s = id.x + id.y * LANES; if (s >= SLOTS) { return; } let w = atomicLoad(&cand[s]); if (w == NOTHING) { return; } atomicStore(&cand[s], NOTHING); let key = atomicLoad(&keys[s]);
  var at = callers(w, key); if (w >= 0x80000000u) { let asker = cacheFind(at & 0x7fffffffu); if (asker == NOTHING) { return; } at = HITS + asker; }
  let h = hits[at]; let x = bitcast<vec3f>(h.xyz); let n = octUnpack(h.w); let there = vec4u(h.xyz, spotPack(n));
  if (spot[s].w == 0u) { fresh[2u * s + 1u] = vec4f(x, 0.0); spot[s] = there; return; }
  if (speaksFor(cells[2u * s + 1u].xyz, x, n)) { spot[s] = there; }
}
