// The response to going loud (STATUS E12(0) — the maintainer: "right now it's just too easy to just go lethal and clear the area … add QRF"). One model, no magic:
// the trigger is what the floor's own alarm and ears already know (a locked-down floor that has HEARD Sam's gunfire — the hearing model's own 80 m gate for a
// report at level ≥ 1, so the suppressed pistol's odd noise never counts and the FN2000's crack always does — or has walked onto two of its men down), the
// telegraph is radio traffic on the net the HUD already shows (the request, control's ETA, thirty seconds, ten seconds — and boots on the stairs through the
// door), and the men are ordinary Guards (guards.ts spawnUnit, side 'guard': the shift's colleagues, Sam's enemies, on the same net) who come up the west
// stairwell, throw its steel fire door open into the corridor with their weapon lights behind it, and clear toward the last fix as ONE four-man element through
// the squad runner (squad.ts order): a file behind the point man's own trail, muzzles covering the openings either side and the rear, the room holding the fix
// stacked on, banged and flowed into by all four (the level's own RoomClearPlan marks for the first two, the deep point and the door for three and four),
// called, held all round for a beat, then the next room by walking distance — until one of them SEES him (the shared sight test at the shared rate, or a bump,
// or a colleague held in front of them), when all four are handed to the ordinary alert AI (closing, bursts, muzzle discipline, suppression, the standoff) and
// re-formed once nobody has had him for eight seconds. Carbines with lights, one stun charge a man, hard men, trained hands (aim.ts TRAINED), short beats.
// They never stand down, never leave, and there is one wave a night (a second request is refused on the radio). Past that — a QRF man killed, three men down,
// or a running gunfight — control locks the COMPOUND down: the exterior lamps come up and men take posts outside every exit (spawned if the renderer's
// character budget has room; on this floor it has none — Sam + 3 staff + 4 QRF is the 8 the shadow-capsule list holds — so the element's rear pair peels off
// and jogs out through the doors to them), pinned there covering the doors from outside: leaving by the fire exit is now a walk into their arcs.
// State on Game.qrf; knobs on Game.tune (qrf / qrfEta / qrfSize / perimeter / perimeterEta / perimeterSize); reset with the
// guards (Game.resetGuards → resetQrf; the men are units, resetGuards despawns them). Never under the tour, the breach's daylight, or with the AI off.
// THE ROOM IS TAKEN BY THE SQUAD'S OWN CODE (tune.qrfSquad — the page ships it on; off = the element's own drill below on the level's authored marks, as it
// was): at the door the four are handed, as a second cast (squadcast.ts), to the very classes Sam's squad runs — doortools.ts's stack on the jambs worked out
// from the door, the hand on the leaf, the can, who goes first, each man's corners, the search that flows them on through the room until none of its floor
// is unseen; orders.ts's walk onto a mark and pose on it. A room that holds the fix is banged and entered; any other is worked from the threshold first
// (sliced from the jambs, its blind pocket banged) and then stepped into. What the squad's code learns, they have with no second edit. Everything else is
// still theirs: when, where to, the file down the corridor, their senses, and the fight on contact (startDrill / phaseDrill).
// THE TEAM IS THE LEVEL'S OWN (`RESPONSE` below — the page ships it on; off = everything above on the office's one stairwell, as it was): a level has a
// team only if its file says so (scenarios.response: who they are, the ways they come in by, how many and how soon as ranges, where the second lift
// stands, and WHY this place has one), each call is dealt a way, a size and a time off the game's own die, the radio speaks the level's words, and where
// they go is what was SAID on the defenders' net (picture.ts: an idea a man on the net held long enough to say, or was told) — kept by control past the
// man who said it — or the alarm's own place, never where the player is. With nothing said, on a floor with no room plans, they walk the level's named
// places. Whoever is against them on the level (a partner, a teammate) is their enemy by the shared sight test, not Sam alone.
// QA: tools/qa/qrf-probes.ts (`probes.ts qrf`), soak.ts --loud-at; tools/qa/qrf-squad-check.ts (the squad's drill); tools/qa/response-check.ts (the level's own team).
import { allDown, seatCount } from './seats';
import { SideSet } from '../math/side';
import { QRF_TINT, QRF_TINT2 } from './consts';
import { Vec3, v3, clamp } from '../math/vec';
import './character';
import './handleside';
import type { Game, Guard } from './game';
import type { RoomClearPlan } from '../scene/level';
import type { ResponseData, ResponsePost } from '../scene/scenarios';
import type { Item } from './throwables';
import { radioHand } from './handwork';
import { MAX_SKINNED, spawnUnit, despawnUnit, escalationOf, bubbleSecs } from './guards';
import './teams';
import './picture';
import { SQUAD_MAX_MEN } from '../scene/scenarios';
import { order, endGuardScript } from './squad';
import './orders';
import type { DoorOp } from './doortools';
import { type SquadCast } from './squadcast';
import './combat';
import { isDaylight, setGroup } from './lighting';
import './dialogue';
import './tactics';
import { w, wg } from './words';
import './doornames';   // (a leaf: the name a level wrote for a door)
import './aim';
import { missionLive } from './mission';
import { chronRadio, chronicle } from './chronicle';
import { simRandom } from '../math/dice';   // the simulation's die (math/dice.ts): the Game's own when it has one, Math.random otherwise
import './rails';   // (a spared teammate on his knee is nobody the response makes out: asked inside its sight loop only)
import { hasHim, OWN } from './owner';
import { word } from './belief';
import { RUST_PASS, RUST_SURFACE } from './crossing';

// ================================================================ authored marks (TODO(level-format lane): these belong in LevelData beside routes / chokepoints / clearPlans)
/** Everything the response needs from THIS level: the door it comes through, where the men stand on the landing behind it (in element order: #1 at the bar), the
 *  fan they take up just inside, the outside posts of the perimeter tier (facing = Character yaw, forward [sin, 0, cos]: 0 south, π/2 east) with the stretch of
 *  wall each sweeps his light across, and where outside men are stood up when the character budget lets the tier spawn them. Checked headlessly against
 *  level.ts: free nav cells, straight walkable legs between neighbours, out of every leaf's sweep. */
