// LevelData: the level as DATA — the shape every level file (levels/*.json) is written in, its canonical serializer and digest. What is wrong with a file is the game's own reader's to say
// (rust/game_start file.rs, faults.rs): `levelFaults` (judge.ts) asks it.
//
// What lives here vs elsewhere:
//   level.ts      the RUNTIME shape the game consumes (Level: Box[] with bit flags and array indices, LightDef[], PropDef[], plans…)
//   levelData.ts  (this) the AUTHORED shape: string ids everywhere, flags by name, materials by key, box references by id, nothing derived
//   levelLoad.ts  the two conversions: loadLevel(LevelData) → Level (the game's own reader is rust/game_start level.rs) and levelToData(Level, opts) → LevelData (the exporter's
//                       direction: tools/level-export.ts runs the legacy procedural generator once and writes levels/office.json)
//
// Design rules (the in-browser editor — lane G1b — and later lanes G2 floors / G4 devices build on this, and the maintainer hand-edits the JSON):
//   - every entity carries a stable string `id`, unique within its array; cross references are ALWAYS by id (a light's fixture box, a switchable's boxes,
//     the breaker's two boxes, a beacon's dome, a plan's door, an objective's door), never by array index — the loader resolves ids to the indices the
//     runtime wants. Array ORDER is still preserved by the loader (boxes[i] of the file is world.statics[i]), so a file diff is an entity diff.
//   - no derived data: nothing the loader or the game computes is stored (area lights off emissive fixtures, AreaLit / Emissive flags, door-leaf boxes,
//     nav / reach / collision bakes). Two deliberate exceptions, both flagged where they occur: PropData.half / height MAY be stored to pin a footprint
//     the fit would not reproduce (plants author theirs), and door FRAME boxes are plain boxes today (group 'doorframe.<door>') because the generator
//     emitted them as geometry — the editor may regenerate them from the door when it moves one.
//   - optional fields mean "the default": absent yaw = 0, absent flags = none, absent mains = fed from the breaker, absent floor = the first floor.
//     The exporter writes a field iff the source had it, so levels/office.json round-trips the procedural level key for key.
//   - numbers are stored at full double precision (JSON's shortest round-trip form): rounding even to 4 decimals moves wall slabs whose centres are
//     sums like (x0 + x1) / 2.
//   - versioned: `version` is bumped on any breaking change; the loader refuses versions it does not know.
import type { Vec3 } from '../../../src/math/vec';
import { AUTHORED_FLAGS, type BoxData, type DoorData, type FlagName, type LightData, type ShapeData, type ShapePieceData } from './levelRows';
export { AUTHORED_FLAGS, type BoxData, type DoorData, type FlagName, type LightData, type ShapeData, type ShapePieceData };
import type { WeaponId } from './level';
import type { RoomClearPlan, BreachPlan, CivilianTask } from './level';
import type { LevelScenarios } from './scenarios';
import { BoxFlag, type Box } from '../../../src/scene/boxes';
import type { MarkingDef } from '../../../src/scene/markings';
import { formOf } from '../../../src/scene/shapes';
export type { MarkingDef } from '../../../src/scene/markings';

export const LEVEL_DATA_VERSION = 2 as const;

const FLAG_BITS: Record<FlagName, number> = { NoPrimary: BoxFlag.NoPrimary, NoShadow: BoxFlag.NoShadow, Cutaway: BoxFlag.Cutaway, Gloss: BoxFlag.Gloss, Glass: BoxFlag.Glass };
/** flag names → the Box.flags bit word (unknown names throw) */
export function flagsToBits(names: readonly string[] | undefined): number {
  let bits = 0; for (const n of names ?? []) { const b = FLAG_BITS[n as FlagName]; if (b === undefined) throw new Error(`unknown box flag '${n}'`); bits |= b; } return bits;
}
/** Box.flags bit word → flag names in AUTHORED_FLAGS order (bits outside the authored subset throw — a level box never carries them at build time) */
export function bitsToFlags(bits: number): FlagName[] {
  const out: FlagName[] = []; let rest = bits;
  for (const n of AUTHORED_FLAGS) if (bits & FLAG_BITS[n]) { out.push(n); rest &= ~FLAG_BITS[n]; }
  if (rest) throw new Error(`box flags 0x${bits.toString(16)}: bits 0x${rest.toString(16)} are runtime-only and cannot be authored`);
  return out;
}

// ------------------------------------------------------------------------------------------------------------------------------------------------ schema
export type Rect = { x0: number; x1: number; z0: number; z1: number };

