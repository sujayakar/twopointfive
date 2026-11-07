//! The types of `host_findme`, written by the skeleton's program from the TypeScript. Never edited by hand: hand.cjs, and run it again.
#![allow(unused_imports)]
use crate::{ids::*, js::*, Quat, V3};
use alloc::{string::String, vec::Vec};

/// (TypeScript: words)
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
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
