// The 'loadout' family (the LOADOUT SCREEN and the squad sizes behind it — src/game/loadout.ts, src/ui/loadout.ts; the staging in nightbreach.ts / rescue.ts),
// run through probes.ts:
//   bun run tools/qa/probes.ts loadout | loadout-pure | loadout-stage
//   bun run tools/qa/loadout-probes.ts --table        the scenario × size table as markdown (what loadout-stage boots, one row a scenario)
// PASS / FAIL each, all headless. Nothing here looks at a picture, and a 'boot' is a CHILD PROCESS (this file with --boot '<query>'), as a pick is a page load: fresh modules, the level
// the query names, the harness Game, game/scenarioRun.ts startFromQuery — the function main.ts calls.
//   loadout-pure     the size as data: `?squad=N` and `?squad4` read back (squad=N wins; the rescue never reads the bare flag; a bad N asks for nothing); every
//                    kind offers five sizes with exactly one its own; the roster of each size; Deploy's query string — untouched at the mission's own size, else
//                    `squad=N` in place of `squad4`, the visitor's other flags kept — reads back to that size; the store remembers per mission and survives junk
//   loadout-stage    every scenario on the list booted at every size 0‥4 (60 boots, and 12 more by the row's own URL): it starts unrefused (or the screen greys that size, with the reason), stands
//                    exactly N teammates with the roster's names and pairs on free floor a body apart, the F drill exists only with two or more on a level with
//                    a breach plan, the pause card's words say the size, ten seconds of play throw nothing, a restart puts the same N back, a quit leaves the
//                    knobs at the pair; with every defender dropped by the player's hand the scenario ends won at every size, alone too (the room clear: taken; the rescue: Sam holds the hostage room himself; the night breach with fewer than two: 'room clear' — with two or more that end is the F drill's, as ever); and at its OWN size the boot by `squad=N` stands the same men on the same marks as the row's own URL
// The screen itself is walked by tools/qa/cards-page.ts, on the desk's module.
import { menuKey } from './squadwords';
import { standUp, ROOT } from './headless.ts';
import { pool, jobsOf } from './pool.ts';
import { resolve } from 'node:path';
import { readFileSync } from 'node:fs';
import { El, withDom } from './scenario-picker-probes.ts';
import { sample } from './sweep.ts';

const { SCENARIOS, specOfQuery, pickSearch, quitSearch, teamWords } = await import(`${ROOT}/src/scene/scenarioList.ts`);
const { scenarioQuery } = await import(`${ROOT}/src/scene/scenarios.ts`);
const { startFromQuery, quitScenario, activeScenario, runningScenario, teamOf } = await import(`${ROOT}/src/game/scenarioRun.ts`);
const L = await import(`${ROOT}/src/game/loadout.ts`);
const K = await import(`${ROOT}/src/game/kit.ts`);
const { NIGHTBREACH } = await import(`${ROOT}/src/game/nightbreach.ts`);
const { RESCUE } = await import(`${ROOT}/src/game/rescue.ts`);
const { killGuard } = await import(`${ROOT}/src/game/combat.ts`);
const { parseLevelData } = await import(`${ROOT}/src/scene/levelData.ts`);
const { loadLevel } = await import(`${ROOT}/src/scene/levelLoad.ts`);

const DT = 1 / 60;
function seeded(a: number) { return () => { a |= 0; a = (a + 0x6D2B79F5) | 0; let t = Math.imul(a ^ (a >>> 15), 1 | a); t = (t + Math.imul(t ^ (t >>> 7), 61 | t)) ^ t; return ((t ^ (t >>> 14)) >>> 0) / 4294967296; }; }
const f2 = (x: number) => (Number.isFinite(x) ? x.toFixed(2) : String(x));
type Verdict = (ok: boolean, label: string, detail: string) => void;
function family(title: string): { verdict: Verdict; done: () => boolean } {
  console.log(`\n=== ${title} ===`); let all = true;
  return { verdict: (ok, label, detail) => { all &&= ok; console.log(`- ${ok ? 'PASS' : 'FAIL'} — ${label}\n    ${detail}`); }, done: () => { console.log(`=== ${title.split(':')[0]}: ${all ? 'ALL PASS' : 'FAILURES above'} ===`); return all; } };
}
function levelOfFile(file: string): any { const data = parseLevelData(readFileSync(resolve(ROOT, 'levels', file), 'utf8')); return loadLevel(data); }
const memStore = () => { const m = new Map<string, string>(); return { getItem: (k: string) => m.get(k) ?? null, setItem: (k: string, v: string) => { m.set(k, v); }, m }; };

