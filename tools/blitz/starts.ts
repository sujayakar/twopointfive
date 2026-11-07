// THE TABLE OF STARTS, AND THE TOOL THAT STANDS EVERY ROW OF IT (tools/blitz/DELETION.md, "The gate of starts"). A game is stood in TypeScript and crosses
// whole into the Rust game at its first step, so a start is the one place where a rule's TypeScript still runs rightly, and the deletion keeps exactly the
// rules that a start was SEEN to run. A start nobody stood before the deletion would meet a rule with no body, and throw. This tool is what makes sure no
// player meets that first: every way a game can be started is a row of tools/blitz/starts.json, written from the game's own lists (never by hand), and
// each row is stood as the page stands it and stepped one second under the Rust game.
//   bun run tools/blitz/starts.ts --table                       write starts.json from the lists as they are now (missions, squad sizes, weather kinds, hours, levers)
//   bun run tools/blitz/starts.ts [--jobs 4] [--only word,word] [--json out.json]    stand every row (a process a row, as a page is a load a start); exit 1 on a throw
//   bun run tools/blitz/starts.ts --one <row id>                one row in this process, its outcome as one line of JSON
//   bun run tools/blitz/starts.ts --against before.json         … and exit 1 only on a row whose outcome is not the one that file has (the tree before the deletion)
// ON THE CENSUS'S TREE (tools/blitz/delete-census.sh) the same run is the census of starts: TPS_RAN_OUT names the file every row's counters go to.
// ON THE DELETED TREE it is the gate: no row may throw what it did not throw before, and none may say "has no TypeScript body".
// WHAT A ROW IS: the words of an address (`search`), and for a few rows a deed after the first second (`then`): the pause card's restart, a quit to the
// title and a second start in the same process, a checkpoint taken and put back. THE MIRROR: the lines of src/main.ts that a start runs before and after
// the game is built, in their order, as far as they touch the simulation (the picture's words are left out: no row of the picture reaches a rule).
// NOT HERE: the Team & kit page's own small game (`?kitview`, src/host/kitView.ts), which only a page can stand: tools/blitz/delete-page.ts.
import { readFileSync, writeFileSync, existsSync, appendFileSync } from 'node:fs';
import { resolve } from 'node:path';

// (STARTS_ROOT: another tree laid out as this one, stood with this tool and this table: the census's copy, a deleted tree beside the checkout)
const ROOT = process.env.STARTS_ROOT ? resolve(process.env.STARTS_ROOT) : resolve(import.meta.dir, '../..'), TABLE = resolve(import.meta.dir, 'starts.json');
const A = process.argv.slice(2), has = (n: string) => A.includes(n), opt = (n: string) => { const i = A.indexOf(n); return i < 0 ? undefined : A[i + 1]; };
interface Row { id: string; search: string; then?: 'restart' | 'second' | 'save' }
interface Out { id: string; ok: boolean; ticks: number; /** a host of the Rust game stood on the game that was stepped */ rust?: boolean; stage?: string; refused?: string | null; error?: string; /** the throw is a deleted function's: the deletion's fault, not the tool's or the tree's */ noBody?: boolean }

// ---- the table, from the game's own lists
if (has('--table')) {
  const { ALL_MISSIONS } = await import(`${ROOT}/src/scene/scenarioList.ts`), { scenarioQuery } = await import(`${ROOT}/src/scene/scenarios.ts`);
  const { WEATHER_KINDS } = await import(`${ROOT}/src/game/weatherKinds.ts`), { HOUR_IDS } = await import(`${ROOT}/src/game/hours.ts`), { LEVERS, TOLD_LEVER } = await import(`${ROOT}/src/game/difficulty.ts`);
  const rows: Row[] = [], add = (id: string, search: string, then?: Row['then']) => rows.push(then ? { id, search, then } : { id, search });
  // (the plain night is no row of the list: the start card's own Start, and `?solo` off the loadout screen)
  const missions: { id: string; q: string }[] = [{ id: 'night', q: '' }, { id: 'night-solo', q: 'solo' }, ...ALL_MISSIONS.map((e: any) => ({ id: e.id as string, q: String(scenarioQuery(e)).replace(/^\?/, '') }))];
  for (const m of missions) { const w = (words: string) => [m.q, words].filter(Boolean).join('&');
    add(m.id, m.q);
    for (let n = 0; n <= 4; n++) add(`${m.id}/squad-${n}`, w(`squad=${n}`));
    for (const k of WEATHER_KINDS) add(`${m.id}/weather-${k}`, w(`weather=${k}`));
    for (const h of HOUR_IDS) add(`${m.id}/hour-${h}`, w(`hour=${h}`));
    add(`${m.id}/hard`, w('difficulty=hard')); add(`${m.id}/standard`, w('difficulty=standard'));
    for (const L of [...LEVERS, TOLD_LEVER]) for (const r of L.rungs) add(`${m.id}/${L.param}-${r.id}`, w(`${L.param}=${r.id}`));
    add(`${m.id}/fog-off`, w('fog=off')); add(`${m.id}/digin`, w('digin')); add(`${m.id}/pressure`, w('pressure')); add(`${m.id}/fogcone`, w('fogcone'));
    add(`${m.id}/hard+rain+dusk`, w('difficulty=hard&weather=rain&hour=dusk'));
    add(`${m.id}/then-restart`, m.q, 'restart'); add(`${m.id}/then-second`, m.q, 'second'); add(`${m.id}/then-save`, m.q, 'save'); }
  writeFileSync(TABLE, '[\n' + rows.map(r => JSON.stringify(r)).join(',\n') + '\n]\n'); console.log(`${rows.length} starts of ${missions.length} missions → tools/blitz/starts.json`); process.exit(0); }

