// WHAT A ROW THE APPLY MAKES WOULD LACK, class by class. A row the bridge's apply makes is its class's prototype and the fields the bridge carries
// (tools/blitz/PAGE.md, "A row the game makes at run time"). Whatever a class's constructor sets beside those is not on a made row: its picture half.
// This joins the two lists that say how much that is:
//   tools/blitz/schema.json                       every type the bridge carries, field by field
//   the readers' table (delete-tick.ts --readers)  every field declared in the simulation's folders, and who still reads it
// and answers, for every carried type that is a declared class: which declared fields are NOT carried, and whether anything left in the page reads them.
//   bun run tools/blitz/delete-tick.ts --readers /tmp/readers.tsv        (the table: a minute)
//   bun run tools/blitz/made-rows.ts --readers /tmp/readers.tsv [--starts a.bin,b.bin] [--all] [--json out.json]
// --starts: whole states as the bridge writes them (a start's n1.bin or c1.bin: tools/blitz/start.sh --out DIR), for how many rows of each table a
//           start holds (diff.ts --tables)
// --all:    every class with an uncarried field, not only those somebody reads
// A class is matched by its name and the file its module names (`scene_boxes` = src/scene/boxes.ts). A type written in place has no declared class and
// is plain data by nature: counted, not listed.
// WHAT IT CANNOT SEE is the readers' table's own limit: a field read by a computed key, a spread or a generic walk; and a constructor that does work
// with no field to show for it (a callback hooked on the world, a row registered on a menu).
import { readFileSync, writeFileSync } from 'node:fs';
import { spawnSync } from 'node:child_process';
import { resolve } from 'node:path';
const ROOT = resolve(import.meta.dir, '../..');
const argv = process.argv.slice(2), opt = (n: string) => { const i = argv.indexOf(n); return i < 0 ? undefined : argv[i + 1]; };
const say = (s = '') => process.stdout.write(s + '\n');
const READERS = opt('--readers');
if (!READERS) { say('made-rows.ts --readers <readers.tsv> [--starts a.bin,b.bin] [--all] [--json out.json]   (the table: bun run tools/blitz/delete-tick.ts --readers <readers.tsv>)'); process.exit(64); }
const S = JSON.parse(readFileSync(resolve(ROOT, 'tools/blitz/schema.json'), 'utf8'));

// ---- the readers' table: a class's fields (a row `Class.field`; a key of a type written inside another is a longer path and is not a class's field)
const lines = readFileSync(READERS, 'utf8').split('\n').filter(Boolean), head = lines[0].split('\t'), col = (n: string) => { const i = head.indexOf(n); if (i < 0) throw new Error(`made-rows: ${READERS} has no column '${n}'`); return i; };
const C = { field: col('field'), file: col('declared in'), picture: col('picture'), sound: col('sound'), hud: col('hud or menu'), host: col('page host'), stays: col('simulation code that stays'), seen: col('of those, seen running in a page play') };
type Reads = { outside: number; picture: number; seen: number; stays: number };
const declared = new Map<string, Map<string, Reads>>();   // `Class\tfile` → field → who reads it
for (const l of lines.slice(1)) { const c = l.split('\t'), p = c[C.field].split('.'); if (p.length !== 2) continue;
  const key = `${p[0]}\t${c[C.file]}`, n = (i: number) => +c[i] || 0; let M = declared.get(key); if (!M) declared.set(key, M = new Map());
  M.set(p[1], { outside: n(C.picture) + n(C.sound) + n(C.hud) + n(C.host), picture: n(C.picture), seen: n(C.seen), stays: n(C.stays) }); }

// ---- the join
const flat = (s: string) => s.toLowerCase().replace(/[^a-z0-9]/g, '');
const fileOf = (mod: string): [string, string] => { for (const [pre, dir] of [['scene_', 'src/scene/'], ['anim_', 'src/anim/'], ['render_', 'src/render/'], ['sim_', 'src/sim/'], ['math_', 'src/math/'], ['host_', 'src/host/']]) if (mod.startsWith(pre)) return [dir, flat(mod.slice(pre.length))]; return ['src/game/', flat(mod)]; };
const camel = (x: string) => x.replace(/_([a-z0-9])/g, (_, c) => c.toUpperCase());
const TABLE_OF = new Map<string, any>(S.tables.map((t: any) => [t.ty, t]));
interface Row { name: string; file: string; table: boolean; carried: number; declared: number; uncarried: string[]; outside: string[]; picture: string[]; seen: string[]; unseen: string[]; unread: string[] }
const rows: Row[] = []; let inPlace = 0, structs = 0;
for (const [path, ty] of Object.entries<any>(S.types)) { if (ty.k !== 'struct') continue; structs++;
  const name: string = ty.ts ?? ty.name, [dir, stem] = fileOf(ty.mod);
  const key = [...declared.keys()].find(k => { const [n, f] = k.split('\t'); return n === name && f.startsWith(dir) && flat(f.split('/').pop()!.replace(/\.ts$/, '')) === stem; });
  if (!key) { inPlace++; continue; }
  const F = declared.get(key)!, carried = new Set<string>(ty.fields.map((f: any) => f.ts ?? camel(f.rust))), un = [...F.keys()].filter(n => !carried.has(n));
  const outside = un.filter(n => F.get(n)!.outside > 0), seen = un.filter(n => !F.get(n)!.outside && F.get(n)!.seen > 0), unseen = un.filter(n => !F.get(n)!.outside && !F.get(n)!.seen && F.get(n)!.stays > 0);
  rows.push({ name, file: key.split('\t')[1], table: TABLE_OF.has(path), carried: ty.fields.length, declared: F.size, uncarried: un, outside, picture: un.filter(n => F.get(n)!.picture > 0), seen, unseen, unread: un.filter(n => !outside.includes(n) && !seen.includes(n) && !unseen.includes(n)) }); }

