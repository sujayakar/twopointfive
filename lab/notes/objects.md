# A bench that judges one object

The bet (9 November): levels of detail need not be made by a simplifier. Each object can be made with care, all its levels with it (triangles, analytic pieces, voxels, coverage), by someone who thinks about that one object: how it is built, what moves, what it is made of. Many such makers can work at once if the tools let each one see, alone and in minutes, whether the object is good. This is that tool.

The contract (agree it with notes/lod.md and notes/meshes.md; version it)
- an object is a small program: parameters in, levels out. No files from elsewhere
- a level: its shape (triangles, pieces, voxels, or only coverage by direction), its error in metres, its materials, its colors by direction if it has no surface
- what moves it: a rig shared by all levels; coarser levels hang on fewer bones; motion that is dropped says how much softness it leaves
- how it stands, what it weighs, what is solid, what breaks (as today's `thing`, `rig`, solid and rigid)

What the bench does with it (headless, one command, a page of pictures and a table; exit 1 when a mark is missed)
1. **Turntable**: near, middle, far, by noon, dusk, lamp, torch; real time beside the reference, the difference under them.
2. **Level swaps**: each level drawn at the distance and footprint where it would take over, against the finest: brightness and color shift, outline error in pixels, shadow's and bounce's error on a floor and a wall beside it, highlights kept as roughness.
3. **In motion**: a slow turn and a walk away through every swap: what pops (largest change a frame), what sparkles.
4. **In a crowd**: a hundred of it: milliseconds, megabytes, at each level; rays a pixel that meet it.
5. **Rules**: stands on something, nothing floats, no two faces share a plane, no piece without a thing, marks and print legible at the distance people stand, silhouettes readable from the game's camera.
6. **Alive**: swings, breaks, floats, is pushed, as its kind should; the run repeats.

The marks come from measuring good objects that exist, not from a guess. The page must be good to look at: a person glances at a sheet of fifty objects and sees the odd one out, which no number will tell.

First objects, chosen because they differ: a chair (pieces), a ficus (a volume that sways), a person (a rig), a shelf of packets (clutter), a car (curves, paint), a patch of gravel (a skin).

Then try the bet: three makers, three objects each, only this bench and these notes. Count how many come out good with no help, how long each took, and what the bench failed to see.


# What came of it (9 November)

    bun run tools/sketch.ts chair            a picture of every level, in seconds, no GPU;  --swap: what each swap will cost, and where
    bun run tools/objects.ts chair           the rules, and does it live as it says: a second, no GPU
    node lab/tools/object.cjs chair          the bench, on a card: 2 to 5 minutes, about 8 cents; $OUT/chair.jpg, chair.json; exit 1 when a mark is missed
    node lab/tools/object.cjs --faults       every planted fault, each with the mark that must miss it

How to make one: makers.md. The contract is `src/objects/object.ts`; a level puts what it likes into a World, so meshes and voxels are levels as soon as a World holds them. The marks are `src/objects/marks.ts`: what the first six reach, with room over the worst.

**The bet.** Three makers, each alone with the bench and the notes, three objects each, four card rounds at most. At the end 7 of 9 reached every mark of the trial; the bicycle missed one by a hair, the fruit crate four (three of them the renderer's own light at dusk). With the side view, added afterwards from what a maker asked for, the radiator's far level had to be tuned; 13 of all 15 objects reach every mark now. Work: 31, 35 and 56 minutes a maker for all three objects, waits left out; 2 to 4 card rounds an object, about 8 cents each. First round: one of nine reached every mark, the rest missed 2 to 15. The finest levels were good from the first (form, parts, color); what took the rounds was coarser levels and what the renderer does that nobody had said.

**What they found that nobody had seen.** All three, apart, found that quadrics were solved from a far origin in single precision (thin round pieces frayed from 20 m on); two that `settle` widens faces along a chain (up to 5 cm); that tints are cut at 1; that a leaf's shade knows only its own spray; that low wide things are thrown about by the rigid solver, and plates of a centimetre fall through the floor.

**What the bench could not see, and now does**: the side view, how far `settle` moves a face, levels that do not move as the finest does, where an outline parts, that a shadow hidden by the reference's grain is unknown and not right, the shadow that real time casts (said). **What it still does not see**: wear and dirt (all are factory new), undersides, one close view only, whether motion has life, thin metal things' shadows (grain), a wrong color that is wrong at every level, whether anybody would like it.

**Where today's renderer is off, by object** (real time beside the reference, near): flat and solid things 1 to 5 % in every light; a person and leaves 12 to 16 % by lamp and torch; crevices at dusk 15 % and 0.27 stops too bright (a third of it the kept light's cells, a little the small things' tree; the rest not found); pebbles 12 % by low light. Cost: levels hardly pay yet. A hundred chairs of 27 pieces cost 0.9 ms more than none, of 11 pieces 0.65; 49 cars 4.2, 4.0 and 3.7 ms; a hundred desk lamps 3.1 and 0.6: most of what a thing costs is by the pixel it covers, unless it has joints.
