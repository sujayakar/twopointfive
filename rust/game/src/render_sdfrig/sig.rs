#![allow(unused_variables)]
//! Every function and method of `render_sdfrig`, with an empty body. Written by the skeleton's program.
#[allow(unused_imports)]
use crate::{ids::*, js::*, Quat, V3};
#[allow(unused_imports)]
use alloc::{string::String, vec::Vec};

/// (src/render/sdfrig.ts:58 sdRoundCone)
pub fn sd_round_cone(p: V3, a: V3, b: V3, r0: f64, r1: f64) -> f64 { todo!() }

/// (src/render/sdfrig.ts:99 bindJoints)
pub fn bind_joints(g: &mut crate::game::Game, ch: RigId) -> JsMap<String, V3> { todo!() }

/// (src/render/sdfrig.ts:115 fitRig)
pub fn fit_rig(g: &mut crate::game::Game, ch: RigId, parts: Option<f64>) -> crate::render_sdfrig::RigFit { todo!() }

/// (src/render/sdfrig.ts:181 fitRigCast)
pub fn fit_rig_cast(g: &mut crate::game::Game, ch: RigId) { todo!() }

/// (src/render/sdfrig.ts:191 meshRows)
pub fn mesh_rows(g: &mut crate::game::Game, ch: RigId) -> crate::render_sdfrig::MeshRows { todo!() }

/// (src/render/sdfrig.ts:246 sdfRigWgslConsts)
pub fn sdf_rig_wgsl_consts() -> String { todo!() }
