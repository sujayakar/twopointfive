#!/usr/bin/env bun
// THE DEFENDERS' WAY ROUND SURVIVES THE FURNITURE — `bun run tools/qa/wayround-check.ts [mission id | level name …] [--seeds 1-3] [--md <file>] [--json <file>]`
// A squad whose neighbour has lost a room sends a pair ROUND to its other door (enemysquads.ts decide → pair → wayRound2): each man needs a standing place a
// few metres outside that door and a path to it inside a length limit. Lived-in rooms' furniture is real, so a stone on that place or a wood pile on that
// path takes the move away and the level plays on with defenders who go straight through the house instead: nothing fails, nothing throws, they are only
// less clever. (Found on the two charges, 11-03: 60 runs in 100 sent a pair round on the level as it was, 46 dressed, and every other check was green.)
//
// WHAT IS ASKED, with no fight: each mission that stages defender squads is stood up at t = 0 (nobody has moved), once with lived rooms OFF and once ON
// (TPS_LIVED=0 | 1, a child process each: a level is loaded once a process), for each seed. In it the game's own function is asked (enemysquads.ts
// squadWaysRound) whether there is a way round, for every man at home of every squad that may leave its rooms,
//   · FROM his post and from a stride inside each door of the room he holds (a man who has heard the fight is at his door when the word comes),
//   · TO every other squad's room, with every door of that room taken as the team's door,
//   · with NOBODY of the team known, and with THE TEAM IN THE ROOM: a man in its doorway, one three metres inside, and the team heard through the walls
//     on every two metres of the room's floor. The search prices ground near where the team is heard, so the way bows out from the room's walls: on the
//     two charges it runs 2 m off the back wall then, where it hugs that wall with nobody known. Firewood on THAT line was what play lost its pair to, and
//     the form with nobody known was green on it,
//   · in both forms (go in by the other door; hold it from outside).
//   - FAIL, LOST: a (man, room, door, form) with a way when lived rooms are off and none when they are on.
//   - FAIL, SQUEEZED: a way the furniture made LONGER that now has under MARGIN_M metres left to its length limit (pressure.ts flankMaxM and its two
//     fellows). At t = 0 it still fits; in a fight the man stands a step off his mark and the team's guns close cells, and it no longer does. This is the
//     form the two charges had: the way round the outside of the house is 45.5 to 45.7 m of the 46 allowed, and sacks and drums on it made it 45.8 to 45.9.
//     LONGER means by more than the search's own GRAIN and by more than SHARE of what the way had to spare without furniture. The grain: the walk grid's
//     cells are half a metre, and a path's length moves in steps of about half a cell's diagonal slack (cell × (√2 − 1) / 2 = 0.104 m) when anything
//     anywhere on the floor changes which of two equal ways the search takes. Measured 11-04: a warehouse way with nothing lived near it moves 0.089 m, and
//     ways come out SHORTER furnished on 138 of 2802 asked; the smallest squeeze that was real in play (the two charges) is 0.13 m of 0.27 to spare.
//     Under the grain the check cannot tell furniture from the search, and says nothing: that is what 'tight as built' on a line warns of.
//   - PASS: every way the level has without its furniture it has with it, none of the tight ones longer. A way that only the furnished level has is said.
//   A level's ways that are tight with NO furniture are counted on its line ('tight as built'): that is the level's own edge, not the furniture's.
// WHAT A LEVEL ALREADY OWES is in a file of its own, tools/qa/fixtures/wayround/<mission id>.txt (its README says how): `#` lines of why, then `<lost> <squeezed>`
// as the check counts them on seeds 1-3 (for both stands; or a line each, `shipped <lost> <squeezed>` and `frozen <lost> <squeezed>`, where they differ). A mission is red when it has MORE of either than its file (no file: 0 0);
// fewer says so and asks for the file to be lowered. The files are debts with a name on them, not a way to pass: those that exist were found the day the
// check was written. With --seeds other than 1-3 no file is read and any loss is red.
// WHAT IT LEAVES OUT, so nobody reads more into a green: the team's own guns (in play a way is also refused where the team has a line on it) and men who
// have left their posts. It asks "did the furniture take a way away", which is the question; whether a pair then goes is the fight's.
// A mission whose squads never get a question (one squad, men that stay in their rooms, rooms with one door) is listed as 'nothing to ask'.
// Run it on BOTH stands (TPS_PROFILE=shipped in front for the page's rules). On the frozen stand the defenders' squads are held off, so every mission says
// 'nothing to ask' there: that is the frozen stand's answer, and the shipped run is the one that asks. Lines are `- PASS — …` / `- FAIL — …`; exit 0 = all pass, 1 = a way lost,
// 2 = bad usage. THE CHECK'S OWN BREAK PATH: `WAYROUND_BREAK=1` asks the furnished side with the path limit at a tenth, as if every way had grown ten
// times as long, and must be red on every mission that has a way at all.
import { existsSync, readdirSync, readFileSync, writeFileSync } from 'node:fs';
import { spawn } from 'node:child_process';
import { resolve } from 'node:path';
const ROOT = resolve(import.meta.dir, '../..');
const argv = process.argv.slice(2), flag = (n: string) => { const i = argv.indexOf(n); if (i < 0) return undefined; const v = argv[i + 1]; argv.splice(i, 2); return v; };
type Row = { squad: string; man: string; from?: string; room: string; door: string; hold: boolean; team?: boolean; way: string | null; L: number; limit?: number; best?: number };
/** a way with less than this left to its limit is one a fight can take away: a man at his decision stands a stride or two off his mark */
const MARGIN_M = 2, SHARE = 0.2, grainOf = (cell: number) => cell * (Math.SQRT2 - 1) / 2;
const STAND_S = 4;

