// WHO HAS THIS MAN (src/game/owner.ts): is the record one fact, written one way, and is it true?
//   bun run tools/qa/owner-check.ts                        every line below on the frozen stand; exit 0 = all pass, 1 = one failed (named)
//   TPS_PROFILE=shipped bun run tools/qa/owner-check.ts    the same on the shipped stand
//   OWNER_SCN=block-squads,depot-four OWNER_SEEDS=1 …      fewer runs · OWNER_JOBS=4 how many at once
//   OWNER_BREAK=1 bun run tools/qa/owner-check.ts …        the red path: the record's own rules read wrong, every claim dropped in the runs, a lint line planted: every line must FAIL
// WHAT IS HELD:
//   table      the ranks are one table: no rule twice, a bit each, stronger is higher
//   record     a claim, a stronger one over it, the release that lets the lower one surface: owner, rank, reason and since say so (`since`: when the claim now on top came to the top)
//   one way    no file of src writes the four columns but owner.ts
//   no asking  none of the ownership questions is exported any more (each that has become a read of the record)
//   runs       in the missions at both their switches, at every read of the record and at the end of every tick: each man's word against each rule's own state (`ownerWitness`)
// WHAT THE 42 DO NOT SHOW: a digest says nothing about a rule that never has anybody in the runs. On 2026-11-05, never true in the 42 even with last stand, buddy, pressure and dig-in up:
// rescueCheckOwns, powerOutOwns, needsBusy, gunHoldKeeps, gunHoldRuns, silentMan, unseenMan, digInStays. True only with those switches up: lastStandMan, pressOwns, buddyOwns. So this
// check runs an arm with the switches up, and a rule the runs still never exercise says so in its `runs` line (`never held`).
// Not game code; nothing here has been seen on a GPU.
import { mkdtempSync, readFileSync, readdirSync, rmSync, statSync } from 'node:fs';
import { tmpdir } from 'node:os';
import { join, resolve } from 'node:path';

const ROOT = resolve(import.meta.dir, '../..');
const BREAK = process.env.OWNER_BREAK === '1';
const O: any = await import(`${ROOT}/src/game/owner.ts`);
let fails = 0;
const verdict = (ok: boolean, label: string, detail: string) => { if (!ok) fails++; console.log(`- ${ok ? 'PASS' : 'FAIL'} — ${label}\n    ${detail}`); };
console.log(`who has this man · profile ${process.env.TPS_PROFILE ?? 'frozen'}${BREAK ? ' · OWNER_BREAK=1 (the red path)' : ''}`);

/** the ownership questions (owner-questions.md) that have become reads of the record */
const GONE = ['runnerOwns'];
/** the switches the shipped game leaves down, up: what the runs cannot show otherwise */
const ALL_ON = { LASTSTAND: { regroup: true }, BUDDY: { on: true }, PRESSURE: { on: true }, DIGIN: { on: true } };

{ const names: string[] = O.ownerRanks(), bits = names.map(n => O.OWN[n] as number);
  const down = bits.every((b, i) => i === 0 || b < bits[i - 1]), one = bits.every(b => b > 0 && (b & (b - 1)) === 0);
  verdict(new Set(names).size === names.length && one && down && !BREAK, 'table: no rule twice, a bit each, stronger is higher', `${names.length} rules, strongest first: ${names.join(', ')}`); }

{ const man: any = { owner: 0, rank: 0, reason: 0, since: -1 };
  const hi = 32, lo = BREAK ? hi : 4, seen: string[] = [];
  const say = (what: string, ok: boolean) => { if (!ok) seen.push(what); };
  O.claimMan(man, lo, 1); say('a claim takes him', man.owner === lo && man.rank === 32 - Math.clz32(lo) && man.since === 1);
  O.claimMan(man, hi, 2, 1); say('a stronger one goes on top with its reason', man.owner === (hi | lo) && man.rank === 32 - Math.clz32(hi) && man.reason === 1 && man.since === 2);
  O.claimMan(man, lo, 3); say('a claim under it moves nothing', man.rank === 32 - Math.clz32(hi) && man.since === 2 && man.reason === 1);
  O.claimMan(man, hi, 4); say('the same rule again keeps its since, and says its reason', man.since === 2 && man.reason === 0);
  O.releaseMan(man, lo, 5); say('the lower one going moves nothing on top', man.rank === 32 - Math.clz32(hi) && man.since === 2);
  O.claimMan(man, lo, 6); O.releaseMan(man, hi, 7); say('the top going lets the lower one surface, since then', man.owner === lo && man.rank === 32 - Math.clz32(lo) && man.since === 7 && man.reason === 0);
  O.releaseMan(man, hi, 8); say('a release of what he does not hold moves nothing', man.owner === lo && man.since === 7);
  O.releaseMan(man, lo, 9); say('nobody: rank 0', man.owner === 0 && man.rank === 0);
  verdict(seen.length === 0 && !BREAK, 'record: claim, release and who surfaces, with its since', seen.length ? `wrong: ${seen.join(' · ')}` : 'owner, rank, reason and since as said'); }

