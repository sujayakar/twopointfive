// The 'consulate' family — the four's hostage rescue on its own level (levels/consulate.json ← tools/levels/consulate.ts, its rescue in the file's `scenarios`
// block; the mission row `rescue-consulate`, scene/scenarioList.ts):
//   bun run tools/qa/probes.ts consulate | consulate-level | consulate-stage | consulate-run
//
//   consulate-level   the file is the generator's level (0 diffs), the validator and the lint are clean on it, it is inside the renderer's fixed sizes, the
//                     rescue's data sits on its floor (rescue.ts checkRescue, the route's own check included), the mission row boots it with four
//   consulate-stage   the rescue staged with four on eight seeds: the four and Sam on the level's marks, three hostages, the captor and a second gun in the
//                     consul's office, the door man by its door, THREE roamers on three of the five marks, every inside
//                     door shut but the records room's hall door; nobody fires and no clock runs while Sam stands. With the pair (the Loadout's smaller
//                     team) it stages too. Every try starts the same way (src/game/samestart.ts, a settled rule): the eight seeds stand the three places
//                     written first. (Until 2026-11-07 the eight were staged again with that rule written off, to read the deal behind it; that pass
//                     and its line 'which three differs from staging to staging' are out)
//   consulate-run     the scripted commander (tools/qa/playthrough.ts as the page ships — ships.ts) on six seeds, both ways in: F at the main door, and F
//                     from the side door's room (PT_RESCUE=split). Every run ends on the scenario's own result; each way brings hostages out at least once;
//                     pressed from the side door's room with four, the side door is taken by a PAIR (two men of one colour) and the main door by the other;
//                     with the pair staged (PT_SQUAD=2) by one man — the rule as it was. The counts are printed, not judged: the tally is
//                     bun run tools/qa/ships.ts --scn rescue-consulate --seeds 1-144 --jobs 4 [--hard] --json out.json   (PT_RESCUE=split for the other way in)
import { readFileSync } from 'node:fs';
import { standUp, ROOT } from './headless.ts';
import { exporterCheck } from './level-file.ts';
const { RESCUE, checkRescue } = await import(`${ROOT}/src/game/rescue.ts`);
const { loadLevel } = await import(`${ROOT}/src/scene/levelLoad.ts`);
const { parseLevelData, validateLevelData } = await import(`${ROOT}/src/scene/levelData.ts`);
const { lintLevel } = await import(`${ROOT}/src/scene/lint.ts`);
const { CONSULATE_ROOMS } = await import(`${ROOT}/tools/levels/consulate-floor.ts`);
const { WORLD } = await import(`${ROOT}/src/scene/world.ts`);
const { BoxFlag, MAX_GLASS } = await import(`${ROOT}/src/scene/boxes.ts`);
const { SCENARIOS, specOfQuery, entryOfQuery } = await import(`${ROOT}/src/scene/scenarioList.ts`);
const { scenarioQuery } = await import(`${ROOT}/src/scene/scenarios.ts`);

