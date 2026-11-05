//! THE BRING-UP'S OWN PART of render_decals: bodies for skeleton stubs the first tick reached (bringup-log.md says which is a boundary no-op and which is translated)
#![allow(unused_imports, unused_variables)]
use super::*;                      // your own module: its types, its other parts' functions
use crate::game::Game;
use crate::ids::*;
use crate::js::{self, JsMap, JsSet};
use crate::m;
use crate::v3::{self, Quat, V3};
use alloc::{boxed::Box, string::String, vec, vec::Vec};

/// BOUNDARY NO-OP (the picture's / the host's): (src/render/decals.ts:178 Decals.clear)
pub fn decals_clear(g: &mut crate::game::Game) {}

/// BOUNDARY NO-OP (the picture's / the host's): (src/render/decals.ts:182 Decals.touch)
pub fn decals_touch(g: &mut crate::game::Game) {}
