//! The numbers of pairup.
use crate::bytes::ToBytes;
use game_derive::Knobs;

#[derive(Clone, Copy, Debug, ToBytes, Knobs)]
pub struct PairupK {
    /// THE switch: the three parts on, and the rules below
    #[k(false)] pub on: bool,
}
