// DEFENDER SQUADS (the Door Kickers maps goal): on a level whose room clear structure declares `squads` (scene/scenarios.ts SquadData — who and where, never
// what they do when), the defenders are not one pool. Each squad holds its rooms until the team is on it; when the team is heard or met ELSEWHERE, each other
// squad makes ONE choice a player can read, said on their net before anybody moves:
//
//   BARRICADE  hold the rooms and get ready: the men stay, shut and wedge the squad's doors and take cover on them (digin.ts does the furniture — run for the
//              squad's men whatever the difficulty row says; only doors of the squad's `barricade` list are wedged). '<Squad>: holding — lock the doors!'
//   FLANK      a PAIR runs round to a door of the room under attack that the team is not known to be at — never across that room, never past where the team
//              was last seen or heard — and goes in from there (a fix out on open floor: by another way than the straight one); the rest of the squad
//              barricades. '<Squad>: two going round — keep them there!', their boots on the way (hearing events the team's ears take), '<Squad>: in
//              position — on them!'.
//   HELP       up to `helpMen` go straight at the fight on the fight AI's own hunt (the stock rally's hand-over: pursuit tactics, doors and all), a man kept
//              in every room the squad holds. '<Squad>: moving to <Ally>!'
//
// WHICH is the situation, never a die: `mayLeave: false` barricades; a squad with fewer than two men to spare past one a room barricades; a squad with men
// still out barricades. A squad in contact that still has two or more men up can hold the team where it is: the first squad to answer sends its pair ROUND;
// one down to its last man (or with no way round to it) gets men STRAIGHT; the second squad to answer does the other thing, a third locks down. A squad
// that has stopped answering: a strong squad (four at home) sends a pair round to where it was, anybody else locks down. The team heard on open floor with
// nobody in contact (surprise lost before the first door): nobody walks out at a noise he cannot place — every squad gets on its doors ('heard it — on the
// doors, call it when you see them!'), and STANDS TO: the next contact call on the net is answered after `readyBeat`, with no thinking beat (`openPair` =
// the first cut, a pair sent round to the noise: it walked onto the team's guns). So several squads hearing one fight neither all pile onto it nor all sit. What varies run to run is the men's own beats (a squad's thinking time is drawn like
// every reaction in the room fight), who is still up when the beat ends, and where the team is. A squad decides again when the fight moves to another squad.
//
// GOING ROUND WHERE A WAY ROUND EXISTS (SQUADS.goRound, on; off = everything above exactly, run for run — the squads-go-round goal: on the small-room maps a
// pair went round in 7, 19 and 11 of 144 scripted runs). Four things the rule above could not do in a room 5 m across whose two men are down before any
// squad has spoken: (1) the LAST-MAN gate is gone — a way round is asked for whatever the squad in contact has left, and men go straight only when there is
// none; (2) the team's door is the one the CONTACT CALL named (the men in the room know which door went in; ears through two walls do not), and a place the
// team was seen or heard closes a path, a spot or a door only with no wall between (roundCtx, wayRound2) — never a bare radius through the wall of the room
// the bang went off in; the search keeps out of every line of fire it knows of, a doorway's own lane included, and every spot outside a door is tried;
// (3) a man sent round WALKS THE WAY HE WAS GIVEN, mark by mark — handed the spot alone he ran to it by the shortest path, which is the one past the team's
// door; (4) a room the team already HAS (the squad that held it has one man up in it, or none) is not gone into by two men through its other door: the
// pair HOLDS that door from outside — '<Squad>: two going round — cut them off!', '<Squad>: in position — we have the door!' — from beside it, or from
// short of the open ground in front of it where the way there has to cross that; they are their squad's men there (digin.ts leaves them be) until the room
// fight moves them, their own rooms are hit, or `holdSecs` of quiet. The timeline says what became of each man sent ('met', 'lost').
//
// WHAT A SQUAD KNOWS is what its men see and hear, and what the net says (one model for the picture and the AI): it is in CONTACT when a man of it at home
// has eyes on the team, hears a loud thing inside the squad's own rooms (a boot on its door, a charge, a round), or drops there with a mate at home to know
// it; its call ('<Squad>: contact — <room>!') needs a man able to make it, and is what the other squads act on after `netLag`. Where the team IS, for a way
// round, is where defenders last saw or heard it — never its true position.
//
// A hit squad's men in its OTHER rooms come to the room under attack after their own beat. A man who gives ground (roomfight.ts breakOff) goes to the squad's
// `rally` point when it is in a room of the squad's the team is not known to be in; a `mayLeave: false` squad's men never leave its rooms (but, stayask.ts: on a floor
// where EVERY squad stays, one man of a squad walks to the posts of a squad that has gone silent on the net).
//
// WHAT THIS FILE OWNS: who of a squad may leave his room, and when. So on a squads level the stock rally (roomfight.ts) and the harder defenders' press
// (pressure.ts consider) do not take a squad's man — one clause each — and digin.ts waits for the squad's word before a man starts on his door. Everything
// else of both files runs as it does (the fall-backs, the wedge, the cover, the cues). No squads block, or SQUADS.on false: nothing here runs, no die is
// drawn, and every seam returns on its first line — the game as it was, run for run.
//
// NOT BUILT: `arrive` squads (they validate and load; nobody comes — sixteen skinned characters are all on the floor with eleven defenders, and taking
// bodies away mid-run is the room clear's bookkeeping to change first).
// (src/game is one import cycle: nothing below is built from another game module's exports at module load.)
import { SideMap, rollbackVar, GameSideMap } from '../math/side';
import { Vec3 } from '../math/vec';
import { type Die } from '../math/dice';   // (dice kept in state carry their word on an object: a rollback snapshot holds it — a mulberry32 closure's it cannot)
import type { Game, Guard } from './game';
import type { Door } from './doors';
import type { RoomFight, FightSkill } from './roomfight';
import type { SquadData, StructureData, StructureRoom } from '../scene/scenarios';
import { squadBarricade } from '../scene/scenarios';
import { eyesFix } from './belief';
import './guards';
import './chronicle';
import './cues';
import { inBounds } from './squad';
import { PRESSURE, run } from './pressure';
import './lanes';
import './digin';
import './dialogue';
import { wg } from './words';
import { netFeedBody } from './netquietfeed';   // (a leaf: netquiet.ts itself is not to be imported from here — see that file's head)
import { stayAskHere } from './stayask';   // (squads that stay in their rooms still miss a room that goes silent: read at staging and in netSend)
import { claimMan, releaseMan, hasHim, ownerWitness, OWN } from './owner';
import { crossToRust, goneWithTick } from './crossing';

