// HOW MUCH OF THE WHOLE STATE MOVES IN A TICK (what an every-tick export of EVERY carried field, changed rows only, would carry; the filtered export
// carries the listed fields of the changed rows: PUPPET.md). A mission played on the Rust game through the headless host; from tick A to tick B the Rust
// game's whole state is taken after every step and held to the one before, part by part: each of the game's own fields, each row of each table.
//   bun run --preload ./tools/blitz/expose.ts tools/blitz/delta-prof.ts <mission>:<seed> [--from 600] [--to 900]
import '../qa/stand.ts';
import { RustHost } from './rust-host.ts';
const argv = process.argv.slice(2), arg = (n: string, d: string) => { const i = argv.indexOf(n); return i < 0 ? d : argv[i + 1]; };
const [scn, seedS] = (argv[0] ?? '').split(':'), A = +arg('--from', '600'), B = +arg('--to', '900');
if (!scn) { console.log('delta-prof.ts <mission>:<seed> [--from 600] [--to 900]'); process.exit(64); }
const F = await RustHost.ready(), X = F.X;
const { Game } = await import('../../src/game/game.ts');
const { play } = await import('../qa/playthrough.ts');
const { SCHEMA, parse } = await import('./export.ts');
const { skipIn, fieldsOf, buf } = await import('./audit.ts');
const say = (s = '') => process.stdout.write(s + '\n');
type Part = { name: string; s: number; e: number };
/** where each part of a whole state stands: the game's own fields (and under `m`, each module's), then every row of every table */
function parts(b: Uint8Array): Part[] { const dv = new DataView(b.buffer, b.byteOffset, b.byteLength), out: Part[] = []; let p = 0;
  const T = SCHEMA.types[SCHEMA.root];
  for (const f of fieldsOf(T)) { const sub = f.N.k === 'type' ? SCHEMA.types[f.N.path] : null;
    if (sub?.k === 'struct' && f.dot === '.m') { for (const h of fieldsOf(sub)) { const e = skipIn(b, dv, p, h.N); out.push({ name: `g.m${h.dot}`, s: p, e }); p = e; } continue; }
    const e = skipIn(b, dv, p, f.N); out.push({ name: `g${f.dot}`, s: p, e }); p = e; }
  for (const t of SCHEMA.tables) { const L = dv.getUint32(p, true), N = parse(t.ty); out.push({ name: `${t.table}.len`, s: p, e: p + 4 }); p += 4; for (let i = 0; i < L; i++) { const e = skipIn(b, dv, p, N); out.push({ name: `${t.table}[${i}]`, s: p, e }); p = e; } }
  out.push({ name: 'tail', s: p, e: b.length }); return out; }
class Stop extends Error {}
let H: RustHost | null = null, k = 0, prev: Uint8Array | null = null, prevP: Part[] | null = null, n = 0;
const by = new Map<string, { bytes: number; rows: number; ticks: number }>(); let moved = 0, whole = 0, filtered0 = 0, tWalk = 0;
const was = Game.prototype.update;
Game.prototype.update = function (this: any, ...a: any[]) { k++;
  if (!H) H = RustHost.attach(this, { quiet: true, die: 20942, afterStep: Hh => { if (k < A) return; if (k === A) filtered0 = Hh.stepBytes;
    X.face_export(Hh.h); const cur = F.outView().slice(), t = performance.now(), P = parts(cur); tWalk += performance.now() - t; whole = cur.length;
    if (prev && prevP) { n++; const old = new Map(prevP.map(x => [x.name, x])); const seen = new Set<string>();
      for (const x of P) { const o = old.get(x.name); if (o && o.e - o.s === x.e - x.s && buf(prev.subarray(o.s, o.e)).equals(buf(cur.subarray(x.s, x.e)))) continue;
        const key = x.name.replace(/\[\d+\]$/, '[row]'), e = by.get(key) ?? { bytes: 0, rows: 0, ticks: 0 }; e.bytes += x.e - x.s; e.rows++; if (!seen.has(key)) { seen.add(key); e.ticks++; } by.set(key, e); moved += x.e - x.s; } }
    prev = cur; prevP = P; } });
  (was as any).apply(this, a); if (k >= B) throw new Stop(); } as any;
const LOG = console.log; try { await play(scn, +(seedS ?? 1), 300, false, {}); } catch (e) { if (!(e instanceof Stop)) throw e; } console.log = LOG;
if (!H || !n) { say(`delta-prof: the play ended at tick ${k}, before ${A}`); process.exit(1); }
const Hh = H as RustHost, filt = (Hh.stepBytes - filtered0) / Math.max(1, Hh.steps - A);
say(`${scn}:${seedS}, ticks ${A} to ${k} (${n} pairs of states): the whole state ${(whole / 1048576).toFixed(2)} MB; the parts that differ from the tick before: ${(moved / n / 1024).toFixed(0)} KB a tick (each part whole: a field of the game's own, a module's field, a table's row); the filtered export over the same ticks: ${(filt / 1024).toFixed(0)} KB a tick; the walk that finds a state's parts ${(tWalk / (n + 1)).toFixed(1)} ms`);
for (const [q, e] of [...by].sort((x, y) => y[1].bytes - x[1].bytes).slice(0, 30)) say(`  ${(e.bytes / n / 1024).toFixed(1).padStart(8)} KB a tick  ${(e.rows / n).toFixed(1).padStart(7)} parts a tick  on ${e.ticks} of ${n} ticks  ${q}`);
say(`  ${([...by].sort((x, y) => y[1].bytes - x[1].bytes).slice(30).reduce((a, [, e]) => a + e.bytes, 0) / n / 1024).toFixed(1).padStart(8)} KB a tick  the other ${Math.max(0, by.size - 30)} kinds of part`);
process.exit(0);
