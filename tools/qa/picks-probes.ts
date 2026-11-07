// The 'picks' family (THE SHORT LIST — scene/picks.ts PICKS, ui/picks.ts for ui/menu.ts's cards, main.ts's ?pick= redirect), run through probes.ts:
//   bun run tools/qa/probes.ts picks | picks-list | picks-card | picks-boot          (or this file: bun run tools/qa/picks-probes.ts [name …])
//   bun run tools/qa/probes.ts picks-play                          (minutes; not in the family) every scenario row played to its end by the scripted commander, its settings on
// PASS / FAIL each, all headless. Nothing here looks at a picture.
//   picks-list   the table as data: ids unique and URL-safe, every mission on the build, a plain line on each, five to seven rows; each row's URL names its
//                mission, team size, difficulty, hour, weather and switches OUT (nothing left to what a machine keeps), reads back to that row and to no other,
//                keeps a visitor's own flags and drops an older run's settings; `?pick=<id>` and `?pick=<place>` answer with that URL once and never twice;
//                the row after each; a quit out of a pick takes its settings off the URL and leaves every other URL as the build's quit leaves it; the rules the
//                maintainer set for the list (a four-person loud breach on a squad map first; the barracks by day and every other row at its level's own night
//                and weather; no kit on a row); and THE ONE CHOICE: every scenario row's URL at the other difficulty differs in that one value and is the same pick
//   picks-card   the real Menu on a fake DOM: the pause card of a run a pick started offers the next row, carrying a difficulty chosen over the row's own;
//                with the switch off no 'next'
//   picks-boot   every row's URL booted in a process of its own with the page's own start-up order (the URL where modules read it, the difficulty, the weather,
//                the hour, the level file, startFromQuery): it starts unrefused as its mission on its level, with the row's team, difficulty, hour, weather and
//                switches in force (on a machine that keeps the OTHER difficulty; the harness stands most page-default feature switches off, so 'in force' is the
//                URL's settings, not every default of the page), no console error — and RUNS: thirty seconds of the simulation with nobody at the keys, nothing thrown, the game's clock and digest moving
//   picks-play   every scenario row played from its staging to its END by tools/qa/playthrough.ts's scripted commander at the row's team size, difficulty, hour and weather,
//                three seeds each, STOOD UP AS THE PAGE OPENS (glass on, the movement pass, turn weight, destruction on, defenders carry frags — the env and --set
//                the one build's page defaults need under the old stand; on the page's they are what it has): no run errors, every run
//                reports the row's own difficulty and hour, and every run ends (won or lost, never an error). The `pair` switch reads
//                the page's URL and is not in these runs (picks-boot holds it on); the solo night has no scripted player (picks-boot holds its boot)
import { resolve } from 'node:path';
import { readFileSync, rmSync } from 'node:fs';
import { groupsFit } from './listed.ts';

const ROOT = resolve(import.meta.dir, '../..');
const TAG = 'PICKS-BOOT ';
const DT = 1 / 60;
const { mulberry32: seeded } = await import(`${ROOT}/src/math/dice.ts`);

// ───────────────────────────────────────────────────────────── THE CHILD: one page load of a query ─────────────────────────────────────────────────────────────
// (first in the file: the page's URL has to be where modules read it BEFORE any game module is imported — game/pairup.ts and its like read location.search once)
if (import.meta.main && process.argv.includes('--boot')) {
  const query = process.argv[process.argv.indexOf('--boot') + 1] ?? '', secs = Number(process.argv[process.argv.indexOf('--secs') + 1]) || 30;
  (globalThis as any).location = { search: query, pathname: '/', hash: '' };
  // (--kept hard: a machine that KEEPS a difficulty — the one store the page reads before the URL's flag; every other key reads as never stored)
  if (process.argv.includes('--kept')) { const kept = process.argv[process.argv.indexOf('--kept') + 1]; (globalThis as any).localStorage = { getItem: (k: string) => (k === 'twopointsix.difficulty' ? kept : null), setItem() {}, removeItem() {} }; }
  const log = console.log, errors: string[] = []; console.log = console.warn = () => {}; console.error = (...a: unknown[]) => { errors.push(a.map(String).join(' ').split('\n')[0]); };
  const R: any = { query, errors, threw: null };
  try {
    const q = new URLSearchParams(query), lf = q.get('levelFile'), levelFile = lf && /^[\w.-]+\.json$/.test(lf) ? lf : null;
    const { standUp } = await import('./headless.ts');
    const { initDifficulty, difficultyNow } = await import(`${ROOT}/src/game/difficulty.ts`);
    const { initWeather } = await import(`${ROOT}/src/game/weatherKinds.ts`);
    const { initHour, hourOf } = await import(`${ROOT}/src/game/hours.ts`);
    const { startFromQuery, runningScenario, activeScenario, teamOf } = await import(`${ROOT}/src/game/scenarioRun.ts`);
    const { parseLevelData } = await import(`${ROOT}/src/scene/levelData.ts`);
    const { loadLevel } = await import(`${ROOT}/src/scene/levelLoad.ts`);
    const { PAIRUP } = await import(`${ROOT}/src/game/pairup.ts`);
    const { SILENT, squadSilent } = await import(`${ROOT}/src/game/silent.ts`);
    const { digestGame } = await import(`${ROOT}/src/game/digest.ts`);
    const { hashStr } = await import(`${ROOT}/src/game/dialogue.ts`).catch(() => ({ hashStr: null as any }));
    // main.ts's order: the difficulty, the weather and the hour are read before the Game exists; the level the URL names; the Game; the scenario
    initDifficulty(q); initWeather(q); initHour(q);
    Math.random = seeded(0x51c4);
    const level = levelFile ? (() => { const data = parseLevelData(readFileSync(resolve(ROOT, 'levels', levelFile), 'utf8')); return loadLevel(data); })() : undefined;
    const W = await standUp({ uploadEvery: 4, ...(level ? { level } : {}) });
    const g = W.game; g.godMode = true;
    SILENT.on = true;   // (the page's: the harness stands the game up with the silent rows off — src/host/standup.ts)
    if (q.has('silentstart')) SILENT.start = true;   // (main.ts's line, copied: the flag is the page's — tools/qa/startpage-look.cjs holds the page's own)
    const { why } = startFromQuery(g, q);
    const run = runningScenario(g, levelFile), ops: any[] = teamOf(g);
    const hostile = () => g.guards.filter((x: any) => x.unit && x.team === 'hostile' && x.state !== 'dead').length;
    const word = (): string => { const d = JSON.stringify(digestGame(g, true)); return hashStr ? hashStr(d).toString(16) : String(d.length); };
    Object.assign(R, { why, active: activeScenario(g), kind: run?.kind ?? null, entry: run?.entry?.id ?? null, squad: run?.squad ?? 0, operatives: ops.filter((x: any) => x.state !== 'dead').length, hostiles: hostile(),
      hostages: g.rescue.active ? g.rescue.hostages.length : 0, staff: g.guards.filter((x: any) => !x.unit).length, civilians: g.civilians?.length ?? 0, difficulty: difficultyNow(), hour: hourOf(g), weather: g.weather.kind ?? null,
      pair: !!PAIRUP.on, boxes: g.level.boxes?.length ?? g.world?.statics?.length ?? 0, levelFile, t0: g.time, d0: word() });
    // IT RUNS: the simulation run on with nobody at the keys (whatever the scenario does with thirty seconds of that is its own business: this holds only that
    // nothing throws and the game moves — picks-play is the run to an end)
    const sam0 = [...g.player.char.pos];
    for (let i = 0; i < Math.round(secs / DT); i++) { g.player.visibility = 0.05; W.step(DT); if (i === 30) R.silent = !!squadSilent(g); }
    Object.assign(R, { t1: g.time, d1: word(), operativesAfter: teamOf(g).filter((x: any) => x.state !== 'dead').length, hostilesAfter: hostile(), samDown: !!g.player.down,
      samMoved: Math.hypot(g.player.char.pos[0] - sam0[0], g.player.char.pos[2] - sam0[2]), stage: g.mission?.stage ?? null });
  } catch (e: any) { R.threw = String(e?.stack ?? e).split('\n').slice(0, 4).join(' | '); }
  log(TAG + JSON.stringify(R));
  process.exit(0);
}

