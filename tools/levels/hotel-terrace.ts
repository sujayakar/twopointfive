// HOTEL-TERRACE: A LOOK PROPOSAL, not a level to play and not in the missions list. The maintainer, 11-05, of the coast skin's first showroom (a sea front of
// warm paving, planks and lawn): "do you think you could get something that feels mirrors edge like with this setting?", then "I think a minimalist hotel
// (think amanzoe) would be an amazing setting". So: a small hotel on a hilltop terrace in full sun, built the way the white roof study is
// (tools/levels/white-roof.ts, which he called "starting to look very good"): THE GROUND IS ARCHITECTURE, white pavers wall to wall; every wall is cast
// panels a step under the ground's white; a colonnade whose shadow lies across the terrace; ONE long pool as the field of colour and one warm wall; few
// things, each big; and the personality placed by hand where somebody would have put it (a stencilled number at each daybed, the pool's depths, a
// drain, bays painted on the forecourt, a striped foot, a louvre, a plate).
// It stands the kit's own pieces (src/scene/gwotKit.ts) over the coast skin's QUIET table (tools/levels/coastSkin.ts COAST_QUIET_OF): no piece is written
// twice. What this file lays itself it lays in the coast palette's rows.
// NOT WATER: the pool is a flat glossy plane a man could walk on. What water needs from this camera is the graphics work's question.
//
//        x 0     8                 20                32      40
//   z 0  ┌──────────────── parapet ───────────────────────────┐
//   z 3  │       ╔══════════ the pool ═══════════╗            │
//   z 8  │       ╚════════════════════════════════╝           │
//        │   court    daybeds 1‥6, three parasols     tank     │      the pool terrace
//   z 13 │  ■ pool ■   ● ● ● ● ● ●  the colonnade      ┌screen┐ │
//   z 15 │ ┌─spa──┐   ┌──────── reception ───────┐    │ bins  │ │
//   z 21 │ └──────┘   └────────────door──────────┘    └───────┘ │
//        │   pots        the forecourt: bays, bollards, a van  │
//   z 28 └─────────────────────────────────────────────────────┘
//   bun run tools/levels/export.ts hotel-terrace [--check]     → levels/hotel-terrace.json
//   page: ?look&solo&levelFile=hotel-terrace.json&hour=midday
import type { Level, LevelDoorDef } from '../../src/scene/level';
import type { LevelScenarios, Rect } from '../../src/scene/scenarios';
import type { SurfacesData } from '../../src/scene/levelData';
import { officeKit, normalize3 as norm, type GeneratorTrace } from '../../src/scene/officeGen';
import { gwotKit, gwotOfOffice } from '../../src/scene/gwotKit';
import { COAST_PALETTE, COAST_FINISH, COAST_MATERIALS, type CoastMat } from '../../src/scene/gwotPalette';
import { WORLD } from '../../src/scene/world';
import { BoxFlag } from '../../src/scene/boxes';
import { PANEL_GREY } from '../../src/scene/stuff';
import { MAX_MARKS_PER_BOX, type MarkingDef, type MarkPal } from '../../src/scene/markings';
import * as MD from '../../src/scene/markingDefaults';
import { coastHost, COAST_QUIET_OF } from './coastSkin';

export const NAME = 'hotel-terrace';
export const NOTES = 'A look proposal: a minimalist hotel terrace in full sun, the kit\'s pieces over the coast skin\'s quiet table, the ground white pavers, one pool, one warm wall. The pool is a flat plane, not water. Not in the missions list. Regenerate: bun run tools/levels/export.ts hotel-terrace';
export const MATERIALS = COAST_MATERIALS;

