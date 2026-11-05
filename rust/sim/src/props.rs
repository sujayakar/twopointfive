//! THE FURNITURE MEN SHOVE (src/game/props.ts PropSystem.update) — every chair, carton, bin and plant of a level for one tick: it coasts and comes to rest,
//! the men walking into it set it going and are held back by what cannot yield, it knocks into its neighbours, a swinging leaf sweeps it aside, a door frame
//! guides it through, the statics stop it; and the scrapes and knocks all that makes.
//!
//! STATEMENT FOR STATEMENT, as doors.rs is: the same things in the same order, the same substeps and passes, every sum in the order the TypeScript writes
//! it. tools/qa/prop-ident.ts and the missions of tools/qa/rust-check.ts hold the two together, column for column.
//!
//! WHAT IS STATE, AND WHAT IS HANDED IN. A thing's own numbers (where it stands and how it turns, how fast, whether it sleeps, who moved it last, its clocks,
//! and the cos and sin of its yaw) are the `props` part of the game's block (state.rs), LAID OUT FOR THE KERNELS TO COME (the maintainer, 2026-11-03: "we
//! should also start changing our data layout to be more cache and simd friendly"):
//!   · ROWS ARE DENSE AND IN THE GAME'S OWN ORDER: rows 0 .. n are the things in the level now, in the order the TypeScript's list keeps them (a level's
//!     own, a thing a blast took out gone from it), and the step walks 0 .. n with no test and no table. A thing is named from outside by its id, which
//!     does not move: `row[id]` says where it is, `id[row]` who is there. A THING THAT LEAVES (PropSystem.remove) is moved behind the live rows, those
//!     after it closing up in order; a level put back (reset) is every row its own id again. Both are the host's (src/sim/propshome.ts PropStore): no
//!     rule here takes a thing out.
//!   · A COLUMN A NUMBER, AS NARROW AS IT IS: the lengths, speeds, angles and clocks are doubles (the rules are worked in doubles); the three truths are
//!     bits of ONE u32 (`flags`); who moved it last is an i32; the two tables u32. Each column starts on a 16-byte boundary and is CAP rows long.
//!   · HOT BEFORE COLD within a width: x z yaw vx vz w cos sin are read for every row on every tick and lie together; idle and the two clocks are read
//!     when a thing is awake or makes a noise. The cos and sin are a column each BESIDE the yaw and are written where the yaw is written, whenever it
//!     is written another value (so a kernel never asks whether they are fresh).
//!   104 bytes a row (eleven doubles and four words); lanes a kernel could run today: two of doubles over the hot columns (the coast and its friction are one statement a row).
//! What the step needs of the rest of the game is an ARGUMENT, written by the host for the one call and kept nowhere (`IO`, scratch): the level's facts
//! of each thing BY ROW (`def`), the men (`con`), which of the box world's leaves may sweep them (`leaf`: a leaf off its hinges is none), the door
//! frames (`way`), the switches and numbers (`par`). What comes back the same way: where each man ends, and the noises in the order they were made (`ev`).
//!
//! NO BOX OF THE BOX WORLD IS WRITTEN HERE. Furniture is not in it (nothing sees, hears or walks by a chair: props.ts says why), so the boxes the picture
//! draws are placed by the host from these columns after the call (Prop.place), as they always were. The statics a thing rests against and the leaves that
//! sweep it are READ from the world's copy (world.rs, collide.rs gather).
use crate::collide;
use crate::maths::{cos, exp, hypot2, sin};
use crate::state::state;
use crate::world::{js_abs, js_ceil, js_max, js_min, world, MAX_LEAVES, REC, R_C, R_COS, R_H, R_SIN};
use core::mem::{offset_of, size_of};
use prim::sqrt;

/// things a level can have and still keep them here (more: the TypeScript keeps them, as it did). A multiple of 4: every column is whole vectors.
pub const CAP: usize = 128;
const _: () = assert!(CAP % 4 == 0);

// the truths of a thing (`flags`)
pub const F_ASLEEP: u32 = 1;
pub const F_DIRTY: u32 = 2;
pub const F_ROLLING: u32 = 4;

/// The furniture's part of a game's block (src/sim/propshome.ts PropStore lays its own array out the same, byte for byte; tools/qa/rust-check.ts holds
/// every offset the TypeScript works out to the one this struct has).
#[repr(C)]
pub struct Props {
    /// the system's clock (PropSystem.time)
    pub time: f64,
    /// rows in the level now (0 .. n are walked), and things of the level in all (n .. total: the ones a blast took out, kept for the level put back)
    pub n: u32,
    pub total: u32,
    // ---- doubles, the hot ones first: read for every row every tick
    pub x: [f64; CAP],
    pub z: [f64; CAP],
    pub yaw: [f64; CAP],
    pub vx: [f64; CAP],
    pub vz: [f64; CAP],
    pub w: [f64; CAP],
    /// the cos and sin of `yaw`, written where the yaw is written
    pub cs: [f64; CAP],
    pub sn: [f64; CAP],
    // ---- doubles an awake thing or a noise reads
    pub idle: [f64; CAP],
    pub push_t: [f64; CAP],
    pub noise_t: [f64; CAP],
    // ---- words
    pub flags: [u32; CAP],
    pub who: [i32; CAP],
    /// who is in a row, and where a thing is
    pub id: [u32; CAP],
    pub row: [u32; CAP],
}
const _: () = assert!(size_of::<Props>() == 16 + 11 * CAP * 8 + 4 * CAP * 4);
const _: () = assert!(offset_of!(Props, x) % 16 == 0 && offset_of!(Props, flags) % 16 == 0 && (CAP * 4) % 16 == 0);
/// the columns in their order (src/sim/propshome.ts PROP_F64 and PROP_U32 name them the same), for the host to hold its offsets to
pub const COLS: [usize; 15] = [
    offset_of!(Props, x),
    offset_of!(Props, z),
    offset_of!(Props, yaw),
    offset_of!(Props, vx),
    offset_of!(Props, vz),
    offset_of!(Props, w),
    offset_of!(Props, cs),
    offset_of!(Props, sn),
    offset_of!(Props, idle),
    offset_of!(Props, push_t),
    offset_of!(Props, noise_t),
    offset_of!(Props, flags),
    offset_of!(Props, who),
    offset_of!(Props, id),
    offset_of!(Props, row),
];

