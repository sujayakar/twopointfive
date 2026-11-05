//! THE BRING-UP'S OWN PART of par: bodies for skeleton stubs the first tick reached (bringup-log.md says which is a boundary no-op and which is translated)
#![allow(unused_imports, unused_variables)]
use super::*;                      // your own module: its types, its other parts' functions
use crate::game::Game;
use crate::ids::*;
use crate::js::{self, JsMap, JsSet};
use crate::m;
use crate::v3::{self, Quat, V3};
use alloc::{boxed::Box, string::String, vec, vec::Vec};

/// BOUNDARY NO-OP (the picture's / the host's): (src/game/par.ts:87 Par.tick)
pub fn par_tick(g: &mut crate::game::Game) {}

/// BOUNDARY NO-OP (the picture's / the host's): (src/game/par.ts:103 Par.fresh)
pub fn par_fresh(g: &mut crate::game::Game, tail: Option<u32>) {}

/// BOUNDARY NO-OP (the picture's / the host's): (src/game/par.ts:128 Par.restore)
pub fn par_restore(g: &mut crate::game::Game, s: Option<crate::par::ParSnap>) {}

/// BOUNDARY NO-OP (the picture's / the host's): (src/game/par.ts:135 Par.note)
pub fn par_note(g: &mut crate::game::Game, s: &str) {}
