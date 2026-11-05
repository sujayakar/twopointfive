//! THE BRING-UP'S OWN PART of snow_blast: bodies for skeleton stubs the first tick reached (bringup-log.md says which is a boundary no-op and which is translated)
#![allow(unused_imports, unused_variables)]
use super::*;                      // your own module: its types, its other parts' functions
use crate::game::Game;
use crate::ids::*;
use crate::js::{self, JsMap, JsSet};
use crate::m;
use crate::v3::{self, Quat, V3};
use alloc::{boxed::Box, string::String, vec, vec::Vec};

/// BOUNDARY NO-OP (the picture's / the host's): (src/game/snowBlast.ts:96 owedOf)
pub fn owed_of(b: &mut crate::snow_blast::SnowBlastShown, live: &[crate::snow_blast::Live]) {}

/// BOUNDARY NO-OP (the picture's / the host's): (src/game/snowBlast.ts:114 attachSnowBlast)
pub fn attach_snow_blast(g: &mut crate::game::Game) {}

/// BOUNDARY NO-OP (the picture's / the host's): (src/game/snowBlast.ts:122 clearSnowBlast)
pub fn clear_snow_blast(g: &mut crate::game::Game) {}

/// BOUNDARY NO-OP (the picture's / the host's): (src/game/snowBlast.ts:168 stepSnowBlast)
pub fn step_snow_blast(g: &mut crate::game::Game, dt: f64) {}

/// BOUNDARY NO-OP (the picture's / the host's): (src/game/snowBlast.ts:176 snowBlastFeed)
pub fn snow_blast_feed(g: &mut crate::game::Game, c_2: &mut Vec<f32>) {}
