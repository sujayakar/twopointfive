//! The types of `fixture_break`.
use crate::{Blank, bytes::ToBytes, ids::*, js::*, Quat, V3};
use alloc::vec::Vec;

#[derive(Clone, Copy, Debug, ToBytes, Default)]
pub struct Piece {
    pub box_: FrameBoxId,
    pub v: V3,
    pub axis: V3,
    pub rate: f64,
    pub ang: f64,
    pub q0: Quat,
    pub floor_y: f64,
    pub yaw: f64,
    pub delay: f64,
    pub rest: bool,
    pub bounced: bool,
    pub glass: bool,
    pub on: Option<BoxId>,
    pub on_x: f64,
    pub on_z: f64,
}

#[derive(Clone, Debug, ToBytes, Default)]
pub struct Rec {
    pub t: FixtureId,
    pub born: f64,
    pub seed: f64,
    pub seen: f64,
    pub hinge: V3,
    pub uh: V3,
    pub l: f64,
    pub w: f64,
    pub th: f64,
    pub om: f64,
    pub rest: f64,
    pub still: bool,
    pub hang: Vec<FrameBoxId>,
    pub pieces: Vec<crate::fixture_break::Piece>,
    pub pop: f64,
    pub last_slot: f64,
    pub glass_out: bool,
    pub tip: V3,
    pub emits: f64,
    pub phase: f64,
    pub fall_at: f64,
    pub h0: V3,
    pub th0: f64,
    pub t0: f64,
    pub lie: V3,
    pub uh0: V3,
    pub skew: f64,
    pub lay: f64,
}

#[derive(Clone, Debug, ToBytes, Default)]
pub struct State {
    pub recs: Vec<crate::fixture_break::Rec>,
    pub prev: JsMap<FixtureId, bool>,
    pub kind: JsMap<FixtureId, f64>,
    pub last_t: f64,
    pub scan: f64,
    pub keys: f64,
}

#[derive(Clone, Copy, Debug, ToBytes, Default)]
pub struct FixtureBreakModFLOOR {
    pub on: Option<BoxId>,
}

#[derive(Clone, Copy, Debug, Default)]
pub struct RestOn {
    pub y: f64,
    pub flat: bool,
}

#[derive(Clone, Copy, Debug, Default)]
pub struct GlowAtOut {
    pub sparks: f64,
}

/// the state this module keeps at its top level
#[derive(Clone, Debug, ToBytes, Blank)]
pub struct FixtureBreakMod {
    /// a stage cuts to the dark as its lamp goes: nothing of one is seen to break
    pub put_down: bool,
    pub state: Option<crate::fixture_break::State>,
    #[is(crate::fixture_break::FixtureBreakModFLOOR { on: None })] pub floor: crate::fixture_break::FixtureBreakModFLOOR,
}

pub const LOOK: f64 = 10.0;

pub fn down() -> V3 { [0.0, -1.0, 0.0] }

pub fn rest_at() -> Vec<f64> { alloc::vec![0.15, 0.5, 0.85] }

