// KILLHOUSE — 'The kill house': a small shoot house on a range, run AGAINST A CLOCK. Five small rooms off one short corridor that a ply partition and a
// door cut in two; a front door on the corridor's west end and a back door into the mess; six defenders dealt afresh every
// visit (rust/game_rules/src/roomclear ROOMCLEAR.fresh). The idea is the PAR: the file's `scenarios.par` block (rust/game_rules/src/par) times the run from the first thing anybody does
// to 'structure taken', adds seconds for every teammate lost, and names a medal.
//
//        x 12        17.5        23         28
//   z 7  ┌────▒▒─────┬──▒▒───────┬──▒▒──────┐        ▒▒ = a window a man steps through once it is shot out: the other way into each one-door room
//        │  tyres    │   bunks   │  radio   │        three rooms on one side of the corridor, one door each, named for its room
//   z 12 ├─tyres─────┴──bunks──┬─┴──radio─────┤
//  front      the front hall hall  far hall │ ▮ breaker     'hall': the door that cuts the corridor (one long lane of fire without it)
//  z 14.5├───store───────┬─────┴mess────────┤        two rooms south of it: the store by 'store' and its window, the mess by 'mess' and by 'back' (outside)
//        │    store      │       mess       │        — two pairs on two doors of one room, on one word, is a play in the mess only
//   z 20 └────▒▒─────────┴─────────back─────┘
//
//   way in 1: the ready line west of 'front' · way in 2: the dark south side, on 'back'
//   TRIED AND TAKEN OUT: an outside door into the store's west wall. 24 shipped seeds a way of play: 106 of 192 teammates up against 126 without it
//   (a second way out for the store's man, behind the stack on the front door).
//   lights: every room lit, both halls dark · garrison: tyres 1 · bunks 1 · radio 1 · store 1 · mess 2
//
//   bun run tools/levels/export.ts killhouse [--check]      → levels/killhouse.json
//   page: ?levelFile=killhouse.json&roomclear&squad4        (&way=2: the back door)
//
// THE SKIN, in the default look (src/scene/look.ts): the floor box for box in other clothes. tools/levels/gwotSkin.ts says what may change (paint and finish,
// decals, the paint of a prop) and checks that nothing a round, a charge or a blast reads did.
// ITS OWN PLACE: a training shoot house on a forward base — a sandbagged shell, raw plywood partitions, barricades, crates and targets, packed earth underfoot,
// a HESCO backstop. What it alone has: THE RANGE'S OWN MARKING PAINT, violet, on the ground only, and white engineer tape. Every mark is read off the level's
// data where it is laid (the level as the skin is handed it: scenarios.roomClear and the doors), so a mark cannot say what the level does not:
//   · A START BOX at EVERY way in (the structure's `team` and each of `starts[]`), all drawn alike — none is numbered, none is 'the' start — with a tape toe
//     line where each of the five stands, on the side he faces;
//   · A NUMERAL at each held room's door, on the side the team comes to it from, its top to the door: the room's place in the level's own room list (1 tyres ·
//     2 bunks · 3 radio · 4 store · 5 mess — each room's own door is named for the room). A room with two doors wears its numeral at both
//     (the mess: at 'mess' and outside 'back');
//   · THE CLEAR LINE: one violet line across each of those doors on the room's side — past it a man is in the room;
//   · THE FIRING LINE east of the house, in front of the three ply targets: a band of violet and tape blocks, and a toe line square to each target.
// No letters, no lane lines, no bays, no tiles, no bands at a door. No accent paint on a wall.
import type { Vec3 } from '../../src/math/vec';
import type { Level, LevelDoorDef } from './lib/level';
import type { KitViewData, LevelScenarios, ParData, Rect, StructureData, TeamStart } from './lib/scenarios';
import type { SurfacesData } from './lib/levelData';
import { officeKit, OFFICE_PALETTE, type GeneratorTrace } from './lib/officeGen';
import { WORLD } from '../../src/scene/world';
import { BoxFlag } from '../../src/scene/boxes';
import { gwotKit, gwotOfOffice, GWOT_MATERIALS } from './lib/gwotKit';
import { PANEL_GREY, saidAs } from '../../src/scene/stuff';
import { GWOT_PALETTE as P } from '../../src/scene/gwotPalette';
import { skinLevel, type Skin } from './gwotSkin';
import { lookMarks } from './lookMarks';

