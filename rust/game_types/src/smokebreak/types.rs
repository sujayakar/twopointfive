//! The types of `smokebreak`.
#![allow(unused_imports)]
use crate::{bytes::ToBytes, ids::*, js::*, Quat, V3};
use alloc::{string::String, vec::Vec};

#[derive(Clone, Copy, Debug, ToBytes, Default)]
pub struct SmokeBreakStats {
    pub men: f64,
    pub lit: f64,
    pub draws: f64,
    pub flames: f64,
    pub embers: f64,
    pub breaths: f64,
    pub frames: f64,
}

#[derive(Clone, Copy, Debug, ToBytes, Default)]
pub struct Man {
    pub t: f64,
    pub flamed: bool,
    pub draws: f64,
    pub outs: f64,
    pub ember_t: f64,
    pub live: bool,
    /// INVENTED by a review: see the proposals
    pub out_at: Option<f64>,
}

#[derive(Clone, Debug, ToBytes, Default)]
pub struct State {
    pub men: JsMap<CharId, crate::smokebreak::Man>,
    pub stats: crate::smokebreak::SmokeBreakStats,
    pub seed: f64,
}

#[derive(Clone, Copy, Debug, Default)]
pub struct HandAt {
    pub up: f64,
    pub drawn: bool,
    pub cycle: f64,
    pub into: f64,
}

#[derive(Clone, Copy, Debug, Default)]
pub struct MouthOf {
    pub pos: V3,
    pub dir: V3,
}

/// the state this module keeps at its top level
#[derive(Clone, Debug, ToBytes)]
pub struct SmokebreakMod {
    /// (src/game/smokebreak.ts:69 states)
    pub states: Option<crate::smokebreak::State>,
}
impl SmokebreakMod {
    pub fn blank() -> Self {
        SmokebreakMod {
            states: None,
        }
    }
}
