#!/usr/bin/env bun
// THE LINE BETWEEN GAMEPLAY AND DRESSING, HELD (src/game/dress.ts; docs/agents/rust.md):
//   bun run tools/qa/dress-check.ts                       the comparison missions with every dressing row down against every row up: the same game after every update
//   bun run tools/qa/dress-check.ts --runs pass-four:1    other runs (tools/qa/playthrough.ts's scenarios)
//   bun run tools/qa/dress-check.ts --each                … and each row down alone (which row it is, when the first is red)
//   bun run tools/qa/dress-check.ts --rows air,moths      only these rows down
//   DRESS_BREAK=1 bun run tools/qa/dress-check.ts         THE RED PATH: a gameplay step (the doors) is put down with the rows, and the check must fail
// Exit 0 = every set is the same game · 1 = a run differs, or left nothing. Under DRESS_BREAK=1 it is the other way about, as every <NAME>_BREAK is: 1 = the
// red path is red (every pair played and parted), 0 = it is NOT (the check is blind, or a run left nothing).
// The stand is the server's unless the caller names another: shipped, the senses on the CPU (TPS_PROFILE / TPS_CPUSENSE in front, as rust-check.ts's missions).
import { spawn } from 'node:child_process';
import { resolve } from 'node:path';
const ROOT = resolve(import.meta.dir ?? '.', '../..'), args = process.argv.slice(2), BREAK = process.env.DRESS_BREAK === '1';
const flag = (n: string) => { const i = args.indexOf(n); return i >= 0 ? args[i + 1] : undefined; };
const RUNS = ['compound-four:1', 'warehouse-four:1', 'pass-four:1', 'hostage-rescue:1', 'block-squads:1'];

if (args[0] === '--run') {
  process.env.TPS_PROFILE ??= 'shipped'; process.env.TPS_CPUSENSE ??= '1'; process.env.PT_DIGEST ??= '1';
  const [scn, seed] = args[1].split(':'), down = (args[2] ?? '').split(',').filter(Boolean);
  const { Game } = await import(`${ROOT}/src/game/game.ts`);
  const PT: any = await import(`${ROOT}/tools/qa/playthrough.ts`);
  const { rawBits, rawBitsParts, RAW_BITS_SEED } = await import(`${ROOT}/src/game/digest.ts`);
  const D: any = await import(`${ROOT}/src/game/dress.ts`);
  for (const k of down) { if (!(k in D.DRESS)) { console.error(`no dressing row '${k}'`); process.exit(2); } D.DRESS[k] = false; }
  let h: number = RAW_BITS_SEED, ticks = 0, held = 0; const ALL = Object.keys(D.DRESS), TRACE = process.env.DRESS_TRACE, trace: Record<string, number>[] = [];
  const update = Game.prototype.update;
  Game.prototype.update = function (this: any, ...a: unknown[]) {
    if (BREAK && down.length) this.doors.update = () => {};   // (the red path: a step that IS gameplay goes down with the rows)
    if (ALL.every(k => D.DRESS[k] === !down.includes(k))) held++;   // (the ticks the table stood as this run was asked to play it: the named rows down, every other up)
    const r = (update as (...x: unknown[]) => unknown).apply(this, a); h = rawBits(this as never, h); ticks++; if (TRACE) trace.push(rawBitsParts(this as never)); return r;
  } as typeof update;
  const t0 = performance.now();
  const r = await PT.play(scn, Number(seed), 300, false, {});
  if (TRACE) (await import('node:fs')).writeFileSync(TRACE, JSON.stringify(trace));
  console.log('DRESS-RUN ' + JSON.stringify({ run: args[1], down: down.length, ticks, held, profile: process.env.TPS_PROFILE, cpu: process.env.TPS_CPUSENSE, end: String(r.end), digest: String(r.digest), bits: h.toString(36), secs: Math.round((performance.now() - t0) / 100) / 10 }));
  process.exit(0);
}

