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
  // Per-frame schedule (rc.ts writes these 16 bytes every frame; everything above only on config change):
  //   x = phase, y = period — this frame the upper-cascade kernel traces the probes whose linear index ≡ phase (mod period), an interleaved 1/period of the
  //       level, round-robin, so every probe is refreshed exactly once per `period` frames at constant work per frame (period 1 = the whole level, today's
  //       3-D dispatch untouched); z = the dice bake's blend this frame as f32 bits (= diceBlend, or 1.0 for the first bake of a fresh / re-primed cache so
  //       it never crawls up from a black history); w = chunks per probe — the probe's (texel, sub-direction) work items are split across this many
  //       workgroups (each redoes the 4-corner parent setup) purely for occupancy: c3 is 80 probes × 1024 items, i.e. 5120 threads looping 16× on a GPU
  //       with about that many lanes. Same items, same per-texel reduction order → the same bits for any chunk count.
  sched: vec4u,
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
  return vec4f(hitRadiance(hit, ro + dir * hit.t, ro), 0.0);
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

fn parentRadiance(slot: u32, dirIdx: u32, dirCount: u32) -> vec4f {
  let b = slot * 4u;
  var w = vec4f(wgW[b], wgW[b + 1u], wgW[b + 2u], wgW[b + 3u]);
  let sum = w.x + w.y + w.z + w.w;
  if (sum < 1e-4) { return vec4f(0.0); }
  w /= sum;
  var L = vec4f(0.0);
  if (w.x > 0.0) { L += w.x * unpackRad(parentBuf[wgIdx[b] * dirCount + dirIdx]); }
  if (w.y > 0.0) { L += w.y * unpackRad(parentBuf[wgIdx[b + 1u] * dirCount + dirIdx]); }
  if (w.z > 0.0) { L += w.z * unpackRad(parentBuf[wgIdx[b + 2u] * dirCount + dirIdx]); }
  if (w.w > 0.0) { L += w.w * unpackRad(parentBuf[wgIdx[b + 3u] * dirCount + dirIdx]); }
  return L;
}

/// .xyz = the radiance along a direction, merged with the cascades above as ever; .w = HOW MUCH OF IT IS THE SKY'S, by luminance (the record's fourth
/// number, which nothing used). The sky is added where a ray of the top cascade meets nothing, from a probe metres from the one that asked, so no ray
/// here can say whose sky it is: the dice bake takes a portal's share of it out, at the probe the light is for (rcDice).
fn mergedInterval(P: vec3f, sd: vec2u, D: u32, slot: u32) -> vec4f {
  let dir = octDecode((vec2f(sd) + frame.rcJitter.xy) / f32(D));
  var r = traceInterval(P, dir, rc.tStart, rc.tLen);
  if (r.w > 0.0) {
    if (rc.hasParent != 0u) { let up = parentRadiance(slot, sd.y * D + sd.x, D * D); return vec4f(r.xyz + up.xyz, up.w); }
    let sky = skyRadiance(dir); return vec4f(r.xyz + sky, luminance(sky));
  }
  return vec4f(r.xyz, 0.0);
}

