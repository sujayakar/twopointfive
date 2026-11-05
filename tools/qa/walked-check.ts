// A LEVEL'S STARTS AND ITS DOORS ARE ON A WAY. scene/ways.ts levelWays works out where a level is walked from what the level says of itself, and the
// look's page wears the floor along those ways. The maintainer, 11-04: "wear, paths in the ground, different materials can all tell a story for how people
// walk through the level, its logic, leading the player for game play". A story that leads the player starts where he starts and goes through the doors he
// will use, so this asks every level whether the rule reaches them. (The rule's own arithmetic is tools/qa/ways-check.ts's, the look task's.)
// ASKED OF THE LEVEL AS THE PAGE LOADS IT, the only reader the ways have: its file with the lived rooms laid and its dressing in, said to the loader by
// word (`lived`, `dressing`) whatever stand the caller's harness is on (their furniture is on the walk grid), the office with its script (scene/level.ts
// withOfficeScript: its scenarios are code, not in its file). A look twin stands on its plain file's floor with its plain file's scenarios: the plain
// file is asked, a twin asked by name is its plain file (scene/look.ts LOOK_LEVELS), and line 1 holds that the twin does stand on it: the same walk
// grid, the same doors, the same ways.
// WHAT IS HELD:
//   1 found     no way is without a path, no name a level's own `scenarios.walked` asks for is missing (levelWays' `lost` and `unknown`), and every
//               count file is a level's and names only a start or a door that level has; and a look twin walks as its plain file does
//   2 starts    from every START the level stands, a way LEAVES: a team's way or the author's own begins there and runs two metres or more. A start is
//               anything in the level's scenarios that says where `sam` stands (tools/qa/said.ts saidIn, as marksight-check reads its men: no kind of mission is named, so a
//               kind the rule does not read yet is owed the day it is written), and the level's own SPAWN where it stands none or where the plain night
//               is played (the level a mission row with no level opens: scenarioList.ts levelWords). Asked of the levels the mission list draws
//               (listedLevels): a showroom leads nobody
//   3 doors     every DOOR is on a way: a traced way two metres long or more CROSSES its frame within the frame's width, or ENDS AT it (an end within
//               0.75 m of the frame's middle: a team sent to a door, the author's row to one, the yard's ways from outer door to outer door). A way
//               that only passes the door along its wall does not lead to it, and the doorstep every door has in the map is no way
//               (RULED, the dress goal 11-05: a way that ENDS at a door counts. The check asks whether anything leads a person to the door, and a lane
//               that arrives there does. "Crosses the frame" alone would ask more than a worn path needs: do not tighten it)
// WHAT THE RULE DOES NOT YET REACH IS OWED, BY NAME, in a file a level: tools/qa/fixtures/walked/<level>.txt, a line `start <name>` or `door <name>` for
// each. The check is RED WHEN A LEVEL OWES ONE ITS FILE DOES NOT NAME (a start or a door the rule does not reach was added, or a way was lost). A line
// whose start or door has a way NOW is said and --write drops it: until then it would excuse that door again, so drop it when it is said. A line that
// starts # is a note and stays where it was written. So the files are the list of what is left: a level's author gives a door a way with a
// `scenarios.walked` row to it (docs/level-format.md; the office's are code), or the rule learns to.
//   bun run tools/qa/walked-check.ts [level …] [--write]     exit 0 = pass · 1 = a FAIL line · 2 = an unknown level or word
//   WALKED_BREAK=1 …                                          the red path: the ways are asked of each level with its routes and its scenarios taken
//                                                             away (nothing walks but door to door outside): lines 2 and 3 must FAIL
import { readFileSync, readdirSync, existsSync, writeFileSync, mkdirSync, unlinkSync } from 'node:fs';
import { resolve } from 'node:path';
import { saidIn } from './said.ts';
import { loadLevel } from '../../src/scene/levelLoad.ts';
import { parseLevelData } from '../../src/scene/levelData.ts';
import { StaticCollision } from '../../src/scene/collision.ts';
import { levelWays, doorMid } from '../../src/scene/ways.ts';
import { withOfficeScript, isOfficeFamily } from '../../src/scene/level.ts';
import { LOOK_LEVELS } from '../../src/scene/look.ts';
import { listedLevels, levelWords } from '../../src/scene/scenarioList.ts';

