# How to make an object

You make one object, alone: a small program in `src/objects/<name>.ts` that puts its levels into a world. Nothing is downloaded. The bench says whether it is good; look at its page too, because no number sees ugly.

1. Copy the one most like yours: `chair.ts` (pieces), `car.ts` (curved pieces), `ficus.ts` (leaves), `shelf.ts` (clutter), `person.ts` (a rig), `gravel.ts` (a skin of the floor). Add it to `index.ts`.
2. `bun run tools/sketch.ts <name>` (seconds, no GPU): a picture of its form, a row a level. Make it look right here first.
3. `bun run tools/sketch.ts --swap <name>` (seconds): each coarser level where it takes over, from two sides, beside the finest, and a picture of where they part. Over 70 cases its outline is the card's (within 0.06 px) and so is how much is filled (within 1.7 %); brightness is within 0.02 stops in the middle, 0.09 for nine of ten, 0.25 at worst (print, crevices); color within 0.7 % for nine of ten. Highlights, lamps and kept light only the card knows.
4. `bun run tools/objects.ts <name>` (a second, no GPU): does it stand, is it whole, does it live as it says. `ALL=1` shows what is right too.
5. `node tools/object.cjs <name>` on a card (two to four minutes): `out/<name>.jpg`, its page, and every mark. `--parts look` (or seen, swap, calm, alive, print, crowd) does only that. Without a card, `--quick` (ten minutes, small and grainy).
6. Mend, again, until every mark is reached and it looks right beside the reference, close by, and from the game's camera.

## What you write (`src/objects/object.ts`)
- `size`, as it truly is; its front to +z; it stands on y = 0. `close`: a place worth looking at near by. `read`: how far off people read what is printed on it (0: nothing is).
- `levels`, the finest first, two or three coarser. Each is whole and is drawn alone. `error`, in metres: no part of the outline is farther than that from the finest's. It takes over where that is a pixel of the game's picture: 1 cm at 27 m, 3 cm at 80 m. Each has at most two thirds of the parts of the one before, and 1.5 times its error or more.
- `moves`: still, loose (how dense it is, does it float: it is dropped, pushed and thrown into water), sways (leaves), walks, works (parts on joints). All levels move as the finest does.
- `put(p)`: `p.box`, `p.drum`, `p.ball`, `p.bar`, `p.add` in the object's own frame; `p.from` for what another maker made. Dice only from `rng(p.seed)`. A joint: `const lid = p.joint([x, y, z], () => { p.box(…) })`, and `put` returns `{ step: (dt, t) => { lid.rot = rotX(0.5 * Math.sin(t)); p.pose(); } }`; a joint made inside another's hangs on it.

## What you make it of
A piece (`src/scene/world.ts`) is a box, cut by up to 8 planes, one quadric (cylinder, cone, ball) and up to 3 curves. `src/scene/materials.ts` has the materials; a `tint` is the root of what is sent back. Patterned ones (wood's planks, fabric's channels, tile, stone, concrete) are laid by the metre and are for rooms: on a small thing take a plain one (print, film, plastic, canvas, paint, gloss, steel, brass, chrome, rubber, crumb, lamp shade) and give it your color. `marks` print a picture of `src/scene/art.ts` on a face. Leaves: `src/scene/plants.ts`.

Two faces never share a plane, the same way out, where both can be seen: make one a little larger. A scene parts all that do, seen or not, by widening one (`settle` in src/scene/faces.ts), and the next, along a chain: `rule.parted` says how far yours would be. Light is kept in cells of 0.32 m: low sun on the faces of a cell leaks a little into gaps finer than that, so pale and deep crevices come out too light. At the same albedo rough materials (plaster, concrete, fabric, cloth) send back a little more toward the light than smooth ones: match levels by the bench, not by the number. A wide ray does not meet pieces under 14 cm (it sees what lies behind them): a small effect, but give what lies behind the mean color of what stands before it. What is printed comes out paler than in the atlas.

## What coarser levels must keep, in this order
1. how bright it is and its color, as a whole: what goes takes its color with it, so give what stays the mean (of squared tints, by area seen)
2. how much of the picture it fills: five thin slats are two as wide as the five together; thinned leaves are larger
3. its outline, within its error; its shadow's holes
4. highlights: a lost bend of a surface is roughness
