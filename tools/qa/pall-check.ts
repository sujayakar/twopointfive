// A BLAST'S PALL (src/game/destruction.ts CHAOS.pall — the maintainer, 10-14: "i'd also expect much more dust/smoke from a wall charge" / "same goes for 40mm" / "and
// frag"). Headless: the smoke solver is a null object here, so this check reads what the game ASKS of it — every emitter handed to g.smoke.emit, by its
// radius, density and seconds (its `mass` = π^1.5 · radius³ · density · ttl, the solver's own gaussian) — and nothing of how it then looks or what a sight
// line reads through it: that is tools/qa/pall-page.cjs, on the real page.
//   bun run tools/qa/pall-check.ts            a wall charge, a hand grenade and a 40 mm on the block's floor: the dust asked of the solver, where it stands,
//                                             how long; the far side only through a hole, and by a wall that stands no emitter in it or beyond it (a
//                                             grenade against one, a 40 mm fired at one); the allowance, the charge it never refuses, the lane's reset; a
//                                             tight spot heaps nothing; the particle layer follows the solver's part; a starved solver; nothing at chaos 0
//                                             or under a show's own chaos; the panel's reset
//   PALL_BREAK=1 …                            every floor stood at chaos 0, where no pall is raised: the lines that want one must fail
// 2026-11-07: the pall, its solver part, the wall charge's own share of it (WALLSHOT) and the destruction switch are settled rules, and a write of the other
// side of any is refused. Gone with that: 'the stand'; every 'without it …' half and the mass 'twice the dust it raised before' (the second floor with the
// rule off was the same floor); 'every other emitter the same'; 'with the solver's part off'; 'the one switch off'; the digest with the rule on and off; the
// recording from before the key and the page's ?pall=0 / ?pall=look. A PALL EMITTER IS KNOWN BY WHAT IT IS (the pall's own call alone asks a coarse puff of
// priority 1 over three voxels or more): until now by a wrap of `pallEmit` on the tools' copy of the game, which the Rust tick never calls, so every count
// of them read 0. THE COUNTERS ARE READ AS DIFFERENCES (they are the game's own and run on over the floors of one process; zeroing the copy's zeroes nothing).
// 'A charge whose wall held' went: it was staged by a hook on the copy (`holeAt` answering null), which the Rust game never asks.
import { standUp, ROOT } from './headless.ts';
import { samSets } from './charge-rig.ts';
import { readFileSync } from 'node:fs';
import { resolve } from 'node:path';
const BREAK = process.env.PALL_BREAK === '1';
const { CHAOS, PALL_STATS } = await import(`${ROOT}/src/game/destruction.ts`);
const { WALLCHARGE, wallAt, wallPt } = await import(`${ROOT}/src/game/wallcharge.ts`);
const { blast: fragBlast, FRAG } = await import(`${ROOT}/src/game/frag.ts`);
const { fireShell, HE40, HE40_STATS } = await import(`${ROOT}/src/game/he40.ts`);
const { loadLevel } = await import(`${ROOT}/src/scene/levelLoad.ts`);
const { parseLevelData } = await import(`${ROOT}/src/scene/levelData.ts`);
const { startFromQuery } = await import(`${ROOT}/src/game/scenarioRun.ts`);
type Vec3 = [number, number, number];
const PLAYER_ID = 1, f1 = (x: number) => x.toFixed(1), f2 = (x: number) => x.toFixed(2);
const massOf = (e: any) => Math.pow(Math.PI, 1.5) * e.radius ** 3 * e.density * e.ttl;
/** the pall's counters now, and what they have gone up by since a reading (they are the game's own and run on from floor to floor) */
const stats = () => ({ ...PALL_STATS }) as Record<string, number>;
const since = (a: Record<string, number>) => { const b = stats(), d: Record<string, number> = {}; for (const k of Object.keys(b)) d[k] = b[k] - (a[k] ?? 0); return d; };
/** the pall's own emitter among all that are asked of the solver: its one call (destruction.ts pallEmit) is the only one that asks a coarse puff of priority 1 over three voxels or more */
const isPall = (e: any) => e.kind === 'puff' && e.lattice === 'coarse' && e.prio === 1 && e.minVoxels === 3;

