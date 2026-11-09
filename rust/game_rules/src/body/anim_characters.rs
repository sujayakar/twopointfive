#![allow(unused_imports, unused_variables)]
pub use ::game_types::anim_characters::types;
pub use types::*;
use crate::game::Game;

/// BOUNDARY NO-OP (the picture's / the host's): (src/anim/characters.ts:108 CharacterRenderer.finish)
pub fn character_renderer_finish(g: &crate::game::Game, count: f64) {}
