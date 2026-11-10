//! THE GAME'S OWN SMOKE ON A COARSE GRID: the smoke the SENSES see through, as density in
//! half-metre cells held in bricks (a brick is 2 m square in plan and 4 m high: floor to ceiling), stepped ten times a second. It is fed by the very emitters
//! the picture's solver is handed and knows nothing else of that solver: no velocity and no pressure, and of the wind only a drift under the open sky
//! (`K.wind`). What it has is THE WALLS:
//! smoke passes from a cell to its neighbour only where nothing solid and no leaf stands between their centres, so a canister fills the room it is in, goes
//! out by the open door and stops at the shut one.
//!
//! THE STATE is a part of the game's block (state.rs; `Smoke`): one row of doubles (`LEN`): a head (the plan in bricks, the emitters live, the steps
//! taken), the emitters still letting out, which brick is where, the light that comes round (glow.rs), and LAST the density of every cell, a brick after
//! another. It holds indices and numbers only, so it is the same bytes in any slot, in a snapshot or on another machine. The part is a POOL: its first
//! word says how many of its bytes are the game's (the row up to the end of the highest brick that stands), and the host carries those and no more.
//! WHAT IS NOT STATE and is made again from the state and the box world, a set of tables a slot of the blocks: where each brick is (`INDEX`, from the
//! state alone) and which faces of each cell are open (`FACES`: asked of the box world's lines once a cell; again round every leaf at every step, since a
//! door swings, and wherever a static's row is another, `SEEN`). So the faces are at every step what asking them all afresh would give: a game put back
//! from a snapshot and one that never was step alike. WHOSE the tables of a slot are is the block's stamp they were made from (`MADE`: another number
//! whenever a block was written wholesale or changed place); where it differs the tables are made again before anything is read from them.
//!
//! The arithmetic is + − × ÷ and the game's own exp and length, in an order that is written down (bricks by slot, cells by index): the same bits on every
//! machine.

use game_types::knobs::Knobs;
use crate::lines;
use crate::maths::{exp, hypot3};
use crate::state::{state, state_read, GAMES};
use crate::world::{js_max, js_min, ASK_HOST, K_ROT, MAX_LEAVES, REC, R_C, R_H, R_KIND, World, Seen};
use core::mem::size_of;
use prim::floor;

/// metres a cell
pub const CELL: f64 = 0.5;
/// cells along a brick's side in plan, and up
pub const BX: usize = 4;
pub const BY: usize = 8;
pub const CELLS: usize = BX * BX * BY;
/// THE PLAN A GRID STANDS ON, in bricks, at most, AND the bricks a game can hold: one number, so there is a brick for every place of the plan and smoke is
/// never refused one (a larger plan has no grid: `step`). A canister in the open is in about twenty by the time it is spent.
pub const MAX_BRICKS: usize = 512;
/// emitters letting out at once (the solver's own list holds as many)
pub const MAX_EMIT: usize = 64;
/// DOUBLES AN EMITTER: where it stands (x y z), what it lets out a second (unit·m³), how long it runs, how long it has, how wide it is (m), and what is missing from the
/// color of what it lets out (1 − its tint, r g b: 0 = smoke's own grey)
pub const EMIT: usize = 10;
/// the head: emitters live · steps taken · the plan in bricks, x then z (0 = no step has been taken on any level yet) · THE BOOK OF THE SMOKE, two
/// sums in unit·m³ that thin at the cells' own rate, so that at every step what the cells hold + what was given back = what was let out: all that was let
/// out · what thin bricks held when they were given back, and what the wind carried over the plan's edge (K.wind) · seconds owed toward the next step · a word of nothing (a block is whole vectors long)
pub const HEAD: usize = 8;
const H_EMIT: usize = 0;
pub const H_STEPS: usize = 1;
const H_NBX: usize = 2;
const H_NBZ: usize = 3;
pub const H_OUT: usize = 4;
pub const H_GIVEN: usize = 5;
const H_CLOCK: usize = 6;
pub const O_EMIT: usize = HEAD;
/// which brick is where: its place in plan, x then z, EACH PLUS ONE (0 = the slot is free: a row of zeros is an empty grid)
pub const O_KEYS: usize = O_EMIT + MAX_EMIT * EMIT;
/// WHAT COLOR A BRICK'S SMOKE IS: what is missing from it, r g b, on average over all the smoke in the brick (dust is tan, soot is dark: whoever draws a game put back has no
/// other word of it). No eye and no rule reads it.
pub const O_DYE: usize = O_KEYS + MAX_BRICKS * 2;
/// THE LIGHT UNDER A ROOF (glow.rs), a number a square metre of the plan in five columns: the best of the six faces at its middle · their mean · the mean
/// with the lamps', which is what leaves its surfaces · the face that looks up · the level of the place it is in (a logarithm). Zeros where nothing has swept: no light.
pub const MAX_GLOW: usize = MAX_BRICKS * (SIDE * SIDE) as usize;
pub const O_GLOW_MAX: usize = O_DYE + MAX_BRICKS * 3;
pub const O_GLOW_MEAN: usize = O_GLOW_MAX + MAX_GLOW;
pub const O_GLOW_A: usize = O_GLOW_MEAN + MAX_GLOW;
/// …and the word of the lamps it is swept under (glow.rs; 0 = never, or not since the light model was last down) · which metre the sweep comes to next ·
/// how many it has still to go through at the quick pace
pub const O_GLOW_UP: usize = O_GLOW_A + MAX_GLOW;
pub const O_GLOW_PLACE: usize = O_GLOW_UP + MAX_GLOW;
pub const O_GLOW_LAMPS: usize = O_GLOW_PLACE + MAX_GLOW;
pub const O_GLOW_AT: usize = O_GLOW_LAMPS + 1;
pub const O_GLOW_LEFT: usize = O_GLOW_AT + 1;
/// THE CELLS, last: brick b's are the `CELLS` doubles from `O_RHO + b * CELLS`, so the row is in use up to the end of the highest brick that stands
pub const O_RHO: usize = O_GLOW_LEFT + 1;
pub const LEN: usize = O_RHO + MAX_BRICKS * CELLS;
/// seconds a step
pub const SUB: f64 = 0.1;
const VOL: f64 = CELL * CELL * CELL;
pub const SIDE: f64 = BX as f64 * CELL;

/// THE GRID'S PART OF A GAME'S BLOCK (state.rs), a pool: `live` is how many of its bytes are the game's state, itself included: 0 where the game has no
/// grid (the part is then nobody's), else the row up to the end of the highest brick that stands (`relive`). What lies past that is read by nobody: a
/// brick's cells are written zeros when it is made.
#[repr(C)]
pub struct Smoke {
    pub live: u32,
    pub spare: u32,
    pub st: [f64; LEN],
}
/// the bytes before the row
pub const ROW: usize = 8;
const _: () = assert!(size_of::<Smoke>() == ROW + LEN * 8);

