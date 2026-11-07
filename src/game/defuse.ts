// TWO CHARGES, ONE CLOCK: the overlay of a defuse mission (scene/scenarioList.ts rows on a level whose file carries a `scenarios.defuse` block — scene/scenarios.ts
// DefuseData: the charges, each a point on a thing in a room, and the seconds they share). Under it is a plain ROOM CLEAR on that level (roomclear.ts): the
// defenders, the rooms, the orders and the door tools are that scenario's own, untouched. What this file adds:
//
//   THE CLOCK     one for every charge. It starts on the team's first move (an order given, or Sam a stride off his start) — never while the squad stands
//                 formed up — and runs on the game clock. At zero EVERY charge still live goes off: frag.ts blast() at its place (the game's own blast: the
//                 room's men judged by distance and cover, the destruction's blast, the bang in everybody's ears), and the mission has failed.
//   MAKING SAFE   a hand on a charge for `workSecs`: Sam's own Space row at it ('make it safe', a sustained verb: a step, a sprint, another verb or Space /
//                 Esc lets go and the work starts over), or a teammate sent to it by the squad's 'work a thing' order (workthings.ts lists every live
//                 charge as a thing: X on it, 1 — the nearest man works it, his partner covers).
//   THE ENDS      FAILED = a charge went off — and it stays failed: the fight plays on, but the house taken afterwards is no win (roomclear.ts says so in
//                 place of its 'structure taken'). WON = every charge safe AND the room clear's own 'taken' (the house is the team's). The clock STOPS
//                 only when the run is over another way: Sam down, or the room clear's own 'wiped'. The squad down with Sam still up does not stop it —
//                 he has hands too. The row over the room clear's objective is this file's while a charge is live or one has gone off (the marker on the
//                 live charge nobody is at, the row saying who is on which); with all safe the row is the room clear's own again.
//
// LIVE = the switch on (`DEFUSE.on`) ∧ the level carries the block ∧ a room clear is staged on it ∧ no show. A level without the block: every entry point
// returns on its first line — the game as it was. No dice; the state is the clock's start, each charge's state and Sam's own work, with a snapshot / restore
// pair for a checkpoint (savestate.ts). Per-player state is asked through one accessor (who()), so a second player is one place to change.
//
// (src/game is one import cycle: nothing here is built from another game module's exports at module load.)
import type { Game, Guard, Interactable, Player } from './game';
import type { DefuseData, DefuseDevice } from '../scene/scenarios';
import type { Vec3 } from '../math/vec';
import './frag';
import './handwork';
import { RUST_PASS, RUST_SURFACE } from './crossing';

/** The knobs. `workSecs` / the blast's numbers stand in where the level's block names none. */
export const DEFUSE = {
  on: true,
  /** seconds of a hand on a charge to make it safe, where the level says nothing */
  workSecs: 6,
  /** Sam's own row: how near the charge he must stand, how far a marker is shown from, how far he may shift while at it, m */
  reach: 1.3, markerM: 5, shift: 0.45,
  /** the clock starts when Sam is this far from where the staging stood him (or on the first order), m */
  startM: 1.2,
  /** the blast where the level says nothing: frag.ts's two radii (certain inside the first for a man it has a line to, falling off to the second), and the
   *  destruction's reach and power */
  lethalM: 5, woundM: 9, radius: 3.5, power: 1,
  /** the last seconds, ticked aloud once a second FOR THE PLAYER WHEREVER HE IS (no place: a tick at the charge is not heard from the other wing; his ears
   *  only, no event) */
  warnSecs: 10,
};
export const DEFUSE_STATS = { started: 0, safe: 0, bySam: 0, blown: 0, won: 0, failed: 0 };

export type ChargeState = 'armed' | 'safe' | 'blown';
/** a checkpoint's slice: seconds run on the clock (-1: not started), each charge's state, who made each safe, the end, Sam's seconds at a charge (null: none) */
export interface DefuseSnap { ran: number; st: ChargeState[]; by?: (string | null)[]; end: 'safe' | 'failed' | null; sam: { dev: number; ran: number } | null }

/** the player the row is for (one seat today) */
const who = (g: Game): Player => g.player;
const mmss = (s: number) => { const t = Math.max(0, Math.ceil(s)); return `${Math.floor(t / 60)}:${String(t % 60).padStart(2, '0')}`; };
const n3 = (d: DefuseDevice): Vec3 => [d.n[0], 0, d.n[1]];

