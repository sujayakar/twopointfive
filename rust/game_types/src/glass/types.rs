//! The types of `glass`.
#![allow(unused_imports)]
use crate::{bytes::ToBytes, ids::*, js::*, Quat, V3};
use alloc::{string::String, vec::Vec};

#[derive(Clone, Debug, ToBytes, Default)]
pub struct GlassState {
    pub panes: Vec<f64>,
    pub seen: JsSet<u32>,
    pub hits: f64,
    pub last_hit_t: f64,
    pub log: Vec<String>,
    pub crack_at: f64,
}

#[derive(Clone, Copy, Debug, Default)]
pub struct NearestOnBox {
    pub q: V3,
    pub d: f64,
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub enum GlassImpactCause {
    Shot,
    Bang,
    Hand,
}

#[derive(Clone, Copy, Debug, Default)]
pub struct GlassImpactOpts {
    pub through: Option<bool>,
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub enum GlassHitCause {
    Shot,
    Bang,
    Hand,
}

/// the state this module keeps at its top level
#[derive(Clone, Debug, ToBytes)]
pub struct GlassMod {
    /// (src/game/glass.ts:59 states)
    pub states: Option<crate::glass::GlassState>,
}
impl GlassMod {
    pub fn blank() -> Self {
        GlassMod {
            states: None,
        }
    }
}

/// (src/game/glass.ts:74 THICK_GLASS_HALF)
pub const THICK_GLASS_HALF: f64 = 0.05;
