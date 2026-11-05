#![allow(unused_variables)]
//! Every function and method of `render_reach_keep`, with an empty body. Written by the skeleton's program.
#[allow(unused_imports)]
use crate::{ids::*, js::*, Quat, V3};
#[allow(unused_imports)]
use alloc::{string::String, vec::Vec};

/// (src/render/reachKeep.ts:45 ReachKeep.constructor)
pub fn reach_keep_new(g: &mut crate::game::Game, world: &crate::render_reach_keep::ReachKeepNewWorld, lights: &crate::render_reach_keep::ReachKeepNewLights) -> crate::render_reach_keep::ReachKeep { todo!() }

/// (src/render/reachKeep.ts:64 ReachKeep.boot)
pub fn reach_keep_boot(g: &mut crate::game::Game, doors: &[crate::render_reach::ReachDoor], interior: &[u8]) { todo!() }

/// (src/render/reachKeep.ts:80 ReachKeep.changed)
pub fn reach_keep_changed(g: &mut crate::game::Game, rect: Option<[f64; 4]>, hold: Option<f64>) { todo!() }

/// (src/render/reachKeep.ts:89 ReachKeep.asBooted)
pub fn reach_keep_as_booted(g: &mut crate::game::Game) { todo!() }

/// (src/render/reachKeep.ts:160 ReachKeep.dir)
pub fn reach_keep_dir(g: &mut crate::game::Game, l: LightId, only: Option<&crate::render_reach::ReachOnly>) { todo!() }

/// (src/render/reachKeep.ts:167 ReachKeep.sun)
pub fn reach_keep_sun(g: &mut crate::game::Game, l: LightId, radius: f64) { todo!() }

/// (src/render/reachKeep.ts:181 ReachKeep.dirMask)
pub fn reach_keep_dir_mask(g: &mut crate::game::Game, d: V3, only: Option<&crate::render_reach::ReachOnly>) -> Option<Vec<u32>> { todo!() }
