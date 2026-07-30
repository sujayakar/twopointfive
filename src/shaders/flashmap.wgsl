// ===========================================================================
// Torch depth maps.
//
// One FLASHMAP_RES^2 layer of traced depth per spot light: layer 0 is the
// player's flashlight, layers 1..TORCH_LAYERS-1 are the dynamic spot lights
// (guard torches) in light-array order. Rebuilt every frame — the lights
// never stop moving and all layers together are ~130k rays, a rounding error
// next to the image trace.
//
// Consumers (in pathtrace.wgsl):
//   - flashVisP: the ReSTIR target function multiplies the player-flashlight
//     candidates by a PCF lookup of layer 0. Unbiased — the survivor still
//     gets a real shadow ray.
//   - flashVisVol: the volumetric march reads a light's layer instead of
//     firing a shadow ray per step, which was the march's dominant cost —
//     rays inside a beam are unoccluded by definition and walk the whole BVH.
//     Guard beams measured +3.7ms of march at five guards before this.
//
// The parameterisation is a pinhole projection along the light's axis with
// its outer cone as the fov, depth stored as radial distance — reader and
// writer share onb() and tanFromCos() so they cannot disagree.
// ===========================================================================

@group(1) @binding(0) var flashDepthOut : texture_storage_2d_array<r32float, write>;

@compute @workgroup_size(8, 8, 1)
fn main(@builtin(global_invocation_id) gid: vec3u) {
  if (gid.x >= u32(FLASHMAP_RES) || gid.y >= u32(FLASHMAP_RES)) { return; }
  let layer = gid.z;

  var pos = vec3f(0.0);
  var axis = vec3f(0.0, 1.0, 0.0);
  var cosOuter = 0.0;
  var alive = false;
  if (layer == 0u) {
    pos = U.flashPos;
    axis = U.flashDir;
    cosOuter = U.flashCosOuter;
    alive = U.flashIntensity > 0.0;
  } else {
    let li = U.dynLightStart + (layer - 1u);
    if (li < U.transientStart) {
      let l = lights[li];
      pos = l.pos;
      axis = l.dir;
      cosOuter = l.cosOuter;
      alive = l.kind == LIGHT_SPOT && l.intensity > 0.0;
    }
  }

  // Dead or absent light: write "nothing occludes" so a stale layer cannot
  // shadow the first frame after a torch switches back on.
  if (!alive) {
    textureStore(flashDepthOut, gid.xy, layer, vec4f(RAY_MAX, 0.0, 0.0, 0.0));
    return;
  }

  let basis = onb(axis);
  let uv = ((vec2f(gid.xy) + 0.5) / f32(FLASHMAP_RES)) * 2.0 - 1.0;
  let rd = normalize(basis * vec3f(uv * tanFromCos(cosOuter), 1.0));

  // skipEmissive, like occluded(): the torch's own lens box sits on top of
  // the light position and would otherwise fill the whole layer with depth ~0.
  let h = trace(pos, rd, RAY_MAX, false, true);
  textureStore(flashDepthOut, gid.xy, layer, vec4f(select(RAY_MAX, h.t, h.valid), 0.0, 0.0, 0.0));
}
