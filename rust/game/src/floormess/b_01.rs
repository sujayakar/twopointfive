//! THE BRING-UP'S OWN PART of floormess: bodies for skeleton stubs the first tick reached (bringup-log.md says which is a boundary no-op and which is translated)
#![allow(unused_imports, unused_variables)]
use super::*;                      // your own module: its types, its other parts' functions
use crate::game::Game;
use crate::ids::*;
use crate::js::{self, JsMap, JsSet};
use crate::m;
use crate::v3::{self, Quat, V3};
use alloc::{boxed::Box, string::String, vec, vec::Vec};

/// BOUNDARY NO-OP (the picture's / the host's): (src/game/floormess.ts:87 spread)
pub fn spread(g: &mut crate::game::Game, x: f64, z: f64, s: crate::floormess::Spread, size: f64, face: Option<[f64; 2]>) {}

/// BOUNDARY NO-OP (the picture's / the host's): (src/game/floormess.ts:108 messBurst)
pub fn mess_burst(g: &mut crate::game::Game, at: V3, size: Option<f64>) {}

/// BOUNDARY NO-OP (the picture's / the host's): (src/game/floormess.ts:115 messWall)
pub fn mess_wall(g: &mut crate::game::Game, p: V3, n: V3, thick: f64, through: bool) {}

/// BOUNDARY NO-OP (the picture's / the host's): (src/game/floormess.ts:123 messOpened)
pub fn mess_opened(g: &mut crate::game::Game, rect: [f64; 4]) {}

/// BOUNDARY NO-OP (the picture's / the host's): (src/game/floormess.ts:130 messEvicted)
pub fn mess_evicted(g: &mut crate::game::Game, c_2: DebrisChunkId) {}
