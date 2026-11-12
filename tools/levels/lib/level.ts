// A LEVEL AS A GENERATOR BUILDS IT AND THE LOADER HANDS IT BACK (Box[] with bit flags and array indices, LightDef[], PropDef[], doors, routes, plans…): the shape level-file tools work on.
// The file's own shape is levelData.ts; levelLoad.ts turns one into the other. (The game reads the file itself: rust/game_start level.rs.) Units: meters. +X east, +Z south, +Y up. Floor top at y=0.
import type { Vec3 } from '../../../src/math/vec';
import type { LightDef } from '../../../src/render/lights';
import type { Box } from '../../../src/scene/boxes';
import type { MarkingDef } from '../../../src/scene/markings';
import type { SurfacesData } from './levelData';
import type { LevelScenarios } from './scenarios';

export type { LightDef };
/** a gun by its word, as a level file's routes hand one out */
export type WeaponId = 'fiveseven' | 'guard9mm' | 'aks74u' | 'ak74m' | 'akms' | 'fn2000' | 'm870' | 'mk12' | 'mp7' | 'mk46' | 'm1014' | 'hk45' | 'g18' | 'pkm' | 'mk18bare' | 'g19bare';

/** One man's beat. `weapon`: the rust/game_rules/src/guns/weapons.rs row he carries (absent = the 9 mm service pistol) — dealt at spawn (rust/game_rules/src/tick/game Guard); nobody on this level is authored one
 *  yet (the carbine lanes decide who). `keys`: the keys on his belt, by lock name (rust/game_rules/src/fittings/doors DoorDef.lock) — what Sam takes off
 *  him when he gives the keyed door up under questioning (rust/game_rules/src/sam/player take_key); absent = the floor key every man on nights carries (rust/game_rules/src/fittings/doors FLOOR_KEY). The leaves
 *  themselves treat every man on the staff as keyed (rust/game_rules/src/tick/game, `key: true`): a route authored keyless would want that contact narrowed too. */
export interface PatrolRoute { name: string; points: Vec3[]; wait: number[]; weapon?: WeaponId; keys?: string[]; }
/** A corridor junction one man can hold in a lockdown (rust/game_rules/src/people/guards): planted at `pos` facing `yaw` (Character convention: forward = [sin yaw, 0, cos yaw]), sweeping. */
export interface Chokepoint { name: string; pos: Vec3; yaw: number; }
/** What a civilian does at a waypoint of her loop while its wait runs (rust/game_rules/src/people/civilians words and poses each): stand (nothing), wipe (a desk / a counter, hands
 *  working at it), bin (kneel and empty it), sit (at the desk there, typing — the late worker's), cooler / printer / shelf (stand at the thing and use it). */
export type CivilianTask = 'stand' | 'wipe' | 'bin' | 'sit' | 'cooler' | 'printer' | 'shelf';
export const CIVILIAN_TASKS: CivilianTask[] = ['stand', 'wipe', 'bin', 'sit', 'cooler', 'printer', 'shelf'];
/** One of the building's own night people (rust/game_rules/src/people/civilians): a cleaner, a late worker. `name` is what the men call her; `kind` picks her rules of the two;
 *  `tint` / `tint2` the tabard-or-shirt and the trousers; the loop is a patrol route's shape — `points` walked in order at a walk, `wait` seconds stood at each —
 *  plus, per waypoint, the `tasks` done there while the wait runs and the yaw she does it `face`-ing (Character convention; absent = as she arrived). She starts
 *  on points[0]. Nothing else about her is authored: no weapon, no keys, no torch, no radio. */
export interface CivilianDef { name: string; kind: 'cleaner' | 'worker'; tint: Vec3; tint2?: Vec3; points: Vec3[]; wait: number[]; tasks?: CivilianTask[]; face?: number[]; }
/** The geometry and the talk for two men clearing one room (rust/game_rules/src/squad_orders/squad RoomClear runs it,
 *  the live AI takes `Level.clearPlans` in a lockdown). #1 is the point man: he stacks on the jamb, breaches and takes the open side of the room deep; #2 covers
 *  high behind him, follows him in a beat later onto the door side (the dead space behind an inward-opening leaf is his), then drops to a knee and puts his
 *  light under the furniture while #1 pushes up his wall. Every position must be reachable in a straight, clear leg from the one before it (the nav grid knows
 *  nothing of chairs); faces are body yaws (Character.forward = [sin, 0, cos]); aim lists are corners the muzzle visits in order. */
