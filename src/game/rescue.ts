// The HOSTAGE RESCUE playtest (?rescue · native --rescue · __rescue · Tab ▸ Sandbox ▸ Hostage rescue). The night breach (nightbreach.ts) asked whether a team entry
// reads as a fight and the room clear (roomclear.ts) whether a structure can be taken room by room; this asks the next thing: does it still work when the team has
// to FLOW — run past rooms it has not cleared, because somebody at the far end of the corridor is about to be shot. The office at night; the shift, the cleaner and
// the late worker, the response team and the cameras off the floor, as under the night breach (the same four gates). Design: the 'fast rescue' answer page's
// section 3. It stands on the room clear's build: the pair are ordered through its command menu (orders.ts — press X), both sides shoot on their own read
// (roomfight.ts), and its fog of war is on under the flag.
//
//   WHO IS WHERE   two HOSTAGES (civilians of the usual make, down and told to stay down — civilians.ts spawnHostage) along the north wall of the manager's office;
//                  a CAPTOR in the office with his gun on the corridor door; a DOOR MAN in the corridor's east half (knob: just inside the office instead); a ROAMER
//                  who comes to the noise — in the break room or the conference room, drawn at each staging. Sam and two operatives (Blue One, Blue Two) at the lobby arch, the corridor's west end: the conference
//                  room, the cubicles' two openings and the server room all lie between them and the hostages.
//   CLOCK ONE      the captor's. `resolve` runs 0 → 1 over RESCUE.resolveSecs (drawn ± resolveSpread at each staging, so it cannot be counted — unless samestart.ts is on: then it is the knob's to the second, every try) from the moment the
//                  clock starts — hot start (default): Sam's first move, shot or order; quiet start: the first thing a defender hears or sees. At 1 the captor
//                  turns on the nearest hostage, takes aimSecs to line up, and fires; the second hostage follows secondHostageSecs later. A dazzled captor's clock
//                  stands still (the charge is the window), a dead captor's passes to whoever of his side is alive in the room — who is secondHostageSecs from
//                  doing it at the soonest, however far the last man had got — and with nobody of theirs in the room it stops. He shouts it (three steps: shouting / counting / about to); the message line says when the step changes and the panel's summary which it is.
//   CLOCK TWO      every defender's. UNSET until setSecs after he first learns the assault is on (his own ears and eyes: the AI's hearing puts him off 'patrol';
//                  at a hot start the captor begins SET — captorSet): a man caught unset reacts late (unsetReact from the moment he first SEES one of the team)
//                  and, dazzled, stands and stares. SET, he is the room fight's defender as it has him: gun on the entry his own ears fear (the way he was put,
//                  until he has heard something), and dazzled he empties a burst blind down it. The roamer's 'set' is setting OFF: he is let off his post to
//                  hunt the last loud thing the team did (the room fight may have sent him to the gunfire sooner, by his own ears).
//   THE TEAM       the room clear's squad: the X menu's orders for where they are and what they watch — fall in, hold, move there, stack up, cover (orders.ts) and
//                  the DOOR TOOLS on any door, the hostage room's two included: open and clear, bang and clear, breach with a charge, clear from outside
//                  (doortools.ts — the same rows, keys and counts as under the room clear: 2 charges and 3 bangs a staging) — and the room fight for when they
//                  shoot. At a hot start the pair FALL IN the moment the clock starts, without being told (dynamic tempo: whatever Sam passes, they pass).
//   F              GO ON THE HOSTAGE ROOM. A charge set on a door has the press first, as everywhere (doortools.ts: F blows it — and under the rescue the charges
//                  on BOTH of the room's doors go on the one press). Otherwise F is the menu's 'bang and clear' on the hostage room's corridor door without the
//                  menu: whoever is free runs to the stack, the lead boots it, the can goes in, they follow it — the door tools' own order, its can out of the
//                  same three ('open and clear' with none left, or with RESCUE.stun off). The men are the menu's throughout: any order takes a man off it at
//                  any moment (nothing is 'committed'). With the clock not yet running (quiet start) F is two words: the first stacks them and they hold, the
//                  second sends them in.
//   THE FLOW       once the clock runs the pair do not only follow: the last man in the file COVERS each opening Sam passes, unasked, says so, and catches up;
//                  the floor across each opening shows what is known of it (rescueflow.ts — RescueFlow, `flow`; RESCUE.flow / .marks).
//   TWO DOORS      F with a man Sam has posted in the server room (hold / move there / cover / stack up), or pressed from in there with both men on him: that
//                  man takes the SIDE door, the other the corridor door, as two of the door tools' orders through one GATE (doortools.ts DoorGate): neither
//                  door is opened until both are stacked, then the side door's shove waits for the corridor door's boot and the leaves move on one frame; each
//                  man throws his own can and goes in on his own count behind it. A man Sam has put on a 'cover' of his own stays on it: F is a man short.
//                  (The side door's jambs on the server room's side have no floor: the scenario authors that face's marks — SIDE_MARKS, through orders.ts
//                  doorMarks — so the menu's stack up and every door tool work on it from in there too.)
//   THE END        SECURED = nobody of theirs alive in the office, one of the team in it, a hostage alive; held holdSecs → RESCUED: CLEAN with both hostages
//                  alive, COSTLY with one (team up and seconds from the clock's start on the line). LOST (FAILED) = both hostages dead, one hit by Sam's own
//                  round, or Sam down (the game's own down card follows). A restart of any kind stages it again.
//
// Nothing here runs unless start() was called: tick() returns on its first line, and with the scenario off the game is the game as it was.
import './seats';
import { SideSet, tableNote } from '../math/side';
import { Vec3, v3 } from '../math/vec';
import type { Game, Guard } from './game';
import type { Door } from './doors';
import type { RoomClearPlan } from '../scene/level';
import type { DoorOp, DoorGate } from './doortools';
import { sideOf } from './orders';
import './doornames';
import type { Assignment } from './orders';
import { spawnUnit, despawnUnit } from './guards';
import { PRESSURE } from './pressure';   // (read inside defenders() only: Hard's master switch — nothing of pressure.ts is touched at load)
import { order, endGuardScript, inBounds } from './squad';
import './combat';
import { RescueFlow } from './rescueflow';
import { RESCUEEND, RescueEnd } from './rescueend';   // (read inside methods only: how a rescue is won — out, or cleared)
import { RESCUEFLUSH, RescueFlush } from './rescueflush';   // (likewise: the men left alive come for the hostages)
import { spawnHostage, hostageSay } from './civilians';
import { WORDS, w } from './words';   // (a leaf: the dialogue pass's one switch)
import { SQUAD4, SQUAD4_BUDGET, type BlindBurst } from './nightbreach';
import type { SquadSize } from './loadout';
import { kitStage } from './kit';
import { isOffice } from './dialogue';   // (called inside the crew's lines only: nothing of dialogue.ts is touched at load)
import './cues';
import type { RescueData, Rect, TeamStart } from '../scene/scenarios';
import { sameTeamStart } from '../scene/scenarios';
import officeData from '../../levels/office.json' with { type: 'json' };   // (the bundled level's own file: its scenarios block is where these knobs start from. JSON, so safe at the top of the game's import cycle)
const OFFICE_RESCUE = (officeData as unknown as { scenarios: { rescue: RescueData } }).scenarios.rescue;
import { SAMESTART } from './samestart';   // (a leaf)
import { gunDeal } from './gunroles';   // (called inside the staging only)
import { simRandom } from '../math/dice';   // the simulation's die (math/dice.ts): the Game's own when it has one, Math.random otherwise
import './owner';
import './rescuecheck';
import { RUST_PASS, RUST_SURFACE, crossToRust, goneWithTick } from './crossing';
import { seatCount, budgetWithSeats } from './seats';   // (how many players' men stand: every budget of characters counts it)

