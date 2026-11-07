// AIMED FIRE CHEWS COVER (the destruction goal's AI piece; record rebuild/destruction/ai-fire.md). A shooter of either side who KNOWS a man is
// behind a breakable thing — a crate, a carton stack, a fabric partition, a thin drywall wall — puts his aimed rounds into it, on one spot, until it gives: a
// cover cell at a time, a partition column to a port. Then the fight is the fight again: he sees the man or he does not.
//   KNOWS     his own knowledge and nobody else's truth: where he last really SAW the man (the fight AI's sighting / the room fight's meter, copied here the
//             frame it happens: coverfireTick), `staleSecs` old at most — or a mate of his side within `callM` who has eyes on that man now (`callSecs`: a
//             call is good for the crate a man is behind, never for a wall into a room this man has not seen him in). Never the man's position read off the
//             game. A place heard is not a place known: only sightings are kept.
//   WHAT      the engine is asked what the line from his gun to that place meets (col.raycast → walls.breakableAt → walls.workLeft): the first thing in it must
//             be a breakable the engine indexed, the man's place within `behindM` beyond its face, nothing but more breakable between it and him, at most
//             `maxLayers` pieces deep and `maxRounds` of this gun's rounds in all. Concrete, metal, a door leaf, glass, loose furniture, anything unindexed, or
//             a wall with nobody known behind it: no round. The place asked about is where he was seen; for a man seen MOVING, also a stride or two on the
//             way he was going (`leadM`, `leadSecs`: he went behind it); and when the line to where he stood is CLEAR yet shows no man, a point `lowY` off the
//             floor there (he ducked).
//   HOW       the same gun, cadence, magazine, reload beat, hands and settle law as any other round of his (his side's skill row: a defender lays slower and
//             wider than a teammate, by the rows that already say so). The aim point is held on one spot while the place he knows stays within `sameM` of it,
//             so the engine's counts add up on one cell. Rounds stop at what they hit, as ever.
//   STOPS     the piece gives (the change feed: his count starts again on what stands behind, if anything) · the knowledge goes stale · he has put
//             `slackK` × what it wanted + `slack` rounds into it and it stands (given up for `giveSecs`) · a friend, a hostage or a civilian is in the lane or
//             beyond it as far as a round that got through would fly (`lane`, `pastM`) · the silent order holds his round · and a FREE man does not start at
//             all when walking round is quicker than shooting through (`roundSecs` a round against the nav path at `walkPace`) — a man on a post or under an
//             order has no feet of his own to weigh.
// THE SWITCH is COVERFIRE.on, and it only ever runs under the destruction goal's own (wallBreakOn: DESTRUCTION.on, tune.wallBreak, chaos > 0). Off = every
// hook returns on its first line: no state, no dice, no round (play-through digests equal the base's). The page: on unless the URL carries ?nocoverfire.
// WITH ROUNDS GOING THROUGH THIN THINGS (wallpen.ts, WALLPEN.on and its `men` row; off = everything above, to the round): the line is first asked as the
// ROUND flies it — a partition, a screen, a wooden door leaf this gun's round crosses with a man known behind it gets a short burst THROUGH it ('thru': at
// most WALLPEN.thruRounds on one sighting, booked as `thru` / `thruShots`, never as a chew), with or without the destruction goal's switch; what the round does
// not cross is chewed as above. So where both are live a partition is fired through, not opened — it still opens under the rounds that cross it, by the
// engine's own count.
// Not here: what the man behind the cover then does, charges, glass.
import { GameSideMap } from '../math/side';
import './condition';
import { Vec3, v3 } from '../math/vec';
import type { Game, Guard } from './game';
import { chaosOf } from './destruction';
import { wallBreakOn, WALL_STATS } from './wallbreak';
import './teams';
import './guards';
import './combat';
import './aim';
import './suppression';
import './civilians';
import { wallMenLive } from './wallpen';
import './glasspen';
import './gunslots';   // (called inside the trigger only)
import './gunroles';   // (called inside the trigger only)
import './silent';
import { crossToRust } from './crossing';

export const COVERFIRE = {
  on: true,
  /** s a sighting of his own stays a place worth rounds · s a mate's eyes-on stays a call · how far a call carries between two men of a side */
  staleSecs: 5, callSecs: 1.2, callM: 14,
  /** a mate's call is good for a PARTITION too (off: for a crate or a stack only — a wall into a room he has not seen the man in is not his to open on a word) */
  callWalls: false,
  /** m from the piece's face to the man's known place, beyond which he is not 'behind it' */
  behindM: { cover: 3, wall: 4 } as Record<string, number>,
  /** pieces of breakable the line may cross before the man · this gun's rounds all of them still want, at most · the reach of it */
  maxLayers: 3, maxRounds: 20, maxM: 30, minM: 1.5,
  /** m from his shoulder to the piece, under which it is not shot at: his muzzle is against it or through it (a round fired there is nobody's) */
  nearM: 1.0,
  /** rounds he will put into a piece that stands: slackK × what it wanted when he started + slack; then it is left alone this long */
  slackK: 1.5, slack: 3, giveSecs: 20,
  /** m off the lane (wider by coneK per metre out) a friend / hostage / civilian holds the round · m past the place a round that got through is followed */
  lane: 0.6, coneK: 0.03, pastM: 8,
  /** the aim point is kept through a newer sighting within this of it · s a chew stays his with his line off it (a step of his own) */
  sameM: 1.5, lingerSecs: 1,
  /** 'he went behind it': the place a man seen moving is taken to be — this many seconds of his pace on from where he was last seen, this far at most */
  leadSecs: 0.8, leadM: 1.5,
  /** the ducked man's point above the floor */
  lowY: 0.55,
  /** a free man's sum: seconds a round (this many a round) against the walk round at this pace */
  roundSecs: 0.25, walkPace: 3, walkAskSecs: 1.5,
  /** the team's line when one of them starts (once per man per saySecs, floor-wide every 4 s); '' = said by nobody */
  line: 'he\'s behind that. going through it.', saySecs: 12,
};
export const COVERFIRE_DEFAULTS = JSON.parse(JSON.stringify(COVERFIRE)) as typeof COVERFIRE;
/** chews started · rounds fired on one · of them stopped at the piece · times a piece being chewed gave · given up (it stood) · rounds held for a friend in or
 *  past the lane · held by the silent order · starts on a mate's call · starts refused: walking is quicker */
