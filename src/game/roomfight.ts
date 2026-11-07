// THE ROOM FIGHT (Game.fight): both sides of a room clear fighting on their own read — no staged exchange, no plan telling a defender which door to look at.
//
// It works on whoever is on the floor: every living HOSTILE unit standing held (spawnUnit's default: a man put somewhere to hold it) is a DEFENDER, every living
// OPERATIVE unit is a TEAMMATE. Nobody registers anything; a stager spawns men and (for the team) orders them about with squad.ts order(), and this module gives
// them eyes, ears and a trigger. It is inert — a return on the first line of tick() — while no such unit stands, while a show has the floor (the Director's
// programs, the tour: their casts are choreographed and stay so), with the AI switched off, and with ROOMFIGHT.on false (the night breach as its branch had it).
//
// DEFENDERS (tickDef). Each keeps, per ENTRY of the room he stands in (the room = a flood of the nav grid from his feet that stops at doorway cells; its entries =
// the doors on its edge), a decaying score of what HIS OWN senses have put there: a kick, a bang, gunfire (1 each), a rattle or a footfall (0.45), an enemy in his
// sight at it — read off the fight AI's own hearing record and fix (guards.ts hearingCheck: the propagation the mixer uses), never off the level's plan. He watches
// the entry with the most against it (a little sticky, a little shy of an entry his mates already watch), coming round to it a beat late; with nothing against any
// of them he faces the way he was put, and every so often his eyes wander to another door (the untrained man's habit — FIGHT_SKILL.hostile.hold / glance). What he
// watches is where his gun rests (his fix while he has no eyes on anybody), where a blind burst goes when a charge takes his eyes (BLIND, nightbreach.ts — or,
// bangBreak of the time, he ducks instead), and what decides whether a man coming through a door finds him looking: a man who shows well off the line he is
// watching costs him FIGHT_SKILL.hostile.surprise before he starts to line him up. The fight itself is the fight AI's (guards.ts alert branch: its sight test, its
// aim-settle model with this side's settleK / spreadK, its bursts) from where he stands.
//   When the room is lost — he can see enough of its men down (his own nerve, drawn once) with the enemy on him — he gets out through an entry nothing has come at,
// to a spot beyond it, and holds that door from the next room; no such door, he goes to cover (suppression.ts coverFrom). He moves on a script with his own senses
// run under it (an enemy seen close stops him to fight). A man who hears gunfire where he knows his side has men posted, outside his own room, comes to help after
// a few seconds: he is let off his post to the fight AI's own hunt (the pursuit tactics, doors and all) and holds wherever he is when it is over.
//
// TEAMMATES (tickOp). An operative under ANY order (a script — breach.ts's drill, the orders menu's, a probe's) keeps the feet and the sector the order gives him;
// this gives him the decision to shoot. Each frame, every living hostile through the game's own sight test (guards.ts canSeeChar: cone, range, line of sight, and
// now the smoke between two AI men) at the game's own rate (sightGainOn: the light on the man, his motion, the smoke); a man with a real sighting this moment
// (rate over 0.25 — the staff's own licence to shoot) is a target once his meter has passed `notice` (a quiet floor: the shape made out over time) or at once
// while gunfire, a charge or a boot is fresh in HIS ears (the hearing model's horizons — as any roused man shoots on a sighting). The muzzle comes onto him
// (GuardScript.over: an overlay the runner prefers — the order's own aim, carriage and facing are never written and are what he goes back to), the reaction beat
// runs, the track climbs by the aim model's own law (aim.ts settleSecs × this side's settleK) and the rounds go in pairs through fireWeapon while the line is clear
// of statics (clearShot), of Sam and of the other teammate (friendInLine — held, never fired past them). With nobody in his sight, a hostile round fired from where
// he has a line, or the man a mate's muzzle is on, turns his muzzle and light that way after his beat — a look; the sight test then says whether there is a man to
// shoot. Dazzled, staggered or held he has nobody. He calls a contact once, a man down once; stopped in an open doorway with the fight on he steps through and off
// the opening (or back off it), a friend stood in his line he steps round — and the order's own legs are handed back when the reason has gone.
// An operative with NO script is the fight AI's (as the night breach hands a survivor over): all this module adds for him is Sam in his line putting his gun down.
//
// THE GAP BETWEEN THE SIDES is FIGHT_SKILL: one row a side, the same keys. Lethality is not in it (a round that finds a man drops him, both ways); what differs is
// how soon a man starts on a target, how soon he is settled, how wide the unsettled rounds go, whether he holds his angle, and what a bang does to him.
//
// What is asked of the renderer goes through Game.sensors (the smoke between each operative / defender pair in reach, ≤ ROOMFIGHT.pairs segments a frame, keys
// 'fightS<a>_<b>'); headless nothing answers and the air is clear. guards.ts reads it back through pairSmoke() for every AI-on-AI sight test.
// (src/game is one import cycle: nothing below is built from another game module's exports at module load.)
import { men, manOf } from './seats';
import { SideSet } from '../math/side';
import './condition';
import { Vec3, v3, clamp, DEG } from '../math/vec';
import type { Game, Guard, Foe } from './game';
import type { Door } from './doors';
import type { Character } from './character';
import './teampace';   // (read inside methods only)
import { canSeeChar, sightGainOn, doorCells, frameOf, bodyVisibility, BODY_LIGHT, LIGHTS } from './guards';
import { order, endGuardScript } from './squad';
import { fireWeapon } from './combat';
import { squadSkill } from './enemysquads';   // (hard on a squads level: the defenders' row as a drilled garrison has it — the same object otherwise)
import { SUPPRESSION, coverFrom } from './suppression';
import { PICTURE, PICTURE_STATS, exposureAsker, believedOf } from './picture';   // (asked inside breakOff only)
import { isHostile } from './teams';
import './aim';
import { BLIND } from './nightbreach';
import { digInReset } from './digin';
import './wallreact';
import { squadRally } from './enemysquads';
import { PRESSURE, pressStop, run as pressRun } from './pressure';
import { buddyStop } from './buddy';
import './pairorders';
import './workthings';
import './spotted';
import './coverfire';
import { lastStandStop } from './laststand';
import './netquiet';
import './lanes';
import { BELIEF, cover, word } from './belief';
import './glasspen';
import { airTrans } from './weather';
import './clearline';
import './gunwall';
import './gunslots';   // (called inside tickOp only)
import { gunBlindGap } from './gunroles';   // (called inside tickOp only)
import './aimcost';
import './friendlyfire';
import { wg } from './words';   // (a leaf: the dialogue pass's one switch)
import { simRandom } from '../math/dice';   // the simulation's die (math/dice.ts): the Game's own when it has one, Math.random otherwise
import './rails';   // (a spared teammate on his knee stops no defender on the move: asked inside tickMove only)
import { hasHim, OWN } from './owner';
import './silent';
import { RUST_PASS, RUST_SURFACE, goneWithTick } from './crossing';