/** One storey. v1 has exactly one (y 0, ceiling 3); lane G2 adds a mezzanine: boxes / doors / props / volumes name the floor they belong to. */
export interface FloorData { id: string; name?: string; /** world Y of the walkable surface */ y: number; /** ceiling height above y (WORLD.ceilingY today) */ ceiling: number; }
/** A named paint: linear diffuse albedo. Boxes and prop parts reference materials by key OR carry an inline albedo (one-offs). `rough` / `metal` = the FINISH the
 *  ray-traced reflection pass reads (boxes.ts Box.rough / metal): GGX roughness 0‥1 (absent = matte, ROUGH_DEFAULT: no ray is
 *  spent) and bare metal (F0 = albedo). A box may override either (BoxData.rough / metal). Old files carry neither and load exactly as before.
 *  `stuff` = WHAT THE PAINT IS MADE OF (scene/stuff.ts: a material of the look, 'plain', 'waits', or the two a round's hit reads, 'glass' and 'rubber'): every box and prop's part that wears the row is that,
 *  unless it says its own (BoxData.stuff). Absent = plain. The look's alone: no rule of the game reads it.
 *  `does` = WHAT THE PAINT IS TO A ROUND, A CHARGE AND A BLAST (scene/materials.ts: a row of the table of materials that has a `does`): every box and
 *  prop's part that wears the row does that, unless it says its own (BoxData.does). Absent = the default row. The rules' alone, never worked out from
 *  `stuff` or from the colour: the loader writes it onto each box, and from then on the box says it. */
export interface MaterialData { albedo: Vec3; rough?: number; metal?: boolean; stuff?: string; does?: string; notes?: string; }
/** What is underfoot for everyone who listens to a footstep (rust/game_rules/src/world/floor.rs: the mixer, the guards' ears and the SOUND meter read ONE map). Zones are floor
 *  rectangles over the interior's default; outside the building it is `outside` except where x or z < grassEdge. TODAY rust/game_rules/src/world/floor.rs still owns its copy of these
 *  numbers (FINISHES / BUILDING / GRASS_EDGE); wiring rust/game_rules/src/world/floor.rs to read this is a follow-up. */
export interface SurfacesData { building: Rect; grassEdge: number; inside: SurfaceMat; outside: SurfaceMat; verge: SurfaceMat; zones: { id: string; mat: SurfaceMat; x0: number; z0: number; x1: number; z1: number; notes?: string }[]; }
export type SurfaceMat = 'carpet' | 'lino' | 'stone' | 'tile' | 'asphalt' | 'grass';

export type PropKindName = 'chair' | 'cardboard' | 'bin' | 'plant';
export interface PropPartData { off: Vec3; h: Vec3; /** relative to the prop's heading; absent = 0 */ yaw?: number; material?: string; albedo?: Vec3; /** as BoxData.stuff */ stuff?: string; /** as BoxData.does */ does?: string; /** markings on the part's faces (as BoxData.markings) */ markings?: MarkingDef[]; }
/** Loose furniture (rust/game_rules/src/fittings/props.rs): a rigid group of parts standing on the floor at (x, z). `half` / `height` are normally FITTED from the parts by the loader
 *  (levelLoad.ts fitPropFootprint — the generator's own rule); store them only to override the fit (a plant's footprint is its foliage's flat sides). */
export interface PropData { id: string; kind: PropKindName; /** PropDef.name; absent = id */ name?: string; floor?: string; x: number; z: number; /** heading; absent = 0 */ yaw?: number; mass: number; parts: PropPartData[]; half?: [number, number]; height?: number; }

/** Emissive fixtures the OCP / panel / blackout can switch: the boxes glow `baseEmissive` when on. */
export interface SwitchableData { id: string; boxes: string[]; group: string; on: boolean; baseEmissive: Vec3; flicker?: boolean; /** absent = true */ mains?: boolean; }
/** One man's beat (level.ts PatrolRoute). `id` doubles as the route name the radio uses unless `name` says otherwise. */
export interface RouteData { id: string; name?: string; points: Vec3[]; wait: number[]; weapon?: WeaponId; keys?: string[]; }
export interface ChokepointData { id: string; name?: string; floor?: string; pos: Vec3; yaw: number; }
/** One of the building's night people (level.ts CivilianDef — rust/game_rules/src/people/civilians): her loop is a route's shape (points walked, wait stood at each) plus what
 *  she does at each waypoint (`tasks`, one per point, CIVILIAN_TASKS) facing which way (`face`, yaws, one per point). `name` = what the men call her (the id if absent). */
export interface CivilianData { id: string; name?: string; kind: 'cleaner' | 'worker'; floor?: string; tint: Vec3; tint2?: Vec3; points: Vec3[]; wait: number[]; tasks?: CivilianTask[]; face?: number[]; notes?: string; }
/** A NAMED MARK (level.ts MarkDef): a place on the floor a level names, where a person stands to use a thing. Read by name: the end of a `walked`
 *  way (rust/game_draw/src/traffic.rs) and a level script's place (rust/game_rules/src/missions/director). No rule of play reads one. `id` is the name; `yaw` = the way a person there faces. */
export interface MarkData { id: string; pos: Vec3; yaw?: number; }
/** Authored volumes (none in the office yet): a hide spot the AI treats as cover-dark, a utility closet, a scripted trigger. Oriented box on a floor. */
export interface VolumeData { id: string; kind: 'hide' | 'utility' | 'trigger'; floor?: string; c: Vec3; h: Vec3; yaw?: number; notes?: string; }

