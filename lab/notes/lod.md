# One rule for how much detail a ray meets

The idea (9 November): whatever stands for something coarser (leaves as fog, a kept cell of light, a blurred sky, a smaller texture) is the same choice as a rasterizer's level of detail. Carry one number with every ray, its *footprint*: how wide an area this answer will be averaged over. Stop at the first thing smaller than it.

- the camera's ray: a pixel, growing with distance · a mirror's: the same cone, longer · a glossy one: widens by its lobe · a wide one: about as wide as it is long
- the tree of boxes is already the levels: give each node how much it covers (six directions), its mean color and glow (six), the spread of its normals, and the light kept there. A ray stops at a node smaller than its footprint. A twig, a hand, a shelf of forty packets and a brick of smoke are then one case.
- so kept light (patches, leaf cells, air cells) is one kind of entry, on a node.
- motion has levels too: leaf, twig, branch, trunk. Dropped motion becomes softness; dropped normals become roughness; dropped holes become coverage.
- later: meshes as one more kind of object, each level with its error in metres, raster finer than rays. A ray starts that error off the surface; the last pixel or two of contact comes from the depth buffer.

What must not be lost
- coverage is not additive: two half-covered nodes may hide the same half. A node with a wall in it must never become fog, or rooms leak: walls stay exact, clutter merges.
- levels are picked by dice, not at a line, so there are no seams.
- correlation: one color a node is not enough (red to the north, white to the south).

Where to begin: a second core beside the first. All that stands above asks two questions (`nearest`, `visible` in src/wgsl/scene.wgsl) and reads a surface (`surfaceAt`). Same scenes, same reference, switched by a word; the first is deleted when it loses. Foliage first, since it has the outer levels already (leaf, grid): the missing middle is the twig's own small block of voxels, in the twig's frame, entered by moving the ray. Then shelves (the street), people, smoke. Then a floor patch: pebbles as instances near, relief, then normals and roughness, made from a hash of where it is; the test is that brightness does not move between levels.

What is known already: notes/measured.md. `foot(w, s)` in scene.wgsl is the first piece of it: every stage says how wide its rays are; small things and leaves are met as far as that.
