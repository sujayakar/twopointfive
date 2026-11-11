// THE PAGES SAY WHAT IS THERE. CLAUDE.md is the index, loaded into every session, so it stays small; docs/agents/<area>.md is one page a system, true to the code today. A page names files, modules,
// functions, tests, numbers and commands in backticks, and every one of them must be there:
//   a path from the top, or the end of one (`rust/sim/src/panes.rs`, `held_leaves.rs`, `doors/`)      some file or folder of the tree ends so. `*` stands for any run of letters
//   a name of the code (`doors_kick_in`, `DoorContact`, `face_step`, `k.doorthrow`, `?doorcost=0`)      it is a word of rust/, src/, tools/, cloud/ or the page's own files
//   a command (`bun run tools/qa/pages.ts boot`, `gate tests`)                                          what it runs is there
// A plain lower-case word or a phrase is nobody's name and is not asked. A page holds no date: git has the history.
// The same is asked of the other documents people work from (OTHER). And NO COMMENT of rust, src, tools or cloud names a .ts, .rs or .wgsl file that is not in the tree (comments.ts).
//   bun run tools/qa/docs-check.ts [page …]       exit 0 · 1 and each fault
//   DOCS_BREAK=1 bun run tools/qa/docs-check.ts   the red path: a page with one fault of each kind planted, in memory: exit 1 with every one found (3: it missed one)
import { execFileSync } from 'node:child_process';
import { existsSync, readFileSync } from 'node:fs';
import { resolve } from 'node:path';
import { goneInComments } from './comments';

const ROOT = resolve(import.meta.dir, '../..'), BREAK = process.env.DOCS_BREAK === '1', asked = process.argv.slice(2);
const INDEX = { bytes: 12_000, lines: 150 }, PAGE = 16_000, WIDE = 200;
/** the other documents people work from, and the most each may be */
const OTHER: Readonly<Record<string, number>> = { 'CLAUDE.md': 12_000, 'README.md': 8_000, 'STYLE.md': 18_000, 'docs/level-format.md': 42_000, '.claude/skills/verify/SKILL.md': 11_000, '.claude/skills/level/SKILL.md': 20_000,
  'rust/game_start/README.md': 34_000, 'tools/blitz/FACE.md': 4_000, 'tools/chars/README.md': 15_000, 'tools/chars/look/README.md': 5_000, 'cloud/README.md': 9_000 };

const read = (f: string) => readFileSync(resolve(ROOT, f), 'utf8');
const files: string[] = (execFileSync('git', ['-C', ROOT, 'ls-files', '--cached', '--others', '--exclude-standard'], { encoding: 'utf8', maxBuffer: 64 << 20 }) as string).split('\n').filter(f => f && existsSync(resolve(ROOT, f)));
/** every path of the tree by its last part: what a short name may be the end of */
const ends = new Map<string, string[]>();
for (const p of new Set(files.flatMap(f => { const at = f.split('/'); return at.map((_, i) => at.slice(0, i + 1).join('/') + (i + 1 < at.length ? '/' : '')); }))) {
  const last = p.replace(/\/$/, '').split('/').pop()!; ends.set(last, [...ends.get(last) ?? [], p]); }
const CODE = /^(rust|src|tools|cloud|types)\/.*\.(rs|ts|cjs|mjs|js|wgsl|sh|py|toml|html)$|^(package\.json|index\.html|sounds\.html|music\.html|vercel\.json|bunfig\.toml)$/;
const words = new Set<string>();
for (const f of files) if (CODE.test(f) && f !== 'tools/qa/docs-check.ts') for (const w of read(f).replace(/\/\*[\s\S]*?\*\/|\/\/.*$/gm, ' ').match(/[A-Za-z_$][\w$]*/g) ?? []) words.add(w);   // (a comment is no name's home)
/** what the level files say of themselves: a record's fields */
for (const f of ['levels/office.json', 'levels/registry.json']) for (const w of read(f).match(/"[A-Za-z_]\w*"(?=\s*:)/g) ?? []) words.add(w.slice(1, -1));
const scripts = Object.keys(JSON.parse(read('package.json')).scripts);

