// Defenders that PRESS a room their side has lost, come ROUND through a second door of it, and FALL BACK to fight again — the loud squad scenarios' harder
// defenders, each behaviour behind its own switch and the whole of it behind `PRESSURE.on` (default OFF: nothing here runs, no die is drawn, and the room fight is
// roomfight.ts exactly as it was — its three seams are `OWN.press` at the head of rally(), `pressTick` at the tail of tick() and `pressStop` in stop()).
//
// Nothing here is more hit points, a faster trigger, a truer aim or a die on the outcome. It is the SAME men with the SAME senses doing something less dumb with
// what they hear, and each thing they do says so first — one model for the picture and the AI: a call is `g.say` on their radio net (the '<callsign> (radio):'
// line in the log, the way tactics.ts's alert men already tell each other 'moving round' / 'set' — what a man knows of a press beyond his own ears is what went
// over that net), boots are 'step' hearing events the teammates' own ears take (roomfight.ts stepEars), a door is the door.
//
//  PRESS (PRESSURE.press). Today a defender who hears a fight at a mate's post goes to it only if the noise is still fresh when his beat is up, and then three
//    times in four (roomfight.ts rally): a room that falls in a second draws nobody, and its neighbours wait in theirs to be cleared in turn. With the switch
//    on, a fight at a post of his side's in another room is a thing he ACTS on for `memorySecs` after he heard it: after the same beat of his own (FIGHT_SKILL
//    rally) he calls it ('they've hit the others — get in there!' / 'with you — moving!') and goes at it, the fight AI's man exactly as the stock rally's is.
//    Who goes is situation, not dice: a lone garrison goes, a room of several leaves its farthest man at home, at most `maxMen` to a press. A man this file
//    does not take is the stock rally's as before (`OWN.press`): the hook never leaves a man standing who would have gone.
//    `gather` (off) is the other way to press, kept for A/B: they RUN to spots outside that room's doors, off their lines, hold them from outside ('hold the
//    doors — let them come to us!') until a fight inside, the trap sprung at one door (`converge`) or `siegeSecs`, and go together on a word ('go — go — go!').
//    One man by himself never goes in on that road: he holds the door from outside and says so. Measured: a formed-up push into a room four rifles hold LOSES
//    (the team keeps more men than today); going at once, while the entry is still on, is what costs the team.
//  FLANK (PRESSURE.flank). The second and later men of a press look at the door their shortest way goes through: if a man of the press has it already and
//    another door of that room is not absurdly far round (≤ flankMaxM, ≤ max(flankK × his shortest way, shortest + flankSlackM)), he takes that one — 'going
//    round — keep them there!', a run on a path that does not cross the room, his boots filed for the team's ears, then 'in position — on them!' and in. So
//    the press arrives at two doors, and the door the team did not watch is the one it comes through.
//  FALL BACK (PRESSURE.fallBack). (a) A man of a press who can see enough of it down (his own nerve, the stock fall-back's number — or every other man of it
//    lies where he can see him) with the enemy on him goes BACK out to a spot outside the door he came by and holds that door from there: 'back — get back!
//    hold the door!'. (b) The last man up in a room, with two or more of the team in his own sight, does not die at his post: after his own beat he leaves by
//    a door the trouble is not at, plants beyond it watching it, and tells whoever of his side holds the room he has come into ('they're right behind me —
//    watch that door!' — their evidence for that door goes up). (c) `outEarly`: he does not have to see them — on a floor gone loud, the team's boots at one
//    door of his room send him out by another before the door goes. WHERE a man falls back to is one seam (`PRESSURE_HOOKS.fallBackTo`) another module may
//    answer — a room his side has dug in. The stock fall-back (roomfight.ts lost → breakOff) is untouched and still runs first. Men somebody else has pinned
//    to their room (the rescue's men with the hostages: rallyAt Infinity / lostAt ahead of the clock) and a man in Sam's arm are never this file's.
//  STAND AND FIGHT (PRESSURE.second — off in the build, ON under the difficulty switch's hard). A team that does not wait in the room it took is met in the
//    open, on its way to the next door, by the men the press sent at that room — and a man who walks on at four rifles while he shoots loses. With this on, a
//    hunter the press called who gets his eyes on one of them OUT of the room he was called to (and is out of it himself) stops where he stands and fights
//    from there, the room fight's holder again: a set gun, the first of them calling it on their net ('contact — they're out in the open! stand and
//    fight!'). A team still in the room is pressed exactly as before. Off = they walk on at you one by one, as they did.
//
// What the counts mean (`pressureStats`, tools/qa/playthrough.ts files them per run): a behaviour FIRED when a man acted on it; its cue is the call at that
// moment; it LANDED when that man first had eyes on an enemy or an enemy on him; `lead` = seconds from HIS call to that (summed; ≥ 1 s = the cue came first);
// `heard` = a teammate's own ears had that man's boots (roomfight.ts Op.stepBy) before he landed.
import { SideMap, SideSet } from '../math/side';
import { Vec3, v3 } from '../math/vec';
import type { Game, Guard } from './game';
import type { Door } from './doors';
import type { RoomFight } from './roomfight';
import { frameOf } from './guards';
import { order } from './squad';
import './teams';
import './glasspen';
import './digin';
import './enemysquads';
import './silent';
import { bang } from './effects';
import { DOORTOOLS } from './doortools';
import './tactics';
import './lanes';
import './teampace';
import './kitload';   // (a teammate's card in the defenders' ears: his footfall as the mixer has it)
import { releaseMan, ownerWitness, OWN } from './owner';
import { word } from './belief';
import { RUST_PASS, RUST_SURFACE, crossToRust } from './crossing';

