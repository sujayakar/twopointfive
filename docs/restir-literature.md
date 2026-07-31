# ReSTIR Literature Review — with emphasis on indirect / global illumination

**Compiled:** 2026-07-27
**Target reader:** a WebGPU compute path tracer that already has ReSTIR DI working, runs 1–2 bounces at ~1152×720 on Apple silicon, and denoises with SVGF-style filters over separate direct/indirect signals.

---

## 0. How this was compiled, and what "verified" means here

Every title, author list, venue and year below was checked against at least one primary or near-primary source: publisher pages (ACM DL, Wiley/CGF, Eurographics DigLib), author publication pages (cwyman.org, dqlin.xyz, graphics.cs.utah.edu, research.nvidia.com/labs/rtr), or arXiv abstract pages.

Two documents were read in full text rather than skimmed from abstracts, and they carry most of the technical weight in this report:

- **The SIGGRAPH 2023 ReSTIR course notes** (March 4, 2024 revision), extracted and read directly. This is written by the ReSTIR authors themselves and is the single best source for the theory *and* the pitfalls.
- **ReSTIR PT Enhanced (I3D 2026)** — a copy was already present in this repo (`restir pt enhanced.pdf`) and was read in full. It is the most implementation-relevant recent paper.

Where a claim rests on secondary sourcing, it is flagged inline and again in §9.

---

## 1. Foundations

### 1.1 Importance Resampling for Global Illumination (RIS) — the pre-ReSTIR ancestor

- **Title:** Importance Resampling for Global Illumination
- **Authors:** Justin F. Talbot, David Cline, Parris Egbert
- **Venue:** Rendering Techniques (Proceedings of the Eurographics Symposium on Rendering), 2005, pp. 139–146
- **DOI:** `10/gfzsm2`

**Core idea.** Draw *M* cheap candidate samples from a tractable source PDF *p*, then select one of them with probability proportional to a *target function* p̂ that you can evaluate but not sample. The selected sample gets an *unbiased contribution weight* W = (1/p̂(Y))·(1/M)·Σ p̂(Xᵢ)/p(Xᵢ), which stands in for the intractable 1/p(Y). Talbot et al. deliberately drop visibility from p̂ for speed.

**Assessment for you.** You already have this — it is the inner loop of ReSTIR DI. Worth re-reading only if your DI implementation has never been validated against a ground-truth path tracer. The course notes are blunt on this: if plain RIS doesn't converge to ground truth in your codebase, nothing downstream will either.

### 1.2 ReSTIR DI — the original

