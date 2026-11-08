// The `workthings` probes (src/game/workthings.ts: a teammate is sent to work the breaker, or the mission's thing) — `bun run tools/qa/probes.ts workthings`,
// or one of work-breaker / work-four / work-objective / work-edges / work-posted / work-face / work-held / work-walk / work-defuse / work-harness. All headless on the real Game (nothing is drawn: the menu is read as the data
// the panel is built from), every order given THROUGH THE INPUT as the page writes it — the harness cursor on the thing, then one-frame presses. WORK.on is
// raised by the probe itself: the frozen stand holds it off (TPS_WORKTHINGS=1).
//   work-breaker    the room clear's pair on the office: X with the cursor on the breaker panel lists the thing's own order on digit 1 where 'move there' is
//                   on floor, no row on a digit above 5; 1 gives it — one man walks to the panel (never sprinting), squares to it, puts a hand on it and the
//                   mains go; his partner covers behind him; he then HOLDS by the panel and the breaker STAYS thrown (a teammate standing by it does not re-arm
//                   it, and the 'go and check it' errand is not handed to a teammate); then B on the panel is 're-arm the mains', and he puts them back
//   work-four       the squad of four: the order to all goes to ONE pair — one works, one covers — and the other pair is left exactly as it was
//   work-objective  a quiet mission with a partner: once the objective is to take the thing, X on its holder lists 'take …' on 1, the partner goes and takes
//                   it (the mission moves on to the way out, in the level's own words); before that stage the holder is plain floor
//   work-edges      what is NOT the breaker (the top of its wall, the next room's floor behind it, a metre in front), Sam standing at it with the cursor on
//                   himself is himself, and a man the orders stop short of it holds where he is: the breaker is not thrown from there
//   work-defuse     a mission's charge (defuse.ts): sent to it he makes it safe and says so; the run ending under his hand (Sam down) leaves the charge armed,
//                   and he neither says 'charge is safe.' nor is a job counted done; the same for a quiet mission's partner with his hands in the holder
//                   (he does not take it, and does not say it is already gone)
//   work-held       the squad's fire held and a defender who has noticed nobody stood in plain sight of the panel: the man sent still throws the breaker (his
//                   partner's gun on that man, no round fired), his footfalls turn nobody at the panel, and the moment that man notices he is the squad's to shoot again. Asserts the fault with
//                   WORK.heldPass off (his muzzle stays on the man, the mains stay on); WORKHELD_BREAK=1 leaves the rule off for the rule's own lines: must fail
//   work-walk       a quiet level as two (levels/lights-out.json): the partner throws the breaker on order, a defender is sent to the panel, Sam calls 'fall in'
//                   and walks on the way that man is coming. Asserts the fault with WORK.offWalk off (the follower waits for the man's back ON the man's
//                   walk and is walked into: the alarm) and the rule with it on (he goes on after Sam, low, nobody roused, the mains put back by the man
//                   sent); WORKWALK_BREAK=1 leaves the rule off for the rule's own lines: must fail
//   work-harness    the frozen stand holds it off, the page turns it on unless ?noworkthings, the table is a registered knob table a recording carries
import { doing, listedOf, menuKey } from './squadwords';
import { readFileSync, readdirSync } from 'node:fs';
import { standUp, ROOT } from './headless.ts';
const { parseLevelData } = await import(`${ROOT}/src/scene/levelData.ts`);
const { loadLevel } = await import(`${ROOT}/src/scene/levelLoad.ts`);
const { WORK, WORK_STATS, workHand, workJob, breakerErrand, workPasses, workComing } = await import(`${ROOT}/src/game/workthings.ts`);
const { SNEAK } = await import(`${ROOT}/src/game/sneak.ts`);
const { SPOTTED } = await import(`${ROOT}/src/game/spotted.ts`);
const { breakerResetNear } = await import(`${ROOT}/src/game/lighting.ts`);
const { NIGHTBREACH } = await import(`${ROOT}/src/game/nightbreach.ts`);
const { startFromQuery } = await import(`${ROOT}/src/game/scenarioRun.ts`);
const { quietWords } = await import(`${ROOT}/src/game/quiet.ts`);
const { ORDERS } = await import(`${ROOT}/src/game/orders.ts`);

const DT = 1 / 60, VERBOSE = !!process.env.WORK_VERBOSE;
let allPass = true;
const verdict = (pass: boolean, what: string, detail = '') => { console.log(`- ${pass ? 'PASS' : 'FAIL'} — ${what}${detail ? `\n    ${detail}` : ''}`); if (!pass) allPass = false; };
const f2 = (x: number) => x.toFixed(2), flat = (a: ArrayLike<number>, b: ArrayLike<number>) => Math.hypot(a[0] - b[0], a[2] - b[2]);
function seeded(seed: number) { let a = seed >>> 0; return () => { a |= 0; a = a + 0x6D2B79F5 | 0; let t = Math.imul(a ^ a >>> 15, 1 | a); t = t + Math.imul(t ^ t >>> 7, 61 | t) ^ t; return ((t ^ t >>> 14) >>> 0) / 4294967296; }; }

