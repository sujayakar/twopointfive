//! The types of `smokebreak`.
use crate::{Blank, bytes::ToBytes, ids::*, js::*, V3};

#[derive(Clone, Copy, Debug, ToBytes, Default)]
pub struct SmokeBreakStats {
    pub lit: f64,
    pub draws: f64,
    pub flames: f64,
    pub embers: f64,
    pub breaths: f64,
}

#[derive(Clone, Copy, Debug, ToBytes, Default)]
pub struct Man {
    pub t: f64,
    pub flamed: bool,
    pub ember_t: f64,
    pub live: bool,
    /// when his breath comes out, on his `t`; None while none is due
    pub out_at: Option<f64>,
}

#[derive(Clone, Debug, ToBytes, Default)]
pub struct State {
    pub men: JsMap<CharId, crate::smokebreak::Man>,
    pub stats: crate::smokebreak::SmokeBreakStats,
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
#[derive(Clone, Debug, ToBytes, Blank)]
pub struct SmokebreakMod {
    pub states: Option<crate::smokebreak::State>,
}