export const PRESSURE = {
  /** the master switch: off = none of this file runs */
  on: false,
  /** the three behaviours, each measurable alone (all need `on`) */
  press: true, flank: true, 
  /** HARD ON A LEVEL THAT DECLARES SQUADS (enemysquads.ts): the press above stands down there — the squads own who moves — so harder defenders are the
   *  squads themselves as a DRILLED GARRISON: enemysquads.ts SQUADS.drill* (each rule its own switch there), read only while `on` and this row are both
   *  up and squads are staged. Off = hard on a squads level is standard run for run, as it was */
  squadDrill: true,
  /** stand and fight: a hunter the press called who meets the team out of the room he was called to stops and fights from where he stands (needs press;
   *  nothing under `gather`, whose men are set outside the doors already) */
  second: false,
  /** men to one press (a far room that goes together — `byWay` — is not cut in two by it: its men are over the count) */
  maxMen: 4,
  /** s a fight he heard stays worth going to */
  memorySecs: 15,
  /** × his beat between hearing the fight and calling it and going (1 = the stock rally's own beat, FIGHT_SKILL rally: 1.5–4 s). The dial for how long a team
   *  that keeps moving has before the press is on it */
  organiseK: 1,
  /** STAND TO — surprise lost before the first entry. The first loud thing they hear is not a room of theirs being hit (the team's gunfire on the way in): the
   *  man best placed calls it on the net ('shots outside — they're coming. stand to!'). While the report stands nobody walks off to it by himself, and the
   *  men of every room but the one the noise is nearest keep their hands off the furniture and stand ready. When a room IS hit ('contact! …' on the net)
   *  it is ALL HANDS, for the rest of the fight: no man is left at home to keep a room and a press takes every man who hears it (maxMen is not asked).
   *  Nobody comes, and nothing more is heard, for `standSecs`: they stand down, on the net, and the stock rules are back (a later report starts it again).
   *  Off = a report is a fight at the nearest mate's post, as it was. A fight that starts as a fight (the first loud thing is a boot, a charge, a round in a
   *  room) never comes through here: a team that kept surprise meets the defenders it met with this off. Off in the build, ON under the
   *  difficulty switch's hard (difficulty.ts HARD.warned) */
  warned: false,
  /** stood to, ALL HANDS GO THROUGH THE PRESS (needs warned): the stock rally's clock does not run while the report stands, so at the hit every man takes his
   *  beat and joins the press (round by another door, stand and fight). Off = as the rule was first measured: the clocks armed at the report have run out by
   *  the hit, and three men in four leave that frame by the stock rally, each by himself, the rest through the press. OFF: measured easier for the team
   *  (The block with surprise lost, seeds 145-288: +18 runs won of 144, +3 to +33; +17 teammates of 576, −17 to +51) — men who all arrive together
   *  at the bang meet the team at its strongest */
  standPress: false,
  /** s between the first man hearing it and his call */
  reportSecs: 1.4,
  /** s stood to with nobody coming and nothing more heard before they stand down */
  standSecs: 75,
  /** s after the room is hit that a man who stood ready is still the press's, not the furniture's (his beat toward the fight starts at the hit) */
  standGrace: 5,
  /** s before a man who has been in a press joins another */
  againSecs: 14,
  /** A CHARGE EACH (0 = none: f6cf9316's press run for run; on under hard on The block only — difficulty.ts HARD_AT; record
   *  rebuild/hard-everywhere/round2-block.md): every defender carries this many stun charges, the team's own kind, and a man of a press let
   *  go at the fight takes the door he was put down for as a DRILL instead of walking in (tactics.ts's kill-zone doorway, built for the night staff's response
   *  team and never yet handed a charge): on the jamb out of the opening, the call on their net ('they're in there — … stack on me, charge first'), his second
   *  fetched or the wait run out, the leaf shoved, 'bang out', the can in round the jamb and heard on the floor before it goes, 'going in'. All of that is what
   *  the player is given to answer it with: a voice at the door, an arm round the jamb, a can bouncing. Handed to each man of a fight once, the first frame the press sees him (a charge spent is not replaced); what it does to a man
   *  is combat.ts FLASH, the same numbers both ways */
  stuns: 0,
  /** A CHARGE GOES ONLY WHERE THEY ARE KNOWN TO BE (on; off = a man let go with a charge marked whatever door the press put him down for — the 10-13 tapes:
   *  eight charges burst in rooms no teammate was in, each after a man of that room was shot through a window from outside, two of them after their own net
   *  had said the team was in the yard, one dazzling their own man). 'A fight at a mate's post' is what sends a press; it does not say the team is IN that
   *  room. A door is taken charge first only on something that does: a living man of theirs who has NOTICED them has had one of the team in his sight in
   *  that room, the team's own unsuppressed gunfire came from inside it, they booted a door of it, or the man at the door has their boots in his ears through it — inside `stunKnowSecs`, and
   *  with nothing newer (by more than `stunMovedSecs`) putting them somewhere else. A man let go without it walks on as the fight AI's man, as he did before
   *  anybody carried a charge, and takes the door as the drill the moment his side has it (`stunLate`); a door already marked and not yet banged is let be the
   *  moment the newest thing they know puts the team elsewhere (`stunOff`: tactics.ts's 'the doorway is plain again'). */
  stunKnowSecs: 20, stunMovedSecs: 3,
  /** A LAST MAN ALONE KEEPS HIS ROOM (on; off = the last of them alive pressed like anybody — 'they've hit the others — get in there!' on a net with
   *  nobody left on it, and off alone at four rifles; the 10-13 tapes have it five times, each stopped short by a covered lane a moment later). With it on
   *  he neither starts a press nor walks off on the stock rally: his room, his doors, and the rules for a room's last man (fall back, dig in) as they are. */
  lastHolds: true,
  /** THE PICKET (off: every staging as it was; off under hard too — difficulty.ts HARD.picket, built and measured, not shipped; record rebuild/hard-everywhere/depot-and-pairs.md):
   *  NEVER TWO MEN IN ONE FLASH, in the room the team comes to first. Its men are the ones no rule above reaches: no fight comes before theirs, so they hear
   *  nothing, go nowhere, and one can through one door folds the pair of them where they stand (measured: worth next to nothing, at standard and hard alike).
   *  Where that room has a second door, one of the pair stands OUTSIDE it instead — beyond the door farthest on foot from the team's start, off its line, his gun
   *  on it (where that door opens into the next room, as on the depot and The block, he stands in that room, beside its men). The can does not reach him; he hears the room go, says so, and comes in by his door behind the men who took it. Only that room (a man of any other
   *  room is one who would have come hunting: posting him outside was measured to make the big floor far easier), only when the two would share a can (burst
   *  1.6 m inside a door, both within its reach with a line to it), never a man the scenario pins. He can be seen, and dealt with, before the entry */
  picket: false,
  /** READS THE DOORS BETWEEN (off: 51ce9c3's hard run for run; ON under hard — HARD.byWay): where his room stands to the fight decides who goes. NEXT DOOR
   *  (his room shares a door with the room being fought over): as ever — he goes on his own beat and the farthest man of his room keeps it, because that door
   *  is the team's next. NOT next door (a corridor, a hall or another room between): nobody is coming for his room next, so nobody is kept and the men of one
   *  room go TOGETHER. The first to place the noise says so and the men of his room know it from him (a noise heard and not placed is otherwise a man who
   *  turns to his furniture), the place stays put for `memorySecs` (not lost when the next shot comes to him by another doorway), and the first whose beat is
   *  up waits for the last of them — no beat is shortened; they leave on the slower man's — and they go by one door (the second takes the first's door and
   *  role: the other door stays free for a man who goes round), so they are let go the same moment and the fight AI has two men on
   *  one fix instead of one man walking twenty metres to land alone on a team that has had fifteen seconds to set. Measured (the depot's stores, 19 m and three
   *  doors from the crew room): the pair that left together was worth three times the two that left one at a time */
  byWay: false,
  /** HARD'S RESCUE CREW (off; ON under hard — HARD.rescueCrew): on a hostage rescue the captor's two men act on his word (rescue.ts crew(): a gun on the
   *  hostage door from a side room · the door man inside · a roamer who hides and comes up behind — each its own RESCUE row). Nothing in this file reads it */
  rescueCrew: false,
  /** HARD'S DRILLED DEFENDERS (off; ON under hard — HARD.drilled): on the night breach by itself, a defender is half as shaken by a stun canister or a
   *  boot, so the burst he fires blind down the door he was watching is partly fired instead of lost to the seconds he spent doubled over (roomfight.ts
   *  DRILLED). Nothing in this file reads it; like every row here it does nothing with `on` off */
  drilled: false,
  /** the way round a flanker will take: metres at most, and against his shortest way to that room */
  flankMaxM: 46, flankK: 3, flankSlackM: 14,
  /** the last man's beat before he gives a room up: s, with two of the enemy in view the whole of it */
  lastManSecs: [0.35, 0.9] as [number, number],
  /** s a fight heard elsewhere on the floor keeps a man that ready to give his room up · his beat between the boots at his door and his move */
  loudSecs: 90, outSecs: [0.8, 1.6] as [number, number],
  /** s from a press's first call until its men stop waiting for more of them (two alive: set; one: he holds the door from outside) */
  gatherSecs: 8,
  /** s more they wait for a man still on his way round */
  flankSecs: 6,
  /** s a press set on a quiet room's doors holds them from outside before it goes in (0 = in as soon as it is set) */
  siegeSecs: 25,
  /** s between the word and the move: the cue's own length */
  wordSecs: 0.9,
  /** THE WAY OUT (off: every level as it was; ON under hard — difficulty.ts HARD.wayOut; record rebuild/hard-everywhere/round2-records-pair.md).
   *  Only on a floor whose structure asks for it (scene/scenarios.ts StructureData.wayOut — levels/records.json: a pair's building, one man in the first room,
   *  every hunter's way in through the one door two rifles are already on). There the men a lost room draws do not walk into it one at a time: they FORM UP
   *  on ONE door of it — the nearest to the first of them (the `gather` road above, whatever that row says) — and hold it from back off it: `wayOutM` past
   *  the reach of the can that comes out of a door first (effects.ts bang.dazzleRadius round where it goes off), each with a line to the door, none in
   *  another's, out of the place's own light where the floor allows — guns on the door the team has to leave by. Nobody goes round, nobody goes in at the
   *  noise, a man with no such spot stays home; the lead says it on their net ('back off the doors — out of their flash! let them come out to us!'): that
   *  line is the cue. They go in on the word only when `wayOutSecs` have brought nobody out. The measured answer is SPEED: they need some eight
   *  seconds from the door going to be set, and a pair that goes straight on through that door meets them walking (the record's rush row); a can thrown
   *  long, another door with a can for the hall, or no light in the doorway are a player's to try — not measured */
  wayOut: false, wayOutM: 0.6, wayOutSecs: 60,
  /** GO AND LOOK (off: every level as it was; ON under hard where the page says so — difficulty.ts HARD.sweep. The maintainer, 10-13: 'harder settings have
   *  better trained guys, more proactive, higher morale, better kit'). Their side has had word of the team — one of them properly in a man's sight, a loud
   *  thing in a man's ears — and then NOTHING for `sweepSecs`. On the floor as it was nobody moves again: each man holds the spot the noise left him on
   *  until the team comes to him, however long they take over it. With this on two of them go and look, together: the holding man nearest the place that
   *  word put the team who has a mate within `sweepPairM` of him, and that mate, on his word on their net ('gone quiet. with me — we go and look.': that
   *  line is the cue, `sweepBeat` before either moves), let go at that place as the stock rally lets a man go — the fight AI's own hunt, its doorway drill
   *  and its search; where they end up calm, they hold. ONE look per word: a place looked at is not looked at again until the team is seen or heard anew,
   *  and never oftener than every `sweepSecs`. Nobody goes to a room a man of theirs holds (he is looking at it), while a man is still out, a press or a
   *  fall-back is live or a report stands; somebody always stays (`sweepMen` is never fewer than two nor all of them); and A LOOK THAT LOSES A MAN IS THE
   *  LAST: nobody else walks out that fight. Never a squad's man (enemysquads.ts has its own form of this), a last stand's, a hostage rescue's, one on
   *  his way to cover or wedging a door (digin.ts; a man SET on his door may go), or one who knows a covered lane lies that way (lanes.ts). The dead say
   *  nothing: a man dropped without a sound is no word. No die is drawn. */
  sweep: false, sweepSecs: 20, sweepMen: 2, sweepBeat: 1.5, sweepPairM: 12,
};

