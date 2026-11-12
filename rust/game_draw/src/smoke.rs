//! THE GAME'S OWN SMOKE AS THE PICTURE IS TOLD IT: what the picture's solver is bent toward, a little at a time (src/smoke/smoke.wgsl, THE GAME'S SMOKE). The grid is
//! the game's (`g.smoke`): half-metre cells in bricks, stepped ten times a second, the same on every machine and in every snapshot. The rows are made again
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
    of: u64,
    made: u32,
    said: bool,
}

impl Smoke {
    pub fn cut(&mut self) { (self.of, self.said) = (0, false); }
    /// nothing was said yet of the game as it was put back: its ways are to be asked before a take (game::sense::smoke_ways)
    pub fn unsaid(&self) -> bool { !self.said }
    pub fn take(&mut self, g: &Game) {
        self.said = true;
        let s = &g.smoke;
        let drawn = sim::bricks::drawn(&g.made.ways, |of, faces| {
            if of == self.of { return; }
            self.of = of;
            (self.head.clear(), self.at.clear(), self.bricks.clear());
            self.made += 1;
            self.head.push(SmokeHead { made: self.made, wide: s.wide, deep: s.deep });
            self.at.resize((s.wide * s.deep) as usize, Word { w: 0 });
            for (b, (rho, open)) in s.bricks.iter().zip(s.rho.chunks_exact(CELLS).zip(faces.chunks_exact(CELLS))) {
                let Some((x, z)) = b.at else { continue };
                self.bricks.push(SmokeBrick { rho: core::array::from_fn(|c| rho[c] as f32), open: sim::bricks::open_words(open), dye: dye(b.dye) });
                self.at[(z as u32 * s.wide + x as u32) as usize].w = self.bricks.len() as u32;
            }
        });
        if drawn.is_none() {
            self.of = 0;
            (self.head.clear(), self.at.clear(), self.bricks.clear());
        }
    }
}
