// ONE TICK'S DIFFERENCE, in a process of its own so that no more than the two decoded states of that one tick are ever held (run.sh starts one a tick):
//   bun run tools/blitz/tick-diff.ts <dir> <tick> [--show N] [--all file.tsv]
// Compares a<k>.bin (TypeScript) with n<k>.bin (Rust), and rust/sim's block s<k>.bin with ns<k>.bin. BYTES FIRST: two files that are the same bytes are never
// decoded. Paths that match a line of known.txt are counted and left out. Prints one summary line, then the unknown paths by module.
// Exit 0 the same but for what is known · 1 they differ · 3 a blob is missing · 4 over the memory ceiling.
import { readFileSync, existsSync, writeFileSync } from 'node:fs';
import { decode, type Row } from './paths';
const argv = process.argv.slice(2); const opt = (n: string, d: string) => { const i = argv.indexOf(n); return i < 0 ? d : argv.splice(i, 2)[1]; };
const SHOW = +opt('--show', '6'), ALL = opt('--all', ''); const [dir, kS] = argv, k = +kS;
const CEIL = +(process.env.BLITZ_CEIL_GB ?? 4) * 1e9;
const say = (s: string) => process.stdout.write(s + '\n');
const gb = () => (process.memoryUsage().rss / 1e9).toFixed(2);
for (const f of [`a${k}`, `n${k}`, `s${k}`, `ns${k}`]) if (!existsSync(`${dir}/${f}.bin`)) { say(`tick ${k}: ${dir}/${f}.bin is missing`); process.exit(3); }
const known = (): { re: RegExp; why: string; n: number }[] => readFileSync(`${import.meta.dir}/known.txt`, 'utf8').split('\n').filter(l => l && !l.startsWith('#')).map(l => { const [re, why] = l.split('\t'); return { re: new RegExp(re), why: why ?? '', n: 0 }; });

// ---- rust/sim's block, part by part (a pool's first word is the bytes of it in use: what lies past them is nobody's)
let blockBytes = 0; const blockLines: string[] = [];
{ const s = readFileSync(`${dir}/s${k}.bin`), t = readFileSync(`${dir}/ns${k}.bin`);
  if (!s.equals(t)) {
    const at = (x: Uint8Array, i: number) => i < x.length ? x[i] : 0, word = (x: Uint8Array, o: number) => (at(x, o) | at(x, o + 1) << 8 | at(x, o + 2) << 16 | at(x, o + 3) << 24) >>> 0;
    const parts: { name: string; off: number; len: number; pool: boolean }[] = existsSync(`${dir}/layout.json`) ? JSON.parse(readFileSync(`${dir}/layout.json`, 'utf8')).parts : [{ name: 'block', off: 0, len: Math.max(s.length, t.length), pool: false }];
    for (const p of parts) { const live = p.pool ? Math.min(p.len, Math.max(8, word(s, p.off), word(t, p.off))) : p.len; let n = 0, first = -1; for (let i = p.off; i < p.off + live; i++) if (at(s, i) !== at(t, i)) { n++; if (first < 0) first = i - p.off; }
      if (n) { blockBytes += n; blockLines.push(`  block part ${p.name}: ${n} bytes of ${live} live differ, the first at +${first}`); } }
    if (s.length !== t.length) { blockBytes ||= 1; blockLines.push(`  the block is ${s.length} bytes in the TypeScript and ${t.length} in Rust`); }
  } }

// ---- the state
let n = 0, u = 0; const by = new Map<string, { n: number; eg: string[] }>(); const K = known(); const all: string[] = [];
const a = readFileSync(`${dir}/a${k}.bin`), b = readFileSync(`${dir}/n${k}.bin`);
if (!a.equals(b)) {
  const A = decode(new Uint8Array(a)), B = decode(new Uint8Array(b));
  const add = (mod: string, path: string, x: string, y: string) => { n++; if (ALL) all.push(`${mod}\t${path}\t${x}\t${y}`); const kn = K.find(e => e.re.test(path)); if (kn) { kn.n++; return; }
    u++; let e = by.get(mod); if (!e) by.set(mod, e = { n: 0, eg: [] }); e.n++; if (e.eg.length < SHOW) e.eg.push(`${path}   ts ${x} | rust ${y}`); };
  // The two lists are the same walk, so they run side by side: a path is looked for at the place the other list has reached, then a little way on (a list one
  // side has more of), and in an index of the whole list only after a long run out of step. What was not found on the way is matched by path at the end, so the
  // answer is the one tools/blitz/paths.ts `diff` gives, without its two tables of every path.
  const NEAR = 512, LONG = 2000; let j = 0, streak = 0, index: Map<string, number> | null = null; const seen = new Uint8Array(B.length), lost: Row[] = [];
  const cmp = (r: Row, q: Row) => { if (q.bits !== r.bits) add(r.mod, r.path, r.bits, q.bits); };
  for (const r of A) {
    if (j < B.length && B[j].path === r.path) { seen[j] = 1; cmp(r, B[j++]); streak = 0; continue; }
    let i = -1;
    if (index) i = index.get(r.path) ?? -1; else { const end = Math.min(B.length, j + NEAR); for (let x = j + 1; x < end; x++) if (B[x].path === r.path) { i = x; break; } }
    if (i >= 0 && !seen[i]) { seen[i] = 1; cmp(r, B[i]); j = i + 1; streak = 0; continue; }
    lost.push(r); if (!index && ++streak > LONG) { index = new Map(); for (let x = 0; x < B.length; x++) index.set(B[x].path, x); }
  }
  if (lost.length) { const left = new Map<string, number>(); for (let x = 0; x < B.length; x++) if (!seen[x]) left.set(B[x].path, x);
    for (const r of lost) { const x = left.get(r.path); if (x === undefined || seen[x]) add(r.mod, r.path, r.bits, '(absent)'); else { seen[x] = 1; cmp(r, B[x]); } } }
  for (let i = 0; i < B.length; i++) if (!seen[i]) add(B[i].mod, B[i].path, '(absent)', B[i].bits);
}
if (ALL) writeFileSync(ALL, all.join('\n') + '\n');
const mods = [...by].sort((x, y) => y[1].n - x[1].n);
say(`tick ${k}: ${n} paths differ, ${n - u} known, ${u} unknown${u ? ' (' + mods.map(([m, e]) => `${m} ${e.n}`).join(' · ') + ')' : ''}; block ${blockBytes ? `${blockBytes} bytes differ` : 'the same bytes'}; ${gb()} GB`);
for (const [m, e] of mods) { say(`  ${m}: ${e.n}`); for (const x of e.eg) say(`      ${x.length > 230 ? x.slice(0, 230) + '…' : x}`); if (e.n > e.eg.length) say(`      … ${e.n - e.eg.length} more`); }
for (const l of blockLines) say(l);
if (process.env.BLITZ_KNOWN) for (const e of K) if (e.n) say(`  known ${e.n}: ${e.why.slice(0, 160)}`);
if (process.memoryUsage().rss > CEIL) { say(`tick ${k}: over the memory ceiling (${gb()} GB resident)`); process.exit(4); }
process.exit(u || blockBytes ? 1 : 0);
