//! THE GAME'S OWN SMOKE ON A COARSE GRID — what `SENSE.grid` puts where the blobs stood (src/game/sense.ts): the smoke the SENSES see through, as density in
//! half-metre cells held in bricks (a brick is 2 m square in plan and 4 m high: floor to ceiling), stepped ten times a second. It is fed by the very emitters
//! the picture's solver is handed and knows nothing else of that solver: no velocity, no pressure, no wind. What it has that the blobs had not is THE WALLS:
//! smoke passes from a cell to its neighbour only where nothing solid and no leaf stands between their centres, so a canister fills the room it is in, goes
//! out by the open door and stops at the shut one.
//!
//! THE STATE is a part of the game's block (state.rs; `Smoke`): one row of doubles (`LEN`): a head (the plan in bricks, the emitters live, the steps
//! taken), the emitters still letting out, which brick is where, the light that comes round (glow.rs), and LAST the density of every cell, a brick after
//! another. It holds indices and numbers only, so it is the same bytes in any slot, in a snapshot or on another machine. The part is a POOL: its first
//! word says how many of its bytes are the game's (the row up to the end of the highest brick that stands), and the host carries those and no more.
//! WHAT IS NOT STATE and is made again from the state and the box world, a set of tables a slot of the blocks: where each brick is (`INDEX`, from the
//! state alone) and which faces of each cell are open (`FACES`: asked of the box world's lines once a cell; again round every leaf at every step, since a
//! door swings, and wherever the host says a static was written). So the faces are at every step what asking them all afresh would give: a game put back
//! from a snapshot and one that never was step alike. WHOSE the tables of a slot are is three numbers (`MADE`): the block's stamp they were made from
//! (src/sim/state.ts SimState.stamp: another number whenever a block was written wholesale or changed place), and the box world and the copy of it the
//! faces were asked of. A call names the three as they are now; where one differs the tables are made again before anything is read from them.
//!
//! The arithmetic is + − × ÷ and the game's own exp and length, in an order that is written down (bricks by slot, cells by index): the same bits on every
//! machine.

use crate::lines;
use crate::maths::{exp, hypot3};
use crate::state::{state, GAMES};
use crate::world::{js_max, js_min, world, ASK_HOST, K_ROT, LEAF_SETS, MAX_LEAVES, REC, R_C, R_H, R_KIND};
use core::mem::size_of;
use prim::floor;

