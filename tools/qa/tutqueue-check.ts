// THE TUTORIAL'S FOURTH MISSION (levels/tut-queue.json, tools/levels/tut-queue.ts): what its idea leans on, asked of the game with the tick's own input.
//
//   bun run tools/qa/tutqueue-check.ts [--verbose]         (probe name `tutqueue`; both stands: the frame, the ring on the command key and the goal rule
//                                                            are raised by hand after each stand-up, as the page has them, and put back)
//
//   1. the level starts on rails with its lesson block whole AND HE IS ON ITS FLOOR with the four behind him: no sky seat, plan mode down, the first step
//      up, and the goal rule not his yet (planmode.ts goalsOn asks the frame's `goal`, which no step has opened);
//   2. BEFORE ITS STEP, SHIFT WITH THE COMMAND KEY IS THE PLAIN KEY: on the guardroom's door all four stack, nobody is parted off, no goal is counted and
//      plan mode stays down;
//   3. FROM ITS STEP IT ADDS A GOAL: on the side door one whole pair goes to stack there and the other pair is still on the first door, plan mode down;
//   4. THE WORD IS SHUT UNTIL A STACK STANDS ON EACH DOOR: pressed while the second pair is still walking it starts no entry; pressed on its own step it
//      starts one on each door;
//   5. FROM EVERY RING HE WALKS TO, THE NEXT TWO THINGS A LINE NAMES ARE ON HIS PICTURE with the view at rest — on the harness's picture and on a small
//      window's (1000 × 673) — clear of the squad's panel on the left and of the line along the bottom;
//   6. NO STEP CAN BE A DEAD END: every step's test has the way out that its room was taken (whoever took it, however);
//   7. NOTHING IN IT IS PLAN MODE'S: no seat, no step gives `plan`, no test asks the mode's view or its picks, and no line or card says "plan";
//   8. THE YARD: called to him and walked to the ring through the hall, every man of the four has a line to the spot by each gate from where the file
//      leaves him; Shift with the key by the first gate turns all four to it, and by the second parts a whole pair off to that one.
// TUTQUEUE_BREAK=1 = no step gives `goal` (the lesson as any other tutorial's): lines 3, 4 and 8 must fail.   exit 0 = all hold · 1 = one does not
import { readFileSync } from 'node:fs';
import { ROOT, standUp } from './headless.ts';
const { parseLevelData } = await import(`${ROOT}/src/scene/levelData.ts`);
const { loadLevel } = await import(`${ROOT}/src/scene/levelLoad.ts`);
const { startFromQuery } = await import(`${ROOT}/src/game/scenarioRun.ts`);
const { RAILS } = await import(`${ROOT}/src/game/rails.ts`);
const { CMDRING } = await import(`${ROOT}/src/game/cmdring.ts`);
const { AUTOPILOT } = await import(`${ROOT}/src/game/autopilot.ts`);
const { checkLesson } = await import(`${ROOT}/src/game/lesson.ts`);
const { knobsCopy, knobsBack } = await import(`${ROOT}/src/game/knobs.ts`);
const { killGuard } = await import(`${ROOT}/src/game/combat.ts`);
const PM: any = await import(`${ROOT}/src/game/planmode.ts`);
const VERBOSE = process.argv.includes('--verbose'), BREAK = process.env.TUTQUEUE_BREAK === '1', LEVEL = 'tut-queue', DT = 1 / 60;
type V = number[];
const flat = (a: V, b: V) => Math.hypot(a[0] - b[0], a[2] - b[2]);

