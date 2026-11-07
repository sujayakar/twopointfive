// The 'rescue-flow' sub-family of 'rescue' (game/rescueflow.ts + F's two-door entry in game/rescue.ts, run by the door tools — the hostage rescue's DYNAMIC TEMPO), run through probes.ts:
//   bun run tools/qa/probes.ts rescue-flow | rescue-flow-marks | rescue-flow-pass | rescue-flow-states | rescue-flow-orders | rescue-flow-twodoor   [--verbose]
// PASS / FAIL each, all headless, on rescue-probes.ts's rig (the scenario started with game.rescue.start(), Sam teleported along the corridor at his walk — nothing
// here plays his part of a fight; what anybody SEES is the harness's reading of the night, as there).
//   rescue-flow-marks    the route's marks against the floor (RescueFlow.check) and the side door's (SIDE_MARKS — what the door tools stack and stand off on
//                        where the racks leave no floor at its jambs): free floor, a line from each cover mark to what it watches, something to look at
//                        through every opening; the door tools' own stack and points of domination for that door
//   rescue-flow-pass     Sam walks the corridor from the arch to the hostage room's door and gives no order: every opening he passes is taken in order, with its
//                        call; that man is on its mark with his gun on it; nobody goes into a room; the man catches up
//   rescue-flow-states   what the player is shown: a strip on the floor at every opening in its state's colour, the objective line's 'behind you', the squad
//                        block's words; one of theirs seen in a room turns it hot and its cover is HELD until he is dead; with both men on him, the last holds
//                        the corridor while Sam is in a room; a hot room found empty goes cold (the strips themselves went on the TypeScript game: nothing here reads them)
//   rescue-flow-orders   the menu overrides: an order to the man on a cover takes him off it and the flow does not send the next man behind Sam's back; men
//                        told to hold are left alone while Sam passes openings; 'fall in' hands them back and the next opening is covered; a lone man covers (flowSolo)
//   rescue-flow-twodoor  the hostage room through both doors on one F, two of the door tools' bang and clears through one gate: a man posted in the server room
//                        takes the side door, the other the corridor door — both leaves on one frame (the corridor door booted, the unlocked side door shoved
//                        with the boot), a can through each, both entries inside a quarter of a second; with both men covering F is refused; a menu
//                        order calls it off on the walk (the door tools' rule)
import { ROOT } from './headless.ts';
// 2026-11-07: no settled switch is held off or written here (three probes were staged without DOORMANNERS.on, DARKFOE.on, RESCUEEND.on and the glass parts;
// the rigs passed RESCUE.clock / flow / marks / twoDoor / flowSolo off; one scene put NVG.on and DARKFOE.on off). Out with their off sides: 'switched on with
// Sam at the server room' (the flow coming on late: it began with the flow off), 'RESCUE.marks off: no strip and no words', 'RESCUE.flow off: nobody covers
// anything', 'flowSolo off: the one man left stays on Sam', and the two 'RESCUE.twoDoor off' lines. GATEHAND_BREAK=1 is a staging now: the man is never
// moved to the side door, so one F splits nothing and the two-door lines fail (it was: DOORTOOLS.gateHand left off).
import { sample } from './sweep.ts';
import { rig, family, guarded, give, orderOf, up, dist, f1, DT } from './rescue-probes.ts';
const { SIDE_MARKS } = await import(`${ROOT}/src/game/rescue.ts`);
const { killGuard } = await import(`${ROOT}/src/game/combat.ts`);
const { DOORTOOLS } = await import(`${ROOT}/src/game/doortools.ts`);

