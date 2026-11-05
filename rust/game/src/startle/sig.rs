#![allow(unused_variables)]
//! Every function and method of `startle`, with an empty body. Written by the skeleton's program.
#[allow(unused_imports)]
use crate::{ids::*, js::*, Quat, V3};
#[allow(unused_imports)]
use alloc::{string::String, vec::Vec};

/// (src/game/startle.ts:66 blank)
pub fn blank() -> crate::startle::St { todo!() }

/// (src/game/startle.ts:71 giveStartle)
pub fn give_startle(g: &mut crate::game::Game, gd: GuardId, at: V3, secs: Option<f64>, torch: Option<bool>) -> bool { todo!() }

/// (src/game/startle.ts:85 startleSight)
pub fn startle_sight(g: &mut crate::game::Game, gd: GuardId, prev: crate::game::GuardState, seen: bool) { todo!() }

/// (src/game/startle.ts:113 startleGunDown)
pub fn startle_gun_down(g: &mut crate::game::Game, gd: GuardId) -> bool { todo!() }

/// (src/game/startle.ts:116 startleTorch)
pub fn startle_torch(g: &mut crate::game::Game, gd: GuardId) -> bool { todo!() }

/// (src/game/startle.ts:118 startled)
pub fn startled(g: &mut crate::game::Game, gd: GuardId) -> bool { todo!() }

/// (src/game/startle.ts:121 startleLeft)
pub fn startle_left(g: &mut crate::game::Game, gd: GuardId) -> f64 { todo!() }

/// (src/game/startle.ts:123 startleRunning)
pub fn startle_running(g: &mut crate::game::Game, gd: GuardId) -> bool { todo!() }

/// (src/game/startle.ts:127 startleScript)
pub fn startle_script(g: &mut crate::game::Game, gd: GuardId) -> Option<crate::game::GuardScript> { todo!() }

/// (src/game/startle.ts:138 choose)
pub fn choose(g: &mut crate::game::Game, gd: GuardId) { todo!() }

/// (src/game/startle.ts:143 runs)
pub fn runs(g: &mut crate::game::Game, gd: GuardId) -> bool { todo!() }


/// INVENTED (conventions 4, an object lent): what run_guard_script left in the script startle_script lent goes back on his row, which startle_script reads `arrived` off next frame (guards.ts:2146; squad.ts:130 writes it; startle.ts:132 reads it). None, or no row: the row stays.
pub fn startle_script_ran(g: &mut crate::game::Game, gd: GuardId, ran: Option<crate::game::GuardScript>) { todo!() }
