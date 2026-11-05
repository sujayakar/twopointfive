// TUT-KIT — the tutorial's third mission, "Your kit, then the squad's": THE FUEL HALT, at night. He leaves his four at their truck on the road and goes in
// alone with one of everything he carries: a door for the interaction key, a gun for a smoke and a frag, a lamp for the pistol, a locked door for his own
// charge, a room for a stun grenade, a doorway for the launcher. Then he calls the four up and has THEM use the same things from the ring on the squad's
// key: the breaker box, a door they bang, a wall they hole, a frag over a wall of sandbags.
//
// THE SENTENCE NOBODY COULD SAY OF ANOTHER LEVEL: every thing on it is used twice, once by his hand and once by his word.
// THE HAND PLACEMENT: the fights are the level's script (levels/tut-kit.script.ts): every man of the halt is held on his mark until his beat, so the first
// of every list here is the only deal there is. The gun's cone through the arch falls on the parked truck and never on the way to the arch's near jamb;
// the sandbags, the gap and the barrier are one crouch, one smoke and one frag long; the bunk room's door stands in the shop he has already cleared, the
// fuel room's door is locked with a gun on it and its wall is a partition, and the two in the wash yard sit behind bags a rifle cannot reach over.
// The gun court, the shop and the store are tut-solo's to the centimetre (tools/levels/tut-solo.ts: this file grew from it and that level is not changed).
//
//        x 0        9          16              25          31          36
//   z 5             ┌──────────┬───────────────┬───────────┬──────────┐
//         pumps     │  truck   │ ▣box nest gun │ bunk room ▮ fuel room│
//         ▢ ▢       │  court   │               ├─────▮─────┬──────────┤ z 10.5
//   road ═ van ═ gate_w        ▒▒▒sandbags▒  gap ▓▓ shop door  shop  ·  store ▮ lane
//                   │   jersey │ arch          │           │          │
//    truck + four   │  pickup  │               ├── gateway ┴──────────┤ z 17.5
//   z 23            └──────────┴───────────────┴─  wash yard ▒ ▒   ▮──┘
//
//   bun run tools/levels/export.ts tut-kit [--check]      → levels/tut-kit.json
//   bun run tools/qa/lesson-check.ts tut-kit              the lesson block against the built floor
//   bun run tools/qa/lesson-run.ts tut-kit                the scripted player through every step to the debrief
//   page: ?levelFile=tut-kit.json&roomclear=1&squad4&hour=night
import type { Vec3 } from '../../src/math/vec';
import type { Level, LevelDoorDef } from '../../src/scene/level';
import type { LevelScenarios, LessonData, LessonStep, StructureData, Rect, IssueData } from '../../src/scene/scenarios';
import type { SurfacesData } from '../../src/scene/levelData';
import { officeKit, normalize3 as norm, type GeneratorTrace } from '../../src/scene/officeGen';
import { gwotKit, gwotOfOffice, GWOT_PALETTE, GWOT_MATERIALS, GW_SHELL } from '../../src/scene/gwotKit';
import type { GwotMat } from '../../src/scene/gwotPalette';
import { WORLD } from '../../src/scene/world';
import { BoxFlag } from '../../src/scene/boxes';
import { findCarLamps } from '../../src/game/carlamps';
import { LAST_IN, LAST_MEN, YARD_IN, YARD_MEN } from '../../levels/tut-kit.script';

export const NAME = 'tut-kit';
export const NOTES = 'The tutorial\'s third mission: a walled fuel halt on a road at night. He goes in alone with his own kit (gate, truck court, arch, gun court, shop, store), then calls his four up and has them use theirs (the breaker box, the bunk room, the fuel room, the wash yard). Its fights are staged by levels/tut-kit.script.ts. Regenerate: bun run tools/levels/export.ts tut-kit';
export const MATERIALS = GWOT_MATERIALS;

const HALF = Math.PI / 2;
/** the yard inside its wall, and the two cross walls */
export const YARD: Rect = { x0: 9, x1: 36, z0: 5, z1: 23 };
/** THE LINE the mission is walked along: the yard gate, the arch, the shop's door and the store's doorway are all centred on it (a door's middle sits on a
 *  walk-grid cell: z 14.25, never 14) */
export const AXIS = 14.25;
const DOORWAY = { a: AXIS - 0.6, b: AXIS + 0.6 }, HINGE = AXIS - 0.58;
const ARCH_X = 16, FRONT_X = 25, PART_X = 30.5, ROOM_Z0 = 10.5, ROOM_Z1 = 17.5;
/** the two rooms behind the shop, against the halt's north wall: the wall between them */
const NPART_X = 31;
/** the gravel a truck drives on: the drive through the gate and the arch, and the apron it turns on before the shop */
const DRIVE = { z0: AXIS - 1.7, z1: AXIS + 1.7 }, APRON = { x0: 19.2, z0: 10.6, z1: 18.4 }, TRUCK_GATE_X = 12.6;

// ---- the marks the level, its script and its scripted player all read ----
/** where he starts: on the road at the van's open back doors, in the light of its load bay, his four at their truck behind him */
export const START: Vec3 = [4.35, 0, 19.0];
/** THE FOUR, in a line along their truck's far side from the halt (they wait there on a knee until he calls them: the tutorial's frame holds them, `rails.waits`) */
export const TRUCK_AT = { x: 5.6, z: 22.6, yaw: 0.08 };
export const OPS: Vec3[] = [[3.6, 0, 24.5], [4.7, 0, 24.6], [5.8, 0, 24.5], [6.9, 0, 24.6]];
/** beside the blue gate, out of the gun's cone (step 'arch') */
export const JAMB: Vec3 = [15.1, 0, 11.7];
/** THE INNER DOOR (step 'open'): a wall shuts the gate's corner of the truck court off from the way to the arch, and its one door is wooden and shut, not
 *  locked: the first thing the interaction key's tap is asked for. Where the wall stands, the door's middle along it, where he stands to open it */
export const INNER_X = 12.5, INNER_Z = 9.25, INNER_FROM: Vec3 = [11.55, 0, INNER_Z];
/** …and it opens AWAY from him, toward the arch: a push from the face he comes to it from */
const SWING_INNER = -1;
/** the far end of the sandbags (step 'low') */
export const BAGS_END: Vec3 = [20.2, 0, 13.6];
/** where the smoke goes: the gap itself, on the gun's line to it. On floor he can see from behind the sandbags' end: the ring is the place to click */
export const GAP: Vec3 = [21.9, 0.1, 12.7];
/** the lamp he shoots out with the pistol: on its pole by the shop's front, over where he stands clear of the charge */
export const LAMP: Vec3 = [24.4, 3.2, 17.7];
/** behind the barrier (steps 'cross', 'back') */
export const BARRIER: Vec3 = [23.3, 0, 13.9];
/** beside the shop's door: the whole mark (0.6 m round it) out of the charge's backblast, 1.6 m from the frame's middle, and out of the doorway's line, which the man inside is looking down (step 'back') */
export const BACK: Vec3 = [24.45, 0, 16.5];
/** the gun's nest and the man on the gun */
export const NEST: Vec3 = [22.6, 0.4, 7.0];
export const GUNNER: Vec3 = [22.7, 0, 6.9];
/** the shop's door (the charge), the man behind it, the doorway to the store (the stun can), the two men in there */
export const SHOP_DOOR: Vec3 = [FRONT_X, 1.0, AXIS];
export const SHOP_MAN: Vec3 = [27.7, 0, AXIS];
export const STORE_DOORWAY: Vec3 = [31.6, 0.2, AXIS];
/** where the stun grenade is asked for (step 'stun'): the middle of the storeroom. A can thrown at the doorway's own mark catches the partition and stays in the shop */
export const STUN_AT: Vec3 = [33.4, 0.1, AXIS];
/** (neither can be seen, or shot, from outside the shop: the two doorways' cone misses both, so the stun grenade goes in before he does) */
export const STORE_MEN: Vec3[] = [[31.4, 0, 11.5], [33.6, 0, 16.3]];
/** THE LAST FIGHT IS BEHIND HIM, AND TO THE RIGHT OF THE SCREEN. The launcher's grenade arms 8 m out, and that is the edge of what the camera shows: only up the
 *  mission's line (east) does the view reach it, clear of the kit panel. So he is sent back to the arch first, and the last two come out through the building
 *  he has just cleared and stand in the shop's doorway, 8.5 m and more from anywhere the arch's line lets him stand: on the screen, armed, one grenade's worth. */
export const ARCH_MOUTH: Vec3 = [16.5, 0, AXIS];
/** WHERE THE LAUNCHER'S LINE STANDS HIM: the arch's mouth on the gate's side, and a small ring. The grenade is a dud if the first thing it meets is inside 8 m of
 *  the tube, which is 8.7 m from his feet to the first man's (measured on the page: from 17.0 on the line the grenade met the man 8.0 m out and did nothing), and
 *  the camera shows about 10.5 m to the right of him in a 16:9 window and 9 m in a 16:10 one (its own numbers, first zoom, no aim pan). So the ring's far edge (16.6) is 9.1 m from the first man: a walk back from the shop that stops as the line
 *  changes is still far enough, and further back is too. Off the line towards the gate because the camera is turned: from there the doorway is 3 % of the
 *  picture further in from its right edge than from the line itself. */