type Said = { t: number; who: string; text: string };
/** everything any man says from here on (Game.say is the one mouth) */
function ears(T: any): Said[] { const g = T.game, said: Said[] = [], say0 = g.say.bind(g); g.say = (gd: any, text: string, radio?: boolean) => { said.push({ t: g.time, who: gd.callsign, text }); return say0(gd, text, radio); }; return said; }
const flowLog = (T: any) => T.R.log.filter((l: any) => l.what.startsWith('flow:')).map((l: any) => l.what);
const INSIDE = (p: number[], B: { x0: number; x1: number; z0: number; z1: number }) => p[0] > B.x0 && p[0] < B.x1 && p[2] > B.z0 && p[2] < B.z1;
const CONF = { x0: 4.12, x1: 13.94, z0: 4.12, z1: 9.94 }, SERVER = { x0: 14.12, x1: 21.88, z0: 4.12, z1: 9.94 }, CUB = { x0: 12.2, x1: 27.8, z0: 12.3, z1: 23.9 }, OFFICE = { x0: 22.12, x1: 29.94, z0: 4.12, z1: 9.94 };
/** Sam along a straight line at `speed`, `each` called after every frame. The default is HIS WALK (Game.tune.playerWalk 1.55 m/s): it was a 3.2 m/s jog — a pace
 *  the player does not have (he walks at 1.55 or sprints at 4.7) and one the team could once match; a teammate's legs now stop at guards.ts PACE.max 2.2 */
function walk(T: any, to: number[], speed = 1.55, each?: () => void) {
  const p = T.game.player.char.pos, from = [p[0], 0, p[2]], n = Math.max(1, Math.ceil(dist(from, to) / (speed * DT)));
  for (let i = 1; i <= n; i++) { T.game.teleportPlayer([from[0] + (to[0] - from[0]) * i / n, 0, from[2] + (to[2] - from[2]) * i / n]); T.step(); each?.(); }
}
/** what is known of an opening, off the flow's own list */
const stateOf = (R: any, id: string): string | null => R.flow.known.find((k: any) => k.o.id === id)?.state ?? null;
const yawTo = (a: number[], b: number[]) => Math.atan2(b[0] - a[0], b[2] - a[2]);
const wrap = (a: number) => { a = (a + Math.PI) % (2 * Math.PI); if (a <= 0) a += 2 * Math.PI; return a - Math.PI; };

