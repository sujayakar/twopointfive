//! THE BRING-UP'S OWN PART of host_api: bodies for skeleton stubs the first tick reached (bringup-log.md says which is a boundary no-op and which is translated)
#![allow(unused_imports, unused_variables)]
use super::*;                      // your own module: its types, its other parts' functions
use crate::game::Game;
use crate::ids::*;
use crate::js::{self, JsMap, JsSet};
use crate::m;
use crate::v3::{self, Quat, V3};
use alloc::{boxed::Box, string::String, vec, vec::Vec};

/// BOUNDARY NO-OP (the picture's / the host's): (src/host/api.ts:75 HostApi.invalidateWorldCache)
pub fn host_api_invalidate_world_cache(g: &mut crate::game::Game) {}

/// BOUNDARY NO-OP (the picture's / the host's): (src/host/api.ts:78 HostApi.restartPicture)
pub fn host_api_restart_picture(g: &mut crate::game::Game, why: &str) {}

/// BOUNDARY NO-OP (the picture's / the host's): (src/host/api.ts:83 HostApi.setMeshShadows)
pub fn host_api_set_mesh_shadows(g: &mut crate::game::Game, m: Option<&crate::render_mesh_shadows::MeshShadows>) {}

/// BOUNDARY NO-OP (the picture's / the host's): (src/host/api.ts:85 HostApi.setRigFit)
pub fn host_api_set_rig_fit(g: &mut crate::game::Game, fit: Option<crate::render_sdfrig::RigFit>) {}

/// BOUNDARY NO-OP (the picture's / the host's): (src/host/api.ts:86 HostApi.setParticles)
pub fn host_api_set_particles(g: &mut crate::game::Game, p: Option<&crate::render_particles::Particles>) {}

/// BOUNDARY NO-OP (the picture's / the host's): (src/host/api.ts:87 HostApi.setDecals)
pub fn host_api_set_decals(g: &mut crate::game::Game) {}

/// BOUNDARY NO-OP (the picture's / the host's): (src/host/api.ts:97 HostApi.probe)
pub fn host_api_probe(g: &mut crate::game::Game) {}
