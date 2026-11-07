// THE BIG SLOT'S TOOLS ON A DOOR (the maintainer, 10-14: "i do think we should change the breaching shotgun to not take up a primary weapon slot (ready or
// not and door kickers have it as a separate slot, like the wall charge) since its slugs are only effective on door hinges anyways" · "and then we should
// also add a dynamic hammer"). The switch, the card's row and the numbers are kit.ts's (`BIGSLOT`); with it off nothing here runs — no row is offered, no
// man carries either thing (kit.ts kitBig, Player.bigKit stay 'none').
//
//   THE BREACHING SHOTGUN IS KIT. Carried in the big slot it is nobody's gun: a man keeps his rifle or his pistol, and the M870 comes out for a LOCK and
//   nothing else. What it does to the lock is shotgun.ts's own rule (fireShell: enough of one shell into a latched, locked leaf by its latch edge from
//   close, and the keep is gone — doors.ts kickIn with `lock`, so from the face the leaf opens toward it stands home unlocked for a hand to pull; a wedge
//   under the leaf is no keep and steel rings and holds). The shell is a shell: its flash, report and hearing event are the row's — heard as gunfire.
//     · YOU: a row of a door locked against you, in the boot's place ('shoot the lock off'). One committed beat (Player.kick with `tool`): planted on the
//       boot's spot, `BIGSLOT.draw` seconds to bring it up, a shell at the lock; a lock that holds gets up to `BIGSLOT.shells` of them.
//     · A TEAMMATE: the door tools' own lock-shooting (doortools.ts toGun / gunStep, written for a man whose GUN was the shotgun) with the gun brought up
//       off its sling as he comes onto his spot (gunUp: his own gun put down for it, `BIGSLOT.draw` seconds before the shell) and slung again the moment
//       the lock is off, the gun is given up on, or the fight has him (gunDown — doortools.ts slingTick is the net under every other way out).
//
//   THE DYNAMIC HAMMER. ONE blow where a boot would do (the leaf opens away from him: never slower than the boot), `BIGSLOT.swings` blows, one every
//   `BIGSLOT.swing` seconds, where the frame holds a boot (bigkit.ts hammerBlows) — and the lock gives: the last blow is doors.ts kickIn with `lock` — the
//   boot's verb and the boot's noise (a 'kick' the whole floor hears, never a shot), from EITHER face: where the frame holds a boot (the face the leaf
//   opens toward) the keep is broken out and the leaf stands home for a hand, as after the shotgun. It takes a wedged door too (the shotgun does not);
//   steel still holds. The blows before the last are the frame taking it: a thud each.
//     · YOU: the same row and the same beat ('hammer it open').
//     · A TEAMMATE: the door tools' lock stage again (toGun picks the man — a shotgun first, where the lock is one it takes — and hammerStep swings).
//
//   THE MOVES (BIGSLOT.moves — the maintainer, 10-15, with Door Kickers 2 footage: "i think the dynamic hammer and shotgun animations / pathing need some
//   work" · "using the hammer shouldn't require being directly in front of the door?" · "the character should have a hammer on their back, they put away
//   their gun, equip the hammer, wind up, and then hit the door in one motion"). What the build before did at the clinic's ward door with the four stacked
//   on it: the row was offered only inside the hand's reach of the leaf (doors.ts USE_REACH, 1.35 m of its middle — nothing from the stack's file), and the
//   one place beside the frame it knew (the latch jamb, kickSpotSide) was the lead's own mark, so he was slid to the boot's spot square in the opening.
//     · WHERE (bigSpots): a list of places for the man with the tool, best first — hard by the latch jamb, then (on the face the leaf opens away from) by
//       the hinge jamb, then off the shoulder of a man who has the jamb — every one of them OUT of the opening; the shotgun's a short arm nearer the lock
//       than its pattern needs. The first with floor, a line to the leaf and nobody standing on it is his. A teammate's list is the same one (shotgun.ts
//       breachSpot asks it first).
//     · YOU: the row is yours from BIGSLOT.reach of the doorway while such a place can be walked to in a straight line, and he WALKS there (samBigWalk) with
//       the tool coming into his hands on the way; the boot's spot in the opening only where nothing beside the frame has floor, and only from the hand's reach.
//     · THE BEAT: his gun put away and the tool in both hands in BIGSLOT.equip s (the shotgun's BIGSLOT.draw), each blow wound up over BIGSLOT.windup s and
//       landed in one swing, BIGSLOT.stow s to put it back. The body's side of it is the animator's (character.ts bigTool — a pose that is a pure function
//       of the beat's clock, written here every frame: samBigPose, hammerOutTick), the hammer on a back or in the hands is rigProps.ts's.
//   Off (`?nobigmoves`, a recording from before the key): the row inside the hand's reach, the one place or the opening, the punch standing in.
// NOBODY HAS SEEN ANY OF IT ON A GPU. With the moves off neither thing is drawn in your hands (a teammate's shotgun is, being a gun in his hands for the
// beat) and the hammer's blow is the strike a takedown uses, standing in.
// No dice, no camera, nothing read off the renderer. State: Player.kick.tool (a committed beat: savestate.ts refuses a save while a kick runs) and
// Guard.slung (on the man: the frame after a load slingTick finds him with no lock to shoot and gives him his own gun back).
// (src/game is one import cycle: nothing below is built from another game module's exports at module load.)
import { manById } from './seats';
import { v3, type Vec3 } from '../math/vec';
import type { Game, Guard } from './game';
import type { Door } from './doors';
import { BIGSLOT, kitBig } from './kit';
import './weapons';
import './combat';
import { lockPoint, breachSpot } from './shotgun';
import { offOf, doorPt, alongOf } from './doortools';
import './chronicle';
import './player';
import { bigRebound, KIT, type Character } from './character';
import { KITANIM, kitInHand, kitSlingFront } from './kitanim';
import './handwork';
import { RUST_PASS, RUST_SURFACE, crossToRust, goneWithTick } from './crossing';