export const SQUADS = {
  /** off = a squads block is ignored: the level's defenders are one pool, as on any level without one */
  on: true,
  /** s between a squad learning where the team is and its word on the net (drawn once per decision) */
  think: [2, 4.5] as [number, number],
  /** s between a squad's contact call and the other squads acting on it */
  netLag: 0.8,
  /** men a squad sends to help, at most · the men of a flank */
  helpMen: 3, flankMen: 2,
  /** s before a squad that has made its choice weighs the same fight again · s after a choice before ANY news is weighed */
  againSecs: 20, settleSecs: 8,
  /** s with nothing of the team seen or heard in its rooms before a squad in contact is holding again · s between two calls of one contact */
  quietSecs: 8, recallSecs: 20,
  /** a hit squad's men in its other rooms: their beat before they come */
  reinforce: [1, 2.5] as [number, number],
  /** the team heard on open floor, nobody in contact. Off (shipped): no squad sends men at a noise — every squad gets on its doors, and the next contact call
   *  on the net is answered after `readyBeat` instead of a thinking beat. On = the first cut: the squad next to the nearest one sends its pair round to the
   *  noise (measured: the pair walks onto the team's guns in the open — 42 of 144 scripted runs lost two men that way before the first door) */
  openPair: false, readyBeat: 0.4,
  /** GOING ROUND WHERE A WAY ROUND EXISTS. On (shipped): (1) a squad asks for a way round when the squad in contact is down to its LAST man too (off: it went
   *  straight to him without asking — and a two-man room is down to its last man before any squad has spoken); (2) which door of the room is the team's is
   *  what the NET said (the contact call names the door the team was met at), and a place the team was seen or heard counts against a path, a spot or a door
   *  only with NO WALL between them (col.shotBlocked — off: three bare radii, 3 / 5 / 4 m, through walls: nothing outside a room 5 m across is 5 m from a
   *  flashbang inside it): a place somebody SAW the team, or heard it through open air, has a line of fire `fireM` long, one heard through a wall `softM`,
   *  the stack outside the team's own door `rearM`, an open doorway of the room `fireM` from a man's place inside it — and a doorway's lane out to `mouthM`
   *  is dear to the search whether its leaf is shut or not; (3) every spot outside a door is tried, not the first alone; (4) the man walks the way that was
   *  picked for him, not the shortest path to its end; (5) `holdLost`: a room its own squad has one man up in, or none, is not gone into by the pair — its
   *  other door is held from outside: from beside it, or — the way to it crossing open ground in a doorway's lane — from short of that ground, where he has
   *  a line on the door inside `holdBackM`; he holds it `holdSecs` of quiet at most; (6) the squad's timeline says what became of each man sent ('met' = he
   *  ran into them short of his spot, 'lost' = he went down, and where). Off = the rule as it was, run for run. */
  goRound: true, fireM: 10, rearM: 6, softM: 3, mouthM: 18, holdLost: true, holdBackM: 16, holdSecs: 40,
  /** HARD ON A SQUADS LEVEL: A DRILLED GARRISON. Read only with harder defenders on (pressure.ts PRESSURE.on — the difficulty row's hard — and its row
   *  `squadDrill`) and squads staged: standard, and hard on a level with no squads, never reach a row of it. The same eleven, the same three choices, the
   *  same words on the net; what changes is how well drilled they are, each rule its own switch (record
   *  rebuild/hard-everywhere/round2-squad-maps.md — which of them bite, and which are shipped off):
   *  `drillNet`    THEY LET THE TEAM COMMIT: × `netK` (2) the thinking beat (`think`) and the net's lag (`netLag`) of a squad that could send men out
   *                (`mayGo`: allowed to leave, nobody of its own still out, two to spare), and × `reinforceK` the beat of a hit squad's men in its
   *                other rooms before they come (`reinforce`). A drilled garrison does not run at the first noise: a squad that hears of a fight and could go says so
   *                ('<Squad>: hold — let them come in. Wait for my word!', cues.ts) and takes twice the beat before it chooses and goes, stood to or not,
   *                so the pair that goes round is set at the other door while the team is going in, instead of being met on its way. A squad that
   *                stays in its rooms whatever is called answers as at standard. THE ONE RULE SHIPPED ON. (netK under 1, the first cut — quicker on
   *                the net — measured in the TEAM's favour: sent sooner, the pair walks onto guns that are still up.)
   *  `drillAngles` THEY HOLD THEIR ANGLES: a man's eyes do not wander off the door he watches (roomfight.ts FIGHT_SKILL.hostile `hold` / `glance`, the
   *                untrained man's habit: every 5 to 11 s a look of 1.2 to 2.6 s at another door).
   *  `drillReact`  QUICKER OFF THE MARK: × `reactK` his beat from 'that is a man' to lining him up (`react` 0.4‥1.1 s) and the extra beat of a man caught
   *                looking the other way (`surprise` 0.5‥0.9 s). The team's own row is 0.12‥0.22 s and is not touched; how well he shoots is not touched.
   *  `drillNerve`  STEADIER UNDER A BANG: × `nerveK` the chance a flash makes him duck for its length instead of firing blind down the door he watched
   *                (`bangBreak` 0.3) and the daze once his eyes clear (`daze` 1‥2 s). He is blind exactly as long and doubled over exactly as long.
   *  `drillFollow` THE PAIR FOLLOWS: two men holding the other door of a room the team has taken ('in position — we have the door!') hear the team's next
   *                entry go in somewhere else (a boot, a charge, gunfire: `followM` or more from their door, not in that room nor within 3 m of it) and come after it together
   *                — '<Squad>: they've moved on — after them!', then their boots.
   *  `drillOwn`    NO MAN FED INTO A LOST ROOM: a hit squad's man in another of its rooms does not walk into the room under attack once fewer than `ownUp` (1: nobody; 2 was a wall on the compound) of
   *                his squad are up in it — he gets ready on his own door ('<Squad>: <room>'s gone — stay on your doors, make them come!'). */
  drillNet: true, netK: 2, reinforceK: 2, drillAngles: false, drillReact: false, reactK: 0.75, drillNerve: false, nerveK: 0.5, drillFollow: false, followM: 6, drillOwn: false, ownUp: 1,
  /** BACK ON THEIR OWN DOORS (off; a map's own row — DRILL_AT): two men holding the other door of a room the team took hear the team go on somewhere else (the
   *  loud thing `drillFollow` waits for: `followM` or more from their door, not in that room) and give the door up — back to the posts they left, in their own
   *  rooms, their eyes toward the noise; or the squad they went round for stops answering (every man of it down, `backBeat` s of silence), and they do the same. Standard's pair stays on a door nobody is behind, out in the open, until the team comes on it from the side. Said
   *  once on their net: '<Squad>: they've left it — back on our doors!'. With `drillFollow` on as well, this one has them */
  drillBack: false, backBeat: 3,
  /** GO AND LOOK (on; read only with pressure.ts `PRESSURE.sweep` up as well — hard's row HARD.sweep, the one switch of the rule on every kind of level): the
   *  garrison has had word of the team — a call on the net, one of them properly in a man's sight, a loud thing in a man's ears — and then nothing for
   *  `sweepSecs`. Standard's squads sit on their doors from there for as long as the team cares to take. A drilled garrison sends a pair to look: of the
   *  squads that have nobody out and can spare two (a man left in every room that had one; `sweepMen` is never fewer than two), the one nearest the place
   *  that word put the team calls it on the net — '<Squad>: gone quiet — two of us go and look!', the cue, `sweepBeat` before either moves — and lets the
   *  two go at it on the fight AI's own hunt (the stock rally's hand-over: their boots are theirs to be heard). Calm again out there with nothing found,
   *  they walk back to the posts they left ('<Squad>: no sign of them — back on our doors!'). One look per word, never oftener than every `sweepSecs`;
   *  nobody goes to a room a man of theirs holds, while any squad is in contact, deciding or has men out, on his way to cover or wedging a door, or with a
   *  covered lane known that way; and A LOOK THAT LOSES A MAN IS THE LAST — no squad sends another that fight.
   *  `sweepStayers` (on): a squad the level tells to stay in its rooms (`mayLeave: false` — it answers no other squad's fight) still sends its look: the pair
   *  is the squad finding out where the team went, not a walk to somebody else's fight. Off = such a squad never leaves, hard or not (Mousehole, the lessons). */
  drillLook: true, sweepSecs: 20, sweepMen: 2, sweepBeat: 1.5, sweepStayers: true,
  /** EACH MAP ITS OWN SET (on): the rows `DRILL_AT` names for the structure the game stands on are read in place of the ones above — what bites on one
   *  squad map helps the team on another (measured), so hard is not the same rule on all four. Off = the rows above on every squad map */
  drillByMap: true,
  /** THE WAY HE COMES BY (on; words only — nobody walks, waits or shoots differently): a hit squad's man who answers 'coming to you' from another of its
   *  rooms says by which of the level's own rooms or places his walk comes in ('Alpha: coming to you through the north gallery — hold on!', or 'from bunks 2'
   *  when he walks straight in from where he stands) — dialogue.ts wayName, read off the plain nav path to the place the squad was hit. With the Defender
   *  cues off, on the office, or on a level that names none of its floor: the line as it was. Off = the line as it was */
  way: true,
  /** UNSEEN DROPS ARE FOUND BY RADIO CHECK OR BODY (off; `?radiocheck`, harness `SQUADS=radiocheck`; record rebuild/quiet-squad-maps/change.md).
   *  Off = the rule as it was, run for run: a squad is in contact the frame a man of it drops in its rooms with a mate at home, whether or not anybody saw
   *  or heard it. On: the drop itself tells nobody. A squad that is not already in contact learns of a man down in its rooms one of three ways —
   *  (a) A MATE'S EYES: a fit man of the squad has the body in his open view (the floor's own body test: 12 m, his cone, a line, the light on it — guards.ts
   *      bodyInView's, per body) — on the frame he drops or any frame after: contact, exactly as before ('<Squad>: contact — <room>!', its other men coming);
   *  (b) THE NET'S CHECK: every `checkEvery` s (± `checkJitter` of itself, × `checkAlertK` once the squad has heard of the team;
   *      each squad its own clock, the first inside one beat of the staging) a man of the squad at home calls '<Squad>: radio check — sound off'; `checkAsk` s on, a man who has not
   *      answered (down unknown to them, or in Sam's arm) is asked for by name ('<Squad>: Tango 8? … Tango 8, sound off' — the cue); `checkSend` s after
   *      the call up to `lookMen` of its men at home WALK to where he stood ('<Squad>: nothing from Tango 8 — two moving to his post!', their boots filed
   *      for the team's ears). No call goes out on the net: no other squad moves. The man who gets there looks about him for a second: the body seen is (a). No body
   *      to see (it was carried off, or he is in Sam's arm somewhere else): '<Squad>: no sign of Tango 8 — back on our doors!', they walk back to their
   *      posts and hold them as before, wary but not roused, and the squad is ready: its answer to the next contact call comes without the thinking
   *      beat (Squad.ready);
   *  (c) A MATE'S EARS: a man of the squad at home who hears a small thing (the body going down, a suppressed round) within `checkHearM` of where he
   *      dropped does not know a man is down — he asks for him by name after his own beat (`checkHeard` s) instead of at the next check, and (b) runs
   *      from there. `heardKnows` on = that sound alone is contact at once (the rule as it was, for a drop somebody heard); `heardAsks` off = it leads
   *      nowhere but his own turn of the head (the ordinary hearing model) and the next routine check.
   *  A drop a mate at home heard GUNFIRE with, or a stun charge or a door blown in (in the second and a half before it, or the second after), is contact at once, as it was.
   *  Not covered (as before the switch): a man who drops outside his squad's rooms, or out on a squad's errand — nobody misses him.
   *  A squad already in contact knows its men as before (a drop there is the old rule). A loud thing in its rooms and eyes on the team are untouched. */
  radioCheck: false, checkEvery: 40, checkJitter: 0.2, checkAlertK: 0.5, checkAsk: 4, checkSend: 8, checkHeard: [1, 2] as [number, number], checkHearM: 4, lookMen: 2, heardKnows: false, heardAsks: true,
  /** A GLIMPSE NOBODY HAS MADE OUT IS NOT CONTACT (off; the second piece under `?radiocheck`). Off = the rule as it was: a squad is in contact the frame one
   *  of its men at home has a hard sighting of the team (guards.ts stamps `lastSeenT` once his sight gain passes 0.25/s — whatever his awareness: 0.01 and
   *  still on patrol will do). On: the squad calls it on his eyes only once HE has noticed — his own state is 'alert' (his awareness passed 0.85, where he
   *  himself shouts and starts to fight) or 'search' (he had a man and lost him). Nothing about the man changes: he notices exactly as fast as before; the
   *  squad only stops running ahead of him. A loud thing in its rooms is untouched. */
  noticedEyes: false,
  /** A CONTACT CALL TAKES A BEAT AND CAN BE CUT OFF (off — and off under plain `?radiocheck`: `?radiocheck=call`, harness `SQUADS=radiocheck-call`, turn both
   *  pieces above on and this one; record rebuild/quiet-squad-maps/call-beat.md). Off = the rule as it was: a squad is in contact the frame
   *  ONE of its men has noticed the team, or has a mate's body in his eye — what he alone knows, the whole net knows that frame. On, ON A QUIET FLOOR (the
   *  squad has never been in contact, has had no word of the team on the net or by its own ears, and no man of it up has heard gunfire, a boot on a door
   *  or a charge): what one man's eyes found is HIS until he has said it. He keys up — '<Squad>: <callsign> — I've got—', a row on the strip and a cue
   *  over him, never his own bubble — and the squad is in contact when his call ends, `callSecs` later (drawn on the squad's own dice, never the game's):
   *  the contact call as it always was, said by him. Dropped, taken, blinded or reeling before that, the call is cut off and the squad has been told
   *  nothing of the team — but half a sentence went out on its own net: with the radio check on and a fit mate left, a caller who is off the net (down or
   *  taken) is asked for BY NAME `callCutAsk` 1‥2 s later, and men go to where he was if he does not answer (the check's own ask; a check already under
   *  way, men out looking or the squad's word due: he is asked for as soon as that is done; [0, 0] = no ask at all, the knob as it was first built — a
   *  drop in its rooms is then the next routine check's to find, one outside them nobody's). A mate who saw or heard it happen goes by his own senses and the rules above. Every man
   *  who notices has his own call and his own clock; the first to end is the squad's. THE MAN HIMSELF IS NOT TOUCHED: he notices, moves and shoots exactly
   *  as he did — and his first round is gunfire a living mate in earshot hears, which ends the quiet: from the frame the floor is not quiet any call on
   *  the air is through at once, and every later one is instant, as it was (a man does not hear his own shot: one with no living mate in earshot says
   *  his call to its end). A loud thing in the squad's rooms and a drop with gunfire are untouched.
   *  `callBody` (on): a mate's body SEEN (the radio check's eyes, a man sent to look included) is one man's knowledge too and takes the same beat;
   *  off (`?radiocheck=call-eyes`, `SQUADS=radiocheck-call-eyes`) = only eyes on the team do, a body seen is the squad's at once. */
  callBeat: false, callSecs: [1.5, 2.5] as [number, number], callBody: true, callCutAsk: [1, 2] as [number, number],
  /** THE NET ASKS AFTER A SQUAD THAT HAS GONE SILENT (off — and off under every other value: `?radiocheck=net`, harness `SQUADS=radiocheck-net`, turn the
   *  three pieces above on, the cut-off ask, and this one; record rebuild/quiet-squad-maps/net-ask.md). Off = each squad counts its own men
   *  and no squad ever asks about ANOTHER: a whole squad can be taken, quietly, and nobody is told. On, while no squad is in a fight or has its word due:
   *  (a) THE NET'S OWN CHECK: every `netEvery` s (± `checkJitter` of itself, × `checkAlertK` once anybody is wary; its own dice) a man at home of the
   *      first squad with one calls '<Squad>: all squads — radio check'. A squad with a man on the net has answered. `checkAsk` s on, a squad nobody
   *      answered for is asked for BY NAME ('<Squad>: Dock? … Dock, sound off' — the cue), by a man of the squad nearest it.
   *  (b) TROUBLE HEARD AND NOTHING AFTER: a squad's own trouble on the air — a call that broke off, a man asked for by name, men sent to a post — with no
   *      word of how it ended is asked about `netBeat` s later ('<Squad>: Dock — what's going on?'). A man of it on the net answers a breath later with
   *      what he knows ('<Dock>: still looking — stand by' · '<Dock>: still nothing from Tango 9' — and every squad is wary · '<Dock>: all up here —
   *      nothing more'); a squad is asked that twice at the most. Nobody left to answer: the ask by name of (a), at once.
   *  (c) NO ANSWER: `checkAsk` s after the ask by name every squad is WARY — stood to (Squad.ready: its answer to a contact call comes without the
   *      thinking beat) and counting its own men at the quicker beat — and `netLookMen` men of the nearest squad that can spare them WALK to that
   *      squad's posts, one after the other ('<Squad>: nothing from Dock — two moving to their rooms!', their boots filed for the team's ears). A squad
   *      can spare them when it is allowed out of its rooms and holding, has no trouble of its own (nobody of it asked for, out, cut off, or sent and
   *      not heard of since), and has that many past a man in every room and one at home on the net (not a man still standing on a door he went round
   *      to): never fewer than `netLookMen`. No such squad: '<Squad>: nothing from Dock — nobody to send, stay on your doors!', and Dock is asked for
   *      once more at a later roll call (not at all if only one squad is left to ask). Not in contact: nobody has seen anything. A body of that squad
   *      in a looker's eye, or the team in it, is his to call like any man's (the call beat: it can be cut off); nothing at the posts: '<Squad>: no
   *      sign of Dock — back on our doors!' and nobody asks after that squad again.
   *  (d) THE LAST MAN DOES NOT WALK OFF ALONE: a man who is the last of his squad at home as far as he knows (every other is out, known down, or was
   *      asked for and did not answer), whose ask by name got no answer, says so on the net ('<Dock>: nothing from Tango 10 — I'm alone here, somebody
   *      look!') and the men of (c) go to that post in his place ('copy, Dock — two moving to you!', a breath later; a second man missing while they
   *      are on their way: 'nothing from Tango 9 either — his post too!'). No squad able to send: he says that ('… I'm alone here, going myself!'),
   *      the squads go wary, and he goes.
   *  (e) A MAN THE NET SENT WHO KEYS UP AND IS CUT OFF IS THE ALARM (`netCutAlarm`, read only with the net's ask on): he was sent to find out why a
   *      squad is silent, his squad heard him start to say what he had found, and then nothing. A man of it at home calls it: '<Squad>: Tango 5 cut
   *      off at Dock — contact, Dock!'. The place on the net is the post the squad sent him to — its own knowledge, not where the team is. Off: his
   *      broken call is one more man to ask for by name, as any cut-off caller's is.
   *  Nobody is told where the team is, by any of it: the places are the squads' own posts. A squad that has put a contact call on the net is not asked
   *  after (everybody knows what became of it: the squads' own words handle it, as they did). */
  netAsk: false, netEvery: 60, netBeat: [6, 9] as [number, number], netLookMen: 2, netStops: 3, netCutAlarm: true,
};
export const SQUADS_DEFAULTS: typeof SQUADS = JSON.parse(JSON.stringify(SQUADS));
try { if (new URLSearchParams(location.search).has('nosquads')) SQUADS.on = false; } catch { /* no page (the harness) */ }   // ?nosquads: one pool for the visit
/** the one flag over the two pieces: '' / anything = both, 'drop' = unseen drops only, 'eyes' = unnoticed glimpses only, '0' = neither */
const senses = (v: string | null) => { SQUADS.radioCheck = v !== '0' && v !== 'eyes'; SQUADS.noticedEyes = v !== '0' && v !== 'drop'; SQUADS.callBeat = v === 'call' || v === 'call-eyes' || v === 'call-noask' || v === 'net'; SQUADS.netAsk = v === 'net'; if (SQUADS.callBeat) { SQUADS.callBody = v !== 'call-eyes'; if (v === 'call-noask') SQUADS.callCutAsk = [0, 0]; } };   // (the third piece, the call's beat, only by its own value: 'call' = both pieces and it, 'call-eyes' = the same with a body seen left instant, 'call-noask' = 'call' with nobody asking after a call that broke off — callCutAsk [0, 0]; 'net' = 'call' and the net asking after a squad that has gone silent — netAsk: everything on)
try { const q = new URLSearchParams(location.search); senses(q.has('radiocheck') ? q.get('radiocheck') : 'call'); } catch { /* no page (the harness) */ }   // THE PAGE OPENS AS ?radiocheck=call (the quiet maps goal's pick for the one build; the harness and SQUADS_DEFAULTS stay off): a squad knows only what its men's senses and its net's check tell it, and a contact call takes a beat. ?radiocheck=0 = off for the visit; ?radiocheck (plain) / =drop / =eyes / =call-noask / =net: that value
/** the page's opening, for a harness stood up as the page is (src/host/profile.ts shippedBoot): `squadSenses('call')` is the line above with no ?radiocheck in the URL */
export const squadSenses = senses;
try { const e = (globalThis as { process?: { env?: Record<string, string | undefined> } }).process?.env?.SQUADS; const t = e ? e.toLowerCase().split(/[\s,+]+/) : [];   // (the harness: SQUADS=radiocheck in the environment — a tool with no --set of its own; radiocheck-drop / radiocheck-eyes: one piece)
  if (t.includes('radiocheck-net')) senses('net'); else if (t.includes('radiocheck-call')) senses('call'); else if (t.includes('radiocheck-call-eyes')) senses('call-eyes'); else if (t.includes('radiocheck-call-noask')) senses('call-noask'); else if (t.includes('radiocheck')) senses(''); else if (t.includes('radiocheck-drop')) senses('drop'); else if (t.includes('radiocheck-eyes')) senses('eyes'); } catch { /* no process (the page) */ }

