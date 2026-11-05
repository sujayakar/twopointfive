#![allow(unused_variables)]
//! Every function and method of `levelfit`, with an empty body. Written by the skeleton's program.
#[allow(unused_imports)]
use crate::{ids::*, js::*, Quat, V3};
#[allow(unused_imports)]
use alloc::{string::String, vec::Vec};

/// (src/game/levelfit.ts:46 menMost)
pub fn men_most(g: &mut crate::game::Game, civilians: f64) -> f64 { todo!() }

/// (src/game/levelfit.ts:49 menHeld)
pub fn men_held(g: &mut crate::game::Game) -> f64 { todo!() }

/// (src/game/levelfit.ts:52 civMost)
pub fn civ_most(g: &mut crate::game::Game) -> f64 { todo!() }

/// (src/game/levelfit.ts:55 handOverMost)
pub fn hand_over_most(g: &mut crate::game::Game, civilians: f64) -> crate::levelfit::HandOverMost { todo!() }

/// (src/game/levelfit.ts:62 levelCaps)
pub fn level_caps() -> crate::levelfit::LevelFit { todo!() }

/// (src/game/levelfit.ts:63 levelCounts)
pub fn level_counts(g: &mut crate::game::Game, level: &crate::levelfit::LevelCountsLevel) -> crate::levelfit::LevelFit { todo!() }

/// (src/game/levelfit.ts:74 levelTooBig)
pub fn level_too_big(g: &mut crate::game::Game, why: &str) -> String { todo!() }

/// (src/game/levelfit.ts:76 flatPanel)
pub fn flat_panel(group: &str, h: &[f64]) -> Option<crate::levelfit::FlatPanel> { todo!() }

/// (src/game/levelfit.ts:83 levelOver)
pub fn level_over(g: &mut crate::game::Game, level: &crate::levelfit::LevelCountsLevel) -> Option<String> { todo!() }

/// (src/game/levelfit.ts:102 cutStatics)
pub fn cut_statics(g: &mut crate::game::Game, boxes: &[BoxId]) -> f64 { todo!() }

/// (src/game/levelfit.ts:103 breakRoom)
pub fn break_room(g: &mut crate::game::Game, col: &crate::levelfit::StoodCol, tables: &JsMap<String, f64>) -> crate::levelfit::BreakRoom { todo!() }

/// (src/game/levelfit.ts:123 fitRows)
pub fn fit_rows(g: &mut crate::game::Game, g_2: &crate::levelfit::Stood, tables: &JsMap<String, f64>) -> Vec<crate::levelfit::FitRow> { todo!() }

/// (src/game/levelfit.ts:162 clumpsMost)
pub fn clumps_most(g: &mut crate::game::Game, boxes: &[BoxId]) -> f64 { todo!() }

/// (src/game/levelfit.ts:166 stoodOver)
pub fn stood_over(g: &mut crate::game::Game, g_2: &crate::levelfit::Stood, tables: &JsMap<String, f64>) -> Option<String> { todo!() }
