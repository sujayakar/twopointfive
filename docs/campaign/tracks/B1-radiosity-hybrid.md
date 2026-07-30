# Track B1 — radiosity → sampling hybrid

Branch `claude/radiosity`, base `be298ab` (campaign trunk after Track A).
Commits, in order: `4027994` headless diagnostics (readback hooks,
`__freezeClock`, crouch matrix scenario), `85f9362` the crouch fix,
`e0771c9` the `indirectMode` split, `2b89f4c` gather + patchRIS, then the
report/verification commits on top.

## What changed and why

The radiosity solve stays exactly as it was: baked form factors, per-frame
inject (baked light tables + torch depth maps, zero rays), two warm Jacobi
steps. What changed is who trusts it, and how much.

1. **Crouch bug (Deliverable 0).** The flashlight enters the solve ONLY
   through torch depth-map layer 0. Crouched, the aim layer is off and the
   crouch idle pose parks the pistol slide 2-3 cm in front of the lens, so
   the pinhole map traced from the lens centre read 87-93% of its texels as
   blocked at centimetre depth (measured, below) → zero injected flashlight
   energy → the room's flashlight bounce vanished, and the depth-map beam
   march lost the shaft. Fix in the codebase's own idiom: the flashmap trace
   skips its owner's dynamic group (`traceSkipping`, same shape as the
   probe's `occludedSkipping`); owner groups are a per-frame table
   (`setTorchGroups`) — a table rather than a derivation from the light
   index because a dead guard keeps its packed body (a corpse to drag) but
   leaves the light list, so light order and group order diverge as soon as
   anyone is down and only the game knows the pairing. `torchVisPoint`
   (radiosity inject) gets the trace pass's 4-tap PCF so patch shadows ramp
   over a texel instead of stepping.
