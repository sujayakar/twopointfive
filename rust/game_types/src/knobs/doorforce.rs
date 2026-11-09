//! The numbers of doorforce.
use crate::bytes::ToBytes;
use game_derive::Knobs;

#[derive(Clone, Copy, Debug, ToBytes, Knobs)]
pub struct DoorforceK {
    /// the two halves, each off = that half as it was: rounds through the leaf · the boot
    /// how sure he must be to start on a door · down to what he stays on it · s his sureness takes to fade to a third · s his newest word of it may be old
    #[k(1.0)] pub sure: f64,
    #[k(0.3)] pub keep: f64,
    #[k(5.0)] pub fade_secs: f64,
    #[k(6.0)] pub stale_secs: f64,
    /// what a sound adds: `gain` ×, nothing under `hearMin` of that sum · a report
    ///  at the gunshot line or over · at least this for a hand on THIS door · the sighting of a man who is now behind it
    #[k(4.0)] pub gain: f64,
    #[k(0.03)] pub hear_min: f64,
    #[k(1.0)] pub shot: f64,
    #[k(0.6)] pub handle: f64,
    #[k(1.5)] pub seen: f64,
    /// a sound whose way to him carries more than this did not come through a shut leaf
    #[k(0.5)] pub shut_carry: f64,
    /// m from the leaf's middle to where the enemy stood, on its far face · m from the leaf to the listener, on his · s a sighting stays worth a door
    #[k(2.5)] pub behind_m: f64,
    #[k(9.0)] pub reach_m: f64,
    #[k(3.0)] pub seen_secs: f64,
    /// m a heard place is put off the sound (hashed off the event and the man) · the height above the floor his rounds are laid at
    #[k(0.4)] pub heard_err_m: f64,
    #[k(1.15)] pub chest_y: f64,
    /// m from the leaf inside which a man is AT the door: with the wall in the straight line, the rounds go through the stretch of leaf nearest him, to a
    ///  point this far behind it
    #[k(1.2)] pub near_leaf_m: f64,
    #[k(0.6)] pub past_leaf_m: f64,
    /// rounds on one word · s before the next ones (which want a newer word)
    #[k(4.0)] pub rounds: f64,
    #[k(1.5)] pub gap_secs: f64,
    /// s on the door before a free man goes to boot it: with rounds for it · with none (steel, a leaf his round does not cross, no line)
    #[k(4.0)] pub kick_secs: f64,
    #[k(2.0)] pub kick_secs_no_shot: f64,
    /// his walk to the boot's spot is given up after this many seconds · s a door he has dealt with (or given up) is left alone
    #[k(6.0)] pub walk_secs: f64,
    #[k(8.0)] pub rest_secs: f64,
}
