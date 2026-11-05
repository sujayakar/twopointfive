// DO TWO RULES FIGHT OVER A MAN? The counter (tools/qa/fights-preload.ts) runs the missions and names every pair of modules that set one deciding field of one man to different
// values within one tick. The pairs known today are listed below with their reason; a pair nobody has listed is red: a new behaviour has met an old one over a man, and someone
// says which of them has him (src/game/owner.ts) or why it is meant before it goes on the list.
//   bun run tools/qa/fights-check.ts                        the 42 runs on the frozen stand; exit 0 = no new pair, 1 = a new pair (named), 2 = a run left no count
//   TPS_PROFILE=shipped bun run tools/qa/fights-check.ts    the same on the shipped stand
//   FIGHTS_SCN=depot-four,block-squads FIGHTS_SEEDS=1 …     fewer runs (a pair's absence then means nothing; a NEW pair still does)
//   FIGHTS_JOBS=4                                           how many at once
//   FIGHTS_BREAK=1 bun run tools/qa/fights-check.ts …       the red path: the list without its pairs of the guards and the room fight, so the same runs must name them: must FAIL
// What it does not see (the review's own list, 2026-11-05): aimYaw, bodyYaw and the animator's fields, doors, the player, solo stealth missions, god mode; a scripted player. A pair
// is two modules in the tick, not who won it. Not game code; nothing here has been seen on a GPU.
import { mkdtempSync, readFileSync, rmSync } from 'node:fs';
import { tmpdir } from 'node:os';
import { join, resolve } from 'node:path';

const ROOT = resolve(import.meta.dir, '../..');
const BREAK = process.env.FIGHTS_BREAK === '1';

/** the pairs known today: why each is no fight of two rules over who has him, or whose it is to mend */
const THE_FIGHT = 'THE ONE REAL FIGHT, the belief part\'s until it lands: the room fight writes the door he watches into what he believes, his eyes or ears write the enemy over it';
const ORDER_THEN_PATH = 'call order, meant: an order drops his way and the squad or the door makes a new one in the same tick, no second opinion';
const ORDER_THEN_POSE = 'call order, meant: the order sets the pose and the cover he has been dealt or the door he is going through refines it';
const SENSES_THEN_FIGHT = 'call order, meant: his senses write the number and a rule of the fight raises it';
const HANDOVER = 'a hand-over: the rule letting go writes it last, once';
export const KNOWN_FIGHTS: Record<string, string> = {
  'pathGoal: orders + squad': ORDER_THEN_PATH,
  'pathGoal: doorpath + orders': ORDER_THEN_PATH,
  'pathGoal: pressure + squad': ORDER_THEN_PATH,
  'pathGoal: lanes + squad': ORDER_THEN_PATH,
  'pathGoal: frag + squad': ORDER_THEN_PATH,
  'pathGoal: roomfight + squad': ORDER_THEN_PATH,
  'pathGoal: combat + squad': ORDER_THEN_PATH,
  'pathGoal: netquiet + squad': ORDER_THEN_PATH,
  'pathGoal: doorpath + frag + squad': ORDER_THEN_PATH,
  'pathGoal: doorforce + lanes': 'worth a look (the review\'s rescue-consulate seed 2, 21 alternations): a door forced under a man the lanes hold',
  'pathGoal: guards + tactics': ORDER_THEN_PATH,
  'pathGoal: guards + lanes': ORDER_THEN_PATH,
  'pathGoal: lanes + tactics': ORDER_THEN_PATH,
  'pathGoal: guards + pressure': ORDER_THEN_PATH,
  'pathGoal: guards + rescueflush': ORDER_THEN_PATH,
  'script.face: doorpath + orders': ORDER_THEN_POSE,
  'script.speed: doorpath + orders': ORDER_THEN_POSE,
  'script.aimAt: cover + orders': ORDER_THEN_POSE,
  'script.stance: cover + orders': ORDER_THEN_POSE,
  'script.upper: cover + orders': ORDER_THEN_POSE,
  'lastKnown: guards + roomfight': THE_FIGHT,
  'lastKnown: combat + guards + roomfight': THE_FIGHT,
  'lastKnown: rescue + roomfight': THE_FIGHT,
  'lastKnown: guards + rescue + roomfight': THE_FIGHT,
  'lastKnown: guards + pressure': THE_FIGHT,
  'lastKnown: guards + rescue': THE_FIGHT,
  'reactT: guards + roomfight': SENSES_THEN_FIGHT,
  'reactT: combat + guards': SENSES_THEN_FIGHT,
  'reactT: guards + rescue': SENSES_THEN_FIGHT,
  'reactT: guards + rescue + roomfight': SENSES_THEN_FIGHT,
  'reactT: digin + guards': SENSES_THEN_FIGHT,
  'reactT: guards + morale': SENSES_THEN_FIGHT,
  'reactT: combat + morale': SENSES_THEN_FIGHT,
  'reactT: morale + roomfight': SENSES_THEN_FIGHT,
  'reactT: rescue + roomfight': SENSES_THEN_FIGHT,
  'reactT: guards + tactics': SENSES_THEN_FIGHT,
  'awareness: combat + guards': SENSES_THEN_FIGHT,
  'awareness: guards + qrf': SENSES_THEN_FIGHT,
  'awareness: guards + roomfight': SENSES_THEN_FIGHT,
  'pinned: pressure + roomfight': HANDOVER,
  'pinned: netquiet + roomfight': HANDOVER,
  'hold: guards + qrf': HANDOVER,
};