type Def = RoomFight['defs'][number];
type Room = NonNullable<Def['room']>;
type Entry = Room['entries'][number];
interface Man {
  D: Def; role: 'press' | 'flank'; door: Door; spot: Vec3;
  /** the room he left for it */
  from: Room | null;
  /** on a leg THIS module ordered (a stock fall-back taking him off it drops him from the press) */
  moving: boolean; set: boolean; setAt: number; tries: number; stepT: number; small0: number;
  /** when he said what he was about to do */
  cueT: number;
  /** when he was let go at the fight */
  freeAt: number; landT: number; arrived: boolean; heard: boolean; fell: boolean; replantT: number; fought: boolean; out: boolean;
  /** when he last asked for a way back */
  askT?: number;
  /** PRESSURE.stunKnown: his door is marked for the drill on his account · when he last listened at it */
  charged?: boolean; earT?: number;
}
interface Press { id: number; room: Room; at: Vec3; t0: number; phase: 'gather' | 'word' | 'go'; teamIn: boolean; setAt: number; loneAt?: number; sprung?: boolean; listenAt: number; moving: boolean; wordAt: number; goAt: number; n0: number; lone: boolean; men: Man[] }
/** a last man who gave his room up */
interface Out { door: Door | null; def: Def; moving: boolean; plantT: number; fought: boolean; stepT: number }
const newStats = () => ({
  pressFormed: 0, pressMen: 0, pressSieges: 0, pressConverge: 0, pressWords: 0, pressLone: 0, pressArrived: 0, pressLate: 0, pressLanded: 0, pressCueFirst: 0, pressHeard: 0, pressLead: 0,
  flankSent: 0, flankSet: 0, flankArrived: 0, flankLate: 0, flankLanded: 0, flankCueFirst: 0, flankHeard: 0, flankLead: 0,
  fbPush: 0, fbLast: 0, fbEarly: 0, fbPlanted: 0, fbFought: 0, fbTold: 0,
  /** stand to: reports called, stood down with nobody coming, rooms hit while stood to, and Σ s between the call and that hit */
  warnCalls: 0, warnStoodDown: 0, warnHits: 0, warnLead: 0,
  secondStood: 0, secondCalls: 0,
  pairsTold: 0, pairsWaited: 0, pairsWent: 0,
  /** PRESSURE.stuns: doors marked charge first · a charge held back (nothing put the team in that room) · of those, with a man of the team in it all the same (a count: the floor's truth) · marked later, once something did · a mark let be
   *  (the team is known to be elsewhere) · charges that went off through a marked door, and of them into a room no man of the team stood in (a count: the
   *  floor's truth, which no defender is given) · the last man alone who kept his room */
  stunMarked: 0, stunHeld: 0, stunHeldIn: 0, stunLate: 0, stunOff: 0, stunBangs: 0, stunEmpty: 0, lastHeld: 0,
  /** PRESSURE.sweep: looks called · men let go on one · a look called off in its beat (the team was seen or heard again, or a man of the pair was no longer free to go) */
  lookCalled: 0, lookMen: 0, lookOff: 0,
});
/** STAND TO: the report standing. 'heard' = a man has it in his ears, 'called' = it is on the net and nobody has come yet, 'over' = a room was hit (or the
 *  first loud thing WAS a room being hit: there never was a report) */