// ---------------------------------------------------------------- the child: one mission, one seed, as its environment has lived rooms
const one = flag('--one');
if (one) {
  const [id, seedS] = one.split(':'), seed = Number(seedS);
  const log = console.log; console.log = console.error = console.warn = () => { };
  const { standUp } = await import('./headless.ts');
  const { ALL_MISSIONS: SCENARIOS } = await import(`${ROOT}/src/scene/scenarioList.ts`);
  const { scenarioQuery } = await import(`${ROOT}/src/scene/scenarios.ts`);
  const { startFromQuery } = await import(`${ROOT}/src/game/scenarioRun.ts`);
  const { parseLevelData } = await import(`${ROOT}/src/scene/levelData.ts`);
  const { loadLevel } = await import(`${ROOT}/src/scene/levelLoad.ts`);
  const { withOfficeScript, isOfficeFamily } = await import(`${ROOT}/src/scene/level.ts`);
  const { squadWaysRound, squadsLive } = await import(`${ROOT}/src/game/enemysquads.ts`);
  const { PRESSURE } = await import(`${ROOT}/src/game/pressure.ts`);
  const { LIVED } = await import(`${ROOT}/src/scene/lived.ts`);
  const E = (SCENARIOS as any[]).find(e => e.id === id); if (!E) { log(JSON.stringify({ error: `no mission '${id}'` })); process.exit(0); }
  let out: { rows: Row[]; squads: boolean; lived: boolean; cell?: number; error?: string };
  try {
    const level = E.level ? (() => { const data = parseLevelData(readFileSync(resolve(ROOT, 'levels', E.level), 'utf8')); return isOfficeFamily(data) ? withOfficeScript(loadLevel(data)) : loadLevel(data); })() : undefined;
    Math.random = (() => { let s = (0x9137 + seed * 7919) >>> 0; return () => { s = (s + 0x6D2B79F5) >>> 0; let t = s; t = Math.imul(t ^ (t >>> 15), t | 1); t ^= t + Math.imul(t ^ (t >>> 7), t | 61); return ((t ^ (t >>> 14)) >>> 0) / 4294967296; }; })();
    const W: any = await standUp({ uploadEvery: 4, ...(level ? { level } : {}) }), g = W.game;
    let q = scenarioQuery(E) as string; if (E.kind === 'roomclear') q = q.replace(/(^|[?&])roomclear(=\d+)?(?=&|$)/, `$1roomclear=${seed}`);
    const { why } = await startFromQuery(g, new URLSearchParams(q)); if (why) throw new Error(`start refused: ${why}`);
    if (process.env.WAYROUND_BREAK === '1' && LIVED.on) PRESSURE.flankMaxM /= 10;
    // the room fight makes its men on its first tick, and a squad counts a man at home two seconds after he was last seen: the game stands for STAND_S with
    // nobody ordered anywhere (the team on its start marks), and is asked the moment a squad has a man at home
    let rows: Row[] = []; for (let f = 0; f < STAND_S * 60 && !rows.length; f++) { W.step(1 / 60); if (f % 6 === 5) rows = squadWaysRound(g); }
    out = { rows, squads: squadsLive(g), lived: !!LIVED.on, cell: g.col.nav.cell };
  } catch (e) { out = { rows: [], squads: false, lived: false, error: String((e as Error)?.message ?? e).slice(0, 300) }; }
  log(JSON.stringify(out)); process.exit(0);
}