const walk = (d: string): string[] => readdirSync(d).flatMap(n => { const p = join(d, n); return statSync(p).isDirectory() ? walk(p) : p.endsWith('.ts') ? [p] : []; });
{ const WRITE = /\b(?:gd|h|m|o|x|D\.gd|man|T|A|B)\.(owner|rank|reason|since)\s*(?:[|&^+-]?=)(?!=)/, bad: string[] = [];
  for (const p of walk(join(ROOT, 'src/game'))) { if (p.endsWith('/owner.ts')) continue; readFileSync(p, 'utf8').split('\n').forEach((l, i) => { if (WRITE.test(l.replace(/\/\/.*$/, ''))) bad.push(`${p.slice(ROOT.length + 1)}:${i + 1}`); }); }
  if (BREAK) bad.push('(a line planted)');
  verdict(bad.length === 0, 'one way: no file of src/game writes the four columns but owner.ts', bad.length ? bad.join(' · ') : 'none does'); }

{ const left: string[] = [];
  for (const p of walk(join(ROOT, 'src/game'))) { const t = readFileSync(p, 'utf8'); for (const q of GONE) if (new RegExp(`export (function|const) ${q}\\b`).test(t)) left.push(`${q} in ${p.slice(ROOT.length + 1)}`); }
  if (BREAK) left.push('(a line planted)');
  verdict(left.length === 0, 'no asking: the ownership questions that have become reads are no exports', left.length ? left.join(' · ') : `${GONE.length} gone: ${GONE.join(', ')}`); }

/** every mission at one set of switches */
async function arm(label: string, set: object): Promise<void> {
  const { SCENARIOS } = await import(`${ROOT}/src/scene/scenarioList.ts`);
  const scns: string[] = process.env.OWNER_SCN ? process.env.OWNER_SCN.split(',') : SCENARIOS.map((e: { id: string }) => e.id);
  const seeds = (process.env.OWNER_SEEDS ?? '1,2').split(',').map(Number), jobs = Number(process.env.OWNER_JOBS ?? 4);
  const todo = scns.flatMap(scn => seeds.map(seed => ({ scn, seed }))), dir = mkdtempSync(join(tmpdir(), 'owner-'));
  const reads: Record<string, [number, number]> = {}, held: Record<string, number> = {}, first: string[] = [], lost: string[] = []; let next = 0, bad = 0, ticks = 0;
  await Promise.all(Array.from({ length: jobs }, async () => { for (;;) {
    const j = todo[next++]; if (!j) return;
    const file = join(dir, `${j.scn}-${j.seed}`);
    const pr = Bun.spawn(['bun', '--preload', resolve(import.meta.dir, 'owner-audit.ts'), resolve(import.meta.dir, 'playthrough.ts'), '--one', `${j.scn}:${j.seed}`, '--out', `${file}.run`, '--set', JSON.stringify(set)],
      { cwd: ROOT, env: { ...process.env, OWNER_OUT: `${file}.json`, OWNER_DROP: BREAK ? String(~0 >>> 0) : '0' }, stdout: 'ignore', stderr: 'pipe' });
    const err = await new Response(pr.stderr).text(); await pr.exited;
    try { const o = JSON.parse(readFileSync(`${file}.json`, 'utf8'));
      ticks += o.ticks; bad += o.bad; for (const f of o.first) if (first.length < 12) first.push(`${j.scn}:${j.seed} ${f}`);
      for (const [k, v] of Object.entries(o.reads) as [string, [number, number]][]) { const R = reads[k] ??= [0, 0]; R[0] += v[0]; R[1] += v[1]; }
      for (const [k, v] of Object.entries(o.held) as [string, number][]) held[k] = (held[k] ?? 0) + v; }
    catch { lost.push(`${j.scn}:${j.seed} — ${err.trim().split('\n').slice(-2).join(' ').slice(0, 200)}`); }
  } }));
  rmSync(dir, { recursive: true, force: true });
  const rules: string[] = O.ownerWitnessed(), never = rules.filter(r => !held[r]);
  const row = (r: string) => `${r}: held ${held[r] ?? 0} man-ticks, read ${reads[r]?.[0] ?? 0} times, ${reads[r]?.[1] ?? 0} of them true`;
  verdict(bad === 0 && !lost.length && ticks > 0, `runs, ${label}: at every read and every tick's end the record says what each rule's own state says`,
    `${todo.length} runs, ${ticks} ticks · ${bad} disagreements${first.length ? `\n    ${first.join('\n    ')}` : ''}${lost.length ? `\n    no count: ${lost.join('\n    ')}` : ''}\n    ${rules.map(row).join('\n    ')}${never.length ? `\n    never held in these runs: ${never.join(', ')}` : ''}`);
}
await arm('as the stand has them', {});
if (!process.env.OWNER_SCN || process.env.OWNER_ALLON) await arm('with last stand, buddy, pressure and dig-in up', ALL_ON);

console.log(fails ? `owner-check: ${fails} line${fails === 1 ? '' : 's'} FAILED${BREAK ? ' (OWNER_BREAK=1: as it must)' : ''}` : 'owner-check: all pass');
process.exit(fails ? 1 : 0);
