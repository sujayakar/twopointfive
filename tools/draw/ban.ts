// THE PAGE THAT DRAWS HOLDS NO GAME: src/host/drawpage.ts, the start page and the load brief (the cards before a game: the desk answers them) and everything they import, at any depth, import none of the
// game's TypeScript (src/game, src/sim, the animator, the bridge). Types do not count: they are no part of the bundle. `bun run build` runs this first.
//   bun run tools/draw/ban.ts      exit 0 · 1 and each import that breaks it, with the road to it
import { existsSync, readFileSync } from 'node:fs';
import { dirname, join, resolve } from 'node:path';

const ROOT = resolve(import.meta.dir, '../..'), ENTRIES = ['src/host/drawpage.ts', 'src/ui/startpage.ts', 'src/ui/loadbrief.ts'].map(f => join(ROOT, f));
const BANNED = /^(src\/(game|sim|net|replay)\/|src\/anim\/skeleton\.ts$|src\/host\/(rustgame|rustdress|standup|profile|loop|tween|ahead)\.ts$|tools\/)|^tps-bridge\//;
/** leaves that stand in those folders and hold nothing of the game: each goes to a folder of its own when src/game does */
const LEAVES = new Set(['src/game/camera.ts', 'src/game/input.ts', 'src/game/crossing.ts', 'src/game/sheetModel.ts', 'src/game/shardModel.ts']);

const scan = new Bun.Transpiler({ loader: 'ts' }), from = new Map<string, string>(), queue = [...ENTRIES], bad: string[] = [];
const rel = (p: string) => p.startsWith(ROOT) ? p.slice(ROOT.length + 1) : p;
for (const e of ENTRIES) from.set(e, '');
const road = (p: string): string => { const r: string[] = []; for (let x: string | undefined = p; x; x = from.get(x)) r.unshift(rel(x)); return r.join(' > '); };
while (queue.length) { const f = queue.shift()!;
  for (const i of scan.scanImports(readFileSync(f, 'utf8'))) {
    if (/\.(wgsl|json|txt|css)$/.test(i.path)) continue;
    const p = i.path.startsWith('.') ? [resolve(dirname(f), i.path), resolve(dirname(f), `${i.path}.ts`), resolve(dirname(f), i.path, 'index.ts')].find(x => /\.ts$/.test(x) && existsSync(x)) : undefined, name = p ? rel(p) : i.path;
    if (BANNED.test(name) && !LEAVES.has(name)) { bad.push(`${name}\n    ${road(f)}`); continue; }
    if (p && !from.has(p)) { from.set(p, f); queue.push(p); }
  } }
for (const b of bad) console.log(`draw ban: ${b}`);
console.log(bad.length ? `draw ban: ${bad.length} imports of the game under the page that draws` : `draw ban: ${from.size} files under the page that draws, none of them the game's`);
process.exit(bad.length ? 1 : 0);
