// ===========================================================================
// Re-modulates the denoised illumination by albedo, applies the medium's
// transmittance, and adds its in-scattered radiance, producing the HDR image
// that bloom and tonemapping consume.
// ===========================================================================

struct CompositeParams {
  debugView  : u32,
  /** 0 when no transient light exists, so its stale texture is ignored. */
  transientOn: f32,
  _pad0      : u32,
  _pad1      : u32,
}

@group(0) @binding(0) var<uniform> C : CompositeParams;
@group(0) @binding(1) var illumTex : texture_2d<f32>;
@group(0) @binding(2) var albedoTex : texture_2d<f32>;
@group(0) @binding(3) var normalDepthTex : texture_2d<f32>;
@group(0) @binding(4) var momentsTex : texture_2d<f32>;
@group(0) @binding(5) var rawTex : texture_2d<f32>;
@group(0) @binding(6) var hdrOut : texture_storage_2d<rgba16float, write>;
/** Bounce light, denoised separately and far more aggressively. */
@group(0) @binding(7) var indirectTex : texture_2d<f32>;
/** Muzzle flashes and bursts. Spatially filtered, never accumulated. */
@group(0) @binding(8) var transientTex : texture_2d<f32>;
/**
 * The medium: rgb is the denoised in-scattered radiance along the pixel's
 * ray, a is the transmittance from the camera to the surface behind it.
 */
@group(0) @binding(9) var volumeTex : texture_2d<f32>;

@compute @workgroup_size(8, 8, 1)
fn main(@builtin(global_invocation_id) gid: vec3u) {
  let dims = vec2i(textureDimensions(illumTex));
  let pixel = vec2i(gid.xy);
  if (pixel.x >= dims.x || pixel.y >= dims.y) { return; }

  let illum = textureLoad(illumTex, pixel, 0);
  let indirect = textureLoad(indirectTex, pixel, 0).rgb;
  let albedo = textureLoad(albedoTex, pixel, 0).rgb;
  let vol = textureLoad(volumeTex, pixel, 0);

  // The surface signals were demodulated by albedo before filtering, so they
  // sum before remodulation, not after.
  let transient = textureLoad(transientTex, pixel, 0).rgb * C.transientOn;
  var color = (illum.rgb + indirect + transient) * albedo;
  // The medium sits between the surface and the eye: it dims what is behind
  // it by its transmittance and adds what it scatters toward the camera.
  color = color * clamp(vol.a, 0.0, 1.0) + max(vol.rgb, vec3f(0.0));

  switch (C.debugView) {
    case 1u: { color = albedo; }
    case 2u: { color = textureLoad(normalDepthTex, pixel, 0).xyz * 0.5 + vec3f(0.5); }
    case 3u: {
      // Variance (red) over history length (green).
      let m = textureLoad(momentsTex, pixel, 0);
      color = vec3f(sqrt(max(m.w, 0.0)), m.z / 48.0, 0.0);
    }
    case 4u: {
      let raw = textureLoad(rawTex, pixel, 0);
      color = raw.rgb * albedo;
    }
    // Indirect only, and direct only — the pair that makes it obvious which
    // signal a given piece of noise belongs to.
    case 5u: { color = indirect * albedo; }
    case 6u: { color = illum.rgb * albedo; }
    case 7u: { color = transient * albedo; }
    // The medium alone: what it scatters, and how much it lets through.
    case 8u: { color = max(vol.rgb, vec3f(0.0)); }
    case 9u: { color = vec3f(clamp(vol.a, 0.0, 1.0)); }
    default: {}
  }

  textureStore(hdrOut, pixel, vec4f(color, 1.0));
}