export const flowMarks = guarded(async () => {
  const F = family('rescue-flow-marks: the route\'s marks and the side door\'s against the floor');
  const T = await rig(3, {}); const R = T.R, g = T.game, nav = g.col.nav;
  const S = SIDE_MARKS, sd = g.doors.byName('server_manager'), DTL = g.doorTools;
  const free = (p: number[]) => !nav.isBlocked(p[0], p[2]);
  const west = [...S.stack, ...S.standoff].every((p: number[]) => free(p) && INSIDE(p, SERVER));
  const reach = dist(S.stack[0], sd.frameCentre) < 1.8 && !!nav.findPath([20.4, 0, 8.7], S.stack[0]) && S.standoff.every((p: number[]) => dist(p, sd.frameCentre) > DOORTOOLS.backblastM + 0.15);
  F.verdict(!!sd && west && reach, 'the side door\'s marks: its stack on the server room\'s floor two strides from the leaf, the stand-off for a charge outside its backblast', `on free floor in the server room ${west} · the stack ${dist(S.stack[0], sd.frameCentre).toFixed(2)} m from the leaf's middle · stand-off ${S.standoff.map((p: number[]) => dist(p, sd.frameCentre).toFixed(1)).join(' / ')} m (backblast ${DOORTOOLS.backblastM})`);
  // cover: with the leaf out of the frame (as once it is shoved, kicked or blown), who in the hostage room has a line to a man on a mark?
  { const col = g.col, keep = col.dynamicBoxes; col.dynamicBoxes = keep.filter((b: any) => b !== sd.box);
    const line = (from: number[], p: number[]) => [1.2, 1.6].some(h => !col.shotBlocked([from[0], 1.45, from[2]], [p[0], h, p[2]]));
    const cells: number[][] = []; for (let x = OFFICE.x0 + 0.3; x < OFFICE.x1; x += 0.3) for (let z = OFFICE.z0 + 0.3; z < OFFICE.z1; z += 0.3) if (!nav.isBlocked(x, z)) cells.push([x, 0, z]);
    const posted = [...R.hostiles.filter((h: any) => INSIDE(h.char.pos, OFFICE)).map((h: any) => [...h.char.pos]), [23.4, 0, 6.6]];   // (the captor where he stands + the inside door man's mark)
    const all = [...S.stack, ...S.standoff], share = all.map((p: number[]) => cells.filter(c => line(c, p)).length / cells.length), seen = all.filter((p: number[]) => posted.some(c => line(c, p)));
    const old = [21.4, 0, 9.42], oldShare = cells.filter(c => line(c, old)).length / cells.length, oldSeen = posted.some(c => line(c, old));
    col.dynamicBoxes = keep;
    F.verdict(cells.length > 100 && posted.length >= 2 && seen.length === 0 && share.every((f: number) => f < 0.25) && oldShare > 0.25 && oldSeen,
      'the side door\'s marks are behind cover from the hostage room with the leaf open: nobody on a defender\'s mark has a line to a man on one, and under a quarter of the room\'s floor does', `seen from the defenders' marks ${seen.length} of ${all.length} · share of the room's ${cells.length} floor samples with a line: ${share.map((f: number) => (100 * f).toFixed(0) + ' %').join(' / ')} (the corner past the latch, where a stack would go by habit: ${(100 * oldShare).toFixed(0)} %, and in the inside door man's view ${oldSeen})`); }
  const posts = g.orders.stackPosts(sd, 1, R.ops), pods = DTL.pods(sd, 1);
  F.verdict(!!posts && posts.length === 2 && dist(posts[0].pos, S.stack[0]) < 0.05 && dist(posts[1].pos, S.stack[1]) < 0.05 && pods.length >= 1 && pods.every((q: any) => free(q.pos) && INSIDE(q.pos, OFFICE)),
    'the door tools take them: a stack on that face of the side door is the marks (its jambs have no floor), and its points of domination are on the office\'s floor', `stack ${posts?.map((q: any) => q.pos.map((v: number) => v.toFixed(1)).join(',')).join(' / ') ?? 'none'} · points inside ${pods.map((q: any) => q.pos.map((v: number) => v.toFixed(1)).join(',')).join(' / ')}`);
  const fd = g.doors.byName('manager'), fposts = g.orders.stackPosts(fd, 1, R.ops), M = g.orders.doorMarks;
  F.verdict(M.length === 1 && M[0].door === sd && !!fposts && fposts.every((q: any) => q.pos[2] > 10) && (R.stop(), g.orders.doorMarks.length === 0 && DTL.linked.length === 0),
    'the marks are the scenario\'s own: one door\'s, the corridor door stacked off its jambs as ever, and gone (with the two doors\' link) when it stops', `marks registered ${M.length} · corridor door stack ${fposts?.map((q: any) => q.pos.map((v: number) => v.toFixed(1)).join(',')).join(' / ')} · after stop(): ${g.orders.doorMarks.length} marks, ${DTL.linked.length} linked doors`);
  return F.done();
});