interface Warn {
  phase: 'heard' | 'called' | 'over'; t0: number; by: Def;
  /** when it was called (−1: never) and the last loud thing any of them heard while it stood */
  callT: number; lastT: number;
  /** the room of theirs the noise points at: its garrison braces, the men of every other room are the reaction force (null: no report) */
  room: Room | null;
  /** when a room was hit */
  hitT?: number;
}
interface State {
  presses: Press[]; of: Map<Def, Man>; outs: Out[]; serial: number;
  /** the fight these counts were of has stopped: the next one starts them again */
  stale: boolean;
  think: Map<Def, { room: Room; at: number; /** PRESSURE.byWay: since when he has waited for a mate */ held?: number }>; stay: Map<Def, Room>; called: Map<Def, { T: Room; t: number }>; stoodT: number;
  /** rooms a man has left for a press this fight (keeperLeft) */
  sent: Set<Room>; doneAt: Map<Def, number>; last: Map<Def, number>; tried: Map<Def, number>;
  /** what a man alone in his room last heard of the team at a door of it: when he listened, which entry, when he heard it */
  ear: Map<Def, { t: number; i: number; at: number }>;
  warn: Warn | null;
  /** PRESSURE.byWay: a far fight he has placed, or that a man of his room placed and told him of (a noise he heard and could not place himself) */
  told: Map<Def, { T: Room; at: Vec3; t: number }>;
  /** PRESSURE.stunKnown: when the team was last HEARD in a room (their own gunfire from inside it, a door of it booted, boots through the door a man stands
   *  at), the newest such sound anywhere, the events already read, and the doors marked charge first with the room beyond each */
  heard: { t: number; p: Vec3 }[]; evSeen: WeakSet<object>; marks: Map<Door, { R: Room | null; team: Guard['team']; own: boolean }>; bangs: WeakMap<object, boolean>; lastHeld: Set<Def>;
  /** PRESSURE.sweep: the newest word their side has of the team (when, where) · the word the last look was called on · the look called and not yet gone */
  word: { t: number; p: Vec3 } | null; swept: number; look: { at: Vec3; goT: number; w: number; men: Def[] } | null; /** the men of the last look while any is still out · one of them went down out there · when the last look was called */ out: Def[] | null; lookLost: boolean; lookT: number;
  stats: ReturnType<typeof newStats>;
}
const STATES = new SideMap<Game, State>('pressure.STATES');
ownerWitness<Game>('press', (gd, g) => { if (!PRESSURE.on || !PRESSURE.press) return false; const st = STATES.get(g), D = g.fight.defs.find(X => X.gd === gd); return !!st && !!D && (st.of.has(D) || st.think.has(D)); });
const stateOf = (g: Game): State => { let s = STATES.get(g); if (!s) { s = { presses: [], of: new Map(), outs: [], serial: 0, stale: false, think: new Map(), stay: new Map(), called: new Map(), stoodT: -100, sent: new Set(), doneAt: new Map(), last: new Map(), tried: new Map(), ear: new Map(), warn: null, told: new Map(), heard: [], evSeen: new SideSet(), marks: new Map(), bangs: new SideMap(), lastHeld: new Set(), word: null, swept: -Infinity, look: null, out: null, lookLost: false, lookT: -Infinity, stats: newStats() }; STATES.set(g, s); } return s; };

