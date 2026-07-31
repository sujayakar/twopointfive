# Track B4 — temporal hardening (dynamic reprojection audit)

Branch `claude/temporal`, forked from `claude/campaign` @ `230295a`.
Everything below was measured on the headless SwiftShader harness at
384×240 unless stated otherwise (`tools/headless/run.py`, sandbox off),
with the requestAnimationFrame loop paused (`__benchGuard`) so scripted
motion frames were not interleaved with free static frames, guards parked
off-screen, and the walk circle placed on ray-cast-verified open floor.

## Findings

### 1. The character did shed its history — continuously, even standing still

Instrument: `__historyStats(rects)` reads the direct-signal moments texture
(history length = frames a pixel has kept) over the union of the player's
projected limb-box rects; the control is a ring of static floor around the
character (6–22 px out, so it excludes the silhouette's disocclusion
halo). "geom" runs use the reference reprojection parameters — no colour
clamp, no history cap — so history length is a pure count of frames since
the last *geometric* validation failure. That isolates the reprojection
from the estimator's clamp heuristics.

**Baseline (before), geom config, 30 warm frames then a 1.8 m circle walk
(45 frames, ~9.6 cm and one animation step per rendered frame):**

| Sample | Static ring median | Character median | Character < 4 frames | Character reset that frame |
|---|---|---|---|---|
| standing (30 frames) | 30 / 30 | **28.2** | **16.5 %** | **6.5 %** |
| walk f15 | 45 / 45 | 11.0 | 21.0 % | 10.8 % |
| walk f30 | 60 / 60 | 24.1 | 15.2 % | 6.6 % |
| walk f45 | 75 / 75 | 36.7 | 15.1 % | 11.0 % |

Static geometry under camera motion holds perfectly (ring = frame count,
< 0.5 % of ring pixels short). The character never does: at every sample
15–21 % of its pixels are under 4 frames old and 6–11 % reset that very
frame, and it churns *while standing still* (torso median 24.8, forearms
12.3 of a possible 30). The history-length map of the standing character
(`__historyRect`, `u` = 30 frames):

```
uuuuuuuuuuuuuuuutt761qpuuuuuu   <- background 30, character interior 1-9
uuuuuuuuuuuuuuuuu813c*puuuuuu      (recent resets) among p-s (25-28)
uuuuuuuuuuuuuuuuu8*31p2uuuuuu
uuuuuuuuuuuuuuuuutnop*5uuuuuu
```

### 2. Mechanism: the normal test, tripped by the character against itself

Ablations, same probe:

- Normal test disabled in `validTap` (accept any tap that passes depth):
  character reads **30/30 standing, 45/45 and 60/60 walking, zero pixels
  < 4** — the depth test and screen bounds contribute nothing.
- Dynamic-pixel taps accepted unconditionally: identical result, except a
  few percent of *rect* pixels short — those are the trailing-edge
  disocclusion halo (floor revealed behind the walking character), i.e.
  correct resets counted against the character by the box-shaped rects.

So 100 % of the character's geometric history loss is the `0.90` normal
test, and it fires without any motion. The character is 26 overlapping
boxes; at every joint (and under the aiming pose, arms over torso) two or
more boxes with different orientations cover the same pixel. The sub-pixel
AA jitter re-rolls which box a pixel sees each frame, the stored previous
normal belongs to the other box, and the tap is rejected. Limb rotation in
motion adds to it (halving the per-frame motion — "gentle" runs — helps but
does not stop it: 12–16 % short standing/walking either way). The
reprojected *position* is exact (per-box rigid transform); it is the
validator that disowns it.

## Changes

### Depth-only tap validation for dynamic hits (`reproject.wgsl`, `pathtrace.wgsl`)

`gPos.w` now tags the surface class (0 miss / 1 static / 2 dynamic). For a
pixel whose visible surface is dynamic, `validTap` skips the normal test and
uses an absolute depth band, `|Δdepth| ≤ max(0.5 m, 2 %·depth)` — wide enough
for one limb layer in front of another, narrow enough that the floor and
walls behind the silhouette (≥ 0.5–0.6 m of ray depth away for anything
above shin height at this camera pitch) stay rejected, which is the bleed the
normal test was buying. Static pixels are unchanged.

