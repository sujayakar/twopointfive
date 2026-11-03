//! THE DOORS' STEP (src/game/doors.ts Doors.update, and Door.place at its end) — every leaf of a level for one tick: what was written onto it since the last
//! one, the men and the bodies in its swing (who turns it, whom it moves, whom it passes), its free swing, its stops, its closer, its latch and its lock's
//! clocks; and then where its box stands.
//!
//! STATEMENT FOR STATEMENT, as lines.rs is: the same doors in the same order, the same three passes over the same contacts, every sum in the order the
//! TypeScript writes it. tools/qa/door-ident.ts and the missions of tools/qa/rust-check.ts hold the two together, column for column.
//!
//! WHAT IS STATE, AND WHAT IS HANDED IN. A leaf's own numbers (its angle, its turn, its latch and lock, its clocks, who drove it) are the `doors` part of the
//! game's block (state.rs): flat columns, a door an index. What the step needs of the rest of the game is an ARGUMENT, written by the host for the one call
//! and kept nowhere (`IO`, scratch): the level's own facts about each door (`def`), the men and bodies near the leaves (`con`: a man's place, what he is
//! doing, what the game knows of him), the switches and numbers of the rules as they stand (`par`). What comes back the same way: where each contact ends
//! and what a leaf did to him, the noises the leaves made, in order (`ev`), and each leaf's two handles (`out`: the picture's).
//!
//! THE LEAF'S BOX IS WRITTEN HERE. A rule in this module that moves a box writes the box world's ROW (world.rs: list 0 of the leaves, a door's own index)
//! and the host makes the Box object the picture draws follow it (src/scene/boxMirror.ts `follow`) before any TypeScript runs again. So a rule here that
//! asks a line after it has turned a leaf asks the leaf where it now stands, with no crossing.
//!
//! The floor a moved man may stand on is the statics' (collide.rs, the same world), asked here as the host's two functions ask it (game.ts: the doors'
//! `standable`; player.ts settleSpot, bodyR).
use crate::collide;
use crate::maths::{cos, exp, hypot2, sin};
use crate::state::state;
use crate::world::{clamp, js_abs, js_max, js_min, world, REC, R_C, R_COS, R_SIN, R_YAW};
use core::mem::size_of;
use prim::{floor, sqrt};

/// doors a level can have and still keep them here (more: the TypeScript keeps them, as it did)
pub const CAP: usize = 64;

// ---- THE COLUMNS (src/sim/doorshome.ts DC names them by the same numbers; tools/qa/rust-check.ts holds the two lists to each other)
pub const ANGLE: usize = 0;
pub const VEL: usize = 1;
pub const QUIET_T: usize = 2;
pub const LATCHED: usize = 3;
pub const LAST_WHO: usize = 4;
pub const SINCE_BASH: usize = 5;
pub const CLOSING: usize = 6;
pub const LOCKED: usize = 7;
pub const LOCK_BROKEN: usize = 8;
pub const NOTICED: usize = 9;
pub const NOTICED_T: usize = 10;
pub const PICK: usize = 11;
pub const PICKING: usize = 12;
pub const PICK_IDLE: usize = 13;
pub const PICK_CLOCK: usize = 14;
pub const PICK_N: usize = 15;
pub const RATTLE_T: usize = 16;
pub const GUARD_T: usize = 17;
pub const PASSERS: usize = 18;
pub const DEALT: usize = 19;
pub const OUTWARD: usize = 20;
pub const CLEARWAY: usize = 21;
pub const PULL_T: usize = 22;
pub const VEL_SEEN: usize = 23;
pub const SWEPT: usize = 24;
pub const SOLID_T: usize = 25;
pub const PRESS_T: usize = 26;
pub const OPEN_T: usize = 27;
pub const MIRRORED: usize = 28;
pub const ANGLE_SEEN: usize = 29;
pub const OFF: usize = 30;
pub const SHOT: usize = 31;
pub const CREAK_ARMED: usize = 32;
pub const POS_ANGLE: usize = 33;
pub const POS_X: usize = 34;
pub const POS_Z: usize = 35;
pub const DRIVE: usize = 36;
pub const DRIVE_WHO: usize = 37;
pub const DRIVE_DIR: usize = 38;
pub const DRIVE_KIND: usize = 39;
pub const DRIVE_T: usize = 40;
pub const DRIVE_FROM: usize = 41;
pub const DRIVE_QUIET: usize = 42;
pub const DRIVE_SIDE: usize = 43;
pub const DRIVE_RATE: usize = 44;
pub const DRIVE_PX: usize = 45;
pub const DRIVE_PZ: usize = 46;
pub const DRIVE_HIT: usize = 47;
pub const NCOL: usize = 48;

/// The doors' part of a game's block: a column a number of a leaf, a door an index (a truth is 1, a man is his id, a set of men its bits as a number).
#[repr(C)]
pub struct Doors {
    pub c: [[f64; CAP]; NCOL],
}
const _: () = assert!(size_of::<Doors>() == NCOL * CAP * 8);

// ---- WHAT THE HOST HANDS IN AND TAKES BACK FOR ONE STEP (scratch: nothing here is kept from one call to the next)
/// contacts one step can carry (more: the TypeScript steps the doors that tick)
pub const MAXC: usize = 256;
/// noises one step can list
pub const MAXE: usize = 1024;

// what the level says of a door (DoorDef and what the Door made of it), a column each
pub const D_HX: usize = 0;
pub const D_HZ: usize = 1;
pub const D_CLOSED: usize = 2;
pub const D_WIDTH: usize = 3;
pub const D_HEIGHT: usize = 4;
pub const D_THICK: usize = 5;
pub const D_MIN_AUTH: usize = 6;
pub const D_MAX_AUTH: usize = 7;
pub const D_KEY_SIDE: usize = 8;
pub const D_CLOSER: usize = 9;
/// 1 = the leaf is off its hinges and has a slab, whose `ang` is D_SLAB_ANG
pub const D_SLAB: usize = 10;
pub const D_SLAB_ANG: usize = 11;
pub const D_FCX: usize = 12;
pub const D_FCZ: usize = 13;
/// what the game knows of the man who last had his hand on it (Doors.man of Door.lastWho), or D_UNASKED where no rule of this step can ask
pub const D_MAN_LAST: usize = 14;
pub const NDEF: usize = 15;
pub const D_UNASKED: f64 = -1.0;

// what comes back about a door: was its box placed, and where its two handles are
pub const O_PLACED: usize = 0;
pub const O_H1X: usize = 1;
pub const O_H1Z: usize = 2;
pub const O_H2X: usize = 3;
pub const O_H2Z: usize = 4;
pub const NOUT: usize = 5;

// a contact (doors.ts DoorContact), a column each: in …
pub const K_X: usize = 0;
pub const K_Y: usize = 1;
pub const K_Z: usize = 2;
pub const K_RADIUS: usize = 3;
pub const K_FLAGS: usize = 4;
pub const K_WHO: usize = 5;
pub const K_HAND: usize = 6;
pub const K_RUN_BASH: usize = 7;
pub const K_RUN_RIP: usize = 8;
pub const K_RUN_LEVEL: usize = 9;
pub const K_VX: usize = 10;
pub const K_VZ: usize = 11;
pub const K_MAN: usize = 12;
/// the doors whose lock his keys turn: bits 0‥31, bits 32‥63
pub const K_OPENS: usize = 13;
pub const K_OPENS_HI: usize = 14;
// … and out (K_X and K_Z are his place after the step)
pub const K_BLOCKED_BY: usize = 15;
pub const K_BASHED: usize = 16;
pub const K_RIPPED: usize = 17;
pub const K_BASHED_AT: usize = 18;
pub const K_STRUCK: usize = 19;
pub const K_STRUCK_BY: usize = 20;
pub const K_STRUCK_TIER: usize = 21;
pub const K_STRUCK_FLOOR: usize = 22;
pub const K_STRUCK_DX: usize = 23;
pub const K_STRUCK_DZ: usize = 24;
pub const K_MOVED: usize = 25;
pub const K_MAN_ASKED: usize = 26;
pub const NCON: usize = 27;
// a contact's flags
pub const KF_BASH: u32 = 1;
pub const KF_QUIET: u32 = 2;
pub const KF_WEAK: u32 = 4;
pub const KF_KEY: u32 = 8;
pub const KF_SOLID: u32 = 16;
pub const KF_ROUND: u32 = 32;
pub const KF_VEL: u32 = 64;
pub const KF_RUN: u32 = 128;
pub const KF_HAND: u32 = 256;

