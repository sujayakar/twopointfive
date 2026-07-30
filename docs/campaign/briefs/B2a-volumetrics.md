# Track B2a — volumetric renderer: every light, in the room, in colour

Owner branch: `claude/volumetrics`, worktree `~/src/twopointfive-wt/volumetrics`.
Base: `claude/campaign` trunk AFTER Track A (instrument) merged.

Read first: `README.md`, `docs/campaign/STATUS.md`,
`docs/campaign/tracks/A-instrument.md`, `tools/headless/README.md`. Then
in depth: the volumetric section of `src/shaders/pathtrace.wgsl`
(`mediumDensity`, `phaseHG`, `volumetricBeams`, where `vol.steady` /
`vol.flash` land), `src/shaders/composite.wgsl` (the `illum.a *
flashColor * volStrength` reconstruction — the reason all haze is torch
tinted), `src/shaders/reproject.wgsl` (how the volumetric alpha is
reprojected as if painted on the surface behind it, unclamped 48-frame
history), `src/game/smoke.ts`, `src/scene/level.ts` (extents: x ±26, z ±18,
ceiling underside y = 3.2, invisible-to-camera ceiling slab), the light
list build in `renderer.ts` (`updateLights`, static vs dynamic vs transient
ranges, `MAX_DYN_LIGHTS`), and how a new compute pass is plumbed (flashmap
is the smallest complete example; radiosity shows a one-time bake).

## Why this track exists

The medium only glows where a torch cone or a 50 ms muzzle flash passes
through it; the moon and all ~29 practicals never scatter; the camera march
spends ~80% of its 12 steps in dead air ABOVE the invisible ceiling (camera
sits ~20 m up, march capped at 26 m, the room is 3.2 m tall); the shaft is
one scalar in the direct signal's alpha with one shared tint; smoke has no
extinction so it can never occlude. So players never see the puffs. This
track makes the room's air a first-class, coloured, physically-lit thing
that a fluid sim (Track B2b) will then fill with density. You define the
density interface B2b plugs into.

## Deliverables

1. **March the slab, not the sky.** Clip the camera march to the medium's
   AABB (y ∈ [0, 3.2] over the level bounds) instead of `min(depth, 26)`.
   Same step count now samples the room 5× denser — free quality. Steps stay
   a slider.
2. **A real volumetric radiance channel.** Add a 4th layer to the trace
   pass's radiance array (`RAW_ILLUM_LAYERS` 3 → 4): RGB in-scattered
   radiance + transmittance in alpha, written by the march. Give it its own
   reproject/atrous parameter slot (the chains are already parameterised by
   dynamic-offset slots) tuned for volumes: shorter history (~8-16), clamp
   ON, wider a-trous. Composite: `color = surface * T + inscatter`. Remove
   the alpha-of-direct hack, `volStrength`, and the shared-tint
   reconstruction; per-light colour now flows through naturally. Keep
   `settings.volumetric` as the density scale but retune its default so haze
   is present-but-tasteful with fog on (state your chosen number and why).
3. **Every light scatters.** Static lights via a **baked static light
   volume**: a `texture_3d` over the slab (cell ≈ 0.5 m; you pick, justify)
   holding RGB in-scattered radiance for unit density, baked once at init
   like radiosity (per voxel: sum over static lights of intensity × falloff ×
   spot cone × a few visibility rays — moonlight through the window holes
   gives god-ray pools for free). Sample it trilinearly in the march
   (isotropic phase for the ambient sum is acceptable — say so). Provide
   `rebakeLightVolume()` and call it when static light intensities change
   (OCP darkening, shot-out fixtures — see `equipment.ts` /
   `setStaticLightIntensity`; a whole-volume rebake dispatch is fine at
   that event rate). Dynamic lights stay in-march: player flashlight and
   guard torches via their depth maps (HG phase, existing), transients via
   real shadow rays (existing) — all now writing RGB into the new channel.
