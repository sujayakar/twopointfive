// THE PAGE WORKS OUT NO RULE THAT NOBODY HAS READ: the list of what the page's TypeScript still decides on its copy of the game (tools/blitz/page-decides.ts) has a sort
// and a reason for every row, none is left to move, and no line of it sorts more rows than when it was last read. A new one is red here until it is moved into the Rust
// game, or read and given its reason.
//   bun run tools/qa/page-decides-check.ts      exit 0 = it holds · 1 = a row nobody has read
//   PAGEDECIDES_BREAK=1  the sorts are read without one of their lines: the rows it sorted are nobody's, and the check must FAIL
import { mkdtempSync, readFileSync, rmSync, writeFileSync } from 'node:fs';
import { tmpdir } from 'node:os';
import { join, resolve } from 'node:path';

const HOME = resolve(import.meta.dir, '../blitz'), args: string[] = []; let dir = '';
if (process.env.PAGEDECIDES_BREAK === '1') {
  const lines = readFileSync(join(HOME, 'page-decides.rows.tsv'), 'utf8').split('\n'), k = lines.findIndex(l => l.split('\t')[1] === 'leaves' && Number(l.split('\t')[4]) > 0);
  dir = mkdtempSync(join(tmpdir(), 'page-decides-')); writeFileSync(join(dir, 'rows.tsv'), lines.filter((_, i) => i !== k).join('\n')); args.push('--rows', join(dir, 'rows.tsv'));
}
const p = Bun.spawnSync(['bun', 'run', join(HOME, 'page-decides.ts'), '--check', ...args], { stdout: 'pipe', stderr: 'pipe' });
if (dir) rmSync(dir, { recursive: true });
const said = (p.stdout.toString() + p.stderr.toString()).trim().split('\n').filter(l => /^page-decides: \d+ functions under|UNSORTED:|throws:|decides:|take them out|sort more rows|page-decides\.r/.test(l));
console.log(`- ${p.exitCode === 0 ? 'PASS' : 'FAIL'} — every rule the page works out on its copy has been read and has its reason\n${said.map(l => `    ${l.trim().slice(0, 400)}`).join('\n')}`);
console.log(`=== page-decides: ${p.exitCode === 0 ? 'ALL PASS' : 'FAIL'} ===`);
process.exit(p.exitCode === 0 ? 0 : 1);
