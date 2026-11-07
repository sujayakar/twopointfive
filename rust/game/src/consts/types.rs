//! The types of `consts`, written by the skeleton's program from the TypeScript. Never edited by hand: hand.cjs, and run it again.
#![allow(unused_imports)]
use crate::{ids::*, js::*, Quat, V3};
use alloc::{string::String, vec::Vec};

/// (src/game/consts.ts:2 PLAYER_ID)
pub const PLAYER_ID: f64 = 1.0;

/// (src/game/consts.ts:5 NOBODY_ID)
pub const NOBODY_ID: f64 = 2.0;

/// (src/game/consts.ts:7 QRF_TINT: a table nothing writes)
pub fn qrf_tint() -> [f64; 3] { [0.1, 0.11, 0.13] }

/// (src/game/consts.ts:7 QRF_TINT2: a table nothing writes)
pub fn qrf_tint2() -> [f64; 3] { [0.05, 0.055, 0.06] }

/// (src/game/consts.ts:8 DRIVE_REACH)
pub const DRIVE_REACH: f64 = 1.1;

/// (src/game/consts.ts:8 DRIVE_SECS)
pub const DRIVE_SECS: f64 = 3.5;

/// (src/game/consts.ts:10 INTERACT_KEY)
pub const INTERACT_KEY: &str = "Space";
