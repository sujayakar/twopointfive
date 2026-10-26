// THE GATHER IN TWO KERNELS (render/gatherSplit.ts; Options > Graphics > Separate shadow rays, `?splitrays=1`, off by default). fgather.wgsl's `main` lights every point
// its rays land on inside itself: directAtPoint is written at four places in it (the rays' loop, the young rays, the floor's mirror ray, the pane's), and a compiler that
// pastes every call into its caller reads the light loop and its three shadow rays four times over. Here the same frame is two programs:
//   trace   the gather as `main` has it, statement for statement, but for the light AT a hit. A hit that wants it is written to a buffer instead — the point, its normal,
//           the bounce-card set it lies in, its paint, and the radiance it already has without the light (emission + the cache's bounce) — and everything that needs no
//           light (a ray into the sky, the cards' term, the glints, a pane's dust) is summed and written beside it.
//   light   one loop over a texel's points with ONE directAtPoint in it: the light at each point, the hit's radiance finished, the firefly bound, the sum, the two stores.
// This text is appended to fgather.wgsl with its two entry points taken out (gatherSplit.ts), so every helper here is that file's own: a change to cosineDirK, the glints or
// the cards reaches both. `main`'s body, glossReflection and glassShade are written a second time below (trace, glossReflectionX, glassShadeX): a change to one of those
// three is made in both files, or this row draws the gather as it was before the change.
// NOT THE SAME RANDOM NUMBERS. In `main` the light loop draws from the texel's one stream, one number for each light that reaches the point, so the next ray's direction
// depends on how many lights the last hit saw: a count only the light loop knows. Here the trace kernel's stream is `main`'s without those draws, and each point's light
// pick draws from a stream of its own (gxFork: the texel's seed and the point's slot). Every draw is still uniform and independent, so the estimate is the same one; a
// frame is other noise of the same size. tools/qa/splitrays-check.ts holds this text to fgather.wgsl's; the pictures are compared with tools/qa/splitrays-twin.cjs's
// (today's one kernel drawing from these streams) by tools/qa/draw-equal.cjs.
// NOT YET HERE (docs/agents/render.md, "Separate shadow rays"): the young-history rays (a Tab row, no preset: the multiplier reads 1 here) and the resampling gather
// (mainGi, ultra's: the plain gather runs instead while this row is on). The shadow rays of the cards' term, the glints and a pane's dust stay in `trace`.
//
// THE BUFFER: GX_V4 vec4u a gather texel, row by row over the gather grid.
//   [0]  the firefly bound for the rays (f32) · E without the lit points, π/K and the cards' term in (3 × f32)
//   [1]  bits 0‥5 = the slots that hold a point, GX_SPEC = the specular field is written, GX_LIVE = this texel has work · the reflection's hit distance (f32) ·
//        the specular with nothing left to light (3 × f16)
//   [2]  the floor's glints and factor, the pane's glints and Fresnel (4 × f16 each), for a lit mirror hit
//   [3 + 2j], [4 + 2j]  point j (0‥3 = the rays, 4 = the floor's mirror ray, 5 = the pane's): P (3 × f32) · N (octahedral, 16 + 16 bits: an axis is exact) ·
//        paint (3 × f16) · radiance without the light (3 × f16) · the card set
@group(1) @binding(17) var<storage, read_write> gx: array<vec4u>;
const GX_V4: u32 = 15u;
const GX_REC: u32 = 3u;
const GX_SLOTS: u32 = 6u;   // (RAYS + 2: fgather.wgsl's RAYS is 4 in every pipeline — nothing overrides it — and the check holds the two together)
const GX_SPEC: u32 = 0x100u;
const GX_LIVE: u32 = 0x200u;
const GX_OCT: f32 = 65534.0;   // (even: ±x, ±y, ±z land on a code and come back exactly)

fn gxPut(gb: u32, j: u32, P: vec3f, N: vec3f, ex: u32, albedo: vec3f, Lpre: vec3f) {
  if (j >= GX_SLOTS) { return; }   // (never past the texel's own words)
  let i = gb + GX_REC + j * 2u;
  let o = vec2u(clamp(octEncode(N) * GX_OCT + 0.5, vec2f(0.0), vec2f(GX_OCT)));
  let l = min(Lpre, vec3f(6.0e4));
  gx[i] = vec4u(bitcast<vec3u>(P), o.x | (o.y << 16u));
  gx[i + 1u] = vec4u(pack2x16float(albedo.xy), pack2x16float(vec2f(albedo.z, l.x)), pack2x16float(l.yz), ex);
}
/** the seed of point j's own stream, from the texel's (rngState straight after rngInit) */
fn gxFork(seed: u32, j: u32) -> u32 {
  var v = (seed ^ ((j + 1u) * 0x9E3779B9u)) * 747796405u + 2891336453u;
  v = ((v >> ((v >> 28u) + 4u)) ^ v) * 277803737u;
  return (v >> 22u) ^ v;
}