type Rect = { x0: number; x1: number; z0: number; z1: number };
const DT = 1 / 60, ID = 'rescue-consulate';
function seeded(a: number) { return () => { a |= 0; a = (a + 0x6D2B79F5) | 0; let t = Math.imul(a ^ (a >>> 15), 1 | a); t = (t + Math.imul(t ^ (t >>> 7), 61 | t)) ^ t; return ((t ^ (t >>> 14)) >>> 0) / 4294967296; }; }
const f1 = (x: number) => (Number.isFinite(x) ? x.toFixed(1) : '—');
const flat = (a: number[], b: number[]) => Math.hypot(a[0] - b[0], a[2] - b[2]);
const inRect = (p: number[], b: Rect, m = 0) => p[0] > b.x0 - m && p[0] < b.x1 + m && p[2] > b.z0 - m && p[2] < b.z1 + m;
type Verdict = (ok: boolean, label: string, detail: string) => void;
function family(title: string): { verdict: Verdict; done: () => boolean } {
  console.log(`\n=== ${title} ===`); let all = true;
  return { verdict: (ok, label, detail) => { all &&= ok; console.log(`- ${ok ? 'PASS' : 'FAIL'} — ${label}\n    ${detail}`); }, done: () => { console.log(`=== ${title.split(':')[0]}: ${all ? 'ALL PASS' : 'FAILURES above'} ===`); return all; } };
}
/** a probe that throws is a FAIL with its first lines; whatever it did to the scenario's knobs is put back */
const { samAt: _a, samYaw: _b, opsAt: _c, ...KNOBS0 } = { ...RESCUE } as any;
function guarded(fn: () => Promise<boolean>): () => Promise<boolean> {
  return async () => { const log = console.log; try { return await fn(); } catch (e: any) { console.log = log; console.log(`- FAIL — the probe threw\n    ${String(e?.stack ?? e).split('\n').slice(0, 4).join(' | ')}`); return false; } finally { console.log = log; Object.assign(RESCUE, KNOBS0); } };
}
const FILE = `${ROOT}/levels/consulate.json`;
const fileLevel = () => loadLevel(parseLevelData(readFileSync(FILE, 'utf8')));
const ROOMS = CONSULATE_ROOMS as Record<string, Rect>;
const spaceOf = (p: number[]): string => Object.entries(ROOMS).find(([, r]) => inRect(p, r))?.[0] ?? Object.entries(ROOMS).find(([, r]) => inRect(p, r, 0.4))?.[0] ?? 'outside';
async function quiet<T>(fn: () => Promise<T>): Promise<T> { const log = console.log; console.log = () => {}; try { return await fn(); } finally { console.log = log; } }
async function stand(seed: number, squad: number): Promise<any> {
  Object.assign(RESCUE, KNOBS0); RESCUE.squad = squad as any; Math.random = seeded(0x7e5c + seed * 7919);
  const W: any = await quiet(() => standUp({ uploadEvery: 2, level: fileLevel() })); W.game.tune.chaos = 0.5; W.game.fogOfWar.on = true;
  return W;
}

// ---------------------------------------------------------------- consulate-level
export const consulateLevel = guarded(async () => {
  const { verdict, done } = family("consulate-level: the file, the lint, the sizes, the rescue's data on its floor, the mission row");
  const data = parseLevelData(readFileSync(FILE, 'utf8')), errs: string[] = validateLevelData(data);
  verdict(errs.length === 0, 'the file validates', errs.length ? errs.slice(0, 6).join(' · ') : `${data.boxes.length} boxes · ${data.doors.length} doors · ${data.props.length} props · scenarios ${Object.keys(data.scenarios ?? {}).join(', ')}`);
  { const ex = exporterCheck('consulate'); verdict(ex.ok, "the file is the generator's level, and its look twin with it (the exporter's own answer: tools/qa/level-file.ts)", ex.said); }
  const lint = lintLevel(data), bad = lint.findings.filter((f: any) => f.severity !== 'info');
  verdict(lint.errors === 0 && lint.warnings === 0, 'the level lint is clean (no error, no warning)', bad.length ? bad.slice(0, 5).map((f: any) => `${f.severity}: ${f.message}`).join(' · ') : `${lint.infos} notes in all`);
  const L = loadLevel(data), glass = L.boxes.filter((b: any) => (b.flags ?? 0) & BoxFlag.Glass).length;
  const out = L.boxes.filter((b: any) => b.c[0] - b.h[0] < WORLD.min[0] - 1e-6 || b.c[0] + b.h[0] > WORLD.max[0] + 1e-6 || b.c[2] - b.h[2] < WORLD.min[2] - 1e-6 || b.c[2] + b.h[2] > WORLD.max[2] + 1e-6).length;
  verdict(L.boxes.length <= WORLD.maxBoxes && glass <= MAX_GLASS && out === 0, "inside the renderer's fixed sizes: the world box, the box count, the glass panes", `${L.boxes.length} boxes of ${WORLD.maxBoxes} · ${glass} glass panes of ${MAX_GLASS} · ${out} boxes outside the world box`);
  const W = await stand(1, 4), g = W.game, S = g.level.scenarios.rescue; W.step(1 / 60);   // (the first tick stands the host the rule's call crosses to)
  const was: string[] = checkRescue(g);
  verdict(was.length === 0, "the rescue's data sits on the floor (checkRescue, the route's own check included)", was.length ? was.join(' · ') : `every mark standable and clear of the furniture; the room's men inside it; a way on foot to every mark; ${S.flow.route.length} openings on the route, each with a stand and a watch`);
  const men = 2 + (S.guards?.length ?? 0) + (S.cast?.roamers ?? 0), chars = 1 + 4 + men + S.hostages.length;
  verdict(men <= 11 && chars <= 16 && (S.cast?.roamers ?? 0) < S.roamers.length, 'the cast fits: eleven defenders at most, sixteen skinned characters in all, fewer roamers than places for them', `${men} defenders (the captor, ${S.guards?.length ?? 0} more in the room, the door man, ${S.cast?.roamers} roamers on ${S.roamers.length} marks) · ${chars} characters with Sam, the four and ${S.hostages.length} hostages`);
  const row = SCENARIOS.find((e: any) => e.id === ID), q = row ? scenarioQuery(row) : '', spec = row ? specOfQuery(new URLSearchParams(q)) : null, back = row ? entryOfQuery(new URLSearchParams(q)) : null;
  verdict(!!row && row.squad === 4 && row.defenders === men && spec?.kind === 'rescue' && spec?.squad === 4 && spec?.level === 'consulate.json' && back?.id === ID, "the mission row boots this level's rescue WITH FOUR (its query reads back to four teammates and to the row), its defenders cell is the cast", `${q} → ${spec ? `${spec.kind}, squad ${spec.squad}, ${spec.level}` : 'no scenario'} · defenders cell ${row?.defenders}`);
  return done();
});