/// THE TABLES, a set a slot of the blocks, made from the block that stands there and the box world its game steps over (the head of this file):
/// the brick at a place of the plan, plus one (0 = none)
static mut INDEX: [[u16; MAX_BRICKS]; GAMES] = [[0; MAX_BRICKS]; GAMES];
/// a cell's faces: 1 = open toward +x · 2 = open upward · 4 = open toward +z · 8 = asked (the three bits are an answer) · 16, 32 = on the plan's near
/// edge, open toward −x, toward −z, and 64 = those two asked (F_WEST, F_NORTH, F_EDGE: only a step with wind asks them)
static mut FACES: [[u8; MAX_BRICKS * CELLS]; GAMES] = [[0; MAX_BRICKS * CELLS]; GAMES];
/// the leaves as the last step found them, a row each as the box world holds it, and how many
static mut LEAF_AT: [[f64; MAX_LEAVES * REC]; GAMES] = [[0.0; MAX_LEAVES * REC]; GAMES];
static mut LEAF_N: [usize; GAMES] = [0; GAMES];
/// WHOSE A SLOT'S TABLES ARE: the stamp of the block INDEX was made from (0: none yet) · have the faces of that state been asked
static mut MADE: [(f64, bool); GAMES] = [(0.0, false); GAMES];
/// the statics as the faces were last asked of them: each one's word, and the rectangle of the plan round it
static mut SEEN: [Seen; GAMES] = [const { Seen::NONE }; GAMES];
/// a step's change of every cell, summed before any is applied; and which bricks the step left with next to nothing in them
static mut DELTA: [f64; MAX_BRICKS * CELLS] = [0.0; MAX_BRICKS * CELLS];
static mut THIN: [bool; MAX_BRICKS] = [false; MAX_BRICKS];
static mut HELD: [f64; MAX_BRICKS] = [0.0; MAX_BRICKS];
/// what of a step came into a brick from the bricks round it, and what is missing from its color
static mut CAME: [[f64; 4]; MAX_BRICKS] = [[0.0; 4]; MAX_BRICKS];

const F_X: u8 = 1;
const F_Y: u8 = 2;
const F_Z: u8 = 4;
const F_ASKED: u8 = 8;
/// …and, of a cell on the plan's near edges alone, the ways OUT of the plan toward −x and toward −z (the far edges' ways out are F_X and F_Z of their
/// last cells): what the wind carries smoke off the level by, asked the first time a wind blows over the cell (F_EDGE = asked)
const F_WEST: u8 = 16;
const F_NORTH: u8 = 32;
const F_EDGE: u8 = 64;

#[inline(always)]
fn made(g: usize) -> &'static mut (f64, bool) {
    unsafe { &mut (*core::ptr::addr_of_mut!(MADE))[g] }
}

struct Grid {
    /// the slot of the blocks this grid's state stands in (a slot that is not one traps: state.rs)
    g: usize,
    st: &'static mut [f64; LEN],
    index: &'static mut [u16; MAX_BRICKS],
    faces: &'static mut [u8; MAX_BRICKS * CELLS],
    nbx: usize,
    nbz: usize,
}
fn grid(g: usize) -> Grid {
    let st = &mut state(g).smoke.st;
    let (nbx, nbz) = dims(st);
    unsafe { Grid { g, st, index: &mut (*core::ptr::addr_of_mut!(INDEX))[g], faces: &mut (*core::ptr::addr_of_mut!(FACES))[g], nbx, nbz } }
}
/// the plan a state says it stands on (0 by 0 where the head holds anything a plan cannot be)
fn dims(st: &[f64; LEN]) -> (usize, usize) {
    let (x, z) = (st[H_NBX], st[H_NBZ]);
    if x >= 1.0 && z >= 1.0 && x * z <= MAX_BRICKS as f64 && floor(x) == x && floor(z) == z {
        (x as usize, z as usize)
    } else {
        (0, 0)
    }
}
/// a cell's faces in four bytes a word
pub const WORDS: usize = CELLS / 4;
/// THE GRID AS IT STANDS, for whoever draws it: the plan in bricks, the steps taken, and each brick
pub struct Stands {
    pub wide: usize,
    pub deep: usize,
    pub steps: f64,
    g: usize,
}
/// a brick: its place in plan, what is in each cell, and the faces each is open by (bits 1, 2, 4; 8 = asked)
pub struct Stood {
    pub x: usize,
    pub z: usize,
    pub rho: &'static [f64],
    pub open: [u32; WORDS],
    /// what is missing from its color
    pub dye: [f64; 3],
}
/// …none before its first step, or where nobody has asked the faces of the state that stands there (a game put back, till its next step or `ways`)
pub fn stands(g: usize, stamp: f64) -> Option<Stands> {
    let st = &state_read(g).smoke.st;
    let (wide, deep) = dims(st);
    let m = made(g);
    if wide == 0 || *m != (stamp, true) {
        return None;
    }
    Some(Stands { wide, deep, steps: st[H_STEPS], g })
}
impl Stands {
    pub fn brick(&self, b: usize) -> Option<Stood> {
        let st = &state_read(self.g).smoke.st;
        let (x, z) = (st[O_KEYS + b * 2], st[O_KEYS + b * 2 + 1]);
        if !(x >= 1.0 && z >= 1.0 && x <= self.wide as f64 && z <= self.deep as f64) {
            return None;
        }
        let faces = unsafe { &(*core::ptr::addr_of!(FACES))[self.g] };
        let face = |c: usize| (faces[b * CELLS + c] & (F_X | F_Y | F_Z | F_ASKED)) as u32;
        let open = core::array::from_fn(|w| (0..4).fold(0, |o, j| o | face(w * 4 + j) << (8 * j)));
        Some(Stood { x: x as usize - 1, z: z as usize - 1, rho: &st[O_RHO + b * CELLS..O_RHO + (b + 1) * CELLS], open, dye: core::array::from_fn(|i| st[O_DYE + b * 3 + i]) })
    }
}
/// THE LIGHT UNDER A ROOF at a point, of the game in slot `g` over box world `w` (glow.rs at)
pub fn under_roof(g: usize, w: &World, p: [f64; 3], tune: &Knobs) -> Option<crate::glow::Field> {
    let st = &state_read(g).smoke.st;
    let (x, z) = dims(st);
    crate::glow::at(g, w, st, x, z, p, tune)
}
/// THE PLAN A GAME'S GRID STANDS ON, in metres along x and z (0 by 0 before its first step)
pub fn plan(g: usize) -> (f64, f64) {
    let (x, z) = dims(&state(g).smoke.st);
    (x as f64 * SIDE, z as f64 * SIDE)
}
#[inline(always)]
fn cell_at(x: usize, y: usize, z: usize) -> usize {
    (y * BX + z) * BX + x
}

