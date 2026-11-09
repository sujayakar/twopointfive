//! The numbers of quietfog.
use crate::bytes::ToBytes;
use game_derive::Knobs;

#[derive(Clone, Copy, Debug, ToBytes, Knobs)]
pub struct QuietfogK {
    #[k(crate::quietfog::QuietRung::Team)] pub def: crate::quietfog::QuietRung,
}
