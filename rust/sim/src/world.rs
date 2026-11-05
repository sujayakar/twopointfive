//! THE BOX WORLD AS THE MODULE HOLDS IT — the statics a level is made of, the leaves that swing in it, and the grid they are filed in (src/scene/collision.ts
//! StaticCollision), laid out flat for the systems here that ask it things: the line tests (lines.rs) and the circle against the boxes (collide.rs).
//!
//! WHOSE IT IS. While the rules that move a box are TypeScript, a box is a TypeScript object and THAT is the truth: what stands here is a copy, written by
//! src/scene/boxMirror.ts and proven against the objects at the start of every tick (docs/agents/rust.md, "the box world's copy"). Nothing here is state of
//! the game: the stamps and the lists are scratch of one question, and a copy that is thrown away is made again from the objects.
//!
//! No allocation: every table is a fixed size, so the module's memory never grows and a view the TypeScript side holds over it never dies. A level too big for
//! a table is not copied at all (boxMirror.ts says so and the TypeScript answers, as it always did).

/// worlds the module can hold at once (a game a world: two games stepped in one process each keep theirs)
pub const WORLDS: usize = 4;
/// the answer when a question is none this module can judge (a length that is no length of this world's, a pane the host has a rule for, a table too
/// short): the host asks the TypeScript
pub const ASK_HOST: u32 = 0xffff_ffff;
/// statics a world can hold
pub const MAX_BOXES: usize = 8192;
/// what snow a static is (the table `snow`; snowBog.ts SNOW_KIND)
pub const SNOW_BARE: u8 = 1;
pub const SNOW_PACKED: u8 = 2;
pub const SNOW_OLD: u8 = 3;
pub const SNOW_BANK: u8 = 4;
/// loose boxes in one list of them, and how many lists are kept: the game's own leaves, the leaves a question swaps in for itself, and THE FURNITURE'S
/// PARTS (list THING_SET: every part box of every thing in the level, the things in the order of the furniture's own list, a thing's parts together)
pub const MAX_LEAVES: usize = 512;
pub const LEAF_SETS: usize = 3;
pub const THING_SET: usize = 2;
/// things of furniture a world can hold (props.rs CAP), the doubles kept of each BY ITS ID (where it stands, x and z; its yaw; the radius of the upright
/// cylinder that holds it), and the words of each IN THE LEVEL NOW, in the furniture's order (its id, where its parts start in list THING_SET, how many)
pub const MAX_THINGS: usize = 128;
pub const TH: usize = 4;
pub const TH_X: usize = 0;
pub const TH_Z: usize = 1;
pub const TH_YAW: usize = 2;
pub const TH_RADIUS: usize = 3;
pub const TL: usize = 3;
/// grid cells (W × H), and box entries over all of them
pub const MAX_CELLS: usize = 16384;
pub const MAX_ITEMS: usize = 131072;
/// tilted statics (filed by their yawed footprint alone: lines.rs asks them first)
pub const MAX_TILTED: usize = 256;
/// cut records for the statics (a leaf has its own beside its row)
pub const MAX_CUTS: usize = 1024;
/// the most planes one box carries (src/scene/cut.ts MAX_CUT_PLANES)
pub const CUT_PLANES: usize = 8;
/// panes one line of fire can list
pub const MAX_PANES: usize = 256;
/// statics one gather can list (the host's own lists are shorter)
pub const MAX_LIST: usize = 4096;
/// PANES OF GLASS THAT ARE NO LONGER WHOLE (glass.rs): a row a pane with anything out of it, on its way out, coming down or gone — written by the side
/// that still owns the breaking (game/glassbreak.ts, through boxMirror.ts), read by every line and ray here that meets the pane. A pane with no row is
/// shut everywhere. GD doubles and GW words a row (glass.rs names them).
pub const MAX_GLASS: usize = 256;
pub const GD: usize = 80;
pub const GW: usize = 64;
/// panes a burst's own reckoning of the glass can name (glass.rs, the view): where each gives, four doubles a pane
pub const MAX_VIEW: usize = 256;
pub const VD: usize = 4;

