// THE MOUNTAIN of the snow levels (the pass road, the tunnel station): a block of the cut rock with snow lying on its top, and the scree below a road. One
// piece and one paint for every generator that stands a road under a mountain, so the rock is one stuff in all of them.
//
// ITS PAINT is the palette's slab, a step darker (GWOT_PALETTE.slab × ROCK_SHADE). A row darkened in one piece is how a level writes shade, and a tone is made
// of what its row is made of (src/scene/stuff.ts tonedAs: it keeps the row's word): slab is drawn in big faces with joints and cracks, which on a
// face four to eight metres tall is cut rock. Block was tried and is courses of masonry, gravel is a wall of cobbles. The rock was a colour of its own
// before ([0.2, 0.21, 0.24]) and was drawn flat: a sixth of the pass road's picture and a tenth of the station's.
// TO A ROUND AND A BLAST it is what it was: a paint with no name, concrete by its tone (game/particleFx.ts surfOf: neither pale nor dark). ROCK_SHADE keeps
// it there, and a throw below holds that.
import type { Vec3 } from '../../src/math/vec';
import type { Box } from '../../src/scene/boxes';
import { GWOT_PALETTE, gwotSimOf, type GwotMat } from '../../src/scene/gwotPalette';
import { tonedAs } from '../../src/scene/stuff';
import { WORLD } from '../../src/scene/world';

/** a palette row darker in one piece, as the kit's own `toned` writes it (src/scene/gwotKit.ts: the same band, the same rounding, the same refusal of a tone
 *  that is another row's colour). The kit does not hand that function out; when it does, this is it */
function toned(m: GwotMat, k: number): Vec3 {
  if (!(k > 0.55 && k < 0.99)) throw new Error(`snowRock: a tone of ${m} at ${k} is outside 0.55‥0.99`);
  const p = GWOT_PALETTE[m], a = p.map(v => Math.round(v * k * 1e4) / 1e4) as Vec3;
  for (const n of Object.keys(GWOT_PALETTE) as GwotMat[]) { const q = GWOT_PALETTE[n]; if (n !== m && a.every((v, c) => Math.abs(v - q[c]) < 0.006)) throw new Error(`snowRock: ${m} × ${k} is the colour of ${n}`); }
  if (gwotSimOf(a) !== undefined) throw new Error(`snowRock: ${m} × ${k} is a paint the simulation knows by name`);
  return tonedAs(p, a);
}
const tone = (a: Vec3) => 0.2126 * a[0] + 0.7152 * a[1] + 0.0722 * a[2];

/** how far the mountain's slab is darkened: the rock stands a clear step under the snow on it and over the night's shadow at its foot */
export const ROCK_SHADE = 0.72;
export const ROCK_PAINT = toned('slab', ROCK_SHADE);
if (tone(ROCK_PAINT) <= 0.1 || tone(ROCK_PAINT) >= 0.5) throw new Error('snowRock: the rock\'s paint must stay concrete by its tone');

/** THE SLOPE BELOW A ROAD, where the snow does not lie: scree, the palette's gravel darkened (the look draws it as stones; it was one sheet in a colour of its
 *  own, [0.035, 0.045, 0.06], and drew flat: a sixth of the pass road's picture). Dark on purpose: the drop is the quiet side of every picture of the road.
 *  To a round it is what that sheet was, a paint with no name that is dark by its tone, and the throw holds that too */
export const SCREE_SHADE = 0.6;
export const SCREE_PAINT = toned('gravel', SCREE_SHADE);
if (tone(SCREE_PAINT) >= 0.1) throw new Error('snowRock: the scree\'s paint must stay dark by its tone');

type Add = (b: Partial<Box> & { c: Vec3; h: Vec3 }) => unknown;
/** a cap of snow on the top of a thing: the generator's own (x, z, width, depth, the top's height, yaw, thickness) */
type Cap = (x: number, z: number, w: number, d: number, y: number, yaw?: number, th?: number) => unknown;

/** THE MOUNTAIN: `rock(x, z, w, d, h, yaw)` stands a block of the cut rock `h` tall, snow lying on its top (set in from its edges: the rock shows as a dark
 *  rim). A turned block is kept whole inside the world: past its edge nothing is traced or lit */
export function rockOf(add: Add, cap: Cap, rough: number) {
  return (x0: number, z0: number, w: number, d: number, h: number, yaw = 0) => {
    const ex = (Math.abs(w * Math.cos(yaw)) + Math.abs(d * Math.sin(yaw))) / 2 + 0.03, ez = (Math.abs(w * Math.sin(yaw)) + Math.abs(d * Math.cos(yaw))) / 2 + 0.03;
    const x = Math.min(WORLD.max[0] - ex, Math.max(WORLD.min[0] + ex, x0)), z = Math.min(WORLD.max[2] - ez, Math.max(WORLD.min[2] + ez, z0));
    add({ c: [x, h / 2, z], h: [w / 2, h / 2, d / 2], yaw, rough, albedo: ROCK_PAINT });
    cap(x - 0.04, z - 0.03, Math.max(0.2, w - 0.3), Math.max(0.2, d - 0.26), h, yaw, 0.07);
  };
}
