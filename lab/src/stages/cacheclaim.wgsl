// a key founded this frame takes the point of whoever founded it (the smallest number, so that a run repeats)
@compute @workgroup_size(64) fn main(@builtin(global_invocation_id) id: vec3u) {
  let s = id.x; if (s >= SLOTS) { return; } let w = atomicLoad(&cand[s]); if (w == NOTHING) { return; } atomicStore(&cand[s], NOTHING);
  if (cells[2u * s].w == 0.0 && norms[s] == 0u) { let h = hits[w]; cells[2u * s + 1u] = vec4f(h.xyz, 0.0); norms[s] = bitcast<u32>(h.w) | 1u; }
}