const KITTED = new SideSet<Guard>('pressure.KITTED');   // PRESSURE.stuns: the men who have been handed theirs
const ROOM_CELLS = 500;   // past this a 'room' is open floor (roomfight.ts's own line between the two)
const flat = (a: Vec3, b: Vec3) => Math.hypot(a[0] - b[0], a[2] - b[2]);
const alive = (D: Def) => D.gd.state !== 'dead';
const cellOf = (g: Game, p: Vec3) => { const nav = g.col.nav; return nav.idx(Math.floor(p[0] / nav.cell), Math.floor(p[2] / nav.cell)); };

/** the room of his side's he hears fought over: the nearest post of a mate's in ANOTHER room to where his own ears put the noise (he knows where his side
 *  stands; he does not know where the enemy does), if that is a real room and not his */
function target(F: RoomFight, D: Def): { T: Room; at: Vec3 } | null {
  if (!D.noiseAt) return null;
  const mine = picketHears(F, D);   // (a picket still on his post: a noise in the room he was posted outside, or at a door of it, is that room's — he knows whose it is)
  if (mine) return { T: mine, at: v3.copy(D.noiseAt) };
  let best: { T: Room; at: Vec3 } | null = null, bd = 9;
  for (const X of F.defs) {
    if (X === D || X.room === D.room || X.mode === 'free') continue;
    const d = flat(X.station, D.noiseAt); if (d >= bd) continue;
    const T = (PICKET_WENT.has(X.gd) ? undefined : PICKETS.get(X.gd)) ?? F.roomAt(X.station);   // (a picket on his post stands for the room he was posted outside, not the one his feet are in)
    if (!T || T === D.room || T.cells.size > ROOM_CELLS || !T.entries.length) continue;
    bd = d; best = { T, at: X.station };
  }
  return best;
}

