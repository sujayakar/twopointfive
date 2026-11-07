// AN ALERTED MAN DOES NOT WAIT AT A SHUT DOOR (DOORFORCE.on: on in the page, ?doorforce=0; off here and on the frozen stand, TPS_DOORFORCE=1).
//
// The maintainer, after "opening a door into the enemy should majorly confuse them": "and if they're alerted they should behave realistically, like
// shooting through the door or eventually kicking through it". Until this a defender who was up and knew a man stood at the other side of a shut door did one of
// two things: holding a post he rested his gun on the door and waited for it to open; free to move he walked to its handle and opened it by hand, into whoever
// was there. (coverfire.ts already has a man who SAW somebody go behind a wooden leaf put one short burst through it — wallpen.ts `men` — and then look.)
//
//   WHO       a man of the defending sides (never a teammate: the squad's orders decide what its men do) whose state is 'alert', on his own feet and senses —
//             not scripted, not taken, not dazzled or doubled over — and with nobody in his eyes (the fight AI's own branch: a man he SEES is fought as ever).
//   KNOWS     his own senses and nobody's truth. Per shut door he keeps how SURE he is that an enemy is at its other side, and where:
//               · his EARS, through the hearing model's own answer for that sound and that listener (guards.ts hearingCheck hands it over): a footfall or
//                 a report made by an enemy within `behindM` of the leaf on its far face, that came to him ACROSS a shut leaf (routed, carrying no more
//                 than `shutCarry` — a sound through open air or round an open way is the fight AI's as ever, whatever door its maker stands near). Each
//                 adds `gain` × how loud it reached him (a shot at the gunshot line or over: `shot`) — so a handful of walked footfalls close behind the
//                 leaf, or two at a run. A crouched tread through a shut door is under `hearMin`: it adds nothing. The place is where the sound was, a
//                 hashed `heardErrM` off (no die drawn). A door's own noise (a tried handle, a pick, a boot its frame held) is word of THAT door, at
//                 least `handle`, placed at its far face — and only while the man whose hand was on it still stands there.
//               · his EYES: the man he is fighting, in his sight a moment ago (`seenSecs`), with a shut leaf now the first thing in his eyes' line to
//                 where he last saw him: `seen`. A man gone round a corner or out by an open doorway beside a shut door is not behind it.
//             What he is sure of fades to a third in `fadeSecs`. He starts on a door at `sure` and stays on it down to `keep`, while his newest word of it is
//             no older than `staleSecs`. A door of his room that he is SET on behind a wedge (digin.ts) is his barricade: he does not start on it. A stand
//             or a boot the fight AI stops asking about (a man in his eyes, a flash, a blast, cover) is over; taken up again it starts again.
//   ROUNDS    a WOODEN leaf this gun's round crosses (wallpen.ts thinLine: the answer is the round's own flight), the place within the leaf's line from his
//             gun — the place itself, or, for a man right AT the door (`nearLeafM`) with the wall and not the leaf in the straight line, the stretch of leaf
//             he is nearest: he stands, lays on it by the aim law every round of his obeys, and puts `rounds` through; then he listens — the next rounds
//             want a newer word than the last ones were fired on, and `gapSecs`. Nobody of his side, no hostage and no civilian in the lane or past it as
//             far as the round would fly (coverfire.ts coverLane). A burst coverfire.ts has just put through the leaf on a sighting is the first. Steel, a leaf his
//             round is spent in, rounds that stop in doors (wallpen.ts off): no round. The leaf takes what a leaf takes: with doorblast.ts DOORSHOT it goes
//             in pieces on its own count, and then there is no door to wait at.
//   THE BOOT  a FREE man (no post, no hold) who has been on the door `kickSecs` — `kickSecsNoShot` when he has no round for it — and is still sure walks to
//             the boot's own spot (doors.ts kickSpot) and kicks it in (kickIn: the keep torn out, the leaf flung, the floor's loudest noise short of a
//             shot), one man a door; then he is the fight AI's again, with the place he believed in as his fix. Where the frame would hold a boot (the
//             face the leaf opens toward — DOORWAY.frameHolds) or the leaf is steel he does not throw one: after the same wait he goes through it by the
//             handle, as he always did. A man holding a post never leaves it for this: his answer is the rounds.
//             The boot is his side's own noise: no man of it (he least of all) takes it for an intruder at a door (guards.ts hearingCheck asks
//             doorForceOwnBoot), and the torn keep is no puzzle to a colleague on a calm pass later.
//             What the flung leaf does to a man behind it is not here (the sibling rule; until it lands the kick is the kick as it was).
//
// Off = every hook answers on its first line: no state, no die, no round (the build as it was, run digest for run digest). No dice at all with it on: the
// rounds' own cadence and cone draw as any round of his does. State per Game in a named side table (math/side.ts: a rollback snapshot holds it); a restart or
// a checkpoint load forgets it with the roster (game.ts resetGuards), as coverfire.ts's is: a belief is a few seconds old at most.
// (src/game is one import cycle: nothing below is built from another game module's exports at module load.)
import { GameSideMap } from '../math/side';
import { Vec3, v3 } from '../math/vec';
import type { Game, Guard } from './game';
import type { Door } from './doors';
import './teams';
import './guards';
import './combat';
import './suppression';
import './aim';
import './character';
import './wallpen';
import './glasspen';
import './coverfire';
import './digin';
import { crossToRust } from './crossing';

