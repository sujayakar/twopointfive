//! THE BRING-UP'S OWN PART of wallwear: bodies for skeleton stubs the first tick reached (bringup-log.md says which is a boundary no-op and which is translated)
#![allow(unused_imports, unused_variables)]
use super::*;                      // your own module: its types, its other parts' functions
use crate::game::Game;
use crate::ids::*;
use crate::js::{self, JsMap, JsSet};
use crate::m;
use crate::v3::{self, Quat, V3};
use alloc::{boxed::Box, string::String, vec, vec::Vec};

/// BOUNDARY NO-OP (the picture's / the host's): (src/game/wallwear.ts:58 WallWear.round)
pub fn wall_wear_round(g: &mut crate::game::Game, box_: BoxId, surf: crate::scene_materials::Surf, key: f64, counted: f64, frac: f64, hit_p: V3, dir: V3) {}

/// BOUNDARY NO-OP (the picture's / the host's): (src/game/wallwear.ts:88 WallWear.reset)
pub fn wall_wear_reset(g: &mut crate::game::Game) {}

/// BOUNDARY NO-OP (the picture's / the host's): (src/game/wallwear.ts:101 WallWear.shape)
pub fn wall_wear_shape(g: &mut crate::game::Game, s: f64, serial: f64, half: f64, mat: f64) {}

/// BOUNDARY NO-OP (the picture's / the host's): (src/game/wallwear.ts:107 WallWear.throwOut)
pub fn wall_wear_throw_out(g: &mut crate::game::Game, box_: BoxId, surf: crate::scene_materials::Surf, hit_p: V3, dir: V3, n: V3, half: f64, through: bool, s: f64) {}
