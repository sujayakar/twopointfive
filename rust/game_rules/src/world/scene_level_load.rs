pub use ::game_types::scene_level_load::types;
pub use types::*;
use alloc::string::String;

/// The name of the file the game's level came from, None for a level nobody loaded from data.
/// A game has one level, so the module holds its one name.
pub fn level_name_of(g: &crate::game::Game) -> Option<String> {
    g.m.scene_level_load.level_names.clone()
}
