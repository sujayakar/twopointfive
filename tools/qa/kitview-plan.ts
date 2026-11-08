#!/usr/bin/env bun
// THE KIT PICTURE'S BLOCK, CHECKED ON THE LEVEL FILES: `bun run tools/qa/kitview-plan.ts [level …] [--quiet]`. What a level's author runs after writing `scenarios.kitView` (docs/level-format.md). No browser
// and no game stood: each way is asked of the Rust planner in the game's own module (`face_kit_plan`, rust/game_start/src/kit_stage.rs plan_of_file: the plan as a start makes it, two passes, what is
// sunk for the camera taken off the floor). That the game stands on it is rust/game_start/tests/kit_levels.rs; what the men do there is tests/kit_stage.rs and kit_acts.rs.
//   B. THE RECORDS OFFICE, both ways: the plan asked of the file with the way's `side` and `slots` taken out agrees with what is written (the side, the hand, each place, the wall, the lamps); with its
//      `hide` out too, the parked car is among what stands in the camera's way on the front way, and nothing on the west.
//   C. EVERY LEVEL WITH A BLOCK, every way: the block passes the level validator; the plan has no problems; no piece of the building is sunk for the camera; five places on free floor and apart; the
//      anchor is a door of the level in a wall, and not steel (a row that works a leaf has none on a span and is refused by steel); where the level's missions start in more than one place the way writes
//      its `ref`; what a set or an `add` lays passes the validator; a man walks from Sam's place through the opening on the nav grid's cells.
//      NEVER A FAILURE: `- WARNING —` where a way names a box by its generated id (`box_NNNN`, its place in the file) · `note —` where a written `hide` takes a box in no line of the camera's, or leaves one
//      standing in them, or where a lamp behind the anchor goes out with the room lights.
//   C3. EVERY CARD OPEN, EVERY SQUAD SIZE: no two men within a body of each other, the fourth man's acting place on free floor.
//   D. A QUIET MISSION'S WAY hangs its pool lamp over a glowing box on a wall (no box, no fixture to shoot out).
//   E. THE WORD `laid` is the loader's and a file may not write it; a broken block does not refuse a level to play (parseLevelData `kitView: 'drop'`), and is refused everywhere else.
// Lines are `- PASS — …` / `- FAIL — …`. The first line printed names the levels that carry a block (nothing else lists them). Exit 0 = all pass · 1 = a row failed · 2 = an
// unknown level, or a module with no `face_kit_plan` (it is older than this tool: bun run tools/blitz/face-build.ts).
// KITPLAN_BREAK=1: what is asked is spoiled (the side worked out read flipped, a block's second place moved onto the wall's line, every way's door read as steel, the way in asked of a point past the floor, the fourth man's place on Sam's): rows of B, C and C3 must FAIL, exit 1.
import { readFileSync, readdirSync, existsSync } from 'node:fs';
import { resolve } from 'node:path';
import { faceHere, FACE_WASM } from '../blitz/face-build';
import { parseLevelData, validateLevelData } from '../levels/lib/levelData';
import { ALL_MISSIONS } from '../../src/scene/scenarioList';

/* eslint-disable @typescript-eslint/no-explicit-any */
type Data = any;
const ROOT = resolve(import.meta.dir, '../..');
const BREAK = process.env.KITPLAN_BREAK === '1';
/** metres: a worked-out stack place against the written one · Sam's line (Sam, the fourth man: laid by eye, off the worked-out line on purpose) · a lamp */
const TOL = { stack: 0.15, sam: 0.45, lamp: 0.01 };
/** the records office's parked car (tools/levels/records.ts CAR), by export id */
const CAR = ['box_0212', 'box_0213', 'box_0214', 'box_0215', 'box_0216', 'box_0217', 'box_0218', 'box_0219', 'box_0220', 'box_0221', 'box_0222'];
/** how the planner's problem "an occluder that is no thing" begins (rust/game_start/src/kit_plan.rs IN_WAY) */
const IN_WAY = "in the camera's way and not a thing to sink";

