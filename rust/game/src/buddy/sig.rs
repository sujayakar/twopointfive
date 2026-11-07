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

/// (src/game/buddy.ts:78 stateOf)
pub fn state_of<'g>(g: &'g mut crate::game::Game) -> &'g mut crate::buddy::State { todo!() }

/// (src/game/buddy.ts:80 flat)
pub fn flat(a: V3, b: V3) -> f64 { todo!() }

/// (src/game/buddy.ts:81 pick)
pub fn pick(g: &mut crate::game::Game, r: [f64; 2]) -> f64 { todo!() }

/// (src/game/buddy.ts:82 alive)
pub fn alive(g: &mut crate::game::Game, d: RoomfightDefId) -> bool { todo!() }

/// (src/game/buddy.ts:83 note)
pub fn note(g: &mut crate::game::Game, gd: GuardId, what: &str) { todo!() }

/// (src/game/buddy.ts:84 cellOf)
pub fn cell_of(g: &mut crate::game::Game, p: V3) -> f64 { todo!() }

/// (src/game/buddy.ts:87 own)
pub fn own(g: &mut crate::game::Game, d: RoomfightDefId) -> bool { todo!() }

/// (src/game/buddy.ts:89 mates)
pub fn mates(g: &mut crate::game::Game, d: RoomfightDefId) -> Vec<RoomfightDefId> { todo!() }

/// (src/game/buddy.ts:93 teamKnown)
pub fn team_known(g: &mut crate::game::Game) -> Vec<V3> { todo!() }

/// (src/game/buddy.ts:103 spotBy)
pub fn spot_by(g: &mut crate::game::Game, c_2: RoomfightDefId) -> V3 { todo!() }

/// (src/game/buddy.ts:116 wayTo)
pub fn way_to(g: &mut crate::game::Game, d: RoomfightDefId, spot: V3, known: &[V3]) -> Option<f64> { todo!() }

/// (src/game/buddy.ts:128 free)
pub fn free(g: &mut crate::game::Game, x: RoomfightDefId, mine: Option<u32>) -> bool { todo!() }

/// (src/game/buddy.ts:141 answerer)
pub fn answerer(g: &mut crate::game::Game, c_2: RoomfightDefId) -> Option<crate::buddy::Answerer> { todo!() }

/// (src/game/buddy.ts:153 send)
pub fn send(g: &mut crate::game::Game, c_2: RoomfightDefId, a: crate::buddy::SendA, t0: f64) { todo!() }

/// (src/game/buddy.ts:160 consider)
pub fn consider(g: &mut crate::game::Game, d: RoomfightDefId) { todo!() }

/// (src/game/buddy.ts:188 tickErrand)
pub fn tick_errand(g: &mut crate::game::Game, e: &mut crate::buddy::Errand) -> bool { todo!() }

/// (src/game/buddy.ts:235 nearestEntry)
pub fn nearest_entry(g: &mut crate::game::Game, r: RoomfightRoomId, p: V3) -> f64 { todo!() }

/// (src/game/buddy.ts:240 buddySay)
pub fn buddy_say(g: &mut crate::game::Game) { todo!() }

/// (src/game/buddy.ts:245 buddyPaired)
pub fn buddy_paired(g: &mut crate::game::Game, d: RoomfightDefId) -> bool { todo!() }

/// (src/game/buddy.ts:247 buddyTick)
pub fn buddy_tick(g: &mut crate::game::Game) { todo!() }

/// (src/game/buddy.ts:256 buddyStop)
pub fn buddy_stop(g: &mut crate::game::Game) { todo!() }

/// (src/game/buddy.ts:258 buddyStats)
pub fn buddy_stats(g: &mut crate::game::Game) -> JsMap<String, f64> { todo!() }

/// (src/game/buddy.ts:259 buddySummary)
pub fn buddy_summary(g: &mut crate::game::Game) -> String { todo!() }