type Def = RoomFight['defs'][number];
type Room = NonNullable<Def['room']>;
export type SquadChoice = 'barricade' | 'flank' | 'help';
type SquadState = 'hold' | 'alerted' | 'contact' | SquadChoice | 'down';
interface Fix { at: Vec3; t: number; ally: Squad | null; by: 'net' | 'ears' }
interface Sent { gd: Guard; role: 'help' | 'flank' | 'reinforce' | /** (drillLook) one of a pair gone to look where the team was last put */ 'look'; phase: 'run' | 'set' | 'loose'; /** (goRound) the room is lost: he holds the door from his spot, he does not go in */ hold?: boolean; spot: Vec3 | null; door: Door | null; fixAt: Vec3; tries: number; stepT: number; t0: number; /** when he got to his spot unseen (goRound) */ there?: number; /** (goRound) the marks of his way still ahead of him, his spot the last */ way?: Vec3[]; /** (goRound) what he watches, holding a door */ aim?: Vec3; /** (goRound) since when he has held where he is */ setT?: number; /** the post he left when he was sent */ from: Vec3; /** (drillBack) the squad he went round for, when he first knew it was not answering, and that he is on his way back to his post */ ally?: Squad | null; goneT?: number; back?: boolean }
/** one row of a squad's timeline (the tally's and the page's): what it did, when, with how many men at home, and why in a few words */
export interface SquadEvent { t: number; what: 'contact' | 'reinforce' | SquadChoice | 'in position' | 'met' | 'lost' | 'quiet' | 'down' | 'rally' | 'stays' | 'follows' | 'back' | 'sweep' | 'check' | 'asks' | 'looks' | 'found' | 'no sign' | 'glimpse' | 'calls' | 'cut off' | 'said' | 'net check' | 'net query' | 'net asks' | 'net looks' | 'net help'; n: number; why: string }
/** (radioCheck) a squad's net check: when the next is due, the one running (called → asked by name), the men who went quiet in its rooms (where, when, and
 *  what small sounds each mate had heard by then), the ones whose fate it knows (`known`: seen down) or has given up asking for (`looked`: looked for at his
 *  post and not found — a body seen later is still seen), and the men out looking */