// ---- WHAT THE HOST HANDS IN AND TAKES BACK FOR ONE STEP (scratch: nothing here is kept from one call to the next)
/// men one step can carry (more: the TypeScript steps the furniture that tick)
pub const MAXC: usize = 256;
/// noises one step can list
pub const MAXE: usize = 1024;
/// door frames
pub const MAXW: usize = 256;

// what the level says of a thing (PropDef and what the Prop made of it), a column each, BY ROW (the host writes rows 0 .. n as they stand)
pub const D_HX: usize = 0;
pub const D_HZ: usize = 1;
pub const D_RADIUS: usize = 2;
pub const D_HEIGHT: usize = 3;
pub const D_MASS: usize = 4;
pub const D_INV_MASS: usize = 5;
pub const D_INV_I: usize = 6;
pub const D_DECEL: usize = 7;
pub const D_VISC: usize = 8;
pub const D_REST: usize = 9;
/// 1 = an office chair (the one kind that rolls on castors)
pub const D_CHAIR: usize = 10;
pub const NDEF: usize = 11;

// a man (props.ts PropContact), a column each: where he is (in, and out: where the furniture leaves him), how he moves, how big, who, how heavy, eased past
pub const K_X: usize = 0;
pub const K_Z: usize = 1;
pub const K_VX: usize = 2;
pub const K_VZ: usize = 3;
pub const K_RADIUS: usize = 4;
pub const K_WHO: usize = 5;
pub const K_MASS: usize = 6;
pub const K_QUIET: usize = 7;
pub const NCON: usize = 8;

// a door frame (PropSystem.doorways)
pub const Y_CX: usize = 0;
pub const Y_CZ: usize = 1;
pub const Y_UX: usize = 2;
pub const Y_UZ: usize = 3;
pub const Y_HALF: usize = 4;
pub const NWAY: usize = 5;

// a noise (props.ts PropEvent): the thing, which sound (src/sim/propshome.ts PROP_SOUNDS) and for a roll whether it starts (1) or stops (0), where, who is
// blamed, how loud and how fast, and whether the thing was rolling on its castors when it was made
pub const E_PROP: usize = 0;
pub const E_SOUND: usize = 1;
pub const E_ROLL: usize = 2;
pub const E_X: usize = 3;
pub const E_Y: usize = 4;
pub const E_Z: usize = 5;
pub const E_WHO: usize = 6;
pub const E_LEVEL: usize = 7;
pub const E_SPEED: usize = 8;
pub const E_ROLLING: usize = 9;
pub const NEV: usize = 10;
const SCRAPE: f64 = 0.0;
const THUD: f64 = 1.0;
const ROLL: f64 = 2.0;

// the step's numbers: in …
pub const P_DT: usize = 0;
pub const P_PROPS: usize = 1;
pub const P_CONTACTS: usize = 2;
pub const P_LEAVES: usize = 3;
pub const P_WAYS: usize = 4;
pub const P_FLAGS: usize = 5;
pub const P_FRICTION: usize = 6;
pub const P_CAS_DECEL: usize = 7;
pub const P_CAS_VISC: usize = 8;
pub const P_CAS_SPIN: usize = 9;
pub const P_CAS_START: usize = 10;
pub const P_CAS_STOP: usize = 11;
// … and out
pub const P_EVENTS: usize = 12;
/// something this step could not be: a noise past the list's end (1), more things, men, leaves or frames than its tables hold or another count of things than the part's (3)
pub const P_WRONG: usize = 13;
// counts for the checks (what this step met): leaves that swept a thing, frames that guided one, things that knocked each other, a thing against a static,
// men held back by a thing that could not yield, substeps
pub const P_SWEPT: usize = 14;
pub const P_GUIDED: usize = 15;
pub const P_PAIRS: usize = 16;
pub const P_WALLS: usize = 17;
pub const P_HELD: usize = 18;
pub const P_SUBSTEPS: usize = 19;
pub const NPAR: usize = 24;
// the switches (P_FLAGS)
pub const F_ENABLED: u32 = 1;
pub const F_CASTOR: u32 = 2;

