// THE KIT PICTURE'S PLAN AND ITS SET, TWO WAYS: src/host/kitViewPlan.ts + kitViewSet.ts against rust/game_start's kit_plan.rs + kit_set.rs, on the game's own maths
// (src/math/exact.ts on, which rust/sim holds to the bit). `bun run tools/blitz/kitplan-diff.ts`
//   exit 0 nothing differs · 1 a difference (the first 10 by path) · 2 the bin could not run
// Every level file that parses and has a kitView block, every way of it as written and as a dozen variants the files do not write (a span, a written wall, places in the leaf's
// swing, added pieces, a pane by name …), and the pure helpers on a sweep of links. Numbers by Object.is: -0 is not 0. The requests are made here and the bin answers them
// (kitplan --ask), so the two sides cannot ask different things. KITPLAN_DIFF_BREAK=1 moves one number on the TypeScript's side, and the compare must fail.
import { readFileSync, readdirSync, writeFileSync, mkdtempSync, rmSync } from 'node:fs';
import { tmpdir, homedir } from 'node:os';
import { join } from 'node:path';
import { EXACT } from '../../src/math/exact.ts';
EXACT.on = true;
const { kitViewPlanWay, kitViewPlanChild, kitPlanGeometry, kitPlanFloor, kitPlanSlots, kitPlanAct4, kitPlanLitInside, kitPlanPorchLit, kitPlanOwnLamps, kitPlanDayOut, kitPlanCovered } = await import('../../src/host/kitViewPlan.ts');
const { kitViewWayOfQuery, kitViewWayOfLink, kitViewSetOf, kitViewSetQuery, kitViewSetDress, kitSetPaint, kitViewAddLevel, kitViewSetLevel, kitViewQuietKind } = await import('../../src/host/kitViewSet.ts');

/* eslint-disable @typescript-eslint/no-explicit-any */
type Data = any;
const ROOT = join(import.meta.dir, '../..'), BREAK = process.env.KITPLAN_DIFF_BREAK === '1';
const tmp = mkdtempSync(join(tmpdir(), 'kitplan-'));
const cases: { req: Data; ts: () => Data }[] = [];
let plans = 0, made = 0;
const ask = (req: Data, ts: () => Data) => { cases.push({ req, ts }); };

const levelOf = new Map<string, Data>();
const load = (path: string): Data => { let d = levelOf.get(path); if (!d) { d = JSON.parse(readFileSync(path, 'utf8')); levelOf.set(path, d); } return d; };
const spawnOf = (d: Data): [number, number] => [d.mission.playerSpawn[0], d.mission.playerSpawn[2]];

const planCase = (path: string, way: Data, extra: { hiddenStand?: boolean; alsoHide?: string[]; left?: string[] } = {}) => {
  plans++;
  const req = { op: 'plan', level: path, ...(typeof way === 'number' ? { way } : { y: way }), ...extra };
  ask(req, () => { const d = load(path), y = typeof way === 'number' ? d.scenarios.kitView.ways[way] : way, { left, ...opts } = extra, L = left ? { ...d, left } : d;
    const P = kitViewPlanWay(L, d.scenarios?.kitView, y, spawnOf(d), opts); if (BREAK) P.slots[0][0] += 1e-9; return P; });
};
const childCase = (path: string, way: Data) => {
  plans++;
  ask({ op: 'child', level: path, ...(typeof way === 'number' ? { way } : { y: way }) }, () => { const d = load(path), y = typeof way === 'number' ? d.scenarios.kitView.ways[way] : way, G = kitPlanGeometry(d, d.scenarios?.kitView, y, spawnOf(d)); if (!G) return null;
    const F = kitPlanFloor(G.level.boxes), sunk: number[][] = []; const plan = kitViewPlanChild(G.level, y, G.ref, (x: number, z: number) => !F.blocked(x, z), (h: readonly number[]) => { sunk.push([...h]); }); return { plan: { ...plan, set: null }, sunk }; });
};
/** what a level-making function answers: null where it handed back the very level */
const whole = (r: Data, d: Data) => { if (r === d) return null; made++; return r; };

