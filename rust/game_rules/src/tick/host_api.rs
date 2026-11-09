#![allow(unused_imports, unused_variables)]
#![allow(unused_variables)]
pub use ::game_types::host_api::types;
pub use types::*;
use crate::game::Game;

pub(super) fn host_api_restart_picture(g: &mut crate::game::Game, why: &str) { g.out.push(crate::out_events::OutEvent::HostApiRestartPicture(crate::out_events::OutHostApiRestartPicture {  })); }

/// BOUNDARY NO-OP (the picture's / the host's):
pub fn host_api_invalidate_world_cache(g: &crate::game::Game) {}
