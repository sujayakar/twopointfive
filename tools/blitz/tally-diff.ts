// TWO TALLIES OF THE SAME RUNS, SIDE BY SIDE (PLAY.md, "The harness's word"): each file is a `tools/qa/playthrough.ts --json`, one made with the TypeScript
// tick (`tools/blitz/rust.sh ts …`) and one with the Rust game stepping (`tools/blitz/rust.sh rust …`). A row a run: how it ended on each side, and every
// place of the run's record that differs, by its first key (a tally that differs where the play was equal tick for tick is the harness reading something
// the puppet lacks: the key names the reader).
//   bun run tools/blitz/tally-diff.ts <ts.json> <rust.json> [--md] [--show N]
import { readFileSync } from 'node:fs';
const argv = process.argv.slice(2), files = argv.filter(a => !a.startsWith('--') && !/^\d+$/.test(a)), md = argv.includes('--md'), show = +(argv[argv.indexOf('--show') + 1] ?? 6) || 6;
if (files.length !== 2) { console.error('tally-diff.ts <ts.json> <rust.json> [--md] [--show N]'); process.exit(2); }
const [A, B] = files.map(f => { const J = JSON.parse(readFileSync(f, 'utf8')); return (J.runs ?? J) as any[]; });
/** every place two values differ, as paths (`trace` and wall-clock figures apart: a trace is the commander's own words, `frameMs` a clock) */
function diff(a: any, b: any, path: string, out: string[]): void {
  if (out.length > 4000) return;
  if (a === b || (typeof a === 'number' && typeof b === 'number' && Number.isNaN(a) && Number.isNaN(b))) return;
  if (a && b && typeof a === 'object' && typeof b === 'object') { const ks = new Set([...Object.keys(a), ...Object.keys(b)]); for (const k of ks) diff(a[k], b[k], path ? `${path}.${k}` : k, out); return; }
  out.push(`${path}: ${JSON.stringify(a)?.slice(0, 60)} | ${JSON.stringify(b)?.slice(0, 60)}`); }
const SKIP = new Set(['trace', 'frameMs']);
const rows: string[][] = []; let same = 0, n = 0; const byKey = new Map<string, number>();
for (const a of A) { const b = B.find(x => x.scn === a.scn && x.seed === a.seed); n++;
  const end = (r: any) => r ? `${r.end} at ${r.t} s, ${r.team} of ${r.of} up, ${r.hostiles} of ${r.hostilesOf} theirs left` : 'not played';
  const out: string[] = []; if (b) for (const k of new Set([...Object.keys(a), ...Object.keys(b)])) if (!SKIP.has(k)) diff(a[k], b[k], k, out);
  const keys = new Map<string, number>(); for (const l of out) { const k = l.split(/[.:]/)[0]; keys.set(k, (keys.get(k) ?? 0) + 1); }
  for (const k of keys.keys()) byKey.set(k, (byKey.get(k) ?? 0) + 1);
  if (b && !out.length) same++;
  rows.push([`${a.scn}:${a.seed}`, end(a), end(b), !b ? '—' : out.length ? `${out.length} places: ${[...keys].map(([k, c]) => `${k} ${c}`).join(', ')}` : 'the same record', out.slice(0, show).join(' · ')]); }
const head = ['run', 'the TypeScript tick', 'the Rust game', 'the record', 'the first places'];
if (md) { console.log(`| ${head.join(' | ')} |`); console.log(`|${'---|'.repeat(head.length)}`); for (const r of rows) console.log(`| ${r.join(' | ')} |`); }
else for (const r of rows) console.log(r.map((c, i) => `${head[i]}: ${c}`).join('\n  ') + '\n');
console.log(`${same} of ${n} runs have the same record on both sides (\`trace\` and \`frameMs\` apart).${byKey.size ? ` Keys that differ, by how many runs: ${[...byKey].sort((x, y) => y[1] - x[1]).map(([k, c]) => `${k} ${c}`).join(', ')}` : ''}`);
