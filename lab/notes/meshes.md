# Meshes drawn about as fine as a pixel, with rays on coarser levels

The dream (9 November): ray tracing in software, in real time, where the raster is as fine as the screen can show, and every ray goes through one carefully made system of levels. This note is the first half and the seam between the two; notes/lod.md is the second.

What was made
- `src/mesh/` shapes (pebbles, a broken column; from seeds, nothing loaded), fewer triangles by quadric error, clusters of 128 in a graph of levels, packing. About 0.3 M triangles a second in one thread; every cut is closed (`bun run tools/meshes.ts`).
- `src/stages/meshes.ts` picks clusters on the GPU (a tree eight wide, round by round), draws them into the pieces' own ids and depth, and looks twice at depth (last frame's, then this one's for what that hid), so that nothing is missing for a frame. If the lists fill, it goes coarser until they do not.
- `src/wgsl/mesh.wgsl` a mesh is one more kind of piece: `meet`, `visible`, `surfaceAt` answer for it. An object has levels, each a list of children (a square of floor: 1,150 stones, or one relief); a mesh has levels, each a tree.
- `src/stages/seam.*` the seam, asked in many ways at once with the same dice. `src/meshpage.ts`, `yard.src.html` a page (`bun run buildyard.ts`); `node tools/meshcard.cjs` measures.

What was measured (A10G, 1920 x 1080 unless said)
- **The raster is nearly free.** At half a pixel of error: 3.2 M triangles 0.69 ms in all (pick 0.20, indices 0.15, draw 0.28, depth 0.07); 2.3 M 0.52; 0.6 M 0.28. The finest column (2.8 M) smaller and smaller: 7.8 M a ms where 0.4 M are seen, 13.8 M a ms where none is bigger than a thousandth of a pixel. Small triangles do not begin to cost: there is no reason for a raster of our own.
- **On a Mac** (Apple GPU, Metal 3, which one was not said; the page's Measure, 10 November; the depth pyramid not counted): 5.0 to 7.0 M a ms, about half the card, and no slowdown under a pixel there either: 5.3 M a ms where the column's triangles are seen, 7.0 where they are tiny. The yard at 9 m, half a pixel: 3.56 M triangles, pick 0.22, indices 0.14, draw 0.71 ms (the card: 3.50 M, 0.20, 0.16, 0.30); a pixel: 1.64 M, 0.56 ms in all. Shared corners draw twice as quickly as fetched ones. 1.3 M triangles of meshes made in 7.8 s.
- **Corners shared by indices written each frame** beat corners fetched one by one: in the draw 1.3 to 1.9 times on full clusters, 4 times where a cluster is a stone of 32 triangles; with the writing of indices counted 1.1, 1.2 and 2.8 times. A triangle's first corner belongs to no other, and tells it (there is no triangle number in WebGPU).
- **Left out**: what depth hides is worth 2.6 times fewer triangles close to the column, 1.05 over the floor. Cones are worth nothing on rough stone.
- **Where it is**: at half a pixel half of what is drawn is within 0.13 px of the finest mesh, nine tenths within 0.45 (0.74 from 30 m). The finest column's triangles are 2 mm: under 0.3 mm a pixel they are seen as soft shapes, and the material's grain goes on from there.
- **Memory**: 65 bytes a triangle of the finest, levels and trees for rays and all (222 MB for 3.4 M; 88 MB for the page's 1.3 M).
- **Rays**: in the real-time frame at 1280 x 720, 9 m away: the finest 45.7 ms, by footprint 23.2, four footprints 18.3; error against the paths 17.0, 12.5, 12.6 %. Coarser is not worse, only cheaper.
- **The seam** (traced triangles 1 to 64 times fewer than drawn; share of the sun wrong): starting on the surface 0.7 to 27 % (stones), 4 to 27 % (column); one error off along the normal 0.6 to 5 and 1 to 4 %; **passing through the backs of triangles within three errors 0.3 to 1.9 and 0.5 to 2.3 %**. Bounced light with that rule 0.4 to 4.5 and 1.4 to 5.6 %. From the depth buffer nothing more is to be had.
- Over the stones seen at 4 to 10 mm a pixel, where a ray meets the relief of what is drawn as stones: shadow 2 to 10 %, bounce 7 to 11 % (too bright: the dark between the stones is lost first).
- **How bright a level is** (path tracer, all on one level): a stone of about 100 triangles sends back 3 % less, of 50 6 %, of 32 10 %, alike at any hour: one factor a level would mend it. A relief flattened to 4, 13, 26 mm: noon +1, +2, +1 %; low sun +9, +35, +75 %; grey sky +9, +26, +29 %. No color mends that.

Traps
- Distance alone is not enough of an error: color and small bumps vanish long before outlines do. Both count now (`COLOR`, `NORMAL` in simplify.ts). Rays go by distance alone (`apart`).
- Which side of a surface a light is on must be asked of the smooth normal, or facets near the sun's edge go dark one by one.
- Normals averaged as vertices merge stand too steep (vertices gather where a shape bends): taken from the triangles that are left, they keep what falls on them. Colors are averaged as light.
- A depth pyramid whose mips round down is read by shifting the pixel, not by scaling 0 to 1.
- Waiting for each frame lets the GPU's clock fall by half: keep some in flight.

Left open: brightness of coarse levels (above), and rims of reliefs, which are simplified each on its own; swaps of a tenth of the error move 1 % of pixels by 12 of 255 (not looked at in motion); building takes 20 to 45 s in one thread; what rays cost on a Mac has not been asked.
