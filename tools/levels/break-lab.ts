// BREAK-LAB: the floor the breaking of things is looked at on (src/host/breaklab.ts). One flat lot, nothing to play: FOUR COLUMNS, ONE A MATERIAL, west to
// east (steel, stone, wood, soft stuff), three exhibits down each (small things, a heavy thing, a thing that stands), the soft column's first a hut (a wall
// and a door for the charges). Every exhibit stands eight metres or more from the next, because a grenade's outermost ring is 6.8 m: one grenade is one
// exhibit's. On the kit's showroom yard (gwot-yard.json) any burst broke a dozen things at once and nobody could tell what to look at. Where each stands, and
// what the lab says of it, is ONE table: src/host/breaklabPlan.ts LAB_EXHIBITS. NOT a level to play and not in the missions list.
//
//         x 5.5         15            24.5           34
//   z 6     trunk, drums  blocks, bags  table, crates  the hut
//   z 13.5  pickup        barriers      stall          sacks
//   z 21    container     blast wall    rope bed       cushions, cot
//                         Sam starts at the foot of the lot (z 25), looking up it
//
//   bun run tools/levels/export.ts break-lab [--check]     → levels/break-lab.json
//   page: /breaklab   (= ?levelFile=break-lab.json&solo&nofow&breaklab)
import type { Level, LevelDoorDef } from '../../src/scene/level';
import type { LevelScenarios, Rect } from '../../src/scene/scenarios';
import type { SurfacesData } from '../../src/scene/levelData';
import { officeKit, normalize3 as norm, type GeneratorTrace } from '../../src/scene/officeGen';
import { gwotKit, gwotOfOffice, GWOT_PALETTE, GWOT_MATERIALS } from '../../src/scene/gwotKit';
import { WORLD } from '../../src/scene/world';
import { BoxFlag } from '../../src/scene/boxes';
import { LAB_COLS as C, LAB_ROWS as R } from '../../src/host/breaklabPlan';

export const NAME = 'break-lab';
export const NOTES = 'The break lab\'s floor: four columns of exhibits, one a material (steel, stone, wood, soft stuff), a hut, a machine and sandbags, each well apart from the next (src/host/breaklabPlan.ts). Regenerate: bun run tools/levels/export.ts break-lab';
export const MATERIALS = GWOT_MATERIALS;

