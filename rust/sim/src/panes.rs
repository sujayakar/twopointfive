//! WHAT A PANE OF GLASS IS TO PLAY (src/game/glassbreak.ts, whose TypeScript is the caller and the picture): what a blow does to a pane, which pieces are held and
//! which fall and when, what a blast takes, when the floor hears it and is littered. Statement for statement with the rule it replaced: the same hashes, the
//! same maths (maths.rs), the same compares, in the same order. tools/qa/pane-ident.ts held the two together, every call both ways in one run.
//!
//! WHAT IS STATE. A pane's state is the `panes` part of the game's block (state.rs): a POOL, its first word the bytes of it in use. It is laid out dense, in
//! four runs closed up by their counts: the level's sheets, a row a pane that has been touched (in the order they were first touched: the rule's own, which
//! serves them a frame's budget in that order), the through-holes rounds have left (pen.rs), and the pieces queued to leave. So a level with no glass broken
//! carries a few kilobytes, and a snapshot of it no more. No address in it, no padding the compiler chose.
//!
//! A ROW IS WHAT THE LINES AND RAYS READ. Its first eighty doubles and sixty-four words are the row glass.rs has always answered by, and they ARE the state now:
//! a line or a ray meeting a pane (glass.rs `open`) reads them where they stand. What is left of a pane's state stands beside them.
//!
//! WHAT IS NOT HERE. What a fragment becomes (plates, spray, chips) is dressing: the rule lists each fragment it lets go (`OUT`), and the host hands them on when
//! the pane's call is back, a pane at a time. Nothing a fragment's handling does is read by the rest of that pane's rule. Its one question of the world, where it
//! comes down, is asked here at the moment it lets go, and its landing is scheduled here (falls.rs) under SHARDS.byFall.
//!
//! BOUNDS. Sheets and rows: a level of more sheets is refused as it loads (src/game/levelfit.ts), by name. Clumps of a sheet: the same (pattern.rs MAX_CLUMPS).
//! THE QUEUE: MAX_Q pieces waiting in a game; a piece past that stays in its frame for the next support pass. No run has come within a tenth of it.
use crate::falls;
use crate::glass::{classify_row, dice_out, site_of, GW_COLS, GW_CRATER, GW_DSEED, GW_FLAGS, GW_GOING, GW_N, GW_OPEN, GW_ROWS, GW_SEED, G_DICE, G_DICE_AT, G_GONE, G_RUN, G_SITE};
use crate::knobs::*;
use crate::lines::{self, ASK_GLASS};
use crate::maths::{hypot2, hypot3, round_half_up};
use crate::pattern::{self, clip_rect, frags_of, hull, order_of, poly_area, Dice, Frag, Pt, FRAGS_PER_SITE, MAX_PTS, REMAINDER};
use crate::state::state;
use crate::world::{js_abs, js_ceil, js_max, js_min, sign_or_one, world, World, GD, GW, REC, R_C, R_COS, R_H, R_SIN};
use core::mem::{offset_of, size_of};
use prim::{floor, sqrt};

pub const MAX_SHEETS: usize = 256;
pub const MAX_ROWS: usize = 256;
pub const MAX_HOLES: usize = 256;
pub const MAX_Q: usize = 4096;
/// the crossings one pane remembers (GLASSPEN.holes: its knob is under this)
pub const HOLE_PTS: usize = 24;
const MAX_SITES: usize = 8;
const MAX_SECTORS: usize = 12;
/// the glass kinds, in their order everywhere (the record the renderer reads says the same): plain float, toughened, laminated
pub const ANNEALED: u32 = 0;
pub const TEMPERED: u32 = 1;
pub const LAMINATED: u32 = 2;
/// what made a piece go
pub const C_HIT: u32 = 0;
pub const C_SUPPORT: u32 = 1;
pub const C_BLAST: u32 = 2;
pub const C_DICE: u32 = 3;
pub const C_RAKE: u32 = 4;
/// what a fragment is
pub const K_PIECE: f64 = 0.0;
pub const K_DAGGER: f64 = 1.0;
pub const K_CLUMP: f64 = 2.0;
pub const K_SHEET: f64 = 3.0;
const BLAST_MIN_K: f64 = 0.05;
const DICE_TIE: f64 = 1e-4;
const EXT_SKY_TOP: f64 = 12.0;
const THICK_GLASS_HALF: f64 = 0.05;
const RHO_TOP: f64 = 0.78 + 0.44;
const PI: f64 = core::f64::consts::PI;
const SKY_UNKNOWN: i32 = 2;
/// a sheet that has no word of its own on what glass it is (setType)
const NO_OVER: u32 = 3;
const D_U: usize = G_DICE_AT;
const D_V: usize = G_DICE_AT + 1;
const D_HU: usize = G_DICE_AT + 2;
const D_HV: usize = G_DICE_AT + 3;
const D_LAST: usize = G_DICE_AT + 4;
const D_LAST_K: usize = G_DICE_AT + 5;
/// (the eighty doubles' last two) when a pane's dice began
const D_T0: usize = G_DICE_AT + 6;
const S_U: usize = 0;
const S_V: usize = 1;
const S_ROT: usize = 2;
const S_RHO: usize = 3;
const S_RZ: usize = 7;
const S_MERGE: usize = 8;

/// A SHEET OF THE LEVEL: which static, which face of it has the sky (asked once, the first time it matters, of the level as it then stands), the word it has been
/// given on what glass it is, and how far it is on the plan from the nearest door's opening (the host's, worked out as the level is stood up)
#[repr(C)]
#[derive(Clone, Copy)]
pub struct Sheet {
    pub bi: u32,
    pub sky: i32,
    pub over: u32,
    pub spare: u32,
    pub gap: f64,
}
/// A PANE THAT HAS BEEN TOUCHED (glassbreak.ts PaneState)
#[repr(C)]
#[derive(Clone, Copy)]
pub struct Row {
    /// the sites (glass.rs: nine doubles each), the dice's blow, half sides and last clump out, when the dice began
    pub d: [f64; GD],
    /// the static, flags and site count, the dice's lattice and seed, each site's radials and seed, a bit a piece out, a bit a piece queued, a bit a crater queued
    pub w: [u32; GW],
    /// energy taken at each site, and blows
    pub e: [f64; MAX_SITES],
    pub hits: [f64; MAX_SITES],
    /// where it stands (read from its box as it is first touched: statics do not move), its half sides in its own frame
    pub c: [f64; 3],
    pub cs: f64,
    pub sn: f64,
    pub hu: f64,
    pub hv: f64,
    pub ht: f64,
    pub energy: f64,
    pub damage: f64,
    pub next_think: f64,
    pub last_dir: [f64; 3],
    pub last_uv: [f64; 2],
    pub sound_at: f64,
    pub who: f64,
    /// what the charges in hand have already done to it: which charge (its number, +1) and how hard, a few at a time
    pub bk: [f64; 2 * BK],
    /// the pane's own word (glass.rs boxSeed): every hash of its break starts here, none from its place in the level's list
    pub key: u32,
    pub ty: u32,
    pub u_axis: u32,
    pub exterior: u32,
    pub hit_n: u32,
    pub n_open: u32,
    pub d_next: u32,
    pub walkable: u32,
    pub q0: u32,
    pub qn: u32,
    pub bk_at: u32,
    pub spare: u32,
}
const BK: usize = 8;
/// THE THROUGH-HOLES OF ONE PANE (pen.rs: the crossings' points, the oldest first)
#[repr(C)]
#[derive(Clone, Copy)]
pub struct Hole {
    pub bi: u32,
    pub n: u32,
    pub p: [f64; 3 * HOLE_PTS],
}
/// A PIECE QUEUED TO LEAVE: when, where the blow came from (a blast's, a walker's), how hard, which piece, why. A pane's run is in order of when, then of which.
#[repr(C)]
#[derive(Clone, Copy)]
pub struct Q {
    pub at: f64,
    pub from: [f64; 3],
    pub k: f64,
    pub fid: u32,
    pub cause: u32,
}
const SHEET_W: usize = size_of::<Sheet>() / 8;
const ROW_W: usize = size_of::<Row>() / 8;
const HOLE_W: usize = size_of::<Hole>() / 8;
const Q_W: usize = size_of::<Q>() / 8;
const BODY: usize = MAX_SHEETS * SHEET_W + MAX_ROWS * ROW_W + MAX_HOLES * HOLE_W + MAX_Q * Q_W;
const EVENTS: usize = 32;
const CHARGES: usize = 16;
/// the head's flags: a pane on an outside wall has changed (the wind's gusts ask again)
pub const FL_GUST: u32 = 1;
/// the panes' part of a game's block: the head, then the four runs in `body`
#[repr(C)]
pub struct Panes {
    /// bytes of this part in use (state.rs: a pool's first word)
    pub used: u32,
    pub sheets: u32,
    pub n: u32,
    pub nh: u32,
    pub nq: u32,
    /// counts every change to the through-holes (glasspen.ts glassTollSig)
    pub rev: u32,
    /// pieces let go this frame (a frame's budget, across the panes)
    pub released: u32,
    pub flags: u32,
    /// the floor's last 'glass' event per pane per shooter (glass.ts liveGap): how many, and (pane × 1024 + who, when)
    pub nev: u32,
    pub nbl: u32,
    /// the number of the last charge seen
    pub seq: u32,
    pub spare: u32,
    pub ev: [f64; 2 * EVENTS],
    /// the charges in hand: when, where, and the charge's number
    pub bl: [f64; 5 * CHARGES],
    pub body: [u64; BODY],
}
const HEAD: usize = offset_of!(Panes, body);
const _: () = assert!(HEAD % 8 == 0);
const _: () = assert!(size_of::<Row>() % 8 == 0 && size_of::<Hole>() % 8 == 0 && size_of::<Q>() % 8 == 0 && size_of::<Sheet>() % 8 == 0);
const _: () = assert!(size_of::<Panes>() == HEAD + BODY * 8);

