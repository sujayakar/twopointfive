#![allow(unused_variables)]
//! Every function and method of `edge_look`, with an empty body. Written by the skeleton's program.
#[allow(unused_imports)]
use crate::{ids::*, js::*, Quat, V3};
#[allow(unused_imports)]
use alloc::{string::String, vec::Vec};

/// (src/game/edgeLook.ts:89 EdgeLook.constructor)
pub fn edge_look_new(g: &mut crate::game::Game) -> crate::edge_look::EdgeLook { todo!() }

/// (src/game/edgeLook.ts:94 EdgeLook.opened)
pub fn edge_look_opened(g: &mut crate::game::Game, b: &crate::edge_look::BreakLike, fresh: &[f64], cause: &str, dir: Option<V3>, quiet: bool) { todo!() }

/// (src/game/edgeLook.ts:98 EdgeLook.fail)
pub fn edge_look_fail(g: &mut crate::game::Game, e: crate::Opaque) { todo!() }

/// (src/game/edgeLook.ts:101 EdgeLook.dress)
pub fn edge_look_dress(g: &mut crate::game::Game, b: &crate::edge_look::BreakLike, fresh: &[f64], cause: &str, dir: Option<V3>, quiet: bool) { todo!() }

/// (src/game/edgeLook.ts:108 EdgeLook.cap)
pub fn edge_look_cap(g: &mut crate::game::Game) { todo!() }

/// (src/game/edgeLook.ts:114 EdgeLook.clear)
pub fn edge_look_clear(g: &mut crate::game::Game) { todo!() }

/// (src/game/edgeLook.ts:117 EdgeLook.update)
pub fn edge_look_update(g: &mut crate::game::Game, dt: f64) { todo!() }

/// (src/game/edgeLook.ts:144 EdgeLook.feet)
pub fn edge_look_feet(g: &mut crate::game::Game, men: &[crate::debris::FootMan], k: crate::debris::FootKnobs, dt: f64) { todo!() }

/// (src/game/edgeLook.ts:158 EdgeLook.flush)
pub fn edge_look_flush(g: &mut crate::game::Game) { todo!() }

/// (src/game/edgeLook.ts:160 EdgeLook.boxes)
pub fn edge_look_boxes(g: &mut crate::game::Game, out: &mut Vec<BoxId>) { todo!() }

/// (src/game/edgeLook.ts:169 EdgeLook.frame)
pub fn edge_look_frame(g: &mut crate::game::Game, b: &crate::edge_look::BreakLike) -> crate::edge_look::Frame { todo!() }

/// (src/game/edgeLook.ts:175 EdgeLook.at)
pub fn edge_look_at(g: &mut crate::game::Game, f: crate::edge_look::Frame, u: f64, y: f64, v: f64) -> V3 { todo!() }

/// (src/game/edgeLook.ts:177 EdgeLook.plate)
pub fn edge_look_plate(g: &mut crate::game::Game, f: crate::edge_look::Frame, u: f64, y: f64, v: f64, a: f64, hb: f64, ht: f64, th: f64, albedo: V3, key: f64, solid: Option<bool>) -> BoxId { todo!() }

/// (src/game/edgeLook.ts:184 EdgeLook.rebuild)
pub fn edge_look_rebuild(g: &mut crate::game::Game, b: &crate::edge_look::BreakLike) { todo!() }

/// (src/game/edgeLook.ts:318 EdgeLook.inBeam)
pub fn edge_look_in_beam(g: &mut crate::game::Game, p: V3, r: f64) -> bool { todo!() }

/// (src/game/edgeLook.ts:323 EdgeLook.shed)
pub fn edge_look_shed(g: &mut crate::game::Game, b: &crate::edge_look::BreakLike, f: Option<crate::edge_look::Frame>, c_2: f64, dir: Option<V3>, quiet: bool, blast: bool) { todo!() }

/// (src/game/edgeLook.ts:389 EdgeLook.scorch)
pub fn edge_look_scorch(g: &mut crate::game::Game, b: &crate::edge_look::BreakLike, f: crate::edge_look::Frame, fresh: &[f64], dir: Option<V3>) { todo!() }
