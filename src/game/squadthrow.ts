// TEAMMATES' FRAGS AND STUN CANISTERS, THROWN ON ORDER — "frag there" / "bang there".
//
// The maintainer (10-13): "would be nice to be able to instruct teammates to throw frags/stuns". Until here a teammate's grenade left his hand only as part
// of a door order (bang and clear, frag and clear). Here the squad has two counted orders on a spot of FLOOR, behind one row of the floor's menu:
//
//   X on a spot of floor · G ('throw ▸', a page) · 1 = frag there · 2 = bang there.      Nothing held, no digit above 2 on the page, Esc / right click = back.
//
// THE COUNT is the door tools' own (doortools.ts `frags` / `bangs`: one bag, whoever's hand it leaves) — what the squad block already shows. No kit of its own.
// WHO THROWS: of the men the menu goes to, the nearest with a throw to the spot — squadsmoke.ts's own test (`squadArcTo`: the hand, the arc solved to bring the
// thing to REST on the spot, down on the floor, its run clear) and its own exclusions (`squadHandFree`) + `handsAtWork` (the man setting a charge, on a door's
// handle or boot, with a hand on a thing he works, or changing a magazine is passed over), a stun canister with the smoke can's run (one body:
// throwables.ts) and a frag with its own (SQUADTHROW.fragRollK, measured). The rows are `free` and `only` the thrower's: nobody else answers.
// HE WORKS HIS THROW OUT (SQUADTHROW.lob / duck / samCall; the maintainer, 10-15: "ordering your squad to throw a grenade is still pretty useless. they should
// figure out how to throw to a spot"). Measured on the office, the four holding by the player: a frag was refused on 88 % of the floor he could see ('you are
// in it.': a frag reaches 9 m) and had no throw to two spots in three of the rest. Three rules, each its own key under the switch: `lob` — where the straight
// arc has no answer a man flies a few throws in his head (rust throwplan: the pool's own flight) and takes one that goes off by the spot — another pace, a
// bank off a wall beside it — from where he stands, else from floor he walks to; `duck` — a man of ours who will be off a frag's line before it bursts (the
// squad steps off its own frag: frag.ts fragFlee) is not a reason to refuse it, nor to pass over floor to throw from; `samCall` — a frag that reaches the
// player alone is his call, named on the row. The order of asking: the straight arc from where a man stands, a worked-out throw from where a man stands,
// a walk to the straight arc, a walk to a worked-out throw.
// A MAN WITH NO THROW WALKS TO ONE (SQUADTHROW.walk; the maintainer, 10-13: "it would be good to have the squad move to do a throw (and maybe visualize who's
// doing it and the path on the ground?)"). Where nobody addressed has a throw from where he stands, `walkSpot` floods the walk grid out from each man who may
// be walked off (hand free; no drill, door tool or wall charge has him; not in a fight; under no order or one he can be handed back: move / stack / cover /
// hold / fall in) — over floor the squad knows, never through a shut leaf, at most `walkM` — and takes the FIRST cell (the fewest steps: the edge of where a
// throw opens, not the middle of it) from which the same arc test passes for a man standing there, he himself outside what it does, and the orders' router
// has a way. He is walked there by `SquadOrders.assign` under the row's own id (so anything that draws an order's man, way and mark draws this), the can
// still in the bag; on the spot he turns to it, the sense above is asked again of the floor as it stands, the can is taken and thrown, and (`back`) he walks
// back to where he stood and to the order he had. Stopped short, he looks once more for a spot from where he is; nothing = his word and back. Any other order
// to him ends it (nothing thrown, nothing spent). Off = 'no throw from here.', as it was.
// THE PICTURE (`view.throws`, drawn by ui/planfloor.ts and ui/squadthrowviz.ts on the overlay): who throws (a ring at his feet in his element's
// colour), his walk, the arc, where it rests — for the highlighted row while the throw page is up, and from the order until the thing leaves his hand.
// THE THROW is the door tools' own: the frag on FRAG.orderFuse from the hand, the canister on combat.ts flashFuse from its landing, the quick toss, his word
// ('frag out!' / 'bang out!'). Each goes off by the game's own road (combat.ts updateItems → frag.ts / detonateStun) under the thrower's id: nothing borrowed.
// SENSE, asked when the menu is read and again as the order is given (nothing is spent on a refusal):
//   none left · no arc for anybody addressed · A FRAG NEVER ONTO ITS OWN: the player, any teammate still breathing (the thrower too; a man out cold counts), a hostage, the man to take
//   alive or a civilian inside the reach the door tools ask of a frag (FRAG.woundM + 1, a line by frag.ts exposedTo) of where it would rest or of the spot
//   itself = refused, naming who. A STUN is not refused for its own side — it is the player's call — but its row NAMES who of ours it would catch
//   (' · too close: you, Red One'), by detonateStun's own tests asked from where it would rest and from the spot, each reach a metre long: it may over-warn.
// A SILENT squad: both rows are silent.ts LOUD_ROWS (marked, and giving one takes the squad out of silent).
// THE SQUAD BLOCK shows the throw for SQUADTHROW.shownSecs ('Red Two · frag out').
//
// SQUADTHROW.on: off in the module and on the frozen stand; on in the page (host/profile.ts). The rows are put on / taken off the menu by squadThrowKey, once
// a frame, so a switch thrown live holds from the next frame. Off, and never on for this Game: one map read a frame. On with nothing ordered: rows on a menu.
//
// State is per Game in a WeakMap: whether the rows are on, each man's last throw (the toss has to finish), what the squad block is showing, the stun row's
// warning as last read. None of it is in a checkpoint — none is taken with a unit on the floor, and all of it is seconds old at most. No dice of its own:
// the thrown thing draws the game's die as any thrown item does (throwables.ts spawn). 'Nobody of ours in it' is asked AT THE THROW, of where each stands then.
// Check: tools/qa/squadthrow-check.ts (`probes.ts squadthrow`).
import { rollbackVar } from '../math/side';
import type { Vec3 } from '../math/vec';

