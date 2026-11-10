//! A WAY ACROSS THE FLOOR — src/scene/collision.ts NavGrid.findPath (A* over the half-metre grid, then the greedy smoothing), nearestFree and
//! walkable; with a penalty too, whose prices are asked as the search comes to need them (`start_priced`, `priced`). The rule is the TypeScript's, statement for statement: the same cells popped in the same order, the same ties, the same waypoints.
//!
//! The grid of shut cells is the host's (it bakes it and re-bakes it under a blast); it writes it here before it asks. Everything in this file is scratch of
//! one question: the search's four tables stand blank between searches, as the TypeScript's do. The scores are KEPT in singles, as the TypeScript keeps them
//! in Float32Arrays (single.rs), and worked out in doubles.
//!
//! The lengths (the smoothing's, and the lines nearestFree asks) are the game's own, so this answers only where EXACT.on.

use game_types::knobs::Knobs;
use crate::lines;
use crate::maths::hypot2;
use crate::single::{get, put, S};
use crate::world::{js_abs, js_max, js_min, or_one, World};
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
/// once by the host, and never a level's.
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
    /// a place along an axis in cells (a cell whose size is a power of two is divided by as it is multiplied by its inverse: the same double)
    #[inline(always)]
    fn cells(&self, v: f64) -> f64 {
        if self.cell == 0.5 { v * 2.0 } else { v / self.cell }
    }
    /// NavGrid.isBlocked: off the grid is shut
    #[inline(always)]
    fn is_blocked(&self, shut: &[u8], wx: f64, wz: f64) -> bool {
        let (x, z) = (self.cells(wx), self.cells(wz));
        // (a floor over nought is what the cast leaves off)
        !(x >= 0.0 && z >= 0.0 && x < self.gw as f64 && z < self.gh as f64) || shut[z as usize * self.gw + x as usize] == 1
    }
    /// NavGrid.walkable. `at`: how far along the last line asked was shut, where this one is asked first (and left so): whether a line is free hangs on no order
    fn walkable(&self, shut: &[u8], a0: f64, a2: f64, b0: f64, b2: f64, at: &mut f64) -> bool {
        let (dx, dz) = (b0 - a0, b2 - a2);
        let len = hypot2(dx, dz);
        let n = ceil(len / (self.cell * 0.5)) + 1.0;
        let (px, pz) = (-dz / or_one(len) * 0.22, dx / or_one(len) * 0.22);
        let blocked = |i: f64| {
            let t = i / n;
            let (x, z) = (a0 + dx * t, a2 + dz * t);
            self.is_blocked(shut, x, z) || self.is_blocked(shut, x + px, z + pz) || self.is_blocked(shut, x - px, z - pz)
        };
        let first = floor(*at / len * n);
        if first >= 0.0 && first <= n && blocked(first) {
            return false;
        }
        let mut i = 0.0;
        while i <= n {
            if blocked(i) {
                *at = i / n * len;
                return false;
            }
            i += 1.0;
        }
        true
    }
    /// NavGrid.nearestFree for a world point that asked: Ok(the cell, or none), Err = a line the host must judge
    fn nearest_free(&self, shut: &[u8], w: &World, gx: usize, gz: usize, wx: f64, wz: f64, tune: &Knobs) -> Result<Option<(usize, usize)>, ()> {
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
                    match lines::solid_line(w, true, lines::ASK_GLASS, wx, 1.0, wz, (x as f64 + 0.5) * self.cell, 1.0, (z as f64 + 0.5) * self.cell, tune) {
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
pub fn find_path(w: &World, gw: usize, gh: usize, cell: f64, a0: f64, a2: f64, b0: f64, b2: f64, max_iter: u32, tune: &Knobs) -> i32 {
    match begin(w, gw, gh, cell, a0, a2, b0, b2, max_iter, tune) {
        Ok(()) => go(&mut Flat),
        Err(said) => said,
    }
}
/// findPath(a, b, max_iter, penalty): the search is set to its start, or what it says without one (Some). The way is `priced`'s to find.
#[allow(clippy::too_many_arguments)]
pub fn start_priced(w: &World, gw: usize, gh: usize, cell: f64, a0: f64, a2: f64, b0: f64, b2: f64, max_iter: u32, tune: &Knobs) -> Option<i32> {
    begin(w, gw, gh, cell, a0, a2, b0, b2, max_iter, tune).err()
}
/// …and found by the price of a step into a cell (its index in the grid), asked once a cell as the search comes to it. It asks for no other way.
pub fn priced(price: &mut dyn FnMut(usize) -> f64) -> i32 {
    go(&mut Priced { price, n: 0 })
}

/// WHAT A STEP INTO A CELL COSTS, TO ONE SEARCH (collision.ts findPath's `penalty`: a multiplier, read as 1 where it is under 1). THE SEARCH IS
/// WRITTEN ONCE, OVER THIS: a search with no penalty is the search whose every price is 1 (`Flat`).
trait Prices {
    /// every price is 1: no cell is dear, so the smoothing has none to count along a leg
    const FLAT: bool;
    /// the price of cell `c`
    fn at(&mut self, c: usize) -> f64;
    /// the search is over
    fn over(&mut self) {}
}
struct Flat;
impl Prices for Flat {
    const FLAT: bool = true;
    #[inline(always)]
    fn at(&mut self, _c: usize) -> f64 {
        1.0
    }
}
/// a caller's prices, each asked once
struct Priced<'a> {
    price: &'a mut dyn FnMut(usize) -> f64,
    n: usize,
}
impl Prices for Priced<'_> {
    const FLAT: bool = false;
    #[inline(always)]
    fn at(&mut self, c: usize) -> f64 {
        let (price, known) = unsafe { (&mut *core::ptr::addr_of_mut!(PRICE), &mut *core::ptr::addr_of_mut!(KNOWN)) };
        if known[c] == 0 {
            price[c] = (self.price)(c);
            known[c] = 1;
            unsafe { (*core::ptr::addr_of_mut!(PRICED))[self.n] = c as u32 };
            self.n += 1;
        }
        price[c]
    }
    fn over(&mut self) {
        let (known, priced) = unsafe { (&mut *core::ptr::addr_of_mut!(KNOWN), &*core::ptr::addr_of!(PRICED)) };
        for &c in &priced[..self.n] {
            known[c as usize & (MAX_NAV - 1)] = 0;
        }
    }
}
/// THE PRICES OF ONE SEARCH, a double a cell; which cells have one; and the list of those, so that the table is blank again as the search ends
static mut PRICE: [f64; MAX_NAV] = [0.0; MAX_NAV];
static mut KNOWN: [u8; MAX_NAV] = [0; MAX_NAV];
static mut PRICED: [u32; MAX_NAV] = [0; MAX_NAV];
/// A SEARCH SET TO ITS START
#[derive(Clone, Copy)]
struct Run {
    gw: usize,
    gh: usize,
    cell: f64,
    a0: f64,
    a2: f64,
    start: usize,
    goal: usize,
    gx: usize,
    gz: usize,
    max_iter: u32,
}
static mut RUN: Run = Run { gw: 0, gh: 0, cell: 0.0, a0: 0.0, a2: 0.0, start: 0, goal: 0, gx: 0, gz: 0, max_iter: 0 };
/// whether a search has the tables (one asked from inside a price has none)
static mut LIVE: bool = false;

