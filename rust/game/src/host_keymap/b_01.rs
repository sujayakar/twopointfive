//! THE BRING-UP'S OWN PART of host_keymap: bodies for skeleton stubs the first tick reached (bringup-log.md says which is a boundary no-op and which is translated)
#![allow(unused_imports, unused_variables)]
use super::*;                      // your own module: its types, its other parts' functions
use crate::game::Game;
use crate::ids::*;
use crate::js::{self, JsMap, JsSet};
use crate::m;
use crate::v3::{self, Quat, V3};
use alloc::{boxed::Box, string::String, vec, vec::Vec};

/// BOUNDARY NO-OP (the picture's / the host's): (src/host/keymap.ts:184 rebuild)
pub fn rebuild(g: &mut crate::game::Game) {}

/// BOUNDARY NO-OP (the picture's / the host's): (src/host/keymap.ts:243 fresh)
pub fn fresh(g: &mut crate::game::Game) {}