/** The level-design devices (rust/game_rules/src/stealth/devices.rs runs them; docs/level-format.md 'Devices'). Every one is drawn by the game itself from its record (a camera's body,
 *  lens and LED; a keypad's plate and LED; a laser's emitter, receiver and beam) — none of it is in `boxes` — and every one but the keypad is a light TARGET
 *  (rust/game_rules/src/tick/game LightTarget kind 'device'): the pistol's OCP knocks it out for the OCP's spell, a round kills it for the encounter, `mains` (absent = true) puts
 *  it on the breaker so a blackout takes it down and the restrike brings it back; `group` (absent = 'security') is the Lights-panel group.
 *   camera  at `pos` (the lens), looking along `yaw` (Character convention: forward = [sin yaw, 0, cos yaw]) tilted `pitch` (radians, negative = down; absent
 *           −0.35), panning between `sweep[0]`‥`sweep[1]` (absolute yaws) over `sweepPeriod` seconds there and back (absent 9; no sweep = fixed); `fovDeg` = the full cone
 *           (absent 70), `range` metres (absent 14). It sees Sam through the guards' own eye test and light meter (rust/game_rules/src/stealth/devices.rs).
 *   keypad  the plate at `pos` facing `yaw` (out of the wall) beside DoorData `door`, which must be `locked` with `lock` = this keypad's id (the lock IS the
 *           link: rust/game_rules/src/fittings/doors Door.lockId); `code` = four digits. Sam opens it once he knows the code (interrogation / overheard); staff always do.
 *   laser   a beam from `from` (the emitter) to `to` (the receiver) — its height is simply their y (0.35 = shin: no way over; ~1.0 = waist: crouch under);
 *           `alarm` = what breaking it does (absent 'suspicion': the radio and the nearest man over; 'alarm': the floor goes up a step). */
export type DeviceData =
  | { id: string; kind: 'camera'; floor?: string; pos: Vec3; yaw: number; pitch?: number; sweep?: [number, number]; sweepPeriod?: number; fovDeg?: number; range?: number; group?: string; mains?: boolean; notes?: string }
  | { id: string; kind: 'keypad'; floor?: string; pos: Vec3; yaw: number; /** DoorData.id it unlocks */ door: string; code: string; notes?: string }
  | { id: string; kind: 'laser'; floor?: string; from: Vec3; to: Vec3; group?: string; mains?: boolean; alarm?: 'suspicion' | 'alarm'; notes?: string };
/** the defaults an absent optional device field means (rust/game_rules/src/stealth/devices.rs and the editor read them from here) */
export const DEVICE_DEFAULTS = { pitch: -0.35, fovDeg: 70, range: 14, sweepPeriod: 9, group: 'security', alarm: 'suspicion' as 'suspicion' | 'alarm' };
/** An HVAC supply VENT (rust/game_rules/src/air/wind.rs runs the air; docs/level-format.md 'Vents'; level.ts VentDef is the runtime shape): a diffuser at `pos` (its face centre,
 *  just under the ceiling for a ceiling one) blowing along `dir` (need not be unit) at `speed` m/s at the face, a cone of half-angle `spread` DEGREES; the air slows to
 *  half by `reach` m and is gone by 2.5 × reach; a jet aimed down turns along its floor in its last half metre. `size?` = face width (m, absent 0.6), `period?` /
 *  `duty?` / `phase?` = slow cycling (seconds, on-fraction 0‥1, offset s; absent = steady), `mains?` (absent = true: on the breaker — a blackout stops the air),
 *  `floor?` as everywhere. The grille itself is an ordinary box (group 'ceiling.vents') like a door's frame is. */
export interface VentData { id: string; floor?: string; pos: Vec3; dir: Vec3; speed: number; spread: number; reach: number; size?: number; period?: number; duty?: number; phase?: number; mains?: boolean; notes?: string; }
/** What HUMS in a room (rust/game_draw Lasting; docs/level-format.md): a sound that goes on at `pos` at `level` (0‥1), dying to nothing `radius` m off; `mains?` (absent = true): it is on the breaker. */
export interface HumData { id: string; kind: 'hum' | 'servers' | 'buzz' | 'wind'; floor?: string; pos: Vec3; level: number; radius: number; mains?: boolean; notes?: string; }
/** The one mission thread, as objectives the fixed stage machine (rust/game_rules/src/missions/mission.rs) maps onto: v1 wants exactly one of each kind. */
export type ObjectiveData =
  | { id: string; kind: 'reach'; /** the room's floor inside its walls */ area: Rect; /** DoorData.id the HUD points at for it */ door: string }
  | { id: string; kind: 'drive'; /** rack n → switchables rack<n>_leds / rack<n>_glow go dark */ rackIndex: number; /** marker point on its LED face */ front: Vec3; halfW: number }
  | { id: string; kind: 'exfil'; door: string; /** past this x you are out */ x: number; z: number; /** the way out through it, a unit (dx, dz); absent = east: past x */ out?: [number, number] };