/** the check's own break word (tools/qa doorshots-check.ts): the doorway rule's switch reads on and the rule does nothing */
const DOORSHOT_BREAK = (globalThis as { process?: { env?: Record<string, string | undefined> } }).process?.env?.DOORSHOTS_BREAK === '1';
export const ROOMFIGHT = {
  /** off = nothing here runs and the night breach is its branch's (staged exchange, plan-forced aims) */
  on: true,
  /** s for a piece of evidence at an entry to fade to a third */
  evidenceSecs: 12,
  /** an entry already being watched keeps this much over a rival (no flicking between two doors) / each mate already on an entry makes it this much less his */
  sticky: 0.25, crowd: 0.2,
  /** below this no entry is feared: he faces the way he was put */
  fearMin: 0.15,
  /** defenders leave a lost room (off = they hold to the last) / go to a fight next door */
  fallBack: true, rally: true,
  /** a man who leaves a lost room by a door never picks a spot outside the BUILDING: one that lies outside the rectangle round every room of the level's
   *  structure (scenarios.roomClear.rooms' bounds) AND has no roof over it (most of five upward rays from head height find nothing — wet.ts's open-sky test).
   *  A door onto the lot is then no way out for him — he takes another door or goes to cover inside. A yard between the rooms (the compound's, the barracks')
   *  is inside the rectangle and stays his to fall back into; roofed floor that is nobody's room (a staging bay, a lobby) is under a roof and stays his too.
   *  The warehouse's dock man fell back through dock_s, was planted in the lot and held the building from outside it. False = as it was. No structure: no rule. */
  fallBackIn: true,
  /** operatives step out of an open doorway they have stopped in / to the side when a friend stands in their line */
  doorway: true, sideStep: true,
  /** a man he has seen, or a point a call / a shot / a footfall turns him to, behind his shoulder while he WALKS: his chest comes round to it and he walks on
   *  backwards / sideways with his gun there (false = as it was: only a man stood still turned; walking, his gun went to its twist limit over one shoulder,
   *  flipped to the other as the point passed behind his back, and he walked on away from the man shooting him) */
  turnWalk: true,
  /** who he shoots first: a man he sees inside `closeM` with his muzzle ON him comes before the man he is already on, and a man he has had no clear round at
   *  for `noShot` s (furniture, a jamb, a leaf in the line) has no hold on him — the next man he sees is his (false = as it was: once on a man he stayed on him
   *  while he could see him, whoever else walked up) */
  nearest: true, closeM: 2.5, noShot: 0.4,
  /** the line of fire at arm's length: a friend's clearance is never more than 0.3 m + a quarter of how far along the line he stands, and one BEYOND a man who
   *  is inside `pointBlank` m counts only if the line itself passes through him (false = as it was: 0.45 m, up to 0.95 for a man on the move, a metre past the
   *  target at any range — in a doorway scrum every gun on the team was down for the mate beside the man they were all on, and he shot them one by one) */
  closeLane: true, pointBlank: 2,
  /** the player standing PAST the man a teammate is on is owed his own breadth and a fifth of how far past he stands, not the whole lane (the whole lane —
   *  0.45 m, 0.95 when he moves — is still his anywhere BETWEEN the gun and its man). false = as it was: with the player a step beyond a hostile, every gun on
   *  the team went to the low ready, the weapon light went down with it, and the man they could no longer see shot them */
  samBehind: true,
  /** the player LEVEL WITH OR BEHIND the muzzle is not in the line at all, and the lane opens from his own breadth at the muzzle to the whole lane a stride in
   *  front of it (0.3 m + a metre per metre ahead). false = as it was: for a man behind the muzzle the lane was measured from the muzzle POINT, so the player
   *  walking shoulder to shoulder with a teammate — inside 0.45 m of his gun, 0.95 on the move — held it down whichever way it pointed, the weapon light went
   *  down with it, and the man it had just found shot the pair from the dark */
  samBeside: true,
  /** the lane the player is owed widens with how fast he is really TRAVELLING (the ground his root has covered, eased over a quarter of a second), never more than the speed he is
   *  asking for. false = as it was: the speed he ASKED for alone — a man shuffling on the spot, or walking into a wall with a key held, counted as on the move
   *  at full pace, so 0.9 m off a line he still stood 'in' it: in a corridor every gun behind him stayed down and the man down the far end shot at them */
  samTravel: true,
  /** s a teammate WITH SOMEWHERE TO BE (walking on an order: following, to a stack, to a mark) keeps his gun on a man he has no round at — glass, a jamb,
   *  furniture in the line — before he says so, lets him go for `noShotPass` s and walks on with his gun where his order has it; a man stood on his post keeps
   *  him (that is what he is there for), and the moment there IS a round at the man he is his again. 0 = as it was: he walked on backwards with his muzzle (and, once
   *  the lights were free, his light) on a man behind a pane for as long as he could see him — with night vision, the whole length of a corridor */
  noShotGo: 1.5, noShotPass: 4,
  /** a man he was on, out of his sight for under a second and a half (a door leaf swung across, a jamb passed) and back in it: his track is where it was
   *  less what the gap cost, not zero, and the beat to take him up again is as short as the gap was. false = as it was: any break in sight, however short, and he lined the same man up from nothing — a man he had
   *  followed through a pane to the room's door walked out of it and shot him first */
  keepTrack: true,
  /** how readily a defender goes to a fight next door (`rally`): × his side's chance to go each time he weighs it (FIGHT_SKILL rallyChance, 0.75 a roll). 1 = as it
   *  was; under 1 more of them are still in their own rooms when the team gets there; 0 = nobody leaves (the `rally` switch off). The dice are drawn either way */
  rallyK: 1,
  /** a teammate hears an ENEMY's footfall (open air, the hearing model's rule for a step) — and the nearest man to it with a line looks that way (off = steps
   *  reach nobody's ears on the team, as it was: a man coming up a corridor behind a stack was seen only by whoever happened to face him) */
  stepEars: true,
  /** a man HOLDING an arc he was given (orders.ts holdArcs: a watch point, on his mark) answers a mate's call or a footfall only inside this many degrees of it
   *  — the other half is the other man's; 180 = as it was (he turned to any call he had a line to, and the two of them ended facing one way) */
  arcHalfDeg: 100,
  /** A MAN TOLD TO COVER KEEPS HIS ANGLE (the maintainer, of a man covering a lane who swung a quarter turn to a man two mates were already shooting, the
   *  lane's own men still coming: "doesn't watch his angle even though red team has it covered"). With `coverKeep` a man stood at the post of a cover order
   *  passes over an enemy who is more than `coverHalfDeg` off the way he was told to watch WHILE a mate's gun is on that man with a round at him: the man is
   *  seen (his meter runs) and is not his — not to his eyes, and no call, shot or footfall of that man's turns his look. The man is his again the moment no
   *  mate has him (the mate is behind a jamb, reloading, has a friend in his line or his muzzle down), his muzzle comes onto this man, or he is inside
   *  `coverNearM`; a man he is already on he hands over only when the mate is past his own beat to fire. A look he had begun at such a man is dropped.
   *  Not covered: coverfire.ts's rounds into cover at a man a mate knows of. Off = as it was: his eyes took any man they saw. */
  coverKeep: false, coverHalfDeg: 45, coverNearM: 6,
  /** s a teammate stays ROUSED after a shot, a charge or a boot last reached his ears: a sighting alone is a man to him (no making the shape out first) and he
   *  reads at the roused rate. 10 = as it was — a quarter of a minute after a room was taken the team looked as calm men look, and a defender walking in through
   *  a connecting door two metres off had three tenths of a second on the man watching it */
  teamHot: 45,
  /** a teammate already coming round to look for a man (a mate's call, a round, a footfall: cue) is not SURPRISED by him when his eyes find him, wherever his
   *  muzzle had got to (off = as it was: the surprise beat whenever his gun was not yet within the sector) */
  cued: true,
  /** s a teammate holds off a man for a friend in his line, with nowhere to step, before a hostile's round from somewhere else takes his eyes: he lets the
   *  masked man go for a couple of seconds and turns to the one shooting (0 = never, as it was: he stood on the masked man until he was shot from behind) */
  letGo: 0.8,
  /** the man whose look a footfall is, with NO line to the walker (a shut door, a corner), brings his gun onto where the sound comes from — the footfall's own
   *  spot through a leaf, the doorway or corner it comes round otherwise — and holds it there while the steps keep coming, `stepLookSecs` at most a look; a shot or a mate's call takes
   *  the look over, and a man on his way to a door's mark or through it keeps his corner (off = as it was: he looked only for a man he already had a line
   *  to, and boots running up to a shut door two metres behind him turned nobody) */
  stepLook: true, stepLookSecs: 5,
  /** a door an ENEMY's hand opens (unlatched and swung: never a rattle, a pick, a leaf slammed to): its noise within 9 m — by path and loudness as a footfall is
   *  heard, at a lower bar: a leaf moving in his sight is seen as much as heard — turns EVERY teammate with a line to that doorway onto it for `doorLookSecs`
   *  (the steps' look is one man's; a door coming open onto the floor they stand on is every man's who can see it), each after his own beat, once per opening.
   *  A shot or a mate's call takes the look over; a man on his way to a door's mark or through it keeps his corner; a man holding a dealt arc answers only
   *  inside it; a gun an order has on ANOTHER door (a stack's, a threshold's jamb man) keeps it while a freer man can look — with none, the one of them with
   *  the least turn looks (off = as it was: nobody turned for a door, and men in a taken room were shot in the back through one they had heard pulled open
   *  three seconds before). KNOWN: a look lights his weapon light like any turn, and a man an entry left on a room's OTHER door is free by this rule */
  doorEars: true, doorLookSecs: 3.5,
  /** ROUNDS THROUGH A DOORWAY KEEP HIS GUN ON IT (the maintainer, on a replay of a stack on a door an enemy had opened: "enemy was immediately in front of
   *  them and shooting, nothing happened"). A hostile's round turned a teammate only when he had a line to the SHOOTER; a man firing through an open
   *  doorway, tucked behind its frame, turned nobody, and the stack's rear men stood with their guns the other way a metre and a half from his muzzle (the one
   *  who had looked for the door coming open looked away again after `doorLookSecs`). With `doorShots` a round that FLEW THROUGH an open doorway towards his side of its wall (the
   *  shot's own muzzle and stop, fired within `doorShotM` of the frame), by a man he has no line to, turns every teammate with a line to that doorway onto it, each after his own beat, for `doorLookSecs`
   *  after the LAST such round: the gun is there when the man steps into the opening, and the sighting is a cued one. A shooter in his line takes the look
   *  over, as ever, and so do a mate's call, a footfall and a door just opened; a man on his way to a door's mark keeps his corner; a man holding a dealt arc
   *  answers only inside it; a gun an order has on another door keeps that door. Nothing is fired at a noise: whether
   *  there is a man to shoot stays his eyes'. ON in the page (?doorshots=0), off here and on the frozen stand (TPS_DOORSHOTS=1) */
  doorShots: false, doorShotM: 8,
  /** a teammate in the fight whose line to his man (or to what he is looking for) runs through SAM lowers his muzzle until Sam is out of it — he was already
   *  holding his fire; now he also takes the gun off him (off = as it was: held, with the gun up on Sam's back) */
  samLine: true,
  /** the feet half of guards.ts LIGHTS.litMan (that switch is this rule's too — see there): a teammate WALKING on an order who stands in the dark on a man
   *  standing in the place's light, with SAM in his line to him — his gun is down for it (samLine) — stops where he is until the line is clear, this many
   *  seconds at most a hold (0 = his feet are never held). Only a walk that closes on the man (the leg he is on ends nearer the man than he stands: a word that pulls
   *  him back is walked), never while he stands in an open doorway's funnel, and only for the player: a mate in the line is the side step's, and moves on by
   *  himself. Only from the dark, by measurement: the same hold by a man whose light was on
   *  stood him still in front of an aimed enemy and lost both teammates most of the time */
  holdShortSecs: 4,
  /** a moving defender stops to fight an enemy seen inside this many metres */
  moveEngageM: 7,
  /** the light read (0‥1, as a meter reads) for a man whose lit weapon light has the watching eye inside its cone: the lens is the brightest thing in view */
  lamp: 0.7,
  /** a man who shows within this many degrees of where a raised gun already points is shot at from that side's `preaim` track, not from nothing */
  preaimDeg: 12,
  /** EYES ON THE SIGHTS (rebuild/four-man-entries/fix.md, found tracing the fours' men shot by a man they were facing and never saw): a man's eye is 58° either
   *  side of where chest and head point BETWEEN them (guards.ts canSeeChar) — and with a long gun in the shoulder the chest is bladed some 80° off the bore and
   *  the head laid 45° down on the stock, so that mean looks 42‥63° off the gun (measured, both sides): what his sights are on sits at the very edge of what he
   *  sees or outside it, past the near field anything to the gun side of it is invisible, and the door a man covers is the door he is blindest to. On: while his
   *  long gun is in the shoulder and level along his aim (the bore inside 30° of it in plan), an AI man ALSO sees other AI men through the same eye laid along
   *  his aim — a cone of `sightsDeg` either side of it, and the 100° near field inside 3.5 m: `sights` the teammates (ON), `sightsFoe` every other side's men
   *  (off: measured, it costs the team more men than `sights` finds). `sightsDeg` 45 keeps that cone INSIDE the 50° sector his gun is taken to watch
   *  (FIGHT_SKILL.operative.sectorDeg): at the eye's own 58° a man who showed in the 50‥58° band while the gun was still swinging round to him was a SURPRISE
   *  (the whole beat owed) where two frames later he would have been a man in his sector — seen sooner, shot later; that lost one office rescue of 48 and one registry rescue of 48 from the threshold,
   *  and 45 has them back. What he saw before he still sees; Sam's reading by anybody's eye is untouched (the stealth game as it was); only while the room fight is live
   *  (operative or hostile units on the floor: every scenario, never the night game, a tour or a showcase). A gun lowered for a friend in the line is not in
   *  the shoulder: his eye is the old one until it is back up. The numbers are in fix.md. */
  sights: true, sightsFoe: false, sightsDeg: 45,
};