/// metres a cell
pub const CELL: f64 = 0.5;
/// cells along a brick's side in plan, and up
pub const BX: usize = 4;
pub const BY: usize = 8;
pub const CELLS: usize = BX * BX * BY;
/// bricks a game can hold (one canister in the open is in about twenty by the time it is spent)
pub const MAX_BRICKS: usize = 160;
/// emitters letting out at once (the solver's own list holds as many)
pub const MAX_EMIT: usize = 64;
/// DOUBLES AN EMITTER: where it stands (x y z), what it lets out a second (unit·m³), how long it runs, how long it has, how wide it is (m)
pub const EMIT: usize = 7;
/// the head: emitters live · steps taken · the plan in bricks, x then z (0 = no step has been taken on any level yet) · THE BOOK OF THE SMOKE, three
/// sums in unit·m³ that thin at the cells' own rate, so that at every step what the cells hold + what was given back + what was refused = what was let
/// out: all that was let out · what thin bricks held when they were given back · what an emitter could not let out because every brick was taken
pub const HEAD: usize = 8;
const H_EMIT: usize = 0;
pub const H_STEPS: usize = 1;
const H_NBX: usize = 2;
const H_NBZ: usize = 3;
pub const H_OUT: usize = 4;
pub const H_GIVEN: usize = 5;
pub const H_LOST: usize = 6;
/// 1 while the grid does not hold all it was asked to: the last step wanted a brick there was no slot for (smoke that could not be let out, or could
/// not spread on), or what was refused is still more than nothing (the step's `gone`). `trans` then answers that the grid cannot
const H_OVER: usize = 7;
pub const O_EMIT: usize = HEAD;
/// which brick is where: its place in plan, x then z, EACH PLUS ONE (0 = the slot is free: a row of zeros is an empty grid)
pub const O_KEYS: usize = O_EMIT + MAX_EMIT * EMIT;
/// THE LIGHT THAT COMES ROUND (glow.rs), a number a square metre of the plan in four columns: the lamps' light at its middle · the bounced light there ·
/// is the way open to the metre toward +x · and toward +z (1 or 0). Zeros where nothing has swept: no light.
pub const MAX_GLOW: usize = 2048;
pub const O_GLOW_D: usize = O_KEYS + MAX_BRICKS * 2;
pub const O_GLOW_B: usize = O_GLOW_D + MAX_GLOW;
pub const O_GLOW_X: usize = O_GLOW_B + MAX_GLOW;
pub const O_GLOW_Z: usize = O_GLOW_X + MAX_GLOW;
/// …and the word of the lamps that field was last swept WHOLE under (glow.rs; 0 = never, or not since the light model was last down)
pub const O_GLOW_LAMPS: usize = O_GLOW_Z + MAX_GLOW;
/// THE CELLS, last: brick b's are the `CELLS` doubles from `O_RHO + b * CELLS`, so the row is in use up to the end of the highest brick that stands
pub const O_RHO: usize = O_GLOW_LAMPS + 1;
pub const LEN: usize = O_RHO + MAX_BRICKS * CELLS;
/// seconds a step
pub const SUB: f64 = 0.1;
/// the bricks of the largest plan a grid stands on (a level's plan in 2 m squares)
const MAX_INDEX: usize = 8192;
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
static mut INDEX: [[u8; MAX_INDEX]; GAMES] = [[0; MAX_INDEX]; GAMES];
/// a cell's faces: 1 = open toward +x · 2 = open upward · 4 = open toward +z · 8 = asked (the three bits are an answer)
static mut FACES: [[u8; MAX_BRICKS * CELLS]; GAMES] = [[0; MAX_BRICKS * CELLS]; GAMES];
/// the leaves as the last step found them, a row each as the box world holds it, and how many
static mut LEAF_AT: [[f64; MAX_LEAVES * REC]; GAMES] = [[0.0; MAX_LEAVES * REC]; GAMES];
static mut LEAF_N: [usize; GAMES] = [0; GAMES];
/// WHOSE A SLOT'S TABLES ARE: the stamp of the block INDEX was made from · the box world FACES and the leaves were asked of · and the copy of it (the
/// host's numbers, never 0: 0 = made from nothing yet)
static mut MADE: [[f64; 3]; GAMES] = [[0.0; 3]; GAMES];
const M_STAMP: usize = 0;
const M_WORLD: usize = 1;
const M_COPY: usize = 2;
/// a step's change of every cell, summed before any is applied; and which bricks the step left with next to nothing in them
static mut DELTA: [f64; MAX_BRICKS * CELLS] = [0.0; MAX_BRICKS * CELLS];
static mut THIN: [bool; MAX_BRICKS] = [false; MAX_BRICKS];
static mut HELD: [f64; MAX_BRICKS] = [0.0; MAX_BRICKS];

const F_X: u8 = 1;
const F_Y: u8 = 2;
const F_Z: u8 = 4;
const F_ASKED: u8 = 8;

#[inline(always)]
fn made(g: usize) -> &'static mut [f64; 3] {
    unsafe { &mut (*core::ptr::addr_of_mut!(MADE))[g] }
}