export const QRF_MARKS = {
  door: 'stairwell',
  /** the landing (level.ts: inner floor x 1.34‥3.88, z 9.57‥12.63; the doorway's walkable column is z 11.25) */
  landing: [[3.1, 0, 11.25], [2.25, 0, 11.25], [3.1, 0, 10.3], [2.25, 0, 10.3]] as Vec3[],
  /** just inside: point man a few strides up the corridor facing east, two and three either side of him quartering the conference glass / the lobby arch, four
   *  last through holding the door end (the leaf swings in along the NORTH side of this end, everybody south or east of its sweep) */
  entry: [[6.7, 0, 11.2], [5.9, 0, 10.55], [5.9, 0, 11.8], [5.4, 0, 11.25]] as Vec3[],
  entryFace: [Math.PI / 2, 2.4, 0.75, Math.PI / 2],
  /** outside posts, in the order they are manned: the fire exit (the way out the mission names) from the lot's east strip, the break room's outside door from the
   *  walkway west of the van, the lobby glazing from under the dark west lamp, the south lot from under its lamp */
  posts: [
    { name: 'fire exit', pos: [38.9, 0, 24.2] as Vec3, yaw: -2.48, sweep: [[36.15, 1.0, 19.9], [36.15, 1.0, 21.5]] as Vec3[] },
    { name: 'break-room door', pos: [27.4, 0, 25.7] as Vec3, yaw: 1.95, sweep: [[30.9, 1.0, 24.15], [32.4, 1.0, 24.15]] as Vec3[] },
    { name: 'lobby front', pos: [2.3, 0, 20.6] as Vec3, yaw: Math.PI / 2, sweep: [[3.9, 1.0, 17.6], [3.9, 1.0, 22.4]] as Vec3[] },
    { name: 'south lot', pos: [17.4, 0, 26.4] as Vec3, yaw: Math.PI, sweep: [[14.5, 1.2, 24.15], [23.5, 1.2, 24.15]] as Vec3[] },   // (a stride east of the south lamp's pole)
  ],
  /** where the perimeter tier's own men are stood up when the budget has room for them: the lot's south-east corner by the gate, out of everybody's way */
  outsideSpawn: [[38.6, 0, 26.6], [37.4, 0, 26.9], [38.8, 0, 25.4], [37.2, 0, 25.6]] as Vec3[],
};

// ================================================================ the team is the level's own (RESPONSE)
/** THE RESPONSE TEAM AS LEVEL DESIGN (the maintainer, 10-18: "let's make it part of the level design and always make sense (ideally w.r.t. the concept/design
 *  of the level) and not all levels should have it"; "having some randomization and modeling their knowledge based on what they've heard on the radio could make
 *  it really fun"). ON in the page (`?response=0` = the build as it was), OFF here and on the frozen harness stand (TPS_RESPONSE=1; a FROZEN_HOLDS row).
 *  With `on`: a level has a team only where its file says so (scene/scenarios.ts ResponseData — the ways in, the marks, the words, how many and how long);
 *  the office's own block (levels/office.scenarios.ts) is the marks above, number for number. `deal` and `net` act only with `on`; each off = that rule as it
 *  was. Untuned by play. */
export const RESPONSE = {
  on: false,
  /** A RUN IS DEALT ITS RESPONSE when the call goes out (so after play began: samestart.ts leaves what is rolled in play alone): how many come, by which way,
   *  after how long — each inside what the level's block allows, off the game's own die. Off (and a call from the panel): the level's first answer */
  deal: true,
  /** THE TEAM GOES ON WHAT WAS SAID ON THE NET (picture.ts): the freshest place a man on the defenders' net has held long enough to have said, kept by
   *  control past the man who said it — never every living man's private fix the instant he has it. Off, or the picture off: the floor's fix, as it was */
  net: true,
  /** no way is dealt whose muster an enemy stands inside this many metres of when the call goes out (nobody is stood up at his elbow); a dealt time is to
   *  the nearest `etaStep` s; the net's word is read this often (s) */
  clearM: 12, etaStep: 5, wordSecs: 0.5,
  /** A SQUAD'S RAID BRINGS THE LEVEL'S TEAM TOO, where its block says so (ResponseData.raid): the team stands with the defenders, and is asked for by a
   *  defender of the block's room who heard gunfire and has lived `callSecs` since to say so — those men down first, nobody calls. Off: no raid ever
   *  brings a team, as it was (`?responseraid=0`) */
  raid: true, callSecs: 6,
};
/** one way in, with the level's words, as the code below reads it — the dealt way of the level's block, or (the switch off) the office's constant above */
interface Marks { door: string | null; name: string; muster: readonly Vec3[]; entry: readonly Vec3[]; entryFace: readonly number[]; coming: string; clear: string | null; stairs: boolean;
  posts: readonly ResponsePost[]; outside: readonly Vec3[]; site: string; at: string; team: string; tag: string; }
/** this level's block (null: it has none — with RESPONSE.on, no team) */
export function responseOf(g: Game): ResponseData | null { return g.level.scenarios?.response ?? null; }
/** A RAID ON A PLACE THAT HAS A TEAM TO CALL (RESPONSE.raid): a squad's room clear under way on a level whose block carries `raid`. The team is then the
 *  DEFENDERS' (side 'hostile': the squad's enemies and Sam's), counted inside the eleven a squad ever faces, and it has no second lift */
