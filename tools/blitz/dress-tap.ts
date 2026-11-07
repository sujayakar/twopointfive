// THE DRESSING'S MID-TICK READS, MEASURED. The swap runs the picture's steps after the whole Rust step; in the TypeScript each runs at its place inside it. For each of the tick's own dressing steps:
// the state where the step stands against the state at the tick's end, field by field (a hash of each field's bytes over all its rows, so memory stays small). A field that differs and that the step
// reads (inventory: dress-step.json) is a value the step would now see differently.
//   game.ts must carry `__tap(line)` before each step: done on a scratch tree, never in the repo.
//   bun run --preload ./tools/blitz/expose.ts tools/blitz/dress-tap.ts <scn>:<seed>:<frames> <dress-step.json> <out.json>
import { writeFileSync, readFileSync } from 'node:fs';
import { exportGame, parse, SCHEMA } from './export';
const [moment, stepsFile, outFile] = process.argv.slice(2); const [scn, seed, frames] = moment.split(':');
const { knobRegistry } = await import('../../src/game/knobs.ts'); const reg = knobRegistry();
const { Game } = await import('../../src/game/game.ts'); const { play } = await import('../qa/playthrough.ts'); class Stop extends Error {}
const STEP = +(process.env.TAP_STEP ?? 137), WARM = 300; let n = 0, on = false, g: any = null; let taps: { line: number; h: Map<string, number>; rows: Record<string, number> }[] = [];
const P = new Map<string, any>(); const pOf = (t: string) => { let x = P.get(t); if (!x) P.set(t, x = parse(t)); return x; };
let rowsNow: Record<string, number> = {};
function hashes(b: Uint8Array, limit?: Record<string, number>): Map<string, number> { const dv = new DataView(b.buffer, b.byteOffset, b.byteLength); let p = 0, cur = 'g'; const H = new Map<string, number>();
  const mix = (x: number) => { H.set(cur, (((H.get(cur) ?? 17) * 31) ^ x) >>> 0); };
  const rd = (n: any): void => { switch (n.k) {
    case 'f64': mix(dv.getUint32(p, true) ^ (dv.getUint32(p + 4, true) * 7)); p += 8; return; case 'f32': mix(dv.getUint32(p, true)); p += 4; return;
    case 'bool': case 'u8': case 'i8': mix(b[p]); p += 1; return; case 'u16': case 'i16': mix(dv.getUint16(p, true)); p += 2; return; case 'u32': case 'i32': case 'usize': case 'id': mix(dv.getUint32(p, true)); p += 4; return; case 'u64': case 'i64': mix(dv.getUint32(p, true)); p += 8; return;
    case 'str': { const L = dv.getUint32(p, true); p += 4; for (let k = 0; k < L; k++) mix(b[p + k]); p += L; return; } case 'unit': return;
    case 'opt': { const t = b[p++]; mix(t); if (t) rd(n.of); return; }
    case 'vec': { const L = dv.getUint32(p, true); p += 4; mix(L); for (let k = 0; k < L; k++) rd(n.of); return; }
    case 'arr': for (let k = 0; k < n.n; k++) rd(n.of); return; case 'tup': for (const x of n.of) rd(x); return;
    case 'map': { const L = dv.getUint32(p, true); p += 4; mix(L); for (let k = 0; k < L; k++) { rd(n.K); rd(n.V); } return; } case 'set': { const L = dv.getUint32(p, true); p += 4; mix(L); for (let k = 0; k < L; k++) rd(n.K); return; }
    case 'type': { const T = SCHEMA.types[n.path]; if (T.k === 'struct') { for (const f of T.fields) { const c0 = cur; cur = `${T.mod}::${T.name}.${f.rust}`; rd(pOf(f.ty)); cur = c0; } return; } if (T.k === 'enum') { mix(dv.getUint16(p, true)); p += 2; return; } const a = dv.getUint16(p, true); p += 2; mix(a); if (T.k === 'data') { const t = T.vars[a].ty; if (t) rd(pOf(t)); return; } const t = T.arms[a].match(/\((.*)\)/)?.[1]; if (t) rd(pOf(t)); return; } }
    throw new Error('no reader ' + n.k); };
  rd({ k: 'type', path: SCHEMA.root }); for (const t of SCHEMA.tables) { const L = dv.getUint32(p, true); p += 4; rowsNow[t.id] = L; cur = `table ${t.id}`; const lim = limit ? Math.min(limit[t.id] ?? L, L) : L; mix(lim); const N = pOf(t.ty); for (let k = 0; k < lim; k++) rd(N); if (L > lim) { const H0 = new Map(H); for (let k = lim; k < L; k++) rd(N); H.clear(); for (const [q, v] of H0) H.set(q, v); } } return H; }
const steps: any[] = JSON.parse(readFileSync(stepsFile, 'utf8')).steps.filter((s: any) => s.file === 'src/game/game.ts' && ['Game.stepSeats', 'Game.stepFxAndDynBoxes', 'Game.drawCharacters'].includes(s.in));
const tally = new Map<number, Map<string, number>>(); const seen = new Map<number, number>(); let samples = 0;
(globalThis as any).__tap = (line: number) => { if (!on) return; try { const b = exportGame(g, reg); rowsNow = {}; const h = hashes(b); taps.push({ line, h, rows: rowsNow }); } catch (e) { console.error('tap', line, String(e).slice(0, 300)); } };
const was = Game.prototype.stepSeats;
Game.prototype.stepSeats = function (this: any, dt: number, cmds: any) { g = this; n++; on = n > WARM && n % STEP === 0; taps = [];
  if (on) exportGame(g, reg);
  (was as any).call(this, dt, cmds); if (!on) return; on = false; samples++;
  const endB = exportGame(g, reg);
  for (const t of taps) { const E = hashes(endB, t.rows); seen.set(t.line, (seen.get(t.line) ?? 0) + 1); let T = tally.get(t.line); if (!T) tally.set(t.line, T = new Map()); const keys = new Set([...t.h.keys(), ...E.keys()]); for (const k of keys) if (t.h.get(k) !== E.get(k)) T.set(k, (T.get(k) ?? 0) + 1); }
  console.error(`sample ${samples} at tick ${n}`);
  if (n >= +frames) throw new Stop(); } as any;
const log0 = console.log; console.log = () => {}; try { await play(scn, +seed, 300, false, {}); } catch (e) { if (!(e instanceof Stop)) throw e; } console.log = log0;
const out: any[] = []; for (const s of steps) { const T = tally.get(s.line); if (!T) { out.push({ line: s.line, row: s.row, callee: s.callee, in: s.in, tapped: false }); continue; } const reads = new Set(s.readsRust ?? []);
  const changed = [...T].map(([q, k]) => ({ q, samples: k, read: reads.has(q) })); out.push({ line: s.line, row: s.row, callee: s.callee, in: s.in, tapped: seen.get(s.line), changedAfter: changed.length, readThenChanged: changed.filter(c => c.read).sort((a, b) => b.samples - a.samples), other: changed.filter(c => !c.read).map(c => c.q).slice(0, 200) }); }
writeFileSync(outFile, JSON.stringify({ moment, samples, ticks: n, steps: out }, null, 0)); console.log(`${moment}: ${samples} samples over ${n} ticks`); for (const s of out) if (s.tapped) console.log(`${s.in.split('.')[1].slice(0, 8)}:${s.line} ${s.row ?? s.callee ?? ''}: ${s.changedAfter} fields change after it, ${s.readThenChanged.length} of them it reads: ${s.readThenChanged.slice(0, 10).map((c: any) => `${c.q}×${c.samples}`).join(' ')}`);
process.exit(0);
