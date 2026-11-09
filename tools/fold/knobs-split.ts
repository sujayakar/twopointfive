// THE KNOB TABLES, ONE FILE A FAMILY: rust/game_types/src/knobs.rs held every table three times over: its fields, a body at zero, and a literal of the values far below. This writes
// rust/game_types/src/knobs/<family>.rs, a family being the rules file the table was made for: each table says its fields with their values and what the old table said of them, and
// #[derive(Knobs)] (rust/game_derive) writes the rest. Run once.
//   TS_LIB=<typescript.js> bun run tools/fold/knobs-split.ts
import { existsSync, readFileSync, readdirSync, statSync, writeFileSync, mkdirSync, rmSync } from 'node:fs';
import { createRequire } from 'node:module';
import { execSync } from 'node:child_process';
const ts = createRequire(import.meta.url)(process.env.TS_LIB ?? 'vendor/typescript/lib/typescript.js');
const OUT = 'rust/game_types/src/knobs', SRC = 'rust/game_types/src/knobs.rs';
const text = readFileSync(SRC, 'utf8');
// (which rules file a table was made for: the tables said so until their zero bodies went)
const madeFor = new Map([...execSync(`git show f7a4157b9:${SRC}`, { encoding: 'utf8', maxBuffer: 1 << 28 }).matchAll(/inline at (src\/[\w/.-]+):(\d+) ([\w.]+)[^\n]*\n(?:[^\n]*\n)*?pub struct (\w+) \{/g)].map(m => [m[4], { file: m[1], line: +m[2], from: m[3] }]));

// ---- the Rust as it stands
interface Field { name: string; ty: string; value?: string; nested?: string; doc: string[] }
interface Table { name: string; derive: string; file: string; line: number; from: string; fields: Field[]; doc: string[]; family: string }
const tables = new Map<string, Table>(), enums: { name: string; text: string; family?: string }[] = [];
const item = /(?:^\/\/\/[^\n]*\n)*^#\[derive\(([^)]*)\)\]\npub struct (\w+) \{\n(.*?)^\}\n/gms;
for (const m of text.matchAll(item)) {
    if (m[2] === 'Knobs') continue;
  const at = madeFor.get(m[2]); if (!at) throw new Error(`${m[2]}: from where?`);
  const fields = [...m[3].matchAll(/^\s*pub (\w+): (.+),$/gm)].map(f => ({ name: f[1], ty: f[2].replace(/crate::knobs::/g, ''), doc: [] as string[] }));
  tables.set(m[2], { name: m[2], derive: m[1].replace(/,?\s*\bDefault\b/, ''), file: at.file, line: at.line, from: at.from, fields, doc: [], family: at.file.replace(/^.*\//, '').replace(/\.ts$/, '').replace(/([a-z0-9])([A-Z])/g, '$1_$2').toLowerCase() });
}
for (const m of text.matchAll(/(?:^\/\/\/[^\n]*\n)*(^#\[derive\([^)]*\)\]\n)((?:#\[[^\n]*\n)*pub enum (\w+) \{\n.*?^\}\n)((?:^impl[^\n]*\{[^\n]*\}\n|^impl[^\n]*\{\n.*?^\}\n)*)/gms)) enums.push({ name: m[3], text: m[1] + m[2] + m[4] });
const after = text.slice(text.indexOf('pub fn page_aim'));
const pageOf = new Map<string, string>();
for (const m of after.matchAll(/^pub fn (page_\w+)\(\) -> (\w+) \{/gm)) pageOf.set(m[1], m[2]);

/** a struct literal's fields: name and the text of its value */
function fieldsOf(s: string, at: number): { fields: [string, string][]; end: number } {
  const out: [string, string][] = []; let i = at;
  const skip = () => { while (/\s/.test(s[i])) i++; };
  for (;;) { skip(); if (s[i] === '}') return { fields: out, end: i + 1 };
    const m = /^(\w+):\s*/.exec(s.slice(i, i + 200)); if (!m) throw new Error(`at ${s.slice(i, i + 80)}`); i += m[0].length;
    const from = i; let depth = 0, str = false;
    for (;; i++) { const c = s[i]; if (str) { if (c === '\\') i++; else if (c === '"') str = false; continue; } if (c === '"') { str = true; continue; }
      if (c === '(' || c === '[' || c === '{') depth++; else if (c === ')' || c === ']' || c === '}') { if (depth === 0) break; depth--; } else if (c === ',' && depth === 0) break; }
    out.push([m[1], s.slice(from, i).trim()]); if (s[i] === ',') i++; }
}
const seen = new Set<string>();
function lay(name: string, s: string, at: number): number {
  const t = tables.get(name); if (!t) throw new Error(`no table ${name}`); seen.add(name);
  const { fields, end } = fieldsOf(s, at);
  if (fields.length !== t.fields.length) throw new Error(`${name}: ${fields.length} values for ${t.fields.length} fields`);
  for (const [k, v] of fields) { const f = t.fields.find(x => x.name === k); if (!f) throw new Error(`${name}.${k}`);
    const n = /^crate::knobs::(\w+) \{/.exec(v);
    if (n && tables.has(n[1])) { if (f.ty !== n[1]) throw new Error(`${name}.${k}: ${f.ty} holds ${n[1]}`); lay(n[1], v, n[0].length); f.nested = n[1]; }
    else { if (/crate::knobs::\w+ \{/.test(v)) throw new Error(`${name}.${k}: a table inside an expression: ${v.slice(0, 100)}`); f.value = v.replace(/crate::knobs::/g, ''); } }
  return end;
}
for (const m of after.matchAll(/^pub fn page_\w+\(\) -> \w+ \{\n\s*crate::knobs::(\w+) \{/gm)) lay(m[1], after, m.index! + m[0].length);
for (const t of tables.keys()) if (!seen.has(t)) throw new Error(`${t} has no values`);

// ---- what the TypeScript the tables were written from said of each row, as it stood at f7a4157b9
const parsed = new Map<string, any>();
const old = (f: string): string | null => { try { return execSync(`git show f7a4157b9:${f}`, { encoding: 'utf8', maxBuffer: 1 << 28, stdio: ['ignore', 'pipe', 'ignore'] }); } catch { return null; } };
const parse = (f: string) => { if (!parsed.has(f)) parsed.set(f, ts.createSourceFile(f, old(f) ?? '', ts.ScriptTarget.Latest, true)); return parsed.get(f); };
const clean = (c: string): string[] => {
  if (c.startsWith('//')) return [c.replace(/^\/\/+\s?/, '').trimEnd()].filter(Boolean);
  return c.replace(/^\/\*+\s?/, '').replace(/\s*\*+\/$/, '').split('\n').map(l => l.replace(/^\s*\*\s?/, '').trimEnd()).filter(Boolean);
};
const low = (s: string) => s.replace(/_/g, '').toLowerCase();
const unwrap = (e: any): any => { while (e && (ts.isAsExpression(e) || ts.isParenthesizedExpression(e) || ts.isSatisfiesExpression?.(e))) e = e.expression; if (e && ts.isCallExpression(e) && e.arguments.length) return unwrap(e.arguments[0]); return e; };
/** the literal a table was written from, by the name the old table gave: `AIM`, a const of its file, or `AimK.guard`, a row of its parent's literal. The line it gave has moved since */
const literalOf = new Map<string, any>();
function literal(t: Table): any {
  if (literalOf.has(t.name)) return literalOf.get(t.name);
  let lit: any = null; const [head, row] = t.from.split('.');
  {
    if (row) { const parent = tables.get(head), up = parent && literal(parent); if (up) { const p = up.properties.find((p: any) => p.name?.getText() === row); if (p && ts.isPropertyAssignment(p)) { const v = unwrap(p.initializer); if (ts.isObjectLiteralExpression(v)) lit = v; } } }
    else { const named = (name: string): any => { let found: any = null; const visit = (n: any): void => { if (ts.isVariableDeclaration(n) && n.name.getText() === name && n.initializer) { const v = unwrap(n.initializer); if (ts.isObjectLiteralExpression(v)) found = v; } if (!found) ts.forEachChild(n, visit); }; visit(parse(t.file)); return found; };
      lit = named(head); const only = lit?.properties[0]; if (lit?.properties.length === 1 && ts.isSpreadAssignment(only) && ts.isIdentifier(only.expression)) lit = named(only.expression.getText()); }
  }
  literalOf.set(t.name, lit); return lit;
}
function docs(t: Table): void {
  const lit = literal(t); if (!lit) { console.log(`no word for ${t.name}: no literal ${t.from} in ${t.file}`); return; }
  const sf = lit.getSourceFile(), src = sf.text as string, colOf = (pos: number) => sf.getLineAndCharacterOfPosition(pos).character;
  const comments = (r: any[] | undefined) => (r ?? []).flatMap(c => clean(src.slice(c.pos, c.end)));
  // the table's own: what stands above its declaration, or above the row that holds it in its parent
  let top = lit.parent; while (top && !ts.isVariableStatement(top) && !ts.isPropertyAssignment(top) && !ts.isSourceFile(top)) top = top.parent;
  if (top && !ts.isSourceFile(top)) t.doc = comments(ts.getLeadingCommentRanges(src, top.getFullStart()));
  let lastLine = -1, last: Field | undefined;
  for (const p of lit.properties) { if (!p.name) continue; const f = t.fields.find(x => low(x.name) === low(p.name.getText())); const line = sf.getLineAndCharacterOfPosition(p.getStart()).line;
    // (a comment indented past the row it stands above is the tail of the row before, wrapped)
    const ranges = ts.getLeadingCommentRanges(src, p.getFullStart()) ?? [], col = colOf(p.getStart()), wrapped = ranges.filter((c: any) => colOf(c.pos) > col), own = ranges.filter((c: any) => colOf(c.pos) <= col);
    if (last) last.doc.push(...comments(wrapped));
    const lead = comments(own), tail = line !== lastLine ? comments(trailOfLine(src, p)) : [];
    lastLine = line; if (f) { f.doc.push(...lead, ...tail); last = f; } else last = undefined; }
}
/** the comment that ends the line a row starts on */
function trailOfLine(src: string, p: any): any[] { const eol = src.indexOf('\n', p.getStart()); const line = src.slice(p.getStart(), eol < 0 ? src.length : eol), at = lastComment(line); if (at < 0) return []; const pos = p.getStart() + at; return [{ pos, end: src.indexOf('\n', pos) < 0 ? src.length : src.indexOf('\n', pos) }]; }
function lastComment(line: string): number { let str = ''; for (let i = 0; i < line.length; i++) { const c = line[i]; if (str) { if (c === '\\') i++; else if (c === str) str = ''; continue; } if (c === '"' || c === "'" || c === '`') { str = c; continue; } if (c === '/' && line[i + 1] === '/') return i; } return -1; }
for (const t of tables.values()) docs(t);

// ---- written
const fam = new Map<string, Table[]>();
for (const t of tables.values()) { const a = fam.get(t.family) ?? []; a.push(t); fam.set(t.family, a); }
for (const e of enums) { const owner = [...tables.values()].find(t => t.fields.some(f => new RegExp(`\\b${e.name}\\b`).test(f.ty))); if (!owner) throw new Error(`no table holds ${e.name}`); e.family = owner.family; }
rmSync(OUT, { recursive: true, force: true }); mkdirSync(OUT, { recursive: true });
const RESERVED = new Set(['as', 'break', 'const', 'continue', 'crate', 'else', 'enum', 'extern', 'false', 'fn', 'for', 'if', 'impl', 'in', 'let', 'loop', 'match', 'mod', 'move', 'mut', 'pub', 'ref', 'return', 'self', 'static', 'struct', 'super', 'trait', 'true', 'type', 'unsafe', 'use', 'where', 'while', 'async', 'await', 'dyn', 'abstract', 'become', 'box', 'do', 'final', 'macro', 'override', 'priv', 'typeof', 'unsized', 'virtual', 'yield']);
const fname = (n: string) => RESERVED.has(n) ? `${n}_rows` : n;
// (what is no longer so: the table has one value now, the page's)
const NOW: [RegExp, string][] = [
  [/off in the module, raised by the page \(host\/profile\.ts pageSwitches; /i, 'on ('],
  [/OFF in this module and on the frozen stand; the page turns it on \(/, 'On ('],
  [/OFF in the module and on the frozen stand \(`TPS_\w+=1` raises it\); ON in the page \(/, 'On ('],
  [/OFF in this module, so everything headless \([^)]*\) stands as it was; the PAGE turns it on at boot before/, 'On at boot, before'],
  [/\(the value here; the page sets LEAN_GUN_M: host\/profile\.ts, \?noleangun = 0\)/, '(?noleangun = 0)'],
  [/\(The stalk's polish: DOWN in this table — dip, back, sway\.m, sway\.twist, apart, wayShape, and/, "(The stalk's polish: dip, back, sway.m, sway.twist, apart, and"],
  [/yank\.arch at the bare arch's own — and written up by holdPolish below with the page's numbers\.\)/, 'yank.arch, which holdPolish writes.)'],
];
const words = (ds: string[], pad: string) => ds.map(d => `${pad}/// ${NOW.reduce((x, [from, to]) => x.replace(from, to), d)}\n`).join('');
let lines = 0;
for (const [family, ts_] of fam) {
  let s = '';
  for (const e of enums.filter(e => e.family === family)) s += `\n${e.text}`;
  for (const t of ts_) { s += `\n${words(t.doc, '')}#[derive(${t.derive}, Knobs)]\npub struct ${t.name} {\n`;
    for (const f of t.fields) s += `${words(f.doc, '    ')}    ${f.nested ? '' : `#[k(${f.value})] `}pub ${f.name}: ${f.ty},\n`;
    s += '}\n'; }
  // what the file names, and no more
  const code = s.split('\n').filter(l => !l.trim().startsWith('///')).join('\n'), has = (w: RegExp) => w.test(code), own = new Set([...ts_.map(t => t.name), ...enums.filter(e => e.family === family).map(e => e.name)]);
  const other = [...tables.keys(), ...enums.map(e => e.name)].some(n => !own.has(n) && new RegExp(`\\b${n}\\b`).test(code));
  const fromCrate = ['Quat', 'V3'].filter(n => has(new RegExp(`\\b${n}\\b`)));
  const head = [`//! The numbers of ${family.replace(/_/g, ' ')}.`, ...(other ? ['use super::*;'] : []), 'use crate::bytes::ToBytes;', ...(has(/\b\w+Id\b/) ? ['use crate::ids::*;'] : []), ...(has(/\bJs(Map|Set)\b|\bTab\b/) ? ['use crate::js::*;'] : []), ...fromCrate.map(n => `use crate::${n};`), ...(has(/\bString\b/) ? ['use alloc::string::String;'] : []), ...(has(/\bVec\b/) ? ['use alloc::vec::Vec;'] : []), 'use game_derive::Knobs;'];
  s = head.join('\n') + '\n' + s;
  writeFileSync(`${OUT}/${fname(family)}.rs`, s); lines += s.split('\n').length;
}
const root = /\/\/\/ every knob table\n#\[derive\(([^)]*)\)\]\npub struct Knobs \{\n(.*?)^\}\n/ms.exec(text)!;
const mods = [...fam.keys()].map(fname).sort();
writeFileSync(`${OUT}/mod.rs`, `//! THE NUMBERS THE GAME IS TUNED BY: a file a family, each table with its values beside its fields. A person turns one by its dotted name: a row of the panel (game_run shown.rs), an address word (words.rs), a tape's Knob command.
use crate::bytes::ToBytes;
use alloc::string::String;
use game_derive::Knobs;

${mods.map(m => `mod ${m}; pub use ${m}::*;\n`).join('')}
/// a table's numbers by their dotted names (\`#[derive(Knobs)]\` writes it)
pub trait KnobTable {
    /// each number under this table: its dotted name, whether it is a yes or no, what it stands at (a yes is 1)
    fn walk(&self, at: &mut String, each: &mut dyn FnMut(&str, bool, f64));
    /// the number of that name; None = there is none
    fn get(&self, name: &str) -> Option<f64>;
    /// turn it; false = there is none. For a yes or no anything but 0 is yes
    fn set(&mut self, name: &str, v: f64) -> bool;
}
/// (what the derive writes) the first part of a dotted name, and the rest
pub fn first_part(name: &str) -> (&str, &str) { name.split_once('.').unwrap_or((name, "")) }
/// (what the derive writes) one number, by its name under \`at\`
pub fn each_of(at: &mut String, name: &str, yes: bool, v: f64, each: &mut dyn FnMut(&str, bool, f64)) { under(at, name, |at| each(at, yes, v)) }
/// (what the derive writes) the dotted name goes one part deeper for \`f\`
pub fn under(at: &mut String, name: &str, f: impl FnOnce(&mut String)) {
    let n = at.len();
    if n > 0 { at.push('.'); }
    at.push_str(name);
    f(at);
    at.truncate(n);
}

/// every knob table
#[derive(${root[1].replace(/,?\s*\bDefault\b/, '')}, Knobs)]
pub struct Knobs {
${root[2]}}
`);
console.log(`${tables.size} tables, ${enums.length} enums, ${fam.size} families, ${lines} lines; rows with a word: ${[...tables.values()].flatMap(t => t.fields).filter(f => f.doc.length).length} of ${[...tables.values()].flatMap(t => t.fields).length}`);

// ---- who asked for a table by its old name
const types = new Set(tables.keys());
const walk = (d: string, out: string[] = []): string[] => { for (const n of readdirSync(d)) { if (n === 'target' || n === 'knobs_list.rs') continue; const p = `${d}/${n}`; if (statSync(p).isDirectory()) walk(p, out); else if (p.endsWith('.rs') && !p.startsWith(OUT) && p !== SRC) out.push(p); } return out; };
for (const p of walk('rust')) { const was = readFileSync(p, 'utf8');
  const now = was.replace(/\bknobs::(page_\w+)\(\)/g, (m, f) => pageOf.has(f) ? `knobs::${pageOf.get(f)}::page()` : m).replace(/\bknobs::(\w+)::zeroed\(\)/g, (m, t) => types.has(t) ? `knobs::${t}::page()` : m);
  let then = now;
  for (const m of now.matchAll(/^\s*use (?:crate|game)::knobs::(page_\w+);\n/gm)) { then = then.replace(m[0], '').replace(new RegExp(`\\b${m[1]}\\(\\)`, 'g'), `crate::knobs::${pageOf.get(m[1])}::page()`); }
  if (then !== was) writeFileSync(p, then); }
rmSync(SRC);