// a noise (doors.ts DoorEvent): the door, the sound (the place of its name in DoorSound), who, how loud, whether the door's kept centre was made anew for
// it, and that centre
pub const E_DOOR: usize = 0;
pub const E_SOUND: usize = 1;
pub const E_WHO: usize = 2;
pub const E_LEVEL: usize = 3;
pub const E_FRESH: usize = 4;
pub const E_PX: usize = 5;
pub const E_PZ: usize = 6;
pub const NEV: usize = 7;
pub const S_CREAK: f64 = 0.0;
pub const S_BANG: f64 = 1.0;
pub const S_LATCH: f64 = 2.0;
pub const S_BASH: f64 = 3.0;
pub const S_RATTLE: f64 = 4.0;

// the step's own numbers: in …
pub const P_DT: usize = 0;
pub const P_DOORS: usize = 1;
pub const P_CONTACTS: usize = 2;
pub const P_FLAGS: usize = 3;
pub const P_HAND_RATE: usize = 4;
pub const P_MIN_SPEED: usize = 5;
pub const P_FLOOR_RAD: usize = 6;
pub const P_RATE: usize = 7;
pub const P_SLIDE: usize = 8;
pub const P_AGAIN: usize = 9;
pub const P_WMIN_X: usize = 10;
pub const P_WMAX_X: usize = 11;
pub const P_WMIN_Z: usize = 12;
pub const P_WMAX_Z: usize = 13;
/// the foot of a walker's span as the statics are asked it (collision.ts footBand(0.2))
pub const P_FOOT: usize = 14;
/// the head of the settled man's span (player.ts settleSpot: crouched or not)
pub const P_SETTLE_Y1: usize = 15;
// … and out
pub const P_EVENTS: usize = 16;
pub const P_CARRIED: usize = 17;
pub const P_STOPPED: usize = 18;
pub const P_PASSED: usize = 19;
pub const P_REACHED: usize = 20;
pub const P_HELD: usize = 21;
pub const P_ASIDE: usize = 22;
pub const P_ROUNDS: usize = 23;
/// something this step could not be: a noise past the list's end (1), a man the host was not asked for (2)
pub const P_WRONG: usize = 24;
/// leaves this step turned (their angle is not what it was): a count for the checks
pub const P_TURNED: usize = 25;
pub const NPAR: usize = 32;
// the switches (P_FLAGS)
pub const F_ONE_WAY: u32 = 1;
pub const F_EXITS_OUT: u32 = 2;
pub const F_LEAF_WAY: u32 = 4;
pub const F_STAY: u32 = 8;
pub const F_THROWS: u32 = 16;
pub const F_SOFT: u32 = 32;
pub const F_STANDABLE: u32 = 64;
pub const F_SETTLE: u32 = 128;

#[repr(C)]
pub struct Io {
    pub def: [[f64; CAP]; NDEF],
    pub out: [[f64; CAP]; NOUT],
    pub con: [[f64; MAXC]; NCON],
    pub ev: [[f64; NEV]; MAXE],
    pub par: [f64; NPAR],
}
static mut IO: Io = unsafe { core::mem::zeroed() };
#[inline(always)]
fn io() -> &'static mut Io {
    unsafe { &mut *core::ptr::addr_of_mut!(IO) }
}
/// where a table of the step's hand-over is (a byte address): 0 def · 1 out · 2 con · 3 ev · 4 par
pub fn addr(table: u32) -> usize {
    let o = io();
    match table {
        0 => o.def.as_ptr() as usize,
        1 => o.out.as_ptr() as usize,
        2 => o.con.as_ptr() as usize,
        3 => o.ev.as_ptr() as usize,
        4 => o.par.as_ptr() as usize,
        _ => 0,
    }
}
/// how much a table holds, by the same numbers (rows: doors, doors, contacts, noises, numbers); 5 = the columns of the state's part
pub fn cap(table: u32) -> usize {
    match table {
        0 | 1 => CAP,
        2 => MAXC,
        3 => MAXE,
        4 => NPAR,
        5 => NCOL,
        _ => 0,
    }
}

// ---- doors.ts's own numbers (tools/qa/rust-check.ts reads both sources and is red where one differs)
const SOLID_SLIDE: f64 = 2.2;
const SOLID_TRIES: f64 = 3.0;
const SOLID_FADE: f64 = 0.5;
const SOLID_SKIN: f64 = 0.02;
const SOLID_RATE: f64 = 18.0;
const SOLID_REACH: f64 = 0.6;
const SOLID_INTO: f64 = 0.4;
const SOLID_BACK: f64 = 3.0;
const PULL_VEL: f64 = 3.9;
const PULL_VEL_QUIET: f64 = 2.2;
const PULL_SECS: f64 = 0.7;
const PRESS_SECS: f64 = 0.08;
const DRIVEN_SECS: f64 = 0.25;
const DRIVEN_VEL: f64 = 0.5;
const STOP_BODY: f64 = 0.3;
const ROUND_SLIDE: f64 = 1.5;
const ROUND_SECS: f64 = 1.2;
const ROUND_STILL: f64 = 1.0;
const SHOVE_OPENS_BELOW: f64 = 1.2;
const SHOVE_RATE: f64 = 7.2;
const TURN_RATE: f64 = 15.0;
const KICK_SUBSTEP: f64 = 0.16;
// what Doors.man answers (doors.ts)
const MAN_SIDE: i32 = 3;
const MAN_FOLDED: i32 = 4;
const MAN_BRACED: i32 = 8;
const MAN_CALM: i32 = 16;
const MAN_HELD: i32 = 32;
// DoorDrive.kind, by its place in doorshome.ts DRIVE_KINDS
const BOOT: f64 = 0.0;
const SWING: f64 = 1.0;
const BODY: f64 = 2.0;
const HAND: f64 = 3.0;
// DoorStrike.tier
const T_OPEN: f64 = 0.0;
const T_SLAM: f64 = 1.0;
const T_BOOT: f64 = 2.0;
// player.ts: a standing man's radius against the statics, in a door frame, and how the one eases into the other
const BODY_R: f64 = 0.42;
const DOOR_R: f64 = 0.3;
const DOOR_FIT_LAT: [f64; 2] = [0.12, 0.5];
const DOOR_FIT_OUT: [f64; 2] = [0.55, 1.0];
/// scene/boxes.ts BoxFlag.SoundSeal
const SOUND_SEAL: u32 = 1 << 16;

/// Math.sign
#[inline(always)]
fn js_sign(x: f64) -> f64 {
    if x > 0.0 {
        1.0
    } else if x < 0.0 {
        -1.0
    } else {
        x
    }
}
/// `x >= 0 ? 1 : -1`
#[inline(always)]
fn side_of(x: f64) -> f64 {
    if x >= 0.0 {
        1.0
    } else {
        -1.0
    }
}

/// One step's view of the game: the leaves' state, the hand-over, which world, and the switches.
struct Step {
    s: &'static mut Doors,
    io: &'static mut Io,
    w: usize,
    f: u32,
    dt: f64,
    n: usize,
    nc: usize,
    ev: usize,
}

