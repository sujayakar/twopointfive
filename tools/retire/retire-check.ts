// THE RETIRE TOOL'S OWN CHECK (tools/retire/retire.ts). The tool rewrites the game's code, and a fold that typechecks can still be another game: a die no longer
// drawn, a call no longer made, a module no longer loaded where it was. So this runs the tool on a small tree of its own (tools/retire/fixture, laid out as the
// repo is: knob tables, the registry, the page's switches, the stand's holds, the journal's lines, a panel's rows, a list of address words, a check) and holds:
//   - every case of fixture/src/game/use.ts gives the same answer, draws the same dice and makes the same calls after the fold as before it,
//   - the folded tree compiles, and no file is left with a placeholder in it,
//   - each thing that names a retired row is gone (its line of the table, its address word, its hold, its stand word, its panel row, its kept row), and what
//     only looks like one is not (the same word in another list),
//   - each row that is not a switch is refused, for its own reason, and the run says so by its exit code,
//   - `--drop` takes a declaration out with the statements that call it, in whatever file they are, and `--drop-at` one line of a check,
//   - a call is not folded away for its name, the same key on a thing that is not the table is not the table's, and a value that is not the page's is refused,
//   - tools/retire/lines.ts finds the line of a check that broke with the fold by where it was printed, and the tool takes it out by that place (and only a
//     line that is a statement of its own),
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
/** the tool on a tree, with what that tree's own page gives every row (tools/dump.ts there, run before anything is folded: the game's is tools/qa/stand-dump.ts) */
const retire = (dir: string, args: string[]) => { const page = join(dir, 'page.json'); if (!existsSync(page)) writeFileSync(page, run(['bun', 'run', join(dir, 'tools/dump.ts')]).out.trim().split('\n').pop()!);
  return run(['bun', 'run', join(HERE, 'retire.ts'), '--page', page, ...args], { RETIRE_ROOT: dir, TS_LIB }); };

// ---- the rows: fifteen that retire, one held number, and the rest the tool must refuse, each for the reason REFUSED gives
const ROWS: { row: string; value: boolean | number | string; hold?: boolean }[] = [
  { row: 'K.on', value: !BREAK }, { row: 'K.extra', value: true }, { row: 'K.old', value: false }, { row: 'K.viaList', value: true }, { row: 'K.panelOnly', value: true }, { row: 'K.num', value: 3 }, { row: 'K.neg', value: -1 },
  { row: 'K.snap', value: true }, { row: 'K.mode', value: 'fast' }, { row: 'K.quote', value: "it's" }, { row: 'K.zero', value: 0 }, { row: 'K.deep.on', value: true }, { row: 'K.kl1', value: true }, { row: 'K.selfref', value: true },
  { row: 'N.reach', value: 5, hold: true }, { row: 'N.wide', value: 6, hold: true },
  { row: 'K.stay', value: false }, { row: 'K.listRead', value: true }, { row: 'K.preset', value: false }, { row: 'N.far', value: true }, { row: 'K.missing', value: true },
  { row: 'K.early', value: true }, { row: 'K.rides', value: false }, { row: 'K.forced', value: true }, { row: 'K.shared', value: true }, { row: 'K.wrong', value: true }, { row: 'LOOSE.a', value: true }, { row: 'K.half1', value: true },
  { row: 'K.iife', value: true }, { row: 'K.rt1', value: false }, { row: 'K.hl1', value: true }, { row: 'K.negp', value: -1 }, { row: 'K.deep2.on', value: true }, { row: 'K.ph', value: false },
  { row: 'M.on', value: true }, { row: 'K.sw', value: 'b' }, { row: 'K.e2', value: true }, { row: 'K.e3', value: true }, { row: 'N.w2', value: 6, hold: true }, { row: 'N.w3', value: 6, hold: true },
];
const REFUSED: Record<string, RegExp> = { 'K.stay': /through a list of keys|writes it as it runs|whose type names it|handed to a call/, 'K.listRead': /read through a list of keys/, 'K.preset': /another value for it than the page's/, 'N.far': /the row is a number/, 'K.missing': /no key missing/,
  'K.early': /read as its module loads/, 'K.rides': /the page's write of it rides K\.stay, which stays/, 'K.forced': /a map of tables' rows holds another value/, 'K.shared': /a list of keys that something else reads too/, 'K.wrong': /the page gives it false .*true was asked for/, 'LOOSE.a': /typed by an index/, 'K.half1': /read by a key whose type names it/,
  'N.wide': /read as its module loads, where it is still 2/, 'K.iife': /read as its module loads/, 'K.rt1': /writes it as it runs, through a list of keys/, 'K.hl1': /a list of keys handed to a call with the table/,
  'K.negp': /a map of tables' rows holds another value/, 'K.deep2.on': /a map of tables' rows holds another value/, 'K.ph': /the game writes it as it runs/,
  'K.sw': /the subject of a switch/, 'K.e2': /read as its module loads \(in a function the module calls as it loads: src\/game\/use\.ts:\d+ mkLoad\(\)\)/, 'K.e3': /read as its module loads \(in a function the module calls as it loads: src\/game\/use\.ts:\d+ new Brain\(\)\)/,
  'N.w2': /read as its module loads, where it is still 2/, 'N.w3': /read as its module loads, where it is still 2/ };

