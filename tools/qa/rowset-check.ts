// ROWSET-CHECK: a mission row's own hour and weather (src/game/rowset.ts ROWSET; scene/scenarios.ts ScenarioEntry.hour / weather).
//   bun run tools/qa/rowset-check.ts            (both stands: TPS_PROFILE=shipped in front for the page's; `probes.ts rowset`, family `missions`)
//   ROWSET_ORDER_BREAK=1 bun run tools/qa/rowset-check.ts   must FAIL its last line: the address's reader (host/standup.ts addressBeforeGame) read with the rule run after the hour is read
// WHAT IT SAYS:
//   · the rule alone: a query that boots a row with its own hour and weather gets both, OVER an `hour=` / `weather=` it carried; the same level at another
//     seed and team size gets them too; a row on the bundled office has none whatever it names; a query that boots no row is left to the character
//   · the rows as shipped: only a Tutorial row with a level file of its own names an hour or a weather, each one a level can be started in, none on a level
//     built for snow, and the rows of one level file name the same
//   · a start through game/scenarioRun.ts startFromQuery (the harness's way in, and what the page's boot does after main.ts ran the same rule over its own
//     query): the game stands at the row's hour and in its weather, with `hour=dawn` on the query too, and the caller's query is not written on
//   · the mission sheet (ui/missions.ts): the Hour and Weather rows are not drawn for the row and H and J do nothing on it; its link is any row's link (no
//     key unpicked, the visitor's picks once picked); a row that names only an hour keeps its Weather row, whose note reads for the row's hour; one that
//     names only a weather keeps its Hour row, whose note says the weather is the mission's own
//   · main.ts runs the rule after the page's switches and the recipe's tables and before it reads the weather, the snow and the hour
// EXIT: 0 = every line holds · 1 = one does not. Headless: no GPU, nothing here saw a picture.
import './stand.ts';   // (the stand's word, written down before this file reads it)
import { readFileSync } from 'node:fs';
import { resolve } from 'node:path';