- **Title:** Spatiotemporal reservoir resampling for real-time ray tracing with dynamic direct lighting
- **Authors:** Benedikt Bitterli, Chris Wyman, Matt Pharr, Peter Shirley, Aaron Lefohn, Wojciech Jarosz
- **Venue:** ACM Transactions on Graphics (Proc. SIGGRAPH) 39(4), July 2020
- **DOI:** [10.1145/3386569.3392481](https://dl.acm.org/doi/10.1145/3386569.3392481)
- **Links:** [project page](https://benedikt-bitterli.me/restir/) · [PDF](https://research.nvidia.com/sites/default/files/pubs/2020-07_Spatiotemporal-reservoir-resampling/ReSTIR.pdf)

**Core idea.** Wrap RIS in *weighted reservoir sampling* so that resampling becomes a streaming O(1)-memory operation, then chain reservoirs across neighboring pixels (spatial reuse) and across frames (temporal reuse). Each pixel's reservoir holds one light sample x₂, its unbiased contribution weight W, a running weight sum, and a count M. Reported results: unbiased estimator 6×–60× faster at equal error, biased estimator 35×–65× faster with some energy loss; up to 3.4M dynamic emissive triangles under 50 ms/frame at ≤8 rays/pixel.

**Key detail people get wrong.** The paper presents *both* a biased and an unbiased estimator. The fast/headline version uses constant 1/M resampling weights plus neighbor rejection, which is **biased** (energy loss). The unbiased variant uses a contribution MIS weight. See §9.

**Assessment for you.** Already done. The thing worth auditing: is your target function p̂ = f_s · G · V · L_e including the visibility term V, and are your resampling MIS weights the generalized balance heuristic rather than 1/M? The course notes recommend keeping V in p̂ and only removing it after you have a validated baseline.

### 1.3 Rearchitecting Spatiotemporal Resampling for Production

- **Title:** Rearchitecting spatiotemporal resampling for production
- **Authors:** Chris Wyman, Alexey Panteleev
- **Venue:** High-Performance Graphics 2021 — Symposium Papers (Eurographics Association)
- **DOI:** [10.2312/hpg.20211281](https://dl.acm.org/doi/abs/10.2312/hpg.20211281)
- **Links:** [slides](http://cwyman.org/presentations/2021_HPG_Productizing_ReSTIR.pdf)

**Core idea.** Engineering, not theory. Introduces *presampled light tiles* (precompute e.g. 128 tiles × 1024 lights per frame; each 8×8 screen tile draws from one tile) so that candidate generation is cache-coherent instead of scattering into a huge light buffer. Plus fused spatiotemporal passes and quality/perf knobs. Claims up to 7× lighting cost reduction and much better memory coherence. ReSTIR PT Enhanced (2026) still uses this exact light-tile scheme, five years later.

**Assessment for you.** Complexity: **low** (a few hundred lines: a compute pass that fills a tile buffer, plus an indexing change in candidate generation). Payoff: **high if and only if you have many lights**. If your scene has a handful of lights and an environment map, this buys you nothing — the win is entirely about memory locality when sampling from thousands of emitters. Check your profile first.

---

## 2. ReSTIR for indirect / global illumination

### 2.1 ReSTIR GI — the practical indirect method

- **Title:** ReSTIR GI: Path Resampling for Real-Time Path Tracing
- **Authors:** Yaobin Ouyang, Shiqiu Liu, Markus Kettunen, Matt Pharr, Jacopo Pantaleoni
- **Venue:** Computer Graphics Forum 40(8) (Proc. High Performance Graphics 2021), pp. 17–29
- **DOI:** [10.1111/cgf.14378](https://onlinelibrary.wiley.com/doi/abs/10.1111/cgf.14378)
- **Results:** MSE improvements of 9.3×–166× over 1 spp path tracing across their test scenes.

#### What the reservoir actually stores — and how it differs from DI

This is the question you asked, so here it is precisely. A ReSTIR GI reservoir stores a **single secondary vertex plus a cached radiance value**, not a path:

| | ReSTIR DI | ReSTIR GI |
|---|---|---|
| Sample is | a point x₂ on an emitter | a point x₂ on *any* surface (the "sample point"), found by BSDF-sampling a ray from the primary hit |
| Stored per reservoir | x₂ position, light/emitter id, W, w_sum, M(c) | **sample point** position + normal, **visible point** position + normal, **outgoing radiance L_o at the sample point** (a full RGB value, computed by path tracing from x₂ for the remaining bounces), the solid-angle PDF of the generating ray, W, w_sum, M(c) |
| Radiance term | evaluated exactly, L_e is a scene property | **cached and reused**, treated as view-independent |
| Shift used for reuse | identity in area measure at x₂ | identity/reconnection shift at x₂: `T([x0,x1,x2,…]) = [y0,y1,x2,…]` — vertices x₂ onward are reused verbatim |
| Jacobian | area-measure reparameterization of the light sample | solid-angle→area reparameterization when reconnecting y₁→x₂: the ratio of `cos θ / d²` terms between the new and old reconnection segments |
| Unbiased? | can be (with proper MIS weights) | **no — structurally biased**, see below |

The reservoir also stores the *visible point* geometry specifically so that reuse can recompute the BSDF at the receiver rather than reusing the source pixel's BSDF.

**Why it is biased, in the authors' own words.** From course notes §6.3: *"ReSTIR GI precomputes the outgoing radiance along x₂x₁ when the sample is produced by a path tracer, and assumes it is unchanged along the reconnected direction x₂y₁, during reuse. Obviously, this is only true for a very limited set of materials like Lambertian diffuse material."* The correct unbiased version would re-evaluate two BSDFs, a geometry factor, and shoot a shadow ray at reuse time. ReSTIR GI skips this to save rays and memory. The RTXDI documentation makes the same admission from the implementation side: secondary surfaces are treated as *"Lambertian reflectors or emitters"*, so directional effects and caustics are lost, and it recommends clamping secondary-surface roughness upward and MIS-ing ReSTIR GI's output against the raw path-traced signal on low-roughness surfaces because *"ReSTIR doesn't work well on specular surfaces."*

**Assessment for you.** ⭐ **This is the single highest value/effort item on the list for your renderer.**

- *Complexity:* **moderate-low.** You already have reservoirs, WRS, spatial/temporal passes, motion-vector reprojection, G-buffer heuristics, and MIS weight machinery from DI. ReSTIR GI is a second reservoir buffer with a different payload and a different Jacobian. Realistically 1–2 weeks including debugging. The genuinely new code is: (a) the reconnection Jacobian, (b) the target function p̂ = |f_s(visible→sample) · G · L_o|, (c) storing a second surface's geometry.
- *Memory:* a packed GI reservoir is comfortably ~48 bytes (two oct-encoded normals, two positions, RGB9E5 radiance, three floats). At 1152×720 ≈ 829k pixels that's ~40 MB per buffer; you need current + previous (+ optionally a spatial ping-pong). Fine on Apple silicon, but check your WebGPU `maxStorageBufferBindingSize` — the default limit is 128 MiB and you should request more explicitly if you want headroom.
- *Payoff:* **large.** At 1 spp indirect, this is the difference between SVGF smearing mush and SVGF having something to filter. The 9.3×–166× MSE numbers are on 1 spp, which is exactly your regime.
- *Interaction with your SVGF setup:* good — you already separate direct and indirect, which is exactly the split ReSTIR GI assumes. But be aware ReSTIR *increases inter-pixel correlation*, and SVGF's variance estimate (which is computed from spatial/temporal sample statistics) will **underestimate** true variance when neighbors share reservoirs. Expect to retune your variance-driven filter radius. This is a real and commonly-underappreciated interaction; see §7.
- *Caveat:* the Lambertian-radiance assumption means glossy indirect will be wrong. With 1–2 bounces and SVGF you probably don't care, but don't be surprised when a mirror floor doesn't reflect indirect correctly.

**Not verified directly:** the CGF paper PDF is 157 MB and could not be fetched. The reservoir-content description above is assembled from the SIGGRAPH 2023 course notes §6.3 (written by, among others, ReSTIR GI co-author Markus Kettunen) and the RTXDI `RestirGI.md` implementation documentation. Both are authoritative but neither is the paper text.

### 2.2 GRIS / ReSTIR PT — the theory, and full path reuse

- **Title:** Generalized resampled importance sampling: foundations of ReSTIR
- **Authors:** Daqi Lin, Markus Kettunen, Benedikt Bitterli, Jacopo Pantaleoni, Cem Yuksel, Chris Wyman (joint first authors)
- **Venue:** ACM Transactions on Graphics (Proc. SIGGRAPH) 41(4), July 2022, 75:1–75:23
- **DOI:** [10.1145/3528223.3530158](https://dl.acm.org/doi/10.1145/3528223.3530158)
- **Links:** [PDF](https://graphics.cs.utah.edu/research/projects/gris/sig22_GRIS.pdf) · [code](https://github.com/DQLin/ReSTIR_PT) · [project page](https://dqlin.xyz/pubs/2022-sig-GRS/)

#### What GRIS generalizes

Classical RIS assumes candidates are independent and have known PDFs over a *single shared domain*. Every one of those assumptions is false in ReSTIR: temporal samples are correlated with themselves from previous frames, reservoir samples have unknown PDFs (that's the whole point of the unbiased contribution weight), and a sample from a neighbor pixel lives in a *different integration domain* because each pixel has a different integrand. GRIS extends RIS to correlated samples with unknown PDFs from varied domains, and from that derives variance bounds and convergence conditions.

#### Why naive path reuse is biased

Two independent reasons, and it is worth keeping them separate:

1. **Domain mismatch / support violation.** Pixel *j*'s path is a sample from pixel *j*'s domain. Feeding it into pixel *i*'s estimator without accounting for which pixels *could* have generated that path over- or under-counts parts of the integration domain. With constant 1/M weights this shows up as unexplained brightening or darkening. The fix is correct resampling MIS weights m_i satisfying Σ m_i(y) = 1 over all techniques whose (shifted) support contains y.
2. **Density change under the map.** The shift mapping T that transports a path from domain A to domain B is a change of variables, and change of variables changes densities. If you don't multiply by |∂T/∂x| — the **Jacobian determinant** of the shift — your weights are simply wrong. Concretely, reconnecting a reused sub-path to a different primary hit changes the geometry term `cos θ / d²`; that ratio *is* the Jacobian for the reconnection shift.

Both must be right. Getting the MIS weights right with a wrong Jacobian still gives you a biased estimator, and vice versa.

#### Shift mappings

A shift mapping T: A → B maps paths in domain A to paths in domain B. Requirements: it must be a **bijection** (invertible) on its domain of definition, and you must be able to compute |T′(x)|. The course notes define a good shift as one satisfying p̄_j(T(x))·|∂T/∂x| ≈ p̄_i(x) — i.e. the shifted path should be about as good a sample for the destination pixel as the original was for the source pixel. GRIS proposes the stronger (sufficient, not necessary) condition p̂_j(T(x)) ≈ p̂_i(x) *and* |∂T/∂x| ≈ 1.

The three shifts you'll see named:

- **Reconnection shift.** Replace the first vertices and reconnect directly to x₂; reuse [x₂ … x_D] verbatim. Cheap, one shadow ray, constant storage. Fails badly when x₁ or x₂ is specular (shifted path contribution goes to ~0), and fails when the reconnection segment length differs wildly from the original (geometry term ratio bursts). This is what ReSTIR GI uses.
- **Random replay.** Copy the base path's random number sequence and re-trace with the same sampling decisions at each bounce. Has an **identity Jacobian in primary sample space** — a major practical reason ReSTIR PT parameterizes paths in PSS. Handles specular chains, but requires re-tracing rays and can drift.
- **Hybrid shift (ReSTIR PT).** Random-replay the early, hard-to-reconnect segments, then reconnect at the first vertex that passes two tests: a **distance condition** `min(‖x_k − x_{k−1}‖, ‖x_k − y_{k−1}‖) ≥ d_min` and a **roughness condition** `min(α_{x_{k−1}}(ℓ_{k−1}), α_{y_{k−1}}(ℓ′_{k−1}), α_{x_k}(ℓ_k)) ≥ α_min`. Storage is constant per pixel: a reconnection vertex plus an RNG seed. Invertibility requires copying the *lobe index* ℓ to the offset path — if that lobe doesn't exist on y_{k−1}, the shift must fail cleanly. ReSTIR PT also tags each path vertex with the sampled lobe and the light-sampling technique so random replay reproduces exactly the same subpath.

**Assessment for you.** *Complexity:* **high — the highest on this list.** Hybrid shift requires: PSS parameterization of your path tracer, per-vertex lobe tagging, deterministic replayable RNG streams, careful bijection bookkeeping, and — critically — **re-tracing rays during resampling**. On hardware with RT cores that's affordable. On WebGPU with a software BVH traversal in a compute shader, random replay means running your traversal loop again inside the reuse pass, for every neighbor, in both directions (see §5.1 — MIS weights need the *reverse* shift too). *Payoff for your configuration:* **poor.** ReSTIR PT's advantage over ReSTIR GI is concentrated in many-bounce transport through glossy/specular chains. You run 1–2 bounces and denoise with SVGF, so you have neither the bounce depth nor the specular fidelity for that advantage to materialize. **Read GRIS for the theory — it will make your MIS weights and Jacobians correct — but do not implement ReSTIR PT.**

### 2.3 Conditional RIS (CRIS)

- **Title:** Conditional Resampled Importance Sampling and ReSTIR
- **Authors:** Markus Kettunen, Daqi Lin, Ravi Ramamoorthi, Thomas Bashford-Rogers, Chris Wyman (joint first authors)
- **Venue:** SIGGRAPH Asia 2023 Conference Papers
- **DOI:** [10.1145/3610548.3618245](https://dl.acm.org/doi/10.1145/3610548.3618245)
- **Code:** [NVlabs/conditional-restir-prototype](https://github.com/NVlabs/conditional-restir-prototype)

**Core idea.** Extends GRIS to *conditional* path spaces, which legitimizes reusing **path suffixes** independently of their prefixes — something plain GRIS can't do without introducing hidden conditional dependencies. The demo application resamples multiple ReSTIR-driven path suffixes in a photon-map-style final gather; as in photon mapping, the final gather breaks up the blotchy correlation artifacts that come from sample sharing. (The "Suffix ReSTIR" name you'll see referenced comes from this line of work.)

**Assessment for you.** *Complexity:* **high.** *Payoff:* **low right now.** This is the theoretical license for a whole class of "obvious" optimizations that are actually illegal, and it matters if you ever build a world-space suffix cache. Worth knowing it exists; not worth implementing. Its most useful takeaway for you is negative knowledge: **any time you make a reuse decision based on the contents of a reservoir rather than on the G-buffer, you have silently moved into a conditional probability space and your MIS weights are no longer valid.**

---

## 3. Resampling weights, neighbor selection, correlation

These are the cheap wins. Almost every item here is a small change to code you already have.

### 3.1 Pairwise MIS weights

- **Source:** Benedikt Bitterli, *Correlations and Reuse for Fast and Accurate Physically Based Light Transport*, PhD thesis, Dartmouth College, January 2022. Generalized family also given in GRIS (Lin et al. 2022). Presented pedagogically in course notes §7.1.3.

**Core idea.** The generalized balance heuristic costs O(M²) target-function evaluations, which is brutal when each evaluation is a shadow ray. Pairwise MIS instead MIS-es each non-canonical neighbor *only against the canonical (current-pixel) sample*, giving O(M) cost while remaining unbiased. The **defensive** variant reserves a guaranteed share of weight for the canonical sample using κ = (M − |R|)/M, which keeps the estimator well-behaved when all the neighbors are garbage.

**Assessment for you.** *Complexity:* **low** (a weight-formula change, ~50 lines). *Payoff:* **high.** This is how you get unbiased spatial reuse without O(M²) rays, which on a software-BVH WebGPU renderer is the difference between affordable and not. If your DI implementation currently uses 1/M weights plus neighbor rejection, moving to defensive pairwise MIS is probably the best ratio of effort to quality on this entire page. Do it before ReSTIR GI, so ReSTIR GI inherits it.

### 3.2 Efficient Spatial Resampling Using the PDF Similarity

- **Author:** Yusuke Tokuyoshi
- **Venue:** Proc. ACM on Computer Graphics and Interactive Techniques 6(1), Article 4 (I3D 2023)
- **DOI:** [10.1145/3585501](https://dl.acm.org/doi/10.1145/3585501)

**Core idea.** A cheap similarity measure between two pixels' target PDFs, used to decide whether spatial reuse is worth doing at all. Cited by the course notes as the formalization of the "ideal shift mapping" condition p̄_j(T(x))·|∂T/∂x| ≈ p̄_i(x).

**Assessment for you.** *Complexity:* **low.** *Payoff:* **moderate.** A principled replacement for hand-tuned normal/depth thresholds. Note the constraint from §2.3: rejection heuristics are only bias-free if they look at *domains* (G-buffer), not at *samples*.

### 3.3 Enhancing Spatiotemporal Resampling with a Novel MIS Weight

- **Authors:** Xingyue Pan, Jiaxuan Zhang, Jiancong Huang, Ligang Liu
- **Venue:** Computer Graphics Forum 43(2) (Proc. Eurographics 2024)
- **DOI:** [10.1111/cgf.15049](https://onlinelibrary.wiley.com/doi/10.1111/cgf.15049)

**Core idea.** A new resampling MIS weight for blending samples across sampling domains that keeps results convergent as the proportion of non-canonical samples rises, applied to temporal resampling to reduce noise caused by scene changes and jitter.

**Assessment for you.** *Complexity:* **low-moderate.** *Payoff:* **moderate**, specifically for the case where camera/object motion makes most of your temporal history invalid. Non-NVIDIA group, less battle-tested than the pairwise MIS above; treat as a refinement, not a starting point.

### 3.4 Stochastic Pairwise MIS for Unbiased Large-Kernel Reuse in Real Time

- **Authors:** Trevor Hedstrom, Markus Kettunen, Daqi Lin, Chris Wyman, Tzu-Mao Li
- **Venue:** Computer Graphics Forum (Proc. Eurographics 2026)
- **DOI:** [10.1111/cgf.70391](https://onlinelibrary.wiley.com/doi/10.1111/cgf.70391) · [project page](https://research.nvidia.com/labs/rtr/publication/hedstrom2026stochastic/)

**Core idea.** Standard ReSTIR reuses from a small fixed number of random neighbors, which collapses toward the raw path tracer when few neighbors hold contributing samples — exactly what happens under camera motion and disocclusion. This paper makes it affordable to *unbiasedly* reuse from many neighbors by stochastically selecting which pairwise MIS terms to evaluate, focusing reuse on pixels that actually have contributing samples.

**Assessment for you.** *Complexity:* **moderate** (builds directly on 3.1). *Payoff:* **moderate-high** if disocclusion noise is a visible problem for you. It directly attacks the "big empty region just came onscreen" failure that SVGF cannot fix.

### 3.5 Compatibility-Guided Neighbor Selection for ReSTIR ⭐

- **Authors:** Orion Junkins, Markus Kettunen, Daqi Lin, Ravi Ramamoorthi, Chris Wyman
- **Venue:** Proc. ACM on Computer Graphics and Interactive Techniques (HPG 2026) — **Best Paper**
- **DOI:** [10.1145/3820024](https://dl.acm.org/doi/10.1145/3820024) · [PDF](https://research.nvidia.com/labs/rtr/publication/junkins2026compatibility/junkins2026compatibility.pdf)

**Core idea.** Almost all recent ReSTIR work (splatting, Area ReSTIR) has improved *temporal* robustness; this paper attacks *spatial* neighbor selection, which had been essentially "pick 3 random pixels in a 30-pixel disk" since 2020. It analyses path compatibility between pixels, empirically maps the fundamental error-vs-correlation tradeoff, and improves the Pareto frontier with a tunable knob.

**Reported results:** SMAPE reduced 6–29%, **temporal covariance reduced 22–49%**, for **2–5% incremental cost**.

**Assessment for you.** ⭐ *Complexity:* **low.** It changes which pixels you pick, not the resampling math. *Payoff:* **high, and unusually well-targeted at your setup.* The 22–49% temporal covariance reduction is the number to focus on: correlation between neighboring pixels and across frames is precisely what breaks SVGF's variance estimator and produces the low-frequency blotches that survive denoising. A 2–5% cost for that is close to free. If you only take two things from this document, take defensive pairwise MIS (§3.1) and this.

### 3.6 Decorrelating ReSTIR Samplers via MCMC Mutations

- **Authors:** Rohan Sawhney, Daqi Lin, Markus Kettunen, Benedikt Bitterli, Ravi Ramamoorthi, Chris Wyman, Matt Pharr
- **Venue:** ACM Transactions on Graphics 43(1), February 2024
- **DOI:** [10.1145/3629166](https://dl.acm.org/doi/abs/10.1145/3629166) · [arXiv:2211.00166](https://arxiv.org/abs/2211.00166)

**Core idea (from the abstract, verbatim in substance).** Unchecked spatiotemporal resampling introduces visible correlation artifacts, and reservoirs holding more than one sample suffer *impoverishment* — duplicate samples. Interleaving MCMC mutations with reservoir resampling alleviates both, especially for glossy materials and hard-to-sample lighting. **The approach introduces no bias**, and one mutation per reservoir sample per frame is enough in practice.

**Assessment for you.** *Complexity:* **moderate.** You need a mutation proposal, an acceptance test, and a way to evaluate the target function at the mutated sample (rays). *Payoff:* **moderate.** The problem it solves — correlation artifacts and sample impoverishment — is real and is the main thing that makes ReSTIR output look "wrong" rather than "noisy". But §5.1 (duplication maps) attacks the same problem for a fraction of the effort, at the cost of a small bias. Try that first.

---

## 4. Domain extensions (mostly informational for you)

### 4.1 Volumetric ReSTIR

- **Title:** Fast volume rendering with spatiotemporal reservoir resampling
- **Authors:** Daqi Lin, Chris Wyman, Cem Yuksel
- **Venue:** ACM Transactions on Graphics (Proc. SIGGRAPH Asia) 40(6), Dec 2021, 279:1–279:18
- **DOI:** [10.1145/3478513.3480499](https://dl.acm.org/doi/10.1145/3478513.3480499) · [project page](https://graphics.cs.utah.edu/research/projects/volumetric-restir/) · [code](https://github.com/DQLin/VolumetricReSTIRRelease)

**Core idea.** Extends reservoir resampling to multi-dimensional path space for participating media. The key insight worth remembering even if you never render volumes: *using cheaper, biased approximations of scattering along candidate paths during resampling does not add bias, as long as shading uses the exact evaluation.* Because resampling only needs a target function — any positive function — you can make p̂ cheap and approximate without affecting correctness, provided the final unbiased contribution weight uses the real thing.

**Assessment for you.** *Payoff:* **skip unless you add volumes.** But internalize the target-function insight — it lets you use a cheap p̂ (e.g. skipping visibility, or using a coarse BRDF) during candidate resampling.

### 4.2 Ghost ReSTIR

- **Title:** Ghost ReSTIR: Volumetric Resampling with Ghost Vertices in Null-Scattering Space
- **Authors:** Song Zhang, Daqi Lin, Pengpei Hong, Markus Kettunen, Cem Yuksel, Chris Wyman
- **Venue:** SIGGRAPH 2026 **Posters**, Article 86
- **DOI:** [10.1145/3799825.3818719](https://doi.org/10.1145/3799825.3818719)

**Core idea.** Reformulates volumetric ReSTIR in null-scattering primary sample space so the target function contains no intermediate transmittance and can be evaluated consistently at every resampling stage. "Ghost vertices" are auxiliary infinite tails appended to each segment, giving a well-defined bijection for reconnection shifts regardless of null-vertex count.

**Assessment for you.** *Skip.* Noted because it is the current state of the art in volumetric ReSTIR. **Flag: this is a 2-page poster, not a full paper** — treat the reported numbers as preliminary.

### 4.3 ReSTIR Subsurface Scattering

- **Title:** ReSTIR Subsurface Scattering for Real-Time Path Tracing
- **Authors:** Mirco Werner, Vincent Schüßler, Carsten Dachsbacher
- **Venue:** Proc. ACM on Computer Graphics and Interactive Techniques (HPG 2024)
- **DOI:** [10.1145/3675372](https://dl.acm.org/doi/10.1145/3675372) · [code](https://github.com/MircoWerner/ReSTIR-SSS) · [project page](https://cg.ivd.kit.edu/restir-sss.php)

**Core idea.** Applies ReSTIR to subsurface transport with BSSRDF importance sampling for candidates, and a subsurface-specific criterion to choose between reconnecting *through* the translucent object versus one vertex later. Notably an independent group (KIT), not the NVIDIA/Utah axis.

**Assessment for you.** *Skip* unless you render skin/marble.

### 4.4 Area ReSTIR

- **Title:** Area ReSTIR: Resampling for Real-Time Defocus and Antialiasing
- **Authors:** Song Zhang, Daqi Lin, Markus Kettunen, Cem Yuksel, Chris Wyman (joint first authors)
- **Venue:** ACM Transactions on Graphics (Proc. SIGGRAPH) 43(4), 2024
- **DOI:** [10.1145/3658210](https://dl.acm.org/doi/10.1145/3658210) · [project page](https://graphics.cs.utah.edu/research/projects/area-restir/) · [partial code](https://github.com/guiqi134/Area-ReSTIR)

**Core idea.** Classic ReSTIR fixes the primary hit x₁ per pixel, so it cannot handle subpixel jitter, antialiasing, or depth of field — the reservoir is tied to one specific primary ray. Area ReSTIR extends reservoirs to integrate over the *pixel footprint* and lens aperture, adding subpixel and lens-position dimensions to the reservoir with corresponding shift mappings.

**Assessment for you.** *Complexity:* **high.** New shift maps on the primary vertex, a redefined integration domain, more reservoir state. *Payoff:* **low-moderate.** You are at 1152×720 with (presumably) TAA-style jitter; Area ReSTIR would help temporal stability under jitter, but §4.5 supersedes it at lower cost.

### 4.5 Reservoir Splatting

- **Title:** Reservoir Splatting for Temporal Path Resampling and Motion Blur
- **Authors:** Jeffrey Liu, Daqi Lin, Markus Kettunen, Chris Wyman, Ravi Ramamoorthi
- **Venue:** SIGGRAPH 2025 Conference Papers
- **DOI:** [10.1145/3721238.3730646](https://dl.acm.org/doi/full/10.1145/3721238.3730646) · [code](https://github.com/Jebbly/Reservoir-Splatting)

**Core idea.** Replace *backprojection* (current pixel looks up where it was last frame via motion vectors) with *forward splatting* (last frame's paths project their primary hits into this frame's pixels). Backprojection fails under camera motion and when subpixel geometry has differing motion; splatting lands each sample in the pixel that actually corresponds to its exact primary hit. Bonus: splatting at multiple time steps gives motion blur, and depth of field falls out without Area ReSTIR's specialized shift maps. Reported to beat Area ReSTIR quality at up to 10% lower cost.

**Assessment for you.** *Complexity:* **moderate.** The awkward part on GPU is the scatter: splatting requires atomic writes into destination pixels and a resolution policy for collisions and holes — which is exactly the kind of thing that is annoying in WGSL (you have `atomicAdd`/`atomicCompareExchangeWeak` on `atomic<u32>`/`atomic<i32>` in storage, but no float atomics). *Payoff:* **moderate.** If your motion-vector-based temporal reuse visibly fails during camera motion, this is the principled fix. Not a first move.

### 4.6 Multi-Layer Reservoir Splatting

- **Title:** Multi-Layer Reservoir Splatting for Temporal Reuse under Disocclusion
- **Authors:** Pengpei Hong, Song Zhang, Daqi Lin, Markus Kettunen, Chris Wyman, Cem Yuksel
- **Venue:** SIGGRAPH 2026 Conference Papers
- **DOI:** [10.1145/3799902.3811232](https://doi.org/10.1145/3799902.3811232) · [project page](https://graphics.cs.utah.edu/research/projects/multi-layer-restir/)

**Core idea.** Keeps multiple screen-space layers so that samples currently hidden behind geometry are retained and can be reused when they become visible again. Uses reservoir splatting to shift samples between layers, and introduces depth ranges plus a redefined integration domain to eliminate most of the extra ray queries that layer-shifting would otherwise need. Only active domains propagated from previous frames are tracked, to bound cost.

**Assessment for you.** *Complexity:* **high** (multi-layer G-buffer, layer management). *Payoff:* **low for you** — the memory cost of N layers at your resolution is fine, but the engineering is substantial and disocclusion is better attacked first via §3.4/§3.5.

### 4.7 ReSTIR BDPT

- **Title:** ReSTIR BDPT: Bidirectional ReSTIR Path Tracing with Caustics
- **Authors:** Trevor Hedstrom, Markus Kettunen, Daqi Lin, Chris Wyman, Tzu-Mao Li
- **Venue:** ACM Transactions on Graphics, 2025
- **DOI:** [10.1145/3744898](https://dl.acm.org/doi/10.1145/3744898) · [PDF](https://cwyman.org/papers/tog25_ReSTIR_BDPT.pdf) · [code](https://github.com/Shmaug/ReSTIR-BDPT)

**Core idea.** GRIS is by default unaware of *how* a path was sampled, which blocks bidirectional reuse. This applies GRIS in a technique-aware extended path space, designs a bidirectional hybrid shift, and adds "caustics reservoirs" that accumulate caustics across frames. Runs ~50 ms/frame across test scenes.

**Assessment for you.** *Skip.* ~50 ms/frame on an RTX-class card is already outside your budget; on WebGPU it is not a real option.

### 4.8 ReSTIR FG

- **Title:** ReSTIR FG: Real-Time Reservoir Resampled Photon Final Gathering
- **Authors:** René Kern, Felix Brüll, Thorsten Grosch (TU Clausthal)
- **Venue:** Eurographics Symposium on Rendering (EGSR) 2024
- **Links:** [EG DigLib](https://diglib.eg.org/items/df98f89d-a0ca-4800-9bc4-74528feaf872) · [code](https://github.com/TU-Clausthal-Rendering/ReSTIR-FG)

**Core idea.** Combines photon final gathering with spatiotemporal resampling to get multi-bounce indirect *and* caustics, which ReSTIR GI / ReSTIR PT / Suffix ReSTIR all handle poorly. Claims competitive runtime and quality against those methods.

**Assessment for you.** *Complexity:* **high** (you'd need photon tracing and a photon acceleration structure). *Payoff:* **low** given 1–2 bounces. Worth flagging that the repo ships a **"ReSTIR FG Lite"** variant explicitly intended to make the core algorithm easier to learn — useful as reading material.

### 4.9 Gradient-Domain ReSTIR Path Tracing

- **Title:** Gradient-Domain ReSTIR Path Tracing
- **Authors:** Yu-Chen Wang, Markus Kettunen, Daqi Lin, Chris Wyman, Lifan Wu, Shuang Zhao
- **Venue:** Computer Graphics Forum (Proc. Eurographics 2026)
- **DOI:** [10.1111/cgf.70328](https://onlinelibrary.wiley.com/doi/10.1111/cgf.70328) · [PDF](https://projects.shuangz.com/ReSTIR-GDPT-eg26/ReSTIR-GDPT-eg26.pdf) · [code](https://github.com/elite-sheep/gradient-restir)

**Core idea.** First real-time gradient-domain rendering method. Estimates pixel *differences* alongside pixel colors and reconstructs via screened Poisson; makes it real-time by spatiotemporal reuse plus a path-space extension for the gradient image. Key observation: the gradient image is relatively sparse, which permits highly selective (and therefore cheap) spatial reuse.

**Assessment for you.** *Complexity:* **high** — you'd need a Poisson reconstruction pass in addition to the sampling changes, and gradient-domain reconstruction interacts awkwardly with SVGF (both are reconstruction operators; you'd be picking one). *Payoff:* **speculative.** Genuinely interesting as an *alternative* to SVGF rather than a complement, but this is a research bet, not a product move.

### 4.10 ToF ReSTIR — flagged as unverified venue

- **Title:** ToF ReSTIR: Time-of-Flight Rendering with Spatio-temporal Reservoir Resampling
- **Authors:** Juhyeon Kim, Wojciech Jarosz, Adithya Pediredla
- **Venue:** **arXiv preprint only** — [arXiv:2605.11536](https://arxiv.org/abs/2605.11536), submitted May 12, 2026. No peer-reviewed venue found.

**Core idea.** Spatiotemporal reuse for time-resolved light transport, with a *path-length-aware shift mapping* that adjusts reused paths to satisfy temporal gating constraints via a Newton's-method geometric transformation.

**Assessment for you.** *Irrelevant to your renderer*, listed for completeness and because the path-length-constrained shift is a novel shift-map construction.

---

## 5. The 2025–2026 state of the art

### 5.1 ReSTIR PT Enhanced ⭐⭐ — the most useful recent paper for a practitioner

- **Title:** ReSTIR PT Enhanced: Algorithmic Advances for Faster and More Robust ReSTIR Path Tracing
- **Authors:** Daqi Lin, Markus Kettunen, Chris Wyman (NVIDIA)
- **Venue:** Proc. ACM Comput. Graph. Interact. Tech. 9(1), Article 13 (I3D 2026), May 2026 — **Best Paper**
- **DOI:** [10.1145/3804494](https://dl.acm.org/doi/10.1145/3804494) · [project page](https://research.nvidia.com/labs/rtr/publication/lin2026restirptenhanced/)

**Core idea.** Explicitly a performance/robustness paper rather than a theory paper. Four contributions, and I read the full text, so these are described exactly:

1. **Reciprocal / paired spatial reuse.** Correct MIS weights for spatial reuse require shifting the neighbor's path to you *and* your path to the neighbor — two shifts per neighbor. Observation: if pixel A reuses from B, then B can reuse from A **for free**, because the reverse reuse needs the identical pair of shifts. They precompute a "reuse texture" of self-inverting coordinate deltas (A→B and B→A) via repeated random 2×2 shuffles that produce a Gaussian delta distribution with a target σ, then randomly flip/mirror/transpose/offset it per frame to avoid tiling structure. For ReSTIR's default 30-pixel disk radius they derive the matching σ = √(8/9π)·R = 16.0.
2. **Scene-independent reconnection criteria.** Replaces ReSTIR PT's hand-tuned `d_min` / `α_min` thresholds with a **dual ray footprint threshold** plus a single-vertex roughness threshold, derived from the observation that the reciprocal area density of a vertex *is* its footprint. Removes per-scene tuning.
3. **Duplication maps for decorrelation.** After each frame, each pixel counts how many reservoirs in its surrounding **17×17** neighborhood share its random seed (i.e. are shifted copies of the same initial candidate), normalized by 288 to a duplication score D ∈ [0,1]. During temporal resampling, c_Cap is modulated as `c_Cap = lerp(c_Cap^default, c_Cap^min, D^α)` with **c_Cap^default = 20, c_Cap^min = 1, α = 0.1**. This dramatically reduces correlation artifacts from high-energy samples. **The authors are explicit that this introduces a small bias** — it conditions confidence weights on sample identity, violating the MIS partition of unity — measured at 3.25% mean absolute relative bias in a deliberately hard scene. They frame it as trading correlation for bias.
4. **Unifying DI and GI into one reservoir.** Rather than running separate ReSTIR DI and ReSTIR PT passes, trace an extra NEE ray from x₁ during path-tree construction and let the *unified* initial resampling choose a short direct path (d = 2) from the same tree. Removes the cost and storage of a separate DI pass, and — the authors note this was somewhat surprising — *improves* quality, because direct lighting now benefits from the shift mapping, especially on glossy highlights.

Plus low-level work: branches → conditional moves, **reservoir storage cut from 88 to 64 bytes**, stream compaction for random replay, forced NEE reconnection, Russian roulette.

**Measured ablation (ms/frame, 1920×1080, RTX 5880 Ada, averaged over 4 scenes):**

| Step | Total | Initial sampling | Temporal | Spatial | DI + other |
|---|---|---|---|---|---|
| Baseline ReSTIR PT | 35.73 | 10.59 | 5.30 | 14.79 | 5.05 |
| + code micro-opt | 32.98 | 10.70 | 4.16 | 13.06 | 5.07 |
| + forced NEE reconnect | 29.75 | 11.44 | 3.40 | 9.73 | 5.19 |
| + replay compaction | 26.81 | 11.79 | 2.66 | 6.99 | 5.37 |
| + paired spatial reuse | 25.02 | 12.56 | 2.80 | 4.11 | 5.55 |
| + Russian roulette | 16.52 | 5.21 | 2.24 | 3.83 | 5.24 |
| + unify DI & GI | **13.04** | 6.47 | 2.14 | 3.43 | **1.00** |
| + all quality features | 15.53 | 6.77 | 3.20 | 3.73 | 1.83 |

Note that spatial reuse falls from 14.79 ms to 3.43 ms — a 4.3× reduction — and that unifying DI+GI collapses the separate DI pass from 5.24 ms to 1.00 ms.

**Assessment for you.** ⭐⭐ **Read this paper even though you will not implement ReSTIR PT.** Three of its four contributions are independent of the path-reuse machinery and transfer directly:

- **Paired/reciprocal spatial reuse** applies verbatim to ReSTIR DI and ReSTIR GI. It halves the number of target-function evaluations in spatial reuse, which on your software-BVH WebGPU renderer means halving shadow rays in that pass. *Complexity: low-moderate* (generate the reuse texture offline or in a setup compute pass; the tricky part is that spatial reuse must split into a shift pre-pass and a resample pass). *Payoff: high.*
- **Duplication maps for adaptive c_Cap** is a ~30-line addition (one extra pass counting matching seeds in a neighborhood, one lerp at temporal reuse time), directly attacks the fireflies-smearing-across-the-screen artifact that SVGF cannot remove, and is far cheaper than MCMC decorrelation (§3.6). *Complexity: low. Payoff: high.* Accept the ~3% bias; you are already running a biased ReSTIR GI and a biased denoiser.
- **Unifying DI and GI reservoirs** is the strategically interesting one and cuts directly against your current architecture. You currently have separate direct/indirect signals for SVGF. Unifying reservoirs would mean giving that up. **My recommendation: don't unify.** Their gain comes largely from eliminating a redundant pass in a system where both passes were full ReSTIR PT; your DI pass is cheap, and separate direct/indirect is load-bearing for your SVGF setup. Keep them split.
- The footprint-based reconnection criteria only matter if you implement hybrid shift. Skip.

### 5.2 ReSTIR PG — path guiding from resampled paths

- **Title:** ReSTIR PG: Path Guiding with Spatiotemporally Resampled Paths
- **Authors:** Zheng Zeng, Markus Kettunen, Chris Wyman, Lifan Wu, Ravi Ramamoorthi, Ling-Qi Yan, Daqi Lin
- **Venue:** SIGGRAPH Asia 2025 Conference Papers
- **DOI:** [10.1145/3757377.3763813](https://dl.acm.org/doi/10.1145/3757377.3763813) · [project page](https://zheng95z.github.io/publications/restirpg25)

**Core idea.** ReSTIR's ceiling is set by the quality of its *initial candidates*, which are typically poorly distributed and are a source of correlation artifacts. Observation: ReSTIR's accepted paths already approximate the target path-contribution density, so their bounce directions already follow the ideal local guiding distribution — the product of incident radiance and cosine-weighted BSDF. So: fit lightweight guiding distributions to each frame's resampled paths by density estimation, and use them to generate better candidates next frame. A feedback loop where ReSTIR bootstraps its own sampler.

**Assessment for you.** *Complexity:* **moderate-high** (per-region distribution fitting, storage, and update). *Payoff:* **moderate, but conceptually the most promising direction on this list for a low-bounce renderer**, because it improves the thing that actually limits you (candidate quality) rather than the reuse math. Worth revisiting once ReSTIR GI is stable. Not a first project.

### 5.3 Spatio-Temporal Control Variates with ReSTIR (ReSTCV)

- **Title:** Spatio-Temporal Control Variates with ReSTIR for Real-Time Rendering
- **Authors:** Zhong Shi, Cunhao Wu, Lifan Wu, Kun Xu
- **Venue:** SIGGRAPH 2026 Conference Papers — Technical Paper Awards **Honorable Mention**
- **DOI:** [10.1145/3799902.3811113](https://dl.acm.org/doi/10.1145/3799902.3811113) · [code](https://github.com/Hercier/ReSTCV)

**Core idea.** ReSTIR selects one representative sample using a **scalar** target function (usually luminance), which structurally produces **color noise** in scenes with chromatic lighting or materials — the selected sample's hue is right for luminance but wrong for chroma. ReSTCV adapts image-space control variates (an offline technique) to real time via spatiotemporal sample reuse, correcting this.

**Assessment for you.** *Complexity:* **moderate.** *Payoff:* **moderate, and specifically relevant** — if you see colored speckle in your indirect signal that SVGF turns into colored blotches, this is the named cause and a principled fix. (ReSTIR PT Enhanced §6.3 also cites "existing techniques for color noise reduction" for the same problem, so this is a recognized issue, not a niche one.)

### 5.4 Other verified 2025–2026 ReSTIR papers (lower relevance)

| Title | Authors | Venue | DOI |
|---|---|---|---|
| Many-Light Rendering Using ReSTIR-Sampled Shadow Maps | Song Zhang, Daqi Lin, Chris Wyman, Cem Yuksel | CGF (Proc. Eurographics 2025) | [10.1111/cgf.70059](https://onlinelibrary.wiley.com/doi/abs/10.1111/cgf.70059) |
| Real-Time Level-of-Detail Rendering with ReSTIR | Yu-Chen Wang, Markus Kettunen, Daqi Lin, Chris Wyman, Lifan Wu, Shuang Zhao | SIGGRAPH 2026 Conf. Papers | [10.1145/3799902.3811100](https://doi.org/10.1145/3799902.3811100) |

The LoD paper is worth one sentence because the *problem* it identifies is subtle and general: **prior ReSTIR methods require the same mesh topology across frames**, so temporal reuse silently breaks whenever an LoD switch occurs. Their fix is a surface point mapping that lets samples be reused across differing topologies. If you ever add LoD or geometry streaming, remember this failure mode exists.

---

## 6. World-space reservoirs, radiance caches, and neural methods

This section matters for you because a 1–2 bounce renderer loses a lot of energy at the truncation point, and a cache is the cheapest way to get it back.

### 6.1 ReGIR — grid-based reservoirs for many lights

- **Title:** Rendering Many Lights with Grid-Based Reservoirs
- **Authors:** Jakub Boksansky, Paula Jukarainen, Chris Wyman
- **Venue:** *Ray Tracing Gems II*, Apress, 2021, Ch. 23, pp. 351–365
- **DOI:** [10.1007/978-1-4842-7185-8_23](https://doi.org/10.1007/978-1-4842-7185-8_23) · [PDF](https://boksajak.github.io/files/RTG2_ReGIRGridBasedReservoirs.pdf)
- (An earlier I3D 2021 poster, "Rendering of Many Lights with Grid-Based Reservoirs", covers the same idea.)

**Core idea.** Fill a world-space grid with reservoirs of light samples, then sample from the cell containing a shading point. Because it is world-space it works at *any* path vertex, not just the primary hit.

### 6.2 World-Space Spatiotemporal Reservoir Reuse (AMD GI-1.0)

- **Title:** World-space spatiotemporal reservoir reuse for ray-traced global illumination
- **Author:** Guillaume Boissé (AMD)
- **Venue:** SIGGRAPH Asia 2021 **Technical Communications**
- **DOI:** [10.1145/3478512.3488613](https://dl.acm.org/doi/10.1145/3478512.3488613) · [PDF](https://gpuopen.com/download/SA2021_WorldSpace_ReSTIR.pdf)

**Core idea.** Cache path-vertex reservoirs into cells of a **GPU-built hash grid**, enabling stochastic reuse of neighboring reservoirs across space and time at arbitrary points in space — i.e. light sampling at any eye-path vertex, not just the primary hit. This became the basis of AMD's GI-1.0.

### 6.3 World-Space Spatiotemporal Path Resampling for Path Tracing

- **Title:** World-Space Spatiotemporal Path Resampling for Path Tracing
- **Authors:** Hangyu Zhang (Xi'an Jiaotong University), Beibei Wang (Nankai University / NJUST)
- **Venue:** Computer Graphics Forum 42(7) (Pacific Graphics 2023)
- **DOI:** [10.1111/cgf.14974](https://onlinelibrary.wiley.com/doi/10.1111/cgf.14974)
- **Results:** 16.6%–41.9% MSE improvement over the prior method at 4.4%–8.4% extra time.

**Core idea.** Caches *sub-paths* (not just light samples) in a world-space grid so reuse can start from non-primary vertices, with a normal-aware hash grid construction to avoid mixing incompatible surfaces into one cell.

**Assessment of the world-space family for you.** *Complexity:* **moderate** — a GPU hash grid with insertion, per-cell reservoir arrays, and cell eviction. WGSL gives you `atomicCompareExchangeWeak` on `atomic<u32>` in storage, which is enough to build a linear-probing hash grid, but this is a real subsystem, not an afternoon. *Payoff:* **moderate**, mainly for extending effective bounce depth beyond your 1–2 and for handling shading points that screen-space reuse can't (off-screen, behind geometry).

⚠️ **But read this warning from the course notes first**, because it is aimed squarely at this idea: *"Basic ReSTIR gives you probability distributions at a point. Generally, a reservoir is not valid over, say, an entire voxel. You can store and use reservoirs that way, but it is very difficult to avoid adding bias (and magnifying correlations within the voxel)."* Every world-space reservoir scheme is trading correctness for reach. Know that going in.

### 6.4 SHaRC — Spatially Hashed Radiance Cache

- **Not a paper.** NVIDIA RTXGI 2.0 SDK component: [NVIDIA-RTX/SHARC](https://github.com/NVIDIA-RTX/SHARC), shipped alongside [RTXGI](https://github.com/NVIDIA-RTX/RTXGI).

**Core idea.** A world-space radiance cache on a spatial hash: query it at any path hit point to get outgoing radiance, and terminate the path there. Unlike NRC (below) it uses **no neural network** and works on any DX/VK ray-tracing GPU.

**Assessment for you.** ⭐ *Complexity:* **moderate** (hash grid + accumulation + a resolve pass; the SDK is HLSL shader-only sources so it's readable but not directly portable to WGSL). *Payoff:* **high, and complementary rather than competing with ReSTIR.** This is the standard answer to "I only have 1–2 bounces and my indirect looks dark." You path-trace 1–2 bounces, then query the cache to approximate the infinite tail. The cache is temporally accumulated so it is nearly noise-free, which means it *reduces* the burden on both ReSTIR and SVGF instead of adding to it. Strongly consider this alongside ReSTIR GI — arguably the highest-payoff non-ReSTIR item in this document for your specific bounce budget.

### 6.5 Neural Radiance Caching

- **Title:** Real-time neural radiance caching for path tracing
- **Authors:** Thomas Müller, Fabrice Rousselle, Jan Novák, Alexander Keller (NVIDIA)
- **Venue:** ACM Transactions on Graphics 40(4) (Proc. SIGGRAPH 2021)
- **DOI:** [10.1145/3450626.3459812](https://dl.acm.org/doi/10.1145/3450626.3459812) · [arXiv:2106.12372](https://arxiv.org/pdf/2106.12372)

**Core idea.** Online-trained MLP radiance cache; paths terminate into the network. Same role as SHaRC, with a neural representation.

**Assessment for you.** *Payoff: skip.* NRC depends on fully-fused MLP kernels exploiting tensor cores and fast shared-memory matrix ops. WebGPU has no cooperative-matrix or tensor-core access; a WGSL MLP would be far too slow. **Use SHaRC's non-neural approach instead** — it exists precisely because NRC's hardware requirements are restrictive.

### 6.6 Neural Visibility Cache — flagged as unverified venue

- **Title:** Neural Visibility Cache for Real-Time Light Sampling
- **Authors:** Jakub Bokšanský, Daniel Meister (AMD)
- **Venue:** **arXiv only** — [arXiv:2506.05930](https://arxiv.org/abs/2506.05930), v1 June 2025, v2 August 2025. No peer-reviewed venue found as of this writing.

**Core idea.** Online-trained fully-fused MLP with multi-resolution hash-grid encoding caching light↔position visibility, feeding visibility into WRS for light selection. Integrated with ReSTIR as an *initial candidate generator*, it reportedly accelerates convergence during disocclusion at ~5% overhead and cuts shadow-ray counts.

**Assessment for you.** *Skip* — same WebGPU/tensor-core problem as NRC. Listed because "improve the initial candidates" (cf. ReSTIR PG, §5.2) is clearly the direction the field is moving, and because "ReSTIR converges badly through disocclusion" keeps recurring as *the* open problem across §3.4, §4.5, §4.6 and this paper.

---

## 7. Practical implementation guidance

### 7.1 The course notes — yes, they exist, and they are the right starting point

- **Title:** A Gentle Introduction to ReSTIR: Path Reuse in Real-Time
- **Authors:** Chris Wyman, Markus Kettunen, Daqi Lin, Benedikt Bitterli, Cem Yuksel, Wojciech Jarosz, Pawel Kozlowski, Giovanni De Francesco
- **Venue:** ACM SIGGRAPH 2023 Courses
- **DOI:** [10.1145/3587423.3595511](https://dl.acm.org/doi/10.1145/3587423.3595511)
- **Course site:** [intro-to-restir.cwyman.org](https://intro-to-restir.cwyman.org/) · **Notes PDF (rev. March 4, 2024):** [2023ReSTIR_Course_Notes.pdf](https://intro-to-restir.cwyman.org/presentations/2023ReSTIR_Course_Notes.pdf)

~55 pages, 9 chapters, includes a Cyberpunk 2077 RT Overdrive integration section by Kozlowski & De Francesco. **This is the best single document in the ReSTIR literature.** Note the notes have been revised since the 2023 course — get the March 2024 version.

### 7.2 M-caps / confidence weights — the actual guidance

The modern framing is *confidence weights* c, not sample counts M. Resampling MIS weight becomes m_i(x) = c_i·p̂_i(x) / Σ_j c_j·p̂_j(x). When merging reservoirs of confidence c₁ and c₂, the result gets c₁ + c₂.

The problem: summing confidences is a **drastic overestimate** of the true effective sample count, since only a limited number of genuinely new samples enter the pool each frame. Left uncapped, confidences grow exponentially (each spatial reuse multiplies them), new samples get exponentially decreasing relative weight, and **the algorithm converges to the wrong result**.

Verbatim recommendation from the notes: *"We commonly cap the sample confidence to somewhere between 5–30. Starting with a cap of 20 is usually good."* ReSTIR PT and ReSTIR PT Enhanced both use **c_Cap = 20** for temporal resampling as the default.

Other confidence rules:
- New independent sample → c = 1. RIS-selecting one of M new samples → c = M.
- Pixels entering the screen (no temporal predecessor) → reset c to 0. It often makes sense to reset on detected occlusion/disocclusion too.
- ⚠️ **Resetting confidence is only legal if the decision is made from the G-buffer.** Resets that depend on sample details introduce bias.
- There is no scene-agnostic optimal cap: small caps underuse history (more variance), large caps increase correlation and make outliers decay slowly, so a firefly gets smeared across neighbors over many frames before it dies. ReSTIR PT Enhanced's duplication maps (§5.1) are the current best answer — adapt c_Cap downward locally where correlation is measured to be high.

### 7.3 Visibility reuse — handle with extreme care

Direct quote from the notes' "Advice for getting started": *"Reuse visibility very carefully. An original appeal of ReSTIR was the ability to reduce ray budgets by reusing visibility samples. Visibility reuse also causes many problematic biases people have great difficulty debugging. (Arguably, it causes most difficult-to-debug biases.) Consider always using visibility in your target functions and MIS weights until you have validated your code works with full visibility. Only then accelerate your algorithm by incrementally starting to reuse ray queries."*

And crucially: *"ReSTIR accelerates in multiple ways. One is by amortizing sample costs across pixels. This benefit remains, even when not reusing visibility."*

**Translation for you:** the common belief that ReSTIR's speedup is fundamentally about reusing shadow rays is wrong. Most of the win is sample amortization, which you keep even with full visibility in p̂. Given that you're on a software BVH where a mis-debugged bias could cost you weeks, keep V in your target function.

The recommended target function for DI is the full integrand: **p̂(x) = f_s(x)·G(x)·V(x)·L_e(x)**. Talbot's original RIS drops V for speed, which worsens the limit distribution and requires extra conditions for correctness.

### 7.4 Temporal bias — where it comes from and what people actually do

Correct temporal MIS weights require evaluating **last frame's p̂ at this frame's sample**, i.e. m_i(X_i) = p_i(X_i) / (p_i(X_i) + p_j(X_i)) where the boxed term p_j(X_i) needs the *previous frame's* scene. If p̂ includes visibility, that means keeping the previous frame's BVH around and tracing against it. Nobody wants to do that.

The notes lay out the approximations and their consequences precisely:

| Approximation | Consequence |
|---|---|
| Use current-frame BVH as a stand-in for prior-frame BVH | small bias, usually acceptable; motion-dependent |
| Assume p_j(X_i) = 0 | **brightening bias** — asserts no sample generated this frame could have been picked last frame, which over-represents them |
| p̃_j(X_i) > p_j(X_i) | lowers m_i → **darkening bias** |
| p̃_j(X_i) < p_j(X_i) | raises m_i → **brightening bias** |
| Recompute with last frame's data but assume visibility unchanged | usually the pragmatic choice |

The same approximation can brighten in one region and darken in another, and can flip under different motion. This is why temporal bias in ReSTIR shows up as motion-dependent shimmer rather than a uniform offset.

**Also:** naive temporal reuse assigns equal weight to the previous frame's sample and the new sample, which **loses roughly 50% of accumulated history every frame**. Confidence weights (§7.2) exist to fix exactly this.

### 7.5 Neighbor rejection

Rejecting dissimilar neighbors (normals, depths, material properties) is standard, and the notes formalize it as an approximation of Veach's *cutoff heuristic*: techniques with too-low PDF values just get dropped from the MIS weight. Combined with 1/M weights it is the original biased ReSTIR.

The bias-safety rule is sharp: **heuristics are unbiased iff they do not look at individual samples or weights.** Reasoning about *domains* (G-buffer geometry) is fine. Making decisions from *sample* contents conditions them and moves you into conditional probability space (§2.3).

### 7.6 The build order the authors recommend

1. A **ground-truth reference path tracer in the same codebase, on the same scenes.** Non-negotiable. *"You do not want to spend months debugging your ReSTIR implementation or integration only to discover, at the very end, that it is biased in some unacceptable way. (This has happened.)"*
2. Basic RIS, validated against ground truth.
3. Candidate generation + integration only.
4. **Spatial reuse** — easier to validate because the scene doesn't change between samples.
5. **Temporal reuse without motion** (still camera). Validate that averaging many still frames converges to ground truth.
6. Temporal reuse with motion.
7. Only then optimize.

Also: *"Don't try to get too clever too fast... Many [RTXDI options] were never intended to be unbiased, and options may not have been tested in all permutations."*

### 7.7 Notes specific to WebGPU / Apple silicon that the literature does not cover

These are my inferences from the papers plus the platform, not claims from any source — treat them as engineering judgment:

- **Rays are your scarce resource, not ALU.** Every paper above assumes hardware ray tracing. On a software BVH in a compute shader, techniques whose cost is "one more ray per neighbor per pixel" are 5–20× more expensive relative to everything else than the papers' timings suggest. This shifts the calculus decisively toward §3.1 (pairwise MIS, O(M) instead of O(M²) evaluations) and §5.1 (paired spatial reuse, halving shifts), and decisively away from anything using random replay.
- **Divergence.** ReSTIR PT Enhanced spends a whole section on divergence reduction (branches → conditional moves, stream compaction). Apple GPUs have 32-wide SIMD groups and are quite sensitive to divergence; the "replace branches with `select()`" advice transfers directly to WGSL.
- **Reservoir size is a real budget.** They cut ReSTIR PT reservoirs from 88 to 64 bytes and considered it worth reporting. At 829k pixels, 64 B/reservoir = 53 MB per buffer. Pack aggressively: octahedral normals (2×16-bit), RGB9E5 or half3 radiance, and consider 16-bit for the confidence weight.
- **No float atomics** in WGSL storage, which constrains reservoir splatting (§4.5) and world-space grid insertion (§6.1–6.3). You can work around it with `atomicCompareExchangeWeak` CAS loops on bitcast u32, but expect contention.
- **Subgroup operations** in WGSL are relatively recent and availability varies; don't build your reservoir merge around them until you've checked support on your target.

---

## 8. Recommended order of work for your renderer

Ranked strictly by (expected quality gain) ÷ (implementation risk), given ReSTIR DI already working, 1–2 bounces, 1152×720, software BVH, SVGF with split direct/indirect.

| # | Item | Where | Effort | Payoff |
|---|---|---|---|---|
| 1 | **Ground-truth reference mode** in-engine (accumulate N thousand samples, compare) | §7.6 | 1 day | Prerequisite for everything. Do not skip. |
| 2 | **Defensive pairwise MIS weights** in DI, replacing 1/M + rejection | §3.1 | 1–2 days | High — unbiased spatial reuse at O(M) ray cost |
| 3 | **ReSTIR GI** | §2.1 | 1–2 weeks | Highest single-feature gain; reuses all your DI plumbing |
| 4 | **Confidence-weight audit**: c_cap = 20, reset on disocclusion, G-buffer-only reset decisions | §7.2 | 1 day | High — this is where silent non-convergence lives |
| 5 | **Compatibility-guided neighbor selection** | §3.5 | 2–3 days | High: −6–29% SMAPE, −22–49% temporal covariance, +2–5% cost. The covariance number is what your SVGF cares about. |
| 6 | **Duplication maps → adaptive c_Cap** (17×17 seed-match count, lerp c_Cap 20→1, α=0.1) | §5.1 | 2–3 days | High — kills smeared fireflies, ~3% bias, far cheaper than MCMC |
| 7 | **Paired / reciprocal spatial reuse** | §5.1 | 3–5 days | High — halves shift work in spatial reuse |
| 8 | **SHaRC-style world-space radiance cache** to terminate paths | §6.4 | 1–2 weeks | High for a 1–2 bounce renderer; recovers the missing energy tail; noise-free so it helps SVGF |
| 9 | Retune SVGF variance estimation for ReSTIR's correlated inputs | §2.1 | ongoing | Necessary; SVGF's variance estimate under-reports when neighbors share reservoirs |
| 10 | Stochastic pairwise MIS (large-kernel reuse) | §3.4 | 3–5 days | Moderate — only if disocclusion noise is your visible problem |
| 11 | ReSTCV (color noise) | §5.3 | 1 week | Moderate — only if you see chromatic speckle |
| — | *Do not do:* ReSTIR PT / hybrid shift, ReSTIR BDPT, NRC, Area ReSTIR, multi-layer splatting | §2.2, §4.7, §6.5, §4.4, §4.6 | — | Cost/benefit fails on WebGPU at 1–2 bounces |

---

## 9. Commonly repeated claims that are wrong or imprecise

Each of these is something I found stated loosely in secondary sources and then checked against primary text.

1. **"ReSTIR is unbiased."** Imprecise. Bitterli et al. 2020 present *two* estimators. The headline-fast one uses constant 1/M resampling weights plus neighbor rejection and is **biased** (the paper itself describes energy loss). Unbiasedness requires either contribution MIS weights or proper resampling MIS weights. Most shipped implementations, including options in RTXDI, are biased by choice. *(Course notes §7.1.1–7.1.2.)*

2. **"ReSTIR GI reuses paths."** Wrong. ReSTIR GI reuses **one secondary vertex plus a cached scalar-per-channel outgoing radiance**, via an identity/reconnection shift at x₂. It does not store or transport a path. *(Course notes §6.3; RTXDI `RestirGI.md`.)*

3. **"ReSTIR GI is unbiased, or can be made unbiased by adding MIS weights."** Wrong. ReSTIR GI has a *structural* bias independent of its MIS weights: it caches L_o along direction x₂→x₁ and reuses it along x₂→y₁, which is only correct for view-independent (Lambertian) BSDFs at x₂. Fixing the MIS weights does not fix this. An unbiased variant would have to re-evaluate two BSDFs, the geometry factor, and trace a shadow ray at reuse time. *(Course notes §6.3, explicitly.)*

4. **"The ReSTIR M-cap is 20."** Two different quantities get conflated here, and blog posts routinely mix them:
   - The 2020 DI paper clamps the *previous frame's* M to at most **20× the current reservoir's M** (a ratio).
   - The modern formulation caps the *confidence weight* c to an absolute constant, recommended **5–30, start at 20** (course notes), with ReSTIR PT using c_Cap = 20.
   These are not the same number and not the same rule. ⚠️ **Partially unverified:** I could not fetch the 2020 paper PDF (>10 MB), so the "20× ratio" form is sourced from third-party implementations and blog write-ups, not the paper text. The absolute c_Cap ∈ [5,30] guidance **is** verified from the course notes and ReSTIR PT Enhanced.

5. **"Bigger M-cap is better because you accumulate more history."** Wrong. Larger caps increase correlation and slow outlier decay — a firefly gets progressively smeared to neighbors over many frames before it is replaced, producing sample impoverishment and visible correlation artifacts. There is no scene-agnostic optimum, which is exactly why ReSTIR PT Enhanced makes it adaptive.

6. **"ReSTIR's speedup comes from reusing shadow rays."** Misleading. The course notes state the amortization benefit *"remains, even when not reusing visibility."* Visibility reuse is an additional optimization and, per the authors, *"arguably causes most difficult-to-debug biases."*

7. **"You can store reservoirs in a voxel/hash grid and reuse them like screen-space reservoirs."** Dangerous oversimplification. Course notes: *"a reservoir is not valid over, say, an entire voxel... it is very difficult to avoid adding bias (and magnifying correlations within the voxel)."* World-space reservoir papers exist (§6.1–6.3) precisely because doing this correctly is nontrivial.

8. **"Neighbor rejection heuristics are free/safe."** Only if they read the G-buffer. Any heuristic that inspects reservoir contents or sample weights conditions the samples and invalidates the MIS weights. *(Course notes §7.1; formalized in CRIS.)*

9. **"ReSTIR PT superseded ReSTIR GI / ReSTIR DI."** Not how it played out. ReSTIR PT originally *assumed a separate DI method existed and skipped direct paths in its own estimator.* It took until ReSTIR PT Enhanced (2026) for DI and GI to be unified into one reservoir — and that was reported as a somewhat surprising quality win, not an obvious consolidation.

10. **"Random replay has a Jacobian you have to compute per vertex."** Imprecise. In **primary sample space** the random-replay portion of a shift has an **identity Jacobian determinant**; only the reconnection phase contributes Jacobian terms. This is one of the two stated reasons ReSTIR PT uses PSS. Outside PSS, you would indeed need per-vertex terms. *(Course notes §6.6.)*

11. **"ReSTIR PT Enhanced's duplication-map decorrelation is free and unbiased."** Wrong, and the authors say so directly: conditioning c_Cap on sample identity violates the MIS partition of unity and introduces bias, measured at **3.25% mean absolute relative bias** in their hardest test scene. It is a deliberate correlation-for-bias trade.

---

## 10. Things I could NOT verify

Listed so you can weight the above accordingly.

1. **The ReSTIR GI (CGF 2021) paper text.** The only PDF I located is 157 MB (NVIDIA CDN) and exceeded fetch limits. Title/authors/venue/abstract/results **are** verified from the Wiley CGF listing and NVIDIA's publication page. The reservoir-contents and bias description in §2.1 comes from the SIGGRAPH 2023 course notes §6.3 and the RTXDI implementation docs — authoritative, but not the paper itself.
2. **The ReSTIR DI (2020) paper text.** PDF exceeded fetch limits. Metadata, abstract and headline numbers verified via ACM DL, Dartmouth, and NVIDIA. The "20× M" temporal clamp form is **not** verified from the paper (see §9 item 4).
3. **The GRIS (2022) paper text.** PDF exceeded fetch limits. Metadata and abstract verified. The shift-mapping and Jacobian material in §2.2 is verified from the course notes chapters 5 and 6 (same author group, and the notes cite GRIS for each specific claim), plus ReSTIR PT Enhanced §2 which restates the reconnection Jacobian (Eq. 2) explicitly.
4. **The Compatibility-Guided Neighbor Selection (HPG 2026) paper text.** PDF exceeded fetch limits. Authors, venue, Best Paper award, and the quantitative results (6–29% SMAPE, 22–49% temporal covariance, 2–5% cost) are from NVIDIA's own publication page and the ACM DL listing, not from reading the paper.
5. **Venue for ToF ReSTIR** (§4.10) and **Neural Visibility Cache** (§6.6) — both appear to be arXiv-only. No peer-reviewed venue found for either.
6. **Ghost ReSTIR** (§4.2) is a **SIGGRAPH Posters** entry, i.e. a short abstract, not a full paper. Its reported speedups are not peer-reviewed at full-paper depth.
7. **Author list for Gradient-Domain ReSTIR Path Tracing** (§4.9) came from a search snippet of the Wiley/CGF listing rather than a direct fetch of the publisher page; it is identical to the Real-Time LoD paper's author list, which is plausible (same group) but I flag the coincidence.
8. **`cwyman.org/paperList.html` renders its list via JavaScript.** The 2026 entries were recovered from it, but full author lists were not available from that page; each 2026 entry's authorship was separately confirmed via research.nvidia.com, ACM DL, or graphics.cs.utah.edu.
9. **Exact page numbers / article numbers** for several 2025–2026 entries were not confirmed; DOIs are given instead, which are stable.
10. **SIGGRAPH 2026 papers list on kesen.realtimerendering.com** has an "Efficient Sampling: ReSTIR and More" session whose entries were empty placeholders when fetched. There may be additional 2026 ReSTIR papers I did not find.

---

## 11. Source index

**Course & surveys**
- [A Gentle Introduction to ReSTIR (SIGGRAPH 2023 Courses)](https://intro-to-restir.cwyman.org/) — [notes PDF](https://intro-to-restir.cwyman.org/presentations/2023ReSTIR_Course_Notes.pdf) — [ACM DL](https://dl.acm.org/doi/10.1145/3587423.3595511)

**Author / lab publication pages** (best places to track new work)
- [Chris Wyman](https://cwyman.org/paperList.html) · [Daqi Lin](https://dqlin.xyz/publications/) · [NVIDIA Real-Time Graphics Research](https://research.nvidia.com/labs/rtr/publication/) · [Utah Graphics Lab](https://graphics.cs.utah.edu/research/projects/)

**Code**
- [ReSTIR PT (GRIS)](https://github.com/DQLin/ReSTIR_PT) · [Volumetric ReSTIR](https://github.com/DQLin/VolumetricReSTIRRelease) · [Conditional ReSTIR](https://github.com/NVlabs/conditional-restir-prototype) · [Area ReSTIR (DI part)](https://github.com/guiqi134/Area-ReSTIR) · [Reservoir Splatting](https://github.com/Jebbly/Reservoir-Splatting) · [ReSTIR SSS](https://github.com/MircoWerner/ReSTIR-SSS) · [ReSTIR FG](https://github.com/TU-Clausthal-Rendering/ReSTIR-FG) · [ReSTCV](https://github.com/Hercier/ReSTCV) · [Gradient-domain ReSTIR](https://github.com/elite-sheep/gradient-restir) · [RTXDI](https://github.com/NVIDIA-RTX/RTXDI) · [SHaRC](https://github.com/NVIDIA-RTX/SHARC) · [RTXGI](https://github.com/NVIDIA-RTX/RTXGI)