export function build(): { level: Level; scenarios: LevelScenarios; groups: string[]; surfaces: SurfacesData } {
  const trace: GeneratorTrace = { groups: [] };
  const K = officeKit(trace), G = gwotKit(coastHost(gwotOfOffice(K), COAST_QUIET_OF)), C = COAST_PALETTE, H = WORLD.ceilingY;
  const { grp, add, slab, doorFrame, lights, switchables } = K;
  const HALF = Math.PI / 2, FLAT = BoxFlag.NoShadow;
  /** a coast row laid by this file: its paint and its finish */
  const row = (m: CoastMat) => ({ rough: COAST_FINISH[m].rough, ...(COAST_FINISH[m].metal ? { metal: true } : {}) });
  /** the painter's two tins (markings' palette entries) */
  const INK: MarkPal = { albedo: [0.1, 0.1, 0.11], rough: 0.55 }, CHALK: MarkPal = { albedo: [0.84, 0.84, 0.82], rough: 0.5 };
  /** a film of one row on the floor: drawn and lit, throwing no shadow */
  const film = (x0: number, z0: number, x1: number, z1: number, m: CoastMat, y = 0.008) => slab(x0, y - 0.004, z0, x1, y, z1, C[m], { ...row(m), flags: FLAT });
  /** a block of one row */
  const mass = (x0: number, y0: number, z0: number, x1: number, y1: number, z1: number, m: CoastMat) => slab(x0, y0, z0, x1, y1, z1, C[m], row(m));
  /** PAINT ON A BOX: markings added to what it carries (scene/markings.ts) */
  const mark = (i: number, ...ms: MarkingDef[][]) => { const b = K.boxes[i]; b.marks = [...(b.marks ?? []), ...ms.flat()]; if (b.marks.length > MAX_MARKS_PER_BOX) throw new Error(`hotel-terrace: box ${i} carries ${b.marks.length} markings, and a box holds ${MAX_MARKS_PER_BOX}`); return i; };
  /** a world place on a box's south face (x, y), or on its top (x, z: z is the SOUTH end of what is painted), in that face's metres from its centre */
  const onS = (i: number, x: number, y: number): [number, number] => [x - K.boxes[i].c[0], y - K.boxes[i].c[1]];
  const onTop = (i: number, x: number, z: number): [number, number] => [x - K.boxes[i].c[0], K.boxes[i].c[2] - z];
  const rectTop = (i: number, x0: number, z0: number, x1: number, z1: number): [number, number, number, number] => [x0 - K.boxes[i].c[0], K.boxes[i].c[2] - z1, x1 - K.boxes[i].c[0], K.boxes[i].c[2] - z0];
  /** A PILLAR: a square shaft half a metre thick from `y0` up to `top` (square and thick ON PURPOSE: a thin round draws fluted today, the round rule's fault,
   *  and this picture is not to be a picture of that) */
  const pillar = (x: number, z: number, top = 3.0, y0 = 0) => mass(x - 0.25, y0, z - 0.25, x + 0.25, top, z + 0.25, 'skimWhite');
  /** a round thing standing up: the kit's octagon of four slabs, which the game stands up as one round (scene/round.ts) */
  const round = (x: number, z: number, y0: number, y1: number, r: number, m: CoastMat) => {
    const a = r * 0.96, t = a * 0.4142;
    for (let k = 0; k < 4; k++) add({ c: [x, (y0 + y1 - 0.002 * k) / 2, z], h: [a, (y1 - 0.002 * k - y0) / 2, t], yaw: k * Math.PI / 4, albedo: C[m], ...row(m) });
  };
  /** A CYPRESS: a spire of three rounds, a metre across at the foot and four and a half metres tall. From above it hides almost no floor and throws a long
   *  shadow. Solid: a man can stand behind it. Its green is dark and flat (the look has no leaves yet) */
  const cypress = (x: number, z: number, tall = 4.5) => { const tiers: [number, number][] = [[0.12, 0.34], [0.5, 0.5], [0.72, 0.42], [0.88, 0.32], [1, 0.2]]; let y = 0; for (const [k, r] of tiers) { round(x, z, y, tall * k, r, 'cypress'); y = tall * k; } };
  /** a skin on a wall's south face: a band of another row a finger proud of it */
  const skinS = (x0: number, x1: number, y0: number, y1: number, zFace: number, m: CoastMat, proud = 0.012) => slab(x0, y0, zFace, x1, y1, zFace + proud, C[m], row(m));
  /** a pool: a glossy plane inside a smooth coping, a grated channel along its south side. NOT water (the head) */
  const pool = (x0: number, z0: number, x1: number, z1: number) => {
    const w = slab(x0, 0.006, z0, x1, 0.014, z1, C.poolDark, { rough: 0.08, flags: FLAT | BoxFlag.Gloss });
    for (const [a, b, c, d] of [[x0 - 0.4, z0 - 0.4, x1 + 0.4, z0], [x0 - 0.4, z1, x1 + 0.4, z1 + 0.4], [x0 - 0.4, z0, x0, z1], [x1, z0, x1 + 0.4, z1]] as const) mass(a, 0, b, c, 0.1, d, 'skimWhite');
    slab(x0 - 0.4, 0.006, z1 + 0.5, x1 + 0.4, 0.012, z1 + 0.68, C.darkSteel, { ...row('darkSteel'), flags: FLAT });
    for (const [a, b, c, d] of [[x0 + 0.3, z0 + 0.3, x1 - 0.3, z0 + 0.38], [x0 + 0.3, z1 - 0.38, x1 - 0.3, z1 - 0.3], [x0 + 0.3, z0 + 0.38, x0 + 0.38, z1 - 0.38], [x1 - 0.38, z0 + 0.38, x1 - 0.3, z1 - 0.38]] as const) film(a, b, c, d, 'enamelWhite', 0.019);   // a line of white tiles a hand in from every edge
    return w;
  };

  // ---- the ground: white pavers wall to wall, in the pieces that carry their own paint ----
  grp('ground');
  slab(0, -0.25, 0, 40, 0, 28, C.stonePale, row('stonePale'));
  grp('floors');
  const terrW = film(0.4, 8.9, 13.3, 13.0, 'stonePale'), terrM = film(13.3, 8.9, 26.7, 13.0, 'stonePale'), terrE = film(26.7, 8.9, 39.6, 13.0, 'stonePale');
  const fore = film(0.4, 21.3, 39.6, 27.6, 'stonePale');

  // ---- the parapet: a low cast wall on the three open sides, a smooth cap on it ----
  grp('walls.parapet');
  G.yardWallX(0.3, 0.2, 39.8, 'mud', [], 1.0); G.yardWallZ(0.3, 0.42, 21.0, 'mud', [], 1.0); G.yardWallZ(39.7, 0.42, 21.0, 'mud', [], 1.0);
  // ---- the boundary: a full wall down the forecourt's east side with the way in, a white gate standing open ----
  grp('walls.boundary');
  G.yardWallZ(39.7, 21.0, 27.9, 'mud', [{ a: 23.4, b: 26.4, y0: 0, y1: 2.6 }]); G.gate(39.7, 24.9, 'z', 3.0, -1, 'paintCream', 78, 2.4);

  // ---- CYPRESSES: a row behind the pool against the parapet, and four at the gate (the maintainer's picture, 11-05: tall narrow spires in a line) ----
  grp('cypresses');
  for (let x = 3.4; x <= 37; x += 2.4) cypress(x, 1.3, 4.2 + ((x * 7) % 3) * 0.25);
  for (const z of [22.2, 27.2]) cypress(38.6, z, 4.6);

  // ---- THE POOL: one long plane along the terrace's north edge; its depths on the coping, a line of white tiles across it ----
  grp('pool');
  const water = pool(8, 2.6, 32, 7.4);
  mark(water, MD.stencilMarks('+y', '1.2', ...onTop(water, 9.0, 6.9), CHALK, 2), MD.stencilMarks('+y', '2.0', ...onTop(water, 29.4, 6.9), CHALK, 2));
  mass(19.6, 0, 7.8, 20.4, 0.9, 8.0, 'brushedSteel');                                     // the ladder's two rails, as one plate at the pool's middle
  mark(terrM, MD.hazardMarks('+y', rectTop(terrM, 19.2, 8.95, 20.8, 9.35), INK, undefined, 2, 'coarse', 1));

  // ---- the pool terrace: six daybeds in three pairs, a parasol to a pair, a number at each bed's foot ----
  grp('terrace');
  const beds: [number, number, number][] = [[10.4, 1, 0], [12.8, 2, 0], [18.8, 3, 1], [21.2, 4, 1], [27.2, 5, 2], [29.6, 6, 2]];
  for (const [x, n, slabOf] of beds) { G.lounger(x, 10.4, HALF, n === 4 ? 'paintRust' : 'rugCream'); const t = [terrW, terrM, terrE][slabOf]; mark(t, MD.stencilMarks('+y', String(n), ...onTop(t, x - 0.12, 12.4), INK, 2)); }
  G.parasol(11.6, 10.0, 1.5); G.parasol(20.0, 10.0, 1.5, 'paintRust'); G.parasol(28.4, 10.0, 1.5);
  G.tub(15.8, 9.6, 'whitewash'); G.tub(24.2, 9.6, 'whitewash');

  // ---- THE COLONNADE: seven columns under one beam, between the terrace and the house. In the sun its shadow is the terrace's pattern ----
  grp('colonnade');
  mass(10.2, 0, 13.0, 29.8, 0.12, 14.88, 'skimWhite'); mass(10.2, 0, 12.6, 29.8, 0.06, 13.0, 'skimWhite');   // its floor a step and a half-step above the terrace: the ground has levels
  for (let x = 11; x <= 29; x += 3) pillar(x, 13.6, 3.0, 0.12);
  mass(10.5, 3.0, 13.2, 29.5, 3.3, 14.0, 'castPanel');

  // ---- RECEPTION: cast panels, a door to the terrace and one to the forecourt, a side door; inside, the one warm wall ----
  grp('walls.reception');
  G.wallX(15, 13.88, 26.12, 'mud', [{ a: 15.0, b: 18.6, y0: 0, y1: 2.6 }, { a: 21.4, b: 25.0, y0: 0, y1: 2.6 }]);   // OPEN to the colonnade: two bays of daylight either side of one pier (a shut house was a black hole in a sunlit picture)
  G.wallX(21, 13.88, 26.12, 'mud', [{ a: 19.4, b: 20.6 }, { a: 15.4, b: 17.6, y0: 0.95, y1: 2.1 }, { a: 22.4, b: 24.6, y0: 0.95, y1: 2.1 }]);
  G.wallZ(14, 15.12, 20.88, 'mud'); G.wallZ(26, 15.12, 20.88, 'mud', [{ a: 17.4, b: 18.6 }]);
  grp('floors.reception');
  slab(14.12, 0.006, 15.12, 25.88, 0.012, 20.88, C.paleSlab, { ...row('paleSlab'), flags: FLAT });
  grp('ceiling');
  slab(13.88, H, 14.88, 26.12, H + 0.2, 21.12, C.skimWhite, { flags: BoxFlag.NoPrimary, rough: COAST_FINISH.skimWhite.rough });
  grp('room.reception');
  skinS(18.6, 21.4, 0, H, 15.12, 'clay', 0.01); slab(25.87, 0, 15.12, 25.88, H, 17.3, C.clay, row('clay')); slab(25.87, 0, 18.7, 25.88, H, 20.88, C.clay, row('clay'));   // the warm wall: the pier between the bays and the east wall, what the camera sees through the open side
  G.table(17.0, 16.6, 2.6, 0.8, 0); G.plankBench(23.0, 19.6, 0, 1.8); G.plankBench(23.0, 16.4, 0, 1.8); G.tub(14.8, 20.2, 'whitewash'); G.tub(25.2, 20.2, 'whitewash');
  // its door's number, a louvre and a plate on the forecourt side: paint and fittings on the wall that faces the camera
  mark(skinS(18.55, 19.15, 1.35, 1.95, 21.12, 'enamelWhite', 0.01), MD.stencilMarks('+z', '1', -0.125, -0.175, INK, 2, 'base'));   // the door's number on a plate (no word names a place: a number, a depth, a bay, a line are the place's own)
  skinS(24.9, 25.7, 0.3, 1.1, 21.12, 'darkSteel', 0.02); skinS(14.3, 15.1, 2.2, 2.6, 21.12, 'brushedSteel', 0.02);
  G.planter(17.6, 21.9, 0, 2.4, 0.6, 'mud'); G.planter(22.4, 21.9, 0, 2.4, 0.6, 'mud');
  mass(19.2, 0, 21.12, 20.8, 0.06, 21.7, 'skimWhite');                                                                   // the front door's threshold: one smooth slab

  // ---- THE SPA: a small house on the west side with a square pool court before it, a column at each corner ----
  grp('walls.spa');
  G.wallX(15, 1.88, 9.12, 'mud', [{ a: 5.0, b: 6.2 }]); G.wallX(21, 1.88, 9.12, 'mud'); G.wallZ(2, 15.12, 20.88, 'mud'); G.wallZ(9, 15.12, 20.88, 'mud', [{ a: 17.4, b: 18.6 }]);
  grp('floors.spa');
  slab(2.12, 0.006, 15.12, 8.88, 0.012, 20.88, C.paleSlab, { ...row('paleSlab'), flags: FLAT });
  slab(1.88, H, 14.88, 9.12, H + 0.2, 21.12, C.skimWhite, { flags: BoxFlag.NoPrimary, rough: COAST_FINISH.skimWhite.rough });
  grp('room.spa');
  G.table(4.0, 17.2, 1.9, 0.7, HALF); G.table(7.0, 18.6, 1.9, 0.7, HALF); G.tub(2.7, 20.2, 'whitewash');
  grp('court');
  pool(3.6, 10.0, 7.4, 12.6);
  // a PERGOLA over it: four pillars, two beams, seven slats. In the sun its shadow is a ladder on the water and the pavers
  for (const [x, z] of [[2.6, 9.2], [8.4, 9.2], [2.6, 13.9], [8.4, 13.9]] as const) pillar(x, z, 2.6);
  for (const z of [9.2, 13.9]) mass(2.2, 2.6, z - 0.12, 8.8, 2.8, z + 0.12, 'skimWhite');
  for (let x = 2.9; x <= 8.2; x += 0.86) mass(x - 0.06, 2.8, 8.8, x + 0.06, 2.96, 14.3, 'enamelWhite');

  // ---- the service yard on the east side, behind a screen of cast panels: a tank, a generator, two bins under a flat canopy ----
  grp('service');
  G.tWall(30.0, 17.9, 3, HALF, 2.4);
  G.waterTank(36.2, 10.2, 'plasticWhite', 1.1); G.generator(33.2, 16.2, 0);
  for (const [x, z] of [[34.6, 18.2], [38.2, 18.2], [34.6, 20.4], [38.2, 20.4]] as const) add({ c: [x, 1.15, z], h: [0.04, 1.15, 0.04], albedo: C.brushedSteel, ...row('brushedSteel') });
  mass(34.3, 2.3, 17.9, 38.5, 2.42, 20.7, 'castPanel');
  G.wheelieBin(35.5, 19.3, 0, 'plasticWhite'); G.wheelieBin(37.2, 19.3, 0.08, 'plasticBlack');

  // ---- THE FORECOURT: three bays painted on the pavers, a striped line to stop at, bollards, a van and a pickup, a bicycle ----
  grp('forecourt');
  for (const x of [27.4, 30.6, 33.8, 37.0]) film(x, 23.4, x + 0.1, 27.4, 'plasticDark', 0.011);
  mark(fore, MD.stencilMarks('+y', 'P1', ...onTop(fore, 28.2, 27.2), INK, 2), MD.stencilMarks('+y', 'P2', ...onTop(fore, 31.4, 27.2), INK, 2), MD.stencilMarks('+y', 'P3', ...onTop(fore, 34.6, 27.2), INK, 2),
    MD.hazardMarks('+y', rectTop(fore, 19.2, 21.8, 20.8, 22.2), INK, undefined, 2, 'coarse', 1));
  G.van(29.0, 25.4, HALF); G.pickup(35.4, 25.4, -HALF, 'truckWhite');
  for (let x = 11; x <= 26; x += 3) mass(x - 0.2, 0, 22.8, x + 0.2, 0.45, 23.2, 'castPanel');                              // six low cast blocks where a car must stop (not thin posts: the pillar's note)
  add({ c: [5.0, 0.75, 23.6], h: [1.2, 0.02, 0.02], albedo: C.brushedSteel, ...row('brushedSteel') }); for (const x of [3.8, 6.2]) add({ c: [x, 0.365, 23.6], h: [0.018, 0.365, 0.018], albedo: C.brushedSteel, ...row('brushedSteel') });
  G.bicycle(5.0, 23.2, HALF + 0.08, 'paintRust');
  G.tub(1.4, 22.2, 'whitewash'); G.tub(10.2, 22.2, 'whitewash');
  G.kerb(0.5, 27.8, 39.5, 27.8, 'block');

  // ---- lights: a bulb in each house, a floodlight over the forecourt, the moon (the page's hour sets the sun) ----
  grp('lights');
  G.bulb(20.0, 18.0, 'bulb_reception', 'reception', true); G.bulb(5.5, 18.0, 'bulb_spa', 'spa', true);
  grp('exterior.lamps');
  G.floodLight(26.4, 21.6, [1, 0.8], 'flood_forecourt', true);
  lights.push({ kind: 'dir', pos: [0, 0, 0], dir: norm([0.45, -0.5, -0.74]), color: [0.55, 0.7, 1.0], intensity: 0.9, range: 1e6, innerDeg: 0, outerDeg: 0, name: 'moon', group: 'exterior', enabled: true, volumetric: 0, radius: 0.009, mains: false });

  // ---- the breaker (reception's north wall, inside) and one emergency beacon: the game's blackout wants both ----
  grp('breaker');
  const brk = add({ c: [14.9, 1.35, 20.83], h: [0.28, 0.36, 0.04], albedo: PANEL_GREY, name: 'breaker_panel' });
  const brkLed = add({ c: [14.72, 1.62, 20.78], h: [0.015, 0.015, 0.01], albedo: C.plasticDark, rough: 0.5, emissive: [0.3, 4, 0.6], flags: BoxFlag.NoShadow });
  grp('beacons');
  add({ c: [20, H - 0.06, 18], h: [0.07, 0.06, 0.07], albedo: [0.2, 0.2, 0.22], flags: BoxFlag.NoShadow | BoxFlag.NoPrimary });
  const dome = add({ c: [20, H - 0.19, 18], h: [0.055, 0.07, 0.055], albedo: [0.35, 0.08, 0.05], flags: BoxFlag.NoShadow | BoxFlag.NoPrimary });
  switchables.push({ name: 'beacon0', boxes: [dome], group: 'emergency', on: false, baseEmissive: [9, 1.2, 0.35], mains: false });

  // ---- doors (closedDir 0: the leaf runs +x from its hinge; π/2: +z) ----
  const doors: LevelDoorDef[] = [
    { name: 'r_front', hinge: [19.42, 21.0], closedDir: 0, width: 1.16, exterior: true },
    { name: 'r_side', hinge: [26.0, 17.42], closedDir: HALF, width: 1.16, exterior: true },
    { name: 's_court', hinge: [5.02, 15.0], closedDir: 0, width: 1.16, exterior: true },
    { name: 's_side', hinge: [9.0, 17.42], closedDir: HALF, width: 1.16, exterior: true },
  ];
  for (const d of doors) doorFrame(d);

  const RECEPTION: Rect = { x0: 14.12, x1: 25.88, z0: 15.12, z1: 20.88 };
  const level: Level = {
    boxes: K.boxes, props: K.props, lights, switchables,
    doors,
    devices: [],
    breaker: { pos: [14.9, 1.35, 20.68], boxes: [brk, brkLed] },
    beacons: [{ pos: [20, H - 0.3, 18], box: dome }],
    playerSpawn: [16.0, 0, 11.6],
    routes: [], chokepoints: [], clearPlans: [],
    sky: { zenith: [0.015, 0.025, 0.05], horizon: [0.045, 0.05, 0.07] },
    // the stealth mission's three places, because the level format wants them
    mission: { serverRoom: { ...RECEPTION }, entryDoor: 'r_front', rack: { index: 0, front: [14.9, 1.0, 19.8], halfW: 0.3 }, exfilDoor: 'r_side', exfilX: 27.6, exfilZ: 18.0, exfilOut: [1, 0] },
  };
  const surfaces: SurfacesData = { building: { x0: 14, x1: 26, z0: 15, z1: 21 }, grassEdge: 0, inside: 'stone', outside: 'stone', verge: 'stone', zones: [] };
  return { level, scenarios: { doorNames: [{ door: 'r_front', name: 'front door' }, { door: 'r_side', name: 'side door' }, { door: 's_court', name: 'spa door' }, { door: 's_side', name: 'spa side door' }] }, groups: trace.groups, surfaces };
}
