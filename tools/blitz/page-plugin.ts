// THE PAGE'S WAY TO THE STATE BRIDGE (tools/blitz: export.ts, import.ts, classes.ts, puppet-apply.ts, out-dispatch.ts, ask.ts, schema.json, puppet-lists.json), for the
// one bundle that needs it: the game's page under `?rust=1` (src/host/rustgame.ts). The bridge's files are generated and stay where they are; nothing is copied
// and nothing of them is edited on disk. This bundler plugin (bunfig.toml [serve.static] for the dev server, tools/build.ts for the static build)
//   · resolves `tps-bridge/<name>` to tools/blitz/<name> (src/host/rustbridge.d.ts says what each holds, so the type check never walks into tools/);
//   · loads export.ts and import.ts with the lines a browser cannot run swapped: the schema read from disk becomes an import of the same JSON, and the
//     tool-only words on `process.env` become `false`;
//   · gives the bridge its window on a module's own variables, as tools/blitz/expose.ts gives a tool: one line at the end of each source file of the
//     simulation's folders, a getter (and for a `let`, a setter) for each top-level name the schema's walk asks for, on globalThis.__tpsTop[<file>]. Nothing
//     reads them unless the Rust game is asked for.
import type { BunPlugin } from 'bun';
import { readFileSync } from 'node:fs';
import { resolve, join } from 'node:path';

const HERE = resolve(import.meta.dir), ROOT = resolve(HERE, '../..');
const SCHEMA = JSON.parse(readFileSync(join(HERE, 'schema.json'), 'utf8'));
const FILES: Record<string, string> = { export: 'export.ts', import: 'import.ts', classes: 'classes.ts', 'puppet-apply': 'puppet-apply.ts', 'out-dispatch': 'out-dispatch.ts', ask: 'ask.ts', 'schema.json': 'schema.json', 'puppet-lists.json': 'puppet-lists.json' };
/** a swap that must find its line: a generated file that changed shape is said at build time, not found out in the page */
function swap(text: string, file: string, from: string | RegExp, to: string): string {
  const out = text.replace(from, to); if (out === text) throw new Error(`page-plugin: tools/blitz/${file} no longer holds ${String(from)}: the page's bridge needs that line swapped`);
  return out;
}
const plugin: BunPlugin = {
  name: 'tps-page-bridge',
  setup(build) {
    build.onResolve({ filter: /^tps-bridge\// }, a => { const f = FILES[a.path.slice('tps-bridge/'.length)]; if (!f) throw new Error(`page-plugin: no bridge file for ${a.path}`); return { path: join(HERE, f) }; });
    build.onLoad({ filter: /[\\/]tools[\\/]blitz[\\/](export|import)\.ts$/ }, async ({ path }) => {
      let text = await Bun.file(path).text();
      if (path.endsWith('export.ts')) {
        text = swap(text, 'export.ts', "import { readFileSync } from 'node:fs';", "import SCHEMA_JSON from './schema.json';");
        text = swap(text, 'export.ts', 'const here = import.meta.dir;', '');
        text = swap(text, 'export.ts', /export const SCHEMA: any = JSON\.parse\(readFileSync\([^\n]*?\)\);/, 'export const SCHEMA: any = SCHEMA_JSON;');
      } else text = swap(text, 'import.ts', /process\.env\.PUP_(PROF|DEBUG)/g, 'false');
      // (what is left of `process` in import.ts stands behind the words just made `false`: never run)
      if (/readFileSync|import\.meta\.dir|process\.env/.test(text)) throw new Error(`page-plugin: ${path.slice(ROOT.length + 1)} still names the disk or the process's words after its swaps`);
      return { contents: text, loader: 'ts' };
    });
    build.onLoad({ filter: /[\\/]src[\\/](game|anim|scene|sim|math|smoke)[\\/].*\.ts$/ }, async ({ path }) => {
      const text = await Bun.file(path).text(), rel = path.slice(path.lastIndexOf('/src/') + 1).split('\\').join('/'), names: string[] = SCHEMA.top[rel] ?? [];
      if (!names.length) return { contents: text, loader: 'ts' };
      const isLet = (n: string) => new RegExp(`^(?:export )?let [^\\n]*\\b${n}\\b\\s*(?:[:=,;])`, 'm').test(text);
      const body = names.map(n => `get ${n}() { return ${n}; }` + (isLet(n) ? `, set ${n}(v) { ${n} = v; }` : '')).join(', ');
      return { contents: `${text}\n;((globalThis as any).__tpsTop ??= {})[${JSON.stringify(rel)}] = { ${body} };\n`, loader: 'ts' };
    });
  },
};
export default plugin;
