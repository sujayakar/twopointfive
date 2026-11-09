// The two conversions between the level FILE (levelData.ts LevelData: ids, flag names, material keys) and the level the game RUNS (level.ts Level: Box[]
// with bit flags, array indices, LightDef[] …):
//   loadLevel(data)          → Level   the whole file, every box in file order
//   levelToData(level, opts) → LevelData   the exporter's direction (tools/level-export.ts turns the legacy procedural generator's output into the file)
// Both preserve ARRAY ORDER exactly — data.boxes[i] ⇄ level.boxes[i] ⇄ world.statics[i] — and resolve every cross reference (a light's fixture, a
// switchable's boxes, the breaker's pair, a beacon's dome) id ⇄ index. Nothing here is procedural: the one computed thing is a prop's collision footprint
// fitted from its parts (fitPropFootprint, the generator's own rule), and a file may pin that too (PropData.half / height).
import type { Vec3 } from '../../../src/math/vec';
import { Box, BoxFlag, makeBox, ROUGH_DEFAULT, type BoxStuff, type DoesWord } from '../../../src/scene/boxes';
import { DOES_DEFAULT, doesSaid } from '../../../src/scene/materials';
import type { Level, LightDef, PropDef, PropPart, LevelDoorDef, PatrolRoute, Chokepoint, RoomClearPlan, BreachPlan, DeviceDef, CivilianDef, VentDef, VolumeDef, MarkDef } from './level';
import { LevelData, LEVEL_DATA_VERSION, BoxData, ShapeData, ShapePieceData, DoorData, LightData, PropData, MaterialData, FloorData, SurfacesData, RoomClearPlanData, BreachPlanData, ObjectiveData, DeviceData, CivilianData, VentData, MarkData, flagsToBits, bitsToFlags } from './levelData';
import { copyMarkings } from '../../../src/scene/markings';
import { formOf, type BoxShape, type ShapePiece } from '../../../src/scene/shapes';

const V = (v: readonly number[]): Vec3 => [v[0], v[1], v[2]];
const VV = (a: readonly (readonly number[])[]): Vec3[] => a.map(V);
/** structured deep copy for the plan blobs (plain JSON data: arrays, objects, numbers, strings) */
function clone<T>(v: T): T { if (Array.isArray(v)) return v.map(clone) as unknown as T; if (v && typeof v === 'object') { const o: any = {}; for (const k of Object.keys(v)) { const x = (v as any)[k]; if (x !== undefined) o[k] = clone(x); } return o; } return v; }

/** The collision footprint the physics layer wants for a loose prop, fitted from its parts exactly as the generator's fitProp measured it: the bounding
 *  rectangle of the (yawed) parts in the prop frame → half extents (≥ 5 cm), and the tallest part top → height (≥ 10 cm). Parts are expected centred on
 *  the prop origin (the generator re-centred them; the editor's lint checks it). `mid` = that rectangle's centre in the prop frame, which the generators'
 *  fitProp re-centres the origin on (officeGen.ts, levelKit.ts). */
export function fitPropFootprint(parts: readonly { off: Vec3; h: Vec3; yaw?: number }[]): { half: [number, number]; height: number; mid: [number, number] } {
  let x0 = Infinity, x1 = -Infinity, z0 = Infinity, z1 = -Infinity, height = 0.1;
  for (const q of parts) {
    const yaw = q.yaw ?? 0; const cs = Math.abs(Math.cos(yaw)), sn = Math.abs(Math.sin(yaw)); const ex = cs * q.h[0] + sn * q.h[2], ez = sn * q.h[0] + cs * q.h[2];
    x0 = Math.min(x0, q.off[0] - ex); x1 = Math.max(x1, q.off[0] + ex); z0 = Math.min(z0, q.off[2] - ez); z1 = Math.max(z1, q.off[2] + ez); height = Math.max(height, q.off[1] + q.h[1]);
  }
  return { half: [Math.max(0.05, (x1 - x0) / 2), Math.max(0.05, (z1 - z0) / 2)], height, mid: [(x0 + x1) / 2, (z0 + z1) / 2] };
}