#[repr(C)]
pub struct Io {
    pub def: [[f64; CAP]; NDEF],
    pub con: [[f64; MAXC]; NCON],
    pub leaf: [f64; MAX_LEAVES],
    pub way: [[f64; MAXW]; NWAY],
    pub ev: [[f64; NEV]; MAXE],
    pub par: [f64; NPAR],
}
static mut IO: Io = unsafe { core::mem::zeroed() };
#[inline(always)]
fn io() -> &'static mut Io {
    unsafe { &mut *core::ptr::addr_of_mut!(IO) }
}
/// where a table of the step's hand-over is (a byte address): 0 def · 2 con · 3 leaf · 4 way · 5 ev · 6 par
pub fn addr(table: u32) -> usize {
    let o = io();
    match table {
        0 => o.def.as_ptr() as usize,
        2 => o.con.as_ptr() as usize,
        3 => o.leaf.as_ptr() as usize,
        4 => o.way.as_ptr() as usize,
        5 => o.ev.as_ptr() as usize,
        6 => o.par.as_ptr() as usize,
        _ => 0,
    }
}
/// how much a table holds, by the same numbers (rows: things, men, leaves, frames, noises, numbers); 9 … 12 = how wide a thing's facts, a man, a frame
/// and a noise are; 13 = the part's length in bytes; 100 + k = where column k of the part starts (COLS)
pub fn cap(table: u32) -> usize {
    match table {
        0 => CAP,
        2 => MAXC,
        3 => MAX_LEAVES,
        4 => MAXW,
        5 => MAXE,
        6 => NPAR,
        9 => NDEF,
        10 => NCON,
        11 => NWAY,
        12 => NEV,
        13 => size_of::<Props>(),
        k if k >= 100 && ((k - 100) as usize) < COLS.len() => COLS[(k - 100) as usize],
        _ => 0,
    }
}

// ---- props.ts's own numbers (tools/qa/rust-check.ts reads both sources and is red where one differs)
const MAX_V: f64 = 5.0;
const MAX_W: f64 = 9.0;
const SLEEP_V: f64 = 0.04;
const SLEEP_W: f64 = 0.06;
const SLEEP_T: f64 = 0.3;
const DOOR_GUIDE: f64 = 1.2;
const Y_MIN: f64 = 0.04;
/// statics one gather of the TypeScript's can list (PropSystem.near)
const NEAR: usize = 256;
const PI: f64 = core::f64::consts::PI;

/// what two footprints do to each other (props.ts `sat`): the way out for B, how deep, and where they touch
#[derive(Clone, Copy)]
struct Sat {
    nx: f64,
    nz: f64,
    depth: f64,
    px: f64,
    pz: f64,
}
/// props.ts support: the farthest point of a footprint along (dx, dz), an edge that lies flat giving its middle
#[inline(always)]
#[allow(clippy::too_many_arguments)]
fn support(cx: f64, cz: f64, cs: f64, sn: f64, hx: f64, hz: f64, dx: f64, dz: f64) -> (f64, f64) {
    let (du, dv) = (dx * cs - dz * sn, dx * sn + dz * cs);
    let su = if du > 0.06 {
        hx
    } else if du < -0.06 {
        -hx
    } else {
        0.0
    };
    let sv = if dv > 0.06 {
        hz
    } else if dv < -0.06 {
        -hz
    } else {
        0.0
    };
    (cx + cs * su + sn * sv, cz - sn * su + cs * sv)
}
/// props.ts satRects: two turned rectangles on the floor, A and B
#[allow(clippy::too_many_arguments)]
fn sat_rects(ax: f64, az: f64, acs: f64, asn: f64, ahx: f64, ahz: f64, bx: f64, bz: f64, bcs: f64, bsn: f64, bhx: f64, bhz: f64) -> Option<Sat> {
    let (dx, dz) = (bx - ax, bz - az);
    let (cuu, cuv) = (js_abs(acs * bcs + asn * bsn), js_abs(acs * bsn - asn * bcs));
    let (mut best, mut bnx, mut bnz, mut from_a) = (1e9, 0.0, 0.0, true);
    // A.u
    let mut d = dx * acs - dz * asn;
    let mut ov = ahx + bhx * cuu + bhz * cuv - js_abs(d);
    if ov <= 0.0 {
        return None;
    }
    if ov < best {
        best = ov;
        let s = if d < 0.0 { -1.0 } else { 1.0 };
        bnx = acs * s;
        bnz = -asn * s;
        from_a = true;
    }
    // A.v
    d = dx * asn + dz * acs;
    ov = ahz + bhx * cuv + bhz * cuu - js_abs(d);
    if ov <= 0.0 {
        return None;
    }
    if ov < best {
        best = ov;
        let s = if d < 0.0 { -1.0 } else { 1.0 };
        bnx = asn * s;
        bnz = acs * s;
        from_a = true;
    }
    // B.u
    d = dx * bcs - dz * bsn;
    ov = bhx + ahx * cuu + ahz * cuv - js_abs(d);
    if ov <= 0.0 {
        return None;
    }
    if ov < best {
        best = ov;
        let s = if d < 0.0 { -1.0 } else { 1.0 };
        bnx = bcs * s;
        bnz = -bsn * s;
        from_a = false;
    }
    // B.v
    d = dx * bsn + dz * bcs;
    ov = bhz + ahx * cuv + ahz * cuu - js_abs(d);
    if ov <= 0.0 {
        return None;
    }
    if ov < best {
        best = ov;
        let s = if d < 0.0 { -1.0 } else { 1.0 };
        bnx = bsn * s;
        bnz = bcs * s;
        from_a = false;
    }
    let (px, pz) = if from_a { support(bx, bz, bcs, bsn, bhx, bhz, -bnx, -bnz) } else { support(ax, az, acs, asn, ahx, ahz, bnx, bnz) };
    Some(Sat { nx: bnx, nz: bnz, depth: best, px, pz })
}

