# Which way of keeping bounced light is best, for what it costs?

The question: frame time and memory against accuracy, as curves, for different radiance caches, in the same renderer against the same reference. Not which is best in general: which is best for small dense levels of known boxes, many small lamps, doors that open, seen from above.

What stays fixed: the gather (rays, denoiser, upscaler), materials, lamps. Only the part that answers "what light lands at x, facing n" changes: `CachePart` in src/stages/realtime.ts (`cacheRead`, `cacheSmooth`, `cacheWants`/`cacheFound`, and the stages that fill it). Foliage, smoke and air keep their own for now.

Contenders, each swept over its size and its rays a frame
1. none: paths followed to the end (2, 3, 4 bounces), as the ceiling and as the cost to beat
2. patches in a hash (today): cells 0.16 to 0.64 m, with and without the coarse level, joins, second entry
3. texels on faces: the level is known boxes, so a texel grid on each standing face, no hash; things that move fall back
4. probes in a grid: spacing 0.5 to 2 m, octahedral irradiance with mean distances (so that they do not leak), and the same as L1 harmonics
5. probes only near surfaces (hash of cells by six directions)
6. surfels: spawned where rays land, sized by curvature and distance
7. a tiny network over a hash grid, trained as it draws. Smallest bet; for scale
(8. if time: rays kept on the screen, reused between neighbors, no world cache)

Measures, all on rooftop by day, night, dark, and the street
- accuracy at rest: `--table`'s idea for any contender (what it answers at 8,192 points against paths, mean and spread), and `--bench`, `--detail`
- leaks: the dark room beside the lit one, thin walls
- after change: `--settle` (doors thrown open, a lamp switched, the sun moved)
- flicker: `--steady`
- cost: GPU ms of its own stages and of the readers (gather, mirror), bytes really used
The curves: error against ms, error against MB, frames to settle against ms. One run, one box a curve. Say what was seen and what was not.

Also wanted: which error is the cache's and which is everything else's (swap in the truth at the hit points and measure what is left).