/** The scenario's knobs (Tab ▸ Sandbox ▸ Hostage rescue, __rescue.knobs). Who stands where, the stun, the lights and the clock's dealt length are read at a staging
 *  ('restage' applies them); the clocks' other numbers, blind fire and the clock's own switch are read as it runs. */
export const RESCUE = {
  /** 'hot' = the captor's clock starts on Sam's first move, shot or order; 'quiet' = on the first thing a defender hears or sees of the team */
  start: 'hot' as 'hot' | 'quiet',
  /** clock one (off = nobody is shot for the team being slow: the A/B, and the tally's control) */
  clock: true,
  /** seconds from the clock's start to the first hostage shot, ± the spread drawn at each staging; the second hostage this long after the first; his aim on her
   *  (35 since the tally — tools/qa/rescue-tally.ts: at 40 a room-by-room crawl with a bang on every door still brought a hostage out in half its runs; at 35
   *  it fails in most, which is what the design asks of it, and the fast entry keeps a dozen seconds in hand) */
  resolveSecs: 35, resolveSpread: 6, secondHostageSecs: 8, aimSecs: 1.2,
  /** clock two: seconds from a defender first learning of the assault to his being set, and the reaction time a man caught before that is held to on his first alert */
  setSecs: 9, unsetReact: 1.1,
  /** hot start: the captor begins SET on the corridor door (he is expecting somebody through it); off = he starts unset like the others */
  captorSet: true,
  /** seconds the secured room must be held for the run to end */
  holdSecs: 15,
  /** the door man: in the corridor's east half, on the straight run — or just inside the office, a second gun on the door */
  doorMan: 'corridor' as 'corridor' | 'inside',
  /** men elsewhere on the floor who come to the noise (0‥2: the break room, the conference room — the second costs a hostage his shadow in the page's renderer);
   *  `seedRoamers`: which of the two rooms the first is in is drawn at each staging (the conference room is the first door the team passes, the break room the
   *  last — so no room is known empty); off = the break room first */
  roamers: 1, seedRoamers: true,
  /** the pair: F is 'bang and clear' — a stun canister in behind the boot, out of the squad's count (off: 'open and clear'); weapon lights once the clock runs;
   *  the response team's hands (aim.ts TRAINED — read only with the room fight off: with it on their side's FIGHT_SKILL row is their hands) */
  stun: true, opLights: true, trained: true,
  /** the defenders: a set man dazzled fires blind down his door (nightbreach.ts BLIND); weapon lights */
  blindFire: true, defLights: false,
  /** the office's own lights (off = the room as the night has it) */
  roomLights: false,
  /** dynamic tempo (rescueflow.ts): once the clock runs the last man in the file covers each opening Sam passes, unasked, and catches up (off = the pair only
   *  follow — the build as it was: the tally's A/B); a lone man still peels off to cover (off = he stays on Sam); the strips on the floor across each opening
   *  and the objective line's 'behind you' */
  flow: true, marks: true,
  /** F with a man posted in the server room (or pressed from in there with both men free): he takes the side door, the other the corridor door, on one signal;
   *  a man Sam has put on a cover of his own is left on it. Off = F takes whoever is up to the corridor door — but for one fix that has no switch: a man sent
   *  there from inside the server room goes round by the corridor (roundBy), not through the side door and the hostage room */
  twoDoor: true,
  /** THE FOUR on two doors: the side door is a PAIR's — the man F gives it takes his pair mate with him when the mate stands in that room too (the other pair
   *  the main door, on the one signal) and somebody is left for the main door. Read only on a level that asks for it (scenarios.rescue.side.pair: the consulate) with
   *  the four-man squad staged — every level before it plays as it did whichever way this stands. Off = one man on the side door whoever stands beside him */
  sidePair: true,
  /** HARD'S RESCUE CREW — read only while Hard has it on (difficulty.ts HARD.rescueCrew → PRESSURE.rescueCrew; Standard never looks at them). `crewCallSecs` after his clock starts the
   *  captor calls the assault on their net, and from that word his two men know it is on (their set clocks start) and act on it, each saying so on the net:
   *  `crewCross`  a man who can walk to a spot with a line of fire to the floor outside the hostage door — one the way in does not see until it is nearly
   *               there: a side room's doorway — takes it and keeps his gun on that floor: the stack is in a crossfire
   *  `crewDoor`   the door man with no such spot leaves the corridor for the hostage room and pulls the door to: a second gun on it, set before the team is there
   *  `crewBehind` a roamer with no such spot gets out of sight of the way in and lets the team pass; and any of them on his spot comes up behind once the
   *               team's noise is at the hostage door and nobody is in his sights
   *  Each off = that much of Standard back (all three off = Hard on a rescue as it was: nothing) */
  crewDoor: true, crewCross: true, crewBehind: true, crewCallSecs: 1.5,
  /** the HUD line shows the captor's clock in seconds (tuning) instead of its three steps */
  clockAsSeconds: false,
  /** how many teammates are staged (game/loadout.ts — the page's ?squad=N; scenarioRun.ts writes it): 2 = the pair, exactly as it always was · 0 / 1 = nobody /
   *  Blue One alone · 3 / 4 = the four-man squad's first three / all of it (nightbreach.ts SQUAD4: a red pair and a blue pair, in pairs for the order menu), the
   *  men past the second stood a stride behind the pair's marks. F takes whoever is up, as ever. */
  squad: 2 as SquadSize,
  /** where the team is staged: the LEVEL's (scene/scenarios.ts RescueData.team — start() writes the team start of the level it stages on here; these are the
   *  office's until it has: the lobby arch at the corridor's west end) */
  samAt: [...OFFICE_RESCUE.team.sam] as Vec3, samYaw: OFFICE_RESCUE.team.samYaw, opsAt: OFFICE_RESCUE.team.ops!.slice(0, 2).map(p => [...p]) as [Vec3, Vec3],
};
/** the team start RESCUE.samAt / samYaw / opsAt were last written from (start()) */
let applied: TeamStart | null = OFFICE_RESCUE.team;   // (the knobs above ARE the bundled office's: a knob moved by hand before the first start on the office stays moved)
tableNote('RESCUE', () => applied, v => { applied = v; });   // (a copy of the knob tables put back puts this back with them: math/side.ts)

/** WHO STANDS WHERE is level data (scene/scenarios.ts RescueData — Level.scenarios.rescue): the hostage room and its
 *  doors, the marks of everybody the scenario stands on the floor, the team's start, the route's openings. Rescue.data is the one place it is read from. */
/** THE OFFICE'S side-door marks, under the name its probes import them by (the game reads the level's own: Rescue.data.side) */
export const SIDE_MARKS = { stack: OFFICE_RESCUE.side!.stack as Vec3[], standoff: OFFICE_RESCUE.side!.standoff as Vec3[] };
const NUMBER = ['no', 'one', 'two', 'three', 'four', 'five', 'six'];
const flat = (a: Vec3, b: Vec3) => Math.hypot(a[0] - b[0], a[2] - b[2]);
const yawTo = v3.yawTo;
/** the door tools' orders that end with a man holding a point inside a room · how fast a man pans his light across the hostage room looking for somebody (search()) */
const DOOR_ORDERS = new Set(['openClear', 'bangClear', 'charge', 'outside', 'threshold']);

/** The rescue's data against the floor it is laid on (headless, like roomclear.ts checkStructure): every mark where a man can be stood, the men of the room
 *  inside it and with a line to what they cover, the hostages inside it, the team's start joined on foot to the room, the side door a door of the level, and the
 *  route's own check (rescueflow.ts). Empty = sound. canStart does NOT gate on it: a mark a hand's width off is a thing to see and fix, not a refusal. */