// what a mirror ray leaves: `now` = its share with nothing left to light (whole when the hit wants no light); for a lit hit the glints and the factor ride in the header
struct GxSpec { now: vec3f, lit: bool, G: vec3f, mul: f32 };

// glossReflection (fgather.wgsl), its statements: the hit's direct light is slot RAYS's to bring
fn glossReflectionX(P: vec3f, owner: u32, gb: u32) -> GxSpec {
  let I = normalize(P - frame.camPos);
  let cosV = clamp(-I.y, 0.0, 1.0);
  let F = GLOSS_F0 + (1.0 - GLOSS_F0) * pow(1.0 - cosV, 5.0);
  let R = vec3f(I.x, -I.y, I.z);
  let up = select(vec3f(0.0, 1.0, 0.0), vec3f(1.0, 0.0, 0.0), abs(R.y) > 0.9);
  let T = normalize(cross(up, R)); let B = cross(R, T);
  let r = GLOSS_SPREAD * sqrt(rnd()); let a = rnd() * 6.2831853;
  var dir = normalize(R + (T * cos(a) + B * sin(a)) * r);
  if (dir.y < 0.02) { dir = R; }
  let Po = P + vec3f(0.0, 0.02, 0.0);
  let hit = traceClosest(Po, dir, TMAX, owner);
  var L = vec3f(0.0);
  var o: GxSpec; o.now = vec3f(0.0); o.lit = false; o.G = vec3f(0.0);
  specHitT = select(1e4, hit.t, hit.idx >= 0);
  if (hit.idx < 0) { L = skyRadiance(dir); }
  else if (!hit.inside) {
    let Ph = Po + dir * hit.t;
    var albedo: vec3f;
    if (hit.idx == CAPSULE_HIT) { albedo = hit.capAlbedo; }
    else {
      let bi = u32(hit.idx);
      if ((boxGeo[bi].flags & BOX_EMISSIVE) != 0u) { L += boxEmissive(bi); }
      albedo = boxAlbedoSec(bi);
    }
    if ((frame.flags & FLAG_BOUNCE) != 0u) { L += albedo * diceIrradiance(Ph + hit.n * c0Spacing(), hit.n) * (1.0 / 3.14159265); }
    if ((frame.flags & FLAG_DIRECT) != 0u) { gxPut(gb, RAYS, Ph, hit.n, 0u, albedo, L); o.lit = true; }
  }
  if ((frame.flags & FLAG_DIRECT) != 0u) {
    var G = glossGlints(Po, R, owner);
    let lg = luminance(G);
    o.G = select(G, G * (GLOSS_GLINT_MAX / lg), lg > GLOSS_GLINT_MAX);
  }
  let Wc = frame.weatherC; var wetOut = select(0.0, frame.weatherA.w, P.x < Wc.x || P.z < Wc.y || P.x > Wc.z || P.z > Wc.w);
  if (WX_KINDS) { if (frame.cardCfg.y != 0u && wxRoofed(P.xz)) { wetOut = 0.0; } }
  o.mul = min(F * (1.0 + 2.5 * wetOut), 1.0) * GLOSS_STRENGTH;
  if (!o.lit) {
    let lum = luminance(L);
    L = select(L, L * (GLOSS_RAY_MAX / lum), lum > GLOSS_RAY_MAX);
    o.now = (L + o.G) * o.mul;
  }
  return o;
}