async function rig(seed = 1) {
  const log = console.log, warn = console.warn, err = console.error; if (!VERBOSE) console.log = console.warn = console.error = () => { };
  let W: any, started: any;
  try {
    const data = parseLevelData(readFileSync(`${ROOT}/levels/${LEVEL}.json`, 'utf8'));
    if (BREAK) for (const T of (data as any).scenarios.lesson.steps) if (T.allow) T.allow = T.allow.filter((a: string) => a !== 'goal');   // (no step opens the goal: the lesson as any other tutorial's)
    W = await standUp({ uploadEvery: 4, level: loadLevel(data), seed });
    // (the page's own: the frame, the ring on the command key, the machinery the goal rule rides on and the rule — the frozen stand holds each off)
    RAILS.on = true; Object.assign(CMDRING, { on: true, tapNow: true, stay: true, plan: true, planRun: true, planOne: true, wholeSquad: true, goals: true }); AUTOPILOT.on = true;
    started = startFromQuery(W.game, `?levelFile=${LEVEL}.json&roomclear=1&squad4`);
  } finally { console.log = log; console.warn = warn; console.error = err; }
  const g = W.game, I = W.input, O = g.orders, KEY = CMDRING.key;
  const quiet = <T,>(f: () => T): T => { if (VERBOSE) return f(); const l = console.log, w = console.warn; console.log = console.warn = () => { }; try { return f(); } finally { console.log = l; console.warn = w; } };
  const R = { W, g, I, O, why: started?.why as string | undefined,
    step(n = 1) { quiet(() => { for (let i = 0; i < n; i++) W.step(DT); }); },
    until(pred: () => boolean, secs: number) { for (let i = 0; i < secs * 60 && !pred(); i++) R.step(); return pred(); },
    /** he is stood on a place by hand (the walk there is the scripted player's: tools/qa/lessons/tut-queue.ts), the cursor at his feet, the view at rest */
    stand(p: V) { g.teleportPlayer([p[0], 0, p[2]]); for (let i = 0; i < 150; i++) { W.cursorAt(g.player.char.pos); R.step(); } },
    /** the command key where the cursor is, Shift held or not, as a hand presses it */
    key(shift: boolean) { if (shift) { I.keys.add('ShiftLeft'); R.step(2); } I.keys.add(KEY); I.pressed.add(KEY); R.step(); I.keys.delete(KEY); R.step(3); if (shift) { I.keys.delete('ShiftLeft'); R.step(2); } },
    press(k: string) { I.keys.add(k); I.pressed.add(k); R.step(); I.keys.delete(k); R.step(3); },
    /** the cursor on a step's own ring */
    mark(id: string): boolean { const at = R.stepOf(id)?.at; return !!at && W.cursorAt(at); },
    stepOf(id: string): any { return (g.level.scenarios.lesson.steps as any[]).find(T => T.id === id); },
    up(): string { return g.lesson.step?.id ?? '(none)'; },
    on(door: string): any[] { return O.men('all').filter((m: any) => O.of(m)?.order === 'stack' && O.of(m)?.door?.def.name === door); },
    set(doors: string[]) { return doors.every(d => R.on(d).length > 0 && R.on(d).every((m: any) => O.of(m)?.arrived)); },
    watching(at: V): any[] { return O.men('all').filter((m: any) => { const A = O.of(m); return A?.order === 'cover' && A.watch && flat(A.watch, at) < 3; }); },
  };
  g.godMode = true; R.step(30); for (const m of O.squad()) m.god = true;
  return R;
}
const names = (l: any[]) => l.map((m: any) => m.callsign).sort().join(', ') || 'nobody';