export const flowPass = guarded(async () => {
  const F = family('rescue-flow-pass: Sam walks the corridor and gives no order — the last man covers each opening, the other stays on him');
  { const T = await rig(12, { doorMan: 'inside', roamers: 0 }); const R = T.R, g = T.game, said = ears(T); T.secs(0.3);
    let inRoom = '', onMark: Record<string, boolean> = {};
    const each = () => {
      if (isNaN(R.t0) || g.time - R.t0 < 1.5) return;   // (the first second and a half is the pair falling in from the arch)
      for (const o of R.ops) { for (const [n, B] of [['conference', CONF], ['server', SERVER], ['cubicles', CUB]] as const) if (INSIDE(o.char.pos, B)) inRoom = `${o.callsign} in the ${n}`;
        const J = R.flow.jobOf(o); if (!J) continue; const O = R.flow.route.find((x: any) => x.id === J.opening);
        if (dist(o.char.pos, O.mark) < 0.6 && Math.abs(wrap(o.char.aimYaw - yawTo(o.char.pos, O.watch))) < 0.3) onMark[O.id] = true; }
    };
    walk(T, [8.2, 0, 11.2], 1.55, each); walk(T, [24.6, 0, 11.2], 1.55, each);
    const tStop = g.time; let back = NaN;
    for (let i = 0; i < 900 && isNaN(back); i++) { T.step(); each(); if (R.flow.jobs.size === 0 && R.ops.every((o: any) => orderOf(T, o) === 'fallIn' && dist(o.char.pos, g.player.char.pos) < 4)) back = g.time - tStop; }
    const L = flowLog(T), takes = L.filter((w: string) => w.startsWith('flow:take:')), ids = takes.map((w: string) => w.split(':')[2]), men = new Set(takes.map((w: string) => w.split(':')[3]));
    F.verdict(ids.join(',') === 'conference,cubicles-w,server,cubicles-e', 'every opening Sam passes is taken, in the order he passes them, with no order given', `${takes.join(' · ')} · menu orders in the log: ${g.orders.log.length} (the unasked fall in)`);
    const calls = ['glass left.', 'opening right.', 'door left.', 'opening right.'], heard = said.filter(s => calls.includes(s.text)).map(s => s.text);
    F.verdict(heard.join('|') === calls.join('|') && said.filter(s => calls.includes(s.text)).every(s => men.has(s.who)), 'each is called by the man taking it: glass left · opening right · door left · opening right', said.map(s => `${(s.t - R.t0).toFixed(1)} ${s.who}: ${s.text}`).join(' · '));
    F.verdict(Object.keys(onMark).length === 4, 'on each he stands on its mark with his gun on it', `on mark with the gun on the opening: ${Object.keys(onMark).join(', ') || 'none'}`);
    F.verdict(!inRoom && !T.bad, 'nobody goes into a room he was only told to pass', inRoom || 'never inside the conference room, the server room or the cubicles');
    F.verdict(back < 8, 'he catches up: both men on Sam again at the hostage room\'s door', `${f1(back)} s after Sam stopped at x ${g.player.char.pos[0].toFixed(1)} (that long a man short)`);
  }
  { // NOT a pass: into the cubicles from the lobby, and onto the corridor out of the cubicles' east doorway — nothing was walked past, nobody is sent back west
    const T = await rig(12, { doorMan: 'inside', roamers: 0 }); const R = T.R, g = T.game; T.secs(0.3);
    g.teleportPlayer([9, 0, 16]); T.secs(1.0); walk(T, [14, 0, 16]); T.secs(2);
    const hold = flowLog(T).some((w: string) => w.startsWith('flow:corridor'));
    walk(T, [22.75, 0, 14]); walk(T, [22.75, 0, 11.2]); walk(T, [24.6, 0, 11.2]); T.secs(3);
    const L = flowLog(T).filter((w: string) => /^flow:(take|pass|queue|corridor)/.test(w));
    F.verdict(!hold && L.length === 0 && R.ops.every((o: any) => orderOf(T, o) === 'fallIn') && !R.objective().text.includes('behind you'), 'round by the lobby and the cubicles, out of the east doorway to the hostage room\'s door: no corridor hold, nothing passed, nobody sent back', `corridor hold ${hold} · ${L.join(' · ') || 'no takes, no passes'} · '${R.objective().text}'`);
  }
  return F.done();
});

