// Write a level file from its generator: tools/levels/<name>.ts exports build() → { level, scenarios?, groups, surfaces? } (a Level made with src/scene/officeGen.ts
// officeKit or src/scene/levelKit.ts, and the scenario data that goes in the file's `scenarios` block) and optionally NAME, NOTES, MATERIALS, PALETTE, DRESS_AS and finish.
// Here the doors are fitted (doorSize.ts), the loud levels dressed (dress.ts) after the lived-in tail, the floor's flats put in order (flatLayers.ts) and a look
// file's skin run (finish); then Level → level data → validate → canonical text → parse → load, and the loaded Level must be the generator's, field for field, or
// nothing is written. A level with a tools/levels/<name>-gwot.ts beside it is exported / checked together with that look file.
//   bun run tools/levels/export.ts <name>            write levels/<name>.json (prints counts, size, digest)
//   bun run tools/levels/export.ts <name> --check    write nothing; exit 1 if levels/<name>.json is missing or differs from what the generator gives now
import { basename, resolve } from 'node:path';
import { existsSync, readFileSync, writeFileSync } from 'node:fs';
const ROOT = resolve(import.meta.dir, '../..');
const { OFFICE_PALETTE, OFFICE_FINISH } = await import(`${ROOT}/src/scene/officeGen.ts`);
const { rowWord, rowDoes: nameDoes } = await import(`${ROOT}/src/scene/stuff.ts`);   // (what each paint is made of, by its name: written into the file's material rows)
const { levelToData, loadLevel } = await import(`${ROOT}/src/scene/levelLoad.ts`);
const { stampDoes, wordWas } = await import('./doesWas.ts');   // (what a box nobody spoke for does: the word its colour had. src/scene/materials.ts)
const { DOES_DEFAULT } = await import(`${ROOT}/src/scene/materials.ts`);
const { validateLevelData, serializeLevelData, parseLevelData, levelDataDigest, structuralDiff, canonicalLevelData } = await import(`${ROOT}/src/scene/levelData.ts`);
const { WORLD } = await import(`${ROOT}/src/scene/world.ts`);

const argv = process.argv.slice(2), name = argv.find(a => !a.startsWith('--')), CHECK = argv.includes('--check');
if (!name || !/^[\w-]+$/.test(name) || !existsSync(resolve(import.meta.dir, `${name}.ts`))) { console.error(`usage: bun run tools/levels/export.ts <name> [--check]   (a generator tools/levels/<name>.ts)`); process.exit(2); }
const gen = await import(resolve(import.meta.dir, `${name}.ts`));
const OUT = resolve(ROOT, `levels/${gen.NAME ?? name}.json`);

const { level, scenarios, groups, surfaces } = gen.build();
{ const { fitDoors, fitWaysOut } = await import(`${ROOT}/src/scene/doorSize.ts`);
  fitDoors(level.doors);   // every leaf the real door of its slot (doorSize.ts — a generator that frames a COPY of a def leaves the level's own unfitted)
  fitWaysOut(level, scenarios?.quiet); }   // … and a way out that names a door goes through the door as fitted
