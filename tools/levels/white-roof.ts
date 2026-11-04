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
import type { Vec3 } from '../../src/math/vec';

export const NAME = 'white-roof';
export const NOTES = 'A look study: a kept roof in full sun, every surface one flat colour and one of the ten kept stuffs (render/lookTex.ts). Not in the missions list. Regenerate: bun run tools/levels/export.ts white-roof';
export const MATERIALS = GWOT_MATERIALS;

// ---- the paints: each ONE flat colour (linear), with the finish its stuff is written around (lookTex.ts LOOK_ROUGH)
type Paint = Pick<Box, 'albedo' | 'rough'> & { stuff: NonNullable<Box['stuff']> };
const paint = (albedo: Vec3, rough: number, stuff: Paint['stuff']): Paint => ({ albedo, rough, stuff });
const ORANGE: Vec3 = [0.88, 0.24, 0.03], YELLOW: Vec3 = [0.88, 0.6, 0.04], BLUE: Vec3 = [0.04, 0.2, 0.72];
const PANEL = paint([0.56, 0.56, 0.54], 0.68, 'cast panel'), PAVER = paint([0.72, 0.73, 0.74], 0.5, 'paver'),   // (cast concrete a step under the white deck it stands on: ground, walls and things sit a step apart before any light falls)
   BLOCK = paint([0.76, 0.77, 0.77], 0.4, 'gloss block'), SKIM = paint([0.78, 0.78, 0.77], 0.55, 'skim');