function raid(g: Game): boolean { return RESPONSE.on && RESPONSE.raid && !!g.nightBreach?.active && !g.rescue?.active && !!responseOf(g)?.raid && (!g.level.scenarios?.snatch || !!g.snatch?.out.on); }   // (a snatch level's block answers only the snatch that ends in the van — snatchout.ts: with that rule off the level is as it was, and nobody comes)
/** a raid: the men who can call it in — the living defenders of the block's room (where the set is), any defender when it names none; never the team itself */
function callers(g: Game): Guard[] {
  const R = responseOf(g), id = R?.raid?.room, nb = g.nightBreach;
  const room = id ? g.roomClear?.rooms.find(r => r.def.id === id) : null;
  const of: readonly Guard[] = room ? room.defenders : id ? [] : nb?.hostiles ?? [];
  return of.filter(x => x.state !== 'dead' && !x.held && !hasHim(x, OWN.runner) && g.guards.includes(x));
}
/** a raid: the man who makes the call now — a caller who heard gunfire himself and has lived `callSecs` since (null: nobody has, yet or ever) */
function raidVoice(g: Game): Guard | null { const Q = g.qrf; return callers(g).find(x => g.time - (Q.heardAt.get(x) ?? Infinity) >= RESPONSE.callSecs) ?? null; }
/** …and its name on the net, for the line that says so */
export function responseTeam(g: Game): string { return responseOf(g)?.team ?? 'the response team'; }
function marksOf(g: Game): Marks | null {
  const Q = g.qrf;
  if (Q.marks && Q.marksOn === RESPONSE.on && Q.marksWay === Q.way) return Q.marks;
  let M: Marks | null;
  if (!RESPONSE.on) M = { door: QRF_MARKS.door, name: 'the west stairwell', muster: QRF_MARKS.landing, entry: QRF_MARKS.entry, entryFace: QRF_MARKS.entryFace, coming: 'coming up the west stairs', clear: 'nobody west of the lobby arch', stairs: true,
    posts: QRF_MARKS.posts as unknown as ResponsePost[], outside: QRF_MARKS.outsideSpawn, site: 'two-six', at: 'on two-six', team: 'the response team', tag: 'QRF' };
  else {
    const R = responseOf(g), y = R ? R.ways[Q.way] ?? R.ways[0] : null;
    M = R && y ? { door: y.door ?? null, name: y.name, muster: y.muster, entry: y.entry, entryFace: y.entryFace, coming: y.coming ?? `coming in by ${y.name}`, clear: y.clear ?? null, stairs: !!y.stairs,
      posts: R.posts ?? [], outside: R.outside ?? [], site: R.site, at: R.at, team: R.team, tag: R.tag ?? 'QRF' } : null;
  }
  Q.marks = M; Q.marksOn = RESPONSE.on; Q.marksWay = Q.way;
  return M;
}
/** how many more men the floor can be given: the eight that cast shadows, as it was — or, the team the level's own, the skinned characters one frame draws
 *  (guards.ts MAX_SKINNED) less the civilians: the men past the eighth character have no shadow proxy (guards.ts spawnUnit's `budget`) */
function roomFor(g: Game): number { const n = (RESPONSE.on ? MAX_SKINNED - g.civilians.length : CHAR_BUDGET) - seatCount(g) - g.guards.length; return raid(g) ? Math.min(n, SQUAD_MAX_MEN - g.nightBreach.hostiles.length - g.qrf.men.length) : n; }   // (a raid: never more than the eleven a squad ever faces, the garrison as staged and the team together)
const budgetOf = (g: Game) => RESPONSE.on ? { budget: MAX_SKINNED - g.civilians.length } : {};
/** the most men this level's response may ever have on the floor, element and outside posts together (the level's `men`; the sandbox's two numbers without) */
function mostMen(g: Game): number { const R = RESPONSE.on ? responseOf(g) : null; return R?.men ? R.men[1] : 8; }

// ================================================================ numbers
/** the renderer's character budget this counts against (guards.ts spawnUnit's MAX_CHARACTERS = scene/boxes.ts BoxWorld's eight shadow-capsule characters, Sam
 *  included): the element is capped at what is left of it, the perimeter tier spawns only into what is left after that and otherwise re-tasks element men */
const CHAR_BUDGET = 8;
/** the element on the move: pace (m/s; the staff jog to a noise at 1.75, chase at 2.7), the pace when the cue is fresh gunfire, spacing down the file (m), how far
 *  either side of the trail two and three walk when the floor there is open, seconds between station updates */
const MOVE_SPEED = 2.15;
/** the fight: contact older than this (nobody's eyes on him, no report of his) and the element re-forms; awareness the scripted men are held at (hunting, not alarmed anew every frame) */
const HUNT_AWARENESS = 0.7;
/** outside: the jog to the post, the light's sweep period across the door (s) */
const POST_SPEED = 2.6;

export type QrfState = 'idle' | 'requested' | 'inbound' | 'arriving' | 'deployed';
export type QrfReason = 'gunfire' | 'bodies' | 'panel' | 'silent';
type Phase = 'landing' | 'file' | 'fan' | 'move' | 'stack' | 'breach' | 'bang' | 'flow' | 'drill' | 'called' | 'hold' | 'fight' | 'regroup';
interface Target { kind: 'room' | 'point'; plan: RoomClearPlan | null; room: string; pos: Vec3; /** RESPONSE: the word (its time) this point was chosen on */ word?: number; }
/** the perimeter tier: called (control's line, men picked and moving) → set (posts taken, or the ETA and ten seconds gone) */
interface Perimeter { state: 'called' | 'set'; calledAt: number; etaAt: number; setAt: number; men: { gd: Guard; post: ResponsePost; arrivedAt: number; fighting: boolean }[]; spawned: number; peeled: number; }

