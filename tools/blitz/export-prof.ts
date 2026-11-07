// WHERE THE PUPPET'S WHOLE EXPORT GOES (the audit's write-back exports the TypeScript game whole as every tick opens: PLAY.md): a mission played by the plain
// TypeScript tick to tick F, then the export timed whole, by root field and by table, and — for a write-back that would export only what a tool may have
// written — by the fields whose TypeScript name is on a list of names (one a line).
//   bun run --preload ./tools/blitz/expose.ts tools/blitz/export-prof.ts <mission>:<seed>:<F> [--names FILE] [--reps 5]
import '../qa/stand.ts';
import { readFileSync } from 'node:fs';
const argv = process.argv.slice(2), arg = (n: string, d: string) => { const i = argv.indexOf(n); return i < 0 ? d : argv[i + 1]; };
const [scn, seedS, FS] = (argv[0] ?? '').split(':'), F = +(FS ?? 1200), REPS = +arg('--reps', '5');
if (!scn) { console.log('export-prof.ts <mission>:<seed>:<F> [--names FILE] [--reps 5]'); process.exit(64); }
const NAMES = arg('--names', '') ? new Set(readFileSync(arg('--names', ''), 'utf8').split('\n').map(s => s.trim()).filter(Boolean)) : null;
const { Game } = await import('../../src/game/game.ts');
const { play } = await import('../qa/playthrough.ts');
const { exportGame, exportPart, fieldAt, bookOf, SCHEMA, parse } = await import('./export.ts');
const { knobRegistry } = await import('../../src/game/knobs.ts'); const reg = knobRegistry();
const say = (s = '') => process.stdout.write(s + '\n'), now = () => performance.now();
class Stop extends Error {}
const was = Game.prototype.update; let k = 0, G: any = null;
Game.prototype.update = function (this: any, ...a: any[]) { G = this; (was as any).apply(this, a); if (++k >= F) throw new Stop(); } as any;
const LOG = console.log; try { await play(scn, +(seedS ?? 1), 300, false, {}); } catch (e) { if (!(e instanceof Stop)) throw e; } console.log = LOG;
if (!G || k < F) { say(`export-prof: the play ended at tick ${k}, before ${F}`); }
const g = G, med = (f: () => unknown) => { const T: number[] = []; for (let i = 0; i < REPS; i++) { const t = now(); f(); T.push(now() - t); } return T.sort((a, b) => a - b)[REPS >> 1]; };
const whole = exportGame(g, reg); for (let i = 0; i < 3; i++) exportGame(g, reg);
const tWhole = med(() => exportGame(g, reg));
say(`${scn}:${seedS} at tick ${k}: the whole export ${(whole.length / 1048576).toFixed(2)} MB, ${tWhole.toFixed(1)} ms (median of ${REPS})`);
type Row = { what: string; ms: number; kb: number; n: number };
const rows: Row[] = [];
// ---- the root: the game's own fields, one by one
const root = SCHEMA.types[SCHEMA.root];
for (const f of root.fields) { let x: any; try { x = fieldAt(g, reg, [f.rust]); } catch { continue; } let n = 0; const ms = med(() => { n = exportPart(g, reg, [[x.ty, x.v]]).length; }); rows.push({ what: `g.${f.rust}`, ms, kb: n / 1024, n: 1 }); }
// ---- the tables
const bk: any = bookOf(g);
for (const t of SCHEMA.tables) { const L = bk.listOf(t.id) as any[]; if (!L.length) continue; const P = L.map(r => [t.ty, r] as [string, any]); let n = 0; const ms = med(() => { n = exportPart(g, reg, P).length; }); rows.push({ what: `table ${t.table} (${t.ty.replace(/^crate::/, '')})`, ms, kb: n / 1024, n: L.length }); }
rows.sort((a, b) => b.ms - a.ms);
const sum = rows.reduce((a, r) => a + r.ms, 0);
say(`by part (each exported alone; the sum ${sum.toFixed(1)} ms; a part reached from two is counted twice):`);
for (const r of rows.slice(0, 28)) say(`  ${r.ms.toFixed(2).padStart(7)} ms  ${r.kb.toFixed(0).padStart(6)} KB  ${String(r.n).padStart(6)} rows  ${r.what}`);
say(`  ${rows.slice(28).reduce((a, r) => a + r.ms, 0).toFixed(2).padStart(7)} ms  the other ${Math.max(0, rows.length - 28)} parts`);
// ---- the fields a tool may have written, by name: every struct field of a table's row type (and of the root) whose TypeScript name is on the list
if (NAMES) {
  const P: [string, any][] = []; let fields = 0; const hit = new Map<string, number>();
  const take = (T: any, o: any) => { if (!T || T.k !== 'struct' || !o) return; for (const f of T.fields) { if (!f.ts || !/^[A-Za-z_$][\w$]*$/.test(f.ts) || !NAMES.has(f.ts)) continue; let v: any; try { v = o[f.ts]; } catch { continue; } if (v === undefined) continue; P.push([f.ty, v]); fields++; hit.set(`${T.name}.${f.ts}`, (hit.get(`${T.name}.${f.ts}`) ?? 0) + 1); } };
  take(root, g);
  for (const t of SCHEMA.tables) for (const r of bk.listOf(t.id) as any[]) take(SCHEMA.types[t.ty], r);
  let n = 0, bad = 0; const one = () => { n = 0; bad = 0; for (const p of P) { try { n += exportPart(g, reg, [p]).length; } catch { bad++; } } };
  let all = 0; const tAll = (() => { try { return med(() => { all = exportPart(g, reg, P).length; }); } catch { return NaN; } })();
  if (Number.isNaN(tAll)) { const ms = med(one); say(`the fields on the list of names (${NAMES.size} names): ${fields} values of ${hit.size} fields, ${(n / 1024).toFixed(0)} KB, ${ms.toFixed(1)} ms exported one by one (${bad} would not export alone)`); }
  else say(`the fields on the list of names (${NAMES.size} names): ${fields} values of ${hit.size} fields, ${(all / 1024).toFixed(0)} KB, ${tAll.toFixed(1)} ms in one export (${(100 * tAll / tWhole).toFixed(0)}% of the whole export's time, ${(100 * all / whole.length).toFixed(0)}% of its bytes)`);
  // (the same without the three heaviest names' fields, to see what a list by hand would cost)
  const top = [...hit].sort((a, b) => b[1] - a[1]).slice(0, 12).map(([q, c]) => `${q} x${c}`).join(', '); say(`  the most rows: ${top}`);
  void parse; }