/** the block's floor with its room clear running, every emitter the game hands the solver kept in `emits` */
export async function floor(opts: { chaos?: number } = {}) {
  const lvl = loadLevel(parseLevelData(readFileSync(resolve(ROOT, 'levels', 'block.json'), 'utf8')));
  const W: any = await standUp({ level: lvl, dress: true }); const g = W.game; g.tune.chaos = BREAK ? 0 : opts.chaos ?? 0.5; g.godMode = true;   // (the dressing rows up, as the page has them: the debris pool a blast's rubble lies in is one)
  if (lvl.scenarios?.roomClear) await startFromQuery(g, new URLSearchParams('roomclear&squad4'));
  g.aiEnabled = false;   // (after the deal: the room clear refuses a floor nobody would move on)
  const step = (n: number) => { for (let i = 0; i < n; i++) W.step(1 / 60); }; step(30);
  const emits: any[] = []; g.smoke.emit = (e: any) => { emits.push({ ...e, pos: [...e.pos], t: g.time, pall: isPall(e) }); };   // (the solver is a null object here and the game's own call to it does arrive: what is asked is kept)
  return { W, g, step, emits };
}
export type Floor = Awaited<ReturnType<typeof floor>>;
/** the pall's emitters among what was asked, and the rest */
const split = (emits: any[]) => ({ pall: emits.filter(e => e.pall), rest: emits.filter(e => !e.pall) });
const dustOf = (emits: any[]) => emits.filter(e => e.kind === 'puff');
const sum = (xs: any[]) => xs.reduce((a, e) => a + massOf(e), 0);
/** the most emitters of a list within 0.15 m of one of them on the plan, itself counted (a burst's column stands over its ring by design: two is no heap) */
const heapOf = (P: any[]) => Math.max(0, ...P.map(e => P.filter(o => Math.hypot(o.pos[0] - e.pos[0], o.pos[2] - e.pos[2]) < 0.15).length));
export const thickOf = (w: any) => 2 * (w.alongX ? w.box.h[2] : w.box.h[0]);
/** how far off the wall's face (the charge's side positive) an emitter stands */
export const offOf = (w: any) => { const face = wallPt(w, 0, 0, 0), nrm = wallPt(w, 0, 1, 0); return (e: any) => (e.pos[0] - face[0]) * (nrm[0] - face[0]) + (e.pos[2] - face[2]) * (nrm[2] - face[2]); };

/** a wall of this floor the game offers a charge on, that a charge then holes: the first by the walls' own order */
export function wallSpot(g: any): any {
  const C = g.wallCharge, H = WALLCHARGE.height;
  for (const B of g.destr.walls.all()) { if (B.kind !== 'wall') continue; const b = B.box, ax = b.h[0] >= b.h[2], T = ax ? b.h[2] : b.h[0];
    for (const side of [1, -1]) { const p: Vec3 = ax ? [b.c[0], H, b.c[2] + side * (T + 0.06)] : [b.c[0] + side * (T + 0.06), H, b.c[2]], from: Vec3 = ax ? [p[0], 0, p[2] + side] : [p[0] + side, 0, p[2]];
      const w0 = wallAt(g, [p[0], 0, p[2]], from), w = w0 && w0.ok && !w0.outer && w0.index === B.id ? C.settle(w0) : null;
      if (w && w.ok && C.setSpot(w) && !C.openAt(w) && C.farFloor(w) && g.destr.walls.wouldOpen(wallPt(w, 0, 0.06, H), {}).some((o: any) => o.id === w.index && o.hole)) return w; } }
  return null;
}
/** the charge set on the spot by Sam's own hands (charge-rig.ts samSets: one pushed onto the tools' copy of the game by hand is a write the Rust game never
 *  gets) and set off through the module's own trigger, two seconds on: what was asked of the solver, and which side of the wall each pall emitter stands on.
 *  `between` runs once the charge is on the wall and before it goes (the setting takes Sam four seconds: what must be fresh at the blast is done here) */
