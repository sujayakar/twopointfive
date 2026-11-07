// ROWS FOR THE EXPECTED REDS, PRINTED FROM A RUN: reads what `bun run tools/qa/probes.ts <name>` printed and writes one row of tools/qa/expected-red.ts for every
// red probe whose failing lines are all the page's own side (nothing refused by the fold, no line made with the other side of a settled switch).
//   bun run tools/qa/probes.ts glasstune > run.txt 2>&1 ; bun run tools/qa/red-rows.ts run.txt [more.txt …] [--whose "<owner>"] [--read 2026-11-07]
// A probe that is in two of the files is read from the first one named, so the newest run goes first.
// A row is NOT a pardon: the list changes no verdict (expected-red.ts says so). It is there so a gate's reds sort into the ones already read and the new one.
// What the tool refuses to write a row for, and says why in a comment line instead:
//   · a probe that still WRITES a settled row (its fold line names refused writes), or has a line under the runner's "other side" note: that red is the check's
//     own, and the check is to be cleared first (the arm cut, the pin taken out). A row over it would hide a real red behind a stale one;
//   · a probe that is red with no failing line and no throw in the output (a silent probe);
//   · a probe that is green.
// `--whose` takes a name in capitals as a constant of the table's file (RUST, the default) and anything else as the owner's words.
// What it cannot know: whether a failing line asks a build from before a settled rule (those lines are deleted, not listed). Read each row before it is kept.
// The row's `is`: 'real' where the Rust game panicked (the place is in the row) · 'open' where the check threw on a function that went with the tick or has
// no TypeScript body (the check's to restate or the call surface's to answer, not sorted) · 'maybe' for failing lines: could be real, its owner's read is owed.
// BY HAND AFTERWARDS, because the output cannot say it: a red that follows a row the check made by hand on the tools' copy of the game (`C.set.push`,
// `walls.openCells`, a box's flags written after the stand-up) or a hook it wrapped there is the CHECK's ('stale'): the Rust game never gets such a write and
// never calls such a hook. A panic right after one is that too, not a fault a player meets.
import { readFileSync } from 'node:fs';

const argv = process.argv.slice(2), opt = (k: string, d: string): string => { const i = argv.indexOf(k); return i >= 0 && argv[i + 1] ? argv[i + 1] : d; };
const files = argv.filter((a, i) => !a.startsWith('--') && !argv[i - 1]?.startsWith('--'));
if (!files.length) { console.error('usage: bun run tools/qa/red-rows.ts <run output> … [--whose "<owner>"] [--read <date>]'); process.exit(2); }
const WHOSE = opt('--whose', 'RUST'), READ = opt('--read', new Date().toISOString().slice(0, 10));

interface Seen { fails: string[]; passes: number; panic: string; other: number; threw: string }
const fresh = (): Seen => ({ fails: [], passes: 0, panic: '', other: 0, threw: '' });
const q = (s: string): string => JSON.stringify(s);
/** a failing line cut to what a row can carry: its words, then the numbers it printed */
const short = (s: string, n: number): string => (s.length > n ? s.slice(0, n - 1).replace(/\s+\S*$/, '') + '…' : s);

const rows: string[] = [], notes: string[] = [], done = new Set<string>();
for (const file of files) {
  const lines = readFileSync(file, 'utf8').split('\n'); let cur = fresh();
  for (let i = 0; i < lines.length; i++) {
    const l = lines[i];
    if (/^(- FAIL\b|  FAIL )/.test(l)) { cur.fails.push(l.replace(/^(- FAIL —?\s*|  FAIL\s+)/, '').trim()); continue; }
    if (/^(- PASS\b|  PASS )/.test(l)) { cur.passes++; continue; }
    if (/^\s+\(made with the other side of a settled switch wanted:/.test(l)) { cur.other++; continue; }
    const p = /panicked at (\S+?):?\s*$/.exec(l);   // (the game's own line, or the same words inside the throw the probe died of; lib.rs's is the lost game's echo)
    if (p && !/^src\/lib\.rs/.test(p[1]) && !cur.panic) { cur.panic = `${p[1].replace(/^\.\//, '')} (${(lines[i + 1] ?? '').trim()})`; continue; }
    if (p) continue;
    const sides = /^\s*\{"sides":(\{.*\})\}\s*$/.exec(l);
    if (sides) { try { cur.threw = String(JSON.parse(sides[1]).threw ?? ''); } catch { /* a cut line: the fold line still closes the probe */ } continue; }
    const fold = /^\s*\{"fold":(\{.*\})\}\s*$/.exec(l); if (!fold) continue;
    let f: { probe: string; red: boolean; refused: { row: string }[] }; try { f = JSON.parse(fold[1]); } catch { cur = fresh(); continue; }
    const seen = cur; cur = fresh();
    if (done.has(f.probe)) continue; done.add(f.probe); if (!f.red) continue;   // (a probe read twice: the FIRST file named wins, green or red — name the newest run first)
    if (f.refused.length || seen.other) { notes.push(`// ${f.probe}: NO ROW — it still writes settled rows (${[...new Set(f.refused.map(r => r.row))].join(', ') || 'see its lines'}): clear the check first`); continue; }
    if (!seen.fails.length && !seen.panic && !seen.threw) { notes.push(`// ${f.probe}: NO ROW — red with no failing line and no throw in this output (a silent probe, or lines this tool does not read): read its output`); continue; }
    const n = seen.fails.length, of = n + seen.passes, first = seen.fails[0] ? ` · first: ${short(seen.fails[0], 260)}` : '';
    const row = seen.panic ? { is: 'real', line: `the Rust game panics at ${seen.panic}${of ? ` · ${seen.passes} of ${of} lines passed before it` : ''}` }
      : /went with the tick|has no TypeScript body/.test(seen.threw) ? { is: 'open', line: `throws: ${short(seen.threw, 200)}${of ? ` · ${n} of ${of} lines failed before it${first}` : ''}` }
      : seen.threw ? { is: 'maybe', line: `throws: ${short(seen.threw, 200)}${of ? ` · ${n} of ${of} lines failed before it${first}` : ''}` }
      : { is: 'maybe', line: `${n} of ${of} lines fail, nothing refused${first}` };
    rows.push(`  ${q(f.probe).replace(/"/g, "'")}: { is: '${row.is}', rust: true, line: ${q(`read ${READ} · ${row.line}`)}, whose: ${/^[A-Z_]+$/.test(WHOSE) ? WHOSE : q(WHOSE)} },`);
  }
}
for (const n of notes) console.log(n);
for (const r of rows.sort()) console.log(r);
console.error(`${rows.length} row${rows.length === 1 ? '' : 's'}, ${notes.length} red probe${notes.length === 1 ? '' : 's'} with no row (each says why above)`);