interface Check { nextAt: number; stage: 'idle' | 'called' | 'asked'; t0: number; caller: Guard | null; who: Guard | null; silent: Quiet[]; known: Set<Guard>; looked: Set<Guard>; look: Looker[]; n: number; /** its own dice (checkDice) */ die: Die; /** (callCutAsk) men whose call broke off, still to be asked for by name: no sooner than `at` */ cut?: { who: Guard; at: number }[]; /** (callCutAsk) the routine check's own time, kept while `nextAt` is pulled in for a man on `cut` — put back if he comes on the net again before he is asked for */ beat?: number; /** (callCutAsk) a mate's call held up an ask that was due: the men go no sooner after it than after any ask */ late?: boolean; /** (callCutAsk) men a 'cut off' row has said will be asked for and who have not been yet: the trace says so if contact comes first */ promised?: Set<Guard>; /** (callCutAsk) men who stopped being lookers by their own eyes, away from the post they left (`from`): taken there, they are filed there */ afield?: Map<Guard, Vec3>; /** (netAsk) men of its own it has sent to look (for a mate, or at another squad's posts): one of them off the net with his end unknown is a man it is still waiting for */ out?: Set<Guard>; /** (netAsk) men of its own asked for by name who did not answer */ asked?: Set<Guard>; /** (netAsk) men the net has sent from this squad: the squad they went to look at, the post they were sent to, when */ netOut?: Map<Guard, { about: Squad; at: Vec3; t: number }>;
  /** the one running was started by `caller`'s own ears (he heard something where `who` stands): it is his to ask, and it dies with him */ byEar: boolean; /** when it last ran (a gap = the switch was off meanwhile: the check starts over) */ tickT: number }
