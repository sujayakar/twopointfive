// THE PAGE DRAWS, AND HOLDS NO RULE AND MAKES NO LEVEL: src/host/drawpage.ts, the start page and the load brief (the cards before a game: the desk answers them) and everything they name, at any depth,
// name no file where the game's TypeScript stood (src/game, src/sim: the game is rust/) and nothing a level is made with (a Level and its file's reader and writer, a generator, a tool), not even as a type: the game reads
// its own level file. A static import, an export-from, an import type
// (`import('…').X`) and a dynamic import all count and are followed, as the TypeScript scanner reads them. `bun run build` runs this first. No TypeScript on this machine (a hosted build; vendor/typescript or TS_LIB=<path to typescript.js>): only what the
// bundle runs is followed, and it says so.
//   bun run tools/draw/ban.ts      exit 0 · 1 and each name that breaks it, with the road to it
//   BAN_BREAK=1 bun run tools/draw/ban.ts     the red path: one name of each kind planted in the page, in memory: exit 1 with all four named (2: no TypeScript here to see a type with, 3: it missed one)
import { existsSync, readFileSync } from 'node:fs';
import { dirname, join, resolve } from 'node:path';

const ROOT = resolve(import.meta.dir, '../..'), ENTRIES = ['src/host/drawpage.ts', 'src/ui/startpage.ts', 'src/ui/loadbrief.ts'].map(f => join(ROOT, f));
const BANNED = /^(src\/(game|sim|replay)\/|src\/anim\/skeleton\.ts$|tools\/)/;
const BREAK = process.env.BAN_BREAK === '1', PLANT = "\nimport type { Level } from '../../tools/levels/lib/level';\nexport type { LevelData } from '../../tools/levels/lib/levelData';\ntype Planted = import('../../tools/levels/lib/levelLoad').Level;\nvoid import('../../tools/levels/floors');\n", PLANTED = 4;
const TS_LIB = process.env.TS_LIB ?? join(ROOT, 'vendor/typescript/lib/typescript.js'), ts: any = existsSync(TS_LIB) ? ((m: any) => m.default ?? m)(await import(TS_LIB)) : null;
if (!ts) { console.log(`draw ban: no TypeScript at ${TS_LIB}: what the bundle runs is followed, and no type`); if (BREAK) process.exit(2); }
const scan = new Bun.Transpiler({ loader: 'ts' });
/** every module a file names */
function named(f: string, text: string): string[] {
  if (!ts) return scan.scanImports(text).map(i => i.path);
  const out: string[] = [], walk = (n: any): void => {
    if ((ts.isImportDeclaration(n) || ts.isExportDeclaration(n)) && n.moduleSpecifier) out.push(n.moduleSpecifier.text);
    else if (ts.isImportEqualsDeclaration(n) && ts.isExternalModuleReference(n.moduleReference)) out.push(n.moduleReference.expression.text);
    else if (ts.isImportTypeNode(n) && ts.isLiteralTypeNode(n.argument)) out.push(n.argument.literal.text);
    else if (ts.isCallExpression(n) && n.expression.kind === ts.SyntaxKind.ImportKeyword && ts.isStringLiteralLike(n.arguments[0])) out.push(n.arguments[0].text);
    ts.forEachChild(n, walk); };
  walk(ts.createSourceFile(f, text, ts.ScriptTarget.Latest, true)); return out; }

const from = new Map<string, string>(), queue = [...ENTRIES], bad: string[] = [];
const rel = (p: string) => p.startsWith(ROOT) ? p.slice(ROOT.length + 1) : p;
for (const e of ENTRIES) from.set(e, '');
const road = (p: string): string => { const r: string[] = []; for (let x: string | undefined = p; x; x = from.get(x)) r.unshift(rel(x)); return r.join(' > '); };
while (queue.length) { const f = queue.shift()!, text = readFileSync(f, 'utf8') + (BREAK && f === ENTRIES[0] ? PLANT : '');
  for (const path of new Set(named(f, text))) {
    if (/\.(wgsl|json|txt|css)$/.test(path)) continue;
    const p = path.startsWith('.') ? [resolve(dirname(f), path), resolve(dirname(f), `${path}.ts`), resolve(dirname(f), path, 'index.ts')].find(x => /\.ts$/.test(x) && existsSync(x)) : undefined, name = p ? rel(p) : path;
    if (BANNED.test(name)) { bad.push(`${name}\n    ${road(f)}`); continue; }
    if (p && !from.has(p)) { from.set(p, f); queue.push(p); }
  } }
for (const b of bad) console.log(`draw ban: ${b}`);
console.log(bad.length ? `draw ban: ${bad.length} names of level making under the page that draws` : `draw ban: ${from.size} files under the page that draws, none of them level making's`);
if (BREAK && bad.length < PLANTED) { console.log(`draw ban: the red path planted ${PLANTED} names and ${bad.length} were seen`); process.exit(3); }
process.exit(bad.length ? 1 : 0);
