// OFF THE LINE OF HIS FIRE — the check behind src/game/fireline.ts (FIRELINE.on, settled on).
//   bun run tools/qa/fireline-check.ts        (also `probes.ts fireline`)
// NO SETTLED SWITCH IS WRITTEN HERE (2026-11-07): the rule, friendly fire, the friends' low ready and snap rows, the team's pace and the picture are the
// page's. Out with the off side: (1) the stand's word, (2) THE FAULT, rule off, (9) the recording from before the table and the stands' list.
// The room clear's floor with the pair (doortools-probes.ts rig), the defenders kept calm in their rooms by hand, friendly fire on, as the page has it (a round
// of Sam's is a round to his own men). Sam stands on the middle aisle of the cubicles and fires the rifle down it with real clicks, five every two seconds; a
// teammate is given the menu's own 'move there' from one side of the aisle to the other, down the passage that crosses it 6.5 m ahead of the muzzle:
//   (3) he stops short of the lane, stands there for as long as the fire lasts, and is never in it under fire;
//   (4) the fire stopped: he walks on and is on his mark — he never ran for any of it;
//   (5) no way round worth walking from there (it is a long way back round the block): he looked, found none, and waited on;
//   (6) Sam a stride back from the crossing instead: after the wait's ceiling he walks round BEHIND Sam and is on his mark with the fire still going,
//       never in the lane and never over a walk;
//  (12) Sam changes magazines in the middle of the string: the wait lasts through the reload — one wait, and no frame in the lane before the last round;
// NOT here (read in the module, not staged): a door order past its stack, a man on a wall charge, a man in a doorway, a man stepping out of Sam's way or
// mid-crossing for the silent squad, the gun swung onto a waiting man; the room fight's own lanes; the page (fireline-page.cjs).
// FIRELINE_BREAK=1 = Sam's trigger is never pulled, so there is no fire to wait for: the lines that count his rounds and the waits must fail.   exit 0 = all hold · 1 = one does not
import './stand.ts';   // (the stand's word, written down before this file reads it)
import { resolve } from 'node:path';
const ROOT = resolve(import.meta.dir, '../..');
const DT = 1 / 60, BREAK = process.env.FIRELINE_BREAK === '1', VERBOSE = !!process.env.FIRELINE_VERBOSE;
const f2 = (x: number) => x.toFixed(2), flat = (a: ArrayLike<number>, b: ArrayLike<number>) => Math.hypot(a[0] - b[0], a[2] - b[2]);

/** the aisle Sam fires down (z), where the passage crosses it (x), and the teammate's two ends */
const Z = 18.0, X = 19.5, FROM = [X, 0, 22.4], TO = [X, 0, 14.6];

interface Arm { /** shut by hand for the teammate: 'fight' = the room fight never calls him busy (what he knows of the enemy stands) · 'all' = that, and nothing he has says there is an enemy at all */ blind?: 'fight' | 'all'; /** 'fall in' on Sam instead of 'move there' */ fallIn?: boolean; /** 'move there' to here instead of the far side */ to?: number[]; /** a new 'move there', to here, the moment he takes a way round */ reorder?: number[]; /** Sam reloads this many seconds into the fire (the clicks go on) */ reload?: number; /** where Sam stands */ sam: number[]; /** seconds of fire */ fire: number; /** frames between clicks (default 24: five rounds every two seconds) */ every?: number; /** the teammate starts here (default: FROM) */ from?: number[]; /** the fire starts when he is this near the aisle's line, m (default: at once) */ startAt?: number; /** a defender stood here, alive and calm */ foe?: number[]; knobs?: Record<string, number> }