export async function rowsetCheck(): Promise<boolean> {
  const { standUp, ROOT } = await import('./headless.ts');
  const { withDom, El } = await import('./scenario-picker-probes.ts');
  const HR = await import(`${ROOT}/src/game/hours.ts`);
  const WK = await import(`${ROOT}/src/game/weatherKinds.ts`);
  const { rowSettings, rowHour } = await import(`${ROOT}/src/game/rowset.ts`);
  const { startFromQuery, quitScenario } = await import(`${ROOT}/src/game/scenarioRun.ts`);
  const { SCENARIOS, ALL_MISSIONS, LESSON_MISSIONS } = await import(`${ROOT}/src/scene/scenarioList.ts`);
  const { scenarioQuery } = await import(`${ROOT}/src/scene/scenarios.ts`);
  const { parseLevelData } = await import(`${ROOT}/src/scene/levelData.ts`);
  const { loadLevel } = await import(`${ROOT}/src/scene/levelLoad.ts`);

  const C = { log: console.log, error: console.error, warn: console.warn }, log = C.log;
  const hush = () => { console.log = console.error = console.warn = () => { }; }, unhush = () => { console.log = C.log; console.error = C.error; console.warn = C.warn; };
  let bad = 0;
  const say = (ok: boolean, what: string, got = '') => { log(`  ${ok ? 'PASS' : 'FAIL'}  ${what}${got ? ` — ${got}` : ''}`); if (!ok) bad++; };
  const qs = (s: string) => new URLSearchParams(String(s).replace(/^[^?]*\?/, ''));
  log(`=== rowset-check (the page\'s stand) ===`);
  // the rows the lines below borrow: a mission with a level file of its own, and a plain one on the bundled office; their own keys are put on for a line and
  // taken off after it
  const plainRow = (e: any) => e.kind === 'roomclear' && !e.hour && !e.weather;
  const L = SCENARIOS.find((e: any) => plainRow(e) && e.level && !WK.SNOW_LEVELS.includes(e.level.replace(/\.json$/, ''))), E = SCENARIOS.find((e: any) => plainRow(e) && !e.level);
  const base = String(scenarioQuery(L)), baseE = String(scenarioQuery(E));
  const own = (e: any, hour?: string, weather?: string) => { if (hour) e.hour = hour; else delete e.hour; if (weather) e.weather = weather; else delete e.weather; };
  try {
    { // the rule alone
      own(L, 'dusk', 'dust'); own(E, 'midday', 'fog');
      const a = qs(base), putA = rowSettings(a), b = qs(base + '&hour=dawn&weather=rain'), putB = rowSettings(b);
      const c = qs(base.replace(/roomclear(=\d+)?/, 'roomclear=7').replace(/&squad4\b|&squad=\d/, '') + '&squad=3'), putC = rowSettings(c);
      const o = qs(baseE), putO = rowSettings(o), officeHour = rowHour(E);
      const d = qs('?quality=high&mute'), putD = rowSettings(d);
      own(L); own(E);
      say(a.get('hour') === 'dusk' && a.get('weather') === 'dust' && putA.hour === 'dusk' && putA.weather === 'dust', 'a query that boots the row gets the row\'s own hour and weather', `${a}`);
      say(b.get('hour') === 'dusk' && b.get('weather') === 'dust' && putB.hour === 'dusk' && b.getAll('hour').length === 1, 'the row\'s own win over an hour and a weather the query carried', `${b}`);
      say(c.get('hour') === 'dusk' && c.get('weather') === 'dust' && putC.hour === 'dusk', 'the same level at another seed and team size is started in them too', `${c}`);
      say(`${o}` === `${qs(baseE)}` && !putO.hour && officeHour === null, 'a row on the bundled office has none, whatever it names', `${o}`);
      say(`${d}` === `${qs('?quality=high&mute')}` && !putD.hour && !putD.weather, 'a query that boots no row is left to the character', `${d}`);
    }
    { // the rows as shipped
      const rows = [...new Set([...ALL_MISSIONS, ...LESSON_MISSIONS])] as any[], named = rows.filter(e => e.hour || e.weather);
      const wrong = named.filter(e => e.group !== 'tutorial' || !e.level || (e.hour && !HR.HOUR_IDS.includes(e.hour)) || (e.weather && !WK.isWeatherKind(e.weather)) || (e.weather && WK.SNOW_LEVELS.includes(String(e.level ?? '').replace(/\.json$/, ''))));
      const split = named.filter(e => rows.some(x => x.level === e.level && (x.hour !== e.hour || x.weather !== e.weather)));
      say(!wrong.length && !split.length, 'only a Tutorial row with a level file of its own names an hour or a weather, each one a level can be started in, none on a level built for snow, and the rows of one level file name the same', `${named.length} row(s) name one: ${named.map(e => `${e.id} ${e.hour ?? '·'}/${e.weather ?? '·'}`).join(', ') || 'none'}${wrong.length ? ` · wrong: ${wrong.map(e => e.id).join(', ')}` : ''}${split.length ? ` · a level file whose rows differ: ${split.map(e => e.id).join(', ')}` : ''}`);
    }
    { // a start through startFromQuery
      hush(); const W: any = await standUp({ uploadEvery: 4, level: loadLevel(parseLevelData(readFileSync(resolve(ROOT, 'levels', L.level), 'utf8'))) }); unhush();
      const g = W.game, at = () => `${HR.hourOf(g)}/${g.weather?.kind ?? 'none'}`, at0 = at();
      const start = (q: string | URLSearchParams) => { hush(); const r = startFromQuery(g, q); unhush(); const v = r.why ? `refused: ${r.why}` : at(); hush(); quitScenario(g); unhush(); return v; };
      own(L, 'dusk', 'dust'); const mine = start(base), theirs = qs(base + '&hour=dawn'), was = `${theirs}`, asked = start(theirs), kept = `${theirs}` === was;
      own(L);
      say(mine === 'dusk/dust' && asked === 'dusk/dust', 'started by its own query, the game stands at the row\'s hour and in its weather, with another hour on the query too', `${mine} · with hour=dawn: ${asked} (before: ${at0})`);
      say(kept, 'the caller\'s query is not written on', was);
    }
    {
      // (the tape's tables are a RECIPE's since the page can also play a game server's ticks: main.ts lays them with its own recipeTables() at each point of the boot
      //  that wrote over them, and the first of those calls is the one the rule must come after. ROWSET_ORDER_BREAK=1 reads the source with the rule's call moved
      //  below the hour's: the line must FAIL.)
      const main = readFileSync(resolve(ROOT, 'src/main.ts'), 'utf8'), src0 = readFileSync(resolve(ROOT, 'src/host/standup.ts'), 'utf8'), call = /^ *rowSettings\(q\);[^\n]*\n/m.exec(src0)?.[0] ?? '', hourAt = /^ *const hour = hourAsked\(q\);[^\n]*\n/m.exec(src0);
      const src = process.env.ROWSET_ORDER_BREAK === '1' && call && hourAt ? (x => x.slice(0, x.indexOf(hourAt[0]) + hourAt[0].length) + call + x.slice(x.indexOf(hourAt[0]) + hourAt[0].length))(src0.replace(call, '')) : src0;
      const p = main.indexOf('pageSwitches(q)'), t = main.search(/^ *recipeTables\(\);/m), a = main.indexOf('addressBeforeGame(q)'), f = src.indexOf('export function addressBeforeGame'), body = f < 0 ? '' : src.slice(f, src.indexOf('\n}', f)), r = body.indexOf('rowSettings(q)'), w = body.indexOf('weatherAsked(q)'), h = body.indexOf('hourAsked(q)'), s = body.indexOf('snowLevelOfQuery(q)');
      say(p > 0 && t > 0 && p < a && t < a && r > 0 && r < w && r < h && r < s, 'main.ts reads the address (host/standup.ts addressBeforeGame) after the page\'s switches and the recipe\'s tables (a tape\'s, or a game server\'s), and that function runs the rule before it reads the weather, the snow and the hour');
    }
  } finally { own(L); own(E); unhush(); }
  log(bad ? `=== rowset-check: FAIL (${bad} line(s) do not hold) ===` : '=== rowset-check: ALL PASS ===');
  return !bad;
}
if (import.meta.main) process.exit((await rowsetCheck()) ? 0 : 1);
