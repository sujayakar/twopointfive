// TEAMMATES ARE SLOWER THAN THE PLAYER, AND SPRINT ONLY FOR A REASON (TEAMPACE.on: on in the page, ?teampace=0; off here and on the frozen stand, TPS_TEAMPACE=1).
//
// The play note this answers: the player moves at 1×, a teammate at 0.8× — "when you send them an order and they start moving, you can always catch up
// with them"; "if you sprint to go somewhere … the teammates should also sprint when they're following you"; sprinting is "in their toolbox, but they should
// use it pretty infrequently, like maybe only if they're running to cover" — "it should just generally make sense, because sprinting is actually just very risky".
//
// Before it the team's legs were capped at guards.ts PACE.max (2.2 m/s) whatever was asked, the player walked at 1.55 and ran at 4.7: an ordered team out-walked
// him to every door, and a file left behind by his sprint closed up at 2.2.
//
//   THE WALK     (with `pacing`, in the table below, this flat share is what he slows to while the player walks after him, and his walk is the player's)
//                a teammate's legs are asked for no more than `k` × the player's walk (tune.playerWalk) on an order or on his own — guards.ts teamPace's cap,
//                in PACE.max's place, for the same men (teamPaced). His card still weighs on it (kitload.ts), a hurt man's cap and the silent squad's are under it.
//   FOLLOWING    a man falling in who is off his place closes at up to `followK` × the player's walk (teamFollow), so a walking player does not string his
//                file out behind him. While the PLAYER SPRINTS a follower who is `behindM` off his place — and has nobody to fight — runs with him at the
//                player's own sprint, and keeps it until the player stops running (`follow`).
//   THE FIGHT    what the fight asks of his legs is not a walk to a place and keeps the pace it had (PACE.max): a man who has somebody in his sights or lost
//                one `fightSecs` ago; the leg through an ordered door (ORDERS.entry: the stun's seconds are counted on it); a step the
//                room fight orders (off a doorway's opening, round a friend in his line); the breach drill; a man leaving a grenade who may not run;
//                and a man a grenade took off his place, for `backSecs` after it (a door order waits for its men, and counts the seconds).
//   A REASON     on his own (`own`) a teammate RUNS for one of two things — and for a third when a tutorial step tells him to (`told`: teamTold, at the
//                foot of this file) — each written down as it is taken (TEAMPACE_STATS, TEAMPACE_DEBUG.log):
//                  blast   he is running from a live grenade (frag.ts fragRunning) — at the pace the flee asks, as anybody's legs;
//                  cover   loud has just reached him, he has no man in his sights, the floor he is on is in the line of a man he knows of, his mark is out
//                          of it and in plain sight across `dashM` or more of the same floor: he runs the leg (teamDash) — never round a corner, never
//                          through a doorway, never a man falling in (his place is the player's to move).
//                Never the silent squad, a squad nobody has found, a man sneaking with the player (blast apart: nobody walks off a grenade), never a hurt man.
//   THE COST     a teammate at a run (`runAt`, the sprint clip's own threshold) carries his gun low and fires nothing (teamSprinting: squad.ts, roomfight.ts),
//                his boots are a sprinter's (`step`: the player's own level) to whoever hears them, and a leaf he meets is shouldered as a runner's is.
//
// Off = the build as it was: every function here answers on its first line.
import type { Game, Guard } from './game';
import type { Vec3 } from '../math/vec';
import { SideMap } from '../math/side';
import { PACE, teamPaced } from './guards';
import { fragRunning } from './frag';
import './sneak';
import './unseen';
import './silent';
import { isHurt } from './condition';
import './picture';
import './kitload';
import { crossToRust, goneWithTick } from './crossing';

