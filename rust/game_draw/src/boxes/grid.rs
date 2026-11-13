//! THE GRID EVERY RAY OF THE PICTURE WALKS (common.wgsl; src/scene/boxes.ts buildGrid says why each number is as it is): which boxes stand in each metre of
//! the floor, how high, and how far the nearest of them is from a metre that is empty. A frame whose boxes register
//! exactly as the last frame's did keeps its grid.
use super::pack::{GLASS, NO_SHADOW};
use super::{Info, Word};
use alloc::vec::Vec;
use game::m;
use crate::Row;
use game::rows::{half, half_value};
use game::scene_world::world;

const W: usize = 40;
const H: usize = 28;
const N: usize = W * H;
/// where the packed cells start: after the plain ones
const CB: usize = N + 1;
const GLOBALS: usize = 16;
/// a global's bounds stand this far outside it, and the height span of a cell is judged this much tighter than it is packed
const GLOBAL_PAD: f64 = 0.005;
const ITEM_PAD_KEEP: f64 = 0.005;
pub const CAPS: usize = 15;
pub const MAX_PANES: usize = 32;

/// where a box stands in the grid: its cells, its height's levels and the slices of it
#[derive(Clone, Copy, PartialEq, Default)]
struct At { x0: i32, x1: i32, z0: i32, z1: i32, low: u32, high: u32, slices: u32 }
const OUT: i32 = -1;
const GLOBAL: i32 = -2;

#[derive(Default)]
pub struct Grid {
    pub cells: Vec<Word>,
    pub items: Vec<Word>,
    /// the scene's block: one row
    pub info: Vec<Info>,
    /// items of the plain list (as many again are packed)
    pub count: usize,
    pub slack: f64,
    statics: Vec<At>,
    static_slack: f64,
    static_globals: Vec<u32>,
    static_moved: bool,
    own: Vec<At>,
    globals: Vec<u32>,
}

/// a box as it is drawn this frame
#[derive(Clone, Copy)]
pub struct Stood { pub c: [f64; 3], pub h: [f64; 3], pub yaw: f64, pub rot: Option<[f64; 4]>, pub flags: u32 }
impl Stood {
    /// its half extents in the world, along x, y and z
    fn extents(&self) -> [f64; 3] {
        let h = self.h;
        let Some(q) = self.rot else {
            let (c, s) = (m::abs(m::cos(self.yaw)), m::abs(m::sin(self.yaw)));
            return [c * h[0] + s * h[2], h[1], s * h[0] + c * h[2]];
        };
        let ax = [[1.0, 0.0, 0.0], [0.0, 1.0, 0.0], [0.0, 0.0, 1.0]].map(|v| game::v3::quat::rotate(q, v));
        core::array::from_fn(|k| m::abs(ax[0][k]) * h[0] + m::abs(ax[1][k]) * h[1] + m::abs(ax[2][k]) * h[2])
    }
}

impl Grid {
    pub fn statics_moved(&mut self) { self.static_moved = true; }

    fn level(y: f64) -> u32 {
        let k = world();
        let q = m::floor((y - k.grid_y0) * (k.grid_y_levels / (k.grid_y1 - k.grid_y0)));
        m::min(k.grid_y_levels - 1.0, m::max(0.0, q)) as u32
    }
    /// where one box registers, and how far what the GPU traces of it reaches past that
    fn register(b: &Stood, may_be_global: bool) -> (At, f64) {
        let none = At { x0: OUT, ..Default::default() };
        if b.flags & (NO_SHADOW | GLASS) != 0 { return (none, 0.0); }
        let (e, c, k) = (b.extents(), b.c, world());
        let mut slack = 0.0;
        if b.rot.is_none() {
            // (the halves the GPU turns it by are not quite a turn: its traced outline is that much wider)
            let (cs, sn) = (m::abs(half_value(half(m::cos(b.yaw)))), m::abs(half_value(half(m::sin(b.yaw)))));
            if cs != 0.0 && sn != 0.0 {
                let (det, h0, h2) = (cs * cs + sn * sn, b.h[0], b.h[2]);
                slack = m::max(0.0, m::max((cs * h0 + sn * h2) / det - e[0], (sn * h0 + cs * h2) / det - e[2]));
            }
        }
        let cell = |v: f64| m::floor(v / k.grid_cell) as i32;
        let (x0, x1) = (cell(c[0] - e[0]).max(0), cell(c[0] + e[0]).min(W as i32 - 1));
        let (z0, z1) = (cell(c[2] - e[2]).max(0), cell(c[2] + e[2]).min(H as i32 - 1));
        if x1 < x0 || z1 < z0 { return (none, slack); }
        if (x1 - x0 + 1) * (z1 - z0 + 1) > 96 && may_be_global { return (At { x0: GLOBAL, ..Default::default() }, slack); }
        let (low, high) = (Self::level(c[1] - e[1] - k.grid_y_pad), Self::level(c[1] + e[1] + k.grid_y_pad));
        let (s0, s1) = (low / 10, high / 10);
        (At { x0, x1, z0, z1, low, high, slices: (0xff_ffff >> (23 - (s1 - s0))) << s0 }, slack)
    }