const ROOT = resolve(import.meta.dir, '../..'), BREAK = process.env.WALKED_BREAK === '1', DIR = `${ROOT}/tools/qa/fixtures/walked`;
type V3 = number[];
const PLAIN_OF = new Map(Object.entries(LOOK_LEVELS).map(([plain, twin]) => [twin, plain]));
/** a level file's name as this check asks it: a look twin is its plain file */
export const plainOf = (name: string): string => { const f = `${name.replace(/\.json$/, '')}.json`; return (PLAIN_OF.get(f) ?? f).replace(/\.json$/, ''); };

/** A LEVEL AS THE PAGE WALKS IT: its file loaded with the lived rooms and the dressing in, the office with its script, the walk grid of its boxes (the
 *  one the page's own reader is handed: game.ts gives buildWearMap `col.nav`) and its ways. tools/levels/walked-overlay.ts draws the same.
 *  `bare` = the red path's: no routes, no scenarios */
export function pageWays(name: string, bare = false) {
  { const data = parseLevelData(readFileSync(`${ROOT}/levels/${name}.json`, 'utf8')), loaded = loadLevel(data, { lived: true, dressing: true }), level: any = isOfficeFamily(data) ? withOfficeScript(loaded) : loaded;
    const nav = new StaticCollision(level.boxes).nav, ways = bare ? levelWays({ ...level, routes: [] }, undefined, nav, { hints: [] }) : levelWays(level, level.scenarios, nav);
    return { data, level, nav, ways }; }
}
/** THE STARTS A LEVEL STANDS: every `sam` its scenarios say, and its own spawn where they say none or where the plain night is played */
export function startsOf(name: string, level: any): { id: string; v: V3 }[] {
  const stood = saidIn({ scenarios: level.scenarios ?? {} }, o => Array.isArray(o.sam) && o.sam.length === 3 ? o.sam as V3 : null);
  return !stood.length || name === levelWords({}) ? [...stood, { id: 'spawn', v: level.playerSpawn as V3 }] : stood;
}

