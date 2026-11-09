//! The types of `interact`.
#![allow(unused_imports)]
use crate::{ids::*, js::*, Quat, V3};
use alloc::{string::String, vec::Vec};

#[derive(Clone, Debug, Default)]
pub struct EmptyThing {
    pub it: crate::game::Interactable,
    pub d: f64,
    pub why: String,
}

/// (src/game/interact.ts:68 TAP_SECS)
pub const TAP_SECS: f64 = 0.25;

/// (src/game/interact.ts:71 WHEEL_STEP)
pub const WHEEL_STEP: f64 = 60.0;

/// (src/game/interact.ts:116 RING_BODY_M)
pub const RING_BODY_M: f64 = 1.0;

/// (src/game/interact.ts:116 RING_THING_M)
pub const RING_THING_M: f64 = 3.5;