// ───────────────────────────────────────────────────────────── a boot, in a process of its own ─────────────────────────────────────────────────────────────
interface Man { name: string; pos: number[]; pair: string | null; alive: boolean; blocked: boolean }
interface Shot { active: string | null; team: string | null; title: string | null; ops: Man[]; hostiles: number; drill: boolean; noDrill: boolean; plan: boolean; knobNB: number; knobRS: number; ordersActive: boolean; line: string }
interface BootReport { query: string; spec: any; why: string | null; start: Shot; start5: Shot; after: Shot; restart: Shot; won: string | null; quit: Shot; errors: string[]; threw: string | null }
function shotOf(W: any, levelFile: string | null, line = ''): Shot {
  const g = W.game, run = runningScenario(g, levelFile), ops: any[] = teamOf(g);
  return { active: activeScenario(g), team: run?.team ?? null, title: run?.title ?? null,
    ops: ops.map((gd: any) => ({ name: gd.callsign, pos: [...gd.char.pos], pair: gd.unit?.pair ?? null, alive: gd.state !== 'dead', blocked: g.col.nav.isBlocked(gd.char.pos[0], gd.char.pos[2]) })),
    hostiles: g.guards.filter((x: any) => x.unit && x.team === 'hostile').length, drill: !!g.nightBreach.drill, noDrill: !!g.nightBreach.noDrill, plan: !!g.level.breach,
    knobNB: NIGHTBREACH.squad, knobRS: RESCUE.squad, ordersActive: !!g.orders.active, line };
}
async function boot(query: string): Promise<BootReport> {
  const errors: string[] = []; let threw: string | null = null;
  const q = new URLSearchParams(query), lf = q.get('levelFile'), levelFile = lf && /^[\w.-]+\.json$/.test(lf) ? lf : null;
  const R: Partial<BootReport> = { query, spec: specOfQuery(q) };
  const err = console.error; console.error = (...a: unknown[]) => { errors.push(a.map(String).join(' ').split('\n')[0]); };
  try {
    Math.random = seeded(0x51c4);
    const W = await standUp({ uploadEvery: 4, ...(levelFile ? { level: levelOfFile(levelFile) } : {}) });
    const g = W.game; g.godMode = true;
    let line = ''; const msg = g.msg.bind(g); g.msg = (t: string, ...r: unknown[]) => { if (!line && /armed enemies|hostage|rescue/i.test(t)) line = t; return msg(t, ...r); };
    const { why } = startFromQuery(g, q); R.why = why;
    R.start = shotOf(W, levelFile, line);
    // ten seconds of play with the keys a player presses first: F (the drill's word, or the rescue's), then X twice (the order menu up and away)
    for (let i = 0; i < 600; i++) { g.player.visibility = 0.05; if (i === 60 || i === 200) W.input.pressed.add('KeyF'); if (i === 320 || i === 340) menuKey(g.orders); W.step(DT); if (i === 4) R.start5 = shotOf(W, levelFile); }
    R.after = shotOf(W, levelFile);
    g.restartEncounter();
    for (let i = 0; i < 5; i++) { g.player.visibility = 0.05; W.step(DT); }
    R.restart = shotOf(W, levelFile);
    // every defender dropped by the player's hand: the scenario's own end, whoever came (alone, nobody else could have won it)
    { let said = ''; const m2 = g.msg.bind(g); g.msg = (t: string, ...r: unknown[]) => { if (/room clear|rooms clear|CLEAN|COSTLY|FAILED|rescued|lost/i.test(t)) said = t; return m2(t, ...r); };
      for (const h of g.guards.filter((x: any) => x.unit && x.team === 'hostile' && x.state !== 'dead')) killGuard(g, h, [1, 0, 0], false, 'shot', 'player');
      const B = g.rescue?.active ? (g.rescue as any).bounds : null; if (B) g.teleportPlayer([(B.x0 + B.x1) / 2, 0, (B.z0 + B.z1) / 2]);   // (the rescue is won by HOLDING the hostage room: Sam stands in it himself)
      for (let i = 0; i < 180 + (B ? Math.ceil(RESCUE.holdSecs * 60) : 0); i++) { g.player.visibility = 0.05; W.step(DT); }
      R.won = g.roomClear?.active ? `${g.roomClear.end} (${g.roomClear.endWhy})` : g.rescue?.active ? (g.rescue.result ? `${g.rescue.result.state} ${g.rescue.result.grade}` : null) : said ? said.replace(/\. Shift.*$/, '') : null; }
    quitScenario(g, false);
    for (let i = 0; i < 30; i++) { g.player.visibility = 0.05; W.step(DT); }
    R.quit = shotOf(W, levelFile);
  } catch (e: any) { threw = String(e?.stack ?? e).split('\n').slice(0, 4).join(' | '); }
  finally { console.error = err; }
  return { ...(R as BootReport), errors, threw };
}
const TAG = 'LOADOUT-BOOT ';
if (import.meta.main && process.argv.includes('--boot')) {
  const query = process.argv[process.argv.indexOf('--boot') + 1] ?? '';
  const log = console.log; console.log = console.warn = () => {};
  const r = await boot(query);
  log(TAG + JSON.stringify(r));
  process.exit(0);
}
async function bootChild(query: string): Promise<BootReport> {
  const p = Bun.spawn(['bun', 'run', import.meta.path, '--boot', query], { cwd: ROOT, stdout: 'pipe', stderr: 'pipe', env: process.env });
  const out = await new Response(p.stdout).text(), errText = await new Response(p.stderr).text(); await p.exited;
  const line = out.split('\n').find(l => l.startsWith(TAG));
  if (!line) return { query, spec: null, errors: [], threw: `the boot printed no report (exit ${p.exitCode}): ${(errText || out).trim().split('\n').slice(-3).join(' | ')}` } as unknown as BootReport;
  return JSON.parse(line.slice(TAG.length));
}