impl Grid {
    /// the brick at (bx, bz) of the plan, if there is one
    #[inline(always)]
    fn brick(&self, bx: i32, bz: i32) -> Option<usize> {
        if bx < 0 || bz < 0 || bx as usize >= self.nbx || bz as usize >= self.nbz {
            return None;
        }
        let i = self.index[bz as usize * self.nbx + bx as usize];
        if i == 0 {
            None
        } else {
            Some(i as usize - 1)
        }
    }
    #[inline(always)]
    fn key(&self, b: usize) -> Option<(i32, i32)> {
        let kx = self.st[O_KEYS + b * 2];
        if kx == 0.0 {
            None
        } else {
            Some((kx as i32 - 1, self.st[O_KEYS + b * 2 + 1] as i32 - 1))
        }
    }
    /// the brick at (bx, bz), made in the lowest free slot if there is none: None = off the plan (there is a slot for every place of it)
    fn brick_or_new(&mut self, bx: i32, bz: i32) -> Option<usize> {
        if bx < 0 || bz < 0 || bx as usize >= self.nbx || bz as usize >= self.nbz {
            return None;
        }
        if let Some(b) = self.brick(bx, bz) {
            return Some(b);
        }
        for b in 0..MAX_BRICKS {
            if self.st[O_KEYS + b * 2] == 0.0 {
                self.st[O_KEYS + b * 2] = (bx + 1) as f64;
                self.st[O_KEYS + b * 2 + 1] = (bz + 1) as f64;
                self.index[bz as usize * self.nbx + bx as usize] = b as u16 + 1;
                for c in 0..CELLS {
                    self.st[O_RHO + b * CELLS + c] = 0.0;
                    self.faces[b * CELLS + c] = 0;
                }
                self.dye(b, [0.0; 3]);
                return Some(b);
            }
        }
        None
    }
    fn free(&mut self, b: usize) {
        if let Some((bx, bz)) = self.key(b) {
            if bx >= 0 && bz >= 0 && (bx as usize) < self.nbx && (bz as usize) < self.nbz {
                self.index[bz as usize * self.nbx + bx as usize] = 0;
            }
        }
        self.st[O_KEYS + b * 2] = 0.0;
        self.st[O_KEYS + b * 2 + 1] = 0.0;
        for c in 0..CELLS {
            self.st[O_RHO + b * CELLS + c] = 0.0;
        }
        self.dye(b, [0.0; 3]);
    }
    fn dye(&mut self, b: usize, d: [f64; 3]) {
        self.st[O_DYE + b * 3..O_DYE + b * 3 + 3].copy_from_slice(&d);
    }
    fn dyed(&self, b: usize) -> [f64; 3] {
        [self.st[O_DYE + b * 3], self.st[O_DYE + b * 3 + 1], self.st[O_DYE + b * 3 + 2]]
    }
    /// `share` of a brick's smoke is of color `d`, and the rest what it was
    fn mix(&mut self, b: usize, share: f64, d: [f64; 3]) {
        let was = self.dyed(b);
        let s = unit(share);
        self.dye(b, core::array::from_fn(|i| was[i] + (d[i] - was[i]) * s));
    }
    /// where each brick is, from the state (a brick the state names off its own plan, or where another already is, is given back)
    fn reindex(&mut self) {
        for i in self.index.iter_mut() {
            *i = 0;
        }
        for b in 0..MAX_BRICKS {
            if let Some((bx, bz)) = self.key(b) {
                if bx >= 0 && bz >= 0 && (bx as usize) < self.nbx && (bz as usize) < self.nbz && self.index[bz as usize * self.nbx + bx as usize] == 0 {
                    self.index[bz as usize * self.nbx + bx as usize] = b as u16 + 1;
                } else {
                    self.st[O_KEYS + b * 2] = 0.0;
                    self.st[O_KEYS + b * 2 + 1] = 0.0;
                }
            }
        }
    }
    /// the faces of every cell whose centre lies within a cell of the rectangle are unknown again (a face is a line from a centre to the next one over)
    fn dirty(&mut self, x0: f64, z0: f64, x1: f64, z1: f64) {
        for b in 0..MAX_BRICKS {
            let Some((bx, bz)) = self.key(b) else { continue };
            let (ox, oz) = (bx as f64 * SIDE, bz as f64 * SIDE);
            if !(ox <= x1 + CELL && oz <= z1 + CELL && ox + SIDE >= x0 - CELL && oz + SIDE >= z0 - CELL) {
                continue;
            }
            for z in 0..BX {
                for x in 0..BX {
                    let (cx, cz) = (ox + (x as f64 + 0.5) * CELL, oz + (z as f64 + 0.5) * CELL);
                    if cx < x0 - CELL || cx > x1 + CELL || cz < z0 - CELL || cz > z1 + CELL {
                        continue;
                    }
                    for y in 0..BY {
                        self.faces[b * CELLS + cell_at(x, y, z)] = 0;
                    }
                }
            }
        }
    }
}

impl Grid {
    /// THE TABLES MADE AGAIN from the state as it stands: where each brick is, and no face is known (which brick a cell's faces are kept under went with
    /// the state they were made from)
    fn remake(&mut self) {
        self.reindex();
        self.forget();
    }
    /// no face is known, and no leaf is remembered
    fn forget(&mut self) {
        for f in self.faces.iter_mut() {
            *f = 0;
        }
        unsafe { (*core::ptr::addr_of_mut!(LEAF_N))[self.g] = 0 };
    }
    /// HOW MUCH OF THE PART IS THE GAME'S NOW: the row up to the end of the highest brick that stands
    fn relive(&mut self) {
        let mut top = 0;
        for b in 0..MAX_BRICKS {
            if self.st[O_KEYS + b * 2] != 0.0 {
                top = b + 1;
            }
        }
        state(self.g).smoke.live = (ROW + (O_RHO + top * CELLS) * 8) as u32;
    }
}
/// THE TABLES OF SLOT `g` ARE THE BLOCK'S THAT STANDS THERE: made from the block of this `stamp`, or made again now. Whoever reads a table goes
/// through here (`step`, `trans`): the grid of a slot is not handed out of this file without its stamp.
fn tables(g: usize, stamp: f64) -> Grid {
    let m = made(g);
    let mut gr = grid(g);
    if m.0 != stamp {
        *m = (stamp, false);
        gr.remake();
    }
    gr
}
/// …and its faces are of the statics as they stand: where one was written, they are asked again (where it stood, and where it stands)
fn over(g: usize, w: &World) {
    made(g).1 = true;
    let mut gr = grid(g);
    let round = |r: &[f64]| {
        let e = r[R_H] + r[R_H + 1] + r[R_H + 2];
        [r[R_C] - e, r[R_C + 2] - e, r[R_C] + e, r[R_C + 2] + e]
    };
    let seen = unsafe { &mut (*core::ptr::addr_of_mut!(SEEN))[g] };
    seen.look(w, round, |r| gr.dirty(r[0], r[1], r[2], r[3]));
}
/// THE GAME IN SLOT `g` HAS A GRID (`home`: an empty one, on no plan yet, nothing swept) OR HAS NONE (its part is nobody's, zeros). The whole row is
/// written, not what was in use of it: a slot keeps what the block before left past its own.
pub fn reset(g: usize, home: bool) {
    let p = &mut state(g).smoke;
    for v in p.st.iter_mut() {
        *v = 0.0;
    }
    p.live = if home { (ROW + O_RHO * 8) as u32 } else { 0 };
    p.spare = 0;
    *made(g) = (0.0, false);
}
/// NO SMOKE ANYWHERE: the cells, the emitters, the book and the count of steps are zeros. The plan the grid stands on and the light that comes round
/// stay: they are the level's, not the smoke's.
pub fn empty(g: usize) {
    let mut gr = grid(g);
    for h in 0..HEAD {
        if h != H_NBX && h != H_NBZ && h != H_CLOCK {
            gr.st[h] = 0.0;
        }
    }
    for v in gr.st[O_EMIT..O_GLOW_MAX].iter_mut() {
        *v = 0.0;
    }
    for v in gr.st[O_RHO..].iter_mut() {
        *v = 0.0;
    }
    gr.relive();
    *made(g) = (0.0, false);
}

