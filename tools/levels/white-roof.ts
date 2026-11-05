// WHITE-ROOF: a look study, not a mission. One sentence nobody could say of another level: a kept roof in full sun where every surface is ONE colour and
// shows what it is made of by its relief, its finish and its pieces.
// The maintainer, 11-04, of a game whose pictures he sent: "it has some very tastefully done high frequency detail but overall quite flat", "it's all quite
// intentionally dressed", "i'm referring specifically to the texture work", and then "could you try making a small level that recreates the ... look".
// So every box the level's own code makes says what it is made of (Box.stuff: the ten kept stuffs of render/lookTex.ts), every colour is a flat one chosen
// for its place, and nothing is weathered. (The kit's lamp fittings are the kit's.) Hand placement, and why:
//   the hall's wall    the whole north side, three metres of cast panels over an orange stucco plinth: the biggest face that looks at the camera
//   the plant          four cast volumes of four heights in the middle of the deck, a yellow line round every top: what the sun throws shadows off
//   the yellow wall    concrete cast against boards behind the plant, a steel band and two pipes on it: the one wall whose relief runs one way
//   the service rooms  a block of painted concrete block with stucco piers, a skimmed floor, steel cabinets; a lounge with a carpet and one orange wall
//   the stair head     one solid orange mass with a louvre and a steel door skin
//   the pad            a painted rectangle of deck with white lines
// He looked at it, 11-04: "directionally nice but still doesn't have enough "personality / cm^2"". His pictures carry three things on every patch and the first
// pass had one of them, the stuff. The other two are placed here by hand, each where somebody would have put it:
//   PAINT a painter left   stencilled bays, numbers and a way out, warning stripes at a plinth and before a door, a hatch's outline, a plate with its bolts:
//                          markings (scene/markings.ts: cells on a face, which the look draws as old paint), on the wall or top that carries them
//   SMALL FITTINGS         conduit with its clips and boxes along the walls, vents in the hall's wall, clamps and flanges on the pipes, a hose cabinet, a
//                          railing whose shadow lies on the deck, and what the last job left behind: a pallet of spare pavers, paint cans, a bucket, a plank
// Walls stand on a half-metre grid: a paver is half a metre and a panel a metre by a half, and their maps lie by the world, so those joints land on walls'
// ends without anybody fitting them. (A 40 cm block does not divide half a metre: a course ends on a cut block at some corners, as a real one does.)
//
//        x 5        11.5      17   19.5              29.5  31     34.5
//   z 5  ══════════════ the hall's wall (white panels, orange plinth) ═══════════
//   z 9  ┌─────────┬─────────┐    ════ the yellow wall ════   ┌──────┐
//        │ service │ lounge  lg         the plant            │ stair│
//   z 15 └──sv─────┴─────────┘      (four volumes)            └──────┘
//   z 17      ┌ the pad ┐
//   z 22      └─────────┘                 Sam
//   z 23.5 ───────────────────── parapet ─────────────────────
//   bun run tools/levels/export.ts white-roof [--check]     → levels/white-roof.json
//   page: ?look&solo&levelFile=white-roof.json&hour=midday
import type { Level, LevelDoorDef } from '../../src/scene/level';
import type { LevelScenarios, Rect } from '../../src/scene/scenarios';
import type { SurfacesData } from '../../src/scene/levelData';
import { officeKit, normalize3 as norm, type GeneratorTrace } from '../../src/scene/officeGen';
import { gwotKit, gwotOfOffice, GWOT_MATERIALS } from '../../src/scene/gwotKit';
import { WORLD } from '../../src/scene/world';
import { BoxFlag, type Box } from '../../src/scene/boxes';
import { markFromRect, type MarkPal } from '../../src/scene/markings';
import * as MD from '../../src/scene/markingDefaults';
import { madeOf } from '../../src/scene/stuff';
import type { Vec3 } from '../../src/math/vec';

export const NAME = 'white-roof';
export const NOTES = 'A look study: a kept roof in full sun, every surface one flat colour and one of the ten kept stuffs (render/lookTex.ts). Not in the missions list. Regenerate: bun run tools/levels/export.ts white-roof';
export const MATERIALS = GWOT_MATERIALS;