// DRESSING (tools/levels/dress.ts): the loud levels get things to wreck, appended LAST under their own group — which the loader leaves out with the destruction switch off
// (game/destructionSwitch.ts), so the switch is raised here for the reload check below: the file must reproduce the DRESSED level.
// LIVED-IN ROOMS (src/scene/lived.ts): what furnishes a room — boxes in group 'lived.<room>', props / lights / fixtures named 'lived.…' — is left out by the loader with
// its switch off, and off must be the level as it was. So: the lived things are the TAIL of each of the generator's lists (nothing kept comes after one, so no kept
// index moves and a fixture's box references stay true), nothing kept names a lived box, and the dressing below is dealt as if they were not there (the same
// dressing whether the switch is on or off). The switch is raised here for the reload check, like the destruction's.
const { LIVED, isLived } = await import(`${ROOT}/src/scene/lived.ts`); LIVED.on = true;
const livedFrom = (() => { const tail = (what: string, tags: (string | undefined)[]) => { let first = tags.findIndex(isLived); if (first < 0) first = tags.length; const stray = tags.findIndex((t, i) => i > first && !isLived(t)); if (stray >= 0) throw new Error(`lived things must come last: ${what} ${stray} ('${tags[stray]}') follows the lived ${what} ${first} ('${tags[first]}')`); return first; };
  const boxes = tail('box', groups), props = tail('prop', level.props.map((p: { name: string }) => p.name)), lights = tail('light', level.lights.map((l: { name: string }) => l.name)), switchables = tail('switchable', level.switchables.map((s: { name: string }) => s.name));
  level.switchables.forEach((s: { name: string; boxes: number[] }, i: number) => { if (i < switchables && s.boxes.some(b => b >= boxes)) throw new Error(`switchable '${s.name}' is not lived and names a lived box`); });
  level.lights.forEach((l: { name: string; fixtureBox?: number }, i: number) => { if (i < lights && l.fixtureBox !== undefined && l.fixtureBox >= boxes) throw new Error(`light '${l.name}' is not lived and names a lived box as its fixture`); });
  for (const b of level.breaker?.boxes ?? []) if (b >= boxes) throw new Error(`the breaker names a lived box`);
  for (const bc of level.beacons ?? []) if (bc.box >= boxes) throw new Error(`a beacon names a lived box`);
  const n = level.boxes.length - boxes;
  if (n || props < level.props.length || lights < level.lights.length || switchables < level.switchables.length) { const rooms = new Map<string, number>(); for (const g of groups.slice(boxes)) rooms.set(g, (rooms.get(g) ?? 0) + 1);
    console.log(`lived: ${n} boxes, ${level.props.length - props} props, ${level.lights.length - lights} lights, ${level.switchables.length - switchables} fixtures — ${[...rooms].map(([g, k]) => `${g.replace(/^lived\.?/, '') || '(no room)'} ${k}`).join(' · ')}`); }
  return { boxes, props, lights, switchables }; })();
/** the dressing, dealt on the level WITHOUT its lived things and appended after them: [the generator's own boxes, lived, dressing] */
const dressBare = (dressLevel: (l: unknown, g: string[]) => { added: number; surfaces: number; by: Record<string, number> }) => {
  const view = { ...level, boxes: level.boxes.slice(0, livedFrom.boxes), props: level.props.slice(0, livedFrom.props), lights: level.lights.slice(0, livedFrom.lights), switchables: level.switchables.slice(0, livedFrom.switchables) }, g2 = groups.slice(0, livedFrom.boxes);
  const nL = level.boxes.length, st = dressLevel(view, g2); level.boxes.push(...view.boxes.slice(livedFrom.boxes)); groups.push(...g2.slice(livedFrom.boxes));
  // (the dressing does not know the furniture is there: a lived thing put where the dressing lays its own — a radio on a table the dressing papers — would stand IN it with both switches on)
  type B = { c: number[]; h: number[]; yaw: number }; const ext = (b: B) => { const c = Math.abs(Math.cos(b.yaw || 0)), n = Math.abs(Math.sin(b.yaw || 0)); return [b.h[0] * c + b.h[2] * n, b.h[1], b.h[0] * n + b.h[2] * c]; };
  const hit: string[] = []; for (let i = livedFrom.boxes; i < nL; i++) { const a = level.boxes[i] as B, ea = ext(a); if (a.c[1] + a.h[1] < 0.06) continue;   // (a floor piece lies under whatever stands there)
    for (let j = nL; j < level.boxes.length; j++) { const b = level.boxes[j] as B, eb = ext(b); if ([0, 1, 2].every(k => Math.abs(a.c[k] - b.c[k]) < ea[k] + eb[k] - 0.004)) { hit.push(`${groups[i]} at (${a.c.map(v => v.toFixed(2)).join(', ')})`); break; } } }
  if (hit.length) throw new Error(`lived things stand in the dressing (${hit.length}): ${hit.slice(0, 6).join(' · ')} — the dressing is dealt as if the furniture were not there; move them off the surfaces it dresses`);
  return st; };
const nGen = level.boxes.length;   // (the generator's own boxes + its lived things: what follows is the dressing)
{ const { DESTRUCTION } = await import(`${ROOT}/src/game/destructionSwitch.ts`); DESTRUCTION.on = true;
  const { dressLevel, LOUD_LEVELS } = await import('./dress.ts');
  if (LOUD_LEVELS.some((l: { gen: string }) => l.gen === (gen.DRESS_AS ?? name)) && !argv.includes('--bare')) { const st = dressBare(dressLevel); console.log(`dressing: ${st.added} boxes on ${st.surfaces} surfaces — ${Object.entries(st.by).map(([k, n]) => `${n} ${k}`).join(', ')}`); } }   // (surfaces: the footstep map — game/floor.ts; a generator that gives none gets the office's)