export interface MissionData { playerSpawn: Vec3; objectives: ObjectiveData[]; }

export interface SkyData { zenith: Vec3; horizon: Vec3; }

/** The drill plans are the RUNTIME structs verbatim (level.ts RoomClearPlan / BreachPlan — marks, facings, aim lists, lines) plus an id; `door` is a
 *  DoorData.id. Typed BY REFERENCE and written generically below (canonical order = known keys first, anything else after, alphabetically): a plan that grows
 *  grows in level.ts, the generator and the game's reader (rust/game_start records.rs), and re-exports. */
export type RoomClearPlanData = RoomClearPlan & { id: string };
export type BreachPlanData = BreachPlan & { id: string };

export interface LevelData {
  version: typeof LEVEL_DATA_VERSION;
  name: string;
  meta: { /** editor snap (m) */ grid?: number; /** the traceable world this level must fit (scene/world.ts WORLD.min/max at export) */ bounds?: { min: Vec3; max: Vec3 }; generator?: string; notes?: string; /** the base texel (m) the file's marking bitmaps were generated for (scene/markings.ts MARKING_TEXEL when absent) */ markTexel?: number; /** HOW KEPT THE PLACE IS, 0‥1 (absent = 0, every level before the key): 0 = left to the weather and the years, 1 = cleaned and painted last night. One word for the whole level, read by the look's surfaces and its painted marks (render/shaders/look.wgsl lookKept) */ kept?: number };
  floors: FloorData[];
  materials: Record<string, MaterialData>;
  surfaces?: SurfacesData;
  boxes: BoxData[];
  props: PropData[];
  lights: LightData[];
  switchables: SwitchableData[];
  doors: DoorData[];
  routes: RouteData[];
  chokepoints: ChokepointData[];
  /** the cleaner / the late worker (rust/game_rules/src/people/civilians) — absent = nobody but the shift on the floor */
  civilians?: CivilianData[];
  volumes: VolumeData[];
  /** named marks — optional block: absent = the level names none (every file before the key) */
  marks?: MarkData[];
  devices: DeviceData[];
  /** HVAC supply vents (rust/game_rules/src/air/wind.rs) — optional block: absent = still air indoors */
  vents?: VentData[];
  /** what hums in its rooms — optional block: absent = nothing does */
  hums?: HumData[];
  /** a way between floors is in no game yet: the list is empty */
  connectors: never[];
  mission: MissionData;
  /** the mains panel: `panel` / `led` are BoxData ids (OCP the panel for a blackout, shoot it for a permanent one); `fumble?` = seconds a man sent to the
   *  tripped panel works at it before the mains come back (absent = 2.2, as on every level before the key; 7 at most) */
  breaker: { pos: Vec3; panel: string; led: string; fumble?: number };
  /** emergency beacons: `box` is the dome's BoxData id — it MUST also be a switchable's box (rust/game_rules/src/tick/game slaves the beam to that target) */
  beacons: { id: string; pos: Vec3; box: string }[];
  sky: SkyData;
  plans: { roomClear: RoomClearPlanData[]; breach?: BreachPlanData };
  /** what the playtest scenarios take from this level (lib/scenarios.ts — the RUNTIME structs verbatim, like the plans: door names are door ids). Optional:
   *  a level without the block stages none of them */
  scenarios?: LevelScenarios;
}

// ------------------------------------------------------------------------------------------------------------------------------------------------ key registry
/** The keys of every record, in the order they are WRITTEN (canonicalLevelData keeps exactly these; serializeLevelData refuses a level it would lose anything of). Which keys a record may carry is the
 *  game's reader's to say. Adding a field to the format = the reader's line (rust/game_start file.rs, records.rs), its interface above, this list, and both directions in levelLoad.ts. The drill plans
 *  are NOT here: they are handled generically. */
