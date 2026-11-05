#![allow(unused_variables)]
//! Every function and method of `rearwatch`, with an empty body. Written by the skeleton's program.
#[allow(unused_imports)]
use crate::{ids::*, js::*, Quat, V3};
#[allow(unused_imports)]
use alloc::{string::String, vec::Vec};

/// (src/game/rearwatch.ts:107 bookOf)
pub fn book_of<'g>(g: &'g mut crate::game::Game) -> &'g mut crate::rearwatch::Book { todo!() }

/// (src/game/rearwatch.ts:113 isOpenWay)
pub fn is_open_way(g: &mut crate::game::Game, d: DoorId) -> bool { todo!() }

/// (src/game/rearwatch.ts:116 rearWays)
pub fn rear_ways(g: &mut crate::game::Game, door: DoorId, side: f64, marks: &[V3]) -> Vec<crate::rearwatch::RearWaysItem> { todo!() }

/// (src/game/rearwatch.ts:136 rearWatchTick)
pub fn rear_watch_tick(g: &mut crate::game::Game) { todo!() }

/// (src/game/rearwatch.ts:237 note)
pub fn note(b: &mut crate::rearwatch::Book, s: &str) { todo!() }

/// (src/game/rearwatch.ts:248 roomKeepTick)
pub fn room_keep_tick(g: &mut crate::game::Game, pl: PlayerId) { todo!() }

/// (src/game/rearwatch.ts:306 walkTo)
pub fn walk_to(g: &mut crate::game::Game, a: V3, b: V3) -> f64 { todo!() }

/// (src/game/rearwatch.ts:312 roomKept)
pub fn room_kept(g: &mut crate::game::Game, gd: GuardId) -> bool { todo!() }

/// (src/game/rearwatch.ts:317 heldByOther)
pub fn held_by_other(g: &mut crate::game::Game, g_2: &crate::rearwatch::Group, o: CoverOpeningId, squad: &[GuardId]) -> bool { todo!() }

/// (src/game/rearwatch.ts:327 rearSees)
pub fn rear_sees(g: &mut crate::game::Game, p: V3, o: CoverOpeningId, front: V3) -> Option<V3> { todo!() }

/// (src/game/rearwatch.ts:331 rearPinOf)
pub fn rear_pin_of(g: &mut crate::game::Game, gd: GuardId) -> Option<crate::rearwatch::RearPin> { todo!() }

/// (src/game/rearwatch.ts:333 rearPin)
pub fn rear_pin(g: &mut crate::game::Game, gd: GuardId) -> Option<crate::rearwatch::RearPin> { todo!() }

/// (src/game/rearwatch.ts:337 rearHolds)
pub fn rear_holds(g: &mut crate::game::Game, op: DoorOpId, gd: GuardId) -> bool { todo!() }

/// (src/game/rearwatch.ts:348 rearLast)
pub fn rear_last(g: &mut crate::game::Game, op: DoorOpId, turn: &[GuardId], first: GuardId) -> Vec<GuardId> { todo!() }

/// (src/game/rearwatch.ts:356 rearWatcher)
pub fn rear_watcher(g: &mut crate::game::Game, op: DoorOpId) -> Option<GuardId> { todo!() }

/// (src/game/rearwatch.ts:361 rearWatchLog)
pub fn rear_watch_log(g: &mut crate::game::Game) -> Vec<String> { todo!() }

/// (src/game/rearwatch.ts:363 rearWatchRows)
pub fn rear_watch_rows(g: &mut crate::game::Game) -> f64 { todo!() }

/// (src/game/rearwatch.ts:364 rearWatchSummary)
pub fn rear_watch_summary(g: &mut crate::game::Game) -> String { todo!() }