interface Quiet { gd: Guard; at: Vec3; t: number; heard: boolean; /** filed alive, in somebody's arm (dropped later, he is a new silence) */ held: boolean; small0: Map<Guard, number> }
/** a man walking to a silent mate's post, or back from it: `calm` = his post and how he held it when he left (he is put back on it as he was, wary — he saw nobody) */
interface Looker { gd: Guard; who: Guard; spot: Vec3; from: Vec3; back: boolean; tries: number; stepT: number; t0: number; /** the squad's news (Squad.newsT) as he set off */ news0: number; /** since when he has been looking about him at the post (or as near it as he got) */ lookT?: number; calm: Calm;
  /** (netAsk) he is another squad's man, sent by the net to look at THIS squad's posts: any body of it in his eye is what he has found */ about?: Squad; /** (netAsk) the whole squad is not answering (not one man of it asked for by its last man): he turns back when anybody of it answers */ all?: boolean; /** (netAsk) the posts still ahead of him after `spot` */ stops?: Vec3[]; /** (netAsk) frames walked (his eyes on the way are asked ten times a second) */ n?: number }
/** (callBeat) one man's contact call on the air: the squad's when it ends (`end`), nobody's if he is cut off first. `at` / `room` = where he puts the team
 *  (kept to his latest sighting), or where the body lies; `body` = the mate he has seen down; `eyes` = he has had the team itself in his eye */
interface Call { by: Guard; t0: number; end: number; at: Vec3; room: StructureRoom | null; body: Guard | null; eyes: boolean }
interface Calm { state: Guard['state']; awareness: number; hold: boolean; pinned: boolean; station: Vec3; home: Vec3; room: Def['room']; ev: number[]; watch: number; want: number; pt: Vec3 }
interface Squad {
  def: SquadData; name: string; rooms: StructureRoom[]; men: Guard[]; barricade: Set<string>;
  state: SquadState; /** its last choice, whatever has happened to it since */ state0: SquadChoice | null; fix: Fix | null;
  newsT: number; decideAt: number; decidedAt: number; contactT: number; calledT: number; calledRoom: StructureRoom | null; fallen: Set<Guard>;
  /** stood to on a noise nobody could place: its next word, on a contact call, comes without the thinking beat */
  ready: boolean;
  /** (drillOwn) when it last said a room of its own is gone */ ownSaid?: number; stayed?: Set<string>;
  /** the room of its own the team was last met in (where a squad coming round to it is going) */ hitRoom: StructureRoom | null;
  /** (radioCheck) made on the squad's first tick with the switch on */ rc?: Check;
  /** (callBeat) the calls its men have on the air, and the dice their lengths are drawn on (made at the first call) */ calls?: Call[]; callDie?: Die;
  /** (callCutAsk) men whose call broke off while they were blinded or reeling — on the net still, so nobody asked: one who goes off it inside HUNG_SECS without having keyed up again is asked for after all */ hung?: { who: Guard; t: number }[];
  /** when its trouble was last on the air with no word of how it ended (a call cut off, a man asked for by name, men sent to a post) — what the other squads heard (read only by netAsk) */ trouble?: number;
  /** (netAsk) a squad is not answering the net: stood to, and counting its own men at the quicker beat */ wary?: boolean; /** (callCutAsk) the dice the beat before a cut-off caller is asked for is drawn on — its own, so the call lengths of a run do not depend on whether anybody asks */ askDie?: Die;
  sent: Sent[]; /** (goRound) men who went round and are home again, wherever that is: the timeline still says if one goes down */ went: Sent[]; due: { gd: Guard; at: Vec3; t: number; /** the room of the squad's he is wanted in */ room?: StructureRoom | null }[]; events: SquadEvent[];
}
interface State { /** (stayask.ts) every squad here stays in its rooms: the net's ask runs whatever its switch says, and a squad that stays sends one man to a silent squad's posts */ stayAsk?: boolean; S: StructureData; squads: Squad[]; of: Map<Guard, Squad>; net: { t: number; squad: Squad; at: Vec3; /** the door of its room the team was met at (goRound; null: a man dropped and nobody saw by which) */ door: Door | null }[]; /** (netAsk) made on the first tick with the switch on */ nc?: NetCheck; /** (drillLook) the newest word the garrison has of the team — when, where — and the word the last look went out on */ word?: { t: number; at: Vec3; done: number; /** when a look was last called · a man of a look went down out there (no squad sends another this fight) · the men of the last look while any is still out · the look called and not yet gone */ saidT: number; lost: boolean; out: Guard[] | null; call: { Q: Squad; go: Def[]; at: Vec3; goT: number; w: number } | null } }
/** (netAsk) the net's check of the SQUADS: when the next roll call is due, the one running (called → a squad asked for by name), who asks, the squad asked
 *  about, the squads nobody asks after again (`done`: men went to its posts — whatever they found — or a body of it was said on the net with nobody of it left to answer) */
interface NetCheck { nextAt: number; stage: 'idle' | 'called' | 'asked'; t0: number; caller: Guard | null; about: Squad | null; done: Set<Squad>; /** squads asked for by name with nobody sent after (nobody free, or a fight took the net): asked for once more at a later check, and no oftener */ tried: Map<Squad, number>; /** how often a squad that answers has been asked what is going on */ queried: Map<Squad, number>; /** lines said a breath after what they answer */ say: { t: number; gd: Guard; text: string; cls?: number }[]; die: Die; tickT: number }
const STATES = new GameSideMap<Game, State>('enemysquads.STATES');
ownerWitness<Game>('squad', (gd, g) => !!STATES.get(g)?.of.has(gd));
ownerWitness<Game>('squadLeg', (gd, g) => { const Q = STATES.get(g)?.of.get(gd); return !!Q && squadLegOf(Q, gd); });
ownerWitness<Game>('squadWord', (gd, g) => { const Q = STATES.get(g)?.of.get(gd); return !!Q && squadWordOf(Q, gd); });
/** the net's ask runs on this floor: its own switch, or (stayask.ts) a floor whose every squad stays in its rooms */
const netAsks = (g: Game): boolean => SQUADS.netAsk || !!STATES.get(g)?.stayAsk;
/** hard on a squads level (SQUADS.drill*): the table while harder defenders are on, their `squadDrill` row is, and squads are staged in this game — else null */
/** WHAT HARD IS ON ONE SQUAD MAP, by its structure's name (`scenarios.roomClear.name`): rows of the drill read in place of SQUADS' own while
 *  `SQUADS.drillByMap` is on. A map not named here gets the table's rows. Record: rebuild/hard-everywhere/round2-squad-maps.md */