/** the pickets of this staging (PRESSURE.picket): a man posted outside his garrison's room knows it is his side's, wherever in it the noise is */
const PICKETS = new SideMap<Guard, Room>('pressure.PICKETS'), PICKET_WENT = new SideSet<Guard>('pressure.PICKET_WENT');
const PICKET_DOOR_M = 2.5;   // (a noise this near a door of his room is AT it)
/** the room a picket hears fought over while he is still on his post: his garrison's, if the noise is in it or at a door of it. Once he has gone in he is a man
 *  like any other of the room he stands in */
function picketHears(F: RoomFight, D: Def): Room | null {
  const mine = PICKETS.get(D.gd); if (!mine || PICKET_WENT.has(D.gd) || D.room === mine || !D.noiseAt) return null;
  return F.roomAt(D.noiseAt) === mine || mine.entries.some(e => flat(e.at, D.noiseAt!) < PICKET_DOOR_M) ? mine : null;
}
/** PRESSURE.picket at a staging (nightbreach.ts stage(), before the cast is spawned): `marks` = where the scenario stands its defenders, `teamAt` = where the team
 *  starts. Returns the marks with the first room's picket moved outside it (`picket` = the room he is of; never the drill's man afterwards) — the same array
 *  when the switch is off or nothing qualifies */