export interface RoomClearPlan {
  door: string;                                                        // Doors.byName — staged: the caller shuts and latches it before start(); live: taken as found (open = no breach, just the shove and in)
  room?: string;                                                       // what they call it on the net ('server room'): the live dispatcher's own lines
  breach?: 'kick' | 'open';                                            // a shut leaf: the boot (staged, or a door nobody has a key to) or — the default is 'kick' only because the tour's plan predates the field — unlatch it keyed and shove it hard into the room ('open': what staff do to their own doors)
  bounds?: { x0: number; x1: number; z0: number; z1: number };         // the room's floor inside its walls (plan validation)
  from?: [Vec3, Vec3];                                                 // staged only: where start() stands the pair before they walk into the stack (live, they come as they are)
  stack: [Vec3, Vec3]; stackFace: [number, number]; stackSide: -1 | 1; stackAim: [Vec3, Vec3];
  kickFrom: Vec3; kickFace: number; fling: number;                    // #1's breaching mark (Doors.kickSpot: square to the leaf, 0.76 m off it, by the lock), and the signed leaf velocity that throws it INTO the room (kick or shove)
  entry: [Vec3[], Vec3[]]; podFace: [number, number]; entryAim: [Vec3[], Vec3[]];   // waypoints through the doorway to each point of domination (the last one), the facing held there, the sector swept getting there
  searchPath: Vec3[]; searchFace: number; searchAimWalk: Vec3[]; searchAimThere: Vec3[];   // #1: up his wall to the far corner, checking ahead as he goes, then across the room from there
  underAim: Vec3[]; upAim: Vec3;                                       // #2: on a knee at his point, light under the table end to end, then back up onto his wall
  formUp: [Vec3[], Vec3[]]; formFace: [number, number];               // waypoints back to the door (last = where each waits), facing there
  lines: { stack1: string; stack2: string; go2: string; in1: string; side2: string; side1: string; push1: string; under2: string; nothing2: string; corner1: string; empty2: string; radio1: string };
}

/** The breach drill's plan (rust/game_rules/src/missions/breach.rs BreachDrill — written for the removed daytime showcase, run live by the night breach): a RoomClearPlan for the door the pair takes (stack marks and the
 *  kick are the drill's own fields and mean what they mean there) plus what only a hot breach has — the men waiting inside on their marks, #1's step BACK off
 *  the doorway after his boot (his own side of the frame: he never crosses the open threshold), #2's mark tight on the jamb with the stun charge in his support
 *  hand and where it lands, each man's flow through the room (battle drill six: through the fatal funnel a stride and a half, check the centre, engage what
 *  stands in it, turn to HIS corner and clear it, hold his sector from his point of domination — the two ending in an L, #2's point covering the room's next
 *  door), whom each engages on the way, and the next threshold both then stack on. entry / entryAim / searchPath etc. stay authored (entry = the flows' own
 *  legs) so RoomClear can dry-run the same door. */