export const DRILL_AT: Record<string, Partial<Pick<typeof SQUADS, 'drillNet' | 'netK' | 'reinforceK' | 'drillAngles' | 'drillReact' | 'reactK' | 'drillNerve' | 'nerveK' | 'drillFollow' | 'followM' | 'drillOwn' | 'ownUp' | 'drillBack' | 'backBeat' | 'drillLook' | 'sweepSecs' | 'sweepMen' | 'sweepBeat' | 'sweepStayers'>>> = {
  // the squads let the team commit: the pair is set at the other door while the team is still going in (from the threshold 223 of 576 teammates up against 293)
  'the block, three squads': { drillNet: true, netK: 2, reinforceK: 2, drillOwn: false, drillBack: false },
  // nothing on: standard is the floor here already (from the threshold 103 of 576 up, 55 of 144 won), and the squads' wait helps the team (a waiting squad is slower onto its own doors)
  'the warehouse': { drillNet: false, drillOwn: false, drillBack: false },
  // nothing on: four squads of two, each in its one room and none may leave it, so there is nobody to hold back on the net and no pair to send (hard was standard's run
  // on every seed measured with these rows at the defaults too: the row says what the card says)
  'the block': { drillNet: false, drillOwn: false, drillBack: false },
  // a hit squad's men come quicker (x0.75), and nobody is fed into a room with none of his squad up in it: the rooms are held when the team gets to them
  'the compound': { drillNet: true, netK: 1, reinforceK: 0.75, drillOwn: true, ownUp: 1, drillBack: false },
  // a pair holding a lost room's other door goes back to its own posts when the squad it went round for stops answering (at standard it died on that door,
  // in the open, 96 times of 96 and killed 2 teammates of 100); the slow net does nothing here, its pair is set in time at standard already
  'the barracks': { drillNet: false, drillOwn: false, drillBack: true, backBeat: 3 },
};
/** where the game stands: its room clear structure's name ('' = no squads staged) */
export function squadsWhere(g: Game): string { return STATES.get(g)?.S.name ?? ''; }
export function squadDrill(g: Game): typeof SQUADS | null {
  const st = STATES.get(g); if (!st || !PRESSURE.on || !PRESSURE.squadDrill) return null;
  const at = SQUADS.drillByMap ? DRILL_AT[st.S.name] : undefined; if (!at) return SQUADS;
  return { ...SQUADS, ...at };   // (made per read: either table may be edited in place by a probe or the panel)
}
/** the panel's line: which of the drill's rules hard has on where the game stands ('' = no squads staged) */
export function squadDrillWords(g: Game): string {
  const st = STATES.get(g); if (!st) return '';
  const K = squadDrill(g); if (!K) return `${st.S.name}: standard squads`;
  const on = [K.drillNet && K.netK !== 1 && `let you commit first (× ${K.netK} net)`, K.drillNet && K.reinforceK !== 1 && `own men's beat × ${K.reinforceK}`, K.drillOwn && `no man fed into a lost room (mates up ${K.ownUp})`, K.drillAngles && 'hold their angles', K.drillReact && `quicker off the mark (× ${K.reactK})`, K.drillNerve && `steadier under a bang (× ${K.nerveK})`, K.drillFollow && !K.drillBack && 'pair follows the next entry', K.drillBack && 'pair gets back on its own doors', K.drillLook && PRESSURE.sweep && 'a pair goes to look when you go quiet'].filter(Boolean);
  return `${st.S.name}: hard = ${on.length ? on.join(' · ') : 'standard (no rule on here)'}${SQUADS.drillByMap && DRILL_AT[st.S.name] ? ' — this map\'s own set' : ''}`;
}
/** roomfight.ts: the defenders' row of FIGHT_SKILL as a drilled garrison has it (drillReact · drillNerve) — `base` itself, the same object, whenever
 *  squadDrill is null or neither is on. (drillAngles is read where the wandering eye is rolled: roomfight.ts applyWatch) */
export function squadSkill(g: Game, base: FightSkill): FightSkill {
  const K = squadDrill(g); if (!K || !(K.drillReact || K.drillNerve)) return base;
  const rk = K.drillReact ? K.reactK : 1, nk = K.drillNerve ? K.nerveK : 1, x = (r: [number, number], k: number): [number, number] => [r[0] * k, r[1] * k];
  return { ...base, react: x(base.react, rk), surprise: x(base.surprise, rk), bangBreak: base.bangBreak * nk, daze: x(base.daze, nk) };   // (made per read: the table under it may be edited in place by a probe or the panel)
}

const flat = (a: Vec3, b: Vec3) => Math.hypot(a[0] - b[0], a[2] - b[2]);
const alive = (gd: Guard) => gd.state !== 'dead' && !gd.fled;   // (runner.ts: a man who dropped it and ran is no man of his squad's any more)
const grown = (b: StructureRoom['bounds'], m: number) => ({ x0: b.x0 - m, x1: b.x1 + m, z0: b.z0 - m, z1: b.z1 + m });
const roomOf = (Q: Squad, p: Vec3, m = 0) => Q.rooms.find(r => inBounds(p, m ? grown(r.bounds, m) : r.bounds)) ?? null;
const note = (g: Game, gd: Guard | null, what: string) => { const L = g.fight.log; L.push({ t: g.time, who: gd ? gd.callsign : '-', what: `[squads] ${what}` }); if (L.length > 600) L.splice(0, 200); };
/** every half metre along a path, its corners included (a path's waypoints are corners: what a leg passes is asked of the leg); returns its length */
function along(from: Vec3, path: Vec3[], each: (p: Vec3, s: number) => void): number {
  let q = from, run0 = 0;
  for (const w of path) { const L = flat(q, w), n = Math.max(1, Math.ceil(L / 0.5)); for (let i = 1; i <= n; i++) each([q[0] + (w[0] - q[0]) * i / n, 0, q[2] + (w[2] - q[2]) * i / n], run0 + L * i / n); run0 += L; q = w; }
  return run0;
}

/** WHAT THE SQUADS HAVE, AS THIS FRAME'S TICK LEAVES IT (src/game/owner.ts; nothing outside the tick changes a squad's lists). `OWN.squadLeg`: out on a leg, or about to be sent, so not free to be sent anywhere else. `OWN.squadWord`: his squad has heard and not yet said, he is about to go, on his way round, or looking for a man who does not answer: nothing else starts on him */
const squadLegOf = (Q: Squad, gd: Guard) => Q.sent.some(s => s.gd === gd) || Q.due.some(d => d.gd === gd);
const squadWordOf = (Q: Squad, gd: Guard) => Q.state === 'alerted' || Q.due.some(d => d.gd === gd) || SQUADS.goRound && Q.sent.some(s => s.gd === gd && s.role === 'flank' && s.phase !== 'loose') || !!Q.rc?.look.some(l => l.gd === gd);
/** the staging is over: nobody is its man */
function squadsLetGo(g: Game, st: State | undefined): void { for (const Q of st?.squads ?? []) for (const m of Q.men) { releaseMan(m, OWN.squad | OWN.squadLeg | OWN.squadWord, g.time); m.staysInRooms = false; } }
// ---------------------------------------------------------------- staging
/** roomclear.ts onStaged: the squads of this staging — each defender is the squad's whose room he was dealt into (`rooms[i]` = defender i's room id). A
 *  structure with no squads block, or the switch off, stages none (and drops any from the last staging). */
