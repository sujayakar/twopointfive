// THE MEN'S COLUMNS, WRITTEN FROM THEIR SCHEMA (src/sim/men/schema.ts) — one table, two languages:
//   bun run tools/sim/men.ts            write src/sim/men/cols.ts and rust/sim/src/men.rs (then `bun run tools/sim/build.ts`: the module is built from the Rust)
//   bun run tools/sim/men.ts --check    are the two files what the table writes? exit 0 yes · 1 no
// THE LAYOUT (rust/sim/src/items.rs is the same kind): a head of four words, then a column a number, every column MEN_ROWS rows long: the doubles, hot before cold, then the
// words, hot before cold. A column is whole vectors on a 16-byte boundary (the rows are a multiple of four, the head 16 bytes), so a walk over the men can read it four
// lanes at a time. The first word column is the row's generation, which is the store's own (src/sim/men/store.ts): odd = a man stands in the row.
import { readFileSync, writeFileSync } from 'node:fs';
import { resolve } from 'node:path';
import { MEN_FIELDS, MEN_KEPT, MEN_ROWS, type ColGroup, type ColOn, type ColType, type ManRef, type MenField } from '../../src/sim/men/schema';

export interface Schema { rows: number; fields: readonly MenField[] }
/** a column as both languages see it */
export interface Col { name: string; rust: string; kind: 'f64' | 'u32'; group: ColGroup }
/** where a field's values are, once laid out */
interface Laid { field: MenField; cols: string[]; bit?: { col: string; mask: number }; has?: { col: string; mask: number } }
const RANK: Record<ColGroup, number> = { hot: 0, cold: 1, animator: 2 };
const snake = (s: string) => s.replace(/[A-Z]/g, c => `_${c.toLowerCase()}`);

/** the columns in the part's order, and where each field is in them */
export function layOut(S: Schema): { cols: Col[]; laid: Laid[] } {
  const f64: Col[] = [], u32: Col[] = [], laid: Laid[] = [];
  const add = (to: Col[], name: string, group: ColGroup) => { to.push({ name, rust: snake(name), kind: to === f64 ? 'f64' : 'u32', group }); return name; };
  const fields = [...S.fields].sort((a, b) => RANK[a.group] - RANK[b.group]);   // (a stable sort: within a group, the table's order)
  /** a truth wants a bit: the current word of its group's, or a new word */
  const words = new Map<ColGroup, { col: string; used: number }>(); let nw = 0;
  const bit = (g: ColGroup) => {
    let w = words.get(g); if (!w || w.used === 32) { w = { col: add(u32, `truths${nw++}`, g), used: 0 }; words.set(g, w); }
    const mask = (1 << w.used++) >>> 0; return { col: w.col, mask };
  };
  for (const f of fields) {
    if (f.type === 'f64') laid.push({ field: f, cols: [add(f64, f.name, f.group)] });
    else if (f.type === 'vec3' || f.type === 'vec3?') {
      const cols = ['X', 'Y', 'Z'].map(a => add(f64, f.name + a, f.group));
      laid.push({ field: f, cols, ...(f.type === 'vec3?' ? { has: bit(f.group) } : {}) });
    } else if (f.type === 'bool') laid.push({ field: f, cols: [], bit: bit(f.group) });
    else laid.push({ field: f, cols: [add(u32, f.name, f.group)] });   // (a word: a place in a list of words, a word of its own, or a man)
  }
  const cols = [...f64, { name: 'gen', rust: 'gen', kind: 'u32' as const, group: 'hot' as const }, ...u32];
  return { cols, laid };
}

/** FNV-1a over what decides the bytes: the rows, and every column and truth in its place. The hash both sides hold */
export function schemaHash(S: Schema): number {
  const { cols, laid } = layOut(S), text = JSON.stringify([S.rows, cols.map(c => [c.name, c.kind]), laid.map(l => [l.field.name, l.field.on, l.field.type, l.bit?.mask ?? 0, l.has?.mask ?? 0])]);
  let h = 0x811c9dc5;
  for (let i = 0; i < text.length; i++) h = Math.imul(h ^ text.charCodeAt(i), 0x01000193) >>> 0;
  return h >>> 0;
}

