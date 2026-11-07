// Tile mask for the shadow client's pass-through (render/denoise.ts, shaders/denoise.wgsl; FLAG_SOFT_TILESKIP). One tiny dispatch a frame on the client's own
// 8x8 tile grid, between its pack (which writes the per-tile hint flag: does any pixel of the tile carry a penumbra hint in either emitter class?) and its
// à-trous chain:
//   `tileDistances` — one invocation per tile: Chebyshev distance d, in tiles, to the nearest marked tile (0 = itself), scanning SCAN_R =
//                     SKIP_T - 1 rings (so d saturates at SKIP_T, "at least that far"), stored as  d | (256 if d >= SKIP_T).
// Bit 256 is the whole decision: the chain passes such a tile through — copies input to output — at EVERY step. SKIP_T is the chain-wide
// threshold from passes.ts (skipThreshold of the chain's total stride sum); it and SCAN_R are prepended as consts by the host, one small
// module per threshold the iteration count can produce — no uniforms, no overrides, and no scene prelude either: nothing here touches group 0.
@group(1) @binding(3) var tileFlagIn: texture_2d<u32>;
@group(1) @binding(4) var tileDistOut: texture_storage_2d<r32uint, write>;   // same grid: d | (256 if d >= SKIP_T)

@compute @workgroup_size(8, 8)
fn tileDistances(@builtin(global_invocation_id) gid: vec3u) {
  // one invocation per tile. Tiles off the grid contain no pixels at all, so they never count as flagged; (2·SCAN_R+1)² u32 taps of a
  // texture a few hundred texels across — nothing next to one chain pass.
  let td = textureDimensions(tileFlagIn);
  if (gid.x >= td.x || gid.y >= td.y) { return; }
  let t = vec2i(gid.xy); let thi = vec2i(td) - 1;
  var d: i32 = SKIP_T;
  for (var dy = -SCAN_R; dy <= SCAN_R; dy++) {
    for (var dx = -SCAN_R; dx <= SCAN_R; dx++) {
      let q = t + vec2i(dx, dy);
      if (any(q < vec2i(0)) || any(q > thi)) { continue; }
      if (textureLoad(tileFlagIn, q, 0).x != 0u) { d = min(d, max(abs(dx), abs(dy))); }
    }
  }
  textureStore(tileDistOut, t, vec4u(u32(d) | select(0u, 256u, d >= SKIP_T), 0u, 0u, 0u));
}
