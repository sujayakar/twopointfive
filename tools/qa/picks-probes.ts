// The 'picks' family (THE SHORT LIST — scene/picks.ts PICKS, ui/picks.ts for ui/menu.ts's cards, main.ts's ?pick= redirect), run through probes.ts:
//   bun run tools/qa/probes.ts picks | picks-list | picks-boot          (or this file: bun run tools/qa/picks-probes.ts [name …])
//   bun run tools/qa/probes.ts picks-play                          (minutes; not in the family) every scenario row played to its end by the scripted commander, its settings on
// PASS / FAIL each, all headless. Nothing here looks at a picture.
//   picks-list   the table as data: ids unique and URL-safe, every mission on the build, a plain line on each, two to seven rows; each row's URL names its
//                mission, team size and difficulty OUT (nothing left to what a machine keeps), reads back to that row and to no other,
//                keeps a visitor's own flags and drops an older run's settings; `?pick=<id>` and `?pick=<place>` answer with that URL once and never twice;
//                the row after each; a quit out of a pick takes its settings off the URL and leaves every other URL as the build's quit leaves it; the rules the
//                maintainer set for the list (a four-person loud breach on a squad map first; every row at its level's own night
//                and weather; no kit on a row); and THE ONE CHOICE: every scenario row's URL at the other difficulty differs in that one value and is the same pick
//   picks-boot   every row's URL booted in a process of its own with the page's own start-up order (the URL where modules read it, the difficulty, the weather,
//                the hour, the level file, startFromQuery): it starts unrefused as its mission on its level, with the row's team, difficulty, hour, weather and
//                switches in force (on a machine that keeps the OTHER difficulty; the harness stands most page-default feature switches off, so 'in force' is the
//                URL's settings, not every default of the page), no console error — and RUNS: thirty seconds of the simulation with nobody at the keys, nothing thrown, the game's clock and digest moving
//   picks-play   every scenario row played from its staging to its END by tools/qa/playthrough.ts's scripted commander at the row's team size, difficulty, hour and weather,
//                three seeds each, STOOD UP AS THE PAGE OPENS (glass on, the movement pass, turn weight, destruction on, defenders carry frags — the env and --set
//                the one build's page defaults need under the old stand; on the page's they are what it has): no run errors, every run
//                reports the row's own difficulty and hour, and every run ends (won or lost, never an error). The `pair` switch reads
//                the page's URL and is not in these runs (picks-boot holds it on); the solo night has no scripted player (picks-boot holds its boot)
import { sample, swept } from './sweep.ts';
import { resolve } from 'node:path';
import { readFileSync } from 'node:fs';

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
    const { hourOf } = await import(`${ROOT}/src/game/hours.ts`);
    const { runningScenario, activeScenario, teamOf } = await import(`${ROOT}/src/game/scenarioRun.ts`);
    const { parseLevelData } = await import(`${ROOT}/src/scene/levelData.ts`);
    const { loadLevel } = await import(`${ROOT}/src/scene/levelLoad.ts`);
    const { PAIRUP } = await import(`${ROOT}/src/game/pairup.ts`);
    const { SILENT, squadSilent } = await import(`${ROOT}/src/game/silent.ts`);
    const { digestGame } = await import(`${ROOT}/src/game/digest.ts`);
    const { hashStr } = await import(`${ROOT}/src/game/dialogue.ts`).catch(() => ({ hashStr: null as any }));
    // main.ts's order: the difficulty, the weather and the hour are read before the Game exists; the level the URL names; the Game; the scenario
    initDifficulty(q);   // (the weather and the hour: the stand reads them off the address before the game is made, as the page does — headless.ts `address`)
    Math.random = seeded(0x51c4);
    const level = levelFile ? (() => { const data = parseLevelData(readFileSync(resolve(ROOT, 'levels', levelFile), 'utf8')); return loadLevel(data); })() : undefined;
    const W = await standUp({ uploadEvery: 4, address: q, ...(level ? { level } : {}) });
    const g = W.game; g.godMode = true;
    SILENT.on = true;   // (the page's: the harness stands the game up with the silent rows off — src/host/standup.ts)
    if (q.has('silentstart')) SILENT.start = true;   // (main.ts's line, copied: the flag is the page's — tools/qa/startpage-look.cjs holds the page's own)
    const { why } = W.start();
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

