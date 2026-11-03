// LIBRARY-HALL: the SHOWROOM OF THE OLDER KITS — NOT a level to play for its own sake and not in the missions list. An open floor with one of every piece of
// the office's kit (src/scene/officeGen.ts officeKit), the loud levels' lived kit (livedKit.ts), the furniture of the records levels (livedPieces.ts), the
// quiet pair's pen (lived-quiet.ts) and the training furniture (lived-train.ts), so each can be exported, looked at on the page
// (?levelFile=library-hall.json&solo&hour=midday) and drawn into the catalogue (docs/library.md, tools/levels/library.ts). The look's own kit stands in its
// own showroom, levels/gwot-yard.json (three of its pieces stand here, by the car): between the two, every piece of the library stands once, and `library.ts --check` is red when one does not.
// It is also the worked example of each call: a level task reads a piece's line here and copies it.
//
//        x 0   4          11                                    40
//   z 1  │     ┌──────────┐  bench   car   drum  van   a partition │      the office's kit: a keeper's hut furnished with it, and what does not fit a room
//        │     │ the hut  │                                      │
//   z 6  │     └──hut──win┘                                      │
//   z 7  │  ▒▒▒▒ livedKit ▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒  │      each kit on its own ground, a bay a piece, in the kit's own order
//   z 12 │  ░░░░ livedPieces ░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░ │
//   z 19 │  ▒▒▒▒ livedQuiet ▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒ │
//   z 22 │  ░░░░ livedTrain ░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░ │
//   z 28 └────────────────────────────────────────────────────────┘      the man starts in the west aisle (x under 3)
// THE FOUR LIVED KITS' PIECES ARE LIVED THINGS (src/scene/lived.ts): the tail of the lists, left out with the switch off. So on the frozen stand the hall is
// the hut and the bare grounds.
import type { Vec3 } from '../../src/math/vec';
import type { Level, LevelDoorDef } from '../../src/scene/level';
import type { LevelScenarios, Rect } from '../../src/scene/scenarios';
import type { SurfacesData } from '../../src/scene/levelData';
import { officeKit, normalize3 as norm, OFFICE_PALETTE, type GeneratorTrace } from '../../src/scene/officeGen';
import { gwotKit, gwotOfOffice, GWOT_MATERIALS, type GwotMat } from '../../src/scene/gwotKit';
import { WORLD } from '../../src/scene/world';
import { BoxFlag } from '../../src/scene/boxes';
import { livedKit, LIVED_PAINT } from './livedKit';
import { livedPieces, INK } from './livedPieces';
import { livedQuiet } from './lived-quiet';
import { livedTrain } from './lived-train';

export const NAME = 'library-hall';
export const NOTES = 'The showroom of the older kits: one of every piece of the office kit, livedKit, livedPieces, lived-quiet and lived-train on an open floor. Regenerate: bun run tools/levels/export.ts library-hall';

/** the look's palette goes into the file's `materials` (tools/levels/export.ts): three of the kits here paint from it */
export const MATERIALS = GWOT_MATERIALS;