// ---------------------------------------------------------------- the parent
const seedsArg = flag('--seeds') ?? '1-3', md = flag('--md'), jsonOut = flag('--json');
const seeds: number[] = []; for (const part of seedsArg.split(',')) { const m = /^(\d+)(?:-(\d+))?$/.exec(part); if (!m) { console.error(`wayround-check: --seeds ${seedsArg}: 1-3 or 1,2,5`); process.exit(2); } for (let s = +m[1]; s <= +(m[2] ?? m[1]); s++) seeds.push(s); }
const { ALL_MISSIONS: SCENARIOS } = await import(`${ROOT}/src/scene/scenarioList.ts`);
const levelName = (e: any) => String(e.level ?? 'office.json').replace(/\.json$/, '');
const want = argv.filter(a => !a.startsWith('--'));
const list = (SCENARIOS as any[]).filter(e => !want.length || want.includes(e.id) || want.includes(levelName(e)));
if (want.length && !list.length) { console.error(`wayround-check: no mission or level of ${want.join(', ')} — ${(SCENARIOS as any[]).map(e => e.id).join(', ')}`); process.exit(2); }
const child = (id: string, seed: number, lived: boolean) => new Promise<{ rows: Row[]; squads: boolean; lived: boolean; cell?: number; error?: string }>(res => {
  const p = spawn('bun', ['run', import.meta.path, '--one', `${id}:${seed}`], { cwd: ROOT, env: { ...process.env, TPS_LIVED: lived ? '1' : '0' }, stdio: ['ignore', 'pipe', 'pipe'] });
  let o = '', e = ''; p.stdout.on('data', (d: Buffer) => { o += d; }); p.stderr.on('data', (d: Buffer) => { e += d; });
  p.on('close', () => { const line = o.trim().split('\n').pop() ?? ''; try { res(JSON.parse(line)); } catch { res({ rows: [], squads: false, lived, error: (e || o).trim().split('\n').pop()?.slice(0, 300) ?? 'no answer' }); } });
});
type Job = { e: any; seed: number; lived: boolean; r?: Awaited<ReturnType<typeof child>> };
const jobs: Job[] = []; for (const e of list) for (const seed of seeds) for (const lived of [false, true]) jobs.push({ e, seed, lived });
{ let k = 0; const worker = async () => { while (k < jobs.length) { const j = jobs[k++]; j.r = await child(j.e.id, j.seed, j.lived); } }; await Promise.all(Array.from({ length: Math.min(4, jobs.length) }, worker)); }
const key = (r: Row) => `${r.squad} · ${r.man}${r.from && r.from !== 'his post' ? ` from his door ${r.from}` : ''} → ${r.room} with the team ${r.team ? 'IN it by' : 'at'} ${r.door}${r.hold ? ' (holds the other door)' : ' (goes in by the other door)'}`;
const STAND = process.env.TPS_PROFILE === 'shipped' ? 'shipped' : 'frozen';
const DEFAULT_SEEDS = '1-3', KNOWN_DIR = resolve(ROOT, 'tools/qa/fixtures/wayround'), known = new Map<string, [number, number]>(), knownBad: string[] = [];
if (seedsArg === DEFAULT_SEEDS && existsSync(KNOWN_DIR)) for (const f of readdirSync(KNOWN_DIR)) { if (!f.endsWith('.txt')) continue;
  const id = f.slice(0, -4), body = readFileSync(resolve(KNOWN_DIR, f), 'utf8').split('\n').map(l => l.trim()).filter(l => l && !l.startsWith('#')), rows = body.map(l => /^(?:(shipped|frozen) )?(\d+) (\d+)$/.exec(l));
  if (!body.length || rows.some(m => !m) || new Set(rows.map(m => m![1] ?? 'both')).size !== rows.length) knownBad.push(`${f}: wants '<lost> <squeezed>', or a line each for 'shipped <lost> <squeezed>' and 'frozen <lost> <squeezed>', after its # lines`);
  else if (!(SCENARIOS as any[]).some(e => e.id === id)) knownBad.push(`${f}: no mission '${id}'`);
  else { const m = rows.find(r => r![1] === STAND) ?? rows.find(r => !r![1]); if (m) known.set(id, [+m[2], +m[3]]); } }
