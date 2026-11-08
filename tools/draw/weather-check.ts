// WHAT THE NEW PAGE DRAWS OF THE AIR, THE WIND AND THE FIRES IS WHAT THE COPY OF THE GAME DREW. One Rust game under the headless host; on one tick both sides answer, and the numbers
// are compared: the fires' records, the wind's lists, the air's map. It goes when the TypeScript copy does: it is how the two are held together while both stand.
//   TPS_PROFILE=shipped tools/blitz/rust.sh rust bun run tools/draw/weather-check.ts [--ticks N] [level[:query]…]
// (The air's map may part by three cells in a thousand: a box's edge that lies exactly on a cell's middle falls either way by a float's rounding, and the page holds floats.)
//   WEATHER_BREAK=1 …    the rows are read half a metre off: it must fail
import { readFileSync } from 'node:fs';
import { resolve } from 'node:path';
import { standUp, ROOT } from '../qa/headless.ts';
import { RustHost } from '../blitz/rust-host.ts';
import { Air, Fire, FireTable, Section, Want, Wind } from '../../src/draw/sections';
import { Drawn } from '../../src/draw/drawn';
import { DrawWorld } from '../../src/draw/world';
import { AirMap } from '../../src/picture/air';
import { AIR_SIM as PIC_AIR } from '../../src/picture/air';

const { loadLevel } = await import(`${ROOT}/src/scene/levelLoad.ts`);
const { parseLevelData } = await import(`${ROOT}/src/scene/levelData.ts`);
const { startFromQuery } = await import(`${ROOT}/src/game/scenarioRun.ts`);
const { fireDraws, FIRE } = await import(`${ROOT}/src/game/fire.ts`);
const { airMask, AIR_SIM } = await import(`${ROOT}/src/game/airSim.ts`);

const args = process.argv.slice(2), ticksAt = args.indexOf('--ticks'), TICKS = ticksAt >= 0 ? Number(args[ticksAt + 1]) : 240, LEVELS = args.filter((a, i) => !a.startsWith('--') && i !== ticksAt + 1);
const BREAK = process.env.WEATHER_BREAK === '1' ? 0.5 : 0, DT = 1 / 60;
let fails = 0;
const line = (ok: boolean, what: string, detail = '') => { if (!ok) fails++; console.log(`- ${ok ? 'PASS' : 'FAIL'} — ${what}${detail ? `\n    ${detail}` : ''}`); };
const log = console.log, quiet = async <T>(f: () => T | Promise<T>): Promise<T> => { console.log = () => { }; try { return await f(); } finally { console.log = log; } };
const near = (a: number, b: number, k = 4e-6) => Math.abs(a - b) <= k * Math.max(1, Math.abs(a), Math.abs(b));

// THE TABLES THAT ARE TWO WHILE BOTH PAGES STAND: the picture's copy is the old file's, number for number
{ const same = (a: any, b: any, keys?: string[]) => (keys ?? Object.keys(b)).filter(k => k in a && JSON.stringify(a[k]) !== JSON.stringify(b[k]));
  const off = [['AIR_SIM', same(AIR_SIM, PIC_AIR, Object.keys(PIC_AIR))]].filter(x => x[1].length);
  line(off.length === 0, 'the picture\'s tables are the old files\'', JSON.stringify(off)); }

