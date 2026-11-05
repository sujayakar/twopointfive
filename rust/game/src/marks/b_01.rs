//! THE BRING-UP'S OWN PART of marks: bodies for skeleton stubs the first tick reached (bringup-log.md says which is a boundary no-op and which is translated)
#![allow(unused_imports, unused_variables)]
use super::*;                      // your own module: its types, its other parts' functions
use crate::game::Game;
use crate::ids::*;
use crate::js::{self, JsMap, JsSet};
use crate::m;
use crate::v3::{self, Quat, V3};
use alloc::{boxed::Box, string::String, vec, vec::Vec};

/// BOUNDARY NO-OP (the picture's / the host's): (src/game/marks.ts:39 bulletMark)
pub fn bullet_mark(g: &mut crate::game::Game, hit_p: V3, n: V3, s: Option<crate::marks::Surface2>, def: &crate::weapons::WeaponDef) {}

/// BOUNDARY NO-OP (the picture's / the host's): (src/game/marks.ts:51 bangScorch)
pub fn bang_scorch(g: &mut crate::game::Game, at: V3) {}

/// BOUNDARY NO-OP (the picture's / the host's): (src/game/marks.ts:90 kickScuff)
pub fn kick_scuff(g: &mut crate::game::Game, d: DoorId, from: V3) {}

/// BOUNDARY NO-OP (the picture's / the host's): (src/game/marks.ts:104 chipMark)
pub fn chip_mark(g: &mut crate::game::Game, p: V3, n: V3, s: Option<crate::marks::Surface2>, size: Option<f64>) {}