/** One side's skill. The same keys both rows; what a key does is the same thing for both. */
export interface FightSkill {
  /** s from 'that is a man' to starting to line him up (drawn in the range) */
  react: [number, number];
  /** × react when the man shows where this man was already looking (inside sectorDeg of his watch); s MORE when he shows further off it than that */
  ready: number; surprise: [number, number]; sectorDeg: number;
  /** × the aim model's settle time (aim.ts AIM.guard.settleSecs) and × the weapon row's cone: under 1 = quicker and tighter than the night staff */
  settleK: number; spreadK: number;
  /** the track (0‥1; a round goes at aim.ts acquireMin 0.55) he starts from on a man who shows where his raised gun already points: the angle he was holding */
  preaim: number;
  /** the sight meter (0‥1, filled at the game's sight-gain rate) at which a shape is a man to shoot — the operatives' overlay; the fight AI's men use its own 0.85 */
  notice: number;
  /** s between the two rounds of a pair, and from a pair to the next look / from one man down to the next */
  pair: number; shift: [number, number];
  /** s he holds an angle before his eyes wander, and how long they stay away (Infinity = he holds it) */
  hold: [number, number]; glance: [number, number];
  /** rad/s he comes round to a new thing to watch while calm */
  turn: number;
  /** chance a flash makes him duck for its length instead of firing blind down what he watched */
  bangBreak: number;
  /** s he flinches when a door of the room he holds is BOOTED in (no round from him meanwhile, wherever his gun was): the beat a boot buys the men behind it —
   *  and what a can thrown in on the boot is timed into. A door pushed open buys nothing */
  startle: [number, number];
  /** s after a flash has cleared from his eyes before he is himself again: until then any man who shows is a surprise to him, wherever he was looking — the
   *  angle he held is no head start (no `ready`, no `preaim`) */
  daze: [number, number];
  /** the share of his room's men he must see down, with the enemy on him, before he gives the room up (drawn per man; over 1 = never) */
  nerve: [number, number];
  /** s of a fight heard where his side has men, outside his room, before he goes; the chance he goes at all */
  rally: [number, number]; rallyChance: number;
  /** s stood in an open doorway with the fight on before he steps off it; s a friend stands in his line before he steps round */
  funnel: number; line: number;
}
export const FIGHT_SKILL: Record<'operative' | 'hostile', FightSkill> = {
  // the team: quick onto a man, settled almost at once, pairs; holds the sector it was given; a bang or a man down moves nobody. (The room clear tally,
  // tools/qa/roomclear-tally.ts: surprise was [0.1, 0.2] and sectorDeg 60 — a teammate with no target of his own, met from outside the arc his gun is on,
  // barely lost a beat, so a pair that covered nothing lost little by it; now such a man is a third to six tenths of a second later onto whoever comes.
  // A teammate already in a fight pays `shift` as before, not this.)
  operative: { react: [0.12, 0.22], ready: 1, surprise: [0.35, 0.6], sectorDeg: 50, settleK: 0.4, spreadK: 0.5, preaim: 0.6, notice: 0.12, pair: 0.16, shift: [0.2, 0.3],
    hold: [Infinity, Infinity], glance: [0, 0], turn: 8, bangBreak: 0, startle: [0, 0], daze: [0, 0], nerve: [2, 2], rally: [0, 0], rallyChance: 0, funnel: 0.35, line: 0.5 },
  // the men in the room: slow to start and to settle, wide until settled, eyes that wander, a bang can break them, half the room down and they go. (The
  // tally: spreadK was 1.4 — a defender who had the drop on a teammate still missed too often for it to cost the team; at 1 his hands are an ordinary
  // guard's. His own reaction and settle numbers are unchanged.)
  hostile: { react: [0.4, 1.1], ready: 0.25, surprise: [0.5, 0.9], sectorDeg: 40, settleK: 1.5, spreadK: 1.0, preaim: 0.55, notice: 0.85, pair: 0.1, shift: [0.4, 0.9],
    hold: [5, 11], glance: [1.2, 2.6], turn: 3, bangBreak: 0.3, startle: [0.4, 0.65], daze: [1, 2], nerve: [0.34, 0.7], rally: [1.5, 4], rallyChance: 0.75, funnel: Infinity, line: Infinity },
};

