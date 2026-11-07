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


/// INVENTED: posehome.ts moveIn, the pool's tenant, as a tick opens: every set of a man in the game at home, and no other. A set at home that is on no list (a man left, a restart made the guards anew) means everybody out, the pool emptied, those there back in. "Those there" is each set of eachPosed: a ragdoll's pose is a pose of its own (g.loose_poses), which keeps the pool's one rule and its count (levelfit.ts fitRows).
pub fn pose_home_tend(g: &mut crate::game::Game) { todo!() }
/// what `stage_open` lent, for `stage_shut`
pub struct Stage { pub set: PoseSetId, pub live: u32, pub back: Vec<(crate::anim_skeleton::Pose, crate::anim_skeleton::Pose)> }
/// INVENTED (skeleton.ts stageOpen): the update of a man whose set is not at home opens: his poses into the pool's end, his handles on them, and the set told it stands. None: the set is at home and nothing is lent
pub fn stage_open(g: &mut crate::game::Game, c: CharId) -> Option<Stage> { todo!() }
/// INVENTED (skeleton.ts, as the update shuts): the poses back into the rows they came from, and the pool as it was, to the byte
pub fn stage_shut(g: &mut crate::game::Game, c: CharId, st: Stage) { todo!() }

