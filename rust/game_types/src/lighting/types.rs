//! The types of `lighting`.
#![allow(unused_imports)]
use crate::{bytes::ToBytes, ids::*, js::*, Quat, V3};
use alloc::{string::String, vec::Vec};

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub enum SetBlackoutCause {
    Shot,
    Ocp,
    Debug,
    Hand,
}
impl SetBlackoutCause {
    pub const fn as_str(self) -> &'static str {
        match self {
            SetBlackoutCause::Shot => "shot",
            SetBlackoutCause::Ocp => "ocp",
            SetBlackoutCause::Debug => "debug",
            SetBlackoutCause::Hand => "hand",
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub enum LightEventCause {
    Shot,
    Ocp,
}

/// the state this module keeps at its top level
#[derive(Clone, Debug, ToBytes)]
pub struct LightingMod {
    /// (src/game/lighting.ts:33 LENS_FROM)
    pub lens_from: JsMap<LightId, V3>,
}
impl LightingMod {
    pub fn blank() -> Self {
        LightingMod {
            lens_from: JsMap::new(),
        }
    }
}

/// the counters this module keeps for the checks
#[derive(Clone, Debug, Default)]
pub struct LightingStats {
    pub asked: i32,
    pub pulled: i32,
    pub max_back: f64,
}

/// (src/game/lighting.ts:34 LENS_SLANT)
pub const LENS_SLANT: f64 = 0.3;
