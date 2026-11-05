// THE GAME'S OWN SENSES AGAINST THE PICTURE, headless (src/host/modelerr.ts: the plan, the models, the spread, the budget; the page's half is `?modelerr`).
//   bun run tools/qa/modelerr.ts                  (TPS_PROFILE=shipped in front for the other stand: the check holds on both)
//   bun run tools/qa/modelerr.ts --report f.json  (also judge that report: one a page made on a GPU, or one this tool wrote)
//   bun run tools/qa/modelerr.ts --write f.json   (keep this run's own report: the models alone, no picture)
//   --plan quick|full                             (the plan walked here; default full)
// What it holds:
//   walks     the plan walked on the office with no picture: every station stood at, light points and sight lines asked at each — lines at every height
//             of the plan (src/host/modelerr.ts EYES), each height with smoke on some of its lines — every model answering (the grid too: the
//             module runs here), and a second walk on a fresh stand asks the same questions and gets the same answers, to the bit
//   spread    the models alone, said and not judged: the grid against the blobs
//   planted   the walk's own answers planted as a picture: the spread says nothing is off; planted two stops brighter and a quarter of the light
//             darker, the spread says exactly that. So the arithmetic that judges a real picture is right
//   budget    a second walk judged against the first walk's answers as its picture, under a tight budget: inside it
//   waits     a walk with a picture that answers 45 frames late and has places for a third of an age's lines a frame (a stand-in that knows only
//             whether the walk's smoke is in the room when it is asked and when it answers): every light point is read with no smoke out, every sight
//             line with the smoke still there — the last age too, which the first GPU run read from a room the walk had already cleared — the lines
//             of one turn (the first of each height, the second of each, …) go to the picture within a frame of each other, and the models answer
//             what they answer on a walk with no picture, to the bit (the plan's clock stands while the picture answers)
//   reports   every report under tools/qa/fixtures/modelerr/ (and --report's) against the budget in src/host/modelerr.ts BUDGET: as it was recorded, and,
//             where this stand is the one the report was made on — its `level` word this stand's, its scene the plain page's with a seed named —
//             with the models AS THEY ARE NOW against the recorded picture: the report's own questions asked again (its stations, points, lines
//             and ages — whatever a plan deals today), and a report that is asked again must be asked every question it holds, in its order.
//             A REPORT THAT IS NOT ASKED AGAIN HOLDS NOTHING HERE: its line says so in capitals and the last line counts it (a page's report is
//             the page's stand's: TPS_PROFILE=shipped; once the office is edited its picture is of another floor, and a GPU must record a new one)
// MODELERR_BREAK=1 is the red path: the second walk's smoke is three times as thick (SENSE.smoke.sigma), so `budget` must FAIL.
// MODELERR_BREAK=move is the other: the walk does not wait for the picture before it changes it, as it did not, so `waits` must FAIL.
// MODELERR_BREAK=turns is the third: the walk puts its lines to the picture a height at a time, so `waits` must FAIL (its lowest lines frames behind).
// EXIT 0 = every line passes · 1 = one fails. Never seen on a GPU: a picture's numbers are in a report only if a page with a GPU put them there.
import { resolve } from 'path';
import { readdirSync, readFileSync, writeFileSync, existsSync } from 'fs';
import { standUp } from './headless.ts';
const ROOT = resolve(import.meta.dir, '../..');
const ME: any = await import(`${ROOT}/src/host/modelerr.ts`), S: any = await import(`${ROOT}/src/game/sense.ts`), SIM: any = await import(`${ROOT}/src/sim/sim.ts`);
const BREAK = process.env.MODELERR_BREAK === '1', BREAK_MOVE = process.env.MODELERR_BREAK === 'move', BREAK_TURNS = process.env.MODELERR_BREAK === 'turns';
const arg = (n: string) => { const i = process.argv.indexOf(n); return i >= 0 ? process.argv[i + 1] : undefined; };
const planName = arg('--plan') ?? 'full', PLAN = ME.PLANS[planName];
if (!PLAN) { console.log(`modelerr: no plan '${planName}' (${Object.keys(ME.PLANS).join(', ')})`); process.exit(2); }
const log = console.log, err = console.error; const hush = (on: boolean) => { console.log = on ? () => {} : log; console.error = on ? () => {} : err; };
let bad = 0; const verdict = (ok: boolean, what: string, detail: string) => { if (!ok) bad++; log(`- ${ok ? 'PASS' : 'FAIL'} — ${what}\n    ${detail}`); };
{ const no = SIM.simWhyNot(); if (no) { log(`=== modelerr: the module does not run here (${no}) — FAIL ===`); process.exit(1); } }