/** counted since the page (or the harness) stood up — the check's and the panel's */
export const BIG_STATS = { samShells: 0, samShot: 0, samBlows: 0, samHammered: 0, drawn: 0, slung: 0, mateBlows: 0, mateHammered: 0, /** BIGSLOT.moves: a teammate's hammer off his back · put back · you walked onto a place beside a frame · of those, by the hinge jamb or off a man's shoulder */ hammersOut: 0, hammersIn: 0, samWalks: 0, samOther: 0 };
const BREAK = (() => { try { return String((globalThis as { process?: { env?: Record<string, string | undefined> } }).process?.env?.BIGKIT_BREAK ?? '') === '1'; } catch { return false; } })();
/** `BIGWEIGHT_BREAK=1` (the check's): the weight is never on — the tool's times and the gun's let-go are the build before's whatever the switch says */
const WEIGHT_BREAK = (() => { try { return String((globalThis as { process?: { env?: Record<string, string | undefined> } }).process?.env?.BIGWEIGHT_BREAK ?? '') === '1'; } catch { return false; } })();

// ------------------------------------------------------------ you
/** the big slot's tool that OPENS this door from where you stand (null: none carried, the switch off, a show, the door open, off its hinges or steel, a
 *  wedge under it and only the shotgun to hand — or, with BIGSLOT.shut off, not locked against you) */
export function samBig(g: Game, d: Door, p = g.player.char.pos): 'shotgun' | 'hammer' | null { return goneWithTick('samBig') as any; }
/** WHERE YOU WORK IT FROM (BIGSLOT.beside; null = the boot's own spot): beside the latch jamb, out of the opening — the shotgun where its pattern has a
 *  line to the lock (shotgun.ts breachSpot, a teammate's own spots, none of them on a man), the hammer a short arm from it (doors.ts kickSpotSide) — when
 *  that floor is free, a couple of steps off and nothing solid stands between it and you. */
