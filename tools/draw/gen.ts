// THE PAGE'S OFFSETS INTO WHAT RUST DRAWS: src/draw/sections.ts, made from the declarations in rust/game_draw (a `section!` a row, `sections!` the list). A
// row lies in the module's memory and is read there: `f[at + Fog.edge]` over one view of that memory as floats, `u[…]` over one as words (src/draw/drawn.ts).
// Every offset is a constant of the bundle. Nothing is copied and no object is made.
//   bun run tools/draw/gen.ts [--check]      write the file · --check: exit 1 where it would change
import { readFileSync, readdirSync, writeFileSync, existsSync } from 'node:fs';
import { join } from 'node:path';

const ROOT = join(import.meta.dir, '../..'), DIR = join(ROOT, 'rust/game_draw/src'), OUT = join(ROOT, 'src/draw/sections.ts');
const die = (s: string): never => { console.error(`draw: ${s}`); process.exit(1); };

interface Field { name: string; ty: string; words: number; doc: string }
interface Row { name: string; doc: string; fields: Field[] }
const docOf = (lines: string[]): string => lines.map(l => l.trim().replace(/^\/\/\/ ?/, '')).join(' ');
function words(ty: string): number { if (/^(f32|u32|i32)$/.test(ty)) return 1; const m = ty.match(/^\[(f32|u32|i32);(\d+)\]$/); return m ? Number(m[2]) : die(`a field is f32, u32, i32 or an array of one, not ${ty}`); }

const rows = new Map<string, Row>();   // (by `file::Name`)
for (const f of (readdirSync(DIR, { recursive: true }) as string[]).filter(f => f.endsWith('.rs'))) { const mod = f.replace(/\.rs$/, '').replace(/[\\/]mod$/, '').split(/[\\/]/).join('::');
  for (const m of readFileSync(join(DIR, f), 'utf8').matchAll(/^section! \{\n([\s\S]*?)^\}/gm)) {
    const head = m[1].match(/^((?:\s*\/\/\/.*\n)*)\s*pub struct (\w+) \{\n([\s\S]*)\}\s*$/) ?? die(`${f}: a section! of no form this reads`);
    const fields: Field[] = []; let doc: string[] = [];
    for (const l of head[3].split('\n')) { const t = l.trim(); if (!t) continue;
      if (t.startsWith('///')) { doc.push(t); continue; }
      const p = t.match(/^pub (\w+): (.+),$/) ?? die(`${f}: ${t}`), ty = p[2].replace(/\s/g, ''); fields.push({ name: p[1], ty, words: words(ty), doc: docOf(doc) }); doc = []; }
    rows.set(`${mod}::${head[2]}`, { name: head[2], doc: docOf(head[1].split('\n').filter(x => x.trim())), fields });
  }
}
const list = (readFileSync(join(DIR, 'lib.rs'), 'utf8').match(/^sections! \{\n([\s\S]*?)^\}/m) ?? die('no sections! in lib.rs'))[1].split('\n').map(l => l.replace(/\/\/.*/, '').trim()).filter(Boolean)
  .map((l, i) => { const m = l.match(/^(\d+) (\w+): ([\w:]+) = [\w.]+;$/) ?? die(`sections!: ${l}`); if (Number(m[1]) !== i) die(`sections!: ${m[2]} is line ${i} and says ${m[1]}`); return { name: m[2], row: rows.get(m[3]) ?? die(`sections!: no section! ${m[3]}`) }; });

// the layout as the module says it (lib.rs `layout`), and its stamp
const text = list.map(s => `${s.name}${s.row.fields.map(f => ` ${f.name}:${f.ty}`).join('')}\n`).join('');
let stamp = 0x811c9dc5; for (let i = 0; i < text.length; i++) stamp = Math.imul(stamp ^ text.charCodeAt(i), 0x01000193) >>> 0;

const camel = (s: string) => s.replace(/_([a-z0-9])/g, (_, c) => c.toUpperCase());
const L: string[] = ['// Written by tools/draw/gen.ts from rust/game_draw. To change a section, change it there.', '',
  '/** the stamp of the sections this was made from: a module of another stamp is refused (drawn.ts) */', `export const LAYOUT = ${stamp};`,
  '/** each section\'s place: in the table the module answers with, and its bit in what a host asks for */', `export const enum Section { ${list.map((s, i) => `${camel(s.name)} = ${i}`).join(', ')}, count = ${list.length} }`];
for (const r of new Set(list.map(s => s.row))) { let at = 0;
  L.push('', ...(r.doc ? [`/** ${r.doc} */`] : []), `export const enum ${r.name} {`);
  for (const f of r.fields) { const kind = f.ty.replace(/^\[/, '').slice(0, 3); L.push(`  /** ${kind}${f.words > 1 ? ` × ${f.words}` : ''}${f.doc ? `: ${f.doc}` : ''} */`, `  ${camel(f.name)} = ${at},`); at += f.words; }
  L.push(`  /** words a row */`, `  row = ${at},`, '}'); }
const made = L.join('\n') + '\n';
if (process.argv.includes('--check')) { const same = existsSync(OUT) && readFileSync(OUT, 'utf8') === made; console.log(same ? `draw: src/draw/sections.ts is of these declarations (stamp ${stamp})` : 'draw: src/draw/sections.ts would change'); process.exit(same ? 0 : 1); }
writeFileSync(OUT, made); console.log(`draw: src/draw/sections.ts written, ${list.length} sections, stamp ${stamp}`);
