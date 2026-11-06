//! WHERE A PANE OF GLASS IS OPEN — the one question every line and ray of this module asks of a pane it meets (src/game/glassbreak.ts openAt, goingAt and
//! the reckoning a burst makes of glass it has not broken yet; src/game/glassPattern.ts gpClassify and gpDiceClump, the geometry under them).
//!
//! STATEMENT FOR STATEMENT with the TypeScript: the same hashes, the same bearings and chords worked out by the same maths (maths.rs), the same compares.
//! tools/qa/glass-ident.ts holds the two together over panes broken by a game.
//!
//! WHOSE THE STATE IS. The game's block's (panes.rs): a line or a ray reads a pane's row where it stands, through `World.panes_g`. A pane with no row is whole: shut
//! everywhere. (A world with no game behind it, a tool's, has a copy here, written by src/scene/boxMirror.ts.)
//!
//! A ROW. Doubles (GD): nine a site, eight sites (u, v, the bearing of radial 0, the four base radii, the ring zone's radius, the share of merged
//! sectors), then the dice of a toughened pane coming down (the blow's u and v, the pane's half sides, the key of the last clump out and which clump
//! that is). Words (GW): the
//! static's index, the flags and the site count, the dice's columns, rows and seed, each site's radials and seed, a bit a piece that is out, a bit a
//! piece that is queued to leave, and a bit a site whose crater is.
//!
//! A BURST'S VIEW. A blast judges its own lines by glass as it is to a burst (game/frag.ts throughGlass): what the blast has just taken is no glass to its
//! fragments (VIEW_BLOWN, answered here from the pieces queued to leave), and a man reckoning a burst that has not happened counts out the panes it
//! would break (VIEW_GIVES: panes.rs `gives`, from the burst's numbers and the pane's own state, by the rule that breaks them). The view stands for the length of one
//! judgment and is none at every other time.

use crate::maths::{atan2, cos, hypot3, sin};
use crate::world::*;
use prim::floor;

// ---- a row's doubles and words
/// doubles a site, and where the dice's five start
pub const G_SITE: usize = 9;
const S_U: usize = 0;
const S_V: usize = 1;
const S_ROT: usize = 2;
const S_RHO: usize = 3;
const S_RZ: usize = 7;
const S_MERGE: usize = 8;
pub const G_DICE_AT: usize = 72;
const D_U: usize = 0;
const D_V: usize = 1;
const D_HU: usize = 2;
const D_HV: usize = 3;
const D_LAST: usize = 4;
const D_LAST_K: usize = 5;
pub const GW_BI: usize = 0;
pub const GW_FLAGS: usize = 1;
pub const GW_COLS: usize = 2;
pub const GW_ROWS: usize = 3;
pub const GW_DSEED: usize = 4;
pub const GW_N: usize = 5;
pub const GW_SEED: usize = 13;
pub const GW_OPEN: usize = 21;
pub const GW_GOING: usize = 42;
pub const GW_CRATER: usize = 63;
/// the flags (the site count sits above them, × 256): the pane has gone · it is toughened glass coming down · its cracks have run to the frame
pub const G_GONE: u32 = 1;
pub const G_DICE: u32 = 2;
pub const G_RUN: u32 = 4;

/// the views: glass stands where it stands · what a blast has just taken is gone to it · what a burst would break is counted out
pub const VIEW_NONE: u32 = 0;
pub const VIEW_BLOWN: u32 = 1;
pub const VIEW_GIVES: u32 = 2;

// ---- glassPattern.ts, its numbers
const MAX_SECTORS: u32 = 12;
const FRAGS_PER_SITE: u32 = 84;
const ANG_JIT: f64 = 0.6;
const RHO_LO: f64 = 0.78;
const RHO_SPAN: f64 = 0.44;
const SPLIT_LO: f64 = 0.35;
const SPLIT_SPAN: f64 = 0.3;
const CUT_NONE: f64 = 0.45;
const CUT_LO: f64 = 0.9;
const CUT_SPAN: f64 = 1.0;
const CUT_TILT: f64 = 0.9;
const PI: f64 = core::f64::consts::PI;
/// gpClassify's answers that are no piece: no site on the pane · the uncut sheet past chord 2 of a pane that has not run
const NO_SITE: i32 = -1;
const REMAINDER: i32 = -2;

