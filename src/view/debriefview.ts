// WHAT THE END CARD SHOWS OF A RUN: the Rust game's answer (rust/game_rules/src/tick/view `debrief_view`), asked once as the card opens and read here by name. The chronicle's
// words and counts and the recap are all worked out there: the card lays them out.
//
// Nothing under src/view imports the game (tools/qa/view-check.ts).
import type { Vec3 } from '../math/vec';

/** Everything the card and the score read, in one flat record: the mission's tallies, the chronicle's counters, the roster's size */
export interface RunCounts {
  elapsed: number; stage: string; done: boolean; failed: boolean; sandbox: boolean; blackout: boolean;
  seen: number; seenUp: number; alarms: number; esc1: number; esc2: number; bodies: number;
  kills: number; kos: number; shots: number; shotsNearMen: number; lightsShot: number; ocps: number;
  kicked: number; picked: number; keys: number; intel: number; questioned: number; roster: number;
  hits: number; down: boolean; loads: number; thrown: number; grabs: number; dropped: number;
  devices: number;
  /** the civilians: times one saw Sam / reports that reached the men / told to stay down / knocked out / killed */
  civSeen: number; civReported: number; civComplied: number; civKo: number; civKilled: number;
}
export interface ScoreLine { label: string; pts: number; }
export interface Badge { id: string; name: string; on: boolean; hint: string; }
/** The score, the rating and the words. `scored`: the run is graded as a sneak (false on a row that leans loud: the rest is still worked out, and no reader shows it).
 *  `grade`: the rating as a line says it ('GHOST 94'), '' when not scored */
export interface Debrief { counts: RunCounts; score: number; lines: ScoreLine[]; rating: string; blurb: string; badges: Badge[]; summary: string; state: 'done' | 'down' | 'sofar'; scored: boolean; grade: string; }
/** One row of the timeline. `down`: the floor's alarm going down (such a row carries no pin) */
export interface DebriefRow { t: number; kind: string; room: string; pos: Vec3; text: string; down: boolean; }
/** a room of the floor map: `label` is what is drawn on it, `name` what the tooltip says */

/** round = a round that landed on him (or one whose line he stood in: a hostage shot by her captor) · blast = a fragmentation burst · other = the game names
 *  who, not how · unknown = neither */
export type RecapCause = 'round' | 'blast' | 'other' | 'unknown';
export interface RecapMan { id: number; name: string; side: string; kind: 'player' | 'man' | 'civ'; }
/** One of ours going down. Plan coordinates [x, z], metres; `gaze` in radians, 0 = +z (the aim yaw he had the sample before he fell). */
export interface RecapDeath {
  t: number; victim: number; name: string; side: string; player: boolean;
  at: [number, number]; gaze: number;
  cause: RecapCause;
  /** who: a character id, −1 = nobody known */ by: number; byName: string; bySide: string;
  /** where his feet were / the muzzle the round left (a round) or the blast's centre */ byAt?: [number, number] | null; from?: [number, number] | null;
  to: [number, number];
  /** the gun's row name as a chronicle writes it: word it with gearText where it is shown */ weapon: string;
  /** metres from `from` to the man, and degrees between his gaze and the way to it (0 = he was looking straight at it, 180 = dead behind him) */ dist: number; off: number;
  blind: boolean;
  /** gunfire, or a burst: a thing ears place (a suppressed round is not) */ loud: boolean;
}
/** kind: a round · blast = a fragmentation burst (it kills) · bang = a stun canister or a charge (it does not). `at` = the shooter's feet. */
export interface RecapShot { t: number; kind: 'round' | 'blast' | 'bang'; who: number; from: [number, number]; to?: [number, number] | null; at?: [number, number] | null; weapon?: string | null; loud?: boolean | null; }
/** men: MAN_STRIDE floats a man = id, x, z, aim yaw, bits (1 down · 2 roused: alert or searching · 4 crouched). doors: 2 a door = the leaf's direction (NaN = off its hinges). */
export interface RecapFrame { t: number; n: number; men: ArrayLike<number>; doors: ArrayLike<number>; }
export interface RecapDoor { hinge: [number, number]; width: number; }
/** A death with its own last seconds: frames from `t0` to `t1` (the death at `death.t`), the rounds and blasts in them, every one of ours lost in them. */
export interface RecapTake { death: RecapDeath; t0: number; t1: number; cast: RecapMan[]; doors: RecapDoor[]; frames: RecapFrame[]; shots: RecapShot[]; deaths: RecapDeath[]; }
export interface DeathRecap { takes: RecapTake[]; /** the take the card opens on: the last player to go down, else the last of ours */ main: number; }
export const MAN_STRIDE = 5;

export interface DebriefView { debrief: Debrief; rows: DebriefRow[]; /** rows the timeline's caps refused */ dropped: number; /** x, z, t a point */ path: ArrayLike<number>; /** nothing while anybody still plays, or nobody of ours went down */ recap?: DeathRecap | null; }
