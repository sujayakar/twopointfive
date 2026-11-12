//! The numbers of roomclear.
use crate::bytes::ToBytes;
use game_derive::Knobs;

/// The scenario's knobs, read at the next staging.
#[derive(Clone, Copy, Debug, ToBytes, Knobs)]
pub struct RoomclearK {
    /// which placement the defenders are dealt (placeDefenders): 0 = the authored one on the first staging. Changed while it is up, the next staging deals the new
    ///  seed's first placement.
    #[k(0.0)] pub seed: f64,
}