/** Game.qrf: the whole response's state (see the module comment). Everything but `state` is bookkeeping the probes and the panel read. */
export class Qrf {
  state: QrfState = 'idle';
  reason: QrfReason | null = null;
  /** game times: the request, when the door goes (request + ETA), when the first man came through; −1 = not yet */
  requestedAt = -1; arriveAt = -1; deployedAt = -1;
  answered = false; said30 = false; said10 = false; refusedAt = -100; everyoneSaid = false;
  /** the element in order (#1 point … #4 rear); dead men stay listed (the roles shift past them); `perim` = the outside tier's record once called */
  men: Guard[] = [];
  perim: Perimeter | null = null;
  /** stun charges left in the element's pool (one per man who came up the stairs; whoever is at the door puts it in) */
  bangs = 0;
  /** the score's inputs: QRF men down (any cause), and whether one of theirs fired the frame Sam went down (a round from the response is what ended the run) */
  lost = 0; gotSam = false; samWasDown = false;
  /** the trigger's memory: when a living staff man last had a report of SAM's at gunfire level in his ears (the hearing model's 80 m gate), and the level-1
   *  reports of anybody's since deployment (the perimeter tier's 'running gunfight') */
  loudT = -1e9; /** a raid: when a man who can call first heard gunfire (−1e9: not yet) */ raidT = -1e9; /** …and when each such man first heard it: the call is his to make `callSecs` after that */ heardAt = new Map<Guard, number>(); shotsSince: number[] = []; heardEvents = new SideSet<object>();
  /** the silent net (tune.qrfSilent — see `silentNet`): when the whole shift was first off the net with Sam's gunfire in its memory (−1 = it is not), whether
   *  control's radio check has been said, and whether the request it led to was made or refused (once a night either way) */
  silentAt = -1; silentSaid = false; silentSpent = false;
  /** what happened when, in order (probes / the panel's line): request, control, thirty, ten, door, in:N, target:…, stack, bang, flow, clear:…, contact, regroup, man-down:…, perimeter:… */
  log: { t: number; what: string }[] = [];
  // ---- the element's script
  phase: Phase = 'landing'; phaseT = 0;
  target: Target | null = null; targetAt = -100; fix: Vec3 | null = null; fixT = -100; cueRush = false;
  cleared = new Map<string, number>();
  // ---- the script's plumbing (module-internal; public only so the plain functions below can reach it)
  wps = new Map<Guard, Vec3[]>();
  trail: Vec3[] = []; stationT = 0; pointId = -1; moveSpeed = MOVE_SPEED;   // pointId: char id of the man walking the target leg (a new #1 after a death takes it up afresh)
  /** DOORSTAY: the stair door has been pulled to behind the element (or there was nothing to pull) */
  doorPulled = false;
  filed = 0; flowed = 0; can: Item | null = null; canLandT = NaN; stackMarks: Vec3[] = []; shovedAt = NaN;
  seenIds = new Set<number>();   // element / perimeter men already reported down
  aimPts = new Map<Guard, Vec3>(); stations = new Map<Guard, Vec3>();
  /** tune.qrfSquad: the element as a second cast of the squad's orders and door tools (built at its first door) · the entry it is on (phase 'drill') · how
   *  many of that entry's beats are in `log` · rooms begun this way, and taken */
  cast: SquadCast | null = null; op: DoorOp | null = null; opBeat = 0; drills = 0; drillsIn = 0;
  /** tune.qrfSquad: the room the squad's drill last took (or gave up on) and the fix they went there on — that fix is used up there (pickTarget) */
  spent: { room: string; fixT: number } | null = null;
  /** RESPONSE, a floor with no room plans: the word (its `fixT`) they have already stood on the place of — the level's named places come next (pickTarget) */
  sweptT = -1e9;
  /** RESPONSE: what this run's call was dealt — which of the level's ways (an index into its block), how many men (−1: the sandbox's number) — and the
   *  marks read off it (marksOf's own cache) */
  way = 0; size = -1; marks: Marks | null = null; marksOn = false; marksWay = -1;
  /** RESPONSE.net: THE NET'S WORD — per enemy, the freshest place said on the defenders' net (a man on it held the idea long enough to have said it, or was
   *  told it), kept by control past the man who said it; `wordT` = when it was last read */
  word = new Map<Guard | 'player', { at: Vec3; t: number; how: string }>(); wordT = -1e9;
  /** does this man belong to the response (element or perimeter)? — the QA soak's script invariant asks */
  owns(gd: Guard): boolean { return this.men.includes(gd) || !!this.perim?.men.some(m => m.gd === gd); }
  /** one line for the panel / probes */
  summary(g: Game): string {
    if (this.state === 'idle') return `QRF ▸ idle${this.loudT > -1e8 ? ` · gunfire heard ${Math.round(g.time - this.loudT)} s ago` : ''} · bodies found ${bodiesFound(g)} · alarm ${escalationOf(g)}`;
    const left = this.arriveAt > g.time ? ` · door in ${Math.ceil(this.arriveAt - g.time)} s` : '';
    const alive = this.men.filter(m => m.state !== 'dead' && g.guards.includes(m)).length;
    return `QRF ▸ ${this.state} (${this.reason})${left}${this.state === 'deployed' ? ` · ${alive}/${this.men.length} up · ${this.phase}${this.target ? ' → ' + this.target.room : ''} · bangs ${this.bangs} · lost ${this.lost}` : ''}${this.perim ? ` · PERIMETER ${this.perim.state} (${this.perim.men.length} out: ${this.perim.spawned} spawned, ${this.perim.peeled} peeled)` : ''}`;
  }
}

// ================================================================ the address
/** `?qrfsquad=0` on the page's address: the element's own drill for this visit */
export function qrfAddressWord(g: Game) { if (ownDrillAsked()) g.tune.qrfSquad = false; }
function ownDrillAsked(): boolean { try { const q = (globalThis as { location?: { search?: string } }).location?.search; return !!q && /[?&]qrfsquad=(0|off|false)\b/i.test(q); } catch { return false; } }
// ================================================================ small shared helpers
const flat = (a: Vec3, b: Vec3) => Math.hypot(a[0] - b[0], a[2] - b[2]);
function note(Q: Qrf, g: Game, what: string) { Q.log.push({ t: g.time, what }); if (Q.log.length > 200) Q.log.splice(0, 50); }
/** control's voice: a line on the net with nobody in the building saying it (the HUD draws 'radio: …' lines — ui/overlay.ts) */
function control(g: Game, text: string) { g.msg('radio: CONTROL — ' + text); g.audio.play('radio', null, 0.55); }
/** … and with a row in the run's timeline (the silent net's two lines: with nobody of the shift left to say why the team came, the debrief would not know) */
function controlRow(g: Game, text: string) { control(g, text); chronicle(g, 'radio', { who: 'CONTROL', text: text.length > 96 ? text.slice(0, 94) + '…' : text }); }
/** a response man's radio line: his bubble if he is on screen, and the net's log line under his callsign (guards.ts say would file a unit's radio under his name
 *  without the 'radio:' the HUD keys on — these men ARE on the shift's net) */
