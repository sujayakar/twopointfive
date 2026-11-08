// A MAN BACK FROM THE FIGHT WALKS FOR WHERE HIS ORDER STANDS (src/game/orders.ts tick, `ORDERS.retakeGoal`). An order that gave him a corner to walk
// round first; the fight takes him; where it leaves him the way to his place is straight. Back on his order he is handed the new way, which has no
// corner — and he went on walking for the old one, stood on it, and after `stuckSecs` said he could not get through, his order marked stopped short.
// The rig makes that state by hand (a move to open floor near him, then a corner written into his way as an order round a room would have left it,
// then the fight's own state for a tick): the line asks where he ends up. Also: the squad row's words for a cover and an arc hold carry no compass
// bearing while the dialogue pass is on (WORDS.on, the page's stand; orders.ts `cover` / `hold`), and do as they did with it off.
// RETAKE_BREAK=1 = the switch left off (and the bearing put back) and the same lines asked: they must fail.
import { standUp, ROOT } from './headless.ts';

const { ORDERS } = await import(`${ROOT}/src/game/orders.ts`);
const { endGuardScript } = await import(`${ROOT}/src/game/squad.ts`);
const { NIGHTBREACH } = await import(`${ROOT}/src/game/nightbreach.ts`);
const PROFILE = await import(`${ROOT}/src/host/profile.ts`), { FROZEN_HOLDS } = PROFILE;
const { knobsCopy, knobsBack } = await import(`${ROOT}/src/game/knobs.ts`);

const DT = 1 / 60, BREAK = process.env.RETAKE_BREAK === '1';
function seeded(a: number) { return () => { a |= 0; a = (a + 0x6D2B79F5) | 0; let t = Math.imul(a ^ (a >>> 15), 1 | a); t = (t + Math.imul(t ^ (t >>> 7), 61 | t)) ^ t; return ((t ^ (t >>> 14)) >>> 0) / 4294967296; }; }
const dist = (a: number[], b: number[]) => Math.hypot(a[0] - b[0], a[2] - b[2]);

export async function retakeCheck(): Promise<boolean> {
  const shipped = process.env.TPS_PROFILE === 'shipped';
  console.log(`\n=== retake: a man back from the fight walks for where his order stands; a cover's row has no compass word (${shipped ? "the page's stand" : 'the frozen stand'})${BREAK ? ' — RETAKE_BREAK: the switch left off' : ''} ===`);
  let all = true; const line = (ok: boolean, label: string, detail = '') => { all &&= ok; console.log(`- ${ok ? 'PASS' : 'FAIL'} — ${label}${detail ? `\n    ${detail}` : ''}`); };
  const squad0 = NIGHTBREACH.squad, rnd0 = Math.random, knobs0 = knobsCopy(); let W: any;
  try {
    Object.assign(NIGHTBREACH, { squad: 4 }); Math.random = seeded(5); W = await standUp();
    // (1) the stand, read before the check sets anything
    { const on = ORDERS.retakeGoal === true, held = FROZEN_HOLDS['ORDERS.retakeGoal'];
      line(shipped ? on : !on && typeof held === 'string', shipped ? 'the page\'s stand has the rule on' : 'the frozen stand holds the rule off, and FROZEN_HOLDS names the word that raises it', `ORDERS.retakeGoal ${ORDERS.retakeGoal} · FROZEN_HOLDS: ${held ?? '(no row)'}`); }
    ORDERS.retakeGoal = !BREAK;
    W.game.godMode = true; W.game.roomClear.start(5);
    const g = W.game, O = g.orders, pl = g.player;
    const step = (n = 1) => { for (let i = 0; i < n; i++) { pl.visibility = 0.05; W.step(DT); } };
    step(30); for (const m of O.squad()) m.god = true;
    const team: any[] = O.squad(), s0 = pl.char.pos;
    const floor = (p: number[]) => { const t = O.targetAt(p); return t?.kind === 'floor' && !t.wall && !t.thing && dist(t.at, p) < 0.4; };
    const straight = (m: any, p: number[]) => { const r = O.route(m, p); return !r.why && r.via.length === 0; };

    // (2) the stale corner
    { let spot: number[] | null = null, corner: number[] | null = null;
      for (const [dx, dz] of [[3, 0], [-3, 0], [0, 3], [0, -3], [2.5, 2.5], [-2.5, -2.5]]) { const p = [s0[0] + dx, 0, s0[2] + dz], c = [s0[0] - dx * 1.2, 0, s0[2] - dz * 1.2];
        if (floor(p) && floor(c) && team.every(m => straight(m, p) && straight(m, c))) { spot = p; corner = c; break; } }
      if (!spot || !corner) line(false, 'a man back from the fight walks for where his order stands', 'no open floor either side of him to ask it on');
      else {
        O.give('move', O.targetAt(spot), 'all'); step(2);
        const As = team.map(m => O.of(m)), posts = As.map((A: any) => A?.post ? [...A.post] : null), ordered = As.every((A: any) => A?.order === 'move' && A.post);
        for (const A of As) if (A) { A.via = [[corner[0], 0, corner[2]]]; A.goal = [corner[0], 0, corner[2]]; }   // (the way an order round a room leaves him: a corner first)
        for (const m of team) endGuardScript(m, 'alert'); step(1);   /* (the seam a fight layer takes a man by: his script ended, his state the fight's) */ const yielded = As.every((A: any) => A && A.yieldT >= 0);
        let retaken = false; for (let i = 0; i < 60 * 8 && !retaken; i++) { step(1); retaken = As.every((A: any) => A && A.yieldT < 0); }   // (nobody to fight: it goes cold by itself, ORDERS.coldSecs)
        const via = As.map((A: any) => A?.via.length ?? -1), goalOn = As.map((A: any, i: number) => A && posts[i] ? dist(A.goal, posts[i]!) : 99);
        step(60 * 12);
        const off = team.map((m, i) => posts[i] ? dist(m.char.pos, posts[i]!) : 99), atCorner = team.filter(m => dist(m.char.pos, corner!) < 1.5).length, short = As.filter((A: any) => A?.short || A?.stopped).length;
        line(ordered && yielded && retaken && off.every(d => d < 1.2) && atCorner === 0,
          'a man back from the fight walks for where his order stands: with a corner left in his way from before and the way now straight, he ends on his place, not on the corner',
          `the move given ${ordered} · a corner ${dist(corner, spot).toFixed(1)} m the other side of him written into each man's way · the fight had them ${yielded}, and let go ${retaken} · back on the order: corners left in the way ${via.join(' / ')}, his goal off his place ${goalOn.map(d => d.toFixed(1)).join(' / ')} m · twelve seconds on: off his place ${off.map(d => d.toFixed(1)).join(' / ')} m, stood at the old corner ${atCorner}, marked short ${short}`);
        O.give('fallIn', O.targetAt(pl.char.pos), 'all'); step(240); } }

  } finally { Object.assign(NIGHTBREACH, { squad: squad0 }); Math.random = rnd0; knobsBack(knobs0); }
  console.log(`=== retake: ${all ? 'ALL PASS' : 'FAILURES above'} ===`);
  return all;
}
