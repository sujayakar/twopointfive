// THE WIRE'S TABLE: what the page reads of the Rust game. `tools/blitz/schema.json` declares it, `tools/wire/fields.json` says which of it goes every tick, and a person changes them with this tool. Nothing here reads the game's TypeScript or the Rust types: a state only Rust uses needs no line in the table at all.
//   bun run tools/wire/wire.ts gen [--check]                                        write puppet.rs, puppet-apply.ts, puppet-lists.json, out-dispatch.ts and the stamp · --check: exit 1 where a file would change
//   bun run tools/wire/wire.ts add <mod::Type.field> --ty <type> [--ts <name>] [--expr <e>] [--start-only]    a field the page reads; refused if nothing the page runs reads that name
//   bun run tools/wire/wire.ts add-struct <mod::Type> <field:type,...>              a struct of the table, new: its line on the wire and its write by path
//   bun run tools/wire/wire.ts drop <mod::Type.field>...                            fields off the wire · --file <list> takes the names one a line
//   bun run tools/wire/wire.ts add-enum <mod::Type> <word[:Tag],...>                a plain enum of the table, new: its line on the wire
//   bun run tools/wire/wire.ts drop-type <mod::Type>...                             a type off the table once no field names it
//   tools/blitz/rust.sh rust bun run tools/wire/cost.ts [address]                   what a tick's export and apply cost now, on the page's list
// An enum arm needs nothing: an arm the table does not know goes out as the tag 0xffff, and the page keeps what it had.
import { execFileSync } from 'node:child_process';
import { existsSync, mkdtempSync, mkdirSync, readFileSync, readdirSync, rmSync, statSync, writeFileSync } from 'node:fs';
import { tmpdir } from 'node:os';
import { join } from 'node:path';
const ROOT = join(import.meta.dir, '../..'), BLITZ = join(ROOT, 'tools/blitz'), HERE = import.meta.dir, SCHEMA = join(BLITZ, 'schema.json'), FIELDS = join(HERE, 'fields.json');
const die = (s: string): never => { console.error(`wire: ${s}`); process.exit(1); };
const rustFile = (name: string): string => { const hits = readdirSync(join(ROOT, 'rust')).map(c => join(ROOT, 'rust', c, 'src', name)).filter(existsSync); if (hits.length !== 1) die(`${name} is in ${hits.length} crates`); return hits[0]; };
const read = (p: string) => readFileSync(p, 'utf8'), readJ = (p: string) => JSON.parse(read(p));
/** fields.json as it is kept: a name a line and no indent, so an add or a drop shows as its own rows in a diff and nothing else moves */
const writeFields = (L: unknown) => writeFileSync(FIELDS, JSON.stringify(L, null, 1).replace(/^ +/gm, ''));
const fnv = (s: string) => { let h = 0x811c9dc5; for (let i = 0; i < s.length; i++) h = Math.imul(h ^ s.charCodeAt(i), 0x01000193) >>> 0; return h >>> 0; };
// THE STATE WORDS: a hash of the whole game, a word a part, for two machines of one networked game to compare. What a host hands in and differs by machine is left out, and so is what is only a step's output
const HOSTS_OWN = new Set(['m.owner', 'out', 'cmd_buf', 'local_seat', 'view_proj', 'puppet', 'm_per_px', 'taps', 'seat', 'tab.camera', 'tab.input', 'tab.cmd_seat', 'engine.settings', 'engine.view_focus', 'engine.effective_scale', 'engine.sharp_on']);
function stateWords(io: string): string { const fieldsOf = (path: string): string[] => { const l = io.split('\n').find(x => x.startsWith(`impl Io for ${path} {`)) ?? die(`no line for ${path}`) as never, w = l.slice(0, l.indexOf(' fn r(')); return [...w.matchAll(/self\.([a-z_0-9]+)\.w\(o\)/g)].map(m => m[1]); };
  // SCRATCH IS NO STATE (schema.json `scratch`, the ONE list: the rows the path diff does not compare). A part named there has no word, and a field of a
  // part named there is left out of that part's word. A row names a part as the field of the type that holds it: `game::Game.alone_own`,
  // `game::Mods.scene_collision`; and a field of a part as a field of the part's own type: `scene_collision::StaticCollision.mirror`. The rule a row
  // must pass: two equal games, the part made anew in one at some tick, every OTHER word equal from there to the end. (A scratch field deeper than a
  // part's own fields is still in its part's word: the word is of the bytes the wire's line writes.)
  const SC: Record<string, boolean> = readJ(SCHEMA).scratch ?? {}, TY: Record<string, any> = readJ(SCHEMA).types, q = (path: string, f: string) => `${path.replace(/^crate::/, '')}.${f}`;
  const tyOf = (holder: string, f: string): string => (TY[holder]?.fields ?? []).find((x: any) => x.rust === f)?.ty ?? '';
  const P: [string, string][] = [];
  const push = (name: string, ex: string, holder = '', f = '') => { if (HOSTS_OWN.has(name) || (holder && SC[q(holder, f)])) return; const ty = holder ? tyOf(holder, f) : '', own = TY[ty]?.k === 'struct' ? fieldsOf(ty) : [], cut = own.filter(x => SC[q(ty, x)]);
    P.push([name, cut.length ? `{ ${own.filter(x => !cut.includes(x)).map(x => `${ex}.${x}.w(o);`).join(' ')} }` : ex]); };
  for (const f of fieldsOf('crate::game::Game')) { if (f === 'm') for (const x of fieldsOf('crate::game::Mods')) push(`m.${x}`, `g.m.${x}`, 'crate::game::Mods', x); else if (f === 'engine') for (const x of fieldsOf('crate::host_api::HostApi')) push(`engine.${x}`, `g.engine.${x}`, 'crate::host_api::HostApi', x); else push(f, `g.${f}`, 'crate::game::Game', f); }
  const ex = io.slice(io.indexOf('pub fn export(g: &crate::game::Game)')), end = ex.indexOf('o.b }'); for (const m of ex.slice(0, end).matchAll(/\bg\.([a-z_0-9.]+)\.w\(&mut o\)/g)) push(m[1], `g.${m[1]}`);
  // A SEAT'S SET BY SEAT, wherever it lies: on Game for the seat standing there (which differs by machine), in `cmd_seat[k].own` for the rest. A word a key of the set, all seats in their order
  const setKeys = fieldsOf('crate::seats::SeatSet').filter(k => k !== 'seat' && !HOSTS_OWN.has(k)), inSet = new Set(fieldsOf('crate::seats::SeatSet')); for (let i = P.length - 1; i >= 0; i--) if (inSet.has(P[i][0])) P.splice(i, 1);
  for (const k of setKeys) P.push([`seats.${k}`, `{ for s in g.tab.cmd_seat.ids() { if s == g.seat { g.${k}.w(o); } else if let Some(x) = &g.tab.cmd_seat[s].own { x.${k}.w(o); } } }`]);
  const L = ['//! A hash of the whole game, a word a part, for two machines of one networked game to compare. Written by tools/wire/wire.ts from the lines of the wire. Left out: what a host hands in and differs by machine, and what is only a step\'s output.', 'use crate::io::{Io, W};', 'use alloc::vec::Vec;', `pub const STATE_NAMES: [&str; ${P.length}] = [${P.map(p => JSON.stringify(p[0])).join(', ')}];`,
    'fn word(b: &[u8]) -> u32 { let mut h: u64 = 0xcbf29ce484222325 ^ (b.len() as u64); let mut c = b.chunks_exact(8); for x in &mut c { let mut a = [0u8; 8]; a.copy_from_slice(x); h = (h ^ u64::from_le_bytes(a)).wrapping_mul(0x100000001b3); h ^= h >> 29; } for x in c.remainder() { h = (h ^ *x as u64).wrapping_mul(0x100000001b3); } (h ^ (h >> 32)) as u32 }',
    '/// `o` and `out` are kept by the caller, so that a call grows nothing once they have reached their size', '/// With the first seat standing on Game, as inside a tick: which seat a machine shows then moves no word', 'pub fn state_words(g: &mut crate::game::Game, o: &mut W, out: &mut Vec<u32>) { out.clear(); crate::seats::on_first_bare(g, &mut |g: &mut crate::game::Game| {', ...P.map(p => `    o.b.clear(); ${p[1].startsWith('{') ? p[1] : `${p[1]}.w(o);`} out.push(word(&o.b));`), '    o.b.clear(); }); }', ''];
  return L.join('\n'); }
