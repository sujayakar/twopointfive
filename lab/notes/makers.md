# How to make an object

You make one object, alone: a small program in `src/objects/<name>.ts` that puts its levels into a world. Nothing is downloaded. The bench says whether it is good; look at its page too, because no number sees ugly.

1. Copy the one most like yours: `chair.ts` (pieces), `car.ts` (curved pieces), `ficus.ts` (leaves), `shelf.ts` (clutter), `person.ts` (a rig), `gravel.ts` (a skin of the floor). Add it to `index.ts`.
2. `bun run tools/objects.ts <name>` (a second, no GPU): does it stand, is it whole, does it live as it says.
3. `node tools/object.cjs <name> --quick` (ten minutes without a GPU: small and grainy, for its form), or without `--quick` on a card (three minutes). Open `out/<name>.jpg`. `--parts look` (or seen, swap, calm, print, crowd) does only that.
4. Mend, again, until every mark is reached and it looks right beside the reference, close by, and from the game's camera.

## What you write (`object.ts`)
- `size`, as it truly is; its front to +z; it stands on y = 0.
- `levels`, the finest first. Each is whole and is drawn alone. `error`, in metres: no part of the outline is farther than that from the finest's. It takes over where that is a pixel: 1 cm at 27 m, 3 cm at 80 m. Each level has at most two thirds of the parts of the one before, and 1.5 times its error or more.
- `moves`: still, loose (how dense, does it float), sways, walks. All levels hang on the same rig.
- `put(p)`: `p.box`, `p.drum`, `p.ball`, `p.bar`, `p.add` in the object's own frame; `p.from` for what another maker made. Dice only from `rng(p.seed)`.

## What you make it of
A piece is a box, cut by up to 8 planes, one quadric (cylinder, cone, ball) and up to 3 curves (a car's). `materials.ts` has the materials; a `tint` is the root of what is sent back. Patterned ones (wood's planks, fabric's channels, tile, concrete) are laid by the metre in the world and are for rooms: on a small thing take a plain one (print, film, plastic, canvas, paint, gloss, steel, brass, rubber, crumb) and give it your color. `marks` print a picture of `art.ts` on a face.

## What coarser levels must keep, in this order
1. how bright it is and its color, as a whole: what goes takes its color with it, so give what stays the mean (of squared tints, by area seen)
2. how much of the picture it fills: five thin slats are two as wide as the five together; thinned leaves are larger
3. its outline, within its error; its shadow's holes
4. highlights: a lost bend of a surface is roughness