/// a man's circle against a thing's footprint (props.ts `cc`): the way out for him, how deep, and the point on the thing
struct Touch {
    nx: f64,
    nz: f64,
    pen: f64,
    px: f64,
    pz: f64,
}

/// A thing's yaw written (Prop's `yaw` setter): its cos and sin are written with it, whenever it is another value than it was (a yaw that goes from −0 to
/// +0 is the same yaw, and keeps the sin it had: the TypeScript's own test, `!==`)
#[inline(always)]
fn turn(c: &mut Props, i: usize, yaw: f64) {
    if yaw != c.yaw[i] {
        c.cs[i] = cos(yaw);
        c.sn[i] = sin(yaw);
    }
    c.yaw[i] = yaw;
}

/// One step's view of the game: the things' state, the hand-over, which world, and the switches.
struct Step {
    s: &'static mut Props,
    io: &'static mut Io,
    w: usize,
    enabled: bool,
    castor: bool,
    n: usize,
    nc: usize,
    nl: usize,
    nw: usize,
    ev: usize,
    wrong: bool,
}

impl Step {
    #[inline(always)]
    fn is(&self, f: u32, i: usize) -> bool {
        self.s.flags[i] & f != 0
    }
    #[inline(always)]
    fn def(&self, c: usize, i: usize) -> f64 {
        self.io.def[c][i]
    }
    /// Prop.wake
    #[inline(always)]
    fn wake(&mut self, i: usize) {
        self.s.flags[i] &= !F_ASLEEP;
        self.s.idle[i] = 0.0;
    }
    /// Prop.speed
    #[inline(always)]
    fn speed(&self, i: usize) -> f64 {
        hypot2(self.s.vx[i], self.s.vz[i])
    }
    /// PropSystem.clampVel
    fn clamp_vel(&mut self, i: usize) {
        let c = &mut *self.s;
        let s = hypot2(c.vx[i], c.vz[i]);
        if s > MAX_V {
            c.vx[i] *= MAX_V / s;
            c.vz[i] *= MAX_V / s;
        }
        if c.w[i] > MAX_W {
            c.w[i] = MAX_W;
        } else if c.w[i] < -MAX_W {
            c.w[i] = -MAX_W;
        }
    }
    /// a noise onto the list (the host hears them after the call, in this order)
    #[allow(clippy::too_many_arguments)]
    fn emit(&mut self, i: usize, sound: f64, roll: f64, y: f64, who: i32, level: f64, speed: f64) {
        if self.ev >= MAXE {
            self.wrong = true;
            return;
        }
        let e = &mut self.io.ev[self.ev];
        e[E_PROP] = self.s.id[i] as f64;
        e[E_SOUND] = sound;
        e[E_ROLL] = roll;
        e[E_X] = self.s.x[i];
        e[E_Y] = y;
        e[E_Z] = self.s.z[i];
        e[E_WHO] = who as f64;
        e[E_LEVEL] = level;
        e[E_SPEED] = speed;
        e[E_ROLLING] = if self.s.flags[i] & F_ROLLING != 0 { 1.0 } else { 0.0 };
        self.ev += 1;
    }
    /// PropSystem.rollNote: a castor chair's coast, said once as it starts and once as it stops
    fn roll_note(&mut self, i: usize, speed: f64) {
        let who = self.s.who[i];
        if !self.is(F_ROLLING, i) && speed > self.io.par[P_CAS_START] {
            self.s.flags[i] |= F_ROLLING;
            self.emit(i, ROLL, 1.0, 0.05, who, 0.0, speed);
        } else if self.is(F_ROLLING, i) && speed < self.io.par[P_CAS_STOP] {
            self.s.flags[i] &= !F_ROLLING;
            self.emit(i, ROLL, 0.0, 0.05, who, 0.0, speed);
        }
    }
    /// PropSystem.noise: a scrape or a knock, no more often than a thing can make one
    fn noise(&mut self, i: usize, sound: f64, speed: f64, who: i32, level: f64) {
        let t = self.s.time;
        if t - self.s.noise_t[i] < (if sound == THUD { 0.35 } else { 0.8 }) {
            return;
        }
        self.s.noise_t[i] = t;
        let y = js_min(0.6, self.def(D_HEIGHT, i) * 0.5);
        self.emit(i, sound, 0.0, y, who, js_min(1.0, level), speed);
    }
    /// PropSystem.circleContact
    fn circle_contact(&mut self, i: usize, cx: f64, cz: f64, r: f64) -> Option<Touch> {
        let (cs, sn) = (self.s.cs[i], self.s.sn[i]);
        let (x, z, hx, hz) = (self.s.x[i], self.s.z[i], self.def(D_HX, i), self.def(D_HZ, i));
        let (dx, dz) = (cx - x, cz - z);
        let (lx, lz) = (cs * dx - sn * dz, sn * dx + cs * dz);
        let qx = if lx < -hx {
            -hx
        } else if lx > hx {
            hx
        } else {
            lx
        };
        let qz = if lz < -hz {
            -hz
        } else if lz > hz {
            hz
        } else {
            lz
        };
        let (ddx, ddz) = (lx - qx, lz - qz);
        let d2 = ddx * ddx + ddz * ddz;
        if d2 >= r * r {
            return None;
        }
        let (nlx, nlz, pen);
        if d2 > 1e-10 {
            let d = sqrt(d2);
            nlx = ddx / d;
            nlz = ddz / d;
            pen = r - d;
        } else {
            let (px, pz) = (hx - js_abs(lx), hz - js_abs(lz));
            if px < pz {
                nlx = if lx < 0.0 { -1.0 } else { 1.0 };
                nlz = 0.0;
                pen = px + r;
            } else {
                nlx = 0.0;
                nlz = if lz < 0.0 { -1.0 } else { 1.0 };
                pen = pz + r;
            }
        }
        Some(Touch { nx: cs * nlx + sn * nlz, nz: -sn * nlx + cs * nlz, pen, px: x + cs * qx + sn * qz, pz: z - sn * qx + cs * qz })
    }
    /// this thing's footprint against a rectangle A (a neighbour's, a leaf's, a static's): satRects(A, this)
    #[inline(always)]
    #[allow(clippy::too_many_arguments)]
    fn sat_with(&mut self, ax: f64, az: f64, acs: f64, asn: f64, ahx: f64, ahz: f64, i: usize) -> Option<Sat> {
        let (cs, sn) = (self.s.cs[i], self.s.sn[i]);
        sat_rects(ax, az, acs, asn, ahx, ahz, self.s.x[i], self.s.z[i], cs, sn, self.def(D_HX, i), self.def(D_HZ, i))
    }

