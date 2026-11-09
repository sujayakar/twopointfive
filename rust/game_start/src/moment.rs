//! A MOMENT OF A GAME: the game as a value and its block of rust/sim, between two ticks. Stood again it is the game that went on from there. A host keeps one for a save or a walk of a
//! tape (rust/game_face), a test or the gate to go back to one; there is one way to put it back.
use alloc::boxed::Box;
use alloc::vec::Vec;
use game::game::Game;

pub struct Moment { pub game: Box<Game>, pub block: Vec<u8> }

impl Moment {
    /// the moment a game stands at
    pub fn of(g: &Game) -> Moment { Moment { game: Box::new(g.clone()), block: crate::block::bytes(g.sim).to_vec() } }
    /// the moment in `slot`, and a game that holds it. The box world's copy in rust/sim is not yet of this moment (`whole`): a host may have something to say of the game first.
    pub fn laid(&self, slot: usize) -> Box<Game> {
        sim::state::put(slot, &self.block);
        let mut g = self.game.clone();
        game::game::stand_in(&mut g, slot);
        // (the box world's copy in rust/sim stands outside the block and is the abandoned moment's: written whole again, by `whole`)
        g.col.mirror.put_backs = -1.0;
        g
    }
    /// the game stands at this moment again, in `slot`
    pub fn stands(&self, slot: usize) -> Box<Game> {
        let mut g = self.laid(slot);
        whole(&mut g);
        g
    }
}

/// the box world's copy with it: it stands again, of this moment, before anybody asks
pub fn whole(g: &mut Game) {
    game::scene_collision::unstand(g);
    game::scene_collision::stands(g);
}
