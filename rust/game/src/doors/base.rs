//! What the parts of `doors` share, and what it hands to the host.
#![allow(unused_variables)]
#[allow(unused_imports)]
use crate::{ids::*, js::*, Quat, V3};
#[allow(unused_imports)]
use alloc::{string::String, vec::Vec};


/// (src/game/doors.ts:1056 Doors.PICK_SECS)
pub const PICK_SECS: f64 = 4.2;

/// (src/game/doors.ts:1061 Doors.PICK_LEVEL)
pub const PICK_LEVEL: f64 = 0.16;

/// (src/game/doors.ts:1061 Doors.UNLOCK_LEVEL)
pub const UNLOCK_LEVEL: f64 = 0.3;

/// (src/game/doors.ts:1061 Doors.KEY_LEVEL)
pub const KEY_LEVEL: f64 = 0.12;

/// (src/game/doors.ts:1183 Doors.USE_REACH)
pub const USE_REACH: f64 = 1.35;
