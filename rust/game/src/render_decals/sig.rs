#![allow(unused_variables)]
//! Every function and method of `render_decals`, with an empty body. Written by the skeleton's program.
#[allow(unused_imports)]
use crate::{ids::*, js::*, Quat, V3};
#[allow(unused_imports)]
use alloc::{string::String, vec::Vec};

/// (src/render/decals.ts:145 Decals.constructor)
pub fn decals_new(g: &mut crate::game::Game, device: crate::Opaque) -> crate::render_decals::Decals { todo!() }

/// (src/render/decals.ts:149 Decals.count)
pub fn decals_count(g: &mut crate::game::Game) -> f64 { todo!() }

/// (src/render/decals.ts:153 Decals.add)
pub fn decals_add(g: &mut crate::game::Game, kind: crate::render_decals::DecalKind, pos: V3, normal: V3, size: Option<f64>, opts: Option<crate::render_decals::DecalOpts>) -> Option<crate::render_decals::Decal> { todo!() }

/// (src/render/decals.ts:178 Decals.clear)
pub fn decals_clear(g: &mut crate::game::Game) { todo!() }

/// (src/render/decals.ts:180 Decals.remove)
pub fn decals_remove(g: &mut crate::game::Game, d: crate::render_decals::Decal) -> bool { todo!() }

/// (src/render/decals.ts:182 Decals.touch)
pub fn decals_touch(g: &mut crate::game::Game) { todo!() }

/// (src/render/decals.ts:261 toLocal)
pub fn to_local(g: &mut crate::game::Game, b: BoxId, v: V3, point: bool) -> V3 { todo!() }

/// (src/render/decals.ts:267 toWorld)
pub fn to_world(g: &mut crate::game::Game, b: BoxId, l: V3, point: bool) -> V3 { todo!() }

/// (src/render/decals.ts:276 clampToFace)
pub fn clamp_to_face(g: &mut crate::game::Game, b: BoxId, p: V3, n: V3, r: f64, on_cut: Option<bool>) -> V3 { todo!() }

/// (src/render/decals.ts:286 perpendicular)
pub fn perpendicular(n: V3) -> V3 { todo!() }