impl Step {
    #[inline(always)]
    fn on(&self, k: usize, i: usize) -> bool {
        self.s.c[k][i] != 0.0
    }
    /// Door.swing
    #[inline]
    fn swing(&self, i: usize) -> f64 {
        let s = &self.s.c;
        if s[OUTWARD][i] != 0.0 && self.f & F_EXITS_OUT != 0 {
            s[OUTWARD][i]
        } else if s[CLEARWAY][i] != 0.0 && self.f & F_LEAF_WAY != 0 {
            s[CLEARWAY][i]
        } else {
            s[DEALT][i]
        }
    }
    /// Door.minA, maxA
    #[inline]
    fn min_a(&self, i: usize) -> f64 {
        if self.f & F_ONE_WAY != 0 && self.swing(i) > 0.0 {
            0.0
        } else {
            self.io.def[D_MIN_AUTH][i]
        }
    }
    #[inline]
    fn max_a(&self, i: usize) -> f64 {
        if self.f & F_ONE_WAY != 0 && self.swing(i) < 0.0 {
            0.0
        } else {
            self.io.def[D_MAX_AUTH][i]
        }
    }
    /// Door.keep
    fn keep(&mut self, i: usize) {
        if self.s.c[ANGLE][i] * self.swing(i) < 0.0 {
            self.s.c[ANGLE][i] = clamp(-self.s.c[ANGLE][i], self.min_a(i), self.max_a(i));
            self.s.c[VEL][i] = -self.s.c[VEL][i];
            self.s.c[MIRRORED][i] += 1.0;
        }
    }
    /// Door.dir
    #[inline]
    fn dir(&self, i: usize) -> (f64, f64) {
        let a = self.io.def[D_CLOSED][i] + self.s.c[ANGLE][i];
        (cos(a), sin(a))
    }
    /// Door.restsTo
    #[inline]
    fn rests_to(&self, i: usize) -> bool {
        self.on(SWEPT, i) && !self.on(LATCHED, i) && self.s.c[ANGLE][i] == 0.0 && self.s.c[VEL][i] == 0.0
    }
    /// Doors.pullFace
    fn pull_face(&self, i: usize, x: f64, z: f64) -> bool {
        let c = self.io.def[D_CLOSED][i];
        ((x - self.io.def[D_HX][i]) * -sin(c) + (z - self.io.def[D_HZ][i]) * cos(c)) * self.swing(i) >= 0.0
    }
    /// Doors.opening
    fn opening(&self, i: usize, dir: f64) -> bool {
        if self.f & F_ONE_WAY != 0 {
            dir == self.swing(i)
        } else {
            js_abs(self.s.c[ANGLE][i]) < 0.05 || dir * self.s.c[ANGLE][i] > 0.0
        }
    }
    /// Doors.hasCloser
    #[inline]
    fn has_closer(&self, i: usize) -> bool {
        self.f & F_STAY == 0 && self.io.def[D_CLOSER][i] != 0.0
    }
    /// the Door's `pos` as an event reads it: its kept centre, made anew where the leaf has turned since it was kept → (fresh, x, z)
    fn pos(&mut self, i: usize) -> (f64, f64, f64) {
        let mut fresh = 0.0;
        if self.s.c[ANGLE][i] != self.s.c[POS_ANGLE][i] {
            self.s.c[POS_ANGLE][i] = self.s.c[ANGLE][i];
            let u = self.dir(i);
            self.s.c[POS_X][i] = self.io.def[D_HX][i] + u.0 * self.io.def[D_WIDTH][i] * 0.5;
            self.s.c[POS_Z][i] = self.io.def[D_HZ][i] + u.1 * self.io.def[D_WIDTH][i] * 0.5;
            fresh = 1.0;
        }
        (fresh, self.s.c[POS_X][i], self.s.c[POS_Z][i])
    }
    /// Doors.emit: a noise onto the list
    fn emit(&mut self, i: usize, sound: f64, who: f64, level: f64) {
        let (fresh, x, z) = self.pos(i);
        if self.ev >= MAXE {
            self.io.par[P_WRONG] = 1.0;
            return;
        }
        self.io.ev[self.ev] = [i as f64, sound, who, level, fresh, x, z];
        self.ev += 1;
    }
    /// Doors.rattle
    fn rattle(&mut self, i: usize, who: f64, level: f64) {
        if self.s.c[RATTLE_T][i] > 0.0 || self.on(OFF, i) {
            return;
        }
        self.s.c[RATTLE_T][i] = 0.6;
        self.s.c[LAST_WHO][i] = who;
        self.emit(i, S_RATTLE, who, level);
    }
    /// Doors.passBit
    fn pass_bit(&self, c: usize) -> i32 {
        let who = self.io.con[K_WHO][c];
        if who < 0.0 {
            0
        } else if who < 32.0 {
            1i32.wrapping_shl(who as u32)
        } else if self.kf(c) & KF_KEY != 0 && self.f & F_STAY != 0 {
            1i32 << (2 + ((who as i64 - 32) % 8) as u32)
        } else {
            0
        }
    }
    #[inline(always)]
    fn kf(&self, c: usize) -> u32 {
        self.io.con[K_FLAGS][c] as u32
    }
    /// doors.ts `opens`: does this contact's key ring get it through door i's lock
    #[inline]
    fn opens(&self, c: usize, i: usize) -> bool {
        self.kf(c) & KF_KEY != 0 || (if i < 32 { self.io.con[K_OPENS][c] as u32 >> i } else { self.io.con[K_OPENS_HI][c] as u32 >> (i - 32) }) & 1 != 0
    }
    /// Doors.man of a contact (the game's answer, handed in)
    #[inline]
    fn man_of(&self, c: usize) -> i32 {
        self.io.con[K_MAN][c] as i32
    }

    // ---- the floor (the host's two functions, over the statics of this world)
    /// collision.ts collideCircle: (x, z) pushed out of the statics over [0.2, y1]
    fn collide(&self, x: &mut f64, z: &mut f64, r: f64, y1: f64, iterations: u32) -> bool {
        let hit = collide::circle(self.w, *x, *z, r, self.io.par[P_FOOT], y1, iterations) != 0;
        let pt = world(self.w).pt;
        *x = pt[0];
        *z = pt[1];
        hit
    }
    /// collision.ts clampToWorld
    fn clamp_to_world(&self, x: &mut f64, z: &mut f64, r: f64) -> bool {
        let p = &self.io.par;
        let nx = js_min(p[P_WMAX_X] - r, js_max(p[P_WMIN_X] + r, *x));
        let nz = js_min(p[P_WMAX_Z] - r, js_max(p[P_WMIN_Z] + r, *z));
        let moved = nx != *x || nz != *z;
        *x = nx;
        *z = nz;
        moved
    }
    /// game.ts: the doors' `standable` — is there floor for a body of radius r at (x, z)
    fn standable(&self, x: f64, z: f64, r: f64) -> bool {
        let (mut qx, mut qz) = (x, z);
        self.collide(&mut qx, &mut qz, js_min(r, 0.3), 1.5, 1);
        hypot2(qx - x, qz - z) < 0.02 && !self.clamp_to_world(&mut qx, &mut qz, 0.3)
    }
    /// player.ts bodyR: a standing man's radius at p (a door frame lets him through narrower)
    fn body_r(&self, x: f64, z: f64) -> f64 {
        let mut k = 0.0;
        for i in 0..self.n {
            let (dx, dz) = (x - self.io.def[D_FCX][i], z - self.io.def[D_FCZ][i]);
            if dx * dx + dz * dz > 1.3 {
                continue;
            }
            let c = self.io.def[D_CLOSED][i];
            let (ux, uz) = (cos(c), sin(c));
            let (lat, out) = (js_abs(dx * ux + dz * uz), js_abs(dz * ux - dx * uz));
            let kl = 1.0 - clamp((lat - DOOR_FIT_LAT[0]) / (DOOR_FIT_LAT[1] - DOOR_FIT_LAT[0]), 0.0, 1.0);
            let ko = 1.0 - clamp((out - DOOR_FIT_OUT[0]) / (DOOR_FIT_OUT[1] - DOOR_FIT_OUT[0]), 0.0, 1.0);
            k = js_max(k, kl * kl * (3.0 - 2.0 * kl) * ko * ko * (3.0 - 2.0 * ko));
        }
        BODY_R - (BODY_R - DOOR_R) * k
    }
    /// player.ts settleSpot: where the statics let a man moved to (x, z) stand
    fn settle(&self, x: &mut f64, z: &mut f64) {
        let y1 = self.io.par[P_SETTLE_Y1];
        for _ in 0..4 {
            let r = self.body_r(*x, *z);
            self.collide(x, z, r, y1, 6);
            if !self.clamp_to_world(x, z, BODY_R) {
                break;
            }
        }
    }

