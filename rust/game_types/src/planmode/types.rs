//! The types of `planmode`.
use crate::ids::*;
use alloc::vec::Vec;

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

pub const GOAL_NONE: &str = "nobody's free for that.";
