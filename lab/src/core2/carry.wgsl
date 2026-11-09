// ================================================================ what is carried: limbs as soft boxes on their bones
const CARRY_CODE = 0xfffffffdu; const CARRY_PEOPLE = 8u; const CARRY_LIMBS = 14u;
/** which limb the last ray stopped in (+ 1), and which one it began at */
var<private> lastLimb: u32 = 0u; var<private> fromLimb: u32 = 0u;
fn limbRow(limb: u32) -> u32 { return BLK_CARRY + 9u + limb * 5u; }
/** where limbs take over from their parts */
fn limbFrom(level: f32) -> f32 { return select(blk.x, blk.y, level > 0.5); }
/** a ray through limb k from t0 on: where it is in it, and how much it meets a metre */
fn limbSpan(k: u32, ro: vec3f, rd: vec3f, t0: f32, t1: f32) -> vec3f {
  let r = limbRow(k); let a = extra[r]; let b = extra[r + 1u]; let c = extra[r + 2u]; let h = extra[r + 3u]; let o = vec3f(dot(a.xyz, ro) + a.w, dot(b.xyz, ro) + b.w, dot(c.xyz, ro) + c.w); let d = vec3f(dot(a.xyz, rd), dot(b.xyz, rd), dot(c.xyz, rd));
  let inv = safeInv(d); let p = (-h.xyz - o) * inv; let q = (h.xyz - o) * inv; let s = vec2f(max(max3(min(p, q)), max(t0, limbFrom(h.w))), min(min3(max(p, q)), t1)); if (s.y <= s.x) { return vec3f(0.0); }
  let w = bitcast<u32>(extra[r + 4u].w); return vec3f(s, length(vec3f(vec3u(w, w >> 8u, w >> 16u) & vec3u(255u)) * (6.4 / 255.0) / (2.0 * h.xyz) * d));
}
/** may a ray meet person i between t0 and t1? */
fn near(i: u32, ro: vec3f, rd: vec3f, t0: f32, t1: f32) -> bool { let q = extra[BLK_CARRY + 1u + i]; let v = q.xyz - ro; let b = clamp(dot(v, rd), t0, min(t1, 1e6)); let e = v - rd * b; return dot(e, e) < q.w * q.w; }
fn people() -> u32 { return bitcast<u32>(extra[BLK_CARRY].x); }
fn carryTau(ro: vec3f, rd: vec3f, t0: f32, t1: f32) -> f32 { var tau = 0.0; if (blk.x >= t1) { return 0.0; } for (var i = 0u; i < people(); i++) { if (!near(i, ro, rd, max(t0, blk.x), t1)) { continue; } for (var j = 0u; j < CARRY_LIMBS; j++) { let k = i * CARRY_LIMBS + j; if (k + 1u == fromLimb) { continue; } let s = limbSpan(k, ro, rd, t0, t1); tau += s.z * (s.y - s.x); } } return tau; }
/** where a ray stops in one, if it does: drawn (t, limb + 1) */
fn carryHit(ro: vec3f, rd: vec3f, t0: f32, t1: f32) -> vec2f { var o = vec2f(t1, 0.0); if (blk.x >= t1) { return o; } for (var i = 0u; i < people(); i++) { if (!near(i, ro, rd, max(t0, blk.x), o.x)) { continue; } for (var j = 0u; j < CARRY_LIMBS; j++) { let k = i * CARRY_LIMBS + j; if (k + 1u == fromLimb) { continue; } let s = limbSpan(k, ro, rd, t0, o.x); if (s.z <= 0.0) { continue; } let t = s.x - log(1.0 - rnd()) / s.z; if (t < s.y) { o = vec2f(t, f32(k + 1u)); } } } return o; }