    /// PropSystem.solvePairs(true)
    fn solve_pairs(&mut self) {
        let n = self.n;
        for a in 0..n {
            for b in a + 1..n {
                if self.is(F_ASLEEP, a) && self.is(F_ASLEEP, b) && !self.is(F_DIRTY, a) && !self.is(F_DIRTY, b) {
                    continue;
                }
                let (dx, dz) = (self.s.x[b] - self.s.x[a], self.s.z[b] - self.s.z[a]);
                let rr = self.def(D_RADIUS, a) + self.def(D_RADIUS, b);
                if dx * dx + dz * dz > rr * rr {
                    continue;
                }
                let (acs, asn) = (self.s.cs[a], self.s.sn[a]);
                let (ax, az, ahx, ahz) = (self.s.x[a], self.s.z[a], self.def(D_HX, a), self.def(D_HZ, a));
                let Some(sat) = self.sat_with(ax, az, acs, asn, ahx, ahz, b) else { continue };
                let (nx, nz) = (sat.nx, sat.nz);
                let (ima, imb, iia, iib) = (self.def(D_INV_MASS, a), self.def(D_INV_MASS, b), self.def(D_INV_I, a), self.def(D_INV_I, b));
                let corr = js_max(0.0, sat.depth - 0.003);
                let wa = ima / (ima + imb);
                {
                    let c = &mut *self.s;
                    c.x[a] -= nx * corr * wa;
                    c.z[a] -= nz * corr * wa;
                    c.x[b] += nx * corr * (1.0 - wa);
                    c.z[b] += nz * corr * (1.0 - wa);
                    c.flags[a] |= F_DIRTY;
                    c.flags[b] |= F_DIRTY;
                }
                let c = &*self.s;
                let (rax, raz, rbx, rbz) = (sat.px - c.x[a], sat.pz - c.z[a], sat.px - c.x[b], sat.pz - c.z[b]);
                let vn = ((c.vx[b] + c.w[b] * rbz) - (c.vx[a] + c.w[a] * raz)) * nx + ((c.vz[b] - c.w[b] * rbx) - (c.vz[a] - c.w[a] * rax)) * nz;
                if vn > -0.05 {
                    continue;
                }
                let vlin = (c.vx[b] - c.vx[a]) * nx + (c.vz[b] - c.vz[a]) * nz;
                let who = if self.is(F_ASLEEP, a) {
                    c.who[b]
                } else if self.is(F_ASLEEP, b) {
                    c.who[a]
                } else if self.speed(a) > self.speed(b) {
                    c.who[a]
                } else {
                    c.who[b]
                };
                self.io.par[P_PAIRS] += 1.0;
                let (ca, cb) = (raz * nx - rax * nz, rbz * nx - rbx * nz);
                let e = js_min(self.def(D_REST, a), self.def(D_REST, b));
                let jn = -(1.0 + e) * vn / (ima + imb + iia * ca * ca + iib * cb * cb);
                {
                    let c = &mut *self.s;
                    c.vx[a] -= jn * nx * ima;
                    c.vz[a] -= jn * nz * ima;
                    c.w[a] -= jn * ca * iia;
                    c.vx[b] += jn * nx * imb;
                    c.vz[b] += jn * nz * imb;
                    c.w[b] += jn * cb * iib;
                }
                self.clamp_vel(a);
                self.clamp_vel(b);
                self.s.who[a] = who;
                self.s.who[b] = who;
                self.wake(a);
                self.wake(b);
                if -vlin > 0.9 && self.s.time - js_max(self.s.push_t[a], self.s.push_t[b]) > 0.05 {
                    let lighter = if self.def(D_MASS, a) < self.def(D_MASS, b) { a } else { b };
                    self.noise(lighter, THUD, -vlin, who, 0.12 + 0.14 * -vlin);
                }
            }
        }
    }