const HEAD = 16;
/** where a column starts: an index into the doubles for a double, into the words for a word, from the part's start */
function offsets(S: Schema, cols: Col[]): Map<string, number> {
  const out = new Map<string, number>(); let at = HEAD;
  for (const c of cols) { out.set(c.name, c.kind === 'f64' ? at / 8 : at / 4); at += S.rows * (c.kind === 'f64' ? 8 : 4); }
  return out;
}
const INIT: Record<ColOn, string> = { man: 'initMan', guard: 'initGuard', player: 'initPlayer', anim: 'initAnim' };
const isManRef = (t: ColType): t is ManRef => typeof t === 'object' && !Array.isArray(t);
const initText = (f: MenField) => (typeof f.init === 'number' && Number.isNaN(f.init) ? 'NaN' : JSON.stringify(f.init));

function accessors(l: Laid, O: Map<string, number>): string[] {
  const f = l.field, n = f.name, t = f.type, doc = f.why ? [`  /** ${f.why} */`] : [];
  const d = (c: string) => `S.d[S.b8 + ${O.get(c)} + this.r]`, u = (c: string) => `S.u[S.b4 + ${O.get(c)} + this.r]`;
  const num = (name: string, read: string, write: string, type = 'number') => `  get ${name}(): ${type} { const S = this.S; return ${read}; } set ${name}(v: ${type}) { const S = this.S; ${write}; }`;
  if (t === 'f64') return [...doc, num(n, d(n), `${d(n)} = v`)];
  if (t === 'u32') return [...doc, num(n, u(n), `${u(n)} = v`)];
  if (t === 'i32') return [...doc, num(n, `${u(n)} | 0`, `${u(n)} = v`)];
  if (t === 'bool') { const b = l.bit!; return [...doc, num(n, `(${u(b.col)} & ${b.mask}) !== 0`, `const w = ${u(b.col)}; ${u(b.col)} = v ? w | ${b.mask} : w & ${~b.mask >>> 0}`, 'boolean')]; }
  if (t === 'vec3' || t === 'vec3?') {
    const [x, y, z] = l.cols, each = l.cols.map(c => num(c, d(c), `${d(c)} = v`));
    if (t === 'vec3') return [...doc, `  get ${n}(): Vec3 { const S = this.S, i = S.b8 + this.r; return [S.d[i + ${O.get(x)}], S.d[i + ${O.get(y)}], S.d[i + ${O.get(z)}]]; }`,
      `  set ${n}(v: Readonly<Vec3>) { const S = this.S, i = S.b8 + this.r; S.d[i + ${O.get(x)}] = v[0]; S.d[i + ${O.get(y)}] = v[1]; S.d[i + ${O.get(z)}] = v[2]; }`, ...each];
    const h = l.has!;
    return [...doc, `  get ${n}(): Vec3 | null { const S = this.S; if ((${u(h.col)} & ${h.mask}) === 0) return null; const i = S.b8 + this.r; return [S.d[i + ${O.get(x)}], S.d[i + ${O.get(y)}], S.d[i + ${O.get(z)}]]; }`,
      `  set ${n}(v: Readonly<Vec3> | null) { const S = this.S, w = ${u(h.col)}, i = S.b8 + this.r; if (v === null) { ${u(h.col)} = w & ${~h.mask >>> 0}; S.d[i + ${O.get(x)}] = 0; S.d[i + ${O.get(y)}] = 0; S.d[i + ${O.get(z)}] = 0; return; } ${u(h.col)} = w | ${h.mask}; S.d[i + ${O.get(x)}] = v[0]; S.d[i + ${O.get(y)}] = v[1]; S.d[i + ${O.get(z)}] = v[2]; }`, ...each];
  }
  if (isManRef(t)) {   // a word that names a man: 0 nobody · 1, 2 … a word of its list · else a handle (never under 256: a live row's generation is odd)
    const also = t.also, ty = [...also.map(w => `'${w}'`), t.man, 'null'].join(' | '), rd = also.map((w, i) => `h === ${i + 1} ? '${w}' : `).join(''), wr = also.map((w, i) => `v === '${w}' ? ${i + 1} : `).join('');
    return [...doc, `  get ${n}(): ${ty} { const S = this.S, h = ${u(n)}; return h === 0 ? null : ${rd}S.manAt(h) as ${t.man}; } set ${n}(v: ${ty}) { const S = this.S; ${u(n)} = v === null ? 0 : ${wr}S.handleOf((v as ${t.man}).r); }`];
  }
  const V = `${n.toUpperCase()}_WORDS`;
  return [...doc, `  get ${n}(): (typeof ${V})[number] { const S = this.S; return ${V}[${u(n)}]; } set ${n}(v: (typeof ${V})[number]) { const S = this.S, i = ${V}.indexOf(v); if (i < 0) throw new Error(\`${n}: no such word: \${v}\`); ${u(n)} = i; }`];
}

