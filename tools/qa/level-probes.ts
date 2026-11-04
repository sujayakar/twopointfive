// The level as data (levels/office.json ⇄ src/scene/levelData.ts / levelLoad.ts): the identity gate and the format's own checks.
//   level-identity   A = the legacy procedural generator (officeGen.ts buildOfficeProcedural), B = buildLevel() (levels/office.json → loadLevel):
//                    every box (c, h, yaw, rot, albedo, emissive, flags, name — in ORDER), light, switchable, door, route, prop, plan, the mission,
//                    breaker, beacons, sky, daylight, breach compared structurally → ZERO diffs; and the two converted to level data hash to one digest.
//                    Plus: the bundled JSON is what the generator writes today (tools/level-export.ts --check semantics, in-process).
//   level-validate   validateLevelData(office) clean; the validator TRIPS on planted faults (unknown box ref, runtime-only flag, duplicate id, wait/points
//                    mismatch, missing objective); the surface map in the file agrees with game/floor.ts (which still owns the behaviour).
//   level-roundtrip  LevelData → Level → LevelData is the identity on the office (ids, groups, materials, pinned footprints all survive), the serializer is
//                    a fixed point (serialize ∘ parse ∘ serialize), and two loads share no mutable state.
//   level-lint       the design lint (src/scene/lint.ts) clean on the office with its sidecar, each planted fault reported under its own code, the marking
//                    table's rows tallied, the CLI's exit codes (tools/qa/level-lint.ts).
//   level-serve      the dev server's /levels/ (tools/levels-api.ts): the index, a file, its sidecar, and what it refuses.
//   bun run tools/qa/probes.ts level        (each prints PASS / FAIL lines; probes.ts sets exit code 1 if any level probe FAILed)
import { mkdtempSync, mkdirSync, readFileSync, writeFileSync, rmSync } from 'node:fs';
import { tmpdir } from 'node:os';
import { join } from 'node:path';
import { ROOT, asExporter } from './headless.ts';

const { buildLevel, OFFICE_DATA } = await import(`${ROOT}/src/scene/level.ts`);
const { buildOfficeProcedural, OFFICE_PALETTE, OFFICE_FINISH } = await import(`${ROOT}/src/scene/officeGen.ts`);
const { loadLevel, levelToData } = await import(`${ROOT}/src/scene/levelLoad.ts`);
const { validateLevelData, serializeLevelData, parseLevelData, levelDataDigest, structuralDiff, canonicalLevelData, indexLevelData, freshId } = await import(`${ROOT}/src/scene/levelData.ts`);
const { WORLD } = await import(`${ROOT}/src/scene/world.ts`);
const { FINISHES, BUILDING, GRASS_EDGE, floorAt } = await import(`${ROOT}/src/game/floor.ts`);
const { EXACT } = await import(`${ROOT}/src/math/exact.ts`);
const { lintLevel, formatLint } = await import(`${ROOT}/src/scene/lint.ts`);
const officeAllow = JSON.parse(readFileSync(`${ROOT}/levels/office.lint.json`, 'utf8')).allow;

let failures = 0;
function verdict(ok: boolean, what: string, detail = '') { if (!ok) failures++; console.log(`  ${ok ? 'PASS' : 'FAIL'} ${what}${detail ? ` — ${detail}` : ''}`); }
export function levelProbeFailures() { return failures; }

/** the exporter's annotations, rebuilt here so the probes do not depend on the tool (same inputs: palette, floor.ts, WORLD) */
function exportOpts(trace: { groups: string[] }) {
  const materials: Record<string, { albedo: number[]; rough?: number; metal?: boolean }> = {}; for (const [k, a] of Object.entries(OFFICE_PALETTE as Record<string, number[]>)) materials[k] = { albedo: [...a], ...((OFFICE_FINISH as Record<string, { rough: number; metal?: boolean }>)[k] ?? {}) };   // palette + finish, as tools/level-export.ts builds them
  const zoneIds = ['corridor_lino', 'lobby_stone', 'server_tile', 'breakroom_lino'];
  return {
    name: 'office', materials, boxGroups: trace.groups,
    surfaces: { building: { ...BUILDING }, grassEdge: GRASS_EDGE, inside: 'carpet', outside: 'asphalt', verge: 'grass', zones: FINISHES.map((f: any, i: number) => ({ id: zoneIds[i], mat: f.mat, x0: f.x0, z0: f.z0, x1: f.x1, z1: f.z1, notes: f.room })) },
    floors: [{ id: 'f0', name: 'ground floor', y: 0, ceiling: WORLD.ceilingY }],
    meta: OFFICE_DATA.meta,
  };
}