struct Grid {
    /// the slot of the blocks this grid's state stands in (a slot that is not one traps: state.rs)
    g: usize,
    st: &'static mut [f64; LEN],
    index: &'static mut [u8; MAX_INDEX],
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
    if x >= 1.0 && z >= 1.0 && x * z <= MAX_INDEX as f64 && floor(x) == x && floor(z) == z {
        (x as usize, z as usize)
    } else {
        (0, 0)
    }
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
    /// the brick at (bx, bz), made if a slot is free (the lowest free one): None = off the plan, or every slot is taken
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
                self.index[bz as usize * self.nbx + bx as usize] = b as u8 + 1;
                for c in 0..CELLS {
                    self.st[O_RHO + b * CELLS + c] = 0.0;
                    self.faces[b * CELLS + c] = 0;
                }
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
    }
    /// where each brick is, from the state (a brick the state names off its own plan, or where another already is, is given back)
    fn reindex(&mut self) {
        for i in 0..MAX_INDEX {
            self.index[i] = 0;
        }
        for b in 0..MAX_BRICKS {
            if let Some((bx, bz)) = self.key(b) {
                if bx >= 0 && bz >= 0 && (bx as usize) < self.nbx && (bz as usize) < self.nbz && self.index[bz as usize * self.nbx + bx as usize] == 0 {
                    self.index[bz as usize * self.nbx + bx as usize] = b as u8 + 1;
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
    if m[M_STAMP] != stamp {
        m[M_STAMP] = stamp;
        m[M_COPY] = 0.0;
        gr.remake();
    }
    gr
}
/// …and its faces were asked of copy `copy` of box world `w`, or none is known now (the copy was made afresh; the game steps over another world)
fn over(g: usize, w: usize, copy: f64) {
    let m = made(g);
    if m[M_WORLD] == w as f64 && m[M_COPY] == copy {
        return;
    }
    m[M_WORLD] = w as f64;
    m[M_COPY] = copy;
    grid(g).forget();
}
/// A STATIC WAS WRITTEN inside this rectangle of the plan of box world `w` (where it stood, or where it stands): the faces near it are asked again at
/// the next step, in every set of tables whose faces were asked of that world
pub fn dirty(w: usize, x0: f64, z0: f64, x1: f64, z1: f64) {
    for g in 0..GAMES {
        let m = made(g);
        if m[M_COPY] != 0.0 && m[M_WORLD] == w as f64 {
            grid(g).dirty(x0, z0, x1, z1);
        }
    }
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
    *made(g) = [0.0; 3];
}
/// NO SMOKE ANYWHERE: the cells, the emitters, the book and the count of steps are zeros. The plan the grid stands on and the light that comes round
/// stay: they are the level's, not the smoke's.
pub fn empty(g: usize) {
    let mut gr = grid(g);
    for h in 0..HEAD {
        if h != H_NBX && h != H_NBZ {
            gr.st[h] = 0.0;
        }
    }
    for v in gr.st[O_EMIT..O_GLOW_D].iter_mut() {
        *v = 0.0;
    }
    for v in gr.st[O_RHO..].iter_mut() {
        *v = 0.0;
    }
    gr.relive();
    *made(g) = [0.0; 3];
}

/// is the way between two cells' centres open: nothing solid and no leaf between them, asked both ways (a line that STARTS inside a box is not stopped by
/// it — collision.ts segmentBlocked's own rule for an eye in a wall — so a cell inside a wall is shut off by the line that ends in it)
fn open(w: usize, set: i32, a: [f64; 3], b: [f64; 3]) -> bool {
    clear(w, set, a, b, CELL) && clear(w, set, b, a, CELL)
}
/// nothing solid and no leaf on the line a → b of this length (the one place this file walks a line: the walk is not copied into every asker)
#[inline(never)]
fn clear(w: usize, set: i32, a: [f64; 3], b: [f64; 3], len: f64) -> bool {
    clear_but(w, set, a, b, len, 0)
}
/// …the same line for what passes boxes of the flags `through` (glow.rs: light goes through a pane, smoke does not)
#[inline(never)]
pub(crate) fn clear_but(w: usize, set: i32, a: [f64; 3], b: [f64; 3], len: f64, through: u32) -> bool {
    lines::blocked(w, a[0], a[1], a[2], b[0], b[1], b[2], len, 0.02, through, set, 0) != -1
}
/// can what is let out at `p` reach the point `c` (a line of any length; one too short to be a line is open)
fn reaches(w: usize, set: i32, p: [f64; 3], c: [f64; 3]) -> bool {
    let len = hypot3(c[0] - p[0], c[1] - p[1], c[2] - p[2]);
    len < 1e-4 || clear(w, set, p, c, len)
}

/// what a step is told: the model's numbers (sense.ts SENSE.smoke)
pub struct K {
    pub decay: f64,
    pub spread: f64,
    pub rise: f64,
    pub rise_rate: f64,
    pub gone: f64,
}

/// LET OUT: an emitter joins the list. When the list is full the one with the least still to let out gives way, and that may be the newcomer.
fn emit(g: &mut Grid, x: f64, y: f64, z: f64, radius: f64, rate: f64, ttl: f64) {
    // (an emitter that is no number lets nothing out, and must not sit in the list as one)
    if !(x - x == 0.0 && y - y == 0.0 && z - z == 0.0 && radius >= 0.0 && radius < 1e6 && rate > 0.0 && ttl > 0.0 && rate * ttl < 1e12) {
        return;
    }
    let n = js_min(g.st[H_EMIT], MAX_EMIT as f64) as usize;
    let mut at = n;
    if n >= MAX_EMIT {
        let left = |st: &[f64; LEN], i: usize| st[O_EMIT + i * EMIT + 3] * js_max(0.0, st[O_EMIT + i * EMIT + 4] - st[O_EMIT + i * EMIT + 5]);
        let mut k = 0;
        for i in 1..n {
            if left(g.st, i) < left(g.st, k) {
                k = i;
            }
        }
        if left(g.st, k) >= rate * ttl {
            return;
        }
        at = k;
    } else {
        g.st[H_EMIT] = (n + 1) as f64;
    }
    let o = O_EMIT + at * EMIT;
    g.st[o] = x;
    g.st[o + 1] = y;
    g.st[o + 2] = z;
    g.st[o + 3] = rate;
    g.st[o + 4] = ttl;
    g.st[o + 5] = 0.0;
    g.st[o + 6] = radius;
}

/// `n` STEPS of a tenth of a second each of the grid of the game in slot `g` (its block's `stamp`), over the copy `copy` of box world `w` as it stands
/// and the leaves of list `set`; the emitters handed over since the last step (`emits`: x y z, how wide, what it lets out a second, for how long, six
/// doubles each) join the list first. 0, or ASK_HOST = no grid stands on this level (its plan is larger than the index), or the game has none: the
/// state is as it was, and no emitter was taken.
pub fn step(g: usize, stamp: f64, w: usize, copy: f64, set: i32, n: u32, emits: &[f64], k: &K, glow: Option<&crate::glow::K>) -> u32 {
    let o = world(w);
    let (mx, mz) = (o.w as f64 * o.cs, o.h as f64 * o.cs);
    let (nbx, nbz) = (-floor(-mx / SIDE), -floor(-mz / SIDE));
    if !(nbx >= 1.0 && nbz >= 1.0 && nbx * nbz <= MAX_INDEX as f64) || set >= LEAF_SETS as i32 || state(g).smoke.live == 0 {
        return ASK_HOST;
    }
    tables(g, stamp);
    over(g, w, copy);
    let mut g = grid(g);
    if g.st[H_NBX] != nbx || g.st[H_NBZ] != nbz {
        // (the first step on this level: a state from any other plan holds nothing that is anywhere here)
        g.st[H_NBX] = nbx;
        g.st[H_NBZ] = nbz;
        for v in g.st[O_GLOW_D..O_RHO].iter_mut() {
            *v = 0.0;
        }
        g.nbx = nbx as usize;
        g.nbz = nbz as usize;
        g.remake();
    }
    for e in emits.chunks_exact(6) {
        emit(&mut g, e[0], e[1], e[2], e[3], e[4], e[5]);
    }
    for _ in 0..n {
        substep(w, set, &mut g, k);
        match glow {
            Some(gk) => crate::glow::step(w, set, g.st, g.nbx, g.nbz, gk),
            None => crate::glow::down(g.st),
        }
        g.st[H_STEPS] += 1.0;
    }
    g.relive();
    0
}

fn substep(w: usize, set: i32, g: &mut Grid, k: &K) {
    // 0. the faces round every leaf that is not as the last step found it (a door swings; one is new, one is gone) are asked again: where it stood and
    //    where it stands. A leaf that has not moved keeps the answers it had.
    {
        let s = g.g;
        let (kept, kept_n) = unsafe { (&mut (*core::ptr::addr_of_mut!(LEAF_AT))[s], &mut (*core::ptr::addr_of_mut!(LEAF_N))[s]) };
        let o = world(w);
        let n = if set < 0 { 0 } else { js_min(o.leaves[set as usize] as f64, MAX_LEAVES as f64) as usize };
        // (its reach in plan whichever way it is turned: the two half sides, and the third where it is tilted off the upright)
        let reach = |r: &[f64]| r[R_H] + r[R_H + 2] + if r[R_KIND] as u32 & K_ROT != 0 { r[R_H + 1] } else { 0.0 };
        for i in 0..if n > *kept_n { n } else { *kept_n } {
            let at = if set < 0 { 0 } else { (set as usize * MAX_LEAVES + i) * REC };
            let (was, now) = (&mut kept[i * REC..(i + 1) * REC], &o.leaf[at..at + REC]);
            let (had, has) = (i < *kept_n, i < n);
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

    // (did this step want a brick there was no slot for)
    let mut refused = false;
    // 1. the emitters let out, each up the column it stands under (from its own foot to the top of its plume: its width, and the rise the blobs had),
    //    and the spent ones leave the list
    let n = js_min(g.st[H_EMIT], MAX_EMIT as f64) as usize;
    let mut kept = 0;
    for i in 0..n {
        let o = O_EMIT + i * EMIT;
        let (x, y0, z, rate, ttl, age, radius) = (g.st[o], g.st[o + 1], g.st[o + 2], g.st[o + 3], g.st[o + 4], g.st[o + 5], g.st[o + 6]);
        if age < ttl {
            let amount = rate * js_min(SUB, ttl - age);
            let top = (BY - 1) as f64;
            let lo = js_min(js_max(floor((y0 - radius) / CELL), 0.0), top);
            let hi = js_min(js_max(floor((y0 + radius + js_min(k.rise, k.rise_rate * age)) / CELL), lo), top);
            let (fx, fz) = (floor(x / CELL), floor(z / CELL));
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
                    if reaches(w, set, p, [(tx + 0.5) * CELL, (js_min(js_max(floor(p[1] / CELL), 0.0), top) + 0.5) * CELL, (tz + 0.5) * CELL]) {
                        cx = tx;
                        cz = tz;
                        break;
                    }
                }
                let (ux, uz) = (cx as usize, cz as usize);
                g.st[H_OUT] += amount;
                if let Some(b) = g.brick_or_new((ux / BX) as i32, (uz / BX) as i32) {
                    let each = amount / VOL / (hi - lo + 1.0);
                    let mut y = lo;
                    while y <= hi {
                        g.st[O_RHO + b * CELLS + cell_at(ux % BX, y as usize, uz % BX)] += each;
                        y += 1.0;
                    }
                } else {
                    // (every brick is taken: this smoke is in no cell. The book has it, and while it is more than nothing `trans` says the grid cannot
                    //  answer)
                    g.st[H_LOST] += amount;
                    refused = true;
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
            kept += 1;
        }
    }
    for i in kept * EMIT..n * EMIT {
        g.st[O_EMIT + i] = 0.0;
    }
    g.st[H_EMIT] = kept as f64;

    // 2. a brick gets the neighbour it would spill into (while there are slots): where a cell at its edge holds smoke AND the way out of that cell
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
                    if open(w, set, p, [p[0] + dx as f64 * CELL, p[1], p[2] + dz as f64 * CELL]) {
                        wanted = true;
                        break 'edge;
                    }
                }
            }
            if wanted && g.brick_or_new(nx, nz).is_none() {
                refused = true;
            }
        }
    }

    // 3. the faces nobody has asked (a new brick's, those round a leaf, those round a static that was written)
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
                    if open(w, set, p, [p[0] + CELL, p[1], p[2]]) {
                        f |= F_X;
                    }
                    if y + 1 < BY && open(w, set, p, [p[0], p[1] + CELL, p[2]]) {
                        f |= F_Y;
                    }
                    if open(w, set, p, [p[0], p[1], p[2] + CELL]) {
                        f |= F_Z;
                    }
                    g.faces[c] = f;
                }
            }
        }
    }

    // 4. smoke passes across every open face toward the thinner side: each cell's change summed from the densities as the step found them, then applied
    let delta = unsafe { &mut *core::ptr::addr_of_mut!(DELTA) };
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
        for y in 0..BY {
            for z in 0..BX {
                for x in 0..BX {
                    let c = b * CELLS + cell_at(x, y, z);
                    let (f, here) = (g.faces[c], g.st[O_RHO + c]);
                    if f & F_X != 0 {
                        let n = if x + 1 < BX { Some(c + 1) } else { east.map(|e| e * CELLS + cell_at(0, y, z)) };
                        if let Some(n) = n {
                            let pass = rate * (here - g.st[O_RHO + n]);
                            delta[c] -= pass;
                            delta[n] += pass;
                        }
                    }
                    if f & F_Y != 0 {
                        let n = c + BX * BX;
                        let pass = rate * (here - g.st[O_RHO + n]);
                        delta[c] -= pass;
                        delta[n] += pass;
                    }
                    if f & F_Z != 0 {
                        let n = if z + 1 < BX { Some(c + BX) } else { south.map(|s| s * CELLS + cell_at(x, y, 0)) };
                        if let Some(n) = n {
                            let pass = rate * (here - g.st[O_RHO + n]);
                            delta[c] -= pass;
                            delta[n] += pass;
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
    for h in [H_OUT, H_GIVEN, H_LOST] {
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
        for i in 0..js_min(g.st[H_EMIT], MAX_EMIT as f64) as usize {
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
    g.st[H_GIVEN] += given;
    g.st[H_OVER] = if refused || g.st[H_LOST] >= k.gone { 1.0 } else { 0.0 };
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
/// −1 = THE GRID CANNOT ANSWER: there is more smoke than bricks, so some is in no cell or stands piled where it could not spread on, and a line would
/// read clearer than it is (the host's blobs, which are stepped beside the grid, answer until the grid holds everything again).
pub fn trans(g: usize, stamp: f64, a: [f64; 3], b: [f64; 3], sigma: f64) -> f64 {
    let g = tables(g, stamp);
    if g.nbx == 0 {
        return 1.0;
    }
    if g.st[H_OVER] != 0.0 {
        return -1.0;
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