/// is the way between two cells' centres open: nothing solid and no leaf between them, asked both ways (a line that STARTS inside a box is not stopped by
/// it — collision.ts segmentBlocked's own rule for an eye in a wall — so a cell inside a wall is shut off by the line that ends in it)
fn open(w: &World, a: [f64; 3], b: [f64; 3], tune: &Knobs) -> bool {
    clear(w, a, b, CELL, tune) && clear(w, b, a, CELL, tune)
}
/// nothing solid and no leaf on the line a → b of this length (the one place this file walks a line: the walk is not copied into every asker)
#[inline(never)]
fn clear(w: &World, a: [f64; 3], b: [f64; 3], len: f64, tune: &Knobs) -> bool {
    clear_but(w, a, b, len, 0, tune)
}
/// …the same line for what passes boxes of the flags `through` (glow.rs: light goes through a pane, smoke does not)
#[inline(never)]
pub(crate) fn clear_but(w: &World, a: [f64; 3], b: [f64; 3], len: f64, through: u32, tune: &Knobs) -> bool {
    lines::blocked(w, a[0], a[1], a[2], b[0], b[1], b[2], len, 0.02, through, true, 0, tune) != -1
}
/// can what is let out at `p` reach the point `c` (a line of any length; one too short to be a line is open)
fn reaches(w: &World, p: [f64; 3], c: [f64; 3], tune: &Knobs) -> bool {
    let len = hypot3(c[0] - p[0], c[1] - p[1], c[2] - p[2]);
    len < 1e-4 || clear(w, p, c, len, tune)
}

/// what a step is told: the model's numbers
pub struct K {
    pub decay: f64,
    pub spread: f64,
    pub rise: f64,
    pub rise_rate: f64,
    pub gone: f64,
    /// THE WIND the smoke drifts by under the open sky, m/s along x and z (0 and 0 = still air, and no statement of the drift
    /// is reached: the step as it was). What share of it a square metre of the plan has is in `WIND`, written by the host before the step.
    pub wind_x: f64,
    pub wind_z: f64,
    /// …and how it rises with height, as the storm's field does: `wind_low` of it at the ground, `wind_gain` more by
    /// `wind_full_at` metres up
    pub wind_low: f64,
    pub wind_gain: f64,
    pub wind_full_at: f64,
    /// …and how much of its own spread the smoke keeps where the wind blows whole (0 ‥ 1; step 4)
    pub wind_keep: f64,
}

/// THE WIND'S SHARE a square metre of the plan (0 under a roof … 1 in the open: game/wind.ts stormShare, the footprints the picture's wind goes by),
/// row by row, `2 · nbx` to a row: written by the host before a step that has wind and read by that step alone. Nothing of a game is kept here.
static mut WIND: [f64; MAX_GLOW] = [0.0; MAX_GLOW];
pub fn wind_addr() -> *mut f64 {
    core::ptr::addr_of_mut!(WIND) as *mut f64
}
/// the most of a cell's smoke the wind carries across one face in a pass (a cell hands on at most half of what it holds, along x and along z), and
/// the most passes a step takes: together the fastest wind the smoke keeps up with, 10 m/s along an axis
const DRIFT_MAX: f64 = 0.25;
const MAX_PASSES: f64 = 8.0;
/// a share as the host wrote it, held to 0 ‥ 1 (what is no number is no wind)
#[inline(always)]
fn unit(s: f64) -> f64 {
    if s > 0.0 {
        if s < 1.0 {
            s
        } else {
            1.0
        }
    } else {
        0.0
    }
}
/// THE WIND'S SHARE at the cell (x, z) of the brick whose first cell is (gx, gz) of the plan, and at the squares beyond its +x and +z faces (this
/// brick's, or the next brick's first where one stands: `east`, `south` — inside the plan wherever that brick can stand; else the cell's own)
#[inline(always)]
fn shares(share: &[f64; MAX_GLOW], wide: usize, gx: usize, gz: usize, x: usize, z: usize, east: bool, south: bool) -> (f64, f64, f64) {
    let s = unit(share[(gz + z) / 2 * wide + (gx + x) / 2]);
    let sx = if x + 1 < BX || east { unit(share[(gz + z) / 2 * wide + (gx + x + 1) / 2]) } else { s };
    let sz = if z + 1 < BX || south { unit(share[(gz + z + 1) / 2 * wide + (gx + x) / 2]) } else { s };
    (s, sx, sz)
}
/// LET OUT: an emitter joins the list of the game in slot `g` (where, how wide, what it lets out a second, for how long). When the list is full the one
/// with the least still to let out gives way, and that may be the newcomer.
pub fn emit(g: usize, x: f64, y: f64, z: f64, radius: f64, rate: f64, ttl: f64, dye: [f64; 3]) {
    // (an emitter that is no number lets nothing out, and must not sit in the list as one)
    if !(x - x == 0.0 && y - y == 0.0 && z - z == 0.0 && radius >= 0.0 && radius < 1e6 && rate > 0.0 && ttl > 0.0 && rate * ttl < 1e12) {
        return;
    }
    let p = &mut state(g).smoke;
    if p.live == 0 {
        return;
    }
    let st = &mut p.st;
    let n = emitters(st);
    let mut at = n;
    if n >= MAX_EMIT {
        let left = |st: &[f64; LEN], i: usize| st[O_EMIT + i * EMIT + 3] * js_max(0.0, st[O_EMIT + i * EMIT + 4] - st[O_EMIT + i * EMIT + 5]);
        let mut k = 0;
        for i in 1..n {
            if left(st, i) < left(st, k) {
                k = i;
            }
        }
        if left(st, k) >= rate * ttl {
            return;
        }
        at = k;
    } else {
        st[H_EMIT] = (n + 1) as f64;
    }
    let o = O_EMIT + at * EMIT;
    st[o] = x;
    st[o + 1] = y;
    st[o + 2] = z;
    st[o + 3] = rate;
    st[o + 4] = ttl;
    st[o + 5] = 0.0;
    st[o + 6] = radius;
    for i in 0..3 {
        st[o + 7 + i] = unit(dye[i]);
    }
}
/// SHUT: every emitter of the game in slot `g` has let out its last; what is in the air stays
pub fn shut(g: usize) {
    let st = &mut state(g).smoke.st;
    let n = emitters(st);
    for i in 0..n {
        let o = O_EMIT + i * EMIT;
        st[o + 4] = st[o + 5];
    }
}
fn emitters(st: &[f64; LEN]) -> usize {
    js_min(st[H_EMIT], MAX_EMIT as f64) as usize
}
/// an emitter as it lets out: where it stands, what it lets out a second, how long it runs, how long it has, how wide it is, what is missing from its color
pub struct Lets {
    pub at: [f64; 3],
    pub rate: f64,
    pub ttl: f64,
    pub age: f64,
    pub radius: f64,
    pub dye: [f64; 3],
}
/// the emitters letting out in the game in slot `g`
pub fn letting(g: usize) -> impl Iterator<Item = Lets> {
    let st = &state_read(g).smoke.st;
    (0..emitters(st)).map(|i| {
        let e = &st[O_EMIT + i * EMIT..O_EMIT + (i + 1) * EMIT];
        Lets { at: [e[0], e[1], e[2]], rate: e[3], ttl: e[4], age: e[5], radius: e[6], dye: [e[7], e[8], e[9]] }
    }).filter(|e| e.age < e.ttl)
}
/// how many emitters are letting out in the game in slot `g`
pub fn emitting(g: usize) -> usize {
    emitters(&state_read(g).smoke.st)
}
/// is there smoke in the game in slot `g`: an emitter, or a brick standing
pub fn smoky(g: usize) -> bool {
    emitting(g) > 0 || state_read(g).smoke.live as usize > ROW + O_RHO * 8
}
/// `dt` SECONDS OF THE GAME in slot `g` → how many steps have come due: four at most (a game that stood still does not owe a hundred)
pub fn beat(g: usize, dt: f64) -> u32 {
    let p = &mut state(g).smoke;
    if p.live == 0 || !(dt > 0.0) {
        return 0;
    }
    let mut left = p.st[H_CLOCK] + dt;
    let mut n = 0;
    while left >= SUB && n < 4 {
        left -= SUB;
        n += 1;
    }
    p.st[H_CLOCK] = if left >= SUB { 0.0 } else { left };
    n
}