// ---- the whole apply, and the same without the parts that never move after a start (the meshes, the clips, the shadow meshes: no tick writes them)
{ const { applyGame } = await import('./import.ts'); const { classTable, valueTable } = await import('./classes.ts'); const hooks = { classes: classTable(), values: valueTable() };
  const still = (T: any, f: any) => T.name === 'MeshData' || T.name === 'Clip' || (T.mod === 'game' && T.name === 'Game' && f.rust === 'mesh_shadows');
  const only = new Set<string>(); for (const T of Object.values<any>(SCHEMA.types)) if (T.k === 'struct') for (const f of T.fields) if (!still(T, f)) only.add(`${T.mod}::${T.name}.${f.rust}`);
  const blob = exportGame(g, reg); for (let i = 0; i < 2; i++) applyGame(g, blob, reg, { hooks });
  const tA = med(() => applyGame(g, blob, reg, { hooks })); for (let i = 0; i < 2; i++) applyGame(g, blob, reg, { hooks, only });
  const tL = med(() => applyGame(g, blob, reg, { hooks, only }));
  say(`the whole apply (the game's own export applied onto itself): ${tA.toFixed(1)} ms; the same with the meshes, the clips and the shadow meshes read past: ${tL.toFixed(1)} ms`); }
process.exit(0);