/** each table's length by its ID's name: what an ID from outside the game is measured against */
function idTables(S: any): string { return ['//! How long each table is, by the name of its ID. Written by tools/wire from the schema; a value from outside the game is measured against this (io.rs, `R::measured`).', '#![allow(unused_imports)]', 'use alloc::vec::Vec;', 'pub fn id_tables(g: &crate::game::Game) -> Vec<(&\'static str, usize)> { alloc::vec![', ...S.tables.map((t: any) => `    ("${t.id}", g.${t.table}.len()),`), '] }', ''].join('\n'); }
/** every generated file, and the stamp both halves carry */
function gen(check: boolean): void { const T = mkdtempSync(join(tmpdir(), 'wire-')); mkdirSync(join(T, 'bridge'));
  const S0 = read(SCHEMA); writeFileSync(join(T, 'bridge/schema.json'), S0);
  execFileSync(process.execPath, ['run', join(HERE, 'puppetgen.cjs'), T, FIELDS, ROOT], { stdio: ['ignore', 'pipe', 'inherit'] });
  const S = readJ(join(T, 'bridge/schema.json')); delete S.layout;
  // (the stamp is of what the two halves read of each other's bytes. `knobSettled` is no part of that: which settings a page's walk leaves out)
  const settled = S.knobSettled; delete S.knobSettled;
  const O = { 'bridge/puppet-apply.ts': join(BLITZ, 'puppet-apply.ts'), 'bridge/puppet-lists.json': join(BLITZ, 'puppet-lists.json'), 'bridge/out-dispatch.ts': join(BLITZ, 'out-dispatch.ts'), 'puppet.rs': rustFile('puppet.rs') } as Record<string, string>;
  const words = JSON.stringify(S) + Object.keys(O).map(k => read(join(T, k))).join(''); const layout = fnv(words); S.layout = layout; S.knobSettled = settled;
  const ioP = rustFile('io.rs'), io0 = read(ioP), io1 = io0.replace(/\npub const LAYOUT: u32 = \d+;\n?/, '').trimEnd() + `\npub const LAYOUT: u32 = ${layout};\n`;
  const swP = join(ROOT, 'rust/game/src/state_words.rs'); const idP = join(ioP, '../id_tables.rs'); const out: [string, string][] = [[SCHEMA, JSON.stringify(S)], [ioP, io1], [swP, stateWords(io1)], [idP, idTables(S)], ...Object.entries(O).map(([k, p]) => [p, read(join(T, k))] as [string, string])];
  // THE TOOL NEVER WRITES A TYPE. The game's types are source a person edits (each module's types.rs, knobs.rs, ids.rs); this list of files is all gen touches, and the two Rust files it makes in whole declare none.
  const made = new Set([SCHEMA, ioP, swP, idP, ...Object.values(O)]); for (const [p, t] of out) { if (!made.has(p)) die(`${p.slice(ROOT.length + 1)}: gen writes no file off its list`); if (p.endsWith('.rs') && p !== ioP && /^\s*(pub(\([a-z]+\))?\s+)?(struct|enum|union)\s/m.test(t)) die(`${p.slice(ROOT.length + 1)}: a generated file declares a type; the game's types are written by hand`); }
  let moved = 0; for (const [p, t] of out) { const was = existsSync(p) ? read(p) : ''; if (was === t) continue; moved++; if (check) console.log(`wire: ${p.slice(ROOT.length + 1)} would change`); else writeFileSync(p, t); }
  rmSync(T, { recursive: true, force: true }); console.log(`wire: ${moved ? `${moved} files ${check ? 'differ' : 'written'}` : 'no file changes'}; layout ${layout}`); if (check && moved) process.exit(1); }
