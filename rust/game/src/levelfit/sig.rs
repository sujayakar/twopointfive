#![allow(unused_variables)]
//! Every function and method of `levelfit`, with an empty body. Written by the skeleton's program.
#[allow(unused_imports)]
use crate::{ids::*, js::*, Quat, V3};
#[allow(unused_imports)]
use alloc::{string::String, vec::Vec};

/// (src/game/levelfit.ts:45 menMost)
pub fn men_most(g: &mut crate::game::Game, civilians: f64) -> f64 { todo!() }

/// (src/game/levelfit.ts:48 menHeld)
pub fn men_held(g: &mut crate::game::Game) -> f64 { todo!() }

/// (src/game/levelfit.ts:51 civMost)
pub fn civ_most(g: &mut crate::game::Game) -> f64 { todo!() }

/// (src/game/levelfit.ts:54 handOverMost)
pub fn hand_over_most(g: &mut crate::game::Game, civilians: f64) -> crate::levelfit::HandOverMost { todo!() }

/// (src/game/levelfit.ts:61 levelCaps)
pub fn level_caps() -> crate::levelfit::LevelFit { todo!() }

/// (src/game/levelfit.ts:62 levelCounts)
pub fn level_counts(g: &mut crate::game::Game, level: &crate::levelfit::LevelCountsLevel) -> crate::levelfit::LevelFit { todo!() }

/// (src/game/levelfit.ts:71 levelTooBig)
pub fn level_too_big(g: &mut crate::game::Game, why: &str) -> String { todo!() }

/// (src/game/levelfit.ts:73 levelOver)
pub fn level_over(g: &mut crate::game::Game, level: &crate::levelfit::LevelCountsLevel) -> Option<String> { todo!() }

/// (src/game/levelfit.ts:92 cutStatics)
pub fn cut_statics(g: &mut crate::game::Game, boxes: &[BoxId]) -> f64 { todo!() }

/// (src/game/levelfit.ts:93 breakRoom)
pub fn break_room(g: &mut crate::game::Game, col: &crate::levelfit::StoodCol, tables: &JsMap<String, f64>) -> crate::levelfit::BreakRoom { todo!() }

/// (src/game/levelfit.ts:112 fitRows)
pub fn fit_rows(g: &mut crate::game::Game, g_2: &crate::levelfit::Stood, tables: &JsMap<String, f64>) -> Vec<crate::levelfit::FitRow> { todo!() }

/// (src/game/levelfit.ts:150 stoodOver)
pub fn stood_over(g: &mut crate::game::Game, g_2: &crate::levelfit::Stood, tables: &JsMap<String, f64>) -> Option<String> { todo!() }