// ------------------------------------------------------------------------------------------------------------------------------------------------ LevelData → Level
/** Build the runtime Level from level data, which is a level (judge.ts says where it is none). Every vector is copied: the caller may mutate its Level and two loads never share state. */
export function loadLevel(data: LevelData): Level {
  const mats = new Map(Object.entries(data.materials));   // (own keys only: 'constructor' & co. are no material's names)
  const albedoOf = (o: { material?: string; albedo?: Vec3 }): Vec3 => {
    if (o.material === undefined) return V(o.albedo!);
    const m = mats.get(o.material); if (!m) throw new Error(`level '${data.name}': unknown material '${o.material}'`); return V(m.albedo);
  };
  /** the finish of a box, or of a shape's piece (ONE reader for both): its own, else its material's */
  const finishOf = (o: { material?: string; rough?: number; metal?: boolean }): { rough?: number; metal?: true } => {
    const m = o.material !== undefined ? mats.get(o.material) : undefined, rough = o.rough ?? m?.rough, metal = o.metal ?? m?.metal;
    return { ...(rough !== undefined ? { rough } : {}), ...(metal ? { metal: true as const } : {}) };
  };
  /** what a box or a part is made of: its own word, else its material row's; nothing for plain (a box that says nothing is plain) */
  const stuffOf = (o: { stuff?: string }, m: MaterialData | undefined): BoxStuff | undefined => { const w = o.stuff ?? m?.stuff; return w === undefined || w === 'plain' ? undefined : w as BoxStuff; };
  /** the same for a box, or a shape's piece, that names its row itself (ONE reader for both, as finishOf is) */
  /** … and what it DOES (scene/materials.ts): its own word, else its material row's, written onto the box here, once; nothing for the default row. Never from `stuff`, never from the colour */
  const doesOf = (o: { does?: string }, m: MaterialData | undefined): { does?: DoesWord } => doesSaid(o.does ?? m?.does);
  const saidOf = (o: { stuff?: string; does?: string; material?: string }): { stuff?: BoxStuff; does?: DoesWord } => { const m = o.material !== undefined ? mats.get(o.material) : undefined, w = stuffOf(o, m); return { ...(w !== undefined ? { stuff: w } : {}), ...doesOf(o, m) }; };
  // boxes, in file order (index i of the file IS static box i)
  const boxIndex = new Map<string, number>();
  const boxes: Box[] = data.boxes.map((b, i) => {
    boxIndex.set(b.id, i);
    const p: Partial<Box> & { c: Vec3; h: Vec3 } = { c: V(b.c), h: V(b.h), yaw: b.yaw ?? 0, albedo: albedoOf(b), emissive: b.emissive ? V(b.emissive) : [0, 0, 0], flags: flagsToBits(b.flags) };
    if (b.rot) p.rot = [b.rot[0], b.rot[1], b.rot[2], b.rot[3]];
    if (b.round) p.round = true;
    if (b.name !== undefined) p.name = b.name;
    if (b.cutHeight !== undefined) p.cutHeight = b.cutHeight;
    const marks = copyMarkings(b.markings); if (marks) p.marks = marks;   // (through markings.ts marking(): the same structure the generator builds)
    // the finish (Box.rough / metal): the box's own, else its material's; absent both = matte dielectric — nothing written, so a file without finishes loads to
    // the very Level it always did (`metal` is only ever carried as true: false is the default and stays unwritten, as levelToData writes it)
    Object.assign(p, finishOf(b), saidOf(b));   // (what it is made of — scene/stuff.ts: the box's own word, else its material's; 'plain' is no word, and a word no box can be is a fault of the file)
    if (b.shape) {   // A SHAPE (scene/shapes.ts): carried on the box as authored — its planes copied, a piece's paint, finish and word resolved as a box's are; given to the box when the level is stood up
      const s = b.shape, sh: BoxShape = { ...formOf(s) };
      if (s.with) sh.with = s.with.map((w): ShapePiece => ({ off: V(w.off), h: V(w.h), ...(w.yaw ? { yaw: w.yaw } : {}), ...(w.material !== undefined || w.albedo !== undefined ? { albedo: albedoOf(w), ...finishOf(w), ...saidOf(w) } : {}), ...formOf(w) }));   // (no paint: the piece wears its box's paint, finish and word — shapes.ts shapeLevel)
      p.shape = sh;
    }
    return makeBox(p);
  });
  const bx = (id: string) => { const i = boxIndex.get(id); if (i === undefined) throw new Error(`level '${data.name}': unknown box '${id}'`); return i; };
  // loose props
  const props: PropDef[] = data.props.map(p => {
    const parts: PropPart[] = p.parts.map(q => { const part: PropPart = { off: V(q.off), h: V(q.h), yaw: q.yaw ?? 0, albedo: albedoOf(q) }; const marks = copyMarkings(q.markings); if (marks) part.marks = marks; const said = saidOf(q); if (said.stuff !== undefined) part.stuff = said.stuff; if (said.does !== undefined) part.does = said.does; return part; });
    const fit = fitPropFootprint(parts);
    return { kind: p.kind, name: p.name ?? p.id, x: p.x, z: p.z, yaw: p.yaw ?? 0, parts, half: p.half ? [p.half[0], p.half[1]] : fit.half, height: p.height ?? fit.height, mass: p.mass };
  });
  // lights
  const lightOf = (l: LightData): LightDef => {
    const d: LightDef = { kind: l.kind, pos: V(l.pos), dir: V(l.dir), color: V(l.color), intensity: l.intensity, range: l.range, innerDeg: l.innerDeg, outerDeg: l.outerDeg, name: l.id, group: l.group, enabled: l.enabled, volumetric: l.volumetric };
    if (l.fixture !== undefined) d.fixtureBox = bx(l.fixture);
    if (l.radius !== undefined) d.radius = l.radius;
    if (l.mains !== undefined) d.mains = l.mains;
    if (l.broad !== undefined) d.broad = l.broad;
    return d;
  };
  const lights = data.lights.map(lightOf);
  const switchables: Level['switchables'] = data.switchables.map(s => {
    const o: Level['switchables'][number] = { name: s.id, boxes: s.boxes.map(bx), group: s.group, on: s.on, baseEmissive: V(s.baseEmissive) };
    if (s.flicker !== undefined) o.flicker = s.flicker; if (s.mains !== undefined) o.mains = s.mains;
    return o;
  });
  const doors: LevelDoorDef[] = data.doors.map(o => {
    const d: LevelDoorDef = { name: o.id, hinge: [o.hinge[0], o.hinge[1]], closedDir: o.closedDir, width: o.width };
    if (o.angle !== undefined) d.angle = o.angle; if (o.exterior !== undefined) d.exterior = o.exterior; if (o.closer !== undefined) d.closer = o.closer;
    if (o.minAngle !== undefined) d.minAngle = o.minAngle; if (o.maxAngle !== undefined) d.maxAngle = o.maxAngle;
    if (o.locked !== undefined) d.locked = o.locked; if (o.lock !== undefined) d.lock = o.lock; if (o.keySide !== undefined) d.keySide = o.keySide; if (o.steel !== undefined) d.steel = o.steel;
    if (o.swing !== undefined) d.swing = o.swing;
    return d;
  });
  const routes: PatrolRoute[] = data.routes.map(r => { const o: PatrolRoute = { name: r.name ?? r.id, points: VV(r.points), wait: [...r.wait] }; if (r.weapon !== undefined) o.weapon = r.weapon; if (r.keys !== undefined) o.keys = [...r.keys]; return o; });
  const chokepoints: Chokepoint[] = data.chokepoints.map(c => ({ name: c.name ?? c.id, pos: V(c.pos), yaw: c.yaw }));
  const civilians: CivilianDef[] | undefined = data.civilians?.map(c => { const o: CivilianDef = { name: c.name ?? c.id, kind: c.kind, tint: V(c.tint), points: VV(c.points), wait: [...c.wait] }; if (c.tint2 !== undefined) o.tint2 = V(c.tint2); if (c.tasks !== undefined) o.tasks = [...c.tasks]; if (c.face !== undefined) o.face = [...c.face]; return o; });
  const devices: DeviceDef[] = data.devices.map(deviceOf);
  const vents: VentDef[] | undefined = data.vents?.map(v => { const o: VentDef = { name: v.id, pos: V(v.pos), dir: V(v.dir), speed: v.speed, spread: v.spread, reach: v.reach }; for (const k of ['size', 'period', 'duty', 'phase', 'mains'] as const) if (v[k] !== undefined) (o as any)[k] = v[k]; return o; });   // (id → name, floor / notes dropped like a door's; optional fields written iff present)
  const planOf = <T extends RoomClearPlanData>(p: T): Omit<T, 'id'> => { const q = clone(p) as any; delete q.id; return q; };
  const clearPlans: RoomClearPlan[] = data.plans.roomClear.map(p => planOf(p) as RoomClearPlan);
  // mission: the fixed thread's three objectives back onto Level.mission's shape (a level has one of each)
  const ob = <K extends ObjectiveData['kind']>(k: K) => data.mission.objectives.find(o => o.kind === k) as Extract<ObjectiveData, { kind: K }>;
  const reach = ob('reach'), drive = ob('drive'), exfil = ob('exfil');
  const level: Level = {
    boxes, props, lights, switchables,
    ...(data.plans.breach ? { breach: planOf(data.plans.breach) as BreachPlan } : {}),
    doors,
    devices,
    ...(vents ? { vents } : {}),
    breaker: { pos: V(data.breaker.pos), boxes: [bx(data.breaker.panel), bx(data.breaker.led)], ...(data.breaker.fumble !== undefined ? { fumble: data.breaker.fumble } : {}) },
    beacons: data.beacons.map(b => ({ pos: V(b.pos), box: bx(b.box) })),
    playerSpawn: V(data.mission.playerSpawn),
    routes, chokepoints, clearPlans,
    ...(civilians ? { civilians } : {}),
    ...(data.volumes.length ? { volumes: data.volumes.map((v): VolumeDef => ({ name: v.id, kind: v.kind, c: V(v.c), h: V(v.h), ...(v.yaw !== undefined ? { yaw: v.yaw } : {}) })) } : {}),   // the level script's trigger volumes (rust/game_rules/src/director onEnter) — written iff authored, so a level with none loads key for key as before
    ...(data.marks?.length ? { marks: data.marks.map((m): MarkDef => ({ name: m.id, pos: V(m.pos), ...(m.yaw !== undefined ? { yaw: m.yaw } : {}) })) } : {}),   // the level's named marks (rust/game_types/src/scene_ways, rust/game_rules/src/director) — written iff the file names any, so a level with none loads key for key as before
    sky: { zenith: V(data.sky.zenith), horizon: V(data.sky.horizon) },
    mission: { serverRoom: { x0: reach.area.x0, x1: reach.area.x1, z0: reach.area.z0, z1: reach.area.z1 }, entryDoor: reach.door, rack: { index: drive.rackIndex, front: V(drive.front), halfW: drive.halfW }, exfilDoor: exfil.door, exfilX: exfil.x, exfilZ: exfil.z, ...(exfil.out ? { exfilOut: [exfil.out[0], exfil.out[1]] as [number, number] } : {}) },
    ...(data.meta.markTexel !== undefined ? { markTexel: data.meta.markTexel } : {}),
    ...(data.meta.kept !== undefined ? { kept: data.meta.kept } : {}),
    ...(data.scenarios ? { scenarios: clone(data.scenarios) } : {}),   // (the runtime structs verbatim: lib/scenarios.ts)
  };
  // data.surfaces → Level.surfaces, what rust/game_rules/src/floor reads for this level's footsteps: non-enumerable, so a Level compared or written as data is what it was
  // (a file with none: rust/game_rules/src/floor's own office map).
  if (data.surfaces) Object.defineProperty(level, 'surfaces', { value: clone(data.surfaces), enumerable: false, writable: true, configurable: true });
  return level;
}
/** one device record → its runtime def (id → name, floor / notes dropped like a door's, every vector copied, optional fields written iff present) */
function deviceOf(v: DeviceData): DeviceDef {
  if (v.kind === 'camera') {
    const d: DeviceDef = { kind: 'camera', name: v.id, pos: V(v.pos), yaw: v.yaw };
    if (v.pitch !== undefined) d.pitch = v.pitch; if (v.sweep !== undefined) d.sweep = [v.sweep[0], v.sweep[1]]; if (v.sweepPeriod !== undefined) d.sweepPeriod = v.sweepPeriod;
    if (v.fovDeg !== undefined) d.fovDeg = v.fovDeg; if (v.range !== undefined) d.range = v.range; if (v.group !== undefined) d.group = v.group; if (v.mains !== undefined) d.mains = v.mains;
    return d;
  }
  if (v.kind === 'keypad') return { kind: 'keypad', name: v.id, pos: V(v.pos), yaw: v.yaw, door: v.door, code: v.code };
  const d: DeviceDef = { kind: 'laser', name: v.id, from: V(v.from), to: V(v.to) };
  if (v.group !== undefined) d.group = v.group; if (v.mains !== undefined) d.mains = v.mains; if (v.alarm !== undefined) d.alarm = v.alarm;
  return d;
}
/** …and back (the exporter's / round trip's direction): name → id, no floor written (absent = the first, as for doors) */
function deviceData(d: DeviceDef): DeviceData {
  if (d.kind === 'camera') {
    const v: DeviceData = { id: d.name, kind: 'camera', pos: V(d.pos), yaw: d.yaw };
    if (d.pitch !== undefined) v.pitch = d.pitch; if (d.sweep !== undefined) v.sweep = [d.sweep[0], d.sweep[1]]; if (d.sweepPeriod !== undefined) v.sweepPeriod = d.sweepPeriod;
    if (d.fovDeg !== undefined) v.fovDeg = d.fovDeg; if (d.range !== undefined) v.range = d.range; if (d.group !== undefined) v.group = d.group; if (d.mains !== undefined) v.mains = d.mains;
    return v;
  }
  if (d.kind === 'keypad') return { id: d.name, kind: 'keypad', pos: V(d.pos), yaw: d.yaw, door: d.door, code: d.code };
  const v: DeviceData = { id: d.name, kind: 'laser', from: V(d.from), to: V(d.to) };
  if (d.group !== undefined) v.group = d.group; if (d.mains !== undefined) v.mains = d.mains; if (d.alarm !== undefined) v.alarm = d.alarm;
  return v;
}

