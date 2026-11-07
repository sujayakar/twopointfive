// Shared by the final gather (fgather.wgsl) and the ray-traced reflection pass (reflect.wgsl): the per-tile light / bounce-card staging, the RNG, and
// the direct light AT A HIT POINT (the exact on-screen textures when the point is visible this frame, else the one-shadow-ray estimate). Textually
// included ahead of either shader by passes.ts; both declare the bindings it reads under the same names (gDepth, gNormal, directTex, softTex) and
// fill wgLights / nL (and wgCards / wgCardOf / nC — the reflection pass leaves nC at 0: no card is ever excluded from a mirror image) in their entry point.
// every hit walks the light list (transient subtraction / off-screen estimate): stage it per 8x8 tile
var<workgroup> wgLights: array<Light, 64>;
var<private> nL: u32;
// torch bounce cards (FLAG_BOUNCE_CARDS), staged per tile likewise: the four rows of each live card, and per light slot the card built from it (−1 = none) so
// directAtPoint can drop an excluded light by slot without a search. nC = 0 whenever the flag is off: every loop over cards is then empty and every
// exclusion set 0 — the pass computes and stores exactly what it did before.
var<workgroup> wgCards: array<vec4f, 32>;
var<workgroup> wgCardOf: array<i32, 64>;
var<private> nC: u32;


var<private> rngState: u32;
fn rngInit(p: vec2u, f: u32) { rngState = (p.x * 1973u + p.y * 9277u + f * 26699u) | 1u; rngState = rngState ^ (rngState >> 15u); rngState *= 0x2c1b3c6du; rngState ^= rngState >> 12u; }
fn rnd() -> f32 { rngState = rngState * 747796405u + 2891336453u; var w = ((rngState >> ((rngState >> 28u) + 4u)) ^ rngState) * 277803737u; w = (w >> 22u) ^ w; return f32(w) * (1.0 / 4294967296.0); }

// On-screen shadowed direct light if the point is visible this frame (exact, already denoised), else a one-ray estimate.
// `ex` (torch bounce cards): bit k set = this point lies inside card k's claim region, whose light's direct term the card carries instead — taken back
// out of the on-screen textures (what that light delivered here: shadowed, through smoke), left out of the off-screen sum. Always 0 with the flag off.
// (Main's body verbatim. Phase 5 once had it set a private 'took the on-screen path' flag for the reflection pass's reuse: two dead-simple stores, yet with
// them live in the reflect kernel the Metal compiler scheduled the light loop differently and ONE light pick in a thousand flipped — a different output cell
// at ultra in one view. The reuse path asks directOnScreen below first instead, and this function stays exactly what every tier ran before.)
fn directAtPoint(P: vec3f, N: vec3f, ex: u32) -> vec3f {
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
          for (var i = min(frame.p5Cfg.w, nL); i < select(0u, nL, frame.rcJitter.w > 0.0); i++) {   // (skipped outright when no transient light is alive — the common case; else from the first transient's slot: they sit at the tail of the list, and every slot visited is still tested — the same subset in the same order, review R2.2)
            let li = wgLights[i];
            if (!lightTransient(li)) { continue; }
            let ev = evalLight(li, P, N);
            // only what the flash actually delivered here is in the texture: a hit in the flash's shadow keeps its steady light
            if (ev.ok && !occluded(P + N * 0.01, ev.L, max(ev.dist.x - li.radius - 0.02, 0.0), (li.owner & 0xffu) << 8u)) { Ed -= ev.E; }   // same skip rule the direct pass used when it PUT the flash there (its carrier never shadows it), or the subtraction misses in the shooter's wedge
          }
          // torch bounce cards: inside card k's footprint its light's direct term leaves the same way — what the direct pass delivered here (its shadow ray's
          // skip rule, the smoke on the segment), so a hit in a man's shadow inside the spot keeps everything it has (ex == 0u unless the flag is on)
          for (var k = 0u; k < select(0u, nC, ex != 0u); k++) {
            if ((ex & (1u << k)) == 0u) { continue; }
            let li = wgLights[u32(wgCards[k * 4u + 1u].w)];
            let ev = evalLight(li, P, N);
            if (!ev.ok) { continue; }
            let tmax = max(ev.dist.x - li.radius - 0.02, 0.0);
            if (occluded(P + N * 0.01, ev.L, tmax, (li.owner & 0xffu) << 8u)) { continue; }
            var sub = ev.E;
            if (frame.numSmoke > 0u) { sub *= smokeTransmittance(P + N * 0.01, ev.L, min(tmax, 30.0)); }
            Ed -= sub;
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
    if (ex != 0u) { let ck = wgCardOf[i]; if (ck >= 0 && (ex & (1u << u32(ck))) != 0u) { continue; } }   // torch bounce cards: this light's direct term here is the card's to carry (never taken with the flag off: ex == 0u)
    if (!lightReaches(i, P)) { continue; }   // walled off from the bounce point's cell (reach.ts): neither in the sum nor pickable — the same failure the direct pass had, one bounce down
    let ev = evalLight(li, P, N);
    if (!ev.ok) { continue; }
    let w = luminance(ev.E);
    if (w < 1e-4 * lampFloorK()) { continue; }
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

