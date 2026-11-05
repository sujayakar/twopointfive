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
/// ROUND THREE (room-clear-four tick 1596: the first thrown thing walking, throwables_walk_there < snow_bog_may asks it). The TypeScript's table is a
/// `WeakMap<Level, string>`; the skeleton keeps `level_names: JsMap<u32, String>` and no number on a Level to ask it with, and the bridge cannot walk a WeakMap, so an
/// imported game holds no name at all. A game has one level: its name is the table's one row.
// SKELETON-GAP: the level's name where the game can ask it by its level (`g.level_name: Option<String>`, or a serial on Level).  BRIDGE-ASK: carry `levelNameOf(g.level)`.
pub fn level_name_of(g: &mut crate::game::Game) -> Option<String> {
    g.m.scene_level_load.level_names.values().next().cloned()
}
