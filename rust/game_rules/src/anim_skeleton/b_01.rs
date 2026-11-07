//! THE BRING-UP'S OWN PART of anim_skeleton: bodies for skeleton stubs the first tick reached (bringup-log.md says which is a boundary no-op and which is translated)
#![allow(unused_imports, unused_variables)]
use super::*;                      // your own module: its types, its other parts' functions
use crate::game::Game;
use crate::ids::*;
use crate::js::{self, JsMap, JsSet};
use crate::m;
use crate::v3::{self, Quat, V3};
use alloc::{boxed::Box, string::String, vec, vec::Vec};

// ---- THE BRING-UP'S HAND-OVER OF rust/sim's MODULE-LEVEL TABLES. A game's block is not all of rust/sim's state: the knobs' values, the clip store, the
// rig and the roles on the bench are statics of the module, written by the host as it goes, and the game's own state remembers THAT it wrote them
// (pose_bench.rig, layers.roles, clip_side[..].at). A game imported into a fresh module remembers writes this module never had.