export function picketMarks<M extends { pos: Vec3; aimAt: Vec3; drill?: boolean }>(g: Game, marks: M[], teamAt: Vec3): (M & { picket?: object })[] {
  if (!PRESSURE.on || !PRESSURE.press || !PRESSURE.picket) return marks;   // (what he does from his post is the press's: no press, no picket)
  const F = g.fight, nav = g.col.nav, by = new Map<Room, number[]>();
  const flashM = bang.dazzleRadius, burstM = DOORTOOLS.airIn;   // (what a can thrown through a door reaches, and how far inside the frame line it goes off)
  marks.forEach((m, i) => { const T = F.roomAt(m.pos); if (T && T.cells.size <= ROOM_CELLS) { const l = by.get(T); if (l) l.push(i); else by.set(T, [i]); } });
  const wayLen = (a: Vec3, b: Vec3) => { const path = nav.findPath(a, b, 20000); if (!path?.length) return Infinity; let L = 0, q = a; for (const w of path) { L += flat(q, w); q = w; } return L; };   // (once a staging: a budget that does not lose the far door of a big floor)
  // the room the team comes to first: the one with the shortest way on foot from its start to the near side of a door of it
  let first: Room | null = null, fd = Infinity, near: Entry | null = null;
  for (const T of by.keys()) for (const e of T.entries) { if (e.door.def.steel) continue; const d = wayLen(teamAt, [e.at[0] + e.out[0] * 1.2, 0, e.at[2] + e.out[1] * 1.2]); if (d < fd) { fd = d; first = T; near = e; } }
  const idx = first ? by.get(first)! : [];
  const T = first as Room | null; if (!T || idx.length < 2) return marks;
  const ways = T.entries.filter(e => !e.door.def.steel); if (ways.length < 2) return marks;
  const share = new Set<number>();   // the men of it one can would take together, through whichever door
  for (const e of ways) {
    const b: Vec3 = [e.at[0] - e.out[0] * burstM, 1.2, e.at[2] - e.out[1] * burstM];
    const hit = idx.filter(i => { const h: Vec3 = [marks[i].pos[0], 1.6, marks[i].pos[2]]; return v3.dist(h, b) < flashM && !g.col.segmentBlocked(b, h); });
    if (hit.length >= 2) for (const i of hit) share.add(i);
  }
  if (!share.size) return marks;
  let best: { e: Entry; spot: Vec3; L: number } | null = null;
  for (const e of ways) {
    if (e === near) continue;   // (never the door the team comes to)
    const fr = frameOf(e.door);
    for (const [depth, lat] of [[2.4, 1.2], [2.4, -1.2], [3.0, 1.6], [3.0, -1.6], [1.2, 1.7], [1.2, -1.7]]) {
      const p: Vec3 = [e.at[0] + e.out[0] * depth + fr.ux * lat * (1 + fr.W / 3.2), 0, e.at[2] + e.out[1] * depth + fr.uz * lat * (1 + fr.W / 3.2)];
      if (nav.isBlocked(p[0], p[2]) || T.cells.has(cellOf(g, p)) || marks.some(m => flat(m.pos, p) < 0.9)) continue;
      if (g.col.solidBlocked([p[0], 1.1, p[2]], [e.at[0] + e.out[0] * 0.5, 1.1, e.at[2] + e.out[1] * 0.5])) continue;   // (he stands where he sees his door)
      const L = wayLen(p, teamAt); if (L === Infinity) continue;
      if (!best || L > best.L) best = { e, spot: p, L };
      break;
    }
  }
  if (!best) return marks;
  let who = -1, wd = Infinity; for (const i of share) { const d = flat(marks[i].pos, best.e.at); if (d < wd) { wd = d; who = i; } }
  const out: (M & { picket?: object })[] = marks.slice();
  out[who] = { ...marks[who], pos: best.spot, aimAt: [best.e.at[0], 1.1, best.e.at[2]] as Vec3, drill: false, picket: T };
  return out;
}
/** …and once he is spawned on it (nightbreach.ts): `room` = what picketMarks put on his mark */
export function postPicket(gd: Guard, room: object) { PICKETS.set(gd, room as Room); }

/** AND WHOM IT KEEPS AT HOME, worked out as it is asked because it turns on what changes between its ticks (who is still alive, what he has just heard): the last of them alive (PRESSURE.lastHolds: nobody is left to go to,
 *  and the other askers read it the same way and lose nothing: no mate is alive to call him (buddy.ts), laststand.ts leaves him be, and cues.ts gives no 'staying' word to an empty net), everybody while a report is stood to, and
 *  the man who stays to keep his room. Whoever the press has not taken or kept (a fight out on the open floor, the press switch off, a man it turned down) is the stock rally's exactly as before.
 *  STOOD TO, NOBODY WALKS OFF TO A REPORT BY HIMSELF, AND WITH `standPress` THE STOCK WALK'S CLOCK DOES NOT RUN ON UNDER IT: that happens as he is asked. `stopClock` false: the asker only wants the word (powerout.ts, darkfoe.ts) */
export function pressKeepsHome(g: Game, D: Def, stopClock = true): boolean {
  if (!PRESSURE.on || !PRESSURE.press) return false;
  const st = stateOf(g);
  if (lastAlone(g.fight, D)) return true;
  if (awaiting(st)) { if (stopClock && PRESSURE.standPress) D.rallyAt = NaN; return true; }
  const keep = st.stay.get(D); return !!keep && aim(st, g.fight, D)?.T === keep && staysHome(g.fight, st, D);
}
/** the man who was left to keep his room still has it to keep: a mate of that room is up (out on the press or back) — alone for good, he is a lone garrison */
function staysHome(F: RoomFight, st: State, D: Def): boolean {
  for (const X of F.defs) if (X !== D && alive(X) && !PICKETS.has(X.gd) && (X.room === D.room || st.of.get(X)?.from === D.room)) return true;   // (a picket standing in it is not of it)
  return false;
}


export function run(g: Game, D: Def, spot: Vec3, door: Door | null, fast = 0) { { const __s = RUST_SURFACE; if (__s) { const __r = __s.call('run', undefined, [g, D, spot, door, fast]); if (__r !== RUST_PASS) return __r as any; } }
  const gd = D.gd;
  D.mode = 'move'; D.moveKind = 'fallback'; D.cameBy = door; D.moveT = 0; D.stallAt = v3.copy(gd.char.pos); D.stallT = 0; D.rallyAt = NaN; gd.pinned = false;
  order(gd, { goal: v3.copy(spot), speed: g.tune.guardChase + fast, face: null, strafe: false, aimAt: null, upper: 'aim', stance: 'none', crouch: false });
}
/** PRESSURE.stuns: a man with a charge on him, let go at the fight: the door he was put down for is taken as the drill, charge first (tactics.ts's doorway).
 *  `T` = the room the fight is in when the caller has it (the press's own); the room fight's seam with the press off names a door only, and the test is then
 *  asked of the rooms that door gives on to, his own aside. PRESSURE.stunKnown: no door is marked on a fight at a mate's post alone — their side must have
 *  something that puts the team IN that room (`teamPlaced`). The press's own men ask again at the door (tickPress); the seam's men are asked the once. */
