// WHICH PART OF THE GAME TWO MACHINES DISAGREE ON. Every machine of a networked game stood through tools/mp/stand.ts with MP_WORDS=<path> writes the whole
// game's words after each word tick to `<path>.<pid>.words` (src/net/step.ts netWhole: one word a part, the parts' names on the first line). This holds
// those files to each other, tick by tick and part by part:
//   MP_WORDS=/tmp/run bun run tools/qa/mpserver-check.ts ; bun run tools/mp/words-diff.ts /tmp/run.*.words
// A line a part that ever differed: the first tick it did, how many of the ticks both files hold, and between which files. exit 0 = no part differed in any
// tick two files share · 1 = some did · 2 = nothing to compare. Files of DIFFERENT games (another server's, another seed) share no meaning: hand it one
// game's files. A page's words of its last word tick are `__net.whole` in the page ({ t, words, names }), written to a file of the same shape by whoever drives it.
import { readFileSync } from 'node:fs';
const files = process.argv.slice(2); if (files.length < 2) { console.error('usage: bun run tools/mp/words-diff.ts <a.words> <b.words> [more]'); process.exit(2); }
interface Run { file: string; names: string[]; at: Map<number, string[]> }
const runs: Run[] = files.map(file => { const L = readFileSync(file, 'utf8').split('\n').filter(Boolean), R: Run = { file: file.split('/').pop()!, names: [], at: new Map() };
  for (const l of L) { const w = l.split(' '); if (w[0] === 'names') R.names = w.slice(1); else R.at.set(Number(w[0]), w.slice(1)); } return R; });
let shared = 0; const first = new Map<string, { t: number; n: number; who: Set<string> }>();
for (let a = 0; a < runs.length; a++) for (let b = a + 1; b < runs.length; b++) { const A = runs[a], B = runs[b];
  if (A.names.join(' ') !== B.names.join(' ')) { console.log(`${A.file} and ${B.file} name their parts differently (two builds): not compared`); continue; }
  for (const [t, wa] of A.at) { const wb = B.at.get(t); if (!wb) continue; shared++;
    for (let i = 0; i < wa.length; i++) if (wa[i] !== wb[i]) { const k = A.names[i] ?? `part ${i}`, e = first.get(k) ?? { t, n: 0, who: new Set<string>() }; if (t < e.t) e.t = t; e.n++; e.who.add(`${A.file} ≠ ${B.file}`); first.set(k, e); } } }
if (!shared) { console.log('words-diff: no two files hold the same tick'); process.exit(2); }
console.log(`words-diff: ${runs.length} files (${runs.map(r => `${r.file} ${r.at.size} ticks`).join(', ')}), ${shared} ticks held to each other pair by pair, ${runs[0].names.length} parts`);
for (const [k, e] of [...first].sort((x, y) => x[1].t - y[1].t)) console.log(`  ${k}: first at tick ${e.t}, ${e.n} time${e.n === 1 ? '' : 's'} · ${[...e.who].slice(0, 3).join(' · ')}${e.who.size > 3 ? ` · and ${e.who.size - 3} more pairs` : ''}`);
console.log(first.size ? `words-diff: ${first.size} part${first.size === 1 ? '' : 's'} differed` : 'words-diff: every part agreed on every shared tick');
process.exit(first.size ? 1 : 0);
