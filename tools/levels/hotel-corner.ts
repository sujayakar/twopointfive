// HOTEL-CORNER: ONE CORNER OF A MINIMALIST HOTEL, a look test and nothing else (not a level to play, in no mission list). The second laying, after the
// first was seen on a real card and the maintainer sent photographs of the kind of place he means: a house whose colonnade FACES THE SUN, a pool whose pale
// shallow water lies FLUSH with the terrace, round columns on square plinths, smooth coursed stone against a rough garden wall, and timber as the one warm
// colour. NO ACCENT COLOUR. If this does not hold at play distance, nothing larger is built on it.
// THE RULES IT IS LAID BY (the maintainer's words on the white roof: "very tastefully done high frequency detail but overall quite flat", "it's all quite
// intentionally dressed"):
//   THE PLAN STANDS ON THE PAVER'S OWN GRID   pavers are half a metre and are laid in bays of three metres by where they are in the WORLD (render/lookTex.ts),
//                  so every column stands on a bay's joint, the pool's edges on a paver's, and the building sets out the floor without a box being laid
//   FOUR VALUES    pale paving underfoot, a darker smooth stone under the colonnade and in every band, dark planting and bronze, and the water
//   EDGES          no two fields butt: a flush band, a dark slot or a step lies between them
//   TWO STONES     the house is smooth and coursed, the garden wall is rough; the floor is jointed, so nothing is written or bolted on a wall
//   MARKS BY CAUSE a few wet prints from the pool's steps to the daybed, the edge dark where water is carried over it, a strip polished through the middle bay
//   THE COLONNADE FACES THE SUN   the first laying's porch faced away from it and was a field of blue shade, half the picture. This one is open to the sky
//                  between its rafters: the sun lies on its floor and its wall, and the rafters' shadows cross both
//   WATER WITHOUT WATER   the pool is a flat glossy plane a man could walk on, pale and level with the paving, its steps drawn under it and three slabs
//                  across it: what water needs from this camera is the graphics work's question
// What this file lays it lays in the coast palette's quiet rows (src/scene/gwotPalette.ts). Its helpers (film, mass, the frame a daybed is laid in) are
// this draft's own copies of the white roof study's; they are owed to the kit once.
//   bun run tools/levels/export.ts hotel-corner [--check]     → levels/hotel-corner.json
//   page: ?look&solo&levelFile=hotel-corner.json&hour=midday
import type { Level, LevelDoorDef } from '../../src/scene/level';
import type { LevelScenarios, Rect } from '../../src/scene/scenarios';
import type { SurfacesData } from '../../src/scene/levelData';
import type { Vec3 } from '../../src/math/vec';
import { officeKit, normalize3 as norm, type GeneratorTrace } from '../../src/scene/officeGen';
import { gwotKit, gwotOfOffice } from '../../src/scene/gwotKit';
import { COAST_PALETTE, COAST_FINISH, COAST_MATERIALS, type CoastMat } from '../../src/scene/gwotPalette';
import { WORLD } from '../../src/scene/world';
import { BoxFlag, type Box } from '../../src/scene/boxes';
import { PANEL_GREY, tonedAs } from '../../src/scene/stuff';
import { type MarkPal } from '../../src/scene/markings';
import * as MD from '../../src/scene/markingDefaults';
import { coastHost, COAST_QUIET_OF } from './coastSkin';

export const NAME = 'hotel-corner';
export const NOTES = 'A look test: one corner of a minimalist hotel (a colonnade that faces the sun, pale water level with the paving, one daybed, one cypress), no accent colour. The pool is a flat plane, not water. Not in the missions list. Regenerate: bun run tools/levels/export.ts hotel-corner';
/** a hotel is cleaned every morning (the file's meta.kept: docs/level-format.md) */
export const KEPT = 1;
export const MATERIALS = COAST_MATERIALS;
/** the quiet way, and this house's own choice in it: its walls are the coursed stone, where the quiet way keeps cast panels for the service side */
const HOUSE_OF = { ...COAST_QUIET_OF, block: 'limestone' } as const;

