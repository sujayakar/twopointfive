#![allow(unused_variables)]
//! Every function and method of `workstow`, with an empty body. Written by the skeleton's program.
#[allow(unused_imports)]
use crate::{ids::*, js::*, Quat, V3};
#[allow(unused_imports)]
use alloc::{string::String, vec::Vec};

/// (src/game/workstow.ts:39 live)
pub fn live(g: &mut crate::game::Game) -> bool { todo!() }

/// (src/game/workstow.ts:45 samJob)
pub fn sam_job(g: &mut crate::game::Game) -> bool { todo!() }

/// (src/game/workstow.ts:48 samStowTick)
pub fn sam_stow_tick(g: &mut crate::game::Game, holster: &mut dyn FnMut(&mut crate::game::Game, bool)) { todo!() }

/// (src/game/workstow.ts:63 samStowing)
pub fn sam_stowing(g: &mut crate::game::Game) -> bool { todo!() }

/// (src/game/workstow.ts:67 mateStowLead)
pub fn mate_stow_lead(g: &mut crate::game::Game) -> f64 { todo!() }

/// (src/game/workstow.ts:69 mateStow)
pub fn mate_stow(g: &mut crate::game::Game, gd: GuardId) { todo!() }

/// (src/game/workstow.ts:75 mateStowTick)
pub fn mate_stow_tick(g: &mut crate::game::Game, gd: GuardId) { todo!() }

/// (src/game/workstow.ts:86 stowHang)
pub fn stow_hang(g: &mut crate::game::Game, gd: GuardId) -> bool { todo!() }

/// (src/game/workstow.ts:88 stowFired)
pub fn stow_fired(g: &mut crate::game::Game, gd: GuardId) { todo!() }

/// (src/game/workstow.ts:91 stowFight)
pub fn stow_fight(g: &mut crate::game::Game, gd: GuardId, a: Option<crate::workstow::StowFightA>, op: &mut crate::workstow::StowFightOp) -> bool { todo!() }
