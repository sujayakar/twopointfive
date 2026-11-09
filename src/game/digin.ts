// DEFENDERS DIG IN AHEAD OF THE TEAM (the harder-defenders goal; default OFF = the room fight exactly as it was).
//
// What it is: a man holding a room the team has NOT reached yet, who learns where the team is — his own ears (the room fight's `noiseAt`: gunfire, a boot, a
// charge somewhere outside his room) or a word on his side's net (the first man of a room to get eyes on the team calls it, once he is past his own reaction) —
// gets ready for them instead of standing where he stood. IT TAKES HIM REAL TIME, in steps the team can catch him at:
//   think (2–5 s) → [gather: alone in his room, run to a room of his side's that still has men, said on the net] → shout, or the room's word on the net when no
//   enemy is in earshot → the bracer runs to a chair / carton / bin,
//   hauls it to the door (0.7 m/s, scraping the whole way), pulls the leaf to, works it under the handle (4–7 s, bumps heard) → the next likeliest door the
//   same way → to cover → set. A room is all set ~15–30 s after its first word of the team; a man caught with his hands on the furniture owes more before his
//   first round (`caught`), a door opened under his hands is not wedged, and anybody in his sight ends the step he is on.
//   · WHICH DOOR: the shortest way round from the noise that does not run through his own room; the men of a room turn about between the two likeliest,
//   · WHO: as shipped the room fight's rally decides first, exactly as ever (a man who means to go to the fight next door goes) and whoever holds his room
//     digs in — the switch never takes a man off the hunt. STAY (off) makes every man who hears them hold and dig in: MEASURED, that is EASIER on a team with
//     flashbangs than today's rush (the men on the hunt are what costs it men between doors; a flashbang undoes most of a dug-in room),
//   · GATHER (off: measured, two men under one flashbang): a man alone in his room doubles up with a room that still has men (never by a way that passes the
//     noise; ≤ 24 m, the move mode's ten seconds),
//   · BRACE: the furniture is really moved (props.ts), and the wedge reuses the Door's own lock (locked against the far face, to a lock no ring fits): it will
//     not open by hand from outside — the team's door tools boot it (their own 'locked. kicking it.'), a charge blows it, a man ordered across is walked round
//     by another door if there is one, Sam gets pick / kick; staff contacts carry a key, so nobody of theirs is ever stuck behind one,
//   · DOORS: a door of his room standing open toward them is walked to and pulled shut first,
//   · COVER: he moves to the spot in the room that SEES that doorway, is not seen from outside it, has a wall at its back and is not in front of another door —
//     the far side of the opening from a mate already on it; FAR: as far back as the room lets him see it (a stun canister reaches eight metres),
//   · DEPTH: every second man of a room takes a spot OFF the door's line instead (neither the doorway nor a can gone off just inside it sees his eyes) that
//     looks onto the floor the team must cross; LEARN (off) makes that every man once their side has heard a flashbang,
//   · STEADY: a man set on a door his room wedged does not flinch when it is booted in (the room fight's startle) — he was waiting for exactly that.
// Nothing here touches hit points, dice, aim or the reaction tables; `steady` withholds one surprise beat from a man who is not surprised, `caught` ADDS one.
// Every step has a cue: their net on the HUD's log (contact called, a man falling back to another room), the shout (said only when Sam or a teammate can hear
// it), the furniture scraping (a GameEvent + the sfx, like any shoved chair; a teammate who hears it says so), a teammate stacked on a wedged door saying it
// is blocked BEFORE anybody boots it, and the boot itself.
//
// Seams (all one added line or one added clause): roomfight.ts tickDef → digInTick, rally → digInStays, reset → digInReset, evidence (the startle) →
// digInBraced, applyWatch → digInLook. Everything else lives here. The two readonly Door fields the wedge writes are put back by digInReset (Door.reset()
// restores `locked` itself). The team has been in a room: its men are the room fight's again (no fear topped up, no second dig-in there).
//
// IMPORT CYCLE (src/game is one): nothing at module level uses another game module's exports.
import { men } from './seats';
import { GameSideMap } from '../math/side';
import { Vec3 } from '../math/vec';
import type { Game, Guard } from './game';
import type { Door } from './doors';
import type { Prop } from './props';
import './guards';
import './squad';
import './pressure';
import { squadsLive } from './enemysquads';
import { w, WORDS } from './words';
import { doorWords } from './orders';   // (call-time only: what a door is called — one name on one HUD)
import { simRandom } from '../math/dice';   // the simulation's die (math/dice.ts): the Game's own when it has one, Math.random otherwise
import './gunroles';   // (call-time only)
import './lanes';   // (call-time only: is this spot in a lane his side knows is covered)
import './owner';
import { goneWithTick } from './crossing';

