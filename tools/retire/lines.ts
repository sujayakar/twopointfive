// WHAT A FOLD DID TO THE CHECKS' LINES (the second half of retiring a switch: tools/retire/retire.ts, docs/agents/qa.md "Retiring a switch"). A check of a rule
// that ships as a switch has lines only the switch made possible: the stand's line, the off arm, the on arm measured against the off arm, a newer rule held
// off for the check's length. Once the switch is gone those lines fail, and they are found by running the checks before the fold and after it:
//   bun run tools/retire/lines.ts run <out dir> [--stand shipped|frozen] [--jobs 3] [--tree <dir>] [--pin rows.json] [--only names.txt] [--importers] <tools/qa/file.ts …>
//       every check those files hold, one process each (a `*-check.ts` with a main of its own as a program, any other by the names tools/qa/probes.ts
//       dispatches to it), output kept as <out dir>/<name>.txt and, beside it, where each PASS / FAIL line was printed from (<name>.trace.jsonl: the tools/
//       frames of the call, by a preload over console.log). `--tree` = another checkout of the repo (the tree before the fold). A command that already has
//       its output there is not run again.
//       `--pin` = the rows that file lists held at its values whatever the check writes (RETIRE_PIN, tools/qa/headless.ts): run on the tree BEFORE the
//       fold it says which lines need a retired path, and the fold's own run must then say the same lines — a line red after the fold and green under
//       the pins is the fold's own fault (in the game or in the check), not the check's staging. `--only` = a file of command names, the rest left out. `--importers` = also every check
//       that imports one of the files given (a changed rig reaches the checks that stand on it; the fold's report names only files it wrote in).
//   bun run tools/retire/lines.ts diff <before dir> <after dir> [--drop out.json] [--json out.json] [--tree <dir>]
//       pairs each command's lines before and after by their words (numbers set aside): the lines that went PASS → FAIL with the place each was printed from
//       (`--drop`: those places as a list for `retire.ts --drop-lines`), the red lines never said before (a staging that failed: never in that list), the
//       lines that went FAIL → PASS, the commands that died, and the checks that ran and
//       said nothing (every line gone: such a check goes whole). Exit 1 when a line broke, a red line is new, a command died or a check went silent.
// The files to give `run` are the tools the fold's report names (`retire.ts --report`: toolWrites, constLines, named). Run both on the page's stand
// (`--stand shipped`, the default): there the game is the same before and after, so a line that changed colour changed because of what the CHECK set.
// Then: retire.ts --drop-lines out.json --write, both typechecks, `run` again on the files it touched: no FAIL that was a PASS before.
import { appendFileSync, existsSync, mkdirSync, readFileSync, readdirSync, writeFileSync } from 'node:fs';
import { basename, join, resolve } from 'node:path';

type Trace = { v: string; text: string; frames: string[] };
async function main(): Promise<void> {
  const argv = process.argv.slice(2), mode = argv[0], opt = (n: string) => { const i = argv.indexOf(n); return i < 0 ? null : argv[i + 1] ?? null; };
  const flags = new Set(['--stand', '--jobs', '--tree', '--drop', '--json', '--pin', '--only']), rest = argv.slice(1).filter((a, i, all) => a !== '--importers' && !flags.has(a) && !flags.has(all[i - 1] ?? ''));
  const TREE = resolve(opt('--tree') ?? join(import.meta.dir, '../..')), SELF = resolve(import.meta.dir, 'lines.ts');
  for (const f of flags) if (argv.includes(f) && (opt(f) === null || opt(f)!.startsWith('--'))) { console.error(`${f} wants a value after it`); process.exit(2); }
  if (mode === 'run' && rest.length >= 2) await run(TREE, SELF, rest[0], rest.slice(1), opt('--stand') ?? 'shipped', Math.max(1, Number(opt('--jobs') ?? 3)), opt('--pin'), opt('--only'), argv.includes('--importers'));
  else if (mode === 'diff' && rest.length === 2) process.exit(diff(TREE, rest[0], rest[1], opt('--drop'), opt('--json')));
  else { console.error('usage: lines.ts run <out dir> [--stand shipped|frozen] [--jobs N] [--tree dir] [--pin rows.json] [--only names.txt] [--importers] <tools/qa/file.ts …>   |   lines.ts diff <before dir> <after dir> [--drop out.json] [--json out.json] [--tree dir]'); process.exit(2); }
}

