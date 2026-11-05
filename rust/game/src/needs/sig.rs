#![allow(unused_variables)]
//! Every function and method of `needs`, with an empty body. Written by the skeleton's program.
#[allow(unused_imports)]
use crate::{ids::*, js::*, Quat, V3};
#[allow(unused_imports)]
use alloc::{string::String, vec::Vec};

/// (src/game/needs.ts:48 needSpot)
pub fn need_spot(g: &mut crate::game::Game, kind: crate::needs::NeedKind) -> Option<crate::needs::NeedSpot> { todo!() }

/// (src/game/needs.ts:53 ownSpot)
pub fn own_spot(g: &mut crate::game::Game, s: &crate::needs::OwnSpotS, kind: crate::needs::NeedKind) -> crate::needs::NeedSpot { todo!() }

/// (src/game/needs.ts:111 mem)
pub fn mem<'g>(g: &'g mut crate::game::Game, gd: GuardId) -> &'g mut crate::needs::NeedMem { todo!() }

/// (src/game/needs.ts:113 needsMem)
pub fn needs_mem(g: &mut crate::game::Game, gd: GuardId) -> Option<crate::needs::NeedsMem> { todo!() }

/// (src/game/needs.ts:115 needsTrip)
pub fn needs_trip(g: &mut crate::game::Game, gd: GuardId) -> Option<crate::needs::NeedsTrip> { todo!() }

/// (src/game/needs.ts:117 needsBusy)
pub fn needs_busy(g: &mut crate::game::Game, gd: GuardId) -> bool { todo!() }

/// (src/game/needs.ts:121 floor)
pub fn floor<'g>(g: &'g mut crate::game::Game) -> &'g mut crate::needs::NeedsModFloorOfValue { todo!() }

/// (src/game/needs.ts:127 log)
pub fn log(g: &mut crate::game::Game, gd: GuardId, what: &str) { todo!() }

/// (src/game/needs.ts:130 frac)
pub fn frac(s: &str) -> f64 { todo!() }

/// (src/game/needs.ts:131 pathMetres)
pub fn path_metres(g: &mut crate::game::Game, from: V3, to: V3) -> f64 { todo!() }

/// (src/game/needs.ts:137 departurePoint)
pub fn departure_point(g: &mut crate::game::Game, gd: GuardId, spot: &crate::needs::NeedSpot) -> crate::needs::DeparturePoint { todo!() }

/// (src/game/needs.ts:144 deal)
pub fn deal(g: &mut crate::game::Game, gd: GuardId) { todo!() }

/// (src/game/needs.ts:165 needsLive)
pub fn needs_live(g: &mut crate::game::Game) -> bool { todo!() }

/// (src/game/needs.ts:169 floorCalm)
pub fn floor_calm(g: &mut crate::game::Game) -> bool { todo!() }

/// (src/game/needs.ts:171 manCalm)
pub fn man_calm(g: &mut crate::game::Game, gd: GuardId, awareness: f64) -> bool { todo!() }

/// (src/game/needs.ts:175 interruptOf)
pub fn interrupt_of(g: &mut crate::game::Game, gd: GuardId) -> Option<String> { todo!() }

/// (src/game/needs.ts:188 needsTick)
pub fn needs_tick(g: &mut crate::game::Game, dt: f64) { todo!() }

/// (src/game/needs.ts:219 scheduleTrip)
pub fn schedule_trip(g: &mut crate::game::Game, gd: GuardId, kind: crate::needs::NeedKind, forced: bool) -> bool { todo!() }

/// (src/game/needs.ts:235 pathLen)
pub fn path_len(from: V3, p: &[V3]) -> f64 { todo!() }

/// (src/game/needs.ts:238 cancelTrip)
pub fn cancel_trip(g: &mut crate::game::Game, gd: GuardId, why: &str) { todo!() }

/// (src/game/needs.ts:248 pickLine)
pub fn pick_line(g: &mut crate::game::Game, pool: &[String], gd: GuardId, salt: &str) -> String { todo!() }

/// (src/game/needs.ts:250 partnerFor)
pub fn partner_for(g: &mut crate::game::Game, gd: GuardId) -> Option<GuardId> { todo!() }

/// (src/game/needs.ts:261 announce)
pub fn announce(g: &mut crate::game::Game, gd: GuardId, t: &mut crate::needs::Trip) { todo!() }

/// (src/game/needs.ts:275 partnerReply)
pub fn partner_reply(g: &mut crate::game::Game, gd: GuardId, t: &mut crate::needs::Trip) { todo!() }

/// (src/game/needs.ts:289 needsPatrol)
pub fn needs_patrol(g: &mut crate::game::Game, gd: GuardId, dt: f64, walk: f64) -> bool { todo!() }

/// (src/game/needs.ts:362 needsSend)
pub fn needs_send(g: &mut crate::game::Game, gd: Option<GuardId>, kind: Option<crate::needs::NeedKind>) -> String { todo!() }

/// (src/game/needs.ts:377 needsSetLevel)
pub fn needs_set_level(g: &mut crate::game::Game, gd: GuardId, kind: crate::needs::NeedKind, level: f64) -> bool { todo!() }

/// (src/game/needs.ts:384 saveNeeds)
pub fn save_needs(g: &mut crate::game::Game, tune: crate::needs::SaveNeedsTune) { todo!() }

/// (src/game/needs.ts:388 loadNeeds)
pub fn load_needs(g: &mut crate::game::Game, tune: &mut crate::needs::LoadNeedsTune) { todo!() }

/// (src/game/needs.ts:399 resetNeeds)
pub fn reset_needs(g: &mut crate::game::Game, tune: &mut crate::needs::ResetNeedsTune) { todo!() }

/// (src/game/needs.ts:436 needsSummary)
pub fn needs_summary(g: &mut crate::game::Game) -> String { todo!() }
