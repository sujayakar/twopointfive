//! THE PANES: every static of glass, nearest to where the view looks first and as many as the info block holds, and behind each what is broken of it (the cells
//! a round or a blast took out).
use super::grid::{MAX_PANES, PANES};
use super::{pack, Row4, Word};
use alloc::vec::Vec;
use game::game::Game;
use game::m;

/// rows a pane: four of its own and four a site
const HEAD: usize = 4;
const SITE: usize = 4;
const SITES: usize = 8;
const ROWS: usize = HEAD + SITES * SITE;
// places in a pane's row (rust/sim panes.rs Row.d and Row.w)
const D_SITE: usize = 9;
const D_DICE: usize = 72;
const W_FLAGS: usize = 1;
const W_COLS: usize = 2;
const W_ROWS: usize = 3;
const W_DICE_SEED: usize = 4;
const W_N: usize = 5;
const W_SEED: usize = 13;
const W_OPEN: usize = 21;
const GONE: u32 = 1;
const DICE: u32 = 2;
const RUN: u32 = 4;
const BANDS: usize = 7;
const SECTORS: usize = 12;

#[derive(Default)]
pub struct Glass {
    /// each listed pane's rows, in the list's order
    pub cells: Vec<Row4>,
    pub moved: bool,
    live: bool,
    sent: Vec<Row4>,
    panes: Vec<u32>,
    near: Vec<(f64, u32)>,
}

impl Glass {
    /// the game is another: what was sent of the last one's panes may not be this one's
    pub fn cut(&mut self) { self.live = true; self.sent.clear(); }
    pub fn take(&mut self, g: &Game, n: usize) {
        self.panes.clear();
        for (i, &b) in g.engine.world.statics[..n].iter().enumerate() { if pack::flags(&g.tab.boxes[b]) & pack::GLASS != 0 { self.panes.push(i as u32); } }
    }
    pub fn frame(&mut self, g: &Game, focus: [f64; 3], info: &mut [Word], on: bool) {
        let st = &g.engine.world.statics;
        self.near.clear();
        if on { for &i in &self.panes { let c = g.tab.boxes[st[i as usize]].c; self.near.push(((c[0] - focus[0]) * (c[0] - focus[0]) + (c[2] - focus[2]) * (c[2] - focus[2]), i)); } }
        self.near.sort_by(|a, b| a.0.total_cmp(&b.0));
        self.near.truncate(MAX_PANES);
        let f = |v: f64| Word { w: (v as f32).to_bits() };
        info[PANES].w = self.near.len() as u32;
        info[PANES + 1].w = self.near.len() as u32;
        for (k, &(_, i)) in self.near.iter().enumerate() {
            let b = &g.tab.boxes[st[i as usize]];
            let o = PANES + 4 + k * 12;
            for j in 0..3 { info[o + j] = f(b.c[j]); info[o + 4 + j] = f(b.h[j]); info[o + 8 + j] = f(b.albedo[j]); }
            info[o + 3] = f(m::cos(b.yaw));
            info[o + 7] = f(m::sin(b.yaw));
            info[o + 11] = f(i as f64);
        }
        self.cells.clear();
        self.cells.resize(MAX_PANES * ROWS, Row4 { w: [0.0; 4] });
        let mut any = false;
        for k in 0..self.near.len() { let i = self.near[k].1; any |= broken(g, i, &mut self.cells[k * ROWS..(k + 1) * ROWS]); }
        // (sent while any pane has something to say, and once more as the last is whole; and only where it is another table than was sent)
        self.moved = (any || self.live) && self.cells != self.sent;
        if self.moved { self.sent.clone_from(&self.cells); }
        self.live = any;
    }
}

/// what is broken of static `bi`, into its rows; false = it is whole
fn broken(g: &Game, bi: u32, out: &mut [Row4]) -> bool {
    let p = sim::panes::pn(g.sim);
    let Some(at) = p.row_of(bi) else { return false };
    let r = sim::panes::row_ref(g.sim, at);
    let flags = r.w[W_FLAGS];
    let (sites, gone, dice) = ((flags >> 8) as usize, flags & GONE != 0, flags & DICE != 0);
    if sites == 0 && !dice && !gone { return false; }
    let open = gone || r.n_open > 0 || (dice && r.d_next > 0);
    let says = open as u32 | if flags & RUN != 0 { 2 } else { 0 } | if dice && !gone { 4 } else { 0 } | if gone { 8 } else { 0 } | if r.ty == 2 { 16 } else { 0 };
    out[0].w = [sites as f32, says as f32, (r.u_axis != 0) as u32 as f32, r.damage as f32];
    out[1].w = [r.hu as f32, r.hv as f32, r.ty as f32, 0.0];
    if dice {
        out[2].w = [r.d[D_DICE] as f32, r.d[D_DICE + 1] as f32, sim::panes::dice_threshold(g.sim, bi) as f32, g.k.glassbreak.dice_grain as f32];
        out[3].w = [r.w[W_COLS] as f32, r.w[W_ROWS] as f32, r.w[W_DICE_SEED] as f32, 0.0];
    }
    for i in 0..sites.min(SITES) {
        let (q, n) = (i * D_SITE, r.w[W_N + i] as usize);
        let mut masks = [0u32; BANDS];
        for b in 0..BANDS { for k in 0..n { let id = i * BANDS * SECTORS + b * SECTORS + k; if r.w[W_OPEN + (id >> 5)] >> (id & 31) & 1 != 0 { masks[b] |= 1 << k; } } }
        let o = HEAD + i * SITE;
        out[o].w = [r.d[q] as f32, r.d[q + 1] as f32, r.d[q + 2] as f32, r.d[q + 7] as f32];
        out[o + 1].w = [n as f32, r.w[W_SEED + i] as f32, m::min(8.0, r.e[i]) as f32, r.d[q + 8] as f32];
        out[o + 2].w = [r.d[q + 3] as f32, r.d[q + 4] as f32, r.d[q + 5] as f32, r.d[q + 6] as f32];
        out[o + 3].w = [(masks[0] | masks[1] << 12) as f32, (masks[2] | masks[3] << 12) as f32, (masks[4] | masks[5] << 12) as f32, masks[6] as f32];
    }
    true
}
