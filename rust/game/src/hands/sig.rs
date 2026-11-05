#![allow(unused_variables)]
//! Every function and method of `hands`, with an empty body. Written by the skeleton's program.
#[allow(unused_imports)]
use crate::{ids::*, js::*, Quat, V3};
#[allow(unused_imports)]
use alloc::{string::String, vec::Vec};

/// (src/game/hands.ts:112 bodyOn)
pub fn body_on(g: &mut crate::game::Game) -> bool { todo!() }

/// (src/game/hands.ts:112 doorOn)
pub fn door_on(g: &mut crate::game::Game) -> bool { todo!() }

/// (src/game/hands.ts:114 handsBusy)
pub fn hands_busy(g: &mut crate::game::Game) -> bool { todo!() }

/// (src/game/hands.ts:115 mid)
pub fn mid(a: V3, b: V3) -> V3 { todo!() }

/// (src/game/hands.ts:121 endDist)
pub fn end_dist(g: &mut crate::game::Game, gd: GuardId, end: crate::hands::EndDistEnd) -> f64 { todo!() }

/// (src/game/hands.ts:127 bodyGripDist)
pub fn body_grip_dist(g: &mut crate::game::Game, gd: GuardId, within: f64) -> f64 { todo!() }

/// (src/game/hands.ts:134 haulBegin)
pub fn haul_begin(g: &mut crate::game::Game, gd: GuardId) { todo!() }

/// (src/game/hands.ts:151 dragHaul)
pub fn drag_haul(g: &mut crate::game::Game, gd: GuardId) -> crate::hands::Haul { todo!() }

/// (src/game/hands.ts:166 unit)
pub fn unit(v: V3, or: V3) -> V3 { todo!() }

/// (src/game/hands.ts:168 standRoom)
pub fn stand_room(g: &mut crate::game::Game, p: V3) -> bool { todo!() }

/// (src/game/hands.ts:170 turnDir)
pub fn turn_dir(a: V3, b: V3, s: f64) -> V3 { todo!() }

/// (src/game/hands.ts:177 lieOf)
pub fn lie_of(rag: RagdollId) -> crate::hands::Lie { todo!() }

/// (src/game/hands.ts:183 lieMix)
pub fn lie_mix(a: crate::hands::Lie, b: crate::hands::Lie, s: f64) -> crate::hands::Lie { todo!() }

/// (src/game/hands.ts:189 carryRides)
pub fn carry_rides(g: &mut crate::game::Game, c_2: crate::hands::CarryRidesC, rag: RagdollId, k: &mut crate::hands::HaulCarry) { todo!() }

/// (src/game/hands.ts:197 liePins)
pub fn lie_pins(rag: RagdollId, dim: [f64; 4], l: crate::hands::Lie) { todo!() }

/// (src/game/hands.ts:203 shoulderLie)
pub fn shoulder_lie(g: &mut crate::game::Game) -> crate::hands::Lie { todo!() }

/// (src/game/hands.ts:212 layLie)
pub fn lay_lie(g: &mut crate::game::Game) -> Option<crate::hands::LayLie> { todo!() }

/// (src/game/hands.ts:227 haulUpright)
pub fn haul_upright(g: &mut crate::game::Game) -> bool { todo!() }

/// (src/game/hands.ts:229 haulCrouch)
pub fn haul_crouch(g: &mut crate::game::Game) -> bool { todo!() }

/// (src/game/hands.ts:231 haulYaw)
pub fn haul_yaw(g: &mut crate::game::Game) -> Option<f64> { todo!() }

/// (src/game/hands.ts:234 carrySpots)
pub fn carry_spots(g: &mut crate::game::Game, h: crate::hands::Haul) -> [V3; 2] { todo!() }

/// (src/game/hands.ts:242 haulPins)
pub fn haul_pins(g: &mut crate::game::Game, dt: f64) -> bool { todo!() }

/// (src/game/hands.ts:281 feed)
pub fn feed(g: &mut crate::game::Game, h: crate::hands::Haul) { todo!() }

/// (src/game/hands.ts:301 haulSlack)
pub fn haul_slack(g: &mut crate::game::Game) -> bool { todo!() }

/// (src/game/hands.ts:308 haulSetDown)
pub fn haul_set_down(g: &mut crate::game::Game) -> bool { todo!() }

/// (src/game/hands.ts:321 haulSetDone)
pub fn haul_set_done(g: &mut crate::game::Game) -> bool { todo!() }

/// (src/game/hands.ts:329 haulPace)
pub fn haul_pace(g: &mut crate::game::Game) -> f64 { todo!() }

/// (src/game/hands.ts:331 haulSpeed)
pub fn haul_speed(g: &mut crate::game::Game) -> f64 { todo!() }

/// (src/game/hands.ts:335 leafU)
pub fn leaf_u(g: &mut crate::game::Game, d: DoorId, angle: f64) -> [f64; 2] { todo!() }

/// (src/game/hands.ts:337 leafDist)
pub fn leaf_dist(g: &mut crate::game::Game, d: DoorId, angle: f64, q: V3) -> f64 { todo!() }

/// (src/game/hands.ts:342 leafPerp)
pub fn leaf_perp(g: &mut crate::game::Game, d: DoorId, angle: f64, q: V3) -> f64 { todo!() }

/// (src/game/hands.ts:343 bodyR)
pub fn body_r(g: &mut crate::game::Game, d: DoorId) -> f64 { todo!() }

/// (src/game/hands.ts:345 sweepDist)
pub fn sweep_dist(g: &mut crate::game::Game, d: DoorId, a0: f64, a1: f64, q: V3) -> f64 { todo!() }

/// (src/game/hands.ts:351 crossesLeaf)
pub fn crosses_leaf(g: &mut crate::game::Game, d: DoorId, angle: f64, p: V3, q: V3) -> bool { todo!() }

/// (src/game/hands.ts:358 openSide)
pub fn open_side(g: &mut crate::game::Game, d: DoorId, a1: f64, q: V3) -> f64 { todo!() }

/// (src/game/hands.ts:367 clearSpot)
pub fn clear_spot(g: &mut crate::game::Game, d: DoorId, a0: f64, a1: f64) -> Option<V3> { todo!() }

/// (src/game/hands.ts:397 limpInSweep)
pub fn limp_in_sweep(g: &mut crate::game::Game, d: DoorId, a0: f64, a1: f64) -> bool { todo!() }

/// (src/game/hands.ts:413 doorMoveBegin)
pub fn door_move_begin(g: &mut crate::game::Game, d: DoorId) -> bool { todo!() }

/// (src/game/hands.ts:441 doorMoveTick)
pub fn door_move_tick(g: &mut crate::game::Game, dt: f64) { todo!() }

/// (src/game/hands.ts:477 handsStep)
pub fn hands_step(g: &mut crate::game::Game, own: bool, dt: f64) -> Option<[f64; 2]> { todo!() }

/// (src/game/hands.ts:507 doorMoveLowers)
pub fn door_move_lowers(g: &mut crate::game::Game) -> bool { todo!() }

/// (src/game/hands.ts:520 teamDoorHand)
pub fn team_door_hand(g: &mut crate::game::Game, gd: GuardId) -> Option<DoorId> { todo!() }

/// (src/game/hands.ts:534 handsCancel)
pub fn hands_cancel(g: &mut crate::game::Game) { todo!() }

/// (src/game/hands.ts:535 handsSummary)
pub fn hands_summary(g: &mut crate::game::Game) -> String { todo!() }
