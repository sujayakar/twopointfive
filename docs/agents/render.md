# Renderer

How a tick's rows become a picture on raw WebGPU. Direct light, the gather, the cascades, the denoiser, reflections, shadows of men and the upscale
are in `render-light.md`; the rest is here. See also `STYLE.md` (the look as rules), `weather.md`, `stealth.md` (what the fog hides), `team-feel.md`
(the tube), `destruction.md`, `qa.md`.

## What the player gets

- Every light, shadow and bounce is worked out live from plain boxes. Smoke rolls under a lamp and hides what the rules say.
- Your eyes adjust: a dark room is drawn up from the yard, and the yard glares from inside it. Goggles swap the picture for the tube's.
- What nobody on the team sees is pushed down by the fog. Your own men are rimmed where they are dark.
- The page picks a preset for the machine and holds a frame rate, shrinking the picture before it stutters.

## Where the code is

| file | what it does |
|---|---|
| `src/engine.ts` | `Engine.render`: what a frame sets up, then the encoder's order |
| `src/render/passes.ts` | `LightingPasses.encode`: from direct light to the canvas; owns the screen targets |
| `src/render/frame.ts`, `src/settings.ts` | `FrameParams`, `FrameFlags`, the scene group, `scenePrelude`; `RenderSettings`, every row a pass reads |
| `src/host/pictureopens.ts`, `src/host/engineflags.ts` | the address's words for the picture, and how it opens on any page |
| `rust/game_draw/src/boxes/`, `src/draw/world.ts`, `src/scene/boxes.ts`, `src/scene/cut.ts` | the box world: packed in Rust, handed over by `DrawWorld`; layouts; cut planes |
| `src/render/gbuffer.ts`, `gbuffer.wgsl`, `gbuffer_cut.wgsl`, `gdown.wgsl`, `look.wgsl`, `detail.wgsl`, `markings.wgsl`, `lookTex.ts` | the surfaces |
| `volumetrics.wgsl`, `vol_common.wgsl`, `dust_look.wgsl` | one march: haze beams, smoke, rain |
| `src/smoke/`, `rust/sim/src/bricks.rs`, `src/draw/smoke.ts`, `smokeLook.ts` | the picture's solver, the game's grid, the bend, the look |
| `lookexpo.ts`, `placeField.ts`, `placePlan.ts`, `photometry.ts`, `lookgrade.ts`, `place_common.wgsl`, `post.wgsl` | exposure, grade, tube |
| `src/render/fogofwar.ts`, `rust/game_draw/src/fog.rs`, `src/draw/fog.ts` | the fog's drawing |
| `src/render/outline.ts`, `rust/game_draw/src/rims.rs`, `src/draw/rims.ts`, `src/host/findme.ts` | the rim in the dark |
| `src/render/decals.ts`, `rust/game_draw/src/marks.rs`, `src/draw/marks.ts`; `src/render/particles.ts`, `src/draw/events.ts` | marks on the world; GPU particles |
| `rust/game_draw/src/boxes/glass.rs`, `src/draw/sky.ts`, `src/picture/` | panes; sky and weather onto the engine; the picture's own rain, snow, air, fires |
| `src/ui/quality.ts`, `src/render/texpool.ts`, `src/host/pace.ts` | presets, the ladder, targets kept across a size, the frame's pace |
| `src/gpu/wgslTrim.ts`, `src/gpu/device.ts`, `src/gpu/keep.ts`, `src/ui/settle.ts` | shader text, the compile queue, the kept page, the start's wait |
| `src/gpu/timer.ts`, `readout.ts`, `perfsweep.ts`, `perfrec.ts`, `pricerun.ts`, `bench.ts`, `framecap.ts` | timers and measuring |
| `src/ui/graphicspanel.ts`, `src/ui/lookpanel.ts` | the debugger's graphics rows and debug views; the look's sliders (`/look`) |

## How it works