/// gpHash: (seed, a, b) → 24 bits
#[inline]
fn hash(seed: u32, a: u32, b: u32) -> u32 {
    let mut h = seed ^ a.wrapping_add(1).wrapping_mul(0x9E37_79B1) ^ b.wrapping_add(1).wrapping_mul(0x85EB_CA77);
    h = (h ^ (h >> 15)).wrapping_mul(0x2C1B_3C6D);
    h = (h ^ (h >> 12)).wrapping_mul(0x297A_2D39);
    (h ^ (h >> 15)) >> 8
}
/// gpJit
#[inline]
pub fn jit(seed: u32, a: u32, b: u32) -> f64 {
    hash(seed, a, b) as f64 / 16777216.0
}

/// one site of a row (glassPattern.ts GpSite, the fields a piece is found by)
pub struct Site<'a> {
    pub d: &'a [f64],
    pub n: i32,
    pub seed: u32,
}
impl Site<'_> {
    /// `((k % n) + n) % n`
    #[inline]
    fn wrap(&self, k: i32) -> u32 {
        (((k % self.n) + self.n) % self.n) as u32
    }
    /// gpBound: where radial k sits, in sectors
    #[inline]
    fn bound(&self, k: i32) -> f64 {
        k as f64 + ANG_JIT * (jit(self.seed, self.wrap(k), 7) - 0.5)
    }
    /// gpAngle
    #[inline]
    pub fn angle(&self, k: i32) -> f64 {
        self.d[S_ROT] + 2.0 * PI * self.bound(k) / self.n as f64
    }
    /// gpRho: the radius at which chord b meets radial k
    #[inline]
    fn rho(&self, k: i32, b: u32) -> f64 {
        self.d[S_RHO + b as usize] * (RHO_LO + RHO_SPAN * jit(self.seed, self.wrap(k), b))
    }
    /// gpChordPt
    #[inline]
    pub fn chord(&self, k: i32, b: u32) -> (f64, f64) {
        let a = self.angle(k);
        let r = self.rho(k, b);
        (self.d[S_U] + r * cos(a), self.d[S_V] + r * sin(a))
    }
    /// gpMerged: sector s has no middle chord
    #[inline]
    pub fn merged(&self, s: i32) -> bool {
        jit(self.seed, s as u32, 21) < self.d[S_MERGE]
    }
    /// gpSplit: where sector s's branch crack sits, in sectors
    #[inline]
    pub fn split(&self, s: i32) -> f64 {
        let b0 = self.bound(s);
        let b1 = self.bound(s + 1);
        b0 + (b1 - b0) * (SPLIT_LO + SPLIT_SPAN * jit(self.seed, s as u32, 23))
    }
    /// gpCut: the snap across half h of sector s (a point on it, its unit normal away from the site); None = this dagger runs unbroken
    pub fn cut(&self, s: i32, h: u32) -> Option<(f64, f64, f64, f64)> {
        if jit(self.seed, s as u32, 27 + h) < CUT_NONE {
            return None;
        }
        let sp = self.split(s);
        let mid = if h == 0 { (self.bound(s) + sp) / 2.0 } else { (sp + self.bound(s + 1)) / 2.0 };
        let am = self.d[S_ROT] + 2.0 * PI * mid / self.n as f64;
        let rc = self.d[S_RHO + 3] * (CUT_LO + CUT_SPAN * jit(self.seed, s as u32, 25 + h));
        let an = am + (jit(self.seed, s as u32, 29 + h) - 0.5) * CUT_TILT;
        Some((self.d[S_U] + rc * cos(am), self.d[S_V] + rc * sin(am), cos(an), sin(an)))
    }
    /// gpSector: the sector of (u, v) round the site, and its place across the fan
    fn sector(&self, u: f64, v: f64) -> (i32, f64) {
        let n = self.n;
        let nf = n as f64;
        let mut t = (atan2(v - self.d[S_V], u - self.d[S_U]) - self.d[S_ROT]) / (2.0 * PI);
        t = (t - floor(t)) * nf;
        let s0 = js_min(nf - 1.0, floor(t)) as i32;
        if t < self.bound(s0) {
            return if s0 == 0 { (n - 1, t + nf) } else { (s0 - 1, t) };
        }
        if t >= self.bound(s0 + 1) {
            return if s0 == n - 1 { (0, t - nf) } else { (s0 + 1, t) };
        }
        (s0, t)
    }
    /// gpSectorBand
    fn sector_band(&self, u: f64, v: f64) -> (i32, u32) {
        let (s, t) = self.sector(u, v);
        if !beyond(self.chord(s, 0), self.chord(s + 1, 0), u, v) {
            return (s, 0);
        }
        let merged = self.merged(s);
        if !merged && !beyond(self.chord(s, 1), self.chord(s + 1, 1), u, v) {
            return (s, 1);
        }
        if !beyond(self.chord(s, 2), self.chord(s + 1, 2), u, v) {
            return (s, if merged { 1 } else { 2 });
        }
        let h = if t >= self.split(s) { 1 } else { 0 };
        let far = match self.cut(s, h) {
            Some((px, py, nx, ny)) => nx * (u - px) + ny * (v - py) > 0.0,
            None => false,
        };
        (s, (if far { 5 } else { 3 }) + h)
    }
}
/// past the chord A → B
#[inline]
fn beyond(a: (f64, f64), b: (f64, f64), u: f64, v: f64) -> bool {
    (b.0 - a.0) * (v - a.1) - (b.1 - a.1) * (u - a.0) < 0.0
}

