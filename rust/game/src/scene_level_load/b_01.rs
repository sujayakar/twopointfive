//! THE BRING-UP'S OWN PART of scene_level_load: bodies for skeleton stubs a tick reached (bringup2-log.md says which is a boundary no-op and which is translated)
#![allow(unused_imports, unused_variables)]
use super::*;                      // your own module: its types, its other parts' functions
use crate::game::Game;
use crate::ids::*;
use crate::js::{self, JsMap, JsSet};
use crate::m;
use crate::v3::{self, Quat, V3};
use alloc::{boxed::Box, string::String, vec, vec::Vec};

/// The name of the file the game's level came from, None for a level nobody loaded from data.   (scene/levelLoad.ts:48 levelNameOf)
/// The TypeScript's table is a `WeakMap<Level, string>`; a game has one level, so the skeleton holds its one name (`level_names: Option<String>`), and the bridge carries it.
pub fn level_name_of(g: &crate::game::Game) -> Option<String> {
    g.m.scene_level_load.level_names.clone()
}