export const DOORFORCE = {
  on: false,
  /** a man walking to boot a door who is put on a post gives the walk up (a switch of its own: ?postends=0, TPS_POSTENDS=1) */
  postEnds: false,
  /** the two halves, each off = that half as it was: rounds through the leaf · the boot */
  shoot: true, kick: true,
  /** how sure he must be to start on a door · down to what he stays on it · s his sureness takes to fade to a third · s his newest word of it may be old */
  sure: 1, keep: 0.3, fadeSecs: 5, staleSecs: 6,
  /** what a sound adds: `gain` × (its level × what the path carries × its falloff — guards.ts's own sum), nothing under `hearMin` of that sum · a report
   *  at the gunshot line or over · at least this for a hand on THIS door · the sighting of a man who is now behind it */
  gain: 4, hearMin: 0.03, shot: 1, handle: 0.6, seen: 1.5,
  /** a sound whose way to him carries more than this did not come through a shut leaf (collision.ts propagate: about 0.3 across one, 0.7 round an open way) */
  shutCarry: 0.5,
  /** m from the leaf's middle to where the enemy stood, on its far face · m from the leaf to the listener, on his · s a sighting stays worth a door */
  behindM: 2.5, reachM: 9, seenSecs: 3,
  /** m a heard place is put off the sound (hashed off the event and the man) · the height above the floor his rounds are laid at */
  heardErrM: 0.4, chestY: 1.15,
  /** m from the leaf inside which a man is AT the door: with the wall in the straight line, the rounds go through the stretch of leaf nearest him, to a
   *  point this far behind it */
  nearLeafM: 1.2, pastLeafM: 0.6,
  /** rounds on one word · s before the next ones (which want a newer word) */
  rounds: 4, gapSecs: 1.5,
  /** s on the door before a free man goes to boot it: with rounds for it · with none (steel, a leaf his round does not cross, no line) */
  kickSecs: 4, kickSecsNoShot: 2,
  /** his walk to the boot's spot is given up after this many seconds · s a door he has dealt with (or given up) is left alone */
  walkSecs: 6, restSecs: 8,
};
/** sounds that counted for a door · sightings that did · stands begun (a man starts on a door) · rounds fired on one · rounds held for a friend in or past
 *  the lane · boots thrown · of them the door gave · boots not thrown (the frame would hold it, steel, no floor at the spot, the walk given up) */