function radio(g: Game, gd: Guard | null | undefined, text: string) {
  if (!gd || gd.state === 'dead') { control(g, text); return; }
  gd.bubble = { text, t: g.time, dur: bubbleSecs(text), radio: true }; radioHand(gd, gd.bubble.dur);   // (handwork.ts: his hand to the handset while it is on the net — picture only)
  g.msg(`radio: ${gd.callsign} — ${text}`); g.audio.play('radio', gd.char.bones.head ?? gd.char.pos, 0.7);
  chronRadio(g, gd, text);   // (the run's timeline keeps what went over the net — chronicle.ts, as guards.ts say does for the shift)
}
/** the shift's man best placed to call it in: living, not in Sam's arm, staff (not a unit), calm for choice */
function staffVoice(g: Game): Guard | null {
  let best: Guard | null = null, br = -1;
  for (const x of g.guards) { if (x.unit || x.team !== 'guard' || x.state === 'dead' || x.held) continue; const r = x.state === 'alert' ? 0 : x.state === 'patrol' ? 2 : 1; if (r > br) { br = r; best = x; } }
  return best;
}
function bodiesFound(g: Game): number { let n = 0; for (const x of g.guards) if (!x.unit && x.state === 'dead' && x.found) n++; return n; }
/** the run is Sam's to answer for (not the tour, not the breach's daylight, somebody home in the guards' heads, Sam on his feet) */
function live(g: Game): boolean { if (raid(g)) return g.aiEnabled && !g.quietUtility && !g.touring && !allDown(g) && hasResponse(g);   // (a raid: day or night, the squad's run itself)
  return g.aiEnabled && !g.quietUtility && !g.touring && !isDaylight(g) && !allDown(g) && !g.nightBreach?.active && !g.rescue?.active && hasResponse(g); }
/** THIS LEVEL HAS A RESPONSE TEAM: the marks above are the office's stairwell. A level that carries its own places (a quiet mission: its file's scenarios.quiet
 *  — scene/scenarios.ts) has the team only if it has the team's door (a floor built round the office's west stair, landing and all); without it there is nobody
 *  to send — no request, no radio promise, no men stood on another building's coordinates. Every other level: as it always was. */
export function hasResponse(g: Game): boolean {
  if (RESPONSE.on) { const R = responseOf(g); return !!R && (!R.raid || raid(g)) && R.ways.some(y => !y.door || !!g.doors.byName(y.door)); }   // (the level's own: no block, no team — whatever doors it has)
  return !g.level.scenarios?.quiet || !!g.doors.byName(QRF_MARKS.door);
}
/** … and, for the panel's buttons: is there a way in to stand a team on (the switch off: the stair door — the gate above passes any level without a quiet block) */
export function responseHere(g: Game): boolean { return RESPONSE.on ? hasResponse(g) : !!g.doors.byName(QRF_MARKS.door); }
function alive(Q: Qrf, g: Game): Guard[] { return Q.men.filter(m => m.state !== 'dead' && g.guards.includes(m)); }
/** the net's word in a save, and back (savestate.ts: a checkpoint is only ever taken with the response idle — the word is what an idle one holds): who by
 *  roster index (−1 = Sam), the clock as an age */
export interface ResponseWordSnap { who: number; at: Vec3; age: number; how: string }
export function responseWordSnapshot(g: Game): ResponseWordSnap[] { const out: ResponseWordSnap[] = []; for (const [who, W] of g.qrf?.word ?? []) { const i = who === 'player' ? -1 : g.guards.indexOf(who); if (who === 'player' || i >= 0) out.push({ who: i, at: v3.copy(W.at), age: g.time - W.t, how: W.how }); } return out; }
export function responseWordRestore(g: Game, rows: ResponseWordSnap[] | undefined) {
  const Q = g.qrf; if (!Q) return; Q.word.clear(); Q.wordT = -1e9;
  for (const r of rows ?? []) { const who = r.who < 0 ? 'player' as const : g.guards[r.who]; if (who && Array.isArray(r.at) && Number.isFinite(r.age)) Q.word.set(who, { at: [r.at[0], 0, r.at[2]], t: g.time - r.age, how: String(r.how) }); }
}

// ================================================================ the trigger and the telegraph
export function resetQrf(g: Game) { { const __s = RUST_SURFACE; if (__s) { const __r = __s.call('resetQrf', undefined, [g]); if (__r !== RUST_PASS) return __r as any; } }
  const Q = g.qrf; if (!Q) return;
  dropDrill(Q);
  for (const m of [...Q.men, ...(Q.perim?.men.map(p => p.gd) ?? [])]) if (m.script && g.guards.includes(m)) endGuardScript(m, 'patrol');   // (resetGuards despawns them next; a bare resetQrf from the panel leaves ordinary alert-able men, then despawns them itself — qrfStandDown)
  const fresh = new Qrf(); Object.assign(Q, fresh);   // (keep the object main.ts / the panel hold; every field back to a fresh response's)
}
/** Panel: take the whole response back out of the world now — men despawned, state idle (a restart does the same through resetGuards). */
export function qrfStandDown(g: Game) { { const __s = RUST_SURFACE; if (__s) { const __r = __s.call('qrfStandDown', undefined, [g]); if (__r !== RUST_PASS) return __r as any; } } const Q = g.qrf; const men = [...Q.men, ...(Q.perim?.men.map(p => p.gd) ?? [])]; resetQrf(g); for (const m of men) if (g.guards.includes(m)) despawnUnit(g, m); g.note('QRF stood down (sandbox)'); }