// ───────────────────────────────────────────────────────────── loadout-pure ─────────────────────────────────────────────────────────────
export async function loadoutPure(): Promise<boolean> {
  const { verdict, done } = family('loadout-pure: the squad size as data');
  { const rd = (s: string, f = true) => L.squadOfQuery(s, f);
    const got = [rd('?squad=0'), rd('?squad=1'), rd('?squad=3'), rd('?squad4'), rd('?squad4&squad=1'), rd('?squad=9'), rd('?squad=many'), rd('?roomclear'), rd('?squad4', false), rd('?squad=4', false), rd('?squad=')];
    const want = [0, 1, 3, 4, 1, null, null, null, null, 4, null];
    verdict(JSON.stringify(got) === JSON.stringify(want), '?squad=N reads 0‥4, ?squad4 is 4, squad=N wins, junk asks for nothing', `read ${JSON.stringify(got)}, want ${JSON.stringify(want)}`); }
  { const s = (x: string) => specOfQuery(x)?.squad ?? null, k = (x: string) => specOfQuery(x)?.kind ?? null;
    const rows: [string, string | null, number | null][] = [['?roomclear', 'roomclear', 2], ['?squad4', 'roomclear', 4], ['?roomclear&squad=3', 'roomclear', 3], ['?roomclear&squad4&squad=1', 'roomclear', 1], ['?nightbreach&squad=0', 'nightbreach', 0],
      ['?nightbreach&squad4', 'nightbreach', 4], ['?rescue', 'rescue', 2], ['?rescue&squad4', 'rescue', 2], ['?rescue&squad=4', 'rescue', 4], ['?rescue&squad=0', 'rescue', 0], ['?squad=3', null, null], ['?roomclear&squad=7', 'roomclear', 2]];
    const bad = rows.filter(([q, kind, n]) => k(q) !== kind || s(q) !== n);
    verdict(!bad.length, 'the page reads a scenario with its size: the old URLs as ever, squad=N on any of the three kinds, alone it names no scenario', bad.length ? bad.map(([q]) => `${q} → ${k(q)} / ${s(q)}`).join(' · ') : rows.map(([q, , n]) => `${q} → ${n ?? 'none'}`).join(' · ')); }
  { const bad: string[] = [];
    for (const e of SCENARIOS) { const C = L.squadChoices(e); if (C.length !== 5 || C.filter((c: any) => c.own).length !== 1 || C.find((c: any) => c.own).n !== e.squad || !C.find((c: any) => c.own).ok) bad.push(e.id); for (const c of C) if (!c.ok && !c.note) bad.push(`${e.id}:${c.n} greyed with no reason`); }
    verdict(!bad.length, 'every mission offers five sizes, exactly one its own (and that one can be staged); a greyed size says why', bad.join(' · ') || `${SCENARIOS.length} missions · greyed: ${SCENARIOS.flatMap((e: any) => L.squadChoices(e).filter((c: any) => !c.ok).map((c: any) => `${e.id}:${c.n}`)).join(', ') || 'none'}`); }
  { const R = [0, 1, 2, 3, 4].map(n => L.rosterOf(n).map((m: any) => `${m.name}/${m.pair}`).join(', '));
    const want = ['', 'Blue One/blue', 'Blue One/blue, Blue Two/blue', 'Red One/red, Red Two/red, Blue One/blue', 'Red One/red, Red Two/red, Blue One/blue, Blue Two/blue'];
    verdict(JSON.stringify(R) === JSON.stringify(want), 'the roster of each size: the pair as ever up to two, the four-man squad\'s first three or all of it past that', R.map((r, n) => `${n}: ${r || 'nobody'}`).join(' · '));
    const w = [0, 1, 2, 3, 4].map(n => teamWords({ squad: n })); verdict(w.join('|') === 'solo|you + one|you + the pair|you + three|you + four', 'the words for each size', w.join(' · ')); }
  { const bad: string[] = []; let same = 0, changed = 0;
    for (const e of SCENARIOS) { const pick = pickSearch(e, '?quality=low&mute&loadout=' + e.id);
      for (const n of [0, 1, 2, 3, 4]) { const to = L.squadSearch(pick, n, e.squad), spec = specOfQuery(to), q = new URLSearchParams(to);
        if (n === e.squad) { same++; if (to !== pickSearch(e, '?quality=low&mute')) bad.push(`${e.id}@${n}: its own size changed the URL (${to})`); } else { changed++; if (q.has('squad4') || q.get('squad') !== String(n)) bad.push(`${e.id}@${n}: ${to}`); }
        if (!spec || spec.kind !== e.kind || spec.squad !== n || (spec.level ?? '') !== (e.level ?? '')) bad.push(`${e.id}@${n}: reads back as ${JSON.stringify(spec)}`);
        if (q.get('quality') !== 'low' || !q.has('mute') || q.has('loadout')) bad.push(`${e.id}@${n}: the visitor's flags (${to})`);
        if (quitSearch(to) !== '?quality=low&mute') bad.push(`${e.id}@${n}: a quit leaves ${quitSearch(to)}`); } }
    verdict(!bad.length, 'Deploy\'s query: the row\'s own URL untouched at its own size, squad=N in place of squad4 otherwise; reads back to that size; other flags ride; a quit drops it', bad.slice(0, 4).join(' · ') || `${same} at the mission's own size, ${changed} changed · e.g. ${L.squadSearch(pickSearch(SCENARIOS[2], ''), 3, 4)}`); }
  return done();
}

