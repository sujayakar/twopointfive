// DOORWAY MANNERS: how a man on foot shares a doorway with his own side. The maintainer, 10-08: "doors could use some work too … AI does a better job not
// crowding". Measured on the build before this file (tools/qa/door-watch.ts over the loud levels): every man plans his walk as if he were alone (guards.ts goTo)
// and the only thing between two of one side is the push that parts their bodies at 0.6 m (collideGuard) — so a file told to fall in or to stack went through a
// frame two abreast and shoving, a follower stopped 'near enough' in the leaf's swing or on top of the man beside him, and a defender who got his line from a
// threshold planted on it and fought from there with a colleague behind him wanting through. Three rules, for the squad (team 'operative') and the defenders
// (team 'hostile') alike, all from where the men ARE and where they are walking — nothing scripted, no new order, nobody runs:
//   PASS  (doorwayGate, the first thing followPath / the scripted strafe ask): a man whose walk goes through a frame within `reach` looks at his own side about
//         it. A friend going his way ahead of him is followed at `gap` (pace eased from `gap` down to nothing at `gapMin`: one in the opening at a time, nobody
//         pushed through; a friend standing on his line by a door, his own way on past him, is walked ROUND — a corner put in his walk beside the man —
//         and one standing where his own place is, is stopped short of). A friend STANDING in the opening, standing just beyond it, or coming the other way and nearer the frame than he is, holds him short
//         of it (`throat`) — and held more than `aside` seconds he steps to a place off a jamb on his own face (`jambOut` off the wall, `jambPast` past the
//         jamb, a `fileGap` between places, clear of the leaf's swing and of everybody), turned to the door, and goes when it is free. A man IN an opening is
//         never held: he clears it. Nobody waits for ever: `patience` seconds held and he goes anyway (the old shove), with `rest` seconds before he can be
//         held again — under every caller's own stall clock, so a held man is never called stuck or planted 'going nowhere' for it.
//   STAND (doorwayStand, asked by orders.ts for a place in the file): no place in the file is in a frame's strip or where a leaf swings, shut or open, and a
//         follower who has stopped 'near enough' on top of the man ahead of him walks on to his own place. (The swing costs length: places slide back
//         along the trail past the arc, so by a door the file stands a metre or two longer behind Sam — `swing` off keeps the strip only. Stepping the
//         place sideways out of the arc instead was tried: its men were off the trail when Sam walked on and fell further behind.)
//   FIGHT (offThreshold, asked by the fight AI where it would plant and fire): planted in an open doorway `funnel` seconds, he steps off the opening — the
//         squad through it to the side its man is on, a defender back off it on his own side — to a spot that still has the shot, instead of fighting from the frame.
//   STAND, THE LANE (doorwayLane; the squad only): doors stand open now (doors.ts DOORSTAY), and a door standing open is a line of fire from whoever is
//         beyond it. So no place in the file is in the lane of one either (`lane`: stepped square off it), the walk an order gives a man to a mark goes
//         round such a lane where there is a way round that takes no other door (`laneWalk`, orders.ts route), and on the walk to a stack the rear man's gun
//         is on a door standing open by the way (`watch`, cover.ts). Measured on the two squad maps where the door build lost men:
//         rebuild/doors/first-door/notes.md in the project files.
// DOORMANNERS.on off = none of it runs: the build as it was, digest for digest (each rule has its own switch under it for an A/B). The night shift's staff (team
// 'guard') are left as they were — the stealth game's frozen runs hold whatever the switch says.
// (src/game is one import cycle: nothing here is built from another game module's exports at module load.)
import { SideMap } from '../math/side';
import { Vec3 } from '../math/vec';
import type { Game, Guard } from './game';
import type { Door } from './doors';
import { frameOf } from './guards';
import './glasspen';
import { crossToRust } from './crossing';