export function checkRescue(g: Game, S: RescueData | undefined = g.level.scenarios?.rescue): string[] { return crossToRust('checkRescue', undefined, [g, S]) as any; }

export type RescueState = 'off' | 'staged' | 'running' | 'secured' | 'rescued' | 'lost';
export type RescueStep = 'waiting' | 'shouting' | 'counting' | 'about to' | 'stopped';
type Role = 'captor' | 'door' | 'guard' | 'roamer';
/** F's entry on the hostage room: the door tools' orders it gave (one a door: `ops`, what of each one's beats the log has had), the gate they go through, the men
 *  it took, whether it is a split (two doors, or one man alone), and when everybody was stacked / the gate opened / the first man was sent in */
interface Entry { ops: { op: DoorOp; side: boolean; seen: number; kicked: boolean; ended: string | null }[]; gate: DoorGate; men: Guard[]; split: boolean; setT: number; goT: number; inT: number }
/** `crew` (Hard's rescue crew — RESCUE.crewDoor): '' not on it (yet) · 'going' the door man on his way in · 'in' inside · 'cross' a man with his gun on the floor
 *  outside the hostage door (`look`) · 'wait' a roamer out of sight until the door goes (`noise0` = the last loud thing the team had done when he hid) · 'out' off it */
interface Def { role: Role; inRoom: boolean; mark: { pos: Vec3; aimAt: Vec3 }; learned: number; set: boolean; sawFoe: boolean; released: boolean; burst: BlindBurst; k: number; crew: '' | 'going' | 'in' | 'cross' | 'wait' | 'out'; look: Vec3 | null; spot: Vec3 | null; noise0: Vec3 | null }

export class Rescue {
  active = false;
  /** the knobs above, on the instance for the console (__rescue.knobs.resolveSecs = 60; __rescue.restage()) */
  readonly knobs = RESCUE;
  ops: Guard[] = []; hostiles: Guard[] = []; hostages: Guard[] = [];
  /** the level's clear plan for the hostage room's corridor door (its `bounds` are the room) */
  plan: RoomClearPlan | null = null;
  /** F's entry on the hostage room while it is under way (null: none — the pair are the order menu's, or standing as they were staged) */
  entry: Entry | null = null;
  /** how many times it has been staged since start() */
  staged = 0;
  /** what happened when, on the game clock ('staged', 'clock', 'order:…', 'learned:<name>', 'set:<name>', 'caught:<name>' (seen his first of the team while unset), 'step:…', 'act', 'executed:<name>', 'teamfire:<name>',
   *  F's entry as the door tools' beats tell it — 'split:front=…,side=…', 'stacked', 'split:set', 'split:go', 'kick', 'door-flung', 'toss', 'bang' / 'nobang', 'enter1' (the side
   *  door's with ':side': 'stacked:side', 'door-flung:side', 'toss:side', 'bang:side', 'enter:side'), 'clear', 'drill:called-off', 'entry:<why it ended short>' —
   *  'act:broken' / 'act:blocked', 'secured', 'unsecured', 'rescued', 'lost:<why>') — what the probes read */
  log: { t: number; what: string }[] = [];
  /** clock one: 0 → 1; the seconds it was dealt at this staging; the game time it started (NaN = not yet) */
  resolve = 0; resolveSecs = 35; t0 = NaN;
  /** Sam's words on the hostage room so far (0 none · 1 stack · 2 go) */
  doorOrders = 0;
  /** blind rounds the set defenders have fired since the staging */
  blindRounds = 0;
  /** how it ended (null while it runs), and the line that says so */
  result: { state: 'rescued' | 'lost'; grade: 'clean' | 'costly' | 'failed'; why: string; hostages: number; team: number; secs: number } | null = null;
  private defs = new Map<Guard, Def>();
  private act: { by: Guard; victim: Guard; at: number; def: unknown; hold: boolean } | null = null;
  /** who held the clock last (a man who takes it over from another has to take the room in hand first: clock()) */
  private capBy: Guard | null = null;
  private executed = new Set<Guard>(); private counted = new Set<Guard>();
  private securedAt = NaN; private step: RescueStep = 'waiting'; private barkAt = 0;
  private noise: Vec3 | null = null; private heard = new SideSet<object>(); private lastOrder: object | null = null;
  /** Hard's crew: when the captor called the assault on their net (NaN = not yet; Infinity = nobody was left to call it) */
  private alarmAt = NaN; private way: Vec3[] | null = null;
  private benched: Guard[] | null = null; private said = new Set<string>();
  private cut = false; private failure: string | null = null;
  /** the hostages' voices (voices()): when the next low line is due, whose turn it is, when a teammate's word to them is due (NaN: none) */
  private murmurAt = 0; private murmurN = 0; private ackAt = NaN;
  /** dynamic tempo: the openings on the route, what is known of each, and the pair's unasked covers (rescueflow.ts) */
  readonly flow: RescueFlow;
  /** the room was called by F's last entry (F does nothing more until one of theirs is back in it) */
  private called = false;
  private searchT = new Map<Guard, number>(); private searchA = new Map<Guard, { A: Assignment; face: number | null; watch: Vec3 | null; there: string }>();
  /** the assignments the scenario itself gave (post()): such a man is as free for F as one falling in */
  private auto = new SideSet<object>();
  /** the corridor just outside the server room's door (roundBy) */
  private serverOut: Vec3 | null = null;
  /** how it is won (rescueend.ts: the hostages led out, or the floor cleared — RESCUEEND) · the men left alive coming for them (rescueflush.ts — RESCUEFLUSH) */
  readonly ending: RescueEnd; readonly flush: RescueFlush;
  constructor(private g: Game) { this.flow = new RescueFlow(g, this); this.ending = new RescueEnd(g, this); this.flush = new RescueFlush(g, this); }

  /** what the scenario takes from the level it stands on (Level.scenarios.rescue; null = the level has none, and it cannot start) */
  get data(): RescueData | null { return this.g.level.scenarios?.rescue ?? null; }
  /** the hostage room's floor inside its walls: the data's own bounds, else the level's clear plan's for the room's door */
  get bounds(): Rect | undefined { const S = this.data; return S?.room.bounds ?? (this.plan ?? this.g.level.clearPlans?.find(p => p.door === S?.room.door))?.bounds; }
  /** the room the side door is taken from (the office: the server room), where the data names one */
  private get fromBounds(): Rect | undefined { const f = this.data?.side?.from; return f ? f.bounds ?? this.g.level.clearPlans?.find(p => p.door === f.door)?.bounds : undefined; }
  /** `dist` out of a door's frame on the side AWAY from a room (its floor rect): the floor point */
  private outOf(d: Door, room: Rect | undefined, dist: number): Vec3 {
    const th = d.def.closedDir, snap = (x: number) => Math.abs(x) < 1e-9 ? 0 : Math.abs(Math.abs(x) - 1) < 1e-9 ? Math.sign(x) : x;
    const s = room ? -sideOf(d, [(room.x0 + room.x1) / 2, 0, (room.z0 + room.z1) / 2]) : 1;
    return [d.frameCentre[0] + snap(-Math.sin(th)) * s * dist, 0, d.frameCentre[2] + snap(Math.cos(th)) * s * dist];
  }
  /** the floor just outside the hostage room's main door · the point its first man in keeps his gun on once the room is called */
  private roomPoint(which: 'outside' | 'hold'): Vec3 { return goneWithTick('Rescue.roomPoint') as any; }