const camel = (x: string) => x.replace(/_([a-z0-9])/g, (_, c) => c.toUpperCase());
const parseQ = (q: string) => { const m = q.match(/^([a-z_0-9]+)::([A-Za-z_0-9]+)\.([a-z_0-9_]+)$/); if (!m) die(`${q}: write it module::Type.field`); return { mod: m![1], name: m![2], f: m![3], path: `crate::${m![1]}::${m![2]}` }; };
const edit = (t: string, a: string | RegExp, b: string, what: string) => { const n = t.replace(a, b); if (n === t) die(`${what}: its line on the wire is not of a form this tool knows`); return n; };
function linesOf(io: string[], kind: 'Io' | 'Poke', path: string): number { const h = `impl ${kind} for ${path} {`; const k = io.findIndex(l => l.startsWith(h)); if (k < 0) die(`${path} has no ${kind} line on the wire`); return k; }
/** every source file the page or a tool runs, and whether one reads this name */
function readers(name: string): string[] { const out: string[] = [], re = new RegExp(`(\\.|\\?\\.)${name}\\b(?!\\s*=[^=>])|\\['${name}'\\]|\\b${name}\\s*[,}]`); const walk = (d: string) => { for (const e of readdirSync(d)) { const p = join(d, e); if (statSync(p).isDirectory()) walk(p); else if (/\.tsx?$/.test(e) && re.test(read(p))) out.push(p.slice(ROOT.length + 1)); } }; walk(join(ROOT, 'src')); return out; }
function widthOf(ty: string): string { const W: Record<string, number> = { f64: 8, f32: 4, u32: 4, i32: 4, usize: 4, bool: 1, u8: 1, i8: 1, u16: 2, i16: 2 }; return W[ty] !== undefined ? `${W[ty]} bytes when it moves` : 'its encoded bytes when it moves'; }
function add(q: string, ty: string, ts: string | undefined, expr: string | undefined, startOnly: boolean): void { const { mod, name, f, path } = parseQ(q); const S = readJ(SCHEMA), T = S.types[path]; if (!T || T.k !== 'struct') die(`${path} is no struct of the table: \`wire add-struct\` first`);
  if (T.fields.some((x: any) => x.rust === f)) die(`${q} is already on the wire`);
  const tsName = ts ?? camel(f), R = readers(tsName); if (!R.length) die(`nothing under src/ reads '.${tsName}'. A state only Rust uses needs no line in the table: it is a field in Rust and nothing more. If the page is about to read it, write the reader first, or give the name it reads with --ts.`);
  const row: any = { rust: f, ty, ts: tsName }; if (expr) row.expr = expr; T.fields.push(row);
  const ioP = rustFile('io.rs'), io = read(ioP).split('\n'), k = linesOf(io, 'Io', path); let l = io[k];
  l = edit(l, ' } fn r(i: &mut R) -> Self {', ` self.${f}.w(o); } fn r(i: &mut R) -> Self {`, q);
  l = l.includes(' s } fn ri(') ? edit(l, ' s } fn ri(', ` s.${f} = Io::r(i); s } fn ri(`, q) : edit(l, ', } } fn ri(', `, ${f}: Io::r(i), } } fn ri(`, q);
  l = edit(l, / \} \}$/, ` self.${f}.ri(i); } }`, q); io[k] = l;
  const p = linesOf(io, 'Poke', path); io[p] = edit(io[p], ' k => panic!', ` ${T.fields.length - 1} => self.${f}.poke(i), k => panic!`, q); writeFileSync(ioP, io.join('\n'));
  writeFileSync(SCHEMA, JSON.stringify(S)); const L = readJ(FIELDS), before = L.pageTick.length; for (const k2 of startOnly ? ['atStartOnly'] : ['everyTick', 'pageTick']) if (!L[k2].includes(`${mod}::${name}.${f}`)) L[k2].push(`${mod}::${name}.${f}`); writeFields(L); gen(false);
  console.log(`wire: ${q} is on the page's list (${startOnly ? 'at a start only' : `${before} fields a tick, now ${L.pageTick.length}`}): it costs ${widthOf(ty)}. It is read by ${R.slice(0, 3).join(', ')}${R.length > 3 ? ` and ${R.length - 3} more` : ''}. Build the module, then \`tools/wire/cost.ts\` says what a tick costs now.`); }
