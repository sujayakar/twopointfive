//! The numbers of samstun.
use crate::bytes::ToBytes;
use game_derive::Knobs;

#[derive(Clone, Copy, Debug, ToBytes, Knobs)]
pub struct SamstunK {
    /// seconds the white takes to clear at the end of a dazzle
    #[k(0.6)] pub fade: f64,
    /// in a room fight, a stun canister of HIS OWN SIDE that he has a line to (his own throw, a teammate's bang watched through the doorway) blinds and folds him
    ///  by the full count, as it does a defender standing there; false = the own-side rule his teammates get (only inside FLASH.ownM, FLASH.ownSecs, no fold)
    #[k(true)] pub own_seen: bool,
}