const { standUp } = await import('./headless.ts');
const { withDom, El } = await import('./scenario-picker-probes.ts') as typeof import('./scenario-picker-probes.ts');
type El = InstanceType<typeof El>;
const P = await import(`${ROOT}/src/scene/picks.ts`);
const { SCENARIOS, ALL_MISSIONS, specOfQuery, quitSearch } = await import(`${ROOT}/src/scene/scenarioList.ts`);
const { setDifficulty, squadsOf } = await import(`${ROOT}/src/game/difficulty.ts`);
const { HOUR_IDS } = await import(`${ROOT}/src/game/hours.ts`);
const { WEATHER_KINDS } = await import(`${ROOT}/src/game/weatherKinds.ts`);

type Verdict = (ok: boolean, label: string, detail: string) => void;
function family(title: string): { verdict: Verdict; done: () => boolean } {
  console.log(`\n=== ${title} ===`); let all = true;
  return { verdict: (ok, label, detail) => { all &&= ok; console.log(`- ${ok ? 'PASS' : 'FAIL'} — ${label}\n    ${detail}`); }, done: () => { console.log(`=== ${title.split(':')[0]}: ${all ? 'ALL PASS' : 'FAILURES above'} ===`); return all; } };
}
const quiet = async <T>(fn: () => Promise<T> | T): Promise<T> => { const log = console.log, err = console.error, warn = console.warn; console.log = console.error = console.warn = () => {}; try { return await fn(); } finally { console.log = log; console.error = err; console.warn = warn; } };
const first = (r: El, cls: string) => r.all(e => e.cls.has(cls))[0];
const buttons = (root: El) => root.all(e => e.tagName === 'BUTTON');
const buttonOf = (root: El, text: string) => buttons(root).find(b => b.textContent === text);
const startsWith = (root: El, text: string) => buttons(root).find(b => b.textContent.startsWith(text));
const paramsOf = (href: string) => new URLSearchParams(href.replace(/^[^?]*\?/, ''));
const otherOf = (d: string | null) => (d === 'hard' ? 'standard' : 'hard');
/** what a pick's settings say its URL must carry — written here from the row, not from pickHref */
function wantOf(p: any, d?: string): { kind: string | null; level: string; squad: number; difficulty: string | null; hour: string | null; weather: string | null; flags: string[] } {
  const e: any = P.pickEntry(p);
  return { kind: e ? e.kind : null, level: e?.level ?? '', squad: e ? p.squad ?? e.squad : 0, difficulty: e ? d ?? p.difficulty ?? 'standard' : null, hour: p.hour && p.hour !== 'night' ? p.hour : null, weather: p.weather ?? null, flags: p.flags ?? [] };
}
function hrefSays(p: any, href: string, d?: string): string | null {
  const w = wantOf(p, d), q = paramsOf(href), s = specOfQuery(q);
  if (w.kind === null) { if (s || !q.has('solo')) return 'not the solo night'; }
  else if (!s || s.kind !== w.kind || (s.level ?? '') !== w.level || s.squad !== w.squad) return `names ${s ? `${s.kind} on ${s.level ?? 'the office'} with ${s.squad}` : 'no scenario'}, not ${w.kind} on ${w.level || 'the office'} with ${w.squad}`;
  if ((q.get('difficulty') ?? null) !== w.difficulty || q.has('hard')) return `difficulty ${q.get('difficulty')}, not ${w.difficulty}`;
  if ((q.get('hour') ?? null) !== w.hour) return `hour ${q.get('hour')}, not ${w.hour}`;
  if ((q.get('weather') ?? null) !== w.weather) return `weather ${q.get('weather')}, not ${w.weather}`;
  for (const f of w.flags) { const [k, v = ''] = f.split('='); if (!q.has(k) || q.get(k) !== v) return `switch ${f} missing`; }
  if (q.get('pick') !== p.id) return `pick=${q.get('pick')}`;
  return null;
}