/** the TypeScript side */
export function tsText(S: Schema): string {
  const { cols, laid } = layOut(S), O = offsets(S, cols), f64 = cols.filter(c => c.kind === 'f64'), u32 = cols.filter(c => c.kind === 'u32');
  const body = (on: ColOn) => laid.filter(l => l.field.on === on).flatMap(l => accessors(l, O));
  /** what a new man's columns start at, other than zero: a method of the class his row is read through, which his object's constructor calls */
  const inits = (on: ColOn) => { const rows = S.fields.filter(f => f.on === on && f.init !== undefined); return rows.length ? [`  ${INIT[on]}(): void { ${rows.map(f => `this.${f.name} = ${initText(f)};`).join(' ')} }`] : []; };
  const lists = laid.filter(l => Array.isArray(l.field.type)).map(l => `const ${l.field.name.toUpperCase()}_WORDS = ${JSON.stringify(l.field.type)} as const;`);
  const uses3 = laid.some(l => l.field.type === 'vec3' || l.field.type === 'vec3?'), mans = [...new Set(laid.flatMap(l => (isManRef(l.field.type) ? [l.field.type.man] : [])))];
  return [
    `// WRITTEN BY tools/sim/men.ts FROM src/sim/men/schema.ts — never by hand (tools/qa/men-check.ts is red on a difference). A man's columns, as the game reads and writes them.`,
    uses3 ? `import type { Vec3 } from '../../math/vec';` : ``,
    ...(mans.length ? [`import type { ${mans.join(', ')} } from '../../game/game';   // (a type alone: nothing of the game is loaded from here)`] : []),
    `import type { MenStore } from './store';`,
    ``,
    `/** the hash of the layout; the module holds the same one (rust/sim/src/men.rs SCHEMA), and store.ts holds the two to each other as a game starts */`,
    `export const MEN_SCHEMA = 0x${schemaHash(S).toString(16)};`,
    `export const MEN_HEAD = ${HEAD};`,
    `export const MEN_F64 = ${JSON.stringify(f64.map(c => c.name))} as const;`,
    `export const MEN_U32 = ${JSON.stringify(u32.map(c => c.name))} as const;`,
    `/** the generation column, as an index into the words from the part's start (the store's own) */`,
    `export const MEN_GEN = ${O.get('gen')};`,
    ...lists,
    ``,
    `/** what every man is: which store his columns are in and his row. Character and Guard extend these, so a column reads as a field */`,
    `export class ManCols {`,
    `  S!: MenStore; r = -1;`,
    ...body('man'),
    ...inits('man'),
    `}`,
    `/** a Guard record's own columns (a guard, a teammate, a civilian) */`,
    `export class GuardCols extends ManCols {`,
    ...body('guard'),
    ...inits('guard'),
    `}`,
    `/** the player's own columns */`,
    `export class PlayerCols extends ManCols {`,
    ...body('player'),
    ...inits('player'),
    `}`,
    `/** the animator's kept state */`,
    `export class AnimCols {`,
    `  S!: MenStore; r = -1;`,
    ...body('anim'),
    `}`,
    ``,
  ].filter((l, i) => l !== '' || i > 0).join('\n');
}