const WAY_WORDS = [null, '', '1', '2', '3', '9', 'any', '0', '1x'];
const KINDS = ['roomclear', 'nightbreach', 'rescue', 'quiet', 'solo', 'squad4', ''];
const SQUADS = ['', 'squad=0', 'squad=1', 'squad=2', 'squad=4', 'squad=5', 'squad=%202', 'squad4'];
const links: string[] = []; for (const k of KINDS) for (const w of ['', 'way=2', 'way=any']) for (const s of SQUADS) links.push('?' + [k, w, s, 'hour=dusk+late'].filter(x => x).join('&'));

const files = readdirSync(`${ROOT}/levels`).filter(f => f.endsWith('.json')).sort();
const blocks: string[] = [];
for (const f of files) { const path = `${ROOT}/levels/${f}`; let d: Data; try { d = load(path); } catch { continue; } if (d?.scenarios?.kitView?.ways && Array.isArray(d.boxes) && d.mission?.playerSpawn) blocks.push(path); }

for (const path of blocks) {
  const d = load(path), K = d.scenarios.kitView, spawn = spawnOf(d);
  ask({ op: 'floor', level: path }, () => { const F = kitPlanFloor(d.boxes), g: string[] = []; for (let z = -1; z <= 56; z++) for (let x = -1; x <= 80; x++) g.push(F.blocked((x + 0.5) * 0.5, (z + 0.5) * 0.5) ? '1' : '0'); return { solid: F.solid, grid: g.join('') }; });
  for (const q of WAY_WORDS) ask({ op: 'way_of_query', level: path, ...(q === null ? {} : { q }) }, () => kitViewWayOfQuery(q, d));
  for (const l of links.slice(0, 12)) ask({ op: 'way_of_link', level: path, link: l }, () => kitViewWayOfLink(new URLSearchParams(l), d));
  for (const [x, z, y] of [[10, 10, 2], [20.25, 14.5, 2.6], [spawn[0], spawn[1], 1.5], [spawn[0], spawn[1], 3.2]]) ask({ op: 'covered', level: path, x, z, y }, () => kitPlanCovered(d.boxes, x, z, y));
  K.ways.forEach((way: Data, i: number) => {
    planCase(path, i); childCase(path, i);
    ask({ op: 'set_of', level: path, way: i }, () => kitViewSetOf(d, i)?.name ?? null);
    ask({ op: 'set_level', level: path, way: i }, () => whole(kitViewSetLevel(d, i), d));
    ask({ op: 'add_level', level: path, way: i }, () => whole(kitViewAddLevel(d, i), d));
    const P = kitViewPlanWay(d, K, way, spawn), G = kitPlanGeometry(d, K, way, spawn)!, B = G.level.boxes as Data[], A = P.anchor, set = way.set !== undefined ? { set: way.set } : {};
    const on = (al: number, o: number): [number, number] => [A.mid[0] + P.s[0] * al + A.n[0] * o, A.mid[1] + P.s[1] * al + A.n[1] * o], hw = A.width / 2;
    const group: string | undefined = B.find(b => b.group)?.group, glass = B.find(b => b.flags?.includes('Glass'))?.id, lamp = (G.level.lights ?? []).find((l: Data) => l.kind !== 'dir');
    const three = (a: [number, number], y: number): [number, number, number] => [a[0], a[1], y];
    const stack1 = on(hw + 0.45, P.wall.face + 0.7);
    const variants: Data[] = [
      { door: way.door, ...set },
      { ...way, side: -P.side },
      { ...way, hide: undefined, pin: undefined },
      { ...set, span: [on(hw + 0.6, 0), on(hw + 2.4, 0)] },
      { ...set, span: [on(-hw, 0), on(hw, 0)] },
      { ...set, span: [on(-hw - 1, 0), on(hw + 1, 0)], kind: 'gap', wall: { top: 3 } },
      { ...set, span: [on(-hw, 0), on(hw, 0)], kind: 'wall' },
      { ...set, span: [on(hw + 0.6, 0), on(hw + 2.4, 0)], kind: 'gap' },
      { ...set, span: [on(-hw, 0), on(hw, 0)], kind: 'door' },
      { ...set, span: [on(0, 0), on(0.1, 0)] },
      { ...set, door: way.door, wall: { face: 0.2, thickness: 0.3, top: 1.7 }, lane: 0.9, out: 4.5, inside: 1, indoors: true, hand: 'right', keepLights: ['b', 'a'], ref: on(0, -3), pin: { yaw: 0.5, pitch: 0.9, dist: 7 }, pane: false,
        light: { porch: { offset: [0.1, 0.2, 2], aim: [0, 1, 0], fixture: { at: [0, 0.1, 1.9] } }, fill: { offset: [1, 5, 2], aim: [1, 1, 1] } } },
      { ...set, door: way.door, slots: [null, three(on(hw - 0.2, 1), 90), three(on(hw + 0.1, 0.4), 90), three(on(-hw - 0.5, 1), 90), three(on(hw + 3, 1), 0)] },
      { ...set, door: way.door, slots: [three(on(-hw - 1, 2), 45), three(stack1, 10), three(on(hw + 0.45 + 0.8, 0.7), 20), three(on(hw + 0.45 + 1.6, 0.7), 30)] },
      { ...set, door: way.door, slots: [null, three(on(-hw + 0.1, 0.5), 90), null, three(on(-hw - 0.1, 0.5), 90)], slotGap: 0.9 },
      { ...way, litInside: true, dayLights: ['kp_none', ...(lamp ? [lamp.id, lamp.group] : [])], hideMore: [...(group ? [group] : []), 'kp_none', 'kp_none'], show: [...(group ? [group] : []), 'kp_shown'], hidePaint: group ? [group] : [], hide: undefined },
      { ...way, hide: [...(group ? [group, group] : []), B[0].id], show: [B[0].id] },
      { ...way, pane: { box: glass ?? 'kp_none' } }, { ...way, pane: { box: glass ?? 'kp_none', stand: [1.5, 2.5, 30] } }, { ...way, pane: { box: B[0].id } },
      ...(way.set === undefined ? [
        { ...way, add: { boxes: [{ id: 'kp_crate', c: [stack1[0], 0.5, stack1[1]], h: [0.4, 0.5, 0.4], albedo: [1, 1, 1] }, { id: 'kp_high', c: [(P.cam.pos[0] + P.slots[0][0]) / 2, 2, (P.cam.pos[2] + P.slots[0][1]) / 2], h: [0.5, 0.5, 0.5], yaw: 0.3, albedo: [1, 1, 1] }, { ...B[0] }],
          lamps: [{ id: 'kp_lamp', kind: 'point', pos: [A.mid[0], 2, A.mid[1]], dir: [0, -1, 0], color: [1, 1, 1], intensity: 1, range: 5, innerDeg: 30, outerDeg: 40, group: 'kp_group', enabled: true, volumetric: 0 }] }, dayLights: ['kp_lamp', 'kp_none'] },
        { ...way, add: { boxes: [{ id: 'kp_crate', c: [stack1[0], 0.5, stack1[1]], h: [0.4, 0.5, 0.4], albedo: [1, 1, 1] }], laid: true } },
        { ...way, show: ['kp_high'], add: { boxes: [{ id: 'kp_high', c: [(P.cam.pos[0] + P.slots[0][0]) / 2, 2, (P.cam.pos[2] + P.slots[0][1]) / 2], h: [0.5, 0.5, 0.5], albedo: [1, 1, 1] }] } },
      ] : []),
      { door: 'kp_no_door' }, {}, { ...way, set: 'kp_no_set' },
    ];
    for (const y0 of variants) { const y = JSON.parse(JSON.stringify(y0)); planCase(path, y); if (variants.indexOf(y0) % 4 === 0) childCase(path, y); }
    planCase(path, i, { hiddenStand: true }); planCase(path, i, { alsoHide: [...(group ? [group] : []), B[0].id, B[B.length - 1].id] }); planCase(path, { ...way, hide: ['kp_left.a'] }, { left: ['kp_left', 'other'] });
    const a4 = { mid: A.mid, s: P.s, n: A.n, width: A.width, slots: P.slots };
    ask({ op: 'act4', level: path, ...a4 }, () => kitPlanAct4(A.mid, P.s, A.n, A.width, P.slots));
    ask({ op: 'act4', level: path, ...a4, useFree: true, width: 4 }, () => { const F = kitPlanFloor(d.boxes); return kitPlanAct4(A.mid, P.s, A.n, 4, P.slots, (x: number, z: number) => !F.blocked(x, z)); });
    ask({ op: 'act4', level: path, ...a4, clear: 0 }, () => kitPlanAct4(A.mid, P.s, A.n, A.width, P.slots, undefined, 0));
    ask({ op: 'act4', level: path, ...a4, clear: 3 }, () => kitPlanAct4(A.mid, P.s, A.n, A.width, P.slots, undefined, 3));
    ask({ op: 'slots', level: path, a: A.a, t: A.t, width: A.width, n: A.n, side: 1, off: 0.82, samOff: 1.2 }, () => { const F = kitPlanFloor(d.boxes); return kitPlanSlots(A.a, A.t, A.width, A.n, 1, 0.82, 1.2, (x: number, z: number) => !F.blocked(x, z)); });
    ask({ op: 'slots', level: path, a: A.a, t: A.t, width: A.width, n: A.n, side: -1, off: 0.82, samOff: 1.2 }, () => { const F = kitPlanFloor(d.boxes); return kitPlanSlots(A.a, A.t, A.width, A.n, -1, 0.82, 1.2, (x: number, z: number) => !F.blocked(x, z)); });
    ask({ op: 'lit_inside', level: path, mid: A.mid, n: A.n, back: P.wall.back }, () => kitPlanLitInside(d.lights ?? [], A.mid, A.n, P.wall.back).map((l: Data) => l.id));
    const addY = { ...way, ...(way.set === undefined ? { add: { lamps: [{ id: 'kp_lamp', kind: 'point', pos: [A.mid[0], 2, A.mid[1]], range: 5, group: 'kp_group' }, { id: 'kp_under', kind: 'point', pos: [A.mid[0], 1, A.mid[1]], range: 5, group: 'exterior' }, { id: 'kp_dir', kind: 'dir', pos: [0, 9, 0], range: 5, group: 'sky' }] } } : {}), dayLights: lamp ? [lamp.group] : undefined };
    for (const lampsOut of [true, false]) for (const indoors of [true, false]) for (const yy of [way, JSON.parse(JSON.stringify(addY))]) ask({ op: 'day_out', level: path, y: yy, lampsOut, indoors }, () => { const G2 = kitPlanGeometry(d, K, yy, spawn); return G2 ? kitPlanDayOut(kitPlanOwnLamps(K, yy), G2.level.boxes, yy.dayLights, indoors, lampsOut) : null; });
  });
}
for (const l of links) { const q = () => new URLSearchParams(l);
  ask({ op: 'set_query', link: l }, () => [...kitViewSetQuery(q()).entries()]);
  ask({ op: 'set_dress', link: l }, () => kitViewSetDress(q()));
  ask({ op: 'quiet_kind', link: l }, () => kitViewQuietKind(q())); }
