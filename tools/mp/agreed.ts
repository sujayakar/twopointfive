// THE LIST OF PARTS EVERY KIND OF HOST AGREES ON, MADE FROM RUNS (src/net/agreed.ts: what the wider word folds, src/net/step.ts netWatch).
//   bun run tools/mp/agreed.ts <folder> [<folder> …] [--leave part,part] [--write]
// Each folder holds ONE game's whole-words files, a file a machine (`*.words`: MP_WORDS in front of anything stood through tools/mp/stand.ts; a page's
// are written by whoever drives it from `__net.whole`). A part is on the list when, in every folder, every pair of machines had the same word for it on
// every word tick they share, and it is not named by --leave (a part with the picture's fields in it that happened to agree). Printed: the parts that
// differed, with the folders they differed in. --write lays the list into src/net/agreed.ts between its two marks; without it nothing is written.
// A LIST MADE FROM RUNS IS ONLY AS WIDE AS THE RUNS: a page beside its server, one seat and two, glass broken, smoke and a frag thrown, a storm.
// exit 0 = a list was made · 2 = a folder had fewer than two files, or two builds' files were mixed.
import { readFileSync, readdirSync, writeFileSync } from 'node:fs';
import { resolve } from 'node:path';
const ROOT = resolve(import.meta.dir, '../..'), A = process.argv.slice(2), WRITE = A.includes('--write');
const leave = new Set((A.includes('--leave') ? A[A.indexOf('--leave') + 1] ?? '' : '').split(',').filter(Boolean));
const dirs = A.filter((x, i) => !x.startsWith('--') && A[i - 1] !== '--leave');
if (!dirs.length) { console.error('usage: bun run tools/mp/agreed.ts <folder of one game\'s .words files> [more folders] [--leave part,part] [--write]'); process.exit(2); }
let names: string[] | null = null; const off = new Map<string, Set<string>>(); let ticks = 0;
for (const d of dirs) {
  const files = readdirSync(d).filter(f => f.endsWith('.words')); if (files.length < 2) { console.error(`${d}: ${files.length} words file(s); two machines are the least`); process.exit(2); }
  const runs = files.map(f => { const L = readFileSync(`${d}/${f}`, 'utf8').split('\n').filter(Boolean), at = new Map<number, string[]>(); let N: string[] = [];
    for (const l of L) { const w = l.split(' '); if (w[0] === 'names') N = w.slice(1); else at.set(Number(w[0]), w.slice(1)); } return { N, at }; });
  for (const r of runs) { names ??= r.N; if (r.N.join(' ') !== names.join(' ')) { console.error(`${d}: its files name other parts than the first folder's (two builds)`); process.exit(2); } }
  for (let a = 0; a < runs.length; a++) for (let b = a + 1; b < runs.length; b++) for (const [t, wa] of runs[a].at) { const wb = runs[b].at.get(t); if (!wb) continue; ticks++;
    for (let i = 0; i < wa.length; i++) if (wa[i] !== wb[i]) { const k = names![i]; if (!off.has(k)) off.set(k, new Set()); off.get(k)!.add(d.split('/').filter(Boolean).pop()!); } }
}
const list = names!.filter(n => !off.has(n) && !leave.has(n) && !n.startsWith('block.'));   // (the alarm's own parts are the alarm's: step.ts netWhole)
console.log(`agreed.ts: ${dirs.length} games, ${ticks} word ticks held pair by pair, ${names!.filter(n => !n.startsWith('block.')).length} parts: ${list.length} agreed everywhere, ${off.size} differed somewhere${leave.size ? `, ${[...leave].filter(n => !off.has(n)).length} left out by name` : ''}`);
for (const [k, where] of off) console.log(`  differed: ${k} (${[...where].join(', ')})`);
if (WRITE) { const p = `${ROOT}/src/net/agreed.ts`, s = readFileSync(p, 'utf8'), a = s.indexOf('// <list>'), b = s.indexOf('// </list>'); if (a < 0 || b < 0) { console.error('src/net/agreed.ts has lost its marks'); process.exit(2); }
  const rows: string[] = []; let row = ' '; for (const n of list) { const w = ` '${n}',`; if (row.length + w.length > 150) { rows.push(row); row = ' '; } row += w; } if (row.trim()) rows.push(row);
  writeFileSync(p, `${s.slice(0, a)}// <list>\n${rows.join('\n')}\n${s.slice(b)}`); console.log(`agreed.ts: ${list.length} parts written to src/net/agreed.ts`); }