// ---- the paints: each ONE flat colour (linear), with the finish its stuff is written around (lookTex.ts LOOK_ROUGH)
type Paint = Pick<Box, 'albedo' | 'rough'> & { stuff: NonNullable<Box['stuff']> };
// (each paint is a ROW the kit knows by: its numbers, its finish and its word, so a kit piece handed the row says what the row is. scene/stuff.ts madeOf)
const paint = (albedo: Vec3, rough: number, stuff: Paint['stuff']): Paint => ({ albedo: madeOf(stuff, [albedo[0], albedo[1], albedo[2]], { rough }), rough, stuff });
const ORANGE: Vec3 = [0.88, 0.24, 0.03], YELLOW: Vec3 = [0.88, 0.6, 0.04], BLUE: Vec3 = [0.04, 0.2, 0.72];
const PANEL = paint([0.56, 0.56, 0.54], 0.68, 'cast panel'), PAVER = paint([0.72, 0.73, 0.74], 0.5, 'paver'),   // (cast concrete a step under the white deck it stands on: ground, walls and things sit a step apart before any light falls)
   BLOCK = paint([0.76, 0.77, 0.77], 0.4, 'gloss block'), SKIM = paint([0.78, 0.78, 0.77], 0.55, 'skim');
const STUCCO = paint(ORANGE, 0.42, 'stucco'), BOARD = paint(YELLOW, 0.42, 'board concrete'), LOUVRE = paint([0.5, 0.51, 0.52], 0.45, 'louvre');
const STEEL = paint([0.56, 0.57, 0.58], 0.32, 'brushed');   // (not `metal`: a mirror of the bounce's metre-wide probes on a hand-sized plate is blotches, on the software adapter at least)
const COAT = paint([0.7, 0.71, 0.72], 0.35, 'enamel'), CARPET = paint(BLUE, 0.95, 'carpet'), SEAT = paint([0.03, 0.14, 0.55], 0.95, 'carpet');
const GLOSS_Y = paint(YELLOW, 0.26, 'enamel'), GLOSS_W = paint([0.8, 0.8, 0.8], 0.3, 'enamel'), GLOSS_O = paint(ORANGE, 0.3, 'enamel');
// ---- the painter's tins (markings' palette entries: a material each, never a shaded colour)
const { ink: INK, chalk: CHALK, signal: SIGNAL } = MD.TIN, RED: MarkPal = { albedo: [0.62, 0.05, 0.04], rough: 0.4 };
// (a band of colour is PAINT ON THE SAME WALL: the wall's own stuff in another colour, a finger proud, so its joints and holes run on under the colour's edge)
const PLINTH = paint(ORANGE, 0.5, 'cast panel'), DADO = paint(ORANGE, 0.36, 'gloss block');
const WOOD = paint([0.4, 0.3, 0.19], 0.84, 'wood'), CARD = paint([0.45, 0.33, 0.2], 0.9, 'plain'), DARK = paint([0.1, 0.1, 0.11], 0.5, 'enamel');
const LINE_Y = paint(YELLOW, 0.4, 'enamel'), LINE_W = paint([0.82, 0.82, 0.82], 0.4, 'enamel'), PAD = paint(ORANGE, 0.55, 'skim'), VOID = paint([0.035, 0.04, 0.05], 0.9, 'plain');

