// THE TUTORIAL'S FOURTH MISSION (levels/tut-queue.json, tools/levels/tut-queue.ts): what its idea leans on, asked of the game with the tick's own input.
//
//   bun run tools/qa/tutqueue-check.ts [--verbose]         (probe name `tutqueue`; both stands: the frame, the ring on the command key, the goal rule
//                                                            and its ring are raised by hand after each stand-up, as the page has them, and put back)
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
//   6. NO STEP OF THE TWO DOOR FIGHTS CAN BE A DEAD END: each one's test has the way out that its room was taken (whoever took it, however). The yard's
//      steps have nothing to take until the last one's cue brings the men in: line 9 is theirs;
//   7. NOTHING IN IT IS PLAN MODE'S: no seat, no step gives `plan`, no test asks the mode's view or its picks, and no line or card says "plan";
//   8. THE YARD: called to him and walked to the ring through the hall, every man of the four has a line to the spot by each gate from where the file
//      leaves him. BOTH GATES ARE THE WHEEL FOR A PART (the frame's `ringPart`; the maintainer, 11-08: "you should need to open the radial menu to get covering a point"): a Shift
//      tap there gives nothing and says to keep the key down; the key kept down with NO Shift is a tap too, no ring and the same note; with Shift, the ring is up for the whole pair
//      a tap would have sent, with no more than five wedges and the step's own line for the one to press, and that wedge turns the pair to the gate while the other pair stays on the first.
//   9. THE WRONG HAND IN THE YARD IS NO DEAD END (four more games, the two door fights taken by hand; QA_SWEEP=all asks them): two stray Shift taps on the way out
//      send two pairs to stand there and nobody watches, and the tap on himself calls them back, whereupon both gates pass by the wheel. Four leave nobody free: the wheel by the first gate puts no
//      ring up and says so, a tap at the gate gives nothing, and THE TAP ON HIMSELF calls them back, whereupon both gates pass. A wrong wedge [1] sends a pair to stand by
//      the gate, the step stays, and the line's own wedge on the next ring ends it. The tap on himself at the second gate calls the first gate's men off too: the line's wheel ends
//      that step on its own gate, the next step asks for the first gate again, its wheel leaves a pair on each, and the fight is played to its end.
//  10. THE KEY TAPPED TWICE IS NOT THE LESSON'S (a third game, CMDRING.twice raised): on the first door's step a second tap inside the window is the
//      key's order again, all four still stack on it, and nobody falls in — no step lists `twice`, so a double press cannot undo what a step asked for.
// TUTQUEUE_BREAK=1 = no step gives `goal` (the lesson as any other tutorial's): lines 3, 4, 8 and 9 must fail (`railsFaults` also calls `ringPart` with no goal a fault: line 1).   exit 0 = all hold · 1 = one does not
import { readFileSync } from 'node:fs';
import { ROOT, standUp } from './headless.ts';
import { swept } from './sweep.ts';
import { locked } from '../../src/view/seatview.ts';
const { parseLevelData } = await import(`${ROOT}/src/scene/levelData.ts`);
const { loadLevel } = await import(`${ROOT}/src/scene/levelLoad.ts`);
const { startFromQuery } = await import(`${ROOT}/src/game/scenarioRun.ts`);
const { RAILS } = await import(`${ROOT}/src/game/rails.ts`);
const { CMDRING, CMDRING_STATS } = await import(`${ROOT}/src/game/cmdring.ts`);
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
    RAILS.on = true; Object.assign(CMDRING, { on: true, tapNow: true, stay: true, plan: true, planRun: true, planOne: true, wholeSquad: true, goals: true, goalRing: true, only: true, flat: true }); RAILS.flatRing = true; AUTOPILOT.on = true;
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
    /** the command key kept down on a step's ring with NO Shift, past the hold: what came of it */
    plainHold(id: string) { R.mark(id); const n0 = O.log.length; I.keys.add(KEY); I.pressed.add(KEY); R.step(); for (let i = 0; i < 45; i++) { R.mark(id); I.keys.add(KEY); R.step(); } const ring = !!O.menu; I.keys.delete(KEY); R.step(6); return { ring, gave: O.log.length - n0, note: g.view.lesson?.rails?.prompt?.note ?? '' }; },
    /** Shift and the command key kept down on a step's ring until the ring is up, with what it shows and a wedge given by its digit (the row 'cover', or the first), every key let go */
    wheel(id: string, first = false) { R.mark(id); I.keys.add('ShiftLeft'); R.step(2); I.keys.add(KEY); I.pressed.add(KEY); R.step(); for (let i = 0; i < 45; i++) { R.mark(id); R.step(); }
      const M = O.menu, rows: string[] = M?.rows.map((r: any) => r.id) ?? [], line = g.view.lesson?.rails?.prompt?.line ?? '', men: any[] = M ? [...M.men] : [], part = !!M?.part, flat = !!M?.flat, digit = first ? 'Digit1' : `Digit${rows.indexOf('cover') + 1}`;
      if (M) { I.keys.add(digit); I.pressed.add(digit); R.step(); I.keys.delete(digit); R.step(3); } I.keys.delete(KEY); I.keys.delete('ShiftLeft'); R.step(10); return { up: !!M, part, flat, men, rows, line }; },
    /** the cursor on a step's own ring */
    mark(id: string): boolean { const at = R.stepOf(id)?.at; return !!at && W.cursorAt(at); },
    stepOf(id: string): any { return (g.level.scenarios.lesson.steps as any[]).find(T => T.id === id); },
    up(): string { return g.view.lesson?.id ?? '(none)'; },
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
  console.log(`the tutorial's fourth mission (levels/${LEVEL}.json)${BREAK ? ' — TUTQUEUE_BREAK=1: no step gives `goal`, lines 3, 4, 8 and 9 must fail' : ''}`);
  try {
    const R = await rig(), g = R.g, O = R.O, L = g.level.scenarios.lesson, pl = g.player, D = g.doorTools, N = O.men('all').length;
    const steps: any[] = L?.steps ?? [], ids = steps.map(T => T.id as string);
    // ---- 1
    { const faults = checkLesson(g), start: V = g.level.scenarios.roomClear.team.sam, idle = O.men('all').filter((m: any) => { const A = O.of(m); return !A || A.post === null; });
      verdict(!R.why && !!L?.rails && faults.length === 0 && R.up() === ids[0] && !!g.view.lesson?.rails && !g.view.lesson?.rails?.sky && !L.rails.sky && !O.plan?.up && flat(pl.char.pos, start) < 1 && !g.playerInvisible && N === 4 && idle.length === 4 && CMDRING.goals && !PM.goalsOn(O),
        'the level starts on rails with its lesson whole, and he is on its floor with the four behind him: no seat, plan mode down, the goal rule not his yet',
        `start ${R.why ?? 'ok'} · lesson faults ${faults.length}${faults.length ? ` (${faults[0]})` : ''} · step '${R.up()}' · sky seat ${!!g.view.lesson?.rails?.sky} · plan mode ${O.plan?.up ? 'UP' : 'down'} · he stands ${flat(pl.char.pos, start).toFixed(1)} m off his start · the four on nothing: ${idle.length} of ${N} · the rule on in the game ${CMDRING.goals}, his here ${PM.goalsOn(O)}`); }
    // ---- 2
    R.stand(R.stepOf('corner').at); R.until(() => R.up() === 'door1', 3);
    { const g0 = PM.PLAN_STATS.goals, onMark = R.mark('door1'); R.key(true); R.step(10); const a = R.on('guard');
      verdict(R.up() !== 'corner' && onMark && a.length === N && PM.PLAN_STATS.goals === g0 && !O.plan?.up,
        'before its step, Shift with the command key is the plain key: all four stack on the guardroom\'s door, no goal is counted, plan mode stays down',
        `the step up after the walk: '${ids[1]}' was ${R.up() !== 'corner'} · the cursor on its ring: ${onMark} · stacked on the door: ${names(a)} · goals counted ${PM.PLAN_STATS.goals - g0} · plan mode ${O.plan?.up ? 'UP' : 'down'}`); }
    // ---- 3
    R.until(() => R.up() === 'door2', 3);
    { const his = PM.goalsOn(O), onMark = R.mark('door2'); R.key(true); R.step(10); const a = R.on('guard'), b = R.on('guard_side'), pair = b.length === 2 && O.pairOf(b[0]) === O.pairOf(b[1]);
      verdict(R.up() === 'door2' && his && onMark && a.length === 2 && pair && !O.plan?.up,
        'from its step it adds a goal: on the side door one whole pair goes to stack there and the other pair is still on the first door',
        `step '${R.up()}' · the rule his now: ${his} · on the guardroom's door: ${names(a)} · on the side door: ${names(b)}${pair ? ` (${O.pairOf(b[0])})` : ''} · plan mode ${O.plan?.up ? 'UP' : 'down'}`);
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
    { const out = (t: any, rooms: string[]): boolean => !!t && ((t.is === 'taken' && rooms.includes(t.room)) || (t.is === 'any' && (t.of as any[]).some(u => out(u, rooms))) || (t.is === 'all' && (t.of as any[]).every(u => out(u, rooms))));
      const fights = steps.slice(0, ids.indexOf('word2') + 1), stuck = fights.filter((T, i) => !out(T.done, [i <= ids.indexOf('word') ? 'guardroom' : 'hall'])).map(T => T.id);
      verdict(fights.length >= 8 && stuck.length === 0, 'no step of the two door fights can be a dead end: each one\'s test has the way out that its room was taken', `${fights.length} steps to the second word · without that way out: ${stuck.join(', ') || 'none'} · the yard's ${steps.length - fights.length}: line 9`); }
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
      // (both gates are the wheel's: a Shift tap gives nothing there and says to keep the key down; kept down past the hold the ring is up for the pair a tap would have sent, and its wedge is theirs)
      const gate = (id: string, at: V) => { R.until(() => R.up() === id, 4); R.step(R.up() === id ? 6 : 0); R.mark(id); const n0 = O.log.length, g0 = PM.PLAN_STATS.goals; R.key(true); R.step(10); const tapped = O.log.length - n0 + (PM.PLAN_STATS.goals - g0), note = g.view.lesson?.rails?.prompt?.note ?? '', plain = R.plainHold(id); return { tapped, note, plain, W: R.wheel(id), at }; };
      const A = gate('gateA', E), e1 = R.watching(E), B = gate('gateB', Nn), e2 = R.watching(E), n2 = R.watching(Nn), pair = (l: any[]) => l.length === 2 && O.pairOf(l[0]) === O.pairOf(l[1]);
      const good = (x: ReturnType<typeof gate>, id: string) => x.tapped === 0 && /held down/.test(x.note) && !x.plain.ring && x.plain.gave === 0 && /held down/.test(x.plain.note) && x.W.part && x.W.flat && x.W.men.length === 2 && x.W.rows.indexOf('cover') === 1 && x.W.rows.length <= 5 && x.W.line === R.stepOf(id).flatLine;
      const say1 = (x: ReturnType<typeof gate>) => `a Shift tap: ${x.tapped ? 'AN ORDER' : 'nothing'}, the note "${x.note}" · kept down with no Shift: ${x.plain.ring ? 'A RING' : 'no ring'}, ${x.plain.gave} orders · with Shift, the ring for: ${x.W.part ? names(x.W.men) : x.W.up ? 'everyone' : 'no ring'}${x.W.flat ? '' : ' (NOT the one ring)'} · its rows: ${x.W.rows.join(', ') || 'none'} · the line "${x.W.line}"`;
      verdict(onCall && following && there && clear && good(A, 'gateA') && pair(e1) && good(B, 'gateB') && pair(n2) && pair(e2) && e1.every((m: any) => e2.includes(m)) && n2.every((m: any) => !e1.includes(m)),
        `the yard: called and walked out through the hall, every man has a line to the spot by each gate; at each gate a Shift tap and the key kept down alone give nothing, and the pair kept down is the ring for a whole pair, whose wedge turns them to it and leaves the other pair`,
        `the key on himself: all four fall in ${following} · he reached the ring: ${there} (${pl.char.pos[0].toFixed(1)}, ${pl.char.pos[2].toFixed(1)}) · ${lines.join(' · ')} · the road gate: ${say1(A)} · turned to it: ${names(e1)} · the back gate: ${say1(B)} · to it ${names(n2)}, left on the road gate ${names(e2)}`); }
    // ---- 9 (more games: both door fights taken by hand, the lesson walked on to the yard's step, the squad called. The probe's cost: QA_SWEEP=all asks them)
    if (swept(0, 1)) { const yard = async () => { const R = await rig(), g = R.g, O = R.O, pl = g.player;
        for (const gd of g.guards) if (gd.team !== 'operative' && /^Tango [1-4]$/.test(gd.callsign) && gd.state !== 'dead') killGuard(g, gd, [1, 0, 0], false, 'shot', 'player');
        for (const r of g.roomClear.rooms) if (r.def.id === 'guardroom' || r.def.id === 'hall') r.cleared = true;
        R.stand([28.4, 0, 16.0]); R.until(() => R.up() === 'call', 60); R.W.cursorAt(pl.char.pos); R.step(2); R.key(false); R.step(10); R.until(() => R.up() === 'yard', 6); R.stand([22.5, 0, 15.0]); R.step(120);
        return { R, g, O, pl, at: (id: string): V => R.stepOf(id).at,
          /** a Shift tap on a bare spot of the picture, near him and on no man: what it logged */
          stray(seen: V[]) { for (const [dx, dz] of [[3, 0], [-3, 0], [0, 3], [0, -3], [2.5, 2.5], [-2.5, -2.5], [2.5, -2.5], [-2.5, 2.5], [4.5, 0], [0, 4.5], [-4.5, 0], [0, -4.5]]) { const p: V = [pl.char.pos[0] + dx, 0.04, pl.char.pos[2] + dz]; if (seen.some(q => flat(q, p) < 2)) continue; R.W.cursorAt(p as [number, number, number]); R.step(2); const t = O.targetUnder(R.I, R.W.cam, R.W.canvas); if (t.kind !== 'floor' || t.thing || O.men('all').some((m: any) => flat(m.char.pos, t.at) < 1.2)) continue; seen.push(p); const n = O.log.length; R.key(true); R.step(10); return O.log.length > n ? O.log[O.log.length - 1].order : 'nothing'; } return 'no spot'; },
          gate(id: string) { R.until(() => R.up() === id, 8); R.step(R.up() === id ? 6 : 0); return R.up() === id; },
          parts(): string { const by = new Map<number, number>(); for (const m of O.men('all')) { const A = O.of(m); if (A && A.post !== null) by.set(A.group, (by.get(A.group) ?? 0) + 1); } return [...by.values()].sort((x, y) => y - x).join(' + ') || 'nobody on anything'; } }; };
      { const Y = await yard(), R = Y.R, seen: V[] = [], t1 = Y.stray(seen), t2 = Y.stray(seen), cover0 = Y.O.men('all').filter((m: any) => Y.O.of(m)?.order === 'cover').length, parts = Y.parts();
        R.stand(Y.at('yard')); R.step(240); const onA = Y.gate('gateA'), lineA = Y.g.view.lesson?.rails?.prompt?.line ?? '';
        R.W.cursorAt(Y.pl.char.pos); R.step(2); R.key(false); R.step(10); const called = Y.O.men('all').every((m: any) => Y.O.of(m)?.order === 'fallIn'); R.step(240);
        const wa = R.wheel('gateA'), onB = Y.gate('gateB'), wb = R.wheel('gateB'); R.until(() => R.up() === 'hold', 6);
        verdict(t1 === 'move' && t2 === 'move' && cover0 === 0 && parts === '2 + 2' && onA && /road gate/.test(lineA) && called && wa.part && onB && wb.part && R.up() === 'hold',
          'two stray Shift taps on the way out send two pairs to stand there and nobody watches; the tap on himself calls them back and both gates then pass by the wheel',
          `the taps: ${t1}, ${t2} · parted ${parts}, covering ${cover0} · '${onA ? 'gateA' : R.up()}' "${lineA}" · the tap on himself: all fall in ${called} · the ring for ${names(wa.men)} · '${onB ? 'gateB' : R.up()}', the ring for ${names(wb.men)} · watching the road ${names(R.watching(Y.at('gateA')))}, the back ${names(R.watching(Y.at('gateB')))} · then '${R.up()}'`); }
      { const Y = await yard(), R = Y.R, O = Y.O, seen: V[] = [], taps = [Y.stray(seen), Y.stray(seen), Y.stray(seen), Y.stray(seen)], parts = Y.parts();
        R.stand(Y.at('yard')); R.step(240); const onA = Y.gate('gateA'), none0 = PM.PLAN_STATS.none, w0 = R.wheel('gateA'), said = PM.PLAN_STATS.none - none0, n0 = O.log.length; R.mark('gateA'); R.key(true); R.step(10); R.mark('gateA'); R.key(false); R.step(10); const tapped = O.log.length - n0, still = R.up() === 'gateA';
        R.W.cursorAt(Y.pl.char.pos); R.step(2); R.key(false); R.step(10); const called = O.men('all').every((m: any) => O.of(m)?.order === 'fallIn'); R.step(240);
        const wa = R.wheel('gateA'), onB = Y.gate('gateB'), wb = R.wheel('gateB'); R.until(() => R.up() === 'hold', 6);
        verdict(taps.every(t => t === 'move') && parts === '1 + 1 + 1 + 1' && onA && !w0.up && said === 1 && tapped === 0 && still && called && wa.part && wa.men.length === 2 && onB && wb.part && wb.men.length === 2 && R.up() === 'hold',
          'four stray Shift taps leave nobody free: no ring by the gate and the squad says so, a tap there gives nothing, the tap on himself calls them back, and both gates then pass',
          `the taps: ${taps.join(', ')} · parted ${parts} · the wheel by the road gate: ${w0.up ? 'A RING' : 'no ring'}, nobody free said ${said} · taps there gave ${tapped}, the step still up ${still} · the tap on himself: all fall in ${called} · then the rings for ${names(wa.men)} and ${names(wb.men)} · then '${R.up()}'`); }
      { const Y = await yard(), R = Y.R, O = Y.O; R.stand(Y.at('yard')); R.step(240); Y.gate('gateA'); const w1 = R.wheel('gateA', true), sent = O.men('all').filter((m: any) => O.of(m)?.order === 'move'), still = R.up() === 'gateA' && R.watching(Y.at('gateA')).length === 0;
        const wa = R.wheel('gateA'); const onB = Y.gate('gateB'), wb = R.wheel('gateB'); R.until(() => R.up() === 'hold', 6);
        verdict(w1.part && names(sent) === names(w1.men) && still && wa.part && names(wa.men) !== names(w1.men) && onB && wb.part && R.up() === 'hold',
          'a wrong wedge [1] by the first gate sends that pair to stand there and the step stays; the line\'s own wedge on the next ring ends it, and the second gate passes',
          `wedge 1: sent ${names(sent)}, the step still up ${still} · the next ring for ${names(wa.men)} · then the ring for ${names(wb.men)} · watching the road ${names(R.watching(Y.at('gateA')))}, the back ${names(R.watching(Y.at('gateB')))} · then '${R.up()}'`); }
      { const Y = await yard(), R = Y.R, O = Y.O; R.stand(Y.at('yard')); R.step(240); Y.gate('gateA'); R.wheel('gateA'); const onB = Y.gate('gateB'), e0 = R.watching(Y.at('gateA')).length;
        R.W.cursorAt(Y.pl.char.pos); R.step(2); R.key(false); R.step(10); const called = O.men('all').every((m: any) => O.of(m)?.order === 'fallIn'), e1 = R.watching(Y.at('gateA')).length, stayed = R.up() === 'gateB'; R.step(240);
        const wb = R.wheel('gateB'), onS = Y.gate('split'), line = Y.g.view.lesson?.rails?.prompt?.line ?? '', ws = R.wheel('split'), n = R.watching(Y.at('gateB')), e = R.watching(Y.at('gateA')); R.until(() => R.up() === 'hold', 6); const onHold = R.up() === 'hold';
        R.until(() => !!Y.g.view.lesson?.finished, 120); const over = !!Y.g.view.lesson?.finished;
        verdict(onB && e0 === 2 && called && e1 === 0 && stayed && wb.part && onS && /road gate/.test(line) && ws.part && ws.line === R.stepOf('split').flatLine && n.length === 2 && e.length === 2 && onHold && over,
          'the tap on himself at the second gate calls the first gate\'s men off too: the line\'s wheel ends that step on its own gate, the next line asks for the first gate again, and its wheel leaves a pair on each for the fight',
          `the road gate first: ${e0} · the tap on himself: all fall in ${called}, on the road gate ${e1}, the step still up ${stayed} · the back gate's ring for ${names(wb.men)} · then '${onS ? 'split' : R.up()}' "${line}" · its ring for ${names(ws.men)}, "${ws.line}" · the road ${names(e)}, the back ${names(n)} · then '${onHold ? 'hold' : R.up()}', the mission's steps done ${over}`); }
    }
    // ---- 10 (a third game: the key tapped twice on the first door's step)
    { const R = await rig(), g = R.g, O = R.O, N = O.men('all').length; CMDRING.twice = true;
      R.stand(R.stepOf('corner').at); R.until(() => R.up() === 'door1', 3); const t0 = CMDRING_STATS.twice, onMark = R.mark('door1'), g0 = g.time; R.key(false); R.key(false); const dt = g.time - g0; R.step(10);
      const a = R.on('guard'), fallen = O.men('all').filter((m: any) => O.of(m)?.order === 'fallIn');
      verdict(onMark && dt < CMDRING.twiceSecs && locked(g.view, 'twice') && a.length === N && fallen.length === 0 && CMDRING_STATS.twice === t0,
        'the key tapped twice is not the lesson\'s: on the first door\'s step a second tap inside the window is the key\'s order again, and all four still stack',
        `the cursor on its ring: ${onMark} · the two taps ${dt.toFixed(2)} s apart (the window is ${CMDRING.twiceSecs} s) · the frame locks it: ${locked(g.view, 'twice')} · stacked on the door: ${names(a)} · falling in: ${fallen.length}`); }
  } finally { knobsBack(keep.knobs); RAILS.on = keep.on; AUTOPILOT.on = keep.ap; }
  console.log(all ? '=== tutqueue-check: ALL PASS ===' : '=== tutqueue-check: FAIL ===');
  return all;
}

if (import.meta.main) process.exit((await tutqueueCheck()) ? 0 : 1);
