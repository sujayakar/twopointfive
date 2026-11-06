#!/usr/bin/env bun
// THE PAGE'S STAND AND THE OLD ONE, CHECKED WITHOUT A PAGE (src/host/profile.ts) — `bun run tools/qa/shipped-check.ts`, and `probes.ts shipped-profile` (family shipped):
//   1. the harness stood up frozen and stood up shipped (TPS_PROFILE=shipped) differ in exactly the rows FROZEN_HOLDS names — a pin added to the harness and not
//      to that list, or a row listed that no longer differs, is red, and says which;
//   2. the same on a mission's own stand (a squad of four started on a level file), so a rule that only a staging writes is seen too;
//   3. the play-through's own as-shipped lines (playthrough.ts: the doors, the chaos) write what the shipped stand already has — under the shipped profile the
//      play-through adds nothing of its own;
//   4. the stand's NAME: no word and 'shipped' are the page's stand, 'frozen' the old one, any other word is refused (src/host/standEnv.ts standOf, the
//      one rule), and the harness's own reading of the environment (profile.ts harnessProfile) says the same;
//   5. every tool that reads a `.TPS_PROFILE` loads tools/qa/stand.ts before its own lines, or names its stand itself first (earlyReaders below);
//   6. no tool asks for a rule OFF by leaving its word out of an environment (wordsLeftOut below): with no word a game stands as the page opens it;
//   7. every bun child a tool starts with Bun.spawn is given an environment by name (bareChildren below): with none it gets the one the tool STARTED with.
// STAND_BREAK=1 = the red path of 4, 5, 6 and 7: no word is expected to be the old stand, stand.ts is forgotten (every reader is early), and the sample
// lines of each slip are read with the tools' own.
// Each stand is its own bun (the pins run as headless.ts loads), with every word either stand reads cleared from its environment first.
// That the SHIPPED stand is the page's is the other half, and needs the page: `node tools/qa/shipped-page.cjs` (software adapter, ~3 minutes a boot).
// Exit 0 = all pass, 1 = a check failed.
import { dirname, join, resolve } from 'node:path';
import { existsSync, mkdtempSync, readdirSync, readFileSync, rmSync, statSync } from 'node:fs';
import { tmpdir } from 'node:os';
import { dumpDiff, type StandDump } from './stand-dump.ts';
import './stand.ts';   // (the stand's word, written down before this file reads it)
import { callEnd } from './call-text.ts';
import { standOf } from '../../src/host/standEnv.ts';
const ROOT = resolve(import.meta.dir, '../..'), BREAK = process.env.STAND_BREAK === '1';
const MISSION = 'levelFile=compound.json&roomclear&squad4';

/** THE TOOLS THAT READ THE STAND'S WORD BEFORE IT IS WRITTEN: a tool's file that reads a `.TPS_PROFILE` (off `process.env` or off any other name for it) and
 *  neither loads stand.ts first (a chain of STATIC imports to it: those load before the file's own lines; headless.ts is such a chain; `import type` loads
 *  nothing) nor names its stand itself before it reads (its first mention is a write: `… ??= 'shipped'`, `… = 'shipped'`). Such a tool, run with no word,
 *  would call the page's stand "frozen". A .cjs page driver cannot load stand.ts, so one that reads the word is named whatever it imports. Also named: a
 *  file that DOES load stand.ts and then writes `??= …'frozen'…`, which can no longer take. STAND_BREAK=1 forgets stand.ts, so every reader is one.
 *  WHAT IT CANNOT SEE: it reads text. A tool that clears the word for a child, or gives a child an environment without it, asks for the page's stand
 *  and is not seen here: a tool that wants the old stand of a child says 'frozen'. (A RULE's word left out is the next function's.) */