function addStruct(path0: string, fields: string): void { const m = path0.match(/^([a-z_0-9]+)::([A-Za-z_0-9]+)$/); if (!m) die(`${path0}: write it module::Type`); const path = `crate::${m![1]}::${m![2]}`, S = readJ(SCHEMA); if (S.types[path]) die(`${path} is already in the table`);
  const F = fields.split(',').map(x => { const i = x.indexOf(':'); return { rust: x.slice(0, i).trim(), ty: x.slice(i + 1).trim(), ts: camel(x.slice(0, i).trim()) }; }); S.types[path] = { k: 'struct', mod: m![1], name: m![2], fields: F }; writeFileSync(SCHEMA, JSON.stringify(S));
  const ioP = rustFile('io.rs'), t = read(ioP).replace(/\npub const LAYOUT: u32 = \d+;\n?/, '').trimEnd(); const w = F.map(x => `self.${x.rust}.w(o);`).join(' '), r = F.map(x => `s.${x.rust} = Io::r(i);`).join(' '), ri = F.map(x => `self.${x.rust}.ri(i);`).join(' '), arms = F.map((x, i) => `${i} => self.${x.rust}.poke(i),`).join(' ');
  writeFileSync(ioP, `${t}\nimpl Io for ${path} { fn w(&self, o: &mut W) { ${w} } fn r(i: &mut R) -> Self { let mut s = Self::default(); ${r} s } fn ri(&mut self, i: &mut R) { ${ri} } }\nimpl Poke for ${path} { fn poke(&mut self, i: &mut R) { match u16::r(i) { 0xffff => self.ri(i), ${arms} k => panic!("bridge: no field {} of ${m![2]}", k) } } }\n`); gen(false); console.log(`wire: ${path} is in the table, with ${F.length} fields`); }
