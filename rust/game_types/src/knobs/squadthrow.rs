//! The numbers of squadthrow.
use crate::bytes::ToBytes;
use alloc::vec::Vec;
use game_derive::Knobs;

#[derive(Clone, Debug, ToBytes, Knobs)]
pub struct SquadthrowK {
    #[k(true)] pub on: bool,
    /// how far a frag runs on past its landing: this × its pace over the floor × the speed it comes down at (squadsmoke.ts squadSmokeRest; a frag is a smaller,
    ///  deader body than a can — throwables.ts FRAG_ITEM; measured headless on the office's floor, throws of 3 m to 12 m: where the fuse found it was a mean
    ///  0.11 m and at most 0.30 m from the spot at 0.04, 0.15 / 0.33 at 0.045, 0.28 / 0.69 at 0.05)
    #[k(0.04)] pub frag_roll_k: f64,
    /// one man's next throw no sooner than this after his last (s)
    #[k(1.2)] pub again_secs: f64,
    /// the squad block names the throw this long (s)
    #[k(2.5)] pub shown_secs: f64,
    /// the stun row's warning asks detonateStun's reaches this much long (m): where it goes off is where it has run to when its fuse ends
    #[k(1.0)] pub slack_m: f64,
    /// … no further than this over the walk grid (m) · never from nearer the spot than this (m) · nor from further than a hand throws (m)
    #[k(12.0)] pub walk_m: f64,
    #[k(1.5)] pub min_m: f64,
    #[k(11.5)] pub max_m: f64,
    /// cells whose arc is asked in one search at most (each up to five arcs, ~0.2 ms an arc) · a search's answer is kept this long for the menu's and the picture's re-reads (s; an order GIVEN asks afresh)
    #[k(12.0)] pub tries: f64,
    /// he gives the walk up after this long on it (s; time the fight has him is not counted) · on his spot he turns to the throw for this long at most (s) · the toss: he stays this long after it leaves his hand (s)
    #[k(16.0)] pub walk_secs: f64,
    #[k(1.2)] pub turn_secs: f64,
    #[k(0.5)] pub toss_secs: f64,
    #[k(alloc::vec![7.5, 10.0, 5.0])] pub lob_paces: Vec<f64>,
    #[k(alloc::vec![10.0, 7.5])] pub lob_bank_paces: Vec<f64>,
    #[k(11.0)] pub lob_speed: f64,
    #[k(1.5)] pub lob_near_m: f64,
    #[k(1.2)] pub lob_up_m: f64,
    #[k(12.0)] pub lob_walls: f64,
    #[k(3.0)] pub lob_wall_m: f64,
    #[k(0.9)] pub lob_wall_h: f64,
    #[k(48.0)] pub lob_sims: f64,
    #[k(10.0)] pub lob_cells: f64,
    #[k(0.5)] pub settle_secs: f64,
    #[k(alloc::vec![0.008333333333333333, 0.025])] pub lob_steps: Vec<f64>,
    #[k(0.5)] pub duck_secs: f64,
    #[k(0.2)] pub duck_throw_secs: f64,
    #[k(16.0)] pub duck_cells: f64,
    #[k(1.0)] pub duck_apart_m: f64,
    #[k(5.0)] pub duck_min_m: f64,
}
