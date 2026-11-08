// NOTHING UNDER src/view IMPORTS THE GAME: the record a HUD is told (src/view/seatview.ts) is read the same by a page that holds no game at all, so its types name
// no class of the game's and its files import nothing that does. A static check, no game stood.
//   bun run tools/qa/view-check.ts        exit 0 = it holds · 1 = a file imports what it may not
//   VIEW_BREAK=1  the folders allowed are asked to be none: the check must FAIL
import { readFileSync, readdirSync, statSync } from 'node:fs';
import { join, resolve, dirname, relative } from 'node:path';

const ROOT = resolve(import.meta.dir, '../..'), HOME = join(ROOT, 'src/view');
/** what a file of src/view may import: its own folder, the plain arithmetic types, and src/draw (the page that holds no game: its own build rule keeps it so) */
const MAY = process.env.VIEW_BREAK === '1' ? [] : ['src/view/', 'src/math/', 'src/draw/'];

export function viewCheck(): boolean {
  const files: string[] = []; const walk = (d: string): void => { for (const e of readdirSync(d)) { const p = join(d, e); if (statSync(p).isDirectory()) walk(p); else if (/\.tsx?$/.test(e)) files.push(p); } }; walk(HOME);
  const bad: string[] = []; let n = 0;
  for (const f of files) for (const m of readFileSync(f, 'utf8').matchAll(/^\s*(?:import|export)\b[^'"\n]*?from\s+['"]([^'"]+)['"]|^\s*import\s+['"]([^'"]+)['"]|\bimport\(\s*['"]([^'"]+)['"]/gm)) { n++;
    const what = m[1] ?? m[2] ?? m[3], to = what.startsWith('.') ? relative(ROOT, resolve(dirname(f), what)) : what;
    if (!MAY.some(x => to.startsWith(x))) bad.push(`${relative(ROOT, f)} imports ${to}`); }
  const ok = files.length > 0 && n > 0 && !bad.length;
  console.log(`- ${ok ? 'PASS' : 'FAIL'} — nothing under src/view imports the game (${files.length} files, ${n} imports)${bad.length ? `: ${bad.join(' · ')}` : ''}`);
  console.log(ok ? '=== view-check: ALL PASS ===' : '=== view-check: FAILED ==='); return ok;
}
if (import.meta.main) process.exit(viewCheck() ? 0 : 1);