export class Defuse {
  /** each charge's state, in the block's order (empty until the first tick of a run) */
  states: ChargeState[] = [];
  /** who made each safe ('Sam' or a callsign) */
  safeBy: (string | null)[] = [];
  /** 'safe' = every charge made safe · 'failed' = one went off · 'over' = the run ended another way first (Sam down, or the room clear's own 'wiped': the
   *  clock stops where it stood and nothing more happens here — the row and the card are the room clear's; the squad down with Sam up is NOT that: the
   *  clock runs on) · null = still on the clock */
  end: 'safe' | 'failed' | 'over' | null = null;
  /** game time the clock started (NaN: the team has not moved yet) and stopped (NaN: running) */
  t0 = NaN; tEnd = NaN;
  log: { t: number; what: string }[] = [];
  private run: object | null = null;       // the Mission this pass belongs to: a fresh one (restartEncounter) starts it over
  private from: Vec3 | null = null;        // where Sam stood when the run came up
  private lastOrder: object | null = null; // the orders' last log row then (the log is trimmed from the front: by identity)
  private sam: { dev: number; t0: number; at: Vec3 } | null = null;
  private offered = -1;
  private marker: Interactable | null = null;
  private beat = -1;
  private wonSaid = false;
  private tookSaid = false;                // the house taken with a charge still live: said once
  constructor(private g: Game) {}

  /** the block in force (null: not live) */
  get data(): DefuseData | null {
    const g = this.g; if (!DEFUSE.on) return null;
    const D = g.level.scenarios?.defuse; if (!D?.devices?.length) return null;
    if (g.touring || g.quietUtility || g.puppet) return null;
    return g.roomClear?.active && g.roomClear.rooms.length ? D : null;
  }
  /** is this pass the running mission's (false between a restart and the first tick after it: the state is the last run's and nobody's) */
  private get mine(): boolean { return this.run === this.g.mission; }
  get active(): boolean { return !!this.data && this.mine; }
  get started(): boolean { return !isNaN(this.t0); }
  /** seconds a hand must stay on a charge */
  get workSecs(): number { return this.data?.workSecs ?? DEFUSE.workSecs; }
  /** seconds left on the clock (the whole of it until the team moves; where it stopped once it has) */
  get left(): number { const D = this.data; if (!D) return 0; return !this.started ? D.secs : D.secs - ((isNaN(this.tEnd) ? this.g.time : this.tEnd) - this.t0); }
  /** has the mission been won: every charge safe and the structure taken */
  get won(): boolean { return this.active && this.end === 'safe' && this.g.roomClear.end === 'taken'; }

  // ------------------------------------------------------------ what the squad's work order reads (workthings.ts)
  /** changes when the list of live charges does */
  get key(): string { return this.active ? this.states.join(',') : ''; }
  /** the charges still live: index, the point a hand goes to, the planar normal toward the man, the menu's name */
  armed(): { i: number; face: Vec3; n: Vec3; name: string }[] {
    const D = this.data; if (!D || !this.mine || this.end) return [];
    return D.devices.flatMap((d, i) => this.states[i] === 'armed' ? [{ i, face: [d.pos[0], d.pos[1], d.pos[2]] as Vec3, n: n3(d), name: d.name }] : []);
  }
  /** why charge i is no longer there to be worked (null: it is) — the word the man says */
  moot(i: number): string | null { const s = this.active ? this.states[i] : undefined; return s === 'armed' ? null : s === 'safe' ? "it's already safe." : "it's gone."; }


  /** a hand has been on charge i long enough (Sam's own row here; a teammate's work order — workthings.ts) */
  makeSafe(i: number, by: Guard | 'Player') {
    const D = this.data; if (!D || !this.mine || this.states[i] !== 'armed' || this.end) return;
    const g = this.g, d = D.devices[i], name = by === 'Player' ? 'Player' : by.callsign;
    this.states[i] = 'safe'; this.safeBy[i] = name; DEFUSE_STATS.safe++; if (by === 'Player') DEFUSE_STATS.bySam++;
    g.audio.play('click', d.pos, 0.9); this.note(`safe: ${d.id} by ${name} with ${this.left.toFixed(1)} s left`);
    const live = this.states.filter(s => s === 'armed').length;
    if (live > 0) { g.msg(`${d.name} is safe, with ${live === 1 ? 'one' : live} to go and ${mmss(this.left)} left`); return; }
    this.end = 'safe'; this.tEnd = g.time; this.sam = null;
    g.msg(g.roomClear.end === 'taken' ? `${d.name} is safe. every charge is safe and the house is yours` : `${d.name} is safe. every charge is safe with ${mmss(this.left)} left. now take the house`);
    this.note('end: safe');
  }


