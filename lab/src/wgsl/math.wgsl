// Plain functions, no bindings.
const PI = 3.14159265;
fn lum(c: vec3f) -> f32 { return dot(c, vec3f(0.2126, 0.7152, 0.0722)); }
fn max3(v: vec3f) -> f32 { return max(v.x, max(v.y, v.z)); }
fn min3(v: vec3f) -> f32 { return min(v.x, min(v.y, v.z)); }
fn sat(x: f32) -> f32 { return clamp(x, 0.0, 1.0); }

fn pcg(v: u32) -> u32 { let s = v * 747796405u + 2891336453u; let w = ((s >> ((s >> 28u) + 4u)) ^ s) * 277803737u; return (w >> 22u) ^ w; }
fn hash33(p: vec3f) -> vec3f { var q = vec3u(bitcast<vec3u>(p)); q = q * 1664525u + 1013904223u; q.x += q.y * q.z; q.y += q.z * q.x; q.z += q.x * q.y; q ^= q >> vec3u(16u); q.x += q.y * q.z; q.y += q.z * q.x; q.z += q.x * q.y; return vec3f(q >> vec3u(8u)) * (1.0 / 16777216.0); }
fn hash21(p: vec2f) -> f32 { return hash33(vec3f(p, 7.0)).x; }

// ---- directions
fn octEnc(n: vec3f) -> vec2f { let s = abs(n.x) + abs(n.y) + abs(n.z); var p = n.xz / s; if (n.y < 0.0) { p = (1.0 - abs(p.yx)) * select(vec2f(-1.0), vec2f(1.0), p >= vec2f(0.0)); } return p * 0.5 + 0.5; }
fn octDec(e: vec2f) -> vec3f { let p = e * 2.0 - 1.0; var n = vec3f(p.x, 1.0 - abs(p.x) - abs(p.y), p.y); if (n.y < 0.0) { let q = (1.0 - abs(n.zx)) * select(vec2f(-1.0), vec2f(1.0), n.xz >= vec2f(0.0)); n = vec3f(q.x, n.y, q.y); } return normalize(n); }
fn octPack(n: vec3f) -> u32 { let q = vec2u(octEnc(n) * 65535.0 + 0.5); return q.x | (q.y << 16u); }
fn octUnpack(u: u32) -> vec3f { return octDec(vec2f(f32(u & 0xffffu), f32(u >> 16u)) / 65535.0); }
fn basis(n: vec3f) -> mat3x3f { let s = select(-1.0, 1.0, n.z >= 0.0); let a = -1.0 / (s + n.z); let b = n.x * n.y * a; return mat3x3f(vec3f(1.0 + s * n.x * n.x * a, s * b, -s * n.x), vec3f(b, s + n.y * n.y * a, -n.y), n); }
fn cosineDir(n: vec3f, u: vec2f) -> vec3f { let r = sqrt(u.x); let a = 2.0 * PI * u.y; return normalize(basis(n) * vec3f(r * cos(a), r * sin(a), sqrt(max(0.0, 1.0 - u.x)))); }
fn coneDir(d: vec3f, radius: f32, u: vec2f) -> vec3f { let r = sqrt(u.x) * radius; let a = 2.0 * PI * u.y; return normalize(basis(d) * vec3f(r * cos(a), r * sin(a), 1.0)); }

// ---- packed colors
fn unpackTint(t: u32) -> vec3f { return vec3f(f32(t & 255u), f32((t >> 8u) & 255u), f32((t >> 16u) & 255u)) * (1.0 / 255.0); }
fn packRGB(c: vec3f) -> u32 { let q = vec3u(clamp(sqrt(c), vec3f(0.0), vec3f(1.0)) * 255.0 + 0.5); return q.x | (q.y << 8u) | (q.z << 16u); }   // (a square root: the eye's steps, and dark paint keeps its hue)
fn unpackRGB(u: u32) -> vec3f { let c = vec3f(f32(u & 255u), f32((u >> 8u) & 255u), f32((u >> 16u) & 255u)) * (1.0 / 255.0); return c * c; }

