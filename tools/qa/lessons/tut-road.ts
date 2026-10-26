// The scripted player for the tutorial's first mission (levels/tut-road.json; tools/qa/lesson-run.ts runs it): what each line asks, through the input only,
// in the order a new player would do it. He is mortal: the fights are the level's script's (levels/tut-road.script.ts) and none of them is aimed at a man who
// does what the line says.
import type { Bot } from '../lesson-run';
import { VAN, WALL, CORNER, GATEWAY, POST, HUT, HUT_DOOR_Z, DOOR_FROM, PLANKS, HUT_MAN, TRUNK, JAMB, LANE_Z, BAGS_END, COVER, COVER_Z, RB_WALL, LAST_MEN, EXIT } from '../../levels/tut-road';
import { SLOTSETUP } from '../../../src/game/slotsetup';
import { DOORSHOT } from '../../../src/game/doorblast';
import { WALLPEN } from '../../../src/game/wallpen';
import { DESTRUCTION as DESTR_HERE } from '../../../src/game/destructionSwitch';

/** is a switch on IN THE GAME THIS RUN PLAYS: on a page the script is its own bundle, with its own copy of the table (the page's is `__knobs`) */
const pageOn = (name: string, here: boolean): boolean => (globalThis as { __knobs?: { registry?: () => Record<string, { on?: boolean }> } }).__knobs?.registry?.()[name]?.on ?? here;
const setupOn = (): boolean => pageOn('SLOTSETUP', SLOTSETUP.on), doorShotOn = (): boolean => pageOn('DOORSHOT', DOORSHOT.on), wallPenOn = (): boolean => pageOn('WALLPEN', WALLPEN.on);
/** …and do rounds reach a leaf at all here (wallbreak.ts wallBreakOn: the destruction's switch) */
const wallsBreak = (B: Bot): boolean => pageOn('DESTRUCTION', DESTR_HERE.on) && (B.g as unknown as { tune: { wallBreak?: boolean } }).tune.wallBreak !== false;

export const query = '?quiet';
/** the level's fights are its script's, which the tutorial frame starts: on a stand without the frame this lesson cannot be played */
export const needs = 'rails';
/** THE LESSONS CAN BE FAILED THE WAY THEIR LINES SAY (each of these runs must end with the player down; tools/qa/tut-road-check.ts asks it):
 *  TUT_ROAD_ARM=stand walks the sandbags standing, TUT_ROAD_ARM=field walks out onto the open snow in front of the gun. Two more words are mistakes that
 *  must cost nothing: `east` stands on the road between the truck and the wall, in the sentry's line; `jamb` runs at the hut's door (it must hold) and
 *  then puts his rounds into the wall beside it, which must still leave him a way through. `again` goes down at the sandbags and comes back by Enter,
 *  and has the second checkpoint put back by hand */
const brk = (): string => (typeof process !== 'undefined' ? process.env.TUT_ROAD_ARM : (globalThis as { __TUT_ROAD_ARM?: string }).__TUT_ROAD_ARM) ?? '';   // (read as the step runs: tools/qa/tut-road-check.ts plays every way in one process)
/** THE HUT'S DOOR TEACHES A DOOR SHOT OUT (game/doorblast.ts DOORSHOT, held off on the frozen stand, and it lives only where walls break: the destruction's
 *  one switch, PT_DESTR), AND THE PLANK WALL A ROUND THROUGH WOOD (game/wallpen.ts WALLPEN, held off there too): all three raised for this run alone, as a
 *  run there raises the frame. With the door's off the line still goes by, on a long burst, and the level's script opens the door; with the wall's off
 *  the man behind the boards is found by walking round them (played so with `TPS_DOORSHOT=0` or `TPS_WALLPEN=0` in front on either stand) */
export const raises = { TPS_DOORSHOT: '1', PT_DESTR: '1', TPS_WALLPEN: '1' };
/** lines that may be true as they come up: the last men already down, the door left standing for the script to open (`jamb`), a gun no reload would help,
 *  a man who stepped off the gateway's wall to shoot the sentry (the old hold, on the frozen stand, has no lean) */
