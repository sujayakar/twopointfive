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
//
// Each layer skips its owner's dynamic group, the way the gameplay probe
// skips its own body: a pose that tucks the weapon or a limb over the lens
// (the crouch idle does exactly this with the pistol slide, ~2.5 cm) must
// not fill the layer with its owner's own geometry, or every consumer of
// the map (radiosity inject, the RIS target, the beam march) reads the
// torch as boxed in. The owner never shadowing its own beam in the map is
// the price; the exact shadow rays on the direct signal still see the body.
// ===========================================================================

/**
 * Owning dynamic group per layer, or DYN_GROUP_NONE. A table rather than a
 * derivation from the light index: layer k is the k-th LIVE torch, but a
 * dead guard keeps its packed body (a corpse to drag), so light order and
 * group order diverge as soon as anyone is down — only the game knows the
 * pairing. Written per frame by renderer.setTorchGroups.
 */
struct FlashmapParams {
  ownerGroup : array<vec4u, 2>,
}

@group(1) @binding(0) var flashDepthOut : texture_storage_2d_array<r32float, write>;
/**
 * Work counters. This pass flushes only its ray count: its BVH traversal is
 * a fixed function of that count (128^2 per live layer), and folding it into
 * the node-visit/box-test slots would break their per-image-pixel meaning.
 */
@group(1) @binding(1) var<storage, read_write> counters : array<atomic<u32>>;
@group(1) @binding(2) var<uniform> FP : FlashmapParams;

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
  let owner = FP.ownerGroup[layer / 4u][layer % 4u];
  let h = traceSkipping(pos, rd, RAY_MAX, false, true, owner);
  if (U.countersOn > 0.5) { atomicAdd(&counters[CT_flashmapRays], 1u); }
  textureStore(flashDepthOut, gid.xy, layer, vec4f(select(RAY_MAX, h.t, h.valid), 0.0, 0.0, 0.0));
}