#[allow(clippy::too_many_arguments)]
fn begin(w: &World, gw: usize, gh: usize, cell: f64, a0: f64, a2: f64, b0: f64, b2: f64, max_iter: u32, tune: &Knobs) -> Result<(), i32> {
    let run = unsafe { &mut *core::ptr::addr_of_mut!(RUN) };
    // (a search asked while another waits for its prices would write over that one's tables)
    if unsafe { LIVE } {
        return Err(ASK_HOST);
    }
    // (a floor of more cells than a search holds: the host has said so by name before it asks, NavGrid.there. Each side is under MAX_FLOOR before the
    //  two are multiplied: the product cannot wrap)
    if gw == 0 || gh == 0 || gw > MAX_FLOOR || gh > MAX_FLOOR || gw * gh > MAX_FLOOR {
        return Err(ASK_HOST);
    }
    // (a point that is no number has no cell: the TypeScript's own answer for it, whatever that is, is the host's to give)
    if !(a0 - a0 == 0.0 && a2 - a2 == 0.0 && b0 - b0 == 0.0 && b2 - b2 == 0.0) || !(cell > 0.0) {
        return Err(ASK_HOST);
    }
    let (shut, g, way) = unsafe { (&*core::ptr::addr_of!(SHUT), &mut *core::ptr::addr_of_mut!(G), &mut *core::ptr::addr_of_mut!(WAY)) };
    unsafe {
        if !BLANK {
            for i in 0..MAX_NAV {
                g[i] = S::INF;
            }
            BLANK = true;
        }
    }
    let grid = Grid { gw, gh, cell };
    let (sx, sz) = (cell_of(a0, cell, gw), cell_of(a2, cell, gh));
    let (mut gx, mut gz) = (cell_of(b0, cell, gw), cell_of(b2, cell, gh));
    if shut[gz * gw + gx] != 0 {
        match grid.nearest_free(shut, w, gx, gz, b0, b2, tune) {
            Err(()) => return Err(ASK_HOST),
            Ok(None) => return Err(NO_WAY),
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
        return Err(1);
    }
    unsafe { LIVE = true };
    *run = Run { gw, gh, cell, a0, a2, start, goal, gx, gz, max_iter };
    Ok(())
}
/// the search's tables as they stood before it: every cell it wrote is in SEEN
fn blank(n_seen: usize) {
    let (g, came, closed, seen) = unsafe { (&mut *core::ptr::addr_of_mut!(G), &mut *core::ptr::addr_of_mut!(CAME), &mut *core::ptr::addr_of_mut!(CLOSED), &*core::ptr::addr_of!(SEEN)) };
    for &c in &seen[..n_seen] {
        let c = c as usize & (MAX_NAV - 1);
        g[c] = S::INF;
        came[c] = 0;
        closed[c] = 0;
    }
}
/// NavGrid.dearAlong: the distinct cells along the straight leg a → b (sampled at half a cell) whose price is above plain floor
fn dear_along<P: Prices>(src: &mut P, grid: &Grid, a0: f64, a2: f64, b0: f64, b2: f64) -> u32 {
    let (gw, gh) = (grid.gw as f64, grid.gh as f64);
    let (dx, dz) = (b0 - a0, b2 - a2);
    let n = ceil(hypot2(dx, dz) / (grid.cell * 0.5)) + 1.0;
    let (mut last, mut count) = (-1.0f64, 0u32);
    let mut k = 0.0;
    while k <= n {
        let t = k / n;
        k += 1.0;
        let (ix, iz) = (floor(grid.cells(a0 + dx * t)), floor(grid.cells(a2 + dz * t)));
        let i = iz * gw + ix;
        if i == last {
            continue;
        }
        last = i;
        if ix >= 0.0 && iz >= 0.0 && ix < gw && iz < gh && src.at(iz as usize * grid.gw + ix as usize) > 1.0 {
            count += 1;
        }
    }
    count
}
/// the search's tables
struct Tables {
    shut: &'static [u8; MAX_NAV],
    g: &'static mut [S; MAX_NAV],
    came: &'static mut [u32; MAX_NAV],
    closed: &'static mut [u8; MAX_NAV],
    seen: &'static mut [u32; MAX_OPEN],
    cells: &'static mut [u32; MAX_NAV],
    way: &'static mut [f64; MAX_WAY * 2],
}
fn tables() -> Tables {
    unsafe {
        Tables {
            shut: &*core::ptr::addr_of!(SHUT),
            g: &mut *core::ptr::addr_of_mut!(G),
            came: &mut *core::ptr::addr_of_mut!(CAME),
            closed: &mut *core::ptr::addr_of_mut!(CLOSED),
            seen: &mut *core::ptr::addr_of_mut!(SEEN),
            cells: &mut *core::ptr::addr_of_mut!(CELLS),
            way: &mut *core::ptr::addr_of_mut!(WAY),
        }
    }
}

/// THE OPEN CELLS, in the order they were opened (one taken off leaves its place to the last), each with its score: a cell opened again stands twice, and both
/// have its newest. The least is found by the least of every BLOCK of places.
struct Open {
    n: usize,
    cell: &'static mut [u32; MAX_OPEN],
    score: &'static mut [S; MAX_OPEN],
    least: &'static mut [u32; MAX_OPEN / BLOCK + 1],
}
const BLOCK: usize = 16;
static mut SCORE: [S; MAX_OPEN] = [S::ZERO; MAX_OPEN];
static mut LEAST: [u32; MAX_OPEN / BLOCK + 1] = [0; MAX_OPEN / BLOCK + 1];
impl Open {
    fn empty() -> Open {
        unsafe { Open { n: 0, cell: &mut *core::ptr::addr_of_mut!(OPEN), score: &mut *core::ptr::addr_of_mut!(SCORE), least: &mut *core::ptr::addr_of_mut!(LEAST) } }
    }
    /// one more, at the end
    #[inline(always)]
    fn push(&mut self, c: usize, f: f64) {
        let (n, b) = (self.n, self.n / BLOCK);
        self.cell[n] = c as u32;
        put(self.score, n, f);
        if n % BLOCK == 0 || get(self.score, n) < get(self.score, self.least[b] as usize) {
            self.least[b] = n as u32;
        }
        self.n += 1;
    }
    /// a cell that stands here already has another score (a better one, but for what the single it is kept as rounds off)
    fn better(&mut self, c: usize, f: f64) {
        for i in 0..self.n {
            if self.cell[i] as usize == c {
                let (l, was) = (self.least[i / BLOCK] as usize, get(self.score, i));
                put(self.score, i, f);
                let (v, least) = (get(self.score, i), get(self.score, l));
                if l == i {
                    if v > was {
                        self.look(i / BLOCK);
                    }
                } else if v < least || (v == least && i < l) {
                    self.least[i / BLOCK] = i as u32;
                }
            }
        }
    }
    /// the cell of the least score, the first of equals, off the list
    #[inline(always)]
    fn pop(&mut self) -> usize {
        let mut mi = self.least[0] as usize;
        let mut mf = get(self.score, mi);
        for &l in &self.least[1..self.n.div_ceil(BLOCK)] {
            let f = get(self.score, l as usize);
            if f < mf {
                (mi, mf) = (l as usize, f);
            }
        }
        let c = self.cell[mi] as usize;
        self.n -= 1;
        self.cell[mi] = self.cell[self.n];
        self.score[mi] = self.score[self.n];
        self.look(mi / BLOCK);
        if self.n / BLOCK != mi / BLOCK {
            self.look(self.n / BLOCK);
        }
        c
    }
    /// a block's least, found again
    #[inline(always)]
    fn look(&mut self, b: usize) {
        let (from, to) = (b * BLOCK, if (b + 1) * BLOCK < self.n { (b + 1) * BLOCK } else { self.n });
        if from >= to {
            return;
        }
        let (mut mi, mut mf) = (from, get(self.score, from));
        for (k, s) in self.score[from..to].iter().enumerate().skip(1) {
            let f = get(core::slice::from_ref(s), 0);
            if f < mf {
                (mi, mf) = (from + k, f);
            }
        }
        self.least[b] = mi as u32;
    }
}

/// THE SEARCH, from its start to its answer
fn go<P: Prices>(src: &mut P) -> i32 {
    let run = unsafe { *core::ptr::addr_of!(RUN) };
    let mut t = tables();
    let said = match search(src, &run, &mut t) {
        Ok(n) => smooth(src, &run, &mut t, n),
        Err(said) => said,
    };
    src.over();
    unsafe { LIVE = false };
    said
}

/// the least way from a cell to the goal: straight and on the diagonal
#[inline(always)]
fn left(run: &Run, x: usize, z: usize) -> f64 {
    let (dx, dz) = (js_abs(x as f64 - run.gx as f64), js_abs(z as f64 - run.gz as f64));
    (dx + dz) + (SQRT2 - 2.0) * js_min(dx, dz)
}
/// the search itself: how many cells its way has (they stand in `cells`), or what it says instead
fn search<P: Prices>(src: &mut P, run: &Run, t: &mut Tables) -> Result<usize, i32> {
    let gw = run.gw;
    let mut open = Open::empty();
    put(t.g, run.start, 0.0);
    open.push(run.start, left(run, run.start % gw, run.start / gw));
    t.seen[0] = run.start as u32;
    let (mut n_seen, mut iter, mut full) = (1usize, 0u32, false);
    while open.n > 0 {
        let it = iter;
        iter = it.wrapping_add(1);
        if !(it < run.max_iter) {
            break;
        }
        let cur = open.pop();
        if cur == run.goal {
            break;
        }
        if t.closed[cur] != 0 {
            continue;
        }
        t.closed[cur] = 1;
        // (cannot be on a floor `begin` took: MAX_FLOOR)
        if open.n + 8 > MAX_OPEN || n_seen + 8 > MAX_OPEN {
            full = true;
            break;
        }
        let (near, n_near) = steps_from(run, t, cur);
        // (every one of them is priced before any is stepped into)
        if !P::FLAT {
            for s in &near[..n_near] {
                src.at(s.cell);
            }
        }
        let from = get(t.g, cur);
        for s in &near[..n_near] {
            let ng = from + (if s.diagonal { SQRT2 } else { 1.0 }) * js_max(1.0, src.at(s.cell));
            let was = get(t.g, s.cell);
            if ng < was {
                let f = ng + left(run, s.x as usize, s.z as usize);
                put(t.g, s.cell, ng);
                t.came[s.cell] = cur as u32 + 1;
                if was < f64::INFINITY {
                    open.better(s.cell, f);
                }
                open.push(s.cell, f);
                t.seen[n_seen] = s.cell as u32;
                n_seen += 1;
            }
        }
    }
    let way = if full { Err(ASK_HOST) } else { way_back(run, t) };
    blank(n_seen);
    way
}

/// a step into a cell: which, where it is, and whether it is on the diagonal
#[derive(Clone, Copy, Default)]
struct Step {
    cell: usize,
    x: u32,
    z: u32,
    diagonal: bool,
}
/// The cells a step from a cell may enter, in the search's own order.
#[inline(always)]
fn steps_from(run: &Run, t: &Tables, cur: usize) -> ([Step; 8], usize) {
    let (gw, gh, shut) = (run.gw, run.gh, t.shut);
    let mut near = [Step::default(); 8];
    let mut n = 0usize;
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
            if shut[ni] != 0 || t.closed[ni] != 0 {
                continue;
            }
            if dx != 0 && dz != 0 && (shut[cz as usize * gw + nx as usize] != 0 || shut[nz as usize * gw + cx as usize] != 0) {
                continue;
            }
            near[n] = Step { cell: ni, x: nx as u32, z: nz as u32, diagonal: dx != 0 && dz != 0 };
            n += 1;
        }
    }
    (near, n)
}

