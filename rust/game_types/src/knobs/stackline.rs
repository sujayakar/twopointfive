//! The numbers of stackline.
use crate::bytes::ToBytes;
use game_derive::Knobs;

#[derive(Clone, Copy, Debug, ToBytes, Knobs)]
pub struct StacklineK {
    /// a leaf at this angle or more (rad), unlatched, STANDS OPEN (rearwatch.ts's own figure); one off its hinges always does
    #[k(0.6)] pub ajar: f64,
    /// strips the opening is cut into · the height a line is asked at (m) · how far short of the frame line a sight line to a strip ends (so the frame's own
    ///  jambs are not in it)
    #[k(4.0)] pub bins: f64,
    #[k(1.2)] pub eye: f64,
    #[k(0.2)] pub inset: f64,
    /// within this of his mark he is on it: the order's own last settle, never paced (m)
    #[k(0.35)] pub on_mark: f64,
}