export const DIGIN = {
  /** THE switch: off = nothing here runs (the room fight as it was) */
  on: false,
  /** EVERY man who hears them coming holds his room and digs in — nobody goes to the fight next door. Off (shipped): the room fight's rally decides first, as
   *  ever (ROOMFIGHT.rallyK is the dial for how many go), and only a man who holds his room digs in. MEASURED: with nobody on the hunt the scripted team loses
   *  FEWER men than today, flashbangs in hand — the men who come to the fight are what kills it between doors */
  stay: false,
  /** REACH (m, on foot): a man whose way to the fight he hears is longer than this cannot join it in time — he holds his room and digs in, whatever the room
   *  fight's rally would have rolled; a man nearer than this is the rally's (or a press rule's) to send, as ever. 0 (shipped) = no reach rule: the rally
   *  decides for everybody and only the men it leaves at home dig in. MEASURED at 18 m: it claims six men in ten on every level there is (they all sit in one
   *  32 × 20 m shell), and men kept home are men not on the hunt — the scripted team loses FEWER. It is the knob a press / push rule wants beside it (near men
   *  go at once, far men get ready, then come): by itself it only takes the hunters away. AS BUILT the way is measured ONCE, when he first hears them, to the
   *  noise (or the net's fix) that told him: a fight that later moves next door does not release a man judged far, nor claim one judged near */
  reachM: 0,
  /** WITH THE PRESS ON TOO: the man a press leaves to KEEP his room (pressure.ts keepRoom) digs in. Off (hard — difficulty.ts HARD.keeperDigs): he does not,
   *  from the moment he is left until the fight is reset — he is the press's keeper and nothing more, and its last-man rules send him out by another door when
   *  the team's boots reach his. MEASURED (The block, the depot, the records building): the keeper is a man who at standard came hunting; dug in he is behind
   *  the one door a stun canister comes through, and costs the team less than the hunter he was. A man the press has left nowhere (no fight he could go to) still digs in, as ever; a man
   *  already at his furniture when he is made the keeper carries on with it */
  keeper: true,
  /** move to a covered spot that holds the door they will come by (off: he turns to that door from where he stands) */
  cover: true,
  /** WHERE HE MAY GO FOR IT: the spot he pushes the furniture from is in HIS OWN ROOM and so is the way to it. 'Behind the prop' is a point 0.7 m past it from
   *  the door: for a bin or a carton against a wall that point is IN the wall, and the nearest free floor to it can be the far side — the warehouse's dock men
   *  walked out into the lot for a carton by the east wall and held the building from outside it. On: such a prop is pushed from the best spot behind it inside
   *  the room (within reach of it, nothing solid between, and the haul from there to the door inside the room too), else the next nearest prop is tried, else no furniture (he shuts the door and takes cover, as a man with nothing to haul does). Off = as it was:
   *  only a last-stand man is refused a goal outside his room */
  braceInRoom: true,
  /** … once HE has noticed them (on; off = on his first glimpse, as it was — the 10-13 tapes: a calm man at 1–20 % awareness radioing 'they're on us. watch
   *  your doors!' half a second before his own 'huh?', and a man dropped 0.18 s after his first glimpse having told every room). Noticed = roused (alert or
   *  searching: the line the squads' net draws for a man's eyes, enemysquads.ts) AND `noticedSecs` of them in his eyes without a break: this tick runs
   *  before the room fight hands a man his own start, so 'past his own start' alone let a roused man's word out on the very frame of the glimpse */
  noticedSecs: 0.35,
  /** WHERE DOORS STAY AS THEY ARE LEFT (doors.ts DOORSTAY — nothing below runs where every leaf has its closer, or with this off: the 2.5 s wait and his own
   *  door only, as it was): a man who has pulled a door of his room to keeps his hand on it until the latch has it — a leaf with no closer takes four or five
   *  seconds from its stop, not the moment a sprung one had left, and he used to walk off from his furniture before it was home, the wedge never placed. He
   *  pushes it home the last of the way (a one-way leaf: it comes to on its frame) and pulls it again if something stopped it. And before he goes to cover he
   *  shuts the room's OTHER doors that stand open, nearest first: never one a mate has in hand, never by a way that leaves his room, and not at all once
   *  they are heard at any door of it (shut or open: then he takes cover, as he did).
   *  SHIPS OFF: it overshoots the build with closers. On the barracks (144 scripted seeds) defenders wedged 669 doors with it on against 248 on the old build
   *  and 249 with it off, the squad booted 74 against 29, and the men it lost in open-and-clear entries went from 62 to 105. On the block with three squads it
   *  only puts back what closers gave (wedges 133 on, 74 off, 72 seeds). Tab ▸ Sandbox ▸ Defenders dig in turns it on */
  shutOpen: false,
  /** …seconds he gives a pulled leaf to latch before he lets it be · how near home (rad) his hand pushes it the rest · the other doors one man goes to at most */
  shutSecs: 7, shutHomeRad: 0.3, shutDoors: 2,
  /** DOUBLE UP (off as shipped: measured, it puts two men under one flashbang and the team loses fewer): a man alone in his room, who hears them coming, runs to
   *  a room of his side's that still has men in it and digs in THERE (said on the net) — the team finds his room empty and the next one full */
  gather: false,
  /** … no further than this by the floor (m), and never by a way that passes this near the noise (m) */
  gatherM: 24, gatherShyM: 6,
  /** seconds between hearing it and moving [min, max] — his own, drawn per man */
  think: [2, 5] as [number, number],
  /** seconds of work at the door to wedge the furniture under it [min, max] — bumps heard the whole time; a door opened meanwhile is not wedged */
  wedgeSecs: [4, 7] as [number, number],
  /** doors one room wedges at most (the one they will come by first, then the next likeliest, while there is furniture and time) */
  wedges: 2,
  /** caught with his hands on the furniture or the door: seconds MORE before his first round [min, max] (his gun was not in his shoulder) */
  caught: [0.5, 0.9] as [number, number],
  /** a teammate (or Sam) who stands this long (s) at the outside of a wedged door has tried the handle: he says it is blocked — BEFORE anybody boots it */
  trySecs: 0.8,
  /** every step's length × this (1 = as shipped; 2 = digging in takes twice as long — measurement: a team that arrives twice as soon) */
  pace: 1,
  /** the noise this near the door he would brace (m): no time for furniture, cover only */
  closeM: 6,
  /** how far a man goes for a prop / hauls it (m, straight) */
  propM: 9,
  /** hauling pace (m/s) */
  haulSpeed: 0.7,
  /** the furniture's scrape: hearing level (a shoved chair is ~0.3–0.5) and seconds between scrapes */
  scrapeLevel: 0.5, scrapeEvery: 0.8,
  /** a shout carries this many metres of open air (× the propagation's carry) */
  shoutM: 18,
  /** a second door counts as a way in they might take when its way round is at most this much longer (m) */
  altM: 10,
  /** a new spot must beat where he stands by this much (score) */
  better: 0.6,
  /** the fear he keeps on his door while dug in (the room fight's evidence score, 0‥3), and for how long after the last loud noise (s) */
  fear: 2.4, holdSecs: 90,
  /** dug in on one door, the noise now says another: seconds before he shifts */
  redigSecs: 6,
  /** A SET MAN IS NOT LEFT STANDING IN A LANE HIS SIDE KNOWS IS COVERED (lanes.ts: fire that dropped a mate or passed one, called on their net — nothing where
   *  the lanes are off, so the frozen stand is as it was). The spot he takes to cover his door is never one in such a lane's line, and a man already set whose
   *  spot comes to lie in one — nobody in his own sights — picks again, at most every `laneSecs`: another spot of HIS room on the same door, out of the line.
   *  No such spot = he stays where he is, as before; he still does not give the room up (holdSet). false = spots as they were dealt before the rule */
  laneSecs: 2,
};
export const DIGIN_DEFAULTS = JSON.parse(JSON.stringify(DIGIN)) as typeof DIGIN;
/** THE FURNITURE COMES HOME (10-13). A hauling man was asked whether it had made it the frame he stopped at his stand by the door — and furniture that had
 *  swung wide of him on the way (he walks a corner, it follows a pace behind the turn) was then still a hand short of the door: 1.32 to 1.44 m from its place
 *  against a test of 1.3, with four to six seconds of his haul clock unspent. He gave it up ('the furniture did not make it to the door') and took cover beside a
 *  carton that stood a step from the door. On: at his stand, with the clock he was given still running and the thing within his reach, his hands bring it the
 *  rest of the way before he is asked. Nothing else of the haul changes: the same clock ends it, the same test wedges it.
 *  A table of its own (difficulty.ts rewrites DIGIN from its defaults at every level change). OFF here and on the frozen stand (TPS_HAULHOME=1 raises it); the
 *  page puts it on (host/profile.ts; ?nohaulhome = the haul as it was). tools/qa/haulhome-check.ts */
