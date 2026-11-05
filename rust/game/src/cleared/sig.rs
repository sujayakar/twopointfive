#![allow(unused_variables)]
//! Every function and method of `cleared`, with an empty body. Written by the skeleton's program.
#[allow(unused_imports)]
use crate::{ids::*, js::*, Quat, V3};
#[allow(unused_imports)]
use alloc::{string::String, vec::Vec};

/// (src/game/cleared.ts:39 ClearedRooms.constructor)
pub fn cleared_rooms_new(g: &mut crate::game::Game) -> crate::cleared::ClearedRooms { todo!() }

/// (src/game/cleared.ts:41 ClearedRooms.reset)
pub fn cleared_rooms_reset(g: &mut crate::game::Game) { todo!() }

/// (src/game/cleared.ts:44 ClearedRooms.build)
pub fn cleared_rooms_build(g: &mut crate::game::Game) -> Vec<u16> { todo!() }

/// (src/game/cleared.ts:63 ClearedRooms.roomAt)
pub fn cleared_rooms_room_at(g: &mut crate::game::Game, p: V3) -> f64 { todo!() }

/// (src/game/cleared.ts:65 ClearedRooms.knownAt)
pub fn cleared_rooms_known_at(g: &mut crate::game::Game, p: V3) -> bool { todo!() }

/// (src/game/cleared.ts:67 ClearedRooms.roomBy)
pub fn cleared_rooms_room_by(g: &mut crate::game::Game, d: DoorId, side: f64) -> f64 { todo!() }

/// (src/game/cleared.ts:69 ClearedRooms.hasFloor)
pub fn cleared_rooms_has_floor(g: &mut crate::game::Game, d: DoorId, side: f64) -> bool { todo!() }

/// (src/game/cleared.ts:71 ClearedRooms.roomsOf)
pub fn cleared_rooms_rooms_of(g: &mut crate::game::Game, p: V3) -> Vec<f64> { todo!() }

/// (src/game/cleared.ts:77 ClearedRooms.isKnown)
pub fn cleared_rooms_is_known(g: &mut crate::game::Game, room: f64) -> bool { todo!() }

/// (src/game/cleared.ts:79 ClearedRooms.unknownEyes)
pub fn cleared_rooms_unknown_eyes(g: &mut crate::game::Game, step: f64) -> Vec<crate::cleared::ClearedRoomsUnknownEyesItem> { todo!() }

/// (src/game/cleared.ts:85 ClearedRooms.mark)
pub fn cleared_rooms_mark(g: &mut crate::game::Game, p: V3) { todo!() }

/// (src/game/cleared.ts:89 ClearedRooms.tick)
pub fn cleared_rooms_tick(g: &mut crate::game::Game) { todo!() }

/// (src/game/cleared.ts:98 ClearedRooms.dearFor)
pub fn cleared_rooms_dear_for(g: &mut crate::game::Game, from: V3, to: V3, dear: f64, also: Option<&[f64]>) -> Option<crate::cleared::DearPrice> { todo!() }

/// (src/game/cleared.ts:105 ClearedRooms.dearOnly)
pub fn cleared_rooms_dear_only(g: &mut crate::game::Game, room: f64, dear: f64) -> crate::FnField { todo!() }

/// (src/game/cleared.ts:107 ClearedRooms.dearAlong)
pub fn cleared_rooms_dear_along(g: &mut crate::game::Game, from: V3, path: &[V3], pen: &mut dyn FnMut(&mut crate::game::Game, f64, f64) -> f64) -> f64 { todo!() }

/// (src/game/cleared.ts:114 ClearedRooms.summary)
pub fn cleared_rooms_summary(g: &mut crate::game::Game) -> String { todo!() }
