#![allow(unused_variables)]
//! Every function and method of `snatchout`, with an empty body. Written by the skeleton's program.
#[allow(unused_imports)]
use crate::{ids::*, js::*, Quat, V3};
#[allow(unused_imports)]
use alloc::{string::String, vec::Vec};

/// (src/game/snatchout.ts:53 flat)
pub fn flat(a: V3, b: V3) -> f64 { todo!() }

/// (src/game/snatchout.ts:55 who)
pub fn who(g: &mut crate::game::Game) -> PlayerId { todo!() }

/// (src/game/snatchout.ts:75 SnatchOut.constructor)
pub fn snatch_out_new(g: &mut crate::game::Game) -> crate::snatchout::SnatchOut { todo!() }

/// (src/game/snatchout.ts:78 SnatchOut.reset)
pub fn snatch_out_reset(g: &mut crate::game::Game) { todo!() }

/// (src/game/snatchout.ts:80 SnatchOut.van)
pub fn snatch_out_van(g: &mut crate::game::Game) -> Option<V3> { todo!() }

/// (src/game/snatchout.ts:82 SnatchOut.on)
pub fn snatch_out_on(g: &mut crate::game::Game) -> bool { todo!() }

/// (src/game/snatchout.ts:84 SnatchOut.ruled)
pub fn snatch_out_ruled(g: &mut crate::game::Game) -> bool { todo!() }

/// (src/game/snatchout.ts:86 SnatchOut.free)
pub fn snatch_out_free(g: &mut crate::game::Game) -> bool { todo!() }

/// (src/game/snatchout.ts:88 SnatchOut.owns)
pub fn snatch_out_owns(g: &mut crate::game::Game, gd: GuardId) -> bool { todo!() }

/// (src/game/snatchout.ts:90 SnatchOut.planted)
pub fn snatch_out_planted(g: &mut crate::game::Game) -> Vec<GuardId> { todo!() }

/// (src/game/snatchout.ts:93 SnatchOut.rows)
pub fn snatch_out_rows(g: &mut crate::game::Game, p: V3, busy: bool, row: &mut dyn FnMut(&mut crate::game::Game, crate::game::Interactable, crate::game::Verb, String, Option<String>, Option<crate::snatchout::SnatchOutRowsRowOver>) -> crate::game::Interactable, offer: &mut dyn FnMut(&mut crate::game::Game, crate::game::Interactable, Vec<crate::game::Interactable>)) { todo!() }

/// (src/game/snatchout.ts:102 SnatchOut.word)
pub fn snatch_out_word(g: &mut crate::game::Game, by: Option<CharId>) -> bool { todo!() }

/// (src/game/snatchout.ts:114 SnatchOut.tick)
pub fn snatch_out_tick(g: &mut crate::game::Game, dt: f64) -> bool { todo!() }

/// (src/game/snatchout.ts:130 SnatchOut.downed)
pub fn snatch_out_downed(g: &mut crate::game::Game, c_2: CharId) -> bool { todo!() }

/// (src/game/snatchout.ts:132 SnatchOut.switched)
pub fn snatch_out_switched(g: &mut crate::game::Game) { todo!() }

/// (src/game/snatchout.ts:133 SnatchOut.into)
pub fn snatch_out_into(g: &mut crate::game::Game, walked: bool) { todo!() }

/// (src/game/snatchout.ts:143 SnatchOut.fighting)
pub fn snatch_out_fighting(g: &mut crate::game::Game, gd: GuardId) -> bool { todo!() }

/// (src/game/snatchout.ts:145 SnatchOut.mover)
pub fn snatch_out_mover(g: &mut crate::game::Game) -> Option<CharId> { todo!() }

/// (src/game/snatchout.ts:147 SnatchOut.mine)
pub fn snatch_out_mine(g: &mut crate::game::Game, gd: GuardId, spec: &crate::snatchout::SnatchOutMineSpec) -> OrdersAssignmentId { todo!() }

/// (src/game/snatchout.ts:148 SnatchOut.drop)
pub fn snatch_out_drop(g: &mut crate::game::Game, gd: GuardId) { todo!() }

/// (src/game/snatchout.ts:149 SnatchOut.rejoin)
pub fn snatch_out_rejoin(g: &mut crate::game::Game, gd: GuardId) { todo!() }

/// (src/game/snatchout.ts:154 SnatchOut.letGo)
pub fn snatch_out_let_go(g: &mut crate::game::Game) { todo!() }

/// (src/game/snatchout.ts:157 SnatchOut.spot)
pub fn snatch_out_spot(g: &mut crate::game::Game, gd: GuardId, route: &[V3], taken: &[V3], own: bool) -> Option<V3> { todo!() }

/// (src/game/snatchout.ts:173 SnatchOut.plant)
pub fn snatch_out_plant(g: &mut crate::game::Game, men: &[GuardId]) { todo!() }

/// (src/game/snatchout.ts:190 SnatchOut.turn)
pub fn snatch_out_turn(g: &mut crate::game::Game) { todo!() }

/// (src/game/snatchout.ts:202 SnatchOut.bound)
pub fn snatch_out_bound(g: &mut crate::game::Game, dt: f64) { todo!() }

/// (src/game/snatchout.ts:242 SnatchOut.objective)
pub fn snatch_out_objective(g: &mut crate::game::Game) -> Option<crate::snatchout::SnatchOutObjective> { todo!() }

/// (src/game/snatchout.ts:250 SnatchOut.snapshot)
pub fn snatch_out_snapshot(g: &mut crate::game::Game) -> crate::snatchout::SnatchOutSnap { todo!() }

/// (src/game/snatchout.ts:251 SnatchOut.restore)
pub fn snatch_out_restore(g: &mut crate::game::Game, s: Option<crate::snatchout::SnatchOutSnap>) { todo!() }

/// (src/game/snatchout.ts:261 snatchOutRows)
pub fn snatch_out_rows_2(g: &mut crate::game::Game, p: V3, busy: bool, row: &mut dyn FnMut(&mut crate::game::Game, crate::game::Interactable, crate::game::Verb, String, Option<String>, Option<crate::snatchout::SnatchOutRows2RowOver>) -> crate::game::Interactable, offer: &mut dyn FnMut(&mut crate::game::Game, crate::game::Interactable, Vec<crate::game::Interactable>)) { todo!() }