export function charge(g: Game, D: Def, door: Door, T?: Room | null): boolean { return crossToRust('charge', undefined, [g, D, door, T]) as any; }
/** the last of his side alive (PRESSURE.lastHolds): there are no others to have been hit, and nobody on their net to go with */
const lastAlone = (F: RoomFight, D: Def) => PRESSURE.lastHolds && !F.defs.some(X => X !== D && alive(X));
/** handed to the fight AI as the stock rally hands a man over: off his post, roused, the fight's place his fix — he works a doorway a slice at a time by himself */
export function loose(g: Game, D: Def, at: Vec3): void { return crossToRust('loose', undefined, [g, D, at]) as any; }






/** the report is heard or called and no room is hit yet (standTo runs first each frame: a fight that is on has ended this) */
const awaiting = (st: State) => PRESSURE.warned && !!st.warn && st.warn.phase !== 'over';
/** the fight a man's ears give him to go to. Stood to, a report is nobody's room being hit, however near a mate's post it was: not while it stands, and not
 *  once a room IS hit either (what he heard before the hit is the report still) */
const reportStands = (st: State, D: Def) => { const w = PRESSURE.warned ? st.warn : null; return !!w && (w.phase !== 'over' || (w.callT >= 0 && D.noiseT < w.hitT! - 0.5)); };
const aim = (st: State, F: RoomFight, D: Def): { T: Room; at: Vec3 } | null => reportStands(st, D) ? null : target(F, D);



/** the fight is over (RoomFight.stop): every press forgotten. The counts stay for whoever reads them after the run; a new fight starts them again. */
export function pressStop(g: Game) {
  const st = STATES.get(g); if (!st) return;
  for (const gd of g.guards) releaseMan(gd, OWN.press, g.time);
  st.presses = []; st.of.clear(); st.outs = []; st.think.clear(); st.stay.clear(); st.called.clear(); st.stoodT = -100; st.sent.clear(); st.doneAt.clear(); st.last.clear(); st.tried.clear(); st.ear.clear(); st.warn = null; st.told.clear(); st.heard = []; st.marks.clear(); st.lastHeld.clear(); st.word = null; st.swept = -Infinity; st.look = null; st.out = null; st.lookLost = false; st.lookT = -Infinity; st.stale = true;
}
export function pressureStats(g: Game): Record<string, number> {
  const s = STATES.get(g)?.stats ?? newStats(), f = g.fight.stats;
  return { ...s, pressLead: +s.pressLead.toFixed(2), flankLead: +s.flankLead.toFixed(2), warnLead: +s.warnLead.toFixed(2), stockRallied: f.rallied, stockFellBack: f.fellBack, stockTookCover: f.tookCover };
}
export function pressureSummary(g: Game): string {
  if (!PRESSURE.on) return 'off';
  const st = STATES.get(g); if (!st) return 'on · nothing yet';
  const s = st.stats;
  return `${['press', 'flank', 'fallBack', 'second'].filter(k => (PRESSURE as any)[k]).join(' + ') || 'nothing switched'} · ${st.presses.length} live · ${s.pressFormed} presses, ${s.pressWords} words, ${s.flankSent} round, ${s.fbPush + s.fbLast} fell back${PRESSURE.second ? `, ${s.secondStood} stood` : ''}`;
}

/** what a visit keeps of this (read once, at import, below; the headless harness has no store) */
const KEPT = ['on', 'press', 'flank', 'second', 'maxMen', 'organiseK', 'warned', 'standPress', 'picket', 'byWay', 'wayOut', 'rescueCrew', 'drilled', 'squadDrill', 'sweep'] as const;   // (not `stuns`: it is hard's on one level — difficulty.ts HARD_AT — and a stored copy would follow the machine to every other)
const LS = 'twopointsix.pressure';
export const PRESSURE_DEFAULTS: typeof PRESSURE = JSON.parse(JSON.stringify(PRESSURE));
export function loadPressure() {
  try { const o = JSON.parse(globalThis.localStorage?.getItem(LS) ?? 'null'); if (o) for (const k of KEPT) if (typeof o[k] === typeof PRESSURE[k]) (PRESSURE as Record<string, unknown>)[k] = o[k]; } catch { /* no store, or not ours: the build's values */ }
}
/** the shipped values back, and the kept copy dropped */
export function resetPressure() {
  Object.assign(PRESSURE, JSON.parse(JSON.stringify(PRESSURE_DEFAULTS)));
  try { globalThis.localStorage?.removeItem(LS); } catch { /* no store */ }
}
export function savePressure() {
  try { globalThis.localStorage?.setItem(LS, JSON.stringify(Object.fromEntries(KEPT.map(k => [k, PRESSURE[k]])))); } catch { /* no store */ }
}
loadPressure();   // (read at import, as digin.ts reads its own: the boot card's difficulty row is up before main.ts gets to its flags; headless there is no store)