/** the plan walked on a fresh stand of the plain page's level, with no picture. Seeded: who carries a lamp where is dealt by the game's die, and an unseeded
 *  stand is another world every time (its `world` word says so) */
async function walk(plan: any, name: string, scene = 'seed=1', seed = 1, again: any = null): Promise<any> {
  hush(true);
  try {
    const W = await standUp({ uploadEvery: 1000, seed }); for (let i = 0; i < 3; i++) W.step(1 / 60);
    const run = new ME.ModelErr(W.game, plan, name, scene, false, again);
    for (let i = 0; i < 200000 && !run.done; i++) { W.step(1 / 60); run.frame(); }
    if (!run.done) throw new Error('the plan did not end');
    return run.report;
  } finally { hush(false); }
}
const clone = (R: any) => JSON.parse(JSON.stringify(R));
const row = (R: any, model: string, ref?: string) => ME.spreadOf(R, ref).find((s: any) => s.model === model);
const seedOf = (scene: string) => { const v = new URLSearchParams(scene).get('seed'); return v !== null && /^\d+$/.test(v) ? +v : undefined; };

// ---- walks
const A = await walk(PLAN, planName), A2 = await walk(PLAN, planName);
{
  const light = A.samples.filter((s: any) => s.k === 'light'), smoke = A.samples.filter((s: any) => s.k === 'smoke');
  const answered = light.every((s: any) => Number.isFinite(s.m.estimate)) && smoke.every((s: any) => Number.isFinite(s.m.blobs) && Number.isFinite(s.m.grid));
  const same = JSON.stringify(A) === JSON.stringify(A2), eyes: number[] = ME.eyesOf(A);
  // (a height at a time: the lines asked there, and how many of them the grid has smoke on — a plan whose low lines all run under the smoke, or over it, measures nothing of that height)
  const at = PLAN.eyes.map((h: number) => { const L = smoke.filter((s: any) => s.a[1] === h && s.b[1] === h); return { h, n: L.length, smoked: L.filter((s: any) => s.m.grid < 0.999).length }; });
  const heights = eyes.length === PLAN.eyes.length && at.every((e: any) => e.n >= ME.LEAST && e.smoked >= ME.LEAST / 2) && at.reduce((n: number, e: any) => n + e.n, 0) === smoke.length;
  // (…and no line twice: of one station's lines at one age, no two are one pair of ends, either way round)
  const ends = (s: any) => { const a = s.a.join(','), b = s.b.join(','); return `${s.s}:${s.t}:${a < b ? a + '|' + b : b + '|' + a}`; }, twice = smoke.length - new Set(smoke.map(ends)).size;
  verdict(A.stations.length === PLAN.stations && light.length >= ME.LEAST && heights && twice === 0 && answered && same, 'walks: the plan stands at every station and asks its lines at every height, every model answers, and a second walk is the first to the bit',
    `${A.stations.length} of ${PLAN.stations} stations · ${light.length} light points, ${smoke.length} sight lines: ${at.map((e: any) => `${e.n} at ${e.h} m (${e.smoked} with smoke on them by the grid)`).join(', ')} · ${twice} asked twice · ${answered ? 'every model answered' : 'A MODEL DID NOT ANSWER'} · a second walk ${same ? 'the same' : 'DIFFERS'} · world ${A.world} · the stand ${process.env.TPS_PROFILE ?? 'frozen'}`);
}
// ---- spread
for (const l of ME.reportText(A)) log(`    ${l}`);
if (arg('--write')) { writeFileSync(arg('--write')!, JSON.stringify(A)); log(`    written: ${arg('--write')}`); }

