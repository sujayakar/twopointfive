//! THE BRING-UP'S OWN PART of wet_ground: bodies for skeleton stubs the first tick reached (bringup-log.md says which is a boundary no-op and which is translated)
#![allow(unused_imports, unused_variables)]
use super::*;                      // your own module: its types, its other parts' functions
use crate::game::Game;
use crate::ids::*;
use crate::js::{self, JsMap, JsSet};
use crate::m;
use crate::v3::{self, Quat, V3};
use alloc::{boxed::Box, string::String, vec, vec::Vec};

/// BOUNDARY NO-OP (the picture's / the host's): (src/game/wetGround.ts:101 WetGround.follow)
pub fn wet_ground_follow(g: &mut crate::game::Game) {}

/// BOUNDARY NO-OP (the picture's / the host's): (src/game/wetGround.ts:113 WetGround.dry)
pub fn wet_ground_dry(g: &mut crate::game::Game) {}