/** DRILLED DEFENDERS — hard on the night breach (difficulty.ts HARD.drilled → pressure.ts PRESSURE.drilled; record
 *  rebuild/hard-everywhere/round2-night-breach.md). One room, three men, no warning: the canister blinds all three and the room is taken in
 *  two seconds, so nothing a defender could do smarter comes before he is shot. What is left is how a man takes a bang, and that is training — nerves only, no
 *  reaction, aim or hit-point number is in it. A drilled man is blind exactly as long as anybody (combat.ts FLASH.secs, untouched) and breaks as often
 *  (bangBreak, untouched); he is HALF AS SHAKEN — `shaken` × the seconds a stun canister doubles him over (FLASH.stagger 2.5‥1.0 s → 1.25‥0.5: combat.ts
 *  detonateStun asks `drilledK`), × his flinch at a boot (startle 0.4‥0.65 s → 0.2‥0.33) and × the daze once his eyes clear (daze 1‥2 s → 0.5‥1).
 *  So his blind burst — the room fight's own, down the door HE WAS WATCHING, twelve turns from three tenths after the flash — is partly fired instead of
 *  lost whole to the seconds he spent doubled over, and it starts AFTER the entry's own beat of silence (doortools.ts burstDry: 0.6 s) has sent the lead in:
 *  that is the bite. The first of them to fire shouts it. A breaching charge folds him as long as anybody (doortools.ts staggerSecs, untouched: still no
 *  burst back through that doorway), and the door worked from the threshold is the other answer. */
export const DRILLED = { shaken: 0.5 };
/** hard's switch, and only the night breach by itself (a layout's rooms — the room clear — and the rescues are other rules' ground). Under the press's own
 *  switch like every row of its table: with harder defenders off nothing here runs */
const drilledOn = (g: Game) => { const nb = g.nightBreach; return PRESSURE.on && PRESSURE.drilled && !!nb?.active && !nb.layout; };
/** × the seconds a stun canister doubles this man over (combat.ts detonateStun): DRILLED.shaken for a drilled defender, 1 for anybody else */
export function drilledK(g: Game, gd: Guard): number { return gd.team === 'hostile' && drilledOn(g) ? DRILLED.shaken : 1; }

interface Entry { door: Door; at: Vec3; out: [number, number] }
interface Room { id: number; cells: Set<number>; entries: Entry[] }
const ROOM_MAX_CELLS = 2400;   // 600 m² of half-metre cells: past this it is open floor, not a room (the fill stops; its entries are whatever doors it met)
const ROOM_DX = [1, -1, 0, 0], ROOM_DZ = [0, 0, 1, -1];   // the fill's four neighbours, in its order

type DefMode = 'hold' | 'move' | 'free';
interface Def {
  gd: Guard; station: Vec3; home: Vec3; room: Room | null; pinned0: boolean;
  /** the lesson's script has him in hand for a staged moment (tickDef): this module does nothing with him until it lets him go */
  staged?: boolean;
  /** evidence per entry of `room`; the entry watched (−1 = home), the one he is coming round to and when, the wandering eye */
  ev: number[]; watch: number; want: number; wantAt: number; glanceTo: number; glanceUntil: number; glanceNext: number;
  /** what he watches, on the floor — the ONE object written into his fix (lastKnown === pt ⇒ the fix is this module's, anything else is his senses'); with BELIEF on it is what he covers and is never in his fix */
  pt: Vec3;
  heardT: number; cnt: { shots: number; kick: number; bang: number; small: number }; noiseAt: Vec3 | null; noiseT: number; noiseOut: boolean;
  lastState: string; lastSeen: number; fixKeep: number; pre: boolean;
  nerve: number; lostAt: number; movedAt: number;
  rallyAt: number; rallyGo: boolean;
  mode: DefMode; moveKind: 'fallback' | 'cover'; moveT: number; cameBy: Door | null; stallAt: Vec3; stallT: number; freeAt: number;
  burst: { until: number; n: number; next: number; broke: boolean; daze: number };
  /** startle.ts (bolt): he is running FROM a man in his eyes — until this time that man does not stop him to fight (−1: never set) */
  boltUntil: number; boltFrom: Foe;
}
interface Meter { m: number; seenT: number; /** when he last held a round at this man for the standing glass in the way (silent.ts silentHoldsGlass; unset = never) */ glassT?: number }
interface Op {
  gd: Guard; meters: Map<Guard, Meter>;
  tgt: Guard | null; turn: { to: Guard; at: number; until: number; pt?: Vec3 | null; t0?: number; door?: Door | null } | null; pass: { h: Guard; until: number; noShot?: boolean } | null; kept: { h: Guard; track: number; t: number } | null; track: number; pre: boolean; readyAt: number; n: number; next: number; lostAt: number; fell?: boolean; first: boolean;
  /** the overlay handed to his script while his muzzle is on what HE has seen (GuardScript.over — squad.ts aims at ov.aimAt === aim with the gun up and squares
   *  him to ov.face); the order's own aimAt / upper / stance / face are never written, so whatever is ordered under it is what he goes back to */
  aim: Vec3; ov: { aimAt: Vec3; face: number | null; walk?: boolean };
  /** a step this module ordered (off a doorway, round a friend): its goal object, and the order's own legs to hand back when the reason has gone */
  step: { mine: Vec3; goal: Vec3 | null; speed: number | undefined; strafe: boolean | undefined; face: number | null | undefined; door: boolean } | null;
  /** when a shot (level ≥ 1), a charge or a boot last reached HIS ears (the hearing model's own horizons): for ROOMFIGHT.teamHot seconds after he looks as a roused man looks */
  hotT: number;
  /** when an ENEMY's footfall last reached his ears along open air (the hearing model's own rule for a step: guards.ts hearingCheck; one through a shut door
   *  or a wall is not it) — a man holding in the dark puts his weapon light on for it (guards.ts LIGHTS.stepSecs) — and whose it was when he was the teammate
   *  NEAREST it: with a line to that man and nobody in his sight he brings his muzzle round to look (cue), as he does for a shot. Whether there is a man there
   *  to shoot stays his eyes' */
  stepT: number; stepBy: { by: Guard; t: number; at: Vec3 } | null;
  /** ROOMFIGHT.doorEars: the door an enemy's hand has just opened where he heard it and has a line to its doorway — who worked it, when, the doorway point he looks at */
  doorBy: { by: Guard; t: number; at: Vec3; door: Door } | null;
  funnelT: number; lineT: number; /** who the round was last held for, and when (showwhy.ts reads it: the fire gate's own answer, lane or cone) */ lineBy: Guard | 'player' | null; lineAt: number; /** seconds on his man with his gun in a wall (gunwall.ts) */ wallT: number; stepUntil: number; reloadUntil: number; saidT: number;
  rounds: number; heldForFriend: number; heldForLine: number; blockT: number;
  /** the feet his hands still remember (aimcost.ts `feetHold`; only fed under AIMCOST.move) */
  feetSm?: number;
  /** when `feetSm` was last fed (game clock): a tick skipped lets go of that much more */
  feetT?: number;
  /** holdShort (LIGHTS.litMan): when Sam last stood in his line to the man he is on while he walked · when that began (a gap of a second starts it afresh) · the
   *  last frame his feet were held for it (orders.ts asks: short()) */
  shortSeen: number; shortSince: number; shortT: number;
}
export interface FightNote { t: number; who: string; what: string }

const flat = (a: Vec3, b: Vec3) => Math.hypot(a[0] - b[0], a[2] - b[2]);
const pick = (r: [number, number]) => r[0] + simRandom() * (r[1] - r[0]);
const eyeOf = (c: Character): Vec3 => c.bones.head ?? v3.add(c.pos, [0, 1.65, 0]);
const pairKey = (a: number, b: number) => a < b ? `fightS${a}_${b}` : `fightS${b}_${a}`;

