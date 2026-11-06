#![allow(unused_variables)]
//! Every function and method of `posehome`, with an empty body. Written by the skeleton's program.
#[allow(unused_imports)]
use crate::{ids::*, js::*, Quat, V3};
#[allow(unused_imports)]
use alloc::{string::String, vec::Vec};

/// (src/game/posehome.ts:31 eachPosed)
pub fn each_posed(g: &mut crate::game::Game, f: &mut dyn FnMut(&mut crate::game::Game, PoseSetId)) { todo!() }

/// (src/game/posehome.ts:38 manSingles)
pub fn man_singles(g: &mut crate::game::Game) -> f64 { todo!() }


/// INVENTED: posehome.ts moveIn, the pool's tenant, as a tick opens: every set of a man in the game at home, and no other. A set at home that is on no list (a man left, a restart made the guards anew) means everybody out, the pool emptied, those there back in. In Rust a ragdoll's pose lives in the pool too (pose_take_home), so "those there" is each set of eachPosed and each ragdoll's pose: that keeps the pool's one rule and its count (levelfit.ts fitRows).
pub fn pose_home_tend(g: &mut crate::game::Game) { todo!() }

