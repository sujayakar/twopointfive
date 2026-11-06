// THE STATE BRIDGE'S WINDOW ON A MODULE'S OWN VARIABLES. A module's top-level `let`s and tables are private, and only part of them are registered with side.ts (a table
// that is built at load and never written is rightly exempt there, but the Rust game needs it all the same). This preload adds one line at the end of every source file
// of the simulation's folders, with no effect on what runs: a getter for each top-level name the schema's walk asks for, kept on globalThis.__tpsTop[<file>]. A tool's only: never in the page.
//   bun --preload ./tools/blitz/expose.ts run tools/blitz/bridge-check.ts
import { plugin } from 'bun';
import { readFileSync } from 'node:fs';
const SCHEMA = JSON.parse(readFileSync(`${import.meta.dir}/schema.json`, 'utf8'));
plugin({ name: 'tps-expose', setup(build) {
  build.onLoad({ filter: /\/src\/(game|anim|scene|sim|math|smoke)\/.*\.ts$/ }, async ({ path }) => {
    const text = await Bun.file(path).text(); const rel = path.slice(path.indexOf('/src/') + 1); const names: string[] = SCHEMA.top[rel] ?? []; if (!names.length) return { contents: text, loader: 'ts' };
    const isLet = (n: string) => new RegExp(`^(?:export )?let [^\\n]*\\b${n}\\b\\s*(?:[:=,;])`, 'm').test(text); const body = names.map(n => `get ${n}() { return ${n}; }` + (isLet(n) ? `, set ${n}(v) { ${n} = v; }` : '')).join(', ');
    return { contents: `${text}\n;((globalThis as any).__tpsTop ??= {})[${JSON.stringify(rel)}] = { ${body} };\n`, loader: 'ts' };
  });
} });