**After, same probes** (geom config; static ring identical to before):

| Sample | Character median (before → after) | < 4 frames (before → after) | reset that frame (before → after) |
|---|---|---|---|
| standing, 30 frames | 28.2 → **29.3** | 16.5 % → **0.8 %** | 6.5 % → **0.0 %** |
| walk f15 (max 15) | 11.0 → **14.1** | 21.0 % → 14.5 % | 10.8 % → 6.1 % |
| walk f30 (max 30) | 24.1 → **29.0** | 15.2 % → 6.1 % | 6.6 % → 2.0 % |
| walk f45 (max 45) | 36.7 → **44.0** | 15.1 % → 2.7 % | 11.0 % → 2.1 % |
| gentle walk f45 | 38.4 → **63.7** | 15.8 % → 8.7 % | 6.6 % → 2.2 % |

After the fix the walking character carries the full elapsed motion count on
torso, head and arms (median = max at every sample); the residual short
pixels are legs plus the disocclusion halo, i.e. real disocclusion. The
history map at walk f45 (background `@` ≥ 62, motion started 45 frames ago):

```
before                      after
@@@Q1D*7@@@   1-15 churn    @@@**III*@@@   36-45 across the whole
@@@Dk97etywR@   through the   @@@IIHIII@@@   figure, single resets
@@TLa67dempHS   whole figure  @@@III*III@@   only at joints
@@Ove78cfiEFM                 @@@IIIII*I@@
```

As shipped (clamp, cap 48, α floor 0.02), where the colour clamp also
shortens history, the character's parts moved from *below* their static
surroundings to at or above them: walking f30 torso median 30.1 → 43.8,
upper arms 27.5 (baseline ~19), forearms 6.8 → 7.6 (the swinging forearms'
radiance genuinely changes each frame, so the clamp keeps them short — that
is the estimator working, not a validation failure). Screenshot pair at
576×360, shipped config, flashlight on, mid-walk: `tracks/B4-temporal/
moving-before.png` / `moving-after.png` — no smear behind the character in
either; the after character is slightly more coherent, but at this dark
location the visible difference is small; the numbers are the evidence.
`history-view-*.png` are the composite debug view 3 (green = history) of the
same moment — in the shipped estimator this whole dim neighbourhood sits at
10–20 frames of history, so the character does not stand out either way.

`__compareToReference` at 384×240 (150 reference frames, 60 test frames,
pinned bench pose), before → after: relRmse **1.2468 → 1.2456**, rmse
0.03530 → 0.03533, relBias −0.200 → −0.202 — unchanged, as expected: the
frozen scene has a motionless character and the tap-validation path only
changes which of the character's own texels it may reuse.

### Particles reproject current-as-previous (`renderer.ts` `updateDynamic`)

Swap-remove eviction reshuffles the particle range every frame, so
`prevDynBoxes[i]` for a particle was usually a different particle. The
particle range (packed last; `main.ts` passes its start index) now copies
current-as-previous, so a particle reprojects onto its own current position:
in flight that fails the depth test and resets (as any 20 ms spark should),
on the floor it reuses its own history — and it never inherits a stranger's
transform. Verified only for runtime health (spawned 40+ sparks/debris under
the harness, evictions running, no errors); the artefact it removes is a
one-frame miscolour I could not isolate in a screenshot at this resolution.

### Docs and invariants

- README "Next steps" item 2 corrected: reprojection is per-box rigid for
  dynamic geometry, not static-only; the remaining gaps are named.
- `reproject.wgsl` header rewritten to describe the actual static + dynamic
  paths; the packing-order invariant (`prevDynBoxes[i]` is the same box only
  while ordering is stable: player group, guards in fixed order, particles
  last) is recorded at `updateDynamic`, at the `main.ts` packing site, and on
  the `prevDynBoxes` binding in `common.wgsl`.

