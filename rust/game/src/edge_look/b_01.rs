//! THE BRING-UP'S OWN PART of edge_look: bodies for skeleton stubs the first tick reached (bringup-log.md says which is a boundary no-op and which is translated)
#![allow(unused_imports, unused_variables)]
use super::*;                      // your own module: its types, its other parts' functions
use crate::game::Game;
use crate::ids::*;
use crate::js::{self, JsMap, JsSet};
use crate::m;
use crate::v3::{self, Quat, V3};
use alloc::{boxed::Box, string::String, vec, vec::Vec};

/// BOUNDARY NO-OP (the picture's / the host's): (src/game/edgeLook.ts:94 EdgeLook.opened)
pub fn edge_look_opened(g: &mut crate::game::Game, b: &crate::edge_look::BreakLike, fresh: &[f64], cause: &str, dir: Option<V3>, quiet: bool) {}

/// BOUNDARY NO-OP (the picture's / the host's): (src/game/edgeLook.ts:98 EdgeLook.fail)
pub fn edge_look_fail(g: &mut crate::game::Game, e: crate::Opaque) {}

/// BOUNDARY NO-OP (the picture's / the host's): (src/game/edgeLook.ts:101 EdgeLook.dress)
pub fn edge_look_dress(g: &mut crate::game::Game, b: &crate::edge_look::BreakLike, fresh: &[f64], cause: &str, dir: Option<V3>, quiet: bool) {}

/// BOUNDARY NO-OP (the picture's / the host's): (src/game/edgeLook.ts:108 EdgeLook.cap)
pub fn edge_look_cap(g: &mut crate::game::Game) {}

/// BOUNDARY NO-OP (the picture's / the host's): (src/game/edgeLook.ts:114 EdgeLook.clear)
pub fn edge_look_clear(g: &mut crate::game::Game) {}

/// BOUNDARY NO-OP (the picture's / the host's): (src/game/edgeLook.ts:117 EdgeLook.update)
pub fn edge_look_update(g: &mut crate::game::Game, dt: f64) {}

/// BOUNDARY NO-OP (the picture's / the host's): (src/game/edgeLook.ts:144 EdgeLook.feet)
pub fn edge_look_feet(g: &mut crate::game::Game, men: &[crate::debris::FootMan], k: crate::debris::FootKnobs, dt: f64) {}

/// BOUNDARY NO-OP (the picture's / the host's): (src/game/edgeLook.ts:158 EdgeLook.flush)
pub fn edge_look_flush(g: &mut crate::game::Game) {}

/// BOUNDARY NO-OP (the picture's / the host's): (src/game/edgeLook.ts:160 EdgeLook.boxes)
pub fn edge_look_boxes(g: &mut crate::game::Game, out: &mut Vec<BoxId>) {}

/// BOUNDARY NO-OP (the picture's / the host's): (src/game/edgeLook.ts:184 EdgeLook.rebuild)
pub fn edge_look_rebuild(g: &mut crate::game::Game, b: &crate::edge_look::BreakLike) {}

/// BOUNDARY NO-OP (the picture's / the host's): (src/game/edgeLook.ts:323 EdgeLook.shed)
pub fn edge_look_shed(g: &mut crate::game::Game, b: &crate::edge_look::BreakLike, f: Option<crate::edge_look::Frame>, c_2: f64, dir: Option<V3>, quiet: bool, blast: bool) {}

/// BOUNDARY NO-OP (the picture's / the host's): (src/game/edgeLook.ts:389 EdgeLook.scorch)
pub fn edge_look_scorch(g: &mut crate::game::Game, b: &crate::edge_look::BreakLike, f: crate::edge_look::Frame, fresh: &[f64], dir: Option<V3>) {}
