// HISTORY, for any signal: last frame's answer for this surface, checked, held to what this frame says nearby, and mixed by how many stand behind it.
// const DIV = the signal's grid over the render grid; CAP = frames; SPREAD = how far (sigmas) history may stand off; NARROW = 1 for a mirror's image
/** which channels have a say: a fraction of light that is not there (−1) has none */
fn said(v: vec4f) -> vec4f { if (KIND == 1) { return select(vec4f(0.0), vec4f(1.0), v >= vec4f(0.0)); } return vec4f(1.0); }
var<workgroup> tile: array<vec4f, 144>;                            // this group's 8 × 8 and two round it
@compute @workgroup_size(8, 8) fn main(@builtin(global_invocation_id) id: vec3u, @builtin(local_invocation_id) lid: vec3u, @builtin(local_invocation_index) li: u32) {
  let dim = textureDimensions(outH); let t0 = vec2i(id.xy) - vec2i(lid.xy) - 2; for (var i = li; i < 144u; i += 64u) { tile[i] = textureLoad(cur, clamp(t0 + vec2i(i32(i % 12u), i32(i / 12u)), vec2i(0), vec2i(dim) - 1), 0); } workgroupBarrier();
  if (id.x >= dim.x || id.y >= dim.y) { return; }
  //2 let now2 = textureLoad(cur2, id.xy, 0); textureStore(outH2, id.xy, now2); var h2 = vec4f(0.0); var o2 = now2;
  let rdim = vec2f(frame.size.xy); let full = min(id.xy * u32(DIV), vec2u(rdim) - 1u); let rd = pxRay(full); let p = unpackPx(textureLoad(surf, full, 0), rd); let at = (lid.y + 2u) * 12u + lid.x + 2u; let c = tile[at];
  if (p.sky && NARROW == 0 && airAt2(vec2u(id.xy)).x <= 0.0) { textureStore(outH, id.xy, c); textureStore(outM, id.xy, vec4f(0.0)); return; }
  if (NARROW == 1 && c.a < 0.0) { textureStore(outH, id.xy, c); textureStore(outM, id.xy, vec4f(0.0)); return; }
  var m1 = vec4f(0.0); var m2 = vec4f(0.0); var wn = 0.0; var wv = vec4f(0.0); let R = 2;
  for (var y = -R; y <= R; y++) { for (var x = -R; x <= R; x++) { let v = tile[u32(i32(at) + y * 12 + x)]; if (NARROW == 1 && v.a < 0.0) { continue; } let k = said(v); m1 += v * k; m2 += v * v * k; wv += k; wn += 1.0; } }
  m1 /= max(wv, vec4f(1.0)); let sd = sqrt(max(m2 / max(wv, vec4f(1.0)) - m1 * m1, vec4f(0.0)));
  let dull = select(0.0, smoothstep(0.1, 0.28, p.rough), NARROW == 1 && LAYER == 0);      // a rough coat finds its light by chance: what its neighbours found this frame says little
  var c1 = c; if (NARROW == 1 && wn > 2.0) { let k = (m1 * wn - c) / (wn - 1.0); let e = sqrt(max((m2 - c * c) / (wn - 1.0) - k * k, vec4f(0.0))); let most = lum(k.rgb) + mix(3.0, 8.0, dull) * lum(e.rgb) + 0.02; c1 = vec4f(c.rgb * min(1.0, most / max(lum(c.rgb), 1e-4)), c.a); }     // one bright sample alone is held to its neighbours
  let aux = textureLoad(auxT, full, 0); var uv = (vec2f(full) + 0.5) / rdim + aux.xy; var x0 = p.x; let fr = textureLoad(frontT, full, 0); var checkT = p.t; var front = false;
  let ad2 = airAt2(vec2u(id.xy)); let ad = ad2.x; let floats = ad > 0.0 && (ad < p.t - 0.1 || p.sky);      // air that stands off the surface
  if (floats) { let img = frame.camPos.xyz + rd * ad; let q = projectPrev(img - windAt(img) * frame.more.z); let now = project(img); uv = (vec2f(full) + 0.5) / rdim + (q.xy - now.xy); }
  if (NARROW == 1) {                                               // a mirror's image moves as if it stood behind the surface
    var t = p.t; var rough = p.rough; if (LAYER == 1 && fr.x != 0u) { t = bitcast<f32>(fr.x); rough = 0.02; front = true; } let img = frame.camPos.xyz + rd * (t + min(c.a, 60.0) * (1.0 - smoothstep(0.15, 0.45, rough))); let q = projectPrev(img); let now = project(img); uv = (vec2f(full) + 0.5) / rdim + (q.xy - now.xy);
  }
  let hp = uv * rdim / f32(DIV) - 0.5 + select(0.0, 0.25, DIV == 2); let b = floor(hp); let f = hp - b; var h = vec4f(0.0); var mo = vec4f(0.0); var ws = 0.0; var hw = vec4f(0.0);
  for (var k = 0u; k < 4u; k++) {
    let q = vec2i(b) + vec2i(i32(k & 1u), i32(k >> 1u)); if (any(q < vec2i(0)) || any(q >= vec2i(dim))) { continue; }
    var w = select(1.0 - f.x, f.x, (k & 1u) == 1u) * select(1.0 - f.y, f.y, k > 1u); if (w < 1e-3) { continue; }
    let fq = min(q * DIV, vec2i(rdim) - 1); let o = unpackPx(textureLoad(surfPrev, fq, 0), vec3f(0.0)); let was = frame.prevPos.xyz + pxRayPrev((vec2f(fq) + 0.5 + frame.jitter.zw) / rdim) * o.t;
    var ok = abs(dot(was - p.x, p.ng)) < 0.006 * p.t + 0.02 && dot(o.ng, p.ng) > 0.7; if (aux.w > 0.5) { ok = abs(o.t - p.t) < 0.25 && dot(o.ng, p.ng) > 0.4; } if (p.leaf) { ok = o.leaf && abs(o.t - p.t) < 0.5; } if (NARROW == 1 && front) { ok = true; } if (floats) { ok = o.t > ad - 0.25; }
    if (AIR == 1) { let was2 = airWas(vec2u(q)); let th = max(was2.y, ad2.y); if (th > 0.04) { let want = distance(frame.camPos.xyz + rd * select(p.t, ad, floats), frame.prevPos.xyz); ok = ok && abs(was2.x - want) < 0.06 * want + 0.35 && min(was2.y, ad2.y) > 0.3 * th; } }       /* what was seen of the air there was as far off, and as thick: or it is other air */
    if (!ok) { continue; } let v = textureLoad(hist, q, 0); let pm = textureLoad(momPrev, q, 0); if (bad4(v) || bad4(pm)) { continue; } if (NARROW == 1 && v.a < 0.0) { continue; } let kv = said(v) * w; h += v * kv; hw += kv; mo += pm * w; ws += w;
    //2 let v2 = textureLoad(hist2, q, 0); if (!bad4(v2)) { h2 += v2 * w; }
  }
  var n = 0.0; var o = c1; var mom = vec4f(lum(c1.rgb), lum(c1.rgb) * lum(c1.rgb), 0.0, 0.0); if (KIND == 1) { mom.w = select(1.0, 0.0, c.y < 0.0); }
  if (ws > 0.02) {
    h = select(c, h / max(hw, vec4f(1e-6)), hw > vec4f(1e-6)); mo /= ws; let st = steady(p.x); let cap = mix(3.0, CAP_NOW, st) * select(1.0, 0.5, aux.w > 0.5 || p.leaf) * select(1.0, mix(1.0, 0.35, sat(ad2.y * 2.5)), floats); n = min(mo.z, cap - 1.0);
    var se = sd; if (KIND == 1) { let q = (clamp(m1, vec4f(0.0), vec4f(1.0)) * wv + 1.0) / (wv + 2.0); /* (where all of them say the same there is little to doubt: a shadow that has moved on is let go) */ let doubt = select(sqrt(q * (1.0 - q)), vec4f(0.05), (sd < vec4f(1e-3)) & (wv > vec4f(19.0))); se = vec4f(max(sd.xyz, doubt.xyz), sd.w + 0.04); }
 
    var e = se * (mix(SPREAD, 4.5, dull) * @tune.leash) + abs(m1) * mix(0.04, 0.25, dull) + 1e-4;
    // light that is rarely found (a small bright window) is in none of most frames' neighbours: what it has varied by over time is let stand, or the dark would be darker than it is
    if (KIND == 0 && AIR == 0) { let sdh = sqrt(max(mo.y - mo.x * mo.x, 0.0)); e += vec4f(sdh * select(1.6, 1.0 * dull, NARROW == 1) * @tune.leash * smoothstep(4.0, 16.0, n)); } let hc = clamp(h, m1 - e, m1 + e); let moved = select(length((hc - h).rgb) / (length(e.rgb) + 1e-4), length((hc - h).xw) / (length(e.xw) + 1e-4), KIND == 1); n *= 1.0 / (1.0 + moved * 0.35);
    /* (torches move, the rest stands: each counts its own frames) */ var ny = 0.0; if (KIND == 1 && hw.y > 1e-6) { ny = min(mo.w, cap - 1.0) / (1.0 + 0.35 * abs(hc.y - h.y) / (e.y + 1e-4)); }
    /* (what torches light is told apart: where it has changed by more than chance, history is of other light) */ if (KIND == 0 && NARROW == 0 && AIR == 0) { let gone = max(abs(h.w - m1.w) - 0.5 * sd.w - 0.05 * m1.w, 0.0) / (max(lum(h.rgb), lum(m1.rgb)) + 1e-4); n /= 1.0 + 12.0 * gone; }
    var k = 1.0 / (n + 1.0); o = mix(hc, c1, select(vec4f(k), vec4f(k, 1.0 / (ny + 1.0), k, k), KIND == 1)); mom = vec4f(mix(mo.x, mom.x, max(k, 0.06)), mix(mo.y, mom.y, max(k, 0.06)), 0.0, select(0.0, ny + 1.0, KIND == 1 && c.y >= 0.0));
    //2 o2 = mix(h2 / ws, now2, k);
    if (NARROW == 1) { o.a = mix(h.a, c.a, max(k, 0.2)); }
    if (KIND == 1) { o = select(o, c, c < vec4f(0.0)); if (p.leaf) { o.w = c.w; } }     // (the sun on a leaf is known without noise, and goes with that leaf's own tilt: not mixed with what other leaves here saw)
  }
  mom.z = n + 1.0;
  //2 textureStore(outH2, id.xy, o2);
  textureStore(outH, id.xy, o); textureStore(outM, id.xy, mom);
}