export const idleOk = ['last', 'door', 'reload', 'off'];
/** the run wears what the mission's page deals him (lesson-run.ts): his own card, the rifle and its magazines */
export const pageKit = true;

const trigger = (B: Bot, at: () => number[], secs: number, stop?: () => boolean) => {
  B.aim(at()); B.step(); B.I.clicked |= 1;
  try { for (let i = 0; i < secs * 60; i++) { B.aim(at()); B.I.buttons |= 1; if (i % 14 === 13) B.I.clicked |= 1; B.step(); if (stop && stop()) break; } } finally { B.I.buttons &= ~1; B.step(); }
};
const chest = (x: any): number[] => x.char.bones?.chest ?? [x.char.pos[0], 1.25, x.char.pos[2]];
const up = (x: any) => x.state !== 'dead';
const line = (B: Bot, x: any) => { const P = B.g.player.char, a = P.bones?.head ?? [P.pos[0], 1.5, P.pos[2]]; return !B.g.col.shotBlocked(a, chest(x)); };
const gun = (B: Bot) => B.g.player.rifle;
const downed = (B: Bot) => B.g.guards.filter((x: any) => !up(x)).length;
const man = (B: Bot, route: string) => B.g.guards.find((x: any) => x.route?.name === route);
/** shoot every man he has a line to until `n` are down (reloading as the gun runs dry), for `secs` at most */
const fight = (B: Bot, n: number, secs: number, move?: () => void, who?: any[]) => {
  const g = B.g, P = g.player, t0 = B.t();
  while (downed(B) < n && B.t() - t0 < secs && !P.down) {
    const w = gun(B); if (w.roundsReady === 0 && !w.reloading) { B.press('KeyR'); continue; }
    const x = (who ?? g.guards).filter((y: any) => up(y) && line(B, y)).sort((a: any, b: any) => Math.hypot(a.char.pos[0] - B.pos()[0], a.char.pos[2] - B.pos()[2]) - Math.hypot(b.char.pos[0] - B.pos()[0], b.char.pos[2] - B.pos()[2]))[0];
    if (!x) { if (move) move(); else B.step(); continue; }
    trigger(B, () => chest(x), 1.2, () => !up(x) || !line(B, x));
  }
  return downed(B) >= n;
};
/** a slot's key, and the half second the thing takes to come into the hand */
const take = (B: Bot, code: string) => { B.press(code, 5); B.until(() => B.g.player.equipT <= 0, 3); B.step(3); };
const offWall = (B: Bot) => { if (B.g.player.wall) B.press('KeyQ', 10); };

/** the gateway's mouth: just through the lower wall's line, between the sentry and the truck he shoots */
const GATEWAY_MOUTH = [POST.x0 + 0.6, (GATEWAY.a + GATEWAY.b) / 2 - 0.75];
const once = new Set<string>();
const stepIs = (B: Bot, id: string) => B.g.lesson.step?.id === id;
/** the rifle's rounds as the checkpoint 'the hut' was kept (its step coming up): what the boards cost is the die's, one round or a few */
let hutRounds = -1;
/** the checkpoint 'the hut' is kept with the door out, the rifle on full auto and as loaded as it was, and two men down: put back, none of that is asked again */
const againHut = (B: Bot) => { const d = B.g.doors.byName('hut')!, w = B.g.player.rifle;
  if (d.isClosed()) B.fail('again: the checkpoint at the hut put its door back on its hinges'); if (w.roundsReady !== hutRounds) B.fail(`again: the checkpoint at the hut left the rifle at ${w.roundsReady} rounds (it was kept at ${hutRounds})`);
  if (downed(B) !== 2) B.fail(`again: ${downed(B)} men are down after the checkpoint at the hut (two were)`); };
/** play these lines again, each when it is the one that is up */
const replay = (B: Bot, ids: string[]) => { for (const id of ids) {
  const gone = () => idleOk.includes(id) && B.g.lesson.i > B.g.level.scenarios.lesson.steps.findIndex((T: { id: string }) => T.id === id);
  if (!B.until(() => stepIs(B, id) || gone(), 4)) B.fail(`again: '${id}' did not come up after the checkpoint (the lesson is on '${B.g.lesson.step?.id}')`); if (gone()) continue; B.step(30); steps[id](B); B.step(30); } };
