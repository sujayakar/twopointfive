#![allow(unused_variables)]
//! Every function and method of `marks`, with an empty body. Written by the skeleton's program.
#[allow(unused_imports)]
use crate::{ids::*, js::*, Quat, V3};
#[allow(unused_imports)]
use alloc::{string::String, vec::Vec};

/// (src/game/marks.ts:17 staticSurface)
pub fn static_surface(g: &mut crate::game::Game, index: f64) -> Option<crate::marks::Surface2> { todo!() }

/// (src/game/marks.ts:24 markable)
pub fn markable(g: &mut crate::game::Game, b: BoxId, n: V3) -> bool { todo!() }

/// (src/game/marks.ts:30 faceHalfExtents)
pub fn face_half_extents(g: &mut crate::game::Game, b: BoxId, n: V3) -> [f64; 2] { todo!() }

/// (src/game/marks.ts:39 bulletMark)
pub fn bullet_mark(g: &mut crate::game::Game, hit_p: V3, n: V3, s: Option<crate::marks::Surface2>, def: &crate::weapons::WeaponDef) { todo!() }

/// (src/game/marks.ts:51 bangScorch)
pub fn bang_scorch(g: &mut crate::game::Game, at: V3) { todo!() }

/// (src/game/marks.ts:90 kickScuff)
pub fn kick_scuff(g: &mut crate::game::Game, d: DoorId, from: V3) { todo!() }

/// (src/game/marks.ts:104 chipMark)
pub fn chip_mark(g: &mut crate::game::Game, p: V3, n: V3, s: Option<crate::marks::Surface2>, size: Option<f64>) { todo!() }

/// (src/game/marks.ts:152 unit)
pub fn unit(x: f64) -> f64 { todo!() }
