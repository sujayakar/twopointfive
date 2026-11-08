// ROWS NOTHING READS LEAVE BOTH SIDES. A settled row the fold has taken out of every Rust body, and that no file of src/ reads, is one line in a TypeScript table and
// one field of a Rust struct, with a line of the wire. This program takes a list of rows (one `TABLE.path.key` a line) and:
//   · cuts each from its table's object literal in src/ (a row's own comment lines above it go too; a sub-table left with no rows goes whole);
//   · cuts each from tools/qa/switches.json;
//   · cuts each from rows_same!'s lists in rust/game_rules/src/difficulty/p_01.rs, which fields-drop cannot see inside a macro's arguments;
//   · writes OUT/fields.txt (`knobs::Type.field` a line) and OUT/types.txt (`knobs::Type` a line, for each sub-table that went whole).
// Then, from the root:
//   bun run tools/wire/wire.ts drop --file OUT/fields.txt ; for each line of types.txt: bun run tools/wire/wire.ts drop-type <line>
//   fold fields-drop --file OUT/fields.txt --write ; fold types-drop --file OUT/types.txt --write
// A row it cannot place (no literal of that name, the key not written out in the literal: a spread, a computed table) is said and left; `--write` writes, without it nothing does.
//   bun run tools/fold/rows-drop.ts --file rows.txt --out OUT [--write]
import { readFileSync, writeFileSync, readdirSync, statSync, mkdirSync } from 'node:fs';
import { join } from 'node:path';

const argv = process.argv.slice(2), opt = (k: string): string | null => { const i = argv.indexOf(k); return i >= 0 ? argv[i + 1] : null; };
const listFile = opt('--file'), out = opt('--out'), WRITE = argv.includes('--write');
if (!listFile || !out) { console.error('usage: bun run tools/fold/rows-drop.ts --file rows.txt --out DIR [--write]'); process.exit(2); }
const simReads = readFileSync('rust/sim/src/knobs.rs', 'utf8');   // (rust/sim's own list, by path: src/game/rustknobs.ts throws at boot on a path with no row)
const asked = readFileSync(listFile, 'utf8').split('\n').map(s => s.trim()).filter(Boolean);
const rows = asked.filter(r => { const held = simReads.includes(`"${r}"`); if (held) console.log(`left: ${r}: rust/sim reads it by name`); return !held; });

function tsFiles(dir: string, acc: string[] = []): string[] {
  for (const n of readdirSync(dir)) { const p = join(dir, n); if (statSync(p).isDirectory()) tsFiles(p, acc); else if (p.endsWith('.ts')) acc.push(p); }
  return acc;
}
const files = new Map<string, string>(tsFiles('src').map(f => [f, readFileSync(f, 'utf8')]));