/// DOUBLES A BOX: centre, half sizes, cos and sin of its yaw (worked out by the side that owns the maths in place — the same two numbers the TypeScript's own
/// cache keeps beside the yaw), the yaw itself, the rotation (x y z w), its cut record's place and plane count, and what kind of box it is.
pub const REC: usize = 16;
pub const R_C: usize = 0;
pub const R_H: usize = 3;
pub const R_COS: usize = 6;
pub const R_SIN: usize = 7;
pub const R_YAW: usize = 8;
pub const R_ROT: usize = 9;
pub const R_CUT: usize = 13;
pub const R_CUTN: usize = 14;
pub const R_KIND: usize = 15;
/// the kinds (R_KIND, a sum): fully rotated (Box.rot), round (Box.round === true), cut (a cut record with the cut rule on), and a pane of glass a walker
/// goes through (the host's rule: its answer for this box when the row was last written)
pub const K_ROT: u32 = 1;
pub const K_ROUND: u32 = 2;
pub const K_CUT: u32 = 4;
pub const K_WALK: u32 = 8;
/// the kinds that say a box is not a plain upright one (a line or a circle then takes the longer way round it)
pub const K_SHAPED: u32 = K_ROT | K_ROUND | K_CUT;

/// the flags of src/scene/boxes.ts BoxFlag that a question here reads
pub const F_NO_SHADOW: u32 = 2;
pub const F_GLASS: u32 = 1 << 17;

#[repr(C)]
pub struct World {
    pub rec: [f64; MAX_BOXES * REC],
    pub leaf: [f64; LEAF_SETS * MAX_LEAVES * REC],
    /// CUT_PLANES × (n.x n.y n.z d) a record: the statics' records, then one a leaf
    pub cut: [f64; (MAX_CUTS + LEAF_SETS * MAX_LEAVES) * CUT_PLANES * 4],
    pub pane_t: [f64; MAX_PANES],
    /// the furniture, a thing an id (TH doubles each)
    pub thing: [f64; MAX_THINGS * TH],
    /// the panes that are no longer whole, GD doubles a row
    pub glass: [f64; MAX_GLASS * GD],
    /// a burst's view of the glass, VD doubles a pane it names
    pub view: [f64; MAX_VIEW * VD],
    /// where the last circle pushed out ended (x, z)
    pub pt: [f64; 2],
    pub cs: f64,
    pub flags: [u32; MAX_BOXES],
    pub leaf_flags: [u32; LEAF_SETS * MAX_LEAVES],
    /// a box met by this question already (its id), as the TypeScript's `stamp`
    pub stamp: [u32; MAX_BOXES],
    /// the grid: cell c holds items[start[c] .. start[c + 1]], in the order they were filed
    pub start: [u32; MAX_CELLS + 1],
    pub items: [u32; MAX_ITEMS],
    pub cell_stamp: [u32; MAX_CELLS],
    pub seg_cells: [u32; MAX_CELLS],
    /// five a tilted static: its index, then the cells it is filed in (x0 x1 z0 z1)
    pub tilted: [i32; MAX_TILTED * 5],
    pub pane_bi: [u32; MAX_PANES],
    /// the statics the last gather listed
    pub list: [u32; MAX_LIST],
    /// the furniture in the level now, in its own order (TL words each)
    pub thing_live: [u32; MAX_THINGS * TL],
    /// …and how many
    pub things: u32,
    /// the panes that are no longer whole, GW words a row; which row is static i's (0 = none: the pane is whole; else the row + 1); how many rows
    pub glass_w: [u32; MAX_GLASS * GW],
    pub glass_of: [u32; MAX_BOXES],
    pub glass_n: u32,
    /// a burst's view: which statics it names, how many, and which kind of view it is (glass.rs VIEW_*)
    pub view_bi: [u32; MAX_VIEW],
    pub view_n: u32,
    pub view_kind: u32,
    /// WHAT SNOW A STATIC IS, a byte a static (snowBog.ts ground: 0 none · SNOW_BARE ground somebody cleared · SNOW_PACKED trodden · SNOW_OLD old · SNOW_BANK snow
    /// that stands or caps): where a thrown thing bogs down is asked of these and of the statics' own rows (items.rs bog_at)
    pub snow: [u8; MAX_BOXES],
    pub w: u32,
    pub h: u32,
    pub boxes: u32,
    pub n_tilted: u32,
    pub leaves: [u32; LEAF_SETS],
    pub stamp_id: u32,
    pub panes: u32,
}