export const flowStates = guarded(async () => {
  const F = family('rescue-flow-states: what the player is shown of each opening');
  { const T = await rig(12, { doorMan: 'inside', roamers: 0 }); const R = T.R; T.secs(0.3);
    walk(T, [8.2, 0, 11.2]); walk(T, [24.6, 0, 11.2]); T.secs(8);
    F.verdict(stateOf(R, 'conference') === 'unknown' && /conf unknown/.test(R.objective().text), 'a look goes stale: ten seconds with no eyes on it and the conference room is unknown again', `'${R.objective().text}'`);
  }
  { // one of theirs in the conference room, seen through the glass: hot, and held for as long as he is
    const T = await rig(5, { doorMan: 'inside', roamers: 2, setSecs: 999 }); const R = T.R, g = T.game; T.secs(0.3);
    const man = R.hostiles.find((h: any) => INSIDE(h.char.pos, CONF));
    walk(T, [8.2, 0, 11.2]); walk(T, [12.5, 0, 11.2]);
    T.secs(5);
    if (man && man.state !== 'dead') killGuard(g, man, [1, 0, 0], false, 'shot', 'player');
    const done = T.secs(6, () => R.flow.jobs.size === 0);
    F.verdict(done && stateOf(R, 'conference') !== 'hot' && R.ops.every((o: any) => o.state === 'dead' || orderOf(T, o) === 'fallIn'), 'he is dead: the room is hot no longer and the man catches up', `state ${stateOf(R, 'conference')} · orders ${R.ops.map((o: any) => orderOf(T, o)).join(' / ')} · ${flowLog(T).filter((w: string) => /hold|done|hot/.test(w)).join(' · ')}`);
  }
  { // Sam turns into the conference room with both men on him: one holds the corridor; the room walked through is cleared, and is not covered when he passes it again
    const T = await rig(12, { doorMan: 'inside', roamers: 0 }); const R = T.R, g = T.game, said = ears(T); T.secs(0.3);
    walk(T, [8.2, 0, 11.2]); walk(T, [9.6, 0, 10.9], 2); T.secs(2.5);   // (short of the door: nothing passed yet, both men up with him)
    walk(T, [9.6, 0, 8.6], 2); T.secs(1.5);
    const cj = [...R.flow.jobs].find(([, J]: any) => J.kind === 'corridor'), inWith = R.ops.filter((o: any) => INSIDE(o.char.pos, CONF) || dist(o.char.pos, g.player.char.pos) < 3).length;
    const heldNow = R.flow.known.filter((k: any) => k.state === 'held').map((k: any) => k.o.id);
    F.verdict(!!cj && said.some(s => s.text === 'corridor.') && inWith >= 1 && heldNow.length === 0, 'Sam goes into the conference room: the last man holds the corridor at its door (corridor.), the other goes in with him; no opening reads HELD for it (his gun is down the corridor)', `corridor job: ${cj ? cj[0].callsign : 'none'} · men with Sam ${inWith} · held: ${heldNow.join(', ') || 'none'} · ${said.map(s => s.text).join(' ')}`);
  }
  { // a hot room goes cold: the man leaves it unseen, Sam walks the room — not HOT for ever
    const T = await rig(5, { doorMan: 'inside', roamers: 2, setSecs: 999 }); const R = T.R, g = T.game; T.secs(0.3);
    const man = R.hostiles.find((h: any) => INSIDE(h.char.pos, CONF));
    walk(T, [8.2, 0, 11.2]); walk(T, [9.6, 0, 10.9], 2); T.secs(1);
    const wasHot = stateOf(R, 'conference') === 'hot';
    if (man) { man.hold = true; man.char.pos[0] = 33; man.char.pos[2] = 21.5; if (man.char.bones.chest) { man.char.bones.chest[0] = 33; man.char.bones.chest[2] = 21.5; } }   // (out of the room, out of everybody's sight: the far end of the break room)
    walk(T, [9.6, 0, 8.6], 2); walk(T, [6.0, 0, 6.0], 2.5); walk(T, [12.0, 0, 6.0], 2.5); T.secs(1);
    const st = stateOf(R, 'conference');
    F.verdict(!!man && wasHot && st !== 'hot' && !/conf HOT/.test(R.objective().text), 'a hot room the team then walks through and finds empty goes cold (and can be cleared)', `was hot ${wasHot} · after the walk ${st} · ${flowLog(T).filter((w: string) => /hot|cold|cleared/.test(w)).join(' · ')} · man ${man?.state} at ${man?.char.pos.map((x: number) => x.toFixed(1))} · '${R.objective().text}'`);
  }
  return F.done();
});

