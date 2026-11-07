// THE LEVELS TABLE (rust/game_start/LEVELS.md): every row of the mission list, started by the Rust start and held to the TypeScript start.
//   bun run tools/blitz/start-levels.ts [--seats 2] [--frames N] [--only id,id] [--write] [--keep DIR] [--at-once N] [--again]
// For each mission (scene/scenarioList.ts ALL_MISSIONS, read through `start.ts --missions`):
//   1. does the Rust start carry the file the page loads for it whole (`start --lacks`)? If not, the row says what it lacks;
//   2. if it does, `start.sh --mission <id> --frames N`: the two starts' states path by path, then both stepped in Rust for N frames with an idle
//      hand (the die's count and every part of the block after every frame), then the two end states path by path.
// `--seats 2` = each mission as a game of two players (the second with nobody at his controls). A mission the game itself refuses for two (a
// rescue, a quiet mission: "it takes one player today") is a row that says so, and is neither green nor red.
// The table is printed; `--write` also writes it to rust/game_start/LEVELS.md, between the two marks of that count of seats. Each mission's state
// files are kept under rust/game_start/target/start-levels/<seats>/<id>/ (`--keep DIR` = elsewhere). A row is GREEN when no path is unknown at
// the start or as the frames end and the two games never part. Exit 0 no row red · 1 a row is · 3 the tool could not run · 64 a wrong word.
// `--again`: ANOTHER TABLE, of the TypeScript start alone: is a SECOND start in one process the first? Each mission is stood, stepped `--frames`
// frames with an idle hand (by the Rust game: the module must be built, tools/blitz/face-build.ts --profile play), then the mission before it on
// the list is stood and stepped as long in the same process, then the mission is stood again, and the two starts are held to each other: rust/sim's
// block byte for byte, the state path for path, but for what is the process's on purpose (tools/blitz/known-again.txt, each with its reason). A
// game's state lives on the Game and a start writes all of it: what a module of src/ kept from a game is put back by tools/qa/fresh.ts, and a row
// here is red when something is not. `--frames 0` = no game is stepped (no module needed): only what a START leaves behind is seen.
// Three missions at once (`--at-once N`): each start is a process of its own with its own folder, about ten seconds of one core a row.
import { spawn, spawnSync } from 'node:child_process';
import { readFileSync, writeFileSync, mkdirSync, existsSync } from 'node:fs';
import { resolve } from 'node:path';
const ROOT = resolve(import.meta.dir, '../..');
const say = (s = '') => process.stdout.write(s + '\n');
const argv = process.argv.slice(2), arg = (n: string, d: string) => { const i = argv.indexOf(n); return i < 0 ? d : argv[i + 1]; };
const FRAMES = +arg('--frames', '300'), SEATS = +arg('--seats', '1'), AT_ONCE = Math.max(1, +arg('--at-once', '3')), ONLY = arg('--only', '').split(',').filter(Boolean), WRITE = argv.includes('--write'), AGAIN = argv.includes('--again');
const KEEP = resolve(arg('--keep', resolve(ROOT, 'rust/game_start/target/start-levels')), `${AGAIN ? 'again-' : ''}seats${SEATS}`);   // (this checkout's own: two checkouts' runs never share a state file)
if (WRITE && AGAIN) { say('start-levels: --again prints its table and writes none'); process.exit(64); }
if (WRITE && ONLY.length) { say('start-levels: --write with --only would write a table of some rows'); process.exit(64); }
if (SEATS !== 1 && SEATS !== 2) { say('start-levels: --seats 1 or 2'); process.exit(64); }
const ENV = { ...process.env, BUN_OPTIONS: '' };
const run = (cmd: string, args: string[], cwd = ROOT) => { const r = spawnSync(cmd, args, { cwd, encoding: 'utf8', maxBuffer: 1 << 26, env: ENV }); return { out: (r.stdout ?? '') + (r.stderr ?? ''), code: r.status ?? -1 }; };
const runLater = (cmd: string, args: string[], env: Record<string, string | undefined> = ENV) => new Promise<{ out: string; code: number }>(done => { let out = ''; const p = spawn(cmd, args, { cwd: ROOT, env }); p.stdout.on('data', d => { out += d; }); p.stderr.on('data', d => { out += d; }); p.on('close', code => done({ out, code: code ?? -1 })); });

