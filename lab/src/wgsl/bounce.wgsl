// What a surface that a ray found sends back: what it is lit by (cacheRead) times all it returns, its coat's share with its matte's.
// (Both in the mean over every way light may come and go. The coat's: its facets hide each other more, the rougher it is; fitted to what the coat's own sampling gives back. The matte's: less what the
// coat turns away, by Schlick's curve a twenty-first of what is left. White rooms are the judge: a hundredth too much a bounce is a tenth too bright in the end.)
fn returns(s: Surf) -> vec3f { let f = coatF0(s); let r = s.rough; let a = f * (0.955 / (1.0 + 1.346 * pow(r, 3.67))) + vec3f(0.0476 * exp(-3.55 * r * r)); let lf = lum(f); return matteOf(s) * matteMean(s.kind, s.albedo) * (1.0 - lf - (1.0 - lf) / 21.0) + a; }
fn sentBack(q: Surf, j: vec2f) -> vec3f { let c = cacheRead(q.x, q.ng, j); if (c.n <= 0.0) { return vec3f(0.0); } return returns(q) / PI * (c.E + frame.sunE.xyz * max(dot(q.ng, frame.sunDir.xyz), 0.0) * c.sun); }