export function build(): { level: Level; scenarios: LevelScenarios; groups: string[]; surfaces: SurfacesData } {
  const trace: GeneratorTrace = { groups: [] };
  const K = officeKit(trace), G = gwotKit(gwotOfOffice(K)), C = OFFICE_PALETTE, H = WORLD.ceilingY;
  const { grp, add, slab, lights, switchables } = K;
  const HALF = Math.PI / 2;
  const HUT: Rect = { x0: 4.12, x1: 10.88, z0: 1.12, z1: 5.88 };

  /** a kit's stretch of the floor, x 3‥39.5 between z0 and z1, on its own ground: `bay(w)` is the middle of the next bay, w wide, along rows 2.3 m apart */
  const zone = (name: string, z0: number, z1: number, floor: GwotMat) => {
    grp(`ground.${name}`); G.ground(3, z0, 39.5, z1, floor);
    let x = 3.4, z = z0 + 1.25;
    return (w = 2.0): [number, number] => {
      if (x + w > 39.3) { x = 3.4; z += 2.3; }
      if (z + 1.0 > z1) throw new Error(`library-hall: the ${name} stretch is full (z ${z0}‥${z1})`);
      const at: [number, number] = [x + w / 2, z]; x += w; return at;
    };
  };

  // ---- the ground (the whole world's: every ray and every foot lands on something), and each kit's stretch of it ----
  grp('ground');
  slab(0, -0.25, 0, 40, 0, 28, C.asphalt);
  const bayKit = zone('livedKit', 6.8, 11.6, 'gravel'), bayPieces = zone('livedPieces', 11.8, 18.9, 'mudFloor'), bayQuiet = zone('livedQuiet', 19.1, 21.7, 'dirt'), bayTrain = zone('livedTrain', 21.9, 27.6, 'dust');

  // ================================================================ THE OFFICE'S KIT: a hut furnished with it ====
  grp('floors');
  slab(4, 0, 1, 11, 0.004, 6, C.carpet);
  grp('ceiling');
  slab(3.9, H, 0.9, 11.1, H + 0.2, 6.1, C.ceiling, { flags: BoxFlag.NoPrimary });
  grp('walls.hut');
  K.wallX(1, 3.88, 11.12, 0.24, C.concrete);
  K.wallX(6, 3.88, 11.12, 0.24, C.concrete, [{ a: 6.4, b: 7.6 }, { a: 8.8, b: 10.2, y0: 0.9, y1: 2.4 }]);   // the door, and a window for the pane below
  K.wallZ(4, 1.12, 5.88, 0.24, C.concrete);
  K.wallZ(11, 1.12, 5.88, 0.24, C.concrete, [{ a: 2.7, b: 4.3, y0: 0.9, y1: 2.4 }]);
  K.paneX(6, 8.8, 10.2, 0.9, 2.4, C.glassExt); K.paneZ(11, 2.7, 4.3, 0.9, 2.4, C.glassExt);
  grp('room.hut');
  K.desk(6.2, 2.0, 1.6, 0.8, 0); K.monitor(6.2, 1.9, 0, true); K.chair(6.2, 2.8, Math.PI); K.papers(6.8, 0.76, 2.15, 0.2);
  K.cabinet(10.45, 1.55, 0); K.shelf(4.45, 4.3, 1.6, 0.4, HALF, 4, 1.9);
  K.plant(10.4, 5.3); K.bin(5.1, 1.6); K.rug(7.6, 4.0, 2.4, 1.6, C.carpetBlue);
  K.wallArt(8.4, 1.7, 1.135, 1.2, 0.8, C.white, true);
  K.cardboardBox(9.3, 5.3, 0.5, 0.2); K.cardboardBox(9.3, 5.3, 0.4, 0.6, 0.5);                 // a carton, and one stacked on it
  grp('lights');
  K.ceilingPanel(7.5, 3.5, true, 'hut');
  grp('yard.office');
  K.bench(13.6, 2.2, 0); K.car(18.6, 3.6, 0.08, C.partBlue);
  // the three pieces of the look's kit that its own showroom does not stand (gwot-yard's kept part is held as it is by lived-check: a piece the look's kit
  // gains is stood here, or in the yard as a lived thing)
  grp('yard.gwot');
  G.drum(22.6, 2.2, 'drumRed', 0.3); G.van(28.4, 3.6, 0, 'truckWhite'); G.partX(3.6, 32.6, 36.6, 'whitewash');
  // the hall's own light: the moon, and four floodlights that all throw the same way (one key: no lamp fills another's shadows), so the pieces stand lit at night too
  grp('exterior.lamps');
  for (const [i, x] of [8, 17, 26, 35].entries()) G.floodLight(x, i % 2 ? 19.0 : 11.7, [0.3, 1], `flood_${i}`, true);
  lights.push({ kind: 'dir', pos: [0, 0, 0], dir: norm([0.45, -0.5, -0.74]), color: [0.55, 0.7, 1.0], intensity: 0.9, range: 1e6, innerDeg: 0, outerDeg: 0, name: 'moon', group: 'exterior', enabled: true, volumetric: 0, radius: 0.009, mains: false });
  // the breaker and one emergency beacon: the game's blackout wants both
  grp('breaker');
  const brk = add({ c: [9.4, 1.35, 1.16], h: [0.28, 0.36, 0.04], albedo: [0.42, 0.44, 0.45], name: 'breaker_panel' });
  const brkLed = add({ c: [9.22, 1.62, 1.21], h: [0.015, 0.015, 0.01], albedo: C.black, rough: 0.5, emissive: [0.3, 4, 0.6], flags: BoxFlag.NoShadow });
  grp('beacons');
  add({ c: [5.0, H - 0.06, 5.2], h: [0.07, 0.06, 0.07], albedo: [0.2, 0.2, 0.22], flags: BoxFlag.NoShadow | BoxFlag.NoPrimary });
  const dome = add({ c: [5.0, H - 0.19, 5.2], h: [0.055, 0.07, 0.055], albedo: [0.35, 0.08, 0.05], flags: BoxFlag.NoShadow | BoxFlag.NoPrimary });
  switchables.push({ name: 'beacon0', boxes: [dome], group: 'emergency', on: false, baseEmissive: [9, 1.2, 0.35], mains: false });
  const doors: LevelDoorDef[] = [{ name: 'hut', hinge: [6.42, 6.0], closedDir: 0, width: 1.16, exterior: true }];
  for (const d of doors) K.doorFrame(d);

  // ================================================================ from here on, LIVED THINGS only (the tail of every list) ====
  // ---- livedKit: the loud levels' rooms ----
  { const L = livedKit(K), P = LIVED_PAINT; L.room('livedKit'); let x: number, z: number;
    [x, z] = bayKit(); L.mat(x - 0.7, z - 0.5, x + 0.7, z + 0.5, P.rubberMat);
    [x, z] = bayKit(); L.duckboard(x - 0.7, z - 0.35, x + 0.7, z + 0.35);
    [x, z] = bayKit(2.4); { const y = L.table(x, z, 1.8, 0.7); L.mug(x - 0.6, y, z); L.papers(x - 0.1, y, z, 0.2); L.sheet(x + 0.5, y, z + 0.05, 0.21, 0.3, -0.3); }   // a table, and the three things that lie on one
    [x, z] = bayKit(); L.crate(x, z, 0.8, 0.5, 0.45);
    [x, z] = bayKit(); L.tyres(x, z, 3);
    [x, z] = bayKit(); L.jerryCans(x, z, 3);
    [x, z] = bayKit(); L.drumFixed(x, z);
    L.hung(5.2, 6.14, true, 0.5, 0.9, 1.7, P.canvas);                                          // (a coat on the hut's wall)
    [x, z] = bayKit(); L.stool(x, z);
    [x, z] = bayKit(); L.chair(x, z, 0.4);
    [x, z] = bayKit(); L.drum(x, z);
    [x, z] = bayKit(); L.bucket(x, z);
    [x, z] = bayKit(); L.cone(x, z);
    [x, z] = bayKit(); L.carton(x, z, 0.45, 0.3);
    [x, z] = bayKit(); L.looseCrate(x, z, 0.5, 0.36, 0.3);
  }
  // ---- livedPieces: the furniture of the records office, the block, the registry and the consulate ----
  { const P = livedPieces({ add: K.add, grp: K.grp, prop: K.prop }, C as unknown as Parameters<typeof livedPieces>[1]); P.room('livedPieces'); let x: number, z: number;
    [x, z] = bayPieces(2.4); P.floor(x, z, 1.8, 1.4, INK.baize);
    [x, z] = bayPieces(2.6); P.table(x, z, 2.0, 0.9); P.plan(x - 0.45, 0.74, z, 0.9, 0.6); P.sheets(x + 0.45, 0.74, z - 0.15); P.ledger(x + 0.5, 0.74, z + 0.22, 0.3);   // paper on a table
    [x, z] = bayPieces(3.0); P.counter(x, z, 2.4, 0.7); P.tins(x - 0.7, 1.0, z); P.tray(x + 0.15, 1.0, z, 0.2); P.stamps(x + 0.8, 1.0, z);                      // a counter and what a clerk keeps on one
    [x, z] = bayPieces(); P.chest(x, z, 1.0, 0.6); P.rolls(x, 0.9, z);
    [x, z] = bayPieces(1.6); P.rollBin(x, z);
    [x, z] = bayPieces(2.6); P.table(x, z, 2.0, 0.9); P.planBook(x - 0.4, 0.74, z); P.books(x + 0.5, 0.74, z + 0.2);
    [x, z] = bayPieces(2.6); P.table(x, z, 2.0, 0.9); P.typewriter(x - 0.6, 0.74, z); P.phone(x - 0.05, 0.74, z + 0.15); P.mugs(x + 0.35, 0.74, z - 0.2); P.kettle(x + 0.75, 0.74, z + 0.15); P.hat(x + 0.4, 0.74, z + 0.2);   // a desk's things
    [x, z] = bayPieces(); P.tripod(x, z);
    [x, z] = bayPieces(2.6); P.poles(x, 0.6, z);
    [x, z] = bayPieces(2.4); P.drafting(x, z);
    [x, z] = bayPieces(3.0); P.ladder(x, 0.03, z, 0, 2.4);
    [x, z] = bayPieces(); P.safe(x, z);
    [x, z] = bayPieces(2.4); P.hooks(x, z, 0);
    [x, z] = bayPieces(2.2); P.rail(x, z, 0, 3, 1.2, 0.2, 0.25, 0.2, INK.hivis);
    [x, z] = bayPieces(); P.coatStand(x, z);
    [x, z] = bayPieces(); P.boots(x, z);
    [x, z] = bayPieces(); P.sack(x, z, 0.3);
    [x, z] = bayPieces(1.6); P.stool(x, z);
    [x, z] = bayPieces(); P.armchair(x, z, 0.3);
    [x, z] = bayPieces(1.6); P.crate(x, z);
    [x, z] = bayPieces(); P.trolley(x, z, 0.2);
    [x, z] = bayPieces(1.4); P.peg(x, z);
    [x, z] = bayPieces(2.6); P.rod(x - 0.9, z, x + 0.9, z + 0.2);
    [x, z] = bayPieces(1.6); P.cone(x, z);
    [x, z] = bayPieces(1.6); P.bucket(x, z);
  }
  // ---- lived-quiet: the pen of the two quiet levels played as two ----
  { const Q = livedQuiet(K, G.M); Q.room('livedQuiet'); let x: number, z: number;
    [x, z] = bayQuiet(2.4); Q.mat(x, z, 1.6, 0.9, 'rubber');
    [x, z] = bayQuiet(2.4); Q.line(x - 0.8, z, x + 0.8, z, 0.06, 'whitewash');
    [x, z] = bayQuiet(2.4); Q.bench(x, z, 1.3, true); Q.mug(x + 0.3, z, 0.44);
    [x, z] = bayQuiet(3.0); { const ys = Q.shelf(x, z, 2.0, 0.36, 1.5, true); Q.paper(x - 0.4, z, ys[1], 0.2); }
    [x, z] = bayQuiet(1.6); Q.pail('pail', x, z);
    [x, z] = bayQuiet(); Q.carton('carton', x, z, 0.2, 0.5, 0.36, 0.3);
    [x, z] = bayQuiet(1.6); Q.stool('stool', x, z);
    [x, z] = bayQuiet(1.6); Q.chock('chock', x, z);
    [x, z] = bayQuiet(); Q.tyre('tyre', x, z);
  }
  // ---- lived-train: the training lessons' furniture, and the seven signature pieces ----
  { const T = livedTrain({ G, K, add, grp }); T.place('livedTrain'); let x: number, z: number;
    [x, z] = bayTrain(2.4); T.table(x - 0.9, z - 0.4, x + 0.9, z + 0.4);
    [x, z] = bayTrain(2.4); T.plank(x - 0.9, z - 0.14, x + 0.9, z + 0.14);
    [x, z] = bayTrain(2.8); T.rack(x - 1.1, z - 0.18, x + 1.1, z + 0.18, 1.7, [0.4, 0.9, 1.4]);
    [x, z] = bayTrain(2.4); T.row(x - 0.6, z, 'x', 4, 0.3, 0.3, 0.06, 0, 0.5, ['olive', 'coyote']);
    [x, z] = bayTrain(1.6); T.stool(x, z);
    [x, z] = bayTrain(1.6); T.bucket(x, z);
    [x, z] = bayTrain(1.6); T.pot(x, z);
    [x, z] = bayTrain(1.6); T.jerry(x, z, 0.3, 'olive');
    [x, z] = bayTrain(1.6); T.helmet(x, z);
    [x, z] = bayTrain(1.6); T.toolRoll(x, z, 0.4);
    [x, z] = bayTrain(1.8); T.chest(x, z, 0.2, 'olive');
    [x, z] = bayTrain(2.8); T.hoist(x, z);
    [x, z] = bayTrain(3.0); T.cauldrons(x, z, 0, 3);
    [x, z] = bayTrain(2.8); T.spools(x, z);
    [x, z] = bayTrain(2.6); T.flag(x, z);
    [x, z] = bayTrain(3.0); T.basins(x, z);
    [x, z] = bayTrain(2.4); T.loft(x, z);
    [x, z] = bayTrain(3.6); T.waterCart(x, z);
    T.done();
  }

  const level: Level = {
    boxes: K.boxes, props: K.props, lights, switchables,
    doors,
    devices: [],
    breaker: { pos: [9.4, 1.35, 1.3], boxes: [brk, brkLed] },
    beacons: [{ pos: [5.0, H - 0.3, 5.2], box: dome }],
    playerSpawn: [1.6, 0, 14.0] as Vec3,
    routes: [], chokepoints: [], clearPlans: [],
    sky: { zenith: [0.015, 0.025, 0.05], horizon: [0.045, 0.05, 0.07] },
    // the stealth mission's three places, because the level format wants them
    mission: { serverRoom: { ...HUT }, entryDoor: 'hut', rack: { index: 0, front: [10.45, 1.0, 1.9], halfW: 0.3 }, exfilDoor: 'hut', exfilX: 7.0, exfilZ: 6.6, exfilOut: [0, 1] },
  };
  const surfaces: SurfacesData = { building: { x0: 4, x1: 11, z0: 1, z1: 6 }, grassEdge: 0, inside: 'carpet', outside: 'asphalt', verge: 'asphalt', zones: [] };
  return { level, scenarios: { doorNames: [{ door: 'hut', name: 'hut door' }] }, groups: trace.groups, surfaces };
}
