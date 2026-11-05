//! A WAY ACROSS THE FLOOR — src/scene/collision.ts NavGrid.findPath (A* over the half-metre grid, then the greedy smoothing), nearestFree and
//! walkable; with a penalty too, whose prices the host hands in as the search comes to need them (`find_priced`, `resume`). The rule is the TypeScript's, statement for statement: the same cells popped in the same order, the same ties, the same waypoints.
//!
//! The grid of shut cells is the host's (it bakes it and re-bakes it under a blast); it writes it here before it asks. Everything in this file is scratch of
//! one question: the search's four tables stand blank between searches, as the TypeScript's do. The scores are KEPT in singles, as the TypeScript keeps them
//! in Float32Arrays (single.rs), and worked out in doubles.
//!
//! The lengths (the smoothing's, and the lines nearestFree asks) are the game's own, so this answers only where EXACT.on.

use crate::lines;
use crate::maths::hypot2;
use crate::single::{get, put, S};
use crate::world::{js_abs, js_max, js_min, or_one};
use prim::floor;

/// the cells the tables of a search are laid out for (a floor is its box grid's, four nav cells to the metre square)
pub const MAX_NAV: usize = 1 << 16;
/// the open list and the list of cells written
const MAX_OPEN: usize = 1 << 17;
/// the waypoints a way can have
pub const MAX_WAY: usize = 1 << 13;
/// THE FLOOR A SEARCH CAN HOLD, in cells: what the host is told (`nav_cap(0)`) and the most `begin` takes. On a floor of no more cells than this no
/// table here can be outgrown, whatever cap the caller puts on the search: a search closes a cell once and lists eight more at most as it does (so the
/// open list and the list of cells written hold 1 + 8 × cells entries at most), and a way has no more waypoints than the floor has cells. The floor is
/// the BUILD's (src/scene/world.ts WORLD: 40 m × 28 m, four cells to the metre square, 4,480 cells), so one over this is a mistake of the build, said
/// once by the host (collision.ts NavGrid.there), and never a level's.
pub const MAX_FLOOR: usize = least(least(MAX_NAV, MAX_WAY), (MAX_OPEN - 1) / 8);
const fn least(a: usize, b: usize) -> usize {
    if a < b {
        a
    } else {
        b
    }
}

static mut SHUT: [u8; MAX_NAV] = [0; MAX_NAV];
static mut G: [S; MAX_NAV] = [S::ZERO; MAX_NAV];
static mut F: [S; MAX_NAV] = [S::ZERO; MAX_NAV];
/// the cell a cell was reached from, PLUS ONE (0 = none: the TypeScript's −1)
static mut CAME: [u32; MAX_NAV] = [0; MAX_NAV];
static mut CLOSED: [u8; MAX_NAV] = [0; MAX_NAV];
static mut OPEN: [u32; MAX_OPEN] = [0; MAX_OPEN];
static mut SEEN: [u32; MAX_OPEN] = [0; MAX_OPEN];
static mut CELLS: [u32; MAX_NAV] = [0; MAX_NAV];
static mut WAY: [f64; MAX_WAY * 2] = [0.0; MAX_WAY * 2];
/// the scores have been made blank (infinite) once: they are zero as the module loads, and a table of infinities would be bytes in the file
static mut BLANK: bool = false;

/// where the host writes the shut cells (a byte address), and where a way's waypoints are (x z a point)
pub fn shut_addr() -> usize {
    core::ptr::addr_of!(SHUT) as usize
}
pub fn way_addr() -> usize {
    core::ptr::addr_of!(WAY) as usize
}

const SQRT2: f64 = 1.4142135623730951;

pub const NO_WAY: i32 = -1;
pub const ASK_HOST: i32 = -2;

/// Math.ceil
#[inline(always)]
fn ceil(x: f64) -> f64 {
    -floor(-x)
}
/// collision.ts clampi over a floored coordinate, as a cell
#[inline(always)]
fn cell_of(x: f64, cell: f64, n: usize) -> usize {
    let c = floor(x / cell);
    let top = (n - 1) as f64;
    if c < 0.0 {
        0
    } else if c > top {
        n - 1
    } else {
        c as usize
    }
}

