# Pairwise-MIS derivations for the ReSTIR merges — UNVERIFIED DRAFTS

Three independent derivations (framings: GRIS-theory-first, course-notes
recipe, implementation-first) of pairwise-MIS weights for this codebase's
exact DI/GI merge scheme, produced 2026-07-29 by a multi-agent workflow.

**Status: the adversarial verification and synthesis stages never ran**
(account spend limit); no formula below has been checked. Before
implementing: run the verify/synthesize stages, or hand-check at minimum
the partition-of-unity conditions, proxy-target zero-sets, the finalize
denominator change, and the GI Jacobian placement.

Acceptance gate for the implementation (task: pairwise MIS): combined
temporal+spatial must beat spatial-only-6-taps on relRmse with
|relBias| <= 0.05 via __compareToReference; with taps=0 and temporal off
it must reduce algebraically to the current fresh-RIS estimator.

## Derivation 1

### di_formulas

SOURCES per pixel: canonical = fresh reservoir from generateReservoir (unchanged internally; its own finalize wSum/(M*targetPdf) stays valid because candidates are uniform over ALL lights, i.e. constant MIS 1/M with full support). Confidence c_c = can.M (~8). Neighbours j=1..k = {temporal at pp if restirTemporal && compat pass && prev.M>0} ∪ {each spatial tap at qp passing bounds+depth/normal compat && prev.M>0}, with confidence c_j = min(prev.M, cap) (cap 20 temporal, 10 spatial, as today). Cn = Σ c_j. The source list is decided ONLY from compat tests + prev.M>0 (never from prev.W or targets) — it must be independent of sample values. A neighbour with prev.W==0 STAYS in the list.

TARGET FUNCTIONS. True canonical target (used for resampling weights and finalize): p̂_c(y) = risTarget(m, h.n, normalize(y.samplePos-h.p), radianceFromLight(y.lightIdx, h.p, y.samplePos)) * select(1, fVis, y.lightIdx==0) — exactly the `tp` mergeReservoir computes today, with OUR pixel's fVis. Neighbour-j target used ONLY inside MIS weights, the albedo-less proxy evaluated at the neighbour's reconstructed geometry: q_j(y) = max(dot(n_j, normalize(y.samplePos - x_j)), 0) * luminance(radianceFromLight(y.lightIdx, x_j, y.samplePos)), where x_j = prev-frame world position reconstructed from (qp, prevNormalDepth.w = ray distance) via prevInvViewProj (+ prevCamPos): x_j = prevCamPos + normalize(unproject(qp) - prevCamPos) * pnd.w, and n_j = pnd.xyz. No flashVis term (it is in [0.08,1], never zero — see proxy validity). Shift map for DI is the identity on the world-space light sample, so all Jacobians are 1.

MIS WEIGHTS (confidence-weighted pairwise balance heuristic, canonical-anchored, pair budget β_j = c_j/Cn with canonical confidence split c_c·β_j per pair; after simplification):
  Neighbour j, evaluated at its own sample X_j:
    m_j(X_j) = c_j·q_j(X_j) / (Cn·q_j(X_j) + c_c·p̂_c(X_j))
  Canonical, evaluated at its sample X_c:
    m_c(X_c) = Σ_j  (c_j·c_c/Cn) · p̂_c(X_c) / (Cn·q_j(X_c) + c_c·p̂_c(X_c))
  (If Cn == 0, m_c = 1.) Partition check: per pair, m_j(y) + α_j(y) = c_j/Cn for every y where the pair denominator > 0, so Σ_j m_j + m_c = 1; where the pair denominator is 0 both p̂_c and q_j vanish, which only happens off supp_c where the contribution is annihilated by p̂_c anyway. Support check: m_j ∝ q_j vanishes exactly off supp_j (proxy zero-set = true zero-set); m_c ∝ p̂_c vanishes off supp_c. Sanity: k=1 gives the exact two-source balance heuristic c_1 p̂_1/(c_1 p̂_1 + c_c p̂_c); equal targets give m_i = c_i/(Cn+c_c) exactly.

RESAMPLING WEIGHTS (what replaces `w = tp * other.W * other.M`):
  Canonical: w_c = m_c(X_c) · p̂_c(X_c) · W_c  where W_c comes from finalizing the fresh reservoir FIRST (equivalently w_c = m_c · can.wSum / can.M, since p̂_c(X_c)·W_c = wSum_c/M_c — can.targetPdf is the stored true target at X_c).
  Neighbour j: w_j = m_j(X_j) · p̂_c(X_j) · W_j   (W_j = prev.W as stored; NO factor of M).
  WRS over these with a reservoirUpdate variant that does NOT increment M; the winner's stored targetPdf = the TRUE p̂_c at the winner.

FINALIZE (replaces wSum/(M*targetPdf) for MERGED reservoirs only): W = wSum / targetPdf (safeDiv, 0 if targetPdf<=1e-9). M no longer appears — it is carried purely as confidence: res.M = c_c + Cn; carry.M = c_c + c_t. generateReservoir's internal finalize keeps its old wSum/(M*tp) form.

WHICH TARGETS ARE EVALUATED WHERE (per neighbour j, 3 evals total, O(k)): (1) p̂_c(X_j) — true canonical target at neighbour's sample (already computed by today's mergeReservoir); (2) q_j(X_j) — proxy at neighbour's own sample, needs (x_j, n_j); (3) q_j(X_c) — proxy of neighbour geometry at the CANONICAL's sample (needs only x_j, n_j and can.samplePos/lightIdx — computable even when prev.W==0, and it MUST be accumulated into m_c for every listed source regardless of prev.W). Plus p̂_c(X_c) once = can.targetPdf (already stored). NEVER substitute the neighbour's stored targetPdf for q_j(X_j): the stored value is the true target (albedo + its own flashVis) while q_j(X_c) is the proxy; mixing the two different functions inside one pair breaks the partition of unity. Use the proxy at BOTH evaluation points.

CARRY (stored) RESERVOIR: a completely separate two-source merge {canonical, temporal} (k=1 exact balance heuristic), reusing evals (1),(2),(3) for the temporal source:
  m_t^carry(X_t) = c_t·q_t(X_t) / (c_t·q_t(X_t) + c_c·p̂_c(X_t))
  m_c^carry(X_c) = c_c·p̂_c(X_c) / (c_t·q_t(X_c) + c_c·p̂_c(X_c))
  w's and finalize as above; carry.M = c_c + c_t. If no temporal source, carry = finalized canonical. Do NOT snapshot the big merge mid-stream: its m weights presume all k+1 sources, so a {canonical,temporal} prefix of it does not satisfy partition of unity over two sources.

## Derivation 1

### gi_formulas

SOURCES: canonical = the single fresh path-trace candidate folded through giUpdate (count 1), c_c = 1 (= res.M after the fresh update); finalize it first with the existing finalizeGIReservoir (wSum/(1·tp), clamp 32) to get W_c; can.targetPdf = freshTarget. Neighbours as in DI (temporal + spatial taps, c_j = min(prev.M, cap), Cn = Σ c_j, list independent of W/targets).

MEASURES AND JACOBIANS. giTarget is a density in solid angle at the receiving point; the sample x2 is fixed in world space (reconnection shift), so mapping a sample between pixel domains multiplies densities by the reconnection Jacobian. Define A(x2, n2, x1) = max(dot(n2, normalize(x1-x2)), 0) / max(|x1-x2|^2, 1e-6) (area-to-solid-angle factor at x2; n2 = sampleNrm). Forward Jacobian neighbour→canonical: J_j = A(x2_j, n2_j, h.p) / A(x2_j, n2_j, x_j) — exactly giJacobian(prev_j, h.p) with its existing clamp(·, 0, 16), since prev_j.visiblePos = x_j' (use the reservoir's stored visiblePos, NOT the reconstructed x_j, for the Jacobian — it is the point the sample was actually connected from). Reverse Jacobian canonical→neighbour: J'_j = A(X_c.samplePos, X_c.sampleNrm, x_j) / A(X_c.samplePos, X_c.sampleNrm, h.p), same clamp.

TARGETS. True canonical target: p̂_c(y) = giTarget(m, h.n, v, normalize(y.samplePos - h.p), y.radiance) (the `tp` giMergePrev computes today). Neighbour proxy in the NEIGHBOUR's own solid-angle measure: q_j(y) = max(dot(n_j, normalize(y.samplePos - x_j)), 0) * luminance(y.radiance), with (x_j, n_j) reconstructed from prevNormalDepth + prevInvViewProj as in DI (0 if |y.samplePos - x_j|^2 <= 1e-6). Diffuse-only guarantees evalBSDF = (albedo/π)·cos with albedo channel-wise ≥ 0.02, so q_j has the same zero-set as the true neighbour target.

