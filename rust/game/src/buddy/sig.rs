#![allow(unused_variables)]
//! Every function and method of `buddy`, with an empty body. Written by the skeleton's program.
#[allow(unused_imports)]
use crate::{ids::*, js::*, Quat, V3};
#[allow(unused_imports)]
use alloc::{string::String, vec::Vec};

/// (src/game/buddy.ts:66 saveBuddy)
pub fn save_buddy(g: &mut crate::game::Game) { todo!() }

/// (src/game/buddy.ts:74 newStats)
pub fn new_stats() -> crate::buddy::StateStats { todo!() }

/// (src/game/buddy.ts:77 stateOf)
pub fn state_of<'g>(g: &'g mut crate::game::Game) -> &'g mut crate::buddy::State { todo!() }

/// (src/game/buddy.ts:79 flat)
pub fn flat(a: V3, b: V3) -> f64 { todo!() }

/// (src/game/buddy.ts:80 pick)
pub fn pick(g: &mut crate::game::Game, r: [f64; 2]) -> f64 { todo!() }

/// (src/game/buddy.ts:81 alive)
pub fn alive(g: &mut crate::game::Game, d: RoomfightDefId) -> bool { todo!() }

/// (src/game/buddy.ts:82 note)
pub fn note(g: &mut crate::game::Game, gd: GuardId, what: &str) { todo!() }

/// (src/game/buddy.ts:83 cellOf)
pub fn cell_of(g: &mut crate::game::Game, p: V3) -> f64 { todo!() }

/// (src/game/buddy.ts:86 own)
pub fn own(g: &mut crate::game::Game, d: RoomfightDefId) -> bool { todo!() }

/// (src/game/buddy.ts:88 mates)
pub fn mates(g: &mut crate::game::Game, d: RoomfightDefId) -> Vec<RoomfightDefId> { todo!() }

/// (src/game/buddy.ts:92 teamKnown)
pub fn team_known(g: &mut crate::game::Game) -> Vec<V3> { todo!() }

/// (src/game/buddy.ts:102 spotBy)
pub fn spot_by(g: &mut crate::game::Game, c_2: RoomfightDefId) -> V3 { todo!() }

/// (src/game/buddy.ts:115 wayTo)
pub fn way_to(g: &mut crate::game::Game, d: RoomfightDefId, spot: V3, known: &[V3]) -> Option<f64> { todo!() }

/// (src/game/buddy.ts:127 free)
pub fn free(g: &mut crate::game::Game, x: RoomfightDefId, mine: Option<u32>) -> bool { todo!() }

/// (src/game/buddy.ts:140 answerer)
pub fn answerer(g: &mut crate::game::Game, c_2: RoomfightDefId) -> Option<crate::buddy::Answerer> { todo!() }

/// (src/game/buddy.ts:152 send)
pub fn send(g: &mut crate::game::Game, c_2: RoomfightDefId, a: crate::buddy::SendA, t0: f64) { todo!() }

/// (src/game/buddy.ts:159 consider)
pub fn consider(g: &mut crate::game::Game, d: RoomfightDefId) { todo!() }

/// (src/game/buddy.ts:187 tickErrand)
pub fn tick_errand(g: &mut crate::game::Game, e: &mut crate::buddy::Errand) -> bool { todo!() }

/// (src/game/buddy.ts:234 nearestEntry)
pub fn nearest_entry(g: &mut crate::game::Game, r: RoomfightRoomId, p: V3) -> f64 { todo!() }

/// (src/game/buddy.ts:238 buddyOwns)
pub fn buddy_owns(g: &mut crate::game::Game, d: RoomfightDefId) -> bool { todo!() }

/// (src/game/buddy.ts:244 buddyTick)
pub fn buddy_tick(g: &mut crate::game::Game) { todo!() }

/// (src/game/buddy.ts:252 buddyStop)
pub fn buddy_stop(g: &mut crate::game::Game) { todo!() }

/// (src/game/buddy.ts:254 buddyStats)
pub fn buddy_stats(g: &mut crate::game::Game) -> JsMap<String, f64> { todo!() }

/// (src/game/buddy.ts:255 buddySummary)
pub fn buddy_summary(g: &mut crate::game::Game) -> String { todo!() }
