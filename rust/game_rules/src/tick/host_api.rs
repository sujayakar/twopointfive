#![allow(unused_imports, unused_variables)]
#![allow(unused_variables)]
pub use ::game_types::host_api::types;
pub use types::*;
use crate::game::Game;

/// BOUNDARY NO-OP (the picture's / the host's):
pub fn host_api_invalidate_world_cache(g: &crate::game::Game) {}
