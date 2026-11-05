#![allow(unused_variables)]
//! Every function and method of `doorway`, with an empty body. Written by the skeleton's program.
#[allow(unused_imports)]
use crate::{ids::*, js::*, Quat, V3};
#[allow(unused_imports)]
use alloc::{string::String, vec::Vec};

/// (src/game/doorway.ts:153 stateOf)
pub fn state_of<'g>(g: &'g mut crate::game::Game, gd: GuardId) -> &'g mut crate::doorway::St { todo!() }

/// (src/game/doorway.ts:158 flat)
pub fn flat(a: V3, b: V3) -> f64 { todo!() }

/// (src/game/doorway.ts:159 ours)
pub fn ours(g: &mut crate::game::Game, gd: GuardId) -> bool { todo!() }

/// (src/game/doorway.ts:160 rel)
pub fn rel(g: &mut crate::game::Game, d: DoorId, p: V3) -> crate::doorway::Rel { todo!() }

/// (src/game/doorway.ts:162 inOpening)
pub fn in_opening(g: &mut crate::game::Game, d: DoorId, p: V3, half: Option<f64>) -> bool { todo!() }

/// (src/game/doorway.ts:164 inSweep)
pub fn in_sweep(g: &mut crate::game::Game, d: DoorId, p: V3) -> bool { todo!() }

/// (src/game/doorway.ts:171 doorwayStand)
pub fn doorway_stand(g: &mut crate::game::Game, p: V3) -> bool { todo!() }

/// (src/game/doorway.ts:177 doorwayLane)
pub fn doorway_lane(g: &mut crate::game::Game, p: V3, but: Option<DoorId>) -> Option<DoorId> { todo!() }

/// (src/game/doorway.ts:193 doorwayLanePrice)
pub fn doorway_lane_price(g: &mut crate::game::Game, not: Option<&mut dyn FnMut(&mut crate::game::Game, DoorId) -> bool>) -> Option<crate::doorway::DoorwayLanePrice> { todo!() }

/// (src/game/doorway.ts:213 doorwayOffLane)
pub fn doorway_off_lane(g: &mut crate::game::Game, p: V3, from: V3, k: f64, taken: Option<&[V3]>) -> Option<V3> { todo!() }

/// (src/game/doorway.ts:229 behindLeaf)
pub fn behind_leaf(g: &mut crate::game::Game, d: DoorId, along: f64, perp: f64) -> bool { todo!() }

/// (src/game/doorway.ts:239 roundsLeaves)
pub fn rounds_leaves(g: &mut crate::game::Game, gd: GuardId) -> bool { todo!() }

/// (src/game/doorway.ts:242 leafStalled)
pub fn leaf_stalled(g: &mut crate::game::Game, gd: GuardId) -> bool { todo!() }

/// (src/game/doorway.ts:261 doorwayFile)
pub fn doorway_file(g: &mut crate::game::Game) -> bool { todo!() }

/// (src/game/doorway.ts:264 doorAhead)
pub fn door_ahead(g: &mut crate::game::Game, gd: GuardId) -> Option<crate::doorway::DoorAhead> { todo!() }

/// (src/game/doorway.ts:279 friends)
pub fn friends(g: &mut crate::game::Game, gd: GuardId) -> Vec<crate::doorway::FriendsItem> { todo!() }

/// (src/game/doorway.ts:287 stepTo)
pub fn step_to(g: &mut crate::game::Game, gd: GuardId, p: V3, pace: f64, dt: f64, turn: bool) -> bool { todo!() }

/// (src/game/doorway.ts:295 spotFree)
pub fn spot_free(g: &mut crate::game::Game, gd: GuardId, p: V3, apart: f64) -> bool { todo!() }

/// (src/game/doorway.ts:302 jambSpot)
pub fn jamb_spot(g: &mut crate::game::Game, gd: GuardId, d: DoorId, side: f64) -> Option<V3> { todo!() }

/// (src/game/doorway.ts:315 through)
pub fn through(g: &mut crate::game::Game, o: GuardId, d: DoorId) -> Option<crate::doorway::Through> { todo!() }

/// (src/game/doorway.ts:323 doorwayGate)
pub fn doorway_gate(g: &mut crate::game::Game, gd: GuardId, dt: f64, speed: f64) -> Option<f64> { todo!() }

/// (src/game/doorway.ts:409 offThreshold)
pub fn off_threshold(g: &mut crate::game::Game, gd: GuardId, aim_at: Option<V3>, dt: f64, need_shot: bool) -> bool { todo!() }

/// (src/game/doorway.ts:454 doorwayInWay)
pub fn doorway_in_way(g: &mut crate::game::Game, p: V3) -> bool { todo!() }

/// (src/game/doorway.ts:459 doorwayHeldBy)
pub fn doorway_held_by(g: &mut crate::game::Game, gd: GuardId) -> Option<String> { todo!() }

/// (src/game/doorway.ts:461 doorwayHeld)
pub fn doorway_held(g: &mut crate::game::Game, gd: GuardId) -> bool { todo!() }

/// (src/game/doorway.ts:464 doorwayClearNear)
pub fn doorway_clear_near(g: &mut crate::game::Game, p: V3, bad: &mut dyn FnMut(&mut crate::game::Game, V3) -> bool, taken: Option<&[V3]>, apart: Option<f64>, face: Option<V3>) -> Option<V3> { todo!() }
