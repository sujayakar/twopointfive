// The run's chronicle, the debrief's memory: the timeline, Sam's path and the counters the stealth score is made of. ALL OF IT IS THE RUST GAME'S (rust/game_rules/src/chronicle):
// it writes the rows and the counts, words them, scores them, and the end card is told (src/view/debriefview.ts). Here: the record's shape as the bridge fills it on the page's copy
// (a lesson's condition reads its rows), the two tables the knobs carry, and the names of the writers: on the game's copy they write nothing.
import { SideMap, SideSet } from '../math/side';
import type { Vec3 } from '../math/vec';
import type { Game, Guard, LightTarget, DownKind } from './game';
import type { Door } from './doors';
import { RUST_PASS, RUST_SURFACE } from './crossing';

// ------------------------------------------------------------ the record
export type ChronKind =
  | 'objective'      // the mission thread moved (mission.ts): the server room reached, hands on the drive, the drive out, out through the fire exit
  | 'spotted'        // an alert man had eyes on Sam — a fresh sighting episode (mission.ts, the same edge that counts stats.spotted): by whom, the floor's alarm level then
  | 'contact'        // a man went to 'alert' (guards.ts updateGuard's transition): who, and what did it (sight / sound / radio / body)
  | 'heard'          // a noise of Sam's making reached a man and counted (guards.ts hearingCheck): what it was, who, what it did to him — merged per noise and per kind
  | 'lost'           // an alert man's chase ended without him: down to a search (guards.ts dropToSearch)
  | 'calm'           // a searching man gave it up (guards.ts, the search branch's end)
  | 'alarm'          // the floor's alarm level changed (guards.ts raiseEscalation / stepDownEscalation): stage, up or down, why
  | 'radio'          // a line went over the guards' net (guards.ts say with radio) — the routine radio check is not kept
  | 'shot'           // Sam fired (combat.ts fireWeapon): the weapon, what the round went into, whether a living man was within SHOT_NEAR_M of it — merged per burst
  | 'lightShot'      // a fixture shot out (combat.ts fireWeapon's light pass)
  | 'ocp'            // the OCP knocked something out (lighting.ts ocp): a fixture, a man's torch
  | 'blackout'       // the mains went (lighting.ts setBlackout): tripped by the OCP, shot, thrown by hand
  | 'thrown'         // a canister left his hand (player.ts, the pending throw's release): smoke / stun
  | 'grab'           // a man taken from behind into his arm (player.ts startGrab)
  | 'interrogated'   // one exchange with a man (game.ts interrogate): who, and the useful thing he gave, if any
  | 'keyTaken'       // a key came off a questioned man's belt (player.ts takeKey)
  | 'ko' | 'kill'    // a man put down by Sam (combat.ts killGuard, by 'player'): out cold (takedown / choke / whip / ring) or shot dead
  | 'bodyFound'      // a body discovered (guards.ts: walked onto, or watched going down): whose, by whom, breathing or not
  | 'doorKicked' | 'lockPicked'   // player.ts updateKick's impact / updatePicking's last click
  | 'hit' | 'down'   // a round found Sam (combat.ts fireWeapon → hitPlayer): still up / down
  | 'needBreak'      // a man left his route for a need / came back onto it / had the trip broken off (needs.ts): who, kind, what — the window the player had
  | 'civilianSpotted' | 'civilianReported' | 'civilianComplied' | 'civilianKO' | 'civilianKilled'   // the civilians (civilians.ts): one SAW Sam (or was grabbed) / told a guard or rang it in (or control rang after she got out) / told to stay down and did / put out cold / killed — by Sam's hand
  | 'cameraSpotted' | 'cameraAlarm' | 'laserTripped' | 'deviceDown' | 'codeLearned' | 'codeEntered'   // the devices (devices.ts): a camera called him in / put the floor up, a beam broken, a camera or an emitter shot dead and called in, a door code learned (told / overheard) / keyed in
  | 'checkpoint';    // a checkpoint written or loaded (the game's own row: `face_checkpoint`, `face_loaded`)
export type ChronData = { [k: string]: string | number | boolean | undefined };
export interface ChronEvent { t: number; kind: ChronKind; room: string; pos: Vec3; data: ChronData; }

/** the timeline holds this many entries; the chattier kinds (a noise heard, a burst fired, a radio line, a man losing / giving up, a man's coffee break, a checkpoint
 *  WRITTEN) stop being kept at SOFT so the rest of a very long run — its alarms, bodies, kills, loads — still lands; anything refused is counted in `dropped` (the card says so) */
