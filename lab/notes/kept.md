# Light kept on surfaces: what is kept, why this way, what it is worth

## What is in

Patches in a hash (wgsl/cache.wgsl): 0.48 m long and wide, 0.2 m thin along the normal. Four things make them better than the ones before, in order of worth
1. Who founds a cell is drawn by lot (a shuffle of the callers' numbers). The smallest screen number used to win, which is always the same corner of the cell.
2. An entry's point roams to wherever a ray just landed in it, if that is joined to where it began: so it holds the mean of its patch, not the light at one point.
3. The entries' own rays found entries too: no coarse level is needed, and rooms nobody looks at are lit.
4. Up to four entries stand for one cell (walls part it), and where none of them speaks for a place the nearest answers. (Before: black.)
Later samples weigh as their cube: the first ones, lit by a table that was still dark, are soon forgotten. The entries' own rays begin at a point. The city's patches are 1.9 m to 15 m, by how far off they are (before: cubes of 6 m).
A run repeats to the bit: an entry calls by its key, never by its slot, and who wins a cell goes by numbers alone.

When a wall gives way: for as long as the changes say so (signals.wgsl, `changes.at`) the entries within reach have a short memory and look every second frame (the first at once, to forget), and each first looks whether the surface under its point is still there. If not it is struck out within two frames (its slot is then free for whoever lands in that cell). What stood before a lamp or before the sky only makes the light change: the short memory is for that. `joined` must be marked anew first (joins.ts). Doors work the same way, so nothing hangs where a leaf was. Entries left in the air by people who walked on are not struck out until a change comes near; they cost a ray in eight frames each and nobody reads them.

## What it is worth, on the one ray core (A10G, 1280 x 720, one box; bounced light against paths in the same gather, mean over blocks of 24 x 24 half-size pixels)

| | night | day | dark rooms | one lit room | street | MB in use |
|---|---|---|---|---|---|---|
| before: 0.32 m, one point, a coarse level | 2.9 | 3.9 | 5.9 | 8.8 | 3.0 | 6 |
| patches that roam, at 0.32 m | 1.9 | 1.7 | 2.3 | 3.7 | 1.6 | 13 to 19 |
| the same, at 0.48 m (what is in) | 2.2 | 2.0 | 3.0 | 4.9 | 2.0 | 8 to 13 |
| the same, at 0.64 m | 2.5 | 2.3 | 4.1 | 6.4 | 2.5 | 6 to 11 |
| at 0.32 m, and a path goes on till it is as wide as an entry (`fine` 1) | 1.6 | 1.4 | 2.0 | 3.4 | 1.3 | |

Milliseconds over a frame with nothing kept: before 1.1 (night), 1.3 (street); at 0.32 m 1.2 and 1.2 to 1.9 (two boxes); at 0.64 m 0.1 to 0.2 less. `fine` at 1 costs 1.4 more in rooms, 3.6 on the street.
After a lamp in the atrium: within 6 % of the truth after 24 frames (before: more than 240). After the blackout, frames 8, 64, 240: 135, 36, 16 % off (before: 306, 143, 76; the light left is a hundredth of what was).

The first race had 0.64 m as good as 0.32. On the one ray core it is not: what is small has left for cells, and cells read this table at close range.

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
