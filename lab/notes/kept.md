# Which way of keeping bounced light: what was found

(9 November, A10G, three races of 60 to 84 settings; the page of curves is outside the repo. "Off" = bounced light against paths in the same gather, mean over blocks of 24 × 24 half-size pixels, rooms by night / by day / dark / street.)

| way | off % | ms over nothing kept | MB in use |
|---|---|---|---|
| today: hash, 0.32 m, one point, coarse level | 3.5 / 3.9 / 5.9 / 3.8 | 1.0 to 1.2 | 6.9 |
| patches that roam, thin, 0.64 m | 2.1 / 1.9 / 2.9 / 2.0 | 1.0 to 1.2 | 7 to 12 |
| the same, 0.32 m | 1.9 / 1.6 / 2.3 / 1.5 | 1.2 to 1.4 | 15 to 22 |
| texels on faces, 0.32 m | 2.3 / 1.9 / 2.4 / 4.2 | 0.6 to 0.7 | 9 |
| probe grid, 0.5 m, lamps asked | 2.9 / 6.1 / 4.6 / 2.0 | 4.2 to 6.5 | 31 |
| probes near surfaces, 0.25 m, lamps kept | 5.3 / 5.1 / 9.1 / 4.6 | 1.2 to 1.5 | 15 |
| surfels, 0.3 m | 2.6 / 2.6 / 3.2 / 3.1 | 8 to 10 | 10 |
| tiny network, lamps kept | 4.8 / 5.4 / 7.9 / 2.0 | 1.9 to 2.5 | 8 |
| nothing kept, 6 more surfaces | 8.8 / 8.0 / 8.9 / 3.0 | 21 to 32 | 0 |

What to take, in order of worth (src/caches/patch*.wgsl, `patch cell=0.64 coarse=0 spread=1 soft=1 tries=4 roam=1 flat=1`)
1. Who founds a cell: shuffle the callers' numbers (`shuffled`). Today the smallest wins, which is always the same corner of the cell: 3.5 to 2.9 %, free.
2. Four entries may stand for a cell, and where none is joined the first answers (today: black). 2.9 to 2.7, 0.05 ms. One entry instead of two: 8.2.
3. The entries' own rays found entries (`spread`): no coarse level is needed, rooms nobody looks at are lit. 2.7 to 2.3.
4. The entry's point roams to where a ray just landed in it, if that is joined to where it began; the cell is 0.2 m thin along its normal (`roam`, `flat`). 2.3 to 1.9; then 0.64 m cells cost 0.2 points and save a third of the time.

Key and size: an entry that is the mean of its patch may be 0.64 m; one with a fixed point 0.32 m. Levels picked by the ray's width: no gain, more memory (a place is asked from near and far, so its fine level is found anyway).
Direct light inside: asking costs 2.3 to 4.6 ms and gains nothing at 0.64 m or finer.
`joined` is 0.2 to 0.7 ms and is what keeps the dark dark (without it dark rooms read 1.5 to 2.3 times too bright).
After a lamp: what is slow is the memory of 96 samples. 24 settles twice as soon (48 frames to within 6 % for 98) for half a point at rest, and stirs five times as much.
A lamp's formula with a ray beside it (`lamp=1`) instead of a drawn point: better in rooms, 2 % dark on the street (long lamps half hidden). Not taken.
The picture: halving the kept light's error moves the finished picture by 0.5 (day, street) to 1.5 points (dark); by night it goes 0.1 to 0.6 the wrong way. The rest is elsewhere.

Traps
- Entries need 2,500 frames to settle at one in eight a frame: old light lingers. Judge a way only after that, or from `dim`.
- What a plan does not list under `rest` is not settled when pictures or settling are asked of it.
- Coarse cells with one fixed point are a lottery: two runs differ by a factor.
- 10 storage buffers a stage on some devices, 4 of them the scene's.

Run: `node lab/tools/kept.cjs lab/tools/kept/<plan>.json` (plans: `bun run tools/kept/plans.ts`); crevices: `sh lab/tools/kept/crevice.sh fruitcrate`.
