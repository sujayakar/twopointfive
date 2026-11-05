//! THE BRING-UP'S OWN PART of wallskin: bodies for skeleton stubs a tick reached (bringup2-log.md says which is a boundary no-op and which is translated)
#![allow(unused_imports, unused_variables)]
use super::*;                      // your own module: its types, its other parts' functions
use crate::game::Game;
use crate::ids::*;
use crate::js::{self, JsMap, JsSet};
use crate::m;
use crate::v3::{self, Quat, V3};
use alloc::{boxed::Box, string::String, vec, vec::Vec};

/// BOUNDARY NO-OP (round three, written ahead of need; the picture's: the wall band's paint, "told by one line of wallbreak's openCells; the host keeps it")   (wallskin.ts:155 WallSkin.clear)
pub fn wall_skin_clear(g: &mut crate::game::Game) {}
