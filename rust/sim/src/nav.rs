//! A WAY ACROSS THE FLOOR — src/scene/collision.ts NavGrid.findPath with no penalty (A* over the half-metre grid, then the greedy smoothing), nearestFree and
//! walkable. The rule is the TypeScript's, statement for statement: the same cells popped in the same order, the same ties, the same waypoints.
//!
//! The grid of shut cells is the host's (it bakes it and re-bakes it under a blast); it writes it here before it asks. Everything in this file is scratch of
//! one question: the search's four tables stand blank between searches, as the TypeScript's do. The scores are KEPT in singles, as the TypeScript keeps them
//! in Float32Arrays (single.rs), and worked out in doubles.
//!
//! The lengths (the smoothing's, and the lines nearestFree asks) are the game's own, so this answers only where EXACT.on.

use crate::lines;
use crate::maths::hypot2;
use crate::single::{get, put, S};
use crate::world::{js_abs, js_min, or_one};
use prim::floor;

/// the cells of the largest grid searched here (a level's floor is its box grid's, four nav cells to the metre square)
pub const MAX_NAV: usize = 1 << 16;
/// the open list and the list of cells written: a search that would outgrow either is the host's
const MAX_OPEN: usize = 1 << 17;
/// the waypoints a way can have
pub const MAX_WAY: usize = 4096;

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
    // (each side is under MAX_NAV before the two are multiplied: the product cannot wrap)
    if gw == 0 || gh == 0 || gw > MAX_NAV || gh > MAX_NAV || gw * gh > MAX_NAV {
        return ASK_HOST;
    }
    let n_cells = gw * gh;
    // (a point that is no number has no cell: the TypeScript's own answer for it, whatever that is, is the host's to give)
    if !(a0 - a0 == 0.0 && a2 - a2 == 0.0 && b0 - b0 == 0.0 && b2 - b2 == 0.0) || !(cell > 0.0) {
        return ASK_HOST;
    }
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
    let (gxf, gzf) = (gx as f64, gz as f64);
    let (mut n_open, mut n_seen) = (0usize, 0usize);
    {
        let (x, z) = ((start % gw) as f64, (start / gw) as f64);
        let (dx, dz) = (js_abs(x - gxf), js_abs(z - gzf));
        put(g, start, 0.0);
        put(f, start, (dx + dz) + (SQRT2 - 2.0) * js_min(dx, dz));
    }
    open[n_open] = start as u32;
    n_open += 1;
    seen[n_seen] = start as u32;
    n_seen += 1;
    let mut iter: u32 = 0;
    let mut full = false;
    loop {
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
        let cur = open[mi] as usize;
        n_open -= 1;
        open[mi] = open[n_open];
        if cur == goal {
            break;
        }
        if closed[cur] != 0 {
            continue;
        }
        closed[cur] = 1;
        let (cx, cz) = ((cur % gw) as i32, (cur / gw) as i32);
        if n_open + 8 > MAX_OPEN || n_seen + 8 > MAX_OPEN {
            full = true;
            break;
        }
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
                let step = if dx != 0 && dz != 0 { SQRT2 } else { 1.0 };
                let ng = get(g, cur) + step;
                if ng < get(g, ni) {
                    let (hx, hz) = (js_abs(nx as f64 - gxf), js_abs(nz as f64 - gzf));
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
    for i in 0..n_seen {
        let c = seen[i] as usize;
        g[c] = S::INF;
        f[c] = S::INF;
        came[c] = 0;
        closed[c] = 0;
    }
    if full {
        return ASK_HOST;
    }
    if !found {
        return NO_WAY;
    }
    cells[..n].reverse();
    // the greedy smoothing: from the anchor, the farthest cell of the way a man can walk straight to
    let centre = |c: u32| -> (f64, f64) { (((c as usize % gw) as f64 + 0.5) * cell, ((c as usize / gw) as f64 + 0.5) * cell) };
    let (mut anchor0, mut anchor2) = (a0, a2);
    let mut out = 0usize;
    let mut i = 0usize;
    while i < n {
        let mut j = n - 1;
        while j > i {
            let (q0, q2) = centre(cells[j]);
            if grid.walkable(shut, anchor0, anchor2, q0, q2) {
                break;
            }
            j -= 1;
        }
        let (p0, p2) = centre(cells[j]);
        if out >= MAX_WAY {
            return ASK_HOST;
        }
        way[out * 2] = p0;
        way[out * 2 + 1] = p2;
        out += 1;
        anchor0 = p0;
        anchor2 = p2;
        i = j + 1;
    }
    out as i32
}
