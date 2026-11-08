// PATH GUIDING. Each patch of the world (a cell and the way it faces) learns from the paths that left it which directions bring light, in 128 bins of equal solid
// angle, and later paths are sent there more often. Nothing is changed but how directions are drawn: every direction keeps a chance, and its light is divided by
// that chance, so the mean is the same. What a path reads was settled before its frame began, and sums are whole numbers, so a run repeats.
const GS = 32768u; const G_REACH = 24u;
fn gKey(x: vec3f, n: vec3f) -> u32 { let c = select(6.0, 0.75, abs(x.x) < 22.0 && abs(x.z) < 17.0 && x.y > -1.0 && x.y < 13.0); let q = vec3i(floor((x + n * 0.08) / c)); let a = abs(n); var f = select(select(5u, 4u, n.z > 0.0), select(3u, 2u, n.y > 0.0), a.y >= a.z); if (a.x >= a.y && a.x >= a.z) { f = select(1u, 0u, n.x > 0.0); } return max(pcg(u32(q.x) + pcg(u32(q.y) + pcg(u32(q.z) + pcg(f + 8u * u32(c) + 55u)))), 1u); }
fn gFind(key: u32) -> u32 { var s = key % GS; for (var i = 0u; i < G_REACH; i++) { let k = atomicLoad(&gkeys[s]); if (k == key) { return s; } if (k == 0u) { return NOTHING; } s = (s + 1u) % GS; } return NOTHING; }
fn gFound(key: u32) -> u32 { var s = key % GS; var i = 0u; for (var t = 0u; t < 2u * G_REACH && i < G_REACH; t++) { let k = atomicLoad(&gkeys[s]); if (k == key) { return s; } if (k == 0u) { let r = atomicCompareExchangeWeak(&gkeys[s], 0u, key); if (r.exchanged || r.old_value == key) { return s; } if (r.old_value == 0u) { continue; } } s = (s + 1u) % GS; i++; } return NOTHING; }
fn binOf(d: vec3f) -> u32 { let z = u32(clamp((d.y * 0.5 + 0.5) * 8.0, 0.0, 7.999)); let p = u32(clamp((atan2(d.z, d.x) / (2.0 * PI) + 0.5) * 16.0, 0.0, 15.999)); return z * 16u + p; }
fn binDir(b: u32, u: vec2f) -> vec3f { let z = (f32(b >> 4u) + u.x) / 8.0 * 2.0 - 1.0; let p = ((f32(b & 15u) + u.y) / 16.0 - 0.5) * 2.0 * PI; let r = sqrt(max(1.0 - z * z, 0.0)); return vec3f(r * cos(p), z, r * sin(p)); }
fn gValue(s: u32, b: u32) -> f32 { return guide[s * 32u + (b >> 2u)][b & 3u]; }
/** what a bin is worth to a surface that faces n: what comes from there, and how squarely */
fn gWorth(s: u32, b: u32, n: vec3f) -> f32 { return gValue(s, b) * max(dot(binDir(b, vec2f(0.5)), n) + 0.22, 0.0); }
const GUIDED = 0.6;
/** a direction for wide incoming light: w = cosine over its chance */
fn guidedDir(x: vec3f, n: vec3f, u: vec2f, pick: f32) -> vec4f {
  let s = gFind(gKey(x, n)); if (s == NOTHING || guide[s * 32u].x <= 0.0) { return wideDir(x, n, u, pick); }
  var W = 0.0; var b = 0u; var v = fract(pick * 7.13 + 0.31); for (var i = 0u; i < 128u; i++) { let w = gWorth(s, i, n); if (w <= 0.0) { continue; } W += w; let q = w / W; if (v < q) { b = i; v = v / q; } else { v = (v - q) / (1.0 - q); } }
  if (W <= 0.0) { return wideDir(x, n, u, pick); } var d: vec3f; if (pick < GUIDED) { d = binDir(b, u); } else { let q = wideDir(x, n, u, (pick - GUIDED) / (1.0 - GUIDED)); if (q.w <= 0.0) { return q; } d = q.xyz; }
  let c = dot(n, d); if (c <= 1e-4) { return vec4f(d, 0.0); } return vec4f(d, c / (GUIDED * gWorth(s, binOf(d), n) / W * (128.0 / (4.0 * PI)) + (1.0 - GUIDED) * widePdf(x, n, d)));
}
/** what this path met on its way, to tell each patch when the path is done */
var<private> gAt: array<u32, 6>; var<private> gThr: array<f32, 6>; var<private> gWas: array<f32, 6>; var<private> gCount: u32;
fn gMark(x: vec3f, n: vec3f, d: vec3f, thr: vec3f, L: vec3f) { if (pathKnobs.w == 0u || gCount >= 6u) { return; } let s = gFound(gKey(x, n)); if (s == NOTHING) { return; } gAt[gCount] = s * 128u + binOf(d); gThr[gCount] = lum(thr); gWas[gCount] = lum(L); gCount += 1u; }
fn gTell(L: vec3f) { for (var i = 0u; i < gCount; i++) { if (gThr[i] < 1e-5) { continue; } let r = clamp((lum(L) - gWas[i]) / gThr[i], 0.0, 2000.0); atomicAdd(&train[gAt[i] * 2u], u32(r * 16.0)); atomicAdd(&train[gAt[i] * 2u + 1u], 1u); } }
