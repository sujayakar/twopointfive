# Track B1 — radiosity → sampling hybrid (bake as guide, rays as truth)

Owner branch: `claude/radiosity`, worktree `~/src/twopointfive-wt/radiosity`.
Base: `claude/campaign` trunk AFTER Track A (instrument) merged — you have the
reservoir-buffer merge, work counters, and `__benchResolution` available.

Read first: `README.md`, `docs/campaign/STATUS.md`,
`docs/campaign/tracks/A-instrument.md` (counters — you will use them),
`tools/headless/README.md`, then in depth: `src/shaders/pathtrace.wgsl`
(the `main` bounce loop 632-921, `radiosityIndirect`, the `radioStatic`
gates), `src/shaders/radiosity.wgsl` + `src/scene/radiosity.ts` (patch
layout, bake, inject, solve; `torchVisPoint` single-tap), the RIS/reservoir
machinery in `src/shaders/common.wgsl` (`Candidate`, `proposeCandidate`,
`risTarget`, `sampleIndirectRIS`, `GIReservoir`, `restirGI`), and
`src/shaders/flashmap.wgsl` + `probe.wgsl` (self-group skip pattern via
`occludedSkipping`). Also `docs/restir-literature.md` §1-2.

## Why this track exists

Radiosity made indirect light noise-free and cheap, but it *is* the estimator
on static hits: characters neither carry patches nor occlude the baked
transfer, so a body never shadows bounce light. Sujay's ask: keep sampling,
use the bake to *guide* it. The solve keeps running; the trace pass stops
believing it blindly and starts verifying it with rays.

## Deliverable 0 — the crouch bug (do first, ship as its own commit)

Diagnosis (established by the campaign, verify then fix): crouching turns
the character's aim layer off (`player.ts` `character.update(..., aim=false)`
when crouching), the pistol hangs at the thigh pointing at the floor, and the
flashlight's pinhole depth map (layer 0, traced from the lens centre in
`flashmap.wgsl`, no self-skip) is 78-92% covered by the character's own
pistol slide at ~2.5 cm. Radiosity injects the flashlight ONLY through that
map (`torchVisPoint`, radiosity.wgsl) → zero injected energy → the room's
flashlight bounce disappears when crouched. Real shadow rays (jittered over
the 5 cm lens sphere) still see ~60%, so direct light survives.

Fix, in the codebase's own idiom: give the flashmap trace a self-group skip
exactly like the gameplay probe (`occludedSkipping` with the owning
character's dynamic group — layer 0 = player group 0; guard layers skip
that guard's group; thread the per-layer group as a small uniform table or
derive it from the light index — justify your choice). Also upgrade
`torchVisPoint` to the 4-tap PCF `torchMapSample` already uses so patch
shadows aren't single-texel hard. Do NOT change the game-side crouch pose in
this track. Verify with the 2×2 headless matrix (crouch × radiosity on/off,
debug view "indirect only", `readHDR` mean luminance, small resolution) and
add the third axis (`flashVisVolumetric` toggle) confirming the volumetric
shaft comes back too. Quote the numbers in the report.

## Deliverable 1 — split the flag

