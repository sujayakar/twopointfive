#![allow(unused_variables)]
//! Every function and method of `kitload`, with an empty body. Written by the skeleton's program.
#[allow(unused_imports)]
use crate::{ids::*, js::*, Quat, V3};
#[allow(unused_imports)]
use alloc::{string::String, vec::Vec};

/// (src/game/kitload.ts:81 gunsOf)
pub fn guns_of(g: &mut crate::game::Game, i: f64, m: &crate::kit::MemberKit) -> Vec<crate::weapons::WeaponId> { todo!() }

/// (src/game/kitload.ts:86 gunLoad)
pub fn gun_load(g: &mut crate::game::Game, w: crate::weapons::WeaponId) -> crate::kitload::KitLoad { todo!() }

/// (src/game/kitload.ts:87 add)
pub fn add(a: [f64; 2], b: Option<[f64; 2]>, n: Option<f64>) -> [f64; 2] { todo!() }

/// (src/game/kitload.ts:89 loadShare)
pub fn load_share(g: &mut crate::game::Game, i: f64, m: &crate::kit::MemberKit, field: &str) -> Option<crate::kitload::KitLoad> { todo!() }

/// (src/game/kitload.ts:102 loadOf)
pub fn load_of(g: &mut crate::game::Game, i: f64, m: &crate::kit::MemberKit) -> crate::kitload::KitLoad { todo!() }

/// (src/game/kitload.ts:110 loadRef)
pub fn load_ref(g: &mut crate::game::Game, i: f64, n: Option<f64>) -> crate::kitload::KitLoad { todo!() }

/// (src/game/kitload.ts:112 snap)
pub fn snap(x: f64, r: f64) -> f64 { todo!() }

/// (src/game/kitload.ts:114 loadWorn)
pub fn load_worn(g: &mut crate::game::Game, i: f64, m: &crate::kit::MemberKit, n: Option<f64>) -> Option<crate::kitload::KitWorn> { todo!() }

/// (src/game/kitload.ts:119 loadPaceK)
pub fn load_pace_k(g: &mut crate::game::Game, gd: GuardId) -> f64 { todo!() }

/// (src/game/kitload.ts:120 loadSeenK)
pub fn load_seen_k(g: &mut crate::game::Game, gd: GuardId) -> f64 { todo!() }

/// (src/game/kitload.ts:121 loadHeardK)
pub fn load_heard_k(g: &mut crate::game::Game, gd: GuardId) -> f64 { todo!() }

/// (src/game/kitload.ts:128 loadWear)
pub fn load_wear(g: &mut crate::game::Game, m: Option<&crate::kit::MemberKit>) { todo!() }

/// (src/game/kitload.ts:130 loadMan)
pub fn load_man(g: &mut crate::game::Game, gd: GuardId, i: f64, m: Option<&crate::kit::MemberKit>, n: Option<f64>) { todo!() }

/// (src/game/kitload.ts:132 loadOn)
pub fn load_on(g: &mut crate::game::Game, who: crate::kitload::LoadOnWho) -> Option<crate::kitload::KitWorn> { todo!() }

/// (src/game/kitload.ts:137 loadPaceOn)
pub fn load_pace_on(g: &mut crate::game::Game, who: crate::kitload::LoadPaceOnWho) -> f64 { todo!() }

/// (src/game/kitload.ts:139 loadSprintOn)
pub fn load_sprint_on(g: &mut crate::game::Game) -> f64 { todo!() }

/// (src/game/kitload.ts:141 samLoadPace)
pub fn sam_load_pace(g: &mut crate::game::Game, speed: f64, sprint: Option<bool>) -> f64 { todo!() }

/// (src/game/kitload.ts:144 loadPace)
pub fn load_pace(g: &mut crate::game::Game, gd: GuardId, speed: f64) -> f64 { todo!() }

/// (src/game/kitload.ts:151 loadGait)
pub fn load_gait(g: &mut crate::game::Game, who: crate::kitload::LoadGaitWho, speed: f64) -> f64 { todo!() }

/// (src/game/kitload.ts:153 loadSeen)
pub fn load_seen(g: &mut crate::game::Game, who: crate::kitload::LoadSeenWho) -> f64 { todo!() }

/// (src/game/kitload.ts:155 loadHeard)
pub fn load_heard(g: &mut crate::game::Game, who: crate::kitload::LoadHeardWho) -> f64 { todo!() }

/// (src/game/kitload.ts:157 loadCross)
pub fn load_cross(g: &mut crate::game::Game, who: crate::kitload::LoadCrossWho) -> f64 { todo!() }