for (const lit of [0, 3.5]) for (const day of [undefined, 0, 1.25]) for (const indoors of [true, false]) for (const lampsOut of [true, false]) ask({ op: 'porch_lit', lit, ...(day === undefined ? {} : { day }), indoors, lampsOut }, () => kitPlanPorchLit(lit, day, indoors, lampsOut));
{ const materials = { b: {}, c: {} }; for (const material of ['a|b|c', 'x|y', 'b', 'c|b', undefined]) { const b = { id: 'q', c: [1, 2, 3], h: [1, 1, 1], ...(material ? { material } : {}) }; ask({ op: 'paint', b, materials }, () => kitSetPaint(b as Data, materials as Data)); } }

// HAND-MADE FILES for what no level writes: doors of every swing and stop, loose props of a group, two ways that add one id, a set with no leaf
{ const path = blocks.find(p => { const d = load(p); return d.doors?.length && d.scenarios.kitView.ways[0]?.set === undefined; })!, d = load(path);
  const write = (name: string, o: Data) => { const p = join(tmp, name); writeFileSync(p, JSON.stringify(o)); return p; }, way0 = d.scenarios.kitView.ways[0], P0 = kitViewPlanWay(d, d.scenarios.kitView, way0, spawnOf(d)), A = P0.anchor;
  const near = (x: number, o: number): [number, number, number] => [A.mid[0] + P0.s[0] * x + A.n[0] * o, A.mid[1] + P0.s[1] * x + A.n[1] * o, 90];
  const swing = (door: Data) => { const o = JSON.parse(JSON.stringify(d)); o.doors = o.doors.map((x: Data) => x.id === way0.door ? { ...x, ...door } : x); return o; };
  for (const [i, door] of [{ swing: 1 }, { swing: -1 }, { maxAngle: 0 }, { minAngle: 0 }, { maxAngle: 1, minAngle: -1 }, { steel: true, locked: true, exterior: true }, { minAngle: -3, maxAngle: 3, swing: 1 }].entries()) { const p = write(`door${i}.json`, swing(door));
    for (const hand of [-0.6, 0, 0.3]) for (const o of [0.3, -0.3]) { const hw = A.width / 2; planCase(p, { ...way0, slots: [null, near(hw * hand, o * 2), near(-hw - 0.1, 0.5), near(hw * hand, o)] }); }
    for (const [al, o] of [[0.05, 0.4], [0.1, 0.1], [0.2, 0.6], [0.02, A.width - 0.1]]) { const at: [number, number, number] = [A.a[0] - A.t[0] * al + A.n[0] * o, A.a[1] - A.t[1] * al + A.n[1] * o, 90]; planCase(p, { ...way0, side: -1, slots: [null, at] }); planCase(p, { ...way0, side: -1, slots: [null, null, at] }); } }
  const props = JSON.parse(JSON.stringify(d)), grp = d.boxes.find((b: Data) => b.group && b.c[1] + b.h[1] > 0.25)!; props.props = [...(props.props ?? []), { id: grp.group, kind: 'chair', x: grp.c[0], z: grp.c[2] }, { id: `${grp.group}.stool`, kind: 'chair', x: grp.c[0] + 0.3, z: grp.c[2] }, { id: `${grp.group}x.stool`, kind: 'chair', x: grp.c[0], z: grp.c[2] }, { id: `${grp.group}.far`, kind: 'chair', x: grp.c[0] + 30, z: grp.c[2] }];
  const pp = write('props.json', props); for (const y of [way0, { ...way0, hide: [grp.group] }, { ...way0, hide: undefined, hideMore: [grp.id] }, { ...way0, hide: [], show: [`${grp.group}.stool`] }, { door: way0.door, hide: undefined, show: [grp.group] }]) planCase(pp, JSON.parse(JSON.stringify(y)));
  const crate = (id: string, x: number) => ({ id, c: [A.mid[0] + x, 0.5, A.mid[1]], h: [0.3, 0.5, 0.3], material: 'kp_a|kp_b' });
  const twos = JSON.parse(JSON.stringify(d)); twos.materials = { ...twos.materials, kp_b: { albedo: [1, 1, 1] } }; delete twos.lights; twos.lights = d.lights;
  twos.scenarios.kitView.ways = [{ ...way0, add: { boxes: [crate('kp_one', 3), { ...d.boxes[0] }], lamps: [{ id: 'kp_lamp', kind: 'point', pos: [1, 2, 3], range: 5, group: 'kp' }] } }, { ...way0, add: { boxes: [crate('kp_one', 4)] } }, way0, { ...way0, add: { boxes: [{ ...d.boxes[1] }] } }, { ...way0, add: { laid: true, boxes: [crate('kp_two', 4)] } }];
  const pt = write('twos.json', twos); for (let i = 0; i < 6; i++) { ask({ op: 'add_level', level: pt, way: i }, () => whole(kitViewAddLevel(twos, i), twos)); ask({ op: 'set_level', level: pt, way: i }, () => whole(kitViewSetLevel(twos, i), twos)); }
  const set = { boxes: [{ id: 'g', group: 'ground', c: [20, -0.1, 14], h: [18, 0.1, 12], albedo: [1, 1, 1] }, { id: 'w1', group: 'wall', c: [20, 1.5, 10], h: [3, 1.5, 0.12], material: 'kp_a|kp_b' }, { id: 'w2', group: 'wall', c: [28, 1.5, 10], h: [3, 1.5, 0.12], albedo: [1, 1, 1] }, { id: 'pane', c: [17.5, 1.4, 10], h: [0.5, 0.5, 0.02], albedo: [1, 1, 1], flags: ['Glass'] }, { id: 'roof', c: [20, 3.1, 14], h: [4, 0.1, 4], albedo: [1, 1, 1] }], lights: [{ id: 'gate', kind: 'point', pos: [24, 2.4, 11], range: 6, group: 'gate' }, { id: 'in', kind: 'point', pos: [24, 2.4, 8], range: 6, group: 'inner' }], ref: [24, 16] };
  const sets = JSON.parse(JSON.stringify(d)); sets.daylight = sets.daylight ?? { lightsOff: [] }; sets.daylight.lightsOff = [...sets.daylight.lightsOff, 'gate', 'kp_gone']; sets.surfaces = sets.surfaces ?? { a: 1 };
  sets.scenarios.kitView = { sets: { gate: set, door: { ...set, doors: [{ id: 'z', hinge: [8, 8], closedDir: 0, width: 0.9 }, { id: 'd', hinge: [23.5, 10], closedDir: 0, width: 1 }] }, tight: { boxes: [{ id: 'c1', c: [24, 1, 14], h: [40, 1, 40], albedo: [1, 1, 1] }], ref: [24, 16] } }, ways: [{ set: 'gate', span: [[23, 10], [25, 10]] }, { set: 'gate', span: [[20, 10], [22, 10]], kind: 'wall', litInside: true, pane: { box: 'pane' } }, { set: 'door', door: 'd' }, way0, { set: 'tight', span: [[23, 10], [25, 10]] }, { set: 'kp_none' }] };
  const ps = write('sets.json', sets); for (let i = 0; i < 6; i++) { planCase(ps, i); childCase(ps, i); ask({ op: 'set_level', level: ps, way: i }, () => whole(kitViewSetLevel(sets, i), sets)); ask({ op: 'add_level', level: ps, way: i }, () => whole(kitViewAddLevel(sets, i), sets)); ask({ op: 'set_of', level: ps, way: i }, () => kitViewSetOf(sets, i)?.name ?? null); } }

