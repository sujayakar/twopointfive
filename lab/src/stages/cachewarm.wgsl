// every standing face of the level calls for its patches, before anything has been seen
@compute @workgroup_size(64) fn main(@builtin(global_invocation_id) id: vec3u) { let i = id.x; if (i + warmAt.x >= warmAt.y) { return; } let h = warmPts[i]; cacheTell(i, bitcast<vec3f>(h.xyz), octUnpack(h.w)); }
