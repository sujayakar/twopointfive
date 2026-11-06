#![allow(unused_variables)]
//! Every function and method of `gunslots`, with an empty body. Written by the skeleton's program.
#[allow(unused_imports)]
use crate::{ids::*, js::*, Quat, V3};
#[allow(unused_imports)]
use alloc::{string::String, vec::Vec};

/// (src/game/gunslots.ts:34 gunsBroken)
pub fn guns_broken() -> bool { todo!() }

/// (src/game/gunslots.ts:36 quietHeard)
pub fn quiet_heard(g: &mut crate::game::Game) -> bool { todo!() }

/// (src/game/gunslots.ts:58 gunPick)
pub fn gun_pick(id_2: &str) -> Option<crate::gunslots::GunPick> { todo!() }

/// (src/game/gunslots.ts:60 gunPicks)
pub fn gun_picks(g: &mut crate::game::Game, slot: crate::gunslots::GunSlot) -> Vec<String> { todo!() }

/// (src/game/gunslots.ts:62 pickRow)
pub fn pick_row(g: &mut crate::game::Game, id_2: &str, suppressed: bool) -> Option<crate::weapons::WeaponId> { todo!() }

/// (src/game/gunslots.ts:64 pickWord)
pub fn pick_word(g: &mut crate::game::Game, id_2: &str, suppressed: Option<bool>) -> String { todo!() }

/// (src/game/gunslots.ts:66 pickIcon)
pub fn pick_icon(g: &mut crate::game::Game, id_2: &str) -> Option<crate::kit::KitWeapon> { todo!() }

/// (src/game/gunslots.ts:70 sideStage)
pub fn side_stage(g: &mut crate::game::Game, gd: GuardId, row: Option<crate::weapons::WeaponId>) { todo!() }

/// (src/game/gunslots.ts:76 swap)
pub fn swap(g: &mut crate::game::Game, gd: GuardId) { todo!() }

/// (src/game/gunslots.ts:82 sideOutAlert)
pub fn side_out_alert(g: &mut crate::game::Game, gd: GuardId) -> bool { todo!() }

/// (src/game/gunslots.ts:85 sideOut)
pub fn side_out(g: &mut crate::game::Game, gd: GuardId, dist: f64) -> bool { todo!() }

/// (src/game/gunslots.ts:93 sideSeen)
pub fn side_seen(g: &mut crate::game::Game, gd: GuardId) { todo!() }

/// (src/game/gunslots.ts:96 sideLocked)
pub fn side_locked(g: &mut crate::game::Game, gd: GuardId) -> bool { todo!() }

/// (src/game/gunslots.ts:98 sideHung)
pub fn side_hung(g: &mut crate::game::Game, gd: GuardId) -> Option<crate::gunslots::SideHung> { todo!() }

/// (src/game/gunslots.ts:101 sideTick)
pub fn side_tick(g: &mut crate::game::Game, gd: GuardId, dt: Option<f64>) { todo!() }
