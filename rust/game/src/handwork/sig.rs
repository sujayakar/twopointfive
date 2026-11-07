#![allow(unused_variables)]
//! Every function and method of `handwork`, with an empty body. Written by the skeleton's program.
#[allow(unused_imports)]
use crate::{ids::*, js::*, Quat, V3};
#[allow(unused_imports)]
use alloc::{string::String, vec::Vec};

/// (src/game/handwork.ts:55 live)
pub fn live(g: &mut crate::game::Game, half: crate::handwork::LiveHalf) -> bool { todo!() }

/// (src/game/handwork.ts:60 workAt)
pub fn work_at(g: &mut crate::game::Game, c_2: CharId, kind: crate::character::WorkKind, p: V3, n: V3, t: f64, dur: f64, more: Option<&crate::handwork::WorkAtMore>) { todo!() }

/// (src/game/handwork.ts:65 faceToward)
pub fn face_toward(spot: V3, from: V3) -> V3 { todo!() }

/// (src/game/handwork.ts:67 rightOf)
pub fn right_of(n: V3) -> V3 { todo!() }

/// (src/game/handwork.ts:70 workStand)
pub fn work_stand(g: &mut crate::game::Game, c_2: CharId, p: V3, n: V3, dt: f64, hand: Option<f64>, pace: Option<f64>) -> f64 { todo!() }

/// (src/game/handwork.ts:79 handThere)
pub fn hand_there(g: &mut crate::game::Game, pl: PlayerId, dt: f64, p: V3, n: V3) -> bool { todo!() }

/// (src/game/handwork.ts:86 samWorkHand)
pub fn sam_work_hand(g: &mut crate::game::Game) -> f64 { todo!() }

/// (src/game/handwork.ts:88 samHand)
pub fn sam_hand(g: &mut crate::game::Game) -> Option<f64> { todo!() }

/// (src/game/handwork.ts:96 padKeys)
pub fn pad_keys(code: &str) -> Vec<[f64; 2]> { todo!() }

/// (src/game/handwork.ts:99 padHands)
pub fn pad_hands(g: &mut crate::game::Game, pad: &crate::handwork::PadHandsPad, dt: f64, secs: f64) -> f64 { todo!() }

/// (src/game/handwork.ts:115 chargeHands)
pub fn charge_hands(g: &mut crate::game::Game, c_2: CharId, p: V3, n: V3, t: f64, dur: f64, hand: Option<f64>) { todo!() }

/// (src/game/handwork.ts:124 breakerFace)
pub fn breaker_face(g: &mut crate::game::Game, from: V3) -> crate::handwork::BreakerFace { todo!() }

/// (src/game/handwork.ts:134 rackFace)
pub fn rack_face(g: &mut crate::game::Game, from: V3) -> crate::handwork::RackFace { todo!() }

/// (src/game/handwork.ts:139 samWork)
pub fn sam_work(g: &mut crate::game::Game) -> Option<crate::handwork::SamWork> { todo!() }

/// (src/game/handwork.ts:152 breakerStart)
pub fn breaker_start(g: &mut crate::game::Game, on: bool) -> bool { todo!() }

/// (src/game/handwork.ts:159 breakerTick)
pub fn breaker_tick(g: &mut crate::game::Game, dt: f64) { todo!() }

/// (src/game/handwork.ts:173 samWorkLetGo)
pub fn sam_work_let_go(g: &mut crate::game::Game, w: crate::handwork::SamWork) -> bool { todo!() }

/// (src/game/handwork.ts:175 leverHands)
pub fn lever_hands(g: &mut crate::game::Game, c_2: CharId, t: f64, dur: f64) { todo!() }

/// (src/game/handwork.ts:182 teamWorkHand)
pub fn team_work_hand(g: &mut crate::game::Game, c_2: CharId, h: Option<crate::handwork::TeamWorkHandH>, kind: Option<crate::handwork::TeamWorkHandKind>, t: Option<f64>, dur: Option<f64>, dir: Option<f64>) -> bool { todo!() }

/// (src/game/handwork.ts:191 leverStand)
pub fn lever_stand(g: &mut crate::game::Game, gd: GuardId, dt: f64) -> Option<crate::handwork::LeverStand> { todo!() }

/// (src/game/handwork.ts:201 driveHands)
pub fn drive_hands(g: &mut crate::game::Game, secs: f64, pulled: bool) { todo!() }

/// (src/game/handwork.ts:210 workProps)
pub fn work_props(g: &mut crate::game::Game, c_2: CharId, out: &mut Vec<BoxId>) { todo!() }

/// (src/game/handwork.ts:222 radioHand)
pub fn radio_hand(g: &mut crate::game::Game, gd: GuardId, secs: f64) { todo!() }

/// (src/game/handwork.ts:231 swapFeed)
pub fn swap_feed(g: &mut crate::game::Game, gd: GuardId, held: Option<crate::handwork::SwapFeedHeld>, plain: bool) { todo!() }

/// (src/game/handwork.ts:240 swapHeld)
pub fn swap_held(g: &mut crate::game::Game, gd: GuardId, held: Option<crate::handwork::SwapHeldHeld>) -> Option<crate::handwork::SwapHeld> { todo!() }

/// (src/game/handwork.ts:244 swapping)
pub fn swapping(g: &mut crate::game::Game, gd: GuardId) -> f64 { todo!() }

/// (src/game/handwork.ts:246 beltTorch)
pub fn belt_torch(g: &mut crate::game::Game, c_2: CharId, out: &mut Vec<BoxId>) { todo!() }