/// the plan of box world `w`, in squares
fn plan_of(w: &World) -> (f64, f64) {
    (-floor(-(w.w as f64 * w.cs) / SIDE), -floor(-(w.h as f64 * w.cs) / SIDE))
}
/// the light's tables of box world `w` made before a step asks for them (glow.rs ready)
pub fn light_ready(g: usize, w: &World, to_sun: [f64; 3], tune: &Knobs) {
    let (x, z) = plan_of(w);
    if x >= 1.0 && z >= 1.0 && x * z <= MAX_BRICKS as f64 {
        crate::glow::ready(g, w, x as usize, z as usize, to_sun, tune);
    }
}
/// `n` STEPS of a tenth of a second each of the grid of the game in slot `g` (its block's `stamp`), over box world `w` as it stands.
/// 0, or ASK_HOST = no grid stands on this level (its plan is larger than a grid can be), or the game has none: the
/// state is as it was.
pub fn step(g: usize, stamp: f64, w: &World, n: u32, k: &K, glow: Option<&crate::glow::K>, tune: &Knobs) -> u32 {
    let (nbx, nbz) = plan_of(w);
    if !(nbx >= 1.0 && nbz >= 1.0 && nbx * nbz <= MAX_BRICKS as f64) || state(g).smoke.live == 0 {
        return ASK_HOST;
    }
    tables(g, stamp);
    over(g, w);
    let mut g = grid(g);
    if g.st[H_NBX] != nbx || g.st[H_NBZ] != nbz {
        // (the first step on this level: a state from any other plan holds nothing that is anywhere here)
        g.st[H_NBX] = nbx;
        g.st[H_NBZ] = nbz;
        for v in g.st[O_GLOW_MAX..O_RHO].iter_mut() {
            *v = 0.0;
        }
        g.nbx = nbx as usize;
        g.nbz = nbz as usize;
        g.remake();
    }
    for _ in 0..n {
        substep(w, &mut g, k, tune);
        match glow {
            Some(gk) => crate::glow::step(g.g, w, g.st, g.nbx, g.nbz, gk, tune),
            None => crate::glow::down(g.st),
        }
        g.st[H_STEPS] += 1.0;
    }
    g.relive();
    0
}

/// THE WAYS OF THE STATE THAT STANDS are asked, over box world `w` as it stands: what the next step would
/// ask first, for whoever draws a game put back before it steps. No state is written. Nothing, where the state's plan is not the level's (no step yet).
pub fn ways(g: usize, stamp: f64, w: &World, tune: &Knobs) {
    let (on, plan) = (dims(&state_read(g).smoke.st), plan_of(w));
    if on.0 == 0 || on != (plan.0 as usize, plan.1 as usize) {
        return;
    }
    tables(g, stamp);
    over(g, w);
    let mut g = grid(g);
    leaves(w, &mut g);
    ask(w, &mut g, tune);
}
/// the faces round every leaf that is not as it was last found (a door swings; one is new, one is gone) are to be asked again: where it stood and where
/// it stands. A leaf that has not moved keeps the answers it had.
fn leaves(w: &World, g: &mut Grid) {
    let s = g.g;
    let (kept, kept_n) = unsafe { (&mut (*core::ptr::addr_of_mut!(LEAF_AT))[s], &mut (*core::ptr::addr_of_mut!(LEAF_N))[s]) };
    let n = js_min(w.leaves() as f64, MAX_LEAVES as f64) as usize;
    // (its reach in plan whichever way it is turned: the two half sides, and the third where it is tilted off the upright)
    let reach = |r: &[f64]| r[R_H] + r[R_H + 2] + if r[R_KIND] as u32 & K_ROT != 0 { r[R_H + 1] } else { 0.0 };
    for i in 0..if n > *kept_n { n } else { *kept_n } {
        let (had, has) = (i < *kept_n, i < n);
        let was = &mut kept[i * REC..(i + 1) * REC];
        let now = if has { &w.leaf[i * REC..(i + 1) * REC] } else { &[][..] };
        if had && has && (0..REC).all(|j| was[j].to_bits() == now[j].to_bits()) {
            continue;
        }
        if had {
            let e = reach(was);
            g.dirty(was[R_C] - e, was[R_C + 2] - e, was[R_C] + e, was[R_C + 2] + e);
        }
        if has {
            let e = reach(now);
            g.dirty(now[R_C] - e, now[R_C + 2] - e, now[R_C] + e, now[R_C + 2] + e);
            for j in 0..REC {
                was[j] = now[j];
            }
        }
    }
    *kept_n = n;
}
/// the faces nobody has asked (a new brick's, those round a leaf, those round a static that was written)
fn ask(w: &World, g: &mut Grid, tune: &Knobs) {
    for b in 0..MAX_BRICKS {
        let Some((bx, bz)) = g.key(b) else { continue };
        let (ox, oz) = (bx as f64 * SIDE, bz as f64 * SIDE);
        for y in 0..BY {
            for z in 0..BX {
                for x in 0..BX {
                    let c = b * CELLS + cell_at(x, y, z);
                    if g.faces[c] & F_ASKED != 0 {
                        continue;
                    }
                    let p = [ox + (x as f64 + 0.5) * CELL, (y as f64 + 0.5) * CELL, oz + (z as f64 + 0.5) * CELL];
                    let mut f = F_ASKED;
                    if open(w, p, [p[0] + CELL, p[1], p[2]], tune) {
                        f |= F_X;
                    }
                    if y + 1 < BY && open(w, p, [p[0], p[1] + CELL, p[2]], tune) {
                        f |= F_Y;
                    }
                    if open(w, p, [p[0], p[1], p[2] + CELL], tune) {
                        f |= F_Z;
                    }
                    g.faces[c] = f;
                }
            }
        }
    }
}