  /** why it cannot start right now (null = it can) */
  canStart(): string | null {
    const g = this.g, S = this.data;
    if (g.quiet?.active) return 'a quiet mission is up on this floor (quit it first)';   // (quiet.ts refuses the other way round: one scenario a floor)
    if (!S) return 'this level has no hostage rescue data (its scenarios.rescue block)';
    if ((S.team.ops?.length ?? 0) < 2) return 'this level\'s rescue block has no start marks for the pair (team.ops needs two)';   // (start() reads both)
    if (!this.bounds) return `this level has no clear plan for the '${S.room.door}' door`;
    if (!g.doors.byName(S.room.door)) return `the door '${S.room.door}' is not in the level`;
    if (!g.tune.teams) return 'sides are switched off (tune.teams)';
    if (!g.aiEnabled) return 'the AI is switched off (sandbox: no AI) — nobody would move';
    if (g.director?.run && !g.director.run.ended) return `the level script's program '${g.director.run.name}' is running`;
    if (g.touring || g.quietUtility || g.puppet) return 'a show is running';
    if (g.nightBreach?.active) return g.roomClear?.active ? 'the room clear is staged' : 'the night breach is staged';
    if (g.guards.some(x => x.unit)) return 'a spawned unit is already on the floor';
    return null;
  }
  /** Start it: the floor cleared of everybody but the cast, a fresh encounter, the cast on its marks. Returns the reason when refused (or when the staging itself
   *  failed — then everything is put back as stop() puts it). */
  start(): string | null { { const __s = RUST_SURFACE; if (__s) { const __r = __s.call('Rescue.start', this, []); if (__r !== RUST_PASS) return __r as any; } }
    if (this.active) return null;
    const why = this.canStart(); if (why) return why;
    { const T = this.data!.team; if (!sameTeamStart(T, applied)) { applied = T; RESCUE.samAt = [...T.sam] as Vec3; RESCUE.samYaw = T.samYaw; RESCUE.opsAt = [[...T.ops![0]] as Vec3, [...T.ops![1]] as Vec3]; } }   // (once per team start: a knob moved by hand since stays moved)
    this.bench();
    this.active = true; this.staged = 0; this.log.length = 0; this.failure = null;
    this.g.restartEncounter();   // (the benched shift is off the list and spawnCivilians stands nobody while this is up: the fresh floor is empty) → onEncounterRestart stages
    return this.active ? null : (this.failure ?? 'the staging failed');
  }
  /** Stop it: the cast gone, the shift and the building's own civilians back, a fresh night encounter. */
  stop() { { const __s = RUST_SURFACE; if (__s) { const __r = __s.call('Rescue.stop', this, []); if (__r !== RUST_PASS) return __r as any; } }
    if (!this.active) return;
    const g = this.g;
    this.active = false;
    this.clearCast();
    if (this.benched) { for (const gd of this.benched) if (!g.engine.lights.lights.includes(gd.flashlight)) g.engine.lights.add(gd.flashlight); g.guards = [...this.benched, ...g.guards]; this.benched = null; }
    g.restartEncounter();
    g.msg(w(g.level.routes.length ? 'hostage rescue off — the night shift is back' : 'hostage rescue off', 'hostage rescue off — the night shift is back'));   // (a level with nobody on a route has no shift to come back)
  }
  /** Stage it afresh (the knobs re-read): a restart of the encounter, which stages. */
  restage() { { const __s = RUST_SURFACE; if (__s) { const __r = __s.call('Rescue.restage', this, []); if (__r !== RUST_PASS) return __r as any; } } if (this.active) this.g.restartEncounter(); }
  /** restartEncounter's last act (game.ts): the scenario, while it is up, stages itself on the fresh encounter; a staging that throws ends it cleanly. */
  onEncounterRestart() {
    if (!this.active) return;
    try { this.stage(); }
    catch (e) { this.failure = `the staging failed: ${e instanceof Error ? e.message : String(e)}`; console.error('hostage rescue:', e); this.stop(); this.g.msg(`hostage rescue — ${this.failure}`); }
  }
  /** who has the pair: 'door' = F's entry on the hostage room is under way (the dynamic tempo stands aside — rescueflow.ts), 'free' otherwise. The men are the
   *  order menu's either way. */
  get mode(): 'free' | 'door' { return this.entry ? 'door' : 'free'; }
  /** F's entry has this man right now: he is on one of its door orders (the flow leaves him be; the menu does not ask — any order takes him) */
  owns(gd: Guard): boolean { const E = this.entry; return this.active && !!E && E.ops.some(e => e.ended === null && e.op.men.includes(gd)); }
  /** is this civilian one of the scenario's hostages (player.ts: no grab row on her) */
  isHostage(gd: Guard): boolean { return this.active && this.hostages.includes(gd); }

  private bench() {
    const g = this.g; const staff = g.guards.filter(x => !x.unit); if (!staff.length) return;
    for (const gd of staff) { gd.flashlight.enabled = false; g.engine.lights.remove(gd.flashlight); }
    this.benched = [...(this.benched ?? []), ...staff]; g.guards = g.guards.filter(x => !!x.unit);
  }
  private clearCast() {
    const g = this.g;
    for (const gd of [...this.ops, ...this.hostiles]) { if (gd.script) endGuardScript(gd, 'patrol'); despawnUnit(g, gd); }
    g.civilians = g.civilians.filter(c => !this.hostages.includes(c));
    this.ops = []; this.hostiles = []; this.hostages = []; this.entry = null; this.defs.clear(); this.act = null;
    this.searchT.clear(); this.searchA.clear(); this.flow.reset(); this.ending.reset(); this.flush.reset(); this.marks(false);
  }
  /** the scenario's own additions to the squad's door work, for as long as it is up: the side door's marks from the server room (SIDE_MARKS → orders.ts
   *  doorMarks) and the room's two doors linked on the charge trigger (doortools.ts linked) */
  private marks(on: boolean) {
    const g = this.g, S = this.data, side = S?.side, fd = S ? g.doors.byName(S.room.door) : undefined, sd = side ? g.doors.byName(side.door) : undefined, M = g.orders.doorMarks;
    for (let i = M.length - 1; i >= 0; i--) if (sd && M[i].door === sd) M.splice(i, 1);
    g.doorTools.linked = g.doorTools.linked.filter(d => d !== fd && d !== sd); g.doorTools.lights = null;
    if (!on) return;
    if (sd && side?.stack?.length) M.push({ door: sd, side: this.faceOf(sd), stack: side.stack, standoff: side.standoff ?? side.stack });
    g.doorTools.linked.push(...[fd, sd].filter((d): d is Door => !!d));
  }
  /** the face of one of the room's doors that is NOT the room's (orders.ts sideOf's sign): the side the team comes at it from */
  private faceOf(d: Door): 1 | -1 { const B = this.bounds; return B ? (-sideOf(d, [(B.x0 + B.x1) / 2, 0, (B.z0 + B.z1) / 2]) as 1 | -1) : 1; }

