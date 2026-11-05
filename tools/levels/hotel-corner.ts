// HOTEL-CORNER: ONE CORNER OF A MINIMALIST HOTEL, a look test and nothing else (not a level to play, in no mission list). The second laying, after the
// first was seen on a real card and the maintainer sent photographs of the kind of place he means: a house whose colonnade FACES THE SUN, a pool whose pale
// shallow water lies FLUSH with the terrace, round columns on square plinths, smooth coursed stone against a rough garden wall, and timber as the one warm
// colour. NO ACCENT COLOUR. If this does not hold at play distance, nothing larger is built on it.
// THE RULES IT IS LAID BY (the maintainer's words on the white roof: "very tastefully done high frequency detail but overall quite flat", "it's all quite
// intentionally dressed"):
//   THE BUILDING SETS OUT THE FLOOR   a column every third metre, a band of darker stone from each to the pool, and every stone cut to lie between two
//   FOUR VALUES    pale paving underfoot, a darker smooth stone under the colonnade and in every band, dark planting and bronze, and the water
//   EDGES          no two fields butt: a flush band, a dark slot or a step lies between them
//   TWO STONES     the house is smooth, the garden wall is rough; the floor is jointed, so nothing is written or bolted on a wall
//   NO MARK WITHOUT A STUFF FOR IT   wet prints and a wet edge were laid twice as flat grey shapes and read as paper: they wait for a soft-edged stain
//   THE COLONNADE FACES THE SUN   the first laying's porch faced away from it and was a field of blue shade, half the picture. This one is open to the sky
//                  between its rafters: the sun lies on its floor and its wall, and the rafters' shadows cross both
//   STONES, NOT A PATTERN   the paving is laid slab by slab, two to a bay and one on the door's axis, with the bed showing in the joints: a tile of half a
//                  metre drew as a bathroom's, and its bays ran where the plan did not
//   WATER WITHOUT WATER   the pool is a flat glossy plane, pale, a hand under its edge, its steps drawn under it and three slabs across it: what water
//                  needs from this camera is the graphics work's question
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
import { PANEL_GREY, tonedAs, madeOf } from '../../src/scene/stuff';
import { type MarkPal } from '../../src/scene/markings';
import * as MD from '../../src/scene/markingDefaults';
import { coastHost, COAST_QUIET_OF } from './coastSkin';