// ───────────────────────────────────────────────────────────── list ─────────────────────────────────────────────────────────────
export async function picksList(): Promise<boolean> {
  const F = family('picks-list: the short list as data — every row a mission on the build, its settings written out on its own URL');
  const L: any[] = P.PICKS, MS = await import(`${ROOT}/src/ui/missions.ts`);
  F.verdict(L.length >= 5 && L.length <= 7 && new Set(L.map(p => p.id)).size === L.length && L.every(p => /^[a-z0-9]+(-[a-z0-9]+)*$/.test(p.id) && !/^\d+$/.test(p.id)),
    'five to seven rows, each under an id of its own that is safe on a URL (and never a bare number: that is a place on the list)', L.map((p, i) => `${i + 1} ${p.id}`).join(' · '));
  { const bad = L.filter(p => P.pickEntry(p) === undefined), ids = MS.missionList().map((m: any) => m.id);
    F.verdict(bad.length === 0 && L.every(p => ids.includes(p.mission)) && P.SOLO_ID === MS.SOLO_NIGHT.id && MS.SOLO_KEY === 'solo', "every row's mission is on the mission list (the solo night under the list's own id and flag)",
      L.map(p => `${p.id} → ${p.mission}`).join(' · ') + (bad.length ? ` — NOT ON THE BUILD: ${bad.map(p => p.mission).join(', ')}` : '')); }
  // THE ONE STARTING PAGE's rows (ui/startpage.ts): off in a harness (no window to listen on), so every older probe measures the cards; its order as data
  // (IN THE LIST'S GROUPS since 10-11: every mission once under its own group, the groups in the list's order, and inside a group the short list's missions
  // first, in play order, then the rest in the list's own; ?nogroups = the short list on top as it was)
  { const SP = await import(`${ROOT}/src/ui/startpage.ts`), all = MS.missionList(), { list, first: n } = SP.startPageList(), top = [...new Set(L.map(p => p.mission))];
    const groups = [...new Set(all.map((m: any) => m.group))], want = groups.flatMap(g => { const of = all.filter((m: any) => m.group === g); return [...top.flatMap(id => of.filter((m: any) => m.id === id)), ...of.filter((m: any) => !top.includes(m.id))]; });
    F.verdict(SP.startPageWanted('') === true && SP.startPageWanted('?roomclear') === false && n === 0 && list.length === all.length && new Set(list.map((m: any) => m.id)).size === all.length && want.every((m: any, i: number) => list[i].id === m.id && list[i].group === m.group)
      && top.every(id => SP.pickOfMission(id)?.mission === id) && list.filter((m: any) => !top.includes(m.id)).every((m: any) => SP.pickOfMission(m.id) === null) && groupsFit(MS, list, groups as string[]),   // (listed.ts: no group draws more than eight; a row cut from the list is not counted)
      "the one starting page is the way in on an address that names nothing; its rows as data: the list's groups in their order, inside a group the short list's missions first in play order, then the rest in the list's own, each mission exactly once and no row outside a group",
      `${list.length} of ${all.length}:\n    ${groups.map(g => `${g}: ${list.filter((m: any) => m.group === g).map((m: any) => (top.includes(m.id) ? '*' : '') + m.id).join(', ')}`).join('\n    ')}`);
    MS.GROUPS_UI.on = false; let flat: any, allFlat: any[]; try { allFlat = MS.missionList(); flat = SP.startPageList(); } finally { MS.GROUPS_UI.on = true; }
    F.verdict(flat.first === top.length && top.every((id, i) => flat.list[i].id === id) && flat.list.length === allFlat.length && new Set(flat.list.map((m: any) => m.id)).size === allFlat.length
      && allFlat.filter((m: any) => !top.includes(m.id)).every((m: any, i: number) => flat.list[flat.first + i].id === m.id),
      "?nogroups: the short list's missions on top in play order, then every other mission in the list's own order, each exactly once",
      `${flat.first} + ${flat.list.length - flat.first}: ${flat.list.slice(0, flat.first).map((m: any) => m.id).join(', ')} | ${flat.list.slice(flat.first).map((m: any) => m.id).join(', ')}`); }
  F.verdict(L.every(p => typeof p.line === 'string' && p.line.length >= 30 && p.line.length <= 110 && !/[—;]/.test(p.line) && /[.?!]$/.test(p.line)), 'one plain line on each: 30 to 110 characters, a whole sentence, no dash or semicolon',
    L.map(p => `${p.id} (${p.line.length}): ${p.line}`).join('\n    '));
  F.verdict(L.every(p => (!p.hour || HOUR_IDS.includes(p.hour)) && (!p.weather || WEATHER_KINDS.includes(p.weather)) && (!p.difficulty || ['standard', 'hard'].includes(p.difficulty)) && (p.squad === undefined || (Number.isInteger(p.squad) && p.squad >= 0 && p.squad <= 4 && P.pickEntry(p)))
    && (p.flags ?? []).every((f: string) => /^[A-Za-z][\w]*(=[\w.-]*)?$/.test(f) && !['pick', 'hour', 'weather', 'difficulty', 'hard', 'squad', 'squad4', 'levelFile', 'solo'].includes(f.split('=')[0]))),
    "every setting is one the build has (hour, weather, difficulty, a team of 0 to 4 on a scenario), and a switch never spells a setting that has its own field", L.map(p => `${p.id}: ${JSON.stringify(wantOf(p))}`).join('\n    '));
  { const FLAGS = '?quality=high&mute', bad = L.flatMap(p => { const h = P.pickHref(p, FLAGS), why = h === null ? 'no URL' : hrefSays(p, h) ?? (h.endsWith('&quality=high&mute') ? null : "the visitor's flags lost"); return why ? [`${p.id}: ${why} (${h})`] : []; });
    F.verdict(bad.length === 0, "each row's URL names its mission, team size, difficulty (said on every scenario row, standard included: picks-boot holds it against a machine that keeps the other), hour, weather and switches, then the visitor's own flags", bad.length ? bad.join('\n    ') : L.map(p => P.pickHref(p, FLAGS)).join('\n    ')); }
  { const OLD = '?levelFile=warehouse.json&roomclear&squad4&squad=3&difficulty=hard&hard&digin&pressure=1&hour=dusk&sun=0.4&weather=fog&pair&silentstart&pick=old-one&solo&mute';
    const bad = L.filter(p => P.pickHref(p, OLD) !== P.pickHref(p, '?mute'));
    F.verdict(bad.length === 0, "an older run's mission and settings on the visitor's URL are replaced whole (level, team, difficulty, hour, sun, weather, switches of other rows, an older pick): only ?mute rides along", `${P.pickHref(L[0], OLD)}` + (bad.length ? ` — NOT: ${bad.map(p => `${p.id}: ${P.pickHref(p, OLD)}`).join(' · ')}` : '')); }
  { const FLAGS = '?quality=high&mute', hrefs: string[] = L.map(p => P.pickHref(p, FLAGS));
    const back = L.every((p, i) => P.pickOfQuery(hrefs[i]) === p), again = L.every((p, i) => P.pickHref(p, hrefs[i]) === hrefs[i]);
    const moved = L.every((p, i) => P.pickOfQuery(hrefs[i] + '&hour=dusk') !== p || paramsOf(hrefs[i]).get('hour') === 'dusk'), other = L.every((p, i) => P.pickOfQuery(hrefs[i].replace(`pick=${p.id}`, `pick=${L[(i + 1) % L.length].id}`)) === null);
    const sized = L.filter(p => P.pickEntry(p)).every((p) => { const h = P.pickHref(p, ''), n = (wantOf(p).squad + 1) % 5; return P.pickOfQuery(`${h}&squad=${n}`.replace(/&squad4\b/, '')) === null; });
    const plain = P.pickOfQuery('?quality=high') === null && P.pickOfQuery('?roomclear') === null && P.pickOfQuery('') === null;
    F.verdict(back && again && moved && other && sized && plain, "each URL reads back to its own row and is its own answer; the same mission with another hour, another team size or another row's name on it is no pick; a plain URL is none",
      `back ${back} · idempotent ${again} · hour moved ${moved} · wrong name ${other} · other size ${sized} · plain ${plain}`); }
  { const byId = L.every(p => P.pickRedirect(`?pick=${p.id}&mute`) === P.pickHref(p, '?mute')), byPlace = L.every((p, i) => P.pickRedirect(`?pick=${i + 1}`) === P.pickHref(p, ''));
    const once = L.every(p => P.pickRedirect(P.pickHref(p, '?mute')) === null), none = P.pickRedirect('?pick=nothing-here') === null && P.pickRedirect(`?pick=${L.length + 1}`) === null && P.pickRedirect('?pick=0') === null && P.pickRedirect('?mute') === null && P.pickRedirect(`?roomclear&pick=${L[0].id}`) === null;
    const atHard = L.filter(p => P.pickEntry(p)).every(p => P.pickRedirect(`?pick=${p.id}&difficulty=hard`) === P.pickHref(p, '', undefined, undefined, 'hard'));
    F.verdict(byId && byPlace && once && none && atHard, '?pick=<id> and ?pick=<place> alone answer with that row\'s URL (the visitor\'s flags kept); the answer is never answered again; an unknown name, a place off the list, a URL that already names a mission: no redirect',
      `?pick=${L[0].id}&mute → ${P.pickRedirect(`?pick=${L[0].id}&mute`)} · ?pick=${L.length} → ${P.pickRedirect(`?pick=${L.length}`)} · by id ${byId} · by place ${byPlace} · once ${once} · none ${none} · ?pick=<id>&difficulty=hard is the row on hard ${atHard}`); }
  { const chain = L.map(p => P.nextPick(p)?.id ?? 'end');
    F.verdict(L.every((p, i) => P.nextPick(p) === (L[i + 1] ?? null)), 'the row after each is the next in play order, and nothing follows the last', chain.join(' → ')); }
  { const urls = ['?levelFile=depot.json&roomclear&squad4&mute&hour=dusk', '?rescue&difficulty=hard', '?solo&weather=fog&quality=high', '?nightbreach&pick=not-a-pick&hour=dawn', ''];
    const same = urls.every(u => P.pickQuitSearch(u) === quitSearch(u)), out = L.map(p => P.pickQuitSearch(P.pickHref(p, '?quality=high&mute')));
    F.verdict(same && out.every(o => o === '?quality=high&mute'), "a quit out of a pick takes its settings off the URL with the scenario's flags (the plain card comes back); any other URL quits exactly as the build's own quit",
      `a pick → '${out[0]}' · ${urls.map(u => `'${u}' → '${P.pickQuitSearch(u)}'`).join(' · ')}`); }
  { const withFlags = L.find(p => (p.flags ?? []).length), h = withFlags ? P.pickHref(withFlags, '?mute') : null, left = h ? P.pickLeaveSearch(h) : '', lq = new URLSearchParams(left);
    const ok = !withFlags || (!lq.has('pick') && withFlags.flags.every((f: string) => !lq.has(f.split('=')[0])) && lq.has('difficulty') && lq.has('mute') && !!specOfQuery(lq));
    const same = ['?levelFile=depot.json&roomclear&squad4&pair', '?pair&pick=nothing', ''].every(u => P.pickLeaveSearch(u) === u);
    const names = P.namesMission('?roomclear') && P.namesMission('?solo&mute') && !P.namesMission('?demo') && !P.namesMission('?mute&quality=high') && !P.namesMission('?squad=3') && !P.namesMission('') && L.every(p => P.namesMission(P.pickHref(p, '')));
    F.verdict(ok && same && names, "the mission list opened off a pick's page builds its links from that page's URL WITHOUT the pick's name and its own switches (the difficulty the list shows stays); any other URL is handed over as it is; and a URL that names a mission, a pick's included, is told from one that does not",
      `${h ?? 'no row has switches'} → '${left}' · others untouched ${same} · names a mission ${names}`); }
  // ---- what no row of today's list does, held on a list made up for it: a row with a team size and switches of its own (a quiet pair), and the walk
  //      through a row that opens on hard
  { const Q: any = { id: 'quiet-two', mission: 'block-four', squad: 1, flags: ['pair', 'silentstart'], tags: ['quiet'], line: 'x' }, H: any = { id: 'on-hard', mission: 'depot-four', difficulty: 'hard', line: 'x' }, S: any = { id: 'plain', mission: 'compound-four', line: 'x' };
    const LL = [S, H, Q], hq = P.pickHref(Q, '?mute&pair&squad=3', LL)!, q = paramsOf(hq);
    const made = q.get('squad') === '1' && q.has('pair') && q.has('silentstart') && q.get('pick') === 'quiet-two' && q.getAll('pair').length === 1 && q.getAll('squad').length === 1 && q.has('mute');
    const back = P.pickOfQuery(hq, LL) === Q && P.pickOfQuery(hq.replace('&silentstart', ''), LL) === null && P.pickHref(Q, hq, LL) === hq;
    const left = new URLSearchParams(P.pickLeaveSearch(hq, LL)), leaves = !left.has('pick') && !left.has('pair') && !left.has('silentstart') && left.has('difficulty') && left.has('mute') && !!specOfQuery(left);
    const quits = P.pickQuitSearch(hq, LL) === '?mute', plainRow = !paramsOf(P.pickHref(S, hq, LL)!).has('pair') && !paramsOf(P.pickHref(S, hq, LL)!).has('silentstart') && paramsOf(P.pickHref(S, hq, LL)!).get('squad') !== '1';
    const hs = P.pickHref(S, '', LL, undefined, 'hard')!, hh = P.pickHref(H, '', LL)!, hstd = P.pickHref(H, '', LL, undefined, 'standard')!;
    const walk = P.pickChosenDifficulty(hs, LL) === 'hard' && P.pickChosenDifficulty(hh, LL) === undefined && P.pickChosenDifficulty(hstd, LL) === 'standard' && P.pickOfQuery(hh, LL) === H && P.pickOfQuery(hstd, LL) === H && P.nextPick(S, LL) === H && P.nextPick(H, LL) === Q;
    const typed = `${hh}&pair`, stale = P.pickOfQuery(typed, LL) === null && !new URLSearchParams(P.pickQuitSearch(typed, LL)).has('pick') && P.pickRedirect(P.pickQuitSearch(typed, LL), LL) === null;
    F.verdict(made && back && leaves && quits && plainRow && walk && stale, "on a made-up list: a row with its own team size and switches writes them once over the visitor's, reads back only whole, sheds them on the way to the mission list and on a quit, and lends them to no other row; a row that opens on hard is its own row at either difficulty, and only a difficulty that is not the row's own counts as chosen; a row's name left on a URL somebody typed over does not ride out of a quit (the plain page would load the mission again)",
      `${hq} · made ${made} · back ${back} · leaves ${leaves} · quits ${quits} · other rows clean ${plainRow} · the hard row ${walk} · typed over, then quit ${stale}: '${P.pickQuitSearch(typed, LL)}'`); }
  // ---- the maintainer's rules for the list (10-08): what comes first, what the rows may set
  { const p = L[0], e: any = P.pickEntry(p), w = wantOf(p);
    F.verdict(!!e && w.squad === 4 && !!squadsOf(e) && !w.flags.includes('silentstart') && !w.flags.includes('pair'), 'the first row is a four-person loud breach on a squad map (his pick of what comes first)',
      `${p.id}: ${e?.id} on ${e?.level} with ${w.squad}${w.flags.length ? ` · ${w.flags.join(' ')}` : ''}`); }
  { const timed = L.filter(p => p.hour || p.weather);
    F.verdict(timed.length === 1 && timed.every(p => p.mission === 'barracks-four' && p.hour === 'midday' && !p.weather) && L.every(p => !(p.flags ?? []).some((f: string) => /^(sun|callout|nocallout)\b/.test(f))), "the barracks is the one row with an hour (midday, no weather, no sun: a dry clear day by the hours' own rule); every other row is its level's own night and weather; no row touches the call out",
      timed.length ? timed.map(p => `${p.id}: ${p.hour ?? ''}${p.weather ? ` ${p.weather}` : ''} → ${P.pickHref(p, '')}`).join(' · ') : 'no row sets an hour or a weather'); }
  { const on = L.every(p => P.onShortList(p.mission)), others = SCENARIOS.filter((e: any) => !L.some(p => p.mission === e.id)).every((e: any) => !P.onShortList(e.id)); P.PICKS_UI.on = false; const off = L.every(p => !P.onShortList(p.mission)); P.PICKS_UI.on = true;
    F.verdict(on && others && off, "a mission the list names is told from the rest (the mission sheet and the loadout screen leave 'untuned' off it), and none is with the switch off", `on the list ${on} · the rest not ${others} · none under ?nopicks ${off} · tagged untuned in the data: ${L.filter(p => (P.pickEntry(p) as any)?.untuned).map(p => p.id).join(', ') || 'none'}`); }
  F.verdict(L.every(p => p.loadout === undefined), 'no row carries kit: the field for a loadout pick is reserved and nothing reads it yet', L.map(p => `${p.id}: ${p.loadout ?? 'none'}`).join(' · '));
  // ---- THE ONE CHOICE: standard or hard, on the row
  { const FLAGS = '?quality=high&mute', S = L.filter(p => P.pickEntry(p)), solo = L.filter(p => !P.pickEntry(p));
    const bad = S.flatMap(p => { const own = P.pickDifficulty(p), d = P.otherDifficulty(own), a = P.pickHref(p, FLAGS), b = P.pickHref(p, FLAGS, undefined, undefined, d);
      const why = hrefSays(p, b, d) ?? (b.replace(`difficulty=${d}`, `difficulty=${own}`) !== a ? 'differs in more than the difficulty' : P.pickOfQuery(b) !== p ? 'does not read back as the row' : P.pickChosenDifficulty(b) !== d || P.pickChosenDifficulty(a) !== undefined ? 'the choice is not read off the URL'
        : P.pickQuitSearch(b) !== FLAGS ? `quits to '${P.pickQuitSearch(b)}'` : P.pickOfQuery(b.replace(`difficulty=${d}`, 'difficulty=custom')) !== null ? 'a difficulty that is neither still reads as the pick' : P.pickHref(p, b) !== a ? "the row's own link from that page is not the row's own URL" : null);
      return why ? [`${p.id}: ${why} (${b})`] : []; });
    const soloSame = solo.every(p => P.pickHref(p, FLAGS, undefined, undefined, 'hard') === P.pickHref(p, FLAGS) && P.pickOfQuery(P.pickHref(p, FLAGS) + '&difficulty=hard') === null);
    F.verdict(bad.length === 0 && soloSame && S.length > 0, "every scenario row's URL at the OTHER difficulty differs in that one value, reads back as the same row (so its quit and its 'next' work), and is told from the row's own; a difficulty that is neither is no pick; the solo night has no such choice",
      bad.length ? bad.join('\n    ') : S.map(p => `${p.id}: ${P.pickDifficulty(p)} | ${P.otherDifficulty(P.pickDifficulty(p))} → ${P.pickHref(p, '', undefined, undefined, P.otherDifficulty(P.pickDifficulty(p)))}`).join('\n    ') + ` · solo unchanged ${soloSame}`); }
  // ---- a row with ONE difficulty: only where the build's own words say hard changes nothing on that level
  { const DW = (await import(`${ROOT}/src/game/difficulty.ts`)).difficultyWords, same = (p: any) => /the squads here are the standard ones/.test(DW('hard', (P.pickEntry(p) as any)?.level || false));
    const S = L.filter(p => P.pickEntry(p)), one = L.filter(p => p.oneDifficulty), bad = [...one.filter(p => !P.pickEntry(p) || p.difficulty === 'hard' || !same(p) || P.pickOtherDifficulty(p) !== null).map(p => `${p.id}: marked, but ${!P.pickEntry(p) ? 'not a scenario' : p.difficulty === 'hard' ? 'it opens on hard' : !same(p) ? 'hard is not the standard squads there' : 'still offers another'}`),
      ...S.filter(p => !p.oneDifficulty && (same(p) || P.pickOtherDifficulty(p) !== P.otherDifficulty(P.pickDifficulty(p)))).map(p => `${p.id}: ${same(p) ? 'hard is the standard squads there and the row still offers it' : 'offers no other difficulty'}`), ...L.filter(p => !P.pickEntry(p) && P.pickOtherDifficulty(p) !== null).map(p => `${p.id}: the solo night offers a difficulty`)];
    const typed = one.every(p => { const h = P.pickHref(p, '', undefined, undefined, 'hard')!; return P.pickOfQuery(h) === p && P.pickRedirect(`?pick=${p.id}&difficulty=hard`) === h; });
    F.verdict(bad.length === 0 && typed, "a row offers no other difficulty exactly where the build says its hard is the standard squads (it opens on standard); typed by hand, hard there still loads and reads back as the row",
      bad.length ? bad.join('\n    ') : `one difficulty: ${one.map(p => p.id).join(', ') || 'none'} · typed hard still the row ${typed} · the other scenario rows: ${S.filter(p => !p.oneDifficulty).map(p => `${p.id} ${P.pickOtherDifficulty(p)}`).join(' · ')}`); }
  return F.done();
}

