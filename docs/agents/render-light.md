# Light

How a pixel is lit: direct light and its reservoirs, the final gather, the radiance cascades, the denoiser, reflections, men in rays, and the upscale
that folds it all into the canvas. The frame's order, the box world, smoke, exposure, presets and measuring are `render.md`. The game's own light
meter is Rust (`rust/sim/src/glow.rs`, `stealth.md`) and reads none of this.

## What the player gets

- A room's lamps cast soft shadows with hard contact, and a flashlight's cone sweeps cleanly across a wall.
- A flash or a switched-off lamp changes the picture on that very frame, with no trail.
- Light bounces: a lit wall colors the floor beside it, and light keeps to its room when the doors are shut.
- Polished floors, wet ground, metal and glass reflect what is really there, men and lasers too.
- The picture stays sharp and calm in motion even when it is lit at a lower size than it is shown.

## Where the code is

| file | what it does |
|---|---|
| `src/render/lights.ts`, `rust/game_draw/src/lights.rs`, `src/draw/lights.ts` | `LightSet`, `RtLight`: the list on the GPU, `MAX_LIGHTS`; Rust's order decides who has room |
| `src/render/reach.ts`, `src/render/reachKeep.ts`, `src/draw/reach.ts` | which 1 m cells a light can reach past the walls, kept by the picture |
| `restir.wgsl`, `direct_common.wgsl` | the direct estimator |
| `src/render/denoise.ts`, `denoise.wgsl`, `softtile.wgsl` | one denoiser core, its clients, the tile pass-through |
| `src/render/gatherSplit.ts`, `fgather.wgsl`, `gather_split.wgsl`, `gather_common.wgsl` | the gather's `trace` and `light`, and `directAtPoint`, shared with reflections |
| `cards.wgsl` | a carried light's footprint as an emitter of its own |
| `src/render/rc.ts`, `rc.wgsl`, `lighting_common.wgsl` | the radiance cascades and the dice |
| `src/render/diceRing.ts`, `dice_ring.wgsl` | the steady bounce by day |
| `src/render/wallSides.ts`, `src/scene/openings.ts` | which side of a wall a read is on; each opening as a light |
| `reflect.wgsl` | traced reflections and the highlights of the one BRDF |
| `composite.wgsl` | albedo times the light, plus reflections and glass; `composite` or its compute twin `compS` |
| `sharp_direct.wgsl`, `taau.wgsl`, `gbaa.wgsl`, `fxaa.wgsl` | sharp direct, the resolve, edge AA, the final sharpen and grain |

## How it works

**Two classes, two numbers.** Each light is sharp or broad (`broad`, a panel, a street lamp). For each class a pixel has `U`, the exact unshadowed
light (`evalLight`, every light the tile can see), and `S`, the estimate through shadow rays. `U` is never filtered and holds no history, so a lamp
switching off is exact and immediate. Only the ratio `S / U` is noisy, and only the ratio is accumulated and blurred.

**The direct estimator** (`restir.wgsl`, one program; `direct` on the timer) keeps one reservoir a pixel a class, and spends a fixed number of rays
(`restirRays`). A frame takes:

