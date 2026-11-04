// THE LOOK PROTOTYPE'S STEADY BOUNCE (render/diceRing.ts): the radiance cache's dice as the mean of its last bake at each of the frame's jitter points.
// `ring` holds those bakes one under another along y (slot k at rows k * cfg.rows and on); bit k of cfg.mask says slot k holds a bake of this cache as it
// stands. Every texel of the dice is written with the mean of the slots that do.
struct RingCfg { n: u32, mask: u32, rows: u32, pad: u32 };
@group(0) @binding(0) var<uniform> cfg: RingCfg;
@group(0) @binding(1) var ring: texture_3d<f32>;
@group(0) @binding(2) var outDice: texture_storage_3d<rgba16float, write>;

@compute @workgroup_size(4, 4, 4)
fn ringMean(@builtin(global_invocation_id) gid: vec3u) {
  let dims = textureDimensions(outDice);
  if (gid.x >= dims.x || gid.y >= dims.y || gid.z >= dims.z) { return; }
  var sum = vec4f(0.0); var got = 0.0;   // (all four numbers: the +y slab's fourth is what a portal took out of that face, which the place field adds back, rc.wgsl rcDice)
  for (var k = 0u; k < cfg.n; k++) {
    if (((cfg.mask >> k) & 1u) == 0u) { continue; }
    sum += textureLoad(ring, vec3u(gid.x, gid.y + k * cfg.rows, gid.z), 0); got += 1.0;
  }
  textureStore(outDice, gid, sum / max(got, 1.0));
}
