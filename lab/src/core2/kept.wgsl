// WHAT CELLS KEEP OF LIGHT, from 0.32 m up: three rows each. Words: J and the share of the sun seen · six colors · how many looks. The six are: of a cell of leaves, the light that lands on it from
// each way (a leaf reads it); of any other, what each of its sides sends back.
struct Fol { L: vec3f, T: vec3f };
fn keptRow(slot: u32, k: u32) -> vec4u { return bitcast<vec4u>(extra[BLK_LIGHT + (slot - 1u) * 3u + k]); }
fn keptSix(q0: vec4u, q1: vec4u, q2: vec4u, axis: u32, plus: bool) -> vec3f {
  if (axis == 0u) { if (plus) { return vec3f(unpack2x16float(q0.z), unpack2x16float(q0.w).x); } return vec3f(unpack2x16float(q0.w).y, unpack2x16float(q1.x)); }
  if (axis == 1u) { if (plus) { return vec3f(unpack2x16float(q1.y), unpack2x16float(q1.z).x); } return vec3f(unpack2x16float(q1.z).y, unpack2x16float(q1.w)); }
  if (plus) { return vec3f(unpack2x16float(q2.x), unpack2x16float(q2.y).x); } return vec3f(unpack2x16float(q2.y).y, unpack2x16float(q2.z));
}
fn cellColor(r: vec4u, r2: vec4u, axis: u32, plus: bool) -> vec3f { let w = (select(select(r2.x, r.w, axis == 1u), r.z, axis == 0u) >> select(0u, 16u, plus)) & 0xffffu; let v = vec3f(f32(w & 31u) / 31.0, f32((w >> 5u) & 63u) / 63.0, f32(w >> 11u) / 31.0); return v * v; }
fn cellMean(r: vec4u, r2: vec4u) -> vec3f { var c = vec3f(0.0); for (var a = 0u; a < 3u; a++) { c += cellColor(r, r2, a, true) + cellColor(r, r2, a, false); } return c / 6.0; }
/** what leaves of this cell send back off one side, and through */
fn cellLeaf(r: vec4u, r2: vec4u) -> array<vec3f, 2> { let a = cellMean(r, r2); let h = leafThrough(a, ((r.y >> 24u) & 1u) != 0u); let k = leafKeeps(a + h); return array<vec3f, 2>(a * k, h * k); }
/** what a cell sends back along -rd; k: the ray's way as its contents see it (its thickness each way, of length 1) */
fn cellSent(cell: u32, r: vec4u, k: vec3f, rd: vec3f) -> vec3f {
  let r2 = bitcast<vec4u>(extra[BLK_CELLS + (cell - 1u) * 2u + 1u]); let slot = r2.z; if (slot == 0u) { return vec3f(0.0); } let q0 = keptRow(slot, 0u);
  if ((r.x >> 24u) > 127u) { let J = vec3f(unpack2x16float(q0.x), unpack2x16float(q0.y).x); let sun = unpack2x16float(q0.y).y; if (sun <= 0.0 || max3(frame.sunE.xyz) <= 0.0) { return J; } let c = cellLeaf(r, r2); var s: Surf; s.albedo = c[0]; s.through = c[1]; return J + leafPhase(s, rd) * frame.sunE.xyz * sun; }
  let q1 = keptRow(slot, 1u); let q2 = keptRow(slot, 2u); let w = k * k; return keptSix(q0, q1, q2, 0u, rd.x < 0.0) * w.x + keptSix(q0, q1, q2, 1u, rd.y < 0.0) * w.y + keptSix(q0, q1, q2, 2u, rd.z < 0.0) * w.z;
}
/** the cell of 0.32 m at a place in steps of it, or 0 */
fn cellAt(c: vec3i) -> u32 { let b = vec3i(floor(vec3f(c) * 0.25)) - vec3i(floor(BLK_LO / BRICK + 0.5)); if (any(b < vec3i(0)) || any(b >= BLK_N)) { return 0u; } let p = blkWord(BLK_PAGE, u32(b.x + BLK_N.x * (b.y + BLK_N.y * b.z))); if (p == 0u) { return 0u; } let q = vec3u(c & vec3i(3)); return blkWord(BLK_BRICKS, (p - 1u) * 588u + 512u + q.x + 4u * (q.y + 4u * q.z)); }
fn folWalk(ro: vec3f, rd: vec3f, t1: f32, lit: bool) -> Fol {
  var o: Fol; if (lit) { o.L = blkL; o.T = vec3f(blkT); return o; }            /* (what nearest has found on its way) */
  let was = blk; blk = vec4f(0.0, 0.0, 1e9, 1e9); let hid = noLeaves; noLeaves = false; o.T = exp(-vec3f(1.0, 0.86, 1.1) * blockWalk(ro, rd, 0.0, t1, 1e9, false).tau); blk = was; noLeaves = hid; return o;
}
fn leafCell(x: vec3f) -> u32 { let c = cellAt(vec3i(floor(x / 0.32))); if (c == 0u || (bitcast<vec4u>(extra[BLK_CELLS + (c - 1u) * 2u]).x >> 24u) < 128u) { return 0u; } return c; }
/** what a leaf at x sends back along a ray that met it: as its cell does */
fn folBack(x: vec3f, rd: vec3f) -> vec3f { let c = leafCell(x); if (c == 0u) { return vec3f(0.0); } return cellSent(c, bitcast<vec4u>(extra[BLK_CELLS + (c - 1u) * 2u]), rd, rd); }
/** the same, the sun's own apart */
fn folJ(x: vec3f) -> vec3f { let c = leafCell(x); if (c == 0u) { return vec3f(0.0); } let slot = bitcast<vec4u>(extra[BLK_CELLS + (c - 1u) * 2u + 1u]).z; if (slot == 0u) { return vec3f(0.0); } let q = keptRow(slot, 0u); return vec3f(unpack2x16float(q.x), unpack2x16float(q.y).x); }
/** the light (all but the sun and the lamps) that lands on a leaf at x from the side n looks to; w = is there any kept. open: how much of that side its own twig leaves open; the rest sees that twig's leaves */
fn folLight(x: vec3f, n: vec3f, open: f32) -> vec4f {
  let g = x / 0.32 - 0.5; let b = vec3i(floor(g)); let f = g - floor(g); var E = vec3f(0.0); var G = vec3f(0.0); var ws = 0.0; let n2 = n * n;
  for (var k = 0; k < 8; k++) { let d = vec3i(k & 1, (k >> 1) & 1, k >> 2); let cell = cellAt(b + d); if (cell == 0u) { continue; } let r = bitcast<vec4u>(extra[BLK_CELLS + (cell - 1u) * 2u]); if ((r.x >> 24u) < 128u) { continue; } let r2 = bitcast<vec4u>(extra[BLK_CELLS + (cell - 1u) * 2u + 1u]); if (r2.z == 0u) { continue; }
    let q0 = keptRow(r2.z, 0u); let q1 = keptRow(r2.z, 1u); let q2 = keptRow(r2.z, 2u); if (q2.w == 0u) { continue; } let w3 = select(1.0 - f, f, d == vec3i(1)); let w = w3.x * w3.y * w3.z + 1e-3;
    let J = vec3f(unpack2x16float(q0.x), unpack2x16float(q0.y).x); let sv = unpack2x16float(q0.y).y; let cc = cellLeaf(r, r2); G += (J * PI + (cc[0] + cc[1]) * frame.sunE.xyz * (0.25 * sv)) * w;
    E += (keptSix(q0, q1, q2, 0u, n.x > 0.0) * n2.x + keptSix(q0, q1, q2, 1u, n.y > 0.0) * n2.y + keptSix(q0, q1, q2, 2u, n.z > 0.0) * n2.z) * w; ws += w; }
  if (ws <= 0.0) { return vec4f(0.0); } return vec4f(E / ws * open + G / ws * (1.0 - open), 1.0);
}
fn folHas(x: vec3f) -> bool { let b = vec3i(floor(x / 0.32 - 0.5)); for (var k = 0; k < 8; k++) { let c = cellAt(b + vec3i(k & 1, (k >> 1) & 1, k >> 2)); if (c != 0u && (bitcast<vec4u>(extra[BLK_CELLS + (c - 1u) * 2u]).x >> 24u) > 127u) { return true; } } return false; }
