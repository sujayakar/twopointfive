// EVERY LEVEL'S FILE IS WHAT ITS GENERATOR WRITES TODAY: `--check` of both exporters, over every generator whose file is in levels/ (a generator with no file there is a showroom, or in the making).
//   bun run tools/qa/exports-check.ts [name …]      exit 0 · 1 and each that is stale, or could not be written, with its own words
import { existsSync, readFileSync, readdirSync } from 'node:fs';
import { resolve } from 'node:path';

const ROOT = resolve(import.meta.dir, '../..'), only = process.argv.slice(2);
const names = readdirSync(resolve(ROOT, 'tools/levels')).filter(f => f.endsWith('.ts')).filter(f => /^export (function|const) build\b/m.test(readFileSync(resolve(ROOT, 'tools/levels', f), 'utf8'))).map(f => f.replace(/\.ts$/, ''));
const asked = [...names.filter(n => existsSync(resolve(ROOT, 'levels', `${n}.json`))).map(n => ({ name: n, run: ['tools/levels/export.ts', n, '--check'] })), { name: 'office', run: ['tools/level-export.ts', '--check'] }].filter(a => !only.length || only.includes(a.name));
const stray = only.filter(n => !asked.some(a => a.name === n)); if (stray.length) { console.error(`exports-check: no generator of a file is called ${stray.join(', ')}`); process.exit(2); }
let fails = 0; const queue = [...asked];
await Promise.all(Array.from({ length: 4 }, async () => { for (let a = queue.shift(); a; a = queue.shift()) {
  const p = Bun.spawn(['bun', 'run', ...a.run], { cwd: ROOT, stdout: 'pipe', stderr: 'pipe' }), [out, err, code] = await Promise.all([new Response(p.stdout).text(), new Response(p.stderr).text(), p.exited]);
  if (code) { fails++; const said = `${err}\n${out}`.split('\n').filter(l => l.trim()); console.log(`- FAIL — ${a.name}: ${(said.find(l => /^(error|--check|export )/.test(l)) ?? said.at(-1) ?? `exit ${code}`).trim()}`); } } }));
console.log(`- ${fails ? 'FAIL' : 'PASS'} — ${asked.length - fails} of ${asked.length} levels' files are what their generators write`);
process.exit(fails ? 1 : 0);