export interface BreachPlan extends RoomClearPlan {
  /** unit XZ pointing through the doorway INTO the room (where the muzzles go once the leaf is off: rust/game_rules/src/missions/breach.rs) — absent = +x, the break room's west door, the
   *  only door the level's own plan takes; a plan built in code for another door (rust/game_rules/src/missions/rescue: the manager's office, north of the corridor) says its own */
  inward?: [number, number];
  /** the opposition: pinned on these marks facing `yaw`, muzzle on `aimAt` (the doorway, mostly) — one standing in the room's middle for #1's centre check */
  hostiles: { pos: Vec3; yaw: number; crouch?: boolean; aimAt: Vec3 }[];
  /** #1 the frame his sole meets the leaf: the step back to this mark on his own side of the frame (strafing, chest kept on `face` = the doorway) */
  retreat: { mark: Vec3; face: number };
  /** #2 while #1 steps out to kick: up the wall to this mark tight on the latch jamb facing `face`, the charge in his support hand; it leaves the hand the moment
   *  the leaf clears and comes down at `land` inside */
  toss: { mark: Vec3; face: number; land: Vec3 };
  /** each man's flow once the charge has gone off (#1 first, #2 on his heels) — see BreachFlow */
  flow: [BreachFlow, BreachFlow];
  /** hostile indices each man engages, in order: on his centre check, then in his sector as he turns to his corner (every line checked clear of the other man's leg) */
  targets: [{ centre: number[]; corner: number[] }, { centre: number[]; corner: number[] }];
  /** the room's NEXT threshold — the door #2 ends up holding from his point, and the stack both then take on it: #2 nearest the leaf (he becomes the point),
   *  #1 tucked in behind, bladed with the wall on `stackSide`, muzzles on `stackAim`; `via` = a lane through the room clear of its furniture, in order from the
   *  entry side, for a man whose straight walk to his mark is blocked (rust/game_rules/src/missions/breach.rs takes as much of it as it needs) */
  next: { door: string; stack: [Vec3, Vec3]; stackFace: [number, number]; stackSide: -1 | 1; stackAim: [Vec3, Vec3]; via?: Vec3[] };
  /** what they say that the drill does not: #1 stepping out to breach, the charge going in, the entry, each man's corner as he clears it, the room called, the
   *  word for the next door, the radio line */
  breachLines: { breach1: string; bang2: string; going1: string; clear1: string; clear2: string; roomClear1: string; next1: string; radio1: string };
}
/** One man's way through the room (rust/game_rules/src/missions/breach.rs OpFlow runs it): `into` = waypoints through the doorway to the spot a stride and a half inside where he checks
 *  the centre (last point; the first should be the doorway's mouth on his side so he takes the frame square), `centre` = the muzzle's check across the room's
 *  middle (from → to, ~0.3 s), `corner` = waypoints on to his point of domination (last), `podFace` = the body yaw held there, `cornerAim` = the sweep that
 *  clears HIS corner on the way (sector edge → … → into the corner itself), `hold` = where the muzzle rests once he owns his sector (for #2: the next door). */
export interface BreachFlow { into: Vec3[]; centre: [Vec3, Vec3]; corner: Vec3[]; podFace: number; cornerAim: Vec3[]; hold: Vec3 }

/** locked: a keyed lever set, cylinder on the corridor face (rust/game_rules/src/fittings/doors DoorDef.locked — guards carry keys; the player picks it or kicks it in, or takes a key off a man he
 *  questions; the room side always opens). lock: which key turns it (rust/game_rules/src/fittings/doors DoorDef.lock) — absent = the floor key; every keyed door here takes that one. */
export interface LevelDoorDef { name: string; hinge: [number, number]; closedDir: number; width: number; angle?: number; exterior?: boolean; closer?: boolean; minAngle?: number; maxAngle?: number; locked?: boolean; lock?: string; keySide?: 1 | -1; steel?: boolean; /** the side it opens into (rust/game_rules/src/fittings/doors DoorDef.swing); absent = dealt from the level (rust/game_rules/src/fittings/doorswing.rs) */ swing?: 1 | -1; }   // steel: a fire door a boot only rings off and no pick opens (rust/game_rules/src/fittings/doors DoorDef.steel — the QRF's west stair door, rust/game_rules/src/pursuit/qrf)
/** The level-design devices as the game runs them (rust/game_rules/src/stealth/devices.rs) — the file's records (levelData.ts DeviceData, every field documented there) with the id as
 *  `name`, the way lights and doors carry theirs. Optional fields keep the file's meaning: absent = the default in levelData.ts DEVICE_DEFAULTS. */
