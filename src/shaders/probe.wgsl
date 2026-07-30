// ===========================================================================
// Light probe.
//
// Answers "how lit is this point, really" for gameplay, using the same light
// list, the same BVH and the same occlusion test the image is rendered with.
//
// Doing this on the CPU would mean a second ray traversal that could disagree
// with the renderer — a shadow you can see but that the game does not believe
// in, which is exactly the bug a stealth game cannot afford. Asking the GPU
// costs one thread per probe and cannot drift.
//
// The result is read back asynchronously, so it lags by a frame or two. That is
// fine for a suspicion meter and is not fine for anything that needs to be
// frame-exact.
// ===========================================================================

const MAX_PROBES: u32 = 4u;
/** Beyond this the beam's return off a wall is negligible next to the practicals. */
const FLASH_BACKSCATTER_RANGE: f32 = 30.0;

struct ProbeParams {
  /** xyz is the world point; w is unused. */
  points : array<vec4f, MAX_PROBES>,
  count  : u32,
  /**
   * Dynamic-box group belonging to whoever this probe is measuring, skipped by
   * the shadow test. A chest-height probe sits inside its own character's torso
   * box, so without this every light reads as blocked, everywhere, always.
   */
  skipGroup : u32,
  _pad0  : u32,
  _pad1  : u32,
}

@group(1) @binding(0) var<storage, read_write> probeOut : array<vec4f>;
@group(1) @binding(1) var<uniform> P : ProbeParams;

/**
 * The player's own flashlight, which lives in the uniforms rather than in
 * lights[] and so is invisible to the loop below. In Splinter Cell your own
 * light very much gives you away, and the guards here score the same meter
 * this feeds — so a torch that only ever helped you would be a lie the whole
 * detection model inherits.
 *
 * Two terms. The direct term is the cone reaching back to the body itself
 * (near zero when aiming away, real when the lens sweeps across you). The
 * backscatter term is the one that matters in play: the lit patch at the end of
 * the beam is a diffuse secondary source, so standing square to a wall a metre
 * away bathes you in your own light. One trace along the beam axis stands in
 * for it — a single tap, so it under-counts wide spill, but it moves the meter
 * the right way with the right dependencies (albedo, distance, cone).
 */
fn flashlightExposure(p: vec3f) -> vec3f {
  var e = vec3f(0.0);
  let toProbe = p - U.flashPos;
  let dp = length(toProbe);
  if (dp > 1e-3) {
    let dir = toProbe / dp;
    let cone = spotAttenuation(U.flashDir, dir, U.flashCosInner, U.flashCosOuter);
    if (cone > 0.001 && !occludedSkipping(p, -dir, dp - EPS * 8.0, P.skipGroup)) {
      e = e + U.flashColor * (U.flashIntensity * cone * falloff(dp * dp));
    }
  }
  let h = trace(U.flashPos, U.flashDir, FLASH_BACKSCATTER_RANGE, false, true);
  if (h.valid) {
    let hitP = h.p + h.n * EPS * 4.0;
    let dHit = max(h.t, 1e-3);
    // Irradiance the beam centre lands on the patch, thrown back off a
    // diffuse surface toward the body: albedo/pi times the patch's solid
    // angle, approximated by the beam's own spread over the return leg.
    let irr = U.flashIntensity * falloff(dHit * dHit);
    let m = materials[h.mat];
    let back = p - hitP;
    let dr = max(length(back), 1e-3);
    let nDotB = max(dot(h.n, back / dr), 0.0);
    let spread = tanFromCos(U.flashCosOuter);
    let solidAngle = min((spread * dHit) * (spread * dHit) / (dr * dr), 1.0);
    if (nDotB > 0.0 && !occludedSkipping(p, -back / dr, dr - EPS * 8.0, P.skipGroup)) {
      e = e + U.flashColor * irr * m.albedo * (INV_PI * nDotB * solidAngle);
    }
  }
  return e;
}

/**
 * Illuminance arriving at a point from every scene light, plus your own torch.
 *
 * Deliberately *not* cosine-weighted against a normal: the question is "how
 * much light is falling on this body", and a body is not a plane. A cosine term
 * would make the reading swing wildly as the character turns, which is not what
 * being visible means.
 */
@compute @workgroup_size(MAX_PROBES, 1, 1)
fn main(@builtin(global_invocation_id) gid: vec3u) {
  let i = gid.x;
  if (i >= P.count) { return; }

  let p = P.points[i].xyz;
  seedRng(vec2u(i, 0u), U.frame);

  var total = vec3f(0.0);
  var unoccluded = 0.0;
  var blocked = 0.0;
  for (var l = 0u; l < U.lightCount; l = l + 1u) {
    let light = lights[l];
    let s = sampleSphereLight(light, p);

    var atten = 1.0;
    if (light.kind == LIGHT_SPOT) {
      atten = spotAttenuation(light.dir, -s.dir, light.cosInner, light.cosOuter);
      if (atten <= 0.001) { continue; }
    }

    // One shadow ray per light. The probe count is tiny, so this is a rounding
    // error next to a full frame — but it is the same occluded() the renderer
    // uses, including its FLAG_EMISSIVE skip, so a light the player can see is
    // a light the game agrees they can see.
    unoccluded = unoccluded + luminance(s.radiance) * atten;
    if (occludedSkipping(p, s.dir, s.dist - EPS * 8.0, P.skipGroup)) { blocked = blocked + 1.0; continue; }
    total = total + s.radiance * atten;
  }

  if (U.flashIntensity > 0.0) {
    total = total + flashlightExposure(p);
  }

  // Diagnostic: a 0.3m ray sideways from a mid-air point should hit nothing.
  // If this reads 1, occluded() is rejecting everything and the light sum is
  // meaningless.
  probeOut[i] = vec4f(f32(U.lightCount), unoccluded, blocked, luminance(total));
}