if (groups.length !== level.boxes.length) throw new Error(`trace: ${groups.length} groups for ${level.boxes.length} boxes`);
// THE FLOOR IN ORDER (tools/levels/flatLayers.ts layFloor): two ground finishes or floor decals of different paint that overlap never share a plane — the picture
// cannot order them (each pixel of the overlap went to either). Swept in tiers, so the level with its lived things or its dressing left out is the same level;
// before the look's skin, so both files of a level carry the same boxes. Nothing is moved down; a solid finish grows 0.7 mm thicker, a decal is lifted.
{ const { layLevelFloor } = await import('./flatLayers.ts'); const { BoxFlag } = await import(`${ROOT}/src/scene/boxes.ts`);
  const n = layLevelFloor(level.boxes, livedFrom.boxes, nGen, BoxFlag);
  if (n) console.log(`floor: ${n} flat${n > 1 ? 's' : ''} raised to draw in order`); }
// HOW KEPT THE PLACE IS, a generator's `KEPT` export, 0‥1: the file's meta.kept (docs/level-format.md). A look's second file is the same PLACE in other
// clothes, so it takes the word of the level it dresses (`DRESS_AS`) unless it says its own: the page draws the look's file, and a word said once in the
// plain generator would otherwise never reach a picture (tools/levels/keptWord.ts, the one writing of the rule).
{ const { generatorKept } = await import('./keptWord.ts'); const kept = await generatorKept(gen, n => import(resolve(import.meta.dir, `${n}.ts`)));
  if (kept !== undefined) level.kept = kept; }
if (scenarios) level.scenarios = scenarios;   // (an ordinary, enumerable field: the file's own block — levelToData writes it)
// WHAT EACH BOX DOES (src/scene/materials.ts, Box.does): a box whose maker said no word is given the word its colour had under the old rule
// (tools/levels/doesWas.ts, frozen) HERE, before the look's skin repaints anything: the level and its look twin are one simulation, and a box in other
// clothes does what it did. Once more after the skin, for what the skin added.
stampDoes(level);
// THE LOOK'S SECOND FILE (tools/levels/gwotSkin.ts): a generator that exports `finish` changes the dressed level's clothes here — it exports `DRESS_AS` = the
// generator whose level it is, so the dressing above is that level's, box for box
if (gen.finish) { const n0 = level.boxes.length; const st = gen.finish(level, groups); if (st) console.log(`look: ${st.repainted} boxes repainted, ${st.kept} kept, ${st.decals} decals, ${st.props} props — ${Object.entries(st.by as Record<string, number>).sort((a, b) => b[1] - a[1]).slice(0, 10).map(([k, n]) => `${n} ${k}`).join(', ')}${st.relaid === undefined ? '' : ` — ${st.relaid} flat decals raised to draw in order, the highest top at ${((st.flatTop ?? 0) * 1000).toFixed(2)} mm, ${st.surfaced} now over a flat of the level`}`); if (groups.length !== level.boxes.length) throw new Error(`finish: ${groups.length} groups for ${level.boxes.length} boxes (from ${n0})`); }
stampDoes(level);
const materials: Record<string, { albedo: [number, number, number]; rough?: number; metal?: boolean; stuff?: string; does?: string }> = {};
/** what a paint's row says its boxes do: what the paint's NAME says (scene/stuff.ts PAINT_DOES, as a kit words a box of it), else, for a row of a
 *  generator's own that no table names, the word its colour had. A box that does another thing says so itself (levelLoad.ts levelToData) */