export function samBigSpot(g: Game, d: Door, from: Vec3, tool: 'shotgun' | 'hammer'): { pos: Vec3; yaw: number; at?: Vec3; /** BIGSLOT.moves: the one corner of his walk there, round a man of the file */ via?: Vec3 } | null { { const __s = RUST_SURFACE; if (__s) { const __r = __s.call('samBigSpot', undefined, [g, d, from, tool]); if (__r !== RUST_PASS) return __r as any; } }
  if (!BIGSLOT.beside || d.off) return null;
  if (BIGSLOT.moves) {   // (the list's first place he can walk to that nobody stands on — a stack's lead on the latch jamb leaves him the next one)
    const side = bigSide(d, from), men = g.guards.filter(m => m.state !== 'dead').map(m => m.char.pos), n0 = doorPt(d, 0, 0, side), n1 = doorPt(d, 0, 1, side), nx = n1[0] - n0[0], nz = n1[2] - n0[2];
    /** how near the straight line a→b a point stands */
    const off = (m: Vec3, a: Vec3, b: Vec3) => { const ax = b[0] - a[0], az = b[2] - a[2], L2 = ax * ax + az * az; if (L2 < 1e-4) return v3.distXZ(m, a);
      const k = Math.max(0, Math.min(1, ((m[0] - a[0]) * ax + (m[2] - a[2]) * az) / L2)); return Math.hypot(m[0] - a[0] - ax * k, m[2] - a[2] - az * k); };
    const leg = (a: Vec3, b: Vec3, but?: Vec3) => !g.col.solidBlocked([a[0], 0.5, a[2]], [b[0], 0.5, b[2]]) && !men.some(m => m !== but && off(m, a, b) < MOVE_LINE);
    // (the places on HIS side of the opening before the ones across it: he does not walk the doorway's width to a jamb while there is a place where he came from)
    const mine = alongOf(d, from) < d.def.width / 2, list = bigSpots(g, d, side, tool), order = [...list.filter(x => x.hinge === mine), ...list.filter(x => x.hinge !== mine)];
    for (const s of order) {
      if (v3.distXZ(s.pos, from) > BIGSLOT.reach + MOVE_SLACK || men.some(m => v3.distXZ(m, s.pos) < MOVE_CLEAR)) continue;
      if (leg(from, s.pos)) return s;   // (a straight walk)
      // a man of the file between him and it: ROUND him on the side off the wall — one corner, abreast of that man, with floor and both legs clear
      const m = men.filter(q => off(q, from, s.pos) < MOVE_LINE).sort((a, b) => v3.distXZ(a, from) - v3.distXZ(b, from))[0]; if (!m || g.col.solidBlocked([from[0], 0.5, from[2]], [s.pos[0], 0.5, s.pos[2]])) continue;
      const via: Vec3 = [m[0] + nx * MOVE_ROUND, 0, m[2] + nz * MOVE_ROUND];
      if (!g.col.nav.isBlocked(via[0], via[2]) && leg(from, via) && leg(via, s.pos)) return { ...s, via };
    }
    return null; }
  const ok = (pos: Vec3) => v3.distXZ(pos, from) <= BESIDE_REACH && !g.col.nav.isBlocked(pos[0], pos[2]) && !g.col.solidBlocked([from[0], 0.5, from[2]], [pos[0], 0.5, pos[2]])
    && !g.guards.some(m => m.state !== 'dead' && v3.distXZ(m.char.pos, pos) < BESIDE_CLEAR);   // (never onto a man: a stack's lead stands on that jamb)
  if (tool === 'hammer') { const s = g.doors.kickSpotSide(d, from); return ok(s.pos) ? s : null; }
  const s = breachSpot(g, d, bigSide(d, from), g.guards.filter(m => m.state !== 'dead').map(m => m.char.pos));
  return s && s.beside && ok(s.pos) ? { pos: s.pos, yaw: s.yaw } : null;
}
/** BIGSLOT.moves: a place further than the row's reach plus this is not walked to (m) · how far off a place every man must be (m: a shoulder's width — he works
 *  beside the lead, not on him) · his pace onto it (m/s: a walk) · seconds the walk may take before the beat begins from where he got to, or is given up */
