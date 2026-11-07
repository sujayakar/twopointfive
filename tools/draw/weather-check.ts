// WHAT THE NEW PAGE DRAWS OF THE FIRES IS WHAT THE COPY OF THE GAME DREW. One Rust game under the headless host; on one tick both sides answer, and the numbers
// are compared: the fires' records. It goes when the TypeScript copy does: it is how the two are held together while both stand.
//   TPS_PROFILE=shipped tools/blitz/rust.sh rust bun run tools/draw/weather-check.ts [--ticks N] [level[:query]…]
//   WEATHER_BREAK=1 …    the rows are read half a metre off: it must fail
import { readFileSync } from 'node:fs';
import { resolve } from 'node:path';
import { standUp, ROOT } from '../qa/headless.ts';
import { RustHost } from '../blitz/rust-host.ts';
import { Fire, FireTable, Section } from '../../src/draw/sections';
import { ALL, Drawn } from '../../src/draw/drawn';

const { loadLevel } = await import(`${ROOT}/src/scene/levelLoad.ts`);
const { parseLevelData } = await import(`${ROOT}/src/scene/levelData.ts`);
const { startFromQuery } = await import(`${ROOT}/src/game/scenarioRun.ts`);
const { fireDraws, FIRE } = await import(`${ROOT}/src/game/fire.ts`);

const args = process.argv.slice(2), ticksAt = args.indexOf('--ticks'), TICKS = ticksAt >= 0 ? Number(args[ticksAt + 1]) : 240, LEVELS = args.filter((a, i) => !a.startsWith('--') && i !== ticksAt + 1);
const BREAK = process.env.WEATHER_BREAK === '1' ? 0.5 : 0, DT = 1 / 60;
let fails = 0;
const line = (ok: boolean, what: string, detail = '') => { if (!ok) fails++; console.log(`- ${ok ? 'PASS' : 'FAIL'} — ${what}${detail ? `\n    ${detail}` : ''}`); };
const log = console.log, quiet = async <T>(f: () => T | Promise<T>): Promise<T> => { console.log = () => { }; try { return await f(); } finally { console.log = log; } };
const near = (a: number, b: number, k = 4e-6) => Math.abs(a - b) <= k * Math.max(1, Math.abs(a), Math.abs(b));

for (const L of LEVELS.length ? LEVELS : ['pass', 'compound:weather=downpour&roomclear&squad4', 'office']) { const [name, query] = L.split(':');
  const W: any = await quiet(() => standUp({ level: loadLevel(parseLevelData(readFileSync(resolve(ROOT, 'levels', `${name}.json`), 'utf8'))), seed: 3, dress: true }));
  const why = (await quiet(() => startFromQuery(W.game, `?levelFile=${name}.json&${query ?? 'roomclear&squad4'}`)) as any)?.why; if (why) { line(false, `${name}: the start was refused`, why); continue; }
  const g = W.game, E = g.engine, world = E.world, G = g as any, seen = new Map<string, [number, string]>(); let D: Drawn = null as never;
  const bad = (k: string, w: string) => { const e = seen.get(k); if (e) e[0]++; else seen.set(k, [1, w]); };
  let fires = 0;
  for (let t = 0; t < TICKS; t++) {
    if (t === 30) W.input?.keys?.add?.('KeyW');
    if (t === 60 || t === 150) g.dropThrowable('frag', [g.player.char.pos[0] + 3, 0, g.player.char.pos[2] + 1]);
    g.godMode = true;
    await quiet(() => W.step(DT));
    const H: any = RustHost.last; if (!H) { line(false, `${name}: no Rust game stands`); break; }
    if (t === 0) D = new Drawn(H.F.X, H.F.mem, ALL);
    D.take(H.h);
    const u = D.u, f = D.f;
    // ---- the fires' records (the copy draws those near the view; the rows are every one that burns)
    fireDraws(g);
    const list = E.fires ?? []; fires = Math.max(fires, list.length);
    for (const r of list as any[]) { let at = -1; for (let i = 0; i < D.n[Section.fires]; i++) { const o = D.at[Section.fires] + i * Fire.row; if (near(f[o + Fire.pos], r.pos[0]) && near(f[o + Fire.pos + 2], r.pos[2])) at = o; }
      if (at < 0) { bad('fires.lost', `tick ${t}: the copy draws a fire at ${r.pos} and no row says it`); continue; }
      const P: [string, number, number][] = [['r', r.r, f[at + Fire.r]], ['h', r.h, f[at + Fire.h]], ['heat', r.heat, f[at + Fire.heat]], ['seed', r.seed, f[at + Fire.seed]], ['time', r.time, f[at + Fire.time] + BREAK], ['flick', r.flick, f[at + Fire.flick]], ['bed', r.bed, f[at + Fire.bed]], ['fog', r.fog, u[at + Fire.fog]], ['wx', r.wind[0], f[at + Fire.wind]], ['wz', r.wind[1], f[at + Fire.wind + 1]], ['y', r.pos[1], f[at + Fire.pos + 1]]];
      for (const [k, a, b] of P) if (!near(a, b, 2e-5)) bad(`fires.${k}`, `tick ${t}: copy ${a} rust ${b}`); }
    if (list.length && E.fireLook) { const t0 = D.at[Section.fireTable]; for (let k = 0; k < 15; k++) if (!near(E.fireLook[k], f[t0 + FireTable.look + k])) bad(`fire.look${k}`, `copy ${E.fireLook[k]} rust ${f[t0 + FireTable.look + k]}`); }
  }
  console.log(`  ${name}: at most ${fires} fires`);
  if (name === 'pass' && !fires) bad('floor.fires', 'no fire was compared');
  line(seen.size === 0, `${name}: the fires are the copy's over ${TICKS} ticks`, [...seen].map(([k, [c, w]]) => `${k} × ${c} (${w})`).join('\n    '));
}
console.log(fails ? `=== weather rows: ${fails} FAILED ===` : '=== weather rows: ALL PASS ===');
process.exit(fails ? 1 : 0);
