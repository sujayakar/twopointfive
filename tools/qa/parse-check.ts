// EVERY TypeScript FILE UNDER tools/ PARSES. `tsconfig.json` covers src/ and tools/qa/tsconfig.json covers tools/qa; tools/blitz, tools/wire, tools/mp,
// tools/fold and the rest are in neither, so a file there that no gate tool happens to import can be broken and nobody is told (a name imported twice
// stopped tools/blitz/mission.ts once, and every kept run with it). This reads each such file through the transpiler, which refuses what the runtime
// would refuse at import: a syntax error, a name declared twice, an import of a name twice. It checks no types.
//   bun run tools/qa/parse-check.ts [root]        exit 0 every file parses · 1 a file does not (each named with the parser's own words)
import { readdirSync, readFileSync, statSync } from 'node:fs';
import { join, resolve } from 'node:path';
const ROOT = resolve(process.argv[2] ?? join(import.meta.dir, '../..')), T = new Bun.Transpiler({ loader: 'ts' });
const files: string[] = [], walk = (d: string) => { for (const n of readdirSync(d)) { if (n === 'node_modules' || n === 'target' || n.startsWith('.')) continue; const p = join(d, n); if (statSync(p).isDirectory()) walk(p); else if (/\.(ts|mts|cts)$/.test(n) && !n.endsWith('.d.ts')) files.push(p); } };
walk(join(ROOT, 'tools'));
let bad = 0;
for (const f of files.sort()) { try { T.transformSync(readFileSync(f, 'utf8')); } catch (e: any) { bad++; console.log(`- FAIL — ${f.slice(ROOT.length + 1)}: ${String(e?.errors?.[0]?.message ?? e?.message ?? e).split('\n')[0]}`); } }
console.log(bad ? `=== parse-check: ${bad} of ${files.length} files under tools/ do not parse ===` : `=== parse-check: ALL PASS · ${files.length} files under tools/ ===`);
process.exit(bad ? 1 : 0);