export const KEYS = {
  top: ['version', 'name', 'meta', 'floors', 'materials', 'surfaces', 'boxes', 'props', 'lights', 'switchables', 'doors', 'routes', 'chokepoints', 'civilians', 'volumes', 'marks', 'devices', 'vents', 'hums', 'connectors', 'mission', 'breaker', 'beacons', 'sky', 'plans', 'scenarios'],
  meta: ['grid', 'bounds', 'generator', 'notes', 'markTexel', 'kept'], bounds: ['min', 'max'], rect: ['x0', 'x1', 'z0', 'z1'],
  floor: ['id', 'name', 'y', 'ceiling'], material: ['albedo', 'rough', 'metal', 'stuff', 'does', 'notes'],
  surfaces: ['building', 'grassEdge', 'inside', 'outside', 'verge', 'zones'], zone: ['id', 'mat', 'x0', 'z0', 'x1', 'z1', 'notes'],
  box: ['id', 'floor', 'group', 'name', 'c', 'h', 'yaw', 'rot', 'round', 'material', 'albedo', 'emissive', 'rough', 'metal', 'stuff', 'does', 'flags', 'cutHeight', 'markings', 'shape'],
  prop: ['id', 'kind', 'name', 'floor', 'x', 'z', 'yaw', 'mass', 'parts', 'half', 'height'], part: ['off', 'h', 'yaw', 'material', 'albedo', 'stuff', 'does', 'markings'],
  marking: ['face', 'res', 'at', 'size', 'pal', 'cells'], markpal: ['albedo', 'mul', 'rough', 'emit'],   // (scene/markings.ts marking() / palEntry() write exactly these, in this order)
  light: ['id', 'kind', 'pos', 'dir', 'color', 'intensity', 'range', 'innerDeg', 'outerDeg', 'group', 'enabled', 'volumetric', 'radius', 'mains', 'broad', 'fixture'],
  switchable: ['id', 'boxes', 'group', 'on', 'baseEmissive', 'flicker', 'mains'],
  door: ['id', 'floor', 'hinge', 'closedDir', 'width', 'angle', 'exterior', 'closer', 'minAngle', 'maxAngle', 'locked', 'lock', 'keySide', 'steel', 'swing'],
  route: ['id', 'name', 'points', 'wait', 'weapon', 'keys'], chokepoint: ['id', 'name', 'floor', 'pos', 'yaw'], civilian: ['id', 'name', 'kind', 'floor', 'tint', 'tint2', 'points', 'wait', 'tasks', 'face', 'notes'], volume: ['id', 'kind', 'floor', 'c', 'h', 'yaw', 'notes'], mark: ['id', 'pos', 'yaw'],
  camera: ['id', 'kind', 'floor', 'pos', 'yaw', 'pitch', 'sweep', 'sweepPeriod', 'fovDeg', 'range', 'group', 'mains', 'notes'], keypad: ['id', 'kind', 'floor', 'pos', 'yaw', 'door', 'code', 'notes'], laser: ['id', 'kind', 'floor', 'from', 'to', 'group', 'mains', 'alarm', 'notes'],
  vent: ['id', 'floor', 'pos', 'dir', 'speed', 'spread', 'reach', 'size', 'period', 'duty', 'phase', 'mains', 'notes'], hum: ['id', 'kind', 'floor', 'pos', 'level', 'radius', 'mains', 'notes'],
  mission: ['playerSpawn', 'objectives'], reach: ['id', 'kind', 'area', 'door'], drive: ['id', 'kind', 'rackIndex', 'front', 'halfW'], exfil: ['id', 'kind', 'door', 'x', 'z', 'out'],
  breaker: ['pos', 'panel', 'led', 'fumble'], beacon: ['id', 'pos', 'box'], sky: ['zenith', 'horizon'],
  plans: ['roomClear', 'breach'],   // (the container only — what is INSIDE a plan is generic, see canonBlob)
  scenarios: ['nightBreach', 'roomClear', 'rescue', 'places', 'quiet', 'lesson', 'par', 'issue', 'kitView', 'response', 'doorNames', 'walked'],   // (the container only, likewise: what is inside is lib/scenarios.ts's, carried as written)
} as const;
/** the keys of a shape's piece (BoxData.shape.with) beside its form, in the order they are written (levelLoad.ts loadLevel and levelToData, canonShape below) */
export const PIECE_KEYS: readonly string[] = ['off', 'h', 'yaw', 'material', 'albedo', 'rough', 'metal', 'stuff', 'does'];

// ------------------------------------------------------------------------------------------------------------------------------------------------ floor references
/** every record that says it is on floor `floorId` (one that says none is on the first) */
export const floorUsers = (d: LevelData, floorId: string): { id: string }[] => [d.boxes, d.props, d.doors, d.volumes, d.chokepoints, d.devices, d.civilians ?? [], d.vents ?? [], d.hums ?? []].flatMap(l => (l as { id: string; floor?: string }[]).filter(e => e.floor === floorId));

/** a level file's text (its faults are the judge's to say: judge.ts) */
export const parseLevelData = (text: string): LevelData => JSON.parse(text) as LevelData;