export const TEAMPACE = {
  on: false,
  /** a teammate's walk, × the player's (tune.playerWalk) */
  k: 0.8,
  /** a follower off his place closes at up to this × the player's walk (at 1 a file strings out behind a walking player: nobody closes on a man at his own pace) */
  followK: 1.3,
  /** followers run while the player sprints; `behindM`: how far off his place a man starts to */
  behindM: 1.0,
  /** a man who lost his man this long ago still moves at the fight's pace (s) */
  fightSecs: 2,
  /** … and a man a grenade took off his place goes back to it at the fight's pace, this long after he stopped running from it (s; 0 = at the walk) */
  backSecs: 6,
  /** on their own: a run for a reason (blast, cover) */
  own: true,
  /** a run to cover: the least leg worth running (m), how exposed the floor he stands on must be (picture.ts exposedAt), how often it is asked (s) */
  dashM: 2.5, dashSeen: 0.5, dashEvery: 0.25,
  /** … and how lately a shot, a charge or a boot must have reached his ears (s) */
  dashHot: 6,
  /** `doorBrisk` (ON in the page — `?doorbrisk=0` — off here and on the frozen harness stand, `TPS_DOORBRISK=1`; a FROZEN_HOLDS row) = NOBODY SPRINTS TO
   *  A STACK: a man on a STACK order (a place in the stack at a door, the plain order's or a door tool's) who would run to cover on his way there
   *  (teamDash) goes at the fight's pace (PACE.max) instead — under the run's threshold, so his gun stays up and fires. The maintainer, of a fight on the pass: "weird really fast squad movement / sprinting that didn't make sense";
   *  asked where, "in a fight running to stack up on a door": three men crossing two to six metres at 4.7 m/s with their guns low, to stand at a door. A
   *  man sent to a spot on the floor still runs for cover, and so does one sent off a charge or to cover an opening. Off = the leg as it was */
  doorBrisk: false,
  /** `pacing` (ON in the page — `?pacing=0` — off here and on the frozen harness stand, `TPS_PACING=1`; a FROZEN_HOLDS row) = A TEAMMATE PACES HIMSELF ON
   *  THE PLAYER, and the flat `k` is gone from his walk. The maintainer, of a pair sent to a wall that "walk[ed] very slowly to it": "maybe we do
   *  something where if you're walking with the squad but behind them, they slow down to let you catch up (and match your speed when you're ahead of
   *  them?)" — "and then we remove the static 0.8 multiplier". So a man walking to the mark an order gave him goes at the PLAYER'S walk; at `k` × it only
   *  while the player is within `paceNear` of him, `paceGap` or more further from that mark than he is, and walking after HIM (moving, within `paceCos`
   *  of the line to him) — `k` of the player's own pace then, so a creeping player gains on him too; and at the player's own pace, no more than the
   *  fight's (PACE.max: never a run; an order asks ORDERS.brisk at most), while the player is that much nearer the mark than he is and bound for it faster
   *  than a walk. A man falling in keeps teamFollow's paces; the silent squad's and the unseen squad's own caps still stand over it. The player's pace is
   *  the one his legs are asked for (char.vel): pressed against a wall he still counts as moving. Off = the walk as it was (teamWalkOf answers teamWalk) */
  pacing: false, paceNear: 12, paceGap: 1, paceCos: 0.5,
  /** `walkAim` (ON in the page — `?walkaim=0` — off here and on the frozen harness stand, `TPS_WALKAIM=1`; a FROZEN_HOLDS row) = ON THE WALK TO A MARK A
   *  MAN LOOKS WHERE HE WALKS UNTIL HE CAN SEE WHAT HE WAS SENT TO WATCH. An order that sends a man to a mark hands him its watch point (the door he is
   *  to stack on) from his first step, and its walking carriage — the gun up and leading, orders.ts tick — carried the gun on that point through whatever
   *  stood between: walking along a wall with the door beyond it, his muzzle was 0.2 to 1 m off the plaster for a third to a half of the walk (and his
   *  weapon light, rightly, dark for it: guards.ts LIGHTS.wallOff). Now, in the walking carriage and with no line from his gun to the order's watch (to
   *  short of it: a door's own leaf stands in its frame), the tick hands his gun a point straight ahead of his chest instead, and he walks his path
   *  facing it (no side-step with his chest to a point he cannot see); once he has the line the watch is his again and the gun comes round to it at the
   *  ordered gun's own rate (guards.ts PACE.turn). ONLY the order's own watch in the walking carriage (Assignment.move: a stack, a walk to a door, a
   *  cover mark, a lane to cover, a stand-off) is asked: a point the unasked cover dealt him (cover.ts, dealt on a line of sight), spotted.ts's, the
   *  fight's (GuardScript.over: his own eyes) and the legs of an entry or a slice — which have no walking carriage: their point is the corner the
   *  muzzle leads him round to — are as they were, and so is every drill (squad.ts is not touched). doorpath.ts hands the same kind of point, the way he
   *  goes, for a gun that would cross an open leaf. `walkAimOff` / `walkAimOn`: how long the line must have been shut before the watch is given up, and clear before it is
   *  his again (s). Off = the gun on the order's watch from the first step, as it was (walkLead answers null) */
  walkAim: false, walkAimOff: 0.25, walkAimOn: 0.5,
  /** m/s from which a teammate is running: the gun goes low, nothing is fired, the boots are a sprinter's (character.ts BLEND.jogAt) */
  runAt: 2.7,
  /** m/s² a man with leave to run picks pace up at (guards.ts PACE.accel is 2.6: a walker leaning into a walk) */
  accel: 6,
  /** a sprinting teammate's footfall (the player's own sprint is 1) */
  step: 1,
  /** two runs of one man this close together are one (s) */
  gap: 1,
};
export type RunWhy = 'follow' | 'blast' | 'cover' | 'told';
/** runs taken, by reason, and the seconds spent at them (read by the checks and the tallies; nothing in the game reads it) */
export const TEAMPACE_STATS = { follow: 0, blast: 0, cover: 0, told: 0, secs: 0 };
export const TEAMPACE_DEBUG: { log?: (s: string) => void } = {};
const DOORBRISK_BREAK = (() => { try { return String((globalThis as { process?: { env?: Record<string, string | undefined> } }).process?.env?.DOORBRISK_BREAK ?? ''); } catch { return ''; } })();   // (QA, TEAMPACE.doorBrisk's own check: '1' = the rule's body out, its stack leg must fail; '2' = the rule taken for every order, its move leg must fail)
const BREAK = (() => { try { return String((globalThis as { process?: { env?: Record<string, string | undefined> } }).process?.env?.TEAMPACE_BREAK ?? '') === '1'; } catch { return false; } })();

