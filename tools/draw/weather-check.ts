// WHAT THE NEW PAGE DRAWS OF THE WEATHER, THE AIR, THE FIRES AND THE SNOW IS WHAT THE COPY OF THE GAME DREW. One Rust game under the headless host; on one tick both sides answer, and the numbers
// are compared: the fires' records, the wind's lists, the air's map, what is outdoors and the drip lines, the snow field baked from the rows beside the one baked from the level, the prints
// pressed by the two, and a blast's crater. It goes when the TypeScript copy does: it is how the two are held together while both stand.
//   TPS_PROFILE=shipped tools/blitz/rust.sh rust bun run tools/draw/weather-check.ts [--ticks N] [level[:query]…]
// (Two parts may part by a little, and why. The drip lines, by a few in a hundred: the picture's straight-down rays do not hold a cut box's shape, so what drips off a car's bonnet
// differs. The air's map, by three cells in a thousand: a box's edge that lies exactly on a cell's middle falls either way by a float's rounding, and the page holds floats.)
//   WEATHER_BREAK=1 …    the rows are read half a metre off: it must fail
import { readFileSync } from 'node:fs';
import { resolve } from 'node:path';
import { standUp, ROOT } from '../qa/headless.ts';
import { RustHost } from '../blitz/rust-host.ts';
import { Air, Fire, FireTable, Geo, Mat, Section, SnowWord, View, Want, Wind } from '../../src/draw/sections';
import { Drawn } from '../../src/draw/drawn';
import { DrawWorld } from '../../src/draw/world';
import { AirMap } from '../../src/picture/air';
import { dripWalk, Roofs } from '../../src/picture/rain';
import { PlanRays } from '../../src/picture/planRays';
import { buildSnowField } from '../../src/render/snowField';
import { formSnowTracks, makeSnowTracks, pressCrater, updateSnowTracks, type Man, SNOWTRACKS } from '../../src/picture/snow/tracks';
import { readMen, sayOf } from '../../src/picture/snow/snow';
import { SnowSight } from '../../src/picture/snow/sight';
import { SNOWMARKS, SnowThings } from '../../src/picture/snow/marks';
import { SNOWBLAST } from '../../src/picture/snow/blast';
import { SNOWBREATH, BREATHSTEAM } from '../../src/picture/snow/breath';
import { SNOWFX, SNOWWISPS, SNOWGRAINS } from '../../src/picture/snow/fx';
import { PRAIN, RAIN_FX } from '../../src/picture/rain';
import { AIR_SIM as PIC_AIR } from '../../src/picture/air';

const { loadLevel } = await import(`${ROOT}/src/scene/levelLoad.ts`);
const { parseLevelData } = await import(`${ROOT}/src/scene/levelData.ts`);
const { startFromQuery } = await import(`${ROOT}/src/game/scenarioRun.ts`);
const { fireDraws, FIRE } = await import(`${ROOT}/src/game/fire.ts`);
const { airMask, AIR_SIM } = await import(`${ROOT}/src/game/airSim.ts`);
const { rainDripPoints, PRAIN: PRAIN0, RAIN_FX: RAIN_FX0 } = await import(`${ROOT}/src/game/rainFx.ts`);
const { snowBlastStats } = await import(`${ROOT}/src/game/snowBlast.ts`);
const { SNOWTRACKS: TRACKS0 } = await import(`${ROOT}/src/game/snowTracks.ts`);
const { SNOWMARKS: MARKS0 } = await import(`${ROOT}/src/game/snowMarks.ts`);
const { SNOWBLAST: BLAST0 } = await import(`${ROOT}/src/game/snowBlast.ts`);
const { SNOWBREATH: BREATH0, BREATHSTEAM: STEAM0 } = await import(`${ROOT}/src/game/snowBreath.ts`);
const { SNOWFX: FX0, SNOWWISPS: WISPS0, SNOWGRAINS: GRAINS0 } = await import(`${ROOT}/src/game/snowFx.ts`);