/** The smoke between two AI men as the renderer last answered it (1 = clear air: no answer yet, headless, or the pair not in reach of each other) — what
 *  guards.ts canSeeChar / sightGainOn put between an AI eye and an AI man, as Guard.smokeTrans stands between a guard's eye and Sam. */
export function pairSmoke(g: Game, a: Character, b: Character): number { { const __s = RUST_SURFACE; if (__s) { const __r = __s.call('pairSmoke', undefined, [g, a, b]); if (__r !== RUST_PASS) return __r as any; } }
  const T = g.fight?.trans.get(pairKey(a.id, b.id)) ?? 1;
  // (a picked weather's FOG stands between them as smoke does — weather.ts airTrans, chest to chest; exactly 1 with no kind picked: T as it was)
  return g.weather?.fog ? T * airTrans(g, a.bones.chest ?? v3.add(a.pos, [0, 1.1, 0]), b.bones.chest ?? v3.add(b.pos, [0, 1.1, 0])) : T;
}
/** A lit weapon light gives its carrier away to the eye it points at: the light to read for `o` (0‥1) when `eye` stands inside the cone of his burning light, in
 *  its range — the lens in the picture — else 0. guards.ts sightGainOn takes it when it beats the light ON him; only while the room fight is live (elsewhere 0:
 *  the showcases' casts are seen exactly as before). The line of sight is canSeeChar's, asked before this. */
export function lampGlare(g: Game, eye: Character, o: Guard): number {
  if (!g.fight?.live) return 0;
  const L = o.flashlight; if (!L || !L.enabled || !(L.intensity > 0) || o.state === 'dead') return 0;
  const e = eyeOf(eye); const dx = e[0] - L.pos[0], dy = e[1] - L.pos[1], dz = e[2] - L.pos[2]; const len = Math.hypot(dx, dy, dz);
  if (len > L.range || len < 1e-3) return 0;
  return (dx * L.dir[0] + dy * L.dir[1] + dz * L.dir[2]) / len >= Math.cos(L.outerDeg * DEG) ? ROOMFIGHT.lamp : 0;
}
/** Is the room fight running this frame (units of it on the floor, no show, the AI on)? guards.ts asks before letting a defender hear an operative at a door. */
export function fightLive(g: Game): boolean { return !!g.fight?.live; }
/** ROOMFIGHT.closeLane: the clearance a friend is owed against a line of fire `L` long, he `t` of the way along it (roomfight's own gate and, for the team in a
 *  room fight, guards.ts colleagueInLine) */
export function closeLane(clear: number, t: number, L: number): number {
  if (!ROOMFIGHT.closeLane) return clear;
  return Math.min(clear, t > 1 && L < ROOMFIGHT.pointBlank ? 0.3 : 0.3 + 0.25 * Math.max(0, t) * L);
}

export class RoomFight {
  readonly knobs = ROOMFIGHT; readonly skill = FIGHT_SKILL;
  /** running this frame (set by tick) */
  live = false;
  /** blind fire for dazzled defenders (a stager's knob: the night breach writes its own here at each staging) */
  blindFire = true; blindRounds = 0;
  /** smoke transmittance per pair key, as last read */
  trans = new Map<string, number>();
  defs: Def[] = []; ops: Op[] = [];
  log: FightNote[] = [];
  stats = { opRounds: 0, heldForFriend: 0, fellBack: 0, /** fallBackIn: spots beyond a door passed over for lying outside the building */ fallBackOut: 0, tookCover: 0, rallied: 0, surprised: 0, broke: 0, stepped: 0, startled: 0 };
  private rooms: Room[] = []; private roomOfCell = new Map<number, Room>(); private fillMark = new Uint8Array(0);
  private called = new Set<Guard>(); private downCalled = new Set<Guard>();
  private rr = 0;
  /** a STAGER'S: 'the floor is roused as of t' (a probe's even meeting; nothing in the module raises it — a teammate's own ears do, Op.hotT). For ten seconds after
   *  either, a teammate looks as a hunting man looks (the sight gain's ×2.2, a sighting enough to shoot on — he has no 'alert' of his own under a script) */
  hotT = -100;
  /** hostile rounds of the last second: who fired, when (a teammate's cue to look) */
  private reports: { by: Guard; t: number; from?: Vec3; to?: Vec3 }[] = []; private heardEv = new SideSet<object>();
  /** how fast the player is really getting anywhere (ROOMFIGHT.samTravel): each frame's displacement of his root, eased over about a quarter second — a
   *  shuffle on the spot nets nothing, a walk into a wall is no walk. `samFor` = s of frames behind the figure (under a quarter second: not known yet) */
  private sams: { x: number; z: number; t: number; v: [number, number]; known: number }[] = [];   // (a row a player, by seat)
  private samSpeedOf(ch: Character): number { const g = this.g, S = g.seats; for (let i = 0; i < S.length; i++) if (S[i].view.player.char === ch) { const T = this.sams[i]; return !T || T.known < 0.25 ? Infinity : Math.hypot(T.v[0], T.v[1]); } return Infinity; }   // (not known yet: the pace he asks for stands)
  constructor(private g: Game) { }

  /** may the module run at all: the switch, the AI, sides, and no show holding the floor */
  get on(): boolean {
    const g = this.g;
    return ROOMFIGHT.on && g.aiEnabled && g.tune.teams && !g.touring && !g.quietUtility && !g.puppet && !(g.director?.run && !g.director.run.ended && !g.rails?.live);   // (a tutorial mission's own program — rails.ts — runs beside the fight, it does not hold the floor)
  }
  private note(gd: Guard | null, what: string) { this.log.push({ t: this.g.time, who: gd ? gd.callsign : '-', what }); if (this.log.length > 600) this.log.splice(0, 200); }

  // ================================================================ what the other parts call
  /** an operative's muzzle is on a man (or coming onto him) — breach.ts: his sweep waits */
  engaged(gd: Guard): boolean { return !!this.ops.find(o => o.gd === gd)?.tgt; }
  /** seconds he has been ON his man with no round at him from where he stands — a jamb across his bore, his gun in a wall (blockT) —: doortools.ts thrPie
   *  asks it of a man on a cut, who brings the gun round after the eye */
  noShot(gd: Guard): number { const O = this.ops.find(o => o.gd === gd); return O?.tgt ? O.blockT : 0; }   // (no man, no count: blockT is not run down once he has let the man go)
  /** an operative is on a man, turning to one, or lost one inside the last `secs` — rescueflow.ts: not the moment to hand him his next errand */
  busy(gd: Guard, secs: number): boolean { return goneWithTick('RoomFight.busy') as any; }
  /** an operative has a living hostile in his sight right now (target or not) */
  contact(gd: Guard): boolean { return goneWithTick('RoomFight.contact') as any; }
  /** does any living teammate see this man right now — the scripted ones by this module's meters, one on the fight AI by its own eyes (the fog of war's question) */
  teamSees(h: Guard): boolean { return this.seenBy(h).length > 0; }
  seenBy(h: Guard): Guard[] {
    const t = this.g.time, out: Guard[] = [];
    for (const O of this.ops) { if (O.gd.state === 'dead') continue; const M = O.meters.get(h); if ((M && t - M.seenT < 0.3) || (!O.gd.script && O.gd.foe === h && t - O.gd.lastSeenT < 0.3)) out.push(O.gd); }
    return out;
  }
  /** THE sight test between two AI men, in one call (cone, range, line of sight, smoke; then the sight-gain rate off the light on the man): `sighting` is the
   *  game's own line for a real one (rate over 0.25). Stateless — anybody may ask it of any pair. */
  sightOn(viewer: Guard, target: Guard): { visible: boolean; rate: number; dist: number; sighting: boolean } { { const __s = RUST_SURFACE; if (__s) { const __r = __s.call('RoomFight.sightOn', this, [viewer, target]); if (__r !== RUST_PASS) return __r as any; } }
    const vis = canSeeChar(this.g, viewer, target.char, false); const known = (this.ops.find(o => o.gd === viewer)?.meters.get(target)?.m ?? 0) >= FIGHT_SKILL.operative.notice;
    const rate = vis.visible ? this.rateOn(viewer, target, vis.dist, known) : 0;
    return { visible: vis.visible, rate, dist: vis.dist, sighting: rate > 0.25 };
  }
  /** guards.ts sightGainOn, with the hunting man's ×2.2 for a scripted man (his state is his script's, never 'alert') while the fight is hot or once he has
   *  made this man out (`known`: his meter past notice) — the staff's own order of things: a glimpse fills the meter, the roused eye then holds him */
  private rateOn(viewer: Guard, target: Guard, dist: number, known = false): number {
    const r = sightGainOn(this.g, viewer, target, dist);
    return viewer.state !== 'alert' && viewer.state !== 'search' && (known || this.g.time - this.hotOf(viewer) < (viewer.team === 'operative' ? ROOMFIGHT.teamHot : 10)) ? r * 2.2 : r;
  }
  /** is the floor HOT in this teammate's ears — a shot, a charge or a boot inside `teamHot` (doortools.ts: a door worked on a hot floor gets a can first) */
  hotFor(gd: Guard): boolean { return this.g.time - this.hotOf(gd) < ROOMFIGHT.teamHot; }
  hotOf(gd: Guard): number { const O = this.ops.find(o => o.gd === gd); return O ? Math.max(O.hotT, this.hotT) : this.hotT; }
  /** where a defender is watching (the floor point), which entry that is (null = the way he was put), and his mode — the probes', the panel's */
  watchOf(gd: Guard): { at: Vec3; door: string | null; settled: string | null; glancing: boolean; mode: DefMode } | null {
    const D = this.defs.find(d => d.gd === gd); if (!D) return null;
    const glancing = this.g.time < D.glanceUntil; const name = (e: number) => e >= 0 && D.room ? D.room.entries[e]?.door.def.name ?? null : null;
    return { at: v3.copy(D.pt), door: name(glancing ? D.glanceTo : D.watch), settled: name(D.watch), glancing, mode: D.mode };   // door = where his eyes are this moment (a glance included); settled = the entry he has decided to fear
  }
  opOf(gd: Guard): Op | null { return this.ops.find(o => o.gd === gd) ?? null; }
  /** LIGHTS.litMan's feet half: this teammate's feet are held this frame — in the dark on a lit man, Sam in his line to him (orders.ts: his order's leg waits, its clocks with it) */
  short(gd: Guard): boolean { if (!LIGHTS.litMan || !this.live) return false; const O = this.opOf(gd); return !!O && this.g.time - O.shortT >= 0 && this.g.time - O.shortT < 0.25; }

