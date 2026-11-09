//! The numbers of roomclear.
use crate::bytes::ToBytes;
use game_derive::Knobs;

/// The scenario's knobs (Tab ▸ Sandbox ▸ Room clear, __roomclear.knobs; read at the next staging).
#[derive(Clone, Copy, Debug, ToBytes, Knobs)]
pub struct RoomclearK {
    /// which placement the defenders are dealt (placeDefenders): 0 = the authored one on the first staging. Changed while it is up, the next staging deals the new
    ///  seed's first placement.
    #[k(0.0)] pub seed: f64,
    /// THE WAY IN: which of the level's team starts the team is stood on — 0, as shipped, and 1 = the structure's own `team`, k = its k-th (`starts[k − 2]`; past
    ///  the last = the last): the way in is the PLAYER'S PICK (the mission sheet's 'Way in' row puts `way=N` on the link — ui/missions.ts), never a surprise.
    ///  −1 = DEALT (`?way=any`, the row's 'Dealt'): on a FRESH visit (the switch above on, a start that named no seed, a Game with a die of its own) one of them is
    ///  taken off that die's seed, else the structure's own. Decided when the scenario STARTS (a restart keeps the way in and deals other marks). A level with no
    ///  `starts` has one way in whatever this says. (Scripted tallies, 24 seeds a level: a second way in costs the scripted commander 16‥21 teammates of 96 — it is
    ///  another problem, not the same one from another side, which is why no visit is handed one unasked.)
    #[k(0.0)] pub way: f64,
}
