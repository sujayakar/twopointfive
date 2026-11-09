// Level lint from the shell — the CLI over tools/levels/lib/lint.ts (pure, no GPU):
// nav reachability spawn → objectives → exfil on the guards' own grid, doors walkable both sides / clear / sealing, overlapping statics,
// zero extents, lights inside solids / absurd, budgets, route legs, floor coverage.
//   bun run tools/qa/level-lint.ts [levels/office.json …] [--json] [--quiet]      none named: every level. Exit 1 if any file has an ERROR finding (warnings print, pass)
// A file may carry a sidecar `<name>.lint.json` = { allow: { code: { ids: [...] | "*", reason } } } for legitimately odd authored things.
import { resolve } from 'node:path';
import { existsSync, readFileSync, readdirSync } from 'node:fs';
import { ROOT } from './here.ts';

const { lintLevel, formatLint } = await import(`${ROOT}/tools/levels/lib/lint.ts`);
const { levelFaults } = await import(`${ROOT}/tools/levels/lib/levelData.ts`);

const args = process.argv.slice(2); const flags = new Set(args.filter(a => a.startsWith('--'))); const files = args.filter(a => !a.startsWith('--'));
if (!files.length) files.push(...readdirSync(resolve(ROOT, 'levels')).filter(f => f.endsWith('.json') && !f.endsWith('.lint.json')).map(f => `levels/${f}`));
let anyErrors = false;
for (const f of files) {
  const path = resolve(ROOT, f);
  if (!existsSync(path)) { console.log(`${f}: no such file`); anyErrors = true; continue; }
  let data: any; try { data = JSON.parse(readFileSync(path, 'utf8')); } catch (e) { console.log(`${f}: not JSON — ${(e as Error).message}`); anyErrors = true; continue; }
  const side = path.replace(/\.json$/, '.lint.json'); let allow: any = undefined;
  if (existsSync(side)) { try { allow = JSON.parse(readFileSync(side, 'utf8')).allow; } catch (e) { console.log(`${f}: sidecar ${side} unreadable — ${(e as Error).message}`); } }
  const v = levelFaults(data); if (v.length && !flags.has('--quiet')) console.log(`${f}: ${v.length} schema error(s) — lint reports them and stops there`);
  const r = lintLevel(data, { allow });
  if (flags.has('--json')) console.log(JSON.stringify({ file: f, ...r }, null, 1));
  else { const lines = formatLint(r); console.log(`${f} — ${lines[0]}`); if (!flags.has('--quiet') || r.errors) for (const l of lines.slice(1)) console.log(l); }
  if (r.errors) anyErrors = true;
  console.log(`=== level-lint ${f}: ${r.errors ? `FAIL (${r.errors} error${r.errors === 1 ? '' : 's'})` : `PASS (${r.warnings} warning${r.warnings === 1 ? '' : 's'})`} ===`);
}
process.exit(anyErrors ? 1 : 0);
