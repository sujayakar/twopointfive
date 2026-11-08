// GLASS AS BUILDINGS HAVE IT — the check behind GLASSREAL (src/game/glassbreak.ts): two rules, both settled since 2026-11-07 (every tool stands the game with them on).
//   drop    toughened glass goes white and drops at once: no wave across the sheet
//   place   which glass a pane is comes from where it sits: to the floor or a door's panel = toughened, from a sill = float
//
//   bun run tools/qa/glassreal-check.ts [--verbose]     (also `probes.ts glassreal`, family glass)
//
// The lines:
//    5  a blast close enough to dice a toughened pane drops it the same way
//    6  place: the rule on boxes it is handed — to the floor, from a sill, a door's panel, beside a door but starting above a body's reach, a narrow sidelight
//    7  place: the office, pane by pane where it stands: the partition and the frontage on its kick wall toughened, the windows float, the laminated two as they were
//    8  place: what a round does with it: one rifle round takes the partition's sheet whole and leaves a hole in a window
//    9  place: EVERY level's glass counted by kind against the table below (a level edit that moves a pane across the line shows here)
// What went with the rules' off sides: line 1 ('drop off: the pane lets go in the wave, as it did'), and the 'off' half of lines 6 to 9 with the table's second
// column. What went with the fragments' TypeScript hook: lines 2, 3 and 4 (the drop's fragments one by one) and the count of the blast's fragments in line 5.
//
// GLASSREAL_BREAK=1 = the pane the blast line shoots is made float, not toughened: line 5 must fail.   exit 0 = all hold · 1 = one does not
import { readFileSync } from 'node:fs';
import { resolve } from 'node:path';
import { standUp, ROOT, fake } from './headless.ts';
import { officePane, CONF_PANE, LOBBY_PANES } from './office-pane.ts';
const { v3 } = await import(`${ROOT}/src/math/vec.ts`);
const { Particles } = await import(`${ROOT}/src/render/particles.ts`);
const { GLASSBREAK, GLASSREAL, GB_STATS } = await import(`${ROOT}/src/game/glassbreak.ts`);
const { glassImpact, GLASSFX } = await import(`${ROOT}/src/game/glass.ts`);
const { makeBox, BoxFlag } = await import(`${ROOT}/src/scene/boxes.ts`);
const { parseLevelData } = await import(`${ROOT}/src/scene/levelData.ts`);
const { loadLevel } = await import(`${ROOT}/src/scene/levelLoad.ts`);

type Vec3 = [number, number, number];
const DT = 1 / 60, VERBOSE = process.argv.includes('--verbose'), BROKEN = process.env.GLASSREAL_BREAK === '1';
const f2 = (x: number) => x.toFixed(2);
function seeded(a: number) { return () => { a |= 0; a = (a + 0x6D2B79F5) | 0; let t = Math.imul(a ^ (a >>> 15), 1 | a); t = (t + Math.imul(t ^ (t >>> 7), 61 | t)) ^ t; return ((t ^ (t >>> 14)) >>> 0) / 4294967296; }; }
const quiet = async <T>(fn: () => Promise<T> | T): Promise<T> => { const l = console.log, e = console.error, w = console.warn; console.log = console.error = console.warn = () => { }; try { return await fn(); } finally { console.log = l; console.error = e; console.warn = w; } };
/** the glass the drop lines make their pane before they shoot it: toughened — or, under GLASSREAL_BREAK, float (which runs to daggers and does not drop) */
const TOUGH = BROKEN ? 'annealed' : 'tempered';

/** EVERY LEVEL'S GLASS BY KIND: [toughened, float, laminated]. A level not listed must hold no glass. (The
 *  relief and the squads block are their base level's boxes: asked too, against the base's row.) */
const CENSUS: Record<string, [number, number, number]> = {
  office: [10, 9, 3], warehouse: [0, 3, 0], 'test-rooms': [0, 1, 0], registry: [1, 4, 2], records: [2, 5, 0],
  pass: [0, 8, 0], mousehole: [0, 3, 0], killhouse: [4, 0, 0], hall: [1, 0, 0], glasshouse: [10, 0, 0],
  defuse: [0, 2, 0], consulate: [1, 5, 1], block: [0, 4, 0], bazaar: [0, 3, 0], barracks: [0, 12, 0],
  'library-hall': [0, 2, 12],   // (the hut's two panes: the office kit's paneX and paneZ, stood once. The twelve are the house front's six windows, two 0.6 m
                                              //  leaves each, on a wall that stands alone under the sky: no side of it is indoors, so each leaf reads as a narrow light)
  'tut-kit': [0, 1, 0],   // (the fuel halt's shop window again, from its sill)
  'tut-team': [0, 8, 0],   // (the same station, box for box)
  'tut-road': [0, 1, 0],   // (the road post's hut: one sill window on the gateway's side)
};
const baseOf = (file: string) => file.replace(/\.json$/, '').replace(/^office-relief$/, 'office');

