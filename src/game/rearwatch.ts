// THE REAR MAN COVERS THE DOOR BEHIND (REARWATCH.on — ships on; `?norearwatch` for a visit, TPS_REARWATCH in the harness's environment).
// A stack on a door stood with every gun on that door from the moment it was let go, and the rear man's spare eyes before that were whatever the unasked
// cover (cover.ts) happened to deal him that quarter second: a door standing open at the stack's back dropped out of the deal whenever a mate was near its
// frame or in his lane, and from the boot, the can or the shove on nobody watched it at all. Traced on the warehouse with four (seed 9, the older stand): the
// squad stacked on one door of a small office with the office's other door open behind it, 'open and clear' two seconds later, and a man was shot through
// the door behind while the four of them looked at the one in front.
//
//   THE WAY BEHIND  a DOOR other than the one being worked, standing `ajar` open or off its hinges, inside REARWATCH.reach of the nearest of the stack's marks, with a face on
//                   the floor the stack stands on (stackroom.ts's floor of that face of the door). Doors onto floor the team has not been on (cleared.ts; open
//                   ground is never 'been on') come first, then the nearest. One somebody else already holds — a posted man of another order, another man's
//                   unasked cover, the stack's own lane man (doorway.ts) — is not counted. (A gap with no leaf, the slot between two shelves, a passage's
//                   mouth: those stay the unasked cover's to deal, as before — the nav grid's gaps are mostly furniture, not ways onto the floor.)
//   THE REAR MAN    of the men whose eyes the order leaves spare (orders.ts spareRear: the furthest from the frame; the last two of four), the one with a line to
//                   that way from his MARK (into the doorway, to its open leaf's edge, or to the floor just before it: rearSees). He is PINNED to it: cover.ts deals it to him ahead of everything else and past the filters that let it drop
//                   (a mate by its frame, a mate brushing his lane, the walk's own narrower list). A fight or a warm contact still takes him, as it takes any
//                   man off any opening (cover.ts free()). With no way behind, or no line to one from any rear mark, nothing is pinned: the deal as it was.
//   THROUGH THE ENTRY  (REARWATCH.through; 'open and clear', 'bang and clear', the charge; three men or more) he keeps it through the boot, the can and the
//                   shove, and FOLLOWS LAST: doortools.ts puts him at the end of the turn, and he goes on that turn — the entry is not a gun short and not
//                   a second slower for him. The first guns through the door are the ones they always were; the entry's own rule (DOORTOOLS.lastOut) then
//                   gives the door they came in by to the last man through, from the inside — him.
//                   (REARWATCH.hold, off: he is kept on it until every other man of the entry is THROUGH the frame, REARWATCH.holdSecs at most. Built first
//                   and measured: the way behind was watched a second or two longer and the room was a gun short for it — over 18 seeds of the barracks
//                   with four, 31 teammates up against 41 with the rule off, 42 with him going on his turn. So he goes on his turn.)
//                   Where the room has no point for him (a small room, four men) he is the one the entry leaves outside, and the way behind stays his
//                   from there. (The door tool files the stack again behind the man who opens: where that file leaves no rear mark a line to the way, nobody
//                   is pinned through the entry — a mark is the stack's to move, not this rule's.)
//                   He is not the man picked to go FIRST either, while the other file has a front man (doortools.ts firstIn: 'who has seen most of the
//                   room' is asked of the others).
//   A PAIR          has no spare man once the door goes: its second man watches the way behind while the stack WAITS (as above), and both go in together.
//
// Nothing here moves a mark (stackroom.ts's), deals a pod, or decides a shot: a pinned gun is an aim, and who fires is the fight's (a team that has not been
// found is held by the same rules as on any other watch). State: a book per Game, DERIVED AGAIN EVERY FRAME from the orders and the doors' state — the one
// things remembered across frames are who had the watch (so it does not hop between two men) and when he last had his line (REARWATCH.grace: the worked
// door's own leaf swings across it), and losing them changes nothing but that. Nothing to save: a load starts the book afresh.
//
// A TAKEN ROOM'S OTHER DOOR, KEPT WHILE THE SQUAD LEAVES (REARWATCH.room; `?noroomkeep`, frozen stand off: TPS_ROOMKEEP=1; off = the build as it was).
// An entry that has ended leaves a man with his gun on each other door of the room nobody of the squad has been beyond (doortools.ts otherDoors, the
// threshold's holdDoors) — and lets go of him the frame ANY new order comes. 'Fall in' is such an order, given with the squad still standing in the room:
// every gun left the room's other doors at once. With the switch on the man who HAD such a door when 'fall in' came keeps it (roomKeepTick): the cover deals
// it to him first, as a stack's rear man's pin; his feet stay where they are, with no goal (orders.ts asks roomKept — a runner held at speed 0 on a goal
// still steers his chest at his next waypoint, and the twist clamp then drags his gun off the door and off a man he has seen); he is the last of the file.
// He is let go when the squad is out of the room, when the last man has passed him (Sam is out and the men who stood behind him then are all nearer Sam now),
// REARWATCH.roomSecs after Sam left whatever the others do, once Sam is REARWATCH.roomM from him, when the squad has been beyond that door or another order works or watches it — and then
// follows, last. An order that sends him to a mark ends the keep; if it is a stack and the kept door is still shut, that door counts as a way behind that
// stack while the squad is in the room (Book.shut, read by rearWays), so a rear man of the stack gets it by the rules above. Sam standing in the room keeps
// him on it. Nothing here deals a door: who has one is the entry's own rule, read a frame late. Not saved (a load: the entry's own hold is gone too).
// (src/game is one import cycle: nothing below is built from another game module's exports at module load.)
import { viewOf } from './seats';
import { SideMap } from '../math/side';
import { Vec3, v3 } from '../math/vec';
import type { Game, Guard } from './game';
import type { Door } from './doors';
import type { DoorOp } from './doortools';
import type { Opening } from './cover';
import type { Assignment } from './orders';
import { sideOf } from './orders';
import { stackRoomProbe } from './stackroom';
import { RUST_PASS, RUST_SURFACE, crossToRust } from './crossing';

