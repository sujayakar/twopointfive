#![allow(unused_variables)]
//! Every function and method of `gunroles`, with an empty body. Written by the skeleton's program.
#[allow(unused_imports)]
use crate::{ids::*, js::*, Quat, V3};
#[allow(unused_imports)]
use alloc::{string::String, vec::Vec};

/// (src/game/gunroles.ts:72 useOn)
pub fn use_on(g: &mut crate::game::Game, key: crate::gunroles::UseOnKey) -> bool { todo!() }

/// (src/game/gunroles.ts:75 gunPrimary)
pub fn gun_primary(g: &mut crate::game::Game, gd: GuardId) -> crate::weapons::WeaponDef { todo!() }

/// (src/game/gunroles.ts:77 gunRole)
pub fn gun_role(g: &mut crate::game::Game, gd: GuardId) -> crate::gunroles::GunRole { todo!() }

/// (src/game/gunroles.ts:78 rankOf)
pub fn rank_of(g: &mut crate::game::Game, gd: GuardId) -> f64 { todo!() }

/// (src/game/gunroles.ts:84 lineOf)
pub fn line_of(g: &mut crate::game::Game, p: V3, to: V3) -> f64 { todo!() }

/// (src/game/gunroles.ts:90 gunHand)
pub fn gun_hand(g: &mut crate::game::Game, gd: GuardId, id_2: crate::weapons::WeaponId) { todo!() }

/// (src/game/gunroles.ts:100 gunDeal)
pub fn gun_deal(g: &mut crate::game::Game, men: &[crate::gunroles::DealMan]) -> Vec<crate::gunroles::GunDealItem> { todo!() }

/// (src/game/gunroles.ts:122 gunKeepsPost)
pub fn gun_keeps_post(g: &mut crate::game::Game, gd: GuardId) -> bool { todo!() }

/// (src/game/gunroles.ts:130 gunFileCost)
pub fn gun_file_cost(g: &mut crate::game::Game, r: &[f64], spots: &[V3], front: V3) -> f64 { todo!() }

/// (src/game/gunroles.ts:136 gunFileRanks)
pub fn gun_file_ranks(g: &mut crate::game::Game, men: &[GuardId]) -> Option<Vec<f64>> { todo!() }

/// (src/game/gunroles.ts:141 gunFirst)
pub fn gun_first(g: &mut crate::game::Game, men: &[GuardId]) -> Vec<GuardId> { todo!() }

/// (src/game/gunroles.ts:147 gunBefore)
pub fn gun_before(g: &mut crate::game::Game, first: GuardId, lead: GuardId) -> bool { todo!() }

/// (src/game/gunroles.ts:150 gunTurn)
pub fn gun_turn(g: &mut crate::game::Game, turn: &[GuardId], first: GuardId) -> Vec<GuardId> { todo!() }

/// (src/game/gunroles.ts:157 gunLast)
pub fn gun_last(g: &mut crate::game::Game, men: &[GuardId]) -> Vec<GuardId> { todo!() }

/// (src/game/gunroles.ts:164 gunBlindGap)
pub fn gun_blind_gap(g: &mut crate::game::Game, gd: GuardId, every: f64) -> f64 { todo!() }

/// (src/game/gunroles.ts:169 gunLane)
pub fn gun_lane(g: &mut crate::game::Game, gd: GuardId, d: f64, reach: f64) -> f64 { todo!() }

/// (src/game/gunroles.ts:176 gunString)
pub fn gun_string(g: &mut crate::game::Game, gd: GuardId) -> f64 { todo!() }

/// (src/game/gunroles.ts:182 gunStringGap)
pub fn gun_string_gap(g: &mut crate::game::Game, gd: GuardId) -> f64 { todo!() }

/// (src/game/gunroles.ts:183 gunStringGrow)
pub fn gun_string_grow(g: &mut crate::game::Game, gd: GuardId, k: f64) -> f64 { todo!() }