export const DOORMANNERS = {
  /** THE switch: off = the build before this file, by run digest */
  on: true,
  /** its three rules, each off = that rule as it was (the head comment) */
  stand: true, fight: true,
  /** metres of his walk ahead looked along for a frame; the strip either face of the frame line that is 'the opening'; how far short of the line a held man stops */
  reach: 3.0, half: 0.45, throat: 1.25,
  /** following a friend through: full pace at `gap` metres from him, none at `gapMin`; how hard he brakes for it (m/s²) */
  gap: 1.05, gapMin: 0.7, brake: 8,
  /** seconds held before he goes anyway (under every caller's own stall clock: the strafe's 1.2 s, the room fight's 1.5 s 'going nowhere', the orders' 4 s
   *  'stuck'), and before he can be held again; seconds held by a man standing just beyond the frame (he may be there to stay) */
  patience: 1.1, rest: 2.0, beyond: 0.6,
  /** seconds held before he steps off to a jamb; the place: metres off the wall, past the jamb, between places, from anybody else; the pace he steps at */
  aside: 0.4, jambOut: 0.6, jambPast: 0.5, fileGap: 0.9, apart: 0.85, step: 1.4,
  /** a friend standing on his line this near, his way on past him: he walks round, this far off the man's middle */
  round: 1.2, roundOut: 0.85,
  /** PASS, a friend ON HIS OWN MARK (his order's mark within `ownM` of his feet): he is a standing man whatever his feet are doing — settling onto it, or
   *  stepping back onto it after a nudge — so a man whose way goes on past him walks round him and is never walked into him; and the rule is also asked on
   *  the last `ownEndM` metres of a walk that ENDS by a door (a stack's second rank stands further than 2.5 m from the frame's middle: the man bound for the
   *  wall place beyond it shoved the rear man off his mark, onto his own, and the two then stood on each other's). Off = the rule as it was */
  ownPlace: true, ownM: 0.45, ownEndM: 2.5,
  /** AN OPEN LEAF ACROSS HIS WALK (the squad; doors.ts does it, game.ts hands it this row as `DoorContact.round`): a door pulled open toward a corridor stands
   *  across it, and a man whose walk goes through where the leaf stands herded it round to its frame with his body — onto the man ahead of him, who had to pull
   *  it again (the registry's copy room with four behind Sam: ten seconds on a door Sam had just opened, the leaf pressed back on the man re-opening it by the
   *  three behind). With this the back of a leaf standing open clear of its doorway is a wall to a teammate at a walk, and he goes along it and round its free
   *  end; where there is no way round he has it out of his way as before (`ROUND_SECS` pressed on it). Off = the leaf turns toward shut under him, as it was */
  leafRound: true,
  /** `leafStall` (ON in the page — `?leafstall=0` — off here and on the frozen harness stand, `TPS_LEAFSTALL=1`; a FROZEN_HOLDS row) = A LEAF NOBODY CAN GO
   *  ROUND IS TAKEN OUT OF THE WAY AT ONCE. A leaf standing open across a passage narrower than the leaf and a man: `leafRound` slid him to its free end,
   *  the wall there put him back, and his touch on the leaf came and went — so the leaf's own count of a press (doors.ts ROUND_SECS, unbroken) never ran
   *  out and he walked on the spot against it (the compound's door `c_n`, its leaf open across the strip a man came along: four seconds). A teammate walking
   *  who has not moved `leafStallM` in `leafStallSecs`, within `leafStallReach` of the back of a leaf standing open that lies ahead of him (`leafStallAhead`,
   *  the cosine between his walk and the way to it — a man held by a mate beside a leaf is not counted): for `leafStallPush` seconds he does not go round
   *  leaves — the leaf turns under him as it did before `leafRound`. Off = he waits for the leaf's own count */
  leafStall: false, leafStallM: 0.12, leafStallSecs: 0.6, leafStallReach: 0.6, leafStallAhead: 0.3, leafStallPush: 1.5,
  /** the swing: this far past the leaf's tip is still 'where it sweeps' */
  sweep: 0.2,
  /** STAND, the LANE of a door standing open (`laneAjar` rad or more, or off its hinges): the floor either face of its frame that a man beyond it looks — and
   *  shoots — straight down, out to `laneM` metres or the first wall; as wide as the opening and `laneOut` past each jamb, wider by `laneFlare` a metre of depth
   *  (what a man well back on the far side sees through it). No place in the file is in one, Sam stopped or walking: it is stepped square off it, `laneClear`
   *  past its edge, on the side its man is on (a place with no floor off the lane stays on the trail). Off = places only keep off the strip and the swing */
  lane: true, laneAjar: 0.6, laneM: 6, laneOut: 0.3, laneFlare: 0.12, laneClear: 0.15, laneApart: 0.7, laneLeaf: true,
  /** STAND, on the walk to the next door: a door standing open by the way (`laneAjar`), with floor beyond it the team has not been on, is the REAR man's to
   *  watch for the whole walk to a stack (cover.ts: the last man of the file, the last two of four — at a walk, his gun on it over his shoulder or walking
   *  backwards past it), not only for his last steps to the mark. Off = on the walk every gun is forward, as it was */
  watch: true,
  /** STAND, a stack in a lane: a door order whose marks stand in the lane of a door standing open onto floor the team has not been on (the stack is what
   *  that door looks at) — one man of it does not walk onto his mark with his back to that door (doortools.ts laneCover: the one furthest from the ordered
   *  door who is not the lead): he covers it first, stood on the line from the middle of the stack through the middle of its opening, `coverM` metres short
   *  of the frame, his gun — and a searcher's weapon light, where guards.ts LIGHTS gives him one — down that line to `coverDeep` metres beyond it (where a
   *  man with a line onto the stack has to stand), while the lead gets onto the door they were sent to. He comes up to his own mark once the lead is on his
   *  and nothing living has been in his eyes for `coverQuiet` s (having looked `coverLook` s at least; `coverCap` s on the place at most, and at once on a
   *  captor's last warning — the door is not kept waiting past that), and the stack waits for him like for any man still walking.
   *  Off = every man straight to his mark, as it was */
  stackCover: true, coverM: 2.0, coverDeep: 2.5, coverLook: 0.8, coverQuiet: 1.0, coverCap: 8,
  /** STAND, the walk to a mark an order gives: the short way does not run down the lane of a door standing open onto floor the team has not been on —
   *  its cells past `laneStrip` metres of the frame cost `laneDear` times plain floor, and where the way round is at most `laneLong` times the short way + 1 m
   *  (and a metre or more of lane the less) it is the way walked (orders.ts route) — never a way through a door the short way does not take. A man going THROUGH
   *  such a door is not kept out of its lane. Its own switch: it does not need `lane` on. Open ground (a flood too big to be a room: cleared.ts) never counts
   *  as floor the team has been on, so a door onto a yard or a big hall stays priced, and the rear man's to watch, for the whole run.
   *  Off = the short way, as it was */
  laneWalk: true, laneDear: 6, laneStrip: 0.7, laneLong: 1.6,
  /** FIGHT: seconds planted in an open doorway before he steps off it, the rest after a step, metres through and past the jamb he steps to */
  funnel: 0.6, funnelRest: 1.5, through: 0.95, past: 0.6,
  /** FIGHT, a defender's step and cover (off = the step as it was): no spot he steps to is more exposed to a rifle at his man (or his fix) than the frame he
   *  stands in — covered, reached only through glass (a pane is no wall: glasspen.ts, at `GLASSPEN.stackEnergy`), reached in clear air — so from a covered
   *  frame he steps only to covered floor or stays; with nobody in his sights a covered spot comes before any other free one; and with them within
   *  `planeDeg` of the door's own wall line (never less than half a metre of it: neither face is 'away from them') covered floor on EITHER face will do —
   *  the face the half-plane calls theirs only ever under cover */
  stepCover: true, planeDeg: 30,
  /** REST (orders.ts restClear): a fall-in follower is not left standing in a frame's strip or where a leaf swings — with Sam stopped, a place the file's
   *  half-metre slack has left in one is given up for his place on the trail (or the nearest clear floor), and a man who has stood `restSecs` in one short
   *  of his place steps to the nearest clear floor (within `clearM`) and waits there. MARKS (orders.ts stackPosts): the stack marks
   *  of the men who are not at a jamb are moved off strips and swings the same way; PODS (doortools.ts offSwing, `podClear`, ships OFF): so are the points men are
   *  left on after an entry — measured on the barracks it loses the entry (11 runs won of 12 → 4: a corner point behind an open leaf is cover). SHORT (orders.ts
   *  restShort): a man who cannot get to his mark does not give it up standing in a strip or a swing, and the
   *  seconds the PASS gate holds him (`heldCap` of them an order) are not counted toward 'stuck'. Each off = as it was */
  restSecs: 1.0, markClear: true, podClear: false, clearM: 1.4, heldCap: 12,
  /** OWN LEAF (orders.ts stackPosts): a door whose own leaf stands open across the face the stack is ordered on — every mark on its hinge side cut off from
   *  the doorway's front by it — is no longer refused 'no room on that door': where that was the answer the marks are asked again with that one leaf left
   *  out, which gives the marks the door has when shut. Off = the leaf is a wall to its own stack, as it was.
   *  OWN MARK (orders.ts ownMarkFirst, asked where a stack's marks and the door tools' tucked marks are handed out; and per place in stackPostsOn's jambs
   *  and files): a mark within a body's room of Sam is his and the files go on past it — except a mark a man of the very order being dealt already
   *  stands on, on this face of this door. Stacked men are dealt the marks they stand on whatever Sam has since come to stand beside and whichever maker
   *  dealt them, so a door tool given to a stack is not refused 'no room on that door' for Sam's own body, nor are the men shuffled along the wall for
   *  it; and the door tools' tuck does not move such a man onto Sam. A charge's stand-off is not touched (nobody stands on it yet: Sam's place there is
   *  his). Off = Sam's place is his whoever was there first, as it was.
   *  FURNITURE (orders.ts onFurniture, asked per place in stackPostsOn's jambs, files and second rank, the passage file and the corner door's places back): a
   *  place a piece of furniture stands on — a carton, a chair, a bin, a plant, where it is now — is the furniture's, and the file goes on past it as it
   *  does past Sam. One place in thirty was dealt onto a prop: on open floor the man shoves it aside, wedged in a corner he cannot, and stood short of
   *  his mark with the man behind him held off his own. It never costs a door its stack: where keeping off the furniture leaves 'no room', the places
   *  are asked again with the furniture left in, and the places back from a corner door take furnished floor last. Off = the makers ask the walls and the nav grid only, as they did
   *  BACK ON (orders.ts, the order tick's 'as near his mark as he will get'): a man who HAD his mark under him and stands within a metre of it, still —
   *  shouldered off it by a man walking past along the wall — steps back onto it. The older rule could not tell him from a man who never got there (the
   *  runner's settle watchdog counts a mark he has already been closer to as one he cannot reach) and moved the mark to where he had been pushed: half a
   *  metre out from the wall, for good. He goes back for it twice; kept from it both times (somebody stands on it now), the mark comes to him as before.
   *  Off = a man pushed off his mark is dealt the floor he was pushed to */
  ownLeaf: true, ownMark: true, offFurniture: true, backOn: true,
};
/** how often the rules above gave an answer (QA; `fur` = times a place was found on furniture: asks, not answers; `back` = times a man was sent back for a mark he was pushed off; `queue` = times a stack's deal was changed by a file with no room to pass, orders.ts fileQueue — the menu's own asks are counted too) */
export const STACK_OWN = { leaf: 0, mark: 0, fur: 0, back: 0, queue: 0 };