/** Ask for the response team. From the trigger below (reason 'gunfire' / 'bodies') or the panel ('panel': no conditions but a live run). Refused — on the radio,
 *  at most every 20 s — once one is already on its way or has been: there is one wave a night. Returns whether it was requested now. */
export function requestQrf(g: Game, reason: QrfReason): boolean { { const __s = RUST_SURFACE; if (__s) { const __r = __s.call('requestQrf', undefined, [g, reason]); if (__r !== RUST_PASS) return __r as any; } }
  const Q = g.qrf, M0 = marksOf(g);
  if (Q.state !== 'idle') {
    if (g.time - Q.refusedAt > 20) { Q.refusedAt = g.time;
      control(g, !RESPONSE.on || !M0 ? (Q.state === 'deployed' ? "negative, two-six — the response team is already on your floor. that's everyone we've got tonight" : wg("negative — QRF is already rolling to you. one team, one elevator. hold what you've got", 'negative — QRF is already rolling to you. one team, one lift; hold what you have'))
        : Q.state === 'deployed' ? `negative, ${M0.site} — ${M0.team} is already with you. that's everyone we've got tonight` : wg(`negative — ${M0.team} is already on its way to you. there's no second team. hold what you've got`, `negative — ${M0.team} is already on its way to you. there is no second one; hold what you have`));
      note(Q, g, 'refused'); }
    return false;
  }
  if (!g.tune.qrf && reason !== 'panel') return false;
  if (!live(g) && reason !== 'panel') return false;
  if (!hasResponse(g)) { note(Q, g, RESPONSE.on ? 'no-team:this level has none' : 'no-team:this level has no stairwell for one'); return false; }
  if (roomFor(g) < 2) { note(Q, g, `no-room:${roomFor(g)}`); return false; }   // (the renderer's character budget has no room for even a pair: nothing is promised on the radio that cannot come — and nothing is dealt)
  const D = dealt(g, reason);   // (the renderer's character budget has no room for even a pair: nothing is promised on the radio that cannot come)
  Q.way = D.way; Q.size = RESPONSE.on ? D.men : -1; if (RESPONSE.on) note(Q, g, `dealt:way ${D.way}:men ${D.men}:eta ${D.eta}`);
  const M = marksOf(g)!;
  Q.state = 'requested'; Q.reason = reason; Q.requestedAt = g.time; Q.arriveAt = g.time + Math.max(12, D.eta); Q.answered = Q.said30 = Q.said10 = false;
  const rd = raid(g), sp = rd ? raidVoice(g) ?? callers(g)[0] ?? null : staffVoice(g); const n = bodiesFound(g);
  if (!RESPONSE.on) {
    const line = reason === 'bodies' ? `control, ${sp?.callsign ?? 'two-six'} — ${n >= 2 ? `${n} men down` : 'men down'} on two-six and whoever did it is still up here. we need the response team` : reason === 'gunfire' ? `control, ${sp?.callsign ?? 'two-six'} — shots fired on two-six${w('', ', automatic fire')}. this is past us — get the response team up here` : `control, ${sp?.callsign ?? 'two-six'} — send the response team up to two-six. now`;
    if (sp) radio(g, sp, line); else if (reason === 'silent') controlRow(g, 'all stations, control — shots reported on two-six and nobody up there is answering. response team, go'); else control(g, 'two-six has gone quiet with an alarm up — response team, go');
  } else {
    const who = sp?.callsign ?? M.site;
    const line = reason === 'bodies' ? `control, ${who} — ${n >= 2 ? `${n} men down` : 'men down'} ${M.at} and whoever did it is still here. we need ${M.team}` : reason === 'gunfire' ? (rd ? `control, ${who} — we are being hit ${M.at}. send ${M.team}` : `control, ${who} — shots fired ${M.at}. this is past us — send ${M.team}`) : `control, ${who} — send ${M.team} to ${M.site}. now`;
    if (sp) radio(g, sp, line); else if (reason === 'silent') controlRow(g, `all stations, control — shots reported ${M.at} and nobody there is answering. ${M.team}, go`); else control(g, `${M.site} has gone quiet with an alarm up — ${M.team}, go`);
  }
  note(Q, g, `request:${reason}`);
  return true;
}
/** staging: no enemy of theirs (Sam, a living operative) stands within `clearM` of this mark — nobody is stood up at his elbow */
function clearOfFoes(g: Game, p: Vec3): boolean {
  if (flat(g.player.char.pos, p) < RESPONSE.clearM) return false;
  for (const x of g.guards) if (x.team === 'operative' && x.state !== 'dead' && flat(x.char.pos, p) < RESPONSE.clearM) return false;
  return true;
}
/** WHAT THIS CALL GETS: the way in, how many, how long. The switch off: the sandbox's two numbers and the one stairwell, as it was. On: the level's block
 *  — its first answer (the first way, its most men, the middle of its time; a block that gives no range: the sandbox's number), or with RESPONSE.deal one of
 *  each off the game's own die. A way an enemy stands at the muster of is never dealt (staging, not knowledge: nobody is stood up at his elbow). A call from
 *  the panel is the sandbox's own numbers on the first way: the rows there say what comes. */
