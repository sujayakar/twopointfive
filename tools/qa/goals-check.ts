// SHIFT + THE KEY ADDS A GOAL, AND THERE IS NO PLAN MODE (src/game/planmode.ts planGoal, CMDRING.goals — a PROTOTYPE). The maintainer, 10-26: "could we
// prototype removing plan mode and having shift Q instead add a goal to the squad? like shift Q on a second door will naturally split the squad in two,
// and F tells all of them to go in. shift Q on a spot to watch automatically has them split up angles".
//   bun run tools/qa/goals-check.ts            (also `probes.ts goals`, in family squadorders)
//   TPS_PROFILE=shipped bun run tools/qa/goals-check.ts      — the same lines on the page's stand
// … AND A TAP ON A SPOT IS WHAT THE KEY GIVES THERE (the maintainer, 11-08: "i think shift Q on a point should just move the squad there, should match what
// regular Q does. you should need to open the radial menu to get covering a point"): a move, to a part of the squad; covering a spot is a wedge of the ring.
// PASS / FAIL each, headless: the relay post (levels/tut-queue.json, the queueing tutorial's floor), the room clear with the squad of four under
// god mode, ONE deal (seed 1: the rule has no die in it). The check raises the ring and the rule itself, so both stands ask the
// same lines. The keys are pressed as a hand presses them: Shift down, the squad's key, the cursor on the thing.
//   1. Shift + the key on a door, nobody on anything: all four stack on that door (the first goal is everyone's).
//   2. Shift + the key on the room's second door: one whole pair is sent to stack there, and the other pair is still stacked on the first.
//   3. both stacked, ONE press of the word's key: an entry starts on each door, the two on one word of the door tools, and men are through both doors
//      within two seconds of each other.
//   4. the key WITHOUT Shift on the first door, after a split: everyone is on that door again — one goal.
//   5. Shift + the key on a spot of floor, nobody on anything: all four MOVE there and nobody covers. On a second spot: two and two, each a whole pair. On a third: two, one and one.
//   6. every group down to one man: Shift + the key on one more spot gives nothing, and the squad says so.
//   7. Shift + the key KEPT DOWN with nobody on anything is still everyone's ring: it comes up, and no goal is added by it.
//   8. the key with Shift and the key without it, on one spot: the same order (move), and nobody covers it.
// … AND KEPT DOWN IT IS THE RING FOR THAT PART (CMDRING.goalRing, orders.ts ringUp; the maintainer, 10-26, having played the first nine: "okay this
// feels good. let's make shift-Q long press open the radial menu to append another order too"):
//  10. all four stacked on one door, Shift + the key KEPT DOWN on the second: the ring comes up for one whole pair, named on it, and nobody is told
//      anything yet. Its first wedge given: that pair is sent to stack on the second door and the other pair is still stacked on the first.
//  11. all four sent to a spot, Shift + the key kept down on a second spot and 'cover' picked: that pair covers it, the only way a spot is covered, and the
//      other pair is still sent. Kept down on a third and the first wedge picked: ONE man is sent there, as the tap would have sent him, and the rest keep theirs.
//  12. every group down to one man: Shift + the key kept down on one more spot puts no ring up and gives nothing.
// GOALS_BREAK=1 = the hand leaves Shift off where a line presses it with the key (the switch itself is a settled row, tools/qa/fold.ts, and a write to it is refused: this is what it
// was off), the same lines asked: 2 to 6 and 10 to 12 must fail. (1 and 8 hold: with nothing going the first goal is everyone's here, and two plain keys give one order. 7 presses its own Shift.)
// GOALRING_BREAK=1 = the hand leaves Shift off only where the key is kept down: 10 to 12 must fail.
//   exit 0 = all hold · 1 = one does not
import { readFileSync } from 'node:fs';
import { ROOT, standUp } from './headless.ts';
import { ringDown } from './squadwords';
const { parseLevelData } = await import(`${ROOT}/src/scene/levelData.ts`);
const { loadLevel } = await import(`${ROOT}/src/scene/levelLoad.ts`);
const { startFromQuery } = await import(`${ROOT}/src/game/scenarioRun.ts`);
const { CMDRING } = await import(`${ROOT}/src/game/cmdring.ts`);
const PM: any = await import(`${ROOT}/src/game/planmode.ts`);
const { AUTOPILOT } = await import(`${ROOT}/src/game/autopilot.ts`);
const { DOORTOOLS, offOf } = await import(`${ROOT}/src/game/doortools.ts`);
const { knobsCopy, knobsBack } = await import(`${ROOT}/src/game/knobs.ts`);
const VERBOSE = process.argv.includes('--verbose'), BREAK = process.env.GOALS_BREAK === '1', RBREAK = process.env.GOALRING_BREAK === '1', LEVEL = 'tut-queue', DT = 1 / 60, SEED = 1;
type V = number[];
/** … and the whole ring table as the stand stood it (halftwice-check.ts reads its own two keys off it) */
export let stoodCmd: Record<string, unknown> | null = null;
const flat = (a: V, b: V) => Math.hypot(a[0] - b[0], a[2] - b[2]);

