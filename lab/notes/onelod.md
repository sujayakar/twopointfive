# The second core: one rule for how much detail a ray meets (measured 9 November)

Code: src/core2 (rays.wgsl, blocks.ts, measure.ts, kept.wgsl, cells.wgsl, smoke.wgsl, carry.*, onerule.ts), methods `onerule` (the picture) and `onelin` (as plain light). `card.cjs <views> --against` sets methods against the reference; `ruleNref` are path tracers that obey the rule alone.

The rule: a ray is f(t) = w + s t wide, shifted by one die a ray. Solids have three classes: gone for rays wider than 0.32 m, than 0.64 m, or never (what is long and flat, whatever its size: it may part one room from another). What has gone, and every leaf, is in the world's cells (0.32 and 0.64 m): thickness along x y z, a tight box, six colors, and kept light (eight threads a look: six rays, the sun, one lamp).

Verdict, 1280 x 720, one box a scene, `realtime` against `onerule`

| | first | second |
|---|---|---|
| surfaces, median % (roof day, dusk, night; garden; ground day, night) | 2.58 4.75 3.93 3.78 5.16 6.38 | 2.60 4.51 3.81 3.70 4.00 4.81 |
| surfaces (street day, dusk, night; store; shelf) | 2.75 6.70 6.71 5.71 8.48 | 2.74 6.51 6.53 5.28 8.48 |
| leaves by day (garden, roof, street) | 11.9 8.4 15.7 | 10.4 6.9 13.8 |
| leaves by lamps (dusk, roof night, ground night), as light | 10.8 10.6 10.8 | 10.9 10.8 12.6 |
| leaves' contrast, 4 px (roof, garden) | 81 79 % | 86 84 % |
| darkest tenth of leaves, stops bright (roof, garden) | 0.48 0.60 | 0.10 0.34 |
| flicker, all | 0.46 to 0.68 | the same |
| off in motion, garden leaves | 22.0 | 20.0 |
| milliseconds, mean of 11 views (a box is sure to 4 %) | 25.2 | 25.7 |
| eight times the garden adds | 0.98 | 0.80 |
| memory with its light, MB: rooftop, street, forest | 1.2 2.7 26 | 1.3 1.7 11 |
| code, KB | 23 | 61 |

What was learned
- The error of the rule alone falls as the square root of s, not its square: how wide a ray is hardly matters, what a cell is like does. (Path tracer: 1 to 3 % over blocks at s = 0.5, brightness within 0.03 stops; with nothing in the place of what left, 10 to 55 %.)
- Cells wider than 0.64 m span walls and leak. A cell must hold one side of a wall (found at measuring, by rays between its contents).
- Size does not tell a wall: the strip over a door is half a metre high. Until objects say so themselves, what is long and flat stays.
- Side by side, what each part is worth: leaves in measured cells, a third to a half of their error by day. Clutter in cells, 0.2 to 1.1 points indoors at the same time (small things have no entry in the light table, and were dark). Limbs as soft boxes: nothing that shows (bounce from six people is too little), 0.1 ms: not worth their code. Smoke by width: 0.7 to 0.9 points near smoke. A twig's own block, or leaves one by one near a bounce ray: 3 to 8 ms, nothing. Leaves that gather with rays of their own: a little better by day, worse by lamps, noisier.
- A lamp's own shadow at what is seen wants solids as they are: cells there cost error and time.
- Content costs the first core little too (8 times the leaves 1 ms, 4 times the stock 0.3): pixels cost, not things. Cells do not change that. Boxes are as quick exact as in cells.
- A look at a cell is nearly all waiting: one ray a thread, every fourth frame, took 0.7 ms (the foliage's) to 0.12.

Open: leaves lit by lamps (street by night, blocks 8 to 14 %); the pool's wall and leaves in mirrors; smoke itself is too bright in its middle in both cores; forest and pass were not measured.