export const HAULHOME = { on: false };
export const DIGIN_STATS = {
  heard: 0, far: 0, dug: 0, moved: 0, stayed: 0, hauls: 0, /** braceInRoom: props pushed from beside (the spot behind was not his room's) · props passed over for it */ braceBeside: 0, bracePassed: 0, braced: 0, kicked: 0, pulledTo: 0, radio: 0, shouts: 0, scrapes: 0, callouts: 0, aborted: 0, caught: 0, steady: 0, tried: 0, deep: 0, bangCalls: 0, gathered: 0, gatherLost: 0,
  /** men of a press / last men who fell back INTO a dug-in room (pressure.ts's fall-back seam) */
  refuge: 0,
  /** DIGIN.lanes: set men who left a spot in a covered lane for another of their room · who found none and stayed */
  laneMoved: 0, laneKept: 0,
  /** rooms that dug in · of them, entered by the team · with a cue before the first step in · with every living man of the room set by then · with a man still at his furniture / door by then */
  rooms: 0, entered: 0, cueFirst: 0, readyFirst: 0, halfReady: 0,
  /** which cue had come by the first step in (a room can have several) */
  cueRadio: 0, cueShout: 0, cueScrape: 0, cueTried: 0, cueKick: 0,
  /** doors still wedged when the team first stepped into their room's fight (found shut and blocked), seconds from a room's first word of the team to its first man in (summed) and to every man set (summed, over readyN rooms) */
  wedgedAtEntry: 0, knowToEntry: 0, knowToReady: 0, readyN: 0,
  /** doors a man would have wedged but passed over for the way they swing: they open away from him, and furniture on his side holds nothing (doors.ts wedgeHolds) */
  opensAway: 0,
  /** …and wedges that WERE put on a door that opens away from the man who put it there (counted whatever the switch says: with the rule on this stays 0) */
  wedgedAway: 0,
};
export const DIGIN_LOG: { t: number; who: string; what: string }[] = [];
/** DIGIN.shutOpen's own counts: leaves pushed home the last of the way · that latched under a hand past the old 2.5 s · pulled again after something stopped
 *  them · other doors of a room gone to · leaves let be unlatched */
