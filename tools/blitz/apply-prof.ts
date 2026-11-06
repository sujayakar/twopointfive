// WHERE AN APPLY'S TIME GOES, a tick at a time. Role a plays to frame F and exports every frame to F+K. Role b plays to F, exports (the book) and applies each in turn, several ways.
//   bun run --preload ./tools/blitz/expose.ts tools/blitz/apply-prof.ts <scn>:<seed>:<F>:<K> --role a|b --dir d --list puppet-fields.json
import { writeFileSync, readFileSync, mkdirSync } from 'node:fs';
import { exportGame, parse, SCHEMA } from './export';
import { applyGame } from './import';
import { classTable, valueTable } from './classes';
import { decode } from './paths';
import { PUT_BACKS } from '../../src/math/side';
const argv = process.argv.slice(2); const [scn, seed, F, K] = argv[0].split(':'); const arg = (n: string, d: string) => { const i = argv.indexOf(n); return i < 0 ? d : argv[i + 1]; };
const dir = arg('--dir', '/tmp/prof'), role = arg('--role', 'a'); mkdirSync(dir, { recursive: true }); const say = (x: string) => { process.stdout.write(x + '\n'); };
const { knobRegistry } = await import('../../src/game/knobs.ts'); const reg = knobRegistry();
const { Game } = await import('../../src/game/game.ts'); const { play } = await import('../qa/playthrough.ts'); class Stop extends Error {}
const frames: number[] = []; for (let i = 0; i <= +K; i++) frames.push(+F + i);
function run(fr: number[], at: (g: any, i: number) => void): Promise<void> { const was = Game.prototype.update; let n = 0, k = 0; Game.prototype.update = function (this: any, ...a: unknown[]) { n++; if (n === fr[k]) { try { at(this, k); } catch (e) { if (!(e instanceof Stop)) { process.stderr.write('at: ' + String((e as any)?.stack ?? e) + '\n'); process.exit(2); } } k++; if (k === fr.length) throw new Stop(); } return (was as any).apply(this, a); } as any; console.log = () => {}; return play(scn, +seed, 300, false, {}).catch((e: unknown) => { if (!(e instanceof Stop)) throw e; }) as Promise<void>; }
if (role === 'a') { await run(frames, (g, i) => { writeFileSync(`${dir}/f${i}.bin`, exportGame(g, reg)); }); process.exit(0); }
const P = JSON.parse(readFileSync(arg('--list', ''), 'utf8')); const only = new Set<string>(P.everyTick); const hooks = { classes: classTable(), values: valueTable() };
const med = (L: number[]) => { const s = [...L].sort((a, b) => a - b); return s[s.length >> 1]; };
await run([+F], g => { exportGame(g, reg); const B: Uint8Array[] = []; for (let i = 1; i <= +K; i++) B.push(new Uint8Array(readFileSync(`${dir}/f${i}.bin`)));
  // how much moves between two ticks
  const leaves = (b: Uint8Array) => decode(b).filter(r => r.q && only.has(r.q)); const L0 = leaves(new Uint8Array(readFileSync(`${dir}/f0.bin`))), L1 = leaves(B[0]); const M0 = new Map(L0.map(r => [r.path, r.bits])); let moved = 0; const by = new Map<string, number>(); for (const r of L1) if (M0.get(r.path) !== r.bits) { moved++; by.set(r.q!, (by.get(r.q!) ?? 0) + 1); } say(`one tick: ${L1.length} values on the every-tick list, ${moved} changed`); say('  most changed: ' + [...by].sort((a, b) => b[1] - a[1]).slice(0, 24).map(([q, n]) => `${q} ${n}`).join(', '));
  // the bare walk
  { const T: number[] = []; for (const b of B) { const dv = new DataView(b.buffer, b.byteOffset, b.byteLength); const t0 = performance.now(); let s = 0; for (let i = 0; i + 8 <= b.length; i += 4096) s += dv.getUint32(i, true); T.push(performance.now() - t0 + (s & 0)); } }
  const A: { ms: number; ph: Record<string, number>; c: Record<string, number> }[] = []; for (let round = 0; round < 3; round++) for (const b of B) { const R = applyGame(g, b, reg, { only, hooks }); A.push({ ms: R.ms, ph: R.phases, c: R.counts }); }
  const tail = A.slice(-B.length * 2); const keys = Object.keys(tail[0].ph); say(`apply, every-tick list, ${B.length} ticks three times, last two rounds' medians: total ${med(tail.map(a => a.ms)).toFixed(2)} ms; ` + keys.map(k => `${k} ${med(tail.map(a => a.ph[k])).toFixed(2)}`).join(', ') + `; reads ${med(tail.map(a => a.c.read))}, assigns ${med(tail.map(a => a.c.assign))}`);
  const W: { ms: number; ph: Record<string, number> }[] = []; for (let round = 0; round < 2; round++) for (const b of B) { const R = applyGame(g, b, reg, { hooks }); W.push({ ms: R.ms, ph: R.phases }); } say(`apply, whole state: ${med(W.slice(-B.length).map(a => a.ms)).toFixed(2)} ms`);
  // the mirror: a query after a put-back, against one without
  const col = g.col; const q = () => { const t0 = performance.now(); for (let i = 0; i < 200; i++) col.segmentBlocked([i % 40, 1, 2], [i % 40 + 3, 1, 9]); return performance.now() - t0; }; const q0 = q(), q1 = q(); PUT_BACKS.n++; const q2 = q(), q3 = q(); say(`200 ray questions: first ${q0.toFixed(2)} ms, again ${q1.toFixed(2)}, after a put-back ${q2.toFixed(2)}, again ${q3.toFixed(2)}`);
  throw new Stop(); }).catch(() => {}); process.exit(0);