  /** is Sam at a charge right now — a sustained verb (interact.ts runningVerb / cancelSustained, savestate.ts playerBusy, player.ts's running row) */
  get working(): boolean { return !!this.sam && this.active; }
  /** handwork.ts samWork: the charge's face he is planted at and squared to while he works it (`at` = the spot he must not shift from, walked in with him) */
  get samJob(): { p: Vec3; n: Vec3; at: Vec3 } | null { const s = this.sam, D = this.data; if (!s || !D || !this.mine) return null; const d = D.devices[s.dev]; return { p: [d.pos[0], d.pos[1], d.pos[2]], n: n3(d), at: s.at }; }
  /** Space again, Esc or a step: let go, the work starts over (true: there was one to let go of) */
  cancel(): boolean { if (!this.sam) return false; this.sam = null; this.g.msg('you let go, so the charge is still live'); return true; }

  // ------------------------------------------------------------ the HUD's row
  /** who is at charge i or on his way to it by an order, as the row says it (null: nobody) */
  onIt(i: number): string | null {
    if (this.sam?.dev === i) return 'you on it';
    const w = this.g.work?.onDevice(i); return w ? `${w.gd.callsign} ${w.on ? 'on it' : 'going'}` : null;
  }

  // ------------------------------------------------------------ checkpoints
  snapshot(): DefuseSnap | undefined {
    if (!this.data || !this.mine) return undefined;
    const g = this.g;
    return { ran: this.started ? (isNaN(this.tEnd) ? g.time : this.tEnd) - this.t0 : -1, st: [...this.states], by: [...this.safeBy], end: this.end === 'over' ? null : this.end, sam: this.sam ? { dev: this.sam.dev, ran: g.time - this.sam.t0 } : null };
  }
  /** a checkpoint loaded (savestate.ts, after the mission slice). A file without the slice on a live level: a fresh clock, as on a restart */
  restore(s: DefuseSnap | undefined) { { const __s = RUST_SURFACE; if (__s) { const __r = __s.call('Defuse.restore', this, [s]); if (__r !== RUST_PASS) return __r as any; } }
    const D = this.data; this.run = null; this.sam = null; if (!D || !s) return;
    const g = this.g, p = who(g).char.pos, ok = (v: unknown): v is ChargeState => v === 'armed' || v === 'safe' || v === 'blown';
    if (!Array.isArray(s.st) || s.st.length !== D.devices.length || !s.st.every(ok) || !Number.isFinite(s.ran)) return;
    // (the end is what the charges say, whatever the file's own word: one blown = failed, all safe = safe, a live one = running)
    const by = Array.isArray(s.by) ? s.by : [];
    this.run = g.mission; this.states = [...s.st]; this.safeBy = s.st.map((v, i) => v === 'safe' && typeof by[i] === 'string' ? (by[i] === 'Sam' ? 'Player' : by[i]) : null);   // (a file from before the player lost his name)
    this.end = s.st.includes('blown') ? 'failed' : s.st.every(v => v === 'safe') ? 'safe' : null;
    this.t0 = s.ran >= 0 ? g.time - s.ran : NaN; this.tEnd = this.end && s.ran >= 0 ? g.time : NaN; this.beat = -1; this.offered = -1; this.wonSaid = false; this.tookSaid = g.roomClear.end === 'taken';
    this.from = [p[0], 0, p[2]]; const lg = g.orders?.log ?? []; this.lastOrder = lg[lg.length - 1] ?? null;
    if (s.sam && Number.isInteger(s.sam.dev) && this.states[s.sam.dev] === 'armed' && Number.isFinite(s.sam.ran)) this.sam = { dev: s.sam.dev, t0: g.time - s.sam.ran, at: [p[0], 0, p[2]] };
    this.note('checkpoint loaded');
  }

  private note(what: string) { this.log.push({ t: this.g.time, what }); if (this.log.length > 200) this.log.splice(0, 50); }
  summary(): string {
    const D = this.data; if (!D) return DEFUSE.on ? 'no charges on this level' : 'off';
    if (!this.mine) return 'staged';
    return `${this.end ?? (this.started ? mmss(this.left) : 'waiting on the first move')} · ${D.devices.map((d, i) => `${d.id} ${this.states[i]}${this.safeBy[i] ? ` (${this.safeBy[i]})` : ''}`).join(' · ')}`;
  }
}