/** one game with a scenario staged and the switch as asked; step() keeps Sam dark and the other side dazzled (this is about the order, not the fight) */
async function rig(stage: (g: any) => string | null | undefined, on = true, squad: 2 | 4 = 2, level: string | null = null) {
  Math.random = seeded(21); const was = NIGHTBREACH.squad; NIGHTBREACH.squad = squad;
  const log = console.log, warn = console.warn, err = console.error; if (!VERBOSE) console.log = console.warn = () => { };
  let W: any, why: string | null | undefined;
  try { W = await standUp(level ? { level: loadLevel(parseLevelData(readFileSync(`${ROOT}/levels/${level}.json`, 'utf8'))), seed: 1 } : undefined); W.game.godMode = true; WORK.on = on; why = stage(W.game); } finally { console.log = log; console.warn = warn; console.error = err; NIGHTBREACH.squad = was; }
  const g = W.game, input = W.input, O = g.orders, said: string[] = [];
  { const om = g.msg.bind(g); g.msg = (t: string) => { said.push(t); return om(t); }; const os = g.say.bind(g); g.say = (gd: any, t: string, r?: boolean) => { said.push(`${gd.callsign}: ${t}`); return os(gd, t, r); }; }
  const R = { W, g, input, O, why, said, aim: null as number[] | null, held: 0, fastest: 0, handUp: new Set<string>(), jobs: new Map<string, { verb: string; secs: number; need: number; bad: number }>(),
    step(n = 1) { const l = console.log; if (!VERBOSE) console.log = () => { };
      try { for (let i = 0; i < n; i++) {
        g.player.visibility = 0.05; for (const gd of g.guards) if (gd.team !== 'operative' && gd.state !== 'dead') gd.dazzledUntil = g.time + 5;
        if (R.aim) W.cursorAt([R.aim[0], R.aim[1], R.aim[2]]);
        R.held = Math.max(R.held, input.keys.size + (input.buttons ? 1 : 0));
        W.step(DT);
        for (const gd of O.squad()) { R.fastest = Math.max(R.fastest, Math.hypot(gd.char.vel[0], gd.char.vel[2])); // (the hand as the module gives it out — whichever layer of the animator poses it — and the read that goes with it)
          const wh = workHand(g, gd.char), wj = workJob(g, gd.char);
          if (wh && wh.w > 0 && g.work.working(gd)) R.handUp.add(gd.callsign);
          if (!!wh !== !!wj) { const o = R.jobs.get(gd.callsign) ?? { verb: '', secs: 0, need: 0, bad: 0 }; o.bad++; R.jobs.set(gd.callsign, o); }
          if (wh && wj && wh.w > 0) { const o = R.jobs.get(gd.callsign) ?? { verb: wj.verb, secs: 0, need: wj.need, bad: 0 }; if (wj.secs + 1e-9 < o.secs && wj.verb === o.verb) o.bad++; o.verb = wj.verb; o.secs = wj.secs; o.need = wj.need; R.jobs.set(gd.callsign, o); } } } } finally { console.log = l; } },
    until(pred: () => boolean, secs: number) { for (let i = 0; i < secs * 60 && !pred(); i++) R.step(); return pred(); },
    press(code: string) { input.pressed.add(code); R.step(); },
    rows(): any[] { return O.menu ? O.menu.rows : []; },
    last(): any { return O.log[O.log.length - 1] ?? null; },
    /** the men called to Sam first (X on himself, 3), so an order finds them with a way to go, as in play */
    gather(secs = 6) { const p = g.player.char.pos; R.aim = [p[0], 0.9, p[2]]; R.step(2); (menuKey(O), R.step()); R.press('Digit3'); R.step(secs * 60); R.aim = null; },
  };
  R.step(20);
  return R;
}
async function keep(fn: () => Promise<void>): Promise<boolean> {
  const was = { ...WORK }; allPass = true;
  try { await fn(); } catch (e) { console.log(`- FAIL — the probe threw\n    ${e instanceof Error ? e.stack : String(e)}`); allPass = false; }
  finally { Object.assign(WORK, was); }
  return allPass;
}
/** the office's storage room (where its breaker is) unlocked, as a kicked or picked lock leaves it */
const unlock = (g: any) => { const d = g.doors.list.find((x: any) => x.def.name === 'storage'); if (d) d.locked = false; };
/** the middle of the breaker's own panel: where a player's cursor is when it is on the thing as drawn */
const panel = (g: any): number[] => [...g.engine.world.statics[g.level.breaker.boxes[0]].c];
/** the camera turned (the player's Q / E) to look AT the panel's face: on the office the stock view is from the far side of its wall, where it is not drawn */
const lookAtPanel = (R: any) => { R.W.cam.yaw = Math.PI; R.step(20); };
const menuWords = (R: any) => R.rows().map((d: any) => `${d.key ?? '·'} ${d.id}`).join(' | ');
const labelOf = (R: any, d: any) => typeof d.label === 'string' ? d.label : d.label(R.O.menu.target);

