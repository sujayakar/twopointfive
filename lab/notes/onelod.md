# The second core: one rule for how much detail a ray meets (measured 9 November)

## As it landed (10 November), against the build before it (912bf126b), one box

| | roof day | garden | ground day | ground night | dusk | dark rooms | dark roof |
|---|---|---|---|---|---|---|---|
| before, ms (960 x 540) | 6.06 | 6.58 | 6.46 | 7.61 | 7.23 | 7.28 | 7.48 |
| now | 5.84 | 6.34 | 6.18 | 7.32 | 7.02 | 6.93 | 7.23 |
| before, error % | 3.34 | 6.78 | 3.12 | 4.41 | 5.42 | 8.28 | 4.27 |
| now | 3.35 | 6.70 | 3.09 | 4.39 | 5.51 | 9.04 | 4.38 |

As plain light against a reference of 2,500 paths the dark rooms are 8.05 before and 7.74 now. Contrast of leaves at 4 px: roof 81 to 86 %, dusk 67 to 71; darkest leaves nearer by far. 900 ticks repeat to the bit; no bad number in 600 frames; poison heals in a frame; a sealed room stays dark.

What the verdict below got wrong: part of the gain indoors was light that came past things. Shadow rays to lamps were as wide as their asker, so what is held close before a torch, or stands before a lamp, had left. They are now as wide as the light looks (`footLight`), and the rooms are where the kept light is dark again: ground by day 4.5 before, 4.1 now (3.8 with the fault); by night 5.6 and 5.7 (5.2); store 5.4 and 5.4; shelf 8.4 and 7.9. Better kept light (notes/kept.md) is what those wait for.

People: their parts leave by size like anything else and nothing stands for them. Kept exact for every ray they cost 0.1 to 0.5 points, since rays that meet them find no kept light.

Cells fitted tighter (10 November, after notes/voxels.md): rays from thirteen ways draw in the sides of a tight box that stand free, and thin what is not solid; and a cell of leaves counts how many stand behind one another, for its light. Over blocks, before and after: garden 4.2 to 4.0 %, its leaves 5.9 to 5.5; roof's leaves 8.5 to 7.7; ground by night 6.7 to 6.1; street by night 8.7 to 8.4, its leaves 12.6 to 9.8; but street's leaves by day 8.6 to 10.7. A frame costs the same; the map of cells a little more, once. Weighed and left out (a tenth of a point or less): colors by how open a place is to its cell's outside, a mean normal a side.

Left out: limbs, twig blocks, leaves with rays of their own. Weather reaches cells by a rule of thumb (cells.wgsl, `weathered`).

## The race, as it was run

It is the only core since 9 November, without limbs; the first one and the studies that weighed the two are in the history (f9fb7c0). Code: wgsl/rays.wgsl, wgsl/kept.wgsl, scene/blocks.ts, blockmap.ts, stages/cells.wgsl. `card.cjs <views> --against` sets methods against the reference; `ruleNref` are path tracers that obey the rule alone.

The rule: a ray is f(t) = w + s t wide, shifted by one die a ray. Solids have three classes: gone for rays wider than 0.32 m, than 0.64 m, or never (what is long and flat, whatever its size: it may part one room from another). What has gone, and every leaf, is in the world's cells (0.32 and 0.64 m): thickness along x y z, a tight box, six colors, and kept light (eight threads a look: six rays, the sun, one lamp).

Verdict, 1280 x 720, one box a scene, the first core against this one

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

(Surfaces: better in nine views of eleven, the same on the shelf, 0.02 worse on the roof by day. Over blocks of 16 pixels the garden and the shelf are 0.1 and 0.3 worse. Forest's memory is computed, the rest counted.)
- A room with no windows and no power, by day: 7.8e-6 of light in the second core, 7.5e-6 in the first, 4.7e-6 by the reference; the room beside it has 0.16. Cells do not leak.

Open: leaves lit by lamps (street by night, blocks 8 to 14 %); the pool's wall and leaves in mirrors; smoke itself is too bright in its middle in both cores; forest and pass were not measured.