const here = (p: string): boolean => {
  const dir = p.endsWith('/'), clean = p.replace(/^\.\//, ''), last = clean.replace(/\/$/, '').split('/').pop()!;
  if (clean.includes('*')) { const re = new RegExp(`(^|/)${clean.replace(/[.+?^${}()|[\]\\]/g, '\\$&').replace(/\*+/g, '[^/]*')}${dir ? '' : '/?'}$`); return [...ends.values()].some(l => l.some(x => re.test(x))); }
  return (ends.get(last) ?? []).some(x => { const y = dir ? x : x.replace(/\/$/, ''); return y === clean || y.endsWith(`/${clean}`); }); };
const ENDING = /\.(rs|ts|cjs|mjs|js|wgsl|json|md|toml|html|sh|txt|py|css|glb|gltf|png|jpg)$/, NAME = /^[A-Za-z_$][\w$]*$/;
/** what a run of code says that is not there */
function gone(code: string): string[] {
  const out: string[] = [], c = code.trim();
  const run = c.match(/(?:^|[\s(;&])bun run ([^\s`]+)/); if (run) { const w = run[1]; if (!scripts.includes(w) && !here(w)) out.push(w); }
  for (const b of c.matchAll(/--bin ([\w-]+)/g)) if (!here(`src/bin/${b[1]}.rs`) && !here(`src/bin/${b[1]}/`)) out.push(`--bin ${b[1]}`);
  const tokens = /[\s]/.test(c) ? c.split(/\s+/).filter(t => t.includes('/') || ENDING.test(t)) : [c];
  for (let t of tokens) {
    t = t.replace(/^[("'[]+|[)"'\],;:.…]+$/g, (m, at: number) => (at === 0 ? '' : m.includes('()') ? '()' : m.startsWith('/') ? '/' : '')).replace(/\(\)$/, '');
    if (!t || /^(https?:|localhost|<|--?\w|\/?\?)/.test(t) && !t.startsWith('?')) continue;
    if (t.startsWith('?')) { for (const w of t.slice(1).split('&')) { const k = w.split('=')[0].replace(/^knob\./, ''); for (const part of k.split('.')) if (NAME.test(part) && !words.has(part)) out.push(t); } continue; }
    if (t.startsWith('/') || t.startsWith('~')) continue;
    if (t.includes('/') || ENDING.test(t)) { if (/^[\w@.*\-/ +]+$/.test(t) && !here(t)) out.push(t); continue; }
    for (const part of t.split(/::|\.|->/)) if (NAME.test(part) && (/_/.test(part) || /[a-z][A-Z]|^[A-Z][a-z]|^[A-Z0-9]{3,}$/.test(part) || t !== part && part.length > 2 || code.trim().endsWith('()')) && !words.has(part)) out.push(t);
  }
  return [...new Set(out)]; }

/** whose work it is, said on its page and at the code: a rewrite of either keeps the line as it stands */
const CREDIT = { line: 'Credit: Stephen Jayakar, from his pull request 1.', at: ['docs/agents/render.md', 'README.md',
  ...['composite', 'fgather', 'gather_common', 'particles', 'reflect', 'restir', 'sdfrig', 'sharp_direct', 'volumetrics'].map(k => `src/render/shaders/${k}.wgsl`)] };
const uncredited = (text: (f: string) => string) => CREDIT.at.filter(f => !text(f).includes(CREDIT.line));
/** the notices the game's maths stands under */
const NOTICE = { at: 'rust/maths/src/fns.rs', says: ['Copyright 2009 The Go Authors', 'Copyright 1984, 1987, 1989, 1992, 2000 by Stephen L. Moshier', 'Copyright (C) 2004 by Sun Microsystems', 'this list of conditions and the following disclaimer'] };

/** every fault of a page */
function faults(text: string, most: number): string[] {
  const out: string[] = [], bytes = Buffer.byteLength(text); if (bytes > most) out.push(`${bytes} bytes, ${most} at most`);
  let fence = false;
  text.split('\n').forEach((line, i) => {
    const say = (s: string) => out.push(`line ${i + 1}: ${s}`);
    if (/^\s*```/.test(line)) { fence = !fence; return; }
    if (line.length > WIDE) say(`${line.length} letters wide, ${WIDE} at most`);
    for (const code of fence ? [line.replace(/\s+#.*$/, '')] : [...line.matchAll(/`([^`]+)`/g)].map(m => m[1])) for (const g of gone(code)) say(`nothing here is '${g}'`);
    if (!fence) for (const m of line.matchAll(/\]\((?!https?:|#)([^)\s]+)\)/g)) if (!here(m[1].split('#')[0])) say(`a link to '${m[1]}', which is not here`);
    const date = line.match(/\b20\d\d-\d\d(-\d\d)?\b|\((?:1[0-2]|0?\d)-[0-3]\d\b/); if (date) say(`a date, '${date[0]}': git has the history`); });
  return out; }

if (BREAK) {
  const planted = '# A page\n\nThe boot is `doors_kick_nobody`, in `rust/game_rules/src/nowhere/` and `gone_test.rs`; see [it](docs/not-here.md). Since 2026-10-13.\n\n```sh\nbun run tools/qa/none-check.ts\n(cd rust/game_start && cargo run --bin nobin)\n```\n'
    + `\`?nosuchword=1\` and \`DoorNobody\` and \`k.nosuchtable.x\`.\n${'x'.repeat(PAGE)}\n`, found = faults(planted, PAGE);
  const want = ['doors_kick_nobody', 'rust/game_rules/src/nowhere/', 'gone_test.rs', 'docs/not-here.md', '2026-10', 'none-check.ts', '--bin nobin', '?nosuchword=1', 'DoorNobody', 'k.nosuchtable', `${PAGE} at most`, 'letters wide'];
  const missed = want.filter(w => !found.some(f => f.includes(w))); if (uncredited(f => read(f).replace(CREDIT.line, '')).length !== CREDIT.at.length) missed.push(CREDIT.line); for (const f of found) console.log(`- FAIL — planted: ${f.slice(0, 160)}`);
  // (a comment of each kind that names a file that is gone, and the same name in a string, which is no comment)
  const into = ['rs', 'ts', 'wgsl'].map(e => files.find(f => /^(rust|src)\//.test(f) && f.endsWith(`.${e}`))!), gone = goneInComments(into, f => `${read(f)}\n// as no_such_file.rs does\n/* src/game/nobody.ts */\n${f.endsWith('.wgsl') ? '' : 'const S: &str = "no_such_file.rs // not_a_comment.ts";'}\n`).filter(x => /no_such_file|nobody|not_a_comment/.test(x));
  if (gone.length !== into.length * 2 || gone.some(x => x.includes('not_a_comment'))) missed.push(`the planted comments (${gone.length} of ${into.length * 2})`);
  console.log(missed.length ? `docs: MISSED ${missed.join(' · ')}` : `docs: every planted fault found (${want.length})`); process.exit(missed.length ? 3 : 1);
}
let bad = 0;
const line = (ok: boolean, text: string) => { if (!ok) bad++; console.log(`- ${ok ? 'PASS' : 'FAIL'} — ${text}`); };
const index = read('CLAUDE.md'), areas = files.filter(f => /^docs\/agents\/[^/]+\.md$/.test(f)).sort(), nLines = index.split('\n').length, bytes = Buffer.byteLength(index);
if (!asked.length) {
  line(bytes <= INDEX.bytes && nLines <= INDEX.lines, `CLAUDE.md is ${bytes} bytes and ${nLines} lines (${INDEX.bytes} and ${INDEX.lines} at most)`);
  const unnamed = areas.filter(f => !index.includes(`\`${f.slice(12)}\``));
  line(!unnamed.length, `the index names every page (${areas.length})${unnamed.length ? `: not ${unnamed.join(' ')}` : ''}`);
  const lost = goneInComments(files, read); line(!lost.length, `no comment names a source file that is gone${lost.length ? ` (${lost.length})` : ''}`); for (const x of lost) console.log(`    ${x}`);
  const told = NOTICE.says.filter(w => !read(NOTICE.at).includes(w)); line(!told.length, `${NOTICE.at} carries its ${NOTICE.says.length} notices${told.length ? `: not '${told.join(`', '`)}'` : ''}`);
  const un = uncredited(f => files.includes(f) ? read(f) : '');
  line(!un.length, `"${CREDIT.line}" stands at all ${CREDIT.at.length} of its places${un.length ? `: not ${un.join(' ')}` : ''}`);
}
for (const f of areas.filter(a => asked.length ? asked.some(w => a.endsWith(`/${w.replace(/\.md$/, '')}.md`) || a === w) : true)) {
  const found = faults(read(f), PAGE); line(!found.length, `${f}${found.length ? '' : `: ${Buffer.byteLength(read(f))} bytes, every name in it is there`}`); for (const x of found) console.log(`    ${x}`); }
for (const [f, most] of Object.entries(OTHER)) if (!asked.length || asked.includes(f)) {
  const found = files.includes(f) ? faults(read(f), most) : ['it is not in the tree']; line(!found.length, `${f}${found.length ? '' : `: ${Buffer.byteLength(read(f))} bytes`}`); for (const x of found) console.log(`    ${x}`); }
console.log(bad ? `=== docs: ${bad} FAILED ===` : '=== docs: ALL PASS ===');
process.exit(bad ? 1 : 0);