2. **`settings.radiosity` → `settings.indirectMode`** (Deliverable 1):
   `"traced" | "radiosityRead" | "gather" | "patchRIS"`, panel select,
   persisted (store revision 4 migrates a stored `radiosity=false` to
   `traced`, and enum values are validated on load). Threaded into
   `Uniforms` at **byte 864**: `indirectMode` u32 (864), `radPatchCount`
   u32 (868), two pads to 880; `UNIFORM_SIZE = 880` (bytes 880-943 are the
   volumetrics track's, untouched). The solve runs for every mode but
   `traced`; reference mode and a patchless scene both force `traced` on
   the CPU (`effectiveIndirectMode`), so `__compareToReference` never
   validates an approximation against itself. The old `radiosityOn` float
   is now a legacy mirror ("the solve is live this frame"); the shader
   branches on `indirectMode`.
3. **Mode `gather`** (Deliverable 2): static primary hits trace bounce 1 for
   real and, at that x1, add the solve's incident irradiance re-radiated by
   x1's albedo (`radiosityIndirect(x1) · albedo(x1)`) instead of tracing on.
   The character standing in the beam is now a real occluder for the
   floor's bounce; the solve is only ever consumed one bounce away from the
   eye, where its baked-static error is diffused rather than photographed.
   Fallbacks are the existing paths, per vertex: dynamic x1 (a bounce ray
   that hits a character has no patch) and patchless faces (`-1` sentinel)
   fall through to the traced continuation (which at the default 1 bounce
   is x1's NEE only). x1's direct NEE (`sampleIndirectRIS`) is kept — the
   solve's G row is one-bounce-removed radiosity, so NEE + sky row + G is
   the complete, non-overlapping set at x1. The gathered x1 is an ordinary
   x2 sample as far as ReSTIR GI is concerned, so GI reuse composes with no
   change. Checkerboard stance: gather traces exactly one bounce per
   pixel, which is the traced-1-bounce cost regime, so the CPU's existing
   `bounces <= 1 → indirectRate inert` rule is left keyed to the bounces
   setting; when the rate does apply, gather pixels sit tiles out exactly
   like traced (their validity alpha follows `traceIndirect`).
4. **Mode `patchRIS`** (Deliverable 3): at the primary vertex (b == 0,
   after `restirDirect`), the solved patches ARE the light list. Per pixel:
   M = 8 patches drawn from a per-frame inclusive CDF over
   `luminance(B_j)·A_j`, one point jittered on each patch (tangents + half
   extents), candidate radiance `B_j/π` folded through `cos_j·A_j/d²`
   (point-equivalent VPL; `d²` clamped by `+A_j/4` against the singularity,
   the same disk-kernel guard `bakeFF` uses), weighted reservoir keeps one
   by `risTarget`, ONE `occluded()` shadow ray to the survivor. Serves
   DYNAMIC primary hits too — a character standing in a lit room is lit by
   the room, which no other path does. Static hits add their own sky
   irradiance (the sky row) since patches only carry surface-to-surface
   transfer; dynamic hits go without a sky term (no patch data — see
   Findings). Data plumbing exactly as briefed: `radStatic` bound
   read-only at `@group(1) @binding(16)` — the trace pass's **10th and
   last** storage buffer (`gpu.ts` requirement 9 → 10, comment updated);
   the emitter radiosity B (with the sky term folded in, as an emitter
   should carry it) rides a third row of the existing `radGSky` texture,
   and the CDF a fourth (`buildPatchCdf`, one 256-thread workgroup:
   chunk-scan + Hillis-Steele over the chunk sums; dispatched only in this
   mode); each patch's RIS weight parks in the free `.w` lane of the B
   ping-pong slot so the scan needs no new buffer. **Not composed with the
   GI reservoir** — see Findings for why that is a measured "not yet", not
   an oversight; patchRIS pixels write an empty GI reservoir.

Two things the estimator taught, both now in the code with the numbers:
the light reservoirs' `W` cap (24-32, sized for a ~1/lightCount source
pdf) chops nearly every patch survivor, whose source pdf is ~1e-3-1e-4
and whose normal `W` is in the hundreds — with the cap patchRIS read
relBias −0.35 vs −0.19 without it; and `SHADOW_CULL` cannot apply to a
value that is the pixel's WHOLE indirect estimate rather than one light of
many. Both removed for this path only.

## Verification

`npm run typecheck` and `npm run build` clean at every commit. Everything
below ran headless on SwiftShader (sandbox off, "chromium needs its own
namespaces"), small resolutions throughout. Screenshots were Read; the
descriptions are of what is actually in them.

### Deliverable 0 — the crouch bug, before/after

New tooling for this (commit `4027994`): `__readFlashmap(layer)` reads a
torch depth-map layer back; `__readRadiosity()` reads the solve's injected
energy E and B; `__freezeClock(on)` feeds every frame the same timestamp
(dt = 0) so two captures of a moving character are the SAME scene — the
`__compareToReference` freeze, generalised. `tools/headless/scenarios/
crouch-matrix.js` pins the player at the corridor spawn, freezes the
guards with their torches out (the flashlight is the only moving light),
and reports everything as flashlight-on minus flashlight-off deltas.

Premise check first (the campaign's diagnosis, verified, not assumed):
crouched depth-map coverage varies with aim and with the crouch idle
clip's arm sway — one aim sweep read 45-86% and one settled pose sampled
6× over the clip read 88.6-92.9% (the campaign's "78-92%" is this range).
So the crouched cells are sampled over the clip, then the light deltas
taken with the clock frozen at one settled pose.

| quantity | before (`be298ab`) | after (`85f9362`) |
|---|---|---|
| layer-0 texels < 10 cm, standing | 0.0% | 0.0% |
| layer-0 texels < 10 cm, crouched (min / median / max over 6 clip samples) | 88.6 / 91.1 / 92.9% | 0.0 / 0.0 / 0.0% |
| Σ luminance(injected E), flash on − off, standing | 58.28 | 58.18 |
| Σ luminance(injected E), flash on − off, crouched | **0.000** | **33.18** |
| shaft near player (window mean, vol 1.0 − vol 0), depth-map march | 0.00002 | 0.01313 |
| shaft near player, real-ray march (control) | 0.00521 | 0.01069 |

Reconciliation: standing injection is unchanged by the fix (58.28 → 58.18,
inside its own run-to-run spread), so the group skip does not leak energy
into the standing case. Crouched injection goes from exactly zero to 33 —
57% of standing rather than 100% because the crouched beam starts at thigh
height and pools closer, injecting genuinely less; the fix is not
inflating it. The shaft control row is the honest caveat: after the fix
the depth-map shaft (0.0131) reads brighter than the real-ray shaft
(0.0107), because the map now ignores the character entirely while real
rays still hit the thigh in front of the lens — see Findings ("self-skip
means the owner never occludes its own torch in the map"). Before the fix
the map's shaft was zero, i.e. below both.

Screens (`crouch-before7.png` / `crouch-after1.png`, default view, 320×164
internal): the crouched player at the corridor spawn with a bright pool at
his feet; after the fix the pool carries a soft haze halo and the crates
and legs around it pick up warm bounce that is absent before — the window
around the player measures 28% brighter (8-bit luma 22.0 → 28.1) with
nothing else in the frame changed.

### Deliverables 1-3 — per-mode correctness (`__compareToReference`)

TODO after final run.

### Per-mode work counters

TODO after final run.

### The headline: the character shadows the bounce light

TODO after shots.

## Recommendation

TODO.

## What SwiftShader cannot say, and the Mac bench script

TODO.

## Findings (not fixed)

TODO.

## Merge notes

TODO.
