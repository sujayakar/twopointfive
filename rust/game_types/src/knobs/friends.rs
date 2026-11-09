//! The numbers of friends.
use crate::bytes::ToBytes;
use game_derive::Knobs;

#[derive(Clone, Copy, Debug, ToBytes, Knobs)]
pub struct FriendsK {
    #[k(8.0)] pub reach_m: f64,
    #[k(0.3)] pub past_m: f64,
    #[k(0.35)] pub clear_m: f64,
    #[k(0.3)] pub lead_s: f64,
    #[k(0.4)] pub lead_max_m: f64,
    #[k(0.25)] pub up_secs: f64,
    #[k(5.0)] pub quiet_s: f64,
}
