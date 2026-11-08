// WHICH ABSENT VALUES ARE NULL: tools/blitz/class-nulls.json, written by `wire.ts gen` from the TypeScript declarations. Rust has one word for nothing (`None`); the
// TypeScript has two, a field that is null and one that is not there. A thing the apply makes (a row, a record) has run no constructor, so a field it is not sent stands as
// not there: where its type says `X | null` that is a third answer, and `=== null` is false of it (a mission of no kind read as one of some kind). So a made thing holds null in
// every Option field that is not optional, before what it is sent. An optional field (`x?:`, or `| undefined`) stays away. Where the table says which itself (a field's `none`), that stands.
//   bun run tools/wire/nulls.ts [--say]     --say: every type no declaration was found of (its Option fields stay away), and the counts
import { readFileSync, readdirSync } from 'node:fs';
import { join } from 'node:path';

const ROOT = join(import.meta.dir, '../..');
interface Field { ty: string; rust: string; ts?: string; expr?: string; none?: string }
interface Type { k: string; mod: string; name: string; ts?: string; fields?: Field[] }
interface Member { optional: boolean; type: string }

const flat = (s: string): string => s.toLowerCase().replace(/[^a-z0-9]/g, '');
const DIRS: [string, string][] = [['scene_', 'src/scene/'], ['anim_', 'src/anim/'], ['render_', 'src/render/'], ['sim_', 'src/sim/'], ['math_', 'src/math/'], ['host_', 'src/host/'], ['ui_', 'src/ui/'], ['smoke_', 'src/smoke/'], ['net_', 'src/net/'], ['replay_', 'src/replay/']];
const homeOf = (mod: string): [string, string] => { for (const [pre, dir] of DIRS) if (mod.startsWith(pre)) return [dir, flat(mod.slice(pre.length))]; return ['src/game/', flat(mod)]; };
/** the key a field stands under on its object; none where it is worked out */
export const keyOf = (f: Field): string | null => f.expr === undefined ? f.ts ?? null : f.expr.match(/^o\.([A-Za-z_$][\w$]*)$/)?.[1] ?? null;

/** the text with comments and the insides of strings blanked, every place kept */
function bare(s: string): string { let o = '', i = 0; const blank = (n: number) => { for (let k = 0; k < n && i < s.length; k++, i++) o += s[i] === '\n' ? '\n' : ' '; };
  while (i < s.length) { const c = s[i], d = s[i + 1];
    if (c === '/' && d === '/') { let k = s.indexOf('\n', i); if (k < 0) k = s.length; blank(k - i); continue; }
    if (c === '/' && d === '*') { let k = s.indexOf('*/', i + 2); k = k < 0 ? s.length : k + 2; blank(k - i); continue; }
    if (c === "'" || c === '"') { let k = i + 1; while (k < s.length && s[k] !== c && s[k] !== '\n') k += s[k] === '\\' ? 2 : 1; o += c; i++; blank(k - i); if (i < s.length) { o += s[i]; i++; } continue; }
    if (c === '`') { let k = i + 1; while (k < s.length && s[k] !== '`') k += s[k] === '\\' ? 2 : 1; o += ' '; i++; blank(k - i); o += ' '; i++; continue; }
    o += c; i++; }
  return o; }
/** where the bracket at `i` is closed */
function closes(s: string, i: number): number { let d = 0; for (let k = i; k < s.length; k++) { if ('{(['.includes(s[k])) d++; else if ('})]'.includes(s[k]) && --d === 0) return k; } return s.length; }