### Light probe follows stance (`main.ts`)

Probe height `1.15` → `0.75` while crouched or dragging a body, so the LIGHT
gauge reflects a crouched chest. Runtime-checked (probe reads 0.146 crouched
vs 0.145 standing at one spot — the paths differ; the values happen to be
close there). Track B3 owns detection: nothing of theirs was edited; if
their branch also touches the `setProbes` line in `frameBody`, take both
(their consumer, my stance height).

### Debug hooks added (`main.ts`, `renderer.ts`)

`__historyStats(rects?)`, `__historyRect(rect)`, `__playerScreenRects()`,
`__benchGuard` (pause the rAF loop during scripted probes), and
`Renderer.readMoments()` (moments targets are now COPY_SRC).
`__compareToReference` gained `dims` and `maxMs` arguments so it can run on
the harness (its 2-minute cap and 1152×720 pin cannot complete under
SwiftShader).

## Findings out of scope

- **Direct history in dim regions is short with the shipped clamp.** The
  static floor around the standing player (a dim room) reads a median of
  ~10–20 frames of history against a possible 48, and it *drops* as the
  character or its beam moves through (19.6 → 10.6 over a 45-frame walk;
  8.6 with the flashlight on). The clamp band `mean ± 3σ` (clampFloor 0)
  collapses on a sparse, dark 1-spp direct signal, so accumulation in the
  demo's dark rooms is effectively 10–20 frames deep, not 48. That is the
  estimator's territory, not this track's; flagging because it is probably
  the largest remaining source of visible flicker in the exact rooms the
  demo lives in.
- **Guards are dynamic geometry too** and read as "the character
  shedding" in any whole-frame or region metric near them; they benefit
  from the same fix (same shader path), but I did not measure them
  separately.
- **`__renderMotion` teleports the player 1.8 m on its first frame** (the
  circle starts at radius, not at the current position) and can walk the
  character through furniture (no collision on a scripted position). Both
  contaminated my early measurements; the probe scenarios in this report
  avoid them by warming up at the circle's start point and ray-casting for
  clearance first. Worth fixing in `renderMotion` if it stays the standard
  motion driver.

## Merge notes

- Files: `src/shaders/reproject.wgsl` (validTap + header),
  `src/shaders/pathtrace.wgsl` (gPos.w tag only — estimator untouched),
  `src/shaders/common.wgsl` (comment), `src/engine/renderer.ts`
  (`updateDynamic` particle range, `readMoments`/`readF16`, momentsHist
  COPY_SRC), `src/main.ts` (particle-start plumbing, probe stance, debug
  hooks, `compareToReference(dims, maxMs)`), `README.md` (Next steps item 2).
- The `Renderer.updateDynamic` signature gained an optional third argument;
  the only caller is `main.ts`.
- No new render targets, buffers, or bindings; `gPos.w` is the only
  wire-format change (2.0 now legal), consumed only by `reproject.wgsl`.

## What the evidence can and cannot show

- The reprojection logic is deterministic integer/float compare on stored
  values; SwiftShader vs Metal changes rounding of the f16 depth in
  `gNormalDepth`, so a tap sitting exactly on the 0.5 m band edge could flip
  — a sub-percent effect, not a class of bug. What this check *would* miss:
  anything that depends on real frame timing (the harness runs at ~0.5 fps
  wall clock with `dt` clamped to 50 ms, so the per-frame motion is 2–3×
  a 60 Hz frame's — a harsher test for the validator, but the beam/clamp
  interplay at true frame rates is not observed), and full-resolution
  behaviour (all numbers are at 384×240; per-pixel logic is
  resolution-independent, part statistics are not).
- Not verified: what the wider dynamic depth band does at the feet/shin
  seam, where the floor is within the 0.5 m band and can lend a foot pixel
  its history. It is spatially local and clamp-bounded; look at the feet
  during the walk cycle on the Mac at 1152×720 to confirm it is invisible.
