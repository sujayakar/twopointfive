// THE RETIRE TOOL'S OWN CHECK (tools/retire/retire.ts). The tool rewrites the game's code, and a fold that typechecks can still be another game: a die no longer
// drawn, a call no longer made, a module no longer loaded where it was. So this runs the tool on a small tree of its own (tools/retire/fixture, laid out as the
// repo is: knob tables, the registry, the page's switches, the stand's holds, the journal's lines, a panel's rows, a list of address words, a check) and holds:
//   - every case of fixture/src/game/use.ts gives the same answer, draws the same dice and makes the same calls after the fold as before it,
//   - the folded tree compiles, and no file is left with a placeholder in it,
//   - each thing that names a retired row is gone (its line of the table, its address word, its hold, its stand word, its panel row, its kept row), and what
//     only looks like one is not (the same word in another list),
//   - each row that is not a switch is refused, for its own reason, and the run says so by its exit code,
//   - `--drop` takes a declaration out with the statements that call it, in whatever file they are, and `--drop-at` one line of a check,
//   - a value that is not true, false, a number or a quoted word stops the tool.
//   bun run tools/retire/retire-check.ts            (TypeScript from vendor/typescript, or TS_LIB=<path to typescript.js>)
//   RETIRE_BREAK=1 bun run tools/retire/retire-check.ts    the red path: one row is folded at the WRONG value, and the cases must say so (the check must fail)
// A shape the tool is found to fold wrongly is added to the fixture as a case first.
import { copyFileSync, mkdirSync, mkdtempSync, readFileSync, readdirSync, rmSync, statSync, writeFileSync, existsSync } from 'node:fs';
import { join, resolve } from 'node:path';
import { tmpdir } from 'node:os';

const HERE = resolve(import.meta.dir), ROOT = resolve(HERE, '../..'), FIXTURE = join(HERE, 'fixture'), BREAK = process.env.RETIRE_BREAK === '1';
const TS_LIB = process.env.TS_LIB ?? join(ROOT, 'vendor/typescript/lib/typescript.js');
if (!existsSync(TS_LIB)) { console.error(`retire-check: no TypeScript at ${TS_LIB} (vendor/typescript, or TS_LIB=<path to typescript.js>)`); process.exit(2); }
const tsMod: any = await import(TS_LIB); const ts: any = tsMod.default ?? tsMod;

let fails = 0;
const line = (ok: boolean, label: string, detail = ''): void => { if (!ok) fails++; console.log(`  ${ok ? 'PASS' : 'FAIL'}  ${label}${detail ? ' — ' + detail : ''}`); };
const run = (cmd: string[], env: Record<string, string> = {}): { code: number; out: string } => { const p = Bun.spawnSync(cmd, { cwd: ROOT, env: { ...process.env, ...env }, stdout: 'pipe', stderr: 'pipe' }); return { code: p.exitCode ?? -1, out: p.stdout.toString() + p.stderr.toString() }; };
const copyTree = (from: string, to: string): void => { mkdirSync(to, { recursive: true }); for (const n of readdirSync(from)) { const a = join(from, n), b = join(to, n); if (statSync(a).isDirectory()) copyTree(a, b); else copyFileSync(a, b); } };
const fresh = (): string => { const d = mkdtempSync(join(tmpdir(), 'retire-check-')); copyTree(FIXTURE, d); return d; };
const walk = (dir: string, out: string[] = []): string[] => { for (const n of readdirSync(dir)) { const p = join(dir, n); if (statSync(p).isDirectory()) walk(p, out); else if (n.endsWith('.ts')) out.push(p); } return out; };
const cases = (dir: string): Record<string, unknown> | string => { const r = run(['bun', 'run', join(dir, 'tools/run.ts')]); try { return JSON.parse(r.out.trim().split('\n').pop()!); } catch { return r.out.slice(0, 400); } };
/** every error TypeScript has for the tree, as `file:line message` */
function errors(dir: string): string[] {
  const opts = ts.convertCompilerOptionsFromJson(JSON.parse(readFileSync(join(dir, 'tsconfig.json'), 'utf8')).compilerOptions, dir).options;
  const prog = ts.createProgram(walk(dir), { ...opts, noEmit: true });
  return [...prog.getSyntacticDiagnostics(), ...prog.getSemanticDiagnostics()].map((d: any) => `${d.file ? d.file.fileName.slice(dir.length + 1) + ':' + (d.file.getLineAndCharacterOfPosition(d.start).line + 1) : ''} ${ts.flattenDiagnosticMessageText(d.messageText, ' ')}`);
}
const retire = (dir: string, args: string[]) => run(['bun', 'run', join(HERE, 'retire.ts'), ...args], { RETIRE_ROOT: dir, TS_LIB });

