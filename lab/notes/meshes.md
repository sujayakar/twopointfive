# Meshes drawn about as fine as a pixel, with rays on coarser levels

The dream (9 November): ray tracing in software, in real time, where the raster is as fine as the screen can show, and every ray goes through one carefully made system of levels. This note is the first half and the seam; notes/lod.md is the second.

What to make
1. **Clusters.** A mesh is cut into clusters of 64 to 128 triangles, and those into a graph of levels: simplify groups of clusters with their shared border held, split again, each with its error in metres. Built in TypeScript at load (or once, offline, packed small). No dependency.
2. **Choosing and drawing.** A compute pass walks the graph for each placement: cull by view and by last frame's depth, take the coarsest cluster whose error is under a pixel, append it to a list; one indirect draw by vertex pulling into the same id and depth targets the pieces use. There are no mesh shaders and no 64-bit atomics in WebGPU, so no software raster of tiny triangles: find where the hardware's small triangles start to cost, and choose the pixel error by that.
3. **First hit** from the id alone, as for pieces: position, normal, coordinates from the triangle; normals that vanish at a level become roughness (keep their spread a cluster).
4. **Few shapes, many places.** It is an instancing engine first: a few rocks, a few chips and leaves, a chair, a car, placed by the thousand, some of them made from a hash of where they are (the floor patch in notes/lod.md).

The seam, which is the research
- A ray begins on the fine surface and meets a coarser one. Each level knows its error; a ray starts that error off the surface along its normal, and takes levels by its footprint from there. The last pixel or two of contact: from the depth buffer.
- Measure it: the reference traces the finest level (triangles as one more kind of leaf in its tree; mind smooth normals at grazing angles). Show shadow and bounce error against the ratio between drawn and traced triangles, from 1 to 64, and where self-shadowing or floating contact begins to be seen.

Until the new ray core exists, let rays meet stand-ins made of pieces (a car has its own already).

Shapes without artists: pebbles and broken stone by cutting and wearing down, bark and cushions by displacement, cloth from the lab's own simulation, car bodies by subdividing the curved pieces. Nothing is downloaded.

Wanted at the end: a floor and a hero object that hold up at any zoom; triangles a millisecond (card, and a page to try on a Mac); memory; the seam's curve; a verdict on what belongs in 2.7.
