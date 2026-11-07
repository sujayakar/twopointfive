// THE PAGE'S OFFSETS INTO WHAT RUST DRAWS: src/draw/sections.ts, made from the declarations in rust/game_draw (a `section!` a row, `sections!` the list). A
// row lies in the module's memory and is read there: `f[at + Fog.edge]` over one view of that memory as floats, `u[…]` over one as words (src/draw/drawn.ts).
// Every offset is a constant of the bundle. Nothing is copied and no object is made.
//   bun run tools/draw/gen.ts [--check]      write the files · --check: exit 1 where one would change
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
  for (const m of readFileSync(join(DIR, f), 'utf8').matchAll(/^(section|event)! \{\n([\s\S]*?)^\}/gm)) {
    const head = m[2].match(/^((?:\s*\/\/\/.*\n)*)\s*pub struct (\w+) \{\n([\s\S]*)\}\s*$/) ?? die(`${f}: a section! of no form this reads`);
    const fields: Field[] = m[1] === 'event' ? ['tick', 'serial'].map(name => ({ name, ty: 'u32', words: 1, doc: '' })) : []; let doc: string[] = [];
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
// (a word that crosses by its number: the Rust enum's arms, in their order, each by the word it has for itself)
const WORDS: [string, string, string, string][] = [['STUFFS', 'scene_stuff', 'BoxStuff', 'what a box may say it is made of'], ['PKINDS', 'render_particles', 'PKind', 'what a particle is'], ['EMIT_SHAPES', 'render_particles', 'EmitShape', 'how particles are thrown'], ['SMOKE_KINDS', 'smoke_types', 'SmokeEmitterKind', 'what smoke comes from'], ['FLOORS', 'floor', 'FloorMat', 'what a floor is']];
for (const [name, mod, en, doc] of WORDS) { const text = readFileSync(join(ROOT, `rust/game_types/src/${mod}/types.rs`), 'utf8'), arms = (text.match(new RegExp(`pub enum ${en} \\{\\n([\\s\\S]*?)\\n\\}`)) ?? die(`no enum ${en}`))[1].split('\n').map(l => l.trim().replace(/,$/, '')).filter(l => /^\w+$/.test(l));
  L.push('', `/** ${doc}, by its number in Rust */`, `export const ${name} = [${arms.map(a => `'${(text.match(new RegExp(`${en}::${a} => "([^"]+)"`)) ?? [0, a[0].toLowerCase() + a.slice(1)])[1]}'`).join(', ')}] as const;`); }
// ---- what goes the other way: the commands' writer, from their declaration
const COMMANDS = join(ROOT, 'src/draw/commands.ts'), C: string[] = ['// Written by tools/draw/gen.ts from rust/game_run/src/command.rs. To change a command, change it there.', "import { Writer } from './writer';", '',
  '/** what a player or a panel does that is not input, in its order, until a step takes it */', 'export class Commands extends Writer {'];
{ const src = (readFileSync(join(ROOT, 'rust/game_run/src/command.rs'), 'utf8').match(/^commands! \{\n([\s\S]*?)^\}/m) ?? die('no commands! in command.rs'))[1]; let doc = '', n = 0;
  for (const l of src.split('\n')) { const t = l.trim(); if (t.startsWith('///')) { doc = t.replace(/^\/\/\/ ?/, ''); continue; }
    const m = t.match(/^(\d+) (\w+) \{([^}]*)\} = /); if (!m) continue; if (Number(m[1]) !== n++) die(`commands!: ${m[2]} says ${m[1]}`);
    const F = m[3].split(',').map(x => x.trim()).filter(Boolean).map(x => { const [name, ty] = x.split(':').map(y => y.trim()); const k = ({ u32: 'u32', i32: 'i32', f64: 'f64', '&str': 'text' } as Record<string, string>)[ty] ?? die(`commands!: ${ty}`); return { name: camel(name), k }; });
    C.push(`  /** ${doc} */`, `  ${m[2][0].toLowerCase()}${m[2].slice(1)}(${F.map(f => `${f.name}: ${f.k === 'text' ? 'string' : 'number'}`).join(', ')}): void { this.u32(${m[1]});${F.map(f => ` this.${f.k}(${f.name});`).join('')} }`); doc = ''; }
  C.push('}', ''); }
// ---- every number a person may turn while the game plays, by its name: where it lies in the knobs, from the wire's table (a settled row is the build's own)
const KNOBS = join(ROOT, 'rust/game_run/src/knobs_list.rs'); let knobText = '';
{ const S = JSON.parse(readFileSync(join(ROOT, 'tools/blitz/schema.json'), 'utf8')), T = S.types as Record<string, { k: string; fields: { rust: string; ty: string }[] }>, settled = new Set<string>(S.knobSettled), rows: string[] = [], typed: string[] = [];
  const PARTS: Record<string, number> = { f64: 1, bool: 1, V3: 3, '[f64; 2]': 2, '[f64; 3]': 3, '[f64; 4]': 4 };
  const walk = (path: string, ty: string): void => { const t = T[ty];
    if (t?.k === 'struct') { for (const f of t.fields) if (!settled.has(`${ty.replace('crate::', '')}.${f.rust}`)) walk(path ? `${path}.${f.rust}` : f.rust, f.ty); return; }
    const n = PARTS[ty]; if (n) typed.push(`    let _: &${ty === 'V3' ? '[f64; 3]' : ty} = &k.${path};`);
    if (n) for (let i = 0; i < n; i++) rows.push(`    ("${path}${n > 1 ? `.${i}` : ''}", offset_of!(Knobs, ${path})${i ? ` + ${8 * i}` : ''}, ${ty === 'bool'}),`); };
  walk('', 'crate::knobs::Knobs');
  knobText = `//! Written by tools/draw/gen.ts from tools/blitz/schema.json: every number of the knob tables that is not settled, by its name, where it lies, and whether it is a yes or no.\nuse core::mem::offset_of;\nuse game::knobs::Knobs;\n\npub static KNOBS: &[(&str, usize, bool)] = &[\n${rows.join('\n')}\n];\n\n/// each is of the type the list's reader takes it for\n#[cfg(test)]\n#[allow(dead_code)]\nfn typed(k: &Knobs) {\n${typed.join('\n')}\n}\n`; }
const files: [string, string][] = [[KNOBS, knobText], [OUT, L.join('\n') + '\n'], [COMMANDS, C.join('\n')]];
if (process.argv.includes('--check')) { const off = files.filter(([p, t]) => !existsSync(p) || readFileSync(p, 'utf8') !== t).map(([p]) => p.slice(ROOT.length + 1)); console.log(off.length ? `draw: ${off.join(', ')} would change` : `draw: the page's half is of these declarations (stamp ${stamp})`); process.exit(off.length ? 1 : 0); }
for (const [p, t] of files) writeFileSync(p, t);
console.log(`draw: ${list.length} sections, stamp ${stamp}; src/draw/sections.ts and commands.ts written`);
