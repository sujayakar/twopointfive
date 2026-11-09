//! The types of `scene_ways`.
#![allow(unused_imports)]
use crate::{bytes::ToBytes, ids::*, js::*, Quat, V3};
use alloc::{string::String, vec::Vec};

#[derive(Clone, Debug, ToBytes, Default)]
pub struct WayHint {
    pub from: String,
    pub to: String,
    pub weight: Option<f64>,
}

#[derive(Clone, Debug, Default)]
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

#[derive(Clone, Debug, Default)]
pub struct WaysPathsItem {
    pub kind: crate::scene_ways::WaysPathsItemKind,
    pub weight: f64,
    pub points: Vec<V3>,
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug, Default)]
pub enum WaysPathsItemKind {
    #[default] Patrol,
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

#[derive(Clone, Copy, Debug, Default)]
pub struct DoorMidD {
    pub hinge: [f64; 2],
    pub closed_dir: f64,
    pub width: f64,
}

#[derive(Clone, Debug, Default)]
pub struct LevelWaysOpts {
    pub hints: Option<Vec<crate::scene_ways::WayHint>>,
}

/// (src/scene/ways.ts:40 PATH_ITER)
pub const PATH_ITER: f64 = 6000.0;

/// (src/scene/ways.ts:40 STEP)
pub const STEP: f64 = 0.2;
