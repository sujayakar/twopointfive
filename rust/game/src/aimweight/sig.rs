#![allow(unused_variables)]
//! Every function and method of `aimweight`, with an empty body. Written by the skeleton's program.
#[allow(unused_imports)]
use crate::{ids::*, js::*, Quat, V3};
#[allow(unused_imports)]
use alloc::{string::String, vec::Vec};

/// (src/game/aimweight.ts:58 can)
pub fn can(g: &mut crate::game::Game) -> bool { todo!() }

/// (src/game/aimweight.ts:60 aimWeighs)
pub fn aim_weighs(g: &mut crate::game::Game) -> bool { todo!() }

/// (src/game/aimweight.ts:62 aimTop)
pub fn aim_top(g: &mut crate::game::Game, pl: PlayerId) -> f64 { todo!() }

/// (src/game/aimweight.ts:65 aimSwing)
pub fn aim_swing(g: &mut crate::game::Game, w: &mut crate::aimweight::AimSwingW, err: f64, top: f64, dt: f64) -> f64 { todo!() }

/// (src/game/aimweight.ts:82 aimWeighBegin)
pub fn aim_weigh_begin(g: &mut crate::game::Game, pl: PlayerId) { todo!() }

/// (src/game/aimweight.ts:85 aimWeighTurn)
pub fn aim_weigh_turn(g: &mut crate::game::Game, dt: f64) -> bool { todo!() }

/// (src/game/aimweight.ts:106 aimGunPoint)
pub fn aim_gun_point(g: &mut crate::game::Game) -> Option<V3> { todo!() }

/// (src/game/aimweight.ts:113 aimGunOn)
pub fn aim_gun_on(g: &mut crate::game::Game) -> bool { todo!() }

/// (src/game/aimweight.ts:118 aimWeighShot)
pub fn aim_weigh_shot(g: &mut crate::game::Game, tgt: V3) -> V3 { todo!() }

/// (src/game/aimweight.ts:129 aimWeighLaunch)
pub fn aim_weigh_launch(g: &mut crate::game::Game, dt: f64, up: bool) -> bool { todo!() }

/// (src/game/aimweight.ts:139 aimWeighLaunchKeep)
pub fn aim_weigh_launch_keep(g: &mut crate::game::Game, pl: PlayerId, dt: f64) -> bool { todo!() }

/// (src/game/aimweight.ts:153 aimWeightSummary)
pub fn aim_weight_summary(g: &mut crate::game::Game) -> String { todo!() }