const BLANK: World = World {
    rec: [0.0; MAX_BOXES * REC],
    leaf: [0.0; LEAF_SETS * MAX_LEAVES * REC],
    cut: [0.0; (MAX_CUTS + LEAF_SETS * MAX_LEAVES) * CUT_PLANES * 4],
    pane_t: [0.0; MAX_PANES],
    thing: [0.0; MAX_THINGS * TH],
    glass: [0.0; MAX_GLASS * GD],
    view: [0.0; MAX_VIEW * VD],
    pt: [0.0; 2],
    cs: 0.0,
    flags: [0; MAX_BOXES],
    leaf_flags: [0; LEAF_SETS * MAX_LEAVES],
    stamp: [0; MAX_BOXES],
    start: [0; MAX_CELLS + 1],
    items: [0; MAX_ITEMS],
    cell_stamp: [0; MAX_CELLS],
    seg_cells: [0; MAX_CELLS],
    tilted: [0; MAX_TILTED * 5],
    pane_bi: [0; MAX_PANES],
    list: [0; MAX_LIST],
    thing_live: [0; MAX_THINGS * TL],
    things: 0,
    glass_w: [0; MAX_GLASS * GW],
    glass_of: [0; MAX_BOXES],
    glass_n: 0,
    view_bi: [0; MAX_VIEW],
    view_n: 0,
    view_kind: 0,
    snow: [0; MAX_BOXES],
    w: 0,
    h: 0,
    boxes: 0,
    n_tilted: 0,
    leaves: [0; LEAF_SETS],
    stamp_id: 0,
    panes: 0,
};
static mut ALL: [World; WORLDS] = [BLANK; WORLDS];

/// world `w` (one thread, and no question is asked inside another: the TypeScript's own rule for its stamps)
#[inline(always)]
pub fn world(w: usize) -> &'static mut World {
    unsafe { &mut (*core::ptr::addr_of_mut!(ALL))[if w < WORLDS { w } else { 0 }] }
}

impl World {
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

/// WHERE A TABLE OF WORLD `w` IS in the module's memory (a byte address), for the side that writes the copy
pub fn addr(w: usize, table: u32) -> usize {
    let o = world(w);
    match table {
        0 => o.rec.as_ptr() as usize,
        1 => o.leaf.as_ptr() as usize,
        2 => o.cut.as_ptr() as usize,
        3 => o.pane_t.as_ptr() as usize,
        4 => o.flags.as_ptr() as usize,
        5 => o.leaf_flags.as_ptr() as usize,
        6 => o.start.as_ptr() as usize,
        7 => o.items.as_ptr() as usize,
        8 => o.tilted.as_ptr() as usize,
        9 => o.pane_bi.as_ptr() as usize,
        12 => o.pt.as_ptr() as usize,
        13 => o.list.as_ptr() as usize,
        14 => o.thing.as_ptr() as usize,
        15 => o.thing_live.as_ptr() as usize,
        16 => o.glass.as_ptr() as usize,
        17 => o.glass_w.as_ptr() as usize,
        18 => o.glass_of.as_ptr() as usize,
        19 => o.view.as_ptr() as usize,
        20 => o.view_bi.as_ptr() as usize,
        21 => o.snow.as_ptr() as usize,
        _ => 0,
    }
}
/// how much a table holds, by the same numbers (rows, not bytes)
pub fn cap(table: u32) -> usize {
    match table {
        0 => MAX_BOXES,
        1 => MAX_LEAVES,
        2 => MAX_CUTS,
        3 => MAX_PANES,
        6 => MAX_CELLS,
        7 => MAX_ITEMS,
        8 => MAX_TILTED,
        10 => WORLDS,
        11 => LEAF_SETS,
        13 => MAX_LIST,
        14 => MAX_THINGS,
        16 => MAX_GLASS,
        19 => MAX_VIEW,
        21 => MAX_BOXES,
        _ => 0,
    }
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
/// Math.PI / 180 (src/math/vec.ts DEG)
pub const DEG: f64 = 0.017453292519943295;
