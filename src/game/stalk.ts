// THE STALK (STALK) — the last three metres to a man's back are one held click. The maintainer's design: "when you're close enough to an enemy (like entering the
// closer than ever state) you can click and hold on an enemy and sam will creep toward him · you might want to click, let go to stop, and click again to continue —
// really works well with the paused animation · the game also controls the trajectory and speed and makes things smooth. it ends with you grabbing him · you get
// kicked out of the mode if he gets too far away, turns around, goes through a door in a way that doesn't make sense". Before it those metres were the player's own
// keys into a window a metre deep behind a man who is walking, seen from 21 m up.
//
//   THE HOLD     hands empty, a living man who is not onto him, inside close.ts's mode range (CLOSE.modeM) with nothing solid between, Sam in his rear arc: the LEFT
//                button PRESSED on him (the reticle's man, or the cursor within `cursorM` of him) takes him as the man stalked. While the button stays down the game
//                walks Sam: crouched, toward a point `stationM` behind the man's back, at the man's own speed plus a closing speed that eases off as he nears
//                (`closeMin` + `closeK` a metre out from `grabM`, never over `pace`), facing him. The cursor is free after the press: the man walks, the mouse need not.
//   LETTING GO   the button up: nothing is stalked. Sam is where he is, in the mode's held stride (character.ts creep: the pose stops mid-step), and his keys are
//                his own. The next press takes the man again.
//   THE GRAB     inside `grabM` with the grab on offer (close.ts closeAt: behind him, in reach, room to stand): the hold becomes the grab — game.ts takes
//                `stalkGrab` exactly where it takes the row's (player.ts startGrab).
//   OUT          he is dead, held, onto Sam, or out of the picture · past CLOSE.modeOutM · a wall or a door leaf between them · he has turned until Sam is no longer
//                behind him (`outCos`: a few degrees of grace past the rear arc's own edge) · Sam's hands are no longer empty or are busy · a move key of the
//                player's own (his feet are his again at once; with `orbit` up his keys take him round the man instead, below). Out = nothing stalked until the button is pressed afresh; a button merely still down does nothing.
//
// What it does NOT touch: what the men see and hear (a creeping Sam files the crouch's footfalls and moves under their eyes as any crouched Sam), the grab itself,
// a civilian's own grab row. With `only` (below) a staff man's 'grab him' row is gone: the held click is the one way to him. The left click that began or carries a stalk is no blow (game.ts asks `stalkHas`).
// ON in the page unless ?stalk=0 (src/host/profile.ts pageSwitches); off here and on the harness's frozen stand (src/host/standup.ts; TPS_STALK=1 turns it on
// there). Off = the build as it was. Check: tools/qa/stalk-check.ts (STALK_BREAK=1 must fail).
// ITS POLISH (the fourth to sixth passes: stalkPolish below) is a hold of its own: down in this table and on the frozen stand, so a frozen run raised with
// TPS_STALK=1 is the stalk as it first stood; TPS_STALKPOLISH=1 raises it there, the page has it up unless ?stalkpolish=0.
import { SideMap } from '../math/side';
import { Vec3, clamp } from '../math/vec';
import type { Game, Guard } from './game';
import { CLOSE, closeLive, closeSubject, closeClear } from './close';
import { holdPolish } from './character';
import { samStaggered } from './samstun';
import { canStand } from './player';
import { RUST_PASS, RUST_SURFACE, crossToRust, goneWithTick } from './crossing';