#[inline(always)]
fn pn(g: usize) -> &'static mut Panes {
    &mut state(g).panes
}
impl Panes {
    #[inline]
    fn mark(&mut self) {
        let words = self.sheets as usize * SHEET_W + self.n as usize * ROW_W + self.nh as usize * HOLE_W + self.nq as usize * Q_W;
        self.used = (HEAD + words * 8) as u32;
    }
    #[inline]
    fn rows_at(&self) -> usize {
        self.sheets as usize * SHEET_W
    }
    #[inline]
    fn holes_at(&self) -> usize {
        self.rows_at() + self.n as usize * ROW_W
    }
    #[inline]
    fn q_at(&self) -> usize {
        self.holes_at() + self.nh as usize * HOLE_W
    }
    #[inline]
    fn sheet(&mut self, s: usize) -> &mut Sheet {
        unsafe { &mut *(self.body.as_mut_ptr().add(s * SHEET_W) as *mut Sheet) }
    }
    #[inline]
    pub fn row(&mut self, r: usize) -> &mut Row {
        let at = self.rows_at() + r * ROW_W;
        unsafe { &mut *(self.body.as_mut_ptr().add(at) as *mut Row) }
    }
    #[inline]
    pub fn hole(&mut self, h: usize) -> &mut Hole {
        let at = self.holes_at() + h * HOLE_W;
        unsafe { &mut *(self.body.as_mut_ptr().add(at) as *mut Hole) }
    }
    #[inline]
    fn q(&mut self, i: usize) -> &mut Q {
        let at = self.q_at() + i * Q_W;
        unsafe { &mut *(self.body.as_mut_ptr().add(at) as *mut Q) }
    }
    /// room for `words` more at word `at` of the body: what stands from there on moves up
    fn open_up(&mut self, at: usize, words: usize, end: usize) {
        self.body.copy_within(at..end, at + words);
    }
    fn end(&self) -> usize {
        self.q_at() + self.nq as usize * Q_W
    }
    /// which row is static `bi`'s (none: it is whole, and has never been touched)
    #[inline]
    pub fn row_of(&self, bi: u32) -> Option<usize> {
        let (n, at) = (self.n as usize, self.rows_at());
        for r in 0..n {
            let w = unsafe { *(self.body.as_ptr().add(at + r * ROW_W + offset_of!(Row, w) / 8) as *const u32) };
            if w == bi {
                return Some(r);
            }
        }
        None
    }
    /// which of the level's sheets is static `bi` (they are listed in ascending order)
    #[inline]
    fn sheet_of(&mut self, bi: u32) -> Option<usize> {
        let (mut lo, mut hi) = (0usize, self.sheets as usize);
        while lo < hi {
            let m = (lo + hi) / 2;
            let v = self.sheet(m).bi;
            if v == bi {
                return Some(m);
            }
            if v < bi {
                lo = m + 1;
            } else {
                hi = m;
            }
        }
        None
    }
    /// a new row, the last: what stands after the rows moves up. Zeros but for its static.
    fn add_row(&mut self, bi: u32) -> usize {
        let (at, end) = (self.holes_at(), self.end());
        self.open_up(at, ROW_W, end);
        let r = self.n as usize;
        self.n += 1;
        self.mark();
        let row = self.row(r);
        *row = unsafe { core::mem::zeroed() };
        row.w[0] = bi;
        r
    }
    /// a new row of holes, the last: the queue moves up
    pub fn add_hole_row(&mut self, bi: u32) -> usize {
        let (at, end) = (self.q_at(), self.end());
        self.open_up(at, HOLE_W, end);
        let h = self.nh as usize;
        self.nh += 1;
        self.mark();
        let row = self.hole(h);
        *row = unsafe { core::mem::zeroed() };
        row.bi = bi;
        h
    }
    /// every hole gone (Game.resetLevelState)
    pub fn clear_holes(&mut self) {
        let (from, to) = (self.q_at(), self.holes_at());
        let end = self.end();
        self.body.copy_within(from..end, to);
        self.nh = 0;
        self.mark();
        self.rev = self.rev.wrapping_add(1);
    }
    /// a piece onto a pane's run, in its order (by when, then by which)
    fn q_put(&mut self, r: usize, e: Q) -> bool {
        if self.nq as usize >= MAX_Q {
            return false;
        }
        let (q0, qn) = {
            let row = self.row(r);
            (row.q0 as usize, row.qn as usize)
        };
        let mut i = qn;
        while i > 0 {
            let p = *self.q(q0 + i - 1);
            if p.at < e.at || (p.at == e.at && p.fid < e.fid) {
                break;
            }
            i -= 1;
        }
        let at = self.q_at() + (q0 + i) * Q_W;
        let end = self.end();
        self.open_up(at, Q_W, end);
        self.nq += 1;
        self.mark();
        *self.q(q0 + i) = e;
        for k in 0..self.n as usize {
            let o = self.row(k);
            if k == r {
                o.qn += 1;
            } else if k > r {
                o.q0 += 1;
            }
        }
        let row = self.row(r);
        row.w[GW_GOING + (e.fid >> 5) as usize] |= 1 << (e.fid & 31);
        if e.fid as usize % FRAGS_PER_SITE < MAX_SECTORS {
            row.w[GW_CRATER] |= 1 << (e.fid as usize / FRAGS_PER_SITE);
        }
        true
    }
    /// the first piece off a pane's run
    fn q_pop(&mut self, r: usize) -> Q {
        let q0 = self.row(r).q0 as usize;
        let e = *self.q(q0);
        let at = self.q_at() + q0 * Q_W;
        let end = self.end();
        self.body.copy_within(at + Q_W..end, at);
        self.nq -= 1;
        self.mark();
        for k in 0..self.n as usize {
            let o = self.row(k);
            if k == r {
                o.qn -= 1;
            } else if k > r {
                o.q0 -= 1;
            }
        }
        let row = self.row(r);
        row.w[GW_GOING + (e.fid >> 5) as usize] &= !(1 << (e.fid & 31));
        let site = e.fid as usize / FRAGS_PER_SITE;
        if e.fid as usize % FRAGS_PER_SITE < MAX_SECTORS {
            let qn = row.qn as usize;
            let mut still = false;
            for i in 0..qn {
                let f = self.q(q0 + i).fid as usize;
                if f / FRAGS_PER_SITE == site && f % FRAGS_PER_SITE < MAX_SECTORS {
                    still = true;
                    break;
                }
            }
            if !still {
                self.row(r).w[GW_CRATER] &= !(1 << site);
            }
        }
        e
    }
    /// static `bi`'s row and its queue gone (setType: the pane is made whole, and goes to the back of the order)
    fn drop_row(&mut self, r: usize) {
        let qn = self.row(r).qn as usize;
        for _ in 0..qn {
            self.q_pop(r);
        }
        let at = self.rows_at() + r * ROW_W;
        let end = self.end();
        self.body.copy_within(at + ROW_W..end, at);
        self.n -= 1;
        self.mark();
    }
}
/// sheet `bi`'s row, as a read for whoever holds the block
#[inline]
pub fn row_ref(g: usize, r: usize) -> &'static mut Row {
    pn(g).row(r)
}

// ---- the game's hashes (glass.ts boxSeed and pointSeed, debris.ts hash01)
/// the language's `x | 0` of a double
#[inline]
pub fn int32(x: f64) -> i32 {
    if x != x || x == f64::INFINITY || x == f64::NEG_INFINITY {
        return 0;
    }
    let t = prim::trunc(x);
    let m = t - 4294967296.0 * floor(t / 4294967296.0);
    (m as u32) as i32
}
#[inline]
fn seed_mix(h: u32, x: f64) -> u32 {
    let h2 = (h ^ (int32(round_half_up(x * 1000.0)) as u32)).wrapping_mul(0x85EB_CA77);
    h2 ^ (h2 >> 13)
}
/// glass.ts boxSeed: what a box is as one word, its centre and half sizes to the millimetre
pub fn box_seed(b: &[f64]) -> u32 {
    let mut h = 0x9E37_79B1u32;
    for k in 0..3 {
        h = seed_mix(h, b[R_C + k]);
    }
    for k in 0..3 {
        h = seed_mix(h, b[R_H + k]);
    }
    h.wrapping_mul(0xC2B2_AE3D)
}
/// glass.ts pointSeed
pub fn point_seed(word: u32, x: f64, y: f64, z: f64) -> u32 {
    let mut h = word;
    h = seed_mix(h, x);
    h = seed_mix(h, y);
    h = seed_mix(h, z);
    h.wrapping_mul(0xC2B2_AE3D)
}
/// glass.ts blastSeed: a blast's seed, its height left out
#[inline]
fn blast_seed(word: u32, at: &[f64; 3]) -> u32 {
    point_seed(word, at[0], 0.0, at[2])
}
/// debris.ts hash01, of a whole number however large a double holds it
#[inline]
pub fn hash01(x: f64) -> f64 {
    let mut h = (int32(x) as u32 ^ 0x9E37_79B9).wrapping_mul(0x85EB_CA6B);
    h ^= h >> 13;
    h = h.wrapping_mul(0xC2B2_AE35);
    h ^= h >> 16;
    (h >> 8) as f64 / 16777216.0
}

// ---- a pane's frame
#[inline]
fn to_uv(r: &Row, x: f64, y: f64, z: f64) -> (f64, f64) {
    let (px, pz) = (x - r.c[0], z - r.c[2]);
    let (lx, lz) = (r.cs * px - r.sn * pz, r.sn * px + r.cs * pz);
    (if r.u_axis == 0 { lx } else { lz }, y - r.c[1])
}
#[inline]
fn to_uv_in(r: &Row, x: f64, y: f64, z: f64) -> (f64, f64) {
    let (u, v) = to_uv(r, x, y, z);
    (js_max(-r.hu * 0.995, js_min(r.hu * 0.995, u)), js_max(-r.hv * 0.995, js_min(r.hv * 0.995, v)))
}
#[inline]
fn to_world(r: &Row, u: f64, v: f64) -> [f64; 3] {
    let (lx, lz) = if r.u_axis == 0 { (u, 0.0) } else { (0.0, u) };
    [r.c[0] + r.cs * lx + r.sn * lz, r.c[1] + v, r.c[2] - r.sn * lx + r.cs * lz]
}
#[inline]
fn normal_of(r: &Row) -> [f64; 3] {
    if r.u_axis == 0 {
        [r.sn, 0.0, r.cs]
    } else {
        [r.cs, 0.0, -r.sn]
    }
}
#[inline]
fn add3(a: [f64; 3], b: [f64; 3]) -> [f64; 3] {
    [a[0] + b[0], a[1] + b[1], a[2] + b[2]]
}
#[inline]
fn sub3(a: [f64; 3], b: [f64; 3]) -> [f64; 3] {
    [a[0] - b[0], a[1] - b[1], a[2] - b[2]]
}
#[inline]
fn scale3(a: [f64; 3], s: f64) -> [f64; 3] {
    [a[0] * s, a[1] * s, a[2] * s]
}
#[inline]
fn dot3(a: [f64; 3], b: [f64; 3]) -> f64 {
    a[0] * b[0] + a[1] * b[1] + a[2] * b[2]
}
/// v3.normalize: a length of nought (or none) divides by one
#[inline]
pub fn norm3(a: [f64; 3]) -> [f64; 3] {
    let mut l = hypot3(a[0], a[1], a[2]);
    if l == 0.0 || l != l {
        l = 1.0;
    }
    [a[0] / l, a[1] / l, a[2] / l]
}
#[inline]
fn lerp(a: f64, b: f64, t: f64) -> f64 {
    a + (b - a) * t
}
/// glassbreak.ts nearestOn: the nearest point of a box (yawed, no more) to p
#[inline]
fn nearest_on(b: &[f64], p: [f64; 3]) -> [f64; 3] {
    let (cs, sn) = (b[R_COS], b[R_SIN]);
    let (px, pz) = (p[0] - b[R_C], p[2] - b[R_C + 2]);
    let lx = js_max(-b[R_H], js_min(b[R_H], cs * px - sn * pz));
    let lz = js_max(-b[R_H + 2], js_min(b[R_H + 2], sn * px + cs * pz));
    let ly = js_max(-b[R_H + 1], js_min(b[R_H + 1], p[1] - b[R_C + 1]));
    [b[R_C] + cs * lx + sn * lz, b[R_C + 1] + ly, b[R_C + 2] - sn * lx + cs * lz]
}
#[inline]
fn rec(o: &World, bi: usize) -> &[f64] {
    &o.rec[bi * REC..bi * REC + REC]
}
/// a sheet of glass: thin across its thin axis (the game's own test: glass.ts isSheet)
#[inline]
fn thin(b: &[f64]) -> bool {
    js_min(b[R_H], b[R_H + 2]) <= THICK_GLASS_HALF
}
#[inline]
fn bit(w: &[u32], at: usize, k: u32) -> bool {
    w[at + (k >> 5) as usize] >> (k & 31) & 1 != 0
}
#[inline]
fn site_n(r: &Row) -> usize {
    (r.w[GW_FLAGS] >> 8) as usize
}
#[inline]
fn run(r: &Row) -> bool {
    r.w[GW_FLAGS] & G_RUN != 0
}
#[inline]
fn gone(r: &Row) -> bool {
    r.w[GW_FLAGS] & G_GONE != 0
}
#[inline]
fn dicing(r: &Row) -> bool {
    r.w[GW_FLAGS] & G_DICE != 0
}
#[inline]
fn dice_of(r: &Row) -> Dice {
    Dice { u: r.d[D_U], v: r.d[D_V], cols: r.w[GW_COLS], rows: r.w[GW_ROWS], seed: r.w[GW_DSEED], hu: r.d[D_HU], hv: r.d[D_HV] }
}
/// glassbreak.ts openUV
#[inline]
fn open_uv(r: &Row, u: f64, v: f64) -> bool {
    if gone(r) {
        return true;
    }
    if dicing(r) {
        return dice_out(&r.d[G_DICE_AT..], &r.w, u, v);
    }
    if r.n_open == 0 {
        return false;
    }
    let fid = classify_row(&r.d, &r.w, u, v);
    fid >= 0 && bit(&r.w, GW_OPEN, fid as u32)
}
/// gpNearest: whose territory (u, v) is in (the least power distance); −1 = no sites
#[inline]
fn nearest_site(r: &Row, u: f64, v: f64) -> i32 {
    let (mut best, mut bd) = (-1, f64::INFINITY);
    for i in 0..site_n(r) {
        let s = &r.d[i * G_SITE..i * G_SITE + G_SITE];
        let (du, dv) = (s[S_U] - u, s[S_V] - v);
        let dd = du * du + dv * dv - s[S_RZ] * s[S_RZ];
        if dd < bd {
            bd = dd;
            best = i as i32;
        }
    }
    best
}
#[inline]
fn tooth(f: &Frag) -> bool {
    f.area < k(GB_TOOTH_AREA) && f.frame_sum() > 1e-4
}