export const flowOrders = guarded(async () => {
  const F = family('rescue-flow-orders: the menu overrides the flow; the switches');
  { const T = await rig(12, { doorMan: 'inside', roamers: 0 }); const R = T.R, g = T.game; T.secs(0.3);
    walk(T, [8.2, 0, 11.2]); walk(T, [11.0, 0, 11.2]);
    const J = [...R.flow.jobs.keys()][0]; const was = J ? orderOf(T, J) : 'none';
    const w = give(T, 'hold'); T.secs(0.3);
    const p0 = R.ops.map((o: any) => [...o.char.pos]);
    walk(T, [24.6, 0, 11.2]); T.secs(4);
    const drift = Math.max(...R.ops.map((o: any, i: number) => dist(o.char.pos, p0[i]))), L = flowLog(T);
    const takesAfter = L.slice(L.findIndex((x: string) => x.startsWith('flow:override')) + 1).filter((x: string) => x.startsWith('flow:take')).length;
    F.verdict(!!J && was === 'cover' && w === null && R.ops.every((o: any) => orderOf(T, o) === 'hold') && drift < 0.8 && L.some((x: string) => x.startsWith('flow:override')) && takesAfter === 0 && R.flow.jobs.size === 0,
      'hold, given while a man is on a cover: he is the menu\'s that frame; Sam runs on past three more openings and nobody is sent', `he was on '${was}' · orders ${R.ops.map((o: any) => orderOf(T, o)).join(' / ')} · moved ${drift.toFixed(2)} m · covers after the order ${takesAfter} · ${L.filter((x: string) => /override|unmanned/.test(x)).join(' · ')}`);
    g.teleportPlayer([13.0, 0, 11.2]); T.secs(0.2);
    const w2 = give(T, 'fallIn'); T.secs(4);
    const n0 = flowLog(T).filter((x: string) => x.startsWith('flow:take')).length;
    walk(T, [19.0, 0, 11.2]); T.secs(0.5);
    const n1 = flowLog(T).filter((x: string) => x.startsWith('flow:take')).length;
    F.verdict(w2 === null && n1 > n0, 'fall in hands them back: the next openings Sam passes are covered again', `covers before ${n0}, after ${n1} · ${flowLog(T).slice(-4).join(' · ')}`);
    // one man put on a cover by Sam: the flow leaves him, and — the other being the only man in the file — takes the openings with that one
    const k = R.ops.indexOf([...R.flow.jobs.keys()][0] ?? R.ops[0]) === 0 ? 1 : 0, stay = R.ops[k];
    const w3 = g.orders.give('cover', { kind: 'floor', at: [25, 0, 11.2] }, k); T.secs(0.3); const sp = [...stay.char.pos];
    walk(T, [24.6, 0, 11.2]); T.secs(6);
    F.verdict(w3 === null && orderOf(T, stay) === 'cover' && dist(stay.char.pos, sp) < 0.6 && !flowLog(T).some((x: string) => x.startsWith('flow:take') && x.endsWith(stay.callsign)), 'a man Sam posts on a cover of his own stays on it whatever Sam passes', `${stay.callsign}: ${orderOf(T, stay)}, moved ${dist(stay.char.pos, sp).toFixed(2)} m`);
  }
  { const T = await rig(12, { doorMan: 'inside', roamers: 0 }); const R = T.R, g = T.game; T.secs(0.3);
    killGuard(g, R.ops[0], [1, 0, 0], false, 'shot', 'guard'); T.secs(0.2);
    walk(T, [8.2, 0, 11.2]); walk(T, [14.0, 0, 11.2]); T.secs(0.3);
    const n = flowLog(T).filter((x: string) => x.startsWith('flow:take')).length;
    F.verdict(n >= 1, 'one man left: he still peels off to cover what Sam passes (flowSolo)', `covers ${n} · ${R.ops[1].callsign}: ${orderOf(T, R.ops[1])}`);
  }
  return F.done();
});

