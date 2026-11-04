// THE LOOK PROTOTYPE — THE GROUND RUNS ON PAST THE LEVEL'S EDGE (a word of the look page's address, `&beyond=1`; OFF unless asked). Every level stands on a
// slab the size of the world, and past its edge an eye's ray meets nothing: black (render/shaders/composite.wgsl, the maintainer's own word, 10-15: "we should
// just have black outside the edges of the map, not extend"). At night that reads as the dark. Under a midday sun the same edge is a black wall a few metres
// from where the team starts, a quarter to a half of the opening frame on a level whose start is at its rim (seen on a real GPU, 11-04).
// With the word, the level's own ground slab is laid once more, a hair under itself and BEYOND.reach metres wider on every side, as a box no ray ever meets
// (BoxFlag.NoShadow): the picture draws it as ground, in the slab's own paint and stuff, lit by the hour's sun and the sky; nothing in the simulation, the
// bounce or a shadow can depend on it (no ray, sweep or collision cell returns it), and inside the world the slab itself covers it. IT IS THE SLAB, DRAWN
// WIDER: the page holds the pair and at every pack of the world for a picture (scene/boxes.ts beforePack) the box takes on what the slab looks like now
// (`groundBeyondFollow`: its paint, the rain's wet and its drying, its gloss and finish), so the rain needs no word about this box; what looks for the
// level's ground by its size passes it over (render/snowField.ts: a box no ray meets). What is a MAP of the level is let go of past the level's edge: the
// readers' own rule, render/frame.ts MAP_EDGE. A level with no slab the size of the world is left as it is.
// IT IS A BOX OF THE LEVEL AS STOOD UP, like the look's trials (scene/geoTry.ts): with the word a checkpoint or a recording is that visit's own.
// The check: tools/qa/groundbeyond-check.ts. Never seen on a GPU.
import { BoxFlag, makeBox, wordOf, type Box } from './boxes';
import { WORLD } from './world';

export const BEYOND = {
  reach: 160,     // m past the world's edge on every side: the camera's far plane is 120 m from the eye (game/camera.ts), so no view ends on this box's own edge
  drop: 0.002,    // m under the slab's own top. Small enough that it is the ground to everything that asks by height (the g-buffer's snow and ground layers
                  // and the rain's ground test take a top over −1 cm; the look deals a face its pieces by its height in whole centimetres, look.wgsl `plane`,
                  // and 3 mm down is already another deal), and that its lip at the rim is a tenth of a pixel; large enough for the depth buffer where the
                  // two overlap, inside the world (a float, near plane 0.5 m: half a millimetre apart is told apart at 60 m)
  cover: 0.999,   // the slab is the level's ground when it covers the world's rectangle (this share of it: a slab that stops short is no ground to run on) …
  top: 0.05,      // … with its top within this of the world's floor (y = 0)
  name: 'ground_beyond',
};

/** the level's own ground: the plain box (not turned, not cut, drawn and met by rays) whose top is the world's floor and that covers the world's rectangle;
 *  of two, the one whose top is higher (the sheet that is seen, not the base under it). Null = this level has none */
export function groundSlab(boxes: readonly Box[]): Box | null {
  const x0 = WORLD.min[0], x1 = WORLD.max[0], z0 = WORLD.min[2], z1 = WORLD.max[2], area = (x1 - x0) * (z1 - z0);
  let best: Box | null = null;
  for (const b of boxes) {
    if (b.flags & (BoxFlag.NoPrimary | BoxFlag.NoShadow | BoxFlag.Dynamic | BoxFlag.Glass | BoxFlag.Cutaway) || b.rot || b.cut?.length || Math.abs(Math.sin(2 * b.yaw)) > 1e-6) continue;
    if (Math.abs(b.c[1] + b.h[1]) > BEYOND.top) continue;
    const turned = Math.abs(Math.sin(b.yaw)) > 0.5, hx = turned ? b.h[2] : b.h[0], hz = turned ? b.h[0] : b.h[2];
    const w = Math.max(0, Math.min(x1, b.c[0] + hx) - Math.max(x0, b.c[0] - hx)), d = Math.max(0, Math.min(z1, b.c[2] + hz) - Math.max(z0, b.c[2] - hz));
    if (w * d / area >= BEYOND.cover && (!best || b.c[1] + b.h[1] > best.c[1] + best.h[1])) best = b;
  }
  return best;
}

/** the laid ground and the slab it is the run of */
export interface GroundBeyond { box: Box; slab: Box }
/** lay the ground past the level's edge (see the head): one box added at the end of `level.boxes`, or nothing. → the pair, or null where the level has no ground slab */
export function groundBeyond(level: { boxes: Box[] }): GroundBeyond | null {
  const g = groundSlab(level.boxes); if (!g) return null;
  const cx = (WORLD.min[0] + WORLD.max[0]) / 2, cz = (WORLD.min[2] + WORLD.max[2]) / 2, hx = (WORLD.max[0] - WORLD.min[0]) / 2 + BEYOND.reach, hz = (WORLD.max[2] - WORLD.min[2]) / 2 + BEYOND.reach;
  const b = makeBox({ c: [cx, g.c[1] - BEYOND.drop, cz], h: [hx, g.h[1], hz], albedo: [0, 0, 0], ...wordOf(g), flags: BoxFlag.NoShadow, name: BEYOND.name, ...(g.metal ? { metal: true } : {}) });
  const pair = { box: b, slab: g }; groundBeyondFollow(pair); level.boxes.push(b); return pair;
}
/** THE BOX LOOKS AS THE SLAB LOOKS NOW — picture only, called at the head of every pack of the world (main.ts hangs it on scene/boxes.ts beforePack, so a
 *  frame, a still and a replay's picture are all packed with it done; after the frame's ticks, whoever draws). FIVE THINGS
 *  are copied over: the slab's paint (by value), the render-only wet factor, its gloss, its roughness and its stretched lobe; the box stays one no ray
 *  meets. That is all the rain writes (game/weather.ts setWet, soaking and drying), so the wet lot reaches the ground past the edge in the same frame
 *  with no word about this box in the weather. Its stuff and metal are the slab's as laid and are not followed; a change of those wants a line here.
 *  Whatever else has written the box's own look is overwritten: under a picked rain the weather's sweep of open slabs writes this box too (and keeps a
 *  row to put it back by), which is why the follow is last. The box is in the level's list, so these fields are in a rollback snapshot like any box's:
 *  they are a function of the slab's, written again at every pack (scene/lookTidy.ts is the same kind of thing). */
export function groundBeyondFollow(p: GroundBeyond): void {
  const b = p.box, g = p.slab;
  b.albedo[0] = g.albedo[0]; b.albedo[1] = g.albedo[1]; b.albedo[2] = g.albedo[2];
  if (g.wetK === undefined) delete b.wetK; else b.wetK = g.wetK;
  if (g.rough === undefined) delete b.rough; else b.rough = g.rough;
  if (g.aniso === undefined) delete b.aniso; else b.aniso = g.aniso;
  b.flags = BoxFlag.NoShadow | (g.flags & BoxFlag.Gloss);
}
