//! The types of `mission`.
#![allow(unused_imports)]
use crate::{ids::*, js::*, Quat, V3};
use alloc::{string::String, vec::Vec};

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub enum TallyK {
    Keys,
    Spotted,
    Shots,
    Kills,
    Bodies,
    Alerts,
    Knockouts,
}

/// (src/game/mission.ts:62 DOWN_CARD_SECS)
pub const DOWN_CARD_SECS: f64 = 2.6;
