// The 'walk-aim' probe: A MAN WALKING TO A STACK DOES NOT POINT HIS GUN THROUGH THE WALL BESIDE HIM. Run through probes.ts:
//   bun run tools/qa/probes.ts walk-aim        (family: squadorders)
// PASS / FAIL each, headless (bang-tally.ts stage()). The floor is emptied of hostiles so nothing but the order moves a gun, the team is told to fall in and
// then to stack on the nearest held room's door — the orders a player gives — and every frame the probe looks at each man who is ON THE WALK to his mark
// (his feet have carried him over the last sixth of a second — a man held at a door is not walking — and he is further from his mark than the last metre and
// a half, where he squares up to the door): where is his gun pointed? It casts its own line from his gun's height along the gun's heading and counts the
// man-frames it ends on something that stands inside arm's length (the weapon light's own near distance, guards.ts LIGHTS.wallM): a gun into the wall
// beside him. Two guns are where they should be and are not counted: one that points the way he is walking (the heading of his last steps: a man coming
// up to a corner looks at the corner) and one that is on the point his order gave him with a clear line to it (the door, a step away). What is left is a
// gun into a wall that is neither where he walks nor on anything he can see.
//   · on the walk a gun is not into a wall: no more than WALLED_MAX of a scenario's walking man-frames (a corner cut close sweeps a muzzle across a jamb)
// The fault this holds (11-04): an order hands a walking man its aim point, the door he is sent to, from his first step, and he carries his gun on it THROUGH
// whatever stands between: walking along a wall with the door beyond it, his muzzle is 0.2 to 1 m off the plaster for a third to a half of the walk — which
// is also why his weapon light is (rightly) dark there, LIGHTS.wallOff. He should look where he walks, or at what he can see.
// What it cannot say: what it looks like (never seen on a GPU), or whether a gun along the walk is where a fight wants it — that is playthrough.ts's tally.
import { ROOT } from './headless.ts';
import { stage, setKnobs } from './bang-tally.ts';
const GDM: any = await import(`${ROOT}/src/game/guards.ts`);
const CBM: any = await import(`${ROOT}/src/game/combat.ts`);

type Verdict = (ok: boolean, label: string, detail: string) => void;
function family(title: string): { verdict: Verdict; done: () => boolean } {
  console.log(`\n=== ${title} ===`); let all = true;
  return { verdict: (ok, label, detail) => { all &&= ok; console.log(`- ${ok ? 'PASS' : 'FAIL'} — ${label}\n    ${detail}`); }, done: () => { console.log(`=== ${title.split(':')[0]}: ${all ? 'ALL PASS' : 'FAILURES above'} ===`); return all; } };
}
const quiet = async <T>(fn: () => T | Promise<T>): Promise<T> => { const log = console.log, err = console.error, warn = console.warn; console.log = console.error = console.warn = () => { }; try { return await fn(); } finally { console.log = log; console.error = err; console.warn = warn; } };
const flat = (a: number[], b: number[]) => Math.hypot(a[0] - b[0], a[2] - b[2]);
/** the scenarios walked: three strongholds whose way to the nearest door runs along a wall, an office and a rescue */
export const WALK_IDS = ['compound-four', 'warehouse-four', 'killhouse-four', 'room-clear-four', 'rescue-registry'];
/** the share of a scenario's walking man-frames a gun may be into a wall */
const WALLED_MAX = 0.05;
/** the last stretch of a walk, where a man squares up to his door: not judged (m from his mark) */
const NEAR_MARK = 1.5;
const GUN_Y = 1.3;
/** on the walk: his feet have carried him this far (m) over the last MOVED_FRAMES frames */
const MOVED = 0.08, MOVED_FRAMES = 10;
/** a gun is ON a point when its heading is within this of the bearing to it (rad), and the point is seen when the line to SHORT of it is clear (m) */
const ON_POINT = 0.35, SHORT = 0.5;
const off = (a: number, b: number) => Math.abs(Math.atan2(Math.sin(a - b), Math.cos(a - b)));

