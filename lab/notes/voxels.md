# Blocks of cells as the coarsest levels (10 November)

The question: would even simpler levels, voxels, do for the coarsest, and for foliage?

    bun run tools/voxels.ts chair ficus                        hand made levels against cells of several kinds, at ray widths of 4 to 32 cm, without a GPU (WIDTHS, KINDS)
    node lab/tools/object.cjs ficus~0.08,0.16 ficus@0.08,0.16  on a card, by the bench's marks: the finest, then cells of those sizes (~), or the coarsest hand made level where those would take over (@)

A block (src/objects/voxels.ts) is measured, never made: rays through the finest level from six sides. A cell holds how deep its contents are along x, y, z across a tight box round them, that box, what it sends back from six sides, a mean normal a side, and how much is leaves: 48 bytes. A ray goes through it as through fog; drawn, it stops where a cell's tight box begins, by a die (voxWalk in wgsl/scene.wgsl).

**What rays meet: yes, from about 16 cm wide.** Cells half as wide as the ray, against the hand made coarsest level (12 objects at 32 cm, no GPU: blocked otherwise, light sent back otherwise, a side's light in all): 10.9, 27.5 and 15.6 % against 10.2, 23.5 and 13.2; at 16 cm closer than hand made in what they block in 7 objects of 9, about even in light. At 4 and 8 cm hand made is clearly better and smaller. On the card, if every ray met the level, the sky's shadow is wrong by 0 to 7 % with cells of 8 and 16 cm and 0 to 5 with hand made levels (chair, person, ficus, tree, fern, moped; under its own grain for most); clutter wants smaller cells (shelf 9 to 12 % at 8 cm, 24 to 29 at 16; hand made 6.5).

**What is drawn: no, for solid things.** At the same distance the hand made coarsest reaches nearly every mark; cells are 0.2 to 0.65 stops too bright on the chair, the moped and (at 16 cm) the shelf, right on the person, and move its color by 1 to 4 %. They are dearer by the pixel too: a crowd costs 1.0 to 1.4 times the finest (chair), 1.8 to 3.5 (person), but half for clutter (shelf 0.44 to 0.52 against 0.76, moped 0.55 to 0.71 against 0.78).

**Foliage is where they look best.** A ficus in cells of 32 cm is 9 KB (thinned leaves 67, all leaves 193) and within its marks but for filling 9 % too much; a street tree at 32 cm 83 KB (94, 272), shadow within its mark, 0.25 stops dark from the front by noon, right from the side. But drawing them costs what all the leaves cost (0.92 and 0.94), thinned leaves 0.74 and 0.67. So: thinned leaves to draw, cells for rays.

**What in a cell earns its bytes** (17 objects at 16 cm: blocked otherwise, light otherwise, color moved): as it is 13.6, 34.8, 1.0 · no tight box 23.1, 40.3, 1.1 · one depth for three 14.7, 41.7, 1.8 · one color for six 13.6, 36.6, 1.8 (a car 4.9 against 0.4 at 32 cm; a chair or a person no difference) · with a mean normal a side 13.6, 30.8, 1.0 (a person 38 to 28, a car 25 to 17) · plain fog 25.2, 47.4, 2.1 · one box a cell instead 28.6, 45.2, 4.0.

**What mended them** (12 objects at 32 cm, all together: 18.0, 40.8, 28.5 % to 15.5, 33.0, 17.8)
- a box round a round thing is too wide from a corner: rays from thirteen ways draw the sides in that stand free (not those that go on into the next cell, or seams open). Filled 7 to 23 % too much, now about 5
- colors less by how shut in a place is by its own cell's contents; then one gain a block, so that the whole sends back what the finest does
- the cell a ray begins at is left out, by its name; left out by nearness (4 mm), its neighbors went too, and light got in
- leaves by the normals that are seen (cosine about the ray), not any. Was 0.3 to 0.5 stops dark
- fitting depths along whole rays (tomography) stops rays better and lights worse: what should be solid lets sun in

**So:** makers stop at an error of 3 to 8 cm; the engine measures the rest, for rays. The world's cells of 0.32 m are that already: the mends above and the normals are for them. Blocks in an object's own frame are worth it only for what moves and is large, or if something must be seen at hundreds of metres.

**In the world's cells, with kept light, on a card** (notes/onelod.md, 2e7c053b0): the sides drawn in hold on surfaces nearly everywhere (garden 4.2 to 3.9, ground by night 6.7 to 6.1, street by night 8.7 to 8.4 % over blocks) once leaves one behind another are counted; normals and openness gain a tenth of a point or less there and were left out; the gain and the leaves' normals were not tried. So the sketch's light is a guide to form, not to what light in the real renderer gains.

Not done: rays that choose cells by width while the picture is drawn from pieces; a block that bends in the wind; metals and highlights (a cell is matte); why solid cells are too bright on the card when the sketch says they are not.

**A hole in the middle of a tree.** Seen by the person who asked for this, in the coarse cells. Three causes:
- *Filling.* Depth by three axes alone makes a sheet that stands askew of them too thin to a ray that meets it square (0.58 of what it is, at worst) and too thick to one along it; a crown seen from in front is such sheets, and only its front stands in the way. Three more numbers a cell (how the ways go together), fitted from rays of thirteen ways, for leaves and for what is thin in its cell. Blocked otherwise at 32 cm rays and cells, no GPU: 10.3 % as measured, 8.9 with sides drawn in, 6.4 now; the thinnest patch of the middle 0.71 and 0.76 against 0.81, now 0.79. In the world's cells (255b489a3): 0.03 to 0.15 points less error in six rooftop views of seven. Fitting depths along whole rays again gained nothing.
- *Color.* A cell with pale limbs and dark leaves was one pale leaf that light goes through. It keeps the two colors apart, and a ray meets one or the other by a die. Noon brightness 0.18 to 0.12 stops off at 16 cm, 0.15 to 0.11 at 64.
- *Wind.* The finest was drawn with its leaves leaning in the bench's wind, the cells as measured in still air: the wind had closed the gap that the cells show. In still air the finest has that gap too; what is filled otherwise falls from 17.6, 15.3, 16.7 % to 7.8, 9.8, 11.7 at 16, 32, 64 cm. So every comparison of cells with leaves above (v2 to v8) is kinder to hand made levels, which lean too, than it should be. Cells that lean are not made.
Leaves are met in their cell's depth and shaded by its own contents (not at its face, unshaded): the crown's brightness comes within 0.02 to 0.06 stops by noon, but the fern's thin fronds get 0.23 too bright. On a card, the middle's mean light through 0.064 and 0.069 (16, 32 cm) against 0.037 at first; now 0.044 and 0.045 against 0.040 (green; floor 0.34). The ficus reaches every mark in cells of 8, 16 and 32 cm; the tree misses brightness by lamp (0.2 stops) and color by a little; the fern 4.