export const DIGIN_SHUT = { home: 0, held: 0, again: 0, more: 0, left: 0 };

const LS_KEY = 'twopointsix.digin';
const KEPT = ['on', 'stay', 'cover', 'gather', 'keeper', 'shutOpen'] as const;
const KEPT_NUM = ['wedges', 'pace', 'reachM'] as const;   // (the sliders the panel has)
const NUM_MAX = { wedges: 3, pace: 3, reachM: 60 } as const;
export function saveDigIn() { try { const out: Record<string, boolean | number> = {}; for (const k of KEPT) out[k] = DIGIN[k]; for (const k of KEPT_NUM) out[k] = DIGIN[k]; localStorage.setItem(LS_KEY, JSON.stringify(out)); } catch { /* no storage: nothing to keep */ } }
export function loadDigIn() {
  try { const j = JSON.parse(localStorage.getItem(LS_KEY) ?? 'null'); if (j && typeof j === 'object') { for (const k of KEPT) { const v = (j as Record<string, unknown>)[k]; if (typeof v === 'boolean') DIGIN[k] = v; } for (const k of KEPT_NUM) { const v = (j as Record<string, unknown>)[k]; if (typeof v === 'number' && Number.isFinite(v)) DIGIN[k] = Math.min(NUM_MAX[k], Math.max(0, v)); } } } catch { /* nothing stored */ }
  try { if (new URLSearchParams(location.search).has('digin')) DIGIN.on = true; } catch { /* no page (the harness) */ }   // ?digin: on for the visit
}
export function resetDigIn() { Object.assign(DIGIN, JSON.parse(JSON.stringify(DIGIN_DEFAULTS))); try { localStorage.removeItem(LS_KEY); } catch { /* none */ } }
loadDigIn();