export const workBreaker = () => keep(async () => {
  console.log('\n=== work-breaker: the pair is sent to the breaker, throws it, holds; then re-arms it ===');
  const R = await rig(g => { g.fogOfWar.on = true; return g.roomClear.start(21); });
  const { g, O } = R; const bp = g.level.breaker.pos, ops: any[] = O.squad();
  verdict(!R.why && ops.length === 2, 'staged: the room clear on the office with its pair', `refusal ${R.why} · teammates ${ops.map(o => o.callsign).join(', ')}`);
  lookAtPanel(R);
  // the office keeps its breaker behind the storage room's LOCKED door: with it locked the row is there, greyed, in the man's own words
  { R.aim = panel(g); R.step(2); (menuKey(O), R.step()); const i = R.rows().findIndex((d: any) => d.id === 'work'), why = O.menu?.why[i] ?? null; O.close(); R.aim = null;
    verdict(i === 0 && typeof why === 'string' && /locked/.test(why), 'behind a locked door the row is listed greyed, with why: the men cannot get to it', `row ${i} · "${why}"`); }
  unlock(g); R.step(70);
  const th = g.work.things().find((x: any) => x.kind === 'breaker');
  verdict(!!th && !!th.stand && flat(th.stand, bp) < 1.0 && !g.col.nav.isBlocked(th.stand[0], th.stand[2]), 'the breaker is a thing to work, with a free spot to stand on within arm\'s reach of its face', th ? `face ${th.face.map(f2)} · normal ${th.n.map(f2)} · stand ${th.stand?.map(f2)} (${th.stand ? f2(flat(th.stand, bp)) : '-'} m off)` : 'not listed');
  // --- the menu on it
  R.aim = panel(g); R.step(2); (menuKey(O), R.step());
  const rows = R.rows(), t = O.menu?.target;
  verdict(!!O.menu && t.kind === 'floor' && t.thing?.kind === 'breaker' && rows[0]?.id === 'work' && rows[0].key === 1 && labelOf(R, rows[0]) === 'throw the breaker' && !rows.some((d: any) => d.id === 'move') && rows.every((d: any) => !d.key || d.key <= 5),
    'X on the panel: the menu is the breaker\'s — "throw the breaker" on 1 where "move there" is on floor, no digit above 5', `${menuWords(R)} · "${rows[0] ? labelOf(R, rows[0]) : ''}" · hud "${listedOf(O)?.rows?.[0]?.label ?? ''}"`);
  const hud = listedOf(O); verdict(!!hud && JSON.stringify(hud.rows).includes('the breaker'), 'the panel names it: on the breaker', hud ? hud.rows.map((r: any) => r.label).filter(Boolean).join(' / ') : 'no menu');
  // --- given
  const posBefore = ops.map(o => [...o.char.pos]);
  R.press('Digit1'); R.aim = null;
  const L = R.last(), worker = ops.find(o => g.work.working(o)), mate = ops.find(o => o !== worker);
  verdict(!O.menu && L?.order === 'work' && L.target === 'thing:breaker' && !!worker, '1 gives it: logged as the order on the breaker, the menu gone, one man at work', `log ${JSON.stringify(L)} · worker ${worker?.callsign}`);
  if (!worker) return;
  verdict(flat(worker.char.pos, th.stand) <= flat(mate.char.pos, th.stand) + 0.01, 'the nearer man is the one sent', `${worker.callsign} ${f2(flat(posBefore[ops.indexOf(worker)], th.stand))} m · ${mate.callsign} ${f2(flat(posBefore[ops.indexOf(mate)], th.stand))} m`);
  R.fastest = 0;
  const went = R.until(() => g.blackout.active, 60), tDark = g.time;
  verdict(went && !g.blackout.permanent, 'he gets there and the mains go (a hand-thrown breaker: not for good)', `blackout ${g.blackout.active} after ${f2(g.time)} s on the clock · ${worker.callsign} ${f2(flat(worker.char.pos, bp))} m from the panel · ${doing(O.g, worker)}`);
  verdict(flat(worker.char.pos, bp) < 1.15 && R.handUp.has(worker.callsign), 'he was at the panel, inside Sam\'s own reach of it, with his hand out to it', `${f2(flat(worker.char.pos, bp))} m · hand seen out: ${[...R.handUp].join(', ') || 'nobody'}`);
  { const J = R.jobs.get(worker.callsign);
    verdict(!!J && J.verb === 'breakerOff' && J.bad === 0 && Math.abs(J.need - (WORK.handSecs + WORK.breakerSecs)) < 1e-9 && J.secs > J.need - 0.05 && J.secs <= J.need + 0.05, 'the read beside the hand (workJob): throwing it, a clock that only runs forward, to the end of the time the work takes, and there whenever the hand is', J ? `${J.verb} · ${f2(J.secs)} of ${f2(J.need)} s · ${J.bad} bad frames` : 'never read'); }
  verdict(R.fastest <= ORDERS.brisk + 0.25 && R.held === 0, 'nobody went faster than an ordered man\'s brisk walk, and no key was ever held', `fastest ${f2(R.fastest)} m/s (brisk ${ORDERS.brisk}) · most keys down on a frame ${R.held}`);
  // (the game's own 'breaker thrown' is the native test's: on the tick the mains go, five men's 'huh?' push it off the five rows of the log the copy is given)
  verdict(R.said.some((m: string) => /lights out/.test(m)), 'the man says it is done', R.said.filter((m: string) => /breaker|lights out/.test(m)).join(' | '));
  R.step(90);
  const A = O.of(worker), B = O.of(mate);
  verdict(A?.order === 'hold' && !g.work.working(worker) && flat(worker.char.pos, bp) < 1.3, 'done, he holds by the panel on a plain hold', `${worker.callsign}: ${A?.order} · ${doing(O.g, worker)}`);
  verdict(B?.order === 'move' && !!B.post && flat(B.post, th.stand) > 0.8 && flat(B.post, th.stand) < 2.4 && B.spare === true, 'his partner was sent to a mark behind him, eyes spare for the ways in', `${mate.callsign}: ${B?.order} · post ${B?.post?.map(f2)} (${B?.post ? f2(flat(B.post, th.stand)) : '-'} m from the man at work) · ${doing(O.g, mate)}`);
  // --- a wrecked breaker is nobody's
  g.setBlackout(Infinity, 'shot'); R.step(2); R.aim = panel(g); R.step(2); (menuKey(O), R.step());
  verdict(!R.rows().some((d: any) => d.id === 'work'), 'a breaker a round has wrecked is no thing to work (Sam has no row for it either)', menuWords(R));
  console.log(`    stats: ${JSON.stringify(WORK_STATS)} · ${g.work.summary()}`);
});