export const STALK = {
  on: false,
  /** THE CLICK IS THE ONLY WAY IN (the maintainer: "we should get rid of E to grab then. lmb is the only way"): with the stalk on, a staff man has no 'grab him'
   *  row — the list says 'hold LMB to grab him' and the interaction key does nothing to him. A civilian keeps her row (nobody stalks her). Nothing with `on`
   *  off. ?stalkonly=0 puts the row back beside the stalk. */
  only: true,
  /** m: the press is ON a man when the cursor's floor point is this near him (or the reticle has him) */
  cursorM: 1.1,
  /** cos of his bearing to Sam under which a press can begin one (−0.25 = the grab's own rear arc, 104.5° off his facing) · and past which Sam is no longer
   *  behind him and the stalk is over (93°: a few degrees of grace so a man swaying on his feet does not throw him out) */
  beginCos: -0.25, outCos: -0.05,
  /** m: the point behind his back Sam is walked toward · inside this the hold becomes the grab (under close.ts's reach of 1.05) */
  stationM: 0.55, grabM: 0.95,
  /** m/s: never faster than this · the closing speed on the man at `grabM` · and what each metre beyond it adds */
  pace: 1.6, closeMin: 0.3, closeK: 0.6,
  /** 1/s: how quickly his speed takes up what the stalk asks, speeding up · and slowing (his own walk's are 6 and 12: gentler in, so the creep starts like a breath) */
  easeUp: 4, easeDown: 9,
  /** THE BODY SAYS HOW CLOSE (the maintainer: "we want very very smooth transitions from regular crouching to the closer than ever state. then, once you click
   *  and hold its very smooth to gradually shift the animation until you end on a grab, and you can be interrupted any time"). Behind a man he could stalk, or
   *  stalking him, Sam's pose is a function of the DISTANCE to that man's back and of nothing else: from `reachFrom` m in, the left arm comes up for his neck
   *  (the tell's own reach, character.ts readyLayer, at a weight that is `reach` by the time the grab fires), and from `riseFrom` m in he comes up out of the
   *  crouch (`rise` of the way to his feet by `riseBy` m — the grab itself stands him up the rest). Both run straight by the distance. Because it is the distance's, letting go of the button holds
   *  the pose where it is — a frame of the move, paused — and the next press goes on from it; his own keys behind the same man show the same body. What the men
   *  see and hear of him is the crouch's, as it was. `tell` off = the tell in reach alone, as before the stalk. */
  tell: true, reachFrom: 2.9, reach: 1, riseFrom: 2.3, riseBy: 1.05, rise: 0.7,
  /** THE STEP IN: the grab a stalk ends in closes the last of the gap in one step over `catchSecs` (a step longer than `catchM` takes up to `catchLong` times that: the haul back has what is left of HOLD.secs.grab; `turnIn`: he comes round to the man's back through the step, not in its first frames) —
   *  a lunge: off the mark over its first three frames, most of the way inside a third of it, landing gently (stepEase). The grab's own way in covers a
   *  fifth of the gap in its first FRAME. His legs are given the step, at no more than `catchStride` m/s. 0 = the grab's own.
   *  THE YANK: once the step has landed the man is hauled `yank` m straight back the way he was facing from — Sam going back with him (`yankWith`; off, he stays where he landed and the man comes 11 cm into him) — over what is left of
   *  the grab: movement across the floor is what the game's camera shows of a grab (the pose's share: character.ts HOLD.yank). Less where something solid
   *  stands behind the pair (`yankRoom` m kept clear behind Sam); 0 = he is held where he stood. */
  catchSecs: 0.3, catchM: 0.75, catchLong: 1, turnIn: false, catchStride: 1.5, yank: 0.28, yankRoom: 0.4, yankWith: false,
  /** THE SAME AT A CORNER (the maintainer: "curious if we can make this language work for working corners. like when you're at a corner you can either grab
   *  or CQC with the right timing"). With his back on a wall (the wall hold, not the stack) and his hands empty, the left button HELD is a man lying in wait:
   *  nothing moves, and the moment a man who is not onto him is inside `cornerM` with Sam at least `cornerDeg`° round from his nose — he has come past the
   *  corner, most of a pace — Sam comes off the wall onto him: the grab, by the same eased step in. Pressed too soon it simply waits; let go, nothing happens; a man who comes
   *  round FACING him is not taken (the right button's knockout is still the player's own timing, as off a wall). `corner` off = the button on a wall does
   *  nothing, as before. The page: ?stalkcorner=0. */
  corner: true, cornerM: 1.35, cornerDeg: 110,
  /** BROKEN OFF BY THE MAN (stalkStep): `jolt` = how much of the animator's duck (character.ts flinchFrom, soft) Sam gives when the man turns off his back with
   *  the stalk on AND comes on round to face him (Sam inside `joltFace`° of his nose within `joltSecs` s: a patrol's quarter turn is not that) — the whole of it,
   *  at once, when the man is onto him. 0 = none: the stalk ends and he sinks back into the crouch, as it was. joltFace 180 = at any turn that ends the stalk.
   *  `joltAtOnce` = the duck for a turn on the out's own frame with none of brokeDuck's tests: the fifth pass as it was, raised only by journal.ts for a recording of it.
   *  THE WAIT SHOWS (closeTick): on the wall with the button held he coils — sunk `cornerCoil` of the way into the crouch, the lead hand tucked toward the man at `cornerReach[0]`
   *  of the grab's reach, growing to `cornerReach[1]` as the nearest man he could take comes from `cornerSee` m to `cornerM`; he comes up out of the coil over
   *  `cornerRise` s of the step in. `cornerShow` off = the wall stance alone, as it was.
   *  (Of the polish, so DOWN here: `jolt` 0 and `cornerShow` off — and above, `catchLong` 1, `turnIn` and `yankWith` off. stalkPolish(true) writes POLISH_UP's.) */
  jolt: 0, joltFace: 50, joltSecs: 0.6, joltAtOnce: false, cornerShow: false, cornerSee: 3.2, cornerReach: [0.25, 0.55] as [number, number], cornerCoil: 0.3, cornerRise: 0.24,  /** THE KEY FINISHES IT (the maintainer, having played the creep: "one change we should make is that I think you should really have to hit E to finish
   *  the grab. it feels weird having it happen automatically"). With `key` up the held click only walks him in: inside the grab's reach he eases to a stop `keyM` m
   *  behind the man and waits there — keeping station, at the man's own speed, if the man walks on — and the grab is the row's own, 'grab him' on the interaction's key (E in the page), back
   *  whatever `only` says and taken AS the stalk's: the step in, the landing, the haul back (game.ts, stalkKeyGrab). He may also walk in on his own keys
   *  and press it. A corner's wait still takes its man by itself (its timing is the press). OFF HERE and on the frozen stand: the hold becomes the grab by
   *  itself, as it was. The page: ?stalkkey=0. */
  key: false, keyM: 0.8,
  /** LOWER, HANDS DOWN (the same message: "i do think having the hands up just doesn't look cool. in chaos theory sam starts regular crouch walk, then
   *  goes lower to the ground and creeps much more deliberately (and that's where stopping just pauses the animation). and when he's really close he
   *  reaches for his knife (maybe we'd just have our hands down)"). With `low` up the tell is the other way about: no arm comes up and he does not rise —
   *  from `lowFrom` m in he SINKS, all the way down by `lowBy` m (the shape of it: `lowDrop` and the two after it — the pelvis lower, the back flatter),
   *  still straight by the distance, so letting go holds it; the creep's carriage keeps its hands DOWN at any distance (`lowHands`: the crouch's own, by
   *  his knees); and each metre beyond the reach adds `lowK` m/s to his closing speed where it added `closeK`: slower, the whole way in. He comes up out
   *  of it in the grab, and fast (`lowSpring`, the rate he leaves the crouch at through a stalk's grab, where the animator's own is 10): the spring. OFF
   *  HERE and on the frozen stand: the arm comes up and he rises, as it was. The page: ?stalklow=0. */
  low: false, lowFrom: 2.6, lowBy: 1.1, lowK: 0.35,
  /** …its shape, all the way down (fed to the animator, whose CREEP.low has the same numbers until it is): the pelvis this much lower (m) and further back
   *  (m) and the spine bowed this many degrees more; the share of the carriage's raised hands given up; the spring's rate, and the seconds of the grab it winds up to it over */
  lowDrop: 0.06, lowBack: 0.03, lowLean: 6, lowHands: 1, lowSpring: 18, lowWind: 0.12,
  /** ARMS OUT (under `low`, bare hands, in his crouch). Closing on a man from behind, his hands are not tucked by his knees: each is out `wideOut` m to its own
   *  side of its shoulder, `wideDown` m under it and `wideFwd` m ahead of it (the left `wideLead` m further, the right as far back), open, palm to the floor,
   *  the elbows bent and back (`widePole`, the elbow's way off the shoulder line: left, up, forward): the Chaos Theory creep's, read frame by frame (the first numbers flung them out like wings). It comes with the sink (all of it once he is `wideBy`
   *  of the way down), slowly, and lets go fast (the animator's CREEP.wide rates); it holds for as long as he waits behind the man ("that's where stopping
   *  just pauses the animation"), and goes with the carriage when there is no man in front of him. OFF HERE and on the frozen stand: his hands by his knees, as it was. The page: ?stalkwide=0. */
  wide: false, wideOut: 0.1, wideDown: 0.32, wideFwd: 0.24, wideLead: 0.07, wideBy: 0.35, widePole: [0.45, 0.15, -0.88] as Vec3,
  /** WEIGHT IN IT (the maintainer of the arms above, on a GPU: "(1) looks weird, but (2) also just looks like very static. the chaos theory animations have some real
   *  weight but also smoothness and precision to them"). With `heft` up his trunk and arms are carried by his legs (character.ts CREEP.heft): his back bowed `heftLean` °
   *  further, his shoulders `heftTwist` ° against each step, his weight `heftSide` m over the foot he is on and his hips `heftDip` m lower through the wide of a step,
   *  each hand swung `heftSwing` m ahead and back against its own foot a beat behind the trunk, and `heftSurge` of his setting off and stopping in his back and his hands.
   *  The arms hang from the shoulders with a bent elbow (`heftPole`): `heftOut` m out, `heftDown` m under, `heftFwd` m ahead (behind, so the hands sit by his hips; the left
   *  `heftLead` m further). Ahead of the shoulder they read from the game's camera, side on, as arms held out (his note on a GPU: "almost zombie like"), though not from behind. ON in the page (?stalkheft=0: the arms as they were), OFF here and on the frozen stand. */
  heft: false, heftTwist: 20, heftSwing: 0.11, heftSide: 0.04, heftLean: 4, heftDip: 0.02, heftSurge: 1, heftOut: 0.18, heftDown: 0.4, heftFwd: -0.2, heftLead: 0, heftPole: [0.6, 0, -0.8] as Vec3,
  /** THE STICK (the maintainer: "you click on the guy and then you can drag towards him to move in that direction. like a joystick centered on the guy.
   *  that would make things feel more controlled than right now which feels automatic"). With `drag` up, the press on a man still begins the creep, and the
   *  point on the screen it was made at is the stick's middle: while the button is held, the mouse's way from there is the way Sam creeps (screen up = the
   *  camera's forward, as the move keys have it), and how far it is, from `dragDead` px (nothing) to `dragFull` px (all of it), is how much of `pace`. The
   *  mouse left where it was pressed = he holds still in the pose. He faces the man throughout; he is never walked closer than `keyM` (the way into the man
   *  is taken off the stick there, the way round him is not); the rest is the creep's own rules (the arc, the walls, the key's grab). PROTOTYPE: OFF here,
   *  on the frozen stand AND in the plain page; up in the sneak lab (?sneaklab) and anywhere with ?stalkdrag=1; ?stalkdrag=0 puts it down in the lab. */
  drag: false, dragDead: 14, dragFull: 140,
  /** ROUND HIM (the maintainer: "when you're clicking on an enemy and moving around them, you should really be "orbiting" them"). With `orbit` up, a move
   *  key while the button holds a man no longer ends the creep: the way the keys point is taken round the man, not past him. The part of it toward or away from
   *  him closes or opens the gap; the part across him carries Sam round on the circle he is on (he keeps facing the man, and a pull of `orbitK` per metre off
   *  that circle holds him to it), at the creep's `pace`, and a man who walks takes the circle with him. The stick (`drag`) is taken the same way. He goes no
   *  further round than `orbitEdge` short of the grab's own arc (close.ts BEHIND_COS, inside where the creep ends), and no closer than `keyM`. Keys let go: the held button walks him in as before. With it up he faces the man in any creep, the aim's weight (aimweight.ts, which turned him
   *  to the cursor's floor point, left where the press was) has no say there. A key that is within `orbitSnap` of straight across him (its share toward him,
   *  0.3 ≈ 17°) is taken as all the way across: the screen's keys are seldom square to the man, and one a little off would spiral Sam in or out. ON in the page
   *  (?stalkorbit=0: a move key ends the creep, as it was), OFF here and on the frozen stand. */
  orbit: false, orbitK: 3, orbitEdge: 0.05, orbitSnap: 0.3,
  /** …and with it a press can begin one from further off ("i think we can make the start radius a bit larger too"): inside `lowBeginM`, kept out to `lowOutM`,
   *  where close.ts's mode range (3.0 and 3.6 m) was the limit. From out there he comes on at his plain crouch walk; the mode's carriage and the sink come in inside it */
  lowBeginM: 4.2, lowOutM: 5,
};
/** tallies for the checks and the page's hook (begun = presses that took a man; out = why each ended without a grab) */
export const STALK_STATS = { begun: 0, grabs: 0, corner: 0, out: { let: 0, far: 0, turned: 0, wall: 0, him: 0, hands: 0, keys: 0 }, last: '' };
type Why = keyof typeof STALK_STATS.out;