    /// PropSystem.solveStatics(p, true): a thing pushed out of the statics round it, its turn and its speed answering the wall, and the knock
    fn solve_statics(&mut self, i: usize) {
        let (radius, height, im, ii, rest) = (self.def(D_RADIUS, i), self.def(D_HEIGHT, i), self.def(D_INV_MASS, i), self.def(D_INV_I, i), self.def(D_REST, i));
        for _ in 0..3 {
            let rr = radius + 0.02;
            let (x, z) = (self.s.x[i], self.s.z[i]);
            let n = collide::gather(self.w, x - rr, z - rr, x + rr, z + rr, Y_MIN, height, NEAR);
            let o = world(self.w);
            let mut any = false;
            for k in 0..n {
                let b = o.list[k] as usize * REC;
                // (props.ts works out the cos and sin of the static's yaw here; the copy keeps the same two beside the yaw, written by the same maths)
                let (bx, bz, bhx, bhz) = (o.rec[b + R_C], o.rec[b + R_C + 2], o.rec[b + R_H], o.rec[b + R_H + 2]);
                let (bcs, bsn) = (o.rec[b + R_COS], o.rec[b + R_SIN]);
                let (dx, dz) = (self.s.x[i] - bx, self.s.z[i] - bz);
                let (lx, lz) = (bcs * dx - bsn * dz, bsn * dx + bcs * dz);
                let (ex, ez) = (js_abs(lx) - bhx, js_abs(lz) - bhz);
                if ex > radius || ez > radius || (ex > 0.0 && ez > 0.0 && ex * ex + ez * ez > radius * radius) {
                    continue;
                }
                let Some(sat) = self.sat_with(bx, bz, bcs, bsn, bhx, bhz, i) else { continue };
                any = true;
                self.io.par[P_WALLS] += 1.0;
                let (nx, nz) = (sat.nx, sat.nz);
                let c = &mut *self.s;
                c.x[i] += nx * sat.depth;
                c.z[i] += nz * sat.depth;
                c.flags[i] |= F_DIRTY;
                let (rx, rz) = (sat.px - c.x[i], sat.pz - c.z[i]);
                let (vpx, vpz) = (c.vx[i] + c.w[i] * rz, c.vz[i] - c.w[i] * rx);
                let vn = vpx * nx + vpz * nz;
                if vn >= 0.0 {
                    continue;
                }
                let vlin = c.vx[i] * nx + c.vz[i] * nz;
                let cr = rz * nx - rx * nz;
                let kn = im + ii * cr * cr;
                let jn = -(1.0 + rest) * vn / kn;
                c.vx[i] += jn * nx * im;
                c.vz[i] += jn * nz * im;
                c.w[i] += jn * cr * ii;
                let (tx, tz) = (-nz, nx);
                let vt = vpx * tx + vpz * tz;
                if js_abs(vt) > 1e-4 {
                    let ct = rz * tx - rx * tz;
                    let kt = im + ii * ct * ct;
                    let mut jt = -vt / kt;
                    let lim = 0.4 * jn;
                    if jt > lim {
                        jt = lim;
                    } else if jt < -lim {
                        jt = -lim;
                    }
                    c.vx[i] += jt * tx * im;
                    c.vz[i] += jt * tz * im;
                    c.w[i] += jt * ct * ii;
                }
                self.clamp_vel(i);
                if -vlin > 0.8 && self.s.time - self.s.push_t[i] > 0.05 {
                    let who = self.s.who[i];
                    self.noise(i, THUD, -vlin, who, 0.1 + 0.15 * -vlin);
                }
            }
            if !any {
                break;
            }
        }
    }