export const REARWATCH = {
  /** THE switch: off = the rear man's eyes as the unasked cover alone dealt them, and every gun on the door from the moment it goes */
  on: true,
  /** … kept from the boot / the can / the shove until the others are through, and he follows last (false: pinned only while the stack waits) */
  through: true,
  /** … and HELD on it until every other man of the entry is through the frame (false, as shipped: he goes on his turn, the last of the queue — holding him
   *  left the room a gun short for a second or two and cost men in the paired fights) */
  hold: false,
  /** hold: the longest he is held back after 'go' whatever the others are doing (a man stuck in the frame does not strand him outside) */
  holdSecs: 6,
  /** how far from the stack's marks a way behind is looked for (cover.ts's own reach is the line's limit) */
  reach: 9,
  /** a leaf this far open is an open way (doorway.ts laneAjar's number) */
  ajar: 0.6,
  /** a line lost for less than this keeps the watch his (the worked door's own leaf swinging across it, a man walking through it) */
  grace: 1.5,
  /** a rear man this long within a stride of his mark and still not on it is let off the watch until he is: walking with his gun behind him he can grind on a
   *  leaf's edge where a man facing his walk slides past — and a stack waits for its last man (s) */
  short: 1.5,
  /** A TAKEN ROOM'S OTHER DOOR IS KEPT WHILE THE SQUAD LEAVES (the head comment's last part): off = the man an entry left on it turns away the frame 'fall in' is given */
  room: true,
  /** … the longest he is kept on it once Sam is out of the room (s): a mate who never leaves does not strand him */
  roomSecs: 8,
  /** … and never further than this from Sam (m): a room big enough for Sam to walk that far inside it is not one he is kept at the far end of — 0 = no limit */
  roomM: 8,
  /** 'the last man has passed him' also waits for every mate who is on his way OUT of the room (his place under 'fall in' is not in it) to be out: freed with
   *  the others a metre off, queueing at the one doorway, the keeper was through the frame ahead of one of them as often as not — he follows LAST. A mate whose
   *  place is in the room is not waited for, and `roomSecs` / `roomM` still let him go. false = freed the moment the men who stood behind him have come by, as it was */
  roomLast: true,
};
/** QA, since the page loaded: frames-with-a-group that had a way behind · … that had it pinned · … with a way and no rear mark that sees it · pins handed out
 *  (a new man or a new way) · entries the rear man was held back in · … let go on the clock instead of the others' crossing · turns he was moved to the end of ·
 *  taken rooms' other doors kept over a 'fall in' · … let go on the clock */