const argv = process.argv.slice(2), quiet = argv.includes('--quiet'), asked = argv.filter(a => !a.startsWith('--')).map(a => a.replace(/^levels\//, '').replace(/\.json$/, ''));
let fails = 0, passes = 0, warnings = 0;
const row = (ok: boolean, what: string) => { if (ok) passes++; else fails++; if (!ok || !quiet) console.log(`- ${ok ? 'PASS' : 'FAIL'} — ${what}`); return ok; };
const say = (s: string) => { if (!quiet) console.log(s); };
const f2 = (v: number) => (Math.round(v * 100) / 100).toString();
const pt = (p: readonly number[]) => `(${p.map(f2).join(', ')})`;
const dist = (a: readonly number[], b: readonly number[]) => Math.hypot(a[0] - b[0], a[1] - b[1]);

// ---- the module: a read of no game, so the die it imports is never rolled
const WASM = faceHere() ?? FACE_WASM;
if (!existsSync(WASM)) { console.log(`kitview-plan: no module at ${WASM} (bun run tools/blitz/face-build.ts)`); process.exit(2); }
const X: Data = new WebAssembly.Instance(new WebAssembly.Module(new Uint8Array(readFileSync(WASM)))).exports;
if (typeof X.face_kit_plan !== 'function') { console.log(`kitview-plan: the module at ${WASM} has no face_kit_plan: it is older than this tool (bun run tools/blitz/face-build.ts)`); process.exit(2); }
const encode = new TextEncoder(), decode = new TextDecoder();
function ask(text: string, way: number): Data {
  const b = encode.encode(text), p = X.face_alloc(b.length) >>> 0; new Uint8Array(X.memory.buffer).set(b, p);
  let ok: number; try { ok = X.face_kit_plan(p, b.length, way); } catch (e) { const at = X.face_panic_ptr() >>> 0; throw new Error(`the planner panicked: ${decode.decode(new Uint8Array(X.memory.buffer).subarray(at, at + X.face_panic_len()))}`, { cause: e }); } finally { X.face_free(p, b.length); }
  const at = X.face_out_ptr() >>> 0, out = decode.decode(new Uint8Array(X.memory.buffer).subarray(at, at + (X.face_out_len() >>> 0)));
  if (!ok) throw new Error(out);
  return JSON.parse(out);
}

// ---- the level files
const files = readdirSync(`${ROOT}/levels`).filter(f => f.endsWith('.json') && !f.includes('.lint.')).map(f => f.replace(/\.json$/, '')).sort();
for (const a of asked) if (!files.includes(a)) { console.log(`kitview-plan: no level '${a}' under levels/`); process.exit(2); }
const cache = new Map<string, { text: string; data: Data }>();
function levelOf(name: string) { let L = cache.get(name); if (L) return L; const text = readFileSync(`${ROOT}/levels/${name}.json`, 'utf8'); L = { text, data: JSON.parse(text) }; cache.set(name, L); return L; }
const names = asked.length ? asked : files, withBlock = names.filter(n => levelOf(n).data.scenarios?.kitView);
/** a way as a request: the file's text with that way rewritten */
function withWay(data: Data, k: number, Y: Data): string { const ways = data.scenarios.kitView.ways.slice(); ways[k] = Y; return JSON.stringify({ ...data, scenarios: { ...data.scenarios, kitView: { ...data.scenarios.kitView, ways } } }); }
const said = (name: string, Y: Data, k: number) => `${name} way ${k} (${Y.door ?? 'span'}${Y.set ? ', set ' + Y.set : ''})`;
/** a place moved onto the anchor's line (the break) */
function onLine(data: Data, Y: Data, p: number[]): number[] { const d = (data.doors as Data[]).find(o => o.id === Y.door); if (!d) return p; const t = [Math.cos(d.closedDir), Math.sin(d.closedDir)], al = (p[0] - d.hinge[0]) * t[0] + (p[1] - d.hinge[1]) * t[1]; return [d.hinge[0] + t[0] * al, d.hinge[1] + t[1] * al, p[2]]; }

// ---- B
function parity() {
  say('\n=== B. the records office, both ways: worked out against written');
  const R = levelOf('records');
  (R.data.scenarios.kitView.ways as Data[]).forEach((Y, k) => {
    const A = ask(R.text, k).plan, bare: Data = { ...Y }; delete bare.side; delete bare.slots;
    const D0 = ask(withWay(R.data, k, bare), k).plan, D = BREAK ? { ...D0, side: -D0.side } : D0;   // (the break: the side worked out read flipped)
    const nude: Data = { ...bare }; delete nude.hide; const O = ask(withWay(R.data, k, nude), k).plan, w = `records way ${k} (${Y.door})`;
    say(`  ${w}: written side ${A.side} hand ${A.hand} · ${A.slots.map(pt).join(' ')}\n  ${' '.repeat(w.length)}  worked out side ${D.side} hand ${D.hand} · ${D.slots.map(pt).join(' ')}`);
    row(D.src.side === 'derived' && D.side === A.side, `${w}: the side worked out (${D.side}) is the written one (${A.side})`);
    row(D.hand === A.hand && A.hand === 'left', `${w}: the hand worked out (${D.hand}) is the written picture's (${A.hand}: the stack left of the door)`);
    // (a place the worked-out line had to step over a box for is not held to the written one: that box is why the place was written)
    const dd: number[] = A.slots.map((p: number[], i: number) => dist(p, D.slots[i])), off = dd.filter((d, i) => d > (i >= 1 && i <= 3 ? TOL.stack : TOL.sam)).length;
    row(off <= D.stepped && off <= 1, `${w}: each place worked out is where it is written, the stack within ${TOL.stack} m and Sam's line within ${TOL.sam} m, but one the line stepped over a box for — ${dd.map((d, i) => `${i}: ${f2(d)}`).join(' · ')} m, ${D.stepped} stepped`);
    row(Math.abs(D.wall.face - 0.12) < 0.005 && Math.abs(D.wall.thickness - 0.24) < 0.005 && D.wall.top >= 2.6, `${w}: the wall measured is the level kit's — face ${D.wall.face} m off the line, ${D.wall.thickness} m thick, ${D.wall.top} m tall`);
    const P = D.lamps.porch, lampOk = Math.abs(P.offset[1] - 0.67) <= TOL.lamp && Math.abs(P.offset[2] - 2.4) <= TOL.lamp && !!P.fixture && Math.abs(P.fixture.at[1] - 0.17) <= TOL.lamp && Math.abs(P.fixture.at[2] - 2.27) <= TOL.lamp;
    row(lampOk, `${w}: the porch lamp and its box hang where the table hangs them within ${TOL.lamp} m — lamp ${pt(P.offset)}, box ${P.fixture ? pt(P.fixture.at) : 'none'}`);
    const occ = new Set((O.occluders as Data[]).map(o => o.id)), car = CAR.filter(id => occ.has(id)).length;
    if (k === 0) row(car === CAR.length, `${w}: the parked car is among what is worked out to stand in the camera's way with no \`hide\` — ${car} of its ${CAR.length} boxes`);
    else row(O.occluders.length === 0, `${w}: nothing stands in the camera's way (the block hides nothing here) — ${O.occluders.length}${O.occluders.length ? ': ' + (O.occluders as Data[]).map(o => o.id).join(', ') : ''}`);
  });
}

// ---- C
/** a `hide` / `show` / `pane.box` that names a box by its GENERATED id (tools/levels/lib/levelLoad.ts gives a box with no name of its own its PLACE in the file): add a box before it in the level's source and the
 *  name is another box's, with nothing red. And of a WRITTEN `hide` (it wins outright, so nothing else looks at it): a box it takes in no line from the camera, and what still stands in those lines */
const GENERATED = /^box_\d{4,}$/;
function hideNotes(w: string, Y: Data, A: Data) {
  const B: Data[] = A.boxes, P = A.plan, few = (v: string[]) => v.length > 4 ? `${v.slice(0, 2).join(', ')} … ${v[v.length - 1]}` : v.join(', ');
  const gen = ([['hide', Y.hide ?? []], ['hideMore', Y.hideMore ?? []], ['show', Y.show ?? []], ['pane.box', Y.pane?.box ? [Y.pane.box] : []]] as [string, string[]][]).map(([k, v]) => [k, v.filter(nm => GENERATED.test(nm) && !B.some(b => b.group === nm))] as [string, string[]]).filter(x => x[1].length);
  if (gen.length) { warnings++; console.log(`- WARNING — ${w}: ${gen.map(([k, v]) => `\`${k}\` names ${v.length} box${v.length === 1 ? '' : 'es'} by generated id (${few(v)})`).join(', ')}: an id of that kind is the box's place in the file. Give the boxes a \`group\` of their own (or a \`name\`) there and write that`); }
  if (Y.hide === undefined) return;
  const hide = new Set<string>(Y.hide), occ = new Set((P.occluders as Data[]).map(o => o.index)), shown = new Set<string>(Y.show ?? []);
  const idle = (P.hidden as number[]).filter(i => (hide.has(B[i].id) || (B[i].group != null && hide.has(B[i].group))) && !occ.has(i) && B[i].top > A.numbers.lowTop);
  const still = (P.occluders as Data[]).filter(o => !P.hidden.includes(o.index) && !shown.has(o.id) && !shown.has(o.group));
  if (idle.length) say(`  note — ${w}: \`hide\` takes ${idle.length} box${idle.length === 1 ? '' : 'es'} in no line from the camera to the men, the anchor or the stage's floor (${few(idle.map(i => B[i].id))})`);
  if (still.length) say(`  note — ${w}: with the \`hide\` as written ${still.length} box${still.length === 1 ? '' : 'es'} still stand${still.length === 1 ? 's' : ''} in the camera's line (${few(still.map(o => o.id))})`);
}
/** a man's walk from Sam's place through the opening, on the nav grid's cells (four ways, the leaf not counted: it swings): a set's sill, drift or post in the doorway would shut it */
function walkable(A: Data): { asked: boolean; hit: boolean; deep: number } {
  const P = A.plan, N = A.numbers; if (P.anchor.kind === 'wall' || P.inside < 1 || !P.slots.length) return { asked: false, hit: true, deep: 0 };
  const deep = Math.min(P.inside - 0.3, 0.75), d = deep - P.wall.back + (BREAK ? 60 : 0), cell = (x: number, z: number) => [Math.floor(x / N.cell), Math.floor(z / N.cell)];
  const free = (i: number, j: number) => i >= 0 && j >= 0 && i < N.w && j < N.h && A.grid[j * N.w + i] === '0';
  const goal = cell(P.anchor.mid[0] - P.anchor.n[0] * d, P.anchor.mid[1] - P.anchor.n[1] * d), from = cell(P.slots[0][0], P.slots[0][1]), seen = new Uint8Array(N.w * N.h), q: number[][] = free(from[0], from[1]) ? [from] : [];
  if (q.length) seen[from[1] * N.w + from[0]] = 1;
  while (q.length) { const [i, j] = q.shift()!; if (i === goal[0] && j === goal[1]) return { asked: true, hit: true, deep }; for (const [a, b] of [[i + 1, j], [i - 1, j], [i, j + 1], [i, j - 1]]) if (free(a, b) && !seen[b * N.w + a]) { seen[b * N.w + a] = 1; q.push([a, b]); } }
  return { asked: true, hit: false, deep };
}
const answers = new Map<string, Data>();
function every(name: string, f: (Y: Data, k: number, A: Data, data: Data) => void) { const L = levelOf(name); (L.data.scenarios.kitView.ways as Data[]).forEach((Y, k) => { const key = `${name} ${k}`; let A = answers.get(key); if (!A) { A = ask(L.text, k); answers.set(key, A); } f(Y, k, A, L.data); }); }
/** what the picture would load, for the validator: the block is said of the file, and what the loader laid is the level's by then */
const laidErrs = (A: Data, none: string): string[] => A.level ? validateLevelData({ ...A.level, scenarios: { ...A.level.scenarios, kitView: undefined } }) : [none];
function generic() {
  say('\n=== C. every level with a block');
  for (const name of withBlock) {
    { const L = levelOf(name), errs: string[] = validateLevelData(L.data).filter((e: string) => e.includes('kitView')); row(errs.length === 0, `${name}: the block passes the level validator${errs.length ? ' — ' + errs.slice(0, 3).join('; ') : ''}`); }
    every(name, (Y, k, A0, data) => { const w = said(name, Y, k), S = data.scenarios;
      const A = BREAK && Y.slots?.[1] ? ask(withWay(data, k, { ...Y, slots: Y.slots.map((p: number[] | null, i: number) => i === 1 && p ? onLine(data, Y, p) : p) }), k) : A0, P = A.plan, inWay = (P.problems as string[]).filter(p => p.startsWith(IN_WAY));
      row(P.problems.length === 0, `${w}: the plan has no problems${P.problems.length ? ' — ' + P.problems.join(' · ') : ''}`);
      row(inWay.length === 0, `${w}: no piece of the building is sunk for the camera (${P.occluders.length} in its way, ${A.sunk.length} boxes out of the picture)${inWay.length ? ' — ' + inWay.join(' · ') : ''}`);
      hideNotes(w, Y, A);
      // (one block, several starts: a quiet mission deals Sam's start, and a level with more than one kind of mission starts each team somewhere else: the side the picture is taken from is the block's to say. A way on a set has the set's own)
      { const kinds = ['roomClear', 'nightBreach', 'rescue'].filter(x => S[x]?.team).length; if ((S.quiet || kinds > 1) && Y.set === undefined) row(Array.isArray(Y.ref), `${w}: the level's missions start in more than one place (${S.quiet ? 'a quiet mission deals its start' : kinds + ' kinds of mission'}) and the way writes its \`ref\`, the side the picture is taken from`); }
      row(P.fit === 5 && P.slots.length === 5, `${w}: five places on free floor, outside every box, ${A.numbers.gap} m apart down the stack and ${A.numbers.apart} m from anybody, nobody in the wall — ${P.fit} of 5 fit (${(P.slotSrc as string[]).filter(x => x === 'authored').length} written)`);
      row(P.anchor.kind === 'door' ? P.anchor.door !== null && P.wall.boxes.length > 0 : P.wall.boxes.length > 0 || P.anchor.kind === 'gap', `${w}: the anchor is ${P.anchor.kind === 'door' ? `the door '${P.anchor.door}' in a wall` : P.anchor.kind === 'wall' ? 'a stretch of wall' : 'a real gap'} — ${P.wall.boxes.length} wall box${P.wall.boxes.length === 1 ? '' : 'es'} on its line, face ${P.wall.face}, top ${P.wall.top}`);
      // (EVERY PICK SHOWS ITS KIT AT WORK: a row that works a leaf — the hammer, the breaching shotgun, a door charge, the wand, a bang or a frag on order — has none on a span and is refused by a steel door. A mission whose own way in is a gate, a gap or a wall takes a set with a door in it, or the first door behind the way in)
      { const steel = BREAK || !!P.anchor.steel; row(P.anchor.kind === 'door' && !steel, `${w}: the anchor has a leaf every door tool can work — ${P.anchor.kind !== 'door' ? `a ${P.anchor.kind}: no leaf (the hammer, the shotgun, a door charge, the wand, a bang or a frag on order would play the gun's row)` : steel ? `'${P.anchor.door}' is steel: the hammer, the shotgun, a charge and the wand are refused` : `'${P.anchor.door}', not steel`}`); }
      if (Y.add !== undefined && Y.set === undefined) { const errs = laidErrs(A, 'nothing was laid in'); row(errs.length === 0, `${w}: with the way's \`add\` laid in after the level's own (what the picture loads: ${A.level ? A.level.boxes.length - data.boxes.length : 0} box(es), ${A.level ? A.level.lights.length - data.lights.length : 0} lamp(s) more) the level validator takes it${errs.length ? ' — ' + errs.slice(0, 3).join('; ') : ''}`); }
      if (Y.set !== undefined) { const errs = laidErrs(A, 'the set was not laid'); row(errs.length === 0, `${w}: with the set in the level's place (what the picture loads) the level validator takes it${errs.length ? ' — ' + errs.slice(0, 3).join('; ') : ''}`); }
      { const W = walkable(A); if (W.asked) row(W.hit, `${w}: the way in is walkable — from Sam's place through the ${P.anchor.kind} to ${f2(W.deep)} m behind it, on the nav grid's cells`); }
      if (Y.set === undefined && !Y.litInside && !((Y.roomLights ?? Y.light?.roomLights ?? 0) > 0)) { const dark = (A.litInside as Data[]).filter(l => l.group !== 'exterior' && !P.keepLights.includes(l.group)); if (dark.length) say(`  note — ${w}: ${dark.length} lamp${dark.length === 1 ? '' : 's'} behind the anchor go${dark.length === 1 ? 'es' : ''} out with the room lights (${dark.map(l => `${l.id} of '${l.group}'`).join(', ')}): the way's \`litInside: true\` keeps the room lit`); }
      say(`  ${w}: side ${P.side} (${P.src.side}) hand ${P.hand} · ${f2(P.out)} m in front, ${f2(P.inside)} m behind · pane ${P.pane ? `${P.pane.box}, ${f2(P.pane.along)} m along the wall` : 'none'} · ${P.occluders.length} in the camera's way, ${P.hidden.length} hidden · ${P.indoors ? 'indoors' : 'outdoors'} · lane ${P.lane.free ? 'free' : 'shut'}`); });
  }
  if (!withBlock.length) say('  (no level asked about carries a block)');
}

// ---- C3
/** who stands where: the file's first `squad` + 1 places, the fourth man on his acting place with his own card open */
const stands = (slots: number[][], a4: number[], squad: number, card: number) => { const o: number[][] = []; for (let i = 0; i <= Math.min(4, squad) && i < slots.length; i++) o.push(i === 4 && card === 4 ? a4 : slots[i]); return o; };
function acting() {
  say('\n=== C3. every card open, every squad size: no man\'s place within a body of another\'s');
  for (const name of withBlock) every(name, (Y, k, A) => { const P = A.plan, a4: number[] = BREAK ? P.slots[0] : A.act4; let m = Infinity, at = '';
    for (let squad = 1; squad <= 4; squad++) for (let card = 1; card <= squad; card++) { const M = stands(P.slots, a4, squad, card); for (let i = 0; i < M.length; i++) for (let j = i + 1; j < M.length; j++) { const d = dist(M[i], M[j]); if (d < m) { m = d; at = `a squad of ${squad}, teammate ${card}'s card: places ${i} and ${j}`; } } }
    row(P.slots.length === 5 && m >= A.numbers.actClear - 1e-6 && A.act4Free, `${said(name, Y, k)}: every card open at every size, no two men within ${A.numbers.actClear} m — the nearest ${f2(m)} m (${at}); the fourth man's acting place ${pt(a4)} ${A.act4Free ? 'on free floor' : 'NOT ON FREE FLOOR'}`); });
}

// ---- D
function quietWays() {
  say('\n=== D. a quiet mission\'s way: the pool lamp hangs over a glowing box on a wall');
  const quietLevels = new Set((ALL_MISSIONS as Data[]).filter(e => e.kind === 'quiet').map(e => (e.level ?? 'office.json').replace(/\.json$/, ''))); quietLevels.add('office');   // (the solo night: no row of the list, the bundled office's first way)
  for (const name of withBlock) { if (!quietLevels.has(name)) continue; const Y = levelOf(name).data.scenarios.kitView.ways[0], F = (answers.get(`${name} 0`) ?? ask(levelOf(name).text, 0)).plan.lamps.porch.fixture;
    row(!!F, `${said(name, Y, 0)}: ${F ? `the box at ${pt(F.at)}` : 'NO WALL CARRIES ONE: nothing to shoot out'}`); }
}

// ---- E
function theWords() {
  say('\n=== E. the block\'s own rules at the validator');
  const name = withBlock.find(n => levelOf(n).data.scenarios.kitView.ways[0].set === undefined); if (!name) return;
  const L = levelOf(name), Y = L.data.scenarios.kitView.ways[0], errs: string[] = validateLevelData(JSON.parse(withWay(L.data, 0, { ...Y, add: { boxes: [], laid: true } }))).filter((e: string) => e.includes('.add'));
  row(errs.length > 0, `a file that writes \`add.laid\` is refused: it is the loader's word, and said in a file it would pass any box off as an added piece — ${errs[0] ?? 'NOT REFUSED'}`);
  const broken = withWay(L.data, 0, { ...Y, door: 7 });
  let refused = false; try { parseLevelData(broken); } catch { refused = true; }
  const warn = console.warn; console.warn = () => { /* (the line the page says) */ }; let played: Data = null; try { played = parseLevelData(broken, { kitView: 'drop' }); } catch { /* */ } console.warn = warn;
  row(refused && !!played && played.scenarios?.kitView === undefined, `a broken block refuses a level everywhere (${refused ? 'refused' : 'NOT REFUSED'}) but where it is loaded to play, where the block is left out (${played ? 'left out' : 'THE LEVEL WAS REFUSED'})`);
}

console.log(`levels with a kitView block (${withBlock.length}${asked.length ? ' of those asked about' : ''}): ${withBlock.join(', ') || 'none'}`);
if (BREAK) console.log('KITPLAN_BREAK=1: what is asked is spoiled — rows below must FAIL');
if (!asked.length || asked.includes('records')) parity();
generic();
acting();
quietWays();
theWords();
if (warnings) console.log(`\n${warnings} WARNING line${warnings === 1 ? '' : 's'} above (names by generated id: said, never failed)`);
console.log(`\nkitview-plan: ${passes} PASS, ${fails} FAIL${BREAK ? ' (under KITPLAN_BREAK=1: FAILs are the expected answer)' : ''}`);
process.exit(fails ? 1 : 0);
