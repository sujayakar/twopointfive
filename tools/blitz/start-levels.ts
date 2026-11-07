// THE LEVELS TABLE (rust/game_start/LEVELS.md): every row of the mission list, started by the Rust start and held to the TypeScript start.
//   bun run tools/blitz/start-levels.ts [--seats 2] [--frames N] [--only id,id] [--write] [--keep DIR] [--at-once N]
// For each mission (scene/scenarioList.ts ALL_MISSIONS, read through `start.ts --missions`):
//   1. does the Rust start carry the file the page loads for it whole (`start --lacks`)? If not, the row says what it lacks;
//   2. if it does, `start.sh --mission <id> --frames N`: the two starts' states path by path, then both stepped in Rust for N frames with an idle
//      hand (the die's count and every part of the block after every frame), then the two end states path by path.
// `--seats 2` = each mission as a game of two players (the second with nobody at his controls). A mission the game itself refuses for two (a
// rescue, a quiet mission: "it takes one player today") is a row that says so, and is neither green nor red.
// The table is printed; `--write` also writes it to rust/game_start/LEVELS.md, between the two marks of that count of seats. Each mission's state
// files are kept under rust/game_start/target/start-levels/<seats>/<id>/ (`--keep DIR` = elsewhere). A row is GREEN when no path is unknown at
// the start or as the frames end and the two games never part. Exit 0 no row red · 1 a row is · 3 the tool could not run · 64 a wrong word.
// Three missions at once (`--at-once N`): each start is a process of its own with its own folder, about ten seconds of one core a row.
import { spawn, spawnSync } from 'node:child_process';
import { readFileSync, writeFileSync, mkdirSync } from 'node:fs';
import { resolve } from 'node:path';
const ROOT = resolve(import.meta.dir, '../..');
const say = (s = '') => process.stdout.write(s + '\n');
const argv = process.argv.slice(2), arg = (n: string, d: string) => { const i = argv.indexOf(n); return i < 0 ? d : argv[i + 1]; };
const FRAMES = +arg('--frames', '300'), SEATS = +arg('--seats', '1'), AT_ONCE = Math.max(1, +arg('--at-once', '3')), ONLY = arg('--only', '').split(',').filter(Boolean), WRITE = argv.includes('--write');
const KEEP = resolve(arg('--keep', resolve(ROOT, 'rust/game_start/target/start-levels')), `seats${SEATS}`);   // (this checkout's own: two checkouts' runs never share a state file)
if (WRITE && ONLY.length) { say('start-levels: --write with --only would write a table of some rows'); process.exit(64); }
if (SEATS !== 1 && SEATS !== 2) { say('start-levels: --seats 1 or 2'); process.exit(64); }
const ENV = { ...process.env, BUN_OPTIONS: '' };
const run = (cmd: string, args: string[], cwd = ROOT) => { const r = spawnSync(cmd, args, { cwd, encoding: 'utf8', maxBuffer: 1 << 26, env: ENV }); return { out: (r.stdout ?? '') + (r.stderr ?? ''), code: r.status ?? -1 }; };
const runLater = (cmd: string, args: string[]) => new Promise<{ out: string; code: number }>(done => { let out = ''; const p = spawn(cmd, args, { cwd: ROOT, env: ENV }); p.stdout.on('data', d => { out += d; }); p.stderr.on('data', d => { out += d; }); p.on('close', code => done({ out, code: code ?? -1 })); });

{ const b = run('cargo', ['build', '--bin', 'start'], resolve(ROOT, 'rust/game_start')); if (b.code !== 0) { say(b.out.split('\n').filter(l => /^error/.test(l)).slice(0, 20).join('\n')); say('start-levels: the Rust start does not build'); process.exit(3); } }
const listed = run('bun', ['run', '--preload', './tools/blitz/expose.ts', 'tools/blitz/start.ts', '--missions']);
const missions = listed.out.split('\n').map(l => l.split('\t')).filter(c => c.length === 4).map(([id, file, link, on]) => ({ id, file, link, listed: on === 'listed' }));
if (!missions.length) { say(listed.out.slice(-2000)); say('start-levels: start.ts --missions listed nothing'); process.exit(3); }
{ const stray = ONLY.filter(id => !missions.some(m => m.id === id)); if (stray.length) { say(`start-levels: no mission ${stray.join(', ')} (bun run --preload ./tools/blitz/expose.ts tools/blitz/start.ts --missions lists them)`); process.exit(64); } }

/** how a row ended: equal · a path differs or the games part · the Rust reader refuses the file · the game refuses the mission · a start stopped */
type Ended = 'yes' | 'NOT EQUAL' | 'no' | 'refused' | 'DID NOT RUN';
interface Row { id: string; file: string; link: string; listed: boolean; ended: Ended; lacks: string; start: string; beside: string; end: string }
// (start.sh cuts a long line: the count is read with or without the end of its list)
const unknownOf = (line: string | undefined) => { if (!line) return 'did not run'; const m = /(\d+) NOT KNOWN(?:: ([^;]*))?/.exec(line), blk = /; block (\d+) bytes/.exec(line); if (!m) return 'did not run';
  const off = [m[1] === '0' ? '' : `${m[1]} (${(m[2] ?? '').trim()})`, blk && blk[1] !== '0' ? `the block differs in ${blk[1]} bytes` : ''].filter(Boolean).join(' · '); return off || '0'; };