    /// Doors.driven
    fn driven(&mut self, i: usize, who: f64, dir: f64, kind: f64, mut quiet: bool, rate: f64, m: i32) {
        let s = side_of(dir);
        if kind != BOOT && m & MAN_CALM != 0 {
            quiet = true;
        }
        let held = kind == BODY || kind == HAND;
        let c = &mut self.s.c;
        if c[DRIVE][i] != 0.0 && c[DRIVE_WHO][i] == who && c[DRIVE_DIR][i] == s && c[DRIVE_KIND][i] == kind {
            if !held || c[DRIVE_T][i] >= DRIVEN_SECS {
                c[DRIVE_HIT][i] = -1.0;
            }
            c[DRIVE_T][i] = 0.0;
            c[DRIVE_QUIET][i] = quiet as u32 as f64;
            c[DRIVE_RATE][i] = rate;
            c[DRIVE_SIDE][i] = (m & MAN_SIDE) as f64;
            if !held {
                c[DRIVE_FROM][i] = c[ANGLE][i];
            }
        } else {
            c[DRIVE][i] = 1.0;
            c[DRIVE_WHO][i] = who;
            c[DRIVE_DIR][i] = s;
            c[DRIVE_KIND][i] = kind;
            c[DRIVE_T][i] = 0.0;
            c[DRIVE_FROM][i] = c[ANGLE][i];
            c[DRIVE_QUIET][i] = quiet as u32 as f64;
            c[DRIVE_SIDE][i] = (m & MAN_SIDE) as f64;
            c[DRIVE_RATE][i] = rate;
            c[DRIVE_PX][i] = f64::NAN;
            c[DRIVE_PZ][i] = f64::NAN;
            c[DRIVE_HIT][i] = -1.0;
        }
    }
    /// Doors.walked
    fn walked(&mut self, i: usize, c: usize, dir: f64, lever: f64) {
        let who = self.io.con[K_WHO][c];
        let d = &self.s.c;
        let has = d[DRIVE][i] != 0.0;
        if has && d[DRIVE_WHO][i] == who && d[DRIVE_KIND][i] == BOOT && d[SINCE_BASH][i] < 0.6 {
            return;
        }
        let on = has && d[DRIVE_WHO][i] == who && d[DRIVE_KIND][i] == BODY && d[DRIVE_DIR][i] == side_of(dir) && d[DRIVE_T][i] <= 2.0 * self.dt + 1e-6 && d[DRIVE_PX][i] == d[DRIVE_PX][i];
        let rate = if on { js_min(hypot2(self.io.con[K_X][c] - d[DRIVE_PX][i], self.io.con[K_Z][c] - d[DRIVE_PZ][i]) / js_max(self.dt, 1e-3) / lever, TURN_RATE) } else { 0.0 };
        let quiet = self.kf(c) & KF_QUIET != 0;
        let m = self.man_of(c);
        self.driven(i, who, dir, BODY, quiet, rate, m);
    }
    /// Doors.carry
    #[allow(clippy::too_many_arguments)]
    fn carry(&mut self, c: usize, i: usize, pen: f64, dist: f64, qx: f64, qz: f64, perp: f64) -> bool {
        let p = &self.io.par;
        let (t_rate, t_slide) = (p[P_RATE], p[P_SLIDE]);
        if pen > t_rate * self.dt {
            return false;
        }
        let u = self.dir(i);
        let (mut nx, mut nz, mut n) = (qx, qz, dist);
        if n < 1e-4 {
            let s = side_of(perp);
            nx = -u.1 * s;
            nz = u.0 * s;
            n = 1.0;
        }
        let k = (pen + 0.001) / n;
        let sl = t_slide * self.dt;
        let (x, z) = (self.io.con[K_X][c] + nx * k, self.io.con[K_Z][c] + nz * k);
        let r = self.io.con[K_RADIUS][c];
        let free = self.f & F_STANDABLE == 0;
        if free || self.standable(x + u.0 * sl, z + u.1 * sl, r) {
            self.io.con[K_X][c] = x + u.0 * sl;
            self.io.con[K_Z][c] = z + u.1 * sl;
        } else if free || self.standable(x, z, r) {
            self.io.con[K_X][c] = x;
            self.io.con[K_Z][c] = z;
        } else {
            return false;
        }
        self.io.con[K_MOVED][c] = 1.0;
        true
    }
    /// Doors.solidCarry
    #[allow(clippy::too_many_arguments)]
    fn solid_carry(&mut self, c: usize, i: usize, pen: f64, dist: f64, qx: f64, qz: f64, perp: f64) -> bool {
        if pen > SOLID_RATE * self.dt {
            return false;
        }
        let (hx, hz, wd) = (self.io.def[D_HX][i], self.io.def[D_HZ][i], self.io.def[D_WIDTH][i]);
        let u = self.dir(i);
        let (mut nx, mut nz, mut n) = (qx, qz, dist);
        if n < 1e-4 {
            let s = side_of(perp);
            nx = -u.1 * s;
            nz = u.0 * s;
            n = 1.0;
        }
        let k = (pen + 0.001) / n;
        let con = &self.io.con;
        let own = if self.kf(c) & KF_VEL != 0 { js_max(0.0, -(con[K_VX][c] * u.0 + con[K_VZ][c] * u.1)) } else { 0.0 };
        let sl = (SOLID_SLIDE + own) * self.dt;
        let (mut p0, mut p2) = (con[K_X][c] + nx * k + u.0 * sl, con[K_Z][c] + nz * k + u.1 * sl);
        let r = con[K_RADIUS][c];
        if self.f & F_SETTLE != 0 {
            self.settle(&mut p0, &mut p2);
        } else if self.f & F_STANDABLE != 0 && !self.standable(p0, p2, r) {
            return false;
        }
        let (px, pz) = (p0 - hx, p2 - hz);
        let t = clamp(px * u.0 + pz * u.1, 0.0, wd);
        let left = r + self.io.def[D_THICK][i] / 2.0 - hypot2(px - u.0 * t, pz - u.1 * t);
        if left > SOLID_SKIN {
            return false;
        }
        self.io.con[K_X][c] = p0;
        self.io.con[K_Z][c] = p2;
        self.io.par[P_CARRIED] += 1.0;
        true
    }
    /// Doors.leafPresses
    fn leaf_presses(&self, i: usize) -> bool {
        let (hx, hz, wd) = (self.io.def[D_HX][i], self.io.def[D_HZ][i], self.io.def[D_WIDTH][i]);
        let ht = self.io.def[D_THICK][i] / 2.0;
        let u = self.dir(i);
        if self.f & F_ONE_WAY != 0 && self.s.c[PULL_T][i] > 0.0 {
            return false;
        }
        let passers = self.s.c[PASSERS][i] as i32;
        for c in 0..self.nc {
            if self.kf(c) & KF_WEAK != 0 {
                continue;
            }
            let con = &self.io.con;
            let r = con[K_RADIUS][c];
            let (px, pz) = (con[K_X][c] - hx, con[K_Z][c] - hz);
            let reach = wd + r + 0.1;
            if px * px + pz * pz > reach * reach {
                continue;
            }
            if self.pass_bit(c) & passers != 0 {
                continue;
            }
            let along = px * u.0 + pz * u.1;
            if along <= 0.14 || along >= wd + r {
                continue;
            }
            let t = clamp(along, 0.0, wd);
            let dist = hypot2(px - u.0 * t, pz - u.1 * t);
            if r + ht - dist > 0.0 {
                return true;
            }
        }
        false
    }

    /// Door.place: the leaf's row of the box world written where it now stands, and its handles handed back
    fn place(&mut self, i: usize) {
        if self.on(OFF, i) {
            return;
        }
        if self.f & F_ONE_WAY != 0 {
            self.keep(i);
        }
        self.s.c[VEL_SEEN][i] = self.s.c[VEL][i];
        self.s.c[ANGLE_SEEN][i] = self.s.c[ANGLE][i];
        let rt = self.s.c[RATTLE_T][i];
        let jig = if rt > 0.35 { sin(rt * 90.0) * 0.005 } else { 0.0 };
        // Door.poseAt(angle + jig)
        let d = &self.io.def;
        let (wd, hx, hz) = (d[D_WIDTH][i], d[D_HX][i], d[D_HZ][i]);
        let a = d[D_CLOSED][i] + (self.s.c[ANGLE][i] + jig);
        let u = (cos(a), sin(a));
        let yaw = -a;
        let n = (-u.1, u.0);
        let off = d[D_THICK][i] / 2.0 + 0.03;
        let c = [hx + u.0 * (wd * 0.5 + 0.01), d[D_HEIGHT][i] / 2.0 + 0.01, hz + u.1 * (wd * 0.5 + 0.01)];
        let h1 = (hx + u.0 * (wd - 0.14) + n.0 * off * 1.0, hz + u.1 * (wd - 0.14) + n.1 * off * 1.0);
        let h2 = (hx + u.0 * (wd - 0.14) + n.0 * off * -1.0, hz + u.1 * (wd - 0.14) + n.1 * off * -1.0);
        let seal = js_abs(self.s.c[ANGLE][i]) < 0.3;
        // the box world's row (list 0 of the leaves: the game's own, a door at its own index)
        let o = world(self.w);
        let b = &mut o.leaf[i * REC..i * REC + REC];
        b[R_C] = c[0];
        b[R_C + 1] = c[1];
        b[R_C + 2] = c[2];
        b[R_COS] = cos(yaw);
        b[R_SIN] = sin(yaw);
        b[R_YAW] = yaw;
        o.leaf_flags[i] = if seal { o.leaf_flags[i] | SOUND_SEAL } else { o.leaf_flags[i] & !SOUND_SEAL };
        let out = &mut self.io.out;
        out[O_PLACED][i] = 1.0;
        out[O_H1X][i] = h1.0;
        out[O_H1Z][i] = h1.1;
        out[O_H2X][i] = h2.0;
        out[O_H2Z][i] = h2.1;
    }

