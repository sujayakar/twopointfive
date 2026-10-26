// SHIFT + THE KEY ADDS A GOAL, AND THERE IS NO PLAN MODE (src/game/planmode.ts planGoal, CMDRING.goals — a PROTOTYPE). The maintainer, 10-26: "could we
// prototype removing plan mode and having shift Q instead add a goal to the squad? like shift Q on a second door will naturally split the squad in two,
// and F tells all of them to go in. shift Q on a spot to watch automatically has them split up angles".
//   bun run tools/qa/goals-check.ts            (also `probes.ts goals`, in family squadorders)
//   TPS_PROFILE=shipped bun run tools/qa/goals-check.ts      — the same lines on the page's stand
// PASS / FAIL each, headless: the plan mode mission's own level (levels/tut-plan.json) with its frame off, the room clear with the squad of four under
// god mode, ONE deal (seed 1: the rule has no die in it). The check raises the ring, the mode's machinery and the rule itself, so both stands ask the
// same lines. The keys are pressed as a hand presses them: Shift down, the squad's key, the cursor on the thing.
//   1. Shift + the key on a door, nobody on anything: plan mode does not come up, and all four stack on that door (the first goal is everyone's).
//   2. Shift + the key on the room's second door: one whole pair is sent to stack there, and the other pair is still stacked on the first.
//   3. both stacked, ONE press of the word's key: an entry starts on each door, the two on one word of the door tools, and men are through both doors
//      within two seconds of each other.
//   4. the key WITHOUT Shift on the first door, after a split: everyone is on that door again — one goal.
//   5. Shift + the key on a spot of floor, nobody on anything: all four cover it. On a second spot: two cover each. On a third: two, one and one.
//   6. every group down to one man: Shift + the key on one more spot gives nothing, and the squad says so.
//   7. Shift + the key KEPT DOWN is still the ring: it comes up, plan mode does not, and no goal is added by it.
//   8. CONTROL — the switch off: Shift + the key puts plan mode up and orders nobody, as it was built. And on a tutorial mission the rule is not asked.
//   9. CONTROL (plumbing) — a recording from before the key replays with it off; the frozen stand holds it off, the page's has it on, FROZEN_HOLDS has its row.
// GOALS_BREAK=1 = the switch left off and the same lines asked: 1 to 7 must fail.
//   exit 0 = all hold · 1 = one does not
import { readFileSync } from 'node:fs';
import { ROOT, standUp } from './headless.ts';
const { parseLevelData } = await import(`${ROOT}/src/scene/levelData.ts`);
const { loadLevel } = await import(`${ROOT}/src/scene/levelLoad.ts`);
const { startFromQuery } = await import(`${ROOT}/src/game/scenarioRun.ts`);
const { RAILS } = await import(`${ROOT}/src/game/rails.ts`);
const { CMDRING } = await import(`${ROOT}/src/game/cmdring.ts`);
const PM: any = await import(`${ROOT}/src/game/planmode.ts`);
const { AUTOPILOT } = await import(`${ROOT}/src/game/autopilot.ts`);
const { DOORTOOLS, offOf } = await import(`${ROOT}/src/game/doortools.ts`);
const { knobsCopy, knobsBack } = await import(`${ROOT}/src/game/knobs.ts`);
const { applyKnobs, knobsSnapshot } = await import(`${ROOT}/src/game/journal.ts`);
const { FROZEN_HOLDS } = await import(`${ROOT}/src/host/profile.ts`);
const VERBOSE = process.argv.includes('--verbose'), BREAK = process.env.GOALS_BREAK === '1', LEVEL = 'tut-plan', DT = 1 / 60, SEED = 1;
type V = number[];
/** the switch as the stand first stood it up, before the check raises it */
let stood: boolean | null = null;
const flat = (a: V, b: V) => Math.hypot(a[0] - b[0], a[2] - b[2]);