// ---- the rows: seven that retire, one held number, and five the tool must refuse
const ROWS = [
  { row: 'K.on', value: !BREAK }, { row: 'K.extra', value: true }, { row: 'K.old', value: false }, { row: 'K.viaList', value: true }, { row: 'K.panelOnly', value: true }, { row: 'K.num', value: 3 }, { row: 'K.neg', value: -1 },
  { row: 'N.reach', value: 5, hold: true },
  { row: 'K.stay', value: false }, { row: 'K.listRead', value: true }, { row: 'K.preset', value: false }, { row: 'N.far', value: true }, { row: 'K.missing', value: true },
];
const REFUSED: Record<string, RegExp> = { 'K.stay': /read through a list of keys|writes it as it runs/, 'K.listRead': /read through a list of keys/, 'K.preset': /another value for it than the page's/, 'N.far': /the row is a number/, 'K.missing': /no key missing/ };

console.log(`retire-check: the retire tool on its own small tree${BREAK ? ' (RETIRE_BREAK=1: K.on folded at the wrong value)' : ''}`);
const made: string[] = [];
try {
  // ---- A. the fold
  const d = fresh(); made.push(d); const rowsFile = join(d, 'rows.json'), reportFile = join(d, 'report.json'); writeFileSync(rowsFile, JSON.stringify(ROWS));
  const errs0 = errors(d), before = cases(d);
  line(errs0.length === 0 && typeof before !== 'string' && Object.keys(before).length >= 40, 'the tree as it stands compiles and its cases run', `${errs0.length} errors · ${typeof before === 'string' ? before : Object.keys(before).length + ' cases'}${errs0.length ? ' · ' + errs0.slice(0, 3).join(' | ') : ''}`);
  const r = retire(d, ['--rows', rowsFile, '--write', '--report', reportFile, '--pure', 'vouched']);
  const rep = existsSync(reportFile) ? JSON.parse(readFileSync(reportFile, 'utf8')) : { retired: [], refused: [] };
  const want = ROWS.filter(x => !(x.row in REFUSED)).map(x => x.row);
  line(r.code === 3 && JSON.stringify([...rep.retired].sort()) === JSON.stringify([...want].sort()), 'the eight rows are done, and the run says by its exit code that some were refused', `exit ${r.code} · retired ${rep.retired.length} of ${ROWS.length}${r.code !== 3 ? ' · ' + r.out.slice(0, 300) : ''}`);
  const refused: Record<string, string> = Object.fromEntries(rep.refused.map((x: { row: string; why: string }) => [x.row, x.why]));
  const wrongWhy = Object.entries(REFUSED).filter(([row, re]) => !re.test(refused[row] ?? ''));
  line(wrongWhy.length === 0 && Object.keys(refused).length === Object.keys(REFUSED).length, 'a row the game writes, reads through a list of keys or holds at another value in a literal is refused, and so is a value of the wrong kind and a row that is not there', wrongWhy.map(([row]) => `${row}: ${refused[row] ?? 'NOT REFUSED'}`).join(' | ') || Object.keys(refused).join(' '));
  const after = cases(d);
  const differ = typeof before === 'string' || typeof after === 'string' ? ['(the cases did not run)'] : Object.keys(before).filter(k => JSON.stringify(before[k]) !== JSON.stringify((after as Record<string, unknown>)[k]));
  line(differ.length === 0, 'every case answers the same, draws the same dice and makes the same calls after the fold', differ.length ? differ.slice(0, 6).map(k => typeof before === 'string' || typeof after === 'string' ? `${after}`.slice(0, 200) : `${k}: ${JSON.stringify(before[k])} → ${JSON.stringify((after as Record<string, unknown>)[k])}`).join(' | ') : `${Object.keys(before).length} cases`);
  const errs = errors(d), text = (f: string) => readFileSync(join(d, f), 'utf8'), nul = walk(d).filter(f => readFileSync(f, 'utf8').includes('\u0000'));
  line(errs.length === 0 && nul.length === 0, 'the folded tree compiles and no file holds a placeholder', `${errs.length} errors${errs.length ? ': ' + errs.slice(0, 4).join(' | ') : ''}${nul.length ? ' · a placeholder in ' + nul.join(' ') : ''}`);
  const k = text('src/game/k.ts'), table = k.slice(k.indexOf('export const K'), k.indexOf('};'));
  line(!/\b(on|extra|old|viaList|panelOnly|num|neg):/.test(table) && /stay: false/.test(table) && !/a rule that is on in the page/.test(k) && /reach: 5, far: 9/.test(k), 'the rows are out of their table with the comment each had, the refused rows stand, and the held number is the page\'s', table.replace(/\s+/g, ' ').slice(0, 160));
  const prof = text('src/host/profile.ts'), stand = text('src/host/standup.ts'), env = text('src/host/standEnv.ts');
  line(!/K\.(on|extra|old|viaList|panelOnly)\b/.test(prof) && /K\.listRead = !q\.has\('other'\)/.test(prof) && !/N\.reach/.test(prof + stand) && !/'K\.(on|extra|viaList|panelOnly)'|'N\.reach'/.test(prof) && /'K\.stay': 'TPS_KEEP=1'/.test(prof),
    'the page\'s writes, the stand\'s holds and the list\'s lines of the retired rows are gone; a refused row keeps all three', prof.slice(prof.indexOf('export const FROZEN_HOLDS')).replace(/\s+/g, ' ').slice(0, 120));
  line(!/TPS_ON|TPS_EXTRA/.test(stand + env) && /'TPS_PROFILE', 'TPS_KEEP'/.test(env) && /pinned\('TPS_KEEP'\)/.test(stand), 'a stand word no hold reads any more is out of the stand\'s list; one that is still read stays', env.split('\n').pop() || env.slice(-80));
  const words = text('src/ui/words.ts');
  line(/VISIT_FLAGS = \['other'\]/.test(words) && /ACTS = \['extra', 'duck', 'run'\]/.test(words), 'an address word nothing reads is out of the list of address words, and the same word in another list stays', words.split('\n').filter(l => l.startsWith('export')).join(' · '));
  const jr = text('src/game/journal.ts'), ui = text('src/ui/bindings.ts');
  line(!/viaList|panelOnly/.test(jr) && /\['stay'\] as const/.test(jr), 'the journal\'s lines for an old file: a list every key of which retired goes with its loop, a list one key of which stays keeps that key', jr.split('\n').filter(l => /for \(|if \(/.test(l)).join(' · ').slice(0, 200));
  line(!/panelOnly|viaList|extra|'the rule'|'the old way'/.test(ui) && /K: \{ stay: null \}/.test(ui) && /\{ k: 'stay'/.test(ui) && /listRead: null/.test(ui) && /'state'/.test(ui), 'the panel\'s toggles and the rows named beside the table\'s name (a list of rows, what a machine keeps) are gone for the retired rows alone', ui.split('\n').filter(l => /KEPT|rows\(/.test(l)).join(' · ').slice(0, 200));
  const use = text('src/game/use.ts'), helper = text('src/game/helper.ts'), side = text('src/game/sidefx.ts');
  line(/^import '\.\/sidefx';$/m.test(use) && !/onlyOff/.test(use + side) && /LOG\.loaded\+\+/.test(side), 'an import whose last name went is still an import, where it was: its module still loads', (use.match(/^import .*sidefx.*$/m) ?? ['(no import)'])[0]);
  line(!/offA|offDeep|addOld/.test(helper + use) && /export function onA/.test(helper) && /add\(x: number\)/.test(helper), 'what only the dropped branches called is gone, to the end of the chain (a function, the function it alone called, a method)', helper.split('\n').filter(l => /function|\(x: number\)/.test(l)).map(l => l.trim().slice(0, 40)).join(' · '));
  line((rep.unread ?? []).length === 2 && /Math\.floor\(die\(\) \* out\.length\);/.test(use) && /tell\(4\);/.test(use), 'a value nothing reads any more keeps what made it (the die drawn, the call made) and is said', (rep.unread ?? []).join(' | ').slice(0, 200));
  line(/return \['a', \.\.\.\(?\['on'\]\)?, 'z'\];/.test(use) && !/locksOf|vouched/.test(use) && (rep.leftovers ?? []).every((l: string) => !/spreadRows/.test(l)), 'a condition beside a call the caller vouches for (--pure) is folded away, and a spread it leaves empty goes from its list', (use.match(/spreadRows.*$/m) ?? [''])[0].slice(0, 160));
  const tool = text('tools/qa/t-check.ts');
  line(!/KK|Object\.assign|for \(const key/.test(tool) && /K\.stay = keep\.stay/.test(tool) && (rep.toolWrites ?? []).some((w: { kind: string }) => w.kind === 'opposite'), 'a check that knows the table only by its name: its raises and put-backs of a retired row are gone, another name for the table too, and its off arm is said', (rep.toolWrites ?? []).map((w: { kind: string; at: string }) => `${w.kind} ${w.at}`).join(' · ').slice(0, 200));

  // ---- B. --drop: a declaration out, with every statement that calls it
  const e = fresh(); made.push(e);
  const rb = retire(e, ['--drop', 'src/game/zcheck.ts:zCheck', '--write']); const caller = readFileSync(join(e, 'src/game/caller.ts'), 'utf8'), eb = errors(e);
  line(rb.code === 0 && eb.length === 0 && !/zCheck/.test(caller) && !/function zCheck/.test(readFileSync(join(e, 'src/game/zcheck.ts'), 'utf8')) && /default: return 3;/.test(caller) && /LIST = \[zKeep\]/.test(caller) && /case 1: return zKeep\(\)/.test(caller),
    '--drop takes a function out with the statements that call it in another file (at the same offsets, and under a `default:`), and out of a list', `exit ${rb.code} · ${eb.length} errors${eb.length ? ': ' + eb.slice(0, 3).join(' | ') : ''} · ${caller.split('\n').filter(l => /switch|run2|LIST/.test(l)).map(l => l.trim()).join(' · ').slice(0, 200)}`);

  // ---- --drop-at: one line of a check out, by the place a stack frame gives for the call that printed it
  const f = fresh(); made.push(f); const rd = retire(f, ['--drop-at', 'tools/qa/t-check.ts:11:3', '--write']); const tc = readFileSync(join(f, 'tools/qa/t-check.ts'), 'utf8');
  line(rd.code === 0 && !/the rule off: the old answer|function offArm/.test(tc) && /the stand: on, as the page opens/.test(tc) && /KK\.on = false/.test(tc), '--drop-at file:line:column takes one line of a check out with what only it used, and nothing else', `exit ${rd.code} · ${tc.split('\n').filter(l => /line\(|offArm/.test(l)).length} lines left that print or are the off arm's`);

  // ---- C. a value that is none of true, false, a number, a quoted word stops the tool before it reads anything
  const rc = retire(fresh(), ['K.on=False']); line(rc.code === 2 && /a value is true, false, a number/.test(rc.out), 'a value that is not true, false, a number or a quoted word stops the tool (a typo is not folded to a branch)', `exit ${rc.code}`);
} finally { for (const d of made) rmSync(d, { recursive: true, force: true }); }
console.log(fails ? `=== retire-check: ${fails} FAIL ===` : '=== retire-check: ALL PASS ===');
process.exit(fails ? 1 : 0);
