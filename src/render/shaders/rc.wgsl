// World-space "2.5D" radiance cascades over the interior slab.
// Cascade n: probe grid dims (NX0>>n, NY, NZ0>>n), directions (D0<<n)^2 octahedral, interval [T_n, T_n + L0*4^n].
// Upper cascades (n>=1) write direction-pre-averaged merged radiance at the CHILD's angular resolution;
// cascade 0 writes fully merged radiance per direction. rcDice bakes c0 into the irradiance dice volume.

struct RcParams {
  cascade: u32, D: u32, Dout: u32, numProbes: u32,
  dims: vec3u, hasParent: u32,
  spacing: vec3f, tStart: f32,
  parentDims: vec3u, tLen: f32,
  parentSpacing: vec3f, diceBlend: f32,   // EMA factor toward this frame's dice bake (1 = no history)
};

@group(1) @binding(0) var<uniform> rc: RcParams;
@group(1) @binding(1) var<storage, read_write> outBuf: array<vec2u>;
@group(1) @binding(2) var<storage, read> parentBuf: array<vec2u>;
@group(1) @binding(3) var diceTex: texture_3d<f32>;
@group(1) @binding(4) var<uniform> dirOmega: array<vec4f, 16>;   // solid angle of each c0 direction texel (.x)
@group(1) @binding(5) var diceOut: texture_storage_3d<rgba16float, write>;
@group(1) @binding(6) var<storage, read> c0Read: array<vec2u>;

fn unpackRad(v: vec2u) -> vec4f { return vec4f(unpack2x16float(v.x), unpack2x16float(v.y)); }
fn packRad(v: vec4f) -> vec2u { return vec2u(pack2x16float(v.xy), pack2x16float(v.zw)); }
fn probePos(idx: vec3u, spacing: vec3f) -> vec3f { return RC_MIN + (vec3f(idx) + 0.5) * spacing; }

// Trace one radiance interval from `origin` along `dir` over [tStart, tStart+tLen]. Returns (radiance, transmittance).
fn traceInterval(origin: vec3f, dir: vec3f, tStart: f32, tLen: f32) -> vec4f {
  let ro = origin + dir * tStart;
  if (any(ro < WORLD_MIN) || any(ro > WORLD_MAX)) { return vec4f(0.0, 0.0, 0.0, 1.0); }
  let hit = traceClosest(ro, dir, tLen, 0u);
  if (hit.idx < 0) { return vec4f(0.0, 0.0, 0.0, 1.0); }
  return vec4f(hitRadiance(hit, ro + dir * hit.t), 0.0);
}

// ---- parent (n+1) lookup: bilinear in xz on the same y layer, optionally visibility weighted ----
var<workgroup> wgW: array<f32, 16>;     // [slot*4 + corner]
var<workgroup> wgIdx: array<u32, 16>;
var<workgroup> wgAcc: array<vec4f, 64>;

fn parentSetup(slot: u32, corner: u32, P: vec3f, py: u32) {
  var w = 0.0; var lin = 0u;
  if (rc.hasParent != 0u) {
    let g = (P.xz - RC_MIN.xz) / rc.parentSpacing.xz - 0.5;
    let i0 = vec2i(floor(g));
    let f = g - vec2f(i0);
    let cx = i32(corner & 1u); let cz = i32(corner >> 1u);
    let ci = clamp(i0 + vec2i(cx, cz), vec2i(0), vec2i(rc.parentDims.xz) - 1);
    w = select(1.0 - f.x, f.x, cx == 1) * select(1.0 - f.y, f.y, cz == 1);
    lin = (u32(ci.y) * rc.parentDims.y + py) * rc.parentDims.x + u32(ci.x);
    if ((frame.flags & FLAG_RC_VISWEIGHT) != 0u && w > 0.0) {
      let Q = probePos(vec3u(u32(ci.x), py, u32(ci.y)), rc.parentSpacing);
      let d = Q - P; let dist = length(d);
      if (dist > 1e-3 && occluded(P, d / dist, dist, 0u)) { w = 0.0; }
    }
  }
  wgW[slot * 4u + corner] = w;
  wgIdx[slot * 4u + corner] = lin;
}

