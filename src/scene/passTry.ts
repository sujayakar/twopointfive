// THE PASS DRESSED WITH ROUND THINGS, A TRIAL (the look page only: main.ts passTryOn; ?dress=0 = the pass as its file has it). Asked 2026-10-29: how far can
// round things go through a level, and "dressing up the pass level we have would be another good experiment". Gone over by hand after the level loads, as the
// lorry is (scene/lorryTry.ts; levels/pass.json is not touched):
//   · a stack of three steel PIPES under snow along the bunk room's wall;
//   · four fuel DRUMS in the pocket of yard by the lean-to, one on its side;
//   · LOGS by the chopping block, two more on the wood, and the block itself round;
//   · a loose TYRE by the cart.
// (The file's low heaps of snow were rounded over too at first: the page's snow surface already draws a smooth hill over every one, so nothing of it showed.)
// Made of what the engine has: a round prism of twelve sides with a turning normal (Box.cutRound: about the box's own y for what stands, about its own x for
// what lies) and an ARC (Box.cutArc) for a top that is part of a wide round. NOT Box.round: on this page, with a geometry trial on, the GPU's own round test is
// compiled out (main.ts roundConfig.gpu) and such a box is drawn as the square box round it, which is how the first stills showed the drums.
// ONE MODEL: each new piece is a real box to every ray (which meets the prism) and to a body (which meets the box round it). The snow's arcs are the file's own
// boxes with their tops cut: a body meets them as before, a ray meets the arc. So sight, rounds and the walk grid change on this page where a new piece
// stands, and nowhere else. Never seen on a GPU; what it costs a frame has not been timed anywhere.
import { BoxFlag, type Box } from './boxes';
import { cutRecord } from './cut';
import type { Vec3 } from '../math/vec';
import { GWOT_PALETTE, SNOW_PALETTE, gwotAs } from './gwotPalette';
import { ring, arc } from './lorryTry';

type More = Partial<Box>;
const P = GWOT_PALETTE, SNOW = SNOW_PALETTE.snow, DECAL = BoxFlag.NoShadow;
const of = (row: Vec3, k: number): Vec3 => [row[0] * k, row[1] * k, row[2] * k];
const mk = (c: Vec3, h: Vec3, yaw: number, albedo: Vec3, o: More = {}): Box => ({ c, h, yaw, albedo, emissive: [0, 0, 0], flags: 0, owner: 0, cutHeight: 1000000, rough: 0.6, ...o });
/** what stands: a round of radius r from y0 to y1 */
const upright = (b: Box, r: number): Box => { b.cut = cutRecord(b.h, ring(r, 2)).cut; b.cutRound = 2; return b; };
/** … as a prism of twelve sides with a turning normal (scene/geoTry.ts roundStacks' way: on the look page with a geometry trial on the GPU's own round test is
 *  compiled out, main.ts roundConfig.gpu, and a `Box.round` is drawn as the square box round it) */
const stand = (x: number, z: number, y0: number, y1: number, r: number, albedo: Vec3, o: More = {}): Box => upright(mk([x, (y0 + y1) / 2, z], [r, (y1 - y0) / 2, r], 0, albedo, o), r);
/** what lies: a round of radius r and length len about its own x, its middle at (x, y, z), its x turned `yaw` */
function lie(x: number, y: number, z: number, len: number, r: number, yaw: number, albedo: Vec3, o: More = {}): Box {
  const b = mk([x, y, z], [len / 2, r, r], yaw, albedo, o); b.cut = cutRecord(b.h, ring(r, 1)).cut; b.cutRound = 1; return b;
}
/** the snow lying along the top of a round that lies: an arc `t` thick over a round of radius r whose middle is at (x, y, z), covering `share` of its width */
function snowOn(x: number, y: number, z: number, len: number, r: number, yaw: number, t = 0.035, share = 0.72): Box {
  const R = r + t, w = R * share, y0 = y + Math.sqrt(Math.max(R * R - w * w, 0)), y1 = y + R;
  return arc(mk([x, (y0 + y1) / 2, z], [len / 2, (y1 - y0) / 2, w], yaw, SNOW, { rough: 1, flags: DECAL }), R, 1, Math.asin(Math.min(1, w / R)) * 180 / Math.PI / 3);
}

