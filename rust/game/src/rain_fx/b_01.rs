//! THE BRING-UP'S OWN PART of rain_fx: bodies for skeleton stubs the first tick reached (bringup-log.md says which is a boundary no-op and which is translated)
#![allow(unused_imports, unused_variables)]
use super::*;                      // your own module: its types, its other parts' functions
use crate::game::Game;
use crate::ids::*;
use crate::js::{self, JsMap, JsSet};
use crate::m;
use crate::v3::{self, Quat, V3};
use alloc::{boxed::Box, string::String, vec, vec::Vec};

/// BOUNDARY NO-OP (the picture's / the host's): (src/game/rainFx.ts:153 updateRainFx)
pub fn update_rain_fx(g: &mut crate::game::Game, dt: f64, cam: CameraId) {}

/// BOUNDARY NO-OP (the picture's / the host's): (src/game/rainFx.ts:271 buildDripEdges)
pub fn build_drip_edges(g: &mut crate::game::Game, st: &mut crate::rain_fx::RainState) {}

/// BOUNDARY NO-OP (the picture's / the host's): (src/game/rainFx.ts:275 setDripEdges)
pub fn set_drip_edges(st: &mut crate::rain_fx::RainState, out: &[crate::rain_fx::DripPoint], boxes: f64) {}

/// BOUNDARY NO-OP (the picture's / the host's): (src/game/rainFx.ts:282 paceDripEdges)
pub fn pace_drip_edges(g: &mut crate::game::Game, st: &mut crate::rain_fx::RainState, now: bool) {}

/// BOUNDARY: the host's debug panel writes this line from the particle pool and the frame's rain stats. No stats run here (update_rain_fx is the host's), so it is a fresh book's line.   (src/game/rainFx.ts:341 rainSummary)
pub fn rain_summary(g: &mut crate::game::Game) -> String { String::from("particle rain: off") }
