//! The numbers of shapes.
use crate::bytes::ToBytes;
use game_derive::Knobs;

#[derive(Clone, Copy, Debug, ToBytes, Knobs)]
pub struct ShapesK {
    #[k(true)] pub on: bool,
}
