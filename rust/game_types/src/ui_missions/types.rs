//! The types of `ui_missions`.
#![allow(unused_imports)]
use crate::{ids::*, js::*, Quat, V3};
use alloc::{string::String, vec::Vec};

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub enum Lean {
    Quiet,
    Loud,
    Either,
}
impl Lean {
    pub const fn as_str(self) -> &'static str {
        match self {
            Lean::Quiet => "quiet",
            Lean::Loud => "loud",
            Lean::Either => "either",
        }
    }
}