/** down as the line warns, then Enter */
const downAndBack = (B: Bot, how: () => void, to: string) => {
  how(); if (!B.until(() => B.g.player.down, 4)) B.fail(`again: he was not hit (${to})`);
  B.step(20); B.press('Enter', 20);
  if (B.g.player.down || !stepIs(B, to)) B.fail(`again: Enter did not put him back at '${to}' (down ${B.g.player.down}, the lesson is on '${B.g.lesson.step?.id}')`);
  B.say(`  down, and back at '${to}' by Enter`);
};
/** the newest checkpoint of this run, put back by the harness */
const handBack = (B: Bot, to: string) => {
  const g = B.g, ck = g.saves.newest(true, g.mission.stats.elapsed), why = ck ? g.saves.loadInStep(g, ck.key) : 'no checkpoint on the shelf'; B.step(20);
  if (why || !stepIs(B, to)) B.fail(`again: the checkpoint was not put back at '${to}' (${why ?? `the lesson is on '${g.lesson.step?.id}'`})`);
  B.say(`  the checkpoint put back: '${to}'`);
};
/** the lane behind the sandbags, from the side door's lee to its far end */
const lane = (B: Bot, o?: { stop?: () => boolean }) => { B.go([JAMB[0] + 0.1, 0, LANE_Z], 12, o); B.go(BAGS_END, 15, o); };
const lastIn = (B: Bot) => { const men = B.g.guards.slice(3); return men.every((x: any, k: number) => !up(x) || Math.hypot(x.char.pos[0] - LAST_MEN[k][0], x.char.pos[2] - LAST_MEN[k][2]) < 0.9); };

