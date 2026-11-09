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
