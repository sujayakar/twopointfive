@compute @workgroup_size(64) fn main(@builtin(global_invocation_id) id: vec3u) {
  let s = id.x + id.y * LANES; if (s >= SLOTS) { return; } let w = atomicLoad(&cand[s]); if (w == NOTHING) { return; } atomicStore(&cand[s], NOTHING); if (any(pointOf(s) != vec3f(0.0))) { return; }
  let x = bitcast<vec3f>(hits[shuffled(w, atomicLoad(&keys[s]))].xyz); fresh[5u * s + 1u].w = x.x; fresh[5u * s + 2u].w = x.y; fresh[5u * s + 3u].w = x.z;
}