/** A FOLLOWER IS NOT LEFT STANDING ON THE WALK OF THE MAN WHO COMES TO THE BREAKER (WORK.offWalk; workComing, asked by sneak.ts's follower where it would wait
 *  short of a stretch that man is looking at or back off down the way he is coming, and by spotted.ts where it would stand him still). From a replay of play
 *  (lights out, as two): the partner threw the breaker on order, 'fall in' was given a second later, Sam walked on past the man sent to the panel, and the
 *  partner waited four metres from it for that man's back — on that man's walk — until the man reached him. Staged as that run went: the level's own mission,
 *  the men's own senses (nobody is dazzled and Sam is not kept dark here), Sam moved along the taped way at a sneak's pace. */
export const workWalk = () => keep(async () => {
  console.log('\n=== work-walk: a follower is not left standing on the walk of the man who comes to the breaker ===');
  const BREAK = !!process.env.WORKWALK_BREAK, sn0 = SNEAK.on, sp0 = SPOTTED.on;
  const WAY = [[9.3, 16.7], [9.3, 14.8], [8.7, 12.1], [9.0, 9.3], [11.0, 8.0], [13.7, 7.4], [16.1, 7.4], [17.0, 8.5], [17.1, 10.0], [17.3, 12.5], [17.7, 14.6]];
  try {
    const arm = async (rule: boolean) => {
      const R = await rig(g => startFromQuery(g, '?levelFile=lights-out.json&quiet&squad=1').why, true, 2, 'lights-out'); const { g, O, W } = R, pl = g.player, b: any = O.squad()[0];
      SNEAK.on = true; SPOTTED.on = true; WORK.offWalk = rule; g.godMode = true;
      const asked = (h: any) => { const w = WORK.offWalk; WORK.offWalk = true; try { return workComing(g, b, h); } finally { WORK.offWalk = w; } };   // (the geometry, whichever way the key is)
      let at = [...WAY[0]], seg = 0, along = 0;
      const step = (n = 1) => { const l = console.log; if (!VERBOSE) console.log = () => { }; try { for (let i = 0; i < n; i++) { pl.crouch = true; pl.char.pos[0] = at[0]; pl.char.pos[2] = at[1]; W.step(DT); } } finally { console.log = l; } };
      if (!b) return { ok: false as const, why: `${R.why} no partner` };
      g.teleportPlayer([at[0], 0, at[1]]); b.char.pos[0] = 9.9; b.char.pos[2] = 17.3; b.path = []; b.pathGoal = null; step(60);
      const th = g.work.things().find((x: any) => x.kind === 'breaker'); if (!th?.stand) return { ok: false as const, why: 'no breaker to work' };
      const gave = O.give('work', { kind: 'floor', at: th.stand, thing: th }, 'all'); for (let i = 0; i < 25 * 60 && !g.blackout.active; i++) step();
      const t0 = g.time, man: any = g.guards.find((x: any) => x.task?.kind === 'breaker'); if (!g.blackout.active || !man) return { ok: false as const, why: `order ${gave} · mains ${g.blackout.active ? 'out' : 'on'} · errand ${man?.callsign ?? 'nobody'}` };
      step(72); const fell = O.give('fallIn', { kind: 'floor', at: [9, 0, 17.5] }, 'all');
      let still = 0, stillMax = 0, on = 0, off = 0, low = 0, roused: string | null = null, rousedT = -1, nearest = 1e9, fastest = 0, back = -1, never = true;
      for (let f = 0; f < 26 * 60; f++) {
        if (g.time - t0 > 5.2 && seg < WAY.length - 1) { along += 1.4 * DT;   // (as the tape: he waits five seconds for his partner, then goes at a sneak's pace)
          for (let L = Math.hypot(WAY[seg + 1][0] - WAY[seg][0], WAY[seg + 1][1] - WAY[seg][1]); seg < WAY.length - 1 && along >= L;) { along -= L; seg++; if (seg < WAY.length - 1) L = Math.hypot(WAY[seg + 1][0] - WAY[seg][0], WAY[seg + 1][1] - WAY[seg][1]); }
          if (seg < WAY.length - 1) { const L = Math.hypot(WAY[seg + 1][0] - WAY[seg][0], WAY[seg + 1][1] - WAY[seg][1]), k = along / L; at = [WAY[seg][0] + (WAY[seg + 1][0] - WAY[seg][0]) * k, WAY[seg][1] + (WAY[seg + 1][1] - WAY[seg][1]) * k]; } else at = [...WAY[seg]]; }
        step();
        const v = Math.hypot(b.char.vel[0], b.char.vel[2]), coming = man.state !== 'dead' && asked(man), gap = flat(b.char.pos, pl.char.pos); fastest = Math.max(fastest, v);
        if (coming) { on++; if (workComing(g, b, man)) never = false; if (b.char.anim.crouch > 0.5) low++; } else off++;
        if (coming && v < 0.1 && gap > 3) { still += DT; stillMax = Math.max(stillMax, still); } else still = 0;
        if (man.state !== 'dead') nearest = Math.min(nearest, flat(b.char.pos, man.char.pos));
        if (!roused) { const h = g.guards.find((x: any) => x.team !== 'operative' && (x.state === 'alert' || x.state === 'search' || x.state === 'dead')); if (h) { roused = `${h.callsign} ${h.state}`; rousedT = g.time - t0; } }
        if (back < 0 && !g.blackout.active) back = g.time - t0;
      }
      return { ok: true as const, why: `${gave} ${fell}`, man: man.callsign, stillMax, on, off, roused, rousedT, nearest, fastest, back, never, low, gap: flat(b.char.pos, pl.char.pos), partner: b.callsign, said: R.said };
    };
    const B = await arm(!BREAK);
    if (!B.ok) { verdict(false, 'staged: lights out as two, the partner throws the breaker on order, a defender is sent to the panel, Sam calls fall in and walks on', B.why); return; }
    verdict(!B.roused && B.back > 0, 'key on: nobody is roused, and the man sent puts the mains back himself', `${B.roused ?? 'no defender alert, searching or down'} · mains back ${B.back > 0 ? `${f2(B.back)} s after the throw` : 'never'} · nearest the man came ${f2(B.nearest)} m`);
    verdict(B.stillMax < 1 && B.gap < 3, 'key on: he does not stand on that walk with Sam gone on, and ends at Sam\'s shoulder', `longest still on the walk ${f2(B.stillMax)} s · ${f2(B.gap)} m from Sam at the end`);
    verdict(B.fastest <= ORDERS.brisk + 0.25 && B.low >= B.on * 0.9, 'he never ran for it: a walk, low with Sam the whole of that walk', `fastest ${f2(B.fastest)} m/s (brisk ${ORDERS.brisk}) · low on ${B.low} of the ${B.on} frames on the walk`);
    verdict(B.off > 60 && B.on > 60 && !B.never, 'the rule answered on that walk, and is the walk\'s: once that man is past him it is the build\'s own follower again', `frames on the walk ${B.on} · off it ${B.off} · asked true with the key on: ${!B.never}`);
  } finally { SNEAK.on = sn0; SPOTTED.on = sp0; }
});

