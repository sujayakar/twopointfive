//! The numbers of look.
use crate::bytes::ToBytes;
use game_derive::Knobs;

#[derive(Clone, Copy, Debug, ToBytes, Knobs)]
pub struct LookK {
    #[k(true)] pub on: bool,
}
