#![allow(unused_variables)]
//! Every function and method of `shutdoor`, with an empty body. Written by the skeleton's program.
#[allow(unused_imports)]
use crate::{ids::*, js::*, Quat, V3};
#[allow(unused_imports)]
use alloc::{string::String, vec::Vec};

/// (src/game/shutdoor.ts:59 flat)
pub fn flat(a: V3, b: V3) -> f64 { todo!() }

/// (src/game/shutdoor.ts:60 ptOf)
pub fn pt_of(g: &mut crate::game::Game, d: DoorId, along: f64, off: f64, side: f64) -> V3 { todo!() }

/// (src/game/shutdoor.ts:61 handleOf)
pub fn handle_of(g: &mut crate::game::Game, d: DoorId) -> V3 { todo!() }

/// (src/game/shutdoor.ts:62 note)
pub fn note(g: &mut crate::game::Game, gd: GuardId, d: DoorId, what: &str) { todo!() }

/// (src/game/shutdoor.ts:65 standsOpen)
pub fn stands_open(g: &mut crate::game::Game, d: DoorId) -> bool { todo!() }

/// (src/game/shutdoor.ts:68 inShutSwing)
pub fn in_shut_swing(g: &mut crate::game::Game, d: DoorId, x: f64, z: f64, pad: Option<f64>) -> bool { todo!() }

/// (src/game/shutdoor.ts:76 walled)
pub fn walled(g: &mut crate::game::Game, a: V3, b: V3) -> bool { todo!() }

/// (src/game/shutdoor.ts:81 stands)
pub fn stands(g: &mut crate::game::Game, p: V3) -> bool { todo!() }

/// (src/game/shutdoor.ts:88 shutSpot)
pub fn shut_spot(g: &mut crate::game::Game, d: DoorId, side: f64) -> Option<crate::shutdoor::Spot> { todo!() }

/// (src/game/shutdoor.ts:104 spotsOf)
pub fn spots_of(g: &mut crate::game::Game, d: DoorId) -> Vec<Option<crate::shutdoor::Spot>> { todo!() }

/// (src/game/shutdoor.ts:110 walk)
pub fn walk(g: &mut crate::game::Game, gd: GuardId, p: V3) -> f64 { todo!() }

/// (src/game/shutdoor.ts:116 fromFile)
pub fn from_file(g: &mut crate::game::Game, gd: GuardId) -> bool { todo!() }

/// (src/game/shutdoor.ts:119 plan)
pub fn plan(g: &mut crate::game::Game, d: DoorId, men: &[GuardId]) -> crate::shutdoor::Plan { todo!() }

/// (src/game/shutdoor.ts:132 inTheWay)
pub fn in_the_way(g: &mut crate::game::Game, d: DoorId, but: Option<GuardId>, seen: Option<bool>) -> Option<String> { todo!() }

/// (src/game/shutdoor.ts:144 contactThrough)
pub fn contact_through(g: &mut crate::game::Game, d: DoorId) -> bool { todo!() }

/// (src/game/shutdoor.ts:153 whyNotShut)
pub fn why_not_shut(g: &mut crate::game::Game, d: DoorId, men: &[GuardId]) -> Option<String> { todo!() }

/// (src/game/shutdoor.ts:163 send)
pub fn send(g: &mut crate::game::Game, gd: GuardId, d: DoorId, spot: crate::shutdoor::Spot) { todo!() }

/// (src/game/shutdoor.ts:169 pull)
pub fn pull(g: &mut crate::game::Game, d: DoorId, gd: GuardId) { todo!() }

/// (src/game/shutdoor.ts:172 finish)
pub fn finish(g: &mut crate::game::Game, gd: GuardId, j: Option<crate::shutdoor::Job>) { todo!() }

/// (src/game/shutdoor.ts:178 tick)
pub fn tick(g: &mut crate::game::Game, gd: GuardId, a: OrdersAssignmentId) { todo!() }

/// (src/game/shutdoor.ts:214 registerShutDoor)
pub fn register_shut_door(g: &mut crate::game::Game) { todo!() }