interface St {
  t: number;                 // the last frame the gate was asked (a gap = he has been doing something else: clocks afresh)
  heldT: number; pass: number;
  /** who he is held for this frame ('Player', or a man's callsign), as the gate's own log says it (showwhy.ts reads it) */
  by?: string;
  /** stepped off to a jamb: the place, the door and face it is on */
  spot: Vec3 | null; door: Door | null; side: 1 | -1; path: Vec3[] | null; noSpot: number; roundT: number;
  /** FIGHT: seconds planted in an opening; the step under way, its deadline; no step before */
  funnelT: number; step: Vec3 | null; stepBy: number; stepRest: number;
  /** leafStall: where he was when his walk last got somewhere, since when; the last frame it was asked; pushing leaves until */
  leafX: number; leafZ: number; leafT: number; leafSeen: number; leafBy: number;
}
const states = new SideMap<Guard, St>('doorway.states');
export const DOORMANNERS_STATS = { held: 0, aside: 0, passed: 0, stepped: 0, round: 0, rest: 0, marks: 0, covered: 0, stepCover: 0, stepGlass: 0, roundOwn: 0, leafStall: 0 };
export const DOORMANNERS_DEBUG: { log: ((s: string) => void) | null } = { log: null };

const flat = (a: Vec3, b: Vec3) => Math.hypot(a[0] - b[0], a[2] - b[2]);
function rel(d: Door, p: Vec3) { const F = frameOf(d), x = p[0] - F.hx, z = p[2] - F.hz; return { along: x * F.ux + z * F.uz, perp: x * F.nx + z * F.nz, F }; }
/** p in d's opening: inside the leaf's span, within `half` of the frame line */
export function inOpening(d: Door, p: Vec3, half = DOORMANNERS.half): boolean { const r = rel(d, p); return r.along >= -0.05 && r.along <= r.F.W + 0.05 && Math.abs(r.perp) <= half; }
/** p where d's leaf swings: inside its arc between its stops (and `sweep` past its tip), off the opening's own strip */
export function inSweep(d: Door, p: Vec3): boolean {
  if (d.off) return false;   // (off its hinges, doorblast.ts: no leaf swings there any more)
  const r = rel(d, p); if (r.along >= -0.05 && r.along <= r.F.W + 0.05 && Math.abs(r.perp) <= DOORMANNERS.half) return false;
  if (Math.hypot(r.along, r.perp) > r.F.W + DOORMANNERS.sweep) return false;
  const a = Math.atan2(r.perp, r.along); return a >= d.minA && a <= d.maxA && Math.abs(a) > 0.05;
}
/** STAND, the lane: the door standing open whose LANE `p` is in (DOORMANNERS.lane: the head of the knobs) — null: none, or the rule is off */
export function doorwayLane(g: Game, p: Vec3, but: Door | null = null): Door | null {
  const K = DOORMANNERS; if (!K.on || !K.stand || !K.lane) return null;
  for (const d of g.doors.list) {
    if (d === but) continue;   // (a stack's own door: stackCover asks about the others)
    if (!d.off && (d.latched || Math.abs(d.angle) < K.laneAjar)) continue;
    if (flat(p, d.frameCentre) > K.laneM + d.def.width) continue;
    const r = rel(d, p), depth = Math.abs(r.perp); if (depth > K.laneM) continue;
    const e = K.laneOut + K.laneFlare * depth; if (r.along < -e || r.along > r.F.W + e) continue;
    if (g.col.solidBlocked([p[0], 0.45, p[2]], [d.frameCentre[0], 0.45, d.frameCentre[2]])) continue;   // (a wall between him and the opening: nobody sees him through it)
    return d;
  }
  return null;
}
/** (STAND, the lane) nav.findPath's `penalty` for a walk an order gives: the lanes of the doors standing open onto floor the team has not been on (a room
 *  nobody of ours has stood in, or open ground: cleared.ts) are dear — bar the doors `not` names (the ones his way goes THROUGH: beyond them the lane is
 *  where he is going). Null: no such door, or the switch is off. */
