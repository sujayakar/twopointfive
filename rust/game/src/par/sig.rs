#![allow(unused_variables)]
//! Every function and method of `par`, with an empty body. Written by the skeleton's program.
#[allow(unused_imports)]
use crate::{ids::*, js::*, Quat, V3};
#[allow(unused_imports)]
use alloc::{string::String, vec::Vec};

/// (src/game/par.ts:38 medalOf)
pub fn medal_of(d: crate::scene_scenarios::ParData, total: f64) -> crate::par::Medal { todo!() }

/// (src/game/par.ts:40 parResult)
pub fn par_result(d: crate::scene_scenarios::ParData, secs: f64, lost: f64, of: Option<f64>) -> crate::par::ParResult { todo!() }

/// (src/game/par.ts:49 clock)
pub fn clock(s: f64) -> String { todo!() }

/// (src/game/par.ts:61 Par.constructor)
pub fn par_new(g: &mut crate::game::Game) -> crate::par::Par { todo!() }

/// (src/game/par.ts:64 Par.data)
pub fn par_data(g: &mut crate::game::Game) -> Option<crate::scene_scenarios::ParData> { todo!() }

/// (src/game/par.ts:71 Par.started)
pub fn par_started(g: &mut crate::game::Game) -> bool { todo!() }

/// (src/game/par.ts:73 Par.secs)
pub fn par_secs(g: &mut crate::game::Game) -> f64 { todo!() }

/// (src/game/par.ts:75 Par.lost)
pub fn par_lost(g: &mut crate::game::Game) -> [f64; 2] { todo!() }

/// (src/game/par.ts:77 Par.fired)
pub fn par_fired(g: &mut crate::game::Game) -> f64 { todo!() }

/// (src/game/par.ts:79 Par.result)
pub fn par_result_2(g: &mut crate::game::Game) -> Option<crate::par::ParResult> { todo!() }

/// (src/game/par.ts:81 Par.shown)
pub fn par_shown(g: &mut crate::game::Game) -> Option<String> { todo!() }

/// (src/game/par.ts:87 Par.tick)
pub fn par_tick(g: &mut crate::game::Game) { todo!() }

/// (src/game/par.ts:103 Par.fresh)
pub fn par_fresh(g: &mut crate::game::Game, tail: Option<u32>) { todo!() }

/// (src/game/par.ts:110 Par.objective)
pub fn par_objective(g: &mut crate::game::Game, text: &str) -> String { todo!() }

/// (src/game/par.ts:115 Par.row)
pub fn par_row(g: &mut crate::game::Game) -> String { todo!() }

/// (src/game/par.ts:125 Par.snapshot)
pub fn par_snapshot(g: &mut crate::game::Game) -> Option<crate::par::ParSnap> { todo!() }

/// (src/game/par.ts:128 Par.restore)
pub fn par_restore(g: &mut crate::game::Game, s: Option<crate::par::ParSnap>) { todo!() }

/// (src/game/par.ts:135 Par.note)
pub fn par_note(g: &mut crate::game::Game, s: &str) { todo!() }
