// THE VIEW'S READER (src/view/read.ts), made from the declarations in rust/game_types/src/view: each type and how its bytes are read (`#[derive(ToBytes)]`: fields in their order, an enum's arm by its place).
// No type under it names a row of the game's: a man is named by his character's id, a door by its name (an ID here is refused).
import { readdirSync, readFileSync } from 'node:fs';
import { join } from 'node:path';

interface Declared { name: string; fields?: { name: string; ty: string }[]; words?: string[] }
const NUMBERS: Record<string, [string, number]> = { f64: ['Float64', 8], f32: ['Float32', 4], u32: ['Uint32', 4], usize: ['Uint32', 4], i32: ['Int32', 4], u16: ['Uint16', 2], i16: ['Int16', 2], u8: ['Uint8', 1], i8: ['Int8', 1] };
const camel = (s: string): string => s.replace(/^r#/, '').replace(/_$/, '').replace(/_([a-z0-9])/g, (_, c) => c.toUpperCase());
/** what stands between the commas of a list, those inside brackets left alone */
function split(t: string): string[] { const out: string[] = []; let d = 0, at = 0; for (let i = 0; i <= t.length; i++) { const c = t[i]; if (c === undefined || (c === ',' && d === 0)) { if (t.slice(at, i).trim()) out.push(t.slice(at, i).trim()); at = i + 1; } else if ('<([{'.includes(c)) d++; else if ('>)]}'.includes(c)) d--; } return out; }
/** the attributes a field or an arm opens with, and the rest of it */
function lead(r: string): { attrs: string; rest: string } { let at = 0; while (r.startsWith('#[', at)) { let end = at + 2; for (let d = 1; d; end++) d += r[end] === '[' ? 1 : r[end] === ']' ? -1 : 0; at = end; while (/\s/.test(r[at] ?? '')) at++; } return { attrs: r.slice(0, at), rest: r.slice(at) }; }
/** what stands between a brace that opened before `at` and the one that closes it */
function inside(s: string, at: number): string { let end = at; for (let d = 1; d; end++) d += s[end] === '{' ? 1 : s[end] === '}' ? -1 : 0; return s.slice(at, end - 1); }

export function viewReader(root: string, die: (why: string) => never): string {
  const types = join(root, 'rust/game_types/src'), sources = new Map<string, string>(), known = new Map<string, Declared>();
  const source = (mod: string): string => { let s = sources.get(mod); if (s === undefined) { s = readdirSync(join(types, mod)).filter(f => f.endsWith('.rs')).map(f => readFileSync(join(types, mod, f), 'utf8')).join('\n').replace(/\/\/[^\n]*/g, ''); sources.set(mod, s); } return s; };
  const declared = (path: string): Declared => {
    const [, mod, name] = path.split('::'), s = source(mod), m = new RegExp(`pub (struct|enum) ${name} \\{`).exec(s) ?? die(`the view names ${path}, which is no struct or enum with a body in rust/game_types/src/${mod}`);
    const rows = split(inside(s, m.index + m[0].length)).map(lead);
    if (m[1] === 'struct') return { name, fields: rows.filter(r => !r.attrs.includes('#[bytes(skip)]')).map(({ rest: r }) => { const f = /^(?:pub(?:\([a-z]+\))? )?((?:r#)?\w+)\s*:\s*([^]+)$/.exec(r) ?? die(`${path}: ${r}`); return { name: f[1], ty: f[2].trim() }; }) };
    const own = [...s.matchAll(new RegExp(`^impl (?:[\\w:]+ for )?${name} \\{`, 'gm'))].map(m => inside(s, m.index + m[0].length)).join('\n');   // (a word is the enum's own, where it says one)
    return { name, words: rows.map(({ attrs, rest: r }) => { if (!/^\w+$/.test(r)) die(`${path}: a view's enum is plain words, and ${r} holds something`); return /#\[word\("([^"]+)"\)\]/.exec(attrs)?.[1] ?? new RegExp(`\\b(?:${name}|Self)::${r} => "([^"]+)"`).exec(own)?.[1] ?? r[0].toLowerCase() + r.slice(1); }) };
  };
  const list: string[] = [], seen = new Set<string>();
  const visit = (path: string): void => { if (seen.has(path)) return; seen.add(path); const d = declared(path); known.set(path, d); for (const f of d.fields ?? []) for (const m of full(path, f.ty).matchAll(/crate::\w+::\w+/g)) if (m[0] !== 'crate::v3::m4') visit(m[0]); list.push(path); };
  /** a type as it is named in full (a declaration names its own module's types bare) */
  const full = (at: string, ty: string): string => ty.replace(/(?<![\w:])([A-Z]\w+)/g, (w, n) => /^(Option|Vec|String|V3)$/.test(n) ? w : new RegExp(`pub (struct|enum) ${n} `).test(source(at.split('::')[1])) ? `crate::${at.split('::')[1]}::${n}` : w);
  for (const m of source('view').matchAll(/^pub (?:struct|enum) (\w+) \{/gm)) if (!/HostNow/.test(m[1])) visit(`crate::view::${m[1]}`);
  const names = new Map<string, string>(); for (const p of list) { const n = known.get(p)!.name, had = names.get(n); if (had) die(`the view's types ${had} and ${p} have one name`); names.set(n, p); }
  const of = (t0: string, where: string): { ts: string; rd: string } => { const t = t0.trim(); let m: RegExpExecArray | null;
    if (NUMBERS[t]) return { ts: 'number', rd: `I.${NUMBERS[t][0].toLowerCase()}()` };
    if (t === 'bool') return { ts: 'boolean', rd: 'I.bool()' }; if (t === 'String' || t === "&'static str") return { ts: 'string', rd: 'I.str()' };
    if (t === 'V3') return { ts: 'Vec3', rd: '[I.float64(), I.float64(), I.float64()]' };
    if ((m = /^Option<([^]*)>$/.exec(t))) { const x = of(m[1], where); return { ts: `${x.ts} | null`, rd: `I.bool() ? ${x.rd} : null` }; }
    if ((m = /^Vec<([^]*)>$/.exec(t))) { const x = of(m[1], where); return { ts: `${/[ |]/.test(x.ts) ? `(${x.ts})` : x.ts}[]`, rd: `I.list(() => ${x.rd})` }; }
    if ((m = /^\[(.+); (\d+)\]$/.exec(t))) { const x = of(m[1], where); return { ts: `[${Array(+m[2]).fill(x.ts).join(', ')}]`, rd: `[${Array(+m[2]).fill(x.rd).join(', ')}]` }; }
    if ((m = /^\(([^]*)\)$/.exec(t))) { const xs = split(m[1]).map(x => of(x, where)); return { ts: `[${xs.map(x => x.ts).join(', ')}]`, rd: `[${xs.map(x => x.rd).join(', ')}]` }; }
    if (known.has(t)) { const n = known.get(t)!.name; return { ts: n, rd: `read${n}In(I)` }; }
    return die(`${where}: a view's field is a number, a word, a truth, a place, a list or an option of those, or another of its types; not ${t}${/Id$/.test(t) ? ' (it names no row: a man is his character\'s id, a door its name)' : ''}`); };
  const L = ['// THE VIEW, READ: the types of rust/game_types/src/view and their bytes, as plain values. Written by tools/draw/gen.ts. To change one, change it there.', "import type { Vec3 } from '../math/vec';", '',
    'class In { p = 0; dv: DataView; constructor(readonly b: Uint8Array) { this.dv = new DataView(b.buffer, b.byteOffset, b.byteLength); }',
    ...[...new Map(Object.values(NUMBERS))].map(([f, w]) => `  ${f.toLowerCase()}(): number { const v = this.dv.get${f}(this.p${w > 1 ? ', true' : ''}); this.p += ${w}; return v; }`),
    '  bool(): boolean { return this.b[this.p++] !== 0; }', '  str(): string { const n = this.uint32(), s = DEC.decode(this.b.subarray(this.p, this.p + n)); this.p += n; return s; }',
    '  list<T>(f: () => T): T[] { const n = this.uint32(), o: T[] = []; for (let i = 0; i < n; i++) o.push(f()); return o; }',
    "  word<T>(W: readonly T[], of: string): T { const k = this.uint16(); if (k >= W.length) throw new Error(`the view: no such word of ${of} (${k})`); return W[k]; } }", 'const DEC = new TextDecoder();', ''];
  for (const p of list) { const { name: n, fields, words } = known.get(p)!;
    if (words) { L.push(`export type ${n} = ${words.map(w => `'${w}'`).join(' | ')};`, `const ${n}Words: readonly ${n}[] = [${words.map(w => `'${w}'`).join(', ')}];`, `function read${n}In(I: In): ${n} { return I.word(${n}Words, '${n}'); }`); continue; }
    const F = fields!.map(f => ({ k: camel(f.name), ...of(full(p, f.ty), `${p}.${f.name}`) }));
    L.push(`export interface ${n} { ${F.map(f => `${f.k}: ${f.ts}`).join('; ')} }`, `function read${n}In(I: In): ${n} { return { ${F.map(f => `${f.k}: ${f.rd}`).join(', ')} }; }`,
      `export function read${n}(b: Uint8Array): ${n} { const I = new In(b), r = read${n}In(I); if (I.p !== b.length) throw new Error(\`the view: ${n} is \${b.length} bytes and \${I.p} were read\`); return r; }`); }
  return L.join('\n') + '\n';
}