interface St { on: Guard | null; grab: Guard | null; t: number; /** on a wall with the button held and nothing in the way of a grab: he is lying in wait */ wait?: boolean; /** the button went down and has not come up or taken a man since: a stalk may begin */ armed?: boolean;
  /** STALK.drag: where on the screen (px) the press that began this stalk was made — the stick's middle */ stick?: [number, number] | null;
  /** STALK.orbit: the circle round the man he is held to (m), the frame it was last held on, and the way asked for as it was taken: on the floor, then its
   *  share into the man and its share round him (+ = the way round that turns left about him) */ orbitR?: number; orbitT?: number; orbitAsk?: [number, number, number, number];
  /** the man who broke a stalk off by turning, and when: if he comes round to FACE Sam inside STALK.joltSecs, Sam ducks */ broke?: { gd: Guard; t: number; /** where Sam stood */ at: Vec3 } | null }
const STATE = new SideMap<Game, St>('stalk.STATE');
/** every key from birth and in one order: a restart's state is compared with an untouched level's, key for key, whatever order play reached them in */
const fresh = (): St => ({ on: null, grab: null, t: -1, wait: false, armed: false, stick: null, orbitR: undefined, orbitT: undefined, orbitAsk: undefined, broke: null });
function end(S: St, why: Why) { if (S.on) { S.armed = false; S.on = null; STALK_STATS.out[why]++; STALK_STATS.last = why; } S.orbitR = S.orbitT = undefined; S.orbitAsk = undefined; }