interface Run { run: string; down: number; ticks: number; held: number; profile: string; cpu: string; end: string; digest: string; bits: string; secs: number }
const one = (run: string, down: string[]): Promise<Run | string> => new Promise(done => {
  const env = { ...process.env, TPS_PROFILE: process.env.TPS_PROFILE ?? 'shipped', TPS_CPUSENSE: process.env.TPS_CPUSENSE ?? '1', PT_DIGEST: '1' } as Record<string, string>;
  const p = spawn('bun', ['run', 'tools/qa/dress-check.ts', '--run', run, down.join(',')], { cwd: ROOT, env }); let out = '', err = '';
  p.stdout.on('data', (d: unknown) => { out += d; }); p.stderr.on('data', (d: unknown) => { err += d; });
  p.on('close', code => { const l = out.split('\n').find(x => x.startsWith('DRESS-RUN ')); done(l ? JSON.parse(l.slice(10)) as Run : `left nothing (exit ${code}): ${err.trim().split('\n').slice(-3).join(' / ').slice(0, 300)}`); });
  p.on('error', e => done(String(e)));
});
/** a few at a time: a run is one core for half a minute */
async function pool<T>(jobs: (() => Promise<T>)[], n: number): Promise<T[]> {
  const out: T[] = new Array(jobs.length); let i = 0;
  await Promise.all(Array.from({ length: Math.min(n, jobs.length) }, async () => { while (i < jobs.length) { const k = i++; out[k] = await jobs[k](); } }));
  return out;
}

const D: any = await import(`${ROOT}/src/game/dress.ts`), ALL = Object.keys(D.DRESS);
const rows = (flag('--rows') ?? ALL.join(',')).split(',').filter(Boolean), runs = (flag('--runs') ?? RUNS.join(',')).split(',');
const sets: { name: string; down: string[] }[] = args.includes('--each') ? rows.map(r => ({ name: r, down: [r] })) : [{ name: rows.length === ALL.length ? 'every row' : rows.join(' '), down: rows }];
const JOBS = Number(flag('--jobs') ?? 4);
let fails = 0, parted = 0, ticks = 0; const lines: string[] = [];
const base = await pool(runs.map(run => () => one(run, [])), JOBS);
const got = await pool(sets.flatMap(s => runs.map(run => () => one(run, s.down))), JOBS);
sets.forEach((s, si) => {
  const bad: string[] = []; let broken = 0, differ = 0;
  runs.forEach((run, ri) => {
    const a = base[ri], b = got[si * runs.length + ri];
    if (typeof a === 'string' || typeof b === 'string') { broken++; bad.push(`${run}: ${typeof a === 'string' ? `up ${a}` : ''} ${typeof b === 'string' ? `down ${b}` : ''}`); return; }
    // (a fair pair: both really played, and EVERY tick of each ran with the table as asked — every row up in the one, the named rows down in the other)
    const same = a.bits === b.bits && a.ticks === b.ticks && a.end === b.end && a.digest === b.digest, fair = a.ticks > 600 && b.down === s.down.length && a.down === 0 && a.held === a.ticks && b.held === b.ticks;
    if (si === 0) ticks += a.ticks;
    if (!fair) broken++; else if (!same) differ++;
    if (!same || !fair) bad.push(`${run}: ${fair ? '' : 'NOT A FAIR PAIR · '}up ${a.ticks} ticks ${a.bits} ends ${a.end} · down ${b.ticks} ticks ${b.bits} ends ${b.end}`);
  });
  if (bad.length) fails++;
  if (!broken && differ === runs.length) parted++;   // (the red path's word: every pair of this set played fair and parted)
  lines.push(`${bad.length ? 'FAIL' : 'PASS'} — ${s.name} down: ${bad.length ? `NOT the same game on ${bad.length} of ${runs.length} runs\n      ${bad.join('\n      ')}` : `the same game after every update on ${runs.length} runs`}`);
});
console.log(lines.map(l => `- ${l}`).join('\n'));
{ const a = base.find(x => typeof x !== 'string') as Run | undefined; if (a) console.log(`  stand: ${a.profile}, the senses ${a.cpu === '1' ? 'on the CPU' : 'off the renderer\'s stand-in'}`); }
console.log(`  ${ticks} ticks a set · ${runs.map((r, i) => { const a = base[i]; return typeof a === 'string' ? `${r} —` : `${r} ${a.bits}`; }).join(' · ')}`);
if (BREAK) { const red = parted === sets.length; console.log(red ? '\nthe red path is red, as it must be: every pair played and parted' : '\nTHE RED PATH IS NOT RED: a pair did not part with a gameplay step down, or a run left nothing (above)'); process.exit(red ? 1 : 0); }
console.log(fails ? `\n${fails} FAILED` : '\nall pass');
process.exit(fails ? 1 : 0);
