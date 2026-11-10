# What a maker hands over, so that an object can be drawn fine

An object is drawn fine and traced in boxes: what the eye meets is the fine shape, and every ray, foot and blast goes on meeting the pieces you put (notes/makers.md). So a fine object is the pieces you make today, and over them, shapes. Nothing is downloaded; everything is a program.

## Three kinds of shape (`src/fine/template.ts`)
- **FITS**: made once, stretched to fit any box. Every vertex knows how much it goes with each side of the box (s, −1 to 1) and where it is from there in metres (o): place = s · half + o. Edges, piping and chips keep their size. Use one that exists first: `hard` (painted, plastic), `metal`, `cast` (stone, concrete, plaster), `wood`, `soft` (cushions), `drum`, `ball`, `limb`, `knot`, `paver`, `board`, `panel`, `sheet`. Of the rooftop's 6,577 things all but 86 are one of 22 such shapes.
- **PLAIN**: a shape of its own, for what nothing else is like. If it is a piece of planes and curves, `odd` makes it from the piece, creases and all.
- **SKINNED**: one skin over a rig. Every vertex has two bones (pieces, in the order the rig makes them) and a weight. All bodies of a rig share one shape.

## What you hand over, for one object
1. **The pieces**, as now. They are the box that stands in: nothing you draw may be farther than 2 cm from them, 3 where nobody walks or shoots (shadows and blasts are theirs). Their tint is the mean of what is drawn: the bench compares the two under a grey sky, within 2 %.
2. **Which shape each piece is drawn with** (usually decided by its material and form: `shapeOf` in src/fine/make.ts). Say so only where you want another.
3. **New shapes, if any**, each a function of nothing: a field and a net (`solid` in src/mesh/made.ts), what rises from it, what color it is (times the piece's material and tint: keep the mean at 1: what is brighter than 1.1 is cut), and its kind. Finest detail: what is seen at 2.7 mm a pixel, the game's nearest (so 1.5 mm is fine enough; below that is the material's). 2,000 to 20,000 triangles; 0.2 to 0.8 MB with its levels; five seconds to make at most, since it is made once and baked.
4. **Its far levels.** Simplifying makes the near ones by itself. Give the last two by hand where form is simple (a box with cut edges, 44 triangles; a sharp one, 12; a drum of 28, 16 and 9 sides), each with its error in metres: the most the outline is off, and no less than half the size of the grain that is lost with it, or the swap is seen.
5. **How it moves**: nothing to add. A shape on a moving piece moves with it; a skinned one with its rig. Rows of a piece's frame must be square and of length 1.
6. **How it breaks**: what is laid on a wall in parts comes out part by part; what is one thing goes whole with its piece, and the wedges and rubble that come are drawn as the boxes they are. Rubble of its own material is the piece's.
7. **Print and patterns** stay the piece's: marks, patterns and bumps are laid on the face a point is nearest to. A skin laid over a face (pavers, boards) hides that face's pattern.

## What it costs (A10G, the whole rooftop, two cars, six people, against boxes in one run)
- Frames, the world moving, 30 views by day, dusk and night: 1080p at 0.75 from 0.30 less to 0.54 ms more (mean 0.29) on 10 to 15; 1080p whole 0.69 less to 0.65 more on 15 to 23; 1440p at 0.75 0.64 less to 0.69 more. Less where curved pieces fill the picture (a car): boxes find those a pixel at a time, triangles do not. Its own passes 0.17 to 0.31. CPU up to 0.4 ms.
- 6,577 things, 53.6 M triangles placed, 0.16 to 0.70 M drawn. 71 shapes (49 of them odd, 38 of those the two cars'), 0.40 M triangles, 16.4 MB, 8.0 squeezed; read back in a tenth of a second (0.2 with the page's text around it). Buffers that do not grow with the scene: 17 MB of indices, 2 of clusters.
- Made: 41 to 48 s in one thread, 23 in two, 17.5 in four or more: the body alone is 17. The table of what is drawn for what: 30 ms.
- A hole: 10 to 25 ms for the blow itself, with boxes the same; it takes 4 to 7 panels. The longest frame after is the boxes' too (50 to 85 ms: what is joined, where lamps reach).
- Swaps, a slow pull back, a thirtieth a step, the settled picture before and after: see the table of 10 November at the end.
- On a Mac (Apple GPU, Metal 3, the page's button, 16 views, 10 November): 0.46 ms more GPU time in the mean, 1.25 less (a car by night) to 1.18 more (walking, rooms, by night), on 19 to 26; by the clock no difference (0.01 ms in the mean). Its own passes 0.2 to 1.0. Shapes read in 0.06 s.
- A thing in view: 12 triangles and up. A shape more: a quarter of a megabyte, nothing a frame.

## What the bench should ask of one (not built)
How far the shape is from its pieces (2 cm); brightness of shape and pieces under a grey sky and a low sun (2 %); each swap, as for levels now; triangles and bytes; seconds to make.

## Far levels (10 November, after Sujay found them poor)
A level's vertices are some of the finest's, and kept their own normals and colors: one grain of noise was then spread over a triangle a hundred times its size, the same on every slab. Each level now has normals from its own triangles (by the squares of areas, so that a wide face is not bent by the slivers of its rim) and the mean color of all that went into each vertex. Where two stuffs meet in one skin the triangles are cut along the line, and the simplifier keeps the line (its vertices go only along it; where three meet they stay): each triangle says whose piece it is made of, in the top byte of its index. Letting dice choose the level near a swap made nothing better.

## Known faults of the first ones
Skins hide the patterns under them (planks of a floor look plainer fine than as a box). A box is cut by the section with an end to show; a skin or a body is cut open. Cloth, water, glass, leaves and the city stay as they were.