export const MAX_EVENTS = 700, SOFT_EVENTS = 520;
/** the kinds that are chatter past the soft cap: the state bridge's copy of the game's own table (the walk names it, tools/blitz/schema.json); nothing here reads it */
const CHATTER: ReadonlySet<ChronKind> = new Set<ChronKind>(['heard', 'shot', 'radio', 'lost', 'calm', 'contact', 'needBreak']);
/** the counters that are plain numbers (chronicleN moves one by name) */
export type ChronCountKey = { [K in keyof ChronCounts]: ChronCounts[K] extends number ? K : never }[keyof ChronCounts];
/** the path: a point every PATH_SECS of the mission clock, at most PATH_CAP of them — when full the buffer is thinned to every other point and the interval doubles,
 *  so the polyline always spans the whole run (a two-hour run ends up at one point every 4 s) */
export const PATH_CAP = 4000, PATH_SECS = 0.5;
/** a noise of the same kind within this many seconds of the last one heard is the same timeline row (footsteps are a row, not forty); a round within SHOT_MERGE
 *  of the last into the same sort of thing is the same row (a magazine emptied into a wall is one line ×30) */
const SHOT_MERGE_SECS = 2.5;
/** a round that stops within this of a living man was fired 'near men' (the score's shots term; a hit is nearer still) */
export const SHOT_NEAR_M = 3;

/** What the score needs beyond MissionStats, counted as the entries are written (so the numbers stay right even when the timeline has stopped keeping chatter). */
export interface ChronCounts {
  seenUp: number;          // sightings that happened with the floor ALREADY up (heightened / lockdown): seen again after they knew is worse
  esc1: number; esc2: number;   // times the floor went up to heightened / to lockdown
  shotsNearMen: number;    // Sam's rounds into or within SHOT_NEAR_M of a living man
  lightsShot: number; ocps: number; blackouts: number;
  kicked: number; picked: number; intel: number; thrown: number; grabs: number; contacts: number; heard: number; radio: number;
  civSeen: number; civReported: number; civComplied: number; civKO: number; civKilled: number;   // the civilians (civilians.ts): saw him · told the men / rang it in · did as told · out cold · dead — the score's own three terms and the 'unseen by staff' badge
  devices: number;         // times security's devices called him in (a camera's 'movement in …' / a beam broken) — the score's small 'they know something' term; a device putting the floor UP is already esc1 / esc2
  hits: number; down: boolean; downBy: string;
  saves: number; loads: number;
}
const newCounts = (): ChronCounts => ({ seenUp: 0, esc1: 0, esc2: 0, shotsNearMen: 0, lightsShot: 0, ocps: 0, blackouts: 0, kicked: 0, picked: 0, intel: 0, thrown: 0, grabs: 0, contacts: 0, heard: 0, radio: 0, civSeen: 0, civReported: 0, civComplied: 0, civKO: 0, civKilled: 0, devices: 0, hits: 0, down: false, downBy: '', saves: 0, loads: 0 });

export class Chronicle {
  events: ChronEvent[] = [];
  /** entries refused by the caps (the card admits to it) */
  dropped = 0;
  counts: ChronCounts = newCounts();
  /** callsigns Sam has had at least one exchange with (the 'talkative' badge wants all of them) */
  questioned = new Set<string>();
  /** the path: x, z, t triples in a flat array, `pathN` of them; sampled every `pathEvery` s of the mission clock, next at `pathNext` */
  path = new Float32Array(PATH_CAP * 3); pathN = 0; pathEvery = PATH_SECS; pathNext = 0;
  /** which hearing-model event has already made a row (three men hearing one shot are one row with three names), and the row it made */
  private heardOf = new SideMap<object, ChronEvent>();

  /** hearing-model events that were COUNTED (counts.heard) but could not make a row (the caps): the next man hearing the same noise must not count it again */
  private heardNoRow = new SideSet<object>();

}