// ------------------------------------------------------------------------------------------------------------------------------------------------ canonical form
// Every record rebuilt with its keys in schema order (KEYS) and undefined-valued keys dropped: what the serializer prints and the digest hashes, so two
// LevelData values that MEAN the same level serialize to the same bytes whatever order an editor assembled them in. The result is a DEEP COPY (every array
// and record new): safe to use as a working buffer without touching the input.
/** deep copy of plain JSON data (arrays / records / leaves), undefined-valued keys dropped, key order kept */
function copyPlain<T>(v: T): T { if (Array.isArray(v)) return v.map(copyPlain) as unknown as T; if (v && typeof v === 'object') { const o: any = {}; for (const k of Object.keys(v)) { const x = (v as any)[k]; if (x !== undefined) o[k] = copyPlain(x); } return o; } return v; }
/** the record with only `keys`, in that order, values deep-copied */
function pick<T extends object>(o: T, keys: readonly string[]): T { const out: any = {}; for (const k of keys) { const v = (o as any)[k]; if (v !== undefined) out[k] = copyPlain(v); } return out; }
const canonLight = (l: LightData): LightData => pick(l, KEYS.light);
/** a shape re-keyed as levelLoad.ts levelToData writes it: the form, then the pieces, each its own keys and then its form */
const canonShape = (x: ShapeData): ShapeData => ({ ...formOf(x), ...(x.with ? { with: x.with.map(w => ({ ...pick(w, PIECE_KEYS), ...formOf(w) })) } : {}) });
const canonMarking = (m: MarkingDef): MarkingDef => ({ ...pick(m, KEYS.marking), pal: (m.pal ?? []).map(p => pick(p, KEYS.markpal)) });
/** Preferred key order inside the drill plans, at any depth: the fields RoomClearPlan / BreachPlan have had, in reading order. canonBlob puts keys it knows
 *  in this order and anything newer AFTER them alphabetically — so the plans stay readable, serialize identically whatever order they were assembled in, and a
 *  field added to BreachPlan by a later lane is carried, not dropped (this list is a preference, never a filter). */
const PLAN_KEY_ORDER = ['id', 'door', 'room', 'breach', 'bounds', 'x0', 'x1', 'z0', 'z1', 'from', 'stack', 'stackFace', 'stackSide', 'stackAim', 'kickFrom', 'kickFace', 'fling',
  'aside', 'retreat', 'toss', 'mark', 'face', 'land', 'hostiles', 'pos', 'yaw', 'crouch', 'aimAt', 'entry', 'podFace', 'entryAim', 'flow', 'into', 'centre', 'corner', 'cornerAim', 'hold', 'targets', 'next',
  'searchPath', 'searchFace', 'searchAimWalk', 'searchAimThere', 'underAim', 'upAim', 'formUp', 'formFace',
  'lines', 'stack1', 'stack2', 'go2', 'in1', 'side2', 'side1', 'push1', 'under2', 'nothing2', 'corner1', 'empty2', 'breachLines', 'breach1', 'bang2', 'going1', 'clear1', 'clear2', 'roomClear1', 'next1'];   // ('radio1' closes both `lines` and `breachLines`: left unranked it sorts after every ranked key in either record, which is where it reads)
const PLAN_RANK = new Map(PLAN_KEY_ORDER.map((k, i) => [k, i]));
/** generic canonical form for a JSON blob: records re-keyed (known keys in PLAN_KEY_ORDER order, the rest alphabetical), undefined dropped, arrays in order */
function canonBlob<T>(v: T): T {
  if (Array.isArray(v)) return v.map(canonBlob) as unknown as T;
  if (v && typeof v === 'object') {
    const keys = Object.keys(v).filter(k => (v as any)[k] !== undefined).sort((a, b) => { const ra = PLAN_RANK.get(a), rb = PLAN_RANK.get(b); return ra !== undefined && rb !== undefined ? ra - rb : ra !== undefined ? -1 : rb !== undefined ? 1 : a < b ? -1 : a > b ? 1 : 0; });
    const o: any = {}; for (const k of keys) o[k] = canonBlob((v as any)[k]); return o;
  }
  return v;
}
const canonPlan = <T extends RoomClearPlanData>(p: T): T => canonBlob(p);
export function canonicalLevelData(d: LevelData): LevelData {
  const mats: Record<string, MaterialData> = {}; for (const k of Object.keys(d.materials)) mats[k] = pick(d.materials[k], KEYS.material);   // (insertion order of materials is kept: it is the palette's authored order)
  const rect = (r: Rect): Rect => pick(r, KEYS.rect);
  return {
    version: d.version, name: d.name,
    meta: { ...pick(d.meta, KEYS.meta), ...(d.meta.bounds ? { bounds: pick(d.meta.bounds, KEYS.bounds) } : {}) },
    floors: d.floors.map(f => pick(f, KEYS.floor)),
    materials: mats,
    ...(d.surfaces ? { surfaces: { ...pick(d.surfaces, KEYS.surfaces), building: rect(d.surfaces.building), zones: d.surfaces.zones.map(z => pick(z, KEYS.zone)) } as SurfacesData } : {}),
    boxes: d.boxes.map(b => ({ ...pick(b, KEYS.box), ...(b.markings ? { markings: b.markings.map(canonMarking) } : {}), ...(b.shape ? { shape: canonShape(b.shape) } : {}) })),
    props: d.props.map(p => ({ ...pick(p, KEYS.prop), parts: p.parts.map(q => ({ ...pick(q, KEYS.part), ...(q.markings ? { markings: q.markings.map(canonMarking) } : {}) })) })),
    lights: d.lights.map(canonLight),
    switchables: d.switchables.map(s => pick(s, KEYS.switchable)),
    doors: d.doors.map(o => pick(o, KEYS.door)),
    routes: d.routes.map(r => pick(r, KEYS.route)),
    chokepoints: d.chokepoints.map(c => pick(c, KEYS.chokepoint)),
    ...(d.civilians ? { civilians: d.civilians.map(c => pick(c, KEYS.civilian)) } : {}),
    volumes: d.volumes.map(v => pick(v, KEYS.volume)),
    ...(d.marks?.length ? { marks: d.marks.map(m => pick(m, KEYS.mark)) } : {}),
    devices: d.devices.map(v => v.kind === 'camera' ? pick(v, KEYS.camera) : v.kind === 'keypad' ? pick(v, KEYS.keypad) : pick(v, KEYS.laser)),
    ...(d.vents ? { vents: d.vents.map(v => pick(v, KEYS.vent)) } : {}),
    ...(d.hums ? { hums: d.hums.map(v => pick(v, KEYS.hum)) } : {}),
    connectors: [],
    mission: { ...pick(d.mission, KEYS.mission), objectives: d.mission.objectives.map(o => o.kind === 'reach' ? { ...pick(o, KEYS.reach), area: rect(o.area) } : o.kind === 'drive' ? pick(o, KEYS.drive) : pick(o, KEYS.exfil)) },
    breaker: pick(d.breaker, KEYS.breaker),
    beacons: d.beacons.map(b => pick(b, KEYS.beacon)),
    sky: pick(d.sky, KEYS.sky),
    plans: { roomClear: d.plans.roomClear.map(canonPlan), ...(d.plans.breach ? { breach: canonPlan(d.plans.breach) } : {}) },
    ...(d.scenarios ? { scenarios: { ...pick(d.scenarios, KEYS.scenarios) } } : {}),   // (the three in KEYS order; inside each, the keys as written — lib/scenarios.ts is their shape)
  };
}