/// site i of a row
#[inline]
pub fn site_of<'a>(d: &'a [f64], w: &[u32], i: usize) -> Site<'a> {
    Site { d: &d[i * G_SITE..i * G_SITE + G_SITE], n: w[GW_N + i] as i32, seed: w[GW_SEED + i] }
}
/// gpClassify: the piece (u, v) is in — its id, REMAINDER or NO_SITE
pub fn classify_row(d: &[f64], w: &[u32], u: f64, v: f64) -> i32 {
    let n = (w[GW_FLAGS] >> 8) as usize;
    let (mut best, mut bd) = (usize::MAX, f64::INFINITY);
    for i in 0..n {
        let s = &d[i * G_SITE..i * G_SITE + G_SITE];
        let (du, dv) = (s[S_U] - u, s[S_V] - v);
        let dd = du * du + dv * dv - s[S_RZ] * s[S_RZ];
        if dd < bd {
            bd = dd;
            best = i;
        }
    }
    if best == usize::MAX {
        return NO_SITE;
    }
    let (s, b) = site_of(d, w, best).sector_band(u, v);
    if b >= 3 && w[GW_FLAGS] & G_RUN == 0 {
        return REMAINDER;
    }
    (best as u32 * FRAGS_PER_SITE + b * MAX_SECTORS + s as u32) as i32
}
#[inline]
fn bit(w: &[u32], at: usize, k: u32) -> bool {
    w[at + (k >> 5) as usize] >> (k & 31) & 1 != 0
}

// ---- toughened glass coming down
/// is the clump (u, v) belongs to out: that clump's place in the order clumps let go against the last one out, by their keys and by their numbers where two keys
/// are one (pattern.rs Dice: gpDiceClump, gpDiceKey; glassbreak.ts startDice's sort). So this one number says "earlier in that order than the last one out", at any moment of a release.
pub fn dice_out(d: &[f64], w: &[u32], u: f64, v: f64) -> bool {
    let dc = crate::pattern::Dice { u: d[D_U], v: d[D_V], cols: w[GW_COLS], rows: w[GW_ROWS], seed: w[GW_DSEED], hu: d[D_HU], hv: d[D_HV] };
    if u != u || v != v {
        return false; // (a point that is no number is on no clump)
    }
    let k = dc.clump(u, v);
    let key = dc.key(k % dc.cols, k / dc.cols);
    key < d[D_LAST] || (key == d[D_LAST] && k as f64 <= d[D_LAST_K])
}