export const steps: Record<string, (B: Bot) => void> = {
  // THE ROAD: off the start mark, up the truck's near side and behind its cab, with the sentry's rounds going into its far side
  // (TUT_ROAD_ARM=east first walks the wrong side of the truck and stands in the sentry's line: he holds his fire for a man there, and nobody is hit)
  move: B => {
    once.clear();
    if (B.g.rails && !B.g.rails.locks('camera')) B.fail('the view can be turned before the line that teaches it');
    if (brk() === 'east') { B.go([6.5, 0, 15.2], 12); B.go([6.6, 0, 13.4], 10); B.wait(6); if (B.g.player.down) B.fail('BREAK east: he stood in the sentry\'s line beside the truck and was hit'); B.go([6.5, 0, 15.6], 10); B.go([3.2, 0, 17.6], 12); }
    // (TUT_ROAD_ARM=mouth: the other thing nothing tells a new player not to do. Up the road on the first line and into the gateway's mouth, between the
    //  sentry and the truck he is shooting, and eight seconds there (he fires a burst every two or three, and reloads); then back for the line)
    if (brk() === 'mouth') { B.go([6.4, 0, 15.6], 12); B.go([GATEWAY_MOUTH[0], 0, GATEWAY_MOUTH[1]], 12, { near: 0.15 }); B.wait(8); if (B.g.player.down) B.fail('mouth: he walked up the road into the gateway\'s mouth on the first line and was hit');
      B.say(`  mouth: eight seconds in the gateway's mouth at (${B.pos()[0].toFixed(2)}, ${B.pos()[2].toFixed(2)}) and nobody hit`); B.go([6.5, 0, 15.6], 12); B.go([3.2, 0, 17.6], 12); }
    B.go(VAN, 20);
  },
  // the view, turned: it was locked until this line, and it is put back as it was once the line has gone by (every walk below is by the compass)
  turn: B => {
    let n = 0; for (; n < 60 && stepIs(B, 'turn'); n++) { B.cam.rotate(0.025); B.step(); }
    if (stepIs(B, 'turn')) B.fail('the view was turned most of a quarter turn and the line is still up');
    for (let i = 0; i < n; i++) { B.cam.rotate(-0.025); B.step(); }
  },
  ready: B => { take(B, 'Digit2'); },
  // round the truck's nose and along under the rock to the post's wall, running: never in the gateway's line
  sprint: B => { B.go([3.2, 0, 11.1], 10); B.go(WALL, 12, { sprint: true, near: 0.6 });
    // (mouth: …and the same at a sprint, carried on past the wall and round its end into the gateway, as a man does who was told to sprint and did)
    if (brk() === 'mouth') { B.go([POST.x0 - 0.7, 0, GATEWAY.a + 0.5], 4, { sprint: true, near: 0.3 }); B.go([GATEWAY_MOUTH[0], 0, GATEWAY_MOUTH[1]], 4, { sprint: true, near: 0.15 }); B.wait(5); if (B.g.player.down) B.fail('mouth: he sprinted to the wall, carried on round its end into the gateway and was hit');
      B.say('  mouth: round the wall\'s end into the gateway at a sprint, five seconds there, and nobody hit'); B.go([POST.x0 - 0.9, 0, GATEWAY.a + 0.6], 6); B.go(WALL, 8, { near: 0.6 }); if (B.g.player.wall) B.press('KeyQ', 10); } },
  // walk into the wall as the page takes one; the hold's own key where walking in is switched off (the frozen stand)
  wall: B => {
    if (B.g.player.wall) B.fail('he is on a wall before the line that teaches it');
    B.nudge([POST.x0 + 0.5, 0, WALL[2]], 1.5, () => !!B.g.player.wall);
    if (!B.g.player.wall) { B.aim([POST.x0 + 0.6, 1.2, WALL[2]]); B.press('KeyQ', 20); }
    if (!B.g.player.wall) B.fail('no wall taken beside the gateway, by walking into it or by its key');
  },
  // along the wall to the gateway's corner, and the cursor on the man past it: the page steps him out with the rifle in the shoulder
  peek: B => {
    const g = B.g, s = man(B, 'sentry');
    const w = () => g.player.wall, stack = () => w()?.kind === 'stack';
    B.nudge([WALL[0], 0, GATEWAY.a + 0.4], 4, () => !w() || line(B, s) || (stack() ? !!w()!.atEdge : (w()!.peek ?? 0) > 0.5));
    if (stack() && !stepIs(B, 'peek')) B.fail('the lean\'s line went by on the slide alone, before the cursor was past the corner');
    for (let i = 0; i < 150 && stepIs(B, 'peek'); i++) { B.aim(chest(s)); B.step(); }
    B.say(`  at the corner: lean ${(g.player.wall?.lean ?? 0).toFixed(2)} · peek ${(g.player.wall?.peek ?? 0).toFixed(2)} · a line to the sentry ${line(B, s)} · body z ${B.pos()[2].toFixed(2)} (the corner is at ${CORNER[2]})`);
    if (stepIs(B, 'peek')) B.fail('at the gateway\'s corner with the cursor on the sentry he did not lean out');
  },
  // …and shoot the man in the gateway's line from there
  fire: B => {
    const g = B.g, s = man(B, 'sentry');
    for (let i = 0; i < 90 && !line(B, s); i++) { B.aim(chest(s)); B.step(); }
    // the old hold (the frozen stand: his back to the wall, hands down): off it and a step into the gateway, on the side his rifle clears
    if (!line(B, s) && g.player.wall?.kind !== 'stack') { offWall(B); B.go([POST.x0 - 0.8, 0, GATEWAY.a + 0.9], 6, { near: 0.12 }); }
    if (!line(B, s)) B.fail('leaning round the gateway\'s corner he has no line to the sentry');
    const r0 = gun(B).roundsReady;
    if (!fight(B, 1, 20, undefined, [s])) B.fail(`the sentry did not drop (line ${line(B, s)}, he is ${s.state}, ${r0 - gun(B).roundsReady} rounds left the gun)`);
    B.say(`  the sentry is down from the corner: ${r0 - gun(B).roundsReady} round(s)`);
    B.step(45); if (!stepIs(B, 'fire')) B.fail('the next line came up within a second of the sentry going down: the moment did not settle');
  },
  // the rifle's selector: its own key on the stand from before the gun's setup, the setup's first row where that is up (the rifle's digit again, then 1)
  auto: B => {
    if (setupOn()) { B.press('Digit2', 12); if (!B.g.player.setup) B.fail('the rifle\'s key pressed again did not open its setup'); B.press('Digit1', 12); } else B.press('KeyZ', 12);
    if (stepIs(B, 'auto')) B.fail(`the selector did not move (the rifle is on ${B.g.player.rifle.mode}, the hands hold ${B.g.player.slot}, its setup is ${B.g.player.setup ? 'up' : 'shut'})`);
    if (B.g.player.slot !== 'rifle') B.fail(`the selector's digit took the ${B.g.player.slot} into his hands`);
  },
  // THE HUT'S DOOR: in by the gateway, square on to the leaf, and the trigger held until it is off its hinges (twelve rifle rounds: doorblast.ts). The way
  // `jamb` puts a long burst into the wall beside it instead: the line goes by on the burst and the script opens the door as the next line comes up
  // off the wall the way the line says: the move keys, square away from it
  off: B => { const P = B.g.player; if (!P.wall) return; const n = P.wall.n, p = B.pos();
    B.nudge([p[0] + n[0] * 1.5, 0, p[2] + n[2] * 1.5], 1.5, () => !P.wall); if (P.wall) B.fail('a second and a half of walking square away from the wall and he is still on it'); },
  inner: B => { B.go([POST.x0 + 1.2, 0, 15.2], 10); B.go(DOOR_FROM, 20, { near: 0.6 }); },
  door: B => {
    const g = B.g, d = g.doors.byName('hut')!, at = [HUT.x0, 1.1, HUT_DOOR_Z], r0 = gun(B).roundsReady, splinters = doorShotOn() && wallsBreak(B);
    if (!d.locked || !d.isClosed()) B.fail('the hut\'s door is open or unlocked before the line that shoots it out');
    if (brk() === 'jamb') {
      // (first the other thing a new player tries: a run at it. A sprint into a locked door is a kick, and this one opens toward him, so its frame holds)
      if ((g.doors as unknown as { frameRule?: boolean }).frameRule) { B.go([HUT.x0 - 2.6, 0, HUT_DOOR_Z], 6, { near: 0.3 }); B.go([HUT.x0 + 0.5, 0, HUT_DOOR_Z], 2.5, { sprint: true, noDoors: true }); B.until(() => !g.player.kick, 3); B.step(30);
        if (!d.locked || !d.isClosed()) B.fail('jamb: a sprint into the hut\'s door opened it (the lesson of the line is the trigger)'); B.say('  jamb: a sprint into the door did not open it'); }
      B.go(DOOR_FROM, 6, { near: 0.3 });
      trigger(B, () => [HUT.x0, 1.1, HUT_DOOR_Z + 1.5], 8, () => !stepIs(B, 'door') || !!g.lesson.settling);
      if (stepIs(B, 'door') && !g.lesson.settling) B.fail(`jamb: ${r0 - gun(B).roundsReady} rounds into the wall beside the door and the line is still up`);
      if ((d as unknown as { off?: boolean }).off) B.fail('jamb: the leaf is off its hinges with every round in the wall beside it');
      B.until(() => !d.isClosed(), 4); if (d.isClosed()) B.fail('jamb: the line went by on his rounds and the door was not opened for him');
      B.say(`  jamb: ${r0 - gun(B).roundsReady} rounds into the wall, the line went by and the door was opened for him`); return; }
    // (slant: a long burst from a stand to one side with the cursor on the man in the back room himself: through the leaf, through the boards, and on
    //  past the leaf once it is out. He waits behind a wall until his own line: he must still be up for it)
    if (brk() === 'slant') { B.go([14.9, 0, 13.5], 6, { near: 0.2 }); const him = man(B, 'hut'), to = () => chest(him);   // (the cursor on him, through the leaf and all)
      trigger(B, to, 6, () => !d.isClosed() || !stepIs(B, 'door') || !!g.lesson.settling); trigger(B, to, 1.5, () => gun(B).roundsReady === 0);
      if (downed(B) !== 1) B.fail(`slant: ${r0 - gun(B).roundsReady} rounds at the door from the side and ${downed(B)} men are down: one of them found the man in the back room before his line`);
      B.say(`  slant: ${r0 - gun(B).roundsReady} rounds at the door from the side, and the man in the back room is still up`); return; }
    B.go(DOOR_FROM, 6, { near: 0.3 });
    // (where gunfire does not splinter a door — the switch off, or walls that do not break — the twentieth round ends the line and the script opens it)
    if (!splinters) { trigger(B, () => at, 10, () => !stepIs(B, 'door') || !!g.lesson.settling);
      if (stepIs(B, 'door') && !g.lesson.settling) B.fail(`gunfire does not splinter a door on this stand, and ${r0 - gun(B).roundsReady} rounds did not end the line`);
      B.say(`  the hut's door: ${r0 - gun(B).roundsReady} round(s) and the line went by (gunfire does not splinter a door on this stand)`); return; }
    trigger(B, () => at, 6, () => !d.isClosed());
    const n = r0 - gun(B).roundsReady; B.say(`  the hut's door: ${d.isClosed() ? 'still standing' : 'out'} after ${n} round(s)`);
    if (d.isClosed() || n > 14) B.fail(`the hut's door took ${n} rounds square on and is ${d.isClosed() ? 'still standing' : 'out'} (it stands twelve)`);
    // (the rounds for the door go on through it, through the boards and into the back room: the man there stands a long pace off their line)
    if (downed(B) !== 1) B.fail(`${downed(B)} men are down after the door: a round through the leaf found the man behind the boards`);
    B.step(40); if (!stepIs(B, 'door')) B.fail('the reload\'s line came up within a second of the door going: the moment did not settle');
  },
  // the reload, and the line stays up until it has run to its end
  reload: B => {
    const w = gun(B), full = () => !w.reloading && w.roundsReady >= w.def.magCapacity;
    if (!stepIs(B, 'reload')) return;   // (a gun with nothing to gain from a reload: the line has gone by)
    B.until(() => !B.g.doors.byName('hut')!.isClosed(), 2); if (B.g.doors.byName('hut')!.isClosed()) B.fail('the reload\'s line is up and the hut\'s door still stands: the script did not open it');
    if (!full() && !w.reloading) B.press('KeyR', 6);
    if (!w.reloading && !full()) B.fail(`R did not start a reload (${w.roundsReady} ready, ${w.spare.length} spare)`);
    while (w.reloading) { B.step(); if (!stepIs(B, 'reload')) B.fail('the next line came up while the rifle was still being reloaded'); }
    B.step(30); if (!stepIs(B, 'reload')) B.fail('the next line came up within half a second of the reload: the moment did not settle');
    B.say(`  reloaded: ${w.roundsReady} ready`);
  },
  // THE PLANK WALL: from outside the doorway, the cursor on the man behind the boards and the trigger held. Where a round does not go through wood (the
  // switch off) the boards stop every one of them, and he is found by walking round their end
  thin: B => {
    const g = B.g, x = man(B, 'hut'); if (!x || !up(x)) B.fail('the man behind the boards is down before his line');
    B.go([HUT.x0 - 0.7, 0, HUT_DOOR_Z + 0.15], 6, { near: 0.25 });
    B.wait(2.2); if (g.player.down) B.fail('he stood at the hut\'s doorway as the man behind the boards fired, and was hit');
    if (!B.until(() => !up(x) || Math.hypot(x.char.pos[0] - HUT_MAN[0], x.char.pos[2] - HUT_MAN[2]) < 0.5, 5)) B.fail(`the man in the back room did not come out of his nook to the boards (he is at ${x.char.pos[0].toFixed(2)}, ${x.char.pos[2].toFixed(2)})`);
    // (slant has put a magazine through the boards on the way to his nook: what is left of them is not asked)
    if (brk() !== 'slant' && line(B, x)) B.fail('he has a clear line to the man behind the boards from the doorway: the boards are not between them');
    const r0 = gun(B).roundsReady;
    if (wallPenOn()) { for (let k = 0; k < 3 && up(x); k++) { if (gun(B).roundsReady === 0) { B.press('KeyR'); B.until(() => !gun(B).reloading, 5); } trigger(B, () => chest(x), 2.5, () => !up(x)); }
      if (up(x)) B.fail(`${r0 - gun(B).roundsReady} rounds through the boards and the man behind them is still up`);
      B.say(`  the man behind the boards is down through them: ${r0 - gun(B).roundsReady} round(s)`); return; }
    trigger(B, () => chest(x), 1.0, () => !up(x)); if (!up(x)) B.fail('a round went through the boards on a stand where it does not');
    B.go([PLANKS.x - 0.9, 0, PLANKS.z1 + 0.6], 8, { stop: () => line(B, x) }); if (!fight(B, 2, 15, () => { B.nudge([PLANKS.x + 0.5, 0, PLANKS.z1 + 0.5], 0.3); }, [x])) B.fail('the man behind the boards did not drop, round their end');
    B.say(`  the man behind the boards is down, round their end (a round does not go through wood on this stand): ${r0 - gun(B).roundsReady} round(s)`);
  },
  // round the boards' end, out of the side door and into the wind wall's lee; then down, as the gun starts along the sandbags
  arch: B => { if (!once.has('arch')) { once.add('arch'); hutRounds = gun(B).roundsReady; } B.go([HUT.x0 + 1.0, 0, HUT_DOOR_Z], 10); B.go([PLANKS.x + 0.6, 0, HUT_DOOR_Z + 0.1], 15); B.go(JAMB, 15); },
  crouch: B => { B.wait(1.2); B.crouch(true); },
  // crouched along behind the sandbags
  low: B => {
    if (brk() === 'again' && !once.has('low')) { once.add('low'); downAndBack(B, () => { B.crouch(false); lane(B, { stop: () => B.g.player.down }); }, 'arch'); againHut(B); replay(B, ['arch', 'crouch']); B.until(() => stepIs(B, 'low'), 4); }
    if (brk() === 'stand') B.crouch(false); lane(B); if (brk() === 'stand') { B.wait(2.5); B.fail('BREAK stand: he walked the sandbags standing and was not hit'); }
  },
  // on behind the roadblock's wall, up, and into it
  // (TUT_ROAD_ARM=field walks on past its end and out onto the gun's own ground instead: the gunner is the game's from then on, and he must go down)
  cover: B => {
    B.go([RB_WALL.x0 + 0.5, 0, COVER[2] + 0.2], 8); B.crouch(false);
    if (brk() === 'field') { B.go([31.6, 0, 14.6], 8, { stop: () => B.g.player.down }); B.wait(5); B.fail('BREAK field: he walked out onto the open snow in front of the gun and was not hit'); }
    B.go(COVER, 6, { near: 0.3 });
    B.nudge([COVER[0], 0, COVER_Z], 1.5, () => !!B.g.player.wall);
    if (!B.g.player.wall) { B.aim([COVER[0], 1.2, COVER_Z - 0.2]); B.press('KeyQ', 20); }
    if (!B.g.player.wall) B.fail('no wall taken at the roadblock, by walking into it or by its key');
  },
  // along it to its bare end, the cursor on the gunner: the lean again, with nobody saying how
  gunner: B => {
    const g = B.g, x = man(B, 'gunner'), w = () => g.player.wall, stack = () => w()?.kind === 'stack';
    B.nudge([RB_WALL.x1 + 0.4, 0, COVER[2] - 0.3], 4, () => !w() || (stack() ? !!w()!.atEdge : (w()!.peek ?? 0) > 0.5));
    for (let i = 0; i < 150 && !(line(B, x) && (!stack() || (w()!.lean ?? 0) > 0.4)); i++) { B.aim(chest(x)); B.step(); }
    if (!line(B, x) && g.player.wall?.kind !== 'stack') { offWall(B); B.go([RB_WALL.x1 + 0.35, 0, COVER_Z + 0.75], 6, { near: 0.12 }); }   // (the old hold: a step out past the wall's end, short of the gun's own ground)
    B.say(`  at the wall's end: ${g.player.wall ? `${g.player.wall.kind} lean ${(g.player.wall.lean ?? 0).toFixed(2)} edge ${!!g.player.wall.atEdge}` : 'off the wall'} · a line to the gunner ${line(B, x)} · body (${B.pos()[0].toFixed(2)}, ${B.pos()[2].toFixed(2)}) · crouched ${!!g.player.crouching} · ${g.player.slot} ${gun(B).roundsReady}`);
    if (!line(B, x)) B.fail('leaning round the end of the roadblock\'s wall he has no line to the gunner');
    if (x.script === undefined || x.script === null) B.fail('the gunner was let go while the player did as the lines say (he is the script\'s until a man walks out in front of him)');
    const r0 = gun(B).roundsReady;
    if (!fight(B, 3, 25, undefined, [x])) B.fail(`the gunner did not drop (line ${line(B, x)}, he is ${x.state}, ${r0 - gun(B).roundsReady} rounds left the gun)`);
    B.say(`  the gunner is down from the wall's end: ${r0 - gun(B).roundsReady} round(s)`);
  },
  // the line that only has him look: it goes by once the three stand inside the gate, and not before
  watch: B => {
    B.until(() => !stepIs(B, 'watch'), 10); if (stepIs(B, 'watch')) return;
    const men = B.g.guards.slice(3).filter(up); if (men.length < 3 || !lastIn(B)) B.fail(`the trigger is asked for before the three stand inside the gate (${men.length} up)`);
  },
  // the three in the gate's lamp, from the wall's end
  last: B => {
    const g = B.g;
    if (brk() === 'again' && !once.has('last')) { once.add('last');
      handBack(B, 'watch'); if (g.guards.slice(3).some((x: any) => x.char.pos[0] < POST.x1)) B.fail('again: one of the last three is inside the gate before their line (the checkpoint is from before they came)');
      if (downed(B) !== 3) B.fail(`again: ${downed(B)} men are down after the checkpoint at the gun (three were)`);
      replay(B, ['watch']); B.until(() => stepIs(B, 'last'), 4); B.step(30); }
    fight(B, 6, 60, () => { const x = g.guards.find(up); if (!x) return; const p = x.char.pos; if (!g.player.wall && Math.hypot(p[0] - B.pos()[0], p[2] - B.pos()[2]) > 12) B.go([p[0], 0, p[2]], 1.5, { stop: () => line(B, x) }); else B.step(10); });
  },
  // THE RADIO: the trunk in the gun's nest, round the end of its sandbags. The line asks for a TAP, and a tap does the top row: it must be the radio's
  // (the gunner lies a pace from it: no row of his may outrank it)
  radio: B => { offWall(B); B.go([TRUNK[0], 0, TRUNK[2] + 0.95], 30, { near: 0.3 }); },
  take: B => { B.go([TRUNK[0], 0, TRUNK[2] + 0.95], 6, { near: 0.3 }); B.step(2);
    const top = B.g.nearby.find((r: any) => !!r.verb); if (top && top.verb !== 'drive') B.fail(`a tap at the trunk gives '${top.kind} ▸ ${top.verb}', not the radio`);
    if (!B.verb('objective', 'drive', [TRUNK[0], 0.5, TRUNK[2] + 0.4])) B.fail('no row to take the radio'); B.until(() => B.g.mission.stage === 'exfil', 20); },
  // on up the road: through the gate
  out: B => { offWall(B); B.go([EXIT.x - 1.6, 0, EXIT.z], 30, { sprint: true }); const ok = B.go([EXIT.x + 1.2, 0, EXIT.z], 10); const d = B.g.doors.byName('gate')!; B.say(`  out: reached ${ok} at (${B.pos()[0].toFixed(2)}, ${B.pos()[2].toFixed(2)}) · the gate angle ${d.angle.toFixed(2)} locked ${d.locked} · stage ${B.g.mission.stage}`); },
};
/** A line stays up a beat after it is done where the level says so (a step's `settle`: the man falls, the door goes, the reload ends): each step waits
 *  that out, as a player does, before the runner asks whether the line has gone by */
for (const id of Object.keys(steps)) { const f = steps[id]; steps[id] = B => { f(B); if (stepIs(B, id) && B.g.lesson.settling) B.until(() => !stepIs(B, id), 6); }; }
