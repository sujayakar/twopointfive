// SHOW ME THIS PATH ON BOTH SIDES AT TICK k:
//   bun run tools/blitz/peek.ts <run dir> <tick> '<regex of paths>' [--most N=40] [--differ]
// For every path that matches: what the TypeScript game held as the tick opened (c<k>.bin), what it held as the tick returned (a<k>.bin), and what the Rust game
// held then (n<k>.bin). A `!` marks a path whose two sides differ after the tick. --differ shows only those. The run dir is the one run.sh names; it keeps the
// last tick it compared, and every tick with --keep. One state is decoded at a time.
import { readFileSync, existsSync } from 'node:fs';
import { decode } from './paths';
const argv = process.argv.slice(2); const opt = (n: string, d: string) => { const i = argv.indexOf(n); return i < 0 ? d : argv.splice(i, 2)[1]; };
const flag = (n: string) => { const i = argv.indexOf(n); if (i < 0) return false; argv.splice(i, 1); return true; };
const MOST = +opt('--most', '40'), DIFFER = flag('--differ'); const [dir, k, re] = argv;
if (!dir || !k || !re) { console.log("usage: bun run tools/blitz/peek.ts <run dir> <tick> '<regex of paths>' [--most N] [--differ]"); process.exit(64); }
const R = new RegExp(re);
const take = (f: string): Map<string, string> | null => { const p = `${dir}/${f}${k}.bin`; if (!existsSync(p)) return null; const M = new Map<string, string>(); for (const r of decode(new Uint8Array(readFileSync(p)))) if (R.test(r.path)) M.set(r.path, r.bits); return M; };
const C = take('c'), A = take('a'), N = take('n');
if (!A && !N) { console.log(`no a${k}.bin or n${k}.bin in ${dir} (run.sh keeps the last tick it compared; --keep keeps them all)`); process.exit(3); }
const paths = [...new Set([...(A?.keys() ?? []), ...(N?.keys() ?? [])])];
const short = (s: string | undefined) => s === undefined ? '(absent)' : s.length > 60 ? s.slice(0, 60) + '…' : s;
let shown = 0, differ = 0;
for (const p of paths) { const a = A?.get(p), n = N?.get(p), d = A && N && a !== n; if (d) differ++; if (DIFFER && !d) continue; if (shown++ >= MOST) continue;
  console.log(`${d ? '!' : ' '} ${p}\n      opens ${short(C?.get(p))}   ts ${short(a)}   rust ${short(n)}`); }
console.log(`tick ${k}: ${paths.length} paths match, ${differ} differ after the tick${shown > MOST ? `; ${shown - MOST} more not shown (--most)` : ''}${C ? '' : '; no c' + k + '.bin (the state as the tick opens)'}`);