// ---------------------------------------------------------------- the room fight, as much of it as this file reads (roomfight.ts keeps these private; mirrored)
interface Entry { door: Door; at: Vec3; out: [number, number] }
interface Room { id: number; cells: Set<number>; entries: Entry[] }
interface Def {
  gd: Guard; station: Vec3; room: Room | null; ev: number[]; watch: number; want: number; glanceNext: number; glanceUntil: number;
  noiseAt: Vec3 | null; noiseT: number; noiseOut: boolean; mode: 'hold' | 'move' | 'free'; rallyAt: number; rallyGo: boolean; moveKind: 'fallback' | 'cover'; moveT: number; cameBy: Door | null; stallAt: Vec3; stallT: number;
}
interface Fight { g: Game; defs: Def[]; ops: { gd: Guard }[]; inRoom(R: Room | null, p: Vec3): boolean; freeNear(p: Vec3): Vec3 | null; note(gd: Guard | null, what: string): void }

type Phase = 'idle' | 'think' | 'toRoom' | 'toProp' | 'haul' | 'toDoor' | 'shut' | 'wedge' | 'toCover' | 'set';
interface Man { far: boolean; owe: number; haulBy: number; deep: boolean; look: Vec3 | null; triedDeep: boolean; gathered: boolean; gatherTo: Room | null; dugOnce: boolean; /** he was told to, and whoever told him said the lines */ ordered: boolean; phase: Phase; until: number; know: Vec3; knowT: number; room: Room | null; entry: number; brace: number; prop: Prop | null; stand: Vec3; rest: Vec3; scrapeT: number; setAt: number; spot: Vec3 | null; kept: boolean }
type Cue = 'radio' | 'shout' | 'scrape' | 'tried' | 'kick';
interface RoomSt { room: Room; expected: number; second: number; at: number; t0: number; men: Guard[]; bracer: Guard | null; wedged: number; cueAt: number; cues: Set<Cue>; enteredAt: number; readyAt: number; callout: boolean }
interface St { men: WeakMap<Guard, Man>; rooms: Map<Room, RoomSt>; wedges: Map<Door, { keySide: 1 | -1; lockId: string; kicked: boolean; tried: boolean; byT: number }>; fix: { at: Vec3; t: number; room: Room | null; heard: boolean } | null; called: WeakSet<Guard>; used: Set<Prop>; away: Set<Door>; frameT: number; bangCall: number }
const states = new GameSideMap<Game, St>('digin.states');

