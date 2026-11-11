//! THE WEATHER'S DUST IN THE ROOMS, as the picture has it: the game's own field (game_rules air_dust `air_grid`), four half-metre cells to a row. Sent when it is another than
//! what was sent last: nothing while no weather stands in the air.
use super::Row4;
use alloc::vec::Vec;
use game::game::Game;

#[derive(Default)]
pub struct Air {
    pub rows: Vec<Row4>,
    pub moved: bool,
    sent: Vec<Row4>,
}

impl Air {
    /// the game is another: what was sent of the last one's field may not be this one's
    pub fn cut(&mut self) { self.sent.clear(); }
    pub fn frame(&mut self, g: &Game) {
        let cells = game::air_dust::air_grid(g);
        self.rows.clear();
        if cells.is_empty() {
            // (none stands: what was sent of the last weather's is not left there for the next)
            self.moved = !self.sent.is_empty();
            if self.moved { self.rows.resize(self.sent.len(), Row4 { w: [1.0; 4] }); self.sent.clear(); }
            return;
        }
        self.moved = false;
        self.rows.extend(cells.chunks(4).map(|c| Row4 { w: core::array::from_fn(|i| c.get(i).copied().unwrap_or(1.0) as f32) }));
        if self.rows == self.sent { return; }
        self.moved = true;
        self.sent.clone_from(&self.rows);
    }
}