// ---- one call's business: whose block, whose world, which leaves, what time it is
#[derive(Clone, Copy)]
pub struct Ctx {
    pub g: usize,
    pub w: usize,
    pub set: i32,
    pub now: f64,
    /// a fragment's landing is the simulation's (shards.ts: its switch, and glass breaking live)
    pub shards: bool,
    /// (shardModel.ts SHARD.gravity: a table of authored numbers, handed in)
    pub gravity: f64,
}
/// one line as the rule asks it of the statics and the leaves: is something solid across it (collision.ts solidBlocked)
#[inline]
fn solid(c: &Ctx, a: [f64; 3], b: [f64; 3]) -> bool {
    lines::solid_line(c.w, c.set, ASK_GLASS, a[0], a[1], a[2], b[0], b[1], b[2]).unwrap_or(true)
}

/// the level's sheets, as the host has them: in scratch, a static and its door gap each
pub fn boot(g: usize, list: &[f64]) {
    let p = pn(g);
    p.n = 0;
    p.nh = 0;
    p.nq = 0;
    p.rev = 0;
    p.released = 0;
    p.flags = 0;
    p.nev = 0;
    p.nbl = 0;
    p.seq = 0;
    let n = list.len() / 2;
    p.sheets = (if n < MAX_SHEETS { n } else { MAX_SHEETS }) as u32;
    for s in 0..p.sheets as usize {
        *p.sheet(s) = Sheet { bi: list[2 * s] as u32, sky: SKY_UNKNOWN, over: NO_OVER, spare: 0, gap: list[2 * s + 1] };
    }
    p.mark();
}
/// is this game's part stood up
#[inline]
pub fn booted(g: usize) -> bool {
    pn(g).used != 0
}

// ---- which glass a pane is
/// glassbreak.ts skySide: which face of the pane has the open sky (+1 the side its normal points to, −1 the other, 0 neither or both). Asked of the level's own
/// geometry once, and kept: statics do not move.
pub fn sky_side(c: &Ctx, s: usize) -> i32 {
    let (bi, known) = {
        let h = pn(c.g).sheet(s);
        (h.bi as usize, h.sky)
    };
    if known != SKY_UNKNOWN {
        return known;
    }
    let (n, a, hu, b_c) = {
        let o = world(c.w);
        let b = rec(o, bi);
        let (cs, sn) = (b[R_COS], b[R_SIN]);
        let wide = b[R_H] >= b[R_H + 2];
        let n = if wide { [sn, 0.0, cs] } else { [cs, 0.0, -sn] };
        let a = if wide { [cs, 0.0, -sn] } else { [sn, 0.0, cs] };
        (n, a, js_max(b[R_H], b[R_H + 2]), [b[R_C], b[R_C + 1], b[R_C + 2]])
    };
    let mut sky = [0u32; 2];
    for k in 0..2 {
        for off in [0.5, 1.0, 1.6] {
            for t in [-0.5, 0.0, 0.5] {
                let d = (if k == 1 { 1.0 } else { -1.0 }) * off;
                let p = [b_c[0] + n[0] * d + a[0] * t * hu, b_c[1], b_c[2] + n[2] * d + a[2] * t * hu];
                if !solid(c, p, [p[0], EXT_SKY_TOP, p[2]]) {
                    sky[k] += 1;
                }
            }
        }
    }
    let e = if (sky[0] >= 5) == (sky[1] >= 5) {
        0
    } else if sky[1] >= 5 {
        1
    } else {
        -1
    };
    pn(c.g).sheet(s).sky = e;
    e
}
/// glassbreak.ts typeOf: the type a pane breaks as when nobody has said. Outside walls are safety glass, a low clerestory strip laminated; inside, a narrow
/// sidelight is laminated and a partition float; under GLASSREAL.place the pane's own place decides between toughened and float.
fn type_by_place(c: &Ctx, s: usize, h0: f64, h1: f64, h2: f64, cy: f64) -> u32 {
    let outside = sky_side(c, s) != 0;
    if if outside { h1 < 0.4 } else { js_max(h0, h2) < 0.35 } {
        return LAMINATED;
    }
    if !truthy(GR_PLACE) {
        return if outside { TEMPERED } else { ANNEALED };
    }
    let foot = cy - h1;
    let gap = pn(c.g).sheet(s).gap;
    if foot < k(GR_LOW_M) || (foot < k(GR_DOOR_FOOT_M) && gap <= k(GR_DOOR_M)) {
        TEMPERED
    } else {
        ANNEALED
    }
}
/// glassbreak.ts typeOfPane: the type of static `bi` as it will break, or NONE: it is no sheet of glass
pub const NO_TYPE: u32 = 9;
pub fn type_of_pane(c: &Ctx, bi: u32) -> u32 {
    let s = match pn(c.g).sheet_of(bi) {
        Some(s) => s,
        None => return NO_TYPE,
    };
    let (h0, h1, h2, cy) = {
        let o = world(c.w);
        let b = rec(o, bi as usize);
        if !thin(b) {
            return NO_TYPE;
        }
        (b[R_H], b[R_H + 1], b[R_H + 2], b[R_C + 1])
    };
    let over = pn(c.g).sheet(s).over;
    if over != NO_OVER {
        return over;
    }
    type_by_place(c, s, h0, h1, h2, cy)
}
/// glassbreak.ts setType: pane `bi` breaks as this from here on, made whole first (NO_OVER: back to its place's word)
pub fn set_type(g: usize, bi: u32, ty: u32) {
    let p = pn(g);
    if let Some(s) = p.sheet_of(bi) {
        p.sheet(s).over = ty;
    }
    if let Some(r) = p.row_of(bi) {
        p.drop_row(r);
    }
    p.nbl = 0;
}
/// glassbreak.ts reset: every pane whole (a word a level's table gave a pane stands, and which face has the sky)
pub fn reset(g: usize) {
    let p = pn(g);
    if p.used == 0 {
        return;
    }
    // (the holes are the next run: they close up onto the sheets)
    let (from, to, nh) = (p.holes_at(), p.rows_at(), p.nh as usize);
    p.body.copy_within(from..from + nh * HOLE_W, to);
    p.n = 0;
    p.nq = 0;
    p.nbl = 0;
    p.flags = 0;
    p.released = 0;
    p.mark();
}

// ---- a pane's first touch
/// glassbreak.ts stateOf: the pane's row, made if this is its first touch; None: it is no sheet of glass
fn state_of(c: &Ctx, bi: u32) -> Option<usize> {
    if let Some(r) = pn(c.g).row_of(bi) {
        return Some(r);
    }
    let ty = type_of_pane(c, bi);
    if ty == NO_TYPE {
        return None;
    }
    let s = pn(c.g).sheet_of(bi)?;
    let exterior = sky_side(c, s) != 0;
    let o = world(c.w);
    let b = rec(o, bi as usize);
    let (hb0, hb1, hb2) = (b[R_H], b[R_H + 1], b[R_H + 2]);
    let u_axis: u32 = if hb0 >= hb2 { 0 } else { 2 };
    let key = box_seed(b);
    let (cc, cs, sn) = ([b[R_C], b[R_C + 1], b[R_C + 2]], b[R_COS], b[R_SIN]);
    let p = pn(c.g);
    let ri = p.add_row(bi);
    let q0 = p.nq;
    let r = p.row(ri);
    r.key = key;
    r.ty = ty;
    r.u_axis = u_axis;
    r.hu = if u_axis == 0 { hb0 } else { hb2 };
    r.hv = hb1;
    r.ht = js_min(hb0, hb2);
    r.exterior = exterior as u32;
    r.c = cc;
    r.cs = cs;
    r.sn = sn;
    r.sound_at = -1e9;
    r.last_dir = [0.0, 0.0, 1.0];
    r.q0 = q0;
    unsafe { TALLY.panes += 1.0 };
    Some(ri)
}

// ---- sites
/// glassbreak.ts addSite: a new site at (u, v) for a blow of energy e, its ring zone fitted to the room the others leave; −1 = no room
fn add_site(r: &mut Row, u: f64, v: f64, e: f64, seed32: u32) -> i32 {
    let ns = site_n(r);
    if ns as f64 >= js_min(MAX_SITES as f64, k(GB_MAX_SITES)) || run(r) {
        return -1;
    }
    let web = r.ty == LAMINATED;
    let base = if web { [k(GB_RHO_L0), k(GB_RHO_L1), k(GB_RHO_L2), k(GB_RHO_L3)] } else { [k(GB_RHO_A0), k(GB_RHO_A1), k(GB_RHO_A2), k(GB_RHO_A3)] };
    let kk = js_max(0.8, js_min(1.4, 0.75 + 0.25 * e));
    let mut rho = [base[0] * kk, base[1] * kk, base[2] * kk, base[3] * kk];
    let mut rz = rho[2] * RHO_TOP * 1.001;
    let mut room = f64::INFINITY;
    for q in 0..ns {
        let s = &r.d[q * G_SITE..q * G_SITE + G_SITE];
        room = js_min(room, hypot2(s[S_U] - u, s[S_V] - v) - s[S_RZ]);
    }
    if room < (if web { k(GB_RZ_MIN) * 0.4 } else { k(GB_RZ_MIN) }) {
        return -1;
    }
    if rz > room {
        let f = room / rz;
        rho[0] *= f;
        rho[1] *= f;
        rho[2] *= f;
        rz = room;
    }
    rho[3] = js_max(rho[3], rho[2] * 2.0);
    let seed = seed32 >> 8;
    let (r0, r1) = if web { (k(GB_RADIALS_L0), k(GB_RADIALS_L1)) } else { (k(GB_RADIALS_A0), k(GB_RADIALS_A1)) };
    let spread = js_max(1.0, r1) as u32;
    let n = js_min(MAX_SECTORS as f64, r0 + ((seed >> 12) % spread) as f64);
    let d = &mut r.d[ns * G_SITE..ns * G_SITE + G_SITE];
    d[S_U] = u;
    d[S_V] = v;
    d[S_ROT] = (seed & 0xfff) as f64 / 4096.0 * PI * 2.0;
    d[S_RHO] = rho[0];
    d[S_RHO + 1] = rho[1];
    d[S_RHO + 2] = rho[2];
    d[S_RHO + 3] = rho[3];
    d[S_RZ] = rz;
    d[S_MERGE] = if web { 0.0 } else { k(GB_MERGE) };
    r.w[GW_N + ns] = n as u32;
    r.w[GW_SEED + ns] = seed;
    r.e[ns] = e;
    r.hits[ns] = 1.0;
    r.w[GW_FLAGS] = (r.w[GW_FLAGS] & 0xff) | ((ns as u32 + 1) << 8);
    unsafe { TALLY.sites += 1.0 };
    ns as i32
}

// ---- what a call let go, for the host (one pane's: the host takes them before it asks of another)
/// the most fragments a pane can let go in one call (a whole sheet's clumps, and a piece or two more)
pub const MAX_OUT: usize = pattern::MAX_CLUMPS + 16;
/// doubles a fragment
pub const OUT_LEN: usize = 32;
/// the corners of the outlines they name, as (u, v) in the pane's frame in the order the outline runs: a clump's twelve or so at most, a piece's sixteen
pub const MAX_OUT_PTS: usize = MAX_OUT * 12;
pub const O_PANE: usize = 0;
pub const O_TYPE: usize = 1;
pub const O_KIND: usize = 2;
pub const O_PTS: usize = 3;
pub const O_NPTS: usize = 4;
pub const O_CX: usize = 5;
pub const O_NX: usize = 8;
pub const O_THICK: usize = 11;
pub const O_AREA: usize = 12;
pub const O_VX: usize = 13;
pub const O_SX: usize = 16;
pub const O_GRAIN: usize = 19;
pub const O_CAUSE: usize = 20;
pub const O_WHO: usize = 21;
pub const O_SEED: usize = 22;
pub const O_KEY: usize = 23;
/// heard: this fragment's going is the pane's crack and hearing event (one a pane a gap)
pub const O_HEARD: usize = 24;
/// its landing, whichever rule's it is: how long until it reaches the floor, and where
pub const O_FALL_T: usize = 25;
pub const O_FALL_X: usize = 26;
/// its landing has been scheduled here (SHARDS.byFall)
pub const O_LANDED: usize = 29;
static mut OUT: [f64; MAX_OUT * OUT_LEN] = [0.0; MAX_OUT * OUT_LEN];
static mut OUT_PTS: [f64; 2 * MAX_OUT_PTS] = [0.0; 2 * MAX_OUT_PTS];
static mut OUT_N: usize = 0;
static mut OUT_NP: usize = 0;
/// (the host starts a call, and reads what is here after it)
#[inline]
pub fn out_clear() {
    unsafe {
        OUT_N = 0;
        OUT_NP = 0;
    }
}
#[inline]
pub fn out_n() -> usize {
    unsafe { OUT_N }
}
#[inline]
pub fn out_addr() -> usize {
    core::ptr::addr_of!(OUT) as usize
}
#[inline]
pub fn out_pts_addr() -> usize {
    core::ptr::addr_of!(OUT_PTS) as usize
}
/// THE COUNTS FOR A TOOL OR A PANEL (glassbreak.ts GB_STATS: no rule reads them back)
#[derive(Clone, Copy)]
#[repr(C)]
pub struct Tally {
    pub panes: f64,
    pub impacts: f64,
    pub sites: f64,
    pub runs: f64,
    pub dropped: f64,
    pub diced: f64,
    pub clumps: f64,
    pub sheets: f64,
    pub blasts: f64,
    pub thinks: f64,
    pub raked: f64,
    pub queue_peak: f64,
    pub queue_full: f64,
}
pub static mut TALLY: Tally = Tally { panes: 0.0, impacts: 0.0, sites: 0.0, runs: 0.0, dropped: 0.0, diced: 0.0, clumps: 0.0, sheets: 0.0, blasts: 0.0, thinks: 0.0, raked: 0.0, queue_peak: 0.0, queue_full: 0.0 };

