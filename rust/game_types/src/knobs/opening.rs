//! The numbers of opening.
use crate::bytes::ToBytes;
use game_derive::Knobs;

#[derive(Clone, Copy, Debug, ToBytes, Knobs)]
pub struct OpeningK {
    /// metres by walking from the player's spawn inside which no man of the shift starts the night
    #[k(10.0)] pub clear_m: f64,
    /// how finely his route is walked looking for the first point that far off
    #[k(0.5)] pub step_m: f64,
}