// glassShade (fgather.wgsl), its statements: the mirror hit's direct light is slot RAYS + 1's to bring; the dust is whole here
fn glassShadeX(Pg: vec3f, n: vec3f, V: vec3f, gb: u32) -> GxSpec {
  let cosI = clamp(dot(-V, n), 0.0, 1.0);
  let F = glassFresnel(cosI) * frame.glassCfg.x;
  let R = V - n * (2.0 * dot(V, n));
  let Po = Pg + n * 0.015;
  var o: GxSpec; o.now = vec3f(0.0); o.lit = false; o.G = vec3f(0.0); o.mul = F;
  if (F > 0.001) {
    var L = vec3f(0.0);
    let hit = traceClosest(Po, R, TMAX, 0u);
    specHitT = select(1e4, hit.t, hit.idx >= 0);
    if (hit.idx < 0) { L = skyRadiance(R); }
    else if (!hit.inside) {
      let Ph = Po + R * hit.t;
      var albedo: vec3f;
      if (hit.idx == CAPSULE_HIT) { albedo = hit.capAlbedo; }
      else {
        let bi = u32(hit.idx);
        if ((boxGeo[bi].flags & BOX_EMISSIVE) != 0u) { L += boxEmissive(bi); }
        albedo = boxAlbedoSec(bi);
      }
      if ((frame.flags & FLAG_BOUNCE) != 0u) { L += albedo * diceIrradiance(Ph + hit.n * c0Spacing(), hit.n) * (1.0 / 3.14159265); }
      if ((frame.flags & FLAG_DIRECT) != 0u) { gxPut(gb, RAYS + 1u, Ph, hit.n, 0u, albedo, L); o.lit = true; }
    }
    if ((frame.flags & FLAG_DIRECT) != 0u) {
      var G = glossGlints(Po, R, 0u);
      let lg = luminance(G);
      o.G = select(G, G * (GLOSS_GLINT_MAX / lg), lg > GLOSS_GLINT_MAX);
    }
    if (!o.lit) {
      let lum = luminance(L);
      L = select(L, L * (GLOSS_RAY_MAX / lum), lum > GLOSS_RAY_MAX);
      o.now = (L + o.G) * F;
    }
  }
  let kd = frame.glassCfg.z * GLASS_DUST;
  if (kd > 0.0 && (frame.flags & FLAG_DIRECT) != 0u) {
    var e0 = vec3f(0.0); var e1 = vec3f(0.0); var w0 = 0.0; var w1 = 0.0; var i0 = -1; var i1 = -1;
    var L0 = vec3f(0.0); var L1 = vec3f(0.0); var d0 = 0.0; var d1 = 0.0; var rest = vec3f(0.0);
    for (var i = 0u; i < nL; i++) {
      let li = wgLights[i];
      if (lightTransient(li)) { continue; }
      if (!lightReaches(i, Pg)) { continue; }
      let ev = evalLight(li, Pg, vec3f(0.0));
      if (!ev.ok) { continue; }
      let E = ev.E * abs(dot(n, ev.L));
      let w = luminance(E);
      if (w < 1e-4) { continue; }
      if (w > w0) { rest += e1; e1 = e0; w1 = w0; i1 = i0; L1 = L0; d1 = d0; e0 = E; w0 = w; i0 = i32(i); L0 = ev.L; d0 = ev.dist.x; }
      else if (w > w1) { rest += e1; e1 = E; w1 = w; i1 = i32(i); L1 = ev.L; d1 = ev.dist.x; }
      else { rest += E; }
    }
    var Ed = rest * 0.5;
    for (var k = 0; k < 2; k++) {
      let ii = select(i1, i0, k == 0); if (ii < 0) { continue; }
      let li = wgLights[u32(ii)]; let Ld = select(L1, L0, k == 0); let dd = select(d1, d0, k == 0); let E = select(e1, e0, k == 0);
      let side = Pg + n * (0.015 * sign(dot(n, Ld)));
      let tmax = select(dd - li.radius - 0.02, 60.0, lightKind(li) == 2u);
      if (tmax > 0.0 && occluded(side, Ld, tmax, (li.owner & 0xffu) << 8u)) { continue; }
      var tr = 1.0;
      if (frame.numSmoke > 0u) { tr = smokeTransmittance(side, Ld, min(max(tmax, 0.0), 30.0)); }
      Ed += E * tr;
    }
    o.now += Ed * (kd / 3.14159265);
  }
  return o;
}