const PLAIN_NOTES = 'The kill house: a five-room shoot house on a range, a room clear for the four run against a par clock (scenarios.par). Regenerate: bun run tools/levels/export.ts killhouse';

/** THE PAR (rust/game_rules/src/par): seconds for each medal, and what a teammate lost adds. Set from 24 seeds a way of the scripted commander as shipped
 *  (rebuild/missions/new/killhouse.md has the spread: clocks 39‥141 s, median 85 careful and 66 straight at the doors): gold is a fast
 *  run with nobody lost, silver a clean run at the careful median, bronze a clean slow one or a quick one that lost a man; a man lost costs the gold-to-silver gap (25 s),
 *  so one man lost is one medal */
export const PAR: ParData = { gold: 65, silver: 90, bronze: 120, lossSecs: 25 };

// THE TEAM & KIT PICTURE (scenarios.kitView): both ways on the level's own doors; the ply rooms are lit so the start box's tape reads against them, under a
// neutral work light (the warm porch lamp turned the violet tape brown). At the front the passage behind the door is unlit and stood as a black slot in
// the middle of the picture: the fill lamp hangs in it (the face is the moon's side, the stack keeps its light). At the back the rooms are turned down.
const KILLHOUSE_WORKLIGHT = { intensity: 12, color: [1, 0.95, 0.88] as [number, number, number] };
const KILLHOUSE_KITVIEW: KitViewData = { ways: [
  { door: 'front', roomLights: 1, light: { porch: KILLHOUSE_WORKLIGHT, fill: { offset: [0, -2.2, 2.8], aim: [0, -2.2, 0], intensity: 2, radius: 0.6 } } },
  { door: 'back', roomLights: 0.6, light: { porch: KILLHOUSE_WORKLIGHT } }] };

