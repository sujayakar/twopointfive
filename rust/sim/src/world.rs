//! WHAT A QUESTION OF THE BOX WORLD STANDS ON: the rows it reads (game_types' boxworld: written by whoever moves a box), its scratch, and the small operations whose answers are the game's own.
pub use game_types::boxworld::*;
use alloc::vec::Vec;

/// the answer when a question is none this module can judge (a length that is no length of this world's, a table too short)
pub const ASK_HOST: u32 = 0xffff_ffff;
/// panes one line of fire can list
pub const MAX_PANES: usize = 256;
/// statics one gather can list (the caller's own lists are shorter)
pub const MAX_LIST: usize = 4096;

/// places for what last stopped a line (a power of two)
pub const LASTS: usize = 1 << 13;

/// THE SCRATCH OF ONE QUESTION, beside the boxes and no part of them: what it has met already, the cells it walked, the panes and statics it lists. One for
/// whoever asks (one thread, and no question is asked inside another): so a question READS the boxes (`&World`) and writes nothing that is anybody's.
pub struct Asking {
    /// a box met by this question already (its id)
    pub stamp: [u32; MAX_BOXES],
    pub cell_stamp: [u32; MAX_CELLS],
    pub seg_cells: [u32; MAX_CELLS],
    pub pane_t: [f64; MAX_PANES],
    pub pane_bi: [u32; MAX_PANES],
    /// the statics the last gather listed
    pub list: [u32; MAX_LIST],
    /// where the last circle pushed out ended (x, z)
    pub pt: [f64; 2],
    pub stamp_id: u32,
    pub panes: u32,
    /// what last stopped a line from about here to about there (lines.rs `stopped_by`): a guess, asked first, and never an answer
    pub last: [u32; LASTS],
}
const IDLE: Asking = Asking { stamp: [0; MAX_BOXES], cell_stamp: [0; MAX_CELLS], seg_cells: [0; MAX_CELLS], pane_t: [0.0; MAX_PANES], pane_bi: [0; MAX_PANES], list: [0; MAX_LIST], pt: [0.0; 2], stamp_id: 0, panes: 0, last: [0; LASTS] };
static mut ASKING: Asking = IDLE;
/// the scratch of the question in hand
#[inline(always)]
pub fn asking() -> &'static mut Asking { unsafe { &mut *core::ptr::addr_of_mut!(ASKING) } }

impl Asking {
    /// a fresh id for one question's stamps; at the wrap every stamp is wiped so that no old one reads as this question's
    #[inline]
    pub fn ask(&mut self) -> u32 {
        self.stamp_id = self.stamp_id.wrapping_add(1);
        if self.stamp_id == 0 {
            self.stamp = [0; MAX_BOXES];
            self.cell_stamp = [0; MAX_CELLS];
            self.stamp_id = 1;
        }
        self.stamp_id
    }
    /// glass static `bi` is across the line in hand from `t0` along it (0 if the line starts inside): onto the list; false = the list is full
    #[inline]
    pub fn note(&mut self, bi: usize, t0: f64) -> bool {
        let at = self.panes as usize;
        if at >= MAX_PANES {
            return false;
        }
        self.pane_bi[at] = bi as u32;
        self.pane_t[at] = js_max(0.0, t0);
        self.panes += 1;
        true
    }
}

