#![allow(unused_variables)]
//! Every function and method of `darkdoor`, with an empty body. Written by the skeleton's program.
#[allow(unused_imports)]
use crate::{ids::*, js::*, Quat, V3};
#[allow(unused_imports)]
use alloc::{string::String, vec::Vec};

/// (src/game/darkdoor.ts:97 darkDoorLines)
pub fn dark_door_lines(kind: Option<crate::darkdoor::LineKind>) -> Vec<String> { todo!() }

/// (src/game/darkdoor.ts:99 bark)
pub fn bark(g: &mut crate::game::Game, gd: GuardId, kind: crate::darkdoor::LineKind, radio: bool) { todo!() }

/// (src/game/darkdoor.ts:105 flat)
pub fn flat(a: V3, b: V3) -> f64 { todo!() }

/// (src/game/darkdoor.ts:106 chestOf)
pub fn chest_of(g: &mut crate::game::Game, c_2: CharId) -> V3 { todo!() }

/// (src/game/darkdoor.ts:107 headOf)
pub fn head_of(g: &mut crate::game::Game, c_2: CharId) -> V3 { todo!() }

/// (src/game/darkdoor.ts:109 ours)
pub fn ours(g: &mut crate::game::Game, gd: GuardId) -> bool { todo!() }

/// (src/game/darkdoor.ts:110 rescued)
pub fn rescued(g: &mut crate::game::Game, gd: GuardId) -> bool { todo!() }

/// (src/game/darkdoor.ts:111 alarmed)
pub fn alarmed(g: &mut crate::game::Game, gd: GuardId) -> bool { todo!() }

/// (src/game/darkdoor.ts:113 own)
pub fn own(g: &mut crate::game::Game, gd: GuardId) -> bool { todo!() }

/// (src/game/darkdoor.ts:115 able)
pub fn able(g: &mut crate::game::Game, gd: GuardId) -> bool { todo!() }

/// (src/game/darkdoor.ts:121 fightWalk)
pub fn fight_walk(g: &mut crate::game::Game, gd: GuardId) -> crate::darkdoor::FightWalk { todo!() }

/// (src/game/darkdoor.ts:128 canCome)
pub fn can_come(g: &mut crate::game::Game, o: GuardId) -> bool { todo!() }

/// (src/game/darkdoor.ts:132 mind)
pub fn mind(g: &mut crate::game::Game, gd: GuardId) { todo!() }

/// (src/game/darkdoor.ts:139 greenMan)
pub fn green_man(g: &mut crate::game::Game, gd: GuardId) -> bool { todo!() }

/// (src/game/darkdoor.ts:144 mateBy)
pub fn mate_by(g: &mut crate::game::Game, gd: GuardId, fit: Option<&mut dyn FnMut(&mut crate::game::Game, GuardId) -> bool>) -> Option<GuardId> { todo!() }

/// (src/game/darkdoor.ts:151 freeMate)
pub fn free_mate(g: &mut crate::game::Game, gd: GuardId) -> Option<GuardId> { todo!() }

/// (src/game/darkdoor.ts:162 roomCells)
pub fn room_cells(g: &mut crate::game::Game, room: f64) -> Vec<f64> { todo!() }

/// (src/game/darkdoor.ts:173 darkRoom)
pub fn dark_room(g: &mut crate::game::Game, room: f64) -> crate::darkdoor::DarkRoom { todo!() }

/// (src/game/darkdoor.ts:185 standsLit)
pub fn stands_lit(g: &mut crate::game::Game, gd: GuardId, r: crate::darkdoor::StandsLitR) -> bool { todo!() }

/// (src/game/darkdoor.ts:190 open)
pub fn open(g: &mut crate::game::Game, d: DoorId) -> bool { todo!() }

/// (src/game/darkdoor.ts:192 doorAhead)
pub fn door_ahead(g: &mut crate::game::Game, gd: GuardId, reach: f64) -> Option<crate::darkdoor::DoorAhead> { todo!() }

/// (src/game/darkdoor.ts:203 standSpot)
pub fn stand_spot(g: &mut crate::game::Game, gd: GuardId, d: DoorId, side: f64) -> Option<V3> { todo!() }

/// (src/game/darkdoor.ts:212 passes)
pub fn passes(s: &crate::darkdoor::Fear, door: f64, t: f64) -> bool { todo!() }

/// (src/game/darkdoor.ts:213 pass)
pub fn pass(g: &mut crate::game::Game, gd: GuardId, door: f64) { todo!() }

/// (src/game/darkdoor.ts:215 drop)
pub fn drop(g: &mut crate::game::Game, gd: GuardId) { todo!() }

/// (src/game/darkdoor.ts:218 end)
pub fn end(g: &mut crate::game::Game, gd: GuardId, unresolved: Option<bool>) { todo!() }

/// (src/game/darkdoor.ts:226 call)
pub fn call(g: &mut crate::game::Game, gd: GuardId, m: GuardId) { todo!() }

/// (src/game/darkdoor.ts:236 darkDoorGate)
pub fn dark_door_gate(g: &mut crate::game::Game, gd: GuardId, dt: f64, speed: f64) -> Option<f64> { todo!() }

/// (src/game/darkdoor.ts:270 hold)
pub fn hold(g: &mut crate::game::Game, gd: GuardId, dt: f64, speed: f64, seen: bool) -> Option<f64> { todo!() }

/// (src/game/darkdoor.ts:308 darkDoorLook)
pub fn dark_door_look(g: &mut crate::game::Game, gd: GuardId) -> Option<V3> { todo!() }

/// (src/game/darkdoor.ts:319 darkDoorTick)
pub fn dark_door_tick(g: &mut crate::game::Game, dt: f64) { todo!() }

/// (src/game/darkdoor.ts:342 lose)
pub fn lose(g: &mut crate::game::Game, gd: GuardId) { todo!() }

/// (src/game/darkdoor.ts:348 darkDoorOf)
pub fn dark_door_of(g: &mut crate::game::Game, gd: GuardId) -> Option<crate::darkdoor::DarkDoorOf> { todo!() }

/// (src/game/darkdoor.ts:350 darkDoorGreen)
pub fn dark_door_green(g: &mut crate::game::Game, gd: GuardId) -> bool { todo!() }

/// (src/game/darkdoor.ts:352 darkDoorSummary)
pub fn dark_door_summary(g: &mut crate::game::Game) -> String { todo!() }