let fails = 0; const table: { mission: string; level: string; asked: number; off: number; on: number; lost: number; squeezed: number; tight: number; gained: number; note: string }[] = [];
const stand = STAND;
console.log(`=== wayround-check: the defenders' way round with lived rooms off and on — the ${stand} stand, seed${seeds.length > 1 ? 's' : ''} ${seedsArg}${process.env.WAYROUND_BREAK === '1' ? ' — WAYROUND_BREAK=1' : ''} ===`);
for (const e of list) {
  const mine = jobs.filter(j => j.e === e), err = mine.find(j => j.r?.error), blank = { mission: e.id, level: levelName(e), asked: 0, off: 0, on: 0, lost: 0, squeezed: 0, tight: 0, gained: 0, note: '' };
  if (err) { fails++; console.log(`- FAIL — ${e.id}: could not be stood up (seed ${err.seed}, lived rooms ${err.lived ? 'on' : 'off'}): ${err.r!.error}`); table.push({ ...blank, note: 'could not be stood up' }); continue; }
  if (mine.some(j => j.lived !== j.r!.lived)) { fails++; console.log(`- FAIL — ${e.id}: the lived rooms' switch did not take in the child (TPS_LIVED)`); table.push({ ...blank, note: 'the switch did not take' }); continue; }
  let asked = 0, off = 0, on = 0, tight = 0; const lost: string[] = [], squeezed: string[] = [], gained: string[] = [];
  for (const seed of seeds) {
    const a = mine.find(j => j.seed === seed && !j.lived)!.r!.rows, b = mine.find(j => j.seed === seed && j.lived)!.r!.rows, B = new Map(b.map(r => [key(r), r]));
    const grain = grainOf(mine.find(j => j.seed === seed && j.lived)!.r!.cell ?? 0.5), m1 = (x: number) => x.toFixed(1), m2 = (x: number) => x.toFixed(2);
    for (const r of a) { asked++; const s = B.get(key(r)); if (r.way) off++; if (s?.way) on++;
      if (r.way && r.limit !== undefined && r.limit - r.L < MARGIN_M) tight++;
      if (r.way && !s?.way) lost.push(`seed ${seed}: ${key(r)}, ${m2(r.L)} m by ${r.way}; furnished ${!s ? 'he is not at home' : s.best === undefined ? 'there is nowhere to stand' : `the shortest is ${m2(s.best)} m of ${m1(s.limit ?? 0)} allowed`}`);
      else if (r.way && s?.way && s.limit !== undefined && r.limit !== undefined && s.limit - s.L < MARGIN_M && s.L - r.L > Math.max(grain, SHARE * (r.limit - r.L))) squeezed.push(`seed ${seed}: ${key(r)}, ${m2(r.L)} m → ${m2(s.L)} m of ${m1(s.limit)} allowed`);
      if (!r.way && s?.way) gained.push(`seed ${seed}: ${key(r)}`); }
  }
  const row = { ...blank, asked, off, on, lost: lost.length, squeezed: squeezed.length, tight, gained: gained.length };
  const own = tight ? `; ${tight} tight as built` : '';
  if (!asked) { row.note = 'nothing to ask'; console.log(`- PASS — ${e.id}: nothing to ask (no squad that may leave its rooms has another squad's room to go round to)`); }
  else if (lost.length || squeezed.length) { const all = [...lost.map(x => 'LOST ' + x), ...squeezed.map(x => 'SQUEEZED ' + x)], [kl, ks] = known.get(e.id) ?? [0, 0], worse = lost.length > kl || squeezed.length > ks; row.note = all[0];
    const said = `of ${off} ways round, the lived rooms' furniture takes ${lost.length} away and squeezes ${squeezed.length} (longer by more than the search's grain and a fifth of its room, and under ${MARGIN_M} m from the limit)`, file = known.has(e.id) ? `its file knows ${kl} and ${ks}` : 'it has no file';
    if (worse) { fails++; console.log(`- FAIL — ${e.id} (${levelName(e)}): ${said}; ${file} — ${asked} asked${own} — ${all.slice(0, 4).join(' · ')}${all.length > 4 ? ` · and ${all.length - 4} more` : ''}`); }
    else console.log(`- PASS — ${e.id} (${levelName(e)}): OWED, no worse than its file: ${said}; ${file}${lost.length < kl || squeezed.length < ks ? ' — FEWER than the file: lower tools/qa/fixtures/wayround/' + e.id + '.txt' : ''} (${asked} asked${own}) — ${all[0]}`); }
  else if (known.has(e.id) && (known.get(e.id)![0] || known.get(e.id)![1])) console.log(`- PASS — ${e.id} (${levelName(e)}): all ${off} ways round stand with lived rooms on, none of the tight ones longer — its file still knows ${known.get(e.id)!.join(' and ')}: PAID, delete tools/qa/fixtures/wayround/${e.id}.txt (${asked} asked${own})`);
  else console.log(`- PASS — ${e.id} (${levelName(e)}): all ${off} ways round stand with lived rooms on, none of the tight ones longer (${asked} asked${own}${gained.length ? `; ${gained.length} more only with them on` : ''})`);
  table.push(row);
}
for (const x of knownBad) { fails++; console.log(`- FAIL — tools/qa/fixtures/wayround/${x}`); }
if (md) writeFileSync(md, `| mission | level | asked | a way, lived off | a way, lived on | lost | squeezed | tight as built | only with lived on | first loss |\n|---|---|---|---|---|---|---|---|---|---|\n` + table.map(r => `| ${r.mission} | ${r.level} | ${r.asked} | ${r.off} | ${r.on} | ${r.lost} | ${r.squeezed} | ${r.tight} | ${r.gained} | ${r.note} |`).join('\n') + '\n');
if (jsonOut) writeFileSync(jsonOut, JSON.stringify({ stand, seeds, table }, null, 1));
console.log(fails ? `=== wayround-check: ${fails} FAILED ===` : `=== wayround-check: ALL PASS (${table.length} missions) ===`);
process.exit(fails ? 1 : 0);