/** are Sam's hands free for it at all (empty, and nothing else has him) */
function hands(g: Game): boolean {
  const pl = g.player;
  return STALK.on && closeLive(g) && !g.puppet && pl.holstered && !pl.down && !pl.holding && !pl.takedown && !pl.dragging && !pl.kick && !pl.picking && !pl.wanding && !pl.wall && !pl.listOpen && able(g);
}
/** …and is the held click his to begin one with (no step of a tutorial mission has taken it) */
const free = (g: Game): boolean => hands(g) && !shut(g);
/** a tutorial mission's step has taken the stalk (rails.ts `shuts`, the frame's word `stalk`): no press begins one, from the floor or from a corner, and
 *  nothing that asks whether he could — the row's hint, the pose that tells — says he can. IT IS THE CLICK THAT IS TAKEN, NOT THE GRAB: the row's key
 *  still grabs a man he has walked up behind, with the stalk's own step in (`stalkKeyCan` asks `hands`), and where the click was the one way to a man
 *  the row is back (`stalkClickOnly`) */
const shut = (g: Game): boolean => !!g.rails?.shuts('stalk');
/** the gate game.ts puts on a grab, so nothing is handed over that it would refuse: not folded by a blast, no hand on a breaker's lever */
const able = (g: Game): boolean => !samStaggered(g) && !g.player.breakerMove;
/** why this man cannot be stalked from `p` right now, or null when he can (`begin`: the stricter test a press must pass — the rear arc itself) */
/** how far off a press can begin one: close.ts's mode range — and with STALK.low the wider `lowBeginM` (he starts at his plain crouch walk and goes lower inside the mode) */
const beginM = (): number => STALK.low ? Math.max(CLOSE.modeM, STALK.lowBeginM) : CLOSE.modeM;
const outM = (): number => STALK.low ? Math.max(CLOSE.modeOutM, STALK.lowOutM) : CLOSE.modeOutM;
/** how far off a press begins a stalk, and how far off one is given up (the lab's rings, close.ts's hint) */
export function stalkBeginM(): number { return beginM(); }
export function stalkOutM(): number { return outM(); }
/** is the held click the ONE way to grab a man from behind (no row grabs him)? `only`, until the key finishes the grab (STALK.key) — and still on a wall,
 *  where the wait at the corner is the click's and no stalk can be the key's */