export function doorwayLanePrice(g: Game, not: (d: Door) => boolean = () => false): ((ix: number, iz: number) => number) | null {
  const K = DOORMANNERS; if (!K.on || !K.stand || !K.laneWalk) return null;
  const C = g.cleared, open = g.doors.list.filter(d => { if ((!d.off && (d.latched || Math.abs(d.angle) < K.laneAjar)) || not(d)) return false;
    const a = C.roomBy(d, 1), b = C.roomBy(d, -1); return !(a && b && C.isKnown(a) && C.isKnown(b)); });
  if (!open.length) return null;
  const nav = g.col.nav, memo = new Map<number, number>();
  return (ix, iz) => { const i = nav.idx(ix, iz); let v = memo.get(i);
    if (v === undefined) { const p: Vec3 = [(ix + 0.5) * nav.cell, 0, (iz + 0.5) * nav.cell]; v = 1;
      for (const d of open) { if (flat(p, d.frameCentre) > K.laneM + d.def.width) continue;
        const r = rel(d, p), depth = Math.abs(r.perp); if (depth > K.laneM || (depth < K.laneStrip && (!K.laneLeaf || d.off || r.perp * d.angle <= 0))) continue;   // (no free strip along the wall on the face its leaf stands on: the grid knows no leaves, and that way round a lane is through the leaf)
        const e = K.laneOut + K.laneFlare * depth; if (r.along < -e || r.along > r.F.W + e) continue;
        if (g.col.solidBlocked([p[0], 0.45, p[2]], [d.frameCentre[0], 0.45, d.frameCentre[2]])) continue;
        v = K.laneDear; break; }
      memo.set(i, v); }
    return v; };
}
/** DOORMANNERS.leafStall, asked by game.ts for a man who goes round leaves, each frame he walks: true = he is walking on the spot by a leaf standing open
 *  (or was, less than `leafStallPush` ago) and this frame the leaf is his to push, not to go round */