/** what a man has leave for — 'close' a follower closing on his place, 'fight' a move the fight asks (neither is a run) — the most his legs may be asked
 *  for while it lasts, and when it ends */
type Leave = { why: RunWhy | 'close' | 'fight'; top: number; until: number };
const leave = new SideMap<Guard, Leave>('teampace.leave');
/** when he last ran under a reason, and which (the tally's own: a run is written down once) */
const ran = new SideMap<Guard, { t: number; why: RunWhy }>('teampace.ran');
/** a run to cover as last asked: when, for which mark, the answer */
const dash = new SideMap<Guard, { t: number; x: number; z: number; yes: boolean }>('teampace.dash');
/** the last moment a man had a reason to run, whatever his legs were doing (the checks' question: was that speed a run for a reason) */
const reasonT = new SideMap<Guard, number>('teampace.reasonT');

/** this teammate's pace is the rule's this frame */
export function teamSlow(g: Game, gd: Guard): boolean { return TEAMPACE.on && !BREAK && teamPaced(g, gd); }
/** the watch as last asked of a man: when, where it was, the answer, and since when the line has said otherwise */
const aimSeen = new SideMap<Guard, { t: number; x: number; z: number; blind: boolean; since: number }>('teampace.aimSeen');
/** a teammate's walk, m/s */
export function teamWalk(g: Game): number { return goneWithTick('teamWalk') as any; }
/** THIS teammate's walk this frame, m/s (TEAMPACE.pacing: paced on the player — the header of the knob has the rule; off: teamWalk) */
export function teamWalkOf(g: Game, gd: Guard): number { return goneWithTick('teamWalkOf') as any; }

function live(g: Game, gd: Guard): Leave | null { const L = leave.get(gd); return L && g.time <= L.until ? L : null; }
/** the fight has his legs: a man in his sights, or one lost `fightSecs` ago (not a man only turning to where he heard something: that is a walk still) */
function fighting(g: Game, gd: Guard): boolean { const O = g.fight.opOf(gd); return !!O && (!!O.tgt || g.time - O.lostAt < TEAMPACE.fightSecs); }
/** he may run from the grenade he is leaving (frag.ts): not with `own` off, not a hurt man — who leave it at the fight's pace */
function blastRun(g: Game, gd: Guard): boolean { return TEAMPACE.own && !isHurt(g, gd); }

