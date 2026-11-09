// a key founded this frame takes the point of whoever founded it; one that roams moves to where a ray just landed, if that is joined to where it began
@compute @workgroup_size(64) fn main(@builtin(global_invocation_id) id: vec3u) {
  let s = id.x + id.y * LANES; if (s >= SLOTS) { return; } let w = atomicLoad(&cand[s]); if (w == NOTHING) { return; } atomicStore(&cand[s], NOTHING); let h = hits[shuffled(w, atomicLoad(&keys[s]))]; let x = bitcast<vec3f>(h.xyz); let p = select(h.w | 1u, EVERY_WAY, h.w == EVERY_WAY);
ROAMS{  if (bitcast<u32>(spot[s].w) == 0u) { fresh[2u * s + 1u] = vec4f(x, 0.0); spot[s] = vec4f(x, bitcast<f32>(p)); return; } let a = cells[2u * s + 1u].xyz; let n = unOct(p); if (!cacheNear(x) || sees(a, x, n)) { spot[s] = vec4f(x, bitcast<f32>(p)); }
}ELSE{  if (cells[2u * s].w == 0.0 && norms[s] == 0u) { fresh[2u * s + 1u] = vec4f(x, 0.0); norms[s] = p; }
}END}
