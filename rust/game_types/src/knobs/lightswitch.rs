//! The numbers of lightswitch.
use crate::bytes::ToBytes;
use alloc::vec::Vec;
use game_derive::Knobs;

#[derive(Clone, Debug, ToBytes, Knobs)]
pub struct LightswitchK {
    /// m from his feet to the plate, on the plan, that a hand reaches it from
    #[k(1.15)] pub reach: f64,
    /// the plate: m above the floor · m past the jamb along the wall (tried in this order, the latch side first) · m off the wall a man stands to work it
    #[k(1.2)] pub height: f64,
    #[k(alloc::vec![0.85, 1.1, 0.6, 1.5, 2.0, 2.6])] pub beside: Vec<f64>,
    #[k(0.5)] pub stand_m: f64,
    /// the most plates one circuit of a room is given (the first found, then the furthest from those kept) and the least they are apart, m
    #[k(2.0)] pub per_room: f64,
    #[k(4.0)] pub apart_m: f64,
    /// where the door's own wall gives no place for one (glass, a corner): m about the door the room's other walls are tried within
    #[k(4.0)] pub scan_m: f64,
    #[k(1.5)] pub guard_secs: f64,
    /// s his hand is at the plate after the tap
    #[k(0.3)] pub hand_secs: f64,
}