/** the server room opened (as a kick of Sam's opens it) and one man sent to stand by the side door on the menu's 'move there' */
async function sideRig(seed: number, knobs: Record<string, unknown> = {}) {
  const T = await rig(seed, { doorMan: 'inside', roamers: 0, ...knobs }); const R = T.R, g = T.game; T.secs(0.3);
  const sv = g.doors.byName('server'); g.doors.kickIn(sv, [17.6, 0, 11.0], g.player.char.id);
  T.secs(0.6);
  return { T, R, g };
}
const beat = (T: any, what: string) => T.at(what);

export const flowTwoDoor = guarded(async () => {
  const F = family('rescue-flow-twodoor: the hostage room through both doors on one F');
  const BREAK = !!process.env.GATEHAND_BREAK;   // (GATEHAND_BREAK=1: nobody is moved to the side door before the F — nothing is split, and the lines below must go red)
  // doors open one way: a can is not thrown at a leaf (DOORTOOLS.bangWay) and a SHOVED leaf is two tenths of a second longer getting out of its can's way
  // than a booted one: the leaves are still asked on one frame, the cans and the men a quarter of a second apart.
  const apart = () => 0.25;
  { let secured = 0, alive = 0, teamUp = 0; const SEEDS = sample([40, 41, 42, 43, 44, 45], 1), N = SEEDS.length;
    for (let s = 0; s < N; s++) {
      const { T, R, g } = await sideRig(SEEDS[s], {}); const DTL = g.doorTools;
      const w = BREAK ? 'not given (GATEHAND_BREAK)' : g.orders.give('move', { kind: 'floor', at: [20.4, 0, 8.7] }, 1);
      const there = T.secs(14, () => INSIDE(R.ops[1].char.pos, SERVER) && dist(R.ops[1].char.pos, [20.4, 0, 8.7]) < 1.2);
      T.press('KeyF');
      const split = R.log.find((l: any) => l.what.startsWith('split:front'))?.what ?? 'no split', kinds = R.entry?.ops.map((e: any) => `${e.op.kind}:${e.op.door.def.name}`).join(' + ') ?? 'no entry', oneGate = !!R.entry && R.entry.ops.every((e: any) => e.op.gate === R.entry.gate);
      // how far each leaf went in, and where the side leaf stood as its man was sent through it. Where it stands at the END is not asked of the side door: on this
      // stand (double-acting leaves, closers) a man who meets an open leaf's free edge on the way to his point pulls it to behind him — at real-size doors the
      // side man's point is on the hinge's hand of the door (the other hand's is in the north wall's margin), so he does. As shipped the leaf stays at its stop.
      const fd = g.doors.byName('manager'), sd = g.doors.byName('server_manager'); let inF = 0, inS = 0, sideAtIn = NaN;
      T.secs(40, () => { inF = Math.max(inF, Math.abs(fd.angle)); inS = Math.max(inS, Math.abs(sd.angle)); if (isNaN(sideAtIn) && T.at('enter:side') !== undefined) sideAtIn = Math.abs(sd.angle);
        return !!T.at('clear') || !!R.result || (!!T.at('enter1') && T.game.time - T.at('enter1')! > 16); });
      const fl = beat(T, 'door-flung'), fs = beat(T, 'door-flung:side'), e1 = beat(T, 'enter1'), es = beat(T, 'enter:side'), b1 = beat(T, 'bang'), bs = beat(T, 'bang:side');
      if (s === 0) { const APART = apart();
        F.verdict(w === null && there && split === 'split:front=Blue One,side=Blue Two' && kinds === 'bangClear:manager + bangClear:server_manager' && oneGate, 'a man moved to the side door on the menu, then one F: he has the side door, the other the corridor door — two of the door tools\' bang and clears on one gate', `move there: ${w ?? 'given'}, in place ${there} · ${split} · ${kinds} · one gate ${oneGate}`);
        F.verdict(T.order(['split:set', 'split:go', 'kick', 'door-flung', 'toss', 'bang', 'enter1']) && fl !== undefined && fs !== undefined && Math.abs(fl - fs) < 0.02 && b1 !== undefined && bs !== undefined && Math.abs(b1 - bs) < APART && !!T.at('toss:side') && e1 !== undefined && es !== undefined && Math.abs(e1 - es) < APART && DTL.bangs === 1,
          `both set → both leaves on one frame → a can through each (two of the squad\'s three), off together → both men in, inside ${APART > 0.1 ? 'a quarter' : 'a tenth'} of a second of each other`, `leaves ${f1(T.since('door-flung'))} / ${f1(T.since('door-flung:side'))} s · bangs ${f1(T.since('bang'))} / ${f1(T.since('bang:side'))} s · in ${f1(T.since('enter1'))} / ${f1(T.since('enter:side'))} s (from the clock's start) · set ${f1(T.since('split:set'))} s · bangs left ${DTL.bangs}`);
        F.verdict(Math.abs(fd.angle) > 0.8 && inS > 0.8 && sideAtIn > 0.8 && fd.lockBroken && !sd.lockBroken, 'both leaves go in: the corridor door booted off its lock and standing in, the side door (unlocked at staging) shoved with the boot and in as its man is sent through',
          `corridor door in to ${inF.toFixed(2)} rad, ${fd.angle.toFixed(2)} at the end, lock broken ${fd.lockBroken} · side door in to ${inS.toFixed(2)} rad, ${isNaN(sideAtIn) ? 'nobody sent through it' : `${sideAtIn.toFixed(2)} as its man was sent in`}, ${sd.angle.toFixed(2)} at the end, lock broken ${sd.lockBroken}`);
      }
      T.secs(6, () => R.state === 'secured' || !!R.result);
      if (R.state === 'secured' || R.result?.state === 'rescued') secured++; alive += up(R.hostages); teamUp += up(R.ops);
    }
    F.verdict(secured >= N / 2, `over ${N} seeds the two-door entry takes the room at least half the time (Sam a statue in the corridor)`, `secured ${secured}/${N} · hostages alive ${alive}/${2 * N} · pair up ${teamUp}/${2 * N}`);
  }
  { const { T, R, g } = await sideRig(9, {});
    walk(T, [8.2, 0, 11.2]); T.secs(1.5);
    const w = g.orders.give('cover', { kind: 'floor', at: [20, 0, 11.2] }, 'all'); T.secs(0.3);
    const did = R.press();
    F.verdict(w === null && did === null && R.mode === 'free' && g.doorTools.bangs === 3 && /nobody's free/.test(g.messages[g.messages.length - 1]?.text ?? ''), 'both men on covers of Sam\'s: F is refused, and says why (no can spent)', `did ${did} · '${g.messages[g.messages.length - 1]?.text}'`);
  }
  { // the menu calls it off — on the walk, and with the boots going in (the door tools' men are theirs only while they stand on the order)
    const { T, R, g } = await sideRig(11, {}); const DTL = g.doorTools;
    g.orders.give('move', { kind: 'floor', at: [20.4, 0, 8.7] }, 1); T.secs(14, () => dist(R.ops[1].char.pos, [20.4, 0, 8.7]) < 1.2);
    T.press('KeyF'); T.secs(1.0);
    const owned = R.ops.map((o: any) => R.owns(o)).join('/'), spent = DTL.bangs;
    const w = give(T, 'hold'); T.secs(0.3);
    F.verdict(owned === 'true/true' && spent === 1 && w === null && R.mode === 'free' && !!T.at('drill:called-off') && R.ops.every((o: any) => orderOf(T, o) === 'hold') && DTL.bangs === 3, 'a menu order on the walk to the two doors calls it off, they take the order, and both cans come back', `F had them ${owned} · hold: ${w ?? 'given'} · orders ${R.ops.map((o: any) => orderOf(T, o)).join(' / ')} · bangs ${spent} → ${DTL.bangs}`);
  }
  return F.done();
});
