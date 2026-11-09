//! The types of `host_findme`.
#![allow(unused_imports)]
use crate::{bytes::ToBytes, Words, ids::*, js::*, Quat, V3};
use alloc::{string::String, vec::Vec};

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug, ToBytes, Words)]
pub enum OutlineHue {
    You,
    Purple,
    Red,
    Blue,
}
