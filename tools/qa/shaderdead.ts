// DECLARATIONS NO PROGRAM OF THE BOOT CONTAINS. Every .wgsl file under src/render/shaders is cut into its module-scope declarations (gpu/wgslTrim.ts wgslDecls) and
// each one's name is looked for, as a declaration, in every shader text the page's boot makes (tools/qa/wgsl-dump.ts, every setting: title, snow, the smoke lab's
// page, the walls' row down). The trim already keeps an unreachable declaration out of the browser's compiler, so this costs a player nothing: it is the SOURCE that carries what a
// deleted pass or a folded switch left behind, and a reader cannot tell it from code that runs.
//   bun run tools/qa/shaderdead.ts            the list, by file (exit 0 always: a report)
//   bun run tools/qa/shaderdead.ts --json f   the same as JSON: [{ file, name, lines }]
// NOT COVERED, so read a name before deleting it: programs made after the boot from the same files (the K = 0 march, restir's lighting-trade twin, a dev hook's hot
// swap, the trace lab), and a file no boot module is made from at all (listed whole, under 'no program of the boot is made from').
export {};
import { readdirSync, readFileSync, mkdtempSync } from 'node:fs';
import { join } from 'node:path';
import { tmpdir } from 'node:os';
import { wgslDecls } from '../../src/gpu/wgslTrim';
const ROOT = join(import.meta.dir, '../..'), DIR = join(ROOT, 'src/render/shaders');
const nameOf = (d: string): string | null => (/^(?:@[a-z_]+(?:\([^)]*\))?\s*)*(?:fn|struct|alias)\s+(\w+)/.exec(d) ?? /^(?:@[a-z_]+(?:\([^)]*\))?\s*)*(?:const|override|var(?:<[^>]*>)?)\s+(\w+)/.exec(d))?.[1] ?? null;
const tmp = mkdtempSync(join(tmpdir(), 'shaderdead-')); let code = '';
for (const s of ['title', 'snow', 'smokelab', 'nowalls']) {
  const out = join(tmp, `${s}.json`); const r = Bun.spawnSync(['bun', 'run', join(ROOT, 'tools/qa/wgsl-dump.ts'), out, '--setting', s], { cwd: ROOT, stdout: 'pipe', stderr: 'pipe' });
  if (r.exitCode !== 0) { console.error(`shaderdead: the dump of '${s}' failed: ${r.stderr.toString().slice(0, 400)}`); process.exit(2); }
  for (const m of JSON.parse(readFileSync(out, 'utf8')) as { code: string }[]) code += '\n' + m.code;
}
const declared = new Set<string>(); for (const m of code.matchAll(/(?:^|[\s;}])(?:fn|struct|alias|const|override|var(?:<[^>]*>)?)\s+(\w+)/g)) declared.add(m[1]);
const rows: { file: string; name: string; lines: number }[] = [], whole: string[] = [];
for (const f of readdirSync(DIR).filter(f => f.endsWith('.wgsl')).sort()) {
  const ds = wgslDecls(readFileSync(join(DIR, f), 'utf8')).map(d => ({ d, n: nameOf(d) })).filter(x => x.n) as { d: string; n: string }[];
  const dead = ds.filter(x => !declared.has(x.n));
  if (ds.length && dead.length === ds.length) { whole.push(`${f} (${ds.length} declarations)`); continue; }
  for (const x of dead) rows.push({ file: f, name: x.n, lines: x.d.split('\n').length });
}
const ji = process.argv.indexOf('--json'); if (ji >= 0) await Bun.write(process.argv[ji + 1], JSON.stringify(rows, null, 1));
const by = new Map<string, typeof rows>(); for (const r of rows) (by.get(r.file) ?? by.set(r.file, []).get(r.file)!).push(r);
console.log(`=== shaderdead: ${rows.length} declarations in ${by.size} files that no program of the boot contains (${rows.reduce((a, r) => a + r.lines, 0)} lines) ===`);
for (const [f, rs] of by) console.log(`  ${f}: ${rs.map(r => r.name + (r.lines > 1 ? ` (${r.lines})` : '')).join(' · ')}`);
if (whole.length) console.log(`  no program of the boot is made from: ${whole.join(' · ')}`);