/// THE STATICS AS A TABLE MADE FROM THEM LAST FOUND THEM: a word of each one's row, and what it takes of the plan (the table's own rectangle of it)
#[derive(Default)]
pub struct Seen {
    wrote: u64,
    of: Vec<(u64, [f64; 5])>,
}
impl Seen {
    /// nothing was seen, and there is something to see
    pub fn empty_of(&self, o: &World) -> bool { self.of.is_empty() && self.wrote != o.wrote }
    pub fn forget(&mut self) { *self = Seen::default(); }
    /// the statics looked at again: `moved` is told the rectangle of every one that is not as it was and how high it reaches, where it stood and where it stands
    pub fn look(&mut self, o: &World, round: impl Fn(&[f64]) -> [f64; 5], mut moved: impl FnMut([f64; 5])) {
        if self.wrote == o.wrote { return; }
        self.wrote = o.wrote;
        let n = o.boxes();
        for i in 0..n.max(self.of.len()) {
            let had = self.of.get(i).copied();
            let word = if i < n { word_of(o, i) } else { 0 };
            if i < n && had.is_some_and(|(was, _)| was == word) { continue; }
            if let Some((_, at)) = had { moved(at); }
            if i < n {
                let now = (word, round(o.row(i)));
                if let Some(was) = self.of.get_mut(i) { *was = now; } else { self.of.push(now); }
                moved(now.1);
            }
        }
        self.of.truncate(n);
    }
}
/// a word of static `i`'s row: its sixteen numbers, its planes and its flags (what it sends back is read as it is)
fn word_of(o: &World, i: usize) -> u64 {
    let mut h: u64 = 0xcbf29ce484222325 ^ o.flags[i] as u64;
    let r = &o.rec[i * REC..(i + 1) * REC];
    let planes = if r[R_KIND] as u32 & K_CUT != 0 { &o.cut[r[R_CUT] as usize * CUT_PLANES * 4..][..js_min(r[R_CUTN], CUT_PLANES as f64) as usize * 4] } else { &[] };
    for v in r.iter().chain(planes) {
        h = (h ^ v.to_bits()).wrapping_mul(0x9e3779b97f4a7c15);
        h ^= h >> 29;
    }
    h
}

// ---- small operations whose every edge is the game's: a NaN and the sign of a zero are part of the answer, and the language's own differ
/// `x < 0 ? -x : x` (a −0 stays −0, where `abs` would clear it)
#[inline(always)]
pub fn neg_up(x: f64) -> f64 {
    if x < 0.0 {
        -x
    } else {
        x
    }
}
/// the ceiling (a −0.x is −0)
#[inline(always)]
pub fn js_ceil(x: f64) -> f64 {
    -prim::floor(-x)
}
/// the lesser of two: NaN if either is, and −0 under +0
#[inline(always)]
pub fn js_min(a: f64, b: f64) -> f64 {
    if a != a || b != b {
        return f64::NAN;
    }
    if a < b {
        a
    } else if b < a {
        b
    } else if a == 0.0 && b == 0.0 {
        f64::from_bits(a.to_bits() | b.to_bits())
    } else {
        a
    }
}
/// the greater of two: NaN if either is, and +0 over −0
#[inline(always)]
pub fn js_max(a: f64, b: f64) -> f64 {
    if a != a || b != b {
        return f64::NAN;
    }
    if a > b {
        a
    } else if b > a {
        b
    } else if a == 0.0 && b == 0.0 {
        f64::from_bits(a.to_bits() & b.to_bits())
    } else {
        a
    }
}
/// −1 under nought, else 1 (a zero and a NaN too)
#[inline(always)]
pub fn sign_or_one(x: f64) -> f64 {
    if x < 0.0 {
        -1.0
    } else {
        1.0
    }
}
/// the greatest of three, each pair as `js_max` takes it
#[inline(always)]
pub fn js_max3(a: f64, b: f64, c: f64) -> f64 {
    js_max(js_max(a, b), c)
}
/// the size of x: a −0 is cleared, NaN is NaN
#[inline(always)]
pub fn js_abs(x: f64) -> f64 {
    if x < 0.0 {
        -x
    } else if x == 0.0 {
        0.0
    } else {
        x
    }
}
/// `x || 1`: 1 for a zero of either sign and for NaN
#[inline(always)]
pub fn or_one(x: f64) -> f64 {
    if x == 0.0 || x != x {
        1.0
    } else {
        x
    }
}
/// `x < a ? a : x > b ? b : x` (NaN passes)
#[inline(always)]
pub fn clamp(x: f64, a: f64, b: f64) -> f64 {
    if x < a {
        a
    } else if x > b {
        b
    } else {
        x
    }
}
#[inline(always)]
pub fn smoothstep(a: f64, b: f64, x: f64) -> f64 {
    let t = clamp((x - a) / (b - a), 0.0, 1.0);
    t * t * (3.0 - 2.0 * t)
}
/// π / 180
pub const DEG: f64 = 0.017453292519943295;