/** `more`: later switches of the ring a check of its own raises (halftwice-check.ts); `squad`: the address word for who comes */
export async function rig(on: boolean, more: Record<string, unknown> = {}, squad = 'squad4') {
  const log = console.log, warn = console.warn, err = console.error; if (!VERBOSE) console.log = console.warn = console.error = () => { };
  let W: any, started: any;
  try {
    const data: any = parseLevelData(readFileSync(`${ROOT}/levels/${LEVEL}.json`, 'utf8')); if (data.scenarios?.lesson) delete data.scenarios.lesson.rails;   // (this floor's lesson off its rails: these lines ask the goals, whichever step a lesson has them from)
    const level = loadLevel(data);
    W = await standUp({ uploadEvery: 4, level, seed: SEED }); stoodCmd ??= { ...CMDRING };
    // (the ring and its later switches as the page has them)
    // (… and the two after them held off: these lines ask the rule as it was built — with nothing going the first goal is everyone's, and a second tap
    //  is the key's order again. CMDRING.goalHalf and CMDRING.twice have their own lines in halftwice-check.ts)
    Object.assign(CMDRING, { on: true, tapNow: true, stay: true, wholeSquad: true, goals: on, goalRing: on && !RBREAK, goalHalf: false, twice: false, ...more }); AUTOPILOT.on = true;
    started = startFromQuery(W.game, `?levelFile=${LEVEL}.json&roomclear=1&${squad}`);
  } finally { console.log = log; console.warn = warn; console.error = err; }
  const g = W.game, O = g.orders, D = g.doorTools, I = W.input, KEY = CMDRING.key;
  const quiet = <T,>(f: () => T): T => { if (VERBOSE) return f(); const l = console.log, w = console.warn; console.log = console.warn = () => { }; try { return f(); } finally { console.log = l; console.warn = w; } };
  const R: any = { W, g, O, D, I, why: started?.why as string | undefined, post: null as null | (() => void),
    step(n = 1) { quiet(() => { for (let i = 0; i < n; i++) { g.player.visibility = 0.05; W.step(DT); R.post?.(); } }); },
    until(pred: () => boolean, secs: number) { for (let i = 0; i < secs * 60 && !pred(); i++) R.step(); return pred(); },
    door(id: string) { return g.doors.list.find((x: any) => x.def.name === id) ?? null; },
    /** the cursor put where the squad's key reads that door (null: nowhere it does) */
    onDoor(d: any): boolean { const c = d.frameCentre, th = d.def.closedDir, nx = -Math.sin(th), nz = Math.cos(th);
      for (const y of [c[1], 0.05, 1.0, 1.8]) for (const k of [0, 0.35, -0.35]) { W.cursorAt([c[0] + nx * k, y, c[2] + nz * k]); R.step(2); const t = O.targetUnder(I, W.cam, W.canvas); if (t.kind === 'door' && t.door === d) return true; }
      return false; },
    /** … and on a bare spot of floor the squad can see from where it stands, clear of `skip` (null: none) */
    onSpot(skip: V[] = []): V | null { const s = g.player.char.pos;
      for (const [dx, dz] of [[3, 0], [-3, 0], [0, 3], [0, -3], [2.5, 2.5], [-2.5, -2.5], [2.5, -2.5], [-2.5, 2.5], [4.5, 0], [0, 4.5], [-4.5, 0], [0, -4.5], [4, 2], [-4, 2], [4, -2], [-4, -2]]) { const p = [s[0] + dx, 0.04, s[2] + dz]; if (skip.some(q => flat(q, p) < 2)) continue;
        W.cursorAt(p); R.step(2); const t = O.targetUnder(I, W.cam, W.canvas);
        if (t.kind === 'floor' && !t.thing && O.men('all').every((m: any) => flat(m.char.pos, t.at) > 1.2 && O.hasLine(m, t))) return [t.at[0], 0, t.at[2]]; }
      return null; },
    /** the squad's key where the cursor is, Shift held or not, as a hand presses it */
    key(shift0: boolean) { const shift = shift0 && !BREAK; if (shift) I.keys.add('ShiftLeft'); I.keys.add(KEY); I.pressed.add(KEY); R.step(); I.keys.delete(KEY); R.step(2); if (shift) { I.keys.delete('ShiftLeft'); R.step(); } },
    /** Shift and the squad's key kept down where the cursor is, until the ring is up (false: none came) */
    hold(): boolean { if (!BREAK && !RBREAK) I.keys.add('ShiftLeft'); I.keys.add(KEY); I.pressed.add(KEY); return R.until(() => !!O.cmdRing, 1.5); },
    /** … a wedge of it given by its digit — the first, or the one whose words fit — and every key let go (its words; null: no such wedge, the ring put away) */
    wedge(w: number | RegExp): string | null { const V = O.g.view.ring, x = (V?.wedges ?? []).find((y: any) => typeof w === 'number' ? y.key === w : w.test(y.text));
      if (!x) { R.away(); return null; } I.pressed.add(`Digit${x.key}`); R.step(); I.keys.clear(); R.step(4); if (O.cmdRing) R.away(); return x.text; },
    away() { I.keys.clear(); ringDown(I); R.step(4); },
    /** all four sent to stack on that door (true: they stand on their marks) */
    stackAll(d: any): boolean { O.give('stack', O.forMen({ kind: 'door', at: [d.frameCentre[0], 0, d.frameCentre[2]], door: d, side: 1 }, O.men('all')), 'all'); return R.until(() => R.onStack(d).length === O.men('all').length && R.set(R.onStack(d)), 40); },
    /** who is on that door by a plain stack, and whether they all stand on their marks */
    onStack(d: any): any[] { return O.men('all').filter((m: any) => O.of(m)?.order === 'stack' && O.of(m)?.door === d); },
    set(men: any[]) { return men.length > 0 && men.every((m: any) => O.of(m)?.arrived); },
    /** everything put away and the men back round Sam */
    home() { I.keys.clear(); I.buttons = 0; ringDown(I); R.step(); O.who = 'all';
      O.give('fallIn', O.targetAt(g.player.char.pos), 'all'); R.until(() => D.ops.length === 0, 40); R.until(() => O.men('all').every((m: any) => flat(m.char.pos, g.player.char.pos) < 5), 30); R.step(30); },
    /** how the men are parted: the size of each group on an order (an order given at once is one group), biggest first */
    parts(): string { const by = new Map<number, number>(); for (const m of O.men('all')) { const A = O.of(m); if (A && A.post !== null) by.set(A.group, (by.get(A.group) ?? 0) + 1); } return [...by.values()].sort((x, y) => y - x).join(' + ') || 'nobody on anything'; },
    /** the men on that order */
    onOrder(id: string): any[] { return O.men('all').filter((m: any) => O.of(m)?.order === id); },
    /** the men covering, by the point each watches */
    watches(): Map<string, string[]> { const out = new Map<string, string[]>(); for (const m of O.men('all')) { const A = O.of(m); if (A?.order !== 'cover' || !A.watch) continue; const k = `${A.watch[0].toFixed(1)},${A.watch[2].toFixed(1)}`; out.set(k, [...(out.get(k) ?? []), m.callsign]); } return out; },
  };
  g.godMode = true; R.step(30); for (const m of O.squad()) m.god = true;
  return R;
}

