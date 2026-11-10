# Light Primer: a renderer to learn from

A small WebGPU renderer, built a step at a time, in which every layer and every buffer can be seen, tuned, switched off, broken on purpose, or replaced by the truth. It is meant to be read: nothing is packed, nothing is clever, and nothing is made faster except in a step whose subject is speed.

    bun run build.ts        # dist/index.html, which opens from disk in Chrome

## How it is put together

- `src/gpu/layout.ts` A struct is declared once. From it come the WGSL struct, its bytes, the menu of views and the inspector.
- `src/gpu/graph.ts` A stage names what it reads and writes; its bindings are written from the names, so its WGSL is the algorithm alone. Only what the panels depend on runs.
- `src/page/knobs.ts` Every number that can be turned, declared next to the stage that reads it. In shaders it is `knobs.<name>`.
- `src/wgsl/` What stages share: rays, lamps, materials, dice. `src/stages/` One file a stage, with its WGSL beside it.
- `src/rooms/` One small room for each question.
- `src/page/chapters.ts` The steps.

Names: P is the pixel, x the first hit, y the second, z the third. They mean the same in the code and on the page.

## Checks

    node tools/look.cjs out.png '{"chapter":1,"frames":64}'     # the built page on the software adapter, as a picture
    node tools/agree.cjs '{"room":"shapes","answer":"wide_raw.value","truth":"wide_true.value"}'   # a buffer's mean beside the truth's
    node tools/check.cjs                                          # every shader, in every variant: text and bindings
    node tools/stages.cjs 8                                       # how long each stage's shader takes to make
    bun run tools/meshes.ts                                       # levels of meshes: sizes, errors, cracks

## Beside the truth

What was measured, on the software adapter, as the mean's distance from the path tracer's:

| | off |
|---|---|
| a white ball in the furnace | 0.01 % (metal 0.02) |
| every way of picking a lamp, with reuse in time and space | 0 to 0.4 % |
| reuse in space while forgetting hidden lamps | 6 % dark |
| highlights by aiming and lobe, weighed | 0.4 % |
| bounced light: shapes, color box, door | 0.6 % dark, 1.4 % bright, 3 % bright |
| mirrors: shapes; the strip floor, whose metals count as matte in bounces | 0.5 %; 10 % dark |
| levels room, with wide rays widening by 0, 0.3, 1 m a meter | 3 % dark, 3 % dark, 0.3 % bright |
| reuse at y in the door room: in time; in time and space | 5 % bright; 3 % dark |
| smoke, scattered once | 0.4 % |
| thin smoke, with bounces; thick | 9 % dark; about 28 % dark before bounces stopped losing what is scattered out of them |
| lantern: light straight from lamps; bounced | 1 % dark; 6 % dark |

The software adapter compiles big shaders slowly (the truth: half a minute). It says nothing about speed, and nothing about how another GPU rounds.