  private stop() {
    pressStop(this.g); buddyStop(this.g); lastStandStop(this.g);
    for (const O of this.ops) { this.restore(O); this.handBack(O); }
    for (const D of this.defs) {   // every man as this module found him: a mover's script ended (nobody is left walking on legs nothing watches), the pin it put on taken off
      if (D.mode === 'move' && D.gd.script && D.gd.state !== 'dead') endGuardScript(D.gd, 'alert');
      if (D.mode === 'free' && D.gd.state !== 'dead') D.gd.hold = true;
      D.gd.pinned = D.pinned0;
    }
    for (const x of [...this.defs.map(d => d.gd), ...this.ops.map(o => o.gd)]) x.skill = null;
    for (const k of this.trans.keys()) this.g.sensors.forget(k);
    this.live = false; this.defs = []; this.ops = []; this.trans.clear(); this.called.clear(); this.downCalled.clear(); this.blindRounds = 0; this.hotT = -100; this.reports = [];
    digInReset(this.g);                                    // digin.ts: wedged doors get their own locks back (nothing to do, and nothing done, unless a room dug in)
  }
  /** a fresh staging: everything this module remembers about the last one dropped (the stager calls it; a restart that swaps every Guard object does it anyway) */
  reset() { this.stop(); this.blindFire = true; this.drillSaid = false; this.log.length = 0; for (const k of Object.keys(this.stats) as (keyof RoomFight['stats'])[]) this.stats[k] = 0; }


  // ================================================================ rooms
  /** the room a point stands in: the nav cells reachable from it without crossing a doorway, and the doors on its edge (cached: neither moves) */
  roomAt(p: Vec3): Room | null { { const __s = RUST_SURFACE; if (__s) { const __r = __s.call('RoomFight.roomAt', this, [p]); if (__r !== RUST_PASS) return __r as any; } }
    const g = this.g, nav = g.col.nav, cs = nav.cell, W = nav.W, H = nav.H; const dc = doorCells(g);
    const gap = g.destr?.walls.shellGaps;   // (a breach in an OUTER wall is a doorway with no door: the flood stops at its cells — wallbreak.ts shellGaps; empty unless one was blown)
    const free = (x: number, z: number) => x >= 0 && z >= 0 && x < W && z < H && nav.blocked[nav.idx(x, z)] !== 1 && !dc.has(nav.idx(x, z)) && !gap?.has(nav.idx(x, z));
    let sx = Math.floor(p[0] / cs), sz = Math.floor(p[2] / cs);
    if (!free(sx, sz)) { let found = false; for (let r = 1; r <= 3 && !found; r++) for (let dz = -r; dz <= r && !found; dz++) for (let dx = -r; dx <= r && !found; dx++) if (free(sx + dx, sz + dz)) { sx += dx; sz += dz; found = true; } if (!found) return null; }
    const hit = this.roomOfCell.get(nav.idx(sx, sz)); if (hit) return hit;
    const cells = new Set<number>([nav.idx(sx, sz)]); const q = [nav.idx(sx, sz)]; const touched = new Map<Door, number>();
    // (`mark` says what `cells` holds — every cell goes into the set, the queue and the marks together — so the fill asks a byte, not the set, four times a cell;
    //  wiped by the queue as the fill ends. A wall blown open floods a room afresh on the frame it goes: wallreact.ts reflood.)
    let mark = this.fillMark; if (mark.length !== W * H) mark = this.fillMark = new Uint8Array(W * H); mark[q[0]] = 1;
    let merge: Room | null = null;
    for (let qi = 0; qi < q.length && cells.size < ROOM_MAX_CELLS && !merge; qi++) {
      const c = q[qi], x = c % W, z = (c / W) | 0;
      for (let k = 0; k < 4; k++) {   // (east, west, south, north — the order the fill has always taken them in: a room's cells are kept in it)
        const nx = x + ROOM_DX[k], nz = z + ROOM_DZ[k]; if (nx < 0 || nz < 0 || nx >= W || nz >= H) continue; const n = nav.idx(nx, nz);
        const d = dc.get(n); if (d) { if (!touched.has(d)) touched.set(d, c); continue; }
        if (nav.blocked[n] === 1 || mark[n] === 1 || gap?.has(n)) continue;
        const other = this.roomOfCell.get(n); if (other) { merge = other; break; }   // open floor an earlier (capped) fill already holds: the same room, not a second one over it
        cells.add(n); q.push(n); mark[n] = 1;
      }
    }
    for (let i = 0; i < q.length; i++) mark[q[i]] = 0;
    const room: Room = merge ?? { id: this.rooms.length, cells, entries: [] };
    if (merge) for (const c of cells) merge.cells.add(c);
    for (const [d, c] of touched) {
      if (room.entries.some(e => e.door === d)) continue;
      const F = frameOf(d); const cx = ((c % W) + 0.5) * cs, cz = (((c / W) | 0) + 0.5) * cs;
      const side = (cx - F.hx) * F.nx + (cz - F.hz) * F.nz >= 0 ? 1 : -1;   // the room's side of the frame line
      room.entries.push({ door: d, at: [d.frameCentre[0], 0, d.frameCentre[2]], out: [-side * F.nx, -side * F.nz] });
    }
    if (!merge) this.rooms.push(room); for (const c of cells) this.roomOfCell.set(c, room);
    return room;
  }
  private inRoom(R: Room | null, p: Vec3): boolean { if (!R) return false; const nav = this.g.col.nav; return R.cells.has(nav.idx(Math.floor(p[0] / nav.cell), Math.floor(p[2] / nav.cell))); }