const MOVE_SLACK = 1.2, MOVE_CLEAR = 0.45;
/** … and how near the straight walk to a place a man may stand (m: nearer, and that place is passed over) */
const MOVE_LINE = 0.3;
/** … and how far off the wall from such a man the corner of the walk round him is (m) */
const MOVE_ROUND = 0.75;
/** how far proud of the leaf the point a hammer lands on is (shotgun.ts lockPoint's own), and how far in from the hinge edge a blow from the hinge jamb lands */
const PROUD = 0.06, HINGES_IN = 0.22;
/** THE PLACES A MAN WORKS A DOOR'S TOOL FROM, on face `side`, best first (BIGSLOT.moves) — [metres past the leaf's edge along the frame line, metres off the
 *  wall]: `jamb` hard by a jamb (the latch's first; the hinge's only on the face the leaf opens AWAY from — on the other the leaf is pulled round over that
 *  floor), `shoulder` off the shoulder of a man who has the latch jamb. All out of the opening. The hammer a short arm from the leaf; the shotgun's muzzle
 *  a hand from the lock. */
const SPOTS: Record<'hammer' | 'shotgun', { jamb: [number, number][]; shoulder: [number, number][] }> = {
  hammer: { jamb: [[0.3, 0.55], [0.18, 0.8]], shoulder: [[0.12, 1.12]] },
  shotgun: { jamb: [[0.35, 0.7], [0.22, 0.88]], shoulder: [[0.12, 1.15]] } };
export type BigSpot = { pos: Vec3; yaw: number; /** the point struck or shot */ at: Vec3; /** by the hinge jamb */ hinge: boolean };
/** … those of them with floor to stand on and a clear line to the leaf (who stands where is the caller's to ask) */
export function bigSpots(g: Game, d: Door, side: 1 | -1, tool: 'shotgun' | 'hammer'): BigSpot[] { { const __s = RUST_SURFACE; if (__s) { const __r = __s.call('bigSpots', undefined, [g, d, side, tool]); if (__r !== RUST_PASS) return __r as any; } }
  const W = d.def.width, lock = lockPoint(d, side), out: BigSpot[] = [], T = SPOTS[tool];
  const put = (pos: Vec3, at: Vec3, hinge: boolean) => { if (!g.col.nav.isBlocked(pos[0], pos[2]) && !g.col.solidBlocked([pos[0], 1.2, pos[2]], at)) out.push({ pos, yaw: v3.yawTo(pos, at), at, hinge }); };
  const shoulder = () => { for (const [past, off] of T.shoulder) put(doorPt(d, W + past, off, side), lock, false); };
  for (const [past, off] of T.jamb) put(doorPt(d, W + past, off, side), lock, false);
  if (tool === 'shotgun') shoulder();   // (a shell is the lock's or nothing's — shotgun.ts: the lock's own side, shoulder and all, before the far jamb)
  if (!g.doors.pullFace(d, doorPt(d, W / 2, 0.5, side))) { const at = tool === 'hammer' ? doorPt(d, HINGES_IN, PROUD, side, lock[1]) : lock;   // (the hammer by the hinges, where he stands)
    for (const [past, off] of T.jamb) put(doorPt(d, -past, off, side), at, true); }
  if (tool === 'hammer') shoulder();
  return out;
}
/** BIGSLOT.moves — THE ROW FROM WHERE A STACK STANDS (player.ts buildInteractables, for a door outside the hand's reach): within BIGSLOT.reach of the
 *  doorway, a tool that opens it, and a place beside the frame he can walk to. */
