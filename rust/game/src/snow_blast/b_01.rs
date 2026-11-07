//! THE BRING-UP'S OWN PART of snow_blast: bodies for skeleton stubs the first tick reached (bringup-log.md says which is a boundary no-op and which is translated)
#![allow(unused_imports, unused_variables)]
use super::*;                      // your own module: its types, its other parts' functions
use crate::game::Game;
use crate::ids::*;
use crate::js::{self, JsMap, JsSet};
use crate::m;
use crate::v3::{self, Quat, V3};
use alloc::{boxed::Box, string::String, vec, vec::Vec};


/// BOUNDARY NO-OP (dress, row DRESS.snow: a blast digs the drawn snow)   (snowBlast.ts:142 snowBlastAt)
pub fn snow_blast_at(g: &crate::game::Game, at: &[f64], radius: f64) -> bool { false }