/// the grid of one question
pub struct Grid {
    pub gw: usize,
    pub gh: usize,
    pub cell: f64,
}
impl Grid {
    /// NavGrid.isBlocked: off the grid is shut
    #[inline]
    fn is_blocked(&self, shut: &[u8], wx: f64, wz: f64) -> bool {
        let (x, z) = (floor(wx / self.cell), floor(wz / self.cell));
        if x < 0.0 || z < 0.0 || x >= self.gw as f64 || z >= self.gh as f64 {
            return true;
        }
        shut[z as usize * self.gw + x as usize] == 1
    }
    /// NavGrid.walkable
    fn walkable(&self, shut: &[u8], a0: f64, a2: f64, b0: f64, b2: f64) -> bool {
        let (dx, dz) = (b0 - a0, b2 - a2);
        let len = hypot2(dx, dz);
        let n = ceil(len / (self.cell * 0.5)) + 1.0;
        let (px, pz) = (-dz / or_one(len) * 0.22, dx / or_one(len) * 0.22);
        let mut i = 0.0;
        while i <= n {
            let t = i / n;
            let (x, z) = (a0 + dx * t, a2 + dz * t);
            if self.is_blocked(shut, x, z) || self.is_blocked(shut, x + px, z + pz) || self.is_blocked(shut, x - px, z - pz) {
                return false;
            }
            i += 1.0;
        }
        true
    }
    /// NavGrid.nearestFree for a world point that asked: Ok(the cell, or none), Err = a line the host must judge
    fn nearest_free(&self, shut: &[u8], w: usize, set: i32, ask: u32, gx: usize, gz: usize, wx: f64, wz: f64) -> Result<Option<(usize, usize)>, ()> {
        let mut fallback: Option<(usize, usize)> = None;
        let mut extra: i32 = 1;
        let (gx, gz) = (gx as i32, gz as i32);
        for r in 1..8i32 {
            for dz in -r..=r {
                for dx in -r..=r {
                    let (ax, az) = (if dx < 0 { -dx } else { dx }, if dz < 0 { -dz } else { dz });
                    if (if ax > az { ax } else { az }) != r {
                        continue;
                    }
                    let (x, z) = (gx + dx, gz + dz);
                    if x < 0 || z < 0 || x >= self.gw as i32 || z >= self.gh as i32 || shut[z as usize * self.gw + x as usize] != 0 {
                        continue;
                    }
                    let c = (x as usize, z as usize);
                    if fallback.is_none() {
                        fallback = Some(c);
                    }
                    match lines::solid_line(w, set, ask, wx, 1.0, wz, (x as f64 + 0.5) * self.cell, 1.0, (z as f64 + 0.5) * self.cell) {
                        None => return Err(()),
                        Some(false) => return Ok(Some(c)),
                        Some(true) => {}
                    }
                }
            }
            if fallback.is_some() {
                let e = extra;
                extra -= 1;
                if e <= 0 {
                    return Ok(fallback);
                }
            }
        }
        Ok(fallback)
    }
}