// ───────────────────────────────────────────────────────────── loadout-stage ─────────────────────────────────────────────────────────────
interface Cell { e: any; n: number; own: boolean; ok: boolean; note: string; r: BootReport | null; ref?: BootReport; bad: string[] }
async function stageAll(only?: string[]): Promise<Cell[]> {
  const cells: Cell[] = [];
  for (const e of sample(SCENARIOS as any[], 1)) if (!only || only.includes(e.id)) for (const c of sample(L.squadChoices(e) as any[], 2)) cells.push({ e, n: c.n, own: c.own, ok: c.ok, note: c.note, r: null, bad: [] });
  const jobs = jobsOf(process.env.LOADOUT_JOBS, 4, 4);
  await pool(cells, jobs, async c => {
    const own = scenarioQuery(c.e) as string, q = c.own ? own.replace(/&squad4\b/, '') + `&squad=${c.n}` : L.squadSearch(own, c.n, c.e.squad);   // (its own size too is booted BY squad=N: the row's own URL is the reference below)
    c.r = await bootChild(q); if (c.own) c.ref = await bootChild(own);
    const r = c.r, bad = c.bad, lf = c.e.level ?? null;
    if (r.threw) { bad.push(`threw: ${r.threw}`); return; }
    // (a row with `only` — scenarios.ts ScenarioEntry — greys every size but its own BY DESIGN, a mission built for one size as a Training row is, and says such
    //  a size still stages from a URL: that is what is held of it here. Every other greyed size is a claim that it CANNOT stage)
    if (!c.ok && c.e.only && c.note === c.e.only) { if (r.why || r.start.ops.length !== c.n) bad.push(`greyed by the row's own-size rule, which still stages — but ${r.why ? `refused: ${r.why}` : `${r.start.ops.length} teammates came`}`); return; }
    if (!c.ok) { if (!r.why && r.start.ops.length === c.n) bad.push('greyed on the screen, but it stages'); return; }
    if (r.why) { bad.push(`refused: ${r.why}`); return; }
    if (r.errors.length) bad.push(`console errors: ${r.errors.slice(0, 2).join(' | ')}`);
    const want = c.e.kind === 'rescue' && c.n <= 2 ? L.rosterOf(c.n).map((m: any) => `${m.name}/null`) : L.rosterOf(c.n).map((m: any) => `${m.name}/${m.pair}`);   // (the rescue's pair is in no pair of the order menu's, as ever)
    for (const [when, s] of [['staged', r.start], ['restarted', r.restart]] as [string, Shot][]) {
      if (s.active !== c.e.kind) bad.push(`${when}: ${s.active} is up, not ${c.e.kind}`);
      if (s.ops.length !== c.n) bad.push(`${when}: ${s.ops.length} teammates, not ${c.n}`);
      if (s.ops.map(m => `${m.name}/${m.pair}`).join() !== want.join()) bad.push(`${when}: ${s.ops.map(m => `${m.name}/${m.pair}`).join(', ') || 'nobody'} — the roster is ${want.join(', ') || 'nobody'}`);
      if (s.ops.some((m, i) => !m.alive || (m.blocked && !(c.e.kind === 'rescue' && i < 2)))) bad.push(`${when}: a man is down or stands in something`);   // (the rescue's first two marks are its shipped stack against the wall — a blocked nav cell at ec3a2ec too; the men a size adds must stand on free floor)
      for (let i = 0; i < s.ops.length; i++) for (let j = i + 1; j < s.ops.length; j++) if (Math.hypot(s.ops[i].pos[0] - s.ops[j].pos[0], s.ops[i].pos[2] - s.ops[j].pos[2]) < 0.6) bad.push(`${when}: ${s.ops[i].name} and ${s.ops[j].name} stand in each other`);
      if (s.team !== teamWords({ squad: c.n })) bad.push(`${when}: the pause card says '${s.team}'`);
      if (s.title !== c.e.title && !(SCENARIOS.some((x: any) => x !== c.e && x.kind === c.e.kind && (x.level ?? null) === lf))) bad.push(`${when}: the pause card names it '${s.title}'`);
      if (c.e.kind !== 'rescue') { const drill = s.plan && c.n >= 2; if (s.drill !== drill || s.noDrill !== !drill) bad.push(`${when}: F's drill ${s.drill ? 'exists' : 'does not'} with ${c.n} on a level ${s.plan ? 'with' : 'without'} a breach plan`); }
      if ((c.e.kind === 'rescue' ? s.knobRS : s.knobNB) !== c.n) bad.push(`${when}: knobs ${s.knobNB} / ${s.knobRS}`);
      if (s.ordersActive !== c.n > 0) bad.push(`${when}: the order menu is ${s.ordersActive ? 'live' : 'off'} with ${c.n}`);
    }
    if (r.start.hostiles < 1) bad.push('nobody holds the floor');
    if (r.after.active !== c.e.kind || r.after.ops.length !== c.n) bad.push(`after ten seconds: ${r.after.active}, ${r.after.ops.length} men`);
    for (let i = 0; i < c.n; i++) if (r.restart.ops[i] && Math.hypot(r.restart.ops[i].pos[0] - r.start5.ops[i].pos[0], r.restart.ops[i].pos[2] - r.start5.ops[i].pos[2]) > 0.05) bad.push(`a restart stands ${r.start5.ops[i].name} elsewhere`);   // (five frames into each: the office rescue's second mark is in a wall's margin and the man is eased out of it, at ec3a2ec too)
    if ((!r.won && !(c.e.kind === 'nightbreach' && c.n >= 2)) || (c.e.kind === 'roomclear' && !/^taken/.test(r.won ?? '')) || (c.e.kind === 'rescue' && !/^rescued/.test(r.won ?? '')) || (c.e.kind === 'nightbreach' && c.n < 2 && !/clear/.test(r.won ?? ''))) bad.push(`every defender down, and it ends '${r.won}'`);
    if (c.n === 0 && c.e.kind === 'roomclear' && !/alone/.test(r.won ?? '')) bad.push(`alone, and the end reads '${r.won}'`);
    if (r.quit.active !== null || r.quit.ops.length || r.quit.knobNB !== 2 || r.quit.knobRS !== 2) bad.push(`after a quit: ${r.quit.active}, ${r.quit.ops.length} men, knobs ${r.quit.knobNB} / ${r.quit.knobRS}`);
    if (c.ref) { const a = r.start.ops.map(m => `${m.name}@${f2(m.pos[0])},${f2(m.pos[2])}`).join(' '), b = c.ref.start?.ops.map(m => `${m.name}@${f2(m.pos[0])},${f2(m.pos[2])}`).join(' ');
      const a2 = r.after.ops.map(m => `${m.name}@${f2(m.pos[0])},${f2(m.pos[2])}`).join(' '), b2 = c.ref.after?.ops.map(m => `${m.name}@${f2(m.pos[0])},${f2(m.pos[2])}`).join(' ');
      if (a !== b || a2 !== b2 || r.start.hostiles !== c.ref.start?.hostiles) bad.push(`its own size by squad=${c.n} is not the row's own URL: ${a} / ${b}`); }
  });
  return cells;
}
export async function loadoutStage(): Promise<boolean> {
  const { verdict, done } = family('loadout-stage: every scenario at every size, each a page load of its own');
  const cells = await stageAll(process.env.LOADOUT_ONLY?.split(','));
  for (const e of SCENARIOS) { const row = cells.filter(c => c.e === e); if (!row.length) continue;
    const bad = row.filter(c => c.bad.length);
    verdict(!bad.length, `${e.id} (${e.kind}${e.level ? ', ' + e.level : ''}; own ${e.squad}): sizes 0‥4`, bad.length ? bad.map(c => `[${c.n}] ${c.bad.slice(0, 3).join(' · ')}`).join('\n    ')
      : row.map(c => `${c.n}${c.own ? '*' : ''}: ${!c.ok ? `greyed (${c.note})` : `${c.r!.start.ops.map(m => m.name).join(', ') || 'alone'}${e.kind !== 'rescue' ? (c.r!.start.drill ? ' · F drill' : ' · no drill') : ''}`}`).join(' | ') + ` · ${row[0].r!.start.hostiles} hold the floor`); }
  { const lines = [...new Set(cells.filter(c => c.r?.start?.line && (c.n < 2)).map(c => `${c.n}: ${c.r!.start.line.replace(/^.*?\. /, '').slice(0, 60)}`))].slice(0, 6);
    console.log(`    (the staging's line with fewer than two: ${lines.join(' · ')})`); }
  return done();
}

if (import.meta.main && !process.argv.includes('--boot')) {
  if (process.argv.includes('--table')) {
    const cells = await stageAll(process.env.LOADOUT_ONLY?.split(','));
    console.log('| scenario | kind · level | 0 | 1 | 2 | 3 | 4 |\n|---|---|---|---|---|---|---|');
    for (const e of SCENARIOS) { const row = cells.filter(c => c.e === e); if (!row.length) continue;
      console.log(`| ${e.title} | ${e.kind} · ${e.level ?? 'office'} | ${row.map(c => (c.bad.length ? `FAILS: ${c.bad[0]}` : !c.ok ? `greyed: ${c.note}` : `${c.own ? '**own** ' : ''}works${e.kind !== 'rescue' ? (c.r!.start.drill ? ', F drill' : ', no F drill') : ''}`)).join(' | ')} |`); }
    process.exit(cells.some(c => c.bad.length) ? 1 : 0);
  }
  const ok = [await loadoutPure(), await loadoutStage()].every(Boolean);
  process.exit(ok ? 0 : 1);
}
