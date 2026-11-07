#![allow(unused_variables)]
//! Every function and method of `snatch`, with an empty body. Written by the skeleton's program.
#[allow(unused_imports)]
use crate::{ids::*, js::*, Quat, V3};
#[allow(unused_imports)]
use alloc::{string::String, vec::Vec};

/// (src/game/snatch.ts:73 Snatch.constructor)
pub fn snatch_new(g: &mut crate::game::Game) -> crate::snatch::Snatch { todo!() }

/// (src/game/snatch.ts:76 Snatch.block)
pub fn snatch_block(g: &mut crate::game::Game) -> Option<crate::scene_scenarios::SnatchData> { todo!() }

/// (src/game/snatch.ts:78 Snatch.owes)
pub fn snatch_owes(g: &mut crate::game::Game) -> bool { todo!() }

/// (src/game/snatch.ts:79 Snatch.data)
pub fn snatch_data(g: &mut crate::game::Game) -> Option<crate::scene_scenarios::SnatchData> { todo!() }

/// (src/game/snatch.ts:80 Snatch.active)
pub fn snatch_active(g: &mut crate::game::Game) -> bool { todo!() }

/// (src/game/snatch.ts:81 Snatch.isMan)
pub fn snatch_is_man(g: &mut crate::game::Game, gd: GuardId) -> bool { todo!() }

/// (src/game/snatch.ts:83 Snatch.isTied)
pub fn snatch_is_tied(g: &mut crate::game::Game, gd: GuardId) -> bool { todo!() }

/// (src/game/snatch.ts:85 Snatch.inHand)
pub fn snatch_in_hand(g: &mut crate::game::Game) -> bool { todo!() }

/// (src/game/snatch.ts:88 Snatch.voice)
pub fn snatch_voice(g: &mut crate::game::Game, gd: GuardId, text: &str) -> Option<String> { todo!() }

/// (src/game/snatch.ts:95 Snatch.roundOn)
pub fn snatch_round_on(g: &mut crate::game::Game, gd: GuardId, shooter: Option<GuardId>) { todo!() }

/// (src/game/snatch.ts:98 Snatch.teamOf)
pub fn snatch_team_of(g: &mut crate::game::Game) -> Vec<crate::snatch::SnatchTeamOfItem> { todo!() }

/// (src/game/snatch.ts:106 Snatch.note)
pub fn snatch_note(g: &mut crate::game::Game, what: &str) { todo!() }

/// (src/game/snatch.ts:108 Snatch.stage)
pub fn snatch_stage(g: &mut crate::game::Game, n: &crate::scene_scenarios::SnatchData) { todo!() }

/// (src/game/snatch.ts:116 Snatch.drop)
pub fn snatch_drop(g: &mut crate::game::Game) { todo!() }

/// (src/game/snatch.ts:118 Snatch.tick)
pub fn snatch_tick(g: &mut crate::game::Game, dt: f64) { todo!() }

/// (src/game/snatch.ts:185 Snatch.giveUp)
pub fn snatch_give_up(g: &mut crate::game::Game, to: CharId, how: &str) { todo!() }

/// (src/game/snatch.ts:193 Snatch.tie)
pub fn snatch_tie(g: &mut crate::game::Game, by: CharId) { todo!() }

/// (src/game/snatch.ts:202 Snatch.win)
pub fn snatch_win(g: &mut crate::game::Game, why: &str) { todo!() }

/// (src/game/snatch.ts:206 Snatch.lose)
pub fn snatch_lose(g: &mut crate::game::Game, how: crate::snatch::SnatchLoseHow, why: &str) { todo!() }

/// (src/game/snatch.ts:216 Snatch.objective)
pub fn snatch_objective(g: &mut crate::game::Game) -> Option<crate::snatch::SnatchObjective> { todo!() }

/// (src/game/snatch.ts:227 Snatch.summary)
pub fn snatch_summary(g: &mut crate::game::Game) -> String { todo!() }

/// (src/game/snatch.ts:232 Snatch.snapshot)
pub fn snatch_snapshot(g: &mut crate::game::Game) -> Option<crate::snatch::SnatchSnap> { todo!() }

/// (src/game/snatch.ts:241 Snatch.restore)
pub fn snatch_restore(g: &mut crate::game::Game, s: Option<&crate::snatch::SnatchSnap>) { todo!() }

/// (src/game/snatch.ts:253 checkSnatch)
pub fn check_snatch(g: &mut crate::game::Game) -> Vec<String> { todo!() }