export function squadsStage(g: Game, S: StructureData | null, hostiles: Guard[], rooms: string[]) {
  squadsLetGo(g, STATES.get(g)); STATES.delete(g);
  if (!SQUADS.on || !S?.squads?.length) return;
  const squads: Squad[] = [], of = new Map<Guard, Squad>();
  for (const def of S.squads) {
    if (def.arrive) continue;   // (late squads: not built — see the file head)
    const mine = S.rooms.filter(r => def.rooms?.includes(r.id)), men = hostiles.filter((_, i) => def.rooms?.includes(rooms[i]));
    const Q: Squad = { def, name: def.name ?? def.id, rooms: mine, men, barricade: new Set(squadBarricade(def, S.rooms)),
      state: 'hold', state0: null, fix: null, newsT: -100, decideAt: 0, decidedAt: -100, contactT: -100, calledT: -100, calledRoom: null, fallen: new Set(), ready: false, hitRoom: null, sent: [], went: [], due: [], events: [] };
    squads.push(Q); for (const m of men) { of.set(m, Q); claimMan(m, OWN.squad, g.time); m.staysInRooms = def.mayLeave === false; }
  }
  if (squads.length) STATES.set(g, { S, squads, of, net: [], stayAsk: stayAskHere(g, squads.map(Q => Q.def)) });
}
/** the staging is gone (roomclear.ts clear) */
export function squadsStop(g: Game) {
  // (radioCheck: its body tests ask the light meter and the smoke about the squad's own men by id, and unit ids are dealt again — the next cast is not read by
  // the light, or through the smoke, the last one's bodies lay in. Only a staging the switch ran in: with it off these keys are left as they always were)
  const st0 = STATES.get(g);
  for (const Q of st0?.squads ?? []) if (Q.rc) for (const a of Q.men) { g.lightForget(`bodyL${a.char.id}`); for (const b of Q.men) if (b !== a) g.lightForget(`bodyS${b.char.id}_${a.char.id}`); }
  if (st0?.nc) for (const Q of st0.squads) for (const a of Q.men) { g.lightForget(`bodyL${a.char.id}`); for (const X of st0.squads) if (X !== Q) for (const b of X.men) g.lightForget(`bodyS${b.char.id}_${a.char.id}`); }   // (netAsk: a man sent to another squad's posts asked the same of ITS bodies)
  squadsLetGo(g, st0); STATES.delete(g); CTX = null;
}
/** squads are on the floor in this game right now */
export function squadsLive(g: Game): boolean { return STATES.has(g); }
/** the squad a man is of — its record as the level wrote it and its men in their staged order (morale.ts: who is green, who leads); null = not a squad's man */
export function squadOf(g: Game, gd: Guard): { def: SquadData; men: Guard[] } | null { return STATES.get(g)?.of.get(gd) ?? null; }

/** roomfight.ts breakOff · pressure.ts lastMan: his squad's men stay in its rooms (`mayLeave: false`) — he gives no room up through a door */
/** roomfight.ts breakOff: where a man who gives ground goes when his squad has a rally point in ANOTHER room of its own than the one he is in, the team is
 *  not known to be in that room, and he can reach it without passing what he is giving ground to — the spot (a free one beside it when a mate is on it), or
 *  null: the room fight's own answer */
export function squadRally(g: Game, gd: Guard, threat: Vec3): Vec3 | null {
  const st = STATES.get(g), Q = st?.of.get(gd); if (!st || !Q || !Q.def.rally) return null;
  const c = gd.char, R = Q.def.rally, here = roomOf(Q, c.pos), there = roomOf(Q, R);
  if (!there || here === there || flat(R, threat) < 4 || teamKnown(g).some(p => inBounds(p, grown(there.bounds, 1)))) return null;
  const nav = g.col.nav;
  for (const [dx, dz] of [[0, 0], [1, 0], [-1, 0], [0, 1], [0, -1], [1, 1], [-1, -1]]) {
    const p: Vec3 = [R[0] + dx * 0.9, 0, R[2] + dz * 0.9];
    if (nav.isBlocked(p[0], p[2]) || !inBounds(p, there.bounds) || Q.men.some(m => m !== gd && alive(m) && flat(m.char.pos, p) < 0.8)) continue;
    const path = nav.findPath(c.pos, p); if (!path?.length) continue;
    let toward = false; const L = along(c.pos, path, q => { if (flat(q, threat) < 2) toward = true; });
    if (toward || L > 30) continue;
    event(g, Q, 'rally', `${gd.callsign} gives ground to the rally point`);
    return p;
  }
  return null;
}

// ---------------------------------------------------------------- the men
const defOf = (g: Game, gd: Guard): Def | undefined => g.fight.defs.find(D => D.gd === gd);
/** his own wits about him: alive, not in Sam's arm, not blinded or reeling */
const fit = (g: Game, m: Guard) => alive(m) && !m.held && g.time >= m.dazzledUntil && g.time >= m.staggerUntil && !hasHim(m, OWN.runner);   // (runner.ts, false with its switch off: from his shout on he makes no call on the squad's net — his own words stand)
/** a squad's men at home and free to be told something: fit, holding a post, nobody of the team in their sight these two seconds, not already sent */
function home(g: Game, Q: Squad): Def[] {
  const out: Def[] = [];
  for (const m of Q.men) { if (!fit(g, m) || g.time - m.lastSeenT < 2 || Q.sent.some(s => s.gd === m) || hasHim(m, OWN.stand) || Q.rc?.look.some(l => l.gd === m)) continue;   // (laststand.ts: a man of the last stand is not the squad's to send)
    if (hasHim(m, OWN.lastMan)) continue;   // (netquiet.ts, through netquietfeed.ts — false on its first line with its switch off: the last man up, at his act, is not the squad's to send to a room nobody of his is left in)
    if (m.script && g.director?.lessonRuns) continue;   // (a staged moment of a tutorial mission's script has him, as roomfight.ts and lanes.ts leave him to it: not at home — sent to look for a mate gone quiet, he walked out of its hand)
    const D = defOf(g, m); if (D && D.mode === 'hold' && D.rallyAt !== Infinity) out.push(D); }
  return out;
}
/** WHERE THE TEAM IS, as far as the defenders know: where each living man of theirs last had one of the team in his sight, or last heard a loud thing —
 *  these few seconds. Never the team's true positions */
function teamKnown(g: Game): Vec3[] {
  const out: Vec3[] = [];
  for (const D of g.fight.defs) {
    const gd = D.gd; if (!alive(gd)) continue;
    const seen = g.time - gd.lastSeenT < 4 ? eyesFix(gd, D.pt) : null; if (seen) out.push(seen);
    if (D.noiseAt && g.time - D.noiseT < 4) out.push(D.noiseAt);
  }
  return out;
}


/** What a way round is judged against, built once a decision and shared by the men it sends (goRound): where the team's guns are as far as this side
 *  knows — the places it was seen or heard (`known`), the stack outside the door the net named (`rear`), and the room's own doorways from a man's place well
 *  inside (`mouths`: a door is a line of fire out of a room the team is in). `hot` = a man standing there is in front of one of them; `lane` = in a doorway's
 *  line whether its leaf is shut or not (a leaf is not a wall: the door a pair set off towards shut was open three seconds later, and the first man was shot
 *  11 m down its line by a man at the back of the room); `cost` = the search's price list (the fight's room all but shut, 4 m round every known place and
 *  the stack dear, a doorway's lane dear, a room somebody else of theirs holds a little dear — wayRound's, plus the lanes). */
interface RoundCtx { t: number; T: Room; key: string; rear: Vec3[]; mouths: Vec3[]; fire: (a: Vec3, b: Vec3) => boolean; hot: (p: Vec3) => boolean; hotCell: (ix: number, iz: number) => boolean; lane: (p: Vec3) => boolean; cost: Map<number, number>; held: Map<number, Room> }
let CTX: RoundCtx | null = null;
rollbackVar('enemysquads.CTX', () => CTX, v => { CTX = v; });   // (a memo keyed by the game's clock: a tick played again must not be handed the first playing's)

// ---------------------------------------------------------------- the squad's word
function event(g: Game, Q: Squad, what: SquadEvent['what'], why: string, by: Guard | null = null) {
  Q.events.push({ t: g.time, what, n: home(g, Q).length, why }); if (Q.events.length > 240) Q.events.splice(0, 80);
  if (netAsks(g)) { if (what === 'asks' || what === 'looks' || what === 'cut off' || what === 'net help') Q.trouble = g.time; else if (what === 'no sign' || what === 'found' || what === 'contact' || what === 'said') Q.trouble = undefined; }   // (what the other squads heard of it, and whether they heard how it ended: a field nothing reads but netAsk)
  note(g, by, `${Q.name} ${what}: ${why}`);
  if (what === 'found') netFeedBody(g, by, Q.men);   // (netquiet.ts, filed through netquietfeed.ts — a return on its first line with its switch off: a body found shakes the finder, and less the men he tells)
}
export const mate = (m: Guard): string => goneWithTick('mate') as any;