export function build(): { level: Level; scenarios: LevelScenarios; groups: string[]; surfaces: SurfacesData } {
  const trace: GeneratorTrace = { groups: [] };
  const K = officeKit(trace), G = gwotKit(gwotOfOffice(K)), H = WORLD.ceilingY;
  const { grp, add, slab, doorFrame, lights } = K;
  const HALF = Math.PI / 2, SHELL = 0.24, PART = 0.12, FLAT = BoxFlag.NoShadow;
  /** a painted line or patch on a floor: a film of paint, drawn and lit, throwing no shadow */
  const film = (x0: number, z0: number, x1: number, z1: number, p: Paint, y = 0.008) => slab(x0, y - 0.004, z0, x1, y, z1, p.albedo, { rough: p.rough, stuff: p.stuff, flags: FLAT });
  /** a line round the top of a block: the yellow every top on this roof is edged with */
  const edged = (x0: number, z0: number, x1: number, z1: number, y: number, w = 0.08) => {
    slab(x0, y, z0, x1, y + 0.005, z0 + w, LINE_Y.albedo, { rough: LINE_Y.rough, stuff: LINE_Y.stuff, flags: FLAT }); slab(x0, y, z1 - w, x1, y + 0.005, z1, LINE_Y.albedo, { rough: LINE_Y.rough, stuff: LINE_Y.stuff, flags: FLAT });
    slab(x0, y, z0 + w, x0 + w, y + 0.005, z1 - w, LINE_Y.albedo, { rough: LINE_Y.rough, stuff: LINE_Y.stuff, flags: FLAT }); slab(x1 - w, y, z0 + w, x1, y + 0.005, z1 - w, LINE_Y.albedo, { rough: LINE_Y.rough, stuff: LINE_Y.stuff, flags: FLAT });
  };
  /** a cast block standing on the deck, its top edged */
  const mass = (x0: number, z0: number, x1: number, z1: number, h: number, p: Paint = PANEL) => { const i = slab(x0, 0, z0, x1, h, z1, p.albedo, { rough: p.rough, stuff: p.stuff }); edged(x0, z0, x1, z1, h); return i; };
  // paint on what is there: the kit's tools (scene/officeGen.ts: a box by its face, markings added to it, a world place in a face's own metres)
  const { paintOn: mark, onS, onTop, rectTop } = K;
  /** a run of conduit on a wall's south face from x0 to x1 at height y, a clip every metre and a half; `drop` = a branch down to that height at x1 (the kit's
   *  piece, standing: scene/officeGen.ts conduit) */
  const conduitS = (x0: number, x1: number, y: number, zFace: number, drop?: number) => K.conduit('x', zFace, x0, x1, 1, y, COAT.albedo, STEEL.albedo, drop, 0, true);
  /** a round thing standing up: the kit's pipe (four slabs, which the game stands up as one round: scene/round.ts), standing */
  const round = (x: number, z: number, y0: number, y1: number, r: number, p: Paint) => K.pipe(x, z, y0, y1, r, p.albedo, [], true);
  /** a skin on a wall's south face: a band of another stuff a finger proud of it (a plinth, a steel band, a door's plate), and a louvre: the kit's band
   *  and vent, standing (the face is given, so the wall has no thickness to add) */
  const skinS = (x0: number, x1: number, y0: number, y1: number, zFace: number, p: Paint, proud = 0.012) => K.band('x', zFace, x0, x1, 1, y0, y1, p.albedo, 0, proud, true);
  const ventS = (x0: number, x1: number, y0: number, y1: number, zFace: number, proud = 0.02) => K.vent('x', zFace, x0, x1, 1, y0, y1, LOUVRE.albedo, 0, proud, true);

  // ---- the deck: pavers wall to wall, and the dark beyond the parapet (a roof ends in air)
  grp('ground');
  slab(0, -0.25, 0, 40, 0, 28, VOID.albedo, { rough: VOID.rough, stuff: VOID.stuff });
  grp('floors');
  const deck = slab(4.88, 0, 4.88, 35.12, 0.006, 23.62, PAVER.albedo, { rough: PAVER.rough, stuff: PAVER.stuff, flags: FLAT });

  // ---- the hall's wall: the north side, cast in panels, on an orange plinth
  grp('walls.hall');
  const hall = K.boxes.length;
  K.wallX(5, 4.88, 35.12, SHELL, PANEL.albedo, [], { rough: PANEL.rough, stuff: PANEL.stuff }, H);
  if (K.boxes.length !== hall + 1) throw new Error('white-roof: the hall wall is one box, and its paint says so');
  skinS(5.12, 34.88, 0, 0.75, 5.12, PLINTH, 0.012);
  [10, 20, 30].forEach((x, k) => {                                                                                                                 // three piers: the wall is not one sheet
    const pier = slab(x - 0.25, 0, 5.12, x + 0.25, H, 5.42, PANEL.albedo, { rough: PANEL.rough, stuff: PANEL.stuff });
    mark(pier, MD.stencilMarks('+z', String(k + 1), -0.125, 0.25, INK, 2, 'base'));                                                               // its number at a man's head
    mark(skinS(x - 0.25, x + 0.25, 0, 0.75, 5.42, PLINTH, 0.012), MD.hazardMarks('+z', [-0.25, -0.375, 0.25, 0.375], CHALK, undefined, 3, 'base'));   // its plinth striped: mind the corner
  });
  // the painter's stencil, a bay at a time, and the way out; two vents over the plinth in each of the two middle bays; conduit under the wall's head, a box where a run drops
  mark(hall, MD.stencilMarks('+z', 'BAY 1', ...onS(hall, 6.7, 1.55), INK), MD.stencilMarks('+z', 'BAY 2', ...onS(hall, 14.3, 1.55), INK), MD.stencilMarks('+z', 'BAY 3', ...onS(hall, 24.3, 1.55), INK), MD.stencilMarks('+z', 'EXIT→', ...onS(hall, 31.8, 1.55), INK));
  for (const x of [12.3, 17.4, 22.3, 27.4]) ventS(x - 0.3, x + 0.3, 0.9, 1.3, 5.12, 0.02);
  conduitS(5.3, 9.4, 2.6, 5.12, 2.2); conduitS(10.6, 19.4, 2.6, 5.12); conduitS(20.6, 29.4, 2.6, 5.12, 2.2); conduitS(30.6, 34.7, 2.6, 5.12);

  // ---- the parapet: a low cast wall on the other three sides, its top edged
  grp('walls.parapet');
  K.wallX(23.5, 4.88, 35.12, SHELL, PANEL.albedo, [], { rough: PANEL.rough, stuff: PANEL.stuff }, 1.0);
  K.wallZ(5, 5.12, 23.38, SHELL, PANEL.albedo, [], { rough: PANEL.rough, stuff: PANEL.stuff }, 1.0);
  K.wallZ(35, 5.12, 23.38, SHELL, PANEL.albedo, [], { rough: PANEL.rough, stuff: PANEL.stuff }, 1.0);
  for (const [x0, z0, x1, z1] of [[4.88, 23.38, 35.12, 23.62], [4.88, 5.12, 5.12, 23.38], [34.88, 5.12, 35.12, 23.38]] as const) slab(x0, 1.0, z0, x1, 1.005, z1, LINE_Y.albedo, { rough: LINE_Y.rough, stuff: LINE_Y.stuff, flags: FLAT });
  film(5.12, 23.2, 34.88, 23.3, LINE_Y); film(5.2, 5.5, 5.3, 23.2, LINE_Y); film(34.7, 5.5, 34.8, 23.2, LINE_Y);   // a yellow line a hand in from every edge

  // ---- the service rooms: painted block, a skimmed floor, stucco piers; and a lounge beside it
  grp('walls.rooms');
  const blockX = { rough: BLOCK.rough, stuff: BLOCK.stuff };
  K.wallX(9, 5.88, 17.12, SHELL, BLOCK.albedo, [], blockX);
  K.wallX(15, 5.88, 17.12, SHELL, BLOCK.albedo, [{ a: 8.0, b: 9.2 }], blockX);
  K.wallZ(6, 9.12, 14.88, SHELL, BLOCK.albedo, [], blockX);
  K.wallZ(17, 9.12, 14.88, SHELL, BLOCK.albedo, [{ a: 11.4, b: 12.6 }], blockX);
  K.wallZ(11.5, 9.12, 14.88, PART, SKIM.albedo, [{ a: 11.4, b: 12.5 }], { rough: SKIM.rough, stuff: SKIM.stuff });
  grp('floors.rooms');
  slab(6.12, 0.006, 9.12, 11.44, 0.012, 14.88, SKIM.albedo, { rough: 0.45, stuff: SKIM.stuff, flags: FLAT });
  slab(11.56, 0.006, 9.12, 16.88, 0.012, 14.88, CARPET.albedo, { rough: CARPET.rough, stuff: CARPET.stuff, flags: FLAT });
  grp('ceiling');
  slab(5.88, H, 8.88, 17.12, H + 0.2, 15.12, SKIM.albedo, { flags: BoxFlag.NoPrimary, rough: SKIM.rough, stuff: SKIM.stuff });
  grp('walls.rooms.dado');       // the block's two outer faces are orange to a metre: the same blocks under the colour, its edge a hard line across them
  skinS(5.88, 7.9, 0, 1.0, 15.12, DADO, 0.01); skinS(9.3, 17.13, 0, 1.0, 15.12, DADO, 0.01);
  for (const [z0, z1] of [[8.88, 11.3], [12.7, 15.12]] as const) slab(17.12, 0, z0, 17.13, 1.0, z1, DADO.albedo, { rough: DADO.rough, stuff: DADO.stuff });
  grp('walls.rooms.fittings');   // what hangs on those two faces: conduit along the head, a lamp case over the service door, a hose cabinet, each door's number on a plate, a vent
  conduitS(6.1, 9.9, 2.75, 15.12); conduitS(10.2, 16.9, 2.75, 15.12, 2.45);
  slab(8.2, 2.3, 15.12, 9.0, 2.4, 15.24, COAT.albedo, { rough: COAT.rough, stuff: COAT.stuff });
  mark(add({ c: [10.7, 1.4, 15.19], h: [0.36, 0.3, 0.06], albedo: GLOSS_W.albedo, rough: GLOSS_W.rough, stuff: GLOSS_W.stuff }), MD.stencilMarks('+z', 'FIRE', -0.2875, 0.02, RED, 1, 'base'), MD.plateMark('+z', [-0.3, -0.2, 0.3, -0.125], RED));
  mark(add({ c: [9.75, 1.6, 15.138], h: [0.16, 0.21, 0.008], albedo: GLOSS_O.albedo, rough: GLOSS_O.rough, stuff: GLOSS_O.stuff, flags: FLAT }), MD.stencilMarks('+z', '1', -0.125, -0.175, CHALK, 2, 'base'));
  mark(add({ c: [17.138, 1.6, 13.15], h: [0.008, 0.21, 0.16], albedo: GLOSS_O.albedo, rough: GLOSS_O.rough, stuff: GLOSS_O.stuff, flags: FLAT }), MD.stencilMarks('+x', '2', -0.125, -0.175, CHALK, 2, 'base'));
  ventS(13.4, 14.4, 2.15, 2.55, 15.12, 0.02);
  grp('room.service');
  for (const [x, z] of [[6.32, 9.32], [11.24, 9.32], [6.32, 14.68]] as const) slab(x - 0.2, 0, z - 0.2, x + 0.2, H, z + 0.2, STUCCO.albedo, { rough: STUCCO.rough, stuff: STUCCO.stuff });   // orange piers in three corners (the fourth is the door's)
  for (const x of [7.4, 8.0]) { slab(x - 0.22, 0.08, 9.12, x + 0.22, 1.0, 9.36, STEEL.albedo, { rough: STEEL.rough, stuff: STEEL.stuff }); skinS(x - 0.12, x + 0.12, 0.45, 0.85, 9.36, LINE_Y, 0.004); }   // two steel cabinets, a yellow label each
  slab(9.4, 0, 9.12, 10.6, 0.9, 9.6, COAT.albedo, { rough: COAT.rough, stuff: COAT.stuff }); ventS(9.5, 10.5, 0.15, 0.75, 9.6, 0.02);   // an air handler under the far wall, a louvre in its face
  grp('room.lounge');
  slab(11.56, 0, 9.12, 16.88, H, 9.15, STUCCO.albedo, { rough: STUCCO.rough, stuff: STUCCO.stuff });                                              // the one orange wall
  for (const z of [10.2, 13.6]) { slab(13.2, 0, z - 0.3, 15.2, 0.4, z + 0.3, SEAT.albedo, { rough: SEAT.rough, stuff: SEAT.stuff }); slab(13.2, 0.4, z - 0.3, 15.2, 0.75, z - 0.18, SEAT.albedo, { rough: SEAT.rough, stuff: SEAT.stuff }); }   // two benches
  slab(13.7, 0, 11.6, 14.7, 0.36, 12.2, COAT.albedo, { rough: 0.3, stuff: COAT.stuff });                                                            // a low table between them

  // ---- the yellow wall: cast against boards, a steel band along it, two pipes up it
  grp('walls.yellow');
  const yel = K.boxes.length;
  K.wallX(9, 19.5, 29.5, SHELL, BOARD.albedo, [], { rough: BOARD.rough, stuff: BOARD.stuff });
  if (K.boxes.length !== yel + 1) throw new Error('white-roof: the yellow wall is one box, and its paint says so');
  mark(yel, MD.stencilMarks('+z', 'AC-02', ...onS(yel, 27.5, 1.95), INK));
  skinS(19.5, 29.5, 0, 0.12, 9.12, PANEL, 0.02);                                                                                                  // a grey skirting
  for (let x = 19.5; x < 29.4; x += 2) mark(skinS(x + 0.004, x + 1.996, 1.1, 1.32, 9.12, STEEL, 0.015), MD.boltMarks('+z', -0.8, 0.8, -0.06, 10, 'coarse'));   // the steel band: five plates butted, four bolt heads a plate
  conduitS(20.7, 29.3, 2.62, 9.12, 2.3);
  round(20.1, 9.32, 0, H, 0.11, GLOSS_Y); round(20.45, 9.32, 0, H, 0.11, GLOSS_Y);
  round(20.1, 9.32, 0, 0.05, 0.17, GLOSS_Y); round(20.45, 9.32, 0, 0.05, 0.17, GLOSS_Y);                                                           // a flange at each foot, and two clamps that hold the pair to the wall
  for (const y of [1.55, 2.4]) slab(19.94, y, 9.12, 20.61, y + 0.06, 9.46, STEEL.albedo, { rough: STEEL.rough, stuff: STEEL.stuff });

  // ---- the plant: four cast volumes of four heights, a louvre and a hatch in their faces, round stacks beside them
  grp('plant');
  // every volume says its number on its top, where the camera reads it; a hatch is a yellow outline; the low one that faces the deck has a striped foot
  const m1 = mass(21.5, 10.0, 24.5, 13.0, 2.4); ventS(22.0, 24.0, 0.3, 1.5, 13.0, 0.03);
  mark(m1, MD.stencilMarks('+y', '1', ...onTop(m1, 22.0, 12.5), INK, 4), MD.outlineMarks('+y', rectTop(m1, 23.3, 10.4, 24.1, 11.2), SIGNAL), MD.ratingPlateMarks('+z', [1.1, 0.0, 1.4, 0.2], 3));
  const m2 = mass(24.5, 10.0, 27.0, 12.0, 1.5); mark(skinS(25.2, 26.3, 0.1, 1.2, 12.0, STEEL, 0.015), MD.boltMarks('+z', -0.5, 0.5, 0.44, 4, 'coarse'), MD.boltMarks('+z', -0.5, 0.5, -0.44, 4, 'coarse'), MD.ratingPlateMarks('+z', [0.15, 0.15, 0.45, 0.35], 5));
  mark(m2, MD.stencilMarks('+y', '2', ...onTop(m2, 25.0, 11.7), INK, 3));
  const m3 = mass(25.0, 13.5, 28.0, 15.5, 1.0);
  mark(m3, MD.stencilMarks('+y', '3', ...onTop(m3, 25.4, 15.1), INK, 3), MD.outlineMarks('+y', rectTop(m3, 26.9, 13.8, 27.7, 14.6), SIGNAL), MD.hazardMarks('+z', [-1.5, -0.5, 1.5, -0.25], SIGNAL, INK));
  const m4 = mass(27.0, 10.5, 28.5, 12.0, 0.5); mark(m4, MD.stencilMarks('+y', '4', ...onTop(m4, 27.5, 11.65), INK, 2));
  slab(21.6, 0.06, 14.4, 23.4, 1.0, 15.6, COAT.albedo, { rough: COAT.rough, stuff: COAT.stuff }); ventS(21.7, 23.3, 0.16, 0.9, 15.6, 0.025);   // a condenser: a painted case, a louvre, a fan on top
  round(22.5, 15.0, 1.0, 1.06, 0.42, paint([0.1, 0.1, 0.11], 0.5, 'enamel'));
  for (const [x, z, h] of [[24.2, 14.6, 1.3], [24.2, 15.4, 0.9], [29.2, 12.0, 1.6]] as const) { round(x, z, 0, h, 0.2, GLOSS_W); round(x, z, h, h + 0.07, 0.29, GLOSS_W); }   // vent stacks with caps

  // ---- the stair head: one orange mass
  grp('stair');
  const stair = slab(31.0, 0, 9.0, 34.5, H, 13.5, STUCCO.albedo, { rough: STUCCO.rough, stuff: STUCCO.stuff });
  mark(stair, MD.stencilMarks('+z', 'STAIR', ...onS(stair, 31.98, 2.3), CHALK), MD.stencilMarks('+z', '2', ...onS(stair, 31.35, 0.95), CHALK, 2));   // its name over the door, its number at the door's side
  mark(skinS(32.2, 33.2, 0, 2.05, 13.5, COAT, 0.03), MD.ratingPlateMarks('+z', [-0.15, 0.45, 0.15, 0.65], 9));                                    // a door's face with a notice on it, its kick plate, a vent beside its head
  mark(skinS(32.25, 33.15, 0.02, 0.32, 13.53, STEEL, 0.006), MD.boltMarks('+z', -0.4, 0.4, 0.1, 6, 'base'));
  ventS(31.3, 31.9, 1.9, 2.5, 13.5, 0.025);
  // the board beside the door that says what is on which floor: four bars of colour with a line of print each
  mark(add({ c: [33.75, 1.55, 13.508], h: [0.3, 0.25, 0.008], albedo: GLOSS_W.albedo, rough: GLOSS_W.rough, stuff: GLOSS_W.stuff, flags: FLAT }),
    [markFromRect('+z', 'base', [-0.25, -0.2, 0.25, 0.2], [{ albedo: ORANGE, rough: 0.4 }, { albedo: BLUE, rough: 0.4 }, { albedo: [0.42, 0.42, 0.41] }], c => { for (let r = 0; r < 4; r++) { const j = c.H - 3 - 4 * r; c.fill(1 + (r % 2), 0, j, 5 + 2 * ((r * 3) % 4), j + 2); c.fill(3, 13, j, c.W - 2 - (r * 5) % 4, j + 1); } })!]);
  round(34.2, 13.75, 0, H, 0.09, GLOSS_O); round(34.2, 13.75, 0, 0.05, 0.14, GLOSS_O); round(34.2, 13.75, 1.5, 1.56, 0.125, GLOSS_O);                // a pipe up the corner, a flange at its foot and a collar half way

  // ---- the pad: a painted rectangle of deck with white lines
  grp('pad');
  const pad = film(8, 17.5, 16, 22, PAD);
  film(8, 17.5, 16, 17.6, LINE_W, 0.011); film(8, 21.9, 16, 22, LINE_W, 0.011); film(8, 17.6, 8.1, 21.9, LINE_W, 0.011); film(15.9, 17.6, 16, 21.9, LINE_W, 0.011);
  film(11.5, 18.5, 12.5, 21, LINE_W, 0.011); film(10.5, 19.4, 13.5, 20.1, LINE_W, 0.011);
  mark(pad, MD.stencilMarks('+y', '01', ...onTop(pad, 8.4, 21.6), CHALK, 2));

  // ---- paint on the deck itself: a hatched bay to keep clear before each of the three outer doors, and the way out for whoever comes up
  mark(deck, MD.hazardMarks('+y', rectTop(deck, 7.8, 15.2, 9.4, 16.4), SIGNAL, undefined, 2, 'coarse', 1), MD.hazardMarks('+y', rectTop(deck, 31.9, 13.7, 33.5, 14.9), SIGNAL, undefined, 2, 'coarse', 1),
    MD.hazardMarks('+y', rectTop(deck, 17.3, 11.2, 18.5, 12.8), SIGNAL, undefined, 2, 'coarse', 1), MD.stencilMarks('+y', 'EXIT→', ...onTop(deck, 26.2, 17.5), SIGNAL));
  grp('drain');   // a grated channel across the plant's front, a yellow line either side
  slab(24.6, 0.006, 16.5, 28.4, 0.012, 16.74, DARK.albedo, { rough: 0.5, stuff: 'louvre', flags: FLAT });
  film(24.6, 16.42, 28.4, 16.5, LINE_Y, 0.011); film(24.6, 16.74, 28.4, 16.82, LINE_Y, 0.011);

  // ---- a railing between the pad and the walk to the service door: orange tube, two rails, a post every stride and a half. In the sun its shadow is a ladder on the deck
  grp('rail');
  const tube = (x0: number, y0: number, z0: number, x1: number, y1: number, z1: number) => slab(x0, y0, z0, x1, y1, z1, GLOSS_O.albedo, { rough: GLOSS_O.rough, stuff: GLOSS_O.stuff });
  for (let x = 10.6; x < 16.4; x += 1.425) tube(x - 0.02, 0, 17.18, x + 0.02, 1.05, 17.22);
  for (const z of [18.4, 19.6]) tube(16.28, 0, z - 0.02, 16.32, 1.05, z + 0.02);
  for (const y of [0.52, 1.03]) { tube(10.58, y, 17.18, 16.32, y + 0.04, 17.22); tube(16.28, y, 17.22, 16.32, y + 0.04, 19.62); }

  // ---- what the last job left by the stair: a pallet of spare pavers, two tins and a bucket, a plank; and a carton by the service wall
  grp('clutter');
  mark(add({ c: [30.0, 0.07, 16.0], h: [0.58, 0.07, 0.5], albedo: WOOD.albedo, rough: WOOD.rough, stuff: WOOD.stuff }), MD.palletDeckMarks([0.58, 0.07, 0.5]));
  for (let k = 0; k < 4; k++) add({ c: [29.8 + 0.012 * (k % 2), 0.16 + 0.04 * k, 15.9 + 0.01 * k], h: [0.25, 0.02, 0.25], yaw: [0, 0.05, -0.03, 0.08][k], albedo: PAVER.albedo, rough: PAVER.rough, stuff: COAT.stuff });   // (a loose paver is the box's own: enamel's map, not the deck's joints through it)
  round(30.9, 16.75, 0, 0.19, 0.085, GLOSS_W); round(31.12, 16.6, 0, 0.19, 0.085, GLOSS_W); round(30.6, 17.05, 0, 0.28, 0.14, GLOSS_O);
  slab(28.9, 0.006, 17.3, 31.4, 0.046, 17.5, WOOD.albedo, { rough: WOOD.rough, stuff: WOOD.stuff });
  mark(add({ c: [12.4, 0.2, 15.42], h: [0.25, 0.2, 0.2], albedo: CARD.albedo, rough: CARD.rough, stuff: CARD.stuff }), MD.cartonMarks([0.25, 0.2, 0.2]));

  // ---- lights: tubes in the rooms, a floodlight for a night, the moon (the hour's sun goes in at its place)
  grp('lights');
  G.tube(8.8, 11.0, false, 'service'); G.tube(8.8, 13.4, false, 'service'); G.tube(14.2, 11.0, false, 'lounge'); G.tube(14.2, 13.4, false, 'lounge');
  // (THE TUBES ARE THE ROOMS' LIGHT, each a light of its own, as in every level of the kit. A point lamp of 60 hung between each pair as well until 11-05: over twice the
  // kit's bulb, it put 1.75 times the night's white on the floor under it before any bounce, and a night shows a lit room at the night's own exposure. docs/agents/levels.md)
  grp('exterior.lamps');
  G.floodLight(18.5, 22.8, [0.3, -1], 'flood_deck', true);
  lights.push({ kind: 'dir', pos: [0, 0, 0], dir: norm([0.5, -0.5, -0.7]), color: [0.55, 0.7, 1.0], intensity: 0.9, range: 1e6, innerDeg: 0, outerDeg: 0, name: 'moon', group: 'exterior', enabled: true, volumetric: 0, radius: 0.009, mains: false });

  // ---- the breaker and one beacon: the game's blackout wants both
  grp('breaker');
  const brk = add({ c: [6.9, 1.35, 9.16], h: [0.28, 0.36, 0.04], albedo: [0.42, 0.44, 0.45], name: 'breaker_panel', stuff: 'enamel' });
  const brkLed = add({ c: [6.72, 1.62, 9.21], h: [0.015, 0.015, 0.01], albedo: [0.02, 0.02, 0.02], rough: 0.5, emissive: [0.3, 4, 0.6], flags: BoxFlag.NoShadow });
  grp('beacons');
  const dome = add({ c: [8.8, H - 0.19, 14.4], h: [0.055, 0.07, 0.055], albedo: [0.35, 0.08, 0.05], flags: BoxFlag.NoShadow | BoxFlag.NoPrimary });
  K.switchables.push({ name: 'beacon0', boxes: [dome], group: 'emergency', on: false, baseEmissive: [9, 1.2, 0.35], mains: false });

  // ---- doors (closedDir 0: the leaf runs +x from the hinge along a z-wall; π/2: +z along an x-wall)
  const doors: LevelDoorDef[] = [
    { name: 'service', hinge: [8.02, 15.0], closedDir: 0, width: 1.16, exterior: true },
    { name: 'lounge', hinge: [17.0, 11.42], closedDir: HALF, width: 1.16, exterior: true },
    { name: 'between', hinge: [11.5, 11.42], closedDir: HALF, width: 1.06 },
  ];
  for (const d of doors) doorFrame(d);

  const SERVICE: Rect = { x0: 6.12, x1: 11.44, z0: 9.12, z1: 14.88 };
  const level: Level = {
    boxes: K.boxes, props: K.props, lights, switchables: K.switchables,
    doors,
    devices: [],
    breaker: { pos: [6.9, 1.35, 9.3], boxes: [brk, brkLed] },
    beacons: [{ pos: [8.8, H - 0.3, 14.4], box: dome }],
    playerSpawn: [19.0, 0, 19.5],
    routes: [], chokepoints: [], clearPlans: [],
    sky: { zenith: [0.015, 0.025, 0.05], horizon: [0.045, 0.05, 0.07] },
    // the stealth mission's three places, because the level format wants them
    mission: { serverRoom: { ...SERVICE }, entryDoor: 'service', rack: { index: 0, front: [8.0, 1.0, 9.7], halfW: 0.3 }, exfilDoor: 'lounge', exfilX: 18.5, exfilZ: 12.0, exfilOut: [1, 0] },
  };
  // ---- what is underfoot (game/floor.ts): pavers and a skimmed floor are stone, the lounge a carpet
  const surfaces: SurfacesData = {
    building: { x0: 5, x1: 35, z0: 5, z1: 23.5 }, grassEdge: 0, inside: 'stone', outside: 'stone', verge: 'stone',
    zones: [{ id: 'lounge', mat: 'carpet', x0: 11.5, z0: 9, x1: 17, z1: 15 }],
  };
  // ---- how the roof is walked (scene/ways.ts: the look's page wears its floors by it). Nobody patrols a roof and no team is staged on this one, so its ways
  // are all its author's: whoever comes up comes by the stair, a fitter goes to the plant and to the service room, the others to the lounge's door and
  // through the lounge. A way's ends are names, so the roof names three of its places (these are words the game may say of this level: game/dialogue.ts)
  const places = { areas: [{ name: 'stair landing', bounds: { x0: 31, x1: 34.5, z0: 13.6, z1: 15.6 } }, { name: 'plant pad', bounds: { x0: 8, x1: 16, z0: 17.5, z1: 22 } }, { name: 'service room', bounds: { ...SERVICE } }] };
  const walked = [{ from: 'stair landing', to: 'lounge', weight: 3 }, { from: 'stair landing', to: 'service', weight: 3 }, { from: 'stair landing', to: 'plant pad', weight: 2 }, { from: 'service', to: 'plant pad', weight: 2 }, { from: 'service room', to: 'between' }, { from: 'between', to: 'lounge' }];
  return { level, scenarios: { places, walked, doorNames: [{ door: 'service', name: 'service door' }, { door: 'lounge', name: 'lounge door' }, { door: 'between', name: 'service to lounge door' }] }, groups: trace.groups, surfaces };
}
