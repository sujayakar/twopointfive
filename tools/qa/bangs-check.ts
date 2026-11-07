// A BANG AND CLEAR'S CAN IS THROWN INTO THE ROOM — the check behind DOORTOOLS.bangInto (src/game/doortools.ts bangSpot and the 'toss' / 'fuse' phases; a
// settled rule, as DOORTOOLS.wordNoCan is: what the word then DOES is asked in `probes.ts threshold-hold`).
//   bun run tools/qa/bangs-check.ts        (also `probes.ts bangs`)
// The maintainer's own case (10-16, with a tape of it: "with bang and clear for a door, they often don't really throw the flash bang into the room"): on
// his tape the thrower stood two metres down the wall, the can appeared in the door frame and went off in the air 1.75 m inside a third of a second later.
// Staged on the block, the squad of four set down outside a shut door, anybody who sees the stack taken off the floor, 'bang and clear' given:
//   (1) a door with a deep room behind it: the can first shows at the thrower's own hand, is passed to the door's edge, and goes off chest high inside the
//       room, where the toss said it would; nobody of ours is dazzled;
//   (2) a second door: thrown into the room there too (1.5 m or more inside, at the point named), from the hand;
// DELETED 2026-11-07 as a line that asked a build from before a settled rule: '(3) a man of the team who is not on the order, standing in the room where the
// can went: the pair still throws, and not at him' (the check held DOORTOOLS.tossMate off to ask it; that rule, settled, keeps the can in the kit when one of
// ours is in its reach with a line from the opening: onesignal-check.ts asks that).
// NOTHING HERE WRITES A SETTLED ROW (2026-11-07). Out: the two stands' rows, 'the rule off: the fault as he met it' and 'a metre or more deeper than it did'
// (the same door played with the rule off), the recordings from before each key and the page's boot flags, and two pins that were refused (the team's
// known face and cleared rooms held off round the order).
// BANGINTO_BREAK=1, the red path: the order given is 'open and clear', so no can leaves a hand and every line must fail (it was: the switch left off).
// Headless: lines are the collision's own. Never seen on a GPU; nobody has played it.
import { standUp, ROOT } from './headless.ts';
const BT: any = await import(`${ROOT}/tools/qa/bang-tally.ts`);
const { DOORTOOLS, doorPt, alongOf, offOf } = await import(`${ROOT}/src/game/doortools.ts`);
const { killGuard } = await import(`${ROOT}/src/game/combat.ts`);
type Vec3 = [number, number, number];

/** the block's doors the tally showed the most of: a hall door with a deep room behind it, and one the can is passed along the wall to */
const SCN = 'block-four', DEEP: [string, 1 | -1] = ['b_s', 1], SECOND: [string, 1 | -1] = ['s_n', -1];