fn parentRadiance(slot: u32, dirIdx: u32, dirCount: u32) -> vec3f {
  let b = slot * 4u;
  var w = vec4f(wgW[b], wgW[b + 1u], wgW[b + 2u], wgW[b + 3u]);
  let sum = w.x + w.y + w.z + w.w;
  if (sum < 1e-4) { return vec3f(0.0); }
  w /= sum;
  var L = vec3f(0.0);
  if (w.x > 0.0) { L += w.x * unpackRad(parentBuf[wgIdx[b] * dirCount + dirIdx]).xyz; }
  if (w.y > 0.0) { L += w.y * unpackRad(parentBuf[wgIdx[b + 1u] * dirCount + dirIdx]).xyz; }
  if (w.z > 0.0) { L += w.z * unpackRad(parentBuf[wgIdx[b + 2u] * dirCount + dirIdx]).xyz; }
  if (w.w > 0.0) { L += w.w * unpackRad(parentBuf[wgIdx[b + 3u] * dirCount + dirIdx]).xyz; }
  return L;
}

fn mergedInterval(P: vec3f, sd: vec2u, D: u32, slot: u32) -> vec3f {
  let dir = octDecode((vec2f(sd) + frame.rcJitter.xy) / f32(D));
  var r = traceInterval(P, dir, rc.tStart, rc.tLen);
  if (r.w > 0.0) {
    if (rc.hasParent != 0u) { return r.xyz + parentRadiance(slot, sd.y * D + sd.x, D * D); }
    return r.xyz + skyRadiance(dir);
  }
  return r.xyz;
}

// ---------------- upper cascades (n >= 1): one workgroup per probe; work item = (output texel, sub-direction) ----------------
@compute @workgroup_size(64)
fn rcUpper(@builtin(workgroup_id) wg: vec3u, @builtin(local_invocation_index) li: u32) {
  let pidx = wg;
  let P = probePos(pidx, rc.spacing);
  if (li < 4u) { parentSetup(0u, li, P, pidx.y); }
  workgroupBarrier();
  let probeLin = (pidx.z * rc.dims.y + pidx.y) * rc.dims.x + pidx.x;
  let nOut = rc.Dout * rc.Dout;
  let nItems = nOut * 4u;                       // always a multiple of 64 for n >= 1 with D0 = 4
  for (var base = 0u; base < nItems; base += 64u) {
    let w = base + li;
    let o = w >> 2u; let s = w & 3u;
    var L = vec3f(0.0);
    if (w < nItems) {
      let cd = vec2u(o % rc.Dout, o / rc.Dout);
      let sd = cd * 2u + vec2u(s & 1u, s >> 1u);
      L = mergedInterval(P, sd, rc.D, 0u);
    }
    wgAcc[li] = vec4f(L, 0.0);
    workgroupBarrier();
    if (s == 0u && w < nItems) {
      let sum = wgAcc[li].xyz + wgAcc[li + 1u].xyz + wgAcc[li + 2u].xyz + wgAcc[li + 3u].xyz;
      outBuf[probeLin * nOut + o] = packRad(vec4f(sum * 0.25, 0.0));
    }
    workgroupBarrier();
  }
}

