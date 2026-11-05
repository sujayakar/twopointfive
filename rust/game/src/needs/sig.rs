#![allow(unused_variables)]
//! Every function and method of `needs`, with an empty body. Written by the skeleton's program.
#[allow(unused_imports)]
use crate::{ids::*, js::*, Quat, V3};
#[allow(unused_imports)]
use alloc::{string::String, vec::Vec};

/// (src/game/needs.ts:49 needSpot)
pub fn need_spot(g: &mut crate::game::Game, kind: crate::needs::NeedKind) -> Option<crate::needs::NeedSpot> { todo!() }

/// (src/game/needs.ts:54 ownSpot)
pub fn own_spot(g: &mut crate::game::Game, s: &crate::needs::OwnSpotS, kind: crate::needs::NeedKind) -> crate::needs::NeedSpot { todo!() }

/// (src/game/needs.ts:113 mem)
pub fn mem<'g>(g: &'g mut crate::game::Game, gd: GuardId) -> &'g mut crate::needs::NeedMem { todo!() }

/// (src/game/needs.ts:115 needsMem)
pub fn needs_mem(g: &mut crate::game::Game, gd: GuardId) -> Option<crate::needs::NeedsMem> { todo!() }

/// (src/game/needs.ts:117 needsTrip)
pub fn needs_trip(g: &mut crate::game::Game, gd: GuardId) -> Option<crate::needs::NeedsTrip> { todo!() }

/// (src/game/needs.ts:122 floor)
pub fn floor<'g>(g: &'g mut crate::game::Game) -> &'g mut crate::needs::NeedsModFloorOfValue { todo!() }

/// (src/game/needs.ts:128 log)
pub fn log(g: &mut crate::game::Game, gd: GuardId, what: &str) { todo!() }

/// (src/game/needs.ts:131 frac)
pub fn frac(s: &str) -> f64 { todo!() }

/// (src/game/needs.ts:132 pathMetres)
pub fn path_metres(g: &mut crate::game::Game, from: V3, to: V3) -> f64 { todo!() }

/// (src/game/needs.ts:138 departurePoint)
pub fn departure_point(g: &mut crate::game::Game, gd: GuardId, spot: &crate::needs::NeedSpot) -> crate::needs::DeparturePoint { todo!() }

/// (src/game/needs.ts:145 deal)
pub fn deal(g: &mut crate::game::Game, gd: GuardId) { todo!() }

/// (src/game/needs.ts:166 needsLive)
pub fn needs_live(g: &mut crate::game::Game) -> bool { todo!() }

/// (src/game/needs.ts:170 floorCalm)
pub fn floor_calm(g: &mut crate::game::Game) -> bool { todo!() }

/// (src/game/needs.ts:172 manCalm)
pub fn man_calm(g: &mut crate::game::Game, gd: GuardId, awareness: f64) -> bool { todo!() }

/// (src/game/needs.ts:176 interruptOf)
pub fn interrupt_of(g: &mut crate::game::Game, gd: GuardId) -> Option<String> { todo!() }

/// (src/game/needs.ts:189 needsTick)
pub fn needs_tick(g: &mut crate::game::Game, dt: f64) { todo!() }

/// (src/game/needs.ts:220 scheduleTrip)
pub fn schedule_trip(g: &mut crate::game::Game, gd: GuardId, kind: crate::needs::NeedKind, forced: bool) -> bool { todo!() }

/// (src/game/needs.ts:236 pathLen)
pub fn path_len(from: V3, p: &[V3]) -> f64 { todo!() }

/// (src/game/needs.ts:239 cancelTrip)
pub fn cancel_trip(g: &mut crate::game::Game, gd: GuardId, why: &str) { todo!() }

/// (src/game/needs.ts:249 pickLine)
pub fn pick_line(g: &mut crate::game::Game, pool: &[String], gd: GuardId, salt: &str) -> String { todo!() }

/// (src/game/needs.ts:251 partnerFor)
pub fn partner_for(g: &mut crate::game::Game, gd: GuardId) -> Option<GuardId> { todo!() }

/// (src/game/needs.ts:262 announce)
pub fn announce(g: &mut crate::game::Game, gd: GuardId, t: &mut crate::needs::Trip) { todo!() }

/// (src/game/needs.ts:276 partnerReply)
pub fn partner_reply(g: &mut crate::game::Game, gd: GuardId, t: &mut crate::needs::Trip) { todo!() }

/// (src/game/needs.ts:290 needsPatrol)
pub fn needs_patrol(g: &mut crate::game::Game, gd: GuardId, dt: f64, walk: f64) -> bool { todo!() }

/// (src/game/needs.ts:363 needsSend)
pub fn needs_send(g: &mut crate::game::Game, gd: Option<GuardId>, kind: Option<crate::needs::NeedKind>) -> String { todo!() }

/// (src/game/needs.ts:378 needsSetLevel)
pub fn needs_set_level(g: &mut crate::game::Game, gd: GuardId, kind: crate::needs::NeedKind, level: f64) -> bool { todo!() }

/// (src/game/needs.ts:385 saveNeeds)
pub fn save_needs(g: &mut crate::game::Game) { todo!() }

/// (src/game/needs.ts:389 loadNeeds)
pub fn load_needs(g: &mut crate::game::Game) { todo!() }

/// (src/game/needs.ts:400 resetNeeds)
pub fn reset_needs(g: &mut crate::game::Game) { todo!() }

/// (src/game/needs.ts:438 needsSummary)
pub fn needs_summary(g: &mut crate::game::Game) -> String { todo!() }