export const REARWATCH_STATS = { ways: 0, pinned: 0, noLine: 0, pins: 0, held: 0, late: 0, last: 0, rooms: 0, roomLate: 0, roomWays: 0, roomFar: 0 };

/** one pinned watch: the way, the door being worked and the face the stack is on, the door tool's order it is under (null: a plain 'stack') */
export interface RearPin { o: Opening; front: Vec3; door: Door; side: 1 | -1; op: DoorOp | null; cls: 0 | 1; t0: number; seenT: number; A: Assignment; going?: boolean; raised?: boolean; held?: boolean; late?: boolean; out?: boolean;
  /** a room's other door kept over 'fall in' (REARWATCH.room): the point he had his gun on when the order came — his when no other line is found */
  to?: Vec3 }
/** one kept door (REARWATCH.room): the pin the cover reads, the room it is a door of, when Sam was first seen out of it (NaN: still in) */
interface RoomKeep { pin: RearPin; room: object; outT: number; /** the men who had further to walk to Sam than he had when Sam left the room: the ones who have to pass him */ behind: Guard[] | null; /** when the leaving is next judged */ at: number }
interface Book { pins: Map<Guard, RearPin>; byOp: WeakMap<DoorOp, Guard>; short: WeakMap<Assignment, number>; letOff: WeakSet<Assignment>; log: string[]; rows: number;
  keeps: Map<Guard, RoomKeep>; was: [Guard, Door, Assignment][];
  /** a taken room's other door that was being kept when an order to stack came: SHUT, it still counts as a way behind that stack (rearWays) while the squad is in the room */
  shut: Map<Door, { o: Opening; room: object; at: Vec3 }> }
const books = new SideMap<Game, Book>('rearwatch.books');

const flat = v3.distXZ, EYE = 1.55, CHEST = 1.1;   // (cover.ts's own heights)
const ENTRIES = ['openClear', 'bangClear', 'charge'];
/** once the door is going a new man takes the watch only in these · a stack counts as on its marks with every man within this of his (m) */
const SLOW = ['toss', 'fuse', 'wait'];
const isOpenWay = (d: Door) => d.off || (!d.latched && Math.abs(d.angle) >= REARWATCH.ajar);

/** The ways behind a stack on face `side` of `door` whose marks are `marks`, best first (uncleared floor beyond, then the nearest) — whoever holds them. */
export function rearWays(g: Game, door: Door, side: 1 | -1, marks: Vec3[]): { o: Opening; front: Vec3; cls: 0 | 1; d: number }[] { { const __s = RUST_SURFACE; if (__s) { const __r = __s.call('rearWays', undefined, [g, door, side, marks]); if (__r !== RUST_PASS) return __r as any; } }
  if (!marks.length) return [];
  const K = REARWATCH, C = g.cleared, F = stackRoomProbe(g, door, side), c: Vec3 = [0, 0, 0];
  for (const m of marks) { c[0] += m[0] / marks.length; c[2] += m[2] / marks.length; }
  const out: { o: Opening; front: Vec3; cls: 0 | 1; d: number }[] = [];
  for (const o of g.cover.ready()) {
    if (!o.door || o.door === door || !(isOpenWay(o.door) || (K.room && books.get(g)?.shut.has(o.door)))) continue;   // (… or a taken room's other door a man was keeping when this stack was ordered: roomKeepTick)
    const d = Math.min(...marks.map(m => flat(o.at, m))); if (d > K.reach) continue;   // (within reach of the nearest mark: the same answer whichever of the file are still on the stack)
    const a: Vec3 = [o.at[0] + o.n[0] * 0.7, 0, o.at[2] + o.n[1] * 0.7], b: Vec3 = [o.at[0] - o.n[0] * 0.7, 0, o.at[2] - o.n[1] * 0.7];
    if (F.floor && !F.has(a) && !F.has(b)) continue;   // (a way onto some other floor: not at THIS stack's back)
    const far = F.floor && F.has(a) !== F.has(b) ? (F.has(a) ? b : a) : flat(a, c) >= flat(b, c) ? a : b, r = C.roomBy(o.door, sideOf(o.door, far));   // (what is beyond it: the face that is not this floor's)
    const near = far === a ? b : a;
    out.push({ o, front: [near[0], CHEST, near[2]], cls: r && C.isKnown(r) ? 1 : 0, d });
  }
  return out.sort((x, y) => x.cls - y.cls || x.d - y.d || (x.o.id < y.o.id ? -1 : 1));
}