export function charge(S: Floor, between?: () => void) {
  const g = S.g, C = g.wallCharge, w0 = wallSpot(g); if (!w0) return null;
  const chaos = g.tune.chaos; if (chaos === 0) g.tune.chaos = 0.5;   // (a floor stood at chaos 0 — a break word's — offers no wall row: the charge is set as on any floor, and goes off at the floor's own chaos)
  const put = samSets(S.W, w0), c = put.charge, w = put.wall; g.tune.chaos = chaos; if (!c) throw new Error(`Sam could not set the charge: ${put.why}`);
  between?.(); const n0 = S.emits.length; C.blow(c); S.step(120);
  const got = S.emits.slice(n0), { pall, rest } = split(got), off = offOf(w), T = thickOf(w);
  return { hole: !!C.holeAt(w), wall: w.index, pall, rest, near: pall.filter(e => off(e) > 0).length, far: pall.filter(e => off(e) < -T).length, inWall: pall.filter(e => off(e) <= 0 && off(e) >= -T).length, dust: dustOf(rest) };
}
/** an open place on the floor to burst a thing at: half a metre off Sam */
export const openAt = (g: any): Vec3 => { const p = g.player.char.pos; return [p[0] + 0.5, 0, p[2] + 0.5]; };
export function burst(S: Floor, kind: 'frag' | '40 mm', where?: Vec3) {
  const g = S.g, n0 = S.emits.length, at = where ?? openAt(g);
  if (kind === 'frag') fragBlast(g, at, { who: PLAYER_ID, name: 'frag grenade', destr: { radius: FRAG.destrM, power: 1 } });
  else fragBlast(g, [at[0], at[1] - FRAG.lift, at[2]], { who: PLAYER_ID, name: '40 mm grenade' });   // (he40.ts's own call, word for word: the round's burst is the frag's blast with its own radii)
  S.step(120); const got = S.emits.slice(n0), { pall, rest } = split(got);
  return { pall, rest, dust: dustOf(rest), at };
}