import type { Game, Guard } from './game';

import type { Assignment, OrderTarget } from './orders';
import type { ThrowSolution } from './throwables';
import './character';
import { fleeMates } from './frag';

import './samstun';
import { workHand } from './workthings';

import './squad40';   // (called inside a row's label only)
import { crossToRust } from './crossing';

export const SQUADTHROW = {
  on: false,
  /** how far a frag runs on past its landing: this × its pace over the floor × the speed it comes down at (squadsmoke.ts squadSmokeRest; a frag is a smaller,
   *  deader body than a can — throwables.ts FRAG_ITEM; measured headless on the office's floor, throws of 3 m to 12 m: where the fuse found it was a mean
   *  0.11 m and at most 0.30 m from the spot at 0.04, 0.15 / 0.33 at 0.045, 0.28 / 0.69 at 0.05) */
  fragRollK: 0.04,
  /** one man's next throw no sooner than this after his last (s) */
  againSecs: 1.2,
  /** the squad block names the throw this long (s) */
  shownSecs: 2.5,
  /** the stun row's warning asks detonateStun's reaches this much long (m): where it goes off is where it has run to when its fuse ends */
  slackM: 1,
  /** A MAN WITH NO THROW FROM WHERE HE STANDS WALKS TO WHERE HE HAS ONE, throws, and goes back (off: nobody walks — 'no throw from here.', as it was) */
  walk: true,
  /** … no further than this over the walk grid (m) · never from nearer the spot than this (m) · nor from further than a hand throws (m) */
  walkM: 12, minM: 1.5, maxM: 11.5,
  /** cells whose arc is asked in one search at most (each up to five arcs, ~0.2 ms an arc) · a search's answer is kept this long for the menu's and the picture's re-reads (s; an order GIVEN asks afresh) */
  tries: 12, keepSecs: 0.4,
  /** he gives the walk up after this long on it (s; time the fight has him is not counted) · on his spot he turns to the throw for this long at most (s) · the toss: he stays this long after it leaves his hand (s) */
  walkSecs: 16, turnSecs: 1.2, tossSecs: 0.5,
  /** A MAN WORKS HIS THROW OUT (rust throwplan; on with the switch, off = the one straight arc that rests on the spot, as it was; `?throwlob=0`): where the
   *  straight arc has no answer he flies a few throws before he makes one — straight at the spot at `lobPaces` (m/s over the ground: the solver's, flat and
   *  hard, a slow lob) and at up to `lobWalls` walls that stand within `lobWallM` of it (met `lobWallH` up; `lobBankPaces`) — and takes the one that GOES OFF
   *  nearest the spot: within `lobNearM` of it, no more than `lobUpM` over its floor, nothing solid between. No throw leaves the hand faster than `lobSpeed`
   *  (the player's own arm). A throw is taken only if it still goes off by the spot stepped at each of `lobSteps` (the page steps at the display's rate; the forecast at a sixtieth). A search flies `lobSims` throws at most, over `lobCells` cells of floor at most, and its answer is kept `lobKeepSecs`. A walker who gets there is given `settleSecs` to come to a stand before his throw is asked for again (the hand of a man still walking is lower than the hand the search threw from). A man who would be inside his own frag where he stands is passed over for the next
   *  man, or a walk, instead of the row being refused in his name. */
  lob: true, lobPaces: [7.5, 10, 5] as readonly number[], lobBankPaces: [10, 7.5] as readonly number[], lobSpeed: 11, lobNearM: 1.5, lobUpM: 1.2, lobWalls: 12, lobWallM: 3, lobWallH: 0.9, lobSims: 48, lobCells: 10, lobKeepSecs: 1, settleSecs: 0.5, lobSteps: [1 / 120, 1 / 40] as readonly number[],
  /** A FRAG WHERE THE PLAYER HAS A LINE IS HIS CALL (on with `lob`; off = refused 'you are in it.', as it was; `?samcall=0`): the row is given all the
   *  same and NAMES him (' · too close: you'), as the stun row always has. The rings on the floor show the reach; 'frag out!' is the warning. Inside
   *  `duckMinM` of it (what it kills at) it is refused for him as before. His own men, hostages and civilians in it still refuse it. */
  samCall: true,
  /** A MAN WHO WILL BE OFF ITS LINE WHEN IT BURSTS IS NOT 'IN IT' (on with `lob` and with frag.ts FRAG.fleeWise, the flee it counts on; off = anybody of ours the burst has a line to refuses it, where he
   *  stands, as it was; `?throwduck=0`). The squad steps off its own frag the moment 'frag out!' is called (frag.ts fragFlee: every man in its line is dealt
   *  the spot his legs make before the fuse — at the squad's pace, never a sprint). The order now asks the same question BEFORE it throws: a teammate on
   *  his feet whose dealt spot the burst has no line to — with `duckSecs` of the fuse in hand, `duckThrowSecs` more for the man whose arm is still coming
   *  down — is not a reason to refuse, and a cell the thrower can duck from is a cell he may throw from. A man with nowhere to go still refuses it, by
   *  name — and so does one whose spot is within `duckApartM` of where another of ours stands or is going (two men to one corner leaves one out in it; with frag.ts FRAG.fleeApart on, the spot is asked for with the deal's own list of that floor, `fleeMates`, and of the man himself, as the deal asks; with it off, as before). Nobody within `duckMinM` of it (FRAG.lethalM and a metre) is asked to duck at all. A search asks it of `duckCells` cells at most. */
  duck: true, duckSecs: 0.5, duckThrowSecs: 0.2, duckCells: 16, duckApartM: 1.0, duckMinM: 5,
};
export const THROW_PAGE = 'throw', THROW_OPEN = 'throwThere', FRAG_THERE = 'fragThere', STUN_THERE = 'stunThere';
export type SquadThrowKind = 'frag' | 'stun';
type Arc = { from: Vec3; sol: ThrowSolution; rest: Vec3 };
/** a throw and the man to make it; `post`: the floor he walks to first (absent: he throws from where he stands) */
export type SquadThrower = Arc & { gd: Guard; post?: Vec3 };
/** A MAN TAKEN OFF WHAT HE WAS DOING, TO BE HANDED BACK (`squadLend` / `squadHandBack`: the walk to a throw here, the turn to a shot in squad40.ts): the men who
 *  shared his order, the target the order was given on, the assignment that has him meanwhile (`A`: another one in its place = re-ordered), where he stood
 *  and what he was doing (`back`: null = nothing he can be handed back) */