- a **fresh** candidate drawn by its unshadowed weight, with a shadow ray;
- the **temporal** reservoir at the reprojected pixel, re-traced from here, its confidence cut by how far its target moved and by a per-tile gradient
  (a lamp whose record is last frame's word for word gives what it gave and is not worked out twice: `SLOT_SAME`);
- **spatial** taps of neighbors' last reservoirs (`restirSpatial`; `restirSpatialAll` runs them everywhere);
- **beliefs** per 8×8 tile: a light every ray of the tile found dark leaves the fresh draw and is audited now and then, because a lamp behind the wall
  otherwise turns the floor into coin flips (`restirKnownDark`, `restirAudit`);
- a **transient** stratum: a light that lives only a few frames (`ttl`, `transient`) is never in a reservoir and gets its own ray, so it arrives and
  leaves at once;
- a **held** stratum: a light a man carries (it has an owner) bypasses the reservoirs and takes `heldRays` stratified rays;
- **motion** bits, up only on frames the camera moved (`camMovedAny`, `restirMotionLive`): stochastic reprojection and cut confidence.

Reach masks run ahead of it. A light skips a cell it cannot reach through full-height walls, and `ReachKeep` takes one step of deciding masks again
each drawn frame. A camera cut drops the beliefs (`dropRestirBeliefs`), and the reservoirs validate themselves by depth.

**The denoiser** (`Denoiser`) is one core with clients. `encodeShadow` runs on the ratio at internal size: temporal (motion vectors, depth checks,
moments, a clamp to this frame's neighborhood), then à-trous iterations at `DN_STRIDES`. The last iteration multiplies `U` back in. The blocker
distance from the direct pass sets how far each pixel may blur, which is what hardens contact. `softtile.wgsl` lets a tile far from any penumbra pass
straight through. `encodeRadiance` is the same core for the gather's irradiance (with a fast history its clamp centers on), the gather's specular, and
per-pixel on reflections. Iterations and caps are the preset's (`Dn`).

**The gather** (a third of internal on every preset; the climb can raise it to a half) shoots four cosine rays a texel and is two programs. `trace`
finds the hits and writes every point that wants light to a buffer; `light` lights them in one loop around one `directAtPoint`. One call site keeps
the text small on a compiler that pastes every call into its caller (`SPLIT_TEXEL_BYTES` is the buffer's size, `splitRefused` says when a GPU refuses
it). A hit's light is on-screen direct where the point is visible, one shadow ray otherwise, plus the cache's bounce and emission; a miss takes the
sky. `giFireflyK` bounds each ray. On High, `torchBounceCards` rebuilds each carried spot's footprint as an emitter (`cards.wgsl`), which takes the
boil out of a flashlight near a wall. `temporal` then denoises it.

**The cascades** (`RadianceCascades`) are a radiance cache in world space, not something the eye sees: one probe a meter at c0, four levels by default
(`RcConfig`), upper levels merged coarse to fine, each with four times the reach and twice the directions a side. c0 is baked into the **dice**, a 3D
irradiance texture that the gather's hits, smoke, particles and the place field read. One grid at every preset (`rcGridOf`). `rcPeriods` lets far
levels refresh over frames on the cheaper tiers; c0 and the dice bake every frame. A change of the level or of the grid re-primes (`invalidateWorld`):
extra whole bakes in one frame, so the bounce snaps. By night the dice is a running mean. By day `DiceRing` takes the mean of a few fixed direction
shifts, since small bright openings otherwise flicker in and out of one frame's directions. `setOverlap` and `FrameFlags.RcRoom` stop daylight
reaching a shut room's probes, and `wallSides` keeps every read on its own side of a wall.

**Reflections** (`reflect` then `rdenoise`, High and Medium: `RtTier`) trace GGX rays at a half or a third of internal for surfaces under
`rtMaxRough`, shading hits with `directAtPoint` and the cache, plus analytic highlights of lamps no ray can find. The specular client filters them by
hit distance. `FrameFlags.Gloss` stands in on the tiers with no reflection pass: the gather traces one mirror ray on polished floors. Glass panes get
their mirror in the gather. Men appear as their round cones. A metal's rays read the cache walled off (`reflWallsConfig`), so it never mirrors the
room behind a wall.

**The composite** multiplies albedo by direct and upsampled indirect, adds reflections, emission, the march, fires and glass, and applies exposure
gain by place.

**The upscale.** With `taau` on the camera is jittered by a Halton sequence over `taauPhases` frames. `taau.wgsl` reprojects last frame's output by
the closest depth of nine, clips history in YCoCg, and blends, raising the new frame's weight where history clipped, moved fast, or where particles
and rain drew with no history. Debug views bypass it.

**Sharp direct** (`sharpDirect`, every preset) separates what must be sharp from what need not be. The g-buffer is drawn at output size and
`gdown.wgsl` takes an internal-size subsample of it; the direct estimator, gather and march run at internal size. Then `sharpU` scores `U` per output
pixel and multiplies by the upsampled ratio, the composite runs at output size as `compS`, and the resolve is one to one. A pixel with no measured
visibility of its own (a thin reveal, a chair leg) traces its own ray (`sharpSliver`). `sharpDecouple` keeps the output raster on the unjittered
camera, so edges stand still for `gbaa`, which blends across box edges from the g-buffer's edge offsets. It is live only below scale 1, and `SHED`
drops it first. Over every preset `pictureOpens` lays `MOTION_CALM`: spatial taps everywhere, an extra penumbra ray, and a calmer resolve in motion.

**Men in rays.** Each man is 15 round cones fitted to his mesh (`RigFit`): every ray's test at every tier. On High `MeshShadows` refits a triangle
tree for the first `MESH_MAX_CHARS` men, and shadow rays from near receivers walk it (`meshShadowsNear`). The tree is of what he wears (`wornTree`: the
whole mesh's, less the parts he has not on, a ray meeting the same triangles in the same order; held by `cast-probes.ts` castTree), and it is made again
only in a frame a bone of his moved, he changed his clothes or his slot is another man's (castSkip). Timed on an A10 alone: the pass 0.77 to 0.09 ms
and the frame 0.5 to 1.5 ms less; the picture is bit for bit what it was on the software adapter. `charSelfShadow` makes the direct light's rays test his own body too.

**Openings.** `planOpeningsSteps` finds where a level is open to the sky, and each opening is a light of its own, a portal (`lookOpeningLights`): a
hole blown in a wall lets daylight in, and a man by it throws a shadow. Openings, wall sides and reach masks run in steps under one allowance a frame
(`PLAN_PACE`, `Steps`), because all of them come due on the frame a wall is blown.

**Cuts and restarts.** A jump over 2.5 m or a turn over 30° between renders is a cut: `resetTaa` makes the next resolve take its own frame and drops
the beliefs. `restartHistories` clears every screen history for when they are wrong wholesale, as when the tube comes off or a level restarts
(`restartPicture`).

## Rules that hold

- **What is exact stays exact.** `U` is never in a history or a filter, so a transient is a this-frame event at every stage. A light with no history
  behind it enters at full weight and leaves nothing.
- **Carried lights are not reservoir samples.** "the flashlight direct lighting seems like a huge regression now. very noisy and lots of rims": a
  light that moves every frame has no history to reuse, so it takes its own rays.
- **The history pairs with depth.** Every screen history validates each tap by depth; nothing averages depths or normals, and `gdown.wgsl` copies one
  texel, never a mean.
- **A slot is a light by identity.** Reservoirs follow a light through slots by last frame's map (`writeRestir`), because the list's order changes.
  Keep the list's key steady for as long as a light burns.
- **Reads keep to their room.** Bounce and metal reflections stop at walls (`bounceWallsConfig`, `reflWallsConfig`, `wallSidesConfig`); `RcRoom` holds
  the cache to it.
- **Secondary hits are diffuse.** Reflection and gather hits shade with no specular lobe and no surface maps: one more narrow lobe would be noise the
  cache cannot settle.
- **One writing.** `directAtPoint` is one text for the gather and reflections. `tools/qa/splitrays-check.ts` holds who reaches the light loop.
- **The lighting trades' twin** (`restirTradePipe`, for `heldAdaptive`, `restirStableRetrace`, `restirDimClass`) is built only if one is turned on. No
  preset turns them on.

## To change it

- **A number of a tier:** `RestirTier`, `Dn`, `RtTier`, `IqTier` or `Taau` in `PRESETS`. Its cost is `presetCost` and `REF`; `quality-check.ts` holds
  the picks.
- **An engine-wide knob:** `defaultSettings`, mirrored in `Engine.render` (`P.restir*`, `K.*` for the denoiser's `DenoiseKnobs`). A row in
  `graphicspanel.ts` and `SWEEP_ROWS` to price it on a GPU.
- **Direct's inputs:** the textures are the same four for `softblur` and `compS`, so a new estimator term goes through `S` and `U`. A class added
  means `denoise.wgsl`, `sharp_direct.wgsl` and `composite.wgsl` too.
- **A new history:** a pooled target with a resample rule in `passes.ts` `resize`, cleared in `restartHistories`.
- **A term at a hit:** `gather_common.wgsl` or `lighting_common.wgsl`, thinking about both the gather and reflections. A new point a gather texel
  lights changes `gather_split.wgsl`'s layout and `SPLIT_TEXEL_BYTES`.
- **Kernels grow Windows' compile time and the software adapter's stack:** run `bun run tools/qa/checks.ts splitrays`, `shaders-check.ts` with
  `inline`, and `kernel-stack.cjs`.
- **To look:** the debug views in `graphicspanel.ts` show direct, indirect, the dice, blur hints, specular, reservoir history, rays and reflections
  alone. `__sharp` has sharp direct's census. `tools/qa/capture-read.ts` shows each target before and after its denoiser.
- **To switch off for a visit:** `?notaau`, `?nosharp`, `?noiq`, `?nortreflect`, `?noleakfix`, `?nomv`, `?nomotioncalm`, `?nobouncewalls`,
  `?noreflwalls`. `?gbaa` is edge AA with no jitter.

## What holds it

Checks (`bun run tools/qa/checks.ts <name>`): `splitrays-check.ts` (the gather's two programs and buffer), `steadybounce-check.ts` (the dice ring),
`bouncewalls-check.ts` (reads stay on their side), `portal-check.ts` (daylight counted once), `huesplit-check.ts`, `highlights-check.ts`,
`reflup-check.ts`, `sunglint-check.ts` (reflections and highlights), `lightsame-check.ts` (a lamp as it was), `perf-probes.ts` (the bounce card's geometry and energy), `sdfrig-probes.ts` (the
round cones), `sweep-ir-check.ts`.

Page script: `wgsl-compile.cjs` (every text compiles); `kernel-stack.cjs` as a tool. There is no native test here: Rust neither draws nor reads this
light. `stand_light.rs` holds the game's own.

## Not seen

Noise, ghosting and light leaks are judged on the maintainer's GPU, not by any check. The software adapter can draw these passes, slowly, but its
stills say only that they ran. A light glimpsed from a sliver of a tile that no reservoir there holds comes late, until an audit draws it: a known
price of the beliefs.