const args = process.argv.slice(2), ticksAt = args.indexOf('--ticks'), TICKS = ticksAt >= 0 ? Number(args[ticksAt + 1]) : 300, LEVELS = args.filter((a, i) => !a.startsWith('--') && i !== ticksAt + 1);
const BREAK = process.env.WEATHER_BREAK === '1' ? 0.5 : 0, DT = 1 / 60;
let fails = 0;
const line = (ok: boolean, what: string, detail = '') => { if (!ok) fails++; console.log(`- ${ok ? 'PASS' : 'FAIL'} — ${what}${detail ? `\n    ${detail}` : ''}`); };
const log = console.log, quiet = async <T>(f: () => T | Promise<T>): Promise<T> => { console.log = () => { }; try { return await f(); } finally { console.log = log; } };
const near = (a: number, b: number, k = 4e-6) => Math.abs(a - b) <= k * Math.max(1, Math.abs(a), Math.abs(b));

// THE TABLES THAT ARE TWO WHILE BOTH PAGES STAND: the picture's copy is the old file's, number for number
{ const same = (a: any, b: any, keys?: string[]) => (keys ?? Object.keys(b)).filter(k => k in a && JSON.stringify(a[k]) !== JSON.stringify(b[k]));
  const off = [['SNOWTRACKS', same(TRACKS0, SNOWTRACKS)], ['SNOWMARKS', same(MARKS0, SNOWMARKS)], ['SNOWBLAST', same(BLAST0, SNOWBLAST)], ['SNOWBREATH', same(BREATH0, SNOWBREATH)], ['BREATHSTEAM', same(STEAM0, BREATHSTEAM)], ['SNOWFX', same(FX0, SNOWFX)], ['SNOWWISPS', same(WISPS0, SNOWWISPS)], ['SNOWGRAINS', same(GRAINS0, SNOWGRAINS)], ['PRAIN', same(PRAIN0, PRAIN)], ['RAIN_FX', same(RAIN_FX0, RAIN_FX)], ['AIR_SIM', same(AIR_SIM, PIC_AIR, Object.keys(PIC_AIR))]].filter(x => x[1].length);
  line(off.length === 0, 'the picture\'s tables are the old files\'', JSON.stringify(off)); }

