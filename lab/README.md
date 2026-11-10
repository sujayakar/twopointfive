# Lab: a renderer to try ideas on

A small WebGPU renderer, from scratch, with no dependencies: one world, one clock in ticks, and stages with typed inputs and outputs. `rooftop.html` is the built page; it opens from disk in Chrome.

    bun run build.ts                                                   # dist/index.html, dist/artifact.html, rooftop.html
    node lab/tools/card.cjs lab/tools/views.json                       # stills and GPU time a pass
    node lab/tools/card.cjs lab/tools/bench.json --bench realtime,raw  # bias, noise, error against the reference
    node lab/tools/card.cjs lab/tools/poison.json --tour 900           # the fixed run, twice: times, and fingerprints that must agree
    node lab/tools/card.cjs lab/tools/poison.json --poison             # bad numbers in every history: does the picture heal?
    node lab/tools/card.cjs lab/tools/poison.json --watch 600          # no stage makes a bad number; one broken on purpose is named, and heals
    node lab/tools/card.cjs lab/tools/poison.json --guards 3           # what the checks cost

    node lab/tools/card.cjs lab/tools/detail.json --detail             # how much of the reference's detail is there, by size in pixels, and how far off from dark to bright
    node lab/tools/card.cjs lab/tools/table.json --table               # the kept light alone, against paths, room by room
    node lab/tools/card.cjs lab/tools/steady.json --steady             # what shimmers, what is off in motion
    node lab/tools/card.cjs lab/tools/steady.json --settle             # frames until right again after a turn, after doors
    node lab/tools/card.cjs lab/tools/leaves.json --leaves             # leaves against the reference, band by band
    node lab/tools/card.cjs lab/tools/against.json --against       # methods and path tracers against the reference: off in the mean, median, over blocks, times by stage; `maps` draws where
    sh lab/tools/two.sh <commit> <views> --bench realtime          # an older build and this one in one box
    SCENE=street node lab/tools/card.cjs …                             # any of them in another scene: rooftop, street, forest, pass
    bun run tools/stands.ts                                            # does everything stand on something, is nothing in anything else

Set methods side by side in ONE run (`--bench a,b`, or `methods` in a view of detail.json): two boxes are not the same. A view may carry `tune` (src/tune.ts). `notes/` holds what is being asked.

The page: `?bench` plays the fixed run and prints it; `?scale=0.75` fixes the render scale; `?tour` starts the tour.

## How it is put together

- `src/graph.ts` A stage names the signals it reads and writes; its bindings are written from those names, so its WGSL is the algorithm alone. In play only what is read back next frame is checked for bad numbers, by its bits.
- `src/watch.ts` For development (`?watch`): every float every stage wrote is looked at and the first stage to make a bad number is named. Nothing is cleaned meanwhile, and without it nothing is paid.
- `src/stages/realtime.ts` The real-time picture, from swappable parts with fixed signatures:
  - `CachePart` how light on surfaces is kept (`cacheRead`, `cacheFound`, and stages that fill it). `hashCache`, `noCache`.
  - `DenoisePart` signal to quieter signal. `svgf`, `noDenoise`.
  - `UpscalePart` color at render size to picture at output size. `taauUp`, `bilinear`.
- `src/stages/path.wgsl` The reference: the same pieces, materials, lamps and sky; nothing cached, clamped, reused or filtered.
- `src/wgsl/scene.wgsl` How a ray meets one piece (box, up to 8 planes, at most one quadric, curves) and one leaf.
- `src/wgsl/rays.wgsl` The two ray queries, `nearest` and `visible`, by one rule: every stage says how wide its ray is (`foot`), and what is smaller than that is met as part of the world's block. `src/scene/blocks.ts` lays the block's cells, `src/blockmap.ts` finds what is in them once, by rays from six sides; `src/stages/cells.wgsl` keeps their light, `src/wgsl/kept.wgsl` reads it. What it is worth, part by part: notes/onelod.md. What an object's levels must say: notes/levels.md.
- `src/wgsl/cache.wgsl` Light on surfaces: a table keyed by place and facing.
- `src/bench.ts` Measures, the fixed run, the poisoning, and `capture`: the noisy signals, what is known exactly, and the reference of a view, for training.
- `src/scene` The world as data. `people.ts` steps people a tick at a time. Everything drawn is a function of (tick, inputs, seed).

## The frame

raster ids (camera, clear layer, the leaves the sun sees) → first hit → cache → lamps (three fractions: standing lamps, torches, sun) → gather (half size) → mirror (half size) → denoise each → air → resolve → upscale → glow, exposure fusion → canvas.

A new part: copy the one it replaces, keep its signature, register it in `src/shot.ts`, and name it after `--bench`.