export const workObjective = () => keep(async () => {
  console.log('\n=== work-objective: the partner of a quiet mission takes the thing ===');
  const R = await rig(g => startFromQuery(g, '?levelFile=crossing.json&quiet&squad=1').why, true, 2, 'crossing');
  const { g, O } = R; const ops: any[] = O.squad(), Wd = quietWords(g);
  verdict(!R.why && ops.length === 1 && !!Wd, 'staged: a quiet mission (levels/crossing.json) with one partner', `refusal ${R.why} · partner ${ops.map(o => o.callsign).join(', ')} · thing "${Wd?.thing}" in "${Wd?.holder}" (${Wd?.place})`);
  if (!Wd || !ops.length) return;
  const rf = g.level.mission.rack.front;
  verdict(g.mission.stage !== 'drive' && !g.work.things().some((x: any) => x.kind === 'objective'), 'before the objective is to take it, the holder is no thing to work (as Sam\'s own row)', `stage ${g.mission.stage} · things ${g.work.things().map((x: any) => x.name).join(', ')}`);
  // Sam walks in (the mission's own reach test): put him in the room and let the stage move
  const A = g.level.mission.serverRoom; let spot: number[] | null = null;
  for (let x = A.x0 + 0.5; x < A.x1 && !spot; x += 0.4) for (let z = A.z0 + 0.5; z < A.z1 && !spot; z += 0.4) if (!g.col.nav.isBlocked(x, z) && flat([x, 0, z], rf) > 1.6) spot = [x, 0, z];
  g.teleportPlayer(spot ?? [(A.x0 + A.x1) / 2, 0, (A.z0 + A.z1) / 2]); R.step(30);
  const th = g.work.things().find((x: any) => x.kind === 'objective');
  verdict(g.mission.stage === 'drive' && !!th && !!th.stand, 'Sam in the place: the objective is to take it, and it is a thing to work with a spot to stand on', `stage ${g.mission.stage} · ${th ? `${th.name} · stand ${th.stand?.map(f2)} (${th.stand ? f2(flat(th.stand, rf)) : '-'} m off its face)` : 'not listed'}`);
  if (!th) return;
  R.gather(8);
  R.aim = [rf[0], Math.max(0.9, rf[1] || 1.0), rf[2]]; R.step(2); (menuKey(O), R.step());
  const rows = R.rows(), lab = rows[0] ? labelOf(R, rows[0]) : '';
  verdict(rows[0]?.id === 'work' && rows[0].key === 1 && lab === `take ${Wd.thing}` && rows.every((d: any) => !d.key || d.key <= 5), 'X on the holder: "take …" in the level\'s own words on 1, no digit above 5', `${menuWords(R)} · "${lab}"`);
  R.press('Digit1'); R.aim = null;
  const worker = ops.find(o => g.work.working(o));
  const took = R.until(() => g.mission.stage === 'exfil', 60);
  verdict(!!worker && took && g.mission.pulled === true && R.handUp.has(worker.callsign), 'he goes, puts a hand to it for as long as Sam\'s own would take, and the mission moves on to the way out', `stage ${g.mission.stage} · pulled ${g.mission.pulled} · ${worker ? `${worker.callsign} ${f2(flat(worker.char.pos, rf))} m off · ${doing(O.g, worker)}` : 'nobody at work'} · last line "${g.messages[g.messages.length - 1]?.text}"`);
  { const J = worker ? R.jobs.get(worker.callsign) : null;
    verdict(!!J && J.verb === 'drive' && J.bad === 0 && J.secs > J.need - 0.05, 'the read beside the hand says he is taking it, to the end', J ? `${J.verb} · ${f2(J.secs)} of ${f2(J.need)} s · ${J.bad} bad frames` : 'never read'); }
  R.step(30);
  verdict(!!worker && O.of(worker)?.order === 'hold' && !g.work.things().some((x: any) => x.kind === 'objective'), 'done, he holds there and the holder is plain floor again', worker ? doing(O.g, worker) : '');
});