export async function goalsCheck(): Promise<boolean> {
  let fails = 0;
  const line = (ok: boolean, what: string, got: string) => { console.log(`  ${ok ? 'PASS' : 'FAIL'}  ${what} — ${got}`); if (!ok) fails++; };
  const knobs0 = knobsCopy(), K = CMDRING;
  console.log(`Shift + the key adds a goal (src/game/planmode.ts planGoal, CMDRING.goals; kept down, the ring for that part: CMDRING.goalRing)${BREAK ? ' — GOALS_BREAK=1: no Shift in the hand, lines 2 to 6 and 10 to 12 must fail' : RBREAK ? ' — GOALRING_BREAK=1: no Shift with the key kept down, lines 10 to 12 must fail' : ''}`);
  try {
    const R = await rig(!BREAK), g = R.g, O = R.O, D = R.D, N = O.men('all').length;
    const A = R.door('guard'), B = R.door('guard_side'), names = (l: any[]) => l.map((m: any) => m.callsign).sort().join(', ') || 'nobody';
    if (R.why || !A || !B || N !== 4) { line(false, 'the room clear on the queueing tutorial\'s level, four men and the guardroom\'s two doors', R.why ?? `${N} men · doors ${A ? 'guard' : '-'} ${B ? 'guard_side' : '-'}`); return false; }

    // 1
    R.home();
    { const onA = R.onDoor(A); R.key(true); const to = R.onStack(A);
      line(onA && to.length === N, '1. Shift + the key on a door, nobody on anything: all four stack on that door',
        `the cursor on the door: ${onA} · sent to stack on it: ${names(to)}`); }
    // 2
    { O.give('stack', { kind: 'door', at: [A.frameCentre[0], 0, A.frameCentre[2]], door: A, side: O.sideFor ? O.forMen({ kind: 'door', at: A.frameCentre, door: A, side: 1 }, O.men('all')).side : 1 }, 'all'); R.until(() => R.set(R.onStack(A)) && R.onStack(A).length === N, 40);
      const all = R.onStack(A).length, onB = R.onDoor(B); R.key(true); const a = R.onStack(A), b = R.onStack(B), pair = b.length === 2 && O.pairOf(b[0]) === O.pairOf(b[1]);
      line(all === N && onB && a.length === 2 && pair && PM.PLAN_STATS.split > 0, '2. Shift + the key on the second door: one whole pair is sent to stack there, and the other pair is still stacked on the first',
        `all four stacked first: ${all === N} · the cursor on the second door: ${onB} · on the first: ${names(a)} · on the second: ${names(b)}${pair ? ` (${O.pairOf(b[0])})` : ''}`); }
    // 3
    { const both = R.until(() => R.set(R.onStack(A)) && R.set(R.onStack(B)), 60); R.step(30);
      const had = new Set<any>(D.ops), t0 = g.time, fresh = () => D.ops.filter((o: any) => !had.has(o)), kept: any[] = [], crossT = new Map<any, number>();
      const words0 = D.stats.words; R.I.pressed.add(DOORTOOLS.blowKey); R.step();
      // (one word of the door tools: its count went up by one, and each entry carries its beat — the word itself is taken off them again as they step in)
      const e = fresh().filter((o: any) => o.kind === 'openClear' || o.kind === 'bangClear'), worded = e.length === 2 && D.stats.words - words0 === 1 && e.every((o: any) => o.beats.some((b: any) => b.what === 'the word')), doors = new Set(e.map((o: any) => o.door));
      R.post = () => { for (const o of fresh()) { if (!kept.includes(o)) kept.push(o); if (!crossT.has(o) && o.men.some((m: any) => m.state !== 'dead' && offOf(o.door, m.char.pos, o.side) < -0.15)) crossT.set(o, g.time - t0); } };
      R.post(); R.until(() => kept.length >= 2 && kept.every(o => crossT.has(o)), 30); R.post = null;
      const cross = e.map((o: any) => crossT.get(o) ?? NaN), gap = Math.abs(cross[0] - cross[1]);
      line(both && worded && doors.has(A) && doors.has(B) && gap <= 2, '3. both stacked, ONE press of the word\'s key: an entry on each door, the two on one word, and men through both doors within two seconds of each other',
        `both stacks set: ${both} · entries the press started: ${e.map((o: any) => `${o.kind} ${o.door.def.name}`).join(' + ') || 'none'} · on one word: ${worded} · first man through each after ${cross.map((c: number) => isNaN(c) ? 'never' : `${c.toFixed(2)} s`).join(' and ')}`);
      R.until(() => D.ops.length === 0, 60); }
    // 4
    R.home();
    { R.onDoor(A); R.key(true); R.step(20); R.onDoor(B); R.key(true); R.step(20); const split = R.onStack(A).length === 2 && R.onStack(B).length === 2;
      const onA = R.onDoor(A); R.key(false); R.step(20); const a = R.onStack(A), b = R.onStack(B), inA = D.ops.filter((o: any) => o.door === A).flatMap((o: any) => o.men).length;
      line(split && onA && b.length === 0 && (a.length === N || inA === N), '4. the key WITHOUT Shift on the first door, after a split: everyone is on that door again',
        `split two and two first: ${split} · after the plain key: on the first ${a.length || inA} of ${N}, on the second ${names(b)}`); }
    // 5, 6
    R.home();
    { const spots: V[] = [], add = (): string => { const p = R.onSpot(spots); if (!p) return 'no spot'; spots.push(p); R.key(true); R.step(10); return R.parts(); };
      const pairs = (): boolean => { const by = new Map<number, any[]>(); for (const m of O.men('all')) { const G = O.of(m)?.group; by.set(G, [...(by.get(G) ?? []), m]); } return [...by.values()].every(l => l.length === 2 && O.pairOf(l[0]) === O.pairOf(l[1])); };
      const s1 = add(), m1 = R.onOrder('move').length, c1 = R.onOrder('cover').length, s2 = add(), whole = pairs(), s3 = add(), m3 = R.onOrder('move').length, c3 = R.onOrder('cover').length;
      line(s1 === '4' && m1 === N && c1 === 0 && s2 === '2 + 2' && whole && s3 === '2 + 1 + 1' && m3 === N && c3 === 0, '5. Shift + the key on a spot, nobody on anything: all four move there and nobody covers. A second spot: two and two, each a whole pair. A third: two, one and one',
        `after the first: ${s1} (moving ${m1}, covering ${c1}) · the second: ${s2}, whole pairs ${whole} · the third: ${s3} (moving ${m3}, covering ${c3})`);
      const s4 = add(), n0 = O.log.length, g0 = PM.PLAN_STATS.goals, s5 = add(), gave = O.log.length - n0, asked = PM.PLAN_STATS.goals - g0;
      line(s4 === '1 + 1 + 1 + 1' && s5 === s4 && gave === 0 && asked === 1, '6. every group down to one man: Shift + the key on one more spot gives nothing',
        `after the fourth: ${s4} · after a fifth: ${s5} · orders it gave: ${gave} · the key read as a goal: ${asked === 1} · the squad's words for it: "${PM.GOAL_NONE}"`); }
    // 8 (the same spot, from the same place: the plain key, then all put back and the key with Shift)
    R.home();
    { const p = R.onSpot(); W8: { if (!p) { line(false, '8. the key with Shift and without it on one spot: the same order', 'no spot'); break W8; }
        R.W.cursorAt([p[0], 0.04, p[2]]); R.step(2); const n0 = O.log.length; R.key(false); R.step(4); const plain = O.log.length > n0 ? O.log[O.log.length - 1].order : 'nothing', cp = R.onOrder('cover').length;
        R.home(); R.W.cursorAt([p[0], 0.04, p[2]]); R.step(2); const n1 = O.log.length; R.key(true); R.step(4); const shifted = O.log.length > n1 ? O.log[O.log.length - 1].order : 'nothing', cs = R.onOrder('cover').length;
        line(plain === 'move' && shifted === plain && cp === 0 && cs === 0, '8. the key with Shift and the key without it, on one spot: the same order, and nobody covers it', `the plain key: ${plain} (covering ${cp}) · with Shift: ${shifted} (covering ${cs})`); } }
    // 7
    R.home();
    { const g0 = PM.PLAN_STATS.goals, n0 = O.log.length, onA = R.onDoor(A); R.I.keys.add('ShiftLeft'); R.I.keys.add(K.key); R.I.pressed.add(K.key); const ring = R.until(() => !!O.cmdRing, 1.5), whole = !O.menu?.part;
      R.I.keys.clear(); ringDown(R.I); R.step(4); const asked = PM.PLAN_STATS.goals - g0, gave = O.log.length - n0;
      line(onA && ring && whole && asked === 0 && gave === 0, '7. Shift + the key KEPT DOWN with nobody on anything is still everyone\'s ring: it comes up, and no goal is added by it',
        `the ring ${ring ? 'up' : 'NOT up'} · goals added ${asked} · orders given ${gave}`); }
    // 10 (the ring for a part: the switches as this run has them)
    R.home(); K.goals = !BREAK; K.goalRing = !BREAK && !RBREAK;
    { const all = R.stackAll(A), onB = R.onDoor(B), n0 = O.log.length, g0 = PM.PLAN_STATS.goals, r0 = PM.PLAN_STATS.rings;
      const ring = R.hold(), part: any[] = O.menu?.part ?? [], cap = String(O.g.view.ring?.caption ?? ''), told = O.log.length - n0;
      const pair = part.length === 2 && O.pairOf(part[0]) === O.pairOf(part[1]), named = pair && cap.startsWith(String(O.pairOf(part[0])));
      const text = ring ? R.wedge(1) : null; if (!ring) R.away(); R.step(20); const a = R.onStack(A), b = R.onStack(B), theirs = pair && names(b) === names(part);
      line(all && onB && ring && pair && named && told === 0 && PM.PLAN_STATS.rings - r0 === 1 && theirs && a.length === 2 && PM.PLAN_STATS.goals - g0 === 1,
        '10. all four on one door, Shift + the key KEPT DOWN on the second: the ring is one whole pair\'s, named on it. Its first wedge: that pair stacks there, the other pair stays',
        `all four stacked first: ${all} · the ring ${ring ? 'up' : 'NOT up'}, for ${names(part)}, under it '${cap}', orders while it was up ${told} · the wedge '${text}' · on the first: ${names(a)} · on the second: ${names(b)}`); }
    // 11
    R.home();
    { const spots: V[] = [];
      const spot = (): boolean => { const p = R.onSpot(spots); if (p) spots.push(p); return !!p; };
      spot(); R.key(true); R.step(10); const s1 = R.parts(), c1 = R.onOrder('cover').length;
      const on2 = spot(), ring2 = on2 && R.hold(), part2: any[] = [...(O.menu?.part ?? [])], w2 = ring2 ? R.wedge(/cover/i) : null; if (!ring2) R.away(); R.step(10); const s2 = R.parts(), cov = R.onOrder('cover'), moving = R.onOrder('move').length, theirs = names(cov) === names(part2);
      const others0 = new Map<any, string>(O.men('all').map((m: any) => [m, O.of(m)?.order])); const on3 = spot(), ring3 = on3 && R.hold(), one: any[] = [...(O.menu?.part ?? [])], w3 = ring3 ? R.wedge(1) : null; if (!ring3) R.away(); R.step(10); const s3 = R.parts(), went = one.length === 1 ? O.of(one[0])?.order : 'nobody'; const rest3 = O.men('all').filter((m: any) => !one.includes(m)), kept3 = rest3.length === 3 && rest3.every((m: any) => O.of(m)?.order === others0.get(m)), by3 = rest3.map((m: any) => `${m.callsign} ${O.of(m)?.order}`).join(', '), cov3 = rest3.filter((m: any) => O.of(m)?.order === 'cover').length, mov3 = rest3.filter((m: any) => O.of(m)?.order === 'move').length;
      line(s1 === '4' && c1 === 0 && part2.length === 2 && !!w2 && s2 === '2 + 2' && theirs && moving === 2 && one.length === 1 && s3 === '2 + 1 + 1' && went === 'move' && kept3 && cov3 + mov3 === 3,
        '11. all four sent to a spot, the ring kept down on a second and \'cover\' picked: that pair covers it and the other is still sent. On a third, its first wedge: one man is sent there and the rest keep theirs',
        `after the tap: ${s1}, covering ${c1} · the second ring for ${names(part2)}, '${w2}' picked: ${s2}, covering ${names(cov)}, moving ${moving} · the third ring for ${names(one)}, '${w3}' picked: his order is ${went}, parts ${s3}, the other three keep theirs ${kept3} (${by3})`); }
    // 12
    R.home();
    { const spots: V[] = [];
      for (let i = 0; i < 4; i++) { const p = R.onSpot(spots); if (!p) break; spots.push(p); R.key(true); R.step(10); }
      const s4 = R.parts(), p5 = R.onSpot(spots), n0 = O.log.length, r0 = PM.PLAN_STATS.rings, no0 = PM.PLAN_STATS.none, ring = !!p5 && R.hold(), said = PM.PLAN_STATS.none - no0, part = (O.menu?.part ?? []).length; R.away(); R.step(10);
      line(s4 === '1 + 1 + 1 + 1' && !!p5 && !ring && said === 1 && O.log.length === n0 && PM.PLAN_STATS.rings === r0 && R.parts() === s4, '12. every group down to one man: Shift + the key kept down on one more spot puts no ring up and gives nothing',
        `parted first: ${s4} · the ring ${ring ? `UP (for ${part || 'everyone'})` : 'not up'} · the squad said nobody is free: ${said === 1} · orders given ${O.log.length - n0} · parted after: ${R.parts()}`); }
  } finally { knobsBack(knobs0); }

  console.log(fails ? `=== goals-check: ${fails} FAIL ===` : '=== goals-check: ALL PASS ===');
  return !fails;
}

if (import.meta.main) process.exit((await goalsCheck()) ? 0 : 1);
