// THE LEVELS TABLE (rust/game_start/LEVELS.md): every row of the mission list, started by the Rust start and held to the TypeScript start.
//   bun run tools/blitz/start-levels.ts [--frames N] [--only id,id] [--write] [--keep DIR]
// For each mission (scene/scenarioList.ts ALL_MISSIONS, read through `start.ts --missions`):
//   1. does the Rust start carry the file the page loads for it whole (`start --lacks`)? If not, the row says which records it lacks;
//   2. if it does, `start.sh --mission <id> --frames N`: the two starts' states path by path, then both stepped in Rust for N frames with an idle
//      hand (the die's count and every part of the block after every frame), then the two end states path by path.
// The table is printed; `--write` also writes it to rust/game_start/LEVELS.md, between the file's two marks. Each mission's state files are kept
// under rust/game_start/target/start-levels/<id>/ (`--keep DIR` = elsewhere). A row is GREEN when no path is unknown
// at the start or as the frames end and the two games never part. Exit 0 every row green · 1 a row is not · 3 the tool could not run.
// Runs one mission at a time (each start is a process of its own); about ten seconds a row.
import { spawnSync } from 'node:child_process';
import { readFileSync, writeFileSync, mkdirSync } from 'node:fs';
import { resolve } from 'node:path';
const ROOT = resolve(import.meta.dir, '../..');
const argv = process.argv.slice(2), arg = (n: string, d: string) => { const i = argv.indexOf(n); return i < 0 ? d : argv[i + 1]; };
const FRAMES = +arg('--frames', '300'), ONLY = arg('--only', '').split(',').filter(Boolean), WRITE = argv.includes('--write');
const KEEP = arg('--keep', resolve(ROOT, 'rust/game_start/target/start-levels'));   // (this checkout's own: two checkouts' runs never share a state file)
if (WRITE && ONLY.length) { say('start-levels: --write with --only would write a table of some rows'); process.exit(64); }
const run = (cmd: string, args: string[], cwd = ROOT) => { const r = spawnSync(cmd, args, { cwd, encoding: 'utf8', maxBuffer: 1 << 26, env: { ...process.env, BUN_OPTIONS: '' } }); return { out: (r.stdout ?? '') + (r.stderr ?? ''), code: r.status ?? -1 }; };
const say = (s = '') => process.stdout.write(s + '\n');

{ const b = run('cargo', ['build', '--bin', 'start'], resolve(ROOT, 'rust/game_start')); if (b.code !== 0) { say(b.out.split('\n').filter(l => /^error/.test(l)).slice(0, 20).join('\n')); say('start-levels: the Rust start does not build'); process.exit(3); } }
const listed = run('bun', ['run', '--preload', './tools/blitz/expose.ts', 'tools/blitz/start.ts', '--missions']);
const missions = listed.out.split('\n').map(l => l.split('\t')).filter(c => c.length === 4).map(([id, file, link, on]) => ({ id, file, link, listed: on === 'listed' }));
{ const stray = ONLY.filter(id => !missions.some(m => m.id === id)); if (stray.length) { say(`start-levels: no mission ${stray.join(', ')} (bun run --preload ./tools/blitz/expose.ts tools/blitz/start.ts --missions lists them)`); process.exit(64); } }
if (!missions.length) { say(listed.out.slice(-2000)); say('start-levels: start.ts --missions listed nothing'); process.exit(3); }