/** the office with a Particles attached, everyone parked, Sam in the corridor, the fracture live. */
async function stage() {
  Math.random = seeded(4711);
  const W: any = await quiet(() => standUp()); const { game, engine } = W;
  engine.particles = new Particles(fake('device'), fake('sceneLayout')); engine.settings.gpuParticles = true;
  game.godMode = true; game.player.visibility = 0.02; game.tune.chaos = 0.5; game.tune.glassBreak = true; game.tune.glassShards = false; GLASSFX.on = true;
  for (const k of Object.keys(GB_STATS)) (GB_STATS as any)[k] = 0;
  for (let i = 0; i < 5; i++) W.step(DT);
  game.guards.forEach((gd: any, i: number) => { gd.hold = true; gd.pinned = true; gd.char.pos = [37.5, 0, 6 + i * 1.5]; gd.char.vel = [0, 0, 0]; gd.path = []; gd.pathGoal = null; gd.heardUpTo = 1e12; gd.dazzledUntil = 1e9; gd.flashlight.enabled = false; });
  game.teleportPlayer([8.3, 0, 11.6]); W.step(DT); W.step(DT);
  return { W, game, gb: game.glassBreak };
}
/** pane bi's frame: its centre, the unit vector along it and its normal, on the plan */
function frame(game: any, bi: number) {
  const b = game.level.boxes[bi]; const cs = Math.cos(b.yaw ?? 0), sn = Math.sin(b.yaw ?? 0), wide = b.h[0] >= b.h[2];
  const tan: Vec3 = wide ? [cs, 0, -sn] : [sn, 0, cs], nrm: Vec3 = wide ? [sn, 0, cs] : [cs, 0, -sn];
  return { b, tan, nrm, hu: Math.max(b.h[0], b.h[2]), at: (u: number, v: number): Vec3 => [b.c[0] + tan[0] * u, b.c[1] + v, b.c[2] + tan[2] * u] };
}
export async function glassrealCheck(): Promise<boolean> {
  console.log('\n=== glassreal: toughened glass drops at once, and a pane\'s glass comes from where it sits ==='); let all = true;
  const verdict = (ok: boolean, label: string, detail: string) => { all &&= ok; console.log(`- ${ok ? 'PASS' : 'FAIL'} — ${label}\n    ${detail}`); };
  const rnd = Math.random, fx = GLASSFX.on;
  if (BROKEN) console.log('  (GLASSREAL_BREAK: the pane the blast line shoots is float, not toughened — line 5 must fail)');
  try {
    // ---- 5: a blast
    { const S = await stage(); const { W, game, gb } = S; const bi = LOBBY_PANES[0], fr = frame(game, bi); gb.setType(bi, TOUGH);
      const at = v3.add(fr.at(0, 0), v3.scale(fr.nrm, 0.6)) as Vec3; const t0 = game.time; gb.blast(at, { who: 1 }); const st = gb.panes.get(bi);
      let goneAt = -1, parts = 0; for (let i = 0; i < 40 && goneAt < 0; i++) { const o0 = gb.openFraction(bi); W.step(DT); const o1 = gb.openFraction(bi); if (o1 === 1) goneAt = game.time - t0; else if (o1 > o0 + 1e-9 && i > 0) parts++; }
      verdict(!!st?.dice && goneAt >= GLASSREAL.hang - DT - 1e-9 && goneAt <= GLASSREAL.hang + 2 * DT && parts === 0, '5 a blast that dices a toughened pane drops it the same way',
        `gone ${goneAt < 0 ? 'never' : goneAt.toFixed(3) + ' s'} after the blast (hang ${GLASSREAL.hang}); ${parts} step(s) with part of it leaving`); }
    // ---- 6: the rule on boxes it is handed
    { const S = await stage(); const { game, gb } = S; const D = game.doors.list[0].def, dx = Math.cos(D.closedDir), dz = Math.sin(D.closedDir);
      const pane = (c: Vec3, hu: number, hv: number, yaw = 0) => makeBox({ c, h: [hu, hv, 0.006], yaw, albedo: [0.8, 0.9, 0.9], flags: BoxFlag.Static | BoxFlag.Glass });
      const yawD = -D.closedDir; const beside = (foot: number, gapM: number): Vec3 => [D.hinge[0] - dx * (gapM + 0.5), foot + 0.5, D.hinge[1] - dz * (gapM + 0.5)];
      const cases: [string, any, string][] = [
        ['to the floor, far from any door', pane([-40, 1.2, -40], 1, 1.1), 'tempered'],
        ['from a sill, far from any door', pane([-40, 1.65, -40], 1, 0.75), 'annealed'],
        ['from a sill, a panel of a door (0.1 m off its opening)', pane(beside(0.9, 0.1), 0.5, 0.5, yawD), 'tempered'],
        ['from a sill, 0.6 m off a door', pane(beside(0.9, 0.6), 0.5, 0.5, yawD), 'annealed'],
        ['over a door\'s head, 0.1 m off it', pane(beside(1.6, 0.1), 0.5, 0.5, yawD), 'annealed'],
        ['a narrow sidelight to the floor', pane([-40, 1.25, -40], 0.275, 0.95), 'laminated'],
      ];
      const a = cases.map(c => gb.typeOf(c[1], -1) as string), okOn = cases.every((c, i) => a[i] === c[2]);
      verdict(okOn, '6 place: the rule on boxes it is handed', cases.map((c, i) => `${c[0]}: ${a[i]}`).join(' · ')); }
    // ---- 7, 8: the office
    { const S = await stage(); const { W, game, gb } = S; const lam = officePane(26.475, 10), win = officePane(16, 24), strip = officePane(9, 4);
      const a = [CONF_PANE, LOBBY_PANES[0], win, lam, strip].map(bi => gb.typeOfPane(bi) as string);
      verdict(a.join() === 'tempered,tempered,annealed,laminated,laminated', '7 place: the office, pane by pane where it stands',
        `partition sheet ${a[0]} · frontage on its kick wall ${a[1]} · north window ${a[2]} · manager's sidelight ${a[3]} · high strip ${a[4]}`);
      const one = (bi: number) => { gb.reset(); const fr = frame(game, bi); glassImpact(game, bi, fr.at(0.1, -0.2), v3.scale(fr.nrm, -1) as Vec3, 1, 1); for (let i = 0; i < 90; i++) W.step(DT); const st = gb.panes.get(bi); return { gone: !!st?.gone, open: gb.openFraction(bi) }; };
      const pOn = one(CONF_PANE), wOn = one(win);
      verdict(pOn.gone && !wOn.gone && wOn.open > 0 && wOn.open < 0.2, '8 place: one rifle round takes the partition\'s sheet whole and leaves a hole in a window',
        `partition gone ${pOn.gone}, window ${(100 * wOn.open).toFixed(1)}% open`); }
    // ---- 9: every level
    { const { readdirSync } = await import('node:fs'); const files = readdirSync(resolve(ROOT, 'levels')).filter(f => f.endsWith('.json') && !f.includes('.lint')).sort();
      const bad: string[] = [], lines: string[] = []; let levels = 0, panes = 0;
      for (const file of files) {
        const src = readFileSync(resolve(ROOT, 'levels', file), 'utf8'); const want = CENSUS[baseOf(file)];
        if (!/"Glass"/.test(src)) { if (want) bad.push(`${file}: the table lists glass and the file has none`); continue; }
        const data = parseLevelData(src); const level = loadLevel(data);
        const W: any = await quiet(() => standUp({ level })); const g = W.game; const count = () => { const n: [number, number, number] = [0, 0, 0]; for (const bi of g.engine.world.glassStatics as number[]) { const t = g.glassBreak.typeOfPane(bi); if (t) n[t === 'tempered' ? 0 : t === 'annealed' ? 1 : 2]++; } return n; };
        const on = count(); levels++; panes += on[0] + on[1] + on[2];
        if (!want) { if (on[0] + on[1] + on[2] > 0) bad.push(`${file}: ${on.join('/')} — not in the table`); continue; }
        if (on.join() !== want.join()) bad.push(`${file}: ${on.join('/')} (table ${want.join('/')})`);
        if (VERBOSE) lines.push(`${file}: toughened/float/laminated ${on.join('/')}`);
      }
      for (const l of lines) console.log(`    ${l}`);
      verdict(bad.length === 0 && levels >= 15, '9 place: every level\'s glass by kind against the table', bad.length ? bad.join(' · ') : `${levels} level files with glass, ${panes} panes, every count as tabled`); }
  } finally { Math.random = rnd; GLASSFX.on = fx; }
  console.log(`=== glassreal: ${all ? 'ALL PASS' : 'FAILURES above'} ===`); if (!all) process.exitCode = 1; return all;
}
if (import.meta.main) await glassrealCheck();