  /** Everybody on his mark on the fresh encounter restartEncounter has just made (no unit standing, no civilian, the level as authored, Sam at the spawn). */
  private stage() {
    const g = this.g, K = RESCUE, S = this.data; if (!S) throw new Error('the level has no hostage rescue data');
    const MARKS = { hostages: S.hostages, captor: S.captor, doorIn: S.doorMan.inside, doorOut: S.doorMan.outside, roamers: S.roamers };
    this.ops = []; this.hostiles = []; this.hostages = []; this.entry = null; this.defs.clear();
    this.called = false; this.searchT.clear(); this.searchA.clear(); this.flow.reset(); this.ending.reset(); this.flush.reset();
    this.plan = g.level.clearPlans?.find(p => p.door === S.room.door) ?? null; this.marks(true);
    { const sv = S.side?.from ? g.doors.byName(S.side.from.door) : undefined; this.serverOut = sv ? this.outOf(sv, this.fromBounds, 1.0) : null; }
    g.fight.reset(); g.fight.blindFire = K.blindFire;   // (the room fight starts clean on the new cast)
    if (K.roomLights && S.room.lights) g.setGroup(S.room.lights, true);
    const d = g.doors.byName(S.room.door)!; d.reset(); d.angle = 0; d.vel = 0; d.latched = true; d.closing = false;   // the hostage room's door shut on its latch (the night leaves it ajar)
    const side = S.side ? g.doors.byName(S.side.door) : undefined; if (side) { side.reset(); side.angle = 0; side.vel = 0; side.latched = true; side.closing = false; side.locked = false; }   // the side door from the server room: shut, and for the scenario not locked — the flank that has to be earned through the server room's own locked door
    // the opposition: captor, door man, roamers — put down to hold their marks, calm, until their own ears and eyes say otherwise (held hostile units: the room
    // fight's defenders from the first frame — what each watches, fears and does about it is his own; this keeps his set clock)
    const cast: { role: Role; inRoom: boolean; mark: { pos: Vec3; aimAt: Vec3 }; name: string }[] = [{ role: 'captor', inRoom: true, mark: MARKS.captor, name: 'Captor' }];
    cast.push(K.doorMan === 'inside' ? { role: 'door', inRoom: true, mark: MARKS.doorIn, name: 'Door man' } : { role: 'door', inRoom: false, mark: MARKS.doorOut, name: 'Door man' });
    (S.guards ?? []).forEach((m, i) => cast.push({ role: 'guard', inRoom: true, mark: m, name: `Room guard ${i + 1}` }));   // (the level's own: more guns that never leave the room — in no crew, not the door man)
    if (S.cast) {   // THE LEVEL'S OWN CAST: so many of its roamer marks manned, which ones drawn here (a partial shuffle off the staging's dice — no room is known empty)
      const idx = MARKS.roamers.map((_, i) => i), n = Math.max(0, Math.min(idx.length, Math.round(S.cast.roamers)));
      if (K.seedRoamers && !SAMESTART.on) for (let i = 0; i < n; i++) { const j = i + Math.floor(simRandom() * (idx.length - i)); [idx[i], idx[j]] = [idx[j], idx[i]]; }   // (seedRoamers off, or samestart.ts on: the first n marks, as written, and nothing drawn, here and twice below)
      idx.slice(0, n).sort((a, b) => a - b).forEach((m, i) => cast.push({ role: 'roamer', inRoom: false, mark: MARKS.roamers[m], name: `Roamer ${i + 1}` }));
    } else {
    const swap = K.seedRoamers && !SAMESTART.on && simRandom() < 0.5 ? 1 : 0;   // (a staging-time draw, like the clock's length below)
    for (let i = 0; i < Math.max(0, Math.min(MARKS.roamers.length, Math.round(K.roamers))); i++) cast.push({ role: 'roamer', inRoom: false, mark: MARKS.roamers[(i + swap) % MARKS.roamers.length], name: `Roamer ${i + 1}` });
    }
    cast.forEach((c, k) => {
      const gd = spawnUnit(g, { team: 'hostile', pos: c.mark.pos, yaw: v3.yawTo(c.mark.pos, c.mark.aimAt), name: c.name, light: K.defLights, budget: budgetWithSeats(g, 1 + cast.length) });   // (a level's own cast may be more than the eight the shadow capsules hold: this many, Sam and them)
      gd.hold = true; gd.pinned = true; this.hostiles.push(gd);
      this.defs.set(gd, { role: c.role, inRoom: c.inRoom, mark: c.mark, learned: NaN, set: false, sawFoe: false, released: false, burst: { until: -1, n: 0, next: 0 }, k, crew: '', look: null, spot: null, noise0: null });
    });
    gunDeal(g, this.hostiles.map((gd, k) => ({ gd, watch: cast[k].mark.aimAt, hostage: cast[k].role === 'captor' })));   // (gunroles.ts: the cast's guns off the table — nobody is touched and no die is drawn with its switch off)
    const capSet = K.start === 'hot' && K.captorSet; if (capSet) this.defs.get(this.hostiles[0])!.set = true;
    // the team at the lobby arch: the pair calm where they stand until an order moves them, Sam with the rifle in his hands
    const yaw = S.team.opsYaw ?? Math.PI / 2, at: Vec3[] = [K.opsAt[0], K.opsAt[1]];
    for (let i = 2; i < K.squad; i++) at.push(S.team.ops?.[i] ? v3.copy(S.team.ops[i]) : g.nightBreach.standBy(at[i - 2], [...at, K.samAt], yaw));   // (the loadout's third and fourth: the level's own marks where it has them, else a stride behind the pair's)
    for (let i = 0; i < K.squad; i++) {
      const m = K.squad > 2 ? SQUAD4[i] : null;   // (up to two: the pair as ever, in no pair of the order menu's — three or four: the four-man squad's roster and colours)
      const gd = spawnUnit(g, m ? { team: 'operative', pos: at[i], yaw, name: m.name, light: false, pair: m.pair, tint: m.tint, tint2: m.tint2, budget: Math.max(budgetWithSeats(g, SQUAD4_BUDGET), seatCount(g) + 1 + g.guards.length) } : { team: 'operative', pos: at[i], yaw, name: i === 0 ? 'Blue One' : 'Blue Two', light: false, budget: seatCount(g) + 1 + g.guards.length });
      gd.hold = true; gd.pinned = true; gd.trained = K.trained; this.ops.push(gd);
    }
    // the hostages: the building's own two civilians' names and clothes when the level has them
    const defs = g.level.civilians ?? [];
    MARKS.hostages.forEach((m, i) => {
      const cd = defs[i];
      this.hostages.push(spawnHostage(g, { name: cd?.name ?? (['Marta', 'Owen', 'Ines'][i] ?? `Hostage ${i + 1}`), kind: cd?.kind ?? (i === 0 ? 'cleaner' : 'worker'), tint: cd?.tint ?? (i === 0 ? [0.3, 0.42, 0.5] : i === 1 ? [0.5, 0.48, 0.42] : [0.46, 0.33, 0.36]), tint2: cd?.tint2, pos: m.pos, face: m.face }));
    });
    this.resolve = 0; this.resolveSecs = Math.max(5, K.resolveSecs + (SAMESTART.on ? 0 : (simRandom() * 2 - 1) * K.resolveSpread)); this.t0 = NaN;   // (samestart.ts on: the clock is the knob's to the second; else a staging-time draw, as every Guard's construction draws: nothing is drawn while it runs)
    this.doorOrders = 0; this.blindRounds = 0; this.result = null;
    this.act = null; this.capBy = null; this.executed.clear(); this.counted.clear(); this.securedAt = NaN; this.step = 'waiting'; this.barkAt = 0; this.noise = null; this.alarmAt = NaN; this.way = null; this.lastOrder = g.orders.log[g.orders.log.length - 1] ?? null;
    const pl = g.player;
    g.teleportPlayer(v3.copy(K.samAt)); pl.char.bodyYaw = pl.char.aimYaw = K.samYaw;
    if (pl.hasRifle) pl.slot = 'rifle'; pl.holstered = false;
    this.staged++; this.said.clear(); this.cut = true; this.murmurAt = g.time + 4; this.murmurN = 0; this.ackAt = NaN;
    kitStage(g, this.ops);   // each member's kit and uniform (kit.ts): nothing written on a teammate while every card is as issued (your own four counts are, under the pouches rule: kitPlayer)
    this.note('staged'); if (capSet) this.note(`set:${this.hostiles[0].callsign}`);
    const who = `${NUMBER[S.hostages.length] ?? S.hostages.length} hostage${S.hostages.length === 1 ? '' : 's'} in the ${this.place()}`;
    const n = this.ops.length, fall = n > 2 ? ', and the squad falls in' : n === 2 ? ', and the pair falls in' : n ? ', and your teammate falls in' : '';   // (the loadout's size — two: the line as it always read)
    g.msg(K.start === 'hot'
      ? `hostage rescue. you hear shots inside. ${who}. the clock starts when you move${fall}. ${n ? 'Press F to bang and clear, or X for orders (door tools on any door)' : "You're alone"}`
      : `hostage rescue. ${who}. nobody knows you're here yet. ${n ? 'Press F to stack up and F again to go, or X for orders' : "You're alone"}`);
  }
  /** the hostage room in words: its name and where it is ("manager's office, far end of the corridor") */
  private place(): string { const R = this.data?.room; return R ? `${R.name}${R.where ? `, ${R.where}` : ''}` : 'hostage room'; }
  note(what: string) { this.log.push({ t: this.g.time, what }); if (this.log.length > 600) this.log.splice(0, 150); }
  private get over(): boolean { return this.result !== null; }
  /** dynamic tempo: the clock is running (the team stops waiting for the word door by door) */
  get dynamic(): boolean { return !isNaN(this.t0); }