export function earlyReaders(root = ROOT): string[] {
  const code = (f: string) => readFileSync(f, 'utf8').split('\n').filter(l => !/^\s*(\/\/|\*|\/\*)/.test(l)).join('\n');
  const imports = (f: string): string[] => [...code(f).matchAll(/^\s*(?:import|export)\s(?!type\b)(?:[^;'"]*?\sfrom\s+)?['"](\.[^'"]+)['"]/gm)].map(m => resolve(dirname(f), m[1])).filter(existsSync);
  const STANDFILE = resolve(root, 'tools/qa/stand.ts'), memo = new Map<string, boolean>();
  const loads = (f: string, seen = new Set<string>()): boolean => { if (f === STANDFILE) return !BREAK; if (!f.endsWith('.ts')) return false; if (memo.has(f)) return memo.get(f)!; if (seen.has(f)) return false; seen.add(f); const r = imports(f).some(i => loads(i, seen)); memo.set(f, r); return r; };
  const READ = /[\w\])]\??\.TPS_PROFILE\b|\[['"]TPS_PROFILE['"]\]/, NAMED = /^[\w\])]\??\.TPS_PROFILE\s*(\?\?=|=(?!=))/, LATE = /\.TPS_PROFILE\s*\?\?=[^;\n]*'frozen'/;
  return toolFiles(root).filter(f => { if (f === STANDFILE) return false; const c = code(f), at = c.search(READ); if (at < 0) return false;
    return loads(f) ? LATE.test(c) : !NAMED.test(c.slice(at)); }).map(f => f.slice(root.length + 1));
}

async function stand(profile: 'frozen' | 'shipped', query: string, dir: string): Promise<StandDump> {
  const out = resolve(dir, `${profile}-${query ? 'mission' : 'plain'}.json`);
  const pr = Bun.spawn(['bun', 'run', resolve(import.meta.dir, 'stand-dump.ts'), '--profile', profile, '--query', query, '--out', out], { stdout: 'ignore', stderr: 'pipe', env: process.env });   // (--profile clears the invoking shell's stand words in the child)
  const [err, code] = await Promise.all([new Response(pr.stderr).text(), pr.exited]);
  if (code !== 0) throw new Error(`stand-dump (${profile}${query ? `, ${query}` : ''}) failed: ${err.slice(-600)}`);
  return JSON.parse(readFileSync(out, 'utf8')) as StandDump;
}

/** every file of the tools a lint here reads, sorted (a fixture folder is data: a tree a tool is tried on) */
function toolFiles(root: string): string[] {
  const walk = (d: string, out: string[] = []): string[] => { for (const n of readdirSync(d)) { const p = join(d, n); if (statSync(p).isDirectory()) { if (!/^(fixtures?|node_modules)$/.test(n)) walk(p, out); } else if (/\.(ts|cjs|mjs|js)$/.test(n)) out.push(p); } return out; };
  return walk(resolve(root, 'tools')).sort();
}

/** A RULE ASKED OFF BY LEAVING ITS WORD OUT. Before a tool with no word stood the page's game, a stand word left out of an environment was the rule off, and
 *  tools asked for an 'off', 'old' or 'before' arm that way. It is the rule ON now, and such an arm plays the rule it is the control of while every line
 *  that only compares words still passes (snap-probes' control runs and its --turn 0, entry-strip's 'before', the doorpath family's own red path, the picks
 *  family's plain mode: found by hand and by two reviews, 2026-11-06). THIS IS A NET WITH HOLES, not the class closed: it names a line of a tool that
 *    (a) gives a stand word a value with `undefined` in it       { ...PAGE, TPS_TURN: undefined } · TPS_X: on ? '1' : undefined · env.TPS_X = off ? undefined : w
 *    (b) puts one in on a condition and nothing otherwise         ...(old ? {} : { TPS_DOORPATH: '1' }) · on ? { TPS_X: '1' } : undefined · ...(on && { TPS_X: '1' })
 *    (c) deletes one from an environment not written `process.env`   delete env.TPS_DOORPATH
 *    (d) deletes over a list that names one, in one statement     for (const k of ['TPS_TURN', …]) … delete E[k]
 *  A word with no prefix of its own (SNEAK, PAIR, ONEWAY: also the names of knob tables) is read only on a line whose code says `env`.
 *  LET THROUGH: a line that SETS the stand itself (`TPS_PROFILE:` or `=`: a word left out there is that stand's own default, by a writer who named the
 *  stand); a put-back of what the tool found (`if (w === undefined) delete E.X; else E.X = w`: that statement alone is taken out of the line before it is
 *  read); and a line that carries the words `word left out on purpose:` and its reason. The mend is the word '0' (tools/qa/stand.ts childEnv).
 *  WHAT IT CANNOT SEE: an environment built over several lines (`const E = { ...process.env }; if (on) E.X = '1'`), a word dropped by a helper or a list
 *  kept in a name, a computed key, `delete process.env.X` before a stand-up in the tool's own process (fifteen such lines today, each read and meant), a
 *  comment that is not at the line's start or after a space and `//`. What closes the class is a run held to the rule its game had (snap-probes `turnOn`).
 *  `files`: [where, text] pairs. Returns where each named line is. */
export function wordsLeftOut(words: readonly string[], files: readonly (readonly [string, string])[]): string[] {
  const esc = (w: string) => w.replace(/[.*+?^${}()|[\]\\]/g, '\\$&'), own = words.filter(w => w !== 'TPS_PROFILE');
  const sure = own.filter(w => /^[A-Z]+_/.test(w)).map(esc).join('|'), bare = own.filter(w => !/^[A-Z]+_/.test(w)).map(esc).join('|');
  const shapes = (alt: string): RegExp[] => alt ? [
    new RegExp(`(?<![.\\w])(?:${alt})\\b['"]?\\]?\\s*:\\s*[^,}]*\\bundefined\\b`), new RegExp(`\\.(?:${alt})\\s*=(?!=)[^;]*\\bundefined\\b`),
    new RegExp(`\\?\\s*(?:\\{\\s*\\}|undefined|null)\\s*:\\s*\\{[^}]*\\b(?:${alt})\\b|\\?\\s*\\{[^}]*\\b(?:${alt})\\b[^}]*\\}\\s*:\\s*(?:\\{\\s*\\}|undefined|null)|&&\\s*\\{[^}]*\\b(?:${alt})\\b`),
    new RegExp(`\\bdelete\\s+(?!process\\.env\\b)[\\w.]+(?:\\.|\\[['"])(?:${alt})\\b`),
    new RegExp(`\\[[^\\]]*['"](?:${alt})['"][^\\]]*\\][^;]*\\bdelete\\s+\\w+\\[\\w+\\]`)] : [];
  const SURE = shapes(sure), BARE = shapes(bare), PUTBACK = /\bif\s*\([^()]*===\s*undefined\)\s*delete\s+([\w.]+)(\.\w+|\[['"]\w+['"]\]);\s*else\s+\1\2\s*=\s*[^;]+;?/g;
  const named: string[] = [];
  for (const [at, text] of files) text.split('\n').forEach((ln, i) => {
    if (/word left out on purpose:/.test(ln) || /\bTPS_PROFILE\b['"]?\s*(?::|=(?!=))/.test(ln)) return;
    const code = ln.replace(/^\s*(?:\/\/|\*|\/\*).*$/, '').replace(/\s\/\/.*$/, '').replace(PUTBACK, '');
    if (SURE.some(re => re.test(code)) || (/\benv\b/i.test(code) && BARE.some(re => re.test(code)))) named.push(`${at}:${i + 1}`); });
  return named;
}
/** the lint's own samples: the slips as they were written (each must be named) and the lines it lets through (none may be) */
const SLIPS = ["const OFF = { ...PAGE, TPS_TURN: undefined }, rs = await snapRuns(ALL, OFF);", "const rs = await snapRuns(runs, { ...PAGE, TPS_TURN: turn === '0' ? undefined : turn });",
  "if (on) env.TPS_DOORPATH = process.env.TPS_DOORPATH ?? '1'; else delete env.TPS_DOORPATH;", "const watched = () => runJobs(RUNS.map(s => [s, { PT_PATHS: '1', ...(OLD() ? {} : { TPS_DOORPATH: '1' }) }]));",
  "const env = { ...process.env, PT_DIGEST: '1', ...(plain ? {} : { GLASSFX: 'on', TPS_MOVES: '1', TPS_TURN: '1' }) };", "for (const k of ['TPS_DOORPATH', 'TPS_TURN']) if (!(k in env)) delete E[k];",
  "const env = { ...base, ...(on && { TPS_RAILS: '1' }) };", "env.TPS_TURN = off ? undefined : word;", "const env = { ...process.env, ONEWAY: undefined };"];
const MEANT = ["const env = { ...process.env, TPS_PROFILE: 'shipped' }; delete env.TPS_WALLRUN;", "try { return await fn(); } finally { if (was === undefined) delete env.TPS_FALLIN; else env.TPS_FALLIN = was; }",
  "for (const k of ['TPS_DOORPATH', 'TPS_TURN']) if (!(k in env)) delete E[k];   // (word left out on purpose: a job that wants one off says '0')", "const old = snap(); delete old.tables.SNEAK; J.applyKnobs(g, old);",
  "const w = had ? process.env.TPS_TURN : undefined;", "const env = childEnv({ PT_PATHS: 'rec', TPS_DOORPATH: on ? '1' : false });", "// was: { ...PAGE, TPS_TURN: undefined }"];

/** A BUN CHILD GIVEN NO ENVIRONMENT. `Bun.spawn` and `Bun.spawnSync` with no `env` (or `env: undefined`) hand a child the environment the process STARTED
 *  with, not `process.env` as it stands (bun 1.4; node's child_process and `Bun.$` hand on the live one). Every word a tool writes after it starts is then
 *  lost on the way: a hold the runner gave a probe (holds.ts: the word '0'), a word a check sets for its arm, the stand written down by stand.ts, and
 *  `QA_STAND_SAID`, without which a child on the old stand says the stand's banner again on its own stderr (picks-boot read that banner as seven console
 *  errors, 2026-11-06; fifty-one calls stood so). The mend is `env: process.env` in the call, or the environment the child is owed.
 *  NAMED: a call with a command written in it that begins 'bun', `process.execPath` or `process.argv[0]` (the first key of an options object or not), and
 *  no `env` key of its own with a value: `env: undefined`, an `env` that is `undefined` on one side of a condition, and a `.env` read off another name do
 *  not count. The call is read whole, bracket by bracket, over as many lines as it takes (call-text.ts).
 *  THIS IS A NET WITH HOLES: it reads the tools' text. WHAT IT CANNOT SEE: a command or its options kept in a name (`Bun.spawn(cmd, opts)`), a child
 *  started by a helper or through another program (`timeout 60 bun …`, a shell line), a call that runs across a regex literal with a quote or a bracket
 *  in it. Other programs (git, a browser) read no word of ours and are not named, and nothing outside tools/ is read (src/ starts a `bun build` of its
 *  own, which reads none either). One helper that every tool starts its children through would close the class at one depth; that is 126 calls in a
 *  hundred files, left for when the tools are thinned.
 *  `files`: [where, text] pairs. Returns where each named call begins. */
export function bareChildren(files: readonly (readonly [string, string])[]): string[] {
  const named: string[] = [], START = /\bBun\.spawn(?:Sync)?\(/g, BUN = /\[\s*(?:['"`]bun['"`]|process\.execPath|process\.argv\[0\])/;
  const given = (call: string): boolean => {
    if (/[{,]\s*env\s*[,}]/.test(call)) return true;   // (shorthand: `{ stdout: 'pipe', env }`)
    for (const m of call.matchAll(/(?<![.\w$])env\s*:/g)) { const rest = call.slice(m.index! + m[0].length), end = rest.search(/,\s*[\w$'"]+\s*:|\}\s*\)\s*$/), value = end < 0 ? rest : rest.slice(0, end); if (!/\bundefined\b/.test(value)) return true; }
    return false; };
  for (const [at, src] of files) for (const m of src.matchAll(START)) { const call = src.slice(m.index!, callEnd(src, m.index! + m[0].length));
    if (BUN.test(call) && !given(call)) named.push(`${at}:${src.slice(0, m.index!).split('\n').length}`); }
  return named;
}
/** that lint's samples, as the calls stood (each must be named) and as a child is given its environment (none may be) */
const BARE = ["const p = Bun.spawn(['bun', 'run', import.meta.path, '--boot', query], { cwd: ROOT, stdout: 'pipe', stderr: 'pipe' });", "const run = Bun.spawnSync(['bun', 'run', join(import.meta.dir, 'capture-read.ts'), file]), outDir = join(dir, 'cap-check');",
  "const run = (f: string) => Bun.spawnSync({ cmd: [process.execPath, 'run', join(ROOT, 'tools/qa/level-lint.ts'), f, '--quiet'], cwd: ROOT, stdout: 'pipe' });", "const p = Bun.spawn(['bun', 'run', resolve(ROOT, 'tools/qa/playthrough.ts'), '--one', `${E.id}:${seed}`, '--out', out],\n  { stdout: 'ignore', stderr: 'pipe' });",
  "const p = Bun.spawn(['bun', '-e', `console.log(process.env.TPS_TURN)`], { stdout: 'pipe' });", "const p = Bun.spawn(['bun', 'run', f], { stdout: 'pipe', env: undefined });", "const p = Bun.spawn(['bun', 'run', f], { env: on ? E : undefined, stdout: 'pipe' });",
  "const p = Bun.spawn(['bun', 'run', f, '--tag', `${job.env}`], { stdout: 'pipe' });", "const p = Bun.spawnSync({ cwd: ROOT, cmd: [`bun`, 'run', f] });", "const p = Bun.spawn([process.argv[0], 'run', f],   // the child's own run\n  { stdout: 'pipe' }); const q = Bun.spawn(['git', 'status'], { env: E });"];
const GIVEN = ["const p = Bun.spawn(['bun', 'run', import.meta.path, '--boot', query], { cwd: ROOT, stdout: 'pipe', stderr: 'pipe', env: process.env });", "const run = Bun.spawnSync(['bun', 'run', join(import.meta.dir, 'capture-read.ts'), file], { env: process.env });",
  "const p = Bun.spawn(['bun', 'run', resolve(ROOT, 'tools/qa/playthrough.ts'), '--one', `${E.id}:${seed}`],\n  { stdout: 'ignore', env: { ...process.env, TPS_SNOW: '1' } });", "const p = Bun.spawn(['bun', 'run', file], { stdout: 'pipe', env });",
  "const p = Bun.spawn(['bun', 'run', file], { env: { ...process.env, X: '1' }, stdout: 'pipe' });",
  "const git = (...a: string[]) => new TextDecoder().decode(Bun.spawnSync(['git', '-C', dir, ...a]).stdout).trim();", "const c = D.spawn('chunk', makeBox({ c: p, h: [0.2, 0.015, 0.14] }), [0, -4, 0], [0, 0, 0], PLAYER_ID);"];

export async function shippedCheck(): Promise<boolean> {
  const { FROZEN_HOLDS, STAND_ENV, harnessProfile } = await import(`${ROOT}/src/host/profile.ts`) as { FROZEN_HOLDS: Record<string, string>; STAND_ENV: readonly string[]; harnessProfile: () => string };
  let ok = true; const check = (pass: boolean, what: string) => { console.log(`- ${pass ? 'PASS' : 'FAIL'} — ${what}`); if (!pass) ok = false; };
  const dir = mkdtempSync(resolve(tmpdir(), 'shipped-check-'));
  try {
    const listed = Object.keys(FROZEN_HOLDS).sort();
    for (const [name, query] of [['the plain stand', ''], ['a mission staged (the compound, four)', MISSION]] as const) {
      const [fr, sh] = await Promise.all([stand('frozen', query, dir), stand('shipped', query, dir)]);
      const rows = dumpDiff(fr, sh), got = rows.map(r => r[0]).sort();
      const unlisted = got.filter(k => !listed.includes(k)), stale = listed.filter(k => !got.includes(k));
      check(unlisted.length === 0 && stale.length === 0, `${name}: frozen and shipped differ in exactly FROZEN_HOLDS' ${listed.length} rows`
        + (unlisted.length ? ` — the harness holds these and the list does not name them: ${unlisted.map(k => { const r = rows.find(x => x[0] === k)!; return `${k} (${JSON.stringify(r[1])} frozen, ${JSON.stringify(r[2])} shipped)`; }).join(' · ')}` : '')
        + (stale.length ? ` — listed, and the two stands agree on them: ${stale.join(' · ')}` : ''));
      if (!query) {
        const T = sh.tables as Record<string, Record<string, unknown>>, tune = sh.tune as Record<string, unknown>;
        const want: [string, unknown, unknown][] = [['DOORWAY.oneWay', T.DOORWAY?.oneWay, true], ['DOORWAY.exitsOut', T.DOORWAY?.exitsOut, false], ['DOORWAY.frameHolds', T.DOORWAY?.frameHolds, true],
          ['DOORSTAY.on', T.DOORSTAY?.on, true], ['DOORMANNERS.on', T.DOORMANNERS?.on, true], ['DOORBLAST.on', T.DOORBLAST?.on, true], ['tune.chaos', tune.chaos, 0.5]];
        const off = want.filter(w => w[1] !== w[2]);
        check(off.length === 0, `the play-through's own as-shipped lines write what the shipped stand already has (${want.length} rows)${off.length ? ` — not so: ${off.map(w => `${w[0]} is ${JSON.stringify(w[1])}, the play-through writes ${JSON.stringify(w[2])}`).join(' · ')}` : ''}`);
      }
    }
    { // every word a harness pin answers to is on the list the comparisons clear (src/host/standEnv.ts) — else a stand compared here is the invoking shell's
      const words = new Set<string>(); for (const f of ['src/host/standup.ts', 'tools/qa/headless.ts', 'tools/qa/playthrough.ts']) for (const m of readFileSync(resolve(ROOT, f), 'utf8').matchAll(/pinned\('([A-Z_]+)'\)/g)) words.add(m[1]);
      const missing = [...words].filter(w => !STAND_ENV.includes(w));
      check(words.size > 0 && missing.length === 0, `every word a harness pin answers to (${words.size}: ${[...words].sort().join(' ')}) is in STAND_ENV${missing.length ? ` — missing: ${missing.join(' ')}` : ''}`); }
    { // the stand's name: the one rule (standEnv.ts standOf), and the harness's reading of the environment is that rule
      const env = process.env, was = env.TPS_PROFILE, said = (f: () => string): string => { try { return f(); } catch { return 'refused'; } };
      const rows = (['', 'shipped', 'frozen', 'Shipped', 'froze', '1'] as const).map(w => { if (w) env.TPS_PROFILE = w; else delete env.TPS_PROFILE; return [w || '(no word)', said(() => standOf(w || undefined)), said(() => harnessProfile())] as const; });
      if (was === undefined) delete env.TPS_PROFILE; else env.TPS_PROFILE = was;
      const want = [BREAK ? 'frozen' : 'shipped', 'shipped', 'frozen', 'refused', 'refused', 'refused'], off = rows.filter((r, i) => r[1] !== want[i] || r[2] !== want[i]);
      check(off.length === 0, `no word and 'shipped' are one stand, the page's; 'frozen' is the old one; any other word is refused${off.length ? ` — not so: ${off.map(r => `${r[0]}: the rule says ${r[1]}, the harness ${r[2]}`).join(' · ')}` : ''}`); }
    { const early = earlyReaders();
      check(early.length === 0, `every tool that reads the stand's word loads stand.ts before its own lines (a static import, or headless.ts), or names its stand first${early.length ? ` — not so (${early.length}): ${early.join(' · ')}` : ''}`); }
    { // (the samples first: a lint that names nothing is green for the wrong reason. STAND_BREAK: the slips are read as tools' lines too — the last line must fail)
      const one = (lines: string[]) => lines.map((l, i) => wordsLeftOut(STAND_ENV, [[String(i), l]]).length > 0), slips = one(SLIPS), meant = one(MEANT);
      const missed = SLIPS.filter((_, i) => !slips[i]), wrong = MEANT.filter((_, i) => meant[i]);
      check(missed.length === 0 && wrong.length === 0, `the lint of words left out names each of its ${SLIPS.length} sample slips and none of the ${MEANT.length} lines it lets through${missed.length ? ` — NOT NAMED: ${missed.join(' ⏎ ')}` : ''}${wrong.length ? ` — WRONGLY NAMED: ${wrong.join(' ⏎ ')}` : ''}`);
      const self = resolve(import.meta.dir, 'shipped-check.ts'), files = toolFiles(ROOT).filter(f => f !== self).map(f => [f.slice(ROOT.length + 1), readFileSync(f, 'utf8')] as const);   // (this file holds the samples)
      const out = wordsLeftOut(STAND_ENV, BREAK ? [...files, ['(STAND_BREAK: the sample slips)', SLIPS.join('\n')] as const] : files);
      check(out.length === 0, `no tool asks for a rule off by leaving its word out of an environment, in the four spellings read for: off is the word '0' (${files.length} files, ${STAND_ENV.length - 1} words)${out.length ? ` — not so (${out.length}): ${out.join(' · ')}` : ''}`);
      // (the same of a bun child given no environment: the samples, then the tools; STAND_BREAK reads the sample calls as a tool's)
      const got = (calls: string[]) => calls.map((c, i) => bareChildren([[String(i), c]]).length > 0), bare = got(BARE), given = got(GIVEN);
      const missedB = BARE.filter((_, i) => !bare[i]), wrongB = GIVEN.filter((_, i) => given[i]);
      check(missedB.length === 0 && wrongB.length === 0, `the lint of bun children given no environment names each of its ${BARE.length} sample calls and none of the ${GIVEN.length} it lets through${missedB.length ? ` — NOT NAMED: ${missedB.join(' ⏎ ')}` : ''}${wrongB.length ? ` — WRONGLY NAMED: ${wrongB.join(' ⏎ ')}` : ''}`);
      const own = [self.slice(ROOT.length + 1), readFileSync(self, 'utf8').replace(/^const (?:BARE|GIVEN|SLIPS|MEANT) = \[[\s\S]*?\];$/gm, '')] as const;   // (this file's own children are read too, its samples cut out)
      const outB = bareChildren(BREAK ? [...files, own, ['(STAND_BREAK: the sample calls)', BARE.join('\n')] as const] : [...files, own]);
      check(outB.length === 0, `every bun child a tool starts with Bun.spawn is given an environment by name, in the spellings read for: with none it gets the one the tool started with, and every word written since is lost (${files.length + 1} files)${outB.length ? ` — not so (${outB.length}): ${outB.join(' · ')}` : ''}`); }
    console.log(`  (the rows, and the word that turns each on for a frozen run:\n${listed.map(k => `     ${k.padEnd(24)} ${FROZEN_HOLDS[k]}`).join('\n')})`);
  } finally { rmSync(dir, { recursive: true, force: true }); }
  return ok;
}

if (import.meta.main) process.exit(await shippedCheck() ? 0 : 1);
