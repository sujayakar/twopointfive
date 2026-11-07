//! src/scene/scenarioList.ts lines 1 to 278
use super::*;                      // your own module: its types, its other parts' functions
use crate::game::Game;
use crate::ids::*;
use crate::js::{self, JsMap, JsSet};
use crate::m;
use crate::v3::{self, Quat, V3};
use alloc::{boxed::Box, string::String, vec, vec::Vec};

/// How many teammates, in the picker's words: 'you + the pair', 'solo' … (scenarioList.ts teamWords)
pub fn team_words(e: crate::scene_scenario_list::TeamWordsE) -> String { crate::loadout::size_words(e.squad) }