// ------------------------------------------------------------------------------------------------------------------------------------------------ Level → LevelData
export interface LevelToDataOpts {
  name: string;
  /** the paint palette: any box / part albedo that equals a material's albedo exactly is written as that material key; the rest stay inline */
  materials?: Record<string, MaterialData>;
  /** per box index: the id to write (absent → a semantic id for referenced boxes, else box_NNNN) and the editor group */
  boxIds?: readonly (string | undefined)[]; boxGroups?: readonly (string | undefined)[];
  floors?: FloorData[]; surfaces?: SurfacesData;
  /** (not `markTexel` or `kept`: those two are the Level's own and are written from it. Either one in an object passed here is dropped) */
  meta?: Omit<LevelData['meta'], 'markTexel' | 'kept'>;
  /** pass-through for what Level has no slot for yet */
  volumes?: LevelData['volumes'];
}
/** Convert a runtime Level (the procedural generator's, or a loaded one) into level data. Optional fields are written iff the Level had them, so
 *  loadLevel(levelToData(L)) reproduces L key for key; ids are generated deterministically (see boxIdFor) unless supplied. */
export function levelToData(level: Level, opts: LevelToDataOpts): LevelData {
  const floors: FloorData[] = opts.floors?.map(f => ({ ...f })) ?? [{ id: 'f0', name: 'ground floor', y: 0, ceiling: 3 }];
  const F0 = floors[0].id;
  const materials: Record<string, MaterialData> = {}; for (const [k, m] of Object.entries(opts.materials ?? {})) materials[k] = { albedo: V(m.albedo), ...(m.rough !== undefined ? { rough: m.rough } : {}), ...(m.metal ? { metal: true } : {}), ...(m.stuff !== undefined ? { stuff: m.stuff } : {}), ...(m.does !== undefined ? { does: m.does } : {}), ...(m.notes ? { notes: m.notes } : {}) };
  const matByAlbedo = new Map<string, string>(); for (const [k, m] of Object.entries(materials)) { const key = m.albedo.join(','); if (!matByAlbedo.has(key)) matByAlbedo.set(key, k); }
  const paint = (albedo: Vec3): { material: string } | { albedo: Vec3 } => { const m = matByAlbedo.get(albedo.join(',')); return m !== undefined ? { material: m } : { albedo: V(albedo) }; };
  /** the finish a box carries beyond what its paint already says: written per box only where it differs from the material's (or, inline, from matte
   *  dielectric) — so loadLevel resolves it back to the same Box, and a palette finish is ONE line in `materials`, not four hundred */
  const finish = (b: { rough?: number; metal?: boolean }, p: { material?: string }): { rough?: number; metal?: boolean } => {
    const m = p.material !== undefined ? materials[p.material] : undefined;
    const out: { rough?: number; metal?: boolean } = {};
    if (b.rough !== m?.rough) out.rough = b.rough ?? ROUGH_DEFAULT;   // (an explicit matte over a finished material: the default written out — loads back as rough 1, the same finish if not the same key)
    if (!!b.metal !== !!m?.metal) out.metal = !!b.metal;
    return out;
  };
  /** what a box says beyond its paint's row (scene/stuff.ts): written per box only where it is another thing than its material says, 'plain' where it is
   *  nothing over a row that says something; a box with an inline albedo writes its word unless it is plain */
  const says = (b: { stuff?: string; does?: string }, p: { material?: string }): { stuff?: string; does?: string } => {
    const row = p.material !== undefined ? materials[p.material] : undefined, mine = b.stuff ?? 'plain', rows = row?.stuff ?? 'plain';
    // … and what it DOES (scene/materials.ts), the same way: per box only where it is another row than its material's, the default row's word where it
    // does nothing over a row that says something
    const does = b.does ?? DOES_DEFAULT, rowDoes = row?.does ?? DOES_DEFAULT;
    return { ...(mine !== rows ? { stuff: mine } : {}), ...(does !== rowDoes ? { does } : {}) };
  };
  // box ids: supplied → the box's own name → what references it (breaker, a light's fixture, a switchable) → box_NNNN
  const firstPiece = level.boxes.findIndex(b => b.piece === true), n = firstPiece < 0 ? level.boxes.length : firstPiece; const ids: string[] = new Array(n);   // (the level's own boxes: a stood level's pieces — scene/shapes.ts — follow them and are written inside the shape that says them)
  const semantic = new Map<number, string>();
  semantic.set(level.breaker.boxes[0], 'breaker_panel'); semantic.set(level.breaker.boxes[1], 'breaker_led');
  for (const l of level.lights) if (l.fixtureBox !== undefined && !semantic.has(l.fixtureBox)) semantic.set(l.fixtureBox, `fix_${l.name}`);
  for (const s of level.switchables) s.boxes.forEach((bi, k) => { if (!semantic.has(bi)) semantic.set(bi, s.boxes.length === 1 ? `sw_${s.name}` : `sw_${s.name}_${k}`); });
  const used = new Set<string>();
  for (let i = 0; i < n; i++) {
    let id = opts.boxIds?.[i] ?? level.boxes[i].name ?? semantic.get(i) ?? `box_${String(i).padStart(4, '0')}`;
    if (used.has(id)) { let k = 2; while (used.has(`${id}_${k}`)) k++; id = `${id}_${k}`; }
    used.add(id); ids[i] = id;
  }
  // bits the runtime derives and may have OR'd into a Level it was handed (rust/game_rules/src/game sets AreaLit on lit fixtures; Emissive / Rot / Dynamic live in the GPU word
  // or on prop boxes) are not authored data: masked here so 'export the level the game is running' works off game.level too. Anything else unknown still throws.
  const DERIVED = BoxFlag.AreaLit | BoxFlag.Emissive | BoxFlag.Rot | BoxFlag.Dynamic | BoxFlag.Thin | BoxFlag.Bar;
  const boxes: BoxData[] = level.boxes.slice(0, n).map((b, i) => {
    if (b.owner) throw new Error(`box ${i}: owner ${b.owner} — level boxes belong to the world`);
    const pt = paint(b.albedo);
    const d: BoxData = { id: ids[i], floor: F0, ...(opts.boxGroups?.[i] ? { group: opts.boxGroups[i] } : {}), ...(b.name !== undefined ? { name: b.name } : {}), c: V(b.c), h: V(b.h), ...(b.yaw !== 0 ? { yaw: b.yaw } : {}), ...(b.rot ? { rot: [b.rot[0], b.rot[1], b.rot[2], b.rot[3]] } : {}), ...(b.round ? { round: true } : {}), ...pt, ...finish(b, pt as { material?: string }) };
    if (b.emissive[0] !== 0 || b.emissive[1] !== 0 || b.emissive[2] !== 0) d.emissive = V(b.emissive);
    const flags = bitsToFlags(b.flags & ~DERIVED); if (flags.length) d.flags = flags;
    if (b.cutHeight !== 1e6) d.cutHeight = b.cutHeight;
    const word = says(b, pt as { material?: string }); if (word.stuff !== undefined) d.stuff = word.stuff; if (word.does !== undefined) d.does = word.does;
    const marks = copyMarkings(b.marks); if (marks) d.markings = marks;
    if (b.shape) {   // A SHAPE (scene/shapes.ts), as it was read: a piece's paint by name where the level has one, its finish and its word only where they are not the material's
      const s = b.shape, sd: ShapeData = { ...formOf(s) };
      if (s.with) sd.with = s.with.map((w): ShapePieceData => { if (!w.albedo) return { off: V(w.off), h: V(w.h), ...(w.yaw ? { yaw: w.yaw } : {}), ...formOf(w) }; const wp = paint(w.albedo); return { off: V(w.off), h: V(w.h), ...(w.yaw ? { yaw: w.yaw } : {}), ...wp, ...finish(w, wp as { material?: string }), ...says(w, wp as { material?: string }), ...formOf(w) }; });
      d.shape = sd;
    }
    return d;
  });
  const props: PropData[] = level.props.map(p => {
    const parts = p.parts.map(q => { const marks = copyMarkings(q.marks), pt = paint(q.albedo); return { off: V(q.off), h: V(q.h), ...(q.yaw !== 0 ? { yaw: q.yaw } : {}), ...pt, ...says(q, pt as { material?: string }), ...(marks ? { markings: marks } : {}) }; });
    const d: PropData = { id: p.name, kind: p.kind, x: p.x, z: p.z, ...(p.yaw !== 0 ? { yaw: p.yaw } : {}), mass: p.mass, parts };
    const fit = fitPropFootprint(p.parts);   // pin the footprint only where the fit would not give the authored numbers back bit for bit (plants override theirs)
    if (fit.half[0] !== p.half[0] || fit.half[1] !== p.half[1]) d.half = [p.half[0], p.half[1]];
    if (fit.height !== p.height) d.height = p.height;
    return d;
  });
  const lightData = (l: LightDef): LightData => {
    const d: LightData = { id: l.name, kind: l.kind, pos: V(l.pos), dir: V(l.dir), color: V(l.color), intensity: l.intensity, range: l.range, innerDeg: l.innerDeg, outerDeg: l.outerDeg, group: l.group, enabled: l.enabled, volumetric: l.volumetric };
    if (l.radius !== undefined) d.radius = l.radius; if (l.mains !== undefined) d.mains = l.mains; if (l.broad !== undefined) d.broad = l.broad;
    if (l.fixtureBox !== undefined) d.fixture = ids[l.fixtureBox];
    return d;
  };
  const slug = (s: string) => s.toLowerCase().replace(/[^a-z0-9]+/g, '_').replace(/^_|_$/g, '');
  const uniqIds = (base: string[]) => { const seen = new Set<string>(); return base.map(id => { let out = id || 'x'; if (seen.has(out)) { let k = 2; while (seen.has(`${out}_${k}`)) k++; out = `${out}_${k}`; } seen.add(out); return out; }); };
  const planIds = uniqIds((level.clearPlans ?? []).map(p => `clear_${slug(p.door)}`));
  const planData = <T extends RoomClearPlan>(p: T, id: string) => ({ id, ...clone(p) });
  const M = level.mission;
  const data: LevelData = {
    version: LEVEL_DATA_VERSION,
    name: opts.name,
    // (what the loader hands back ON THE LEVEL from the file's meta is written from the Level, iff it has it: its marks' texel and how kept it is. The
    //  Level is the one source: either key in a caller's opts.meta is dropped, so a file cannot say a word its Level does not, and a round trip holds)
    meta: (() => { const { markTexel: _texel, kept: _kept, ...rest } = clone((opts.meta ?? {}) as LevelData['meta']); return { ...rest, ...(level.markTexel !== undefined ? { markTexel: level.markTexel } : {}), ...(level.kept !== undefined ? { kept: level.kept } : {}) }; })(),
    floors,
    materials,
    ...(opts.surfaces ? { surfaces: clone(opts.surfaces) } : {}),
    boxes, props,
    lights: level.lights.map(lightData),
    switchables: level.switchables.map(s => ({ id: s.name, boxes: s.boxes.map(bi => ids[bi]), group: s.group, on: s.on, baseEmissive: V(s.baseEmissive), ...(s.flicker !== undefined ? { flicker: s.flicker } : {}), ...(s.mains !== undefined ? { mains: s.mains } : {}) })),
    doors: level.doors.map(o => { const d: LevelData['doors'][number] = { id: o.name, hinge: [o.hinge[0], o.hinge[1]], closedDir: o.closedDir, width: o.width }; for (const k of ['angle', 'exterior', 'closer', 'minAngle', 'maxAngle', 'locked', 'lock', 'keySide', 'steel', 'swing'] as const) if (o[k] !== undefined) (d as any)[k] = o[k]; return d; }),
    routes: uniqIds(level.routes.map(r => slug(r.name))).map((id, i) => { const r = level.routes[i]; return { id, ...(r.name !== id ? { name: r.name } : {}), points: VV(r.points), wait: [...r.wait], ...(r.weapon !== undefined ? { weapon: r.weapon } : {}), ...(r.keys !== undefined ? { keys: [...r.keys] } : {}) }; }),
    chokepoints: uniqIds((level.chokepoints ?? []).map(c => slug(c.name))).map((id, i) => { const c = level.chokepoints![i]; return { id, ...(c.name !== id ? { name: c.name } : {}), pos: V(c.pos), yaw: c.yaw }; }),
    ...(level.civilians ? { civilians: uniqIds(level.civilians.map(c => slug(c.name))).map((id, i): CivilianData => { const c = level.civilians![i]; return { id, ...(c.name !== id ? { name: c.name } : {}), kind: c.kind, tint: V(c.tint), ...(c.tint2 !== undefined ? { tint2: V(c.tint2) } : {}), points: VV(c.points), wait: [...c.wait], ...(c.tasks !== undefined ? { tasks: [...c.tasks] } : {}), ...(c.face !== undefined ? { face: [...c.face] } : {}) }; }) } : {}),
    volumes: level.volumes ? level.volumes.map((v): LevelData['volumes'][number] => ({ id: v.name, kind: v.kind, c: V(v.c), h: V(v.h), ...(v.yaw !== undefined ? { yaw: v.yaw } : {}) })) : clone(opts.volumes ?? []), devices: (level.devices ?? []).map(deviceData),   // (a Level that carried volumes writes them back; one without keeps the pass-through)
    ...(level.marks?.length ? { marks: level.marks.map((m): MarkData => ({ id: m.name, pos: V(m.pos), ...(m.yaw !== undefined ? { yaw: m.yaw } : {}) })) } : {}),
    ...(level.vents ? { vents: level.vents.map((d): VentData => { const v: VentData = { id: d.name, pos: V(d.pos), dir: V(d.dir), speed: d.speed, spread: d.spread, reach: d.reach }; for (const k of ['size', 'period', 'duty', 'phase', 'mains'] as const) if (d[k] !== undefined) (v as any)[k] = d[k]; return v; }) } : {}),
    connectors: [],
    mission: { playerSpawn: V(level.playerSpawn), objectives: [
      { id: 'reach_room', kind: 'reach', area: { x0: M.serverRoom.x0, x1: M.serverRoom.x1, z0: M.serverRoom.z0, z1: M.serverRoom.z1 }, door: M.entryDoor },
      { id: 'pull_drive', kind: 'drive', rackIndex: M.rack.index, front: V(M.rack.front), halfW: M.rack.halfW },
      { id: 'exfil', kind: 'exfil', door: M.exfilDoor, x: M.exfilX, z: M.exfilZ, ...(M.exfilOut ? { out: [M.exfilOut[0], M.exfilOut[1]] as [number, number] } : {}) },
    ] },
    breaker: { pos: V(level.breaker.pos), panel: ids[level.breaker.boxes[0]], led: ids[level.breaker.boxes[1]], ...(level.breaker.fumble !== undefined ? { fumble: level.breaker.fumble } : {}) },
    beacons: level.beacons.map((b, i) => ({ id: `beacon${i}`, pos: V(b.pos), box: ids[b.box] })),
    sky: { zenith: V(level.sky.zenith), horizon: V(level.sky.horizon) },
    plans: { roomClear: (level.clearPlans ?? []).map((p, i) => planData(p, planIds[i]) as RoomClearPlanData), ...(level.breach ? { breach: planData(level.breach, `breach_${slug(level.breach.door)}`) as BreachPlanData } : {}) },
    ...(level.scenarios ? { scenarios: clone(level.scenarios) } : {}),   // (the level's own block: lib/scenarios.ts)
  };
  return data;
}
