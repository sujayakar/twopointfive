//! What the parts of `posehome` share, and what it hands to the host.
#![allow(unused_variables)]
#[allow(unused_imports)]
use crate::{ids::*, js::*, Quat, V3};
#[allow(unused_imports)]
use alloc::{string::String, vec::Vec};


/// what `stage_open` lent, for `stage_shut`
pub struct Stage { pub set: PoseSetId, pub live: u32, pub back: Vec<(crate::anim_skeleton::Pose, crate::anim_skeleton::Pose)> }