export async function walkedCheck(args: string[] = []): Promise<number> {
  const write = args.includes('--write') && !BREAK, bad = args.filter(a => a.startsWith('--') && a !== '--write');
  if (bad.length) { console.error(`walked: no such word ${bad.join(' ')}     bun run tools/qa/walked-check.ts [level …] [--write]`); return 2; }
  const files = readdirSync(`${ROOT}/levels`).filter(f => f.endsWith('.json') && !f.includes('.lint.')).map(f => f.replace(/\.json$/, ''));
  const every = [...new Set(files.map(plainOf))].sort(), asked = [...new Set(args.filter(a => !a.startsWith('--')).map(plainOf))];
  for (const n of asked) if (!existsSync(`${ROOT}/levels/${n}.json`)) { console.log(`walked: no level ${n}`); return 2; }
  const levels = asked.length ? asked : every, listed = new Set<string>(listedLevels().map(l => l.level));
  console.log(`\n=== walked: a level's starts and its doors are on a way, as the page loads it${BREAK ? ' (WALKED_BREAK: no routes, no scenarios)' : ''} ===`);
  let ways = 0, starts = 0, doors = 0, owedStarts = 0, owedDoors = 0, judged = 0; const sick: string[] = [], fresh: string[] = [], mended: string[] = [], wrote: string[] = [], unbitten: string[] = [];
  if (!asked.length && existsSync(DIR)) for (const f of readdirSync(DIR).filter(f => f.endsWith('.txt'))) if (!every.includes(f.slice(0, -4))) sick.push(`${f} is the count file of no level`);
  const len = (p: V3[]) => p.reduce((n, q, k) => k ? n + Math.hypot(q[0] - p[k - 1][0], q[2] - p[k - 1][2]) : 0, 0), near = (a: V3, b: V3, r: number) => Math.hypot(a[0] - b[0], a[2] - b[2]) <= r;
  for (const name of levels) {
    const { level, nav, ways: A } = pageWays(name, BREAK), paths = (A.paths as { kind: string; points: V3[] }[]).filter(w => len(w.points) >= 2); ways += A.paths.length;
    { const twin = LOOK_LEVELS[`${name}.json`]?.replace(/\.json$/, ''); if (twin && existsSync(`${ROOT}/levels/${twin}.json`)) { const T = pageWays(twin, BREAK), doorsOf = (l: any) => JSON.stringify(l.doors.map((d: any) => [d.name, d.hinge, d.closedDir, d.width, !!d.exterior]));
      const differs = [T.nav.blocked.length !== nav.blocked.length || T.nav.blocked.some((v: number, k: number) => v !== nav.blocked[k]) ? 'walk grid' : '', doorsOf(T.level) !== doorsOf(level) ? 'doors' : '', JSON.stringify(T.ways.paths) !== JSON.stringify(A.paths) ? 'ways' : ''].filter(x => x);
      if (differs.length) sick.push(`${twin} does not walk as ${name} does (${differs.join(', ')}): the page loads the twin, and this asks the plain file`); } }
    if (A.lost) sick.push(`${name}: ${A.lost} with no path`); if (A.unknown.length) sick.push(`${name}: no such ${A.unknown.join(', ')}`);
    // a NAMED MARK is where a person stands (LevelData.marks): on floor the walk grid calls open, or no way can end at it
    for (const m of (level.marks ?? []) as { name: string; pos: V3 }[]) if (nav.isBlocked(m.pos[0], m.pos[2])) sick.push(`${name}: the mark '${m.name}' at (${m.pos[0]}, ${m.pos[2]}) stands where nobody can (the walk grid's cell is shut)`);
    const leaves = (s: V3) => paths.some(w => (w.kind === 'team' || w.kind === 'hint') && near(w.points[0], s, 0.3));
    /** is this door on a way: does one cross its frame, face to face within its width, or end at it? */
    const onWay = (d: { hinge: readonly [number, number]; closedDir: number; width: number }) => { const c = doorMid(d) as V3, ax = Math.cos(d.closedDir), az = Math.sin(d.closedDir), side = (p: V3) => -az * (p[0] - c[0]) + ax * (p[2] - c[2]);
      const crosses = (pts: V3[]) => { let prev: { p: V3; s: number } | null = null, on: V3 | null = null;   // (a point of the way that lies ON the frame's line is where it crosses, if the way comes out the other side)
        for (const p of pts) { const sd = side(p); if (Math.abs(sd) < 1e-9) { on = p; continue; }
          if (prev && prev.s * sd < 0) { const t = prev.s / (prev.s - sd), x = on ? on[0] : prev.p[0] + (p[0] - prev.p[0]) * t, z = on ? on[2] : prev.p[2] + (p[2] - prev.p[2]) * t; if (Math.abs(ax * (x - c[0]) + az * (z - c[2])) <= d.width / 2 + 0.15) return true; }
          prev = { p, s: sd }; on = null; } return false; };
      return paths.some(w => near(w.points[0], c, 0.75) || near(w.points[w.points.length - 1], c, 0.75) || crosses(w.points)); };
    // what is owed now, and every name a line may say
    const owes: string[] = [], known = new Set<string>(), doorWord = (d: { name: string }) => `door ${String(d.name).replace(/\s/g, '_')}`;
    if (listed.has(name)) { judged++; const all = startsOf(name, level); starts += all.length; for (const s of all) { known.add(`start ${s.id}`); if (!leaves(s.v)) owes.push(`start ${s.id}`); } }
    doors += level.doors.length; for (const d of level.doors) { known.add(doorWord(d)); if (!onWay(d)) owes.push(doorWord(d)); }
    owedStarts += owes.filter(o => o.startsWith('start ')).length; owedDoors += owes.filter(o => o.startsWith('door ')).length;
    // … against what its file names
    const file = `${DIR}/${name}.txt`, lines = existsSync(file) ? readFileSync(file, 'utf8').split('\n').filter((l, k) => !(k === 0 && l.startsWith('#'))).map(l => l.replace(/\s+$/, '')).filter(l => l) : [], held = lines.filter(l => !l.startsWith('#'));
    const news = owes.filter(o => !held.includes(o)), gone = held.filter(o => known.has(o) && !owes.includes(o)), unknown = BREAK ? [] : held.filter(o => !known.has(o));
    if (news.length) fresh.push(`${name}: ${news.join(', ')}`); else unbitten.push(name); if (gone.length) mended.push(`${name}: ${gone.join(', ')}`);
    if (unknown.length && !write) sick.push(`${name}.txt names what ${name} does not have: ${unknown.join(', ')}`);
    if (write) { const kept = lines.filter((l, k) => l.startsWith('#') || (owes.includes(l) && lines.indexOf(l) === k)), body = [...kept, ...news];   // (what was there stays in its order, its notes beside what they were written by, each line once; what is newly owed goes last)
      const text = body.length ? `# what the ways of ${name} do not reach yet, as the page loads it: a start no way leaves from, a door no way crosses or ends at (tools/qa/walked-check.ts --write rewrites this first line; a line that starts # under it is a note and is kept)\n${body.join('\n')}\n` : '';
      if (text && (!existsSync(file) || readFileSync(file, 'utf8') !== text)) { mkdirSync(DIR, { recursive: true }); writeFileSync(file, text); wrote.push(name); }
      if (!text && existsSync(file)) { unlinkSync(file); wrote.push(`${name} (nothing owed and no note: its file is gone)`); } }
  }
  const ok1 = sick.length === 0, ok23 = write || fresh.length === 0;
  console.log(`- ${ok1 ? 'PASS' : 'FAIL'} — 1 no way of any level is without a path, no name a level's own ways ask for is missing, every count file is a level's and names what the level has, and a look twin walks as its plain file\n    ${levels.length} levels, ${ways} ways${sick.length ? ' — ' + sick.join(' · ') : ''}`);
  console.log(`- ${ok23 ? 'PASS' : 'FAIL'} — 2, 3 from every start a level stands a way leaves, and every door is on a way, but for those its file names as owed\n    ${starts} starts on the ${judged} levels the mission list draws, ${owedStarts} owed · ${doors} doors, ${owedDoors} owed${fresh.length ? ` — OWED AND NOT IN ITS FILE: ${fresh.join(' · ')}` : ''}${mended.length ? `\n    ON A WAY NOW, still in its file (--write drops it): ${mended.join(' · ')}` : ''}${write ? `\n    --write: ${wrote.length ? 'wrote ' + wrote.join(', ') : 'every file was up to date'}` : ''}`);
  if (BREAK) { console.log(`    (the red path: ${fresh.length} of ${levels.length} levels owe something their file does not name${unbitten.length ? `; NOT BITTEN (every door of theirs is owed already, or they have none): ${unbitten.join(', ')}` : ''})`); if (!fresh.length) { console.log('=== walked: THE RED PATH DID NOT BITE (exit 0: a break word that passes) ==='); return 0; } }
  console.log(`=== walked: ${ok1 && ok23 ? 'ALL PASS' : 'FAIL'} ===`);
  return ok1 && ok23 ? 0 : 1;
}
if (import.meta.main) process.exit(await walkedCheck(process.argv.slice(2)));
