//! The types of `planmode`.
#![allow(unused_imports)]
use crate::{ids::*, js::*, Quat, V3};
use alloc::{string::String, vec::Vec};

#[derive(Clone, Debug, Default)]
pub struct GoalParts {
    pub idle: Vec<GuardId>,
    pub groups: Vec<Vec<GuardId>>,
}

#[derive(Clone, Debug, Default)]
pub struct GoalMen {
    pub men: Vec<GuardId>,
    pub how: crate::planmode::GoalMenHow,
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug, Default)]
pub enum GoalMenHow {
    #[default] All,
    Idle,
    His,
    Same,
    Split,
}

/// the counters this module keeps for the checks
#[derive(Clone, Debug, Default)]
pub struct PlanmodeStats {
    pub goals: i32,
    pub split: i32,
    pub rings: i32,
    pub none: i32,
}

/// (src/game/planmode.ts:23 GOAL_NONE)
pub const GOAL_NONE: &str = "nobody's free for that.";