  /** runner.ts (called only with its switch on): a man of his own side has run in and TOLD him where they are — what evidence() does for a loud noise his own
   *  ears put there: the entry that way is the one to fear, the noise record reads the place, and whatever glance he was on ends. False = no post of this file's */
  told(gd: Guard, at: Vec3): boolean {
    const D = this.defs.find(X => X.gd === gd); if (!D) return false;
    const e = this.entryToward(D, at); if (e >= 0) D.ev[e] = Math.min(3, D.ev[e] + 1);
    D.noiseAt = v3.copy(at); D.noiseT = this.g.time; D.noiseOut = !this.inRoom(D.room, at); D.glanceUntil = -1;
    if (BELIEF.on) word(this.g, gd, at);
    return true;
  }
  private entryToward(D: Def, at: Vec3): number {
    const E = D.room?.entries ?? []; let best = -1, bd = Infinity;
    E.forEach((e, i) => { const d = flat(e.at, at); if (d < bd) { bd = d; best = i; } });
    return best;
  }
  /** BLIND FIRE (nightbreach.ts BLIND): a dazzled defender keeps what he was watching as his fix and, a flinch after the flash, empties a burst down it without
   *  seeing anything — or (bangBreak) ducks for the length of it. A turn that comes while he is held, doubled over, head down, gun down, dry or mid-reload is lost. */
  private blind(D: Def, k: number) {
    const g = this.g, h = D.gd, B = D.burst, S = squadSkill(g, FIGHT_SKILL.hostile); if (g.time >= h.dazzledUntil) return;
    if (h.dazzledUntil > B.until + 1e-6) {
      B.until = h.dazzledUntil; B.n = 0; B.next = g.time + BLIND.after + k * BLIND.stagger; B.broke = simRandom() < S.bangBreak; B.daze = S.daze[1] > 0 ? pick(S.daze) * (this.drilled() ? DRILLED.shaken : 1) : 0;
      if (B.broke) { this.stats.broke++; this.note(h, 'breaks under the bang'); }
    }
    if (!h.held) cover(h, D.pt);
    if (B.broke) { if (g.tune.suppression) h.sup.downT = Math.max(h.sup.downT, 0.2); return; }   // head down (suppression.ts headDown: crouched, gun lowered) for as long as he cannot see
    if (!this.blindFire || B.n >= BLIND.rounds || g.time < B.next) return;
    B.n++; B.next = g.time + gunBlindGap(h, BLIND.every);   // (gunroles.ts: a dealt pistol fires at a pistol's rate; BLIND.every with its switch off)
    const c = h.char;
    if (h.held || g.time < h.staggerUntil || c.gunDir[1] < -BLIND.lowGun) return;
    if (g.tune.suppression && (h.sup.v >= SUPPRESSION.pinned || h.sup.latched || h.sup.downT > 0)) return;
    if (!h.weapon.fire()) return;
    fireWeapon(g, c, v3.mad(c.muzzle, c.gunDir, 8), false, { steady: true, spread: BLIND.spread, blind: true });
    this.blindRounds++;
    if (!this.drillSaid && this.drilled()) { this.drillSaid = true; g.say(h, wg('can\'t see — shoot through the doorway!', 'can\'t see — put it down the door!')); }   // (the cue, once a staging: the first drilled man to fire blind says what they are doing)
  }
  private drilled(): boolean { return drilledOn(this.g); }   // (DRILLED above)
  private drillSaid = false;
  /** is the room lost, as far as HE can tell: enough of its men down in his sight, with the enemy on him — or, alone, pinned with nowhere to put his head up */
  private lost(D: Def) {
    const g = this.g, gd = D.gd, c = gd.char;
    if (!ROOMFIGHT.fallBack || gd.state !== 'alert' || g.time < D.lostAt || g.time - D.movedAt < 6) return;
    D.lostAt = g.time + 0.3;
    const eye = eyeOf(c); let dead = 0, up = 0;
    for (const X of this.defs) {
      if (X.room !== D.room) continue;
      if (X.gd.state !== 'dead') { if (X.mode !== 'free') up++; continue; }
      const b = X.gd.char.bones.hips ?? X.gd.char.pos;
      if (g.col.segmentBlocked(eye, [b[0], 0.3, b[2]])) continue;                                // a man down is one HE has a line to…
      if (flat(b, c.pos) < 5 || !g.tune.bodyLight || bodyVisibility(g, X.gd) >= BODY_LIGHT.lit) dead++;   // …a few steps off, or lit where he lies (the floor's own bodies-by-light bar, guards.ts)
    }
    const pressed = g.time - gd.lastSeenT < 4 || (g.tune.suppression && gd.sup.v >= SUPPRESSION.flinch) || g.time - gd.witness.heard.lastShotT < 2.5;
    const frac = dead / Math.max(1, dead + up);
    const alone = dead + up <= 1 && g.tune.suppression && gd.sup.latched && gd.sup.pinnedT > 1.2;
    if (!pressed || !((dead > 0 && frac >= D.nerve) || alone)) return;
    this.breakOff(D);
  }
  /** fallBackIn: `p` lies outside the rectangle round every room of the level's structure with open sky over it (a level with no structure has no building
   *  to be outside of) */
  private outOfBuilding(p: Vec3): boolean {
    if (!ROOMFIGHT.fallBackIn) return false;
    const rooms = this.g.level.scenarios?.roomClear?.rooms; if (!rooms?.length) return false;
    let x0 = Infinity, x1 = -Infinity, z0 = Infinity, z1 = -Infinity;
    for (const r of rooms) { x0 = Math.min(x0, r.bounds.x0); x1 = Math.max(x1, r.bounds.x1); z0 = Math.min(z0, r.bounds.z0); z1 = Math.max(z1, r.bounds.z1); }
    if (p[0] >= x0 && p[0] <= x1 && p[2] >= z0 && p[2] <= z1) return false;
    let roofed = 0; for (const [dx, dz] of [[0, 0], [0.35, 0], [-0.35, 0], [0, 0.35], [0, -0.35]]) if (this.g.col.solidBlocked([p[0] + dx, 1.9, p[2] + dz], [p[0] + dx, 9, p[2] + dz], 0.02)) roofed++;
    return roofed < 3;
  }
  private threatOf(D: Def): Vec3 {
    const g = this.g, gd = D.gd;
    if (g.time - gd.sightT < 6) return gd.sightPos;
    if (gd.sup.src && g.time - gd.sup.srcT < SUPPRESSION.srcMemorySecs) return gd.sup.src;
    let bi = -1, bv = 0.3; D.ev.forEach((v, i) => { if (v > bv) { bv = v; bi = i; } });
    return bi >= 0 && D.room ? D.room.entries[bi].at : D.pt;
  }
  /** out of the room by a door nothing has come at, to hold it from the far side; no such door, to cover */
  private breakOff(D: Def) {
    const g = this.g, gd = D.gd, c = gd.char, nav = g.col.nav; const threat = this.threatOf(D);
    D.movedAt = g.time;
    const stays = hasHim(D.gd, OWN.stand);   // (laststand.ts, default off: a man of the last stand gives his room up through no door — cover inside it)
    const rp = stays ? null : squadRally(g, gd, threat);   // enemysquads.ts: his squad's rally point in another of its rooms, when he can get there (no squads: null on its first line)
    if (rp) {
      pressRun(g, D, rp, null, 0.3);   // (pressure.ts's leg on a script: the move this file's own fall-back makes, with no door behind him to hold)
      this.stats.fellBack++; this.note(gd, 'falls back to his squad\'s rally point'); g.say(gd, 'fall back — back to the others!');
      return;
    }
    // PICTURE.fallback (picture.ts): the door 'nothing has come at' and the spot beyond it are judged against EVERY enemy he has a firm idea of, not the one
    // threat point — a door within 3 m of a believed enemy, or whose far side a step out is in the straight line of one he has fresh word of, is passed over, his walk passes within 2 m of none, and of the spots beyond a door he takes the
    // first no believed enemy has a line to (none such: the first, as it was). No door left: cover in the room, as when there was none. Off, or with
    // no idea of any enemy at all: `was`, the choice as it was dealt, is the choice (ideas gone stale or seen empty no longer bar a door or a walk, but
    // still weigh, less, in the lines asked of the spot and the door's far side).
    const ask = PICTURE.on && PICTURE.fallback ? exposureAsker(g, gd) : null, known = ask ? believedOf(g, gd) : [];
    let best: { spot: Vec3; door: Door; L: number; safe: boolean } | null = null, was: { spot: Vec3; door: Door; L: number } | null = null;
    const nearLeg = (a: Vec3, b: Vec3, k: Vec3) => { const dx = b[0] - a[0], dz = b[2] - a[2], d2 = dx * dx + dz * dz, t = d2 > 0 ? Math.max(0, Math.min(1, ((k[0] - a[0]) * dx + (k[2] - a[2]) * dz) / d2)) : 0; return Math.hypot(k[0] - a[0] - dx * t, k[2] - a[2] - dz * t) < 2; };   // (the path is its corners: a believed man beside the middle of a straight leg counts)
    const leg = (spot: Vec3, mind: boolean): number | null => {
      const path = nav.findPath(c.pos, spot); if (!path?.length) return null;
      let L = 0, q: Vec3 = c.pos, toward = false; for (const w of path) { if (mind && known.some(k => nearLeg(q, w, k))) toward = true; L += flat(q, w); q = w; if (flat(w, threat) < 2) toward = true; }
      return L > 18 || toward || flat(q, spot) > 0.8 ? null : L;
    };
    const here = ask ? ask(c.pos) : 0;
    (stays || gd.staysInRooms ? [] : D.room?.entries ?? []).forEach((e, i) => {   // (a squad whose men stay in its rooms gives none up through a door: cover)
      if (D.ev[i] > 0.3 || flat(e.at, threat) < 3 || e.door.def.steel || g.doors.lockedOut(e.door, c.pos)) return;
      // a spot beyond the door, off its line (to either side of the opening) so the doorway is his to shoot into and not to stand in
      const F = frameOf(e.door); let spot: Vec3 | null = null, safe: Vec3 | null = null;
      for (const [depth, lat] of [[3.0, 1.6], [3.0, -1.6], [2.4, 1.2], [2.4, -1.2], [1.2, 2.4], [1.2, -2.4], [1.2, 1.7], [1.2, -1.7], [2.6, 0]]) {   // (lat is from the opening's middle; the shallow wide ones are for a corridor)
        const p: Vec3 = [e.at[0] + e.out[0] * depth + F.ux * lat * (1 + F.W / 3.2), 0, e.at[2] + e.out[1] * depth + F.uz * lat * (1 + F.W / 3.2)];
        if (nav.isBlocked(p[0], p[2]) || this.inRoom(D.room, p)) continue;
        if (this.outOfBuilding(p)) { if (!spot) this.stats.fallBackOut++; continue; }   // (fallBackIn: the far side of this door is the lot — counted as it was: up to the first spot that fits)
        if (this.defs.some(X => X !== D && X.gd.state !== 'dead' && flat(X.gd.char.pos, p) < 0.9)) continue;
        spot ??= p; if (!ask) break;
        if (ask(p) <= 0) { safe = p; break; }
      }
      if (!spot) return;
      const L0 = leg(spot, false); if (L0 !== null && (!was || L0 < was.L)) was = { spot, door: e.door, L: L0 };
      if (!ask) return;
      const mouth = ask([e.at[0] + e.out[0], 0, e.at[2] + e.out[1]]);
      if (known.some(k => flat(e.at, k) < 3) || (mouth > PICTURE.maybeK && mouth > here)) return;   // (…nor a door whose far side, a step out, is in the straight line of a man he has fresh word of — a line he is not already in where he stands: he would walk out into it)
      let sp = safe ?? spot, L1 = leg(sp, true); if (L1 === null && sp !== spot) { sp = spot; L1 = leg(sp, true); }   // (the hidden spot's walk does not hold: the first, as it was)
      const hid = sp === safe;
      if (L1 !== null && (!best || (hid && !best.safe) || (hid === best.safe && L1 < best.L))) best = { spot: sp, door: e.door, L: L1, safe: hid };   // (a door with a hidden spot beyond it before a nearer one without)
    });
    if (!ask) best = was; else if ((best as { spot: Vec3 } | null)?.spot !== (was as { spot: Vec3 } | null)?.spot) PICTURE_STATS.moved++;
    const B = best as { spot: Vec3; door: Door; L: number } | null;
    if (B) {
      D.mode = 'move'; D.moveKind = 'fallback'; D.cameBy = B.door; D.moveT = 0; D.stallAt = v3.copy(c.pos); D.stallT = 0; gd.pinned = false;
      order(gd, { goal: B.spot, speed: g.tune.guardChase + 0.3, face: null, strafe: false, aimAt: null, upper: 'aim', stance: 'none', crouch: false });
      this.stats.fellBack++; this.note(gd, `falls back through ${B.door.def.name} to ${B.spot[0].toFixed(1)},${B.spot[2].toFixed(1)}`); g.say(gd, simRandom() < 0.5 ? 'fall back — fall back!' : 'they\'re in — get out!');
      return;
    }
    const p = coverFrom(g, gd, threat);
    if (p) {
      D.mode = 'move'; D.moveKind = 'cover'; D.cameBy = null; D.moveT = 0; D.stallAt = v3.copy(c.pos); D.stallT = 0; gd.pinned = false;
      order(gd, { goal: p, speed: g.tune.guardChase, face: null, strafe: false, aimAt: null, upper: 'aim', stance: 'none', crouch: true });
      this.stats.tookCover++; this.note(gd, 'takes cover'); g.say(gd, 'cover — get to cover!');
    }
  }
  /** a defender on the move (his script's legs, his own senses): an enemy seen close stops him to fight; there, or going nowhere, he plants */
  private tickMove(D: Def, dt: number): void { return goneWithTick('RoomFight.tickMove') as any; }