if (!existsSync(TABLE)) { console.error('no tools/blitz/starts.json: bun run tools/blitz/starts.ts --table'); process.exit(2); }
const ROWS: Row[] = JSON.parse(readFileSync(TABLE, 'utf8'));

// ---- one row, in this process
if (has('--one')) { const id = opt('--one')!, row = ROWS.find(r => r.id === id); if (!row) { console.error(`no start '${id}' in the table`); process.exit(2); }
  const out: Out = { id, ok: false, ticks: 0 }; let g: any;
  // (the lines a stand prints are not this tool's: its one line goes out last, marked)
  try {
    const q = new URLSearchParams(row.search);
    const { standUp } = await import(`${ROOT}/tools/qa/headless.ts`);   // (the page's stand: profile.ts shippedBoot has run its `pageSwitches` of no word)
    const { pageSwitches } = await import(`${ROOT}/src/host/profile.ts`), { LESSON_MISSIONS } = await import(`${ROOT}/src/scene/scenarioList.ts`), { lessonGroup } = await import(`${ROOT}/src/scene/scenarios.ts`), { kitLessonPage, kitSoloIf } = await import(`${ROOT}/src/game/kit.ts`);
    const { initDifficulty } = await import(`${ROOT}/src/game/difficulty.ts`), { rowSettings } = await import(`${ROOT}/src/game/rowset.ts`), { initWeather, snowLevelOfQuery, WEATHER_BOOT } = await import(`${ROOT}/src/game/weatherKinds.ts`), { initHour } = await import(`${ROOT}/src/game/hours.ts`);
    const { startFromQuery, quitScenario } = await import(`${ROOT}/src/game/scenarioRun.ts`), { parseLevelData } = await import(`${ROOT}/src/scene/levelData.ts`), { lookLevelFile } = await import(`${ROOT}/src/scene/look.ts`), { loadLevel } = await import(`${ROOT}/src/scene/levelLoad.ts`);
    const levelOfFile = (file: string): any => { const data = parseLevelData(readFileSync((f => existsSync(f) ? f : resolve(ROOT, 'levels', file))(resolve(ROOT, 'levels', lookLevelFile(file))), 'utf8')); return loadLevel(data); };
    // BEFORE THE GAME EXISTS, in main.ts's order
    pageSwitches(q);
    const lessonRow = LESSON_MISSIONS.find((e: any) => e.level === q.get('levelFile')) ?? null; kitLessonPage(lessonRow && lessonGroup(lessonRow.group) ? lessonRow : null);
    initDifficulty(q, lessonGroup(lessonRow?.group));
    rowSettings(q);
    initWeather(q); if (snowLevelOfQuery(q)) WEATHER_BOOT.kind = 'snow';
    initHour(q);
    const file = q.get('levelFile'), level = file && /^[\w.-]+\.json$/.test(file) ? levelOfFile(file) : undefined;
    // THE ONE STAND-UP, then the scenario the address names, then the plain night's own card
    const W = await standUp({ uploadEvery: 4, ...(level ? { level } : {}), search: row.search ? `?${row.search}` : '' }); g = W.game;
    const second = (): string | null => { const b = startFromQuery(g, q); kitSoloIf(g, q.has('solo') || !g.quiet.active); return b.spec && b.why ? String(b.why) : null; };
    out.refused = second();
    const step = (n: number) => { for (let i = 0; i < n; i++) { W.step(1 / 60); out.ticks++; } };
    step(60);
    if (row.then === 'restart') { g.restart(); step(60); }
    else if (row.then === 'second') { quitScenario(g); step(30); out.refused = second(); step(60); }
    else if (row.then === 'save') { const r = g.saves.write(g, 'manual', 'starts'); step(30); const k = g.saves.newest?.(false)?.key ?? g.saves.slots()[0]?.key; if (!k) throw new Error(`no checkpoint to put back (the write said ${JSON.stringify(r)?.slice(0, 120)})`); const why = g.saves.load(g, k); if (why) throw new Error(`the checkpoint was refused: ${why}`); step(60); }
    out.ok = true; out.stage = String(g.mission?.stage ?? ''); out.rust = !!g.rustHost;
  } catch (e) { const m = e instanceof Error ? `${e.message}\n${(e.stack ?? '').split('\n').slice(1, 5).join('\n')}` : String(e); out.error = m.slice(0, 1200); out.noBody = /has no TypeScript body|went with the tick/.test(m); out.stage = String(g?.mission?.stage ?? ''); }
  process.stdout.write(`\nSTART ${JSON.stringify(out)}\n`); process.exit(out.ok ? 0 : 1); }