export async function pallCheck(): Promise<boolean> {
  console.log(`\n=== pall: a wall charge, a hand grenade and a 40 mm raise a pall of dust and smoke${BREAK ? ' — PALL_BREAK: every floor at chaos 0' : ''} ===`);
  let all = true; const verdict = (ok: boolean, label: string, detail: string) => { all &&= ok; console.log(`- ${ok ? 'PASS' : 'FAIL'} — ${label}\n    ${detail}`); };
  const rnd0 = Math.random, K = CHAOS.pall, density0 = K.wall.density;
  const log = console.log, quiet = async <T>(fn: () => Promise<T>): Promise<T> => { console.log = () => { }; try { return await fn(); } finally { console.log = log; } };
  try {
    // A WALL CHARGE
    { const S = await quiet(() => floor()), s0 = stats(), A = await quiet(async () => charge(S)), st = since(s0);
      const mA = A ? sum(A.pall) : 0, long = A && A.pall.length ? Math.min(...A.pall.map(e => e.ttl)) : 0;
      verdict(!!A && A.hole && A.near === K.wall.near && A.far === K.wall.far && A.inWall === 0 && long >= 1 && st.palls === 1,
        'a wall charge that opens its wall: the pall stands on both sides of it, seconds long, none of it in the wall',
        `hole ${A?.hole}, ${A?.pall.length ?? 0} emitters (${A?.near} on the charge's side, ${A?.far} through the hole, ${A?.inWall} in the wall), the shortest ${f2(long)} s, mass ${f2(mA)} · the charge's other puffs in two seconds: ${A?.dust.length ?? 0}, mass ${f2(A ? sum(A.dust.filter(e => !e.pall)) : 0)} · counters since the floor stood ${JSON.stringify(st)}`); }
    // …on a wall that IS whole (the entry point asked directly) every emitter stands in the charge's room, clear of the wall — the solver lays an emitter down
    // through whatever is thin enough
    { const S = await quiet(() => floor()), g = S.g, w = wallSpot(g); let near = 0, bad = 0, close = 9;
      if (w) { g.destr.pallWall(wallPt(w, 0, 0, 0), w.alongX ? [0, 0, w.side] : [w.side, 0, 0], w.alongX ? [1, 0, 0] : [0, 0, 1], thickOf(w), false); const P = split(S.emits).pall, off = offOf(w);
        near = P.filter(e => off(e) > 0).length; bad = P.filter(e => off(e) <= 0).length; close = Math.min(9, ...P.map(e => off(e) / e.radius)); }
      verdict(!!w && near === K.wall.near && bad === 0 && close >= 1.5, 'on a whole wall every emitter stands in the charge\'s room, a radius and a half or more off the face',
        `a whole wall: ${near} on the charge's side, ${bad} in or beyond it, the closest ${f2(close)} radii off the face`); }
    // A BURST HARD BY A WALL THAT STANDS (one no blast here opens: not on the walls' own list): a hand grenade lying against it, and a 40 mm fired at it down
    // its own path (he40.ts: the round flies, arms, bursts off the face — its arming run shortened to the lane this floor has)
    { const hardWall = (g: any): any => { const soft = new Set(g.destr.walls.all().map((B: any) => B.id));
        for (let j = 0; j < g.col.boxes.length; j++) { const b = g.col.boxes[j]; if (soft.has(j) || (b.flags & 1) || b.h[1] < 1.2 || Math.min(b.h[0], b.h[2]) > 0.2 || Math.max(b.h[0], b.h[2]) < 1) continue; const ax = b.h[0] >= b.h[2];
          for (const side of [1, -1] as const) { const w = { index: j, box: b, alongX: ax, u: ax ? b.c[0] : b.c[2], side };
            if (!g.col.solidBlocked(wallPt(w, 0, 3.5, 1.3), wallPt(w, 0, 0.05, 1.3)) && !g.col.solidBlocked(wallPt(w, 0, 3.5, 0.3), wallPt(w, 0, 0.05, 0.3)) && g.col.solidBlocked(wallPt(w, 0, 0.3, 1.3), wallPt(w, 0, -thickOf(w) - 0.3, 1.3))) return w; } }
        return null; };
      const stands = (g: any, w: any) => g.col.solidBlocked(wallPt(w, 0, 0.3, 1.3), wallPt(w, 0, -thickOf(w) - 0.3, 1.3));
      const S = await quiet(() => floor()), g = S.g, w = hardWall(g), n1 = S.emits.length, s1 = stats();
      if (w) await quiet(async () => { burst(S, 'frag', wallPt(w, 0, 0.06, 0.1)); });
      const P1 = split(S.emits.slice(n1)).pall, off = w ? offOf(w) : () => 0, bad1 = P1.filter(e => off(e) <= 0).length, close1 = Math.min(9, ...P1.map(e => off(e) / e.radius)), up1 = !!w && stands(g, w), heap1 = heapOf(P1), shut1 = since(s1).shut;
      const S2 = await quiet(() => floor()), g2 = S2.g, w2 = hardWall(g2), arm0 = HE40.armM, b0 = HE40_STATS.bursts, n2 = S2.emits.length; let P2: any[] = [], bad2 = -1, close2 = 0;
      if (w2) { HE40.armM = 2; try { await quiet(async () => { fireShell(g2, g2.player.char, wallPt(w2, 0, 3.5, 1.3), wallPt(w2, 0, 0, 1.3)); S2.step(90); }); } finally { HE40.armM = arm0; }
        P2 = split(S2.emits.slice(n2)).pall; const o2 = offOf(w2); bad2 = P2.filter(e => o2(e) <= 0).length; close2 = Math.min(9, ...P2.map(e => o2(e) / e.radius)); }
      const up2 = !!w2 && stands(g2, w2);
      verdict(!!w && up1 && P1.length >= 4 && bad1 === 0 && close1 >= 1.5 && heap1 <= 2 && up2 && HE40_STATS.bursts === b0 + 1 && P2.length >= 4 && bad2 === 0 && close2 >= 1.5,
        'a burst hard by a wall that stands — a hand grenade lying against it, a 40 mm fired at it — lays its pall in the room it burst in: no emitter in the wall or beyond it, each a radius and a half or more off the face, no heap of them on one spot (the ring\'s half that fell in the wall is turned back into the room, not laid on the burst)',
        `${w ? `wall #${w.index}, still standing ${up1}` : 'NO such wall on this floor'} · the grenade: ${P1.length} emitters, ${bad1} in or beyond the wall, the closest ${f2(close1)} radii off it, at most ${heap1} within 0.15 m of one another, ${shut1} left out, radii ${f2(Math.min(9, ...P1.map(e => e.radius)))}‥${f2(Math.max(0, ...P1.map(e => e.radius)))} · the 40 mm: fired 3.5 m at it, ${HE40_STATS.bursts - b0} burst, ${P2.length} emitters, ${bad2} in or beyond the wall, the closest ${f2(close2)} radii off it, still standing ${up2}`); }
    // A HAND GRENADE and A 40 MM, the same way
    for (const kind of ['frag', '40 mm'] as const) {
      const S = await quiet(() => floor()), s0 = stats(), A = await quiet(async () => burst(S, kind)), st = since(s0);
      const mA = sum(A.pall), reach = Math.max(0, ...A.pall.map(e => Math.hypot(e.pos[0] - A.at[0], e.pos[2] - A.at[2]))), dark = A.pall.filter(e => e.tint && e.tint[0] < 0.5).length;
      verdict(A.pall.length >= 4 && dark >= 1 && Math.min(...A.pall.map(e => e.ttl), 99) >= 0.9 && reach <= 1.6 && st.palls === 1,
        `a ${kind === 'frag' ? 'hand grenade' : '40 mm round'} bursting on the floor: a ring of dust round it and a darker column over it`,
        `${A.pall.length} emitters (${dark} dark), within ${f2(reach)} m of the burst, mass ${f2(mA)} · its other puffs: ${A.dust.filter(e => !e.pall).length}, mass ${f2(sum(A.dust.filter(e => !e.pall)))} · counters since the floor stood ${JSON.stringify(st)}`); }
    // A TIGHT SPOT (the block's slot between a locker and the outer walls, a review's find: five of six emitters on one point at 9 a second): what has no
    // room is left out or shrunk, never heaped
    { const S = await quiet(() => floor()), g = S.g, n0 = S.emits.length, s0 = stats(); await quiet(async () => { g.destr.pallBurst([35.64, 0.3, 4.75], 1); });
      const P = split(S.emits.slice(n0)).pall, heap = heapOf(P), st = since(s0), most = K.burst.density * K.maxGain * (1 + K.burst.coreGain), peak = Math.max(0, ...P.map(e => P.filter(o => Math.hypot(o.pos[0] - e.pos[0], o.pos[2] - e.pos[2]) < 0.15).reduce((a, o) => a + o.density, 0)));
      verdict(st.palls === 1 && P.length >= 1 && st.shut >= 1 && heap <= 2 && peak <= most + 1e-6, 'a burst in a tight spot heaps nothing: what has no room is left out, no point is asked for more than a ring emitter and the column together',
        `the slot at (35.64, 4.75): ${P.length} emitters, ${st.shut} left out, at most ${heap} within 0.15 m of one another, the most asked at one point ${f2(peak)} /s (a ring emitter and the column at their most: ${f2(most)})`); }
    // THE PARTICLE LAYER FOLLOWS THE SOLVER'S PART (a stand-in for the GPU particles, there for the pall's own calls alone, that keeps what it is asked:
    // headless there are none): with the pall its long dust goes; over a solver that took none of it, none
    { const S = await quiet(() => floor()), g = S.g, asked: any[] = [], E = g.engine, had = { p: E.particles, s: E.settings }, at = openAt(g), s0 = stats();
      const fake = new Proxy({}, { get: (_t, k) => k === 'emit' ? (kind: string, _p: any, _d: any, n: number, o: any) => { asked.push({ kind, n, life: o?.life ?? 0 }); return n; } : () => 0 });
      const one = () => { E.settings = { ...(had.s ?? {}), gpuParticles: true }; E.particles = fake; try { g.destr.pallBurst(at, 1); } finally { E.particles = had.p; E.settings = had.s; } };
      const long = () => asked.filter(a => a.kind === 'dust' && a.life >= K.burst.dustLife - 1e-6).length, budget = g.smoke.budget;
      one(); const withIt = long(); g.smoke.budget = () => K.slack - 1; one(); const starved = long() - withIt; g.smoke.budget = budget; const st = since(s0);
      verdict(withIt === 1 && starved === 0 && st.palls === 2, 'the pall\'s particle layer goes with its solver part, not over a solver that took none of it (a cloud nobody is hidden by)', `long dust asked: ${withIt} with the pall · ${starved} over a starved solver · ${st.palls} palls`); }
    // THE ALLOWANCE: two bursts more than perWindow inside a second raise perWindow palls; once the window has passed the next one goes
    { const S = await quiet(() => floor()), g = S.g, at = openAt(g), s0 = stats();
      const N = K.perWindow + 2; await quiet(async () => { for (let k = 0; k < N; k++) { g.destr.blast([at[0] + k * 0.2, 0.2, at[2]], { who: PLAYER_ID, radius: 2.5 }); S.step(3); } });
      const first = since(s0); await quiet(async () => { S.step(Math.ceil(K.windowSecs * 60) + 10); g.destr.blast([at[0], 0.2, at[2]], { who: PLAYER_ID, radius: 2.5 }); S.step(2); }); const then = since(s0);
      verdict(first.palls === K.perWindow && first.refused === 2 && then.palls === K.perWindow + 1, `the allowance: ${N} bursts in half a second raise ${K.perWindow} palls and the other two the old dust alone; ${K.windowSecs} s on the next burst raises its own`, `in the window: ${first.palls} palls, ${first.refused} refused · after it: ${then.palls} palls`);
      // …the wall charge is never the one refused; a restart of the floor (the lane's reset) empties the window. (The bursts go off once the charge is on the
      //  wall: its setting takes longer than the window is)
      const S2 = await quiet(() => floor()), g2 = S2.g, at2 = openAt(g2), t0 = stats();
      const C2 = await quiet(async () => charge(S2, () => { for (let k = 0; k < K.perWindow; k++) { g2.destr.blast([at2[0] + k * 0.3, 0.2, at2[2]], { who: PLAYER_ID, radius: 2.5 }); S2.step(3); } })), afterCharge = since(t0).palls;
      await quiet(async () => { g2.destr.blast([at2[0], 0.2, at2[2]], { who: PLAYER_ID, radius: 2.5 }); }); const full = since(t0);
      await quiet(async () => { g2.destr.reset(); g2.destr.blast([at2[0], 0.2, at2[2]], { who: PLAYER_ID, radius: 2.5 }); }); const end = since(t0);
      verdict(!!C2 && C2.pall.length >= K.wall.near && afterCharge === K.perWindow + 1 && full.palls === K.perWindow + 1 && full.refused === 1 && end.palls === K.perWindow + 2,
        'a wall charge blown inside a full window still raises its pall (the player\'s own set piece is never the one refused) and counts in it; the lane\'s reset empties the window', `after ${K.perWindow} bursts the charge: ${C2?.pall.length ?? 0} emitters, ${afterCharge} palls · one more burst: ${full.palls} palls, ${full.refused} refused · after reset() the same burst: ${end.palls} palls`); }
    // A STARVED SOLVER: under `slack` free slots nothing of the pall is asked
    { const S = await quiet(() => floor()), g = S.g, s0 = stats(); g.smoke.budget = () => K.slack - 1; await quiet(async () => { burst(S, 'frag'); }); const starved = split(S.emits).pall.length, st = since(s0);
      verdict(starved === 0 && st.starved >= 4, `a solver with fewer than ${K.slack} slots free is asked for none of it`, `starved: ${starved} emitters, ${st.starved} turned away`); }
    // NOTHING at chaos 0, nor under a show's own chaos
    { const S = await quiet(() => floor({ chaos: 0 })), s0 = stats(); await quiet(async () => { burst(S, 'frag'); }); const a = split(S.emits).pall.length, pa = since(s0).palls;
      const S3 = await quiet(() => floor()), s3 = stats(); S3.g.destr.showChaos = 0.8; await quiet(async () => { burst(S3, 'frag'); }); const c = split(S3.emits).pall.length, pc = since(s3).palls;
      verdict(a === 0 && pa === 0 && c === 0 && pc === 0, 'chaos 0, or a level script\'s show running on its own chaos (its cameras keep the picture they were set on): no pall', `chaos 0: ${a} emitters, ${pa} palls · a show at 0.8: ${c}, ${pc}`);
      }
  } finally { console.log = log; K.wall.density = density0; Math.random = rnd0; }
  console.log(`=== pall: ${all ? 'ALL PASS' : 'FAILURES above'} ===`);
  return all;
}
if (import.meta.main) process.exit((await pallCheck()) ? 0 : 1);