for (const L of LEVELS.length ? LEVELS : ['pass', 'compound:weather=downpour&roomclear&squad4', 'office']) { const [name, query] = L.split(':');
  const W: any = await quiet(() => standUp({ level: loadLevel(parseLevelData(readFileSync(resolve(ROOT, 'levels', `${name}.json`), 'utf8'))), seed: 3, dress: true }));
  const why = (await quiet(() => startFromQuery(W.game, `?levelFile=${name}.json&${query ?? 'roomclear&squad4'}`)) as any)?.why; if (why) { line(false, `${name}: the start was refused`, why); continue; }
  const g = W.game, E = g.engine, world = E.world, G = g as any, seen = new Map<string, [number, string]>(); let D: Drawn = null as never, Page: DrawWorld = null as never;   // (Page: the statics as the page holds them, of the rows alone)
  const bad = (k: string, w: string) => { const e = seen.get(k); if (e) e[0]++; else seen.set(k, [1, w]); };
  let fires = 0, winds = 0, drips = 0, craters = 0, field = false, pressed = 0, differ = 0, kept = 0;
  // the picture's side of the snow, fed from the rows alone
  let tracks: any = null, picField: any = null, things: SnowThings | null = null, sight: SnowSight | null = null; const men: Man[] = [], made = new Map<number, Man>();
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
    sayOf(D, Page.statics);
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
    const lists: [string, any[], number][] = [['active', g.wind.active, Section.winds], ['pool', (g.wind as any).gustsAround([0, 1, 2].map(k => f[D.at[Section.view] + View.target + k])), Section.gusts]];   // (the one rule, asked of where the snapshot looks: the host's camera has moved on since the tick took it)
    for (const [k, S, sec] of lists) { if (S.length !== D.n[sec]) { bad(`winds.${k}.count`, `tick ${t}: copy ${S.length} rust ${D.n[sec]}`); continue; } winds = Math.max(winds, S.length);
      for (let i = 0; i < S.length; i++) { const s = S[i], o = D.at[sec] + i * Wind.row, P: [string, number, number][] = [['kind', s.kind === 'storm' ? 1 : 0, u[o + Wind.kind]], ['speed', s.speed, f[o + Wind.speed] + BREAK], ['x', s.pos[0], f[o + Wind.pos]], ['dx', s.dir[0], f[o + Wind.dir]], ['dz', s.dir[2], f[o + Wind.dir + 2]], ['reach', s.reach, f[o + Wind.reach]], ['r0', s.r0, f[o + Wind.r0]], ['dMax', s.dMax, f[o + Wind.dMax]], ['boundR', s.bound.r, f[o + Wind.bound + 3]], ['roofs', s.roofs?.length ?? 0, u[o + Wind.roofs]]];
        for (const [w, a, b] of P) if (!near(a, b)) bad(`winds.${k}.${w}`, `tick ${t}, ${s.name}: copy ${a} rust ${b}`); } }
    { const A = D.at[Section.air], s = g.wind.byName('storm'); if (s && !near(s.speed, f[A + Air.stormSpeed])) bad('air.storm', `tick ${t}: copy ${s.speed} rust ${f[A + Air.stormSpeed]}`); if (!!g.weather.levelRoofs !== (u[A + Air.levelRoofs] !== 0)) bad('air.levelRoofs', `copy ${g.weather.levelRoofs}`); }
    // ---- the prints: the picture's own pressed from the rows beside the copy's on the same tick (a crater told by the copy's own hearing of the blast)
    if (picField && tracks) { const T0 = G.snowTracks; if (T0 && T0.craters > craters) { const l = (snowBlastStats(g) as any).last; if (l) pressCrater(tracks, l.x, l.z, l.size, T0.craters, SNOWBLAST); craters = T0.craters; }
      readMen(D, men, made); sight!.fresh(); updateSnowTracks(men, tracks, picField, DT, sight!); things!.update(tracks, picField, DT, sight!.active ? (x, z) => sight!.groundLook(x, z) !== 2 : null); formSnowTracks(tracks, DT);
      if (T0 && t === TICKS - 1) { for (let k = 0; k < 40; k++) { formSnowTracks(tracks, 0.1); formSnowTracks(T0, 0.1); } const hi = T0.data.length - tracks.w * 3; for (let i = 0; i < hi; i++) { if (T0.data[i]) pressed++; if (Math.abs(T0.data[i] - tracks.data[i]) > 12) differ++; } kept = T0.kept; if (differ > Math.max(40, pressed * 0.06)) bad('snow.prints', `${differ} texels of ${pressed} pressed (${T0.prints} prints in the copy, ${tracks.prints} in the picture; ${T0.marks}, ${tracks.marks} marks; ${T0.kept}, ${tracks.kept} kept)`); } }
    if (t % 40 && t !== TICKS - 1) continue;
    // ---- the air's map, of the same statics, the panes and the leaves as the rows say them
    { const was = AIR_SIM.on; AIR_SIM.on = true; const M = airMask(g), P = new AirMap(D, Page.statics); P.redraw(); AIR_SIM.on = was; let diff = 0, first = ''; for (let i = 0; i < M.mask.length; i++) if (M.mask[i] !== P.mask[i]) { diff++; first ||= `cell ${i % M.w}, ${Math.floor(i / M.w)}: copy ${M.mask[i]} rows ${P.mask[i]}`; } if (diff > M.mask.length * 0.003) bad('air.map', `tick ${t}: ${diff} cells; ${first}`); }
    // ---- what is outdoors, and the drip lines off the statics
    { const R = new Roofs(D); R.from();
      for (let x = 1; x < 40; x += 3) for (let z = 1; z < 28; z += 3) if (R.under(x + 0.3, z + 0.7) !== g.weather.underRoof(x + 0.3, z + 0.7)) bad('roofs.under', `tick ${t}: (${x}, ${z}) copy ${g.weather.underRoof(x + 0.3, z + 0.7)}`);
      if (t === TICKS - 1 && E.settings.particleRain !== false) { const a = rainDripPoints(g), job: any = { boxes: 0, serial: 0, left: Infinity, hold: 0, bi: 0, ei: 0, k: 0, paid: false, out: [], rays: new PlanRays(Page.statics) }; dripWalk(Page.statics, job, R.outdoors); drips = a.length; if (Math.abs(a.length - job.out.length) > Math.max(3, a.length * 0.08)) { const has = (q: any) => job.out.some((d: any) => Math.hypot(d.pos[0] - q.pos[0], d.pos[1] - q.pos[1], d.pos[2] - q.pos[2]) < 1e-3), lost = a.filter((q: any) => !has(q)); bad('rain.drips', `copy ${a.length} lines, picture ${job.out.length}, lost ${lost.length}; first ${lost.slice(0, 4).map((q: any) => `${q.pos.map((v: number) => v.toFixed(2))} box ${q.box} ${world.statics[q.box].c}`)}`); } } }
    // ---- the snow field, baked from the rows as the page has them
    if (D.n[Section.snowWords] && !field) { field = true; G.syncSnowLook?.();
      const boxes = world.statics.map((b: any, i: number) => { const a = D.at[Section.geo] + i * Geo.row, m = D.at[Section.mat] + i * Mat.row, w = u[a + 7]; return { c: [f[a], f[a + 1], f[a + 2]], h: [f[a + 4], f[a + 5], f[a + 6]], yaw: 2 * Math.atan2(f[m + 9], f[m + 11]), flags: w & (0xe7 | (1 << 17)), emissive: [f[m + 4], f[m + 5], f[m + 6]], albedo: [f[m], f[m + 1], f[m + 2]], does: [undefined, 'snow', 'packed snow', 'old snow'][u[D.at[Section.snowWords] + i * SnowWord.row]] }; });
      picField = buildSnowField({ boxes } as never, { wind: [0.8, 0.3] }); const F0 = G.snowField; tracks = makeSnowTracks(); things = new SnowThings(D); sight = new SnowSight(D, Page.statics);
      if (!F0 || !picField) bad('snow.field', `the copy's ${!!F0} the rows' ${!!picField}`);
      else { let dm = 0, dc = 0; for (let i = 0; i < F0.mound.length; i++) { if (Math.abs(F0.mound[i] - picField.mound[i]) > 0.02) dm++; if (Math.abs(F0.cover[i] - picField.cover[i]) > 0.02) dc++; } if (dm + dc > F0.mound.length * 0.005) bad('snow.field.texels', `${dm} mound and ${dc} cover texels of ${F0.mound.length}`); if (F0.slab !== picField.slab) bad('snow.slab', `copy ${F0.slab} rows ${picField.slab}`); } }
  }
  console.log(`  ${name}: at most ${fires} fires, ${winds} sources, ${drips} drip points, ${pressed} texels pressed (${differ} differ), ${craters} craters, ${kept} marks kept`);
  if (name === 'pass' && !fires) bad('floor.fires', 'no fire was compared');
  if (!winds) bad('floor.winds', 'no wind was compared');
  if (!drips) bad('floor.drips', 'no drip line was compared');
  if (name === 'pass' && !pressed) bad('floor.pressed', 'no print was compared');
  if (name === 'pass' && !craters) bad('floor.craters', 'no crater was compared');
  line(seen.size === 0, `${name}: fires, winds, air, roofs, drips${field ? ', the snow field' : ''} are the copy's over ${TICKS} ticks`, [...seen].map(([k, [c, w]]) => `${k} × ${c} (${w})`).join('\n    '));
}
console.log(fails ? `=== weather rows: ${fails} FAILED ===` : '=== weather rows: ALL PASS ===');
process.exit(fails ? 1 : 0);
