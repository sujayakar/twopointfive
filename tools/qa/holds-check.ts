// THE STAND A PROBE IS OWED BY ITS NAME (tools/qa/holds.ts): the table and the one call, held to their word. It stands no game.
//   bun run tools/qa/holds-check.ts        exit 0 = all hold · 1 = one does not      (probes.ts holds; the same on both stands: it reads no stand)
//   HOLDS_BREAK=1 in front = a probe is run with nothing held and no fence round it, a check file's main is read as if it called its parts bare, and the runner
//   is asked with a word set by hand: lines (2), (4), (5) and (7) must FAIL (the lines about the table itself stay green).
// What it is for: a check was red run as a file and green in its family, twice in one day, because the runner held rows off for it and its own main did not.
// So (1) the table names rows the game has, under the words the frozen stand's own list gives them; (2) every probe the runner dispatches by a name in the table
// runs with its words at '0' and its rows off, and both are back after; (3) a word whoever runs the tool set is left as he set it; (4) the fence puts back what a
// probe wrote; (5) every check file with a main of its own that the runner dispatches such a name to runs it through the same call, by that name; (6) no name in
// the table is one the runner no longer has; (7) the runner's own run() holds a probe's words.
// WHAT (5) CANNOT SEE: that every arm of a main goes through the call (it reads the file's text for the call and the names; a main with one arm wrapped and one
// bare passes); a check file none of whose probes is in the table (it is not read at all: run alone its parts have no fence between them); and a probe the
// runner dispatches in a shape it does not read is said, not guessed.
import { readFileSync, readdirSync } from 'node:fs';
import { resolve } from 'node:path';
import { ROOT } from './headless.ts';
import { HOLDS, holdsOf, named, holdOff, asProbe, asProbes, exitCode, type Hold } from './holds.ts';

const HERE = import.meta.dir, tools = readdirSync(HERE).filter(x => x.endsWith('.ts') && !/^(holds|holds-check|probes)\.ts$/.test(x)).map(f => ({ f, src: readFileSync(resolve(HERE, f), 'utf8') }));

const BREAK = process.env.HOLDS_BREAK === '1';
let fails = 0; const line = (ok: boolean, label: string, detail = ''): void => { console.log(`  ${ok ? 'PASS' : 'FAIL'}  ${label}${detail ? ` — ${detail}` : ''}`); if (!ok) fails++; };
const few = (xs: string[], n = 6): string => xs.length ? `${xs.slice(0, n).join(' · ')}${xs.length > n ? ` … (${xs.length})` : ''}` : 'none';
/** the call under test, or under the break word a stand-in that runs the probe bare */
const under = <T>(name: string, fn: () => T | Promise<T>): Promise<T> => BREAK ? Promise.resolve(fn()) : asProbe(name, fn);

const { knobRegistry } = await import(`${ROOT}/src/game/knobs.ts`), { FROZEN_HOLDS } = await import(`${ROOT}/src/host/profile.ts`);
const R = knobRegistry() as Record<string, Record<string, unknown>>, rows: Hold[] = []; for (const r of HOLDS) for (const h of r.holds) if (!rows.some(o => o.word === h.word)) rows.push(h);
const at = (h: Hold): string => `${h.table}.${h.key}`, read = (h: Hold): unknown => R[h.table]?.[h.key];
if (BREAK) console.log('HOLDS_BREAK=1: probes run bare, mains read as bare, the runner asked with a word set by hand — lines (2), (4), (5) and (7) must FAIL');