export function leafStalled(g: Game, gd: Guard): boolean { return crossToRust('leafStalled', undefined, [g, gd]) as any; }



/** FIGHT. A man the fight AI would have stand where he is — planted to fire, or holding a post he was pinned on where he stopped: stood in an open doorway
 *  `funnel` seconds, he steps off the opening — one of the squad through it on the side his man (or his fix) is on, else back; a defender back on the side
 *  away from them, never out toward them — to a free spot with a line of fire to `aimAt`, and, with nobody in his sights (`needShot` false), failing that to
 *  any free spot off it. A defender's spot is never more exposed to a rifle at them than the frame itself (a window is no wall), covered floor comes before
 *  other free floor, and with them along the door's own wall covered floor on either face will do (`stepCover`). True = this file moved his feet this frame
 *  (he is on the step); false = stand as he would have. */
export function offThreshold(g: Game, gd: Guard, aimAt: Vec3 | null, dt: number, needShot: boolean): boolean { return crossToRust('offThreshold', undefined, [g, gd, aimAt, dt, needShot]) as any; }

/** REST / MARKS: `p` is in a door's way for a man left standing there — in some frame's own strip, or where a leaf swings (asked whatever `stand` and
 *  `swing` say: restClear and markClear are their own switches) */
export function doorwayInWay(g: Game, p: Vec3): boolean {
  for (const d of g.doors.list) { if (flat(p, d.frameCentre) > d.def.width + 1) continue; if (inOpening(d, p) || inSweep(d, p)) return true; }
  return false;
}
/** REST / MARKS: the nearest floor to `p`, within `clearM`, that is not `bad` — floor a man can stand on and walk straight to from `p`, never through a
 *  frame to its other face, `apart` clear of every `taken`; of the nearest ring that has any, the spot farthest from the doors by it. null: none */
