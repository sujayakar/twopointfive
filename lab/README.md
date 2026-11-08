# Lab: a renderer to try ideas on

A small WebGPU renderer, from scratch, with no dependencies: one world, one clock in ticks, and a list of stages with typed inputs and outputs.

- `src/graph.ts` writes a stage's bindings from the signals it names, so a stage's WGSL is the algorithm alone.
- `src/stages/realtime.ts` builds the real-time picture from swappable parts: `CachePart`, `DenoisePart`, `UpscalePart`.
- `src/stages/path.wgsl` is the reference: the same pieces, materials, lamps and sky, traced with nothing cached, clamped or filtered.
- `src/scene` is the world as data. Everything drawn is a function of (tick, inputs, seed), so a run repeats exactly.

`bun run build.ts` writes one page, `dist/index.html`.