/** the commands that run a check file: its probe names in probes.ts's dispatch (`on('name', …)` on a line that imports the file), else the file itself */
function commandsOf(tree: string, file: string, probes: string[]): string[][] {
  const b = basename(file), names: string[] = [];
  for (const l of probes) { if (!l.includes(`./${b}'`) && !l.includes(`/${b}\``) && !l.includes(`./${b}"`)) continue;
    // (the probe's own name is what its line runs it under — `await run('name', …)`; the first word of `on(…)` is the name too, except on a line that opens
    // a family, where it is the family: asking for that would run every probe of it)
    const own = [...l.matchAll(/await \w+\('([a-z0-9-]+)',/g)].map(m => m[1]);
    if (own.length) names.push(...own); else for (const m of l.matchAll(/\bon\(([^()]*)\)/g)) { const first = /'([^']+)'/.exec(m[1]); if (first) names.push(first[1]); } }
  const uniq = [...new Set(names)].filter(n => /^[a-z0-9-]+$/.test(n));
  if (uniq.length) return uniq.map(n => ['tools/qa/probes.ts', n]);
  return /-(check|probes)\.ts$/.test(b) && existsSync(join(tree, file)) ? [[file]] : [];
}
/** `files` and every check under tools/ that imports one of them, however far (a rig's change reaches the checks that stand on the rig: thresholdmenu-probes
 *  is imported by twenty). The harness, the play-through and the probe runner are imported by everything and are not followed. */
function withImporters(tree: string, files: string[]): string[] {
  const all: string[] = []; const walk = (d: string): void => { for (const e of readdirSync(join(tree, d), { withFileTypes: true })) { if (e.isDirectory()) { if (e.name !== 'fixture' && e.name !== 'node_modules') walk(join(d, e.name)); } else if (/\.(ts|cjs|js)$/.test(e.name)) all.push(join(d, e.name)); } }; walk('tools');
  const known = new Set(all), NOT = new Set(['tools/qa/headless.ts', 'tools/qa/playthrough.ts', 'tools/qa/probes.ts']);
  /** an import's words as a file of the tree: beside the importer (`./x.ts`, `../y`), or from the tree's root (`${ROOT}/tools/qa/x.ts`); the ending may be left off */
  const fileOf = (from: string, spec: string): string | null => { const root = /^\$\{[A-Za-z_]+\}\/(.*)$/.exec(spec), p = root ? root[1] : /^\.{1,2}\//.test(spec) ? join(from, '..', spec) : null; if (!p) return null;
    for (const c of [p, `${p}.ts`, `${p}.js`, `${p}.cjs`]) if (known.has(c)) return c; return null; };
  const deps = new Map<string, Set<string>>();
  // (…and a file named in a string any other way is taken as one it runs: `Bun.spawn(['bun', 'run', resolve(import.meta.dir, 'rig.ts')])`, `resolve(ROOT, 'tools/qa/rig.ts')`)
  for (const f of all) { const s = new Set<string>(), text = readFileSync(join(tree, f), 'utf8'); for (const m of text.matchAll(/(?:from|import|require)\s*\(?\s*['"`]([^'"`\n]+)['"`]/g)) { const d = fileOf(f, m[1]); if (d) s.add(d); }
    for (const m of text.matchAll(/['"`]((?:tools\/[\w./-]+|[\w-]+)\.(?:ts|cjs|js))['"`]/g)) { const d = known.has(m[1]) ? m[1] : fileOf(f, `./${m[1]}`); if (d && d !== f) s.add(d); } deps.set(f, s); }
  const seeds = files.map(f => join(f)), lost = seeds.filter(f => !known.has(f)), wide = seeds.filter(f => NOT.has(f)); if (lost.length) console.log(`--importers: not files of the tree's tools, so nothing is followed from them: ${lost.join(' ')}`);
  if (wide.length) console.log(`--importers: every check imports ${wide.join(', ')}: nothing is followed from there (give the family's own files, or run the gate)`);
  const got = new Set(seeds.filter(f => !NOT.has(f))); for (let grew = true; grew;) { grew = false; for (const [f, s] of deps) if (!got.has(f) && !NOT.has(f) && [...s].some(d => got.has(d))) { got.add(f); grew = true; } }
  return [...new Set([...seeds, ...[...got].filter(f => /-(check|probes)\.ts$/.test(f)).sort()])];
}
/** the name a command's output is kept under */
const tagOf = (cmd: string[]): string => cmd[cmd.length - 1].replace('tools/qa/', '').replace(/\//g, '_').replace(/\.ts$/, '');
async function run(tree: string, self: string, outDir: string, files: string[], stand: string, jobs: number, pin: string | null, only: string | null, importers: boolean): Promise<void> {
  mkdirSync(outDir, { recursive: true }); if (importers) files = withImporters(tree, files);
  const probes = existsSync(join(tree, 'tools/qa/probes.ts')) ? readFileSync(join(tree, 'tools/qa/probes.ts'), 'utf8').split('\n') : [], seen = new Set<string>(), queue: { file: string; cmd: string[] }[] = [], none: string[] = [];
  for (const f of files) { const cs = commandsOf(tree, f, probes); if (!cs.length) none.push(f); for (const cmd of cs) { const k = cmd.join(' '); if (!seen.has(k)) { seen.add(k); queue.push({ file: f, cmd }); } } }
  if (only) { const want = new Set(readFileSync(only, 'utf8').split(/\s+/).filter(Boolean)), all = queue.splice(0); queue.push(...all.filter(q => want.has(tagOf(q.cmd)))); const none = [...want].filter(w => !all.some(q => tagOf(q.cmd) === w)); if (none.length) console.log(`--only: no command of these files is named ${none.slice(0, 12).join(', ')}`); }
  console.log(`${queue.length} commands for ${files.length} files on the ${stand} stand${none.length ? ` · ${none.length} files hold no check of their own: ${none.slice(0, 8).join(' ')}` : ''}`);
  let next = 0, done = 0;
  const worker = async (): Promise<void> => { for (;;) { const job = queue[next++]; if (!job) return;
      const tag = tagOf(job.cmd), out = join(outDir, `${tag}.txt`), trace = join(outDir, `${tag}.trace.jsonl`);
      // (a command that has its output is not run again — where it was run the same way: a run asked for with a pin does not take lines run without one, nor the other way round)
      if (existsSync(out) && /\nexit -?\d+\n$/.test(readFileSync(out, 'utf8').slice(-40))) { const marked = existsSync(trace) && readFileSync(trace, 'utf8').includes('"v":"PIN"'), known = existsSync(join(outDir, 'not-pinned.txt')) && readFileSync(join(outDir, 'not-pinned.txt'), 'utf8').split('\n').some(l => l.split(' ')[0] === tag);
        if (pin ? marked || known : !marked) { done++; continue; } console.log(`  ${tag}: its lines there were run ${marked ? 'with' : 'without'} a pin: run again`); }
      writeFileSync(trace, '');
      const p = Bun.spawn(['bun', 'run', '--preload', self, ...job.cmd], { cwd: tree, env: { PATH: process.env.PATH ?? '', HOME: process.env.HOME ?? '', TPS_PROFILE: stand, LINES_TRACE_OUT: trace, ...(pin ? { RETIRE_PIN: resolve(pin) } : {}) }, stdout: 'pipe', stderr: 'pipe' });
      const timer = setTimeout(() => p.kill(), 1500 * 1000);
      const [o, e, code] = await Promise.all([new Response(p.stdout).text(), new Response(p.stderr).text(), p.exited]); clearTimeout(timer);
      writeFileSync(out, `# ${job.file} :: bun run ${job.cmd.join(' ')}\n${o}${e}\nexit ${code}\n`);
      // (the pin is the harness's: a check that never loads it wrote its rows as it liked; and the mark says how many of the list's rows were held)
      const mark = pin ? /"v":"PIN","text":"(\d+) of (\d+) rows held"/.exec(readFileSync(trace, 'utf8')) : null, part = !!mark && mark[1] !== mark[2], unpinned = !!pin && !mark;
      if (unpinned || part) appendFileSync(join(outDir, 'not-pinned.txt'), `${tag}${part ? ` ${mark![1]} of ${mark![2]} rows held` : ''}\n`);
      console.log(`  ${++done}/${queue.length} ${tag}: exit ${code}${unpinned ? ' · NOT PINNED (it never loaded tools/qa/headless.ts): its lines say nothing of the pins' : part ? ` · PINNED IN PART (${mark![1]} of ${mark![2]} rows: the rest are in no table the harness knows)` : ''}`); } };
  await Promise.all(Array.from({ length: jobs }, worker));
}

const LINE = /^\s*(?:-\s+)?(PASS|FAIL)\s*(?:—|-)?\s*(.*)$/;
const norm = (s: string): string => s.replace(/[-+]?\d+(?:\.\d+)?/g, '#').slice(0, 160);
function read(path: string): { head: string; lines: [string, string][]; code: number | null; text: string } {
  const text = readFileSync(path, 'utf8'), head = text.split('\n', 1)[0], rc = /exit (-?\d+)\s*$/.exec(text), lines: [string, string][] = [];
  for (const l of text.split('\n')) { const m = LINE.exec(l); if (m) lines.push([m[1], m[2].trim()]); }
  return { head, lines, code: rc ? Number(rc[1]) : null, text };
}
/** the longest run of characters two strings share */
function shared(a: string, b: string): number { let best = 0; const prev = new Uint16Array(b.length + 1), cur = new Uint16Array(b.length + 1);
  for (let i = 1; i <= a.length; i++) { for (let j = 1; j <= b.length; j++) { cur[j] = a.charCodeAt(i - 1) === b.charCodeAt(j - 1) ? prev[j - 1] + 1 : 0; if (cur[j] > best) best = cur[j]; } prev.set(cur); }
  return best; }
/** the pairs of two lists of words that line up, in order (longest common subsequence) */
function paired(a: string[], b: string[]): [number, number][] {
  const n = a.length, m = b.length, L: Uint16Array[] = Array.from({ length: n + 1 }, () => new Uint16Array(m + 1));
  for (let i = n - 1; i >= 0; i--) for (let j = m - 1; j >= 0; j--) L[i][j] = a[i] === b[j] ? L[i + 1][j + 1] + 1 : Math.max(L[i + 1][j], L[i][j + 1]);
  const out: [number, number][] = []; let i = 0, j = 0; while (i < n && j < m) { if (a[i] === b[j]) { out.push([i, j]); i++; j++; } else if (L[i + 1][j] >= L[i][j + 1]) i++; else j++; }
  return out;
}
function diff(tree: string, before: string, after: string, dropOut: string | null, jsonOut: string | null): number {
  const src = new Map<string, string[]>(), srcLine = (f: string, n: number): string => { let s = src.get(f); if (!s) { s = existsSync(join(tree, f)) ? readFileSync(join(tree, f), 'utf8').split('\n') : []; src.set(f, s); } return s[n - 1] ?? ''; };
  /** the frame of the call that printed a line. Innermost first: a line that writes both 'PASS' and 'FAIL' AND holds a run of the verdict's own words printed
   *  it straight (that statement is the place); one that writes both and holds none of its words is a helper that formats verdicts, and the place is the
   *  call of it — the nearest frame above it holding the verdict's words (a second helper may stand between), else the frame just above. With no such
   *  line at all: the innermost frame holding the verdict's words. */
  const place = (frames: string[], label: string): string | null => { const holds = (i: number): boolean => { const [f, ln] = frames[i].split(':'); return shared(srcLine(f, Number(ln)).slice(0, 600), label.slice(0, 200)) >= 14; };
    for (let i = 0; i < frames.length; i++) { const [f, ln] = frames[i].split(':'), text = srcLine(f, Number(ln)); if (!(text.includes("'PASS'") && text.includes("'FAIL'"))) continue;
      if (holds(i)) return frames[i];
      for (let j = i + 1; j < Math.min(frames.length, i + 4); j++) if (holds(j)) return frames[j];
      return i + 1 < frames.length ? frames[i + 1] : null; }
    for (let i = 0; i < frames.length; i++) if (holds(i)) return frames[i];
    return null; };
  const res = { commands: 0, broke: [] as { cmd: string; label: string; at: string | null }[], mended: [] as { cmd: string; label: string }[], fresh: [] as { cmd: string; label: string; at: string | null }[], died: [] as { cmd: string; exit: number | null; tail: string }[], silent: [] as string[], sameRed: 0, gone: 0 };
  /** did it stop on an error nothing caught? (an exit that is neither 0 nor 1; or a 1 with the words of one near its end and fewer lines than before: a
   *  check that fails its lines exits 1 too, and says all of them) */
  const threw = (A: ReturnType<typeof read>, B: ReturnType<typeof read>): boolean => (A.code !== 0 && A.code !== 1) || (A.code === 1 && A.lines.length < B.lines.length && /(^|\n)\s*(\w*Error: |error: )|\n\s+at \S[^\n]*:\d+:\d+\)?\n/.test(A.text.slice(-2000)));
  for (const f of readdirSync(after).sort()) { if (!f.endsWith('.txt') || !existsSync(join(before, f))) continue;
    const B = read(join(before, f)), A = read(join(after, f)); res.commands++;
    const tp = join(after, f.replace(/\.txt$/, '.trace.jsonl')), all: Trace[] = existsSync(tp) ? readFileSync(tp, 'utf8').split('\n').filter(Boolean).map(l => JSON.parse(l) as Trace) : [], traces = all.filter(t => t.v === 'FAIL'), used = new Set<number>();
    // (how many lines of this run each place printed: a place that printed several — a helper's own print, a loop that prints a line a case — is not ONE
    // line of the check, and taking it out would take the green ones with it)
    const printed = new Map<string, number>(); for (const t of all) { const p = place(t.frames, t.text.replace(/^\s*(?:-\s+)?(?:PASS|FAIL)\s*(?:—|-)?\s*/, '')); if (p) printed.set(p, (printed.get(p) ?? 0) + 1); }
    const at = (label: string): string | null => { const i = traces.findIndex((t, k) => !used.has(k) && (t.text.includes(label.slice(0, 120)) || norm(t.text).includes(norm(label).slice(0, 100)))); if (i < 0) return null; used.add(i); const p = place(traces[i].frames, label); return p && (printed.get(p) ?? 0) > 1 ? null : p; };
    if (threw(A, B) && !threw(B, A)) { res.died.push({ cmd: A.head, exit: A.code, tail: A.text.trim().split('\n').slice(-6, -1).join(' / ').slice(0, 240) }); continue; }
    if (!A.lines.length && B.lines.length) { res.silent.push(A.head); continue; }   // (it ran and said nothing: every line it had is gone — a check that holds nothing goes whole, `retire.ts --drop`)
    const pairs = paired(B.lines.map(x => norm(x[1])), A.lines.map(x => norm(x[1]))), inPair = new Set(pairs.map(p => p[1]));
    for (const [i, j] of pairs) { const b = B.lines[i], a = A.lines[j];
      if (b[0] === 'PASS' && a[0] === 'FAIL') res.broke.push({ cmd: A.head, label: a[1].slice(0, 200), at: at(a[1]) });
      else if (b[0] === 'FAIL' && a[0] === 'PASS') res.mended.push({ cmd: A.head, label: a[1].slice(0, 160) });
      else if (b[0] === 'FAIL') { res.sameRed++; at(a[1]); } }
    // a red line with no line of the same words before: red before too under other words (a digest, a name in it changed) where a red line that is also
    // unpaired before shares most of its words; else it broke
    const inPairB = new Set(pairs.map(p => p[0])), loose = B.lines.map((b, i) => ({ b, i })).filter(x => !inPairB.has(x.i)), usedB = new Set<number>();
    A.lines.forEach((a, j) => { if (inPair.has(j) || a[0] !== 'FAIL') return; const na = norm(a[1]);
      const alike = (x: string, y: string): boolean => { const n = Math.min(x.length, y.length); let p = 0, q = 0; while (p < n && x[p] === y[p]) p++; while (q < n - p && x[x.length - 1 - q] === y[y.length - 1 - q]) q++; return p + q >= 0.6 * n; };   // (the same line but for a word in the middle of it)
      const was = loose.find(x => !usedB.has(x.i) && x.b[0] === 'FAIL' && alike(norm(x.b[1]), na));
      // (… else it is a red line NEVER SAID BEFORE, in any colour: the check's staging failing — 'a door to stage it on'. Never listed to drop: taking a
      // staging line out leaves the check silent where it should be red. By hand.)
      // (… or green before with the same CLAIM — the words before ' — ', where the detail starts — a line that moved, or whose detail changed with its
      // colour: it broke. A shared opening is not enough: 'boot at the east door: the door gave' and 'boot at the east door: no door to stage it on' are two lines)
      const claim = (s: string): string => norm(s.split(' — ')[0]).trim(), ca = claim(a[1]), grn = was || !ca ? undefined : loose.find(x => !usedB.has(x.i) && x.b[0] === 'PASS' && claim(x.b[1]) === ca);
      if (was) { usedB.add(was.i); res.sameRed++; at(a[1]); } else if (grn) { usedB.add(grn.i); res.broke.push({ cmd: A.head, label: a[1].slice(0, 200), at: at(a[1]) }); } else res.fresh.push({ cmd: A.head, label: a[1].slice(0, 200), at: at(a[1]) }); });
    res.gone += loose.filter(x => !usedB.has(x.i) && x.b[0] === 'PASS').length;   // (said before, not said after: taken out with the switch, or never reached)
  }
  const places = [...new Set(res.broke.map(b => b.at).filter((x): x is string => !!x))].sort(), unplaced = res.broke.filter(b => !b.at);
  console.log(`${res.commands} commands · ${res.broke.length} lines PASS → FAIL (${places.length} places${unplaced.length ? `, ${unplaced.length} NOT PLACED: by hand` : ''}) · ${res.fresh.length} red lines never said before (a staging that failed: by hand) · ${res.mended.length} FAIL → PASS · ${res.sameRed} red before and after · ${res.gone} said before and not after · ${res.died.length} commands died${res.silent.length ? ` · ${res.silent.length} say nothing any more (drop them whole)` : ''}`);
  for (const c of res.silent) console.log(`  SILENT ${c.slice(0, 140)}`);
  for (const d of res.died) console.log(`  DIED ${d.cmd.slice(0, 110)} exit ${d.exit} | ${d.tail}`);
  for (const b of res.broke) console.log(`  BROKE ${b.at ?? '(not placed)'} | ${b.label.slice(0, 120)}`);
  for (const b of res.fresh) console.log(`  NEW RED ${b.at ?? '(not placed)'} | ${b.label.slice(0, 120)}`);
  if (jsonOut) writeFileSync(jsonOut, JSON.stringify(res, null, 1));
  if (dropOut) writeFileSync(dropOut, JSON.stringify(places, null, 1));
  return res.broke.length || res.fresh.length || res.died.length || res.silent.length ? 1 : 0;
}

// ---- the preload (this same file loaded in front of a check, LINES_TRACE_OUT set): every PASS / FAIL line is also written to that file with its call's frames
if (import.meta.main) await main();
else if (process.env.LINES_TRACE_OUT) {
  const out = process.env.LINES_TRACE_OUT, real = console.log.bind(console), HERE = process.cwd();   // (the check runs in its tree: the folder as the system names it, which is how a stack frame names its files — a tree reached through a link too)
  console.log = (...a: unknown[]): void => {
    real(...a);
    try {
      for (const ln of a.map(x => typeof x === 'string' ? x : String(x)).join(' ').split('\n')) { const m = /^\s*(?:-\s+)?(PASS|FAIL)\b/.exec(ln); if (!m) continue;
        // (a frame's file as it is named from the tree's root: a checkout under some other `src/` must not be read as the game's)
        const root = HERE.replace(/\/+$/, '') + '/';
        const frames = (new Error().stack ?? '').split('\n').map(f => /\(?(\/[^\s()]+?\.ts):(\d+):(\d+)\)?\s*$/.exec(f)).filter((f): f is RegExpExecArray => !!f && f[1].startsWith(root) && /^(tools|src)\//.test(f[1].slice(root.length)) && !f[1].endsWith('/tools/retire/lines.ts')).map(f => `${f[1].slice(root.length)}:${f[2]}:${f[3]}`);
        appendFileSync(out, JSON.stringify({ v: m[1], text: ln.trim().slice(0, 240), frames }) + '\n'); }
    } catch { /* the check's own output is what matters */ }
  };
}