#[derive(Clone, Copy)]
struct Fr {
    kind: f64,
    centre: [f64; 3],
    normal: [f64; 3],
    area: f64,
    velocity: [f64; 3],
    spin: [f64; 3],
    grain: f64,
    cause: u32,
    seed: u32,
    key: f64,
}
/// falling.ts fallOf: where and when something let go at c with velocity v reaches what is under it (statics and leaves as the ray has them, the world's floor if nothing is)
pub fn fall_of(cx: &Ctx, c: [f64; 3], v: [f64; 3], gravity: f64, reach_k: f64) -> (f64, [f64; 3]) {
    let vy = -v[1];
    let t_of = |h: f64| (-vy + sqrt(vy * vy + 2.0 * gravity * h)) / gravity;
    let mut reach = hypot2(v[0], v[2]) * t_of(js_max(0.02, c[1])) * reach_k;
    let d = if reach > 1e-4 { [v[0] * reach_k * t_of(js_max(0.02, c[1])) / reach, 0.0, v[2] * reach_k * t_of(js_max(0.02, c[1])) / reach] } else { [0.0, 0.0, 0.0] };
    let mut out = [0.0; 4];
    if reach > 0.05 {
        let m = lines::ray_rule(cx.w, c[0], c[1], c[2], d[0], d[1], d[2], reach + 0.05, cx.set, -1, true, &mut out);
        if m >= 0 {
            reach = js_max(0.0, out[0] - 0.05);
        }
    }
    let (x, z) = (c[0] + d[0] * reach, c[2] + d[2] * reach);
    let m = lines::ray_rule(cx.w, x, c[1], z, 0.0, -1.0, 0.0, c[1] + 0.5, cx.set, -1, true, &mut out);
    let h = js_max(0.02, if m >= 0 { out[0] } else { c[1] });
    (t_of(h), [x, c[1] - h, z])
}
/// glassbreak.ts emit, and what shards.ts release decides of a fragment that is the simulation's: the break heard, its landing, and the fragment listed for the host
fn emit(c: &Ctx, ri: usize, f: Fr, pts: &[Pt], rev: bool) {
    let (bi, ty, who, thick, heard) = {
        let r = pn(c.g).row(ri);
        let mut heard = 0.0;
        if f.cause != C_HIT && c.now - r.sound_at >= k(GB_CRACK_GAP) {
            r.sound_at = c.now;
            heard = 1.0;
        }
        (r.w[0], r.ty, r.who, 2.0 * r.ht, heard)
    };
    let (mut fall_t, mut fall_p, mut landed) = (0.0, [0.0; 3], 0.0);
    if c.shards {
        let (t, p) = fall_of(c, f.centre, f.velocity, c.gravity, 0.8);
        fall_t = t;
        fall_p = p;
        if truthy(SH_BY_FALL) {
            landing(c, &f, ty, who, t, p);
            landed = 1.0;
        }
    }
    unsafe {
        let n = OUT_N;
        if n >= MAX_OUT {
            return;
        }
        let (np0, np) = (OUT_NP, pts.len());
        let out = &mut *core::ptr::addr_of_mut!(OUT);
        let opts = &mut *core::ptr::addr_of_mut!(OUT_PTS);
        if np0 + np <= MAX_OUT_PTS {
            for i in 0..np {
                let p = if rev { pts[np - 1 - i] } else { pts[i] };
                opts[2 * (np0 + i)] = p.0;
                opts[2 * (np0 + i) + 1] = p.1;
            }
            OUT_NP = np0 + np;
        }
        let o = &mut out[n * OUT_LEN..(n + 1) * OUT_LEN];
        o[O_PANE] = bi as f64;
        o[O_TYPE] = ty as f64;
        o[O_KIND] = f.kind;
        o[O_PTS] = np0 as f64;
        o[O_NPTS] = np as f64;
        o[O_CX] = f.centre[0];
        o[O_CX + 1] = f.centre[1];
        o[O_CX + 2] = f.centre[2];
        o[O_NX] = f.normal[0];
        o[O_NX + 1] = f.normal[1];
        o[O_NX + 2] = f.normal[2];
        o[O_THICK] = thick;
        o[O_AREA] = f.area;
        o[O_VX] = f.velocity[0];
        o[O_VX + 1] = f.velocity[1];
        o[O_VX + 2] = f.velocity[2];
        o[O_SX] = f.spin[0];
        o[O_SX + 1] = f.spin[1];
        o[O_SX + 2] = f.spin[2];
        o[O_GRAIN] = f.grain;
        o[O_CAUSE] = f.cause as f64;
        o[O_WHO] = who;
        o[O_SEED] = f.seed as f64;
        o[O_KEY] = f.key;
        o[O_HEARD] = heard;
        o[O_FALL_T] = fall_t;
        o[O_FALL_X] = fall_p[0];
        o[O_FALL_X + 1] = fall_p[1];
        o[O_FALL_X + 2] = fall_p[2];
        o[O_LANDED] = landed;
        OUT_N = n + 1;
    }
}
/// (fallshome.ts FALL_SOUNDS, by their place; FALL_UNHEARD, FALL_IF_SOUNDED, FALL_HEARD)
const SOUND_TINKLE: f64 = 0.0;
const SOUND_SHATTER: f64 = 1.0;
const EV_UNHEARD: f64 = 0.0;
const EV_IF_SOUNDED: f64 = 1.0;
const EV_HEARD: f64 = 2.0;
/// shards.ts release, what of it is the simulation's under SHARDS.byFall: a fragment's one landing — a sound for the ear if it has one of its own, the glass it leaves, the floor told
fn landing(c: &Ctx, f: &Fr, ty: u32, who: f64, t: f64, p: [f64; 3]) {
    let a = js_max(1e-4, f.area);
    let dice = f.kind == K_CLUMP || ty == TEMPERED;
    let powder = !dice && f.kind != K_SHEET && f.grain > 0.0;
    let plateable = !dice && !powder && a >= k(SH_PLATE_MIN_AREA);
    let cap = k(SH_PENDING_MAX);
    let at = c.now + t;
    let put = |t: f64, name: f64, gain: f64, rate: f64, amount: f64, ev: f64| {
        falls::schedule(c.g, falls::L_GLASS, t, p[0], p[1], p[2], name, gain, rate, amount, who, ev, 0.0, 0.0, cap);
    };
    if powder {
        put(c.now + (t + 0.05), SOUND_TINKLE, 0.0, 1.0, a, EV_UNHEARD);
    } else if dice {
        put(at, SOUND_SHATTER, js_min(1.0, 0.35 + a * 4.0), 1.15 + 0.3 * hash01(f.seed as f64), a * 0.8, EV_IF_SOUNDED);
    } else if f.kind == K_SHEET {
        put(at, SOUND_TINKLE, if plateable { 0.0 } else { 0.5 }, 0.8, a * 0.3, EV_HEARD);
    } else {
        put(at, SOUND_TINKLE, if plateable { 0.0 } else { js_min(0.9, 0.3 + a * 8.0) }, 0.9 + 0.5 * hash01(f.seed as f64), a, EV_HEARD);
    }
}

/// glassbreak.ts afterRelease: the pane has a piece less
fn after_release(c: &Ctx, ri: usize) {
    let (w, was, ext) = {
        let r = pn(c.g).row(ri);
        (walkable_now(r), r.walkable, r.exterior)
    };
    if w as u32 != was {
        pn(c.g).row(ri).walkable = w as u32;
        wrote();
    }
    if ext != 0 {
        pn(c.g).flags |= FL_GUST;
    }
}
/// A STATIC'S KIND MOVED (a pane a walker now goes through): the host says so to the copy when the call is back
static mut WROTE: bool = false;
#[inline]
fn wrote() {
    unsafe { WROTE = true }
}
/// …has it, since the last time it asked
#[inline]
pub fn take_wrote() -> bool {
    unsafe {
        let w = WROTE;
        WROTE = false;
        w
    }
}
/// glassbreak.ts walkableNow
fn walkable_now(r: &mut Row) -> bool {
    if gone(r) {
        return true;
    }
    if dicing(r) || r.ty != ANNEALED || !run(r) {
        return false;
    }
    let v_clear = k(GB_WALK_CLEAR_H) - r.c[1];
    let frags = frags_of(&r.d, &r.w, r.hu, r.hv);
    for f in frags.list() {
        if f.v_min < v_clear && !bit(&r.w, GW_OPEN, f.fid) && !tooth(f) {
            return false;
        }
    }
    true
}
fn set_open(r: &mut Row, fid: u32) {
    r.w[GW_OPEN + (fid >> 5) as usize] |= 1 << (fid & 31);
    r.n_open += 1;
}
/// glassbreak.ts releaseCrater: a site's crater wedges go together, as one fragment. → fragments let go
fn release_crater(c: &Ctx, ri: usize, i: usize, cause: u32) -> u32 {
    let (hull_n, hp, frag) = {
        let r = pn(c.g).row(ri);
        let n = r.w[GW_N + i] as usize;
        let mut any = false;
        for s in 0..n {
            let fid = (i * FRAGS_PER_SITE + s) as u32;
            if !bit(&r.w, GW_OPEN, fid) {
                set_open(r, fid);
                any = true;
            }
        }
        if !any {
            return 0;
        }
        unsafe { TALLY.dropped += 1.0 };
        let site = site_of(&r.d, &r.w, i);
        let mut pts = [(0.0, 0.0); MAX_SECTORS];
        for s in 0..n {
            pts[s] = site.chord(s as i32, 0);
        }
        let mut hl = [(0.0, 0.0); 32];
        let m = hull(&mut pts[..n], &mut hl);
        let g = clip_rect(&hl[..m], r.hu, r.hv);
        if g.n < 3 {
            return 0;
        }
        let (area, cu, cv) = poly_area(&g.p[..g.n]);
        let nr = normal_of(r);
        let out_sign = if dot3(nr, r.last_dir) >= 0.0 { 1.0 } else { -1.0 };
        let nn = scale3(nr, out_sign);
        let seed = r.w[GW_SEED + i];
        let h = hash01(seed as f64);
        (
            g.n,
            g.p,
            (Fr { kind: K_PIECE, centre: to_world(r, cu, cv), normal: nn, area, velocity: add3(scale3(nn, 2.5 + 2.0 * h), [0.0, -0.2, 0.0]), spin: [0.0, 0.0, 0.0], grain: 0.004, cause, seed: seed ^ 0xC2A7E5, key: (i * FRAGS_PER_SITE) as f64 }, out_sign),
        )
    };
    let rev = outline_rev(pn(c.g).row(ri), frag.1);
    emit(c, ri, frag.0, &hp[..hull_n], rev);
    after_release(c, ri);
    1
}
/// glassbreak.ts outline: (u, v, w) is right-handed for a pane along x and left-handed for one along z: is the (u, v) polygon to be walked backwards to run anticlockwise from the side it leaves by
#[inline]
fn outline_rev(r: &Row, out_sign: f64) -> bool {
    (out_sign > 0.0) != (r.u_axis == 0)
}
/// glassbreak.ts releaseFrag
fn release_frag(c: &Ctx, ri: usize, fid: u32, cause: u32, from: Option<[f64; 3]>, kk: f64) {
    let (f, fr, rev) = {
        let r = pn(c.g).row(ri);
        if bit(&r.w, GW_OPEN, fid) {
            return;
        }
        let f = match frags_of(&r.d, &r.w, r.hu, r.hv).by(fid) {
            Some(f) => *f,
            None => return,
        };
        set_open(r, fid);
        unsafe { TALLY.dropped += 1.0 };
        let n = normal_of(r);
        let cw = to_world(r, f.cu, f.cv);
        let mut dir_out = r.last_dir;
        if let Some(fr) = from {
            let d = sub3(cw, fr);
            if hypot3(d[0], d[1], d[2]) > 1e-6 {
                dir_out = norm3(d);
            }
        }
        let out_sign = if dot3(n, dir_out) >= 0.0 { 1.0 } else { -1.0 };
        let nn = scale3(n, out_sign);
        let key = r.key as f64;
        let fd = fid as f64;
        let (h1, h2, h3) = (hash01(key * 7919.0 + fd * 131.0), hash01(key * 104729.0 + fd * 193.0), hash01(fd * 7.0 + key));
        let speed = if cause == C_BLAST {
            k(GB_BLAST_OUT) * kk * (0.6 + 0.8 * h3)
        } else if cause == C_HIT {
            k(GB_OUT) * (0.8 + 1.2 * h3)
        } else if cause == C_RAKE {
            k(GB_OUT) * (1.2 + 0.8 * h3)
        } else {
            k(GB_OUT) * (0.15 + 0.5 * h3)
        };
        let base = if (cause == C_BLAST || cause == C_RAKE) && from.is_some() { scale3(norm3(add3(scale3(dir_out, 0.7), scale3(nn, 0.5))), speed) } else { scale3(nn, speed) };
        let velocity = add3(base, [0.0, -0.2 - 0.6 * h1, 0.0]);
        let spin = k(GB_SPIN);
        let seed = point_seed(r.key, cw[0], cw[1], cw[2]) ^ fid;
        (
            f,
            Fr { kind: if f.b >= 3 { K_DAGGER } else { K_PIECE }, centre: cw, normal: nn, area: f.area, velocity, spin: [(h1 - 0.5) * spin, (h2 - 0.5) * 2.0, (h3 - 0.5) * spin], grain: 0.0, cause, seed, key: fd },
            outline_rev(r, out_sign),
        )
    };
    emit(c, ri, fr, &f.p[..f.n as usize], rev);
    after_release(c, ri);
}
/// glassbreak.ts knockOut: a hard blow on a piece: it goes now (a crater's wedge takes the whole crater with it). → fragments let go
fn knock_out(c: &Ctx, ri: usize, fid: u32) -> u32 {
    let r = pn(c.g).row(ri);
    if bit(&r.w, GW_OPEN, fid) {
        return 0;
    }
    let (i, b) = (fid as usize / FRAGS_PER_SITE, (fid as usize % FRAGS_PER_SITE) / MAX_SECTORS);
    if b == 0 {
        return release_crater(c, ri, i, C_HIT);
    }
    if frags_of(&r.d, &r.w, r.hu, r.hv).by(fid).is_none() {
        return 0;
    }
    release_frag(c, ri, fid, C_HIT, None, 1.0);
    1
}
/// glassbreak.ts setRun: the radials reach the frame
fn set_run(c: &Ctx, ri: usize) {
    let r = pn(c.g).row(ri);
    if run(r) || r.ty != ANNEALED || site_n(r) == 0 {
        return;
    }
    r.w[GW_FLAGS] |= G_RUN;
    unsafe { TALLY.runs += 1.0 };
    r.next_think = c.now + k(GB_THINK);
}

