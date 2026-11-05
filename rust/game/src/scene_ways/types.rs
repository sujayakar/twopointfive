//! The types of `scene_ways`, written by the skeleton's program from the TypeScript. Never edited by hand: hand.cjs, and run it again.
#![allow(unused_imports)]
use crate::{ids::*, js::*, Quat, V3};
use alloc::{string::String, vec::Vec};

/// (TypeScript: WayHint, src/scene/ways.ts:27)
#[derive(Clone, Debug)]
pub struct WayHint {
    pub from: String,
    pub to: String,
    pub weight: Option<f64>,
}
impl WayHint {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        WayHint {
            from: String::new(),
            to: String::new(),
            weight: None,
        }
    }
}
impl Default for WayHint { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: Ways, src/scene/ways.ts:28)
#[derive(Clone, Debug)]
pub struct Ways {
    pub x0: f64,
    pub z0: f64,
    pub cell: f64,
    pub w: f64,
    pub h: f64,
    pub patrol: Vec<f32>,
    pub team: Vec<f32>,
    pub doors: Vec<f32>,
    pub hint: Vec<f32>,
    pub paths: Vec<crate::scene_ways::WaysPathsItem>,
    pub unknown: Vec<String>,
    pub lost: f64,
}
impl Ways {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        Ways {
            x0: 0.0,
            z0: 0.0,
            cell: 0.0,
            w: 0.0,
            h: 0.0,
            patrol: Vec::new(),
            team: Vec::new(),
            doors: Vec::new(),
            hint: Vec::new(),
            paths: Vec::new(),
            unknown: Vec::new(),
            lost: 0.0,
        }
    }
}
impl Default for Ways { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/scene/ways.ts:33 Ways.paths, src/scene/ways.ts:33)
#[derive(Clone, Debug)]
pub struct WaysPathsItem {
    pub kind: crate::scene_ways::WaysPathsItemKind,
    pub weight: f64,
    pub points: Vec<V3>,
}
impl WaysPathsItem {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        WaysPathsItem {
            kind: crate::scene_ways::WaysPathsItemKind::Patrol,
            weight: 0.0,
            points: Vec::new(),
        }
    }
}
impl Default for WaysPathsItem { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: words)
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub enum WaysPathsItemKind {
    Patrol,
    Team,
    Doors,
    Hint,
}
impl WaysPathsItemKind {
    pub const fn as_str(self) -> &'static str {
        match self {
            WaysPathsItemKind::Patrol => "patrol",
            WaysPathsItemKind::Team => "team",
            WaysPathsItemKind::Doors => "doors",
            WaysPathsItemKind::Hint => "hint",
        }
    }
}

/// (TypeScript: inline at src/scene/ways.ts:38 doorMid(d), src/scene/ways.ts:38)
#[derive(Clone, Copy, Debug)]
pub struct DoorMidD {
    pub hinge: [f64; 2],
    pub closed_dir: f64,
    pub width: f64,
}
impl DoorMidD {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        DoorMidD {
            hinge: [0.0; 2],
            closed_dir: 0.0,
            width: 0.0,
        }
    }
}
impl Default for DoorMidD { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/scene/ways.ts:43 levelWays(opts), src/scene/ways.ts:43)
#[derive(Clone, Debug)]
pub struct LevelWaysOpts {
    pub hints: Option<Vec<crate::scene_ways::WayHint>>,
}
impl LevelWaysOpts {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        LevelWaysOpts {
            hints: None,
        }
    }
}
impl Default for LevelWaysOpts { fn default() -> Self { Self::zeroed() } }

/// (src/scene/ways.ts:40 PATH_ITER)
pub const PATH_ITER: f64 = 6000.0;

/// (src/scene/ways.ts:40 STEP)
pub const STEP: f64 = 0.2;