/// findPath(a, b, max_iter) with no penalty: how many waypoints were written (x z each, at way_addr) · NO_WAY (the TypeScript's null) · ASK_HOST
#[allow(clippy::too_many_arguments)]
pub fn find_path(w: usize, set: i32, ask: u32, gw: usize, gh: usize, cell: f64, a0: f64, a2: f64, b0: f64, b2: f64, max_iter: u32) -> i32 {
    begin(&mut Flat, w, set, ask, gw, gh, cell, a0, a2, b0, b2, max_iter)
}
/// findPath(a, b, max_iter, penalty): as `find_path`, or NEED = the search has come to cells whose price it does not have (`need_len` of them at
/// `need_addr`, a cell's index each). The host writes each one's price at `price_addr` (a double a cell, by the cell's index) and calls `resume`,
/// which answers as this does. A SEARCH THAT WAITS IS SCRATCH OF ONE QUESTION: until it is resumed to its end or dropped (`drop_run`), every
/// other search asked here is the host's (ASK_HOST), so the host never asks one from inside a price.
#[allow(clippy::too_many_arguments)]
pub fn find_priced(w: usize, set: i32, ask: u32, gw: usize, gh: usize, cell: f64, a0: f64, a2: f64, b0: f64, b2: f64, max_iter: u32) -> i32 {
    begin(&mut Asked, w, set, ask, gw, gh, cell, a0, a2, b0, b2, max_iter)
}
pub fn need_addr() -> usize {
    core::ptr::addr_of!(WANT) as usize
}
pub fn need_len() -> usize {
    unsafe { *core::ptr::addr_of!(N_WANT) }
}
pub fn price_addr() -> usize {
    core::ptr::addr_of!(PRICE) as usize
}

/// a search asked for prices it does not have (the cells are at `need_addr`)
pub const NEED: i32 = -3;
/// the cells one NEED can name (a search's step names eight at most; a leg of the smoothing, the cells along it)
pub const MAX_NEED: usize = 1024;

/// WHAT A STEP INTO A CELL COSTS, TO ONE SEARCH (collision.ts findPath's `penalty`: a multiplier, read as 1 where it is under 1). THE SEARCH IS
/// WRITTEN ONCE, OVER THIS: a search with no penalty is the search whose every price is 1 (`Flat`); one with a penalty of the host's is the search
/// over the prices the host has handed in so far (`Asked`), which does not know a cell until the host has priced it. A rule that moves here
/// brings a source of its own that always knows, and the waiting below goes with `Asked`.
trait Prices {
    /// every price is 1: no cell is dear, so the smoothing has none to count along a leg
    const FLAT: bool;
    /// the price of cell `c`, or None = not known yet: the source has noted the cell, and the search stops where it is and says NEED
    fn at(&mut self, c: usize) -> Option<f64>;
}
struct Flat;
impl Prices for Flat {
    const FLAT: bool = true;
    #[inline(always)]
    fn at(&mut self, _c: usize) -> Option<f64> {
        Some(1.0)
    }
}
/// the host's prices, a cell at a time as the search comes to need them (PRICE, KNOWN, WANT below)
struct Asked;
impl Prices for Asked {
    const FLAT: bool = false;
    #[inline(always)]
    fn at(&mut self, c: usize) -> Option<f64> {
        let (price, known) = unsafe { (&*core::ptr::addr_of!(PRICE), &*core::ptr::addr_of!(KNOWN)) };
        if known[c] != 0 {
            Some(price[c])
        } else {
            want_cell(c);
            None
        }
    }
}
/// THE HOST'S PRICES OF ONE SEARCH, a double a cell, written by the host where the search asked; which cells have one; and the list of those, so
/// that the table is blank again as the search ends
static mut PRICE: [f64; MAX_NAV] = [0.0; MAX_NAV];
static mut KNOWN: [u8; MAX_NAV] = [0; MAX_NAV];
static mut PRICED: [u32; MAX_NAV] = [0; MAX_NAV];
static mut WANT: [u32; MAX_NEED] = [0; MAX_NEED];
/// …and how many it names now. Its own word, not the run's: the source of prices writes it while the search holds the run
static mut N_WANT: usize = 0;
/// A SEARCH BETWEEN ITS CALLS: where it stands, kept here so that one that waits for prices can go on (only `Asked` ever waits)
struct Run {
    live: bool,
    /// the search is done and its way's cells stand in CELLS: what is left is the smoothing, which is run from its top at every resume
    smoothing: bool,
    /// `cur` was popped and closed, and waits for its neighbours' prices
    held: bool,
    gw: usize,
    gh: usize,
    cell: f64,
    a0: f64,
    a2: f64,
    start: usize,
    goal: usize,
    gx: usize,
    gz: usize,
    n_open: usize,
    n_seen: usize,
    iter: u32,
    max_iter: u32,
    cur: usize,
    n: usize,
    n_priced: usize,
}
static mut RUN: Run = Run { live: false, smoothing: false, held: false, gw: 0, gh: 0, cell: 0.0, a0: 0.0, a2: 0.0, start: 0, goal: 0, gx: 0, gz: 0, n_open: 0, n_seen: 0, iter: 0, max_iter: 0, cur: 0, n: 0, n_priced: 0 };

