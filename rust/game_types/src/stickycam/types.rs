//! The types of `stickycam`.
#![allow(unused_imports)]
use crate::{bytes::ToBytes, ids::*, js::*, Quat, V3};
use alloc::{string::String, vec::Vec};

#[derive(Clone, Copy, Debug, ToBytes, Default)]
pub struct StickyCam {
    pub pos: V3,
    pub n: V3,
    pub vel: Option<V3>,
    pub age: f64,
    pub si: f64,
    pub id: f64,
}

#[derive(Clone, Debug, Default)]
pub struct StickyLivePl {
    pub cams: Vec<StickyCamId>,
}

#[derive(Clone, Debug, Default)]
pub struct StickyRowsRowOver {
    pub kind: Option<crate::stickycam::StickyRowsRowOverKind>,
    pub pos: Option<V3>,
    pub line1: Option<String>,
    pub line2: Option<String>,
    pub in_reach: Option<bool>,
    pub off: Option<bool>,
    pub progress: Option<f64>,
    pub target: Option<FixtureId>,
    pub door: Option<DoorId>,
    pub guard: Option<GuardId>,
    pub verb: Option<crate::game::Verb>,
    pub running: Option<bool>,
    pub close: Option<crate::close::CloseKind>,
    pub past: Option<bool>,
    pub framed: Option<bool>,
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub enum StickyRowsRowOverKind {
    Cam,
    Guard,
    Breaker,
    Door,
    Light,
    Body,
    Objective,
    Held,
    Wall,
    Switch,
    Charge,
    Cancel,
}
impl StickyRowsRowOverKind {
    pub const fn as_str(self) -> &'static str {
        match self {
            StickyRowsRowOverKind::Cam => "cam",
            StickyRowsRowOverKind::Guard => "guard",
            StickyRowsRowOverKind::Breaker => "breaker",
            StickyRowsRowOverKind::Door => "door",
            StickyRowsRowOverKind::Light => "light",
            StickyRowsRowOverKind::Body => "body",
            StickyRowsRowOverKind::Objective => "objective",
            StickyRowsRowOverKind::Held => "held",
            StickyRowsRowOverKind::Wall => "wall",
            StickyRowsRowOverKind::Switch => "switch",
            StickyRowsRowOverKind::Charge => "charge",
            StickyRowsRowOverKind::Cancel => "cancel",
        }
    }
}

/// the state this module keeps at its top level
#[derive(Clone, Debug, ToBytes)]
pub struct StickycamMod {
    /// (src/game/stickycam.ts:76 VISIT)
    pub visit: Option<f64>,
}
impl StickycamMod {
    pub fn blank() -> Self {
        StickycamMod {
            visit: None,
        }
    }
}

/// the counters this module keeps for the checks
#[derive(Clone, Debug, Default)]
pub struct StickycamStats {
    pub fired: i32,
    pub stuck: i32,
    pub dropped: i32,
    pub retired: i32,
    pub blind: i32,
    pub taken: i32,
}

/// (src/game/stickycam.ts:71 EYE_ID)
pub const EYE_ID: f64 = 200.0;

/// (src/game/stickycam.ts:202 NONE: a table nothing writes)
pub fn none() -> Vec<crate::fogofwar::FogEye> { Vec::new() }

/// (src/game/stickycam.ts:202 NONE_CAMS: a table nothing writes)
pub fn none_cams() -> Vec<StickyCamId> { Vec::new() }

/// (src/game/stickycam.ts:241 BODY: a table nothing writes)
pub fn body() -> V3 { [0.05, 0.05, 0.055] }

/// (src/game/stickycam.ts:241 LENS: a table nothing writes)
pub fn lens() -> V3 { [0.012, 0.012, 0.014] }

/// (src/game/stickycam.ts:241 LENS_LIVE: a table nothing writes)
pub fn lens_live() -> V3 { [0.42, 0.05, 0.04] }

/// (src/game/stickycam.ts:261 CAM_KEY)
pub const CAM_KEY: f64 = 8388608.0;

