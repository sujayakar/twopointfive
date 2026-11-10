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

The software adapter compiles big shaders slowly (the truth: half a minute). It says nothing about speed, and nothing about how another GPU rounds.
