//! THE BRING-UP'S OWN PART of render_reach_keep: bodies for skeleton stubs a tick reached (bringup2-log.md says which is a boundary no-op and which is translated)
#![allow(unused_imports, unused_variables)]
use super::*;                      // your own module: its types, its other parts' functions
use crate::game::Game;
use crate::ids::*;
use crate::js::{self, JsMap, JsSet};
use crate::m;
use crate::v3::{self, Quat, V3};
use alloc::{boxed::Box, string::String, vec, vec::Vec};


/// BOUNDARY: the host's reach keeper takes the level's doors and roofed cells and builds its masks; a run with no picture has no keeper (`engine.reach?.boot`), so nothing happens   (render/reachKeep.ts:64 ReachKeep.boot)
pub fn reach_keep_boot(g: &mut crate::game::Game, doors: &[crate::render_reach::ReachDoor], interior: &[u8]) {}

/// BOUNDARY: the host's reach keeper makes this directional light's mask; a run with no picture has no keeper (`engine.reach?.dir`), so nothing happens   (render/reachKeep.ts:160 ReachKeep.dir)
pub fn reach_keep_dir(g: &mut crate::game::Game, l: LightId, only: Option<&crate::render_reach::ReachOnly>) {}

/// BOUNDARY: the host's reach keeper makes this wide sun's mask for its disc; a run with no picture has no keeper (`engine.reach?.sun`), so nothing happens   (render/reachKeep.ts:167 ReachKeep.sun)
pub fn reach_keep_sun(g: &mut crate::game::Game, l: LightId, radius: f64) {}