/// how much of its rise a plume in the whole of the wind loses (the drawn one, in a storm's yard, tops out at about 1.1 m)
const LAID: f64 = 0.4;
fn substep(w: &World, g: &mut Grid, k: &K, tune: &Knobs) {
    // 0. the leaves as they stand now
    leaves(w, g);

    // 1. the emitters let out, each up the column it stands under (from its own foot to the top of its plume: its width, and the rise the blobs had),
    //    and the spent ones leave the list
    let windy = k.wind_x != 0.0 || k.wind_z != 0.0;
    let share = unsafe { &*core::ptr::addr_of!(WIND) };
    let wide = g.nbx * 2;
    let n = emitters(g.st);
    let mut kept = 0;
    // (is there any color to carry)
    let mut dyes = (0..MAX_BRICKS).any(|b| g.st[O_KEYS + b * 2] != 0.0 && g.dyed(b) != [0.0; 3]);
    for i in 0..n {
        let o = O_EMIT + i * EMIT;
        let (x, y0, z, rate, ttl, age, radius) = (g.st[o], g.st[o + 1], g.st[o + 2], g.st[o + 3], g.st[o + 4], g.st[o + 5], g.st[o + 6]);
        let dye = [g.st[o + 7], g.st[o + 8], g.st[o + 9]];
        dyes |= dye != [0.0; 3];
        if age < ttl {
            let amount = rate * js_min(SUB, ttl - age);
            let top = (BY - 1) as f64;
            // (its foot and the top of its plume, in cells: each cell between takes what of the plume is in it)
            let foot = js_min(js_max((y0 - radius) / CELL, 0.0), top);
            let (fx, fz) = (floor(x / CELL), floor(z / CELL));
            // (the wind lays a plume over: in all of it, it rises LAID less)
            let (mx, mz) = (floor(x), floor(z));
            let blown = if windy && mx >= 0.0 && mz >= 0.0 && mx < wide as f64 && mz < (g.nbz * 2) as f64 { unit(share[mz as usize * wide + mx as usize]) } else { 0.0 };
            let rise = js_min(k.rise, k.rise_rate * age) * (1.0 - LAID * blown);
            let head = js_min(js_max((y0 + radius + rise) / CELL, foot + 0.1), top + 1.0);
            // (a point off the plan lets out nowhere)
            if fx >= 0.0 && fz >= 0.0 && fx < (g.nbx * BX) as f64 && fz < (g.nbz * BX) as f64 {
                // the column: the one it stands in if what it lets out can reach that cell's middle, else the first of those round it that it can reach (a
                // canister against a wall stands in a cell whose middle is inside the wall)
                const ROUND: [(f64, f64); 9] = [(0.0, 0.0), (-1.0, 0.0), (1.0, 0.0), (0.0, -1.0), (0.0, 1.0), (-1.0, -1.0), (1.0, -1.0), (-1.0, 1.0), (1.0, 1.0)];
                let p = [x, js_max(y0, 0.05), z];
                let (mut cx, mut cz) = (fx, fz);
                for (dx, dz) in ROUND {
                    let (tx, tz) = (fx + dx, fz + dz);
                    if tx < 0.0 || tz < 0.0 || tx >= (g.nbx * BX) as f64 || tz >= (g.nbz * BX) as f64 {
                        continue;
                    }
                    if reaches(w, p, [(tx + 0.5) * CELL, (js_min(js_max(floor(p[1] / CELL), 0.0), top) + 0.5) * CELL, (tz + 0.5) * CELL], tune) {
                        cx = tx;
                        cz = tz;
                        break;
                    }
                }
                let (ux, uz) = (cx as usize, cz as usize);
                g.st[H_OUT] += amount;
                if let Some(b) = g.brick_or_new((ux / BX) as i32, (uz / BX) as i32) {
                    if dyes {
                        let had: f64 = g.st[O_RHO + b * CELLS..O_RHO + (b + 1) * CELLS].iter().sum();
                        g.mix(b, amount / VOL / (had + amount / VOL), dye);
                    }
                    let each = amount / VOL / (head - foot);
                    let mut y = floor(foot);
                    while y < head {
                        g.st[O_RHO + b * CELLS + cell_at(ux % BX, y as usize, uz % BX)] += each * (js_min(head, y + 1.0) - js_max(foot, y));
                        y += 1.0;
                    }
                }
            }
        }
        let age = age + SUB;
        if age < ttl {
            let t = O_EMIT + kept * EMIT;
            g.st[t] = x;
            g.st[t + 1] = y0;
            g.st[t + 2] = z;
            g.st[t + 3] = rate;
            g.st[t + 4] = ttl;
            g.st[t + 5] = age;
            g.st[t + 6] = radius;
            g.st[t + 7..t + 10].copy_from_slice(&dye);
            kept += 1;
        }
    }
    for i in kept * EMIT..n * EMIT {
        g.st[O_EMIT + i] = 0.0;
    }
    g.st[H_EMIT] = kept as f64;

    // 2. a brick gets the neighbour it would spill into: where a cell at its edge holds smoke AND the way out of that cell
    //    into the neighbour is open. A wall along a brick's edge costs no brick behind it.
    let rate = k.spread * 0.5 * SUB / (CELL * CELL);
    let spill = k.gone / (CELLS as f64 * VOL);
    const OUT: [(i32, i32); 4] = [(-1, 0), (1, 0), (0, -1), (0, 1)];
    for b in 0..MAX_BRICKS {
        let Some((bx, bz)) = g.key(b) else { continue };
        let (ox, oz) = (bx as f64 * SIDE, bz as f64 * SIDE);
        for (side, &(dx, dz)) in OUT.iter().enumerate() {
            let (nx, nz) = (bx + dx, bz + dz);
            if nx < 0 || nz < 0 || nx as usize >= g.nbx || nz as usize >= g.nbz || g.brick(nx, nz).is_some() {
                continue;
            }
            let mut wanted = false;
            'edge: for y in 0..BY {
                for i in 0..BX {
                    let (x, z) = edge_cell(side, i);
                    if !(g.st[O_RHO + b * CELLS + cell_at(x, y, z)] > spill) {
                        continue;
                    }
                    let p = [ox + (x as f64 + 0.5) * CELL, (y as f64 + 0.5) * CELL, oz + (z as f64 + 0.5) * CELL];
                    if open(w, p, [p[0] + dx as f64 * CELL, p[1], p[2] + dz as f64 * CELL], tune) {
                        wanted = true;
                        break 'edge;
                    }
                }
            }
            if wanted {
                g.brick_or_new(nx, nz);
            }
        }
    }

    // 3. the faces nobody has asked
    ask(w, g, tune);

    let delta = unsafe { &mut *core::ptr::addr_of_mut!(DELTA) };
    let came = unsafe { &mut *core::ptr::addr_of_mut!(CAME) };
    if dyes {
        *came = [[0.0; 4]; MAX_BRICKS];
    }
    // 3b. THE WIND (K.wind): under the open sky the air moves and the smoke goes with it — across the open faces in plan, out of the cell UPWIND (a
    //     donor's own density: nothing is made), at the wind's speed at that height × the share of it the two squares have (none under a roof: a
    //     room's smoke stays a room's, and the wind comes in at a door only as far as the lee lets it). What it carries over the plan's edge has left
    //     the level: into the book as given back. A PASS carries at most DRIFT_MAX of a cell across a face, so a faster wind takes more passes of the
    //     step, each from the densities the one before left; a wind faster than MAX_PASSES carry is slowed WHOLE, both axes by one number, never
    //     bent. A brick the smoke is blown toward is made by 2, at the next step. No statement of this is reached in still air.
    let mut blown = 0.0;
    if windy {
        let fast = js_max(js_max(k.wind_x, -k.wind_x), js_max(k.wind_z, -k.wind_z)) * SUB / CELL * (k.wind_low + js_max(k.wind_gain, 0.0));
        let passes = js_max(1.0, js_min(MAX_PASSES, -floor(-fast / DRIFT_MAX)));
        let slowed = js_min(1.0, passes * DRIFT_MAX / fast) / passes;
        let (cx, cz) = (k.wind_x * SUB / CELL * slowed, k.wind_z * SUB / CELL * slowed);
        let mut pass = 0.0;
        while pass < passes {
            pass += 1.0;
            for b in 0..MAX_BRICKS {
                if g.st[O_KEYS + b * 2] != 0.0 {
                    for c in 0..CELLS {
                        delta[b * CELLS + c] = 0.0;
                    }
                }
            }
            for b in 0..MAX_BRICKS {
                let Some((bx, bz)) = g.key(b) else { continue };
                let (east, south) = (g.brick(bx + 1, bz), g.brick(bx, bz + 1));
                let (gx, gz) = (bx as usize * BX, bz as usize * BX);
                let (ox, oz) = (bx as f64 * SIDE, bz as f64 * SIDE);
                for z in 0..BX {
                    for x in 0..BX {
                        let (s, sx, sz) = shares(share, wide, gx, gz, x, z, east.is_some(), south.is_some());
                        if s == 0.0 && sx == 0.0 && sz == 0.0 {
                            continue;
                        }
                        for y in 0..BY {
                            // (the wind at this layer's middle, as the storm's field has it: slower at the ground)
                            let h = k.wind_low + k.wind_gain * js_max(0.0, js_min(1.0, (y as f64 + 0.5) * CELL / k.wind_full_at));
                            let (ux, uz) = (cx * h * 0.5 * (s + sx), cz * h * 0.5 * (s + sz));
                            let c = b * CELLS + cell_at(x, y, z);
                            // (a cell on the plan's near edge is asked its ways out the first time a wind blows over it)
                            if ((x == 0 && bx == 0) || (z == 0 && bz == 0)) && g.faces[c] & F_EDGE == 0 {
                                let p = [ox + (x as f64 + 0.5) * CELL, (y as f64 + 0.5) * CELL, oz + (z as f64 + 0.5) * CELL];
                                let mut f = g.faces[c] | F_EDGE;
                                if x == 0 && bx == 0 && open(w, p, [p[0] - CELL, p[1], p[2]], tune) {
                                    f |= F_WEST;
                                }
                                if z == 0 && bz == 0 && open(w, p, [p[0], p[1], p[2] - CELL], tune) {
                                    f |= F_NORTH;
                                }
                                g.faces[c] = f;
                            }
                            let (f, here) = (g.faces[c], g.st[O_RHO + c]);
                            if f & F_X != 0 {
                                let n = if x + 1 < BX { Some(c + 1) } else { east.map(|e| e * CELLS + cell_at(0, y, z)) };
                                if let Some(n) = n {
                                    let over = if ux > 0.0 { ux * here } else { ux * g.st[O_RHO + n] };
                                    delta[c] -= over;
                                    delta[n] += over;
                                    if dyes {
                                        carry(g, came, c, n, over);
                                    }
                                } else if cx > 0.0 && bx as usize + 1 == g.nbx {
                                    let out = cx * h * s * here;
                                    delta[c] -= out;
                                    blown += out;
                                }
                            }
                            if f & F_Z != 0 {
                                let n = if z + 1 < BX { Some(c + BX) } else { south.map(|e| e * CELLS + cell_at(x, y, 0)) };
                                if let Some(n) = n {
                                    let over = if uz > 0.0 { uz * here } else { uz * g.st[O_RHO + n] };
                                    delta[c] -= over;
                                    delta[n] += over;
                                    if dyes {
                                        carry(g, came, c, n, over);
                                    }
                                } else if cz > 0.0 && bz as usize + 1 == g.nbz {
                                    let out = cz * h * s * here;
                                    delta[c] -= out;
                                    blown += out;
                                }
                            }
                            if f & F_WEST != 0 && cx < 0.0 {
                                let out = -cx * h * s * here;
                                delta[c] -= out;
                                blown += out;
                            }
                            if f & F_NORTH != 0 && cz < 0.0 {
                                let out = -cz * h * s * here;
                                delta[c] -= out;
                                blown += out;
                            }
                        }
                    }
                }
            }
            for b in 0..MAX_BRICKS {
                if g.st[O_KEYS + b * 2] != 0.0 {
                    for c in 0..CELLS {
                        g.st[O_RHO + b * CELLS + c] = js_max(0.0, g.st[O_RHO + b * CELLS + c] + delta[b * CELLS + c]);
                    }
                }
            }
        }
    }
    // 4. smoke passes across every open face toward the thinner side: each cell's change summed from the densities as the step found them, then applied.
    //    WHERE THE WIND BLOWS THE SMOKE GOES WITH THE AIR AND DOES LITTLE OF ITS OWN (K.wind; the picture's solver pulls the smoke's whole velocity
    //    toward the wind's, and the wind is flat, so its own motions die and it stays as low as it was let out — smoke/wind_force.wgsl): a face
    //    passes less by the share of the wind its squares have. UPWARD, nothing in the open; SIDEWAYS, down to `wind_keep` of it (enough to get
    //    round what the wind pushes it against, as the picture's does by its pressure). In still air and under a roof: all of it, as ever.
    let calm = 1.0 - unit(k.wind_keep);
    for b in 0..MAX_BRICKS {
        if g.st[O_KEYS + b * 2] != 0.0 {
            for c in 0..CELLS {
                delta[b * CELLS + c] = 0.0;
            }
        }
    }
    for b in 0..MAX_BRICKS {
        let Some((bx, bz)) = g.key(b) else { continue };
        let (east, south) = (g.brick(bx + 1, bz), g.brick(bx, bz + 1));
        let (gx, gz) = (bx as usize * BX, bz as usize * BX);
        for y in 0..BY {
            for z in 0..BX {
                for x in 0..BX {
                    let c = b * CELLS + cell_at(x, y, z);
                    let (f, here) = (g.faces[c], g.st[O_RHO + c]);
                    let (rx, ry, rz) = if windy {
                        let (s, sx, sz) = shares(share, wide, gx, gz, x, z, east.is_some(), south.is_some());
                        (rate * (1.0 - calm * 0.5 * (s + sx)), rate * (1.0 - s), rate * (1.0 - calm * 0.5 * (s + sz)))
                    } else {
                        (rate, rate, rate)
                    };
                    if f & F_X != 0 {
                        let n = if x + 1 < BX { Some(c + 1) } else { east.map(|e| e * CELLS + cell_at(0, y, z)) };
                        if let Some(n) = n {
                            let pass = rx * (here - g.st[O_RHO + n]);
                            delta[c] -= pass;
                            delta[n] += pass;
                            if dyes {
                                carry(g, came, c, n, pass);
                            }
                        }
                    }
                    if f & F_Y != 0 {
                        let n = c + BX * BX;
                        let pass = ry * (here - g.st[O_RHO + n]);
                        delta[c] -= pass;
                        delta[n] += pass;
                    }
                    if f & F_Z != 0 {
                        let n = if z + 1 < BX { Some(c + BX) } else { south.map(|s| s * CELLS + cell_at(x, y, 0)) };
                        if let Some(n) = n {
                            let pass = rz * (here - g.st[O_RHO + n]);
                            delta[c] -= pass;
                            delta[n] += pass;
                            if dyes {
                                carry(g, came, c, n, pass);
                            }
                        }
                    }
                }
            }
        }
    }
    // 5. … applied, thinned by the step's decay (and the book with it). A brick with next to nothing in it is given back once nothing can feed it: no
    //    emitter stands in it or beside it, and no brick round it holds smoke at the edge they share: the very test that makes a brick (2), so one made
    //    this step is not given back in it. What it held goes into the book. Every brick is judged by what the step left, before any is given back.
    let keep = exp(-k.decay * SUB);
    for h in [H_OUT, H_GIVEN] {
        g.st[h] *= keep;
    }
    let thin = unsafe { &mut *core::ptr::addr_of_mut!(THIN) };
    let held = unsafe { &mut *core::ptr::addr_of_mut!(HELD) };
    for b in 0..MAX_BRICKS {
        thin[b] = true;
        if g.st[O_KEYS + b * 2] == 0.0 {
            continue;
        }
        let mut mass = 0.0;
        for c in 0..CELLS {
            let v = js_max(0.0, (g.st[O_RHO + b * CELLS + c] + delta[b * CELLS + c]) * keep);
            g.st[O_RHO + b * CELLS + c] = v;
            mass += v;
        }
        held[b] = mass * VOL;
        thin[b] = held[b] < k.gone;
        // (what came in is mixed into what is there now; the thinning leaves the color as it is)
        if dyes && came[b][0] > 0.0 && mass > 0.0 {
            let [i, d @ ..] = came[b];
            g.mix(b, i * keep / mass, d.map(|v| v / i));
        }
    }
    let mut given = 0.0;
    for b in 0..MAX_BRICKS {
        let Some((bx, bz)) = g.key(b) else { continue };
        if !thin[b] {
            continue;
        }
        // (the side of the neighbour that faces this brick is the opposite one: OUT pairs them 0 ↔ 1, 2 ↔ 3)
        let mut fed = OUT.iter().enumerate().any(|(side, &(dx, dz))| {
            g.brick(bx + dx, bz + dz).map_or(false, |n| {
                (0..BY).any(|y| {
                    (0..BX).any(|i| {
                        let (x, z) = edge_cell(side ^ 1, i);
                        g.st[O_RHO + n * CELLS + cell_at(x, y, z)] > spill
                    })
                })
            })
        });
        for i in 0..emitters(g.st) {
            let o = O_EMIT + i * EMIT;
            let (ex, ez) = (floor(g.st[o] / SIDE) - bx as f64, floor(g.st[o + 2] / SIDE) - bz as f64);
            fed |= ex >= -1.0 && ex <= 1.0 && ez >= -1.0 && ez <= 1.0;
        }
        // (`thin` from here on: this brick goes)
        thin[b] = !fed;
        if !fed {
            given += held[b];
        }
    }
    // (given back together, once every brick has been judged by the same densities)
    for b in 0..MAX_BRICKS {
        if thin[b] && g.st[O_KEYS + b * 2] != 0.0 {
            g.free(b);
        }
    }
    g.st[H_GIVEN] += given + blown * VOL * keep;
}