for (const L of LEVELS.length ? LEVELS : ['pass', 'compound:weather=downpour&roomclear&squad4', 'office']) { const [name, query] = L.split(':');
  const W: any = await quiet(() => standUp({ level: loadLevel(parseLevelData(readFileSync(resolve(ROOT, 'levels', `${name}.json`), 'utf8'))), seed: 3, dress: true }));
  const why = (await quiet(() => startFromQuery(W.game, `?levelFile=${name}.json&${query ?? 'roomclear&squad4'}`)) as any)?.why; if (why) { line(false, `${name}: the start was refused`, why); continue; }
  const g = W.game, E = g.engine, world = E.world, G = g as any, seen = new Map<string, [number, string]>(); let D: Drawn = null as never, Page: DrawWorld = null as never;   // (Page: the statics as the page holds them, of the rows alone)
  const bad = (k: string, w: string) => { const e = seen.get(k); if (e) e[0]++; else seen.set(k, [1, w]); };
  let fires = 0, winds = 0;
  for (let t = 0; t < TICKS; t++) {
    if (t === 30) W.input?.keys?.add?.('KeyW');
    if (t === 60 || t === 150) g.dropThrowable('frag', [g.player.char.pos[0] + 3, 0, g.player.char.pos[2] + 1]);
    g.godMode = true;
    await quiet(() => W.step(DT));
    const H: any = RustHost.last; if (!H) { line(false, `${name}: no Rust game stands`); break; }
    if (t === 0) D = new Drawn(H.F.X, H.F.mem, Want.all);
    D.take(H.h);
    if (t === 0) Page = new DrawWorld(E.device, D);
    D.frame(H.h, 1, [20, 0, 14]); Page.upload();
    const u = D.u, f = D.f;
    // ---- the fires' records (the copy draws those near the view; the rows are every one that burns)
    fireDraws(g);
    const list = E.fires ?? []; fires = Math.max(fires, list.length);
    for (const r of list as any[]) { let at = -1; for (let i = 0; i < D.n[Section.fires]; i++) { const o = D.at[Section.fires] + i * Fire.row; if (near(f[o + Fire.pos], r.pos[0]) && near(f[o + Fire.pos + 2], r.pos[2])) at = o; }
      if (at < 0) { bad('fires.lost', `tick ${t}: the copy draws a fire at ${r.pos} and no row says it`); continue; }
      const P: [string, number, number][] = [['r', r.r, f[at + Fire.r]], ['h', r.h, f[at + Fire.h]], ['heat', r.heat, f[at + Fire.heat]], ['seed', r.seed, f[at + Fire.seed]], ['time', r.time, f[at + Fire.time] + BREAK], ['flick', r.flick, f[at + Fire.flick]], ['bed', r.bed, f[at + Fire.bed]], ['fog', r.fog, u[at + Fire.fog]], ['wx', r.wind[0], f[at + Fire.wind]], ['wz', r.wind[1], f[at + Fire.wind + 1]], ['y', r.pos[1], f[at + Fire.pos + 1]]];
      for (const [k, a, b] of P) if (!near(a, b, 2e-5)) bad(`fires.${k}`, `tick ${t}: copy ${a} rust ${b}`); }
    if (list.length && E.fireLook) { const t0 = D.at[Section.fireTable]; for (let k = 0; k < 15; k++) if (!near(E.fireLook[k], f[t0 + FireTable.look + k])) bad(`fire.look${k}`, `copy ${E.fireLook[k]} rust ${f[t0 + FireTable.look + k]}`); }
    // ---- the wind's lists
    const lists: [string, any[], number][] = [['active', g.wind.active, Section.winds], ['pool', (g.wind as any).forParticles ?? [], Section.gusts]];
    for (const [k, S, sec] of lists) { if (S.length !== D.n[sec]) { bad(`winds.${k}.count`, `tick ${t}: copy ${S.length} rust ${D.n[sec]}`); continue; } winds = Math.max(winds, S.length);
      for (let i = 0; i < S.length; i++) { const s = S[i], o = D.at[sec] + i * Wind.row, P: [string, number, number][] = [['kind', s.kind === 'storm' ? 1 : 0, u[o + Wind.kind]], ['speed', s.speed, f[o + Wind.speed] + BREAK], ['x', s.pos[0], f[o + Wind.pos]], ['dx', s.dir[0], f[o + Wind.dir]], ['dz', s.dir[2], f[o + Wind.dir + 2]], ['reach', s.reach, f[o + Wind.reach]], ['r0', s.r0, f[o + Wind.r0]], ['dMax', s.dMax, f[o + Wind.dMax]], ['boundR', s.bound.r, f[o + Wind.bound + 3]], ['roofs', s.roofs?.length ?? 0, u[o + Wind.roofs]]];
        for (const [w, a, b] of P) if (!near(a, b)) bad(`winds.${k}.${w}`, `tick ${t}, ${s.name}: copy ${a} rust ${b}`); } }
    { const A = D.at[Section.air], s = g.wind.byName('storm'); if (s && !near(s.speed, f[A + Air.stormSpeed])) bad('air.storm', `tick ${t}: copy ${s.speed} rust ${f[A + Air.stormSpeed]}`); if (!!g.weather.levelRoofs !== (u[A + Air.levelRoofs] !== 0)) bad('air.levelRoofs', `copy ${g.weather.levelRoofs}`); }
    if (t % 40 && t !== TICKS - 1) continue;
    // ---- the air's map, of the same statics, the panes and the leaves as the rows say them
    if (name !== 'pass') { const was = AIR_SIM.on; AIR_SIM.on = true;   // (the pass's banks are told with the snow)
      const M = airMask(g), P = new AirMap(D, Page.statics); P.redraw(); AIR_SIM.on = was; let diff = 0, first = ''; for (let i = 0; i < M.mask.length; i++) if (M.mask[i] !== P.mask[i]) { diff++; first ||= `cell ${i % M.w}, ${Math.floor(i / M.w)}: copy ${M.mask[i]} rows ${P.mask[i]}`; } if (diff > M.mask.length * 0.003) bad('air.map', `tick ${t}: ${diff} cells; ${first}`); }
  }
  console.log(`  ${name}: at most ${fires} fires, ${winds} sources`);
  if (name === 'pass' && !fires) bad('floor.fires', 'no fire was compared');
  if (!winds) bad('floor.winds', 'no wind was compared');
  line(seen.size === 0, `${name}: fires, winds, air are the copy's over ${TICKS} ticks`, [...seen].map(([k, [c, w]]) => `${k} × ${c} (${w})`).join('\n    '));
}
console.log(fails ? `=== weather rows: ${fails} FAILED ===` : '=== weather rows: ALL PASS ===');
process.exit(fails ? 1 : 0);
