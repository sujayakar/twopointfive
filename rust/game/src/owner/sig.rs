#![allow(unused_variables)]
//! Every function and method of `owner`, with an empty body. Written by the skeleton's program.
#[allow(unused_imports)]
use crate::{ids::*, js::*, Quat, V3};
#[allow(unused_imports)]
use alloc::{string::String, vec::Vec};

/// (src/game/owner.ts:42 bit)
pub fn bit(i: f64) -> f64 { todo!() }

/// (src/game/owner.ts:49 rankOf)
pub fn rank_of(word: f64) -> f64 { todo!() }

/// (src/game/owner.ts:51 rowOf)
pub fn row_of(rank: f64) -> crate::owner::RowOf { todo!() }

/// (src/game/owner.ts:57 settle)
pub fn settle(g: &mut crate::game::Game, gd: GuardId, word: f64, t: f64, why: f64) { todo!() }

/// (src/game/owner.ts:64 claimMan)
pub fn claim_man(g: &mut crate::game::Game, gd: GuardId, rule: f64, t: f64, why: Option<f64>) { todo!() }

/// (src/game/owner.ts:71 releaseMan)
pub fn release_man(g: &mut crate::game::Game, gd: GuardId, rule: f64, t: f64) { todo!() }

/// (src/game/owner.ts:76 hasHim)
pub fn has_him(g: &mut crate::game::Game, gd: GuardId, rules: f64) -> bool { todo!() }

/// (src/game/owner.ts:81 ownerOf)
pub fn owner_of(g: &mut crate::game::Game, gd: GuardId) -> Option<crate::owner::OwnerOf> { todo!() }

/// (src/game/owner.ts:92 ownerWords)
pub fn owner_words(g: &mut crate::game::Game, gd: GuardId, t: f64) -> String { todo!() }

/// (src/game/owner.ts:101 ownerSummary)
pub fn owner_summary(g: &mut crate::game::Game, men: &[GuardId], t: f64) -> String { todo!() }

/// (src/game/owner.ts:110 ownerWitness)
pub fn owner_witness<G: Clone + core::fmt::Debug>(g: &mut crate::game::Game, name: crate::owner::OwnerRule, says: &mut dyn FnMut(&mut crate::game::Game, GuardId, G) -> bool, at_read: Option<bool>) { todo!() }

/// (src/game/owner.ts:112 ownerDisagrees)
pub fn owner_disagrees<G: Clone + core::fmt::Debug>(g: &mut crate::game::Game, gd: GuardId, g_2: &G, rules: Option<f64>, reading: Option<bool>) -> Vec<crate::owner::OwnerDisagreesItem> { todo!() }

/// (src/game/owner.ts:118 ownerWitnessed)
pub fn owner_witnessed(g: &mut crate::game::Game) -> Vec<String> { todo!() }

/// (src/game/owner.ts:119 ownerRanks)
pub fn owner_ranks() -> Vec<String> { todo!() }
