#![allow(unused_variables)]
//! Every function and method of `scene_cut`, with an empty body. Written by the skeleton's program.
#[allow(unused_imports)]
use crate::{ids::*, js::*, Quat, V3};
#[allow(unused_imports)]
use alloc::{string::String, vec::Vec};

/// (src/scene/cut.ts:38 cutBoxesOn)
pub fn cut_boxes_on(g: &mut crate::game::Game) -> bool { todo!() }

/// (src/scene/cut.ts:52 cutPlane)
pub fn cut_plane(n: V3, d: f64) -> crate::scene_cut::CutPlane { todo!() }

/// (src/scene/cut.ts:57 cutThrough)
pub fn cut_through(p: V3, n: V3) -> crate::scene_cut::CutPlane { todo!() }

/// (src/scene/cut.ts:68 cutSupport)
pub fn cut_support(h: V3, n: V3) -> f64 { todo!() }

/// (src/scene/cut.ts:79 cutVerts)
pub fn cut_verts(h: V3, cut: Option<&[crate::scene_cut::CutPlane]>) -> Vec<V3> { todo!() }

/// (src/scene/cut.ts:111 cutRecord)
pub fn cut_record(h: V3, planes: &[crate::scene_cut::CutPlane]) -> crate::scene_cut::CutResult { todo!() }

/// (src/scene/cut.ts:141 cutBounds)
pub fn cut_bounds(h: V3, cut: Option<&[crate::scene_cut::CutPlane]>) -> crate::scene_cut::CutBounds { todo!() }

/// (src/scene/cut.ts:150 cutFit)
pub fn cut_fit(h: V3, cut: Option<&[crate::scene_cut::CutPlane]>) -> crate::scene_cut::CutFit { todo!() }

/// (src/scene/cut.ts:159 cutToWorld)
pub fn cut_to_world(v: V3, yaw: f64, rot: Option<Quat>) -> V3 { todo!() }

/// (src/scene/cut.ts:182 cutPrism)
pub fn cut_prism(axis: f64, half_t: f64, poly: &[[f64; 2]]) -> crate::scene_cut::CutPrism { todo!() }

/// (src/scene/cut.ts:235 cutClip)
pub fn cut_clip(g: &mut crate::game::Game, cut: &[crate::scene_cut::CutPlane], ox: f64, oy: f64, oz: f64, dx: f64, dy: f64, dz: f64, t0: f64, t1: f64) -> bool { todo!() }