export function samBigFar(g: Game, d: Door, p: Vec3): 'shotgun' | 'hammer' | null { return crossToRust('samBigFar', undefined, [g, d, p]) as any; }
/** BIGSLOT.weight — THE TOOL CHANGES HANDS WITH THE WEIGHT OF A GUN SWAP (the maintainer, 10-18: "the swaps when taking out the hammer and putting it back
 *  feel too fast -- swapping the guns feels good, can we make sure they all feel similarly physical?"). A rifle off his back takes the kit's hand 0.71 s
 *  (character.ts KIT: reach + draw) and 0.76 s to put back (stow + the hand away); the hammer was in both hands 0.6 s after his own gun VANISHED from them
 *  and back on him in 0.45 s. With the weight on: the tool's own times are `equipW` / `drawW` / `stowW`; YOUR gun is first carried to its holster or sling
 *  by the kit's own hand (samBigLeadOf: the beat's clock starts that far below zero, kitanim.ts runs the errand — samBigCarry); and the hammer's first
 *  wind-up may begin `flow` s before the hammer is fully out (bigOutBy), so out, up and down read as one move — `flow` is the whole of what was added to
 *  the hammer's way out, so a blow from empty hands lands when it did and a blow with a gun up is later by that gun's own stow, no more. A teammate's
 *  record carries the same times (his own gun hangs on him: it still falls to its sling as his hands leave it); his hammer's blow waits what it did, his
 *  shotgun's shell `drawW`. Off (`?nobigweight`, a recording from before the key) = the times and the let-go of the build before. */
const weighed = (): boolean => BIGSLOT.moves && BIGSLOT.weight && !WEIGHT_BREAK;
/** … is on (kitanim.ts asks) */
export const bigWeighed = (): boolean => weighed();
/** … seconds from the hand leaving for the tool to the tool in his hands · to put it back · after which a hammer's wind-up may begin (a shell: the same as bigEquip) */
export const bigEquip = (kind: 'hammer' | 'shotgun'): number => goneWithTick('bigEquip') as any;
export const bigStow = (): number => goneWithTick('bigStow') as any;
export const bigOutBy = (kind: 'hammer' | 'shotgun'): number => goneWithTick('bigOutBy') as any;
/** … how long YOUR own gun (or the can in your fist) takes to be put away by hand before the hand goes for the tool: the kit's own stow for what its hand
 *  HOLDS (`kitWas` — what the last errand brought there, not what the game says is up: a pistol the wall's own arms have just handed back was never the kit's
 *  to carry, and is let go as before; a rifle on a sling in front is let down, not taken over the shoulder) — 0 with nothing in that hand, with the kit's hand
 *  off, or the weight off. An upper bound: an errand that is over sooner (a gun asked back half way out) ends the wait with it — samBigTick. */
export function samBigLeadOf(g: Game): number {
  if (!weighed() || !KITANIM.on || !KITANIM.equip || g.touring) return 0;
  const it = g.player.kitWas; if (!it || !kitInHand(g)) return 0;
  return it === 'rifle' && kitSlingFront(g) ? KIT.front.stow : KIT.stow[it];
}
/** … and that errand is the kit's to run this frame (kitanim.ts kitTick / kitInHand): while the beat's clock is below zero, and after it until the empty
 *  hand has come away from the holster (the tool's own pose starts on the hand the pose under it shows: an errand dropped there would jump it) */
export function samBigCarry(g: Game): boolean {
  const T = g.player.kick?.tool; if (!T?.moves || !(T.lead! > 0)) return false;
  return T.t < 0 || (g.player.char.anim.kitBusy && T.t < CARRY_TAIL);
}
const CARRY_TAIL = 0.6;
/** the hammer's wind-up (s): `BIGSLOT.heft` with the moves on — the swing is drawn — and `BIGSLOT.windup`, as it was, with them off */
export const bigWind = (): number => goneWithTick('bigWind') as any;
/** … and from one blow to the next: with the moves on never less than a follow-through and a wind-up */
export const bigSwing = (): number => goneWithTick('bigSwing') as any;
/** BIGSLOT.moves — THE TAIL: the beat is over and his legs are his own, the tool still going back onto him (the pose kept on `anim.bigTool`, its clock kept
 *  here). No round leaves his gun inside it (`equipT`, set as the beat ends — samBigEnded), no tool's row is his (samBig), no checkpoint is written
 *  (savestate.ts), his goggles are a nod (kitanim.ts). */