export type DeviceDef =
  | { kind: 'camera'; name: string; pos: Vec3; yaw: number; pitch?: number; sweep?: [number, number]; sweepPeriod?: number; fovDeg?: number; range?: number; group?: string; mains?: boolean }
  | { kind: 'keypad'; name: string; pos: Vec3; yaw: number; /** LevelDoorDef.name — that door's `lock` is this keypad's name */ door: string; code: string }
  | { kind: 'laser'; name: string; from: Vec3; to: Vec3; group?: string; mains?: boolean; alarm?: 'suspicion' | 'alarm' };
/** An HVAC supply VENT (rust/game_rules/src/air/wind.rs — the file's VentData with the id as `name`): a ceiling / wall diffuser at `pos` (its face centre) blowing along `dir` at
 *  `speed` m/s at the face, a cone of half-angle `spread` degrees whose air slows to half by `reach` m (∝ 1/(1 + (d/reach)²)) and is gone by 2.5 × reach; a jet
 *  aimed at the floor turns along it in its last half metre. `size` = the face's width (m, absent 0.6: the grille and the jet's core); `period` / `duty` / `phase`
 *  = slow cycling (s, on-fraction, offset; absent = steady); `mains` (absent = true): the air handler is on the breaker — the vents spin down in a blackout.
 *  ONE field: it moves the GPU particles (paper trembles and slides under a diffuser, dust drifts) and forces the smoke solver's air (a plume under a vent bends),
 *  so the guards' smoke sight lines follow it for free. */
/** an authored volume at run time (levelData.ts VolumeData: id → name; an oriented box — centre, half-extents, yaw — of a kind) */
export interface VolumeDef { name: string; kind: 'hide' | 'utility' | 'trigger'; c: Vec3; h: Vec3; yaw?: number; }
/** a named mark at run time (a place + a facing) */
export interface MarkDef { name: string; pos: Vec3; yaw?: number; }
export interface VentDef { name: string; pos: Vec3; dir: Vec3; speed: number; spread: number; reach: number; size?: number; period?: number; duty?: number; phase?: number; mains?: boolean; }
/** what hums in a room (levelData.ts HumData: id → name) */
export interface HumDef { name: string; kind: 'hum' | 'servers' | 'buzz' | 'wind'; pos: Vec3; level: number; radius: number; mains?: boolean; }
export type PropKind = 'chair' | 'cardboard' | 'bin' | 'plant';
/** One box of a loose prop, in the prop's local frame: `off` from the prop origin (on the floor, y up), `yaw` relative to the prop's heading (frameAt / Box.yaw convention);
 *  `marks` = the markings on the part's faces (scene/markings.ts — the tape and glyphs on a carton), carried onto the box rust/game_rules/src/fittings/props.rs makes of it. */
export interface PropPart { off: Vec3; h: Vec3; yaw: number; albedo: Vec3; marks?: MarkingDef[]; /** what the part is made of, where its paint's name does not say (Box.stuff) */ stuff?: import('../../../src/scene/boxes').BoxStuff; /** what the part does, as Box.does (scene/materials.ts) */ does?: import('../../../src/scene/boxes').DoesWord; }
/** Loose furniture handed to the physics layer (rust/game_rules/src/fittings/props.rs) instead of `boxes`: a rigid group of parts standing on the floor at (x, z) with
 *  heading `yaw`, its collision footprint (half extents of the local XZ rectangle about the origin), overall height and mass (kg). */