export function build(): { level: Level; scenarios: LevelScenarios; groups: string[]; surfaces: SurfacesData } {
  const trace: GeneratorTrace = { groups: [] };
  const K = officeKit(trace), G = gwotKit(coastHost(gwotOfOffice(K), HOUSE_OF)), C = COAST_PALETTE, H = WORLD.ceilingY;
  const { grp, add, slab, doorFrame, lights, switchables } = K;
  const HALF = Math.PI / 2, FLAT = BoxFlag.NoShadow, r4 = (v: number) => Math.round(v * 1e4) / 1e4;
  /** a row's paint (the row's own array) or a tone of it, which keeps its word; and its finish */
  const tones = new Map<string, Vec3>();
  const P = (m: CoastMat, t = 1): Vec3 => { const key = `${m}:${t}`; let a = tones.get(key); if (!a) { a = t === 1 ? C[m] : tonedAs(C[m], C[m].map(v => r4(Math.min(0.95, v * t))) as Vec3); tones.set(key, a); } return a; };
  const row = (m: CoastMat) => ({ rough: COAST_FINISH[m].rough, ...(COAST_FINISH[m].metal ? { metal: true } : {}) });
  /** a film of one row on the floor: drawn and lit, throwing no shadow. `t`: a tone of the row; `more`: its own finish or flags */
  const film = (x0: number, z0: number, x1: number, z1: number, m: CoastMat, y = 0.008, t = 1, more: Partial<Box> = {}) => slab(x0, y - 0.004, z0, x1, y, z1, P(m, t), { ...row(m), flags: FLAT, ...more });
  /** a block of one row */
  const mass = (x0: number, y0: number, z0: number, x1: number, y1: number, z1: number, m: CoastMat, t = 1) => slab(x0, y0, z0, x1, y1, z1, P(m, t), row(m));
  /** the frame of a thing standing at (x, z) turned by yaw: a box by its own length (lx) and width (lz) */
  const at = (x: number, z: number, yaw: number) => { const cs = Math.cos(yaw), sn = Math.sin(yaw);
    return (lx: number, lz: number, y0: number, y1: number, hx: number, hz: number, m: CoastMat, t = 1, more: Partial<Box> = {}) => add({ c: [x + cs * lx + sn * lz, (y0 + y1) / 2, z - sn * lx + cs * lz], h: [hx, (y1 - y0) / 2, hz], yaw, albedo: P(m, t), ...row(m), ...more }); };
  /** a number that is the same every build for the same place: what is dealt (a post's turn, a print's fade) is dealt by where it stands */
  const dealt = (a: number, b: number) => { const s = Math.sin(a * 127.1 + b * 311.7) * 43758.5453; return s - Math.floor(s); };

  /** A DAYBED: a timber plinth 2.0 by 0.9 floating on a recessed dark foot, a linen mattress set in from its edge, one bolster */
  const daybed = (x: number, z: number, yaw: number) => { const b = at(x, z, yaw);
    b(0, 0, 0, 0.08, 0.92, 0.37, 'bronze'); b(0, 0, 0.08, 0.25, 1.0, 0.45, 'timber'); b(-0.02, 0, 0.25, 0.37, 0.93, 0.4, 'canvasWhite'); b(0.72, 0, 0.37, 0.47, 0.14, 0.36, 'canvasWhite', 0.94); };
  /** A CYPRESS (the maintainer's picture, 11-05: tall narrow spires in a line): a trunk, a core and eight upright posts turned every way, their tops
   *  staggered, one thin tip. Upright strokes, a ragged taper, a dark side: not rings. Solid: a man can stand behind it. Its green is flat (no leaves yet) */
  const cypress = (x: number, z: number, tall: number) => {
    mass(x - 0.08, 0, z - 0.08, x + 0.08, 0.34, z + 0.08, 'timber');
    add({ c: [x, 0.3 + (tall * 0.94 - 0.3) / 2, z], h: [0.19, (tall * 0.94 - 0.3) / 2, 0.19], yaw: 0.4, albedo: P('cypress'), ...row('cypress') });
    for (let i = 0; i < 8; i++) { const a = i * Math.PI / 4 + dealt(x + i, z) * 0.5, r = 0.24 + dealt(x, z + i) * 0.1, top = tall * (0.45 + 0.45 * dealt(x * 3 + i, z * 5)), foot = 0.3 + dealt(i, x + z) * 0.25;
      add({ c: [x + Math.cos(a) * r, (foot + top) / 2, z + Math.sin(a) * r], h: [0.15, (top - foot) / 2, 0.15], yaw: dealt(i * 7, x - z) * HALF, albedo: P('cypress', 0.85 + 0.3 * dealt(x + i * 3, z - i)), ...row('cypress') }); }
    add({ c: [x, tall * 0.97, z], h: [0.1, tall * 0.03, 0.1], yaw: 0.2, albedo: P('cypress', 1.2), ...row('cypress') });
  };

  // ---- the terrace: one pale paving, wall to wall ----
  grp('ground');
  slab(0, -0.25, 0, 40, 0, 28, P('stonePale'), row('stonePale'));

  // ---- THE HOUSE: one long room behind a wall of smooth coursed stone, three openings that stand in the colonnade's bays, the middle one on the axis ----
  const WX0 = 12, WX1 = 27, WZ0 = 3, WZ1 = 9, AXIS = 19.5;
  grp('walls.house');
  G.wallX(WZ1, WX0 - 0.12, WX1 + 0.12, 'block', [{ a: 15.75, b: 17.25, y0: 0, y1: 2.7 }, { a: AXIS - 0.9, b: AXIS + 0.9, y0: 0, y1: 2.7 }, { a: 21.75, b: 23.25, y0: 0, y1: 2.7 }]);
  G.wallX(WZ0, WX0 - 0.12, WX1 + 0.12, 'block', [{ a: AXIS - 0.6, b: AXIS + 0.6 }]);
  G.wallZ(WX0, WZ0 + 0.12, WZ1 - 0.12, 'block'); G.wallZ(WX1, WZ0 + 0.12, WZ1 - 0.12, 'block', [{ a: 5.4, b: 6.6 }]);
  // a timber shutter folded back against the wall on each side of an opening: the one warm colour, and it is a thing somebody opens every morning
  for (const [a, b] of [[15.75, 17.25], [AXIS - 0.9, AXIS + 0.9], [21.75, 23.25]]) for (const x of [a - 0.4, b + 0.4]) mass(x - 0.36, 0.04, WZ1 + 0.125, x + 0.36, 2.66, WZ1 + 0.17, 'cedar');
  grp('floors.house');
  film(WX0 + 0.12, WZ0 + 0.12, WX1 - 0.12, WZ1 - 0.12, 'stoneShade', 0.008);
  grp('ceiling');
  slab(WX0 - 0.12, H, WZ0 - 0.12, WX1 + 0.12, H + 0.2, WZ1 + 0.12, P('skimWhite'), { flags: BoxFlag.NoPrimary, rough: COAST_FINISH.skimWhite.rough });
  grp('room.house');
  { const b = at(16.4, 4.6, 0); b(0, 0, 0, 0.08, 1.42, 0.32, 'bronze'); b(0, 0, 0.08, 1.02, 1.5, 0.4, 'timber'); for (let i = 0; i < 3; i++) b(-0.9, 0.05, 1.02 + i * 0.05, 1.07 + i * 0.05, 0.22, 0.16, 'canvasWhite', 1 - i * 0.03); }   // a long desk, three folded towels on its end

  // ---- THE COLONNADE: round columns on square plinths, one on every bay's joint; a beam; rafters of warm timber a metre apart with the sky between them.
  // It faces the sun: at noon the light lies on its floor and wall and the rafters rule both ----
  grp('colonnade');
  const CZ = 12, COLS = [12, 15, 18, 21, 24, 27];
  film(WX0 - 0.3, CZ - 0.3, WX1 + 0.3, CZ + 0.3, 'stoneBand', 0.008);                                       // the line the columns stand on, flush: the edge between the paving and the floor under the rafters
  film(WX0, WZ1 + 0.12, WX1, CZ - 0.3, 'stoneShade', 0.008);
  film(AXIS - 0.6, WZ0 + 0.12, AXIS + 0.6, CZ - 0.3, 'stoneShade', 0.011, 0.88, { rough: 0.22 });           // a strip polished by feet, from the front door through the middle opening to the columns
  for (const x of COLS) { mass(x - 0.3, 0, CZ - 0.3, x + 0.3, 0.14, CZ + 0.3, 'cream', 0.92); K.pipe(x, CZ, 0.14, 2.64, 0.22, P('cream'), [], true); for (const b of K.boxes.slice(-4)) Object.assign(b, row('cream')); mass(x - 0.27, 2.64, CZ - 0.27, x + 0.27, 2.72, CZ + 0.27, 'cream'); }
  mass(WX0 - 0.3, 2.72, CZ - 0.22, WX1 + 0.3, 3.02, CZ + 0.22, 'cream');
  for (let x = WX0; x <= WX1; x++) mass(x - 0.08, 3.02, WZ1 + 0.14, x + 0.08, 3.2, CZ + 0.4, 'cedar');
  // THE BUILDING SETS OUT THE FLOOR: a band of the darker stone from every column to the pool's edge
  grp('bands');
  const X0 = 15, X1 = 30, Z0 = 16, Z1 = 20.5, EDGE = 0.5;
  for (const x of COLS) film(x - 0.15, CZ + 0.3, x + 0.15, Z0 - EDGE, 'stoneBand', 0.008);

  // ---- THE POOL: pale shallow water level with the paving. A flush edge of the smooth stone, a dark slot the water runs into all round, three broad
  // steps drawn under the water at its west end, and three slabs across it on the house's axis ----
  grp('pool');
  const WATER: Vec3 = P('poolPale'), STEPS: Vec3[] = [[0.3, 0.42, 0.36], [0.25, 0.38, 0.33], [0.21, 0.345, 0.3]];
  slab(X0, 0.002, Z0, X1, 0.01, Z1, WATER, { rough: 0.06, flags: FLAT | BoxFlag.Gloss });
  STEPS.forEach((c, i) => slab(X0 + i * 0.5, 0.01, Z0, X0 + (i + 1) * 0.5, 0.014, Z1, c, { rough: 0.08, flags: FLAT | BoxFlag.Gloss }));
  const edge = film(X0 - EDGE, Z1, X1, Z1 + EDGE, 'stoneShade', 0.008); film(X0 - EDGE, Z0 - EDGE, X1, Z0, 'stoneShade', 0.008); film(X0 - EDGE, Z0, X0, Z1, 'stoneShade', 0.008);
  film(X0 - 0.04, Z0 - 0.04, X1, Z0, 'bronze', 0.012); film(X0 - 0.04, Z1, X1, Z1 + 0.04, 'bronze', 0.012); film(X0 - 0.04, Z0, X0, Z1, 'bronze', 0.012);   // the slot
  for (let i = 0; i < 3; i++) { const z = Z0 + 0.15 + i * 1.45; mass(AXIS - 0.75, 0, z, AXIS + 0.75, 0.06, z + 1.3, 'cream', 0.94); }
  // its depths, small on the edge where a swimmer reads them (never on the water)
  { const INK: MarkPal = { albedo: P('bronze'), rough: 0.45 }, b = K.boxes[edge]; b.marks = [...MD.stencilMarks('+y', '0.9', (X0 + 0.9) - b.c[0], b.c[2] - (Z1 + 0.3), INK, 1, 'base'), ...MD.stencilMarks('+y', '1.4', (X0 + 8.2) - b.c[0], b.c[2] - (Z1 + 0.3), INK, 1, 'base')]; }
  film(X0 - EDGE + 0.02, Z1 - 2.6, X0 - 0.05, Z1 - 0.9, 'stoneShade', 0.011, 0.72, { rough: 0.2 });         // the edge is dark and shines where somebody climbed out over it
  for (const z of [Z1 - 2.5, Z1 - 1.0]) mass(X0 - 0.3, 0, z - 0.035, X0 - 0.23, 0.95, z + 0.035, 'bronze');   // two square rails at the steps, thick enough to read

  // ---- THE SLOT DRAIN: one dark line the terrace's whole length, a lift-out length every six metres ----
  grp('drain');
  film(11.76, Z1 + 1.0, X1, Z1 + 1.1, 'bronze', 0.008);
  for (let x = 15; x < X1; x += 6) film(x, Z1 + 0.97, x + 0.6, Z1 + 1.13, 'stoneBand', 0.011);

  // ---- THE GARDEN WALL: rough stone to the house's smooth, a flat cope; along its foot a strip of gravel with clipped sage ----
  grp('garden');
  mass(10.5, 0, 6.0, 11.0, 1.1, 25.5, 'rubble'); mass(10.47, 1.1, 5.97, 11.03, 1.16, 25.53, 'stoneShade');
  mass(11.0, 0, 25.0, 16.6, 1.1, 25.5, 'rubble'); mass(11.03, 1.1, 24.97, 16.63, 1.16, 25.53, 'stoneShade', 0.97);
  film(11.0, CZ + 0.5, 11.7, 25.0, 'paleGravel', 0.008); film(11.7, CZ + 0.5, 11.76, 25.0, 'stoneBand', 0.011);
  for (let z = 14; z < 25; z += 2) mass(11.1, 0, z - 0.25, 11.6, 0.5, z + 0.25, 'sage', 0.9 + 0.2 * dealt(z, 3));

  // ---- one daybed beside the steps, pulled a little out of line; the prints of whoever climbed out and lay down on it ----
  grp('terrace');
  daybed(12.9, Z1 - 1.9, HALF + 0.06);
  for (let i = 0; i < 4; i++) { const u = i / 3, x = X0 - EDGE - 0.12 - u * 0.75, z = Z1 - 1.75 - u * 0.12, s = i % 2 ? 0.09 : -0.09;
    add({ c: [x, 0.003, z + s], h: [0.1, 0.003, 0.04], yaw: 0.16, albedo: P('stonePale', 0.74 + 0.2 * u), rough: 0.25 + 0.3 * u, flags: FLAT }); }

  // ---- one cypress in a square of dry earth cut in the paving, edged; it is placed by where its shadow falls ----
  grp('cypress');
  film(24.85, 13.25, 26.15, 14.55, 'stoneBand', 0.008); film(24.95, 13.35, 26.05, 14.45, 'dryEarth', 0.011);
  cypress(25.5, 13.9, 7.2);

  // ---- lights: two bulbs in the house, the moon (the page's hour sets the sun) ----
  grp('lights');
  G.bulb(16.5, 6.0, 'bulb_house_w', 'house', true); G.bulb(22.5, 6.0, 'bulb_house_e', 'house', true);
  lights.push({ kind: 'dir', pos: [0, 0, 0], dir: norm([0.45, -0.5, -0.74]), color: [0.55, 0.7, 1.0], intensity: 0.9, range: 1e6, innerDeg: 0, outerDeg: 0, name: 'moon', group: 'exterior', enabled: true, volumetric: 0, radius: 0.009, mains: false });

  // ---- the breaker and one emergency beacon: the game's blackout wants both ----
  grp('breaker');
  const brk = add({ c: [14.6, 1.35, WZ0 + 0.17], h: [0.28, 0.36, 0.04], albedo: PANEL_GREY, name: 'breaker_panel' });
  const brkLed = add({ c: [14.42, 1.62, WZ0 + 0.22], h: [0.015, 0.015, 0.01], albedo: C.plasticDark, rough: 0.5, emissive: [0.3, 4, 0.6], flags: BoxFlag.NoShadow });
  grp('beacons');
  add({ c: [AXIS, H - 0.06, 6], h: [0.07, 0.06, 0.07], albedo: [0.2, 0.2, 0.22], flags: BoxFlag.NoShadow | BoxFlag.NoPrimary });
  const dome = add({ c: [AXIS, H - 0.19, 6], h: [0.055, 0.07, 0.055], albedo: [0.35, 0.08, 0.05], flags: BoxFlag.NoShadow | BoxFlag.NoPrimary });
  switchables.push({ name: 'beacon0', boxes: [dome], group: 'emergency', on: false, baseEmissive: [9, 1.2, 0.35], mains: false });

  // ---- doors (closedDir 0: the leaf runs +x from its hinge; π/2: +z) ----
  const doors: LevelDoorDef[] = [
    { name: 'h_front', hinge: [AXIS - 0.58, WZ0], closedDir: 0, width: 1.16, exterior: true },
    { name: 'h_side', hinge: [WX1, 5.42], closedDir: HALF, width: 1.16, exterior: true },
  ];
  for (const d of doors) doorFrame(d);

  const HOUSE: Rect = { x0: WX0 + 0.12, x1: WX1 - 0.12, z0: WZ0 + 0.12, z1: WZ1 - 0.12 };
  const level: Level = {
    boxes: K.boxes, props: K.props, lights, switchables,
    doors,
    devices: [],
    breaker: { pos: [14.6, 1.35, WZ0 + 0.32], boxes: [brk, brkLed] },
    beacons: [{ pos: [AXIS, H - 0.3, 6], box: dome }],
    playerSpawn: [21.6, 0, 14.4],
    routes: [], chokepoints: [], clearPlans: [],
    sky: { zenith: [0.015, 0.025, 0.05], horizon: [0.045, 0.05, 0.07] },
    // the stealth mission's three places, because the level format wants them
    mission: { serverRoom: { ...HOUSE }, entryDoor: 'h_front', rack: { index: 0, front: [14.6, 1.0, WZ0 + 1.2], halfW: 0.3 }, exfilDoor: 'h_side', exfilX: 28.6, exfilZ: 6.0, exfilOut: [1, 0] },
  };
  const surfaces: SurfacesData = { building: { x0: WX0, x1: WX1, z0: WZ0, z1: WZ1 }, grassEdge: 0, inside: 'stone', outside: 'stone', verge: 'stone', zones: [] };
  return { level, scenarios: { doorNames: [{ door: 'h_front', name: 'front door' }, { door: 'h_side', name: 'side door' }] }, groups: trace.groups, surfaces };
}