// ---------------- cascade 0: (64 / D0^2) probes along x per workgroup, one direction per lane ----------------
@compute @workgroup_size(64)
fn rcC0(@builtin(workgroup_id) wg: vec3u, @builtin(local_invocation_index) li: u32) {
  let DD = rc.D * rc.D;
  let perWg = 64u / DD;
  let slot = li / DD;
  let d = li % DD;
  let px = wg.x * perWg + slot;
  let valid = px < rc.dims.x && slot < perWg;
  let pidx = vec3u(min(px, rc.dims.x - 1u), wg.y, wg.z);
  let P = probePos(pidx, rc.spacing);
  if (d < 4u && slot < 4u) { parentSetup(slot, d, P, pidx.y); }
  workgroupBarrier();
  if (!valid) { return; }
  let sd = vec2u(d % rc.D, d / rc.D);
  let L = mergedInterval(P, sd, rc.D, slot);
  let probeLin = (pidx.z * rc.dims.y + pidx.y) * rc.dims.x + pidx.x;
  outBuf[probeLin * DD + d] = packRad(vec4f(L, 0.0));
}

// ---------------- dice bake: c0 → 3D texture (7 slabs along Y: +x -x +y -y +z -z mean) ----------------
@compute @workgroup_size(64)
fn rcDice(@builtin(global_invocation_id) gid: vec3u) {
  let dims = rc.dims;
  let lin = gid.x;
  if (lin >= dims.x * dims.y * dims.z) { return; }
  let px = lin % dims.x; let py = (lin / dims.x) % dims.y; let pz = lin / (dims.x * dims.y);
  let probeLin = (pz * dims.y + py) * dims.x + px;
  let DD = rc.D * rc.D;
  var E0 = vec3f(0.0); var E1 = vec3f(0.0); var E2 = vec3f(0.0); var E3 = vec3f(0.0); var E4 = vec3f(0.0); var E5 = vec3f(0.0);
  var mean = vec3f(0.0);
  for (var d = 0u; d < DD; d++) {
    let L = unpackRad(c0Read[probeLin * DD + d]).xyz;
    let sd = vec2u(d % rc.D, d / rc.D);
    let dir = octDecode((vec2f(sd) + frame.rcJitter.xy) / f32(rc.D));
    let om = dirOmega[d].x;
    let Lw = L * om;
    mean += Lw;
    E0 += Lw * max(dir.x, 0.0);  E1 += Lw * max(-dir.x, 0.0);
    E2 += Lw * max(dir.y, 0.0);  E3 += Lw * max(-dir.y, 0.0);
    E4 += Lw * max(dir.z, 0.0);  E5 += Lw * max(-dir.z, 0.0);
  }
  mean *= (1.0 / (4.0 * 3.14159265));
  // short EMA against last frame's bake: the direction set is jittered per frame, this keeps bounce / meter / smoke ambient steady
  let k = rc.diceBlend;
  let c0i = vec3u(px, 0u * dims.y + py, pz); let c1i = vec3u(px, 1u * dims.y + py, pz); let c2i = vec3u(px, 2u * dims.y + py, pz); let c3i = vec3u(px, 3u * dims.y + py, pz);
  let c4i = vec3u(px, 4u * dims.y + py, pz); let c5i = vec3u(px, 5u * dims.y + py, pz); let c6i = vec3u(px, 6u * dims.y + py, pz);
  textureStore(diceOut, c0i, vec4f(mix(textureLoad(diceTex, c0i, 0).xyz, E0, k), 1.0));
  textureStore(diceOut, c1i, vec4f(mix(textureLoad(diceTex, c1i, 0).xyz, E1, k), 1.0));
  textureStore(diceOut, c2i, vec4f(mix(textureLoad(diceTex, c2i, 0).xyz, E2, k), 1.0));
  textureStore(diceOut, c3i, vec4f(mix(textureLoad(diceTex, c3i, 0).xyz, E3, k), 1.0));
  textureStore(diceOut, c4i, vec4f(mix(textureLoad(diceTex, c4i, 0).xyz, E4, k), 1.0));
  textureStore(diceOut, c5i, vec4f(mix(textureLoad(diceTex, c5i, 0).xyz, E5, k), 1.0));
  textureStore(diceOut, c6i, vec4f(mix(textureLoad(diceTex, c6i, 0).xyz, mean, k), 1.0));
}