export async function levelIdentity() {
  console.log('\n=== level-identity: the procedural generator vs levels/office.json → loadLevel, field for field ===');
  // (the whole comparison runs AS THE EXPORTER RUNS — headless.ts asExporter: on the engine's maths on every stand. The file on disk is what the exporter wrote,
  // and it raises no stand; on a stand with the game's own maths up (math/exact.ts EXACT.on: the shipped one) the generator's sin / cos, and the loader's and the
  // converter's fit of a prop's footprint, round a last bit another way, which would read as a stale file that no re-export can mend. What the generator makes
  // on this stand's own maths is counted in a note after the verdicts, not gated.)
  let bundled = '';
  asExporter(() => {
    const trace = { groups: [] as string[] };
    const A = buildOfficeProcedural(trace);          // the legacy generator
    const B = buildLevel();                          // the bundled JSON through the loader — what main.ts and the harness boot
    const diffs: string[] = []; const n = structuralDiff(A, B, diffs, 5);
    console.log(`  A (generator): ${A.boxes.length} boxes · ${A.lights.length} lights · ${A.switchables.length} switchables · ${A.doors.length} doors · ${A.routes.length} routes · ${A.props.length} props · ${A.clearPlans.length} clear plans`);
    console.log(`  B (office.json): ${B.boxes.length} boxes · ${B.lights.length} lights · ${B.switchables.length} switchables · ${B.doors.length} doors · ${B.routes.length} routes · ${B.props.length} props · ${B.clearPlans.length} clear plans`);
    verdict(n === 0, `Level A ≡ Level B structurally (${n} diff${n === 1 ? '' : 's'})`, n ? `first ${diffs.length}: ${diffs.join(' | ')}` : 'boxes in order with c/h/yaw/rot/albedo/emissive/flags/name/owner/cutHeight, lights, switchables (box indices), doors, routes, chokepoints, props (parts, half, height, mass), clear plans, breach, daylight, mission, breaker, beacons, sky, spawn');
    // per-part tallies, so a reader sees what was compared even at 0
    const parts: [string, unknown, unknown][] = [['boxes', A.boxes, B.boxes], ['lights', A.lights, B.lights], ['switchables', A.switchables, B.switchables], ['doors', A.doors, B.doors], ['routes', A.routes, B.routes], ['chokepoints', A.chokepoints, B.chokepoints], ['props', A.props, B.props], ['clearPlans', A.clearPlans, B.clearPlans], ['breach', A.breach, B.breach], ['daylight', A.daylight, B.daylight], ['mission', A.mission, B.mission], ['breaker', A.breaker, B.breaker], ['beacons', A.beacons, B.beacons], ['sky', A.sky, B.sky], ['playerSpawn', A.playerSpawn, B.playerSpawn]];
    console.log('  per part: ' + parts.map(([k, a, b]) => `${k} ${structuralDiff(a, b, [], 0)}`).join(' · ') + ' (diffs)');
    verdict(Object.keys(A).join(',') === Object.keys(B).join(','), 'same top-level key order', Object.keys(B).join(','));
    // the strictest leaf check there is: Object.is over every number (−0 is the one thing === forgives) — reported, not gated: the file prints −0 as 0
    let negZeroA = 0; const scan = (v: any) => { if (typeof v === 'number') { if (Object.is(v, -0)) negZeroA++; } else if (v && typeof v === 'object') for (const k of Object.keys(v)) scan(v[k]); }; scan(A);
    console.log(`  note: the generator emits ${negZeroA} negative zero(s) (the bike's tilted-tube quaternions: 0·sin); the file holds them as 0 — equal under ===, and ±0 cannot change a rotation`);
    // digests: both Levels converted with the same annotations hash alike, and equal the bundled data's digest
    const dA = levelDataDigest(levelToData(A, exportOpts(trace))), dB = levelDataDigest(levelToData(B, { ...exportOpts(trace) })), dFile = levelDataDigest(OFFICE_DATA);
    verdict(dA === dB && dB === dFile, 'digest(generator → data) = digest(loaded → data) = digest(levels/office.json)', `${dA} / ${dB} / ${dFile}`);
    // staleness: what the exporter would write now IS the bundled file (byte for byte after canonical serialization)
    const fresh = serializeLevelData(levelToData(A, exportOpts(trace))); bundled = serializeLevelData(OFFICE_DATA);
    verdict(fresh === bundled, 'levels/office.json is what tools/level-export.ts writes from the generator today (not stale)', fresh === bundled ? `${(fresh.length / 1024).toFixed(1)} KB, ${fresh.split('\n').length - 1} lines` : 'run: bun run tools/level-export.ts');
  });
  const onDisk = await Bun.file(`${ROOT}/levels/office.json`).text();
  verdict(onDisk === bundled, 'the file on disk is in canonical form (serialize(parse(file)) === file)', onDisk === bundled ? '' : 'hand-edited out of canonical order/format — re-save through serializeLevelData');
  if (EXACT.on) { let said = 'the generator threw'; try { const d: string[] = [], n = structuralDiff(buildOfficeProcedural({ groups: [] }), buildLevel(), d, 2); said = `${n} difference(s)${n ? `: ${d.join(' | ')}` : ''}`; } catch { /* a note, never a verdict */ }
    console.log(`  note: on this stand's own maths, the generator against the loaded file: ${said} — not gated (a last bit of a sin or a cos; more than a handful means a branch in the generator turns on one)`); }
}