`settings.radiosity` currently drives BOTH the solve dispatch and
`U.radiosityOn` (the trace pass's consumption). Introduce
`settings.indirectMode: "traced" | "radiosityRead" | "gather" | "patchRIS"`
(default = today's behavior: `radiosityRead` when `radiosity` is on, else
`traced`), a panel select in the lighting group, persisted with the other
settings (bump the store revision; keep old blobs migrating cleanly). The
solve runs whenever any mode other than `traced` is selected. Reference mode
still forces `traced`. Thread the mode into `Uniforms` (append, never move
fields; recompute `UNIFORM_SIZE` by hand and note the byte in a comment).

## Deliverable 2 — mode "gather" (final gather at x1)

Static primary hits trace bounce 1 for real (character in the beam now
shadows the floor bounce), and the x1 vertex reads `radiosityIndirect(x1)`
re-modulated by x1's albedo as its incoming indirect instead of tracing on.
Fallbacks: dynamic x1 (bounce ray hits a character — no patch), sentinel
-1 faces → the existing NEE/traced path at that vertex. Set the indirect
layer's validity alpha honestly. Decide and document how the checkerboard
indirect rate interacts (gather traces exactly one bounce, so the CPU's
`bounces <= 1 → rate inert` rule needs a stated stance). Keep x1's direct
NEE (`sampleIndirectRIS`) — deleting it is a separate, measured decision you
may propose in the report with numbers, not a default.

## Deliverable 3 — mode "patchRIS" (patches as virtual lights)

At the primary vertex (b == 0, after `restirDirect`), estimate indirect by
resampling PATCHES as emitters through the existing RIS shape (M
candidates → weighted reservoir on a `risTarget`-style proxy → ONE
`occluded()` shadow ray to the winner). Candidate = a patch point jittered
over the cell (tangents + half extents), radiance = the patch's OUTGOING
radiosity B_j/π folded through cos_j·A_j/d² (point-equivalent, clamped:
near-field VPL singularity — clamp d² and cap W as the codebase does).
This mode also serves DYNAMIC primary hits (characters lit by patch
bounce), which no current path does. Data plumbing: the trace pass needs
per-patch geometry and B_j — Track A freed storage-buffer slots, so bind
`radStatic` (patch lanes) and `radDyn` (B ping-pong region, parity-known)
read-only in the trace pass if the count fits (verify against the
requirement in gpu.ts and update it); otherwise mirror the existing
`radGSky` texture packing precedent. Proposal quality: uniform 1/N over
~3400 patches with M=8 will speckle — implement a per-frame CDF/alias over
`luminance(B_j)·A_j` (a small prefix-sum kernel after solve B; N ≤ 4096)
and sample from it. Compose with the GI reservoir if cheap: a patch survivor
IS an x2 sample — but only if the merge weights are honest; if it's not
clean, land patchRIS without GI reuse and say so.

## Verification you must do (evidence in the report)

- Type/build clean after every step.
- Headless (sandbox off for Chromium, reason "chromium needs its own
  namespaces"), all at small resolution via `__benchResolution` /
  `renderer.resize`:
  1. Crouch matrix before/after Deliverable 0 (numbers).
  2. Per mode {traced, radiosityRead, gather, patchRIS} at the pinned pose:
     `__compareToReference` RMSE vs the traced reference (report the table;
     reference forces indirectMode=traced by construction — make sure your
     mode plumbing respects that), work counters (rays, shadow rays, BVH
     visits per pixel — the whole point of this track's cost story), and a
     screenshot Read with a description.
  3. The character-shadows-bounce-light shot: player standing in the beam
     ~1 m from a wall corner, debug view "indirect only", for radiosityRead
     (no body shadow) vs gather (body shadow present) — Read both PNGs and
     describe the shadow. This is the deliverable's headline.
- State honestly what SwiftShader evidence cannot show (timings) and list
  the exact `__bench` runs Sujay should do on the Mac, per mode.

## Rules

- Work only inside your worktree; commit at every green step, at least
  hourly; never `git stash`; never rewrite history.
- Yours: `pathtrace.wgsl` indirect region, `radiosity.wgsl`, `radiosity.ts`,
  `flashmap.wgsl`, the settings/panel entries for indirectMode, renderer
  plumbing for the new bindings and dispatch gating. Not yours: volumetric
  march (Track B2 owns it — do not restructure `volumetricBeams`; the
  crouch shaft symptom is fixed by your flashmap change, that's all),
  guards/detection, reproject/particles.
- Comments: short, why-not-what, no edit narration. A one-paragraph design
  note goes in `docs/campaign/tracks/B1-radiosity-hybrid.md`, not in code.

## Report

`docs/campaign/tracks/B1-radiosity-hybrid.md`: the crouch fix numbers, the
per-mode RMSE + counter tables, the two headline screenshots described, a
recommendation for the DEFAULT mode with the trade-off stated, Findings,
merge notes, and the Mac bench script (exact console commands per mode).
Final message = same content + branch HEAD sha + anything below 100% with
the reason.
