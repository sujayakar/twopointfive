#![allow(unused_variables)]
//! Every function and method of `friendlyfire`, with an empty body. Written by the skeleton's program.
#[allow(unused_imports)]
use crate::{ids::*, js::*, Quat, V3};
#[allow(unused_imports)]
use alloc::{string::String, vec::Vec};

/// (src/game/friendlyfire.ts:53 ffLive)
pub fn ff_live(g: &mut crate::game::Game) -> bool { todo!() }

/// (src/game/friendlyfire.ts:55 ffBody)
pub fn ff_body(g: &mut crate::game::Game, gd: GuardId) -> bool { todo!() }

/// (src/game/friendlyfire.ts:57 ffSide)
pub fn ff_side(g: &mut crate::game::Game, sg: Option<GuardId>) -> bool { todo!() }

/// (src/game/friendlyfire.ts:63 friendInCone)
pub fn friend_in_cone(g: &mut crate::game::Game, self_: GuardId, muzzle: V3, to: V3, spread: f64, def: Option<&crate::weapons::WeaponDef>, on: Option<CharId>) -> Option<crate::friendlyfire::FriendInCone> { todo!() }

/// (src/game/friendlyfire.ts:92 ffUnpressed)
pub fn ff_unpressed(g: &mut crate::game::Game, sg: Option<GuardId>, muzzle: V3, to: V3, spread: f64, def: &crate::weapons::WeaponDef) -> bool { todo!() }

/// (src/game/friendlyfire.ts:98 ffBroken)
pub fn ff_broken() -> bool { todo!() }

/// (src/game/friendlyfire.ts:101 ffPast)
pub fn ff_past(g: &mut crate::game::Game, from: V3, to: V3, into: Option<CharId>) { todo!() }

/// (src/game/friendlyfire.ts:111 ffHeld)
pub fn ff_held(g: &mut crate::game::Game) { todo!() }

/// (src/game/friendlyfire.ts:113 ffNote)
pub fn ff_note(g: &mut crate::game::Game, sg: Option<GuardId>, is_player: bool, into_sam: bool) { todo!() }
