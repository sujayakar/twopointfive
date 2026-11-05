//! THE BRING-UP'S OWN PART of smoke_types: bodies for skeleton stubs the first tick reached (bringup-log.md says which is a boundary no-op and which is translated)
#![allow(unused_imports, unused_variables)]
use super::*;                      // your own module: its types, its other parts' functions
use crate::game::Game;
use crate::ids::*;
use crate::js::{self, JsMap, JsSet};
use crate::m;
use crate::v3::{self, Quat, V3};
use alloc::{boxed::Box, string::String, vec, vec::Vec};

/// BOUNDARY NO-OP (the picture's / the host's): (src/smoke/types.ts:64 SmokeSystemLike.emit)
pub fn smoke_system_like_emit(g: &mut crate::game::Game, e: crate::smoke_types::SmokeEmitter) {}

/// BOUNDARY NO-OP (the picture's / the host's): (src/smoke/types.ts:65 SmokeSystemLike.push)
pub fn smoke_system_like_push(g: &mut crate::game::Game, p: crate::smoke_types::SmokePush) {}

/// BOUNDARY NO-OP (the picture's / the host's): (src/smoke/types.ts:73 SmokeSystemLike.keepHot)
pub fn smoke_system_like_keep_hot(g: &mut crate::game::Game, heat: f64) {}

/// BOUNDARY NO-OP (the picture's / the host's): (src/smoke/types.ts:80 SmokeSystemLike.clearAll)
pub fn smoke_system_like_clear_all(g: &mut crate::game::Game) {}

/// BOUNDARY NO-OP (the picture's / the host's): (src/smoke/types.ts:82 SmokeSystemLike.obstaclesChanged)
pub fn smoke_system_like_obstacles_changed(g: &mut crate::game::Game, c_2: V3, r: f64) {}

/// BOUNDARY NO-OP (the picture's / the host's): (src/smoke/types.ts:86 SmokeSystemLike.setWind)
pub fn smoke_system_like_set_wind(g: &mut crate::game::Game, srcs: &[crate::smoke_types::WindGpuSource], relax: f64) {}

/// BOUNDARY NO-OP (the picture's / the host's): (src/smoke/types.ts:89 SmokeSystemLike.setStir)
pub fn smoke_system_like_set_stir(g: &mut crate::game::Game, amount: f64) {}
