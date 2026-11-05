#![allow(unused_variables)]
//! Every function and method of `nightbreach`, with an empty body. Written by the skeleton's program.
#[allow(unused_imports)]
use crate::{ids::*, js::*, Quat, V3};
#[allow(unused_imports)]
use alloc::{string::String, vec::Vec};

/// (src/game/nightbreach.ts:111 blindTurn)
pub fn blind_turn(g: &mut crate::game::Game, h: GuardId, k: f64, b: &mut crate::nightbreach::BlindBurst, door: V3) -> bool { todo!() }

/// (src/game/nightbreach.ts:152 NightBreach.drillOps)
pub fn night_breach_drill_ops(g: &mut crate::game::Game) -> Vec<GuardId> { todo!() }

/// (src/game/nightbreach.ts:173 NightBreach.constructor)
pub fn night_breach_new(g: &mut crate::game::Game) -> crate::nightbreach::NightBreach { todo!() }

/// (src/game/nightbreach.ts:176 NightBreach.teamOf)
pub fn night_breach_team_of(g: &mut crate::game::Game, layout: Option<&crate::nightbreach::NightBreachLayout>) -> Option<crate::scene_scenarios::TeamStart> { todo!() }

/// (src/game/nightbreach.ts:179 NightBreach.applyTeam)
pub fn night_breach_apply_team(g: &mut crate::game::Game, layout: Option<&crate::nightbreach::NightBreachLayout>) { todo!() }

/// (src/game/nightbreach.ts:181 NightBreach.canStart)
pub fn night_breach_can_start(g: &mut crate::game::Game, layout: Option<&crate::nightbreach::NightBreachLayout>) -> Option<String> { todo!() }

/// (src/game/nightbreach.ts:196 NightBreach.start)
pub fn night_breach_start(g: &mut crate::game::Game, layout: Option<&crate::nightbreach::NightBreachLayout>) -> Option<String> { todo!() }

/// (src/game/nightbreach.ts:207 NightBreach.stop)
pub fn night_breach_stop(g: &mut crate::game::Game) { todo!() }

/// (src/game/nightbreach.ts:220 NightBreach.adopt)
pub fn night_breach_adopt(g: &mut crate::game::Game, gd: GuardId, mark: &crate::nightbreach::NightBreachMark) { todo!() }

/// (src/game/nightbreach.ts:222 NightBreach.restage)
pub fn night_breach_restage(g: &mut crate::game::Game) { todo!() }

/// (src/game/nightbreach.ts:225 NightBreach.onEncounterRestart)
pub fn night_breach_on_encounter_restart(g: &mut crate::game::Game) { todo!() }

/// (src/game/nightbreach.ts:233 NightBreach.bench)
pub fn night_breach_bench(g: &mut crate::game::Game) { todo!() }

/// (src/game/nightbreach.ts:241 NightBreach.stage)
pub fn night_breach_stage(g: &mut crate::game::Game) { todo!() }

/// (src/game/nightbreach.ts:287 NightBreach.standBy)
pub fn night_breach_stand_by(g: &mut crate::game::Game, lead: V3, taken: &[V3], yaw: Option<f64>) -> V3 { todo!() }

/// (src/game/nightbreach.ts:296 NightBreach.note)
pub fn night_breach_note(g: &mut crate::game::Game, what: &str) { todo!() }

/// (src/game/nightbreach.ts:297 NightBreach.once)
pub fn night_breach_once(g: &mut crate::game::Game, key: &str, text: &str) { todo!() }

/// (src/game/nightbreach.ts:300 NightBreach.order)
pub fn night_breach_order(g: &mut crate::game::Game) -> Option<crate::nightbreach::NightBreachOrder> { todo!() }

/// (src/game/nightbreach.ts:312 NightBreach.ownsPair)
pub fn night_breach_owns_pair(g: &mut crate::game::Game) -> bool { todo!() }

/// (src/game/nightbreach.ts:316 NightBreach.callOff)
pub fn night_breach_call_off(g: &mut crate::game::Game) -> Option<String> { todo!() }

/// (src/game/nightbreach.ts:330 NightBreach.tick)
pub fn night_breach_tick(g: &mut crate::game::Game, dt: f64, input: InputId, cam: Option<CameraId>) { todo!() }

/// (src/game/nightbreach.ts:370 NightBreach.blindFire)
pub fn night_breach_blind_fire(g: &mut crate::game::Game) { todo!() }

/// (src/game/nightbreach.ts:378 NightBreach.objective)
pub fn night_breach_objective(g: &mut crate::game::Game) -> Option<crate::nightbreach::NightBreachObjective> { todo!() }

/// (src/game/nightbreach.ts:389 NightBreach.state)
pub fn night_breach_state(g: &mut crate::game::Game) -> crate::nightbreach::NightBreachState { todo!() }

/// (src/game/nightbreach.ts:400 NightBreach.summary)
pub fn night_breach_summary(g: &mut crate::game::Game) -> String { todo!() }