export interface Walk { walk: number; walled: number; worst: string }
/** one scenario: the floor emptied, fall in, stack on the nearest held room's door; the walking man-frames and how many of them had the gun into a wall */
export async function walkToStack(id: string): Promise<Walk> {
  setKnobs({});
  const S: any = await quiet(() => stage(id, 1)); const { g, step } = S, O = g.orders, reach = GDM.LIGHTS.wallM;
  const cands: { ent: any; dist: number }[] = []; for (const d of S.defs) for (const e of d.room.entries) if (!cands.some(c => c.ent === e)) cands.push({ ent: e, dist: flat(g.player.char.pos, e.at) });
  cands.sort((a, b) => a.dist - b.dist);
  for (const x of g.guards) if (x.team === 'hostile' && x.state !== 'dead') await quiet(() => CBM.killGuard(g, x, [0, 0, 1], true));
  const team: any[] = O.squad().filter((x: any) => x.state !== 'dead'), sam = g.player.char.pos, per = new Map<any, number>(), trail = new Map<any, number[][]>(); let walk = 0, walled = 0;
  /** his gun is on the point his order gave him, and he has a line to it */
  const onSeenPoint = (x: any): boolean => { const s = x.script, aim = s?.over ? s.over.aimAt : s?.aimAt, ch = x.char; if (!aim) return false;
    const eye = [ch.pos[0], 1.45 - 0.45 * ch.anim.crouch, ch.pos[2]], d = [aim[0] - eye[0], aim[1] - eye[1], aim[2] - eye[2]], l = Math.hypot(d[0], d[1], d[2]), offP = ch.aimYaw - Math.atan2(d[0], d[2]);
    if (Math.abs(Math.atan2(Math.sin(offP), Math.cos(offP))) > ON_POINT) return false; const k = Math.max(0, (l - SHORT) / (l || 1));
    return !g.col.segmentBlocked(eye, [eye[0] + d[0] * k, eye[1] + d[1] * k, eye[2] + d[2] * k]); };
  step(60 * 6); await quiet(() => O.give('fallIn', O.targetAt([sam[0], 0, sam[2]]), 'all')); for (let f = 0; f < 60 * 12; f++) step();
  for (const c of cands) { if (await quiet(() => O.give('stack', O.targetAt(c.ent.door.frameCentre), 'all'))) continue;
    for (let f = 0; f < 60 * 30; f++) { step();
      for (const x of team) { const A = O.of(x), ch = x.char, tr = trail.get(x) ?? []; trail.set(x, tr); tr.push([ch.pos[0], 0, ch.pos[2]]); if (tr.length > MOVED_FRAMES + 1) tr.shift();
        if (!A?.post || A.arrived || flat(ch.pos, A.post) < NEAR_MARK || tr.length <= MOVED_FRAMES || flat(tr[0], ch.pos) < MOVED) continue;
        walk++; const o = [ch.pos[0], GUN_Y, ch.pos[2]];
        const along = off(ch.aimYaw, Math.atan2(ch.pos[0] - tr[0][0], ch.pos[2] - tr[0][2])) <= ON_POINT;   // (his gun is on the way his feet have just carried him)
        if (!along && g.col.segmentBlocked(o, [o[0] + Math.sin(ch.aimYaw) * reach, GUN_Y, o[2] + Math.cos(ch.aimYaw) * reach]) && !onSeenPoint(x)) { walled++; per.set(x, (per.get(x) ?? 0) + 1); } }
      if (team.every(x => O.of(x)?.arrived)) break; }
    break; }
  const worst = [...per.entries()].sort((a, b) => b[1] - a[1]).map(([x, n]) => `${x.callsign} ${n}`).join(', ');
  return { walk, walled, worst };
}

export async function walkAim(): Promise<boolean> {
  const F = family('walk-aim: a man walking to a stack does not point his gun through the wall beside him');
  try {
    const rows: Walk[] = []; for (const id of WALK_IDS) rows.push(await walkToStack(id));
    const bad = rows.filter(r => r.walled > r.walk * WALLED_MAX).length, thin = rows.filter(r => r.walk < 300).length;
    F.verdict(bad === 0 && thin === 0, `on the walk to a stack a gun is not into a wall: at most ${(WALLED_MAX * 100).toFixed(0)} in 100 walking man-frames of a scenario have something standing inside ${GDM.LIGHTS.wallM.toFixed(1)} m of the gun, along its heading, with the gun neither on the way he walks nor on the point his order gave him in plain sight`,
      WALK_IDS.map((id, i) => `${id} ${rows[i].walled} of ${rows[i].walk}${rows[i].walled ? ` (${rows[i].worst})` : ''}`).join(' · ') + ` · scenarios over the bar: ${bad} of ${rows.length}${thin ? ` · ${thin} with under 300 walking man-frames: nothing to judge` : ''}`);
    return F.done();
  } catch (e) { console.log(`- FAIL — the probe threw\n    ${e instanceof Error ? e.stack : String(e)}`); return false; }
}
