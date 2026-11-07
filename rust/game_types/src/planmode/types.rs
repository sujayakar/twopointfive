//! The types of `planmode`, written by the skeleton's program from the TypeScript. Never edited by hand: hand.cjs, and run it again.
#![allow(unused_imports)]
use crate::{ids::*, js::*, Quat, V3};
use alloc::{string::String, vec::Vec};

/// (TypeScript: inline at src/game/planmode.ts:26 goalParts ->, src/game/planmode.ts:26)
#[derive(Clone, Debug)]
pub struct GoalParts {
    pub idle: Vec<GuardId>,
    pub groups: Vec<Vec<GuardId>>,
}
impl GoalParts {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        GoalParts {
            idle: Vec::new(),
            groups: Vec::new(),
        }
    }
}
impl Default for GoalParts { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/planmode.ts:40 goalMen ->, src/game/planmode.ts:40)
#[derive(Clone, Debug)]
pub struct GoalMen {
    pub men: Vec<GuardId>,
    pub how: crate::planmode::GoalMenHow,
}
impl GoalMen {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        GoalMen {
            men: Vec::new(),
            how: crate::planmode::GoalMenHow::All,
        }
    }
}
impl Default for GoalMen { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: words)
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub enum GoalMenHow {
    All,
    Idle,
    His,
    Same,
    Split,
}
impl GoalMenHow {
    pub const fn as_str(self) -> &'static str {
        match self {
            GoalMenHow::All => "all",
            GoalMenHow::Idle => "idle",
            GoalMenHow::His => "his",
            GoalMenHow::Same => "same",
            GoalMenHow::Split => "split",
        }
    }
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
