//! The numbers of inhand.
use crate::bytes::ToBytes;
use game_derive::Knobs;

#[derive(Clone, Copy, Debug, ToBytes, Knobs)]
pub struct InhandK {
    #[k(0.3)] pub from: f64,
    #[k(0.92)] pub to: f64,
    #[k(55.0)] pub wrist: f64,
    #[k(0.34)] pub both: f64,
    #[k(9.0)] pub rate: f64,
    #[k(0.03)] pub holster_out: f64,
}
