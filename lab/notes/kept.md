# Light kept on surfaces: what is kept, why this way, what it is worth

## What is in

Patches in a hash (wgsl/cache.wgsl): 0.48 m long and wide, 0.2 m thin along the normal. Four things make them better than the ones before, in order of worth
1. Who founds a cell is drawn by lot (a shuffle of the callers' numbers). The smallest screen number used to win, which is always the same corner of the cell.
2. An entry's point roams to wherever a ray just landed in it, if that is joined to where it began: so it holds the mean of its patch, not the light at one point.
3. The entries' own rays found entries too: no coarse level is needed, and rooms nobody looks at are lit.
4. Up to four entries stand for one cell (walls part it), and where none of them speaks for a place the nearest answers. (Before: black.)
Later samples weigh as their cube: the first ones, lit by a table that was still dark, are soon forgotten. The entries' own rays begin at a point. The city's patches are 1.9 m to 15 m, by how far off they are (before: cubes of 6 m).
A run repeats to the bit: an entry calls by its key, never by its slot, and who wins a cell goes by numbers alone.

When a wall gives way: for as long as the changes say so (signals.wgsl, `changes.at`) the entries within reach have a short memory and look every second frame (the first at once, to forget), and each first looks whether the surface under its point is still there. If not it is struck out within two frames (its slot is then free for whoever lands in that cell). What stood before a lamp or before the sky only makes the light change: the short memory is for that. `joined` must be marked anew first (joins.ts). Doors work the same way, so nothing hangs where a leaf was. What moves founds only patches four times as wide (as before): fine ones would be left behind wherever it has been, and slots are never given back. Those left in the air are not struck out until a change comes near; they cost a ray in eight frames each and nobody reads them.

## What it is worth, on the one ray core (A10G)

Bounced light against paths in the same gather: mean over blocks of 24 x 24 half-size pixels, %, 1280 x 720 (kept.cjs). Before: 160182a, after: ac04070.

| | night | day | dark rooms | one lit room | street | MB in use |
|---|---|---|---|---|---|---|
| before: 0.32 m, one point, a coarse level | 3.0 | 3.9 | 6.0 | 8.8 | 3.0 | 6 |
| now, 0.48 m | 1.5 | 1.5 | 3.2 | 5.0 | 2.1 | 8 |
| at 0.64 m | 1.7 | 1.7 | 3.7 | 4.9 | 2.4 | 5 to 6 |
| a path goes on till it is as wide as an entry (`fine` 1) | 1.0 | 1.2 | 2.5 | 3.9 | 1.4 | 8 |

(One lit room and the street before are from another box.) Milliseconds over a frame with nothing kept, night: before 1.14, now 1.20; 0.64 m the same to 0.1; `fine` at 1 costs 2.0 more in rooms, 4.0 on the street. At 0.32 m, before lamps were drawn by how near they are: 1.9, 1.7, 2.3, 3.7, 1.6 for 0.3 ms at rest, 13 to 19 MB, and 2 ms while every door moves.
Frames 4, 8, 16, 32 after the atrium's lamps: 11, 9, 5.9, 5.0 (before 14, 12, 9.6, 8.7; within 6 % only after 240). After the blackout: 102, 42, 11, 5 (before 281, 126, 31, 12).
Not better: the darkest tenth of one lit room among dark ones reads 1.21 of the truth (before 0.91); mirrors by night 8.8 % off (before 7.7).

Whole pictures (bench, 960 x 540, b5f4d1b beside ac04070; ms, error %)

| | before | now |
|---|---|---|
| roof day | 5.87, 3.30 | 5.98, 3.19 |
| garden | 6.40, 6.46 | 6.48, 6.57 |
| ground day | 6.24, 3.26 | 6.30, 3.69 |
| ground night | 7.40, 4.30 | 7.43, 4.28 |
| dusk | 7.08, 5.43 | 7.23, 4.94 |
| dark rooms | 7.01, 8.97 | 7.02, 7.86 |
| dark roof | 7.40, 4.27 | 7.35, 4.15 |
| roof night | 7.71, 4.46 | 7.79, 4.33 |
| street, store | 15.56, 6.47; 13.43, 4.68 | 15.71, 6.04; 13.60, 4.36 |

Ground by day alone, under the coat that goes by the way it looks (ea891a6 beside 0bb28dd): 3.56 before, 3.32 now; after 240 frames and after 1,500 the same, so it is not slow to settle. Why it came out worse in the row above, third of eight views and under the old coat, is not known.
The fixed run, doors in motion: 16.8 ms before, 17.1 now (in the open 0.15 more); it repeats to the bit; 600 frames watched without a bad number; poison heals in a frame.
By night, direct light alone is 4.4 % off in the median and 7.7 % over blocks (worst twentieth 26), the same before and after: that is what is left, and what the old table's darkness hid in part.

What made the difference after the first run on this core: a long lamp near by, drawn evenly, says next to nothing most times and a hundred times too much now and then, and was clamped: the blue room was a tenth dark. Its nearer parts are drawn more often (`aimNear`): night 2.1 to 1.5, day 1.9 to 1.5.
The first race had 0.64 m as good as 0.32. On the one ray core it was not, until lamps were drawn that way; 0.48 m is the middle.

## The race before it (9 November, the first ray core: rooms by night / by day / dark / street)

| way | off % | ms over nothing kept | MB |
|---|---|---|---|
| hash, 0.32 m, one point, coarse level | 3.5 / 3.9 / 5.9 / 3.8 | 1.0 to 1.2 | 7 |
| patches that roam, thin, 0.64 m | 2.1 / 1.9 / 2.9 / 2.0 | 1.0 to 1.2 | 7 to 12 |
| the same, 0.32 m | 1.9 / 1.6 / 2.3 / 1.5 | 1.2 to 1.4 | 15 to 22 |
| texels on faces, 0.32 m | 2.3 / 1.9 / 2.4 / 4.2 | 0.6 to 0.7 | 9 |
| probe grid, 0.5 m, lamps asked | 2.9 / 6.1 / 4.6 / 2.0 | 4.2 to 6.5 | 31 |
| probes near surfaces, 0.25 m, lamps kept | 5.3 / 5.1 / 9.1 / 4.6 | 1.2 to 1.5 | 15 |
| surfels, 0.3 m | 2.6 / 2.6 / 3.2 / 3.1 | 8 to 10 | 10 |
| tiny network, lamps kept | 4.8 / 5.4 / 7.9 / 2.0 | 1.9 to 2.5 | 8 |
| nothing kept, 6 more surfaces | 8.8 / 8.0 / 8.9 / 3.0 | 21 to 32 | 0 |

The ways that lost are in the history (d133cfa, src/caches).
- Levels picked by the ray's width: 0.2 points for 40 % more time and a third more memory (a place is asked from near and far, so its fine entry is found anyway).
- Direct light outside, asked at each reading: 2.3 to 4.6 ms, and nothing gained at 0.64 m or finer.
- `joined` is 0.2 to 0.7 ms and is what keeps the dark dark: without it dark rooms read 1.5 to 2.3 times too bright.
- A lamp's formula with a ray beside it instead of a drawn point: better in rooms, 2 % dark on the street (long lamps half hidden).

## Traps

- Entries need 2,500 frames to settle at one look in eight: old light lingers. Judge only after that.
- What a plan does not list under `rest` is not settled when pictures or settling are asked of it.
- Coarse cells with one fixed point are a lottery: two runs differ by a factor.
- 10 storage buffers a stage on some devices, 4 of them the scene's: `cache` and `cache claim` use all ten.
- Packed words (keys, normals) stay in buffers of words: as floats they may be flushed to nothing.