/** the Rust side */
export function rustText(S: Schema): string {
  const { cols } = layOut(S);
  const size = HEAD + cols.reduce((n, c) => n + S.rows * (c.kind === 'f64' ? 8 : 4), 0);
  return [
    `//! WRITTEN BY tools/sim/men.ts FROM src/sim/men/schema.ts — never by hand (tools/qa/men-check.ts is red on a difference). A man's columns, as a rule here reads them.`,
    `//! A man is a row: \`state(g).men.<column>[row]\`. The host writes the columns through src/sim/men/cols.ts, in these same bytes, with no copy.`,
    `use core::mem::{offset_of, size_of};`,
    ``,
    `/// rows of each column (src/sim/men/schema.ts MEN_ROWS)`,
    `pub const ROWS: usize = ${S.rows};`,
    `/// the hash of the layout: src/sim/men/cols.ts MEN_SCHEMA is the same number`,
    `pub const SCHEMA: u32 = 0x${schemaHash(S).toString(16)};`,
    ``,
    `#[repr(C)]`,
    `pub struct Men {`,
    `    pub spare: [u32; ${HEAD / 4}],`,
    ...cols.map(c => `    pub ${c.rust}: [${c.kind}; ROWS],`),
    `}`,
    `const _: () = assert!(size_of::<Men>() == ${size});`,
    ``,
    `/// where each column is in the part, in bytes, in the order of the schema's layout (the host checks these against its own)`,
    `pub const COLS: [usize; ${cols.length}] = [${cols.map(c => `offset_of!(Men, ${c.rust})`).join(', ')}];`,
    `// every column is whole vectors from a 16-byte boundary: the part starts on one (state.rs), the head is 16 bytes, the rows a multiple of four`,
    `const _: () = assert!(ROWS % 4 == 0);`,
    `const _: () = { let mut i = 0; while i < COLS.len() { assert!(COLS[i] % 16 == 0); i += 1; } };`,
    ``,
    `#[no_mangle]`,
    `pub extern "C" fn men_schema() -> u32 { SCHEMA }`,
    `/// column \`i\`'s place in the part in bytes, or the part's rows (\`i\` = the number of columns) · 0 past that`,
    `#[no_mangle]`,
    `pub extern "C" fn men_col(i: u32) -> usize { let i = i as usize; if i < COLS.len() { COLS[i] } else if i == COLS.len() { ROWS } else { 0 } }`,
    `/// EVERY NUMBER OF A MAN'S ROW AS THIS MODULE READS IT, folded (FNV-1a over the words, the columns in their order): what tools/qa/men-check.ts holds to the TypeScript's accessors`,
    `#[no_mangle]`,
    `pub extern "C" fn men_fold(g: u32, row: u32) -> u32 {`,
    `    let m = &crate::state::state(g as usize).men;`,
    `    let r = row as usize % ROWS;`,
    `    let mut h: u32 = 0x811c9dc5;`,
    `    let mut w = |v: u32| { h = (h ^ v).wrapping_mul(0x01000193); };`,
    ...cols.map(c => c.kind === 'f64' ? `    let b = m.${c.rust}[r].to_bits(); w(b as u32); w((b >> 32) as u32);` : `    w(m.${c.rust}[r]);`),
    `    h`,
    `}`,
    ``,
  ].join('\n');
}

const ROOT = resolve(import.meta.dir, '../..');
export const MEN_OUT = { ts: resolve(ROOT, 'src/sim/men/cols.ts'), rust: resolve(ROOT, 'rust/sim/src/men.rs') };
export const MEN_TABLE: Schema = { rows: MEN_ROWS, fields: MEN_FIELDS };
/** a name twice, a kept field that is also a column, rows that are no multiple of four: the table's own mistakes */
export function schemaFaults(S: Schema, kept: readonly { name: string; on: ColOn }[] = MEN_KEPT): string[] {
  const out: string[] = [], seen = new Set<string>();
  if (S.rows % 4 !== 0) out.push(`${S.rows} rows is no multiple of four`);
  for (const f of S.fields) { const k = `${f.on}.${f.name}`; if (seen.has(k)) out.push(`${k} is in the table twice`); seen.add(k); if (f.name === 'S' || f.name === 'r' || f.name === 'gen') out.push(`${k}: the name is the store's`); }
  for (const f of kept) if (seen.has(`${f.on}.${f.name}`)) out.push(`${f.on}.${f.name} is a column and kept on the object`);
  return out;
}

if (import.meta.main) {
  const faults = schemaFaults(MEN_TABLE); if (faults.length) { console.error(`src/sim/men/schema.ts:\n  ${faults.join('\n  ')}`); process.exit(1); }
  const ts = tsText(MEN_TABLE), rust = rustText(MEN_TABLE);
  if (process.argv.includes('--check')) {
    const same = (p: string, t: string) => { try { return readFileSync(p, 'utf8') === t; } catch { return false; } };
    const off = [same(MEN_OUT.ts, ts) ? '' : MEN_OUT.ts, same(MEN_OUT.rust, rust) ? '' : MEN_OUT.rust].filter(Boolean);
    if (off.length) { console.error(`not what src/sim/men/schema.ts writes: ${off.join(', ')} (bun run tools/sim/men.ts)`); process.exit(1); }
    console.log(`the men's columns are what the schema writes (${layOut(MEN_TABLE).cols.length} columns, hash ${schemaHash(MEN_TABLE).toString(16)})`);
  } else {
    writeFileSync(MEN_OUT.ts, ts); writeFileSync(MEN_OUT.rust, rust);
    console.log(`wrote the men's columns: ${layOut(MEN_TABLE).cols.length} columns, hash ${schemaHash(MEN_TABLE).toString(16)}`);
  }
}