/** Game.update, after the door tools' tick (which says whose eyes are spare) and before the cover's (which deals). */
export function rearWatchTick(g: Game): void { return crossToRust('rearWatchTick', undefined, [g]) as any; }

/** orders.ts, the fall-in leg: this man's feet stay where they are — he keeps a taken room's other door while the squad leaves it (REARWATCH.room) */
export function roomKept(g: Game, gd: Guard): boolean { return REARWATCH.room && !!books.get(viewOf(g, gd))?.keeps.has(gd); }   // (his seat's book: seats.ts)



/** The point of a way behind a man stood at `p` can watch: the doorway itself or its open leaf's free edge, as cover.ts sees any opening (sightFrom) — or, the
 *  frame out of his line (a door in the wall he stands along, down the corridor: the line to its frame grazes that wall), the floor just BEFORE it on the
 *  stack's side (`front`), where whoever comes through it steps out. null: none of them. */
export function rearSees(g: Game, p: Vec3, o: Opening, front: Vec3): Vec3 | null { { const __s = RUST_SURFACE; if (__s) { const __r = __s.call('rearSees', undefined, [g, p, o, front]); if (__r !== RUST_PASS) return __r as any; } }
  return g.cover.sightFrom(p, o) ?? (flat(p, front) <= REARWATCH.reach && flat(p, front) > 0.9 && !g.col.segmentBlocked([p[0], EYE, p[2]], front) ? front : null);
}
/** cover.ts deal(): the way this man is pinned to (null: none — he is dealt as anybody) */
export function rearPinOf(g: Game, gd: Guard): RearPin | null { const B = books.get(g); return !B ? null : (REARWATCH.on ? B.pins.get(gd) : undefined) ?? (REARWATCH.room ? B.keeps.get(gd)?.pin : undefined) ?? null; }
/** … whatever the switch says (QA, the debug read-out) */
export function rearPin(g: Game, gd: Guard): RearPin | null { return books.get(g)?.pins.get(gd) ?? null; }

/** doortools.ts enter(): the turn the men go in by, with the rear man — pinned on a way behind as the word comes — put LAST (never the man picked to go
 *  first; a pair's turn is left alone) */
export function rearLast(g: Game, op: DoorOp, turn: Guard[], first: Guard): Guard[] {
  const K = REARWATCH; if (!K.on || !K.through || turn.length < 3) return turn;
  const w = rearWatcher(g, op); if (!w || w === first || !turn.includes(w) || turn[turn.length - 1] === w) return turn;
  REARWATCH_STATS.last++;
  return [...turn.filter(m => m !== w), w];
}

/** doortools.ts firstIn(): the man of this order who is pinned on a way behind and will be held on it through the entry (null: none — a pair, the switch off) */
export function rearWatcher(g: Game, op: DoorOp): Guard | null {
  const K = REARWATCH; if (!K.on || !K.through || op.thr || !ENTRIES.includes(op.kind) || op.men.filter(m => m.state !== 'dead').length < 3) return null;
  const B = books.get(g), w = B?.byOp.get(op); return w && B!.pins.get(w)?.op === op ? w : null;
}

export function rearWatchLog(g: Game): string[] { return books.get(g)?.log ?? []; }
/** how many rows that log has ever been handed (it keeps the newest 200: a reader that prints new rows counts on this, not on its length) */
export function rearWatchRows(g: Game): number { return books.get(g)?.rows ?? 0; }