// ---- a walk of an object literal: strings, templates and comments skipped, entries found
interface Entry { key: string; start: number; valueStart: number; end: number; obj: boolean }
function skip(s: string, i: number): number {   // past the string, template or comment that opens at i; i if none does
  const c = s[i];
  if (c === '/' && s[i + 1] === '/') { const e = s.indexOf('\n', i); return e < 0 ? s.length : e; }
  if (c === '/' && s[i + 1] === '*') return s.indexOf('*/', i + 2) + 2;
  if (c === "'" || c === '"') { for (let j = i + 1; ; j++) { if (s[j] === '\\') j++; else if (s[j] === c) return j + 1; } }
  if (c === '`') { for (let j = i + 1; ; j++) { if (s[j] === '\\') j++; else if (s[j] === '`') return j + 1; else if (s[j] === '$' && s[j + 1] === '{') j = closeOf(s, j + 1); } }
  return i;
}
function closeOf(s: string, open: number): number {   // the index of the bracket that closes the one at `open`
  let depth = 0;
  for (let i = open; i < s.length; ) { const j = skip(s, i); if (j !== i) { i = j; continue; }
    const c = s[i]; if ('{[('.includes(c)) depth++; else if ('}])'.includes(c)) { depth--; if (!depth) return i; } i++; }
  throw new Error('unbalanced');
}
function entriesOf(s: string, open: number): Entry[] | null {   // null: something in it is not `key: value` (a spread, a method, a computed key)
  const close = closeOf(s, open), E: Entry[] = []; let i = open + 1;
  while (i < close) {
    const j = skip(s, i); if (j !== i) { i = j; continue; }
    if (/[\s,]/.test(s[i])) { i++; continue; }
    const m = /^(?:([A-Za-z_$][\w$]*)|'([^']*)'|"([^"]*)")\s*:/.exec(s.slice(i, i + 200)); if (!m) return null;
    const start = i, valueStart = i + m[0].length; let v = valueStart;
    while (v < close) { const k = skip(s, v); if (k !== v) { v = k; continue; } const c = s[v]; if ('{[('.includes(c)) { v = closeOf(s, v) + 1; continue; } if (c === ',') break; v++; }
    let vs = valueStart; while (true) { const k = skip(s, vs); if (k !== vs) { vs = k; continue; } if (/\s/.test(s[vs])) { vs++; continue; } break; }
    E.push({ key: m[1] ?? m[2] ?? m[3], start, valueStart: vs, end: v, obj: s[vs] === '{' });
    i = v + 1;
  }
  return E;
}
function tableOf(name: string): { file: string; open: number } | null {
  const re = new RegExp(`(?:^|\\n)\\s*export\\s+const\\s+${name}\\b[^=\\n]*=\\s*(?:[^\\n{]*\\()?\\{`); let found: { file: string; open: number } | null = null;
  const all: { file: string; open: number }[] = [];
  for (const [f, s] of files) { const m = re.exec(s); if (m) all.push({ file: f, open: m.index + m[0].length - 1 }); }
  if (all.length < 2) return all[0] ?? null;
  // (two tables of one name: the registry's is the one src/game/knobs.ts imports)
  const reg = files.get('src/game/knobs.ts') ?? '', from = new RegExp(`import \\{[^}]*\\b${name}\\b[^}]*\\} from '\\./(\\w+)'`).exec(reg);
  found = from ? all.find(t => t.file === `src/game/${from[1]}.ts`) ?? null : null;
  return found;
}
/** the span to cut for an entry: its line(s) with its own comment above when it stands alone on them, else itself and its comma */
function spanOf(s: string, e: Entry): [number, number] {
  let a = e.start, b = e.end; if (s[b] === ',') b++;
  const ls = s.lastIndexOf('\n', a - 1) + 1, alone = !s.slice(ls, a).trim();
  const le0 = s.indexOf('\n', b), rest = s.slice(b, le0).trim(), endsLine = !rest || rest.startsWith('//') || (rest.startsWith('/*') && rest.endsWith('*/'));
  if (alone && endsLine) {
    a = ls; b = le0 + 1;
    // (its own comment: lines that are comment and nothing else, and not a section's heading; a block's first line ends the walk)
    for (;;) { const p = s.lastIndexOf('\n', a - 2) + 1, l = s.slice(p, a).trim(); if (l.startsWith('// ----')) break; const line = l.startsWith('//') || (l.startsWith('*') && !l.startsWith('*/')) || l.endsWith('*/') && (l.startsWith('/*') || l.startsWith('*')); const open = l.startsWith('/*') && !l.includes('*/'); if (!line && !open) break; if (l.startsWith('/*') && l.includes('*/') && !l.endsWith('*/')) break; a = p; if (l.startsWith('/*')) break; }
    return [a, b];
  }
  while (/[ \t]/.test(s[b] ?? '')) b++;
  return [a, b];
}

// ---- the Rust tables
const knobsRs = readFileSync('rust/game_types/src/knobs.rs', 'utf8');
const structs = new Map<string, Map<string, string>>();
for (const m of knobsRs.matchAll(/pub struct (\w+) \{([^}]*)\}/g)) {
  const f = new Map<string, string>(); for (const g of m[2].matchAll(/pub (\w+): ([^,\n]+),/g)) f.set(g[1], g[2].trim().replace(/^crate::knobs::/, '')); structs.set(m[1], f);
}
const same = (a: string, b: string) => a.replace(/_/g, '').toLowerCase() === b.replace(/_/g, '').toLowerCase();
function rustField(type: string, key: string): [string, string] | null { const f = structs.get(type); if (!f) return null; for (const [n, t] of f) if (same(n, key)) return [n, t]; return null; }