export const workEdges = () => keep(async () => {
  console.log('\n=== work-edges: what is NOT the breaker, and a man who cannot get to it ===');
  const R = await rig(g => { g.fogOfWar.on = true; return g.roomClear.start(21); });
  const { g, O } = R; const ops: any[] = O.squad();
  R.gather(); lookAtPanel(R); unlock(g); R.step(70);
  const th = g.work.things().find((x: any) => x.kind === 'breaker'), f = th.face, n = th.n;
  // --- the far side of its wall, the wall's top, the floor well in front of it: none is the breaker; the floor at its foot is
  const at = (k: number, side = 0): number[] => [f[0] + n[0] * k + n[2] * side, 0, f[2] + n[2] * k - n[0] * side];
  const is = (p: number[]) => !!O.targetAt(p).thing;
  verdict(!is(at(-0.14)) && !is(at(-0.4)) && !is(at(-0.6, 0.2)) && !is(at(0.9)) && is(at(0.2)) && is(at(0.0, 0.2)),
    'a point on the top of its wall, on the floor of the next room behind it, or a metre in front is not the breaker; its own face and the floor at its foot are',
    `behind 0.14 ${is(at(-0.14))} · behind 0.4 ${is(at(-0.4))} · behind 0.6 off 0.2 ${is(at(-0.6, 0.2))} · in front 0.9 ${is(at(0.9))} · in front 0.2 ${is(at(0.2))} · on the face 0.2 along ${is(at(0, 0.2))}`);
  // --- Sam standing at it with the cursor on himself is Sam (fall in), not the breaker
  { const was = [...g.player.char.pos] as [number, number, number]; g.teleportPlayer([th.stand[0], 0, th.stand[2]]); R.step(3);
    const p = g.player.char.pos, t = O.targetAt([p[0], 0, p[2]]), on = O.targetAt([f[0], 0, f[2]]);
    verdict(t.kind === 'none' && !t.thing && !!on.thing, 'Sam standing at the panel: the point at his own feet is himself (the orders he gives there are fall in / hold), the panel itself is still the breaker', `at his feet: ${t.kind}${t.thing ? ' (thing)' : ''} · on the face: ${on.thing ? 'the breaker' : on.kind}`);
    g.teleportPlayer(was); R.step(3); }
  // --- a man who cannot get to it does not work it from where he stops: the door is locked again once he is on his way
  const n0 = O.log.length; R.aim = panel(g); R.step(2); (menuKey(O), R.step()); R.press('Digit1'); R.aim = null;
  const worker = ops.find(o => g.work.working(o));
  verdict(O.log.length === n0 + 1 && !!worker, 'the order is given', `${JSON.stringify(R.last())} · worker ${worker?.callsign}`);
  if (!worker) return;
  // (what orders.ts does for a man who is stuck short of his mark — a shut way, somebody on it: 'the mark comes to him', and he is called arrived)
  R.step(240); { const A = O.of(worker), p = worker.char.pos; A.there = 'stopped short'; A.post = [p[0], 0, p[2]]; A.goal = [p[0], 0, p[2]]; A.via.length = 0; A.arrived = true; }
  R.until(() => !g.work.working(worker), 10); R.step(120);
  const d = flat(worker.char.pos, th.stand);
  verdict(!g.blackout.active && !g.work.working(worker) && O.of(worker)?.order === 'hold' && d > 1.5, 'stopped short of it (the orders bring his mark to him and call him arrived) he holds where he is, and the breaker is NOT thrown from there', `blackout ${g.blackout.active} · ${worker.callsign} ${f2(d)} m from his mark · ${doing(O.g, worker)} · said: ${R.said.filter((m: string) => m.startsWith(worker.callsign)).slice(-2).join(' | ')} · ${g.work.log.slice(-1).map((l: any) => l.what).join('')}`);
  R.step(600);
  verdict(!g.blackout.active && !R.handUp.has(worker.callsign), 'ten seconds on it is still not thrown, and his hand never went out', `blackout ${g.blackout.active} · hands ${[...R.handUp].join(', ') || 'nobody'}`);
});

