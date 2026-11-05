#![allow(unused_variables)]
//! Every function and method of `condition`, with an empty body. Written by the skeleton's program.
#[allow(unused_imports)]
use crate::{ids::*, js::*, Quat, V3};
#[allow(unused_imports)]
use alloc::{string::String, vec::Vec};

/// (src/game/condition.ts:56 isGuard)
pub fn is_guard(g: &mut crate::game::Game, m: crate::condition::Man) -> bool { todo!() }

/// (src/game/condition.ts:59 isHurt)
pub fn is_hurt(g: &mut crate::game::Game, m: crate::condition::Man, now: Option<f64>) -> bool { todo!() }

/// (src/game/condition.ts:66 conditionHurt)
pub fn condition_hurt(g: &mut crate::game::Game, m: &mut crate::condition::Man) -> bool { todo!() }

/// (src/game/condition.ts:74 conditionPace)
pub fn condition_pace(g: &mut crate::game::Game, gd: GuardId, speed: f64) -> f64 { todo!() }

/// (src/game/condition.ts:77 conditionSup)
pub fn condition_sup(g: &mut crate::game::Game, m: crate::condition::Man, v: f64) -> f64 { todo!() }

/// (src/game/condition.ts:80 conditionCalm)
pub fn condition_calm(g: &mut crate::game::Game, gd: GuardId) -> bool { todo!() }

/// (src/game/condition.ts:83 conditionOf)
pub fn condition_of(g: &mut crate::game::Game, m: crate::condition::Man) -> crate::condition::Condition { todo!() }

/// (src/game/condition.ts:95 conditionSummary)
pub fn condition_summary(g: &mut crate::game::Game) -> String { todo!() }

/// (src/game/condition.ts:105 conditionPose)
pub fn condition_pose(g: &mut crate::game::Game) { todo!() }
