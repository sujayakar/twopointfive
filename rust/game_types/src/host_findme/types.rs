//! The types of `host_findme`.
use crate::{Words, bytes::ToBytes, crosses::Crosses};

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug, ToBytes, Words, Crosses)]
pub enum OutlineHue {
    You,
    Purple,
    Red,
    Blue,
}