type Verdict = (ok: boolean, label: string, detail: string) => void;
function family(title: string): { verdict: Verdict; done: () => boolean } {
  console.log(`\n=== ${title} ===`); let all = true;
  return { verdict: (ok, label, detail) => { all &&= ok; console.log(`- ${ok ? 'PASS' : 'FAIL'} — ${label}\n    ${detail}`); }, done: () => { console.log(`=== ${title.split(':')[0]}: ${all ? 'ALL PASS' : 'FAILURES above'} ===`); return all; } };
}
const quiet = async <T>(fn: () => Promise<T> | T): Promise<T> => { const log = console.log, err = console.error, warn = console.warn; console.log = console.error = console.warn = () => {}; try { return await fn(); } finally { console.log = log; console.error = err; console.warn = warn; } };
const first = (r: El, cls: string) => r.all(e => e.cls.has(cls))[0];
const buttons = (root: El) => root.all(e => e.tagName === 'BUTTON');
const startsWith = (root: El, text: string) => buttons(root).find(b => b.textContent.startsWith(text));
const paramsOf = (href: string) => new URLSearchParams(href.replace(/^[^?]*\?/, ''));
const otherOf = (d: string | null) => (d === 'hard' ? 'standard' : 'hard');
/** what a pick's settings say its URL must carry — written here from the row, not from pickHref */
function wantOf(p: any, d?: string): { kind: string | null; level: string; squad: number; difficulty: string | null; hour: string | null; weather: string | null; flags: string[] } {
  const e: any = P.pickEntry(p);
  return { kind: e ? e.kind : null, level: e?.level ?? '', squad: e ? p.squad ?? e.squad : 0, difficulty: e ? d ?? p.difficulty ?? 'standard' : null, hour: null, weather: null, flags: [] };
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
  const L: any[] = P.PICKS;
  F.verdict(L.length >= 2 && L.length <= 7 && new Set(L.map(p => p.id)).size === L.length && L.every(p => /^[a-z0-9]+(-[a-z0-9]+)*$/.test(p.id) && !/^\d+$/.test(p.id)),
    'two to seven rows, each under an id of its own that is safe on a URL (and never a bare number: that is a place on the list)', L.map((p, i) => `${i + 1} ${p.id}`).join(' · '));
  { const bad = L.filter(p => P.pickEntry(p) === undefined), ids = ['solo-night', ...ALL_MISSIONS.map((m: any) => m.id)];
    F.verdict(bad.length === 0 && L.every(p => ids.includes(p.mission)), "every row's mission is on the mission list (the solo night under the list's own id and flag)",
      L.map(p => `${p.id} → ${p.mission}`).join(' · ') + (bad.length ? ` — NOT ON THE BUILD: ${bad.map(p => p.mission).join(', ')}` : '')); }
  F.verdict(L.every(p => typeof p.line === 'string' && p.line.length >= 30 && p.line.length <= 110 && !/[—;]/.test(p.line) && /[.?!]$/.test(p.line)), 'one plain line on each: 30 to 110 characters, a whole sentence, no dash or semicolon',
    L.map(p => `${p.id} (${p.line.length}): ${p.line}`).join('\n    '));
  F.verdict(L.every(p => (!p.difficulty || ['standard', 'hard'].includes(p.difficulty)) && (p.squad === undefined || (Number.isInteger(p.squad) && p.squad >= 0 && p.squad <= 4 && P.pickEntry(p)))),
    "every setting is one the build has (difficulty, a team of 0 to 4 on a scenario)", L.map(p => `${p.id}: ${JSON.stringify(wantOf(p))}`).join('\n    '));
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
  { const p0 = L.find(p => P.pickEntry(p)), h = P.pickHref(p0, '?mute'), left = P.pickLeaveSearch(h), lq = new URLSearchParams(left);
    const ok = !lq.has('pick') && lq.has('difficulty') && lq.has('mute') && !!specOfQuery(lq);
    const same = ['?levelFile=depot.json&roomclear&squad4&pair', '?pair&pick=nothing', ''].every(u => P.pickLeaveSearch(u) === u);
    const names = P.namesMission('?roomclear') && P.namesMission('?solo&mute') && !P.namesMission('?demo') && !P.namesMission('?mute&quality=high') && !P.namesMission('?squad=3') && !P.namesMission('') && L.every(p => P.namesMission(P.pickHref(p, '')));
    F.verdict(ok && same && names, "the mission list opened off a pick's page builds its links from that page's URL WITHOUT the pick's name (the difficulty the list shows stays); any other URL is handed over as it is; and a URL that names a mission, a pick's included, is told from one that does not",
      `${h} → '${left}' · others untouched ${same} · names a mission ${names}`); }
  // ---- what no row of today's list does, held on a list made up for it: a row with a team size of its own, and the walk
  //      through a row that opens on hard
  { const Q: any = { id: 'quiet-two', mission: 'block-four', squad: 1, line: 'x' }, H: any = { id: 'on-hard', mission: 'depot-four', difficulty: 'hard', line: 'x' }, S: any = { id: 'plain', mission: 'compound-four', line: 'x' };
    const LL = [S, H, Q], hq = P.pickHref(Q, '?mute&pair&squad=3', LL)!, q = paramsOf(hq);
    const made = q.get('squad') === '1' && !q.has('pair') && q.get('pick') === 'quiet-two' && q.getAll('squad').length === 1 && q.has('mute');
    const back = P.pickOfQuery(hq, LL) === Q && P.pickOfQuery(hq.replace('squad=1', 'squad=2'), LL) === null && P.pickHref(Q, hq, LL) === hq;
    const left = new URLSearchParams(P.pickLeaveSearch(hq, LL)), leaves = !left.has('pick') && left.has('difficulty') && left.has('mute') && !!specOfQuery(left);
    const quits = P.pickQuitSearch(hq, LL) === '?mute', plainRow = paramsOf(P.pickHref(S, hq, LL)!).get('squad') !== '1';
    const hs = P.pickHref(S, '', LL, undefined, 'hard')!, hh = P.pickHref(H, '', LL)!, hstd = P.pickHref(H, '', LL, undefined, 'standard')!;
    const walk = P.pickChosenDifficulty(hs, LL) === 'hard' && P.pickChosenDifficulty(hh, LL) === undefined && P.pickChosenDifficulty(hstd, LL) === 'standard' && P.pickOfQuery(hh, LL) === H && P.pickOfQuery(hstd, LL) === H && P.nextPick(S, LL) === H && P.nextPick(H, LL) === Q;
    const typed = `${hh}&pair`, stale = P.pickOfQuery(typed, LL) === null && !new URLSearchParams(P.pickQuitSearch(typed, LL)).has('pick') && P.pickRedirect(P.pickQuitSearch(typed, LL), LL) === null;
    F.verdict(made && back && leaves && quits && plainRow && walk && stale, "on a made-up list: a row with its own team size writes it once over the visitor's, reads back only whole, sheds its name on the way to the mission list and its settings on a quit, and lends them to no other row; a row that opens on hard is its own row at either difficulty, and only a difficulty that is not the row's own counts as chosen; a row's name left on a URL somebody typed over does not ride out of a quit (the plain page would load the mission again)",
      `${hq} · made ${made} · back ${back} · leaves ${leaves} · quits ${quits} · other rows clean ${plainRow} · the hard row ${walk} · typed over, then quit ${stale}: '${P.pickQuitSearch(typed, LL)}'`); }
  // ---- the maintainer's rules for the list (10-08): what comes first, what the rows may set
  { const p = L[0], e: any = P.pickEntry(p), w = wantOf(p);
    F.verdict(!!e && w.squad === 4 && !!squadsOf(e) && !w.flags.includes('silentstart') && !w.flags.includes('pair'), 'the first row is a four-person loud breach on a squad map (his pick of what comes first)',
      `${p.id}: ${e?.id} on ${e?.level} with ${w.squad}${w.flags.length ? ` · ${w.flags.join(' ')}` : ''}`); }
  { const on = L.every(p => P.onShortList(p.mission)), others = SCENARIOS.filter((e: any) => !L.some(p => p.mission === e.id)).every((e: any) => !P.onShortList(e.id));
    F.verdict(on && others, "a mission the list names is told from the rest (the mission sheet leaves 'untuned' off it)", `on the list ${on} · the rest not ${others} · tagged untuned in the data: ${L.filter(p => (P.pickEntry(p) as any)?.untuned).map(p => p.id).join(', ') || 'none'}`); }
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
  // ---- the one choice means something: no scenario row stands where the build says its hard is the standard squads, and the solo night offers none
  { const DW = (await import(`${ROOT}/src/game/difficulty.ts`)).difficultyWords, same = (p: any) => /the squads here are the standard ones/.test(DW('hard', (P.pickEntry(p) as any)?.level || false));
    const S = L.filter(p => P.pickEntry(p)), bad = [...S.filter(p => same(p) || P.pickOtherDifficulty(p) !== P.otherDifficulty(P.pickDifficulty(p))).map(p => `${p.id}: ${same(p) ? 'hard is the standard squads there and the row still offers it' : 'offers no other difficulty'}`), ...L.filter(p => !P.pickEntry(p) && P.pickOtherDifficulty(p) !== null).map(p => `${p.id}: the solo night offers a difficulty`)];
    F.verdict(bad.length === 0, "every scenario row offers the other difficulty, and hard means something on each; the solo night offers none", bad.length ? bad.join('\n    ') : S.map(p => `${p.id} ${P.pickOtherDifficulty(p)}`).join(' · ')); }
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

// ───────────────────────────────────────────────────────────── boot ─────────────────────────────────────────────────────────────
async function bootChild(query: string, secs: number, kept: string): Promise<any> {
  const p = Bun.spawn(['bun', 'run', import.meta.path, '--boot', query, '--secs', String(secs), '--kept', kept], { cwd: ROOT, stdout: 'pipe', stderr: 'pipe', env: process.env });
  const [out, errText] = await Promise.all([new Response(p.stdout).text(), new Response(p.stderr).text()]); await p.exited;   // (both pipes drained together: a child loud on one never waits on the other)
  const line = out.split('\n').find(l => l.startsWith(TAG));
  return line ? JSON.parse(line.slice(TAG.length)) : { query, threw: `the boot printed no report (exit ${p.exitCode}): ${(errText || out).trim().split('\n').slice(-3).join(' | ')}` };
}
export async function picksBoot(secs = swept(6, 30)): Promise<boolean> {
  const F = family(`picks-boot: a sample of the rows' URLs booted in a process of its own on a machine that keeps the other difficulty — its level, its team, the URL's settings in force — then ${secs} s of the simulation`);
  const L: any[] = P.PICKS, missions: any[] = ALL_MISSIONS;
  // (every row at its own difficulty, and the first row once more at the other: the one choice the card offers on a row)
  const C: { p: any; i: number; d?: string }[] = sample([...L.map((p, i) => ({ p, i })), ...(P.pickEntry(L[0]) ? [{ p: L[0], i: 0, d: otherOf(wantOf(L[0]).difficulty) }] : [])], 3);
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
        if (r.active !== null || r.operatives !== 0) why.push(`a scenario is up (${r.active}) or somebody stands beside Sam (${r.operatives})`);
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


if (import.meta.main) {
  const want = process.argv.slice(2).filter(a => !a.startsWith('-')); const on = (n: string) => !want.length || want.includes(n) || want.includes('picks');
  let ok = true;
  if (on('picks-list')) ok = (await picksList()) && ok;
  if (on('picks-boot')) ok = (await picksBoot()) && ok;
  process.exit(ok ? 0 : 1);
}
