#![allow(unused_variables)]
//! Every function and method of `handleside`, with an empty body. Written by the skeleton's program.
#[allow(unused_imports)]
use crate::{ids::*, js::*, Quat, V3};
#[allow(unused_imports)]
use alloc::{string::String, vec::Vec};

/// (src/game/handleside.ts:81 live)
pub fn live(g: &mut crate::game::Game) -> bool { todo!() }

/// (src/game/handleside.ts:84 fresh)
pub fn fresh(g: &mut crate::game::Game, st: Option<&crate::handleside::St>) -> bool { todo!() }

/// (src/game/handleside.ts:85 sideAt)
pub fn side_at(g: &mut crate::game::Game, d: DoorId, p: V3) -> f64 { todo!() }

/// (src/game/handleside.ts:86 offAt)
pub fn off_at(g: &mut crate::game::Game, d: DoorId, p: V3) -> f64 { todo!() }

/// (src/game/handleside.ts:87 ptOf)
pub fn pt_of(g: &mut crate::game::Game, d: DoorId, along: f64, off: f64, side: f64) -> V3 { todo!() }

/// (src/game/handleside.ts:89 handleAt)
pub fn handle_at(g: &mut crate::game::Game, d: DoorId) -> V3 { todo!() }

/// (src/game/handleside.ts:90 flat)
pub fn flat(a: V3, b: V3) -> f64 { todo!() }

/// (src/game/handleside.ts:92 walled)
pub fn walled(g: &mut crate::game::Game, a: V3, b: V3) -> bool { todo!() }

/// (src/game/handleside.ts:97 stands)
pub fn stands(g: &mut crate::game::Game, p: V3, nav: Option<bool>) -> bool { todo!() }

/// (src/game/handleside.ts:102 atHandle)
pub fn at_handle(g: &mut crate::game::Game, d: DoorId, p: V3, reach: Option<f64>) -> bool { todo!() }

/// (src/game/handleside.ts:107 faceOf)
pub fn face_of(g: &mut crate::game::Game, d: DoorId, side: f64) -> crate::handleside::Face { todo!() }

/// (src/game/handleside.ts:114 handleSpot)
pub fn handle_spot(g: &mut crate::game::Game, d: DoorId, side: f64) -> Option<crate::handleside::Spot> { todo!() }

/// (src/game/handleside.ts:115 findFace)
pub fn find_face(g: &mut crate::game::Game, d: DoorId, side: f64) -> crate::handleside::Face { todo!() }

/// (src/game/handleside.ts:144 ajarOf)
pub fn ajar_of(g: &mut crate::game::Game, d: DoorId) -> f64 { todo!() }

/// (src/game/handleside.ts:146 wantsHand)
pub fn wants_hand(g: &mut crate::game::Game, gd: GuardId, d: DoorId, from: V3) -> bool { todo!() }

/// (src/game/handleside.ts:155 ahead)
pub fn ahead(g: &mut crate::game::Game, gd: GuardId) -> Option<crate::handleside::Ahead> { todo!() }

/// (src/game/handleside.ts:163 release)
pub fn release(g: &mut crate::game::Game, gd: GuardId, rest: bool) { todo!() }

/// (src/game/handleside.ts:165 pullOpen)
pub fn pull_open(g: &mut crate::game::Game, gd: GuardId, d: DoorId, far: bool) { todo!() }

/// (src/game/handleside.ts:171 bend)
pub fn bend(g: &mut crate::game::Game, gd: GuardId, d: DoorId, a: crate::handleside::BendA, a_2: V3, b: V3) -> bool { todo!() }

/// (src/game/handleside.ts:177 halt)
pub fn halt(g: &mut crate::game::Game, gd: GuardId, dt: f64) { todo!() }

/// (src/game/handleside.ts:182 handleGate)
pub fn handle_gate(g: &mut crate::game::Game, gd: GuardId, dt: f64, speed: f64, strafe: Option<bool>) -> bool { todo!() }

/// (src/game/handleside.ts:189 handleHeld)
pub fn handle_held(g: &mut crate::game::Game, gd: GuardId) -> bool { todo!() }

/// (src/game/handleside.ts:190 gate)
pub fn gate(g: &mut crate::game::Game, gd: GuardId, dt: f64, speed: f64, strafe: bool) -> bool { todo!() }

/// (src/game/handleside.ts:280 wayTo)
pub fn way_to(g: &mut crate::game::Game, a: V3, to: V3) -> Option<Vec<V3>> { todo!() }

/// (src/game/handleside.ts:290 stepTo)
pub fn step_to(g: &mut crate::game::Game, gd: GuardId, to: V3, dt: f64, speed: f64, strafe: bool, pass: Option<bool>) -> bool { todo!() }

/// (src/game/handleside.ts:306 handPress)
pub fn hand_press(g: &mut crate::game::Game, gd: GuardId) -> Option<f64> { todo!() }

/// (src/game/handleside.ts:315 toolSpot)
pub fn tool_spot(g: &mut crate::game::Game, d: DoorId, side: f64, at: V3, peek: Option<bool>, order: Option<bool>) -> Option<crate::handleside::ToolSpot> { todo!() }

/// (src/game/handleside.ts:337 nearStand)
pub fn near_stand(g: &mut crate::game::Game, d: DoorId, side: f64, at: V3) -> Option<crate::handleside::NearStand> { todo!() }

/// (src/game/handleside.ts:352 toolStep)
pub fn tool_step(g: &mut crate::game::Game, gd: GuardId, pos: V3) -> bool { todo!() }

/// (src/game/handleside.ts:360 latchFirst)
pub fn latch_first(g: &mut crate::game::Game, d: DoorId, stack: V3) -> bool { todo!() }
