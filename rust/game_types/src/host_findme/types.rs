//! The types of `host_findme`.
#![allow(unused_imports)]
use crate::{bytes::ToBytes, ids::*, js::*, Quat, V3};
use alloc::{string::String, vec::Vec};

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug, ToBytes)]
pub enum OutlineHue {
    You,
    Purple,
    Red,
    Blue,
}
impl OutlineHue {
    pub const fn as_str(self) -> &'static str {
        match self {
            OutlineHue::You => "you",
            OutlineHue::Purple => "purple",
            OutlineHue::Red => "red",
            OutlineHue::Blue => "blue",
        }
    }
}
