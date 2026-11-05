// HOTEL-CORNER: ONE CORNER OF A MINIMALIST HOTEL, a look test and nothing else (not a level to play, in no mission list). After the hotel terrace's first
// pictures read as a blockout, the test is the smallest patch that could convince: twelve metres by twelve holding a porch's columns, its floor, the band a
// column sets out, a slot drain, a pool's coping and the water's edge, one daybed and one cypress with its shadow. NO ACCENT COLOUR. If this does not hold
// at play distance, nothing larger is built on it.
// THE RULES IT IS LAID BY (the maintainer's words on the white roof: "very tastefully done high frequency detail but overall quite flat", "it's all quite
// intentionally dressed"; and a review of the terrace):
//   FOUR VALUES    pale stone underfoot, a darker smooth stone under the porch and in every band, dark timber and planting, near-black water
//   EDGES          no two fields butt: a flush band, a dark slot or a step lies between them
//   THE BUILDING SETS OUT THE FLOOR   a band from every column to the pool; the drain runs the pool's whole length; the openings stand in the bays
//   BLANK WALLS    the floor is jointed, so the walls are a smooth skim with nothing on them; no bolt, no stencil, no stripe on the guest's side
//   MARKS BY CAUSE wet prints from the pool's steps to the daybed, the coping dark where water is carried over it, a strip polished through the middle bay
//   WATER WITHOUT WATER   the pool is dark, sunk under its coping, with entry steps that fade into it and a bench under its near side. It is a flat glossy
//                  plane a man could walk on: what water needs from this camera is the graphics work's question
// What this file lays it lays in the coast palette's quiet rows (src/scene/gwotPalette.ts), each toned by `k`: hotel-corner-bright.ts builds the same
// corner a third brighter, because how pale a sunlit plane may be painted before it sits at white is the thing being found out.
// Its helpers (film, mass, the frame a daybed is laid in) are this draft's own copies of the white roof study's; they are owed to the kit once.
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
export const NOTES = 'A look test: one corner of a minimalist hotel (porch, band, slot drain, coping, dark water, one daybed, one cypress), no accent colour. The pool is a flat plane, not water. Not in the missions list. Regenerate: bun run tools/levels/export.ts hotel-corner';
export const MATERIALS = COAST_MATERIALS;