const flat = (a: Vec3, b: Vec3) => Math.hypot(a[0] - b[0], a[2] - b[2]);
const oneOf = <T>(a: T[]): T => a[Math.floor(simRandom() * a.length)] ?? a[0];
const at = (p: Vec3, y: number): Vec3 => [p[0], y, p[2]];
function log(F: Fight, gd: Guard | null, what: string) { DIGIN_LOG.push({ t: F.g.time, who: gd ? gd.callsign : '-', what }); if (DIGIN_LOG.length > 400) DIGIN_LOG.splice(0, 150); F.note(gd, `dig-in: ${what}`); }

// ---------------------------------------------------------------- who can hear it (Sam and the living teammates: the cue is only a cue if it reaches them)
function listeners(F: Fight): { gd: Guard | null; head: Vec3; vel: Vec3 }[] {
  const g = F.g, out: { gd: Guard | null; head: Vec3; vel: Vec3 }[] = [];
  for (const pl of men(g)) if (!pl.down) out.push({ gd: null, head: at(pl.char.pos, 1.6), vel: pl.char.vel });   // (every player who is up: seats.ts)
  for (const O of F.ops) if (O.gd.state !== 'dead') out.push({ gd: O.gd, head: at(O.gd.char.pos, 1.6), vel: O.gd.char.vel });
  return out;
}
function cue(F: Fight, R: RoomSt | null, kind: Cue) { if (!R || !isNaN(R.enteredAt)) return; R.cues.add(kind); if (isNaN(R.cueAt)) R.cueAt = F.g.time; }






/** (its own fields on a man, all unread with the rule off: the OTHER door of his room he has gone to shut — an entry's index, -1 / unset = none —, how many
 *  such he has gone to this time, when his hand took the leaf it is on and when it last pulled) */
interface Man { /** DIGIN.noticed: since when he has had them in his eyes without a break */ eyesT?: number; /** DIGIN.lanes: not asked for a spot out of a covered lane before this */ laneT?: number; also?: number; alsoN?: number; /** HAULHOME: the haul (by its clock) whose last hand he is bringing · how far it was from its place when last it had gained, and when */ homeT?: number; homeD?: number; homeAt?: number; handT?: number; pullT?: number; pushes?: number; pullA?: number; stalls?: number }





