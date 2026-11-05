// SMOKE-LAB: the floor the smoke is looked at on. Each place on it shows one thing smoke does, and nothing on it is there for any other reason:
//   the yard      open ground under the sky, a low wall of sandbags across it       the wind takes smoke, and smoke parts round a thing and closes behind it
//   the tunnel    a roofed passage 16 m long, shut at its far end, open at the near   smoke rolls down a passage and out of its mouth
//   the kitchen   a room with a door to the yard, a door to the pantry and a window hole     smoke leaks: through a door when it opens, through a hole all the time
//   the pantry    the room next door                                                 what comes through from the room beside it
//   the shed      a small room with one door                                         a shut room fills to the ceiling and stays full
//   two lit fronts  the office's two vending machines against the house, facing the yard   a bright, sharp thing to look at THROUGH smoke, and a light that colours it
// NOT a level to play for its own sake and not in the missions list: the page opens it as the smoke lab (src/host/smokelab.ts: the solver's bricks and its
// spin drawn, each kind of curl on a box, every number a slider). Nobody stands on it but Sam. The tunnel and the rooms are deep up the screen, since the
// camera looks from the south and a wall hides two metres of floor behind it; the tunnel's mouth is its south end for the same reason.
//
//        x 6   8.6    12            17            22        28      32
//   z 5        │      ┌─────────────┬─────────────┐
//   z 6  ┌─────┤      │   kitchen   bt   pantry   │
//        │     │      │             │             │
//   z 10 │ the │      └──kitchen─win┴───pantry────┘     z 10.5 ┌───────┐
//        │tunnel tunnel                                        shed  shed │
//   z 14 │     │              the yard                         │       │
//        │     │            ══ sandbags ══                 z 16 └───────┘
//   z 22 └ ─ ─ ┘  (the tunnel's mouth)
//   bun run tools/levels/export.ts smoke-lab [--check]     → levels/smoke-lab.json
//   page: /smokelab   (= ?look&levelFile=smoke-lab.json&solo&hud&smokelab)
import type { Level, LevelDoorDef } from '../../src/scene/level';
import type { LevelScenarios, Rect } from '../../src/scene/scenarios';
import type { SurfacesData } from '../../src/scene/levelData';
import { officeKit, vendingSpec, normalize3 as norm, type GeneratorTrace } from '../../src/scene/officeGen';
import { gwotKit, gwotOfOffice, GWOT_PALETTE, GWOT_MATERIALS } from '../../src/scene/gwotKit';
import { WORLD } from '../../src/scene/world';
import { BoxFlag } from '../../src/scene/boxes';
import * as MD from '../../src/scene/markingDefaults';

export const NAME = 'smoke-lab';
export const NOTES = 'The smoke lab\'s floor: a yard, a tunnel, two rooms with a door between them and a shed, each there to show one thing smoke does. Regenerate: bun run tools/levels/export.ts smoke-lab';
/** the kit's palette goes into the file's `materials` (tools/levels/export.ts) */
export const MATERIALS = GWOT_MATERIALS;