/// `over` of smoke goes from cell `c` to cell `n` (or back, where it is less than nothing): into another brick it takes its brick's color along
#[inline(always)]
fn carry(g: &Grid, came: &mut [[f64; 4]; MAX_BRICKS], c: usize, n: usize, over: f64) {
    let (from, to, v) = if over > 0.0 { (c / CELLS, n / CELLS, over) } else { (n / CELLS, c / CELLS, -over) };
    if from == to || !(v > 0.0) {
        return;
    }
    let d = g.dyed(from);
    came[to][0] += v;
    for i in 0..3 {
        came[to][1 + i] += v * d[i];
    }
}

/// the cell `i` along a brick's edge on `side` (0 = toward −x · 1 = toward +x · 2 = toward −z · 3 = toward +z), as (x, z) in the brick
#[inline(always)]
fn edge_cell(side: usize, i: usize) -> (usize, usize) {
    match side {
        0 => (0, i),
        1 => (BX - 1, i),
        2 => (i, 0),
        _ => (i, BX - 1),
    }
}

/// HOW MUCH OF THE LIGHT ALONG a → b COMES THROUGH the smoke of the game in slot `g`, whose block's stamp is `stamp` (1 = clear air): the density of the
/// cell each quarter metre of the line lies in, summed, times `sigma` per metre per unit. Read from the state alone and the index made from it: no box
/// is asked, so it answers whether or not a tick is open.
pub fn trans(g: usize, stamp: f64, a: [f64; 3], b: [f64; 3], sigma: f64) -> f64 {
    let g = tables(g, stamp);
    if g.nbx == 0 {
        return 1.0;
    }
    let (dx, dy, dz) = (b[0] - a[0], b[1] - a[1], b[2] - a[2]);
    let len = hypot3(dx, dy, dz);
    if !(len >= 1e-6) || !(len < 1e6) {
        return 1.0;
    }
    let n = -floor(-len / (CELL * 0.5));
    let ds = len / n;
    let mut sum = 0.0;
    let mut i = 0.0;
    while i < n {
        let t = (i + 0.5) / n;
        sum += g.rho_at(a[0] + dx * t, a[1] + dy * t, a[2] + dz * t);
        i += 1.0;
    }
    let tau = sigma * sum * ds;
    if tau <= 0.0 {
        1.0
    } else {
        exp(-tau)
    }
}

impl Grid {
    /// the density of the cell a point lies in (0 where there is no brick, and above and below the grid)
    #[inline]
    fn rho_at(&self, px: f64, py: f64, pz: f64) -> f64 {
        let (fx, fy, fz) = (floor(px / CELL), floor(py / CELL), floor(pz / CELL));
        // (a point that is no number, or off the plan, meets nothing)
        if !(fx >= 0.0 && fz >= 0.0 && fy >= 0.0 && fy < BY as f64 && fx < (self.nbx * BX) as f64 && fz < (self.nbz * BX) as f64) {
            return 0.0;
        }
        let (x, y, z) = (fx as usize, fy as usize, fz as usize);
        match self.brick((x / BX) as i32, (z / BX) as i32) {
            None => 0.0,
            Some(b) => self.st[O_RHO + b * CELLS + cell_at(x % BX, y, z % BX)],
        }
    }
}