export const LAUNCH_FROM: Vec3 = [16.1, 0, AXIS + 1.0];
/** the shop's doorway from outside: where the launcher's grenade goes */
export const LAST_COVER: Vec3 = [25.9, 1.0, AXIS];
/** where they stand behind it, and where they come onto the floor: the level's script's own numbers (it is the file the page takes) */
export { LAST_MEN, LAST_IN };
/** where he and the four stand when the breaker is asked for: the gun court inside the arch */
export const COURT: Rect = { x0: ARCH_X + 0.3, x1: FRONT_X - 0.3, z0: YARD.z0 + 0.3, z1: YARD.z1 - 0.3 };
/** inside the wall: the arch and the gun court together (the squad called up stands in file behind him, most of it still under the arch) */
export const INSIDE: Rect = { x0: INNER_X + 0.4, x1: FRONT_X - 0.3, z0: YARD.z0 + 0.3, z1: YARD.z1 - 0.3 };
/** THE BREAKER BOX: on the gun court's north wall, on the face the camera sees, a room's width from the nest he has already put out */
export const BREAKER: Vec3 = [19.4, 1.35, YARD.z0 + 0.165];
/** THE BUNK ROOM's door (in the shop's north wall: the shop is his by then, and a stack has two metres of bare wall each side of it), the fuel room's
 *  (locked, in the wall between the two rooms), and the place on that wall their charge goes: clear of the door's frame and of every cot */
export const BUNK_DOOR_X = 27.75, FUEL_DOOR_Z = 9.25;
export const BUNK_DOOR: Vec3 = [BUNK_DOOR_X, 1.0, ROOM_Z0 + 0.3], WALL_MARK: Vec3 = [NPART_X - 0.05, 1.1, 6.6];
/** the squad's charge is seen whichever way its men face (they stand off with their backs to it): a lookout in the bunk room, on the wall */
const WALL_EYE = { at: [28.4, 1.6, 6.6] as Vec3, yaw: HALF, fov: 100 };
/** it opens INTO the bunk room, away from the stack */
const SWING_BUNK = -1;
/** the two of the bunk room (neither in the door's line from the shop, neither within the charge's reach of the wall between the rooms) and the fuel
 *  room's one, in its far corner from that wall */
export const BUNK_MEN: Vec3[] = [[26.6, 0, 8.4], [28.3, 0, 6.9]], FUEL_MAN: Vec3 = [34.9, 0, 8.4];
/** the rows of his own ring this mission teaches (`rails.rows`): a door by hand, his charge and its trigger */
const OWN_ROWS = ['push', 'pull', 'charge'];
/** where he leads the four back to: the gun court, square on to the wash yard's gateway and six metres from the floor behind its sandbags. A man's
 *  arm reaches about six and a half over a wall that high (squadthrow.ts, mapped on this floor), so the line is done close to this mark, not across the court */
export const YARD_FROM: Vec3 = [23.8, 0.02, 19.75];
/** THE WASH YARD: the gateway in its front wall, the wall of sandbags across it, the floor behind them where the frag is asked for, its back door */
export const YARD_GATE = { a: 18.25, b: 21.25 }, YARD_BAGS = { x: 27.9, z: 19.75, len: 3.4 }, YARD_DOOR_Z = 19.75;
export const FRAG_AT: Vec3 = [29.6, 0.05, 19.8];
/** where they stand behind the bags and where they come onto the floor: the level's script's own numbers */
export { YARD_MEN, YARD_IN };

/** THE TUTORIAL FRAME'S KEYS, a step each: `allow` turns actions on for the rest of the mission (from the first step he has what the first two missions
 *  taught him: `rails.allow`), `cue` goes to the level's script as the step comes up ('lesson:<cue>'), `checkpoint` names the one save of a beat. Every
 *  checkpoint is where a fight has just ended and nobody alive can see him. A save put back starts the level's script over from its first beat, and
 *  the step's own cue again where it has one (the wash yard's two walk in again after 'the fuel room'); 'the gun' and 'the store' have none, their
 *  next beats come with the steps after them. Whether the four still wait is read from the steps (`rails.waits`), never kept. */
export const FRAME: Record<string, { allow?: string[]; deny?: string[]; cue?: string; checkpoint?: string }> = {
  open: { allow: ['use'] }, arch: { cue: 'gun' }, low: { cue: 'bags' },
  smoke: { allow: ['slot:smoke'], cue: 'gap' }, cross: { cue: 'blind' }, frag: { allow: ['slot:frag'] },
  pistol: { allow: ['slot:pistol'], checkpoint: 'the gun' }, blow: { allow: ['word'] }, shop: { cue: 'shop' },
  stun: { allow: ['slot:flash'], cue: 'room', checkpoint: 'the shop' },
  leave: { checkpoint: 'the store' }, behind: { cue: 'trucks' }, launch: { allow: ['alt'] }, last: { cue: 'loose' },
  call: { allow: ['order'], cue: 'call', checkpoint: 'the building' }, breaker: { cue: 'box' }, nv: { allow: ['nvg'], cue: 'dark' },
  door: { allow: ['menu', 'ring'], cue: 'bunk', checkpoint: 'the dark' }, wall: { cue: 'wall', checkpoint: 'the bunk room' },
  lead: { cue: 'yard', checkpoint: 'the fuel room' }, lights: { cue: 'seen' }, clear: { cue: 'free' },
};

const SHOP: Rect = { x0: FRONT_X, x1: PART_X, z0: ROOM_Z0, z1: ROOM_Z1 }, STORE: Rect = { x0: PART_X, x1: YARD.x1, z0: ROOM_Z0, z1: ROOM_Z1 };
const BUNK: Rect = { x0: FRONT_X, x1: NPART_X, z0: YARD.z0, z1: ROOM_Z0 }, FUEL: Rect = { x0: NPART_X, x1: YARD.x1, z0: YARD.z0, z1: ROOM_Z0 }, WASH: Rect = { x0: FRONT_X, x1: YARD.x1, z0: ROOM_Z1, z1: YARD.z1 };
/** a room's floor inside its walls */
const inner = (R: Rect): Rect => ({ x0: R.x0 + 0.12, x1: R.x1 - 0.12, z0: R.z0 + 0.12, z1: R.z1 - 0.12 });

/** HIS KIT is what every squad's leader is issued (game/kit.ts, the pouches rule: four pouches a man): one smoke, one stun grenade, one frag and one
 *  grenade for the launcher. The door charge comes out of the squad's bag. The card is named here, with nothing added, for one reason: a mission that
 *  names his card writes his counts (kitIssue), so the launcher comes up on its grenade. Without the block a tutorial's page leaves him as a Player is
 *  built, the tube on five sponge rounds no line here teaches. A card with spares is over the rule with a squad on the floor and is cut back to this at
 *  the staging, so there are none: every step that spends a can is also done by its men going down, and one thrown wide costs him the easy way, never
 *  the mission */
export const ISSUE: IssueData = { members: { '0': { weapon: 'rifle' } } };

