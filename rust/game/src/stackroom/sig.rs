#![allow(unused_variables)]
//! Every function and method of `stackroom`, with an empty body. Written by the skeleton's program.
#[allow(unused_imports)]
use crate::{ids::*, js::*, Quat, V3};
#[allow(unused_imports)]
use alloc::{string::String, vec::Vec};

/// (src/game/stackroom.ts:92 stackRoomBookKeep)
pub fn stack_room_book_keep(g: &mut crate::game::Game) -> crate::stackroom::StackRoomBookKeep { todo!() }

/// (src/game/stackroom.ts:98 bookOf)
pub fn book_of<'g>(g: &'g mut crate::game::Game) -> &'g mut crate::stackroom::Book { todo!() }

/// (src/game/stackroom.ts:104 centre)
pub fn centre(g: &mut crate::game::Game, c_2: f64) -> V3 { todo!() }

/// (src/game/stackroom.ts:107 doorBetween)
pub fn door_between(g: &mut crate::game::Game, a: V3, b: V3) -> bool { todo!() }

/// (src/game/stackroom.ts:117 floorOf)
pub fn floor_of(g: &mut crate::game::Game, d: DoorId, side: f64) -> Option<crate::stackroom::Floor> { todo!() }

/// (src/game/stackroom.ts:141 stepsTo)
pub fn steps_to(g: &mut crate::game::Game, f: &crate::stackroom::Floor, p: V3) -> f64 { todo!() }

/// (src/game/stackroom.ts:150 onFloor)
pub fn on_floor(g: &mut crate::game::Game, f: &crate::stackroom::Floor, p: V3) -> bool { todo!() }

/// (src/game/stackroom.ts:156 stackRoomHas)
pub fn stack_room_has(g: &mut crate::game::Game, d: DoorId, side: f64, p: V3) -> bool { todo!() }

/// (src/game/stackroom.ts:160 stackRoomKeeps)
pub fn stack_room_keeps(g: &mut crate::game::Game, d: DoorId, side: f64, p: V3) -> bool { todo!() }

/// (src/game/stackroom.ts:167 stoodAt)
pub fn stood_at(g: &mut crate::game::Game, d: DoorId, q: V3) -> bool { todo!() }

/// (src/game/stackroom.ts:174 furnished)
pub fn furnished(g: &mut crate::game::Game, d: DoorId, q: V3) -> bool { todo!() }

/// (src/game/stackroom.ts:180 stackRoomFix)
pub fn stack_room_fix(g: &mut crate::game::Game, d: DoorId, side: f64, posts: Option<&[crate::orders::StackPost]>, taken: &[V3], min_m: Option<f64>) -> Option<Vec<crate::orders::StackPost>> { todo!() }

/// (src/game/stackroom.ts:244 inView)
pub fn in_view(g: &mut crate::game::Game, d: DoorId, far: &[V3], p: V3) -> bool { todo!() }

/// (src/game/stackroom.ts:250 farCells)
pub fn far_cells(g: &mut crate::game::Game, d: DoorId, side: f64) -> Vec<V3> { todo!() }

/// (src/game/stackroom.ts:256 stackViewOpen)
pub fn stack_view_open(g: &mut crate::game::Game, d: DoorId) -> bool { todo!() }

/// (src/game/stackroom.ts:259 stackViewReach)
pub fn stack_view_reach(g: &mut crate::game::Game, d: DoorId) -> f64 { todo!() }

/// (src/game/stackroom.ts:261 stackViewSees)
pub fn stack_view_sees(g: &mut crate::game::Game, d: DoorId, side: f64, p: V3) -> bool { todo!() }

/// (src/game/stackroom.ts:264 stackViewFix)
pub fn stack_view_fix(g: &mut crate::game::Game, d: DoorId, side: f64, posts: Option<&[crate::orders::StackPost]>, taken: &[V3], men: &[crate::stackroom::StackViewFixMenItem]) -> Option<Vec<crate::orders::StackPost>> { todo!() }

/// (src/game/stackroom.ts:350 nextRoom)
pub fn next_room(g: &mut crate::game::Game, d: DoorId, side: f64, f: &crate::stackroom::Floor, used: &[V3], min_m: f64, shy: bool) -> Option<crate::stackroom::NextRoom> { todo!() }

/// (src/game/stackroom.ts:372 stackRoomProbe)
pub fn stack_room_probe(g: &mut crate::game::Game, d: DoorId, side: f64) -> crate::stackroom::StackRoomProbe { todo!() }

/// (src/game/stackroom.ts:377 stackRoomSummary)
pub fn stack_room_summary(g: &mut crate::game::Game) -> String { todo!() }
