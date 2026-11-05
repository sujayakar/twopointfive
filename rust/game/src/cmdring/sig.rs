#![allow(unused_variables)]
//! Every function and method of `cmdring`, with an empty body. Written by the skeleton's program.
#[allow(unused_imports)]
use crate::{ids::*, js::*, Quat, V3};
#[allow(unused_imports)]
use alloc::{string::String, vec::Vec};

/// (src/game/cmdring.ts:212 ringZoom)
pub fn ring_zoom(g: &mut crate::game::Game, w: f64, h: f64) -> f64 { todo!() }

/// (src/game/cmdring.ts:216 ringOpen)
pub fn ring_open(g: &mut crate::game::Game, input: InputId, w: f64, h: f64, time: f64, lit: Option<f64>) -> crate::cmdring::RingState { todo!() }

/// (src/game/cmdring.ts:222 ringPin)
pub fn ring_pin(g: &mut crate::game::Game, r: &mut crate::cmdring::RingState, sx: f64, sy: f64, w: f64, h: f64) { todo!() }

/// (src/game/cmdring.ts:234 ringWant)
pub fn ring_want(g: &mut crate::game::Game, r: &mut crate::cmdring::RingState, input: InputId, digit: f64, time: f64, lone: Option<f64>, shapes: Option<f64>) -> f64 { todo!() }

/// (src/game/cmdring.ts:255 ringSeats)
pub fn ring_seats(g: &mut crate::game::Game, rows: &[crate::cmdring::SeatRow], place: bool, word_key: &str, first: Option<bool>) -> crate::cmdring::RingSeats { todo!() }

/// (src/game/cmdring.ts:287 flatSeats)
pub fn flat_seats(g: &mut crate::game::Game, rows: &[crate::cmdring::FlatRow], kind: &str, word_key: &str, lead: f64) -> crate::cmdring::FlatSeats { todo!() }

/// (src/game/cmdring.ts:305 ringHoldSecs)
pub fn ring_hold_secs(g: &mut crate::game::Game) -> f64 { todo!() }

/// (src/game/cmdring.ts:310 ringLatch)
pub fn ring_latch(r: &mut crate::cmdring::RingState, key_down: bool, key_hit: bool, pointed: bool) -> crate::cmdring::RingLatch { todo!() }

/// (src/game/cmdring.ts:321 wedgeAt)
pub fn wedge_at(dx: f64, dy: f64, n: f64) -> f64 { todo!() }