const reqFile = join(tmp, 'requests.jsonl'); writeFileSync(reqFile, cases.map(c => JSON.stringify(c.req)).join('\n') + '\n');
const ansFile = join(tmp, 'answers.jsonl');
const run = Bun.spawnSync(['sh', '-c', `cargo run -q --bin kitplan -- --ask "${reqFile}" > "${ansFile}"`], { cwd: `${ROOT}/rust/game_start`, env: { ...process.env, PATH: `${join(homedir(), '.cargo/bin')}:${process.env.PATH ?? ''}`, RUSTFLAGS: '-Awarnings' }, stderr: 'pipe' });
if (run.exitCode !== 0) { console.log(`kitplan-diff: the bin failed\n${run.stderr.toString().slice(-3000)}`); rmSync(tmp, { recursive: true }); process.exit(2); }
const answers = readFileSync(ansFile, 'utf8').split('\n'); answers.pop();
if (answers.length !== cases.length) { console.log(`kitplan-diff: ${cases.length} asked, ${answers.length} answered`); process.exit(2); }

const diffs: string[] = []; let differing = 0;
const short = (v: Data) => { const t = Object.is(v, -0) ? '-0' : JSON.stringify(v); return t === undefined ? 'undefined' : t.length > 90 ? t.slice(0, 90) + '…' : t; };
function cmp(t: Data, r: Data, at: string, out: string[]) {
  if (out.length > 6) return;
  if (typeof t === 'number' || typeof r === 'number') { if (!Object.is(t, r)) out.push(`${at}: ts ${short(t)} rust ${short(r)}`); return; }
  if (t === null || r === null || typeof t !== 'object' || typeof r !== 'object') { if ((t ?? null) !== (r ?? null)) out.push(`${at}: ts ${short(t)} rust ${short(r)}`); return; }
  if (Array.isArray(t) !== Array.isArray(r)) { out.push(`${at}: an array on one side`); return; }
  if (Array.isArray(t)) { if (t.length !== r.length) out.push(`${at}: length ts ${t.length} rust ${r.length}: ts ${short(t)} rust ${short(r)}`); for (let i = 0; i < Math.min(t.length, r.length); i++) cmp(t[i], r[i], `${at}[${i}]`, out); return; }
  for (const k of new Set([...Object.keys(t), ...Object.keys(r)])) { if (t[k] === undefined && !(k in r)) continue; if (r[k] === undefined && t[k] !== undefined) { out.push(`${at}.${k}: only in ts: ${short(t[k])}`); continue; } if (t[k] === undefined) { out.push(`${at}.${k}: only in rust: ${short(r[k])}`); continue; } cmp(t[k], r[k], `${at}.${k}`, out); }
}
const name = (q: Data) => { const { level, y, ...o } = q; return `${String(level ?? '').split('/').pop()} ${short(y ? { y } : o).slice(0, 110)}`; };
cases.forEach((c, i) => { let t: Data; const out: string[] = [];
  try { t = c.ts(); } catch (e) { t = { threw: String(e) }; }
  cmp(t, JSON.parse(answers[i]), '', out);
  if (out.length) { differing++; if (diffs.length < 10) diffs.push(`${name(c.req)}\n    ${out.slice(0, 4).join('\n    ')}`); } });
rmSync(tmp, { recursive: true });
// (KITPLAN_DIFF_WORDS=1: every kind of problem the sweep reached, how many times: a zero that never reached a row says nothing of that row)
if (process.env.KITPLAN_DIFF_WORDS === '1') { const seen = new Map<string, number>(); for (const a of answers) for (const w of a.matchAll(/"problems":\[(.*?)\],"set"/g)) for (const q of w[1].split('","')) { const k = q.replace(/'[^']*'/g, "'…'").replace(/-?\d+(\.\d+)?/g, '#').slice(0, 70); if (k) seen.set(k, (seen.get(k) ?? 0) + 1); } for (const [k, n] of [...seen].sort()) console.log(`${n}\t${k}`); }
if (differing) { console.log(diffs.join('\n')); console.log(`kitplan-diff: ${plans} plans, ${made} levels made, ${differing} differ (of ${cases.length})`); process.exit(1); }
console.log(`kitplan-diff: ${plans} plans, ${made} levels made, 0 differ`);
process.exit(0);
