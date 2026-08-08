// Half-resolution (a third with the lossy gatherThird option: frame.lossyCfg.x is the divisor) stochastic final gather. Per pixel: K cosine-distributed rays traced for real through the box grid
// (+ character capsules); at each hit the outgoing radiance is albedo/pi x (steady direct light, one importance-picked
// shadow ray + the exact unshadowed rest at half weight for the dim tail) + the cached multi-bounce irradiance (dice
// volume, read one bounce away from the eye where its coarseness cannot be seen) + emission; misses take the sky.
// Output is irradiance E at the primary hit (albedo applied in composite). History-accumulated + à-trous filtered
// downstream; transient lights never enter (they light the first hit directly through the history-free direct pass).
@group(1) @binding(0) var gDepth: texture_depth_2d;
@group(1) @binding(1) var gNormal: texture_2d<f32>;
@group(1) @binding(2) var gId: texture_2d<u32>;
@group(1) @binding(3) var directTex: texture_2d<f32>;    // denoised on-screen direct (sharp class) — reused when a gather ray lands on a visible surface
@group(1) @binding(4) var softTex: texture_2d<f32>;      // (broad class)
@group(1) @binding(5) var diceTex: texture_3d<f32>;
@group(1) @binding(6) var outIndirect: texture_storage_2d<rgba16float, write>;   // rgb = E estimate (a unused: temporal overwrites it with view depth)

override RAYS: u32 = 4u;
const TMAX: f32 = 40.0;

// every hit walks the light list (transient subtraction / off-screen estimate): stage it per 8x8 tile
var<workgroup> wgLights: array<Light, 64>;
var<private> nL: u32;

var<private> rngState: u32;
fn rngInit(p: vec2u, f: u32) { rngState = (p.x * 1973u + p.y * 9277u + f * 26699u) | 1u; rngState = rngState ^ (rngState >> 15u); rngState *= 0x2c1b3c6du; rngState ^= rngState >> 12u; }
fn rnd() -> f32 { rngState = rngState * 747796405u + 2891336453u; var w = ((rngState >> ((rngState >> 28u) + 4u)) ^ rngState) * 277803737u; w = (w >> 22u) ^ w; return f32(w) * (1.0 / 4294967296.0); }

// On-screen shadowed direct light if the point is visible this frame (exact, already denoised), else a one-ray estimate.
fn directAtPoint(P: vec3f, N: vec3f) -> vec3f {
  let clip = frame.viewProj * vec4f(P, 1.0);
  if (clip.w > 0.0) {
    let ndc = clip.xyz / clip.w;
    let uv = ndc.xy * vec2f(0.5, -0.5) + 0.5;
    if (all(uv > vec2f(0.0)) && all(uv < vec2f(1.0))) {
      let px = vec2u(uv * frame.screen);
      let d = textureLoad(gDepth, px, 0);
      if (d < 1.0) {
        let Ps = worldFromDepth(vec2f(px) + 0.5, d);
        let Ns = textureLoad(gNormal, px, 0).xyz * 2.0 - 1.0;
        if (distance(Ps, P) < 0.12 && dot(Ns, N) > 0.7) {
          var Ed = textureLoad(directTex, px, 0).xyz + textureLoad(softTex, px, 0).xyz;
          // the on-screen direct term includes transient lights (muzzle flashes); this estimate feeds accumulated history,
          // so take their (unshadowed) share back out — a flash must never outlive itself as bounce
          for (var i = 0u; i < select(0u, nL, frame.rcJitter.w > 0.0); i++) {   // (skipped outright when no transient light is alive — the common case)
            let li = wgLights[i];
            if (!lightTransient(li)) { continue; }
            let ev = evalLight(li, P, N);
            // only what the flash actually delivered here is in the texture: a hit in the flash's shadow keeps its steady light
            if (ev.ok && !occluded(P + N * 0.01, ev.L, max(ev.dist.x - li.radius - 0.02, 0.0), (li.owner & 0xffu) << 8u)) { Ed -= ev.E; }   // same skip rule the direct pass used when it PUT the flash there (its carrier never shadows it), or the subtraction misses in the shooter's wedge
          }
          return max(Ed, vec3f(0.0));
        }
      }
    }
  }
  // off-screen hit: score every steady light unshadowed, spend ONE shadow ray on an importance-picked light and apply
  // its visibility to the whole sum (exact when one light dominates, which is the common case at a bounce point)
  var U = vec3f(0.0); var W = 0.0; var pick = -1; var pw = 0.0; var pL = vec3f(0.0); var pDist = 0.0; var pRad = 0.0;
  let Po = P + N * 0.01;
  for (var i = 0u; i < nL; i++) {
    let li = wgLights[i];
    if (lightTransient(li)) { continue; }
    let ev = evalLight(li, P, N);
    if (!ev.ok) { continue; }
    let w = luminance(ev.E);
    if (w < 1e-4) { continue; }
    U += ev.E; W += w;
    if (rnd() * W < w) { pick = i32(i); pw = w; pL = ev.L; pDist = ev.dist.x; pRad = li.radius; }
  }
  if (pick < 0) { return vec3f(0.0); }
  let pk = wgLights[u32(pick)];
  let tmax = select(pDist - pRad - 0.02, 60.0, lightKind(pk) == 2u);
  var vis = 1.0;
  if (tmax > 0.0 && occluded(Po, pL, tmax, (pk.owner & 0xffu) << 8u)) { vis = 0.0; }
  return U * vis;
}