/** the most a teammate's legs may be asked for this frame (guards.ts teamPace, for a man teamPaced has): PACE.max with the switch off. Asking changes nothing */
export function teamCap(g: Game, gd: Guard): number { return goneWithTick('teamCap') as any; }
/** he is at the team's walk: no leave for the fight's pace (an entry, a step of the room fight's) or for a run this frame, nobody in his sights, no grenade
 *  he is running from (guards.ts goTo asks for orders.ts ORDERS.propRound: the way round a chair is for a man walking, never for one going into a room or
 *  running from a blast — frag.ts judges a flee's leg by the plain way). A follower closing up on his place ('close') is walking still. False with the
 *  switch off */
export function teamAtWalk(g: Game, gd: Guard): boolean { if (!teamSlow(g, gd) || fragRunning(g, gd) || fighting(g, gd)) return false; const L = live(g, gd); return !L || L.why === 'close'; }
/** seconds since he last had a reason to run (Infinity: never, or the rule is off) — a check that holds the team to its pace asks it of a man going faster */
export function teamRunAgo(g: Game, gd: Guard): number { const t = reasonT.get(gd); return t === undefined || t > g.time ? Infinity : g.time - t; }
/** he is at a run this frame: gun low, nothing fired, a sprinter's boots */
export function teamSprinting(g: Game, gd: Guard): boolean { return goneWithTick('teamSprinting') as any; }
/** A TEAMMATE'S FOOTFALL, the level one step is heard at (guards.ts, squad.ts, pressure.ts — `base`: the caller's own walking level): a run's 0.8 over the
 *  walk-or-run line his card draws, the silent squad's step or `base` under it, × what his card rattles — and at a run under the rule, a sprinter's */
export function teamTread(g: Game, gd: Guard, speed: number, base: number): number { return crossToRust('teamTread', undefined, [g, gd, speed, base]) as any; }

/** a move the fight asks of him — the leg through an ordered door, a step the room fight orders, the breach drill — keeps the pace it had, for `secs` more
 *  than this frame */
export function teamFight(g: Game, gd: Guard, secs = 0): void { return crossToRust('teamFight', undefined, [g, gd, secs]) as any; }
/** A RUN TO COVER, the question itself: loud has reached him in the last `dashHot` seconds, he has nobody in his sights (a man who has fights), the floor he
 *  is on is in a known man's line, his mark `to` (`d` off, in plain sight on the same floor; `least`: the shortest leg worth it) is out of it. `running` =
 *  asked of a man already on the leg: what STARTED him — the loud, the line on the floor under him — is not asked again (a leg begun is run out: the floor
 *  he crosses leaves the line before he is behind anything); what would stop any man still is — a man in his sights, a mark the line has found, a wall */
export function teamDashNow(g: Game, gd: Guard, to: Vec3, d: number, least = TEAMPACE.dashM, running = false): boolean { return crossToRust('teamDashNow', undefined, [g, gd, to, d, least, running]) as any; }
/** … and the pace to run the leg at, or null (orders.ts, of a man walking to the mark an order gave him): asked every `dashEvery` seconds a man and whenever
 *  his mark has moved, kept between; once running he runs the leg out. `stack` = his mark is a place in a stack at a door: with `doorBrisk` the leg is the
 *  fight's pace, under the fight's leave, and no run at all */
export function teamDash(g: Game, gd: Guard, to: Vec3, d: number, stack = false): number | null { return crossToRust('teamDash', undefined, [g, gd, to, d, stack]) as any; }
/** A RUN THE MISSION TELLS HIM TO MAKE (orders.ts, of a man walking to the mark an order gave him; rails.ts runs: a tutorial step's `onWay.run` — the way
 *  back to the trucks once the job is done): the pace to run the leg at, or null. One more reason, under the same leave as the others: `own` off, a quiet
 *  man and a hurt one still walk — and, as on a run to cover, a man with somebody in his sights does not turn his back and run with his gun low: he
 *  fights at the fight's pace until it is over. The caller asks only outside the last metres of the leg (orders.ts ORDERS.briskIn): they are walked in */
export function teamTold(g: Game, gd: Guard): number | null { return crossToRust('teamTold', undefined, [g, gd]) as any; }
/** the pace a teammate leaves a grenade at (frag.ts, squadthrow.ts — what teamCap gives a man fragRunning): the flee's own with the rule on and a run
 *  allowed him, the old cap without */
export function teamBlastPace(g: Game, gd: Guard, run: number): number {
  return gd.team === 'operative' && PACE.on ? (TEAMPACE.on && !BREAK && blastRun(g, gd) ? run : Math.min(run, PACE.max)) : run;
}
