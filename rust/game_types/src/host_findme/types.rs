//! The types of `host_findme`.
#![allow(unused_imports)]
use crate::{Words, bytes::ToBytes, crosses::Crosses, ids::*, js::*, Quat, V3};
use alloc::{string::String, vec::Vec};

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug, ToBytes, Words, Crosses)]
pub enum OutlineHue {
    You,
    Purple,
    Red,
    Blue,
}