/** dress the pass (see the head). Returns how many boxes it added: 0, and nothing touched, on a level that is not the pass. */
export function dressPass(level: { boxes: Box[] }): number {
  const B = level.boxes, near = (b: Box, x: number, y: number, z: number) => Math.abs(b.c[0] - x) < 0.02 && Math.abs(b.c[1] - y) < 0.02 && Math.abs(b.c[2] - z) < 0.02;
  const block = B.find(b => near(b, 22.9, 0.16, 12.5)); if (!block) return 0;   // (the chopping block: not there = not the pass this file knows)
  const out: Box[] = [], HALF = Math.PI / 2;

  // ---- pipes along the bunk room's wall, south of its door: two on the ground and one in the notch, snow along each top, their bores dark ----
  { const STEEL = P.iron, RUSTY = P.rust, r = 0.16, len = 1.8, z = 15.75;   // (the palette's own rows, as they are: a row darkened under 0.55 of itself is no row to the look's material maps or to what a round throws off it; bright galvanised steel under a noon sun read as plastic)
    for (const [x, y, col] of [[27.66, r, STEEL], [27.33, r, RUSTY], [27.495, r + 0.277, STEEL]] as const) {
      out.push(lie(x, y, z, len, r, HALF, col, { rough: 0.6, metal: true }));
      for (const s of [-1, 1]) out.push(lie(x, y, z + s * (len / 2 + 0.003), 0.006, r * 0.8, HALF, of(P.iron, 0.6), { rough: 0.9, flags: DECAL }));
    }
    out.push(snowOn(27.495, r + 0.277, z, len - 0.06, r, HALF, 0.05)); out.push(snowOn(27.66, r, z, len - 0.1, r, HALF, 0.03, 0.45)); out.push(snowOn(27.33, r, z, len - 0.1, r, HALF, 0.03, 0.45)); }

  // ---- fuel drums in the pocket of yard between the lean-to and the bay: three standing, one on its side ----
  for (const [x, z, col] of [[7.7, 9.85, P.rust], [8.36, 9.72, P.drumBlue], [8.02, 10.4, gwotAs('olive', 'metal')]] as const) out.push(stand(x, z, 0, 0.88, 0.29, col, { rough: 0.6 }));   // (olive is dark wood to a round by the palette: this one is steel in that paint. No disc of snow on a lid: the cut pass gives a round's own end a drum's dark rim and no snow finish, and the look lays its own snow there)
  out.push(lie(9.15, 0.29, 10.75, 0.88, 0.29, 0.5, P.drumRed, { rough: 0.6 })); out.push(snowOn(9.15, 0.29, 10.75, 0.8, 0.29, 0.5, 0.04, 0.6));

  // ---- the wood: the block is round, logs lie by it waiting to be split, and two lie along the top of the pile ----
  block.h = [0.22, block.h[1], 0.22]; block.yaw = 0; delete block.marks; upright(block, 0.22);
  for (const [x, z, len, r, yaw, y] of [[23.69, 12.22, 1.0, 0.12, 0.25, 0.12], [23.84, 12.59, 0.9, 0.105, -0.2, 0.105], [23.2, 13.5, 1.05, 0.13, 0.6, 0.13], [23.76, 12.39, 0.85, 0.1, 0.05, 0.31]] as const) {
    out.push(lie(x, y, z, len, r, yaw, P.woodGrey, { rough: 0.9 }));
    for (const s of [-1, 1]) out.push(lie(x + s * (len / 2 + 0.003) * Math.cos(yaw), y, z - s * (len / 2 + 0.003) * Math.sin(yaw), 0.006, r * 0.9, yaw, P.poplar, { rough: 0.85, flags: DECAL }));
    if (y > 0.2 || r > 0.12) out.push(snowOn(x, y, z, len - 0.08, r, yaw, 0.03, 0.55));
  }
  for (const z of [11.39, 11.56]) out.push(lie(22.1, 0.42 + 0.07, z, 1.3, 0.07, 0, P.woodGrey, { rough: 0.9 }));

  // ---- a loose tyre by the cart, flat in the snow ----
  out.push(stand(15.3, 20.75, 0, 0.2, 0.33, P.rubber, { rough: 0.85 })); out.push(stand(15.3, 20.75, 0.2, 0.203, 0.17, of(P.rubber, 0.6), { rough: 1, flags: DECAL }));

  for (const b of out) B.push(b);
  return out.length;
}