async function rig(on: boolean) {
  const log = console.log, warn = console.warn, err = console.error; if (!VERBOSE) console.log = console.warn = console.error = () => { };
  let W: any, started: any;
  try {
    const level = loadLevel(parseLevelData(readFileSync(`${ROOT}/levels/${LEVEL}.json`, 'utf8')));
    W = await standUp({ uploadEvery: 4, level, seed: SEED }); stood ??= !!CMDRING.goals;
    // (the mission's frame off: the lines are the rule's, not the lesson's · the ring, the mode's machinery and its later switches as the page has them)
    RAILS.on = false; Object.assign(CMDRING, { on: true, tapNow: true, stay: true, plan: true, planRts: true, planRun: true, planLook: true, planWaits: true, planKnee: true, planOne: true, wholeSquad: true, goals: on }); AUTOPILOT.on = true;
    started = startFromQuery(W.game, `?levelFile=${LEVEL}.json&roomclear=1&squad4`);
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
    key(shift: boolean) { if (shift) I.keys.add('ShiftLeft'); I.keys.add(KEY); I.pressed.add(KEY); R.step(); I.keys.delete(KEY); R.step(2); if (shift) { I.keys.delete('ShiftLeft'); R.step(); } },
    /** who is on that door by a plain stack, and whether they all stand on their marks */
    onStack(d: any): any[] { return O.men('all').filter((m: any) => O.of(m)?.order === 'stack' && O.of(m)?.door === d); },
    set(men: any[]) { return men.length > 0 && men.every((m: any) => O.of(m)?.arrived); },
    /** everything put away and the men back round Sam */
    home() { I.keys.clear(); I.buttons = 0; O.cmdRing = null; O.close(); if (O.plan.up) { I.keys.add('ShiftLeft'); I.pressed.add(KEY); R.step(); I.keys.delete('ShiftLeft'); R.step(); } O.plan.chains = []; O.plan.pick = null; O.who = 'all';
      O.give('fallIn', O.targetAt(g.player.char.pos), 'all'); R.until(() => D.ops.length === 0, 40); R.until(() => O.men('all').every((m: any) => flat(m.char.pos, g.player.char.pos) < 5), 30); R.step(30); },
    /** the men covering, by the point each watches */
    watches(): Map<string, string[]> { const out = new Map<string, string[]>(); for (const m of O.men('all')) { const A = O.of(m); if (A?.order !== 'cover' || !A.watch) continue; const k = `${A.watch[0].toFixed(1)},${A.watch[2].toFixed(1)}`; out.set(k, [...(out.get(k) ?? []), m.callsign]); } return out; },
  };
  g.godMode = true; R.step(30); for (const m of O.squad()) m.god = true;
  return R;
}