/** once a frame: a room all set, the team's first step into a room that dug in, and wedges that have been booted */
function watch(F: Fight, st: St) {
  const g = F.g, S = DIGIN_STATS;
  for (const [d, w] of st.wedges) if (!w.kicked && d.lockBroken) {                // the wedge found the hard way: the door that would not open IS the cue, if nothing came before it
    w.kicked = true; S.kicked++;
    for (const R of st.rooms.values()) if (R.room.entries.some(e => e.door === d)) cue(F, R, 'kick');
  }
  for (const [d, w] of st.wedges) if (!w.tried && !w.kicked && d.locked) {          // a hand on the handle from outside: the wedge found the quiet way, by whoever stands at the door
    const c = d.frameCentre, by = listeners(F).find(L => flat(L.head, c) < 1.7 && g.doors.side(d, L.head) === d.keySide && Math.hypot(L.vel[0], L.vel[2]) < 0.4);
    if (!by) { w.byT = NaN; continue; }
    if (isNaN(w.byT)) w.byT = g.time;
    if (g.time - w.byT < DIGIN.trySecs) continue;
    w.tried = true; S.tried++;
    const dn = doorWords(d);   // (words.ts: the line NAMES its door — a man within reach of two doors said 'door's blocked' of the one he was not stacked on)
    if (WORDS.on && Math.abs(d.angle) > 0.05) { if (by.gd) oneOf(['', '']); }   // (a leaf standing open is no door to call wedged: the line is not said — his die is still drawn, the rule and its cue are as they were)
    else if (by.gd) g.say(by.gd, oneOf(WORDS.on ? [`the ${dn} is blocked. something heavy behind it.`, `the ${dn} is wedged from inside. they know we're here.`] : ['door\'s blocked. something heavy behind it.', 'it\'s wedged from inside. they know we\'re here.']), true); else g.msg(WORDS.on ? `the ${dn} is wedged from inside` : 'the door is wedged from inside');
    log(F, by.gd, `tries ${d.def.name}: wedged`);
    for (const R of st.rooms.values()) if (R.room.entries.some(e => e.door === d)) cue(F, R, 'tried');
  }
  for (const R of st.rooms.values()) {
    if (!isNaN(R.enteredAt)) continue;
    const home = F.defs.filter(X => X.gd.state !== 'dead' && X.room === R.room), ph = (X: Def) => st.men.get(X.gd)?.phase ?? 'idle';
    if (isNaN(R.readyAt) && home.length && home.every(X => ph(X) === 'set')) { R.readyAt = g.time; S.readyN++; S.knowToReady += g.time - R.t0; log(F, null, `room ${R.room.id} all set, ${(g.time - R.t0).toFixed(1)} s after the first word of them`); }
    if (!listeners(F).some(L => F.inRoom(R.room, L.head))) continue;
    R.enteredAt = g.time; S.entered++; S.knowToEntry += g.time - R.t0;
    if (!isNaN(R.cueAt)) S.cueFirst++;
    if (R.cues.has('radio')) S.cueRadio++; if (R.cues.has('shout')) S.cueShout++; if (R.cues.has('scrape')) S.cueScrape++; if (R.cues.has('tried')) S.cueTried++; if (R.cues.has('kick')) S.cueKick++;
    const busy = home.filter(X => ['toProp', 'haul', 'toDoor', 'shut', 'wedge', 'think', 'toCover'].includes(ph(X))).length;
    if (!isNaN(R.readyAt)) S.readyFirst++; else if (busy) S.halfReady++;
    for (const e of R.room.entries) if (st.wedges.has(e.door) && !e.door.lockBroken && e.door.locked) S.wedgedAtEntry++;
    log(F, null, `team into room ${R.room.id} ${(g.time - R.t0).toFixed(1)} s after its first word of them: cues ${[...R.cues].join('+') || 'none'}${isNaN(R.cueAt) ? '' : ' (first ' + (g.time - R.cueAt).toFixed(1) + ' s before)'}, ${home.filter(X => ph(X) === 'set').length}/${home.length} set, ${busy} still busy, ${R.wedged} wedged`);
  }
}


/** where his dig-in stands ('idle' = not at it) */
export function digInPhase(g: Game, gd: Guard): string { return states.get(g)?.men.get(gd)?.phase ?? 'idle'; }
/** this room's doors that stand wedged (not yet booted) */
export function digInWedged(g: Game, room: unknown): { door: Door; at: Vec3 }[] { return goneWithTick('digInWedged') as any; }


/** pressure.ts lastMan: he is SET on his door (dug in, gun on it) */
export function digInSet(g: Game, gd: Guard): boolean { return states.get(g)?.men.get(gd)?.phase === 'set'; }


/** a fresh staging (roomfight.ts reset): wedged doors get their own lock back, everything remembered dropped */
export function digInReset(g: Game) {
  const st = states.get(g); if (!st) return;                                     // (nothing dug in since the last one: nothing to put back — and nothing touched with the switch off)
  for (const [d, w] of st.wedges) { const mut = d as unknown as { keySide: 1 | -1; lockId: string }; mut.keySide = w.keySide; mut.lockId = w.lockId; if (!d.lockBroken) d.locked = !!d.def.locked; }
  states.delete(g);
  for (const k of Object.keys(DIGIN_STATS) as (keyof typeof DIGIN_STATS)[]) DIGIN_STATS[k] = 0; DIGIN_LOG.length = 0;
  DIGIN_SHUT.home = DIGIN_SHUT.held = DIGIN_SHUT.again = DIGIN_SHUT.more = DIGIN_SHUT.left = 0;
}

export function digInSummary(g: Game): string {
  if (!DIGIN.on && !squadsLive(g)) return 'off';   // (enemysquads.ts: a squads level's men dig in on their squad's word whatever the switch says)
  const S = DIGIN_STATS, st = states.get(g);
  return `${S.dug} dug in (${S.moved} moved) · ${S.braced} doors wedged (${S.kicked} booted) · ${st ? st.rooms.size : 0} rooms · cue first ${S.cueFirst}/${S.entered}`;
}
