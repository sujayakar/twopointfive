#![allow(unused_variables)]
//! Every function and method of `wallwear`, with an empty body. Written by the skeleton's program.
#[allow(unused_imports)]
use crate::{ids::*, js::*, Quat, V3};
#[allow(unused_imports)]
use alloc::{string::String, vec::Vec};

/// (src/game/wallwear.ts:41 matOf)
pub fn mat_of(surf: crate::scene_materials::Surf) -> f64 { todo!() }

/// (src/game/wallwear.ts:58 WallWear.round)
pub fn wall_wear_round(g: &mut crate::game::Game, box_: BoxId, surf: crate::scene_materials::Surf, key: f64, counted: f64, frac: f64, hit_p: V3, dir: V3) { todo!() }

/// (src/game/wallwear.ts:88 WallWear.reset)
pub fn wall_wear_reset(g: &mut crate::game::Game) { todo!() }

/// (src/game/wallwear.ts:92 WallWear.own)
pub fn wall_wear_own<'g>(g: &'g mut crate::game::Game, hit_p: V3) -> Option<&'g mut crate::render_decals::Decal> { todo!() }

/// (src/game/wallwear.ts:101 WallWear.shape)
pub fn wall_wear_shape(g: &mut crate::game::Game, d: &mut crate::render_decals::Decal, s: f64) { todo!() }

/// (src/game/wallwear.ts:107 WallWear.throwOut)
pub fn wall_wear_throw_out(g: &mut crate::game::Game, box_: BoxId, surf: crate::scene_materials::Surf, hit_p: V3, dir: V3, n: V3, half: f64, through: bool, s: f64) { todo!() }
