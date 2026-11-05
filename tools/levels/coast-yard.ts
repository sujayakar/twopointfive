// COAST-YARD: THE COAST SKIN'S SHOWROOM (tools/levels/coastSkin.ts, src/scene/gwotPalette.ts COAST_PALETTE) — NOT a level to play and not in the missions
// list. A stretch of sea front in full sun: a beach and a timber deck, a paved promenade with a lawn, a rendered kiosk with a café terrace under
// parasols, a bin shelter, a bike rack, and a road with a kerb and bollards. It stands the kit's pieces that belong to no one setting, and the street and
// garden pieces, IN THE COAST SKIN: every one of them is the kit's own call, laid over the skin's host, so this file is also the proof that a piece
// written once has two skins (?levelFile=coast-yard.json&solo&hour=midday). What the generator lays itself (the ground, the deck, the shelter's roof)
// it lays in the coast palette's own rows.
//
//        x 0        10              20        26       33      40
//   z 0  ┌─────────────────────────────────────────────────────┐
//        │  sand      [  the deck: loungers  ]     sand        │      the beach
//   z 6  ├╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌ kerb ╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌┤
//        │  lawn    │ wall   stores      ┌─kiosk──┐  bins       │
//   z 15 │  beds    │                    └──door──┘  shelter    │      the promenade (paving)
//        │          gate      bikes      café: parasols         │
//   z 22 │  planters ·  ·  ·  bollards  ·  ·  ·  planters       │
//   z 23 ├═════════════════════ kerb ═══════════════════════════┤
//        │  the road: a van, a pickup, a motorbike, barriers     │
//   z 28 └─────────────────────────────────────────────────────┘
import type { Level, LevelDoorDef } from '../../src/scene/level';
import type { LevelScenarios, Rect } from '../../src/scene/scenarios';
import type { SurfacesData } from '../../src/scene/levelData';
import { officeKit, normalize3 as norm, type GeneratorTrace } from '../../src/scene/officeGen';
import { gwotKit, gwotOfOffice } from '../../src/scene/gwotKit';
import { COAST_PALETTE, COAST_FINISH, COAST_MATERIALS, coastAs, type CoastMat } from '../../src/scene/gwotPalette';
import { WORLD } from '../../src/scene/world';
import { BoxFlag } from '../../src/scene/boxes';
import { PANEL_GREY } from '../../src/scene/stuff';
import { coastHost } from './coastSkin';

export const NAME = 'coast-yard';
export const NOTES = 'The coast skin\'s showroom: a sea front with the kit\'s setting-neutral pieces and the street and garden pieces, each the kit\'s own call laid over tools/levels/coastSkin.ts. Regenerate: bun run tools/levels/export.ts coast-yard';
/** the coast palette goes into the file's `materials` (tools/levels/export.ts) */
export const MATERIALS = COAST_MATERIALS;

/** `skin` false = the same sea front with the kit's pieces in the kit's own paint (tools/qa/coast-check.ts lays the two beside each other: the skin moves
 *  no box and changes nothing a round reads). The exporter builds it skinned */