    /// Doors.update's body for door i
    fn door(&mut self, i: usize) {
        let dt = self.dt;
        let thr = self.f & F_THROWS != 0;
        if self.on(OFF, i) {
            self.s.c[VEL][i] = 0.0;
            self.s.c[LATCHED][i] = 0.0;
            self.s.c[CLOSING][i] = 0.0;
            if self.io.def[D_SLAB][i] != 0.0 {
                self.s.c[ANGLE][i] = self.io.def[D_SLAB_ANG][i];
            }
            return;
        }
        let (hx, hz, wd) = (self.io.def[D_HX][i], self.io.def[D_HZ][i], self.io.def[D_WIDTH][i]);
        let ht = self.io.def[D_THICK][i] / 2.0;
        let ow = self.f & F_ONE_WAY != 0;
        let sw = self.swing(i);
        let angle0 = self.s.c[ANGLE][i];
        if self.on(SWEPT, i) && (self.s.c[VEL][i] != self.s.c[VEL_SEEN][i] || self.s.c[ANGLE][i] != self.s.c[ANGLE_SEEN][i]) {
            self.s.c[SWEPT][i] = 0.0;
        }
        let mut eased = false;
        if ow {
            self.keep(i);
            if self.s.c[VEL][i] != self.s.c[VEL_SEEN][i] && !self.on(CLOSING, i) && !self.on(LATCHED, i) {
                if self.s.c[VEL][i] * sw < 0.0 {
                    if self.s.c[ANGLE][i] * sw < SHOVE_OPENS_BELOW {
                        self.s.c[VEL][i] = -self.s.c[VEL][i];
                        self.s.c[PULL_T][i] = PULL_SECS;
                    }
                } else if self.s.c[VEL][i] * sw > 0.0 {
                    // contacts.find(c => !c.weak && c.who === d.lastWho)
                    let last = self.s.c[LAST_WHO][i];
                    let mut by = usize::MAX;
                    for c in 0..self.nc {
                        if self.kf(c) & KF_WEAK == 0 && self.io.con[K_WHO][c] == last {
                            by = c;
                            break;
                        }
                    }
                    if by != usize::MAX && self.pull_face(i, self.io.con[K_X][by], self.io.con[K_Z][by]) && hypot2(self.io.con[K_X][by] - hx, self.io.con[K_Z][by] - hz) < wd + 1.2 {
                        self.s.c[PULL_T][i] = PULL_SECS;
                    } else {
                        self.s.c[OPEN_T][i] = 0.0;
                    }
                }
            }
            if (self.s.c[ANGLE][i] - self.s.c[ANGLE_SEEN][i]) * sw > 1e-4 {
                self.s.c[OPEN_T][i] = 0.0;
                eased = true;
            }
        }
        if thr {
            let da = self.s.c[ANGLE][i] - self.s.c[ANGLE_SEEN][i];
            let pulled = ow && self.s.c[PULL_T][i] > 0.0;
            let rate = js_abs(da) / js_max(dt, 1e-3);
            if pulled && self.on(DRIVE, i) {
                self.s.c[DRIVE][i] = 0.0;
            }
            let vel = self.s.c[VEL][i];
            let free = !self.on(CLOSING, i) && !self.on(LATCHED, i) && !pulled;
            if vel != self.s.c[VEL_SEEN][i] && vel != 0.0 && free && self.opening(i, js_sign(vel)) {
                if !(self.on(DRIVE, i) && self.s.c[DRIVE_T][i] == 0.0 && self.s.c[DRIVE_KIND][i] == BOOT) {
                    let m = self.man_last(i);
                    self.driven(i, self.s.c[LAST_WHO][i], vel, SWING, false, 0.0, m);
                }
            } else if da != 0.0
                && rate >= (if self.on(DRIVE, i) && self.s.c[DRIVE_KIND][i] == HAND && self.s.c[DRIVE_T][i] < DRIVEN_SECS { 0.5 } else { 1.0 }) * self.io.par[P_HAND_RATE]
                && rate <= TURN_RATE
                && free
                && self.opening(i, js_sign(da))
            {
                let m = self.man_last(i);
                self.driven(i, self.s.c[LAST_WHO][i], da, HAND, false, rate, m);
            }
        } else if self.on(DRIVE, i) {
            self.s.c[DRIVE][i] = 0.0;
        }
        let kicked = self.s.c[SINCE_BASH][i] < 0.6;
        if kicked && !self.on(LATCHED, i) {
            let total = clamp(self.s.c[ANGLE][i] + self.s.c[VEL][i] * dt, self.min_a(i), self.max_a(i)) - self.s.c[ANGLE][i];
            let n = -floor(-(js_abs(total) / KICK_SUBSTEP));
            if n <= 1.0 {
                self.s.c[ANGLE][i] = clamp(self.s.c[ANGLE][i] + self.s.c[VEL][i] * dt, self.min_a(i), self.max_a(i));
            } else {
                let mut k = 0.0;
                while k < n {
                    let a0 = self.s.c[ANGLE][i];
                    self.s.c[ANGLE][i] = a0 + total / n;
                    if self.leaf_presses(i) {
                        if !thr {
                            self.s.c[ANGLE][i] = a0;
                        }
                        break;
                    }
                    k += 1.0;
                }
            }
        }
        let (mut touched, mut yank, mut pushed, mut body_block) = (false, false, false, false);
        let (mut push_dir, mut key_touched) = (0.0, false);
        let mut here: i32 = 0;
        let mut pressed = false;
        let mut ran = false;
        let mut walker = usize::MAX;
        let comes = self.s.c[VEL][i] * sw > 0.0 || (self.s.c[ANGLE][i] - self.s.c[ANGLE_SEEN][i]) * sw > 1e-4;
        // (DOORWAY.solid) he stands back from its swing
        if ow && self.s.c[PULL_T][i] > 0.0 && comes && !self.on(OFF, i) && !self.on(LATCHED, i) && self.s.c[SOLID_T][i] <= SOLID_TRIES - 1.0 {
            for c in 0..self.nc {
                let kf = self.kf(c);
                if kf & KF_SOLID == 0 || kf & KF_WEAK != 0 {
                    continue;
                }
                let th = self.io.def[D_CLOSED][i];
                let (ax, az) = (cos(th), sin(th));
                let (cx, cz, r) = (self.io.con[K_X][c], self.io.con[K_Z][c], self.io.con[K_RADIUS][c]);
                let (px, pz) = (cx - hx, cz - hz);
                let a = px * ax + pz * az;
                let o = sw * (ax * pz - az * px);
                let rr = wd + r + self.io.def[D_THICK][i] / 2.0 + SOLID_SKIN;
                if o <= 0.0 || a < 0.0 || a * a + o * o >= rr * rr {
                    continue;
                }
                let u = self.dir(i);
                if side_of(u.0 * pz - u.1 * px) != sw {
                    continue;
                }
                let (bx, bz) = (-az * sw, ax * sw);
                let back = sqrt(rr * rr - a * a) - o;
                let aside = sqrt(rr * rr - o * o) - a;
                let way = if self.free(cx + bx * back, cz + bz * back) {
                    1
                } else if self.free(cx + ax * aside, cz + az * aside) {
                    2
                } else {
                    0
                };
                if way == 0 {
                    continue;
                }
                let (vx, vz) = (self.io.con[K_VX][c], self.io.con[K_VZ][c]);
                let own = if kf & KF_VEL != 0 { if way == 1 { js_max(0.0, -(bx * vx + bz * vz)) } else { js_max(0.0, -(ax * vx + az * vz)) } } else { 0.0 };
                let step = js_min(if way == 1 { back } else { aside }, (SOLID_BACK + own) * dt);
                let (mut p0, mut p2) = if way == 1 { (cx + bx * step, cz + bz * step) } else { (cx + ax * step, cz + az * step) };
                if self.f & F_SETTLE != 0 {
                    self.settle(&mut p0, &mut p2);
                }
                self.io.con[K_X][c] = p0;
                self.io.con[K_Z][c] = p2;
                self.io.par[P_HELD] += 1.0;
                if way == 2 {
                    self.io.par[P_ASIDE] += 1.0;
                }
            }
        }
        // (DOORWAY.solid) his hand reaches it before his body
        if ow
            && self.s.c[PULL_T][i] <= 0.0
            && !self.on(OFF, i)
            && self.s.c[SOLID_T][i] <= SOLID_TRIES - 1.0
            && self.s.c[OPEN_T][i] >= DRIVEN_SECS
            && self.s.c[VEL][i] * sw <= DRIVEN_VEL
            && self.s.c[ANGLE][i] * sw < js_min(1.35, 0.8 * js_abs(if sw > 0.0 { self.max_a(i) } else { self.min_a(i) }))
        {
            for c in 0..self.nc {
                let kf = self.kf(c);
                if kf & KF_SOLID == 0 || kf & KF_VEL == 0 || kf & KF_BASH != 0 || kf & KF_QUIET != 0 || kf & KF_WEAK != 0 {
                    continue;
                }
                let u = self.dir(i);
                let (r, who) = (self.io.con[K_RADIUS][c], self.io.con[K_WHO][c]);
                let (px, pz) = (self.io.con[K_X][c] - hx, self.io.con[K_Z][c] - hz);
                let along = px * u.0 + pz * u.1;
                let perp = u.0 * pz - u.1 * px;
                let side = side_of(perp);
                let gap = js_abs(perp) - r - self.io.def[D_THICK][i] / 2.0;
                if side != sw || gap <= 0.0 || gap > SOLID_REACH || along <= 0.14 || along >= wd {
                    continue;
                }
                if self.on(LOCKED, i) && self.on(LATCHED, i) && !self.opens(c, i) && side == self.io.def[D_KEY_SIDE][i] {
                    continue;
                }
                let (vx, vz) = (self.io.con[K_VX][c], self.io.con[K_VZ][c]);
                let into = -side * (u.0 * vz - u.1 * vx);
                let sp = hypot2(vx, vz);
                if into < SOLID_INTO || into < 0.5 * sp {
                    continue;
                }
                let meets = along + (vx * u.0 + vz * u.1) * (gap / into);
                if meets <= 0.0 || meets >= wd {
                    continue;
                }
                pressed = true;
                if self.s.c[PRESS_T][i] < PRESS_SECS {
                    continue;
                }
                let quiet = kf & KF_QUIET != 0;
                if self.on(LATCHED, i) || self.rests_to(i) {
                    self.s.c[LATCHED][i] = 0.0;
                    if self.on(CREAK_ARMED, i) {
                        self.emit(i, S_CREAK, who, if quiet { 0.12 } else { 0.3 });
                        self.s.c[CREAK_ARMED][i] = 0.0;
                    }
                }
                self.s.c[VEL][i] = sw * js_max(self.s.c[VEL][i] * sw, if quiet { PULL_VEL_QUIET } else { PULL_VEL });
                self.s.c[CLOSING][i] = 0.0;
                self.s.c[SWEPT][i] = 0.0;
                self.s.c[PULL_T][i] = PULL_SECS;
                touched = true;
                pushed = true;
                self.s.c[LAST_WHO][i] = who;
                self.io.par[P_REACHED] += 1.0;
            }
        }
        for pass in 0..3u32 {
            for c in 0..self.nc {
                let kf = self.kf(c);
                let (weak, key, bash, quiet) = (kf & KF_WEAK != 0, kf & KF_KEY != 0, kf & KF_BASH != 0, kf & KF_QUIET != 0);
                if (if weak { 2 } else if key { 0 } else { 1 }) != pass {
                    continue;
                }
                let (r, who) = (self.io.con[K_RADIUS][c], self.io.con[K_WHO][c]);
                let (vx, vz) = (self.io.con[K_X][c] - hx, self.io.con[K_Z][c] - hz);
                let reach = wd + r + 0.1;
                if vx * vx + vz * vz > reach * reach {
                    continue;
                }
                if weak && self.io.con[K_Y][c] - r > self.io.def[D_HEIGHT][i] {
                    continue;
                }
                let solid = kf & KF_SOLID != 0;
                for _it in 0..3 {
                    let u = self.dir(i);
                    let (px, pz) = (self.io.con[K_X][c] - hx, self.io.con[K_Z][c] - hz);
                    let along = px * u.0 + pz * u.1;
                    let perp = u.0 * pz - u.1 * px;
                    let t = clamp(along, 0.0, wd);
                    let (qx, qz) = (px - u.0 * t, pz - u.1 * t);
                    let dist = hypot2(qx, qz);
                    let pen = r + ht - dist;
                    if pen <= (if weak { 0.002 } else { 0.0 }) {
                        break;
                    }
                    let bit = if !weak { self.pass_bit(c) } else { 0 };
                    let man = if thr && !weak {
                        self.io.con[K_MAN_ASKED][c] = 1.0;
                        self.man_of(c)
                    } else {
                        0
                    };
                    let folded = man & MAN_FOLDED != 0;
                    if bit & (self.s.c[PASSERS][i] as i32) != 0 {
                        here |= bit;
                        break;
                    }
                    if ow && self.s.c[PULL_T][i] > 0.0 && !self.on(LATCHED, i) && !weak {
                        if solid && self.s.c[SOLID_T][i] <= SOLID_TRIES - 1.0 {
                            if side_of(perp) == sw && comes && along > 0.14 && along < wd + r {
                                if self.solid_carry(c, i, pen, dist, qx, qz, perp) {
                                    if self.s.c[VEL][i] * sw > PULL_VEL {
                                        self.s.c[VEL][i] = sw * PULL_VEL;
                                    }
                                    break;
                                }
                                self.s.c[SOLID_T][i] += 1.0;
                                self.io.par[P_STOPPED] += 1.0;
                                if self.s.c[SOLID_T][i] > SOLID_TRIES - 1.0 {
                                    self.io.par[P_PASSED] += 1.0;
                                }
                                self.s.c[ANGLE][i] -= sw * js_min(self.s.c[ANGLE][i] * sw, (pen + 0.001) / js_max(0.18, t));
                                self.s.c[VEL][i] = 0.0;
                                self.s.c[PULL_T][i] = 0.0;
                                break;
                            }
                        } else {
                            if bit != 0 {
                                self.s.c[PASSERS][i] = ((self.s.c[PASSERS][i] as i32) | bit) as f64;
                                here |= bit;
                            }
                            break;
                        }
                    }
                    if weak && (self.on(LATCHED, i) || pushed || kicked || eased || (ow && self.s.c[PULL_T][i] > 0.0)) {
                        let k = pen / js_max(dist, 1e-4);
                        self.io.con[K_X][c] += qx * k;
                        self.io.con[K_Z][c] += qz * k;
                        break;
                    }
                    // (DOORTHROW) a leaf somebody else drove open toward him
                    let into = side_of(perp);
                    let dr = thr && !weak && self.on(DRIVE, i);
                    let toward = dr && self.s.c[DRIVE_WHO][i] != who && self.s.c[DRIVE_DIR][i] == into;
                    let at_him = toward && !self.on(DRIVE_QUIET, i) && (self.s.c[DRIVE_KIND][i] == BOOT || self.s.c[DRIVE_SIDE][i] == 0.0 || self.s.c[DRIVE_SIDE][i] != (man & MAN_SIDE) as f64);
                    let met = dr && self.s.c[DRIVE_HIT][i] == who && self.s.c[DRIVE_T][i] < self.io.par[P_AGAIN];
                    if dr && toward && !self.on(CLOSING, i) && along > 0.14 && along < wd + r {
                        let vin = into * self.s.c[VEL][i] * t;
                        let kind = self.s.c[DRIVE_KIND][i];
                        let tier = if kind == BOOT {
                            T_BOOT
                        } else if kicked || self.f & F_SOFT == 0 {
                            T_SLAM
                        } else {
                            T_OPEN
                        };
                        let held = !kicked && (kind == BODY || kind == HAND);
                        let fresh = held && self.s.c[DRIVE_T][i] < DRIVEN_SECS;
                        let stands = !folded && man & (if tier == T_OPEN { MAN_HELD } else { MAN_BRACED }) == 0;
                        let min_speed = self.io.par[P_MIN_SPEED];
                        if at_him && self.s.c[DRIVE_HIT][i] != who && stands && (if held { fresh && self.s.c[DRIVE_RATE][i] * t >= min_speed } else { vin >= min_speed }) {
                            let n = if dist > 1e-4 { 1.0 / dist } else { 0.0 };
                            let con = &mut self.io.con;
                            con[K_STRUCK][c] = i as f64;
                            con[K_STRUCK_BY][c] = self.s.c[DRIVE_WHO][i];
                            con[K_STRUCK_TIER][c] = tier;
                            con[K_STRUCK_FLOOR][c] = (tier == T_BOOT && js_abs(self.s.c[ANGLE][i] - self.s.c[DRIVE_FROM][i]) < self.io.par[P_FLOOR_RAD]) as u32 as f64;
                            if n != 0.0 && n == n {
                                con[K_STRUCK_DX][c] = qx * n;
                                con[K_STRUCK_DZ][c] = qz * n;
                            } else {
                                con[K_STRUCK_DX][c] = -u.1 * into;
                                con[K_STRUCK_DZ][c] = u.0 * into;
                            }
                            self.s.c[DRIVE_HIT][i] = who;
                            if self.carry(c, i, pen, dist, qx, qz, perp) {
                                break;
                            }
                        } else if (folded || met) && (vin >= 0.2 || fresh) && self.carry(c, i, pen, dist, qx, qz, perp) {
                            break;
                        }
                    }
                    if weak {
                        body_block = true;
                    } else {
                        touched = true;
                        if key || !key_touched {
                            self.s.c[LAST_WHO][i] = who;
                        }
                        if key {
                            key_touched = true;
                        }
                    }
                    if !weak && !self.opens(c, i) && self.on(LOCKED, i) && self.on(LATCHED, i) && side_of(perp) == self.io.def[D_KEY_SIDE][i] && along > 0.14 && along < wd + r {
                        let k = shove(pen, dt, true) / js_max(dist, 1e-4);
                        self.io.con[K_X][c] += qx * k;
                        self.io.con[K_Z][c] += qz * k;
                        self.io.con[K_BLOCKED_BY][c] = i as f64;
                        if self.on(CREAK_ARMED, i) && pen > 0.015 {
                            self.s.c[CREAK_ARMED][i] = 0.0;
                            self.rattle(i, who, if quiet { 0.08 } else { 0.12 });
                        }
                        break;
                    }
                    let limp = folded || met;
                    let pulling = key && !limp && !bash && self.on(CLOSING, i) && js_abs(self.s.c[ANGLE][i]) > 0.35;
                    let has_hand = kf & KF_HAND != 0;
                    let has_run = kf & KF_RUN != 0;
                    if along > 0.14 && along < wd + r {
                        let sgn = if perp >= 0.0 { -1.0 } else { 1.0 };
                        let lever = js_max(0.18, t);
                        let wide = js_min(1.35, 0.8 * js_abs(if sw > 0.0 { self.max_a(i) } else { self.min_a(i) }));
                        let presses = ow && !weak && !limp && sgn * sw < 0.0 && self.s.c[ANGLE][i] * sw < wide && self.s.c[OPEN_T][i] >= DRIVEN_SECS && self.s.c[VEL][i] * sw <= DRIVEN_VEL;
                        if presses {
                            pressed = true;
                            let hauled = bash && !has_hand && self.s.c[SINCE_BASH][i] > 0.9;
                            if hauled || self.s.c[PRESS_T][i] >= (if has_hand { self.io.con[K_HAND][c] } else { PRESS_SECS }) {
                                if hauled {
                                    self.s.c[LATCHED][i] = 0.0;
                                    self.s.c[VEL][i] = sw * (if has_run { self.io.con[K_RUN_RIP][c] } else { 7.5 });
                                    self.s.c[SINCE_BASH][i] = 0.0;
                                    if has_run {
                                        self.io.con[K_RIPPED][c] = i as f64;
                                    }
                                    self.emit(i, S_BASH, who, if has_run { self.io.con[K_RUN_LEVEL][c] } else { 1.0 });
                                } else {
                                    if self.on(LATCHED, i) || self.rests_to(i) {
                                        self.s.c[LATCHED][i] = 0.0;
                                        if self.on(CREAK_ARMED, i) {
                                            self.emit(i, S_CREAK, who, if quiet { 0.12 } else { 0.3 });
                                            self.s.c[CREAK_ARMED][i] = 0.0;
                                        }
                                    }
                                    self.s.c[VEL][i] = sw * js_max(self.s.c[VEL][i] * sw, if quiet { PULL_VEL_QUIET } else { PULL_VEL });
                                }
                                self.s.c[CLOSING][i] = 0.0;
                                self.s.c[SWEPT][i] = 0.0;
                                self.s.c[PULL_T][i] = PULL_SECS;
                                touched = true;
                                pushed = true;
                                if key || !key_touched {
                                    self.s.c[LAST_WHO][i] = who;
                                }
                                if key {
                                    key_touched = true;
                                }
                                if bit != 0 && !(solid && self.s.c[SOLID_T][i] <= SOLID_TRIES - 1.0) {
                                    self.s.c[PASSERS][i] = ((self.s.c[PASSERS][i] as i32) | bit) as f64;
                                    here |= bit;
                                }
                                break;
                            }
                        }
                        // (DoorContact.round) he goes round a leaf standing wide open
                        if kf & KF_ROUND != 0
                            && ow
                            && self.f & F_STAY != 0
                            && !bash
                            && !self.on(LATCHED, i)
                            && !self.on(CLOSING, i)
                            && sgn * sw < 0.0
                            && self.s.c[ANGLE][i] * sw >= wide
                            && js_abs(self.s.c[VEL][i]) < ROUND_STILL
                            && self.s.c[OPEN_T][i] >= DRIVEN_SECS
                        {
                            if !pressed && self.s.c[PRESS_T][i] == 0.0 {
                                self.io.par[P_ROUNDS] += 1.0;
                            }
                            pressed = true;
                            if self.s.c[PRESS_T][i] < ROUND_SECS {
                                self.s.c[VEL][i] *= 0.5;
                                let k = shove(pen, dt, true) / js_max(dist, 1e-4);
                                let sl = ROUND_SLIDE * dt;
                                self.io.con[K_X][c] += qx * k + u.0 * sl;
                                self.io.con[K_Z][c] += qz * k + u.1 * sl;
                                break;
                            }
                        }
                        let max_turn = TURN_RATE * dt;
                        let raw = sgn * (pen / lever) * 1.05;
                        let capped = js_abs(raw) > max_turn;
                        let want = self.s.c[ANGLE][i] + (if capped { js_sign(raw) * max_turn } else { raw });
                        if pulling && js_abs(want) > js_abs(self.s.c[ANGLE][i]) {
                            if bit != 0 {
                                self.s.c[PASSERS][i] = ((self.s.c[PASSERS][i] as i32) | bit) as f64;
                                here |= bit;
                            }
                            yank = true;
                            break;
                        }
                        let against = push_dir != 0.0 && sgn != push_dir;
                        if thr && !weak && !key && against && !folded && self.opening(i, sgn) {
                            self.walked(i, c, sgn, lever);
                            walker = c;
                        }
                        if !weak && !key && against {
                            let k = shove(pen, dt, false) / js_max(dist, 1e-4);
                            self.io.con[K_X][c] += qx * k;
                            self.io.con[K_Z][c] += qz * k;
                            break;
                        }
                        let na = clamp(want, self.min_a(i), self.max_a(i));
                        let moved = na - self.s.c[ANGLE][i];
                        if js_abs(moved) > 1e-5 {
                            let to = self.rests_to(i);
                            if thr && !weak && !folded && self.opening(i, js_sign(moved)) && !(self.on(DRIVE, i) && self.s.c[DRIVE_WHO][i] != who && self.s.c[DRIVE_T][i] == 0.0 && self.s.c[DRIVE_KIND][i] != BODY) {
                                self.walked(i, c, moved, lever);
                                walker = c;
                            }
                            if !weak {
                                pushed = true;
                                if key {
                                    push_dir = sgn;
                                }
                                if ow && moved * sw > 0.0 {
                                    self.s.c[OPEN_T][i] = 0.0;
                                }
                                self.s.c[SWEPT][i] = (key && !bash && self.f & F_STAY != 0 && js_abs(na) < js_abs(self.s.c[ANGLE][i])) as u32 as f64;
                            }
                            if self.on(LATCHED, i) || to {
                                self.s.c[LATCHED][i] = 0.0;
                                if self.on(CREAK_ARMED, i) && !bash {
                                    self.emit(i, S_CREAK, who, if quiet { 0.12 } else { 0.3 });
                                    self.s.c[CREAK_ARMED][i] = 0.0;
                                }
                            }
                            if bash && !(presses && has_hand) && self.s.c[SINCE_BASH][i] > 0.9 && js_abs(self.s.c[ANGLE][i]) < 1.2 {
                                let open = has_run && (!ow || sgn * sw > 0.0);
                                self.s.c[VEL][i] = sgn * (if open { self.io.con[K_RUN_BASH][c] } else { 7.5 });
                                self.s.c[SINCE_BASH][i] = 0.0;
                                if open {
                                    self.io.con[K_BASHED][c] = i as f64;
                                    self.io.con[K_BASHED_AT][c] = self.s.c[ANGLE][i];
                                    ran = true;
                                }
                                self.emit(i, S_BASH, who, 1.0);
                            } else if !kicked && !ran {
                                self.s.c[VEL][i] = if weak { self.s.c[VEL][i] * 0.5 } else { self.s.c[VEL][i] * 0.5 + (moved / js_max(dt, 1e-3)) * 0.5 };
                            }
                            self.s.c[ANGLE][i] = na;
                        }
                        if js_abs(want - na) > 1e-5 && (na == 0.0 || pen > r - STOP_BODY) {
                            let k = shove(if na == 0.0 { pen } else { pen - js_max(0.0, r - STOP_BODY) }, dt, false) / js_max(dist, 1e-4);
                            self.io.con[K_X][c] += qx * k;
                            self.io.con[K_Z][c] += qz * k;
                            if key && !limp && !bash && !presses && !(ow && na == 0.0) {
                                if self.f & F_STAY == 0 {
                                    yank = true;
                                } else if bit != 0 && !self.on(LATCHED, i) && js_abs(self.s.c[ANGLE][i]) > 0.5 {
                                    self.s.c[PASSERS][i] = ((self.s.c[PASSERS][i] as i32) | bit) as f64;
                                    here |= bit;
                                }
                            }
                        }
                        if capped {
                            break;
                        }
                    } else {
                        // the hinge region, or beyond the tip: a post
                        if pulling {
                            if bit != 0 {
                                self.s.c[PASSERS][i] = ((self.s.c[PASSERS][i] as i32) | bit) as f64;
                                here |= bit;
                            }
                            yank = true;
                            break;
                        }
                        let k = shove(pen, dt, false) / js_max(dist, 1e-4);
                        self.io.con[K_X][c] += qx * k;
                        self.io.con[K_Z][c] += qz * k;
                        if key && !limp && !bash && along >= wd && !self.on(LATCHED, i) && js_abs(self.s.c[ANGLE][i]) > 0.5 {
                            if self.f & F_STAY == 0 {
                                yank = true;
                            } else if bit != 0 {
                                self.s.c[PASSERS][i] = ((self.s.c[PASSERS][i] as i32) | bit) as f64;
                                here |= bit;
                            }
                        }
                        break;
                    }
                }
            }
        }
        self.s.c[SINCE_BASH][i] += dt;
        self.s.c[PASSERS][i] = ((self.s.c[PASSERS][i] as i32) & here) as f64;
        if ow {
            self.s.c[PRESS_T][i] = if pressed { self.s.c[PRESS_T][i] + dt } else { 0.0 };
            self.s.c[OPEN_T][i] += dt;
            if self.s.c[SOLID_T][i] > 0.0 {
                self.s.c[SOLID_T][i] = js_max(0.0, self.s.c[SOLID_T][i] - SOLID_FADE * dt);
            }
        }
        if yank {
            self.s.c[CLOSING][i] = 1.0;
            if js_sign(self.s.c[ANGLE][i]) * self.s.c[VEL][i] > -3.0 {
                self.s.c[VEL][i] = -js_sign(self.s.c[ANGLE][i]) * 3.0;
            }
            self.s.c[QUIET_T][i] = 0.0;
        } else if touched {
            self.s.c[QUIET_T][i] = 0.0;
            self.s.c[CLOSING][i] = 0.0;
        } else if !body_block {
            self.s.c[QUIET_T][i] += dt;
        }
        // free swing with friction, limit stops
        if (!touched || kicked || yank) && !self.on(LATCHED, i) {
            if !kicked {
                self.s.c[ANGLE][i] += self.s.c[VEL][i] * dt;
            }
            self.s.c[VEL][i] *= exp(-2.2 * dt);
            let (lo, hi) = (self.min_a(i), self.max_a(i));
            if self.s.c[ANGLE][i] >= hi || self.s.c[ANGLE][i] <= lo {
                let lim = if self.s.c[ANGLE][i] >= hi { hi } else { lo };
                self.s.c[ANGLE][i] = lim;
                let vel = self.s.c[VEL][i];
                if ow && lim == 0.0 {
                    if vel * sw < -2.5 {
                        self.emit(i, S_BANG, self.s.c[LAST_WHO][i], js_min(1.0, js_abs(vel) / 8.0));
                    }
                    if vel * sw < 0.0 {
                        self.s.c[VEL][i] = 0.0;
                    }
                } else {
                    if js_abs(vel) > 2.5 {
                        self.emit(i, S_BANG, self.s.c[LAST_WHO][i], js_min(1.0, js_abs(vel) / 8.0));
                    }
                    self.s.c[VEL][i] = if js_sign(lim) * vel > 0.0 { -vel * 0.25 } else { vel };
                }
            }
            let closing = self.on(CLOSING, i);
            if ((self.has_closer(i) && self.s.c[QUIET_T][i] > 2.0) || closing) && !body_block {
                let k = if closing { 5.0 } else { 3.0 };
                let acc = -self.s.c[ANGLE][i] * k - self.s.c[VEL][i] * (if closing { 4.2 } else { 3.2 });
                self.s.c[VEL][i] += acc * dt;
            }
            if body_block {
                self.s.c[VEL][i] *= exp(-12.0 * dt);
            }
            if js_abs(self.s.c[ANGLE][i]) < 0.012 && js_abs(self.s.c[VEL][i]) < 0.35 && self.s.c[QUIET_T][i] > 0.3 && self.s.c[PASSERS][i] == 0.0 {
                if self.on(SWEPT, i) && !self.on(CLOSING, i) && self.f & F_STAY != 0 {
                    self.s.c[ANGLE][i] = 0.0;
                    self.s.c[VEL][i] = 0.0;
                    self.s.c[CREAK_ARMED][i] = 1.0;
                } else {
                    self.s.c[ANGLE][i] = 0.0;
                    self.s.c[VEL][i] = 0.0;
                    self.s.c[LATCHED][i] = 1.0;
                    self.s.c[CREAK_ARMED][i] = 1.0;
                    self.s.c[CLOSING][i] = 0.0;
                    self.s.c[SWEPT][i] = 0.0;
                    self.s.c[PULL_T][i] = 0.0;
                    self.emit(i, S_LATCH, self.s.c[LAST_WHO][i], 0.16);
                }
            }
        }
        if self.s.c[QUIET_T][i] > 1.2 {
            self.s.c[CREAK_ARMED][i] = 1.0;
        }
        // the lock
        if self.on(PICKING, i) {
            self.s.c[PICKING][i] = 0.0;
            self.s.c[PICK_IDLE][i] = 0.0;
        } else if self.s.c[PICK][i] > 0.0 {
            self.s.c[PICK_IDLE][i] += dt;
            if self.s.c[PICK_IDLE][i] > 2.0 {
                self.s.c[PICK][i] = js_max(0.0, self.s.c[PICK][i] - dt / 4.0);
            }
        }
        if self.s.c[RATTLE_T][i] > 0.0 {
            self.s.c[RATTLE_T][i] = js_max(0.0, self.s.c[RATTLE_T][i] - dt);
        }
        if self.s.c[PULL_T][i] > 0.0 {
            self.s.c[PULL_T][i] = js_max(0.0, self.s.c[PULL_T][i] - dt);
        }
        if self.on(DRIVE, i) {
            self.s.c[DRIVE_T][i] += dt;
            if walker != usize::MAX && self.s.c[DRIVE_KIND][i] == BODY && self.s.c[DRIVE_WHO][i] == self.io.con[K_WHO][walker] {
                self.s.c[DRIVE_PX][i] = self.io.con[K_X][walker];
                self.s.c[DRIVE_PZ][i] = self.io.con[K_Z][walker];
            }
        }
        self.place(i);
        if self.s.c[ANGLE][i] != angle0 {
            self.io.par[P_TURNED] += 1.0;
        }
    }
    /// what the game knows of the man who last had door i (handed in; a step that asks where the host saw no rule that could is wrong, and says so)
    fn man_last(&mut self, i: usize) -> i32 {
        let m = self.io.def[D_MAN_LAST][i];
        if m == D_UNASKED {
            self.io.par[P_WRONG] = 2.0;
            return 0;
        }
        m as i32
    }
    /// the step-back's test (Doors.update `free`): do the statics let him stand at (x, z)
    fn free(&self, x: f64, z: f64) -> bool {
        if self.f & F_SETTLE == 0 {
            return true;
        }
        let (mut qx, mut qz) = (x, z);
        self.settle(&mut qx, &mut qz);
        hypot2(qx - x, qz - z) < 0.05
    }
}