  /** What F does, for a caller that is not the key (the panel's button, the console): a charge set on a door has the press — it blows — else it is Sam's word
   *  on the hostage room. */
  press(): 'blow' | 'stack' | 'go' | 'bang' | null { { const __s = RUST_SURFACE; if (__s) { const __r = __s.call('Rescue.press', this, []); if (__r !== RUST_PASS) return __r as any; } }
    const DT = this.g.doorTools; if (!this.active || this.g.player.down) return null;
    DT.sync();   // (a staging this frame: last run's charges are not this one's)
    if (DT.set.length) { DT.trigger(); return 'blow'; }
    return this.orderDoor();
  }
  /** Sam's word on the hostage room (F — a charge set on a door has had the press before this is asked: doortools.ts). The clock running: one press is bang and
   *  clear — stack, and go the moment they are set. Before it (quiet start): the first press stacks them, the second sends them in. Returns what the press did. */
  orderDoor(): 'stack' | 'go' | 'bang' | null {
    const g = this.g, DT = g.doorTools;
    if (!this.active || this.over || g.player.down) return null;
    DT.sync();
    const hot = RESCUE.start === 'hot', E = this.entry;
    if (E) {   // under way: the one thing left to say is the second word, to men stacked and holding for it
      if (E.gate.go) return null;
      E.gate.go = true; this.doorOrders = 2;
      this.note('order:go'); g.msg('you: go');
      return 'go';
    }
    if (this.called) { if (!this.captor()) return null; this.called = false; }   // the room is theirs already — unless one of theirs has come back into it: then the order stands again
    const S = this.data; if (!S) return null;
    const fd = g.doors.byName(S.room.door), sd = (S.side ? g.doors.byName(S.side.door) : null) ?? null; if (!fd) return null;
    const on = DT.ops.filter(o => o.door === fd || o.door === sd), ch = on.find(o => o.kind === 'charge');   // the menu already has men at work on one of the room's doors: F does not take them off it
    if (ch && (DT.chargeOn(ch.door) || ch.phase === 'place')) { g.msg(DT.chargeOn(ch.door) ? 'the charge is set. press F to blow it' : 'the charge is not set yet'); return null; }   // (a charge already blown: its men are going in — the next line's answer)
    if (on.some(o => o.kind !== 'outside')) { g.msg('they are on that door already'); return null; }
    const who = this.pick(); if (!who) { g.msg(!this.ops.length ? "you're alone, so you'll have to take the hostage room yourself" : "nobody's free for the hostage room. everyone still up is covering where you put them (X, then fall in)"); return null; }
    const dyn = this.dynamic || hot;   // (a hot start's first order starts the clock itself)
    const two = this.launch(fd, sd, who.front, who.side, dyn); if (two === null) return null;
    const did: 'stack' | 'bang' = dyn ? 'bang' : 'stack';
    this.doorOrders = dyn ? 2 : 1;
    if (hot) this.startClock('order');   // (after the men are spoken for: nobody falls in unasked)
    this.note(`order:${did}`); g.msg(did === 'bang' ? `you: ${this.entry?.ops.some(e => e.op.kind === 'bangClear') ? 'bang' : 'open'} and clear — hostage room${two}` : `you: stack up — hostage room${two}`);
    return did;
  }
  /** Who F takes, and to which door (null: nobody is free). A man Sam has put on a 'cover' of his own stays on it — unless he stands in the server room, where
   *  any man Sam has posted is the side door's; with nobody posted there and Sam himself in it with both men, the man nearer the side door takes it. */
  private pick(): { front: Guard[]; side: Guard[] } | null {
    const g = this.g, O = g.orders, up = this.ops.filter(o => o.state !== 'dead'); if (!up.length) return null; if (!RESCUE.twoDoor) return { front: up, side: [] };   // (the switch off: F takes whoever is up, as it always did)
    const posted = (o: Guard) => { const A = O.of(o); return !!A && A.post !== null && !this.auto.has(A) && !this.flow.gave(A); };   // Sam put him there himself
    const sd = this.data?.side ? g.doors.byName(this.data.side.door) : undefined, SB = this.fromBounds;
    const side: Guard[] = [];
    if (sd && SB && this.bounds) {
      const near = (l: Guard[]) => l.filter(o => inBounds(o.char.pos, SB)).sort((a, b) => flat(a.char.pos, sd.frameCentre) - flat(b.char.pos, sd.frameCentre))[0] ?? null;
      let first = near(up.filter(posted));
      if (!first && inBounds(g.player.char.pos, SB)) { const free = up.filter(o => !posted(o)); if (free.length >= 2) first = near(free); }
      if (first) { side.push(first);
        // a level laid out for four (side.pair) and RESCUE.sidePair: his pair mate in the same room goes through the side door behind him — a pair on each door —
        // as long as that leaves somebody for the main door (a pair alone in there still splits one and one, as ever)
        const pr = first.unit?.pair, mate = RESCUE.sidePair && this.data?.side?.pair && pr ? up.find(o => o !== first && o.unit?.pair === pr && inBounds(o.char.pos, SB)) : undefined;
        if (mate && up.some(o => o !== first && o !== mate && !(posted(o) && O.of(o)!.order === 'cover'))) side.push(mate); }
    }
    const front = up.filter(o => !side.includes(o) && !(posted(o) && O.of(o)!.order === 'cover'));
    return front.length || side.length ? { front, side } : null;
  }
  /** F's entry, given: the door tools' own order on the corridor door for `front` and on the side door for `side` (doortools.ts order — bang and clear while the
   *  squad has a can and RESCUE.stun is on, open and clear otherwise), through one gate; `go` false = they stack and hold for the second word. Returns the
   *  words for which doors (null: no order could be given, and why has been said). */
  private launch(fd: Door, sd: Door | null, front: Guard[], side: Guard[], go: boolean): string | null {
    const g = this.g, DT = g.doorTools;
    const gate = DT.together(go), ops: Entry['ops'] = [], why: string[] = [];
    const give = (who: Guard[], d: Door, isSide: boolean) => {
      const op = DT.order(RESCUE.stun && DT.bangs > 0 ? 'bangClear' : 'openClear', who, d, this.faceOf(d), gate);
      if (typeof op === 'string') { why.push(`${isSide ? 'side door' : WORDS.on && !isOffice(g.level) ? 'main door' : 'corridor door'} — ${op}`); return; }
      this.flow.release(op.men);   // (theirs no longer: the flow's jobs on them end where they stand)
      ops.push({ op, side: isSide, seen: 0, kicked: false, ended: null });
      for (const o of op.men) this.roundBy(o);
    };
    if (front.length) give(front, fd, false);   // (the corridor door first: with one can left it is that door's, and the side door's man goes in behind it — doortools.ts)
    if (side.length && sd) give(side, sd, true);
    if (why.length || !ops.length) g.msg(`the hostage room: ${why.join(' · ') || 'nobody to take it'}`);
    if (!ops.length) return null;
    const cans = ops.filter(e => e.op.kind === 'bangClear').length;
    if (RESCUE.stun && cans < ops.length) g.msg(cans ? `one flashbang left, so it'll go in through the ${ops.find(e => e.op.kind === 'bangClear')!.side ? 'side' : isOffice(g.level) ? 'corridor' : 'main'} door and the teammates at the other door will go in after it` : "no flashbangs left, so they'll go in without one");
    const taken = ops.flatMap(e => e.op.men), split = !!this.bounds && RESCUE.twoDoor && (ops.some(e => e.side) || taken.length === 1);
    this.entry = { ops, gate, men: taken, split, setT: NaN, goT: NaN, inT: NaN }; this.said.delete('set');
    if (split) this.note(`split:${ops.map(e => `${e.side ? 'side' : 'front'}=${e.op.men.map(m => m.callsign).join('+')}`).join(',')}`);
    return ops.length > 1 ? ' · both doors' : ops[0].side ? ' · side door' : '';
  }
  private startClock(why: string) {
    if (!isNaN(this.t0)) return;
    const g = this.g;
    this.t0 = g.time; this.note(`clock:${why}`);
    // dynamic tempo: nobody had to say it — where Sam goes now, the pair flows (unless he has already given them something to do)
    const up = this.ops.filter(o => o.state !== 'dead');
    if (this.mode === 'free' && up.length) { const p = g.player.char.pos, idle = up.filter(o => !g.orders.has(o));   // (a first order to ONE man leaves the other at the arch otherwise)
      if (idle.length === up.length) g.orders.give('fallIn', { kind: 'none', at: [p[0], 0, p[2]] }, 'all');
      else for (const o of idle) g.orders.give('fallIn', { kind: 'none', at: [p[0], 0, p[2]] }, g.orders.squad().indexOf(o));
      if (idle.length) this.lastOrder = g.orders.log[g.orders.log.length - 1] ?? null; }
  }