4. **Extinction.** Density both scatters and absorbs: accumulate
   transmittance along the camera march and attenuate surface radiance;
   attenuate a light's in-scatter contribution by a cheap density integral
   toward that light only for the flashlight/torches (2-3 taps along the
   beam using the density function) — enough that dense smoke visibly
   dims a beam passing through it. State what you did NOT attenuate
   (static volume bake ignores dynamic density — acceptable, note it).
5. **The density interface (contract with Track B2b).** `mediumDensity(p)`
   samples `density_static(p)` (today's fog noise + puffs, kept as the
   default source) PLUS a `texture_3d<f32>` "smokeVolume" bound in the
   trace pass (sampled, filterable, `rgba16float`, R = density; allocate it
   at the fluid grid resolution you agree in a shared header comment —
   propose 208×144×13 @ 0.25 m over the slab — and fill it with ZERO plus a
   tiny CPU-writable test blob you can toggle via a debug hook
   `__smokeTest(x, z, r, d)` so this track is testable standalone). B2b
   replaces the filler with the simulation; nothing else in your code
   should need to change. Document the contract at the top of the WGSL:
   texture format, dims/origin/cell uniforms (append to `Uniforms`, never
   move fields, recompute UNIFORM_SIZE by hand), sampling convention.
6. **Guard-vision hook (data only).** Expose a CPU-side coarse density
   query for gameplay: an async low-res readback of the smoke volume
   (e.g. 1/4 res, few frames of lag, same pattern as the probe readback)
   with `sampleSmokeDensityCPU(x, y, z)` — Track B3 (detection) will
   integrate it along LOS. If B2b lands the sim, this reads real smoke.

## Verification you must do (evidence in the report)

- Typecheck + build clean after every step. Requirement update in `gpu.ts`
  if any binding count changes (count, don't guess) — you MUST still init
  on SwiftShader (4 storage textures per stage max in the trace pass; your
  4th layer costs no extra slot, keep it that way).
- Headless (sandbox off for Chromium, reason "chromium needs its own
  namespaces"), small resolution:
  1. Moon god-rays: pinned pose looking at the north window pools, fog on —
     screenshot Read; describe the pools of light AND visible shafts in the
     air. Then `settings.fogAmount = 0` control shot (shafts vanish, pools
     stay).
  2. `__smokeTest` blob placed under a fluorescent AND in a moonlit pool AND
     in a dark corner: three shots Read — the blob must read as lit smoke
     in the first two (coloured differently) and as a dark occluding puff
     in the third against a lit background behind it.
  3. Flashlight through the test blob: beam visibly dimmer past the blob
     (extinction working) — before/after Read.
  4. Work counters delta: volumetric steps/px and shadow rays/px per
     configuration vs the base commit; `__compareToReference` at small res
     for a fog-on config (the reference path must render the same
     volumetric model — check what reference mode disables and make it
     honest for the new channel; if reference cannot match by
     construction, say precisely why and gate the config out of the
     compare).
- Explicit list of the `__bench` runs Sujay must do on the Mac (fog on/off,
  step counts) since ms are the real unknown here.

## Rules

- Work only inside your worktree; commit at every green step, at least
  hourly; never `git stash`; never rewrite history.
- Yours: the volumetric march, composite/reproject/atrous plumbing for the
  new channel, the light-volume bake pass, uniform/settings/panel for
  volumetrics, `smoke.ts` only as far as feeding `density_static` (do not
  build the fluid — B2b does). Not yours: the radiance estimator outside the
  volumetric term, radiosity, guards, reproject's geometry logic.
- Comments: short, why-not-what, no edit narration.

## Report

`docs/campaign/tracks/B2a-volumetrics.md`: what changed and why, the
density-interface contract restated (B2b codes to it), the four
verification bundles with numbers and screenshot descriptions, the Mac
bench script, Findings, merge notes. Final message = same + branch HEAD sha
+ anything below 100% with the reason.
