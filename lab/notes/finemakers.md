# What a maker hands over, so that an object can be drawn fine

An object is drawn fine and traced in boxes: what the eye meets is the fine shape, and every ray, foot and blast goes on meeting the pieces you put (notes/makers.md). So a fine object is the pieces you make today, and over them, shapes. Nothing is downloaded; everything is a program.

## Three kinds of shape (`src/fine/template.ts`)
- **FITS**: made once, stretched to fit any box. Every vertex knows how much it goes with each side of the box (s, −1 to 1) and where it is from there in metres (o): place = s · half + o. Edges, piping and chips keep their size. Use one that exists first: `hard` (painted, plastic), `metal`, `cast` (stone, concrete, plaster), `wood`, `soft` (cushions), `drum`, `ball`, `limb`, `knot`, `paver`, `board`, `panel`, `sheet`. Of the rooftop's 6,200 things all but 86 are one of 22 such shapes.
- **PLAIN**: a shape of its own, for what nothing else is like. If it is a piece of planes and curves, `odd` makes it from the piece, creases and all.
- **SKINNED**: one skin over a rig. Every vertex has two bones (pieces, in the order the rig makes them) and a weight. All bodies of a rig share one shape.

## What you hand over, for one object
1. **The pieces**, as now. They are the box that stands in: nothing you draw may be farther than 2 cm from them, 3 where nobody walks or shoots (shadows and blasts are theirs). Their tint is the mean of what is drawn: the bench compares the two under a grey sky, within 2 %.
2. **Which shape each piece is drawn with** (usually decided by its material and form: `shapeOf` in src/fine/make.ts). Say so only where you want another.
3. **New shapes, if any**, each a function of nothing: a field and a net (`solid` in src/mesh/made.ts), what rises from it, what color it is (times the piece's material and tint: keep the mean at 1), and its kind. Finest detail: what is seen at 2.7 mm a pixel, the game's nearest (so 1.5 mm is fine enough; below that is the material's). 2,000 to 20,000 triangles; 0.2 to 0.8 MB with its levels; five seconds to make at most, since it is made once and baked.
4. **Its far levels.** Simplifying makes the near ones by itself. Give the last two by hand where form is simple (a box with cut edges, 44 triangles; a sharp one, 12; a drum of 28, 16 and 9 sides), each with its error in metres: the most the outline is off, and no less than half the size of the grain that is lost with it, or the swap is seen.
5. **How it moves**: nothing to add. A shape on a moving piece moves with it; a skinned one with its rig. Rows of a piece's frame must be square and of length 1.
6. **How it breaks**: what is laid on a wall in parts comes out part by part; what is one thing goes whole with its piece, and the wedges and rubble that come are drawn as the boxes they are. Rubble of its own material is the piece's.
7. **Print and patterns** stay the piece's: marks, patterns and bumps are laid on the face a point is nearest to. A skin laid over a face (pavers, boards) hides that face's pattern.

## What it costs (A10G, the whole rooftop)
- 71 shapes, 18 MB, 9 squeezed; read back in a tenth of a second. Made: 41 s in one thread, 17 in four or more (the body alone is 16: it is the one that should be made finer by hand). The scene's table, 6,200 things at 80 bytes, takes 30 ms.
- A thing in view: 128 triangles and up. A shape more: half a megabyte, nothing a frame. A thing more: nothing that can be measured up to thousands.
- 0.2 to 0.3 ms a frame for all of it, less what the boxes no longer draw.

## What the bench should ask of one (not built)
How far the shape is from its pieces (2 cm); brightness of shape and pieces under a grey sky and a low sun (2 %); each swap, as for levels now; triangles and bytes; seconds to make.