  /** cues.ts: a man of Hard's crew lying in wait — on his gun from a side room, or staying dark to come up behind — whose place the net must not give away */
  crewHides(gd: Guard): boolean { const D = this.defs.get(gd); return !!D && (D.crew === 'cross' || D.crew === 'wait') && this.crewOn; }
  /** Hard's rescue crew is on: Hard's own row (difficulty.ts HARD.rescueCrew → PRESSURE.rescueCrew) and one of its three rules */
  private get crewOn() { return PRESSURE.on && PRESSURE.rescueCrew && (RESCUE.crewDoor || RESCUE.crewCross || RESCUE.crewBehind); }

  /** A man F sends to the corridor door from inside the server room goes out the way he came in — by the server room's own door: the short way is the side door
   *  and the hostage room itself (the side door is unlocked for the scenario, and a leaf opens to anybody in uniform). His first leg is the corridor outside it
   *  (orders.ts walks its own way round only for a door locked against Sam). */
  private roundBy(op: Guard) {
    const A = this.g.orders.of(op), SB = this.fromBounds;
    if (!A || !A.post || A.via.length || !SB || !this.serverOut || !inBounds(op.char.pos, SB) || inBounds(A.post, SB)) return;
    A.via = [v3.copy(this.serverOut)]; A.goal = A.via[0];
  }
  /** A man the scenario lets go of where he stands: handed to the order menu as its 'cover' (gun up on `look` — default the way he faces — until Sam says
   *  otherwise; the room fight shoots for him through it). */
  private post(op: Guard, look?: Vec3, door?: string) {
    const p = op.char.pos, y = look ? v3.yawTo(p, look) : op.char.bodyYaw;
    const watch: Vec3 = look ? [look[0], 1.1, look[2]] : [p[0] + Math.sin(y) * 4, 1.1, p[2] + Math.cos(y) * 4];
    const words = door ? `covering ${door}` : 'holding';
    this.g.orders.assign(op, { order: 'cover', post: [p[0], 0, p[2]], face: y, watch, stance: 'none', upper: 'aim', going: words, there: words });
    const A = this.g.orders.of(op); if (A) this.auto.add(A);
  }


  /** rescuecheck.ts: the living and the dead of the cast as their radio check reads them — who he is, whether he is posted in the hostage room, his post,
   *  when he learned of the team (NaN: he has not), whether the rescue has let him off his post, and whether Hard's crew has dealt with him (given him a
   *  part, or let him out of it: either way the crew's own words and moves cover him) */
  cast(): { gd: Guard; role: 'captor' | 'door' | 'guard' | 'roamer'; inRoom: boolean; post: Vec3; learnedAt: number; released: boolean; crewed: boolean }[] {
    return this.hostiles.map(gd => { const D = this.defs.get(gd)!; return { gd, role: D.role, inRoom: D.inRoom, post: D.mark.pos, learnedAt: D.learned, released: D.released, crewed: D.crew !== '' }; });
  }

  /** the floor just outside the hostage room's main door (rescueflush.ts: which man is nearest it) */
  get outsidePoint(): Vec3 { return goneWithTick('Rescue.outsidePoint') as any; }
  /** the floor a metre outside the hostage room's side door; null on a level with none */
  get sideOutside(): Vec3 | null { return goneWithTick('Rescue.sideOutside') as any; }
  /** who holds the hostages right now: the living man of theirs in the office nearest a living hostage (null = nobody: the clock stands) */
  captor(): Guard | null { { const __s = RUST_SURFACE; if (__s) { const __r = __s.call('Rescue.captor', this, []); if (__r !== RUST_PASS) return __r as any; } }
    const B = this.bounds; let best: Guard | null = null, bd = 1e9;
    const all = this.hostages.filter(h => h.state !== 'dead'); if (!all.length) return null;
    const alive = RESCUEEND.on || RESCUEFLUSH.on ? all.filter(x => inBounds(x.char.pos, B)) : all;   // (the rules that let a hostage leave the room: a man standing in it has the ones still IN it — one walked out of it is had by line and reach, flush.holder below)
    const was = this.capBy; if (alive.length && was && was.state !== 'dead' && !was.held && inBounds(was.char.pos, B)) return was;   // the man who has it keeps it while he can: a second man walking in takes nothing over
    if (alive.length) for (const h of this.hostiles) {
      if (h.state === 'dead' || h.held || !inBounds(h.char.pos, B)) continue;
      const d = Math.min(...alive.map(x => flat(x.char.pos, h.char.pos))); if (d < bd) { bd = d; best = h; }
    }
    return best ?? this.flush.holder(was);   // (rescueflush.ts, null with its switch off: nobody of theirs in the room — a man with a line to one of them from outside it has her)
  }
  /** THE CAPTOR'S LAST WARNING, as the men at a door have it (doortools.ts DOORTOOLS.thrLast): `d` is one of the hostage room's doors, a man of theirs still
   *  holds the hostages, and his clock is at its last step — 'about to' (the step the HUD words for Sam the moment it is reached, and the captor's "last
   *  chance! i mean it!" from his next bark: never a count of seconds) — or his gun is on one of them already ("that's it — time's up!"), or one of them
   *  has been shot. The step itself is the cue: no sound is propagated for it. Read only: nothing of the clock is touched */
  lastWarning(d: Door): boolean { if (RUST_SURFACE?.tapFiring('Rescue.lastWarning')) return RUST_SURFACE.tapAnswer() as any; { const __s = RUST_SURFACE; if (__s) { const __r = __s.call('Rescue.lastWarning', this, [d]); if (__r !== RUST_PASS) return __r as any; } }
    const S = this.data; if (!this.active || this.over || !RESCUE.clock || isNaN(this.t0) || !S) return false;
    if (d.def.name !== S.room.door && d.def.name !== S.side?.door) return false;
    return (this.step === 'about to' || this.act !== null || this.executed.size > 0) && this.captor() !== null;
  }
  /** the act is over, one way or the other: the man off the scenario's script and back on the room fight's roster as it had him */
  private endAct() {
    const A = this.act; if (!A) return; this.act = null;
    const by = A.by, F = this.g.fight;
    if (by.state !== 'dead' && by.script) endGuardScript(by, 'alert');
    if (by.state !== 'dead') by.hold = A.hold;
    if (A.def && this.g.guards.includes(by) && !F.defs.some(d => d.gd === by)) F.defs.push(A.def as typeof F.defs[number]);
  }

