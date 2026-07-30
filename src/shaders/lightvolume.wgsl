// ===========================================================================
// Static light volume.
//
// A 3D grid over the room's air holding the radiance in-scattered toward the
// camera per unit scattering coefficient, from every STATIC light: for each
// voxel centre, the sum over static lights of intensity * colour * falloff *
// spot * traced visibility, folded through the isotropic phase 1/(4 pi).
// Moonlight through the window holes lands here as the god-ray pools; the
// practicals add their soft coloured haze. Baked once at init and again
// whenever a static intensity changes (the OCP darkening a lamp, a fixture
// shot out) — a whole-volume dispatch is fine at that event rate.
//
// Isotropic phase is deliberate: the volume stores a single ambient sum, so
// there is no per-light direction left to feed an anisotropic phase. Only
// the static scene shadows the bake (occludedStatic) — characters must not be
// frozen into it — and dynamic density is ignored: dense smoke does not dim
// what the static lights scatter, only what the torches do (the march
// attenuates those). The trace pass samples this trilinearly; see the
// contract at the top of pathtrace.wgsl's volumetric section.
// ===========================================================================

struct LightVolParams {
  /** World position of the grid's minimum corner. */
  origin : vec3f,
  /** Static light count: lights[0..count) contribute. */
  count  : u32,
  /** Metres per cell, per axis. */
  cell   : vec3f,
  /** Visibility rays per light per voxel, jittered over the emitter. */
  rays   : u32,
  dims   : vec3u,
  _pad   : u32,
}

@group(1) @binding(0) var<uniform> LV : LightVolParams;
@group(1) @binding(1) var volOut : texture_storage_3d<rgba16float, write>;
/** Work counters — the bake reports only its shadow-ray count. */
@group(1) @binding(2) var<storage, read_write> counters : array<atomic<u32>>;

@compute @workgroup_size(4, 4, 4)
fn bake(@builtin(global_invocation_id) gid: vec3u) {
  if (gid.x >= LV.dims.x || gid.y >= LV.dims.y || gid.z >= LV.dims.z) { return; }
  seedRng(gid.xy, gid.z * 7919u + 13u);
  let p = LV.origin + (vec3f(gid) + 0.5) * LV.cell;

  var e = vec3f(0.0);
  var rays = 0u;
  for (var li = 0u; li < LV.count; li = li + 1u) {
    let l = lights[li];
    if (l.intensity <= 0.0) { continue; }
    // Contribution cull before any ray: an unshadowed practical whose light
    // rounds away at this voxel is not worth its visibility rays.
    let dp = l.pos - p;
    if (luminance(l.color * (l.intensity * falloff(dot(dp, dp)))) < 1e-4) { continue; }

    var acc = vec3f(0.0);
    for (var s = 0u; s < LV.rays; s = s + 1u) {
      // Jittered over the emitter, so the pool edges the moon throws through
      // the windows come out soft rather than voxel-stepped.
      let smp = sampleSphereLight(l, p);
      var atten = 1.0;
      if (l.kind == LIGHT_SPOT) {
        atten = spotAttenuation(l.dir, -smp.dir, l.cosInner, l.cosOuter);
        if (atten <= 0.0) { continue; }
      }
      rays = rays + 1u;
      if (occludedStatic(p, smp.dir, smp.dist - EPS * 8.0)) { continue; }
      acc = acc + smp.radiance * atten;
    }
    e = e + acc / f32(LV.rays);
  }

  if (U.countersOn > 0.5 && rays > 0u) { atomicAdd(&counters[CT_lightVolBakeRays], rays); }
  textureStore(volOut, gid, vec4f(e * (1.0 / (4.0 * PI)), 1.0));
}