if (!AGAIN) { const b = run('cargo', ['build', '--bin', 'start'], resolve(ROOT, 'rust/game_start')); if (b.code !== 0) { say(b.out.split('\n').filter(l => /^error/.test(l)).slice(0, 20).join('\n')); say('start-levels: the Rust start does not build'); process.exit(3); } }
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
/** --again: the mission stood, played, the one before it on the list stood and played, the mission stood again: the second start held to the first */
async function againOf(m: typeof missions[number]): Promise<Row> {
  const row: Row = { ...m, ended: 'DID NOT RUN', lacks: '', start: '', beside: '', end: '' };
  const out = resolve(KEEP, m.id); mkdirSync(out, { recursive: true });
  // (the missions before it on the list, nearest first: start.ts stands the first of them the game stands for this many players)
  const at = missions.indexOf(m), before = missions.map((_, i) => missions[(at + missions.length - 1 - i) % missions.length].id).filter(id => id !== m.id).slice(0, SEATS > 1 ? missions.length : 1).join(',');
  const r = await runLater('bun', ['run', '--preload', './tools/blitz/expose.ts', 'tools/blitz/start.ts', '--mission', m.id, '--out', out, ...(SEATS > 1 ? ['--seats', String(SEATS)] : []), '--again', '--played', String(FRAMES), '--between', before], FRAMES > 0 ? { ...ENV, TPS_GAME: 'rust' } : ENV);
  // (a mission the game refuses is refused at its FIRST start, before anything is stood: a refusal after `stood on the page's stand` is a red)
  const L = r.out.split('\n'), first = L.findIndex(l => /^start\.ts: .* stood on the page's stand/.test(l)), refusal = L.findIndex(l => /^start\.ts: the game refused/.test(l));
  if (refusal >= 0 && first < 0) { row.ended = 'refused'; row.lacks = `the game refuses it for ${SEATS} players`; return row; }
  const said = L.find(l => /^start\.ts: stood again/.test(l));
  if (r.code !== 0 || !said) { row.lacks = r.out.trim().split('\n').filter(l => !/^the call surface/.test(l)).slice(-3).join(' · ').slice(0, 300); return row; }
  const stoodBetween = /, then (\S+) stood/.exec(said)?.[1];
  row.beside = stoodBetween ? `after ${stoodBetween}` : 'NOTHING STOOD BETWEEN';
  // (a carried name nothing classes that a game changed: left as it is by a start, and named so that it gets its row — tools/qa/fresh.ts)
  const unlisted = L.find(l => /classed by nothing/.test(l)); if (unlisted) row.lacks = unlisted.replace(/^start\.ts: /, '').slice(0, 400);
  // OPEN, AND FORGIVEN BY NAME HERE: a mission's start writes its team's numbers into the process's knob tables, each kind into its own (a room
  // clear is staged by the night breach's code, so into that table and its own), and no start puts back a table it does not write: a mission
  // stood after one of another kind finds that kind's numbers as left. So for each row, the TEAM'S keys of the tables its own start does NOT
  // write are let differ, and nothing else of them; the tables it does write are held like any path. (The mend is a mission's numbers on the
  // Game: rust/game_start/README.md, the team's start.)
  const kind = /[?&](roomclear|nightbreach|rescue|quiet)\b/.exec(m.link)?.[1] ?? 'nightbreach', writes = kind === 'roomclear' ? ['roomclear', 'nightbreach'] : [kind];
  const others = ['roomclear', 'nightbreach', 'rescue'].filter(k => !writes.includes(k));
  const known = resolve(out, 'known-row.txt'); writeFileSync(known, others.map(k => `^g\\.(k\\.${k}\\.(sam_at|sam_yaw|ops_at|ops_yaw|squad|seed)\\b|m\\.${k}\\.applied\\b)\tthe ${k} missions' team numbers in the process's knob table, as the last mission that wrote them left them: this mission's start does not write that table (OPEN: start-levels.ts againOf)\n`).join(''));
  const d = await runLater('bun', ['run', 'tools/blitz/diff.ts', resolve(out, 'c1.bin'), resolve(out, 'again.bin'), '--blocks', resolve(out, 's0.bin'), resolve(out, 'again-s.bin'), '--out', resolve(out, 'diff-again.tsv'), '--canon-file', 'tools/blitz/canon.txt', '--known', `tools/blitz/known-again.txt,${known}`, '--live', '--show', '4']);
  let line = d.out.split('\n').find(l => /^\d+ paths differ/.test(l));
  // THE BLOCK, byte for byte, with one thing let be: in the part that holds the POSES, a row the first start had not made (every byte of it zero
  // there). A clip's pose at its first frame is a memo kept in that part, filled as clips are first used (src/game/character.ts AT0: a pure
  // function of the clip), so a game stood after another has rows the first lacked. A byte that differs where the first start HAS one is a red.
  if (line && !/; block 0 bytes/.test(line) && existsSync(resolve(out, 'parts.txt'))) {
    const a = readFileSync(resolve(out, 's0.bin')), b = readFileSync(resolve(out, 'again-s.bin')), P = readFileSync(resolve(out, 'parts.txt'), 'utf8').split('\n').map(l => l.split('\t')).find(c => c[0] === 'poses');
    let memo = 0, other = a.length === b.length ? 0 : 1;
    if (!other) for (let i = 0; i < a.length; i++) if (a[i] !== b[i]) { if (P && i >= +P[1] && i < +P[1] + +P[2] && a[i] === 0) memo++; else other++; }
    if (!other && memo) { line = line.replace(/; block \d+ bytes/, '; block 0 bytes'); row.end = `${memo} bytes of pose rows the first start had not made`; }
  }
  row.start = unknownOf(line); row.end = [/(\d+) known/.exec(d.out)?.[1] ?? '', row.end].filter(Boolean).join(' · ');
  row.ended = row.start === 'did not run' ? 'DID NOT RUN' : row.start === '0' && stoodBetween ? 'yes' : 'NOT EQUAL';
  return row;
}
if (AGAIN) {
  const todo = missions.filter(m => !ONLY.length || ONLY.includes(m.id)), rows: Row[] = new Array(todo.length); let next = 0;
  const worker = async () => { for (;;) { const i = next++; if (i >= todo.length) return; const row = rows[i] = await againOf(todo[i]);
    say(`${row.id}\t${row.file}\t${row.ended === 'yes' ? 'EQUAL' : row.ended === 'refused' ? 'refused by the game' : 'RED'}\t${row.ended === 'refused' ? '' : `${row.beside}\tnot known ${row.start}\tthe process's ${row.end}`}${row.ended === 'refused' || !row.lacks ? '' : `\t${row.lacks}`}`); } };
  await Promise.all(Array.from({ length: Math.min(AT_ONCE, todo.length) }, worker));
  const equal = rows.filter(r => r.ended === 'yes').length, refused = rows.filter(r => r.ended === 'refused').length, red = rows.length - equal - refused;
  say(); say(`${equal} of ${rows.length - refused} missions start a second time in one process as they did the first${FRAMES > 0 ? ` (after ${FRAMES} frames of the first game and of the mission before it on the list` : ' (no game stepped'}, die seed 20942${SEATS > 1 ? `, ${SEATS} seats` : ''})${refused ? `; ${refused} the game refuses` : ''}${red ? `; ${red} RED` : ''}.`);
  process.exit(red === 0 ? 0 : 1);
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