**A frame, in order.** `Engine.render` sizes the targets (`resize`), jitters the camera, uploads boxes and lights, writes the frame block, then
encodes `fogSight`, `smoke`, `mesh`, `gbuffer` and its internal-size copy (`gdown.wgsl`), the cascades (`rc c3` down to `rc c0`, `rc dice`),
`diceRing`, `place`, `air`, `powder`, `particles`. `passes.encode` goes on: `direct`, `softblur`, `gather`, `gatherLight`, `temporal`, `reflect`,
`rdenoise`, `volumetrics`; then `sharpU` and `compS`, or a plain `composite`; the particles' splat; `gbaa`; `fogofwar`; the outline; `taau`; `bloom`;
`post`; `fxaa`. Each name is also its timer's row.

**Sizes.** The output is the canvas, `maxInternalHeight` rows at most. Internal size is that times `renderScale`. The gather runs at a third or a half
of internal, the march at a half or a quarter, reflections at a half or a third. `TexturePool` keeps a size's targets, and a change resamples the
histories, so a step of the ladder costs little.

**The frame block.** `FrameParams` goes through `writeFrame` to `struct Frame` in `common.wgsl`. `FRAME_HEAD_BYTES` go up each frame, the wall-side
tables behind them only when changed. `FrameFlags` are its bits. Group 0 of nearly every pass is `createSceneBindGroup`: frame, boxes, grid, lights,
smoke, capsules, fog sight, reach, mesh. `levelSwitches` puts what a level raises in the block, not in shader text, so no level asks for other
programs.

**Who sets what.** `Quality.apply` writes a preset's rows into `RenderSettings`, and `Engine.render` mirrors settings into the passes each frame, so
writing a setting is all a panel row does. `pictureWords` reads the address before the engine exists, `pictureOpens` after the first preset.
`lookConfig` and `lookAmounts` are the look's rows; `?look=<row>:<amount>` sets one for a visit.

**The box world.** Rust packs every box (`pack.rs`: 32 bytes of place and flags, 64 of material): statics first, and again only where one changed,
then the frame's own, up to `MAX`. `grid.rs` builds the grid every ray walks. `DrawWorld` hands the rows over as they lie.

**Cut boxes** carry up to `MAX_CUT_PLANES` planes in their own frame. A second pipeline (`gbuffer_cut.wgsl`) discards what lies past a plane, and
every GPU ray meets the cut solid. The trace grid and the smoke's obstacle bake keep the uncut box.

**Surfaces.** Maps (`look.wgsl`, made by the page in `lookTex.ts`, on workers), wear (`detail.wgsl`) and decals are read only in the g-buffer, so a
ray's hit sees a box's flat paint. A box's stuff is a word in its own record, never guessed from its color.

**The march.** `volumetrics.wgsl` is one program on every tier, and there is no froxel grid. A tier differs by numbers (`Caps`, `volSteps`, its size).
It returns the light in-scattered and what is left of what lies behind.