// `main` (fgather.wgsl) without the light at a hit, and without the young-history rays
@compute @workgroup_size(8, 8)
fn trace(@builtin(global_invocation_id) gid: vec3u, @builtin(local_invocation_index) lidx: u32) {
  nL = min(frame.numLights, 64u);
  for (var i = lidx; i < nL; i += 64u) { wgLights[i] = lights[i]; }
  nC = select(0u, min(frame.cardCfg.x, 8u), (frame.flags & FLAG_BOUNCE_CARDS) != 0u);
  wgCardOf[lidx] = -1;
  if (lidx < nC * 4u) { wgCards[lidx] = textureLoad(cardsTex, vec2u(lidx >> 2u, lidx & 3u), 0); }
  workgroupBarrier();
  if (lidx < nC && wgCards[lidx * 4u].w > 0.0) { wgCardOf[u32(wgCards[lidx * 4u + 1u].w)] = i32(lidx); }
  workgroupBarrier();
  let odims = textureDimensions(outIndirect);
  if (gid.x >= odims.x || gid.y >= odims.y) { return; }
  let gb = (gid.y * odims.x + gid.x) * GX_V4;
  let gdiv = u32(frame.lossyCfg.x);
  let full = min(gid.xy * gdiv + (gdiv - 1u) / 2u, vec2u(frame.screen) - 1u);
  let depth = textureLoad(gDepth, full, 0);
  let gloss = (frame.flags & FLAG_GLOSS) != 0u;
  let glassOn = (frame.flags & FLAG_GLASS) != 0u;
  let specOn = gloss || glassOn;
  if (depth >= 1.0 || (frame.flags & FLAG_INDIRECT) == 0u) {   // nothing to gather: the zeros are stored here, and `light` passes the texel by
    textureStore(outIndirect, gid.xy, vec4f(0.0)); if (specOn) { textureStore(outSpecular, gid.xy, vec4f(0.0)); }
    gx[gb + 1u] = vec4u(0u);
    return;
  }
  rngInit(gid.xy, frame.frameIdx);
  let P = worldFromDepth(vec2f(full) + 0.5, depth);
  let N = normalize(textureLoad(gNormal, full, 0).xyz * 2.0 - 1.0);
  let id = textureLoad(gId, full, 0).x;
  let owner = (id >> 20u) & 0xffu;
  let Po = P + N * 0.02;
  let Kff = frame.lossyCfg.w;
  var Lcap = 1e9;
  if (Kff > 0.0) { let Eref = diceIrradiance(P + N * c0Spacing(), N); Lcap = max(Kff * luminance(Eref) * (1.0 / 3.14159265), 0.35); }
  let cap = min(Lcap, 24.0);
  var E = vec3f(0.0);
  var mask = 0u;
  for (var k = 0u; k < RAYS; k++) {
    let dir = cosineDirK(N, k, RAYS);
    let hit = traceClosest(Po, dir, TMAX, owner);
    var L = vec3f(0.0);
    var lit = false;
    if (hit.idx < 0) { L = skyRadiance(dir); }
    else if (!hit.inside) {
      let Ph = Po + dir * hit.t;
      var albedo: vec3f;
      var ex = 0u;
      if (hit.idx == CAPSULE_HIT) { albedo = hit.capAlbedo; }
      else {
        let bi = u32(hit.idx);
        if ((boxGeo[bi].flags & (BOX_EMISSIVE | BOX_AREALIT)) == BOX_EMISSIVE) { L += boxEmissive(bi); }
        albedo = boxAlbedoSec(bi);
        for (var c = 0u; c < nC; c++) { if (onCard(Ph, hit.n, wgCards[c * 4u], wgCards[c * 4u + 1u])) { ex |= 1u << c; } }
      }
      if ((frame.flags & FLAG_BOUNCE) != 0u) {
        var Eb: vec3f;
        if (BOUNCE_WALLS && (bounceHitMan(hit) || bounceLeftNoWall(N, owner, hit.t))) { Eb = diceIrradianceWalled(Ph, hit.n, bounceHitOwner(hit) | (owner << 8u)); } else { Eb = diceIrradianceFrom(Ph, hit.n, Po, N); }
        L += albedo * Eb * (1.0 / 3.14159265);
      }
      if ((frame.flags & FLAG_DIRECT) != 0u) { gxPut(gb, k, Ph, hit.n, ex, albedo, L); mask |= 1u << k; lit = true; }
    }
    if (!lit) {   // (a lit hit takes the firefly bound in `light`, with its light in)
      let lum = luminance(L);
      E += select(L, L * (cap / lum), lum > cap);
    }
  }
  E *= 3.14159265 / f32(RAYS);
  if (nC > 0u && (frame.flags & FLAG_DIRECT) != 0u) { E += cardIrradiance(P, N, Po, owner, gid.xy); }
  var specNow = vec3f(0.0);
  if (specOn) {
    var gG = vec3f(0.0); var gM = 0.0; var pG = vec3f(0.0); var pF = 0.0;
    let bidx = id & 0xFFFFFu;
    if (gloss && ((id >> 28u) & 1u) == 0u && bidx != 0xFFFFFu && (boxGeo[bidx].flags & BOX_GLOSS) != 0u && N.y > 0.95) {
      let o = glossReflectionX(P, owner, gb);
      specNow = o.now;
      if (o.lit) { mask |= 1u << RAYS; gG = o.G; gM = o.mul; }
    }
    if (glassOn) {
      let toP = P - frame.camPos; let dist = length(toP); let V = toP / max(dist, 1e-4);
      let gh = glassTrace(frame.camPos, V, 0.0, dist - 0.02);
      if (gh.idx >= 0) {
        let o = glassShadeX(frame.camPos + V * gh.t, gh.n, V, gb);
        specNow += o.now;
        if (o.lit) { mask |= 1u << (RAYS + 1u); pG = o.G; pF = o.mul; }
      }
    }
    mask |= GX_SPEC;
    gx[gb + 2u] = vec4u(pack2x16float(gG.xy), pack2x16float(vec2f(gG.z, gM)), pack2x16float(pG.xy), pack2x16float(vec2f(pG.z, pF)));
  }
  gx[gb] = vec4u(bitcast<u32>(cap), bitcast<vec3u>(E));
  gx[gb + 1u] = vec4u(mask | GX_LIVE, bitcast<u32>(specHitT), pack2x16float(specNow.xy), pack2x16float(vec2f(specNow.z, 0.0)));
}

