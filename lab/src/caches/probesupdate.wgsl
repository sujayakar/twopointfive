// a probe's turn: RAYS rays, evenly round it and turned by chance; then each texel takes what the rays that fall to it brought
const NL = 8u; var<workgroup> dd: array<vec4f, NL>; var<workgroup> de: array<vec4f, NL>; var<workgroup> go: u32; var<workgroup> rd: array<vec4f, RAYS>; var<workgroup> rl: array<vec4f, RAYS>;
@compute @workgroup_size(64) fn main(@builtin(workgroup_id) g: vec3u, @builtin(local_invocation_index) li: u32) {
  let p = g.x + g.y * 32768u; if (li == 0u) { var yes = 0u; if (p < PROBES) { let a = at[p]; let l = irr[p * WORDS + WORDS - 1u]; if (a.w > 0.5) { yes = 1u; if (l > 12u && steady(a.xyz) > 0.9 && (pcg(p) & TURN) != (frame.counts.z & TURN)) { yes = 0u; } } } go = yes; }
  let on = workgroupUniformLoad(&go); if (on == 0u) { return; }
  let a = at[p]; let x = a.xyz; let st = steady(x); byReach = true; leafMode = 2u; rngKey(p, 91u); let turn = rnd(); let z0 = rnd(); let ax = vec3f(rnd(), rnd(), rnd()); footSolid(0.32, 1.0);
  for (var i = li; i < RAYS; i += 64u) {
    let z = 1.0 - 2.0 * fract((f32(i) + 0.5) / f32(RAYS) + 0.5 * (z0 - 0.5) / f32(RAYS)); let s = sqrt(max(1.0 - z * z, 0.0)); let an = 2.3999632 * f32(i) + 6.2831853 * turn; let d0 = vec3f(s * cos(an), z, s * sin(an)); let d = normalize(basis(octDec(ax.xy)) * d0); rngKey(p * RAYS + i, 92u);
    let L = min(look(x, d, 1.0), vec3f(60.0)); rd[i] = vec4f(d, min(lookT, STEP * 2.6)); rl[i] = vec4f(finite4(vec4f(L, 0.0), 1e3).xyz, 0.0);
  }
  if (DIRECT && li < NL) { rngKey(p * NL + li, 131u); let q = lampsFrom(x); dd[li] = vec4f(q[0], sunFrom(x)); de[li] = vec4f(q[1] / f32(NL), 0.0); }
  workgroupBarrier();
  var n = f32(irr[p * WORDS + WORDS - 1u]); if (changes.all.y > 0.5) { n = min(n, 1.0); } let cap = mix(min(3.0, KEEP), KEEP, st); let m = min(n + 1.0, cap); let k = min(1.0, 2.0 / (m + 1.0));
  if (OCT) { for (var t = li; t < ON * ON; t += 64u) {
    let d = octDec((vec2f(f32(t % ON), f32(t / ON)) + 0.5) / f32(ON)); var e = vec3f(0.0); var w = 0.0; for (var i = 0u; i < RAYS; i++) { let c = max(dot(d, rd[i].xyz), 0.0); e += rl[i].xyz * c; w += c; } e *= PI / max(w, 1e-3); if (DIRECT) { for (var i = 0u; i < NL; i++) { e += de[i].xyz * max(dot(d, dd[i].xyz), 0.0); } }
    let o = p * WORDS + t * 2u; let was = vec3f(unpack2x16float(irr[o]), unpack2x16float(irr[o + 1u]).x); let r = clamp(mix(was, e, k), vec3f(0.0), vec3f(6e4)); let v = bitcast<vec3f>((bitcast<vec3u>(r) + (vec3u(pcg(o + frame.counts.z), pcg(o * 3u + frame.counts.z), pcg(o * 7u + frame.counts.z)) & vec3u(8191u))) & vec3u(0xffffe000u)); irr[o] = pack2x16float(v.xy); irr[o + 1u] = pack2x16float(vec2f(v.z, 0.0)); } }
  else if (li == 0u) {
    var v: array<vec3f, 4>; for (var i = 0u; i < RAYS; i++) { let L = rl[i].xyz; v[0] += L; v[1] += L * (3.0 * rd[i].x); v[2] += L * (3.0 * rd[i].y); v[3] += L * (3.0 * rd[i].z); } if (DIRECT) { for (var i = 0u; i < NL; i++) { let L = de[i].xyz * (f32(RAYS) * 0.25 / PI); v[0] += L; v[1] += L * (3.0 * dd[i].x); v[2] += L * (3.0 * dd[i].y); v[3] += L * (3.0 * dd[i].z); } } let o = p * WORDS; var f: array<f32, 12>;
    for (var i = 0u; i < 6u; i++) { let w = unpack2x16float(irr[o + i]); f[2u * i] = w.x; f[2u * i + 1u] = w.y; } for (var i = 0u; i < 12u; i++) { let r = mix(f[i], v[i / 3u][i % 3u] / f32(RAYS), k); f[i] = bitcast<f32>((bitcast<u32>(finite(r, 6e4)) + (pcg(o * 12u + i + frame.counts.z * 977u) & 8191u)) & 0xffffe000u); }
    for (var i = 0u; i < 6u; i++) { irr[o + i] = pack2x16float(vec2f(f[2u * i], f[2u * i + 1u])); } }
  if (VIS == 2u) { for (var t = li; t < DN * DN; t += 64u) {
    let d = octDec((vec2f(f32(t % DN), f32(t / DN)) + 0.5) / f32(DN)); var e = vec2f(0.0); var w = 0.0; for (var i = 0u; i < RAYS; i++) { let c = pow(max(dot(d, rd[i].xyz), 0.0), 40.0); e += vec2f(rd[i].w, rd[i].w * rd[i].w) * c; w += c; }
    if (w > 1e-4) { let o = p * DN * DN + t; let was = unpack2x16float(dist[o]); let r = mix(was, e / w, select(k, 1.0, was.x <= 0.0)); dist[o] = pack2x16float(r); } } }
  if (DIRECT && li == 0u) { var su = 0.0; for (var i = 0u; i < NL; i++) { su += dd[i].w / f32(NL); } irr[p * WORDS + WORDS - 2u] = bitcast<u32>(clamp(finite(mix(bitcast<f32>(irr[p * WORDS + WORDS - 2u]), su, k), 1.0), 0.0, 1.0)); }
  if (li == 0u) { irr[p * WORDS + WORDS - 1u] = u32(min(n + 1.0, cap + 1.0)); }
}