export async function goalsCheck(): Promise<boolean> {
  let fails = 0;
  const line = (ok: boolean, what: string, got: string) => { console.log(`  ${ok ? 'PASS' : 'FAIL'}  ${what} — ${got}`); if (!ok) fails++; };
  const knobs0 = knobsCopy(), rails0 = RAILS.on, K = CMDRING;
  console.log(`Shift + the key adds a goal (src/game/planmode.ts planGoal, CMDRING.goals)${BREAK ? ' — GOALS_BREAK=1: the switch left off, lines 1 to 7 must fail' : ''}`);
  try {
    const R = await rig(!BREAK), g = R.g, O = R.O, D = R.D, N = O.men('all').length;
    const A = R.door('guard'), B = R.door('guard_side'), names = (l: any[]) => l.map((m: any) => m.callsign).sort().join(', ') || 'nobody';
    if (R.why || !A || !B || N !== 4) { line(false, 'the room clear on the plan mission\'s level, four men and the guardroom\'s two doors', R.why ?? `${N} men · doors ${A ? 'guard' : '-'} ${B ? 'guard_side' : '-'}`); return false; }

    // 1
    R.home();
    { const onA = R.onDoor(A); R.key(true); const up = O.plan.up, to = R.onStack(A);
      line(onA && !up && to.length === N, '1. Shift + the key on a door, nobody on anything: plan mode does not come up, and all four stack on that door',
        `the cursor on the door: ${onA} · plan mode ${up ? 'UP' : 'down'} · sent to stack on it: ${names(to)}`); if (O.plan.up) R.home(); }
    // 2
    { O.give('stack', { kind: 'door', at: [A.frameCentre[0], 0, A.frameCentre[2]], door: A, side: O.sideFor ? O.forMen({ kind: 'door', at: A.frameCentre, door: A, side: 1 }, O.men('all')).side : 1 }, 'all'); R.until(() => R.set(R.onStack(A)) && R.onStack(A).length === N, 40);
      const all = R.onStack(A).length, onB = R.onDoor(B); R.key(true); const a = R.onStack(A), b = R.onStack(B), pair = b.length === 2 && O.pairOf(b[0]) === O.pairOf(b[1]);
      line(all === N && onB && !O.plan.up && a.length === 2 && pair && PM.PLAN_STATS.split > 0, '2. Shift + the key on the second door: one whole pair is sent to stack there, and the other pair is still stacked on the first',
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
    { const spots: V[] = []; const sizes = (): string => [...R.watches().values()].map((l: string[]) => l.length).sort((x: number, y: number) => y - x).join(' + ') || 'nobody covering';
      const add = (): string => { const p = R.onSpot(spots); if (!p) return 'no spot'; spots.push(p); R.key(true); R.step(10); return sizes(); };
      const s1 = add(), s2 = add(), s3 = add();
      line(s1 === '4' && s2 === '2 + 2' && s3 === '2 + 1 + 1' && !O.plan.up, '5. Shift + the key on a spot, nobody on anything: all four cover it. A second spot: two cover each. A third: two, one and one',
        `covering after the first: ${s1} · the second: ${s2} · the third: ${s3}`);
      const s4 = add(), n0 = O.log.length, g0 = PM.PLAN_STATS.goals, s5 = add(), gave = O.log.length - n0, asked = PM.PLAN_STATS.goals - g0;
      line(s4 === '1 + 1 + 1 + 1' && s5 === s4 && gave === 0 && asked === 1, '6. every group down to one man: Shift + the key on one more spot gives nothing',
        `after the fourth: ${s4} · after a fifth: ${s5} · orders it gave: ${gave} · the key read as a goal: ${asked === 1} · the squad's words for it: "${PM.GOAL_NONE}"`); }
    // 7
    R.home();
    { const g0 = PM.PLAN_STATS.goals, n0 = O.log.length, onA = R.onDoor(A); R.I.keys.add('ShiftLeft'); R.I.keys.add(K.key); R.I.pressed.add(K.key); const ring = R.until(() => !!O.cmdRing, 1.5), up = O.plan.up;
      R.I.keys.clear(); O.cmdRing = null; O.close(); R.step(4); const asked = PM.PLAN_STATS.goals - g0, gave = O.log.length - n0;
      line(onA && ring && !up && asked === 0 && gave === 0, '7. Shift + the key KEPT DOWN is still the ring: it comes up, plan mode does not, and no goal is added by it',
        `the ring ${ring ? 'up' : 'NOT up'} · plan mode ${up ? 'UP' : 'down'} · goals added ${asked} · orders given ${gave}`); if (O.plan.up) R.home(); }
    // 8
    R.home(); K.goals = false;
    { const n0 = O.log.length; R.onDoor(A); R.key(true); const up = O.plan.up, gave = O.log.length - n0; if (O.plan.up) { R.I.keys.add('ShiftLeft'); R.I.pressed.add(K.key); R.step(); R.I.keys.delete('ShiftLeft'); R.step(); }
      K.goals = true; const plain = PM.goalsOn(O), rails = g.rails; g.rails = { live: true }; const lesson = PM.goalsOn(O); g.rails = rails;
      line(up && gave === 0 && plain === true && lesson === false, '8. CONTROL — the switch off: Shift + the key puts plan mode up and orders nobody. And on a tutorial mission the rule is not asked',
        `off: plan mode ${up ? 'up' : 'DOWN'}, orders given ${gave} · on: the rule is asked here ${plain}, on a tutorial mission ${lesson}`); }
    // 9
    { K.goals = true; const snap = JSON.parse(JSON.stringify(knobsSnapshot(g))), old = JSON.parse(JSON.stringify(snap)); delete old.tables.CMDRING.goals;
      applyKnobs(g, old); const before = K.goals; applyKnobs(g, snap); const on = K.goals;
      const shipped = String(process.env.TPS_PROFILE ?? '') === 'shipped', row = FROZEN_HOLDS['CMDRING.goals'];
      line(before === false && on === true && stood === shipped && /TPS_GOALS=1/.test(row ?? ''), '9. CONTROL (plumbing) — a recording from before the key replays with it off; the stands and the FROZEN_HOLDS row',
        `a file without the key: ${before} · one with it on: ${on} · this stand (${shipped ? 'the page\'s' : 'frozen'}) stood it ${stood} · the row: ${row ?? 'none'}`); }
  } finally { knobsBack(knobs0); RAILS.on = rails0; }

  console.log(fails ? `=== goals-check: ${fails} FAIL ===` : '=== goals-check: ALL PASS ===');
  return !fails;
}

if (import.meta.main) process.exit((await goalsCheck()) ? 0 : 1);
