// THE SUN'S REACH MASK (render/reach.ts reachMaskForDirectional; game.ts buildDirReach; the direct estimator's candidate domain for the hour's sun and the moon).
// A cell under a roof reads "the sun reaches" when one of its looks has a clear path toward the sun. A look has to stand in open air: the corner looks sit 9 cm
// inside the cell, a wall on a cell's line is 12 cm to either side of it, and a look that started inside the wall left through its outer face, so every cell along
// the inside of an outer wall read "reaches". There the hour's sun, thousands of times a room's bulb, was in the fresh draw of every pixel, and a man standing at
// the wall (where men stack) wore a white speckle for every pick of the bulb.
// Held here on every level file with a roof, at every hour with a sun:
//   1. a cell the mask calls lit HAS an open-air point that sees the sun (a fine grid of the cell, asked of every shadow-casting box, glass passing);
//   2. little that is true is lost. A cell wrongly DARK has no sun in its draw, so the patch a window throws is cut off at the cell's line: on five levels, at every
//      hour, every dark cell is asked on a grid of its own (8 × 8 × four heights, none of them the mask's looks) and may hold a sliver of sun, never more than a
//      twentieth of its open air (the looks are a sample: 37 a cell × five heights; the five × three there were missed a patch a fifth of a cell wide and a band under a window's head);
//   3. the pass at midday: the cells along the inside of the house's outer walls are dark, and a cell a window lets the sun into is lit.
// No GPU: the picture is never SEEN here.
//   bun run tools/qa/sunreach-check.ts                    exit 0 = every line passed, 1 = one failed (both stands: TPS_PROFILE=shipped in front)
//   SUNREACH_BREAK=1 bun run tools/qa/sunreach-check.ts   the looks as they were (five a cell, standing where they fall): must FAIL
import { readFileSync, readdirSync } from 'node:fs';
import { standUp, ROOT } from './headless.ts';
const { parseLevelData } = await import(`${ROOT}/src/scene/levelData.ts`);
const { loadLevel } = await import(`${ROOT}/src/scene/levelLoad.ts`);
const { applyHour, hourSun, HOUR_IDS } = await import(`${ROOT}/src/game/hours.ts`);
const { reachLevel, REACH_OPTS, DIR_SUBS, DIR_HEIGHTS } = await import(`${ROOT}/src/render/reach.ts`);
const { BoxFlag, rayBox } = await import(`${ROOT}/src/scene/boxes.ts`);
const { WORLD } = await import(`${ROOT}/src/scene/world.ts`);
const BRK = process.env.SUNREACH_BREAK === '1'; let red = false;
if (BRK) REACH_OPTS.solidLooks = true;
const check = (ok: boolean, what: string, detail = '') => { red ||= !ok; console.log(`- ${ok ? 'PASS' : 'FAIL'} — ${what}${detail ? `\n    ${detail}` : ''}`); };
console.log(`\n=== sunreach: which cells under a roof the sun can reach${BRK ? ' (the looks as they were: must fail)' : ''} ===`);
const W_ = WORLD.gridW, H_ = WORLD.gridH;
const only = process.argv.slice(2).filter(a => !a.startsWith('-'));
const files = readdirSync(`${ROOT}/levels`).filter(f => f.endsWith('.json') && !f.endsWith('.lint.json')).map(f => f.slice(0, -5)).filter(f => !only.length || only.includes(f)).sort();
let levels = 0, hours = 0, lit = 0, dark = 0, hollow = 0, lost = 0, slivers = 0, asked = 0, worst = 0; const hollowAt: string[] = [], lostAt: string[] = [], threw: string[] = [];
const STRICT = ['pass', 'office', 'compound', 'barracks', 'registry'];   // the levels whose dark cells are asked on the finer grid (a minute; all 58 would be ten)
for (const name of files) {
  const l = console.log, w = console.warn; console.log = console.warn = () => { };
  let g: any; try { const Wd: any = await standUp({ level: loadLevel(parseLevelData(readFileSync(`${ROOT}/levels/${name}.json`, 'utf8'))), seed: 1 }); for (let i = 0; i < 4; i++) Wd.step(1 / 60); g = Wd.game; } catch (e) { g = null; threw.push(`${name}: ${String(e).slice(0, 80)}`); }
  console.log = l; console.warn = w;
  if (!g || !g.reachInterior || !(g.reachInterior as Uint8Array).some((v: number) => v)) continue;   // no roof on this level: every cell is open ground
  levels++;
  const inr = g.reachInterior as Uint8Array;
  const casters = (g.engine.world.statics as any[]).filter(b => !(b.flags & (BoxFlag.NoShadow | BoxFlag.Glass | BoxFlag.Dynamic)));
  const plain = casters.filter(b => !b.rot && b.round !== true && !(b.cut && b.cut.length));   // (upright boxes, turned or not: what game.ts buildDirReach calls solid)
  const inSolid = (p: number[]) => plain.some(b => { if (Math.abs(p[1] - b.c[1]) >= b.h[1]) return false; const c = Math.cos(b.yaw ?? 0), s = Math.sin(b.yaw ?? 0), dx = p[0] - b.c[0], dz = p[2] - b.c[2]; return Math.abs(c * dx - s * dz) < b.h[0] && Math.abs(s * dx + c * dz) < b.h[2]; });
  for (const id of HOUR_IDS) {
    if (id === 'night') continue;
    applyHour(g, id); const sun = hourSun(g); if (!sun || !sun.reachDir) continue; hours++;
    const m = sun.reachDir as Uint32Array, d = sun.dir, n = Math.hypot(d[0], d[1], d[2]) || 1, to = [-d[0] / n, -d[1] / n, -d[2] / n];
    // (the mask is the union over the sun's disc: its middle and four points of its rim, hours.ts sunReach. A point that sees any of the five sees the sun.)
    const u0 = Math.abs(to[1]) < 0.9 ? [0, 1, 0] : [1, 0, 0], u = [to[1] * u0[2] - to[2] * u0[1], to[2] * u0[0] - to[0] * u0[2], to[0] * u0[1] - to[1] * u0[0]], ul = Math.hypot(u[0], u[1], u[2]);
    u[0] /= ul; u[1] /= ul; u[2] /= ul; const v = [to[1] * u[2] - to[2] * u[1], to[2] * u[0] - to[0] * u[2], to[0] * u[1] - to[1] * u[0]], k = sun.radius > 0.05 ? Math.tan(sun.radius) : 0;
    const dirs = [to]; if (k > 0) for (let i = 0; i < 4; i++) { const a = i * Math.PI / 2, q = [to[0] + (u[0] * Math.cos(a) + v[0] * Math.sin(a)) * k, to[1] + (u[1] * Math.cos(a) + v[1] * Math.sin(a)) * k, to[2] + (u[2] * Math.cos(a) + v[2] * Math.sin(a)) * k], ql = Math.hypot(q[0], q[1], q[2]); dirs.push([q[0] / ql, q[1] / ql, q[2] / ql]); }
    const sees = (p: number[]) => dirs.some(t => { const dist = t[1] > 0.05 ? Math.min(60, (WORLD.ceilingY + 1.5 - p[1]) / t[1]) : 60; return !casters.some(b => rayBox(p as any, t as any, b, dist)); });
    for (let cz = 0; cz < H_; cz++) for (let cx = 0; cx < W_; cx++) {
      const ci = cz * W_ + cx; if (!inr[ci]) continue;
      const lv = reachLevel(m, ci); if (lv) lit++; else dark++;
      const x0 = WORLD.min[0] + cx * WORLD.gridCell, z0 = WORLD.min[2] + cz * WORLD.gridCell;
      if (lv) {   // 1. some open-air point of the cell sees the sun: the five looks' own heights and two more, on a 3 cm grid
        let any = false;
        out: for (const y of [0.05, 0.4, 0.8, 1.25, 1.7]) for (let j = 0; j < 32; j++) for (let i = 0; i < 32; i++) { const p = [x0 + (i + 0.5) / 32, y, z0 + (j + 0.5) / 32]; if (!inSolid(p) && sees(p)) { any = true; break out; } }
        if (!any) for (const [i, j] of DIR_SUBS as [number, number][]) for (const y of DIR_HEIGHTS as number[]) { const p = [x0 + (i + 0.5) / 16, y, z0 + (j + 0.5) / 16]; if (!inSolid(p) && sees(p)) { any = true; break; } }   // (… and the mask's own looks where they stand in open air: a sliver through a door's gap can be thinner than the grid)
        if (!any && hollow < 4) fine: for (const y of DIR_HEIGHTS as number[]) for (let j = 0; j < 128; j++) for (let i = 0; i < 128; i++) { const p = [x0 + (i + 0.5) / 128, y, z0 + (j + 0.5) / 128]; if (!inSolid(p) && sees(p)) { any = true; break fine; } }   // (… and a look moved out of a solid stands 2 cm off its face: a sliver a centimetre wide, over a wall's top and under the roof's edge, is seen by that look alone. Rare: a few cells a run, and not asked once four cells have failed)
        if (!any) { hollow++; if (hollowAt.length < 8) hollowAt.push(`${name} ${id} cell ${cx},${cz}`); }
      } else if (STRICT.includes(name)) {    // 2. a dark cell holds a sliver of sun at most (its own grid, not the mask's looks)
        let open = 0, sun = 0;
        for (const y of [0.03, 0.65, 1.1, 1.5]) for (let j = 0; j < 8; j++) for (let i = 0; i < 8; i++) { const p = [x0 + (i + 0.5) / 8, y, z0 + (j + 0.5) / 8]; if (inSolid(p)) continue; open++; if (sees(p)) sun++; }
        asked++; if (sun) slivers++; const share = open ? sun / open : 0; if (share > worst) worst = share;
        if (share > 0.05) { lost++; if (lostAt.length < 8) lostAt.push(`${name} ${id} cell ${cx},${cz} (${sun} of ${open})`); }
      }
    }
    if (name === 'pass' && id === 'midday') {
      const row = (cz: number, a: number, b: number) => { let s = ''; for (let x = a; x <= b; x++) s += reachLevel(m, cz * W_ + x); return s; };
      const west = [5, 6, 7, 8, 9].map(z => reachLevel(m, z * W_ + 12)).join('');
      check(west === '00000', 'the pass at midday: the cells along the inside of the house\'s west wall are dark to the sun', `column 12, rows 5 to 9: ${west} (0 = the sun cannot reach)`);
      check(row(10, 22, 23) === '00' && row(8, 13, 33) === '0'.repeat(21), 'the pass at midday: the wall between two windows and the middle of every room are dark to the sun', `row 10, cells 22 and 23: ${row(10, 22, 23)} · row 8, cells 13 to 33: ${row(8, 13, 33)}`);
      check(reachLevel(m, 10 * W_ + 25) === 3 && reachLevel(m, 10 * W_ + 14) === 3, 'the pass at midday: a cell a window lets the sun into is lit', `row 10: ${row(10, 11, 34)}`);
    }
  }
  applyHour(g, 'night');
}
check(threw.length === 0, 'every level stood up', threw.join(' · '));
check(levels >= (only.length ? 1 : 5) && hours >= levels * 3, 'the levels with a roof were read at every hour with a sun', `${levels} levels, ${hours} level hours, ${lit} cells under a roof lit and ${dark} dark`);
check(hollow === 0, 'a cell the mask calls lit has an open-air point that sees the sun', hollow ? `${hollow} cells lit with no such point: ${hollowAt.join(' · ')}` : `${lit} lit cells, each with one`);
check(lost === 0 && (asked > 0 || only.length > 0), 'a dark cell holds a sliver of sun at most: under a twentieth of its open air', lost ? `${lost} cells: ${lostAt.join(' · ')}` : `${asked} dark cells asked on ${STRICT.filter(n => files.includes(n)).join(', ')}: ${slivers} hold a sliver, the widest ${(worst * 100).toFixed(1)} % of its cell`);
console.log(red ? '\n=== sunreach: FAILED ===' : '\n=== sunreach: ALL PASS ===');
process.exit(red ? 1 : 0);