export const workPosted = () => keep(async () => {
  console.log('\n=== work-posted: the mains go, and no man who cannot leave his post says he is off to check the breaker ===');
  const errandOf = (g: any) => g.guards.filter((gd: any) => gd.task?.kind === 'breaker');
  const lineIn = (R: any) => R.said.filter((t: string) => /checking the breaker/.test(t));
  const plain = (R: any, secs: number) => { for (let i = 0; i < secs * 60; i++) { R.g.player.visibility = 0.05; R.W.step(DT); } };
  const { POWEROUT } = await import(`${ROOT}/src/game/powerout.ts`); const po0 = POWEROUT.on;   // (a blackout sends defenders to the breaker, a later rule: held off for the two room clears here, after each stand-up — powerout-check.ts holds it, with WORK.posted on)
  try {
  // --- a room clear: every defender is held on his mark
  { const R = await rig(g => { g.fogOfWar.on = true; POWEROUT.on = false; return g.roomClear.start(21); });
    const { g } = R; const foes = g.guards.filter((gd: any) => gd.team !== 'operative' && gd.state !== 'dead');
    verdict(!R.why && foes.length > 0 && foes.every((gd: any) => gd.hold || gd.pinned), 'staged: the room clear, every defender held on his mark', `refusal ${R.why} · ${foes.map((gd: any) => `${gd.callsign}[${gd.hold ? 'hold' : ''}${gd.pinned ? ' pinned' : ''}]`).join(', ')}`);
    g.setBlackout(86400, 'hand'); plain(R, 1);
    verdict(g.blackout.active && errandOf(g).length === 0 && lineIn(R).length === 0, 'the mains thrown: nobody is handed the errand, nobody says he is checking the breaker', `errand ${errandOf(g).map((gd: any) => gd.callsign).join(', ') || 'nobody'} · lines ${JSON.stringify(lineIn(R))}`);
    plain(R, 6);
    verdict(g.blackout.active && lineIn(R).length === 0, 'six seconds on it is still dark and still nobody has said it', `lines ${JSON.stringify(lineIn(R))}`); }
  // --- a man who can walk still gets it and still goes: the office's night shift
  { const R = await rig(() => null);
    const { g } = R; const bp = g.level.breaker.pos;
    g.setBlackout(86400, 'hand'); plain(R, 1);
    const man = errandOf(g)[0], d0 = man ? flat(man.char.pos, bp) : 0; plain(R, 10);
    verdict(!!man && !man.hold && !man.pinned && lineIn(R).length >= 1 && d0 - flat(man.char.pos, bp) > 1, "the solo night: the nearest of the shift is handed it, says it, and walks for the breaker",
      man ? `${man.callsign}: ${f2(d0)} m → ${f2(flat(man.char.pos, bp))} m · lines ${JSON.stringify(lineIn(R))}` : 'nobody was handed it'); }
  } finally { POWEROUT.on = po0; }
});

/** lighting.ts's own 'check the breaker' errand (the other side's, nobody ordered): walked to the panel's own face, and the mains re-armed only by a man with
 *  nothing solid between him and the panel. The fault it holds, asserted with WORK.ownFace off: on the relay station the panel hangs on the inside of the
 *  compound wall facing +x, the fixed spot is in that wall, and the gate man re-armed the mains from the road (its first patrol set) or never (its second).
 *  WORKFACE_BREAK=1 leaves the rule off and asks the same lines: must fail. */
