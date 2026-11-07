// WHAT A FOLD DID TO THE CHECKS' LINES (the second half of retiring a switch: tools/retire/retire.ts, docs/agents/qa.md "Retiring a switch"). A check of a rule
// that ships as a switch has lines only the switch made possible: the stand's line, the off arm, the on arm measured against the off arm, a newer rule held
// off for the check's length. Once the switch is gone those lines fail, and they are found by running the checks before the fold and after it:
//   bun run tools/retire/lines.ts run <out dir> [--stand shipped|frozen] [--jobs 3] [--tree <dir>] <tools/qa/file.ts …>
//       every check those files hold, one process each (a `*-check.ts` with a main of its own as a program, any other by the names tools/qa/probes.ts
//       dispatches to it), output kept as <out dir>/<name>.txt and, beside it, where each PASS / FAIL line was printed from (<name>.trace.jsonl: the tools/
//       frames of the call, by a preload over console.log). `--tree` = another checkout of the repo (the tree before the fold). A command that already has
//       its output there is not run again.
//   bun run tools/retire/lines.ts diff <before dir> <after dir> [--drop out.json] [--json out.json] [--tree <dir>]
//       pairs each command's lines before and after by their words (numbers set aside): the lines that went PASS → FAIL with the place each was printed from
//       (`--drop`: those places as a list for `retire.ts --drop-lines`), the lines that went FAIL → PASS, the commands that died, and the checks that ran and
//       said nothing (every line gone: such a check goes whole). Exit 1 when there is any of the first, third or fourth.
// The files to give `run` are the tools the fold's report names (`retire.ts --report`: toolWrites, constLines, named). Run both on the page's stand
// (`--stand shipped`, the default): there the game is the same before and after, so a line that changed colour changed because of what the CHECK set.
// Then: retire.ts --drop-lines out.json --write, both typechecks, `run` again on the files it touched: no FAIL that was a PASS before.
import { appendFileSync, existsSync, mkdirSync, readFileSync, readdirSync, writeFileSync } from 'node:fs';
import { basename, join, resolve } from 'node:path';

type Trace = { v: string; text: string; frames: string[] };
async function main(): Promise<void> {
  const argv = process.argv.slice(2), mode = argv[0], opt = (n: string) => { const i = argv.indexOf(n); return i < 0 ? null : argv[i + 1] ?? null; };
  const flags = new Set(['--stand', '--jobs', '--tree', '--drop', '--json']), rest = argv.slice(1).filter((a, i, all) => !flags.has(a) && !flags.has(all[i - 1] ?? ''));
  const TREE = resolve(opt('--tree') ?? join(import.meta.dir, '../..')), SELF = resolve(import.meta.dir, 'lines.ts');
  if (mode === 'run' && rest.length >= 2) await run(TREE, SELF, rest[0], rest.slice(1), opt('--stand') ?? 'shipped', Math.max(1, Number(opt('--jobs') ?? 3)));
  else if (mode === 'diff' && rest.length === 2) process.exit(diff(TREE, rest[0], rest[1], opt('--drop'), opt('--json')));
  else { console.error('usage: lines.ts run <out dir> [--stand shipped|frozen] [--jobs N] [--tree dir] <tools/qa/file.ts …>   |   lines.ts diff <before dir> <after dir> [--drop out.json] [--json out.json] [--tree dir]'); process.exit(2); }
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
async function run(tree: string, self: string, outDir: string, files: string[], stand: string, jobs: number): Promise<void> {
  mkdirSync(outDir, { recursive: true });
  const probes = existsSync(join(tree, 'tools/qa/probes.ts')) ? readFileSync(join(tree, 'tools/qa/probes.ts'), 'utf8').split('\n') : [], seen = new Set<string>(), queue: { file: string; cmd: string[] }[] = [], none: string[] = [];
  for (const f of files) { const cs = commandsOf(tree, f, probes); if (!cs.length) none.push(f); for (const cmd of cs) { const k = cmd.join(' '); if (!seen.has(k)) { seen.add(k); queue.push({ file: f, cmd }); } } }
  console.log(`${queue.length} commands for ${files.length} files on the ${stand} stand${none.length ? ` · ${none.length} files hold no check of their own: ${none.slice(0, 8).join(' ')}` : ''}`);
  let next = 0, done = 0;
  const worker = async (): Promise<void> => { for (;;) { const job = queue[next++]; if (!job) return;
      const tag = job.cmd[job.cmd.length - 1].replace('tools/qa/', '').replace(/\//g, '_').replace(/\.ts$/, ''), out = join(outDir, `${tag}.txt`), trace = join(outDir, `${tag}.trace.jsonl`);
      if (existsSync(out) && /\nexit -?\d+\n$/.test(readFileSync(out, 'utf8').slice(-40))) { done++; continue; }
      writeFileSync(trace, '');
      const p = Bun.spawn(['bun', 'run', '--preload', self, ...job.cmd], { cwd: tree, env: { PATH: process.env.PATH ?? '', HOME: process.env.HOME ?? '', TPS_PROFILE: stand, LINES_TRACE_OUT: trace }, stdout: 'pipe', stderr: 'pipe' });
      const timer = setTimeout(() => p.kill(), 1500 * 1000);
      const [o, e, code] = await Promise.all([new Response(p.stdout).text(), new Response(p.stderr).text(), p.exited]); clearTimeout(timer);
      writeFileSync(out, `# ${job.file} :: bun run ${job.cmd.join(' ')}\n${o}${e}\nexit ${code}\n`);
      console.log(`  ${++done}/${queue.length} ${tag}: exit ${code}`); } };
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
  const res = { commands: 0, broke: [] as { cmd: string; label: string; at: string | null }[], mended: [] as { cmd: string; label: string }[], died: [] as { cmd: string; exit: number | null; tail: string }[], silent: [] as string[], sameRed: 0, gone: 0 };
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
      if (was) { usedB.add(was.i); res.sameRed++; at(a[1]); } else res.broke.push({ cmd: A.head, label: a[1].slice(0, 200), at: at(a[1]) }); });
    res.gone += loose.filter(x => !usedB.has(x.i) && x.b[0] === 'PASS').length;   // (said before, not said after: taken out with the switch, or never reached)
  }
  const places = [...new Set(res.broke.map(b => b.at).filter((x): x is string => !!x))].sort(), unplaced = res.broke.filter(b => !b.at);
  console.log(`${res.commands} commands · ${res.broke.length} lines PASS → FAIL (${places.length} places${unplaced.length ? `, ${unplaced.length} NOT PLACED: by hand` : ''}) · ${res.mended.length} FAIL → PASS · ${res.sameRed} red before and after · ${res.gone} said before and not after · ${res.died.length} commands died${res.silent.length ? ` · ${res.silent.length} say nothing any more (drop them whole)` : ''}`);
  for (const c of res.silent) console.log(`  SILENT ${c.slice(0, 140)}`);
  for (const d of res.died) console.log(`  DIED ${d.cmd.slice(0, 110)} exit ${d.exit} | ${d.tail}`);
  for (const b of res.broke) console.log(`  BROKE ${b.at ?? '(not placed)'} | ${b.label.slice(0, 120)}`);
  if (jsonOut) writeFileSync(jsonOut, JSON.stringify(res, null, 1));
  if (dropOut) writeFileSync(dropOut, JSON.stringify(places, null, 1));
  return res.broke.length || res.died.length || res.silent.length ? 1 : 0;
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
