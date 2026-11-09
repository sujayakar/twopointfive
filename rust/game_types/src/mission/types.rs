//! The types of `mission`.
#![allow(unused_imports)]
use crate::{bytes::ToBytes, ids::*, js::*, Quat, V3};
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

/// THE MISSION THAT IS UP, AS IT SAYS OF ITSELF: whoever stages one writes it, whoever wants a fact of a mission reads it here, and no rule asks which mission it is.
#[derive(Clone, Debug, ToBytes, Default)]
pub struct Staged {
    /// its kind (None: the level's own night)
    pub up: Option<crate::scene_scenarios::MissionKind>,
    /// the teammates it staged, dead or alive
    pub team: Vec<GuardId>,
    /// the men it staged against them, dead or alive
    pub foes: Vec<GuardId>,
}
