#![allow(unused_imports, unused_variables)]
#![allow(unused_variables)]
pub use ::game_types::host_api::types;
pub use types::*;
use crate::game::Game;

/// (src/host/api.ts:79 HostApi.restartPicture)
pub fn host_api_restart_picture(g: &mut crate::game::Game, why: &str) { g.out.push(crate::out_events::OutEvent::HostApiRestartPicture(crate::out_events::OutHostApiRestartPicture {  })); }

/// BOUNDARY NO-OP (the picture's / the host's): (src/host/api.ts:75 HostApi.invalidateWorldCache)
pub fn host_api_invalidate_world_cache(g: &crate::game::Game) {}