export const DOORFORCE_STATS = { heard: 0, seen: 0, stands: 0, shots: 0, heldFriend: 0, kicks: 0, gave: 0, noKick: 0 };
export const DOORFORCE_LOG: string[] = [];

const BREAK = (() => { try { return String((globalThis as { process?: { env?: Record<string, string | undefined> } }).process?.env?.DOORFORCE_BREAK ?? '') === '1'; } catch { return false; } })();

/** what one man believes of one door: how sure, where the man is (the floor), when he last had word, and that word's kind */
interface Belief { door: Door; sure: number; at: Vec3; t: number; seen: boolean }
/** the door he is on: since when, the rounds of this word, the word they were fired on, when the next may go, his lay on the spot */
interface Stand { door: Door; t0: number; n: number; firedT: number; next: number; lay: number; fired: number; noShotT: number; to: Vec3 | null; toKey: [number, number, number] }
interface Kick { door: Door; spot: Vec3; yaw: number; phase: 'walk' | 'set' | 'kick'; t0: number; setT: number }
/** `served`: the last time the alert branch asked for him (a stand or a boot nobody asks about is over) · `cfT`: when coverfire.ts last had a round of his through a thin thing */
interface Man { beliefs: Map<Door, Belief>; saw: { at: Vec3; t: number; used: boolean } | null; on: Stand | null; kick: Kick | null; rest: Map<Door, number>; served: number; cfT: number }
/** `boots`: who threw one of this rule's boots, and when (his own side does not take it for an intruder's) */
interface State { lastT: number; men: Map<Guard, Man>; kickers: Map<Door, Guard>; boots: Map<number, number> }
const states = new GameSideMap<Game, State>('doorforce.states');

function log(g: Game, s: string) { DOORFORCE_LOG.push(`${g.time.toFixed(2)} ${s}`); if (DOORFORCE_LOG.length > 400) DOORFORCE_LOG.splice(0, 200); }






// ---------------------------------------------------------------- QA and the panel
/** QA: what `gd` believes of `d` (null = nothing) */
export function doorForceBelief(g: Game, gd: Guard, d: Door): { sure: number; at: Vec3; seen: boolean; age: number } | null {
  const B = states.get(g)?.men.get(gd)?.beliefs.get(d); return B ? { sure: B.sure, at: v3.copy(B.at), seen: B.seen, age: g.time - B.t } : null;
}
/** QA: what `gd` is doing about a door this frame */
export function doorForceOn(g: Game, gd: Guard): { door: Door; fired: number; kick: 'walk' | 'set' | 'kick' | null } | null {
  const M = states.get(g)?.men.get(gd); if (!M) return null;
  if (M.kick) return { door: M.kick.door, fired: M.on?.fired ?? 0, kick: M.kick.phase };
  return M.on ? { door: M.on.door, fired: M.on.fired, kick: null } : null;
}
/** QA (tools/qa/doorforce-check.ts): hand `gd` the record a real sighting of his foe at `at` leaves, `age` seconds old. The game never calls it. */
export function doorForceSaw(g: Game, gd: Guard, at: Vec3, age = 0): void { return crossToRust('doorForceSaw', undefined, [g, gd, at, age]) as any; }
/** a restart or a checkpoint load (game.ts resetGuards: the roster is new men) / QA: forget everything about this game */
export function doorForceReset(g: Game) { states.delete(g); }
/** panel / console line */
export function doorForceSummary(): string {
  const s = DOORFORCE_STATS;
  return `an alerted man at a shut door ${DOORFORCE.on ? 'ON' : 'off'} ▸ stands ${s.stands} (heard ${s.heard}, seen ${s.seen}) · rounds through ${s.shots} (held for a friend ${s.heldFriend}) · boots ${s.kicks} (gave ${s.gave}; not thrown ${s.noKick})${DOORFORCE_LOG.length ? ` · last: ${DOORFORCE_LOG[DOORFORCE_LOG.length - 1]}` : ''}`;
}