export async function bangsCheck(): Promise<boolean> {
  const BREAK = process.env.BANGINTO_BREAK === '1', ROW = BREAK ? 'openClear' : 'bangClear';
  console.log(`\n=== bangs: a bang and clear's can is thrown into the room, from the hand${BREAK ? ' — BANGINTO_BREAK: open and clear given, no can' : ''} ===`);
  let all = true; const line = (ok: boolean, label: string, detail = '') => { all &&= ok; console.log(`- ${ok ? 'PASS' : 'FAIL'} — ${label}${detail ? `\n    ${detail}` : ''}`); };
  const rnd0 = Math.random;
  const log = console.log, quiet = async <T>(fn: () => T | Promise<T>): Promise<T> => { console.log = () => { }; try { return await fn(); } finally { console.log = log; } };
  const flat = (a: number[], b: number[]) => Math.hypot(a[0] - b[0], a[2] - b[2]), f1 = (x: number) => x.toFixed(1);
  try {
    /** one bang and clear on a shut door, watched to its burst */
    const run = async (name: string, side: 1 | -1, who: 'all' | 'red' = 'all', mateAt: number[] | null = null) => {
      const S: any = await quiet(() => BT.stage(SCN, 1)); const { g, step } = S;
      const O = g.orders, D = g.doorTools, nav = g.col.nav, d = g.doors.list.find((x: any) => x.def.name === name), W = d.def.width;
      const free = (p: number[]): number[] | null => { const q = nav.isBlocked(p[0], p[2]) ? nav.nearestFreePoint(p[0], p[2]) : p; return q ? [q[0], 0, q[2]] : null; };
      const gate = doorPt(d, W / 2, 0.7, side), clearTo = (p: number[]) => !g.col.solidBlocked([p[0], 1.1, p[2]], [gate[0], 1.1, gate[2]]);
      const team: any[] = O.squad().filter((x: any) => x.state !== 'dead'), spots: number[][] = [];
      for (const back of [3.2, 4.2, 2.4, 5.2, 6.2]) for (const lat of [1.1, -1.1, 0.4, -0.4, 1.9, -1.9]) { const p = free(doorPt(d, W / 2 + lat, back, side)); if (p && offOf(d, p, side) > 0.9 && clearTo(p) && flat(p, d.frameCentre) > 1.8 && !spots.some(s => flat(s, p) < 0.8)) spots.push(p); }
      { const far = [...spots].sort((x, y) => flat(y, d.frameCentre) - flat(x, d.frameCentre))[0]; spots.splice(spots.indexOf(far), 1); spots.splice(Math.min(team.length, spots.length), 0, far); }   // (Sam on the farthest spot: out of the stack's way)
      if (spots.length < team.length + 1) return null;
      const yawIn = Math.atan2(d.frameCentre[0] - spots[0][0], d.frameCentre[2] - spots[0][2]);
      team.forEach((x, i) => { const p = spots[i]; x.char.pos[0] = p[0]; x.char.pos[2] = p[2]; x.char.vel = [0, 0, 0]; x.path = []; x.pathGoal = null; x.speed = 0; x.char.bodyYaw = x.char.aimYaw = yawIn; });
      await quiet(() => g.teleportPlayer(spots[team.length] as any));
      d.angle = 0; d.vel = 0; d.latched = true; d.closing = false; d.place();
      for (const x of g.guards) { if (x.state === 'dead' || x.team !== 'hostile') continue;   // (the entry is counted, not the building: whoever sees the stack gather is taken off the floor)
        const c = [x.char.pos[0], 1.5, x.char.pos[2]]; if ([...spots.slice(0, team.length + 1), gate].some(p => !g.col.solidBlocked(c as any, [p[0], 1.3, p[2]]))) await quiet(() => killGuard(g, x, [0, 0, 1], true)); }
      step(10);
      const mate: any = mateAt ? team.find(x => /^Blue/.test(x.callsign)) : null;
      const t0 = g.time, dz0 = new Map<any, number>(g.guards.map((x: any) => [x, x.dazzledUntil ?? 0]));
      const why: any = await quiet(() => O.give(ROW, { kind: 'door', at: [d.frameCentre[0], 0, d.frameCentre[2]], door: d, side }, who));
      if (why) return { why: String(why) } as any;
      let op: any = null, can: any = null, out: any = { why: null, beats: [] as string[] }, nb = 0, passTo: number[] | null = null;
      for (let i = 0; i < 60 * 40; i++) { step();
        op ??= [...D.ops, ...D.done].find((o: any) => o.door === d && o.kind === 'bangClear');   // (BANGINTO_BREAK: never one)
        if (!op) continue;
        // (5): a man of the team who is NOT on the order walks into the room once it is given (given with him in there it is refused in words: 'behind that door'), to where the can went the time before
        if (mate && !out.mate) { out.mate = mate.callsign; mate.char.pos[0] = mateAt![0]; mate.char.pos[2] = mateAt![2]; mate.path = []; mate.pathGoal = null; await quiet(() => O.give('hold', { kind: 'point', at: [mateAt![0], 0, mateAt![2]] }, [mate])); }
        while (nb < op.beats.length) { const b = op.beats[nb++]; out.beats.push(`${(b.t - t0).toFixed(1)} ${b.what}`); if (/^toss:/.test(b.what)) { out.toss = `${b.what}${out.into ? ` · ${out.into}` : ''}`; out.tToss = b.t; } else if (/^can into the room/.test(b.what)) out.into = b.what; }
        if (!can && op.can) { can = op.can; const th = op.threw.char.pos; out.thrower = op.threw.callsign; out.handM = flat(th, can.pos); out.aim = op.aim ? { in: -offOf(d, op.aim.at, op.side), at: op.aim.at } : null; out.passed = !!op.pass; out.opSide = op.side;
          passTo = op.pass ? [...op.pass.to] : null; out.passMin = passTo ? flat(can.pos, passTo) : null;
          if (mate) { const m = mate.char.pos; out.mateIn = -offOf(d, m, op.side); out.mateM = out.aim ? flat(out.aim.at, m) : null; out.mateLine = out.aim ? flat(out.aim.at, m) <= DOORTOOLS.thrFlashM && !g.col.segmentBlocked(out.aim.at, [m[0], 1.45, m[2]]) : false; } }
        if (can && passTo && op.pass) out.passMin = Math.min(out.passMin, flat(can.pos, passTo));   // (the pass, while it is one: how near the door's edge the can FLEW before it was turned in there)
        if (can && can.fired) { const p = op.bangAt ?? can.pos; out.in = -offOf(d, p, op.side); out.y = p[1]; out.at = [p[0], p[1], p[2]]; out.flight = g.time - out.tToss; step(20);
          out.own = team.filter(x => (x.dazzledUntil ?? 0) > (dz0.get(x) ?? 0) + 1e-6).length; break; } }
      return out;
    };
    const say = (r: any) => !r ? 'no floor for the team at that door' : r.why ? `refused — ${r.why}` : r.in === undefined ? `no burst: ${r.beats.join(' → ')}` : `${r.thrower} threw · the can first shows ${f1(r.handM)} m from him${r.passed ? ` (passed to the door: flew to ${r.passMin.toFixed(2)} m of its edge)` : ''} · burst ${f1(r.in)} m inside at ${f1(r.y)} m up, ${r.flight.toFixed(2)} s after the toss · ours dazzled ${r.own} · ${r.toss}`;

    // (1) a door with a deep room behind it
    const on = await run(DEEP[0], DEEP[1]);
    line(!!on && !on.why && on.handM < 0.6 && on.passed && on.passMin < 0.3 && !!on.aim && Math.abs(on.in - on.aim.in) < 0.4 && flat(on.at, on.aim.at) < 0.4 && on.y > 0.9 && on.y < 1.4 && on.own === 0,
      `${DEEP[0]}: the can first shows at the thrower's own hand, is passed to the door's edge, and goes off chest high inside the room, where the toss said it would; nobody of ours dazzled`, say(on) + (on?.aim ? ` · aimed ${f1(on.aim.in)} m in` : ' · no aim'));
    // (2) a second door
    const two = await run(SECOND[0], SECOND[1]);
    line(!!two && !two.why && !!two.aim && two.in >= DOORTOOLS.bangIntoMin - 0.1 && flat(two.at, two.aim.at) < 0.4 && two.handM < 0.6 + (two.passed ? 0 : DOORTOOLS.bangHandM) && two.own === 0,
      `${SECOND[0]}: thrown into the room there too — ${DOORTOOLS.bangIntoMin} m or more inside, at the point named, from the hand (or from the door's edge by a man who stands at it)`, say(two) + (two?.aim ? ` · aimed ${f1(two.aim.in)} m in` : ' · no aim'));
  } finally { console.log = log; Math.random = rnd0; }
  console.log(`=== bangs: ${all ? 'ALL PASS' : 'FAILURES above'} ===`);
  return all;
}
if (import.meta.main) process.exit((await bangsCheck()) ? 0 : 1);
