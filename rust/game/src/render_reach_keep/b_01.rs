//! THE BRING-UP'S OWN PART of render_reach_keep: bodies for skeleton stubs a tick reached (bringup2-log.md says which is a boundary no-op and which is translated)
#![allow(unused_imports, unused_variables)]
use super::*;                      // your own module: its types, its other parts' functions
use crate::game::Game;
use crate::ids::*;
use crate::js::{self, JsMap, JsSet};
use crate::m;
use crate::v3::{self, Quat, V3};
use alloc::{boxed::Box, string::String, vec, vec::Vec};

/// BOUNDARY NO-OP (round three, written ahead of need; the renderer's keeper of its reach masks is told a wall changed: the host's)   (render/reachKeep.ts:80 ReachKeep.changed)
pub fn reach_keep_changed(g: &mut crate::game::Game, rect: Option<[f64; 4]>, hold: Option<f64>) {}