export function build(): { level: Level; scenarios: LevelScenarios; groups: string[]; surfaces: SurfacesData } {
  const trace: GeneratorTrace = { groups: [] };
  const K = officeKit(trace), C = OFFICE_PALETTE, H = WORLD.ceilingY;
  const { grp, add, slab, wallX, wallZ, desk, monitor, chair, cabinet, shelf, bench, cardboardBox, ceilingPanel, doorFrame, lights, switchables } = K;
  const TYRE: Vec3 = [0.05, 0.05, 0.055];

  // ---- ground, floors, roof ----
  grp('ground');
  slab(0, -0.25, 0, 40, 0, 28, C.asphalt);
  grp('floors');
  slab(12, 0, 7, 28, 0.004, 20, C.lino);                                   // bare sealed screed through the house
  grp('ceiling');
  slab(11.9, H, 6.9, 28.1, H + 0.2, 20.1, C.ceiling, { flags: BoxFlag.NoPrimary });

  // ---- walls: a blockwork shell 0.24 thick, painted ply partitions 0.12 ----
  grp('walls.exterior');
  // ANOTHER WAY IN (rust/game_start levels second-way): the four rooms with one door each have a WINDOW on an outside wall — glass from a 0.15 m kerb (a walker is stopped by anything standing 0.2 m high) to just under
  // door-head height, so once it is shot out a man steps through it — on the far side from their doors and away from both ready lines (the store's stands west of every line from a mark inside to a man on the back door's ready line): a door wedged from inside is then
  // one choice, and the walk round to the glass, against the clock, is the other. Glass and not a second door: a door into the store was measured to cost the
  // team men (below); a pane is no way out for the man inside and no way for anybody's feet until it is broken.
  const port = (a: number) => ({ a, b: a + 1.2, y0: 0.15, y1: 1.88, glass: 'whole' as const });   // (its head under 1.9 m: levels doors reads a taller hole with no sill of 0.3 m as a doorway without a leaf)
  wallX(7, 11.88, 28.12, 0.24, C.concrete, [port(14.7), port(18.8), port(24.1)]);                   // north: tyres, bunks, radio
  wallX(20, 11.88, 28.12, 0.24, C.concrete, [port(14.6), { a: 24.6, b: 25.8, y0: 0, y1: 2.2 }]);    // south: the store's window; the back door
  wallZ(12, 7, 20, 0.24, C.concrete, [{ a: 12.6, b: 13.8, y0: 0, y1: 2.2 }]);                      // west: the front door
  wallZ(28, 7, 20, 0.24, C.concrete);                                                              // east
  grp('walls.interior');
  wallX(12, 12.12, 27.88, 0.12, C.wall, [{ a: 14, b: 15.2 }, { a: 19.6, b: 20.8 }, { a: 25, b: 26.2 }]);   // corridor | tyres, bunks, radio
  wallX(14.5, 12.12, 27.88, 0.12, C.wall, [{ a: 15.4, b: 16.6 }, { a: 23, b: 24.2 }]);             // corridor | store, mess
  wallZ(17.5, 7.12, 11.94, 0.12, C.wall);                                                          // tyres | bunks
  wallZ(23, 7.12, 11.94, 0.12, C.wall);                                                            // bunks | radio
  wallZ(20, 14.56, 19.88, 0.12, C.wall);                                                           // store | mess
  wallZ(21.6, 12.06, 14.44, 0.12, C.wall, [{ a: 12.65, b: 13.85 }]);                               // the corridor cut in two: the front hall | the far hall, door 'hall'

  // ---- what stands in each room: a shoot house's own things — tyres, ply barricades, cots, crates ----
  grp('room.tyres');
  add({ c: [12.75, 0.45, 7.75], h: [0.42, 0.45, 0.42], albedo: TYRE }); add({ c: [13.7, 0.3, 7.7], h: [0.42, 0.3, 0.42], albedo: TYRE });   // tyre stacks in the corner
  add({ c: [12.7, 0.3, 10.9], h: [0.42, 0.3, 0.42], albedo: TYRE });
  add({ c: [14.6, 0.55, 9.6], h: [0.8, 0.55, 0.05], albedo: C.wood });                              // a ply barricade square to the door
  grp('room.bunks');
  bench(22.4, 8.4, Math.PI / 2); bench(22.4, 10.6, Math.PI / 2); cabinet(20.6, 7.5, 0);
  cardboardBox(18.3, 7.7, 0.5, 0.2);
  grp('room.radio');
  desk(26.6, 7.7, 1.6, 0.8, 0); monitor(26.6, 7.6, 0, true); chair(26.6, 8.6, Math.PI);
  shelf(23.5, 9.0, 1.6, 0.4, Math.PI / 2, 4, 1.9);
  grp('room.store');
  shelf(13.4, 19.5, 2.2, 0.45, 0, 4, 1.9);
  add({ c: [17.6, 0.5, 17.0], h: [0.6, 0.5, 0.6], albedo: C.wood }); add({ c: [13.2, 0.4, 16.4], h: [0.5, 0.4, 0.5], albedo: C.wood });   // crates
  cardboardBox(18.9, 19.3, 0.5, 0.3); cardboardBox(12.8, 15.2, 0.45, 0.6);
  grp('room.mess');
  desk(27.2, 16.8, 1.6, 0.8, Math.PI / 2); chair(26.2, 16.8, Math.PI / 2); cabinet(27.4, 19.3, -Math.PI / 2);
  add({ c: [22.0, 0.5, 16.4], h: [0.5, 0.5, 0.5], albedo: C.wood });                                // a crate to fight from
  bench(21.2, 15.1, 0);
  // ---- the range round it: the ready line by the front door, a sandbag line on the south side, a berm and three ply targets to the east ----
  grp('range');
  desk(8.0, 9.8, 1.8, 0.8, 0); bench(5.6, 10.0, Math.PI / 2);
  add({ c: [19.6, 0.4, 24.6], h: [1.6, 0.4, 0.3], albedo: C.partTan });                             // sandbags
  add({ c: [35.0, 0.9, 13.5], h: [0.6, 0.9, 5.0], albedo: C.concrete });                            // the backstop
  for (const z of [10.4, 13.5, 16.6]) { add({ c: [33.4, 0.75, z], h: [0.025, 0.75, 0.28], albedo: C.wood }); add({ c: [33.4, 1.62, z], h: [0.025, 0.12, 0.12], albedo: C.wood }); }

  // ---- lights: a panel a room, all five lit; two dark panels over the halls; a range lamp over the backstop, the moon ----
  grp('lights');
  ceilingPanel(14.6, 9.6, true, 'room_tyres'); ceilingPanel(20.2, 9.6, true, 'room_bunks'); ceilingPanel(25.5, 9.6, true, 'room_radio');
  ceilingPanel(16.0, 17.2, true, 'room_store'); ceilingPanel(24.0, 17.2, true, 'room_mess', false, 1.2, 0.6, [7, 6.8, 6.4]);
  ceilingPanel(15, 13.25, false, 'corridor', false, 1.2, 0.3); ceilingPanel(24, 13.25, false, 'corridor', false, 1.2, 0.3);
  grp('exterior.lamps');
  { const x = 36.4, z = 13.5, dir: Vec3 = [-1, 0, 0], yawL = Math.atan2(dir[0], dir[2]);
    add({ c: [x, 2.6, z], h: [0.06, 2.6, 0.06], albedo: C.metal });
    add({ c: [x + dir[0] * 0.28, 5.24, z], h: [0.04, 0.04, 0.32], yaw: yawL, albedo: C.metal });
    const head = add({ c: [x + dir[0] * 0.55, 5.15, z], h: [0.13, 0.025, 0.22], yaw: yawL, albedo: C.metal, emissive: [40, 24, 7], flags: BoxFlag.NoShadow });
    lights.push({ kind: 'spot', pos: [x + dir[0] * 0.55, 5.05, z], dir: [dir[0] * 0.35, -1, 0], color: [1.0, 0.62, 0.22], intensity: 200, range: 14, innerDeg: 30, outerDeg: 55, name: 'range_lamp', group: 'exterior', enabled: true, volumetric: 0.12, fixtureBox: head, radius: 0.18, mains: false, broad: true }); }
  lights.push({ kind: 'dir', pos: [0, 0, 0], dir: norm([0.45, -0.5, -0.74]), color: [0.55, 0.7, 1.0], intensity: 0.9, range: 1e6, innerDeg: 0, outerDeg: 0, name: 'moon', group: 'exterior', enabled: true, volumetric: 0, radius: 0.009, mains: false });

  // ---- the breaker (the corridor's east end) and one emergency beacon ----
  grp('breaker');
  const brk = add({ c: [27.84, 1.35, 13.25], h: [0.04, 0.36, 0.28], albedo: PANEL_GREY, name: 'breaker_panel' });
  const brkLed = add({ c: [27.79, 1.62, 13.07], h: [0.01, 0.015, 0.015], albedo: C.black, emissive: [0.3, 4, 0.6], flags: BoxFlag.NoShadow });
  grp('beacons');
  add({ c: [20, H - 0.06, 13.25], h: [0.07, 0.06, 0.07], albedo: [0.2, 0.2, 0.22], flags: BoxFlag.NoShadow | BoxFlag.NoPrimary });
  const dome = add({ c: [20, H - 0.19, 13.25], h: [0.055, 0.07, 0.055], albedo: [0.35, 0.08, 0.05], flags: BoxFlag.NoShadow | BoxFlag.NoPrimary });
  switchables.push({ name: 'beacon0', boxes: [dome], group: 'emergency', on: false, baseEmissive: [9, 1.2, 0.35], mains: false });

  // ---- doors (closedDir 0: the leaf runs +x from the hinge along a z-wall; π/2: +z along an x-wall) ----
  const doors: LevelDoorDef[] = [
    { name: 'front', hinge: [12.0, 12.62], closedDir: Math.PI / 2, width: 1.16, exterior: true },
    { name: 'back', hinge: [24.62, 20.0], closedDir: 0, width: 1.16, exterior: true, swing: 1 },   // opens outward: the mess's men can wedge 'mess' alone (rust/game_start levels second-way)
    { name: 'tyres', hinge: [14.02, 12.0], closedDir: 0, width: 1.16 },
    { name: 'bunks', hinge: [19.62, 12.0], closedDir: 0, width: 1.16 },
    { name: 'radio', hinge: [25.02, 12.0], closedDir: 0, width: 1.16 },
    { name: 'store', hinge: [15.42, 14.5], closedDir: 0, width: 1.16 },
    { name: 'mess', hinge: [23.02, 14.5], closedDir: 0, width: 1.16 },
    { name: 'hall', hinge: [21.6, 12.67], closedDir: Math.PI / 2, width: 1.16 },
  ];
  for (const d of doors) doorFrame(d);

  // ---- LIVED IN (lib/lived.ts: all of it goes with the switch off — group 'lived.<place>', props 'lived.<place>.<kind><n>'; LAST, after every kept thing) ----
  // THE RANGE ROUND THE HOUSE. The house's five rooms have their things; the ground round it had a table, a bench, one line of sandbags and the butts. What a
  // range is made of stands on it now, in the look's own kit (lib/gwotKit.ts, so a tyre and a drum are round): the READY LINE is where a detail waits its
  // turn (ammunition crates by the table, the water can, the drum the empty cases go in, the tyres nobody has carried round to the house yet), the SOUTH side
  // has the end of its sandbag line and the stones cleared off the back way in, the BUTTS have lane markers past the firing line, a sandbag return at each end
  // of the backstop and the spall at its foot, and the dark NORTH side is where the house's spares lie (block for the next wall, timber for the barricades).
  // Nothing stands in a start box or on its way to its door, within 2 m of the house's walls (the way round, and every stack), or before a window.
  { const G = gwotKit(gwotOfOffice(K)), spans: [number, string][] = [];
    const place = (n: string) => { spans.push([K.props.length, n]); grp(`lived.${n}`); };
    place('ready');
    G.crates(6.6, 8.5, 0.12, 2); G.crates(9.7, 9.5, -0.2, 1); G.jerryCan(7.5, 8.75, 0.3, 'jerryYellow');
    G.drum(4.6, 8.7, 'rust');
    G.tyres(5.0, 15.7, 3); G.tyres(5.76, 15.86, 2); G.tyres(4.5, 16.5, 2);
    G.rubble(3.0, 11.6, 0.9, 6);
    place('south');
    G.tyres(17.3, 24.6, 3); G.tyres(16.56, 24.5, 2); G.drum(21.8, 24.6, 'drumBlue');
    G.rubble(13.5, 23.2, 1.0, 7);
    place('butts');
    G.tyres(30.6, 8.0, 2); G.tyres(30.6, 19.0, 2);
    G.sandbags(34.9, 19.3, 1.6, 0, 3); G.sandbags(34.9, 7.7, 1.6, 0, 3);
    G.rubble(34.0, 12.0, 0.4, 5, 'sandbag'); G.rubble(34.0, 15.1, 0.4, 5, 'sandbag');
    G.rubble(37.6, 17.4, 1.0, 7); G.rubble(37.7, 10.4, 0.9, 6); G.rubble(37.3, 21.8, 0.8, 5);   // behind the backstop, where nobody walks: what the berm was dug out of
    place('north');
    G.blocks(22.2, 4.9, 0.15, 2); G.woodPile(17.2, 4.9, 0.1, 1.4);
    G.rubble(11.0, 5.2, 0.9, 6);
    spans.push([K.props.length, '']);
    const serial = new Map<string, number>();
    for (let s = 0; s + 1 < spans.length; s++) for (let i = spans[s][0]; i < spans[s + 1][0]; i++) { const key = `lived.${spans[s][1]}.${K.props[i].kind}`, n = serial.get(key) ?? 0; serial.set(key, n + 1); K.props[i].name = `${key}${n}`; }
  }

  const TYRES: Rect = { x0: 12.12, x1: 17.44, z0: 7.12, z1: 11.94 }, BUNKS: Rect = { x0: 17.56, x1: 22.94, z0: 7.12, z1: 11.94 }, RADIO: Rect = { x0: 23.06, x1: 27.88, z0: 7.12, z1: 11.94 };
  const STORE: Rect = { x0: 12.12, x1: 19.94, z0: 14.56, z1: 19.88 }, MESS: Rect = { x0: 20.06, x1: 27.88, z0: 14.56, z1: 19.88 };

  const level: Level = {
    boxes: K.boxes, props: K.props, lights, switchables,
    doors,
    devices: [],
    breaker: { pos: [27.7, 1.35, 13.25], boxes: [brk, brkLed] },
    beacons: [{ pos: [20, H - 0.3, 13.25], box: dome }],
    playerSpawn: [9.0, 0, 13.2],
    routes: [], chokepoints: [], clearPlans: [],
    sky: { zenith: [0.015, 0.025, 0.05], horizon: [0.04, 0.05, 0.08] },
    // the stealth mission's three places, because the level format wants them (the room clear stands the mission still)
    mission: { serverRoom: { ...RADIO }, entryDoor: 'radio', rack: { index: 0, front: [27.6, 1.0, 10.4], halfW: 0.3 }, exfilDoor: 'front', exfilX: 10.5, exfilZ: 13.2 },
  };

  // ---- the room clear: six men in five rooms, four marks or three a room so a fresh visit deals them differently ----
  const N1: Vec3 = [14.6, 1.1, 11.9], N2: Vec3 = [20.2, 1.1, 11.9], N3: Vec3 = [25.6, 1.1, 11.9], S4: Vec3 = [16.0, 1.1, 14.6], S5: Vec3 = [23.6, 1.1, 14.6];
  const roomClear: StructureData = {
    name: 'kill house', maxMen: 6,
    // way in 1: the ready line, on the front door · way in 2: the dark south side, on the mess's back door
    team: { sam: [9.0, 0, 13.2], samYaw: Math.PI / 2, ops: [[10.2, 0, 12.1], [10.2, 0, 14.3], [8.0, 0, 12.1], [8.0, 0, 14.3]], opsYaw: Math.PI / 2 },
    starts: [{ sam: [25.2, 0, 23.2], samYaw: Math.PI, ops: [[24.1, 0, 22.0], [26.3, 0, 22.0], [24.1, 0, 24.2], [26.3, 0, 24.2]], opsYaw: Math.PI }],
    rooms: [
      { id: 'tyres', name: 'tyres', bounds: TYRES, garrison: 1,
        doors: [{ id: 'tyres', from: 'front hall' }],
        marks: [
          { pos: [13.0, 0, 9.2], watch: 'tyres', aimAt: N1 },
          { pos: [16.6, 0, 7.8], watch: 'tyres', aimAt: N1 },
          { pos: [16.2, 0, 10.8], watch: 'tyres', aimAt: N1 },
        ] },
      { id: 'bunks', name: 'bunks', bounds: BUNKS, garrison: 1,
        doors: [{ id: 'bunks', from: 'front hall' }],
        marks: [
          { pos: [21.4, 0, 9.4], watch: 'bunks', aimAt: N2 },
          { pos: [18.4, 0, 11.2], watch: 'bunks', aimAt: N2 },
          { pos: [19.2, 0, 8.6], watch: 'bunks', aimAt: N2 },
        ] },
      { id: 'radio', name: 'radio', bounds: RADIO, garrison: 1,
        doors: [{ id: 'radio', from: 'far hall' }],
        marks: [
          { pos: [24.6, 0, 8.0], watch: 'radio', aimAt: N3 },
          { pos: [27.2, 0, 10.4], watch: 'radio', aimAt: N3 },
          { pos: [24.2, 0, 11.0], watch: 'radio', aimAt: N3 },
        ] },
      { id: 'store', name: 'store', bounds: STORE, garrison: 1,
        doors: [{ id: 'store', from: 'front hall' }],
        marks: [
          { pos: [13.0, 0, 18.2], watch: 'store', aimAt: S4 },
          { pos: [18.8, 0, 15.6], watch: 'store', aimAt: S4 },
          { pos: [15.6, 0, 18.6], watch: 'store', aimAt: S4 },
          { pos: [16.2, 0, 17.4], watch: 'store', aimAt: S4 },
        ] },
      { id: 'mess', name: 'mess', bounds: MESS, garrison: 2,
        doors: [{ id: 'mess', from: 'far hall' }, { id: 'back', from: 'outside' }],
        marks: [
          { pos: [25.8, 0, 15.4], watch: 'mess', aimAt: S5 },
          { pos: [26.6, 0, 18.4], watch: 'back', aimAt: [25.2, 1.1, 19.9] },
          { pos: [22.6, 0, 18.8], watch: 'mess', aimAt: S5 },
          { pos: [23.2, 0, 17.6], watch: 'mess', aimAt: S5 },
        ] },
    ],
  };
  const surfaces: SurfacesData = {
    building: { x0: 12, x1: 28, z0: 7, z1: 20 }, grassEdge: 0, inside: 'lino', outside: 'asphalt', verge: 'asphalt', zones: [],
  };
  return { level, scenarios: { roomClear, par: { ...PAR }, kitView: KILLHOUSE_KITVIEW }, groups: trace.groups, surfaces };
}
function norm(v: Vec3): Vec3 { const l = Math.hypot(v[0], v[1], v[2]); return [v[0] / l, v[1] / l, v[2] / l]; }

