// THE FLOODLIT YARD IN THE DEFAULT LOOK (src/scene/look.ts): tools/levels/floodlit.ts's level, box for box, in other clothes — tools/levels/gwotSkin.ts says
// what may change (paint and finish, decals, the paint of a prop) and checks that nothing a round, a charge or a blast reads did.
// `bun run tools/levels/export.ts floodlit` writes both files.
// ITS OWN PLACE: the fuel post of a bush airstrip — blockwork behind a block wall, black steel frames, and aviation ORANGE, on the ground only. What it alone
// has: A LANDING PAD. The yard the floodlights were hung for is a helicopter pad: the ring and its H in the middle of the open ground between the pickup and
// the fuel drums, approach chevrons in through the gate, a wind tee in the dead east end. The pad says, from above, where the open ground is — the eleven
// metres the level is about — and it lies the same whichever way the team comes in. No letters or numbers at doors, no bays, no lanes.
import { build as buildPlain, NOTES as PLAIN_NOTES } from './floodlit';
import { GWOT_MATERIALS } from '../../src/scene/gwotKit';
import { GWOT_PALETTE as P } from '../../src/scene/gwotPalette';
import { skinLevel, LOOK_NOTE, type Skin } from './gwotSkin';
import { lookMarks } from './lookMarks';
import type { Level } from '../../src/scene/level';
import type { Vec3 } from '../../src/math/vec';
import { saidAs } from '../../src/scene/stuff';

export const NAME = 'floodlit-gwot';
export const DRESS_AS = 'floodlit';
export const MATERIALS = GWOT_MATERIALS;
export const NOTES = `${PLAIN_NOTES ?? ''}${LOOK_NOTE}`;
export const build = buildPlain;
/** the pad's paint. The palette (src/scene/gwotPalette.ts) has no paint of this hue, so it is a colour of this file's own and only decals wear it */
const ORANGE: Vec3 = [0.74, 0.2, 0.03];
/** the middle of the pad: the open yard between the pickup (x ≤ 13.4) and the drums (x ≥ 26.7), on the gate's line */
const PAD: [number, number] = [20.0, 18.6], PAD_R = 3.2;
const SKIN: Skin = {
  wall: 'block', house: 'render', inner: 'whitewash', frameOut: 'iron', frameIn: 'woodGrey', dado: null,
  floor: { lino: 'slab' }, noRug: /./, metals: ['olive', 'galv', 'iron', 'rust'], patches: 0,   // (no patches: one would lie over the pad)
  // floodlit.ts's own colours as the stuff they are: the yard's packed earth is GRAVEL (a second ground inside the wall: the dust stops at the gate), the fuel
  // drums and the generator set wear their paint, the sandbags under the eaves are sandbags
  own: [[[0.34, 0.3, 0.24], 'gravel'], [[0.2, 0.3, 0.42], 'drumBlue'], [[0.5, 0.45, 0.33], 'sandbag'], [[0.5, 0.42, 0.1], 'genYellow'], [[0.5, 0.47, 0.4], 'truckTan']],
  dress: (G, _level, T) => {
    const M = lookMarks(T), white = saidAs('paint', T.tone(P.plasticWhite, 0.9)), [cx, cz] = PAD;   // (road paint in the plastic's white: a colour borrowed, not the stuff)
    // THE GROUND A FUEL POST WEARS, laid before the pad's paint so the paint lies on it. WHEELS: the track along the outside of the south wall that the team
    // came up, in at the gate, and round to where the pickup stands. FEET: the beat from the front door round the pad to the gate (nobody walks across a
    // pad), the way in from the foot gate, and outside the wall the walk from the track to the shed's door. FUEL: what the drums have dripped. Each is its
    // own ground darker or a second finish; the pad itself stays clean.
    { const lot = T.floorAt(20, 26.6) ?? P.dust, yard = T.floorAt(8, 20) ?? P.gravel;
      G.ruts(0.6, 26.5, 39.4, 26.5); G.ruts(20.0, 26.0, 20.0, 24.3);
      /** a beat feet have worn: its own ground darker down its middle, with a fainter shoulder either side (no edge to read as a painted line) */
      const beat = (pts: [number, number][], ground: Vec3) => { M.path(pts, 1.2, T.tone(ground, 0.9)); M.path(pts, 0.5, T.tone(ground, 0.78)); };
      G.ruts(18.3, 23.0, 12.6, 22.9);
      beat([[18.4, 13.7], [16.0, 15.6], [16.0, 21.6], [18.2, 23.3]], yard);
      beat([[19.6, 13.7], [24.0, 15.6], [24.0, 21.6], [21.8, 23.3]], yard);
      beat([[4.6, 21.8], [9.0, 22.0]], yard);
      beat([[8.6, 25.3], [3.0, 25.3], [3.0, 20.1]], lot);
      G.patch(27.3, 18.6, 2.0, 1.5, 'oil', 0.3); G.patch(26.2, 19.3, 1.0, 0.6, 'oil', -0.4); G.patch(11.0, 17.0, 1.1, 0.8, 'oil', 0.2); }
    // THE RING: an octagon, the way a pad is painted with a roller and a string
    { const pts: [number, number][] = []; for (let i = 0; i <= 8; i++) { const a = Math.PI / 8 + i * Math.PI / 4; pts.push([cx + Math.cos(a) * PAD_R, cz + Math.sin(a) * PAD_R]); } M.path(pts, 0.16, ORANGE); }
    // THE H, read from the gate
    for (const s of [-1, 1]) M.seg(cx + s * 0.8, cz - 1.25, cx + s * 0.8, cz + 1.25, 0.3, white);
    M.seg(cx - 0.8, cz, cx + 0.8, cz, 0.3, white);
    // THE APPROACH: two chevrons in from the gate, pointing at the pad
    for (const z of [23.1, 22.5]) { M.seg(cx - 0.9, z + 0.45, cx, z, 0.14, ORANGE); M.seg(cx + 0.9, z + 0.45, cx, z, 0.14, ORANGE); }
    // THE WIND TEE in the dead east end of the yard: a T on the ground, its stem down the prevailing wind
    { const x = 32.4, z = 20.6; M.seg(x - 0.9, z - 0.8, x + 0.9, z - 0.8, 0.22, white); M.seg(x, z - 0.8, x, z + 1.0, 0.22, white); }
  },
};
export const finish = (level: Level, groups: string[]) => skinLevel(level, groups, SKIN);
