//! THE BRING-UP'S OWN PART of smoke_types: bodies for skeleton stubs the first tick reached (bringup-log.md says which is a boundary no-op and which is translated)
#![allow(unused_imports, unused_variables)]
use super::*;                      // your own module: its types, its other parts' functions
use crate::game::Game;
use crate::ids::*;
use crate::js::{self, JsMap, JsSet};
use crate::m;
use crate::v3::{self, Quat, V3};
use alloc::{boxed::Box, string::String, vec, vec::Vec};

// THE SOLVER'S OBJECT IS ALSO WHERE THE GAME'S OWN SMOKE IS FED (crate::sense). What the HOST'S solver answers (inSmokeDomain, round, drawing) is input a tick reads: with no solver they are the harness's null
// object's answers (headless.ts: emit / push / clearAll dropped, inSmokeDomain false, the optional ones absent).  BRIDGE-ASK: the host's answers for these four.
/// (src/smoke/types.ts:64 SmokeSystemLike.emit)
pub fn smoke_system_like_emit(g: &mut crate::game::Game, e: &crate::smoke_types::SmokeEmitter) { crate::sense::smoke_emit(g, e); }

/// how many more emitters would still be packed this step: the game's own count where it owns the smoke, else the host's solver's (none here)   (types.ts:74)
pub fn smoke_system_like_budget(g: &crate::game::Game) -> f64 { crate::sense::smoke_budget(g) }
/// the host's solver's answer; with no solver, false   (types.ts:67 inSmokeDomain)
pub fn smoke_system_like_in_smoke_domain(g: &crate::game::Game, p: V3) -> bool { false }
/// the host's solver's answer; with no solver it did not take the round as a line   (types.ts:70 round)
pub fn smoke_system_like_round(g: &crate::game::Game, from: V3, dir: V3, len: f64) -> bool { false }
/// the host's solver's answer; absent = no   (types.ts:77 drawing)
pub fn smoke_system_like_drawing(g: &crate::game::Game) -> bool { false }



/// BOUNDARY NO-OP (the picture's / the host's): (src/smoke/types.ts:80 SmokeSystemLike.clearAll)
pub fn smoke_system_like_clear_all(g: &mut crate::game::Game) { crate::sense::smoke_clear(g); }


/// BOUNDARY NO-OP (the picture's / the host's): (src/smoke/types.ts:86 SmokeSystemLike.setWind)
pub fn smoke_system_like_set_wind(g: &crate::game::Game, srcs: &[crate::smoke_types::WindGpuSource], relax: f64) {}