export interface Lent { mates: Guard[]; gd: Guard; t: OrderTarget; A: Assignment | null; home: Vec3; homeYaw: number; back: (Pick<Assignment, 'order' | 'post' | 'face' | 'going' | 'there'> & Partial<Assignment>) | null; }
/** a flight's count, the Rust game's throw search's */
export const THROWPLAN_STATS = { sims: 0 };
export const SQUADTHROW_STATS = { thrown: 0, frags: 0, stuns: 0, refused: 0, walks: 0, walked: 0, gaveUp: 0, searches: 0, arcs: 0, ducks: 0, cells: 0 };
/** every one thrown on the order, newest last (QA; capped) */
export const SQUADTHROW_LOG: { t: number; kind: SquadThrowKind; who: string; from: Vec3; at: Vec3; land: Vec3; rest: Vec3 }[] = [];

/** his hands are on something else: he is the man setting a charge on a door or a wall, the man on the handle / the boot of a door order, he has a hand on
 *  a thing he was sent to work, or he is changing a magazine — another man throws, or nobody does (squad40.ts asks it of the man with the launcher) */
export function handsAtWork(g: Game, gd: Guard): boolean {
  if (g.doorTools.ops.some(o => o.lead === gd && (o.phase === 'place' || o.phase === 'kick'))) return true;
  if (g.wallCharge.ops.some(o => o.lead === gd && o.phase === 'place')) return true;
  if (workHand(g, gd.char)) return true;
  const O = g.fight.opOf(gd); return gd.reloadT > 0 || (!!O && O.reloadUntil > g.time);
}