// ------------------------------------------------------------ writing: the Rust game's. A name a call surface crosses by still crosses
export function chronicle(g: Game, kind: ChronKind, data: ChronData = {}, pos?: Vec3): ChronEvent | null { { const __s = RUST_SURFACE; if (__s) { const __r = __s.call('chronicle', undefined, [g, kind, data, pos]); if (__r !== RUST_PASS) return __r as any; } } return null; }
export function chronicleN(_g: Game, _kind: ChronKind, _count: ChronCountKey, _data: ChronData = {}, _pos?: Vec3): ChronEvent | null { return null; }
export function chronObjective(_g: Game, _text: string, _pos?: Vec3): ChronEvent | null { return null; }
export function chronLost(_g: Game, _gd: Guard): ChronEvent | null { return null; }
export function chronAlarm(_g: Game, _stage: number, _up: boolean, _cause = ''): ChronEvent | null { return null; }
export function chronRadio(_g: Game, _gd: Guard, _text: string): ChronEvent | null { return null; }
export function chronShot(_g: Game, _weapon: string, _hitGuard: Guard | null, _hitP: Vec3): ChronEvent | null { return null; }
export function chronLightShot(_g: Game, _tg: LightTarget): ChronEvent | null { return null; }
export function chronBlackout(_g: Game, _cause: string): ChronEvent | null { return null; }
export function chronGrab(_g: Game, _gd: Guard): ChronEvent | null { return null; }
export function chronInterrogated(_g: Game, _gd: Guard, _kind: string | null, _stage: number): ChronEvent | null { return null; }
export function chronKey(_g: Game, _gd: Guard): ChronEvent | null { return null; }
export function chronDown(g: Game, gd: Guard, how: DownKind, line?: string): ChronEvent | null { { const __s = RUST_SURFACE; if (__s) { const __r = __s.call('chronDown', undefined, [g, gd, how, line]); if (__r !== RUST_PASS) return __r as any; } } return null; }
export function chronHit(g: Game, by?: string): ChronEvent | null { { const __s = RUST_SURFACE; if (__s) { const __r = __s.call('chronHit', undefined, [g, by]); if (__r !== RUST_PASS) return __r as any; } } return null; }
export function chronDoor(g: Game, kind: 'doorKicked' | 'lockPicked', d: Door): ChronEvent | null { { const __s = RUST_SURFACE; if (__s) { const __r = __s.call('chronDoor', undefined, [g, kind, d]); if (__r !== RUST_PASS) return __r as any; } } return null; }

// ------------------------------------------------------------ the tables the knobs carry
/** The stealth score's price list (points off a hundred; every term only ever takes away, so nothing a run does can raise its score — the probes hold it to that).
 *  In the Chaos Theory spirit: being SEEN is the expensive thing (more so once the floor already knows), an alarm and each step the floor goes up cost on their own,
 *  a body found costs, a kill costs ten times a knockout, rounds put near men and fixtures shot out cost a little, getting hit / going down / leaning on a
 *  checkpoint cost, and dawdling past par costs a point a minute. */
export const SCORE = {
  seen: 12, seenUp: 4,            // per sighting episode; + seenUp for one that happened with the floor already heightened / locked down
  alarm: 6,                       // per alarm episode (somebody went to alert while nobody was)
  heightened: 5, lockdown: 8,     // the floor going up a level
  body: 6,                        // per body found, dead or out cold
  kill: 10, ko: 1,                // per man: shot dead / put down breathing
  shotNear: 2, shotNearCap: 12,   // per round into or within SHOT_NEAR_M of a living man, capped
  light: 1, lightCap: 4,          // per fixture shot out, capped (the OCP is free: it is the quiet tool)
  hit: 4, down: 25, load: 3,      // a round in the vest; going down; each checkpoint load
  civReported: 15, civKO: 10, civKilled: 40,   // the civilians (civilians.ts): a report that reached the men (told to one, rung in, control's call after she got out); one put out cold — ten knockouts' worth of a guard's; one KILLED — four kills' worth (and with tune.civilianKillFails the run is over anyway); being merely SEEN by one costs nothing by itself: it is what she then does that costs
  device: 2, deviceCap: 8,        // per time a camera / a tripwire called him in (they know SOMETHING moved — being heard's worth), capped; the floor going up on a camera's word costs as any rise does
  parSecs: 20 * 60, perMin: 1, timeCap: 10,   // a point per whole minute over par, capped
};
/** the badges' two numbers: fixtures shot out for 'lights out', and the clock 'speedrun' beats (a straight run — lobby, corridor, the picked server door, the rack,
 *  out through the break room — is about two minutes flat out) */
export const BADGE = { lightsOutN: 3, speedrunSecs: 4 * 60 };