async function arm(o: Arm) {
  const { rig } = await import('./doortools-probes.ts');
  const { FIRELINE, FIRELINE_STATS: ST } = await import(`${ROOT}/src/game/fireline.ts`);
  const { FRIENDLYFIRE, FRIENDLYFIRE_STATS: FF } = await import(`${ROOT}/src/game/friendlyfire.ts`);
  const { hostileTo } = await import(`${ROOT}/src/game/teams.ts`);
  const log = console.log, warn = console.warn; if (!VERBOSE) console.log = console.warn = () => { };
  try {
    const R: any = await rig(5101, { stage: 'rc' }); const { W, game, input } = R, pl = game.player, O = game.orders;
    Object.assign(FIRELINE, o.knobs ?? {});
    for (const k of Object.keys(ST)) (ST as any)[k] = 0; const hit0 = FF.samOnMate;
    const mate = R.ops[0], other = R.ops[1], foes = R.hostiles.filter((h: any) => h.state !== 'dead'), foe = o.foe ? foes[0] : null; if (foe) foe.god = true;   // (he is there to have a line, not to be shot)
    const put = (gd: any, p: number[]) => { gd.char.pos[0] = p[0]; gd.char.pos[2] = p[2]; gd.char.vel[0] = gd.char.vel[2] = 0; gd.path = []; gd.pathGoal = null; };
    const home = new Map<any, number[]>(); for (const h of foes) home.set(h, [h.char.pos[0], 0, h.char.pos[2]]);
    /** the defenders hear nothing and stay where they are; the second teammate is parked out of it */
    const calm = () => { for (const h of game.guards) if (hostileTo(h, 'player') && h.state !== 'dead') { h.state = 'patrol'; h.awareness = 0; if (h === foe) put(h, o.foe!); else if (home.has(h)) put(h, home.get(h)!); } put(other, [10, 0, 22.6]); };
    let aimAt: number[] = [o.sam[0] + 9, 1.3, Z], trigger = false, f = 0;
    const step = (n = 1) => { for (let i = 0; i < n; i++) { calm(); pl.char.pos[0] = o.sam[0]; pl.char.pos[2] = o.sam[2]; W.cursorAt(aimAt as any); if (trigger && !BREAK && f % (o.every ?? 24) === 0) { pl.fireCd = 0; input.clicked |= 1; } f++; R.step(); } };
    game.teleportPlayer(o.sam); put(mate, o.from ?? FROM); step(30);
    if (!pl.rifleUp) { input.pressed.add('Digit2'); step(1); if (!pl.rifleUp) { input.pressed.add('KeyE'); step(1); } step(90); }
    if (pl.rifle.def.auto && pl.rifle.automatic) { input.pressed.add('KeyB'); step(2); }   // (SEMI: a click a round)
    const yaw = pl.char.aimYaw, dx = Math.sin(yaw), dz = Math.cos(yaw);
    /** how far off the gun's own line a man is, and how far along it (the muzzle's, this frame) */
    const off = (p: ArrayLike<number>) => { const m = pl.char.muzzle; const rx = p[0] - m[0], rz = p[2] - m[2]; return { along: rx * dx + rz * dz, side: Math.abs(rx * dz - rz * dx) }; };
    const laneHalf = (along: number) => FRIENDLYFIRE.bodyM + FIRELINE.clearM + along * FRIENDLYFIRE.coneK * FIRELINE.spread;
    if (o.blind) game.fight.busy = () => false; if (o.blind === 'all') game.fight.contact = () => false;
    const dest = o.fallIn ? o.sam : o.to ?? TO, destR = o.fallIn ? 3 : 0.5;
    const why = o.fallIn ? O.give('fallIn', O.targetAt(o.sam), undefined, { men: [mate] }) : O.give('move', O.targetAt(dest), undefined, { men: [mate] });
    let fired = 0, inLane = 0, inBody = 0, still = 0, stillMax = 0, nearest = 1e9, fastest = 0, t0 = -1, tStop = -1, tThere = -1, movedAfter = -1, behind = 0, underFire = false, re = -1, reWhy: string | null = null, tThere2 = -1, reloadF = 0, reloaded = false, inString = 0; const shots0 = pl.rifle.shotsFired;
    const trace: string[] = [];
    for (let i = 0; i < (o.fire + 14) * 60; i++) {
      const p = mate.char.pos;
      if (t0 < 0 && (o.startAt === undefined || Math.abs(p[2] - Z) <= o.startAt)) { t0 = game.time; trigger = true; f = 0; }
      if (t0 >= 0 && tStop < 0 && game.time - t0 >= o.fire) { trigger = false; tStop = game.time; }
      if (o.reload !== undefined && !reloaded && t0 >= 0 && game.time - t0 >= o.reload) { reloaded = true; input.pressed.add('KeyR'); }
      if (o.reorder && re < 0 && ST.ways === 1) { re = game.time; reWhy = O.give('move', O.targetAt(o.reorder), undefined, { men: [mate] }); }
      step();
      if (pl.rifle.reloading) reloadF++;
      const firing = pl.rifle.shotsFired > shots0 && game.time - pl.lastFireT < FIRELINE.fireS, q = off(p), v = Math.hypot(mate.char.vel[0], mate.char.vel[2]); fastest = Math.max(fastest, v);
      if (pl.rifle.shotsFired > shots0 && tStop < 0 && q.along > 0.15 && q.side <= laneHalf(q.along)) inString++;
      if (firing) { if (q.along > 0.15 && q.side <= laneHalf(q.along)) inLane++; if (q.along > 0.15 && q.side <= 0.28) inBody++; nearest = Math.min(nearest, q.side - laneHalf(Math.max(0, q.along)));
        if (v < 0.1) { still += DT; stillMax = Math.max(stillMax, still); } else still = 0; if ((p[0] - pl.char.pos[0]) * dx + (p[2] - pl.char.pos[2]) * dz < -0.3) behind++; }
      if (tStop >= 0 && movedAfter < 0 && v > 0.5) movedAfter = game.time - tStop;
      if (tThere < 0 && flat(p, dest) < destR) { tThere = game.time; underFire = firing; }
      if (o.reorder && re >= 0 && tThere2 < 0 && flat(p, o.reorder) < 0.6) tThere2 = game.time;
      if (VERBOSE && i % 30 === 0) trace.push(`${f2(game.time - (t0 < 0 ? game.time : t0))}s ${mate.callsign} ${f2(p[0])},${f2(p[2])} v ${f2(v)} side ${f2(q.side)} along ${f2(q.along)} ${firing ? 'FIRE' : ''} ${mate.state}`);
      if ((tThere >= 0 || tThere2 >= 0) && tStop >= 0 && game.time - tStop > 1.5) break;
    }
    fired = pl.rifle.shotsFired - shots0;
    if (VERBOSE) { console.log = log; console.log(trace.join('\n')); if (foe) console.log('foe', foe.callsign, foe.team, foe.state, foe.char.pos, 'blocked', game.col.segmentBlocked([foe.char.pos[0], 1.6, foe.char.pos[2]], [mate.char.pos[0], 1.2, mate.char.pos[2]])); console.log(JSON.stringify({ why, ST, fired })); }
    return { g: game, why, mate: mate.callsign as string, state: mate.state as string, up: mate.state !== 'dead' && mate.char.alive, fired, inLane, inBody, stillMax, nearest, fastest, hit: FF.samOnMate - hit0,
      there: tThere >= 0, thereUnderFire: underFire, movedAfter, behind, re, reWhy, there2: tThere2 >= 0, reloadF, inString, st: { ...ST } as Record<string, number>, bad: R.bad as string };
  } finally { console.log = log; console.warn = warn; }
}