function dealt(g: Game, reason: QrfReason): { way: number; men: number; eta: number } {
  const T = g.tune, R = RESPONSE.on ? responseOf(g) : null, sandbox = clamp(Math.round(T.qrfSize), 2, 4);
  if (!R || reason === 'panel') return { way: 0, men: sandbox, eta: T.qrfEta };
  const men: [number, number] = R.men ?? [sandbox, sandbox], eta: [number, number] = R.eta ?? [T.qrfEta, T.qrfEta];
  const open: number[] = []; R.ways.forEach((y, i) => { if (!y.door || g.doors.byName(y.door)) open.push(i); });
  if (!RESPONSE.deal) return { way: open[0] ?? 0, men: men[1], eta: (eta[0] + eta[1]) / 2 };
  const a = simRandom(), b = simRandom(), c = simRandom();   // (three draws whatever the level allows: a recording replays them in step)
  const clear = open.filter(i => clearOfFoes(g, R.ways[i].muster[0])), from = clear.length ? clear : open;
  const step = RESPONSE.etaStep, span = men[1] - men[0] + 1;
  return { way: from[Math.floor(a * from.length) % from.length] ?? 0, men: men[0] + Math.floor(b * span) % span, eta: clamp(Math.round((eta[0] + c * (eta[1] - eta[0])) / step) * step, eta[0], eta[1]) };
}








const sectorPoint = (p: Vec3, yaw: number, d = 5, y = 1.0): Vec3 => [p[0] + Math.sin(yaw) * d, y, p[2] + Math.cos(yaw) * d];

/** THE LEVEL'S BLOCK AGAINST ITS BUILT FLOOR (tools/qa/response-check.ts runs it on every level file that has one): a way's door is a door of the level;
 *  its four muster marks and four fan marks stand on free floor a body's width apart; each man's walk from his muster mark to his fan mark is a short one,
 *  and from the fan the floor the level's men walk can be reached; every outside post, and the marks its men come on at, likewise; a quiet mission's every
 *  start has a way whose muster is not at its elbow. Returns what is wrong, in words (nothing: the block stands). */
export function checkResponse(g: Game): string[] {
  const R = responseOf(g), out: string[] = []; if (!R) return out;
  const nav = g.col.nav, at = (p: readonly number[]) => `(${p[0]}, ${p[2]})`, free = (p: readonly number[]) => !nav.isBlocked(p[0], p[2]);
  const walk = (a: readonly number[], b: readonly number[]) => pathLen(g, [a[0], 0, a[2]], [b[0], 0, b[2]]);
  const apart = (ps: readonly Vec3[], what: string) => { for (let i = 0; i < ps.length; i++) for (let j = i + 1; j < ps.length; j++) if (flat(ps[i], ps[j]) < 0.6) out.push(`${what}: marks ${i + 1} and ${j + 1} are ${flat(ps[i], ps[j]).toFixed(2)} m apart`); };
  const floor = g.level.routes[0]?.points[0] ?? g.level.playerSpawn;
  R.ways.forEach((y, k) => {
    const W = `way ${k + 1} '${y.name}'`;
    if (y.door && !g.doors.byName(y.door)) out.push(`${W}: no door '${y.door}' on this level`);
    y.muster.forEach((p, i) => { if (!free(p)) out.push(`${W}: muster mark ${i + 1} ${at(p)} is not free floor`); });
    y.entry.forEach((p, i) => { if (!free(p)) out.push(`${W}: fan mark ${i + 1} ${at(p)} is not free floor`); });
    apart(y.muster, `${W} muster`); apart(y.entry, `${W} fan`);
    y.muster.forEach((p, i) => { const e = y.entry[i]; if (!e || !free(p) || !free(e)) return; const L = walk(p, e); if (!(L <= 30)) out.push(`${W}: man ${i + 1}'s walk from muster to his fan mark is ${L === Infinity ? 'no walk at all' : L.toFixed(1) + ' m'} (30 m at most)`); });
    if (y.entry[0] && free(y.entry[0]) && walk(y.entry[0], floor) === Infinity) out.push(`${W}: no walk from the fan ${at(y.entry[0])} to the level's floor ${at(floor)}`);
    if (y.door) { const d = g.doors.byName(y.door); if (d && y.muster[0] && y.entry[0] && g.doors.pullFace(d, y.muster[0]) === g.doors.pullFace(d, y.entry[0])) out.push(`${W}: the muster and the fan are on the same side of '${y.door}'`); }
  });
  for (const s of g.level.scenarios?.quiet?.starts ?? []) if (!R.ways.some(y => flat(s.sam, y.muster[0]) >= RESPONSE.clearM)) out.push(`start '${s.name ?? at(s.sam)}': every way's muster is within ${RESPONSE.clearM} m of it`);
  (R.posts ?? []).forEach((p, i) => { if (!free(p.pos)) out.push(`post ${i + 1} '${p.name}' ${at(p.pos)} is not free floor`); else if (!(R.outside ?? []).some(o => free(o) && walk(o, p.pos) < Infinity)) out.push(`post ${i + 1} '${p.name}': no walk to it from any outside mark`); });
  (R.outside ?? []).forEach((p, i) => { if (!free(p)) out.push(`outside mark ${i + 1} ${at(p)} is not free floor`); });
  (R.sweep ?? []).forEach((p, i) => { if (!free(p.pos)) out.push(`sweep place ${i + 1} '${p.name}' ${at(p.pos)} is not free floor`); else if (R.ways[0]?.entry[0] && walk(R.ways[0].entry[0], p.pos) === Infinity) out.push(`sweep place ${i + 1} '${p.name}': no walk to it from the first way in`); });
  return out;
}
function pathLen(g: Game, a: Vec3, b: Vec3): number { const p = g.col.nav.findPath(a, b); if (!p) return Infinity; let L = 0, q = a; for (const r of p) { L += flat(q, r); q = r; } return L; }

/** hold all round where they stand: each man squares off to a quarter (point ahead, two left, three right, four behind), guns up, for a beat */
function hold360(g: Game, Q: Qrf, el: Guard[]) {
  Q.phase = 'hold'; Q.phaseT = 0;
  const f = el[0].char.bodyYaw;
  el.forEach((m, k) => { const yaw = f + [0, -Math.PI / 2, Math.PI / 2, Math.PI][k % 4]; order(m, { goal: null, face: yaw, strafe: false, stance: 'none', upper: 'aim', crouch: k === 3, aimAt: sectorPoint(m.char.pos, yaw) }); });
  note(Q, g, 'hold');
}