// ---------------------------------------------------------------- consulate-stage
export const consulateStage = guarded(async () => {
  const { verdict, done } = family('consulate-stage: the rescue boots on the level with four — the cast on its marks, three roamers of five places, nothing moves until Sam does');
  const bad: string[] = [], SEEDS = [1, 2, 3, 4, 5, 6, 7, 8]; let rounds = 0, clocks = 0, chars = 0, written = '';
  /** eight stagings, every one read the same way */
  const pass = async (): Promise<string[]> => { const draws: string[] = [];
    for (const seed of SEEDS) { const tag = `seed ${seed}`;
      const W = await stand(seed, 4), g = W.game, S = g.level.scenarios.rescue, R = g.rescue;
      const why = await quiet(async () => R.start()); if (why) { bad.push(`${tag}: start refused — ${why}`); continue; }
      const near = (who: string, p: number[], m: number[]) => { if (flat(p, m) > 0.6) bad.push(`${tag}: ${who} is ${f1(flat(p, m))} m off the mark`); };
      if (R.ops.length !== 4) bad.push(`${tag}: ${R.ops.length} teammates`); R.ops.forEach((o: any, i: number) => near(o.callsign, o.char.pos, S.team.ops[i])); near('Sam', g.player.char.pos, S.team.sam);
      const pairs = R.ops.map((o: any) => o.unit?.pair).join(','); if (pairs !== 'red,red,blue,blue') bad.push(`${tag}: the four are not a red pair and a blue pair (${pairs})`);
      if (R.hostages.length !== 3) bad.push(`${tag}: ${R.hostages.length} hostages`); R.hostages.forEach((h: any, i: number) => { near(`hostage ${i + 1}`, h.char.pos, S.hostages[i].pos); if (!inRect(h.char.pos, R.bounds)) bad.push(`${tag}: hostage ${i + 1} outside the room`); });
      if (R.hostiles.length !== 6) bad.push(`${tag}: ${R.hostiles.length} of theirs, not 6`);
      near('the captor', R.hostiles[0].char.pos, S.captor.pos); near('the door man', R.hostiles[1].char.pos, S.doorMan.outside.pos); near('the room guard', R.hostiles[2].char.pos, S.guards[0].pos);
      const inRoom = R.hostiles.filter((h: any) => inRect(h.char.pos, R.bounds)).length; if (inRoom !== 2) bad.push(`${tag}: ${inRoom} of theirs in the consul's office, not 2`);
      const ro = R.hostiles.slice(3).map((h: any) => spaceOf(h.char.pos)); draws.push(ro.join('+'));
      written = S.roamers.slice(0, Math.round(S.cast?.roamers ?? 3)).map((m: any) => spaceOf(m.pos)).join('+');
      if (new Set(ro).size !== ro.length || ro.some((s: string) => !['records', 'visa', 'waiting', 'kitchen', 'back'].includes(s))) bad.push(`${tag}: the roamers stand in ${ro.join(', ')}`);
      for (const d of g.doors.list) if (!d.def.exterior && d.def.name !== 'records_hall' && Math.abs(d.angle) > 0.05) bad.push(`${tag}: door '${d.def.name}' is open at the staging`);
      if (Math.abs(g.doors.byName('records_hall').angle) < 0.8) bad.push(`${tag}: the records room's hall door is not standing open`);
      chars = Math.max(chars, 1 + g.guards.length + (g.civilians?.length ?? 0));
      const seen = new Set<any>(); await quiet(async () => { for (let i = 0; i < 300; i++) { W.step(DT); for (const e of g.events) if (!seen.has(e)) { seen.add(e); if (e.kind === 'guardShot' || e.kind === 'shot') rounds++; } } });
      if (!isNaN(R.t0)) clocks++; if (R.result) bad.push(`${tag}: over before Sam moved — ${R.result.why}`);
    }
    return draws; };
  const draws = await pass();
  // (src/game/samestart.ts: every try starts the same way — the places written first, whatever the seed)
  verdict(new Set(draws).size === 1 && draws[0] === written, 'every try starts the same way: the three places written first hold a man on all eight seeds', `${new Set(draws).size} different draws on eight seeds: ${[...new Set(draws)].join(' · ')} · written first: ${written}`);
  verdict(bad.length === 0, `staged on eight seeds: the four (a red pair, a blue pair) and Sam inside the front door, three hostages, the captor and a second gun in the consul's office, the door man by its door, three roamers each in a room of his own, every inside door shut but the records room's hall door`, bad.length ? bad.slice(0, 6).join(' · ') : `${chars} characters at most (the skinning holds 16)`);
  verdict(rounds === 0 && clocks === 0, "five seconds with Sam standing: nobody fires and the captor's clock has not started", `${rounds} rounds · the clock running on ${clocks} stagings`);
  { const W = await stand(1, 2), R = W.game.rescue, why = await quiet(async () => R.start());
    verdict(!why && R.ops.length === 2 && R.hostiles.length === 6, 'with the pair (the Loadout\'s smaller team) it stages too: two teammates, the same six against them', why ? `refused — ${why}` : `${R.ops.length} teammates · ${R.hostiles.length} of theirs`); }
  { // who F takes from the side door's room (rescue.ts pick): all four in there = a pair of one colour on the side door, the other pair on the main door;
    // one pair alone in there = one and one — the main door is never left to nobody
    const W = await stand(1, 4), g = W.game, R = g.rescue as any; await quiet(async () => R.start());
    R.ops.forEach((o: any, i: number) => { o.char.pos[0] = 25.4 + (i % 2) * 1.2; o.char.pos[2] = 16.5 + (i >> 1) * 1.2; }); g.teleportPlayer([26.5, 0, 19.5]);
    const names = (l: any[]) => l.map(o => o.callsign).join('+') || 'nobody', all = R.pick();
    R.ops[2].state = 'dead'; R.ops[3].state = 'dead'; const reds = R.pick();
    verdict(!!all && all.side.length === 2 && all.side[0].unit.pair === all.side[1].unit.pair && all.front.length === 2 && all.front.every((o: any) => o.unit.pair !== all.side[0].unit.pair) && !!reds && reds.side.length === 1 && reds.front.length === 1,
      'F from the back office: with the four in it a pair takes the side door and the other pair the main door; with one pair left, one man each',
      `four: side ${all ? names(all.side) : '—'} · main ${all ? names(all.front) : '—'} — one pair left: side ${reds ? names(reds.side) : '—'} · main ${reds ? names(reds.front) : '—'}`); }
  return done();
});

// ---------------------------------------------------------------- consulate-run