// ---- toughened glass
/// glassbreak.ts startDice
fn start_dice(c: &Ctx, ri: usize, u: f64, v: f64, seed: u32) {
    let d = {
        let r = pn(c.g).row(ri);
        if dicing(r) || gone(r) {
            return;
        }
        let cols = js_max(1.0, round_half_up(2.0 * r.hu / k(GB_CLUMP_M))) as u32;
        let rows = js_max(1.0, round_half_up(2.0 * r.hv / k(GB_CLUMP_M))) as u32;
        r.w[GW_COLS] = cols;
        r.w[GW_ROWS] = rows;
        r.w[GW_DSEED] = seed >> 8;
        r.d[D_U] = u;
        r.d[D_V] = v;
        r.d[D_HU] = r.hu;
        r.d[D_HV] = r.hv;
        r.d[D_LAST] = f64::NEG_INFINITY;
        r.d[D_LAST_K] = -1.0;
        r.d[D_T0] = c.now;
        r.w[GW_FLAGS] |= G_DICE;
        r.d_next = 0;
        unsafe { TALLY.diced += 1.0 };
        dice_of(r)
    };
    let order = order_of(&d);
    let k0 = d.clump(u, v);
    let mut key0 = 0.0;
    for cl in order {
        if cl.k == k0 {
            key0 = cl.key;
            break;
        }
    }
    let mut next = 0;
    while next < order.len() && order[next].key <= key0 + DICE_TIE {
        release_clump(c, ri, &d, order[next]);
        next += 1;
        pn(c.g).row(ri).d_next = next as u32;
    }
    if next >= order.len() {
        dice_done(c, ri);
    }
}
fn dice_done(c: &Ctx, ri: usize) {
    let r = pn(c.g).row(ri);
    r.w[GW_FLAGS] |= G_GONE;
    r.walkable = 1;
    wrote();
    if r.exterior != 0 {
        pn(c.g).flags |= FL_GUST;
    }
}
/// glassbreak.ts stepDice
fn step_dice(c: &Ctx, ri: usize) {
    let (d, t0) = {
        let r = pn(c.g).row(ri);
        (dice_of(r), r.d[D_T0])
    };
    if truthy(GR_DROP) {
        if c.now - t0 >= k(GR_HANG) {
            drop_all(c, ri, &d);
        } else {
            let r = pn(c.g).row(ri);
            if r.walkable != 0 {
                r.walkable = 0;
                wrote();
            }
        }
        return;
    }
    let order = order_of(&d);
    let rr = (c.now - t0 - k(GB_DICE_HANG)) * k(GB_DICE_SPEED);
    let mut next = pn(c.g).row(ri).d_next as usize;
    let mut n = 0.0;
    let per = k(GB_CLUMPS_PER_FRAME);
    while next < order.len() && (order[next].key <= order[next - 1].key + DICE_TIE || (order[next].key <= rr && n < per)) {
        release_clump(c, ri, &d, order[next]);
        next += 1;
        n += 1.0;
        pn(c.g).row(ri).d_next = next as u32;
    }
    if next >= order.len() {
        dice_done(c, ri);
    } else {
        let r = pn(c.g).row(ri);
        if r.walkable != 0 {
            r.walkable = 0;
            wrote();
        }
    }
}
/// glassbreak.ts releaseClump
fn release_clump(c: &Ctx, ri: usize, d: &Dice, cl: pattern::Clump) {
    {
        let r = pn(c.g).row(ri);
        r.d[D_LAST] = cl.key;
        r.d[D_LAST_K] = cl.k as f64;
    }
    unsafe {
        TALLY.clumps += 1.0;
        TALLY.dropped += 1.0;
    }
    let mut pts = [(0.0, 0.0); MAX_PTS];
    let np = d.poly(cl.c as u32, cl.r as u32, &mut pts);
    if np < 3 {
        return;
    }
    let (area, cu, cv) = poly_area(&pts[..np]);
    if area < 1e-8 {
        return;
    }
    let (fr, rev) = {
        let r = pn(c.g).row(ri);
        let n = normal_of(r);
        let out_sign = if dot3(n, r.last_dir) >= 0.0 { 1.0 } else { -1.0 };
        let nn = scale3(n, out_sign);
        let (key, ck) = (r.key as f64, cl.k as f64);
        let (h1, h2, h3) = (hash01(key * 7919.0 + ck * 131.0), hash01(key * 104729.0 + ck * 193.0), hash01(ck * 7.0 + key));
        let rad = js_max(1e-3, hypot2(cu - d.u, cv - d.v));
        let (a, b) = (to_world(r, 0.0, 0.0), to_world(r, (cu - d.u) / rad, (cv - d.v) / rad));
        let spread = sub3(b, a);
        let velocity = add3(add3(scale3(nn, k(GB_OUT) * (0.3 + 0.9 * h1)), scale3(spread, 0.5 * h2)), [0.0, -0.3 - 0.5 * h3, 0.0]);
        let spin = k(GB_SPIN);
        let centre = to_world(r, cu, cv);
        (
            Fr { kind: K_CLUMP, centre, normal: nn, area, velocity, spin: [(h1 - 0.5) * spin, (h2 - 0.5) * 2.0, (h3 - 0.5) * spin], grain: k(GB_DICE_GRAIN), cause: C_DICE, seed: d.seed ^ (cl.k + 1).wrapping_mul(0x9E37_79B1), key: ck },
            outline_rev(r, out_sign),
        )
    };
    emit(c, ri, fr, &pts[..np], rev);
}
/// glassbreak.ts dropAll (GLASSREAL.drop): every clump still standing leaves on this step, handed on in square blocks of them
fn drop_all(c: &Ctx, ri: usize, d: &Dice) {
    let order = order_of(d);
    const MAX_BLOCKS: usize = 1024;
    let blocks = js_min(MAX_BLOCKS as f64, js_max(1.0, k(GR_BLOCKS)));
    let (cols, rows) = (d.cols as f64, d.rows as f64);
    let mut m = 1.0;
    while js_ceil(cols / m) * js_ceil(rows / m) > blocks {
        m += 1.0;
    }
    let (du, dv) = (2.0 * d.hu / cols, 2.0 * d.hv / rows);
    let (bc, br) = (js_ceil(cols / m) as usize, js_ceil(rows / m) as usize);
    let mut n = [0u32; MAX_BLOCKS];
    let mut su = [0.0; MAX_BLOCKS];
    let mut sv = [0.0; MAX_BLOCKS];
    let mi = m as usize;
    let mut next = pn(c.g).row(ri).d_next as usize;
    while next < order.len() {
        let cl = order[next];
        unsafe {
            TALLY.clumps += 1.0;
            TALLY.dropped += 1.0;
        }
        let b = (cl.r as usize / mi) * bc + cl.c as usize / mi;
        let p = d.centre(cl.c as u32, cl.r as u32);
        n[b] += 1;
        su[b] += p.0;
        sv[b] += p.1;
        next += 1;
    }
    {
        let r = pn(c.g).row(ri);
        r.d_next = next as u32;
        r.d[D_LAST] = f64::INFINITY;
        r.d[D_LAST_K] = f64::INFINITY;
    }
    for b in 0..bc * br {
        if n[b] == 0 {
            continue;
        }
        let (c0, r0) = ((b % bc) * mi, (b / bc) * mi);
        let u0 = -d.hu + c0 as f64 * du;
        let u1 = js_min(d.hu, u0 + m * du);
        let v0 = -d.hv + r0 as f64 * dv;
        let v1 = js_min(d.hv, v0 + m * dv);
        let (cu, cv) = (su[b] / n[b] as f64, sv[b] / n[b] as f64);
        let (fr, rev) = {
            let r = pn(c.g).row(ri);
            let nr = normal_of(r);
            let out_sign = if dot3(nr, r.last_dir) >= 0.0 { 1.0 } else { -1.0 };
            let nn = scale3(nr, out_sign);
            let (key, bd) = (r.key as f64, b as f64);
            let (h1, h3) = (hash01(key * 7919.0 + bd * 131.0), hash01(bd * 7.0 + key));
            let near = js_max(0.0, 1.0 - hypot2(cu - d.u, cv - d.v) / 0.6);
            let spin = k(GB_SPIN);
            (
                Fr { kind: K_CLUMP, centre: to_world(r, cu, cv), normal: nn, area: n[b] as f64 * du * dv, velocity: add3(scale3(nn, k(GB_OUT) * (0.1 + 0.25 * h1 + 0.8 * near)), [0.0, -0.3 - 0.5 * h3, 0.0]), spin: [(h1 - 0.5) * spin, 0.0, (h3 - 0.5) * spin], grain: k(GB_DICE_GRAIN), cause: C_DICE, seed: d.seed ^ (b as u32 + 1).wrapping_mul(0x9E37_79B1), key: bd },
                outline_rev(r, out_sign),
            )
        };
        emit(c, ri, fr, &[(u0, v0), (u1, v0), (u1, v1), (u0, v1)], rev);
    }
    dice_done(c, ri);
}
/// glassbreak.ts releaseSheet: the laminated sheet leaves its frame whole
fn release_sheet(c: &Ctx, ri: usize, away: [f64; 3], kk: f64) {
    let (fr, pts, rev) = {
        let p = pn(c.g);
        let qn = p.row(ri).qn as usize;
        let r = p.row(ri);
        if gone(r) {
            return;
        }
        r.w[GW_FLAGS] |= G_GONE;
        r.walkable = 1;
        wrote();
        unsafe {
            TALLY.sheets += 1.0;
            TALLY.dropped += 1.0;
        }
        for _ in 0..qn {
            p.q_pop(ri);
        }
        let r = p.row(ri);
        if r.exterior != 0 {
            p.flags |= FL_GUST;
        }
        let r = p.row(ri);
        let n = normal_of(r);
        let out_sign = if dot3(n, away) >= 0.0 { 1.0 } else { -1.0 };
        let nn = scale3(n, out_sign);
        let (hu, hv) = (r.hu, r.hv);
        let area = 4.0 * hu * hv;
        let seed = point_seed(r.key, r.c[0], r.c[1], r.c[2]);
        (
            Fr { kind: K_SHEET, centre: r.c, normal: nn, area, velocity: add3(scale3(nn, k(GB_BLAST_OUT) * kk * 0.6), [0.0, 0.4, 0.0]), spin: [1.5 * kk, 0.0, 0.4], grain: 0.0, cause: C_BLAST, seed, key: 0.0 },
            [(-hu, -hv), (hu, -hv), (hu, hv), (-hu, hv)],
            outline_rev(r, out_sign),
        )
    };
    emit(c, ri, fr, &pts, rev);
}

