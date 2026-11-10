// What a surface that a ray found sends back: what it is lit by (cacheRead) times all it returns, its coat's share with its matte's.
// (Both in the mean over every way light may come and go. The coat's: its facets hide each other more, the rougher it is; fitted to what the coat's own sampling gives back. The matte's: less what the
// coat turns away, by Schlick's curve a twenty-first of what is left. White rooms are the judge: a hundredth too much a bounce is a tenth too bright in the end.)
fn returns(s: Surf) -> vec3f { let f = coatF0(s); let r = s.rough; let a = f * (0.955 / (1.0 + 1.346 * pow(r, 3.67))) + vec3f(0.0476 * exp(-3.55 * r * r)); let lf = lum(f); return matteOf(s) * matteMean(s.kind, s.albedo) * (1.0 - lf - (1.0 - lf) / 21.0) + a; }
/** of it, what torches light */
var<private> sentTorch = vec3f(0.0);
fn sentBack(q: Surf, j: vec2f, rd: vec3f) -> vec3f { askMoves = (q.flags & MOVES) != 0u; let c = cacheRead(q.x, q.ng, j); sentTorch = vec3f(0.0); if (c.n <= 0.0) { return vec3f(0.0); } let p = sentParts(q, rd); sentTorch = p[0] * c.torch; return p[0] * c.E + p[1] * c.sun; }
/** what it sends back along -rd of each unit of what lands on it; and of the sun, where all of that is seen (by the coat's own lobe, a little wide: spread evenly, windows in the sun lit what stands across from them) */
fn sentParts(q: Surf, rd: vec3f) -> array<vec3f, 2> {
  let lf = lum(coatF0(q)); let matte = matteOf(q) * matteMean(q.kind, q.albedo) * (1.0 - lf - (1.0 - lf) / 21.0);
  return array<vec3f, 2>(returns(q) / PI, matte * frame.sunE.xyz * (max(dot(q.ng, frame.sunDir.xyz), 0.0) / PI) + frame.sunE.xyz * coatOf(q, -rd, frame.sunDir.xyz, 0.08));
}