    /// PropSystem.step: one substep of `h` seconds
    fn step(&mut self, h: f64) {
        let (n, nc) = (self.n, self.nc);
        if self.enabled {
            // 1. it coasts, the floor slows it, and it sleeps
            let fs = self.io.par[P_FRICTION];
            for i in 0..n {
                if self.is(F_ASLEEP, i) {
                    continue;
                }
                let cas = self.castor && self.def(D_CHAIR, i) != 0.0;
                let (dec, vis, spin) = if cas {
                    (self.io.par[P_CAS_DECEL], self.io.par[P_CAS_VISC], self.io.par[P_CAS_SPIN])
                } else {
                    (self.def(D_DECEL, i), self.def(D_VISC, i), 1.5 * self.def(D_DECEL, i))
                };
                let radius = self.def(D_RADIUS, i);
                let c = &mut *self.s;
                c.x[i] += c.vx[i] * h;
                c.z[i] += c.vz[i] * h;
                let yaw = c.yaw[i] + c.w[i] * h;
                turn(c, i, yaw);
                c.flags[i] |= F_DIRTY;
                let sp = hypot2(c.vx[i], c.vz[i]);
                if sp > 0.0 {
                    let ns = js_max(0.0, sp - dec * fs * h) * exp(-vis * fs * h);
                    c.vx[i] *= ns / sp;
                    c.vz[i] *= ns / sp;
                }
                let aw = js_abs(c.w[i]);
                if aw > 0.0 {
                    let na = js_max(0.0, aw - spin * fs / radius * h) * exp(-vis * fs * h);
                    c.w[i] *= na / aw;
                }
                if cas || self.is(F_ROLLING, i) {
                    let v = if cas { self.speed(i) } else { 0.0 };
                    self.roll_note(i, v);
                }
                let c = &mut *self.s;
                if sp < SLEEP_V && aw < SLEEP_W {
                    c.idle[i] += h;
                    if c.idle[i] > SLEEP_T {
                        c.flags[i] |= F_ASLEEP;
                        c.vx[i] = 0.0;
                        c.vz[i] = 0.0;
                        c.w[i] = 0.0;
                    }
                } else {
                    c.idle[i] = 0.0;
                }
                if c.yaw[i] > PI * 4.0 || c.yaw[i] < -PI * 4.0 {
                    let yaw = c.yaw[i] % (PI * 2.0);
                    turn(c, i, yaw);
                }
            }
        }
        // 2. the men push
        for q in 0..nc {
            let inv_mc = 1.0 / self.io.con[K_MASS][q];
            for i in 0..n {
                let (cx, cz, cr0) = (self.io.con[K_X][q], self.io.con[K_Z][q], self.io.con[K_RADIUS][q]);
                let (dx, dz) = (cx - self.s.x[i], cz - self.s.z[i]);
                let rr = self.def(D_RADIUS, i) + cr0;
                if dx * dx + dz * dz > rr * rr {
                    continue;
                }
                let Some(cc) = self.circle_contact(i, cx, cz, cr0) else { continue };
                let (nx, nz, pen) = (cc.nx, cc.nz, cc.pen);
                if !self.enabled {
                    self.io.con[K_X][q] += nx * pen;
                    self.io.con[K_Z][q] += nz * pen;
                    continue;
                }
                let t = self.s.time;
                let fresh = t - self.s.push_t[i] > 0.25;
                self.s.push_t[i] = t;
                self.wake(i);
                let who = self.io.con[K_WHO][q] as i32;
                let (im, ii) = (self.def(D_INV_MASS, i), self.def(D_INV_I, i));
                let c = &mut *self.s;
                c.who[i] = who;
                c.flags[i] |= F_DIRTY;
                let (rx, rz) = (cc.px - c.x[i], cc.pz - c.z[i]);
                let (vpx, vpz) = (c.vx[i] + c.w[i] * rz, c.vz[i] - c.w[i] * rx);
                let vn = (vpx - self.io.con[K_VX][q]) * nx + (vpz - self.io.con[K_VZ][q]) * nz;
                if vn > 0.0 {
                    let cr = rz * nx - rx * nz;
                    let kk = im + ii * cr * cr + inv_mc;
                    let jn = vn / kk;
                    c.vx[i] -= jn * nx * im;
                    c.vz[i] -= jn * nz * im;
                    c.w[i] -= jn * cr * ii;
                    self.clamp_vel(i);
                    let dv = jn * im;
                    if fresh && dv > 0.7 {
                        let quiet = self.io.con[K_QUIET][q] != 0.0;
                        self.noise(i, SCRAPE, dv, who, (0.28 * dv) * (if quiet { 0.5 } else { 1.0 }));
                    }
                }
                let share = im / (im + inv_mc);
                let c = &mut *self.s;
                c.x[i] -= nx * pen * share;
                c.z[i] -= nz * pen * share;
                self.io.con[K_X][q] += nx * pen * (1.0 - share);
                self.io.con[K_Z][q] += nz * pen * (1.0 - share);
            }
        }
        if self.enabled {
            // 3. thing against thing
            self.solve_pairs();
            // 4. the leaves sweep them aside
            let t = self.s.time;
            for l in 0..self.nl {
                let o = world(self.w);
                let b = self.io.leaf[l] as usize * REC;
                let (lx, lz, lhx, lhz) = (o.leaf[b + R_C], o.leaf[b + R_C + 2], o.leaf[b + R_H], o.leaf[b + R_H + 2]);
                let (lcs, lsn) = (o.leaf[b + R_COS], o.leaf[b + R_SIN]); // (kept beside the leaf's yaw by whoever wrote the row)
                let lr = hypot2(lhx, lhz);
                for i in 0..n {
                    let (dx, dz) = (self.s.x[i] - lx, self.s.z[i] - lz);
                    let reach = lr + self.def(D_RADIUS, i);
                    if dx * dx + dz * dz > reach * reach {
                        continue;
                    }
                    let Some(sat) = self.sat_with(lx, lz, lcs, lsn, lhx, lhz, i) else { continue };
                    self.io.par[P_SWEPT] += 1.0;
                    self.s.x[i] += sat.nx * sat.depth;
                    self.s.z[i] += sat.nz * sat.depth;
                    self.s.flags[i] |= F_DIRTY;
                    self.wake(i);
                    let c = &mut *self.s;
                    c.push_t[i] = t;
                    c.who[i] = 0;
                    let vn = c.vx[i] * sat.nx + c.vz[i] * sat.nz;
                    let want = js_min(2.0, sat.depth / h * 0.5);
                    if vn < want {
                        c.vx[i] += (want - vn) * sat.nx;
                        c.vz[i] += (want - vn) * sat.nz;
                    }
                }
            }
            // 4b. a door frame guides a shoved thing through
            for f in 0..self.nw {
                let (fcx, fcz, fux, fuz, half) = (self.io.way[Y_CX][f], self.io.way[Y_CZ][f], self.io.way[Y_UX][f], self.io.way[Y_UZ][f], self.io.way[Y_HALF][f]);
                for i in 0..n {
                    if self.is(F_ASLEEP, i) || t - self.s.push_t[i] > 0.12 || self.s.who[i] == 0 {
                        continue;
                    }
                    let (dx, dz) = (self.s.x[i] - fcx, self.s.z[i] - fcz);
                    let (radius, hx, hz) = (self.def(D_RADIUS, i), self.def(D_HX, i), self.def(D_HZ, i));
                    if dx * dx + dz * dz > (half + radius + 0.4) * (half + radius + 0.4) {
                        continue;
                    }
                    let (cs, sn) = (self.s.cs[i], self.s.sn[i]);
                    let (along, off) = (dx * fux + dz * fuz, -dx * fuz + dz * fux);
                    let eu = hx * js_abs(cs * fux - sn * fuz) + hz * js_abs(sn * fux + cs * fuz);
                    let en = hx * js_abs(-cs * fuz - sn * fux) + hz * js_abs(-sn * fuz + cs * fux);
                    let lim = half - eu - 0.01;
                    if lim < 0.0 || js_abs(along) > half + 0.05 || js_abs(off) > en + 0.15 {
                        continue;
                    }
                    let over = js_abs(along) - lim;
                    if over <= 0.0 {
                        continue;
                    }
                    let c = &mut *self.s;
                    if js_abs(-c.vx[i] * fuz + c.vz[i] * fux) < 0.5 * js_abs(c.vx[i] * fux + c.vz[i] * fuz) {
                        continue;
                    }
                    let kk = js_min(over, DOOR_GUIDE * h) * (if along > 0.0 { -1.0 } else { 1.0 });
                    c.x[i] += fux * kk;
                    c.z[i] += fuz * kk;
                    c.flags[i] |= F_DIRTY;
                    self.io.par[P_GUIDED] += 1.0;
                }
            }
            // 5. the statics, last of what moves a thing
            for i in 0..n {
                if !self.is(F_ASLEEP, i) || self.is(F_DIRTY, i) {
                    self.solve_statics(i);
                }
            }
        }
        // 6. what a thing could not yield the man gives back, and a thing found squeezed sheds what he keeps feeding it
        for q in 0..nc {
            for i in 0..n {
                let (cx, cz, cr0) = (self.io.con[K_X][q], self.io.con[K_Z][q], self.io.con[K_RADIUS][q]);
                let (dx, dz) = (cx - self.s.x[i], cz - self.s.z[i]);
                let rr = self.def(D_RADIUS, i) + cr0;
                if dx * dx + dz * dz > rr * rr {
                    continue;
                }
                let Some(cc) = self.circle_contact(i, cx, cz, cr0) else { continue };
                self.io.con[K_X][q] += cc.nx * cc.pen;
                self.io.con[K_Z][q] += cc.nz * cc.pen;
                if cc.pen > 0.004 {
                    self.io.par[P_HELD] += 1.0;
                    let c = &mut *self.s;
                    c.vx[i] *= 0.3;
                    c.vz[i] *= 0.3;
                    c.w[i] *= 0.3;
                }
            }
        }
    }
}