console.log(`retire-check: the retire tool on its own small tree${BREAK ? ' (RETIRE_BREAK=1: K.on folded at the wrong value)' : ''}`);
const made: string[] = [];
try {
  // ---- A. the fold
  const d = fresh(); made.push(d); const rowsFile = join(d, 'rows.json'), reportFile = join(d, 'report.json'); writeFileSync(rowsFile, JSON.stringify(ROWS));
  const errs0 = errors(d), before = cases(d);
  line(errs0.length === 0 && typeof before !== 'string' && Object.keys(before).length >= 104, 'the tree as it stands compiles and its cases run', `${errs0.length} errors · ${typeof before === 'string' ? before : Object.keys(before).length + ' cases'}${errs0.length ? ' · ' + errs0.slice(0, 3).join(' | ') : ''}`);
  const r = retire(d, ['--rows', rowsFile, '--write', '--report', reportFile, '--pure', 'vouched', ...(BREAK ? ['--trust', 'K.on'] : [])]);   // (the red path: the tool reads the page's own write and would refuse the wrong value, so it is told to take it on trust)
  const rep = existsSync(reportFile) ? JSON.parse(readFileSync(reportFile, 'utf8')) : { retired: [], refused: [] };
  const want = ROWS.filter(x => !(x.row in REFUSED)).map(x => x.row);
  line(r.code === 3 && JSON.stringify([...rep.retired].sort()) === JSON.stringify([...want].sort()), 'the rows that are switches are done, and the run says by its exit code that some were refused', `exit ${r.code} · retired ${rep.retired.length} of ${ROWS.length}${r.code !== 3 ? ' · ' + r.out.slice(0, 300) : ''}`);
  const refused: Record<string, string> = Object.fromEntries(rep.refused.map((x: { row: string; why: string }) => [x.row, x.why]));
  const wrongWhy = Object.entries(REFUSED).filter(([row, re]) => !re.test(refused[row] ?? ''));
  line(wrongWhy.length === 0 && Object.keys(refused).length === Object.keys(REFUSED).length, 'refused, each for its own reason: a row the game writes, reads through a list of keys, reads as its module loads, holds at another value in a literal or a map, or writes through a list something else reads; a row whose page value rides a row that stays, or is not the value asked for; a table typed by an index; a value of the wrong kind; a row that is not there', wrongWhy.map(([row]) => `${row}: ${refused[row] ?? 'NOT REFUSED'}`).join(' | ') || Object.keys(refused).join(' '));
  const after = cases(d);
  const differ = typeof before === 'string' || typeof after === 'string' ? ['(the cases did not run)'] : Object.keys(before).filter(k => JSON.stringify(before[k]) !== JSON.stringify((after as Record<string, unknown>)[k]));
  line(differ.length === 0, 'every case answers the same, draws the same dice and makes the same calls after the fold', differ.length ? differ.slice(0, 6).map(k => typeof before === 'string' || typeof after === 'string' ? `${after}`.slice(0, 200) : `${k}: ${JSON.stringify(before[k])} → ${JSON.stringify((after as Record<string, unknown>)[k])}`).join(' | ') : `${Object.keys(before).length} cases`);
  const errs = errors(d), text = (f: string) => readFileSync(join(d, f), 'utf8'), nul = walk(d).filter(f => readFileSync(f, 'utf8').includes('\u0000'));
  line(errs.length === 0 && nul.length === 0, 'the folded tree compiles and no file holds a placeholder', `${errs.length} errors${errs.length ? ': ' + errs.slice(0, 4).join(' | ') : ''}${nul.length ? ' · a placeholder in ' + nul.join(' ') : ''}`);
  const k = text('src/game/k.ts'), table = k.slice(k.indexOf('export const K'), k.indexOf('};'));
  line(!/^\s+(on|extra|old|viaList|panelOnly|num|neg|kl1|selfref):/m.test(table) && /deep: \{ gain: 2 \}/.test(table) && /stay: false/.test(table) && !/a rule that is on in the page/.test(k) && /reach: 5, far: 9/.test(k), 'the rows are out of their table with the comment each had, the refused rows stand, and the held number is the page\'s', table.replace(/\s+/g, ' ').slice(0, 160));
  const prof = text('src/host/profile.ts'), stand = text('src/host/standup.ts'), env = text('src/host/standEnv.ts');
  line(!/K\.(on|extra|old|viaList|panelOnly)\b/.test(prof) && /K\.listRead = !q\.has\('other'\)/.test(prof) && !/N\.reach/.test(prof + stand) && !/'K\.(on|extra|viaList|panelOnly)'|'N\.reach'/.test(prof) && /'K\.stay': 'TPS_KEEP=1'/.test(prof),
    'the page\'s writes, the stand\'s holds and the list\'s lines of the retired rows are gone; a refused row keeps all three', prof.slice(prof.indexOf('export const FROZEN_HOLDS')).replace(/\s+/g, ' ').slice(0, 120));
  line(!/TPS_ON|TPS_EXTRA/.test(stand + env) && /'TPS_PROFILE', 'TPS_KEEP'/.test(env) && /pinned\('TPS_KEEP'\)/.test(stand), 'a stand word no hold reads any more is out of the stand\'s list; one that is still read stays', env.split('\n').pop() || env.slice(-80));
  const words = text('src/ui/words.ts');
  line(/VISIT_FLAGS = \['other'\]/.test(words) && /ACTS = \['extra', 'duck', 'run'\]/.test(words), 'an address word nothing reads is out of the list of address words, and the same word in another list stays', words.split('\n').filter(l => l.startsWith('export')).join(' · '));
  const jr = text('src/game/journal.ts'), ui = text('src/ui/bindings.ts');
  line(!/viaList|panelOnly|snap/.test(jr) && /\['stay'\] as const/.test(jr) && /SHARED = \['shared', 'stay'\]/.test(jr), 'the journal\'s lines for an old file: a list every key of which retired goes with its loop, a list one key of which stays keeps that key, a line that read the recording\'s own word goes with its write, and a list something else reads is left whole', jr.split('\n').filter(l => /for \(|if \(/.test(l)).join(' · ').slice(0, 200));
  line(!/panelOnly|viaList|extra|'the rule'|'the old way'|k: 'on'/.test(ui) && /K: \{ stay: null \}/.test(ui) && /\{ k: 'stay', label: 'a row that stays' \}/.test(ui) && /\{ k: 'stay', label: 'on' \}/.test(ui) && /knobRows\('sec', K as unknown as KnobTable, null, \(\) => \{\}, \[\{ k: 'stay', kind: 'toggle' \}\]\)/.test(ui) && /listRead: null/.test(ui) && /'state'/.test(ui) && /K: \{ forced: false \}/.test(ui) && /\{ label: 'state', get: \(\) => K\.stay/.test(ui), 'the panel\'s toggles and the rows named beside the table\'s name or the table itself (a list of rows, what a machine keeps), and a control kept as data, are gone for the retired rows alone; a label that reads like a key stays', ui.split('\n').filter(l => /KEPT|rows\(/.test(l)).join(' · ').slice(0, 200));
  const use = text('src/game/use.ts'), helper = text('src/game/helper.ts'), side = text('src/game/sidefx.ts');
  line(/^import '\.\/sidefx';$/m.test(use) && /^import '\.\/sidefx2'; import \{ Kind \} from '\.\/sidefx2';$/m.test(use) && /^import '\.\/sidefx3';/m.test(use) && /^import '\.\/sidefx4';/m.test(use) && !/onlyOff\(|onlyOff2/.test(use + side) && /LOG\.loaded\+\+/.test(side), 'an import whose last use as a value went — its last name gone, only names of types left, a name some other word in the file still spells, a whole module taken by one name — is still an import where it was: its module still loads', (use.match(/^import .*sidefx.*$/gm) ?? ['(no import)']).join(' · '));
  { const um = text('src/game/usem.ts'), um2 = text('src/game/usem2.ts'), kt = text('src/game/k.ts'), m = text('src/game/m.ts');
    line(/^import '\.\/m'; ?$/m.test(um) && !/\bM\b/.test(um.replace(/^import.*$/gm, '').replace(/\/\*\*.*\*\//g, '')) && /^import '\.\/m'; ?$/m.test(um2) && /const y = tell\(2\); return y;/.test(um2) && /export const M = \{\s*\}/.test(m), 'a table read only in conditions the fold takes away whole (beside something else in an `&&`, as a `?:` of its own, into one name of two in a `const`) leaves a bare import of its module where the import stood', `${(um.match(/^import.*m';.*$/m) ?? ['(none)'])[0]} · ${(um2.match(/^import.*m';.*$/m) ?? ['(none)'])[0]} · ${(um2.match(/const .*$/m) ?? [''])[0].slice(0, 60)}`);
    line(/oldGain: 4/.test(kt) && /\{ k: 'oldGain'/.test(ui) && /legacyY: 4/.test(use) && /const s = xs\.sort\(\)|xs\.sort\(\);/.test(use) && /seen\.delete\('a'\)/.test(use) && /gv\.val/.test(use) && /viaSink\.v/.test(use) && /f\.tables\.K\.extra === true/.test(use),
      'a number of the table that a list still names as a word stays though its last reader went; a key of a part of a thing that is read whole a level up stays; a method of the language\'s own that changes its thing (`sort`, a Map\'s `delete`), a getter a subclass overrides, a getter that calls through an interface, and an untyped thing\'s key named like the table are all left as they were', `${(use.match(/libSort\(\).*$/m) ?? [''])[0].slice(0, 110)}`); }
  line(/store\.get\('ab'\)/.test(use) && /store\.get\('abc'\);/.test(use) && /store\.round\(2\)/.test(use) && /seat\.next/.test(use) && /swapped\(\)/.test(use) && /withDefault\(\)/.test(use) && /seat\['next'\] > 2/.test(use) && /seat2\.next > 2/.test(use) && /virt\.peek\(\) > 5/.test(use), 'a call is not folded away for its name: a method of the game\'s own called `get` or `round`, a getter (by its name, by a key, under a type that calls it plain data), a method another class overrides, a function a `let` holds, a default worked out at the call all stay', `${(use.match(/userGet\(\).*$/m) ?? [''])[0].slice(0, 90)}`);
  line(/export \{ fastX \}/.test(use) && /"it's"/.test(use) && /\(3\)\.toFixed\(1\)/.test(use) && /=> \(\{ budget: g \}\)/.test(use) && /legacyC: 4/.test(use) && /return \{ a: 1 \};/.test(use) && /return D\.gain;/.test(use) && /return typeof true \+ typeof 3;|return typeof\(true\) \+ typeof\(3\);/.test(use) && /returnParen\(\): number \{ return ?\(?1\)?; \}/.test(use) && /const f = \(\) => \(\{ a: 1 \}\);/.test(use) && /return \(!flag\(\)\)\.toString\(\)\.length;/.test(use) && /legacyZ: 4/.test(use) && /\(line: string, bare = true\): string =>/.test(use), 'the text comes out well formed: a name kept where a name is wanted, a word with a quote in it, a number before a dot, an object as an arrow\'s body, a spread left with nothing to spread, a part of the table under a name of its own, and a key of an object that is also read whole stays', (use.match(/parenNumber.*$/m) ?? [''])[0].slice(0, 80));
  line(!/offA|offDeep|addOld/.test(helper + use) && /export function onA/.test(helper) && /add\(x: number\)/.test(helper), 'what only the dropped branches called is gone, to the end of the chain (a function, the function it alone called, a method)', helper.split('\n').filter(l => /function|\(x: number\)/.test(l)).map(l => l.trim().slice(0, 40)).join(' · '));
  line((rep.unread ?? []).length === 5 && /xs\.sort\(\);/.test(use) && /seen\.delete\('a'\);/.test(use) && /Math\.floor\(die\(\) \* out\.length\);/.test(use) && /tell\(4\);/.test(use) && /store\.get\('abc'\);/.test(use), 'a value nothing reads any more keeps what made it (the die drawn, the call made) and is said', (rep.unread ?? []).join(' | ').slice(0, 400));
  line(/return \['a', \.\.\.\(?\['on'\]\)?, 'z'\];/.test(use) && !/locksOf|vouched/.test(use) && !(rep.leftovers ?? []).includes(`src/game/use.ts:${use.split('\n').findIndex(l => l.includes('spreadRows')) + 1}`), 'a condition beside a call the caller vouches for (--pure) is folded away, and a spread it leaves empty goes from its list', (use.match(/spreadRows.*$/m) ?? [''])[0].slice(0, 160));
  const tool = text('tools/qa/t-check.ts');
  const typed = text('tools/qa/t-typed.ts');
  line(!/KK\.on|Object\.assign|for \(const key/.test(tool) && /K\.stay = keep\.stay/.test(tool) && /setFlag\(true, 'night'\)/.test(tool) && /\[, K\.stay\] = both/.test(tool) && /const pair = \[K\.stay, \(?K\.stay\)?\];/.test(tool) && /typedRead = \(\): boolean => true;/.test(typed) && /typedRead2 = \(\): boolean => true;/.test(typed) && !/the stand: on/.test(typed) && /F\.verdict\(K\.stay === false, 'a row that stays'\);/.test(typed) && (rep.toolWrites ?? []).some((w: { kind: string }) => w.kind === 'opposite'), 'a check that knows the table only by its name: its raises and put-backs of a retired row are gone, under another name for the table too, its off arm is said, a call that sets the stage for the next line stays, a pattern and a comma that wrote the row still compile, a check that takes the table through a typed import is folded too, and there a verdict reached through a thing\'s own function is known for one', (rep.toolWrites ?? []).map((w: { kind: string; at: string }) => `${w.kind} ${w.at}`).join(' · ').slice(0, 200));

  // ---- B. --drop: a declaration out, with every statement that calls it
  const e = fresh(); made.push(e);
  const rb = retire(e, ['--drop', 'src/game/zcheck.ts:zCheck', '--write']); const caller = readFileSync(join(e, 'src/game/caller.ts'), 'utf8'), eb = errors(e);
  line(rb.code === 0 && eb.length === 0 && !/zCheck/.test(caller) && !/function zCheck/.test(readFileSync(join(e, 'src/game/zcheck.ts'), 'utf8')) && /default: return 3;/.test(caller) && /LIST = \[zKeep\]/.test(caller) && /case 1: return zKeep\(\)/.test(caller),
    '--drop takes a function out with the statements that call it in another file (at the same offsets, and under a `default:`), and out of a list', `exit ${rb.code} · ${eb.length} errors${eb.length ? ': ' + eb.slice(0, 3).join(' | ') : ''} · ${caller.split('\n').filter(l => /switch|run2|LIST/.test(l)).map(l => l.trim()).join(' · ').slice(0, 200)}`);

  // ---- the check's lines, end to end (tools/retire/lines.ts): the fixture's own check run before the fold and after it, the line that broke found by where it
  //      was printed, taken out by that place, and the check run once more
  { const t = fresh(); made.push(t); const L = (args: string[]) => run(['bun', 'run', join(HERE, 'lines.ts'), ...args, '--tree', t]), was = join(t, 'lines-before'), now = join(t, 'lines-after'), then = join(t, 'lines-then'), drop = join(t, 'drop.json');
    L(['run', was, 'tools/qa/t-check.ts']); const rr = join(t, 'rows.json'); writeFileSync(rr, JSON.stringify(ROWS.filter(x => !(x.row in REFUSED))));
    retire(t, ['--rows', rr, '--write']); L(['run', now, 'tools/qa/t-check.ts']); const dd = L(['diff', was, now, '--drop', drop]); const places: string[] = existsSync(drop) ? JSON.parse(readFileSync(drop, 'utf8')) : [];
    const rdrop = retire(t, ['--drop-lines', drop, '--write']); L(['run', then, 'tools/qa/t-check.ts']); const d2 = L(['diff', was, then]); const tc = readFileSync(join(t, 'tools/qa/t-check.ts'), 'utf8');
    line(dd.code === 1 && /4 lines PASS → FAIL \(3 places, 1 NOT PLACED/.test(dd.out) && places.length === 3 && places.every(p => /^tools\/qa\/t-check\.ts:\d+:\d+$/.test(p)),
      'a check\'s lines that asked for the retired path are found by running the check before and after (PASS to FAIL) and placed by where each was printed, through a helper or straight from a function; a line whose place printed other lines too (a line a case) is not placed', `${(dd.out.split('\n')[0] ?? '').slice(0, 120)} · placed ${places.join(' ')}`);
    line(rdrop.code === 0 && /check lines taken out by name: 2 · NOT FOUND \(want a hand\): 1/.test(rdrop.out) && !/the rule off/.test(tc) && /\n  table\(\);/.test(tc) && /function table\(\)/.test(tc) && /printed straight from a function: a row that stays/.test(tc) && /await direct\(\)/.test(tc) && /await each\('kept'/.test(tc) && /the first line of a group/.test(tc) && /a verdict kept in a name/.test(tc),
      'taken out by those places: the two that are verdicts of their own go (the function that printed one straight keeps its other line and is still called); the one kept in a name is said as not found and the group round it is not touched, and the function that prints a line a case is as it was', `the drop: exit ${rdrop.code} · ${(rdrop.out.match(/check lines taken out.*$/m) ?? [''])[0].slice(0, 90)}`);
    line(d2.code === 1 && /2 lines PASS → FAIL/.test(d2.out) && /a verdict kept in a name/.test(d2.out) && /the second case/.test(d2.out), 'and the check run once more is red for those two lines alone', (d2.out.split('\n')[0] ?? '').slice(0, 110));
    // a check that throws after the fold (it exits 1, as a check with a red line does) is said as died; a line red before and after under other words is not a line that broke
    const o1 = join(t, 'o1'), o2 = join(t, 'o2'); mkdirSync(o1); mkdirSync(o2);
    writeFileSync(join(o1, 'a.txt'), '# a :: bun run a\n  PASS  one\n  PASS  two\n  PASS  three\n\nexit 0\n'); writeFileSync(join(o2, 'a.txt'), '# a :: bun run a\n  PASS  one\n\nTypeError: undefined is not an object (evaluating \'K.on.x\')\n      at check (/x/tools/qa/a.ts:3:9)\n\nexit 1\n');
    writeFileSync(join(o1, 'b.txt'), '# b :: bun run b\n  FAIL  the digest holds: got a3f9c2 of the stored one\n  PASS  two\n\nexit 1\n'); writeFileSync(join(o2, 'b.txt'), '# b :: bun run b\n  FAIL  the digest holds: got d4e8aa of the stored one\n  PASS  two\n\nexit 1\n');
    const d3 = L(['diff', o1, o2]);
    line(d3.code === 1 && /0 lines PASS → FAIL/.test(d3.out) && /1 red before and after/.test(d3.out) && /1 commands died/.test(d3.out) && /DIED # a/.test(d3.out), 'a check that throws after the fold is said as died although it exits 1, and a line that was red before under other words is not counted as broken', (d3.out.split('\n')[0] ?? '').slice(0, 150)); }

  // ---- one row of a chained page write (`K.viaList = K.snap = …`) retires alone: what it is given is the end of the chain, and the other row's write stays
  { const t = fresh(); made.push(t); const r1 = retire(t, ['K.viaList=true', '--write']); const pf = readFileSync(join(t, 'src/host/profile.ts'), 'utf8');
    line(r1.code === 0 && !/REFUSED/.test(r1.out) && /^\s*K\.snap = !q\.has\('novialist'\);/m.test(pf) && !/K\.viaList/.test(pf) && errors(t).length === 0, 'one row of a write that gives two rows one value retires alone: the other row is written there, not read, and its write stays', `exit ${r1.code} · ${(pf.match(/^.*novialist.*$/m) ?? ['(no line)'])[0].trim().slice(0, 80)}${/REFUSED/.test(r1.out) ? ' · ' + (r1.out.match(/REFUSED.*$/m) ?? [''])[0].slice(0, 160) : ''}`); }

  // ---- --drop-at: one line of a check out, by the place a stack frame gives for the call that printed it
  const f = fresh(); made.push(f); const tc0 = readFileSync(join(f, 'tools/qa/t-check.ts'), 'utf8').split('\n'), at0 = (words: string): string => { const i = tc0.findIndex(l => l.includes(words)); return `tools/qa/t-check.ts:${i + 1}:${tc0[i].indexOf('line(') + 1}`; };
  const rd = retire(f, ['--drop-at', at0('the rule off: the old answer'), '--write']); const tc = readFileSync(join(f, 'tools/qa/t-check.ts'), 'utf8');
  line(rd.code === 0 && !/the rule off: the old answer|function offArm/.test(tc) && /the stand: on, as the page opens/.test(tc) && /KK\.on = false/.test(tc), '--drop-at file:line:column takes one line of a check out with what only it used, and nothing else', `exit ${rd.code} · ${tc.split('\n').filter(l => /line\(|offArm/.test(l)).length} lines left that print or are the off arm's`);
  const g = fresh(); made.push(g); const rg = retire(g, ['--drop-at', at0('a verdict kept in a name'), '--drop-at', 'tools/qa/t-check.ts:5:60', '--drop-at', 'src/game/use.ts:20', '--write']);
  const rg2 = retire(g, ['--drop-at', at0('a verdict kept in a name'), '--drop-at', 'tools/qa/t-check.ts:5:60', '--write']);
  line(rg.code === 2 && /the game's own code is not taken out by a place/.test(rg.out) && rg2.code === 0 && /NOT FOUND \(want a hand\): 2/.test(rg2.out) && readFileSync(join(g, 'tools/qa/t-check.ts'), 'utf8') === tc0.join('\n'),
    '--drop-at takes nothing where the place is not a statement of its own (a verdict kept in a name, inside a group) or is the helper every line goes through, and never a line of the game', `exit ${rg.code}, then ${rg2.code} · ${(rg2.out.match(/check lines taken out.*$/m) ?? [''])[0].slice(0, 110)}`);

  // ---- C. a value that is none of true, false, a number, a quoted word stops the tool before it reads anything
  const c0 = fresh(); made.push(c0); const rc = retire(c0, ['K.on=False']); line(rc.code === 2 && /a value is true, false, a number/.test(rc.out), 'a value that is not true, false, a number or a quoted word stops the tool (a typo is not folded to a branch)', `exit ${rc.code}`);
} finally { for (const d of made) rmSync(d, { recursive: true, force: true }); }
console.log(fails ? `=== retire-check: ${fails} FAIL ===` : '=== retire-check: ALL PASS ===');
process.exit(fails ? 1 : 0);