// ───────────────────────────────────────────────────────────── card ─────────────────────────────────────────────────────────────
const menuDeps = (W: any, extra: any = {}) => ({ engine: W.engine, game: W.game, quality: { auto: false, current: 'high', adaptive: true, lastChange: '' }, audio: { volume: { master: 1, sfx: 1, ambience: 1, music: 1 }, setVolumes() {} }, overlay: { showLightDots: true }, onStart() {}, ...extra });
/** a localStorage for the length of `fn` (the harness has none): what the card keeps per machine */
async function withStore<T>(fn: (store: Map<string, string>) => Promise<T> | T): Promise<T> {
  const G = globalThis as any, had = 'localStorage' in G, prev = G.localStorage, m = new Map<string, string>();
  G.localStorage = { getItem: (k: string) => m.get(k) ?? null, setItem: (k: string, v: string) => { m.set(k, String(v)); }, removeItem: (k: string) => { m.delete(k); } };
  try { return await fn(m); } finally { if (had) G.localStorage = prev; else delete G.localStorage; }
}
export async function picksCard(): Promise<boolean> {
  const F = family('picks-card: the real Menu on a fake DOM — the pause card of a run a pick started offers the next row');
  const SEARCH = '?quality=high&mute', L: any[] = P.PICKS;
  try {
    await withStore(store => withDom(SEARCH, async (root) => {
      const { Menu } = await import(`${ROOT}/src/ui/menu.ts`), UI = await import(`${ROOT}/src/ui/picks.ts`);
      const loads: string[] = []; Object.assign((globalThis as any).location, { pathname: '/', hash: '', assign: (u: string) => { loads.push(u); }, reload: () => { loads.push('(reload)'); } });
      Math.random = seeded(0x51c4); setDifficulty('standard', false); P.PICKS_UI.on = true;
      const W = await quiet(() => standUp({ uploadEvery: 4 })); let starts = 0;
      let up: any = null, refused: string | null = null;   // (what main.ts's scenario seam would say is running under the card)
      const menu = new Menu(menuDeps(W, { onStart() { starts++; }, scenario: { current: () => up, refused: () => refused, quit() {} } }) as any);
      menu.open('start');
      // ---- the pause card of a run a pick started: the next row
      { const loc = (globalThis as any).location, was = loc.search; loads.length = 0;
        const RUN = { title: 'Night breach', team: 'you + the pair', kind: 'nightbreach' };
        loc.search = P.pickHref(L[0], SEARCH); up = null; menu.open('pause'); const notUp = startsWith(root, 'Next: ');   // (the URL is the pick's, its scenario is not up: refused at boot, the plain night played)
        up = RUN; menu.open('pause'); const nx = startsWith(root, 'Next: '); nx?.onclick?.();
        const okNext = !!nx && nx.textContent === `Next: ${UI.pickTitle(L[1])} ›` && loads.length === 1 && loads[0] === '/' + P.pickHref(L[1], SEARCH);
        up = P.pickEntry(L[L.length - 1]) ? RUN : null; loc.search = P.pickHref(L[L.length - 1], SEARCH); menu.open('pause'); const last = startsWith(root, 'Next: ');
        up = RUN; loc.search = '?roomclear&quality=high'; menu.open('pause'); const plain = startsWith(root, 'Next: ');
        // (the first row started at the OTHER difficulty: the choice goes on to the row after, where that is a scenario and not already its own)
        { const d = otherOf(wantOf(L[0]).difficulty), carries = !!P.pickEntry(L[1]) && d !== wantOf(L[1]).difficulty; loads.length = 0;
          loc.search = P.pickHref(L[0], SEARCH, undefined, undefined, d); up = RUN; menu.open('pause'); const cx = startsWith(root, 'Next: '); cx?.onclick?.();
          const want = P.pickHref(L[1], SEARCH, undefined, undefined, carries ? d : undefined);
          F.verdict(!!cx && cx.textContent === `Next: ${UI.pickTitle(L[1])}${carries ? ` · ${d}` : ''} ›` && loads.length === 1 && loads[0] === '/' + want, "a run started at the row's other difficulty still offers the next row, and carries that choice on to it (said on the button)",
            `${cx?.textContent ?? 'none'} → ${loads[0] ?? 'no load'}`); loads.length = 0; }
        // (every row to the one after it, started at the other difficulty: carried only onto a row that offers that difficulty and does not open on it)
        { const bad: string[] = [], said: string[] = [];
          for (let i = 0; i + 1 < L.length; i++) { const a = L[i], b = L[i + 1], d = P.pickOtherDifficulty(a); if (!d) continue;
            const got = UI.nextPickOf(P.pickHref(a, SEARCH, undefined, undefined, d)), carries = P.pickOtherDifficulty(b) !== null && d !== wantOf(b).difficulty, want = P.pickHref(b, SEARCH, undefined, undefined, carries ? d : undefined);
            said.push(`${a.id} at ${d} → ${got?.title ?? 'none'}`); if (!got || got.href !== want || got.title !== UI.pickTitle(b) + (carries ? ` · ${d}` : '')) bad.push(`${a.id} at ${d} → ${got?.title ?? 'none'} (${got?.href ?? 'no URL'}), wanted ${want}`); }
          F.verdict(bad.length === 0 && said.length > 0, "down the whole list: a difficulty chosen over a row's own goes on to the next row only where that row offers it and does not already open on it (a row with one difficulty, and the solo night, open as they are)", bad.length ? bad.join('\n    ') : said.join(' · ')); }
        loc.search = was; up = null;
        F.verdict(okNext && !last && !plain && !notUp, "the pause card of a run a pick started offers 'Next: <the row after>' = that row's own page load; the last row's offers none, a run started any other way offers none, and neither does a pick's page whose scenario is not what is up; no rows on a pause card",
          `after ${L[0].id}: ${nx?.textContent ?? 'none'} → ${loads[0] ?? 'no load'} · after the last: ${last?.textContent ?? 'none'} · ?roomclear: ${plain?.textContent ?? 'none'} · the pick's URL with no scenario up: ${notUp?.textContent ?? 'none'}`); }
      // ---- the switch off
      { P.PICKS_UI.on = false; const loc = (globalThis as any).location, was = loc.search;
        loc.search = P.pickHref(L[0], SEARCH); menu.open('pause'); const nx = startsWith(root, 'Next: '); loc.search = was; P.PICKS_UI.on = true;
        F.verdict(!nx, "with the switch off the pause card carries no 'Next'", `next: ${nx?.textContent ?? 'none'}`); }
      menu.close();
    }));
  } catch (e: any) { F.verdict(false, 'the card probe ran to its end', String(e?.stack ?? e).split('\n').slice(0, 4).join(' | ')); }
  finally { P.PICKS_UI.on = true; }
  return F.done();
}

