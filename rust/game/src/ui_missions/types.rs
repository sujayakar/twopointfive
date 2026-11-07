//! The types of `ui_missions`, written by the skeleton's program from the TypeScript. Never edited by hand: hand.cjs, and run it again.
#![allow(unused_imports)]
use crate::{ids::*, js::*, Quat, V3};
use alloc::{string::String, vec::Vec};

/// (TypeScript: words)
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
