// A ROW OFF THE CALL SURFACE: its line of tools/blitz/calls.json and its arm of rust/game/src/call_gen.rs go together, and no other row moves (a row's number
// is its name: a row that leaves leaves a hole). For a function nothing outside a tick asks any more.
//   bun run tools/blitz/calls-drop.ts <key>…        the rows' keys, as calls.json has them ('objectiveText', 'Game.objectivePos'); refused if a key is no row
//                                                   or a TypeScript file still names it to cross
import { readFileSync, writeFileSync, readdirSync, statSync } from 'node:fs';
import { join } from 'node:path';
const ROOT = join(import.meta.dir, '../..'), CALLS = join(ROOT, 'tools/blitz/calls.json'), GEN = join(ROOT, 'rust/game/src/call_gen.rs');
const die = (s: string): never => { console.error(`calls-drop: ${s}`); process.exit(1); };
const keys = process.argv.slice(2); if (!keys.length) die('which rows?');
const text = readFileSync(CALLS, 'utf8'), J = JSON.parse(text) as { rows: { id: number; key: string; stub?: string }[] };
/** the file's own form: one line, a space after every comma and colon */
const form = (v: unknown): string => v === null || typeof v !== 'object' ? JSON.stringify(v) : Array.isArray(v) ? `[${v.map(form).join(', ')}]` : `{${Object.entries(v as object).map(([k, x]) => `${JSON.stringify(k)}: ${form(x)}`).join(', ')}}`;
if (form(J) !== text.trimEnd()) die('calls.json is not in the form this program writes: it would rewrite more than the rows');
const gone = keys.map(k => J.rows.find(r => r.key === k) ?? die(`${k} is no row`)), files: string[] = [];
const walk = (d: string): void => { for (const e of readdirSync(d)) { const p = join(d, e); if (statSync(p).isDirectory()) { if (e !== 'node_modules') walk(p); } else if (/\.tsx?$/.test(e)) files.push(p); } };
walk(join(ROOT, 'src')); for (const f of files) { const t = readFileSync(f, 'utf8'); for (const r of gone) if (t.includes(`call('${r.key}'`) || t.includes(`'${r.stub ?? r.key}', this`) || t.includes(`crossing('${r.stub ?? r.key}'`)) die(`${f.slice(ROOT.length + 1)} still crosses by ${r.key}`); }
let gen = readFileSync(GEN, 'utf8'); for (const r of gone) { const L = gen.split('\n'), i = L.findIndex(l => l.startsWith(`${r.id} => `)); if (i < 0) die(`${r.key}: no arm ${r.id} in call_gen.rs`); L.splice(i, 1); gen = L.join('\n'); }
J.rows = J.rows.filter(r => !gone.includes(r)); writeFileSync(CALLS, form(J) + (text.endsWith('\n') ? '\n' : '')); writeFileSync(GEN, gen); console.log(`calls-drop: ${gone.length} rows off (${gone.map(r => `${r.key} ${r.id}`).join(', ')}); ${J.rows.length} left`);