// ---- the walk: each row, deepest first, so that a sub-table left empty can go whole
const said: string[] = [], fields = new Set<string>(), types = new Set<string>(); const taken: string[] = [];
const byTable = new Map<string, string[][]>(); for (const r of rows) { const p = r.split('.'); (byTable.get(p[0]) ?? byTable.set(p[0], []).get(p[0])!).push(p.slice(1)); }
for (const [table, paths] of byTable) {
  const T = tableOf(table);
  const root = rustField('Knobs', table);
  if (!T || !root) { for (const p of paths) said.push(`${table}.${p.join('.')}: ${!T ? 'no one literal of that name in src/' : 'no such table in the Rust Knobs'}`); continue; }
  // what each sub-table holds, before and after
  const want = new Set(paths.map(p => p.join('.')));
  const cuts: [number, number][] = [];
  const walk = (open: number, prefix: string[], rustType: string): boolean => {   // true: every row of this literal goes
    const s = files.get(T.file)!, E = entriesOf(s, open);
    if (!E) { for (const w of want) if (w.startsWith(prefix.join('.'))) said.push(`${table}.${w}: its literal has a spread, a method or a computed key`); return false; }
    const mine: [Entry, [string, string] | null, boolean][] = [];
    for (const e of E) {
      const path = [...prefix, e.key], name = path.join('.'), rf = rustField(rustType, e.key);
      let gone = want.has(name);
      if (!gone && e.obj && [...want].some(w => w.startsWith(name + '.'))) {
        gone = walk(e.valueStart, path, rf ? rf[1] : '');
      }
      mine.push([e, rf, gone]);
    }
    const whole = mine.every(m => m[2]);
    for (const [e, rf, gone] of mine) {
      if (!gone) continue;
      const name = [...prefix, e.key].join('.');
      if (!whole || !prefix.length) {   // (a row with no Rust field left is the table's on the page alone: an earlier drop took the other side)
        cuts.push(spanOf(files.get(T.file)!, e)); taken.push(`${table}.${name}`);
        if (rf) { fields.add(`knobs::${rustType}.${rf[0]}`); if (structs.get(rf[1])) dropInner(rf[1]); }
      }
    }
    return whole && prefix.length > 0;
  };
  const dropInner = (type: string) => { types.add(`knobs::${type}`); for (const [, t] of structs.get(type) ?? []) if (structs.get(t)) dropInner(t); };
  walk(T.open, [], root[1]);
  // (a row under a sub-table that went whole was cut with it: its own field goes with its struct)
  for (const p of paths) if (!taken.includes(`${table}.${p.join('.')}`) && !said.some(l => l.startsWith(`${table}.${p.join('.')}:`))) { const up = taken.find(t => `${table}.${p.join('.')}`.startsWith(t + '.')); if (!up) said.push(`${table}.${p.join('.')}: no such key in its literal`); }
  cuts.sort((a, b) => b[0] - a[0]);
  let s = files.get(T.file)!; for (const [a, b] of cuts) s = s.slice(0, a) + s.slice(b);
  // (the rows the panel keeps per machine, by name: a row that went leaves that list too)
  { const from = T.open, nextTable = /\nexport const [A-Z][A-Z_0-9]* =/g; nextTable.lastIndex = closeOf(s, from); const n = nextTable.exec(s), to = n ? n.index : s.length, head = s.slice(0, from), mid = s.slice(from, to), tail = s.slice(to);
    let m2 = mid; for (const p of paths) if (p.length === 1 && taken.includes(`${table}.${p[0]}`)) m2 = m2.replace(/const KEPT\b[^=]*= \[[^\]]*\]/, m => m.replace(new RegExp(`'${p[0]}',? ?`), ''));
    s = head + m2 + tail; }
  files.set(T.file, s);
  const left = entriesOf(s, tableOf(table)!.open); if (left && !left.length) said.push(`${table}: the table has no rows left: its declaration, its line of src/game/knobs.ts and knobs::Knobs.${root[0]} are by hand`);
}

