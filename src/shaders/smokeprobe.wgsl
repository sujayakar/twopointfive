// ===========================================================================
// Coarse smoke-density readback for gameplay (guard line-of-sight, the
// player's visibility). Box-averages the smoke density volume down to a
// small grid the CPU can afford to map back asynchronously — the same lag-
// tolerant pattern as the light probe. Smoke only: the ambient fog noise
// is not something anyone hides in, and it is a shader function anyway.
// ===========================================================================

struct SmokeProbeParams {
  /** Coarse grid dims (x, y, z). */
  dims   : vec3u,
  /** Fine cells per coarse cell along x and z (y is kept at full resolution). */
  factor : u32,
}

@group(1) @binding(0) var<storage, read_write> coarseOut : array<f32>;
@group(1) @binding(1) var<uniform> SP : SmokeProbeParams;
@group(1) @binding(2) var smokeIn : texture_3d<f32>;

@compute @workgroup_size(4, 4, 4)
fn main(@builtin(global_invocation_id) gid: vec3u) {
  if (gid.x >= SP.dims.x || gid.y >= SP.dims.y || gid.z >= SP.dims.z) { return; }
  let f = i32(SP.factor);
  var sum = 0.0;
  for (var dz = 0; dz < f; dz = dz + 1) {
    for (var dx = 0; dx < f; dx = dx + 1) {
      let c = vec3i(i32(gid.x) * f + dx, i32(gid.y), i32(gid.z) * f + dz);
      sum = sum + textureLoad(smokeIn, c, 0).r;
    }
  }
  coarseOut[(gid.z * SP.dims.y + gid.y) * SP.dims.x + gid.x] = sum / f32(f * f);
}