export interface PropDef { kind: PropKind; name: string; x: number; z: number; yaw: number; parts: PropPart[]; half: [number, number]; height: number; mass: number; }
export interface Level {
  boxes: Box[];
  /** Chairs, floor-standing cardboard boxes, bins, potted plants. Deliberately NOT in `boxes` (so the static GPU list, the static collision and the nav bake
   *  never see them and nothing is baked twice); the game's PropSystem simulates them and emits their boxes as dynamics every frame. */
  props: PropDef[];
  lights: LightDef[];
  /** Emissive ceiling panels etc. that the OCP / UI can switch: name -> box indices */
  switchables: { name: string; boxes: number[]; group: string; on: boolean; baseEmissive: Vec3; flicker?: boolean; mains?: boolean }[];
  playerSpawn: Vec3;
  routes: PatrolRoute[];
  /** lockdown posts (optional: the viewer's stage has none) */
  chokepoints?: Chokepoint[];
  /** rooms the lockdown pair knows how to clear (rust/game_rules/src/people/guards plan_clears picks the nearest to the last fix; optional like the posts) */
  clearPlans?: RoomClearPlan[];
  doors: LevelDoorDef[];
  /** what its leaves are painted (rust/game_rules/src/fittings/doors/paint.rs) — absent = as a leaf is made, in the office's colours */
  doorPaint?: import('./levelRows').DoorPaintData;
  /** cameras / keypads / laser tripwires (rust/game_rules/src/stealth/devices.rs); optional like the posts — the viewer's stage and older levels have none */
  devices?: DeviceDef[];
  /** HVAC supply vents (rust/game_rules/src/air/wind.rs): the ceiling diffusers whose jets move the particles and the smoke; optional — a level with none has still air indoors */
  vents?: VentDef[];
  /** what hums in its rooms; optional — a level with none is silent till something is done in it */
  hums?: HumDef[];
  /** the cleaner, the late worker (rust/game_rules/src/people/civilians); optional — a level with none has nobody but the shift on it */
  civilians?: CivilianDef[];
  /** authored volumes (LevelData volumes[] — the editor's V tool): the level script's onEnter triggers test them (rust/game_rules/src/missions/director); absent = the level authors none */
  volumes?: VolumeDef[];
  /** named marks (`marks[]`: the end of a `scenarios.walked` way, or a script's place) */
  marks?: MarkDef[];
  /** what the playtest SCENARIOS take from this level (lib/scenarios.ts: the room clear's rooms, the rescue's hostage room and marks, where the team starts) —
   *  the level file's own `scenarios` block. Absent = no scenario can be staged on this level */
  scenarios?: LevelScenarios;
  /** what is underfoot (rust/game_rules/src/world/floor.rs: which footstep, how far it carries) — the level file's own `surfaces` block, attached by loadLevel NON-ENUMERABLY (it is
   *  an annotation of the file, like its ids: nothing that walks a Level as data sees it). Absent = the office's map, as on every level before this field */
  surfaces?: SurfacesData;
  /** `fumble` (s, absent = 2.2): how long a man sent to the tripped panel works at it before the mains come back */
  breaker: { pos: Vec3; boxes: number[]; fumble?: number };
  beacons: { pos: Vec3; box: number }[];
  sky: { zenith: Vec3; horizon: Vec3 };
  /** the breach drill's plan (optional; rust/game_rules/src/missions/breach.rs BreachDrill) */
  breach?: BreachPlan;
  /** the script it is played by (the file's `script`) — absent = none */
  script?: string;
  /** it is built for snow (the file's `snow`) */
  snow?: boolean;
  /** the base texel (m) the level's marking bitmaps were generated for (scene/markings.ts; absent = MARKING_TEXEL 0.025) — host/drawpage.ts hands it to the engine */
  markTexel?: number;
  /** how kept the place is, 0‥1 (the file's meta.kept; absent = 0): host/drawpage.ts hands it to the look (scene/look.ts lookKeptOf) */
  kept?: number;
  /** The one mission thread's geometry (rust/game_rules/src/tick/game Mission): the room to reach (floor rectangle) and the door the HUD points at for it, the rack whose drive
   *  gets pulled (its index n names the switchables `rack${n}_leds` / `rack${n}_glow` that go dark; `front` is the marker point on its LED face), the door
   *  to leave by and the x past which you are outside it. */
  mission: { serverRoom: { x0: number; x1: number; z0: number; z1: number }; entryDoor: string; rack: { index: number; front: Vec3; halfW: number }; exfilDoor: string; exfilX: number; exfilZ: number;
    /** the way OUT through the exit, a unit vector on the floor (dx, dz): past (exfilX, exfilZ) going this way is out. Absent = east, [1, 0] — the office's */
    exfilOut?: [number, number];
    /** what they are called, where the level names them: the room, the thing, what it is in, the way out */
    names?: { place?: string; thing?: string; holder?: string; out?: string } };
}