export function build(): { level: Level; scenarios: LevelScenarios; groups: string[]; surfaces: SurfacesData } {
  const trace: GeneratorTrace = { groups: [] };
  const K = officeKit(trace), G = gwotKit(gwotOfOffice(K)), P = GWOT_PALETTE, H = WORLD.ceilingY;
  const { grp, add, slab, doorFrame, lights, switchables } = K;
  const HALF = Math.PI / 2;

  // ---- ground: dust to the horizon, packed dirt where the lab stands, a trodden line from the tunnel's door to the shed's ----
  grp('ground');
  slab(0, -0.25, 0, 40, 0, 28, P.dust, { rough: 1 });
  G.ground(5, 4, 34, 23.5, 'dirt');
  G.patch(18, 13.6, 9.0, 1.4, 'dust'); G.patch(14.1, 11.4, 1.6, 2.2, 'dust', 0.2); G.patch(19.6, 11.4, 1.6, 2.2, 'dust', -0.2); G.patch(7.3, 22.6, 2.2, 1.6, 'gravel');
  G.patch(24.0, 19.4, 1.5, 1.0, 'oil', 0.3);
  grp('floors');
  G.ground(6, 6, 8.6, 22, 'slab', 0.006);                                        // the tunnel
  G.ground(12, 5, 22, 10, 'mudFloor', 0.006);                                    // the kitchen and the pantry
  G.ground(28, 10.5, 32, 16, 'slab', 0.006);                                     // the shed
  grp('ceiling');
  slab(5.9, H, 5.9, 8.7, H + 0.2, 22.0, P.render, { flags: BoxFlag.NoPrimary, rough: 0.9 });
  slab(11.9, H, 4.9, 22.1, H + 0.2, 10.1, P.mudLight, { flags: BoxFlag.NoPrimary, rough: 0.95 });
  slab(27.9, H, 10.4, 32.1, H + 0.2, 16.1, P.render, { flags: BoxFlag.NoPrimary, rough: 0.9 });

  // ---- the tunnel: shut at its far end, its mouth at the near end, one door half way down its yard side ----
  grp('walls.tunnel');
  G.wallX(6, 5.88, 8.72, 'render');
  G.wallZ(6, 6.12, 22, 'render');
  G.wallZ(8.6, 6.12, 22, 'render', [{ a: 13.4, b: 14.6 }]);
  // ---- the kitchen and the pantry: a door each to the yard, a door between them, a window hole with no glass in the kitchen's wall ----
  grp('walls.house');
  G.wallX(5, 11.88, 22.12, 'mud');
  G.wallX(10, 11.88, 22.12, 'mud', [{ a: 13.5, b: 14.7 }, { a: 15.6, b: 16.4, y0: 1.0, y1: 2.0 }, { a: 19, b: 20.2 }]);
  G.wallZ(12, 5.12, 9.88, 'mud'); G.wallZ(22, 5.12, 9.88, 'mud');
  G.partZ(17, 5.12, 9.88, 'mudLight', [{ a: 7.0, b: 8.1 }]);
  // ---- the shed: four walls and one door ----
  grp('walls.shed');
  G.wallX(10.5, 27.88, 32.12, 'render'); G.wallX(16, 27.88, 32.12, 'render');
  G.wallZ(32, 10.62, 15.88, 'render');
  G.wallZ(28, 10.62, 15.88, 'render', [{ a: 12.4, b: 13.6 }]);

  // ---- a little in each room, so a room reads as a room; the middle of each floor is left clear for the smoke ----
  grp('room.kitchen');
  G.table(13.0, 6.2, 1.6, 0.75, 0); G.stove(16.3, 5.7); G.plasticChair(13.4, 7.0, Math.PI);
  grp('room.pantry');
  G.sacks(21.2, 5.8, 0, 3); G.crates(18.0, 5.7, 0, 2);
  grp('room.shed');
  G.drums(31.3, 15.2, 2); G.jerryCan(31.4, 11.2, 0.3);
  grp('yard');
  G.sandbags(18.0, 17.4, 3.2, 0, 4);                                             // the low wall smoke parts round
  G.drums(24.2, 19.6, 3); G.tyres(10.4, 20.6, 2);

  // ---- two lit fronts to look at through smoke: the office's vending machines (officeGen's vendingSpec: its bodies, its markings, its brightness), their backs
  //      to the house's yard wall between the kitchen's window and the pantry's door, facing down the screen. Throw a can in front of them.
  grp('yard.fronts');
  for (const [kind, x, name] of [['red', 17.05, 'front_red'], ['blue', 18.02, 'front_blue']] as const) {
    const V = vendingSpec(kind, K.TX), z = 10.12 + V.h[2], rw = V.h[0] - V.colW;   // (the dispenser's recess under the lit field, a flush block under the control column: the office's own three boxes)
    const box = add({ c: [x, 1.125, z], h: V.h, albedo: V.body, rough: 0.5, ...K.mk(V.marks) });
    add({ c: [x - V.colW, 0.175, z - 0.03], h: [rw, 0.175, V.h[2] - 0.03], albedo: V.plinth, ...K.mk(MD.dispenserMarks([rw, 0.175, V.h[2] - 0.03], K.TX)) });
    add({ c: [x + rw, 0.175, z], h: [V.colW, 0.175, V.h[2]], albedo: V.plinth });
    switchables.push({ name, boxes: [box], group: 'fronts', on: true, baseEmissive: [0, 0, 0] });
  }

  // ---- lights: a bulb a room, two down the tunnel, a floodlight over the yard, the moon ----
  grp('lights');
  G.bulb(14.6, 7.6, 'bulb_kitchen', 'kitchen', true); G.bulb(19.6, 7.6, 'bulb_pantry', 'pantry', true);
  G.bulb(30.0, 13.4, 'bulb_shed', 'shed', true);
  G.bulb(7.3, 9.5, 'bulb_tunnel_far', 'tunnel', true); G.bulb(7.3, 17.5, 'bulb_tunnel_near', 'tunnel', true);
  grp('exterior.lamps');
  G.floodLight(26.6, 21.6, [-1, -0.45], 'flood_yard', true);
  lights.push({ kind: 'dir', pos: [0, 0, 0], dir: norm([0.45, -0.5, -0.74]), color: [0.55, 0.7, 1.0], intensity: 0.9, range: 1e6, innerDeg: 0, outerDeg: 0, name: 'moon', group: 'exterior', enabled: true, volumetric: 0, radius: 0.009, mains: false });

  // ---- the breaker (the pantry's far wall, inside) and one emergency beacon: the game's blackout wants both ----
  grp('breaker');
  const brk = add({ c: [20.6, 1.35, 5.16], h: [0.28, 0.36, 0.04], albedo: [0.42, 0.44, 0.45], name: 'breaker_panel' });
  const brkLed = add({ c: [20.42, 1.62, 5.21], h: [0.015, 0.015, 0.01], albedo: P.plasticBlack, rough: 0.5, emissive: [0.3, 4, 0.6], flags: BoxFlag.NoShadow });
  grp('beacons');
  add({ c: [14.6, H - 0.06, 9.2], h: [0.07, 0.06, 0.07], albedo: [0.2, 0.2, 0.22], flags: BoxFlag.NoShadow | BoxFlag.NoPrimary });
  const dome = add({ c: [14.6, H - 0.19, 9.2], h: [0.055, 0.07, 0.055], albedo: [0.35, 0.08, 0.05], flags: BoxFlag.NoShadow | BoxFlag.NoPrimary });
  switchables.push({ name: 'beacon0', boxes: [dome], group: 'emergency', on: false, baseEmissive: [9, 1.2, 0.35], mains: false });

  // ---- doors (closedDir 0: the leaf runs +x from the hinge along a z-wall; π/2: +z along an x-wall) ----
  const doors: LevelDoorDef[] = [
    { name: 'tunnel', hinge: [8.6, 13.42], closedDir: HALF, width: 1.16, exterior: true },
    { name: 'kitchen', hinge: [13.52, 10.0], closedDir: 0, width: 1.16, exterior: true },
    { name: 'pantry', hinge: [19.02, 10.0], closedDir: 0, width: 1.16, exterior: true },
    { name: 'between', hinge: [17.0, 7.02], closedDir: HALF, width: 1.06 },
    { name: 'shed', hinge: [28.0, 12.42], closedDir: HALF, width: 1.16, exterior: true },
  ];
  for (const d of doors) doorFrame(d);

  const SHED: Rect = { x0: 28.12, x1: 31.88, z0: 10.62, z1: 15.88 };
  const level: Level = {
    boxes: K.boxes, props: K.props, lights, switchables,
    doors,
    devices: [],
    breaker: { pos: [20.6, 1.35, 5.3], boxes: [brk, brkLed] },
    beacons: [{ pos: [14.6, H - 0.3, 9.2], box: dome }],
    playerSpawn: [18.0, 0, 14.0],
    routes: [], chokepoints: [], clearPlans: [],
    sky: { zenith: [0.015, 0.025, 0.05], horizon: [0.045, 0.05, 0.07] },
    // the stealth mission's three places, because the level format wants them
    mission: { serverRoom: { ...SHED }, entryDoor: 'shed', rack: { index: 0, front: [31.2, 1.0, 13.0], halfW: 0.3 }, exfilDoor: 'tunnel', exfilX: 7.3, exfilZ: 14.0, exfilOut: [-1, 0] },
  };
  // ---- what is underfoot (game/floor.ts): earth is 'asphalt' (the nearest of the six footstep materials), a slab and beaten earth indoors are stone
  const surfaces: SurfacesData = {
    building: { x0: 6, x1: 32, z0: 5, z1: 22 }, grassEdge: 0, inside: 'asphalt', outside: 'asphalt', verge: 'asphalt',
    zones: [
      { id: 'tunnel', mat: 'stone', x0: 6, z0: 6, x1: 8.6, z1: 22 },
      { id: 'house', mat: 'stone', x0: 12, z0: 5, x1: 22, z1: 10 },
      { id: 'shed', mat: 'stone', x0: 28, z0: 10.5, x1: 32, z1: 16 },
    ],
  };
  // what a row calls the doors (scenarios.doorNames; no bearing in a name): the lab has no mission of its own, so this is its one block
  return { level, scenarios: { doorNames: [{ door: 'tunnel', name: 'tunnel door' }, { door: 'kitchen', name: 'kitchen door' }, { door: 'pantry', name: 'pantry door' }, { door: 'between', name: 'kitchen to pantry door' }, { door: 'shed', name: 'shed door' }] }, groups: trace.groups, surfaces };
}