#[allow(clippy::too_many_arguments)]
fn begin<P: Prices>(src: &mut P, w: usize, set: i32, ask: u32, gw: usize, gh: usize, cell: f64, a0: f64, a2: f64, b0: f64, b2: f64, max_iter: u32) -> i32 {
    let run = unsafe { &mut *core::ptr::addr_of_mut!(RUN) };
    // (a search asked while another waits for its prices would write over that one's tables)
    if run.live {
        return ASK_HOST;
    }
    // (a floor of more cells than a search holds: the host has said so by name before it asks, NavGrid.there. Each side is under MAX_FLOOR before the
    //  two are multiplied: the product cannot wrap)
    if gw == 0 || gh == 0 || gw > MAX_FLOOR || gh > MAX_FLOOR || gw * gh > MAX_FLOOR {
        return ASK_HOST;
    }
    // (a point that is no number has no cell: the TypeScript's own answer for it, whatever that is, is the host's to give)
    if !(a0 - a0 == 0.0 && a2 - a2 == 0.0 && b0 - b0 == 0.0 && b2 - b2 == 0.0) || !(cell > 0.0) {
        return ASK_HOST;
    }
    let (shut, g, f, open, seen, way) = unsafe {
        (
            &*core::ptr::addr_of!(SHUT),
            &mut *core::ptr::addr_of_mut!(G),
            &mut *core::ptr::addr_of_mut!(F),
            &mut *core::ptr::addr_of_mut!(OPEN),
            &mut *core::ptr::addr_of_mut!(SEEN),
            &mut *core::ptr::addr_of_mut!(WAY),
        )
    };
    unsafe {
        if !BLANK {
            for i in 0..MAX_NAV {
                g[i] = S::INF;
                f[i] = S::INF;
            }
            BLANK = true;
        }
    }
    let grid = Grid { gw, gh, cell };
    let (sx, sz) = (cell_of(a0, cell, gw), cell_of(a2, cell, gh));
    let (mut gx, mut gz) = (cell_of(b0, cell, gw), cell_of(b2, cell, gh));
    if shut[gz * gw + gx] != 0 {
        match grid.nearest_free(shut, w, set, ask, gx, gz, b0, b2) {
            Err(()) => return ASK_HOST,
            Ok(None) => return NO_WAY,
            Ok(Some((x, z))) => {
                gx = x;
                gz = z;
            }
        }
    }
    let (start, goal) = (sz * gw + sx, gz * gw + gx);
    if start == goal {
        way[0] = (gx as f64 + 0.5) * cell;
        way[1] = (gz as f64 + 0.5) * cell;
        return 1;
    }
    {
        let (x, z) = ((start % gw) as f64, (start / gw) as f64);
        let (dx, dz) = (js_abs(x - gx as f64), js_abs(z - gz as f64));
        put(g, start, 0.0);
        put(f, start, (dx + dz) + (SQRT2 - 2.0) * js_min(dx, dz));
    }
    open[0] = start as u32;
    seen[0] = start as u32;
    *run = Run { live: true, smoothing: false, held: false, gw, gh, cell, a0, a2, start, goal, gx, gz, n_open: 1, n_seen: 1, iter: 0, max_iter, cur: 0, n: 0, n_priced: 0 };
    unsafe { *core::ptr::addr_of_mut!(N_WANT) = 0 };
    go(src)
}
/// the prices asked for are written: the search goes on from where it waited. ASK_HOST = no search waits.
pub fn resume() -> i32 {
    let run = unsafe { &mut *core::ptr::addr_of_mut!(RUN) };
    if !run.live {
        return ASK_HOST;
    }
    let (known, priced, want) = unsafe { (&mut *core::ptr::addr_of_mut!(KNOWN), &mut *core::ptr::addr_of_mut!(PRICED), &*core::ptr::addr_of!(WANT)) };
    for &c in &want[..unsafe { *core::ptr::addr_of!(N_WANT) }] {
        let c = c as usize & (MAX_NAV - 1);
        if known[c] == 0 {
            known[c] = 1;
            priced[run.n_priced] = c as u32;
            run.n_priced += 1;
        }
    }
    unsafe { *core::ptr::addr_of_mut!(N_WANT) = 0 };
    go(&mut Asked)
}
/// a search that waits is let go (the host's price threw): its tables are blank again, and the next search is a new one
pub fn drop_run() {
    let run = unsafe { &mut *core::ptr::addr_of_mut!(RUN) };
    if !run.live {
        return;
    }
    if !run.smoothing {
        blank(run.n_seen);
    }
    end(0);
}
/// the search's four tables as they stood before it: every cell it wrote is in SEEN
fn blank(n_seen: usize) {
    let (g, f, came, closed, seen) = unsafe { (&mut *core::ptr::addr_of_mut!(G), &mut *core::ptr::addr_of_mut!(F), &mut *core::ptr::addr_of_mut!(CAME), &mut *core::ptr::addr_of_mut!(CLOSED), &*core::ptr::addr_of!(SEEN)) };
    for &c in &seen[..n_seen] {
        let c = c as usize & (MAX_NAV - 1);
        g[c] = S::INF;
        f[c] = S::INF;
        came[c] = 0;
        closed[c] = 0;
    }
}
/// the search has answered `r`: no price is known any more, and none waits
fn end(r: i32) -> i32 {
    let run = unsafe { &mut *core::ptr::addr_of_mut!(RUN) };
    let (known, priced) = unsafe { (&mut *core::ptr::addr_of_mut!(KNOWN), &*core::ptr::addr_of!(PRICED)) };
    for &c in &priced[..run.n_priced] {
        known[c as usize & (MAX_NAV - 1)] = 0;
    }
    run.n_priced = 0;
    unsafe { *core::ptr::addr_of_mut!(N_WANT) = 0 };
    run.live = false;
    r
}
/// a cell the search needs the price of and has not: named for the host, once (false = the list is full: what is named is priced first, and the
/// search asks again)
fn want_cell(c: usize) -> bool {
    let (want, n) = unsafe { (&mut *core::ptr::addr_of_mut!(WANT), &mut *core::ptr::addr_of_mut!(N_WANT)) };
    if want[..*n].iter().any(|&q| q as usize == c) {
        return true;
    }
    if *n >= MAX_NEED {
        return false;
    }
    want[*n] = c as u32;
    *n += 1;
    true
}
/// NavGrid.dearAlong: the distinct cells along the straight leg a → b (sampled at half a cell) whose price is above plain floor. Err = a cell on it
/// has no price yet (the leg is walked to its end all the same, so that every cell of it is asked for at once: the TypeScript reads them all)
fn dear_along<P: Prices>(src: &mut P, gw: usize, gh: usize, cell: f64, a0: f64, a2: f64, b0: f64, b2: f64) -> Result<u32, ()> {
    let (dx, dz) = (b0 - a0, b2 - a2);
    let n = ceil(hypot2(dx, dz) / (cell * 0.5)) + 1.0;
    let (mut last, mut count, mut missing) = (-1.0f64, 0u32, false);
    let mut k = 0.0;
    while k <= n {
        let t = k / n;
        k += 1.0;
        let (ix, iz) = (floor((a0 + dx * t) / cell), floor((a2 + dz * t) / cell));
        let i = iz * gw as f64 + ix;
        if i == last {
            continue;
        }
        last = i;
        if ix >= 0.0 && iz >= 0.0 && ix < gw as f64 && iz < gh as f64 {
            let c = iz as usize * gw + ix as usize;
            match src.at(c) {
                None => missing = true,
                Some(p) => {
                    if p > 1.0 {
                        count += 1;
                    }
                }
            }
        }
    }
    if missing {
        Err(())
    } else {
        Ok(count)
    }
}
/// THE SEARCH, from where it stands (its start, or where it waited), to its answer or its next wait
fn go<P: Prices>(src: &mut P) -> i32 {
    let run = unsafe { &mut *core::ptr::addr_of_mut!(RUN) };
    let (shut, g, f, came, closed, open, seen, cells, way) = unsafe {
        (
            &*core::ptr::addr_of!(SHUT),
            &mut *core::ptr::addr_of_mut!(G),
            &mut *core::ptr::addr_of_mut!(F),
            &mut *core::ptr::addr_of_mut!(CAME),
            &mut *core::ptr::addr_of_mut!(CLOSED),
            &mut *core::ptr::addr_of_mut!(OPEN),
            &mut *core::ptr::addr_of_mut!(SEEN),
            &mut *core::ptr::addr_of_mut!(CELLS),
            &mut *core::ptr::addr_of_mut!(WAY),
        )
    };
    let (gw, gh, cell) = (run.gw, run.gh, run.cell);
    let grid = Grid { gw, gh, cell };
    if !run.smoothing {
        let (start, goal, max_iter) = (run.start, run.goal, run.max_iter);
        let (gxf, gzf) = (run.gx as f64, run.gz as f64);
        let (mut n_open, mut n_seen, mut iter) = (run.n_open, run.n_seen, run.iter);
        let mut full = false;
        loop {
            let cur;
            if run.held {
                // (popped and closed before the wait: its neighbours' prices are here now)
                cur = run.cur;
                run.held = false;
            } else {
                if n_open == 0 {
                    break;
                }
                let it = iter;
                iter = iter.wrapping_add(1);
                if !(it < max_iter) {
                    break;
                }
                // pop the least f (the first of equals, as the scan finds it)
                // (a cell is under MAX_NAV, a power of two: the mask says so to the compiler, and the scan is then two loads and a compare an entry)
                let mut mi = 0usize;
                let mut mf = get(f, open[0] as usize & (MAX_NAV - 1));
                for (i, &c) in open[..n_open].iter().enumerate().skip(1) {
                    let fi = get(f, c as usize & (MAX_NAV - 1));
                    if fi < mf {
                        mi = i;
                        mf = fi;
                    }
                }
                cur = open[mi] as usize;
                n_open -= 1;
                open[mi] = open[n_open];
                if cur == goal {
                    break;
                }
                if closed[cur] != 0 {
                    continue;
                }
                closed[cur] = 1;
                // (cannot be on a floor `begin` took: MAX_FLOOR)
                if n_open + 8 > MAX_OPEN || n_seen + 8 > MAX_OPEN {
                    full = true;
                    break;
                }
            }
            // the cells a step from `cur` may enter, in the search's own order, said ONCE: both passes below go by this list (which cells they are
            // does not hang on what the second pass writes)
            let mut near = [(0usize, false); 8];
            let mut n_near = 0usize;
            {
                let (cx, cz) = ((cur % gw) as i32, (cur / gw) as i32);
                for dz in -1..=1i32 {
                    for dx in -1..=1i32 {
                        if dx == 0 && dz == 0 {
                            continue;
                        }
                        let (nx, nz) = (cx + dx, cz + dz);
                        if nx < 0 || nz < 0 || nx >= gw as i32 || nz >= gh as i32 {
                            continue;
                        }
                        let ni = nz as usize * gw + nx as usize;
                        if shut[ni] != 0 || closed[ni] != 0 {
                            continue;
                        }
                        if dx != 0 && dz != 0 && (shut[cz as usize * gw + (cx + dx) as usize] != 0 || shut[(cz + dz) as usize * gw + cx as usize] != 0) {
                            continue;
                        }
                        near[n_near] = (ni, dx != 0 && dz != 0);
                        n_near += 1;
                    }
                }
            }
            if !P::FLAT {
                // every one of them is priced before any is stepped into. One whose price is not known: the cell waits with all of them asked for,
                // and is opened when they are here
                let mut wait = false;
                for &(ni, _) in &near[..n_near] {
                    if src.at(ni).is_none() {
                        wait = true;
                    }
                }
                if wait {
                    run.held = true;
                    run.cur = cur;
                    run.n_open = n_open;
                    run.n_seen = n_seen;
                    run.iter = iter;
                    return NEED;
                }
            }
            for &(ni, diagonal) in &near[..n_near] {
                let step = if diagonal { SQRT2 } else { 1.0 };
                // (a price is here: the pass above saw to it, and a search that finds none all the same is the host's, not a guess. A step at the
                //  price of 1 is the step: the same double)
                let Some(price) = src.at(ni) else {
                    blank(n_seen);
                    return end(ASK_HOST);
                };
                let ng = get(g, cur) + step * js_max(1.0, price);
                if ng < get(g, ni) {
                    let (hx, hz) = (js_abs((ni % gw) as f64 - gxf), js_abs((ni / gw) as f64 - gzf));
                    put(g, ni, ng);
                    came[ni] = cur as u32 + 1;
                    put(f, ni, ng + ((hx + hz) + (SQRT2 - 2.0) * js_min(hx, hz)));
                    open[n_open] = ni as u32;
                    n_open += 1;
                    seen[n_seen] = ni as u32;
                    n_seen += 1;
                }
            }
        }
        let mut n = 0usize;
        let found = came[goal] != 0;
        if found && !full {
            let mut c = goal;
            loop {
                if c == start {
                    break;
                }
                cells[n] = c as u32;
                n += 1;
                if came[c] == 0 {
                    break;
                }
                c = came[c] as usize - 1;
            }
        }
        blank(n_seen);
        if full {
            return end(ASK_HOST);
        }
        if !found {
            return end(NO_WAY);
        }
        cells[..n].reverse();
        run.n = n;
        run.smoothing = true;
    }
    let n = run.n;
    // (with a price: which raw cells the search paid extra for, read once for the whole way. Each was priced as the search reached it; one that
    //  was not is asked for like any other)
    if !P::FLAT {
        let mut wait = false;
        for &c in &cells[..n] {
            if src.at(c as usize).is_none() {
                wait = true;
            }
        }
        if wait {
            return NEED;
        }
    }
    // the greedy smoothing: from the anchor, the farthest cell of the way a man can walk straight to — and, with a price, to which the straight leg
    // crosses no more dear cells than the raw cells it stands in for
    let centre = |c: u32| -> (f64, f64) { (((c as usize % gw) as f64 + 0.5) * cell, ((c as usize / gw) as f64 + 0.5) * cell) };
    let (mut anchor0, mut anchor2) = (run.a0, run.a2);
    let mut out = 0usize;
    let mut i = 0usize;
    while i < n {
        let mut j = n - 1;
        while j > i {
            let (q0, q2) = centre(cells[j]);
            if grid.walkable(shut, anchor0, anchor2, q0, q2) {
                if P::FLAT {
                    break;
                }
                match dear_along(src, gw, gh, cell, anchor0, anchor2, q0, q2) {
                    Err(()) => return NEED,
                    Ok(d) => {
                        // (the raw cells this leg stands in for: priced above, every one)
                        let mut stood = 0u32;
                        for &c in &cells[i..=j] {
                            match src.at(c as usize) {
                                None => return end(ASK_HOST),
                                Some(p) => {
                                    if p > 1.0 {
                                        stood += 1;
                                    }
                                }
                            }
                        }
                        if d <= stood {
                            break;
                        }
                    }
                }
            }
            j -= 1;
        }
        let (p0, p2) = centre(cells[j]);
        // (cannot be on a floor `begin` took: a waypoint is a cell of the way, each once)
        if out >= MAX_WAY {
            return end(ASK_HOST);
        }
        way[out * 2] = p0;
        way[out * 2 + 1] = p2;
        out += 1;
        anchor0 = p0;
        anchor2 = p2;
        i = j + 1;
    }
    end(out as i32)
}