export function build(skin = true): { level: Level; scenarios: LevelScenarios; groups: string[]; surfaces: SurfacesData } {
  const trace: GeneratorTrace = { groups: [] };
  const K = officeKit(trace), host = gwotOfOffice(K), G = gwotKit(skin ? coastHost(host) : host), C = COAST_PALETTE, H = WORLD.ceilingY;
  const { grp, add, slab, doorFrame, lights, switchables } = K;
  const HALF = Math.PI / 2;
  /** a coast row laid by this file: its paint and its finish */
  const row = (m: CoastMat) => ({ rough: COAST_FINISH[m].rough, ...(COAST_FINISH[m].metal ? { metal: true } : {}) });

  // ---- ground: paving to the horizon, sand along the north, the road along the south, a lawn ----
  grp('ground');
  slab(0, -0.25, 0, 40, 0, 28, C.paverGrey, row('paverGrey'));
  G.ground(0, 6.4, 40, 22.9, 'dust');                                           // the promenade: warm paving (the kit's ground, in the skin)
  grp('ground.beach');
  slab(0, 0, 0, 40, 0.006, 6.1, C.sand, { ...row('sand'), flags: BoxFlag.NoShadow });
  slab(1.6, 0, 8.6, 8.6, 0.008, 19.6, C.lawn, { ...row('lawn'), flags: BoxFlag.NoShadow });
  grp('ground.road');
  slab(0, 0, 23.3, 40, 0.006, 28, C.roadTarmac, { ...row('roadTarmac'), flags: BoxFlag.NoShadow });
  G.kerb(0.2, 23.1, 39.8, 23.1, 'block', [9, 10]);                              // the road's kerb, dropped where the gate's drive crosses
  G.kerb(0.2, 6.25, 39.8, 6.25, 'tWall');                                       // the sea wall's edge
  G.patch(21, 25.4, 2.4, 1.4, 'oil', 0.2);
  grp('deck');
  slab(10, 0, 1.4, 22, 0.12, 5.6, C.deck, row('deck'));                         // the timber deck on the sand
  slab(15, 0, 5.6, 17, 0.06, 6.4, C.deckGrey, row('deckGrey'));                 // its step down to the promenade
  grp('floors');
  slab(26, 0, 9, 33, 0.006, 15, C.paleSlab, { ...row('paleSlab'), flags: BoxFlag.NoShadow });
  grp('ceiling');
  slab(25.9, H, 8.9, 33.1, H + 0.2, 15.1, C.skimWhite, { flags: BoxFlag.NoPrimary, rough: COAST_FINISH.skimWhite.rough });

  // ---- the garden wall (white render, coped) with its gate, and the lawn's beds ----
  grp('walls.garden');
  G.yardWallZ(10, 8, 20, 'mud', [{ a: 13.4, b: 14.6, y0: 0, y1: 2.2 }]); G.yardWallX(8, 1.5, 10.12);
  for (const [x, z] of [[10, 8], [10, 20], [1.5, 8]] as const) G.pier(x, z);
  G.gate(10, 14, 'z', 1.2, -1, 'paintBlue', 75, 2.2);
  grp('garden');
  G.plantBed(5.0, 11.0, 4.4, 1.3, 0, 3); G.plantBed(5.0, 16.6, 4.4, 1.3, 0, 2);
  G.bush(2.6, 18.6, 1.2); G.bush(7.8, 9.4, 0.9); G.bush(2.4, 9.6, 1.0);
  G.tub(8.9, 12.6); G.tub(8.9, 15.4, 'mudLight'); G.lounger(4.4, 13.9, 0.3, 'rugNavy'); G.plankBench(7.4, 18.9, 0.1);

  // ---- the kiosk: white render, its east wall terracotta, a door and a window to the terrace, a side door ----
  grp('walls.kiosk');
  G.wallX(9, 25.88, 33.12, 'mud'); G.wallZ(26, 9.12, 14.88, 'mud', [{ a: 11.4, b: 12.6 }]);
  G.wallX(15, 25.88, 33.12, 'mud', [{ a: 27.6, b: 28.8 }, { a: 30.0, b: 31.6, y0: 0.95, y1: 2.0 }]);
  K.wallZ(33, 9.12, 14.88, 0.24, coastAs('terracotta', 'concrete'), [], { rough: COAST_FINISH.terracotta.rough });
  G.shutters('x', 15, 30.0, 31.6, 1, 'open', 'paintBlue', 0.95, 2.0); G.spout('x', 9, 29.5, -1);
  G.dado('x', 15, 26.12, 27.58, -1, 'paintBlue'); G.dado('x', 15, 28.82, 32.88, -1, 'paintBlue');
  grp('room.kiosk');
  G.table(29.6, 11.2, 1.6, 0.75, 0); G.plasticChair(29.2, 12.0, Math.PI); G.crates(32.2, 10.0, 0, 2, 'paintBlue'); G.gasBottle(26.6, 9.7); G.jerryCan(27.2, 9.6, 0.2, 'jerryYellow');
  G.tyres(32.3, 14.2, 2);

  // ---- the café terrace: three parasols, tables and chairs under them ----
  grp('terrace');
  G.parasol(25.2, 17.6); G.parasol(29.0, 18.4, 1.3, 'tarpBlue'); G.parasol(32.8, 17.6, 1.3, 'rugRed');
  for (const [x, z, a] of [[25.2, 17.6, 0.2], [29.0, 18.4, -0.3], [32.8, 17.6, 0.5]] as const) { G.table(x + 0.5, z + 0.1, 0.9, 0.7, a); G.plasticChair(x + 0.5, z + 0.95, Math.PI + a); G.plasticChair(x + 0.5, z - 0.75, a); G.plasticChair(x - 0.45, z + 0.1, HALF + a, 'plasticGreen'); }
  G.tub(27.2, 15.6); G.tub(29.25, 15.6); G.planter(34.6, 20.6, HALF, 2.4, 0.6); G.planter(23.0, 20.9, 0.1, 2.0, 0.6, 'mud', false);

  // ---- the bin shelter: four steel posts under a tiled roof, three bins ----
  grp('shelter');
  for (const [x, z] of [[35.0, 9.4], [38.6, 9.4], [35.0, 11.8], [38.6, 11.8]] as const) add({ c: [x, 1.15, z], h: [0.04, 1.15, 0.04], albedo: C.brushedSteel, ...row('brushedSteel') });
  slab(34.7, 2.3, 9.1, 38.9, 2.38, 12.1, C.roofTile, row('roofTile'));
  G.wheelieBin(35.7, 10.6, 0); G.wheelieBin(35.4, 13.0, 0.05, 'plasticBlue'); G.wheelieBin(36.3, 13.1, -0.1, 'plasticBlack'); G.wheelieBin(37.3, 13.0, 0.15);   // (one under the roof, three stood out for the camera)

  // ---- the stores beside the kiosk: what a place keeps round the back ----
  grp('stores');
  G.drums(22.4, 9.0, 3); G.drumDown(24.3, 10.4, 0.3, 'drumBlue'); G.crates(20.6, 8.9, 0.1, 2); G.blocks(19.0, 9.0, 0.1, 2); G.bricks(17.6, 9.1, 0.2);
  G.pipes(21.0, 11.4, 0.1); G.log(18.4, 11.0, 0.4, 1.2); G.tyres(16.4, 9.2, 3); G.generator(23.6, 13.0, HALF); G.jerryCan(22.6, 13.6, 0.5, 'jerryYellow');
  G.waterTank(13.0, 9.4, 'plasticWhite', 1.1); G.pump(15.0, 9.6, 0.3); G.satDish(25.5, 8.3, 0.4); G.ladder('z', 26, 10.2, -1);
  G.handcart(18.6, 13.4, 0.4, 'produce'); G.tWall(12.2, 12.6, 2, HALF, 2.2);

  // ---- the promenade: the bike rack, planters and bollards along the road's edge, a container as a beach store ----
  grp('promenade');
  add({ c: [15.0, 0.75, 20.9], h: [1.6, 0.02, 0.02], albedo: C.brushedSteel, ...row('brushedSteel') }); for (const x of [13.4, 16.6]) add({ c: [x, 0.365, 20.9], h: [0.018, 0.365, 0.018], albedo: C.brushedSteel, ...row('brushedSteel') });
  G.bicycle(14.0, 20.5, HALF + 0.1); G.bicycle(15.0, 20.5, HALF - 0.05, 'drumRed'); G.bicycle(16.0, 20.5, HALF + 0.05, 'genYellow');
  G.planter(4.0, 22.2, 0, 2.4, 0.6); G.planter(8.0, 22.2, 0, 2.4, 0.6, 'mud'); G.planter(37.0, 22.2, 0, 2.4, 0.6);
  G.bollard(20.0, 22.5, 9, 1.6); G.bollard(12.0, 22.5, 2, 1.6, 0, 'genYellow');
  G.jersey(11.6, 6.9, 2, 0); G.kerbStones(11, 20.3, 19, 20.3);

  // ---- the beach and the deck: loungers, a parasol, towels ----
  grp('beach');
  G.lounger(12.0, 2.6, HALF, 'rugRed'); G.lounger(13.4, 2.6, HALF, 'rugNavy'); G.lounger(19.4, 2.8, HALF + 0.15); G.parasol(16.4, 3.0, 1.5, 'rugCream');
  G.lounger(27.0, 3.2, HALF - 0.2, 'rugOchre'); G.parasol(28.4, 3.0, 1.2, 'tarpBlue'); G.lounger(5.0, 3.4, HALF + 0.3, 'rugGreen'); G.plasticChair(30.0, 4.0, 2.6);
  G.rug(8.0, 3.0, 1.8, 0.9, 'mat', 0.2); G.container(35.4, 3.0, 0, 'paintBlue');                 // a container as the beach's store

  // ---- the road: a van, a pickup, a motorbike, barriers ----
  grp('road');
  G.van(8.0, 25.2, 0.03); G.pickup(27.0, 25.8, Math.PI, 'truckTan', { load: 'crates' }); G.motorbike(17.0, 24.2, 0.2); G.jersey(34.0, 24.4, 1, 0.3);

  // ---- lights: a bulb in the kiosk, a floodlight over the terrace, the moon (the page's hour sets the sun) ----
  grp('lights');
  G.bulb(29.5, 12.0, 'bulb_kiosk', 'kiosk', true);
  grp('exterior.lamps');
  G.floodLight(33.4, 15.6, [-1, 0.6], 'flood_terrace', true);
  lights.push({ kind: 'dir', pos: [0, 0, 0], dir: norm([0.45, -0.5, -0.74]), color: [0.55, 0.7, 1.0], intensity: 0.9, range: 1e6, innerDeg: 0, outerDeg: 0, name: 'moon', group: 'exterior', enabled: true, volumetric: 0, radius: 0.009, mains: false });

  // ---- the breaker (the kiosk's north wall, inside) and one emergency beacon: the game's blackout wants both ----
  grp('breaker');
  const brk = add({ c: [31.5, 1.35, 9.16], h: [0.28, 0.36, 0.04], albedo: PANEL_GREY, name: 'breaker_panel' });
  const brkLed = add({ c: [31.32, 1.62, 9.21], h: [0.015, 0.015, 0.01], albedo: C.plasticDark, rough: 0.5, emissive: [0.3, 4, 0.6], flags: BoxFlag.NoShadow });
  grp('beacons');
  add({ c: [29.5, H - 0.06, 13.6], h: [0.07, 0.06, 0.07], albedo: [0.2, 0.2, 0.22], flags: BoxFlag.NoShadow | BoxFlag.NoPrimary });
  const dome = add({ c: [29.5, H - 0.19, 13.6], h: [0.055, 0.07, 0.055], albedo: [0.35, 0.08, 0.05], flags: BoxFlag.NoShadow | BoxFlag.NoPrimary });
  switchables.push({ name: 'beacon0', boxes: [dome], group: 'emergency', on: false, baseEmissive: [9, 1.2, 0.35], mains: false });

  // ---- doors (closedDir 0: the leaf runs +x from the hinge along a z-wall; π/2: +z along an x-wall) ----
  const doors: LevelDoorDef[] = [
    { name: 'k_front', hinge: [27.62, 15.0], closedDir: 0, width: 1.16, exterior: true },
    { name: 'k_side', hinge: [26.0, 11.42], closedDir: HALF, width: 1.16, exterior: true },
  ];
  for (const d of doors) doorFrame(d);

  const KIOSK: Rect = { x0: 26.12, x1: 32.88, z0: 9.12, z1: 14.88 };
  const level: Level = {
    boxes: K.boxes, props: K.props, lights, switchables,
    doors,
    devices: [],
    breaker: { pos: [31.5, 1.35, 9.3], boxes: [brk, brkLed] },
    beacons: [{ pos: [29.5, H - 0.3, 13.6], box: dome }],
    playerSpawn: [20.0, 0, 18.5],
    routes: [], chokepoints: [], clearPlans: [],
    sky: { zenith: [0.015, 0.025, 0.05], horizon: [0.045, 0.05, 0.07] },
    // the stealth mission's three places, because the level format wants them
    mission: { serverRoom: { ...KIOSK }, entryDoor: 'k_front', rack: { index: 0, front: [31.5, 1.0, 10.2], halfW: 0.3 }, exfilDoor: 'k_side', exfilX: 24.5, exfilZ: 12.0, exfilOut: [-1, 0] },
  };
  // ---- what is underfoot (game/floor.ts): paving and the kiosk's slab are stone, the road asphalt, the lawn grass; the six have no word for sand or timber
  const surfaces: SurfacesData = {
    building: { x0: 26, x1: 33, z0: 9, z1: 15 }, grassEdge: 0, inside: 'stone', outside: 'stone', verge: 'stone',
    zones: [
      { id: 'road', mat: 'asphalt', x0: 0, z0: 23.3, x1: 40, z1: 28 },
      { id: 'lawn', mat: 'grass', x0: 1.6, z0: 8.6, x1: 8.6, z1: 19.6 },
      { id: 'beach', mat: 'asphalt', x0: 0, z0: 0, x1: 40, z1: 6.1 },
    ],
  };
  return { level, scenarios: { doorNames: [{ door: 'k_front', name: 'kiosk door' }, { door: 'k_side', name: 'kiosk side door' }] }, groups: trace.groups, surfaces };
}