export function doorwayClearNear(g: Game, p: Vec3, bad: (q: Vec3) => boolean, taken: Vec3[] = [], apart = DOORMANNERS.apart, face: Vec3 | null = null): Vec3 | null {
  // (`face`: with `p` in a frame's own strip, the face of that frame the answer is on — the one `face` is on)
  const K = DOORMANNERS, by = g.doors.list.filter(d => flat(p, d.frameCentre) <= d.def.width + 1 + K.clearM).map(d => { const at = rel(d, p); return { d, at: Math.abs(at.perp) <= K.half && face ? rel(d, face) : at }; });
  for (let r = 0.2; r <= K.clearM + 1e-6; r += 0.2) {
    let best: Vec3 | null = null, bs = -1;
    for (let k = 0; k < 12; k++) {
      const a = k * Math.PI / 6, q: Vec3 = [p[0] + Math.sin(a) * r, 0, p[2] + Math.cos(a) * r];
      if (g.col.nav.isBlocked(q[0], q[2]) || bad(q) || taken.some(t => flat(t, q) < apart) || g.col.solidBlocked([p[0], 0.45, p[2]], [q[0], 0.45, q[2]])) continue;
      let ok = true, far = Infinity;
      for (const { d, at } of by) { const rq = rel(d, q); far = Math.min(far, flat(q, d.frameCentre));
        if (Math.abs(at.perp) > K.half && rq.perp * at.perp < 0 && rq.along > -0.3 && rq.along < rq.F.W + 0.3) { ok = false; break; } }
      if (ok && far > bs) { best = q; bs = far; }
    }
    if (best) return best;
  }
  return null;
}
