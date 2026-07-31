# Track B2b — GPU fluid simulation + smoke sources

Owner branch: `claude/fluid`, worktree `~/src/twopointfive-wt/fluid`.
Base: `claude/campaign` trunk AFTER Track B2a (volumetrics) merged — you
consume its density-texture contract (`smokeVolume` texture_3d, dims/origin
uniforms, `sampleSmokeDensityCPU` readback stub). Read
`docs/campaign/tracks/B2a-volumetrics.md` first and code to that contract;
if the contract is inadequate, adjust it in ONE place and record the change
in your report — do not fork it.

Read also: `README.md`, `docs/campaign/STATUS.md`, `tools/headless/README.md`,
`src/game/smoke.ts`, `src/game/particles.ts`, `src/game/flashes.ts`,
`src/game/equipment.ts` (slots — you add a 4th), `parked/physics.ts` +
`parked/README.md` (a verified rigid-body module shelved for renderer
reasons that don't apply to one small canister — reuse it, don't rewrite it),
`src/scene/level.ts` (bounds, walls, crates), and the new-pass plumbing
recipe (flashmap/radiosity precedents in `renderer.ts`; the campaign memo
lists every site: shader assembly in `shaders.ts`, layout, pipelines
inside the error scope, resource creation outside `Targets` for world-space
grids, dispatch position after flashmap and before pathtrace, uniform
append rules).

## Why this track exists

Smoke that hangs in the air, curls, pools along the floor, gets pushed by
nothing but its own heat, glows under whatever light is nearby, and hides
you from a torch beam — that is the shot that sells the tech demo. B2a made
the air renderable and coloured; you fill it with a real simulation and give
the player and the world ways to make smoke.

## Deliverables

1. **Solver.** Stable fluids on the shared grid (208×144×13 @ 0.25 m over
   the slab, or the dims B2a fixed): semi-Lagrangian advection of velocity
   and of scalars (smoke density + temperature), buoyancy (temperature and
   density), vorticity confinement (this is what makes it read as smoke —
   don't skip it), pressure projection by Jacobi (iteration count on a
   slider; you decide the default from the visual result and state the
   divergence residual you measured), density dissipation and temperature
   cooling. Solid obstacles: a one-time voxelization of static geometry
   (walls, columns, crates, cubicle panels ≥ cell height) into an
   occupancy field baked at init (reuse the scene bind group: `boxes`/`bvh`
   are available to compute passes) — smoke flows AROUND columns and
   pools inside the conference room instead of passing through walls.
   Boundary: closed room, ceiling at y = 3.2. Storage: ping-pong 3D
   textures or storage buffers as fits the 4-storage-texture-per-stage
   default (SwiftShader must still init — the headless harness is your
   only run loop; storage buffers dodge the format tier questions).
   Passes go through the existing `compute()` helper so they show up in
   the GPU profiler by name.
2. **Sources & sinks (game side).** Smoke enters the sim, not the puff
   list — replace `smoke.ts`'s puff role with a source list the injection
   kernel splats each frame: muzzle bursts (short, hot, directional along
   the barrel), bullet impacts, shot-out light fixtures smoldering for ~20 s
   (a slow thin plume from the fixture — new event hook in
   `equipment.shootOut`), a couple of always-on ambient wisps (a coffee cup?
   a server rack exhaust — pick a spot from the level and justify), and a
   **smoke canister**: 4th equipment slot ("SMOKE GN", key 4, 2 charges,
   HUD count), thrown on the aim arc using `parked/physics.ts` moved into
   `src/game/` (compile it, keep its selfTest reachable via a debug hook),
   bounces, settles, then emits a strong sustained source for ~8 s. The
   flying canister is a dynamic box like a particle. Player throw pose: the
   arm-tuck/aim states already exist — reuse, don't animate from scratch.
3. **Density interface fill.** Each frame the sim's density field IS the
   `smokeVolume` texture B2a samples (write it in the solver's last pass,
   or copy). Keep `density_static` (ambient fog noise) as B2a left it.
   Remove the puff uniforms/loop from `mediumDensity` only if the sim
   fully replaces them — if you keep both, say why.
4. **Gameplay readback.** Implement B2a's `sampleSmokeDensityCPU` for real:
   async 1/4-res readback of the density field a few frames behind (probe
   readback pattern), so Track B3's guard LOS can integrate smoke. Also:
   the player's own visibility should drop inside smoke — attenuate the
   probe (light gauge) by the density integral above the player, or better,
   let the tracer do it if B2a's extinction already dims the probe rays'
   lights (check; if not, do the CPU-side attenuation and note it).

## Verification you must do (evidence in the report)

- Typecheck + build clean; SwiftShader init still succeeds (binding counts
  vs gpu.ts requirement — count, don't guess).
- Headless (sandbox off for Chromium, reason "chromium needs its own
  namespaces"), small resolution, sim dims optionally halved via a debug
  scale for iteration speed (state which resolution each shot used):
  1. Canister in the corridor under the moon pools: sequence of stills at
     t = 0.5 / 2 / 5 / 10 s (`__renderStill` with your throw scripted via a
     debug hook) — Read each: cloud grows, curls, spreads along the floor,
     pools against the corridor wall/columns, dissipates. Say what you see.
  2. Flashlight into the canister cloud: the beam should visibly stop
     inside it (extinction + in-scatter) — Read and describe.
  3. Column obstacle test: source upwind of a column, still after ~4 s —
     the plume splits around it. Read and describe.
  4. Divergence residual after projection (readback a number), mass drift
     of the density field over 300 sim steps with no sources (readback),
     and per-pass GPU profiler names present. Determinism: two runs with the
     same seed produce the same density checksum (readback + hash).
  5. `physics` selfTest passes headlessly (26 assertions per its README —
     quote the actual pass line).
- The Mac bench list: sim on/off, grid full vs half, Jacobi iteration
  counts — the ms question is entirely Sujay's Mac's to answer; make the
  runs one-liners.

## Rules

- Work only inside your worktree; commit at every green step, at least
  hourly; never `git stash`; never rewrite history.
- Yours: the fluid pass(es), `smoke.ts` → sources, the canister item and
  physics revival, injection into the density contract, the density CPU
  readback. Not yours: the volumetric march/composite/reproject (B2a's —
  consume the contract, don't restructure it), radiosity, guards' state
  machine (B3 — you provide `sampleSmokeDensityCPU`, they integrate it).
- Comments: short, why-not-what, no edit narration. Design rationale
  goes in the track report.

## Report

`docs/campaign/tracks/B2b-fluid.md`: solver design in ~15 lines (grid,
passes, iteration counts, why), the source list with tuned numbers, the
five verification bundles with numbers/descriptions, contract deltas (if
any), the Mac bench script, Findings, merge notes. Final message = same +
branch HEAD sha + anything below 100% with the reason.
