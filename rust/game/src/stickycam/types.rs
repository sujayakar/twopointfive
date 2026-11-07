//! The types of `stickycam`, written by the skeleton's program from the TypeScript. Never edited by hand: hand.cjs, and run it again.
#![allow(unused_imports)]
use crate::{ids::*, js::*, Quat, V3};
use alloc::{string::String, vec::Vec};

/// (TypeScript: StickyCam, src/game/stickycam.ts:65)
#[derive(Clone, Copy, Debug)]
pub struct StickyCam {
    pub pos: V3,
    pub n: V3,
    pub vel: Option<V3>,
    pub age: f64,
    pub si: f64,
    pub id: f64,
}
impl StickyCam {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        StickyCam {
            pos: [0.0; 3],
            n: [0.0; 3],
            vel: None,
            age: 0.0,
            si: 0.0,
            id: 0.0,
        }
    }
}
impl Default for StickyCam { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: StickySnap, src/game/stickycam.ts:67)
#[derive(Clone, Debug)]
pub struct StickySnap {
    pub p: Vec<f64>,
    pub n: Vec<f64>,
    pub si: f64,
    pub id: f64,
}
impl StickySnap {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        StickySnap {
            p: Vec::new(),
            n: Vec::new(),
            si: 0.0,
            id: 0.0,
        }
    }
}
impl Default for StickySnap { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: CamMark, src/game/stickycam.ts:270)
#[derive(Clone, Copy, Debug)]
pub struct CamMark {
    pub at: V3,
    pub sends: bool,
}
impl CamMark {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        CamMark {
            at: [0.0; 3],
            sends: false,
        }
    }
}
impl Default for CamMark { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/stickycam.ts:183 stickyLive(pl), src/game/stickycam.ts:183)
#[derive(Clone, Debug)]
pub struct StickyLivePl {
    pub cams: Vec<StickyCamId>,
}
impl StickyLivePl {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        StickyLivePl {
            cams: Vec::new(),
        }
    }
}
impl Default for StickyLivePl { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/stickycam.ts:221 stickyRows(row), ../.npm-global/lib/node_modules/typescript/lib/lib.es5.d.ts:1583)
#[derive(Clone, Debug)]
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
impl StickyRowsRowOver {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        StickyRowsRowOver {
            kind: None,
            pos: None,
            line1: None,
            line2: None,
            in_reach: None,
            off: None,
            progress: None,
            target: None,
            door: None,
            guard: None,
            verb: None,
            running: None,
            close: None,
            past: None,
            framed: None,
        }
    }
}
impl Default for StickyRowsRowOver { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: words)
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
#[derive(Clone, Debug)]
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

/// (src/game/stickycam.ts:277 NO_MARKS: a table nothing writes)
pub fn no_marks() -> Vec<crate::stickycam::CamMark> { Vec::new() }