/// doors.ts shove: the correction a leaf applies to a living contact this frame
#[inline]
fn shove(pen: f64, dt: f64, exact: bool) -> f64 {
    if exact {
        pen + 0.001
    } else {
        js_min(pen + 0.001, SHOVE_RATE * dt)
    }
}

/// Doors.update for the game in slot `g`, over world `w`: the hand-over read, every door stepped and placed, the answers written. Returns how many noises
/// were listed.
pub fn update(g: usize, w: usize) -> u32 {
    let io = io();
    let (n, nc) = (io.par[P_DOORS] as usize, io.par[P_CONTACTS] as usize);
    let (f, dt) = (io.par[P_FLAGS] as u32, io.par[P_DT]);
    let mut k = P_EVENTS;
    while k < NPAR {
        io.par[k] = 0.0;
        k += 1;
    }
    if n > CAP || nc > MAXC {
        io.par[P_WRONG] = 3.0;
        return 0;
    }
    for i in 0..n {
        io.out[O_PLACED][i] = 0.0;
    }
    let mut st = Step { s: &mut state(g).doors, io, w, f, dt, n, nc, ev: 0 };
    for i in 0..n {
        st.door(i);
    }
    st.io.par[P_EVENTS] = st.ev as f64;
    st.ev as u32
}