// the light at every point `trace` wrote for this texel, and the gather's two stores
@compute @workgroup_size(8, 8)
fn light(@builtin(global_invocation_id) gid: vec3u, @builtin(local_invocation_index) lidx: u32) {
  nL = min(frame.numLights, 64u);
  for (var i = lidx; i < nL; i += 64u) { wgLights[i] = lights[i]; }
  nC = select(0u, min(frame.cardCfg.x, 8u), (frame.flags & FLAG_BOUNCE_CARDS) != 0u);
  wgCardOf[lidx] = -1;
  if (lidx < nC * 4u) { wgCards[lidx] = textureLoad(cardsTex, vec2u(lidx >> 2u, lidx & 3u), 0); }
  workgroupBarrier();
  if (lidx < nC && wgCards[lidx * 4u].w > 0.0) { wgCardOf[u32(wgCards[lidx * 4u + 1u].w)] = i32(lidx); }
  workgroupBarrier();
  let odims = textureDimensions(outIndirect);
  if (gid.x >= odims.x || gid.y >= odims.y) { return; }
  let gb = (gid.y * odims.x + gid.x) * GX_V4;
  let h1 = gx[gb + 1u];
  if ((h1.x & GX_LIVE) == 0u) { return; }
  let h0 = gx[gb];
  let cap = bitcast<f32>(h0.x);
  rngInit(gid.xy, frame.frameIdx);
  let seed = rngState;
  var hs = vec4u(0u);
  if ((h1.x & GX_SPEC) != 0u) { hs = gx[gb + 2u]; }
  var Esum = vec3f(0.0);
  var spec = vec3f(unpack2x16float(h1.z), unpack2x16float(h1.w).x);
  for (var j = 0u; j < GX_SLOTS; j++) {
    if ((h1.x & (1u << j)) == 0u) { continue; }
    let a = gx[gb + GX_REC + j * 2u]; let b = gx[gb + GX_REC + j * 2u + 1u];
    let Ph = bitcast<vec3f>(a.xyz);
    let n = octDecode(vec2f(f32(a.w & 0xffffu), f32(a.w >> 16u)) / GX_OCT);
    let ab = unpack2x16float(b.x); let al = unpack2x16float(b.y); let ll = unpack2x16float(b.z);
    let albedo = vec3f(ab, al.x);
    rngState = gxFork(seed, j);
    var L = vec3f(al.y, ll) + albedo * directAtPoint(Ph, n, b.w) * (1.0 / 3.14159265);
    let c = select(GLOSS_RAY_MAX, cap, j < RAYS);
    let lum = luminance(L);
    L = select(L, L * (c / lum), lum > c);
    if (j < RAYS) { Esum += L; }
    else {
      let g = select(hs.zw, hs.xy, j == RAYS);
      let g0 = unpack2x16float(g.x); let g1 = unpack2x16float(g.y);
      spec += (L + vec3f(g0, g1.x)) * g1.y;
    }
  }
  let E = bitcast<vec3f>(h0.yzw) + Esum * (3.14159265 / f32(RAYS));
  textureStore(outIndirect, gid.xy, vec4f(E * frame.indirectScale, 1.0));
  if ((h1.x & GX_SPEC) != 0u) { textureStore(outSpecular, gid.xy, vec4f(spec, bitcast<f32>(h1.y))); }
}
