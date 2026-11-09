//! THE BRING-UP'S OWN PART of smoke_types: bodies for skeleton stubs the first tick reached (bringup-log.md says which is a boundary no-op and which is translated)
#![allow(unused_imports, unused_variables)]
use super::*;                      // your own module: its types, its other parts' functions
use crate::game::Game;
use crate::ids::*;
use crate::js::{self, JsMap, JsSet};
use crate::m;
use crate::v3::{self, Quat, V3};
use alloc::{boxed::Box, string::String, vec, vec::Vec};

// THE SOLVER'S OBJECT IS ALSO WHERE THE GAME'S OWN SMOKE IS FED (crate::sense). 
// NOTHING COMES BACK FROM THE PICTURE: the tick says what smoke it wants (an emission, a round's line, a push, a clear) as out events, always, and the host's solver
// takes it or drops it by what it is doing and by its own budget.
/// (src/smoke/types.ts:64 SmokeSystemLike.emit)
pub fn smoke_system_like_emit(g: &mut crate::game::Game, e: &crate::smoke_types::SmokeEmitter) { crate::sense::smoke_emit(g, e); }

/// how many more emitters would still be packed this step: the game's own count where it owns the smoke, else the host's solver's (none here)   (types.ts:74)
pub fn smoke_system_like_budget(g: &crate::game::Game) -> f64 { crate::sense::smoke_budget(g) }

/// BOUNDARY NO-OP (the picture's / the host's): (src/smoke/types.ts:80 SmokeSystemLike.clearAll)
pub fn smoke_system_like_clear_all(g: &mut crate::game::Game) { crate::sense::smoke_clear(g); }


/// BOUNDARY NO-OP (the picture's / the host's): (src/smoke/types.ts:86 SmokeSystemLike.setWind)
pub fn smoke_system_like_set_wind(g: &crate::game::Game, srcs: &[crate::smoke_types::WindGpuSource], relax: f64) {}