const STUCCO = paint(ORANGE, 0.42, 'stucco'), BOARD = paint(YELLOW, 0.42, 'board concrete'), LOUVRE = paint([0.5, 0.51, 0.52], 0.45, 'louvre');
const STEEL = paint([0.56, 0.57, 0.58], 0.32, 'brushed');   // (not `metal`: a mirror of the bounce's metre-wide probes on a hand-sized plate is blotches, on the software adapter at least)
const COAT = paint([0.7, 0.71, 0.72], 0.35, 'enamel'), CARPET = paint(BLUE, 0.95, 'carpet'), SEAT = paint([0.03, 0.14, 0.55], 0.95, 'carpet');
const GLOSS_Y = paint(YELLOW, 0.26, 'enamel'), GLOSS_W = paint([0.8, 0.8, 0.8], 0.3, 'enamel'), GLOSS_O = paint(ORANGE, 0.3, 'enamel');
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
  const mass = (x0: number, z0: number, x1: number, z1: number, h: number, p: Paint = PANEL) => { slab(x0, 0, z0, x1, h, z1, p.albedo, { rough: p.rough, stuff: p.stuff }); edged(x0, z0, x1, z1, h); };
  /** a round thing standing up: the kit's octagon of four slabs, which the game stands up as one round (scene/round.ts) */
  const round = (x: number, z: number, y0: number, y1: number, r: number, p: Paint) => {
    const a = r * 0.96, t = a * 0.4142;
    for (let k = 0; k < 4; k++) add({ c: [x, (y0 + y1 - 0.002 * k) / 2, z], h: [a, (y1 - 0.002 * k - y0) / 2, t], yaw: k * Math.PI / 4, albedo: p.albedo, rough: p.rough, stuff: p.stuff });
  };
  /** a skin on a wall's south face: a band of another stuff a finger proud of it (a plinth, a steel band, a louvre, a door's plate) */
  const skinS = (x0: number, x1: number, y0: number, y1: number, zFace: number, p: Paint, proud = 0.012) => slab(x0, y0, zFace, x1, y1, zFace + proud, p.albedo, { rough: p.rough, stuff: p.stuff });

  // ---- the deck: pavers wall to wall, and the dark beyond the parapet (a roof ends in air)
  grp('ground');
  slab(0, -0.25, 0, 40, 0, 28, VOID.albedo, { rough: VOID.rough, stuff: VOID.stuff });
  grp('floors');
  slab(4.88, 0, 4.88, 35.12, 0.006, 23.62, PAVER.albedo, { rough: PAVER.rough, stuff: PAVER.stuff, flags: FLAT });

  // ---- the hall's wall: the north side, cast in panels, on an orange plinth
  grp('walls.hall');
  K.wallX(5, 4.88, 35.12, SHELL, PANEL.albedo, [], { rough: PANEL.rough, stuff: PANEL.stuff }, H);
  skinS(5.12, 34.88, 0, 0.75, 5.12, STUCCO, 0.02);
  for (const x of [10, 20, 30]) { slab(x - 0.25, 0, 5.12, x + 0.25, H, 5.42, PANEL.albedo, { rough: PANEL.rough, stuff: PANEL.stuff }); skinS(x - 0.25, x + 0.25, 0, 0.75, 5.42, STUCCO, 0.02); }   // three piers: the wall is not one sheet

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
  grp('room.service');
  for (const [x, z] of [[6.32, 9.32], [11.24, 9.32], [6.32, 14.68]] as const) slab(x - 0.2, 0, z - 0.2, x + 0.2, H, z + 0.2, STUCCO.albedo, { rough: STUCCO.rough, stuff: STUCCO.stuff });   // orange piers in three corners (the fourth is the door's)
  for (const x of [7.4, 8.0]) { slab(x - 0.22, 0.08, 9.12, x + 0.22, 1.0, 9.36, STEEL.albedo, { rough: STEEL.rough, stuff: STEEL.stuff }); skinS(x - 0.12, x + 0.12, 0.45, 0.85, 9.36, LINE_Y, 0.004); }   // two steel cabinets, a yellow label each
  slab(9.4, 0, 9.12, 10.6, 0.9, 9.6, COAT.albedo, { rough: COAT.rough, stuff: COAT.stuff }); skinS(9.5, 10.5, 0.15, 0.75, 9.6, LOUVRE, 0.02);   // an air handler under the far wall, a louvre in its face
  grp('room.lounge');
  slab(11.56, 0, 9.12, 16.88, H, 9.15, STUCCO.albedo, { rough: STUCCO.rough, stuff: STUCCO.stuff });                                              // the one orange wall
  for (const z of [10.2, 13.6]) { slab(13.2, 0, z - 0.3, 15.2, 0.4, z + 0.3, SEAT.albedo, { rough: SEAT.rough, stuff: SEAT.stuff }); slab(13.2, 0.4, z - 0.3, 15.2, 0.75, z - 0.18, SEAT.albedo, { rough: SEAT.rough, stuff: SEAT.stuff }); }   // two benches
  slab(13.7, 0, 11.6, 14.7, 0.36, 12.2, COAT.albedo, { rough: 0.3, stuff: COAT.stuff });                                                            // a low table between them

  // ---- the yellow wall: cast against boards, a steel band along it, two pipes up it
  grp('walls.yellow');
  K.wallX(9, 19.5, 29.5, SHELL, BOARD.albedo, [], { rough: BOARD.rough, stuff: BOARD.stuff });
  skinS(19.5, 29.5, 0, 0.12, 9.12, PANEL, 0.02);                                                                                                  // a grey skirting
  skinS(19.5, 29.5, 1.1, 1.32, 9.12, STEEL, 0.015);
  round(20.1, 9.32, 0, H, 0.11, GLOSS_Y); round(20.45, 9.32, 0, H, 0.11, GLOSS_Y);

  // ---- the plant: four cast volumes of four heights, a louvre and a hatch in their faces, round stacks beside them
  grp('plant');
  mass(21.5, 10.0, 24.5, 13.0, 2.4); skinS(22.0, 24.0, 0.3, 1.5, 13.0, LOUVRE, 0.03);
  mass(24.5, 10.0, 27.0, 12.0, 1.5); skinS(25.2, 26.3, 0.1, 1.2, 12.0, STEEL, 0.015);
  mass(25.0, 13.5, 28.0, 15.5, 1.0);
  mass(27.0, 10.5, 28.5, 12.0, 0.5);
  slab(21.6, 0.06, 14.4, 23.4, 1.0, 15.6, COAT.albedo, { rough: COAT.rough, stuff: COAT.stuff }); skinS(21.7, 23.3, 0.16, 0.9, 15.6, LOUVRE, 0.025);   // a condenser: a painted case, a louvre, a fan on top
  round(22.5, 15.0, 1.0, 1.06, 0.42, paint([0.1, 0.1, 0.11], 0.5, 'enamel'));
  for (const [x, z, h] of [[24.2, 14.6, 1.3], [24.2, 15.4, 0.9], [29.2, 12.0, 1.6]] as const) { round(x, z, 0, h, 0.2, GLOSS_W); round(x, z, h, h + 0.07, 0.29, GLOSS_W); }   // vent stacks with caps

  // ---- the stair head: one orange mass
  grp('stair');
  slab(31.0, 0, 9.0, 34.5, H, 13.5, STUCCO.albedo, { rough: STUCCO.rough, stuff: STUCCO.stuff });
  skinS(32.2, 33.2, 0, 2.05, 13.5, COAT, 0.03); skinS(32.25, 33.15, 0.02, 0.32, 13.53, STEEL, 0.006); skinS(31.3, 31.9, 1.9, 2.5, 13.5, LOUVRE, 0.025);   // a door's face with its kick plate, a vent beside its head
  round(34.2, 13.75, 0, H, 0.09, GLOSS_O);

  // ---- the pad: a painted rectangle of deck with white lines
  grp('pad');
  film(8, 17.5, 16, 22, PAD);
  film(8, 17.5, 16, 17.6, LINE_W, 0.011); film(8, 21.9, 16, 22, LINE_W, 0.011); film(8, 17.6, 8.1, 21.9, LINE_W, 0.011); film(15.9, 17.6, 16, 21.9, LINE_W, 0.011);
  film(11.5, 18.5, 12.5, 21, LINE_W, 0.011); film(10.5, 19.4, 13.5, 20.1, LINE_W, 0.011);

  // ---- lights: tubes in the rooms, a floodlight for a night, the moon (the hour's sun goes in at its place)
  grp('lights');
  G.tube(8.8, 11.0, false, 'service'); G.tube(8.8, 13.4, false, 'service'); G.tube(14.2, 11.0, false, 'lounge'); G.tube(14.2, 13.4, false, 'lounge');
  for (const [x, z, name, group] of [[8.8, 12.2, 'lamp_service', 'service'], [14.2, 12.2, 'lamp_lounge', 'lounge']] as const)
    lights.push({ kind: 'point', pos: [x, H - 0.25, z], dir: [0, -1, 0], color: [1.0, 0.98, 0.95], intensity: 60, range: 9, innerDeg: 0, outerDeg: 180, name, group, enabled: true, volumetric: 0.1, radius: 0.3 });
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
  return { level, scenarios: { doorNames: [{ door: 'service', name: 'service door' }, { door: 'lounge', name: 'lounge door' }, { door: 'between', name: 'service to lounge door' }] }, groups: trace.groups, surfaces };
}
