# The terrace's corner, in boxes and in fine geometry (10 November)

`scene/corner.ts` (study 'corner'): the rooftop's garden corner finished: 47 shapes (`mesh/made.ts`), 7,200 parts, 19.5 M triangles placed, 0.72 M of shapes, 48 MB, 31 s in one thread. `tools/cornercard.cjs`, `buildcorner.ts` (the page).

- **Drawn fine, traced in boxes.** Each fine thing has a piece that stands in for it (`ghost`: what people walk on, what breaks, what rays meet; recolored to the mean of what it stands for). `withMeshes(method, true)`: kernels without triangles (`Stage.before`), raster and first hit with them, what is lost of light from mesh/lost.ts. Against boxes, 1080p at 0.75: +0.03 to +0.79 ms of 11 to 17 (23 views, day, dusk, night); 1440p 0.0 to +1.3 of 18 to 30. Meshes 0.15 to 0.30 of it.
- **Rays on the fine**: +6.7 to +20.5 ms. Error against its own paths the same (four views: 9.0 and 9.0 %; boxes 6.0 %): light of each kind of mesh within 1 to 3 % of each other.
- Text that is never run weighs: with the triangles' walk merely in the kernel gather took 5.2 for 2.3 ms. A walk's stack weighs as much: 48 entries to 29 (what the trees need) 25.4 to 21.9 ms, close by 38 to 28.
- Stand-ins in the old colors made everything round them 3 to 30 % too bright.
- Shade looked for in the picture's depth (`near`): no difference (third time asked).
- A hole: what is faced with whole things loses those whose middle is in the ring (mesh/knock.ts); the boxes behind break as they do; the rim is toothed by itself.
- The section: meshes are cut by a discard, and box cores make the cap.
- Trap: bit 7 of a piece's flags is BROKEN; a mesh and its stand-in share bit 6 (a mesh's count of curves is 3).
