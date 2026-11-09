//! The numbers of rearwatch.
use crate::bytes::ToBytes;
use game_derive::Knobs;

#[derive(Clone, Copy, Debug, ToBytes, Knobs)]
pub struct RearwatchK {
    /// how far from the stack's marks a way behind is looked for (cover.ts's own reach is the line's limit)
    #[k(9.0)] pub reach: f64,
    /// a leaf this far open is an open way
    #[k(0.6)] pub ajar: f64,
    /// a line lost for less than this keeps the watch his (the worked door's own leaf swinging across it, a man walking through it)
    #[k(1.5)] pub grace: f64,
    /// a rear man this long within a stride of his mark and still not on it is let off the watch until he is: walking with his gun behind him he can grind on a
    ///  leaf's edge where a man facing his walk slides past — and a stack waits for its last man (s)
    #[k(1.5)] pub short: f64,
    /// … the longest he is kept on it once Sam is out of the room (s): a mate who never leaves does not strand him
    #[k(8.0)] pub room_secs: f64,
    /// … and never further than this from Sam (m): a room big enough for Sam to walk that far inside it is not one he is kept at the far end of — 0 = no limit
    #[k(8.0)] pub room_m: f64,
}
