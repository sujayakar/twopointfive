@compute @workgroup_size(8, 8) fn main(@builtin(global_invocation_id) id: vec3u) {
  let dim = vec2u(frame.size.zw); if (id.x >= dim.x || id.y >= dim.y) { return; }
  let at = id.y * dim.x + id.x; rngInit(id.xy, 11u + pathKnobs.x * 7919u); qSeed = pcg(id.x + pcg(id.y + pcg(frame.counts.w + 991u))); qIndex = pathKnobs.x; qDim = 0u;
  let j = rnd2(); let L = walk(frame.camPos.xyz, camRay((vec2f(id.xy) + j) / vec2f(dim)), 0u, true, frame.misc.y, length(frame.camUp.xyz) * 2.0 / frame.size.w, 0.0, pathKnobs.y);
  var acc = vec4f(0.0); if (pathKnobs.z > 0u) { acc = sums[at]; }
  acc += vec4f(max(finite4(vec4f(L, 0.0), 1e6).rgb, vec3f(0.0)), 1.0); sums[at] = acc;
  textureStore(outHdr, id.xy, vec4f(acc.xyz / acc.w, 1.0));
}