/// PropSystem.update for the game in slot `g`, over world `w`: the hand-over read, the substeps taken, the men's places and the noises written. Returns how
/// many noises were listed. (The boxes the picture draws are the host's to place from the columns: a thing whose F_DIRTY is up.)
pub fn update(g: usize, w: usize) -> u32 {
    let io = io();
    let (n, nc, nl, nw) = (io.par[P_PROPS] as usize, io.par[P_CONTACTS] as usize, io.par[P_LEAVES] as usize, io.par[P_WAYS] as usize);
    let (f, dt) = (io.par[P_FLAGS] as u32, io.par[P_DT]);
    let mut k = P_EVENTS;
    while k < NPAR {
        io.par[k] = 0.0;
        k += 1;
    }
    // (the rows walked are the part's own count: the host says how many it wrote facts for, and the two must be one number)
    let s = &mut state(g).props;
    if n != s.n as usize || n > CAP || s.total as usize > CAP || nc > MAXC || nl > MAX_LEAVES || nw > MAXW {
        io.par[P_WRONG] = 3.0;
        return 0;
    }
    for l in 0..nl {
        if !(io.leaf[l] >= 0.0 && io.leaf[l] < MAX_LEAVES as f64) {
            io.par[P_WRONG] = 3.0;
            return 0;
        }
    }
    let mut st = Step { s, io, w, enabled: f & F_ENABLED != 0, castor: f & F_CASTOR != 0, n, nc, nl, nw, ev: 0, wrong: false };
    // (≤ 1/60 s a substep: props.ts update)
    let steps = js_min(4.0, js_max(1.0, js_ceil(dt * 60.0 - 1e-6)));
    let h = dt / steps;
    let mut s = 0.0;
    while s < steps {
        st.io.par[P_SUBSTEPS] += 1.0;
        st.s.time += h;
        st.step(h);
        s += 1.0;
    }
    st.io.par[P_EVENTS] = st.ev as f64;
    if st.wrong {
        st.io.par[P_WRONG] = 1.0;
    }
    st.ev as u32
}
