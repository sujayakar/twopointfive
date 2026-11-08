// THE FILES THE PAGE THAT DRAWS KEEPS, OUT OF THE GAME'S FOLDERS: each `git mv`d, then every name of it in the tree rewritten: a relative import of any kind, and a path written out
// (`${ROOT}/src/game/x.ts`, a list keyed by path, a comment, a doc). A rule of the Rust side's comments says where the TypeScript was: rust/ is left as it is. Run it again on a newer tree: what has moved stays, and
// whatever names the old place is brought along. Then the generators write what they write.
//   bun run tools/draw/move.ts
import { $ } from 'bun';
import { existsSync, mkdirSync, readFileSync, writeFileSync } from 'node:fs';
import { dirname, join, normalize, relative, resolve } from 'node:path';

const ROOT = resolve(import.meta.dir, '../..'), SELF = 'tools/draw/move.ts', STAY = new Set([SELF, 'tools/blitz/schema.json']);   // (the wire's table says of two classes where they were (`file`): no tool reads it, and the table's text is what the layout's number is made of)
const MOVES: [string, string][] = [...['camera', 'input', 'gear'].map((f): [string, string] => [`src/game/${f}.ts`, `src/host/${f}.ts`]), ...['sheetModel', 'shardModel'].map((f): [string, string] => [`src/game/${f}.ts`, `src/render/${f}.ts`]), ['src/game/audio_iface.ts', 'src/audio/iface.ts'],
  ...['format', 'shelf', 'kept', 'keptui', 'keepstore', 'keeper', 'minimap'].map((f): [string, string] => [`src/replay/${f}.ts`, `src/tape/${f}.ts`])];
const TO = new Map(MOVES);
for (const [a, b] of MOVES) if (existsSync(join(ROOT, a)) && existsSync(join(ROOT, b))) throw new Error(`${b} stands already, and ${a} is still there`);

const CODE = /\.(ts|tsx|cjs|mjs|js)$/, TEXT = /\.(ts|tsx|cjs|mjs|js|json|tsv|md|py|sh|html|txt|wgsl)$/, IN = /^(src|tools|levels|cloud|docs\/agents|docs\/internal)\/|^[^/]+\.(md|html)$/;
const spec = /(\bfrom\s*|\bimport\s*\(\s*|\bimport\s+|\brequire\(\s*)(['"])(\.\.?\/[^'"\n]+)\2/g;
/** a relative name in `from`, as it reads from `to` */
function renamed(sp: string, from: string, to: string): string { const at = normalize(join(dirname(from), sp)), key = TO.has(at) ? at : TO.has(`${at}.ts`) ? `${at}.ts` : null; if (!key && from === to) return sp;
  let goes = key ? TO.get(key)! : at; if (key && !sp.endsWith('.ts')) goes = goes.replace(/\.ts$/, ''); const r = relative(dirname(to), goes); return r.startsWith('.') ? r : `./${r}`; }
/** a path written out: whole (`src/game/x`, with or without its `.ts`), or from its folder with its `.ts` (`game/x.ts`) */
const paths = MOVES.flatMap(([a, b]): [RegExp, string][] => { const A = a.replace(/\.ts$/, ''), B = b.replace(/\.ts$/, ''), s = A.slice('src/'.length), t = B.slice('src/'.length); return [[new RegExp(`\\b${A}\\b(?!-)`, 'g'), B], [new RegExp(`(?<![\\w/.])${s}\\.ts\\b`, 'g'), `${t}.ts`]]; });
paths.push([/(?<![\w/.])audio_iface\.ts\b/g, 'audio/iface.ts']);

const files = (await $`git ls-files -co --exclude-standard`.cwd(ROOT).text()).trim().split('\n').filter(f => !STAY.has(f) && IN.test(f) && TEXT.test(f) && !f.startsWith('docs/perf') && existsSync(join(ROOT, f)));
const out = new Map<string, string>(); let named = 0;
for (const f of files) { const was = readFileSync(join(ROOT, f), 'utf8'); if (was.includes('\0')) continue; const to = TO.get(f) ?? f; let s = was;
  if (CODE.test(f)) s = s.replace(spec, (m, a, q, sp) => { const r = renamed(sp, f, to); return r === sp ? m : `${a}${q}${r}${q}`; });
  for (const [re, by] of paths) s = s.replace(re, by);
  if (s !== was) { out.set(f, s); named++; } }
for (const [a, b] of MOVES) if (existsSync(join(ROOT, a))) { mkdirSync(dirname(join(ROOT, b)), { recursive: true }); await $`git mv ${a} ${b}`.cwd(ROOT); }
for (const [f, s] of out) writeFileSync(join(ROOT, TO.get(f) ?? f), s);
// (the camera and the Input are no longer the game's copy, so what writes them is no row of what the page decides on it)
{ const P = join(ROOT, 'tools/blitz/page-decides.rows.tsv'); writeFileSync(P, readFileSync(P, 'utf8').split('\n').filter(l => !/^(src\/host\/(input|camera)\.ts:\*|src\/game\/rails\.ts:Rails\.hostView|src\/main\.ts:main › refreshFrozenStill\*)\t/.test(l)).join('\n')); }
console.log(`move: ${MOVES.filter(([, b]) => existsSync(join(ROOT, b))).length} files stand in their new places, ${named} files named one of them and were rewritten`);
await $`bun run tools/wire/wire.ts gen`.cwd(ROOT);
