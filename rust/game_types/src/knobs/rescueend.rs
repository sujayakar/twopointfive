//! The numbers of rescueend.
use crate::bytes::ToBytes;
use game_derive::Knobs;

#[derive(Clone, Copy, Debug, ToBytes, Knobs)]
pub struct RescueendK {
    /// a hostage is out inside this many metres of the way out
    #[k(3.5)] pub out_m: f64,
    /// the beat between the last man of theirs going down and the run's end
    #[k(1.5)] pub clear_secs: f64,
    /// the row on a hostage: its marker shows inside `markerM`, the word can be given inside `reachM`
    #[k(2.4)] pub marker_m: f64,
    #[k(2.2)] pub reach_m: f64,
    /// a led hostage stops this far behind whoever leads her, the next one this much further back
    #[k(1.3)] pub gap_m: f64,
    #[k(0.7)] pub gap_step_m: f64,
}