/// glassbreak.ts toUV: a world point in the pane's own (u along it, v up)
#[inline]
fn to_uv(b: &[f64], x: f64, y: f64, z: f64) -> (f64, f64) {
    let (cs, sn) = (b[R_COS], b[R_SIN]);
    let (px, pz) = (x - b[R_C], z - b[R_C + 2]);
    let (lx, lz) = (cs * px - sn * pz, sn * px + cs * pz);
    (if b[R_H] >= b[R_H + 2] { lx } else { lz }, y - b[R_C + 1])
}

/// the view's word on a pane it may name: glass a burst would break is no glass to the man reckoning it (glassbreak.ts wouldGive, worked out by its
/// owner into the view's rows: a pane that gives whole has a reach below nought, float glass gives within its reach of the pane's nearest point)
fn gives(o: &World, bi: usize, x: f64, y: f64, z: f64) -> bool {
    if o.panes_g != NO_BLOCK {
        return crate::panes::gives(o.panes_g as usize - 1, o, bi, x, y, z);
    }
    for k in 0..o.view_n as usize {
        if o.view_bi[k] as usize != bi {
            continue;
        }
        let q = &o.view[k * VD..k * VD + VD];
        return q[3] < 0.0 || hypot3(x - q[0], y - q[1], z - q[2]) <= q[3];
    }
    false
}

/// IS STATIC `bi`, A PANE OF GLASS, OPEN AT (x, y, z) — the game's rule (glassbreak.ts: `openAt(bi, X) || also(bi, X)`): a piece has fallen out there, or
/// the view in hand counts the glass there out.
pub fn open(o: &World, bi: usize, x: f64, y: f64, z: f64) -> bool {
    // (where the pane's state stands: its row in the game's block, the rule being this module's own (panes.rs) — or the copy's, the rule being the host's)
    let (d, w): (&[f64], &[u32]) = if o.panes_g != NO_BLOCK {
        match crate::panes::row_for(o.panes_g as usize - 1, bi as u32) {
            Some(row) => row,
            None => return o.view_kind == VIEW_GIVES && gives(o, bi, x, y, z),
        }
    } else {
        let r = o.glass_of[bi] as usize;
        if r == 0 || r > o.glass_n as usize {
            return o.view_kind == VIEW_GIVES && gives(o, bi, x, y, z);
        }
        (&o.glass[(r - 1) * GD..r * GD], &o.glass_w[(r - 1) * GW..r * GW])
    };
    let fl = w[GW_FLAGS];
    if fl & G_GONE != 0 {
        return true;
    }
    let (u, v) = to_uv(&o.rec[bi * REC..bi * REC + REC], x, y, z);
    if fl & G_DICE != 0 {
        // (a pane coming down is on its way out to a blast, and counted out by a man reckoning one)
        return dice_out(&d[G_DICE_AT..], w, u, v) || o.view_kind != VIEW_NONE;
    }
    let fid = classify_row(d, w, u, v);
    if fid >= 0 && bit(w, GW_OPEN, fid as u32) {
        return true;
    }
    match o.view_kind {
        // goingAt: the piece there is queued to leave, or it is a crater's wedge and one of its crater is
        VIEW_BLOWN => fid >= 0 && (bit(w, GW_GOING, fid as u32) || ((fid as u32 % FRAGS_PER_SITE) / MAX_SECTORS == 0 && w[GW_CRATER] >> (fid as u32 / FRAGS_PER_SITE) & 1 != 0)),
        VIEW_GIVES => gives(o, bi, x, y, z),
        _ => false,
    }
}