// ---- every row, a process each
const only = (opt('--only') ?? '').split(',').filter(Boolean), jobs = Math.max(1, Number(opt('--jobs') ?? 4));
const todo = ROWS.filter(r => !only.length || only.some(w => r.id === w || r.id.startsWith(w + '/') || r.id.endsWith('/' + w)));
const results: Out[] = [], t0 = Date.now(); let next = 0;
async function worker(): Promise<void> { for (;;) { const row = todo[next++]; if (!row) return;
    const p = Bun.spawn(['bun', 'run', import.meta.path, '--one', row.id], { cwd: ROOT, env: { ...process.env, TPS_GAME: process.env.TPS_GAME ?? 'rust' }, stdout: 'pipe', stderr: 'pipe' });
    const [text, err] = await Promise.all([new Response(p.stdout).text(), new Response(p.stderr).text()]); await p.exited;
    const line = text.split('\n').filter(l => l.startsWith('START ')).pop(); let out: Out;
    try { out = JSON.parse(line!.slice(6)); } catch { out = { id: row.id, ok: false, ticks: 0, error: `the row's process left no line (exit ${p.exitCode}): ${(err || text).trim().split('\n').slice(-3).join(' / ').slice(0, 600)}`, noBody: /has no TypeScript body|went with the tick/.test(err + text) }; }
    results.push(out); if (!out.ok) console.log(`  ${out.noBody ? 'NO BODY' : 'threw  '}  ${out.id}: ${(out.error ?? '').split('\n')[0].slice(0, 300)}`); } }
await Promise.all(Array.from({ length: jobs }, worker));
results.sort((a, b) => a.id < b.id ? -1 : 1);
if (opt('--json')) writeFileSync(opt('--json')!, JSON.stringify(results, null, 0).replace(/\},\{/g, '},\n{') + '\n');
const bad = results.filter(r => !r.ok), noBody = bad.filter(r => r.noBody), refused = results.filter(r => r.ok && r.refused);
const notRust = results.filter(r => r.ok && !r.rust); if (notRust.length) console.log(`${notRust.length} rows stepped a game with NO host of the Rust game on it (not a start of the Rust game): ${notRust.slice(0, 12).map(r => r.id).join(' ')}`);
console.log(`${results.length} starts stood in ${((Date.now() - t0) / 1000).toFixed(0)} s: ${results.length - bad.length} stepped a second and more · ${bad.length} threw (${noBody.length} of them on a function with no TypeScript body) · ${refused.length} were refused by the scenario itself and played the level as it stood`);
let red = bad.length + notRust.length;
if (opt('--against')) { const was = new Map<string, Out>((JSON.parse(readFileSync(opt('--against')!, 'utf8')) as Out[]).map(o => [o.id, o])); const first = (o?: Out) => o ? (o.ok ? `ok ${o.stage ?? ''} ${o.refused ?? ''}` : (o.error ?? '').split('\n')[0]) : 'not stood';
  const moved = results.filter(r => first(r) !== first(was.get(r.id))); red = moved.length + noBody.length;
  console.log(`against ${opt('--against')}: ${moved.length} rows end another way than before${moved.length ? ':' : ''}`); for (const r of moved.slice(0, 40)) console.log(`  ${r.id}: was [${first(was.get(r.id)).slice(0, 140)}] is [${first(r).slice(0, 200)}]`); }
process.exit(red ? 1 : 0);