export function stalkClickOnly(g?: Game): boolean { return STALK.on && STALK.only && (!STALK.key || !!g?.player.wall) && !(g && shut(g)); }   // (a step that takes the click gives the row back: a line that named the click would name a dead button)
/** COULD A PRESS ON THIS MAN BEGIN ONE from where Sam stands (the row's hint asks) */
export function stalkCould(g: Game, gd: Guard): boolean { return goneWithTick('stalkCould') as any; }

/** STALK.corner: the man the wait at a corner takes this frame — the nearest who has come past him — or null (not on a wall with empty hands; nobody past) */
function cornerGate(g: Game): boolean {
  const pl = g.player, w = pl.wall;
  return !(!STALK.on || !STALK.corner || !w || w.kind === 'stack' || w.drawn || !closeLive(g) || g.puppet || !pl.holstered || pl.down || pl.holding || pl.takedown || pl.dragging || pl.kick || pl.picking || pl.wanding || pl.listOpen || !able(g) || shut(g) || (pl.crouch && !canStand(g)));   // (the last: startGrab would refuse it, with its line, every frame the button stayed down)
}
/** THE POLISH, UP OR DOWN TOGETHER (the page's ?stalkpolish=0, and journal.ts for a recording from before the fourth pass): the turn through the step, the longer
 *  long step, the wait's pose at a corner, the duck when the man turns round — and the hold's share (character.ts holdPolish). Up = POLISH_UP's numbers, as
 *  the page has them; down = the tables' own, as the frozen stand has them (src/host/standup.ts: TPS_STALKPOLISH=1 raises them on a frozen run). */