  /** the hostages' count, the room secured and held, the run's end */
  private outcome() {
    const g = this.g, K = RESCUE; if (this.over) return;
    for (const h of this.hostages) {
      if (h.state !== 'dead' || this.counted.has(h)) continue; this.counted.add(h);
      if (this.executed.has(h)) { this.note(`executed:${h.callsign}`); g.msg(`${h.civ?.name ?? 'a hostage'} has been shot`); }
      else if ((RESCUEEND.on || RESCUEFLUSH.on) && this.hostiles.some(x => x.char.id === h.killerId)) { this.note(`killed:${h.callsign}`); g.msg(`${h.civ?.name ?? 'a hostage'} has been killed`); }   // (a hostage on her feet caught by a grenade of THEIRS: frag.ts names its thrower — theirs to answer for, as a round of theirs is)
      else { this.note(`teamfire:${h.callsign}`); this.end('lost', `${h.civ?.name ?? 'a hostage'} was hit by your own fire`); return; }
    }
    const alive = this.hostages.filter(h => h.state !== 'dead').length;
    if (!alive) { this.end('lost', this.hostages.length === 2 ? 'both hostages are dead' : this.hostages.length === 1 ? 'the hostage is dead' : 'every hostage is dead'); return; }
    if (g.player.down) { this.end('lost', 'you are down'); return; }   // (the game's own down card follows; the pair fighting on without Sam is for later)
    const B = this.bounds, pad = isNaN(this.securedAt) ? 0 : 0.4;   // (once it is held, a step onto the threshold does not let go of it; and a room taken without a sound — a quiet start's clock never started — is taken all the same)
    const theirs = this.hostiles.some(h => h.state !== 'dead' && !h.held && inBounds(h.char.pos, B));
    const ours = (!g.player.down && inBounds(g.player.char.pos, B, pad)) || this.ops.some(o => o.state !== 'dead' && inBounds(o.char.pos, B, pad));
    if (RESCUEEND.on) { const how = this.ending.won(); if (how) { this.end('rescued', how === 'out' ? `${alive} of ${this.hostages.length} hostages out` : `${alive} of ${this.hostages.length} hostages alive · building cleared`); return; } }   // (rescueend.ts: the two ways it is won — the hold below ends nothing)
    const kept = RESCUEEND.on && !isNaN(this.securedAt) && (!theirs || !this.hostages.some(h => h.state !== 'dead' && inBounds(h.char.pos, B)));   // (the rule on: a room taken stays taken while nobody of theirs is back in it with the hostages — the team may walk out of it with them, and leave them anywhere)
    if ((!theirs && ours) || kept) {
      if (isNaN(this.securedAt)) { this.securedAt = g.time; this.note('secured'); g.msg(RESCUEEND.on ? 'hostage room secured. now bring them out or clear the building' : `hostage room secured. hold it for ${Math.round(K.holdSecs)} seconds`);
        if (WORDS.on && !this.said.has('h:secured')) { this.said.add('h:secured'); const h = this.hostages.find(x => x.state !== 'dead'); if (h && hostageSay(g, h, 'secured')) this.ackAt = g.time + 2.4; } }
      else if (!RESCUEEND.on && g.time - this.securedAt >= K.holdSecs) this.end('rescued', `${alive} of ${this.hostages.length} hostages alive`);
    } else if (!isNaN(this.securedAt)) { this.securedAt = NaN; this.note('unsecured'); g.msg(theirs ? "contact in the hostage room, so it isn't yours yet" : 'nobody is holding the hostage room'); }
  }
  private end(state: 'rescued' | 'lost', why: string) {
    const g = this.g;
    const team = (g.player.down ? 0 : 1) + this.ops.filter(o => o.state !== 'dead').length;
    const secs = isNaN(this.t0) ? 0 : (isNaN(this.securedAt) || state === 'lost' ? g.time : this.securedAt) - this.t0;
    const timed = isNaN(this.t0) ? 'room secured before anybody knew' : isNaN(this.securedAt) && state === 'rescued' ? `${secs.toFixed(0)} s after the clock started` : `room secured ${secs.toFixed(0)} s after the clock started`;   // (a floor cleared with the room never stood in — rescueend.ts — has no 'secured' to time)
    const left = this.hostages.filter(h => h.state !== 'dead').length;
    const grade = state === 'lost' ? 'failed' : left === this.hostages.length ? 'clean' : 'costly';
    this.result = { state, grade, why, hostages: left, team, secs };
    this.endAct();
    this.note(`${state}${state === 'lost' ? `:${why}` : ''}`);
    g.msg(state === 'rescued'
      ? `${grade === 'clean' ? 'CLEAN' : 'COSTLY'} — ${why} · team ${team}/${1 + this.ops.length} up · ${timed}. press Shift+Enter to restart`
      : `FAILED — ${why}. press Shift+Enter to restart`);
  }

  /** where it stands, in one word */
  get state(): RescueState {
    if (!this.active) return 'off';
    if (this.result) return this.result.state;
    if (!isNaN(this.securedAt)) return 'secured';
    return isNaN(this.t0) ? 'staged' : 'running';
  }
  /** one line for the panel / the console */
  summary(): string {
    if (!this.active) return 'hostage rescue off';
    const up = (l: Guard[]) => l.filter(x => x.state !== 'dead').length;
    const clock = !RESCUE.clock ? 'clock off' : isNaN(this.t0) ? 'clock not started' : RESCUE.clockAsSeconds ? `clock ${(this.resolve * this.resolveSecs).toFixed(0)}/${this.resolveSecs.toFixed(0)} s` : `captor ${this.step}`;
    const set = this.hostiles.filter(h => h.state !== 'dead' && this.defs.get(h)?.set).length;
    return `hostage rescue ${this.state} · hostages ${up(this.hostages)}/${this.hostages.length} · ${clock} · team ${up(this.ops)}/${this.ops.length} up${this.entry ? `, F's entry (${this.entry.ops.map(e => `${e.side ? 'side' : 'front'}: ${e.ended ?? e.op.phase}`).join(', ')})` : ''} · defenders ${up(this.hostiles)}/${this.hostiles.length} up, ${set} set${this.blindRounds ? ` · ${this.blindRounds} blind rounds` : ''}${this.result ? ` · ${this.result.why}` : ''} · staged ×${this.staged}${RESCUE.flow || RESCUE.marks ? ` · ${this.flow.summary()}` : ''}`;
  }
}