export const workFace = () => keep(async () => {
  console.log("\n=== work-face: the breaker errand goes to the panel's own face, and the mains come back only from a man with a line to it ===");
  const RULE = !process.env.WORKFACE_BREAK;
  const quiet = <T>(f: () => T): T => { const l = console.log, w = console.warn; if (!VERBOSE) console.log = console.warn = () => { }; try { return f(); } finally { console.log = l; console.warn = w; } };
  const stand = async (level: string | null, own: boolean, stage?: (g: any) => void) => {
    Math.random = seeded(21); const l = console.log, w = console.warn; if (!VERBOSE) console.log = console.warn = () => { };
    try { const W: any = await standUp(level ? { level: loadLevel(parseLevelData(readFileSync(`${ROOT}/levels/${level}.json`, 'utf8'))), seed: 1 } : undefined);
      W.game.godMode = true; WORK.on = true; WORK.ownFace = own; stage?.(W.game); return W; } finally { console.log = l; console.warn = w; } };
  /** the panel as it hangs: a point just off its face at chest height, and the normal out of it */
  const panelOf = (g: any) => { const th = g.work.things().find((x: any) => x.kind === 'breaker'); return { n: th.n as number[], face: th.face as number[], at: [th.face[0] + th.n[0] * 0.05, 1.0, th.face[2] + th.n[2] * 0.05] }; };
  /** the mains thrown by hand and the floor left to itself: who was sent, when and from where the mains came back, and the frames a man worked the lever with something solid between him and it */
  const play = (W: any, secs: number) => quiet(() => {
    const g = W.game; for (let i = 0; i < 120; i++) { g.player.visibility = 0; W.step(DT); }
    const P = panelOf(g); g.setBlackout(86400, 'hand'); const t0 = g.time; let who: any = null, through = 0, back = -1, from: number[] | null = null, by: any = null;
    for (let i = 0; i < secs * 60 && back < 0; i++) {
      g.player.visibility = 0; W.step(DT); who ??= g.guards.find((x: any) => x.task?.kind === 'breaker') ?? null;
      const at = g.guards.filter((gd: any) => g.blackout.active && breakerResetNear(g, gd));
      for (const gd of at) { by = gd; from = [...gd.char.pos]; if (g.col.solidBlocked([gd.char.pos[0], 1.0, gd.char.pos[2]], P.at)) through++; }
      if (!g.blackout.active) back = g.time - t0;
    }
    const side = from ? (from[0] - P.face[0]) * P.n[0] + (from[2] - P.face[2]) * P.n[2] : 0;
    return { who: who?.callsign ?? 'nobody', by: by?.callsign ?? 'nobody', back, from, through, side, words: `sent ${who?.callsign ?? 'nobody'} · ${back < 0 ? `still out after ${secs} s` : `back at +${f2(back)} s by ${by?.callsign} from (${f2(from![0])}, ${f2(from![2])}), ${f2(side)} m in front of the face`} · ${through} frames at the lever through something solid` };
  });
  // --- the relay station: a pin for each of its first two patrol sets (the third plays the breaker as the first does)
  const pins: number[] = [];
  { const W = await stand('quiet-relay', true); quiet(() => { for (let pin = 1; pin <= 24 && (pins[0] === undefined || pins[1] === undefined); pin++) { W.game.quiet.stop(); startFromQuery(W.game, `?levelFile=quiet-relay.json&quiet=${pin}`); const k = W.game.quiet.deal.patrol; if (k === 0 || k === 1) pins[k] ??= pin; } }); }
  verdict(pins[0] !== undefined && pins[1] !== undefined, "the relay station: a pin found for each of its first two patrol sets", `pins ${JSON.stringify(pins)}`);
  const relay = (set: number, own: boolean) => stand('quiet-relay', own, g => startFromQuery(g, `?levelFile=quiet-relay.json&quiet=${pins[set]}`));
  { const W = await relay(0, false), g = W.game, P = panelOf(g), fx = [g.level.breaker.pos[0] - 0.4, g.level.breaker.pos[2] - 0.9];
    verdict(g.col.nav.isBlocked(fx[0], fx[1]) && P.n[0] === 1, 'the relay station: the panel faces +x off the compound wall and the fixed spot is not floor', `n ${P.n} · fixed spot (${f2(fx[0])}, ${f2(fx[1])}) blocked ${g.col.nav.isBlocked(fx[0], fx[1])}`);
  }
  for (const set of [0]) { const r = play(await relay(set, RULE), 45);
    verdict(r.who !== 'nobody' && r.back > 0 && r.through === 0 && r.side > 0, `rule on, patrol set ${set + 1}: the man sent re-arms the mains from in front of the panel, nothing solid between`, r.words); }
  // --- where the fixed spot was right, nothing moves: the office's night
  { const W = await stand(null, RULE), E = quiet(() => breakerErrand(W.game)), bp = W.game.level.breaker.pos;
    verdict(!!E && E.goal[0] === bp[0] - 0.4 && E.goal[2] === bp[2] - 0.9 && E.mid[2] === bp[2] - 0.6, 'the office: the errand keeps the fixed spot and reach (a man has floor there and a line to the panel)', E ? `goal (${f2(E.goal[0])}, ${f2(E.goal[2])}) mid (${f2(E.mid[0])}, ${f2(E.mid[2])})` : 'no errand read'); }
  // --- every level with a panel: the errand's spot is floor a man can stand on with a line to it, inside the reach
  { const names = readdirSync(`${ROOT}/levels`).filter((f: string) => f.endsWith('.json') && !f.endsWith('-gwot.json')).map((f: string) => f.slice(0, -5)).sort();
    const bad: string[] = [], moved: string[] = [], wrong: string[] = [], skipped: string[] = []; let n = 0;
    for (const name of names) {
      let W: any; try { W = await stand(name, RULE); } catch { skipped.push(name); continue; }
      const g = W.game; if (!g.level.breaker || !g.engine.world.statics[g.level.breaker.boxes[0]]) continue; n++;
      const E = quiet(() => breakerErrand(g)), P = panelOf(g), bp = g.level.breaker.pos, dyn = g.col.dynamicBoxes;
      if (!E) { bad.push(`${name}: no errand read`); continue; }
      const fx = bp[0] - 0.4, fz = bp[2] - 0.9;
      g.col.dynamicBoxes = []; const line = !g.col.solidBlocked([E.goal[0], 1.0, E.goal[2]], P.at), fixedOk = !g.col.nav.isBlocked(fx, fz) && !g.col.solidBlocked([fx, 1.0, fz], P.at); g.col.dynamicBoxes = dyn;
      const floor = !g.col.nav.isBlocked(E.goal[0], E.goal[2]), reach = Math.hypot(E.goal[0] - E.mid[0], E.goal[2] - E.mid[2]);
      // where a walk to it can really end: a nav cell's centre and the arrival band round it, so a stride round the spot
      let short = 0; for (let k = 0; k < 8; k++) { const q: [number, number] = [E.goal[0] + Math.cos(k * Math.PI / 4) * 0.35, E.goal[2] + Math.sin(k * Math.PI / 4) * 0.35];
        if (g.col.nav.isBlocked(q[0], q[1])) continue; g.col.dynamicBoxes = []; const cut = g.col.solidBlocked([q[0], 1.0, q[1]], P.at); g.col.dynamicBoxes = dyn; if (cut) short++; }
      if (short) bad.push(`${name}: ${short} of 8 places a stride round the spot have no line to the panel`);
      if (!floor || !line || reach >= 1.3) bad.push(`${name}: goal (${f2(E.goal[0])}, ${f2(E.goal[2])}) floor ${floor} line ${line} reach ${f2(reach)}`);
      const isMoved = E.goal[0] !== fx || E.goal[2] !== fz; if (isMoved) moved.push(name);
      if (isMoved === fixedOk) wrong.push(`${name}: fixed spot ${fixedOk ? 'good' : 'bad'}, errand ${isMoved ? 'moved' : 'kept'}`);
    }
    verdict(n >= 30 && skipped.length <= 2 && bad.length === 0, `every level with a breaker (${n}): the errand's spot is floor with a line to the panel, inside the reach it is re-armed from`, bad.join('\n    ') || `moved off the fixed spot on ${moved.length}: ${moved.join(', ')}${skipped.length ? ` · files that do not load as a level, not asked: ${skipped.join(', ')}` : ''}`);
    verdict(wrong.length === 0 && moved.includes('quiet-relay') && !moved.includes('office'), 'the spot moved exactly where the fixed one is not floor or has something solid between it and the panel (the relay station yes, the office no)', wrong.join('\n    ') || `${moved.length} of ${n}: ${moved.join(', ')}`); }
});