    /// the grid of these boxes, the first `n` of them statics; true = it is another than it was
    pub fn build(&mut self, all: &[Stood], n: usize) -> bool {
        let mut same = !self.static_moved && self.statics.len() == n && !self.cells.is_empty();
        if !same {
            self.statics.clear();
            self.static_globals.clear();
            self.static_slack = 0.0;
            for i in 0..n {
                let (mut at, s) = Self::register(&all[i], self.static_globals.len() < GLOBALS);
                self.static_slack = m::max(self.static_slack, s);
                if at.x0 == GLOBAL { self.static_globals.push(i as u32); at.x0 = OUT; }
                self.statics.push(at);
            }
            self.static_moved = false;
        }
        let (mut slack, mut globals) = (self.static_slack, self.static_globals.clone());
        let own = all.len() - n;
        if self.own.len() != own { same = false; self.own.resize(own, At::default()); }
        for k in 0..own {
            let (mut at, s) = Self::register(&all[n + k], globals.len() < GLOBALS);
            slack = m::max(slack, s);
            if at.x0 == GLOBAL { globals.push((n + k) as u32); at.x0 = OUT; }
            if at.x0 == OUT { at = At { x0: OUT, ..Default::default() }; }
            if self.own[k] != at { self.own[k] = at; same = false; }
        }
        if globals != self.globals { same = false; self.globals = globals; }
        self.slack = slack + world().grid_jitter;
        if !same { self.lay(); }
        self.say(all);
        !same
    }

    fn lay(&mut self) {
        let mut counts = [0u32; N + 1];
        let mut mask = [0u32; N];
        let each = |at: &At, f: &mut dyn FnMut(usize)| if at.x0 >= 0 { for z in at.z0..=at.z1 { for x in at.x0..=at.x1 { f(z as usize * W + x as usize); } } };
        for at in self.statics.iter().chain(self.own.iter()) { each(at, &mut |c| { counts[c] += 1; mask[c] |= at.slices; }); }
        let mut sum = 0;
        for c in 0..N { let k = counts[c]; counts[c] = sum; sum += k; }
        counts[N] = sum;
        let total = sum as usize;
        self.count = total;
        self.items.clear();
        self.items.resize(2 * total, Word { w: 0 });
        let mut filled = [0u32; N];
        for (i, at) in self.statics.iter().chain(self.own.iter()).enumerate() {
            let packed = i as u32 | (at.low << 16) | (at.high << 24);
            each(at, &mut |c| { let k = (counts[c] + filled[c]) as usize; filled[c] += 1; self.items[k].w = i as u32; self.items[total + k].w = packed; });
        }
        // how far each empty cell is from the nearest that is not, by king's moves
        let mut dist = [0u32; N];
        for c in 0..N { dist[c] = if counts[c + 1] > counts[c] { 0 } else { 255 }; }
        let near = |dist: &mut [u32; N], c: usize, x: usize, z: usize, dx: i32, dz: i32| {
            let (px, pz) = (x as i32 + dx, z as i32 + dz);
            if px >= 0 && pz >= 0 && px < W as i32 && pz < H as i32 { let d = dist[pz as usize * W + px as usize] + 1; if d < dist[c] { dist[c] = d; } }
        };
        for z in 0..H { for x in 0..W { let c = z * W + x; for (dx, dz) in [(-1, 0), (0, -1), (-1, -1), (1, -1)] { near(&mut dist, c, x, z, dx, dz); } } }
        for z in (0..H).rev() { for x in (0..W).rev() { let c = z * W + x; for (dx, dz) in [(1, 0), (0, 1), (1, 1), (-1, 1)] { near(&mut dist, c, x, z, dx, dz); } } }
        self.cells.resize(CB + 2 * N, Word { w: 0 });
        for c in 0..=N { self.cells[c].w = counts[c]; }
        for c in 0..N {
            self.cells[CB + 2 * c].w = mask[c] | (dist[c] << 24);
            self.cells[CB + 2 * c + 1].w = counts[c] | ((counts[c + 1] - counts[c]).min(255) << 24);
        }
    }

    /// the scene's block: the header, the globals and their bounds
    fn say(&mut self, all: &[Stood]) {
        if self.info.is_empty() { self.info.push(Info::ZERO); }
        let (k, info) = (world(), &mut self.info[0]);
        (info.num_boxes, info.num_globals, info.grid_w, info.grid_h) = (all.len() as u32, self.globals.len() as u32, W as u32, H as u32);
        (info.grid[0], info.grid[1]) = (self.slack as f32, (k.grid_y_pad - ITEM_PAD_KEEP) as f32);
        let pad = GLOBAL_PAD + self.slack;
        for i in 0..GLOBALS {
            let (at, y, lo, hi) = (&mut info.globals[i / 4][i % 4], i / 2, 2 * i, 2 * i + 1);
            let pair = 2 * (i % 2);
            let Some(&gi) = self.globals.get(i) else {
                *at = 0;
                info.globals_y[y][pair..pair + 2].fill(0.0);
                (info.globals_b[lo], info.globals_b[hi]) = ([0.0; 4], [0.0; 4]);
                continue;
            };
            let (c, e) = (all[gi as usize].c, all[gi as usize].extents());
            *at = gi;
            info.globals_y[y][pair] = (c[1] - e[1] - k.grid_y_pad) as f32;
            info.globals_y[y][pair + 1] = (c[1] + e[1] + k.grid_y_pad) as f32;
            for j in 0..3 { info.globals_b[lo][j] = (c[j] - e[j] - pad) as f32; info.globals_b[hi][j] = (c[j] + e[j] + pad) as f32; }
            (info.globals_b[lo][3], info.globals_b[hi][3]) = (0.0, 0.0);
        }
        info.caps_per_char = CAPS as u32;
        info.items_b = self.count as u32;
    }
}