// ---- the queue
/// a piece's turn to leave (glassbreak.ts `pending.push`, the sort that follows it: a pane's run is kept in that order)
fn queue(c: &Ctx, ri: usize, fid: u32, at: f64, cause: u32, from: [f64; 3], kk: f64) {
    let p = pn(c.g);
    if !p.q_put(ri, Q { at, from, k: kk, fid, cause }) {
        unsafe { TALLY.queue_full += 1.0 };
        return;
    }
    unsafe {
        if p.nq as f64 > TALLY.queue_peak {
            TALLY.queue_peak = p.nq as f64;
        }
    }
}

// ---- a blow
/// what impact() answers
#[derive(Clone, Copy, Default)]
pub struct Blow {
    pub live: bool,
    pub ty: u32,
    pub was_open: bool,
    pub punched: bool,
    pub diced: bool,
    pub released: u32,
    pub cracked: bool,
}
/// glassbreak.ts impact: a blow of `energy` (1 = one pistol round square on) met pane `bi` at (x, y, z) travelling `dir`. `through`: the caller's word on the round
/// (1 went on · 0 stopped · −1 none: the glass decides by energy). The glass does what it does; what the round does next is the caller's.
pub fn impact(c: &Ctx, bi: u32, x: [f64; 3], dir: Option<[f64; 3]>, energy: f64, who: f64, through: i32) -> Blow {
    let mut res = Blow { ty: LAMINATED, ..Blow::default() };
    let ty = type_of_pane(c, bi);
    res.ty = if ty == NO_TYPE { LAMINATED } else { ty };
    let ri = match state_of(c, bi) {
        Some(r) => r,
        None => return res,
    };
    res.live = true;
    let (u, v, seed, ty) = {
        let r = pn(c.g).row(ri);
        let (u, v) = to_uv_in(r, x[0], x[1], x[2]);
        if open_uv(r, u, v) {
            res.was_open = true;
            res.punched = true;
            return res;
        }
        r.hit_n += 1;
        r.energy += energy;
        r.damage = js_min(1.0, r.damage + 0.12 * energy);
        r.who = who;
        r.last_uv = [u, v];
        if let Some(d) = dir {
            if hypot3(d[0], d[1], d[2]) > 1e-6 {
                r.last_dir = norm3(d);
            }
        }
        unsafe { TALLY.impacts += 1.0 };
        (u, v, point_seed(r.key, x[0], x[1], x[2]), r.ty)
    };
    if ty == TEMPERED {
        let r = pn(c.g).row(ri);
        if r.energy >= k(GB_BREAK_E_T) || through == 1 {
            let started = !dicing(r);
            start_dice(c, ri, u, v, seed);
            res.diced = true;
            res.punched = open_uv(pn(c.g).row(ri), u, v);
            res.released = if started { 1 } else { 0 };
            res.cracked = true;
        }
        return res;
    }
    if ty == LAMINATED {
        let r = pn(c.g).row(ri);
        if classify_row(&r.d, &r.w, u, v) < 0 && add_site(r, u, v, energy, seed) >= 0 {
            res.cracked = true;
            return res;
        }
        let i = nearest_site(r, u, v);
        if i >= 0 {
            r.e[i as usize] += energy;
            r.hits[i as usize] += 1.0;
        }
        return res;
    }
    // float glass
    let hard = if through >= 0 { through == 1 } else { energy >= k(GB_PUNCH_E) };
    let mut no_room = false;
    let fid = {
        let r = pn(c.g).row(ri);
        classify_row(&r.d, &r.w, u, v)
    };
    if fid >= 0 {
        // on a piece that is still in: the blow counts on its site, and a hard one knocks the piece out
        let r = pn(c.g).row(ri);
        let i = fid as usize / FRAGS_PER_SITE;
        r.e[i] += energy;
        r.hits[i] += 1.0;
        if hard {
            res.released += knock_out(c, ri, fid as u32);
        }
    } else {
        // on the uncut sheet: a new star if there is room for one
        let r = pn(c.g).row(ri);
        let i = add_site(r, u, v, energy, seed);
        if i >= 0 {
            res.cracked = true;
            if hard {
                res.released += release_crater(c, ri, i as usize, C_HIT);
            }
        } else {
            let j = nearest_site(r, u, v);
            if j >= 0 {
                r.e[j as usize] += energy;
                r.hits[j as usize] += 1.0;
                no_room = true;
            }
        }
    }
    let goes = {
        let r = pn(c.g).row(ri);
        !run(r) && (r.energy >= k(GB_BREAK_E_A) || energy >= k(GB_RUN_SINGLE_E) || (through == 1 && no_room))
    };
    if goes {
        set_run(c, ri);
        let (open_now, f2) = {
            let r = pn(c.g).row(ri);
            (open_uv(r, u, v), classify_row(&r.d, &r.w, u, v))
        };
        if hard && !open_now && f2 >= 0 {
            res.released += knock_out(c, ri, f2 as u32);
        }
        let j = if hard {
            let r = pn(c.g).row(ri);
            nearest_site(r, u, v)
        } else {
            -1
        };
        if j >= 0 && k(GB_RUN_STAR) > 0.0 {
            ring_out(c, ri, j as usize);
        }
    }
    let r = pn(c.g).row(ri);
    res.punched = open_uv(r, u, v);
    if res.released > 0 {
        res.cracked = true;
    }
    if r.next_think <= 0.0 {
        r.next_think = c.now + k(GB_THINK);
    }
    res
}
/// the blow that RUNS a pane breaks its own star out: the rings round the site it landed on follow the crater over the next few frames
fn ring_out(c: &Ctx, ri: usize, j: usize) {
    let (frags, key) = {
        let r = pn(c.g).row(ri);
        (frags_of(&r.d, &r.w, r.hu, r.hv), r.key as f64)
    };
    let top = js_min(2.0, k(GB_RUN_STAR));
    for f in frags.list() {
        if f.i as usize == j && f.b >= 1 && f.b as f64 <= top {
            let r = pn(c.g).row(ri);
            if !bit(&r.w, GW_OPEN, f.fid) && !bit(&r.w, GW_GOING, f.fid) {
                let at = c.now + k(GB_RUN_RING_SECS) * (f.b as f64 + hash01(key * 53.0 + f.fid as f64));
                queue(c, ri, f.fid, at, C_HIT, [0.0; 3], 1.0);
            }
        }
    }
}
/// glassbreak.ts widen: a pellet through glass its own shell has just broken: the piece it passed through, if one still stands, goes with it. → fragments let go
pub fn widen(c: &Ctx, bi: u32, x: [f64; 3]) -> u32 {
    let ri = match pn(c.g).row_of(bi) {
        Some(r) => r,
        None => return 0,
    };
    let fid = {
        let r = pn(c.g).row(ri);
        if r.ty != ANNEALED {
            return 0;
        }
        let (u, v) = to_uv_in(r, x[0], x[1], x[2]);
        if open_uv(r, u, v) {
            return 0;
        }
        let fid = classify_row(&r.d, &r.w, u, v);
        if fid < 0 {
            return 0;
        }
        fid as u32
    };
    let n = knock_out(c, ri, fid);
    let r = pn(c.g).row(ri);
    if n > 0 && r.next_think <= 0.0 {
        r.next_think = c.now + k(GB_THINK);
    }
    n
}

// ---- a blast
/// the charges in hand: is this arrival one already seen. → which of them, and its number
fn charge_of(p: &mut Panes, now: f64, at: [f64; 3]) -> (usize, f64) {
    let mut w = 0;
    for i in 0..p.nbl as usize {
        let t = p.bl[5 * i];
        if now >= t && now - t < 0.25 {
            if w != i {
                for k in 0..5 {
                    p.bl[5 * w + k] = p.bl[5 * i + k];
                }
            }
            w += 1;
        }
    }
    p.nbl = w as u32;
    for i in 0..w {
        let b = &p.bl[5 * i..5 * i + 5];
        if hypot2(b[1] - at[0], b[3] - at[2]) < 0.05 && js_abs(b[2] - at[1]) < 0.6 {
            return (i, b[4]);
        }
    }
    if w == CHARGES {
        // (sixteen charges inside a quarter of a second: the oldest is forgotten, and a second arrival of it would count as a blow of its own)
        for i in 1..CHARGES {
            for k in 0..5 {
                p.bl[5 * (i - 1) + k] = p.bl[5 * i + k];
            }
        }
        w -= 1;
    }
    p.seq += 1;
    let seq = p.seq as f64;
    let b = &mut p.bl[5 * w..5 * w + 5];
    b[0] = now;
    b[1] = at[0];
    b[2] = at[1];
    b[3] = at[2];
    b[4] = seq;
    p.nbl = w as u32 + 1;
    unsafe { TALLY.blasts += 1.0 };
    (w, seq)
}
#[inline]
fn prev_k(r: &Row, seq: f64) -> f64 {
    for i in 0..BK {
        if r.bk[2 * i] == seq {
            return r.bk[2 * i + 1];
        }
    }
    0.0
}
#[inline]
fn set_k(r: &mut Row, seq: f64, kk: f64) {
    for i in 0..BK {
        if r.bk[2 * i] == seq {
            r.bk[2 * i + 1] = kk;
            return;
        }
    }
    let at = r.bk_at as usize % BK;
    r.bk[2 * at] = seq;
    r.bk[2 * at + 1] = kk;
    r.bk_at = r.bk_at.wrapping_add(1);
}
/// the arrival in hand (glassbreak.ts blastIn): its charge seen, ready for the sheets one at a time
pub fn blast_begin(c: &Ctx, at: [f64; 3]) {
    let (i, seq) = charge_of(pn(c.g), c.now, at);
    let p = pn(c.g);
    unsafe {
        BLAST_SEQ = seq;
        BLAST_AT = [p.bl[5 * i + 1], p.bl[5 * i + 2], p.bl[5 * i + 3]];
    }
}
static mut BLAST_SEQ: f64 = 0.0;
static mut BLAST_AT: [f64; 3] = [0.0; 3];
/// glassbreak.ts blastIn over one sheet of the level. → it took the blow (blast()'s count of panes changed)
pub fn blast_sheet(c: &Ctx, s: usize, at: [f64; 3], radius: f64, power: f64, who: f64) -> bool {
    let bi = pn(c.g).sheet(s).bi;
    let (seq, twin_at) = unsafe { (BLAST_SEQ, BLAST_AT) };
    let q = {
        let o = world(c.w);
        nearest_on(rec(o, bi as usize), at)
    };
    let d = hypot3(q[0] - at[0], q[1] - at[1], q[2] - at[2]);
    if d > radius {
        return false;
    }
    let kk = power * (1.0 - d / radius);
    if kk <= BLAST_MIN_K {
        return false;
    }
    let had_row = pn(c.g).row_of(bi);
    let prev = match had_row {
        Some(r) => prev_k(pn(c.g).row(r), seq),
        None => 0.0,
    };
    if prev >= kk {
        return false;
    }
    let ty = type_of_pane(c, bi);
    if ty == NO_TYPE || (ty == TEMPERED && kk <= k(GB_BLAST_DICE_K) && had_row.is_none()) {
        return false;
    }
    let ri = match state_of(c, bi) {
        Some(r) => r,
        None => return false,
    };
    let (away, qu, qv) = {
        let r = pn(c.g).row(ri);
        if gone(r) {
            return false;
        }
        set_k(r, seq, kk);
        let away = if d > 1e-6 { norm3(sub3(q, at)) } else { normal_of(r) };
        r.last_dir = away;
        r.who = who;
        let (qu, qv) = to_uv(r, q[0], q[1], q[2]);
        r.last_uv = [qu, qv];
        r.damage = js_min(1.0, r.damage + 0.3 * (kk - prev));
        (away, qu, qv)
    };
    if pn(c.g).row(ri).ty == TEMPERED {
        let r = pn(c.g).row(ri);
        if kk > k(GB_BLAST_DICE_K) && !dicing(r) {
            r.energy += 1.0 + 2.0 * kk;
            let seed = blast_seed(r.key, &twin_at);
            start_dice(c, ri, qu, qv, seed);
            return true;
        }
        return false;
    }
    {
        // stars toward the blast: glass.ts crackNear's construction (a hashed point on the face inside its frame by a tenth, pulled a third of the way to the nearest point)
        let r = pn(c.g).row(ri);
        let web = r.ty == LAMINATED;
        let stars_of = |x: f64| if web { 3.0 } else { 1.0 + round_half_up(2.0 * js_min(1.0, x)) };
        let mut s = blast_seed(r.key, &twin_at);
        let stars = stars_of(kk) as u32;
        let had = if prev > 0.0 { stars_of(prev) as u32 } else { 0 };
        for n in 0..stars {
            s = (s ^ (s >> 15)).wrapping_mul(0x2C1B_3C6D).wrapping_add(0x297A_2D39);
            if n < had {
                continue;
            }
            let fu = ((s & 0xffff) as f64 / 65535.0) * 2.0 - 1.0;
            let fv = ((s >> 16) as f64 / 65535.0) * 2.0 - 1.0;
            let a = to_world(r, fu * 0.9 * r.hu, fv * 0.9 * r.hv);
            let m = [lerp(a[0], q[0], 0.35), lerp(a[1], q[1], 0.35), lerp(a[2], q[2], 0.35)];
            let (pu, pv) = to_uv(r, m[0], m[1], m[2]);
            if open_uv(r, pu, pv) || classify_row(&r.d, &r.w, pu, pv) >= 0 {
                continue;
            }
            add_site(r, pu, pv, 0.6 + kk, s);
        }
        r.energy += if prev > 0.0 { 2.0 * (kk - prev) } else { 1.0 + 2.0 * kk };
        if web {
            if kk > k(GB_BLAST_SHEET_K) {
                release_sheet(c, ri, away, kk);
            }
            return true;
        }
    }
    if kk > k(GB_BLAST_RUN_K) {
        set_run(c, ri);
    }
    let reach = blast_reach(kk);
    let frags = {
        let r = pn(c.g).row(ri);
        frags_of(&r.d, &r.w, r.hu, r.hv)
    };
    for f in frags.list() {
        let r = pn(c.g).row(ri);
        if bit(&r.w, GW_OPEN, f.fid) || bit(&r.w, GW_GOING, f.fid) {
            continue;
        }
        let dd = hypot2(f.cu - qu, f.cv - qv);
        if dd > reach {
            continue;
        }
        if kk < k(GB_TOOTH_K) && tooth(f) {
            continue;
        }
        queue(c, ri, f.fid, c.now + dd * 0.03, C_BLAST, at, kk);
    }
    let r = pn(c.g).row(ri);
    if r.next_think <= 0.0 {
        r.next_think = c.now + k(GB_THINK);
    }
    true
}
/// metres round a pane's nearest point to a blast of strength k within which its pieces are taken
#[inline]
fn blast_reach(kk: f64) -> f64 {
    1.4 * kk + 0.15
}

