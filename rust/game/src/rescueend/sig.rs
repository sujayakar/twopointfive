#![allow(unused_variables)]
//! Every function and method of `rescueend`, with an empty body. Written by the skeleton's program.
#[allow(unused_imports)]
use crate::{ids::*, js::*, Quat, V3};
#[allow(unused_imports)]
use alloc::{string::String, vec::Vec};

/// (src/game/rescueend.ts:42 flat)
pub fn flat(a: V3, b: V3) -> f64 { todo!() }

/// (src/game/rescueend.ts:44 who)
pub fn who(g: &mut crate::game::Game) -> PlayerId { todo!() }

/// (src/game/rescueend.ts:55 RescueEnd.constructor)
pub fn rescue_end_new(g: &mut crate::game::Game) -> crate::rescueend::RescueEnd { todo!() }

/// (src/game/rescueend.ts:58 RescueEnd.reset)
pub fn rescue_end_reset(g: &mut crate::game::Game) { todo!() }

/// (src/game/rescueend.ts:60 RescueEnd.out)
pub fn rescue_end_out(g: &mut crate::game::Game) -> Option<V3> { todo!() }

/// (src/game/rescueend.ts:63 RescueEnd.left)
pub fn rescue_end_left(g: &mut crate::game::Game) -> Vec<GuardId> { todo!() }

/// (src/game/rescueend.ts:64 RescueEnd.alive)
pub fn rescue_end_alive(g: &mut crate::game::Game) -> Vec<GuardId> { todo!() }

/// (src/game/rescueend.ts:66 RescueEnd.free)
pub fn rescue_end_free(g: &mut crate::game::Game) -> bool { todo!() }

/// (src/game/rescueend.ts:69 RescueEnd.rows)
pub fn rescue_end_rows(g: &mut crate::game::Game, p: V3, busy: bool, row: &mut dyn FnMut(&mut crate::game::Game, crate::game::Interactable, crate::game::Verb, String, Option<String>, Option<crate::rescueend::RescueEndRowsRowOver>) -> crate::game::Interactable, offer: &mut dyn FnMut(&mut crate::game::Game, crate::game::Interactable, Vec<crate::game::Interactable>)) { todo!() }

/// (src/game/rescueend.ts:82 RescueEnd.word)
pub fn rescue_end_word(g: &mut crate::game::Game, by: Option<CharId>) -> bool { todo!() }

/// (src/game/rescueend.ts:91 RescueEnd.tick)
pub fn rescue_end_tick(g: &mut crate::game::Game) { todo!() }

/// (src/game/rescueend.ts:98 RescueEnd.won)
pub fn rescue_end_won(g: &mut crate::game::Game) -> Option<crate::rescueend::RescueEndWon> { todo!() }

/// (src/game/rescueend.ts:107 RescueEnd.objective)
pub fn rescue_end_objective(g: &mut crate::game::Game) -> crate::rescueend::RescueEndObjective { todo!() }

/// (src/game/rescueend.ts:115 RescueEnd.snapshot)
pub fn rescue_end_snapshot(g: &mut crate::game::Game) -> crate::rescueend::RescueEndSnap { todo!() }

/// (src/game/rescueend.ts:116 RescueEnd.restore)
pub fn rescue_end_restore(g: &mut crate::game::Game, s: crate::rescueend::RescueEndSnap) { todo!() }

/// (src/game/rescueend.ts:124 rescueEndRows)
pub fn rescue_end_rows_2(g: &mut crate::game::Game, p: V3, busy: bool, row: &mut dyn FnMut(&mut crate::game::Game, crate::game::Interactable, crate::game::Verb, String, Option<String>, Option<crate::rescueend::RescueEndRows2RowOver>) -> crate::game::Interactable, offer: &mut dyn FnMut(&mut crate::game::Game, crate::game::Interactable, Vec<crate::game::Interactable>)) { todo!() }