  /** a friend — Sam, or another living operative — within a body's breadth of the line from → to (and a metre past it), wider for one on the move */
  friendInLine(from: Vec3, to: Vec3, self: Guard, samStill = false): 'player' | Guard | null {   // (samStill: asked AS IF the player stood still where he is — clearline.ts's question, never the fire gate's)
    const g = this.g; const dx = to[0] - from[0], dz = to[2] - from[2]; const L2 = dx * dx + dz * dz; if (L2 < 0.0025) return null;
    const L = Math.sqrt(L2), tMax = 1 + 1.0 / L;
    const hit = (ch: Character, mate: boolean): boolean => {
      const h = ch.bones.hips ?? ch.pos; const tr = ((h[0] - from[0]) * dx + (h[2] - from[2]) * dz) / L2, t = clamp(tr, 0, tMax);
      const sam = !mate && !!manOf(g, ch), pace = samStill && sam ? 0 : Math.hypot(ch.vel[0], ch.vel[2]), wide = 0.45 + Math.min(0.5, 0.3 * (ROOMFIGHT.samTravel && sam ? Math.min(pace, this.samSpeedOf(ch)) : pace)), clear = mate ? closeLane(wide, tr, L) : ROOMFIGHT.samBehind && tr > 1 ? Math.min(wide, 0.3 + 0.2 * (tr - 1) * L) : ROOMFIGHT.samBeside ? (tr <= 0 ? 0 : Math.min(wide, 0.3 + tr * L)) : wide;   // (the narrow lane at arm's length is between the team's own men: the player BETWEEN a gun and its man is owed the whole lane, always — standing PAST the man he is owed his own body's breadth and a fifth of how far past, samBehind)
      return Math.hypot(from[0] + dx * t - h[0], from[2] + dz * t - h[2]) < clear;
    };
    for (const pl of men(g)) if (!pl.down && hit(pl.char, false)) return 'player';
    for (const o of g.guards) if (o !== self && o.state !== 'dead' && o.char.alive && !isHostile(self.team, o.team) && hit(o.char, o.team === 'operative' && !!o.unit)) return o;
    return null;
  }
  /** the feet half of LIGHTS.litMan — in the dark on a lit man, Sam in the line to him, and an order's walk still to make: his feet wait (orders.ts reads
   *  short() and asks his legs for nothing; the order, its mark and its carriage are untouched and are his again a quarter of a second after this stops being stamped) */
  private holdShort(O: Op): void { return goneWithTick('RoomFight.holdShort') as any; }
  /** his muzzle back to the order: the overlay off his script (the order's own aim, carriage and facing were never touched) */
  private restore(O: Op) { const s = O.gd.script; if (s && s.over === O.ov) s.over = null; O.track = 0; }
  /** …and handed back: the order as it stood before the step, if nobody has ordered him since */
  private handBack(O: Op) {
    const b = O.step; O.step = null; const s = O.gd.script;
    if (b && s && s.goal === b.mine && O.gd.state !== 'dead') order(O.gd, { goal: b.goal, speed: b.speed, strafe: b.strafe, face: b.face });
  }
  /** the open doorway `p` stands in (its frame's span, within 0.75 m of the frame line; a shut leaf is a wall, not an opening) */
  funnelOf(p: Vec3): Door | null { return goneWithTick('RoomFight.funnelOf') as any; }
}