// ------------------------------------------------------------------------------------------------------------------------------------------------ serialize / digest
/** Compact one-line JSON with a space after ':' and ',' (numbers in their shortest round-trip form — what JSON.stringify prints). */
function compact(v: unknown): string {
  if (Array.isArray(v)) { const parts: string[] = []; for (let i = 0; i < v.length; i++) { const x = v[i]; if (x === undefined || typeof x === 'function' || (typeof x === 'number' && !Number.isFinite(x))) throw new Error(`serializeLevelData: unwritable array element [${i}] (${String(x)}) — validate before saving`); parts.push(compact(x)); } return `[${parts.join(', ')}]`; }   // (by index: a hole reads as undefined and is refused too)
  if (v && typeof v === 'object') return `{ ${Object.entries(v).filter(([, x]) => x !== undefined).map(([k, x]) => `${JSON.stringify(k)}: ${compact(x)}`).join(', ')} }`;
  if (typeof v === 'number' && !Number.isFinite(v)) throw new Error(`serializeLevelData: non-finite number ${v}`);
  return JSON.stringify(v);
}
/** The level file's text: canonical key order, top-level keys one per line, every entity (box, prop, light, route, plan…) on ONE line — so `git grep`
 *  finds a box by id or coordinate, a diff is a list of changed entities, and a hand edit touches one line. Parses with plain JSON.parse. */
export function serializeLevelData(data: LevelData): string {
  const canonical = canonicalLevelData(data), lost: string[] = [];
  if (structuralDiff(data, canonical, lost, 8)) throw new Error(`serializeLevelData: the writing would lose part of '${data.name}' (a member KEYS does not name?)\n  ${lost.join('\n  ')}`);
  const d = canonical as unknown as Record<string, unknown>;
  const lines: string[] = ['{'];
  const keys = Object.keys(d);
  keys.forEach((k, ki) => {
    const v = d[k]; const last = ki === keys.length - 1 ? '' : ',';
    if (Array.isArray(v)) {
      if (!v.length) { lines.push(`  ${JSON.stringify(k)}: []${last}`); return; }
      lines.push(`  ${JSON.stringify(k)}: [`); v.forEach((e, i) => lines.push(`    ${compact(e)}${i === v.length - 1 ? '' : ','}`)); lines.push(`  ]${last}`);
    } else if (v && typeof v === 'object' && (k === 'materials' || k === 'plans' || k === 'surfaces' || k === 'mission')) {
      // records of records: one member per line (materials by name; plans.roomClear one plan per line; surfaces / mission field per line)
      const o = v as Record<string, unknown>; const oks = Object.keys(o).filter(x => o[x] !== undefined);
      lines.push(`  ${JSON.stringify(k)}: {`);
      oks.forEach((x, i) => {
        const e = o[x]; const l2 = i === oks.length - 1 ? '' : ',';
        if (Array.isArray(e) && e.length && e.every(y => y && typeof y === 'object' && !Array.isArray(y))) { lines.push(`    ${JSON.stringify(x)}: [`); e.forEach((y, j) => lines.push(`      ${compact(y)}${j === e.length - 1 ? '' : ','}`)); lines.push(`    ]${l2}`); }
        else lines.push(`    ${JSON.stringify(x)}: ${compact(e)}${l2}`);
      });
      lines.push(`  }${last}`);
    } else if (v && typeof v === 'object' && k === 'scenarios') {
      // one scenario per block, one field per line, a list of records (rooms, hostages, roamers) one record per line
      const o = v as Record<string, Record<string, unknown>>; const oks = Object.keys(o);
      lines.push(`  ${JSON.stringify(k)}: {`);
      oks.forEach((x, i) => {
        const sc = o[x], fs = Object.keys(sc).filter(f => sc[f] !== undefined);
        // (a block that is itself a list — doorNames, a row a door: one record per line; written as an object's fields it came back keyed '0', '1' and was refused)
        if (Array.isArray(sc)) { const rows = sc as unknown[], l2 = i === oks.length - 1 ? '' : ',';
          if (!rows.length) lines.push(`    ${JSON.stringify(x)}: []${l2}`);
          else { lines.push(`    ${JSON.stringify(x)}: [`); rows.forEach((y, n) => lines.push(`      ${compact(y)}${n === rows.length - 1 ? '' : ','}`)); lines.push(`    ]${l2}`); }
          return; }
        lines.push(`    ${JSON.stringify(x)}: {`);
        fs.forEach((f, j) => {
          const e = sc[f], l3 = j === fs.length - 1 ? '' : ',';
          if (Array.isArray(e) && e.length && e.every(y => y && typeof y === 'object' && !Array.isArray(y))) { lines.push(`      ${JSON.stringify(f)}: [`); e.forEach((y, n) => lines.push(`        ${compact(y)}${n === e.length - 1 ? '' : ','}`)); lines.push(`      ]${l3}`); }
          else lines.push(`      ${JSON.stringify(f)}: ${compact(e)}${l3}`);
        });
        lines.push(`    }${i === oks.length - 1 ? '' : ','}`);
      });
      lines.push(`  }${last}`);
    } else lines.push(`  ${JSON.stringify(k)}: ${compact(v)}${last}`);
  });
  lines.push('}');
  return lines.join('\n') + '\n';
}