export async function levelValidate() {
  console.log('\n=== level-validate: the schema validator on the office, on planted faults, and the surface map vs floor.ts ===');
  const errs = validateLevelData(OFFICE_DATA);
  verdict(errs.length === 0, `validateLevelData(office): ${errs.length} error(s)`, errs.slice(0, 5).join(' | '));
  // planted faults, one at a time on a deep copy — each must be reported (the tripwires are live)
  const base = JSON.stringify(OFFICE_DATA);
  const plant = (name: string, mut: (d: any) => void, expect: RegExp) => {
    const d = JSON.parse(base); mut(d); const e: string[] = validateLevelData(d);
    const hit = e.some(x => expect.test(x)); verdict(hit, `planted: ${name}`, hit ? `→ "${e.find(x => expect.test(x))}"` : `NOT reported (${e.length} other error(s): ${e.slice(0, 2).join(' | ')})`);
  };
  plant('a switchable pointing at a box id that does not exist', d => { d.switchables[0].boxes[0] = 'box_nope'; }, /unknown box 'box_nope'/);
  plant('a runtime-only flag (AreaLit) authored on a box', d => { d.boxes[10].flags = ['AreaLit']; }, /runtime-only flag 'AreaLit'/);
  plant('two boxes with one id', d => { d.boxes[5].id = d.boxes[4].id; }, /duplicate id/);
  plant('a route whose wait list is short a point', d => { d.routes[0].wait.pop(); }, /wait has \d+ entries for \d+ points/);
  plant('a route named by a number (the game looks a route up by its name)', d => { d.routes[0].name = 7; }, /routes\[0\].*name string/);
  plant('a chokepoint with an empty name', d => { d.chokepoints[0] = { ...d.chokepoints[0], name: '' }; }, /chokepoints\[0\].*name string/);
  plant("the mission without its 'exfil' objective", d => { d.mission.objectives = d.mission.objectives.filter((o: any) => o.kind !== 'exfil'); }, /exactly one 'exfil'/);
  plant('a light fixture naming a missing box', d => { d.lights[0].fixture = 'fix_gone'; }, /unknown box 'fix_gone'/);
  plant('a beacon dome no switchable owns', d => { d.beacons[0].box = d.boxes[0].id; }, /not owned by any switchable/);
  plant('a box with both material and albedo', d => { d.boxes[0].albedo = [1, 1, 1]; }, /exactly one of material \/ albedo/);
  plant('a clear plan on a door that is not in the file', d => { d.plans.roomClear[0].door = 'nowhere'; }, /unknown door 'nowhere'/);
  plant('a keypad device (G4 stub) on an unknown door', d => { d.devices.push({ id: 'kp0', kind: 'keypad', pos: [1, 1, 1], yaw: 0, door: 'vault', code: '1234' }); }, /unknown door 'vault'/);
  // devices (G4): the keypad ⇄ door link is the lock id, codes are four digits, beams have length, cameras have a sane cone
  plant('a keypad whose code is not four digits', d => { d.devices.find((v: any) => v.kind === 'keypad').code = '12a4'; }, /code must be four digits/);
  plant('a keypad on a door whose lock is not the keypad', d => { d.doors.find((o: any) => o.id === 'server_manager').lock = 'floor'; }, /lock must be the keypad's id/);
  plant('a keypad on an unlocked door', d => { const o = d.doors.find((x: any) => x.id === 'server_manager'); delete o.locked; }, /is not locked/);
  plant('two keypads claiming one door', d => { const k = d.devices.find((v: any) => v.kind === 'keypad'); d.devices.push({ ...k, id: 'kp_dup' }); }, /already has keypad|lock must be the keypad's id/);
  plant('a laser whose ends coincide', d => { const l = d.devices.find((v: any) => v.kind === 'laser'); l.to = [...l.from]; }, /a beam needs length/);
  plant("a laser with alarm 'panic'", d => { d.devices.find((v: any) => v.kind === 'laser').alarm = 'panic'; }, /alarm 'suspicion' \| 'alarm'/);
  plant('a camera with a 200° cone', d => { d.devices.find((v: any) => v.kind === 'camera').fovDeg = 200; }, /fovDeg in \(0, 170\]/);
  plant('a camera with the old battery key', d => { d.devices.find((v: any) => v.kind === 'camera').battery = true; }, /unknown key 'battery'/);
  // vents (game/wind.ts): ids unique, dir non-zero, the numbers in their bands, no stray keys
  plant('a vent with a zero direction', d => { d.vents[0].dir = [0, 0, 0]; }, /vents\[0\] 'vent_cub_nw': dir must be a non-zero Vec3/);
  plant('a vent blowing at 40 m/s', d => { d.vents[1].speed = 40; }, /speed m\/s in \[0, 12\]/);
  plant('two vents with one id', d => { d.vents[2].id = d.vents[1].id; }, /vents\[2\]: duplicate id/);
  plant('a vent with a typo\'d key ("reech")', d => { d.vents[3].reech = 2; }, /unknown key 'reech'/);
  plant('a stair connector (G2 stub) to an unknown floor', d => { d.connectors.push({ id: 'st0', kind: 'stair', from: { floor: 'f0', c: [1, 0, 1], h: [1, 1, 1] }, to: { floor: 'mezz', c: [1, 3, 1], h: [1, 1, 1] } }); }, /connectors\[0\]\.to/);
  plant('version 2 handed to the v1 loader', d => { d.version = 2; }, /version: expected 1/);
  // hand-edit typos and prototype tricks: an unknown key anywhere is data that would be ignored on load and dropped on save — reported, per record
  plant('a typo\'d key on a box ("emmisive")', d => { d.boxes[7].emmisive = [1, 1, 1]; }, /boxes\[7\] 'box_0007': unknown key 'emmisive'/);
  plant('a typo\'d key on a light ("colour")', d => { d.lights[1].colour = [1, 1, 1]; }, /lights\[1\] 'storage_bulb': unknown key 'colour'/);
  plant('an unknown top-level section', d => { d.triggers = []; }, /level: unknown key 'triggers'/);
  plant('playerYaw (dropped from v1: nothing reads it)', d => { d.mission.playerYaw = 1; }, /mission: unknown key 'playerYaw'/);
  plant("a box painted with material 'constructor'", d => { delete d.boxes[2].material; d.boxes[2].material = 'constructor'; }, /unknown material 'constructor'/);
  plant('one box listed twice in a switchable', d => { d.switchables[3].boxes.push(d.switchables[3].boxes[0]); }, /listed twice/);
  plant('a typo\'d plans key ("breech")', d => { d.plans.breech = d.plans.breach; delete d.plans.breach; }, /plans: unknown key 'breech'/);
  plant('an id that is a record with a non-callable toString', d => { d.boxes[1].id = { toString: 0 }; d.doors[0].id = { toString: 0 }; d.lights[0].fixture = { toString: 0 }; }, /boxes\[1\]: missing id/);
  plant('meta as an array', d => { d.meta = []; }, /meta: must be an object/);
  plant('materials as an array', d => { d.materials = []; }, /materials: must be an object/);
  plant('meta.bounds set to null', d => { d.meta.bounds = null; }, /meta\.bounds: \{ min: Vec3, max: Vec3 \}/);
  // shapes a hand edit can leave behind must be REPORTED, never thrown on (validate runs at boot: a TypeError there is a blank page)
  plant('a null where a box should be', d => { d.boxes[3] = null; }, /boxes\[3\]: must be an object/);
  plant('daylight set to null', d => { d.daylight = null; }, /daylight: must be an object/);
  plant('a clear plan with its lines block deleted', d => { delete d.plans.roomClear[1].lines; }, /lines: all twelve strings/);
  plant('the required devices array left out', d => { delete d.devices; }, /devices: must be an array/);
  // loadLevel refuses invalid data (throws with the list) rather than building half a level
  { const d = JSON.parse(base); d.breaker.panel = 'gone'; let threw = ''; try { loadLevel(d); } catch (e: any) { threw = String(e.message ?? e); } verdict(/unknown box 'gone'/.test(threw), 'loadLevel throws on invalid data instead of building it', threw.split('\n')[0]); }
  // the surface map mirrored into the file agrees with floor.ts, rect for rect, and floorAt agrees with the zones at their centres
  const S = OFFICE_DATA.surfaces;
  verdict(!!S, 'office.json carries the footstep surface map (surfaces)');
  if (S) {
    const same = S.building.x0 === BUILDING.x0 && S.building.x1 === BUILDING.x1 && S.building.z0 === BUILDING.z0 && S.building.z1 === BUILDING.z1 && S.grassEdge === GRASS_EDGE && S.zones.length === FINISHES.length && S.zones.every((z: any, i: number) => z.mat === FINISHES[i].mat && z.x0 === FINISHES[i].x0 && z.x1 === FINISHES[i].x1 && z.z0 === FINISHES[i].z0 && z.z1 === FINISHES[i].z1);
    verdict(same, 'surfaces ≡ game/floor.ts (BUILDING, GRASS_EDGE, FINISHES) — floor.ts still owns the behaviour; the file mirrors it until it is wired', same ? `${S.zones.length} zones` : 'DRIFT: re-export or update floor.ts');
    const centres = S.zones.map((z: any) => ({ z, got: floorAt([(z.x0 + z.x1) / 2, 0, (z.z0 + z.z1) / 2]) }));
    verdict(centres.every((c: any) => c.got === c.z.mat), 'floorAt() at every zone centre returns that zone\'s material', centres.map((c: any) => `${c.z.id} → ${c.got}`).join(' · '));
  }
  // every finish box the generator laid (group 'floors', not a rug) sits inside the zone list or is carpet — a cheap cross-check that geometry and map agree
  { const finishes = OFFICE_DATA.boxes.filter((b: any) => b.group === 'floors' && b.c[1] > 0.001 && b.c[1] < 0.01);
    const matOf = (b: any) => b.material ?? '?';
    const rows = finishes.map((b: any) => ({ id: b.id, mat: matOf(b), under: floorAt(b.c) }));
    const bad = rows.filter((r: any) => (r.mat === 'lino' && r.under !== 'lino') || (r.mat === 'stone' && r.under !== 'stone') || ((r.mat === 'carpetBlue' || r.mat === 'carpetRed') && r.under !== 'carpet'));
    verdict(bad.length === 0, `the ${finishes.length} finish decals the generator laid agree with floorAt() underneath them`, bad.length ? bad.map((r: any) => `${r.id} ${r.mat} over ${r.under}`).join(' | ') : rows.map((r: any) => `${r.mat}→${r.under}`).join(' ')); }
}

export async function levelRoundtrip() {
  console.log('\n=== level-roundtrip: LevelData → Level → LevelData, the serializer as a fixed point, no shared state between loads ===');
  const D0 = OFFICE_DATA;
  const L = loadLevel(D0);
  // carry the annotations Level has no slot for (ids, groups, floors, surfaces, meta, materials) across, as the editor would hold them
  const D1 = levelToData(L, { name: D0.name, materials: D0.materials, boxIds: D0.boxes.map((b: any) => b.id), boxGroups: D0.boxes.map((b: any) => b.group), floors: D0.floors, surfaces: D0.surfaces, meta: D0.meta, volumes: D0.volumes, devices: D0.devices, connectors: D0.connectors });
  const diffs: string[] = []; const n = structuralDiff(canonicalLevelData(D0), canonicalLevelData(D1), diffs, 5);
  verdict(n === 0, `data → level → data: ${n} diff(s)`, n ? diffs.join(' | ') : `digest ${levelDataDigest(D0)} both ways`);
  verdict(levelDataDigest(D0) === levelDataDigest(D1), 'digests equal across the round trip');
  // the canonical form keeps EVERYTHING levelToData writes (raw exporter output vs its canonical copy: a field added to levelToData but not to KEYS would show here
  // instead of silently vanishing from the file) — and it is a deep copy (poking it leaves the source alone)
  { const dropped: string[] = []; const nd = structuralDiff(D1, canonicalLevelData(D1), dropped, 5);
    verdict(nd === 0, `canonicalLevelData drops nothing levelToData writes (${nd} diff(s) raw vs canonical)`, dropped.join(' | '));
    const C = canonicalLevelData(D0); C.boxes[0].c[0] += 5; C.materials.wall.albedo[0] = 9; C.plans.roomClear[0].stack[0][0] = -1;
    verdict(D0.boxes[0].c[0] !== C.boxes[0].c[0] && D0.materials.wall.albedo[0] !== 9 && D0.plans.roomClear[0].stack[0][0] !== -1, 'canonicalLevelData is a deep copy (mutating it leaves OFFICE_DATA intact)'); }
  // Level → data → Level as well (the direction the game cares about), from the loaded level
  const L2 = loadLevel(D1); const d2: string[] = []; const n2 = structuralDiff(L, L2, d2, 5);
  verdict(n2 === 0, `level → data → level: ${n2} diff(s)`, d2.join(' | '));
  // the serializer is a fixed point and the parser accepts what it writes
  const t1 = serializeLevelData(D0); const t2 = serializeLevelData(parseLevelData(t1));
  verdict(t1 === t2, 'serialize(parse(serialize(d))) === serialize(d)', `${t1.split('\n').length - 1} lines`);
  verdict(JSON.stringify(canonicalLevelData(JSON.parse(t1))) === JSON.stringify(canonicalLevelData(D0)), 'plain JSON.parse of the file gives the canonical data back');
  // key order independence: a copy with every record's keys reversed serializes to the same bytes and digest (the one ordered MAP, materials, keeps its
  // authored palette order by design — its entries are reversed inside, not the map itself)
  { const shuffle = (v: any, keepOrder = false): any => Array.isArray(v) ? v.map(x => shuffle(x)) : v && typeof v === 'object' ? Object.fromEntries((keepOrder ? Object.entries(v) : Object.entries(v).reverse()).map(([k, x]) => [k, shuffle(x, k === 'materials')])) : v;
    const Ds = shuffle(D0); verdict(serializeLevelData(Ds) === t1 && levelDataDigest(Ds) === levelDataDigest(D0), 'key order does not matter: a key-reversed copy serializes and hashes identically (materials keep their palette order by design)'); }
  // forward compatibility of the drill plans (typed by reference to level.ts, handled generically): a breach plan GROWN the way the breach-drill lane grows it —
  // aside → retreat, per-man flows, targets as records, a next-door stack, two more lines — must validate, serialize with nothing dropped (known keys first,
  // new ones after), parse back equal, and come out of loadLevel on Level.breach field for field
  { const d = JSON.parse(JSON.stringify(D0)); const B = d.plans.breach;
    delete B.aside; B.retreat = { mark: [27.3, 0, 19.9], face: 1.57 };
    B.flow = [{ into: [[28.1, 0, 17.6], [29.6, 0, 17.4]], centre: [[31, 1.1, 17], [33, 1.1, 19]], corner: [[29.4, 0, 21.6]], podFace: 2.9, cornerAim: [[31.5, 1, 23.5], [28.5, 1, 23.4]], hold: [31.6, 1, 23.8] },
              { into: [[28.1, 0, 18.1], [29.5, 0, 18.4]], centre: [[31, 1.1, 19], [33, 1.1, 16]], corner: [[29.3, 0, 14.6]], podFace: 0.4, cornerAim: [[28.6, 1, 13.8], [30.6, 1, 12.5]], hold: [30.6, 1.1, 12.5] }];
    B.targets = [{ centre: [1], corner: [0] }, { centre: [2], corner: [] }];
    B.next = { door: 'breakroom_n', stack: [[31.6, 0, 12.9], [32.4, 0, 13.0]], stackFace: [3.0, 3.05], stackSide: 1, stackAim: [[30.9, 1.05, 12.3], [30.8, 1.9, 12.25]] };
    B.breachLines = { breach1: 'breaching!', ...B.breachLines, next1: 'next door — stack left.' };
    const e: string[] = validateLevelData(d);
    verdict(e.length === 0, 'a breach plan grown by a later lane (retreat / flow / record targets / next / more lines) validates', e.slice(0, 3).join(' | '));
    const text2 = serializeLevelData(d); const back = parseLevelData(text2);
    const lost: string[] = []; const nLost = structuralDiff(d.plans.breach, back.plans.breach, lost, 5);   // the RAW grown plan vs what came back (structuralDiff ignores key order, so only a dropped / altered field can show)
    verdict(nLost === 0, 'its new fields survive serialize → parse (nothing dropped, order canonical)', nLost ? lost.join(' | ') : Object.keys(back.plans.breach).join(','));
    const grown = loadLevel(back).breach as any;
    verdict(!!grown && grown.retreat?.face === 1.57 && grown.flow?.length === 2 && grown.targets?.[0]?.centre?.[0] === 1 && grown.next?.door === 'breakroom_n' && grown.breachLines?.next1 === 'next door — stack left.' && grown.aside === undefined && grown.id === undefined, 'loadLevel hands the grown plan to Level.breach verbatim (id stripped, aside gone, retreat / flow / next / lines present)');
    B.flow[0].podFace = Infinity; const e2: string[] = validateLevelData(JSON.parse(JSON.stringify(d, (k, v) => v === Infinity ? '__inf' : v).replace('"__inf"', '1e999')));
    verdict(e2.some(x => /flow\[0\]\.podFace: non-finite/.test(x)), 'and a non-finite number inside a field this file does not name is still caught', e2.find(x => /non-finite/.test(x)) ?? e2.slice(0, 2).join(' | ')); }
  // two loads share nothing mutable: poke one, the other and the bundled data stay put
  { const X = buildLevel(), Y = buildLevel();
    const wallI = D0.boxes.findIndex((b: any) => b.material === 'wall'), wall0 = D0.materials.wall.albedo[0];   // a MATERIAL-painted box: the one path where a missing copy would alias materials.wall.albedo across hundreds of boxes
    X.boxes[0].c[0] += 1; X.boxes[0].flags |= 32; X.routes.push({ name: 'scratch', points: [[0, 0, 0]], wait: [0] }); X.lights[0].pos[1] = 99; X.clearPlans[0].stack[0][0] = -5; X.props[0].parts[0].off[0] = 7; X.mission.rack.front[0] = 0; X.boxes[wallI].albedo[0] = 0.99;
    const untouched = Y.boxes[0].c[0] === D0.boxes[0].c[0] && Y.boxes[0].flags === 0 && Y.routes.length === D0.routes.length && Y.lights[0].pos[1] === D0.lights[0].pos[1] && Y.clearPlans[0].stack[0][0] === D0.plans.roomClear[0].stack[0][0] && Y.props[0].parts[0].off[0] === D0.props[0].parts[0].off[0] && Y.mission.rack.front[0] === (D0.mission.objectives.find((o: any) => o.kind === 'drive') as any).front[0]
      && Y.boxes[wallI].albedo[0] === wall0 && D0.materials.wall.albedo[0] === wall0 && X.boxes[wallI + 1].albedo[0] === wall0;
    verdict(untouched, 'two buildLevel() calls share no arrays with each other or with the bundled data — a material-painted albedo included (mutating one leaves the other, its neighbours and OFFICE_DATA intact)'); }
  // 'export the level the game is running': a Level the Game constructor has touched carries AreaLit on lit fixtures — levelToData masks the derived bits
  // instead of throwing, and still refuses a genuinely foreign one
  { const G = buildLevel(); const lit = G.switchables[0].boxes[0]; G.boxes[lit].flags |= 32 /* AreaLit */; let threw = ''; let out: any = null;
    try { out = levelToData(G, { name: 'live', materials: D0.materials }); } catch (e: any) { threw = String(e.message ?? e); }
    verdict(!threw && !!out && !(out.boxes[lit].flags ?? []).includes('AreaLit'), 'levelToData off a game-touched Level: AreaLit masked, no throw', threw || `box ${out?.boxes[lit].id} flags ${JSON.stringify(out?.boxes[lit].flags ?? [])}`);
    G.boxes[lit].flags |= 1 << 16 /* SoundSeal: never on a level box */; threw = ''; try { levelToData(G, { name: 'live' }); } catch (e: any) { threw = String(e.message ?? e); }
    verdict(/runtime-only/.test(threw), 'but a foreign bit (SoundSeal on a static) still throws', threw.split('\n')[0]); }
  // the serializer refuses what JSON cannot hold rather than writing a file JSON.parse rejects
  { let threw = ''; try { const d = JSON.parse(JSON.stringify(D0)); d.routes[0].wait[1] = undefined; serializeLevelData(d); } catch (e: any) { threw = String(e.message ?? e); }
    verdict(/unwritable array element/.test(threw), 'serializeLevelData throws on an undefined array slot instead of printing `undefined`', threw.slice(0, 90));
    threw = ''; try { const d = JSON.parse(JSON.stringify(D0)); delete d.routes[1].wait[2]; serializeLevelData(d); } catch (e: any) { threw = String(e.message ?? e); }
    verdict(/unwritable array element \[2\]/.test(threw), 'and on an array HOLE (delete wait[2]) instead of printing `[0, , 1.5]`', threw.slice(0, 90)); }
  // id helpers the editor lane builds on
  { const ix = indexLevelData(D0); const okIx = ix.boxes.get(D0.breaker.panel) === L.breaker.boxes[0] && ix.boxes.get(D0.lights[0].fixture) === L.lights[0].fixtureBox && ix.doors.get('server') === D0.doors.findIndex((d: any) => d.id === 'server');
    verdict(okIx, 'indexLevelData: ids resolve to the indices loadLevel used (breaker panel, a fixture, a door)');
    const nid = freshId('box', D0.boxes); verdict(nid === `box_${String(D0.boxes.length).padStart(4, '0')}` || !ix.boxes.has(nid), `freshId('box') → ${nid} (unused)`); }
  // sizes, for the record
  const text = serializeLevelData(D0);
  console.log(`  office.json: ${(text.length / 1024).toFixed(1)} KB, ${text.split('\n').length - 1} lines · ${D0.boxes.length} boxes (${D0.boxes.filter((b: any) => b.material).length} by material, ${D0.boxes.filter((b: any) => b.albedo).length} inline) · ${Object.keys(D0.materials).length} materials · ${new Set(D0.boxes.map((b: any) => b.group)).size} groups · ${D0.props.length} props (${D0.props.filter((p: any) => p.half || p.height).length} pinned footprints)`);
}

/** THE LEVEL LINT'S OWN CHECK (src/scene/lint.ts lintLevel: the design checks the schema validator does not make — reachability, sealing, budgets, overlaps,
 *  things inside other things), no game stood up: the office with its sidecar (levels/office.lint.json) is clean; without the sidecar the only findings are the
 *  two the sidecar allows; each planted fault is reported with its own code and severity; an allow entry silences exactly its finding; the marking table's rows
 *  are tallied; and the CLI (tools/qa/level-lint.ts) exits 0 on the office and 1 on a file with an error. */
export async function levelLint() {
  console.log('\n=== level-lint: clean on the office, trips on planted faults, the marking table\'s rows, the CLI ===');
  const r0 = lintLevel(OFFICE_DATA, { allow: officeAllow });
  verdict(r0.errors === 0 && r0.warnings === 0, `office + sidecar: ${formatLint(r0)[0]}`);
  const rNo = lintLevel(OFFICE_DATA); verdict(rNo.errors === 0 && rNo.findings.every((f: any) => f.code === 'door-seal' && f.ids[0] === 'door:conference'), 'without the sidecar the only findings are the conference door\'s two door-seal warnings (its glazing is unglazed — documented in levels/office.lint.json)', rNo.findings.map((f: any) => f.code).join(' '));
  verdict(r0.ms < 250, `lint runs in ${r0.ms.toFixed(0)} ms (nav bake included)`);
  const base = JSON.stringify(OFFICE_DATA);
  const plant = (name: string, mut: (d: any) => void, code: string, sev = 'error') => { const d = JSON.parse(base); mut(d); const r = lintLevel(d, { allow: officeAllow }); const hit = r.findings.find((f: any) => f.code === code && f.severity === sev); verdict(!!hit, `planted: ${name} → ${sev} ${code}`, hit ? hit.message.slice(0, 110) : `NOT reported (${r.findings.slice(0, 3).map((f: any) => f.code).join(' ')})`); };
  plant('a light moved inside the corridor wall', d => { d.lights[1].pos = [20, 1.5, 10.0]; }, 'light-in-solid');
  plant('a light with range 0', d => { d.lights[0].range = 0; }, 'light-range');
  plant('a 5000-candela desk lamp', d => { d.lights[0].intensity = 5000; }, 'light-intensity', 'warn');
  plant('a spot with a 0° cone', d => { d.lights[0].outerDeg = 0; d.lights[0].innerDeg = 0; }, 'light-cone', 'warn');
  plant('a box with zero height', d => { d.boxes[300].h = [0.5, 0, 0.5]; }, 'box-extent');
  plant('an exact duplicate slab', d => { d.boxes.push({ ...d.boxes[400], id: 'box_dup' }); }, 'box-duplicate', 'warn');
  plant('a box poking out of the world', d => { d.boxes.push({ id: 'far', c: [45, 1, 14], h: [1, 1, 1], albedo: [1, 1, 1] }); }, 'box-bounds', 'warn');
  plant('the spawn inside the reception desk', d => { d.mission.playerSpawn = [8.2, 0, 15.2]; }, 'spawn-blocked');
  plant('every doorway into the server room walled up', d => { d.boxes.push({ id: 'blk1', c: [17.6, 1.1, 10.0], h: [0.7, 1.1, 0.06], material: 'wall' }); d.boxes.push({ id: 'blk2', c: [22.0, 1.1, 8.55], h: [0.06, 1.1, 0.6], material: 'wall' }); }, 'nav-objective');
  plant('… which also makes the server door itself unwalkable (its sides still meet through the manager\'s office: a warning)', d => { d.boxes.push({ id: 'blk1', c: [17.6, 1.1, 10.0], h: [0.7, 1.1, 0.06], material: 'wall' }); }, 'door-sides', 'warn');
  plant('both of a room\'s doors walled → its doors\' sides are disconnected (an error)', d => { d.boxes.push({ id: 'blk1', c: [17.6, 1.1, 10.0], h: [0.7, 1.1, 0.06], material: 'wall' }); d.boxes.push({ id: 'blk2', c: [22.0, 1.1, 8.55], h: [0.06, 1.1, 0.6], material: 'wall' }); }, 'door-sides');
  plant('a cabinet left standing in the storage doorway', d => { d.boxes.push({ id: 'cab', c: [32.6, 0.9, 10.0], h: [0.25, 0.9, 0.05], material: 'metal' }); }, 'door-gap', 'warn');
  plant('a door moved into the open lobby (nothing to seal against)', d => { d.doors[4].hinge = [7, 18]; }, 'door-seal', 'warn');
  plant('a patrol point inside a wall', d => { d.routes[0].points[1] = [20, 0, 10.0]; }, 'route-solid', 'warn');
  plant('a patrol leg to a sealed-off spot', d => { d.routes[1].points[2] = [38.5, 0, 2]; d.boxes.push({ id: 'pen1', c: [38.6, 1, 3.2], h: [1.45, 1, 0.06], material: 'wall' }, { id: 'pen2', c: [37.2, 1, 1.6], h: [0.06, 1, 1.7], material: 'wall' }); }, 'route-unreachable');   // (a pen in the lot's north-east corner, walls to the world's edge)
  plant('1700 boxes', d => { for (let i = 0; i < 760; i++) d.boxes.push({ id: 'bb' + i, c: [1 + (i % 30) * 0.05, 5, 1 + Math.floor(i / 30) * 0.05], h: [0.01, 0.01, 0.01], albedo: [1, 1, 1] }); }, 'box-budget', 'warn');
  plant('30 analytic lights', d => { for (let i = 0; i < 26; i++) d.lights.push({ ...d.lights[0], id: 'll' + i, pos: [10 + i * 0.9, 2.5, 20.5], fixture: undefined }); }, 'light-budget', 'warn');
  plant('a switchable that glows black', d => { d.switchables[3].baseEmissive = [0, 0, 0]; }, 'switchable-dark', 'warn');
  { const d = JSON.parse(base); const r = lintLevel(d, { allow: officeAllow }); const vend = d.switchables.find((s: any) => s.id === 'vending'); verdict(vend && !(vend.baseEmissive[0] + vend.baseEmissive[1] + vend.baseEmissive[2] > 0) && !r.findings.some((f: any) => f.code === 'switchable-dark'), 'a switchable lit through its boxes\' EMISSIVE MARKINGS (the vending front: baseEmissive black by construction) is not switchable-dark'); }
  plant('… but strip that marking\'s emission and it is dark again', d => { const b = d.boxes.find((x: any) => x.id === 'sw_vending'); for (const m of b.markings) for (const p of m.pal) delete p.emit; }, 'switchable-dark', 'warn');
  plant('a keypad buried in the wall it hangs on', d => { d.devices.find((x: any) => x.id === 'pad_side').pos = [22.0, 1.4, 9.36]; }, 'keypad-in-solid', 'warn');
  plant('a hole in the lobby floor (the stone finish and the base carpet moved away)', d => { for (const b of d.boxes) if (b.group === 'floors' || b.group === 'ground') { b.c = [b.c[0] + 100, b.c[1], b.c[2]]; } }, 'floor-hole', 'warn');
  plant('a schema error (unknown box in a switchable) is reported by lint too, as schema', d => { d.switchables[0].boxes[0] = 'nope'; }, 'schema');
  // allow-list mechanics
  { const d = JSON.parse(base); d.lights[0].intensity = 5000; const r = lintLevel(d, { allow: { ...officeAllow, 'light-intensity': { ids: ['light:desk_lamp'], reason: 'probe' } } }); verdict(!r.findings.some((f: any) => f.code === 'light-intensity'), 'an allow entry for (code, id) silences exactly that finding'); const r2 = lintLevel(d, { allow: { 'light-intensity': { ids: ['light:storage_bulb'], reason: 'probe' } } }); verdict(r2.findings.some((f: any) => f.code === 'light-intensity'), '… and not the same code on another id'); }
  // the marking table's rows (stats.markRows; a warning, then an error, as a level nears and passes the table's size)
  { verdict(typeof r0.stats.markRows === 'number' && r0.stats.markRows > 100 && r0.stats.markRows < 6144 && !r0.findings.some((f: any) => f.code === 'mark-rows'), `office: ${r0.stats.markRows} marking-table rows, no mark-rows finding`);
    const d = JSON.parse(base); const src = d.boxes.find((b: any) => b.markings?.length && b.markings[0].size[0] * b.markings[0].size[1] >= 40);
    for (let i = 0; i < 400; i++) { const nb = { ...JSON.parse(JSON.stringify(src)), id: `mrow${i}`, c: [1 + (i % 20) * 0.6, 6.5, 1 + Math.floor(i / 20) * 0.4], albedo: [0.3 + i * 0.001, 0.3, 0.3] }; delete nb.material; delete nb.name; d.boxes.push(nb); }   // 400 DISTINCT lists (albedo differs) of a big bitmap
    const r2 = lintLevel(d, { allow: officeAllow }); const f2 = r2.findings.find((f: any) => f.code === 'mark-rows');
    verdict(!!f2 && (f2.severity === 'error' || f2.severity === 'warn') && (r2.stats.markRows as number) > 6144 * 0.75, `+400 distinct big lists: ${r2.stats.markRows} rows → mark-rows ${f2?.severity}`, f2?.message.slice(0, 100)); }
  // the CLI: exit 0 on the office, 1 on a file with an error, honours the sidecar
  { const dir = mkdtempSync(join(tmpdir(), 'tps-lint-')); try {
      const good = join(dir, 'good.json'); writeFileSync(good, serializeLevelData(OFFICE_DATA)); writeFileSync(join(dir, 'good.lint.json'), JSON.stringify({ allow: officeAllow }));
      const bad = join(dir, 'bad.json'); const bd = JSON.parse(base); bd.lights[0].range = 0; writeFileSync(bad, serializeLevelData(bd));
      const run = (f: string) => Bun.spawnSync({ cmd: [process.execPath, 'run', join(ROOT, 'tools/qa/level-lint.ts'), f, '--quiet'], cwd: ROOT, stdout: 'pipe', stderr: 'pipe' });
      const a = run(good), b2 = run(bad); const ao = new TextDecoder().decode(a.stdout), bo = new TextDecoder().decode(b2.stdout);
      verdict(a.exitCode === 0 && /PASS \(0 warnings\)/.test(ao), 'CLI on the office (with sidecar) → exit 0, "PASS (0 warnings)"', ao.trim().split('\n').pop());
      verdict(b2.exitCode === 1 && /light-range/.test(bo) && /FAIL \(1 error\)/.test(bo), 'CLI on a level with a range-0 light → exit 1, the finding printed', bo.trim().split('\n').slice(-2).join(' | ').slice(0, 160));
    } finally { rmSync(dir, { recursive: true, force: true }); } }
}

/** THE DEV SERVER'S LEVEL FILES (tools/levels-api.ts, what `?levelFile=<name>.json` fetches on localhost), on a scratch folder: the index, a file, its lint
 *  sidecar, and what it refuses (a name that is a path, anything but a read). */
export async function levelServe() {
  console.log('\n=== level-serve: the dev server\'s /levels/ (index, a file, its sidecar, the refusals) ===');
  const { handleLevels, NAME_RE } = await import(`${ROOT}/tools/levels-api.ts`);
  const dir = mkdtempSync(join(tmpdir(), 'tps-levels-')); const text = serializeLevelData(OFFICE_DATA);
  const req = (path: string, init?: RequestInit) => new Request(`http://localhost:5999${path}`, init);
  try {
    let r = await handleLevels(req('/levels/'), dir); verdict(r.status === 200 && (await r.json()).levels.length === 0, 'GET /levels/ with no levels folder → 200, an empty list');
    r = await handleLevels(req('/not-levels'), dir); verdict(r === null, 'a path that is not /levels → null (the server\'s other routes answer)');
    mkdirSync(join(dir, 'levels')); writeFileSync(join(dir, 'levels', 'probe_office.json'), text); writeFileSync(join(dir, 'levels', 'probe_office.lint.json'), JSON.stringify({ allow: {} })); writeFileSync(join(dir, 'levels', 'notes.txt'), 'x');
    r = await handleLevels(req('/levels/'), dir); const idx = await r.json(); verdict(idx.levels.length === 1 && idx.levels[0].name === 'probe_office' && idx.levels[0].bytes === Buffer.byteLength(text), 'GET /levels/ lists the level by name and size, and neither its sidecar nor a file that is not a level');
    r = await handleLevels(req('/levels/probe_office.json'), dir); verdict(r.status === 200 && (await r.text()) === text, 'GET /levels/probe_office.json → the file, byte for byte');
    r = await handleLevels(req('/levels/probe_office.json', { method: 'HEAD' }), dir); verdict(r.status === 200 && (await r.text()) === '', 'HEAD → 200 with no body');
    r = await handleLevels(req('/levels/probe_office.lint.json'), dir); verdict(r.status === 200 && JSON.parse(await r.text()).allow !== undefined, 'GET its lint sidecar → the sidecar');
    r = await handleLevels(req('/levels/nothing.json'), dir); const r2 = await handleLevels(req('/levels/nothing.lint.json'), dir); verdict(r.status === 404 && r2.status === 404, 'a level, or a sidecar, that is not there → 404');
    const bad = ['../x.json', 'a/b.json', 'x.txt', '.json', 'x..json', 'x.lint.json/y', encodeURIComponent('../package') + '.json']; const got: number[] = [];
    for (const b of bad) got.push((await handleLevels(req(`/levels/${b}`), dir))?.status ?? 0);
    verdict(got[0] === 0 && got.slice(1).every(s => s === 400) && !NAME_RE.test('a.b') && !NAME_RE.test('../x') && NAME_RE.test('office-relief'), `a name that is a path, has a dot or is not .json is refused, and ../ is no longer a /levels path at all (${got.join(' ')})`);
    r = await handleLevels(req('/levels/probe_office.json', { method: 'POST', headers: { 'Content-Type': 'application/json' }, body: '{}' }), dir); const d = await handleLevels(req('/levels/probe_office.json', { method: 'DELETE' }), dir);
    verdict(r.status === 405 && d.status === 405 && readFileSync(join(dir, 'levels', 'probe_office.json'), 'utf8') === text, 'POST and DELETE → 405, and the file is as it was: nothing writes through the server');
    r = await handleLevels(req('/levels/probe_office.json', { headers: { host: '192.168.1.20:5173' } }), dir); verdict(r.status === 200, 'a page opened by the machine\'s own address (HOST=0.0.0.0) is answered as localhost is: the files are the repo\'s own');
  } finally { rmSync(dir, { recursive: true, force: true }); }
}