export function build(): { level: Level; scenarios: LevelScenarios; groups: string[]; surfaces: SurfacesData } {
  const trace: GeneratorTrace = { groups: [] };
  const K = officeKit(trace), G = gwotKit(gwotOfOffice(K)), P = GWOT_PALETTE, H = WORLD.ceilingY;
  const { grp, add, slab, doorFrame, lights, switchables } = K;
  const HALF = Math.PI / 2;

  // ---- ground: dust to the edge, packed dirt under the lot, a gravel strip at its foot ----
  grp('ground');
  slab(0, -0.25, 0, 40, 0, 28, P.dust, { rough: 1 });
  G.ground(3, 3, 37.5, 23.5, 'dirt');
  G.ground(3, 23.5, 37.5, 26.5, 'gravel');
  for (const x of Object.values(C)) for (const z of R) if (!(x === C.soft && z === R[0])) G.ground(x - 3.2, z - 2.6, x + 3.2, z + 2.6, 'gravel', 0.004);   // a gravel pad an exhibit: the eye finds the rows
  grp('floors');
  G.ground(31.5, 4, 36, 8.5, 'slab', 0.006);                                    // the hut: a bare slab
  grp('ceiling');
  slab(31.4, H, 3.9, 36.1, H + 0.2, 8.6, P.render, { flags: BoxFlag.NoPrimary, rough: 0.9 });

  // ---- the hut: mud-brick to the north and west, cement render to the south and east, a door in the south wall and one in the west ----
  grp('walls.hut');
  G.wallX(4, 31.38, 36.12, 'mud'); G.wallX(8.5, 31.38, 36.12, 'render', [{ a: 31.9, b: 33.1 }]);
  G.wallZ(31.5, 4.12, 8.38, 'mud', [{ a: 5.6, b: 6.8 }]); G.wallZ(36, 4.12, 8.38, 'render');
  grp('lab.hut');
  G.table(33.0, 5.0, 1.6, 0.75, 0); G.crates(35.2, 4.8, 0, 2); G.sacks(35.2, 7.5, 0, 3); G.crates(34.2, 7.7, 0, 1);

  // ---- steel ----
  grp('lab.steel');
  G.trunk(C.steel - 0.7, R[0], 0, 'paintGreen'); G.drums(C.steel + 0.6, R[0], 3);
  G.pickup(C.steel, R[1], Math.PI, 'truckWhite');
  G.container(C.steel, R[2], HALF, 'paintRust');
  // ---- stone ----
  grp('lab.stone');
  G.blocks(C.stone - 0.9, R[0], 0, 2); G.sandbags(C.stone + 0.8, R[0], 1.6, 0, 3);
  G.jersey(C.stone, R[1], 2, 0);
  G.tWall(C.stone, R[2], 3, 0);
  // ---- wood ----
  grp('lab.wood');
  G.table(C.wood - 0.5, R[0], 1.6, 0.75, 0); G.crates(C.wood + 1.1, R[0], 0, 2);
  G.stall(C.wood, R[1], Math.PI, 2.2, 'tarpBlue');
  G.charpoy(C.wood - 0.4, R[2], 0, null); G.woodPile(C.wood + 1.5, R[2], HALF);
  // ---- soft stuff ----
  grp('lab.soft');
  G.sacks(C.soft - 0.5, R[1], 0, 4); G.sacks(C.soft + 0.6, R[1] + 0.1, 0.2, 2);
  G.toshak(C.soft, R[2] - 0.6, 1.9, 0); G.cot(C.soft, R[2] + 0.9, 0);

  // ---- light to see every piece by: a high moon and three floods over the lot (bare lights: a lab, not a place), a bulb in the hut ----
  grp('lights');
  G.bulb(33.8, 6.3, 'bulb_hut', 'hut', true);
  lights.push({ kind: 'dir', pos: [0, 0, 0], dir: norm([0.3, -0.85, -0.42]), color: [0.55, 0.7, 1.0], intensity: 2.6, range: 1e6, innerDeg: 0, outerDeg: 0, name: 'moon', group: 'exterior', enabled: true, volumetric: 0, radius: 0.009, mains: false });
  for (const [name, x, z] of [['lab_west', 10, 13.5], ['lab_mid', 20, 13.5], ['lab_east', 30, 15]] as const)
    lights.push({ kind: 'spot', pos: [x, 6.5, z], dir: [0, -1, 0], color: [1.0, 0.9, 0.74], intensity: 300, range: 24, innerDeg: 50, outerDeg: 78, name, group: 'exterior', enabled: true, volumetric: 0, radius: 0.2, mains: false, broad: true });

  // ---- the breaker (the hut's north wall, inside) and one emergency beacon: the game's blackout wants both ----
  grp('breaker');
  const brk = add({ c: [34.6, 1.35, 4.16], h: [0.28, 0.36, 0.04], albedo: [0.42, 0.44, 0.45], name: 'breaker_panel' });
  const brkLed = add({ c: [34.42, 1.62, 4.21], h: [0.015, 0.015, 0.01], albedo: P.plasticBlack, rough: 0.5, emissive: [0.3, 4, 0.6], flags: BoxFlag.NoShadow });
  grp('beacons');
  add({ c: [33.8, H - 0.06, 7.6], h: [0.07, 0.06, 0.07], albedo: [0.2, 0.2, 0.22], flags: BoxFlag.NoShadow | BoxFlag.NoPrimary });
  const dome = add({ c: [33.8, H - 0.19, 7.6], h: [0.055, 0.07, 0.055], albedo: [0.35, 0.08, 0.05], flags: BoxFlag.NoShadow | BoxFlag.NoPrimary });
  switchables.push({ name: 'beacon0', boxes: [dome], group: 'emergency', on: false, baseEmissive: [9, 1.2, 0.35], mains: false });

  // ---- doors (closedDir 0: the leaf runs +x from the hinge along a z-wall; π/2: +z along an x-wall) ----
  const doors: LevelDoorDef[] = [
    { name: 'hut_s', hinge: [31.92, 8.5], closedDir: 0, width: 1.16 },
    { name: 'hut_w', hinge: [31.5, 5.62], closedDir: HALF, width: 1.16 },
  ];
  for (const d of doors) doorFrame(d);

  const HUT: Rect = { x0: 31.62, x1: 35.88, z0: 4.12, z1: 8.38 };
  const level: Level = {
    boxes: K.boxes, props: K.props, lights, switchables,
    doors,
    devices: [],
    breaker: { pos: [34.6, 1.35, 4.3], boxes: [brk, brkLed] },
    beacons: [{ pos: [33.8, H - 0.3, 7.6], box: dome }],
    playerSpawn: [20, 0, 25],
    routes: [], chokepoints: [], clearPlans: [],
    sky: { zenith: [0.03, 0.05, 0.09], horizon: [0.07, 0.085, 0.12] },
    // the stealth mission's three places, because the level format wants them
    mission: { serverRoom: { ...HUT }, entryDoor: 'hut_s', rack: { index: 0, front: [33.2, 1.0, 5.6], halfW: 0.3 }, exfilDoor: 'hut_w', exfilX: 20, exfilZ: 25.5, exfilOut: [0, 1] },
  };
  const surfaces: SurfacesData = {
    building: { x0: 3, x1: 37.5, z0: 3, z1: 23.5 }, grassEdge: 0, inside: 'asphalt', outside: 'asphalt', verge: 'asphalt',
    zones: [{ id: 'hut', mat: 'stone', x0: 31.5, z0: 4, x1: 36, z1: 8.5 }],
  };
  return { level, scenarios: { doorNames: [{ door: 'hut_s', name: 'hut door' }, { door: 'hut_w', name: 'hut side door' }] }, groups: trace.groups, surfaces };
}
