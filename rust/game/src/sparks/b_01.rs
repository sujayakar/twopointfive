//! THE BRING-UP'S OWN PART of sparks: bodies for skeleton stubs the first tick reached (bringup-log.md says which is a boundary no-op and which is translated)
#![allow(unused_imports, unused_variables)]
use super::*;                      // your own module: its types, its other parts' functions
use crate::game::Game;
use crate::ids::*;
use crate::js::{self, JsMap, JsSet};
use crate::m;
use crate::v3::{self, Quat, V3};
use alloc::{boxed::Box, string::String, vec, vec::Vec};

/// BOUNDARY NO-OP (the picture's / the host's): (src/game/sparks.ts:77 Sparks.clear)
pub fn sparks_clear(g: &mut crate::game::Game) {}

/// BOUNDARY NO-OP (the picture's / the host's): (src/game/sparks.ts:79 Sparks.update)
pub fn sparks_update(g: &mut crate::game::Game, dt: f64) {}

/// BOUNDARY NO-OP (the picture's / the host's): (src/game/sparks.ts:101 Sparks.boxes)
pub fn sparks_boxes(g: &mut crate::game::Game, out: &mut Vec<BoxId>) {}
