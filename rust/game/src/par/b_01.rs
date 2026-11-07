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

/// BOUNDARY: the HUD's words only (objective_text's readers are the page's overlay, corner and tab card); at the swap the host puts the par clock or the result on this row. Hands back the row as `row() + text` gives it with no par live. (src/game/par.ts:110 Par.objective)
pub fn par_objective(g: &mut crate::game::Game, text: &str) -> String { String::from(text) }