// ─────────────────────────────── the skin ───────────────────────────────
export const NAME = 'killhouse';
export const MATERIALS = GWOT_MATERIALS;
export const NOTES = `${PLAIN_NOTES} — IN THE DEFAULT LOOK (2010s GWOT: tools/levels/gwotSkin.ts): the same boxes repainted, decals added; what a man collides with, hides behind or sees through is the floor's.`;
/** the range's marking paint. The palette (src/scene/gwotPalette.ts) has no paint of this hue, so it is a colour of this file's own and only decals wear it;
 *  the outside doors' paint is this, darker: rust/game_rules/src/doors/paint.rs */
const VIOLET: Vec3 = [0.42, 0.2, 0.66];
/** the three ply targets' lanes (tools/levels/killhouse.ts, group 'range': targets at x 33.4) and where the firing line lies, between the east wall's foot and them */
const TARGET_Z = [10.4, 13.5, 16.6], FIRE_X = 29.5;
const SKIN: Skin = {
  wall: 'sandbag', house: 'ply', inner: 'ply', frameOut: 'woodGrey', frameIn: 'poplar', dado: null, ground: 'dust', verge: 'dirt', patches: 6,
  floor: { lino: 'dirt' },
  noRug: /./, metals: ['olive', 'iron', 'rust'],
  // (the two halves of the corridor: the scuffed band along a hall's walls — the scenarios name the held rooms only)
  rooms: [{ id: 'front hall', bounds: { x0: 12.12, x1: 21.54, z0: 12.06, z1: 14.44 } }, { id: 'far hall', bounds: { x0: 21.66, x1: 27.88, z0: 12.06, z1: 14.44 } }],
  // plywood for everything the floor has in wood (the barricade, the crates, the targets, the table tops), sandbags and a HESCO berm on the range
  pick: c => c.name === 'wood' ? 'ply' : c.group === 'range' && c.name === 'partTan' ? 'sandbag' : c.group === 'range' && c.name === 'concrete' ? 'hesco' : undefined,
  dress: (G, level, T) => {
    const M = lookMarks(T), tape = saidAs('paint', T.tone(P.plasticWhite, 0.9)), worn = T.tone(VIOLET, 0.8);
    const rc = (level.scenarios as LevelScenarios | undefined)?.roomClear; if (!rc) throw new Error('killhouse: the level carries no room clear to read its marks off');
    // THE GROUND A RANGE WEARS, laid before any paint so the marks lie on it: the range road along the south edge and the spur a truck backs up toward the
    // ready line (wheels), the beat the details walk round the house a stride off its walls and out to the firing line (feet), and gravel raked between the
    // firing line and the targets. Each is the lot's own earth darker, or a second finish: nothing here is a colour.
    { const lot = T.floorAt(9, 24) ?? P.dust;
      /** a beat feet have worn: the lot's own earth darker down its middle, with a fainter shoulder either side (no edge to read as a painted line) */
      const beat = (pts: [number, number][]) => { M.path(pts, 1.2, T.tone(lot, 0.9)); M.path(pts, 0.5, T.tone(lot, 0.78)); };
      G.ruts(0.6, 26.5, 39.4, 26.5); G.ruts(3.2, 25.6, 3.2, 17.4);
      beat([[9.1, 15.3], [9.1, 21.9], [29.0, 21.9], [29.0, 18.5]]);          // round the south side, past the back door's box, to the line
      beat([[9.1, 11.1], [9.1, 5.6], [29.0, 5.6], [29.0, 8.5]]);             // round the dark north side, under the windows
      M.fill(29.9, 8.7, 33.0, 18.3, P.gravel); }
    /** where a man stands: a tape line half a metre long, a short stride ahead of his feet, square to the way he faces (yaw: he faces (sin, cos)) */
    const toe = (p: Vec3 | [number, number], yaw: number) => { const x = p[0], z = p[p.length - 1], fx = Math.sin(yaw), fz = Math.cos(yaw), cx = x + fx * 0.3, cz = z + fz * 0.3;
      M.seg(cx - fz * 0.25, cz + fx * 0.25, cx + fz * 0.25, cz - fx * 0.25, 0.07, tape); };
    // A START BOX at every way in, all alike
    const ways = [rc.team, ...(rc.starts ?? [])].filter((w): w is TeamStart => !!w);
    for (const w of ways) { const men: Vec3[] = [w.sam, ...(w.ops ?? [])], PAD = 0.7;
      const x0 = Math.min(...men.map(p => p[0])) - PAD, x1 = Math.max(...men.map(p => p[0])) + PAD, z0 = Math.min(...men.map(p => p[2])) - PAD, z1 = Math.max(...men.map(p => p[2])) + PAD;
      M.outline(x0, z0, x1, z1, 0.1, VIOLET);
      toe(w.sam, w.samYaw); for (const p of w.ops ?? []) toe(p, w.opsYaw ?? w.samYaw); }
    // A NUMERAL and THE CLEAR LINE at every door of every held room, in the room list's own order
    rc.rooms.forEach((room, k) => { if (k > 8) return; const b = room.bounds, rx = (b.x0 + b.x1) / 2, rz = (b.z0 + b.z1) / 2;
      for (const rd of room.doors) { const d = level.doors.find(q => q.name === rd.id); if (!d) throw new Error(`killhouse: room '${room.id}' names a door '${rd.id}' the level has not`);
        const ux = Math.cos(d.closedDir), uz = Math.sin(d.closedDir), mx = d.hinge[0] + ux * d.width / 2, mz = d.hinge[1] + uz * d.width / 2;
        // (the door's normal, away from the room)
        let nx = -uz, nz = ux; if ((mx + nx - rx) ** 2 + (mz + nz - rz) ** 2 < (mx - nx - rx) ** 2 + (mz - nz - rz) ** 2) { nx = -nx; nz = -nz; }
        M.glyph(String(k + 1), mx + nx * 0.8, mz + nz * 0.8, 0.55, VIOLET, Math.atan2(nx, nz));
        const L = d.width / 2 + 0.25; M.seg(mx - nx * 0.45 - ux * L, mz - nz * 0.45 - uz * L, mx - nx * 0.45 + ux * L, mz - nz * 0.45 + uz * L, 0.1, worn); } });
    // THE FIRING LINE in front of the targets: a band along it, a toe line square to each target (a firer faces +x: yaw π/2)
    M.hazard(FIRE_X - 0.15, TARGET_Z[0] - 1.55, FIRE_X + 0.15, TARGET_Z[2] + 1.55, VIOLET, tape, 0.31);
    for (const z of TARGET_Z) toe([FIRE_X - 0.75, z], Math.PI / 2);
  },
};
export const finish = (level: Level, groups: string[]) => skinLevel(level, groups, SKIN);
