#![allow(unused_variables)]
//! Every function and method of `sim_fallshome`, with an empty body. Written by the skeleton's program.
#[allow(unused_imports)]
use crate::{ids::*, js::*, Quat, V3};
#[allow(unused_imports)]
use alloc::{string::String, vec::Vec};

/// (src/sim/fallshome.ts:52 FallStore.constructor)
pub fn fall_store_new(g: &mut crate::game::Game, home: crate::Opaque) -> crate::sim_fallshome::FallStore { todo!() }

/// (src/sim/fallshome.ts:53 FallStore.d)
pub fn fall_store_d(g: &mut crate::game::Game) -> Vec<f64> { todo!() }

/// (src/sim/fallshome.ts:57 FallStore.waiting)
pub fn fall_store_waiting(g: &mut crate::game::Game) -> f64 { todo!() }

/// (src/sim/fallshome.ts:59 FallStore.waitingIn)
pub fn fall_store_waiting_in(g: &mut crate::game::Game, l: f64) -> f64 { todo!() }

/// (src/sim/fallshome.ts:61 FallStore.any)
pub fn fall_store_any(g: &mut crate::game::Game) -> bool { todo!() }

/// (src/sim/fallshome.ts:63 FallStore.at)
pub fn fall_store_at(g: &mut crate::game::Game, x: f64, z: f64) -> f64 { todo!() }

/// (src/sim/fallshome.ts:65 FallStore.cells)
pub fn fall_store_cells(g: &mut crate::game::Game) -> Vec<[f64; 3]> { todo!() }

/// (src/sim/fallshome.ts:67 FallStore.landing)
pub fn fall_store_landing(g: &mut crate::game::Game, i: f64, l: Option<f64>) -> JsMap<crate::sim_fallshome::FallStoreLandingKey, f64> { todo!() }

/// (src/sim/fallshome.ts:71 FallStore.schedule)
pub fn fall_store_schedule(g: &mut crate::game::Game, t: f64, x: f64, y: f64, z: f64, name: crate::sim_fallshome::FallSound, gain: f64, rate: f64, amount: f64, who: f64, event: f64, level: f64, cap: f64, l: Option<f64>, dust: Option<f64>) -> bool { todo!() }

/// (src/sim/fallshome.ts:76 FallStore.deposit)
pub fn fall_store_deposit(g: &mut crate::game::Game, x: f64, z: f64, amount: f64) { todo!() }

/// (src/sim/fallshome.ts:78 FallStore.due)
pub fn fall_store_due(g: &mut crate::game::Game, time: f64, level: f64, gap: f64, l: Option<f64>) -> Vec<crate::sim_fallshome::FallDue> { todo!() }

/// (src/sim/fallshome.ts:86 FallStore.step)
pub fn fall_store_step(g: &mut crate::game::Game, x: f64, z: f64, level: f64, crunch_full: f64, grind: f64, gain: f64, add: f64) -> crate::sim_fallshome::FallStoreStep { todo!() }

/// (src/sim/fallshome.ts:93 FallStore.heard)
pub fn fall_store_heard(g: &mut crate::game::Game, time: f64, gap: f64, l: Option<f64>) -> bool { todo!() }

/// (src/sim/fallshome.ts:96 FallStore.clear)
pub fn fall_store_clear(g: &mut crate::game::Game, list_only: Option<bool>, l: Option<f64>) { todo!() }

/// (src/sim/fallshome.ts:102 FallStore.rule)
pub fn fall_store_rule<T: Clone + core::fmt::Debug>(g: &mut crate::game::Game, what: &str, there: &mut dyn FnMut(&mut crate::game::Game) -> T, here: &mut dyn FnMut(&mut crate::game::Game) -> T) -> T { todo!() }

/// (src/sim/fallshome.ts:114 nanSafe)
pub fn nan_safe(_k: &str, v: crate::Opaque) -> crate::Opaque { todo!() }

/// (src/sim/fallshome.ts:117 cell)
pub fn cell(x: f64, z: f64) -> f64 { todo!() }

/// (src/sim/fallshome.ts:118 depositHere) TWIN → sim::falls::deposit(&mut state(g).falls, x, z, amount)
pub fn deposit_here(d: &mut Vec<f64>, o: f64, x: f64, z: f64, amount: f64) { todo!() }

/// (src/sim/fallshome.ts:123 scheduleHere) TWIN → sim::falls::schedule(g, l, t, x, y, z, name, gain, rate, amount, who, event, level, dust, cap) -> u32
pub fn schedule_here(d: &mut Vec<f64>, o: f64, l: f64, t: f64, x: f64, y: f64, z: f64, name: f64, gain: f64, rate: f64, amount: f64, who: f64, event: f64, level: f64, dust: f64, cap: f64) -> bool { todo!() }

/// (src/sim/fallshome.ts:134 dueHere) TWIN → sim::falls::due(g, l, time, level, gap, out: &mut [f64]) -> u32
pub fn due_here(d: &mut Vec<f64>, o: f64, l: f64, time: f64, level: f64, gap: f64) -> Vec<crate::sim_fallshome::FallDue> { todo!() }

/// (src/sim/fallshome.ts:148 stepHere) TWIN → sim::falls::step(g, x, z, level, crunch_full, grind, gain, add, k_out: &mut f64) -> f64
pub fn step_here(d: &mut Vec<f64>, o: f64, x: f64, z: f64, level: f64, crunch_full: f64, grind: f64, gain: f64, add: f64) -> crate::sim_fallshome::StepHere { todo!() }

/// (src/sim/fallshome.ts:155 clearHere) TWIN → sim::falls::clear(g, l, all: bool, events)
pub fn clear_here(d: &mut Vec<f64>, o: f64, l: f64, list_only: bool) { todo!() }
