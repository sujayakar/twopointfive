// A WALL TAKEN AT A RUN — the check behind WALLSNAP.pressRun (src/game/wallsnap.ts `pressRun` / wsPressNear, player.ts wallPress / pressFace / wallRanOn; on in the
// page, ?wallrun=0; off in the module and on the frozen stand, TPS_WALLRUN=1; it acts only with the wall taken without the key, WALLSNAP.press, and the hold's
// feel pass, WALLSNAP.on, which these lines turn on).
//   bun run tools/qa/probes.ts wallrun          (families `wallsnap`, `wall-snap`)        WALLRUN_BREAK=1 = the rule off and the same lines asked: must FAIL
// the maintainer, 11-04, from the cover lesson on the pass road: "sprinting straight into a wall should snap to cover".
// WHAT IS ASKED, at the east end of the office corridor (seven metres of run-up along it; the feel pass's own rig, wallsnap-probes.ts world) unless said:
//   run up      a sprint straight at the wall has it BEFORE he touches it (inside pressFlow), still at his sprint the frame before, no frame of the take
//               longer than a stride of that sprint, and it ends as a walk's hold ends: 0.45 m off the face, back to it, stance 'wall', nothing said
//   stays on    the run that took it, still held, is not a way off: three seconds later he is on it. Shift let go and pressed again takes him off, and
//               the key still held does not put him back; another move key with Shift down takes him off at a run; the key let go and pressed again
//               with Shift down takes him off
//   his hands   the latch is read off the keys, not off what they come to: folded by a blast with the keys still down he is on the wall after it; the view
//               turned a quarter turn with the keys still down leaves him on it; and R on the stack reloads with the run still held
//   not asked   a sprint on a diagonal into the corner, the keys let go short of the wall, a sprint into a shut door's leaf: never a hold on that wall
//   any camera  24 yaws × 8 key combinations with Shift, stood against the wall: a hold on it if and only if the stick is within pressDeg of square into it
//   the hands   the rifle stacks in his hands; the pistol goes away; with an ENEMY alert the pistol does NOT go away (no hold) and the rifle still stacks;
//               inside pressAfterShot of a round of his own, or with a mouse button held, nothing takes it; Shift in a crouch takes it, down where Shift
//               in a crouch stays down (CROUCHRUN, the page's) and stood up where it stands him up
//   a thing     the long wait is read off the rows in reach, as at a walk: the mains breaker's row is out of a sprinting man's reach, so the wall is his as
//               he arrives, there as further along it, and the row is in reach from the hold; in the crouched dash (where Shift in a crouch stays down)
//               the row is in reach and the wait is the long one, against the wall
//   the camera  its look-at changes speed in no frame of a sprint's take by more than it does in a walk's take or in a sprint the wall stops
//   off         the switch off: a sprint into the wall is a man run into a wall, a walk still takes it, and Shift takes him off a wall he walked onto
//   recordings  a recording carries the key; one from before it replays with it off; the frozen stand holds it off by name, and this stand has it as said
//   checkpoint  under SAVETIMELINE.holds a save made on a wall with the run still held loads on the wall and stays on it with the keys as they were; the same
//               file with the run cut out of the hold is a hold Shift takes him off; a hold walked onto carries no such key
//   the lesson  the pass road's cover lesson (levels/tut-road.json) played as he played it — Shift never let go from the truck straight into the post's wall:
//               not on the wall while the sprint's line is still up (a mission that teaches the wall keeps it shut until its line: rails.ts locksWall), on it
//               within half a second of 'Walk into the wall to snap to it' coming up, still on it a second and a half later with the keys held, and the
//               mission then plays to its end. Its own process, on the page's stand whichever stand asks (a tutorial is played as the page opens it), with
//               the scripted player's two steps swapped for that run (tools/qa/lessons/tut-road.ts `sprint` / `wall`: nothing of that file is changed)
// Headless: positions, clocks, the camera's look-at and the animator's stance are read; how the take LOOKS at a sprint has never been seen.
import { ROOT } from './headless.ts';
import { world } from './wallsnap-probes.ts';
const DT = 1 / 60, DEG = Math.PI / 180, R = 0.42, CZ = 11.2;
/** THE COVER LESSON WITH SHIFT NEVER LET GO (this file run with --lesson, on the page's stand): one line, `WALLRUN-LESSON {…}` */
async function lessonArm(): Promise<void> {
  const L: any = await import('./lessons/tut-road.ts'), LV: any = await import('../levels/tut-road.ts'), { runLesson } = await import('./lesson-run.ts');
  const walk0 = L.steps.wall; let early = false, openAt = -1, onAt = -1, kind = '', stayed = 0, n = 0;
  L.steps.sprint = (B: any) => { const P = B.g.player; B.go([3.2, 0, 11.1], 10); B.go([7.6, 0, LV.WALL[2]], 12, { sprint: true, near: 0.4 });
    B.hold(['ShiftLeft'], 8, () => { B.nudge([LV.POST.x0 + 0.5, 0, LV.WALL[2]], 7, () => { n++; const shut = !!B.g.rails?.locksWall?.();
      if (P.wall && shut) early = true; if (!shut && openAt < 0) openAt = n; if (P.wall && onAt < 0) { onAt = n; kind = P.wall.kind ?? 'back'; } if (onAt >= 0 && n > onAt) stayed += P.wall ? 1 : 0;
      return onAt >= 0 && n - onAt >= 90; }); return true; }); };
  L.steps.wall = (B: any) => { if (B.g.player.wall) B.step(5); else walk0(B); };   // (the line went by on the run: nothing left for it to do)
  (L.idleOk as string[] | undefined)?.push('wall');
  const out = await runLesson('tut-road', { hard: true });
  console.log(`WALLRUN-LESSON ${JSON.stringify({ early, openAt, onAt, kind, stayed, code: out.code, why: out.why, steps: out.steps.length, idleOk: Array.isArray(L.idleOk) })}`);
}
export async function wallRunCheck(): Promise<boolean> {
  const BRK = process.env.WALLRUN_BREAK === '1'; let all = true;
  const verdict = (ok: boolean, what: string, detail: string) => { all &&= ok; console.log(`- ${ok ? 'PASS' : 'FAIL'} — ${what}\n    ${detail}`); };
  console.log(`\n=== wallrun: a sprint held straight into a wall takes it${BRK ? ' (the rule off: must fail)' : ''} ===`);
  const { WALLSNAP } = await import(`${ROOT}/src/game/wallsnap.ts`); const { CROUCHRUN } = await import(`${ROOT}/src/game/player.ts`);
  const { knobsSnapshot, applyKnobs } = await import(`${ROOT}/src/game/journal.ts`); const PR: any = await import(`${ROOT}/src/host/profile.ts`);
  const was = { on: WALLSNAP.on, press: WALLSNAP.press, run: WALLSNAP.pressRun }, log = console.log, err = console.error, warn = console.warn;
  const undo: (() => void)[] = [];   // (what a line raises besides the three switches: put back at the end whether or not the line got there)
  const quiet = async <T>(f: () => T | Promise<T>): Promise<T> => { console.log = console.error = console.warn = () => { }; try { return await f(); } finally { console.log = log; console.error = err; console.warn = warn; } };
  const f2 = (x: number) => x.toFixed(2), f3 = (x: number) => x.toFixed(3);
  try {
    const S = await quiet(() => world(true)); const { pl, c, g, W } = S as any; const stood: boolean = WALLSNAP.pressRun;   // (as the stand left it: read once a game has been stood, before these lines set it)
    const set = (rule: boolean) => { WALLSNAP.on = true; WALLSNAP.press = true; WALLSNAP.pressRun = rule && !BRK; }; set(true);
    const end = g.col.raycast([29, 0.5, CZ], [1, 0, 0], 20), EX: number = 29 + (end?.t ?? NaN), sprint: number = g.tune.playerSprint;   // the corridor's east wall: its face, looking west
    const gap = () => EX - R - c.pos[0];   // what is left between his body and that wall
    const onEast = () => !!pl.wall && pl.wall.n[0] < -0.99;
    /** hold `ks` from (x, CZ) facing the wall for `frames`: the frame the east wall was taken, the gap and his stride the frame before, the frame his back was on it, the longest stride of the take, what was said */
    const run = (x: number, ks: string[], kit: 'empty' | 'pistol' | 'rifle' = 'empty', frames = 200, each?: (f: number) => void, z = CZ) => {
      S.place(x, z, 90, kit); g.messages.length = 0; S.keys(...ks); let took = -1, seated = -1, gapAt = NaN, into = 0, stride = 0, last = 0, prev = c.pos[0];
      for (let f = 0; f < frames; f++) { each?.(f); S.step(1); const st = Math.abs(c.pos[0] - prev); prev = c.pos[0];
        if (onEast()) { if (took < 0) { took = f; gapAt = gap() + st; into = last; } if (seated < 0) stride = Math.max(stride, st); if (seated < 0 && pl.wall.settle >= 1) seated = f; } else last = st; }
      const said = g.messages.map((m: any) => m.text).filter((t: string) => /wall/.test(t)); return { took, seated, gapAt, into, stride, said };
    };
    const rest = () => { S.keys(); S.step(20); if (pl.wall) { S.tap('KeyQ'); S.step(20); } };
    const RUN = ['KeyD', 'ShiftLeft'], FROM = 29;

    // ---- run up
    { const r = run(FROM, RUN); const w = pl.wall, off = gap() + R, yaw = w ? Math.abs(Math.atan2(Math.sin(c.bodyYaw - Math.atan2(w.n[0], w.n[2])), Math.cos(c.bodyYaw - Math.atan2(w.n[0], w.n[2])))) / DEG : NaN;
      const ok = Number.isFinite(EX) && r.took >= 0 && r.gapAt > 0.03 && r.gapAt <= WALLSNAP.pressFlow + 0.03 && r.into >= 0.9 * sprint * DT && r.seated >= 0 && (r.seated - r.took) * DT <= 0.3 * (WALLSNAP.slow ? WALLSNAP.slowTake : 1) && r.stride <= r.into + 1e-4
        && Math.abs(off - 0.45) <= 0.012 && yaw <= 3 && c.anim.stance === 'wall' && !pl.sprinting && !r.said.length;
      verdict(ok, 'a sprint straight at the wall has it before he touches it, at his sprint the frame before, and ends as a walk\'s hold does',
        `hold at ${r.took < 0 ? 'never' : f2(r.took * DT) + ' s'} with ${f2(r.gapAt)} m to go (pressFlow ${WALLSNAP.pressFlow}), his stride then ${f3(r.into)} m (a sprint's ${f3(sprint * DT)}) · back on the wall ${r.seated < 0 ? 'never' : f2((r.seated - r.took) * DT) + ' s later'} · longest stride of the take ${f3(r.stride)} m · ${f3(off)} m off the face, back ${f2(yaw)}° off square, stance '${c.anim.stance}' · said: ${r.said.join(' | ') || 'nothing'}`); rest(); }
    // ---- stays on, and the ways off
    { const a = run(FROM, RUN, 'empty', 290); const stayed = onEast();
      S.keys('KeyD'); S.step(6); const walkOn = onEast(); S.keys(...RUN); S.step(3); const shiftOff = !pl.wall; let back = false; for (let f = 0; f < 90; f++) { S.step(1); back ||= !!pl.wall; } rest();
      const b = run(FROM, RUN); S.keys('KeyW', 'ShiftLeft'); S.step(3); const turnOff = !pl.wall; S.step(20); const ranOn = pl.sprinting; rest();
      const d = run(FROM, RUN); S.keys('ShiftLeft'); S.step(6); const restOn = onEast(); S.keys(...RUN); S.step(3); const againOff = !pl.wall; rest();
      verdict(a.took >= 0 && stayed && walkOn && shiftOff && !back && b.took >= 0 && turnOff && ranOn && d.took >= 0 && restOn && againOff,
        'the run that took the wall is not a way off it while it is held; Shift let go and pressed again is, and so is another move key or the key let go and pressed again',
        `Shift and the key held ${f2((290 - a.took) * DT)} s after the hold: on ${stayed} → Shift up: on ${walkOn} → Shift down again: off ${shiftOff}, back on with the key still held ${back} · another key (along the wall), Shift down: off ${turnOff}, sprinting on ${ranOn} · the key let go: on ${restOn}, pressed again with Shift: off ${againOff}`); }
    // ---- his hands, not what they come to
    { const a = run(FROM, RUN); const st0 = pl.staggerUntil; undo.push(() => { pl.staggerUntil = st0; }); pl.staggerUntil = g.time + 0.6; S.step(20); const folded = pl.staggerUntil > g.time && onEast(); S.step(60); const after = onEast() && g.time > pl.staggerUntil; pl.staggerUntil = st0; rest();
      const b = run(FROM, RUN); undo.push(() => { W.cam.yaw = 0; W.cam.rebuild(); }); let turned = true; for (let k = 1; k <= 90; k++) { W.cam.yaw = k * DEG; W.cam.rebuild(); S.step(1); turned &&= onEast(); } S.step(30); turned &&= onEast(); W.cam.yaw = 0; W.cam.rebuild(); rest();
      const r = run(FROM, RUN, 'rifle'); const stack = pl.wall?.kind === 'stack'; pl.rifle.mag = Math.max(0, pl.rifle.mag - 5); S.tap('KeyR'); S.step(3); const loads = !!pl.rifle.reloading && onEast(); pl.rifle.cancelReload?.(); c.anim.cancelReload?.(); rest();
      verdict(a.took >= 0 && folded && after && b.took >= 0 && turned && r.took >= 0 && stack && loads, 'the keys still down, he stays on it through what is not his letting go: folded by a blast, the view turned a quarter turn; and R on the stack reloads with the run still held',
        `folded 0.6 s with Shift and the key down: on the wall in it ${folded}, and after it ${after} · the view turned 90° over a second and a half: on it throughout ${turned} · the rifle stacked off a run ${stack}, R with the keys still down: reloading ${loads}`); }
    // ---- not asked for
    { S.place(EX - R - 0.6, CZ, 90); S.keys('KeyD', 'KeyW', 'ShiftLeft'); let diag = false; for (let f = 0; f < 180; f++) { S.step(1); diag ||= !!pl.wall; } rest();
      let up = false; const bump = run(FROM, RUN, 'empty', 200, () => { if (!up && gap() <= WALLSNAP.pressFlow + 0.25) { up = true; S.keys(); } }); rest();
      const dr = g.doors.byName('server'); if (dr) { dr.angle = 0; dr.vel = 0; }
      S.place(17.6, 11.75, 180); S.keys('KeyW', 'ShiftLeft'); let leaf = false; for (let f = 0; f < 120; f++) { S.step(1); leaf ||= !!pl.wall && c.pos[2] > 10.06; } rest(); if (dr) { dr.angle = 0; dr.vel = 0; }
      verdict(!diag && bump.took < 0 && up && !leaf && !!dr, 'not asked for, never a hold: a sprint on a diagonal into the corner, the keys let go short of the wall, a sprint into a shut door\'s leaf',
        `diagonal for 3 s ${diag ? 'HELD' : 'no hold'} · keys up ${f2(WALLSNAP.pressFlow + 0.25)} m out ${bump.took < 0 ? 'no hold' : 'HELD'} · into the server door's leaf ${dr ? (leaf ? 'HELD' : 'no hold') : '(no door found)'}`); }
    // ---- any camera
    { const combos: string[][] = [['KeyW'], ['KeyW', 'KeyD'], ['KeyD'], ['KeyD', 'KeyS'], ['KeyS'], ['KeyS', 'KeyA'], ['KeyA'], ['KeyA', 'KeyW']]; let asked = 0, held = 0; const wrong: string[] = [];
      for (let k = 0; k < 24; k++) { const camYaw = k * 15 * DEG;
        for (const ks of combos) { W.cam.yaw = camYaw; W.cam.rebuild(); const B = W.cam.planarBasis(); let mx = 0, mz = 0;
          for (const key of ks) { const s = key === 'KeyW' || key === 'KeyD' ? 1 : -1, v = key === 'KeyW' || key === 'KeyS' ? B.fwd : B.right; mx += s * v[0]; mz += s * v[2]; }
          const l = Math.hypot(mx, mz), ang = Math.acos(Math.max(-1, Math.min(1, mx / l))) / DEG;   // from square INTO the east wall (its normal is −x)
          if (Math.abs(ang - WALLSNAP.pressDeg) < 2) continue;
          S.place(EX - R - 0.03, CZ, 0); W.cam.yaw = camYaw; W.cam.rebuild(); S.keys(...ks, 'ShiftLeft'); let took = false; for (let f = 0; f < 30; f++) { S.step(1); took ||= onEast(); } S.keys(); S.step(12); if (pl.wall) { S.tap('KeyQ'); S.step(12); }
          asked++; if (took) held++; if (took !== (ang < WALLSNAP.pressDeg)) wrong.push(`camera ${k * 15}° ${ks.map(x => x.slice(3)).join('+')} (${ang.toFixed(0)}° off square) ${took ? 'held' : 'no hold'}`); } }
      W.cam.yaw = 0; W.cam.rebuild();
      verdict(!wrong.length && held >= 24 && asked - held >= 96, 'whichever way the camera is turned: a sprint takes the wall if and only if the stick is within pressDeg of square into it', `${asked} asked, ${held} held · wrong: ${wrong.slice(0, 4).join(' · ') || 'none'}${wrong.length > 4 ? ` (+${wrong.length - 4})` : ''}`); }
    // ---- the hands
    { const rifle = run(FROM, RUN, 'rifle'); const stack = pl.wall?.kind === 'stack' && pl.rifleUp; rest();
      const pistol = run(FROM, RUN, 'pistol'); const away = !!pl.wall && pl.holstered; rest(); const backIn = !pl.holstered && pl.slot === 'pistol';
      const gd = g.guards[0], st0 = gd.state; undo.push(() => { gd.state = st0; S.input.buttons = 0; }); gd.state = 'alert';
      const pA = run(FROM, RUN, 'pistol'); const kept = !pl.holstered; rest(); const rA = run(FROM, RUN, 'rifle'); rest(); gd.state = st0;
      let fired = false; const shot = run(FROM, RUN, 'rifle', 130, () => { if (!fired && gap() < 1.5) { fired = true; pl.lastFireT = g.time; } }); let later = -1; for (let f = 0; f < 90; f++) { S.step(1); if (onEast() && later < 0) later = f; } rest();
      S.input.buttons = 4; const btn = run(FROM, RUN, 'empty', 150); S.input.buttons = 0; let upB = -1; for (let f = 0; f < 60; f++) { S.step(1); if (onEast() && upB < 0) upB = f; } rest();
      S.place(FROM + 3, CZ, 90); pl.crouch = true; S.step(10); S.keys(...RUN); let low = -1, lowAt = false; for (let f = 0; f < 240; f++) { S.step(1); if (onEast() && low < 0) { low = f; lowAt = pl.crouch; } } const lowOn = onEast() && pl.crouch; rest();
      verdict(rifle.took >= 0 && stack && pistol.took >= 0 && away && backIn && pA.took < 0 && kept && rA.took >= 0 && shot.took < 0 && fired && later >= 0 && btn.took < 0 && upB >= 0 && low >= 0 && lowAt === CROUCHRUN.on && lowOn === CROUCHRUN.on,
        'the hands at a run: the rifle stacks, the pistol goes away and comes back; an enemy alert keeps the pistol up; a round of his own or a button held holds it off; Shift in a crouch takes it, down or stood up as that Shift leaves him',
        `rifle: hold ${rifle.took >= 0}, stacked in his hands ${stack} · pistol: hold ${pistol.took >= 0}, put away ${away}, back in hand off the wall ${backIn} · an enemy alert: pistol hold ${pA.took >= 0} (still in hand ${kept}), rifle ${rA.took >= 0} · a round ${fired ? 'fired on the way in' : 'NOT FIRED'}: hold inside the wait ${shot.took >= 0}, once pressAfterShot (${WALLSNAP.pressAfterShot} s) is past ${later >= 0} · a mouse button held: hold ${btn.took >= 0}, with it let go ${upB >= 0} · crouched with Shift: hold ${low >= 0}, down as he took it ${lowAt}, down on it ${lowOn} (Shift in a crouch stays down: ${CROUCHRUN.on})`); }
    // ---- a thing to use on the wall
    { const bp: number[] = g.level.breaker.pos; let plain = -1, thing = -1, touchP = -1, touchT = -1;
      const at = (x: number) => { S.keys(); g.teleportPlayer([x, 0, bp[2] - 1.3]); c.bodyYaw = c.aimYaw = 0; pl.holstered = true; pl.crouch = false; S.setCursor([x, 1.2, bp[2] + 2]); S.step(30); };
      const z0 = () => c.pos[2];
      at(bp[0]); { S.keys('KeyS', 'ShiftLeft'); let prev = z0(); for (let f = 0; f < 150; f++) { S.step(1); if (touchT < 0 && Math.abs(z0() - prev) < 1e-4 && f > 5) touchT = f; prev = z0(); if (pl.wall && thing < 0) thing = f; } } const heldT = !!pl.wall; S.keys(); S.step(20); const row = !!pl.wall && g.nearby.some((it: any) => it.kind === 'breaker' && it.inReach); rest();
      at(bp[0] - 2.6); { S.keys('KeyS', 'ShiftLeft'); let prev = z0(); for (let f = 0; f < 150; f++) { S.step(1); if (touchP < 0 && Math.abs(z0() - prev) < 1e-4 && f > 5) touchP = f; prev = z0(); if (pl.wall && plain < 0) plain = f; } } rest();
      let dash = -1, touchD = -1, low = false; at(bp[0]); pl.crouch = true; S.step(10); { S.keys('KeyS', 'ShiftLeft'); let prev = z0(); for (let f = 0; f < 200; f++) { S.step(1); if (f === 5) low = pl.crouch; if (touchD < 0 && Math.abs(z0() - prev) < 1e-4 && f > 5) touchD = f; prev = z0(); if (pl.wall && dash < 0) dash = f; } } rest(); S.setCursor(null);
      const waited = dash >= 0 && touchD >= 0 ? (dash - touchD) * DT : NaN, dashOk = dash >= 0 && (CROUCHRUN.on ? low && waited >= WALLSNAP.pressThing - 0.12 && waited <= WALLSNAP.pressThing + 0.05 : !low);
      verdict(heldT && thing >= 0 && plain >= 0 && Math.abs(plain - thing) <= 2 && row && dashOk, 'a thing on the wall (the mains breaker): out of a sprinting man\'s reach, so the wall is his as he arrives, there as further along it, and its row is in reach from the hold; in reach in the crouched dash, which waits the long wait against the wall',
        `at the breaker, a sprint: hold at frame ${thing}${touchT >= 0 ? `, his feet still from ${touchT}` : ''}, its row in reach on the hold ${row} · 2.6 m along: hold at frame ${plain}${touchP >= 0 ? `, his feet still from ${touchP}` : ''} · at the breaker, crouched with Shift (${CROUCHRUN.on ? `stays down ${low}` : `Shift stands him up: a sprint, as above`}): hold at frame ${dash}${touchD >= 0 ? `, ${Number.isFinite(waited) ? f2(waited) : '—'} s after his feet stopped (pressThing ${WALLSNAP.pressThing})` : ''}`); }
    // ---- the camera
    { /** the worst change of the look-at's speed in one frame (m), from ten frames before `mark()` first says so to two seconds after */
      const cam = (x: number, ks: string[], rule: boolean, mark: () => boolean) => { set(rule); S.place(x, CZ, 90); S.setCursor(null); S.keys(...ks); let p = [...W.cam.target], v = [0, 0, 0], at = -1; const acc: number[] = [];
        for (let f = 0; f < 330 && (at < 0 || f < at + 120); f++) { S.step(1); const t = W.cam.target, nv = [t[0] - p[0], t[1] - p[1], t[2] - p[2]]; acc.push(Math.hypot(nv[0] - v[0], nv[1] - v[1], nv[2] - v[2])); p = [...t]; v = nv; if (at < 0 && mark()) at = f; }
        rest(); set(true); return at < 0 ? { at, worst: NaN } : { at, worst: Math.max(...acc.slice(Math.max(0, at - 10))) }; };
      const taken = cam(FROM, RUN, true, onEast), walked = cam(EX - R - 2.2, ['KeyD'], true, onEast), stopped = cam(FROM, RUN, false, () => gap() < 0.005), most = Math.max(walked.worst, stopped.worst);
      verdict(taken.at >= 0 && walked.at >= 0 && stopped.at >= 0 && taken.worst <= most * 1.1 + 0.001, 'the camera: its look-at changes speed in no frame of a sprint\'s take by more than it does in a walk\'s take or in a sprint the wall stops',
        `the worst change of the look-at's speed in a frame, from ten frames before to two seconds after: a sprint's take ${taken.at < 0 ? 'NO HOLD' : (taken.worst * 1000).toFixed(1) + ' mm'} · a walk's take ${walked.at < 0 ? 'NO HOLD' : (walked.worst * 1000).toFixed(1) + ' mm'} · a sprint stopped by the wall (the rule off) ${stopped.at < 0 ? 'NEVER STOPPED' : (stopped.worst * 1000).toFixed(1) + ' mm'}`); }
    // ---- the switch off
    { WALLSNAP.on = true; WALLSNAP.press = true; WALLSNAP.pressRun = false; const a = run(FROM, RUN, 'empty', 200); rest(); const b = run(EX - R - 1.4, ['KeyD']); const walked = onEast();
      S.keys(...RUN); S.step(3); const off = !pl.wall; let back = false; for (let f = 0; f < 60; f++) { S.step(1); back ||= !!pl.wall; } rest(); set(true);
      verdict(a.took < 0 && b.took >= 0 && walked && off && !back, 'the switch off: a sprint into the wall is a man run into a wall, a walk still takes it, and Shift takes him off a wall he walked onto', `sprint ${a.took < 0 ? 'no hold' : 'HELD'} · walk ${b.took >= 0 ? 'holds' : 'NO HOLD'} · Shift on it: off ${off}, back on with the key still held ${back}`); }
    // ---- recordings and the stands
    { const before = WALLSNAP.pressRun; WALLSNAP.pressRun = true; const snap: any = JSON.parse(JSON.stringify(knobsSnapshot(g))), has = typeof snap.tables?.WALLSNAP?.pressRun === 'boolean'; const old = JSON.parse(JSON.stringify(snap)); delete old.tables.WALLSNAP.pressRun;
      const offOld = await quiet(() => { applyKnobs(g, old); const off = WALLSNAP.pressRun === false; applyKnobs(g, snap); return off && WALLSNAP.pressRun === true; }); WALLSNAP.pressRun = before;
      const word = process.env.TPS_WALLRUN, shipped = PR.harnessProfile() === 'shipped', want = word !== undefined ? word === '1' : shipped;
      verdict(has && offOld && PR.FROZEN_HOLDS['WALLSNAP.pressRun'] === 'TPS_WALLRUN=1' && (PR.STAND_ENV as string[]).includes('TPS_WALLRUN') && stood === want, `a recording carries the switch, one from before the key replays with it off, the frozen stand holds it off by name, and this stand has it ${want ? 'on, as the page opens' : 'off, as every frozen digest was recorded'}`,
        `in the file ${has} · off for an old file ${offOld} · FROZEN_HOLDS ${PR.FROZEN_HOLDS['WALLSNAP.pressRun']} · in the stands' list ${(PR.STAND_ENV as string[]).includes('TPS_WALLRUN')} · as the stand left it ${stood} (word ${word ?? 'none'}, ${shipped ? 'shipped' : 'frozen'})`); }
    // ---- the cover lesson, Shift never let go (its own process, the page's stand)
    { const env: Record<string, string | undefined> = { ...process.env, TPS_PROFILE: 'shipped' }; delete env.WALLRUN_BREAK; if (BRK) env.TPS_WALLRUN = '0'; else delete env.TPS_WALLRUN;
      const r = Bun.spawnSync(['bun', 'run', import.meta.path, '--lesson'], { env: env as Record<string, string>, stdout: 'pipe', stderr: 'pipe' }), line = r.stdout.toString().split('\n').find(l => l.startsWith('WALLRUN-LESSON '));
      const o: any = line ? JSON.parse(line.slice(15)) : null, after = o && o.onAt >= 0 && o.openAt >= 0 ? (o.onAt - o.openAt) * DT : NaN;
      verdict(!!o && !o.early && o.onAt >= 0 && after >= 0 && after <= 0.5 && o.stayed >= 90 && o.code === 0, 'the cover lesson with Shift never let go: not on the wall before the line that teaches it, on it as that line comes up, still on it with the keys held, and the mission plays to its end',
        o ? `on the wall while its line was still shut ${o.early} · the wall's line up at frame ${o.openAt} of the run-in, on the wall (${o.kind || '—'}) ${o.onAt < 0 ? 'NEVER' : f2(after) + ' s after'} · on it ${o.stayed} of the next 90 frames · the mission: ${o.code === 0 ? `ended, ${o.steps} steps` : `exit ${o.code}: ${o.why}`}` : `no line from the lesson's run (exit ${r.exitCode}): ${r.stderr.toString().trim().split('\n').slice(-2).join(' | ').slice(0, 300)}`); }
    // ---- a checkpoint (THE LAST LINE THAT USES THE RIG'S MAN: a load makes a new Player, and world()'s place / run / gap keep the old one)
    { const SS = await import(`${ROOT}/src/game/savestate.ts`); const holds0 = SS.SAVETIMELINE.holds; undo.push(() => { SS.SAVETIMELINE.holds = holds0; }); SS.SAVETIMELINE.holds = true; const P = () => g.player;
      const save = (label: string) => quiet(() => SS.saveState(g, { label, kind: 'harness' })), load = async (file: any) => { const yaw = W.cam.yaw; await quiet(() => SS.loadState(g, JSON.parse(JSON.stringify(file)))); W.cam.yaw = yaw; W.cam.rebuild(); };
      const held = () => { let on = true; for (let f = 0; f < 60; f++) { S.step(1); on &&= !!P().wall && P().wall.n[0] < -0.99; } return on; };
      // (both saves first, the loads after: once a load has made a new Player the rig's place() is no use)
      set(true); const w = run(EX - R - 1.4, ['KeyD']); S.step(60); const walkFile: any = await save('walked onto the wall'); rest();
      const r = run(FROM, RUN, 'empty', 200); const file: any = await save('ran onto the wall'), inFile = file?.wallHold?.hold?.ran === 8, plainFile = walkFile?.wallHold?.hold !== undefined && !('ran' in walkFile.wallHold.hold);
      const carried = held();
      await load(file); S.keys(...RUN); const loaded = held();
      const cut = JSON.parse(JSON.stringify(file)); if (cut.wallHold?.hold) delete cut.wallHold.hold.ran; await load(cut); S.keys(...RUN); S.step(3); const cutOff = !P().wall; S.keys(); S.step(20);
      SS.SAVETIMELINE.holds = holds0; set(true);
      verdict(w.took >= 0 && r.took >= 0 && inFile && plainFile && carried && loaded && cutOff, 'a checkpoint made on a wall with the run still held loads on the wall and stays on it with the keys as they were; the file with the run cut out of it is a hold Shift takes him off',
        `ran on ${r.took >= 0}, the run's keys in the file's hold ${inFile}; a wall walked onto has no such key ${plainFile} · a second on with the keys held: carried on ${carried}, loaded ${loaded} · the run cut out of the file: off at once ${cutOff}`); }
  } finally { for (const u of undo.reverse()) try { u(); } catch { /* the rig is gone */ } WALLSNAP.on = was.on; WALLSNAP.press = was.press; WALLSNAP.pressRun = was.run; console.log = log; console.error = err; console.warn = warn; }
  console.log(`=== wallrun: ${all ? 'ALL PASS' : 'FAILURES above'} ===`);
  return all;
}
if (import.meta.main) { if (process.argv.includes('--lesson')) { await lessonArm(); process.exit(0); } process.exit((await wallRunCheck()) ? 0 : 1); }
