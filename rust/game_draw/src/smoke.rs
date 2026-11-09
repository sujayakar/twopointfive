//! THE GAME'S OWN SMOKE AS THE PICTURE IS TOLD IT: what the picture's solver is bent toward, a little at a time (src/smoke/smoke.wgsl, THE GAME'S SMOKE). The grid is
//! the block's (sim::bricks): half-metre cells in bricks, stepped ten times a second, the same on every machine and in every snapshot. The rows are made again
//! when a step was taken, and lie as the GPU reads them. Of a game put back no way from cell to cell is known till its next step: they are asked before the first
//! of it is drawn (`unsaid`), so a picture has the whole of it at once.
use crate::boxes::Word;
use alloc::vec::Vec;
use game::game::Game;
use sim::bricks::{CELLS, WORDS};

section! {
    /// the grid's plan: one row, on a level that has one and knows its ways
    pub struct SmokeHead {
        /// how many times the rows were made
        pub made: u32,
        /// the plan in bricks, along x and along z
        pub wide: u32,
        pub deep: u32,
    }
}
section! {
    /// a brick that stands: four half-metre cells along x, then along z, then eight up
    pub struct SmokeBrick {
        /// how much smoke is in each cell, in the solver's own unit as it is drawn
        pub rho: [f32; 128],
        /// a byte a cell: 1 = open toward +x · 2 = up · 4 = toward +z · 8 = asked
        pub open: [u32; 32],
        /// what is missing from its color: a byte of nothing, then 1 − tint as r, g, b (smoke.wgsl WHAT IS IN A CELL)
        pub dye: u32,
    }
}
const _: () = assert!(CELLS == 128 && WORDS == 32);

/// a color nobody sees is none
fn dye(d: [f64; 3]) -> u32 {
    let w = d.map(|v| ((v * 255.0 + 0.5) as u32).min(255));
    if w.iter().all(|v| *v < 8) { 0 } else { w[0] << 8 | w[1] << 16 | w[2] << 24 }
}

#[derive(Default)]
pub struct Smoke {
    pub head: Vec<SmokeHead>,
    /// a word a place of the plan: the brick's row and one (0 = none stands there)
    pub at: Vec<Word>,
    pub bricks: Vec<SmokeBrick>,
    of: Option<(f64, f64)>,
    made: u32,
    said: bool,
}

impl Smoke {
    pub fn cut(&mut self) { (self.of, self.said) = (None, false); }
    /// nothing was said yet of the game as it was put back: its ways are to be asked before a take (game::sense::smoke_ways)
    pub fn unsaid(&self) -> bool { !self.said }
    pub fn take(&mut self, g: &Game) {
        self.said = true;
        let stamp = sim::state::stamp(g.sim);
        let stands = sim::bricks::stands(g.sim, stamp);
        let of = stands.as_ref().map(|s| (stamp, s.steps));
        if of.is_some() && self.of == of { return; }
        self.of = of;
        (self.head.clear(), self.at.clear(), self.bricks.clear());
        let Some(s) = stands else { return };
        self.made += 1;
        self.head.push(SmokeHead { made: self.made, wide: s.wide as u32, deep: s.deep as u32 });
        self.at.resize(s.wide * s.deep, Word { w: 0 });
        for b in 0..sim::bricks::MAX_BRICKS {
            let Some(b) = s.brick(b) else { continue };
            self.bricks.push(SmokeBrick { rho: core::array::from_fn(|c| b.rho[c] as f32), open: b.open, dye: dye(b.dye) });
            self.at[b.z * s.wide + b.x].w = self.bricks.len() as u32;
        }
    }
}