/** drillFollow's and drillBack's ears: a holding man hears the team go on somewhere else, and nobody of the pair (his mates on that door within `reach`) has one of them in sight */
function movedOn(g: Game, Q: Squad, s: Sent, D: Def, reach: number): { at: Vec3; pair: Sent[] } | null {
  const gd = s.gd, at = D.noiseAt;
  if (!at || g.time - D.noiseT > 1 || g.time - (s.setT ?? s.t0) < 4 || g.time - gd.lastSeenT < 3 || !fit(g, gd)) return null;
  if (flat(at, s.aim ?? gd.char.pos) < (squadDrill(g) ?? SQUADS).followM || lostRoomHas(g, s, at)) return null;
  const pair = Q.sent.filter(x => x.role === 'flank' && x.phase === 'set' && x.hold && alive(x.gd) && fit(g, x.gd) && (x === s || flat(x.gd.char.pos, gd.char.pos) < reach));
  return pair.some(x => g.time - x.gd.lastSeenT < 3) ? null : { at, pair };   // (his mate has a man in sight: nobody leaves)
}
/** A DRILLED GARRISON (SQUADS.drillBack): the door they hold has nobody behind it any more, and out here they are in the open on the way the team is
 *  coming. Two things tell them: the squad they went round for has stopped answering (every man of it down: `backBeat` s of nothing on the net), or
 *  `drillFollow`'s ears — the team heard going in somewhere else. Every man of the squad still holding that room's doors, nobody of the team in his sight,
 *  goes back to the post he left (a scripted leg, his boots filed for the team's ears), and the room fight plants him there facing where they were last
 *  heard. Said once on the net, by the man who knew. */
function back(g: Game, Q: Squad, s: Sent, D: Def) {
  const gd = s.gd, K = squadDrill(g) ?? SQUADS, gone = !!s.ally && s.ally.state === 'down';
  let at: Vec3, pair: Sent[];
  if (gone) {
    s.goneT ??= g.time;
    if (g.time - s.goneT < K.backBeat || g.time - (s.setT ?? s.t0) < 4 || !fit(g, gd)) return;
    pair = Q.sent.filter(x => x.role === 'flank' && !x.back && x.hold && (x.phase === 'set' || x.phase === 'run') && alive(x.gd) && fit(g, x.gd));   // (a mate still on his way to that door turns round with him)
    if (pair.some(x => g.time - x.gd.lastSeenT < 3)) return;   // (one of them has a man in sight: nobody leaves)
    at = D.noiseAt ?? s.fixAt;
  } else { const M = movedOn(g, Q, s, D, Infinity); if (!M) return; ({ at, pair } = M); }
  const names: string[] = [], door = s.door ? s.door.def.name : 'the door';
  for (const x of pair) {
    const X = defOf(g, x.gd); if (!X || X.mode === 'free') continue;
    x.phase = 'run'; x.back = true; x.hold = false; x.spot = x.from; x.door = null; x.way = undefined; x.tries = 0;
    X.ev.fill(0); X.pt[0] = at[0]; X.pt[2] = at[2];   // (what he faces when he is planted again: where they were last heard, not the door he gave up)
    run(g, X, x.from, null, 0.3); names.push(x.gd.callsign);
  }
  if (!names.length) return;
  g.say(gd, gone ? `${Q.name}: ${s.ally!.name} ${wg("isn't", 'is not')} answering — back on our doors!` : `${Q.name}: they've left it — back on our doors!`, true);
  event(g, Q, 'back', `${names.join(' and ')} leave${names.length > 1 ? '' : 's'} ${door}: ${gone ? `${s.ally!.name} is not answering` : `the team was heard ${flat(at, s.aim ?? gd.char.pos).toFixed(0)} m off, going on`}, ${(g.time - (s.setT ?? s.t0)).toFixed(1)} s after setting`, gd);
}
/** is that place in or within three metres of the room a holding man's door leads to (the room the team took): a noise in its other doorway, in the corridor
 *  along its wall or heard a few metres off through the wall is still the fight for THAT room — the team working it from its other door has not moved on */
function lostRoomHas(g: Game, s: Sent, at: Vec3): boolean {
  if (!s.door) return false;
  const F = g.fight, here = F.roomAt(s.gd.char.pos), p: Vec3 = [0, 0, 0];
  for (const r of [0, 1.5, 3]) for (let k = 0; k < (r ? 8 : 1); k++) {
    p[0] = at[0] + Math.cos(k * Math.PI / 4) * r; p[2] = at[2] + Math.sin(k * Math.PI / 4) * r;
    const there = F.roomAt(p); if (there && there !== here && (there as Room).entries.some(e => e.door === s.door)) return true;
  }
  return false;
}

/** when a stun charge, or a door blown in, last reached this man's ears — as loud with a drop as gunfire is (a charge that fells the man behind the door is
 *  no silent drop). His Witness counts them and does not time them: the count is read once a frame for every man of a squad with the switch on (radioCheck),
 *  so it is seen to move the frame after he heard it; a gap in the reading (the switch was off meanwhile) starts over */
const BANGS = new SideMap<Guard, { n: number; t: number; read: number }>('enemysquads.BANGS');

const GLIMPSED = new SideMap<Guard, number>('enemysquads.GLIMPSED');

/** tools and the panel: the calls on the air in this game right now (who, since when, until when, about what) */
export function squadCalls(g: Game): { squad: string; by: Guard; t0: number; end: number; body: Guard | null }[] {
  return (STATES.get(g)?.squads ?? []).flatMap(Q => (Q.calls ?? []).map(c => ({ squad: Q.name, by: c.by, t0: c.t0, end: c.end, body: c.body })));
}



// ---------------------------------------------------------------- what a tally and the panel read
export interface SquadReport { id: string; name: string; men: number; up: number; state: string; out: number; events: SquadEvent[] }
export function squadsReport(g: Game): SquadReport[] {
  const st = STATES.get(g); if (!st) return [];
  return st.squads.map(Q => ({ id: Q.def.id, name: Q.name, men: Q.men.length, up: Q.men.filter(alive).length, state: Q.state, out: Q.sent.filter(s => alive(s.gd)).length, events: Q.events.map(e => ({ ...e, t: Math.round(e.t * 10) / 10 })) }));
}
export function squadsSummary(g: Game): string {
  const st = STATES.get(g); if (!st) return SQUADS.on ? 'no squads on this floor' : 'off';
  return st.squads.map(Q => { const out = Q.sent.filter(s => alive(s.gd)).length; return `${Q.name} ${Q.men.filter(alive).length}/${Q.men.length} ${Q.state}${out ? ` (${out} out)` : ''}`; }).join(' · ');
}
/** a squad's men by its name, the fallen among them (cues.ts: who speaks for it on the net) */
export function squadMen(g: Game, name: string): Guard[] { return STATES.get(g)?.squads.find(Q => Q.name === name)?.men ?? []; }
/** A STATIC ASK FOR A CHECK (tools/qa/wayround-check.ts), read only: for every man at home of every squad that may leave its rooms, every OTHER squad's room
 *  and every door of that room taken as the team's, would he find a way round, the question decide() asks through pair(). Asked twice: with nobody of the
 *  team known (`team` false), and with THE TEAM IN THE ROOM (`team` true): a man in the team's doorway and one three metres inside it are firm places, as
 *  the squad's own eyes and the net give them in a fight, so the ground they have a line on is shut to the way as it is then.
 *  `hold` = the room counted lost (he holds its other door from outside) against going in by it. `L` = the way's length, `limit` = what it may be, `best` =
 *  the shortest way found whether inside the limit or not (undefined: no standing place at all). Nothing is written: the search's memo is put back. */
export function squadWaysRound(g: Game): { squad: string; man: string; from: string; room: string; door: string; hold: boolean; team: boolean; way: string | null; L: number; limit?: number; best?: number }[] { return crossToRust('squadWaysRound', undefined, [g]) as any; }