**Smoke has two models.** What eyes see through is the game's: half-meter cells in `rust/sim/src/bricks.rs`, stepped ten times a second, the same
everywhere. What is drawn is `src/smoke/smoke.wgsl`, a GPU solver on sparse 8³ bricks: a coarse lattice (one brick is one game cell) and a fine one
for gun gas, out of one pool (`smokeGridConfig`). `smokeFrom` hands the grid over where a step was taken, and the solver is **bent toward it** a
little every step (`SmokeParams`: `bend`, `pull`). Only the game's own share of a cell is measured; a fire's plume or a breath says so in its `K`
word. After a load or a seek `SmokeSystem.anew` takes the grid whole before the first frame, its color with it (a brick's, two meters square: dust is tan). `SMOKE_LOOK` is how smoke is drawn.

**Eyes.** Buffers hold exposed light. `hourExposure` says what an hour's picture stands for, and every source is drawn at the lesser of its authored
and its real strength under it (`photometry.ts`). `PlaceField` solves one light level a place from the cache, on `PlaceMap`'s cells. `lookExpoFor`
gives each place one gain from its level and the level where Sam stands, bounded by the night's exposure. `agx` in `post.wgsl` tone-maps, with
`lookGrade` turning the contrast. **The tube** is `post.wgsl`'s swap above half of `nvAmount`, which `tubeOf` reads off `View.night`. Infrared lights
are in the list only then (`LightSet.ir`), and the frame the tube comes off restarts every history.

**Fog.** `FogPass` works on a copy of HDR after the composite: a pixel no eye sees in plan is pushed down. `hint` asks a table of bearings before it
walks the grid; `?fogpass=old` only walks. Under the rules' `full` picture `FogSight` runs first, so any pass can ask `fogSeenAt` of any point.

**The rest of the frame.** The rim: `OutlinePass` draws a scissored triangle a man off the g-buffer's id target, as strong as that part of him is
dark. Marks: the list is the game's, and the g-buffer edits albedo, normal and gloss, so the light answers by itself. Particles: `particles.wgsl`
steps and shades them against the trace grid, `particles_draw.wgsl` splats into HDR with no history, and `src/draw/events.ts` plays each emit once.
Glass: a pane is out of the raster and the trace grid, listed in the info block (`MAX_PANES`, nearest the view first); the gather adds what it
mirrors, the composite its tint and bend. Outdoors: `skyRadiance` paints the sky; rain is the march and `src/picture/rain.ts`, snow `snowField.ts` and
`src/picture/snow/`.

**Quality.** `PRESETS` has `high`, `medium`, `low`, `verylow`. At boot `calibrateTick` times the live picture behind the card and `plan` picks a
preset and scale from `COST`. Then `sample` judges a window a second under `ladderBands`; `ladderAim` is 60, the maintainer's pick. Under the aim with
the GPU the limit, the scale is cut to what the cost model says lands on target, down to `SCALE_FLOOR`, then the preset drops (`HOLD`). With room,
`CLIMB` takes a step at a time and undoes it on a bad window. `SHED` drops sharp direct under auto before it takes pixels ("Auto drops it"). `PACED`
leaves alone a page held to a low rate by something else. `?quality=` pins a tier, `?adapt=0` holds the one it stands on. `EVENPACE` draws at the
ladder's rate on a faster display.

**Shader text.** A scene module is `scenePrelude` plus its kernel; `trimWgsl` keeps what an entry point can reach and folds branches its constants
decide. A program is made once a visit: the page is a shell that owns the device (`Keeper`), and each load leases a view that answers an old ask with
the old object. `PIPE_QUEUE` holds compiles back on Windows only, where a big kernel takes seconds and starves the browser's own drawing.
`laterPipelineAsync` compiles behind the boot what no first frame draws, while a stand-in draws. `src/ui/settle.ts` holds the brief until every
program is in, up to a cap.

## Rules that hold

- **General systems for light.** "i worry that we're finding these cases one by one and not fixing them systematically", then "I think we should
  pretty much fix all 9 things you found". The exposure asks no hour and names no opening.
- **Picture only, one way.** No GPU number reaches a rule. "whenever we have something like smoke that's "dressing" here computed on the GPU that
  matches something authoritatively on the CPU, we should have a control system that bends the GPU sim towards the CPU state gradually". Detail finer
  than the grid is free: "i'm actually fine with vision being a bit different than the visuals, can be just the brick level stuff".
- **Smoke stays near High.** "the smoke looks so sick in our game that i'd rather we keep it closer to high but then tradeoff other things in the
  frame budget". Only `verylow` marches at a quarter, and medium and low take "Per-pixel, two lights a step".
- **Four presets.** "I think we can just remove ultra entirely".
- **Past the level's edge is black.** "we should just have black outside the edges of the map, not extend" (`composite.wgsl`).
- **Compile once, and never in play.** "ideally not needing to recompile for the gear selection screen and levels", "i think we shouldn't do truly
  lazy compilation where then have stutters during gameplay".
- **Every pass has a timer** or a row in `passtimer-exempt.ts` that says why not. A pass that rests a frame says `timer.idle`, or its row keeps its
  last time.
- **The scene group is full** on some adapters: a new storage buffer may not fit (`particles.ts` counts them).

## To change it

- **A new pass:** open it in `Engine.render` or `passes.encode` with `timestampWrites: ts('name')`. A target it holds comes from `TexturePool`. Add
  its name to `calibrateTick`'s fixed part if it does not scale with pixels.
- **A word for a shader:** a field in `FrameParams`, `writeFrame`, `struct Frame`; then `bun run tools/qa/frame-layout.ts`.
- **A setting:** `RenderSettings`, `defaultSettings`; if tiers differ, a row in `PRESETS` written in `Quality.apply` and priced in `presetCost`; a row
  in `graphicspanel.ts`; a row in `SWEEP_ROWS` to price it.
- **A word on the address:** `pictureWords` if it moves shader text or buffers, `urlFlagsOnEngine` for a setting.
- **Something new drawn:** `overview.md` has the section's way. A new box field goes in `pack.rs` and the layouts in `src/scene/boxes.ts`.
- **A kernel's text:** `bun run tools/qa/checks.ts shaders`, and `node tools/qa/wgsl-compile.cjs` asks a browser's compiler. A grown kernel costs
  Windows compile time (`inline` counts it) and nears the software adapter's limit (`kernel-stack.cjs`).
  A big body is asked in a loop. Credit: Stephen Jayakar, from his pull request 1.

**To measure.** F9 records frames (`tools/qa/perf-report.ts`). F8 flips each of `SWEEP_ROWS` over a held world and `tools/qa/perf-sweep.ts` says what
it gave back: only that says what a feature costs. F6 keeps every target of two whole frames with the renderer's trace (`tools/qa/capture-read.ts`);
Shift+F6 a window. `?pricelevels` on `/look` prices every level, `?bench` the benchmark scenes, `?hud` shows the passes.

**Timing traps.** Passes overlap on the GPU, so their sum overstates a frame: read `unionMs`. A render pass's stamp spans the wait for the display, so
totals leave out `RENDER_PASSES`; `gbuffer` alone follows its own work. Another tab on the same GPU spoils a take.

**To judge a picture.** A whole frame scaled down hides what matters. Crop shadow edges, light-pool edges and emitters, enlarge, compare, and use the
debug views. For a clean A against B, give each setting a fresh `?paused` page.

**With no GPU.** `?paused` and `__draw.run` step a page on the software adapter. A still there shows that rows reach the picture and that a pass ran.
It says nothing of time or of how a thing looks. Say "never seen on a GPU" of any look.

## What holds it

Checks (`bun run tools/qa/checks.ts <name>`): `frame-layout.ts` (the block against the struct), `shaders-check.ts` (parse, `trim`, `fold`, `inline`),
`quality-check.ts` (what the ladder decides), `passtimer-check.ts`, `framecap-check.ts` (measuring), `keeppage-check.ts`, `pipequeue-check.ts`,
`settle-check.ts`, `pace-check.ts` (compiling, pace), `lookexpo-check.ts`, `smokelook-check.ts`, `materials-probes.ts`, `zfight-check.ts`.

Page scripts (`bun run tools/qa/pages.ts <name>`): `wgsl-compile.cjs`, `keeppage-page.cjs`, `weather-page.cjs`, `smokejump-page.cjs` (after a jump the
drawn smoke is the game's); the tools `framecap-page.cjs`, `perf-sweep-page.cjs`, `shader-count-page.cjs`.

Native tests in `rust/game_start/tests/`: `draw.rs` (drawing changes nothing of the game; frames stand between ticks), `box_tables.rs`, `fog.rs`,
`marks.rs`, `smoke_said.rs`, `weather_rows.rs`, `tube.rs`.

## Not seen

No check sees a GPU or judges a picture. What a frame costs is known only from takes on real GPUs. The shorter smoke step, and a
plume's shape across presets, were never timed or compared. A rim follows this frame's light with no easing, so a flash can make it jump.