/** the entry let go of (contact, a reset, its end): every man off the cast's orders where he stands, the tools emptied — a can they had not yet thrown back in the element's pool */
function dropDrill(Q: Qrf) {
  const C = Q.cast, op = Q.op; Q.op = null; if (!C) return;
  if (op && C.tools.ops.includes(op)) Q.bangs = C.tools.bangs + (op.bangHeld ? 1 : 0);   // let go of while it ran: the can in a hand comes back
  else if (op && 'ended' in op) Q.bangs = C.tools.bangs;   // ended by the tools: their count is the count (end() gave an unthrown can back)
  // (neither: the tools started afresh under it and their count is a new bag's — the element's own, as last read, stands)
  C.orders.clear(); C.tools.reset(); C.tools.charges = 0; C.tools.frags = 0;
}



/** Lock the compound down (the trigger above, or the panel): control's word, the exterior lamps up, and men to the outside posts — spawned by the gate if the
 *  character budget has room, else the element's rear men re-tasked (two stay inside whatever the knob says). One activation a night. */
export function callPerimeter(g: Game, why = 'panel'): boolean { { const __s = RUST_SURFACE; if (__s) { const __r = __s.call('callPerimeter', undefined, [g, why]); if (__r !== RUST_PASS) return __r as any; } }
  const Q = g.qrf; if (Q.perim) return false;
  if (Q.state !== 'deployed' && why !== 'panel') return false;
  if (!hasResponse(g)) return false;   // (the posts are the office's lot)
  const MK = marksOf(g)!; if (!MK.posts.length) return false;
  const want = Math.min(clamp(Math.round(g.tune.perimeterSize), 2, 4), MK.posts.length);
  const P: Perimeter = { state: 'called', calledAt: g.time, etaAt: g.time + Math.max(5, g.tune.perimeterEta), setAt: -1, men: [], spawned: 0, peeled: 0 };
  Q.perim = P;
  control(g, !RESPONSE.on ? `all stations — lock the compound down. nobody leaves the site: outside team, a man on every exit, ${Math.round(g.tune.perimeterEta)} seconds. exterior lights coming up. (${why})`
    : `all stations — lock ${MK.site} down, ${why}. nobody leaves: a man on every way out, ${Math.round(g.tune.perimeterEta)} seconds${g.targets.some(t => t.group === 'exterior') ? '. outside lights coming up' : ''}`);
  setGroup(g, 'exterior', true);   // the lot's sodium lamps, all of them (the dark one by the lobby too): a switch in the gatehouse — back as authored on a restart (resetLevelState)
  if (missionLive(g)) g.mission.stats.qrf = 2;
  // who mans it
  const outside = RESPONSE.on ? MK.outside.filter(p => clearOfFoes(g, p)) : MK.outside;   // (RESPONSE: a mark an enemy stands by stands nobody up — the rear men go out instead)
  const room = roomFor(g); const nSpawn = Math.min(want, Math.max(0, room), outside.length, RESPONSE.on ? Math.max(0, mostMen(g) - alive(Q, g).length) : want);   // (RESPONSE: never more of them on the floor than the level's most)
  for (let i = 0; i < nSpawn; i++) {
    try {
      const gd = spawnUnit(g, { team: 'guard', pos: outside[i] as Vec3, yaw: Math.PI, weapon: 'aks74u', name: `GATE-${i + 1}`, light: true, tint: [...QRF_TINT], tint2: [...QRF_TINT2], ...budgetOf(g) });
      gd.trained = true; gd.hold = false; gd.state = 'search'; gd.awareness = HUNT_AWARENESS; gd.heardUpTo = g.eventSeq;
      P.men.push({ gd, post: MK.posts[P.men.length], arrivedAt: -1, fighting: false }); P.spawned++;
    } catch (e) { note(Q, g, `perim-spawn-failed:${String((e as Error)?.message ?? e).slice(0, 40)}`); break; }
  }
  const el = alive(Q, g); const spare = Math.max(0, el.length - 2);
  const nPeel = Math.min(want - P.men.length, spare);
  const onDrill = nPeel > 0 && !!Q.op; if (onDrill) { dropDrill(Q); note(Q, g, 'drill-end:peel'); }   // (tune.qrfSquad: men leave the element under an entry — it is let go of whole, and who is left re-forms)
  for (let i = 0; i < nPeel; i++) {
    const gd = el[el.length - 1 - i];   // the rear men go out
    if (gd.script) endGuardScript(gd, 'search'); gd.state = 'search'; gd.standoff = null; gd.pieing = null; gd.searchPlan = null;
    P.men.push({ gd, post: MK.posts[P.men.length], arrivedAt: -1, fighting: false }); P.peeled++;
  }
  Q.men = Q.men.filter(m => !P.men.some(p => p.gd === m));   // (peeled men leave the element's order; they stay 'owned' through perim)
  for (const m of P.men.map(p => p.gd)) { Q.wps.delete(m); Q.stations.delete(m); }
  for (const p of P.men) toPost(g, p);
  if (onDrill) hold360(g, Q, alive(Q, g));
  const names = P.men.map(p => p.gd.callsign);
  if (P.men.length) radio(g, P.men[0].gd, `${names.join(', ')} — going outside. ${P.men.map(p => p.post.name).join(' and ')}, from the outside, lights on the doors`);
  else control(g, wg("there's no outside team — the lights are all you get. watch the exits from inside", 'no outside team to be had — the lamps are all you get. watch the exits from inside'));
  note(Q, g, `perimeter:${why}:${P.spawned}s+${P.peeled}p`);
  return true;
}
function toPost(g: Game, p: Perimeter['men'][number]) {
  const gd = p.gd; gd.pinned = false; gd.hold = false; p.fighting = false;
  order(gd, { goal: v3.copy(p.post.pos), speed: POST_SPEED, face: p.post.yaw, strafe: false, stance: 'none', upper: 'aim', crouch: false, aimAt: v3.copy(p.post.sweep[0]) });
}