async function rowOf(m: typeof missions[number]): Promise<Row> {
  const row: Row = { ...m, ended: 'DID NOT RUN', lacks: '', start: '', beside: '', end: '' };
  const lk = (await runLater(resolve(ROOT, 'rust/game_start/target/debug/start'), ['--lacks', `levels/${m.file}`])).out.split('\n').map(l => l.split('\t')).find(c => c.length >= 3);
  if (!lk) { row.lacks = 'the reader could not read the file'; return row; }
  row.lacks = lk[2] ?? '';
  if (lk[1] !== 'carried') { row.ended = 'no'; return row; }
  const out = resolve(KEEP, m.id); mkdirSync(out, { recursive: true });
  const r = await runLater('bash', ['tools/blitz/start.sh', '--mission', m.id, ...(SEATS > 1 ? ['--seats', String(SEATS)] : []), '--frames', String(FRAMES), '--no-build', '--show', '4', '--out', out]);
  const lines = r.out.split('\n');
  const refusal = lines.find(l => /^start\.ts: the game refused/.test(l));
  if (refusal) { row.ended = 'refused'; row.lacks = [`the game refuses it for ${SEATS} players (${refusal.replace(/^start\.ts: the game refused the \w+: /, '')})`, row.lacks].filter(Boolean).join(' · '); return row; }
  row.start = unknownOf(lines.find(l => /^\d+ paths differ/.test(l)));
  const b = lines.find(l => l.startsWith('BESIDE'));
  row.beside = !b ? 'did not run' : /PART at frame (\d+)/.test(b) ? `part at frame ${/PART at frame (\d+)/.exec(b)![1]}` : `${FRAMES} in step`;
  row.end = unknownOf(lines.find(l => l.startsWith('as the frames end')));
  const panic = lines.findIndex(l => /panicked at/.test(l));
  if (r.code === 2 && panic >= 0) row.lacks = [`a Rust game panicked: ${(lines[panic + 1] ?? '').slice(0, 200)}`, row.lacks].filter(Boolean).join(' · ');
  const ran = ![row.start, row.beside, row.end].includes('did not run');
  row.ended = !ran ? 'DID NOT RUN' : r.code === 0 && row.start === '0' && row.end === '0' && row.beside.endsWith('in step') ? 'yes' : 'NOT EQUAL';
  return row;
}
const todo = missions.filter(m => !ONLY.length || ONLY.includes(m.id)), rows: Row[] = new Array(todo.length);
{ let next = 0;
  const worker = async () => { for (;;) { const i = next++; if (i >= todo.length) return; const row = rows[i] = await rowOf(todo[i]);
    say(`${row.id}\t${row.file}\t${row.ended === 'yes' ? 'GREEN' : row.ended === 'refused' ? 'refused by the game' : 'RED'}\tstart ${row.start}\t${row.beside}\tend ${row.end}${row.ended === 'yes' ? '' : `\t${row.lacks}`}`); } };
  await Promise.all(Array.from({ length: Math.min(AT_ONCE, todo.length) }, worker)); }

const cell = (s: string) => s.replace(/\|/g, '\\|');
const table = [
  `| mission | the file the page loads | starts in Rust | paths not known at the start | ${FRAMES} frames beside the TypeScript start | paths not known as they end | what the start lacks |`,
  '|---|---|---|---|---|---|---|',
  ...rows.map(r => `| \`${r.id}\`${r.listed ? '' : ' (off the list)'} | \`${r.file}\` | ${r.ended === 'refused' ? 'not for this many players' : r.ended} | ${cell(r.start)} | ${cell(r.beside)} | ${cell(r.end)} | ${cell(r.lacks)} |`),
].join('\n');
const green = rows.filter(r => r.ended === 'yes').length, refused = rows.filter(r => r.ended === 'refused').length, red = rows.length - green - refused;
const head = `${green} of ${rows.length - refused} missions${SEATS > 1 ? ` the game stages for ${SEATS} players` : ''} start in Rust equal to the TypeScript start (die seed 20942, ${FRAMES} frames${SEATS > 1 ? `, ${SEATS} seats` : ''})${refused ? `; ${refused} the game refuses for ${SEATS} players` : ''}.`;
say(); say(head);
if (WRITE) {
  const file = resolve(ROOT, 'rust/game_start/LEVELS.md'), text = readFileSync(file, 'utf8'), mark = SEATS > 1 ? `table seats=${SEATS}` : 'table', A = `<!-- ${mark} -->`, B = `<!-- /${mark} -->`;
  const a = text.indexOf(A), b = text.indexOf(B);
  if (a < 0 || b < a) { say(`start-levels: ${file} has no ${A} … ${B}`); process.exit(3); }
  writeFileSync(file, `${text.slice(0, a + A.length)}\n${head}\n\n${table}\n${text.slice(b)}`);
  say(`written to ${file}`);
}
process.exit(red === 0 ? 0 : 1);