/** the members of a body, by name: what each is declared as. A class's constructor brings its parameter properties */
function members(body: string): Map<string, Member> { const out = new Map<string, Member>(); let i = 0;
  const put = (name: string, optional: boolean, type: string) => { if (!out.has(name)) out.set(name, { optional, type: type.trim() }); };
  while (i < body.length) { const rest = body.slice(i), m = rest.match(/^\s*(?:(?:readonly|private|protected|public|declare|override)\s+)*([A-Za-z_$][\w$]*)\s*([?!]?)\s*(:|=(?!=))/);
    const ctor = rest.match(/^\s*constructor\s*\(/);
    if (ctor) { const at = i + ctor[0].length - 1, end = closes(body, at); for (const p of split(body.slice(at + 1, end))) { const q = p.match(/^\s*(?:(?:readonly|private|protected|public)\s+)+([A-Za-z_$][\w$]*)\s*(\??)\s*:\s*([\s\S]*)$/); if (q) put(q[1], q[2] === '?', q[3].split(/\s=\s/)[0]); } i = end + 1; continue; }
    if (m) { let k = i + m[0].length, d = 0; for (; k < body.length; k++) { const c = body[k]; if ('{([<'.includes(c)) d++; else if ('})]'.includes(c) || (c === '>' && body[k - 1] !== '=')) d--; else if (d <= 0 && (c === ';' || c === ',' || (c === '\n' && /^\s*(?:$|[A-Za-z_$}\[@])/m.test(body.slice(k + 1, k + 80)) && !/[|&=(,:?]\s*$/.test(body.slice(i, k)) && !/^\s*[|&?:.]/.test(body.slice(k + 1))))) break; }
      const text = body.slice(i + m[0].length, k); put(m[1], m[2] === '?', m[3] === '=' ? `= ${text}` : text); i = k + 1; continue; }
    // (a method, an accessor, an index: on past it)
    const n = rest.search(/[;\n{(]/); if (n < 0) break; if ('{('.includes(rest[n])) i = closes(body, i + n) + 1; else i += n + 1; }
  return out; }
function split(s: string): string[] { const out: string[] = []; let d = 0, at = 0; for (let k = 0; k < s.length; k++) { const c = s[k]; if ('{([<'.includes(c)) d++; else if ('})]'.includes(c) || (c === '>' && s[k - 1] !== '=')) d--; else if (c === ',' && d <= 0) { out.push(s.slice(at, k)); at = k + 1; } } out.push(s.slice(at)); return out; }

/** tools/blitz/class-nulls.json's text, and what no declaration was found of */
export function nullTable(schema: { types: Record<string, Type> }): { text: string; unknown: string[]; nulls: number; away: number } {
  const decls = new Map<string, { file: string; members: Map<string, Member>; from: string[] }[]>();
  for (const f of (readdirSync(join(ROOT, 'src'), { recursive: true }) as string[]).filter(f => f.endsWith('.ts') && !f.endsWith('.d.ts'))) { const file = `src/${f.split(/[\\/]/).join('/')}`, s = bare(readFileSync(join(ROOT, file), 'utf8'));
    for (const m of s.matchAll(/\b(?:interface|class)\s+(\w+)([^{;=()]*)\{|\btype\s+(\w+)(?:<[^=]*>)?\s*=\s*((?:\w+\s*&\s*)*)\{/g)) { const name = m[1] ?? m[3], from = ((m[2] ?? '').replace(/implements.*$/, '') + (m[4] ?? '')).match(/[A-Z]\w*/g) ?? [], at = m.index! + m[0].length - 1, L = decls.get(name) ?? decls.set(name, []).get(name)!; L.push({ file, members: members(s.slice(at + 1, closes(s, at))), from }); } }
  const declOf = (T: Type): Map<string, Member> | null => { const L = decls.get(T.ts ?? T.name) ?? []; const [dir, stem] = homeOf(T.mod); const D = L.find(d => d.file.startsWith(dir) && flat(d.file.split('/').pop()!.replace(/\.ts$/, '')) === stem) ?? (L.length === 1 ? L[0] : null); if (!D) return null;
    const M = new Map(D.members); for (const b of D.from) { const B = decls.get(b), X = B?.find(d => d.file === D.file) ?? (B?.length === 1 ? B[0] : null); for (const [k, v] of X?.members ?? []) if (!M.has(k)) M.set(k, v); } return M; };   // (what it is built on is its own too)
  // (a type written in place: its body stands in the declaration of a field that holds it)
  const holders = new Map<string, [string, Field][]>(); for (const [p, T] of Object.entries(schema.types)) for (const f of T.fields ?? []) for (const m of f.ty.matchAll(/crate::[\w:]+/g)) (holders.get(m[0]) ?? holders.set(m[0], []).get(m[0])!).push([p, f]);
  const memo = new Map<string, Map<string, Member> | null>();
  const membersOf = (path: string, deep = 0): Map<string, Member> | null => { if (memo.has(path)) return memo.get(path)!; memo.set(path, null); const T = schema.types[path]; let M = T ? declOf(T) : null;
    if (!M && T && deep < 6) for (const [hp, hf] of holders.get(path) ?? []) { const H = membersOf(hp, deep + 1), k = keyOf(hf), d = k ? H?.get(k) : undefined; if (!d) continue;
      const keys = (T.fields ?? []).map(keyOf).filter(Boolean) as string[]; for (let i = d.type.indexOf('{'); i >= 0; i = d.type.indexOf('{', i + 1)) { const B = members(d.type.slice(i + 1, closes(d.type, i))); if (keys.every(x => B.has(x))) { M = B; break; } } if (M) break; }
    memo.set(path, M); return M; };
  const out: Record<string, string[]> = {}, unknown: string[] = []; let nulls = 0, away = 0;
  for (const [path, T] of Object.entries(schema.types).sort((a, b) => a[0] < b[0] ? -1 : 1)) { if (T.k !== 'struct' || T.mod === 'view') continue; const L: string[] = [];
    for (const f of T.fields ?? []) { if (!/^Option<(?!Option<)/.test(f.ty)) continue; const k = keyOf(f); if (!k || f.none === 'undefined') continue; if (f.none === 'null') { L.push(k); nulls++; continue; } const d = membersOf(path)?.get(k);
      if (!d) { unknown.push(`${path.replace(/^crate::/, '')}.${f.rust}`); continue; }
      if (d.optional || /\bundefined\b/.test(d.type)) { away++; continue; }
      L.push(k); nulls++; }
    if (L.length) out[`${T.mod}::${T.name}`] = L; }
  return { text: JSON.stringify(out).replace(/\],"/g, '],\n"') + '\n', unknown, nulls, away };
}
if (import.meta.main) { const R = nullTable(JSON.parse(readFileSync(join(ROOT, 'tools/blitz/schema.json'), 'utf8'))); console.log(`nulls: ${R.nulls} fields are null, ${R.away} stay away, ${R.unknown.length} with no declaration`); if (process.argv.includes('--say')) for (const u of R.unknown) console.log(`  ${u}`); }