const POLISH_UP = { turnIn: true, cornerShow: true, yankWith: true, catchLong: 1.1, jolt: 0.6 } as const;   // (THE PAGE'S NUMBERS — a tuned one goes here, not in the table, which has them down)
const POLISH_DOWN = { turnIn: STALK.turnIn, cornerShow: STALK.cornerShow, yankWith: STALK.yankWith, catchLong: STALK.catchLong, jolt: STALK.jolt } as const;   // (the table's own, read once as this file loads)
export function stalkPolish(up: boolean) {
  const U = up ? POLISH_UP : POLISH_DOWN; STALK.turnIn = U.turnIn; STALK.cornerShow = U.cornerShow; STALK.yankWith = U.yankWith; STALK.catchLong = U.catchLong; STALK.jolt = U.jolt; holdPolish(up);
}
/** THE WAIT AT A CORNER, ON HIS BODY (close.ts closeTick): lying in wait this frame — where his hands point (the neck of the nearest man he could take, else
 *  out from the wall at his own shoulder height), how much of the grab's reach, and how far he is sunk */
export function stalkWait(g: Game): { at: Vec3; reach: number; coil: number } | null { { const __s = RUST_SURFACE; if (__s) { const __r = __s.call('stalkWait', undefined, [g]); if (__r !== RUST_PASS) return __r as any; } }
  const S = STATE.get(g), pl = g.player, w = pl.wall; if (!STALK.cornerShow || !S || !S.wait || !w || g.time - S.t > 0.1 || !cornerGate(g)) return null;
  const p = pl.char.pos; let best: Guard | null = null, bd = STALK.cornerSee;
  for (const gd of g.guards) { if (gd.civ || !closeSubject(g, gd) || gd.state === 'alert') continue; const d = Math.hypot(p[0] - gd.char.pos[0], p[2] - gd.char.pos[2]); if (d < bd && closeClear(g, gd.char.pos, p)) { best = gd; bd = d; } }
  const neck = best ? best.char.bones.neck ?? best.char.bones.head : null, R = STALK.cornerReach;
  const at: Vec3 = neck ? [neck[0], neck[1], neck[2]] : best ? [best.char.pos[0], 1.45, best.char.pos[2]] : [p[0] + w.n[0] * 0.8, 1.35, p[2] + w.n[2] * 0.8];
  const near = best ? clamp((STALK.cornerSee - bd) / Math.max(0.05, STALK.cornerSee - STALK.cornerM), 0, 1) : 0;
  return { at, reach: R[0] + (R[1] - R[0]) * near, coil: STALK.cornerCoil };
}
/** STALK.key: the row's grab on this man is about to be taken (game.ts) — is it the stalk's? With the key's rule up, his hands as a stalk wants them and
 *  room to come up it is, whether the held click walked him in or his own keys did. A question only: `stalkKeyTook` once the grab has him. */
export function stalkKeyCan(g: Game, gd: Guard): boolean { return crossToRust('stalkKeyCan', undefined, [g, gd]) as any; }
/** the man being stalked right now, for the picture and the checks */
export function stalkOn(g: Game): Guard | null { return STATE.get(g)?.on ?? null; }
/** a reset of the floor's men (game.ts resetGuards), a load: nothing is stalked */
export function stalkReset(g: Game) { const S = STATE.get(g); if (S) Object.assign(S, fresh()); }