export async function tutqueueCheck(): Promise<boolean> {
  let all = true; const keep = { on: RAILS.on, knobs: knobsCopy(), ap: AUTOPILOT.on };
  const verdict = (ok: boolean, label: string, detail: string) => { all &&= ok; console.log(`- ${ok ? 'PASS' : 'FAIL'} — ${label}\n    ${detail}`); };
  console.log(`the tutorial's fourth mission (levels/${LEVEL}.json)${BREAK ? ' — TUTQUEUE_BREAK=1: no step gives `goal`, lines 3, 4 and 8 must fail' : ''}`);
  try {
    const R = await rig(), g = R.g, O = R.O, L = g.level.scenarios.lesson, pl = g.player, D = g.doorTools, N = O.men('all').length;
    const steps: any[] = L?.steps ?? [], ids = steps.map(T => T.id as string);
    // ---- 1
    { const faults = checkLesson(g), start: V = g.level.scenarios.roomClear.team.sam, idle = O.men('all').filter((m: any) => { const A = O.of(m); return !A || A.post === null; });
      verdict(!R.why && !!L?.rails && faults.length === 0 && R.up() === ids[0] && g.rails.live && !g.rails.skyOn && !L.rails.sky && !O.plan.up && flat(pl.char.pos, start) < 1 && !g.playerInvisible && N === 4 && idle.length === 4 && CMDRING.goals && !PM.goalsOn(O),
        'the level starts on rails with its lesson whole, and he is on its floor with the four behind him: no seat, plan mode down, the goal rule not his yet',
        `start ${R.why ?? 'ok'} · lesson faults ${faults.length}${faults.length ? ` (${faults[0]})` : ''} · step '${R.up()}' · sky seat ${!!g.rails.skyOn} · plan mode ${O.plan.up ? 'UP' : 'down'} · he stands ${flat(pl.char.pos, start).toFixed(1)} m off his start · the four on nothing: ${idle.length} of ${N} · the rule on in the game ${CMDRING.goals}, his here ${PM.goalsOn(O)}`); }
    // ---- 2
    R.stand(R.stepOf('corner').at); R.until(() => R.up() === 'door1', 3);
    { const g0 = PM.PLAN_STATS.goals, onMark = R.mark('door1'); R.key(true); R.step(10); const a = R.on('guard');
      verdict(R.up() !== 'corner' && onMark && a.length === N && PM.PLAN_STATS.goals === g0 && !O.plan.up,
        'before its step, Shift with the command key is the plain key: all four stack on the guardroom\'s door, no goal is counted, plan mode stays down',
        `the step up after the walk: '${ids[1]}' was ${R.up() !== 'corner'} · the cursor on its ring: ${onMark} · stacked on the door: ${names(a)} · goals counted ${PM.PLAN_STATS.goals - g0} · plan mode ${O.plan.up ? 'UP' : 'down'}`); }
    // ---- 3
    R.until(() => R.up() === 'door2', 3);
    { const his = PM.goalsOn(O), onMark = R.mark('door2'); R.key(true); R.step(10); const a = R.on('guard'), b = R.on('guard_side'), pair = b.length === 2 && O.pairOf(b[0]) === O.pairOf(b[1]);
      verdict(R.up() === 'door2' && his && onMark && a.length === 2 && pair && !O.plan.up,
        'from its step it adds a goal: on the side door one whole pair goes to stack there and the other pair is still on the first door',
        `step '${R.up()}' · the rule his now: ${his} · on the guardroom's door: ${names(a)} · on the side door: ${names(b)}${pair ? ` (${O.pairOf(b[0])})` : ''} · plan mode ${O.plan.up ? 'UP' : 'down'}`);
      // ---- 4 (the second pair still walks: the word's key is not his yet)
      const ops0 = D.ops.length; R.press('KeyF'); R.step(20); const early = D.ops.length - ops0, was = R.up();
      const stood = R.until(() => R.set(['guard', 'guard_side']), 60); R.until(() => R.up() === 'word', 4); R.step(30);
      const had = new Set<any>(D.ops); R.press('KeyF'); R.step(4); const e = D.ops.filter((o: any) => !had.has(o)), doors = new Set(e.map((o: any) => o.door.def.name));
      verdict(was === 'door2' && early === 0 && stood && doors.has('guard') && doors.has('guard_side'),
        'the word is shut until a stack stands on each door: pressed early it starts no entry, pressed on its own step it starts one on each door',
        `pressed on step '${was}' with a pair still walking: entries started ${early} · both stacks stood: ${stood} · pressed on its step: ${[...doors].join(' + ') || 'no entry'}`);
      R.until(() => D.ops.length === 0, 60); }
    // ---- 5
    { const sizes: [number, number][] = [[1600, 1000], [1000, 673]], rows: string[] = []; let ok = true;
      const shown = (p: V, w: number, h: number) => { const [x, y, front] = R.W.cam.project([p[0], p[1] ?? 0, p[2]], w, h); return front && x > 0.15 * w && x < 0.96 * w && y > 0.07 * h && y < 0.89 * h ? null : `${x.toFixed(0)},${y.toFixed(0)} of ${w} × ${h}`; };
      for (const [ring, a, b] of [['corner', 'door1', 'door2'], ['around', 'back', 'word2'], ['yard', 'gateA', 'gateB']]) {
        R.stand(R.stepOf(ring).at); const off: string[] = [];
        for (const n of [a, b]) for (const [w, h] of sizes) { const why = shown(R.stepOf(n).at, w, h); if (why) off.push(`'${n}' at ${why}`); }
        if (off.length) ok = false; rows.push(`from '${ring}': ${off.length ? `OFF — ${off.join(' · ')}` : `'${a}' and '${b}' on both pictures`}`);
      }
      // (…and the hall's front door from the guardroom's corner, where its line comes up)
      R.stand(R.stepOf('corner').at); for (const [w, h] of sizes) { const why = shown(R.stepOf('front').at, w, h); if (why) { ok = false; rows.push(`from 'corner': OFF — 'front' at ${why}`); } }
      verdict(ok, 'from every ring he walks to, the next two things a line names are on his picture with the view at rest, on a small window too', rows.join(' · ')); }
    // ---- 6
    { const has = (t: any, room: string): boolean => !!t && ((t.is === 'taken' && t.room === room) || (t.is === 'any' && (t.of as any[]).some(u => has(u, room))) || (t.is === 'all' && (t.of as any[]).every(u => has(u, 'road') || has(u, 'lane'))));
      const roomOf = (i: number) => i <= ids.indexOf('word') ? ['guardroom'] : i <= ids.indexOf('word2') ? ['hall'] : ['road', 'lane'];
      const stuck = steps.filter((T, i) => !roomOf(i).some(r => has(T.done, r))).map(T => T.id);
      verdict(steps.length >= 10 && stuck.length === 0, 'no step can be a dead end: every step\'s test has the way out that its room was taken', `${steps.length} steps · without that way out: ${stuck.join(', ') || 'none'}`); }
    // ---- 7
    { const kinds = (t: any): string[] => !t ? [] : [t.is, ...((t.of as any[] | undefined) ?? []).flatMap(kinds)];
      const gives = [...(L.rails.allow as string[]), ...steps.flatMap(T => (T.allow as string[] | undefined) ?? [])], asks = steps.flatMap(T => kinds(T.done)).filter(k => k === 'planView' || k === 'picked');
      const words = steps.flatMap(T => [T.line, T.explain?.title, T.explain?.text, T.after?.line, T.after?.word, T.planLine]).filter((s): s is string => !!s).filter(s => /\bplan/i.test(s));
      verdict(!L.rails.sky && !gives.includes('plan') && asks.length === 0 && words.length === 0 && steps.every(T => !T.planLine) && gives.includes('goal') !== BREAK,
        'nothing in it is plan mode\'s: no seat, no step gives `plan`, no test asks the mode\'s view or its picks, no line or card says "plan"',
        `a seat: ${!!L.rails.sky} · gives: ${[...new Set(gives)].join(' ')} · the mode's tests: ${asks.join(' ') || 'none'} · lines that say it: ${words.length}`); }
    // ---- 8 (the hall taken by hand: its men put down, the lesson walked on to the yard's steps)
    { for (const gd of g.guards) if (gd.team !== 'operative' && /^Tango [34]$/.test(gd.callsign) && gd.state !== 'dead') killGuard(g, gd, [1, 0, 0], false, 'shot', 'player');
      for (const r of g.roomClear.rooms) if (r.def.id === 'hall') r.cleared = true;
      R.stand([28.4, 0, 16.0]); R.until(() => R.up() === 'call', 20);
      const onCall = R.up() === 'call'; R.W.cursorAt(pl.char.pos); R.step(2); R.key(false); R.step(10); const following = O.men('all').every((m: any) => O.of(m)?.order === 'fallIn');
      // (the walk out by the hall's front door, as a hand goes: straight keys toward each point, the file strung out behind him)
      const walk = (p: V, secs: number) => { for (let i = 0; i < secs * 60 && flat(pl.char.pos, p) > 0.5; i++) { const s = pl.char.pos, dx = p[0] - s[0], dz = p[2] - s[2]; R.W.cursorAt([p[0], 0, p[2]]);
        for (const k of ['KeyW', 'KeyA', 'KeyS', 'KeyD']) R.I.keys.delete(k);
        const d = Math.hypot(dx, dz), { fwd, right } = R.W.cam.planarBasis(), a = (dx * fwd[0] + dz * fwd[2]) / d, b = (dx * right[0] + dz * right[2]) / d, T = 0.3827;   // (the walk keys are the picture's: lesson-run.ts steers the same way)
        if (a > T) R.I.keys.add('KeyW'); if (a < -T) R.I.keys.add('KeyS'); if (b > T) R.I.keys.add('KeyD'); if (b < -T) R.I.keys.add('KeyA'); R.step(); }
        for (const k of ['KeyW', 'KeyA', 'KeyS', 'KeyD']) R.I.keys.delete(k); };
      const yard: V = R.stepOf('yard').at; walk([26.0, 0, 14.2], 8); walk([26.0, 0, 12.0], 6); walk(yard, 12); R.step(120);
      const there = flat(pl.char.pos, yard) < 1.6, E: V = R.stepOf('gateA').at, Nn: V = R.stepOf('gateB').at;
      const lines = O.men('all').map((m: any) => `${m.callsign} (${m.char.pos[0].toFixed(1)}, ${m.char.pos[2].toFixed(1)}) ${O.hasLine(m, { kind: 'floor', at: E }) ? 'road' : 'NO ROAD'} ${O.hasLine(m, { kind: 'floor', at: Nn }) ? 'back' : 'NO BACK'}`);
      const clear = O.men('all').every((m: any) => O.hasLine(m, { kind: 'floor', at: E }) && O.hasLine(m, { kind: 'floor', at: Nn }));
      R.until(() => R.up() === 'gateA', 4); R.mark('gateA'); R.key(true); R.step(10); const e1 = R.watching(E);
      R.until(() => R.up() === 'gateB', 4); R.mark('gateB'); R.key(true); R.step(10); const e2 = R.watching(E), n2 = R.watching(Nn), pair = n2.length === 2 && O.pairOf(n2[0]) === O.pairOf(n2[1]);
      verdict(onCall && following && there && clear && e1.length === N && e2.length === 2 && pair,
        'the yard: called and walked out through the hall, every man has a line to the spot by each gate; the first gate turns all four, the second parts a whole pair off to it',
        `the key on himself: all four fall in ${following} · he reached the ring: ${there} (${pl.char.pos[0].toFixed(1)}, ${pl.char.pos[2].toFixed(1)}) · ${lines.join(' · ')} · turned to the road gate: ${names(e1)} · then to the back gate: ${names(n2)}, left on the road gate: ${names(e2)}`); }
  } finally { knobsBack(keep.knobs); RAILS.on = keep.on; AUTOPILOT.on = keep.ap; }
  console.log(all ? '=== tutqueue-check: ALL PASS ===' : '=== tutqueue-check: FAIL ===');
  return all;
}

if (import.meta.main) process.exit((await tutqueueCheck()) ? 0 : 1);