// ---- how many rows a start holds of each table
const STARTS = (opt('--starts') ?? '').split(',').filter(Boolean), counts = new Map<string, number[]>();
for (const f of STARTS) { const r = spawnSync('bun', ['run', 'tools/blitz/diff.ts', resolve(f), resolve(f), '--tables'], { cwd: ROOT, encoding: 'utf8', env: { ...process.env, BUN_OPTIONS: '' } });
  if (r.status !== 0) { say(`made-rows: ${f} could not be read (diff.ts --tables): ${(r.stdout + r.stderr).trim().split('\n').pop()}`); process.exit(3); }
  for (const l of r.stdout.split('\n')) { const [n, c] = l.split('\t'); if (c === undefined) continue; const a = counts.get(n) ?? []; a.push(+c); counts.set(n, a); } }

// ---- said
const sum = (f: (r: Row) => number) => rows.reduce((n, r) => n + f(r), 0), plain = rows.filter(r => !r.uncarried.length), tabs = rows.filter(r => r.table);
const read = rows.filter(r => r.outside.length || r.seen.length);
say(`the bridge carries ${structs} struct types; ${rows.length} are a declared class the readers' table has fields for (${inPlace} are written in place: plain data)`);
say(`every declared field carried, PLAIN DATA TODAY: ${plain.length} of ${rows.length} classes; of the ${S.tables.length} row classes, ${tabs.filter(r => !r.uncarried.length).length} of the ${tabs.length} matched`);
say(`fields declared and NOT carried: ${sum(r => r.uncarried.length)}, in ${rows.length - plain.length} classes`);
say(`  read from outside the simulation's folders (the picture, the sound, the HUD, a menu, the page's host): ${sum(r => r.outside.length)} (the picture: ${sum(r => r.picture.length)})`);
say(`  read by simulation code that stays and was seen running in a page play:                             ${sum(r => r.seen.length)}`);
say(`  read by simulation code that stays, never seen in a page play:                                      ${sum(r => r.unseen.length)}`);
say(`  read by nobody left:                                                                                ${sum(r => r.unread.length)}`);
say(`classes with an uncarried field somebody reads: ${read.length}, ${sum(r => r.outside.length + r.seen.length)} fields`);
if (STARTS.length) { const empty = S.tables.filter((t: any) => (counts.get(t.ts) ?? []).every(c => c === 0)).length; say(`of the ${S.tables.length} tables, ${empty} hold no row in any of the ${STARTS.length} starts given (their rows are only made in play)`); }
say();
say(`class\tfile\ta row class\t${STARTS.length ? 'rows at each start\t' : ''}carried\tnot carried\tread from outside\tread by simulation code seen in a page play`);
const shown = (argv.includes('--all') ? rows.filter(r => r.uncarried.length) : read).sort((a, b) => (b.outside.length + b.seen.length) - (a.outside.length + a.seen.length) || a.name.localeCompare(b.name));
const few = (a: string[]) => a.length ? `${a.length}: ${a.slice(0, 8).join(', ')}${a.length > 8 ? ', …' : ''}` : '0';
for (const r of shown) say(`${r.name}\t${r.file}\t${r.table ? 'yes' : ''}\t${STARTS.length ? `${r.table ? (counts.get(r.name) ?? []).join(' · ') : ''}\t` : ''}${r.carried}\t${r.uncarried.length}\t${few(r.outside)}\t${few(r.seen)}`);
if (STARTS.length) { say(); say('the row classes with every field carried, and their rows at each start:');
  for (const t of S.tables as any[]) { const r = rows.find(x => x.table && x.name === t.ts); if (r && r.uncarried.length) continue; say(`${t.ts}\t${(counts.get(t.ts) ?? []).join(' · ')}${r ? '' : '\t(no declared class matched)'}`); } }
{ const j = opt('--json'); if (j) writeFileSync(j, JSON.stringify({ rows, counts: Object.fromEntries(counts) }, null, 1)); }
