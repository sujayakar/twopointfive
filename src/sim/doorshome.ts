// THE DOORS' STATE, AS COLUMNS — where every leaf's own numbers are kept (game/doors.ts Door reads and writes them through its accessors), in TypeScript
// and in the Rust module alike, and how they move between the two.
//
// A leaf's state is not fields on an object: it is a COLUMN A NUMBER, a door an index (the layout the Rust side steps: rust/sim/src/doors.rs Doors). The
// columns are the `doors` part of the game's block (sim/state.ts) from the moment the doors are made, and the doors' step runs over them with no copy in or
// out, in the module (doors.rs update, the row RUST.doors on) or in game/doors.ts. `Door.angle` is the same read either way, so everything that reads a
// door — the door tools, the men, a checkpoint file, the digest — reads the same values whichever side stepped.
//
// A truth is 1 or 0, a man is his id, a set of men (Door.passers) its bits as a number, a kind (DoorDrive.kind) its place in DRIVE_KINDS.
//
// (Imports sim.ts and state.ts alone: like the rest of src/sim, this is below the game.)
import { sim, f64, type SimExports } from './sim';
import { simRoom, simPart, P_DOORS, type SimState } from './state';

/** doors a level can have and still keep them in the module (rust/sim/src/doors.rs CAP; more: they stay in TypeScript, the row on or not) */
export const DOOR_CAP = 64;
/** THE COLUMNS, by the numbers rust/sim/src/doors.rs gives them (its constants are these names in capitals: tools/qa/rust-check.ts holds the two lists to each other) */
export const DC = { angle: 0, vel: 1, quietT: 2, latched: 3, lastWho: 4, sinceBash: 5, closing: 6, locked: 7, lockBroken: 8, noticed: 9, noticedT: 10, pick: 11, picking: 12, pickIdle: 13,
  pickClock: 14, pickN: 15, rattleT: 16, guardT: 17, passers: 18, dealt: 19, outward: 20, clearway: 21, pullT: 22, velSeen: 23, swept: 24, solidT: 25, pressT: 26, openT: 27, mirrored: 28,
  angleSeen: 29, off: 30, shot: 31, creakArmed: 32, posAngle: 33, posX: 34, posZ: 35, drive: 36, driveWho: 37, driveDir: 38, driveKind: 39, driveT: 40, driveFrom: 41, driveQuiet: 42,
  driveSide: 43, driveRate: 44, drivePx: 45, drivePz: 46, driveHit: 47 } as const;
export const DOOR_COLS = 48;
/** DoorDrive.kind as a column holds it (its place here) */
export const DRIVE_KINDS = ['boot', 'swing', 'body', 'hand'] as const;
/** DoorSound as an event of the module's names it (its place here: doors.ts DoorSound's own order) */
export const DOOR_SOUNDS = ['creak', 'bang', 'latch', 'bash', 'rattle', 'pick', 'unlock', 'kick', 'clang', 'thud'] as const;
/** DoorStrike.tier, the same way */
export const STRIKE_TIERS = ['open', 'slam', 'boot'] as const;

/** the doors' bit in the head of a game's block (sim/state.ts: one a family, never reused), and how long their part is */
const DOOR_BIT = 4;
simRoom(P_DOORS, DOOR_COLS * DOOR_CAP * 8);
/** THE COLUMNS OF ONE SET OF DOORS (a level's, or a lone door's): the `doors` part of its game's block `home` (never a view of it: the block moves), DOOR_CAP
 *  doors a column, for the game's life. Value K of door i is `d[base + K * stride + i]`; `cap` is how many doors there are. */
export class DoorStore {
  /** where the columns start in `home.d`, and doors a column */
  readonly base: number; readonly stride = DOOR_CAP;
  constructor(readonly home: SimState, readonly cap: number) {
    if (cap > DOOR_CAP) throw new Error(`a level of ${cap} doors: the game holds ${DOOR_CAP} (sim/doorshome.ts DOOR_CAP, rust/sim/src/doors.rs CAP)`);   // (said as the level loads: there is no other step for the doors of a bigger one)
    home.keep(DOOR_BIT, 'doors'); this.base = simPart(P_DOORS) >>> 3;
  }
  /** the array the columns are in now (asked at every read: a block's view dies when the block moves) */
  get d(): Float64Array { return this.home.d; }
}