const rowDoes = (name: string, albedo: readonly number[]): { does?: string } => { const said = nameDoes(name) as { does?: string }; if (said.does !== undefined) return said.does !== DOES_DEFAULT ? said : {}; const w = wordWas(albedo); return w !== DOES_DEFAULT ? { does: w } : {}; };
{ const seen = new Set<string>();
  for (const [m, alb] of Object.entries(OFFICE_PALETTE as Record<string, [number, number, number]>)) { const k = alb.join(','); if (seen.has(k)) continue; seen.add(k);
    materials[m] = { albedo: [alb[0], alb[1], alb[2]], ...((OFFICE_FINISH as Record<string, { rough: number; metal?: boolean }>)[m] ?? {}), ...rowWord(m), ...rowDoes(m, alb) }; }
  // a generator's OWN materials (its `MATERIALS` export — the GWOT kit's palette, src/scene/gwotKit.ts GWOT_MATERIALS): added after the office's, a colour the office already names keeps the office's name
  for (const [m, row] of Object.entries({ ...(gen.PALETTE ?? {}), ...(gen.MATERIALS ?? {}) } as Record<string, { albedo: [number, number, number]; rough?: number; metal?: boolean; stuff?: string; does?: string }>)) { const k = row.albedo.join(','); if (seen.has(k) || materials[m]) continue; seen.add(k);
    materials[m] = { albedo: [row.albedo[0], row.albedo[1], row.albedo[2]], ...(row.rough !== undefined ? { rough: row.rough } : {}), ...(row.metal ? { metal: true } : {}), ...(row.stuff !== undefined ? { stuff: row.stuff } : rowWord(m)), ...(row.does !== undefined ? (row.does !== DOES_DEFAULT ? { does: row.does } : {}) : rowDoes(m, row.albedo)) }; } }   // (what the paint is made of: the generator's own word for its row, else the table's for the name. scene/stuff.ts)
const data = levelToData(level, {
  name: gen.NAME ?? name, materials, boxGroups: groups, ...(surfaces ? { surfaces } : {}),
  floors: [{ id: 'f0', name: 'ground floor', y: 0, ceiling: WORLD.ceilingY }],
  meta: { bounds: { min: [...WORLD.min], max: [...WORLD.max] }, generator: `tools/levels/${name}.ts build → tools/levels/export.ts`, notes: gen.NOTES ?? '' },
});
// A PALETTE'S ROWS ARE WRITTEN ONLY WHERE THE FILE WEARS THEM (a generator's `PALETTE` export: rows as `MATERIALS` has them, each free to say its own
// `stuff` and `does`; ./wornRows.ts). So a row added to such a palette tomorrow moves no file that does not wear it. `MATERIALS` is written whole, which
// is why a new row of the kit's palette makes every look file stale: moving that table here is one step that rewrites those files' materials blocks.
if (gen.PALETTE) { const { dropUnworn } = await import('./wornRows.ts'), P = gen.PALETTE as Record<string, { albedo: readonly number[] }>;
  // (a palette's row may not take the name of a material the file already has in other numbers, the office's or the kit's: one of the two would be lost with no word)
  const clash = Object.keys(P).filter(m => Object.hasOwn(materials, m) && materials[m].albedo.join(',') !== P[m].albedo.join(','));
  if (clash.length) { console.error(`export ${name}: PALETTE names ${clash.length} row(s) the file already has in other numbers: ${clash.join(', ')}`); process.exit(1); }
  dropUnworn(data, P); }
