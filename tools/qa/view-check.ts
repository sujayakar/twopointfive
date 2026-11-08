// NOTHING UNDER src/view, AND NOTHING UNDER THE HUD IN PLAY, IMPORTS THE GAME: the record a HUD is told (src/view/seatview.ts) and what draws it are read and run the same by a page that holds no game at all,
// so their types name no class of the game's and their files import nothing that does, at any depth, not even as a type, and reach for no hook a page hangs on the window. A static check, no game stood. Developer
// pictures that read what only the TypeScript holds are no part of the HUD (ui/devhud.ts: they stay on the page that holds a game).
//   bun run tools/qa/view-check.ts        exit 0 = it holds · 1 = a file imports what it may not
//   VIEW_BREAK=1  every form of a forbidden import is planted, in memory, in a file the walk reaches at the far end of an import: the walk must FAIL
// (vendor/typescript, or TS_LIB=<path to typescript.js>: imports are found by the TypeScript scanner)
import { existsSync, readFileSync, readdirSync, statSync } from 'node:fs';
import { join, resolve, dirname, relative } from 'node:path';

const ROOT = resolve(import.meta.dir, '../..'), HOME = join(ROOT, 'src/view');
const TS_LIB = process.env.TS_LIB ?? join(ROOT, 'vendor/typescript/lib/typescript.js');
if (!existsSync(TS_LIB)) { console.error(`no TypeScript at ${TS_LIB} (vendor/typescript, or TS_LIB=<path to typescript.js>)`); process.exit(2); }
const ts = (await import(TS_LIB)).default as { preProcessFile(text: string, imports: boolean, js: boolean): { importedFiles: { fileName: string }[] } };
/** the HUD in play: each of these and whatever they import */
const HUD = ['overlay', 'cmdring', 'ringlone', 'bubbles', 'netplace', 'findring', 'planfloor', 'handbox', 'tabcard', 'corner', 'squadthrowviz', 'attentionviz', 'railcard', 'railclear'].map(f => `src/ui/${f}.ts`).concat('src/host/hud.ts');
/** a file under src/view may import its own folder, the plain arithmetic types, and src/draw, which itself imports nothing of the game (tools/draw/ban.ts) */
const MAY_VIEW = ['src/view/', 'src/math/', 'src/draw/'];
/** anything else on the walk: these folders and leaves, each walked in its turn, so a file here that imports the game is named too */
const MAY_HUD = [...MAY_VIEW, 'src/ui/', 'src/host/keymap.ts', 'src/host/hud.ts', 'src/host/findme.ts', 'src/audio/floors.ts'];
/** a dynamic import or a require of something no scanner can read, and a hook the page hangs on the window */
const UNREADABLE = /\b(?:import|require)\s*\(\s*(?!['"][^'"]*['"]\s*\))/, HOOK = /\b(?:window|globalThis|self)\b[^;\n]{0,24}?(?:\.\s*__|\[\s*['"`]__)/;
/** every way a file could say it: each must be red, whatever else holds */
const FORMS = ["import type {\n  Game\n}\n  from '../game/game';", "import{Game}from'../game/game';", "export { Game } from '../game/game';", "const a = import('../game/game');", "const b = require('../game/game');", "import type { C } from '../game';", "import x from '../game/thing.json';", "const c = import(`../game/${n}`);", "const d = (window as any).__game;", "const e = window['__game'];"];
const PLANT_IN = 'src/ui/railclear.ts';

/** what is wrong with one file's text, and the files it leads on to */
function scan(f: string, text: string): { bad: string[]; next: string[]; n: number } {
  const bad: string[] = [], next: string[] = [], name = relative(ROOT, f), may = name.startsWith('src/view/') ? MAY_VIEW : MAY_HUD;
  const code = text.replace(/\/\*[\s\S]*?\*\/|(^|[^:'"`])\/\/[^\n]*/g, '$1');   // (a comment may name what it likes)
  if (UNREADABLE.test(code)) bad.push(`${name} imports by a name no scanner can read`);
  if (HOOK.test(code)) bad.push(`${name} reaches for a hook on the window`);
  const found = ts.preProcessFile(text, true, true).importedFiles;
  for (const { fileName: what } of found) {
    if (!what.startsWith('.')) { bad.push(`${name} imports ${what}`); continue; }
    const to = resolve(dirname(f), what), file = [`${to}.ts`, `${to}.tsx`, join(to, 'index.ts'), to].find(x => existsSync(x) && statSync(x).isFile()) ?? `${to}.ts`, rel = relative(ROOT, file);
    if (!/\.tsx?$/.test(rel)) { bad.push(`${name} imports ${rel}: no TypeScript`); continue; }
    if (!may.some(x => rel.startsWith(x))) { bad.push(`${name} imports ${rel}`); continue; }
    if (!existsSync(file)) { bad.push(`${name} imports ${rel}: not there`); continue; }
    next.push(file);
  }
  return { bad, next, n: found.length };
}
export function viewCheck(plant = process.env.VIEW_BREAK === '1'): boolean {
  const planted = join(ROOT, PLANT_IN), queue: string[] = []; const walk = (d: string): void => { for (const e of readdirSync(d)) { const p = join(d, e); if (statSync(p).isDirectory()) walk(p); else if (/\.tsx?$/.test(e)) queue.push(p); } }; walk(HOME);
  for (const f of HUD) { if (!existsSync(join(ROOT, f))) { console.log(`- FAIL — ${f} is on the list and is not there`); return false; } queue.push(join(ROOT, f)); }
  // (the scanner itself: each form, in a file of the walk, is red and says so)
  const blind = FORMS.filter(form => !scan(planted, form).bad.length);
  console.log(`- ${blind.length ? 'FAIL' : 'PASS'} — each of ${FORMS.length} ways of saying it is seen${blind.length ? `; not seen: ${blind.join(' · ')}` : ''}`);
  const seen = new Set<string>(), bad: string[] = []; let n = 0;
  while (queue.length) { const f = queue.pop()!; if (seen.has(f)) continue; seen.add(f);
    const r = scan(f, readFileSync(f, 'utf8') + (plant && f === planted ? `\n${FORMS.join('\n')}\n` : '')); n += r.n; bad.push(...r.bad); queue.push(...r.next); }
  const reached = seen.has(planted), ok = !blind.length && reached && n > 0 && !bad.length;
  console.log(`- ${ok ? 'PASS' : 'FAIL'} — nothing under src/view or the HUD in play imports the game (${seen.size} files, ${n} imports)${bad.length ? `: ${bad.join(' · ')}` : ''}`);
  console.log(ok ? '=== view-check: ALL PASS ===' : '=== view-check: FAILED ==='); return ok;
}
if (import.meta.main) process.exit(viewCheck() ? 0 : 1);