// (1) the table against the game and the frozen stand's list
{ const bad: string[] = [];
  for (const h of rows) { const word = (FROZEN_HOLDS as Record<string, string>)[at(h)];
    if (typeof read(h) !== 'boolean') bad.push(`${at(h)}: no such switch in the knob registry`);
    else if (!word || !new RegExp(`^${h.word}=1\\b`).test(word)) bad.push(`${at(h)}: the frozen stand's list raises it by '${word ?? 'no word'}', the table says ${h.word}`); }
  for (const r of HOLDS) for (const h of r.holds) { const o = rows.find(x => x.word === h.word)!; if (at(o) !== at(h) || o.now !== h.now) bad.push(`${h.word}: written two ways in the table (${r.list})`); }
  if (new Set(HOLDS.map(r => r.list)).size !== HOLDS.length) bad.push('two rows of the table share a name');
  for (const { f, src } of tools) for (const m of src.matchAll(/\bheldList\('([^']+)'\)/g)) if (!HOLDS.some(r => r.list === m[1])) bad.push(`${f} asks for the row '${m[1]}', and the table has none of that name`);
  line(!bad.length && rows.length > 0, '(1) every held row is a switch the game has, under the word the frozen stand raises it by', bad.length ? few(bad, 4) : `${rows.length} switches in ${HOLDS.length} rows`); }

// the probes the runner has: every name it knows (its own `list`, from a child), which of them it runs (a name given to run(), or the first of an on()),
// and where each one's body is: a file (read off the two shapes the runner writes a dispatch line in — the name and the import on one line; a loop over
// [name, export] pairs with the import on the line under it), or the runner itself (a run('name', …) line that imports nothing)
const probesSrc = readFileSync(resolve(HERE, 'probes.ts'), 'utf8'), srcLines = probesSrc.split('\n');
const listed = Bun.spawnSync(['bun', 'run', resolve(HERE, 'probes.ts'), 'list'], { stdout: 'pipe', stderr: 'pipe' }), known = new Set<string>();
for (const ln of listed.stdout.toString().split('\n').slice(1)) for (const w of ln.split(/[\s:]+/)) if (/^[A-Za-z0-9][A-Za-z0-9-]*$/.test(w)) known.add(w);
/** the text of a call's arguments: from just after its '(' to the ')' that closes it */
const argsAt = (s: string, from: number): string => { let depth = 1, i = from; for (; i < s.length && depth; i++) { if (s[i] === '(') depth++; else if (s[i] === ')') depth--; } return s.slice(from, i - 1); };
const ran = new Set<string>(), placed = new Map<string, string>(), exported = new Map<string, string>(), inRunner = new Set<string>(), scripts = new Set<string>(), PAIRS = /for \(const \[\w+, \w+\] of \[(\[.*\])\]/;
srcLines.forEach((ln, i) => {
  for (const m of ln.matchAll(/\bon\(/g)) { const first = /^'([^']+)'/.exec(argsAt(ln, m.index! + 3)); if (first) ran.add(first[1]); }
  if (!/await run\(/.test(ln)) return;
  const script = /runScript\('([^']+)'/.exec(ln)?.[1], imp = /import\('\.\/([^']+)'\)\)(?: as any)?\)?\.(\w+)\(/.exec(ln), file = imp?.[1] ?? /import\('\.\/([^']+)'\)/.exec(ln)?.[1] ?? script; if (script) scripts.add(script);
  const lit = /await run\('([^']+)'/.exec(ln);
  if (lit) { ran.add(lit[1]); if (file) { placed.set(lit[1], file); if (imp) exported.set(lit[1], imp[2]); } else inRunner.add(lit[1]); return; }
  const pairs = PAIRS.exec(ln) ?? PAIRS.exec(srcLines[i - 1] ?? ''); if (pairs && file) for (const m of pairs[1].matchAll(/\['([^']+)', '(\w+)'\]/g)) { ran.add(m[1]); placed.set(m[1], file); exported.set(m[1], m[2]); }
});
/** the runner's probes: a known name it runs, or one that is nowhere a family's name only */
const probeNames = [...known].filter(n => ran.has(n)).sort(), heldNames = probeNames.filter(n => holdsOf(n).length);

// (2) each dispatched name with a hold: its words '0' inside and its rows off, both back after. Every row is put ON first (the module's own default is off)
{ const bad: string[] = []; let asked = 0;
  for (const name of heldNames) { const hs = holdsOf(name), was = new Map<Hold, unknown>();
    for (const h of hs) { delete process.env[h.word]; was.set(h, read(h)); R[h.table][h.key] = true; }
    const inside = await under(name, () => hs.flatMap(h => [...(process.env[h.word] !== '0' ? [`${h.word} reads ${process.env[h.word] ?? 'unset'}`] : []), ...(h.now && read(h) !== false ? [`${at(h)} is not off`] : []), ...(!h.now && read(h) !== true ? [`${at(h)} was written, and it is the word's alone`] : [])]));
    const after = hs.flatMap(h => [...(process.env[h.word] !== undefined ? [`${h.word} left at ${process.env[h.word]}`] : []), ...(read(h) !== true ? [`${at(h)} not put back`] : [])]);
    for (const [h, v] of was) R[h.table][h.key] = v; for (const h of hs) delete process.env[h.word];
    asked += hs.length; if (inside.length || after.length) bad.push(`${name}: ${[...inside, ...after].join(', ')}`); }
  line(!bad.length && heldNames.length > 0, '(2) a probe of a name in the table runs with its words at 0 and its rows off, and both are back after it', bad.length ? few(bad, 3) : `${heldNames.length} probes of the runner, ${asked} holds`); }

// (3) a word whoever runs the tool set is left as he set it: =1 and nothing of that row is touched (the name's other words held all the same); =0 and it is
// the hold itself — the row off inside, the word still his after
{ const name = heldNames.find(n => holdsOf(n).filter(h => h.now).length >= 2) ?? '', hs = holdsOf(name).filter(h => h.now), mine = hs[0], other = hs[1];
  if (!mine || !other) line(false, '(3) a word whoever runs the tool set himself is left as he set it', 'no probe of the runner holds two rows: the line is not asked');
  else { const was = read(mine), T = R[mine.table];
    process.env[mine.word] = '1'; T[mine.key] = true;
    const one = await asProbe(name, () => ({ word: process.env[mine.word], row: T[mine.key], otherWord: process.env[other.word] })), oneAfter = process.env[mine.word];
    process.env[mine.word] = '0'; T[mine.key] = true;
    const zero = await asProbe(name, () => ({ word: process.env[mine.word], row: T[mine.key] })), zeroAfter = process.env[mine.word], zeroRow = T[mine.key];
    delete process.env[mine.word]; T[mine.key] = was;
    const ok = one.word === '1' && one.row === true && oneAfter === '1' && one.otherWord === '0' && zero.word === '0' && zero.row === false && zeroAfter === '0' && zeroRow === true;
    line(ok, '(3) a word whoever runs the tool set himself is left as he set it: =1 is his to ask, =0 is the hold itself', `${name}: ${mine.word}=1 reads ${one.word} inside with ${at(mine)} ${one.row}, ${other.word} ${one.otherWord ?? 'unset'} · ${mine.word}=0 reads ${zero.word} inside with ${at(mine)} ${zero.row}, the word ${zeroAfter ?? 'gone'} after and the row ${zeroRow === true ? 'back' : 'not back'}`); } }

// (4) the fence: a knob a probe wrote, the cut-pieces switch and the plain night's card are back after it; and a row the registry has not stops a hold by name
{ const cut = (await import(`${ROOT}/src/scene/cut.ts`)).cutConfig, kit = await import(`${ROOT}/src/game/kit.ts`);
  const [tName, T] = Object.entries(R).find(([, t]) => Object.values(t).some(v => typeof v === 'number'))!, key = Object.keys(T).find(k => typeof T[k] === 'number')!, was = T[key] as number, cutWas = cut.on, soloWas = kit.KIT_SOLO.on;
  await under('holds-check-fence', () => { T[key] = was + 1; T.addedByTheCheck = 1; cut.on = !cutWas; kit.KIT_SOLO.on = !soloWas; });
  let refused = ''; try { (await holdOff([{ word: 'TPS_HOLDS_CHECK', table: 'NO_SUCH_TABLE', key: 'on', now: true }]))(); } catch (e: any) { refused = String(e?.message ?? e); }
  const ok = T[key] === was && !('addedByTheCheck' in T) && cut.on === cutWas && kit.KIT_SOLO.on === soloWas && /NO_SUCH_TABLE\.on/.test(refused) && process.env.TPS_HOLDS_CHECK === undefined;
  const detail = `${tName}.${key} ${T[key] === was ? 'back' : 'LEFT WRITTEN'} · an added key ${'addedByTheCheck' in T ? 'LEFT' : 'dropped'} · cut pieces ${cut.on === cutWas ? 'back' : 'LEFT'} · the plain night's card ${kit.KIT_SOLO.on === soloWas ? 'back' : 'LEFT'} · a row no table has ${refused ? 'refused by name' : 'NOT refused'}`;
  T[key] = was; delete T.addedByTheCheck; cut.on = cutWas; kit.KIT_SOLO.on = soloWas; delete process.env.TPS_HOLDS_CHECK;
  line(ok, '(4) what a probe wrote is put back after it: a knob, a key it added, the cut-pieces switch, the plain night\'s card', detail); }

// (5) a check file with a main of its own (or a script the runner runs as a child), dispatched under a name in the table: (a) it runs through asProbe /
// asProbes, (b) each such probe stands in it by its name (the name its main runs it under), and where the file keeps a table of its probes the name is beside
// the same function the runner calls for it, (c) a name it hands straight to asProbe is a probe the runner dispatches to that file
{ const files = new Map<string, string[]>(), unplaced: string[] = []; for (const n of heldNames) { const f = placed.get(n); if (f) files.set(f, [...(files.get(f) ?? []), n]); else if (!inRunner.has(n)) unplaced.push(n); }
  const bad: string[] = []; let mains = 0;
  const srcOf = (f: string): string => { const s = readFileSync(resolve(HERE, f), 'utf8'); return BREAK ? s.replaceAll('asProbe', 'bare') : s; };
  for (const [f, names] of files) { const src = srcOf(f); if (!/\bimport\.meta\.main\b/.test(src) && !scripts.has(f)) continue; mains++;
    if (!/\basProbes?\(/.test(src)) { bad.push(`${f}: its main runs ${names[0]}${names.length > 1 ? ` (+${names.length - 1})` : ''} bare`); continue; }
    const missing = names.filter(n => !src.includes(`'${n}'`)); if (missing.length) bad.push(`${f}: its main does not name ${few(missing, 3)}`);
    for (const m of src.matchAll(/\['([a-z0-9-]+)', (\w+)\]|'([a-z0-9-]+)': (\w+)\b/g)) { const name = m[1] ?? m[3], beside = m[2] ?? m[4], fn = exported.get(name);   // (a table's row, written ['name', fn] or 'name': fn)
      if (placed.get(name) === f && fn && fn !== beside) bad.push(`${f}: '${name}' stands beside ${beside}, and the runner calls ${fn} for it`); } }
  for (const { f, src } of tools) {
    for (const m of src.matchAll(/\basProbe\('([^']+)'/g)) if (placed.get(m[1]) !== f) bad.push(`${f}: asProbe('${m[1]}') — the runner ${placed.has(m[1]) ? `runs that probe from ${placed.get(m[1])}` : 'has no probe of that name in a file'}`); }
  line(!bad.length && !unplaced.length && mains > 0, '(5) every check file with a main of its own that holds such a probe runs it through asProbe by its name: run as a file it stands where its family stands',
    bad.length ? few(bad, 5) : unplaced.length ? `the check cannot tell where the runner keeps: ${few(unplaced, 6)} (write the dispatch line with the name and the import on one line)` : `${mains} files with a main, of ${files.size} that hold such probes`); }

// (6) no name in the table is one the runner no longer has: each name as written in a row is, or begins, a probe the runner runs
{ const dead: string[] = [];
  for (const r of HOLDS) for (const e of r.names) if (!probeNames.some(n => named(e, n))) dead.push(`${e} (${r.list})`);
  line(!dead.length && listed.exitCode === 0 && probeNames.length > 100, '(6) every name in the table is a probe the runner has', listed.exitCode !== 0 ? `probes.ts list exited ${listed.exitCode}` : dead.length ? `the runner has none of: ${few(dead, 8)}` : `${probeNames.length} probes of ${known.size} names known, ${heldNames.length} of them in the table`); }

// (7) the runner's own run() holds a probe's words (probes.ts selftest-holds, in a child: a body that reads its words, on the stand of the first held name); and a
// file's part loop says a name it has not and runs nothing for it
{ const env: Record<string, string> = { PATH: process.env.PATH ?? '', HOME: process.env.HOME ?? '', ...(process.env.TPS_PROFILE ? { TPS_PROFILE: process.env.TPS_PROFILE } : {}) };
  if (BREAK) for (const h of rows) env[h.word] = '1';
  const p = Bun.spawnSync(['bun', 'run', resolve(HERE, 'probes.ts'), 'selftest-holds'], { env, stdout: 'pipe', stderr: 'pipe' }), out = p.stdout.toString(), said = /^- (PASS|FAIL) — (.*)$/m.exec(out);
  let ran = 0; const log = console.log; console.log = () => {}; let none: boolean | null; try { none = await asProbes({ 'holds-check-part': () => { ran++; return true; } }, ['no-such-part']); } finally { console.log = log; }
  line(p.exitCode === 0 && said?.[1] === 'PASS' && none === null && ran === 0 && exitCode(none) === 2, '(7) the runner\'s run() holds a probe\'s words by its name, and a part a file has not is said (exit 2), not passed over',
    `probes.ts selftest-holds: exit ${p.exitCode}${said ? ` · ${said[2].slice(0, 150)}` : ` · ${(p.stderr.toString() || out).trim().split('\n').slice(-1)[0]?.slice(0, 150)}`}`); }

console.log(fails ? `=== holds-check: ${fails} FAIL ===` : '=== holds-check: ALL PASS ===');
process.exit(fails ? 1 : 0);
