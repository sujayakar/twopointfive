// ===========================================================================
// Flashlight depth map.
//
// FLASHMAP_RES^2 rays traced from the lens across the spot cone, storing hit
// distance along each ray. Rebuilt every frame — the light never stops moving
// and 16k rays are a rounding error next to the image trace.
//
// Consumers (both in pathtrace.wgsl, both optional via uniforms):
//   - flashVisP:  the ReSTIR target function multiplies flashlight candidates
//     by a PCF lookup, so reservoirs stop wasting weight on samples behind
//     cover. Unbiased — the survivor still gets a real shadow ray.
//   - flashVisVol: the volumetric march reads the map instead of firing a
//     shadow ray per step, which was the march's dominant cost (those rays
//     are unoccluded by definition inside the beam and walk the whole BVH).
//
// The parameterisation is a pinhole projection along flashDir with the outer
// cone as its fov, depth stored as radial distance — reader and writer share
// onb() and flashTanOuter() so they cannot disagree about the mapping.
// ===========================================================================

@group(1) @binding(0) var flashDepthOut : texture_storage_2d<r32float, write>;

@compute @workgroup_size(8, 8, 1)
fn main(@builtin(global_invocation_id) gid: vec3u) {
  if (gid.x >= u32(FLASHMAP_RES) || gid.y >= u32(FLASHMAP_RES)) { return; }

  // Dead light: write "nothing occludes" so a stale map cannot shadow the
  // first frame after the torch switches back on.
  if (U.flashIntensity <= 0.0) {
    textureStore(flashDepthOut, gid.xy, vec4f(RAY_MAX, 0.0, 0.0, 0.0));
    return;
  }

  let basis = onb(U.flashDir);
  let uv = ((vec2f(gid.xy) + 0.5) / f32(FLASHMAP_RES)) * 2.0 - 1.0;
  let local = vec3f(uv * flashTanOuter(), 1.0);
  let rd = normalize(basis * local);

  // skipEmissive, like occluded(): the torch's own lens box sits on top of
  // flashPos and would otherwise fill the whole map with depth ~0.
  let h = trace(U.flashPos, rd, RAY_MAX, false, true);
  textureStore(flashDepthOut, gid.xy, vec4f(select(RAY_MAX, h.t, h.valid), 0.0, 0.0, 0.0));
}