export const NAME = 'hotel-corner';
export const NOTES = 'A look test: one corner of a minimalist hotel (a colonnade that faces the sun, paving laid stone by stone, pale water a hand under its edge, one daybed), no accent colour. The pool is a flat plane, not water. Not in the missions list. Regenerate: bun run tools/levels/export.ts hotel-corner';
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

  // ---- the ground under everything: the dark bed the paving is laid on, cut away where the pool is ----
  const WX0 = 12, WX1 = 27, WZ0 = 3, WZ1 = 9, AXIS = 19.5, CZ = 12, COLS = [12, 15, 18, 21, 24, 27];
  const X0 = 15, X1 = 40, Z0 = 16, Z1 = 20.5, EDGE = 0.5, DROP = 0.05, GAP = 0.015, TOP = 0.012;
  grp('ground');
  const BED: Vec3 = madeOf('earth', [0.2, 0.18, 0.15], { rough: 1 });
  slab(0, -0.6, 0, X0, 0, 28, BED); slab(X0, -0.6, 0, X1, 0, Z0, BED); slab(X0, -0.6, Z1, X1, 0, 28, BED); slab(X0, -0.9, Z0, X1, -0.6, Z1, BED);
  /** PAVING IS STONES, NOT A PATTERN: a rect laid as nx by nz equal slabs of the pale stone with a joint's width of the bed showing between them, each a
   *  tone dealt by where it lies. Big calm pieces: a tile of half a metre is a bathroom's */
  const TONES = [1, 0.965, 1.025];
  const paved = (x0: number, z0: number, x1: number, z1: number, nx: number, nz: number) => { const w = (x1 - x0 - (nx - 1) * GAP) / nx, d = (z1 - z0 - (nz - 1) * GAP) / nz;
    for (let i = 0; i < nx; i++) for (let j = 0; j < nz; j++) { const x = x0 + i * (w + GAP), z = z0 + j * (d + GAP); film(x, z, x + w, z + d, 'stoneSlab', TOP, TONES[Math.floor(dealt(x, z) * 3)]); } };
  /** … and a band of the darker stone, in lengths */
  const band = (x0: number, z0: number, x1: number, z1: number, n = 1) => { const along = x1 - x0 > z1 - z0, L = ((along ? x1 - x0 : z1 - z0) - (n - 1) * GAP) / n;
    for (let i = 0; i < n; i++) { const a = (along ? x0 : z0) + i * (L + GAP); if (along) film(a, z0, a + L, z1, 'stoneBand', TOP, 1.34); else film(x0, a, x1, a + L, 'stoneBand', TOP, 1.34); } };

  // ---- THE HOUSE: one long room behind a wall of smooth coursed stone, three openings that stand in the colonnade's bays, the middle one on the axis ----
  grp('walls.house');
  G.wallX(WZ1, WX0 - 0.12, WX1 + 0.12, 'block', [{ a: 15.75, b: 17.25, y0: 0, y1: 2.7 }, { a: AXIS - 0.9, b: AXIS + 0.9, y0: 0, y1: 2.7 }, { a: 21.75, b: 23.25, y0: 0, y1: 2.7 }]);
  G.wallX(WZ0, WX0 - 0.12, WX1 + 0.12, 'block', [{ a: AXIS - 0.6, b: AXIS + 0.6 }]);
  G.wallZ(WX0, WZ0 + 0.12, WZ1 - 0.12, 'block'); G.wallZ(WX1, WZ0 + 0.12, WZ1 - 0.12, 'block', [{ a: 5.4, b: 6.6 }]);
  // a timber shutter folded back against the wall on each side of an opening: the one warm colour, and it is a thing somebody opens every morning
  for (const [a, b] of [[15.75, 17.25], [AXIS - 0.9, AXIS + 0.9], [21.75, 23.25]]) for (const x of [a - 0.34, b + 0.34]) mass(x - 0.3, 0.04, WZ1 + 0.125, x + 0.3, 2.66, WZ1 + 0.17, 'cedar');
  grp('floors.house');
  film(WX0 + 0.12, WZ0 + 0.12, WX1 - 0.12, WZ1 - 0.12, 'stoneShade', TOP);
  film(0, 0, WX0 - 0.12, WZ1 + 0.3, 'paleGravel', TOP); film(WX0 - 0.12, 0, X1, WZ0 - 0.12, 'paleGravel', TOP); film(WX1 + 0.12, WZ0 - 0.12, X1, CZ - 0.3, 'paleGravel', TOP);   // gravel round the house's other sides
  grp('ceiling');
  slab(WX0 - 0.12, H, WZ0 - 0.12, WX1 + 0.12, H + 0.2, WZ1 + 0.12, P('skimWhite'), { flags: BoxFlag.NoPrimary, rough: COAST_FINISH.skimWhite.rough });
  grp('room.house');
  { const b = at(16.4, 4.6, 0); b(0, 0, 0, 0.08, 1.42, 0.32, 'bronze'); b(0, 0, 0.08, 1.02, 1.5, 0.4, 'cedar'); for (let i = 0; i < 3; i++) b(-0.9, 0.05, 1.02 + i * 0.05, 1.07 + i * 0.05, 0.22, 0.16, 'canvasWhite', 1 - i * 0.03); }   // a long desk, three folded towels on its end

  // ---- THE COLONNADE: round columns on tall square plinths, one at every third metre; a capital, a beam that sits on it, a cornice; rafters of smooth warm
  // timber with the sky between them. It faces the sun: the light lies on its floor and wall and the rafters rule both ----
  grp('colonnade');
  band(WX0 - 0.3, CZ - 0.3, WX0 + 1.5 - GAP / 2, CZ + 0.3); band(WX0 + 1.5 + GAP / 2, CZ - 0.3, WX1 - 1.5 - GAP / 2, CZ + 0.3, 4); band(WX1 - 1.5 + GAP / 2, CZ - 0.3, WX1 + 0.3, CZ + 0.3);                                                       // the line the columns stand on, flush: the edge between the paving and the floor under the rafters
  film(WX0 - 0.12, WZ1 + 0.12, WX1 + 0.12, CZ - 0.3 - GAP, 'stoneShade', TOP);
  for (const x of COLS) { mass(x - 0.3, 0, CZ - 0.3, x + 0.3, 0.5, CZ + 0.3, 'cream', 0.94); K.pipe(x, CZ, 0.5, 2.52, 0.21, P('cream'), [], true); for (const b of K.boxes.slice(-4)) Object.assign(b, row('cream')); mass(x - 0.3, 2.52, CZ - 0.3, x + 0.3, 2.64, CZ + 0.3, 'cream'); }
  mass(WX0 - 0.3, 2.64, CZ - 0.25, WX1 + 0.3, 2.94, CZ + 0.25, 'cream'); mass(WX0 - 0.34, 2.94, CZ - 0.3, WX1 + 0.34, 3.02, CZ + 0.3, 'cream', 0.97);
  for (let x = WX0; x <= WX1 + 1e-6; x += 0.75) mass(x - 0.06, 3.02, WZ1 + 0.14, x + 0.06, 3.26, CZ + 0.42, 'cedar');

  // ---- THE TERRACE: the building sets out the floor. A band of the darker stone runs from every column to the pool's edge; between two bands lie two
  // slabs' width of the pale stone, and in the bay on the door's axis ONE, so the widest stones in the place lead from the door to the water ----
  grp('terrace');
  const TZ0 = CZ + 0.3 + GAP, TZ1 = Z0 - EDGE - GAP, SZ0 = Z1 + EDGE + GAP, SZ1 = 25 - GAP, LAST = 33;
  const DZ0 = Z1 + 1.4, DZ1 = DZ0 + 0.1;   // the slot drain lies IN a joint: one course of stones this side of it, three beyond
  for (let x = COLS[0]; x <= LAST; x += 3) { band(x - 0.15, TZ0, x + 0.15, TZ1, 3); band(x - 0.15, SZ0, x + 0.15, DZ0 - GAP); band(x - 0.15, DZ1 + GAP, x + 0.15, SZ1, 3);
    if (x < LAST) { const one = x + 1.5 === AXIS, a = x + 0.15 + GAP, b = x + 2.85 - GAP, n = one ? 1 : 2; paved(a, TZ0, b, TZ1, n, 3); paved(a, SZ0, b, DZ0 - GAP, n, 1); paved(a, DZ1 + GAP, b, SZ1, n, 3); } }
  band(COLS[0] - 0.15, TZ1 + GAP, COLS[0] + 0.15, SZ0 - GAP, 5); paved(COLS[0] + 0.15 + GAP, TZ1 + GAP, X0 - EDGE - GAP, SZ0 - GAP, 2, 5);   // the pool's west end: the same two stones' width

  // ---- THE POOL: pale shallow water a hand under its edge. The edge is the terrace's own stone in lengths that break on the bands' lines and stand a
  // finger over the water; three broad steps are drawn under it at its west end; three thick slabs cross it on the house's axis ----
  grp('pool');
  const WATER: Vec3 = P('poolPale'), STEPS: Vec3[] = [[0.3, 0.42, 0.36], [0.25, 0.38, 0.33], [0.21, 0.345, 0.3]], LINER: Vec3 = madeOf('skim', [0.36, 0.42, 0.39], { rough: 0.5 });
  slab(X0, -0.5, Z0, X1, -DROP, Z1, WATER, { rough: 0.06, flags: BoxFlag.Gloss });
  STEPS.forEach((c, i) => slab(X0 + i * 0.5, -DROP, Z0, X0 + (i + 1) * 0.5, -DROP + 0.004, Z1, c, { rough: 0.08, flags: FLAT | BoxFlag.Gloss }));
  slab(X0, -0.5, Z0, X0 + 0.02, 0, Z1, LINER); slab(X0 + 0.02, -0.5, Z0, X1 - 0.1, 0, Z0 + 0.02, LINER); slab(X0 + 0.02, -0.5, Z1 - 0.02, X1 - 0.1, 0, Z1, LINER);   // the pool's own wall, pale, a hand of it over the water
  const edges: number[] = [];
  for (let x = X0; x < LAST; x += 1.5) { const a = x + GAP / 2, b = x + 1.5 - GAP / 2; edges.push(mass(a, 0, Z1 - 0.03, b, 0.03, Z1 + EDGE, 'stoneSlab', 0.99)); mass(a, 0, Z0 - EDGE, b, 0.03, Z0 + 0.03, 'stoneSlab', 1.01); }
  mass(X0 - EDGE, 0, Z1 + GAP / 2 - 0.03, X0 - GAP / 2, 0.03, Z1 + EDGE, 'stoneSlab'); mass(X0 - EDGE, 0, Z0 - EDGE, X0 - GAP / 2, 0.03, Z0 + 0.03 - GAP / 2, 'stoneSlab');   // the two corner stones
  for (let i = 0; i < 3; i++) { const z = Z0 + 0.03 + GAP + i * 1.5; mass(X0 - EDGE, 0, z, X0 + 0.03, 0.03, z + 1.5 - GAP - (i === 2 ? 0.06 : 0), 'stoneSlab', TONES[i]); }
  for (let i = 0; i < 3; i++) { const z = Z0 + 0.05 + i * 1.48; mass(AXIS - 0.75, -DROP - 0.12, z, AXIS + 0.75, 0.03, z + 1.42, 'cream', 0.94); }
  // its depths, a hand tall on the edge where a swimmer reads them (never on the water)
  { const INK: MarkPal = { albedo: P('bronze'), rough: 0.45 }; for (const [i, text] of [[0, '0.9'], [5, '1.4']] as const) { const b = K.boxes[edges[i]]; b.marks = MD.stencilMarks('+y', text, -0.1, -0.05, INK, 1, 'fine'); } }
  // one slim rail at the steps: two uprights and a bar that runs out over them
  mass(X0 - 0.3, 0.03, Z1 - 1.72, X0 - 0.24, 0.92, Z1 - 1.66, 'bronze'); mass(X0 + 1.0, -DROP, Z1 - 1.72, X0 + 1.06, 0.92, Z1 - 1.66, 'bronze'); mass(X0 - 0.3, 0.92, Z1 - 1.72, X0 + 1.06, 0.98, Z1 - 1.66, 'bronze');

  // ---- THE SLOT DRAIN: one dark line the terrace's whole length, a lift-out length every six metres ----
  grp('drain');
  film(COLS[0] - 0.15, DZ0, LAST + 0.15, DZ1, 'bronze', TOP);
  for (let x = 16.5; x < LAST; x += 6) film(x - 0.3, DZ0 - 0.02, x + 0.3, DZ1 + 0.02, 'stoneBand', TOP + 0.004);

  // ---- THE GARDEN WALL: rough stone to the house's smooth, a flat cope; along its foot a bed of gravel, bare until the look has leaves ----
  grp('garden');
  mass(10.5, 0, 6.0, 11.0, 1.1, 25.5, 'rubble'); mass(10.47, 1.1, 5.97, 11.03, 1.16, 25.53, 'stoneShade');
  mass(11.0, 0, 25.0, LAST + 0.5, 1.1, 25.5, 'rubble'); mass(11.03, 1.1, 24.97, LAST + 0.53, 1.16, 25.53, 'stoneShade', 0.97);
  film(11.0, WZ1 + 0.3, COLS[0] - 0.15 - GAP, 25.0, 'paleGravel', TOP);
  // ---- one daybed beside the steps, pulled a little out of line: a plain frame, a thick mattress, a bolster, a folded towel; a small stone table ----
  { const b = at(12.95, Z1 - 1.9, HALF + 0.06); b(0, 0, 0, 0.1, 0.9, 0.36, 'bronze'); b(0, 0, 0.1, 0.2, 1.0, 0.45, 'cedar'); b(-0.02, 0, 0.2, 0.38, 0.94, 0.41, 'canvasWhite'); b(0.74, 0, 0.38, 0.48, 0.13, 0.36, 'canvasWhite', 0.94); b(-0.7, 0.08, 0.38, 0.43, 0.16, 0.22, 'canvasWhite', 0.88); }
  mass(13.62, 0, Z1 - 3.25, 14.0, 0.4, Z1 - 2.87, 'cream', 0.96);

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