/** A stable 52-bit content hash (hex) of the CANONICAL compact JSON — identity checks between an exported file, a loaded level converted back, an editor's
 *  buffer. Not cryptographic. (−0 prints as 0 and undefined keys vanish, exactly as they do in the file.) */
export function levelDataDigest(d: LevelData): string {
  const s = JSON.stringify(canonicalLevelData(d));
  let h1 = 0xdeadbeef ^ s.length, h2 = 0x41c6ce57 ^ s.length;
  for (let i = 0; i < s.length; i++) { const c = s.charCodeAt(i); h1 = Math.imul(h1 ^ c, 2654435761); h2 = Math.imul(h2 ^ c, 1597334677); }
  h1 = Math.imul(h1 ^ (h1 >>> 16), 2246822507) ^ Math.imul(h2 ^ (h2 >>> 13), 3266489909);
  h2 = Math.imul(h2 ^ (h2 >>> 16), 2246822507) ^ Math.imul(h1 ^ (h1 >>> 13), 3266489909);
  return ((h2 >>> 0) & 0xfffff).toString(16).padStart(5, '0') + (h1 >>> 0).toString(16).padStart(8, '0');
}

/** Every structural difference between two JSON-ish values (Levels, LevelData, anything): missing / extra keys (an undefined-valued key counts as absent),
 *  array length, type, and leaf inequality by === (so −0 equals 0, as it does once written to a file). Paths like `boxes[412].c[1]`. Stops collecting
 *  after `limit` entries but keeps COUNTING: returns the total. The identity gate between the procedural level and the loaded one is `0`. */
export function structuralDiff(a: unknown, b: unknown, out: string[] = [], limit = 50, path = '$'): number {
  let n = 0;
  const note = (msg: string) => { n++; if (out.length < limit) out.push(`${path}: ${msg}`); };
  if (Array.isArray(a) || Array.isArray(b)) {
    if (!Array.isArray(a) || !Array.isArray(b)) { note(`array vs ${Array.isArray(a) ? typeof b : typeof a}`); return n; }
    if (a.length !== b.length) note(`length ${a.length} vs ${b.length}`);
    for (let i = 0; i < Math.min(a.length, b.length); i++) n += structuralDiff(a[i], b[i], out, limit, `${path}[${i}]`);
    return n;
  }
  if (a && b && typeof a === 'object' && typeof b === 'object') {
    const ka = Object.keys(a).filter(k => (a as any)[k] !== undefined), kb = new Set(Object.keys(b).filter(k => (b as any)[k] !== undefined));
    for (const k of ka) { if (!kb.has(k)) { note(`key '${k}' only on the left (${JSON.stringify((a as any)[k])?.slice(0, 60)})`); continue; } n += structuralDiff((a as any)[k], (b as any)[k], out, limit, `${path}.${k}`); kb.delete(k); }
    for (const k of kb) note(`key '${k}' only on the right (${JSON.stringify((b as any)[k])?.slice(0, 60)})`);
    return n;
  }
  if (a !== b) note(`${JSON.stringify(a)} vs ${JSON.stringify(b)}`);
  return n;
}