/** `k`: every paint this file lays, times k (1 = the palette's rows as they are) */
export function build(k = 1): { level: Level; scenarios: LevelScenarios; groups: string[]; surfaces: SurfacesData } {
  const trace: GeneratorTrace = { groups: [] };
  const K = officeKit(trace), G = gwotKit(coastHost(gwotOfOffice(K), COAST_QUIET_OF)), C = COAST_PALETTE, H = WORLD.ceilingY;
  const { grp, add, slab, doorFrame, lights, switchables } = K;
  const HALF = Math.PI / 2, FLAT = BoxFlag.NoShadow, r4 = (v: number) => Math.round(v * 1e4) / 1e4;
  /** a row's paint at this build's tone (the row's own array at k = 1; else a tone of it, which keeps its word), and its finish */
  const tones = new Map<string, Vec3>();
  const P = (m: CoastMat, t = 1): Vec3 => { const key = `${m}:${t}`; let a = tones.get(key); if (!a) { a = k * t === 1 ? C[m] : tonedAs(C[m], C[m].map(v => r4(Math.min(0.95, v * k * t))) as Vec3); tones.set(key, a); } return a; };
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

  // ---- the terrace: one pale stone, wall to wall ----
  grp('ground');
  slab(0, -0.25, 0, 40, 0, 28, P('stonePale'), row('stonePale'));

  // ---- THE POOL: dark, sunk under its coping on three sides, an overflow slot on the fourth; steps that fade into it at its west end, a bench under its near side ----
  grp('pool');
  const X0 = 14, X1 = 30, Z0 = 3.0, Z1 = 7.4, COPE = 0.14;
  slab(X0, 0.002, Z0, X1, 0.01, Z1, P('poolDark'), { rough: 0.06, flags: FLAT | BoxFlag.Gloss });
  const STEPS: Vec3[] = [[0.22, 0.36, 0.33], [0.13, 0.26, 0.25], [0.07, 0.17, 0.17], [0.04, 0.11, 0.11]];
  STEPS.forEach((c, i) => slab(X0 + i * 0.4, 0.01, Z0, X0 + (i + 1) * 0.4, 0.014, Z1, c.map(v => r4(v * k)) as Vec3, { rough: 0.08, flags: FLAT | BoxFlag.Gloss }));
  slab(X0 + 1.6, 0.01, Z1 - 0.5, X1, 0.014, Z1, STEPS[3].map(v => r4(v * k)) as Vec3, { rough: 0.08, flags: FLAT | BoxFlag.Gloss });   // the bench under the near side, one step lighter than the deep
  const cope = mass(X0 - 0.45, 0, Z1, X1 + 0.45, COPE, Z1 + 0.45, 'stoneShade'); mass(X0 - 0.45, 0, Z0, X0, COPE, Z1, 'stoneShade'); mass(X1, 0, Z0, X1 + 0.45, COPE, Z1, 'stoneShade');
  film(X0 - 0.45, Z0 - 0.15, X1 + 0.45, Z0, 'bronze', 0.008);                                             // the overflow: the water runs to a black slot along its far side
  // its depths, small and crisp on the coping where a swimmer reads them (never on the water)
  { const INK: MarkPal = { albedo: P('bronze'), rough: 0.45 }, b = K.boxes[cope]; b.marks = [...MD.stencilMarks('+y', '1.2', (X0 + 2.2) - b.c[0], b.c[2] - (Z1 + 0.36), INK, 1, 'base'), ...MD.stencilMarks('+y', '2.0', (X1 - 1.4) - b.c[0], b.c[2] - (Z1 + 0.36), INK, 1, 'base')]; }
  // the coping is dark and shines where water is carried over it: at the steps
  slab(X0 - 0.3, COPE, Z1 + 0.02, X0 + 1.9, COPE + 0.004, Z1 + 0.43, P('stoneShade', 0.72), { rough: 0.2, flags: FLAT });
  for (const x of [X0 + 0.2, X0 + 1.4]) mass(x - 0.04, COPE, Z1 + 0.18, x + 0.04, 1.0, Z1 + 0.26, 'bronze');   // two square rails at the steps

  // ---- THE SLOT DRAIN: one dark line the pool's whole length, a lift-out length every six metres ----
  grp('drain');
  film(X0 - 0.45, 8.3, X1 + 0.45, 8.42, 'bronze', 0.008);
  for (let x = 17; x < X1; x += 6) film(x, 8.28, x + 0.6, 8.44, 'stoneBand', 0.011);

  // ---- THE PORCH: square columns at three metres, a floor of the darker stone that runs on through the openings, one beam, a roof the camera does not see ----
  grp('porch');
  const COLS = [15.5, 18.5, 21.5, 24.5, 27.5];
  film(12.0, 11.9, 29.0, 12.1, 'stoneBand', 0.008);                                                        // the edge between the terrace and the porch: a flush band
  film(12.0, 12.1, 29.0, 14.88, 'stoneShade', 0.008);
  for (const x of [12.5, ...COLS]) { mass(x - 0.26, 0, 12.04, x + 0.26, 0.05, 12.56, 'stoneBand'); mass(x - 0.225, 0.05, 12.075, x + 0.225, 3.0, 12.525, 'skimWhite'); }
  mass(12.2, 3.0, 12.05, 28.0, 3.35, 12.55, 'skimWhite');
  slab(12.2, 3.0, 12.55, 28.0, 3.2, 15.0, P('skimWhite'), { flags: BoxFlag.NoPrimary, rough: COAST_FINISH.skimWhite.rough });
  // THE BUILDING SETS OUT THE FLOOR: a band of the darker stone from every column to the drain
  grp('bands');
  for (const x of COLS) film(x - 0.15, 8.42, x + 0.15, 11.9, 'stoneBand', 0.008);
  film(19.3, 12.1, 20.7, 15.2, 'stoneShade', 0.011, 0.86, { rough: 0.22 });                                // a strip polished by feet through the middle bay

  // ---- the house behind the porch: blank smooth walls, three openings that stand in the bays, a front door on the same line ----
  grp('walls.house');
  G.wallX(15, 12.88, 28.12, 'mud', [{ a: 16.25, b: 17.75, y0: 0, y1: 2.7 }, { a: 19.25, b: 20.75, y0: 0, y1: 2.7 }, { a: 22.25, b: 23.75, y0: 0, y1: 2.7 }]);
  G.wallX(21, 12.88, 28.12, 'mud', [{ a: 19.4, b: 20.6 }, { a: 16.25, b: 17.75, y0: 0, y1: 2.7 }, { a: 22.25, b: 23.75, y0: 0, y1: 2.7 }]);
  G.wallZ(13, 15.12, 20.88, 'mud'); G.wallZ(28, 15.12, 20.88, 'mud', [{ a: 17.4, b: 18.6 }]);
  grp('floors.house');
  film(13.12, 15.12, 27.88, 20.88, 'stoneShade', 0.008);
  grp('ceiling');
  slab(12.88, H, 14.88, 28.12, H + 0.2, 21.12, P('skimWhite'), { flags: BoxFlag.NoPrimary, rough: COAST_FINISH.skimWhite.rough });
  grp('room.house');
  { const b = at(16.0, 18.6, 0); b(0, 0, 0, 0.08, 1.42, 0.32, 'bronze'); b(0, 0, 0.08, 1.02, 1.5, 0.4, 'timber'); for (let i = 0; i < 3; i++) b(-0.9, 0.05, 1.02 + i * 0.05, 1.07 + i * 0.05, 0.22, 0.16, 'canvasWhite', 1 - i * 0.03); }   // a long desk, three folded towels on its end

  // ---- one daybed, pulled a little out of line toward the shade; the prints of whoever came up the steps to it ----
  grp('terrace');
  daybed(17.4, 10.3, HALF + 0.12);
  for (let i = 0; i < 9; i++) { const u = i / 8, x = X0 + 0.9 + u * 2.3, z = Z1 + 0.6 + u * 2.0, s = i % 2 ? 0.11 : -0.11;
    add({ c: [x + s, 0.003, z - s * 0.6], h: [0.05, 0.003, 0.12], yaw: -0.85, albedo: P('stonePale', 0.7 + 0.24 * u), rough: 0.25 + 0.3 * u, flags: FLAT }); }

  // ---- one cypress in a square of dry earth cut in the terrace, edged; it is placed by where its shadow falls ----
  grp('cypress');
  film(24.3, 9.3, 25.9, 10.9, 'stoneBand', 0.008); film(24.4, 9.4, 25.8, 10.8, 'dryEarth', 0.011);
  cypress(25.1, 10.1, 7.2);

  // ---- lights: a bulb in the house, the moon (the page's hour sets the sun) ----
  grp('lights');
  G.bulb(20.5, 18.0, 'bulb_house', 'house', true);
  lights.push({ kind: 'dir', pos: [0, 0, 0], dir: norm([0.45, -0.5, -0.74]), color: [0.55, 0.7, 1.0], intensity: 0.9, range: 1e6, innerDeg: 0, outerDeg: 0, name: 'moon', group: 'exterior', enabled: true, volumetric: 0, radius: 0.009, mains: false });

  // ---- the breaker and one emergency beacon: the game's blackout wants both ----
  grp('breaker');
  const brk = add({ c: [14.6, 1.35, 20.83], h: [0.28, 0.36, 0.04], albedo: PANEL_GREY, name: 'breaker_panel' });
  const brkLed = add({ c: [14.42, 1.62, 20.78], h: [0.015, 0.015, 0.01], albedo: C.plasticDark, rough: 0.5, emissive: [0.3, 4, 0.6], flags: BoxFlag.NoShadow });
  grp('beacons');
  add({ c: [20.5, H - 0.06, 18], h: [0.07, 0.06, 0.07], albedo: [0.2, 0.2, 0.22], flags: BoxFlag.NoShadow | BoxFlag.NoPrimary });
  const dome = add({ c: [20.5, H - 0.19, 18], h: [0.055, 0.07, 0.055], albedo: [0.35, 0.08, 0.05], flags: BoxFlag.NoShadow | BoxFlag.NoPrimary });
  switchables.push({ name: 'beacon0', boxes: [dome], group: 'emergency', on: false, baseEmissive: [9, 1.2, 0.35], mains: false });

  // ---- doors (closedDir 0: the leaf runs +x from its hinge; π/2: +z) ----
  const doors: LevelDoorDef[] = [
    { name: 'h_front', hinge: [19.42, 21.0], closedDir: 0, width: 1.16, exterior: true },
    { name: 'h_side', hinge: [28.0, 17.42], closedDir: HALF, width: 1.16, exterior: true },
  ];
  for (const d of doors) doorFrame(d);

  const HOUSE: Rect = { x0: 13.12, x1: 27.88, z0: 15.12, z1: 20.88 };
  const level: Level = {
    boxes: K.boxes, props: K.props, lights, switchables,
    doors,
    devices: [],
    breaker: { pos: [14.6, 1.35, 20.68], boxes: [brk, brkLed] },
    beacons: [{ pos: [20.5, H - 0.3, 18], box: dome }],
    playerSpawn: [21.6, 0, 10.4],
    routes: [], chokepoints: [], clearPlans: [],
    sky: { zenith: [0.015, 0.025, 0.05], horizon: [0.045, 0.05, 0.07] },
    // the stealth mission's three places, because the level format wants them
    mission: { serverRoom: { ...HOUSE }, entryDoor: 'h_front', rack: { index: 0, front: [14.6, 1.0, 19.8], halfW: 0.3 }, exfilDoor: 'h_side', exfilX: 29.6, exfilZ: 18.0, exfilOut: [1, 0] },
  };
  const surfaces: SurfacesData = { building: { x0: 13, x1: 28, z0: 15, z1: 21 }, grassEdge: 0, inside: 'stone', outside: 'stone', verge: 'stone', zones: [] };
  return { level, scenarios: { doorNames: [{ door: 'h_front', name: 'front door' }, { door: 'h_side', name: 'side door' }] }, groups: trace.groups, surfaces };
}