// ---- the panels: a row of feelRows('TABLE', [...]) or knobRows(panel, section, TABLE, ..., [...]) that names a row gone goes with it; so does its entry in teamfeel.ts's kept ranges
{ const f = 'src/ui/bindings.ts'; let s = files.get(f) ?? ''; const gone = new Map<string, Set<string>>();
  for (const r of taken) { const i = r.indexOf('.'), t = r.slice(0, i), k = r.slice(i + 1); (gone.get(t) ?? gone.set(t, new Set()).get(t)!).add(k); }
  const cutRows = (open: number, keys: Set<string>) => {   // the array at `open`: each `{ k: 'key', ... }` it holds whose key is gone
    const close = closeOf(s, open); let body = s.slice(open, close + 1), out = '', i = 0;
    while (i < body.length) { const j = skip(body, i); if (j !== i) { out += body.slice(i, j); i = j; continue; }
      if (body[i] === '{') { const e = closeOf(body, i), m = /^\{\s*k:\s*'([^']+)'/.exec(body.slice(i, e + 1));
        if (m && keys.has(m[1])) { let b = e + 1; if (body[b] === ',') b++; const eol = body.indexOf('\n', b), tail = body.slice(b, eol); if (!tail.trim() || tail.trim().startsWith('//')) { b = eol + 1; const ls = out.lastIndexOf('\n'); if (!out.slice(ls + 1).trim()) out = out.slice(0, ls + 1); } i = b; panelRows++; continue; }
        out += body.slice(i, e + 1); i = e + 1; continue; }
      out += body[i++]; }
    s = s.slice(0, open) + out + s.slice(close + 1);
  };
  var panelRows = 0;
  for (const [t, keys] of gone) for (const re of [new RegExp(`feelRows\\('${t}', \\[`, 'g'), new RegExp(`knobRows\\(panel, \\w+, ${t}\\b[^\\[\\n]*\\[`, 'g')]) {
    const at: number[] = []; for (let m; (m = re.exec(s)); ) at.push(m.index + m[0].length - 1);
    for (const o of at.reverse()) cutRows(o, keys);
  }
  if (files.has(f)) files.set(f, s);
  const tf = 'src/game/teamfeel.ts'; let t = files.get(tf) ?? '';
  for (const [tb, keys] of gone) for (const k of keys) { const row = `\\b${k}: (?:null|\\[[^\\]]*\\]),? ?`; t = t.replace(new RegExp(`(KEPT\\.${tb}, \\{[^}]*?)${row}`), '$1').replace(new RegExp(`(\\n  ${tb}: \\{[^\\n]*?)${row}`), '$1'); }
  if (files.has(tf)) files.set(tf, t); console.log(`panels: ${panelRows} rows`);
}
// ---- the words that held a row on the old stand: its line of src/host/profile.ts's list, and standup.ts's pin of that word
{ let p = files.get('src/host/profile.ts') ?? '', u = files.get('src/host/standup.ts') ?? '';
  for (const r of taken) { const m = new RegExp(`^\\s*'${r.replace(/\./g, '\\.')}': '(TPS_\\w+)=[^']*',[^\\n]*\\n`, 'm').exec(p); if (!m) continue; p = p.replace(m[0], ''); u = u.replace(new RegExp(`^[ \\t]*if \\(pinned\\('${m[1]}'\\)\\)[^\\n]*\\n`, 'm'), ''); }
  files.set('src/host/profile.ts', p); files.set('src/host/standup.ts', u); }
const goneRows = new Set(taken); const sw = readFileSync('tools/qa/switches.json', 'utf8').split('\n');
const swKept = sw.filter(l => { const m = /^\s*\{"row":"([^"]+)"/.exec(l); if (!m) return true; return !(goneRows.has(m[1]) || [...goneRows].some(g => m[1].startsWith(g + '.'))); });
let swOut = swKept.join('\n').replace(/,(\s*\])/, '$1');
{ const J = JSON.parse(swOut), c: Record<string, number> = {}; for (const r of J.rows) c[r.kind] = (c[r.kind] ?? 0) + 1; swOut = swOut.replace(/"counts": \{[^}]*\}/, `"counts": ${JSON.stringify(c)}`); }

for (const l of said) console.log(`left: ${l}`);
console.log(`rows-drop: ${taken.length} rows, ${fields.size} fields, ${types.size} types${WRITE ? '' : ' (nothing written: --write)'}`);
if (WRITE) {
  for (const [f, s] of files) if (s !== readFileSync(f, 'utf8')) writeFileSync(f, s);
  writeFileSync('tools/qa/switches.json', swOut);
  { const p = 'rust/game_rules/src/difficulty/p_01.rs'; const byType = new Map<string, Set<string>>(); for (const f of fields) { const [t, n] = f.replace('knobs::', '').split('.'); (byType.get(t) ?? byType.set(t, new Set()).get(t)!).add(n); }
    writeFileSync(p, readFileSync(p, 'utf8').replace(/rows_same!\(a, b, crate::knobs::(\w+); ([^)]*?)(,?\s*)\)/g, (_m, t: string, list: string, tail: string) => `rows_same!(a, b, crate::knobs::${t}; ${list.split(',').map(x => x.trim()).filter(x => x && !byType.get(t)?.has(x)).join(', ')}${tail})`)); }
  mkdirSync(out, { recursive: true }); writeFileSync(join(out, 'fields.txt'), [...fields].join('\n') + '\n'); writeFileSync(join(out, 'types.txt'), [...types].join('\n') + (types.size ? '\n' : ''));
}