// ---- support
static mut HELD_BITS: [u32; 21] = [0; 21];
/// glassbreak.ts heldSet: which intact pieces of a float pane are HELD, grown outward from the frame and the uncut sheet, so every held piece is joined to them through held pieces
fn held_set(r: &Row, without: i32) -> &'static [u32; 21] {
    let held = unsafe { &mut *core::ptr::addr_of_mut!(HELD_BITS) };
    *held = [0; 21];
    let frags = frags_of(&r.d, &r.w, r.hu, r.hv);
    let ac = pattern::across(frags, &r.d, &r.w);
    let (side, top, wedged, stand_ny, stand_foot, tol, hang) = (k(GB_GRIP_SIDE), k(GB_GRIP_TOP), k(GB_GRIP_WEDGED), k(GB_STAND_NY), k(GB_STAND_FOOT), k(GB_BALANCE_TOL), k(GB_HANG_AREA));
    let mut changed = true;
    while changed {
        changed = false;
        for (at, f) in frags.list().iter().enumerate() {
            if bit(&r.w, GW_OPEN, f.fid) || f.fid as i32 == without || held[(f.fid >> 5) as usize] >> (f.fid & 31) & 1 != 0 {
                continue;
            }
            let mut ok = false;
            let (mut u0, mut u1) = (f64::INFINITY, f64::NEG_INFINITY);
            let mut grip = (f.frame[pattern::F_LEFT] + f.frame[pattern::F_RIGHT] + f.frame[pattern::F_BOTTOM]) * side + f.frame[pattern::F_TOP] * top;
            if f.has_foot {
                u0 = f.foot.0;
                u1 = f.foot.1;
            }
            let mut e = ac.head[at];
            while e != 0 {
                let n = &ac.nbs[e as usize - 1];
                e = n.next;
                if n.dead {
                    continue;
                }
                if n.fid == REMAINDER {
                    ok = true;
                    break;
                }
                if n.fid < 0 {
                    continue;
                }
                let o = n.fid as u32;
                if frags.place(o) == 0 || bit(&r.w, GW_OPEN, o) || o as i32 == without || held[(o >> 5) as usize] >> (o & 31) & 1 == 0 {
                    continue;
                }
                grip += n.len * wedged;
                if n.ny < -stand_ny {
                    u0 = js_min(u0, n.u0);
                    u1 = js_max(u1, n.u1);
                }
            }
            if !ok && u1 > u0 {
                ok = f.cu >= u0 - tol && f.cu <= u1 + tol && u1 - u0 >= stand_foot * (f.u_max - f.u_min);
            }
            if !ok {
                ok = grip > 0.0 && f.area <= hang * grip;
            }
            if ok {
                held[(f.fid >> 5) as usize] |= 1 << (f.fid & 31);
                changed = true;
            }
        }
    }
    unsafe { &*core::ptr::addr_of!(HELD_BITS) }
}
/// …for a tool
pub fn held_bits(g: usize, bi: u32, without: i32) -> &'static [u32; 21] {
    let p = pn(g);
    match p.row_of(bi) {
        Some(ri) if p.row(ri).ty == ANNEALED => held_set(p.row(ri), without),
        _ => unsafe {
            *core::ptr::addr_of_mut!(HELD_BITS) = [0; 21];
            &*core::ptr::addr_of!(HELD_BITS)
        },
    }
}
/// glassbreak.ts supportPass: what a change left unheld falls, a beat later and staggered
fn support_pass(c: &Ctx, ri: usize) {
    let (frags, key, lu, lv) = {
        let r = pn(c.g).row(ri);
        if r.ty != ANNEALED || r.n_open == 0 {
            return;
        }
        (frags_of(&r.d, &r.w, r.hu, r.hv), r.key as f64, r.last_uv[0], r.last_uv[1])
    };
    let held = *held_set(pn(c.g).row(ri), -1);
    let (per, jit) = (k(GB_STAGGER_PER_M), k(GB_STAGGER_JIT));
    for f in frags.list() {
        let r = pn(c.g).row(ri);
        if bit(&r.w, GW_OPEN, f.fid) || held[(f.fid >> 5) as usize] >> (f.fid & 31) & 1 != 0 || bit(&r.w, GW_GOING, f.fid) {
            continue;
        }
        let d = hypot2(f.cu - lu, f.cv - lv);
        queue(c, ri, f.fid, c.now + d * per + jit * hash01(key * 31.0 + f.fid as f64), C_SUPPORT, [0.0; 3], 1.0);
    }
}

// ---- per frame
/// glassbreak.ts update's top: a new frame's budget
#[inline]
pub fn frame_begin(g: usize) {
    pn(g).released = 0;
}
/// who may rake (host's: men on their feet with their hands free): x y z, vx vy vz, id
pub const MAN: usize = 7;
pub const MAX_MEN: usize = 16;
static mut MEN: [f64; MAX_MEN * MAN] = [0.0; MAX_MEN * MAN];
static mut MEN_N: usize = 0;
#[inline]
pub fn men_addr() -> usize {
    core::ptr::addr_of!(MEN) as usize
}
#[inline]
pub fn men_set(n: usize) {
    unsafe { MEN_N = if n < MAX_MEN { n } else { MAX_MEN } }
}
/// glassbreak.ts update, one pane's part
pub fn step_row(c: &Ctx, ri: usize) {
    {
        let r = pn(c.g).row(ri);
        if gone(r) {
            return;
        }
        if dicing(r) {
            step_dice(c, ri);
            return;
        }
    }
    let now = c.now;
    if pn(c.g).row(ri).qn > 0 {
        let mut went = false;
        let budget = k(GB_RELEASES_PER_FRAME);
        loop {
            let p = pn(c.g);
            let (qn, q0) = {
                let r = p.row(ri);
                (r.qn, r.q0 as usize)
            };
            if qn == 0 || p.q(q0).at > now || p.released as f64 >= budget {
                break;
            }
            let e = p.q_pop(ri);
            let r = p.row(ri);
            let exists = frags_of(&r.d, &r.w, r.hu, r.hv).by(e.fid).map(|f| (f.b, f.i));
            if let Some((b, i)) = exists {
                if !bit(&r.w, GW_OPEN, e.fid) {
                    let from = if e.cause == C_BLAST || e.cause == C_RAKE { Some(e.from) } else { None };
                    if b == 0 {
                        release_crater(c, ri, i as usize, e.cause);
                    } else {
                        release_frag(c, ri, e.fid, e.cause, from, e.k);
                    }
                    pn(c.g).released += 1;
                    went = true;
                }
            }
        }
        let r = pn(c.g).row(ri);
        if went && r.next_think <= 0.0 {
            r.next_think = now + k(GB_THINK);
        }
    }
    {
        let r = pn(c.g).row(ri);
        if r.next_think > 0.0 && now >= r.next_think {
            r.next_think = 0.0;
            unsafe { TALLY.thinks += 1.0 };
            support_pass(c, ri);
        }
    }
    let rake_it = {
        let r = pn(c.g).row(ri);
        run(r) && r.walkable == 0 && k(GB_RAKE_OPEN) > 0.0 && r.ty == ANNEALED
    };
    if rake_it {
        let n = unsafe { MEN_N };
        for m in 0..n {
            let men = unsafe { &*core::ptr::addr_of!(MEN) };
            rake_by(c, ri, &men[m * MAN..m * MAN + MAN]);
        }
    }
}
/// glassbreak.ts rakeBy: a man on his feet walks into what is left of a run pane that is mostly hole: the pieces in his way go, away from him, nearest first
fn rake_by(c: &Ctx, ri: usize, m: &[f64]) {
    let pos = [m[0], m[1], m[2]];
    let vel = [m[3], m[4], m[5]];
    let (frags, mu, key, v_clear) = {
        let r = pn(c.g).row(ri);
        let n = normal_of(r);
        let w = dot3(sub3(pos, r.c), n);
        if js_abs(w) > k(GB_RAKE_M) || pos[1] >= r.c[1] + r.hv {
            return;
        }
        let (mu, _) = to_uv(r, pos[0], pos[1], pos[2]);
        if js_abs(mu) > r.hu + 0.1 {
            return;
        }
        let vn = -dot3(vel, n) * sign_or_one(w);
        let sp = hypot2(vel[0], vel[2]);
        if vn <= 0.3 || vn < 0.6 * sp {
            return;
        }
        (frags_of(&r.d, &r.w, r.hu, r.hv), mu, r.key as f64, k(GB_WALK_CLEAR_H) - r.c[1])
    };
    {
        let p = pn(c.g);
        let (q0, qn) = {
            let r = p.row(ri);
            (r.q0 as usize, r.qn as usize)
        };
        for i in 0..qn {
            if p.q(q0 + i).cause == C_RAKE {
                return;
            }
        }
        if open_fraction_row(p.row(ri)) < k(GB_RAKE_OPEN) {
            return;
        }
    }
    let from = [pos[0], 1.0, pos[2]];
    let mut took = 0.0;
    for f in frags.list() {
        let r = pn(c.g).row(ri);
        if bit(&r.w, GW_OPEN, f.fid) || f.v_min >= v_clear || bit(&r.w, GW_GOING, f.fid) {
            continue;
        }
        if tooth(f) {
            continue;
        }
        queue(c, ri, f.fid, c.now + 0.04 + 0.12 * js_abs(f.cu - mu) + 0.05 * hash01(key * 71.0 + f.fid as f64), C_RAKE, from, 1.0);
        took += 1.0;
    }
    if took > 0.0 {
        pn(c.g).row(ri).who = m[6];
        unsafe { TALLY.raked += took };
    }
}

