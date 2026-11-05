// THE TICK LOOP (README.md): the TypeScript game and the Rust game stepped over the same ticks of a mission from one start, and the first tick whose states differ
// by something not in known.txt. Called by run.sh. This process decodes nothing: the TypeScript side, the Rust runner and each tick's diff are processes of their
// own, one at a time, and it stops at the first tick that differs.
//   exit 0 every tick the same but for what is known · 1 an unknown difference · 2 the Rust tick panicked or died · 3 the build or the TypeScript side failed
//        4 a process went over the memory ceiling · 64 bad arguments
import { mkdtempSync, mkdirSync, rmSync, existsSync, writeFileSync } from 'node:fs';
import { tmpdir } from 'node:os';
import { join, resolve } from 'node:path';
const ROOT = resolve(import.meta.dir, '../..');
const argv = process.argv.slice(2);
const flag = (n: string) => { const i = argv.indexOf(n); if (i < 0) return false; argv.splice(i, 1); return true; };
const opt = (n: string, d: string) => { const i = argv.indexOf(n); return i < 0 ? d : argv.splice(i, 2)[1]; };
const KEEP = flag('--keep'), VERBOSE = flag('--verbose'), NO_BUILD = flag('--no-build');
const TICKS = +opt('--ticks', '40'), FROM = +opt('--from', '1'), SHOW = opt('--show', '6'), OUT = opt('--out', ''), DIE = opt('--die', '');
const play = argv[0] ?? '';
const say = (s = '') => process.stdout.write(s + '\n');
if (!/^[\w.-]+:\d+$/.test(play) || argv.length !== 1 || !(TICKS >= 1) || !(FROM >= 1)) {
  say('usage: tools/blitz/run.sh <mission>:<seed> [--ticks N=40] [--from K=1] [--keep] [--verbose] [--show N=6] [--out DIR] [--no-build]');
  say('  <mission>   a scenario of tools/qa/playthrough.ts (room-clear-four, …); a wrong name lists them');
  say('  --from K    take the common start as the mission\'s tick K opens (the TypeScript game alone plays the ticks before it)');
  say('  --ticks N   step N ticks side by side; tick numbers in the output count from 1 at the start');
  say('  --keep      keep every tick\'s blobs (default: only the last tick compared, and the start)');
  process.exit(64);
}
const dir = OUT ? resolve(OUT) : mkdtempSync(join(tmpdir(), 'blitz-')); mkdirSync(dir, { recursive: true });
const env: Record<string, string> = { ...(process.env as Record<string, string>), TPS_PROFILE: process.env.TPS_PROFILE ?? 'shipped' }; delete env.BUN_OPTIONS;
const BUN = process.execPath, secs = (t: number) => `${((performance.now() - t) / 1000).toFixed(0)} s`;
const text = (b: Uint8Array | null | undefined) => new TextDecoder().decode(b ?? new Uint8Array());
say(`blitz: ${play}, ${TICKS} ticks from the mission's tick ${FROM}, stand ${env.TPS_PROFILE}; blobs in ${dir}`);

// ---- 1. the native runner
const BIN = join(ROOT, 'rust/game_run/target/debug/game_run');
if (!NO_BUILD) { const t = performance.now();
  const r = Bun.spawnSync(['cargo', 'build', '--manifest-path', 'rust/game_run/Cargo.toml'], { cwd: ROOT, env, stdout: 'pipe', stderr: 'pipe' });
  const log = text(r.stderr); writeFileSync(join(dir, 'build.log'), log);
  if (r.exitCode !== 0) { const L = log.split('\n'); const first = L.findIndex(l => /^error/.test(l)); say(`build: FAILED (${secs(t)}); the first error, and the rest in ${join(dir, 'build.log')}`); say(L.slice(Math.max(0, first), first + 40).join('\n')); process.exit(3); }
  say(`build: ok (${secs(t)})`);
}
if (!existsSync(BIN)) { say(`no runner at ${BIN}`); process.exit(3); }

// ---- 2. the TypeScript side, every tick
let ticks = TICKS;
{ const t = performance.now();
  const r = Bun.spawnSync([BUN, 'run', '--preload', './tools/blitz/expose.ts', 'tools/blitz/ts-ticks.ts', play, '--out', dir, '--from', String(FROM), '--ticks', String(TICKS), ...(DIE ? ['--die', DIE] : [])], { cwd: ROOT, env, stdout: 'pipe', stderr: 'pipe' });
  const out = text(r.stdout).trim(); if (out) say(out);
  if (r.exitCode !== 0 && r.exitCode !== 5) { say(text(r.stderr).split('\n').slice(-30).join('\n')); say(`typescript: FAILED, exit ${r.exitCode} (${secs(t)})`); process.exit(r.exitCode === 4 ? 4 : 3); }
  if (r.exitCode === 5) { ticks = 0; while (existsSync(join(dir, `a${ticks + 1}.bin`))) ticks++; if (!ticks) process.exit(3); say(`typescript: comparing the ${ticks} ticks it wrote`); }
  say(`typescript: ok (${secs(t)})`);
}