// ---------------- upper cascades (n >= 1): work item = (output texel, sub-direction) ----------------
// Dispatch shapes (rc.ts): period 1 and 1 chunk → (dims.x, dims.y, dims.z), one workgroup per probe, wg = the probe index — the original path;
// otherwise → (ceil(numProbes / period) × chunks, 1, 1): wg.x / chunks picks the scheduled probe (linear index s·period + phase, x-major like
// probeLin), wg.x % chunks the slice of its items. A workgroup past the level's last probe leaves as a whole before any barrier (wg is uniform).
@compute @workgroup_size(64)
fn rcUpper(@builtin(workgroup_id) wg: vec3u, @builtin(local_invocation_index) li: u32) {
  let nOut = rc.Dout * rc.Dout;
  let nItems = nOut * 4u;                       // always a multiple of 64 for n >= 1 with D0 = 4
  let period = max(rc.sched.y, 1u); let chunks = max(rc.sched.w, 1u);
  let scheduled = period > 1u || chunks > 1u;
  let lin = select(0u, (wg.x / chunks) * period + rc.sched.x, scheduled);
  if (scheduled && lin >= rc.numProbes) { return; }
  let pidx = select(wg, vec3u(lin % rc.dims.x, (lin / rc.dims.x) % rc.dims.y, lin / (rc.dims.x * rc.dims.y)), scheduled);
  let per = select(nItems, ((nItems / 64u + chunks - 1u) / chunks) * 64u, scheduled);   // whole 64-item rows per chunk, so a texel's 4 sub-direction lanes stay in one workgroup iteration
  let i0 = select(0u, (wg.x % chunks) * per, scheduled);
  let i1 = min(nItems, i0 + per);
  if (i0 >= i1) { return; }
  let P = probePos(pidx, rc.spacing);
  if (li < 4u) { parentSetup(0u, li, P, pidx.y); }
  workgroupBarrier();
  let probeLin = (pidx.z * rc.dims.y + pidx.y) * rc.dims.x + pidx.x;
  for (var base = i0; base < i1; base += 64u) {
    let w = base + li;
    let o = w >> 2u; let s = w & 3u;
    var L = vec4f(0.0);
    if (w < nItems) {
      let cd = vec2u(o % rc.Dout, o / rc.Dout);
      let sd = cd * 2u + vec2u(s & 1u, s >> 1u);
      L = mergedInterval(P, sd, rc.D, 0u);
    }
    wgAcc[li] = L;
    workgroupBarrier();
    if (s == 0u && w < nItems) {
      let sum = wgAcc[li] + wgAcc[li + 1u] + wgAcc[li + 2u] + wgAcc[li + 3u];
      outBuf[probeLin * nOut + o] = packRad(sum * 0.25);
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
  outBuf[probeLin * DD + d] = packRad(L);
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
  var S: array<f32, 7>;   // the sky's part of each face's light, and of the mean's, by luminance (the record's fourth number: mergedInterval)
  for (var d = 0u; d < DD; d++) {
    let rec = unpackRad(c0Read[probeLin * DD + d]); let L = rec.xyz;
    let sd = vec2u(d % rc.D, d / rc.D);
    let dir = octDecode((vec2f(sd) + frame.rcJitter.xy) / f32(rc.D));
    let om = dirOmega[d].x;
    let Lw = L * om;
    mean += Lw;
    E0 += Lw * max(dir.x, 0.0);  E1 += Lw * max(-dir.x, 0.0);
    E2 += Lw * max(dir.y, 0.0);  E3 += Lw * max(-dir.y, 0.0);
    E4 += Lw * max(dir.z, 0.0);  E5 += Lw * max(-dir.z, 0.0);
    let sw = rec.w * om;
    S[0] += sw * max(dir.x, 0.0); S[1] += sw * max(-dir.x, 0.0); S[2] += sw * max(dir.y, 0.0); S[3] += sw * max(-dir.y, 0.0); S[4] += sw * max(dir.z, 0.0); S[5] += sw * max(-dir.z, 0.0); S[6] += sw;
  }
  // DAYLIGHT COUNTED ONCE (common.wgsl LIGHT_PORTAL): the sky this probe sees out of an opening that is lit as a portal is the portal's to send, and comes
  // out of the probe's light here. HOW MUCH: what the portal sends each face with nothing in the way (its own sum, portalFaces: sixteen directions are
  // too few for a probe to find a doorway by) × the share of the opening that is open now, and never more than the sky the probe's rays really brought
  // that face (a wall across the yard had its part already). In the hue of the sky toward the opening. A portal the probe's cell cannot see (its reach
  // mask) takes nothing. `gave` = what came out of the +y face, by luminance: the place field reads that face as the light that falls on a floor and
  // must read the same light whoever carries it (place_field.wgsl reading; the slab's fourth number).
  var gave = 0.0;
  if (frame.placeCfg.y > 0.5) {
    let P = probePos(vec3u(px, py, pz), rc.spacing);
    var V: array<vec3f, 7>; var T: array<f32, 7>;
    for (var i = u32(frame.placeCfg.y) - 1u; i < frame.numLights; i++) {
      let li = lights[i];
      if (!lightPortal(li) || !lightReaches(i, P)) { continue; }
      let dv = li.pos - P; let dist = length(dv);
      if (dist > li.range || dot(dv, li.dir) >= 0.0) { continue; }   // (out of its range, or behind it)
      let k0 = portalArea(li) * li.cosInner * lightWindow(dist, li.range); let sky = skyRadiance(dv / max(dist, 1e-4)); let hue = sky / max(luminance(sky), 1e-9);
      let F = portalFaces(li, P);
      for (var f = 0; f < 7; f++) { let p = k0 * F[f]; V[f] += hue * p; T[f] += p; }
    }
    E0 = max(E0 - V[0] * min(1.0, S[0] / max(T[0], 1e-9)), vec3f(0.0)); E1 = max(E1 - V[1] * min(1.0, S[1] / max(T[1], 1e-9)), vec3f(0.0));
    let floorLight = E2; E2 = max(E2 - V[2] * min(1.0, S[2] / max(T[2], 1e-9)), vec3f(0.0)); gave = luminance(floorLight - E2); E3 = max(E3 - V[3] * min(1.0, S[3] / max(T[3], 1e-9)), vec3f(0.0));
    E4 = max(E4 - V[4] * min(1.0, S[4] / max(T[4], 1e-9)), vec3f(0.0)); E5 = max(E5 - V[5] * min(1.0, S[5] / max(T[5], 1e-9)), vec3f(0.0));
    mean = max(mean - V[6] * min(1.0, S[6] / max(T[6], 1e-9)), vec3f(0.0));
  }
  mean *= (1.0 / (4.0 * 3.14159265));
  // short EMA against last frame's bake: the direction set is jittered per frame, this keeps bounce / meter / smoke ambient steady. sched.z carries this
  // frame's factor (rc.ts): the configured diceBlend, except 1.0 on the first bake of a fresh or re-primed cache — no history worth blending toward
  let k = bitcast<f32>(rc.sched.z);
  let c0i = vec3u(px, 0u * dims.y + py, pz); let c1i = vec3u(px, 1u * dims.y + py, pz); let c2i = vec3u(px, 2u * dims.y + py, pz); let c3i = vec3u(px, 3u * dims.y + py, pz);
  let c4i = vec3u(px, 4u * dims.y + py, pz); let c5i = vec3u(px, 5u * dims.y + py, pz); let c6i = vec3u(px, 6u * dims.y + py, pz);
  textureStore(diceOut, c0i, vec4f(mix(textureLoad(diceTex, c0i, 0).xyz, E0, k), 1.0));
  textureStore(diceOut, c1i, vec4f(mix(textureLoad(diceTex, c1i, 0).xyz, E1, k), 1.0));
  { let was = textureLoad(diceTex, c2i, 0); textureStore(diceOut, c2i, vec4f(mix(was.xyz, E2, k), mix(was.w, gave, k))); }   // (the fourth number of THIS slab: what a portal took out of the face, eased as the light is; 0 with no portal lit)
  textureStore(diceOut, c3i, vec4f(mix(textureLoad(diceTex, c3i, 0).xyz, E3, k), 1.0));
  textureStore(diceOut, c4i, vec4f(mix(textureLoad(diceTex, c4i, 0).xyz, E4, k), 1.0));
  textureStore(diceOut, c5i, vec4f(mix(textureLoad(diceTex, c5i, 0).xyz, E5, k), 1.0));
  textureStore(diceOut, c6i, vec4f(mix(textureLoad(diceTex, c6i, 0).xyz, mean, k), 1.0));
}