// ───────────────────────────────────────────────────────────── boot ─────────────────────────────────────────────────────────────
async function bootChild(query: string, secs: number, kept: string): Promise<any> {
  const p = Bun.spawn(['bun', 'run', import.meta.path, '--boot', query, '--secs', String(secs), '--kept', kept], { cwd: ROOT, stdout: 'pipe', stderr: 'pipe', env: process.env });
  const [out, errText] = await Promise.all([new Response(p.stdout).text(), new Response(p.stderr).text()]); await p.exited;   // (both pipes drained together: a child loud on one never waits on the other)
  const line = out.split('\n').find(l => l.startsWith(TAG));
  return line ? JSON.parse(line.slice(TAG.length)) : { query, threw: `the boot printed no report (exit ${p.exitCode}): ${(errText || out).trim().split('\n').slice(-3).join(' | ')}` };
}
export async function picksBoot(secs = 30): Promise<boolean> {
  const F = family(`picks-boot: every row's URL booted in a process of its own on a machine that keeps the other difficulty — its level, its team, the URL's settings in force — then ${secs} s of the simulation`);
  const L: any[] = P.PICKS, MS = await import(`${ROOT}/src/ui/missions.ts`), missions: any[] = MS.missionList();
  // (every row at its own difficulty, and the first row once more at the other: the one choice the card offers on a row)
  const C: { p: any; i: number; d?: string }[] = [...L.map((p, i) => ({ p, i })), ...(P.pickEntry(L[0]) ? [{ p: L[0], i: 0, d: otherOf(wantOf(L[0]).difficulty) }] : [])];
  const hrefOf = (c: { p: any; d?: string }) => P.pickHref(c.p, '?quality=high&mute', undefined, undefined, c.d);
  const jobs = Math.max(1, Math.min(4, Number(process.env.PICKS_JOBS) || 3)), boots: any[] = new Array(C.length); let next = 0;
  await Promise.all(Array.from({ length: jobs }, async () => { for (;;) { const k = next++; if (k >= C.length) return; boots[k] = await bootChild(hrefOf(C[k]), secs, otherOf(wantOf(C[k].p, C[k].d).difficulty)); } }));   // (each on a machine that keeps the OTHER difficulty: the URL's is the one in force)
  for (let k = 0; k < C.length; k++) { const { p, i, d } = C[k], r = boots[k], w = wantOf(p, d), e: any = P.pickEntry(p), m = missions.find(x => x.id === p.mission), href = hrefOf(C[k]);
    const why: string[] = [];
    if (r.threw) why.push(`threw: ${r.threw}`);
    else {
      if ((r.errors ?? []).length) why.push(`console errors: ${r.errors.slice(0, 3).join(' | ')}`);
      if (e) {
        if (r.why !== null) why.push(`REFUSED: ${r.why}`);
        if (r.active !== (e.kind === 'roomclear' ? r.active : e.kind) || r.kind !== e.kind) why.push(`running ${r.kind}, not ${e.kind}`);
        if ((r.levelFile ?? '') !== (e.level ?? '')) why.push(`level ${r.levelFile}, not ${e.level}`);
        if (r.operatives !== w.squad || r.squad !== w.squad) why.push(`${r.operatives} teammates (scenario says ${r.squad}), not ${w.squad}`);
        if (w.squad === e.squad && r.entry !== e.id) why.push(`reads as row '${r.entry}', not '${e.id}'`);
        if (r.hostiles !== e.defenders) why.push(`${r.hostiles} defenders, the sheet says ${e.defenders}`);
        if (r.difficulty !== w.difficulty) why.push(`difficulty ${r.difficulty}, not ${w.difficulty}`);
      } else {
        const want = /^(\d+), and (\d+) civilians$/.exec(m?.defenders ?? '');
        if (r.active !== null || r.operatives !== 0) why.push(`a scenario is up (${r.active}) or somebody stands beside Sam (${r.operatives})`);
        if (!want || r.staff !== Number(want[1]) || r.civilians !== Number(want[2])) why.push(`the shift ${r.staff} and ${r.civilians} civilians, the sheet says ${m?.defenders}`);
      }
      if ((r.hour ?? 'night') !== (w.hour ?? 'night')) why.push(`hour ${r.hour}, not ${w.hour ?? 'night'}`);
      if ((r.weather ?? null) !== w.weather) why.push(`weather ${r.weather}, not ${w.weather}`);
      if (r.pair !== w.flags.includes('pair')) why.push(`the pair switch ${r.pair ? 'on' : 'off'}`);
      if (r.silent !== w.flags.includes('silentstart')) why.push(`the team stands up ${r.silent ? 'silent' : 'loud'}`);
      if (!(r.t1 - r.t0 > secs - 1) || r.d0 === r.d1) why.push(`did not play: the clock went ${r.t0?.toFixed?.(1)} → ${r.t1?.toFixed?.(1)} s, the game's digest ${r.d0 === r.d1 ? 'did not move' : 'moved'}`);
    }
    F.verdict(why.length === 0, `${i + 1} ${p.id}${d ? ` at the other difficulty (${d})` : ''}: ${href}`, why.length ? why.join(' · ') : `${e ? `started unrefused as ${e.kind} on ${e.level ?? 'the office'} · ${r.operatives} teammates · ${r.hostiles} defenders${r.hostages ? ` · ${r.hostages} hostages` : ''} · ${r.difficulty}` : `the plain night · the shift ${r.staff}, civilians ${r.civilians}`} · hour ${r.hour ?? 'night'} · weather ${r.weather ?? "the level's own"}${r.pair ? ' · the pair switch on' : ''}${r.silent ? ' · the team silent from the first second' : ''} · ${r.boxes} boxes · played ${(r.t1 - r.t0).toFixed(0)} s: ${r.operativesAfter} teammates and ${r.hostilesAfter} defenders up, no error`); }
  return F.done();
}