// ---- the hand-over of one step (rust/sim/src/doors.rs: its tables and their columns, by the same numbers)
/** the level's facts of a door */
export const DD = { hx: 0, hz: 1, closed: 2, width: 3, height: 4, thick: 5, minAuth: 6, maxAuth: 7, keySide: 8, closer: 9, slab: 10, slabAng: 11, fcx: 12, fcz: 13, manLast: 14 } as const;
/** what comes back of a door */
export const DO = { placed: 0, h1x: 1, h1z: 2, h2x: 3, h2z: 4 } as const;
/** a contact, in and out */
export const DK = { x: 0, y: 1, z: 2, radius: 3, flags: 4, who: 5, hand: 6, runBash: 7, runRip: 8, runLevel: 9, vx: 10, vz: 11, man: 12, opens: 13, opensHi: 14,
  blockedBy: 15, bashed: 16, ripped: 17, bashedAt: 18, struck: 19, struckBy: 20, struckTier: 21, struckFloor: 22, struckDx: 23, struckDz: 24, moved: 25, manAsked: 26 } as const;
export const KF = { bash: 1, quiet: 2, weak: 4, key: 8, solid: 16, round: 32, vel: 64, run: 128, hand: 256 } as const;
/** a noise */
export const DE = { door: 0, sound: 1, who: 2, level: 3, fresh: 4, px: 5, pz: 6 } as const;
export const DOOR_EV = 7;
/** the step's numbers, in and out */
export const DP = { dt: 0, doors: 1, contacts: 2, flags: 3, handRate: 4, minSpeed: 5, floorRad: 6, rate: 7, slide: 8, again: 9, wminX: 10, wmaxX: 11, wminZ: 12, wmaxZ: 13, foot: 14, settleY1: 15,
  events: 16, carried: 17, stopped: 18, passed: 19, reached: 20, held: 21, aside: 22, rounds: 23, wrong: 24, turned: 25 } as const;
export const DF = { oneWay: 1, exitsOut: 2, leafWay: 4, stay: 8, throws: 16, soft: 32, standable: 64, settle: 128 } as const;
/** the man of a door's last hand was not asked of the game (no rule of this step can want him) */
export const MAN_UNASKED = -1;

/** how wide the hand-over's tables are: what comes back of a door, a contact's numbers, the rules' numbers (doors.rs NOUT, NCON, NPAR) */
export const DOOR_OUTS = 5;
export const DOOR_CONS = 27;
export const DOOR_PARS = 32;
/** where the hand-over's tables are in the module's memory (indices into its doubles), and how much each holds: asked once */
export interface DoorIo { x: SimExports; def: number; out: number; con: number; ev: number; par: number; maxContacts: number; maxEvents: number }
let IO: DoorIo | null = null;
export function doorIo(): DoorIo {
  if (IO) return IO;
  const x = sim();
  if (x.doors_cap(0) !== DOOR_CAP || x.doors_cap(5) !== DOOR_COLS || x.doors_cap(4) !== DOOR_PARS || Object.keys(DO).length !== DOOR_OUTS || Object.keys(DK).length !== DOOR_CONS) throw new Error(`the Rust module keeps ${x.doors_cap(0)} doors of ${x.doors_cap(5)} columns and src/sim/doorshome.ts ${DOOR_CAP} of ${DOOR_COLS}: bun run tools/sim/build.ts`);
  return IO = { x, def: x.doors_addr(0) / 8, out: x.doors_addr(1) / 8, con: x.doors_addr(2) / 8, ev: x.doors_addr(3) / 8, par: x.doors_addr(4) / 8, maxContacts: x.doors_cap(2), maxEvents: x.doors_cap(3) };
}
/** the module's memory as doubles, for the hand-over (never kept) */
export const doorMem = f64;

/** COUNTS FOR THE CHECKS (the host's: no game reads them): door steps the module took and the TypeScript took, leaves the module turned, noises it listed,
 *  steps answered both ways (collision.ts BOTH) and how many of those differed, with the first in words */
export const DOORS_STEPPED = { there: 0, here: 0, turned: 0, noises: 0, both: 0, differ: 0, first: '' };