fn cosineDir(N: vec3f) -> vec3f {
  let up = select(vec3f(0.0, 1.0, 0.0), vec3f(1.0, 0.0, 0.0), abs(N.y) > 0.9);
  let T = normalize(cross(up, N)); let B = cross(N, T);
  let r = sqrt(rnd()); let a = rnd() * 6.2831853;
  return normalize(T * (cos(a) * r) + B * (sin(a) * r) + N * sqrt(max(1.0 - r * r, 0.0)));
}

@compute @workgroup_size(8, 8)
fn main(@builtin(global_invocation_id) gid: vec3u, @builtin(local_invocation_index) lidx: u32) {
  nL = min(frame.numLights, 64u);
  for (var i = lidx; i < nL; i += 64u) { wgLights[i] = lights[i]; }
  workgroupBarrier();
  let odims = textureDimensions(outIndirect);
  if (gid.x >= odims.x || gid.y >= odims.y) { return; }
  let gdiv = u32(frame.lossyCfg.x);                                            // this pass runs at 1/gdiv resolution: 2, or 3 (lossy option)
  let full = min(gid.xy * gdiv + (gdiv - 1u) / 2u, vec2u(frame.screen) - 1u);   // the full-res pixel a texel is computed at: top-left of its 2x2 as always (the min never binds then), the centre of its 3x3
  let depth = textureLoad(gDepth, full, 0);
  if (depth >= 1.0 || (frame.flags & FLAG_INDIRECT) == 0u) { textureStore(outIndirect, gid.xy, vec4f(0.0)); return; }
  rngInit(gid.xy, frame.frameIdx);
  let P = worldFromDepth(vec2f(full) + 0.5, depth);
  let N = normalize(textureLoad(gNormal, full, 0).xyz * 2.0 - 1.0);
  let id = textureLoad(gId, full, 0).x;
  let owner = (id >> 20u) & 0xffu;
  let Po = P + N * 0.02;
  var E = vec3f(0.0);
  for (var k = 0u; k < RAYS; k++) {
    let dir = cosineDir(N);
    let hit = traceClosest(Po, dir, TMAX, owner);
    var L = vec3f(0.0);
    if (hit.idx < 0) { L = skyRadiance(dir); }
    else if (!hit.inside) {
      let Ph = Po + dir * hit.t;
      var albedo: vec3f;
      if (hit.idx == CAPSULE_HIT) { albedo = hit.capAlbedo; }
      else {
        let bi = u32(hit.idx);
        if ((boxGeo[bi].flags & (BOX_EMISSIVE | BOX_AREALIT)) == BOX_EMISSIVE) { L += boxEmissive(bi); }
        albedo = boxAlbedo(bi);
      }
      var Eh = vec3f(0.0);
      if ((frame.flags & FLAG_DIRECT) != 0u) { Eh += directAtPoint(Ph, hit.n); }
      if ((frame.flags & FLAG_BOUNCE) != 0u) { Eh += diceIrradiance(Ph + hit.n * c0Spacing(), hit.n); }   // one probe spacing off the surface (per axis) like every other dice read: sampling closer pulls light through off-lattice walls
      L += albedo * Eh * (1.0 / 3.14159265);
    }
    // firefly guard: one ray in K finding a small intensely lit patch is an outlier the filters would smear into a blotch
    // (generous cap: a desk-lamp pool or a torch spot seen from the wall next to it is a LARGE, legitimately bright patch, not a
    // firefly — clamping it hard is what made small bright lights cast no bounce)
    let lum = luminance(L);
    L = select(L, L * (24.0 / lum), lum > 24.0);
    E += L;
  }
  // cosine-weighted pdf = cos/pi  ⇒  E = pi/K · Σ L
  E *= 3.14159265 / f32(RAYS);
  textureStore(outIndirect, gid.xy, vec4f(E * frame.indirectScale, 1.0));
}
