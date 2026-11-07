#![allow(unused_variables)]
//! Every function and method of `host_api`, with an empty body. Written by the skeleton's program.
#[allow(unused_imports)]
use crate::{ids::*, js::*, Quat, V3};
#[allow(unused_imports)]
use alloc::{string::String, vec::Vec};

/// (src/host/api.ts:76 HostApi.invalidateWorldCache)
pub fn host_api_invalidate_world_cache(g: &mut crate::game::Game) { todo!() }

/// (src/host/api.ts:79 HostApi.restartPicture)
pub fn host_api_restart_picture(g: &mut crate::game::Game, why: &str) { g.out.push(crate::out_events::OutEvent::HostApiRestartPicture(crate::out_events::OutHostApiRestartPicture { why: String::from(why) })); }

/// (src/host/api.ts:81 HostApi.startPlansOver)
pub fn host_api_start_plans_over(g: &mut crate::game::Game) { todo!() }

/// (src/host/api.ts:89 HostApi.setMeshShadows)
pub fn host_api_set_mesh_shadows(g: &mut crate::game::Game, m: Option<&crate::render_mesh_shadows::MeshShadows>) { todo!() }

/// (src/host/api.ts:91 HostApi.setRigFit)
pub fn host_api_set_rig_fit(g: &mut crate::game::Game, fit: Option<&crate::render_sdfrig::RigFit>) { todo!() }

/// (src/host/api.ts:92 HostApi.setParticles)
pub fn host_api_set_particles(g: &mut crate::game::Game) { todo!() }

/// (src/host/api.ts:93 HostApi.setDecals)
pub fn host_api_set_decals(g: &mut crate::game::Game) { todo!() }

/// (src/host/api.ts:103 HostApi.probe)
pub fn host_api_probe(g: &mut crate::game::Game) { todo!() }