export async function firelineCheck(): Promise<boolean> {
  console.log('\n=== fireline: a teammate walking on an order waits short of the lane the player is firing down, and crosses when he stops ===');
  let all = true; const verdict = (ok: boolean, what: string, detail: string) => { all &&= ok; console.log(`- ${ok ? 'PASS' : 'FAIL'} — ${what}\n    ${detail}`); };
  const { knobsCopy, knobsBack } = await import(`${ROOT}/src/game/knobs.ts`) as any;
  const { FIRELINE } = await import(`${ROOT}/src/game/fireline.ts`);
  const { ORDERS } = await import(`${ROOT}/src/game/orders.ts`);
  const knobs0 = knobsCopy(), rnd = Math.random;
  try {
    const FAR = [13, 0, Z], NEAR = [18.1, 0, Z];
    const B = await arm({ sam: FAR, fire: 7 });
    verdict(!B.why && B.fired >= 12 && B.inLane === 0 && B.stillMax >= 4 && B.nearest > 0.2 && B.hit === 0 && B.up && B.st.holds === 1 && B.st.enemy === 0, 'he stops short of the lane, stands there while the fire lasts, and is never in it under fire',
      `${B.mate}: ${B.fired} rounds · frames in the lane ${B.inLane} · stood ${f2(B.stillMax)} s · nearest its edge ${f2(B.nearest)} m · rounds into him ${B.hit} · waits ${B.st.holds}${B.why ? ` · order refused: ${B.why}` : ''}`);
    verdict(B.there && !B.thereUnderFire && B.movedAfter >= 0 && B.movedAfter < FIRELINE.fireS + 1 && B.fastest <= ORDERS.brisk + 0.25, 'the fire stopped: he walks on inside a second and a bit and is on his mark; he never ran',
      `walking again ${B.movedAfter < 0 ? 'never' : `${f2(B.movedAfter)} s`} after the last round · on his mark ${B.there} · fastest ${f2(B.fastest)} m/s (brisk ${ORDERS.brisk})`);
    verdict(B.st.noWay >= 1 && B.st.ways === 0, 'no way round worth the walk from there: he looked after the wait\'s ceiling, found none inside the allowance, and waited on', `looks that found none ${B.st.noWay} · ways taken ${B.st.ways} · ceiling ${FIRELINE.maxS} s, allowance ${FIRELINE.wayMoreM} m`);
    const C = await arm({ sam: NEAR, fire: 11.5 });
    verdict(!C.why && C.st.holds >= 1 && C.st.ways === 1 && C.thereUnderFire && C.inLane === 0 && C.hit === 0 && C.behind > 30 && C.stillMax >= FIRELINE.maxS - 1 && C.fastest <= ORDERS.brisk + 0.25, 'Sam a stride back from the crossing: after the ceiling he walks round behind Sam and is on his mark with the fire still going, never in the lane, at a walk',
      `${C.mate}: ${C.fired} rounds · stood ${f2(C.stillMax)} s first · ways taken ${C.st.ways} · frames behind Sam's back ${C.behind} · frames in the lane ${C.inLane} · rounds into him ${C.hit} · on his mark under fire ${C.thereUnderFire} · fastest ${f2(C.fastest)} m/s${C.why ? ` · order refused: ${C.why}` : ''}`);
    const H = await arm({ sam: FAR, fire: 8, reload: 2.6 });
    verdict(!H.why && H.reloadF >= 60 && H.fired >= 10 && H.st.holds === 1 && H.inString === 0 && H.hit === 0 && H.there && !H.thereUnderFire,
      'Sam changes magazines in the middle of the string: the wait lasts through the reload — one wait, no frame in the lane before the last round, and on his mark after',
      `${H.mate}: ${H.fired} rounds with ${f2(H.reloadF * DT)} s of reload among them · waits ${H.st.holds} · frames in the lane before the last round ${H.inString} · rounds into him ${H.hit} · on his mark after ${H.there}${H.thereUnderFire ? ' (under fire)' : ''}`);
    for (const r of [B, C, H]) if (r.bad) verdict(false, 'no man left the floor', r.bad);
  } catch (e) { console.log(`- FAIL — the probe threw\n    ${e instanceof Error ? e.stack : String(e)}`); all = false; }
  finally { knobsBack(knobs0); Math.random = rnd; }
  if (BREAK) console.log('(FIRELINE_BREAK=1: Sam never fired — the lines above that count his rounds and the waits must have failed)');
  return all;
}
if (import.meta.main) process.exit((await firelineCheck()) ? 0 : 1);
