//! The types of `wallsnap`.
#![allow(unused_imports)]
use crate::{ids::*, js::*, Quat, V3};
use alloc::{string::String, vec::Vec};

#[derive(Clone, Copy, Debug, Default)]
pub struct WsCornerAngle {
    pub deg: f64,
    pub dist: f64,
}

/// (src/game/wallsnap.ts:26 LEAN_GUN_M)
pub const LEAN_GUN_M: f64 = 0.42;