// ---- questions
/// glassbreak.ts openFraction of a row
fn open_fraction_row(r: &Row) -> f64 {
    if gone(r) {
        return 1.0;
    }
    if dicing(r) {
        return r.d_next as f64 / (r.w[GW_COLS] * r.w[GW_ROWS]) as f64;
    }
    if r.n_open == 0 {
        return 0.0;
    }
    let mut a = 0.0;
    for f in frags_of(&r.d, &r.w, r.hu, r.hv).list() {
        if bit(&r.w, GW_OPEN, f.fid) {
            a += f.area;
        }
    }
    js_min(1.0, a / (4.0 * r.hu * r.hv))
}
pub fn open_fraction(g: usize, bi: u32) -> f64 {
    let p = pn(g);
    match p.row_of(bi) {
        Some(ri) => open_fraction_row(p.row(ri)),
        None => 0.0,
    }
}
/// glassbreak.ts walkable
#[inline]
pub fn walkable(g: usize, bi: u32) -> bool {
    let p = pn(g);
    match p.row_of(bi) {
        Some(ri) => p.row(ri).walkable != 0,
        None => false,
    }
}
/// glassbreak.ts openSig: a number that changes whenever what is open in any pane changes
pub fn open_sig(g: usize) -> i32 {
    let p = pn(g);
    let mut s = p.n as i32;
    for ri in 0..p.n as usize {
        let r = p.row(ri);
        let v = r.w[0] as i32 * 7 + r.n_open as i32 + (if dicing(r) { r.d_next as i32 } else { 0 }) + (if gone(r) { 4096 } else { 0 });
        s = s.wrapping_mul(31).wrapping_add(v);
    }
    s
}
/// glassbreak.ts openAt: is the point where a piece of static `bi` has fallen out
pub fn open_at(g: usize, bi: u32, x: f64, y: f64, z: f64) -> bool {
    let p = pn(g);
    match p.row_of(bi) {
        Some(ri) => {
            let r = p.row(ri);
            let (u, v) = to_uv(r, x, y, z);
            open_uv(r, u, v)
        }
        None => false,
    }
}
/// A LINE OR A RAY OF THIS MODULE MEETS STATIC `bi`: glass.rs `open`, reading the pane's row where it stands
#[inline]
pub fn row_for(g: usize, bi: u32) -> Option<(&'static [f64], &'static [u32])> {
    let p = pn(g);
    let ri = p.row_of(bi)?;
    let r: &'static Row = p.row(ri);
    if r.n_open == 0 && r.qn == 0 && !dicing(r) && !gone(r) {
        return None;
    }
    Some((&r.d, &r.w))
}
/// glassbreak.ts wouldGive, the view's half: would a burst of this size take pane `bi` out of the way at (x, y, z). `sure`: a man counting on it (the near reach alone), else a wary one (the whole pane or nothing).
pub fn gives(g: usize, o: &World, bi: usize, x: f64, y: f64, z: f64) -> bool {
    let (at, radius, power, sure) = ([o.view_at[0], o.view_at[1], o.view_at[2]], o.view_at[3], o.view_at[4], o.view_at[5] != 0.0);
    let p = pn(g);
    let s = match p.sheet_of(bi as u32) {
        Some(s) => s,
        None => return false,
    };
    let b = rec(o, bi);
    if !thin(b) || !(radius > 0.0) || !(power > 0.0) {
        return false;
    }
    let q = nearest_on(b, at);
    let kk = power * (1.0 - hypot3(q[0] - at[0], q[1] - at[1], q[2] - at[2]) / radius);
    if kk < js_min(js_min(k(GB_BLAST_DICE_K), k(GB_BLAST_SHEET_K)), if sure { k(GB_TOOTH_K) } else { BLAST_MIN_K }) {
        return false;
    }
    let (over, sky) = {
        let h = p.sheet(s);
        (h.over, h.sky)
    };
    // (every sheet's sky was settled as the view went on: panes_view)
    let ty = if over != NO_OVER {
        over
    } else {
        let outside = sky != 0 && sky != SKY_UNKNOWN;
        let (h0, h1, h2) = (b[R_H], b[R_H + 1], b[R_H + 2]);
        if if outside { h1 < 0.4 } else { js_max(h0, h2) < 0.35 } {
            LAMINATED
        } else if !truthy(GR_PLACE) {
            if outside {
                TEMPERED
            } else {
                ANNEALED
            }
        } else {
            let foot = b[R_C + 1] - h1;
            let gap = p.sheet(s).gap;
            if foot < k(GR_LOW_M) || (foot < k(GR_DOOR_FOOT_M) && gap <= k(GR_DOOR_M)) {
                TEMPERED
            } else {
                ANNEALED
            }
        }
    };
    if ty == TEMPERED {
        return kk > k(GB_BLAST_DICE_K);
    }
    if ty == LAMINATED {
        return kk > k(GB_BLAST_SHEET_K);
    }
    if !sure {
        return kk > BLAST_MIN_K;
    }
    let reach = blast_reach(kk);
    kk >= k(GB_TOOTH_K) && reach >= 0.0 && hypot3(x - q[0], y - q[1], z - q[2]) <= reach
}
/// as a view goes on: which glass each pane is, settled before any line is judged through it (a first answer casts solid rays of its own)
pub fn settle_sky(c: &Ctx) {
    for s in 0..pn(c.g).sheets as usize {
        sky_side(c, s);
    }
}
/// the sheet's numbers for the host: its static · sky · word · gap
pub fn sheet_count(g: usize) -> u32 {
    pn(g).sheets
}
pub fn sheet_bi(g: usize, s: usize) -> u32 {
    pn(g).sheet(s).bi
}
/// is `bi` a sheet the wind should ask of: its face to the sky, or 0
pub fn sky_known(c: &Ctx, bi: u32) -> i32 {
    match pn(c.g).sheet_of(bi) {
        Some(s) => sky_side(c, s),
        None => 0,
    }
}
/// the head's flags, and they are read: a pane on an outside wall has changed
pub fn take_gust(g: usize) -> bool {
    let p = pn(g);
    let f = p.flags & FL_GUST != 0;
    p.flags &= !FL_GUST;
    f
}
/// the floor's last 'glass' event per pane per shooter (glass.ts liveGap): a hit on this pane by this shooter may be told now. Entries past the gap say nothing and go.
pub fn heard_gap(g: usize, pane: u32, who: f64, now: f64, gap: f64) -> bool {
    let p = pn(g);
    let key = pane as f64 * 1024.0 + (int32(who) & 1023) as f64;
    let mut w = 0;
    let mut said = false;
    for i in 0..p.nev as usize {
        let (kk, t) = (p.ev[2 * i], p.ev[2 * i + 1]);
        let since = now - t;
        if kk == key {
            said = since >= 0.0 && since < gap;
        }
        if since >= 0.0 && since < gap {
            p.ev[2 * w] = kk;
            p.ev[2 * w + 1] = t;
            w += 1;
        }
    }
    p.nev = w as u32;
    if said {
        return false;
    }
    if (w) < EVENTS {
        p.ev[2 * w] = key;
        p.ev[2 * w + 1] = now;
        p.nev += 1;
    }
    true
}
pub fn forget_heard(g: usize) {
    let p = pn(g);
    if p.used != 0 {
        p.nev = 0;
    }
}
/// where each of a row's fields stands (bytes from its start), for the host's reads, and how long a row is
pub fn field(i: u32) -> usize {
    match i {
        0 => offset_of!(Row, d),
        1 => offset_of!(Row, w),
        2 => offset_of!(Row, e),
        3 => offset_of!(Row, hits),
        4 => offset_of!(Row, c),
        5 => offset_of!(Row, hu),
        6 => offset_of!(Row, hv),
        7 => offset_of!(Row, ht),
        8 => offset_of!(Row, energy),
        9 => offset_of!(Row, damage),
        10 => offset_of!(Row, next_think),
        11 => offset_of!(Row, last_dir),
        12 => offset_of!(Row, last_uv),
        13 => offset_of!(Row, sound_at),
        14 => offset_of!(Row, who),
        15 => offset_of!(Row, key),
        16 => offset_of!(Row, ty),
        17 => offset_of!(Row, u_axis),
        18 => offset_of!(Row, exterior),
        19 => offset_of!(Row, hit_n),
        20 => offset_of!(Row, n_open),
        21 => offset_of!(Row, d_next),
        22 => offset_of!(Row, walkable),
        23 => offset_of!(Row, q0),
        24 => offset_of!(Row, qn),
        25 => size_of::<Row>(),
        26 => size_of::<Q>(),
        27 => offset_of!(Q, at),
        28 => offset_of!(Q, from),
        29 => offset_of!(Q, k),
        30 => offset_of!(Q, fid),
        31 => offset_of!(Q, cause),
        _ => 0,
    }
}
/// a row's place from the block's start, in bytes; the queue's; how many rows there are
pub fn row_addr(g: usize, ri: usize) -> usize {
    let p = pn(g);
    let at = p.rows_at() + ri * ROW_W;
    offset_of!(crate::state::State, panes) + HEAD + at * 8
}
pub fn q_addr(g: usize) -> usize {
    let p = pn(g);
    offset_of!(crate::state::State, panes) + HEAD + p.q_at() * 8
}
pub fn row_n(g: usize) -> u32 {
    pn(g).n
}
pub fn row_index(g: usize, bi: u32) -> i32 {
    match pn(g).row_of(bi) {
        Some(r) => r as i32,
        None => -1,
    }
}
/// shards.ts crossed under SHARDS.byFall: where the jet off a sheet's far face comes down (the air has most of a fine's speed inside a stride)
pub fn jet(c: &Ctx, at: [f64; 3], d: [f64; 3], speed: f64, amount: f64, who: f64) {
    let v = [d[0] * (speed * 0.35), d[1] * (speed * 0.35), d[2] * (speed * 0.35)];
    let (t, p) = fall_of(c, at, v, c.gravity, 0.8);
    falls::schedule(c.g, falls::L_GLASS, c.now + t, p[0], p[1], p[2], SOUND_TINKLE, 0.0, 1.0, amount, who, EV_UNHEARD, 0.0, 0.0, k(SH_PENDING_MAX));
}
pub fn hole_rows(g: usize) -> u32 {
    pn(g).nh
}
pub fn hole_bi(g: usize, r: usize) -> u32 {
    pn(g).hole(r).bi
}
pub fn hole_n(g: usize, r: usize) -> u32 {
    pn(g).hole(r).n
}
pub fn hole_pt(g: usize, r: usize, i: usize) -> f64 {
    pn(g).hole(r).p[i]
}
pub fn holes_rev(g: usize) -> u32 {
    pn(g).rev
}
pub fn clear_holes(g: usize) {
    let p = pn(g);
    if p.used != 0 {
        p.clear_holes();
    }
}
/// glassbreak.ts typeOf
pub fn type_place(c: &Ctx, bi: u32) -> u32 {
    let s = match pn(c.g).sheet_of(bi) {
        Some(s) => s,
        None => return ANNEALED,
    };
    let (h0, h1, h2, cy) = {
        let o = world(c.w);
        let b = rec(o, bi as usize);
        (b[R_H], b[R_H + 1], b[R_H + 2], b[R_C + 1])
    };
    type_by_place(c, s, h0, h1, h2, cy)
}
/// glassbreak.ts fill's open threshold: the midpoint between the last clump out and the next in
pub fn dice_threshold(g: usize, bi: u32) -> f64 {
    let p = pn(g);
    let ri = match p.row_of(bi) {
        Some(r) => r,
        None => return -1.0,
    };
    let r = p.row(ri);
    let d = dice_of(r);
    let order = order_of(&d);
    let next = r.d_next as usize;
    if next >= order.len() {
        1e9
    } else if next == 0 {
        -1.0
    } else {
        (order[next - 1].key + order[next].key) / 2.0
    }
}