const errs = validateLevelData(data);
if (errs.length) { console.error(`validateLevelData: ${errs.length} error(s)\n  ${errs.slice(0, 30).join('\n  ')}`); process.exit(1); }
const text = serializeLevelData(data);
const parsed = parseLevelData(text);
const dataDiffs: string[] = [], levelDiffs: string[] = [];
const nData = structuralDiff(canonicalLevelData(data), parsed, dataDiffs), nLevel = structuralDiff(level, loadLevel(parsed), levelDiffs);
if (nData || nLevel) {
  console.error(`export REFUSED: the file would not reproduce the level — ${nLevel} Level diff(s), ${nData} data diff(s)`);
  for (const d of [...levelDiffs, ...dataDiffs].slice(0, 20)) console.error('  ' + d);
  process.exit(1);
}
const digest = levelDataDigest(parsed), sc = Object.keys(data.scenarios ?? {});
console.log(`${data.name} → boxes ${data.boxes.length} · props ${data.props.length} · lights ${data.lights.length} · switchables ${data.switchables.length} · doors ${data.doors.length} · routes ${data.routes.length} · clear plans ${data.plans.roomClear.length} · scenarios ${sc.length ? sc.join(', ') : 'none'}`);
console.log(`  ${text.split('\n').length - 1} lines, ${(text.length / 1024).toFixed(1)} KB · digest ${digest} · reload identical to the generator (0 diffs)`);
if (CHECK) {
  const cur = existsSync(OUT) ? readFileSync(OUT, 'utf8') : null;
  if (cur !== text) { console.error(`--check: ${OUT} ${cur === null ? 'does not exist' : 'is STALE'} — run bun run tools/levels/export.ts ${name}`); process.exit(1); }
  console.log(`--check: ${OUT} is up to date`);
} else { writeFileSync(OUT, text); console.log(`wrote ${OUT}`); }
// IN STEP: a level that has a file in the default look (tools/levels/<name>-gwot.ts — the same boxes in other paint, src/scene/look.ts) is exported / checked
// with it, so the two never drift apart (tools/qa/look-check.ts compares them box for box).
if (existsSync(resolve(import.meta.dir, `${name}-gwot.ts`))) { const p = Bun.spawnSync(['bun', 'run', import.meta.path, `${name}-gwot`, ...argv.filter(a => a.startsWith('--'))], { stdout: 'inherit', stderr: 'inherit' }); if (p.exitCode !== 0) process.exit(p.exitCode ?? 1); }
// ANOTHER WAY IN: a level that stands a room clear or a rescue is asked the second-way rule the day it is written: whatever its defenders wedge, every
// room with men in it is still reached without a charge (tools/qa/second-way-check.ts, on the shipped stand). The file is written either way. THE RULE
// IS ABOUT A LEVEL A SQUAD MISSION PLAYS (any row of scenarioList.ts ALL_MISSIONS that is not a quiet one: the missions, the lessons, the rows off the
// list), so a red check
// FAILS THE EXPORT of such a level and is a NOTE on any other: a test yard or a showroom stands a team to try things on and no player is ever shut out
// of it, and a level in the making reads here what it will owe the day it has a row. SECONDWAY=0 in front skips the question.
if (!CHECK && !name.endsWith('-gwot') && (data.scenarios?.roomClear || data.scenarios?.rescue) && process.env.SECONDWAY !== '0') {
  const { ALL_MISSIONS } = await import(`${ROOT}/src/scene/scenarioList.ts`), played = (ALL_MISSIONS as { kind: string; level?: string }[]).some(e => e.kind !== 'quiet' && (e.level ?? 'office.json') === basename(OUT));   // (every row a squad plays: the lessons' and the rows off the list with the rest)
  const p = Bun.spawnSync(['bun', 'run', resolve(import.meta.dir, '../qa/second-way-check.ts'), basename(OUT)], { stdout: 'pipe', stderr: 'pipe', env: { ...process.env, TPS_PROFILE: 'shipped' } });
  if (p.exitCode === 1 && played) { console.error(`second way in: FAILED on ${basename(OUT)} (the file is written)\n${p.stdout.toString()}${p.stderr.toString()}`); process.exit(1); }
  if (p.exitCode === 1) console.log(`  second way in: A NOTE, not a failure: no squad mission on the list plays ${basename(OUT)}, and the day one does this fails its export\n${p.stdout.toString().split('\n').filter(l => /NO WAY IN|FAIL —/.test(l)).map(l => `    ${l.trim()}`).join('\n')}`);
  else console.log(p.exitCode === 0 ? '  second way in: every room they can shut has another way in' : `  second way in: NOT ASKED (${p.stderr.toString().trim().split('\n').pop() ?? 'the check did not run'})`);
}
// A DEFENDER SEES WHAT HE WATCHES: the same day, every man the file stands with a `pos` and an `aimAt` is asked for his line and for the ground he watches,
// against the level's own count file (tools/qa/marksight-check.ts, on the shipped stand; the look twin with it). The file is written either way; a red check
// fails the export: something was stood in a man's view, or a level new to the check has no count file yet (the check's --write, once on each stand).
// MARKSIGHT=0 in front skips it for a level still in the making.
if (!CHECK && !name.endsWith('-gwot') && process.env.MARKSIGHT !== '0') {
  const twin = OUT.replace(/\.json$/, '-gwot.json'), p = Bun.spawnSync(['bun', 'run', resolve(import.meta.dir, '../qa/marksight-check.ts'), basename(OUT), ...(existsSync(twin) ? [basename(twin)] : [])], { stdout: 'pipe', stderr: 'pipe', env: { ...process.env, TPS_PROFILE: 'shipped' } });
  if (p.exitCode === 1) { console.error(`a defender sees what he watches: FAILED on ${basename(OUT)} (the file is written)\n${p.stdout.toString()}${p.stderr.toString()}`); process.exit(1); }
  console.log(p.exitCode === 0 ? '  marks\' sight: no man the file stands sees less than its count file says (or it stands none)' : `  marks' sight: NOT ASKED (${p.stderr.toString().trim().split('\n').pop() ?? 'the check did not run'})`);
}