interface Row { id: string; file: string; link: string; listed: boolean; carried: boolean; ran: boolean; lacks: string; start: string; beside: string; end: string; green: boolean }
const rows: Row[] = [];
// (start.sh cuts a long line: the count is read with or without the end of its list)
const unknownOf = (line: string | undefined) => { if (!line) return 'did not run'; const m = /(\d+) NOT KNOWN(?:: ([^;]*))?/.exec(line); return !m ? 'did not run' : m[1] === '0' ? '0' : `${m[1]} (${(m[2] ?? '').trim()})`; };
for (const m of missions) {
  if (ONLY.length && !ONLY.includes(m.id)) continue;
  const row: Row = { ...m, carried: false, ran: false, lacks: '', start: '', beside: '', end: '', green: false };
  rows.push(row);
  const office = !m.link.includes('levelFile=');   // (its missions are a TypeScript module's, attached by name: start.ts stands both starts on a copy of the file with them written in)
  const lk = run(resolve(ROOT, 'rust/game_start/target/debug/start'), ['--lacks', `levels/${m.file}`]).out.split('\n').map(l => l.split('\t')).find(c => c.length >= 3);
  if (!lk) { row.lacks = 'the reader could not read the file'; say(`${m.id}\tunread file`); continue; }
  row.carried = lk[1] === 'carried'; row.lacks = [lk[2], lk[3] ? `unread: ${lk[3]}` : '', office ? 'its missions are not in its file (levels/office.scenarios.ts, attached by name): held on a copy of the file with them written in' : ''].filter(Boolean).join(' · ');
  if (!row.carried) { say(`${m.id}\tnot carried: ${row.lacks}`); continue; }
  const out = resolve(KEEP, m.id); mkdirSync(out, { recursive: true });
  const r = run('bash', ['tools/blitz/start.sh', '--mission', m.id, '--frames', String(FRAMES), '--no-build', '--show', '4', '--out', out]);
  const lines = r.out.split('\n');
  row.start = unknownOf(lines.find(l => /^\d+ paths differ/.test(l)));
  const b = lines.find(l => l.startsWith('BESIDE'));
  row.beside = !b ? 'did not run' : /PART at frame (\d+)/.test(b) ? `part at frame ${/PART at frame (\d+)/.exec(b)![1]}` : `${FRAMES} in step`;
  row.end = unknownOf(lines.find(l => l.startsWith('as the frames end')));
  if (r.code === 2) row.start = row.start === 'did not run' ? `the Rust start panicked: ${(lines.find(l => /panicked at/.test(l)) ? lines[lines.findIndex(l => /panicked at/.test(l)) + 1] : '').slice(0, 160)}` : row.start;
  row.ran = row.start !== 'did not run' && !row.start.startsWith('the Rust start panicked') && row.end !== 'did not run' && row.beside !== 'did not run';
  row.green = r.code === 0 && row.start === '0' && row.end === '0' && row.beside.endsWith('in step');
  say(`${m.id}\t${m.file}\tstart ${row.start}\t${row.beside}\tend ${row.end}\t${row.green ? 'GREEN' : 'RED'}`);
}

const cell = (s: string) => s.replace(/\|/g, '\\|');
const table = [
  `| mission | the file the page loads | starts in Rust | paths not known at the start | ${FRAMES} frames beside the TypeScript start | paths not known as they end | what the start lacks |`,
  '|---|---|---|---|---|---|---|',
  ...rows.map(r => `| \`${r.id}\`${r.listed ? '' : ' (off the list)'} | \`${r.file}\` | ${r.green ? 'yes' : !r.carried ? 'no' : r.ran ? 'NOT EQUAL' : 'DID NOT RUN'} | ${cell(r.start)} | ${cell(r.beside)} | ${cell(r.end)} | ${cell(r.lacks)} |`),
].join('\n');
const green = rows.filter(r => r.green).length;
const head = `${green} of ${rows.length} missions start in Rust equal to the TypeScript start (die seed 20942, ${FRAMES} frames).`;
say(); say(head);
if (WRITE) {
  const file = resolve(ROOT, 'rust/game_start/LEVELS.md'), text = readFileSync(file, 'utf8'), A = '<!-- table -->', B = '<!-- /table -->';
  const a = text.indexOf(A), b = text.indexOf(B);
  if (a < 0 || b < a) { say(`start-levels: ${file} has no ${A} … ${B}`); process.exit(3); }
  writeFileSync(file, `${text.slice(0, a + A.length)}\n${head}\n\n${table}\n${text.slice(b)}`);
  say(`written to ${file}`);
}
process.exit(green === rows.length ? 0 : 1);