// ---- planted
{
  const P = clone(A); P.picture = true; for (const s of P.samples) s.pic = s.k === 'light' ? s.m.estimate : s.m.grid;
  const e0 = row(P, 'estimate'), g0 = row(P, 'grid'), b0 = row(P, 'blobs'), bg = row(A, 'blobs', 'grid');
  const exact = e0.n > 0 && g0.n > 0 && e0.max === 0 && g0.max === 0 && e0.flips === 0 && g0.flips === 0 && b0.n === bg.n && b0.p95 === bg.p95 && b0.flips === bg.flips;
  const Q = clone(A); Q.picture = true; for (const s of Q.samples) s.pic = s.k === 'light' ? s.m.estimate / 4 : Math.max(0, s.m.grid - 0.25);
  const e1 = row(Q, 'estimate'), g1 = row(Q, 'grid');
  const stops = Math.abs(e1.stops95 - 2) < 1e-9 && e1.bias > 0, quarter = Math.abs(g1.max - 0.25) < 1e-12 && g1.bias > 0 && g1.bias <= 0.25;
  verdict(exact && stops && quarter, 'planted: a planted picture is read back as it was planted',
    `the walk as its own picture: estimate off by ${e0.max}, grid by ${g0.max}, blobs as against the grid (${b0.p95.toFixed(4)} at the 95th on ${b0.n}) · two stops darker planted: ${e1.stops95.toFixed(6)} stops, the model leaning ${e1.bias.toFixed(3)} brighter on the meter · a quarter of the light taken: ${g1.max.toFixed(6)} at worst, leaning ${g1.bias.toFixed(3)}`);
}
// ---- budget (and the red path)
{
  const sigma = S.SENSE.smoke.sigma; if (BREAK) S.SENSE.smoke.sigma = sigma * 3;
  let B: any; try { B = await walk(PLAN, planName); } finally { S.SENSE.smoke.sigma = sigma; }
  const aligned = B.samples.length === A.samples.length && B.world === A.world;
  B.picture = true; if (aligned) for (let i = 0; i < B.samples.length; i++) { const s = A.samples[i]; B.samples[i].pic = s.k === 'light' ? s.m.estimate : s.m.grid; }
  const row1 = { p50: 0.01, p95: 0.01, flips: 0.01 }, parts = ME.partsOf(B), tight: Record<string, any> = {};   // (a walk says where its samples stand, and is judged a part at a time: a tight row for each part of the two models)
  for (const m of ['estimate', 'grid']) if (parts.length) for (const p of parts) tight[`${m}:${p.key}`] = row1; else tight[m] = row1;
  const v = ME.judge(B, tight), each = ['estimate', 'grid'].every(m => v.lines.some((l: string) => l.includes(`${m} (`) && /budget .*: inside$/.test(l)));   // (each of the two judged in a part at least: a part with too few samples of a kind judges nothing)
  verdict(aligned && v.ok && each, 'budget: a walk judged against the first walk as its picture is inside a tight budget',
    `${v.lines.filter((l: string) => /budget/.test(l)).join('\n    ')}${BREAK ? '\n    THE SMOKE WAS THREE TIMES AS THICK ON THIS WALK' : ''}`);
}
// ---- waits
{
  // (places for a third of an age's lines a frame over the eight the walk leaves the game: an age goes to the picture over three frames, as the full plan's does on a page)
  const LAG = 45, Q = ME.PLANS.quick, ROOM = 8 + Math.ceil(Q.lines * Q.eyes.length / 3); let R: any = null, lightLate = 0, lineLate = 0;
  const asked = new Map<string, { at: number; line: boolean; live: boolean }>();
  hush(true);
  try {
    const W = await standUp({ uploadEvery: 1000, seed: 1 }); for (let i = 0; i < 3; i++) W.step(1 / 60);
    const g = W.game, smoke = g.smoke, probe = g.probe; let live = false, frame = 0;
    const over = (base: any, o: any) => Object.create(base, Object.getOwnPropertyDescriptors(o));   // (the game's own thing with these members laid over it: whatever else the game asks of it is answered as ever)
    // (the stand-in picture: the walk's own canister is in the room from its emit to the walk's clearAll; an answer is of the room as it was asked AND as it is when the answer comes)
    g.smoke = over(smoke, { emit(e: any) { if (e.kind === 'canister') live = true; smoke.emit(e); }, clearAll() { live = false; smoke.clearAll?.(); } });
    const mine = (k: string) => k.startsWith('modelerr:');
    const ask = (k: string, line: boolean) => { if (!asked.has(k)) asked.set(k, { at: frame, line, live }); };
    g.probe = over(probe, { room: ROOM,
      query(k: string, ...a: any[]) { if (mine(k)) ask(k, false); else probe.query(k, ...a); },
      querySegment(k: string, ...a: any[]) { if (mine(k)) ask(k, true); else probe.querySegment(k, ...a); },
      get(k: string) { if (!mine(k)) return probe.get(k); const q = asked.get(k); if (!q || frame - q.at < LAG) return undefined;
        return [q.line ? (q.live && live ? 0.25 : 1) : (q.live || live ? 9 : 0.5)]; } });
    const run = new ME.ModelErr(g, Q, 'quick', 'seed=1', true);
    if (BREAK_MOVE) Object.defineProperty(run, 'changes', { get: () => false });
    let stood = -2;
    for (let i = 0; i < 200000 && !run.done; i++) { W.step(1 / 60); frame++; run.frame();
      if (BREAK_TURNS && run.station !== stood) { stood = run.station; run.lines.sort((a: any, b: any) => b[0][1] - a[0][1]); } }   // (the red path: a station's lines a height at a time, highest first, as soon as it is stood at)
    R = run.done ? run.report : null;
  } finally { hush(false); }
  const P = await walk(ME.PLANS.quick, 'quick');
  if (R) for (const s of R.samples) { if (s.k === 'light' && s.pic !== 0.5) lightLate++; if (s.k === 'smoke' && s.pic !== 0.25) lineLate++; }
  const lights = R ? R.samples.filter((s: any) => s.k === 'light').length : 0, lines = R ? R.samples.length - lights : 0;
  // NO HEIGHT LATER THAN ANOTHER: of one station's lines at one age, the first line of each height, the second of each, and so on are one TURN, and
  // the lines of a turn go to the picture within a frame of each other (a walk that queued a height at a time has its lowest height's first line two
  // frames behind its highest's here, and that height's picture that much older). `behind` = the widest turn, in frames
  let behind = Infinity; const turns: string[] = [];
  if (R) { const turn = new Map<string, number[]>(), nth = new Map<string, number>(), by = new Map<number, { n: number; sum: number }>(), first = new Map<string, number>();
    R.samples.forEach((s: any, i: number) => { if (s.k !== 'smoke') return; const at = asked.get(`modelerr:${i}`)?.at ?? Infinity, g = `${s.s}:${s.t}`, h = s.a[1];
      const k = `${g}:${h}`, n = nth.get(k) ?? 0; nth.set(k, n + 1); const t = `${g}#${n}`; turn.set(t, [...(turn.get(t) ?? []), at]);
      if (!first.has(g)) first.set(g, at); const e = by.get(h) ?? { n: 0, sum: 0 }; e.n++; e.sum += at - first.get(g)!; by.set(h, e); });
    behind = Math.max(0, ...[...turn.values()].map(f => Math.max(...f) - Math.min(...f)));
    for (const [h, e] of [...by].sort((a, b) => b[0] - a[0])) turns.push(`${(e.sum / e.n).toFixed(2)} at ${h} m`); }
  const sameModels = !!R && R.samples.length === P.samples.length && R.samples.every((s: any, i: number) => JSON.stringify(s.m) === JSON.stringify(P.samples[i].m)) && R.world === P.world;
  verdict(!!R && lights >= ME.LEAST && lines >= ME.LEAST && lightLate === 0 && lineLate === 0 && behind <= 1 && R.unanswered === 0 && sameModels && R.v === 2, 'waits: the walk changes the picture only when the picture has answered what was asked of it as it stood, and puts its lines to it a line of each height by turns',
    R ? `a picture ${LAG} frames late with ${ROOM - 8} places a frame · ${lights} light points, ${lightLate} read with smoke out · ${lines} sight lines at ${Q.ages.join(' and ')} s, ${lineLate} read from a room with no smoke left in it · the widest turn ${behind} frame${behind === 1 ? '' : 's'} (frames behind the first line of its age, a height on average: ${turns.join(', ')}) · ${R.unanswered} never answered · the models' answers ${sameModels ? 'those of a walk with no picture, to the bit' : 'NOT those of a walk with no picture'}${BREAK_MOVE ? '\n    THE WALK DID NOT WAIT' : ''}${BREAK_TURNS ? '\n    THE WALK QUEUED ITS LINES A HEIGHT AT A TIME' : ''}` : 'the plan did not end');
}
// ---- reports
let again = 0, pictures = 0;
/** the words of a page's address that leave the world the plain page's, the one this tool stands up (a mission's or a level's own word is another world) */
const PLAIN = new Set(['seed', 'solo', 'paused', 'mute', 'quality']);
{
  const dir = `${ROOT}/tools/qa/fixtures/modelerr`, files = existsSync(dir) ? readdirSync(dir).filter(f => f.endsWith('.json')).sort().map(f => `${dir}/${f}`) : [];
  if (arg('--report') && !files.includes(resolve(arg('--report')!))) files.push(resolve(arg('--report')!));
  if (!files.length) log('- reports: none is kept yet (tools/qa/fixtures/modelerr/), and none was named: no picture has been judged');
  for (const f of files) {
    let R: any; try { R = JSON.parse(readFileSync(f, 'utf8')); } catch (e) { verdict(false, `report ${f}`, `not read: ${e}`); continue; }
    const name = f.startsWith(ROOT) ? f.slice(ROOT.length + 1) : f, was = ME.judge(R);
    log(`- report ${name}: ${R.scene || 'the plain page'} · plan ${R.plan} · world ${R.world} · ${R.picture ? `${R.samples.filter((s: any) => s.pic !== null).length} of ${R.samples.length} answered by the picture` : 'NO PICTURE IN IT'}${R.where ? ` · ${Object.entries(R.where).map(([k, v]) => `${k} ${v}`).join(', ')}` : ''}`);
    for (const l of was.lines) log(`    as recorded: ${l}`);
    let now: any = null; if (R.picture) pictures++;
    // WHY IT IS NOT ASKED AGAIN HERE, if it is not (null: it is). The level's word is the stand's, whatever the seed (the first walk's report has it)
    const plan = ME.planAgain(R), seed = seedOf(R.scene), odd = [...new URLSearchParams(R.scene).keys()].filter(k => !PLAIN.has(k));
    const why = !R.picture ? null : !plan ? 'it does not say its plan\'s row and its level (a page from before the reports did)'
      : R.level !== A.level ? `the level here (${A.level}) is not the one it was made on (${R.level}): a page's report is the page's stand's (TPS_PROFILE=shipped), and an edited office is another floor`
      : seed === undefined ? 'its scene names no seed: who carries a lamp where is dealt by the game\'s die, and this stand\'s would be another night'
      : odd.length ? `its scene is not the plain page's (${odd.join(', ')}): this tool stands up no other` : null;
    if (why) log(`    NOT ASKED AGAIN HERE — ${why}`);
    else if (R.picture) { const N = await walk(plan, R.plan, R.scene, seed, R);
      const same = N.samples.length === R.samples.length && N.samples.every((s: any, i: number) => s.k === R.samples[i].k && s.s === R.samples[i].s && JSON.stringify(s.p ?? [s.a, s.b, s.t]) === JSON.stringify(R.samples[i].p ?? [R.samples[i].a, R.samples[i].b, R.samples[i].t]));
      if (!same) { verdict(false, `report ${name}: asked again`, `the walk did not ask the report's questions in the report's order (${N.samples.length} of ${R.samples.length} samples)`); continue; }
      again++; N.picture = true; N.v = R.v; const moved: Record<string, number> = {};   // (the picture's answers are the report's: read as that report was made — src/host/modelerr.ts Report.v)
      // (…and judged BY THE REPORT'S OWN PARTS: where it says its samples stand, there; where it does not, whole, by the plain rows)
      for (let i = 0; i < N.samples.length; i++) { const o = R.samples[i].o; if (o === undefined) delete N.samples[i].o; else N.samples[i].o = o; }
      for (let i = 0; i < N.samples.length; i++) { const a = N.samples[i], b = R.samples[i]; a.pic = b.pic;
        for (const k of Object.keys(a.m)) if (typeof a.m[k] === 'number' && typeof b.m?.[k] === 'number') moved[k] = Math.max(moved[k] ?? 0, Math.abs(a.m[k] - b.m[k]) / Math.max(Math.abs(b.m[k]), 1e-9)); }
      log(`    its questions asked again here · each model's answers against the ones in the report, at most: ${Object.entries(moved).map(([k, v]) => `${k} ${v === 0 ? 'the same' : v.toExponential(1)}`).join(', ')}`);
      now = ME.judge(N); for (const l of now.lines) log(`    as the models are now: ${l}`); }
    const v = now ?? was;
    verdict(v.ok, `report ${name}: ${!R.picture ? 'no picture, nothing judged' : !v.judged ? 'no model has a budget row yet, nothing judged' : now ? 'the models as they are now, against the recorded picture' : 'AS RECORDED ONLY (its picture holds nothing of the models here)'}`, v.judged ? `${v.judged} models judged` : 'nothing judged');
  }
}
const stale = pictures - again, held = pictures ? ` · ${again} of ${pictures} report${pictures > 1 ? 's' : ''} with a picture asked again here${stale ? ` (${stale} HOLD${stale > 1 ? '' : 'S'} NOTHING HERE: see above)` : ''}` : '';
log(bad ? `\n=== modelerr: ${bad} FAILED${BREAK ? ' (the red path: `budget` must be one of them)' : BREAK_MOVE || BREAK_TURNS ? ' (the red path: `waits` must be one of them)' : ''}${held} ===` : `\n=== modelerr: ALL PASS${held} ===`);
process.exit(bad ? 1 : 0);