function drop(Q: string[]): void { const S = readJ(SCHEMA), L = readJ(FIELDS), ioP = rustFile('io.rs'), io = read(ioP).split('\n'), index = new Map<string, number>(); io.forEach((l, i) => { const m = l.match(/^impl (Io|Poke) for (crate::[A-Za-z_0-9:]+) \{/); if (m) index.set(`${m[1]} ${m[2]}`, i); }); const set = new Set(Q); let n = 0, listed = 0;
  const lineOf = (kind: string, path: string) => { const k = index.get(`${kind} ${path}`); if (k === undefined) die(`${path} has no ${kind} line on the wire`); return k!; };
  for (const q of Q) { const { f, path } = parseQ(q), T = S.types[path]; if (!T) die(`${path} is no type of the table`); const at = T.fields.findIndex((x: any) => x.rust === f); if (at < 0) die(`${q} is not on the wire`); T.fields.splice(at, 1); n++;
    const k = lineOf('Io', path); let l = io[k]; for (const [a, b] of [[` self.${f}.w(o);`, ''], [` s.${f} = Io::r(i);`, ''], [` ${f}: Io::r(i),`, ''], [` self.${f}.ri(i);`, '']]) l = l.split(a).join(b); if (new RegExp(`\\b${f}\\b`).test(l)) die(`${q}: its line on the wire is not of a form this tool knows`); io[k] = l;
    const p = lineOf('Poke', path); let i2 = 0; io[p] = io[p].replace(` ${at} => self.${f}.poke(i),`, '').replace(/ \d+ => self\.([a-z_0-9]+)\.poke\(i\),/g, (_, x) => ` ${i2++} => self.${x}.poke(i),`); }
  for (const k of ['everyTick', 'pageTick', 'atStartOnly']) { const b = L[k].length; L[k] = L[k].filter((x: string) => !set.has(x)); listed += b - L[k].length; } S.knobSettled = (S.knobSettled ?? []).filter((x: string) => !set.has(x));
  writeFileSync(ioP, io.join('\n')); writeFileSync(SCHEMA, JSON.stringify(S)); writeFields(L); gen(false); console.log(`wire: ${n} fields off the wire (${listed} list rows)`); }
/** a plain enum on the table: its words, `word` or `word:Tag` where the Rust arm is not the word with a capital first. An arm it doesn't know goes out as the tag 0xffff and the page keeps what it had */
function addEnum(path0: string, list: string): void { const m = path0.match(/^([a-z_0-9]+)::([A-Za-z_0-9]+)$/); if (!m) die(`${path0}: write it module::Type`); const path = `crate::${m![1]}::${m![2]}`, S = readJ(SCHEMA); if (S.types[path]) die(`${path} is on the table already`);
  const rows = list.split(',').map(x => x.trim()).filter(Boolean).map(x => { const [w, tag] = x.split(':'); return { w, tag: tag ?? w[0].toUpperCase() + w.slice(1) }; }); if (!rows.length) die('add-enum: no words'); for (const r of rows) if (!/^[A-Z][A-Za-z0-9_]*$/.test(r.tag)) die(`add-enum: ${r.tag} is no Rust arm: write ${r.w}:Tag`);
  S.types[path] = { k: 'enum', mod: m![1], name: m![2], words: rows.map(r => r.w), tags: rows.map(r => r.tag) }; writeFileSync(SCHEMA, JSON.stringify(S));
  const ioP = rustFile('io.rs'), t = read(ioP).replace(/\npub const LAYOUT: u32 = \d+;\n?/, '').trimEnd();
  const w = rows.map((r, i) => `Self::${r.tag} => ${i}u16,`).join(' '), rr = rows.map((r, i) => `${i} => Self::${r.tag},`).join(' ');
  writeFileSync(ioP, `${t}\nimpl Io for ${path} { fn w(&self, o: &mut W) { { let t: u16 = match self { ${w} #[allow(unreachable_patterns)] _ => 0xffff, }; t.w(o) } } fn r(i: &mut R) -> Self { match u16::r(i) { ${rr} _ => panic!("bridge: no such word of ${m![2]}") } } }\nimpl Poke for ${path} {}\n`); gen(false); console.log(`wire: ${path} is in the table, with ${rows.length} words`); }
/** a type off the table, once no field names it: its lines on the wire go with it. The Rust type itself is a person's to delete */
function dropType(Q: string[]): void { const S = readJ(SCHEMA), ioP = rustFile('io.rs'); let io = read(ioP).split('\n');
  for (const q of Q) { const path = q.startsWith('crate::') ? q : `crate::${q}`, T = S.types[path]; if (!T) die(`${path} is no type of the table`); const named = new RegExp(`\\b${path.replace(/[.*+?^${}()|[\]\\]/g, '\\$&')}\\b`);
    const users = Object.entries(S.types).filter(([k, x]: [string, any]) => k !== path && JSON.stringify([x.fields, x.vars]).match(named)).map(([k]) => k); if (users.length) die(`${path} is still named by ${users.slice(0, 5).join(', ')}`);
    delete S.types[path]; const before = io.length; io = io.filter(l => !l.startsWith(`impl Io for ${path} {`) && !l.startsWith(`impl Poke for ${path} {`)); console.log(`wire: ${path} off the table, ${before - io.length} lines on the wire`); }
  writeFileSync(ioP, io.join('\n')); writeFileSync(SCHEMA, JSON.stringify(S)); gen(false); }
const A = process.argv.slice(2), opt = (n: string) => { const i = A.indexOf(n); return i < 0 ? undefined : A[i + 1]; };
switch (A[0]) { case 'gen': gen(A.includes('--check')); break;
  case 'add': add(A[1], opt('--ty') ?? die('add: --ty') as never, opt('--ts'), opt('--expr'), A.includes('--start-only')); break;
  case 'add-struct': addStruct(A[1], A[2]); break;
  case 'add-enum': addEnum(A[1], A[2]); break;
  case 'drop-type': dropType(A.slice(1)); break;
  case 'drop': { const f = opt('--file'); drop(f ? read(f).split('\n').map(x => x.trim()).filter(Boolean) : A.slice(1)); break; }
  default: console.error(read(import.meta.path).split('\n').slice(0, 7).join('\n')); process.exit(64); }
