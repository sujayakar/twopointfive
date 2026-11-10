//! WHAT A QUESTION OF THE BOX WORLD STANDS ON: the rows it reads (game_types' boxworld: written by whoever moves a box), its scratch, and the small operations whose answers are the game's own.
pub use game_types::boxworld::*;

/// the answer when a question is none this module can judge (a length that is no length of this world's, a table too short)
pub const ASK_HOST: u32 = 0xffff_ffff;
/// panes one line of fire can list
pub const MAX_PANES: usize = 256;
/// statics one gather can list (the caller's own lists are shorter)
pub const MAX_LIST: usize = 4096;
/// doubles and words of a pane's row in the game's block (glass.rs names them)
pub const GD: usize = 80;
pub const GW: usize = 64;

/// places for what last stopped a line (a power of two)
pub const LASTS: usize = 1 << 13;

/// THE SCRATCH OF ONE QUESTION, beside the boxes and no part of them: what it has met already, the cells it walked, the panes and statics it lists. One for
/// whoever asks (one thread, and no question is asked inside another): so a question READS the boxes (`&World`) and writes nothing that is anybody's.
pub struct Asking {
    /// a box met by this question already (its id), as the TypeScript's `stamp`
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
    /// a fresh id for one question's stamps (the TypeScript's `++stampId`); at the wrap every stamp is wiped so that no old one reads as this question's
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
pub struct Seen {
    wrote: u64,
    boxes: usize,
    word: [u64; MAX_BOXES],
    at: [[f64; 4]; MAX_BOXES],
}
impl Seen {
    pub const NONE: Self = Seen { wrote: 0, boxes: 0, word: [0; MAX_BOXES], at: [[0.0; 4]; MAX_BOXES] };
    /// nothing was seen, and there is something to see
    pub fn empty_of(&self, o: &World) -> bool { self.boxes == 0 && self.wrote != o.wrote }
    pub fn forget(&mut self) { (self.wrote, self.boxes) = (0, 0); }
    /// the statics looked at again: `moved` is told the rectangle of every one that is not as it was, where it stood and where it stands
    pub fn look(&mut self, o: &World, round: impl Fn(&[f64]) -> [f64; 4], mut moved: impl FnMut([f64; 4])) {
        if self.wrote == o.wrote { return; }
        self.wrote = o.wrote;
        let n = o.boxes();
        for i in 0..n.max(self.boxes) {
            let (had, has) = (i < self.boxes, i < n);
            let word = if has { word_of(o, i) } else { 0 };
            if had && has && word == self.word[i] { continue; }
            if had { moved(self.at[i]); }
            if has {
                self.at[i] = round(o.row(i));
                self.word[i] = word;
                moved(self.at[i]);
            }
        }
        self.boxes = n;
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

// ---- the TypeScript's own small operations, where the language's differ
/// `x < 0 ? -x : x` as the TypeScript writes it (a −0 stays −0, where `abs` would clear it)
#[inline(always)]
pub fn neg_up(x: f64) -> f64 {
    if x < 0.0 {
        -x
    } else {
        x
    }
}
/// Math.ceil (a −0.x is −0, as there)
#[inline(always)]
pub fn js_ceil(x: f64) -> f64 {
    -prim::floor(-x)
}
/// Math.min of two: NaN if either is, and −0 under +0
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
/// Math.max of two: NaN if either is, and +0 over −0
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
/// Math.sign(x) || 1
#[inline(always)]
pub fn sign_or_one(x: f64) -> f64 {
    if x < 0.0 {
        -1.0
    } else {
        1.0
    }
}
/// Math.max of three, as Math.max(a, b, c) gives it
#[inline(always)]
pub fn js_max3(a: f64, b: f64, c: f64) -> f64 {
    js_max(js_max(a, b), c)
}
/// Math.abs: a −0 is cleared, NaN is NaN
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
/// src/math/vec.ts clamp: `x < a ? a : x > b ? b : x` (NaN passes)
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
/// src/math/vec.ts smoothstep
#[inline(always)]
pub fn smoothstep(a: f64, b: f64, x: f64) -> f64 {
    let t = clamp((x - a) / (b - a), 0.0, 1.0);
    t * t * (3.0 - 2.0 * t)
}
/// Math.PI / 180
pub const DEG: f64 = 0.017453292519943295;
