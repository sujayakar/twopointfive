/** a place that too little covers asks for a disc: one a bucket a frame */
fn cacheTell(who: u32, x: vec3f, n: vec3f, moves: bool) {
  if (who >= HITS || moves || covered(x, n) > COVER) { return; } let near = cacheNear(x); let key = bucketKey(vec3i(floor(x / cellSize(near))), near); let b = bucketMake(key); if (b == NOTHING) { return; }
  var r = select(FAR_R, RADIUS, near); if (near && askPiece != NOTHING) { let h = pieces[askPiece].half; let mid = h.x + h.y + h.z - max3(h) - min3(h); r = clamp(1.3 * mid, 0.25 * RADIUS, RADIUS); }
  let o = octPack(n); let qr = u32(clamp(r / FAR_R * 255.0 + 0.5, 1.0, 255.0)); hits[who] = vec4u(bitcast<vec3u>(x), (o & 0xfff0fff0u) | (qr & 15u) | ((qr >> 4u) << 16u)); atomicMin(&buckets[b * WIDTH + 1u], who ^ (pcg(key) & 0xfffffu));
}
