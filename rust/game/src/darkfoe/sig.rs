#![allow(unused_variables)]
//! Every function and method of `darkfoe`, with an empty body. Written by the skeleton's program.
#[allow(unused_imports)]
use crate::{ids::*, js::*, Quat, V3};
#[allow(unused_imports)]
use alloc::{string::String, vec::Vec};

/// (src/game/darkfoe.ts:79 frac)
pub fn frac(x: f64) -> f64 { todo!() }

/// (src/game/darkfoe.ts:81 idShare)
pub fn id_share(g: &mut crate::game::Game, gd: GuardId, salt: Option<f64>) -> f64 { todo!() }

/// (src/game/darkfoe.ts:83 darkFoeCarries)
pub fn dark_foe_carries(g: &mut crate::game::Game, gd: GuardId) -> bool { todo!() }

/// (src/game/darkfoe.ts:85 darkFoeRuled)
pub fn dark_foe_ruled(g: &mut crate::game::Game, gd: GuardId) -> bool { todo!() }

/// (src/game/darkfoe.ts:91 darkFoeLit)
pub fn dark_foe_lit(g: &mut crate::game::Game, gd: GuardId) -> bool { todo!() }

/// (src/game/darkfoe.ts:93 mine)
pub fn mine(g: &mut crate::game::Game, gd: GuardId) { todo!() }

/// (src/game/darkfoe.ts:94 placeOnly)
pub fn place_only() -> bool { todo!() }

/// (src/game/darkfoe.ts:96 floorLight)
pub fn floor_light(g: &mut crate::game::Game, x: f64, z: f64, skip: &mut dyn FnMut(&mut crate::game::Game, f64) -> bool) -> f64 { todo!() }

/// (src/game/darkfoe.ts:100 darkFoeBlind)
pub fn dark_foe_blind(g: &mut crate::game::Game, gd: GuardId) -> bool { todo!() }

/// (src/game/darkfoe.ts:110 learns)
pub fn learns(g: &mut crate::game::Game, gd: GuardId) -> bool { todo!() }

/// (src/game/darkfoe.ts:115 hasWord)
pub fn has_word(g: &mut crate::game::Game, gd: GuardId) -> bool { todo!() }

/// (src/game/darkfoe.ts:126 click)
pub fn click(g: &mut crate::game::Game, gd: GuardId, on: bool) { todo!() }

/// (src/game/darkfoe.ts:129 litInView)
pub fn lit_in_view(g: &mut crate::game::Game, p: V3) -> bool { todo!() }

/// (src/game/darkfoe.ts:137 darkFoeSees)
pub fn dark_foe_sees(g: &mut crate::game::Game, p: V3, edge: Option<bool>) -> bool { todo!() }

/// (src/game/darkfoe.ts:141 inRoom)
pub fn in_room(g: &mut crate::game::Game, d: RoomfightDefId, p: V3) -> bool { todo!() }

/// (src/game/darkfoe.ts:143 threatOf)
pub fn threat_of(g: &mut crate::game::Game, d: RoomfightDefId) -> V3 { todo!() }

/// (src/game/darkfoe.ts:151 fenced)
pub fn fenced(g: &mut crate::game::Game, gd: GuardId) -> bool { todo!() }

/// (src/game/darkfoe.ts:153 wayTo)
pub fn way_to(g: &mut crate::game::Game, d: RoomfightDefId, q: V3) -> bool { todo!() }

/// (src/game/darkfoe.ts:162 darkFoeSpot)
pub fn dark_foe_spot(g: &mut crate::game::Game, d: RoomfightDefId) -> Option<crate::darkfoe::DarkFoeSpot> { todo!() }

/// (src/game/darkfoe.ts:182 walk)
pub fn walk(g: &mut crate::game::Game, d: RoomfightDefId, goal: V3) { todo!() }

/// (src/game/darkfoe.ts:189 darkFoeKept)
pub fn dark_foe_kept(g: &mut crate::game::Game, gd: GuardId, d: Option<RoomfightDefId>) -> Option<String> { todo!() }

/// (src/game/darkfoe.ts:207 darkFoeTick)
pub fn dark_foe_tick(g: &mut crate::game::Game, dt: f64) { todo!() }

/// (src/game/darkfoe.ts:236 darkFoeOf)
pub fn dark_foe_of(g: &mut crate::game::Game, gd: GuardId) -> Option<crate::darkfoe::DarkFoeOf> { todo!() }

/// (src/game/darkfoe.ts:238 darkFoeSummary)
pub fn dark_foe_summary(g: &mut crate::game::Game) -> String { todo!() }
