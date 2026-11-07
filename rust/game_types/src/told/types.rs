//! The types of `told`, written by the skeleton's program from the TypeScript. Never edited by hand: hand.cjs, and run it again.
#![allow(unused_imports)]
use crate::{ids::*, js::*, Quat, V3};
use alloc::{string::String, vec::Vec};

/// (TypeScript: words)
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub enum ToldRung {
    Bare,
    Guided,
    Standard,
}
impl ToldRung {
    pub const fn as_str(self) -> &'static str {
        match self {
            ToldRung::Bare => "bare",
            ToldRung::Guided => "guided",
            ToldRung::Standard => "standard",
        }
    }
}

/// (src/game/told.ts:31 TOLD_RUNGS: a table nothing writes)
pub fn told_rungs() -> Vec<String> { alloc::vec![String::from("guided"), String::from("standard"), String::from("bare")] }