MIS WEIGHTS — Jacobian placement made explicit. m must compare densities in a COMMON measure. In the canonical pixel's measure, neighbour j's density at y is q_j(T_{c→j}(y))·J_{c→j}(y). Evaluated at the two points we need:
  Neighbour j at its own sample X_j (its density there is q_j(X_j)/J_j in our measure; multiply the pair ratio through by J_j):
    m_j(X_j) = c_j·q_j(X_j) / (Cn·q_j(X_j) + c_c·p̂_c(T_j(X_j))·J_j)
  Canonical at X_c:
    m_c(X_c) = Σ_j (c_j·c_c/Cn) · p̂_c(X_c) / (Cn·q_j(X_c)·J'_j + c_c·p̂_c(X_c))
  (Cn==0 ⇒ m_c = 1.) Here p̂_c(T_j(X_j)) is just tpc = giTarget(m, h.n, v, dir(h.p→prev_j.samplePos), prev_j.radiance).

RESAMPLING WEIGHTS (replaces giMergePrev's `tp * prev.W * prev.M * j`):
  w_j = m_j(X_j) · tpc · J_j · prev_j.W        — the Jacobian J_j appears ONCE as a factor of the resampling weight (as today) AND inside m_j's denominator as written above; it does NOT multiply q_j.
  w_c = m_c(X_c) · can.targetPdf · W_c.
  giUpdate variant with no M increment; winner keeps tp = true p̂_c at winner; visiblePos = h.p for reused winners as today.

FINALIZE: W = min(safeDiv(wSum, targetPdf), 32.0) — drop the M factor, keep the 32 clamp (its magnitude is comparable because the ~1/M that used to sit in finalize now lives in the m's; retune only if measurements say so). res.M = c_c + Cn.

CARRY: separate {canonical, temporal} merge exactly as DI, with Jacobians:
  m_t^carry(X_t) = c_t·q_t(X_t) / (c_t·q_t(X_t) + c_c·tpc_t·J_t)
  m_c^carry(X_c) = c_c·p̂_c(X_c) / (c_t·q_t(X_c)·J'_t + c_c·p̂_c(X_c))
  w_t = m_t^carry·tpc_t·J_t·prev_t.W; w_c = m_c^carry·can.targetPdf·W_c; W = min(wSum/tp, 32); carry.M = c_c + c_t. All four target/proxy/Jacobian values are shared with the big merge.

## Derivation 1

### proxy_target_validity

WHY A PROXY IS LEGAL AT ALL: in GRIS the resampling-MIS weights m_i are FREE functions — they affect variance only — subject to exactly two conditions: (a) Σ over sources that can produce y of m_i(y) = 1 for a.e. y in the union of supports where the final target p̂_c(y) > 0 (points with p̂_c = 0 get resampling weight m·p̂_c·W = 0, can never be selected, and the shaded integrand f = BSDF·L·V has supp(f) ⊆ supp(p̂_c) because albedo ≥ 0.02 and fVis ≥ 0.08 keep p̂_c > 0 wherever nDotL·radiance > 0, with real visibility applied by the shadow ray); (b) m_i(y) = 0 wherever source i cannot produce y. The m_i are NOT required to be built from the true target densities — the balance heuristic with the true targets is merely the low-variance choice. Therefore replacing the neighbour's true target p̂_j by any function q_j with THE SAME ZERO-SET keeps (a) and (b) intact (our pairwise construction gives partition of unity identically in the ratios, whatever positive function is plugged in) and is exactly as unbiased; only variance changes, and for diffuse materials q_j differs from p̂_j by a per-pixel, per-sample bounded positive factor (albedo luminance and flashVis in [0.08,1]), so the variance penalty is small.

EXACT SUPPORT CONDITIONS REQUIRED (all hold in this codebase):
1. Albedo floor: every albedo CHANNEL ≥ 0.02 > 0, so luminance(albedo·rad) > 0 ⟺ luminance(rad) > 0 (luminance has strictly positive channel weights). This is the "albedoRefl ≥ 0.02 so never zero" guarantee; it must be per-channel, not just on the scalar reflectance.
2. flashVis ∈ [0.08, 1] never reaches 0, so omitting it from the proxy cannot make q_j > 0 where p̂_j = 0.
3. The proxy MUST re-evaluate radianceFromLight AT the neighbour's position x_j (DI): spot-cone attenuation and the flashlight cone depend on the receiver position, and the neighbour's cone zeros are genuine support boundaries of its reservoir. Evaluating radiance at h.p instead would over-cover the neighbour's support ⇒ bias.
4. Asymmetry of the two failure directions: q_j UNDER-covering supp_j (proxy 0 at a point the neighbour could produce, e.g. from reconstruction error at grazing angles) is SAFE — the pair budget reroutes to the canonical through α_j and partition of unity still holds; it costs variance only. q_j OVER-covering supp_j (proxy > 0 where the neighbour can truly never produce the sample) leaks weight to a phantom source and darkens the result. The only over-coverage channels here are (i) epsilon-level reconstruction error at spot-cone boundaries (negligible, and the compat test rejects the geometric discontinuities that would make it large) and (ii) visibility-zeroed stored reservoirs — see edge cases; fix (ii) by not zeroing the carry.
5. Consistency: one fixed function q_j per source per merge, used at BOTH X_j and X_c. Never mix the neighbour's STORED targetPdf (true target, includes albedo and ITS flashVis) into m_j while using the proxy inside m_c's α_j term — two different "p̂_j"s break the pairwise partition of unity.
6. The canonical side of every ratio uses the TRUE p̂_c (we have our material and fVis), which is also the target used in the resampling weights and finalize — required so that {p̂_c > 0} is simultaneously the canonical support, the final-target support, and the set where partition of unity is needed.

## Derivation 1

### edge_cases

1. other.W == 0 early-return (currently skips adding M): remove. Source-list membership is decided by compat tests and prev.M > 0 ONLY. A listed neighbour with W == 0 contributes w_j = 0 (skip its WRS insertion) but (i) its c_j still counts in Cn, (ii) its α_j term is still accumulated into m_c — this needs only its geometry (x_j, n_j) and the canonical sample, not its own sample — and (iii) its c_j still goes into the output M. Under the naive scheme, skipping M on dead neighbours shrank the 1/M normalizer and brightened the image (a chunk of the +0.23/+0.32); under MIS, dropping α_j instead would darken. Both are wrong; keep the source listed.

2. tp <= 0 branch (canonical true target at the neighbour's sample is 0, i.e. the sample is outside OUR support): w_j = 0, do not insert into WRS; c_j stays in Cn and in output M; α_j still accumulated. No special-cased "r.M += other.M; return" anymore — M bookkeeping is unconditional per listed source. Equally, q_j(X_j) == 0 (grazing/reconstruction) ⇒ m_j = 0 ⇒ w_j = 0, same handling; this is the safe under-coverage direction. Guard every ratio with safeDiv(a,b) = select(0.0, a/b, b > 1e-9).

3. M caps and MIS validity: c_j = min(prev.M, cap) are pure confidence weights inside valid MIS weights, so ANY positive caps are now unbiased — the +0.23 "temporal M-cap bias" disappears because bias never came from capping per se but from the naive m ∝ M combine. Requirements: use the SAME capped c_j in m_j, in α_j, in Cn, and in the M written out; caps stay applied on READ (store uncapped-accumulated M as today); keep temporal 20 / spatial 10 as variance knobs, retunable freely.

4. Carry/store split: keep the policy (stored stream = fresh + temporal only; spatial feedback loop measured at relBias 1.28–1.93 stays excluded), but the carry must become its OWN two-source merge {canonical, temporal} with k=1 exact balance-heuristic weights (formulas above), reusing the already-computed evals. The current mid-stream snapshot (carry = res taken after the temporal mergeReservoir) is INVALID under MIS: the big merge's m_t presumes k+1 sources (its denominator contains Cn and the c_c/k-style split), so a two-source prefix of that stream does not satisfy partition of unity over {canonical, temporal} and would be biased. Randomness: the carry WRS uses its own rand() draws; correlation with the shading reservoir is harmless (each is independently unbiased).

5. Occluded-zeroed stored reservoirs: zeroing res.W after the shadow ray is EXACT for shading (it multiplies the realized contribution by the true visibility of the selected sample). But zeroing the stored carry (the `carry.W = 0` when the same sample lost the shadow ray, pathtrace.wgsl:427-431/548-551) shrinks the stored reservoir's effective support to (a random subset of) {p̂ > 0 ∧ V > 0}, while next frame's proxy q_t still covers {p̂ > 0} — over-coverage ⇒ Σ m < 1 on the gap ⇒ systematic darkening in the temporal chain that MIS cannot see. For strict unbiasedness: STORE THE UN-ZEROED CARRY (delete the carry-zeroing block; keep zeroing res). Trade-off: the old scheme's "don't hand a known-shadowed sample to next frame" variance saving is lost; if measurements show unacceptable variance/fireflies, re-enable the zeroing and book it as a known small negative bias — but measure the MIS scheme without it first, since temporal relBias ≈ 0 is the acceptance criterion.

6. Degenerate canonical (all fresh candidates invalid: wSum = 0 or targetPdf = 0): W_c = 0 via existing finalize, w_c = 0 through safeDiv; c_c STILL counts in every pair denominator and in output M (the canonical source existed and could have produced samples; removing it would over-weight neighbours). If Cn == 0 (no valid neighbour sources — first frame, disocclusion with all taps failing compat, temporal off + 0 taps): m_c = 1, res = carry = finalized canonical, which reduces exactly to plain RIS. Neighbour excluded from the list when prev.M == 0 or geometry reconstruction fails (sky / far depth) or compat fails — exclusion must not depend on prev.W or any target value.

7. GI clamps: giJacobian's clamp(·, 0, 16) and finalize's min(·, 32) are pre-existing bias/firefly knobs; keep them, use the SAME clamped J value in both m_j and w_j (consistency keeps the estimator well-defined; the clamps themselves remain the only intentional residual bias). The W clamp now applies to wSum/tp, whose magnitude matches the old wSum/(M·tp) because ~1/M moved into the m's.

8. WRS plumbing: reservoirUpdate/giUpdate increment M per call — the merges need variants that do NOT touch M (M is set once at the end to c_c + Cn, resp. c_c + c_t). The winner's stored targetPdf must be the true p̂_c at the winner, because finalize divides by it.

## Derivation 1

### wgsl_sketch

// ---- uniforms: add prevInvViewProj: mat4x4f, prevCamPos: vec3f ----
fn safeDiv(a: f32, b: f32) -> f32 { return select(0.0, a / b, b > 1e-9); }

fn prevWorldFromPixel(qp: vec2i, dims: vec2u, dist: f32) -> vec3f { // pnd.w is ray distance
  let uv = (vec2f(qp) + 0.5) / vec2f(dims);
  let h4 = U.prevInvViewProj * vec4f(uv.x*2.0-1.0, 1.0-uv.y*2.0, 0.5, 1.0);
  return U.prevCamPos + normalize(h4.xyz/h4.w - U.prevCamPos) * dist;
}

// WRS insert, no M bump (Reservoir and GIReservoir variants)
fn wrsAdd(r: ptr<function, Reservoir>, pos: vec3f, idx: u32, tp: f32, w: f32) {
  (*r).wSum += w;
  if (w > 0.0 && rand() * (*r).wSum < w) { (*r).samplePos = pos; (*r).lightIdx = idx; (*r).targetPdf = tp; }
}

// ---------------- DI ----------------
fn diProxy(nj: vec3f, xj: vec3f, idx: u32, sp: vec3f) -> f32 {
  let d = sp - xj; let dir = d * inverseSqrt(max(dot(d,d), 1e-4));
  return max(dot(nj, dir), 0.0) * luminance(radianceFromLight(idx, xj, sp));
}
fn diTrue(h: Hit, v: vec3f, m: Material, fVis: f32, idx: u32, sp: vec3f) -> f32 {
  let d = sp - h.p; let dir = d * inverseSqrt(max(dot(d,d), 1e-4));
  return risTarget(m, h.n, dir, radianceFromLight(idx, h.p, sp)) * select(1.0, fVis, idx == 0u);
}

// restirDirect merge phase (replaces temporal/spatial mergeReservoir + finalize):
var can = generateReservoir(...); finalizeReservoir(&can);          // unchanged fresh RIS
let cc = can.M;
// gather sources: srcs[0..k): {rsv (M capped -> c), xj, nj}; temporal = slot 0 if present.
// listed iff compat pass && prev.M > 0 && reconstruction ok. Cn = sum(c).
var mC = 1.0;
var qAtCan: array<f32, MAXK>;                                        // q_j(X_c), reused for carry
if (Cn > 0.0) {
  mC = 0.0;
  for (j) { qAtCan[j] = diProxy(nj, xj, can.lightIdx, can.samplePos);
            mC += (c_j * cc / Cn) * safeDiv(can.targetPdf, Cn*qAtCan[j] + cc*can.targetPdf); }
  if (can.targetPdf <= 0.0) { mC = 0.0; }                            // degenerate canonical
}
var res = emptyReservoir();
wrsAdd(&res, can.samplePos, can.lightIdx, can.targetPdf, mC * can.targetPdf * can.W);
for (j) {
  if (src.rsv.W > 0.0) {
    let tpc = diTrue(h, v, m, fVis, src.rsv.lightIdx, src.rsv.samplePos); // true target, OUR fVis
    if (tpc > 0.0) {
      let qj = diProxy(nj, xj, src.rsv.lightIdx, src.rsv.samplePos);
      let mj = safeDiv(c_j * qj, Cn*qj + cc*tpc);
      wrsAdd(&res, src.rsv.samplePos, src.rsv.lightIdx, tpc, mj * tpc * src.rsv.W);
    }
  } // W==0 or tpc==0: contributes nothing to wSum, but c_j already in Cn/mC/M
}
res.M = cc + Cn;  res.W = safeDiv(res.wSum, res.targetPdf);          // NO /M

var carry = can;                                                     // finalized canonical
if (temporal source t exists) {                                      // k=1 exact balance pair
  let ct = c_t;  let qtC = qAtCan[0];
  var c2 = emptyReservoir();
  let mcC = safeDiv(cc * can.targetPdf, ct*qtC + cc*can.targetPdf);
  wrsAdd(&c2, can.samplePos, can.lightIdx, can.targetPdf, mcC * can.targetPdf * can.W);
  if (t.rsv.W > 0.0 && tpc_t > 0.0) {                                // reuse tpc_t, qt from loop
    let mtC = safeDiv(ct * qt, ct*qt + cc*tpc_t);
    wrsAdd(&c2, t.rsv.samplePos, t.rsv.lightIdx, tpc_t, mtC * tpc_t * t.rsv.W);
  }
  c2.M = cc + ct;  c2.W = safeDiv(c2.wSum, c2.targetPdf);  carry = c2;
}
// shadow ray on res survivor: if occluded { res.W = 0; }  — do NOT zero carry.
reservoirCur[...] = carry;

// ---------------- GI ----------------
fn areaToSA(x2: vec3f, n2: vec3f, x1: vec3f) -> f32 {                // cos_at_x2 / d^2
  let d = x1 - x2; let d2 = max(dot(d,d), 1e-6);
  return max(dot(n2, d * inverseSqrt(d2)), 0.0) / d2;
}
fn giProxy(nj: vec3f, xj: vec3f, x2: vec3f, rad: vec3f) -> f32 {
  let d = x2 - xj; let d2 = dot(d,d); if (d2 <= 1e-6) { return 0.0; }
  return max(dot(nj, d * inverseSqrt(d2)), 0.0) * luminance(rad);
}
// canonical: fresh giUpdate as today (count 1), then finalizeGIReservoir -> W_c; cc = 1.
var mC = 1.0;
if (Cn > 0.0) { mC = 0.0;
  for (j) { let Jrev = clamp(areaToSA(can.samplePos, can.sampleNrm, xj)
                      / max(areaToSA(can.samplePos, can.sampleNrm, h.p), 1e-9), 0.0, 16.0);
            let qjC  = giProxy(nj, xj, can.samplePos, can.radiance) * Jrev;   // stored for carry
            mC += (c_j * cc / Cn) * safeDiv(can.targetPdf, Cn*qjC + cc*can.targetPdf); }
  if (can.targetPdf <= 0.0) { mC = 0.0; } }
var res = emptyGIReservoir();
giWrsAdd(&res, can.samplePos, can.sampleNrm, can.radiance, h.p, mC * can.targetPdf * can.W, can.targetPdf);
for (j with prev.W > 0.0) {
  let J   = giJacobian(prev_j, h.p);                                 // j->canonical, existing clamp 16
  let tpc = giTarget(m, h.n, v, normalize(prev_j.samplePos - h.p), prev_j.radiance);
  if (tpc > 0.0 && J > 0.0) {
    let qj = giProxy(nj, xj, prev_j.samplePos, prev_j.radiance);
    let mj = safeDiv(c_j * qj, Cn*qj + cc*tpc*J);                    // J beside p̂_c, not q_j
    giWrsAdd(&res, prev_j.samplePos, prev_j.sampleNrm, prev_j.radiance, h.p, mj * tpc * J * prev_j.W, tpc);
  }
}
res.M = cc + Cn;  res.W = min(safeDiv(res.wSum, res.targetPdf), 32.0);
// carry: {canonical, temporal} pair — mcC = safeDiv(cc*can.targetPdf, ct*qtC + cc*can.targetPdf);
//        mtC = safeDiv(ct*qt, ct*qt + cc*tpc_t*J_t); w_t = mtC*tpc_t*J_t*prev_t.W; W = min(wSum/tp, 32); M = cc+ct.
// occlusion ray: zero res only; store carry un-zeroed.

## Derivation 1

### predicted_outcome

Temporal-only: relBias +0.23 -> approximately 0 (expect |relBias| < 0.02-0.03; residuals only from the GI J-clamp/W-clamp and reconstruction epsilon; the two former bias sources — naive M-proportional combine and the W==0 M-skip — are gone, and removing the carry visibility-zeroing eliminates the temporal-chain darkening term). relRmse should be similar or slightly better than today's temporal-only. Spatial-only (6 taps): relBias -0.02..-0.05 -> approximately 0; relRmse stays around 0.65-0.74 or improves modestly (MIS down-weights geometrically mismatched neighbours instead of letting them dilute the estimate, at the cost of slightly conservative pairwise weights vs full Talbot MIS). Combined temporal+spatial: the headline change — relBias +0.32 -> approximately 0, and relRmse should drop from 1.08 to BELOW the spatial-only 0.65 (plausibly 0.4-0.6), because the two reuse streams now compound (temporal history raises effective sample count that spatial taps then share) instead of multiplying their biases; combined becomes the best configuration, which is the acceptance criterion. If the carry visibility-zeroing is kept for variance reasons, expect a small residual NEGATIVE bias (a few percent) in any temporal configuration; everything else unchanged. GI retains a small intentional residual from clamp(J,16) and min(W,32). Verify by measuring all three modes with the existing relBias/relRmse harness (per MEMORY.md: measure before calling it done) and A/B the carry-zeroing removal separately.

## Derivation 2

### di_formulas

SETUP (applies to DI and GI): the combine is restructured into (a) the UNCHANGED inner fresh RIS (generateReservoir, weights tp*total, old finalize W_c = wSum/(M_c*targetPdf)) producing the CANONICAL reservoir `canon`, and (b) an OUTER pairwise-MIS combine over k+1 techniques = {canon} + k admitted neighbours. k MUST be fixed before any streaming, so a gather pass runs the existing reprojection + depth/normal tests for the temporal pixel and all spatial taps first; a neighbour is admitted iff the geometry tests pass AND prev.M > 0 (NOT gated on W). k = number admitted (0..1+spatialTaps). M_i = the same caps as today, min(M, MCap) temporal / min(M, MCap*0.5) spatial, applied ONCE and used everywhere below. M_c = canon.M (= candidate loop count including invalid candidates — they are part of the uniform technique). The canonical is streamed LAST because its MIS weight depends on all k pairs. This inverts the current structure where `res` is seeded with the fresh reservoir and neighbours merge into it — flagged as a structural disagreement in edge_cases.

TARGET FUNCTIONS.
p_hat_c(y) — canonical target at OUR surface, identical to the current merge tp: risTarget(m, h.n, normalize(y.samplePos - h.p), radianceFromLight(y.lightIdx, h.p, y.samplePos)) * select(1, fVis, y.lightIdx == 0). This is BOTH the MIS canonical-side density and the resampling target stored in res.targetPdf.
p_tilde_i(y) — neighbour i's ALBEDO-LESS, FLASHVIS-LESS proxy at ITS surface (p_i, n_i): max(dot(n_i, normalize(y.samplePos - p_i)), 0) * luminance(radianceFromLight(y.lightIdx, p_i, y.samplePos)). n_i = prevNormalDepth[q_i].xyz. p_i is reconstructed (DI has no stored visiblePos): add U.prevInvViewProj and U.prevCamPos to uniforms; ndc = vec2((q_i.x+0.5)/dims.x*2-1, 1-2*(q_i.y+0.5)/dims.y); far = unproject(prevInvViewProj, vec4(ndc,1,1)); p_i = prevCamPos + prevNormalDepth[q_i].w * normalize(far.xyz - prevCamPos) (pnd.w stores the primary-ray hit t). The stored other.targetPdf is NEVER used inside the MIS ratios (see proxy_target_validity — mixing it with the proxy breaks the partition of unity). It also is not needed anywhere else; it remains stored only because the struct keeps it for the winner's own tp.

PAIR SHARE FUNCTION (one well-defined function of y per neighbour): alpha_i(y) = A/(A+B) with A = f32(k) * M_i * p_tilde_i(y), B = M_c * p_hat_c(y); alpha_i = 0 when A+B == 0. (k*M_i vs M_c is the RTXDI convention: the canonical's confidence is split across the k pairs.)

DEFENSIVE PAIRWISE MIS WEIGHTS:
- neighbour i, at its own sample y_i:  m_i(y_i) = alpha_i(y_i) / f32(k+1).
- canonical, at its sample y_c:        m_c = (1/f32(k+1)) * (1 + SUM_{i=1..k} (1 - alpha_i(y_c))), where alpha_i(y_c) reuses p_hat_c(y_c) = canon.targetPdf and evaluates p_tilde_i(y_c) at neighbour i's surface. These satisfy m_c + SUM m_i = 1 pointwise and m_c >= 1/(k+1) (defensive floor).

STREAMING WEIGHTS (a reservoirUpdate variant WITHOUT the implicit M += 1; M is bookkept manually):
- neighbour i: w_i = m_i(y_i) * p_hat_c(y_i) * W_i. The old factor *M_i is DELETED — the confidence now lives inside m_i. Streamed only if W_i > 0 and p_hat_c(y_i) > 0 and m_i > 0; res.M += M_i and the m_c term accumulate regardless.
- canonical (last): w_c = m_c * canon.targetPdf * canon.W  (equivalently m_c * canon.wSum / M_c).

FINALIZE (outer only): res.W = res.wSum / res.targetPdf — NO division by M. res.M = M_c + SUM M_i is carried purely as confidence for next frame's caps. Keep the old finalizeReservoir untouched for the inner fresh RIS.

CARRY: built as a SEPARATE k=1 pairwise combine over {canon, temporal} (alphas recomputed with k=1; the temporal neighbour's p_hat_c(y_t) and p_tilde_t evaluations are shared with the shading combine; the WRS coin flips must be independent draws). If the temporal input was not admitted, carry = canon finalized by the OLD rule (a plain fresh reservoir's W is already its unbiased contribution weight). The mid-stream snapshot `carry = res` is structurally incompatible with pairwise MIS and is removed.

WHERE TARGETS ARE EVALUATED, per admitted neighbour: p_hat_c(y_i) — in the merge, as today; p_tilde_i(y_i) — new, at reconstructed (p_i, n_i); p_tilde_i(y_c) — new, same surface, canonical's winning sample; p_hat_c(y_c) — free (canon.targetPdf). Net extra cost: two radianceFromLight-based proxy evals plus one position reconstruction per neighbour.

## Derivation 2

### gi_formulas

CANONICAL: the fresh bounce sample as its own reservoir: M_c = 1, targetPdf = freshTarget = giTarget(m, h.n, v, bounceDir, rad), W_c = luminance(bounceWeight*rad)/freshTarget (old finalize with M=1; W_c = 0 if freshTarget <= 0, in which case the canonical streams weight 0 but still anchors the partition).

NEIGHBOUR SURFACE: unlike DI, no reconstruction is needed — p_i = prev.visiblePos (the stored x1 the reservoir's current W is expressed for; this is exactly the point today's giJacobian already treats as the old connection endpoint), n_i = prevNormalDepth[q_i].xyz. Flag: this DI/GI asymmetry (DI reconstructs, GI reads visiblePos) is deliberate.

TARGETS.
p_hat_c(y) = giTarget(m, h.n, v, normalize(y.samplePos - h.p), y.radiance) — our true target, solid angle at OUR x1 (unchanged).
p_tilde_i(y) = max(dot(n_i, normalize(y.samplePos - p_i)), 0) / PI * luminance(y.radiance) — albedo-less Lambert proxy, solid angle at ITS x1. y.radiance is the stored outgoing radiance at x2; using it for a DIFFERENT outgoing direction is exactly valid only because materials are diffuse-only (Lo view-independent) — this proxy leans on that project constraint. The stored prev.targetPdf is NOT used in the ratios.

JACOBIAN PLACEMENT — the pair ratio must compare densities in ONE measure. Both targets above are solid-angle at different receivers; the shared measure is area at x2, and the conversion factors are reconnection Jacobians. Define the UNCLAMPED, epsilon-guarded shift Jacobian J(x2, n2; from -> to) = [cos(n2, dir(x2->to)) / |to - x2|^2] * [|from - x2|^2 / cos(n2, dir(x2->from))], returning 0 if the `from` cosine < 1e-6. Then:
- J_ic = J(prev.samplePos, prev.sampleNrm; prev.visiblePos -> h.p)  — today's giJacobian(prev, h.p) WITHOUT the 0..16 clamp.
- J_ci = J(canon.samplePos, canon.sampleNrm; h.p -> p_i) — the reverse shift of OUR fresh sample to the neighbour's x1; needs canon.sampleNrm (available).

PAIR SHARES (area measure, expressed via solid-angle targets; note which side carries the Jacobian):
- at the neighbour's sample y_i: alpha_i(y_i) = A/(A+B), A = f32(k) * M_i * p_tilde_i(y_i), B = M_c * p_hat_c(y_i) * J_ic.
- at the canonical sample y_c:   alpha_i(y_c) = A'/(A'+B'), A' = f32(k) * M_i * p_tilde_i(y_c) * J_ci, B' = M_c * canon.targetPdf.
(The Jacobian always multiplies the target that was evaluated in the OTHER pixel's solid-angle domain relative to where the sample's stored expression lives; algebraically both lines are the same area-measure ratio with a common positive factor divided out.) The clamp is deliberately ABSENT here: alpha is confined to [0,1] so an extreme Jacobian cannot firefly, it just drives alpha to 0 or 1; clamping inside alpha would break the exact function-of-y consistency between the two evaluations.

MIS WEIGHTS: m_i(y_i) = alpha_i(y_i)/f32(k+1); m_c = (1/f32(k+1)) * (1 + SUM_i (1 - alpha_i(y_c))).

STREAMING (giUpdate with weight below, count = M_i for the neighbour / M is bookkept as in DI):
- neighbour i: w_i = m_i(y_i) * p_hat_c(y_i) * W_i * clamp(J_ic, 0, 16). The *M_i factor of the current giMergePrev is DELETED. The 0..16 clamp survives ONLY on this streaming Jacobian (pre-existing fireflies guard, pre-existing small bias, unchanged).
- canonical (last): w_c = m_c * canon.targetPdf * W_c (identity shift, J = 1).

FINALIZE (outer): W = min(wSum / targetPdf, 32) — no /M, cap retained. Carry: same k=1 combine over {canon, temporal} as DI; no temporal => carry = canon with the old single-sample finalize. Post-merge real occlusion ray, zeroing, and the transfer-to-carry-iff-same-samplePos rule are unchanged.

## Derivation 2

### proxy_target_validity

Unbiasedness does NOT require the MIS weights to be built from the true technique densities. By the GRIS theorem (SIGGRAPH 2023 ReSTIR course, generalized RIS), the combine E[SUM_j m_j(Y_j) * p_hat_c(Y_j) * W_j] integrates p_hat_c exactly provided only: (1) partition of unity — SUM_j m_j(y) = 1 for every y in the integrand's support, and (2) support condition — m_j(y) = 0 wherever technique j has zero density of producing y. The functions inside the ratios are otherwise arbitrary; a wrong-but-consistent proxy changes variance, never the mean.

(1) holds BY CONSTRUCTION here regardless of the proxy: each pair uses one function alpha_i(y), the neighbour side gets alpha_i/(k+1) and the canonical side gets (1-alpha_i)/(k+1) of the SAME function, plus the defensive 1/(k+1); the sum telescopes to 1 identically. The critical consistency requirement this imposes: within a pair, p_tilde_i must be the SAME function at both evaluation points (same p_i, n_i, same current-frame light data, same clamping policy). This is precisely why the stored other.targetPdf must NOT be used as the neighbour density at y_i: it is albedo- and flashVis-laden and cannot be evaluated at y_c, so the m_c side would necessarily use a different function (the proxy), alpha at y_i and alpha at y_c would no longer be one function, SUM m_j(y) != 1 pointwise, and bias of order the albedo/flashVis mismatch returns. Proxy on BOTH sides of the neighbour term, true target on both sides of the canonical term, is exact. (Using the true canonical target against a proxy neighbour target is fine — asymmetry between techniques is allowed; only intra-pair inconsistency is not.)

(2) requires the proxy's zero set to COVER the true neighbour target's support boundary: p_tilde_i(y) must be 0 wherever the neighbour's stream could not have selected y. DI: true target = nDotL * luminance(rad * albedoRefl) * flashVisFactor. albedoRefl = max(mix(albedo,1,metallic), 0.02) >= 0.02 per channel, so luminance(rad*albedoRefl) = 0 iff luminance(rad) = 0 (rad >= 0); flashVis is clamped to [0.08, 1] and never 0; nDotL and the spot cone/attenuation live inside the proxy too (via nDotL and radianceFromLight). Hence proxy and true target have IDENTICAL zero sets at the same surface — condition (2) holds exactly. GI: true target = luminance(albedo/PI * cos * rad); proxy = cos/PI * luminance(rad). These share zero sets unless a per-channel-zero albedo nulls exactly the channels carrying all the radiance (e.g. pure-red albedo, pure-green radiance): then proxy > 0 where the true density is 0, m_i > 0 is assigned to a technique that never produces that y, the producing techniques' weights sum to < 1 there, and the result is BOUNDED DARKENING — never a firefly (the safe failure direction). With this project's broadband albedos it does not occur; worth one code comment, not a guard.

Residual, deliberately accepted support caveat (shared with RTXDI): the proxy evaluates CURRENT-frame light data while the temporal technique actually sampled under last frame's lights. Where the flashlight cone swung off a stored sample, proxy = 0 for a producible y — the sample is merely discarded (safe). Where it swung onto a region the old target excluded, proxy > 0 for an unproducible y — bounded energy loss for the frames a light moves. The defensive floor m_c >= 1/(k+1) additionally bounds the damage of any proxy misestimate: the canonical fresh technique covers the whole integrand support (uniform over all steady lights + flashlight for DI; cosine hemisphere for GI), so no region of the integrand can be starved.

## Derivation 2

### edge_cases

1. other.W == 0 early-return (currently skips adding M): replaced by the gather rule. Admission to the pair set is decided ONLY by the geometry tests and prev.M > 0 — never by W, because k and the pair set must not depend on sample values (a W=0 reservoir is a technique that ran and produced a null/occluded sample, not a technique that didn't exist; dropping it from k would leave SUM m < 1 and lose energy, and conditioning k on outcomes correlates the partition with the samples). Every admitted neighbour ALWAYS: counts in k, adds its capped M_i to res.M, and contributes its (1 - alpha_i(y_c)) term to m_c — that term needs only the neighbour's surface geometry, which is intact even in a zeroed reservoir. Its own sample is streamed only when W_i > 0 AND m_i(y_i) > 0 AND p_hat_c(y_i) > 0.

2. tp <= 0 branch (canonical target of the neighbour sample non-positive): do not stream (the weight would be 0), but still add M_i and still accumulate the m_c term — the m_c term is a function of y_c, not y_i, so it is unaffected. This preserves the current branch's M += other.M behaviour and extends it to the W==0 case above.

3. M caps and MIS validity: apply min(M, cap) exactly once per neighbour; the SAME capped M_i must appear in alpha_i(y_i), alpha_i(y_c), and the res.M accumulation (one local variable). Under pairwise MIS, ANY positive confidence values yield a valid partition, so the caps are now purely a variance/responsiveness knob — unlike the naive combine, where the cap itself was one of the bias sources (the measured +0.23 temporal). M_c = canon.M includes invalid fresh candidates; correct, since they belong to the uniform candidate technique and the inner finalize already accounts for them.

4. Carry/store split: KEPT — the spatial feedback loop it prevents is a cross-frame correlation problem that MIS weights do not address. But the current mid-stream snapshot (carry = res after temporal, before taps) is structurally incompatible with pairwise MIS, since every m depends on the final k. Carry becomes its own k=1 defensive pairwise combine over {canon, temporal}: alphas recomputed with k=1, target/proxy evaluations shared with the shading combine, WRS coin flips independent. If the temporal input was not admitted, carry = canon under the OLD finalize (W = wSum/(M*targetPdf)) — consistent, because a pure fresh reservoir's W is already its unbiased contribution weight, and next frame consumes only samplePos/lightIdx/M/W (+ GI fields), all well-defined under either finalize.

5. Occluded-zeroed stored reservoirs (W=0, wSum=0, sample and M intact): handled by rule 1 — they enter k, add M, and shape m_c via geometry-only terms; their dead sample never streams. This frame's post-merge occlusion zeroing of res, and the transfer-to-carry only when carry holds the same lightIdx/samplePos, are unchanged.

6. k == 0 (nothing admitted): skip the outer machinery entirely; res = canon with the old finalize. (Identical to running the outer path with m_c = 1: W = m_c*tp*W_c/tp = W_c.)

7. alpha denominator 0: alpha_i = 0, so m_i = 0 and the pair's full share (1-alpha = 1) flows to m_c.

8. Two finalizes must coexist: keep finalizeReservoir (divide by M*targetPdf) for generateReservoir's internal uniform-candidate RIS only; add finalizeReservoirMIS / finalizeGIReservoirMIS (divide by targetPdf alone; GI keeps min(.., 32)) for outer combines. Applying the /M finalize to an outer reservoir would double-count the confidences already inside the m's and re-introduce a systematic energy loss.

9. Retained, pre-existing bias sources (out of scope, list in comments): the GI streaming-Jacobian clamp(0,16) (absent from the MIS alphas, where it is unnecessary because alpha is in [0,1]); the GI W cap of 32; visibility-zeroing of stored reservoirs; current-frame target evaluation for the temporal technique (rule of proxy_target_validity). All are bounded and mostly darkening-direction.

10. Structural disagreements with the RTXDI/course recipe, all deliberate: (a) temporal and spatial fuse into ONE pairwise stream here (RTXDI runs a 2-technique balance-heuristic temporal pass, then a pairwise spatial pass), so the temporal reservoir is simply pair #1 — with the side effect that in temporal-only configs (k=1) the defensive floor caps history at ~1/2 selection mass; if temporal-only variance regresses unacceptably, the k==1 case may drop the defensive term (pure 2-technique balance heuristic: m_t = M_t*p_tilde_t/(M_t*p_tilde_t + M_c*p_hat_c), m_c = 1 - alpha evaluated at y_c), which is also unbiased. (b) RTXDI re-evaluates the neighbour's TRUE target from its G-buffer material; neighbour material is unavailable here, forcing the albedo-less proxy on both pair evaluations. (c) canonical streams last instead of seeding the reservoir. (d) DI neighbour positions must be reconstructed via new U.prevInvViewProj + U.prevCamPos from prevNormalDepth's stored ray-t; GI uses stored visiblePos instead.

## Derivation 2

### wgsl_sketch

// ---- new uniforms: prevInvViewProj: mat4x4f, prevCamPos: vec3f ----

fn prevPixelWorldPos(q: vec2i, t: f32, dims: vec2u) -> vec3f {
  let uv = (vec2f(q) + 0.5) / vec2f(dims);
  let ndc = vec2f(uv.x * 2.0 - 1.0, 1.0 - 2.0 * uv.y);
  var far = U.prevInvViewProj * vec4f(ndc, 1.0, 1.0);
  let dir = normalize(far.xyz / far.w - U.prevCamPos);
  return U.prevCamPos + dir * t;                       // pnd.w stores primary-ray t
}

// Albedo-less, flashVis-less DI proxy at a neighbour surface. Same zero set as risTarget.
fn diProxyTarget(pq: vec3f, nq: vec3f, lightIdx: u32, samplePos: vec3f) -> f32 {
  let delta = samplePos - pq;  let d2 = max(dot(delta, delta), 1e-4);
  let ndl = dot(nq, delta * inverseSqrt(d2));
  if (ndl <= 0.0) { return 0.0; }
  return ndl * luminance(radianceFromLight(lightIdx, pq, samplePos));
}

fn diCanonTarget(p: vec3f, n: vec3f, m: Material, fVis: f32, lightIdx: u32, samplePos: vec3f) -> f32 {
  let delta = samplePos - p;  let d2 = max(dot(delta, delta), 1e-4);
  let dir = delta * inverseSqrt(d2);
  return risTarget(m, n, dir, radianceFromLight(lightIdx, p, samplePos))
       * select(1.0, fVis, lightIdx == 0u);            // == today's merge tp
}

// reservoirUpdate WITHOUT the M += 1 (M bookkept by the caller)
fn wrsOuter(r: ptr<function, Reservoir>, pos: vec3f, idx: u32, tp: f32, w: f32) {
  (*r).wSum += w;
  if (w > 0.0 && rand() * (*r).wSum < w) { (*r).samplePos = pos; (*r).lightIdx = idx; (*r).targetPdf = tp; }
}
fn finalizeOuter(r: ptr<function, Reservoir>) {
  (*r).W = select(0.0, (*r).wSum / (*r).targetPdf, (*r).targetPdf > 1e-9);   // NO /M
}

struct NbCtx { buf: u32, M: f32, pos: vec3f, nrm: vec3f, temporal: bool }

fn restirDirect(...) -> vec3f {
  let fVis = flashTargetVis(h.p);
  var canon = generateReservoir(h.p, h.n, v, m, u32(U.restirCandidates), fVis);
  finalizeReservoir(&canon);                            // inner finalize unchanged

  // -------- gather pass: existing reprojection + geometry tests, no W gate ------
  var nb: array<NbCtx, 8>;  var k = 0u;                 // slot 0 temporal if admitted
  // temporal: same okDepth/okNormal tests; admit iff pass && prev.M > 0;
  //   NbCtx(bufIdx(pp), min(prev.M, U.restirMCap), prevPixelWorldPos(pp, pnd.w, dims), pnd.xyz, true)
  // each spatial tap: same tests at qp; admit iff pass && prev.M > 0;
  //   NbCtx(bufIdx(qp), min(prev.M, U.restirMCap * 0.5), prevPixelWorldPos(qp, qnd.w, dims), qnd.xyz, false)

  if (k == 0u) { /* res = canon; occlusion + store canon as carry; return */ }

  // -------- outer defensive pairwise-MIS combine (shading) ------
  var res = emptyReservoir();  res.M = canon.M;
  var mcSum = 1.0;                                      // defensive "1 +"
  var carry = emptyReservoir(); carry.M = canon.M; var mcSumT = 1.0; var hasT = false;
  for (var i = 0u; i < k; i++) {
    let prev = reservoirPrev[nb[i].buf];  let Mi = nb[i].M;
    let tpC  = diCanonTarget(h.p, h.n, m, fVis, prev.lightIdx, prev.samplePos);   // p_hat_c(y_i)
    let tpN  = diProxyTarget(nb[i].pos, nb[i].nrm, prev.lightIdx, prev.samplePos);// p_tilde_i(y_i)
    let tpNC = diProxyTarget(nb[i].pos, nb[i].nrm, canon.lightIdx, canon.samplePos);// p_tilde_i(y_c)
    // pair shares, k-technique combine
    let a  = f32(k) * Mi * tpN;   let b  = canon.M * tpC;
    let mi = select(0.0, (a / (a + b)) / f32(k + 1u), a + b > 0.0);
    let a2 = f32(k) * Mi * tpNC;  let b2 = canon.M * canon.targetPdf;
    mcSum += 1.0 - select(0.0, a2 / (a2 + b2), a2 + b2 > 0.0);
    if (prev.W > 0.0 && tpC > 0.0 && mi > 0.0) { wrsOuter(&res, prev.samplePos, prev.lightIdx, tpC, mi * tpC * prev.W); }
    res.M += Mi;
    if (nb[i].temporal) {                               // carry: independent k=1 combine
      hasT = true;
      let aT  = Mi * tpN;   let miT = select(0.0, (aT / (aT + b)) * 0.5, aT + b > 0.0);
      let aT2 = Mi * tpNC;  mcSumT += 1.0 - select(0.0, aT2 / (aT2 + b2), aT2 + b2 > 0.0);
      if (prev.W > 0.0 && tpC > 0.0 && miT > 0.0) { wrsOuter(&carry, prev.samplePos, prev.lightIdx, tpC, miT * tpC * prev.W); }
      carry.M += Mi;
    }
  }
  // canonical streams LAST
  let wC = (mcSum / f32(k + 1u)) * canon.targetPdf * canon.W;
  wrsOuter(&res, canon.samplePos, canon.lightIdx, canon.targetPdf, wC);
  finalizeOuter(&res);
  if (hasT) {
    wrsOuter(&carry, canon.samplePos, canon.lightIdx, canon.targetPdf, (mcSumT * 0.5) * canon.targetPdf * canon.W);
    finalizeOuter(&carry);
  } else { carry = canon; }                             // old finalize already applied
  // occlusion ray on res survivor; zero res (+carry iff same lightIdx & samplePos); shade; store carry — all unchanged
}

// ================= GI =================
fn giShiftJ(x2: vec3f, n2: vec3f, fromP: vec3f, toP: vec3f) -> f32 {  // UNCLAMPED, guarded
  let to = toP - x2;  let fr = fromP - x2;
  let dT2 = max(dot(to, to), 1e-6);  let dF2 = max(dot(fr, fr), 1e-6);
  let cF = abs(dot(n2, fr * inverseSqrt(dF2)));
  if (cF < 1e-6) { return 0.0; }
  return (abs(dot(n2, to * inverseSqrt(dT2))) / dT2) * (dF2 / cF);
}
fn giProxyTarget(pq: vec3f, nq: vec3f, x2: vec3f, rad: vec3f) -> f32 { // valid because diffuse-only
  let ndl = max(dot(nq, normalize(x2 - pq)), 0.0);
  return ndl * luminance(rad) / PI;
}

fn restirGI(...) -> vec3f {
  var canon = emptyGIReservoir();
  giUpdate(&canon, samplePos, sampleNrm, rad, h.p, luminance(bounceWeight * rad), freshTarget, 1.0);
  finalizeGIReservoir(&canon);                          // W_c, old finalize (M=1)
  // gather pass as DI, but neighbour pos = prev.visiblePos (no reconstruction), nrm from prevNormalDepth
  var res = emptyGIReservoir(); res.M = 1.0; var mcSum = 1.0;  // + carry twin as in DI
  for (var i = 0u; i < k; i++) {
    let prev = giPrev[nb[i].buf];  let Mi = nb[i].M;    // Mi = min(prev.M, cap)
    let tpC = giTarget(m, h.n, v, normalize(prev.samplePos - h.p), prev.radiance);
    let Jic = giShiftJ(prev.samplePos, prev.sampleNrm, prev.visiblePos, h.p);
    let Jci = giShiftJ(canon.samplePos, canon.sampleNrm, h.p, prev.visiblePos);
    let tpN  = giProxyTarget(prev.visiblePos, nb[i].nrm, prev.samplePos, prev.radiance);
    let tpNC = giProxyTarget(prev.visiblePos, nb[i].nrm, canon.samplePos, canon.radiance);
    let a  = f32(k) * Mi * tpN;         let b  = 1.0 * tpC * Jic;          // M_c = 1
    let mi = select(0.0, (a / (a + b)) / f32(k + 1u), a + b > 0.0);
    let a2 = f32(k) * Mi * tpNC * Jci;  let b2 = 1.0 * canon.targetPdf;
    mcSum += 1.0 - select(0.0, a2 / (a2 + b2), a2 + b2 > 0.0);
    if (prev.W > 0.0 && tpC > 0.0 && mi > 0.0) {
      giUpdateOuter(&res, prev.samplePos, prev.sampleNrm, prev.radiance, h.p,
                    mi * tpC * prev.W * clamp(Jic, 0.0, 16.0), tpC);       // clamp on STREAM only
    }
    res.M += Mi;
    // carry twin: same lines with k=1 factors when nb[i].temporal
  }
  giUpdateOuter(&res, canon.samplePos, canon.sampleNrm, canon.radiance, h.p,
                (mcSum / f32(k + 1u)) * canon.targetPdf * canon.W, canon.targetPdf);
  finalizeGIOuter(&res);                                // W = min(wSum / targetPdf, 32); NO /M
  // occlusion ray, zero/transfer rule, shading, store carry — unchanged
}

## Derivation 2

### predicted_outcome

Temporal-only: relBias +0.23 -> approximately 0 (expect |relBias| <= 0.05; the residual comes only from current-frame target evaluation of last frame's technique, i.e. the moving flashlight, and vanishes in static-light measurements). relRmse may tick UP versus the naive temporal combine, because the defensive k=1 floor caps history at ~1/2 selection mass (e.g. M_t=20 vs M_c=8 gives m_t ~ 0.36 instead of the naive 20/28 ~ 0.71); if that regression matters, the k==1 non-defensive fallback in edge_cases restores full history weight while staying unbiased.

Spatial-only (6 taps): relBias -0.02..-0.05 -> ~0, relRmse ~0.65-0.74 -> same or slightly better (~0.6-0.7); the correct confidence weighting removes the mild energy dilution without changing the sampling substance.

Combined temporal+spatial: this is the headline. relBias +0.32 -> ~0 (same <= 0.05 band as the components — with a proper partition of unity the biases no longer compound), and relRmse should drop from 1.08 to BELOW the spatial-only figure — expect roughly 0.4-0.55, since the temporal and spatial histories now add effective samples instead of fighting. The acceptance criterion is exactly the goal stated: combined strictly beats both temporal-only and spatial-only on rmse at ~zero bias. If combined bias is ~0 but rmse does NOT beat spatial-only, the first suspects are (1) k miscounted in the gather pass (a rejected tap still counted, or W==0 neighbours dropped from k), and (2) the carry combine reusing the shading combine's WRS randoms. If a small NEGATIVE bias remains everywhere (-0.02..-0.05), that is the expected residue of the retained clamps (GI Jacobian 16, GI W cap 32), visibility-zeroed stored reservoirs, and moving-light support mismatch — all bounded, all darkening-direction, none from the merge itself. GI should mirror DI in both respects. Measure per MEMORY.md: A/B each mode (temporal-only, spatial-only, combined) against the reference before calling it done.

## Derivation 3

### di_formulas

SETUP. Per pixel, the combine is one canonical technique plus k neighbour techniques, where k counts every neighbour that passes the GEOMETRY gates only (reprojection on-screen, clip.w>0, depth/normal compat). Gates may not depend on reservoir contents.

- Canonical c: R_c = generateReservoir(...) finalized with the OLD formula W_c = wSum/(M_c * targetPdf). This stays as-is: inside generateReservoir all candidates are iid uniform proposals, so the constant 1/M IS the balance heuristic there. R_c.targetPdf = p_c(X_c) (true target at our pixel, incl. our fVis). M_c = R_c.M (~8, includes invalid-candidate increments).
- Neighbour i (temporal or spatial tap): sample X_i = (lightIdx_i, samplePos_i), weight W_i, confidence M_i = min(stored M, cap) with cap = MCap for temporal, MCap*0.5 for spatial, applied BEFORE any use.

TARGET FUNCTIONS AND WHERE EACH IS EVALUATED.
1. p_c = TRUE canonical target: risTarget(m, h.n, dir, radianceFromLight(idx, h.p, samplePos)) * select(1, fVis, idx==0). Evaluated at X_i inside each merge (this is the existing `tp`; it becomes the resample-weight target and the stored targetPdf when X_i wins). p_c(X_c) is already stored in R_c.targetPdf. p_c is used ONLY for resampling weights and finalize — never inside MIS ratios.
2. qTilde_r = albedo-less PROXY target of receiver r = (x_r, n_r, fVis_r):
   qTilde_r(idx, sp) = max(dot(n_r, normalize(sp - x_r)), 0) * luminance(radianceFromLight(idx, x_r, sp)) * select(1, flashTargetVis(x_r), idx==0).
   The flashTargetVis factor is optional for correctness (it is clamped to [0.08,1], so it never changes the zero-set); include it for variance quality. The proxy is used for BOTH sides of every MIS ratio — including our own pixel — so albedo asymmetry never enters the ratios. Do NOT mix the stored other.targetPdf into MIS ratios: each technique must use ONE consistent function, so qTilde_i(X_i) is recomputed at the reconstructed neighbour receiver.
   Neighbour receiver: x_i reconstructed from (qp, pnd.w) via new uniforms prevInvViewProj + prevCamPos (pnd.w is ray DISTANCE, not NDC depth, so the camera origin is needed: x_i = prevCamPos + pnd.w * normalize(unprojectFar(qp) - prevCamPos)); n_i = pnd.xyz.

MIS WEIGHTS (confidence-weighted pairwise, canonical as defensive partner in every pair). True weights carry 1/k; the implementation streams them unnormalized (times k) and divides by k in finalize, which is legal because a uniform scale of all WRS weights does not change selection probabilities and finalize divides it back out.

For neighbour i at its own sample X_i:
  mu_i = M_i * qTilde_i(X_i) / (M_i * qTilde_i(X_i) + M_c * qTilde_c(X_i)),  mu_i = 0 if denominator <= 0 or qTilde_i(X_i) <= 0.
  m_i(X_i) = mu_i / k.
For the canonical at X_c, one pair term per gate-passing neighbour i (accumulated even when that neighbour's reservoir is null):
  pi_i = M_c * qTilde_c(X_c) / (M_c * qTilde_c(X_c) + M_i * qTilde_i(X_c)),  pi_i = 1 if M_i = 0 or the denominator is 0 or R_c has no winner (wSum==0, in which case it is irrelevant).
  m_c(X_c) = (1/k) * sum_i pi_i;  m_c = 1 when k = 0.
Partition of unity holds per pair: for any y, M_i qTilde_i(y)/(D) + M_c qTilde_c(y)/(D) = 1, and the (1/k)-average over the k pairs sums the whole technique set to 1.

RESAMPLE WEIGHTS fed to the WRS stream (unnormalized, i.e. times k):
  neighbour i:  w_i = mu_i * p_c(X_i) * W_i            <-- NOTE: no *M_i factor anymore; confidence lives inside mu_i
  canonical (inserted LAST, after all pair terms are known):  w_c = (sum_i pi_i) * p_c(X_c) * W_c   (= 1 * p_c(X_c) * W_c when k=0)
Winner keeps targetPdf = p_c at its own sample, exactly as today.

FINALIZE (replaces wSum/(M*targetPdf) for the COMBINED reservoirs only):
  W = wSum / (max(k, 1) * targetPdf),   0 if the denominator underflows. No M anywhere. M is still accumulated (M_out = M_c + sum_i M_i) but is pure confidence bookkeeping for next frame's caps and MIS weights.

Sanity reductions: k=0 gives W = W_c exactly (current no-reuse behaviour preserved). Temporal-only gives k=1, i.e. the exact two-technique confidence-weighted balance heuristic.

## Derivation 3

### gi_formulas

Same pairwise structure; the only differences are the shift Jacobian and the measure used inside the MIS ratios.

RECEIVERS. Our receiver is (h.p, h.n). The neighbour receiver is (prev.visiblePos, pnd.xyz): the code guarantees visiblePos is always the merging pixel's own h.p at store time, so it IS pixel q's previous surface point — no reconstruction needed for GI (pnd.xyz still supplies the receiver normal).

TARGETS.
1. p_c = TRUE canonical target in solid angle at our pixel: giTarget(m, h.n, v, dir, prev.radiance) — unchanged; evaluated at X_i in each merge (the existing tp) and at X_c (stored as R_c.targetPdf = freshTarget). Used only for resample weights / stored targetPdf / finalize.
2. Proxy in the SHARED AREA MEASURE at x2, for receiver r = (x_r, n_r) and GI sample y = (x2=sp, n2, Lo):
   qTildeGI_r(y) = max(dot(n_r, w), 0) * luminance(Lo) * |dot(n2, w)| / d2,  where w = normalize(sp - x_r), d2 = |sp - x_r|^2.
   (nDotL * luminance(Lo) is the albedo-less, view-independent proxy of giTarget — exact up to the albedo/pi factor for this project's diffuse-only materials; the |dot(n2,w)|/d2 factor converts each receiver's solid-angle density into the common area measure at x2, so ratios across receivers are meaningful. Ratios of same-measure densities are measure-invariant, so area measure inside MIS is compatible with solid-angle resample weights outside.)

JACOBIAN PLACEMENT — explicit. giJacobian(prev, h.p) = [cos(n2, toNew)/dNew2] * [dOld2/cos(n2, toOld)] converts the neighbour's contribution weight from solid angle at ITS receiver to solid angle at OURS, so it multiplies the RESAMPLE WEIGHT ONLY, exactly where the current code has it:
   w_i = mu_i * p_c(X_i) * W_i * giJacobian(prev, h.p)
The Jacobian does NOT appear inside mu_i or pi_i: the geometric factors it would contribute are already inside the two qTildeGI evaluations (each carries its own |dot(n2,w)|/d2), and MIS ratios must stay unclamped geometry. Keep the [0,16] clamp on the resample-weight Jacobian only (existing stability trade; documented residual bias).

MIS WEIGHTS (identical shape to DI):
  mu_i = M_i * qTildeGI_i(X_i) / (M_i * qTildeGI_i(X_i) + M_c * qTildeGI_c(X_i)),  where qTildeGI_i(X_i) uses receiver (prev.visiblePos, pnd.xyz) and qTildeGI_c(X_i) uses (h.p, h.n); zero on zero/invalid denominator.
  pi_i = M_c * qTildeGI_c(X_c) / (M_c * qTildeGI_c(X_c) + M_i * qTildeGI_i(X_c)),  where X_c = (canon.samplePos, canon.sampleNrm, canon.radiance) is the fresh bounce sample and qTildeGI_i(X_c) evaluates the neighbour receiver against OUR x2; pi_i = 1 if M_i = 0 / denominator 0 / canonical null.

CANONICAL. R_c = the single fresh candidate as its own finalized reservoir: giUpdate(weight = luminance(bounceWeight*rad), tp = freshTarget, count 1) then W_c = wSum/(1 * tp) = luminance(bounceWeight*rad)/freshTarget. Inserted last with w_c = (sum_i pi_i) * canon.targetPdf * W_c (Jacobian = 1: same receiver). With k=0 this reduces algebraically to bounceWeight*rad, preserving the code's documented no-reuse invariant.

FINALIZE: W = min(wSum / (max(k,1) * targetPdf), 32) — keep the 32 cap (documented residual bias), drop the M from the denominator.

The prev.M cap (MCap temporal, MCap*0.5 spatial) is applied before M_i is used anywhere, as today.

## Derivation 3

### proxy_target_validity

MIS weights never need to be the true targets. Unbiasedness of the pairwise combine needs exactly two properties of the functions used in the ratios: (a) each pair's two weights sum to 1 pointwise — automatic for the ratio form with ANY nonnegative q's, whatever their absolute scale or missing factors; and (b) the support condition: for every y that the canonical can produce, q_i(y) > 0 implies technique i could actually have produced y with W>0 (equivalently supp(qTilde_i) is a subset of supp(technique i)). If (b) failed at some y — proxy positive where the neighbour can never emit — the canonical's pair weight pi would be < 1 where it must be 1, and energy would be lost. The converse direction (technique support larger than proxy support) is harmless: such neighbour samples get mu_i = 0 and the canonical pair weight is 1 there, still summing to 1. Given (a) and (b), the q values themselves affect only variance, so dropping albedo is free.

DI: the support condition holds EXACTLY. A neighbour pixel's reservoir chain can emit any y with true target p_i(y) > 0 at that pixel (fresh uniform candidates there cover the full lightIdx x emitter-surface domain, and any tp>0 candidate can win), and can emit nothing with p_i(y) = 0 (samples only win/survive with tp > 0). The proxy's zero-set equals the true risTarget's zero-set: albedoRefl >= 0.02 componentwise and flashVis >= 0.08 are strictly positive bounded factors, and the luminance weights are all positive, so luminance(radiance*refl) = 0 iff luminance(radiance) = 0; both reduce to {nDotL <= 0 OR radiance-at-receiver = 0}. Hence supp(proxy) = supp(true target) = supp(technique), and the albedo-less proxy preserves unbiasedness exactly.

Two documented approximations remain, both industry-standard and both absent in a static measurement scene: (1) temporal targets are evaluated with CURRENT light uniforms at the previous surface (prev flash/guard-torch poses are not retained); under light motion the support sets can transiently disagree, giving bounded, frame-local error that vanishes when lights are static — which is the regime the relBias numbers are measured in. (2) GI only: a neighbour's fresh bounce can only ever land on x2 VISIBLE from its receiver, but qTildeGI cannot test visibility, so at points occluded from the neighbour the canonical is slightly underweighted — a small darkening confined to occlusion boundaries, the same approximation every shipping ReSTIR GI makes by excluding visibility from targets; the defensive pair form bounds the loss since m_c stays strictly positive.

Consistency requirement that motivated recomputing the neighbour target rather than reusing the stored one: the two evaluations inside one pair (at X_i and at X_c) must be the SAME function q_i, or the pair no longer sums to 1. The stored other.targetPdf includes the neighbour's albedo and its frame's flashVis while anything we can evaluate at X_c does not, so the stored value must not appear in MIS ratios; qTilde_i is recomputed at the neighbour receiver for both points, and for symmetry the proxy is also used on the canonical side of every ratio (the TRUE target is reserved for resample weights, stored targetPdf, and finalize).

## Derivation 3

### edge_cases

1. other.W == 0 (or wSum == 0) with the geometry gate passed: the technique EXISTS and produced a null sample; W_i already prices null outcomes into its expectation, so the deterministic pair term must still be charged. Rule: k += 1, canonSum += pi_i, res.M += M_i ALWAYS on gate pass; skip only the wSum insertion. The current early-return that skips adding M (and, implicitly, skips the technique) is exactly an overweighting of the surviving streams — branching m_c on neighbour nullness inflates the estimator by a factor of (1 + P(null)*m_i) pointwise, and is a large chunk of the measured +0.23/+0.32. pi_i is computable for null reservoirs because it depends only on neighbour geometry, M_i, and X_c — never on X_i. If M_i = 0 (never-written reservoir, garbage fields), pi_i = 1 by the M_i=0 guard, which is exactly neutral, so no special-casing is needed. Occlusion-zeroed reservoirs (W=0, M>0, sample fields intact) fall out correctly under the same rule.

2. tp = p_c(X_i) <= 0 (also the GI d2 <= 1e-6 degenerate-shift guard, and mu_i-denominator <= 0): per-SAMPLE degeneracies. Same rule: w_i = 0 but k, pi_i, and M_i are still counted. Only per-PIXEL geometry gates (bounds, clip.w, depth/normal compat) may remove a technique from k, because gating on sample-dependent quantities would make the technique set correlate with the sampled values.

3. M caps: capping confidence is always MIS-valid — M_i is a free positive weight that only shapes variance — PROVIDED one consistent value is used everywhere: apply min(M, MCap) / min(M, MCap*0.5) once at load, then use that same M_i in mu_i, pi_i, and the res.M accumulation. M no longer appears in the combined finalize, so the caps can no longer interact with the estimator normalization at all; they only steer the balance between techniques and next frame's caps.

4. Carry/store split: carry can no longer be a mid-stream snapshot of res, because MIS weights depend on the technique set (the 1/k and canonSum differ between {canonical, temporal} and {canonical, temporal, spatials}). Run TWO parallel WRS streams over shared loads: the temporal neighbour is inserted into both with the same mu_t and w_t (mu is k-independent; the 1/k lives in finalize); spatial taps feed res only; the canonical is inserted LAST into each with that stream's own canonSum; each finalizes with its own k (kCarry in {0,1}, so carry's divisor is always 1). The streams may pick different winners — fine; the occlusion-transfer already compares sample identity. Stored carry.M = M_c + M_t as today.

5. Occlusion-zeroing of the stored reservoir (visibility reuse): mechanics unchanged — shade-time shadow ray on res's survivor; zero res and transfer to carry iff carry holds the same sample. The final shading estimator f*V*W stays exactly unbiased for the shadowed integral (E[g(Y)W] = integral of g for any g, including g = f*V). The zeroing itself censors the stored stream in a visibility-correlated way that the MIS weights deliberately do not model; it can only remove energy from samples already found shadowed, is the same trade the current code makes on purpose, and is the main expected residual (slightly negative bias in penumbras) after the fix, alongside the GI Jacobian clamp and the GI W cap of 32.

6. k = 0 (reprojection off-screen, clip.w <= 0, all gates fail, or reuse disabled): res = carry = canonical alone; m_c = 1, finalize divisor 1, W = W_c — bit-for-bit the current no-reuse path.

## Derivation 3

### wgsl_sketch

// ---- uniforms: add prevInvViewProj : mat4x4f, prevCamPos : vec3f (DI reconstruction only)

fn prevWorldFromPixel(qp: vec2i, dims: vec2u, dist: f32) -> vec3f {
  let uv  = (vec2f(qp) + 0.5) / vec2f(dims);
  let ndc = vec2f(uv.x * 2.0 - 1.0, 1.0 - 2.0 * uv.y);
  let far = U.prevInvViewProj * vec4f(ndc, 1.0, 1.0);
  return U.prevCamPos + normalize(far.xyz / far.w - U.prevCamPos) * dist; // pnd.w is ray distance
}

fn diProxyTarget(x: vec3f, nrm: vec3f, idx: u32, sp: vec3f, fv: f32) -> f32 {
  let d  = sp - x;  let d2 = max(dot(d, d), 1e-4);
  let ndl = dot(nrm, d * inverseSqrt(d2));
  if (ndl <= 0.0) { return 0.0; }
  return ndl * luminance(radianceFromLight(idx, x, sp)) * select(1.0, fv, idx == 0u);
}

struct Mis { k: f32, canonSum: f32 }   // one per output stream (res, carry)

// canon is generateReservoir(...) finalized with the OLD wSum/(M*tp); canonProxy = diProxyTarget(h.p,h.n,canon.lightIdx,canon.samplePos,fVis) when canon.wSum>0 else 0
fn mergeReservoirMIS(r: ptr<function,Reservoir>, mis: ptr<function,Mis>, otherIn: Reservoir, mCap: f32,
                     nx: vec3f, nn: vec3f, nfv: f32, canon: Reservoir, canonProxy: f32,
                     p: vec3f, n: vec3f, v: vec3f, m: Material, fVis: f32) {
  var o = otherIn;  o.M = min(o.M, mCap);
  (*mis).k += 1.0;  (*r).M += o.M;                              // ALWAYS on gate pass
  var pi = 1.0;
  if (canon.wSum > 0.0 && o.M > 0.0) {
    let qN = diProxyTarget(nx, nn, canon.lightIdx, canon.samplePos, nfv);   // qTilde_i(X_c)
    let den = canon.M * canonProxy + o.M * qN;
    pi = select(1.0, canon.M * canonProxy / den, den > 0.0);
  }
  (*mis).canonSum += pi;
  if (o.M <= 0.0 || o.W <= 0.0) { return; }
  let d = o.samplePos - p;  let dir = d * inverseSqrt(max(dot(d,d),1e-4));
  let tp = risTarget(m, n, dir, radianceFromLight(o.lightIdx, p, o.samplePos))
         * select(1.0, fVis, o.lightIdx == 0u);                 // TRUE canonical target
  if (tp <= 0.0) { return; }
  let qS = diProxyTarget(nx, nn, o.lightIdx, o.samplePos, nfv); // qTilde_i(X_i)
  let qC = diProxyTarget(p,  n,  o.lightIdx, o.samplePos, fVis);// qTilde_c(X_i)
  let den = o.M * qS + canon.M * qC;
  if (qS <= 0.0 || den <= 0.0) { return; }
  let w = (o.M * qS / den) * tp * o.W;                          // mu_i * p_c(X_i) * W_i  (no *M_i)
  (*r).wSum += w;
  if (rand() * (*r).wSum < w) { (*r).samplePos = o.samplePos; (*r).lightIdx = o.lightIdx; (*r).targetPdf = tp; }
}

fn insertCanonical(r: ptr<function,Reservoir>, mis: Mis, canon: Reservoir) {
  let mC = select(1.0, mis.canonSum, mis.k > 0.0);              // unnormalized (x k)
  let w  = mC * canon.targetPdf * canon.W;                      // p_c(X_c) * W_c
  (*r).M += canon.M;  (*r).wSum += w;
  if (w > 0.0 && rand() * (*r).wSum < w) { (*r).samplePos = canon.samplePos; (*r).lightIdx = canon.lightIdx; (*r).targetPdf = canon.targetPdf; }
}

fn finalizeCombined(r: ptr<function,Reservoir>, k: f32) {
  let den = max(k, 1.0) * (*r).targetPdf;
  (*r).W = select(0.0, (*r).wSum / den, den > 1e-9);            // NO M in the denominator
}

// restirDirect flow:
//   canon = generateReservoir(...); finalizeReservoir(&canon);      // old finalize, unchanged
//   canonProxy = (canon.wSum > 0) ? diProxyTarget(h.p, h.n, canon.lightIdx, canon.samplePos, fVis) : 0
//   res = empty; carry = empty; misR = Mis(0,0); misC = Mis(0,0)
//   temporal gate pass: nx = prevWorldFromPixel(pp, dims, pnd.w); nn = pnd.xyz; nfv = flashTargetVis(nx)
//     mergeReservoirMIS(&res,&misR, prev, MCap, nx,nn,nfv, canon,canonProxy, ...)
//     mergeReservoirMIS(&carry,&misC, prev, MCap, nx,nn,nfv, canon,canonProxy, ...)
//   each spatial tap gate pass: nx = prevWorldFromPixel(qp, dims, qnd.w); nn = qnd.xyz; nfv = flashTargetVis(nx)
//     mergeReservoirMIS(&res,&misR, prev, MCap*0.5, ...)
//   insertCanonical(&res,misR,canon); insertCanonical(&carry,misC,canon)
//   finalizeCombined(&res, misR.k);   finalizeCombined(&carry, misC.k)
//   shading shadow ray + res->carry zero-transfer + store carry: UNCHANGED

// ---- GI (no reconstruction: receiver = (prev.visiblePos, pnd.xyz)) ----
fn giProxyTarget(x: vec3f, nrm: vec3f, sp: vec3f, n2: vec3f, rad: vec3f) -> f32 {
  let d = sp - x;  let d2 = max(dot(d, d), 1e-6);
  let w = d * inverseSqrt(d2);
  let ndl = dot(nrm, w);
  if (ndl <= 0.0) { return 0.0; }
  return ndl * luminance(rad) * abs(dot(n2, w)) / d2;           // shared AREA measure at x2
}

fn giMergePrevMIS(r: ptr<function,GIReservoir>, mis: ptr<function,Mis>, prevIn: GIReservoir, mCap: f32,
                  nn: vec3f, canon: GIReservoir, canonProxy: f32, h: Hit, v: vec3f, m: Material) {
  var prev = prevIn;  prev.M = min(prev.M, mCap);
  (*mis).k += 1.0;  (*r).M += prev.M;
  var pi = 1.0;
  if (canon.wSum > 0.0 && prev.M > 0.0) {
    let qN = giProxyTarget(prev.visiblePos, nn, canon.samplePos, canon.sampleNrm, canon.radiance);
    let den = canon.M * canonProxy + prev.M * qN;
    pi = select(1.0, canon.M * canonProxy / den, den > 0.0);
  }
  (*mis).canonSum += pi;
  if (prev.M <= 0.0 || prev.W <= 0.0) { return; }
  let d = prev.samplePos - h.p;  let d2 = dot(d, d);
  if (d2 <= 1e-6) { return; }
  let dir = d * inverseSqrt(d2);
  let tp = giTarget(m, h.n, v, dir, prev.radiance);             // TRUE target, solid angle at us
  if (tp <= 0.0) { return; }
  let qS = giProxyTarget(prev.visiblePos, nn, prev.samplePos, prev.sampleNrm, prev.radiance);
  let qC = giProxyTarget(h.p, h.n, prev.samplePos, prev.sampleNrm, prev.radiance);
  let den = prev.M * qS + canon.M * qC;
  if (qS <= 0.0 || den <= 0.0) { return; }
  let j = giJacobian(prev, h.p);                                // clamped; resample weight ONLY
  giUpdate(r, prev.samplePos, prev.sampleNrm, prev.radiance, h.p,
           (prev.M * qS / den) * tp * prev.W * j, tp, 0.0);     // count 0: M added above
}

// restirGI flow: canon = 1-candidate reservoir (giUpdate(..., luminance(bounceWeight*rad), freshTarget, 1)),
//   old finalize -> W_c; canonProxy = giProxyTarget(h.p, h.n, canon.samplePos, canon.sampleNrm, canon.radiance);
//   two streams + insertCanonicalGI (w = canonSum * canon.targetPdf * canon.W, Jacobian 1) exactly as DI;
//   finalizeGICombined: W = min(wSum / (max(k,1) * targetPdf), 32); occlusion + store: UNCHANGED.

## Derivation 3

### predicted_outcome

Temporal-only: the k=1 pairwise combine is the exact confidence-weighted two-technique balance heuristic, so relBias +0.23 should collapse to ~0 (expect |relBias| < ~0.03; residuals are the visibility-censored stored stream, slightly negative in penumbras, and light-motion target mismatch, which is zero in a static measurement scene). relRmse should improve as well since a 0.23 bias was a large part of the error.

Spatial-only (6 taps): relBias -0.02..-0.05 -> ~0 (the small negative energy dilution from tp<=0 M-adds and the skip-M asymmetry disappears); relRmse stays around or slightly below the current 0.65-0.74.

Combined: relBias +0.32 -> ~0, and relRmse should drop from 1.08 to at or below the spatial-only level (roughly 0.5-0.65): with the M-proportional overweighting gone, the temporal stream now adds effective samples on top of the spatial ones instead of compounding bias, so the combination should finally beat either mode alone. If combined RMSE lands above spatial-only despite ~0 bias, the suspect is pairwise's 1/k dilution of the temporal technique (its maximum MIS weight is 1/k ~ 1/7), and the remedy is reducing spatial taps or moving to the O(K^2) full balance heuristic, not revisiting the bias math.

GI: same directional expectations, with a persistent small negative floor from the Jacobian clamp at 16, the W cap at 32, and the visibility-less proxy support mismatch (slight darkening at occlusion boundaries). Verify per project practice by measuring: temporal-only, spatial-only, and combined relBias/relRmse against the reference, plus the k=0 invariants (reuse disabled must reproduce current output exactly for both DI and GI).

