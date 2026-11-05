//! THE BRING-UP'S OWN PART of fixture_break: bodies for skeleton stubs the first tick reached (bringup-log.md says which is a boundary no-op and which is translated)
#![allow(unused_imports, unused_variables)]
use super::*;                      // your own module: its types, its other parts' functions
use crate::game::Game;
use crate::ids::*;
use crate::js::{self, JsMap, JsSet};
use crate::m;
use crate::v3::{self, Quat, V3};
use alloc::{boxed::Box, string::String, vec, vec::Vec};

/// BOUNDARY NO-OP (the picture's / the host's): (src/game/fixtureBreak.ts:166 pose)
pub fn pose(g: &mut crate::game::Game, r: &mut crate::fixture_break::Rec) {}

/// BOUNDARY NO-OP (the picture's / the host's): (src/game/fixtureBreak.ts:179 land)
pub fn land(r: &mut crate::fixture_break::Rec, s: f64) {}

/// BOUNDARY NO-OP (the picture's / the host's): (src/game/fixtureBreak.ts:197 fixtureBreakBoxes)
pub fn fixture_break_boxes(g: &mut crate::game::Game, out: &mut Vec<BoxId>, dt: f64) {}
