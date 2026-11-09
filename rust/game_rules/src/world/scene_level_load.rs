#![allow(unused_imports, unused_variables)]
pub use ::game_types::scene_level_load::types;
pub use types::*;
use crate::game::Game;
use alloc::string::String;

/// The name of the file the game's level came from, None for a level nobody loaded from data.   (tools/levels/lib/levelLoad.ts:48 levelNameOf)
/// The TypeScript's table is a `WeakMap<Level, string>`; a game has one level, so the skeleton holds its one name (`level_names: Option<String>`), and the bridge carries it.
pub fn level_name_of(g: &crate::game::Game) -> Option<String> {
    g.m.scene_level_load.level_names.clone()
}
