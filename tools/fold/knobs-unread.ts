// THE KNOB ROWS NOTHING READS, FROM THE COMPILER: every field of the tables marked #[deprecated] on a copy and every crate checked gives a warning at each place a field is named, whatever its name
// (`on` is on every table, so no text search can tell one from another). A row whose only places are the tables, the wire, an assignment or a struct literal has no reader; nor has a line of the
// simulation's list that the simulation never reads. This writes the lists the drop takes.
//   cargo check --message-format=short in each crate, its output a log (no tests: a row only a test reads goes, and its test is mended by hand), then
//   bun run tools/fold/knobs-unread.ts <copy's rust dir> <out dir> <crate>=<its log>…      out: fields.txt (`knobs::Table.field`), types.txt (`knobs::Table`), sim.txt (a name of the simulation's list a line)
import { readFileSync, writeFileSync, readdirSync, statSync } from 'node:fs';
import { join } from 'node:path';
const [RUST, OUT, ...LOGS] = process.argv.slice(2);
const read = new Set<string>(), named = new Set<string>();
const files = (d: string, out: string[] = []): string[] => { for (const n of readdirSync(d)) { if (n === 'target') continue; const p = join(d, n); if (statSync(p).isDirectory()) files(p, out); else if (p.endsWith('.rs')) out.push(p); } return out; };
const all = files(RUST), simList = readFileSync(join(RUST, 'sim/src/knobs.rs'), 'utf8'), body = simList.slice(simList.indexOf('table! { Knob, KNOBS, KNOB_NAMES;')), names = [...body.slice(0, body.indexOf('\n}')).matchAll(/^\s*(\w+) = "/gm)].map(m => m[1]);
// a line of the simulation's list is read if any file but the list itself and base.rs names it
const others = all.filter(p => !p.endsWith('sim/src/knobs.rs') && !p.endsWith('rustknobs/base.rs')).map(p => readFileSync(p, 'utf8')).join('\n'), words = new Set(others.match(/\b[A-Z][A-Z0-9_]+\b/g) ?? []);
const simUnread = names.filter(n => !words.has(n));
const unreadLine = (line: string): boolean => { const m = /^\s*k\[(\w+)\] = /.exec(line); return !!m && simUnread.includes(m[1]); };
const lines = new Map<string, string[]>(); const textOf = (p: string): string[] => { if (!lines.has(p)) lines.set(p, readFileSync(p, 'utf8').split('\n')); return lines.get(p)!; };
if (!LOGS.length) throw new Error('no logs: with none every row reads as unread');
for (const crateLog of LOGS) { const [crate, log] = crateLog.split('='); const logText = readFileSync(log, 'utf8'); if (!/warning: use of deprecated field/.test(logText)) throw new Error(`${log} has no warning in the short format`); for (const m of logText.matchAll(/^(\S+?):(\d+):(\d+): warning: use of deprecated field `\S*?knobs::(\w+)::(\w+)`/gm)) {
  const [, path, l, c, table, field] = m, key = `${table}.${field}`; named.add(key);
  if (/game_types\/src\/knobs\/|game_wire\//.test(path)) continue;
  const p = path.startsWith('/') ? path : join(RUST, crate, path), line = textOf(p)[+l - 1], rest = line.slice(+c - 1);
  if (line.includes('rows_same!') || unreadLine(line)) continue;
  if (/^\w+\s*=[^=]/.test(rest) || /^\w+:\s/.test(rest)) continue;   // (a write, or a value in a literal)
  read.add(key);
} }
const tables = readdirSync(join(RUST, 'game_types/src/knobs')).map(n => readFileSync(join(RUST, 'game_types/src/knobs', n), 'utf8')).join('\n');
const fields = new Map<string, { f: string; ty: string }[]>();
for (const m of tables.matchAll(/pub struct (\w+) \{\n?(.*?)\n?^\}/gms)) fields.set(m[1], [...m[2].matchAll(/pub (\w+): ([^,\n]+),/g)].map(x => ({ f: x[1], ty: x[2].replace(/crate::knobs::/g, '') })));
const dead = new Set<string>(), gone = new Set<string>();
for (const [t, fs] of fields) if (t !== 'Knobs') for (const { f, ty } of fs) if (!fields.has(ty) && !read.has(`${t}.${f}`)) dead.add(`${t}.${f}`);
for (let moved = true; moved;) { moved = false;
  for (const [t, fs] of fields) { if (t === 'Knobs' || gone.has(t)) continue; if (fs.every(x => dead.has(`${t}.${x.f}`))) { gone.add(t); moved = true; } }
  for (const [t, fs] of fields) for (const { f, ty } of fs) if (gone.has(ty) && !dead.has(`${t}.${f}`)) { dead.add(`${t}.${f}`); moved = true; } }
writeFileSync(join(OUT, 'fields.txt'), [...dead].sort().map(x => `knobs::${x}\n`).join(''));
writeFileSync(join(OUT, 'types.txt'), [...gone].sort().map(x => `knobs::${x}\n`).join(''));
writeFileSync(join(OUT, 'sim.txt'), simUnread.map(x => `${x}\n`).join(''));
console.log(`${[...fields.values()].flat().length} fields, ${named.size} named somewhere, ${read.size} read; ${dead.size} rows and ${gone.size} tables have no reader; ${simUnread.length} of ${names.length} lines of the simulation's list are read by nothing`);