/// the way the search found, its cells from the first step to the goal: how many
fn way_back(run: &Run, t: &mut Tables) -> Result<usize, i32> {
    if t.came[run.goal] == 0 {
        return Err(NO_WAY);
    }
    let (mut n, mut c) = (0usize, run.goal);
    while c != run.start {
        t.cells[n] = c as u32;
        n += 1;
        if t.came[c] == 0 {
            break;
        }
        c = t.came[c] as usize - 1;
    }
    t.cells[..n].reverse();
    Ok(n)
}

/// THE GREEDY SMOOTHING: from the anchor, the farthest cell of the way a man can walk straight to, and, with a price, to which the straight leg crosses
/// no more dear cells than the raw cells it stands in for (how many of those there are up to each is counted once for the whole way).
fn smooth<P: Prices>(src: &mut P, run: &Run, t: &mut Tables, n: usize) -> i32 {
    let grid = Grid { gw: run.gw, gh: run.gh, cell: run.cell };
    // (`seen` is free again: the search is over)
    if !P::FLAT {
        t.seen[0] = 0;
        for k in 0..n {
            t.seen[k + 1] = t.seen[k] + (src.at(t.cells[k] as usize) > 1.0) as u32;
        }
    }
    let centre = |c: u32| -> (f64, f64) { (((c as usize % run.gw) as f64 + 0.5) * run.cell, ((c as usize / run.gw) as f64 + 0.5) * run.cell) };
    let (mut anchor0, mut anchor2) = (run.a0, run.a2);
    let (mut out, mut i) = (0usize, 0usize);
    while i < n {
        let mut j = n - 1;
        let mut shut_at = 0.0;
        while j > i {
            let (q0, q2) = centre(t.cells[j]);
            if grid.walkable(t.shut, anchor0, anchor2, q0, q2, &mut shut_at) && (P::FLAT || dear_along(src, &grid, anchor0, anchor2, q0, q2) <= t.seen[j + 1] - t.seen[i]) {
                break;
            }
            j -= 1;
        }
        let (p0, p2) = centre(t.cells[j]);
        // (cannot be on a floor `begin` took: a waypoint is a cell of the way, each once)
        if out >= MAX_WAY {
            return ASK_HOST;
        }
        t.way[out * 2] = p0;
        t.way[out * 2 + 1] = p2;
        out += 1;
        anchor0 = p0;
        anchor2 = p2;
        i = j + 1;
    }
    out as i32
}