if (import.meta.main) {
  const known = BREAK ? Object.fromEntries(Object.entries(KNOWN_FIGHTS).filter(([k]) => !k.includes('guards + roomfight'))) : KNOWN_FIGHTS;
  const { SCENARIOS } = await import(`${ROOT}/src/scene/scenarioList.ts`);
  const scns: string[] = process.env.FIGHTS_SCN ? process.env.FIGHTS_SCN.split(',') : SCENARIOS.map((e: { id: string }) => e.id);
  const seeds = (process.env.FIGHTS_SEEDS ?? '1,2').split(',').map(Number), jobs = Number(process.env.FIGHTS_JOBS ?? 4);
  const todo = scns.flatMap(scn => seeds.map(seed => ({ scn, seed }))), dir = mkdtempSync(join(tmpdir(), 'fights-'));
  const pairs: Record<string, { n: number; runs: number }> = {}, lost: string[] = []; let next = 0, ticks = 0;
  await Promise.all(Array.from({ length: jobs }, async () => { for (;;) {
    const j = todo[next++]; if (!j) return;
    const file = join(dir, `${j.scn}-${j.seed}`);
    const pr = Bun.spawn(['bun', '--preload', resolve(import.meta.dir, 'fights-preload.ts'), resolve(import.meta.dir, 'playthrough.ts'), '--one', `${j.scn}:${j.seed}`, '--out', `${file}.run`], { cwd: ROOT, env: { ...process.env, FIGHTS_OUT: `${file}.json` }, stdout: 'ignore', stderr: 'pipe' });
    const err = await new Response(pr.stderr).text(); await pr.exited;
    try { const o = JSON.parse(readFileSync(`${file}.json`, 'utf8')) as { ticks: number; pairs: Record<string, number> }; ticks += o.ticks; for (const [k, v] of Object.entries(o.pairs)) { const P = pairs[k] ??= { n: 0, runs: 0 }; P.n += v; P.runs++; } }
    catch { lost.push(`${j.scn}:${j.seed} — ${err.trim().split('\n').slice(-2).join(' ').slice(0, 200)}`); }
  } }));
  rmSync(dir, { recursive: true, force: true });
  const fresh = Object.keys(pairs).filter(k => !known[k]).sort((a, b) => pairs[b].n - pairs[a].n), quiet = Object.keys(known).filter(k => !pairs[k]);
  console.log(`who fights over a man · profile ${process.env.TPS_PROFILE ?? 'frozen'}${BREAK ? ' · FIGHTS_BREAK=1 (the red path)' : ''} · ${todo.length} runs, ${ticks} ticks with a write`);
  for (const k of Object.keys(pairs).filter(x => known[x]).sort((a, b) => pairs[b].n - pairs[a].n)) console.log(`  known ${String(pairs[k].n).padStart(6)} ticks in ${String(pairs[k].runs).padStart(2)} runs  ${k}`);
  if (quiet.length) console.log(`  listed, not met in these runs: ${quiet.join(' · ')}`);
  if (lost.length) { console.log(`- FAIL — ${lost.length} runs left no count\n    ${lost.join('\n    ')}`); process.exit(2); }
  if (fresh.length) { console.log(`- FAIL — ${fresh.length} pair${fresh.length === 1 ? '' : 's'} nobody has listed: say who has the man (src/game/owner.ts) or why it is meant, then list it here`); for (const k of fresh) console.log(`    ${pairs[k].n} ticks in ${pairs[k].runs} runs  ${k}`); process.exit(1); }
  console.log(`- PASS — no pair of modules fights over a man that is not on the list (${Object.keys(pairs).length} met, of ${Object.keys(known).length} listed)`);
}