export const samBigTail = (g: Game): boolean => goneWithTick('samBigTail') as any;
/** … what is left of it (s) */
export const samBigLeft = (g: Game): number => { return goneWithTick('samBigLeft') as any; };
/** … dropped (player.ts cancelActions — a teleport, a show, a restage: his hands are his own, and nothing of the beat is left) */
export function samBigDrop(g: Game) { g.player.char.anim.bigTool = null; }
/** how far he is taken to that spot (m) */
const BESIDE_REACH = 1.7;
/** … and how far off it every man must be (m) */
const BESIDE_CLEAR = 0.6;
/** the row performed (game.ts): the beat begins — player.ts startKick with the tool in the boot's place */
export function samBigStart(g: Game, d: Door): void { return crossToRust('samBigStart', undefined, [g, d]) as any; }
/** the blows the hammer needs on d from p: ONE where a boot would do (the leaf opens away: it is flung in, in the boot's own time) — `BIGSLOT.swings` where
 *  the frame holds a boot (doors.ts bootHolds: the face it opens toward), the first a thud in the frame */
export const hammerBlows = (g: Game, d: Door, p: readonly number[]): number => (d.locked && g.doors.bootHolds(d, p as [number, number, number]) ? Math.max(1, BIGSLOT.swings) : 1);   // (a latch that is not locked gives at one blow from either face)
/** which face of d he is on (the lock point's side) */
export const bigSide = (d: Door, p: readonly number[]): 1 | -1 => (offOf(d, p as [number, number, number], 1) >= 0 ? 1 : -1);

// ------------------------------------------------------------ carried where it can be seen (BIGSLOT.moves)
/** the big slot's tool a man carries, for the drawer (combat.ts handProps → rigProps.ts): null with the moves off, on a show's puppet, or with nothing carried */
export function bigCarried(g: Game, c: { id: number }): 'hammer' | 'shotgun' | null {
  if (!BIGSLOT.on || !BIGSLOT.moves) return null;
  { const pl = manById(g, c.id); if (pl) { const k = pl.bigKit; return k === 'hammer' || k === 'shotgun' ? k : null; } }
  const gd = g.guards.find(m => m.char.id === c.id); if (!gd) return null;
  const k = kitBig(gd); return k === 'hammer' || k === 'shotgun' ? k : null;
}

// ------------------------------------------------------------ a teammate
/** he carries the hammer in his big slot (kit.ts: a staged teammate's card) */
export function carriesHammer(gd: Guard): boolean { return gd.state !== 'dead' && kitBig(gd) === 'hammer'; }
/** … and is slung again: his own gun back in his hands, as he put it down (a dead man keeps what he fell with) */
export function gunDown(gd: Guard): void { return crossToRust('gunDown', undefined, [gd]) as any; }
/** … and put back: begun now — or as the blow that has just landed is followed through (a blow still to come is not swung); hammerOutTick has it on his back
 *  BIGSLOT.stow seconds on. `hard` (the fight has him — doortools.ts; he fires — bigFired): on his back AT ONCE, his gun in his hands, as the sim has it. */
export function hammerIn(g: Game, gd: Guard, hard = false) { { const __s = RUST_SURFACE; if (__s) { const __r = __s.call('hammerIn', undefined, [g, gd, hard]); if (__r !== RUST_PASS) return __r as any; } }
  const H = gd.bigOut; if (!H) return;
  if (H.end < 0) BIG_STATS.hammersIn++;
  if (hard) { gd.bigOut = null; gd.char.anim.bigTool = null; return; }
  if (H.end >= 0) return;
  const now = g.time - H.t0; H.blows = H.blows.filter(b => b <= now);
  H.end = H.blows.length ? Math.max(now, H.blows[H.blows.length - 1] + bigRebound('hammer')) : now;   // (a blow just landed is followed through first)
}
/** a round has left this man's gun (combat.ts fireWeapon, a man other than Sam with the tool's pose on him): the hammer is on his back — what is drawn is what the sim did */
export function bigFired(g: Game, c: Character) { const gd = g.guards.find(x => x.char === c); if (gd?.bigOut) hammerIn(g, gd, true); else c.anim.bigTool = null; }
