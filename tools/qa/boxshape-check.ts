// ONE LAYOUT FOR EVERY BOX (scene/boxes.ts boxOf): how many hidden classes do the boxes a tick reads have? Every box is to be made with every field of the interface in
// place, in one order, so a read of a box in a hot loop (the line tests, the rays, the pack, the fog's list) is one field load on every engine. A literal somewhere, or a
// field set that boxOf does not lay down, forks the layout again and the loops fall back to the slow lookup — nothing breaks, the tick just costs more. This counts.
//   bun run tools/qa/boxshape-check.ts [--runs compound-four:1,pass-four:1]      under bun (JavaScriptCore: the structure id out of bun:jsc describe)
//   node --no-warnings --allow-natives-syntax --import ./tools/qa/framecost-v8.mjs tools/qa/boxshape-check.ts     under node (V8: %HaveSameMap); V8_ENTRY = this file
//   BOXSHAPE_BREAK=1    the red path: one box a tick is made as a bare literal — the check must FAIL
// Looked at once a second of each scripted run: the statics the line tests walk (g.col.boxes), the door leaves, and the tick's dynamic list (g.dynBoxes).
// EXIT 0 = one layout for each of the three (a copy is made by copyBox: a spread would be a second one) · 1 = more · 2 = unknown flag
export {};
process.env.TPS_PROFILE ??= 'shipped'; process.env.TPS_CPUSENSE ??= '1';
const argv = process.argv.slice(2); const flag = (n: string) => { const i = argv.indexOf(n); return i >= 0 ? argv[i + 1] : undefined; };
for (const a of argv) if (a.startsWith('--') && a !== '--runs') { console.error(`unknown flag ${a} — --runs`); process.exit(2); }
const runs = (flag('--runs') ?? 'compound-four:1,pass-four:1').split(',');
const isBun = !!(globalThis as { Bun?: unknown }).Bun;
let shapeOf: (o: object) => string;
if (isBun) { const { describe } = await import('bun:jsc' as string) as { describe: (o: unknown) => string }; shapeOf = o => /Structure (0x[0-9a-f]+)/i.exec(describe(o))?.[1] ?? describe(o).slice(0, 80); }
else { let same: (a: unknown, b: unknown) => boolean;
  try { same = new Function('a', 'b', 'return %HaveSameMap(a, b)') as typeof same; } catch { console.error('under node this needs --allow-natives-syntax'); process.exit(2); }
  const reps: object[] = []; shapeOf = o => { let i = reps.findIndex(r => same!(r, o)); if (i < 0) { i = reps.length; reps.push(o); } return `map${i}`; }; }
const { Game } = await import('../../src/game/game.ts');
const PT = await import('./playthrough.ts');
const BREAK = process.env.BOXSHAPE_BREAK === '1';
interface Seen { n: number; keys: string }
const seen: Record<string, Map<string, Seen>> = { statics: new Map(), leaves: new Map(), dynamic: new Map() };
const look = (kind: string, list: object[]) => { const M = seen[kind]; for (const b of list) { const s = shapeOf(b); const e = M.get(s); if (e) e.n++; else M.set(s, { n: 1, keys: Object.keys(b).join(',') }); } };
let ticks = 0; const update = Game.prototype.update;
Game.prototype.update = function (this: any, ...a: unknown[]) { const r = (update as (...x: unknown[]) => unknown).apply(this, a);
  if (BREAK && ticks % 60 === 30) this.dynBoxes.push({ yaw: 0, albedo: [0, 0, 0], emissive: [0, 0, 0], flags: 0, owner: 0, cutHeight: 1e6, c: [0, -50, 0], h: [0.01, 0.01, 0.01] });
  if (ticks++ % 60 === 30) { look('statics', this.col.boxes); look('leaves', this.col.dynamicBoxes); look('dynamic', this.dynBoxes); } return r; } as typeof update;
for (const run of runs) { const [scn, seed] = run.split(':'); await (PT as any).play(scn, Number(seed), 300, false, {}); }
let bad = false; const lines: string[] = [];
for (const [kind, max] of [['statics', 1], ['leaves', 1], ['dynamic', 1]] as const) { const M = seen[kind]; const ok = M.size <= max; if (!ok) bad = true;
  lines.push(`- ${ok ? 'PASS' : 'FAIL'} — ${kind}: ${M.size} layout${M.size === 1 ? '' : 's'} (at most ${max})`);
  if (!ok || M.size > 1) for (const [, e] of [...M].sort((x, y) => y[1].n - x[1].n).slice(0, 12)) lines.push(`      ${String(e.n).padStart(8)} × ${e.keys}`); }
console.log(`box layouts · ${isBun ? 'bun (JavaScriptCore)' : 'node (V8)'} · ${runs.join(', ')} · ${ticks} ticks, looked at once a second\n${lines.join('\n')}`);
process.exit(bad ? 1 : 0);
