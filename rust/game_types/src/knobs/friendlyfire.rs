//! The numbers of friendlyfire.
use crate::bytes::ToBytes;
use game_derive::Knobs;

#[derive(Clone, Copy, Debug, ToBytes, Knobs)]
pub struct FriendlyfireK {
    /// the share of a round's `spread` its line can be thrown sideways by (fireWeapon adds ±spread/2 to each axis of a unit direction: 0.71 at most on the floor plan)
    #[k(0.75)] pub cone_k: f64,
    /// half the breadth a friend's body is owed about that cone, m
    #[k(0.34)] pub body_m: f64,
    /// the round cannot miss the man it is for while its cone is no wider than this at his distance, m (his column is 0.28): a friend BEYOND him is then not in its way
    #[k(0.2)] pub sure_m: f64,
    #[k(0.6)] pub near_m: f64,
}