export const COVERFIRE_STATS = { chews: 0, shots: 0, landed: 0, opened: 0, gaveUp: 0, heldFriend: 0, heldSilent: 0, calls: 0, walked: 0, opChews: 0, defChews: 0, thru: 0, thruShots: 0 };
export const COVERFIRE_LOG: string[] = [];
/** QA: why a known place was not worth rounds, counted per frame asked (far · clear · door · solid · close · beyond · prop · gone · behind · deep · rounds · call) */
export const COVERFIRE_WHY: Record<string, number> = {};
const ZERO: typeof COVERFIRE_STATS = { chews: 0, shots: 0, landed: 0, opened: 0, gaveUp: 0, heldFriend: 0, heldSilent: 0, calls: 0, walked: 0, opChews: 0, defChews: 0, thru: 0, thruShots: 0 };

type Who = Guard | 'player';
interface Know { who: Who; at: Vec3; /** the floor he stood on · how he was moving over it when seen */ y0: number; v: [number, number]; t: number; /** a mate's call, not his own eyes: good for a crate only, and for the place alone */ call?: boolean }
/** `kt` the sighting it was laid on · `n` rounds since the piece last gave, `fired` in all · `t` the last frame it was his */
interface Chew { id: number; kind: string; who: Who; to: Vec3; hit: Vec3; /** where on the piece the running count began */ h0: Vec3; kt: number; n: number; fired: number; want: number; layers: number; budget: number; fresh: boolean; lay: number; t: number; next: number; k: number; call: boolean; seen: boolean }
interface Man { know: Know[]; chew: Chew | null; gave: Map<number, number>; no: { id: number; until: number } | null; saidT: number }
interface State { /** the last frame ticked: a clock that has gone BACK (a checkpoint load) forgets every man */ lastT: number; men: Map<Guard, Man>; serial: number; base: Record<string, number>; stats: typeof COVERFIRE_STATS; byId: Map<number, { landed: number; gave: number }>; saidT: number; pruneT: number }

const states = new GameSideMap<Game, State>('coverfire.states');
/** the engine's half: walls and cover give under rounds */
const chewLive = (g: Game) => wallBreakOn(g) && chaosOf(g) > 0;
function live(g: Game): boolean {
  if (!COVERFIRE.on || !g.aiEnabled || g.touring) return false;
  return chewLive(g) || wallMenLive(g);   // (wallpen.ts: rounds go THROUGH thin things — a man who knows somebody is behind one has a round for it whether or not walls break)
}




// ---------------------------------------------------------------- the hooks
/** EYES ON, NO LINE (guards.ts alert branch: chest and head both blocked · roomfight.ts tickOp: the same for a teammate under an order): may the round go at
 *  `to` anyway, because what blocks it is breakable and worth the rounds. The caller's own gates, cadence and cone fire it. */
export function coverSeen(g: Game, gd: Guard, who: Who | null, to: Vec3): boolean { return crossToRust('coverSeen', undefined, [g, gd, who, to]) as any; }

/** the run's own numbers (tools/qa/playthrough.ts RunOut.coverfire): the counters, the rounds that landed on pieces that never gave anything (`never`, over
 *  `neverPieces` of them), and the engine's own counts since this game's first frame under the switch (rounds that met a breakable by anybody's gun, what opened) */
export function coverfireReport(g: Game): Record<string, number> {
  const S = states.get(g); const out: Record<string, number> = { on: live(g) ? 1 : 0, ...(S?.stats ?? ZERO), never: 0, neverPieces: 0, pieces: 0 };
  if (S) { for (const r of S.byId.values()) { out.pieces++; if (!r.gave) { out.never += r.landed; out.neverPieces++; } } }
  const base = S?.base ?? {}; for (const k of ['rounds', 'ports', 'holes', 'coverCells', 'coverDead'] as const) out[`wall_${k}`] = (WALL_STATS as Record<string, number>)[k] - (base[k] ?? 0);
  return out;
}
/** QA (tools/qa/coverfire-probes.ts): hand `gd` the record a real sighting of `who` leaves — the man's own place as if seen `age` seconds ago, standing still.
 *  The game never calls it: what a man knows comes from coverfireTick alone. */
export function coverfireSaw(g: Game, gd: Guard, who: Who, age = 0): void { return crossToRust('coverfireSaw', undefined, [g, gd, who, age]) as any; }
/** QA: what `gd` is on this frame (the piece, the rounds he has put in it), null = nothing */
export function coverfireOn(g: Game, gd: Guard): { id: number; kind: string; fired: number; want: number; to: Vec3 } | null {
  const Cw = states.get(g)?.men.get(gd)?.chew; return Cw ? { id: Cw.id, kind: Cw.kind, fired: Cw.fired, want: Cw.want, to: v3.copy(Cw.to) } : null;
}
/** a restart or a checkpoint load (game.ts resetGuards: the roster is new men) / QA: forget everything about this game */
export function coverfireReset(g: Game) { states.delete(g); }