// ---- 3. the Rust side a tick at a time, each tick's diff before the next is stepped
const t0 = performance.now();
const run = Bun.spawn([BIN, dir, String(ticks), '--lockstep'], { cwd: ROOT, env, stdin: 'pipe', stdout: 'pipe', stderr: 'pipe' });
const errText = new Response(run.stderr).text();
const lines = (async function* () { const rd = run.stdout.getReader(), dec = new TextDecoder(); let buf = '';
  for (;;) { const { value, done } = await rd.read(); if (done) break; buf += dec.decode(value, { stream: true }); let i; while ((i = buf.indexOf('\n')) >= 0) { yield buf.slice(0, i); buf = buf.slice(i + 1); } }
  if (buf) yield buf; })();
const drop = (k: number) => { if (KEEP || k < 1) return; for (const f of [`a${k}`, `n${k}`, `s${k}`, `ns${k}`, ...(k > 1 ? [`c${k}`] : [])]) rmSync(join(dir, `${f}.bin`), { force: true }); };
const before = (k: number) => k > 1 ? `ticks 1..${k - 1} were the same but for what is known` : 'no tick before it was compared';
const foot = (k: number) => { say(`blobs of tick ${k} kept in ${dir}`); say(`  look at a path:  bun run tools/blitz/peek.ts ${dir} ${k} '<regex of paths>'`); };
let last = 0, panic: string[] | null = null, panicAt = 0, code = 0; const notes: string[] = [];
for await (const l of lines) {
  if (panic) { panic.push(l); continue; }
  if (l.startsWith('PANIC ')) { panic = []; panicAt = +l.slice(6); continue; }
  if (l.startsWith('NOTE ')) { notes.push(l.slice(5)); if (VERBOSE) say(`  rust: ${l.slice(5)}`); continue; }
  const m = /^TICK (\d+) /.exec(l); if (!m) { if (VERBOSE) say(`  rust: ${l}`); continue; }
  const k = +m[1];
  const d = Bun.spawnSync([BUN, 'run', 'tools/blitz/tick-diff.ts', dir, String(k), '--show', SHOW], { cwd: ROOT, env, stdout: 'pipe', stderr: 'pipe' });
  const out = text(d.stdout).trimEnd();
  if (d.exitCode === 0) { if (VERBOSE || k === 1 || k % 10 === 0 || k === ticks) say(out); drop(k - 1); last = k; if (k < ticks) { run.stdin.write('go\n'); run.stdin.flush(); } continue; }
  say(out || text(d.stderr).trimEnd()); code = d.exitCode === 1 ? 1 : d.exitCode === 4 ? 4 : 3;
  if (code === 1) { say(`STOPPED AT TICK ${k}: the first tick with a difference that known.txt does not name (${before(k)})`); for (const n of notes.filter(n => n.startsWith(`tick ${k}:`))) say(`  rust: ${n}`); foot(k); }
  break;
}
try { run.stdin.end(); } catch {}
if (code) { run.kill(); await run.exited; process.exit(code); }
const rc = await run.exited;
if (panic || rc !== 0) {
  const k = panicAt || last + 1;
  if (panic) { say(`THE RUST TICK ${k} PANICKED (${before(k)}):`); for (const l of panic) say(`  ${l.replaceAll(ROOT + '/', '')}`); }
  else { say(`THE RUST RUNNER DIED IN TICK ${k} with ${run.signalCode ?? `exit ${rc}`} and no panic (a stack overflow shows like this):`); say((await errText).split('\n').slice(-12).map(l => `  ${l}`).join('\n')); }
  say(`blobs in ${dir}; the tick's start on the TypeScript side is c${k}.bin`);
  process.exit(2);
}
say(`ticks 1..${ticks}: nothing unknown, and rust/sim's block the same bytes at every tick (${secs(t0)} for the Rust side and the diffs)`);
if (ticks < TICKS) { say(`but the TypeScript side stopped after ${ticks} of ${TICKS} ticks`); foot(ticks); process.exit(3); }
foot(ticks);
process.exit(0);
