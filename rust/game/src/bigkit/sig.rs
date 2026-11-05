#![allow(unused_variables)]
//! Every function and method of `bigkit`, with an empty body. Written by the skeleton's program.
#[allow(unused_imports)]
use crate::{ids::*, js::*, Quat, V3};
#[allow(unused_imports)]
use alloc::{string::String, vec::Vec};

/// (src/game/bigkit.ts:65 bigBroken)
pub fn big_broken() -> bool { todo!() }

/// (src/game/bigkit.ts:73 samBig)
pub fn sam_big(g: &mut crate::game::Game, d: DoorId, p: Option<V3>) -> Option<crate::bigkit::SamBig> { todo!() }

/// (src/game/bigkit.ts:82 samBigSpot)
pub fn sam_big_spot(g: &mut crate::game::Game, d: DoorId, from: V3, tool: crate::bigkit::SamBigSpotTool) -> Option<crate::bigkit::SamBigSpot> { todo!() }

/// (src/game/bigkit.ts:125 bigSpots)
pub fn big_spots(g: &mut crate::game::Game, d: DoorId, side: f64, tool: crate::bigkit::BigSpotsTool) -> Vec<crate::bigkit::BigSpot> { todo!() }

/// (src/game/bigkit.ts:138 samBigFar)
pub fn sam_big_far(g: &mut crate::game::Game, d: DoorId, p: V3) -> Option<crate::bigkit::SamBigFar> { todo!() }

/// (src/game/bigkit.ts:144 samBigWalk)
pub fn sam_big_walk(g: &mut crate::game::Game, dt: f64) -> f64 { todo!() }

/// (src/game/bigkit.ts:154 samBigPose)
pub fn sam_big_pose(g: &mut crate::game::Game) -> Option<crate::bigkit::SamBigPose> { todo!() }

/// (src/game/bigkit.ts:167 weighed)
pub fn weighed(g: &mut crate::game::Game) -> bool { todo!() }

/// (src/game/bigkit.ts:169 bigWeighed)
pub fn big_weighed(g: &mut crate::game::Game) -> bool { todo!() }

/// (src/game/bigkit.ts:171 bigEquip)
pub fn big_equip(g: &mut crate::game::Game, kind: crate::bigkit::BigEquipKind) -> f64 { todo!() }

/// (src/game/bigkit.ts:172 bigStow)
pub fn big_stow(g: &mut crate::game::Game) -> f64 { todo!() }

/// (src/game/bigkit.ts:173 bigOutBy)
pub fn big_out_by(g: &mut crate::game::Game, kind: crate::bigkit::BigOutByKind) -> f64 { todo!() }

/// (src/game/bigkit.ts:178 samBigLeadOf)
pub fn sam_big_lead_of(g: &mut crate::game::Game) -> f64 { todo!() }

/// (src/game/bigkit.ts:185 samBigCarry)
pub fn sam_big_carry(g: &mut crate::game::Game) -> bool { todo!() }

/// (src/game/bigkit.ts:191 bigWind)
pub fn big_wind(g: &mut crate::game::Game) -> f64 { todo!() }

/// (src/game/bigkit.ts:193 bigSwing)
pub fn big_swing(g: &mut crate::game::Game) -> f64 { todo!() }

/// (src/game/bigkit.ts:197 samBigTail)
pub fn sam_big_tail(g: &mut crate::game::Game) -> bool { todo!() }

/// (src/game/bigkit.ts:199 samBigLeft)
pub fn sam_big_left(g: &mut crate::game::Game) -> f64 { todo!() }

/// (src/game/bigkit.ts:201 samBigEnded)
pub fn sam_big_ended(g: &mut crate::game::Game) { todo!() }

/// (src/game/bigkit.ts:203 samBigDrop)
pub fn sam_big_drop(g: &mut crate::game::Game) { todo!() }

/// (src/game/bigkit.ts:206 samBigTailTick)
pub fn sam_big_tail_tick(g: &mut crate::game::Game, dt: f64) { todo!() }

/// (src/game/bigkit.ts:217 samBigStart)
pub fn sam_big_start(g: &mut crate::game::Game, d: DoorId) { todo!() }

/// (src/game/bigkit.ts:219 samBigTick)
pub fn sam_big_tick(g: &mut crate::game::Game, dt: f64) -> bool { todo!() }

/// (src/game/bigkit.ts:263 hammerBlows)
pub fn hammer_blows(g: &mut crate::game::Game, d: DoorId, p: &[f64]) -> f64 { todo!() }

/// (src/game/bigkit.ts:265 bigSide)
pub fn big_side(g: &mut crate::game::Game, d: DoorId, p: &[f64]) -> f64 { todo!() }

/// (src/game/bigkit.ts:269 bigCarried)
pub fn big_carried(g: &mut crate::game::Game, c_2: crate::bigkit::BigCarriedC) -> Option<crate::bigkit::BigCarried> { todo!() }

/// (src/game/bigkit.ts:278 carriesHammer)
pub fn carries_hammer(g: &mut crate::game::Game, gd: GuardId) -> bool { todo!() }

/// (src/game/bigkit.ts:282 gunUp)
pub fn gun_up(g: &mut crate::game::Game, gd: GuardId) -> bool { todo!() }

/// (src/game/bigkit.ts:293 gunDown)
pub fn gun_down(g: &mut crate::game::Game, gd: GuardId) { todo!() }

/// (src/game/bigkit.ts:301 hammerBlow)
pub fn hammer_blow(g: &mut crate::game::Game, gd: GuardId, d: DoorId, last: bool) -> bool { todo!() }

/// (src/game/bigkit.ts:312 hammerOut)
pub fn hammer_out(g: &mut crate::game::Game, gd: GuardId, at: V3) -> crate::game::GuardBigOut { todo!() }

/// (src/game/bigkit.ts:318 hammerIn)
pub fn hammer_in(g: &mut crate::game::Game, gd: GuardId, hard: Option<bool>) { todo!() }

/// (src/game/bigkit.ts:327 bigFired)
pub fn big_fired(g: &mut crate::game::Game, c_2: CharId) { todo!() }

/// (src/game/bigkit.ts:330 hammerOutTick)
pub fn hammer_out_tick(g: &mut crate::game::Game, mine: &mut dyn FnMut(&mut crate::game::Game, GuardId) -> bool) { todo!() }