/** the man of `men` who would throw it, with his throw: the nearest to the spot with a hand free and an arc that brings the thing to rest on it (null: none) */
export function squadThrower(g: Game, men: Guard[], at: Vec3, kind: SquadThrowKind): { gd: Guard; from: Vec3; sol: ThrowSolution; rest: Vec3 } | null { return crossToRust('squadThrower', undefined, [g, men, at, kind]) as any; }

/** SQUADTHROW.duck: a man of ours on his feet at `p` is off this frag's line before it bursts — the spot his legs will be dealt when it is called (frag.ts
 *  fleeSpot, the squad's pace, `duckSecs` of the fuse in hand and `late` more) is one the burst has no line to him at, down behind it */
let duckMates: { g: Game; gd: Guard; t: number; x: number; y: number; z: number; m: ReturnType<typeof fleeMates> } | null = null;
rollbackVar('squadthrow.duckMates', () => duckMates, v => { duckMates = v; });   // (a memo keyed by the game's clock)

/** the orders he can be handed back when the throw is made */
const RESUMED = ['move', 'stack', 'cover', 'hold', 'fallIn'];
/** may this man be walked off to a throw: no drill owns him, no door tool or wall charge has him, the fight does not, and the order he is under (if any) is one he can be handed back
 *  (squad40.ts asks it of a man it would turn to a shot) */
export function mayWalk(g: Game, gd: Guard): boolean {
  const S = g.orders, A = S.of(gd);
  if (S.owners.some(o => o.owns(gd))) return false;
  if (g.doorTools.ops.some(o => o.men.includes(gd)) || g.wallCharge.ops.some(o => o.men.includes(gd))) return false;
  // (a room fight drives a teammate in 'patrol' with no yield stamp: the man it has a gun on is its `tgt`; a stack's or a stand-off's rear man keeps the squad's back)
  if (gd.state === 'alert' || (A && A.yieldT >= 0) || g.fight.opOf(gd)?.tgt || A?.spare) return false;
  return !A || RESUMED.includes(A.order);
}

/** THE THROW AS IT WOULD BE ORDERED NOW: the nearest man with a throw from where he stands, else (SQUADTHROW.walk) the man with the shortest walk to one, with the
 *  floor he throws from as `post` (null: none). The walk's search is kept SQUADTHROW.keepSecs for the menu's and the picture's re-reads; `fresh` asks again. */
export function squadThrowPlan(g: Game, men: Guard[], at: Vec3, kind: SquadThrowKind, fresh = false, view = false): SquadThrower | null { return crossToRust('squadThrowPlan', undefined, [g, men, at, kind, fresh, view]) as any; }

/** WHO OF OURS A FRAG RESTING THERE WOULD HAVE IN IT (null: nobody) — in the row's words. The door tools' own test of a body (doortools.ts fragBehind: inside
 *  FRAG.woundM + 1, a line by the burst's own exposure or a plain one), asked from where the arc says it rests AND from the spot asked for.
 *  (`moved`: the thrower asked where he WILL stand, not where he is; `sam` false: the player is not asked about — SQUADTHROW.samCall, his call outside `duckMinM`)
 *  `kept` (the menu's and the picture's re-reads, never the order's own ask nor the hand's): the verdict of the last 0.3 s for the same throw with everybody
 *  of ours where he was — the duck rule asks a flee of every man in it, and the rows are read every frame
 *  `burst` (squad40.ts: a round that goes off as it lands): nobody is counted on to duck — there is no fuse to duck in — the reach is the one handed in,
 *  and `no` its own words for 'no frag.' */
export function squadFragCatches(g: Game, rest: Vec3, at: Vec3, moved?: { gd: Guard; at: Vec3 }, sam = true, kept = false, burst?: { reach: number; no: string }): string | null { return crossToRust('squadFragCatches', undefined, [g, rest, at, moved, sam, kept, burst]) as any; }

/** WHO OF OURS A STUN CANISTER GOING OFF THERE WOULD CATCH: 'you' and callsigns (none: []). combat.ts detonateStun's own tests — the night game's line inside
 *  the dazzle radius, or the room fight's (a line from the can or from over it; its own side only on top of it, the player in full where he has a line:
 *  SAMSTUN.ownSeen) — asked from where it would rest and from the spot, every reach SQUADTHROW.slackM long. */
export function squadStunCatches(g: Game, rest: Vec3, at: Vec3): string[] { return crossToRust('squadStunCatches', undefined, [g, rest, at]) as any; }