// ───────────────────────────────────────────────────────────── play ─────────────────────────────────────────────────────────────
export async function picksPlay(seeds = '1-3'): Promise<boolean> {
  const F = family(`picks-play: every scenario row played to its end by the scripted commander with the row's settings, stood up as the page opens (glass, moves, turn weight, destruction, defenders' frags) (seeds ${seeds})`);
  const L: any[] = P.PICKS, tmp = `${process.env.TMPDIR ?? '/tmp'}/picks-play-${process.pid}`;
  for (let i = 0; i < L.length; i++) { const p = L[i], e: any = P.pickEntry(p), w = wantOf(p);
    if (!e) { console.log(`- (no scripted player for the solo night: ${p.id} is held by picks-boot)`); continue; }
    const set: any = { DIFFICULTY: { level: w.difficulty } }; if (w.weather) set.WEATHER = { kind: w.weather }; set.FRAG = { defenders: true };
    const env: Record<string, string> = { ...process.env as any, PT_DIGEST: '1', GLASSFX: 'on', TPS_MOVES: '1', TPS_TURN: '1', PT_DESTR: '1' }; if (w.squad !== e.squad) env.PT_SQUAD = String(w.squad); if (w.hour) env.PT_HOUR = w.hour;   // (PICKS_PLAIN, 'the harness's plain stand-up instead', went 2026-11-06: it asked by leaving these words out, which is the page's stand since the flip — the same game as this arm)
    const out = `${tmp}-${p.id}.json`, proc = Bun.spawn(['bun', 'run', `${ROOT}/tools/qa/playthrough.ts`, '--scn', e.id, '--seeds', seeds, '--jobs', '3', '--json', out, '--set', JSON.stringify(set)], { cwd: ROOT, env, stdout: 'pipe', stderr: 'pipe' });
    const [text, err] = await Promise.all([new Response(proc.stdout).text(), new Response(proc.stderr).text()]); await proc.exited;
    let runs: any[] = []; try { runs = JSON.parse(readFileSync(out, 'utf8')).runs; } catch { /* no file: the run died */ } try { rmSync(out); } catch { /* none */ }
    const bad = runs.filter(r => r.end === 'error' || r.difficulty !== w.difficulty || (r.hour ?? 'night') !== (w.hour ?? 'night') || (w.squad !== e.squad && r.of !== w.squad));
    F.verdict(proc.exitCode === 0 && runs.length > 0 && bad.length === 0, `${i + 1} ${p.id}: ${e.id}${w.squad !== e.squad ? ` with ${w.squad}` : ''} · ${w.difficulty}${w.hour ? ` · ${w.hour}` : ''}${w.weather ? ` · ${w.weather}` : ''}${w.flags.length ? ` (without ${w.flags.join(', ')}: a page switch)` : ''}`,
      runs.length ? runs.map(r => `seed ${r.seed}: ${r.end}${r.why ? ` (${r.why})` : ''} at ${Math.round(r.t)} s, ${r.team} of ${r.of} up, ${r.hostiles} of ${r.hostilesOf} defenders left, ${r.difficulty}${r.hour ? `, ${r.hour}` : ''}, digest ${r.digest}`).join('\n    ') : `no runs (exit ${proc.exitCode}): ${(err || text).trim().split('\n').slice(-3).join(' | ')}`); }
  return F.done();
}

if (import.meta.main) {
  const want = process.argv.slice(2).filter(a => !a.startsWith('-')); const on = (n: string) => !want.length || want.includes(n) || want.includes('picks');
  let ok = true;
  if (on('picks-list')) ok = (await picksList()) && ok;
  if (on('picks-card')) ok = (await picksCard()) && ok;
  if (on('picks-boot')) ok = (await picksBoot()) && ok;
  if (want.includes('picks-play')) ok = (await picksPlay()) && ok;
  process.exit(ok ? 0 : 1);
}