export function build(): { level: Level; scenarios: LevelScenarios; groups: string[]; surfaces: SurfacesData } {
  const trace: GeneratorTrace = { groups: [] };
  const K = officeKit(trace), G = gwotKit(gwotOfOffice(K)), P = GWOT_PALETTE, H = WORLD.ceilingY, M = GWOT_MATERIALS, t = GW_SHELL / 2;
  const { grp, add, slab, doorFrame, lights, switchables } = K;
  const doors: LevelDoorDef[] = [];
  const zones: SurfacesData['zones'] = [];

  // ---- the ground: the plain, the road, the forecourt, the drive ----
  // THE GROUND IS LAID AS THE PLACE IS USED (the maintainer, of the gun court: "ground looks pretty flat here"). The yard is the plain's own dust inside its
  // walls; a gravel drive a truck wide comes in by the trucks' gate in the south wall, turns through the arch and opens into a turning apron before the shop; where wheels,
  // feet and stores have been the earth is darker (`ground.marks`, below, with the stones that line the drive). The apron reaches past where he stands for
  // the shop's door, so the ground he is asked to hide on is no paler than it was
  grp('ground');
  slab(0, -0.25, 0, 40, 0, 28, P.dust, { rough: 1 });
  G.ground(0, 12.3, YARD.x0, 15.7, 'gravel');                       // the road, up to the gate
  G.ground(1.6, 7.6, 8.2, 12.2, 'slab');                            // the forecourt under the pumps
  G.ground(TRUCK_GATE_X - 1.7, DRIVE.z1, TRUCK_GATE_X + 1.7, YARD.z1, 'gravel');   // the drive: in by the trucks' gate…
  G.ground(TRUCK_GATE_X - 1.7, DRIVE.z0, APRON.x0, DRIVE.z1, 'gravel');            // …round the corner and through the arch
  G.ground(APRON.x0, APRON.z0, FRONT_X, APRON.z1, 'gravel');        // the turning apron before the shop
  G.ground(FRONT_X, ROOM_Z1, YARD.x1, YARD.z1, 'dirt');             // the wash yard
  G.ground(YARD.x1, 12.4, 40, 16.1, 'dirt');                        // the lane behind the halt, at the store's back door: it runs off the map
  G.ground(YARD.x1, YARD_DOOR_Z - 1.0, 40, YARD_DOOR_Z + 1.0, 'dirt');   // …and the foot path to the wash yard's back door

  // ---- the yard wall, the gate in it, the two cross walls ----
  grp('walls.yard');
  const gate = [{ ...DOORWAY, y0: 0, y1: 2.2 }];
  // (the north wall stops at the building: the bunk room and the fuel room stand against the halt's edge, and their own wall is the wall there)
  G.yardWallX(YARD.z0, YARD.x0 - 0.12, FRONT_X); G.yardWallX(YARD.z1, YARD.x0 - 0.12, YARD.x1 + 0.12);
  G.yardWallZ(YARD.x0, YARD.z0, YARD.z1, 'mud', gate); G.yardWallZ(YARD.x1, ROOM_Z1, YARD.z1, 'mud', [{ a: YARD_DOOR_Z - 0.6, b: YARD_DOOR_Z + 0.6, y0: 0, y1: 2.2 }]);
  for (const [x, z] of [[9, 5], [9, 23], [36, 23], [16, 5], [16, 23], [25, 23]] as const) G.pier(x, z);
  // the wash yard's back door, onto the plain behind the halt: steel, shut and locked until the two who end the mission come in by it
  doors.push({ name: 'wash', hinge: [YARD.x1, YARD_DOOR_Z - 0.58], closedDir: HALF, width: 1.16, locked: true, keySide: 1, swing: 1, steel: true, exterior: true, closer: false });
  // the yard gate stands open, out along the road
  doors.push({ name: 'gate_w', hinge: [YARD.x0, AXIS + 0.58], closedDir: -HALF, width: 1.16, exterior: true, angle: -1.45, closer: false });
  // the lane's two walls, off the map's edge: nobody walks round the halt from the back door, and nobody comes to it but by the lane
  G.yardWallX(12.4, YARD.x1 + 0.12, 40); G.yardWallX(16.1, YARD.x1 + 0.12, 40);
  // the trucks' own gate, in the truck court's south wall: two steel leaves, shut, on each face of the wall (the wall stands behind them: nobody walks, sees
  // or shoots through). The gate on the road is a man's width; this is where the white truck and every wheel track in the yard came in
  grp('walls.truckgate');
  G.gate(TRUCK_GATE_X, YARD.z1 + 0.15, 'x', 3.2, 1, 'paintBlue', 0); G.gate(TRUCK_GATE_X, YARD.z1 - 0.15, 'x', 3.2, -1, 'paintBlue', 0);
  grp('walls.arch');
  G.yardWallZ(ARCH_X, YARD.z0, AXIS - 1.6, 'mudLight'); G.yardWallZ(ARCH_X, AXIS + 1.6, YARD.z1, 'mudLight');   // the arch: a truck's width, no leaf
  G.pier(ARCH_X, AXIS - 1.6); G.pier(ARCH_X, AXIS + 1.6);
  G.gate(ARCH_X + 0.06, AXIS, 'z', 3.2, 1, 'paintBlue', 84);
  grp('walls.front');
  // (the wash yard's gateway: three metres of the front wall left out, a pier each side, no leaf)
  G.yardWallZ(FRONT_X, ROOM_Z1, YARD_GATE.a, 'mud'); G.yardWallZ(FRONT_X, YARD_GATE.b, YARD.z1, 'mud'); G.pier(FRONT_X, YARD_GATE.a); G.pier(FRONT_X, YARD_GATE.b);

  // ---- the forecourt on the road: two pumps under a tin canopy, the van that was at them ----
  grp('forecourt');
  G.pump(4.0, 9.9, HALF); G.pump(6.2, 9.9, HALF);
  for (const [x, z] of [[2.4, 8.2], [7.6, 8.2], [2.4, 11.6], [7.6, 11.6]] as const) add({ c: [x, 1.6, z], h: [0.05, 1.6, 0.05], ...M.galv });
  slab(2.1, 3.2, 7.9, 7.9, 3.32, 11.9, P.tin, { flags: BoxFlag.NoPrimary, rough: 0.6 });
  G.tube(4.0, 9.9, true, 'exterior', true, true); G.tube(6.2, 9.9, true, 'exterior', true);
  G.drums(2.3, 9.2, 3); G.jerryCan(7.4, 10.6, 0.4); G.jerryCan(7.6, 10.2, -0.2, 'drumRed'); G.tyres(2.2, 11.2, 3);
  G.patch(5.0, 10.4, 3.4, 2.0, 'oil', 0.2, 0.019);
  grp('forecourt.van');
  // across the road, nose to the pumps it was leaving. It is as its driver left it, everything on: two
  // head lamps (beams across the forecourt), the tail lamps, the lamp in the load bay over its open back doors, which is the light he starts in. Each lens
  // is a glowing sliver a hair off the van's own and a little wider, so a round at a lamp meets the sliver first and puts it out; the van's own lens behind
  // it stays the dead glass the lamp check finds (carlamps.ts, whose own search finds the lenses here: the kit's sizes are named in one place)
  const VAN_AT = [4.5, 15.0] as const, van0 = K.boxes.length; G.van(VAN_AT[0], VAN_AT[1], HALF + 0.12);
  { const lenses = findCarLamps(K.boxes.slice(van0)).map(l => ({ what: l.what, b: K.boxes[van0 + l.box] })), heads = lenses.filter(l => l.what === 'head lamp').map(l => l.b), tails = lenses.filter(l => l.what === 'tail lamp').map(l => l.b);
    if (heads.length !== 2 || tails.length !== 2) throw new Error(`tut-kit: the van's lamps were not found (${heads.length} head, ${tails.length} tail): the kit's lenses have moved`);
    const mid = [(heads[0].c[0] + heads[1].c[0]) / 2, (heads[0].c[2] + heads[1].c[2]) / 2], l = Math.hypot(mid[0] - VAN_AT[0], mid[1] - VAN_AT[1]), nx = (mid[0] - VAN_AT[0]) / l, nz = (mid[1] - VAN_AT[1]) / l;
    heads.forEach((b, k) => {
      const fb = add({ c: [b.c[0] + nx * 0.04, b.c[1], b.c[2] + nz * 0.04], h: [0.008, b.h[1] + 0.006, b.h[2] + 0.006], yaw: b.yaw, ...M.plasticWhite, emissive: [34, 31, 24], flags: BoxFlag.NoShadow });
      lights.push({ kind: 'spot', pos: [b.c[0] + nx * 0.08, b.c[1], b.c[2] + nz * 0.08], dir: norm([nx, -0.1, nz]), color: [1.0, 0.93, 0.78], intensity: 150, range: 14, innerDeg: 12, outerDeg: 30, name: `van_lamp_${k}`, group: 'exterior', enabled: true, volumetric: 0.35, fixtureBox: fb, radius: 0.06, mains: false });
    });
    tails.forEach((b, k) => {   // (each a small red light of its own, so a round puts the glow out as it does any lamp's)
      const fb = add({ c: [b.c[0] - nx * 0.03, b.c[1], b.c[2] - nz * 0.03], h: [0.006, b.h[1] + 0.006, b.h[2] + 0.006], yaw: b.yaw, albedo: [0.5, 0.05, 0.04], rough: 0.4, emissive: [7, 0.4, 0.25], flags: BoxFlag.NoShadow });
      lights.push({ kind: 'point', pos: [b.c[0] - nx * 0.1, b.c[1], b.c[2] - nz * 0.1], dir: [0, -1, 0], color: [1.0, 0.12, 0.08], intensity: 1.6, range: 3, innerDeg: 0, outerDeg: 180, name: `van_tail_${k}`, group: 'exterior', enabled: true, volumetric: 0, fixtureBox: fb, radius: 0.04, mains: false });
    });
    const bay: Vec3 = [VAN_AT[0] - nx * 2.3, 1.84, VAN_AT[1] - nz * 2.3];
    const bayLens = add({ c: bay, h: [0.07, 0.012, 0.07], ...M.plasticWhite, emissive: [22, 19, 13], flags: BoxFlag.NoShadow });
    lights.push({ kind: 'point', pos: [bay[0], bay[1] - 0.05, bay[2]], dir: [0, -1, 0], color: [1.0, 0.86, 0.62], intensity: 16, range: 6, innerDeg: 0, outerDeg: 180, name: 'van_bay', group: 'exterior', enabled: true, volumetric: 0.2, fixtureBox: bayLens, radius: 0.05, mains: false });
  }
  G.patch(4.4, 15.3, 3.0, 4.6, 'oil', 0.1, 0.019); G.rubble(5.9, 16.6, 0.5, 5);
  G.lantern(8.55, 11.0, 'lantern_road', 'exterior');                // at the wall's foot beside the gate
  // THE SQUAD'S TRUCK, on the road behind where he starts, nose to the halt: the four wait at its tail until he calls them
  grp('road.truck');
  G.pickup(TRUCK_AT.x, TRUCK_AT.z, TRUCK_AT.yaw, 'truckTan');

  // ---- the truck court: a barrier across the gate's line, the inner wall and its door, the parked truck the gun finds ----
  grp('court.truck');
  G.jersey(12.5, AXIS, 2, HALF);
  // THE INNER DOOR. The gate's corner of the court is walled off from the way to the arch: a wall north from the barrier's end with one wooden door in it,
  // shut and not locked, and a second barrier from the first one's other end to the arch. The way to the blue gate is through that door: the first thing
  // the interaction key's tap is asked for, in a court nobody is shooting into yet
  G.yardWallZ(INNER_X, YARD.z0, 12.2, 'mudLight', [{ a: INNER_Z - 0.6, b: INNER_Z + 0.6, y0: 0, y1: 2.2 }]); G.pier(INNER_X, 12.0);
  doors.push({ name: 'inner', hinge: [INNER_X, INNER_Z - 0.58], closedDir: HALF, width: 1.16, swing: SWING_INNER, closer: false });
  G.jersey(14.1, 16.55, 2, 0);
  G.pickup(12.6, 19.2, 0.25, 'truckWhite', { load: 'drums' });
  G.drums(10.2, 21.6, 4); G.tyres(15.2, 21.8, 4); G.crates(10.0, 6.4, 0.1, 3); G.gasBottle(11.0, 6.2); G.gasBottle(11.4, 6.5);
  G.sacks(14.6, 6.5, 0.2, 4); G.patch(12.4, 18.8, 4.2, 2.6, 'oil', 0.3, 0.019);
  G.lantern(14.6, 13.0, 'lantern_sentry', 'yard', 0);
  G.floodLight(15.5, 22.6, [-0.6, -0.8], 'flood_court', true, 3.6, true);   // (in the corner by the arch's wall, thrown back across the truck court: from the far corner it went over that wall and lit the gun court's south half, the shop's door with it)

  // ---- the gun court: the nest in the far corner, the sandbags inside the arch, the gap, the barrier before the shop ----
  grp('court.gun');
  G.sandbags(22.6, 8.2, 3.0, 0, 4); G.sandbags(21.0, 7.2, 2.0, HALF, 4);            // the nest: two low walls, the gun between them
  G.crates(24.0, 6.2, 0.1, 2); G.patch(22.6, 7.0, 2.6, 2.0, 'damp');
  G.sandbags(18.675, 12.2, 4.75, 0, 6);                                            // inside the arch, chest high: a crouched man behind them is out of the gun's sight (a hand north of the gate's open leaf, which stands along them)
  G.hesco(23.4, 12.6, 2, 0);                                                       // the barrier before the shop's door
  G.drumDown(17.6, 16.8, 0.4); G.tyres(19.4, 17.4, 2); G.handcart(20.6, 20.4, 0.5, 'sacks'); G.woodPile(17.2, 21.8, 0.1); G.drums(24.2, 21.6, 3);
  G.blocks(17.4, 6.4, 0.1, 3); G.jerryCan(18.2, 6.2, 0.6);

  // the gun court's own lamp: a mast by the shop's corner, its arm out over the court, the head between the nest and the gap. It lights the gun, the sandbags'
  // far end and the gap, and stops short of the shop's door: the ground he stands on for the door is lit by the shop's lamp alone (the line that has him shoot it out)
  { const x = 22.3, z = 9.9, h = 4.6, px = 24.6;
    add({ c: [px, (h + 0.11) / 2, z], h: [0.05, (h + 0.11) / 2, 0.05], ...M.galv });
    add({ c: [(px + x) / 2, h + 0.085, z], h: [(px - x) / 2 + 0.1, 0.025, 0.025], ...M.galv });
    add({ c: [x, h + 0.06, z], h: [0.2, 0.05, 0.2], ...M.plasticBlack });
    const fb = add({ c: [x, h, z], h: [0.12, 0.012, 0.12], ...M.plasticWhite, emissive: [40, 34, 22], flags: BoxFlag.NoShadow });
    lights.push({ kind: 'spot', pos: [x, h - 0.03, z], dir: [0, -1, 0], color: [1.0, 0.8, 0.5], intensity: 120, range: 10, innerDeg: 34, outerDeg: 58, name: 'lamp_gun', group: 'exterior', enabled: true, volumetric: 0.25, fixtureBox: fb, radius: 0.1 });
  }

  // ---- the ground's marks: wheel tracks, worn earth, stains, the stones along the drive ----
  // Flat things are in three layers a millimetre and more apart (worn earth under the wheel tracks under a stain), each darker than what it lies on and none
  // under a third of it (STYLE.md). The low things are under 0.19 m: feet, wheels and the walk grid go over them, and they take the lamps and throw a shadow
  grp('ground.marks');
  { const worn = (x: number, z: number, w: number, d: number, yaw = 0, mat: GwotMat = 'dirt') => G.patch(x, z, w, d, mat, yaw, 0.0085);
    const stain = (x: number, z: number, w: number, d: number, yaw = 0, mat: GwotMat = 'damp') => G.patch(x, z, w, d, mat, yaw, 0.026);
    let nth = 0;
    /** a whitewashed stone at the drive's edge, as a checkpoint lines its way in */
    const stone = (x: number, z: number, turn = 0) => { const k = nth++, hy = 0.04 + (k % 3) * 0.008; add({ c: [x + ((k * 7) % 5 - 2) * (turn ? 0.03 : 0.06), hy, z + ((k * 3) % 5 - 2) * (turn ? 0.06 : 0.03)], h: [0.13 + (k % 3) * 0.025, hy, 0.07 + (k % 2) * 0.02], yaw: turn + ((k * 37) % 11 - 5) * 0.11, ...(k % 4 === 3 ? M.block : k % 4 === 1 ? M.mudLight : M.whitewash) }); };
    const stones = (z: number, xs: number[]) => xs.forEach(x => stone(x, z));
    // the road and where he starts: the tracks every truck has left, which turn off the road before the man's gate and go round the corner to the
    // trucks' own; the verge the van pulled onto; the forecourt's kerb; a foot path in from the man's gate
    G.ruts(0.3, AXIS + 0.04, 3.3, AXIS, 'damp'); G.ruts(5.9, AXIS, 6.6, AXIS + 0.25, 'damp');
    G.ruts(6.5, 15.9, 6.25, 18.0); G.ruts(6.25, 18.2, 6.2, 22.6); G.ruts(6.3, 22.9, 7.6, 24.5); G.ruts(7.9, 24.7, 11.7, 24.75); G.ruts(TRUCK_GATE_X, 24.9, TRUCK_GATE_X, 23.5);
    worn(4.5, 17.1, 8.4, 2.3, 0.02); worn(3.4, 18.7, 4.2, 1.5, -0.07);
    stones(DRIVE.z0 - 0.3, [0.9, 1.9, 2.9, 3.9, 4.9, 5.9, 6.9]); stones(15.85, [0.8, 1.8, 2.8]);
    G.rubble(1.3, 20.9, 0.6, 5); G.rubble(7.7, 21.4, 0.5, 4);
    // the truck court: the drive in from the trucks' gate and round to the arch (the white truck stands on it, as it came in), a foot path from the
    // man's gate, the earth under the stores, the sentry's fire
    G.ruts(TRUCK_GATE_X, 22.7, TRUCK_GATE_X, 21.3, 'damp'); G.ruts(TRUCK_GATE_X, 17.2, TRUCK_GATE_X + 0.15, 16.0, 'damp'); G.ruts(12.85, 15.8, 13.7, 14.6, 'damp'); G.ruts(13.9, AXIS + 0.02, 15.8, AXIS + 0.04, 'damp');
    stain(13.0, 18.5, 1.2, 0.8, 0.5); worn(10.0, AXIS, 1.9, 1.2, 0.03);
    worn(12.4, 6.8, 5.8, 2.3, -0.03); stain(11.2, 6.8, 1.4, 0.9, 0.2); worn(10.0, 21.4, 1.5, 1.7, 0.1); worn(12.6, 12.2, 4.2, 0.8, 0.02);
    worn(11.3, 10.2, 1.5, 1.3, 0.4, 'damp'); for (let k = 0; k < 7; k++) { const a = k * 0.9 + 0.2; add({ c: [11.3 + Math.cos(a) * 0.62, 0.05, 10.2 + Math.sin(a) * 0.55], h: [0.1, 0.05 + (k % 2) * 0.02, 0.08], yaw: a, ...(k % 3 ? M.mudDark : M.block) }); }
    stones(DRIVE.z0 - 0.13, [11.2, 12.2, 13.2, 14.0]); stones(DRIVE.z1 + 0.13, [14.9]); stone(TRUCK_GATE_X - 1.83, 13.0, HALF);
    for (const z of [16.7, 17.9, 19.1, 20.3, 21.5]) stone(TRUCK_GATE_X - 1.83, z, HALF); for (const z of [16.9, 18.1, 19.3, 20.5]) stone(TRUCK_GATE_X + 1.83, z, HALF);
    G.rubble(9.6, 8.9, 0.5, 5); G.rubble(15.4, 17.6, 0.5, 5);
    // the gun court: the tracks up to the shop's door and the arc a truck backs round on, the oil where it stands, the earth under the nest and the wood
    G.ruts(16.2, AXIS + 0.04, 18.9, AXIS, 'damp'); G.ruts(19.5, AXIS, 22.6, AXIS - 0.04, 'damp'); G.ruts(21.5, 15.1, 20.8, 16.7, 'damp'); G.ruts(20.75, 16.8, 20.25, 18.3, 'damp'); G.ruts(20.2, 18.6, 19.95, 20.7);
    G.patch(22.2, 14.4, 2.6, 1.1, 'oil', 0.06, 0.019); G.patch(22.6, 14.1, 1.3, 1.6, 'oil', -0.2, 0.0198); stain(22.7, 14.2, 0.9, 0.6, 0.4);
    worn(22.2, 7.6, 4.6, 3.2, 0.03); worn(17.7, 6.7, 3.0, 1.9, -0.05); worn(19.1, 21.0, 5.2, 2.7, -0.08); worn(17.9, 17.3, 2.4, 1.8, 0.15); worn(24.1, 21.4, 1.9, 1.7, 0.1, 'damp');
    stones(DRIVE.z1 + 0.13, [16.6, 17.6, 18.6]); stones(APRON.z1 + 0.13, [19.6, 20.6, 21.6, 22.6, 23.6]); stones(APRON.z0 - 0.13, [19.5, 20.4]);
    stone(APRON.x0 - 0.13, 16.7, HALF); stone(APRON.x0 - 0.13, 17.6, HALF);
    G.rubble(16.6, 9.4, 0.6, 6); G.rubble(24.3, 19.7, 0.5, 5); G.rubble(16.9, 19.5, 0.5, 4);
    // the plain round the halt: a foot track along the south wall, stones, the scrub nobody clears
    worn(25.3, 25.3, 21.4, 1.1, 0.012); worn(21.0, 2.6, 30, 1.0, -0.01);
    G.rubble(14.5, 24.4, 0.7, 6); G.rubble(30.2, 25.7, 0.7, 6); G.rubble(27.0, 3.6, 0.7, 5);
    G.bush(10.2, 26.5, 0.9); G.bush(18.6, 26.9, 1.1); G.bush(26.4, 24.5, 0.8); G.bush(33.2, 26.5, 1.0); G.bush(1.6, 23.2, 0.9); G.bush(5.6, 26.3, 0.8); G.bush(13.5, 2.2, 1.0); G.bush(31.0, 1.9, 0.9);
  }

  // ---- the building against the yard's far wall: the shop and the store behind it on the mission's line, the bunk room and the fuel room behind them ----
  grp('shop.floor'); G.ground(SHOP.x0, BUNK.z0, STORE.x1, SHOP.z1, 'slab', 0.006);
  grp('shop.roof'); slab(SHOP.x0 - 0.1, H, BUNK.z0 - 0.1, STORE.x1 + 0.1, H + 0.2, SHOP.z1 + 0.1, P.render, { flags: BoxFlag.NoPrimary, rough: 0.95 });
  grp('shop.walls');
  const bunkDoorway = { a: BUNK_DOOR_X - 0.6, b: BUNK_DOOR_X + 0.6 };
  G.wallX(ROOM_Z0, FRONT_X - t, YARD.x1 + t, 'render', [bunkDoorway]); G.wallX(ROOM_Z1, FRONT_X - t, YARD.x1 + t, 'render');
  // (one window, on the door's far side from where the charge's line stands him: the wall he waits against is blank)
  G.wallZ(FRONT_X, ROOM_Z0 + t, ROOM_Z1 - t, 'render', [{ a: 11.2, b: 12.5, y0: 0.9, y1: 2.0, glass: 'whole' }, DOORWAY]);
  // the store's back door, on the mission's line: steel, shut and locked from the store's side until the two for the launcher come in by it from the lane
  // behind the halt. Steel: no boot, pick or charge of his opens it
  G.wallZ(YARD.x1, ROOM_Z0 + t, ROOM_Z1 - t, 'render', [DOORWAY]);
  doors.push({ name: 'store', hinge: [YARD.x1, HINGE], closedDir: HALF, width: 1.16, locked: true, keySide: 1, swing: 1, steel: true, exterior: true, closer: false });
  G.wallZ(PART_X, ROOM_Z0 + t, ROOM_Z1 - t, 'render', [DOORWAY]);   // the doorway to the store: no leaf (a full wall, not a partition: a round that misses in the shop stops in it)
  doors.push({ name: 'shop', hinge: [FRONT_X, HINGE], closedDir: HALF, width: 1.16, locked: true, closer: false });   // (no closer: once it is open the doorway is a line of fire for the rest of the mission)
  G.dado('z', FRONT_X, 10.6, AXIS - 0.5, -1, 'paintGreen'); G.dado('z', FRONT_X, AXIS + 0.5, 17.4, -1, 'paintGreen');
  // THE TWO ROOMS BEHIND THE SHOP: blank walls to the gun court and the plain (nobody in them is seen or shot from outside), one door from the shop into the
  // bunk room, one locked door on from there into the fuel room. Full walls outside; the wall between the two rooms is a partition, the one wall here
  // the squad's charge goes through
  grp('back.walls');
  G.wallX(YARD.z0, FRONT_X - t, YARD.x1 + t, 'render'); G.wallZ(FRONT_X, YARD.z0 + t, ROOM_Z0 - t, 'render'); G.wallZ(YARD.x1, YARD.z0 + t, ROOM_Z0 - t, 'render');
  G.partZ(NPART_X, YARD.z0 + t, ROOM_Z0 - t, 'whitewash', [{ a: FUEL_DOOR_Z - 0.6, b: FUEL_DOOR_Z + 0.6 }]);   // (a PARTITION, whitewashed where the shell is render: built at the shell's thickness, wallcharge.ts refused it as concrete)
  doors.push({ name: 'bunk', hinge: [BUNK_DOOR_X - 0.58, ROOM_Z0], closedDir: 0, width: 1.16, swing: SWING_BUNK });   // (it opens into the bunk room: the stack is on the shop's side)
  doors.push({ name: 'fuel', hinge: [NPART_X, FUEL_DOOR_Z - 0.58], closedDir: HALF, width: 1.16, swing: -1, locked: true });   // (locked: its man has the key and his gun on it, the squad has a charge for the wall. Not steel: a door no tool opens is no way in at all to the level's own checks)
  zones.push({ id: 'shop', mat: 'stone', x0: SHOP.x0, z0: BUNK.z0, x1: STORE.x1, z1: SHOP.z1 });
  grp('shop.room');
  G.table(27.2, 16.6, 1.8, 0.7, 0); G.plasticChair(26.2, 16.4, 0.2);   // (the north wall is bare both sides of the bunk room's door: two metres each way for a stack)
  G.bulb(27.8, AXIS, 'bulb_shop', 'shop');
  grp('store.room');
  G.trunk(35.3, 11.5, HALF, 'paintGreen');
  G.crates(35.2, 15.75, 0.1, 3); G.crates(31.4, 16.8, 0.2, 2); G.drums(35.2, 16.8, 2); G.jerryCan(33.6, 11.0, 0.3);
  G.bulb(33.4, AXIS, 'bulb_store', 'store');
  grp('bunk.room');     // where the halt's men sleep: two cots under the far wall, a floor mattress, a trunk, tea
  G.cot(26.4, 5.75, 0); G.cot(28.9, 5.75, 0, false); G.toshak(25.6, 8.3, 1.9, HALF); G.trunk(25.55, 6.7, HALF, 'paintBlue');
  G.rug(27.9, 8.0, 2.6, 1.6, 'afghan'); G.lowTable(25.9, 9.8, 0.2); G.wallHanging('x', YARD.z0, 27.7, 1, 'baluch', 1.4, 0.9);
  G.bulb(28.0, 7.9, 'bulb_bunk', 'bunk');
  grp('fuel.room');     // the generator and what feeds it (the floor inside the charge's place on the wall is bare: a hole there is a way in)
  G.generator(34.6, 6.0, 0.1); G.drums(35.2, 9.7, 3); G.jerryCan(33.4, 10.0, 0.4, 'jerryYellow'); G.jerryCan(33.9, 10.05, -0.2); G.jerryCan(34.3, 9.95, 0.9, 'drumRed');
  G.patch(34.2, 6.9, 1.8, 1.2, 'oil', 0.2, 0.019); G.wire(34.6, 6.6, 34.6, ROOM_Z0 - 0.2);
  G.bulb(33.6, 8.0, 'bulb_fuel', 'fuel');

  // ---- a place somebody lives and works in. ADDITIONS ONLY, every one off the mission's line, off each man's mark and out of every door's sweep: none
  //  of it is cover a line asks for, and none of it changes a fight. It is the level's own furniture as built, on both stands, like the table and the
  //  crates above: not the lived-in pass (src/scene/lived.ts), which furnishes levels that were recorded bare ----
  grp('dress.road');     // the roadside: scrub along the verge
  G.bush(1.1, 17.4); G.bush(0.9, 25.6, 0.8); G.tyres(7.6, 19.8, 2);
  grp('dress.truck');    // the truck court's north corner: where the gate's men sit out a watch
  G.charpoy(10.6, 7.5, 0.12); G.lowTable(10.6, 16.7, 0.3); G.plasticChair(11.3, 17.1, 2.6); G.jerryCan(15.5, 7.3, 0.2);   // (nothing between the inner door and where he opens it from)
  grp('dress.shop');     // the shop is the room they live in: a stove, a floor mattress and tea, a rug under the table
  G.rug(27.5, 16.2, 3.2, 1.9, 'afghan'); G.toshak(29.3, 16.95, 1.9, 0); G.lowTable(29.2, 16.1, 0.1); G.stove(25.6, 15.7);
  G.gasBottle(30.0, 12.6);
  grp('dress.store');    // the store: flour and rice against the wall, a mat by the trunk
  G.sacks(32.6, 11.15, 0.1, 4); G.rug(34.4, 12.2, 1.2, 1.6, 'mat', HALF); G.gasBottle(30.95, 16.0);
  // THE WASH YARD: an open yard beside the building, a gateway on the gun court, a wall of sandbags across it three metres in, the back door the last
  // two come in by. SEVEN COURSES, 1.4 m: a man down behind it is out of sight of every rifle in the court (at six a teammate on his feet shot both
  // before the frag left a hand), and a man's arm still lobs a frag over it from the court (at eight, and from further than about six and a half
  // metres, nobody has the throw). The floor behind the bags is bare: a frag's place, and the way round their south end afterwards
  grp('wash.yard');
  G.sandbags(YARD_BAGS.x, YARD_BAGS.z, YARD_BAGS.len, HALF, 7);
  G.waterTank(26.9, 22.0); G.patch(27.3, 21.4, 2.2, 1.5, 'damp', 0.2); G.washLine(32.4, 22.3, 35.4, 22.3, 4); G.motorbike(34.9, 18.4, 0.3);
  G.bush(35.3, 21.6, 0.8); G.sacks(33.4, 22.2, 0.1, 3);
  G.lantern(33.0, 18.2, 'lantern_wash', 'exterior');

  // ---- light: the moon the hour swaps for its sun, a lamp over the gate and one over the shop's door. Every lamp on a pole and every bulb is on the
  //  mains (the breaker box on the gun court's wall puts them all out); the van's lamps and the lanterns are their own ----
  grp('lights');
  // (the moon stands behind the shop, low over the back of the halt: the shop's own front, the door he blows and the wall he waits against, is in its
  //  shadow, so the lamp over the door is the one light on him there and shooting it out leaves him in the dark)
  lights.push({ kind: 'dir', pos: [0, 0, 0], dir: norm([-0.62, -0.5, -0.6]), color: [0.55, 0.7, 1.0], intensity: 0.22, range: 1e6, innerDeg: 0, outerDeg: 0, name: 'moon', group: 'exterior', enabled: true, volumetric: 0, radius: 0.009, mains: false });
  const lamp = (x: number, z: number, name: string, h = 3.4, outerDeg = 52, side = -1) => {
    add({ c: [x, (h + 0.11) / 2, z + side * 0.6], h: [0.04, (h + 0.11) / 2, 0.04], ...M.galv });
    add({ c: [x, h + 0.085, z + side * 0.3], h: [0.025, 0.025, 0.26], ...M.galv });
    add({ c: [x, h + 0.06, z], h: [0.2, 0.05, 0.2], ...M.plasticBlack });
    const fb = add({ c: [x, h, z], h: [0.12, 0.012, 0.12], ...M.plasticWhite, emissive: [40, 34, 22], flags: BoxFlag.NoShadow });
    lights.push({ kind: 'spot', pos: [x, h - 0.03, z], dir: [0, -1, 0], color: [1.0, 0.8, 0.5], intensity: 90, range: 9, innerDeg: outerDeg - 22, outerDeg, name, group: 'exterior', enabled: true, volumetric: 0.25, fixtureBox: fb, radius: 0.1 });
  };
  lamp(10.4, 12.2, 'lamp_gate'); lamp(LAMP[0], LAMP[2], 'lamp_shop', LAMP[1], 46, 1);   // (its pole on the far side of its head: clear of the way from beside the door to the doorway, and of a pistol's line to the lamp from behind the barrier)

  // ---- the breaker box, on the gun court's north wall where the view sees it, and the beacon every level carries ----
  grp('breaker');
  const brk = add({ c: [BREAKER[0], BREAKER[1], BREAKER[2]], h: [0.28, 0.36, 0.045], albedo: [0.42, 0.44, 0.45], name: 'mains_box' });
  const led = add({ c: [BREAKER[0] - 0.18, 1.62, BREAKER[2] + 0.05], h: [0.015, 0.015, 0.01], albedo: P.plasticBlack, rough: 0.5, emissive: [0.3, 4, 0.6], flags: BoxFlag.NoShadow });
  add({ c: [BREAKER[0] + 0.2, 2.05, BREAKER[2] - 0.03], h: [0.015, 0.36, 0.012], ...M.plasticBlack });   // its cable, up the wall and over
  grp('beacons');
  add({ c: [22.9, 2.28, 5.18], h: [0.07, 0.06, 0.07], albedo: [0.2, 0.2, 0.22], flags: BoxFlag.NoShadow });
  const dome = add({ c: [22.9, 2.16, 5.2], h: [0.055, 0.07, 0.055], albedo: [0.35, 0.08, 0.05], flags: BoxFlag.NoShadow });
  switchables.push({ name: 'beacon0', boxes: [dome], group: 'emergency', on: false, baseEmissive: [9, 1.2, 0.35], mains: false });

  // ---- THE HALT: six men in its rooms, each on his mark until the script calls his beat (levels/tut-kit.script.ts names them down the rooms: Tango 1 in
  //  the shop, 2 and 3 in the store, 4 and 5 in the bunk room, 6 in the fuel room). Every room's marks are exactly its garrison: there is one deal. The
  //  man on the gun, the two for the launcher and the two of the wash yard are the script's own ----
  const SHOP_IN: Vec3 = [FRONT_X + 0.4, 1.2, AXIS], PART_IN: Vec3 = [PART_X, 1.2, AXIS], BUNK_IN: Vec3 = [BUNK_DOOR_X, 1.1, ROOM_Z0 - 0.3], FUEL_IN: Vec3 = [NPART_X + 0.3, 1.1, FUEL_DOOR_Z];
  const structure: StructureData = {
    name: 'the fuel halt', maxMen: 6,
    wallCharges: 2,
    team: { sam: START, samYaw: Math.PI, ops: OPS, opsYaw: Math.PI },
    rooms: [
      { id: 'shop', name: 'shop', bounds: { x0: SHOP.x0 + 0.12, x1: STORE.x1 - 0.12, z0: SHOP.z0 + 0.12, z1: SHOP.z1 - 0.12 }, garrison: 3, doors: [{ id: 'shop', from: 'gun court' }, { id: 'store', from: 'lane' }, { id: 'bunk', from: 'bunk room' }],
        marks: [{ pos: SHOP_MAN, watch: 'shop', aimAt: SHOP_IN }, { pos: STORE_MEN[0], watch: 'shop', aimAt: PART_IN }, { pos: STORE_MEN[1], watch: 'shop', aimAt: PART_IN }] },
      { id: 'bunk', name: 'bunk room', bounds: inner(BUNK), garrison: 2, doors: [{ id: 'bunk', from: 'shop' }, { id: 'fuel', from: 'fuel room' }],
        marks: [{ pos: BUNK_MEN[0], watch: 'bunk', aimAt: BUNK_IN }, { pos: BUNK_MEN[1], watch: 'bunk', aimAt: BUNK_IN }] },
      { id: 'fuel', name: 'fuel room', bounds: inner(FUEL), garrison: 1, doors: [{ id: 'fuel', from: 'bunk room' }],
        marks: [{ pos: FUEL_MAN, watch: 'fuel', aimAt: FUEL_IN }] },
    ],
    squads: [
      { id: 'front', name: 'Front', rooms: ['shop'], rally: [27.7, 0, AXIS], mayLeave: false },
      { id: 'back', name: 'Back', rooms: ['bunk', 'fuel'], rally: [28.0, 0, 7.9], mayLeave: false },
    ],
  };

  const DOWN = (n: number) => ({ is: 'men', down: n } as const);
  const BEHIND: Rect = { x0: 22.3, x1: 24.6, z0: 13.3, z1: 15.3 };
  /** where a smoke can hides the gap from the gun: the court between the two, the gap itself included (a can thrown back the way he came does not pass) */
  const SMOKED: Rect = { x0: 21.4, x1: 24.8, z0: 7.6, z1: 14.9 };   // (from past where the line before left him: a can dropped at his own feet behind the sandbags hides nothing)
  /** the men down by the end of each fight: the gun, the shop's, the store's two, the launcher's two, the bunk room's two, the fuel room's, the wash yard's two */
  const N = { gun: 1, shop: 2, store: 4, last: 6, bunk: 8, fuel: 9, yard: 11 };
  const TAKEN = (room: 'bunk' | 'fuel', R: Rect) => ({ is: 'any', of: [{ is: 'taken', room }, { is: 'squad', area: inner(R), n: 3 }] } as const);
  const lesson: LessonData = {
    title: 'Your kit, then the squad\'s',
    // (what the first two missions taught is his from the first step: his feet, his rifle, the view. NO FOG OF WAR, as on the mission before: the squad's
    //  own charge and frag go off where he is not standing, and the lesson is to see them. The squad's ring is the game's own, every row of it
    //  (no `orders` list): the digits a line names here are the digits the rest of the game has. His own ring's rows are the ones a line teaches: a
    //  door opened and shut by hand, his charge)
    rails: { allow: ['move', 'sprint', 'crouch', 'wall', 'aim', 'fire', 'reload', 'mode', 'slot:rifle', 'camera', 'zoom'], respawn: 5, fog: false, lasers: false, flatRing: true, waits: { low: true }, rows: OWN_ROWS },
    steps: ([
      // ================= HIS OWN KIT, ALONE =================
      // THE TRUCK COURT: a tap of the interaction key on the thing in front of him
      { id: 'gate', line: 'Leave your squad at the truck and go in to the wooden door', done: { is: 'near', at: INNER_FROM, r: 0.7 }, at: INNER_FROM },   // (inside the door's reach: the next line's tap has its row wherever in the mark he stops)
      { id: 'open', line: 'Press [Space] to open the door', done: { is: 'door', door: 'inner', state: 'open' }, settle: 0.5, at: [INNER_X, 1.1, INNER_Z] },
      { id: 'arch', line: 'Go through and move up beside the blue gate', done: { is: 'near', at: JAMB, r: 1.3 }, hold: 1, at: JAMB },   // (a second there: the gun opens on the truck before the next line, whenever he came)
      // THE GUN COURT: a smoke to cross behind, a frag for the nest
      { id: 'low', line: 'Press [C] to crouch, then follow the sandbags', done: { is: 'all', of: [{ is: 'flag', flag: 'crouch' }, { is: 'near', at: BAGS_END, r: 1.1 }] }, at: BAGS_END },
      { id: 'smoke', line: 'Press [3], then click the ring to throw smoke there', done: { is: 'any', of: [{ is: 'threw', kind: 'smoke', area: SMOKED }, DOWN(N.gun)] }, hold: 2.5, at: GAP },
      { id: 'cross', line: 'Cross behind the smoke', done: { is: 'in', area: BEHIND }, at: BARRIER },
      { id: 'frag', line: 'Press [5], then click the ring to throw a frag at the gun', done: DOWN(N.gun), settle: 1.5, at: NEST },
      // THE SHOP'S DOOR: the pistol and the lamp (his half of the dark), then the key held, the ring, Tab, his own charge
      // (his one frag thrown, the game puts the pistol in his hands by itself and the first of these two lines goes by unread: it is there for the man
      //  who shot the gunner with his rifle instead. So the lamp's line names the pistol too)
      { id: 'pistol', line: 'Press [1] to switch to your pistol, because it\'s much quieter', done: { is: 'any', of: [{ is: 'slot', slot: 'pistol', drawn: true }, DOWN(N.shop)] } },   // (…or the shop's man is down already: a player who kept his rifle is not held on this line through the rooms after it)
      { id: 'lamp', line: 'Shoot out the light with your pistol so you can hide in the dark', done: { is: 'any', of: [{ is: 'target', name: 'lamp_shop', on: false }, DOWN(N.shop)] }, settle: 0.8, at: LAMP },
      // (the ring is put up and put away before anything is picked in it. Nothing is taken from him for the Tab line: with the interaction key locked the
      //  frame drops his ring by itself and the line would go by unread. A man who picks the charge straight from the first ring has done the line
      //  after it, and both go by)
      { id: 'ring', line: 'Hold [Space] on the door to see what you can do', done: { is: 'any', of: [{ is: 'flag', flag: 'ownRing' }, { is: 'door', door: 'shop', state: 'charged' }, { is: 'door', door: 'shop', state: 'open' }] }, at: SHOP_DOOR,
        explain: { title: 'Using things', text: 'Press [Space] to use what\'s in front of you, like the door you just opened. This door is locked, so a tap won\'t open it.\nHold [Space] and a ring will come up with everything you can do there.' } },
      { id: 'tab', line: 'Press [Tab] to close it, like any menu', done: { is: 'flag', flag: 'ownRing', value: false } },
      { id: 'charge', line: 'Hold [Space] on the door again and pick the charge', done: { is: 'any', of: [{ is: 'door', door: 'shop', state: 'charged' }, { is: 'door', door: 'shop', state: 'open' }] }, at: SHOP_DOOR },
      { id: 'back', line: 'Stand clear with your back to the wall beside the door', done: { is: 'any', of: [{ is: 'near', at: BACK, r: 0.6 }, { is: 'door', door: 'shop', state: 'open' }] }, at: BACK },
      // (`blowLine`: his own charge is a row of his own actions where that rule is on — game/ownblow.ts — and stood clear it is the only row, so a tap gives it)
      { id: 'blow', line: 'Press [F] to blow the door', blowLine: 'Press [Space] to blow the door', done: { is: 'door', door: 'shop', state: 'open' }, settle: 0.6, at: SHOP_DOOR },
      { id: 'shop', line: 'Lean in and shoot the guard inside', done: DOWN(N.shop), settle: 1.5, at: [SHOP_MAN[0], 1.0, SHOP_MAN[2]] },
      // THE STORE: a stun grenade through a doorway (the store's doorway has no leaf and stands in line with the shop's door, so a man who throws from
      // inside the shop is looking down that line: said before the throw, on a card, with the one thing to do about it in the line)
      { id: 'stun', line: 'Press [4], then click the ring to throw a stun grenade, and get behind the wall', done: { is: 'any', of: [{ is: 'threw', kind: 'flash', area: STORE }, DOWN(N.store)] }, settle: 0.4, at: STUN_AT,
        explain: { title: 'Throwing a stun grenade', text: 'A stun grenade blinds anyone nearby who can see it go off, and that includes you. Walls and closed doors block the flash, but an open doorway doesn\'t. Throw it through the doorway, then move to the side so you\'re behind the wall when it goes off.' } },
      { id: 'store', line: 'Go in and shoot them both', done: DOWN(N.store), settle: 1.5, at: [33.6, 1.0, AXIS] },
      // THE LAUNCHER: back at the arch with the rifle, two more in the doorway he has just come out of
      { id: 'rifle', line: 'Press [2] to switch back to your rifle', done: { is: 'slot', slot: 'rifle', drawn: true }, settle: 0.8 },
      { id: 'leave', line: 'Go back to the blue gate', done: { is: 'any', of: [{ is: 'near', at: LAUNCH_FROM, r: 0.5 }, DOWN(N.last)] }, at: LAUNCH_FROM,
        explain: { title: 'Using the rifle\'s launcher', text: 'Your rifle has a grenade launcher under the barrel. You can see how many grenades you\'ve got next to your ammo at the bottom left. Hold the right mouse button to aim and let go to fire. A grenade won\'t go off if it lands too close to you.' } },
      // (the two it is for come out first, on a line that only asks him to look; the script ends that line once they stand in the doorway)
      { id: 'behind', line: 'Turn around and watch the shop doorway for two more enemies', done: { is: 'cue', name: 'trucks' } },
      { id: 'launch', line: 'Hold the right mouse button, aim at the doorway, and let go', done: { is: 'any', of: [{ is: 'threw', kind: 'he40' }, DOWN(N.last)] }, settle: 1.5, at: LAST_COVER },   // (his one grenade fired, hit or miss: the line must not go on asking for a launcher that is empty)
      { id: 'last', line: 'Shoot the rest', done: DOWN(N.last), settle: 1.5, at: LAST_COVER },

      // ================= THE SQUAD'S KIT, ON HIS WORD =================
      // THE FOUR COME UP (the tap on himself the mission before taught: here it has a use)
      { id: 'call', line: 'Press [B] on yourself to call your squad up', after: { line: 'They\'re coming up to you' }, onWay: { run: true }, at: [ARCH_MOUTH[0] + 1.6, 0.02, AXIS], done: { is: 'squad', area: INSIDE, n: 3 }, hold: 0.6 },
      // THE DARK: a tap of the command key on a thing sends a man to work it (the card says what light is to both sides before the lights go)
      { id: 'breaker', line: 'Press [B] on the breaker box to cut the lights', after: { line: 'A buddy team is going for the breaker box' }, at: BREAKER, done: { is: 'flag', flag: 'dark' },
        explain: { quiet: true, title: 'Light', text: 'Enemies can only fight what they can see. In the dark they\'ll slow down and turn on flashlights.\nYour squad has night vision, so cut the lights and you\'ll own the dark.' } },
      { id: 'nv', line: 'Press [N] to turn on night vision', done: { is: 'flag', flag: 'nv' } },
      // THE BUNK ROOM: the command key HELD is the squad's ring. First only the hold, then the wedge once the ring is up, named by its words with its digit
      { id: 'door', line: '[X] on the bunk room door', flatLine: 'Pick bang and clear with [4]', menuLine: 'Pick bang and clear with [4]', noTap: 'Keep [B] held down', after: { line: 'They\'re stacking up with a stun grenade ready', word: 'They\'re going in behind the stun grenade', on: ['bangClear'] }, at: BUNK_DOOR, done: TAKEN('bunk', BUNK), hold: 1.5,
        explain: { quiet: true, title: 'The squad\'s ring', text: 'A tap of [B] gives your squad the usual order for whatever you\'re pointing at.\n[X] and a ring will come up with everything else they can do there. On a door, they can throw a stun grenade in first.' } },
      // THE FUEL ROOM: a wall charge (a door is where a room expects them, and this one is locked with a gun on it)
      { id: 'wall', line: '[X] on the wall', flatLine: 'Press [5] for more, then pick the wall charge with [3]', menuLine: 'Press [5] for more, then pick the wall charge with [3]', noTap: 'Keep [B] held down', after: { line: 'They\'re putting the charge on the wall', on: ['wallCharge'] }, eye: WALL_EYE, at: WALL_MARK,
        done: { is: 'any', of: [{ is: 'flag', flag: 'chargeClear' }, ...TAKEN('fuel', FUEL).of] }, hold: 0.4,
        explain: { quiet: true, title: 'Walls', text: 'The next room\'s door is locked, and the enemy inside is watching it anyway.\nA charge on the wall will get your squad in where nobody\'s waiting.' } },
      { id: 'breach', line: 'Press [F] to blow it', after: { word: 'They\'re going through the wall' }, eye: WALL_EYE, at: WALL_MARK, done: TAKEN('fuel', FUEL), hold: 1.5 },
      // THE WASH YARD: a throw on his order, over cover no rifle reaches past. He leads the four back out; two come in by the yard's back door behind
      // their flashlights and get down behind the bags (the script ends the line that has him look once they are set)
      { id: 'lead', line: 'Press [B] on yourself, then lead your squad back out to the court', at: YARD_FROM, done: { is: 'all', of: [{ is: 'near', at: YARD_FROM, r: 1.6 }, { is: 'squad', area: COURT, n: 2 }] } },
      { id: 'lights', line: 'Two more are hiding in the yard, and you can see their flashlights', at: FRAG_AT, done: { is: 'any', of: [{ is: 'cue', name: 'seen' }, DOWN(N.yard)] } },
      { id: 'throw', line: '[X] on the ground behind the sandbags', flatLine: 'Press [5] for more, then pick the frag with [1]', menuLine: 'Press [5] for more, then pick the frag with [1]', noTap: 'Keep [B] held down', after: { line: 'A teammate is throwing a frag over the sandbags', on: ['fragThere'] }, at: FRAG_AT,
        done: { is: 'any', of: [{ is: 'all', of: [{ is: 'did', what: 'order:fragThere' }, DOWN(N.yard - 1)] }, DOWN(N.yard)] }, settle: 1.5 },
      { id: 'clear', line: 'Finish off anyone who\'s left', at: FRAG_AT, done: DOWN(N.yard), settle: 1.5 },
    ] as LessonStep[]).map(T => ({ ...T, ...FRAME[T.id] })),
  };

  for (const d of doors) doorFrame(d);
  const level: Level = {
    boxes: K.boxes, props: K.props, lights, switchables, doors, devices: [],
    breaker: { pos: [BREAKER[0], BREAKER[1], BREAKER[2] + 0.14], boxes: [brk, led] }, beacons: [{ pos: [22.9, 2.05, 5.3], box: dome }],
    playerSpawn: START, routes: [], chokepoints: [], clearPlans: [],   // nobody patrols it: the room clear brings its own cast
    sky: { zenith: [0.003, 0.004, 0.008], horizon: [0.007, 0.008, 0.012] },   // (a dark sky over a thin moon: the plain past the lamps and the dark past the world's edge are one dark)
    // (the script's eyes: a play program may ask of the player and his team only whether they stand in a volume)
    volumes: [
      { name: 'shop_line', kind: 'trigger', c: [FRONT_X - 1.35, 1, AXIS], h: [1.35, 1, 0.6] },
      { name: 'shop_in', kind: 'trigger', c: [(FRONT_X + PART_X) / 2, 1, (ROOM_Z0 + ROOM_Z1) / 2], h: [(PART_X - FRONT_X) / 2 - 0.05, 1, (ROOM_Z1 - ROOM_Z0) / 2] },
      { name: 'store_in', kind: 'trigger', c: [(PART_X + YARD.x1) / 2 + 0.05, 1, (ROOM_Z0 + ROOM_Z1) / 2], h: [(YARD.x1 - PART_X) / 2 - 0.1, 1, (ROOM_Z1 - ROOM_Z0) / 2] },
      { name: 'bunk_in', kind: 'trigger', c: [(BUNK.x0 + BUNK.x1) / 2, 1, (BUNK.z0 + BUNK.z1) / 2], h: [(BUNK.x1 - BUNK.x0) / 2 - 0.1, 1, (BUNK.z1 - BUNK.z0) / 2 - 0.1] },
      { name: 'fuel_in', kind: 'trigger', c: [(FUEL.x0 + FUEL.x1) / 2, 1, (FUEL.z0 + FUEL.z1) / 2], h: [(FUEL.x1 - FUEL.x0) / 2 - 0.1, 1, (FUEL.z1 - FUEL.z0) / 2 - 0.1] },
      { name: 'wash_in', kind: 'trigger', c: [(WASH.x0 + WASH.x1) / 2, 1, (WASH.z0 + WASH.z1) / 2], h: [(WASH.x1 - WASH.x0) / 2 - 0.1, 1, (WASH.z1 - WASH.z0) / 2 - 0.1] },
    ],
    // the stealth mission's three places, because the level format wants them (the room clear stands the mission still)
    mission: { serverRoom: { x0: SHOP.x0, x1: STORE.x1, z0: SHOP.z0, z1: SHOP.z1 }, entryDoor: 'shop', rack: { index: 0, front: [34.86, 0.45, 11.5], halfW: 0.3 }, exfilDoor: 'gate_w', exfilX: 8.2, exfilZ: AXIS, exfilOut: [-1, 0] },
  };
  const surfaces: SurfacesData = { building: { ...YARD }, grassEdge: 0, inside: 'asphalt', outside: 'asphalt', verge: 'asphalt', zones };
  const scenarios: LevelScenarios = { roomClear: structure, lesson, issue: ISSUE,
    doorNames: [{ door: 'gate_w', name: 'yard gate' }, { door: 'inner', name: 'inner door' }, { door: 'shop', name: 'shop door' }, { door: 'store', name: 'back door' }, { door: 'bunk', name: 'bunk room door' }, { door: 'fuel', name: 'fuel room door' }, { door: 'wash', name: 'yard door' }],
    places: { areas: [
      { name: 'the road', bounds: { x0: 0, x1: YARD.x0, z0: 0, z1: 28 } }, { name: 'the truck court', bounds: { x0: YARD.x0, x1: ARCH_X, z0: YARD.z0, z1: YARD.z1 } },
      { name: 'the gun court', bounds: { x0: ARCH_X, x1: FRONT_X, z0: YARD.z0, z1: YARD.z1 } }, { name: 'the wash yard', bounds: { ...WASH } },
    ] } };
  return { level, scenarios, groups: trace.groups, surfaces };
}
